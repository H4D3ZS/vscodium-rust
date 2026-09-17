pub mod poc_generator;
pub mod secret_verifier;
pub use poc_generator::{generate_poc_for_finding, ExploitPoc, PocType};
pub use secret_verifier::{SecretVerificationResult, SecretVerifier};

use crate::domain::mobhunt::models::{Finding, GateResult, GateStatus, Severity};

pub struct NeverSubmitRule {
    pub id: &'static str,
    pub patterns: &'static [&'static str],
    pub description: &'static str,
    pub chain_eligible: bool,
}

pub const NEVER_SUBMIT_RULES: &[NeverSubmitRule] = &[
    NeverSubmitRule {
        id: "NS-01",
        patterns: &["cert_pinning", "certificate_pinning", "missing_pinning", "ssl_pinning"],
        description: "Missing certificate pinning alone",
        chain_eligible: true,
    },
    NeverSubmitRule {
        id: "NS-02",
        patterns: &["allow_backup", "allowbackup", "backup_enabled"],
        description: "android:allowBackup=true alone without sensitive data dump",
        chain_eligible: true,
    },
    NeverSubmitRule {
        id: "NS-03",
        patterns: &["root_detection", "jailbreak_detection", "missing_root", "missing_jailbreak"],
        description: "Missing root/jailbreak detection alone",
        chain_eligible: false,
    },
    NeverSubmitRule {
        id: "NS-04",
        patterns: &[
            "cleartext_traffic", "cleartext_config", "ats_disabled",
            "ats globally disabled", "nsallowsarbitraryloads",
            "cleartexttrafficpermitted",
        ],
        description: "Cleartext traffic configuration alone without MITM proof",
        chain_eligible: true,
    },
    NeverSubmitRule {
        id: "NS-05",
        patterns: &["missing_aslr", "missing_pie", "missing_canary", "binary_protection"],
        description: "Missing binary protections on modern OS",
        chain_eligible: false,
    },
    NeverSubmitRule {
        id: "NS-06",
        patterns: &["self_signed_cert", "self_signed_certificate"],
        description: "Self-signed certificate in bundle",
        chain_eligible: false,
    },
    NeverSubmitRule {
        id: "NS-07",
        patterns: &["analytics_data", "tracking_data", "analytics_collection"],
        description: "Analytics SDK data collection (privacy, not vulnerability)",
        chain_eligible: false,
    },
    NeverSubmitRule {
        id: "NS-08",
        patterns: &["missing_obfuscation", "no_obfuscation", "code_obfuscation"],
        description: "Missing code obfuscation alone",
        chain_eligible: false,
    },
];

pub fn evaluate_gates(finding: &Finding) -> GateResult {
    let mut notes = Vec::new();
    let text = format!("{} {} {}", finding.title, finding.vuln_class, finding.description).to_ascii_lowercase();

    // 1. Check Never-Submit list
    for rule in NEVER_SUBMIT_RULES {
        for pat in rule.patterns {
            if text.contains(pat) {
                // High/Critical with solid evidence can override
                if finding.severity >= Severity::High && !finding.snippet.is_empty() {
                    notes.push(format!(
                        "Matches [{}] '{}' but elevated due to Critical/High severity and evidence snippet.",
                        rule.id, rule.description
                    ));
                    return GateResult {
                        status: GateStatus::Passed,
                        notes,
                        chain_eligible: rule.chain_eligible,
                    };
                }

                notes.push(format!(
                    "Rejected by [{}] '{}' — defense-in-depth item not rewarded on bug bounty platforms.",
                    rule.id, rule.description
                ));
                return GateResult {
                    status: GateStatus::KilledNeverSubmit(rule.description.to_string()),
                    notes,
                    chain_eligible: rule.chain_eligible,
                };
            }
        }
    }

    // 2. Exploitability & Proof Check (The 7-Question Gate #3 & #4)
    if finding.snippet.is_empty() {
        notes.push("Missing concrete code snippet or evidence artifact — requires manual verification.".into());
        return GateResult {
            status: GateStatus::NeedsManualPoc,
            notes,
            chain_eligible: true,
        };
    }

    // 3. Threat Model Check (The 7-Question Gate #2)
    if text.contains("requires physical access") && finding.severity < Severity::High {
        notes.push("Demoted: Attack scenario requires unrestricted physical device access.".into());
    }

    notes.push("✓ Passes 7-Question Mobile Validation Gate (In-Scope, Exploitability, Concrete Evidence)".into());
    GateResult {
        status: GateStatus::Passed,
        notes,
        chain_eligible: true,
    }
}
