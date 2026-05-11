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

/// Open the login flow for a cloud provider. The frontend popover for
/// "☁️ Login to Claude (Browser)" / "💎 Login to Gemini (Browser)" calls into
/// here.
///
/// Modes:
///   - `"api_key"` (default; what we want 99% of the time): opens the
///     provider's API key console in the user's *default browser*. They
///     copy a key from there into the Cloud API Keys section of Settings.
///     This is the only mode that survives modern auth — Anthropic and
///     Google both use httpOnly cookies for chat-product sessions, so the
///     WebView scrape path can't actually see them.
///   - `"webview"`: opens a Tauri WebView pointed at the chat product
///     (claude.ai / gemini.google.com). Kept for the historical "use my
///     paid subscription" scrape; only useful for non-httpOnly cookies.
///
/// Returns the URL that was opened so the caller can echo it in a toast.
#[tauri::command]
pub async fn open_ai_login(
    app: tauri::AppHandle,
    provider: String,
    mode: Option<String>,
) -> Result<String, String> {
    let mode = mode.unwrap_or_else(|| "api_key".to_string());
    match mode.as_str() {
        "api_key" => crate::ai_auth::open_api_key_console(&provider),
        "webview" => {
            crate::ai_auth::open_login_window(app, provider).await?;
            Ok("opened-in-webview".to_string())
        }
        other => Err(format!("Unknown login mode '{}'. Use 'api_key' or 'webview'.", other)),
    }
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
