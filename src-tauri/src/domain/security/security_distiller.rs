use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// A single per-line security finding, shaped for inline editor decoration
/// (gutter marker + squiggle). This is the "security lens" — live vuln
/// detection as you type, which Cursor has no native equivalent for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    /// 1-indexed line number.
    pub line: usize,
    /// 0-indexed byte column where the match starts (best-effort).
    pub column: usize,
    /// "CRITICAL" | "HIGH" | "MEDIUM" | "LOW".
    pub severity: String,
    /// Short kebab category, e.g. "hardcoded-secret", "command-injection".
    pub category: String,
    /// Human-readable one-line explanation.
    pub message: String,
    /// CWE id when applicable, e.g. "CWE-798".
    pub cwe: Option<String>,
}

/// (pattern, severity, category, message, cwe). Language-agnostic sink/secret
/// heuristics — deliberately high-signal so the live lens stays quiet on clean
/// code. Deeper analysis is delegated to semgrep / the DeepHat threat engine.
const RULES: &[(&str, &str, &str, &str, Option<&str>)] = &[
    ("mem::transmute", "HIGH", "unsafe-transmute", "Memory transmute — verify layout/lifetime invariants", Some("CWE-843")),
    ("eval(", "CRITICAL", "code-injection", "Dynamic eval of input enables code injection", Some("CWE-95")),
    ("exec(", "HIGH", "command-injection", "Dynamic exec — sanitize/allowlist arguments", Some("CWE-78")),
    ("os.system(", "HIGH", "command-injection", "Shell command from variable — command injection risk", Some("CWE-78")),
    ("subprocess.call(", "MEDIUM", "command-injection", "Subprocess with shell input — verify no injection", Some("CWE-78")),
    ("shell=True", "HIGH", "command-injection", "shell=True passes input to a shell — injection risk", Some("CWE-78")),
    ("md5", "MEDIUM", "weak-crypto", "MD5 is broken for security use", Some("CWE-327")),
    ("sha1", "MEDIUM", "weak-crypto", "SHA-1 is broken for security use", Some("CWE-327")),
    ("verify=False", "HIGH", "tls-verification-disabled", "TLS verification disabled — MITM risk", Some("CWE-295")),
    ("rejectUnauthorized: false", "HIGH", "tls-verification-disabled", "TLS verification disabled — MITM risk", Some("CWE-295")),
    ("dangerouslySetInnerHTML", "MEDIUM", "xss", "Unescaped HTML injection — XSS if input is untrusted", Some("CWE-79")),
    ("innerHTML", "MEDIUM", "xss", "innerHTML with untrusted input enables XSS", Some("CWE-79")),
    ("pickle.loads", "HIGH", "unsafe-deserialization", "Unpickling untrusted data executes code", Some("CWE-502")),
    ("yaml.load(", "MEDIUM", "unsafe-deserialization", "yaml.load without SafeLoader can execute code", Some("CWE-502")),
];

/// Secret-key regexes: (label, prefix). Kept as literal-prefix scans for speed
/// on the keystroke path; entropy checks are left to the offline secrets_scan.
const SECRET_PREFIXES: &[(&str, &str)] = &[
    ("aws-access-key", "AKIA"),
    ("github-token", "ghp_"),
    ("github-pat", "github_pat_"),
    ("openai-key", "sk-"),
    ("slack-token", "xox"),
    ("private-key-block", "-----BEGIN"),
];

pub struct SecurityDistiller;

impl SecurityDistiller {
    /// Audits a file content for high-risk patterns and returns a "Risk Weight".
    /// This weight is used to adjust the Pythagorean geometric coordinates' priority.
    pub fn audit_content(content: &str) -> f32 {
        let mut risk_score: f32 = 0.0;

        // Pattern 1: Unsafe Usage (Traditional CVE source)
        if content.contains("unsafe {") {
            risk_score += 0.4;
        }

        // Pattern 2: Panic Prone Logic
        if content.contains(".unwrap()") || content.contains(".expect(") {
            risk_score += 0.2;
        }

        // Pattern 3: Sensitive Data Handling (Mock pattern)
        if content.contains("password") || content.contains("secret") || content.contains("token") {
            risk_score += 0.3;
        }

        // Pattern 4: Memory Transmutation
        if content.contains("mem::transmute") {
            risk_score += 0.5;
        }

        risk_score.min(1.0)
    }

    /// Per-line security lens. Returns structured findings the editor can render
    /// as inline gutter markers + squiggles, live as the user types. Fast enough
    /// for the keystroke path (linear scan, literal matches); deep analysis is
    /// delegated to `run_semgrep` and the DeepHat threat engine on demand.
    pub fn scan_lines(content: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        for (idx, raw) in content.lines().enumerate() {
            let line_no = idx + 1;
            // Skip obvious comment-only lines to cut false positives on notes/docs.
            let trimmed = raw.trim_start();
            let is_comment = trimmed.starts_with("//")
                || trimmed.starts_with('#')
                || trimmed.starts_with('*')
                || trimmed.starts_with("<!--");
            let lower = raw.to_lowercase();

            for (needle, severity, category, message, cwe) in RULES {
                // weak-crypto hashes appear as identifiers; match case-insensitively.
                let hit = if *category == "weak-crypto" {
                    lower.find(needle)
                } else {
                    raw.find(needle)
                };
                if let Some(col) = hit {
                    if is_comment && *severity != "CRITICAL" {
                        continue;
                    }
                    findings.push(SecurityFinding {
                        line: line_no,
                        column: col,
                        severity: severity.to_string(),
                        category: category.to_string(),
                        message: message.to_string(),
                        cwe: cwe.map(|c| c.to_string()),
                    });
                }
            }

            for (label, prefix) in SECRET_PREFIXES {
                if let Some(col) = raw.find(prefix) {
                    findings.push(SecurityFinding {
                        line: line_no,
                        column: col,
                        severity: "CRITICAL".to_string(),
                        category: "hardcoded-secret".to_string(),
                        message: format!("Possible hardcoded {} — move to a secret store / env var", label),
                        cwe: Some("CWE-798".to_string()),
                    });
                }
            }
        }
        findings
    }

    /// Generates security metadata for the context indexer.
    pub fn get_security_metadata(content: &str) -> serde_json::Value {
        let risk = Self::audit_content(content);
        json!({
            "risk_level": risk,
            "security_tier": if risk > 0.7 { "CRITICAL" } else if risk > 0.3 { "ELEVATED" } else { "STANDARD" },
            "audit_needed": risk > 0.5
        })
    }

    /// Runs semgrep on a file/directory and returns the output.
    pub fn run_semgrep(path: &std::path::Path) -> Result<Value, String> {
        let output = std::process::Command::new("semgrep")
            .arg("scan")
            .arg("--json")
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to execute semgrep: {}. Is it installed?", e))?;

        if !output.status.success() && output.stderr.len() > 0 {
             return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        serde_json::from_slice(&output.stdout).map_err(|e| format!("Failed to parse semgrep JSON: {}", e))
    }

    /// Runs cargo audit on the current workspace and returns the output.
    pub fn run_cargo_audit(workspace_root: &std::path::Path) -> Result<Value, String> {
        let output = std::process::Command::new("cargo")
            .arg("audit")
            .arg("--json")
            .current_dir(workspace_root)
            .output()
            .map_err(|e| format!("Failed to execute cargo audit: {}. Is it installed?", e))?;

        serde_json::from_slice(&output.stdout).map_err(|e| format!("Failed to parse cargo audit JSON: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has(findings: &[SecurityFinding], category: &str) -> bool {
        findings.iter().any(|f| f.category == category)
    }

    #[test]
    fn detects_hardcoded_aws_secret() {
        let src = "let key = \"AKIAIOSFODNN7EXAMPLE\";";
        let f = SecurityDistiller::scan_lines(src);
        assert!(has(&f, "hardcoded-secret"), "should flag AKIA secret: {:?}", f);
        assert!(f.iter().any(|x| x.severity == "CRITICAL" && x.line == 1));
    }

    #[test]
    fn detects_command_injection() {
        let src = "import os\nos.system(user_input)\n";
        let f = SecurityDistiller::scan_lines(src);
        assert!(has(&f, "command-injection"));
        // The finding must point at line 2, not line 1.
        assert!(f.iter().any(|x| x.category == "command-injection" && x.line == 2));
    }

    #[test]
    fn weak_crypto_is_case_insensitive() {
        let f = SecurityDistiller::scan_lines("let h = MD5(data);");
        assert!(has(&f, "weak-crypto"), "MD5 uppercase should match: {:?}", f);
    }

    #[test]
    fn skips_noncritical_matches_in_comments() {
        // A MEDIUM rule (md5) inside a comment must be suppressed…
        let commented = SecurityDistiller::scan_lines("# md5 is considered weak");
        assert!(!has(&commented, "weak-crypto"), "comment md5 should be skipped: {:?}", commented);
        // …but a CRITICAL secret is never suppressed, even in a comment.
        let secret_comment = SecurityDistiller::scan_lines("# leftover key AKIAIOSFODNN7EXAMPLE");
        assert!(has(&secret_comment, "hardcoded-secret"));
    }

    #[test]
    fn clean_code_yields_no_findings() {
        let f = SecurityDistiller::scan_lines("let total = a + b;\nprintln!(\"{}\", total);");
        assert!(f.is_empty(), "clean code should be quiet: {:?}", f);
    }

    #[test]
    fn column_is_reported() {
        let f = SecurityDistiller::scan_lines("    eval(payload)");
        let hit = f.iter().find(|x| x.category == "code-injection").expect("eval should match");
        assert_eq!(hit.column, 4, "column should be the byte offset of the match");
    }
}
