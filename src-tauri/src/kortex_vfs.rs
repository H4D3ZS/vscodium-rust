//! aim-vfs daemon lifecycle — manages the virtual filesystem daemon as a
//! resolved+managed sidecar. When running, the VFS bridge can delegate file
//! operations to the daemon instead of doing raw in-process I/O.

use std::sync::Mutex;
use std::process::Child;

use serde::Serialize;

use crate::kortex_bin::resolve_aim_vfs;

/// Running daemon state.
struct VfsDaemon {
    child: Child,
    port: u16,
}

static DAEMON: Mutex<Option<VfsDaemon>> = Mutex::new(None);

#[derive(Serialize)]
pub struct VfsDaemonStatus {
    pub running: bool,
    pub port: Option<u16>,
}

const VFS_PORT: u16 = 1537;

/// Start the aim-vfs daemon as a sidecar process. Idempotent — returns
/// the port if already running.
#[tauri::command]
pub async fn kortex_vfs_start() -> Result<u16, String> {
    {
        let guard = DAEMON.lock().unwrap();
        if let Some(d) = guard.as_ref() {
            return Ok(d.port); // Already running — idempotent
        }
    }

    let bin = resolve_aim_vfs()
        .ok_or("aim-vfs binary not found. Build it: cd kortex && cargo build --release -p vfs_layer")?;

    let child = std::process::Command::new(&bin)
        .arg("--port")
        .arg(VFS_PORT.to_string())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn aim-vfs: {e}"))?;

    *DAEMON.lock().unwrap() = Some(VfsDaemon { child, port: VFS_PORT });
    println!("[kortex-vfs] aim-vfs started on :{VFS_PORT}");
    Ok(VFS_PORT)
}

/// Stop the aim-vfs daemon.
#[tauri::command]
pub async fn kortex_vfs_stop() -> Result<(), String> {
    let mut guard = DAEMON.lock().unwrap();
    let mut daemon = guard.take()
        .ok_or("aim-vfs is not running")?;
    let _ = daemon.child.kill();
    println!("[kortex-vfs] aim-vfs stopped");
    Ok(())
}

/// Check daemon status.
#[tauri::command]
pub async fn kortex_vfs_status() -> Result<VfsDaemonStatus, String> {
    let mut guard = DAEMON.lock().unwrap();
    let running = match guard.as_mut() {
        Some(d) => {
            // Check if the process is still alive.
            match d.child.try_wait() {
                Ok(Some(_)) => false, // Process exited
                Ok(None) => true,     // Still running
                Err(_) => false,
            }
        }
        None => false,
    };
    if !running {
        *guard = None; // Clean up dead process
    }
    Ok(VfsDaemonStatus {
        running,
        port: guard.as_ref().map(|d| d.port),
    })
}
