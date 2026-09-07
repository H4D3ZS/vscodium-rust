use tauri::State;
use serde_json::Value;

/// First binary on PATH from `names`, or `None`.
fn first_on_path(names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|n| which::which(n).ok())
        .map(|p| p.to_string_lossy().to_string())
}

/// Resolve `(adapter_binary, adapter_args)` for a `launch.json` config. Honors
/// an explicit `adapter_path` (+ optional `adapter_args`) override; otherwise
/// maps the well-known `type` values to their DAP entry points.
fn resolve_debug_adapter(config: &Value) -> Result<(String, Vec<String>), String> {
    let explicit_args = || -> Vec<String> {
        config
            .get("adapter_args")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
            .unwrap_or_default()
    };
    if let Some(p) = config.get("adapter_path").and_then(|v| v.as_str()) {
        return Ok((p.to_string(), explicit_args()));
    }

    let dtype = config.get("type").and_then(|v| v.as_str()).unwrap_or("");
    match dtype {
        // ── JavaScript / TypeScript (vscode-js-debug) ──
        "node" | "pwa-node" | "chrome" | "pwa-chrome" | "msedge" | "pwa-msedge" | "node-terminal" => {
            first_on_path(&["js-debug-adapter", "dapDebugServer", "node-debug2-adapter"])
                .map(|p| (p, vec![]))
                .or_else(|| first_on_path(&["node"]).map(|p| (p, vec![])))
                .ok_or_else(|| "JS debugger not found — install `js-debug-adapter` (npm i -g @vscode/js-debug) or set adapter_path".into())
        }
        // ── Python (debugpy) ──
        "python" | "debugpy" => {
            let py = config
                .get("python")
                .and_then(|v| v.as_str())
                .map(String::from)
                .or_else(|| first_on_path(&["python3", "python", "py"]))
                .ok_or("Python not found — install Python or set `python` in the config")?;
            // `python -m debugpy.adapter` speaks DAP on stdio.
            Ok((py, vec!["-m".into(), "debugpy.adapter".into()]))
        }
        // ── Go (delve) ──
        "go" | "delve" | "dlv" => first_on_path(&["dlv"])
            .map(|p| (p, vec!["dap".into()]))
            .ok_or_else(|| "Delve not found — `go install github.com/go-delve/delve/cmd/dlv@latest`".into()),
        // ── Rust / C / C++ via CodeLLDB (stdio mode) ──
        "lldb" | "codelldb" | "rust" | "rust-lldb" => first_on_path(&["codelldb", "lldb-dap", "lldb-vscode"])
            .map(|p| (p, vec![]))
            .ok_or_else(|| "CodeLLDB not found — install the CodeLLDB adapter or `lldb-dap`, or set adapter_path".into()),
        // ── C / C++ via cpptools (OpenDebugAD7 / gdb-mi wrapper) ──
        "cppdbg" | "cppvsdbg" | "gdb" => first_on_path(&["OpenDebugAD7", "cppdbg"])
            .map(|p| (p, vec![]))
            .ok_or_else(|| "C/C++ debugger not found — install cpptools (OpenDebugAD7) or use `type: \"lldb\"` with CodeLLDB".into()),
        // ── .NET (netcoredbg) ──
        "coreclr" | "clr" | "dotnet" | "netcoredbg" => first_on_path(&["netcoredbg"])
            .map(|p| (p, vec!["--interpreter=vscode".into()]))
            .ok_or_else(|| "netcoredbg not found — https://github.com/Samsung/netcoredbg/releases".into()),
        // ── PHP (Xdebug / vscode-php-debug) ──
        "php" => first_on_path(&["php-debug-adapter"])
            .map(|p| (p, vec![]))
            .ok_or_else(|| "PHP debug adapter not found — install `vscode-php-debug`".into()),
        // ── Ruby (rdbg / debug gem) ──
        "ruby" | "rdbg" => first_on_path(&["rdbg"])
            .map(|p| (p, vec!["--open".into(), "--stop-at-load".into()]))
            .ok_or_else(|| "rdbg not found — `gem install debug`".into()),
        // ── LLDB alias kept for back-compat with older launch.json ──
        "" => Err("launch.json entry has no `type`. Set one (node, python, go, lldb, cppdbg, coreclr, …) or an `adapter_path`.".into()),
        _ => Err(format!(
            "Unknown debug type '{dtype}'. Add `adapter_path` (+ optional `adapter_args`) to the config, or install its debug adapter."
        )),
    }
}

#[tauri::command]
pub async fn debug_start(
    app: tauri::AppHandle,
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    config: Value,
) -> Result<(), String> {
    let (adapter_path, adapter_args) = resolve_debug_adapter(&config)?;
    let mut dm = state.ext.debug.lock().await;
    let init_rx = dm
        .start_session(&adapter_path, &adapter_args, app)
        .map_err(|e| e.to_string())?;
    let adapter_id = config.get("type").and_then(|v| v.as_str()).unwrap_or("generic");
    let init = serde_json::json!({
        "type": "request",
        "command": "initialize",
        "arguments": {
            "clientID": "vscodium-rust",
            "clientName": "VSCodium-Rust",
            "adapterID": adapter_id,
            "pathFormat": "path",
            "linesStartAt1": true,
            "columnsStartAt1": true
        },
        "seq": 1
    });
    dm.send_message(init.to_string()).map_err(|e| e.to_string())?;

    dm.wait_for_initialize(init_rx, std::time::Duration::from_secs(15))
        .map_err(|e| e.to_string())?;

    let initialized = serde_json::json!({
        "type": "event",
        "event": "initialized",
        "seq": 2
    });
    dm.send_message(initialized.to_string()).map_err(|e| e.to_string())?;

    let config_done = serde_json::json!({
        "type": "request",
        "command": "configurationDone",
        "seq": 3
    });
    dm.send_message(config_done.to_string()).map_err(|e| e.to_string())?;

    let launch = serde_json::json!({
        "type": "request",
        "command": "launch",
        "arguments": config,
        "seq": 4
    });
    dm.send_message(launch.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn debug_send(state: State<'_, std::sync::Arc<crate::EditorState>>, msg: String) -> Result<(), String> {
    let mut dm = state.ext.debug.lock().await;
    dm.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn debug_stop(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    let mut dm = state.ext.debug.lock().await;
    dm.stop_session().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_file_symbols(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
) -> Result<Value, String> {
    let root = state.editor.active_root.lock().await.clone();
    let full = if std::path::Path::new(&path).is_absolute() {
        std::path::PathBuf::from(&path)
    } else if let Some(r) = root {
        std::path::PathBuf::from(r).join(&path)
    } else {
        std::path::PathBuf::from(&path)
    };
    if !full.exists() {
        return Err(format!("File not found: {}", full.display()));
    }
    crate::symbols::analyze_to_json(&full)
}
