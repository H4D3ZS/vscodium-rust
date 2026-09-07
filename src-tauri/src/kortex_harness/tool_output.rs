//! Deterministic compression of **tool result** payloads before they re-enter
//! the prompt — kortex's answer to Headroom's SmartCrusher / CodeCompressor /
//! CCR, done as pure structure-walking with no learned model and no telemetry.
//!
//! `agent_harness::compress_old_tool_results` already drops *old* results to a
//! one-line head. This is the finer lever: it shrinks the *bulk* of a result
//! (even a recent one) while keeping its shape and signal, and stashes the
//! original so the model can `recall({"id": "<id>"})` the full text on demand —
//! the same "context lives compressed, expands when needed" contract the tool
//! schemas use via `expand`.
//!
//! Two crushers, picked by content:
//!   * **JSON** → shape-preserving digest: keys kept (they're signal), long
//!     string values truncated, large arrays capped with an `…(+N more)` marker.
//!     Valid-JSON-ish and readable; a 4000-token API blob becomes ~200 tokens
//!     without losing the structure the model reasons over.
//!   * **text / logs** → head + tail with the middle elided (`[… N lines /
//!     M chars elided …]`), because the diagnostic signal in a log lives at the
//!     ends, not the middle.
//!
//! Behind the same `KORTEX_HARNESS` opt-in as the rest of the harness, with an
//! independent `KORTEX_HARNESS_TOOL_OUTPUT` off-switch. Idempotent: a compacted
//! message carries a marker line and is left alone on a resend.

use super::tool_digest::approx_tokens;
use super::turn_stash;
use serde_json::Value;

/// Marker appended to a compacted tool message so a resend of the same turn is a
/// no-op (idempotency) and the model knows the full text is recallable.
const MARK: &str = "\u{2039}kortex:compacted\u{203a}"; // ‹kortex:compacted›

#[derive(Debug, Clone)]
pub struct ToolOutputConfig {
    pub enabled: bool,
    /// Only compress a result whose content exceeds this many chars (~/4 tokens).
    pub min_chars: usize,
    /// Truncate an individual JSON string value past this many chars.
    pub max_string: usize,
    /// Cap a JSON array to this many elements (+ an `…(+N more)` marker).
    pub max_array: usize,
    /// Text/log crush: lines kept from the head and the tail.
    pub head_lines: usize,
    pub tail_lines: usize,
    /// Stash the original so `recall` can rehydrate it. Off → the full text is
    /// gone (smaller, but unrecoverable).
    pub recallable: bool,
}

impl Default for ToolOutputConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_chars: 1200, // ~300 tokens; below this, compressing isn't worth it
            max_string: 240,
            max_array: 8,
            head_lines: 24,
            tail_lines: 12,
            recallable: true,
        }
    }
}

impl ToolOutputConfig {
    pub fn from_env() -> Self {
        // On when the harness is on, unless explicitly disabled.
        let harness_on = matches!(
            std::env::var("KORTEX_HARNESS").ok().as_deref(),
            Some("1") | Some("true") | Some("on")
        );
        let sub_off = matches!(
            std::env::var("KORTEX_HARNESS_TOOL_OUTPUT").ok().as_deref(),
            Some("0") | Some("false") | Some("off")
        );
        let mut cfg = Self { enabled: harness_on && !sub_off, ..Self::default() };
        if let Some(n) = env_usize("KORTEX_HARNESS_TOOL_MIN") {
            cfg.min_chars = n;
        }
        cfg
    }
}

fn env_usize(k: &str) -> Option<usize> {
    std::env::var(k).ok()?.trim().parse().ok()
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ToolOutputReport {
    pub messages_scanned: usize,
    pub messages_compacted: usize,
    pub chars_before: usize,
    pub chars_after: usize,
}

impl ToolOutputReport {
    pub fn approx_tokens_saved(&self) -> usize {
        approx_tokens_of(self.chars_before).saturating_sub(approx_tokens_of(self.chars_after))
    }
}

fn approx_tokens_of(chars: usize) -> usize {
    (chars / 4).max(if chars == 0 { 0 } else { 1 })
}

/// Compress every large `role:"tool"` / `role:"function"` message in an OpenAI
/// chat body in place. Returns what it did. Safe on any shape.
pub fn compress_tool_messages(body: &mut Value, cfg: &ToolOutputConfig) -> ToolOutputReport {
    let mut report = ToolOutputReport::default();
    if !cfg.enabled {
        return report;
    }
    let Some(messages) = body
        .get_mut("messages")
        .and_then(Value::as_array_mut)
    else {
        return report;
    };

    for msg in messages.iter_mut() {
        let is_tool = matches!(
            msg.get("role").and_then(Value::as_str),
            Some("tool") | Some("function")
        );
        if !is_tool {
            continue;
        }
        // Only the string-content form; structured content is left untouched.
        let Some(content) = msg.get("content").and_then(Value::as_str) else {
            continue;
        };
        report.messages_scanned += 1;

        if content.contains(MARK) || content.len() < cfg.min_chars {
            continue; // already compacted, or not worth it
        }

        let Some(compact) = crush(content, cfg) else {
            continue; // crusher couldn't beat the original meaningfully
        };

        let suffix = if cfg.recallable {
            let id = turn_stash::next_id();
            turn_stash::put(&id, content);
            format!(
                "\n\n{MARK} compacted from {} chars — recall({{\"id\": \"{id}\"}}) for the full output",
                content.len()
            )
        } else {
            format!("\n\n{MARK} compacted from {} chars", content.len())
        };

        report.messages_compacted += 1;
        report.chars_before += content.len();
        report.chars_after += compact.len() + suffix.len();

        let out = format!("{compact}{suffix}");
        if let Some(obj) = msg.as_object_mut() {
            obj.insert("content".into(), Value::String(out));
        }
    }
    report
}

/// Crush one result string. `None` when compression wouldn't help (small or the
/// digest isn't meaningfully shorter). Tries JSON first, falls back to log/text.
pub fn crush(content: &str, cfg: &ToolOutputConfig) -> Option<String> {
    let trimmed = content.trim_start();
    let compact = if trimmed.starts_with('{') || trimmed.starts_with('[') {
        match serde_json::from_str::<Value>(content) {
            Ok(v) => {
                let crushed = crush_json(&v, cfg, 0);
                // Pretty is easier for the model to read than a minified blob and
                // the token cost of the whitespace is dwarfed by what we removed.
                serde_json::to_string_pretty(&crushed).unwrap_or_else(|_| crush_text(content, cfg))
            }
            Err(_) => crush_text(content, cfg), // JSON-looking but invalid → treat as text
        }
    } else {
        crush_text(content, cfg)
    };

    // Only accept the crush if it's a real win (≥25% off). Otherwise the recall
    // indirection isn't worth the lost detail.
    if compact.len() + compact.len() / 3 < content.len() {
        Some(compact)
    } else {
        None
    }
}

/// Shape-preserving JSON digest. Keys are kept; long strings truncated; big
/// arrays capped. Recurses to a bounded depth so a deeply nested blob can't
/// blow the stack or the budget.
fn crush_json(v: &Value, cfg: &ToolOutputConfig, depth: usize) -> Value {
    const MAX_DEPTH: usize = 6;
    if depth >= MAX_DEPTH {
        return Value::String("…".into());
    }
    match v {
        Value::String(s) if s.len() > cfg.max_string => {
            let head: String = s.chars().take(cfg.max_string).collect();
            Value::String(format!("{head}…(+{} chars)", s.len() - head.len()))
        }
        Value::Array(items) => {
            let keep = cfg.max_array.min(items.len());
            let mut out: Vec<Value> = items
                .iter()
                .take(keep)
                .map(|it| crush_json(it, cfg, depth + 1))
                .collect();
            if items.len() > keep {
                out.push(Value::String(format!("…(+{} more items)", items.len() - keep)));
            }
            Value::Array(out)
        }
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (k, val) in map {
                out.insert(k.clone(), crush_json(val, cfg, depth + 1));
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

/// Head + tail with the middle elided. The signal in a log/stack trace/build
/// output lives at the ends. Two regimes: many lines → elide by *line*; few
/// lines (e.g. one giant minified/base64 blob) → elide by *char*.
fn crush_text(content: &str, cfg: &ToolOutputConfig) -> String {
    // Char budgets for the single-blob fallback.
    const HEAD_CHARS: usize = 800;
    const TAIL_CHARS: usize = 400;

    let lines: Vec<&str> = content.lines().collect();
    if lines.len() > cfg.head_lines + cfg.tail_lines + 1 {
        // Line regime. Bound each kept line too, so an all-long-lines log can't
        // pass the bulk through in the head/tail.
        let clip = |l: &&str| -> String {
            if l.len() > cfg.max_string {
                let head: String = l.chars().take(cfg.max_string).collect();
                format!("{head}…(+{} chars)", l.len() - head.len())
            } else {
                (*l).to_string()
            }
        };
        let head: Vec<String> = lines[..cfg.head_lines].iter().map(clip).collect();
        let tail: Vec<String> = lines[lines.len() - cfg.tail_lines..].iter().map(clip).collect();
        let elided_lines = lines.len() - cfg.head_lines - cfg.tail_lines;
        let elided_chars: usize = lines[cfg.head_lines..lines.len() - cfg.tail_lines]
            .iter()
            .map(|l| l.len() + 1)
            .sum();
        let mut out = String::with_capacity(content.len() / 2);
        out.push_str(&head.join("\n"));
        out.push_str(&format!("\n\n[… {elided_lines} lines / {elided_chars} chars elided …]\n\n"));
        out.push_str(&tail.join("\n"));
        return out;
    }

    // Few lines: char regime for a long single blob.
    let chars: Vec<char> = content.chars().collect();
    if chars.len() <= HEAD_CHARS + TAIL_CHARS + 64 {
        return content.to_string(); // nothing worth eliding
    }
    let head: String = chars[..HEAD_CHARS].iter().collect();
    let tail: String = chars[chars.len() - TAIL_CHARS..].iter().collect();
    let elided = chars.len() - HEAD_CHARS - TAIL_CHARS;
    format!("{head}\n\n[… {elided} chars elided …]\n\n{tail}")
}

/// Answer a `recall({"id": "<id>"})` call: the full original tool output stashed
/// when it was compacted, if still resident. Mirrors `expand_tool` for schemas.
pub fn recall_output(id: &str) -> Option<String> {
    turn_stash::get(id)
}

/// Rough token estimate helper re-exported for the report/panel.
pub fn tokens(s: &str) -> usize {
    approx_tokens(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn on() -> ToolOutputConfig {
        ToolOutputConfig { enabled: true, ..Default::default() }
    }

    #[test]
    fn small_output_untouched() {
        let cfg = on();
        assert!(crush("short and sweet", &cfg).is_none());
    }

    #[test]
    fn json_array_is_capped() {
        let cfg = on();
        let items: Vec<Value> = (0..100).map(|i| json!({"n": i, "s": "x".repeat(300)})).collect();
        let blob = serde_json::to_string(&json!({"results": items})).unwrap();
        let out = crush(&blob, &cfg).expect("should crush");
        assert!(out.len() * 2 < blob.len(), "should be much smaller");
        assert!(out.contains("more items"), "array cap marker present");
        assert!(out.contains("chars)"), "long string truncated");
        assert!(out.contains("results"), "keys preserved");
    }

    #[test]
    fn invalid_json_falls_back_to_text() {
        let cfg = ToolOutputConfig { head_lines: 2, tail_lines: 1, ..on() };
        // starts with '{' but isn't valid JSON
        let content = format!("{{not json\n{}", "line\n".repeat(50));
        let out = crush(&content, &cfg).expect("crushes as text");
        assert!(out.contains("elided"));
    }

    #[test]
    fn log_keeps_head_and_tail() {
        let cfg = ToolOutputConfig { head_lines: 3, tail_lines: 2, min_chars: 10, ..on() };
        let mut s = String::new();
        for i in 0..200 {
            s.push_str(&format!("log line number {i}\n"));
        }
        let out = crush(&s, &cfg).expect("crushes");
        assert!(out.contains("log line number 0"));
        assert!(out.contains("log line number 1"));
        assert!(out.contains("log line number 199"));
        assert!(out.contains("elided"));
        assert!(!out.contains("log line number 100"));
    }

    #[test]
    fn compress_messages_stashes_and_marks() {
        let _g = turn_stash::test_lock();
        turn_stash::clear();
        let big = "y".repeat(4000);
        let mut body = json!({
            "messages": [
                {"role": "user", "content": "go"},
                {"role": "tool", "content": big.clone()},
                {"role": "assistant", "content": "ok"},
            ]
        });
        let cfg = on();
        let r = compress_tool_messages(&mut body, &cfg);
        assert_eq!(r.messages_scanned, 1);
        assert_eq!(r.messages_compacted, 1);
        assert!(r.approx_tokens_saved() > 0);

        let tool_content = body["messages"][1]["content"].as_str().unwrap();
        assert!(tool_content.contains(MARK));
        assert!(tool_content.contains("recall("));
        // the stashed id is recallable
        let id = tool_content
            .split("\"id\": \"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .unwrap();
        assert_eq!(recall_output(id).as_deref(), Some(big.as_str()));
    }

    #[test]
    fn idempotent_on_resend() {
        let _g = turn_stash::test_lock();
        turn_stash::clear();
        let mut body = json!({
            "messages": [{"role": "tool", "content": "z".repeat(4000)}]
        });
        let cfg = on();
        compress_tool_messages(&mut body, &cfg);
        let once = body.clone();
        let r2 = compress_tool_messages(&mut body, &cfg);
        assert_eq!(r2.messages_compacted, 0, "second pass is a no-op");
        assert_eq!(body, once);
    }

    #[test]
    fn disabled_is_noop() {
        let mut body = json!({"messages": [{"role": "tool", "content": "q".repeat(4000)}]});
        let before = body.clone();
        let r = compress_tool_messages(&mut body, &ToolOutputConfig::default());
        assert_eq!(r.messages_compacted, 0);
        assert_eq!(body, before);
    }

    #[test]
    fn non_recallable_omits_id() {
        let _g = turn_stash::test_lock();
        turn_stash::clear();
        let mut body = json!({"messages": [{"role": "tool", "content": "w".repeat(4000)}]});
        let cfg = ToolOutputConfig { recallable: false, ..on() };
        compress_tool_messages(&mut body, &cfg);
        let c = body["messages"][0]["content"].as_str().unwrap();
        assert!(c.contains(MARK));
        assert!(!c.contains("recall("));
    }
}
