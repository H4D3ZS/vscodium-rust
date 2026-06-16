//! Claude Code / Claurst-aligned agent harness (reference: `claude-map/`, `claurst/src-rust/`).
//!
//! Ports behavioral guardrails from Claude Code's query loop and Claurst's `run_query_loop`:
//! verify-before-done, anti-stuck loops, tool-result budgeting, Qwen3 tool protocol.
//!
//! Fable-5 integration: critical thinking protocol that forces the model to reason
//! about its approach BEFORE each action, matching Composer 2's deep-reasoning loop.

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

/// Fable-5 critical thinking protocol — forces the model to reason before acting.
/// This is the core pattern from the Fable-5 traces: every tool call is preceded
/// by explicit reasoning about WHAT to do, WHY, and what could go wrong.
const FABLE5_CRITICAL_THINKING: &str = r###"
### CRITICAL THINKING PROTOCOL (Fable-5 / Composer 2 parity)
Before EVERY action (tool call), you MUST reason in a `<think>` block:

1. **ANALYZE**: What is the current state? What do I know from the last tool result?
2. **PLAN**: What is the single best next action? Why this tool and not another?
3. **ANTICIPATE**: What could go wrong? What's the fallback if this fails?
4. **EXECUTE**: Call the tool with precise arguments.

Example flow:
<think>
I see the user wants to add authentication. The codebase uses Express with no auth middleware.
Current state: server/index.js has 3 routes, no session management.
Plan: Add passport.js + express-session. Start with npm install, then create auth middleware.
Risk: package.json might have conflicting deps. Fallback: manual session impl.
Decision: run_command("npm install passport express-session") first.
</think>
<tool_call>{"name":"run_command","arguments":{"command":"npm install passport express-session"}}</tool_call>

This thinking is VISIBLE to the user — it shows your reasoning process.
Do NOT skip thinking blocks. Do NOT think silently and just call tools.
The thinking IS the value — it shows HOW you solve problems, not just WHAT you do.
"###;

/// Phase-specific thinking prompts that adapt the critical thinking to the current context.
pub fn phase_thinking_prompt(iteration: usize, has_failures: bool) -> String {
    if iteration == 0 {
        return "### [ITERATION 0 — DEEP REASONING]\n\
            Before your first action, think deeply:\n\
            - What exactly is the user asking for?\n\
            - What files/patterns are relevant? (use BRAIN map, don't grep)\n\
            - What's the implementation order? (dependencies first)\n\
            - What verification command will you run at the end?\n\
            Output your plan between `<TASK_PLAN>` and `</TASK_PLAN>`, then begin executing."
            .to_string();
    }

    if has_failures {
        return format!(
            "### [ITERATION {} — RECOVERY MODE]\n\
             Previous action FAILED. Before retrying:\n\
             - What went wrong? (read the error carefully)\n\
             - Is the same approach likely to work again? (if tried 2x already, NO)\n\
             - What's a DIFFERENT approach?\n\
             Think in `<think>` then execute with a changed strategy.",
            iteration
        );
    }

    if iteration % 5 == 0 {
        return format!(
            "### [ITERATION {} — REFLECTION]\n\
             Step back and assess:\n\
             - What's done vs what remains?\n\
             - Am I on track or drifting?\n\
             - Is there a faster path I'm missing?\n\
             Then continue executing.",
            iteration
        );
    }

    String::new()
}

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

/// Terse, single-format directive for small/local models (≤7B, e.g. gemma-4).
///
/// Small models obey the FIRST strong instruction they latch onto. The full
/// FABLE5 "write deep <think> reasoning before every action" protocol makes them
/// produce a prose PLAN and never reach the tool call. They also can't reconcile
/// two tool-call formats (```json vs <tool_call> XML). So small models get ONE
/// format, an explicit anti-plan rule, and nothing that rewards prose.
const SMALL_MODEL_DIRECTIVE: &str = r###"
### ACT — DO NOT PLAN (small-model contract)
Your reply MUST be EITHER a single tool call OR the token MISSION_ACCOMPLISHED.
- Do NOT write numbered plans, "I will…", "Here's my approach", or any prose-only answer.
- Do NOT write <think> blocks. Do NOT explain before acting. Just emit the tool call.
- Use EXACTLY ONE format — a fenced JSON block, nothing before or after it:
```json
{"name": "view_file", "arguments": {"path": "src/main.rs"}}
```
- One tool call per reply. After the result comes back, emit the NEXT tool call.
- If you catch yourself writing a plan, STOP and emit the first step as a tool call instead.
"###;

/// System prompt addon keyed to model family (Claude Code behavioral contract).
/// `is_small` selects the terse act-now contract for weak local models; large
/// models get the full reasoning/workflow protocol.
pub fn harness_system_addon(model: &str, is_small: bool) -> String {
    if is_small {
        // Small models: terse + single-format. NO FABLE5 (it induces plan-prose),
        // NO conflicting XML protocol. Keep only the anti-stuck guard + act-now.
        let mut out = String::from(ANTI_STUCK);
        out.push_str(SMALL_MODEL_DIRECTIVE);
        return out;
    }
    let mut out = String::from(VERIFY_BEFORE_DONE);
    out.push_str(ANTI_STUCK);
    out.push_str(CLAUDE_CODE_WORKFLOW);
    out.push_str(FABLE5_CRITICAL_THINKING);
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

/// Verify gate — checks if recent tool results contain build/test failures.
/// Returns (has_failures, failure_summary) so the loop can inject recovery prompts.
pub fn check_verification_failure(messages: &[ChatMessage]) -> (bool, String) {
    let recent_tools: Vec<&str> = messages
        .iter()
        .rev()
        .take(6)
        .filter(|m| m.role == "tool")
        .filter_map(|m| m.content.as_ref().map(|c| c.as_str()))
        .collect();

    let failure_patterns = [
        "error[E", "error:", "FAILED", "failed to", "Error:", "panic:",
        "thread.*panicked", "exit status 1", "exit code 1",
        "typecheck failed", "cargo check failed", "npm ERR!",
        "Compilation failed", "BUILD FAILURE",
    ];

    for content in &recent_tools {
        for pattern in &failure_patterns {
            if content.contains(pattern) {
                // Extract the first relevant error line
                let summary = content
                    .lines()
                    .find(|l| l.contains(pattern) || l.contains("error"))
                    .unwrap_or("Unknown error")
                    .chars()
                    .take(200)
                    .collect::<String>();
                return (true, summary);
            }
        }
    }

    (false, String::new())
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

    #[test]
    fn detect_build_failure() {
        let messages = vec![
            ChatMessage {
                role: "tool".to_string(),
                content: Some(MessageContent::Text(
                    "error[E0432]: unresolved import `foo`\n  --> src/main.rs:3:5".to_string(),
                )),
                ..Default::default()
            },
        ];
        let (has_failure, summary) = check_verification_failure(&messages);
        assert!(has_failure);
        assert!(summary.contains("error[E0432]"));
    }

    #[test]
    fn no_false_positive_on_success() {
        let messages = vec![
            ChatMessage {
                role: "tool".to_string(),
                content: Some(MessageContent::Text(
                    "Finished dev [unoptimized + debuginfo] target(s) in 0.42s".to_string(),
                )),
                ..Default::default()
            },
        ];
        let (has_failure, _) = check_verification_failure(&messages);
        assert!(!has_failure);
    }

    #[test]
    fn phase_thinking_prompt_iter0() {
        let prompt = phase_thinking_prompt(0, false);
        assert!(prompt.contains("DEEP REASONING"));
        assert!(prompt.contains("TASK_PLAN"));
    }

    #[test]
    fn phase_thinking_prompt_recovery() {
        let prompt = phase_thinking_prompt(3, true);
        assert!(prompt.contains("RECOVERY MODE"));
    }
}
