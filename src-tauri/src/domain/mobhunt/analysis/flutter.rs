use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use crate::domain::mobhunt::analysis::shared::scan_secrets;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub struct FlutterAuditResult {
    pub findings: Vec<Finding>,
    pub engine_version_or_hash: Option<String>,
    pub endpoints: Vec<String>,
    pub dart_classes: Vec<String>,
    pub secrets: Vec<Finding>,
}

/// Audits Flutter binary assets (`libflutter.so`, `libapp.so`, `Flutter.framework`, `App.framework`)
pub fn audit_flutter_binary(file_path: &Path, bytes: &[u8]) -> FlutterAuditResult {
    let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let file_name_lower = file_name.to_ascii_lowercase();

    let mut findings = Vec::new();
    let mut endpoints = Vec::new();
    let mut dart_classes = Vec::new();
    let mut secrets = Vec::new();
    let mut engine_hash = None;

    let path_str = file_path.display().to_string();

    // 1. Check if this is the Flutter Engine binary (`libflutter.so` or `Flutter.framework/Flutter`)
    if file_name_lower == "libflutter.so" || file_name == "Flutter" {
        engine_hash = extract_flutter_engine_hash(bytes);

        if let Some(ref hash) = engine_hash {
            findings.push(Finding {
                id: format!("FLUTTER-ENG-{}", &hash[..8]),
                rule_id: "FLUTTER-ENG-HASH".to_string(),
                title: format!("Flutter Engine Hash Identified ({})", &hash[..8]),
                vuln_class: "MASVS-PLATFORM-1".to_string(),
                severity: Severity::Info,
                platform: Platform::CrossPlatform,
                file_path: path_str.clone(),
                line: 1,
                snippet: format!("Commit Hash: {}", hash),
                description: format!(
                    "Discovered Flutter Engine git commit hash: {}. This commit identifies the exact Flutter framework revision and Dart SDK version used to compile the application.",
                    hash
                ),
                impact: "Enables precise symbol resolution and binary patch diffing against the upstream Flutter engine repository.".to_string(),
                remediation: "Ensure the Flutter engine is kept up to date with the latest stable release containing engine security patches.".to_string(),
                cvss: CvssData {
                    score: 0.0,
                    severity: Severity::Info,
                    vector: "CVSS:3.1/AV:L/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:N".to_string(),
                    exploitability: 0.0,
                    impact: 0.0,
                },
                gate: GateResult {
                    status: GateStatus::Passed,
                    notes: vec!["Engine identification complete".to_string()],
                    chain_eligible: false,
                },
                reproduction_steps: vec![
                    format!("Examine binary strings in `{}` for Flutter commit hash.", file_name),
                    format!("Verify engine commit at `https://github.com/flutter/engine/commit/{}`", hash),
                ],
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("engine_hash".to_string(), hash.clone());
                    m
                },
            });
        }

        // Check for BoringSSL / SSL Verification routine symbols
        if let Some(pinning_finding) = check_flutter_ssl_symbols(bytes, &path_str) {
            findings.push(pinning_finding);
        }
    }

    // 2. Check if this is the Dart AOT Snapshot binary (`libapp.so` or `App.framework/App`)
    if file_name_lower == "libapp.so" || file_name == "App" {
        let extracted_strings = extract_ascii_and_utf8_strings(bytes, 4);

        // Scan strings for secrets
        let joined_content = extracted_strings.join("\n");
        let mut string_secrets = scan_secrets(&path_str, &joined_content, Platform::CrossPlatform);
        secrets.append(&mut string_secrets.clone());
        findings.append(&mut string_secrets);

        // Regex for internal and API endpoints
        let url_re = Regex::new(r#"https?://[a-zA-Z0-9.\-_]+(?::\d+)?(?:/[^\s<>"'\\]*)?"#).ok();
        let firebase_re = Regex::new(r#"https://[a-zA-Z0-9_\-]+\.firebaseio\.com"#).ok();
        let s3_re = Regex::new(r#"https://[a-zA-Z0-9_\-]+\.s3(?:\.[a-zA-Z0-9_\-]+)?\.amazonaws\.com"#).ok();
        let dart_class_re = Regex::new(r#"package:[a-zA-Z0-9_]+/([a-zA-Z0-9_/]+)\.dart"#).ok();
        let method_channel_re = Regex::new(r#"(?i)(MethodChannel|EventChannel|BasicMessageChannel)\(["']([^"']+)["']\)"#).ok();

        for s in &extracted_strings {
            if let Some(ref re) = url_re {
                for mat in re.find_iter(s) {
                    let u = mat.as_str().to_string();
                    if !endpoints.contains(&u) && !u.contains("schema.org") && !u.contains("w3.org") {
                        endpoints.push(u);
                    }
                }
            }

            if let Some(ref re) = firebase_re {
                if let Some(mat) = re.find(s) {
                    let fb_url = mat.as_str().to_string();
                    findings.push(Finding {
                        id: format!("FLUTTER-FIREBASE-{}", &fb_url[8..16]),
                        rule_id: "FLUTTER-FIREBASE-RTDB".to_string(),
                        title: "Flutter Dart AOT Leaked Firebase Realtime Database URL".to_string(),
                        vuln_class: "MASVS-STORAGE-2".to_string(),
                        severity: Severity::Medium,
                        platform: Platform::CrossPlatform,
                        file_path: path_str.clone(),
                        line: 1,
                        snippet: fb_url.clone(),
                        description: format!(
                            "Discovered embedded Firebase Realtime Database URL inside Dart AOT snapshot: `{}`. If database rules are misconfigured, this allows unauthenticated public read/write of application records.",
                            fb_url
                        ),
                        impact: "Potential unauthenticated data exfiltration via the Firebase REST API (.json endpoint).".to_string(),
                        remediation: "Verify Firebase Security Rules ensuring `.read` and `.write` require authenticated user UID tokens.".to_string(),
                        cvss: CvssData {
                            score: 6.5,
                            severity: Severity::Medium,
                            vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N".to_string(),
                            exploitability: 3.9,
                            impact: 2.5,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec!["Database URL confirmed in Dart AOT snapshot".to_string()],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Verify Firebase endpoint `{}/.json` with a GET request.", fb_url),
                            "Check if response returns 401 Permission Denied (secure) or 200 OK with JSON data (vulnerable).".to_string(),
                        ],
                        metadata: {
                            let mut m = HashMap::new();
                            m.insert("firebase_url".to_string(), fb_url);
                            m
                        },
                    });
                }
            }

            if let Some(ref re) = s3_re {
                if let Some(mat) = re.find(s) {
                    let s3_url = mat.as_str().to_string();
                    findings.push(Finding {
                        id: format!("FLUTTER-S3-{}", &s3_url[8..16]),
                        rule_id: "FLUTTER-S3-BUCKET".to_string(),
                        title: "Flutter Dart AOT Leaked Amazon S3 Bucket URL".to_string(),
                        vuln_class: "MASVS-STORAGE-2".to_string(),
                        severity: Severity::Low,
                        platform: Platform::CrossPlatform,
                        file_path: path_str.clone(),
                        line: 1,
                        snippet: s3_url.clone(),
                        description: format!("Discovered hardcoded Amazon S3 bucket endpoint in Dart AOT snapshot: `{}`", s3_url),
                        impact: "If S3 bucket ACL allows public ListBucket or GetObject, sensitive cloud storage assets may be exposed.".to_string(),
                        remediation: "Enforce S3 Block Public Access and use AWS IAM signed URLs instead of public bucket references.".to_string(),
                        cvss: CvssData {
                            score: 4.3,
                            severity: Severity::Low,
                            vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N".to_string(),
                            exploitability: 2.8,
                            impact: 1.4,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec!["S3 bucket extracted from Dart binary".to_string()],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![format!("Test S3 bucket ACL: `aws s3 ls {} --no-sign-request`", s3_url)],
                        metadata: HashMap::new(),
                    });
                }
            }

            if let Some(ref re) = dart_class_re {
                if let Some(cap) = re.captures(s) {
                    let cls = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                    if !dart_classes.contains(&cls) {
                        dart_classes.push(cls);
                    }
                }
            }

            if let Some(ref re) = method_channel_re {
                if let Some(cap) = re.captures(s) {
                    let channel_type = cap.get(1).map(|m| m.as_str()).unwrap_or("MethodChannel");
                    let channel_name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                    findings.push(Finding {
                        id: format!("FLUTTER-CHANNEL-{}", channel_name.replace('/', "_")),
                        rule_id: "FLUTTER-METHOD-CHANNEL".to_string(),
                        title: format!("Flutter {} Attack Surface: `{}`", channel_type, channel_name),
                        vuln_class: "MASVS-PLATFORM-1".to_string(),
                        severity: Severity::Info,
                        platform: Platform::CrossPlatform,
                        file_path: path_str.clone(),
                        line: 1,
                        snippet: format!("{}(\"{}\")", channel_type, channel_name),
                        description: format!(
                            "Discovered Flutter native bridge channel `{}`. Flutter uses this bridge to invoke native Android (Java/Kotlin) or iOS (Obj-C/Swift) handlers.",
                            channel_name
                        ),
                        impact: "Exposes native device capabilities to the Dart runtime. Any native method invoked without permission checks can be abused if deep links route to channel calls.".to_string(),
                        remediation: "Audit the corresponding native `MethodChannelHandler` implementation to validate input parameters.".to_string(),
                        cvss: CvssData {
                            score: 0.0,
                            severity: Severity::Info,
                            vector: "CVSS:3.1/AV:L/AC:L/PR:N/UI:N/S:U/C:N/I:N/A:N".to_string(),
                            exploitability: 0.0,
                            impact: 0.0,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec!["Native bridge channel identified".to_string()],
                            chain_eligible: false,
                        },
                        reproduction_steps: vec![
                            format!("Trace `{}` in native decompiled code or Frida scripts.", channel_name),
                        ],
                        metadata: {
                            let mut m = HashMap::new();
                            m.insert("channel_name".to_string(), channel_name.to_string());
                            m
                        },
                    });
                }
            }
        }
    }

    FlutterAuditResult {
        findings,
        engine_version_or_hash: engine_hash,
        endpoints,
        dart_classes,
        secrets,
    }
}

/// Extract 40-character git commit hash from Flutter Engine binary
fn extract_flutter_engine_hash(bytes: &[u8]) -> Option<String> {
    // Engine hashes are 40 hex characters, often preceded or followed by Flutter engine markers
    let hash_re = Regex::new(r#"[0-9a-f]{40}"#).ok()?;
    let text = String::from_utf8_lossy(bytes);

    // Look for occurrences near flutter keywords
    for mat in hash_re.find_iter(&text) {
        let hash_candidate = mat.as_str();
        let start = mat.start().saturating_sub(100);
        let end = (mat.end() + 100).min(text.len());
        let context = &text[start..end];
        if context.contains("flutter") || context.contains("engine") || context.contains("dart") {
            return Some(hash_candidate.to_string());
        }
    }

    None
}

/// Check for BoringSSL and Flutter custom SSL verification routines
fn check_flutter_ssl_symbols(bytes: &[u8], file_path: &str) -> Option<Finding> {
    let text = String::from_utf8_lossy(bytes);

    let has_boringssl_pin = text.contains("session_verify_cert_chain")
        || text.contains("ssl_crypto_x509_session_verify_cert_chain")
        || text.contains("HandshakeVerifier");

    if has_boringssl_pin {
        Some(Finding {
            id: "FLUTTER-SSL-BORINGSSL".to_string(),
            rule_id: "FLUTTER-SSL-PINNING-TARGET".to_string(),
            title: "Flutter Engine BoringSSL Certificate Verification Symbol Located".to_string(),
            vuln_class: "MASVS-NETWORK-3".to_string(),
            severity: Severity::Low,
            platform: Platform::CrossPlatform,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "session_verify_cert_chain".to_string(),
            description: "Flutter engine embeds a custom BoringSSL stack that bypasses system CA trust stores. The `session_verify_cert_chain` function is the exact target used by security researchers and Frida bypass scripts to disable Flutter SSL verification.".to_string(),
            impact: "Researchers can patch this verification routine or hook it via Frida to intercept all Flutter HTTPS traffic.".to_string(),
            remediation: "Implement application-layer certificate hash pinning or public key pinning in Dart using crypto verification if TLS inspection resistance is required.".to_string(),
            cvss: CvssData {
                score: 3.1,
                severity: Severity::Low,
                vector: "CVSS:3.1/AV:L/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N".to_string(),
                exploitability: 1.5,
                impact: 1.5,
            },
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["Flutter BoringSSL symbol detected".to_string()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Attach Frida script hooking `session_verify_cert_chain` to return 1 (success).".to_string(),
                "Route device traffic through an intercepting proxy (Burp Suite / Caido / mitmproxy).".to_string(),
            ],
            metadata: HashMap::new(),
        })
    } else {
        None
    }
}

/// Helper to extract ASCII and UTF-8 strings from binary data
pub fn extract_ascii_and_utf8_strings(bytes: &[u8], min_len: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current = Vec::new();

    for &b in bytes {
        if b.is_ascii_graphic() || b == b' ' || b == b'\t' {
            current.push(b);
        } else {
            if current.len() >= min_len {
                if let Ok(s) = String::from_utf8(current.clone()) {
                    strings.push(s);
                }
            }
            current.clear();
        }
    }

    if current.len() >= min_len {
        if let Ok(s) = String::from_utf8(current) {
            strings.push(s);
        }
    }

    strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_strings() {
        let data = b"Hello\x00World\x00https://api.example.com/v1\x0012";
        let strings = extract_ascii_and_utf8_strings(data, 4);
        assert!(strings.contains(&"Hello".to_string()));
        assert!(strings.contains(&"World".to_string()));
        assert!(strings.contains(&"https://api.example.com/v1".to_string()));
        assert!(!strings.contains(&"12".to_string()));
    }
}
