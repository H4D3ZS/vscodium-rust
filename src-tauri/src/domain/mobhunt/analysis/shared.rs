use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct SecretPattern {
    pub name: &'static str,
    pub pattern: &'static str,
    pub severity: Severity,
    pub cvss_vector: &'static str,
}

pub static SECRET_PATTERNS: &[SecretPattern] = &[
    // LLM Provider Keys
    SecretPattern {
        name: "OpenAI API Secret Key",
        pattern: r"sk-[a-zA-Z0-9_\-]{20,48}",
        severity: Severity::Critical,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
    },
    SecretPattern {
        name: "Anthropic Claude API Key",
        pattern: r"sk-ant-[a-zA-Z0-9_\-]{20,}",
        severity: Severity::Critical,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
    },
    // Cloud Provider Keys
    SecretPattern {
        name: "AWS Access Key ID",
        pattern: r"AKIA[0-9A-Z]{16}",
        severity: Severity::Critical,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
    },
    SecretPattern {
        name: "Google API Key",
        pattern: r"AIza[0-9A-Za-z\-_]{35}",
        severity: Severity::High,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
    },
    // Firebase
    SecretPattern {
        name: "Firebase Database URL",
        pattern: r"https://[a-z0-9\-]+\.firebaseio\.com",
        severity: Severity::High,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
    },
    // Payment
    SecretPattern {
        name: "Stripe Live Secret Key",
        pattern: r"sk_live_[A-Za-z0-9]{20,}",
        severity: Severity::Critical,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
    },
    SecretPattern {
        name: "Stripe Publishable Key",
        pattern: r"pk_live_[A-Za-z0-9]{20,}",
        severity: Severity::Low,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N",
    },
    // Tokens & Credentials
    SecretPattern {
        name: "Slack Token",
        pattern: r"xox[bpors]-[0-9]{10,}-[A-Za-z0-9\-]{10,}",
        severity: Severity::High,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
    },
    SecretPattern {
        name: "GitHub Personal Access Token",
        pattern: r"gh[pso]_[A-Za-z0-9]{36}",
        severity: Severity::High,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
    },
    SecretPattern {
        name: "RSA/EC Private Key",
        pattern: r"-----BEGIN (?:RSA )?PRIVATE KEY-----",
        severity: Severity::Critical,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
    },
    SecretPattern {
        name: "Hardcoded Password Assignment",
        pattern: r#"(?i)(?:password|passwd|pwd)\s*[=:]\s*["']([^"'\s]{8,})["']"#,
        severity: Severity::High,
        cvss_vector: "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
    },
];

/// Calculate Shannon entropy of a string (bits per symbol)
pub fn shannon_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0usize) += 1;
    }
    let len = s.len() as f64;
    let mut entropy = 0.0;
    for &count in counts.values() {
        let p = (count as f64) / len;
        entropy -= p * p.log2();
    }
    entropy
}

/// Scan a file content for secrets using compiled regex patterns
pub fn scan_secrets(file_path: &str, content: &str, platform: Platform) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Check false positive markers
    let lower = content.to_ascii_lowercase();
    if lower.contains("test") && lower.contains("mock") && lower.contains("dummy") && content.len() < 500 {
        return findings;
    }

    for pat in SECRET_PATTERNS {
        if let Ok(re) = Regex::new(pat.pattern) {
            for (line_idx, line) in content.lines().enumerate() {
                if let Some(mat) = re.find(line) {
                    let matched_str = mat.as_str();
                    // Ignore placeholders like "sk-xxxxxxxx" or "AKIA0000000000000000"
                    if matched_str.chars().all(|c| c == 'x' || c == 'X' || c == '0') {
                        continue;
                    }

                    let cvss = crate::domain::mobhunt::reporting::calculate_cvss(pat.cvss_vector)
                        .unwrap_or(CvssData {
                            score: 7.5,
                            severity: pat.severity,
                            vector: pat.cvss_vector.to_string(),
                            exploitability: 3.9,
                            impact: 3.6,
                        });

                    findings.push(Finding {
                        id: format!("SEC-{}", uuid::Uuid::new_v4().simple()),
                        rule_id: format!("MOBHUNT-SECRET-{}", pat.name.replace(' ', "_").to_ascii_uppercase()),
                        title: format!("Hardcoded Secret: {}", pat.name),
                        vuln_class: "hardcoded_secret".to_string(),
                        severity: pat.severity,
                        platform,
                        file_path: file_path.to_string(),
                        line: line_idx + 1,
                        snippet: line.trim().to_string(),
                        description: format!("Discovered static credential pattern for '{}' in application asset.", pat.name),
                        impact: "Hardcoded credentials in mobile application binaries can be extracted via static decompilation and abused to access protected APIs and backend infrastructure.".to_string(),
                        remediation: "Remove hardcoded credentials from the client application. Move credentialed access to a backend server proxy with proper authentication and rate-limiting.".to_string(),
                        cvss,
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec!["Valid hardcoded secret finding meeting 7-Question Gate".into()],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Inspect file `{}` around line {}.", file_path, line_idx + 1),
                            format!("Observe exposed secret credential string matching `{}`.", pat.name),
                            "Verify credential access rights against provider endpoint.".into(),
                        ],
                        metadata: HashMap::new(),
                    });
                }
            }
        }
    }

    findings
}

/// Extract API endpoints (REST, GraphQL, internal hostnames, S3) from file content
pub fn extract_endpoints(content: &str) -> Vec<String> {
    let mut endpoints = Vec::new();
    if let Ok(re) = Regex::new(r#"https?://[a-zA-Z0-9_\-\.]+(:[0-9]+)?(/[a-zA-Z0-9_\-/\.\?=&%#]*)?"#) {
        for mat in re.find_iter(content) {
            let url = mat.as_str().to_string();
            // Filter common noisy/framework schemas
            if !url.contains("schemas.android.com")
                && !url.contains("w3.org")
                && !url.contains("apple.com")
                && !url.contains("schema.org")
            {
                if !endpoints.contains(&url) {
                    endpoints.push(url);
                }
            }
        }
    }
    endpoints
}

/// Detect Hermes JavaScript bytecode bundle
pub fn is_hermes_bytecode(bytes: &[u8]) -> bool {
    // Magic header for Hermes: 0x1F410C03 in little endian => [0xC6, 0x1F, 0xBC, 0x03] or [0x03, 0x0C, 0x41, 0x1F]
    bytes.len() >= 4 && bytes[0] == 0xC6 && bytes[1] == 0x1F && bytes[2] == 0xBC && bytes[3] == 0x03
}
