//! Sentinel settings: configurable service/workspace paths (Windows port of the
//! FlutterSentinel stack — no more hardcoded macOS `/Users/hades` paths).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelSettings {
    /// FlutterSentinel/SecuritySentinel repo root. Auto-detected on first run.
    pub sentinel_root: Option<String>,
    /// Where targets, findings, PoCs and reports are written.
    pub work_dir: String,
    /// Sidecar service directories (relative to `sentinel_root` when relative).
    pub fbhbot_dir: String,
    pub backend_dir: String,
    pub mobsf_dir: String,
    pub mcp_dir: String,
    pub ai_hunter_dir: String,
    /// Executables. `None` = use PATH resolution.
    pub node_bin: Option<String>,
    pub python_bin: Option<String>,
    /// Boot the FBHBot/backend sidecars when the IDE starts.
    pub auto_start_sidecars: bool,
    /// Actually hit live providers during secret validation.
    pub live_secret_check: bool,
    /// MobSF REST API key for authenticated scans (empty = keyless dev mode).
    pub mobsf_api_key: Option<String>,
    /// Local port forwarding to the jailbroken device's SSH (22) over usbmuxd.
    pub jb_ssh_port: Option<u16>,
    /// Default bug bounty program label for new targets.
    pub default_program: String,
}

impl Default for SentinelSettings {
    fn default() -> Self {
        Self {
            sentinel_root: None,
            work_dir: String::new(),
            fbhbot_dir: "core/fbhbot".to_string(),
            backend_dir: "backend".to_string(),
            mobsf_dir: "core/mobsf".to_string(),
            mcp_dir: "mcp-server".to_string(),
            ai_hunter_dir: "core/ai-hunter".to_string(),
            node_bin: None,
            python_bin: None,
            auto_start_sidecars: false,
            live_secret_check: false,
            mobsf_api_key: None,
            jb_ssh_port: None,
            default_program: String::new(),
        }
    }
}

/// Probe plausible FlutterSentinel roots (env var, cwd, parents). The old
/// codebase hardcoded `/Users/hades/Desktop/bugbounty/...` — we fix that by
/// resolving against the live filesystem on the user's OS.
pub fn detect_root() -> Option<PathBuf> {
    if let Ok(env_root) = std::env::var("FLUTTERSENTINEL_ROOT") {
        let p = PathBuf::from(env_root);
        if p.join("package.json").is_file() {
            return Some(p);
        }
    }
    let cwd = std::env::current_dir().ok()?;
    for candidate in [
        cwd.join("FlutterSentinel"),
        cwd.clone(),
        cwd.join("..").join("FlutterSentinel"),
    ] {
        if candidate.join("package.json").is_file() {
            return Some(candidate);
        }
    }
    None
}

impl SentinelSettings {
    pub fn load(config_dir: &Path) -> Self {
        let mut s = Self::default();
        let path = config_dir.join("sentinel.json");
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(parsed) = serde_json::from_str::<SentinelSettings>(&text) {
                s = parsed;
            }
        }
        if s.sentinel_root.is_none() {
            s.sentinel_root = detect_root().map(|p| p.to_string_lossy().to_string());
        }
        if s.work_dir.is_empty() {
            let root = s
                .sentinel_root
                .as_ref()
                .map(PathBuf::from)
                .unwrap_or_else(|| config_dir.to_path_buf());
            s.work_dir = root.join("targets").to_string_lossy().to_string();
        }
        let _ = std::fs::create_dir_all(&s.work_dir);
        s
    }

    pub fn save(&self, config_dir: &Path) -> Result<(), String> {
        let path = config_dir.join("sentinel.json");
        let text = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, text).map_err(|e| e.to_string())
    }

    /// Absolute path to a per-service directory (joins onto `sentinel_root`).
    pub fn service_dir(&self, dir: &str) -> PathBuf {
        let p = PathBuf::from(dir);
        if p.is_absolute() {
            return p;
        }
        self.sentinel_root
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_default()
            .join(p)
    }
}
