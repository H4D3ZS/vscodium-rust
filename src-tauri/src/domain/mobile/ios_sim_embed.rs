//! Native Simulator.app window embedding (macOS).
//! Launches Simulator programmatically and docks its window over an IDE panel viewport
//! via Accessibility positioning — same Metal renderer as Xcode, zero JPEG pipeline.

#[cfg(target_os = "macos")]
mod mac {
    use serde_json::Value;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Mutex;
    use std::thread;
    use std::time::Duration;
    use tauri::{AppHandle, Manager};

    static EMBED_ACTIVE: AtomicBool = AtomicBool::new(false);
    static EMBED_PAUSED: AtomicBool = AtomicBool::new(false);
    static LAYOUT: Mutex<(f64, f64, f64, f64)> = Mutex::new((0.0, 0.0, 0.0, 0.0));
    static DOCK_THREAD: Mutex<Option<thread::JoinHandle<()>>> = Mutex::new(None);
    static WINDOW_BIN: Mutex<Option<PathBuf>> = Mutex::new(None);

    const SIM_WINDOW_VERSION: &str = "2";

    fn cache_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("com.hades.vscode-rust-app")
            .join("ios-simulator")
    }

    fn helper_source() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../tools/ios-simulator/helpers/sim-window.swift")
    }

    fn sim_window_version_ok() -> bool {
        let stamp = cache_dir().join(".sim-window-version");
        stamp.exists()
            && std::fs::read_to_string(&stamp)
                .map(|v| v.trim() == SIM_WINDOW_VERSION)
                .unwrap_or(false)
    }

    fn stamp_sim_window_version() {
        let _ = std::fs::create_dir_all(cache_dir());
        let _ = std::fs::write(cache_dir().join(".sim-window-version"), SIM_WINDOW_VERSION);
    }

    fn ensure_sim_window() -> Result<PathBuf, String> {
        let bin = cache_dir().join("sim-window");
        let src = helper_source();
        if bin.exists() && sim_window_version_ok() {
            if let Ok(src_m) = std::fs::metadata(&src).and_then(|m| m.modified()) {
                if let Ok(bin_m) = std::fs::metadata(&bin).and_then(|m| m.modified()) {
                    if bin_m >= src_m {
                        *WINDOW_BIN.lock().unwrap() = Some(bin.clone());
                        return Ok(bin);
                    }
                }
            }
        }
        if !src.exists() {
            return Err(format!("Missing {}", src.display()));
        }
        let output = Command::new("swiftc")
            .args(["-O", src.to_str().unwrap_or_default(), "-o", bin.to_str().unwrap_or_default()])
            .output()
            .map_err(|e| format!("swiftc sim-window: {e}"))?;
        if !output.status.success() || !bin.exists() {
            return Err(format!(
                "sim-window compile failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        stamp_sim_window_version();
        *WINDOW_BIN.lock().unwrap() = Some(bin.clone());
        Ok(bin)
    }

    fn bin_path() -> Result<PathBuf, String> {
        WINDOW_BIN
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| "sim-window not compiled — call warmup".into())
    }

    fn run_set_frame(x: f64, y: f64, w: f64, h: f64) {
        let Ok(bin) = bin_path() else { return };
        let _ = Command::new(&bin)
            .args([
                "set-frame",
                &format!("{x:.1}"),
                &format!("{y:.1}"),
                &format!("{w:.1}"),
                &format!("{h:.1}"),
            ])
            .output();
    }

    fn run_hide() {
        if let Ok(bin) = bin_path() {
            let _ = Command::new(&bin).arg("hide").output();
        }
    }

    pub fn warmup() {
        thread::spawn(|| {
            let _ = ensure_sim_window();
        });
    }

    pub fn set_screen_layout(x: f64, y: f64, w: f64, h: f64) {
        *LAYOUT.lock().unwrap() = (x, y, w, h);
    }

    pub fn apply_layout_from_webview(
        app: &AppHandle,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<(), String> {
        let win = app
            .get_webview_window("main")
            .ok_or_else(|| "main window not found".to_string())?;
        let inner = win.inner_position().map_err(|e| e.to_string())?;
        let scale = win.scale_factor().map_err(|e| e.to_string())?;
        let sx = inner.x as f64 + x * scale;
        let sy = inner.y as f64 + y * scale;
        let sw = width * scale;
        let sh = height * scale;
        set_screen_layout(sx, sy, sw, sh);
        if EMBED_ACTIVE.load(Ordering::SeqCst) && !EMBED_PAUSED.load(Ordering::SeqCst) {
            run_set_frame(sx, sy, sw, sh);
        }
        Ok(())
    }

    fn start_dock_thread() {
        if let Some(h) = DOCK_THREAD.lock().unwrap().take() {
            let _ = h.join();
        }
        EMBED_ACTIVE.store(true, Ordering::SeqCst);
        let handle = thread::Builder::new()
            .name("ios-sim-dock".into())
            .spawn(|| {
                while EMBED_ACTIVE.load(Ordering::SeqCst) {
                    if !EMBED_PAUSED.load(Ordering::SeqCst) {
                        let (x, y, w, h) = *LAYOUT.lock().unwrap();
                        if w > 20.0 && h > 20.0 {
                            run_set_frame(x, y, w, h);
                        }
                    }
                    thread::sleep(Duration::from_millis(16));
                }
            })
            .expect("dock thread");
        *DOCK_THREAD.lock().unwrap() = Some(handle);
    }

    pub fn open_simulator(udid: &str) -> Result<Value, String> {
        let bin = ensure_sim_window()?;
        let output = Command::new(&bin)
            .args(["open", udid])
            .output()
            .map_err(|e| format!("sim-window open: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.is_empty() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "sim-window open failed (exit {:?}): {}",
                output.status.code(),
                stderr.trim()
            ));
        }
        serde_json::from_str(&stdout).map_err(|e| format!("sim-window JSON: {e} — {stdout}"))
    }

    pub fn start_docking() {
        start_dock_thread();
    }

    pub fn pause() {
        EMBED_PAUSED.store(true, Ordering::SeqCst);
        run_hide();
    }

    pub fn resume() {
        EMBED_PAUSED.store(false, Ordering::SeqCst);
        let (x, y, w, h) = *LAYOUT.lock().unwrap();
        if w > 20.0 && h > 20.0 {
            run_set_frame(x, y, w, h);
        }
    }

    pub fn stop() {
        EMBED_ACTIVE.store(false, Ordering::SeqCst);
        EMBED_PAUSED.store(false, Ordering::SeqCst);
        run_hide();
        if let Some(h) = DOCK_THREAD.lock().unwrap().take() {
            let _ = h.join();
        }
        *LAYOUT.lock().unwrap() = (0.0, 0.0, 0.0, 0.0);
    }

    pub fn is_active() -> bool {
        EMBED_ACTIVE.load(Ordering::SeqCst)
    }

    pub fn is_paused() -> bool {
        EMBED_PAUSED.load(Ordering::SeqCst)
    }

    pub fn embed_mode_enabled() -> bool {
        std::env::var("IOS_SIM_EMBED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    pub fn stream_mode_enabled() -> bool {
        std::env::var("IOS_SIM_STREAM")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }
}

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(not(target_os = "macos"))]
pub fn warmup() {}

#[cfg(not(target_os = "macos"))]
pub fn stop() {}

#[cfg(not(target_os = "macos"))]
pub fn pause() {}

#[cfg(not(target_os = "macos"))]
pub fn resume() {}

#[cfg(not(target_os = "macos"))]
pub fn is_active() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn is_paused() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn stream_mode_enabled() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn apply_layout_from_webview(
    _app: &tauri::AppHandle,
    _x: f64,
    _y: f64,
    _width: f64,
    _height: f64,
) -> Result<(), String> {
    Err("iOS sim embed requires macOS".into())
}
