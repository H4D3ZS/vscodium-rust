use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct LogicFlawPattern {
    pub id: &'static str,
    pub title: &'static str,
    pub pattern: &'static str,
    pub severity: Severity,
    pub cvss: f64,
    pub cwe: &'static str,
    pub masvs: &'static str,
    pub description: &'static str,
    pub remediation: &'static str,
}

pub static LOGIC_FLAW_PATTERNS: &[LogicFlawPattern] = &[
    LogicFlawPattern {
        id: "LOGIC-PAYMENT-BYPASS",
        title: "Client-Side In-App Purchase / Billing Validation Bypass",
        pattern: r#"(?i)(?:isPaid|isSubscribed|isPremium|hasPurchased)\s*=\s*true|(?:validatePurchase|checkPaymentStatus)\s*\([^)]*\)\s*\{\s*return\s+true"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-602",
        masvs: "MASVS-RESILIENCE-1",
        description: "Application implements client-side trust decisions for subscription or in-app purchase validation (`isPaid = true` or `validatePurchase` returning true). Attackers using runtime hooking (Frida / Objection) can easily unlock premium features without making payment.",
        remediation: "Never trust client-side purchase state. Always validate Google Play Billing purchase tokens or Apple StoreKit receipts via a secure backend server before granting access.",
    },
    LogicFlawPattern {
        id: "LOGIC-AUTH-BYPASS",
        title: "Client-Side Authentication / Admin Authorization Check",
        pattern: r#"(?i)(?:isAdmin|isSuperUser|isAuthenticated)\s*=\s*true|(?:checkAuth|verifyAdmin)\s*\([^)]*\)\s*\{\s*return\s+true"#,
        severity: Severity::Critical,
        cvss: 8.8,
        cwe: "CWE-602",
        masvs: "MASVS-AUTH-1",
        description: "Application sets authorization flags (`isAdmin = true`) or bypasses authentication client-side. The client device is an untrusted environment controlled entirely by the user.",
        remediation: "All role checks and administrative actions must be authorized server-side on each API endpoint based on cryptographically signed JWT or session tokens.",
    },
    LogicFlawPattern {
        id: "LOGIC-INTERNAL-ENDPOINT",
        title: "Hardcoded Internal, Staging, or Admin API Route",
        pattern: r#"https?://[a-zA-Z0-9.\-_]+(?::\d+)?/(?:admin|internal|debug|staging|dev|sandbox)(?:/[^\s<>"'\\]*)?"#,
        severity: Severity::Medium,
        cvss: 5.3,
        cwe: "CWE-200",
        masvs: "MASVS-PLATFORM-1",
        description: "Application binary contains references to internal, administrative, staging, or debug API endpoints. These endpoints often lack production-grade authentication or rate limiting.",
        remediation: "Remove non-production endpoints from release builds and ensure all administrative routes require multi-factor server-side authentication.",
    },
];

/// Scans code files for business logic flaws and exposed non-production endpoints
pub fn audit_logic_flaws(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for pattern in LOGIC_FLAW_PATTERNS {
        if let Ok(re) = Regex::new(pattern.pattern) {
            for (line_idx, line) in content.lines().enumerate() {
                if let Some(mat) = re.find(line) {
                    findings.push(Finding {
                        id: format!("{}-{}", pattern.id, line_idx + 1),
                        rule_id: pattern.id.to_string(),
                        title: pattern.title.to_string(),
                        vuln_class: pattern.masvs.to_string(),
                        severity: pattern.severity,
                        platform: Platform::CrossPlatform,
                        file_path: file_path.to_string(),
                        line: line_idx + 1,
                        snippet: mat.as_str().trim().to_string(),
                        description: pattern.description.to_string(),
                        impact: format!("Violates {} ({}). Susceptible to client-side bypass or sensitive interface exposure.", pattern.masvs, pattern.cwe),
                        remediation: pattern.remediation.to_string(),
                        cvss: CvssData {
                            score: pattern.cvss,
                            severity: pattern.severity,
                            vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N".to_string(),
                            exploitability: 3.9,
                            impact: 3.6,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec![format!("Matches logic flaw pattern {}", pattern.id)],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Inspect code at `{}:{}`.", file_path, line_idx + 1),
                            format!("Snippet: `{}`", mat.as_str().trim()),
                        ],
                        metadata: HashMap::new(),
                    });
                }
            }
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_flaw_detection() {
        let code = r#"
            boolean isSubscribed = true;
            String adminUrl = "https://api.example.com/admin/v1/users";
        "#;
        let findings = audit_logic_flaws(code, "SubscriptionService.java");
        assert!(findings.iter().any(|f| f.rule_id == "LOGIC-PAYMENT-BYPASS"));
        assert!(findings.iter().any(|f| f.rule_id == "LOGIC-INTERNAL-ENDPOINT"));
    }
}
