//! Skill security profiling — agentskills.io / skills.sh compatible SKILL.md trees.
//!
//! Red-team skills legitimately mention payloads, backdoors, and shells in prose.
//! Those are tagged `red-team-context` (info) unless executable scripts contain
//! actual invocation patterns.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AuditSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuditCategory {
    RedTeamContext,
    SuspiciousPattern,
    ExecutablePayload,
    NetworkExfil,
    Obfuscation,
    Structure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub severity: AuditSeverity,
    pub category: AuditCategory,
    pub file: String,
    pub line: usize,
    pub message: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAuditReport {
    pub skill_id: String,
    pub skill_path: String,
    pub red_team_skill: bool,
    pub safe_to_use: bool,
    pub blocked: bool,
    pub summary: String,
    pub findings: Vec<AuditFinding>,
    pub info_count: usize,
    pub warning_count: usize,
    pub critical_count: usize,
}

const RED_TEAM_PATH_HINTS: &[&str] = &[
    "red-team", "redteam", "red_team", "offensive", "pentest", "penetration",
    "exploit", "cybersecurity", "security-research", "threat", "malware",
    "reverse-shell", "payload", "c2", "command-and-control",
];

const RED_TEAM_TAG_HINTS: &[&str] = &[
    "red-team", "redteam", "offensive", "pentest", "security", "exploit",
    "cyber", "malware", "payload",
];

const DOC_KEYWORDS: &[&str] = &[
    "backdoor", "reverse shell", "reverse-shell", "payload", "shellcode",
    "exfil", "c2", "command and control", "rootkit", "keylogger", "ransomware",
    "privilege escalation", "lateral movement", "persistence",
];

const SCRIPT_EXTENSIONS: &[&str] = &["py", "sh", "bash", "ps1", "bat", "cmd", "js", "ts", "rb", "pl"];

fn is_red_team_skill(skill_root: &Path, frontmatter: &str, skill_id: &str) -> bool {
    let blob = format!(
        "{} {} {}",
        skill_id.to_lowercase(),
        skill_root.to_string_lossy().to_lowercase(),
        frontmatter.to_lowercase()
    );
    if RED_TEAM_PATH_HINTS.iter().any(|h| blob.contains(h)) {
        return true;
    }
    for line in frontmatter.lines() {
        let l = line.to_lowercase();
        if l.contains("tags:") || l.contains("category:") {
            if RED_TEAM_TAG_HINTS.iter().any(|t| l.contains(t)) {
                return true;
            }
        }
    }
    false
}

fn is_script_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| SCRIPT_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn is_markdown(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.eq_ignore_ascii_case("SKILL.md") || n.ends_with(".md"))
        .unwrap_or(false)
}

fn push_finding(
    findings: &mut Vec<AuditFinding>,
    severity: AuditSeverity,
    category: AuditCategory,
    file: &str,
    line: usize,
    message: &str,
    snippet: &str,
) {
    findings.push(AuditFinding {
        severity,
        category,
        file: file.to_string(),
        line,
        message: message.to_string(),
        snippet: snippet.chars().take(200).collect(),
    });
}

fn scan_line_scripts(
    findings: &mut Vec<AuditFinding>,
    rel: &str,
    line_no: usize,
    line: &str,
    red_team: bool,
) {
    let lower = line.to_lowercase();

    let critical_patterns: &[(&str, &str)] = &[
        (r"(?i)curl\s+[^\n|]+\|\s*(ba)?sh", "curl pipe to shell"),
        (r"(?i)wget\s+[^\n|]+\|\s*(ba)?sh", "wget pipe to shell"),
        (r"(?i)iex\s*\(\s*irm\s", "PowerShell remote script execution (iex irm)"),
        (r"(?i)invoke-expression\s*\(", "Invoke-Expression"),
        (r"(?i)eval\s*\(\s*base64", "eval(base64...)"),
        (r"(?i)frombase64string.*iex", "base64 decode + IEX chain"),
        (r#"(?i)os\.system\s*\(\s*['"]curl"#, "os.system curl fetch"),
        (r"(?i)subprocess\.[a-z]+\([^)]*shell\s*=\s*True", "Python subprocess with shell=True"),
    ];

    for (pat, msg) in critical_patterns {
        if let Ok(re) = Regex::new(pat) {
            if re.is_match(line) {
                push_finding(
                    findings,
                    AuditSeverity::Critical,
                    AuditCategory::ExecutablePayload,
                    rel,
                    line_no,
                    msg,
                    line.trim(),
                );
            }
        }
    }

    if lower.contains("base64") && (lower.contains("decode") || lower.contains("b64decode")) {
        if lower.contains("exec") || lower.contains("eval") || lower.contains("iex") {
            push_finding(
                findings,
                AuditSeverity::Critical,
                AuditCategory::Obfuscation,
                rel,
                line_no,
                "Base64 decode followed by execution",
                line.trim(),
            );
        }
    }

    if !red_team {
        if lower.contains("discord.com/api/webhooks") || lower.contains("pastebin.com/raw") {
            push_finding(
                findings,
                AuditSeverity::Warning,
                AuditCategory::NetworkExfil,
                rel,
                line_no,
                "Possible outbound data exfil endpoint in script",
                line.trim(),
            );
        }
    }
}

fn scan_line_docs(
    findings: &mut Vec<AuditFinding>,
    rel: &str,
    line_no: usize,
    line: &str,
    red_team: bool,
) {
    let lower = line.to_lowercase();
    for kw in DOC_KEYWORDS {
        if lower.contains(kw) {
            let (sev, cat, msg) = if red_team {
                (
                    AuditSeverity::Info,
                    AuditCategory::RedTeamContext,
                    "Red-team documentation keyword (expected for this skill category)",
                )
            } else {
                (
                    AuditSeverity::Warning,
                    AuditCategory::SuspiciousPattern,
                    "Security-sensitive keyword in skill documentation",
                )
            };
            push_finding(findings, sev, cat, rel, line_no, msg, line.trim());
            break;
        }
    }
}

fn read_frontmatter(skill_md: &Path) -> String {
    let Ok(content) = std::fs::read_to_string(skill_md) else {
        return String::new();
    };
    let body = content.trim();
    if !body.starts_with("---") {
        return String::new();
    }
    if let Some(end) = body[3..].find("\n---") {
        return body[3..3 + end].to_string();
    }
    String::new()
}

/// Audit a skill directory (must contain SKILL.md).
pub fn audit_skill_tree(skill_root: &Path, skill_id: &str) -> SkillAuditReport {
    let mut findings = Vec::new();
    let skill_md = skill_root.join("SKILL.md");
    if !skill_md.is_file() {
        push_finding(
            &mut findings,
            AuditSeverity::Critical,
            AuditCategory::Structure,
            "SKILL.md",
            0,
            "Missing SKILL.md - not a valid agentskills.io skill",
            "",
        );
        return finalize_report(skill_id, skill_root, false, findings);
    }

    let fm = read_frontmatter(&skill_md);
    let red_team = is_red_team_skill(skill_root, &fm, skill_id);

    for entry in WalkDir::new(skill_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path
            .components()
            .any(|c| c.as_os_str() == std::ffi::OsStr::new(".git"))
        {
            continue;
        }
        let rel = path
            .strip_prefix(skill_root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| path.to_string_lossy().to_string());

        for (i, line) in std::fs::read_to_string(path)
            .unwrap_or_default()
            .lines()
            .enumerate()
        {
            let line_no = i + 1;
            if is_script_path(path) {
                scan_line_scripts(&mut findings, &rel, line_no, line, red_team);
            } else if is_markdown(path) || path.extension().is_none() {
                scan_line_docs(&mut findings, &rel, line_no, line, red_team);
            }
        }
    }

    finalize_report(skill_id, skill_root, red_team, findings)
}

fn finalize_report(
    skill_id: &str,
    skill_root: &Path,
    red_team: bool,
    findings: Vec<AuditFinding>,
) -> SkillAuditReport {
    let info_count = findings
        .iter()
        .filter(|f| f.severity == AuditSeverity::Info)
        .count();
    let warning_count = findings
        .iter()
        .filter(|f| f.severity == AuditSeverity::Warning)
        .count();
    let critical_count = findings
        .iter()
        .filter(|f| f.severity == AuditSeverity::Critical)
        .count();

    let blocked = critical_count > 0;
    let safe_to_use = !blocked && warning_count == 0;
    let summary = if blocked {
        format!(
            "{critical_count} critical issue(s) - install blocked until reviewed"
        )
    } else if warning_count > 0 {
        format!(
            "{warning_count} warning(s), {info_count} info - review before trusting"
        )
    } else if red_team {
        "Red-team skill - documentation keywords flagged as context only".to_string()
    } else {
        "No suspicious patterns detected".to_string()
    };

    SkillAuditReport {
        skill_id: skill_id.to_string(),
        skill_path: skill_root.to_string_lossy().to_string(),
        red_team_skill: red_team,
        safe_to_use,
        blocked,
        summary,
        findings,
        info_count,
        warning_count,
        critical_count,
    }
}

pub fn audit_report_to_json(report: &SkillAuditReport) -> serde_json::Value {
    serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({}))
}
