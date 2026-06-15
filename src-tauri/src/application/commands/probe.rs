//! Tauri commands for the multi-key API probe engine (brutecat methodology).
//!
//! All commands are MCP-callable: the AI in the IDE can use these as tools during
//! an authorized pentest / bug-bounty session — exactly mirroring the workflow from
//! the brutecat article where Claude received `probe_api`, `report_vulnerability`,
//! `get_endpoint_context`, and `confirm_testing_complete` as MCP tools.

use tauri::State;

use crate::{
    domain::security::{
        finding_ledger::{
            EndpointGroup, FindingLedger, FindingLedgerHandle, LedgerStats, SecurityFinding,
            Severity, GroupContext, CompletionReport, OperationRecord,
        },
        probe_engine::{ApiKey, ProbeEngine, ProbeGroup, ProbeRequest, KNOWN_VISIBILITY_LABELS},
    },
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// ── Engine singleton ──────────────────────────────────────────────────────────

fn engine() -> &'static ProbeEngine {
    static ENGINE: OnceLock<ProbeEngine> = OnceLock::new();
    ENGINE.get_or_init(ProbeEngine::new)
}

// ── Request / response DTOs ───────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionArgs {
    pub target_base_url: String,
    pub groups: Vec<EndpointGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeApiArgs {
    pub session_id: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
    /// Which API keys (from the session) to use. Empty = all keys.
    #[serde(default)]
    pub key_labels: Vec<String>,
    #[serde(default)]
    pub visibility_label: Option<String>,
    #[serde(default)]
    pub follow_redirects: bool,
    /// Optional group + path to mark as tested.
    #[serde(default)]
    pub group_name: Option<String>,
    #[serde(default)]
    pub endpoint_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProbeApiResult {
    pub group: ProbeGroup,
    /// op_ids recorded (one per key that returned a distinct hash group representative).
    pub op_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportVulnerabilityArgs {
    pub session_id: String,
    pub title: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
    pub op_ids: Vec<String>,
    #[serde(default)]
    pub mitre_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddKeysArgs {
    pub session_id: String,
    pub keys: Vec<ApiKey>,
}

/// Per-session key store (separate from the ledger so keys are mutable mid-session).
type KeyStore = std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, Vec<ApiKey>>>>;

fn key_store() -> &'static KeyStore {
    static KS: OnceLock<KeyStore> = OnceLock::new();
    KS.get_or_init(|| {
        std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()))
    })
}

// ── Tauri commands ────────────────────────────────────────────────────────────

/// Create a new probe session with a target base URL and endpoint groups.
/// Returns the session_id used in all subsequent calls.
#[tauri::command]
pub async fn probe_create_session(
    ledger: State<'_, FindingLedgerHandle>,
    args: CreateSessionArgs,
) -> Result<String, String> {
    let sid = ledger.lock().await.create_session(args.target_base_url, args.groups);
    Ok(sid)
}

/// Add API keys to a session. Keys accumulate — call multiple times to add batches.
#[tauri::command]
pub async fn probe_add_keys(args: AddKeysArgs) -> Result<usize, String> {
    let mut ks = key_store().lock().await;
    let entry = ks.entry(args.session_id).or_default();
    entry.extend(args.keys);
    Ok(entry.len())
}

/// Core tool — fire a request across all (or selected) session keys.
/// Groups responses by hash. Records each unique probe as an operation.
///
/// This is the MCP `probe_api()` equivalent from the brutecat methodology.
#[tauri::command]
pub async fn probe_api(
    ledger: State<'_, FindingLedgerHandle>,
    args: ProbeApiArgs,
) -> Result<ProbeApiResult, String> {
    let ks = key_store().lock().await;
    let all_keys = ks.get(&args.session_id).cloned().unwrap_or_default();
    drop(ks);

    // Filter to requested key labels, or use all.
    let keys: Vec<ApiKey> = if args.key_labels.is_empty() {
        all_keys
    } else {
        all_keys.into_iter().filter(|k| args.key_labels.contains(&k.label)).collect()
    };

    let req = ProbeRequest {
        method: args.method.clone(),
        url: args.url.clone(),
        headers: args.headers,
        body: args.body.clone(),
        visibility_label: args.visibility_label,
        follow_redirects: args.follow_redirects,
    };

    // Use the session_id as the op_id prefix so IDs are traceable.
    let op_prefix = args.session_id.clone();
    let group = engine().probe_all(req, &keys, &op_prefix).await;

    // Record one operation per hash group (representative sample).
    let mut op_ids = Vec::new();
    let mut ledger_lock = ledger.lock().await;
    for entry in group.by_hash.values() {
        let op_id = ledger_lock.record_operation(
            &args.session_id,
            &args.method,
            &args.url,
            &args.body,
            entry.sample.status,
            &entry.sample.body_hash,
            &entry.sample.body,
            &entry.sample.key_label,
            entry.sample.duration_ms,
            args.group_name.as_deref(),
            args.endpoint_path.as_deref(),
        );
        op_ids.push(op_id);
    }
    drop(ledger_lock);

    Ok(ProbeApiResult { group, op_ids })
}

/// Probe a single endpoint across all collected visibility labels.
/// Returns a map of label → ProbeApiResult, letting the AI identify which
/// labels unlock hidden methods.
#[tauri::command]
pub async fn probe_visibility_labels(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
    method: String,
    url: String,
    #[allow(unused)] body: String,
    group_name: Option<String>,
    endpoint_path: Option<String>,
) -> Result<std::collections::HashMap<String, ProbeApiResult>, String> {
    let mut results = std::collections::HashMap::new();
    for &label in KNOWN_VISIBILITY_LABELS {
        let args = ProbeApiArgs {
            session_id: session_id.clone(),
            method: method.clone(),
            url: url.clone(),
            headers: vec![],
            body: body.clone(),
            key_labels: vec![],
            visibility_label: Some(label.to_string()),
            follow_redirects: false,
            group_name: group_name.clone(),
            endpoint_path: endpoint_path.clone(),
        };
        // Re-use the inner helper logic inline to avoid borrow issues.
        let ks = key_store().lock().await;
        let all_keys = ks.get(&session_id).cloned().unwrap_or_default();
        drop(ks);
        let req = ProbeRequest {
            method: method.clone(),
            url: url.clone(),
            headers: vec![],
            body: body.clone(),
            visibility_label: Some(label.to_string()),
            follow_redirects: false,
        };
        let op_prefix = format!("{session_id}-vis-{}", label.to_lowercase());
        let group = engine().probe_all(req, &all_keys, &op_prefix).await;
        let mut op_ids = Vec::new();
        let mut ledger_lock = ledger.lock().await;
        for entry in group.by_hash.values() {
            let op_id = ledger_lock.record_operation(
                &session_id, &args.method, &args.url, &args.body,
                entry.sample.status, &entry.sample.body_hash,
                &entry.sample.body, &entry.sample.key_label,
                entry.sample.duration_ms,
                group_name.as_deref(), endpoint_path.as_deref(),
            );
            op_ids.push(op_id);
        }
        drop(ledger_lock);
        results.insert(label.to_string(), ProbeApiResult { group, op_ids });
    }
    Ok(results)
}

/// Report a confirmed vulnerability with evidence op_ids.
/// This is the MCP `report_vulnerability()` equivalent.
#[tauri::command]
pub async fn probe_report_vulnerability(
    ledger: State<'_, FindingLedgerHandle>,
    args: ReportVulnerabilityArgs,
) -> Result<String, String> {
    let severity = match args.severity.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        _ => Severity::Info,
    };
    let find_id = ledger.lock().await.report_finding(
        &args.session_id,
        args.title,
        severity,
        args.description,
        args.recommendation,
        args.op_ids,
        args.mitre_id,
    );
    Ok(find_id)
}

/// Mark a finding as analyst-verified (replayed and confirmed reproducible).
#[tauri::command]
pub async fn probe_verify_finding(
    ledger: State<'_, FindingLedgerHandle>,
    find_id: String,
) -> Result<bool, String> {
    Ok(ledger.lock().await.verify_finding(&find_id))
}

/// Return the full context string for the AI before it tests a group.
/// Includes: prior findings summaries, discovered IDs, patterns.
/// This is the MCP `get_endpoint_context()` equivalent.
#[tauri::command]
pub async fn probe_get_endpoint_context(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
    group_name: String,
) -> Result<GroupContext, String> {
    Ok(ledger.lock().await.get_group_context(&session_id, &group_name))
}

/// Validate that all in-scope endpoints in the session have been probed.
/// Returns Ok(report) if complete, Err(report) listing untested endpoints if not.
/// This is the MCP `confirm_testing_complete()` equivalent — prevents the AI
/// from declaring a group done prematurely.
#[tauri::command]
pub async fn probe_confirm_complete(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
) -> Result<CompletionReport, String> {
    match ledger.lock().await.confirm_testing_complete(&session_id) {
        Ok(report) => Ok(report),
        Err(report) => {
            // Surface untested as a structured error so the AI knows what's missing.
            Err(serde_json::to_string(&report).unwrap_or_else(|_| format!(
                "testing incomplete: {} endpoints untested",
                report.untested.len()
            )))
        }
    }
}

/// Replay a specific operation by op_id — returns the original request+response record.
/// Frontend "Play" button hits this to re-fire (note: re-fire goes through probe_api).
#[tauri::command]
pub async fn probe_get_operation(
    ledger: State<'_, FindingLedgerHandle>,
    op_id: String,
) -> Result<OperationRecord, String> {
    ledger
        .lock()
        .await
        .get_operation(&op_id)
        .cloned()
        .ok_or_else(|| format!("op_id '{op_id}' not found"))
}

/// Return all operations for a session (for the frontend timeline view).
#[tauri::command]
pub async fn probe_session_operations(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
) -> Result<Vec<OperationRecord>, String> {
    Ok(ledger.lock().await.session_operations(&session_id).into_iter().cloned().collect())
}

/// Return all confirmed findings for a session.
#[tauri::command]
pub async fn probe_session_findings(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
) -> Result<Vec<SecurityFinding>, String> {
    Ok(ledger.lock().await.session_findings(&session_id).into_iter().cloned().collect())
}

/// High-level session stats.
#[tauri::command]
pub async fn probe_session_stats(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
) -> Result<LedgerStats, String> {
    Ok(ledger.lock().await.stats(&session_id))
}

/// Add a discovered ID/token to the session context for carry-forward.
#[tauri::command]
pub async fn probe_add_discovered(
    ledger: State<'_, FindingLedgerHandle>,
    session_id: String,
    item: String,
) -> Result<(), String> {
    ledger.lock().await.add_discovered(&session_id, item);
    Ok(())
}

/// List all sessions.
#[tauri::command]
pub async fn probe_list_sessions(
    ledger: State<'_, FindingLedgerHandle>,
) -> Result<Vec<String>, String> {
    Ok(ledger.lock().await.list_sessions().into_iter().map(|s| s.session_id.clone()).collect())
}
