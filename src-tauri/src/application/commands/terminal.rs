use tauri::{State, AppHandle, Emitter, Manager};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

use std::io::Write;
use std::path::PathBuf;

use serde_json::json as json_serde;

/// Per-terminal cap on the UI `pending` buffer (drained by
/// `terminal_take_pending`) — prevents unbounded growth if polling stops.
const MAX_PENDING: usize = 1_024_000;

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
/// The default terminal for consumers: Git Bash (bundled portable → system →
/// PATH) on Windows, so everyone gets a consistent POSIX shell instead of their
/// cmd/PowerShell. Falls back to COMSPEC/PowerShell only if no bash is found.
/// Non-Windows uses $SHELL.
fn default_consumer_shell() -> String {
    #[cfg(windows)]
    {
        if let Some(bash) = crate::ide_shell::resolve_git_bash_exe() {
            return bash.to_string_lossy().to_string();
        }
        return std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string());
    }
    #[cfg(not(windows))]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
    }
}

/// up front: if the requested exe can't be found on PATH, fall back to the
/// always-present Windows PowerShell, then cmd.exe.
fn resolve_shell_exe(requested: &str) -> String {
    if exe_is_resolvable(requested) {
        return requested.to_string();
    }
    #[cfg(windows)]
    {
        let req = requested.to_lowercase();
        if req.contains("bash") || req == "sh" || req == "sh.exe" {
            if let Some(bash) = crate::ide_shell::resolve_git_bash_exe() {
                println!("[Term] Using HADES Git Bash: {}", bash.display());
                return bash.to_string_lossy().to_string();
            }
        }
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
    state: State<'_, std::sync::Arc<crate::EditorState>>,
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
        if s.is_empty() { default_consumer_shell() } else { s }
    } else {
        default_consumer_shell()
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
    if let Some(path) = crate::ide_shell::augmented_path_for_git_bash() {
        if is_bash_like(&shell_exe) {
            cmd.env("PATH", path);
        }
    }

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
        let root = state.editor.active_root.lock().await;
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
        let state = app_handle.state::<std::sync::Arc<crate::EditorState>>();
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();

                    // PRIMARY transport: append to the pending buffer the UI
                    // drains via `terminal_take_pending`. Blocking lock so we
                    // never drop bytes. (The old `terminal-data` emit was pure
                    // overhead — the frontend renders via polling only.)
                    if let Ok(mut pend) = state.terminal.pending.lock() {
                        let buf = pend.entry(term_id.clone()).or_default();
                        if buf.len() + data.len() > MAX_PENDING {
                            let excess = buf.len() + data.len() - MAX_PENDING;
                            buf.drain(..excess);
                        }
                        buf.push_str(&data);
                    }

                    // Line history for agent terminal tools (terminal_read_output),
                    // distinct from the UI `pending` buffer above.
                    if let Some(mut buffers) = state.terminal.buffers.try_lock().ok() {
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

    state.terminal.masters.lock().await.insert(id.clone(), master);
    state.terminal.writers.lock().await.insert(id.clone(), writer);
    state.terminal.processes.lock().await.insert(id, child);

    Ok(())
}

#[tauri::command]
pub async fn close_terminal(state: State<'_, std::sync::Arc<crate::EditorState>>, id: String) -> Result<(), String> {
    state.terminal.writers.lock().await.remove(&id);
    state.terminal.masters.lock().await.remove(&id);
    if let Some(mut child) = state.terminal.processes.lock().await.remove(&id) {
        let _ = child.kill();
    }
    state.terminal.buffers.lock().await.remove(&id);
    if let Ok(mut pend) = state.terminal.pending.lock() {
        pend.remove(&id);
    }
    Ok(())
}

#[tauri::command]
pub async fn terminal_send_data(

    state: State<'_, std::sync::Arc<crate::EditorState>>,
    id: String,
    data: String,
) -> Result<(), String> {
    let mut writers = state.terminal.writers.lock().await;
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
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    id: String,
    rows: u16,
    cols: u16,
) -> Result<(), String> {
    let masters = state.terminal.masters.lock().await;
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
pub async fn terminal_terminate(state: State<'_, std::sync::Arc<crate::EditorState>>, id: String) -> Result<(), String> {
    let mut processes = state.terminal.processes.lock().await;
    if let Some(mut child) = processes.remove(&id) {
        let _ = child.kill();
    }
    state.terminal.masters.lock().await.remove(&id);
    state.terminal.writers.lock().await.remove(&id);
    state.terminal.buffers.lock().await.remove(&id);
    Ok(())
}

#[tauri::command]
pub async fn terminal_get_status(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    id: String,
) -> Result<serde_json::Value, String> {
    let mut processes = state.terminal.processes.lock().await;
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
pub async fn terminal_read_output(state: State<'_, std::sync::Arc<crate::EditorState>>, id: String) -> Result<String, String> {
    state.terminal_read_output(id).await
}

/// Drain and return any PTY output produced since the last call. This is the
/// primary terminal transport — the frontend polls it because the global
/// `terminal-data` event stream does not reliably reach the webview.
#[tauri::command]
pub async fn terminal_take_pending(state: State<'_, std::sync::Arc<crate::EditorState>>, id: String) -> Result<String, String> {
    let out = {
        let mut pend = state
            .terminal.pending
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
        let mut shells = vec!["powershell.exe".to_string(), "cmd.exe".to_string()];
        if let Some(bash) = crate::ide_shell::resolve_git_bash_exe() {
            shells.push(bash.to_string_lossy().to_string());
        } else {
            shells.push("bash.exe".to_string());
        }
        Ok(shells)
    }
    #[cfg(not(windows))]
    {
        Ok(vec!["/bin/bash".to_string(), "/bin/zsh".to_string(), "/bin/sh".to_string()])
    }
}

/// Resolve the opencode binary and working directory.
/// Returns `(executable, args, cwd)`.
/// Priority:
///   1. `opencode` / `kilo` on PATH (globally installed)
///   2. `bun run --cwd <workspace>/opencode packages/cli/src/index.ts`
///   3. `bunx opencode` (no-install path)
fn resolve_opencode_launch(workspace_root: &PathBuf) -> (String, Vec<String>, Option<PathBuf>) {
    // 1. Global binary
    for name in &["opencode", "opencode.cmd", "kilo", "kilo.cmd"] {
        if exe_is_resolvable(name) {
            return (name.to_string(), vec![], Some(workspace_root.clone()));
        }
    }

    // 2. Local source in the repo (opencode/ or claurst/kilocode/)
    let candidates = [
        workspace_root.join("opencode").join("packages").join("cli").join("src").join("index.ts"),
        workspace_root.join("claurst").join("kilocode").join("packages").join("opencode").join("src").join("index.ts"),
    ];
    for entry in &candidates {
        if entry.exists() {
            if exe_is_resolvable("bun") {
                return (
                    "bun".to_string(),
                    vec!["run".to_string(), entry.to_string_lossy().to_string()],
                    Some(entry.parent().and_then(|p| p.parent()).and_then(|p| p.parent()).and_then(|p| p.parent()).unwrap_or(workspace_root).to_path_buf()),
                );
            }
        }
    }

    // 3. bunx fallback
    if exe_is_resolvable("bun") {
        return ("bun".to_string(), vec!["x".to_string(), "opencode".to_string()], Some(workspace_root.clone()));
    }
    if exe_is_resolvable("npx") {
        return ("npx".to_string(), vec!["--yes".to_string(), "opencode".to_string()], Some(workspace_root.clone()));
    }

    // Nothing found — return opencode directly; spawn will fail with a clear error
    ("opencode".to_string(), vec![], Some(workspace_root.clone()))
}

/// Spawn a new PTY session running the opencode TUI, pre-configured with the
/// IDE's current AI provider settings. The spawned terminal is identical to a
/// regular shell terminal — the frontend drives it through the same
/// `terminal_take_pending` / `terminal_send_data` / `resize_terminal` commands.
#[tauri::command]
pub async fn spawn_opencode_terminal(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    app: AppHandle,
    id: String,
) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize { rows: 24, cols: 220, pixel_width: 0, pixel_height: 0 })
        .map_err(|e: anyhow::Error| e.to_string())?;

    // Read IDE AI settings
    let local_url = state.ai.engine.lemonade_base().await;
    let current_model = state.ai.current_model.lock().await.clone();

    // Load API keys from disk
    let api_keys_path = state.config_dir.join("api_keys.json");
    let api_keys: serde_json::Value = if api_keys_path.exists() {
        let content = std::fs::read_to_string(&api_keys_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };

    // Resolve workspace root
    let workspace_root = {
        let root = state.editor.active_root.lock().await;
        root.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    };

    let (exe, args, cwd) = resolve_opencode_launch(&workspace_root);

    let mut cmd = CommandBuilder::new(&exe);
    for arg in &args {
        cmd.arg(arg);
    }

    // Always inject terminal basics
    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");

    // Provider env vars — Lemonade's OpenAI-compatible surface is the local
    // default. It is keyless, but the var must be non-empty or clients skip it.
    cmd.env("OPENAI_BASE_URL", format!("{}/api/v1", local_url.trim_end_matches('/')));
    cmd.env("OPENAI_API_KEY", "lemonade");

    // Layer on real cloud keys so opencode can detect + use them
    if let Some(key) = api_keys.get("anthropic").and_then(|v| v.as_str()) {
        if !key.is_empty() { cmd.env("ANTHROPIC_API_KEY", key); }
    }
    if let Some(url) = api_keys.get("anthropic_base_url").and_then(|v| v.as_str()) {
        if !url.is_empty() { cmd.env("ANTHROPIC_BASE_URL", url); }
    }
    if let Some(key) = api_keys.get("openai").and_then(|v| v.as_str()) {
        if !key.is_empty() {
            // Real OpenAI key overrides the ollama stub
            cmd.env("OPENAI_API_KEY", key);
            let base_url = api_keys.get("openai_base_url").and_then(|v| v.as_str()).unwrap_or("https://api.openai.com/v1");
            cmd.env("OPENAI_BASE_URL", base_url);
        }
    }
    if let Some(key) = api_keys.get("google").and_then(|v| v.as_str()) {
        if !key.is_empty() { cmd.env("GOOGLE_API_KEY", key); }
    }
    if let Some(key) = api_keys.get("groq").and_then(|v| v.as_str()) {
        if !key.is_empty() { cmd.env("GROQ_API_KEY", key); }
    }
    if let Some(key) = api_keys.get("openrouter").and_then(|v| v.as_str()) {
        if !key.is_empty() { cmd.env("OPENROUTER_API_KEY", key); }
    }
    if let Some(key) = api_keys.get("mistral").and_then(|v| v.as_str()) {
        if !key.is_empty() { cmd.env("MISTRAL_API_KEY", key); }
    }

    // Hint opencode at the workspace root + current model
    cmd.env("OPENCODE_CWD", workspace_root.to_string_lossy().as_ref());
    if !current_model.is_empty() {
        cmd.env("OPENCODE_MODEL", &current_model);
    }

    if let Some(ref cwd_path) = cwd {
        if cwd_path.is_dir() {
            cmd.cwd(cwd_path.as_os_str().to_owned());
        }
    }

    let child_result = pair.slave.spawn_command(cmd);
    let child = child_result.map_err(|e| {
        format!(
            "opencode not found. Install with: bun i -g opencode\nError: {}",
            e
        )
    })?;

    let writer = pair.master.take_writer().map_err(|e: anyhow::Error| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e: anyhow::Error| e.to_string())?;
    let master = pair.master;

    let app_handle = app.clone();
    let term_id = id.clone();
    std::thread::spawn(move || {
        let state = app_handle.state::<std::sync::Arc<crate::EditorState>>();
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Ok(mut pend) = state.terminal.pending.lock() {
                        let buf = pend.entry(term_id.clone()).or_default();
                        // Same 1MB drain cap as spawn_terminal — without it a
                        // chatty TUI grows this String unbounded if the UI
                        // stops polling.
                        if buf.len() + data.len() > MAX_PENDING {
                            let excess = buf.len() + data.len() - MAX_PENDING;
                            buf.drain(..excess);
                        }
                        buf.push_str(&data);
                    }
                    if let Some(mut buffers) = state.terminal.buffers.try_lock().ok() {
                        let history = buffers.entry(term_id.clone()).or_insert_with(Vec::new);
                        history.push(data);
                        if history.len() > 2000 { history.drain(0..500); }
                    }
                }
                _ => break,
            }
        }
    });

    state.terminal.masters.lock().await.insert(id.clone(), master);
    state.terminal.writers.lock().await.insert(id.clone(), writer);
    state.terminal.processes.lock().await.insert(id, child);

    Ok(())
}

/// Locate the Claude Code CLI. Global install first, then a bunx/npx fallback so
/// the terminal still comes up on a machine where it was never installed.
///
/// **Order matters on Windows.** npm installs three siblings: a bare `claude`
/// (a `#!/bin/sh` script), `claude.cmd`, and `claude.ps1`. Only the `.cmd` is
/// executable by `CreateProcessW`, which is what both `std::process::Command`
/// and portable_pty end up calling. Picking the extensionless script fails with
/// "cannot find the file specified" under `Command`, and — worse — ConPTY
/// returns Ok and the child dies instantly, leaving a blank terminal.
pub(crate) fn resolve_claude_launch(workspace_root: &PathBuf) -> (String, Vec<String>) {
    #[cfg(windows)]
    const CANDIDATES: &[&str] = &["claude.cmd", "claude.exe", "claude"];
    #[cfg(not(windows))]
    const CANDIDATES: &[&str] = &["claude"];

    // Prefer the NATIVE .exe over the `.cmd` shim.
    //
    // `claude.cmd` is a batch file whose last line invokes the real
    // `claude.exe`. Running it means `cmd.exe` sits between us and the CLI, and
    // cmd.exe allocates a console — so a black window flashes on every single
    // chat message, which looks like the IDE is shelling out behind the user's
    // back. `CREATE_NO_WINDOW` on our direct child does not reliably suppress
    // the console the shim's own child ends up with.
    //
    // The exe is not on PATH (npm only links the three shims), so probe the npm
    // global layout directly. Skipping the shim also removes a process from the
    // tree, which makes kill-on-stop more reliable.
    #[cfg(windows)]
    if let Some(exe) = native_claude_exe() {
        return (exe, vec![]);
    }

    for name in CANDIDATES {
        if exe_is_resolvable(name) {
            return (name.to_string(), vec![]);
        }
    }
    let _ = workspace_root;
    #[cfg(windows)]
    let (bunx, npx) = ("bunx.cmd", "npx.cmd");
    #[cfg(not(windows))]
    let (bunx, npx) = ("bunx", "npx");

    if exe_is_resolvable(bunx) {
        return (bunx.to_string(), vec!["@anthropic-ai/claude-code".to_string()]);
    }
    if exe_is_resolvable(npx) {
        return (
            npx.to_string(),
            vec!["--yes".to_string(), "@anthropic-ai/claude-code".to_string()],
        );
    }
    (CANDIDATES[0].to_string(), vec![])
}

/// Path to Claude Code's real `claude.exe`, bypassing npm's `.cmd` shim.
///
/// npm links `claude`, `claude.cmd` and `claude.ps1` into its global bin, but the
/// actual binary lives under `node_modules/@anthropic-ai/claude-code/bin/`. Only
/// the shims are on PATH, so this has to be probed by path.
///
/// Honours `CLAUDE_CODE_EXE` first, so a non-standard install (bun, volta, a
/// portable copy) can point at its own binary instead of falling back to the shim.
#[cfg(windows)]
pub(crate) fn native_claude_exe() -> Option<String> {
    const REL: &str = r"node_modules\@anthropic-ai\claude-code\bin\claude.exe";

    if let Ok(explicit) = std::env::var("CLAUDE_CODE_EXE") {
        let p = PathBuf::from(&explicit);
        if p.is_file() {
            return Some(explicit);
        }
    }

    let mut roots: Vec<PathBuf> = Vec::new();
    if let Ok(appdata) = std::env::var("APPDATA") {
        roots.push(PathBuf::from(appdata).join("npm"));
    }
    if let Some(home) = dirs::home_dir() {
        roots.push(home.join(".bun").join("install").join("global"));
        roots.push(home.join("AppData").join("Roaming").join("npm"));
    }
    if let Ok(prefix) = std::env::var("NPM_CONFIG_PREFIX") {
        roots.push(PathBuf::from(prefix));
    }

    roots
        .into_iter()
        .map(|r| r.join(REL))
        .find(|p| p.is_file())
        .map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod claude_launch_tests {
    use super::*;

    /// The `.cmd` shim routes through cmd.exe, which allocates a console — a
    /// black window flashing on every chat message. When the native exe is
    /// present it must win.
    #[cfg(windows)]
    #[test]
    fn native_exe_is_preferred_over_the_cmd_shim() {
        let Some(exe) = native_claude_exe() else {
            // Claude Code not installed in a standard location on this machine;
            // nothing to assert, and the shim fallback is correct there.
            return;
        };
        assert!(exe.to_lowercase().ends_with("claude.exe"), "got {exe}");
        assert!(std::path::Path::new(&exe).is_file());

        let (chosen, args) = resolve_claude_launch(&PathBuf::from("."));
        assert_eq!(chosen, exe, "resolver must pick the native exe, not the shim");
        assert!(args.is_empty(), "native exe takes no wrapper args, got {args:?}");
        assert!(
            !chosen.to_lowercase().ends_with(".cmd"),
            "the .cmd shim allocates a console window on every spawn"
        );
    }

    /// On Windows the extensionless `claude` is a `#!/bin/sh` script that
    /// `CreateProcessW` cannot run — the `.cmd` shim must win.
    #[test]
    fn windows_never_picks_the_bare_sh_script() {
        let (exe, _) = resolve_claude_launch(&PathBuf::from("."));
        if cfg!(windows) {
            assert_ne!(exe, "claude", "bare `claude` is a sh script on Windows");
        }
    }
}

/// Spawn a PTY running the **Claude Code CLI against the local Lemonade server**.
///
/// Rather than growing our own agent loop, this hands the workspace to a harness
/// that already has mature tool calling, permission handling, hooks and skills,
/// and points its inference at Lemonade on `:13305`. The spawned terminal is an
/// ordinary IDE terminal — the frontend drives it through the same
/// `terminal_take_pending` / `terminal_send_data` / `resize_terminal` commands.
///
/// Before spawning we apply the measured per-model `ctx_size` + `llamacpp.args`
/// (see `commands::ai::apply_lemonade_tuning`). Those are global to the Lemonade
/// server and only take effect at load, so a model left loaded under a different
/// model's settings runs up to 4x slower with no error.
///
/// - `skip_permissions` (default **true**) passes `--dangerously-skip-permissions`.
///   The IDE's own tool layer already runs without governance gates, and the
///   workspace is a git repo, so `git diff` / `git checkout .` stay the review and
///   undo mechanism.
/// - `allow_net` (default **false**) gates outbound internet. When false, proxy
///   env vars point at a dead port with loopback exempted, so inference still
///   reaches Lemonade but nothing leaves the machine. `WebFetch`/`WebSearch`
///   require it set to true.
#[tauri::command]
pub async fn spawn_claude_terminal(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    app: AppHandle,
    id: String,
    model: Option<String>,
    skip_permissions: Option<bool>,
    allow_net: Option<bool>,
    extra_args: Option<Vec<String>>,
) -> Result<(), String> {
    let workspace_root = {
        let root = state.editor.active_root.lock().await;
        root.clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    };

    // Resolve the model: explicit argument, else whatever the IDE has selected.
    let model = match model.map(|m| sanitize_no_nul(&m)).filter(|m| !m.is_empty()) {
        Some(m) => m,
        None => sanitize_no_nul(&state.ai.current_model.lock().await.clone()),
    };
    if model.is_empty() {
        return Err("No model selected. Pick a Lemonade model in Settings → Inference Backend first.".to_string());
    }
    // Normalize a `…:latest` id from Lemonade's Ollama-compat tag list.
    let model = super::ai::canonical_model_id(&model).to_string();

    let lemonade_base = state.ai.engine.lemonade_base().await;
    let lemonade_base = lemonade_base.trim_end_matches('/').to_string();

    // The IDE's selected model defaults to a cloud name (`gpt-4o`) and can be an
    // Ollama tag, neither of which Lemonade serves. Spawning anyway produces a
    // session where every request 404s with no visible cause, so validate first.
    // An unreachable server yields an empty list — treat that as "unknown" and
    // let the spawn proceed rather than blocking on a health check.
    let served = super::ai::lemonade_served_models(&lemonade_base).await;
    if !served.is_empty() && !served.iter().any(|m| m == &model) {
        let mut available = served.clone();
        available.sort();
        return Err(format!(
            "Lemonade does not serve '{}'. Pick a local model in Settings → Inference Backend.\nAvailable: {}",
            model,
            available.join(", ")
        ));
    }

    // A model that cannot call tools makes Claude Code loop on malformed calls
    // forever, so refuse up front with a name rather than hanging the session.
    if !super::ai::supports_tool_calling(&model) {
        return Err(format!(
            "Model '{}' cannot reliably call tools (measured 0/6), so Claude Code would loop on \
             malformed tool calls. Pick a tool-calling model — see the Lemonade notes in MEMORY.md.",
            model
        ));
    }

    // Global, load-time-only Lemonade settings. Returns expected tok/s (0.0 = unmeasured).
    let _expected_tps = super::ai::apply_lemonade_tuning(&lemonade_base, &model).await;

    let (exe, args) = resolve_claude_launch(&workspace_root);
    let mut cmd = CommandBuilder::new(&exe);
    for arg in &args {
        cmd.arg(arg);
    }

    let skip_permissions = skip_permissions.unwrap_or(true);
    if skip_permissions {
        cmd.arg("--dangerously-skip-permissions");
    }
    for arg in extra_args.unwrap_or_default() {
        let a = sanitize_no_nul(&arg);
        if !a.is_empty() {
            cmd.arg(a);
        }
    }

    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");

    // Lemonade exposes the Anthropic-compatible API at `/v1/messages`, so the base
    // URL is the bare host:port — do NOT append `/api/v1` (that is the
    // OpenAI-compat path and yields 404s on every request).
    cmd.env("ANTHROPIC_BASE_URL", &lemonade_base);
    // Any non-empty value; Lemonade ignores it. Both keys are set explicitly so a
    // real cloud key inherited from the parent environment cannot hijack routing
    // and silently bill the user's Anthropic account.
    cmd.env("ANTHROPIC_AUTH_TOKEN", "lemonade");
    cmd.env("ANTHROPIC_API_KEY", "lemonade");

    // Every alias must resolve locally. Without the haiku/sonnet/opus mappings,
    // anything that asks for "sonnet" (subagents, plan mode, /model) resolves to a
    // cloud model name Lemonade does not serve, and the request fails.
    cmd.env("ANTHROPIC_MODEL", &model);
    cmd.env("ANTHROPIC_SMALL_FAST_MODEL", &model);
    cmd.env("ANTHROPIC_DEFAULT_HAIKU_MODEL", &model);
    cmd.env("ANTHROPIC_DEFAULT_SONNET_MODEL", &model);
    cmd.env("ANTHROPIC_DEFAULT_OPUS_MODEL", &model);

    // Claude Code defaults to max_tokens=32000, which would eat a 32k context
    // window whole. A LARGER budget is not safer here: llama.cpp enforces
    // `prompt + max_tokens <= n_ctx` on every request, so output budget is headroom
    // taken from the prompt — and compaction, the request with the biggest prompt,
    // is the one that then fails.
    let (ctx_size, _, _) = super::ai::lemonade_tuning(&model);
    let max_out = super::ai::claude_max_output_tokens();
    cmd.env("CLAUDE_CODE_MAX_OUTPUT_TOKENS", max_out.to_string());
    // Claude Code does not recognise local model ids and assumes a 200k window,
    // sizing auto-compact to that. The served window is `ctx_size`, and going
    // past it returns empty HTTP 200s (llama.cpp 400 rewritten as success), so
    // the agent would go silent mid-session with no error.
    //
    // Telling it the literal truth is still wrong, though: it compacts at ~80% of
    // whatever it is told, which at 98,304 means waiting until ~78,600 — and with
    // ~36,700 tokens of system prompt and tools on top, that compaction request
    // overflows the window and the session can no longer save itself.
    cmd.env(
        "CLAUDE_CODE_MAX_CONTEXT_TOKENS",
        super::ai::claude_max_context_tokens(ctx_size).to_string(),
    );
    // Same reason as the chat path: the default request timeout is shorter than a
    // local model takes to finish a long answer, and the retry queues behind it.
    let (_, _, expected_tps) = super::ai::lemonade_tuning(&model);
    cmd.env(
        "API_TIMEOUT_MS",
        super::ai::claude_api_timeout_ms(expected_tps, max_out).to_string(),
    );
    cmd.env("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1");

    // Airgap: even with the flag above, the CLI still opens HTTPS to Anthropic and
    // a telemetry host. Point all proxying at a dead port and exempt loopback, so
    // inference reaches Lemonade but nothing leaves the machine.
    if !allow_net.unwrap_or(false) {
        cmd.env("HTTPS_PROXY", "http://127.0.0.1:1");
        cmd.env("https_proxy", "http://127.0.0.1:1");
        cmd.env("HTTP_PROXY", "http://127.0.0.1:1");
        cmd.env("http_proxy", "http://127.0.0.1:1");
        cmd.env("NO_PROXY", "localhost,127.0.0.1,::1");
        cmd.env("no_proxy", "localhost,127.0.0.1,::1");
    }

    if workspace_root.is_dir() {
        cmd.cwd(workspace_root.as_os_str().to_owned());
    }

    println!(
        "[claude-code] {} -> {} ({}), ctx {}, max_out {}, {}, {}",
        exe,
        lemonade_base,
        model,
        ctx_size,
        max_out,
        if allow_net.unwrap_or(false) { "net ALLOWED" } else { "airgapped" },
        if skip_permissions { "skip-perms" } else { "prompts on" },
    );
    if expected_tps > 0.0 {
        println!("[claude-code] expected ~{:.1} tok/s for {}", expected_tps, model);
    }

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize { rows: 24, cols: 220, pixel_width: 0, pixel_height: 0 })
        .map_err(|e: anyhow::Error| e.to_string())?;

    let child = pair.slave.spawn_command(cmd).map_err(|e| {
        format!(
            "Claude Code not found. Install with: npm i -g @anthropic-ai/claude-code\nError: {}",
            e
        )
    })?;

    let writer = pair.master.take_writer().map_err(|e: anyhow::Error| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e: anyhow::Error| e.to_string())?;
    let master = pair.master;

    let app_handle = app.clone();
    let term_id = id.clone();
    std::thread::spawn(move || {
        let state = app_handle.state::<std::sync::Arc<crate::EditorState>>();
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    if let Ok(mut pend) = state.terminal.pending.lock() {
                        let buf = pend.entry(term_id.clone()).or_default();
                        if buf.len() + data.len() > MAX_PENDING {
                            let excess = buf.len() + data.len() - MAX_PENDING;
                            buf.drain(..excess);
                        }
                        buf.push_str(&data);
                    }
                    if let Ok(mut buffers) = state.terminal.buffers.try_lock() {
                        let history = buffers.entry(term_id.clone()).or_insert_with(Vec::new);
                        history.push(data);
                        if history.len() > 2000 { history.drain(0..500); }
                    }
                }
                _ => break,
            }
        }
    });

    state.terminal.masters.lock().await.insert(id.clone(), master);
    state.terminal.writers.lock().await.insert(id.clone(), writer);
    state.terminal.processes.lock().await.insert(id, child);

    Ok(())
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
    ///   cargo test pty_smoke -- --ignored --nocapture
    #[test]
    #[ignore = "hangs on some Windows hosts — run manually with --ignored"]
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
