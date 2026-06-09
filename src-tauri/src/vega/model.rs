//! APEX Vega — core data model.
//!
//! Types shared across the scanner engine, the JS module host, and the Tauri
//! command layer. Ported from Subgraph Vega's Java model but reshaped for Rust.
//! See `.planning/vega-integration/01-VEGA-INVENTORY.md` for the spec these
//! types implement.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Finding severity. Ordered so `>=` comparisons work for filtering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    /// Parse the `<severity>` text from a Vega alert XML (case-insensitive).
    pub fn from_str_loose(s: &str) -> Severity {
        match s.trim().to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" | "med" => Severity::Medium,
            "low" => Severity::Low,
            _ => Severity::Info,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "info",
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }
}

/// Static metadata for an alert type, loaded from `resources/vega/alerts/<key>.xml`.
/// The filename (minus `.xml`) is the `type_key` that modules pass to `ctx.alert(...)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertDefinition {
    pub type_key: String,
    pub title: String,
    pub class: String,
    pub severity: Severity,
    /// One or more `<impact>` lines.
    pub impact: Vec<String>,
    pub remediation: String,
    pub discussion: String,
}

/// A concrete finding raised at scan time via `ctx.alert(...)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert type key, e.g. "vinfo-sql-inject". Resolves to an `AlertDefinition`.
    pub type_key: String,
    /// Resolved title (from definition, or the key if unknown).
    pub title: String,
    pub severity: Severity,
    /// The affected resource (URI path), from `opts.resource`.
    pub resource: String,
    /// Dedupe key from `opts.key` — used by `ctx.alertExists`.
    pub key: String,
    /// Evidence string from `opts.output` (e.g. matched body/header).
    #[serde(default)]
    pub output: String,
    /// Optional detection technique label, e.g. "Blind Text Injection Differential".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detection_type: Option<String>,
    /// Index of the saved request/response that triggered this (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_id: Option<u64>,
    /// Any extra fields passed in `opts` we don't model explicitly.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
    /// Unix-ms timestamp when raised.
    pub timestamp_ms: u64,
}

/// Where a fuzzable parameter lives in a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParamLocation {
    Query,
    Post,
    Path,
    Header,
    Cookie,
}

/// A single parameter the scanner can mutate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzableParam {
    pub name: String,
    pub value: String,
    pub location: ParamLocation,
}

/// Minimal HTTP request model. Expanded in Phase 3 (engine) — kept lean for now.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
}

impl HttpRequest {
    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }
}

/// Minimal HTTP response model. Expanded in Phase 3.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
    /// True if the fetch failed (network error / timeout). Modules check this.
    #[serde(default)]
    pub fetch_fail: bool,
}

impl HttpResponse {
    pub fn has_header(&self, name: &str) -> bool {
        let n = name.to_lowercase();
        self.headers.iter().any(|(k, _)| k.to_lowercase() == n)
    }

    pub fn first_header(&self, name: &str) -> Option<(String, String)> {
        let n = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == n)
            .cloned()
    }
}

/// The state of one crawled path + its fuzzable parameters. The JS `ps` object.
/// Fleshed out in Phase 3; defined here so the model compiles end-to-end.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PathState {
    pub uri: String,
    pub is_post_target: bool,
    pub params: Vec<FuzzableParam>,
    /// Index into `params` currently being fuzzed.
    pub fuzz_index: Option<usize>,
    /// Baseline response fingerprint (set after the first/unaltered fetch).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<u64>,
}

impl PathState {
    pub fn is_parametric(&self) -> bool {
        !self.params.is_empty()
    }

    pub fn fuzzable_parameter(&self) -> Option<&FuzzableParam> {
        self.fuzz_index.and_then(|i| self.params.get(i))
    }
}
