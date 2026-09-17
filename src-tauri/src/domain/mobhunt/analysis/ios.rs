use crate::domain::mobhunt::models::{AppMetadata, Finding, GateResult, GateStatus, Platform, Severity};
use crate::domain::mobhunt::reporting::calculate_cvss;
use regex::Regex;
use std::collections::HashMap;

/// Audit Info.plist content for security misconfigurations
pub fn audit_info_plist(plist_content: &str, file_path: &str, app: &mut AppMetadata) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Extract App Name & Identifier if available
    if let Some(caps) = Regex::new(r"<key>CFBundleIdentifier</key>\s*<string>([^<]+)</string>").ok().and_then(|r| r.captures(plist_content)) {
        app.identifier = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
    }
    if let Some(caps) = Regex::new(r"<key>CFBundleName</key>\s*<string>([^<]+)</string>").ok().and_then(|r| r.captures(plist_content)) {
        app.name = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
    }
    if let Some(caps) = Regex::new(r"<key>CFBundleShortVersionString</key>\s*<string>([^<]+)</string>").ok().and_then(|r| r.captures(plist_content)) {
        app.version = caps.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
    }

    // 1. ATS: NSAllowsArbitraryLoads
    if plist_content.contains("NSAllowsArbitraryLoads") && plist_content.contains("<true/>") {
        let cvss = calculate_cvss("AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N").unwrap_or_default();
        findings.push(Finding {
            id: format!("IOS-ATS-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "IOS-03".to_string(),
            title: "App Transport Security (ATS) Globally Disabled".to_string(),
            vuln_class: "cleartext_traffic".to_string(),
            severity: Severity::High,
            platform: Platform::Ios,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "<key>NSAllowsArbitraryLoads</key><true/>".to_string(),
            description: "App Transport Security (ATS) is globally disabled, allowing cleartext HTTP communication to arbitrary remote servers.".to_string(),
            impact: "Network eavesdroppers and Man-in-the-Middle attackers can intercept sensitive authentication tokens, session cookies, and personal data.".to_string(),
            remediation: "Remove NSAllowsArbitraryLoads or restrict cleartext exceptions to specific, strictly required domain keys via NSExceptionDomains.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["High severity ATS configuration allowing cleartext traffic across all domains.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Extract and inspect `Info.plist` from application bundle.".into(),
                "Locate `<key>NSAppTransportSecurity</key>` dictionary.".into(),
                "Observe `<key>NSAllowsArbitraryLoads</key><true/>` enabling unencrypted connections.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    // 2. Custom URL Schemes (Deep link attack surface)
    if let Ok(re) = Regex::new(r"<key>CFBundleURLSchemes</key>\s*<array>(.*?)</array>") {
        if let Some(caps) = re.captures(plist_content) {
            let inner = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let scheme_re = Regex::new(r"<string>([^<]+)</string>").unwrap();
            let mut schemes = Vec::new();
            for s_cap in scheme_re.captures_iter(inner) {
                if let Some(scheme) = s_cap.get(1) {
                    schemes.push(scheme.as_str().to_string());
                }
            }
            if !schemes.is_empty() {
                app.custom_schemes = schemes.clone();
                let cvss = calculate_cvss("AV:N/AC:L/PR:N/UI:R/S:U/C:L/I:L/A:N").unwrap_or_default();
                findings.push(Finding {
                    id: format!("IOS-SCHEME-{}", uuid::Uuid::new_v4().simple()),
                    rule_id: "IOS-01".to_string(),
                    title: format!("Custom URL Schemes Exposed: {}", schemes.join(", ")),
                    vuln_class: "url_scheme_hijack".to_string(),
                    severity: Severity::Medium,
                    platform: Platform::Ios,
                    file_path: file_path.to_string(),
                    line: 1,
                    snippet: format!("CFBundleURLSchemes: [{}]", schemes.join(", ")),
                    description: format!("The application registers {} custom URL schemes ({}) that can be invoked by other applications or WebViews.", schemes.len(), schemes.join(", ")),
                    impact: "Malicious applications or phishing websites can trigger openURL handlers to perform unauthorized actions, hijack authentication callbacks, or fuzz internal parsers.".to_string(),
                    remediation: "Validate caller origin and authenticate all parameters passed via custom URL schemes. Prefer Universal Links (Associated Domains) over custom schemes for authentication callbacks.".to_string(),
                    cvss,
                    gate: GateResult {
                        status: GateStatus::Passed,
                        notes: vec!["Exposed custom URL scheme attack surface for deep link exploitation.".into()],
                        chain_eligible: true,
                    },
                    reproduction_steps: vec![
                        format!("Identify registered schemes in `Info.plist`: {}", schemes.join(", ")),
                        format!("Craft an HTML page or call `openURL` targeting `{}:<payload>`.", schemes.first().cloned().unwrap_or_default()),
                        "Inspect whether input parameters trigger actions without explicit user verification.".into(),
                    ],
                    metadata: HashMap::new(),
                });
            }
        }
    }

    // 3. Excessive LSApplicationQueriesSchemes
    if let Ok(re) = Regex::new(r"<key>LSApplicationQueriesSchemes</key>\s*<array>(.*?)</array>") {
        if let Some(caps) = re.captures(plist_content) {
            let inner = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let count = inner.matches("<string>").count();
            if count > 50 {
                findings.push(Finding {
                    id: format!("IOS-QUERY-{}", uuid::Uuid::new_v4().simple()),
                    rule_id: "IOS-PRIV-01".to_string(),
                    title: format!("Excessive Query Schemes Declared ({} schemes)", count),
                    vuln_class: "privacy_probing".to_string(),
                    severity: Severity::Low,
                    platform: Platform::Ios,
                    file_path: file_path.to_string(),
                    line: 1,
                    snippet: format!("LSApplicationQueriesSchemes count: {}", count),
                    description: "Application declares an unusually large list of schemes in LSApplicationQueriesSchemes, which is commonly used to fingerprint installed apps on device.".to_string(),
                    impact: "Can be used to profile installed apps on the user's device for tracking or behavioral fingerprinting.".to_string(),
                    remediation: "Remove unnecessary schemes and restrict LSApplicationQueriesSchemes to essential integration partners only.".to_string(),
                    cvss: calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N").unwrap_or_default(),
                    gate: GateResult {
                        status: GateStatus::Passed,
                        notes: vec!["Privacy query scheme inventory check.".into()],
                        chain_eligible: true,
                    },
                    reproduction_steps: vec!["Review declared LSApplicationQueriesSchemes entries in Info.plist.".into()],
                    metadata: HashMap::new(),
                });
            }
        }
    }

    findings
}

/// Audit Entitlements for security flaws
pub fn audit_entitlements(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Check for get-task-allow (Debuggable)
    if content.contains("<key>get-task-allow</key>") && content.contains("<true/>") {
        let cvss = calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H").unwrap_or_default();
        findings.push(Finding {
            id: format!("IOS-ENT-DEBUG-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "IOS-06-DEBUG".to_string(),
            title: "Production Binary Marked as Debuggable (get-task-allow: true)".to_string(),
            vuln_class: "debuggable_binary".to_string(),
            severity: Severity::Critical,
            platform: Platform::Ios,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "<key>get-task-allow</key><true/>".to_string(),
            description: "The application entitlement `get-task-allow` is set to true. Any unprivileged process on jailbroken or developer-configured devices can attach debuggers (lldb) and read memory directly.".to_string(),
            impact: "Permits runtime inspection, memory dumping of encryption keys and tokens, and method hooking without runtime protection barriers.".to_string(),
            remediation: "Build the release package with a production distribution provisioning profile so `get-task-allow` is stripped or set to false.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["Critical entitlement finding: debuggable production binary.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Extract embedded entitlements using `codesign -d --entitlements :- <binary>`.".into(),
                "Observe `<key>get-task-allow</key><true/>`.".into(),
                "Attach debugger via `lldb -p <pid>` without requiring codesign entitlement bypasses.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    // Check for Shared App Groups
    if content.contains("com.apple.security.application-groups") {
        let cvss = calculate_cvss("AV:L/AC:L/PR:N/UI:N/S:U/C:L/I:L/A:N").unwrap_or_default();
        findings.push(Finding {
            id: format!("IOS-ENT-GRP-{}", uuid::Uuid::new_v4().simple()),
            rule_id: "IOS-06-APPGROUP".to_string(),
            title: "Shared Application Group Container Declared".to_string(),
            vuln_class: "app_group_container".to_string(),
            severity: Severity::Low,
            platform: Platform::Ios,
            file_path: file_path.to_string(),
            line: 1,
            snippet: "<key>com.apple.security.application-groups</key>".to_string(),
            description: "Application declares shared app group containers. Shared containers allow app extensions and related group members to read/write common sandbox directories.".to_string(),
            impact: "If file permissions inside the group container are not strictly managed, malicious extensions or local attackers can access shared cached files.".to_string(),
            remediation: "Ensure all sensitive files written to app group directories employ NSFileProtectionComplete and sqlite encryption.".to_string(),
            cvss,
            gate: GateResult {
                status: GateStatus::Passed,
                notes: vec!["Shared container asset review.".into()],
                chain_eligible: true,
            },
            reproduction_steps: vec![
                "Check entitlements for group identifier strings.".into(),
                "Inspect shared container directory at `/private/var/mobile/Containers/Shared/AppGroup/<UUID>`.".into(),
            ],
            metadata: HashMap::new(),
        });
    }

    findings
}

/// Inspect Mach-O binary header for encryption state (cryptid)
pub fn check_macho_encryption(bytes: &[u8]) -> Option<bool> {
    // Look for LC_ENCRYPTION_INFO (0x21) or LC_ENCRYPTION_INFO_64 (0x2C)
    // In little-endian: 0x2C is [0x2C, 0x00, 0x00, 0x00], 0x21 is [0x21, 0x00, 0x00, 0x00]
    if bytes.len() < 1024 {
        return None;
    }
    for i in 0..(bytes.len().min(4096) - 20) {
        if (bytes[i] == 0x2C || bytes[i] == 0x21) && bytes[i + 1] == 0 && bytes[i + 2] == 0 && bytes[i + 3] == 0 {
            // cryptid is at offset 16 from load command header in encryption_info
            let cryptid_offset = i + 16;
            if cryptid_offset + 4 <= bytes.len() {
                let cryptid = u32::from_le_bytes([
                    bytes[cryptid_offset],
                    bytes[cryptid_offset + 1],
                    bytes[cryptid_offset + 2],
                    bytes[cryptid_offset + 3],
                ]);
                return Some(cryptid != 0);
            }
        }
    }
    None
}
