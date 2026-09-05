//! Free Claude Code — Tauri commands for managing the FCC sidecar proxy.

use crate::infrastructure::fcc_sidecar;

/// Start the FCC sidecar proxy server.
#[tauri::command]
pub async fn fcc_start() -> Result<String, String> {
    let mut guard = fcc_sidecar::FCC_SIDECAR.lock().await;

    // Initialize if not yet done
    if guard.is_none() {
        let fcc_dir = resolve_fcc_dir();
        let port = resolve_fcc_port();
        fcc_sidecar::init_fcc(fcc_dir, port).await;
        drop(guard);
        guard = fcc_sidecar::FCC_SIDECAR.lock().await;
    }

    guard.as_mut()
        .ok_or("FCC sidecar not available")?
        .start()
        .await?;

    Ok("FCC started".to_string())
}

/// Stop the FCC sidecar proxy server.
#[tauri::command]
pub async fn fcc_stop() -> Result<String, String> {
    let mut guard = fcc_sidecar::FCC_SIDECAR.lock().await;
    if let Some(sidecar) = guard.as_mut() {
        sidecar.stop().await;
    }
    Ok("FCC stopped".to_string())
}

/// Get FCC sidecar status.
#[tauri::command]
pub async fn fcc_status() -> Result<fcc_sidecar::FccStatus, String> {
    let guard = fcc_sidecar::FCC_SIDECAR.lock().await;
    match guard.as_ref() {
        Some(sidecar) => Ok(sidecar.status().await),
        None => Ok(fcc_sidecar::FccStatus::Stopped),
    }
}

/// Probe FCC health endpoint.
#[tauri::command]
pub async fn fcc_health() -> Result<bool, String> {
    let guard = fcc_sidecar::FCC_SIDECAR.lock().await;
    match guard.as_ref() {
        Some(sidecar) => Ok(sidecar.health_check().await),
        None => Ok(false),
    }
}

/// Get the FCC proxy base URL.
#[tauri::command]
pub async fn fcc_get_url() -> Result<String, String> {
    let guard = fcc_sidecar::FCC_SIDECAR.lock().await;
    match guard.as_ref() {
        Some(sidecar) => Ok(sidecar.base_url()),
        None => Ok(format!("http://127.0.0.1:{}", resolve_fcc_port())),
    }
}

/// Open the FCC Admin UI in the default browser.
#[tauri::command]
pub async fn fcc_open_admin() -> Result<(), String> {
    let port = resolve_fcc_port();
    let url = format!("http://127.0.0.1:{}/admin", port);

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &url])
            .spawn()
            .map_err(|e| format!("Failed to open admin UI: {e}"))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open admin UI: {e}"))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open admin UI: {e}"))?;
    }

    Ok(())
}

/// Check if Python/uv environment is ready for FCC.
#[tauri::command]
pub async fn fcc_check_env() -> Result<fcc_sidecar::PythonEnvCheck, String> {
    let fcc_dir = resolve_fcc_dir();
    Ok(fcc_sidecar::check_python_env(&fcc_dir))
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Resolve the FCC directory path. Checks multiple locations.
fn resolve_fcc_dir() -> std::path::PathBuf {
    // 1. Check alongside the executable (production)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let fcc = dir.join("third_party").join("free-claude-code");
            if fcc.join("server.py").exists() {
                return fcc;
            }
        }
    }

    // 2. Check the workspace root (development)
    if let Ok(cwd) = std::env::current_dir() {
        let fcc = cwd.join("third_party").join("free-claude-code");
        if fcc.join("server.py").exists() {
            return fcc;
        }
    }

    // 3. Fallback
    std::path::PathBuf::from("third_party/free-claude-code")
}

/// Resolve the FCC port from environment or default.
fn resolve_fcc_port() -> u16 {
    std::env::var("FCC_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8082)
}
