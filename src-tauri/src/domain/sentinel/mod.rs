//! FlutterSentinel integration domain: a `SentinelEngine` that owns the
//! settings file, the SQLite work DB, the sidecar process table and the
//! ported native tools (jwt / crypto / secrets / poc / report).
//!
//! The engine is exposed two ways so both frontends can reach it:
//!   * Tauri shell: `app.manage(Arc<SentinelEngine>)` (commands take
//!     `State<'_, Arc<SentinelEngine>>`).
//!   * Headless / native-gpui shell: globals `register()` + `engine()`.
//! Domain code never imports `tauri::`.

pub mod crypto;
pub mod db;
pub mod frida_presets;
pub mod jbops;
pub mod jwt;
pub mod mobile;
pub mod poc;
pub mod report;
pub mod secrets;
pub mod settings;
pub mod sidecars;

pub use settings::SentinelSettings;

use crate::domain::sentinel::db::SentinelDb;
use crate::domain::sentinel::poc::{generate_poc, write_poc, PoCKinds, PoCOptions};
use crate::domain::sentinel::report::{render_markdown, write_report, ReportInput, ReportPlatform};
use crate::domain::sentinel::secrets::{validate_secret, SecretKind};
use crate::domain::sentinel::sidecars::{SidecarHandle, SidecarKind, SidecarTable};
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

/// Global engine (also managed by Tauri state). Only set once, from setup.
static GLOBAL: OnceLock<Arc<SentinelEngine>> = OnceLock::new();

/// Register the engine for headless/native shells (call exactly once, at boot).
pub fn register(engine: Arc<SentinelEngine>) -> Result<()> {
    GLOBAL
        .set(engine)
        .map_err(|_| anyhow!("sentinel engine already registered"))
}

/// The active engine, if the domain was booted.
pub fn engine() -> Option<Arc<SentinelEngine>> {
    GLOBAL.get().cloned()
}

/// Boot the engine from a config dir (used by both shells).
pub fn boot(config_dir: impl AsRef<Path>) -> Result<Arc<SentinelEngine>> {
    let engine = Arc::new(SentinelEngine::new(config_dir)?);
    register(engine.clone()).map_err(|e| anyhow!("engine already alive: {e}"))?;
    Ok(engine)
}

/// The engine: one instance per process owning settings, DB, sidecars and
/// running sidecar metadata.
pub struct SentinelEngine {
    config_dir: PathBuf,
    work_dir: PathBuf,
    logs_dir: PathBuf,
    settings: RwLock<SentinelSettings>,
    db: Mutex<SentinelDb>,
    sidecars: SidecarTable,
    handles: Mutex<Vec<SidecarHandle>>,
}

impl SentinelEngine {
    pub fn new(config_dir: impl AsRef<Path>) -> Result<Self> {
        let config_dir = config_dir.as_ref().to_path_buf();
        let settings = settings::SentinelSettings::load(&config_dir);
        let work_dir = PathBuf::from(&settings.work_dir);
        std::fs::create_dir_all(&work_dir)
            .map_err(|e| anyhow!("create work dir {}: {e}", work_dir.display()))?;
        let logs_dir = config_dir.join("logs");
        std::fs::create_dir_all(&logs_dir).map_err(|e| anyhow!("create logs dir: {e}"))?;
        let db = SentinelDb::open(&config_dir.join("sentinel.db"))
            .or_else(|_| SentinelDb::open_memory())?;
        Ok(Self {
            config_dir,
            work_dir,
            logs_dir,
            settings: RwLock::new(settings),
            db: Mutex::new(db),
            sidecars: SidecarTable::new(),
            handles: Mutex::new(Vec::new()),
        })
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    pub fn logs_dir(&self) -> &Path {
        &self.logs_dir
    }

    pub fn settings(&self) -> SentinelSettings {
        self.settings.read().unwrap().clone()
    }

    pub fn update_settings(&self, f: impl FnOnce(&mut SentinelSettings)) -> Result<()> {
        let mut guard = self.settings.write().unwrap();
        f(&mut guard);
        let updated = guard.clone();
        drop(guard);
        updated
            .save(&self.config_dir)
            .map_err(|e| anyhow!("save settings: {e}"))?;
        let work_dir = PathBuf::from(&updated.work_dir);
        std::fs::create_dir_all(&work_dir).map_err(|e| anyhow!("create work dir: {e}"))?;
        self.commit_settings(updated)
    }

    fn commit_settings(&self, s: SentinelSettings) -> Result<()> {
        let mut guard = self.settings.write().unwrap();
        *guard = s;
        Ok(())
    }

    pub fn db(&self) -> &Mutex<SentinelDb> {
        &self.db
    }

    // ── Sidecars ──

    pub fn spawn_sidecar(&self, kind: SidecarKind) -> Result<Value> {
        let s = self.settings();
        let logs = self.logs_dir().to_path_buf();
        let (child, log_path) = sidecars::spawn(&s, kind, logs).map_err(|e| anyhow!("{e}"))?;
        let pid = child.id();
        let handle = SidecarHandle {
            kind,
            pid: Some(pid),
            started_at: chrono::Utc::now().to_rfc3339(),
            log_path,
        };
        self.sidecars.register(kind.label(), child);
        self.handles.lock().unwrap().push(handle);
        Ok(self
            .sidecar_status(kind)
            .unwrap_or(json!({ "kind": kind.label(), "state": "running" })))
    }

    pub fn stop_sidecar(&self, kind: SidecarKind) -> Result<Value> {
        let killed = self.sidecars.kill(kind.label());
        Ok(json!({ "kind": kind.label(), "was_running": killed, "state": "stopped" }))
    }

    pub fn sidecar_status(&self, kind: SidecarKind) -> Option<Value> {
        let label = kind.label();
        let running = self.sidecars.live(label);
        if !running {
            return Some(json!({ "kind": label, "state": "stopped" }));
        }
        let port = kind
            .port()
            .map(|p| sidecars::port_open("127.0.0.1", p, 250))
            .unwrap_or(false);
        Some(json!({
            "kind": label,
            "state": "running",
            "port": kind.port(),
            "healthy": port,
        }))
    }

    pub fn all_sidecar_status(&self) -> Vec<Value> {
        [
            SidecarKind::Fbhbot,
            SidecarKind::Backend,
            SidecarKind::Mobsf,
            SidecarKind::AiHunter,
            SidecarKind::McpServer,
        ]
        .iter()
        .filter_map(|k| self.sidecar_status(*k))
        .collect()
    }

    pub fn bootstrap_sidecars(&self) -> Result<Value> {
        let s = self.settings();
        if !s.auto_start_sidecars {
            return Ok(json!({ "skipped": true, "reason": "auto_start_sidecars is off" }));
        }
        let mut started = Vec::new();
        for kind in [
            SidecarKind::Fbhbot,
            SidecarKind::Backend,
            SidecarKind::Mobsf,
            SidecarKind::AiHunter,
        ] {
            match self.spawn_sidecar(kind) {
                Ok(v) => started.push(v),
                Err(e) => started.push(json!({ "kind": kind.label(), "error": e.to_string() })),
            }
        }
        Ok(json!({ "started": started }))
    }

    // ── High-level operations (the tools layer calls these) ──

    pub fn scan_secrets(&self, text: &str) -> Vec<Value> {
        secrets::scan_text(text)
            .into_iter()
            .map(|m| {
                let mut rendered = json!({
                    "kind": m.kind.to_string(),
                    "value": m.value,
                    "position": m.position,
                    "entropy": m.entropy,
                    "valid_format": m.valid_format,
                });
                if !m.valid_format {
                    rendered["value"] = Value::String(redact(&m.value));
                }
                rendered
            })
            .collect()
    }

    pub fn validate_secret(&self, kind: SecretKind, value: &str) -> Value {
        let m = validate_secret(kind, value);
        json!({
            "kind": m.kind.to_string(),
            "value": redact(&m.value),
            "entropy": m.entropy,
            "valid_format": m.valid_format,
        })
    }

    pub fn gen_poc(&self, kind: PoCKinds, opts: &PoCOptions) -> Result<Value> {
        let path =
            write_poc(&self.work_dir, &opts.target, kind, opts).map_err(|e| anyhow!("{e}"))?;
        Ok(json!({
            "kind": kind.label(),
            "path": path.to_string_lossy(),
            "code": generate_poc(kind, opts),
        }))
    }

    pub fn make_report(&self, platform: ReportPlatform, input: &ReportInput) -> Result<Value> {
        let path = write_report(&self.work_dir, platform, input).map_err(|e| anyhow!("{e}"))?;
        let body = render_markdown(platform, input);
        let db = self.db.lock().unwrap();
        let row = db.create_report(
            &input.target,
            platform.label(),
            &input.title,
            &body,
            "draft",
        )?;
        Ok(json!({
            "path": path.to_string_lossy(),
            "report_row": row,
        }))
    }

    // ── Mobile bounty drive (acquire → MobSF/local scan → findings) ──

    pub fn mobsf_up(&self) -> bool {
        sidecars::port_open("127.0.0.1", 8000, 250)
    }

    pub async fn mobsf_ping(&self) -> Value {
        let s = self.settings();
        let client =
            mobile::MobsfClient::new("127.0.0.1", 8000, s.mobsf_api_key.as_deref().unwrap_or(""));
        match client.ping().await {
            Ok(v) => json!({ "reachable": true, "scans": v }),
            Err(e) => json!({ "reachable": false, "error": e.to_string() }),
        }
    }

    /// APKs/iPAs sitting in `work_dir/uploads/` (or anywhere under it).
    pub fn discover_mobile_apps(&self) -> Vec<Value> {
        let uploads = self.work_dir.join("uploads");
        let _ = std::fs::create_dir_all(&uploads);
        mobile::find_mobile_apps(&uploads)
            .into_iter()
            .map(|p| {
                json!({
                    "path": p.to_string_lossy(),
                    "name": p.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default(),
                })
            })
            .collect()
    }

    /// `adb shell pm path <pkg>` + `adb pull` from a connected device.
    pub fn pull_mobile_apk(&self, package: &str) -> Result<Value> {
        let out = self.work_dir.join("uploads");
        std::fs::create_dir_all(&out).map_err(|e| anyhow!("create uploads: {e}"))?;
        let apk = mobile::pull_apk_from_device(package, &out).map_err(|e| anyhow!("{e}"))?;
        let size = std::fs::metadata(&apk).map(|m| m.len()).unwrap_or(0);
        Ok(
            json!({ "package": package, "path": apk.to_string_lossy(), "size": size, "device": "adb" }),
        )
    }

    /// Run the full mobile pipeline for one app and persist the scan.
    pub async fn analyze_mobile(&self, apk_path: &str) -> Result<Value> {
        let s = self.settings();
        let cand = {
            let p = std::path::PathBuf::from(apk_path);
            if p.is_absolute() {
                p
            } else {
                self.work_dir.join("uploads").join(apk_path)
            }
        };
        if !cand.is_file() {
            return Err(anyhow!(
                "no such apk: {} (try sentinel_mobile_discover first)",
                cand.display()
            ));
        }
        let env = mobile::MobileEnv {
            mobsf_url: "http://127.0.0.1:8000".to_string(),
            api_key: s.mobsf_api_key.clone().unwrap_or_default(),
            artifacts_dir: self.work_dir.join("mobile"),
            mobsf_up: self.mobsf_up(),
        };
        let result = mobile::analyze_apk(&cand, &env)
            .await
            .map_err(|e| anyhow!("{e}"))?;

        let analysis = result.get("analysis").cloned().unwrap_or_default();
        let findings = analysis
            .get("findings")
            .and_then(|f| f.as_array())
            .map(Vec::len)
            .unwrap_or(0);
        let endpoints = analysis
            .get("endpoints")
            .and_then(|e| e.as_array())
            .map(Vec::len)
            .unwrap_or(0);
        let mut sev = "info".to_string();
        if let Some(fs) = analysis.get("findings").and_then(|f| f.as_array()) {
            if let Some(top) = fs
                .iter()
                .find(|f| f["severity"].as_str().map(|s| s != "info").unwrap_or(false))
            {
                sev = top["severity"].as_str().unwrap_or("info").to_string();
            }
        }
        let mut row = db::MobileAssetRow {
            app_name: analysis
                .get("app_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            package: analysis
                .get("package")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            version: analysis
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            apk_name: cand
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default(),
            file_path: cand.to_string_lossy().to_string(),
            scan_ref: result
                .get("scan_hash")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            method: result
                .get("method")
                .and_then(|v| v.as_str())
                .unwrap_or("local")
                .to_string(),
            status: "scanned".to_string(),
            findings_count: findings as i64,
            severity: sev,
            endpoints_count: endpoints as i64,
        };
        let asset_row = self
            .db()
            .lock()
            .unwrap()
            .create_mobile_asset(&mut row)
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({
            "method": result.get("method"),
            "analysis": analysis,
            "asset_row": asset_row,
        }))
    }

    pub fn list_mobile_assets(&self) -> Vec<Value> {
        self.db()
            .lock()
            .unwrap()
            .list_mobile_assets()
            .unwrap_or_default()
    }

    pub fn delete_mobile_asset(&self, id: &str) -> Result<Value> {
        self.db()
            .lock()
            .unwrap()
            .delete_mobile_asset(id)
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({ "deleted": id }))
    }

    pub fn gen_frida_script(&self, kind: mobile::FridaScriptKind) -> Result<Value> {
        let dir = self.work_dir.join("frida");
        let path = mobile::write_frida_script(kind, &dir).map_err(|e| anyhow!("{e}"))?;
        Ok(json!({
            "kind": kind.label(),
            "path": path.to_string_lossy(),
            "code": mobile::frida_script(kind),
        }))
    }

    // ── Jailbreak ops (live iPhone XR: tunnel, frida, dump → MobSF) ──

    pub async fn jb_status(&self) -> Value {
        jbops::status().await
    }

    pub async fn jb_devices(&self) -> Result<Value> {
        Ok(json!({ "devices": jbops::list_devices().await? }))
    }

    pub async fn jb_apps(&self, udid: Option<String>) -> Result<Value> {
        Ok(json!({ "apps": jbops::list_apps(udid.as_deref()).await? }))
    }

    pub async fn jb_live_scan(
        &self,
        target: &str,
        kind: jbops::LiveScriptKind,
        filter: &str,
    ) -> Result<Value> {
        jbops::frida_run(target, kind, filter).await
    }

    pub async fn jb_ensure_tunnel(&self, udid: &str) -> Result<Value> {
        let port = self
            .settings
            .read()
            .unwrap()
            .jb_ssh_port
            .unwrap_or(jbops::DEFAULT_SSH_PORT);
        jbops::ensure_tunnel(udid, port, 22).await
    }

    pub async fn jb_ssh(&self, command: &str) -> Result<Value> {
        let port = self
            .settings
            .read()
            .unwrap()
            .jb_ssh_port
            .unwrap_or(jbops::DEFAULT_SSH_PORT);
        Ok(json!({ "output": jbops::ssh_exec(port, command, 30).await? }))
    }

    /// Dump an installed app's bundle from the device and hand the resulting
    /// `.ipa` to the existing MobSF/local pipeline. Returns asset + scan ref.
    pub async fn jb_dump_to_ipa(&self, udid: Option<String>, bundle: &str) -> Result<Value> {
        let udid = match udid {
            Some(u) if !u.trim().is_empty() => u,
            _ => jbops::device_udid()
                .await
                .ok_or_else(|| anyhow!("no device found — plug in the iPhone and tap Trust"))?,
        };
        let port = self
            .settings
            .read()
            .unwrap()
            .jb_ssh_port
            .unwrap_or(jbops::DEFAULT_SSH_PORT);
        jbops::ensure_tunnel(&udid, port, 22).await?;
        let out_dir = self.work_dir.join("uploads");
        std::fs::create_dir_all(&out_dir)?;
        let ipa = jbops::dump_to_ipa(&udid, bundle, port, &out_dir).await?;
        // Feed the static pipeline: same path sentinel_mobile_analyze uses.
        let scan = self.analyze_mobile(&ipa.to_string_lossy()).await?;
        Ok(json!({
            "udid": udid,
            "bundle": bundle,
            "ipa": ipa.to_string_lossy(),
            "scan": scan,
        }))
    }

    pub fn stats(&self) -> Value {
        self.db
            .lock()
            .unwrap()
            .stats()
            .unwrap_or_else(|e| json!({ "error": e.to_string() }))
    }
}

/// Blank everything but the first/last four chars of a secret for display.
fn redact(s: &str) -> String {
    if s.len() <= 8 {
        return "****".to_string();
    }
    let chars: Vec<char> = s.chars().collect();
    format!(
        "{}...{}",
        chars[..4].iter().collect::<String>(),
        chars[chars.len() - 4..].iter().collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_boots_in_memory() {
        let dir = std::env::temp_dir().join(format!("sentinel-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let e = SentinelEngine::new(&dir).expect("boot");
        e.db.lock()
            .unwrap()
            .create_target("example.com", None, None, None, None, None)
            .unwrap();
        assert_eq!(e.stats()["targets"], 1);
    }

    #[test]
    fn global_register_once() {
        assert!(register(Arc::new(
            SentinelEngine::new(std::env::temp_dir().join("sentinel-global-test")).unwrap()
        ))
        .is_ok());
        let _ = engine();
    }
}
