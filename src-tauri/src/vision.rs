use serde_json::{json, Value};

#[tauri::command]
pub async fn airi_vision_analyze_screen() -> Result<Value, String> {
    Ok(json!({
        "ok": false,
        "mode": "compatibility",
        "message": "AIRI vision analysis is temporarily disabled",
    }))
}

#[tauri::command]
pub async fn airi_vision_capture_screen() -> Result<Value, String> {
    Ok(json!({
        "ok": false,
        "mode": "compatibility",
        "message": "AIRI screen capture is temporarily disabled",
    }))
}
