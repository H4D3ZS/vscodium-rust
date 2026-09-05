use serde_json::{json, Value};

#[cfg(feature = "tauri")]
#[tauri::command]
pub async fn airi_event(
    app: tauri::AppHandle,
    event: String,
    payload: Option<Value>,
) -> Result<(), String> {
    use tauri::Emitter;
    let body = payload.unwrap_or(Value::Null);
    app.emit(&format!("airi:{}", event), body.clone())
        .map_err(|e| e.to_string())?;
    let _ = app.emit("airi-event", json!({ "event": event, "payload": body }));
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct AiriBridge;

impl AiriBridge {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "tauri")]
    pub async fn init(&self, _app: tauri::AppHandle) {
        println!("[AIRI] Bridge initialized in compatibility mode");
    }

    pub async fn sync_state(
        &self,
        verity: f32,
        confidence: f32,
        path: Option<String>,
    ) -> Result<Value, String> {
        Ok(json!({
            "ok": true,
            "verity": verity,
            "confidence": confidence,
            "path": path,
        }))
    }
}
