use crate::domain::mobhunt::models::{AppMetadata, Finding, GateResult, GateStatus, Platform, Severity};
use crate::domain::mobhunt::reporting::calculate_cvss;
use regex::Regex;
use std::collections::HashMap;

/// Audit AndroidManifest.xml (text or decoded) for exported components, debuggable flags, and permissions
pub fn audit_manifest(manifest_content: &str, file_path: &str, app: &mut AppMetadata) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Extract package name
    if let Some(caps) = Regex::new(r#"package\s*=\s*"([^"]+)""#).ok().and_then(|r| r.captures(manifest_content)) {
        app.identifier = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
    }

    // 1. Insecure Debuggable Flag
    if manifest_content.contains(r#"android:debuggable="true""#) {
        let cvss = calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H").unwrap_or_default();
        findings.push(Finding {
            id: format!("AND-DEBUG-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "AND-01-DEBUG".to_string(),
            title: "Application Marked as Debuggable (android:debuggable=\"true\")".to_string(),
            vuln_class: "debuggable_binary".to_string(),
            severity: Severity::Critical,
            platform: Platform::Android,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "android:debuggable=\"true\"".to_string(),
            description: "The application is compiled in debug mode. On any Android device with USB debugging enabled, attackers can connect via ADB and run `run-as <package>` to access all private app sandbox files, SQLite databases, and execute code.".to_string(),
            impact: "Full sandbox compromise and arbitrary code execution under application UID via ADB run-as without requiring root.".to_string(),
            remediation: "Ensure `android:debuggable` is set to false in release builds (or omitted so gradle automatically sets it to false).".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["Critical Android finding: debuggable production build.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Connect Android device or emulator with ADB enabled.".into(),
                format!("Execute `adb shell run-as {}`.", if app.identifier.is_empty() { "<package>" } else { &app.identifier }),
                "Inspect private app data directory `/data/data/<package>/` with read/write access.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    // 2. Exported Components without permission checks
    let component_tags = ["activity", "receiver", "service", "provider"];
    for tag in &component_tags {
        let re_pattern = format!(r#"<{} [^>]*android:name="([^"]+)"[^>]*>"#, tag);
        if let Ok(re) = Regex::new(&re_pattern) {
            for cap in re.captures_iter(manifest_content) {
                let full_tag = cap.get(0).map(|m| m.as_str()).unwrap_or("");
                let comp_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");

                let is_exported = full_tag.contains(r#"android:exported="true""#);
                let has_permission = full_tag.contains("android:permission=");

                if is_exported && !has_permission {
                    let cvss = calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N").unwrap_or_default();
                    findings.push(Finding {
                        id: format!("AND-EXP-{}-{}", tag.to_ascii_uppercase(), uuid::Uuid::new_v4().simple()),
                        rule_id: format!("AND-EXP-{}", tag.to_ascii_uppercase()),
                        title: format!("Exported Android {} Exposed without Permission: {}", tag, comp_name),
                        vuln_class: "exported_component".to_string(),
                        severity: Severity::Medium,
                        platform: Platform::Android,
                        file_path: file_path.to_string(),
                        line: 1,
                        snippet: full_tag.to_string(),
                        description: format!("The {} '{}' is explicitly exported (`android:exported=\"true\"`) without requiring calling permissions.", tag, comp_name),
                        impact: "Any third-party malicious application installed on the device can launch or interact with this component directly to trigger unauthorized state changes or data leakage.".to_string(),
                        remediation: "Set `android:exported=\"false\"` if the component is internal, or protect it with a custom `android:permission` with `signature` protection level.".to_string(),
                        cvss,
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec![format!("Exported {} surface check.", tag)],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("From terminal, invoke component via ADB: `adb shell am start -n {}/{}`.", app.identifier, comp_name),
                            "Verify that component launches without permission errors or authentication gates.".into(),
                        ],
                        metadata: HashMap::new(),
                    });
                }
            }
        }
    }

    // 3. Deep link intent filters
    if manifest_content.contains("android.intent.action.VIEW") && manifest_content.contains("android.intent.category.BROWSABLE") {
        if let Ok(re) = Regex::new(r#"android:scheme="([^"]+)""#) {
            for cap in re.captures_iter(manifest_content) {
                if let Some(scheme) = cap.get(1) {
                    let s = scheme.as_str().to_string();
                    if s != "http" && s != "https" && !app.custom_schemes.contains(&s) {
                        app.custom_schemes.push(s);
                    }
                }
            }
        }
    }

    findings
}

/// Audit network_security_config.xml for insecure cleartext traffic and user trust
pub fn audit_network_security_config(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Insecure cleartext permitted
    if content.contains(r#"cleartextTrafficPermitted="true""#) {
        let cvss = calculate_cvss("AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N").unwrap_or_default();
        findings.push(Finding {
            id: format!("AND-NSC-CLEARTEXT-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "AND-NSC-01".to_string(),
            title: "Network Security Config Permits Cleartext Traffic".to_string(),
            vuln_class: "cleartext_traffic".to_string(),
            severity: Severity::High,
            platform: Platform::Android,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "cleartextTrafficPermitted=\"true\"".to_string(),
            description: "The application's network security configuration explicitly permits unencrypted cleartext HTTP connections.".to_string(),
            impact: "Permits eavesdropping and interception of sensitive network payloads on hostile or untrusted Wi-Fi networks.".to_string(),
            remediation: "Set `cleartextTrafficPermitted=\"false\"` across base-config and restrict domain exceptions strictly.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["High severity cleartext configuration allowed in network security config.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Inspect `res/xml/network_security_config.xml`.".into(),
                "Observe `<base-config cleartextTrafficPermitted=\"true\">` allowing plain HTTP.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    // User Certificate Trust (facilitates MitM)
    if content.contains(r#"<certificates src="user""#) {
        let cvss = calculate_cvss("AV:L/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N").unwrap_or_default();
        findings.push(Finding {
            id: format!("AND-NSC-USERCA-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "AND-NSC-02".to_string(),
            title: "Application Trusts User-Installed CA Certificates in Production".to_string(),
            vuln_class: "trust_user_ca".to_string(),
            severity: Severity::Medium,
            platform: Platform::Android,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "<certificates src=\"user\" />".to_string(),
            description: "Application configuration trusts user-installed Certificate Authorities (CAs). On Android 7.0+, apps default to system CAs only; enabling user CAs permits passive proxy interception (e.g. Burp Suite / Charles).".to_string(),
            impact: "Adversaries with local access or malicious certificate profiles installed on the device can decrypt and inspect all application TLS traffic.".to_string(),
            remediation: "Remove `<certificates src=\"user\" />` from release network_security_config or restrict it to `<debug-overrides>`.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["User CA trust enabled in production configuration.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Inspect `network_security_config.xml` trust anchors.".into(),
                "Observe user-installed certificates declared in non-debug trust-anchors.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    findings
}

/// Audit DEX strings & bytecode for insecure API invocations
pub fn audit_dex_strings(strings: &[String], file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let mut has_js_interface = false;
    let mut has_file_access = false;
    let mut has_ecb_mode = false;
    let mut has_des_cipher = false;

    for s in strings {
        if s.contains("addJavascriptInterface") {
            has_js_interface = true;
        }
        if s.contains("setAllowFileAccessFromFileURLs") || s.contains("setAllowUniversalAccessFromFileURLs") {
            has_file_access = true;
        }
        if s.contains("AES/ECB") || s.contains("DES/ECB") {
            has_ecb_mode = true;
        }
        if s.contains("DESede") || s.contains("Cipher.getInstance(\"DES\"") {
            has_des_cipher = true;
        }
    }

    if has_js_interface && has_file_access {
        let cvss = calculate_cvss("AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H").unwrap_or_default();
        findings.push(Finding {
            id: format!("AND-WEBVIEW-RCE-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "AND-WEBVIEW-01".to_string(),
            title: "Insecure WebView JavaScript Bridge & Universal File Access".to_string(),
            vuln_class: "webview_js_bridge_rce".to_string(),
            severity: Severity::High,
            platform: Platform::Android,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "addJavascriptInterface + setAllowUniversalAccessFromFileURLs".to_string(),
            description: "WebView instance configures both native JavaScript interfaces and universal file access from file URLs.".to_string(),
            impact: "If an untrusted remote website or local HTML file is rendered, attackers can traverse sandbox files and invoke Java bridge methods.".to_string(),
            remediation: "Disable setAllowFileAccessFromFileURLs and setAllowUniversalAccessFromFileURLs. Restrict addJavascriptInterface to trusted internal origins.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["High severity WebView misconfiguration identified in bytecode.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Decompile APK using jadx or inspect DEX strings.".into(),
                "Identify WebView configuration enabling universal file access and JS interfaces.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    if has_ecb_mode || has_des_cipher {
        let cvss = calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N").unwrap_or_default();
        findings.push(Finding {
            id: format!("AND-CRYPTO-WEAK-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "AND-CRYPTO-01".to_string(),
            title: "Weak Cryptographic Primitive In Use (ECB Mode / DES)".to_string(),
            vuln_class: "crypto_misuse".to_string(),
            severity: Severity::Medium,
            platform: Platform::Android,
            file_path: file_path.to_string(),
            line: 1,
            snippet: if has_des_cipher { "Cipher: DES/DESede" } else { "Cipher Mode: ECB" }.to_string(),
            description: "The application invokes legacy weak ciphers (DES) or insecure cipher block modes (ECB). ECB mode does not employ initialization vectors (IV) and leaks data patterns.".to_string(),
            impact: "Ciphertext pattern leakage allows local or network adversaries to reconstruct sensitive encrypted session tokens or credentials.".to_string(),
            remediation: "Use standard AES-GCM (Galois/Counter Mode) with unique initialization vectors (IVs) or ChaCha20-Poly1305.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["Weak cryptographic algorithm detected in DEX string references.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Inspect decompiled cryptographic helper classes in APK.".into(),
                "Locate `Cipher.getInstance(\"AES/ECB/PKCS5Padding\")` or DES cipher instances.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    findings
}
