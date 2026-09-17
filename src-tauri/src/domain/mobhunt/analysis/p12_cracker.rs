use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Attempts to crack password-protected PKCS#12 (.p12 / .pfx) client certificates bundled with the application
pub fn audit_p12_file(file_path: &Path, bundle_id: Option<&str>, app_name: Option<&str>) -> Option<Finding> {
    let path_str = file_path.display().to_string();

    // Generate smart contextual password candidates
    let candidates = generate_p12_candidates(bundle_id, app_name);

    let mut cracked_password = None;

    for candidate in &candidates {
        if check_p12_password(file_path, candidate) {
            cracked_password = Some(candidate.clone());
            break;
        }
    }

    if let Some(password) = cracked_password {
        let display_pwd = if password.is_empty() {
            "(empty password)"
        } else {
            &password
        };

        Some(Finding {
            id: format!("P12-CRACKED-{}", file_path.file_name().and_then(|n| n.to_str()).unwrap_or("cert")),
            rule_id: "P12-PASSWORD-CRACKED".to_string(),
            title: "Bundled PKCS#12 Private Key Password Successfully Cracked".to_string(),
            vuln_class: "MASVS-STORAGE-1".to_string(),
            severity: Severity::Critical,
            platform: Platform::CrossPlatform,
            file_path: path_str.clone(),
            line: 1,
            snippet: format!("Password: {}", display_pwd),
            description: format!(
                "Application bundles a PKCS#12 (.p12/.pfx) certificate keystore with a weak, predictable password: `{}`. An attacker can extract client mTLS certificates, private RSA/ECDSA keys, and authenticate directly to backend APIs as the application.",
                display_pwd
            ),
            impact: "Complete compromise of client-side mTLS mutual authentication and cryptographic identity.".to_string(),
            remediation: "Never embed static PKCS#12 client private keys in client application bundles. Use device Keychain / Android Keystore with dynamic device enrollment.".to_string(),
            cvss: CvssData {
                score: 8.8,
                severity: Severity::Critical,
                vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N".to_string(),
                exploitability: 3.9,
                impact: 4.9,
            },
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec![format!("P12 keystore cracked with password: {}", display_pwd)],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                format!("Run: openssl pkcs12 -in \"{}\" -nodes -password \"pass:{}\" -out extracted_key.pem", path_str, password),
                "Verify that both the client certificate and private key are successfully dumped.".to_string(),
            ],
            metadata: {
                let mut m = HashMap::new();
                m.insert("password".to_string(), password);
                m
            },
        })
    } else {
        // P12 found but password not in smart dictionary
        Some(Finding {
            id: format!("P12-BUNDLED-{}", file_path.file_name().and_then(|n| n.to_str()).unwrap_or("cert")),
            rule_id: "P12-CERTIFICATE-BUNDLED".to_string(),
            title: "Bundled PKCS#12 Client Keystore / Private Key Located".to_string(),
            vuln_class: "MASVS-STORAGE-1".to_string(),
            severity: Severity::High,
            platform: Platform::CrossPlatform,
            file_path: path_str.clone(),
            line: 1,
            snippet: file_path.file_name().and_then(|n| n.to_str()).unwrap_or("cert.p12").to_string(),
            description: "Application bundles a PKCS#12 (.p12 / .pfx) certificate archive. If this keystore contains private keys, offline dictionary cracking attacks can recover backend credentials.".to_string(),
            impact: "Potential compromise of backend client authentication keys.".to_string(),
            remediation: "Do not ship client certificates with embedded private keys in production mobile binaries.".to_string(),
            cvss: CvssData {
                score: 7.5,
                severity: Severity::High,
                vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N".to_string(),
                exploitability: 3.9,
                impact: 3.6,
            },
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["PKCS#12 bundle detected in application resources".to_string()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                format!("Extract \"{}\" and run an offline wordlist attack (hashcat mode 29700 / john).", path_str),
            ],
            metadata: HashMap::new(),
        })
    }
}

/// Generates smart contextual candidate passwords for P12 cracking
pub fn generate_p12_candidates(bundle_id: Option<&str>, app_name: Option<&str>) -> Vec<String> {
    let mut words = vec![
        "".to_string(), // Empty password is very common in mobile bundles
        "1234".to_string(),
        "123456".to_string(),
        "password".to_string(),
        "secret".to_string(),
        "admin".to_string(),
        "client".to_string(),
        "cert".to_string(),
        "certificate".to_string(),
        "test".to_string(),
        "root".to_string(),
        "0000".to_string(),
        "1111".to_string(),
        "mobile".to_string(),
        "ios".to_string(),
        "android".to_string(),
    ];

    if let Some(bid) = bundle_id {
        let bid_trimmed = bid.trim();
        if !bid_trimmed.is_empty() {
            words.push(bid_trimmed.to_string());
            words.push(bid_trimmed.to_ascii_lowercase());

            // Add component parts (e.g. "com.example.superapp" -> "superapp", "example")
            for part in bid_trimmed.split('.') {
                if part.len() > 2 && part != "com" && part != "org" && part != "io" && part != "net" {
                    words.push(part.to_string());
                    words.push(part.to_ascii_lowercase());
                    words.push(format!("{}123", part));
                }
            }
        }
    }

    if let Some(name) = app_name {
        let name_trimmed = name.trim();
        if !name_trimmed.is_empty() {
            words.push(name_trimmed.to_string());
            words.push(name_trimmed.to_ascii_lowercase());
            words.push(format!("{}123", name_trimmed));
            words.push(format!("{}_secret", name_trimmed.to_ascii_lowercase()));
        }
    }

    words.dedup();
    words
}

/// Tests if OpenSSL can verify the PKCS#12 file with the candidate password
fn check_p12_password(p12_path: &Path, password: &str) -> bool {
    // Attempt with legacy flags (required for RC2-40-CBC / 3DES PKCS#12 files)
    let output = Command::new("openssl")
        .args([
            "pkcs12",
            "-in",
            &p12_path.display().to_string(),
            "-password",
            &format!("pass:{}", password),
            "-nokeys",
            "-legacy",
            "-provider",
            "default",
        ])
        .output();

    if let Ok(res) = output {
        if res.status.success() {
            let stderr = String::from_utf8_lossy(&res.stderr);
            let stdout = String::from_utf8_lossy(&res.stdout);
            if !stderr.contains("Mac verify error")
                && !stderr.contains("bad decrypt")
                && !stdout.contains("Mac verify error")
            {
                return true;
            }
        }
    }

    // Fallback without -legacy flag (for modern OpenSSL or AES-encrypted P12s)
    let output_modern = Command::new("openssl")
        .args([
            "pkcs12",
            "-in",
            &p12_path.display().to_string(),
            "-password",
            &format!("pass:{}", password),
            "-nokeys",
        ])
        .output();

    if let Ok(res) = output_modern {
        if res.status.success() {
            let stderr = String::from_utf8_lossy(&res.stderr);
            if !stderr.contains("Mac verify error") && !stderr.contains("bad decrypt") {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_generation() {
        let candidates = generate_p12_candidates(Some("com.target.fintech"), Some("FintechApp"));
        assert!(candidates.contains(&"".to_string()));
        assert!(candidates.contains(&"1234".to_string()));
        assert!(candidates.contains(&"fintech".to_string()));
        assert!(candidates.contains(&"FintechApp".to_string()));
    }
}
