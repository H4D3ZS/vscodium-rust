//! Provider plumbing: Ollama payloads/options/diagnostics/native API,
//! model pulling/listing, intent classification, local fallback.
use anyhow::{anyhow, Result};
use futures::StreamExt;
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
    pub(crate) fn ollama_sampling(model: &str, is_chat_mode: bool, req_temp: Option<f32>) -> (f32, f32, u32) {
        if Self::is_gemma4_model(model) {
            // https://ollama.com/library/gemma4:12b — temp=1.0, top_p=0.95, top_k=64
            return (req_temp.unwrap_or(1.0), 0.95, 64);
        }
        let temp = if is_chat_mode {
            req_temp.unwrap_or(0.75)
        } else {
            req_temp.unwrap_or(0.6)
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
        // Explicit tiny tags
        if m.contains("mini") || m.contains("tiny") || m.contains("0.5b") || m.contains("1.5b")
            || m.contains("3b") || m.contains("4b")
        {
            return true;
        }
        // Gemma4 effective-param variants (e2b/e4b) behave like small models
        if (m.contains("gemma4") || m.contains("gemma-4"))
            && (m.contains("e2b") || m.contains("e4b") || m.contains("2b") || m.contains("4b"))
        {
            return true;
        }
        // Parse parameter count: ≤ 7B → small. 8B+ gets native tools.
        if let Some(n) = Self::parse_model_param_count(&m) {
            return n <= 7;
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
        crate::ollama_offload::clamp_num_ctx(Self::model_native_num_ctx(model))
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
        // Explicit GPU-layer offload override (Metal/partial offload tuning).
        if let Ok(n) = std::env::var("HADES_NUM_GPU") {
            if let Ok(n) = n.parse::<i64>() {
                opts["num_gpu"] = json!(n);
            }
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
                .or_insert_with(|| json!(crate::ollama_offload::keep_alive()));
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
        let base = self.resolved_ollama_base(req).await;
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

    pub async fn check_ollama_status(&self) -> Result<bool> {
        let url = {
            let u = self.ollama_url.lock().await;
            normalize_ollama_base_url(&u)
        };
        let mut urls = vec![url.clone()];
        if url.contains(":1536") {
            urls.push(url.replace(":1536", ":11434"));
        }
        for u in urls {
            for path in ["/api/tags", "/v1/api/tags"] {
                let mut req = self.client.get(format!("{}{}", u, path));
                let k = self.ollama_bearer_for_base(&u);
                if !k.trim().is_empty() {
                    req = req.bearer_auth(k.trim());
                }
                match req
                    .timeout(std::time::Duration::from_secs(3))
                    .send()
                    .await
                {
                    Ok(r) if r.status().is_success() => return Ok(true),
                    Ok(r) if r.status().as_u16() == 404 => continue, // try /v1 fallback
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    /// Probe the configured Ollama endpoint and report exactly what went wrong
    /// so the user doesn't have to guess between "wrong URL", "no bearer",
    /// "nginx returned HTML", "401" or "ollama empty".
    pub async fn diagnose_ollama(&self) -> serde_json::Value {
        let url_raw = self.ollama_url.lock().await.clone();
        let url_trim = normalize_ollama_base_url(&url_raw);

        let bearer = self.ollama_bearer_for_base(&url_trim);
        let bearer_configured = !bearer.trim().is_empty();

        // Probe `/api/tags` first (native Ollama). If the reverse proxy only
        // exposes the OpenAI-compatible `/v1/` surface (the default emitted by
        // `tools/vps-ollama-proxy/bootstrap.sh` before this patch), retry on
        // `/v1/api/tags` so the diagnostic and the model list both succeed
        // even on existing deployments that haven't been re-bootstrapped.
        let mut endpoint = format!("{}/api/tags", url_trim);
        let mut fallback_used = false;
        let mut req = self.client.get(&endpoint);
        if bearer_configured {
            req = req.bearer_auth(bearer.trim());
        }
        let mut resp = req
            .timeout(std::time::Duration::from_secs(6))
            .send()
            .await;

        if let Ok(ref r) = resp {
            if r.status().as_u16() == 404 {
                let alt = format!("{}/v1/api/tags", url_trim);
                let mut alt_req = self.client.get(&alt);
                if bearer_configured {
                    alt_req = alt_req.bearer_auth(bearer.trim());
                }
                if let Ok(alt_resp) = alt_req
                    .timeout(std::time::Duration::from_secs(6))
                    .send()
                    .await
                {
                    if alt_resp.status().is_success() {
                        endpoint = alt;
                        fallback_used = true;
                        resp = Ok(alt_resp);
                    }
                }
            }
        }

        match resp {
            Err(e) => {
                // Transport-level failure (DNS, TLS, connect, timeout).
                serde_json::json!({
                    "ok": false,
                    "url": url_trim,
                    "endpoint": endpoint,
                    "bearer_configured": bearer_configured,
                    "status": null,
                    "model_count": 0,
                    "models": serde_json::Value::Array(Vec::new()),
                    "error_kind": if e.is_timeout() { "timeout" }
                        else if e.is_connect() { "connect" }
                        else if e.is_request() { "request" }
                        else { "transport" },
                    "error": e.to_string(),
                    "hint": if e.to_string().contains("relative URL without a base") {
                        "The Ollama base URL must include a scheme. Try https://your-host (or http:// for localhost / LAN)."
                    } else if e.is_connect() {
                        "Cannot reach the URL. Check it resolves, is reachable from this machine, and that the port is open."
                    } else if e.is_timeout() {
                        "Reached the host but it didn't respond within 6s. Verify nginx/Ollama is actually running."
                    } else {
                        "Transport error before any HTTP status was received (often TLS or invalid URL)."
                    }
                })
            }
            Ok(r) => {
                let status = r.status();
                let status_code = status.as_u16();
                let content_type = r
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();
                let body = r.text().await.unwrap_or_default();
                let body_trim = body.trim();
                let body_preview: String = body_trim.chars().take(400).collect();
                let body_looks_html = body_trim.starts_with('<')
                    || content_type.to_lowercase().contains("text/html");

                let parsed: Result<serde_json::Value, _> = serde_json::from_str(&body);
                let models: Vec<String> = match &parsed {
                    Ok(v) => v
                        .get("models")
                        .and_then(|m| m.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|m| {
                                    m.get("name").and_then(|n| n.as_str()).map(|s| s.to_string())
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    Err(_) => Vec::new(),
                };

                let ok = status.is_success() && !models.is_empty();
                let hint = if !status.is_success() {
                    match status_code {
                        401 | 402 | 403 => Self::ollama_auth_hint(&url_trim, status_code),
                        404 => "Reached the server but neither /api/tags nor /v1/api/tags is exposed. Add a `location /api/` block to your nginx config (see tools/vps-ollama-proxy/bootstrap.sh).",
                        502 | 503 | 504 => "Nginx gateway error: upstream Ollama may be down, or nginx limit_conn/limit_req is throttling your client IP. Raise OLLAMA_CONN_PER_IP on the VPS (tools/vps-ollama-proxy/bootstrap.sh) and reload nginx.",
                        _ => "Server returned a non-2xx status. See the body preview below.",
                    }
                } else if body_looks_html {
                    "Endpoint returned HTML instead of JSON — almost always an nginx misroute or a captive-portal page in front of the proxy."
                } else if parsed.is_err() {
                    "Endpoint returned non-JSON. Check that nginx proxies /api/ directly to Ollama with no rewrites."
                } else if models.is_empty() {
                    "Connected fine but the server has zero models installed. Run `ollama pull <name>` on the VPS or click Pull New Model here."
                } else if fallback_used {
                    "Connected via the /v1/ fallback. Your nginx exposes only the OpenAI-compat surface — IDE will keep working, but to use the native Ollama CLI add a `location /api/` block (see tools/vps-ollama-proxy/bootstrap.sh)."
                } else {
                    "Connected. Models discovered."
                };

                serde_json::json!({
                    "ok": ok,
                    "url": url_trim,
                    "endpoint": endpoint,
                    "bearer_configured": bearer_configured,
                    "status": status_code,
                    "content_type": content_type,
                    "model_count": models.len(),
                    "models": models,
                    "body_preview": body_preview,
                    "body_looks_html": body_looks_html,
                    "fallback_used": fallback_used,
                    "hint": hint,
                })
            }
        }
    }

    /// `/api/foo` then `/v1/api/foo` on 404 (nginx rewrite; see `tools/vps-ollama-proxy/bootstrap.sh`).
    pub(crate) fn ollama_try_urls(base: &str, api_path: &str) -> Vec<String> {
        let p = api_path.trim();
        let p = if p.starts_with('/') {
            p.to_string()
        } else {
            format!("/{}", p)
        };
        let base = base.trim_end_matches('/');
        let mut urls = vec![format!("{}{}", base, p)];
        if let Some(rest) = p.strip_prefix("/api/") {
            urls.push(format!("{}/v1/api/{}", base, rest));
        }
        urls
    }

    /// Ollama GET from Rust so the webview is not subject to nginx CORS.
    pub async fn ollama_native_get(&self, path: String) -> Result<Value> {
        self.ollama_request("GET", &path, None).await
    }

    /// Ollama POST from Rust (same CORS bypass + `/v1` fallback as GET).
    pub async fn ollama_native_post(&self, path: String, body: Value) -> Result<Value> {
        self.ollama_request("POST", &path, Some(&body)).await
    }

    /// Unified Ollama HTTP request with retry, fallback URLs, and CORS bypass.
    async fn ollama_request(&self, method: &str, path: &str, body: Option<&Value>) -> Result<Value> {
        let _permit = self.ollama_http_permit().await;
        let base = {
            let u = self.ollama_url.lock().await;
            normalize_ollama_base_url(&u)
        };
        let bearer = self.ollama_bearer_for_base(&base).trim().to_string();
        let urls = Self::ollama_try_urls(&base, path);

        'attempt: for attempt in 0u32..6u32 {
            let mut last: Option<String> = None;
            for url in &urls {
                let mut req = match method {
                    "POST" => self.client.post(url),
                    _ => self.client.get(url),
                };
                if let Some(b) = body {
                    req = req.json(b);
                }
                if !bearer.is_empty() {
                    req = req.bearer_auth(&bearer);
                }
                match req.send().await {
                    Ok(resp) => {
                        let status = resp.status();
                        let code = status.as_u16();
                        let bytes = resp.bytes().await?;
                        if status.is_success() {
                            let v: Value = serde_json::from_slice(&bytes).map_err(|e| {
                                anyhow!("ollama_request JSON: {} (status {})", e, status)
                            })?;
                            return Ok(v);
                        }
                        if code == 404 && urls.len() > 1 && !url.contains("/v1/api/") {
                            last = Some(format!("{} {} -> 404", method, url));
                            continue;
                        }
                        if (code == 503 || code == 429) && attempt < 5 {
                            let ms = 400u64 * (1u64 << attempt).min(10_000);
                            tokio::time::sleep(Duration::from_millis(ms)).await;
                            continue 'attempt;
                        }
                        let preview: String =
                            String::from_utf8_lossy(&bytes).chars().take(280).collect();
                        return Err(anyhow!("{} {} -> {}: {}", method, url, status, preview));
                    }
                    Err(e) => {
                        last = Some(e.to_string());
                        continue;
                    }
                }
            }
            if attempt < 5 {
                if last.as_ref().map(|s| s.contains("503") || s.contains("429")).unwrap_or(false) {
                    let ms = 400u64 * (1u64 << attempt).min(10_000);
                    tokio::time::sleep(Duration::from_millis(ms)).await;
                    continue 'attempt;
                }
            }
            return Err(anyhow!(
                "ollama_request({}) exhausted fallbacks: {}",
                method,
                last.unwrap_or_default()
            ));
        }
        unreachable!()
    }

    pub async fn pull_model(&self, name: &str) -> Result<()> {
        let url = {
            let u = self.ollama_url.lock().await;
            normalize_ollama_base_url(&u)
        };

        let payload = json!({ "name": name, "stream": false });
        let key = self.ollama_bearer_for_base(&url);
        let bearer = key.trim();
        for path in ["/api/pull", "/v1/api/pull"] {
            let mut req = self
                .client
                .post(format!("{}{}", url, path))
                .json(&payload);
            if !bearer.is_empty() {
                req = req.bearer_auth(bearer);
            }
            let resp = req.send().await?;
            if resp.status().is_success() {
                return Ok(());
            }
            if resp.status().as_u16() != 404 {
                return Err(anyhow!("Failed to pull model: {}", resp.status()));
            }
        }
        Err(anyhow!(
            "Failed to pull model: neither /api/pull nor /v1/api/pull is exposed on {}",
            url
        ))
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

    /// Pick an installed local Ollama model. Retained as a utility; no longer
    /// used for auto-fallback (we never switch the user's chosen model).
    #[allow(dead_code)]
    pub(crate) async fn pick_local_fallback_model(&self) -> Option<String> {
        let models = self.list_models("ollama").await.ok()?;
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
            let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
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
                "ollama" | "antigravity" | "mimo" | "xiaomi"
                | "cyberifrit" | "cyber-ifrit" | "cyberifrit-cloud");
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
                // Graceful fallbacks so the picker is never broken.
                if is_mimo {
                    return Ok(vec!["mimo-v2.5-pro".to_string(), "mimo-v2.5".to_string()]);
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

        if provider.to_lowercase() == "ollama" {
            let _permit = self.ollama_http_permit().await;
            let base = {
                let u = self.ollama_url.lock().await;
                normalize_ollama_base_url(&u)
            };
            let mut bases = vec![base.clone()];
            if base.contains(":1536") {
                bases.push(base.replace(":1536", ":11434"));
            }
            let mut last_err: Option<String> = None;
            for current_base in &bases {
                let bearer = self.ollama_bearer_for_base(current_base);
                let bearer = bearer.trim();

                'attempt: for attempt in 0u32..6u32 {
                    for path in ["/api/tags", "/v1/api/tags"] {
                        let url = format!("{}{}", current_base, path);
                        let mut req = self.client.get(&url);
                        if !bearer.is_empty() {
                            req = req.bearer_auth(bearer);
                        }
                        match req.send().await {
                            Ok(resp) if resp.status().is_success() => {
                                let json: Value = resp.json().await?;
                                let mut model_names = Vec::new();
                                if let Some(models) = json.get("models").and_then(|m| m.as_array()) {
                                    for m in models {
                                        if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                                            model_names.push(sanitize_ollama_model_id(name));
                                        }
                                    }
                                }
                                model_names = model_names
                                    .into_iter()
                                    .map(|m| sanitize_ollama_model_id(&m))
                                    .collect();
                                return Ok(model_names);
                            }
                            Ok(resp) if resp.status().as_u16() == 404 => {
                                last_err = Some(format!("{} returned 404", url));
                                continue;
                            }
                            Ok(resp) => {
                                let status = resp.status();
                                let code = status.as_u16();
                                if (code == 503 || code == 429) && attempt < 5 {
                                    let ms = 400u64 * (1u64 << attempt).min(10_000);
                                    tokio::time::sleep(Duration::from_millis(ms)).await;
                                    continue 'attempt;
                                }
                                let body = resp.text().await.unwrap_or_default();
                                last_err = Some(format!("{} -> {}: {}", url, status, body.chars().take(100).collect::<String>()));
                                continue;
                            }
                            Err(e) => {
                                last_err = Some(e.to_string());
                                continue;
                            }
                        }
                    }
                }
            }
            return Err(anyhow!(
                "Ollama list_models: neither /api/tags nor /v1/api/tags reachable on any of bases {:?} (Last error: {})",
                bases,
                last_err.clone().unwrap_or_default()
            ));
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
        } else if provider.to_lowercase() == "anthropic" {
            request = request
                .header("x-api-key", &provider_key)
                .header("anthropic-version", "2023-06-01");
        } else if provider.to_lowercase() == "ollama" {
            // No auth
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
            "ollama" => {
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
