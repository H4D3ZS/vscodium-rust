//! Cross-app vulnerability fingerprint model.
//!
//! Decouples a confirmed vulnerability from the app it was first found in, so
//! the *pattern* can be replayed against any later target that shares the same
//! component. This is the compounding layer: every confirmed bug makes the next
//! ten apps faster to triage instead of starting from zero each time.
//!
//! The key design point is that a shared component (SDK, auth library,
//! payment processor) does **not** guarantee the same bug exists — the backend
//! integration around it may differ. A component match is therefore a
//! *hypothesis to verify automatically*, not an auto-confirmed finding. The
//! matcher (matcher.rs) surfaces hypotheses; verification is a separate step.
//!
//! Authorized testing / bug bounty use only.

use serde::{Deserialize, Serialize};

/// Broad category of reusable technology a fingerprint identifies. This is the
/// join key across apps — not the app itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentKind {
    Sdk,
    AuthLibrary,
    PaymentProcessor,
    BackendFramework,
    Other,
}

/// How we recognize a component in a *new* app without re-deriving from
/// scratch. A signature is a cheap first-pass gate: it must match on already-
/// extracted artifacts (class-dump symbols, strings, traffic hosts, request
/// shapes) before any vulnerability pattern is considered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DetectionSignature {
    /// Exported class / symbol names visible in a class-dump or symbol table.
    BinarySymbols { symbols: Vec<String> },
    /// Characteristic hardcoded strings, API key prefixes, or URL fragments.
    StringConstants { constants: Vec<String> },
    /// API host suffix, e.g. "stripe.com" matches "api.stripe.com".
    NetworkHostPattern { pattern: String },
    /// Traffic fingerprint: a header set and body field naming convention that
    /// must all be present in the target's captured requests.
    RequestShape {
        headers: Vec<String>,
        body_fields: Vec<String>,
    },
}

/// The vulnerability class. Mirrors the categories that actually pay out in
/// mobile-backed API programs (BOLA, mass assignment, JWT, trusted headers...),
/// plus the 2026-fresh agentic surface (AI features acting on injected content).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VulnClass {
    Bola,
    MassAssignment,
    JwtAlgConfusion,
    HeaderTrust,
    RaceCondition,
    MissingRateLimit,
    SecretInTraffic,
    /// An in-app / on-device AI agent acted on attacker-injected content as if
    /// it were an instruction (prompt injection), with concrete downstream
    /// effect. The channel + payload strategy live in the root cause trigger.
    AgenticInjection,
    Other,
}

/// How confident we are that a fingerprint reflects a real, reproducible bug.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceTier {
    Speculative,
    Probable,
    Confirmed,
}

/// The structured root cause, expressed so it is machine-matchable against a
/// concrete endpoint rather than stored as free-text prose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TriggerCondition {
    /// The endpoint accepts an object id but never verifies ownership.
    MissingOwnershipCheck { id_field: String },
    /// The backend honors a privilege/state field it should strip.
    MassAssignmentField { field: String },
    /// JWT accepted with `alg` = `none`.
    JwtAlgNone,
    /// Backend blindly trusts an internal-service / forwarded header.
    TrustedHeader { header: String },
    /// The app's agent interpreted embedded content as an instruction. `channel`
    /// is the ingestion surface (share extension, deep link, file import,
    /// clipboard, notification, ...) and `category` the payload strategy.
    EmbeddedInstruction { channel: String, category: String },
}

/// Verification outcome of a matched hypothesis, after the auto-replay step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootCause {
    /// Templated path, e.g. "/api/v1/users/{id}/profile". Placeholders use
    /// `{name}` and match any single non-`/` segment.
    pub endpoint_pattern: String,
    pub trigger: TriggerCondition,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproStep {
    /// One of CreateAccount, CaptureToken, ReplayRequest, InjectField,
    /// StripSignature, AddHeader — a symbolic operation, not app-specific.
    pub op: String,
    pub note: String,
}

/// A parameterized proof-of-exploit: symbolic steps that can be instantiated
/// against a new target with that target's real values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReproTemplate {
    pub steps: Vec<ReproStep>,
    /// Placeholder names referenced by steps, e.g. "{base_url}", "{auth_token_b}".
    pub placeholders: Vec<String>,
    /// Optional canary marker format (e.g. "AGENT_CANARY_{id}"). A unique
    /// per-run token is embedded in delivered content; its reappearance in an
    /// outbound action or captured traffic is the deterministic proof the
    /// instruction executed, rather than "the model probably did something".
    #[serde(default)]
    pub canary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentFingerprint {
    pub id: String,
    pub kind: ComponentKind,
    pub name: String,
    /// Semver range if the bug is version-specific; `None` means "any version".
    pub version_range: Option<String>,
    pub signature: DetectionSignature,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VulnFingerprint {
    pub id: String,
    pub component_id: String,
    pub class: VulnClass,
    pub root_cause: RootCause,
    pub confidence: ConfidenceTier,
    pub first_seen_app: String,
    /// Unix timestamp (seconds) of first confirmation.
    pub first_seen_at: i64,
    pub repro: ReproTemplate,
}

/// A target reduced to the artifacts the matcher needs. Produced by the static
/// (class-dump / strings / entitlements) and dynamic (traffic) pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppAnalysis {
    pub app_id: String,
    pub symbols: Vec<String>,
    pub string_constants: Vec<String>,
    pub hosts: Vec<String>,
    pub headers: Vec<String>,
    pub body_fields: Vec<String>,
    /// Concrete endpoints discovered via traffic capture or static routes.
    pub endpoints: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_serialize_snake_case() {
        let sig = DetectionSignature::BinarySymbols {
            symbols: vec!["Stripe".into()],
        };
        let json = serde_json::to_string(&sig).unwrap();
        assert!(json.contains("\"type\":\"binary_symbols\""));
        assert!(json.contains("\"symbols\":[\"Stripe\"]"));
    }
}
