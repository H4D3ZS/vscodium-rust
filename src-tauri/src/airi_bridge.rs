use serde_json::{json, Value};

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
