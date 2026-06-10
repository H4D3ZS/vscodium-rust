use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

pub struct ClaudeBridgeManager {
    bridge: Arc<Mutex<Option<ClaudeBridge>>>,
}

impl ClaudeBridgeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bridge = Arc::new(Mutex::new(Some(ClaudeBridge::new()?)));
        Ok(Self { bridge })
    }

    pub async fn send_prompt(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut bridge = self.bridge.lock().await;
        if let Some(ref mut bridge) = *bridge {
            bridge.send_prompt(prompt).await
        } else {
            Err("Claude bridge not initialized".into())
        }
    }
}

pub struct GeminiBridgeManager {
    bridge: Arc<Mutex<Option<GeminiBridge>>>,
}

impl GeminiBridgeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bridge = Arc::new(Mutex::new(Some(GeminiBridge::new()?)));
        Ok(Self { bridge })
    }

    pub async fn send_prompt(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut bridge = self.bridge.lock().await;
        if let Some(ref mut bridge) = *bridge {
            bridge.send_prompt(prompt).await
        } else {
            Err("Gemini bridge not initialized".into())
        }
    }
}