use crate::domain::mobhunt::models::{AppMetadata, Finding};
use chrono::Local;

pub fn generate_h1_report(finding: &Finding, app: &AppMetadata) -> String {
    let now = Local::now().format("%Y-%m-%d").to_string();
    let platform_str = finding.platform.as_str();
    let app_id = if !app.identifier.is_empty() {
        &app.identifier
    } else {
        "N/A"
    };

    let repro_steps = if finding.reproduction_steps.is_empty() {
        "1. Inspect the application binary or decompiled bundle at the specified file path.\n\
         2. Verify the vulnerable configuration/artifact indicated in the snippet.\n\
         3. Trigger the IPC call or execute dynamic inspection via Frida over USB."
            .to_string()
    } else {
        finding
            .reproduction_steps
            .iter()
            .enumerate()
            .map(|(idx, s)| format!("{}. {}", idx + 1, s))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"# {title}

**Target Application:** {app_name} (`{app_id}`)
**Version:** {app_ver} (Build {app_build})
**Platform:** {platform_str}
**Severity:** {severity} (CVSS: {cvss_score:.1} - `{cvss_vector}`)
**Date:** {date}

---

## 1. Vulnerability Summary
{description}

- **Vulnerability Class:** `{vuln_class}`
- **Rule Identifier:** `{rule_id}`
- **Affected Component / Path:** `{file_path}:{line}`

```
{snippet}
```

---

## 2. Technical Details & Root Cause
The mobile application `{app_id}` exposes a security boundary flaw or insecure configuration in `{file_path}`.
When inspected, the asset violates OWASP Mobile Application Security Verification Standard (MASVS) controls.

---

## 3. Steps to Reproduce
{repro_steps}

---

## 4. Impact
{impact}

---

## 5. Suggested Remediation
{remediation}

---
*Report generated natively via MobHunt Mobile Security Engine (Sentinel/Apex) · AI Assisted by ModelScope (Official Qwen Ambassador Pipeline).*
"#,
        title = finding.title,
        app_name = if app.name.is_empty() { "Target Mobile App" } else { &app.name },
        app_id = app_id,
        app_ver = if app.version.is_empty() { "1.0.0" } else { &app.version },
        app_build = if app.build.is_empty() { "1" } else { &app.build },
        platform_str = platform_str,
        severity = finding.severity.as_str(),
        cvss_score = finding.cvss.score,
        cvss_vector = finding.cvss.vector,
        date = now,
        description = finding.description,
        vuln_class = finding.vuln_class,
        rule_id = finding.rule_id,
        file_path = finding.file_path,
        line = finding.line,
        snippet = finding.snippet,
        repro_steps = repro_steps,
        impact = finding.impact,
        remediation = finding.remediation,
    )
}
