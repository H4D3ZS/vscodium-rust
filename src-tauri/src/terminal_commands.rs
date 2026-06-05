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

/// Return true if `exe` is launchable: an existing absolute/relative path, or a
/// bare name found on PATH (trying the raw name and, on Windows, `.exe`).
fn exe_is_resolvable(exe: &str) -> bool {
    let p = std::path::Path::new(exe);
    if p.components().count() > 1 || p.is_absolute() {
        return p.exists();
    }
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            if dir.join(exe).exists() {
                return true;
            }
            #[cfg(windows)]
            if !exe.to_lowercase().ends_with(".exe") && dir.join(format!("{exe}.exe")).exists() {
                return true;
            }
        }
    }
    false
}

/// On Windows, a missing exe does NOT make ConPTY's spawn fail — it returns Ok
/// and the child dies instantly, leaving a blank terminal. So resolve the shell
/// up front: if the requested exe can't be found on PATH, fall back to the
/// always-present Windows PowerShell, then cmd.exe.
fn resolve_shell_exe(requested: &str) -> String {
    if exe_is_resolvable(requested) {
        return requested.to_string();
    }
    #[cfg(windows)]
    {
        println!("[Term] '{requested}' not found on PATH — falling back to powershell.exe");
        if exe_is_resolvable("powershell.exe") {
            return "powershell.exe".to_string();
        }
        return "cmd.exe".to_string();
    }
    #[cfg(not(windows))]
    {
        requested.to_string()
    }
}

/// PowerShell OSC 133 shell-integration script (Warp-style command blocks).
/// Written to a temp .ps1 and dot-sourced at shell startup via `-Command` so it
/// runs BEFORE the first prompt and never echoes into the command line. Typing
/// the multi-line script over stdin made ConPTY echo the whole thing as visible
/// garbage; injecting at launch avoids that entirely. Keep the OSC payloads in
/// sync with the parser in `src/terminalBlocks.ts`.
const POWERSHELL_SHELL_INTEGRATION_PS1: &str = r#"
if (-not $global:__vscr_si) {
  $global:__vscr_si = $true
  $global:__vscr_orig_prompt = $function:prompt

  # ── Natural shell prompt (keep the system default, e.g. "PS C:\path>") ────
  # We only wrap it to emit OSC 133 (which drives the command-block gutter marks);
  # the visible prompt is whatever the shell normally shows — no custom glyphs.
  function global:prompt {
    $code = if ($?) { 0 } else { if ($LASTEXITCODE) { $LASTEXITCODE } else { 1 } }
    $e = [char]27; $b = [char]7
    $cwd = (Get-Location).Path
    [Console]::Write("$e]133;D;$code$b$e]133;A$b$e]133;P;Cwd=$cwd$b")
    $orig = & $global:__vscr_orig_prompt
    [Console]::Write("$e]133;B$b")
    return $orig
  }

  if (Get-Module -ListAvailable PSReadLine) {
    # OSC 133;E (command line) + 133;C (pre-exec) on Enter — drives the blocks.
    Set-PSReadLineKeyHandler -Key Enter -ScriptBlock {
      $line = $null; $cur = $null
      [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$line, [ref]$cur)
      $e = [char]27; $b = [char]7
      [Console]::Write("$e]133;E;$line$b$e]133;C$b")
      [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }

    # Fish/Warp-style INLINE autosuggestions from history (shell-native, not AI).
    # Guarded: -PredictionSource/-PredictionViewStyle need PSReadLine 2.1+/2.2+;
    # on Windows PowerShell 5.1's bundled 2.0 the try/catch degrades silently.
    try {
      Set-PSReadLineOption -PredictionSource History -PredictionViewStyle InlineView
      Set-PSReadLineOption -Colors @{ InlinePrediction = "$([char]27)[38;5;240m" }
    } catch { }
    try {
      Set-PSReadLineKeyHandler -Key UpArrow   -Function HistorySearchBackward
      Set-PSReadLineKeyHandler -Key DownArrow -Function HistorySearchForward
      Set-PSReadLineKeyHandler -Key Tab       -Function MenuComplete
      Set-PSReadLineKeyHandler -Key Ctrl+r    -Function ReverseSearchHistory
      Set-PSReadLineKeyHandler -Key RightArrow -Function ForwardChar
    } catch { }
  }

  # ── cmder aliases ────────────────────────────────────────────────────────
  function global:..    { Set-Location .. }
  function global:...   { Set-Location ../.. }
  function global:....  { Set-Location ../../.. }
  function global:ll    { Get-ChildItem -Force @args }
  function global:la    { Get-ChildItem -Force @args }
  function global:gst   { git status @args }
  function global:gco   { git checkout @args }
  function global:gp    { git push @args }
  function global:gl    { git log --oneline -20 @args }
  function global:gd    { git diff @args }
  function global:gaa   { git add -A @args }
  function global:which { Get-Command @args | Select-Object -ExpandProperty Source }
  function global:grep  { $input | Select-String @args }
  function global:touch {
    param($p)
    if (Test-Path $p) { (Get-Item $p).LastWriteTime = Get-Date }
    else { New-Item -ItemType File $p | Out-Null }
  }
  function global:gl    { git log --oneline --all --graph --decorate @args }
  function global:e.    { explorer . }
  function global:clear { Clear-Host }
}
"#;

/// Bash/Zsh OSC 133 shell integration (Warp-style command blocks) + cmder aliases.
/// Sourced via `--init-file` (bash) or `ENV` (zsh) at startup.
const BASH_SHELL_INTEGRATION_SH: &str = r#"
# vscodium-rust terminal — OSC 133 blocks + cmder-style aliases (no AI)
if [ -z "${__vscr_si:-}" ]; then
  export __vscr_si=1
  __vscr_osc() { printf '\033]%s\007' "$1"; }
  __vscr_precmd() {
    local code=$?
    __vscr_osc "133;D;${code}"
    __vscr_osc "133;A"
    __vscr_osc "133;P;Cwd=${PWD}"
  }
  __vscr_preexec() {
    __vscr_osc "133;C"
    [ -n "${BASH_COMMAND:-}" ] && __vscr_osc "133;E;${BASH_COMMAND}"
  }
  if [ -n "${BASH_VERSION:-}" ]; then
    PROMPT_COMMAND="__vscr_precmd${PROMPT_COMMAND:+;}$PROMPT_COMMAND"
    trap '__vscr_preexec' DEBUG
  elif [ -n "${ZSH_VERSION:-}" ]; then
    precmd_functions+=(__vscr_precmd)
    preexec_functions+=(__vscr_preexec)
  fi
  # cmder aliases (bash)
  alias ..='cd ..'
  alias ...='cd ../..'
  alias ....='cd ../../..'
  alias ll='ls -la'
  alias la='ls -la'
  alias gst='git status'
  alias gco='git checkout'
  alias gp='git push'
  alias gl='git log --oneline --graph --decorate -20'
  alias gd='git diff'
  alias gaa='git add -A'
  alias e.='explorer . 2>/dev/null || xdg-open . 2>/dev/null || open .'
  alias clear='printf "\033[2J\033[H"'
fi
"#;

/// Write the shell-integration script to a stable temp path and return it.
/// Stable filename + identical content means concurrent terminal spawns racing
/// the write are harmless (last writer wins, same bytes).
fn write_shell_integration_script() -> std::io::Result<PathBuf> {
    let path = std::env::temp_dir().join("vscr_shell_integration.ps1");
    let mut f = std::fs::File::create(&path)?;
    f.write_all(POWERSHELL_SHELL_INTEGRATION_PS1.as_bytes())?;
    Ok(path)
}

fn write_bash_integration_script() -> std::io::Result<PathBuf> {
    let path = std::env::temp_dir().join("vscr_shell_integration.sh");
    let mut f = std::fs::File::create(&path)?;
    f.write_all(BASH_SHELL_INTEGRATION_SH.as_bytes())?;
    Ok(path)
}

fn is_bash_like(shell: &str) -> bool {
    let s = shell.to_lowercase();
    s.contains("bash") || s.ends_with("/sh") || s == "sh"
}

fn is_zsh(shell: &str) -> bool {
    shell.to_lowercase().contains("zsh")
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
    // Resolve missing exes up front — ConPTY won't error on them, it just
    // produces a dead, blank terminal. (e.g. pwsh.exe → powershell.exe.)
    let shell_exe = resolve_shell_exe(&shell_exe);

    let mut cmd = CommandBuilder::new(shell_exe.clone());
    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");

    // Warp-style command blocks: inject OSC 133 shell integration at STARTUP so it
    // loads before the first prompt and never echoes into the command line. (Typing
    // the script over stdin made ConPTY echo the whole multi-line thing.) PowerShell
    // only; `-NoExit` keeps the session interactive after dot-sourcing.
    {
        let sl = shell_exe.to_lowercase();
        if sl.contains("powershell") || sl.contains("pwsh") {
            if let Ok(script_path) = write_shell_integration_script() {
                cmd.arg("-NoExit");
                cmd.arg("-Command");
                cmd.arg(format!(". '{}'", script_path.display()));
            }
        } else if is_bash_like(&shell_exe) {
            if let Ok(script_path) = write_bash_integration_script() {
                cmd.arg("--init-file");
                cmd.arg(script_path.display().to_string());
                cmd.arg("-i");
            }
        } else if is_zsh(&shell_exe) {
            if let Ok(script_path) = write_bash_integration_script() {
                cmd.env("ENV", script_path.display().to_string());
            }
        }
    }

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

                    // PRIMARY transport: append to the pending buffer the UI
                    // drains via `terminal_take_pending`. Blocking lock so we
                    // never drop bytes. The legacy `terminal-data` event is kept
                    // as a best-effort secondary for any external listeners.
                    if let Ok(mut pend) = state.terminal_pending.lock() {
                        pend.entry(term_id.clone()).or_default().push_str(&data);
                    }
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
    if let Ok(mut pend) = state.terminal_pending.lock() {
        pend.remove(&id);
    }
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

/// Drain and return any PTY output produced since the last call. This is the
/// primary terminal transport — the frontend polls it because the global
/// `terminal-data` event stream does not reliably reach the webview.
#[tauri::command]
pub async fn terminal_take_pending(state: State<'_, EditorState>, id: String) -> Result<String, String> {
    let out = {
        let mut pend = state
            .terminal_pending
            .lock()
            .map_err(|e| format!("pending lock poisoned: {e}"))?;
        pend.get_mut(&id)
            .map(std::mem::take)
            .unwrap_or_default()
    };
    Ok(out)
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

#[cfg(test)]
mod pty_smoke {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    /// Headless replica of `spawn_terminal`'s PTY core: open a pty, spawn the
    /// resolved default shell, and read for up to 2s. Proves whether
    /// portable-pty produces shell output on THIS machine, isolating PTY
    /// mechanics from the Tauri emit/listen path. Run:
    ///   cargo test pty_smoke -- --nocapture
    #[test]
    fn pty_produces_shell_output() {
        let shell = resolve_shell_exe(if cfg!(windows) { "powershell.exe" } else { "/bin/bash" });
        eprintln!("[pty_smoke] resolved shell = {shell}");

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize { rows: 24, cols: 80, pixel_width: 0, pixel_height: 0 })
            .expect("openpty failed");

        let mut cmd = CommandBuilder::new(shell.clone());
        cmd.env("TERM", "xterm-256color");
        if let Some(cwd) = default_terminal_cwd() {
            cmd.cwd(cwd.as_os_str().to_owned());
        }

        let _child = pair.slave.spawn_command(cmd).expect("spawn_command failed");
        let mut reader = pair.master.try_clone_reader().expect("clone reader failed");
        let _master = pair.master; // keep alive while reading

        let (tx, rx) = mpsc::channel::<usize>();
        std::thread::spawn(move || {
            let mut buf = [0u8; 8192];
            let mut total = 0usize;
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        total += n;
                        eprint!("{}", String::from_utf8_lossy(&buf[..n]));
                        let _ = tx.send(total);
                    }
                    _ => break,
                }
            }
        });

        // Collect bytes for up to 2 seconds.
        let mut got = 0usize;
        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while std::time::Instant::now() < deadline {
            if let Ok(t) = rx.recv_timeout(Duration::from_millis(200)) {
                got = t;
                if got > 0 { break; }
            }
        }
        eprintln!("\n[pty_smoke] total bytes read in 2s = {got}");
        assert!(got > 0, "PTY produced NO output in 2s — shell '{shell}' failed to render under portable-pty on this host");
    }
}
