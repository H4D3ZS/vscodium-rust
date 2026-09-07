//! Difficulty-adaptive **cascade router** — compute proportional to difficulty.
//!
//! The root inefficiency of LLM inference is spending the *same* compute on
//! every token regardless of how hard it is: a 27B reasoner streams its full
//! weights to emit "the" exactly as it does to close a subtle bug. Most tokens,
//! and most whole turns, are easy — a 4B gets them right with high confidence.
//!
//! The cascade makes the cheap **Operator** (small model on Lemonade) answer by
//! default and escalates to the expensive **Reasoner** (big model on ROCmFPX)
//! *only when the Operator is uncertain*. On the easy majority the big model
//! never runs — a real compute cut, distinct from speculative decoding (which is
//! lossless but still reads the big model's weights every verify step). The
//! trade is a *tunable, measured* quality knob: the confidence gate decides how
//! eagerly to escalate.
//!
//! Two decisions:
//!   1. **Pre-classify** the request → start on the Operator or jump straight to
//!      the Reasoner for obviously-hard asks (skip a wasted small-model call).
//!   2. **Post-check** the Operator's answer → accept it, or escalate. The
//!      signal is the response's token logprobs (mean + worst-token), with a
//!      text-based fallback (hedging/refusal phrases) when a backend doesn't
//!      return logprobs.
//!
//! The policy is pure and unit-tested; `run_cascade` is the async runner that
//! executes it over two OpenAI-compatible endpoints. Opt-in via `KORTEX_CASCADE`.

use serde::Serialize;
use serde_json::{json, Value};

/// Which model tier handles (or handled) a turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Operator,
    Reasoner,
}

#[derive(Debug, Clone)]
pub struct CascadeConfig {
    pub enabled: bool,
    /// Escalate when the answer's **mean** token probability is below this.
    /// 0.55 ≈ the model was, on average, a coin-flip or worse — send it up.
    pub p_avg_min: f64,
    /// Escalate when any **single** token's probability dipped below this — one
    /// real hesitation (a fork the small model wasn't sure of) is enough.
    pub p_tok_min: f64,
    /// Requests longer than this (chars) or matching a "hard" keyword start on
    /// the Reasoner directly.
    pub hard_len: usize,
}

impl Default for CascadeConfig {
    fn default() -> Self {
        Self { enabled: false, p_avg_min: 0.55, p_tok_min: 0.08, hard_len: 4000 }
    }
}

impl CascadeConfig {
    /// Defaults on for forward-compatibility with the day this gets wired into
    /// a live routing decision — today `run_cascade` has no call site, so this
    /// flag currently changes nothing. `KORTEX_CASCADE=0` pre-disables it.
    pub fn from_env() -> Self {
        let mut cfg = Self {
            enabled: super::env_flag::on("KORTEX_CASCADE", true),
            ..Self::default()
        };
        if let Some(v) = env_f64("KORTEX_CASCADE_PAVG") {
            cfg.p_avg_min = v.clamp(0.0, 1.0);
        }
        if let Some(v) = env_f64("KORTEX_CASCADE_PTOK") {
            cfg.p_tok_min = v.clamp(0.0, 1.0);
        }
        cfg
    }
}

fn env_f64(k: &str) -> Option<f64> {
    std::env::var(k).ok()?.trim().parse().ok()
}

/// Keywords that mark a turn as hard enough to skip the Operator entirely.
const HARD_KEYWORDS: &[&str] = &[
    "refactor", "architecture", "design ", "prove", "why does", "why is",
    "race condition", "deadlock", "borrow checker", "lifetime", "unsafe",
    "concurren", "debug ", "root cause", "trade-off", "tradeoff", "optimize",
    "algorithm", "complexity", "security", "vulnerab", "step by step",
];

/// Pre-classification: should this turn even start on the Operator?
pub fn start_tier(system: &str, prompt: &str, has_tools: bool, cfg: &CascadeConfig) -> Tier {
    // A big prompt (lots of context to reason over) or a hard-keyword ask goes
    // straight to the Reasoner — the Operator call would likely be escalated
    // anyway, so skip paying for it.
    if prompt.len() + system.len() > cfg.hard_len {
        return Tier::Reasoner;
    }
    let hay = prompt.to_lowercase();
    if HARD_KEYWORDS.iter().any(|k| hay.contains(k)) {
        return Tier::Reasoner;
    }
    // Tool-driving turns are fine on the Operator (it's the structured-output
    // specialist); pure-reasoning turns without tools that are non-trivial in
    // length lean Reasoner.
    if !has_tools && prompt.chars().count() > 600 {
        return Tier::Reasoner;
    }
    Tier::Operator
}

/// Confidence extracted from a completion — the escalation signal.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Confidence {
    /// Mean per-token probability in [0,1]. `None` when no logprobs were returned.
    pub mean_p: Option<f64>,
    /// Worst single-token probability in [0,1]. `None` when no logprobs.
    pub min_p: Option<f64>,
    /// A hedging / refusal / "I'm not sure" phrase was found in the text.
    pub hedged: bool,
    /// The answer was empty or unusably short.
    pub empty: bool,
}

const HEDGE_PHRASES: &[&str] = &[
    "i'm not sure", "im not sure", "i am not sure", "not certain", "i cannot",
    "i can't help", "i don't know", "i do not know", "as an ai", "unclear to me",
    "might be wrong", "i'm unable", "cannot determine", "hard to say",
];

/// Pull the confidence signal out of an OpenAI `/chat/completions` response body
/// and the extracted answer text. Tolerant of the several logprob shapes
/// (`choices[].logprobs.content[].logprob`, or the older `token_logprobs`).
pub fn confidence_from_response(resp: &Value, text: &str) -> Confidence {
    let logprobs = collect_logprobs(resp);
    let (mean_p, min_p) = if logprobs.is_empty() {
        (None, None)
    } else {
        let mean_lp = logprobs.iter().sum::<f64>() / logprobs.len() as f64;
        let min_lp = logprobs.iter().cloned().fold(f64::INFINITY, f64::min);
        (Some(mean_lp.exp()), Some(min_lp.exp()))
    };
    let lower = text.to_lowercase();
    Confidence {
        mean_p,
        min_p,
        hedged: HEDGE_PHRASES.iter().any(|p| lower.contains(p)),
        empty: text.trim().chars().count() < 2,
    }
}

/// Gather per-token logprobs from whatever shape the backend used.
fn collect_logprobs(resp: &Value) -> Vec<f64> {
    let mut out = Vec::new();
    let Some(choice) = resp.get("choices").and_then(|c| c.get(0)) else {
        return out;
    };
    let lp = &choice["logprobs"];
    // Modern: logprobs.content = [{ token, logprob, ... }, ...]
    if let Some(arr) = lp.get("content").and_then(Value::as_array) {
        for tok in arr {
            if let Some(v) = tok.get("logprob").and_then(Value::as_f64) {
                out.push(v);
            }
        }
    }
    // Legacy / llama.cpp completion: logprobs.token_logprobs = [f64, ...]
    if out.is_empty() {
        if let Some(arr) = lp.get("token_logprobs").and_then(Value::as_array) {
            for v in arr {
                if let Some(f) = v.as_f64() {
                    out.push(f);
                }
            }
        }
    }
    out
}

/// The routing decision after the Operator answered.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum Decision {
    /// Keep the Operator's answer.
    Accept,
    /// Re-run on the Reasoner; `reason` is why.
    Escalate { reason: String },
}

impl Decision {
    pub fn is_escalate(&self) -> bool {
        matches!(self, Decision::Escalate { .. })
    }
}

/// Decide accept-vs-escalate from the confidence signal.
pub fn decide(conf: &Confidence, cfg: &CascadeConfig) -> Decision {
    if conf.empty {
        return Decision::Escalate { reason: "operator returned an empty answer".into() };
    }
    if conf.hedged {
        return Decision::Escalate { reason: "operator hedged / declined".into() };
    }
    if let Some(mean_p) = conf.mean_p {
        if mean_p < cfg.p_avg_min {
            return Decision::Escalate {
                reason: format!(
                    "low mean confidence {:.2} < {:.2}",
                    mean_p, cfg.p_avg_min
                ),
            };
        }
    }
    if let Some(min_p) = conf.min_p {
        if min_p < cfg.p_tok_min {
            return Decision::Escalate {
                reason: format!(
                    "a token dipped to {:.3} < {:.3} (a real fork)",
                    min_p, cfg.p_tok_min
                ),
            };
        }
    }
    Decision::Accept
}

/// What the cascade did, for the stats panel / trace.
#[derive(Debug, Clone, Serialize)]
pub struct CascadeReport {
    pub started_on: Tier,
    pub answered_by: Tier,
    pub escalated: bool,
    pub reason: Option<String>,
    pub operator_confidence: Option<Confidence>,
    /// Approx tokens the Operator spent (wasted if it escalated).
    pub operator_tokens: usize,
    /// Approx tokens the Reasoner spent (0 if it never ran).
    pub reasoner_tokens: usize,
}

impl CascadeReport {
    /// True when the big model never ran — the win case.
    pub fn saved_reasoner(&self) -> bool {
        self.answered_by == Tier::Operator
    }
}

/// One OpenAI-compatible chat call that asks for logprobs, with the `/api/v1`→
/// `/v1` path fallback the rest of the stack uses. Returns `(text, raw_json)`.
async fn chat_with_logprobs(
    client: &reqwest::Client,
    base: &str,
    model: &str,
    messages: &Value,
    max_tokens: u64,
) -> Result<(String, Value), String> {
    let root = base.trim_end_matches('/');
    let body = json!({
        "model": model,
        "messages": messages,
        "max_tokens": max_tokens,
        "temperature": 0.3,
        "stream": false,
        "logprobs": true,
    });
    let tok = std::env::var("LEMONADE_TOKEN").unwrap_or_default();
    let mut last_err = String::from("unknown");
    for path in ["/api/v1/chat/completions", "/v1/chat/completions"] {
        let mut req = client.post(format!("{root}{path}")).json(&body);
        if !tok.trim().is_empty() {
            req = req.bearer_auth(tok.trim());
        }
        match req.send().await {
            Ok(r) => {
                if r.status().as_u16() == 404 {
                    last_err = format!("404 at {path}");
                    continue;
                }
                let raw = r.text().await.unwrap_or_default();
                let v: Value = serde_json::from_str(&raw)
                    .map_err(|e| format!("parse: {e}"))?;
                let text = v["choices"][0]["message"]["content"]
                    .as_str()
                    .or_else(|| v["response"].as_str())
                    .unwrap_or("")
                    .to_string();
                return Ok((text, v));
            }
            Err(e) => last_err = format!("{e}"),
        }
    }
    Err(format!("cascade chat unreachable at {root}: {last_err}"))
}

/// Run the cascade for one turn: pre-classify, try the Operator, escalate to the
/// Reasoner only if the confidence gate says so. Returns the final answer and a
/// report of what happened. When disabled, goes straight to the Reasoner (the
/// historical single-model behaviour) with no extra call.
#[allow(clippy::too_many_arguments)]
pub async fn run_cascade(
    client: &reqwest::Client,
    cfg: &CascadeConfig,
    operator_base: &str,
    operator_model: &str,
    reasoner_base: &str,
    reasoner_model: &str,
    system: &str,
    prompt: &str,
    has_tools: bool,
) -> Result<(String, CascadeReport), String> {
    // #2: a semantic-cache hit short-circuits the whole cascade — zero
    // inference. Keyed on the reasoner model id (the assistant's identity), so
    // the tier that produced it is irrelevant to the lookup.
    let cache = super::semantic_cache::global();
    if let Some(hit) = cache.lookup(reasoner_model, system, prompt) {
        return Ok((
            hit,
            CascadeReport {
                started_on: Tier::Operator,
                answered_by: Tier::Operator,
                escalated: false,
                reason: Some("semantic-cache hit — no inference".into()),
                operator_confidence: None,
                operator_tokens: 0,
                reasoner_tokens: 0,
            },
        ));
    }

    let messages = json!([
        { "role": "system", "content": system },
        { "role": "user", "content": prompt },
    ]);

    let reasoner = |client: &reqwest::Client| {
        let messages = messages.clone();
        let base = reasoner_base.to_string();
        let model = reasoner_model.to_string();
        let client = client.clone();
        async move {
            chat_with_logprobs(&client, &base, &model, &messages, 4096)
                .await
                .map(|(t, _)| t)
        }
    };

    // Disabled, or pre-classified hard → Reasoner only.
    let start = if cfg.enabled {
        start_tier(system, prompt, has_tools, cfg)
    } else {
        Tier::Reasoner
    };
    if start == Tier::Reasoner {
        let text = reasoner(client).await?;
        let reasoner_tokens = approx_tokens(&text);
        cache.store(reasoner_model, system, prompt, &text);
        return Ok((
            text,
            CascadeReport {
                started_on: Tier::Reasoner,
                answered_by: Tier::Reasoner,
                escalated: false,
                reason: (!cfg.enabled).then(|| "cascade disabled".to_string()),
                operator_confidence: None,
                operator_tokens: 0,
                reasoner_tokens,
            },
        ));
    }

    // Try the Operator.
    let (op_text, op_raw) =
        chat_with_logprobs(client, operator_base, operator_model, &messages, 2048).await?;
    let conf = confidence_from_response(&op_raw, &op_text);
    let decision = decide(&conf, cfg);
    let operator_tokens = approx_tokens(&op_text);

    match decision {
        Decision::Accept => {
            cache.store(reasoner_model, system, prompt, &op_text);
            Ok((
            op_text,
            CascadeReport {
                started_on: Tier::Operator,
                answered_by: Tier::Operator,
                escalated: false,
                reason: None,
                operator_confidence: Some(conf),
                operator_tokens,
                reasoner_tokens: 0,
            },
        ))
        }
        Decision::Escalate { reason } => {
            let text = reasoner(client).await?;
            let reasoner_tokens = approx_tokens(&text);
            cache.store(reasoner_model, system, prompt, &text);
            Ok((
                text,
                CascadeReport {
                    started_on: Tier::Operator,
                    answered_by: Tier::Reasoner,
                    escalated: true,
                    reason: Some(reason),
                    operator_confidence: Some(conf),
                    operator_tokens,
                    reasoner_tokens,
                },
            ))
        }
    }
}

fn approx_tokens(s: &str) -> usize {
    (s.len() / 4).max(if s.is_empty() { 0 } else { 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> CascadeConfig {
        CascadeConfig { enabled: true, ..Default::default() }
    }

    #[test]
    fn easy_short_tool_turn_starts_on_operator() {
        assert_eq!(start_tier("sys", "rename foo to bar", true, &cfg()), Tier::Operator);
    }

    #[test]
    fn hard_keyword_jumps_to_reasoner() {
        assert_eq!(
            start_tier("sys", "explain the race condition in this scheduler", true, &cfg()),
            Tier::Reasoner
        );
    }

    #[test]
    fn big_prompt_jumps_to_reasoner() {
        let big = "x".repeat(5000);
        assert_eq!(start_tier("sys", &big, true, &cfg()), Tier::Reasoner);
    }

    #[test]
    fn long_reasoning_without_tools_leans_reasoner() {
        let p = "please walk me through what this code does line by line ".repeat(12);
        assert_eq!(start_tier("sys", &p, false, &cfg()), Tier::Reasoner);
    }

    #[test]
    fn confidence_from_modern_logprobs() {
        let resp = json!({
            "choices": [{
                "logprobs": { "content": [
                    { "token": "a", "logprob": -0.1 },
                    { "token": "b", "logprob": -0.2 },
                    { "token": "c", "logprob": -0.05 },
                ]}
            }]
        });
        let c = confidence_from_response(&resp, "abc");
        // mean logprob ≈ -0.1167 → p ≈ 0.89
        assert!(c.mean_p.unwrap() > 0.85 && c.mean_p.unwrap() < 0.92);
        assert!(c.min_p.unwrap() > 0.80); // exp(-0.2)
        assert!(!c.hedged && !c.empty);
    }

    #[test]
    fn legacy_token_logprobs_shape() {
        let resp = json!({
            "choices": [{ "logprobs": { "token_logprobs": [-0.3, -0.3] } }]
        });
        let c = confidence_from_response(&resp, "hi there");
        assert!(c.mean_p.is_some());
    }

    #[test]
    fn no_logprobs_is_text_only_signal() {
        let resp = json!({ "choices": [{ "message": { "content": "ok" } }] });
        let c = confidence_from_response(&resp, "ok");
        assert!(c.mean_p.is_none() && c.min_p.is_none());
    }

    #[test]
    fn accept_high_confidence() {
        let conf = Confidence { mean_p: Some(0.9), min_p: Some(0.5), hedged: false, empty: false };
        assert_eq!(decide(&conf, &cfg()), Decision::Accept);
    }

    #[test]
    fn escalate_on_low_mean() {
        let conf = Confidence { mean_p: Some(0.4), min_p: Some(0.4), hedged: false, empty: false };
        assert!(decide(&conf, &cfg()).is_escalate());
    }

    #[test]
    fn escalate_on_single_token_fork() {
        let conf = Confidence { mean_p: Some(0.8), min_p: Some(0.02), hedged: false, empty: false };
        assert!(decide(&conf, &cfg()).is_escalate());
    }

    #[test]
    fn escalate_on_hedge_even_with_no_logprobs() {
        let conf = confidence_from_response(&json!({"choices":[{}]}), "I'm not sure, but maybe.");
        assert!(decide(&conf, &cfg()).is_escalate());
    }

    #[test]
    fn escalate_on_empty() {
        let conf = confidence_from_response(&json!({"choices":[{}]}), "");
        assert!(decide(&conf, &cfg()).is_escalate());
    }

    #[test]
    fn no_logprobs_high_text_quality_is_accepted() {
        // No logprobs, no hedge, non-empty → trust the Operator (can't prove it
        // wrong without a signal; escalating everything would defeat the point).
        let conf = confidence_from_response(&json!({"choices":[{}]}), "Here is the fix: add a semicolon.");
        assert_eq!(decide(&conf, &cfg()), Decision::Accept);
    }
}
