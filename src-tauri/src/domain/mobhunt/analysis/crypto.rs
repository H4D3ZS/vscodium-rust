use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct CryptoPattern {
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

pub static CRYPTO_PATTERNS: &[CryptoPattern] = &[
    CryptoPattern {
        id: "CRYPTO-WEAK-DES",
        title: "Weak/Deprecated Cipher Algorithm: DES / 3DES",
        pattern: r#"(?i)Cipher\.getInstance\s*\(\s*["'](?:DES|DESede|3DES)(?:/[^"']*)?["']\s*\)"#,
        severity: Severity::Critical,
        cvss: 8.5,
        cwe: "CWE-327",
        masvs: "MASVS-CRYPTO-1",
        description: "Application uses deprecated DES or Triple-DES (DESede) cipher algorithms. DES has an effective 56-bit key length and can be brute-forced in hours. 3DES is vulnerable to Sweet32 collision attacks.",
        remediation: "Migrate to AES-256-GCM or ChaCha20-Poly1305 authenticated encryption.",
    },
    CryptoPattern {
        id: "CRYPTO-WEAK-RC4",
        title: "Broken Cipher Algorithm: RC4 / ARCFOUR",
        pattern: r#"(?i)Cipher\.getInstance\s*\(\s*["'](?:RC4|ARCFOUR)(?:/[^"']*)?["']\s*\)"#,
        severity: Severity::Critical,
        cvss: 8.5,
        cwe: "CWE-327",
        masvs: "MASVS-CRYPTO-1",
        description: "Application uses the RC4 stream cipher. RC4 possesses severe statistical biases in keystream bytes (RFC 7465 prohibits its use in TLS).",
        remediation: "Replace RC4 with AES-GCM or ChaCha20-Poly1305.",
    },
    CryptoPattern {
        id: "CRYPTO-INSECURE-ECB",
        title: "Insecure Cipher Mode: ECB (Electronic Codebook)",
        pattern: r#"(?i)Cipher\.getInstance\s*\(\s*["'](?:AES/ECB/[^"']+|AES|DES/ECB/[^"']+)["']\s*\)"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-327",
        masvs: "MASVS-CRYPTO-1",
        description: "Application encrypts data using ECB (Electronic Codebook) mode. ECB encrypts identical plaintext blocks into identical ciphertext blocks, preserving data patterns and allowing block reordering/replay attacks.",
        remediation: "Use authenticated cipher modes: AES/GCM/NoPadding.",
    },
    CryptoPattern {
        id: "CRYPTO-BROKEN-MD5",
        title: "Broken Hash Function: MD5",
        pattern: r#"(?i)MessageDigest\.getInstance\s*\(\s*["']MD5["']\s*\)|DigestUtils\.md5Hex\("#,
        severity: Severity::Medium,
        cvss: 5.9,
        cwe: "CWE-328",
        masvs: "MASVS-CRYPTO-1",
        description: "Application generates cryptographic hashes using MD5. MD5 is completely broken due to collision vulnerabilities and should never be used for security purposes, passwords, or digital signatures.",
        remediation: "Use SHA-256, SHA-384, SHA-512, or SHA-3.",
    },
    CryptoPattern {
        id: "CRYPTO-BROKEN-SHA1",
        title: "Deprecated Hash Function: SHA-1",
        pattern: r#"(?i)MessageDigest\.getInstance\s*\(\s*["']SHA-?1["']\s*\)|DigestUtils\.sha1Hex\("#,
        severity: Severity::Low,
        cvss: 3.7,
        cwe: "CWE-328",
        masvs: "MASVS-CRYPTO-1",
        description: "Application uses SHA-1. SHA-1 is deprecated by NIST due to practical collision attacks.",
        remediation: "Migrate to SHA-256 or SHA-3.",
    },
    CryptoPattern {
        id: "CRYPTO-STATIC-IV",
        title: "Static or Hardcoded Initialization Vector (IV)",
        pattern: r#"(?i)IvParameterSpec\s*\(\s*(?:new\s+byte\s*\[\s*\d+\s*\]|"[0-9a-zA-Z]{16}"\.getBytes\(\))\s*\)"#,
        severity: Severity::High,
        cvss: 7.4,
        cwe: "CWE-329",
        masvs: "MASVS-CRYPTO-1",
        description: "Application initializes cipher with a static, all-zero, or hardcoded IV (`new byte[16]` or static string). Reusing IVs with modes like CBC or GCM allows plaintext recovery and breaks confidentiality.",
        remediation: "Generate a cryptographically secure random IV for every encryption operation using `SecureRandom.nextBytes(iv)` and prepend the IV to the ciphertext.",
    },
    CryptoPattern {
        id: "CRYPTO-WEAK-PRNG",
        title: "Insecure Random Number Generator Used in Security Context",
        pattern: r#"(?i)new\s+java\.util\.Random\(\)|Random\s+\w+\s*=\s*new\s+Random\(\)"#,
        severity: Severity::Medium,
        cvss: 5.3,
        cwe: "CWE-338",
        masvs: "MASVS-CRYPTO-1",
        description: "Application uses `java.util.Random` for random number generation. `java.util.Random` is a linear congruential generator whose internal state can be deduced from a few consecutive outputs.",
        remediation: "Use `java.security.SecureRandom` for all security-sensitive tokens, nonces, passwords, and cryptographic keys.",
    },
];

/// Scans source files for cryptographic vulnerabilities
pub fn audit_cryptography(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for pattern in CRYPTO_PATTERNS {
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
                        impact: format!("Violates {} ({}). Compromises data confidentiality and integrity.", pattern.masvs, pattern.cwe),
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
                            notes: vec![format!("Matches Cryptography pattern {}", pattern.id)],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Inspect cryptography initialization at `{}:{}`.", file_path, line_idx + 1),
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
    fn test_weak_crypto_detection() {
        let code = r#"
            Cipher c = Cipher.getInstance("DES/ECB/PKCS5Padding");
            IvParameterSpec iv = new IvParameterSpec(new byte[16]);
            MessageDigest md = MessageDigest.getInstance("MD5");
        "#;
        let findings = audit_cryptography(code, "CryptoUtils.java");
        assert!(findings.iter().any(|f| f.rule_id == "CRYPTO-WEAK-DES"));
        assert!(findings.iter().any(|f| f.rule_id == "CRYPTO-INSECURE-ECB"));
        assert!(findings.iter().any(|f| f.rule_id == "CRYPTO-STATIC-IV"));
        assert!(findings.iter().any(|f| f.rule_id == "CRYPTO-BROKEN-MD5"));
    }
}
