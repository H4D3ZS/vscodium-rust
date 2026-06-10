//! Remote SSH workspace — mount, sync, exec, and live file I/O.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Mutex;
use crate::process_ext::hidden_command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSession {
    pub host: String,
    pub user: String,
    pub port: u16,
    pub remote_path: String,
    pub local_mirror: String,
    pub mounted_at: i64,
}

static REMOTE_SESSION: Mutex<Option<RemoteSession>> = Mutex::new(None);

fn ssh_target(user: &str, host: &str) -> String {
    format!("{user}@{host}")
}

fn ssh_base_args(port: u16) -> Vec<String> {
    vec![
        "-p".into(),
        port.to_string(),
        "-o".into(),
        "BatchMode=yes".into(),
        "-o".into(),
        "ConnectTimeout=12".into(),
        "-o".into(),
        "StrictHostKeyChecking=accept-new".into(),
    ]
}

fn run_ssh(user: &str, host: &str, port: u16, remote_cmd: &str) -> Result<std::process::Output, String> {
    let mut cmd = hidden_command("ssh");
    for a in ssh_base_args(port) {
        cmd.arg(a);
    }
    cmd.arg(ssh_target(user, host)).arg(remote_cmd);
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd.output().map_err(|e| format!("ssh failed: {e}"))
}

fn mirror_dir(config_dir: &Path, user: &str, host: &str, remote_path: &str) -> PathBuf {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    user.hash(&mut h);
    host.hash(&mut h);
    remote_path.hash(&mut h);
    config_dir
        .join("remote_workspaces")
        .join(format!("{}_{}_{:x}", user, host.replace('.', "_"), h.finish()))
}

#[tauri::command]
pub async fn remote_ssh_probe(
    host: String,
    user: String,
    port: Option<u16>,
) -> Result<serde_json::Value, String> {
    let port = port.unwrap_or(22);
    let out = run_ssh(&user, &host, port, "echo ok")?;
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
    let cmd = format!("ls -la {}", shell_escape(&remote_path));
    let out = run_ssh(&user, &host, port, &cmd)?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok(serde_json::json!({
        "ok": true,
        "listing": String::from_utf8_lossy(&out.stdout),
    }))
}

#[tauri::command]
pub async fn remote_ssh_mount(
    state: tauri::State<'_, crate::EditorState>,
    host: String,
    user: String,
    remote_path: String,
    port: Option<u16>,
) -> Result<serde_json::Value, String> {
    let port = port.unwrap_or(22);
    let probe = run_ssh(&user, &host, port, "echo ok")?;
    if !probe.status.success() {
        return Err(format!(
            "SSH probe failed: {}",
            String::from_utf8_lossy(&probe.stderr).trim()
        ));
    }

    let local = mirror_dir(&state.config_dir, &user, &host, &remote_path);
    std::fs::create_dir_all(&local).map_err(|e| e.to_string())?;

    let resolved = if remote_path.starts_with('~') {
        let home_out = run_ssh(&user, &host, port, "echo $HOME")?;
        let home = String::from_utf8_lossy(&home_out.stdout).trim().to_string();
        remote_path.replacen('~', &home, 1)
    } else {
        remote_path.clone()
    };

    sync_pull(&user, &host, port, &resolved, &local)?;

    let session = RemoteSession {
        host: host.clone(),
        user: user.clone(),
        port,
        remote_path: resolved.clone(),
        local_mirror: local.to_string_lossy().into_owned(),
        mounted_at: chrono::Utc::now().timestamp(),
    };
    if let Ok(mut g) = REMOTE_SESSION.lock() {
        *g = Some(session.clone());
    }

    Ok(serde_json::json!({
        "ok": true,
        "local_path": session.local_mirror,
        "remote_path": session.remote_path,
        "message": "Remote workspace mirrored locally — open this folder as workspace root",
    }))
}

#[tauri::command]
pub async fn remote_ssh_sync_pull() -> Result<serde_json::Value, String> {
    let session = REMOTE_SESSION
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "No remote session mounted".to_string())?;
    sync_pull(
        &session.user,
        &session.host,
        session.port,
        &session.remote_path,
        Path::new(&session.local_mirror),
    )?;
    Ok(serde_json::json!({ "ok": true, "local_path": session.local_mirror }))
}

#[tauri::command]
pub async fn remote_ssh_sync_push() -> Result<serde_json::Value, String> {
    let session = REMOTE_SESSION
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "No remote session mounted".to_string())?;
    sync_push(
        &session.user,
        &session.host,
        session.port,
        &session.remote_path,
        Path::new(&session.local_mirror),
    )?;
    Ok(serde_json::json!({ "ok": true, "remote_path": session.remote_path }))
}

#[tauri::command]
pub async fn remote_ssh_disconnect() -> Result<(), String> {
    if let Ok(mut g) = REMOTE_SESSION.lock() {
        *g = None;
    }
    Ok(())
}

#[tauri::command]
pub async fn remote_ssh_status() -> Result<serde_json::Value, String> {
    let session = REMOTE_SESSION.lock().map_err(|e| e.to_string())?;
    match session.as_ref() {
        None => Ok(serde_json::json!({ "mounted": false })),
        Some(s) => Ok(serde_json::json!({
            "mounted": true,
            "host": s.host,
            "user": s.user,
            "port": s.port,
            "remote_path": s.remote_path,
            "local_mirror": s.local_mirror,
            "mounted_at": s.mounted_at,
        })),
    }
}

#[tauri::command]
pub async fn remote_ssh_exec(
    command: String,
    cwd: Option<String>,
) -> Result<serde_json::Value, String> {
    let session = REMOTE_SESSION
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "No remote session — mount first".to_string())?;
    let remote_cmd = if let Some(dir) = cwd {
        format!("cd {} && {}", shell_escape(&dir), command)
    } else {
        format!("cd {} && {}", shell_escape(&session.remote_path), command)
    };
    let out = run_ssh(&session.user, &session.host, session.port, &remote_cmd)?;
    Ok(serde_json::json!({
        "ok": out.status.success(),
        "stdout": String::from_utf8_lossy(&out.stdout),
        "stderr": String::from_utf8_lossy(&out.stderr),
        "exit_code": out.status.code(),
    }))
}

#[tauri::command]
pub async fn remote_ssh_read_file(remote_file: String) -> Result<String, String> {
    let session = REMOTE_SESSION
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "No remote session".to_string())?;
    let path = if remote_file.starts_with('/') {
        remote_file
    } else {
        format!("{}/{}", session.remote_path.trim_end_matches('/'), remote_file)
    };
    let out = run_ssh(
        &session.user,
        &session.host,
        session.port,
        &format!("cat {}", shell_escape(&path)),
    )?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[tauri::command]
pub async fn remote_ssh_write_file(remote_file: String, content: String) -> Result<(), String> {
    let session = REMOTE_SESSION
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "No remote session".to_string())?;
    let path = if remote_file.starts_with('/') {
        remote_file
    } else {
        format!("{}/{}", session.remote_path.trim_end_matches('/'), remote_file)
    };
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, content.as_bytes());
    let cmd = format!(
        "mkdir -p $(dirname {}) && echo {} | base64 -d > {}",
        shell_escape(&path),
        shell_escape(&b64),
        shell_escape(&path)
    );
    let out = run_ssh(&session.user, &session.host, session.port, &cmd)?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok(())
}

pub fn active_session() -> Option<RemoteSession> {
    REMOTE_SESSION.lock().ok().and_then(|g| g.clone())
}

fn sync_pull(user: &str, host: &str, port: u16, remote: &str, local: &Path) -> Result<(), String> {
    std::fs::create_dir_all(local).map_err(|e| e.to_string())?;
    if which::which("rsync").is_ok() {
        let mut cmd = hidden_command("rsync");
        cmd.args([
            "-az",
            "--delete",
            "-e",
            &format!("ssh -p {port} -o BatchMode=yes -o StrictHostKeyChecking=accept-new"),
            &format!("{}:{}/", ssh_target(user, host), remote.trim_end_matches('/')),
            &format!("{}/", local.display()),
        ]);
        let out = cmd.output().map_err(|e| e.to_string())?;
        if out.status.success() {
            return Ok(());
        }
    }
    let mut cmd = hidden_command("scp");
    cmd.args(["-r", "-P", &port.to_string()]);
    for a in ["-o", "BatchMode=yes", "-o", "StrictHostKeyChecking=accept-new"] {
        cmd.arg(a);
    }
    cmd.arg(format!("{}:{}/.", ssh_target(user, host), remote.trim_end_matches('/')));
    cmd.arg(format!("{}/", local.display()));
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok(())
}

fn sync_push(user: &str, host: &str, port: u16, remote: &str, local: &Path) -> Result<(), String> {
    if which::which("rsync").is_ok() {
        let mut cmd = hidden_command("rsync");
        cmd.args([
            "-az",
            "-e",
            &format!("ssh -p {port} -o BatchMode=yes -o StrictHostKeyChecking=accept-new"),
            &format!("{}/", local.display()),
            &format!("{}:{}/", ssh_target(user, host), remote.trim_end_matches('/')),
        ]);
        let out = cmd.output().map_err(|e| e.to_string())?;
        if out.status.success() {
            return Ok(());
        }
    }
    let mut cmd = hidden_command("scp");
    cmd.args(["-r", "-P", &port.to_string()]);
    for a in ["-o", "BatchMode=yes", "-o", "StrictHostKeyChecking=accept-new"] {
        cmd.arg(a);
    }
    cmd.arg(format!("{}/.", local.display()));
    cmd.arg(format!("{}:{}/", ssh_target(user, host), remote.trim_end_matches('/')));
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).to_string());
    }
    Ok(())
}

fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
