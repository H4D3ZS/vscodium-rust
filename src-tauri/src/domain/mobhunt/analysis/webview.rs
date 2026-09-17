use crate::domain::mobhunt::models::{CvssData, Finding, GateResult, GateStatus, Platform, Severity};
use regex::Regex;
use std::collections::HashMap;

pub struct WebViewPattern {
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

pub static WEBVIEW_PATTERNS: &[WebViewPattern] = &[
    WebViewPattern {
        id: "WV-UNIVERSAL-ACCESS",
        title: "WebView Universal Access from File URLs Enabled",
        pattern: r#"(?i)\.setAllowUniversalAccessFromFileURLs\s*\(\s*true\s*\)"#,
        severity: Severity::Critical,
        cvss: 8.8,
        cwe: "CWE-94",
        masvs: "MASVS-PLATFORM-6",
        description: "Application enables `setAllowUniversalAccessFromFileURLs(true)`. Scripts running in the context of local file:// URLs are permitted to access content from any origin (including reading arbitrary private application sandboxed files, sqlite databases, and cookies).",
        remediation: "Explicitly set `setAllowUniversalAccessFromFileURLs(false)` and `setAllowFileAccessFromFileURLs(false)`.",
    },
    WebViewPattern {
        id: "WV-FILE-ACCESS-URLS",
        title: "WebView File Access from File URLs Enabled",
        pattern: r#"(?i)\.setAllowFileAccessFromFileURLs\s*\(\s*true\s*\)"#,
        severity: Severity::High,
        cvss: 7.5,
        cwe: "CWE-94",
        masvs: "MASVS-PLATFORM-6",
        description: "Application enables `setAllowFileAccessFromFileURLs(true)`. Allows JavaScript from a file:// URL to access other local files in the application sandbox.",
        remediation: "Disable `setAllowFileAccessFromFileURLs(false)`.",
    },
    WebViewPattern {
        id: "WV-FILE-ACCESS",
        title: "WebView Local File Access Enabled",
        pattern: r#"(?i)\.setAllowFileAccess\s*\(\s*true\s*\)"#,
        severity: Severity::Medium,
        cvss: 5.3,
        cwe: "CWE-200",
        masvs: "MASVS-PLATFORM-6",
        description: "Application explicitly enables `setAllowFileAccess(true)`. If WebView can be navigated to untrusted URLs or load deep links, attackers can access local assets via file:// schemes.",
        remediation: "Set `setAllowFileAccess(false)` unless strictly necessary for local HTML assets.",
    },
    WebViewPattern {
        id: "WV-DEBUGGING-ENABLED",
        title: "WebView Remote Content Debugging Enabled",
        pattern: r#"(?i)WebView\.setWebContentsDebuggingEnabled\s*\(\s*true\s*\)"#,
        severity: Severity::High,
        cvss: 7.1,
        cwe: "CWE-215",
        masvs: "MASVS-CODE-2",
        description: "Application enables Chrome DevTools remote debugging via `setWebContentsDebuggingEnabled(true)`. Any process or developer with ADB access can inspect DOM, execute arbitrary JavaScript, and steal session tokens and localStorage data.",
        remediation: "Ensure `setWebContentsDebuggingEnabled(false)` or guard with `if (BuildConfig.DEBUG)`.",
    },
    WebViewPattern {
        id: "WV-JS-BRIDGE-EXPOSED",
        title: "WebView JavaScript Interface Bridge Exposed",
        pattern: r#"(?i)\.addJavascriptInterface\s*\(\s*[^,]+,\s*["']([^"']+)["']\s*\)"#,
        severity: Severity::High,
        cvss: 7.8,
        cwe: "CWE-749",
        masvs: "MASVS-PLATFORM-6",
        description: "Application injects native Java/Kotlin objects into the WebView DOM via `addJavascriptInterface()`. Any page rendered in this WebView (including XSS or MITM injections) can invoke exposed public methods.",
        remediation: "Only expose interfaces to trusted domains. Ensure all exposed methods have the `@JavascriptInterface` annotation and never expose file system, Intent, or credential retrieval methods.",
    },
    WebViewPattern {
        id: "WV-DEEPLINK-LOADURL",
        title: "Unvalidated Deep Link / Intent Extra Loaded Directly into WebView",
        pattern: r#"(?i)\.loadUrl\s*\(\s*(?:intent\.getStringExtra|getIntent\(\)\.getStringExtra|uri\.getQueryParameter|data\.getQueryParameter)\s*\("#,
        severity: Severity::Critical,
        cvss: 8.8,
        cwe: "CWE-79",
        masvs: "MASVS-PLATFORM-6",
        description: "Application reads a URL parameter directly from an Intent extra or deep link query parameter and passes it straight into `loadUrl()` without domain whitelist validation.",
        remediation: "Validate that the URL's scheme is strictly 'https://' and the host matches an allowed internal domain whitelist before invoking `loadUrl()`.",
    },
    WebViewPattern {
        id: "WV-BRIDGE-FILE-ACCESS",
        title: "JavaScript Bridge Exposes Dangerous File System Operations",
        pattern: r#"(?i)@JavascriptInterface[\s\S]{1,150}(?:readFile|writeFile|deleteFile|openFileOutput|FileInputStream|FileOutputStream)"#,
        severity: Severity::Critical,
        cvss: 8.5,
        cwe: "CWE-749",
        masvs: "MASVS-PLATFORM-6",
        description: "Exposed JavaScript bridge method implements file reading or writing operations. A compromised web page can read or overwrite files in the application's private directory.",
        remediation: "Do not expose filesystem primitives over JavaScript bridges. Keep file operations strictly in native code with rigid internal path validation.",
    },
    WebViewPattern {
        id: "WV-BRIDGE-INTENT-LAUNCH",
        title: "JavaScript Bridge Exposes Native Intent Launching Capability",
        pattern: r#"(?i)@JavascriptInterface[\s\S]{1,150}(?:startActivity|sendBroadcast|startService|PendingIntent\.send)"#,
        severity: Severity::High,
        cvss: 8.2,
        cwe: "CWE-749",
        masvs: "MASVS-PLATFORM-6",
        description: "Exposed JavaScript bridge method allows JavaScript to invoke `startActivity()`, `sendBroadcast()`, or fire `PendingIntent`s. Untrusted web content can trigger arbitrary internal activities or privileged components.",
        remediation: "Do not allow JavaScript to construct or launch arbitrary Android Intents.",
    },
    WebViewPattern {
        id: "WV-BRIDGE-DATA-THEFT",
        title: "JavaScript Bridge Exposes Authentication Tokens or Credentials",
        pattern: r#"(?i)@JavascriptInterface[\s\S]{1,150}(?:getToken|getAuthToken|getApiKey|getSecret|getPassword|getCredentials|SharedPreferences)"#,
        severity: Severity::Critical,
        cvss: 8.6,
        cwe: "CWE-749",
        masvs: "MASVS-PLATFORM-6",
        description: "Exposed JavaScript bridge method returns auth tokens, API keys, or SharedPreferences contents to the web layer. If the WebView navigates to an external site or suffers XSS, the session token is immediately compromised.",
        remediation: "Do not return raw tokens or secrets to JavaScript. Use secure HTTP-only cookies or dedicated restricted message passing.",
    },
];

/// Analyzes source code, smali, or decompiled Java/Kotlin files for WebView and JavaScript bridge vulnerabilities
pub fn audit_webview_and_bridges(content: &str, file_path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for pattern in WEBVIEW_PATTERNS {
        if let Ok(re) = Regex::new(pattern.pattern) {
            for (line_idx, line) in content.lines().enumerate() {
                if let Some(mat) = re.find(line) {
                    findings.push(Finding {
                        id: format!("{}-{}", pattern.id, line_idx + 1),
                        rule_id: pattern.id.to_string(),
                        title: pattern.title.to_string(),
                        vuln_class: pattern.masvs.to_string(),
                        severity: pattern.severity,
                        platform: Platform::Android,
                        file_path: file_path.to_string(),
                        line: line_idx + 1,
                        snippet: mat.as_str().trim().to_string(),
                        description: pattern.description.to_string(),
                        impact: format!("Violates {} ({}). High risk of Cross-Site Scripting, local file theft, or privilege escalation.", pattern.masvs, pattern.cwe),
                        remediation: pattern.remediation.to_string(),
                        cvss: CvssData {
                            score: pattern.cvss,
                            severity: pattern.severity,
                            vector: "CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:N".to_string(),
                            exploitability: 2.8,
                            impact: 5.2,
                        },
                        gate: GateResult {
                            status: GateStatus::Passed,
                            notes: vec![format!("Matches WebView vulnerability pattern {}", pattern.id)],
                            chain_eligible: true,
                        },
                        reproduction_steps: vec![
                            format!("Inspect WebView configuration at `{}:{}`.", file_path, line_idx + 1),
                            format!("Found snippet: `{}`", mat.as_str().trim()),
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
    fn test_webview_patterns() {
        let code = r#"
            webView.getSettings().setJavaScriptEnabled(true);
            webView.getSettings().setAllowUniversalAccessFromFileURLs(true);
            webView.loadUrl(getIntent().getStringExtra("target_url"));
        "#;
        let findings = audit_webview_and_bridges(code, "InsecureWebViewActivity.java");
        assert!(findings.iter().any(|f| f.rule_id == "WV-UNIVERSAL-ACCESS"));
        assert!(findings.iter().any(|f| f.rule_id == "WV-DEEPLINK-LOADURL"));
    }
}
