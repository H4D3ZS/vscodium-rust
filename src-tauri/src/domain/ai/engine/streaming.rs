//! chat_complete and single-shot completion streaming paths.
use anyhow::{anyhow, Result};
use futures::StreamExt;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use super::types::*;
use super::sentient::Sentient;

/// Maximum bytes held in the live chat stream buffer. Matches the autonomous
/// loop (`autonomous.rs`). The frontend drains this every ~200ms via
/// `chat_stream_drain`, but if the poller is paused (tab hidden, panel closed,
/// poll error) the buffer must still be bounded or a verbose model can grow it
/// to hundreds of MB between drains. Keep the tail (most recent tokens) so the
/// rendered stream stays coherent after a wrap.
const MAX_CHAT_STREAM_BUF: usize = 512_000;

/// Append `chunk` to the shared chat-stream buffer, keeping it capped to
/// `MAX_CHAT_STREAM_BUF` bytes (ring buffer: drop oldest excess).
fn push_chat_stream(buf: &Arc<std::sync::Mutex<String>>, chunk: &str) {
    if chunk.is_empty() {
        return;
    }
    if let Ok(mut b) = buf.lock() {
        let new_len = b.len() + chunk.len();
        if new_len > MAX_CHAT_STREAM_BUF {
            // Keep the most recent bytes; drop the oldest excess.
            let keep = MAX_CHAT_STREAM_BUF.saturating_sub(chunk.len());
            let cur_len = b.len();
            if keep < cur_len {
                b.drain(0..(cur_len - keep));
            }
        }
        b.push_str(chunk);
    }
}

impl Sentient {
    pub async fn chat_complete(
        self: Arc<Self>, 
        prompt: &str, 
        system_override: Option<String>,
        provider_override: Option<String>,
        model_override: Option<String>,
        on_chunk: Option<Arc<dyn Fn(&str) + Send + Sync>>
    ) -> Result<AiResponse> {
        let mut messages = Vec::new();
        if let Some(sys) = system_override {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(sys)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            });
        }
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: Some(MessageContent::Text(prompt.to_string())),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        });

        let _keys_path = self.api_keys_path();
        let (provider, model) = if let Some(p) = provider_override {
             // User explicitly specified provider:model
             if let Some((prov, m)) = p.split_once(':') {
                 (prov.to_string(), m.to_string())
             } else {
                 (p, model_override.unwrap_or_else(|| "".to_string()))
             }
        } else if let Ok(p) = std::env::var("AI_PROVIDER") {
             // Environment variable override
             (p, std::env::var("AI_MODEL").unwrap_or_else(|_| "".to_string()))
        } else {
            // DEFAULT: local-first. Lemonade is the only local backend; leaving the
            // model empty lets the caller's configured Lemonade model win.
            ("lemonade".to_string(), String::new())
        };

        // A stale cloud default (`gpt-4o`) is not something Lemonade serves; blank
        // it so the request carries the configured local model instead.
        let model = if provider == "lemonade" && model == "gpt-4o" {
            String::new()
        } else {
            model
        };

        let local_url = self.lemonade_base().await;

        let req = AiRequest {
            provider,
            model,
            messages,
            temperature: Some(0.7),
            autonomous: false,
            mode: None,
            cyber_mode: None,
            root_access: None,
            inference_url: Some(local_url),
            tools: None,
            reasoning_budget: None,
            reasoning_effort: None,
            reasoning_enabled: None,
            feature: None,
        };

        let result = self.clone().autonomous_loop(req, on_chunk).await?;
        Ok(AiResponse { content: result })
    }

    /// Clear the in-memory conversation history. Used by the IDE's
    /// "clear AI memory / new chat" action. Does not touch persisted .aim
    /// slots — that is the caller's choice (see `clear_ai_memory` command).
    pub async fn clear_conversation(&self) {
        let mut state = self.conversation_state.lock().await;
        state.clear();
    }

    pub async fn optimize_memory(&self) -> Result<()> {
        // ── Phase 1: snapshot + truncate under lock (fast, no async I/O) ──
        let (vault_json, pressure);
        {
            let mut state = self.conversation_state.lock().await;
            // Hard cap: 30 messages for small models, 100 for large.
            if state.len() > 30 {
                let sys = state.iter().find(|m| m.role == "system").cloned();
                let keep_from = state.len().saturating_sub(15);
                let keep: Vec<ChatMessage> = state.drain(keep_from..).collect();
                state.clear();
                if let Some(s) = sys { state.push(s); }
                state.extend(keep);
            }
            if state.len() <= 5 {
                return Ok(());
            }

            println!(
                "[AI] Optimizing memory: summarizing history of {} messages",
                state.len()
            );

            vault_json = serde_json::to_string(&*state).unwrap_or_default();
            pressure = self.perf_monitor.get_memory_pressure().await;

            // Truncate in-place while we hold the lock — cheap O(n) clone.
            let system_msg = state.iter().find(|m| m.role == "system").cloned();
            let last_messages: Vec<ChatMessage> = state.iter().rev().take(3).rev().cloned().collect();
            let mut new_state = Vec::new();
            if let Some(s) = system_msg { new_state.push(s); }
            new_state.extend(last_messages);
            *state = new_state;
        }
        // Lock is released — vaulting runs without blocking the next turn.

        // ── Phase 2: vault the snapshot (CPU-heavy: FHT + quantize + LZ4) ──
        let threshold = match pressure {
            crate::performance::MemoryPressure::Normal => 32768,
            crate::performance::MemoryPressure::Warning => 16384,
            crate::performance::MemoryPressure::Critical => 8192,
        };

        if vault_json.len() > threshold {
            let vault_key = format!("history_vault_{}", self.session_id);
            let _ = self
                .memory_optimizer
                .store_high_density(&vault_key, &vault_json)
                .await;
            println!(
                "[AI] Session history vaulted via TurboQuant SCQ index: {} (Level: {:?})",
                vault_key, pressure
            );
        }

        Ok(())
    }

    /// Phase-Wrap: compress the current context window into .aim and reset to a fresh window.
    /// Called every PHASE_WRAP_EVERY iterations to keep the context window small ("1 gist token").
    /// After compression, the messages vec is reset to:
    ///   [system_identity, compact_brain_gist, last_user_mission]
    /// The AI never loses memory because everything is in .aim.
    pub(crate) async fn auto_phase_wrap(
        &self,
        messages: &mut Vec<ChatMessage>,
        iteration: u32,
        files_written: &[String],
    ) {
        // Build a summary of what happened in the last context window
        let context_snapshot = messages.iter()
            .filter(|m| m.role == "assistant" || (m.role == "tool" && m.tool_call_id.is_some()))
            .rev()
            .take(6)
            .map(|m| {
                let content = m.content.as_ref().map(|c| c.as_str()).unwrap_or("");
                let snippet = content.chars().take(120).collect::<String>().replace('\n', " ");
                format!("[{}] {}", m.role, snippet)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");

        if !context_snapshot.is_empty() {
            self.memory_store.store_phase_outcome(
                iteration,
                format!("Phase wrap at iter {}: {}", iteration, context_snapshot),
                files_written.to_vec(),
            ).await;
        }

        // Auto-learn from any file writes that happened in this phase
        for file in files_written {
            self.memory_store.auto_learn_from_write(file, "phase_wrap").await;
        }

        // Build compact brain gist for reinsertion (~100 tokens)
        let gist = self.memory_store.build_compact_gist().await;

        // Find the original system message and the last user message (the mission)
        let system_msg = messages.iter().find(|m| m.role == "system").cloned();
        let last_user_msg = messages.iter().rev().find(|m| m.role == "user").cloned();

        // Reset to minimal working set
        let mut new_messages: Vec<ChatMessage> = Vec::new();

        if let Some(sys) = system_msg {
            new_messages.push(sys);
        }

        // Inject compact brain gist as a system context message
        if !gist.is_empty() {
            new_messages.push(ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(format!(
                    "KORTEX BRAIN STATE (persistent memory from .aim):\n{}\n\nContinue the mission using this memory. You remember everything above.",
                    gist
                ))),
                tool_calls: None,
                tool_call_id: None,
                metadata: Some(serde_json::json!({"type": "brain_gist", "iteration": iteration})),
            });
        }

        // Reinsert the active mission so the AI knows what it's doing
        if let Some(user) = last_user_msg {
            new_messages.push(user);
        }

        *messages = new_messages;
        println!("[Phase-Wrap] Context compressed at iter {}. Window reset to {} messages. Brain gist: {} chars.",
            iteration, messages.len(), gist.len());
        self.emit_event("memory-update", serde_json::json!({
            "slots": self.memory_store.get_all_slots().await.len(),
            "phase_wrap": iteration,
            "type": "phase_wrap"
        }));
    }

}

impl Sentient {
    /// Optimized single-turn completion for low-latency FIM (Fill-In-Middle).
    /// Bypasses tool loading, autonomous verification, and memory injection.
    #[allow(unused_assignments)] // has_google_base_url flag is set in a vestigial branch
    pub async fn single_shot_completion(&self, mut req: AiRequest) -> Result<String> {
        // Resolve Lemonade omni collections → LLM component (see autonomous_loop).
        if req.provider.to_lowercase() == "lemonade" {
            let resolved = self.resolve_lemonade_chat_model(&req).await;
            if resolved != req.model {
                req.model = resolved;
            }
        }
        let effective_provider = if req.model.to_lowercase().contains("claude-opus-4-8") {
            "highwayapi"
        } else {
            req.provider.as_str()
        };
        let effective_provider_lc = effective_provider.to_lowercase();
        let is_local = effective_provider_lc == "antigravity" || effective_provider_lc == "lemonade";
        // Use standard chat endpoint for single-turn logic
        let endpoint = self.get_endpoint(effective_provider, &req);
        let key = self.get_key_for_provider(effective_provider).trim().to_string();

        let mut has_google_base_url = false;
        if let Ok(url) = std::env::var("GOOGLE_BASE_URL") {
            if !url.is_empty() {
                has_google_base_url = true;
            }
        }
        if !has_google_base_url {
            let keys_path = self.api_keys_path();
            if let Ok(content) = std::fs::read_to_string(&keys_path) {
                if let Ok(keys) = serde_json::from_str::<Value>(&content) {
                    if let Some(custom_url) = keys["google_base_url"].as_str() {
                        if !custom_url.is_empty() {
                            has_google_base_url = true;
                        }
                    }
                }
            }
        }

        let payload = if is_local {
            let is_vision = Self::is_vision_model(&req.model);
            let openai_compat =
                self.use_openai_compat_endpoint(&req, &req.model).await;
            let messages =
                Self::build_local_messages(&req.messages, is_vision, openai_compat);
            let temp = req.temperature.unwrap_or(0.1);
            let chat_stream = req.feature.as_deref() == Some("Chat");
            let mut body = json!({
                "model": req.model,
                "messages": messages,
                "temperature": temp,
                "stream": chat_stream,
            });
            if !openai_compat {
                body["options"] = Self::local_inference_options(&req.model, temp, 1024);
                body["keep_alive"] = json!(crate::gpu_offload::keep_alive());
            }
            body
        } else {
            let trimmed = if is_highway_family(effective_provider) {
                trim_assistant_prefill(&req.messages)
            } else {
                req.messages.clone()
            };
            let use_top_level =
                is_highway_family(effective_provider) || is_opus_48_model(&req.model);
            let (system_text, conv_messages) = if use_top_level {
                split_leading_system_messages(&trimmed)
            } else {
                (String::new(), trimmed.clone())
            };
            let is_vision = Self::is_vision_model(&req.model);
            let openai_compat =
                self.use_openai_compat_endpoint(&req, &req.model).await;
            let api_messages = Self::build_local_messages(
                if use_top_level { &conv_messages } else { &trimmed },
                is_vision,
                openai_compat,
            );
            let mut body = json!({
                "model": req.model,
                "messages": api_messages,
                "stream": false,
            });
            // Always bound generation. Without a cap, a thinking/reasoning model
            // (Lemonade Fable/Qwen) can loop indefinitely — the per-chunk stall
            // timeout never fires because tokens keep arriving. This was the
            // "thought for 58m → wall of slashes" failure.
            body["max_tokens"] = json!(16000);
            if use_top_level {
                if !system_text.trim().is_empty() {
                    body["system"] = json!(system_text);
                }
                if !is_opus_48_model(&req.model) {
                    body["temperature"] = json!(req.temperature.unwrap_or(0.1));
                }
            } else {
                body["temperature"] = json!(req.temperature.unwrap_or(0.1));
            }
            body
        };

        // Local backends resolve their bearer from the base URL (keyless for a
        // local Lemonade, JWT for a cloud one).
        let local_base_for_auth = if is_local {
            self.resolved_local_base(&req).await
        } else {
            String::new()
        };

        let mut request = self.client.post(endpoint.clone());
        if effective_provider_lc == "anthropic" {
            request = request
                .header("x-api-key", &key)
                .header("anthropic-version", "2023-06-01");
        } else if effective_provider_lc == "google" || effective_provider_lc == "gemini" {
            request = request.bearer_auth(&key)
                             .header("x-goog-api-key", &key);
        } else if matches!(effective_provider_lc.as_str(), "highwayapi" | "interfaceai" | "jiekou") {
            request = apply_highway_auth(request, &key);
        } else if is_local {
            let k = self.local_bearer_for_base(&local_base_for_auth);
            if !k.trim().is_empty() {
                request = request.bearer_auth(k.trim());
            }
        } else if !key.is_empty() {
            request = request.bearer_auth(&key);
        }

        let mut response_result = request
            .try_clone()
            .ok_or_else(|| anyhow!("Failed to clone request builder"))?
            .json(&payload)
            .send()
            .await;

        if response_result.is_err() && endpoint.contains(":1536") {
            let provider_lc = effective_provider_lc.clone();
            let fallback_endpoint = if provider_lc == "google" || provider_lc == "gemini" {
                "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions".to_string()
            } else if provider_lc == "openai" {
                "https://api.openai.com/v1/chat/completions".to_string()
            } else if provider_lc == "anthropic" {
                "https://api.anthropic.com/v1/messages".to_string()
            } else {
                endpoint.replace(":1536", ":13305")
            };

            println!("[AI] Proxy port 1536 unreachable in single_shot_completion, retrying directly on fallback: {}", fallback_endpoint);
            let mut fallback_request = self.client.post(fallback_endpoint);
            
            if provider_lc == "google" || provider_lc == "gemini" {
                fallback_request = fallback_request.bearer_auth(&key)
                                                     .header("x-goog-api-key", &key);
            } else if is_local {
                let k = self.local_bearer_for_base(&local_base_for_auth);
                if !k.trim().is_empty() {
                    fallback_request = fallback_request.bearer_auth(k.trim());
                }
            } else if !key.is_empty() {
                fallback_request = fallback_request.bearer_auth(&key);
            }
            response_result = fallback_request.json(&payload).send().await;
        }

        let mut resp = response_result.map_err(|e| anyhow!("FIM HTTP request failed: {}", e))?;
        // Lemonade fallback: some deployments (cloud gate / older builds) only
        // expose /v1/chat/completions instead of /api/v1/chat/completions.
        if resp.status().as_u16() == 404
            && effective_provider_lc == "lemonade"
            && endpoint.contains("/api/v1/chat/completions")
        {
            let alt = endpoint.replace("/api/v1/chat/completions", "/v1/chat/completions");
            let mut retry = self.client.post(alt);
            if !key.is_empty() {
                retry = retry.bearer_auth(&key);
            }
            resp = retry.json(&payload).send().await
                .map_err(|e| anyhow!("Lemonade /v1 fallback failed: {}", e))?;
        }
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Chat request failed: {}", body));
        }

        let chat_stream = is_local && req.feature.as_deref() == Some("Chat");
        if chat_stream {
            return self.single_shot_native_stream(resp).await;
        }

        let val: Value = resp.json().await?;
        
        let raw = if effective_provider_lc == "anthropic" {
            val["content"][0]["text"].as_str().unwrap_or("").to_string()
        } else if is_local {
            // the local backend might be hit via /v1/chat/completions (OpenAI format) or /api/chat (Native format)
            if let Some(content) = val.pointer("/choices/0/message/content").and_then(|v| v.as_str()) {
                content.to_string()
            } else if let Some(content) = val.pointer("/message/content").and_then(|v| v.as_str()) {
                content.to_string()
            } else {
                "".to_string()
            }
        } else {
            // Check standard content first, then reasoning_content (Lemonade thinking models)
            let content = val["choices"][0]["message"]["content"].as_str().unwrap_or("");
            if content.is_empty() {
                val["choices"][0]["message"]["reasoning_content"].as_str().unwrap_or("").to_string()
            } else {
                content.to_string()
            }
        };

        Ok(raw.trim().to_string())
    }

    /// Stream the local backend /api/chat for fast Chat replies — tokens land in `chat_stream_buf`.
    pub(crate) async fn single_shot_native_stream(&self, response: reqwest::Response) -> Result<String> {
        if let Ok(mut b) = self.chat_stream_buf.lock() {
            b.clear();
        }
        let mut full = String::new();
        // Reasoning (thinking) tokens are shown in the collapsible thinking
        // channel, NOT concatenated into the answer — a runaway reasoning loop
        // was leaking straight into the reply (the "wall of slashes"). Kept as a
        // fallback only if the model emits no regular content.
        let mut reasoning_buf = String::new();
        let mut stream = response.bytes_stream();
        let mut line_buf = String::new();
        // Hard wall-clock cap. The per-chunk stall timeout can't stop a model
        // that keeps emitting tokens forever, so bound the whole generation.
        let start = std::time::Instant::now();
        const MAX_GEN: Duration = Duration::from_secs(600);
        'stream: loop {
            if start.elapsed() > MAX_GEN {
                self.emit_event("ai-thinking", json!({ "thought": "[stopped: generation exceeded 10-minute cap]" }));
                break 'stream;
            }
            let next = tokio::time::timeout(Duration::from_secs(180), stream.next()).await;
            let chunk = match next {
                Ok(Some(Ok(c))) => c,
                Ok(Some(Err(e))) => return Err(anyhow!("Chat stream error: {}", e)),
                Ok(None) => break,
                Err(_) => return Err(anyhow!("Chat stream timed out")),
            };
            line_buf.push_str(&String::from_utf8_lossy(&chunk));
            while let Some(pos) = line_buf.find('\n') {
                let line = line_buf[..pos].trim().to_string();
                line_buf.drain(..=pos);
                if line.is_empty() || line == "data: [DONE]" {
                    continue;
                }
                let json_str = line.strip_prefix("data: ").unwrap_or(&line);
                let val: Value = match serde_json::from_str(json_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if val["done"].as_bool() == Some(true) {
                    break 'stream;
                }
                if let Some(thinking) = val["message"]["thinking"].as_str() {
                    if !thinking.is_empty() {
                        self.emit_event("ai-thinking", json!({ "thought": thinking }));
                    }
                }
                // Standard content — OpenAI SSE deltas (Lemonade, compat proxies)
                // or native NDJSON message objects.
                let content = val.pointer("/choices/0/delta/content")
                    .and_then(|v| v.as_str())
                    .or_else(|| val.pointer("/choices/0/message/content").and_then(|v| v.as_str()))
                    .or_else(|| val.pointer("/message/content").and_then(|v| v.as_str()))
                    .unwrap_or("");
                if !content.is_empty() {
                    full.push_str(content);
                    push_chat_stream(&self.chat_stream_buf, content);
                } else {
                    // Lemonade reasoning_content format (thinking models)
                    let reasoning = val.pointer("/choices/0/delta/reasoning_content")
                        .and_then(|v| v.as_str())
                        .or_else(|| val.pointer("/choices/0/message/reasoning_content").and_then(|v| v.as_str()))
                        .or_else(|| val.pointer("/message/reasoning_content").and_then(|v| v.as_str()))
                        .unwrap_or("");
                    if !reasoning.is_empty() {
                        self.emit_event("ai-thinking", json!({ "thought": reasoning }));
                        reasoning_buf.push_str(reasoning);
                    }
                }
            }
        }
        // Fall back to the reasoning text only when the model produced no real
        // answer content (some thinking models put everything in that channel).
        let out = if full.trim().is_empty() { reasoning_buf } else { full };
        Ok(out.trim().to_string())
    }

}
