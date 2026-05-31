use serde_json::{json, Value};
use tauri::Emitter;

/// Forward an AIRI avatar event from the agent runtime to the webview. The
/// AIRI overlay listens for namespaced Tauri events (`airi:thought`,
/// `airi:edit_proposed`, `airi:phase_wrap`, `airi:vision_frame`, …) and
/// animates the VRM avatar accordingly. The frontend's DOM `CustomEvent`
/// path does NOT reach Tauri `listen()`, so this command is what actually
/// drives the avatar — it re-emits as the namespaced `airi:{event}` the
/// overlay subscribes to. Previously no handler existed and every avatar
/// reaction silently no-op'd.
#[tauri::command]
pub async fn airi_event(
    app: tauri::AppHandle,
    event: String,
    payload: Option<Value>,
) -> Result<(), String> {
    let body = payload.unwrap_or(Value::Null);
    // Namespaced event the AiriOverlay listens for (e.g. "airi:thought").
    app.emit(&format!("airi:{}", event), body.clone())
        .map_err(|e| e.to_string())?;
    // Also a generic channel for any consumer that wants the raw stream.
    let _ = app.emit("airi-event", json!({ "event": event, "payload": body }));
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct AiriBridge;

impl AiriBridge {
    pub fn new() -> Self {
        Self
    }

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
