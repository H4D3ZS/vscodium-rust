//! HADES Real-Time Vision System - Stubbed lightweight implementation

use std::sync::Arc;
use tokio::sync::RwLock;

#[allow(dead_code)]
pub struct VisionState {
    pub last_analysis: String,
    pub last_update: u64,
    pub frames_captured: u32,
}

#[allow(dead_code)]
pub struct HadesVision {
    state: Arc<RwLock<VisionState>>,
    inference_url: String,
    local_model: String,
    cloud_model: String,
    use_cloud: Arc<RwLock<bool>>,
    client: reqwest::Client,
}

impl HadesVision {
    pub fn new(inference_url: &str, local_model: &str, cloud_model: &str, use_cloud: bool) -> Self {
        Self {
            state: Arc::new(RwLock::new(VisionState {
                last_analysis: "Vision is stubbed to conserve memory".to_string(),
                last_update: 0,
                frames_captured: 0,
            })),
            inference_url: inference_url.to_string(),
            local_model: local_model.to_string(),
            cloud_model: cloud_model.to_string(),
            use_cloud: Arc::new(RwLock::new(use_cloud)),
            client: reqwest::Client::builder()
                .tcp_keepalive(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn capture_and_analyze(&self) -> Result<String, String> {
        Ok("Vision system is a stub. Local image analysis is disabled to save RAM.".to_string())
    }

    pub async fn get_last_analysis(&self) -> String {
        self.state.read().await.last_analysis.clone()
    }

    pub async fn switch_to_cloud(&self) {
        *self.use_cloud.write().await = true;
    }

    pub async fn switch_to_local(&self) {
        *self.use_cloud.write().await = false;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS
// ═══════════════════════════════════════════════════════════════════════════

use std::sync::OnceLock;
static HADES_VISION: OnceLock<Arc<HadesVision>> = OnceLock::new();

pub fn init_hades_vision(_target_fps: u32, inference_url: &str, local_model: &str, cloud_model: &str, use_cloud: bool) {
    let _ = HADES_VISION.set(Arc::new(HadesVision::new(inference_url, local_model, cloud_model, use_cloud)));
}

#[tauri::command]
pub async fn hades_vision_get_current_view() -> Result<String, String> {
    if let Some(vision) = HADES_VISION.get() {
        vision.capture_and_analyze().await 
    } else {
        Err("Vision not initialized".to_string())
    }
}

#[tauri::command]
pub async fn hades_vision_get_temporal_analysis(_frame_count: usize) -> Result<String, String> {
    if let Some(vision) = HADES_VISION.get() {
        Ok(vision.get_last_analysis().await)
    } else {
        Err("Vision not initialized".to_string())
    }
}

#[tauri::command]
pub async fn hades_vision_switch_to_cloud() {
    if let Some(v) = HADES_VISION.get() { 
        v.switch_to_cloud().await; 
    }
}

#[tauri::command]
pub async fn hades_vision_switch_to_local() {
    if let Some(v) = HADES_VISION.get() { 
        v.switch_to_local().await; 
    }
}

