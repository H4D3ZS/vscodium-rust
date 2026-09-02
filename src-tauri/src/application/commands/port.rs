use serde_json::{json, Value};
use std::process::Command;
use std::sync::{LazyLock, Mutex};

#[derive(Clone, serde::Serialize)]
struct ForwardedPort {
    local_port: u16,
    label: String,
    protocol: String,
}

static FORWARDED: LazyLock<Mutex<Vec<ForwardedPort>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Parse listening TCP ports from `netstat` (Windows-first; best-effort elsewhere).
#[tauri::command]
pub async fn list_listening_ports() -> Result<Value, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("netstat")
            .args(["-ano", "-p", "tcp"])
            .output()
            .map_err(|e| e.to_string())?
    } else {
        Command::new("ss")
            .args(["-ltn"])
            .output()
            .or_else(|_| Command::new("netstat").args(["-ltn"]).output())
            .map_err(|e| e.to_string())?
    };

    let text = String::from_utf8_lossy(&output.stdout);
    let mut ports: Vec<u16> = Vec::new();
    for line in text.lines() {
        let lower = line.to_lowercase();
        if !lower.contains("listen") {
            continue;
        }
        for token in line.split_whitespace() {
            if let Some((_, port)) = token.rsplit_once(':') {
                if let Ok(p) = port.parse::<u16>() {
                    if p > 0 && !ports.contains(&p) {
                        ports.push(p);
                    }
                }
            }
        }
    }
    ports.sort_unstable();
    ports.truncate(50);

    let forwarded = FORWARDED.lock().map_err(|e| e.to_string())?.clone();
    Ok(json!({ "listening": ports, "forwarded": forwarded }))
}

#[tauri::command]
pub async fn port_forward_add(local_port: u16, label: Option<String>) -> Result<Value, String> {
    let mut guard = FORWARDED.lock().map_err(|e| e.to_string())?;
    if !guard.iter().any(|p| p.local_port == local_port) {
        guard.push(ForwardedPort {
            local_port,
            label: label.unwrap_or_else(|| format!("localhost:{local_port}")),
            protocol: "http".into(),
        });
    }
    Ok(json!({ "ok": true, "forwarded": guard.clone() }))
}

#[tauri::command]
pub async fn port_forward_remove(local_port: u16) -> Result<(), String> {
    let mut guard = FORWARDED.lock().map_err(|e| e.to_string())?;
    guard.retain(|p| p.local_port != local_port);
    Ok(())
}
