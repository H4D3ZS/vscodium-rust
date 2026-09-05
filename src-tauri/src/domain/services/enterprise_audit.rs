//! Enterprise audit trail — append-only JSONL for compliance and support.
//! Events are stored under `<config_dir>/audit.jsonl` (rotated at 10k lines).

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use tauri::State;


const MAX_LINES: usize = 10_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub ts: u64,
    pub actor: String,
    pub action: String,
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnterprisePolicy {
    #[serde(default)]
    pub org_name: String,
    #[serde(default = "default_true")]
    pub audit_enabled: bool,
    #[serde(default)]
    pub require_secure_mode: bool,
    #[serde(default = "default_retention")]
    pub audit_retention_days: u32,
    /// Disallow cloud model providers when true.
    #[serde(default)]
    pub offline_only: bool,
    #[serde(default)]
    pub allowed_models: Vec<String>,
    #[serde(default)]
    pub blocked_models: Vec<String>,
    /// Empty = all MCP servers allowed.
    #[serde(default)]
    pub allowed_mcp_servers: Vec<String>,
    /// Hosts/URLs in scope for offensive tools (e.g. `*.customer.com`, `api.target.io`).
    #[serde(default)]
    pub engagement_targets: Vec<String>,
    #[serde(default)]
    pub engagement_id: String,
    #[serde(default)]
    pub require_engagement_scope: bool,
    #[serde(default = "default_true")]
    pub block_private_network_scan: bool,
    /// Empty = no extra restriction beyond defaults.
    #[serde(default)]
    pub tool_allowlist: Vec<String>,
    #[serde(default)]
    pub tool_denylist: Vec<String>,
    #[serde(default)]
    pub siem_webhook_url: String,
    #[serde(default = "default_true")]
    pub dlp_redact_secrets: bool,
    #[serde(default = "default_true")]
    pub audit_tool_calls: bool,
    #[serde(default = "default_true")]
    pub audit_file_writes: bool,
    #[serde(default = "default_true")]
    pub audit_model_calls: bool,
}

fn default_true() -> bool {
    true
}

fn default_retention() -> u32 {
    90
}

fn policy_path(config_dir: &Path) -> PathBuf {
    config_dir.join("enterprise_policy.json")
}

fn audit_path(config_dir: &Path) -> PathBuf {
    config_dir.join("audit.jsonl")
}

pub fn load_policy(config_dir: &Path) -> EnterprisePolicy {
    std::fs::read_to_string(policy_path(config_dir))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_policy(config_dir: &Path, p: &EnterprisePolicy) -> Result<(), String> {
    std::fs::create_dir_all(config_dir).map_err(|e| e.to_string())?;
    let j = serde_json::to_string_pretty(p).map_err(|e| e.to_string())?;
    std::fs::write(policy_path(config_dir), j).map_err(|e| e.to_string())
}

pub fn append_audit(config_dir: &Path, actor: &str, action: &str, detail: Value) -> Result<(), String> {
    let policy = load_policy(config_dir);
    if !policy.audit_enabled {
        return Ok(());
    }
    std::fs::create_dir_all(config_dir).map_err(|e| e.to_string())?;
    let entry = AuditEntry {
        ts: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        actor: actor.to_string(),
        action: action.to_string(),
        detail,
    };
    let line = serde_json::to_string(&entry).map_err(|e| e.to_string())?;
    let path = audit_path(config_dir);
    {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| e.to_string())?;
        writeln!(f, "{line}").map_err(|e| e.to_string())?;
    }
    rotate_if_needed(&path)?;
    Ok(())
}

fn rotate_if_needed(path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let count = BufReader::new(file).lines().count();
    if count <= MAX_LINES {
        return Ok(());
    }
    let keep = MAX_LINES.saturating_sub(2000);
    let file = File::open(path).map_err(|e| e.to_string())?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect();
    let tail = lines.into_iter().skip(count.saturating_sub(keep));
    let mut out = String::new();
    for l in tail {
        out.push_str(&l);
        out.push('\n');
    }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

pub fn list_audit(config_dir: &Path, limit: usize) -> Vec<AuditEntry> {
    let path = audit_path(config_dir);
    let Ok(file) = File::open(path) else {
        return Vec::new();
    };
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .collect();
    lines
        .into_iter()
        .rev()
        .take(limit.min(500))
        .filter_map(|l| serde_json::from_str(&l).ok())
        .collect()
}

#[tauri::command]
pub async fn enterprise_get_policy(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<EnterprisePolicy, String> {
    Ok(load_policy(&state.config_dir))
}

#[tauri::command]
pub async fn enterprise_set_policy(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    policy: EnterprisePolicy,
) -> Result<(), String> {
    save_policy(&state.config_dir, &policy)?;
    let _ = append_audit(
        &state.config_dir,
        "admin",
        "enterprise.policy_update",
        json!({ "org_name": policy.org_name, "require_secure_mode": policy.require_secure_mode }),
    );
    Ok(())
}

#[tauri::command]
pub async fn enterprise_audit_list(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    limit: Option<usize>,
) -> Result<Vec<AuditEntry>, String> {
    Ok(list_audit(&state.config_dir, limit.unwrap_or(100)))
}

#[tauri::command]
pub async fn enterprise_audit_export(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<String, String> {
    let src = audit_path(&state.config_dir);
    if !src.exists() {
        return Err("No audit log yet.".into());
    }
    let dest = state
        .config_dir
        .join(format!("audit_export_{}.jsonl", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    std::fs::copy(&src, &dest).map_err(|e| e.to_string())?;
    let _ = append_audit(
        &state.config_dir,
        "admin",
        "enterprise.audit_export",
        json!({ "path": dest.to_string_lossy() }),
    );
    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn enterprise_audit_log(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    action: String,
    detail: Option<Value>,
) -> Result<(), String> {
    append_audit(
        &state.config_dir,
        "user",
        &action,
        detail.unwrap_or(json!({})),
    )
}

/// Seed cyber-enterprise defaults (audit, DLP, private-network block).
#[tauri::command]
pub async fn enterprise_seed_cyber_policy(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    org_name: Option<String>,
) -> Result<EnterprisePolicy, String> {
    let mut policy = crate::enterprise_governance::default_cyber_enterprise_policy(
        org_name.as_deref().unwrap_or("Security Team"),
    );
    if let Some(existing) = std::fs::read_to_string(policy_path(&state.config_dir))
        .ok()
        .and_then(|s| serde_json::from_str::<EnterprisePolicy>(&s).ok())
    {
        if !existing.org_name.is_empty() {
            policy.org_name = existing.org_name;
        }
        if !existing.engagement_targets.is_empty() {
            policy.engagement_targets = existing.engagement_targets;
        }
        if !existing.engagement_id.is_empty() {
            policy.engagement_id = existing.engagement_id;
        }
    }
    save_policy(&state.config_dir, &policy)?;
    let _ = append_audit(
        &state.config_dir,
        "admin",
        "enterprise.policy_seed",
        json!({ "org": policy.org_name }),
    );
    Ok(policy)
}

/// Initialize engagement folder layout under workspace root.
#[tauri::command]
pub async fn enterprise_init_engagement(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    engagement_id: String,
    targets: Vec<String>,
) -> Result<Value, String> {
    let root = state.editor.active_root.lock().await.clone();
    let root = root.ok_or_else(|| "Open a workspace folder first".to_string())?;
    let id = engagement_id
        .trim()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect::<String>();
    if id.is_empty() {
        return Err("engagement_id required".into());
    }
    for sub in [
        format!("reports/{id}"),
        format!("recon/{id}"),
        format!("exploits/{id}"),
        format!("payloads/{id}"),
    ] {
        let p = root.join(&sub);
        std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    }
    let mut policy = load_policy(&state.config_dir);
    policy.engagement_id = id.clone();
    policy.engagement_targets = targets.clone();
    policy.require_engagement_scope = true;
    save_policy(&state.config_dir, &policy)?;
    let _ = append_audit(
        &state.config_dir,
        "user",
        "engagement.init",
        json!({ "id": id, "targets": targets, "root": root }),
    );
    Ok(json!({
        "engagement_id": id,
        "targets": targets,
        "paths": ["reports", "recon", "exploits", "payloads"]
    }))
}

/// Export findings under `reports/` as SARIF 2.1.0 JSON.
#[tauri::command]
pub async fn enterprise_export_sarif(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<String, String> {
    let root = state.editor.active_root.lock().await.clone();
    let root = root.unwrap_or_else(|| state.config_dir.clone());
    let reports = root.join("reports");
    let mut results = Vec::new();
    if reports.is_dir() {
        collect_report_findings(&reports, &mut results);
    }
    let sarif = json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "HADES IDE",
                    "informationUri": "https://github.com/H4D3ZS/vscodium-rust-ide-saas",
                    "version": env!("CARGO_PKG_VERSION"),
                }
            },
            "results": results,
        }]
    });
    let dest = state.config_dir.join(format!(
        "findings_export_{}.sarif.json",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    ));
    std::fs::write(&dest, serde_json::to_string_pretty(&sarif).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let _ = append_audit(
        &state.config_dir,
        "user",
        "enterprise.sarif_export",
        json!({ "path": dest, "count": results.len() }),
    );
    Ok(dest.to_string_lossy().to_string())
}

fn collect_report_findings(dir: &Path, out: &mut Vec<Value>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_report_findings(&path, out);
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "md" && ext != "json" {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        let rule_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("finding")
            .to_string();
        let message = content.lines().take(12).collect::<Vec<_>>().join("\n");
        out.push(json!({
            "ruleId": rule_id,
            "level": "warning",
            "message": { "text": message },
            "locations": [{
                "physicalLocation": {
                    "artifactLocation": { "uri": path.to_string_lossy() }
                }
            }]
        }));
    }
}
