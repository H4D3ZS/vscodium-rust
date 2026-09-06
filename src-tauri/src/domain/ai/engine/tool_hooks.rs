//! Per-tool-call hooks — the piece of Claude Code parity the lifecycle-hook
//! systems (Kiro, antigravity) didn't cover.
//!
//! Config is read from a Claude-Code-compatible `settings.json`:
//!
//! ```json
//! {
//!   "hooks": {
//!     "PreToolUse":  [ { "matcher": "bash|shell", "hooks": [ { "type": "command", "command": "…", "timeout": 10 } ] } ],
//!     "PostToolUse": [ { "matcher": "",           "hooks": [ { "type": "command", "command": "…" } ] } ]
//!   }
//! }
//! ```
//!
//! Search order: `<root>/.claude/settings.local.json`, then `<root>/.claude/settings.json`.
//! `matcher` is a regex over the tool name; `""` / absent matches every tool.
//!
//! Each hook receives a JSON object on **stdin**:
//! `{ "hook_event_name", "tool_name", "tool_input", "tool_result"? , "cwd" }`.
//! A **PreToolUse** hook blocks the call by exiting `2`, or by printing JSON with
//! `{"decision":"block"}` / `{"permissionDecision":"deny"}` (with an optional
//! `reason` / `permissionDecisionReason`). Anything else = allow.
//! **PostToolUse** hooks are fire-and-forget.

use serde_json::{json, Value};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

/// Result of the PreToolUse gate.
#[derive(Debug, Clone)]
pub enum PreHookOutcome {
    Allow,
    Block(String),
}

const DEFAULT_TIMEOUT_SECS: u64 = 15;

fn load_settings(root: &Path) -> Option<Value> {
    for name in ["settings.local.json", "settings.json"] {
        let p = root.join(".claude").join(name);
        if let Ok(text) = std::fs::read_to_string(&p) {
            if let Ok(v) = serde_json::from_str::<Value>(&text) {
                // shallow-merge would be nicer, but first hit wins is fine and
                // matches how most people use settings.local.json (an override).
                return Some(v);
            }
        }
    }
    None
}

fn matching_commands(settings: &Value, event: &str, tool_name: &str) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    let Some(groups) = settings
        .get("hooks")
        .and_then(|h| h.get(event))
        .and_then(|e| e.as_array())
    else {
        return out;
    };
    for group in groups {
        let matcher = group.get("matcher").and_then(Value::as_str).unwrap_or("");
        let matches = matcher.is_empty()
            || regex_lite_match(matcher, tool_name)
            || matcher == tool_name;
        if !matches {
            continue;
        }
        let Some(hooks) = group.get("hooks").and_then(|h| h.as_array()) else {
            continue;
        };
        for h in hooks {
            if h.get("type").and_then(Value::as_str) != Some("command") {
                continue;
            }
            if let Some(cmd) = h.get("command").and_then(Value::as_str) {
                let timeout = h
                    .get("timeout")
                    .and_then(Value::as_u64)
                    .unwrap_or(DEFAULT_TIMEOUT_SECS);
                out.push((cmd.to_string(), timeout));
            }
        }
    }
    out
}

/// Minimal alternation/substring matcher so we don't pull a regex crate in just
/// for this. Supports `a|b|c` and plain substrings; falls back to `contains`.
fn regex_lite_match(pattern: &str, name: &str) -> bool {
    pattern
        .split('|')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .any(|p| name == p || name.contains(p))
}

async fn run_one(cmd: &str, timeout: u64, payload: &Value, cwd: &Path) -> (Option<i32>, String) {
    #[cfg(windows)]
    let mut command = {
        let mut c = tokio::process::Command::new("cmd");
        c.arg("/C").arg(cmd);
        c
    };
    #[cfg(not(windows))]
    let mut command = {
        let mut c = tokio::process::Command::new("sh");
        c.arg("-c").arg(cmd);
        c
    };
    command
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let Ok(mut child) = command.spawn() else {
        return (None, String::new());
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(payload.to_string().as_bytes()).await;
        let _ = stdin.shutdown().await;
    }
    match tokio::time::timeout(Duration::from_secs(timeout), child.wait_with_output()).await {
        Ok(Ok(out)) => (
            out.status.code(),
            String::from_utf8_lossy(&out.stdout).into_owned(),
        ),
        _ => (None, String::new()), // timeout or spawn error → treat as no-op
    }
}

/// PreToolUse gate. Cheap no-op when there is no `.claude/settings*.json` or no
/// matching hook.
pub async fn run_pre_tool_hooks(root: &Path, tool_name: &str, tool_input: &Value) -> PreHookOutcome {
    let Some(settings) = load_settings(root) else {
        return PreHookOutcome::Allow;
    };
    let cmds = matching_commands(&settings, "PreToolUse", tool_name);
    if cmds.is_empty() {
        return PreHookOutcome::Allow;
    }
    let payload = json!({
        "hook_event_name": "PreToolUse",
        "tool_name": tool_name,
        "tool_input": tool_input,
        "cwd": root.to_string_lossy(),
    });
    for (cmd, timeout) in cmds {
        let (code, stdout) = run_one(&cmd, timeout, &payload, root).await;
        if code == Some(2) {
            return PreHookOutcome::Block(format!("blocked by PreToolUse hook: {cmd}"));
        }
        if let Ok(v) = serde_json::from_str::<Value>(stdout.trim()) {
            let decision = v.get("decision").and_then(Value::as_str);
            let perm = v.get("permissionDecision").and_then(Value::as_str);
            if decision == Some("block") || perm == Some("deny") {
                let reason = v
                    .get("reason")
                    .or_else(|| v.get("permissionDecisionReason"))
                    .and_then(Value::as_str)
                    .unwrap_or("blocked by PreToolUse hook")
                    .to_string();
                return PreHookOutcome::Block(reason);
            }
        }
    }
    PreHookOutcome::Allow
}

/// PostToolUse — fire-and-forget. Never blocks, never fails the turn.
pub async fn run_post_tool_hooks(
    root: &Path,
    tool_name: &str,
    tool_input: &Value,
    tool_result: Option<&Value>,
) {
    let Some(settings) = load_settings(root) else { return };
    let cmds = matching_commands(&settings, "PostToolUse", tool_name);
    if cmds.is_empty() {
        return;
    }
    let payload = json!({
        "hook_event_name": "PostToolUse",
        "tool_name": tool_name,
        "tool_input": tool_input,
        "tool_result": tool_result.cloned().unwrap_or(Value::Null),
        "cwd": root.to_string_lossy(),
    });
    for (cmd, timeout) in cmds {
        let _ = run_one(&cmd, timeout, &payload, root).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matcher_alternation() {
        assert!(regex_lite_match("bash|shell|run_terminal_cmd", "shell"));
        assert!(regex_lite_match("write|edit", "write_file"));
        assert!(!regex_lite_match("bash|shell", "read_file"));
    }

    #[test]
    fn no_settings_is_allow() {
        let dir = std::env::temp_dir().join("kortex-hooks-none");
        let _ = std::fs::create_dir_all(&dir);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let out = rt.block_on(run_pre_tool_hooks(&dir, "bash", &json!({"command": "ls"})));
        assert!(matches!(out, PreHookOutcome::Allow));
    }

    #[test]
    fn parses_and_selects_commands() {
        let s = json!({
            "hooks": {
                "PreToolUse": [
                    { "matcher": "bash", "hooks": [ { "type": "command", "command": "echo hi", "timeout": 5 } ] },
                    { "matcher": "",     "hooks": [ { "type": "command", "command": "echo all" } ] }
                ]
            }
        });
        let got = matching_commands(&s, "PreToolUse", "bash");
        assert_eq!(got.len(), 2);
        assert_eq!(got[0], ("echo hi".to_string(), 5));
        assert_eq!(got[1].1, DEFAULT_TIMEOUT_SECS);
        assert!(matching_commands(&s, "PreToolUse", "read_file").len() == 1); // only the "" matcher
    }
}
