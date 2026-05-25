use serde_json::{json, Value};

pub fn capture_main_screenshot(_app: &tauri::AppHandle) -> Result<Value, String> {
    Ok(json!({
        "ok": false,
        "mode": "compatibility",
        "message": "vision_bridge.rs was missing; screenshot capture is temporarily disabled",
    }))
}

#[tauri::command]
pub async fn capture_preview_screenshot() -> Result<Value, String> {
    Ok(json!({
        "ok": false,
        "mode": "compatibility",
        "message": "Preview screenshot capture is temporarily disabled",
    }))
}
