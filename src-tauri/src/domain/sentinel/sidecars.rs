//! Sentinel sidecar manager: spawns/stoops/probes the FlutterSentinel service
//! stack (fbhbot, backend, mobsf, ai-hunter) as hidden processes, with all
//! paths resolved through `SentinelSettings` (Windows port of the macOS
//! hardcoded `/Users/hades/...` launches).

use crate::domain::sentinel::settings::SentinelSettings;
use crate::infrastructure::process_ext::CommandExtHidden;
use serde::Serialize;
use std::collections::HashMap;
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;

/// A managed sidecar service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum SidecarKind {
    Fbhbot,
    Backend,
    Mobsf,
    AiHunter,
    McpServer,
}

impl SidecarKind {
    pub fn label(&self) -> &'static str {
        match self {
            SidecarKind::Fbhbot => "fbhbot",
            SidecarKind::Backend => "backend",
            SidecarKind::Mobsf => "mobsf",
            SidecarKind::AiHunter => "ai-hunter",
            SidecarKind::McpServer => "mcp-server",
        }
    }

    pub fn from_label(s: &str) -> Option<Self> {
        match s {
            "fbhbot" => Some(SidecarKind::Fbhbot),
            "backend" => Some(SidecarKind::Backend),
            "mobsf" => Some(SidecarKind::Mobsf),
            "ai-hunter" | "aiHunter" => Some(SidecarKind::AiHunter),
            "mcp-server" | "mcp_server" => Some(SidecarKind::McpServer),
            _ => None,
        }
    }

    /// Primary HTTP port the service listens on (None = stdio service).
    pub fn port(&self) -> Option<u16> {
        match self {
            SidecarKind::Fbhbot => Some(3001),
            SidecarKind::Backend => Some(4000),
            SidecarKind::Mobsf => Some(8000),
            SidecarKind::AiHunter => Some(3000),
            SidecarKind::McpServer => None,
        }
    }

    /// Log file name (relative to work_dir/logs).
    pub fn log_file(&self) -> &'static str {
        match self {
            SidecarKind::Fbhbot => "fbhbot.log",
            SidecarKind::Backend => "backend.log",
            SidecarKind::Mobsf => "mobsf.log",
            SidecarKind::AiHunter => "ai-hunter.log",
            SidecarKind::McpServer => "mcp-server.log",
        }
    }
}

/// Resolve a settings field into the per-kind service directory.
fn service_dir(settings: &SentinelSettings, kind: SidecarKind) -> PathBuf {
    let rel = match kind {
        SidecarKind::Fbhbot => &settings.fbhbot_dir,
        SidecarKind::Backend => &settings.backend_dir,
        SidecarKind::Mobsf => &settings.mobsf_dir,
        SidecarKind::AiHunter => &settings.ai_hunter_dir,
        SidecarKind::McpServer => &settings.mcp_dir,
    };
    settings.service_dir(rel)
}

/// A live child process the engine is tracking.
#[derive(Debug, Serialize)]
pub struct SidecarHandle {
    pub kind: SidecarKind,
    pub pid: Option<u32>,
    pub started_at: String,
    pub log_path: String,
}

/// Build the spawn `std::process::Command` for a sidecar (hidden, stdio
/// redirected to the service log file so no console window ever flashes).
pub fn build_command(
    settings: &SentinelSettings,
    kind: SidecarKind,
    logs_dir: PathBuf,
) -> Result<(Command, PathBuf), String> {
    let dir = service_dir(settings, kind);
    let log_path = logs_dir.join(kind.log_file());
    if let Some(parent) = log_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let log = std::fs::File::create(&log_path)
        .map_err(|e| format!("open {}: {e}", log_path.display()))?;

    let node = settings
        .node_bin
        .clone()
        .unwrap_or_else(|| "node".to_string());
    let python = settings
        .python_bin
        .clone()
        .unwrap_or_else(|| "python".to_string());

    let mut cmd = match kind {
        SidecarKind::Fbhbot => {
            let mut c = Command::new(&node);
            c.arg("dist/index.js").current_dir(dir.clone());
            c.env("PORT", "3001");
            let state_dir = dir.join("state");
            let _ = std::fs::create_dir_all(&state_dir);
            c.env("FBHBOT_STATE_DIR", state_dir.to_string_lossy().to_string());
            c
        }
        SidecarKind::Backend => {
            let mut c = Command::new(&node);
            c.arg("dist/server.js").current_dir(dir.clone());
            c.env("PORT", "4000");
            c
        }
        SidecarKind::Mobsf => {
            if dir.join("manage.py").is_file() {
                let mut c = Command::new(&python);
                c.arg("manage.py")
                    .arg("runserver")
                    .arg("0.0.0.0:8000")
                    .current_dir(dir.clone());
                c
            } else if dir.join("run.bat").is_file() {
                let mut c = Command::new("cmd");
                c.arg("/c").arg("run.bat").current_dir(dir.clone());
                c
            } else {
                return Err("mobsf dir has neither manage.py nor run.bat".to_string());
            }
        }
        SidecarKind::AiHunter => {
            let mut c = Command::new(&node);
            c.arg("server/src/index.js").current_dir(dir.clone());
            c.env("PORT", "3000");
            c
        }
        SidecarKind::McpServer => {
            let mut c = Command::new(&node);
            c.arg("index.js").current_dir(dir.clone());
            c
        }
    };

    cmd.hidden_sidecar();
    cmd.stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?));
    cmd.stderr(Stdio::from(log));
    Ok((cmd, log_path))
}

/// Spawn a sidecar and return its log path.
pub fn spawn(
    settings: &SentinelSettings,
    kind: SidecarKind,
    logs_dir: PathBuf,
) -> Result<(Child, String), String> {
    let (cmd, log_path) = build_command(settings, kind, logs_dir)?;
    let mut cmd = cmd;
    let child = cmd
        .spawn()
        .map_err(|e| format!("spawn {}: {e}", kind.label()))?;
    let pid = child.id();
    crate::infrastructure::process_ext::suppress_child_console_after_spawn(pid);
    Ok((child, log_path.to_string_lossy().to_string()))
}

/// Probe whether a TCP port is listening (sidecar healthy check).
pub fn port_open(host: &str, port: u16, timeout_ms: u64) -> bool {
    use std::net::ToSocketAddrs;
    let addr = match (host, port).to_socket_addrs() {
        Ok(mut it) => match it.next() {
            Some(a) => a,
            None => return false,
        },
        Err(_) => return false,
    };
    TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)).is_ok()
}

/// A registry of alive children, keyed by sidecar label.
#[derive(Default)]
pub struct SidecarTable {
    inner: Mutex<HashMap<String, Child>>,
}

impl SidecarTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn live(&self, label: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        match guard.get_mut(label) {
            Some(child) => match child.try_wait() {
                Ok(Some(_)) => {
                    guard.remove(label);
                    false
                }
                Ok(None) => true,
                Err(_) => {
                    guard.remove(label);
                    false
                }
            },
            None => false,
        }
    }

    pub fn register(&self, label: &str, child: Child) {
        self.inner.lock().unwrap().insert(label.to_string(), child);
    }

    /// Kill a tracked child. Returns false if it was already gone.
    pub fn kill(&self, label: &str) -> bool {
        let mut guard = self.inner.lock().unwrap();
        if let Some(mut child) = guard.remove(label) {
            let _ = child.kill();
            let _ = child.wait();
            true
        } else {
            false
        }
    }

    pub fn kill_all(&self) {
        let mut guard = self.inner.lock().unwrap();
        for (_, mut child) in guard.drain() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    pub fn labels(&self) -> Vec<String> {
        self.inner.lock().unwrap().keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::sentinel::settings::SentinelSettings;

    #[test]
    fn ports_are_stable() {
        assert_eq!(SidecarKind::Fbhbot.port(), Some(3001));
        assert_eq!(SidecarKind::Backend.port(), Some(4000));
        assert_eq!(SidecarKind::Mobsf.port(), Some(8000));
        assert_eq!(SidecarKind::AiHunter.port(), Some(3000));
    }

    #[test]
    fn service_dir_resolves_via_root() {
        let mut s = SentinelSettings::default();
        s.sentinel_root = Some("C:\\FS".into());
        let dir = service_dir(&s, SidecarKind::Fbhbot);
        assert!(
            dir.to_string_lossy().ends_with("core\\fbhbot")
                || dir.to_string_lossy().ends_with("core/fbhbot")
        );
    }

    #[test]
    fn port_probe_closed_port() {
        assert!(!port_open("127.0.0.1", 1, 50));
    }
}
