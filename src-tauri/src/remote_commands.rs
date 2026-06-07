//! Remote workspace stubs — SSH probe and directory listing for future full Remote-SSH parity.

use std::process::Stdio;
use crate::process_ext::hidden_command;

#[tauri::command]
pub async fn remote_ssh_probe(
    host: String,
    user: String,
    port: Option<u16>,
) -> Result<serde_json::Value, String> {
    let port = port.unwrap_or(22);
    let target = format!("{user}@{host}");
    let out = hidden_command("ssh")
        .args([
            "-p",
            &port.to_string(),
            "-o",
            "BatchMode=yes",
            "-o",
            "ConnectTimeout=8",
            "-o",
            "StrictHostKeyChecking=accept-new",
            &target,
            "echo ok",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("ssh not found or failed: {e}"))?;
    Ok(serde_json::json!({
        "ok": out.status.success(),
        "host": host,
        "user": user,
        "port": port,
        "stdout": String::from_utf8_lossy(&out.stdout).trim(),
        "stderr": String::from_utf8_lossy(&out.stderr).trim(),
    }))
}

#[tauri::command]
pub async fn remote_ssh_list_dir(
    host: String,
    user: String,
    remote_path: String,
    port: Option<u16>,
) -> Result<serde_json::Value, String> {
    let port = port.unwrap_or(22);
    let target = format!("{user}@{host}");
    let cmd = format!("ls -la {}", remote_path.replace('\'', "'\\''"));
    let out = hidden_command("ssh")
        .args([
            "-p",
            &port.to_string(),
            "-o",
            "BatchMode=yes",
            "-o",
            "ConnectTimeout=8",
            &target,
            &cmd,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok(serde_json::json!({
        "ok": true,
        "listing": String::from_utf8_lossy(&out.stdout),
    }))
}
