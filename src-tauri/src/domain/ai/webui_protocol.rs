//! Shared WebUI tool-call protocol.
//!
//! A single source of truth for (a) the system prompt that teaches a free web-chat
//! model (Claude/GPT/Qwen/DeepSeek/Gemini) to drive the IDE's real tools, and (b) the
//! parser that extracts emitted tool calls from the model's reply text.
//!
//! Consumed by:
//! - `infrastructure::webui_mcp_bridge` — the ZeroScript-style browser-extension bridge.
//! - `application::autonomous_supervisor` — the 24/7 unattended loop.
//! - `application::commands::auth::webui_agent_run` — the legacy in-app webview path.

use serde_json::Value;

/// System prompt prepended to the first message of a WebUI agent session.
///
/// The tool names here MUST match canonical names accepted by
/// `domain::tools::AiTools::call_tool` so the bridge can execute them directly.
pub const WEBUI_TOOL_PROTOCOL: &str = r#"You are an autonomous coding agent driving a REAL IDE on the user's machine through a LIVE tool bridge. This is NOT a sandbox, NOT hypothetical, and NOT a roleplay: every tool call you emit is actually executed on their filesystem and the real output is returned to you in the next message.

CRITICAL — read this twice:
- You DO have file and shell access, through the tools below (not directly).
- NEVER say "I can't access your files", "I don't have a terminal/connection", "I operate in a sandbox", or ask the user to paste file contents. All of that is FALSE here and causes the task to FAIL.
- If you are unsure what's on disk, the correct move is to EMIT A TOOL CALL and read it — not to ask the user.

## How to call a tool
Reply with ONLY one or more fenced JSON blocks. No prose, no greeting, no explanation before the first block:
```json
{"tool": "list_files", "args": {"path": "src"}}
```
I execute each call and reply with the real results. You then emit the next call(s).

## Your VERY FIRST reply
Must be a tool call (usually `list_files` or `view_file`). Do not greet, do not ask permission, do not explain your plan — just emit the JSON and act.

## Tools (name → args)
- view_file {path}                              — read a file
- list_files {path}                             — list a directory
- grep {pattern, path?}                         — search code
- write_to_file {path, content}                 — create/overwrite a file
- search_replace_edit {path, search, replace}   — surgical edit
- run_command {command}                         — run a shell command
- dev_cargo_diagnostics {}                       — cargo check (Rust)
- deep_security_audit {path}                    — structured CWE security audit

## Rules
- One reply = the next tool call(s). Keep going until the whole task is done.
- After any code edit, verify (dev_cargo_diagnostics for Rust, or run_command for typecheck/tests).
- When the task is fully complete AND verified, write the single token TASK_COMPLETE on its own line."#;

/// Completion sentinel — the model writes this token alone when finished.
pub const TASK_COMPLETE_TOKEN: &str = "TASK_COMPLETE";

/// Extract `(tool_name, args)` pairs from fenced ```json {"tool","args"}``` blocks.
///
/// Tolerant of language tags after the opening fence (```json, ```JSON, bare ```) and
/// of prose between blocks. Blocks that don't parse or lack `tool`/`args` are skipped.
pub fn parse_webui_tool_calls(text: &str) -> Vec<(String, Value)> {
    let mut calls = Vec::new();
    let mut search = text;
    while let Some(start) = search.find("```") {
        let after = &search[start + 3..];
        let nl = after.find('\n').unwrap_or(after.len());
        let rest = if nl < after.len() { &after[nl + 1..] } else { "" };
        if let Some(end) = rest.find("```") {
            let block = &rest[..end];
            if let Ok(v) = serde_json::from_str::<Value>(block.trim()) {
                if let (Some(name), Some(args)) =
                    (v.get("tool").and_then(|x| x.as_str()), v.get("args"))
                {
                    calls.push((name.to_string(), args.clone()));
                }
            }
            search = &rest[end + 3..];
        } else {
            break;
        }
    }
    calls
}

/// True when the model signalled completion (token on its own, not inside a tool block).
pub fn is_task_complete(text: &str) -> bool {
    text.to_uppercase().contains(TASK_COMPLETE_TOKEN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_call() {
        let txt = "Sure.\n```json\n{\"tool\":\"view_file\",\"args\":{\"path\":\"a.rs\"}}\n```\nDone.";
        let calls = parse_webui_tool_calls(txt);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "view_file");
        assert_eq!(calls[0].1["path"], "a.rs");
    }

    #[test]
    fn parses_multiple_calls() {
        let txt = "```json\n{\"tool\":\"list_files\",\"args\":{\"path\":\".\"}}\n```\nthen\n```json\n{\"tool\":\"grep\",\"args\":{\"pattern\":\"fn main\"}}\n```";
        let calls = parse_webui_tool_calls(txt);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[1].0, "grep");
    }

    #[test]
    fn skips_malformed_block() {
        let txt = "```json\n{not valid json}\n```";
        assert!(parse_webui_tool_calls(txt).is_empty());
    }

    #[test]
    fn detects_completion() {
        assert!(is_task_complete("all good\nTASK_COMPLETE"));
        assert!(!is_task_complete("still working"));
    }
}
