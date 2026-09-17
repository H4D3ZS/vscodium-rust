use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct SslPinPattern {
    pub id: &'static str,
    pub title: &'static str,
    pub pattern: &'static str,
    pub severity: Severity,
    pub cvss: f64,
    pub platform: Platform,
    pub description: &'static str,
    pub remediation: &'static str,
}

pub static SSL_PIN_PATTERNS: &[SslPinPattern] = &[
    SslPinPattern {
        id: "SSL-PIN-OKHTTP",
        title: "OkHttp Certificate Pinning Detected",
        pattern: r#"(?i)CertificatePinner\.Builder\(\)|Lokhttp3/CertificatePinner;|Lcom/squareup/okhttp/CertificatePinner;|setCertificatePinner\("#,
        severity: Severity::Info,
        cvss: 0.0,
        platform: Platform::Android,
        description: "Application implements OkHttp CertificatePinner. This verifies public key hashes or certificates directly in code.",
        remediation: "Maintain backup pin sets to prevent denial of service during emergency certificate rotations.",
    },
    SslPinPattern {
        id: "SSL-PIN-TRUSTMANAGER",
        title: "Custom X509TrustManager SSL Verification Override",
        pattern: r#"(?i)Ljavax/net/ssl/X509TrustManager;|implements\s+X509TrustManager|checkServerTrusted\s*\("#,
        severity: Severity::Medium,
        cvss: 5.9,
        platform: Platform::Android,
        description: "Application declares custom X509TrustManager and checkServerTrusted implementation. If empty or improperly validated, TLS certificate checks will be completely bypassed.",
        remediation: "Avoid implementing custom X509TrustManager; use Android Network Security Configuration to specify custom CAs and pins declaratively.",
    },
    SslPinPattern {
        id: "SSL-PIN-NSC",
        title: "Network Security Config Certificate Pinning Declared",
        pattern: r#"(?i)<pin-set|<pin\s+digest="SHA-256"|<trust-anchors>"#,
        severity: Severity::Info,
        cvss: 0.0,
        platform: Platform::Android,
        description: "Application declares declarative certificate pinning via Android Network Security Configuration (network_security_config.xml).",
        remediation: "Ensure pins include backup keys (e.g. disaster recovery intermediate or root CA) and expiration dates if applicable.",
    },
    SslPinPattern {
        id: "SSL-PIN-CONSCRYPT",
        title: "Conscrypt High-Performance SSL Engine Detected",
        pattern: r#"(?i)Lorg/conscrypt/ConscryptEngineSocket;|Lorg/conscrypt/OpenSSLContextImpl;"#,
        severity: Severity::Info,
        cvss: 0.0,
        platform: Platform::Android,
        description: "Application bundles Conscrypt as its security provider for TLS / BoringSSL connections.",
        remediation: "Ensure Conscrypt library dependencies are kept up to date.",
    },
    SslPinPattern {
        id: "SSL-PIN-CUSTOM",
        title: "Custom SSL Pinning Class Located",
        pattern: r#"(?i)class\s+\w*(?:PinningTrustManager|SSLPinning|CertificatePinning|PublicKeyPinner)"#,
        severity: Severity::Low,
        cvss: 3.1,
        platform: Platform::CrossPlatform,
        description: "Custom SSL pinning or certificate validation class detected in application codebase.",
        remediation: "Audit custom pinning logic to ensure failure does not fall back to unpinned plain TLS.",
    },
    SslPinPattern {
        id: "SSL-PIN-TRUSTKIT-IOS",
        title: "iOS TrustKit SSL Pinning Framework Detected",
        pattern: r#"(?i)TrustKit\.initSharedInstance|TrustKit\.initializeWithConfiguration|kTSKPublicKeyHashes|kTSKEnforcePinning"#,
        severity: Severity::Info,
        cvss: 0.0,
        platform: Platform::Ios,
        description: "Application integrates TrustKit for iOS SSL public key pinning and reporting.",
        remediation: "Ensure kTSKEnforcePinning is enabled in production builds and report-uri endpoints are monitored.",
    },
    SslPinPattern {
        id: "SSL-PIN-SECTRUST-IOS",
        title: "iOS Low-Level SecTrustEvaluate SSL Pinning Implementation",
        pattern: r#"(?i)SecTrustEvaluateWithError|SecTrustEvaluate\(|SecTrustCopyPublicKey|serverTrustPolicy"#,
        severity: Severity::Low,
        cvss: 3.7,
        platform: Platform::Ios,
        description: "Application performs low-level Security framework certificate evaluation using SecTrustEvaluate or SecTrustEvaluateWithError.",
        remediation: "Validate SecTrustResultType handling to ensure untrusted certificates or recovery states do not allow connection establishment.",
    },
];

/// Scans decompiled code, smali, or source files for SSL pinning implementations
pub fn audit_ssl_pinning(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for pattern in SSL_PIN_PATTERNS {
        if let Ok(re) = Regex::new(pattern.pattern) {
            for (line_idx, line) in content.lines().enumerate() {
                if let Some(mat) = re.find(line) {
                    findings.push(Finding {
                        id: format!("{}-{}", pattern.id, line_idx + 1),
                        rule_id: pattern.id.to_string(),
                        title: pattern.title.to_string(),
                        vuln_class: "MASVS-NETWORK-3".to_string(),
                        severity: pattern.severity,
                        platform: pattern.platform,
                        file_path: file_path.to_string(),
                        line: line_idx + 1,
                        snippet: mat.as_str().trim().to_string(),
                        description: pattern.description.to_string(),
                        impact: "Identifies TLS certificate pinning architecture and reverse engineering / dynamic hooking attack surface.".to_string(),
                        remediation: pattern.remediation.to_string(),
                        cvss: CvssData {
                            score: pattern.cvss,
                            severity: pattern.severity,
                            vector: "CVSS:3.1/AV:L/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:N".to_string(),
                            exploitability: 1.0,
                            impact: 1.0,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec![format!("Matches SSL pinning pattern {}", pattern.id)],
                            chain_eligible: pattern.severity >= Severity::Medium,
                        },
                        reproduction_steps: vec![
                            format!("Inspect code at `{}:{}`.", file_path, line_idx + 1),
                            format!("Match identified: `{}`", mat.as_str().trim()),
                        ],
                        metadata: HashMap::new(),
                    });
                    // One finding per rule per file to avoid flooding
                    break;
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
    fn test_okhttp_detection() {
        let code = r#"
            OkHttpClient client = new OkHttpClient.Builder()
                .certificatePinner(new CertificatePinner.Builder()
                    .add("api.example.com", "sha256/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=")
                    .build())
                .build();
        "#;
        let findings = audit_ssl_pinning(code, "ApiClient.java");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "SSL-PIN-OKHTTP");
    }
}
