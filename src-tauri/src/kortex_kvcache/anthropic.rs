//! Anthropic Messages API ⇄ OpenAI chat-completions translation (plan §2.3).
//!
//! Claude Code talks to `ANTHROPIC_BASE_URL/v1/messages`. The proxy only
//! specials `/v1/chat/completions` and `/v1/completions`, so without this the
//! harness compressor and the KV cache never see Claude Code traffic.
//!
//! `handle_messages` in `proxy.rs`:
//!   1. `anthropic_to_openai` — request translation.
//!   2. run it through the normal `/v1/chat/completions` path (KV cache +
//!      harness apply here), forcing `stream:false` upstream.
//!   3. `openai_response_to_anthropic` — response translation.
//!   4. if the client asked for a stream, `anthropic_message_to_sse` renders the
//!      Anthropic event sequence from the finished message (one burst of
//!      well-formed events — we don't trickle tokens; same trade-off as the
//!      Tier 2 SSE replay).
//!
//! **Scope.** What Claude Code actually sends: system prompt (string or text
//! blocks), text / `tool_use` / `tool_result` content blocks, `tools` with
//! `input_schema`, `tool_choice`, `stop_sequences`, sampling params. Not
//! covered: image blocks (dropped with a marker), extended-thinking blocks
//! (passed through as text), fine-grained streaming timing.

use anyhow::{anyhow, Result};
use serde_json::{json, Map, Value};

// ─────────────────────────── request: Anthropic → OpenAI ────────────────────

/// Translate an Anthropic `/v1/messages` request body into an OpenAI
/// `/v1/chat/completions` body. Errors only on structurally invalid input
/// (e.g. `messages` not an array).
pub fn anthropic_to_openai(a: &Value) -> Result<Value> {
    let obj = a.as_object().ok_or_else(|| anyhow!("request body is not a JSON object"))?;

    let mut messages: Vec<Value> = Vec::new();

    // system: string | [{type:"text", text, cache_control?}]
    if let Some(sys) = obj.get("system") {
        if let Some(text) = flatten_text(sys) {
            if !text.is_empty() {
                messages.push(json!({ "role": "system", "content": text }));
            }
        }
    }

    for m in obj.get("messages").and_then(Value::as_array).into_iter().flatten() {
        let role = m.get("role").and_then(Value::as_str).unwrap_or("user");
        let content = m.get("content");

        // Simple string content.
        if let Some(s) = content.and_then(Value::as_str) {
            messages.push(json!({ "role": role, "content": s }));
            continue;
        }

        let blocks = match content.and_then(Value::as_array) {
            Some(b) => b,
            None => {
                messages.push(json!({ "role": role, "content": "" }));
                continue;
            }
        };

        // A user message carrying tool_result block(s) becomes one OpenAI
        // `tool` message per result (OpenAI has no multi-result role).
        let mut text_parts: Vec<String> = Vec::new();
        let mut tool_calls: Vec<Value> = Vec::new();
        let mut emitted_tool_msg = false;

        for b in blocks {
            match b.get("type").and_then(Value::as_str).unwrap_or("") {
                "text" => {
                    if let Some(t) = b.get("text").and_then(Value::as_str) {
                        text_parts.push(t.to_string());
                    }
                }
                "thinking" | "redacted_thinking" => {
                    // Keep the reasoning visible as plain text rather than drop it.
                    if let Some(t) = b.get("thinking").and_then(Value::as_str) {
                        text_parts.push(t.to_string());
                    }
                }
                "tool_use" => {
                    let id = b.get("id").and_then(Value::as_str).unwrap_or("");
                    let name = b.get("name").and_then(Value::as_str).unwrap_or("");
                    let input = b.get("input").cloned().unwrap_or_else(|| json!({}));
                    tool_calls.push(json!({
                        "id": id,
                        "type": "function",
                        "function": {
                            "name": name,
                            "arguments": serde_json::to_string(&input).unwrap_or_else(|_| "{}".into()),
                        }
                    }));
                }
                "tool_result" => {
                    let tid = b.get("tool_use_id").and_then(Value::as_str).unwrap_or("");
                    let body = flatten_text(b.get("content").unwrap_or(&Value::Null))
                        .unwrap_or_default();
                    let body = if b.get("is_error").and_then(Value::as_bool).unwrap_or(false) {
                        format!("[tool error] {body}")
                    } else {
                        body
                    };
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": tid,
                        "content": body,
                    }));
                    emitted_tool_msg = true;
                }
                "image" => {
                    text_parts.push("[image omitted — not supported over the kortex bridge]".into());
                }
                other => {
                    tracing::debug!("[anthropic] ignoring unknown content block '{other}'");
                }
            }
        }

        let joined = text_parts.join("\n");
        if !tool_calls.is_empty() {
            let mut msg = Map::new();
            msg.insert("role".into(), json!("assistant"));
            msg.insert(
                "content".into(),
                if joined.is_empty() { Value::Null } else { json!(joined) },
            );
            msg.insert("tool_calls".into(), Value::Array(tool_calls));
            messages.push(Value::Object(msg));
        } else if !joined.is_empty() || !emitted_tool_msg {
            // Plain text turn (or an empty turn we still need to represent).
            messages.push(json!({ "role": role, "content": joined }));
        }
    }

    let mut out = Map::new();
    if let Some(model) = obj.get("model") {
        out.insert("model".into(), model.clone());
    }
    out.insert("messages".into(), Value::Array(messages));

    // Sampling / control params.
    copy_if_present(obj, &mut out, "temperature", "temperature");
    copy_if_present(obj, &mut out, "top_p", "top_p");
    copy_if_present(obj, &mut out, "top_k", "top_k"); // llama.cpp honours it
    copy_if_present(obj, &mut out, "max_tokens", "max_tokens");
    copy_if_present(obj, &mut out, "stream", "stream");
    if let Some(stop) = obj.get("stop_sequences") {
        out.insert("stop".into(), stop.clone());
    }
    if let Some(uid) = obj.get("metadata").and_then(|m| m.get("user_id")) {
        out.insert("user".into(), uid.clone());
    }

    if let Some(tools) = obj.get("tools").and_then(Value::as_array) {
        let mapped: Vec<Value> = tools
            .iter()
            .filter_map(|t| {
                let name = t.get("name").and_then(Value::as_str)?;
                let mut f = Map::new();
                f.insert("name".into(), json!(name));
                if let Some(d) = t.get("description") {
                    f.insert("description".into(), d.clone());
                }
                f.insert(
                    "parameters".into(),
                    t.get("input_schema").cloned().unwrap_or_else(|| json!({"type": "object"})),
                );
                Some(json!({ "type": "function", "function": Value::Object(f) }))
            })
            .collect();
        if !mapped.is_empty() {
            out.insert("tools".into(), Value::Array(mapped));
        }
    }
    if let Some(tc) = obj.get("tool_choice") {
        out.insert("tool_choice".into(), translate_tool_choice(tc));
    }

    Ok(Value::Object(out))
}

fn translate_tool_choice(tc: &Value) -> Value {
    match tc.get("type").and_then(Value::as_str) {
        Some("auto") => json!("auto"),
        Some("any") => json!("required"),
        Some("tool") => {
            let name = tc.get("name").and_then(Value::as_str).unwrap_or("");
            json!({ "type": "function", "function": { "name": name } })
        }
        _ => json!("auto"),
    }
}

/// Anthropic text can be a bare string or an array of `{type, text|content}`
/// blocks. Flatten to a single string; return `None` only for `null`.
fn flatten_text(v: &Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::String(s) => Some(s.clone()),
        Value::Array(arr) => {
            let mut parts = Vec::new();
            for b in arr {
                if let Some(s) = b.as_str() {
                    parts.push(s.to_string());
                } else if let Some(t) = b.get("text").and_then(Value::as_str) {
                    parts.push(t.to_string());
                } else if let Some(inner) = b.get("content") {
                    if let Some(s) = flatten_text(inner) {
                        parts.push(s);
                    }
                }
            }
            Some(parts.join("\n"))
        }
        other => Some(other.to_string()),
    }
}

fn copy_if_present(src: &Map<String, Value>, dst: &mut Map<String, Value>, from: &str, to: &str) {
    if let Some(v) = src.get(from) {
        if !v.is_null() {
            dst.insert(to.into(), v.clone());
        }
    }
}

// ─────────────────────────── response: OpenAI → Anthropic ───────────────────

/// Translate a non-streaming OpenAI chat-completion response into an Anthropic
/// message object.
pub fn openai_response_to_anthropic(o: &Value) -> Value {
    let choice = o
        .get("choices")
        .and_then(Value::as_array)
        .and_then(|a| a.first())
        .cloned()
        .unwrap_or_else(|| json!({}));
    let msg = choice.get("message").cloned().unwrap_or_else(|| json!({}));

    let mut content: Vec<Value> = Vec::new();
    if let Some(text) = msg.get("content").and_then(Value::as_str) {
        if !text.is_empty() {
            content.push(json!({ "type": "text", "text": text }));
        }
    }
    for tc in msg.get("tool_calls").and_then(Value::as_array).into_iter().flatten() {
        let f = tc.get("function").cloned().unwrap_or_else(|| json!({}));
        let name = f.get("name").and_then(Value::as_str).unwrap_or("");
        let args_str = f.get("arguments").and_then(Value::as_str).unwrap_or("{}");
        let input: Value = serde_json::from_str(args_str).unwrap_or_else(|_| json!({}));
        content.push(json!({
            "type": "tool_use",
            "id": tc.get("id").and_then(Value::as_str).unwrap_or(""),
            "name": name,
            "input": input,
        }));
    }
    if content.is_empty() {
        content.push(json!({ "type": "text", "text": "" }));
    }

    let stop_reason = match choice.get("finish_reason").and_then(Value::as_str) {
        Some("length") => "max_tokens",
        Some("tool_calls") => "tool_use",
        Some("content_filter") => "stop_sequence",
        _ => "end_turn",
    };

    let usage = o.get("usage").cloned().unwrap_or_else(|| json!({}));
    let input_tokens = usage.get("prompt_tokens").and_then(Value::as_u64).unwrap_or(0);
    let output_tokens = usage.get("completion_tokens").and_then(Value::as_u64).unwrap_or(0);

    json!({
        "id": o.get("id").and_then(Value::as_str).unwrap_or("msg_kortex"),
        "type": "message",
        "role": "assistant",
        "model": o.get("model").and_then(Value::as_str).unwrap_or(""),
        "content": content,
        "stop_reason": stop_reason,
        "stop_sequence": Value::Null,
        "usage": { "input_tokens": input_tokens, "output_tokens": output_tokens },
    })
}

/// Render a finished Anthropic message object as the SSE event sequence a
/// streaming client expects. Not token-timed — every content block is emitted
/// whole — but every event is well-formed and ordered, so a client parser is
/// satisfied.
pub fn anthropic_message_to_sse(msg: &Value) -> String {
    let mut out = String::new();
    let mut ev = |event: &str, data: Value, out: &mut String| {
        out.push_str("event: ");
        out.push_str(event);
        out.push_str("\ndata: ");
        out.push_str(&data.to_string());
        out.push_str("\n\n");
    };

    let empty = Vec::new();
    let blocks = msg.get("content").and_then(Value::as_array).unwrap_or(&empty);

    let start_msg = json!({
        "type": "message_start",
        "message": {
            "id": msg.get("id").cloned().unwrap_or_else(|| json!("msg_kortex")),
            "type": "message",
            "role": "assistant",
            "model": msg.get("model").cloned().unwrap_or_else(|| json!("")),
            "content": [],
            "stop_reason": Value::Null,
            "stop_sequence": Value::Null,
            "usage": msg.get("usage").cloned().unwrap_or_else(|| json!({"input_tokens": 0, "output_tokens": 0})),
        }
    });
    ev("message_start", start_msg, &mut out);

    for (i, block) in blocks.iter().enumerate() {
        match block.get("type").and_then(Value::as_str) {
            Some("tool_use") => {
                ev(
                    "content_block_start",
                    json!({
                        "type": "content_block_start",
                        "index": i,
                        "content_block": {
                            "type": "tool_use",
                            "id": block.get("id").cloned().unwrap_or_else(|| json!("")),
                            "name": block.get("name").cloned().unwrap_or_else(|| json!("")),
                            "input": {},
                        }
                    }),
                    &mut out,
                );
                let partial = serde_json::to_string(
                    block.get("input").unwrap_or(&json!({})),
                )
                .unwrap_or_else(|_| "{}".into());
                ev(
                    "content_block_delta",
                    json!({
                        "type": "content_block_delta",
                        "index": i,
                        "delta": { "type": "input_json_delta", "partial_json": partial }
                    }),
                    &mut out,
                );
            }
            _ => {
                ev(
                    "content_block_start",
                    json!({
                        "type": "content_block_start",
                        "index": i,
                        "content_block": { "type": "text", "text": "" }
                    }),
                    &mut out,
                );
                ev(
                    "content_block_delta",
                    json!({
                        "type": "content_block_delta",
                        "index": i,
                        "delta": {
                            "type": "text_delta",
                            "text": block.get("text").and_then(Value::as_str).unwrap_or(""),
                        }
                    }),
                    &mut out,
                );
            }
        }
        ev(
            "content_block_stop",
            json!({ "type": "content_block_stop", "index": i }),
            &mut out,
        );
    }

    ev(
        "message_delta",
        json!({
            "type": "message_delta",
            "delta": {
                "stop_reason": msg.get("stop_reason").cloned().unwrap_or_else(|| json!("end_turn")),
                "stop_sequence": Value::Null,
            },
            "usage": { "output_tokens": msg.get("usage").and_then(|u| u.get("output_tokens")).cloned().unwrap_or_else(|| json!(0)) }
        }),
        &mut out,
    );
    ev("message_stop", json!({ "type": "message_stop" }), &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Synthetic — shaped from the public Anthropic Messages API docs, not a
    // live Claude Code capture. Replace with a real capture when one exists
    // (plan §2.3 "Done when").
    fn cc_request() -> Value {
        json!({
            "model": "claude-sonnet-4-5",
            "max_tokens": 1024,
            "temperature": 0,
            "system": [
                { "type": "text", "text": "You are a coding agent." },
                { "type": "text", "text": "Be terse.", "cache_control": { "type": "ephemeral" } }
            ],
            "tools": [
                { "name": "read_file",
                  "description": "Read a file",
                  "input_schema": { "type": "object", "properties": { "path": { "type": "string" } }, "required": ["path"] } }
            ],
            "tool_choice": { "type": "auto" },
            "stop_sequences": ["\n\nHuman:"],
            "stream": true,
            "messages": [
                { "role": "user", "content": "read config.toml" },
                { "role": "assistant", "content": [
                    { "type": "text", "text": "on it" },
                    { "type": "tool_use", "id": "toolu_01", "name": "read_file", "input": { "path": "config.toml" } }
                ]},
                { "role": "user", "content": [
                    { "type": "tool_result", "tool_use_id": "toolu_01", "content": "port = 8080" }
                ]}
            ]
        })
    }

    #[test]
    fn request_system_blocks_become_a_system_message() {
        let o = anthropic_to_openai(&cc_request()).unwrap();
        let msgs = o["messages"].as_array().unwrap();
        assert_eq!(msgs[0]["role"], "system");
        assert_eq!(msgs[0]["content"], "You are a coding agent.\nBe terse.");
    }

    #[test]
    fn request_tool_use_and_result_round_trip_to_openai_shape() {
        let o = anthropic_to_openai(&cc_request()).unwrap();
        let msgs = o["messages"].as_array().unwrap();
        // system, user, assistant(tool_calls), tool
        assert_eq!(msgs.len(), 4);
        assert_eq!(msgs[1]["role"], "user");
        assert_eq!(msgs[1]["content"], "read config.toml");

        let asst = &msgs[2];
        assert_eq!(asst["role"], "assistant");
        assert_eq!(asst["content"], "on it");
        let call = &asst["tool_calls"][0];
        assert_eq!(call["id"], "toolu_01");
        assert_eq!(call["type"], "function");
        assert_eq!(call["function"]["name"], "read_file");
        assert_eq!(call["function"]["arguments"], "{\"path\":\"config.toml\"}");

        let tool = &msgs[3];
        assert_eq!(tool["role"], "tool");
        assert_eq!(tool["tool_call_id"], "toolu_01");
        assert_eq!(tool["content"], "port = 8080");
    }

    #[test]
    fn request_tools_and_params_translate() {
        let o = anthropic_to_openai(&cc_request()).unwrap();
        assert_eq!(o["model"], "claude-sonnet-4-5");
        assert_eq!(o["max_tokens"], 1024);
        assert_eq!(o["temperature"], 0);
        assert_eq!(o["stream"], true);
        assert_eq!(o["stop"][0], "\n\nHuman:");
        assert_eq!(o["tool_choice"], "auto");
        let t = &o["tools"][0];
        assert_eq!(t["type"], "function");
        assert_eq!(t["function"]["name"], "read_file");
        assert_eq!(t["function"]["parameters"]["required"][0], "path");
    }

    #[test]
    fn tool_choice_any_becomes_required() {
        let mut r = cc_request();
        r["tool_choice"] = json!({ "type": "any" });
        let o = anthropic_to_openai(&r).unwrap();
        assert_eq!(o["tool_choice"], "required");

        r["tool_choice"] = json!({ "type": "tool", "name": "read_file" });
        let o = anthropic_to_openai(&r).unwrap();
        assert_eq!(o["tool_choice"]["function"]["name"], "read_file");
    }

    #[test]
    fn response_text_and_tool_calls_become_content_blocks() {
        let openai = json!({
            "id": "chatcmpl-9",
            "model": "escha",
            "choices": [{
                "index": 0,
                "finish_reason": "tool_calls",
                "message": {
                    "role": "assistant",
                    "content": "let me check",
                    "tool_calls": [{
                        "id": "call_1", "type": "function",
                        "function": { "name": "read_file", "arguments": "{\"path\":\"a.txt\"}" }
                    }]
                }
            }],
            "usage": { "prompt_tokens": 12, "completion_tokens": 7 }
        });
        let a = openai_response_to_anthropic(&openai);
        assert_eq!(a["type"], "message");
        assert_eq!(a["role"], "assistant");
        assert_eq!(a["stop_reason"], "tool_use");
        assert_eq!(a["content"][0]["type"], "text");
        assert_eq!(a["content"][0]["text"], "let me check");
        assert_eq!(a["content"][1]["type"], "tool_use");
        assert_eq!(a["content"][1]["name"], "read_file");
        assert_eq!(a["content"][1]["input"]["path"], "a.txt");
        assert_eq!(a["usage"]["input_tokens"], 12);
        assert_eq!(a["usage"]["output_tokens"], 7);
    }

    #[test]
    fn finish_reason_maps() {
        let mk = |fr: &str| {
            json!({ "choices": [{ "finish_reason": fr, "message": { "content": "x" } }] })
        };
        assert_eq!(openai_response_to_anthropic(&mk("stop"))["stop_reason"], "end_turn");
        assert_eq!(openai_response_to_anthropic(&mk("length"))["stop_reason"], "max_tokens");
        assert_eq!(openai_response_to_anthropic(&mk("tool_calls"))["stop_reason"], "tool_use");
    }

    #[test]
    fn sse_render_is_well_formed_and_ordered() {
        let msg = openai_response_to_anthropic(&json!({
            "id": "chatcmpl-x", "model": "escha",
            "choices": [{ "finish_reason": "stop", "message": { "content": "hello world" } }],
            "usage": { "prompt_tokens": 3, "completion_tokens": 2 }
        }));
        let sse = anthropic_message_to_sse(&msg);
        let events: Vec<&str> = sse
            .lines()
            .filter_map(|l| l.strip_prefix("event: "))
            .collect();
        assert_eq!(
            events,
            vec![
                "message_start",
                "content_block_start",
                "content_block_delta",
                "content_block_stop",
                "message_delta",
                "message_stop",
            ]
        );
        // every data line parses as JSON
        for d in sse.lines().filter_map(|l| l.strip_prefix("data: ")) {
            serde_json::from_str::<Value>(d).unwrap();
        }
        assert!(sse.contains("\"text_delta\""));
        assert!(sse.contains("hello world"));
    }

    #[test]
    fn sse_tool_use_block_uses_input_json_delta() {
        let msg = openai_response_to_anthropic(&json!({
            "choices": [{
                "finish_reason": "tool_calls",
                "message": { "content": null, "tool_calls": [{
                    "id": "call_9", "type": "function",
                    "function": { "name": "grep", "arguments": "{\"q\":\"fn main\"}" }
                }]}
            }]
        }));
        let sse = anthropic_message_to_sse(&msg);
        assert!(sse.contains("\"input_json_delta\""));
        assert!(sse.contains("partial_json"));
        assert!(sse.contains("fn main"));
    }

    #[test]
    fn plain_string_messages_still_work() {
        let r = json!({
            "model": "m", "max_tokens": 10,
            "messages": [{ "role": "user", "content": "hi" }]
        });
        let o = anthropic_to_openai(&r).unwrap();
        assert_eq!(o["messages"][0]["role"], "user");
        assert_eq!(o["messages"][0]["content"], "hi");
        assert!(o.get("tools").is_none());
    }
}
