//! Emulator Stream Backend
//! 
//! Captures Android emulator screen via scrcpy/adb and streams to frontend via WebSocket
//! Also manages emulator spawning and lifecycle

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket as Ws};
use warp::Filter;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use futures::{SinkExt, StreamExt};

static STREAM_ACTIVE: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

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
pub fn list_available_avds() -> Result<Vec<AndroidVirtualDevice>, String> {
    let output = Command::new("avdmanager")
        .args(&["list", "avd"])
        .output()
        .map_err(|e| format!("Failed to run avdmanager: {}", e))?;

    if !output.status.success() {
        return Err(format!("avdmanager failed: {}", String::from_utf8_lossy(&output.stderr)));
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
    // Find emulator.exe in Android SDK
    let android_home = std::env::var("ANDROID_HOME")
        .or_else(|_| std::env::var("ANDROID_SDK_ROOT"))
        .unwrap_or_else(|_| "C:\\Users\\HADES\\AppData\\Local\\Android\\Sdk".to_string());

    let emulator_path = format!("{}\\emulator\\emulator.exe", android_home);

    // Start emulator
    let _child = Command::new(&emulator_path)
        .args(&["-avd", &avd_name, "-no-audio", "-gpu", "host"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to spawn emulator: {}", e))?;

    // Wait for emulator to boot (poll adb)
    for _ in 0..60 {
        // Wait up to 60 seconds
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        
        let output = Command::new("adb")
            .args(&["devices"])
            .output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("emulator-") {
                // Extract device ID
                for line in stdout.lines() {
                    if line.contains("emulator-") && line.contains("device") {
                        let device_id = line.split_whitespace().next().unwrap_or("");
                        return Ok(format!("Emulator '{}' started successfully! Device ID: {}", avd_name, device_id));
                    }
                }
            }
        }
    }

    Ok(format!("Emulator '{}' is starting but may take longer to boot", avd_name))
}

/// List running emulators
#[tauri::command]
pub fn list_running_emulators() -> Result<Vec<EmulatorProcess>, String> {
    let output = Command::new("adb")
        .args(&["devices"])
        .output()
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

/// Start emulator stream capture
#[tauri::command]
pub async fn start_emulator_stream(device_id: Option<String>) -> Result<String, String> {
    if STREAM_ACTIVE.load(Ordering::SeqCst) {
        return Ok("Stream already running".to_string());
    }

    let target_device = device_id.unwrap_or_else(|| "emulator-5554".to_string());

    // Try scrcpy first
    let scrcpy_result = Command::new("scrcpy")
        .args(&["--no-display", "--no-control", "--no-audio", "--serial", &target_device, "--bit-rate", "2M"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    if scrcpy_result.is_ok() {
        STREAM_ACTIVE.store(true, Ordering::SeqCst);
        return Ok(format!("scrcpy stream started for {}", target_device));
    }

    // Fallback to adb screencap loop
    let device_id_clone = target_device.clone();
    tokio::spawn(async move {
        loop {
            if !STREAM_ACTIVE.load(Ordering::SeqCst) {
                break;
            }

            // Capture frame via adb
            let output = Command::new("adb")
                .args(&["-s", &device_id_clone, "shell", "screencap", "-p"])
                .output();

            if let Ok(_out) = output {
                // Frame captured
                FRAME_COUNT.fetch_add(1, Ordering::SeqCst);
            }

            // 10 FPS
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    STREAM_ACTIVE.store(true, Ordering::SeqCst);
    Ok(format!("adb capture loop started for {}", target_device))
}

/// Stop emulator stream
#[tauri::command]
pub fn stop_emulator_stream() -> Result<String, String> {
    STREAM_ACTIVE.store(false, Ordering::SeqCst);
    Ok("Stream stopped".to_string())
}

/// Get stream status
#[tauri::command]
pub fn get_stream_status() -> StreamStatus {
    StreamStatus {
        active: STREAM_ACTIVE.load(Ordering::SeqCst),
        frame_count: FRAME_COUNT.load(Ordering::SeqCst),
    }
}

#[derive(serde::Serialize)]
pub struct StreamStatus {
    pub active: bool,
    pub frame_count: usize,
}

/// Create WebSocket route for emulator stream
pub fn ws_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("emulator-stream")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async move {
                handle_websocket(websocket).await;
            })
        })
}

async fn handle_websocket(websocket: Ws) {
    let (mut ws_tx, mut ws_rx) = websocket.split();
    let frame_buffer = Arc::new(Mutex::new(Vec::<u8>::new()));

    // Frame capture task
    let buffer_clone = Arc::clone(&frame_buffer);
    let capture_task = tokio::spawn(async move {
        loop {
            if !STREAM_ACTIVE.load(Ordering::SeqCst) {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                continue;
            }

            // Capture frame via adb
            let output = Command::new("adb")
                .args(&["-s", "emulator-5554", "shell", "screencap", "-p"])
                .output();

            if let Ok(out) = output {
                let mut buf = buffer_clone.lock().await;
                *buf = out.stdout;
                FRAME_COUNT.fetch_add(1, Ordering::SeqCst);
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    // Send frames to client
    let buffer_clone = Arc::clone(&frame_buffer);
    let send_task = tokio::spawn(async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            let buf = buffer_clone.lock().await;
            if !buf.is_empty() {
                let base64_frame = BASE64.encode(&*buf);
                let send_result = ws_tx.send(Message::text(base64_frame)).await;
                if send_result.is_err() {
                    break;
                }
            }
        }
    });

    // Receive messages from client (for controls)
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    // Handle control commands (tap, swipe, etc.)
                    let _text = msg.to_str().unwrap();
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
                break;
            }
        }
    }

    capture_task.abort();
    send_task.abort();
}

/// Initialize emulator stream server
pub async fn init_stream_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let routes = ws_route();
    
    println!("📺 Emulator stream server starting on port {}", port);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;

    Ok(())
}
