//! scrcpy Integration - Embed Android emulator in IDE
//!
//! Provides commands to start/stop scrcpy stream, spawn emulator headless, and capture frames

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::android_sdk::{adb_exists, emulator_path, get_adb_cmd};

static SCRCPY_RUNNING: AtomicBool = AtomicBool::new(false);
static EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);
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

/// Start emulator headless (no external window) and scrcpy stream
#[tauri::command]
pub async fn spawn_emulator_headless(avd_name: String, port: u16) -> Result<String, String> {
    if EMULATOR_RUNNING.load(Ordering::SeqCst) {
        return Ok("Emulator already running".to_string());
    }

    // Find emulator.exe path
    let emulator_path = emulator_path();

    // Start emulator in background (no display window on Windows isn't directly supported,
    // but we can minimize it and use scrcpy for display)
    let emulator_result = Command::new(emulator_path.to_string_lossy().as_ref())
        .args(&[
            "-avd", &avd_name,
            "-no-audio",
            "-gpu", "host",
            "-no-window",  // Headless mode (Linux only, but we try)
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    // If -no-window fails (Windows), start normally and let scrcpy handle display
    let emulator_result = match emulator_result {
        Ok(child) => Ok(child),
        Err(_) => {
            // Fallback: start with window, we'll capture via scrcpy anyway
            Command::new(emulator_path.to_string_lossy().as_ref())
                .args(&[
                    "-avd", &avd_name,
                    "-no-audio",
                    "-gpu", "host",
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        }
    };

    match emulator_result {
        Ok(_) => {
            EMULATOR_RUNNING.store(true, Ordering::SeqCst);
            
            // Wait for emulator to boot (poll adb)
            for _ in 0..30 {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                
                let output = get_adb_cmd().arg("devices").output();

                if let Ok(out) = output {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if stdout.contains("emulator-") && stdout.contains("device") {
                        // Extract device ID
                        for line in stdout.lines() {
                            if line.contains("emulator-") && line.contains("device") {
                                let device_id = line.split_whitespace().next().unwrap_or("emulator-5554");
                                
                                // Start scrcpy stream
                                let stream_url = start_scrcpy_for_device(device_id.to_string(), port).await?;
                                return Ok(stream_url);
                            }
                        }
                    }
                }
            }
            
            Ok(format!("Emulator '{}' starting (may take longer to boot)", avd_name))
        }
        Err(e) => Err(format!("Failed to spawn emulator: {}", e)),
    }
}

/// Helper to start scrcpy for a device
async fn start_scrcpy_for_device(device_id: String, port: u16) -> Result<String, String> {
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
    if !adb_exists() {
        return Err(
            "ADB not found. Install Android SDK Platform-Tools or set ANDROID_HOME.".to_string(),
        );
    }

    let output = get_adb_cmd()
        .args(["-s", &device_id, "exec-out", "screencap", "-p"])
        .output()
        .map_err(|e| format!("Failed to run adb: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("device") {
            return Err(format!("Emulator {} not found. Start an Android emulator first.", device_id));
        }
        return Err(format!("screencap failed: {}", stderr));
    }

    // Check if we got actual image data
    if output.stdout.is_empty() || output.stdout.len() < 100 {
        return Err("Emulator screen is black or not ready yet. Wait for boot to complete.".to_string());
    }

    // Convert to base64
    let base64_image = BASE64.encode(&output.stdout);
    FRAME_COUNT.fetch_add(1, Ordering::SeqCst);

    Ok(base64_image)
}

/// Send tap event to emulator
#[tauri::command]
pub async fn send_emulator_tap(device_id: String, x: i32, y: i32) -> Result<String, String> {
    get_adb_cmd()
        .args(["-s", &device_id, "shell", "input", "tap", &x.to_string(), &y.to_string()])
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
    get_adb_cmd()
        .args([
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
    
    get_adb_cmd()
        .args(["-s", &device_id, "shell", "input", "text", &escaped])
        .output()
        .map_err(|e| format!("Failed to send text: {}", e))?;

    Ok("text sent".to_string())
}

/// Send key event to emulator
#[tauri::command]
pub async fn send_emulator_key(device_id: String, keycode: i32) -> Result<String, String> {
    get_adb_cmd()
        .args(["-s", &device_id, "shell", "input", "keyevent", &keycode.to_string()])
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
