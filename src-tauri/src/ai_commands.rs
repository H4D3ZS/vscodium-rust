use crate::EditorState;
use crate::ai_engine::{AiRequest, ChatMessage, MessageContent, AiResponse, normalize_ollama_base_url};
use crate::ripgrep_search::{self, RipgrepQuery};
pub use crate::ripgrep_search::SearchResult;
use tauri::{State, AppHandle, Emitter};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::fs;

#[tauri::command]
pub async fn grep_files(
    state: State<'_, EditorState>,
    pattern: String,
    path: Option<String>,
    include: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state
            .active_root
            .lock()
            .await
            .clone()
            .unwrap_or_else(|| PathBuf::from("."))
    };

    ripgrep_search::ripgrep_search(RipgrepQuery {
        pattern: &pattern,
        root: &root,
        include: include.as_deref(),
        max_results: 100,
        case_insensitive: true,
        fixed_string: false,
        file: None,
    })
}

#[tauri::command]
pub async fn set_ai_status(
    _state: State<'_, EditorState>,
    status: String,
) -> Result<(), String> {
    println!("[AI Status] Updated to: {}", status);
    Ok(())
}

#[tauri::command]
pub async fn ai_tool_result(
    _state: State<'_, EditorState>,
    call_id: String,
    result: String,
) -> Result<(), String> {
    eprintln!(
        "[Tool Result] call_id={}, result_len={}",
        call_id,
        result.len()
    );
    Ok(())
}

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    state: State<'_, EditorState>, 
    mut request: AiRequest
) -> Result<String, String> {
    let log_entry = format!("[ai_chat] REQUEST: {:?}\n", request);
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(log_entry.as_bytes()) });

    // Signal Kairos: user is actively using AI â€” reset idle timer
    state.kairos.report_activity().await;

    // Ensure Ollama cloud/local URL is on the request (agent loop bearer auth uses it).
    if request.ollama_url.as_ref().map(|u| u.trim().is_empty()).unwrap_or(true) {
        let url = state.ollama_url.lock().await.clone();
        if !url.trim().is_empty() {
            request.ollama_url = Some(url);
        }
    }

    // Update MemoryLayer state â€” agent is now active
    let _ = state.memory_layer.update_state("Active", &format!("Processing: {}", 
        request.messages.last()
            .and_then(|m| m.content.as_ref().map(|c| c.to_text()))
            .unwrap_or_default()
            .chars().take(80).collect::<String>()
    ));

    // Inject the Hades persistent memory context as a system message
    if let Ok(hades_ctx) = state.memory_layer.get_aggregate_context() {
        if !hades_ctx.trim().is_empty() {
            request.messages.insert(0, ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(hades_ctx)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            });
        }
    }

    // Set up chunk callback for real-time UI streaming
    // Clone app BEFORE moving it into the Arc so we can emit the final ai-content event
    // directly after the loop completes, bypassing the silent_emits suppression used
    // by background AIRI autonomous tasks.
    let app_for_final = app.clone();
    let app_handle = std::sync::Arc::new(app);
    let accumulated = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let accumulated_clone = accumulated.clone();

    let on_chunk = Some(std::sync::Arc::new(move |chunk: &str| {
        if !chunk.is_empty() {
            let _ = accumulated_clone.lock().map(|mut acc| acc.push_str(chunk));
        }
        let _ = app_handle.emit("ai-content-delta", serde_json::json!({ "delta": chunk }));
    }) as std::sync::Arc<dyn Fn(&str) + Send + Sync>);

    // Clear any stale streamed tokens from a prior turn before this one starts.
    if let Ok(mut b) = state.ai_engine.chat_stream_buf.lock() { b.clear(); }

    let result = state
        .ai_engine
        .clone()
        .autonomous_loop(request, on_chunk)
        .await
        .map_err(|e| {
            let err_log = format!("[ai_chat] ERROR: {}\n", e);
            eprintln!("{}", err_log.trim());
            let _ = std::fs::OpenOptions::new()
                .create(true).append(true)
                .open("ai_chat.log")
                .and_then(|mut f| { use std::io::Write; f.write_all(err_log.as_bytes()) });
            e.to_string()
        })?;

    // If result is empty but we streamed content, use accumulated chunks
    let final_response = if result.trim().is_empty() {
        accumulated
            .lock()
            .ok()
            .map(|acc| acc.trim().to_string())
            .unwrap_or_default()
    } else {
        result.clone()
    };

    // ALWAYS emit response to frontend (CRITICAL: use accumulated if result empty)
    let trimmed = final_response.trim();
    let emit_result = app_for_final.emit("ai-content", serde_json::json!({ "content": trimmed }));

    if let Err(e) = &emit_result {
        eprintln!("[ai_chat] EMIT FAILED: {}", e);
    } else {
        eprintln!("[ai_chat] Response emitted to frontend");
    }

    let done_log = format!(
        "[ai_chat] DONE: result_len={}, final_len={}, emit_ok={}\n",
        result.len(),
        trimmed.len(),
        emit_result.is_ok()
    );
    eprintln!("{}", done_log.trim());
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(done_log.as_bytes()) });
        
    // Update MemoryLayer: agent completed the task
    let _ = state.memory_layer.update_state("Idle", "Task completed");

    // Trim conversation state after each agent turn to keep RSS bounded.
    let engine_clone = state.ai_engine.clone();
    tauri::async_runtime::spawn(async move {
        let _ = engine_clone.optimize_memory().await;
    });

    // Satisfy AiResponse usage warning
    let _response = AiResponse { content: final_response.clone() };

    Ok(final_response)
}

/// Trivial-chat fast path. Skips the autonomous loop, the phase
/// machinery, tool catalog construction, system-prompt assembly, and
/// every retry. One HTTP round-trip to the provider and we're done.
/// Used by the frontend for short, action-less prompts like "hello"
/// where running the full agent loop is gross overkill (and was making
/// "hi" take 5+ seconds while the model dutifully ran git_status and
/// grep). Cursor's Agent mode behaves the same way â€” it doesn't open
/// the codebase index for a greeting.
#[tauri::command]
pub async fn ai_chat_fast(
    state: State<'_, EditorState>,
    request: AiRequest,
) -> Result<String, String> {
    state.kairos.report_activity().await;

    let engine = state.ai_engine.clone();
    if let Ok(mut b) = engine.chat_stream_buf.lock() {
        b.clear();
    }
    // We deliberately do NOT inject the heavy Hades context here; the
    // whole point is sub-second latency. The conversation history the
    // frontend already passed is enough for trivial chat.
    let result = engine
        .single_shot_completion(request)
        .await
        .map_err(|e| e.to_string())?;

    // Frontend polls `chat_stream_drain` during fast chat — merge any tail
    // still in the buffer with the HTTP return value.
    let drained = {
        let mut b = engine
            .chat_stream_buf
            .lock()
            .map_err(|e| format!("chat_stream_buf lock poisoned: {e}"))?;
        std::mem::take(&mut *b)
    };
    let final_response = if result.trim().is_empty() {
        drained.trim().to_string()
    } else {
        result
    };

    // Keep the existing UI plumbing happy: every other code path
    // delivers the response via the `ai-content` event, so emit it
    // here too. The frontend `ai-content` listener calls
    // `updateLastAgentMessage` and flips `isAgentThinking` off.
    engine.emit_event(
        "ai-content",
        serde_json::json!({ "content": final_response.clone() }),
    );

    Ok(final_response)
}

/// Background-agent entry point. Same engine, same tool surface as
/// `ai_chat`, but every `emit_event` is suppressed for the duration of
/// the run so the foreground chat UI keeps streaming whatever it was
/// already showing. The frontend `runBackgroundAgent` slice calls this
/// to dispatch parallel work the user kicked off via `/bg <prompt>` or
/// the Background Agents tray.
#[tauri::command]
pub async fn ai_chat_oneshot(
    state: State<'_, EditorState>,
    mut request: AiRequest,
) -> Result<String, String> {
    state.kairos.report_activity().await;

    // Same context injection as `ai_chat` â€” background work still needs
    // the Hades memory header so the model has consistent grounding.
    if let Ok(hades_ctx) = state.memory_layer.get_aggregate_context() {
        if !hades_ctx.trim().is_empty() {
            request.messages.insert(0, ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(hades_ctx)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            });
        }
    }

    let engine = state.ai_engine.clone();
    let _silent = engine.enter_silent();
    if request.ollama_url.as_ref().map(|u| u.trim().is_empty()).unwrap_or(true) {
        let url = state.ollama_url.lock().await.clone();
        if !url.trim().is_empty() {
            request.ollama_url = Some(url);
        }
    }
    let result = engine
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(root) = state.active_root.lock().await.as_ref() {
        let root_str = root.to_string_lossy().to_string();
        let _hooks = crate::stop_hooks::run_stop_hooks(&root_str, &result);
    }

    Ok(result)
}

#[tauri::command]
pub async fn ai_inline_complete(
    state: State<'_, EditorState>,
    prefix: String,
    suffix: String,
    language: String,
    file_path: String,
    model: Option<String>,
    provider: Option<String>,
) -> Result<String, String> {
    // Use active provider/model from state for completions
    let current_model = state.current_model.lock().await.clone();
    let ollama_url_val = state.ollama_url.lock().await.clone();

    // Honor an explicit Autocomplete-feature model when the frontend supplies one
    // (Settings → per-feature model selection). This is usually a small fast coder
    // model — previously these args were dropped and the heavy chat model was used.
    let (comp_provider, comp_model, comp_ollama_url) = if let (Some(p), Some(m)) =
        (provider.as_ref().filter(|s| !s.is_empty()), model.as_ref().filter(|s| !s.is_empty()))
    {
        let p_lc = p.to_lowercase();
        let url = if p_lc == "ollama" || p_lc == "antigravity" { Some(ollama_url_val.clone()) } else { None };
        (p_lc, m.to_string(), url)
    } else {
        // Detect provider from the active model string.
        let m = current_model.as_str();
        if m.contains(':') || (!m.contains('.') && m.contains('/')) || m.to_lowercase().starts_with("llama") || m.to_lowercase().starts_with("qwen") || m.to_lowercase().starts_with("deepseek") || m.to_lowercase().starts_with("gemma") || m.to_lowercase().starts_with("mistral") || m.to_lowercase().starts_with("phi") || m.to_lowercase().starts_with("codellama") {
            ("ollama".to_string(), m.to_string(), Some(ollama_url_val))
        } else if m.to_lowercase().contains("claude-opus-4-8") {
            ("highwayapi".to_string(), m.to_string(), None)
        } else if m.to_lowercase().contains("claude") {
            ("anthropic".to_string(), m.to_string(), None)
        } else if m.to_lowercase().contains("gemini") {
            ("google".to_string(), m.to_string(), None)
        } else if m.to_lowercase().contains("gpt") || m.to_lowercase().contains("o1") || m.to_lowercase().contains("o3") {
            ("openai".to_string(), m.to_string(), None)
        } else {
            // Fallback: try as Ollama
            ("ollama".to_string(), m.to_string(), Some(ollama_url_val))
        }
    };

    // For Ollama models that support FIM tokens (qwen2.5-coder, deepseek-coder, codellama)
    let uses_fim_tokens = comp_provider == "ollama" && (
        comp_model.to_lowercase().contains("coder") ||
        comp_model.to_lowercase().contains("codellama") ||
        comp_model.to_lowercase().contains("deepseek")
    );

    if uses_fim_tokens {
        let base = comp_ollama_url
            .clone()
            .unwrap_or_else(|| "http://127.0.0.1:11434".to_string())
            .trim_end_matches('/')
            .to_string();
        let fim_prompt = format!("<fim_prefix>{}<fim_suffix>{}<fim_middle>", prefix, suffix);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .map_err(|e| e.to_string())?;
        let res = client
            .post(format!("{base}/api/generate"))
            .json(&serde_json::json!({
                "model": comp_model,
                "prompt": fim_prompt,
                "stream": false,
                "raw": true,
                "options": {
                    "temperature": 0.1,
                    "num_predict": 128,
                    "stop": ["<fim_suffix>", "<fim_middle>", "<|fim_suffix|>", "<|fim_middle|>"]
                }
            }))
            .send()
            .await
            .map_err(|e| format!("Ollama FIM generate failed: {e}"))?;
        if res.status().is_success() {
            if let Ok(body) = res.json::<serde_json::Value>().await {
                if let Some(text) = body.get("response").and_then(|v| v.as_str()) {
                    let cleaned = text
                        .trim()
                        .trim_start_matches("```")
                        .trim_start_matches(&language)
                        .trim_start_matches('\n')
                        .trim_end_matches("```")
                        .trim()
                        .to_string();
                    if !cleaned.is_empty() {
                        return Ok(cleaned);
                    }
                }
            }
        }
    }

    let fim_prompt = if uses_fim_tokens {
        format!("<fim_prefix>{}<fim_suffix>{}<fim_middle>", prefix, suffix)
    } else {
        format!(
            "Complete the following {} code. Return ONLY the completion text, no explanation, no markdown fencing, no extra whitespace.\n\n<prefix>\n{}\n</prefix>\n<suffix>\n{}\n</suffix>",
            language, prefix, suffix
        )
    };

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: Some(MessageContent::Text(
                format!("You are an inline code completion engine for file '{}' (language: {}). Return ONLY the exact code that should be inserted at the cursor position. No explanation, no markdown, no comments. Just the raw code completion.", file_path, language)
            )),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        },
        ChatMessage {
            role: "user".to_string(),
            content: Some(MessageContent::Text(fim_prompt)),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        },
    ];

    let request = AiRequest {
        provider: comp_provider,
        model: comp_model,
        messages,
        temperature: Some(0.1),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Completion".to_string()),
        ollama_url: comp_ollama_url,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    // Single-shot, no agentic loop: inline completion must be FAST. The old path
    // ran the full autonomous_loop (tool parsing, memory, brain injection) for a
    // one-line FIM call — huge latency. single_shot_completion hits the provider
    // directly with stream:false.
    let result = state.ai_engine
        .single_shot_completion(request)
        .await
        .map_err(|e| e.to_string())?;

    // Strip any markdown fences
    let cleaned = result
        .trim()
        .trim_start_matches("```")
        .trim_start_matches(&language)
        .trim_start_matches('\n')
        .trim_end_matches("```")
        .trim()
        .to_string();

    Ok(cleaned)
}

/// Cursor-style "next edit" prediction. Given the current buffer + cursor line and the
/// span the user just changed, predict the SINGLE most likely next edit elsewhere in the
/// file (line range + replacement). Returns null-ish (empty `new_text`) when nothing useful.
#[tauri::command]
pub async fn predict_next_edit(
    state: State<'_, EditorState>,
    content: String,
    cursor_line: usize,
    language: String,
    file_path: String,
    recent_change: Option<String>,
    model_override: Option<String>,
) -> Result<serde_json::Value, String> {
    use serde_json::json;

    // Don't bother on tiny / huge buffers — keeps latency sane and signal high.
    let total_lines = content.lines().count();
    if total_lines < 4 || content.len() > 60_000 {
        return Ok(json!({ "has_edit": false }));
    }

    let current_model = state.current_model.lock().await.clone();
    let model_name = model_override
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(current_model);
    let ollama_url_val = state.ollama_url.lock().await.clone();
    let (provider, model, ollama_url) = {
        let m = model_name.as_str();
        let ml = m.to_lowercase();
        if ml.contains("claude-opus-4-8") { ("highwayapi".to_string(), m.to_string(), None) }
        else if ml.contains("claude") { ("anthropic".to_string(), m.to_string(), None) }
        else if ml.contains("gemini") { ("google".to_string(), m.to_string(), None) }
        else if ml.contains("gpt") || ml.contains("o1") || ml.contains("o3") { ("openai".to_string(), m.to_string(), None) }
        else { ("ollama".to_string(), m.to_string(), Some(ollama_url_val)) }
    };

    // Number the lines so the model can reference exact line numbers (1-based).
    let numbered: String = content
        .lines()
        .enumerate()
        .map(|(i, l)| format!("{:>4}| {}", i + 1, l))
        .collect::<Vec<_>>()
        .join("\n");

    let change_note = recent_change
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| format!("\nThe user JUST changed this near line {}:\n{}\n", cursor_line, s))
        .unwrap_or_default();

    let sys = format!(
        "You are a next-edit prediction engine for file '{}' ({}). The user is editing. \
Based on the change they just made, predict the SINGLE most likely NEXT edit they need elsewhere \
in the file (a propagation: update a matching signature, a stale reference, a paired return, an \
import, a sibling case, etc.). Respond with STRICT JSON only, no prose, no markdown:\n\
{{\"has_edit\":bool,\"start_line\":int,\"end_line\":int,\"new_text\":\"...\",\"reason\":\"<=8 words\"}}\n\
Rules: start_line/end_line are 1-based inclusive and reference the NUMBERED source. new_text fully \
replaces those lines (no line-number prefixes, preserve indentation). If no confident propagation \
exists, return {{\"has_edit\":false}}. Never invent edits at the cursor itself.",
        file_path, language
    );

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: Some(MessageContent::Text(sys)),
            tool_calls: None, tool_call_id: None, metadata: None,
        },
        ChatMessage {
            role: "user".to_string(),
            content: Some(MessageContent::Text(format!(
                "Cursor at line {}.{}\nSOURCE:\n{}",
                cursor_line, change_note, numbered
            ))),
            tool_calls: None, tool_call_id: None, metadata: None,
        },
    ];

    let request = AiRequest {
        provider, model, messages,
        temperature: Some(0.1),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Completion".to_string()),
        ollama_url,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: Some("Autocomplete".to_string()),
    };

    let raw = state.ai_engine.clone()
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    // Extract the JSON object even if the model wrapped it in fences/prose.
    let body = {
        let t = raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        match (t.find('{'), t.rfind('}')) {
            (Some(a), Some(b)) if b > a => t[a..=b].to_string(),
            _ => return Ok(json!({ "has_edit": false })),
        }
    };

    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return Ok(json!({ "has_edit": false })),
    };

    let has = parsed.get("has_edit").and_then(|v| v.as_bool()).unwrap_or(false);
    let start = parsed.get("start_line").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let end = parsed.get("end_line").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let new_text = parsed.get("new_text").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let reason = parsed.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();

    // Validate range, reject no-ops and edits that land on the cursor line.
    if !has || start == 0 || end < start || end > total_lines || new_text.is_empty() {
        return Ok(json!({ "has_edit": false }));
    }
    let original: String = content.lines().skip(start - 1).take(end - start + 1).collect::<Vec<_>>().join("\n");
    if original.trim() == new_text.trim() {
        return Ok(json!({ "has_edit": false }));
    }

    Ok(json!({
        "has_edit": true,
        "start_line": start,
        "end_line": end,
        "new_text": new_text,
        "reason": reason,
        "old_text": original
    }))
}

#[tauri::command]
pub async fn ai_explain_code(
    state: State<'_, EditorState>,
    code: String,
    file_path: String,
    detail_level: String,
) -> Result<String, String> {
    let prompt = format!(
        "Explain what this {} code does in {} detail level:\n\n```\n{}\n```\n\nProvide a clear explanation covering:\n1. What the code does (plain English)\n2. Key logic flow\n3. Any important patterns or concepts used",
        file_path.split('.').last().unwrap_or("code"),
        detail_level,
        code
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code explanation assistant. Explain code clearly in plain English.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Explain".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_document_code(
    state: State<'_, EditorState>,
    code: String,
    _file_path: String,
    format: String,
    language: String,
) -> Result<String, String> {
    let prompt = format!(
        "Generate {} documentation for this {} code:\n\n```{}\n```\n\nInclude:\n- Function/class descriptions\n- Parameter explanations\n- Return value descriptions\n- Usage examples if helpful",
        format, language, code
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a documentation generator. Generate clean, professional code documentation.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Document".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_generate_code(
    state: State<'_, EditorState>,
    prompt: String,
    language: String,
    framework: Option<String>,
    _file_path: Option<String>,
) -> Result<String, String> {
    let full_prompt = if let Some(fw) = framework {
        format!("Generate {} code using {} framework for: {}\n\nInclude proper imports, error handling, and best practices.", language, fw, prompt)
    } else {
        format!("Generate {} code for: {}\n\nInclude proper imports, error handling, and best practices.", language, prompt)
    };
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code generation assistant. Generate clean, functional, production-ready code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(full_prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.4),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Generate".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_refactor_code(
    state: State<'_, EditorState>,
    _code: String,
    file_path: String,
    start_line: Option<usize>,
    end_line: Option<usize>,
    refactor_type: String,
    target_name: Option<String>,
) -> Result<String, String> {
    let range = match (start_line, end_line) {
        (Some(s), Some(e)) => format!(" (lines {} to {})", s, e),
        (Some(s), None) => format!(" (starting from line {})", s),
        _ => String::new(),
    };
    
    let prompt = format!(
        "Refactor this {} code{} using {} refactoring approach.{}\n\nProvide improved code with better readability, performance, and best practices.",
        file_path.split('.').last().unwrap_or("code"),
        range,
        refactor_type,
        target_name.map(|n| format!(" Target name: {}", n)).unwrap_or_default()
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code refactoring assistant. Improve code quality while preserving functionality.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Refactor".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_debug_code(
    state: State<'_, EditorState>,
    code: String,
    file_path: String,
    error_message: Option<String>,
    start_line: Option<usize>,
    end_line: Option<usize>,
) -> Result<Value, String> {
    let code_section = match (start_line, end_line) {
        (Some(s), Some(e)) => format!(" (lines {} to {})", s, e),
        _ => String::new(),
    };
    
    let prompt = if let Some(err) = error_message {
        format!(
            "Debug this {} code{} that has error: {}\n\nError: {}\n\nProvide:\n1. Diagnosis of the problem\n2. List of specific issues found\n3. Fixed code\n4. Suggestions for prevention",
            file_path.split('.').last().unwrap_or("code"),
            code_section,
            err,
            code
        )
    } else {
        format!(
            "Debug this {} code{} for bugs, errors, and issues.\n\nProvide:\n1. Diagnosis of problems found\n2. List of specific issues (logic errors, race conditions, security issues, etc.)\n3. Fixed code\n4. Suggestions for improvement\n\nCode:\n```\n{}\n```",
            file_path.split('.').last().unwrap_or("code"),
            code_section,
            code
        )
    };
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code debugging assistant. Find and fix bugs, errors, and issues in code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Debug".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let response = state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())?;
    
    Ok(json!({
        "diagnosis": "Analysis complete",
        "issues": ["See fixed code below"],
        "fixed_code": response,
        "suggestions": ["Review the fixed code and apply any needed adjustments"]
    }))
}

#[tauri::command]
pub async fn ai_multi_cursor_edit(
    state: State<'_, EditorState>,
    _code: String,
    file_path: String,
    pattern: String,
    replacement: String,
    match_scope: String,
    apply: bool,
) -> Result<Value, String> {
    let prompt = format!(
        "Find all occurrences of '{}' in this {} code and {} them.\n\nPattern: {}\nReplacement: {}\nMatch scope: {}\n\nReturn the modified code with all changes applied. If apply=false, show preview of changes.",
        pattern,
        file_path.split('.').last().unwrap_or("code"),
        if apply { "replace" } else { "preview replacement for" },
        pattern,
        replacement,
        match_scope
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a multi-cursor editing assistant. Find patterns and edit them consistently across code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("MultiEdit".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let modified = state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())?;
    
    Ok(json!({
        "matches": [format!("Found occurrences of: {}", pattern)],
        "modified_code": modified,
        "preview_only": !apply
    }))
}

#[tauri::command]
pub async fn ai_pr_review(
    state: State<'_, EditorState>,
    _pr_url: Option<String>,
    diff_content: Option<String>,
    focus_areas: Option<Vec<String>>,
) -> Result<Value, String> {
    let diff = diff_content.ok_or("diff_content required for PR review")?;
    let focus = focus_areas.unwrap_or_else(|| {
        vec!["security".to_string(), "performance".to_string(), "style".to_string()]
    });

    // Cap diff size so 2b–4b local models stay coherent; report the truncation.
    const MAX_DIFF_CHARS: usize = 24_000;
    let truncated = diff.chars().count() > MAX_DIFF_CHARS;
    let diff_slice: String = diff.chars().take(MAX_DIFF_CHARS).collect();

    let prompt = format!(
        "Review the following code diff. Focus areas: {}.\n\
         For each issue report: file, severity (critical/major/minor), what is wrong, and a concrete fix.\n\
         Be specific — quote the offending lines. If the diff is clean, say so.\n\
         End with exactly one verdict line: VERDICT: APPROVE or VERDICT: REQUEST_CHANGES.\n\n\
         ```diff\n{}\n```{}",
        focus.join(", "),
        diff_slice,
        if truncated { "\n\n(NOTE: diff truncated for review)" } else { "" }
    );

    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a rigorous senior code reviewer. Review diffs for correctness, security, performance, and style. Never invent issues; only report what the diff shows.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Review".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let review = state
        .ai_engine
        .clone()
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    let verdict = if review.to_uppercase().contains("VERDICT: APPROVE") {
        "approve"
    } else if review.to_uppercase().contains("REQUEST_CHANGES") {
        "request_changes"
    } else {
        "unspecified"
    };

    Ok(json!({
        "review": review,
        "verdict": verdict,
        "focus": focus,
        "diff_chars": diff.chars().count(),
        "truncated": truncated,
    }))
}

#[tauri::command]
pub async fn ai_get_context(
    state: State<'_, EditorState>,
    query: String,
    max_files: Option<usize>,
    _include_types: Option<Vec<String>>,
) -> Result<Value, String> {
    let max = max_files.unwrap_or(5);

    // Semantic-first: use the vector index when embeddings are available
    // (requires Ollama + an indexed workspace), then fall back to grep so the
    // tool always returns something — and reports which method it used.
    if let Ok(hits) = state.vector_indexer.search_codebase(&query, max).await {
        if !hits.is_empty() {
            let files: Vec<Value> = hits
                .into_iter()
                .map(|h| {
                    json!({
                        "path": h.file_path,
                        "line": h.start_line,
                        "snippet": h.content.chars().take(200).collect::<String>(),
                        "score": h.relevance_score,
                    })
                })
                .collect();
            return Ok(json!({
                "query": query,
                "files": files,
                "count": files.len(),
                "method": "semantic"
            }));
        }
    }

    let results = grep_files(state.clone(), query.clone(), None, None).await.unwrap_or_default();

    let mut unique_files: Vec<Value> = Vec::new();
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

    for r in results.into_iter().take(max * 3) {
        let path = r.path.clone();
        if !seen_paths.contains(&path) && unique_files.len() < max {
            seen_paths.insert(path.clone());
            unique_files.push(json!({
                "path": path,
                "line": r.line,
                "snippet": r.content.chars().take(100).collect::<String>(),
            }));
        }
    }

    Ok(json!({
        "query": query,
        "files": unique_files,
        "count": unique_files.len(),
        "method": "grep_fallback"
    }))
}

#[tauri::command]
pub async fn airi_broadcast(
    app: AppHandle,
    event: String,
    payload: Option<Value>,
) -> Result<(), String> {
    let payload = payload.unwrap_or(json!({}));
    app.emit(&event, &payload)
        .map_err(|e| format!("Failed to broadcast: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn call_tool(
    state: State<'_, EditorState>,
    name: String,
    arguments: Value,
) -> Result<Value, String> {
    state.ai_tools
        .call_tool(&name, arguments.clone())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_execute_command(
    state: State<'_, EditorState>,
    command: String,
    cwd: Option<String>,
    timeout: Option<u64>,
) -> Result<String, String> {
    println!("[DEBUG] ai_execute_command: {}", command);

    // Route through run_command so shell grep/rg is intercepted → bundled ripgrep.
    let mut args = json!({
        "command": command,
        "shell_hint": "bash",
    });
    if timeout.is_some() {
        args["timeout_ms"] = json!(timeout.unwrap_or(120_000));
    }
    if cwd.is_some() {
        args["cwd"] = json!(cwd);
    }

    let result = state
        .ai_tools
        .call_tool("run_command", args)
        .await
        .map_err(|e| e.to_string())?;

    if result.get("status").and_then(|v| v.as_str()) == Some("blocked") {
        return Err(
            result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Command blocked")
                .to_string(),
        );
    }

    if let Some(results) = result.get("results").and_then(|v| v.as_str()) {
        return Ok(if results.is_empty() {
            "No matches".to_string()
        } else {
            results.to_string()
        });
    }

    let stdout = result.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    let stderr = result.get("stderr").and_then(|v| v.as_str()).unwrap_or("");
    let success = result.get("success").and_then(|v| v.as_bool()).unwrap_or(false);

    if success {
        if stdout.is_empty() && !stderr.is_empty() {
            Ok(format!("Command succeeded (stderr only):\n{}", stderr))
        } else if stdout.is_empty() {
            Ok("Command succeeded (no output)".to_string())
        } else {
            Ok(stdout.to_string())
        }
    } else {
        Err(format!(
            "Command failed:\nSTDOUT: {}\nSTDERR: {}",
            stdout, stderr
        ))
    }
}

#[tauri::command]
pub fn ai_modify_file(
    _state: State<'_, EditorState>,
    path: String,
    instruction: String,
) -> Result<(), String> {
    println!(
        "AI requested modification for path: {}, instruction: {}",
        path, instruction
    );
    Ok(())
}

#[tauri::command]
pub async fn propose_file_change(
    app: AppHandle,
    _state: State<'_, EditorState>,
    path: String,
    content: String,
    description: String,
) -> Result<Value, String> {
    let path_buf = PathBuf::from(&path);

    let old_content = if path_buf.exists() {
        fs::read_to_string(&path_buf).unwrap_or_default()
    } else {
        String::new()
    };

    let payload = json!({
        "path": path,
        "old_content": old_content,
        "new_content": content,
        "description": description
    });
    let _ = app.emit("propose-edit", &payload);

    Ok(json!({
        "path": path,
        "oldContent": old_content,
        "newContent": content,
        "description": description
    }))
}

/// Composer-style fast apply: merge a SEARCH/REPLACE block and open the diff review panel.
#[tauri::command]
pub async fn preview_search_replace(
    app: AppHandle,
    state: State<'_, EditorState>,
    path: String,
    search_text: String,
    replace_text: String,
    description: Option<String>,
) -> Result<(), String> {
    use crate::patch_engine::PatchBlock;

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(format!("File not found: {path}"));
    }
    let old_content = fs::read_to_string(&path_buf).map_err(|e| e.to_string())?;
    let patch = PatchBlock {
        search: search_text,
        replace: replace_text,
    };
    let mut pe = state.patch_engine.lock().await;
    let new_content = pe
        .apply_patches(&path_buf, &old_content, &[patch])
        .await
        .map_err(|e| e.to_string())?;

    let desc = description.unwrap_or_else(|| "Search/replace preview".to_string());
    app.emit(
        "propose-edit",
        json!({
            "path": path,
            "old_content": old_content,
            "new_content": new_content,
            "description": desc,
        }),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
pub async fn compress_session_data(
    state: State<'_, EditorState>,
    key: String,
    data: String,
) -> Result<(), String> {
    state
        .memory_optimizer
        .compress_and_store(&key, &data)

        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_ollama_status(state: State<'_, EditorState>) -> Result<bool, String> {
    state
        .ai_engine
        .check_ollama_status()
        .await
        .map_err(|e| e.to_string())
}


#[tauri::command]
pub async fn pull_ollama_model(state: State<'_, EditorState>, name: String) -> Result<(), String> {
    state
        .ai_engine
        .pull_model(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_ollama_url(state: State<'_, EditorState>, url: String) -> Result<(), String> {
    let normalized = normalize_ollama_base_url(&url);
    {
        let mut current = state.ollama_url.lock().await;
        *current = normalized.clone();
    }

    state.ai_engine.set_ollama_url(normalized).await;
    Ok(())
}

/// Force the context indexer to rescan the active workspace. Powers the
/// "Re-index" button under Settings â†’ Indexing & Docs and the `/reindex`
/// slash command. We call `reindex_if_needed` (not a hard rebuild) so
/// large repos don't get flattened by an accidental click; the indexer
/// itself decides if anything has changed on disk.
#[tauri::command]
pub async fn reindex_workspace(
    state: State<'_, EditorState>,
) -> Result<Value, String> {
    let root = {
        let guard = state.active_root.lock().await;
        guard.clone().unwrap_or_else(|| std::path::PathBuf::from("."))
    };
    state
        .context_indexer
        .reindex_if_needed(&root)
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "status": "ok",
        "root": root.to_string_lossy(),
    }))
}

/// List every workspace rule the rules engine currently sees. Used by the
/// Cursor-style "Rules, Skills, Subagents" settings panel so the user can
/// inspect which files are being injected into the system prompt without
/// having to grep the workspace. Returns each rule's name, full text, and
/// source path so the UI can deep-link to the file.
#[tauri::command]
pub async fn list_workspace_rules(
    state: State<'_, EditorState>,
) -> Result<Value, String> {
    let rules = state.ai_engine.rules_engine.get_workspace_rules();
    let items: Vec<Value> = rules
        .into_iter()
        .map(|r| serde_json::json!({
            "name": r.name,
            "content": r.content,
            "file_path": r.file_path.to_string_lossy(),
        }))
        .collect();
    Ok(serde_json::json!({
        "count": items.len(),
        "rules": items,
    }))
}

#[tauri::command]
pub async fn unload_ollama_model(state: State<'_, EditorState>, name: String) -> Result<(), String> {
    state
        .ai_engine
        .attachment_manager.unload_model(&name)

        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ollama_ps(state: State<'_, EditorState>) -> Result<Value, String> {
    state
        .ai_engine
        .check_ollama_status()
        .await
        .map(|status| json!({ "initialized": status }))
        .map_err(|e| e.to_string())
}

/// Verbose Ollama probe: returns URL, status code, bearer-configured flag,
/// model list, body preview and a human-readable hint. Use this when the
/// model dropdown comes back empty so the user knows *why*.
#[tauri::command]
pub async fn diagnose_ollama(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(state.ai_engine.diagnose_ollama().await)
}

#[tauri::command]
pub async fn set_ai_model(state: State<'_, EditorState>, model: String) -> Result<(), String> {
    let mut current = state.current_model.lock().await;
    *current = model.clone();
    state.ai_engine.set_advisor_model(Some(model)).await;

    Ok(())
}

#[tauri::command]
pub async fn set_advisor_model(state: State<'_, EditorState>, model: Option<String>) -> Result<(), String> {
    let mut advisor = state.advisor_model.lock().await;
    *advisor = model;
    Ok(())
}

#[tauri::command]
pub async fn list_provider_models(
    state: State<'_, EditorState>,
    provider: String,
) -> Result<Vec<String>, String> {
    state
        .ai_engine
        .list_models(&provider)
        .await
        .map_err(|e| e.to_string())
}

/// Native Ollama GET (`/api/tags`, etc.) from Rust â€” bypasses browser CORS on locked-down proxies.
#[tauri::command]
pub async fn ollama_native_get(state: State<'_, EditorState>, path: String) -> Result<Value, String> {
    state
        .ai_engine
        .ollama_native_get(path)
        .await
        .map_err(|e| e.to_string())
}

/// Native Ollama POST (`/api/generate`, `/api/chat`, â€¦) from Rust â€” same CORS bypass as GET.
#[tauri::command]
pub async fn ollama_native_post(
    state: State<'_, EditorState>,
    path: String,
    body: Value,
) -> Result<Value, String> {
    state
        .ai_engine
        .ollama_native_post(path, body)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_agent_messages(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.ai_engine.memory_store.get_messages().await))
}

#[tauri::command]
pub async fn get_brain_telemetry(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(state.ai_engine.memory_store.get_brain_telemetry().await)
}

#[tauri::command]
pub async fn store_message(
    state: State<'_, EditorState>,
    role: String,
    content: String,
    timestamp: i64,
) -> Result<(), String> {
    state.ai_engine.memory_store.store_message_params(role, content, timestamp).await;
    Ok(())
}

#[tauri::command]
pub async fn sync_agent_messages(
    state: State<'_, EditorState>,
    messages: Vec<ChatMessage>,
) -> Result<(), String> {
    state.ai_engine.memory_store.store_conversation(&messages).await;
    state.ai_engine.memory_store.flush_to_disk().await;
    Ok(())
}
#[tauri::command]
pub async fn list_chat_sessions(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.ai_engine.memory_store.list_sessions().await))
}

#[tauri::command]
pub async fn load_chat_session(state: State<'_, EditorState>, path: String) -> Result<Value, String> {
    let messages = state
        .ai_engine
        .memory_store
        .restore_session_from_path(PathBuf::from(path))
        .await;
    Ok(json!(messages))
}

#[tauri::command]
pub async fn archive_chat_session(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.memory_store.archive_current_session().await;
    Ok(())
}

#[tauri::command]
pub async fn create_new_session(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.memory_store.create_new_session().await;
    Ok(())
}

/// Drain the live agent-activity buffer. The webview can't receive the Tauri
/// event stream, so the activity terminal polls this to mirror what the agent
/// is doing in real time. Each entry is a JSON line: `{"kind","payload"}`.
#[tauri::command]
pub async fn agent_activity_drain(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let mut log = state
        .ai_engine
        .activity_log
        .lock()
        .map_err(|e| format!("activity_log lock poisoned: {e}"))?;
    Ok(std::mem::take(&mut *log))
}

/// Drain buffered chat tokens (live model output). The `ai-content-delta` event
/// is dead in the webview, so the chat panel polls this to render streamed text.
#[tauri::command]
pub async fn chat_stream_drain(state: State<'_, EditorState>) -> Result<String, String> {
    let mut b = state
        .ai_engine
        .chat_stream_buf
        .lock()
        .map_err(|e| format!("chat_stream_buf lock poisoned: {e}"))?;
    Ok(std::mem::take(&mut *b))
}

/// Drain queued edit proposals from the autonomous loop. Each is
/// `{path, oldContent, newContent, description, additions, deletions}`. The diff-
/// review panel polls this (the event stream is dead in the webview) and turns
/// each into a reviewable pending change.
#[tauri::command]
pub async fn agent_proposals_drain(state: State<'_, EditorState>) -> Result<Vec<Value>, String> {
    let mut q = state
        .ai_engine
        .pending_proposals
        .lock()
        .map_err(|e| format!("pending_proposals lock poisoned: {e}"))?;
    Ok(std::mem::take(&mut *q))
}

/// Restore a file to the given content. Used by the diff-review panel to REJECT
/// an already-applied agent edit (revert to the pre-edit snapshot). Path may be
/// absolute or project-relative.
#[tauri::command]
pub async fn revert_file_content(
    state: State<'_, EditorState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let root = state.ai_engine.ai_tools.get_root_path();
    let full = if std::path::Path::new(&path).is_absolute() {
        PathBuf::from(&path)
    } else {
        root.join(&path)
    };
    fs::write(&full, content).map_err(|e| format!("revert failed for {path}: {e}"))?;
    Ok(())
}

/// Inspect a `.aim` Neural Weight-Map binary for the in-IDE viewer. Returns the
/// header (magic/version/written_at/entry_count/size) plus a capped list of
/// entries (key, weight, mtime, gist preview). Path may be absolute or
/// workspace-relative.
#[tauri::command]
pub async fn aim_inspect(state: State<'_, EditorState>, path: String) -> Result<serde_json::Value, String> {
    let root = state.ai_engine.ai_tools.get_root_path();
    let full = if std::path::Path::new(&path).is_absolute() {
        PathBuf::from(&path)
    } else {
        root.join(&path)
    };
    if !full.exists() {
        return Err(format!("AIM file not found: {}", full.display()));
    }
    let bytes = std::fs::read(&full).map_err(|e| e.to_string())?;
    let size = bytes.len() as u64;

    // `.aim` has two on-disk flavors in this project:
    //   • JSON   — the Kortex brain/memory (`{"kortex":{...}}`)
    //   • binary — the aim_store Neural Weight-Map (magic `AIM\x01`)
    // Detect by the first non-whitespace byte and handle both.
    let first = bytes.iter().copied().find(|b| !b.is_ascii_whitespace()).unwrap_or(0);
    if first == b'{' || first == b'[' {
        let v: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|e| format!("AIM file looks like JSON but failed to parse: {e}"))?;
        let mut pretty = serde_json::to_string_pretty(&v).unwrap_or_default();
        let truncated = pretty.len() > 300_000;
        if truncated {
            pretty.truncate(300_000);
            pretty.push_str("\n… [truncated for display]");
        }
        let entities = v.pointer("/kortex/entities").and_then(|e| e.as_object()).map(|o| o.len());
        let tree_count = v.pointer("/kortex/project_tree").and_then(|t| t.as_array()).map(|a| a.len());
        return Ok(serde_json::json!({
            "format": "json",
            "size_bytes": size,
            "pretty": pretty,
            "truncated": truncated,
            "entities": entities,
            "tree_count": tree_count,
        }));
    }

    // Binary AIM\x01 (aim_store).
    let insp = crate::aim_store::AimStore::inspect(&full, 2000).map_err(|e| e.to_string())?;
    let mut val = serde_json::to_value(insp).map_err(|e| e.to_string())?;
    if let Some(obj) = val.as_object_mut() {
        obj.insert("format".to_string(), serde_json::json!("aim-binary"));
    }
    Ok(val)
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct OllamaLibraryEntry {
    name: String,
    ram_gb: u32,
    tags: Vec<String>,
    desc: String,
}

static OLLAMA_LIBRARY: &str = include_str!("../resources/ollama_library.json");
static OLLAMA_LOCAL_REGISTRY: &str = include_str!("../resources/ollama_local_registry.json");

fn load_ollama_catalog() -> Result<Vec<OllamaLibraryEntry>, String> {
    let mut all: Vec<OllamaLibraryEntry> =
        serde_json::from_str(OLLAMA_LIBRARY).map_err(|e| format!("catalog parse: {e}"))?;
    let local: Vec<OllamaLibraryEntry> =
        serde_json::from_str(OLLAMA_LOCAL_REGISTRY).unwrap_or_default();
    let mut seen: std::collections::HashSet<String> = all.iter().map(|e| e.name.clone()).collect();
    for e in local {
        if seen.insert(e.name.clone()) {
            all.insert(0, e);
        }
    }
    Ok(all)
}

/// Search the bundled Ollama model catalog (offline-safe). Used by the first-run wizard.
#[tauri::command]
pub async fn search_ollama_library(
    query: String,
    limit: Option<usize>,
) -> Result<Vec<OllamaLibraryEntry>, String> {
    let cap = limit.unwrap_or(24).min(60);
    let q = query.trim().to_lowercase();
    let all = load_ollama_catalog()?;
    if q.is_empty() {
        return Ok(all.into_iter().take(cap).collect());
    }
    let mut hits: Vec<OllamaLibraryEntry> = all
        .into_iter()
        .filter(|e| {
            e.name.to_lowercase().contains(&q)
                || e.desc.to_lowercase().contains(&q)
                || e.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
        .take(cap)
        .collect();
    if hits.is_empty() {
        hits = load_ollama_catalog()
            .unwrap_or_default()
            .into_iter()
            .filter(|e| e.name.to_lowercase().starts_with(&q))
            .take(cap)
            .collect();
    }
    Ok(hits)
}

