use crate::domain::mobile::iphone_device::resolve_go_ios;
use tauri::command;
use tokio::process::Command;

/// Starts a port forward to the jailbroken device's SSH daemon using go-ios.
/// Defaults to forwarding local port 2222 to device port 22.
#[command]
pub async fn bounty_start_ssh_tunnel(
    udid: String,
    local_port: u16,
    device_port: u16,
) -> Result<String, String> {
    let go_ios = resolve_go_ios().ok_or_else(|| "go-ios not found".to_string())?;

    let mut cmd = Command::new(go_ios);
    cmd.args(&[
        "forward",
        &local_port.to_string(),
        &device_port.to_string(),
        "--udid",
        &udid,
    ]);

    // We spawn it detached so it stays running
    cmd.spawn()
        .map_err(|e| format!("Failed to start go-ios forward: {}", e))?;

    Ok(format!(
        "Tunnel started on local port {} to device port {}",
        local_port, device_port
    ))
}

/// Executes a shell command on the jailbroken device over the SSH tunnel.
#[command]
pub async fn bounty_execute_over_ssh(local_port: u16, command: String) -> Result<String, String> {
    let mut cmd = Command::new("ssh");
    cmd.args(&[
        "-p",
        &local_port.to_string(),
        "-o",
        "StrictHostKeyChecking=no",
        "-o",
        "UserKnownHostsFile=/dev/null",
        "root@127.0.0.1",
        &command,
    ]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Injects a Frida payload onto the device via USB.
#[command]
pub async fn bounty_inject_frida_payload(
    process: String,
    script_path: String,
) -> Result<String, String> {
    let mut cmd = Command::new("frida");
    cmd.args(&["-U", "-n", &process, "-l", &script_path, "--no-pause"]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
