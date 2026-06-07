//! Claude Code / Claurst-aligned agent harness (reference: `claude-map/`, `claurst/src-rust/`).
//!
//! Ports behavioral guardrails from Claude Code's query loop and Claurst's `run_query_loop`:
//! verify-before-done, anti-stuck loops, tool-result budgeting, Qwen3 tool protocol.

use crate::ai_engine::{ChatMessage, MessageContent};

pub const DEFAULT_TOOL_RESULT_CHAR_BUDGET: usize = 50_000;

const VERIFY_BEFORE_DONE: &str = r###"
### VERIFY BEFORE DONE (Claude Code parity)
Before reporting a task complete, verify it actually works: run tests, execute the script, check output.
- After editing code: call `verify_implementation`, `dev_cargo_diagnostics`, or `run_command` with the project check command.
- Do NOT emit `MISSION_ACCOMPLISHED` while builds or typechecks are failing.
- If you cannot verify (no test exists), say so explicitly rather than claiming success.
"###;

const ANTI_STUCK: &str = r###"
### ANTI-LOOP (Claude Code / Claurst parity)
- Do NOT repeat the same tool with the same arguments more than twice.
- If root `list_directory` / `list_files` failed or returned nothing useful, use `search_codebase`, `semantic_search`, or targeted `grep` — do NOT loop on directory listing.
- If an approach failed twice, change strategy before trying again.
"###;

const QWEN3_AGENT_PROTOCOL: &str = r###"
### QWEN3 AGENT PROTOCOL
- Prefer native tool calls. If the model API does not invoke tools, emit XML blocks:
  `<tool_call>{"name":"view_file","arguments":{"path":"src/main.rs"}}</tool_call>`
- Keep chain-of-thought in thinking tags; tool JSON must be valid and separate from prose.
- Read with `view_file` before editing. Use `grep` / `search_codebase` / `semantic_search` instead of blind repo walks.
- One concrete action per turn when possible; wait for tool results before the next edit.
"###;

const CLAUDE_CODE_WORKFLOW: &str = r###"
### CLAUDE CODE WORKFLOW
1. Understand the request and locate relevant files (search, not root listing).
2. Read targets, plan minimally, then edit with surgical tools (`search_replace_edit`, `patch_file_content`, `write_to_file`).
3. Run verification (`run_command`, `verify_implementation`, `dev_cargo_diagnostics`).
4. Only then declare completion with `MISSION_ACCOMPLISHED` on its own line.
"###;

pub fn is_qwen3_family(model: &str) -> bool {
    let m = model.to_ascii_lowercase();
    m.contains("qwen3") || m.contains("qwen-3") || m.contains("qwen3.5") || m.contains("qwen3.6")
}

pub fn is_reasoning_tag_model(model: &str) -> bool {
    let m = model.to_ascii_lowercase();
    is_qwen3_family(model)
        || m.contains("qwq")
        || m.contains("deepseek-r1")
        || m.contains("kimi")
        || m.contains("minimax")
}

/// System prompt addon keyed to model family (Claude Code behavioral contract).
pub fn harness_system_addon(model: &str) -> String {
    let mut out = String::from(VERIFY_BEFORE_DONE);
    out.push_str(ANTI_STUCK);
    out.push_str(CLAUDE_CODE_WORKFLOW);
    if is_qwen3_family(model) {
        out.push_str(QWEN3_AGENT_PROTOCOL);
    } else if is_reasoning_tag_model(model) {
        out.push_str(
            "\n### REASONING MODEL\nUse thinking for analysis; emit tool calls or completion tokens outside thinking blocks.\n",
        );
    }
    out
}

/// Claurst-style todo nudge after turn 2 in long agent runs.
pub fn todo_nudge(iteration: usize) -> Option<&'static str> {
    if iteration >= 2 {
        Some(
            "### [TASK PROGRESS]\nComplete remaining plan steps before declaring done. \
             Run verification before MISSION_ACCOMPLISHED.\n",
        )
    } else {
        None
    }
}

/// Three identical consecutive tool fingerprints → stuck (claude-map death-spiral guard).
pub fn detect_stuck_loop(recent_tool_calls: &[String]) -> bool {
    recent_tool_calls.len() >= 3 && {
        let n = recent_tool_calls.len();
        recent_tool_calls[n - 1] == recent_tool_calls[n - 2]
            && recent_tool_calls[n - 2] == recent_tool_calls[n - 3]
    }
}

/// Truncate oldest tool-role messages when total tool output exceeds budget (claurst `apply_tool_result_budget`).
pub fn apply_tool_result_budget(messages: &mut [ChatMessage], max_chars: usize) {
    let mut total: usize = messages
        .iter()
        .filter(|m| m.role == "tool")
        .map(|m| m.content.as_ref().map(|c| c.as_str().len()).unwrap_or(0))
        .sum();
    if total <= max_chars {
        return;
    }
    for msg in messages.iter_mut() {
        if msg.role != "tool" {
            continue;
        }
        if total <= max_chars {
            break;
        }
        let len = msg.content.as_ref().map(|c| c.as_str().len()).unwrap_or(0);
        if len <= 512 {
            continue;
        }
        let keep = 512.min(len);
        let truncated = msg.content.as_ref().map(|c| c.as_str()).unwrap_or("");
        let snippet = &truncated[..truncated.floor_char_boundary(keep)];
        msg.content = Some(MessageContent::Text(format!(
            "{}\n\n[… truncated {} chars — call view_file for full content …]",
            snippet,
            len.saturating_sub(keep)
        )));
        total = total.saturating_sub(len.saturating_sub(keep + 80));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_stuck_triple() {
        let recent = vec![
            "grep::{\"pattern\":\"x\"}".to_string(),
            "grep::{\"pattern\":\"x\"}".to_string(),
            "grep::{\"pattern\":\"x\"}".to_string(),
        ];
        assert!(detect_stuck_loop(&recent));
    }

    #[test]
    fn qwen36_detected() {
        assert!(is_qwen3_family("cyberifrit/qwen3.6:35b"));
    }
}
