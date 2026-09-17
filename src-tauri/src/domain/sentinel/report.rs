//! Report renderer: turns a finding into platform-ready markdown (HackerOne /
//! Bugcrowd / Intigriti / YesWeHack) with a CVSS 3.1 estimate. No external
//! scoring dependency — mappings are kept embedded so it compiles in both the
//! Tauri and headless-native builds.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ReportPlatform {
    HackerOne,
    Bugcrowd,
    Intigriti,
    YesWeHack,
    Generic,
}

impl ReportPlatform {
    pub fn label(&self) -> &'static str {
        match self {
            ReportPlatform::HackerOne => "HackerOne",
            ReportPlatform::Bugcrowd => "Bugcrowd",
            ReportPlatform::Intigriti => "Intigriti",
            ReportPlatform::YesWeHack => "YesWeHack",
            ReportPlatform::Generic => "Generic",
        }
    }

    pub fn from_label(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hackerone" | "h1" => Some(ReportPlatform::HackerOne),
            "bugcrowd" | "bc" => Some(ReportPlatform::Bugcrowd),
            "intigriti" | "inti" => Some(ReportPlatform::Intigriti),
            "yeswehack" | "ywh" => Some(ReportPlatform::YesWeHack),
            "generic" => Some(ReportPlatform::Generic),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportInput {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub severity: String, // critical/high/medium/low/info
    #[serde(default)]
    pub target: String, // domain or app name
    #[serde(default)]
    pub cwe: String,
    #[serde(default)]
    pub owasp: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub evidence: String, // request/response or PoC snippet
    #[serde(default)]
    pub remediation: String,
    #[serde(default)]
    pub recommendation: String,
}

impl ReportInput {
    pub fn new(title: &str, target: &str, severity: &str) -> Self {
        Self {
            title: title.to_string(),
            severity: severity.to_string(),
            target: target.to_string(),
            cwe: String::new(),
            owasp: String::new(),
            description: String::new(),
            evidence: String::new(),
            remediation: String::new(),
            recommendation: String::new(),
        }
    }
}

/// Map a severity to a CVSS 3.1 base-vector estimate (for triage guidance).
pub fn severity_cvss(severity: &str) -> (f32, &'static str) {
    match severity.to_lowercase().as_str() {
        "critical" => (9.8, "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"),
        "high" => (8.1, "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:L/A:L"),
        "medium" => (5.3, "CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:L/I:L/A:N"),
        "low" => (3.1, "CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:U/C:L/I:N/A:N"),
        _ => (0.0, "CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:U/C:N/I:N/A:N"),
    }
}

fn esc(code: &str) -> String {
    code.replace('`', "\\`")
}

fn snippet_block(s: &str) -> String {
    let lines: Vec<&str> = s.lines().take(120).collect();
    format!("```\n{}\n```", lines.join("\n"))
}

/// Render a full markdown report following the HackerOne/Bugcrowd template.
pub fn render_markdown(platform: ReportPlatform, input: &ReportInput) -> String {
    let (score, vector) = severity_cvss(&input.severity);
    let evidence_block = if input.evidence.trim().is_empty() {
        String::new()
    } else {
        format!(
            "\n### Steps to Reproduce / Evidence\n{}\n",
            snippet_block(&input.evidence)
        )
    };
    let rem = if input.remediation.trim().is_empty() {
        String::new()
    } else {
        format!("\n### Remediation\n{}\n", esc(&input.remediation))
    };
    let rec = if input.recommendation.trim().is_empty() {
        String::new()
    } else {
        format!("\n### Recommendation\n{}\n", esc(&input.recommendation))
    };

    match platform {
        ReportPlatform::HackerOne => format!(
            "# {} \n\n- Target: {}\n- Severity: {} (CVSS {:.1} — {})\n- CWE: {}\n- OWASP: {}\n\n## Summary\n{}\n{}",
            input.title,
            input.target,
            input.severity.to_uppercase(),
            score,
            vector,
            first_cwe(&input.cwe),
            first_owasp(&input.owasp),
            snippet_block(&input.description),
            evidence_block,
        ),
        ReportPlatform::Bugcrowd => format!(
            "# {} \n\n**Target:** {}\n**Severity:** {} (CVSS {:.1} — {})\n**CWE:** {}\n**OWASP:** {}\n\n## Description\n{}\n{}",
            input.title,
            input.target,
            input.severity.to_uppercase(),
            score,
            vector,
            first_cwe(&input.cwe),
            first_owasp(&input.owasp),
            snippet_block(&input.description),
            evidence_block,
        ),
        ReportPlatform::Intigriti => format!(
            "# {} \n\n**Target:** {}\n**Impact:** {}\n**CWE:** {}\n\n## Description\n{}\n{}",
            input.title,
            input.target,
            input.severity.to_uppercase(),
            first_cwe(&input.cwe),
            snippet_block(&input.description),
            evidence_block,
        ),
        ReportPlatform::YesWeHack => format!(
            "# {} \n\n- Asset: {}\n- Bug class: {}\n- CVE/CWE: {}\n\n## Reproduction\n{}\n{}",
            input.title,
            input.target,
            first_owasp(&input.owasp),
            first_cwe(&input.cwe),
            snippet_block(&input.description),
            evidence_block,
        ),
        ReportPlatform::Generic => format!(
            "# {} \n\n- Target: {}\n- Severity: {}\n- CWE: {}\n- OWASP: {}\n\n## Summary\n{}\n{}\n{}\n{}\n",
            input.title,
            input.target,
            input.severity.to_uppercase(),
            first_cwe(&input.cwe),
            first_owasp(&input.owasp),
            snippet_block(&input.description),
            evidence_block,
            rem,
            rec,
        ),
    }
}

fn first_cwe(s: &str) -> String {
    match s.split(',').next() {
        Some(v) if !v.trim().is_empty() => v.trim().to_string(),
        _ => "CWE-000".to_string(),
    }
}

fn first_owasp(s: &str) -> String {
    match s.split(',').next() {
        Some(v) if !v.trim().is_empty() => v.trim().to_string(),
        _ => "A16".to_string(),
    }
}

/// Write a rendered report to `work_dir/reports/<target>--<title>.md`.
pub fn write_report(
    work_dir: &Path,
    platform: ReportPlatform,
    input: &ReportInput,
) -> Result<std::path::PathBuf, String> {
    let dir = work_dir.join("reports");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let target = slug(&input.target);
    let title = slug(&input.title);
    let path = dir.join(format!("{}--{}.md", target, title));
    std::fs::write(&path, render_markdown(platform, input)).map_err(|e| e.to_string())?;
    Ok(path)
}

fn slug(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h1_report_contains_cvss() {
        let r = ReportInput::new("SQL Injection in search", "example.com", "high");
        let md = render_markdown(ReportPlatform::HackerOne, &r);
        assert!(md.contains("CVSS:3.1"));
        assert!(md.contains("HIGH"));
        assert!(md.contains("example.com"));
    }

    #[test]
    fn severity_mapping() {
        assert_eq!(severity_cvss("Critical").0, 9.8);
        assert_eq!(severity_cvss("high").0, 8.1);
        assert_eq!(severity_cvss("Low").0, 3.1);
    }

    #[test]
    fn evidence_is_rendered() {
        let mut r = ReportInput::new("XSS", "t", "medium");
        r.evidence = "GET /x HTTP/1.1\nHost: t".to_string();
        let md = render_markdown(ReportPlatform::Bugcrowd, &r);
        assert!(md.contains("GET /x"));
        assert!(md.contains("Steps to Reproduce"));
    }
}
