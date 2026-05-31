//! Emulator Stream Backend - Embedded emulator in IDE panel
//!
//! Launches emulator minimized, captures frames via ADB screencap,
//! streams to EmulatorPreview via Tauri events. No separate window visible.
//! Like Android Studio's "Running Devices" panel.

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

static EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);
static STREAM_RUNNING: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);
static CAPTURE_TASK: Mutex<Option<tokio::task::JoinHandle<()>>> = Mutex::new(None);

fn get_android_sdk_path() -> String {
    for var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if let Ok(val) = std::env::var(var) {
            if std::path::Path::new(&val).exists() { return val; }
        }
    }
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    let locations: Vec<String> = vec![
        format!("{}\\AppData\\Local\\Android\\Sdk", home),
        format!("{}\\Android\\Sdk", home),
        "C:\\Android\\Sdk".to_string(),
        "C:\\Program Files\\Android\\Sdk".to_string(),
        "C:\\Program Files (x86)\\Android\\Sdk".to_string(),
    ];
    for loc in &locations {
        if std::path::Path::new(loc).exists() { return loc.clone(); }
    }
    format!("{}\\AppData\\Local\\Android\\Sdk", home)
}

fn get_adb_cmd() -> Command {
    Command::new(format!("{}\\platform-tools\\adb.exe", get_android_sdk_path()))
}

fn get_avdmanager_cmd() -> Command {
    let p = format!("{}\\cmdline-tools\\latest\\bin\\avdmanager.bat", get_android_sdk_path());
    if std::path::Path::new(&p).exists() { Command::new(p) }
    else { Command::new(format!("{}\\tools\\bin\\avdmanager.bat", get_android_sdk_path())) }
}

fn _get_emulator_cmd() -> Command {
    Command::new(format!("{}\\emulator\\emulator.exe", get_android_sdk_path()))
}

/// Minimize emulator window by title or class
fn minimize_emulator_window() {
    #[cfg(target_os = "windows")] {
        use std::ffi::CString;
        use windows::Win32::UI::WindowsAndMessaging::{
            FindWindowA, ShowWindow, SW_HIDE, SW_MINIMIZE
        };
        use windows::Win32::Foundation::HWND;
        use windows::core::PCSTR;

        let target_titles = [
            "Android Emulator", "Emulator", "Pixel", "Nexus",
            "emu", "Emulator", "QEMU", "Virtual Device"
        ];

        // First try FindWindowA with class names (more reliable than titles)
        let classes = ["Qt5QWindowIcon", "Qt5QWindow", "AndroidEmulator", "emulator"];
        for cls in &classes {
            if let Ok(c_cls) = CString::new(*cls) {
                unsafe {
                    if let Ok(hwnd) = FindWindowA(
                        PCSTR::from_raw(c_cls.as_ptr() as *const u8),
                        PCSTR::null()
                    ) {
                        if hwnd != HWND::default() {
                            let _ = ShowWindow(hwnd, SW_HIDE);
                            return;
                        }
                    }
                }
            }
        }

        // Fallback: try title match
        for title in &target_titles {
            if let Ok(c_title) = CString::new(*title) {
                unsafe {
                    if let Ok(hwnd) = FindWindowA(
                        PCSTR::null(),
                        PCSTR::from_raw(c_title.as_ptr() as *const u8)
                    ) {
                        if hwnd != HWND::default() {
                            let _ = ShowWindow(hwnd, SW_MINIMIZE);
                            return;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidVirtualDevice {
    pub name: String, pub device: String, pub path: String,
    pub target: String, pub abi: String, pub skin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorProcess {
    pub device_id: String, pub avd_name: String,
    pub port: u16, pub status: String,
}

/// List available AVDs
#[tauri::command]
pub async fn list_available_avds() -> Result<Vec<AndroidVirtualDevice>, String> {
    let output = get_avdmanager_cmd().args(&["list", "avd"]).output()
        .map_err(|e| format!("avdmanager: {}", e))?;
    if !output.status.success() {
        return Err(format!("avdmanager failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    let mut avds = Vec::new();
    let mut cur: Option<AndroidVirtualDevice> = None;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let t = line.trim();
        if t.starts_with("Name:") {
            if let Some(avd) = cur.take() { avds.push(avd); }
            cur = Some(AndroidVirtualDevice {
                name: t.split(':').nth(1).unwrap_or("").trim().to_string(),
                device: String::new(), path: String::new(),
                target: String::new(), abi: String::new(), skin: String::new(),
            });
        } else if let Some(ref mut a) = cur {
            if t.starts_with("Device:") { a.device = t.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if t.starts_with("Path:") { a.path = t.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if t.starts_with("Target:") { a.target = t.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if t.starts_with("ABI:") { a.abi = t.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if t.starts_with("Skin:") { a.skin = t.split(':').nth(1).unwrap_or("").trim().to_string(); }
        }
    }
    if let Some(avd) = cur { avds.push(avd); }
    Ok(avds)
}

/// Create a new AVD
#[tauri::command]
pub async fn create_avd(name: String, device: Option<String>, image: Option<String>) -> Result<String, String> {
    let d = device.unwrap_or_else(|| "pixel_4".to_string());
    let img = image.unwrap_or_else(|| "system-images;android-34;google_apis_playstore;x86_64".to_string());
    let output = get_avdmanager_cmd().args(&["create", "avd", "-n", &name, "-k", &img, "-d", &d, "-f"]).output()
        .map_err(|e| format!("avdmanager: {}", e))?;
    if output.status.success() {
        Ok(format!("AVD '{}' created", name))
    } else {
        Err(format!("Failed: {}\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout)))
    }
}

/// Launch emulator in HEADLESS mode (no visible window).
/// Display appears ONLY in the IDE panel via ADB screencap streaming.
#[tauri::command]
pub async fn spawn_emulator_by_name(avd_name: String) -> Result<String, String> {
    if EMULATOR_RUNNING.load(Ordering::SeqCst) {
        return Ok("Emulator already running".to_string());
    }

    // Check ADB and AVD
    let adb = format!("{}\\platform-tools\\adb.exe", get_android_sdk_path());
    if !std::path::Path::new(&adb).exists() {
        return Err(format!("ADB not found at {}", adb));
    }
    let avds = list_available_avds().await?;
    if !avds.iter().any(|a| a.name == avd_name) {
        return Err(format!("AVD '{}' not found. Available: {}", avd_name,
            avds.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")));
    }
    let emu = format!("{}\\emulator\\emulator.exe", get_android_sdk_path());
    if !std::path::Path::new(&emu).exists() {
        return Err(format!("Emulator not found at {}", emu));
    }

    println!("[Emulator] Launching '{}' headless...", avd_name);
    match Command::new(&emu).args(&[
        "-avd", &avd_name,
        "-no-audio",
        "-no-window",              // KEY: NO external window (Android Studio style)
        "-gpu", "swiftshader_indirect",  // Works on ALL GPUs including AMD
        "-no-snapshot",
        "-memory", "2048",
        "-cores", "4",
        "-no-boot-anim",
        "-screen", "touch",
        "-netdelay", "none",
        "-netspeed", "full",
        "-port", "5554",
        "-ports", "5554,5555",
    ]).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
        Ok(child) => {
            EMULATOR_RUNNING.store(true, Ordering::SeqCst);
            println!("[Emulator] PID: {}", child.id());
            
            // Immediately minimize any emulator window that appears
            minimize_emulator_window();

            // Wait for boot - poll adb for 'device' state
            let start = std::time::Instant::now();
            let max_wait = std::time::Duration::from_secs(180);
            let device_id = "emulator-5554";
            
            while start.elapsed() < max_wait {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                // Try adb get-state first (more reliable)
                let mut adb = get_adb_cmd();
                adb.args(&["-s", device_id, "get-state"]);
                if let Ok(out) = adb.output() {
                    if String::from_utf8_lossy(&out.stdout).trim() == "device" {
                        let secs = start.elapsed().as_secs();
                        println!("[Emulator] Booted in {}s", secs);
                        return Ok(format!(
                            "Emulator '{}' booted in {}s\n\nEmbedded display will appear in the Emulator Preview panel.\n\nTap/swipe on the preview to interact.",
                            avd_name, secs
                        ));
                    }
                }

                // Also minimize any window that appeared
                minimize_emulator_window();
            }

            // Try adb devices fallback check
            let mut adb2 = get_adb_cmd(); adb2.arg("devices");
            if let Ok(out) = adb2.output() {
                if String::from_utf8_lossy(&out.stdout).contains("emulator-") {
                    return Ok(format!("Emulator '{}' is online (may still be booting)", avd_name));
                }
            }
            Ok(format!("Emulator '{}' is starting", avd_name))
        }
        Err(e) => {
            EMULATOR_RUNNING.store(false, Ordering::SeqCst);
            Err(format!("Failed to launch emulator: {}. Make sure no other emulator is running.", e))
        }
    }
}

#[tauri::command]
pub fn list_running_emulators() -> Result<Vec<EmulatorProcess>, String> {
    let output = get_adb_cmd().arg("devices").output()
        .map_err(|e| format!("adb: {}", e))?;
    let mut emus = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.contains("emulator-") && line.contains("device") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            emus.push(EmulatorProcess {
                device_id: parts[0].to_string(),
                avd_name: String::new(),
                port: parts[0].trim_start_matches("emulator-").parse().unwrap_or(5554),
                status: "running".to_string(),
            });
        }
    }
    Ok(emus)
}

/// Start ADB screencap streaming (no separate window needed).
/// Captures frames via `adb exec-out screencap -p` and emits `emulator:frame` events.
#[tauri::command]
pub async fn start_emulator_stream(app: AppHandle, _device_id: String) -> Result<String, String> {
    if CAPTURE_TASK.lock().unwrap().is_some() {
        return Err("Stream already running".to_string());
    }
    STREAM_RUNNING.store(true, Ordering::SeqCst);

    let app_clone = app.clone();
    let handle = tokio::spawn(async move {
        let device = "emulator-5554";
        let mut last_frame = std::time::Instant::now();
        let min_interval = std::time::Duration::from_millis(100); // ~10fps max
        let mut consecutive_failures = 0;

        while STREAM_RUNNING.load(Ordering::SeqCst) {
            let now = std::time::Instant::now();
            if now.duration_since(last_frame) < min_interval {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                continue;
            }

            // Periodically re-hide any emulator window (runs every ~5s)
            if FRAME_COUNT.load(Ordering::SeqCst) % 50 == 0 {
                minimize_emulator_window();
            }

            // Capture via ADB screencap (built into Android SDK, no external deps)
            let mut adb = get_adb_cmd();
            adb.args(&["-s", device, "exec-out", "screencap", "-p"]);
            
            match adb.output() {
                Ok(output) => {
                    if output.status.success() && !output.stdout.is_empty() {
                        consecutive_failures = 0;
                        let b64 = BASE64.encode(&output.stdout);
                        last_frame = std::time::Instant::now();

                        let (w, h) = if output.stdout.len() > 24 {
                            let w = u32::from_be_bytes([output.stdout[16], output.stdout[17], output.stdout[18], output.stdout[19]]);
                            let h = u32::from_be_bytes([output.stdout[20], output.stdout[21], output.stdout[22], output.stdout[23]]);
                            (w, h)
                        } else { (1080u32, 1920u32) };

                        let _ = app_clone.emit("emulator:frame", serde_json::json!({
                            "base64": b64,
                            "width": w,
                            "height": h,
                            "format": "png",
                            "frame": FRAME_COUNT.fetch_add(1, Ordering::SeqCst),
                            "timestamp": std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                        }));
                        continue;
                    }
                    consecutive_failures += 1;
                    if consecutive_failures == 1 || consecutive_failures % 20 == 0 {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("[EmulatorStream] ADB screencap attempt {}: status={}, stderr='{}'",
                            consecutive_failures, output.status, stderr.trim());
                    }
                }
                Err(e) => {
                    consecutive_failures += 1;
                    if consecutive_failures == 1 || consecutive_failures % 20 == 0 {
                        println!("[EmulatorStream] ADB connection failed (attempt {}): {}", consecutive_failures, e);
                    }
                }
            }

            let delay = std::cmp::min(500 * (1 + consecutive_failures / 10), 2000);
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        }
    });

    *CAPTURE_TASK.lock().unwrap() = Some(handle);
    Ok("Emulator stream started. Display is embedded in the IDE panel.".to_string())
}

#[tauri::command]
pub async fn stop_emulator_stream() -> Result<String, String> {
    EMULATOR_RUNNING.store(false, Ordering::SeqCst);
    STREAM_RUNNING.store(false, Ordering::SeqCst);
    let mut task = CAPTURE_TASK.lock().unwrap();
    if let Some(h) = task.take() { h.abort(); }
    Ok("Stream stopped".to_string())
}

#[derive(Serialize)]
pub struct StreamStatus { pub running: bool, pub stream_running: bool, pub frame_count: usize }

#[tauri::command]
pub fn get_stream_status() -> StreamStatus {
    StreamStatus {
        running: EMULATOR_RUNNING.load(Ordering::SeqCst),
        stream_running: STREAM_RUNNING.load(Ordering::SeqCst),
        frame_count: FRAME_COUNT.load(Ordering::SeqCst),
    }
}
