use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Ios,
    Android,
    CrossPlatform,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Ios => "iOS",
            Platform::Android => "Android",
            Platform::CrossPlatform => "Cross-Platform",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "INFO",
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "CRITICAL" => Severity::Critical,
            "HIGH" => Severity::High,
            "MEDIUM" => Severity::Medium,
            "LOW" => Severity::Low,
            _ => Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssData {
    pub score: f64,
    pub severity: Severity,
    pub vector: String,
    pub exploitability: f64,
    pub impact: f64,
}

impl Default for CvssData {
    fn default() -> Self {
        Self {
            score: 0.0,
            severity: Severity::Info,
            vector: String::new(),
            exploitability: 0.0,
            impact: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GateStatus {
    #[default]
    NeedsManualPoc,
    Passed,
    KilledNeverSubmit(String),
    OutOfScope(String),
    LowConfidence,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GateResult {
    pub status: GateStatus,
    pub notes: Vec<String>,
    pub chain_eligible: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub rule_id: String,
    pub title: String,
    pub vuln_class: String,
    pub severity: Severity,
    pub platform: Platform,
    pub file_path: String,
    pub line: usize,
    pub snippet: String,
    pub description: String,
    pub impact: String,
    pub remediation: String,
    pub cvss: CvssData,
    pub gate: GateResult,
    pub reproduction_steps: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Finding {
    pub fn new(
        rule_id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        severity: Severity,
        platform: Platform,
    ) -> Self {
        let r_id = rule_id.into();
        Self {
            id: format!("{}-{}", r_id, uuid_simple()),
            rule_id: r_id,
            title: title.into(),
            vuln_class: "Security Vulnerability".to_string(),
            severity,
            platform,
            file_path: String::new(),
            line: 0,
            snippet: String::new(),
            description: description.into(),
            impact: String::new(),
            remediation: String::new(),
            cvss: CvssData::default(),
            gate: GateResult::default(),
            reproduction_steps: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{:x}", nanos)[..8].to_string()
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppMetadata {
    pub name: String,
    pub identifier: String, // bundle_id or package_name
    pub version: String,
    pub build: String,
    pub min_sdk_or_os: String,
    pub target_sdk: Option<u32>,
    pub platform: Option<Platform>,
    pub is_encrypted: bool,
    pub custom_schemes: Vec<String>,
    pub permissions: Vec<String>,
    pub frameworks: Vec<String>,
    pub ai_models_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub target_app: AppMetadata,
    pub total_findings: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub info_count: usize,
    pub gate_passed_count: usize,
    pub findings: Vec<Finding>,
    pub api_endpoints: Vec<String>,
    pub hardcoded_secrets: Vec<Finding>,
    pub duration_ms: u64,
}
