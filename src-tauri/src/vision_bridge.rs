use tauri::{AppHandle, Manager, Runtime};
use serde::{Deserialize, Serialize};
use base64;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotResult {
    pub status: String,
    pub image_base64: String,
    pub width: u32,
    pub height: u32,
}

/// Capture main window screenshot (for HUD overlay)
pub fn capture_main_screenshot<R: Runtime>(_app: &AppHandle<R>) -> Result<ScreenshotResult, String> {
    // Use the full-screen capture from vision.rs (desktop capture)
    // AIRI needs full desktop awareness, not just the app window
    crate::vision::capture_screen()
        .map(|png_bytes| {
            // Decode to get dimensions (or use defaults)
            let (width, height) = (1920, 1080); // Placeholder; actual decode would give real dims
            ScreenshotResult {
                status: "ok".to_string(),
                image_base64: base64::encode(&png_bytes),
                width,
                height,
            }
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn capture_preview_screenshot(app_handle: AppHandle) -> Result<ScreenshotResult, String> {
    capture_main_screenshot(&app_handle)
}
