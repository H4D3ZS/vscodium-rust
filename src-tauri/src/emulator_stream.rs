//! Emulator Stream Backend - Android Emulator Management
//! 
//! Provides commands to list, spawn, and stream Android emulators

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};

static SCRCPY_RUNNING: AtomicBool = AtomicBool::new(false);
static EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Get Android SDK path
fn get_android_sdk_path() -> String {
    // Try environment variables first
    if let Ok(sdk_home) = std::env::var("ANDROID_HOME") {
        return sdk_home;
    }
    if let Ok(sdk_root) = std::env::var("ANDROID_SDK_ROOT") {
        return sdk_root;
    }
    
    // Default Windows path
    "C:\\Users\\HADES\\AppData\\Local\\Android\\Sdk".to_string()
}

/// Get avdmanager command with full path
fn get_avdmanager_cmd() -> Command {
    let sdk_path = get_android_sdk_path();
    let avdmanager_path = format!("{}\\cmdline-tools\\latest\\bin\\avdmanager.bat", sdk_path);
    
    // Check if exists, fallback to alternative paths
    if std::path::Path::new(&avdmanager_path).exists() {
        Command::new(avdmanager_path)
    } else {
        // Try alternative paths
        let alt_path = format!("{}\\tools\\bin\\avdmanager.bat", sdk_path);
        Command::new(alt_path)
    }
}

/// Get emulator command with full path
fn get_emulator_cmd() -> Command {
    let sdk_path = get_android_sdk_path();
    let emulator_path = format!("{}\\emulator\\emulator.exe", sdk_path);
    Command::new(emulator_path)
}

/// Get adb command with full path
fn get_adb_cmd() -> Command {
    let sdk_path = get_android_sdk_path();
    let adb_path = format!("{}\\platform-tools\\adb.exe", sdk_path);
    Command::new(adb_path)
}

/// Available Android Virtual Device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidVirtualDevice {
    pub name: String,
    pub device: String,
    pub path: String,
    pub target: String,
    pub abi: String,
    pub skin: String,
}

/// Emulator process info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorProcess {
    pub device_id: String,
    pub avd_name: String,
    pub port: u16,
    pub status: String,
}

/// List available AVDs
#[tauri::command]
pub async fn list_available_avds() -> Result<Vec<AndroidVirtualDevice>, String> {
    let mut cmd = get_avdmanager_cmd();
    cmd.args(&["list", "avd"]);
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to run avdmanager: {}. Make sure Android SDK is installed and ANDROID_HOME is set.", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("avdmanager failed: {}. Try: avdmanager create avd -n \"Pixel_4\" -k \"system-images;android-34;google_apis_playstore;x86_64\" -d \"pixel_4\"", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut avds = Vec::new();
    let mut current_avd: Option<AndroidVirtualDevice> = None;

    for line in stdout.lines() {
        let line = line.trim();
        
        if line.starts_with("Name:") {
            if let Some(avd) = current_avd.take() {
                avds.push(avd);
            }
            current_avd = Some(AndroidVirtualDevice {
                name: line.split(':').nth(1).unwrap_or("").trim().to_string(),
                device: String::new(),
                path: String::new(),
                target: String::new(),
                abi: String::new(),
                skin: String::new(),
            });
        } else if let Some(ref mut avd) = current_avd {
            if line.starts_with("Device:") {
                avd.device = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("Path:") {
                avd.path = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("Target:") {
                avd.target = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("ABI:") {
                avd.abi = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("Skin:") {
                avd.skin = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }

    if let Some(avd) = current_avd {
        avds.push(avd);
    }

    Ok(avds)
}

/// Spawn emulator by AVD name
#[tauri::command]
pub async fn spawn_emulator_by_name(avd_name: String) -> Result<String, String> {
    let mut cmd = get_emulator_cmd();
    cmd.args(&["-avd", &avd_name, "-no-audio", "-gpu", "host"]);
    
    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn emulator: {}", e))?;
    
    EMULATOR_RUNNING.store(true, Ordering::SeqCst);
    
    // Wait for emulator to boot (poll adb)
    for _ in 0..30 {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        
        let mut adb_cmd = get_adb_cmd();
        adb_cmd.arg("devices");
        
        if let Ok(out) = adb_cmd.output() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("emulator-") && stdout.contains("device") {
                for line in stdout.lines() {
                    if line.contains("emulator-") && line.contains("device") {
                        let device_id = line.split_whitespace().next().unwrap_or("emulator-5554");
                        return Ok(format!("Emulator '{}' started successfully! Device ID: {}", avd_name, device_id));
                    }
                }
            }
        }
    }
    
    Ok(format!("Emulator '{}' is starting (may take longer to boot)", avd_name))
}

/// List running emulators
#[tauri::command]
pub fn list_running_emulators() -> Result<Vec<EmulatorProcess>, String> {
    let mut adb_cmd = get_adb_cmd();
    adb_cmd.arg("devices");
    
    let output = adb_cmd.output()
        .map_err(|e| format!("Failed to run adb: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut emulators = Vec::new();

    for line in stdout.lines() {
        if line.contains("emulator-") && line.contains("device") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let device_id = parts[0].to_string();
                let port = device_id
                    .trim_start_matches("emulator-")
                    .parse::<u16>()
                    .unwrap_or(5554);

                emulators.push(EmulatorProcess {
                    device_id,
                    avd_name: String::new(),
                    port,
                    status: "running".to_string(),
                });
            }
        }
    }

    Ok(emulators)
}

/// Start emulator stream
#[tauri::command]
pub async fn start_emulator_stream(device_id: String) -> Result<String, String> {
    // For now, just return success - actual streaming will be implemented
    Ok(format!("Stream started for {}", device_id))
}

/// Stop emulator stream
#[tauri::command]
pub fn stop_emulator_stream() -> Result<String, String> {
    SCRCPY_RUNNING.store(false, Ordering::SeqCst);
    Ok("Stream stopped".to_string())
}

/// Get stream status
#[tauri::command]
pub fn get_stream_status() -> StreamStatus {
    StreamStatus {
        running: SCRCPY_RUNNING.load(Ordering::SeqCst),
        frame_count: FRAME_COUNT.load(Ordering::SeqCst),
    }
}

#[derive(serde::Serialize)]
pub struct StreamStatus {
    pub running: bool,
    pub frame_count: usize,
}
