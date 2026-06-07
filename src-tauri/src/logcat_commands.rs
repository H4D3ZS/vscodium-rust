//! Logcat streaming — thin Tauri adapters.

use crate::EditorState;
use serde_json::json;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn logcat_start(
    state: State<'_, EditorState>,
    app: AppHandle,
    device: Option<String>,
    filter: Option<String>,
) -> Result<serde_json::Value, String> {
    state.logcat_service.start(app, device, filter)?;
    Ok(json!({ "status": "running" }))
}

#[tauri::command]
pub async fn logcat_stop(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    state.logcat_service.stop()?;
    Ok(json!({ "status": "stopped" }))
}

#[tauri::command]
pub async fn logcat_status(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    Ok(json!({ "running": state.logcat_service.is_running() }))
}
