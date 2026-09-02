//! Native window controls for the custom (decorations:false) title bar.
//!
//! The frontend's window buttons call these directly instead of the
//! `@tauri-apps/api` window plugin. Going through the AppHandle and operating
//! on the resolved "main" window is the most reliable path: it cannot be
//! no-op'd by a JS/Rust version drift in the plugin IPC payload, and it does
//! not depend on `core:window:*` capability grants. Each call resolves the
//! window fresh and reports precise errors.

use tauri::{AppHandle, Manager};

fn main_window(app: &AppHandle) -> Result<tauri::WebviewWindow, String> {
    app.get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())
}

#[tauri::command]
pub fn win_minimize(app: AppHandle) -> Result<(), String> {
    let w = main_window(&app)?;
    w.minimize().map_err(|e| format!("minimize: {e}"))
}

#[tauri::command]
pub fn win_toggle_maximize(app: AppHandle) -> Result<bool, String> {
    let w = main_window(&app)?;
    // If the window is stuck in native fullscreen (e.g. macOS green-button
    // fullscreen, which makes a decorations:false window uncontrollable),
    // the maximize button should exit fullscreen first so the user can
    // always escape it.
    if w.is_fullscreen().unwrap_or(false) {
        w.set_fullscreen(false).map_err(|e| format!("exit fullscreen: {e}"))?;
        return Ok(false);
    }
    let is_max = w.is_maximized().map_err(|e| format!("is_maximized: {e}"))?;
    if is_max {
        w.unmaximize().map_err(|e| format!("unmaximize: {e}"))?;
    } else {
        w.maximize().map_err(|e| format!("maximize: {e}"))?;
    }
    Ok(!is_max)
}

#[tauri::command]
pub fn win_close(app: AppHandle) -> Result<(), String> {
    let w = main_window(&app)?;
    w.close().map_err(|e| format!("close: {e}"))
}

#[tauri::command]
pub fn win_start_drag(app: AppHandle) -> Result<(), String> {
    let w = main_window(&app)?;
    w.start_dragging().map_err(|e| format!("start_dragging: {e}"))
}

/// Diagnostics: returns current window state so the frontend can confirm the
/// command path reached Rust and which window it targeted.
#[tauri::command]
pub fn win_state(app: AppHandle) -> Result<serde_json::Value, String> {
    let w = main_window(&app)?;
    Ok(serde_json::json!({
        "label": w.label(),
        "is_minimized": w.is_minimized().unwrap_or(false),
        "is_maximized": w.is_maximized().unwrap_or(false),
        "is_fullscreen": w.is_fullscreen().unwrap_or(false),
        "is_visible": w.is_visible().unwrap_or(true),
        "is_focused": w.is_focused().unwrap_or(false),
    }))
}
