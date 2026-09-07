//! Output-verbosity steering — the decode-side lever.
//!
//! Every other compression in this crate cuts the *prompt* (fewer tokens in →
//! less prefill). This cuts the *response* (fewer tokens out → less decode),
//! which is the memory-bandwidth-bound half the `docs/kortex-decode-throughput`
//! work is about. Headroom reports ~31.7% output reduction from verbosity
//! steering + effort routing; this is the deterministic, local, no-extra-model
//! version of the same idea.
//!
//! Two parts, both idempotent and behind the `KORTEX_HARNESS` opt-in:
//!
//!   1. **A terse response contract** prepended to the system prompt: no
//!      preamble, no restating the question, no "here's what I'll do" narration,
//!      code/answer first. A small local model left to itself pads heavily; the
//!      directive is where most of the saved tokens come from.
//!   2. **Effort routing** — a light classifier reads the last user turn and
//!      sets a *ceiling* on `max_tokens` per class (terse / normal / deep). It
//!      only ever lowers an absent or runaway limit toward a sane ceiling; it
//!      never lowers a caller's smaller value and never caps a "deep" turn hard,
//!      so a genuine design/explain request is never truncated.

use serde_json::{json, Value};

/// Marker in the injected directive so a resend is a no-op.
const MARK: &str = "\u{2039}kortex:steer\u{203a}";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effort {
    /// Small ask — a lookup, a yes/no, a one-line edit. Answer, don't essay.
    Terse,
    /// Ordinary coding turn.
    Normal,
    /// Explain / design / architecture — reasoning room, barely capped.
    Deep,
}

impl Effort {
    /// Ceiling on output tokens for this class. A guardrail against a local
    /// model running away, not a hard target — the directive does the real work.
    fn ceiling(self) -> u64 {
        match self {
            Effort::Terse => 1024,
            Effort::Normal => 2048,
            Effort::Deep => 6144,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SteerConfig {
    pub enabled: bool,
    /// Inject the terse response contract into the system prompt.
    pub inject_directive: bool,
    /// Apply effort-routed `max_tokens` ceilings.
    pub route_effort: bool,
}

impl Default for SteerConfig {
    fn default() -> Self {
        Self { enabled: false, inject_directive: true, route_effort: true }
    }
}

impl SteerConfig {
    /// Same on-with-the-harness default as `tool_output::ToolOutputConfig`.
    pub fn from_env() -> Self {
        let harness_on = crate::domain::ai::env_flag::on("KORTEX_HARNESS", true);
        let sub_off = matches!(
            std::env::var("KORTEX_HARNESS_STEER").ok().as_deref(),
            Some("0") | Some("false") | Some("off")
        );
        Self { enabled: harness_on && !sub_off, ..Self::default() }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SteerReport {
    pub applied: bool,
    pub directive_injected: bool,
    pub effort: Option<&'static str>,
    /// `max_tokens` before / after (None = unset by caller).
    pub max_tokens_before: Option<u64>,
    pub max_tokens_after: Option<u64>,
}

/// The terse response contract. Deliberately short — every token here is paid on
/// every request, so it has to earn its place against the tokens it saves.
pub const DIRECTIVE: &str = "\
# Response style

Be terse and information-dense. Answer or act first; explain only if asked.
- No preamble, no restating the request, no \"Sure, I'll…\" or \"Here's what I'll do\".
- No summary of an action before or after doing it unless it changed something surprising.
- Code and diffs first; prose second and only when it adds information.
- One tool call per turn when a tool is needed; no narration around it.
- Stop when the answer is complete. Do not pad to sound thorough.";

/// Steer an OpenAI chat body in place: inject the directive and route effort.
pub fn steer_response(body: &mut Value, cfg: &SteerConfig) -> SteerReport {
    let mut report = SteerReport::default();
    if !cfg.enabled {
        return report;
    }
    let Some(obj) = body.as_object_mut() else {
        return report;
    };

    // Idempotency: our directive already present?
    let already = obj
        .get("messages")
        .and_then(Value::as_array)
        .map(|ms| {
            ms.iter().any(|m| {
                m.get("role").and_then(Value::as_str) == Some("system")
                    && m.get("content")
                        .and_then(Value::as_str)
                        .is_some_and(|c| c.contains(MARK))
            })
        })
        .unwrap_or(false);
    if already {
        report.applied = true;
        return report;
    }

    let effort = classify(obj);
    report.effort = Some(match effort {
        Effort::Terse => "terse",
        Effort::Normal => "normal",
        Effort::Deep => "deep",
    });

    if cfg.route_effort {
        let ceiling = effort.ceiling();
        let before = obj.get("max_tokens").and_then(Value::as_u64);
        report.max_tokens_before = before;
        // Only ever LOWER an absent or runaway limit to the ceiling; never raise
        // a caller's deliberate smaller budget.
        let after = match before {
            Some(v) if v <= ceiling => v,
            _ => ceiling,
        };
        obj.insert("max_tokens".into(), json!(after));
        report.max_tokens_after = Some(after);
    }

    if cfg.inject_directive {
        let text = format!("{MARK}\n{DIRECTIVE}");
        prepend_system_text(obj, &text);
        report.directive_injected = true;
    }

    report.applied = true;
    report
}

/// Classify the effort of the latest user turn from cheap lexical signals.
fn classify(obj: &serde_json::Map<String, Value>) -> Effort {
    let last_user = obj
        .get("messages")
        .and_then(Value::as_array)
        .and_then(|ms| {
            ms.iter()
                .rev()
                .find(|m| m.get("role").and_then(Value::as_str) == Some("user"))
        })
        .and_then(|m| m.get("content"))
        .map(content_text)
        .unwrap_or_default();

    let lower = last_user.to_lowercase();
    const DEEP: &[&str] = &[
        "explain", "why", "design", "architecture", "architect", "walk through",
        "walkthrough", "in detail", "detailed", "comprehensive", "trade-off",
        "tradeoff", "compare", "reasoning", "deep dive", "how does", "how do",
        "pros and cons", "step by step", "step-by-step",
    ];
    if DEEP.iter().any(|k| lower.contains(k)) {
        return Effort::Deep;
    }
    // Short, keyword-free asks are terse; everything else is normal.
    if last_user.trim().chars().count() <= 240 {
        Effort::Terse
    } else {
        Effort::Normal
    }
}

/// Flatten a message `content` (string or OpenAI content-part array) to text.
fn content_text(c: &Value) -> String {
    match c {
        Value::String(s) => s.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter_map(|p| p.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join(" "),
        _ => String::new(),
    }
}

/// Prepend text to the first system message, or insert one. (Local copy so this
/// module doesn't depend on `mod.rs`'s private helper.)
fn prepend_system_text(obj: &mut serde_json::Map<String, Value>, text: &str) {
    let Some(messages) = obj.get_mut("messages").and_then(Value::as_array_mut) else {
        return;
    };
    if let Some(sys) = messages
        .iter_mut()
        .find(|m| m.get("role").and_then(Value::as_str) == Some("system"))
    {
        if let Some(c) = sys.get("content").and_then(Value::as_str) {
            let merged = format!("{text}\n\n{c}");
            sys.as_object_mut()
                .unwrap()
                .insert("content".into(), Value::String(merged));
            return;
        }
    }
    messages.insert(0, json!({ "role": "system", "content": text }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn on() -> SteerConfig {
        SteerConfig { enabled: true, ..Default::default() }
    }

    fn body(user: &str, max_tokens: Option<u64>) -> Value {
        let mut b = json!({
            "model": "local",
            "messages": [
                {"role": "system", "content": "You are an agent."},
                {"role": "user", "content": user}
            ]
        });
        if let Some(m) = max_tokens {
            b["max_tokens"] = json!(m);
        }
        b
    }

    #[test]
    fn disabled_is_noop() {
        let mut b = body("fix the typo", None);
        let before = b.clone();
        let r = steer_response(&mut b, &SteerConfig::default());
        assert!(!r.applied);
        assert_eq!(b, before);
    }

    #[test]
    fn injects_directive_into_system() {
        let mut b = body("rename foo to bar", None);
        let r = steer_response(&mut b, &on());
        assert!(r.applied && r.directive_injected);
        let sys = b["messages"][0]["content"].as_str().unwrap();
        assert!(sys.contains("Response style"));
        assert!(sys.contains("You are an agent."), "original system text kept");
    }

    #[test]
    fn terse_short_ask() {
        let mut b = body("add a semicolon", None);
        let r = steer_response(&mut b, &on());
        assert_eq!(r.effort, Some("terse"));
        assert_eq!(r.max_tokens_after, Some(1024));
    }

    #[test]
    fn deep_ask_gets_room() {
        let mut b = body("explain why the borrow checker rejects this and the design trade-offs", None);
        let r = steer_response(&mut b, &on());
        assert_eq!(r.effort, Some("deep"));
        assert_eq!(r.max_tokens_after, Some(6144));
    }

    #[test]
    fn never_raises_a_smaller_caller_limit() {
        let mut b = body("add a semicolon", Some(256));
        let r = steer_response(&mut b, &on());
        assert_eq!(r.max_tokens_before, Some(256));
        assert_eq!(r.max_tokens_after, Some(256), "caller's smaller budget preserved");
    }

    #[test]
    fn lowers_a_runaway_limit() {
        let mut b = body("add a semicolon", Some(100_000));
        let r = steer_response(&mut b, &on());
        assert_eq!(r.max_tokens_after, Some(1024), "runaway capped to terse ceiling");
    }

    #[test]
    fn idempotent() {
        let mut b = body("do the thing", None);
        steer_response(&mut b, &on());
        let once = b.clone();
        let r2 = steer_response(&mut b, &on());
        assert!(r2.applied);
        assert!(!r2.directive_injected, "not injected twice");
        assert_eq!(b, once);
    }

    #[test]
    fn long_ask_is_normal() {
        let long = "please ".repeat(60); // >240 chars, no deep keywords
        let mut b = body(&long, None);
        let r = steer_response(&mut b, &on());
        assert_eq!(r.effort, Some("normal"));
        assert_eq!(r.max_tokens_after, Some(2048));
    }
}
