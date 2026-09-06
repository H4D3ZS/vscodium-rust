//! One place to answer "does this prompt fit the model's window, and if not,
//! what do we drop?" — so the agent loop stops paying for context it can't use.
//!
//! Before this, budgeting was ~six hard-coded fractions scattered through
//! `autonomous.rs` (`ctx * 35 / 100`, `.min(8_000)`, `.min(12_000)` …) and a
//! crude backwards char-count trim. This module gives the loop a single
//! `ContextBudget` + a priority-aware `fit()` that keeps the system prompt and
//! the current turn, sheds the *oldest* tool results first, and reports exactly
//! what it did (so a UI can show where the tokens went).

use serde::Serialize;

use super::engine::types::ChatMessage;

/// Rough token estimate. Code + JSON tokenise denser than prose, so this runs a
/// hair under 4 chars/token and rounds up. Good to ±10%, which is what the
/// `headroom` slack in [`ContextBudget`] is for.
pub fn estimate_tokens(text: &str) -> usize {
    (text.len() * 10 + 35) / 36
}

/// Wire cost of one message: content + tool-call JSON + per-message framing.
pub fn message_tokens(m: &ChatMessage) -> usize {
    let mut t = 4; // role + delimiters
    if let Some(c) = &m.content {
        t += estimate_tokens(&c.to_text());
    }
    if let Some(calls) = &m.tool_calls {
        for c in calls {
            t += estimate_tokens(&c.function.name) + estimate_tokens(&c.function.arguments) + 8;
        }
    }
    t
}

pub fn messages_tokens(ms: &[ChatMessage]) -> usize {
    ms.iter().map(message_tokens).sum()
}

/// The model's usable window and how much of it we refuse to fill with prompt.
#[derive(Debug, Clone, Copy)]
pub struct ContextBudget {
    /// Real context window in tokens.
    pub window: usize,
    /// Held back for the model's reply.
    pub output_reserve: usize,
    /// Extra slack against estimate error, so a near-miss doesn't 400 the server.
    pub headroom: usize,
}

impl ContextBudget {
    /// `probed` = `n_ctx` from the server's `/props` when known (0 otherwise, in
    /// which case we fall back to a by-name guess).
    pub fn resolve(model: &str, probed: usize, output_reserve: usize) -> Self {
        let window = if probed >= 2048 { probed } else { window_for_model(model) };
        Self {
            window,
            output_reserve: output_reserve.min(window / 2),
            headroom: (window / 32).max(256),
        }
    }

    /// Max tokens the assembled prompt (messages + tools) may occupy.
    pub fn prompt_ceiling(&self) -> usize {
        self.window
            .saturating_sub(self.output_reserve + self.headroom)
    }
}

fn window_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();
    if m.contains("claude") || m.contains("gpt-4") || m.contains("gpt-5") || m.contains("o1") || m.contains("o3") {
        200_000
    } else if m.contains("gemini") {
        1_000_000
    } else if m.contains("qwen3") || m.contains("qwen2.5") || m.contains("qwen-3") {
        32_768
    } else if m.contains("deepseek") {
        65_536
    } else if m.contains("gemma") || m.contains("llama") || m.contains("mistral") {
        8_192
    } else {
        8_192
    }
}

/// What [`fit`] did to the message list.
#[derive(Debug, Clone, Serialize)]
pub struct FitReport {
    pub window: usize,
    pub ceiling: usize,
    pub tools_tokens: usize,
    pub before_tokens: usize,
    pub after_tokens: usize,
    pub dropped_messages: usize,
    /// Whether the result is now under the ceiling. `false` means even the
    /// system prompt + current turn overflow — the caller should compact
    /// harder (summarise the system prompt, split the turn).
    pub fit: bool,
}

/// Priority-aware trim. Invariants:
///   - keep `messages[0]` when it's the `system` message,
///   - keep the final message (the current user turn),
///   - drop from the *oldest* end inward, `tool` results first, then history.
///
/// Returns a [`FitReport`] either way; a no-op when the prompt already fits.
pub fn fit(messages: &mut Vec<ChatMessage>, tools_tokens: usize, budget: ContextBudget) -> FitReport {
    let ceiling = budget.prompt_ceiling();
    let before = messages_tokens(messages) + tools_tokens;

    let mut report = FitReport {
        window: budget.window,
        ceiling,
        tools_tokens,
        before_tokens: before,
        after_tokens: before,
        dropped_messages: 0,
        fit: before <= ceiling,
    };
    if before <= ceiling || messages.len() <= 2 {
        return report;
    }

    let has_system = messages.first().map(|m| m.role == "system").unwrap_or(false);
    let protected_head = usize::from(has_system);

    // Pass 1: shed the oldest `tool` results.
    let mut i = protected_head;
    while messages_tokens(messages) + tools_tokens > ceiling
        && messages.len() > 2
        && i < messages.len() - 1
    {
        if messages[i].role == "tool" {
            messages.remove(i);
            report.dropped_messages += 1;
        } else {
            i += 1;
        }
    }

    // Pass 2: still over → shed the oldest non-system, non-final message.
    while messages_tokens(messages) + tools_tokens > ceiling && messages.len() > 2 {
        let idx = protected_head;
        if idx >= messages.len() - 1 {
            break;
        }
        messages.remove(idx);
        report.dropped_messages += 1;
    }

    report.after_tokens = messages_tokens(messages) + tools_tokens;
    report.fit = report.after_tokens <= ceiling;
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::engine::types::MessageContent;

    fn msg(role: &str, body: &str) -> ChatMessage {
        ChatMessage {
            role: role.into(),
            content: Some(MessageContent::Text(body.into())),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        }
    }

    #[test]
    fn estimate_is_in_the_right_ballpark() {
        // ~360 chars ≈ 100 tokens.
        assert!((90..=115).contains(&estimate_tokens(&"x".repeat(360))));
    }

    #[test]
    fn resolve_reserves_and_leaves_headroom() {
        let b = ContextBudget::resolve("qwen3-8b", 0, 4096);
        assert_eq!(b.window, 32_768);
        assert!(b.prompt_ceiling() < b.window - 4096);
    }

    #[test]
    fn no_op_when_it_already_fits() {
        let mut ms = vec![msg("system", "sys"), msg("user", "hi")];
        let r = fit(&mut ms, 0, ContextBudget::resolve("m", 8192, 1024));
        assert_eq!(ms.len(), 2);
        assert_eq!(r.dropped_messages, 0);
        assert!(r.fit);
    }

    #[test]
    fn sheds_oldest_tool_results_first_keeps_system_and_last_turn() {
        let big = "y".repeat(30_000); // ~8.3k tokens each — dropping one won't be enough
        let mut ms = vec![
            msg("system", "you are helpful"),
            msg("user", "q1"),
            msg("assistant", "calling tool"),
            msg("tool", &big),
            msg("tool", &big),
            msg("assistant", "answer 1"),
            msg("user", "q2 — the current turn"),
        ];
        let budget = ContextBudget { window: 8192, output_reserve: 1024, headroom: 256 };
        let r = fit(&mut ms, 0, budget);

        assert_eq!(ms.first().unwrap().role, "system");
        assert_eq!(ms.last().unwrap().content.as_ref().unwrap().to_text(), "q2 — the current turn");
        assert!(r.dropped_messages >= 2, "should drop the two fat tool results");
        // the fat tool bodies must be gone
        assert!(!ms.iter().any(|m| m.role == "tool" && m.content.as_ref().unwrap().to_text().len() > 1000));
    }

    #[test]
    fn reports_when_even_the_minimum_overflows() {
        let huge = "z".repeat(200_000);
        let mut ms = vec![msg("system", &huge), msg("user", &huge)];
        let r = fit(&mut ms, 0, ContextBudget { window: 8192, output_reserve: 1024, headroom: 256 });
        assert!(!r.fit);
        assert_eq!(ms.len(), 2); // can't drop below system + current turn
    }
}
