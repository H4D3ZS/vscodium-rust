use tauri::State;
use crate::EditorState;
use serde_json::{json, Value};

#[tauri::command]
pub async fn debug_start(
    app: tauri::AppHandle,
    state: State<'_, EditorState>,
    config: Value,
) -> Result<(), String> {
    let mut dm = state.debug_manager.lock().await;
    let adapter_path = config["adapter_path"].as_str().ok_or("adapter_path required")?;
    dm.start_session(adapter_path, app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn debug_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut dm = state.debug_manager.lock().await;
    dm.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn debug_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut dm = state.debug_manager.lock().await;
    dm.stop_session().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_file_symbols(
    _state: State<'_, EditorState>,
    _path: String,
) -> Result<Value, String> {
    // Symbol analysis placeholder
    Ok(json!({ "symbols": [] }))
}
