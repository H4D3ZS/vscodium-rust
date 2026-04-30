//! scrcpy Integration - Embed Android emulator in IDE
//! 
//! Provides commands to start/stop scrcpy stream and capture frames

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

static SCRCPY_RUNNING: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Start scrcpy stream for device
#[tauri::command]
pub async fn start_scrcpy_stream(device_id: String, port: u16) -> Result<String, String> {
    if SCRCPY_RUNNING.load(Ordering::SeqCst) {
        return Ok(format!("http://localhost:{}/video", port));
    }

    // Start scrcpy server
    let scrcpy_result = Command::new("scrcpy")
        .args(&[
            "--serial", &device_id,
            "--no-audio",
            "--no-control",
            "--port", &port.to_string(),
            "--bit-rate", "2M",
            "--max-fps", "30",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match scrcpy_result {
        Ok(_) => {
            SCRCPY_RUNNING.store(true, Ordering::SeqCst);
            Ok(format!("http://localhost:{}/video", port))
        }
        Err(e) => Err(format!("Failed to start scrcpy: {}", e)),
    }
}

/// Stop scrcpy stream
#[tauri::command]
pub fn stop_scrcpy_stream() -> Result<String, String> {
    SCRCPY_RUNNING.store(false, Ordering::SeqCst);
    
    // Kill scrcpy process
    #[cfg(windows)]
    {
        Command::new("taskkill")
            .args(&["/F", "/IM", "scrcpy.exe"])
            .output()
            .ok();
    }

    Ok("scrcpy stopped".to_string())
}

/// Capture single frame from emulator
#[tauri::command]
pub async fn capture_emulator_frame(device_id: String) -> Result<String, String> {
    // Use adb screencap
    let output = Command::new("adb")
        .args(&["-s", &device_id, "shell", "screencap", "-p"])
        .output()
        .map_err(|e| format!("Failed to run adb: {}", e))?;

    if !output.status.success() {
        return Err(format!("screencap failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    // Convert to base64
    let base64_image = BASE64.encode(&output.stdout);
    FRAME_COUNT.fetch_add(1, Ordering::SeqCst);

    Ok(base64_image)
}

/// Send tap event to emulator
#[tauri::command]
pub async fn send_emulator_tap(device_id: String, x: i32, y: i32) -> Result<String, String> {
    Command::new("adb")
        .args(&["-s", &device_id, "shell", "input", "tap", &x.to_string(), &y.to_string()])
        .output()
        .map_err(|e| format!("Failed to send tap: {}", e))?;

    Ok("tap sent".to_string())
}

/// Send swipe event to emulator
#[tauri::command]
pub async fn send_emulator_swipe(
    device_id: String,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    duration: i32,
) -> Result<String, String> {
    Command::new("adb")
        .args(&[
            "-s", &device_id,
            "shell", "input", "swipe",
            &x1.to_string(), &y1.to_string(),
            &x2.to_string(), &y2.to_string(),
            &duration.to_string(),
        ])
        .output()
        .map_err(|e| format!("Failed to send swipe: {}", e))?;

    Ok("swipe sent".to_string())
}

/// Send text input to emulator
#[tauri::command]
pub async fn send_emulator_text(device_id: String, text: String) -> Result<String, String> {
    // Escape special characters for adb
    let escaped = text.replace(' ', "%s").replace('"', "\\\"");
    
    Command::new("adb")
        .args(&["-s", &device_id, "shell", "input", "text", &escaped])
        .output()
        .map_err(|e| format!("Failed to send text: {}", e))?;

    Ok("text sent".to_string())
}

/// Send key event to emulator
#[tauri::command]
pub async fn send_emulator_key(device_id: String, keycode: i32) -> Result<String, String> {
    Command::new("adb")
        .args(&["-s", &device_id, "shell", "input", "keyevent", &keycode.to_string()])
        .output()
        .map_err(|e| format!("Failed to send key: {}", e))?;

    Ok("key sent".to_string())
}

/// Get scrcpy status
#[tauri::command]
pub fn get_scrcpy_status() -> ScrcpyStatus {
    ScrcpyStatus {
        running: SCRCPY_RUNNING.load(Ordering::SeqCst),
        frame_count: FRAME_COUNT.load(Ordering::SeqCst),
    }
}

#[derive(serde::Serialize)]
pub struct ScrcpyStatus {
    pub running: bool,
    pub frame_count: usize,
}
