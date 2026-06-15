//! In-IDE iOS Simulator — headless CoreSimulator + native Swift NSView (default).

use serde::Serialize;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::AppHandle;

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
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::{Duration, Instant};
    use sysinfo::System;
    use tauri::async_runtime::JoinHandle;

    static MIRROR_RUNNING: AtomicBool = AtomicBool::new(false);
    static PAUSED: AtomicBool = AtomicBool::new(false);
    static CAPTURE_TASK: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
    static CAPTURE_CHILD: Mutex<Option<std::process::Child>> = Mutex::new(None);
    static INPUT_CHILD: Mutex<Option<std::process::Child>> = Mutex::new(None);
    static STREAM_META: Mutex<Option<StreamMeta>> = Mutex::new(None);
    static ACTIVE_UDID: Mutex<String> = Mutex::new(String::new());
    static PROFILE: Mutex<MirrorProfile> = Mutex::new(MirrorProfile::BALANCED);
    static DEVICE_CACHE: Mutex<Option<(Instant, Vec<SimDeviceInfo>)>> = Mutex::new(None);
    static STREAM_READY: AtomicBool = AtomicBool::new(false);
    static FRAMES_EMITTED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    const DEVICE_CACHE_TTL: Duration = Duration::from_secs(15);
    const HELPERS_VERSION: &str = "10";

    #[derive(Clone, Copy)]
    struct MirrorProfile {
        /// Sole encode throttle (ms between JPEGs). Rust does not re-throttle.
        frame_interval_ms: u64,
        capture_scale: f64,
        jpeg_quality: f64,
        /// Long-edge pixel cap — sized for sidebar embed, not full device buffer.
        max_px: u32,
        label: &'static str,
    }

    impl MirrorProfile {
        const EFFICIENT: Self = Self {
            frame_interval_ms: 33,
            capture_scale: 1.0,
            jpeg_quality: 0.42,
            max_px: 480,
            label: "efficient",
        };
        const BALANCED: Self = Self {
            frame_interval_ms: 33,
            capture_scale: 1.0,
            jpeg_quality: 0.45,
            max_px: 600,
            label: "balanced",
        };
        const SMOOTH: Self = Self {
            frame_interval_ms: 16,
            capture_scale: 1.0,
            jpeg_quality: 0.48,
            max_px: 780,
            label: "smooth",
        };
    }

    fn detect_mirror_profile() -> MirrorProfile {
        let mut sys = System::new();
        sys.refresh_memory();
        let gb = sys.total_memory() / (1024 * 1024 * 1024);
        if gb <= 8 {
            MirrorProfile::EFFICIENT
        } else if gb <= 16 {
            MirrorProfile::BALANCED
        } else {
            MirrorProfile::SMOOTH
        }
    }

    fn active_profile() -> MirrorProfile {
        *PROFILE.lock().unwrap()
    }

    fn invalidate_device_cache() {
        *DEVICE_CACHE.lock().unwrap() = None;
    }

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

    fn helpers_version_ok() -> bool {
        let stamp = cache_dir().join(".helpers-version");
        stamp.exists()
            && std::fs::read_to_string(&stamp)
                .map(|v| v.trim() == HELPERS_VERSION)
                .unwrap_or(false)
    }

    fn stamp_helpers_version() {
        let _ = std::fs::write(cache_dir().join(".helpers-version"), HELPERS_VERSION);
    }

    fn invalidate_helpers_if_stale() {
        if helpers_version_ok() {
            return;
        }
        let _ = std::fs::remove_file(cache_dir().join("sim-capture"));
        let _ = std::fs::remove_file(cache_dir().join("sim-input"));
        let _ = std::fs::remove_file(cache_dir().join("sim-window"));
        let _ = std::fs::remove_file(cache_dir().join(".helpers-version"));
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

    fn helper_stale(src: &Path, bin: &Path) -> bool {
        if !bin.exists() {
            return true;
        }
        let src_m = std::fs::metadata(src).and_then(|m| m.modified()).ok();
        let bin_m = std::fs::metadata(bin).and_then(|m| m.modified()).ok();
        match (src_m, bin_m) {
            (Some(s), Some(b)) => s > b,
            _ => true,
        }
    }

    fn ensure_sim_capture() -> Result<PathBuf, String> {
        invalidate_helpers_if_stale();
        let bin = ensure_cache_dir().join("sim-capture");
        let src = helper_source_dir().join("sim-capture.swift");
        if bin.exists() && !helper_stale(&src, &bin) && helpers_version_ok() {
            return Ok(bin);
        }
        if !src.exists() {
            return Err(format!("Missing {}", src.display()));
        }
        let dev_dir = developer_dir();
        let output = Command::new("swiftc")
            .args([
                "-O",
                "-F",
                "/Library/Developer/PrivateFrameworks",
                "-F",
                &format!("{dev_dir}/Platforms/iPhoneSimulator.platform/Developer/Library/PrivateFrameworks"),
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
            .output()
            .map_err(|e| format!("swiftc failed to start: {e}"))?;
        if !output.status.success() || !bin.exists() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let detail = stderr.trim();
            let msg = if detail.is_empty() {
                "Failed to compile sim-capture — install full Xcode and run: sudo xcode-select -s /Applications/Xcode.app".to_string()
            } else {
                format!("sim-capture compile failed: {detail}")
            };
            return Err(msg);
        }
        stamp_helpers_version();
        Ok(bin)
    }

    fn ensure_sim_input() -> Result<PathBuf, String> {
        invalidate_helpers_if_stale();
        let bin = ensure_cache_dir().join("sim-input");
        let src = helper_source_dir().join("sim-input.m");
        if bin.exists() && !helper_stale(&src, &bin) && helpers_version_ok() {
            return Ok(bin);
        }
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
        stamp_helpers_version();
        Ok(bin)
    }

    fn spawn_input_helper(udid: Option<&str>) -> Result<(), String> {
        kill_child(&INPUT_CHILD);
        let bin = ensure_sim_input()?;
        let mut cmd = Command::new(&bin);
        cmd.stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped());
        if let Some(u) = udid {
            cmd.arg(u);
        }
        let mut child = cmd.spawn().map_err(|e| format!("sim-input spawn: {e}"))?;
        if let Some(stderr) = child.stderr.take() {
            std::thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                    if !line.trim().is_empty() {
                        eprintln!("[sim-input] {line}");
                    }
                }
            });
        }
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

    fn parse_stderr_line(line: &str) {
        let Some(json_part) = line.strip_prefix("[sim-capture] ") else {
            return;
        };
        let Ok(v) = serde_json::from_str::<Value>(json_part) else {
            return;
        };
        if v.get("type").and_then(|t| t.as_str()) != Some("stream-started") {
            return;
        }
        let w = v.get("pixelWidth").and_then(|x| x.as_u64()).unwrap_or(1170) as u32;
        let h = v.get("pixelHeight").and_then(|x| x.as_u64()).unwrap_or(2532) as u32;
        let udid = v
            .get("deviceUDID")
            .and_then(|x| x.as_str())
            .unwrap_or("");
        let name = v
            .get("deviceName")
            .and_then(|x| x.as_str())
            .unwrap_or("iPhone");
        *STREAM_META.lock().unwrap() = Some(StreamMeta {
            pixel_width: w,
            pixel_height: h,
            device_udid: udid.to_string(),
            device_name: name.to_string(),
        });
        crate::ios_stream::set_meta(w, h, name, udid);
    }

    fn shutdown_other_booted(except_udid: Option<&str>) -> Result<(), String> {
        for d in list_devices_internal()?.into_iter().filter(|d| d.booted) {
            if except_udid == Some(d.udid.as_str()) {
                continue;
            }
            let _ = Command::new("xcrun")
                .args(["simctl", "shutdown", &d.udid])
                .output();
        }
        Ok(())
    }

    fn dismiss_external_simulator_app() {
        // simctl boot often opens Simulator.app in its own window. CoreSimulator keeps
        // running headlessly after quit — our IOSurface panel does not need the app UI.
        let _ = Command::new("osascript")
            .args(["-e", "tell application \"Simulator\" to quit"])
            .output();
        thread::sleep(Duration::from_millis(150));
    }

    async fn ensure_simulator_ready(udid: &str, auto_boot: bool) -> Result<(), String> {
        if auto_boot {
            if !simulator_boot_unnecessary(udid)? {
                boot_device(udid.to_string())?;
            }
            wait_for_boot(Some(udid), 60_000).await
        } else {
            invalidate_device_cache();
            let booted = list_devices_internal()?
                .iter()
                .any(|d| d.udid == udid && d.booted);
            if booted {
                Ok(())
            } else {
                Err("No booted simulator — enable auto-boot or boot a device first".into())
            }
        }
    }

    async fn wait_for_boot(udid: Option<&str>, max_ms: u64) -> Result<(), String> {
        let steps = max_ms / 200;
        for _ in 0..steps {
            invalidate_device_cache();
            let devices = list_devices_internal()?;
            let ready = if let Some(u) = udid {
                devices.iter().any(|d| d.udid == u && d.booted)
            } else {
                devices.iter().any(|d| d.booted)
            };
            if ready {
                return Ok(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        Err("Timed out waiting for simulator to boot — check Xcode runtimes".into())
    }

    /// Compile capture/input helpers off the hot path (panel mount / preflight).
    pub fn warmup_helpers() {
        crate::ios_sim_embed::warmup();
        invalidate_helpers_if_stale();
        thread::spawn(|| {
            let _ = ensure_sim_capture();
            let _ = ensure_sim_input();
        });
    }

    fn kill_child(slot: &Mutex<Option<std::process::Child>>) {
        if let Some(mut child) = slot.lock().unwrap().take() {
            let _ = child.kill();
            let _ = child.wait();
        }
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
        let profile = detect_mirror_profile();
        Ok(json!({
            "ok": xcode_ok && simctl && !devices.is_empty(),
            "developer_dir": dev_dir,
            "simctl": simctl,
            "device_count": devices.len(),
            "booted": booted,
            "profile": profile.label,
            "mode": mirror_mode_label(),
            "accessibility_hint": if crate::ios_sim_embed::embed_mode_enabled() {
                "IOS_SIM_EMBED=1: grant Accessibility for sim-window helper."
            } else {
                ""
            },
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

    fn list_devices_internal_fresh() -> Result<Vec<SimDeviceInfo>, String> {
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

    fn list_devices_internal() -> Result<Vec<SimDeviceInfo>, String> {
        let mut cache = DEVICE_CACHE.lock().unwrap();
        if let Some((at, list)) = cache.as_ref() {
            if at.elapsed() < DEVICE_CACHE_TTL {
                return Ok(list.clone());
            }
        }
        let list = list_devices_internal_fresh()?;
        *cache = Some((Instant::now(), list.clone()));
        Ok(list)
    }

    pub fn list_devices() -> Result<Vec<SimDeviceInfo>, String> {
        list_devices_internal()
    }

    fn mirror_mode_label() -> &'static str {
        if crate::ios_sim_native::stream_mode_enabled() {
            "stream"
        } else if crate::ios_sim_embed::embed_mode_enabled() {
            "embed"
        } else {
            "native"
        }
    }

    pub fn session_state() -> Value {
        let udid = ACTIVE_UDID.lock().unwrap().clone();
        let profile = active_profile();
        let meta = STREAM_META.lock().unwrap().clone();
        json!({
            "running": MIRROR_RUNNING.load(Ordering::SeqCst),
            "paused": PAUSED.load(Ordering::SeqCst),
            "udid": if udid.is_empty() { Value::Null } else { json!(udid) },
            "profile": profile.label,
            "frame_interval_ms": profile.frame_interval_ms,
            "mode": mirror_mode_label(),
            "width": meta.as_ref().map(|m| m.pixel_width).unwrap_or(393),
            "height": meta.as_ref().map(|m| m.pixel_height).unwrap_or(852),
        })
    }

    pub fn pause_mirror() {
        PAUSED.store(true, Ordering::SeqCst);
        crate::ios_sim_native::pause();
        crate::ios_sim_embed::pause();
        crate::ios_stream::set_paused(true);
    }

    pub fn resume_mirror() {
        PAUSED.store(false, Ordering::SeqCst);
        crate::ios_sim_native::resume();
        crate::ios_sim_embed::resume();
        crate::ios_stream::set_paused(false);
    }

    async fn start_native_mirror(
        app: AppHandle,
        target_udid: Option<String>,
        auto_boot: bool,
        profile: MirrorProfile,
    ) -> Result<Value, String> {
        let udid = target_udid
            .clone()
            .ok_or_else(|| "No simulator device selected".to_string())?;

        if auto_boot {
            ensure_simulator_ready(&udid, true).await?;
        } else if !list_devices_internal()?
            .iter()
            .any(|d| d.udid == udid && d.booted)
        {
            return Err("No booted simulator — enable auto-boot or boot a device first".into());
        }

        // Headless native path — never leave Simulator.app floating outside the IDE panel.
        crate::ios_sim_embed::stop();
        dismiss_external_simulator_app();

        let target_booted = list_devices_internal()?
            .iter()
            .any(|d| d.udid == udid && d.booted);

        let app_start = app.clone();
        let udid_start = udid.clone();
        crate::ios_sim_native::start(&app_start, &udid_start)?;

        spawn_input_helper(Some(&udid))?;

        *ACTIVE_UDID.lock().unwrap() = udid.clone();
        MIRROR_RUNNING.store(true, Ordering::SeqCst);
        PAUSED.store(false, Ordering::SeqCst);
        STREAM_READY.store(true, Ordering::SeqCst);

        let device_name = list_devices_internal()
            .ok()
            .and_then(|devs| devs.iter().find(|d| d.udid == udid).map(|d| d.name.clone()))
            .unwrap_or_else(|| "iPhone".to_string());

        *STREAM_META.lock().unwrap() = Some(StreamMeta {
            pixel_width: 393,
            pixel_height: 852,
            device_udid: udid.clone(),
            device_name: device_name.clone(),
        });

        Ok(json!({
            "ok": true,
            "mode": "native",
            "udid": udid,
            "already_booted": target_booted,
            "profile": profile.label,
            "width": 393,
            "height": 852,
            "device_name": device_name,
        }))
    }

    fn is_already_booted_boot_err(text: &str) -> bool {
        let s = text.to_ascii_lowercase();
        s.contains("current state: booted")
            || s.contains("current state booted")
            || s.contains("unable to boot device in current state")
            || s.contains("code=405")
            || s.contains("code: 405")
    }

    fn simulator_boot_unnecessary(udid: &str) -> Result<bool, String> {
        invalidate_device_cache();
        Ok(list_devices_internal_fresh()?
            .into_iter()
            .find(|d| d.udid == udid)
            .map(|d| d.booted || d.state == "Booting")
            .unwrap_or(false))
    }

    pub fn boot_device(udid: String) -> Result<String, String> {
        if simulator_boot_unnecessary(&udid)? {
            return Ok(format!("Already booted: {udid}"));
        }
        let output = Command::new("xcrun")
            .args(["simctl", "boot", &udid])
            .output()
            .map_err(|e| format!("simctl boot: {e}"))?;
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        );
        if output.status.success() || is_already_booted_boot_err(&combined) {
            invalidate_device_cache();
            Ok(format!("Booted {udid}"))
        } else {
            Err(combined.trim().to_string())
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

    async fn start_embed_mirror(
        target_udid: Option<String>,
        auto_boot: bool,
        profile: MirrorProfile,
    ) -> Result<Value, String> {
        let udid = target_udid
            .clone()
            .ok_or_else(|| "No simulator device selected".to_string())?;

        if auto_boot {
            ensure_simulator_ready(&udid, true).await?;
        } else if !list_devices_internal()?
            .iter()
            .any(|d| d.udid == udid && d.booted)
        {
            return Err("No booted simulator — enable auto-boot or boot a device first".into());
        }

        let target_booted = list_devices_internal()?
            .iter()
            .any(|d| d.udid == udid && d.booted);

        let open_fut = tauri::async_runtime::spawn_blocking({
            let u = udid.clone();
            move || crate::ios_sim_embed::open_simulator(&u)
        });
        let input_fut = {
            let u = udid.clone();
            tauri::async_runtime::spawn_blocking(move || spawn_input_helper(Some(&u)))
        };
        let (open_res, input_res) = tokio::join!(open_fut, input_fut);
        let open = open_res.map_err(|e| e.to_string())??;
        input_res.map_err(|e| e.to_string())??;

        if !open.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Err(
                open.get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("sim-window open failed")
                    .to_string(),
            );
        }

        *ACTIVE_UDID.lock().unwrap() = udid.clone();
        MIRROR_RUNNING.store(true, Ordering::SeqCst);
        PAUSED.store(false, Ordering::SeqCst);
        STREAM_READY.store(true, Ordering::SeqCst);

        crate::ios_sim_embed::start_docking();

        let width = open.get("width").and_then(|v| v.as_f64()).unwrap_or(393.0) as u32;
        let height = open.get("height").and_then(|v| v.as_f64()).unwrap_or(852.0) as u32;
        let device_name = target_udid
            .as_ref()
            .and_then(|u| list_devices_internal().ok()?.iter().find(|d| d.udid == *u).map(|d| d.name.clone()))
            .unwrap_or_else(|| "iPhone".to_string());

        *STREAM_META.lock().unwrap() = Some(StreamMeta {
            pixel_width: width,
            pixel_height: height,
            device_udid: udid.clone(),
            device_name: device_name.clone(),
        });

        Ok(json!({
            "ok": true,
            "mode": "embed",
            "udid": udid,
            "already_booted": target_booted,
            "profile": profile.label,
            "width": width,
            "height": height,
            "device_name": device_name,
        }))
    }

    async fn start_stream_mirror(
        target_udid: Option<String>,
        auto_boot: bool,
        profile: MirrorProfile,
    ) -> Result<Value, String> {
        let target_booted = if let Some(ref u) = target_udid {
            invalidate_device_cache();
            list_devices_internal()?
                .iter()
                .any(|d| d.udid == *u && d.booted)
        } else {
            list_devices_internal()?.iter().any(|d| d.booted)
        };

        if auto_boot {
            if let Some(ref u) = target_udid {
                ensure_simulator_ready(u, true).await?;
            } else if !target_booted {
                boot_default_iphone()?;
                wait_for_boot(None, 30_000).await?;
            }
        } else if !target_booted {
            return Err("No booted simulator — enable auto-boot or boot a device first".into());
        }

        let capture_bin = tauri::async_runtime::spawn_blocking(ensure_sim_capture)
            .await
            .map_err(|e| e.to_string())??;

        let mut child = Command::new(&capture_bin);
        child
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env(
                "SIM_CAPTURE_SCALE",
                format!("{:.2}", profile.capture_scale),
            )
            .env(
                "SIM_CAPTURE_QUALITY",
                format!("{:.2}", profile.jpeg_quality),
            )
            .env(
                "SIM_CAPTURE_MIN_MS",
                profile.frame_interval_ms.to_string(),
            )
            .env("SIM_CAPTURE_MAX_PX", profile.max_px.to_string());
        if let Some(ref u) = target_udid {
            child.arg(u);
        }
        let mut child = child
            .spawn()
            .map_err(|e| format!("sim-capture spawn: {e}"))?;

        if let Some(stderr) = child.stderr.take() {
            thread::spawn(move || {
                use std::io::{BufRead, BufReader};
                for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                    parse_stderr_line(&line);
                }
            });
        }

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "sim-capture stdout missing".to_string())?;

        *CAPTURE_CHILD.lock().unwrap() = Some(child);

        // Input helper after boot + capture attach (HID needs booted device).
        spawn_input_helper(target_udid.as_deref())?;

        if let Some(ref u) = target_udid {
            *ACTIVE_UDID.lock().unwrap() = u.clone();
        }

        let stream_url = crate::ios_stream::ensure_server().await?;
        crate::ios_stream::set_running(true);
        crate::ios_stream::set_paused(false);

        MIRROR_RUNNING.store(true, Ordering::SeqCst);

        let (frame_tx, mut frame_rx) = tokio::sync::mpsc::channel::<bytes::Bytes>(3);
        std::thread::Builder::new()
            .name("ios-capture-pipe".into())
            .spawn(move || {
                use std::io::Read;
                let mut reader = std::io::BufReader::new(stdout);
                let mut len_buf = [0u8; 4];
                while MIRROR_RUNNING.load(Ordering::SeqCst) {
                    if reader.read_exact(&mut len_buf).is_err() {
                        break;
                    }
                    let len = u32::from_be_bytes(len_buf) as usize;
                    if len == 0 || len > 2_000_000 {
                        break;
                    }
                    let mut jpeg = Vec::with_capacity(len);
                    jpeg.resize(len, 0);
                    if reader.read_exact(&mut jpeg).is_err() {
                        break;
                    }
                    if PAUSED.load(Ordering::SeqCst) {
                        continue;
                    }
                    if frame_tx.blocking_send(bytes::Bytes::from(jpeg)).is_err() {
                        break;
                    }
                }
                MIRROR_RUNNING.store(false, Ordering::SeqCst);
            })
            .map_err(|e| format!("capture pipe thread: {e}"))?;

        let handle = tauri::async_runtime::spawn(async move {
            while let Some(jpeg) = frame_rx.recv().await {
                crate::ios_stream::publish_frame(jpeg);
                FRAMES_EMITTED.fetch_add(1, Ordering::SeqCst);
                STREAM_READY.store(true, Ordering::SeqCst);
            }
            crate::ios_stream::set_running(false);
        });

        *CAPTURE_TASK.lock().unwrap() = Some(handle);

        let meta = STREAM_META.lock().unwrap().clone();
        Ok(json!({
            "ok": true,
            "mode": "stream",
            "udid": target_udid,
            "already_booted": target_booted,
            "profile": profile.label,
            "stream_url": stream_url,
            "width": meta.as_ref().map(|m| m.pixel_width).unwrap_or(393),
            "height": meta.as_ref().map(|m| m.pixel_height).unwrap_or(852),
            "device_name": meta.as_ref().map(|m| m.device_name.clone()).unwrap_or_else(|| "iPhone".into()),
        }))
    }

    pub async fn start_mirror(
        app: AppHandle,
        udid: Option<String>,
        auto_boot: bool,
    ) -> Result<Value, String> {
        *PROFILE.lock().unwrap() = detect_mirror_profile();
        let profile = active_profile();
        crate::ios_stream::set_profile(profile.label);

        let target_udid = udid.clone().or_else(|| {
            list_devices_internal()
                .ok()
                .and_then(|devs| {
                    devs.iter()
                        .find(|d| d.booted)
                        .or_else(|| devs.iter().find(|d| d.name.contains("iPhone")))
                        .map(|d| d.udid.clone())
                })
        });

        let stream_mode = crate::ios_sim_native::stream_mode_enabled();
        let embed_mode = crate::ios_sim_embed::embed_mode_enabled();

        if MIRROR_RUNNING.load(Ordering::SeqCst) {
            let active = ACTIVE_UDID.lock().unwrap().clone();
            if target_udid.as_deref() == Some(active.as_str()) {
                PAUSED.store(false, Ordering::SeqCst);
                crate::ios_sim_native::resume();
                crate::ios_sim_embed::resume();
                crate::ios_stream::set_paused(false);
                let meta = STREAM_META.lock().unwrap().clone();
                let mut reused = json!({
                    "ok": true,
                    "reused": true,
                    "udid": target_udid,
                    "profile": profile.label,
                    "width": meta.as_ref().map(|m| m.pixel_width).unwrap_or(393),
                    "height": meta.as_ref().map(|m| m.pixel_height).unwrap_or(852),
                    "device_name": meta.as_ref().map(|m| m.device_name.clone()).unwrap_or_else(|| "iPhone".into()),
                    "mode": mirror_mode_label(),
                });
                if stream_mode {
                    reused["stream_url"] = json!(crate::ios_stream::stream_url().unwrap_or_default());
                }
                return Ok(reused);
            }
            stop_mirror().await?;
        }

        let prev_udid = ACTIVE_UDID.lock().unwrap().clone();
        if let Some(ref u) = target_udid {
            if prev_udid.is_empty() || prev_udid != *u {
                shutdown_other_booted(Some(u))?;
            }
        }

        STREAM_READY.store(false, Ordering::SeqCst);
        FRAMES_EMITTED.store(0, Ordering::SeqCst);
        *STREAM_META.lock().unwrap() = None;
        PAUSED.store(false, Ordering::SeqCst);

        if stream_mode {
            return start_stream_mirror(target_udid, auto_boot, profile).await;
        }
        if embed_mode {
            return start_embed_mirror(target_udid, auto_boot, profile).await;
        }
        start_native_mirror(app, target_udid, auto_boot, profile).await
    }

    pub async fn stop_mirror() -> Result<String, String> {
        MIRROR_RUNNING.store(false, Ordering::SeqCst);
        PAUSED.store(false, Ordering::SeqCst);
        STREAM_READY.store(false, Ordering::SeqCst);
        FRAMES_EMITTED.store(0, Ordering::SeqCst);
        *ACTIVE_UDID.lock().unwrap() = String::new();
        if let Some(h) = CAPTURE_TASK.lock().unwrap().take() {
            h.abort();
        }
        kill_child(&CAPTURE_CHILD);
        kill_child(&INPUT_CHILD);
        *STREAM_META.lock().unwrap() = None;
        crate::ios_sim_native::stop();
        crate::ios_sim_embed::stop();
        if !crate::ios_sim_embed::embed_mode_enabled() && !crate::ios_sim_native::stream_mode_enabled() {
            dismiss_external_simulator_app();
        }
        crate::ios_stream::stop_server();
        Ok("iOS Simulator mirror stopped".into())
    }

    pub fn send_touch(x_ratio: f64, y_ratio: f64, phase: &str) -> Result<(), String> {
        let event = json!({
            "type": "touch",
            "phase": phase,
            "x": x_ratio.clamp(0.0, 1.0),
            "y": y_ratio.clamp(0.0, 1.0),
        });
        match write_input_event(event.clone()) {
            Err(e) if e.contains("not running") || e.contains("stdin unavailable") => {
                let udid = ACTIVE_UDID.lock().unwrap().clone();
                if udid.is_empty() {
                    return Err(e);
                }
                spawn_input_helper(Some(&udid))?;
                write_input_event(event)
            }
            other => other,
        }
    }

    pub fn update_native_surface_size(w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        if let Some(meta) = STREAM_META.lock().unwrap().as_mut() {
            meta.pixel_width = w;
            meta.pixel_height = h;
        } else {
            let udid = ACTIVE_UDID.lock().unwrap().clone();
            *STREAM_META.lock().unwrap() = Some(StreamMeta {
                pixel_width: w,
                pixel_height: h,
                device_udid: udid,
                device_name: "iPhone".into(),
            });
        }
    }

    pub fn send_home() -> Result<(), String> {
        write_input_event(json!({ "type": "button-tap", "name": "home" }))
    }

    pub fn capture_screenshot() -> Result<String, String> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let udid = {
            let active = ACTIVE_UDID.lock().unwrap().clone();
            if !active.is_empty() {
                active
            } else {
                list_devices_internal()?
                    .into_iter()
                    .find(|d| d.booted)
                    .map(|d| d.udid)
                    .ok_or_else(|| "No booted simulator — start mirror or boot a device".to_string())?
            }
        };

        let path = ensure_cache_dir().join(format!(
            "screenshot-{}.png",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0)
        ));
        let path_str = path.to_str().ok_or("invalid screenshot path")?;

        let output = Command::new("xcrun")
            .args(["simctl", "io", &udid, "screenshot", path_str])
            .output()
            .map_err(|e| format!("simctl screenshot: {e}"))?;
        if !output.status.success() {
            let detail = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout),
            )
            .trim()
            .to_string();
            return Err(if detail.is_empty() {
                "simctl screenshot failed".into()
            } else {
                detail
            });
        }

        let bytes = std::fs::read(&path).map_err(|e| format!("read screenshot: {e}"))?;
        let _ = std::fs::remove_file(&path);
        if bytes.is_empty() {
            return Err("screenshot file empty".into());
        }
        Ok(STANDARD.encode(bytes))
    }

    pub fn is_running() -> bool {
        MIRROR_RUNNING.load(Ordering::SeqCst)
    }
}

#[cfg(target_os = "macos")]
pub fn touch_forward(x_ratio: f64, y_ratio: f64, phase: &str) -> Result<(), String> {
    mac::send_touch(x_ratio, y_ratio, phase)
}

#[cfg(target_os = "macos")]
pub fn native_surface_size(w: u32, h: u32) {
    mac::update_native_surface_size(w, h);
}

#[cfg(not(target_os = "macos"))]
pub fn touch_forward(_x_ratio: f64, _y_ratio: f64, _phase: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn native_surface_size(_w: u32, _h: u32) {}

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
pub fn ios_sim_capture_screenshot() -> Result<String, String> {
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
pub fn ios_sim_capture_screenshot() -> Result<String, String> {
    mac::capture_screenshot()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_mirror_running() -> bool {
    mac::is_running()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_warmup() {
    mac::warmup_helpers();
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_pause() {
    mac::pause_mirror();
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_resume() {
    mac::resume_mirror();
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_session_state() -> Value {
    mac::session_state()
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_stream_url() -> Option<String> {
    crate::ios_stream::stream_url()
}

#[tauri::command]
pub async fn ios_sim_stream_status() -> crate::ios_stream::StreamStatus {
    crate::ios_stream::stream_status().await
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_stream_url() -> Option<String> {
    None
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_warmup() {}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_pause() {}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_resume() {}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_session_state() -> Value {
    json!({ "running": false, "paused": false })
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn ios_sim_embed_layout(
    app: AppHandle,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> Result<(), String> {
    if crate::ios_sim_native::stream_mode_enabled() {
        return Ok(());
    }
    if crate::ios_sim_embed::embed_mode_enabled() {
        return crate::ios_sim_embed::apply_layout_from_webview(&app, x, y, width, height);
    }
    crate::ios_sim_native::apply_layout(&app, x, y, width, height)
}

#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub fn ios_sim_embed_layout(
    _app: AppHandle,
    _x: f64,
    _y: f64,
    _width: f64,
    _height: f64,
) -> Result<(), String> {
    Err("iOS sim embed requires macOS".into())
}
