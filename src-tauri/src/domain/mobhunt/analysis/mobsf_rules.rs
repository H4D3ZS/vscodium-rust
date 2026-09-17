use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct MobSfRule {
    pub id: &'static str,
    pub title: &'static str,
    pub pattern: &'static str,
    pub severity: Severity,
    pub cvss: f64,
    pub cwe: &'static str,
    pub masvs: &'static str,
    pub platform: Platform,
    pub description: &'static str,
    pub remediation: &'static str,
}

pub static MOBSF_SAST_RULES: &[MobSfRule] = &[
    // ═══ Android SAST Rules (from android_rules.yaml) ═══
    MobSfRule {
        id: "MOBSF-AND-01",
        title: "Insecure Implementation of SSL (TrustAllSSLSocket / NonValidating)",
        pattern: r#"(?i)TrustAllSSLSocket|AllTrustSSLSocketFactory|NonValidatingSSLSocketFactory|ALLOW_ALL_HOSTNAME_VERIFIER|\.setDefaultHostnameVerifier\(|NullHostnameVerifier\("#,
        severity: Severity::High,
        cvss: 7.4,
        cwe: "CWE-295",
        masvs: "MASVS-NETWORK-3",
        platform: Platform::Android,
        description: "Application implements custom SSL socket factories or hostname verifiers that trust all certificates or accept self-signed certificates. This completely neutralizes TLS protections.",
        remediation: "Use standard system trust managers and default hostname verification. Do not override HostnameVerifier with empty implementations.",
    },
    MobSfRule {
        id: "MOBSF-AND-02",
        title: "WebView Loads Files from External Storage",
        pattern: r#"(?i)\.loadUrl\(.*getExternalStorageDirectory\("#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-919",
        masvs: "MASVS-PLATFORM-6",
        platform: Platform::Android,
        description: "WebView loads files directly from external storage. Any application on the device with storage permissions can modify or swap these files to execute malicious JavaScript.",
        remediation: "Load assets exclusively from internal assets (asset://) or internal private storage directories.",
    },
    MobSfRule {
        id: "MOBSF-AND-03",
        title: "World-Readable / World-Writable File Creation",
        pattern: r#"(?i)MODE_WORLD_READABLE|MODE_WORLD_WRITEABLE"#,
        severity: Severity::Critical,
        cvss: 8.4,
        cwe: "CWE-276",
        masvs: "MASVS-STORAGE-1",
        platform: Platform::Android,
        description: "Application creates private files using deprecated MODE_WORLD_READABLE or MODE_WORLD_WRITEABLE flags, allowing any app installed on the device to read or corrupt sensitive application data.",
        remediation: "Use Context.MODE_PRIVATE for all file and SharedPreferences operations.",
    },
    MobSfRule {
        id: "MOBSF-AND-04",
        title: "Insecure Cryptographic PRNG Seed",
        pattern: r#"(?i)SecureRandom\.setSeed\(|setSeed\(.*System\.currentTimeMillis\(\)"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-330",
        masvs: "MASVS-CRYPTO-6",
        platform: Platform::Android,
        description: "Calling setSeed() on SecureRandom with static or predictable data (such as current system timestamp) reduces entropy and makes generated keys predictable.",
        remediation: "Allow SecureRandom to seed itself from the OS entropy source (/dev/urandom).",
    },
    MobSfRule {
        id: "MOBSF-AND-05",
        title: "Insecure Local SQLite Database Raw Execution",
        pattern: r#"(?i)rawQuery\(.*(\+|concat).*|execSQL\(.*(\+|concat).*"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-89",
        masvs: "MASVS-STORAGE-2",
        platform: Platform::Android,
        description: "SQLite database queries constructed by concatenating user inputs into rawQuery or execSQL are vulnerable to SQL Injection.",
        remediation: "Use parameterized queries with selectionArgs or standard Room ORM persistence.",
    },

    // ═══ iOS Swift & ObjC Rules (from swift_rules.yaml / objective_c_rules.yaml) ═══
    MobSfRule {
        id: "MOBSF-IOS-01",
        title: "Insecure WebView JavaScript Injection via loadHTMLString",
        pattern: r#"(?i)loadHTMLString\(.*webView"#,
        severity: Severity::High,
        cvss: 8.8,
        cwe: "CWE-95",
        masvs: "MASVS-PLATFORM-5",
        platform: Platform::Ios,
        description: "Unsanitized input rendered in a WKWebView via loadHTMLString can lead to cross-site scripting (XSS) and access to native bridge functions.",
        remediation: "Sanitize HTML strings and ensure WKWebView content security policies (CSP) restrict script execution.",
    },
    MobSfRule {
        id: "MOBSF-IOS-02",
        title: "Insecure Keychain Accessibility Level (kSecAttrAccessibleAlways)",
        pattern: r#"(?i)kSecAttrAccessibleAlways|kSecAttrAccessibleAlwaysThisDeviceOnly"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-312",
        masvs: "MASVS-STORAGE-1",
        platform: Platform::Ios,
        description: "Keychain item is stored with kSecAttrAccessibleAlways, meaning data can be decrypted even when the device is locked or rebooted.",
        remediation: "Use kSecAttrAccessibleAfterFirstUnlock or kSecAttrAccessibleWhenUnlockedThisDeviceOnly for sensitive credentials.",
    },
    MobSfRule {
        id: "MOBSF-IOS-03",
        title: "allowsAnyHTTPSCertificate / Disabling TLS Validation",
        pattern: r#"(?i)allowsAnyHTTPSCertificate|allowsAnyHTTPSCertificateForHost|continueWithoutCredentialForAuthenticationChallenge"#,
        severity: Severity::Critical,
        cvss: 8.6,
        cwe: "CWE-295",
        masvs: "MASVS-NETWORK-3",
        platform: Platform::Ios,
        description: "Application bypasses standard SSL/TLS certificate validation in NSURLSession or NSURLConnection delegate methods.",
        remediation: "Do not override server trust evaluation; use SecTrustEvaluateWithError to validate standard certificates.",
    },
    MobSfRule {
        id: "MOBSF-IOS-04",
        title: "Weak Hash Algorithm (MD5 / SHA1 in Security Context)",
        pattern: r#"(?i)CC_MD5\(|CC_SHA1\(|CommonCrypto\.CC_MD5"#,
        severity: Severity::Medium,
        cvss: 5.3,
        cwe: "CWE-327",
        masvs: "MASVS-CRYPTO-4",
        platform: Platform::Ios,
        description: "Application uses MD5 or SHA1 algorithms for hashing. Both suffer from known collision vulnerabilities.",
        remediation: "Upgrade to SHA-256 (CC_SHA256) or SHA-3 for hashing, and bcrypt/Argon2 for credential verification.",
    },
];

pub fn scan_mobsf_rules(file_path: &str, content: &str, target_platform: Platform) -> Vec<Finding> {
    let mut findings = Vec::new();

    for rule in MOBSF_SAST_RULES {
        if rule.platform != Platform::CrossPlatform && rule.platform != target_platform && target_platform != Platform::CrossPlatform {
            continue;
        }

        if let Ok(re) = Regex::new(rule.pattern) {
            for (line_idx, line) in content.lines().enumerate() {
                if let Some(mat) = re.find(line) {
                    findings.push(Finding {
                        id: format!("{}-{}", rule.id, line_idx + 1),
                        rule_id: rule.id.to_string(),
                        title: rule.title.to_string(),
                        vuln_class: rule.masvs.to_string(),
                        severity: rule.severity,
                        platform: rule.platform,
                        file_path: file_path.to_string(),
                        line: line_idx + 1,
                        snippet: mat.as_str().trim().to_string(),
                        description: rule.description.to_string(),
                        impact: format!("Violates {} ({}). High security risk in production mobile binaries.", rule.masvs, rule.cwe),
                        remediation: rule.remediation.to_string(),
                        cvss: CvssData {
                            score: rule.cvss,
                            severity: rule.severity,
                            vector: format!("CVSS:3.1/AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N"),
                            exploitability: 2.5,
                            impact: 4.0,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec![format!("Matches verified MobSF SAST rule {}", rule.id)],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Inspect source code at `{}:{}`.", file_path, line_idx + 1),
                            format!("Verify occurrence of `{}`.", mat.as_str().trim()),
                        ],
                        metadata: HashMap::new(),
                    });
                }
            }
        }
    }

    findings
}
