use crate::domain::mobhunt::models::{Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::sync::OnceLock;

static PROVIDER_TAG_REGEX: OnceLock<Regex> = OnceLock::new();
static SQL_CONCAT_REGEX: OnceLock<Regex> = OnceLock::new();
static PATH_TRAVERSAL_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_provider_tag_regex() -> &'static Regex {
    PROVIDER_TAG_REGEX.get_or_init(|| {
        Regex::new(r#"(?s)<provider\b([^>]*?)(?:/>|>(.*?)</provider>)"#).unwrap()
    })
}

fn get_sql_concat_regex() -> &'static Regex {
    SQL_CONCAT_REGEX.get_or_init(|| {
        Regex::new(r#"(?i)(?:rawQuery|execSQL|query|update|delete)\s*\([^)]*?\+[^)]*?\)"#).unwrap()
    })
}

fn get_path_traversal_regex() -> &'static Regex {
    PATH_TRAVERSAL_REGEX.get_or_init(|| {
        Regex::new(r#"(?i)(?:openFile|openAssetFile|openFileHelper)\s*\([^)]*?\)"#).unwrap()
    })
}

/// Audit AndroidManifest.xml for insecure or unpermissioned ContentProvider declarations
pub fn audit_manifest_providers(manifest_content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let provider_re = get_provider_tag_regex();

    for cap in provider_re.captures_iter(manifest_content) {
        let attrs = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let is_exported = attrs.contains(r#"android:exported="true""#);
        let has_perm = attrs.contains(r#"android:permission="#)
            || (attrs.contains(r#"android:readPermission="#) && attrs.contains(r#"android:writePermission="#));
        let grant_uri_perms = attrs.contains(r#"android:grantUriPermissions="true""#);

        // Extract name & authorities
        let name = Regex::new(r#"android:name="([^"]+)""#)
            .ok()
            .and_then(|re| re.captures(attrs).map(|c| c.get(1).unwrap().as_str().to_string()))
            .unwrap_or_else(|| "UnknownProvider".to_string());

        let authorities = Regex::new(r#"android:authorities="([^"]+)""#)
            .ok()
            .and_then(|re| re.captures(attrs).map(|c| c.get(1).unwrap().as_str().to_string()))
            .unwrap_or_else(|| "".to_string());

        if is_exported && !has_perm {
            let mut f = Finding::new(
                "SEC-AND-CP-001",
                format!("Exported Unprotected ContentProvider: {name}"),
                format!(
                    "The ContentProvider '{name}' (authority: '{authorities}') is exported (android:exported=\"true\") \
                    without requiring any read or write permissions (android:permission). Any third-party app installed \
                    on the device can query, insert, update, or delete sensitive app data stored in this provider."
                ),
                Severity::High,
                Platform::Android,
            );
            f.file_path = file_path.to_string();
            f.snippet = attrs.lines().take(5).collect::<Vec<_>>().join(" ");
            f.remediation = "Set android:exported=\"false\" if the ContentProvider is only used internally within the application. \
                If cross-app data sharing is strictly required, protect the provider with a signature-level permission (android:protectionLevel=\"signature\")."
                    .to_string();
            f.gate = GateResult {
                status: GateStatus::Passed,
                notes: vec![
                    "Directly accessible by third-party malware or hostile apps on device without permissions.".to_string(),
                ],
                chain_eligible: true,
            };
            findings.push(f);
        }

        if grant_uri_perms {
            let mut f = Finding::new(
                "SEC-AND-CP-002",
                format!("ContentProvider Grants Unrestricted URI Permissions: {name}"),
                format!(
                    "The ContentProvider '{name}' sets android:grantUriPermissions=\"true\", which allows temporary access \
                    delegation to any caller holding an intent URI with FLAG_GRANT_READ_URI_PERMISSION or FLAG_GRANT_WRITE_URI_PERMISSION. \
                    If intent redirection exists anywhere in the app, this can be leveraged to exfiltrate private files or databases."
                ),
                Severity::Medium,
                Platform::Android,
            );
            f.file_path = file_path.to_string();
            f.snippet = attrs.lines().take(4).collect::<Vec<_>>().join(" ");
            f.remediation = "Use scoped <grant-uri-permission> elements with specific sub-paths rather than enabling unrestricted \
                grantUriPermissions=\"true\" globally on the entire provider."
                    .to_string();
            findings.push(f);
        }
    }

    findings
}

/// Audit Java/Kotlin/Smali code for SQL injection and path traversal in ContentProviders
pub fn audit_provider_source_code(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Check for SQL injection via string concatenation in query/rawQuery
    let sql_re = get_sql_concat_regex();
    for (line_no, line) in content.lines().enumerate() {
        if sql_re.is_match(line) {
            // Filter out safe string constants if no variables are being concatenated
            if line.contains('"') && (line.contains('+') || line.contains(".append(") || line.contains("$")) {
                let mut f = Finding::new(
                    "SEC-AND-CP-SQLI",
                    "Potential SQL Injection in ContentProvider / Database Query",
                    format!(
                        "A database operation in {file_path}:{line_no} appears to concatenate unvalidated inputs directly \
                        into an SQL statement or selection clause. If this ContentProvider is exported or accessible to \
                        untrusted intents, an attacker can manipulate the query logic to extract unauthorized rows or bypass authentication."
                    ),
                    Severity::High,
                    Platform::Android,
                );
                f.file_path = file_path.to_string();
                f.line = line_no + 1;
                f.snippet = line.trim().to_string();
                f.remediation = "Use parameterized queries with selectionArgs ('?') instead of string concatenation, or use an ORM like Room with compile-time query verification."
                    .to_string();
                f.gate = GateResult {
                    status: GateStatus::Passed,
                    notes: vec![
                        "High severity database injection flaw. Verifiable with adb shell content query.".to_string(),
                    ],
                    chain_eligible: true,
                };
                findings.push(f);
            }
        }
    }

    // Check for Path Traversal in openFile / openAssetFile
    let path_re = get_path_traversal_regex();
    if path_re.is_match(content) {
        let has_canonical_check = content.contains("getCanonicalPath()")
            || content.contains("canonicalPath")
            || content.contains("normalize()")
            || content.contains("startsWith(");

        if !has_canonical_check {
            for (line_no, line) in content.lines().enumerate() {
                if line.contains("openFile(") || line.contains("openAssetFile(") || line.contains("openFileHelper(") {
                    let mut f = Finding::new(
                        "SEC-AND-CP-PATH-TRAVERSAL",
                        "ContentProvider Path Traversal in openFile() / openAssetFile()",
                        format!(
                            "ContentProvider file-access method detected at line {line_no} without canonical path verification. \
                            An attacker can supply URI segments containing '../' to traverse outside the designated file root \
                            and read arbitrary private app data or sandbox files."
                        ),
                        Severity::High,
                        Platform::Android,
                    );
                    f.file_path = file_path.to_string();
                    f.line = line_no + 1;
                    f.snippet = line.trim().to_string();
                    f.remediation = "Validate the canonical path of the target file using file.getCanonicalPath().startsWith(baseDir.getCanonicalPath()). \
                        Reject URIs containing directory traversal characters."
                            .to_string();
                    f.gate = GateResult {
                        status: GateStatus::Passed,
                        notes: vec![
                            "Arbitrary file disclosure via directory traversal in exported file provider.".to_string(),
                        ],
                        chain_eligible: true,
                    };
                    findings.push(f);
                    break;
                }
            }
        }
    }

    findings
}
