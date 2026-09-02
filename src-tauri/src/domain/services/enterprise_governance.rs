//! Enterprise governance — tool/MCP/model policy, engagement scope, DLP redaction.

use crate::enterprise_audit::{append_audit, load_policy, EnterprisePolicy};
use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::OnceLock;

const OFFENSIVE_TOOLS: &[&str] = &[
    "run_command",
    "network_port_scanner",
    "web_security_audit",
    "apex_scan_url",
    "apex_red_team_scan",
    "deep_security_audit",
    "browser_navigate",
    "browser_open",
    "browser_subagent",
    "ai_vuln_hunt",
];

fn url_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)https?://[^\s"'<>]+|(?:\b[\w.-]+\.(?:com|net|org|io|app|dev|local|internal|corp|xyz|co|uk|edu|gov)(?::\d+)?)\b"#)
            .expect("url regex")
    })
}

fn host_from_token(token: &str) -> Option<String> {
    let t = token.trim().trim_end_matches(&['/', ',', ';', ')', ']', '"', '\''][..]);
    if t.is_empty() {
        return None;
    }
    if let Some(rest) = t.strip_prefix("http://").or_else(|| t.strip_prefix("https://")) {
        let host = rest.split(&['/', '?', '#'][..]).next().unwrap_or(rest);
        return Some(host.split(':').next().unwrap_or(host).to_ascii_lowercase());
    }
    if t.contains('.') || t == "localhost" {
        return Some(t.split(':').next().unwrap_or(t).to_ascii_lowercase());
    }
    None
}

pub fn extract_hosts_from_value(v: &Value) -> Vec<String> {
    let mut hosts = Vec::new();
    match v {
        Value::String(s) => {
            for m in url_re().find_iter(s) {
                if let Some(h) = host_from_token(m.as_str()) {
                    hosts.push(h);
                }
            }
        }
        Value::Object(map) => {
            for (k, val) in map {
                let kl = k.to_ascii_lowercase();
                if kl.contains("url") || kl.contains("host") || kl.contains("target") || kl == "command" {
                    hosts.extend(extract_hosts_from_value(val));
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                hosts.extend(extract_hosts_from_value(item));
            }
        }
        _ => {}
    }
    hosts.sort();
    hosts.dedup();
    hosts
}

fn is_private_or_local_host(host: &str) -> bool {
    let h = host.to_ascii_lowercase();
    if h == "localhost" || h.ends_with(".local") || h.ends_with(".internal") {
        return true;
    }
    if let Ok(ip) = h.parse::<std::net::IpAddr>() {
        return match ip {
            std::net::IpAddr::V4(v4) => {
                let o = v4.octets();
                o[0] == 10
                    || o[0] == 127
                    || (o[0] == 172 && (16..=31).contains(&o[1]))
                    || (o[0] == 192 && o[1] == 168)
                    || (o[0] == 169 && o[1] == 254)
            }
            std::net::IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified(),
        };
    }
    false
}

fn host_matches_pattern(host: &str, pattern: &str) -> bool {
    let h = host.to_ascii_lowercase();
    let p = pattern.trim().to_ascii_lowercase();
    if p.is_empty() {
        return false;
    }
    if p == "*" {
        return true;
    }
    if p.starts_with("*.") {
        let suffix = &p[1..];
        return h.ends_with(suffix) || h == p.trim_start_matches('*').trim_start_matches('.');
    }
    h == p || h.ends_with(&format!(".{p}"))
}

fn host_in_engagement(host: &str, targets: &[String]) -> bool {
    targets.iter().any(|t| host_matches_pattern(host, t))
}

pub fn is_offensive_tool(name: &str) -> bool {
    OFFENSIVE_TOOLS.contains(&name)
}

pub struct GovernanceDecision {
    pub allowed: bool,
    pub reason: String,
}

pub fn evaluate_tool(
    config_dir: &Path,
    tool_name: &str,
    args: &Value,
    agent_mode: Option<&str>,
) -> GovernanceDecision {
    let policy = load_policy(config_dir);

    if policy.tool_denylist.iter().any(|t| t == tool_name) {
        return GovernanceDecision {
            allowed: false,
            reason: format!("Tool '{tool_name}' is denied by org policy"),
        };
    }

    if !policy.tool_allowlist.is_empty()
        && !policy.tool_allowlist.iter().any(|t| t == tool_name)
    {
        return GovernanceDecision {
            allowed: false,
            reason: format!("Tool '{tool_name}' is not on the org allowlist"),
        };
    }

    let sec_mode = agent_mode
        .map(|m| {
            matches!(
                m,
                "BugBounty" | "Bug Bounty" | "RedTeam" | "Red Team" | "BlueTeam" | "Blue Team"
            )
        })
        .unwrap_or(false);

    let offensive = is_offensive_tool(tool_name) || sec_mode;
    let hosts = extract_hosts_from_value(args);

    if policy.block_private_network_scan && offensive {
        for h in &hosts {
            if is_private_or_local_host(h) {
                return GovernanceDecision {
                    allowed: false,
                    reason: format!(
                        "Blocked scan/command against private/local host '{h}' (org policy block_private_network_scan)"
                    ),
                };
            }
        }
    }

    if policy.require_engagement_scope && offensive && !policy.engagement_targets.is_empty() {
        if hosts.is_empty() {
            return GovernanceDecision {
                allowed: false,
                reason: "Engagement scope required — include an in-scope URL/host in tool args"
                    .into(),
            };
        }
        for h in &hosts {
            if !host_in_engagement(h, &policy.engagement_targets) {
                return GovernanceDecision {
                    allowed: false,
                    reason: format!(
                        "Host '{h}' is outside engagement scope: {:?}",
                        policy.engagement_targets
                    ),
                };
            }
        }
    } else if offensive && !policy.engagement_targets.is_empty() {
        for h in &hosts {
            if !host_in_engagement(h, &policy.engagement_targets) {
                return GovernanceDecision {
                    allowed: false,
                    reason: format!(
                        "Host '{h}' not in engagement allowlist: {:?}",
                        policy.engagement_targets
                    ),
                };
            }
        }
    }

    GovernanceDecision {
        allowed: true,
        reason: String::new(),
    }
}

pub fn model_allowed(config_dir: &Path, model: &str) -> Result<(), String> {
    let policy = load_policy(config_dir);
    let m = model.to_ascii_lowercase();
    if policy
        .blocked_models
        .iter()
        .any(|b| m.contains(&b.to_ascii_lowercase()))
    {
        return Err(format!("Model blocked by org policy: {model}"));
    }
    if !policy.allowed_models.is_empty()
        && !policy
            .allowed_models
            .iter()
            .any(|a| m.contains(&a.to_ascii_lowercase()))
    {
        return Err(format!(
            "Model not on org allowlist (allowed: {:?})",
            policy.allowed_models
        ));
    }
    if policy.offline_only {
        let cloud_markers = [
            "cyberifrit",
            "anthropic",
            "openai",
            "claude",
            "gpt-",
            "gemini",
            "openrouter",
        ];
        if cloud_markers.iter().any(|mark| m.contains(mark)) {
            return Err("Org policy offline_only — cloud models disabled".into());
        }
    }
    Ok(())
}

pub fn mcp_server_allowed(config_dir: &Path, server_id: &str) -> Result<(), String> {
    let policy = load_policy(config_dir);
    if policy.allowed_mcp_servers.is_empty() {
        return Ok(());
    }
    if policy
        .allowed_mcp_servers
        .iter()
        .any(|s| s == server_id || s == "*")
    {
        Ok(())
    } else {
        Err(format!(
            "MCP server '{server_id}' not on org allowlist: {:?}",
            policy.allowed_mcp_servers
        ))
    }
}

pub fn redact_secrets_in_value(config_dir: &Path, v: &Value) -> Value {
    let policy = load_policy(config_dir);
    if !policy.dlp_redact_secrets {
        return v.clone();
    }
    redact_value(v)
}

fn redact_value(v: &Value) -> Value {
    match v {
        Value::String(s) => Value::String(redact_string(s)),
        Value::Array(a) => Value::Array(a.iter().map(redact_value).collect()),
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, val)| (k.clone(), redact_value(val)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn redact_string(s: &str) -> String {
    static KEY_RE: OnceLock<Regex> = OnceLock::new();
    static AWS_RE: OnceLock<Regex> = OnceLock::new();
    let key_re = KEY_RE.get_or_init(|| {
        Regex::new(r#"(?i)(api[_-]?key|secret|password|token|authorization)\s*[:=]\s*['"]?([^\s'"]{8,})"#)
            .unwrap()
    });
    static BEARER_RE: OnceLock<Regex> = OnceLock::new();
    let bearer = BEARER_RE.get_or_init(|| Regex::new(r"(?i)Bearer\s+[A-Za-z0-9._-]{12,}").unwrap());
    let aws = AWS_RE.get_or_init(|| Regex::new(r"AKIA[0-9A-Z]{16}").unwrap());
    let mut out = s.to_string();
    out = key_re
        .replace_all(&out, |caps: &regex::Captures| {
            let prefix = caps.get(1).map(|m| m.as_str()).unwrap_or("secret");
            format!("{prefix}=[REDACTED]")
        })
        .to_string();
    out = bearer.replace_all(&out, "Bearer [REDACTED]").to_string();
    out = aws.replace_all(&out, "AKIA[REDACTED]").to_string();
    out
}

pub fn audit_tool_call(
    config_dir: &Path,
    tool: &str,
    args: &Value,
    outcome: &str,
    detail: Option<Value>,
) {
    let policy = load_policy(config_dir);
    if !policy.audit_enabled || !policy.audit_tool_calls {
        return;
    }
    let args_redacted = redact_secrets_in_value(config_dir, args);
    let mut d = json!({
        "tool": tool,
        "outcome": outcome,
        "args": args_redacted,
    });
    if let Some(extra) = detail {
        if let Some(obj) = d.as_object_mut() {
            obj.insert("extra".into(), extra);
        }
    }
    let _ = append_audit(config_dir, "agent", "tool.execute", d);
    if !policy.siem_webhook_url.is_empty() {
        let url = policy.siem_webhook_url.clone();
        let payload = json!({
            "source": "hades-ide",
            "action": "tool.execute",
            "tool": tool,
            "outcome": outcome,
            "ts": chrono::Utc::now().timestamp(),
        });
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let _ = client.post(&url).json(&payload).send().await;
        });
    }
}

pub fn default_cyber_enterprise_policy(org: &str) -> EnterprisePolicy {
    EnterprisePolicy {
        org_name: org.to_string(),
        audit_enabled: true,
        require_secure_mode: true,
        audit_retention_days: 365,
        offline_only: false,
        allowed_models: vec![],
        blocked_models: vec![],
        allowed_mcp_servers: vec![],
        engagement_targets: vec![],
        engagement_id: String::new(),
        require_engagement_scope: false,
        block_private_network_scan: true,
        tool_allowlist: vec![],
        tool_denylist: vec![],
        siem_webhook_url: String::new(),
        dlp_redact_secrets: true,
        audit_tool_calls: true,
        audit_file_writes: true,
        audit_model_calls: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_private_host_when_policy_says_so() {
        let dir = tempfile::tempdir().unwrap();
        let mut p = default_cyber_enterprise_policy("test");
        p.block_private_network_scan = true;
        p.engagement_targets = vec!["example.com".into()];
        crate::enterprise_audit::save_policy(dir.path(), &p).unwrap();
        let args = json!({ "url": "http://127.0.0.1/admin" });
        let d = evaluate_tool(dir.path(), "web_security_audit", &args, Some("BugBounty"));
        assert!(!d.allowed);
    }

    #[test]
    fn allows_in_scope_host() {
        let dir = tempfile::tempdir().unwrap();
        let mut p = default_cyber_enterprise_policy("test");
        p.engagement_targets = vec!["*.example.com".into()];
        crate::enterprise_audit::save_policy(dir.path(), &p).unwrap();
        let args = json!({ "url": "https://api.example.com/v1" });
        let d = evaluate_tool(dir.path(), "apex_scan_url", &args, Some("BugBounty"));
        assert!(d.allowed);
    }
}
