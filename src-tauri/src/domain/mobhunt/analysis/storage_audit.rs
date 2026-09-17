use crate::domain::mobhunt::models::{Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::sync::OnceLock;

static SHARED_PREF_PUT_REGEX: OnceLock<Regex> = OnceLock::new();
static KEYCHAIN_ALWAYS_REGEX: OnceLock<Regex> = OnceLock::new();
static NSUSERDEFAULTS_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_shared_pref_put_regex() -> &'static Regex {
    SHARED_PREF_PUT_REGEX.get_or_init(|| {
        Regex::new(r#"(?i)\.putString\s*\(\s*"(.*?)"\s*,\s*(.*?)\)"#).unwrap()
    })
}

fn get_keychain_always_regex() -> &'static Regex {
    KEYCHAIN_ALWAYS_REGEX.get_or_init(|| {
        Regex::new(r#"(?i)kSecAttrAccessibleAlways(?:ThisDeviceOnly)?"#).unwrap()
    })
}

fn get_nsuserdefaults_regex() -> &'static Regex {
    NSUSERDEFAULTS_REGEX.get_or_init(|| {
        Regex::new(r#"(?i)UserDefaults\.standard\.set\s*\([^,]+,\s*forKey:\s*"(.*?)""#).unwrap()
    })
}

/// Audit Android code for insecure storage (world-readable/writeable prefs, plaintext credentials in SharedPreferences)
pub fn audit_android_storage(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 1. Check for deprecated / dangerous MODE_WORLD_READABLE or MODE_WORLD_WRITEABLE
    if content.contains("MODE_WORLD_READABLE") || content.contains("Context.MODE_WORLD_READABLE") {
        let mut f = Finding::new(
            "SEC-AND-STORE-001",
            "Insecure SharedPreferences: MODE_WORLD_READABLE",
            format!(
                "The application creates a world-readable file or SharedPreferences using MODE_WORLD_READABLE in {file_path}. \
                Any other app installed on the device can read this file directly without permissions."
            ),
            Severity::High,
            Platform::Android,
        );
        f.file_path = file_path.to_string();
        f.remediation = "Use MODE_PRIVATE or EncryptedSharedPreferences to store application data.".to_string();
        f.gate = GateResult {
            status: GateStatus::Passed,
            notes: vec!["Direct unauthorized file read vulnerability across all apps on device.".to_string()],
            chain_eligible: true,
        };
        findings.push(f);
    }

    if content.contains("MODE_WORLD_WRITEABLE") || content.contains("Context.MODE_WORLD_WRITEABLE") {
        let mut f = Finding::new(
            "SEC-AND-STORE-002",
            "Insecure SharedPreferences: MODE_WORLD_WRITEABLE",
            format!(
                "The application creates a world-writeable file or SharedPreferences using MODE_WORLD_WRITEABLE in {file_path}. \
                Any third-party app can overwrite, corrupt, or tamper with this configuration file."
            ),
            Severity::Critical,
            Platform::Android,
        );
        f.file_path = file_path.to_string();
        f.remediation = "Never use MODE_WORLD_WRITEABLE. Use MODE_PRIVATE or EncryptedSharedPreferences.".to_string();
        f.gate = GateResult {
            status: GateStatus::Passed,
            notes: vec!["Direct unauthorized data tampering vulnerability across apps on device.".to_string()],
            chain_eligible: true,
        };
        findings.push(f);
    }

    // 2. Check for sensitive keys stored in standard SharedPreferences instead of EncryptedSharedPreferences
    let pref_re = get_shared_pref_put_regex();
    let sensitive_keys = [
        "token", "auth", "secret", "password", "passwd", "jwt", "apikey", "api_key",
        "private_key", "pin", "credential", "session_id", "access_token", "refresh_token",
    ];

    for (line_no, line) in content.lines().enumerate() {
        if let Some(cap) = pref_re.captures(line) {
            let key_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let key_lower = key_name.to_ascii_lowercase();

            for s in &sensitive_keys {
                if key_lower.contains(s) {
                    let mut f = Finding::new(
                        "SEC-AND-STORE-PLAINTEXT",
                        format!("Plaintext Sensitive Credential in SharedPreferences: '{key_name}'"),
                        format!(
                            "The sensitive credential '{key_name}' is stored in plaintext SharedPreferences at line {line_no} in {file_path}. \
                            Standard SharedPreferences files are stored unencrypted in /data/data/<package>/shared_prefs/ and are \
                            accessible via ADB backup, device rooting, or file provider vulnerabilities."
                        ),
                        Severity::Medium,
                        Platform::Android,
                    );
                    f.file_path = file_path.to_string();
                    f.line = line_no + 1;
                    f.snippet = line.trim().to_string();
                    f.remediation = "Store sensitive tokens, passwords, and cryptographic keys using androidx.security.crypto.EncryptedSharedPreferences \
                        backed by the Android Keystore MasterKey."
                            .to_string();
                    findings.push(f);
                    break;
                }
            }
        }
    }

    findings
}

/// Audit iOS Swift/Obj-C code for insecure Keychain access flags and sensitive NSUserDefaults usage
pub fn audit_ios_storage(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 1. Insecure Keychain accessibility: kSecAttrAccessibleAlways / kSecAttrAccessibleAlwaysThisDeviceOnly
    let always_re = get_keychain_always_regex();
    for (line_no, line) in content.lines().enumerate() {
        if always_re.is_match(line) {
            let mut f = Finding::new(
                "SEC-IOS-STORE-ALWAYS",
                "Insecure Keychain Accessibility: kSecAttrAccessibleAlways",
                format!(
                    "The application configures Keychain items with '{line}' at line {line_no} in {file_path}. \
                    kSecAttrAccessibleAlways was deprecated by Apple because items with this protection class are \
                    stored without hardware encryption protection when the device is locked, making them accessible to \
                    lockscreen bypass exploits and background processes."
                ),
                Severity::Medium,
                Platform::Ios,
            );
            f.file_path = file_path.to_string();
            f.line = line_no + 1;
            f.snippet = line.trim().to_string();
            f.remediation = "Use kSecAttrAccessibleWhenUnlockedThisDeviceOnly or kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly, \
                or enforce biometrics using SecAccessControlCreateWithFlags with .biometryAny."
                    .to_string();
            findings.push(f);
        }
    }

    // 2. Sensitive data stored in NSUserDefaults / UserDefaults
    let ud_re = get_nsuserdefaults_regex();
    let sensitive_keys = [
        "token", "auth", "secret", "password", "jwt", "apikey", "api_key",
        "credential", "access_token", "refresh_token", "private_key",
    ];

    for (line_no, line) in content.lines().enumerate() {
        if let Some(cap) = ud_re.captures(line) {
            let key_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let key_lower = key_name.to_ascii_lowercase();

            for s in &sensitive_keys {
                if key_lower.contains(s) {
                    let mut f = Finding::new(
                        "SEC-IOS-STORE-NSUSERDEFAULTS",
                        format!("Sensitive Credential Stored in UserDefaults: '{key_name}'"),
                        format!(
                            "The sensitive credential '{key_name}' is stored in unencrypted UserDefaults at line {line_no} in {file_path}. \
                            UserDefaults values are stored as plaintext XML plists in the app's Library/Preferences folder and \
                            are automatically included in unencrypted iTunes/iCloud backups."
                        ),
                        Severity::Medium,
                        Platform::Ios,
                    );
                    f.file_path = file_path.to_string();
                    f.line = line_no + 1;
                    f.snippet = line.trim().to_string();
                    f.remediation = "Store authentication tokens, passwords, and sensitive keys in the iOS Keychain Services API with \
                        kSecAttrAccessibleWhenUnlockedThisDeviceOnly rather than UserDefaults."
                            .to_string();
                    findings.push(f);
                    break;
                }
            }
        }
    }

    findings
}
