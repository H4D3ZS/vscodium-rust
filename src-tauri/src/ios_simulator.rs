//! Xcode iOS Simulator mirroring for macOS (Apple Silicon, Intel, Hackintosh with Xcode).
//! Adapted from codex-plusplus-ios-simulator (MIT) — headless IOSurface capture + HID input.

use serde::Serialize;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
pub struct SimDeviceInfo {
    pub udid: String,
    pub name: String,
    pub runtime: String,
    pub state: String,
    pub booted: bool,
}

#[cfg(target_os = "macos")]
mod mac {
    use super::*;
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use tauri::async_runtime::JoinHandle;

    static MIRROR_RUNNING: AtomicBool = AtomicBool::new(false);
    static CAPTURE_TASK: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
    static INPUT_CHILD: Mutex<Option<std::process::Child>> = Mutex::new(None);
    static STREAM_META: Mutex<Option<StreamMeta>> = Mutex::new(None);

    #[derive(Clone)]
    struct StreamMeta {
        pixel_width: u32,
        pixel_height: u32,
        device_udid: String,
        device_name: String,
    }

    fn helper_source_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tools/ios-simulator/helpers")
    }

    fn cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("com.hades.vscode-rust-app")
            .join("ios-simulator")
    }

    fn ensure_cache_dir() -> PathBuf {
        let d = cache_dir();
        let _ = std::fs::create_dir_all(&d);
        d
    }

    fn developer_dir() -> String {
        Command::new("xcode-select")
            .arg("-p")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "/Applications/Xcode.app/Contents/Developer".to_string())
    }

    fn ensure_sim_capture() -> Result<PathBuf, String> {
        let bin = ensure_cache_dir().join("sim-capture");
        if bin.exists() {
            return Ok(bin);
        }
        let src = helper_source_dir().join("sim-capture.swift");
        if !src.exists() {
            return Err(format!("Missing {}", src.display()));
        }
        let status = Command::new("swiftc")
            .args([
                "-O",
                "-F",
                "/Library/Developer/PrivateFrameworks",
                "-framework",
                "CoreImage",
                "-framework",
                "Foundation",
                "-framework",
                "IOSurface",
                src.to_str().unwrap_or_default(),
                "-o",
                bin.to_str().unwrap_or_default(),
            ])
            .status()
            .map_err(|e| format!("swiftc failed to start: {e}"))?;
        if !status.success() || !bin.exists() {
            return Err("Failed to compile sim-capture (need Xcode + swiftc)".into());
        }
        Ok(bin)
    }

    fn ensure_sim_input() -> Result<PathBuf, String> {
        let bin = ensure_cache_dir().join("sim-input");
        if bin.exists() {
            return Ok(bin);
        }
        let src = helper_source_dir().join("sim-input.m");
        if !src.exists() {
            return Err(format!("Missing {}", src.display()));
        }
        let status = Command::new("clang")
            .args([
                "-fobjc-arc",
                "-O2",
                "-framework",
                "Foundation",
                "-framework",
                "CoreGraphics",
                src.to_str().unwrap_or_default(),
                "-o",
                bin.to_str().unwrap_or_default(),
            ])
            .status()
            .map_err(|e| format!("clang failed to start: {e}"))?;
        if !status.success() || !bin.exists() {
            return Err("Failed to compile sim-input (need Xcode clang)".into());
        }
        Ok(bin)
    }

    fn spawn_input_helper(udid: Option<&str>) -> Result<(), String> {
        let bin = ensure_sim_input()?;
        let mut cmd = Command::new(&bin);
        cmd.stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped());
        if let Some(u) = udid {
            cmd.arg(u);
        }
        let child = cmd.spawn().map_err(|e| format!("sim-input spawn: {e}"))?;
        *INPUT_CHILD.lock().unwrap() = Some(child);
        Ok(())
    }

    fn write_input_event(event: Value) -> Result<(), String> {
        let mut guard = INPUT_CHILD.lock().unwrap();
        let child = guard
            .as_mut()
            .ok_or_else(|| "Input helper not running — start mirror first".to_string())?;
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "sim-input stdin unavailable".to_string())?;
        let line = serde_json::to_string(&event).map_err(|e| e.to_string())?;
        stdin
            .write_all(line.as_bytes())
            .and_then(|_| stdin.write_all(b"\n"))
            .map_err(|e| format!("sim-input write: {e}"))?;
        Ok(())
    }

    fn parse_stderr_meta(line: &str) {
        let Some(json_part) = line.strip_prefix("[sim-capture] ") else {
            return;
        };
        let Ok(v) = serde_json::from_str::<Value>(json_part) else {
            return;
        };
        if v.get("type").and_then(|t| t.as_str()) != Some("stream-started") {
            return;
        }
        *STREAM_META.lock().unwrap() = Some(StreamMeta {
            pixel_width: v.get("pixelWidth").and_then(|x| x.as_u64()).unwrap_or(1170) as u32,
            pixel_height: v.get("pixelHeight").and_then(|x| x.as_u64()).unwrap_or(2532) as u32,
            device_udid: v
                .get("deviceUDID")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            device_name: v
                .get("deviceName")
                .and_then(|x| x.as_str())
                .unwrap_or("iPhone")
                .to_string(),
        });
    }

    pub fn preflight() -> Result<Value, String> {
        let dev_dir = developer_dir();
        let xcode_ok = dev_dir.contains("Xcode.app") || Path::new(&dev_dir).exists();
        let simctl = Command::new("xcrun")
            .args(["-find", "simctl"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        let devices = list_devices_internal().unwrap_or_default();
        let booted = devices.iter().any(|d| d.booted);
        Ok(json!({
            "ok": xcode_ok && simctl && !devices.is_empty(),
            "developer_dir": dev_dir,
            "simctl": simctl,
            "device_count": devices.len(),
            "booted": booted,
            "hint": if !xcode_ok {
                "Run: sudo xcode-select -s /Applications/Xcode.app"
            } else if devices.is_empty() {
                "Download an iOS simulator runtime in Xcode → Settings → Platforms"
            } else if !booted {
                "Boot a simulator from the device picker"
            } else {
                ""
            },
        }))
    }

    fn list_devices_internal() -> Result<Vec<SimDeviceInfo>, String> {
        let output = Command::new("xcrun")
            .args(["simctl", "list", "devices", "available", "--json"])
            .output()
            .map_err(|e| format!("simctl list: {e}"))?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        let v: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        if let Some(devices) = v.get("devices").and_then(|d| d.as_object()) {
            for (runtime, list) in devices {
                if let Some(arr) = list.as_array() {
                    for d in arr {
                        let state = d
                            .get("state")
                            .and_then(|s| s.as_str())
                            .unwrap_or("Shutdown")
                            .to_string();
                        out.push(SimDeviceInfo {
                            udid: d
                                .get("udid")
                                .and_then(|u| u.as_str())
                                .unwrap_or("")
                                .to_string(),
                            name: d
                                .get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("Simulator")
                                .to_string(),
                            runtime: runtime.clone(),
                            booted: state == "Booted",
                            state,
                        });
                    }
                }
            }
        }
        out.sort_by(|a, b| b.booted.cmp(&a.booted).then(a.name.cmp(&b.name)));
        Ok(out)
    }

    pub fn list_devices() -> Result<Vec<SimDeviceInfo>, String> {
        list_devices_internal()
    }

    pub fn boot_device(udid: String) -> Result<String, String> {
        let _ = Command::new("osascript")
            .args(["-e", "tell application \"Simulator\" to quit"])
            .status();
        let output = Command::new("xcrun")
            .args(["simctl", "boot", &udid])
            .output()
            .map_err(|e| format!("simctl boot: {e}"))?;
        if output.status.success()
            || String::from_utf8_lossy(&output.stderr).contains("current state Booted")
        {
            Ok(format!("Booted {udid}"))
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    pub fn boot_default_iphone() -> Result<String, String> {
        let devices = list_devices_internal()?;
        if let Some(d) = devices.iter().find(|d| d.booted) {
            return Ok(format!("Already booted: {}", d.name));
        }
        let pick = devices
            .iter()
            .find(|d| d.name.contains("iPhone") && !d.name.contains("SE"))
            .or_else(|| devices.iter().find(|d| d.name.contains("iPhone")))
            .ok_or_else(|| "No iPhone simulators installed".to_string())?;
        boot_device(pick.udid.clone())
    }

    pub async fn start_mirror(
        app: AppHandle,
        udid: Option<String>,
        auto_boot: bool,
    ) -> Result<Value, String> {
        if MIRROR_RUNNING.load(Ordering::SeqCst) {
            return Ok(json!({ "ok": true, "message": "Already mirroring" }));
        }

        if auto_boot {
            let devices = list_devices_internal()?;
            let target_booted = udid
                .as_ref()
                .and_then(|u| devices.iter().find(|d| d.udid == *u))
                .map(|d| d.booted)
                .unwrap_or(false);
            if !devices.iter().any(|d| d.booted) && !target_booted {
                if let Some(ref u) = udid {
                    boot_device(u.clone())?;
                } else {
                    boot_default_iphone()?;
                }
                for _ in 0..40 {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    if list_devices_internal()?.iter().any(|d| d.booted) {
                        break;
                    }
                }
            }
        }

        let capture_bin = ensure_sim_capture()?;
        spawn_input_helper(udid.as_deref())?;

        let mut child = Command::new(&capture_bin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("sim-capture spawn: {e}"))?;

        if let Some(stderr) = child.stderr.take() {
            thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                    parse_stderr_meta(&line);
                }
            });
        }

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "sim-capture stdout missing".to_string())?;

        MIRROR_RUNNING.store(true, Ordering::SeqCst);
        let app_clone = app.clone();

        let handle = tauri::async_runtime::spawn(async move {
            let mut reader = std::io::BufReader::new(stdout);
            let mut frame_index: u64 = 0;
            while MIRROR_RUNNING.load(Ordering::SeqCst) {
                let mut len_buf = [0u8; 4];
                if reader.read_exact(&mut len_buf).is_err() {
                    break;
                }
                let len = u32::from_be_bytes(len_buf) as usize;
                if len == 0 || len > 20_000_000 {
                    break;
                }
                let mut jpeg = vec![0u8; len];
                if reader.read_exact(&mut jpeg).is_err() {
                    break;
                }
                let meta = STREAM_META.lock().unwrap().clone();
                let b64 = BASE64.encode(&jpeg);
                let (w, h, name, udid_s) = meta
                    .map(|m| (m.pixel_width, m.pixel_height, m.device_name, m.device_udid))
                    .unwrap_or((1170, 2532, "iPhone".to_string(), String::new()));
                let _ = app_clone.emit(
                    "ios-simulator:frame",
                    json!({
                        "base64": b64,
                        "width": w,
                        "height": h,
                        "frame": frame_index,
                        "deviceName": name,
                        "deviceUdid": udid_s,
                    }),
                );
                frame_index += 1;
            }
            MIRROR_RUNNING.store(false, Ordering::SeqCst);
        });

        *CAPTURE_TASK.lock().unwrap() = Some(handle);
        Ok(json!({ "ok": true, "message": "iOS Simulator mirror started" }))
    }

    pub async fn stop_mirror() -> Result<String, String> {
        MIRROR_RUNNING.store(false, Ordering::SeqCst);
        if let Some(h) = CAPTURE_TASK.lock().unwrap().take() {
            h.abort();
        }
        if let Some(mut child) = INPUT_CHILD.lock().unwrap().take() {
            let _ = child.kill();
        }
        Ok("iOS Simulator mirror stopped".into())
    }

    pub fn send_touch(x_ratio: f64, y_ratio: f64, phase: &str) -> Result<(), String> {
        write_input_event(json!({
            "type": "touch",
            "phase": phase,
            "x": x_ratio.clamp(0.0, 1.0),
            "y": y_ratio.clamp(0.0, 1.0),
        }))
    }

    pub fn send_home() -> Result<(), String> {
        write_input_event(json!({ "type": "button-tap", "name": "home" }))
    }

    pub fn is_running() -> bool {
        MIRROR_RUNNING.load(Ordering::SeqCst)
    }
}

#[cfg(not(target_os = "macos"))]
fn mac_only_msg() -> String {
    "Xcode iOS Simulator mirroring requires macOS with full Xcode installed.".into()
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_preflight() -> Result<Value, String> {
    Ok(json!({
        "ok": false,
        "hint": "Use acheron / vPhone on Windows and Linux. Xcode Simulator mirror is macOS-only."
    }))
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_list_devices() -> Result<Vec<SimDeviceInfo>, String> {
    Err(mac_only_msg())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn ios_sim_boot_device(_udid: String) -> Result<String, String> {
    Err(mac_only_msg())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn ios_sim_start_mirror(
    _app: AppHandle,
    _udid: Option<String>,
    _auto_boot: Option<bool>,
) -> Result<Value, String> {
    Err(mac_only_msg())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn ios_sim_stop_mirror() -> Result<String, String> {
    Err(mac_only_msg())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_send_touch(_x_ratio: f64, _y_ratio: f64, _phase: String) -> Result<(), String> {
    Err(mac_only_msg())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_send_home() -> Result<(), String> {
    Err(mac_only_msg())
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_mirror_running() -> bool {
    false
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_preflight() -> Result<Value, String> {
    mac::preflight()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_list_devices() -> Result<Vec<SimDeviceInfo>, String> {
    mac::list_devices()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn ios_sim_boot_device(udid: String) -> Result<String, String> {
    mac::boot_device(udid)
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn ios_sim_start_mirror(
    app: AppHandle,
    udid: Option<String>,
    auto_boot: Option<bool>,
) -> Result<Value, String> {
    mac::start_mirror(app, udid, auto_boot.unwrap_or(true)).await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn ios_sim_stop_mirror() -> Result<String, String> {
    mac::stop_mirror().await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_send_touch(x_ratio: f64, y_ratio: f64, phase: String) -> Result<(), String> {
    mac::send_touch(x_ratio, y_ratio, &phase)
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_send_home() -> Result<(), String> {
    mac::send_home()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_mirror_running() -> bool {
    mac::is_running()
}
