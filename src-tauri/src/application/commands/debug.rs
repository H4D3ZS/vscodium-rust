use tauri::State;
use serde_json::Value;

fn resolve_debug_adapter(config: &Value) -> Result<String, String> {
    if let Some(p) = config.get("adapter_path").and_then(|v| v.as_str()) {
        return Ok(p.to_string());
    }
    let dtype = config.get("type").and_then(|v| v.as_str()).unwrap_or("");
    match dtype {
        "node" | "pwa-node" => which::which("node-debug2-adapter")
            .or_else(|_| which::which("node"))
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|_| "Node debug adapter not found. Install 'js-debug' or set adapter_path in launch.json".into()),
        "cppdbg" | "lldb" => which::which("OpenDebugAD7")
            .or_else(|_| which::which("lldb-vscode"))
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|_| "C++ debug adapter not found — install CodeLLDB or cpptools".into()),
        _ => Err(format!(
            "Unknown debug type '{dtype}'. Add \"adapter_path\" to launch.json or install a debug adapter extension."
        )),
    }
}

#[tauri::command]
pub async fn debug_start(
    app: tauri::AppHandle,
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    config: Value,
) -> Result<(), String> {
    let adapter_path = resolve_debug_adapter(&config)?;
    let mut dm = state.ext.debug.lock().await;
    let init_rx = dm.start_session(&adapter_path, app).map_err(|e| e.to_string())?;
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
