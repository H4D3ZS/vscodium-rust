//! Kortex harness — deterministic agent-request compression.
//!
//! Sits in front of a local llama-server and rewrites an OpenAI-style chat
//! request so a large agent harness (Claude Code, the built-in loop) fits a
//! small `n_ctx`:
//!
//!   * tool schemas (~75% of Claude Code's ~28k boot payload) → a Hermes-style
//!     compact signature block (`tool_digest`),
//!   * a fixed calling contract + optional GBNF grammar for reliable tool calls
//!     on a 3B-active model (`contract`),
//!   * a synthetic `expand` tool that rehydrates one full schema on demand —
//!     the "context lives compressed and expands when needed" model.
//!
//! No embeddings, no retrieval, no model calls. Pure structure walking, behind
//! an explicit opt-in (`KORTEX_HARNESS=1`) so the default proxy path is
//! byte-for-byte unchanged.

pub mod contract;
pub mod tool_digest;

use serde_json::{json, Value};
use tool_digest::{approx_tokens, digest_tools, ToolDigest};

/// Tools always kept as real, natively-callable entries (never compacted away).
/// Everything else moves to the signature block until `expand`ed.
const DEFAULT_CORE_TOOLS: &[&str] = &[
    "read_file", "read", "write_file", "write", "edit_file", "edit",
    "bash", "run_terminal_cmd", "grep", "glob", "list_dir", "codebase_search",
];

#[derive(Debug, Clone)]
pub struct HarnessConfig {
    pub enabled: bool,
    /// Names kept inline as live `tools` entries.
    pub core_tools: Vec<String>,
    /// Also keep the first N non-core tools inline (cheap headroom before we
    /// force everything through `expand`). 0 = only the core set.
    pub keep_extra_inline: usize,
    /// Emit a GBNF grammar into the request (`grammar` field) so a small model
    /// can't produce a malformed tool call.
    pub constrain_grammar: bool,
}

impl Default for HarnessConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            core_tools: DEFAULT_CORE_TOOLS.iter().map(|s| s.to_string()).collect(),
            keep_extra_inline: 3,
            constrain_grammar: false,
        }
    }
}

impl HarnessConfig {
    pub fn from_env() -> Self {
        let on = matches!(
            std::env::var("KORTEX_HARNESS").ok().as_deref(),
            Some("1") | Some("true") | Some("on")
        );
        Self {
            enabled: on,
            constrain_grammar: matches!(
                std::env::var("KORTEX_HARNESS_GRAMMAR").ok().as_deref(),
                Some("1") | Some("true") | Some("on")
            ),
            ..Self::default()
        }
    }
}

/// What the rewrite did, for logging / the stats panel.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct HarnessReport {
    pub applied: bool,
    pub tools_in: usize,
    pub tools_inline_out: usize,
    pub tools_compacted: usize,
    pub approx_tokens_before: usize,
    pub approx_tokens_after: usize,
}

impl HarnessReport {
    pub fn saved(&self) -> usize {
        self.approx_tokens_before
            .saturating_sub(self.approx_tokens_after)
    }
}

/// Rewrite an OpenAI `/v1/chat/completions` body in place. Idempotent and
/// defensive: any unexpected shape leaves `body` untouched and returns
/// `applied = false`.
pub fn compress_openai_request(body: &mut Value, cfg: &HarnessConfig) -> HarnessReport {
    let mut report = HarnessReport::default();
    if !cfg.enabled {
        return report;
    }
    let Some(obj) = body.as_object_mut() else {
        return report;
    };

    // Already processed? (our marker survives a resend of the same turn.)
    if obj.get("_kortex_harness").is_some() {
        report.applied = true;
        return report;
    }

    let tools = match obj.get("tools").and_then(Value::as_array) {
        Some(t) if t.len() > 4 => t.clone(),
        _ => return report, // nothing worth compressing
    };
    report.tools_in = tools.len();
    report.approx_tokens_before = approx_tokens(&serde_json::to_string(&tools).unwrap_or_default());

    let digests: Vec<ToolDigest> = digest_tools(&tools);

    let is_core = |name: &str| cfg.core_tools.iter().any(|c| c == name);
    let mut inline: Vec<Value> = Vec::new();
    let mut compacted: Vec<&ToolDigest> = Vec::new();
    let mut extra_kept = 0usize;

    for d in &digests {
        if is_core(&d.name) {
            inline.push(d.full_schema.clone());
        } else if extra_kept < cfg.keep_extra_inline {
            inline.push(d.full_schema.clone());
            extra_kept += 1;
        } else {
            compacted.push(d);
        }
    }

    if compacted.is_empty() {
        return report; // everything fit in the core/extra budget already
    }

    // Build the compact signature block.
    let mut block = String::from(contract::CONTRACT);
    block.push_str("\n## Compact tool signatures\n\n");
    for d in &compacted {
        block.push_str("  ");
        block.push_str(&d.signature);
        block.push('\n');
    }

    // `expand` is how a compacted tool gets its full schema back.
    inline.push(contract::expand_tool_def());

    // Stash the full schemas so a later `expand` call (handled by the proxy or
    // the agent) can rehydrate without another round-trip to the client.
    let schema_map: serde_json::Map<String, Value> = compacted
        .iter()
        .map(|d| (d.name.clone(), d.full_schema.clone()))
        .collect();

    prepend_system_text(obj, &block);
    obj.insert("tools".into(), Value::Array(inline.clone()));
    obj.insert("_kortex_harness".into(), json!({ "compacted_schemas": schema_map }));

    if cfg.constrain_grammar {
        let names: Vec<&str> = inline
            .iter()
            .filter_map(|t| t.get("function").and_then(|f| f.get("name")).and_then(Value::as_str))
            .collect();
        obj.entry("grammar")
            .or_insert_with(|| Value::String(contract::tool_call_grammar(&names)));
    }

    report.applied = true;
    report.tools_inline_out = inline.len();
    report.tools_compacted = compacted.len();
    report.approx_tokens_after = approx_tokens(&serde_json::to_string(&inline).unwrap_or_default())
        + approx_tokens(&block);
    report
}

/// Look up a full schema previously stashed by `compress_openai_request`, so an
/// `expand({"tool": name})` call can be answered locally.
pub fn rehydrate<'a>(body: &'a Value, tool: &str) -> Option<&'a Value> {
    body.get("_kortex_harness")?
        .get("compacted_schemas")?
        .get(tool)
}

/// Prepend text to the first `system` message, or insert one if absent.
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

    fn tool(name: &str) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": name,
                "description": format!("Does the {name} thing in great detail with many caveats and notes."),
                "parameters": {
                    "type": "object",
                    "properties": { "x": {"type": "string"}, "y": {"type": "integer"} },
                    "required": ["x"]
                }
            }
        })
    }

    fn req_with(n: usize) -> Value {
        let tools: Vec<Value> = (0..n).map(|i| tool(&format!("tool_{i}"))).collect();
        json!({
            "model": "local",
            "messages": [{"role": "system", "content": "You are an agent."},
                         {"role": "user", "content": "go"}],
            "tools": tools
        })
    }

    #[test]
    fn disabled_is_noop() {
        let mut b = req_with(20);
        let before = b.clone();
        let r = compress_openai_request(&mut b, &HarnessConfig::default());
        assert!(!r.applied);
        assert_eq!(b, before);
    }

    #[test]
    fn compacts_non_core_tools() {
        let mut b = req_with(20);
        let cfg = HarnessConfig { enabled: true, ..Default::default() };
        let r = compress_openai_request(&mut b, &cfg);
        assert!(r.applied);
        assert_eq!(r.tools_in, 20);
        // 3 extra kept inline + the synthetic `expand`
        assert_eq!(r.tools_inline_out, 4);
        assert_eq!(r.tools_compacted, 17);
        assert!(r.saved() > 0);
        // system message now carries the contract + signatures
        let sys = b["messages"][0]["content"].as_str().unwrap();
        assert!(sys.contains("<tool_call>"));
        assert!(sys.contains("tool_19("));
        assert!(sys.contains("You are an agent."));
    }

    #[test]
    fn keeps_core_tools_inline() {
        let mut b = json!({
            "messages": [{"role":"user","content":"go"}],
            "tools": [tool("read_file"), tool("edit_file"), tool("bash"),
                      tool("weird_a"), tool("weird_b"), tool("weird_c"),
                      tool("weird_d"), tool("weird_e")]
        });
        let cfg = HarnessConfig { enabled: true, keep_extra_inline: 0, ..Default::default() };
        let r = compress_openai_request(&mut b, &cfg);
        let names: Vec<&str> = b["tools"].as_array().unwrap().iter()
            .filter_map(|t| t["function"]["name"].as_str()).collect();
        assert!(names.contains(&"read_file"));
        assert!(names.contains(&"bash"));
        assert!(names.contains(&"expand"));
        assert!(!names.contains(&"weird_a"));
        assert_eq!(r.tools_compacted, 5);
    }

    #[test]
    fn idempotent() {
        let mut b = req_with(20);
        let cfg = HarnessConfig { enabled: true, ..Default::default() };
        compress_openai_request(&mut b, &cfg);
        let once = b.clone();
        compress_openai_request(&mut b, &cfg);
        assert_eq!(b, once);
    }

    #[test]
    fn rehydrate_finds_stashed_schema() {
        let mut b = req_with(20);
        let cfg = HarnessConfig { enabled: true, ..Default::default() };
        compress_openai_request(&mut b, &cfg);
        let s = rehydrate(&b, "tool_15").expect("stashed");
        assert_eq!(s["function"]["name"], "tool_15");
    }

    #[test]
    fn small_tool_arrays_untouched() {
        let mut b = req_with(3);
        let cfg = HarnessConfig { enabled: true, ..Default::default() };
        let r = compress_openai_request(&mut b, &cfg);
        assert!(!r.applied);
    }
}
