//! In-window native iOS simulator display (macOS).
//! Headless CoreSimulator IOSurface → Swift NSView overlay above WKWebView.

#[cfg(target_os = "macos")]
mod mac {
    use std::ffi::{c_char, c_void, CString};
    use std::sync::Mutex;
    use tauri::{AppHandle, Manager};

    extern "C" {
        fn sim_host_attach(webview: *mut c_void) -> *mut c_void;
        fn sim_host_start(host: *mut c_void, udid: *const c_char) -> i32;
        fn sim_host_set_frame(host: *mut c_void, x: f64, y: f64, w: f64, h: f64);
        fn sim_host_set_visible(host: *mut c_void, visible: i32);
        fn sim_host_stop(host: *mut c_void);
        fn sim_host_detach(host: *mut c_void);
        fn sim_host_set_touch_callback(cb: Option<extern "C" fn(f64, f64, *const c_char)>);
        fn sim_host_set_size_callback(cb: Option<extern "C" fn(u32, u32)>);
    }

    extern "C" fn touch_forward_cb(x: f64, y: f64, phase: *const c_char) {
        if phase.is_null() {
            return;
        }
        let phase = unsafe { std::ffi::CStr::from_ptr(phase) };
        let phase = phase.to_string_lossy();
        let _ = crate::ios_simulator::touch_forward(x, y, &phase);
    }

    extern "C" fn size_forward_cb(w: u32, h: u32) {
        crate::ios_simulator::native_surface_size(w, h);
    }

    fn register_callbacks() {
        unsafe {
            sim_host_set_touch_callback(Some(touch_forward_cb));
            sim_host_set_size_callback(Some(size_forward_cb));
        }
    }

    static HOST: Mutex<Option<usize>> = Mutex::new(None);

    fn host_ptr() -> Option<*mut c_void> {
        HOST.lock().unwrap().map(|p| p as *mut c_void)
    }

    fn set_host_ptr(p: *mut c_void) {
        *HOST.lock().unwrap() = if p.is_null() { None } else { Some(p as usize) };
    }

    fn webview_ptr(app: &AppHandle) -> Result<*mut c_void, String> {
        let win = app
            .get_webview_window("main")
            .ok_or_else(|| "main window not found".to_string())?;
        let ns_view = win.ns_view().map_err(|e| e.to_string())?;
        if ns_view.is_null() {
            return Err("ns_view null".into());
        }
        Ok(ns_view)
    }

    fn run_on_main_thread<R, F>(app: &AppHandle, f: F) -> Result<R, String>
    where
        R: Send + 'static,
        F: FnOnce() -> Result<R, String> + Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        app.run_on_main_thread(move || {
            let _ = tx.send(f());
        })
        .map_err(|e| e.to_string())?;
        rx.recv().map_err(|_| "main thread cancelled".to_string())?
    }

    pub fn stream_mode_enabled() -> bool {
        std::env::var("IOS_SIM_STREAM")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    pub fn embed_mode_enabled() -> bool {
        std::env::var("IOS_SIM_EMBED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    pub fn attach(app: &AppHandle) -> Result<(), String> {
        register_callbacks();
        if HOST.lock().unwrap().is_some() {
            return Ok(());
        }
        let webview = webview_ptr(app)? as usize;
        let host = run_on_main_thread(app, move || {
            let host = unsafe { sim_host_attach(webview as *mut c_void) };
            if host.is_null() {
                Err("sim_host_attach failed".into())
            } else {
                Ok(host as usize)
            }
        })?;
        set_host_ptr(host as *mut c_void);
        Ok(())
    }

    pub fn start(app: &AppHandle, udid: &str) -> Result<(), String> {
        attach(app)?;
        let host = host_ptr().ok_or_else(|| "sim host not attached".to_string())? as usize;
        let udid = udid.to_string();
        let rc = run_on_main_thread(app, move || {
            let c_udid = CString::new(udid).map_err(|e| e.to_string())?;
            Ok(unsafe { sim_host_start(host as *mut c_void, c_udid.as_ptr()) })
        })?;
        if rc != 0 {
            return Err("sim_host_start failed — is simulator booted?".into());
        }
        Ok(())
    }

    pub fn apply_layout(app: &AppHandle, x: f64, y: f64, width: f64, height: f64) -> Result<(), String> {
        let Some(host) = host_ptr() else {
            return Ok(());
        };
        if width < 8.0 || height < 8.0 {
            return Ok(());
        }
        let host = host as usize;
        app.run_on_main_thread(move || {
            unsafe { sim_host_set_frame(host as *mut c_void, x, y, width, height) };
        })
        .map_err(|e| e.to_string())
    }

    pub fn pause() {
        if let Some(host) = host_ptr() {
            unsafe { sim_host_set_visible(host as *mut c_void, 0) };
        }
    }

    pub fn resume() {
        if let Some(host) = host_ptr() {
            unsafe { sim_host_set_visible(host as *mut c_void, 1) };
        }
    }

    pub fn stop() {
        if let Some(host_usize) = HOST.lock().unwrap().take() {
            let host = host_usize as *mut c_void;
            unsafe {
                sim_host_stop(host);
                sim_host_detach(host);
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(not(target_os = "macos"))]
pub fn stream_mode_enabled() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn embed_mode_enabled() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
pub fn attach(_app: &tauri::AppHandle) -> Result<(), String> {
    Err("native sim requires macOS".into())
}

#[cfg(not(target_os = "macos"))]
pub fn start(_app: &tauri::AppHandle, _udid: &str) -> Result<(), String> {
    Err("native sim requires macOS".into())
}

#[cfg(not(target_os = "macos"))]
pub fn apply_layout(
    _app: &tauri::AppHandle,
    _x: f64,
    _y: f64,
    _width: f64,
    _height: f64,
) -> Result<(), String> {
    Err("native sim requires macOS".into())
}

#[cfg(not(target_os = "macos"))]
pub fn pause() {}

#[cfg(not(target_os = "macos"))]
pub fn resume() {}

#[cfg(not(target_os = "macos"))]
pub fn stop() {}
