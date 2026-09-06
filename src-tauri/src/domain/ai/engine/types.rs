//! Chat message / request / response types and provider-agnostic helpers.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

impl Default for MessageContent {
    fn default() -> Self {
        MessageContent::Text(String::new())
    }
}

impl MessageContent {
    pub fn as_str(&self) -> &str {
        match self {
            MessageContent::Text(s) => s,
            MessageContent::Parts(parts) => {
                for part in parts {
                    if let ContentPart::Text { text } = part {
                        return text;
                    }
                }
                ""
            }
        }
    }

    pub fn to_text(&self) -> String {
        match self {
            MessageContent::Text(s) => s.clone(),
            MessageContent::Parts(parts) => {
                let mut text = String::new();
                for part in parts {
                    if let ContentPart::Text { text: t } = part {
                        text.push_str(t);
                    }
                }
                text
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrlPart },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrlPart {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ChatMessage {
    pub role: String,
    pub content: Option<MessageContent>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub function: ToolFunction,
    pub context: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiRequest {
    pub provider: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub autonomous: bool,
    pub mode: Option<String>,
    pub cyber_mode: Option<bool>,
    pub root_access: Option<bool>,
    /// Base URL of the local inference server (Lemonade / llama-server / the
    /// Kortex proxy). `local backend_url` is still accepted on the wire for back-compat
    /// with any persisted request payloads.
    #[serde(alias = "ollama_url")]
    pub inference_url: Option<String>,
    pub tools: Option<Vec<Value>>,
    /// Anthropic/Gemini extended thinking: budget_tokens (e.g. 8000)
    #[serde(default)]
    pub reasoning_budget: Option<u32>,
    /// OpenAI/xAI reasoning_effort: "low" | "medium" | "high"
    #[serde(default)]
    pub reasoning_effort: Option<String>,
    /// Enable reasoning mode (think-tag models or native reasoning)
    #[serde(default)]
    pub reasoning_enabled: Option<bool>,
    /// Per-feature model selection feature name (Chat/Apply/Autocomplete/QuickEdit/SCM)
    #[serde(default)]
    pub feature: Option<String>,
}

pub(crate) fn trim_assistant_prefill(messages: &[ChatMessage]) -> Vec<ChatMessage> {
    let mut trimmed = messages.to_vec();
    // OpenAI-compat Claude gateways reject requests whose last message is an
    // empty assistant placeholder (we add one client-side for streaming UI).
    while let Some(last) = trimmed.last() {
        if !last.role.eq_ignore_ascii_case("assistant") {
            break;
        }
        let empty_text = last
            .content
            .as_ref()
            .map(|c| c.as_str().trim().is_empty())
            .unwrap_or(true);
        let no_tools = last.tool_calls.as_ref().map(|t| t.is_empty()).unwrap_or(true);
        if empty_text && no_tools {
            trimmed.pop();
        } else {
            break;
        }
    }
    trimmed
}

pub(crate) fn apply_highway_auth(
    request: reqwest::RequestBuilder,
    provider_key: &str,
) -> reqwest::RequestBuilder {
    request
        .header("x-api-key", provider_key)
        .header("Authorization", format!("Bearer {}", provider_key))
}

pub(crate) fn is_highway_family(provider: &str) -> bool {
    matches!(
        provider.to_lowercase().as_str(),
        "highwayapi" | "interfaceai" | "jiekou"
    )
}

pub(crate) fn is_opus_48_model(model: &str) -> bool {
    model.to_lowercase().contains("claude-opus-4-8")
}

/// Claude Opus 4.8 / Highway gateways reject `role:system` as the first entry in
/// `messages[]`. Hoist leading system turns to the top-level `system` field.
pub(crate) fn split_leading_system_messages(messages: &[ChatMessage]) -> (String, Vec<ChatMessage>) {
    let mut system_parts: Vec<String> = Vec::new();
    let mut i = 0usize;
    while i < messages.len() && messages[i].role.eq_ignore_ascii_case("system") {
        if let Some(c) = &messages[i].content {
            let t = c.to_text().trim().to_string();
            if !t.is_empty() {
                system_parts.push(t);
            }
        }
        i += 1;
    }
    (system_parts.join("\n\n"), messages[i..].to_vec())
}

/// Bare hostnames (`ai.example.com`) are not valid bases for `reqwest` — they become
/// path segments and trigger `builder error: relative URL without a base`. Only infer
/// a scheme when the string already looks like a hostname (contains `.` in the host
/// part, or starts with `localhost`).
pub fn normalize_local_base_url(raw: &str) -> String {
    let s = raw.trim().trim_end_matches('/');
    if s.is_empty() {
        return "http://127.0.0.1:11434".to_string();
    }
    if s.starts_with("http://") || s.starts_with("https://") {
        return s.to_string();
    }
    if s.starts_with("//") {
        return format!("https:{}", s.trim_end_matches('/'));
    }
    if !local_should_infer_scheme(s) {
        return s.to_string();
    }
    let hostish = s.trim_start_matches('/');
    let lower = hostish.to_lowercase();
    let scheme = if local_looks_like_loopback_or_lan(&lower) {
        "http"
    } else {
        "https"
    };
    format!("{}://{}", scheme, hostish)
        .trim_end_matches('/')
        .to_string()
}

/// Strip `:cloud` / `-cloud` suffixes — local agent runs must not target hosted cloud model IDs.
pub(crate) fn sanitize_local_model_id(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    if s.ends_with(":cloud") {
        s.truncate(s.len().saturating_sub(":cloud".len()));
    }
    if s.ends_with("-cloud") {
        s.truncate(s.len().saturating_sub("-cloud".len()));
    }
    s
}

pub(crate) fn local_should_infer_scheme(s: &str) -> bool {
    let head = s
        .split('/')
        .next()
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("")
        .trim_start_matches('/');
    if head.is_empty() {
        return false;
    }
    let lower = head.to_lowercase();
    if lower.starts_with("localhost") {
        return true;
    }
    // IPv4 / numeric host
    if head
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
        || head.starts_with('[')
    {
        return head.contains('.') || head.contains(':');
    }
    if !head.contains('.') {
        return false;
    }
    let labels: Vec<&str> = head.split('.').filter(|p| !p.is_empty()).collect();
    if labels.len() < 2 {
        return false;
    }
    labels.last().map_or(false, |tld| tld.len() >= 2)
}

pub(crate) fn local_looks_like_loopback_or_lan(lower: &str) -> bool {
    if lower.starts_with("localhost")
        || lower.starts_with("127.")
        || lower.starts_with("0.0.0.0")
        || lower.starts_with("[::1]")
    {
        return true;
    }
    if lower.starts_with("192.168.") {
        return true;
    }
    if lower.starts_with("10.") {
        return true;
    }
    if let Some(rest) = lower.strip_prefix("172.") {
        if let Some((oct2, _)) = rest.split_once('.') {
            if let Ok(n) = oct2.parse::<u32>() {
                return (16..=31).contains(&n);
            }
        }
    }
    false
}

/// Decrements `silent_emits` on drop so a `?` early-exit in
/// `ai_chat_oneshot` can't leave the engine permanently silent.
pub struct SilentEmitGuard {
    pub(crate) counter: Arc<AtomicUsize>,
}

impl Drop for SilentEmitGuard {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

