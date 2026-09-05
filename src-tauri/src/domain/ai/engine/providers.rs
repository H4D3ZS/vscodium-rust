//! Provider plumbing: Ollama payloads/options/diagnostics/native API,
//! model pulling/listing, intent classification, local fallback.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::time::Duration;
use tracing::instrument;
use super::types::*;
use super::sentient::Sentient;

impl Sentient {
    /// Parse the parameter count (in billions) from a model name like "qwen3:30b" → 30.
    /// Returns None if not found. Used for smarter context/tool decisions.
    pub(crate) fn parse_model_param_count(model: &str) -> Option<u32> {
        // Try to find a pattern like ":30b", ":7b", ":72b", "30b-", "7b-" etc.
        let m = model.to_lowercase();
        // Look for number followed by 'b' (optionally preceded by ':' or '-')
        let re_parts: Vec<&str> = m.split(|c| c == ':' || c == '-' || c == '_' || c == '/').collect();
        for part in re_parts {
            // Strip leading non-digits and trailing non-digits to extract e.g. "30b" → 30
            let digits: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
            let suffix: String = part.chars().skip(digits.len()).collect();
            if suffix.starts_with('b') && !digits.is_empty() {
                if let Ok(n) = digits.parse::<u32>() {
                    return Some(n);
                }
            }
        }
        None
    }

    /// Gemma 4 family (Ollama `gemma4:*`) — reasoning, native tools, multimodal.
    pub(crate) fn is_gemma4_model(model: &str) -> bool {
        let m = model.to_lowercase();
        m.contains("gemma4") || m.contains("gemma-4")
    }

    /// Ollama sampling tuned per model family (Gemma 4 uses publisher defaults).
    /// Small models (1B-4B) get slightly higher temperature to help with
    /// tool call diversity and reduce repetition loops.
    pub(crate) fn ollama_sampling(model: &str, is_chat_mode: bool, req_temp: Option<f32>) -> (f32, f32, u32) {
        if Self::is_gemma4_model(model) {
            // https://ollama.com/library/gemma4:12b — temp=1.0, top_p=0.95, top_k=64
            return (req_temp.unwrap_or(1.0), 0.95, 64);
        }
        let is_small = Self::is_small_model_name(model);
        let temp = if is_chat_mode {
            req_temp.unwrap_or(if is_small { 0.85 } else { 0.75 })
        } else {
            req_temp.unwrap_or(if is_small { 0.7 } else { 0.6 })
        };
        (temp, 1.0, 40)
    }

    pub(crate) fn ollama_num_predict(model: &str, is_chat_mode: bool) -> u32 {
        if Self::is_gemma4_model(model) && !is_chat_mode {
            return 16_384;
        }
        8192
    }

    /// Strip Gemma 4 `<|channel>thought` blocks from assistant text (keep final answer).
    pub(crate) fn strip_gemma4_thought_channels(content: &str) -> (String, Vec<String>) {
        let mut out = String::new();
        let mut thoughts = Vec::new();
        let mut pos = 0usize;
        while pos < content.len() {
            if let Some(rel) = content[pos..].find("<|channel>thought") {
                let start = pos + rel;
                out.push_str(&content[pos..start]);
                let after_marker = start + "<|channel>thought".len();
                if let Some(end_rel) = content[after_marker..].find("<channel|>") {
                    let end = after_marker + end_rel;
                    let thought = content[after_marker..end]
                        .trim_start_matches('\n')
                        .trim();
                    if !thought.is_empty() {
                        thoughts.push(thought.to_string());
                    }
                    pos = end + "<channel|>".len();
                } else {
                    out.push_str(&content[start..]);
                    break;
                }
            } else {
                out.push_str(&content[pos..]);
                break;
            }
        }
        (out.trim().to_string(), thoughts)
    }

    /// Returns whether a model is "small" (≤ 7B params) based on its name.
    /// Small models use the JSON-in-system-prompt tool protocol.
    /// 8B+ models use native OpenAI-compatible function calling.
    pub(crate) fn is_small_model_name(model: &str) -> bool {
        let m = model.to_lowercase();
        // Authoritative check FIRST: an explicit parameter count in the name wins.
        // ≤7B → small; 8B+ gets the native-tool path. This must run before any
        // substring heuristics, otherwise names like "14b", "24b", "34b", "12b"
        // get falsely matched by bare "4b"/"3b"/"2b" substrings and a perfectly
        // capable mid-size model is downgraded to the text-JSON fallback (and the
        // ≥14B forced-tool-choice path becomes unreachable).
        if let Some(n) = Self::parse_model_param_count(&m) {
            return n <= 7;
        }
        // No numeric parameter count in the name — fall back to size tags.
        if m.contains("mini") || m.contains("tiny") || m.contains("0.5b") || m.contains("1.5b") {
            return true;
        }
        // Gemma4 effective-param edge variants (e2b/e4b) behave like small models.
        // Match only the explicit "e2b"/"e4b" tokens — never a bare "2b"/"4b" that
        // could be the tail of "12b"/"24b".
        if (m.contains("gemma4") || m.contains("gemma-4"))
            && (m.contains("e2b") || m.contains("e4b"))
        {
            return true;
        }
        false
    }

    /// Rough token estimate for a message slice (~4 chars/token + 8 overhead/msg).
    pub(crate) fn estimate_messages_tokens(messages: &[ChatMessage]) -> usize {
        messages.iter().map(|m| {
            let content_chars = m.content.as_ref().map(|c| c.as_str().len()).unwrap_or(0);
            let tool_chars = m.tool_calls.as_ref()
                .map(|tc| tc.iter().map(|t| t.function.arguments.len() + 64).sum::<usize>())
                .unwrap_or(0);
            (content_chars + tool_chars) / 4 + 8
        }).sum()
    }

    /// Approximate context-window token limit for a model name.
    pub(crate) fn model_context_limit(model: &str) -> usize {
        let m = model.to_lowercase();
        if m.contains("claude") { return 180_000; }
        if m.contains("gpt-4o") || m.contains("gpt-4-turbo") { return 128_000; }
        if m.contains("gemini") { return 128_000; }
        if Self::is_gemma4_model(model) {
            if m.contains("26b") || m.contains("31b") {
                return 256_000;
            }
            return 131_072; // Gemma 4 12B / E-series — 128K native
        }
        if m.contains("qwen3.8") || m.contains("qwen-ambassador") { return 131_072; }
        if m.contains("qwen3.6") || m.contains("qwen3-6") { return 65_536; }
        if m.contains("qwen3") || m.contains("qwen-3") { return 32_768; }
        if let Some(n) = Self::parse_model_param_count(&m) {
            return match n {
                0..=7   => 8_192,
                8..=13  => 16_384,
                14..=32 => 32_768,
                _       => 65_536,
            };
        }
        32_768
    }

    /// Returns the recommended `num_ctx` for a given model name.
    /// Tiered by model size, then clamped to the host RAM tier (KV cache is
    /// the hidden RAM hog — 16K ctx on a 7b costs ~1.5–2GB of KV alone).
    pub(crate) fn recommended_num_ctx(model: &str) -> usize {
        crate::gpu_offload::clamp_num_ctx(Self::model_native_num_ctx(model))
    }

    /// Model-size-keyed context recommendation, before RAM-tier clamping.
    pub(crate) fn model_native_num_ctx(model: &str) -> usize {
        let m = model.to_lowercase();

        if Self::is_gemma4_model(model) {
            if m.contains("26b") || m.contains("31b") {
                return 32768;
            }
            if m.contains("12b") || m.contains("e4b") {
                return 32768; // 128K native; 32K fits laptop RAM for multi-file agent loops
            }
            return 16384; // e2b / edge
        }

        // Parse parameter count to tier context window
        let param_count = Self::parse_model_param_count(&m).unwrap_or(0);

        // 30B+ models: large context for complex agentic chains
        // With 40GB RAM + Ollama CPU offload, 32K is safe even on RX 580
        if param_count >= 30
            || m.contains("70b") || m.contains("72b") || m.contains("65b")
            || m.contains("34b") || m.contains("33b") || m.contains("32b")
        {
            return 32768;
        }

        // 14B models: 24K context — sweet spot for multi-file agentic work
        if param_count >= 14 || m.contains("14b") || m.contains("15b") {
            return 24576;
        }

        // 8–13B models: 16K — enough for most coding tasks
        if param_count >= 8 || m.contains("8b") || m.contains("9b") || m.contains("12b") || m.contains("13b") {
            return 16384;
        }

        // 3–7B: 8K — fits in VRAM entirely
        8192
    }

    /// Ollama only honors `num_ctx` / `num_predict` inside the `options` object
    /// (top-level fields are ignored → default 4096 ctx → exceed_context_size_error).
    pub(crate) fn ollama_inference_options(model: &str, temperature: f32, num_predict: u32) -> Value {
        let (_, top_p, top_k) = Self::ollama_sampling(model, false, Some(temperature));
        let mut opts = json!({
            "num_ctx": Self::recommended_num_ctx(model),
            "num_predict": num_predict,
            "temperature": temperature,
        });
        // VRAM-tiered GPU-layer offload: keep as many layers in VRAM as fit,
        // spill the rest to RAM (so e.g. a 27B runs on 8GB instead of OOMing).
        // `HADES_NUM_GPU` hard-overrides inside recommended_num_gpu; None leaves
        // the field unset so Ollama auto-decides.
        if let Some(n) = crate::gpu_offload::recommended_num_gpu(model) {
            opts["num_gpu"] = json!(n);
        }
        if Self::is_gemma4_model(model) {
            opts["top_p"] = json!(top_p);
            opts["top_k"] = json!(top_k);
        }
        opts
    }

    /// Last-chance guard: force `options.num_ctx` on every Ollama payload before POST.
    pub(crate) fn ensure_ollama_payload(payload: &mut Value, model: &str, temperature: f32, num_predict: u32) {
        if let Some(obj) = payload.as_object_mut() {
            obj.remove("num_ctx");
            obj.remove("num_predict");
            let num_ctx = Self::recommended_num_ctx(model);
            obj.insert(
                "options".to_string(),
                Self::ollama_inference_options(model, temperature, num_predict),
            );
            // RAM-tiered residency: lite keeps its single model warm so weights
            // aren't reloaded between agent turns (Ollama default is only 5m).
            obj.entry("keep_alive".to_string())
                .or_insert_with(|| json!(crate::gpu_offload::keep_alive()));
            println!("[AI] Ollama payload num_ctx={} model={}", num_ctx, model);
        }
    }

    /// Returns true if the model supports vision / image input via Ollama.
    /// Ollama passes images as a top-level `images` array (base64) on each message.
    pub(crate) fn is_vision_model(model: &str) -> bool {
        crate::vision_sidecar::is_vision_capable_model(model)
    }

    /// Ensure tool `arguments` string is valid JSON (Ollama rejects malformed history).
    pub(crate) fn sanitize_tool_arguments(raw: &str) -> String {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return "{}".to_string();
        }
        if serde_json::from_str::<Value>(trimmed).is_ok() {
            return trimmed.to_string();
        }
        let mut s = trimmed.to_string();
        let mut depth = 0i32;
        let mut in_str = false;
        let mut esc = false;
        for ch in s.chars() {
            if esc {
                esc = false;
                continue;
            }
            if ch == '\\' && in_str {
                esc = true;
                continue;
            }
            if ch == '"' {
                in_str = !in_str;
                continue;
            }
            if !in_str {
                match ch {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
            }
        }
        if in_str {
            s.push('"');
        }
        for _ in 0..depth.max(0) {
            s.push('}');
        }
        if serde_json::from_str::<Value>(&s).is_ok() {
            return s;
        }
        "{}".to_string()
    }

    pub(crate) fn truncate_for_ollama(s: &str, max: usize) -> String {
        if s.len() <= max {
            return s.to_string();
        }
        format!(
            "{}… [truncated {} chars]",
            &s[..max],
            s.len().saturating_sub(max)
        )
    }

    /// Serialize tool calls for Ollama. OpenAI-compat (`/v1`) wants `arguments` as a JSON
    /// string; native `/api/chat` accepts a parsed object (avoids template parser 400s).
    pub(crate) fn ollama_tool_calls_json(calls: &[ToolCall], openai_compat: bool) -> Value {
        json!(
            calls
                .iter()
                .map(|tc| {
                    let args_str = Self::sanitize_tool_arguments(&tc.function.arguments);
                    let mut func = json!({ "name": tc.function.name });
                    if openai_compat {
                        func["arguments"] = json!(args_str);
                    } else {
                        let args_val: Value =
                            serde_json::from_str(&args_str).unwrap_or(json!({}));
                        func["arguments"] = args_val;
                    }
                    json!({
                        "id": if tc.id.is_empty() { Value::Null } else { json!(&tc.id) },
                        "type": tc.type_field,
                        "function": func,
                    })
                })
                .collect::<Vec<_>>()
        )
    }

    pub(crate) async fn ollama_use_openai_compat_endpoint(&self, req: &AiRequest, model: &str) -> bool {
        // Lemonade only speaks the OpenAI-compatible protocol (no native
        // /api/chat), so its payloads must always use the OpenAI shape.
        if req.provider.eq_ignore_ascii_case("lemonade") {
            return true;
        }
        let base = self.resolved_local_base(req).await;
        let m = model.to_lowercase();
        Self::is_cyberifrit_managed_ollama_url(&base)
            || m.contains("bugtrace")
            || model.contains("hf.co/")
    }

    /// Transform `ChatMessage` list into a Ollama-compatible JSON messages array.
    /// For vision models: extracts `image_url` content parts → `images: [base64]` field.
    /// For text-only or non-vision Ollama models: serialises messages normally.
    pub(crate) fn build_ollama_messages(messages: &[ChatMessage], vision: bool, openai_compat: bool) -> Value {
        const TOOL_CONTENT_MAX: usize = 48_000;
        let msg_array: Vec<Value> = messages
            .iter()
            .map(|m| {
            let role = &m.role;
            let role_lc = role.to_lowercase();

            match &m.content {
                None | Some(MessageContent::Text(_)) => {
                    let mut text = m.content.as_ref().map(|c| c.to_text()).unwrap_or_default();
                    if role_lc == "tool" {
                        text = Self::truncate_for_ollama(&text, TOOL_CONTENT_MAX);
                    }
                    let mut obj = json!({ "role": role, "content": text });
                    // Pass tool_calls through if present (Ollama native tools)
                    if let Some(tc) = &m.tool_calls {
                        if !tc.is_empty() {
                            obj["tool_calls"] = Self::ollama_tool_calls_json(tc, openai_compat);
                            // Duplicate ```json tool blocks in content break Ollama parsers.
                            if role_lc == "assistant" {
                                obj["content"] = json!("");
                            }
                        }
                    }
                    if let Some(tcid) = &m.tool_call_id {
                        obj["tool_call_id"] = json!(tcid);
                    }
                    obj
                }
                Some(MessageContent::Parts(parts)) => {
                    if vision {
                        // Separate text parts from image parts
                        let mut text_buf = String::new();
                        let mut images: Vec<String> = Vec::new();

                        for part in parts {
                            match part {
                                ContentPart::Text { text } => {
                                    if !text_buf.is_empty() { text_buf.push('\n'); }
                                    text_buf.push_str(text);
                                }
                                ContentPart::ImageUrl { image_url } => {
                                    // Strip the data-URI prefix (data:image/...;base64,)
                                    let b64 = if let Some(pos) = image_url.url.find(',') {
                                        image_url.url[pos + 1..].to_string()
                                    } else {
                                        image_url.url.clone()
                                    };
                                    images.push(b64);
                                }
                            }
                        }

                        let mut obj = json!({ "role": role, "content": text_buf });
                        if !images.is_empty() {
                            obj["images"] = json!(images);
                        }
                        obj
                    } else {
                        // Non-vision Ollama: collapse parts to plain text, drop images
                        let text: String = parts.iter().filter_map(|p| {
                            if let ContentPart::Text { text } = p { Some(text.as_str()) } else { None }
                        }).collect::<Vec<_>>().join("\n");
                        json!({ "role": role, "content": text })
                    }
                }
            }
        }).collect();

        json!(msg_array)
    }

        /// Lightweight Lemonade health probe. Real lemonade-server exposes its
    /// OpenAI-compatible surface under `/api/v1/`; older builds and reverse
    /// proxies expose `/v1/` — try both. Cloud (JWT-gated) Lemonade gets the
    /// signed-in bearer token, local is keyless.
    pub async fn check_lemonade_status(&self) -> Result<bool> {
        let root = self.lemonade_base().await;
        for path in ["/api/v1/models", "/v1/models"] {
            let mut req = self.client.get(format!("{}{}", root, path));
            let tok = self.get_key_for_provider("lemonade");
            if !tok.trim().is_empty() {
                req = req.bearer_auth(tok.trim());
            }
            match req
                .timeout(std::time::Duration::from_secs(3))
                .send()
                .await
            {
                Ok(r) if r.status().is_success() => return Ok(true),
                Ok(r) if r.status().as_u16() == 404 => continue,
                _ => {}
            }
        }
        Ok(false)
    }

    /// Probe the configured Ollama endpoint and report exactly what went wrong
    /// so the user doesn't have to guess between "wrong URL", "no bearer",
    /// "nginx returned HTML", "401" or "ollama empty".
        /// `/api/foo` then `/v1/api/foo` on 404 (nginx rewrite; see `tools/vps-ollama-proxy/bootstrap.sh`).
        /// Ollama GET from Rust so the webview is not subject to nginx CORS.
        /// Ollama POST from Rust (same CORS bypass + `/v1` fallback as GET).
        /// Unified Ollama HTTP request with retry, fallback URLs, and CORS bypass.
            /// Pull a model on the Lemonade server. lemonade-server exposes
    /// `POST /api/v1/pull` with `{"model_name": ...}`; long timeout because
    /// model downloads can take many minutes.
    pub async fn pull_lemonade_model(&self, name: &str) -> Result<()> {
        let root = self.lemonade_base().await;
        let payload = json!({ "model_name": name });
        let tok = self.get_key_for_provider("lemonade");
        for path in ["/api/v1/pull", "/v1/pull"] {
            let mut req = self
                .client
                .post(format!("{}{}", root, path))
                .timeout(std::time::Duration::from_secs(1800))
                .json(&payload);
            if !tok.trim().is_empty() {
                req = req.bearer_auth(tok.trim());
            }
            let resp = req.send().await?;
            if resp.status().is_success() {
                return Ok(());
            }
            if resp.status().as_u16() != 404 {
                let status = resp.status();
                let body: String = resp.text().await.unwrap_or_default().chars().take(160).collect();
                return Err(anyhow!("Lemonade pull failed: HTTP {} {}", status, body));
            }
        }
        Err(anyhow!("Lemonade pull failed: no pull endpoint on {}", root))
    }

}

impl Sentient {
    /// Model-driven intent routing. Ask the SAME provider/model whether the
    /// user's latest message needs autonomous tool use (ACTION) or is plain
    /// conversation (CHAT). Returns Some(true)=action, Some(false)=chat, None
    /// when the call fails/times out (caller falls back to a cheap heuristic).
    pub(crate) async fn classify_action_intent(&self, req: &AiRequest, user_text: &str) -> Option<bool> {
        let user_text = user_text.trim();
        if user_text.is_empty() {
            return Some(false);
        }
        let sys = "You are the intent router for an autonomous coding + security agent that can read/write files, \
run shell commands, search a codebase, scan, recon, and exploit authorized targets. \
Classify the user's latest message into exactly one bucket:\n\
- ACTION: they want the agent to DO something with tools — write/edit/refactor code, run commands, build, \
debug, investigate the codebase, scan/recon/audit/exploit a target, fix a bug, etc.\n\
- CHAT: a greeting, small talk, or a factual/explanatory question that can be answered with words alone and needs no tools.\n\
Reply with EXACTLY ONE word: ACTION or CHAT. No punctuation, no explanation.";
        let classify_req = AiRequest {
            provider: req.provider.clone(),
            model: req.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: Some(MessageContent::Text(sys.to_string())),
                    ..Default::default()
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: Some(MessageContent::Text(format!(
                        "User message:\n{}\n\nOne word — ACTION or CHAT:",
                        user_text.chars().take(2000).collect::<String>()
                    ))),
                    ..Default::default()
                },
            ],
            temperature: Some(0.0),
            autonomous: false,
            mode: Some("Chat".to_string()),
            cyber_mode: None,
            root_access: None,
            ollama_url: req.ollama_url.clone(),
            tools: Some(vec![]),
            reasoning_budget: None,
            reasoning_effort: None,
            reasoning_enabled: Some(false),
            feature: Some("Chat".to_string()),
        };
        let fut = self.single_shot_completion(classify_req);
        match tokio::time::timeout(std::time::Duration::from_secs(15), fut).await {
            Ok(Ok(text)) => {
                let t = text.to_ascii_uppercase();
                // Read the first decisive token; some models prefix reasoning.
                let action_idx = t.find("ACTION");
                let chat_idx = t.find("CHAT");
                match (action_idx, chat_idx) {
                    (Some(a), Some(c)) => Some(a <= c), // whichever appears first wins
                    (Some(_), None) => Some(true),
                    (None, Some(_)) => Some(false),
                    (None, None) => None,
                }
            }
            _ => None, // timeout or provider error → caller falls back
        }
    }

    /// Whether this turn is an authorized offensive-security engagement (mode,
    /// persona tag, or pentest keywords in the user message).
    pub(crate) fn is_offensive_engagement(mode: &str, user_text: &str) -> bool {
        let mode_l = mode.to_ascii_lowercase();
        if mode_l.contains("bugbounty")
            || mode_l.contains("bug bounty")
            || mode_l.contains("redteam")
            || mode_l.contains("red team")
            || mode_l.contains("blueteam")
            || mode_l.contains("blue team")
            || mode_l.contains("pentest")
        {
            return true;
        }
        let t = user_text.to_ascii_lowercase();
        if t.contains("[persona:")
            || t.contains("[intent:")
            || t.contains("[scope lock")
            || t.contains("no refusal")
        {
            return true;
        }
        const HINTS: &[&str] = &[
            "pentest",
            "pen test",
            "penetration test",
            "bug bounty",
            "bugbounty",
            "red team",
            "redteam",
            "web_security_audit",
            "sec_distro_inventory",
            "scope lock",
            "kill chain",
            "mitre att&ck",
            "weaponize",
            "reverse shell",
            "payload",
            "poc_",
            "vuln hunt",
            "offensive security",
            "nuclei",
            "sqlmap",
        ];
        HINTS.iter().any(|h| t.contains(h))
    }

    /// Heuristic refusal detector for authorized-security continuity. Returns
    /// true when an assistant *text* reply (no tool calls) is a moralizing
    /// decline rather than real work. Tuned to avoid false positives on
    /// legitimate output: a hard decline phrase fires immediately; softer
    /// markers count only when the reply is short (a real answer with code /
    /// tool plans is long) and stacks two or more markers. When
    /// `aggressive_offensive` is true (pentest / bug-bounty turns), a single
    /// soft marker on a short reply is enough.
    pub(crate) fn is_refusal(text: &str, aggressive_offensive: bool) -> bool {
        let t = text.trim();
        if t.is_empty() {
            return false;
        }
        let lower = t.to_ascii_lowercase();
        // Strong, unambiguous decline phrases.
        const HARD: &[&str] = &[
            "i can't help with that", "i cannot help with that",
            "i can't assist with that", "i cannot assist with that",
            "i can't help you with that", "i won't be able to help",
            "i'm not able to help with that", "i am not able to help with that",
            "i can't provide", "i cannot provide", "i won't provide",
            "i can't create that", "i cannot create that",
            "i must decline", "i have to decline", "i'll have to decline",
            "i can't engage in", "i cannot engage in",
            "i'm unable to assist", "i am unable to assist",
            "i can't assist with hacking", "i cannot assist with hacking",
            "i can't help with hacking", "i won't help with",
            "as an ai language model", "i can't comply", "i cannot comply",
            "i can't participate in", "i cannot participate in",
            "this request involves illegal", "is illegal and unethical",
            "against my guidelines", "against my policies", "content policy",
            "safety guidelines", "policy prohibits", "i'm unable to",
            "i am unable to", "not able to provide", "for educational purposes only",
            "could be used for malicious", "potentially harmful",
            "i apologize, but i", "sorry, but i can't", "sorry, but i cannot",
            "decline this request", "i need to decline", "must refuse",
            "without proper authorization", "without explicit authorization",
            "i recommend consulting", "seek legal advice",
        ];
        if HARD.iter().any(|p| lower.contains(p)) {
            return true;
        }
        // Softer markers: only a refusal when the reply is short and stacks ≥2
        // (or ≥1 when aggressive_offensive).
        const SOFT: &[&str] = &[
            "i can't", "i cannot", "i'm not able", "i am not able",
            "i'm sorry", "i am sorry", "unable to assist", "against my",
            "not appropriate", "cannot help", "can't help",
            "consult a professional", "seek professional",
            "without proper authorization", "is unethical",
            "i'm just an ai", "ethical guidelines", "i won't",
            "not comfortable", "i'd rather not",
        ];
        let short = t.len() < 800;
        let soft_hits = SOFT.iter().filter(|p| lower.contains(*p)).count();
        if aggressive_offensive {
            short && soft_hits >= 1
        } else {
            short && soft_hits >= 2
        }
    }

    /// Pick an installed local model. Retained as a utility; no longer used for
    /// auto-fallback (we never switch the user's chosen model).
    #[allow(dead_code)]
    pub(crate) async fn pick_local_fallback_model(&self) -> Option<String> {
        let models = self.list_models("lemonade").await.ok()?;
        if models.is_empty() {
            return None;
        }
        const PREFERRED: &[&str] = &[
            "neuraldevil", "cyberifrit", "sec-eng", "bugtrace", "dolphin",
            "uncensor", "abliterat", "hermes", "wizard-vicuna",
        ];
        for key in PREFERRED {
            if let Some(m) = models.iter().find(|m| m.to_ascii_lowercase().contains(key)) {
                return Some(m.clone());
            }
        }
        if let Some(m) = models.iter().find(|m| {
            let l = m.to_ascii_lowercase();
            l.contains("coder") || l.contains("qwen") || l.contains("deepseek")
        }) {
            return Some(m.clone());
        }
        models.into_iter().next()
    }

}

impl Sentient {
    /// Dynamically get models for a provider
    #[instrument(skip(self))]
    pub async fn list_models(&self, provider: &str) -> Result<Vec<String>> {
        println!("Listing models for provider: {}", provider);
        // Interface AI (highwayapi.ai) doesn't expose a /models listing on the
        // free base — return the known model so it appears without an error.
        if matches!(provider.to_lowercase().as_str(), "highwayapi" | "interfaceai" | "jiekou") {
            return Ok(vec!["claude-opus-4-8".to_string()]);
        }
        // Headless web-chat providers are keyless and have a single fixed "model"
        // (the provider id itself); no remote /models endpoint to query.
        if provider.to_lowercase().starts_with("webchat") {
            return Ok(vec![provider.to_lowercase()]);
        }
        let provider_key = self.get_key_for_provider(provider);

        let mut has_google_base_url = false;
        let mut custom_google_base = String::new();
        if let Ok(url) = std::env::var("GOOGLE_BASE_URL") {
            if !url.is_empty() {
                has_google_base_url = true;
                custom_google_base = url.trim().trim_end_matches('/').to_string();
            }
        }
        if !has_google_base_url {
            let keys_path = self.api_keys_path();
            if let Ok(content) = std::fs::read_to_string(&keys_path) {
                if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                    if let Some(custom_url) = keys["google_base_url"].as_str() {
                        if !custom_url.is_empty() {
                            has_google_base_url = true;
                            custom_google_base = custom_url.trim().trim_end_matches('/').to_string();
                        }
                    }
                }
            }
        }

        // ApiRadar removed — leaked-key aggregator, no longer supported.
        if provider.to_lowercase() == "apiradar" {
            return Ok(Vec::new());
        }
        {
            let plc = provider.to_lowercase();
            // Keyless-friendly providers: local servers + our own OpenAI-compatible
            // endpoints (MiMo/Cyber-Ifrit may front a keyless local AMD box; the
            // listing has graceful fallbacks below if the fetch fails).
            let keyless_ok = matches!(plc.as_str(),
                "antigravity" | "mimo" | "xiaomi"
                | "cyberifrit" | "cyber-ifrit" | "cyberifrit-cloud"
                | "lemonade" | "huggingface" | "openmodel");
            if provider_key.is_empty() && !keyless_ok {
                return Err(anyhow!("API key not found for provider: {}", provider));
            }
        }

        if provider.to_lowercase() == "deepseek" {
            let endpoint = "https://api.deepseek.com/v1/models";
            let mut request = self.client.get(endpoint);
            if !provider_key.trim().is_empty() {
                request = request.bearer_auth(provider_key.trim());
            }
            let response = request
                .send()
                .await
                .map_err(|e| anyhow!("DeepSeek list_models: {}", e))?;
            let result: Value = response
                .json()
                .await
                .map_err(|e| anyhow!("DeepSeek list_models JSON: {}", e))?;
            let mut model_ids = Vec::new();
            if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                for m in data {
                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                        model_ids.push(id.to_string());
                    }
                }
            }
            return Ok(model_ids);
        }

        // Lemonade — OpenAI-compatible API. Base comes from the stored value
        // (set via `set_lemonade_url` from the frontend), then env, then default.
        if provider.to_lowercase() == "lemonade" {
            let lemonade_url = self.lemonade_base().await;
            let root = lemonade_url.trim_end_matches('/');
            // Real lemonade-server serves /api/v1/models; some gateways only /v1/models.
            let mut response = None;
            let mut last_err: Option<anyhow::Error> = None;
            for path in ["/api/v1/models", "/v1/models"] {
                let mut request = self.client.get(format!("{}{}", root, path));
                // Cloud Lemonade sits behind the Cyber-Ifrit JWT gate — attach the
                // signed-in token (resolved via get_key_for_provider). Local
                // (127.0.0.1/localhost) is keyless, so the token is simply absent.
                let tok = self.get_key_for_provider("lemonade");
                if !tok.trim().is_empty() {
                    request = request.bearer_auth(tok.trim());
                }
                match request.send().await {
                    Ok(r) if r.status().as_u16() == 404 => { response = Some(r); continue; }
                    Ok(r) => { response = Some(r); break; }
                    Err(e) => { last_err = Some(anyhow!("Lemonade list_models: {}", e)); }
                }
            }
            let response = match response {
                Some(r) => r,
                None => return Err(last_err.unwrap_or_else(|| anyhow!("Lemonade list_models: unreachable"))),
            };
            // Surface auth/gate failures instead of silently returning an empty
            // list. The JWT gate answers 401 (sign in) / 402-403 (tier not
            // entitled) with a JSON error body that would otherwise parse to a
            // model-less result and look like "no models".
            let status = response.status();
            let raw = response.text().await.unwrap_or_default();
            if !status.is_success() {
                let hint = if status.as_u16() == 401 {
                    " — sign in (Settings → Account) so your cloud token is sent"
                } else if matches!(status.as_u16(), 402 | 403) {
                    " — signed in but your plan/tier is not entitled to cloud Lemonade"
                } else { "" };
                let snippet: String = raw.chars().take(160).collect();
                return Err(anyhow!("Lemonade returned HTTP {}{}: {}", status.as_u16(), hint, snippet));
            }
            let result: Value = serde_json::from_str(&raw)
                .map_err(|e| anyhow!("Lemonade list_models JSON: {} (body: {})", e, raw.chars().take(120).collect::<String>()))?;
            let mut model_ids = Vec::new();
            if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                for m in data {
                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                        model_ids.push(id.to_string());
                    }
                }
            }
            return Ok(model_ids);
        }

        // Hugging Face Router — OpenAI-compatible API
        if provider.to_lowercase() == "huggingface" {
            let hf_url = "https://router.huggingface.co/v1";
            let endpoint = format!("{}/models", hf_url);
            let mut request = self.client.get(&endpoint);
            // HF token from env or api_keys.json
            if let Ok(content) = std::fs::read_to_string(self.api_keys_path()) {
                if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                    if let Some(token) = keys["huggingface"].as_str() {
                        if !token.is_empty() {
                            request = request.bearer_auth(token);
                        }
                    }
                }
            }
            if let Ok(token) = std::env::var("HF_TOKEN") {
                if !token.is_empty() {
                    request = request.bearer_auth(&token);
                }
            }
            let response = request
                .send()
                .await
                .map_err(|e| anyhow!("Hugging Face list_models: {}", e))?;
            let result: Value = response
                .json()
                .await
                .map_err(|e| anyhow!("Hugging Face list_models JSON: {}", e))?;
            let mut model_ids = Vec::new();
            if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                for m in data {
                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                        model_ids.push(id.to_string());
                    }
                }
            }
            return Ok(model_ids);
        }

        // ModelScope (Qwen Ambassador) — OpenAI-compatible cloud inference.
        if provider.to_lowercase() == "modelscope" || provider.to_lowercase() == "qwen-ambassador" {
            let configured = std::env::var("MODELSCOPE_BASE_URL").ok()
                .filter(|s| !s.trim().is_empty())
                .or_else(|| {
                    self.brain_dir.parent()
                        .map(|p| p.join("api_keys.json"))
                        .and_then(|p| std::fs::read_to_string(p).ok())
                        .and_then(|c| serde_json::from_str::<Value>(&c).ok())
                        .and_then(|k| k["modelscope_base_url"].as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                })
                .unwrap_or_else(|| "https://api-inference.modelscope.ai/v1".to_string());
            let base = configured.trim().trim_end_matches('/').to_string();
            let models_url = if base.ends_with("/v1") {
                format!("{}/models", base)
            } else {
                format!("{}/v1/models", base)
            };
            let mut req = self.client.get(&models_url);
            if !provider_key.trim().is_empty() {
                req = req.bearer_auth(provider_key.trim());
            }
            let fetched: Vec<String> = match req
                .timeout(Duration::from_secs(10))
                .send().await
            {
                Ok(resp) if resp.status().is_success() => {
                    let result: Value = resp.json().await.unwrap_or(json!({}));
                    result.get("data").and_then(|d| d.as_array()).map(|arr| {
                        arr.iter().filter_map(|m| m.get("id").and_then(|i| i.as_str()).map(|s| s.to_string())).collect()
                    }).unwrap_or_default()
                }
                _ => Vec::new(),
            };
            if !fetched.is_empty() {
                return Ok(fetched);
            }
            return Ok(vec![
                "Qwen-Ambassador/Qwen3.8-Max".to_string(),
            ]);
        }

        // Xiaomi MiMo + Cyber-Ifrit — OpenAI-compatible. Derive `/v1/models` from
        // the same configured chat endpoint (env/keys override aware) and parse
        // `data[].id`. MiMo falls back to a curated list; Cyber-Ifrit hosts custom
        // named models so it degrades to an empty list (the user types the name).
        {
            let plc = provider.to_lowercase();
            let is_mimo = plc == "mimo" || plc == "xiaomi";
            let is_cyberifrit = plc == "cyberifrit" || plc == "cyber-ifrit" || plc == "cyberifrit-cloud";
            if is_mimo || is_cyberifrit {
                // Resolve base: env override → api_keys.json override → default.
                let (env_var, keys_field, default_base) = if is_mimo {
                    ("MIMO_BASE_URL", "mimo_base_url", "https://api.xiaomimimo.com/v1")
                } else {
                    ("CYBERIFRIT_BASE_URL", "cyberifrit_base_url", "https://api.cyberifrit.xyz")
                };
                let configured = std::env::var(env_var).ok()
                    .filter(|s| !s.trim().is_empty())
                    .or_else(|| {
                        self.brain_dir.parent()
                            .map(|p| p.join("api_keys.json"))
                            .and_then(|p| std::fs::read_to_string(p).ok())
                            .and_then(|c| serde_json::from_str::<Value>(&c).ok())
                            .and_then(|k| k[keys_field].as_str().map(|s| s.to_string()))
                            .filter(|s| !s.trim().is_empty())
                    })
                    .unwrap_or_else(|| default_base.to_string());
                let base = configured.trim().trim_end_matches('/').to_string();
                let models_url = if base.ends_with("/v1") {
                    format!("{}/models", base)
                } else if base.ends_with("/models") {
                    base.clone()
                } else {
                    format!("{}/v1/models", base)
                };
                let mut req = self.client.get(&models_url);
                if !provider_key.trim().is_empty() {
                    req = req.bearer_auth(provider_key.trim());
                }
                let fetched: Vec<String> = match req.send().await {
                    Ok(resp) if resp.status().is_success() => {
                        let result: Value = resp.json().await.unwrap_or(json!({}));
                        result.get("data").and_then(|d| d.as_array()).map(|arr| {
                            arr.iter().filter_map(|m| m.get("id").and_then(|i| i.as_str()).map(|s| s.to_string())).collect()
                        }).unwrap_or_default()
                    }
                    _ => Vec::new(),
                };
                if !fetched.is_empty() {
                    return Ok(fetched);
                }
                // Graceful fallbacks so the picker is never broken. These are the
                // CURRENT MiMo API model ids (the old mimo-v2.5* ids 404). Used only
                // when the live /v1/models fetch returns nothing (e.g. the Token Plan
                // key can't list). `mimo-v2-omni` is the multimodal ("mix") model.
                if is_mimo {
                    return Ok(vec![
                        "mimo-v2-pro".to_string(),
                        "mimo-v2-flash".to_string(),
                        "mimo-v2-omni".to_string(),
                    ]);
                }
                return Ok(Vec::new());
            }
        }

        // Local DeepSeek-V2 over MLX or llama.cpp on Apple Silicon. Calls
        // the local `/v1/models` and returns whatever the server exposes.
        // If the server isn't running we surface a clear error (and the
        // ApiRadar curated list still has sensible defaults).
        let provider_lc = provider.to_lowercase();
        if provider_lc == "deepseek-ane"
            || provider_lc == "deepseek_ane"
            || provider_lc == "ds2-ane"
        {
            let base = std::env::var("DEEPSEEK_ANE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string())
                .trim()
                .trim_end_matches('/')
                .to_string();
            // Walk through possible "models" paths in priority order — MLX-LM
            // and llama-server agree on /v1/models but a bare base URL works
            // too.
            let endpoint = if base.ends_with("/v1") {
                format!("{}/models", base)
            } else if base.ends_with("/v1/models") {
                base.clone()
            } else {
                format!("{}/v1/models", base)
            };
            let mut request = self.client.get(&endpoint);
            if !provider_key.trim().is_empty() {
                request = request.bearer_auth(provider_key.trim());
            }
            let response = request
                .send()
                .await
                .map_err(|e| anyhow!(
                    "DeepSeek-ANE local server not reachable at {} — start it with \
                     `bash tools/deepseek-ane/start-server.sh` (or set DEEPSEEK_ANE_URL): {}",
                    endpoint, e
                ))?;
            let result: Value = response
                .json()
                .await
                .map_err(|e| anyhow!("DeepSeek-ANE list_models JSON: {}", e))?;
            let mut model_ids = Vec::new();
            if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                for m in data {
                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                        model_ids.push(id.to_string());
                    }
                }
            }
            if model_ids.is_empty() {
                // Sensible fallback so the picker isn't empty if the server
                // doesn't enumerate models — these are the Mac-friendly
                // quantizations our bootstrap.sh downloads.
                model_ids.push("deepseek-v2-lite-chat-q4_k_m".to_string());
                model_ids.push("deepseek-coder-v2-lite-instruct-q4_k_m".to_string());
            }
            return Ok(model_ids);
        }


        let endpoint = if (provider.to_lowercase() == "google" || provider.to_lowercase() == "gemini") && has_google_base_url {
            if custom_google_base.ends_with("/models") {
                custom_google_base.clone()
            } else if custom_google_base.ends_with("/v1") {
                format!("{}/models", custom_google_base)
            } else {
                format!("{}/v1/models", custom_google_base)
            }
        } else {
            let endpoint_ref = match provider.to_lowercase().as_str() {
                "google" | "gemini" => "https://generativelanguage.googleapis.com/v1beta/models",
                "openai" => "https://api.openai.com/v1/models",
                "anthropic" => "https://api.anthropic.com/v1/models",
                "groq" => "https://api.groq.com/openai/v1/models",
                "openrouter" => "https://openrouter.ai/api/v1/models",
                "openmodel" => "https://api.openmodel.ai/v1/models",
                "mistral" => "https://api.mistral.ai/v1/models",
                "xai" => "https://api.x.ai/models",
                "cerebras" => "https://api.cerebras.ai/v1/models",
                "nvidia" => "https://integrate.api.nvidia.com/v1/models",
                "apiradar" => "https://apiradar.live/api/v1/models",
                "openwebui" | "openwebui-claude" | "openwebui-gpt" | "openwebui-gemini" => {
                    "http://127.0.0.1:8080/api/models"
                }
                _ => {
                    return Err(anyhow!(
                        "Model listing not supported for provider: {}",
                        provider
                    ))
                }
            };
            endpoint_ref.to_string()
        };

        let mut request = self.client.get(endpoint);

        if provider.to_lowercase() == "google" || provider.to_lowercase() == "gemini" {
            if has_google_base_url {
                request = request.bearer_auth(&provider_key);
            } else {
                request = request.query(&[("key", &provider_key)]);
            }
        } else if provider.to_lowercase() == "anthropic" || provider.to_lowercase() == "openmodel" {
            request = request
                .header("x-api-key", &provider_key)
                .header("anthropic-version", "2023-06-01");
        } else if provider.to_lowercase() == "lemonade" {
            // Local Lemonade is keyless
        } else {
            request = request.bearer_auth(&provider_key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch models: {}", e))?;

        let result: Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse models response: {}", e))?;

        let mut model_ids = Vec::new();

        match provider.to_lowercase().as_str() {
            "google" | "gemini" => {
                if let Some(models) = result.get("models").and_then(|m| m.as_array()) {
                    for m in models {
                        if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                            let id = name.to_string(); // Keep full name for Google provider
                                                       // Filter for chat-capable Gemini models only
                            if id.contains("gemini")
                                && !id.contains("vision")
                                && !id.contains("embedding")
                                && !id.contains("text-")
                            {
                                model_ids.push(id);
                            }
                        }
                    }
                }
                if model_ids.is_empty() {
                    // Try OpenAI compatible parsing since it might be a custom proxy/reseller
                    if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                        for m in data {
                            if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                                if id.contains("gemini") {
                                    model_ids.push(id.to_string());
                                }
                            }
                        }
                    }
                }
            }
            "anthropic" => {
                if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                    for m in data {
                        if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                            model_ids.push(id.to_string());
                        }
                    }
                }
            }
            "lemonade" => {
                if let Some(models) = result.get("models").and_then(|m| m.as_array()) {
                    for m in models {
                        if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                            model_ids.push(name.to_string());
                        }
                    }
                }
            }
            _ => {
                // OpenAI compatible
                if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                    for m in data {
                        if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                            model_ids.push(id.to_string());
                        }
                    }
                }
                // Hardcoded fallback for OpenModel if no models returned
                if model_ids.is_empty() && provider.to_lowercase() == "openmodel" {
                    model_ids.extend([
                        "deepseek-v4-flash".to_string(),
                        "deepseek-chat".to_string(),
                        "deepseek-v4".to_string(),
                        "gpt-4o".to_string(),
                        "claude-sonnet-4-20250514".to_string(),
                        "qwen3-max".to_string(),
                        "mimo-v2.5-pro".to_string(),
                    ]);
                }
            }
        }

        Ok(model_ids)
    }

    /// Trims the conversation history to stay within a character limit
    pub(crate) async fn trim_context(
        &self,
        mut messages: Vec<ChatMessage>,
        max_chars: usize,
    ) -> Vec<ChatMessage> {
        if messages.is_empty() {
            return messages;
        }

        // Always try to keep the system message
        let system_msg = if messages[0].role == "system" {
            Some(messages.remove(0))
        } else {
            None
        };

        let mut current_chars = system_msg
            .as_ref()
            .map(|m| m.content.as_ref().map(|c| c.to_text().len()).unwrap_or(0))
            .unwrap_or(0);
        let mut final_messages = Vec::new();

        // Traverse backwards and keep messages until limit is reached
        for msg in messages.into_iter().rev() {
            let msg_len = msg.content.as_ref().map(|c| c.to_text().len()).unwrap_or(0);
            if current_chars + msg_len > max_chars && !final_messages.is_empty() {
                break;
            }
            current_chars += msg_len;
            final_messages.insert(0, msg);
        }

        if let Some(sys) = system_msg {
            final_messages.insert(0, sys);
        }

        final_messages
    }

}

impl Sentient {
    /// Resolve a Lemonade model name to the one we should actually chat with.
    ///
    /// Lemonade "omni" entries (recipe `collection.omni`, e.g. `LMX-Omni-5.5B-Lite`)
    /// are bundles of an LLM + Whisper (ASR) + TTS + image model. Sending the
    /// COLLECTION name to `/v1/chat/completions` makes Lemonade eagerly load every
    /// component — including whisper-server, which can crash/fail on many machines —
    /// so the whole chat 400s with `whisper-server failed to start`. The official
    /// Lemonade app sidesteps this: for text chat it resolves the collection to its
    /// primary LLM component and sends THAT id (the ASR/TTS parts load lazily, only
    /// when you actually send audio). We mirror that here.
    ///
    /// Returns the LLM component id when `model` is a collection, otherwise `model`
    /// unchanged. Best-effort: any lookup failure returns the original name.
    pub(crate) async fn resolve_lemonade_chat_model(&self, req: &AiRequest) -> String {
        let model = req.model.clone();

        // Normalize model ID: Lemonade uses colon for quantizer (model:Q4_K_M),
        // but the IDE may store it with a hyphen (model-Q4_K_M). Fix this
        // before querying the server.
        let model = Self::normalize_lemonade_model_id(&model);

        // Resolve the server root using our dedicated async resolver:
        let base = self.lemonade_base().await;
        let root = base
            .trim()
            .trim_end_matches('/')
            .trim_end_matches("/v1/chat/completions")
            .trim_end_matches("/chat/completions")
            .trim_end_matches("/v1")
            .trim_end_matches('/')
            .to_string();
        let encoded = model.replace(' ', "%20");
        let url = format!("{}/api/v1/models/{}", root, encoded);

        let resp = match self
            .client
            .get(&url)
            .timeout(Duration::from_secs(8))
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => r,
            _ => return model, // not found / not reachable → use as-is
        };
        let info: Value = match resp.json().await {
            Ok(v) => v,
            Err(_) => return model,
        };

        // Not a collection → nothing to resolve.
        let components = info.get("components").and_then(|c| c.as_array());
        let has_components = components.map(|a| !a.is_empty()).unwrap_or(false);
        if !has_components {
            return model;
        }

        // Prefer the component whose recipe is an LLM engine (never an ASR/TTS/image
        // backend). The `models` array carries full per-component metadata.
        const LLM_RECIPES: &[&str] = &["llamacpp", "ryzenai-llm", "flm", "vllm"];
        const NON_LLM_RECIPES: &[&str] = &["whispercpp", "kokoro", "sd-cpp", "moonshine"];
        if let Some(models) = info.get("models").and_then(|m| m.as_array()) {
            // 1) explicit LLM recipe
            for m in models {
                let recipe = m.get("recipe").and_then(|r| r.as_str()).unwrap_or("");
                if LLM_RECIPES.contains(&recipe) {
                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                        return id.to_string();
                    }
                }
            }
            // 2) tool-calling / vision label, and not an ASR/TTS/image recipe
            for m in models {
                let recipe = m.get("recipe").and_then(|r| r.as_str()).unwrap_or("");
                if NON_LLM_RECIPES.contains(&recipe) {
                    continue;
                }
                let is_chat = m
                    .get("labels")
                    .and_then(|l| l.as_array())
                    .map(|arr| {
                        arr.iter().any(|x| {
                            matches!(x.as_str(), Some("tool-calling") | Some("vision") | Some("mtp"))
                        })
                    })
                    .unwrap_or(false);
                if is_chat {
                    if let Some(id) = m.get("id").and_then(|i| i.as_str()) {
                        return id.to_string();
                    }
                }
            }
        }
        // 3) fallback: first component id
        if let Some(first) = components
            .and_then(|a| a.first())
            .and_then(|c| c.as_str())
        {
            return first.to_string();
        }
        model
    }

    /// Synchronous default for the Lemonade base when no async lock is convenient.
    fn lemonade_base_blocking_default(&self) -> String {
        std::env::var("LEMONADE_URL")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| "http://localhost:13305".to_string())
    }

    /// Normalize model ID for Lemonade: the server uses hyphens for custom
    /// models (e.g. `model-Q4_K_M`) but Ollama-style models use colons
    /// (e.g. `model:Q4_K_M`). Only normalize if the model isn't found as-is.
    fn normalize_lemonade_model_id(model: &str) -> String {
        // Don't normalize — Lemonade already uses the correct format.
        // Custom models use hyphens, official models use colons.
        // The resolve_lemonade_chat_model function handles the lookup.
        model.to_string()
    }
}

#[cfg(test)]
mod model_classification_tests {
    use super::*;

    #[test]
    fn param_count_parses_common_names() {
        assert_eq!(Sentient::parse_model_param_count("qwen2.5-coder:7b"), Some(7));
        assert_eq!(Sentient::parse_model_param_count("qwen2.5-coder:14b"), Some(14));
        assert_eq!(Sentient::parse_model_param_count("mistral-small:24b"), Some(24));
        assert_eq!(Sentient::parse_model_param_count("yi:34b"), Some(34));
        assert_eq!(Sentient::parse_model_param_count("gemma-4-12b-coder-fable5"), Some(12));
        assert_eq!(Sentient::parse_model_param_count("qwen3:30b-a3b"), Some(30));
        assert_eq!(Sentient::parse_model_param_count("codestral"), None);
    }

    #[test]
    fn small_models_are_flagged_small() {
        for m in [
            "tinyllama:1.1b",
            "qwen2.5-coder:3b",
            "gemma2:2b",
            "phi-3-mini",
            "qwen2.5:0.5b",
            "gemma-4-e2b",
            "gemma-4-e4b",
        ] {
            assert!(Sentient::is_small_model_name(m), "{m} should be small");
        }
    }

    #[test]
    fn mid_and_large_models_are_not_small() {
        // Regression: bare "4b"/"3b"/"2b" substrings inside 14b/24b/34b/12b
        // previously misclassified capable models as small, blocking the native
        // tool-calling path that bug-bounty PoC workflows depend on.
        for m in [
            "qwen2.5-coder:14b",
            "mistral-small:24b",
            "yi:34b",
            "deepseek-coder:33b",
            "gemma-4-12b-coder-fable5-composer2.5-v1",
            "qwen3:32b",
            "llama3.1:8b",
        ] {
            assert!(!Sentient::is_small_model_name(m), "{m} should NOT be small");
        }
    }
}
