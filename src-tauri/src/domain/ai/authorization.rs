//! Pre-action authorization — least privilege the model can't talk its way past.
//!
//! "Before the Tool Call: Deterministic Pre-Action Authorization" (2026) makes
//! the case the security surveys echo: a high-impact action must clear a
//! *deterministic* gate, not the model's judgment — because adaptive prompt
//! injection defeats judgment. This module classifies a tool call's blast radius
//! from the call itself (name + arguments) and applies a policy: `Allow`,
//! `Confirm` (needs human ok), or `Deny`.
//!
//! It's the enforcement half of the provenance story: `provenance` keeps
//! injected instructions labelled as data; `authorization` makes sure that even
//! if the model *is* subverted, the irreversible action still hits a wall. Pure
//! and deterministic, so the policy is auditable and unit-tested.
//!
//! Opt-in via `KORTEX_AUTHZ`.

use serde::Serialize;
use serde_json::Value;

/// Blast radius of a tool call, least to most dangerous.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Impact {
    /// Read-only: view a file, search, list. No state change.
    Read,
    /// Writes inside the workspace: edit/create a tracked file.
    Write,
    /// Runs a command / spawns a process.
    Execute,
    /// Leaves the machine: network fetch, push, publish.
    Network,
    /// Irreversible or out-of-workspace: delete, force-push, rm -rf, secrets.
    Destructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Decision {
    Allow,
    Confirm,
    Deny,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthDecision {
    pub impact: Impact,
    pub decision: Decision,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct AuthzConfig {
    pub enabled: bool,
    /// Impact at or above which a call needs human confirmation.
    pub confirm_at: Impact,
    /// Impact at or above which a call is denied outright (before confirm).
    /// `Destructive` by default only *confirms*; set to deny to hard-block.
    pub deny_at: Option<Impact>,
}

impl Default for AuthzConfig {
    fn default() -> Self {
        Self { enabled: false, confirm_at: Impact::Execute, deny_at: None }
    }
}

impl AuthzConfig {
    /// A provider-agnostic tool-safety gate — it changes what tool calls are
    /// allowed to run, not what a model says — so it ships on by default at
    /// its dispatcher call site regardless of backend. Deny-level blocking
    /// stays opt-in (`deny_at` defaults `None`); the default is Confirm-and-log
    /// on Execute+, never a hard block. `KORTEX_AUTHZ=0` disables entirely.
    pub fn from_env() -> Self {
        Self {
            enabled: super::env_flag::on("KORTEX_AUTHZ", true),
            confirm_at: match std::env::var("KORTEX_AUTHZ_CONFIRM_AT").ok().as_deref() {
                Some("read") => Impact::Read,
                Some("write") => Impact::Write,
                Some("network") => Impact::Network,
                Some("destructive") => Impact::Destructive,
                _ => Impact::Execute,
            },
            deny_at: match std::env::var("KORTEX_AUTHZ_DENY_AT").ok().as_deref() {
                Some("destructive") => Some(Impact::Destructive),
                Some("network") => Some(Impact::Network),
                Some("execute") => Some(Impact::Execute),
                _ => None,
            },
        }
    }
}

const READ_TOOLS: &[&str] = &[
    "read_file", "read", "view_file", "grep", "glob", "list_dir", "codebase_search",
    "search", "expand", "recall",
];
const WRITE_TOOLS: &[&str] = &["write_file", "write", "edit_file", "edit", "write_to_file", "apply_patch"];
const EXEC_TOOLS: &[&str] = &["bash", "run_terminal_cmd", "run_command", "shell", "exec"];
const NET_TOOLS: &[&str] = &["web_fetch", "web_search", "fetch", "http", "curl"];

/// Shell fragments that make an Execute call Destructive.
const DESTRUCTIVE_CMD: &[&str] = &[
    "rm -rf", "rm -r", "git push --force", "git push -f", "force-push",
    "drop table", "drop database", "mkfs", "dd if=", ":(){", "shutdown",
    "reboot", "> /dev/", "chmod -r 777", "git reset --hard", "del /f",
    "remove-item -recurse", "format ",
];
/// Paths / patterns that make a Write call Destructive (out-of-workspace,
/// secrets, VCS internals).
const SENSITIVE_PATH: &[&str] = &[
    "..", "~/", "/etc/", "/usr/", "c:\\windows", "id_rsa", ".ssh/", ".env",
    ".git/", "credentials", "secrets", "/system32",
];

fn arg_str(args: &Value, keys: &[&str]) -> String {
    for k in keys {
        if let Some(s) = args.get(*k).and_then(Value::as_str) {
            return s.to_string();
        }
    }
    // fall back to the whole argument blob (so we still catch a payload in an
    // unexpected field)
    args.to_string()
}

/// Classify a tool call's impact from its name and arguments.
pub fn classify(tool: &str, args: &Value) -> Impact {
    let name = tool.to_lowercase();

    if EXEC_TOOLS.iter().any(|t| name == *t) {
        let cmd = arg_str(args, &["command", "cmd", "script", "input"]).to_lowercase();
        if DESTRUCTIVE_CMD.iter().any(|d| cmd.contains(d)) {
            return Impact::Destructive;
        }
        return Impact::Execute;
    }
    if NET_TOOLS.iter().any(|t| name == *t) {
        return Impact::Network;
    }
    if WRITE_TOOLS.iter().any(|t| name == *t) {
        let path = arg_str(args, &["path", "file", "filename", "target"]).to_lowercase();
        if SENSITIVE_PATH.iter().any(|p| path.contains(p)) {
            return Impact::Destructive;
        }
        return Impact::Write;
    }
    if READ_TOOLS.iter().any(|t| name == *t) {
        // even a read of a sensitive path is worth flagging
        let path = arg_str(args, &["path", "file", "filename"]).to_lowercase();
        if SENSITIVE_PATH.iter().any(|p| path.contains(p) && *p != "..") {
            return Impact::Write; // read of secrets → treat as elevated
        }
        return Impact::Read;
    }
    // Unknown tool: be conservative — treat as Execute so it hits the gate.
    Impact::Execute
}

/// Apply the policy to a classified call.
pub fn authorize(tool: &str, args: &Value, cfg: &AuthzConfig) -> AuthDecision {
    let impact = classify(tool, args);
    let decision = if cfg.deny_at.is_some_and(|d| impact >= d) {
        Decision::Deny
    } else if impact >= cfg.confirm_at {
        Decision::Confirm
    } else {
        Decision::Allow
    };
    let reason = match decision {
        Decision::Allow => format!("{} is {:?} — below the confirm threshold", tool, impact),
        Decision::Confirm => format!("{} is {:?} — needs confirmation", tool, impact),
        Decision::Deny => format!("{} is {:?} — denied by policy", tool, impact),
    };
    AuthDecision { impact, decision, reason }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn cfg() -> AuthzConfig {
        AuthzConfig { enabled: true, ..Default::default() }
    }

    #[test]
    fn read_is_allowed() {
        let d = authorize("read_file", &json!({"path": "src/main.rs"}), &cfg());
        assert_eq!(d.impact, Impact::Read);
        assert_eq!(d.decision, Decision::Allow);
    }

    #[test]
    fn write_is_allowed_by_default_policy() {
        // default confirm_at = Execute, so an ordinary write is allowed
        let d = authorize("edit_file", &json!({"path": "src/lib.rs"}), &cfg());
        assert_eq!(d.impact, Impact::Write);
        assert_eq!(d.decision, Decision::Allow);
    }

    #[test]
    fn plain_command_needs_confirmation() {
        let d = authorize("bash", &json!({"command": "cargo build"}), &cfg());
        assert_eq!(d.impact, Impact::Execute);
        assert_eq!(d.decision, Decision::Confirm);
    }

    #[test]
    fn destructive_command_is_flagged() {
        let d = authorize("bash", &json!({"command": "rm -rf /"}), &cfg());
        assert_eq!(d.impact, Impact::Destructive);
        assert_eq!(d.decision, Decision::Confirm); // confirm by default
    }

    #[test]
    fn destructive_can_be_hard_denied() {
        let c = AuthzConfig { deny_at: Some(Impact::Destructive), ..cfg() };
        let d = authorize("bash", &json!({"command": "git push --force"}), &c);
        assert_eq!(d.decision, Decision::Deny);
    }

    #[test]
    fn write_to_sensitive_path_is_destructive() {
        let d = authorize("write_file", &json!({"path": "../../etc/passwd"}), &cfg());
        assert_eq!(d.impact, Impact::Destructive);
    }

    #[test]
    fn reading_secrets_is_elevated() {
        let d = authorize("read_file", &json!({"path": ".ssh/id_rsa"}), &cfg());
        assert!(d.impact >= Impact::Write, "reading a private key shouldn't be a plain read");
    }

    #[test]
    fn network_call_is_network_impact() {
        let d = authorize("web_fetch", &json!({"url": "https://x"}), &cfg());
        assert_eq!(d.impact, Impact::Network);
    }

    #[test]
    fn unknown_tool_is_conservative() {
        let d = authorize("mystery_tool", &json!({}), &cfg());
        assert_eq!(d.impact, Impact::Execute);
        assert_eq!(d.decision, Decision::Confirm);
    }

    #[test]
    fn tighter_policy_confirms_writes() {
        let c = AuthzConfig { confirm_at: Impact::Write, ..cfg() };
        let d = authorize("edit_file", &json!({"path": "src/lib.rs"}), &c);
        assert_eq!(d.decision, Decision::Confirm);
    }
}
