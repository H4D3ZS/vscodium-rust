use crate::{EditorState, domain::TerminalDataPayload};
use tauri::{State, AppHandle, Emitter, Manager};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

use std::io::Write;
use std::path::PathBuf;

use serde_json::json as json_serde;

/// Strip any embedded NUL byte. portable_pty hands the raw bytes to
/// CreateProcessW which rejects strings containing `\0` and turns it into a
/// noisy "system cannot find the file specified" error. Sanitising here means
/// a stale value (e.g. from a corrupted localStorage entry) does not bring the
/// whole terminal down.
fn sanitize_no_nul(s: &str) -> String {
    s.split('\0').next().unwrap_or("").trim().to_string()
}

/// Best-effort fallback cwd when the configured one is missing or invalid.
fn default_terminal_cwd() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        if let Ok(p) = std::env::var("USERPROFILE") {
            let pb = PathBuf::from(p);
            if pb.is_dir() {
                return Some(pb);
            }
        }
    } else if let Ok(p) = std::env::var("HOME") {
        let pb = PathBuf::from(p);
        if pb.is_dir() {
            return Some(pb);
        }
    }
    std::env::current_dir().ok()
}

#[tauri::command]
pub async fn spawn_terminal(
    state: State<'_, EditorState>,
    app: AppHandle,
    id: String,
    shell: Option<String>,
) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let shell_exe_raw = if let Some(s) = shell {
        if s.is_empty() {
            if cfg!(target_os = "windows") {
                std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string())
            } else {
                #[allow(clippy::redundant_closure)]
                std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
            }
        } else {
            s
        }
    } else if cfg!(target_os = "windows") {
        std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string())
    } else {
        #[allow(clippy::redundant_closure)]
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
    };
    let shell_exe = sanitize_no_nul(&shell_exe_raw);
    let shell_exe = if shell_exe.is_empty() {
        if cfg!(target_os = "windows") {
            "powershell.exe".to_string()
        } else {
            "/bin/bash".to_string()
        }
    } else {
        shell_exe
    };

    let mut cmd = CommandBuilder::new(shell_exe.clone());
    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");

    // Resolve a usable cwd: active project root if present and on disk,
    // otherwise the user home, otherwise the process cwd. Never pass a path
    // that contains a NUL byte (CreateProcessW rejects it) or that no longer
    // exists (stale activeRoot in localStorage was the cause of "spawn pwsh.exe
    // failed: cannot find file" after deleting a project folder).
    let effective_cwd: Option<PathBuf> = {
        let root = state.active_root.lock().await;
        let from_state = root.as_ref().and_then(|r| {
            let cleaned = sanitize_no_nul(&r.display().to_string());
            let pb = PathBuf::from(cleaned);
            if pb.is_dir() {
                Some(pb)
            } else {
                None
            }
        });
        from_state.or_else(default_terminal_cwd)
    };
    if let Some(ref cwd) = effective_cwd {
        cmd.cwd(cwd.as_os_str().to_owned());
    }

    let child_result = pair.slave.spawn_command(cmd);

    let child = match child_result {
        Ok(c) => c,
        Err(e) => {
            println!(
                "[Term] Failed to spawn {} (cwd={:?}): {}. Trying fallback...",
                shell_exe, effective_cwd, e
            );
            if cfg!(target_os = "windows") && shell_exe.to_lowercase() != "powershell.exe" {
                let mut fallback_cmd = CommandBuilder::new("powershell.exe");
                if let Some(ref cwd) = effective_cwd {
                    fallback_cmd.cwd(cwd.as_os_str().to_owned());
                }
                pair.slave.spawn_command(fallback_cmd).map_err(|e2| {
                    format!(
                        "Primary shell ({}) failed: {}. Fallback (powershell.exe) failed: {}",
                        shell_exe, e, e2
                    )
                })?
            } else if cfg!(target_os = "windows") {
                let mut cmd_fallback = CommandBuilder::new("cmd.exe");
                if let Some(ref cwd) = effective_cwd {
                    cmd_fallback.cwd(cwd.as_os_str().to_owned());
                }
                pair.slave
                    .spawn_command(cmd_fallback)
                    .map_err(|e2: anyhow::Error| {
                        format!(
                            "Primary shell ({}) failed: {}. Fallback (cmd.exe) failed: {}",
                            shell_exe, e, e2
                        )
                    })?
            } else {
                return Err(e.to_string());
            }
        }
    };

    let writer = pair.master.take_writer().map_err(|e: anyhow::Error| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e: anyhow::Error| e.to_string())?;

    let master = pair.master;

    // Spawn reader thread
    let app_handle = app.clone();
    let term_id = id.clone();
    std::thread::spawn(move || {
        let state = app_handle.state::<EditorState>();
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    
                    let _ = app_handle.emit(
                        "terminal-data",
                        TerminalDataPayload {
                            term_id: term_id.clone(),
                            data: data.clone(),
                        },
                    );

                    if let Some(mut buffers) = state.terminal_buffers.try_lock().ok() {
                        let history = buffers.entry(term_id.clone()).or_insert_with(Vec::new);
                        history.push(data);
                        if history.len() > 2000 {
                            history.drain(0..500);
                        }
                    }
                }
                _ => break,
            }
        }
    });

    state.terminal_masters.lock().await.insert(id.clone(), master);
    state.terminal_writers.lock().await.insert(id.clone(), writer);
    state.terminal_processes.lock().await.insert(id, child);

    Ok(())
}

#[tauri::command]
pub async fn close_terminal(state: State<'_, EditorState>, id: String) -> Result<(), String> {
    state.terminal_writers.lock().await.remove(&id);
    state.terminal_masters.lock().await.remove(&id);
    if let Some(mut child) = state.terminal_processes.lock().await.remove(&id) {
        let _ = child.kill();
    }
    state.terminal_buffers.lock().await.remove(&id);
    Ok(())
}

#[tauri::command]
pub async fn terminal_send_data(

    state: State<'_, EditorState>,
    id: String,
    data: String,
) -> Result<(), String> {
    let mut writers = state.terminal_writers.lock().await;
    if let Some(writer) = writers.get_mut(&id) {
        writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Terminal not found".to_string())
    }
}

#[tauri::command]
pub async fn resize_terminal(
    state: State<'_, EditorState>,
    id: String,
    rows: u16,
    cols: u16,
) -> Result<(), String> {
    let masters = state.terminal_masters.lock().await;
    if let Some(master) = masters.get(&id) {
        master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Terminal not found".to_string())
    }
}

#[tauri::command]
pub fn terminal_toggle(app: AppHandle, visible: bool) -> Result<(), String> {
    app.emit("toggle-terminal", visible)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn terminal_terminate(state: State<'_, EditorState>, id: String) -> Result<(), String> {
    let mut processes = state.terminal_processes.lock().await;
    if let Some(mut child) = processes.remove(&id) {
        let _ = child.kill();
    }
    state.terminal_masters.lock().await.remove(&id);
    state.terminal_writers.lock().await.remove(&id);
    state.terminal_buffers.lock().await.remove(&id);
    Ok(())
}

#[tauri::command]
pub async fn terminal_get_status(
    state: State<'_, EditorState>,
    id: String,
) -> Result<serde_json::Value, String> {
    let mut processes = state.terminal_processes.lock().await;
    if let Some(child) = processes.get_mut(&id) {
        match child.try_wait() {
            Ok(Some(status)) => {
                Ok(json_serde!({ "active": false, "success": status.success() }))
            }
            Ok(None) => Ok(json_serde!({ "active": true })),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Ok(json_serde!({ "active": false, "info": "Process not found or already exited" }))
    }
}

#[tauri::command]
pub async fn terminal_read_output(state: State<'_, EditorState>, id: String) -> Result<String, String> {
    state.terminal_read_output(id).await
}

#[tauri::command]
pub fn get_available_shells() -> Result<Vec<String>, String> {
    #[cfg(windows)]
    {
        Ok(vec!["powershell.exe".to_string(), "cmd.exe".to_string(), "bash.exe".to_string()])
    }
    #[cfg(not(windows))]
    {
        Ok(vec!["/bin/bash".to_string(), "/bin/zsh".to_string(), "/bin/sh".to_string()])
    }
}
