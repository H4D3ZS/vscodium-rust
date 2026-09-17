//! Correlation event model.
//!
//! The three event streams the correlation engine ingests simultaneously while
//! an app is exercised normally: static findings (from IPA unpack), live
//! network traffic (hooked at the HTTP client, not a proxy), and live storage
//! writes (keychain / UserDefaults / CoreData).
//!
//! Authorized testing / bug bounty use only.

use serde::{Deserialize, Serialize};

use crate::domain::fingerprint::model::{ConfidenceTier, VulnClass};

/// Impact severity of a correlated finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
            Self::Info => write!(f, "INFO"),
            Self::Low => write!(f, "LOW"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::High => write!(f, "HIGH"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Kind of static artifact extracted at IPA-unpack time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StaticKind {
    /// A hardcoded credential: API key, token, client secret, password.
    HardcodedSecret,
    /// A hardcoded private key / certificate material.
    PrivateKey,
    /// An endpoint that looks privileged (admin, internal, debug).
    DangerousEndpoint,
    /// A weak access-control entitlement or overly broad permission.
    WeakEntitlement,
}

/// A finding produced by static analysis (class-dump / strings / entitlements).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticFinding {
    pub id: String,
    pub kind: StaticKind,
    /// The literal secret value, endpoint path, or entitlement value.
    pub value: String,
    /// Where it was found: class, file, or binary section.
    pub location: String,
}

/// A single observed outbound request, captured at the HTTP client layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub id: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    /// Response/request body snippet (truncated by the capture layer).
    pub body: String,
    pub timestamp_ms: i64,
}

/// Where a key-value write landed on the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageKind {
    Keychain,
    UserDefaults,
    CoreData,
    File,
}

/// A key-value write observed at the storage layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageEvent {
    pub id: String,
    pub store: StorageKind,
    pub key: String,
    pub value: String,
    pub timestamp_ms: i64,
}

/// What two (or more) streams were cross-referenced to produce a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrelationKind {
    /// A static secret appeared in a live outbound request (header/url/body).
    SecretUsedLive,
    /// A static secret was written to an insecure on-device store.
    SecretPersistedInsecure,
    /// An auth token captured in traffic was then persisted on-device.
    TokenPersisted,
    /// A statically-flagged privileged endpoint was reached live.
    PrivilegedEndpointReached,
    /// A runtime storage write that itself looks like a new secret.
    RuntimeSecretCreated,
    /// An app-embedded agent acted on injected content as an instruction.
    AgenticInjection,
}

impl CorrelationKind {
    /// The fingerprint class this finding promotes to, for the compounding DB.
    pub fn vuln_class(self) -> VulnClass {
        match self {
            CorrelationKind::SecretUsedLive
            | CorrelationKind::SecretPersistedInsecure
            | CorrelationKind::TokenPersisted
            | CorrelationKind::RuntimeSecretCreated => VulnClass::SecretInTraffic,
            CorrelationKind::PrivilegedEndpointReached => VulnClass::MissingRateLimit,
            CorrelationKind::AgenticInjection => VulnClass::AgenticInjection,
        }
    }
}

/// A scored, evidence-backed finding emitted by the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub kind: CorrelationKind,
    pub title: String,
    pub summary: String,
    pub severity: Severity,
    pub confidence: ConfidenceTier,
    /// Ids of the source events that were cross-referenced.
    pub evidence_event_ids: Vec<String>,
    pub first_seen_ms: i64,
    /// How many times the same pattern has re-observed (dedup count).
    pub occurrences: u32,
    /// Priority score used for queue ordering.
    pub score: f32,
}

impl Finding {
    fn severity_weight(severity: Severity) -> f32 {
        match severity {
            Severity::Critical => 1.0,
            Severity::High => 0.8,
            Severity::Medium => 0.6,
            Severity::Low => 0.4,
            Severity::Info => 0.2,
        }
    }

    fn confidence_weight(confidence: ConfidenceTier) -> f32 {
        match confidence {
            ConfidenceTier::Confirmed => 1.0,
            ConfidenceTier::Probable => 0.7,
            ConfidenceTier::Speculative => 0.4,
        }
    }

    pub fn compute_score(severity: Severity, confidence: ConfidenceTier) -> f32 {
        Self::severity_weight(severity) * Self::confidence_weight(confidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_orders() {
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }

    #[test]
    fn kind_maps_to_class() {
        assert_eq!(
            CorrelationKind::SecretUsedLive.vuln_class(),
            VulnClass::SecretInTraffic
        );
        assert_eq!(
            CorrelationKind::AgenticInjection.vuln_class(),
            VulnClass::AgenticInjection
        );
    }
}