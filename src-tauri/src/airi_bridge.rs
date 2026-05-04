use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};

/// A bridge to the AIRI visual manifold using native Tauri IPC.
#[derive(Clone)]
pub struct AiriBridge {
    app_handle: Arc<Mutex<Option<AppHandle>>>,
}

impl AiriBridge {
    pub fn new() -> Self {
        Self {
            app_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Primary initialization for the IPC bridge.
    pub async fn init(&self, handle: AppHandle) {
        let mut lock = self.app_handle.lock().await;
        *lock = Some(handle);
        println!("[AIRI] Native IPC Bridge initialized.");
    }

    /// Connection method for backward compatibility.
    pub async fn connect(&self, _url: &str) -> anyhow::Result<()> {
        Ok(())
    }

    /// Synchronizes internal reasoning states to the visual avatar using native Tauri events.
    pub async fn sync_state(&self, verity: f32, focus: f32, node: Option<String>) -> anyhow::Result<()> {
        let payload = json!({
            "mood": if verity >= 1.0 { "happy" } else if verity > 0.5 { "think" } else { "sad" },
            "focus": focus,
            "eye_track": node,
            "speech_lock": verity < 1.0, 
            "source": "hades:core"
        });

        let lock = self.app_handle.lock().await;
        if let Some(handle) = lock.as_ref() {
            // High-speed IPC broadcast
            let _ = handle.emit("hades-sync", payload);
        }
        Ok(())
    }
}
