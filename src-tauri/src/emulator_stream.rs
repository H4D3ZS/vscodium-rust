//! Emulator Stream Backend
//! 
//! Captures Android emulator screen via scrcpy/adb and streams to frontend via WebSocket

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

static STREAM_ACTIVE: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Start emulator stream capture
#[tauri::command]
pub async fn start_emulator_stream() -> Result<String, String> {
    if STREAM_ACTIVE.load(Ordering::SeqCst) {
        return Ok("Stream already running".to_string());
    }

    // Try scrcpy first
    let scrcpy_result = Command::new("scrcpy")
        .args(&["--no-display", "--no-control", "--no-audio", "--tcpip=5555", "--bit-rate", "2M"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    if scrcpy_result.is_ok() {
        STREAM_ACTIVE.store(true, Ordering::SeqCst);
        return Ok("scrcpy stream started".to_string());
    }

    // Fallback to adb screencap loop
    tokio::spawn(async {
        loop {
            if !STREAM_ACTIVE.load(Ordering::SeqCst) {
                break;
            }

            // Capture frame via adb
            let output = Command::new("adb")
                .args(&["-s", "emulator-5554", "shell", "screencap", "-p"])
                .output();

            if let Ok(out) = output {
                // Frame captured - would send to WebSocket clients
                FRAME_COUNT.fetch_add(1, Ordering::SeqCst);
            }

            // 10 FPS
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    STREAM_ACTIVE.store(true, Ordering::SeqCst);
    Ok("adb capture loop started".to_string())
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
                handleWebSocket(websocket).await;
            })
        })
}

async fn handleWebSocket(websocket: WebSocket) {
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
    let send_task = tokio::spawn(async move {
        let buffer_clone = Arc::clone(&frame_buffer);
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            let buf = buffer_clone.lock().await;
            if !buf.is_empty() {
                let base64_frame = BASE64.encode(&*buf);
                if ws_tx.send(Message::text(base64_frame)).await.is_err() {
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
