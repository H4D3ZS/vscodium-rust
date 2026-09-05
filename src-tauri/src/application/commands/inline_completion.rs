//! Cursor-style inline completion (ghost text) served by a local model.
//!
//! This is the one Cursor feature the IDE did not have: `lsp_completion` is
//! static LSP, not model-driven.
//!
//! # Why a separate model
//!
//! A suggestion that arrives after the keystroke it predicted is worthless, so
//! this path is latency-bound, not quality-bound. The agent model (35B MoE)
//! generates at 13.3 tok/s and cannot serve it at any context size. So
//! completion runs on [`ModelRole::Completion`] — a small model held resident
//! *alongside* the agent model. Measured warm latency: 217-328ms.
//!
//! That requires Lemonade's `max_loaded_models >= 2`; with only one slot, loading
//! the completion model would evict the agent model on every keystroke.
//!
//! # Why `/v1/completions` and not the chat endpoint
//!
//! Chat-shaped prompts make a small model *converse about* the code: measured,
//! Qwen3-0.6B echoed the current line back instead of continuing it. The raw
//! text-completion endpoint continues the token stream, which is what ghost text
//! actually is. Same model, same prompt body, correct output.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::ai::{lemonade_role_candidates, ModelRole};

/// Lemonade base URL, matching the resolution used elsewhere in `ai.rs`.
fn lemonade_base_url() -> String {
    std::env::var("LEMONADE_URL").unwrap_or_else(|_| "http://localhost:13305".to_string())
}

/// Prefix context sent to the model. The completion model runs an 8192-token
/// window and only the code immediately around the cursor predicts the next
/// token; sending more costs latency for nothing.
const PREFIX_CHARS: usize = 2_000;
/// Suffix context. Smaller than the prefix — it disambiguates (e.g. stops the
/// model re-emitting a closing brace that already exists) without dominating.
const SUFFIX_CHARS: usize = 500;
/// Ghost text longer than this is noise; a wrong long suggestion costs more to
/// read and reject than a short one saves.
const MAX_COMPLETION_TOKENS: u32 = 64;
/// Hard cap on suggestion length when no resume marker matched — without it a
/// model that ran past the insertion point emits a whole second function.
const MAX_COMPLETION_LINES: usize = 8;
/// Cache ceiling. Entries are tiny; this bounds a long editing session.
const CACHE_CAPACITY: usize = 256;
/// Hard ceiling per request. Past this the keystroke it predicted is long gone,
/// so failing fast beats returning a stale suggestion.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(6);

/// One inline-completion request. `prefix` and `suffix` are the document split
/// at the cursor.
#[derive(Debug, Deserialize)]
pub struct InlineCompletionArgs {
    pub prefix: String,
    pub suffix: String,
    /// Language id (`rust`, `typescript`, …). Only used to label the prompt.
    #[serde(default)]
    pub language: String,
    /// Return only the first line. Multi-line ghost text is disruptive mid-line.
    #[serde(default)]
    pub single_line: bool,
}

/// A suggestion, plus enough detail for the UI to show why it was fast or slow.
#[derive(Debug, Serialize, Default, Clone)]
pub struct InlineCompletion {
    /// Text to insert at the cursor. Empty means "no suggestion" — never an error.
    pub text: String,
    pub model: String,
    pub latency_ms: u64,
    /// Served from cache; no model call was made.
    pub cached: bool,
}

/// Prefix/suffix-keyed suggestion cache.
///
/// Typing re-issues near-identical requests constantly, and the completion
/// model's first call is a ~2.6s cold start. Paying that once per context
/// instead of once per keystroke is the difference between usable and not.
static CACHE: Mutex<Option<HashMap<u64, String>>> = Mutex::new(None);

fn cache_key(prefix: &str, suffix: &str, single_line: bool) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    prefix.hash(&mut h);
    suffix.hash(&mut h);
    single_line.hash(&mut h);
    h.finish()
}

fn cache_get(key: u64) -> Option<String> {
    CACHE.lock().ok()?.as_ref()?.get(&key).cloned()
}

fn cache_put(key: u64, value: String) {
    let Ok(mut guard) = CACHE.lock() else { return };
    let map = guard.get_or_insert_with(HashMap::new);
    // Cheap bound: clear wholesale rather than track recency. The cache exists to
    // absorb keystroke bursts, and a burst's entries are all recent anyway.
    if map.len() >= CACHE_CAPACITY {
        map.clear();
    }
    map.insert(key, value);
}

/// Take the last `n` chars, splitting on a char boundary so multi-byte source
/// (comments, string literals) cannot panic the slice.
fn tail(s: &str, n: usize) -> &str {
    if s.len() <= n {
        return s;
    }
    let start = s.len() - n;
    match s.char_indices().find(|(i, _)| *i >= start) {
        Some((i, _)) => &s[i..],
        None => s,
    }
}

/// Take the first `n` chars, on a char boundary.
fn head(s: &str, n: usize) -> &str {
    if s.len() <= n {
        return s;
    }
    match s.char_indices().nth_back_safe(n) {
        Some(i) => &s[..i],
        None => s,
    }
}

/// Helper: byte index of the nth char, saturating at the end.
trait NthBackSafe {
    fn nth_back_safe(self, n: usize) -> Option<usize>;
}
impl<I: Iterator<Item = (usize, char)>> NthBackSafe for I {
    fn nth_back_safe(mut self, n: usize) -> Option<usize> {
        let mut last = 0;
        for (i, _) in self.by_ref() {
            if i > n {
                return Some(last);
            }
            last = i;
        }
        Some(last)
    }
}

/// A completion-role model that is **already loaded**, if any.
///
/// Reads `all_models_loaded` from `GET /api/v1/health` — models that are
/// resident right now, not merely downloaded. Returning `None` (server
/// unreachable, nothing suitable resident) means "no ghost text", never "load
/// something": the agent model's VRAM is not ours to spend.
async fn lemonade_loaded_completion_model(
    client: &reqwest::Client,
    base: &str,
) -> Option<&'static str> {
    let resp = client
        .get(format!("{}/api/v1/health", base.trim_end_matches('/')))
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .ok()?;
    let body = resp.json::<serde_json::Value>().await.ok()?;
    let loaded: Vec<&str> = body
        .get("all_models_loaded")?
        .as_array()?
        .iter()
        .filter_map(|m| m.get("model_name").and_then(|n| n.as_str()))
        .collect();

    lemonade_role_candidates(ModelRole::Completion)
        .iter()
        .find(|want| loaded.contains(*want))
        .copied()
}

/// Clean a raw model continuation into insertable ghost text.
///
/// Small models pad, fence, and restate. Left raw, that lands as garbage in the
/// buffer, so every one of these rules corresponds to an observed failure.
pub fn sanitize_completion(raw: &str, suffix: &str, single_line: bool) -> String {
    let mut out = raw;

    // Markdown fences, even when told not to emit them.
    if let Some(rest) = out.trim_start().strip_prefix("```") {
        // Drop the language tag on the fence line.
        out = rest.split_once('\n').map(|(_, r)| r).unwrap_or("");
    }
    if let Some(idx) = out.find("```") {
        out = &out[..idx];
    }

    if single_line {
        let first = out.split('\n').next().unwrap_or("");
        return first.trim_end().to_string();
    }

    // Cut where the document resumes.
    //
    // Small models do not stop at the insertion point — measured, they emit the
    // completion, then the closing brace, then `fn main()`, then test cases.
    // Truncating at the first line that matches where the suffix picks up is
    // what makes the suggestion end in the right place.
    //
    // Matched line-wise, never as a substring: the resume marker is usually a
    // bare `}`, and a substring search for that would cut a legitimate
    // completion like `if x { y() }` in half.
    let mut lines: Vec<&str> = out.lines().collect();
    if let Some(marker) = suffix
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
    {
        if let Some(pos) = lines.iter().position(|l| l.trim() == marker) {
            lines.truncate(pos);
        }
    }

    // Runaway guard for when no marker matched (end of file, or the model went
    // somewhere unrelated). Long ghost text costs more to read and reject than
    // it saves.
    lines.truncate(MAX_COMPLETION_LINES);

    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }

    lines
        .join("\n")
        .trim_end_matches(['\n', '\r', ' ', '\t'])
        .to_string()
}

/// Build the raw-completion prompt.
///
/// Deliberately NOT chat-shaped. A lightweight comment header gives the model the
/// language and the code that follows the cursor, then the prefix is handed over
/// verbatim so the natural continuation of the token stream is the completion.
/// The suffix is deliberately NOT included in the prompt. Measured: passing it as
/// a comment header made the model imitate the header instead of the code — a
/// TypeScript completion started emitting `// code after the cursor:` back. The
/// suffix is used for truncation in [`sanitize_completion`] instead, which is
/// where it actually earns its keep.
fn build_prompt(prefix: &str, suffix: &str, language: &str) -> String {
    let _ = suffix;
    let mut p = String::new();
    if !language.is_empty() {
        p.push_str(&format!("// language: {language}\n"));
    }
    p.push_str(prefix);
    p
}

/// Complete at the cursor. Returns empty text rather than an error when there is
/// nothing useful to say — a failed completion must never interrupt typing.
#[tauri::command]
pub async fn ai_inline_completion(
    args: InlineCompletionArgs,
) -> Result<InlineCompletion, String> {
    let started = Instant::now();

    // Nothing to predict from an empty buffer, and mid-identifier suggestions are
    // noise — wait for a boundary.
    if args.prefix.trim().is_empty() {
        return Ok(InlineCompletion::default());
    }

    let prefix = tail(&args.prefix, PREFIX_CHARS);
    let suffix = head(&args.suffix, SUFFIX_CHARS);
    let key = cache_key(prefix, suffix, args.single_line);

    if let Some(hit) = cache_get(key) {
        return Ok(InlineCompletion {
            text: hit,
            model: String::new(),
            latency_ms: started.elapsed().as_millis() as u64,
            cached: true,
        });
    }

    let client = reqwest::Client::new();
    let base = lemonade_base_url();

    // Only use a completion model that is ALREADY RESIDENT. Never trigger a load.
    //
    // Two separate hazards, both measured:
    //
    //  1. With one LLM slot, loading the completion model EVICTS the agent model,
    //     so every keystroke would cost a ~32s reload of the 35B.
    //  2. Even with a spare slot, co-residency is not free. The agent model loses
    //     VRAM to it and slows down measurably:
    //
    //       35B alone                        : 12.4 tok/s gen, 460 tok/s prefill
    //       35B + completion + embedding      :  8.7 tok/s gen, 320 tok/s prefill
    //
    //     That is ~30% off the model the user actually works in, and raising
    //     `-ncmoe` only recovers a little of it (18 gave 9.9 / 333). The VRAM is
    //     simply spent.
    //
    // The agent model is the priority, so ghost text is a bonus that costs it
    // nothing: if someone has deliberately loaded a completion model, use it;
    // otherwise stay silent and let the caller fall back.
    let Some(model) = lemonade_loaded_completion_model(&client, &base).await else {
        return Ok(InlineCompletion::default());
    };

    let body = serde_json::json!({
        "model": model,
        "prompt": build_prompt(prefix, suffix, &args.language),
        "max_tokens": MAX_COMPLETION_TOKENS,
        "temperature": 0.1,
        "top_p": 0.95,
        // Two blank lines means the model has moved on to a new construct; that
        // is past the end of a useful suggestion.
        "stop": ["\n\n\n", "```"],
    });

    let resp = client
        .post(format!("{}/v1/completions", base.trim_end_matches('/')))
        .json(&body)
        .timeout(REQUEST_TIMEOUT)
        .send()
        .await;

    // Any transport failure is "no suggestion". Typing continues.
    let Ok(resp) = resp else { return Ok(InlineCompletion::default()) };
    if !resp.status().is_success() {
        return Ok(InlineCompletion::default());
    }
    let Ok(json) = resp.json::<serde_json::Value>().await else {
        return Ok(InlineCompletion::default());
    };

    let raw = json
        .get("choices")
        .and_then(|c| c.as_array())
        .and_then(|a| a.first())
        .and_then(|c| c.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or_default();

    let text = sanitize_completion(raw, suffix, args.single_line);
    if !text.trim().is_empty() {
        cache_put(key, text.clone());
    }

    Ok(InlineCompletion {
        text,
        model: model.to_string(),
        latency_ms: started.elapsed().as_millis() as u64,
        cached: false,
    })
}

#[cfg(test)]
mod inline_completion_tests {
    use super::*;

    /// Every rule here corresponds to an observed small-model failure; raw output
    /// is not safe to insert into a buffer.
    #[test]
    fn fences_are_stripped() {
        assert_eq!(sanitize_completion("```rust\na + b\n```", "", false), "a + b");
        assert_eq!(sanitize_completion("a + b\n```", "", false), "a + b");
    }

    /// The model re-emitting the closing brace that already follows the cursor
    /// was the most common duplication in practice.
    #[test]
    fn text_already_after_the_cursor_is_not_repeated() {
        let out = sanitize_completion("    a + b\n}\n", "\n}\n", false);
        assert_eq!(out, "    a + b");
        assert!(!out.contains('}'), "must not duplicate the existing brace");
    }

    /// Observed against the live model: it does not stop at the insertion point.
    /// It emits the completion, the closing brace, then a whole new function and
    /// test cases. Truncating where the document resumes is what makes ghost text
    /// end in the right place — a trailing-only match cannot do it, because the
    /// duplicate is buried in the middle.
    #[test]
    fn overrun_past_the_insertion_point_is_cut_at_the_resume_marker() {
        // Note: a real newline-separated fixture, not `\`-continued — the
        // continuation form strips the leading indentation being asserted on.
        let raw = concat!(
            "        let w = word.to_lowercase();\n",
            "        *counts.entry(w).or_insert(0) += 1;\n",
            "    }\n",
            "    counts\n",
            "}\n",
            "\n",
            "fn main() {\n",
            "    let text = \"hello world\";\n",
        );
        let out = sanitize_completion(raw, "    }\n    counts\n}\n", false);
        assert_eq!(
            out,
            "        let w = word.to_lowercase();\n        *counts.entry(w).or_insert(0) += 1;"
        );
        assert!(!out.contains("fn main"), "must not emit a second function");
    }

    /// With no resume marker to find (end of file), the runaway guard bounds it.
    #[test]
    fn unmatched_completion_is_length_capped() {
        let raw = (1..=20).map(|i| format!("line{i};")).collect::<Vec<_>>().join("\n");
        let out = sanitize_completion(&raw, "", false);
        assert_eq!(out.lines().count(), MAX_COMPLETION_LINES);
    }

    /// The suffix must not reach the prompt. Passing it as a comment header made
    /// the model imitate the header — a TypeScript completion emitted
    /// `// code after the cursor:` back into the buffer.
    #[test]
    fn suffix_never_leaks_into_the_prompt() {
        let p = build_prompt("const x = 1;\n", "SUFFIX_SENTINEL\n}", "typescript");
        assert!(!p.contains("SUFFIX_SENTINEL"), "suffix must not be prompted, only used to truncate");
    }

    /// The dedup must be a trailing-overlap match, not a substring search: the
    /// duplicated token is usually a bare `}`, and searching for that anywhere
    /// would cut a legitimate completion in half.
    #[test]
    fn braces_inside_a_completion_survive_the_dedup() {
        assert_eq!(
            sanitize_completion("if x { y() }", "\n}\n", false),
            "if x { y() }"
        );
    }

    #[test]
    fn single_line_mode_takes_only_the_first_line() {
        assert_eq!(
            sanitize_completion("let x = 1;\nlet y = 2;", "", true),
            "let x = 1;"
        );
    }

    /// Multi-byte source must not panic the context slicing.
    #[test]
    fn multibyte_source_is_sliced_on_char_boundaries() {
        let s = "// 日本語のコメント\nfn f() {".repeat(200);
        let t = tail(&s, PREFIX_CHARS);
        assert!(t.len() <= PREFIX_CHARS + 4);
        assert!(std::str::from_utf8(t.as_bytes()).is_ok());
        let h = head(&s, SUFFIX_CHARS);
        assert!(std::str::from_utf8(h.as_bytes()).is_ok());
    }

    /// The prompt must hand the prefix over verbatim and end with it — anything
    /// appended after it makes the model continue THAT instead of the code.
    #[test]
    fn prompt_ends_with_the_users_code() {
        let p = build_prompt("fn add(a: i32) {", "\n}\n", "rust");
        assert!(p.ends_with("fn add(a: i32) {"), "prompt tail must be the cursor context");
        assert!(p.contains("// language: rust"));
    }

    /// An empty buffer must not spend a model call.
    #[tokio::test]
    async fn blank_prefix_returns_no_suggestion_without_calling_the_model() {
        let out = ai_inline_completion(InlineCompletionArgs {
            prefix: "   \n  ".into(),
            suffix: String::new(),
            language: "rust".into(),
            single_line: false,
        })
        .await
        .unwrap();
        assert!(out.text.is_empty());
        assert!(!out.cached);
    }
}
