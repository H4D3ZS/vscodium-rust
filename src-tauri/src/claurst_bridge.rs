// claurst_bridge.rs — external-process bridge to the `claurst` agent CLI.
//
// WHY A PROCESS BOUNDARY (not crate linking):
//   The claurst workspace is licensed GPL-3.0. Linking its crates directly into
//   this (proprietary) Tauri backend would make the whole IDE a derivative work
//   subject to GPL. Running claurst as a *separate process* and talking to it
//   over stdio is mere aggregation — no license contamination. It also avoids
//   duplicating the in-process Sentient agent: claurst becomes an *optional,
//   user-selected* alternative backend rather than a second tangled agent core.
//
// PROTOCOL:
//   We invoke `claurst --print --output-format stream-json` and feed the prompt
//   on stdin (text input-format). claurst emits NDJSON on stdout:
//     {"type":"text_delta","text":"..."}
//     {"type":"tool_start","tool":"..."}
//     {"type":"result","usage":{...},"cost_usd":...}
//   We forward each line to the frontend as a `claurst-stream` Tauri event and
//   return the accumulated assistant text when the process exits.

use crate::state::EditorState;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, Emitter, State};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Locate the `claurst` binary. Order: explicit env override → MSI/NSIS bundle
/// (`src-tauri/binaries/claurst.exe`) → dev workspace build → PATH.
fn find_claurst() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("CLAURST_BIN") {
        let pb = PathBuf::from(p);
        if pb.exists() {
            return Some(pb);
        }
    }

    let exe = if cfg!(windows) { "claurst.exe" } else { "claurst" };

    // Installer layout: shipped via bundle.resources "binaries/*"
    if let Ok(cur) = std::env::current_exe() {
        if let Some(dir) = cur.parent() {
            for root in [
                dir.join("binaries"),
                dir.join("resources").join("binaries"),
                dir.to_path_buf(),
            ] {
                let cand = root.join(exe);
                if cand.is_file() {
                    return Some(cand);
                }
            }
        }
    }
    // Dev after prebuild:sidecar
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let cand = PathBuf::from(manifest).join("binaries").join(exe);
        if cand.is_file() {
            return Some(cand);
        }
    }

    // Candidate roots: the project working dir and a couple of likely parents,
    // so this resolves whether the IDE runs from src-tauri/ or the repo root.
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.clone());
        if let Some(parent) = cwd.parent() {
            roots.push(parent.to_path_buf());
        }
    }
    for root in roots {
        for sub in ["claurst/src-rust/target/release", "../claurst/src-rust/target/release"] {
            let cand = root.join(sub).join(exe);
            if cand.exists() {
                return Some(cand);
            }
        }
    }

    // Fall back to PATH.
    which_on_path(exe)
}

fn which_on_path(exe: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let cand = dir.join(exe);
        if cand.is_file() {
            return Some(cand);
        }
    }
    None
}

/// Report whether the external claurst agent is available (and its version).
#[tauri::command]
pub async fn claurst_status() -> Result<Value, String> {
    let Some(bin) = find_claurst() else {
        return Ok(json!({
            "available": false,
            "reason": "claurst binary not found. Run npm run prebuild:sidecar before release, or: cd claurst/src-rust && cargo build --release --bin claurst (or set CLAURST_BIN)."
        }));
    };

    let mut cmd = Command::new(&bin);
    cmd.arg("--version").stdout(Stdio::piped()).stderr(Stdio::null());
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let version = match cmd.output().await {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        Err(e) => return Ok(json!({ "available": false, "path": bin.to_string_lossy(), "reason": e.to_string() })),
    };

    Ok(json!({
        "available": true,
        "path": bin.to_string_lossy(),
        "version": version,
    }))
}

/// Run a single headless query against the external claurst agent, streaming
/// `claurst-stream` events to the frontend and returning the final text.
#[tauri::command]
pub async fn claurst_run(
    app: AppHandle,
    state: State<'_, EditorState>,
    prompt: String,
    session_id: Option<String>,
    cwd: Option<String>,
) -> Result<String, String> {
    let bin = find_claurst().ok_or_else(|| {
        "claurst binary not found. Build it with: cd claurst/src-rust && cargo build --release --bin claurst".to_string()
    })?;

    // Resolve working directory: explicit arg → active project root → cwd.
    let workdir: PathBuf = if let Some(c) = cwd.filter(|s| !s.trim().is_empty()) {
        PathBuf::from(c)
    } else if let Some(root) = state.active_root.lock().await.clone() {
        root
    } else {
        std::env::current_dir().map_err(|e| e.to_string())?
    };

    let session = session_id.unwrap_or_else(|| format!("ide-{}", chrono::Utc::now().timestamp_millis()));

    let mut cmd = Command::new(&bin);
    cmd.current_dir(&workdir)
        .arg("--print")
        .arg("--output-format")
        .arg("stream-json")
        .arg("--input-format")
        .arg("text")
        .arg("--session-id")
        .arg(&session)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let mut child = cmd.spawn().map_err(|e| format!("failed to spawn claurst: {e}"))?;

    // Feed the prompt on stdin, then close it so claurst proceeds.
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(prompt.as_bytes())
            .await
            .map_err(|e| format!("write prompt failed: {e}"))?;
        stdin.shutdown().await.ok();
    }

    let stdout = child.stdout.take().ok_or("no stdout from claurst")?;
    let mut reader = BufReader::new(stdout).lines();

    let _ = app.emit("claurst-stream", json!({ "type": "start", "session": session }));

    let mut full_text = String::new();
    while let Ok(Some(line)) = reader.next_line().await {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(trimmed) {
            Ok(v) => {
                if v.get("type").and_then(|t| t.as_str()) == Some("text_delta") {
                    if let Some(t) = v.get("text").and_then(|t| t.as_str()) {
                        full_text.push_str(t);
                    }
                }
                let _ = app.emit("claurst-stream", v);
            }
            Err(_) => {
                // Non-JSON line (shouldn't happen in stream-json) — forward raw.
                let _ = app.emit("claurst-stream", json!({ "type": "raw", "line": trimmed }));
            }
        }
    }

    // Drain stderr for diagnostics on failure.
    let mut stderr_buf = String::new();
    if let Some(stderr) = child.stderr.take() {
        let mut err_reader = BufReader::new(stderr).lines();
        while let Ok(Some(l)) = err_reader.next_line().await {
            stderr_buf.push_str(&l);
            stderr_buf.push('\n');
        }
    }

    let status = child.wait().await.map_err(|e| e.to_string())?;
    if !status.success() {
        let msg = if stderr_buf.trim().is_empty() {
            format!("claurst exited with status {status}")
        } else {
            format!("claurst failed: {}", stderr_buf.trim())
        };
        let _ = app.emit("claurst-stream", json!({ "type": "error", "error": msg }));
        return Err(msg);
    }

    let _ = app.emit("claurst-stream", json!({ "type": "done" }));
    Ok(full_text)
}
