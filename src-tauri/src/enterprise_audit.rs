//! Enterprise audit trail — append-only JSONL for compliance and support.
//! Events are stored under `<config_dir>/audit.jsonl` (rotated at 10k lines).

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use tauri::State;

use crate::EditorState;

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
pub async fn enterprise_get_policy(state: State<'_, EditorState>) -> Result<EnterprisePolicy, String> {
    Ok(load_policy(&state.config_dir))
}

#[tauri::command]
pub async fn enterprise_set_policy(
    state: State<'_, EditorState>,
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
    state: State<'_, EditorState>,
    limit: Option<usize>,
) -> Result<Vec<AuditEntry>, String> {
    Ok(list_audit(&state.config_dir, limit.unwrap_or(100)))
}

#[tauri::command]
pub async fn enterprise_audit_export(state: State<'_, EditorState>) -> Result<String, String> {
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
    state: State<'_, EditorState>,
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
