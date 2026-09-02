//! Finding ledger — brutecat methodology operation ID + provenance system.
//!
//! Every probe is assigned an `op_id`. When the AI (or human) reports a finding,
//! it cites the op_ids that produced the evidence. The ledger stores the full
//! request+response for each operation so analysts can one-click-replay any finding.
//!
//! Key innovations:
//! - `confirm_testing_complete()` blocks the AI from marking a group done until
//!   all in-scope endpoints received at least one probe.
//! - Context carry-forward: `get_group_context()` assembles a compact summary of
//!   prior findings and discovered IDs for the AI's next group pass.
//! - Drift-log: lightweight key/value accumulation between cycles (mirrors the
//!   asymmetric_orchestrator pattern but focused on pentest context).
//!
//! Authorized testing / bug bounty use only.

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

// ── Severity / tier ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "CRITICAL"),
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Low => write!(f, "LOW"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

// ── Operation record ──────────────────────────────────────────────────────────

/// Immutable record of a single probe. Stored forever (bounded ring).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub op_id: String,
    pub session_id: String,
    pub timestamp_ns: u64,
    /// Method + URL fired.
    pub method: String,
    pub url: String,
    /// Request body (≤1 KB stored).
    pub request_body: String,
    /// HTTP status returned.
    pub status: u16,
    /// Response body hash (SHA-256 hex).
    pub body_hash: String,
    /// Response body snippet (≤2 KB).
    pub body_snippet: String,
    /// Key label used for this specific probe.
    pub key_label: String,
    pub duration_ms: u64,
}

// ── Finding ───────────────────────────────────────────────────────────────────

/// A confirmed security finding with full provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub find_id: String,
    pub session_id: String,
    pub timestamp_ns: u64,
    pub title: String,
    pub severity: Severity,
    pub description: String,
    pub recommendation: String,
    /// op_ids that produced the evidence for this finding.
    pub op_ids: Vec<String>,
    /// Analyst confirmed the finding is reproducible.
    pub verified: bool,
    /// MITRE ATT&CK technique ID if applicable (e.g. "T1078").
    pub mitre_id: Option<String>,
}

// ── Endpoint / group definitions ──────────────────────────────────────────────

/// A single testable API endpoint within a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointSpec {
    pub method: String,
    pub path: String,
    /// Optional JSON schema for constructing payloads.
    pub schema: Option<serde_json::Value>,
}

/// A functional group of related endpoints (mirrors brutecat's group-based strategy).
/// Testing is not considered complete until all in-scope endpoints have been probed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointGroup {
    pub name: String,
    pub description: String,
    pub attack_vectors: Vec<String>,
    pub endpoints: Vec<EndpointSpec>,
    /// Paths the AI should skip probing (retrieves schema via `get_endpoint_context` instead).
    pub out_of_scope: Vec<String>,
}

// ── Test session ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSession {
    pub session_id: String,
    pub target_base_url: String,
    pub groups: Vec<EndpointGroup>,
    pub timestamp_ns: u64,
    /// All (group_name, method, path) tuples that have been probed at least once.
    pub tested_endpoints: HashSet<String>,
    /// Discoveries surfaced during this session (IDs, tokens, patterns).
    pub discovered_context: Vec<String>,
}

impl TestSession {
    pub fn mark_tested(&mut self, group: &str, method: &str, path: &str) {
        self.tested_endpoints.insert(format!("{group}:{method}:{path}"));
    }

    pub fn is_endpoint_tested(&self, group: &str, method: &str, path: &str) -> bool {
        self.tested_endpoints.contains(&format!("{group}:{method}:{path}"))
    }

    /// All in-scope endpoints that have not yet been probed.
    pub fn untested_endpoints(&self) -> Vec<(String, String, String)> {
        let mut missing = Vec::new();
        for g in &self.groups {
            for ep in &g.endpoints {
                if g.out_of_scope.contains(&ep.path) {
                    continue;
                }
                if !self.is_endpoint_tested(&g.name, &ep.method, &ep.path) {
                    missing.push((g.name.clone(), ep.method.clone(), ep.path.clone()));
                }
            }
        }
        missing
    }
}

// ── Completion report ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionReport {
    pub session_id: String,
    pub complete: bool,
    /// If not complete, lists the untested endpoints.
    pub untested: Vec<(String, String, String)>,
    pub total_endpoints: usize,
    pub tested_count: usize,
    pub finding_count: usize,
}

// ── Context summary (carry-forward) ──────────────────────────────────────────

/// Compact context string assembled from prior findings for AI context injection.
/// Passed as `prior_findings` to subsequent group test passes to enable escalation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupContext {
    pub group_name: String,
    /// Plain-text summary the AI receives as context.
    pub summary: String,
    /// Discovered IDs / tokens surfaced so far (useful for chaining).
    pub discovered_ids: Vec<String>,
    /// Findings already confirmed in this group.
    pub prior_findings: Vec<SummaryFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryFinding {
    pub find_id: String,
    pub severity: String,
    pub title: String,
    pub description_snippet: String,
}

// ── Ledger inner ──────────────────────────────────────────────────────────────

const MAX_OPERATIONS: usize = 10_000;

struct LedgerInner {
    sessions: HashMap<String, TestSession>,
    /// Ring buffer of operations (all sessions).
    operations: VecDeque<OperationRecord>,
    findings: Vec<SecurityFinding>,
    op_counter: u64,
    find_counter: u64,
    session_counter: u64,
}

impl LedgerInner {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            operations: VecDeque::with_capacity(MAX_OPERATIONS),
            findings: Vec::new(),
            op_counter: 0,
            find_counter: 0,
            session_counter: 0,
        }
    }

    fn next_op_id(&mut self) -> String {
        self.op_counter += 1;
        format!("op_{:04}", self.op_counter)
    }

    fn next_find_id(&mut self) -> String {
        self.find_counter += 1;
        format!("find_{:04}", self.find_counter)
    }

    fn now_ns() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_nanos() as u64
    }
}

// ── Public handle ─────────────────────────────────────────────────────────────

pub type FindingLedgerHandle = Arc<Mutex<FindingLedger>>;

pub struct FindingLedger(LedgerInner);

impl FindingLedger {
    pub fn new() -> FindingLedgerHandle {
        Arc::new(Mutex::new(Self(LedgerInner::new())))
    }

    // ── Session management ──

    pub fn create_session(
        &mut self,
        target_base_url: String,
        groups: Vec<EndpointGroup>,
    ) -> String {
        let inner = &mut self.0;
        inner.session_counter += 1;
        let session_id = format!("sess_{:04}", inner.session_counter);
        inner.sessions.insert(
            session_id.clone(),
            TestSession {
                session_id: session_id.clone(),
                target_base_url,
                groups,
                timestamp_ns: LedgerInner::now_ns(),
                tested_endpoints: HashSet::new(),
                discovered_context: Vec::new(),
            },
        );
        session_id
    }

    pub fn get_session(&self, session_id: &str) -> Option<&TestSession> {
        self.0.sessions.get(session_id)
    }

    pub fn list_sessions(&self) -> Vec<&TestSession> {
        self.0.sessions.values().collect()
    }

    /// Add a discovered ID/token to the session context (carry-forward).
    pub fn add_discovered(&mut self, session_id: &str, item: String) {
        if let Some(s) = self.0.sessions.get_mut(session_id) {
            s.discovered_context.push(item);
        }
    }

    // ── Operation recording ──

    /// Record a probe result and return its assigned op_id.
    pub fn record_operation(
        &mut self,
        session_id: &str,
        method: &str,
        url: &str,
        request_body: &str,
        status: u16,
        body_hash: &str,
        body_snippet: &str,
        key_label: &str,
        duration_ms: u64,
        group_name: Option<&str>,
        path: Option<&str>,
    ) -> String {
        let op_id = self.0.next_op_id();
        let op = OperationRecord {
            op_id: op_id.clone(),
            session_id: session_id.to_string(),
            timestamp_ns: LedgerInner::now_ns(),
            method: method.to_string(),
            url: url.to_string(),
            request_body: request_body.chars().take(1024).collect(),
            status,
            body_hash: body_hash.to_string(),
            body_snippet: body_snippet.chars().take(2048).collect(),
            key_label: key_label.to_string(),
            duration_ms,
        };
        if self.0.operations.len() >= MAX_OPERATIONS {
            self.0.operations.pop_front();
        }
        self.0.operations.push_back(op);

        // Mark endpoint tested.
        if let (Some(g), Some(p)) = (group_name, path) {
            if let Some(s) = self.0.sessions.get_mut(session_id) {
                s.mark_tested(g, method, p);
            }
        }
        op_id
    }

    pub fn get_operation(&self, op_id: &str) -> Option<&OperationRecord> {
        self.0.operations.iter().find(|op| op.op_id == op_id)
    }

    pub fn session_operations(&self, session_id: &str) -> Vec<&OperationRecord> {
        self.0.operations.iter().filter(|op| op.session_id == session_id).collect()
    }

    // ── Finding management ──

    pub fn report_finding(
        &mut self,
        session_id: &str,
        title: String,
        severity: Severity,
        description: String,
        recommendation: String,
        op_ids: Vec<String>,
        mitre_id: Option<String>,
    ) -> String {
        let find_id = self.0.next_find_id();
        self.0.findings.push(SecurityFinding {
            find_id: find_id.clone(),
            session_id: session_id.to_string(),
            timestamp_ns: LedgerInner::now_ns(),
            title,
            severity,
            description,
            recommendation,
            op_ids,
            verified: false,
            mitre_id,
        });
        find_id
    }

    pub fn verify_finding(&mut self, find_id: &str) -> bool {
        if let Some(f) = self.0.findings.iter_mut().find(|f| f.find_id == find_id) {
            f.verified = true;
            true
        } else {
            false
        }
    }

    pub fn session_findings(&self, session_id: &str) -> Vec<&SecurityFinding> {
        self.0.findings.iter().filter(|f| f.session_id == session_id).collect()
    }

    // ── confirm_testing_complete ──

    /// Returns `Ok(CompletionReport)` if all in-scope endpoints have been probed.
    /// Returns `Err` if any remain untested, listing them so the AI knows what to do next.
    pub fn confirm_testing_complete(&self, session_id: &str) -> Result<CompletionReport, CompletionReport> {
        let session = match self.0.sessions.get(session_id) {
            Some(s) => s,
            None => {
                return Err(CompletionReport {
                    session_id: session_id.to_string(),
                    complete: false,
                    untested: vec![],
                    total_endpoints: 0,
                    tested_count: 0,
                    finding_count: 0,
                });
            }
        };

        let total: usize = session
            .groups
            .iter()
            .map(|g| g.endpoints.iter().filter(|ep| !g.out_of_scope.contains(&ep.path)).count())
            .sum();

        let untested = session.untested_endpoints();
        let tested_count = total.saturating_sub(untested.len());
        let finding_count = self.0.findings.iter().filter(|f| f.session_id == session_id).count();

        let report = CompletionReport {
            session_id: session_id.to_string(),
            complete: untested.is_empty(),
            untested,
            total_endpoints: total,
            tested_count,
            finding_count,
        };

        if report.complete {
            Ok(report)
        } else {
            Err(report)
        }
    }

    // ── Context carry-forward ──

    /// Assembles a context string for the AI before testing a new group.
    /// Includes: prior findings summaries, discovered IDs, patterns noticed.
    pub fn get_group_context(&self, session_id: &str, group_name: &str) -> GroupContext {
        let session = self.0.sessions.get(session_id);
        let discovered_ids = session
            .map(|s| s.discovered_context.clone())
            .unwrap_or_default();

        let prior_findings: Vec<SummaryFinding> = self
            .0
            .findings
            .iter()
            .filter(|f| f.session_id == session_id)
            .map(|f| SummaryFinding {
                find_id: f.find_id.clone(),
                severity: f.severity.to_string(),
                title: f.title.clone(),
                description_snippet: f.description.chars().take(300).collect(),
            })
            .collect();

        let summary = if prior_findings.is_empty() && discovered_ids.is_empty() {
            format!("Beginning group '{group_name}'. No prior context.")
        } else {
            let finding_lines: Vec<String> = prior_findings
                .iter()
                .map(|f| format!("  [{sev}] {id}: {title}", sev = f.severity, id = f.find_id, title = f.title))
                .collect();
            let id_lines = discovered_ids.join(", ");
            format!(
                "Group '{group_name}' context from prior passes:\n\
                 FINDINGS ({n}):\n{findings}\n\
                 DISCOVERED IDs/TOKENS: {ids}",
                n = prior_findings.len(),
                findings = finding_lines.join("\n"),
                ids = if id_lines.is_empty() { "none".into() } else { id_lines },
            )
        };

        GroupContext {
            group_name: group_name.to_string(),
            summary,
            discovered_ids,
            prior_findings,
        }
    }

    // ── Stats ──

    pub fn stats(&self, session_id: &str) -> LedgerStats {
        let op_count = self.0.operations.iter().filter(|op| op.session_id == session_id).count();
        let find_count = self.0.findings.iter().filter(|f| f.session_id == session_id).count();
        let verified_count = self.0.findings.iter().filter(|f| f.session_id == session_id && f.verified).count();
        LedgerStats { op_count, find_count, verified_count }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerStats {
    pub op_count: usize,
    pub find_count: usize,
    pub verified_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn confirm_complete_blocks_until_all_probed() {
        let handle = FindingLedger::new();
        let mut ledger = handle.lock().await;

        let groups = vec![EndpointGroup {
            name: "auth".into(),
            description: "Auth endpoints".into(),
            attack_vectors: vec![],
            endpoints: vec![
                EndpointSpec { method: "GET".into(), path: "/v1/token".into(), schema: None },
                EndpointSpec { method: "POST".into(), path: "/v1/token".into(), schema: None },
            ],
            out_of_scope: vec![],
        }];

        let sid = ledger.create_session("https://api.example.com".into(), groups);

        // Not complete yet.
        assert!(ledger.confirm_testing_complete(&sid).is_err());

        // Probe one.
        ledger.record_operation(&sid, "GET", "/v1/token", "", 200, "abc", "body", "key0", 10, Some("auth"), Some("/v1/token"));
        assert!(ledger.confirm_testing_complete(&sid).is_err());

        // Probe both.
        ledger.record_operation(&sid, "POST", "/v1/token", "{}", 200, "def", "body", "key0", 10, Some("auth"), Some("/v1/token"));
        assert!(ledger.confirm_testing_complete(&sid).is_ok());
    }

    #[tokio::test]
    async fn context_carry_forward() {
        let handle = FindingLedger::new();
        let mut ledger = handle.lock().await;
        let sid = ledger.create_session("https://api.example.com".into(), vec![]);

        ledger.add_discovered(&sid, "user_id=12345".into());
        ledger.report_finding(&sid, "IDOR".into(), Severity::High, "Can read other users".into(), "Add auth check".into(), vec!["op_0001".into()], None);

        let ctx = ledger.get_group_context(&sid, "billing");
        assert!(ctx.summary.contains("IDOR"));
        assert!(ctx.summary.contains("user_id=12345"));
    }
}
