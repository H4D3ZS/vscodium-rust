use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct DeepLinkPattern {
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

pub static DEEPLINK_PATTERNS: &[DeepLinkPattern] = &[
    DeepLinkPattern {
        id: "INTENT-REDIRECT",
        title: "Android Intent Redirection Vulnerability",
        pattern: r#"(?i)startActivity\s*\(\s*\(\s*Intent\s*\)\s*(?:intent\.getParcelableExtra|getIntent\(\)\.getParcelableExtra)\s*\("#,
        severity: Severity::Critical,
        cvss: 8.8,
        cwe: "CWE-926",
        masvs: "MASVS-PLATFORM-1",
        description: "Application retrieves an Intent object directly from an incoming Intent's parcelable extras and passes it to `startActivity()` without verifying the target component. This allows malicious third-party apps to access unexported, internal, or privileged activities and ContentProviders.",
        remediation: "Never start an untrusted Intent directly. Validate `intent.getComponent()` against an explicit whitelist of allowed components, or check that the target component does not grant URI permissions to private content providers.",
    },
    DeepLinkPattern {
        id: "INTENT-PATH-TRAVERSAL",
        title: "Intent Extra Used Directly in File Path (Path Traversal)",
        pattern: r#"(?i)(?:openFileOutput|FileInputStream|FileOutputStream|new\s+File)\s*\(\s*(?:intent\.getStringExtra|getIntent\(\)\.getStringExtra)\s*\("#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-22",
        masvs: "MASVS-STORAGE-1",
        description: "Application uses a string parameter from an incoming Intent extra directly to open or create a file. If the parameter contains directory traversal characters (`../`), an attacker can read or overwrite files outside the intended directory.",
        remediation: "Sanitize the filename using `new File(filename).getName()` to strip directory paths, and verify the resolved canonical path remains within the intended directory.",
    },
    DeepLinkPattern {
        id: "INTENT-UNSAFE-OVERRIDE",
        title: "Insecure Intent:// Scheme Parsing in WebView",
        pattern: r#"(?i)Intent\.parseUri\s*\(\s*url\s*,\s*Intent\.URI_INTENT_SCHEME\s*\)"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-926",
        masvs: "MASVS-PLATFORM-1",
        description: "Application parses `intent://` URIs in WebView `shouldOverrideUrlLoading` via `Intent.parseUri(url, Intent.URI_INTENT_SCHEME)`. Without proper filtering, malicious web content can launch arbitrary exported or internal components or trigger Intent redirection.",
        remediation: "Add the `Intent.CATEGORY_BROWSABLE` category, remove the selector component, and verify `intent.getComponent() == null` before launching.",
    },
    DeepLinkPattern {
        id: "INTENT-IMPLICIT-BROADCAST",
        title: "Implicit Broadcast Sent Without Specifying Receiver Permission",
        pattern: r#"(?i)sendBroadcast\s*\(\s*(?:new\s+Intent|intent)\s*\)(?!;[\s\S]*permission)"#,
        severity: Severity::Medium,
        cvss: 5.3,
        cwe: "CWE-927",
        masvs: "MASVS-PLATFORM-2",
        description: "Application sends an implicit broadcast without specifying receiver permissions. Any application on the device with a matching broadcast receiver filter can intercept the broadcast and read any sensitive extras.",
        remediation: "Use explicit Intents with `setComponent()` or `setPackage()`, use `LocalBroadcastManager`, or specify a custom signature-level permission.",
    },
    DeepLinkPattern {
        id: "DEEPLINK-DANGEROUS-SCHEME",
        title: "Dangerous URI Scheme Handler in Deep Link Routing",
        pattern: r#"(?i)(?:javascript:|file://|content://)\s*["']"#,
        severity: Severity::High,
        cvss: 7.4,
        cwe: "CWE-20",
        masvs: "MASVS-PLATFORM-1",
        description: "Application references or handles dangerous URI schemes (javascript:, file://, content://) in deep link or URL routing logic.",
        remediation: "Strictly restrict deep link handling to trusted custom application schemes and verified https:// App Links / Universal Links.",
    },
];

/// Scans source files or smali for deep link and intent handling vulnerabilities
pub fn audit_deeplinks_and_intents(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for pattern in DEEPLINK_PATTERNS {
        if let Ok(re) = Regex::new(pattern.pattern) {
            for (line_idx, line) in content.lines().enumerate() {
                if let Some(mat) = re.find(line) {
                    findings.push(Finding {
                        id: format!("{}-{}", pattern.id, line_idx + 1),
                        rule_id: pattern.id.to_string(),
                        title: pattern.title.to_string(),
                        vuln_class: pattern.masvs.to_string(),
                        severity: pattern.severity,
                        platform: Platform::Android,
                        file_path: file_path.to_string(),
                        line: line_idx + 1,
                        snippet: mat.as_str().trim().to_string(),
                        description: pattern.description.to_string(),
                        impact: format!("Violates {} ({}). Risk of unauthorized component access or intent injection.", pattern.masvs, pattern.cwe),
                        remediation: pattern.remediation.to_string(),
                        cvss: CvssData {
                            score: pattern.cvss,
                            severity: pattern.severity,
                            vector: "CVSS:3.1/AV:L/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:N".to_string(),
                            exploitability: 1.8,
                            impact: 5.2,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec![format!("Matches Intent pattern {}", pattern.id)],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Inspect intent handling at `{}:{}`.", file_path, line_idx + 1),
                            format!("Identified code: `{}`", mat.as_str().trim()),
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
    fn test_intent_redirect() {
        let code = r#"
            Intent forwardIntent = (Intent) intent.getParcelableExtra("android.intent.extra.INTENT");
            startActivity((Intent) intent.getParcelableExtra("target_intent"));
        "#;
        let findings = audit_deeplinks_and_intents(code, "ForwardingActivity.java");
        assert!(findings.iter().any(|f| f.rule_id == "INTENT-REDIRECT"));
    }
}
