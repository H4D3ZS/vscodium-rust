//! Live correlation engine.
//!
//! Ingests three simultaneous streams — static findings, live network traffic,
//! live storage writes — and cross-references them into scored findings. The
//! value is *correlation, not collection*: it converts "a hardcoded secret was
//! found" into "the hardcoded secret was observed in a live outbound request",
//! which is the difference between a plausible issue and a confirmed one.
//!
//! The engine is pure in-memory analysis over event streams the (authorized)
//! acquisition layer produces; it holds no networking or device state itself.
//!
//! Authorized testing / bug bounty use only.

use std::collections::HashMap;

use crate::domain::correlation::model::*;
use crate::domain::fingerprint::model::ConfidenceTier;

/// Minimum length for a value to be treated as a correlatable secret (avoids
/// matching trivial strings like "1" or "id").
const MIN_SECRET_LEN: usize = 6;

pub struct CorrelationEngine {
    secrets: Vec<StaticFinding>,
    privileged_endpoints: Vec<StaticFinding>,
    findings: Vec<Finding>,
    /// dedup key -> index into `findings`.
    dedup: HashMap<String, usize>,
    /// Authorization-bearing tokens observed in traffic (for persistence check).
    captured_tokens: Vec<String>,
    /// Number of findings present at the last `mark_baseline` call.
    baseline_len: usize,
}

impl Default for CorrelationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CorrelationEngine {
    pub fn new() -> Self {
        Self {
            secrets: Vec::new(),
            privileged_endpoints: Vec::new(),
            findings: Vec::new(),
            dedup: HashMap::new(),
            captured_tokens: Vec::new(),
            baseline_len: 0,
        }
    }

    fn anchor_of(kind: CorrelationKind, value: &str) -> String {
        format!("{kind:?}:{value}")
    }

    fn emit(&mut self, kind: CorrelationKind, anchor_value: &str, mut finding: Finding) {
        finding.score = Finding::compute_score(finding.severity, finding.confidence);
        let anchor = Self::anchor_of(kind, anchor_value);
        if let Some(&idx) = self.dedup.get(&anchor) {
            if let Some(existing) = self.findings.get_mut(idx) {
                existing.occurrences += 1;
            }
            return;
        }
        self.dedup.insert(anchor, self.findings.len());
        self.findings.push(finding);
    }

    pub fn ingest_static(&mut self, f: StaticFinding) {
        match f.kind {
            StaticKind::HardcodedSecret | StaticKind::PrivateKey => self.secrets.push(f),
            StaticKind::DangerousEndpoint => self.privileged_endpoints.push(f),
            StaticKind::WeakEntitlement => {
                // Informational on its own; no cross-stream correlation yet.
                let id = format!("static-{}", uuid::Uuid::new_v4());
                self.emit(
                    CorrelationKind::RuntimeSecretCreated,
                    &f.value,
                    Finding {
                        id,
                        kind: CorrelationKind::RuntimeSecretCreated,
                        title: "Broad entitlement detected".into(),
                        summary: format!("Entitlement grants: {}", f.value),
                        severity: Severity::Low,
                        confidence: ConfidenceTier::Speculative,
                        evidence_event_ids: vec![f.id.clone()],
                        first_seen_ms: 0,
                        occurrences: 1,
                        score: 0.0,
                    },
                );
            }
        }
    }

    pub fn ingest_network(&mut self, e: NetworkEvent) {
        let needle_pool = [e.url.as_str(), e.body.as_str()]
            .into_iter()
            .chain(e.headers.iter().map(|(_, v)| v.as_str()))
            .collect::<Vec<_>>();

        // Secret-in-traffic: a static secret observed in a live request.
        let matched_secrets: Vec<StaticFinding> = self
            .secrets
            .iter()
            .filter(|s| {
                s.value.len() >= MIN_SECRET_LEN && needle_pool.iter().any(|n| n.contains(&s.value))
            })
            .cloned()
            .collect();
        for secret in matched_secrets {
            self.emit(
                CorrelationKind::SecretUsedLive,
                &secret.value,
                Finding {
                    id: format!("f-{}", uuid::Uuid::new_v4()),
                    kind: CorrelationKind::SecretUsedLive,
                    title: "Confirmed live use of hardcoded secret".into(),
                    summary: format!(
                        "Secret from {} appears in {} {}. Evidence: {}",
                        secret.location, e.method, e.url, secret.value
                    ),
                    severity: Severity::High,
                    confidence: ConfidenceTier::Confirmed,
                    evidence_event_ids: vec![secret.id.clone(), e.id.clone()],
                    first_seen_ms: e.timestamp_ms,
                    occurrences: 1,
                    score: 0.0,
                },
            );
        }

        // Privileged endpoint reached live.
        let matched_endpoints: Vec<StaticFinding> = self
            .privileged_endpoints
            .iter()
            .filter(|pe| e.url.contains(&pe.value))
            .cloned()
            .collect();
        for pe in matched_endpoints {
            self.emit(
                CorrelationKind::PrivilegedEndpointReached,
                &pe.value,
                Finding {
                    id: format!("f-{}", uuid::Uuid::new_v4()),
                    kind: CorrelationKind::PrivilegedEndpointReached,
                    title: "Privileged endpoint reached live".into(),
                    summary: format!("Static-flagged endpoint hit: {} {}", e.method, e.url),
                    severity: Severity::Medium,
                    confidence: ConfidenceTier::Probable,
                    evidence_event_ids: vec![pe.id.clone(), e.id.clone()],
                    first_seen_ms: e.timestamp_ms,
                    occurrences: 1,
                    score: 0.0,
                },
            );
        }

        // Capture authorization tokens for the persistence correlation.
        for (name, value) in &e.headers {
            if name.eq_ignore_ascii_case("authorization")
                && (value.starts_with("Bearer ") || value.starts_with("bearer "))
            {
                let token = value
                    .split_once(' ')
                    .map(|(_, t)| t.trim().to_string())
                    .unwrap_or_default();
                if token.len() >= MIN_SECRET_LEN && !self.captured_tokens.contains(&token) {
                    self.captured_tokens.push(token);
                }
            }
        }
    }

    pub fn ingest_storage(&mut self, e: StorageEvent) {
        // Secret persisted to an insecure store.
        let matched_secrets: Vec<StaticFinding> = self
            .secrets
            .iter()
            .filter(|s| {
                s.value.len() >= MIN_SECRET_LEN
                    && (e.value.contains(&s.value) || e.key.contains(&s.value))
            })
            .cloned()
            .collect();
        for secret in matched_secrets {
            self.emit(
                CorrelationKind::SecretPersistedInsecure,
                &secret.value,
                Finding {
                    id: format!("f-{}", uuid::Uuid::new_v4()),
                    kind: CorrelationKind::SecretPersistedInsecure,
                    title: "Hardcoded secret persisted on device".into(),
                    summary: format!(
                        "Secret from {} written to {:?} under key {}",
                        secret.location, e.store, e.key
                    ),
                    severity: Severity::Medium,
                    confidence: ConfidenceTier::Confirmed,
                    evidence_event_ids: vec![secret.id.clone(), e.id.clone()],
                    first_seen_ms: e.timestamp_ms,
                    occurrences: 1,
                    score: 0.0,
                },
            );
        }

        // A token captured in traffic was then persisted.
        let matched_tokens: Vec<String> = self
            .captured_tokens
            .iter()
            .filter(|t| e.value.contains(t.as_str()))
            .cloned()
            .collect();
        for token in matched_tokens {
            self.emit(
                CorrelationKind::TokenPersisted,
                &token,
                Finding {
                    id: format!("f-{}", uuid::Uuid::new_v4()),
                    kind: CorrelationKind::TokenPersisted,
                    title: "Session token persisted on device".into(),
                    summary: format!(
                        "Auth token captured in traffic was written to {:?} under key {}",
                        e.store, e.key
                    ),
                    severity: Severity::Medium,
                    confidence: ConfidenceTier::Confirmed,
                    evidence_event_ids: vec![e.id.clone()],
                    first_seen_ms: e.timestamp_ms,
                    occurrences: 1,
                    score: 0.0,
                },
            );
        }

        // A runtime storage write that itself looks like new secret material.
        if e.value.len() >= 12 && looks_like_secret(&e.key) && looks_like_secret(&e.value) {
            self.emit(
                CorrelationKind::RuntimeSecretCreated,
                &e.key,
                Finding {
                    id: format!("f-{}", uuid::Uuid::new_v4()),
                    kind: CorrelationKind::RuntimeSecretCreated,
                    title: "Runtime-created secret material stored".into(),
                    summary: format!(
                        "{:?} write under key {} contains secret-looking value",
                        e.store, e.key
                    ),
                    severity: Severity::Low,
                    confidence: ConfidenceTier::Speculative,
                    evidence_event_ids: vec![e.id.clone()],
                    first_seen_ms: e.timestamp_ms,
                    occurrences: 1,
                    score: 0.0,
                },
            );
        }
    }

    /// Sorted findings (highest score first).
    pub fn findings(&self) -> Vec<Finding> {
        let mut f = self.findings.clone();
        f.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        f
    }

    /// Findings first observed at/after `since_ms` (used for relaunch diffing).
    pub fn findings_since(&self, since_ms: i64) -> Vec<Finding> {
        self.findings()
            .into_iter()
            .filter(|f| f.first_seen_ms >= since_ms)
            .collect()
    }

    /// Snapshot the current findings, so `new_since_baseline` can report only
    /// what happened after this point.
    pub fn mark_baseline(&mut self) {
        self.baseline_len = self.findings.len();
    }

    /// Findings that were not present at the last `mark_baseline` — the
    /// "what's new this session" diff for app relaunches. Findings are
    /// append-only (dedup only bumps occurrence counts), so this is simply the
    /// suffix of the findings vec past the baseline length.
    pub fn new_since_baseline(&self) -> Vec<Finding> {
        if self.baseline_len >= self.findings.len() {
            return Vec::new();
        }
        let mut f = self.findings[self.baseline_len..].to_vec();
        f.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        f
    }
}

/// Heuristic: does this key/value look like credential material?
fn looks_like_secret(s: &str) -> bool {
    let lower = s.to_lowercase();
    ["key", "secret", "token", "pass", "credential", "auth"]
        .iter()
        .any(|k| lower.contains(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn secret() -> StaticFinding {
        StaticFinding {
            id: "s1".into(),
            kind: StaticKind::HardcodedSecret,
            value: "sk-ant-abcdef123".into(),
            location: "AppDelegate".into(),
        }
    }

    #[test]
    fn secret_in_traffic_is_confirmed() {
        let mut e = CorrelationEngine::new();
        e.ingest_static(secret());
        e.ingest_network(NetworkEvent {
            id: "n1".into(),
            method: "GET".into(),
            url: "https://api.example.com/v1/me".into(),
            headers: vec![("Authorization".into(), "Bearer sk-ant-abcdef123".into())],
            body: String::new(),
            timestamp_ms: 100,
        });
        let findings = e.findings();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, CorrelationKind::SecretUsedLive);
        assert_eq!(findings[0].confidence, ConfidenceTier::Confirmed);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn secret_persisted_is_confirmed() {
        let mut e = CorrelationEngine::new();
        e.ingest_static(secret());
        e.ingest_storage(StorageEvent {
            id: "st1".into(),
            store: StorageKind::UserDefaults,
            key: "api_key".into(),
            value: "sk-ant-abcdef123".into(),
            timestamp_ms: 200,
        });
        let findings = e.findings();
        assert!(findings
            .iter()
            .any(|f| f.kind == CorrelationKind::SecretPersistedInsecure));
    }

    #[test]
    fn token_persisted_after_capture() {
        let mut e = CorrelationEngine::new();
        e.ingest_network(NetworkEvent {
            id: "n1".into(),
            method: "POST".into(),
            url: "https://api.example.com/login".into(),
            headers: vec![("Authorization".into(), "Bearer tok-999999".into())],
            body: String::new(),
            timestamp_ms: 100,
        });
        e.ingest_storage(StorageEvent {
            id: "st1".into(),
            store: StorageKind::Keychain,
            key: "session".into(),
            value: "tok-999999".into(),
            timestamp_ms: 200,
        });
        let findings = e.findings();
        assert!(findings
            .iter()
            .any(|f| f.kind == CorrelationKind::TokenPersisted));
    }

    #[test]
    fn privileged_endpoint_reached() {
        let mut e = CorrelationEngine::new();
        e.ingest_static(StaticFinding {
            id: "p1".into(),
            kind: StaticKind::DangerousEndpoint,
            value: "/internal/admin".into(),
            location: "Routes".into(),
        });
        e.ingest_network(NetworkEvent {
            id: "n1".into(),
            method: "GET".into(),
            url: "https://api.example.com/internal/admin/users".into(),
            headers: vec![],
            body: String::new(),
            timestamp_ms: 100,
        });
        let findings = e.findings();
        assert!(findings
            .iter()
            .any(|f| f.kind == CorrelationKind::PrivilegedEndpointReached));
    }

    #[test]
    fn dedup_bumps_occurrences() {
        let mut e = CorrelationEngine::new();
        e.ingest_static(secret());
        let net = |id: &str| NetworkEvent {
            id: id.into(),
            method: "GET".into(),
            url: "https://api.example.com/v1/me".into(),
            headers: vec![("X-Key".into(), "sk-ant-abcdef123".into())],
            body: String::new(),
            timestamp_ms: 100,
        };
        e.ingest_network(net("n1"));
        e.ingest_network(net("n2"));
        let findings = e.findings();
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].occurrences, 2);
    }

    #[test]
    fn baseline_diffing_reports_only_new() {
        let mut e = CorrelationEngine::new();
        e.ingest_static(secret());
        e.ingest_network(NetworkEvent {
            id: "n1".into(),
            method: "GET".into(),
            url: "https://api.example.com/v1/me".into(),
            headers: vec![("X-Key".into(), "sk-ant-abcdef123".into())],
            body: String::new(),
            timestamp_ms: 100,
        });
        e.mark_baseline();

        // New activity in a "relaunch": a different secret persisted.
        e.ingest_static(StaticFinding {
            id: "s2".into(),
            kind: StaticKind::HardcodedSecret,
            value: "x7k29q1z8m4n6".into(),
            location: "Config".into(),
        });
        e.ingest_storage(StorageEvent {
            id: "st2".into(),
            store: StorageKind::File,
            key: "saved.txt".into(),
            value: "x7k29q1z8m4n6".into(),
            timestamp_ms: 500,
        });

        let new = e.new_since_baseline();
        assert_eq!(new.len(), 1);
        assert_eq!(new[0].kind, CorrelationKind::SecretPersistedInsecure);
    }
}