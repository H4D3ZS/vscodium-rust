use tauri::State;
use crate::EditorState;

#[tauri::command]
pub async fn backend_ping() -> String {
    "PONG".to_string()
}

#[tauri::command]
pub fn get_config_path(state: State<'_, EditorState>) -> String {
    state.config_dir.to_string_lossy().to_string()
}

#[tauri::command]
pub async fn open_ai_login(app: tauri::AppHandle, provider: String) -> Result<(), String> {
    crate::ai_auth::open_login_window(app, provider).await
}

#[tauri::command]
pub fn get_yolo_mode(state: State<'_, EditorState>) -> bool {
    state.ai_engine.is_yolo_mode()
}

#[tauri::command]
pub fn set_yolo_mode(state: State<'_, EditorState>, enabled: bool) {
    state.ai_engine.set_yolo_mode(enabled);
}

#[tauri::command]
pub fn start_mitm_server() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn stop_mitm_server() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn get_mitm_status() -> String {
    "idle".to_string()
}
