//! Mobile bug-bounty profit loop: acquire APKs/iPAs → MobSF static scan (or a
//! local strings/dex fallback when the sidecar is down) → distill findings with
//! CWE + OWASP MASVS tags → persist scan metadata → optional Frida bypass
//! scripts for dynamic confirmation.
//!
//! Headless-safe: no `tauri::` imports; `reqwest` (multipart) drives the MobSF
//! REST API, `std::process` drives adb.

use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::io::Read;
use std::path::{Path, PathBuf};

/// MobSF APIv2 client. `api_key` empty disables the auth header (MobSF default).
#[derive(Clone)]
pub struct MobsfClient {
    base_url: String,
    http: reqwest::Client,
}

impl MobsfClient {
    pub fn new(host: &str, port: u16, api_key: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        if !api_key.is_empty() {
            if let Ok(v) = reqwest::header::HeaderValue::from_str(api_key) {
                headers.insert("X-Mobsf-Api-Key", v);
            }
        }
        let base_url = format!("http://{host}:{port}");
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { base_url, http }
    }

    fn scan_type(path: &Path) -> &'static str {
        if path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("ipa"))
            .unwrap_or(false)
        {
            "ipa"
        } else {
            "apk"
        }
    }

    /// POST /api/v1/upload → {"hash": ...}.
    pub async fn upload(&self, path: &Path) -> Result<String> {
        let scan_type = Self::scan_type(path);
        let fname = path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "app.bin".to_string());
        let bytes = std::fs::read(path).map_err(|e| anyhow!("read {}: {e}", path.display()))?;
        let part = reqwest::multipart::Part::bytes(bytes)
            .file_name(fname)
            .mime_str(if scan_type == "ipa" {
                "application/octet-stream"
            } else {
                "application/vnd.android.package-archive"
            })
            .map_err(|e| anyhow!("mime: {e}"))?;
        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("scan_type", scan_type)
            .text("force", "1");
        let resp = self
            .http
            .post(format!("{}/api/v1/upload", self.base_url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("mobsf upload: {e}"))?;
        let v: Value = resp
            .json()
            .await
            .map_err(|e| anyhow!("mobsf upload json: {e}"))?;
        if let Some(err) = v.get("error").and_then(|x| x.as_str()) {
            return Err(anyhow!("mobsf upload error: {err}"));
        }
        v.get("hash")
            .and_then(|h| h.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("no hash in upload response: {v}"))
    }

    /// POST /api/v1/scan triggers the (blocking) analysis.
    pub async fn scan(&self, hash: &str, scan_type: &str) -> Result<Value> {
        let form = reqwest::multipart::Form::new()
            .text("hash", hash.to_string())
            .text("scan_type", scan_type.to_string())
            .text("re_scan", "1");
        let resp = self
            .http
            .post(format!("{}/api/v1/scan", self.base_url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("mobsf scan: {e}"))?;
        let v: Value = resp
            .json()
            .await
            .map_err(|e| anyhow!("mobsf scan json: {e}"))?;
        if let Some(err) = v.get("error").and_then(|x| x.as_str()) {
            return Err(anyhow!("mobsf scan error: {err}"));
        }
        Ok(v)
    }

    /// POST /api/v1/report_json → the full static-analysis report.
    pub async fn report_json(&self, hash: &str, scan_type: &str) -> Result<Value> {
        let form = reqwest::multipart::Form::new()
            .text("hash", hash.to_string())
            .text("scan_type", scan_type.to_string());
        let resp = self
            .http
            .post(format!("{}/api/v1/report_json", self.base_url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("mobsf report_json: {e}"))?;
        let v: Value = resp
            .json()
            .await
            .map_err(|e| anyhow!("mobsf report json: {e}"))?;
        if let Some(err) = v
            .get("error")
            .and_then(|x| x.as_str())
            .or_else(|| v.get("detail").and_then(|x| x.as_str()))
        {
            return Err(anyhow!("mobsf report error: {err}"));
        }
        Ok(v)
    }

    /// POST /api/v1/report_pdf → raw PDF bytes.
    pub async fn report_pdf(&self, hash: &str, scan_type: &str) -> Result<Vec<u8>> {
        let form = reqwest::multipart::Form::new()
            .text("hash", hash.to_string())
            .text("scan_type", scan_type.to_string());
        let resp = self
            .http
            .post(format!("{}/api/v1/report_pdf", self.base_url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow!("mobsf report_pdf: {e}"))?;
        if !resp.status().is_success() {
            return Err(anyhow!("mobsf report_pdf status {:?}", resp.status()));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| anyhow!("mobsf pdf bytes: {e}"))?;
        Ok(bytes.to_vec())
    }

    /// Health check: is the API reachable + authenticated?
    pub async fn ping(&self) -> Result<Value> {
        let resp = self
            .http
            .get(format!("{}/api/v1/scans", self.base_url))
            .send()
            .await
            .map_err(|e| anyhow!("mobsf ping: {e}"))?;
        let v: Value = resp
            .json()
            .await
            .map_err(|e| anyhow!("mobsf ping json: {e}"))?;
        if let Some(err) = v.get("detail").and_then(|x| x.as_str()) {
            return Err(anyhow!("mobsf: {err}"));
        }
        Ok(v)
    }
}

/// Where artifacts land and how the engine reaches MobSF.
pub struct MobileEnv {
    pub mobsf_url: String,
    pub api_key: String,
    pub artifacts_dir: PathBuf,
    pub mobsf_up: bool,
}

/// Recursively (bounded depth) find APK/AAB/IPA files under `root`. Used for
/// the acquisition step: drop acquired apps into `work_dir/uploads/`.
pub fn find_mobile_apps(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    find_mobile_apps_at(root, &mut out, 0);
    out
}

fn find_mobile_apps_at(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > 3 || out.len() >= 64 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            find_mobile_apps_at(&p, out, depth + 1);
        } else if p.is_file() {
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                if matches!(ext.to_ascii_lowercase().as_str(), "apk" | "aab" | "ipa") {
                    out.push(p);
                }
            }
        }
    }
}

/// Pull an installed app from a connected adb device → returns local APK path.
/// Falls back gracefully: works whether `adb` is on PATH or resolved via SDK.
pub fn pull_apk_from_device(package: &str, out_dir: &Path) -> Result<PathBuf> {
    let mut cmd = adb_cmd();
    cmd.arg("shell").arg("pm").arg("path").arg(package);
    let out = cmd
        .output()
        .map_err(|e| anyhow!("adb shell pm path: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let remote = stdout
        .lines()
        .filter_map(|l| l.trim().strip_prefix("package:"))
        .next()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("no package {package} on device (adb output: {stdout})"))?;

    std::fs::create_dir_all(out_dir)?;
    let safe = package.replace(['/', '\\', ':'], "_");
    let dest = out_dir.join(format!("{safe}.apk"));
    let mut pull = adb_cmd();
    pull.arg("pull").arg(&remote).arg(&dest);
    let res = pull.output().map_err(|e| anyhow!("adb pull: {e}"))?;
    if !res.status.success() {
        return Err(anyhow!(
            "adb pull failed: {}",
            String::from_utf8_lossy(&res.stderr)
        ));
    }
    Ok(dest)
}

/// Resolve `adb` from ANDROID_HOME/ANDROID_SDK_ROOT, then PATH. Hidden-spawned.
fn adb_cmd() -> std::process::Command {
    use crate::infrastructure::process_ext::CommandExtHidden;
    let exe = if cfg!(windows) { "adb.exe" } else { "adb" };
    let mut cmd = resolve_adb(exe);
    cmd.hidden_sidecar();
    cmd
}

fn resolve_adb(exe: &str) -> std::process::Command {
    for var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if let Ok(v) = std::env::var(var) {
            let p = PathBuf::from(v).join("platform-tools").join(exe);
            if p.exists() {
                return std::process::Command::new(p);
            }
        }
    }
    if let Ok(p) = which::which("adb") {
        return std::process::Command::new(p);
    }
    std::process::Command::new(exe)
}

// ═══ MobSF report → findings distillation ═══

/// Severity for a MobSF `binary_analysis` category.
fn bin_sev(category: &str) -> &'static str {
    let c = category.to_ascii_lowercase();
    if c.contains("hardcoded")
        || c.contains("weak_encryption")
        || c.contains("certificate_signature_validation")
        || c.contains("signature_verification_bypass")
        || c.contains("pincert")
        || c.contains("secret_key")
        || c.contains("malformed")
        || c.contains("dns_rebinding")
    {
        "high"
    } else if c.contains("exported")
        || c.contains("debug")
        || c.contains("log")
        || c.contains("insecure")
    {
        "medium"
    } else {
        "info"
    }
}

fn cwe_for(category: &str) -> &'static str {
    let c = category.to_ascii_lowercase();
    if c.contains("hardcoded") || c.contains("secret_key") || c.contains("api_key") {
        "CWE-798"
    } else if c.contains("encryption")
        || c.contains("certificate")
        || c.contains("signature")
        || c.contains("pincert")
        || c.contains("malformed")
    {
        "CWE-327"
    } else if c.contains("exported") {
        "CWE-926"
    } else {
        "CWE-200"
    }
}

fn masvs_for(category: &str) -> &'static str {
    let c = category.to_ascii_lowercase();
    if c.contains("hardcoded") || c.contains("secret_key") || c.contains("api_key") {
        "MASWE-2"
    } else if c.contains("encryption")
        || c.contains("certificate")
        || c.contains("signature")
        || c.contains("pincert")
    {
        "MASWE-6"
    } else if c.contains("exported") {
        "MASVS-CODE-4"
    } else {
        "MASVS-STORAGE-1"
    }
}

fn extract_string_list(v: &Value, keys: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for k in keys {
        match v.get(k) {
            Some(Value::Array(items)) => {
                for it in items {
                    match it {
                        Value::String(s) => out.push(s.clone()),
                        Value::Object(map) => {
                            for f in ["domain", "url", "path", "name"] {
                                if let Some(Value::String(s)) = map.get(f) {
                                    out.push(s.clone());
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Some(Value::String(s)) => out.push(s.clone()),
            _ => {}
        }
    }
    out
}

/// Turn a MobSF `report_json` document into triaged findings + recon endpoints.
pub fn distill_report(report: &Value) -> Value {
    let mut findings: Vec<Value> = Vec::new();

    // Binary-analysis categories (the big one).
    if let Some(bin) = report.get("binary_analysis").and_then(|b| b.as_object()) {
        for (cat, items) in bin {
            if items.is_array() && !items.as_array().unwrap().is_empty() {
                let sev = bin_sev(cat);
                findings.push(json!({
                    "kind": cat,
                    "severity": sev,
                    "cwe": cwe_for(cat),
                    "masvs": masvs_for(cat),
                    "count": items.as_array().unwrap().len(),
                    "detail": format!("[+] {cat} ({} instances)", items.as_array().unwrap().len()),
                }));
            }
        }
    }

    // Exported components missing intent filters / no permission.
    for comp in [
        "exported_activities",
        "exported_services",
        "exported_receivers",
        "exported_providers",
        "exported_utils",
    ] {
        if let Some(items) = report.get(comp).and_then(|v| v.as_array()) {
            if !items.is_empty() {
                findings.push(json!({
                    "kind": comp,
                    "severity": if comp == "exported_utils" { "high" } else { "medium" },
                    "cwe": "CWE-926",
                    "masvs": "MASVS-CODE-4",
                    "count": items.len(),
                    "detail": format!("[+] {comp}: {} exposed", items.len()),
                }));
            }
        }
    }

    // Hostile permissions that change the attack surface.
    const HOT_PERMS: &[&str] = &[
        "android.permission.INSTALL_PACKAGES",
        "android.permission.REQUEST_INSTALL_PACKAGES",
        "android.permission.QUERY_ALL_PACKAGES",
        "android.permission.SYSTEM_ALERT_WINDOW",
        "android.permission.MANAGE_EXTERNAL_STORAGE",
    ];
    if let Some(perms) = report.get("permissions").and_then(|v| v.as_array()) {
        for plist in ["permissions", "androidpermissions"] {
            let _ = plist;
        }
        let granted_owned: Vec<String> = perms
            .iter()
            .filter_map(|p| {
                p.get("permission")
                    .and_then(|x| x.as_str())
                    .map(|s| s.to_string())
            })
            .collect();
        for hot in HOT_PERMS {
            if granted_owned.iter().any(|p| p == hot) {
                findings.push(json!({
                    "kind": "permission",
                    "severity": "high",
                    "cwe": "CWE-250",
                    "masvs": "MASVS-PLATFORM-1",
                    "count": 1,
                    "detail": format!("[+] Hostile permission granted: {hot}"),
                }));
            }
        }
    }

    // Debuggable / backup-allowed flags.
    if let Some(dbg) = report
        .get("application_debuggable")
        .and_then(|v| v.as_bool())
    {
        if dbg {
            findings.push(json!({
                "kind": "debuggable_application",
                "severity": "critical",
                "cwe": "CWE-489",
                "masvs": "MASVS-RESILIENCE-1",
                "count": 1,
                "detail": "[+] Application is debuggable (production misuse)",
            }));
        }
    }
    if let Some(backup) = report
        .get("android_backup_enabled")
        .and_then(|v| v.as_bool())
    {
        if backup {
            findings.push(json!({
                "kind": "insecure_backup",
                "severity": "medium",
                "cwe": "CWE-921",
                "masvs": "MASVS-STORAGE-2",
                "count": 1,
                "detail": "[+] android:allowBackup=true — data extractable via adb backup",
            }));
        }
    }
    if let Some(clear) = report.get("cleartext_traffic").and_then(|v| v.as_bool()) {
        if clear {
            findings.push(json!({
                "kind": "cleartext_traffic",
                "severity": "high",
                "cwe": "CWE-319",
                "masvs": "MASVS-NETWORK-1",
                "count": 1,
                "detail": "[+] usesCleartextTraffic enabled / network security config allows cleartext",
            }));
        }
    }

    findings.sort_by(|a, b| sev_rank(&b["severity"]).cmp(&sev_rank(&a["severity"])));

    let endpoints = {
        let mut eps = extract_string_list(report, &["domains", "urls"]);
        eps.sort();
        eps.dedup();
        eps.into_iter().take(200).collect::<Vec<_>>()
    };

    json!({
        "findings": findings,
        "endpoints": endpoints,
        "security_score": report.get("security_score"),
        "package": report.get("package_name").and_then(|v| v.as_str()).unwrap_or(""),
        "app_name": report.get("app_name").and_then(|v| v.as_str()).unwrap_or(""),
        "version": report.get("version_name").and_then(|v| v.as_str()).unwrap_or(""),
        "target_sdk": report.get("target_sdk"),
        "min_sdk": report.get("min_sdk"),
        "permissions_total": report.get("permissions").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0),
    })
}

fn sev_rank(v: &Value) -> usize {
    match v.as_str().unwrap_or("info") {
        "critical" => 5,
        "high" => 4,
        "medium" => 3,
        "low" => 2,
        _ => 1,
    }
}

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

/// Extract printable strings from a byte blob (dex/so) as ASCII runs.
fn printable_strings(data: &[u8], min_len: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = Vec::new();
    for &b in data {
        if (0x20..=0x7e).contains(&b) || b == b'\t' {
            cur.push(b);
        } else {
            if cur.len() >= min_len {
                out.push(String::from_utf8_lossy(&cur).into_owned());
            }
            cur.clear();
            if out.len() >= 40_000 {
                break;
            }
        }
    }
    if cur.len() >= min_len {
        out.push(String::from_utf8_lossy(&cur).into_owned());
    }
    out
}

/// Local fallback when MobSF is down: list the APK container, pull the largest
/// classes.dex, extract strings, and run the Sentinel secret scanner + URL
/// harvest over them. Bounded, returns structured samples.
pub fn local_fallback_scan(apk: &Path) -> Result<Value> {
    let file = std::fs::File::open(apk).map_err(|e| anyhow!("open {}: {e}", apk.display()))?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| anyhow!("not a valid zip/apk: {e}"))?;

    let mut entries = Vec::new();
    for i in 0..archive.len() {
        let name = archive
            .by_index(i)
            .map(|f| f.name().to_string())
            .unwrap_or_default();
        entries.push(name);
    }

    // Find classes.dex (biggest one).
    let mut dex_name: Option<String> = None;
    let mut dex_max = 0usize;
    for name in &entries {
        if name.ends_with("classes.dex") {
            if let Ok(f) = archive.by_name(name) {
                let size = f.size() as usize;
                if size > dex_max {
                    dex_max = size;
                    dex_name = Some(name.clone());
                }
            }
        }
    }

    let mut blobs = Vec::new();
    if let Some(dname) = dex_name {
        let mut f = archive
            .by_name(&dname)
            .map_err(|e| anyhow!("read {dname}: {e}"))?;
        let mut buf = Vec::with_capacity(dex_max.min(16 * 1024 * 1024));
        let mut chunk = [0u8; 65536];
        loop {
            let n = f.read(&mut chunk).map_err(|e| anyhow!("read dex: {e}"))?;
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
            if buf.len() >= 16 * 1024 * 1024 {
                break;
            }
        }
        blobs.push((dname, buf));
    }

    let mut interesting = Vec::new();
    let mut endpoints: Vec<String> = Vec::new();
    let mut secret_hits: Vec<Value> = Vec::new();
    let url_re = regex::Regex::new(r"https?://[A-Za-z0-9._\-:/?#\[\]@!$&'()*+,;=%]{6,}").unwrap();

    let mut pool = String::with_capacity(1024 * 1024);
    for (name, data) in &blobs {
        let strings = printable_strings(data, 6);
        interesting.push(json!({ "blob": name, "count": strings.len() }));
        endpoints.extend(
            strings
                .iter()
                .filter_map(|s| url_re.find(s).map(|m| m.as_str().to_string()))
                .map(|u| u.chars().take(256).collect::<String>()),
        );
        for s in strings.iter().take(3000) {
            if pool.len() < 1024 * 1024 {
                pool.push_str(s);
                pool.push('\n');
            }
        }
    }

    secret_hits.extend(
        crate::domain::sentinel::secrets::scan_text(&pool)
            .into_iter()
            .map(|m| {
                json!({
                    "kind": m.kind.to_string(),
                    "value": if m.valid_format { m.value.clone() } else { redact(&m.value) },
                    "position": m.position,
                    "entropy": m.entropy,
                    "valid_format": m.valid_format,
                })
            }),
    );

    endpoints.sort();
    endpoints.dedup();

    Ok(json!({
        "method": "local",
        "entries": entries.iter().take(60).cloned().collect::<Vec<_>>(),
        "dex": blobs.first().map(|(n, _)| n.clone()),
        "interesting": interesting,
        "endpoints": endpoints.iter().take(100).cloned().collect::<Vec<_>>(),
        "secret_hits": secret_hits,
    }))
}

/// Full analyze: MobSF pipeline when reachable, else local fallback. Writes
/// `report.json` + optional `report.pdf` into `artifacts_dir/<stub>/`.
pub async fn analyze_apk(apk: &Path, env: &MobileEnv) -> Result<Value> {
    let stub = apk
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "app".to_string());
    let out_dir = env.artifacts_dir.join(&stub);
    std::fs::create_dir_all(&out_dir)?;

    if env.mobsf_up {
        let client = MobsfClient::new("127.0.0.1", 8000, &env.api_key);
        let scan_type = MobsfClient::scan_type(apk);
        match client.upload(apk).await {
            Ok(hash) => {
                let _ = client.scan(&hash, scan_type).await;
                let report = client.report_json(&hash, scan_type).await?;
                let _ = &report;
                let json_path = out_dir.join("report.json");
                std::fs::write(&json_path, serde_json::to_vec_pretty(&report)?)
                    .map_err(|e| anyhow!("write report.json: {e}"))?;

                let pdf_path = match client.report_pdf(&hash, scan_type).await {
                    Ok(bytes) => {
                        let p = out_dir.join("report.pdf");
                        let _ = std::fs::write(&p, bytes);
                        Some(p.to_string_lossy().to_string())
                    }
                    Err(_) => None,
                };

                let distilled = distill_report(&report);
                Ok(json!({
                    "apk": apk.to_string_lossy().to_string(),
                    "method": "mobsf",
                    "scan_hash": hash,
                    "artifacts": {
                        "report_json": json_path.to_string_lossy().to_string(),
                        "report_pdf": pdf_path.unwrap_or_default(),
                    },
                    "analysis": distilled,
                }))
            }
            Err(up_err) => {
                // Upload itself failed (e.g. invalid ADB key) — degrade to local.
                let local = local_fallback_scan(apk).unwrap_or_else(|e| {
                    json!({ "method": "local", "error": e.to_string(), "findings": [], "endpoints": [] })
                });
                Ok(
                    json!({ "apk": apk.to_string_lossy().to_string(), "method": "local", "mobsf_error": format!("{up_err:#}"), "analysis": local }),
                )
            }
        }
    } else {
        Ok(json!({
            "apk": apk.to_string_lossy().to_string(),
            "method": "local",
            "analysis": local_fallback_scan(apk)?,
        }))
    }
}

// ═══ Frida bypass-script generation ═══

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FridaScriptKind {
    SslUnpin,
    RootHide,
    SslUnpinIos,
}

impl FridaScriptKind {
    pub fn from_label(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "ssl_unpin" | "ssl" | "pinning" | "unpin" => Some(FridaScriptKind::SslUnpin),
            "root_hide" | "root" | "hide" => Some(FridaScriptKind::RootHide),
            "ios" | "ssl_ios" | "ssl_unpin_ios" => Some(FridaScriptKind::SslUnpinIos),
            _ => None,
        }
    }

    pub fn file_name(&self) -> &'static str {
        match self {
            FridaScriptKind::SslUnpin => "ssl_unpin.js",
            FridaScriptKind::RootHide => "root_hide.js",
            FridaScriptKind::SslUnpinIos => "ssl_unpin_ios.js",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            FridaScriptKind::SslUnpin => "ssl_unpin",
            FridaScriptKind::RootHide => "root_hide",
            FridaScriptKind::SslUnpinIos => "ssl_unpin_ios",
        }
    }
}

/// Universal SSL-pinning bypass (OkHttp / TrustManager / NSURLSession).
pub fn frida_script(kind: FridaScriptKind) -> String {
    match kind {
        FridaScriptKind::SslUnpin | FridaScriptKind::SslUnpinIos => {
            "# Universal SSL-pinning bypass (Sentinel)\n\
             // Android: OkHttp CertificatePinner + TrustManager\n\
             // iOS: NSURLSession SSL pinning == done via challenge handler\n"
                .to_string()
                + r#"Java.perform(function () {
    // Android OkHttp CertificatePinner
    var OkHttp = Java.use("okhttp3.CertificatePinner");
    OkHttp.check.overload("java.lang.String", "[Ljava.security.cert.Certificate;").implementation = function () { return; };
    OkHttp.check.overload("java.lang.String", "java.util.List").implementation = function () { return; };
    // TrustManager successors that trust everything
    var X509 = Java.use("javax.net.ssl.X509TrustManager");
    Java.perform(function () {
        var classes = Java.enumerateLoadedClassesSync();
        classes.forEach(function (c) {
            if (c.indexOf("TrustManager") !== -1) {
                try {
                    var C = Java.use(c);
                    C.checkServerTrusted.implementation = function () { return; };
                } catch (e) {}
            }
        });
    });
    // iOS / native (NSURLSession / SecTrust) when in objective-c context
    var ObjC = undefined;
    try { ObjC = require('frida-objc'); } catch (e) {}
    "#
        }
        .to_string()
                + r##"
    if (typeof ObjC !== "undefined") {
        var handler = ObjC.classes.NSURLSession;
        // generic: hook SecTrustEvaluate to always pass
        var SecTrust = Module.findExportByName(null, "SecTrustEvaluateWithError");
        if (SecTrust) {
            Interceptor.attach(SecTrust, {
                onLeave: function (retval) {
                    retval.replace(ptr(1));
                }
            });
        }
    }
}); // Java.perform
"##,
        FridaScriptKind::RootHide => r#"# Root-detection bypass (Sentinel)
Java.perform(function () {
    var System = Java.use("java.lang.System");
    System.getenv.implementation = function (key) {
        var v = this.getenv(key);
        if (key && key.toLowerCase().indexOf("su") !== -1) return null;
        return v;
    };
    var Runtime = Java.use("java.lang.Runtime");
    Runtime.exec.overload("[Ljava.lang.String;").implementation = function (cmd) {
        var joined = cmd.join(" ");
        if (joined.indexOf("su") !== -1 || joined.indexOf("magisk") !== -1 || joined.indexOf("frida") !== -1) {
            throw Java.use("java.io.IOException").$new("blocked by sentinel root-hide");
        }
        return this.exec(cmd);
    };
    // native libc open("/system/bin/su", ...) -> -1
    var open = Module.findExportByName(null, "open");
    if (open) {
        Interceptor.attach(open, {
            onEnter: function (args) {
                var path = args[0].readCString();
                if (path && (path.indexOf("/su") !== -1 || path.indexOf("magisk") !== -1)) {
                    this.path = path;
                    args[0] = Memory.allocUtf8String("/dev/null");
                }
            }
        });
    }
});"#.to_string(),
    }
}

pub fn write_frida_script(kind: FridaScriptKind, dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let path = dir.join(kind.file_name());
    std::fs::write(&path, frida_script(kind))
        .map_err(|e| anyhow!("write {}: {e}", path.display()))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distill_finds_binary_and_exported() {
        let report = json!({
            "package_name": "com.example.app",
            "app_name": "Example",
            "version_name": "1.0",
            "security_score": 55,
            "binary_analysis": {
                "hardcoded_secrets": ["a", "b", "c"],
                "weak_encryption_algorithm": ["DES"]
            },
            "exported_activities": ["a", "b"],
            "application_debuggable": true,
            "cleartext_traffic": true,
            "permissions": [{"permission": "android.permission.INSTALL_PACKAGES"}],
            "domains": [{"domain": "api.example.com"}, {"url": "https://api.example.com/v1"}]
        });
        let d = distill_report(&report);
        let findings = d["findings"].as_array().unwrap();
        assert!(findings
            .iter()
            .any(|f| f["kind"] == "hardcoded_secrets" && f["severity"] == "high"));
        assert!(findings
            .iter()
            .any(|f| f["kind"] == "debuggable_application" && f["severity"] == "critical"));
        assert!(findings.iter().any(|f| f["kind"] == "cleartext_traffic"));
        assert!(findings.iter().any(|f| f["kind"] == "permission"
            && f["detail"].as_str().unwrap().contains("INSTALL_PACKAGES")));
        assert_eq!(d["endpoints"].as_array().unwrap().len(), 2);
        assert_eq!(d["security_score"], json!(55));
    }

    #[test]
    fn frida_scripts_are_nonempty_and_targeted() {
        assert!(frida_script(FridaScriptKind::SslUnpin).contains("CertificatePinner"));
        assert!(frida_script(FridaScriptKind::SslUnpinIos).contains("SecTrustEvaluate"));
        assert!(frida_script(FridaScriptKind::RootHide).contains("magisk"));
    }

    #[test]
    fn find_apps_recurses() {
        let tmp = std::env::temp_dir().join("sentinel_mobile_test");
        let sub = tmp.join("nested");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(sub.join("app.apk"), b"PK\x03\x04").unwrap();
        std::fs::write(tmp.join("note.txt"), "x").unwrap();
        std::fs::write(tmp.join("signed.ipa"), b"xx").unwrap();
        let found = find_mobile_apps(&tmp);
        assert_eq!(found.len(), 2);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn pull_and_scan_type_detect() {
        assert_eq!(MobsfClient::scan_type(Path::new("a.IPA")), "ipa");
        assert_eq!(MobsfClient::scan_type(Path::new("x.apk")), "apk");
        assert!(
            printable_strings(b"ab  cdxyz123456 !!hello world\n\x00\x01\x02\x03", 6)
                .iter()
                .any(|s| s.contains("hello"))
        );
    }
}
