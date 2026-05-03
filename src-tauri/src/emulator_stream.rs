//! Emulator Stream Backend - Real-time framebuffer streaming
//!
//! Captures emulator window via BitBlt and emits frames as Tauri events.
//! No WebSocket needed — uses existing Tauri IPC.

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

static SCRCPY_RUNNING: AtomicBool = AtomicBool::new(false);
static EMULATOR_RUNNING: AtomicBool = AtomicBool::new(false);
static FRAME_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Holds the spawned capture task handle so we can stop it
static CAPTURE_TASK: Mutex<Option<tokio::task::JoinHandle<()>>> = Mutex::new(None);

fn get_android_sdk_path() -> String {
    // Try environment variables first
    if let Ok(sdk_home) = std::env::var("ANDROID_HOME") {
        if std::path::Path::new(&sdk_home).exists() {
            return sdk_home;
        }
    }
    if let Ok(sdk_root) = std::env::var("ANDROID_SDK_ROOT") {
        if std::path::Path::new(&sdk_root).exists() {
            return sdk_root;
        }
    }

    // Try common locations
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| "C:\\Users\\Default".to_string());

    let locations = vec![
        format!("{}\\AppData\\Local\\Android\\Sdk", home),
        format!("{}\\Android\\Sdk", home),
        "C:\\Android\\Sdk".to_string(),
        "C:\\Program Files\\Android\\Sdk".to_string(),
        "C:\\Program Files (x86)\\Android\\Sdk".to_string(),
    ];

    for loc in &locations {
        if std::path::Path::new(loc).exists() {
            return loc.clone();
        }
    }

    // Fallback
    locations[0].clone()
}

fn get_avdmanager_cmd() -> Command {
    let sdk_path = get_android_sdk_path();
    let p = format!("{}\\cmdline-tools\\latest\\bin\\avdmanager.bat", sdk_path);
    if std::path::Path::new(&p).exists() {
        Command::new(p)
    } else {
        Command::new(format!("{}\\tools\\bin\\avdmanager.bat", sdk_path))
    }
}

fn get_emulator_cmd() -> Command {
    Command::new(format!("{}\\emulator\\emulator.exe", get_android_sdk_path()))
}

fn get_adb_cmd() -> Command {
    Command::new(format!("{}\\platform-tools\\adb.exe", get_android_sdk_path()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidVirtualDevice {
    pub name: String,
    pub device: String,
    pub path: String,
    pub target: String,
    pub abi: String,
    pub skin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorProcess {
    pub device_id: String,
    pub avd_name: String,
    pub port: u16,
    pub status: String,
}

#[tauri::command]
pub async fn list_available_avds() -> Result<Vec<AndroidVirtualDevice>, String> {
    let mut cmd = get_avdmanager_cmd();
    cmd.args(&["list", "avd"]);
    let output = cmd.output()
        .map_err(|e| format!("Failed to run avdmanager: {}", e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("avdmanager failed: {}", stderr));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut avds = Vec::new();
    let mut current_avd: Option<AndroidVirtualDevice> = None;
    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("Name:") {
            if let Some(avd) = current_avd.take() { avds.push(avd); }
            current_avd = Some(AndroidVirtualDevice {
                name: line.split(':').nth(1).unwrap_or("").trim().to_string(),
                device: String::new(), path: String::new(),
                target: String::new(), abi: String::new(), skin: String::new(),
            });
        } else if let Some(ref mut avd) = current_avd {
            if line.starts_with("Device:") { avd.device = line.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if line.starts_with("Path:") { avd.path = line.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if line.starts_with("Target:") { avd.target = line.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if line.starts_with("ABI:") { avd.abi = line.split(':').nth(1).unwrap_or("").trim().to_string(); }
            else if line.starts_with("Skin:") { avd.skin = line.split(':').nth(1).unwrap_or("").trim().to_string(); }
        }
    }
    if let Some(avd) = current_avd { avds.push(avd); }
    Ok(avds)
}

/// Create a new Android Virtual Device
#[tauri::command]
pub async fn create_avd(
    name: String,
    device: Option<String>,
    image: Option<String>,
) -> Result<String, String> {
    let device = device.unwrap_or_else(|| "pixel_4".to_string());
    let image = image.unwrap_or_else(|| "system-images;android-34;google_apis_playstore;x86_64".to_string());

    let mut cmd = get_avdmanager_cmd();
    cmd.args(&[
        "create", "avd",
        "-n", &name,
        "-k", &image,
        "-d", &device,
        "-f",
    ]);

    let output = cmd.output()
        .map_err(|e| format!("Failed to run avdmanager: {}", e))?;

    if output.status.success() {
        Ok(format!("AVD '{}' created successfully! (device: {}, image: {})", name, device, image))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!(
            "Failed to create AVD: {}\n{}\n\nMake sure the system image exists:\n  sdkmanager \"{}\"",
            stderr, stdout, image
        ))
    }
}

#[tauri::command]
pub async fn spawn_emulator_by_name(avd_name: String) -> Result<String, String> {
    if EMULATOR_RUNNING.load(Ordering::SeqCst) {
        return Ok("Emulator already running".to_string());
    }

    // Check ADB is available
    let adb_path = get_adb_cmd()
        .get_program()
        .to_string_lossy()
        .to_string();
    if !std::path::Path::new(&adb_path).exists() {
        return Err(format!(
            "ADB not found at '{}'. Make sure Android SDK is installed and ANDROID_HOME is set.",
            adb_path
        ));
    }

    // Check AVD exists
    let avds = list_available_avds().await?;
    if !avds.iter().any(|a| a.name == avd_name) {
        return Err(format!(
            "AVD '{}' not found. Available: {}. Create one with: avdmanager create avd -n \"{}\" -k \"system-images;android-34;google_apis_playstore;x86_64\" -d \"pixel_4\"",
            avd_name,
            avds.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", "),
            avd_name
        ));
    }

    let emulator_path = format!("{}\\emulator\\emulator.exe", get_android_sdk_path());
    if !std::path::Path::new(&emulator_path).exists() {
        return Err(format!("Emulator not found at '{}'", emulator_path));
    }

    // Start emulator with MAXIMUM compatibility:
    // - NO -no-window (we NEED the window for BitBlt capture)
    // - swiftshader_indirect GPU for AMD/Intel/NVIDIA compatibility
    // - No snapshot to force clean boot
    // - Show kernel boot for debugging
    println!("[Emulator] Starting AVD '{}'...", avd_name);
    let mut cmd = Command::new(&emulator_path);
    cmd.args(&[
        "-avd", &avd_name,
        "-no-audio",
        "-gpu", "swiftshader_indirect",
        "-no-snapshot",
        "-show-kernel",
        "-memory", "2048",
        "-cores", "4",
        "-wipe-data",
        "-netdelay", "none",
        "-netspeed", "full",
        "-no-boot-anim",
        "-screen", "touch",
    ]);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());

    match cmd.spawn() {
        Ok(child) => {
            EMULATOR_RUNNING.store(true, Ordering::SeqCst);
            println!("[Emulator] AVD '{}' spawned (PID: {})", avd_name, child.id());

            // Wait for emulator to boot - poll adb for 'device' state (not just 'emulator-')
            let start_time = std::time::Instant::now();
            let max_wait = std::time::Duration::from_secs(180); // 3 minutes max
            let device_id = format!("emulator-5554");

            while start_time.elapsed() < max_wait {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                let mut adb = get_adb_cmd();
                adb.args(&["-s", &device_id, "get-state"]);
                if let Ok(out) = adb.output() {
                    let state = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if state == "device" {
                        let elapsed = start_time.elapsed().as_secs();
                        println!("[Emulator] AVD '{}' booted in {}s!", avd_name, elapsed);
                        return Ok(format!(
                            "Emulator '{}' booted in {}s! Device ID: {}",
                            avd_name, elapsed, device_id
                        ));
                    }
                }

                // Also check if adb devices shows it
                let mut adb2 = get_adb_cmd();
                adb2.arg("devices");
                if let Ok(out) = adb2.output() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if stdout.contains(&device_id) && stdout.contains("device") {
                        let elapsed = start_time.elapsed().as_secs();
                        println!("[Emulator] AVD '{}' online in {}s", avd_name, elapsed);
                        return Ok(format!(
                            "Emulator '{}' online in {}s! Device ID: {}",
                            avd_name, elapsed, device_id
                        ));
                    }
                }
            }

            // If we get here, emulator is running but not fully booted
            Ok(format!(
                "Emulator '{}' is starting (may take longer to boot). Check AVD Manager for status.",
                avd_name
            ))
        }
        Err(e) => {
            EMULATOR_RUNNING.store(false, Ordering::SeqCst);
            let error_detail = format!("{}", e);

            // Provide helpful error messages
            if error_detail.contains("2") || error_detail.contains("No such file") {
                Err(format!(
                    "Emulator executable not found at '{}'. Check Android SDK installation.",
                    emulator_path
                ))
            } else if error_detail.contains("Access is denied") {
                Err(format!(
                    "Access denied running emulator. Try running as Administrator, or check if another emulator instance is running (taskkill /F /IM emulator.exe)."
                ))
            } else {
                Err(format!("Failed to spawn emulator: {}", e))
            }
        }
    }
}

#[tauri::command]
pub fn list_running_emulators() -> Result<Vec<EmulatorProcess>, String> {
    let mut cmd = get_adb_cmd(); cmd.arg("devices");
    let output = cmd.output().map_err(|e| format!("Failed to run adb: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut emulators = Vec::new();
    for line in stdout.lines() {
        if line.contains("emulator-") && line.contains("device") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                emulators.push(EmulatorProcess {
                    device_id: parts[0].to_string(),
                    avd_name: String::new(),
                    port: parts[0].trim_start_matches("emulator-").parse().unwrap_or(5554),
                    status: "running".to_string(),
                });
            }
        }
    }
    Ok(emulators)
}

/// Start real-time emulator framebuffer streaming.
/// Captures emulator window via BitBlt at ~15fps and emits `emulator:frame` events.
#[tauri::command]
pub async fn start_emulator_stream(app: AppHandle, _device_id: String) -> Result<String, String> {
    if SCRCPY_RUNNING.load(Ordering::SeqCst) {
        return Err("Stream already running".to_string());
    }
    SCRCPY_RUNNING.store(true, Ordering::SeqCst);

    let app_clone = app.clone();
    let handle = tokio::spawn(async move {
        use std::ffi::CString;
        use windows::Win32::Foundation::{HWND, RECT};
        use windows::Win32::Graphics::Gdi::{
            BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject,
            GetDC, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, SRCCOPY,
        };
        use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, GetWindowRect};
        use windows::core::PCSTR;
        use image::{ImageBuffer, Rgba};
        use std::io::Cursor;

        let emulator_titles = [
            "Android Emulator", "Pixel", "iPhone Simulator",
            "Virtual iPhone", "Acheron", "scrcpy", "Flutter"
        ];

        while SCRCPY_RUNNING.load(Ordering::SeqCst) {
            let mut hwnd_opt = None;
            for title in &emulator_titles {
                if let Ok(c_title) = CString::new(*title) {
                    unsafe {
                        if let Ok(hwnd) = FindWindowA(
                            PCSTR::null(),
                            PCSTR::from_raw(c_title.as_ptr() as *const u8)
                        ) {
                            if hwnd != HWND::default() {
                                hwnd_opt = Some(hwnd);
                                break;
                            }
                        }
                    }
                }
            }

            let (x, y, w, h) = if let Some(hwnd) = hwnd_opt {
                let mut rect = RECT::default();
                unsafe { let _ = GetWindowRect(hwnd, &mut rect); }
                let rw = (rect.right - rect.left).max(1) as u32;
                let rh = (rect.bottom - rect.top).max(1) as u32;
                (rect.left, rect.top, rw, rh)
            } else {
                // Fallback: capture full screen
                (0, 0, 800u32, 600u32)
            };

            // Cap at 1280x720 for performance
            let cw = w.min(1280);
            let ch = h.min(720);

            let rgba_opt: Option<Vec<u8>> = unsafe {
                let hdc_screen = GetDC(HWND(std::ptr::null_mut()));
                let hdc_mem = CreateCompatibleDC(hdc_screen);
                let mut bmi = BITMAPINFO {
                    bmiHeader: BITMAPINFOHEADER {
                        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                        biWidth: cw as i32,
                        biHeight: -(ch as i32),
                        biPlanes: 1,
                        biBitCount: 32,
                        biCompression: 0,
                        biSizeImage: 0,
                        biXPelsPerMeter: 0,
                        biYPelsPerMeter: 0,
                        biClrUsed: 0,
                        biClrImportant: 0,
                    },
                    bmiColors: [Default::default()],
                };
                let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
                let hbitmap = CreateDIBSection(hdc_screen, &bmi, DIB_RGB_COLORS, &mut bits, None, 0);
                let (result, hb) = if let Ok(hb) = hbitmap {
                    let _ = SelectObject(hdc_mem, hb);
                    let blt_ok = BitBlt(hdc_mem, 0, 0, cw as i32, ch as i32, hdc_screen, x, y, SRCCOPY);
                    if blt_ok.is_ok() {
                        let size = (cw * ch * 4) as usize;
                        let mut rgba = Vec::with_capacity(size);
                        std::ptr::copy_nonoverlapping(bits, rgba.as_mut_ptr() as *mut _, size);
                        rgba.set_len(size);
                        (Some(rgba), Some(hb))
                    } else { (None, Some(hb)) }
                } else { (None, None) };
                if let Some(hb) = hb {
                    let _ = DeleteObject(hb);
                }
                let _ = ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);
                result
            };

            if let Some(rgba) = rgba_opt {
                if let Some(img) = ImageBuffer::<Rgba<u8>, _>::from_raw(cw, ch, rgba) {
                    let mut png_buf = Cursor::new(Vec::new());
                    if img.write_to(&mut png_buf, image::ImageFormat::Png).is_ok() {
                        let b64 = BASE64.encode(png_buf.into_inner());
                        let _ = app_clone.emit("emulator:frame", serde_json::json!({
                            "base64": b64,
                            "width": cw,
                            "height": ch,
                            "format": "png",
                            "frame": FRAME_COUNT.fetch_add(1, Ordering::SeqCst),
                            "timestamp": std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis(),
                        }));
                    }
                }
            }

            // ~15fps
            tokio::time::sleep(std::time::Duration::from_millis(66)).await;
        }
    });

    {
        let mut task = CAPTURE_TASK.lock().unwrap();
        *task = Some(handle);
    }

    Ok("Emulator stream started successfully".to_string())
}

#[tauri::command]
pub async fn stop_emulator_stream() -> Result<String, String> {
    SCRCPY_RUNNING.store(false, Ordering::SeqCst);
    let mut task = CAPTURE_TASK.lock().unwrap();
    if let Some(h) = task.take() {
        h.abort();
    }
    Ok("Stream stopped".to_string())
}

#[derive(Serialize)]
pub struct StreamStatus {
    pub running: bool,
    pub frame_count: usize,
}

#[tauri::command]
pub fn get_stream_status() -> StreamStatus {
    StreamStatus {
        running: SCRCPY_RUNNING.load(Ordering::SeqCst),
        frame_count: FRAME_COUNT.load(Ordering::SeqCst),
    }
}
