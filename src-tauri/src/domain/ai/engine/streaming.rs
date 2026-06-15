//! chat_complete and single-shot completion streaming paths.
use anyhow::{anyhow, Result};
use futures::StreamExt;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use super::types::*;
use super::sentient::Sentient;

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

        let _keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
        let (provider, model) = if let Some(p) = provider_override {
             // User explicitly specified provider:model
             if let Some((prov, m)) = p.split_once(':') {
                 (prov.to_string(), m.to_string())
             } else {
                 (p, model_override.unwrap_or_else(|| "qwen3.5:12b".to_string()))
             }
        } else if let Ok(p) = std::env::var("AI_PROVIDER") {
             // Environment variable override
             (p, std::env::var("AI_MODEL").unwrap_or_else(|_| "qwen3.5:12b".to_string()))
        } else {
            // DEFAULT: Prefer local Ollama for offline-first mode
            // Priority: Ollama (local) > External APIs (only if configured)
            ("ollama".to_string(), "qwen3.5:12b".to_string())
        };

        // Validate selected model exists in local Ollama
        let model = if provider == "ollama" {
            // Try to use user's model, fallback to qwen3.5:12b
            if model.is_empty() || model == "gpt-4o" {
                "qwen3.5:12b".to_string()
            } else {
                model
            }
        } else {
            model
        };

        let ollama_url = {
            let u = self.ollama_url.lock().await;
            normalize_ollama_base_url(&u)
        };

        let req = AiRequest {
            provider,
            model,
            messages,
            temperature: Some(0.7),
            autonomous: false,
            mode: None,
            cyber_mode: None,
            root_access: None,
            ollama_url: Some(ollama_url),
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
        let mut state = self.conversation_state.lock().await;
        // Hard cap: never allow more than 100 messages in conversation state.
        // Phase-wrap should already keep it tiny, but this catches edge cases.
        if state.len() > 100 {
            // Extract system message BEFORE mutating the vec
            let sys = state.iter().find(|m| m.role == "system").cloned();
            let keep_from = state.len().saturating_sub(50);
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

        // This ensures they stay in RAM but at 4x lower density for 8GB systems.
        let state_json = serde_json::to_string(&*state).unwrap_or_default();

        let pressure = self.perf_monitor.get_memory_pressure().await;
        let threshold = match pressure {
            crate::performance::MemoryPressure::Normal => 32768,
            crate::performance::MemoryPressure::Warning => 16384,
            crate::performance::MemoryPressure::Critical => 8192,
        };

        if state_json.len() > threshold {
            // We vault the full state before truncation if it's large
            let vault_key = format!("history_vault_{}", self.session_id);
            let _ = self
                .memory_optimizer
                .store_high_density(&vault_key, &state_json)
                .await;
            println!(
                "[AI] Session history vaulted via TurboQuant SCQ index: {} (Level: {:?})",
                vault_key, pressure
            );
        }

        // Keep system prompt, first user message, and last 3 messages
        let system_msg = state.iter().find(|m| m.role == "system").cloned();
        let last_messages: Vec<ChatMessage> = state.iter().rev().take(3).rev().cloned().collect();

        let mut new_state = Vec::new();
        if let Some(s) = system_msg {
            new_state.push(s);
        }

        new_state.extend(last_messages);
        *state = new_state;

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
    pub async fn single_shot_completion(&self, req: AiRequest) -> Result<String> {
        let effective_provider = if req.model.to_lowercase().contains("claude-opus-4-8") {
            "highwayapi"
        } else {
            req.provider.as_str()
        };
        let effective_provider_lc = effective_provider.to_lowercase();
        let is_ollama = effective_provider_lc == "ollama" || effective_provider_lc == "antigravity";
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
            let keys_path = self.brain_dir.parent().unwrap().join("api_keys.json");
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

        let payload = if is_ollama {
            let is_vision = Self::is_vision_model(&req.model);
            let ollama_openai_compat =
                self.ollama_use_openai_compat_endpoint(&req, &req.model).await;
            let messages =
                Self::build_ollama_messages(&req.messages, is_vision, ollama_openai_compat);
            let temp = req.temperature.unwrap_or(0.1);
            let chat_stream = req.feature.as_deref() == Some("Chat");
            let mut body = json!({
                "model": req.model,
                "messages": messages,
                "temperature": temp,
                "stream": chat_stream,
            });
            if !ollama_openai_compat {
                body["options"] = Self::ollama_inference_options(&req.model, temp, 1024);
                body["keep_alive"] = json!(crate::ollama_offload::keep_alive());
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
            let ollama_openai_compat =
                self.ollama_use_openai_compat_endpoint(&req, &req.model).await;
            let api_messages = Self::build_ollama_messages(
                if use_top_level { &conv_messages } else { &trimmed },
                is_vision,
                ollama_openai_compat,
            );
            let mut body = json!({
                "model": req.model,
                "messages": api_messages,
                "stream": false,
            });
            if use_top_level {
                if !system_text.trim().is_empty() {
                    body["system"] = json!(system_text);
                }
                body["max_tokens"] = json!(16000);
                if !is_opus_48_model(&req.model) {
                    body["temperature"] = json!(req.temperature.unwrap_or(0.1));
                }
            } else {
                body["temperature"] = json!(req.temperature.unwrap_or(0.1));
            }
            body
        };

        let ollama_base_for_auth = if effective_provider_lc == "ollama" {
            let raw = if let Some(ref u) = req.ollama_url {
                u.clone()
            } else {
                self.ollama_url.lock().await.clone()
            };
            normalize_ollama_base_url(&raw)
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
        } else if effective_provider_lc == "ollama" {
            let k = self.ollama_bearer_for_base(&ollama_base_for_auth);
            if !k.trim().is_empty() {
                request = request.bearer_auth(k.trim());
            }
        } else if !is_ollama {
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
                endpoint.replace(":1536", ":11434")
            };

            println!("[AI] Proxy port 1536 unreachable in single_shot_completion, retrying directly on fallback: {}", fallback_endpoint);
            let mut fallback_request = self.client.post(fallback_endpoint);
            
            if provider_lc == "google" || provider_lc == "gemini" {
                fallback_request = fallback_request.bearer_auth(&key)
                                                     .header("x-goog-api-key", &key);
            } else if provider_lc == "ollama" {
                let k = self.ollama_bearer_for_base(&ollama_base_for_auth);
                if !k.trim().is_empty() {
                    fallback_request = fallback_request.bearer_auth(k.trim());
                }
            } else if !is_ollama {
                fallback_request = fallback_request.bearer_auth(&key);
            }
            response_result = fallback_request.json(&payload).send().await;
        }

        let resp = response_result.map_err(|e| anyhow!("FIM HTTP request failed: {}", e))?;
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Chat request failed: {}", body));
        }

        let chat_stream = is_ollama && req.feature.as_deref() == Some("Chat");
        if chat_stream {
            return self.single_shot_ollama_stream(resp).await;
        }

        let val: Value = resp.json().await?;
        
        let raw = if effective_provider_lc == "anthropic" {
            val["content"][0]["text"].as_str().unwrap_or("").to_string()
        } else if is_ollama {
            // Ollama might be hit via /v1/chat/completions (OpenAI format) or /api/chat (Native format)
            if let Some(content) = val.pointer("/choices/0/message/content").and_then(|v| v.as_str()) {
                content.to_string()
            } else if let Some(content) = val.pointer("/message/content").and_then(|v| v.as_str()) {
                content.to_string()
            } else {
                "".to_string()
            }
        } else {
            val["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string()
        };

        Ok(raw.trim().to_string())
    }

    /// Stream Ollama /api/chat for fast Chat replies — tokens land in `chat_stream_buf`.
    pub(crate) async fn single_shot_ollama_stream(&self, response: reqwest::Response) -> Result<String> {
        if let Ok(mut b) = self.chat_stream_buf.lock() {
            b.clear();
        }
        let mut full = String::new();
        let mut stream = response.bytes_stream();
        let mut line_buf = String::new();
        'stream: loop {
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
                let content = val.pointer("/choices/0/message/content")
                    .and_then(|v| v.as_str())
                    .or_else(|| val.pointer("/message/content").and_then(|v| v.as_str()))
                    .unwrap_or("");
                if !content.is_empty() {
                    full.push_str(content);
                    if let Ok(mut b) = self.chat_stream_buf.lock() {
                        b.push_str(content);
                    }
                }
            }
        }
        Ok(full.trim().to_string())
    }

}
