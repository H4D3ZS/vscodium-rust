//! Free Claude Code sidecar — spawns and monitors the FCC Python proxy server.
//!
//! FCC routes Anthropic Messages API traffic from Claude Code / Codex through
//! 19+ providers (Lemonade, OpenRouter, etc.). The sidecar manages
//! the process lifecycle and provides health-check endpoints.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Status of the FCC sidecar process.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FccStatus {
    Stopped,
    Starting,
    Running { uptime_secs: u64 },
    Error(String),
}

/// Running FCC sidecar process handle.
pub struct FccSidecar {
    child: Option<Child>,
    status: Arc<RwLock<FccStatus>>,
    port: u16,
    fcc_dir: PathBuf,
    started_at: Option<Instant>,
}

impl FccSidecar {
    /// Create a new sidecar manager. Does not start the process.
    pub fn new(fcc_dir: PathBuf, port: u16) -> Self {
        Self {
            child: None,
            status: Arc::new(RwLock::new(FccStatus::Stopped)),
            port,
            fcc_dir,
            started_at: None,
        }
    }

    /// Start the FCC proxy server. Returns error if already running.
    pub async fn start(&mut self) -> Result<(), String> {
        {
            let status = self.status.read().await;
            if matches!(*status, FccStatus::Running { .. } | FccStatus::Starting) {
                return Err("FCC sidecar is already running".to_string());
            }
        }

        // Find the Python executable — prefer `uv` if available, fall back to `python`
        let (python_cmd, pre_args) = find_python_command(&self.fcc_dir)?;

        *self.status.write().await = FccStatus::Starting;

        let server_py = self.fcc_dir.join("server.py");
        if !server_py.exists() {
            let msg = format!("FCC server.py not found at {}", server_py.display());
            *self.status.write().await = FccStatus::Error(msg.clone());
            return Err(msg);
        }

        tracing::info!(
            "[fcc-sidecar] starting: {} server.py on port {}",
            python_cmd,
            self.port
        );

        let mut cmd = Command::new(&python_cmd);
        for a in &pre_args {
            cmd.arg(a);
        }
        cmd.arg("server.py");
        cmd.current_dir(&self.fcc_dir);
        cmd.env("PORT", self.port.to_string());
        cmd.env("HOST", "127.0.0.1");
        // Suppress FCC's auto-browser-open since we manage that ourselves
        cmd.env("FCC_OPEN_BROWSER", "false");
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // On Windows, hide the spawned process from the user's taskbar
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        let child = cmd
            .spawn()
            .map_err(|e| {
                let msg = format!("Failed to spawn FCC: {e}");
                let status = self.status.clone();
                let err_msg = msg.clone();
                tokio::spawn(async move {
                    *status.write().await = FccStatus::Error(err_msg);
                });
                msg
            })?;

        self.child = Some(child);
        self.started_at = Some(Instant::now());

        // Spawn stdout/stderr drain threads so the process doesn't block
        if let Some(ref mut child) = self.child {
            if let Some(stdout) = child.stdout.take() {
                std::thread::spawn(move || {
                    use std::io::BufRead;
                    let reader = std::io::BufReader::new(stdout);
                    for line in reader.lines().map_while(Result::ok) {
                        tracing::debug!("[fcc] {line}");
                    }
                });
            }
            if let Some(stderr) = child.stderr.take() {
                std::thread::spawn(move || {
                    use std::io::BufRead;
                    let reader = std::io::BufReader::new(stderr);
                    for line in reader.lines().map_while(Result::ok) {
                        tracing::warn!("[fcc] {line}");
                    }
                });
            }
        }

        // Wait for health check (max 15 seconds)
        let healthy = wait_for_health("127.0.0.1", self.port, Duration::from_secs(15)).await;

        if healthy {
            *self.status.write().await = FccStatus::Running { uptime_secs: 0 };
            tracing::info!("[fcc-sidecar] running on port {}", self.port);
            Ok(())
        } else {
            let msg = "FCC failed to become healthy within 15s".to_string();
            *self.status.write().await = FccStatus::Error(msg.clone());
            self.stop().await;
            Err(msg)
        }
    }

    /// Stop the sidecar process gracefully.
    pub async fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            tracing::info!("[fcc-sidecar] stopping");
            let _ = child.kill();
            let _ = child.wait();
        }
        self.started_at = None;
        *self.status.write().await = FccStatus::Stopped;
    }

    /// Get current status.
    pub async fn status(&self) -> FccStatus {
        let mut status = self.status.read().await.clone();
        // Update uptime if running
        if let FccStatus::Running { .. } = &status {
            if let Some(started) = self.started_at {
                status = FccStatus::Running {
                    uptime_secs: started.elapsed().as_secs(),
                };
            }
        }
        status
    }

    /// Probe the health endpoint.
    pub async fn health_check(&self) -> bool {
        health_check("127.0.0.1", self.port).await
    }

    /// Get the proxy base URL.
    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Returns (program, leading args) — the caller appends `server.py`.
/// `uv` needs `uv run python server.py`; plain interpreters take the
/// script directly.
fn find_python_command(fcc_dir: &PathBuf) -> Result<(String, Vec<String>), String> {
    // Try `uv run` first (FCC's recommended way)
    if let Ok(output) = Command::new("uv").arg("--version").output() {
        if output.status.success() {
            // Check if uv.lock exists in the FCC directory
            if fcc_dir.join("uv.lock").exists() {
                return Ok(("uv".to_string(), vec!["run".to_string(), "python".to_string()]));
            }
        }
    }

    // Fall back to python3 or python
    for cmd in &["python3", "python"] {
        if let Ok(output) = Command::new(cmd).arg("--version").output() {
            if output.status.success() {
                return Ok((cmd.to_string(), Vec::new()));
            }
        }
    }

    Err("Python not found. Install Python 3.14+ or uv.".to_string())
}

async fn wait_for_health(host: &str, port: u16, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if health_check(host, port).await {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    false
}

async fn health_check(host: &str, port: u16) -> bool {
    let url = format!("http://{host}:{port}/v1/models");
    match reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
    {
        Ok(client) => matches!(client.get(&url).send().await, Ok(r) if r.status().is_success()),
        Err(_) => false,
    }
}

// ── Singleton ────────────────────────────────────────────────────────────────

/// Global FCC sidecar instance. Initialized on boot if `fcc.enabled` is true.
lazy_static::lazy_static! {
    pub static ref FCC_SIDECAR: Arc<tokio::sync::Mutex<Option<FccSidecar>>> =
        Arc::new(tokio::sync::Mutex::new(None));
}

/// Python environment check result — returned by `check_python_env`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PythonEnvCheck {
    pub uv_available: bool,
    pub uv_version: Option<String>,
    pub python_available: bool,
    pub python_version: Option<String>,
    pub python_path: Option<String>,
    pub fcc_dir_exists: bool,
    pub fcc_deps_installed: bool,
    pub ready: bool,
}

/// Check the Python/uv environment for FCC. Used by the settings UI to show
/// setup status and offer one-click install.
pub fn check_python_env(fcc_dir: &PathBuf) -> PythonEnvCheck {
    let mut check = PythonEnvCheck {
        uv_available: false,
        uv_version: None,
        python_available: false,
        python_version: None,
        python_path: None,
        fcc_dir_exists: fcc_dir.join("server.py").exists(),
        fcc_deps_installed: false,
        ready: false,
    };

    // Check uv
    if let Ok(output) = Command::new("uv").arg("--version").output() {
        if output.status.success() {
            check.uv_available = true;
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            check.uv_version = Some(version);
        }
    }

    // Check Python
    for cmd in &["python3", "python"] {
        if let Ok(output) = Command::new(cmd).arg("--version").output() {
            if output.status.success() {
                check.python_available = true;
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                check.python_version = Some(version);
                check.python_path = Some(cmd.to_string());
                break;
            }
        }
    }

    // Check if FCC deps are installed (look for .venv or uv.lock)
    if fcc_dir.join(".venv").exists() || fcc_dir.join("uv.lock").exists() {
        check.fcc_deps_installed = true;
    }

    check.ready = check.fcc_dir_exists && (check.uv_available || check.python_available);
    check
}

/// Initialize the global FCC sidecar with the given directory and port.
pub async fn init_fcc(fcc_dir: PathBuf, port: u16) {
    let sidecar = FccSidecar::new(fcc_dir, port);
    *FCC_SIDECAR.lock().await = Some(sidecar);
}

/// Start the global FCC sidecar.
pub async fn start_fcc() -> Result<(), String> {
    let mut guard = FCC_SIDECAR.lock().await;
    match guard.as_mut() {
        Some(sidecar) => sidecar.start().await,
        None => Err("FCC sidecar not initialized".to_string()),
    }
}

/// Stop the global FCC sidecar.
pub async fn stop_fcc() {
    let mut guard = FCC_SIDECAR.lock().await;
    if let Some(sidecar) = guard.as_mut() {
        sidecar.stop().await;
    }
}

/// Get FCC status.
pub async fn fcc_status() -> FccStatus {
    let guard = FCC_SIDECAR.lock().await;
    match guard.as_ref() {
        Some(sidecar) => sidecar.status().await,
        None => FccStatus::Stopped,
    }
}

/// Get FCC base URL.
pub async fn fcc_base_url() -> Option<String> {
    let guard = FCC_SIDECAR.lock().await;
    guard.as_ref().map(|s| s.base_url())
}
