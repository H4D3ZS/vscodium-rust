use crate::EditorState;
use crate::ai_engine::{AiRequest, ChatMessage, MessageContent, AiResponse, normalize_ollama_base_url};
use tauri::{State, AppHandle, Emitter};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub path: String,
    pub line: usize,
    pub content: String,
}

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
        state.active_root.lock().await.clone().unwrap_or_else(|| PathBuf::from("."))
    };

    let mut results = Vec::new();

    // 1. Try ripgrep (Optimized for speed)
    let rg_result = std::process::Command::new("rg")
        .args(&["-n", "--no-heading", "--max-count=100", "--color=never"])
        .args(include.as_ref().map(|i| vec!["-g", i]).unwrap_or_default())
        .arg(&pattern)
        .current_dir(&root)
        .output();

    if let Ok(output) = rg_result {
        if output.status.success() || !output.stdout.is_empty() {
             let stdout = String::from_utf8_lossy(&output.stdout);
             for line in stdout.lines().take(100) {
                 let parts: Vec<&str> = line.splitn(3, ':').collect();
                 if parts.len() == 3 {
                     if let Ok(ln) = parts[1].parse::<usize>() {
                         results.push(SearchResult {
                             path: parts[0].to_string(),
                             line: ln,
                             content: parts[2].trim().to_string(),
                         });
                     }
                 }
             }
             if !results.is_empty() { return Ok(results); }
        }
    }

    // 2. Fallback: Pure-Rust Resident Search (WalkDir + Regex)
    println!("[DEBUG] Ripgrep unavailable. Activating internal search engine for: {}", pattern);
    let re = regex::RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;

    let walker = ignore::WalkBuilder::new(&root)
        .standard_filters(true)
        .max_depth(Some(10))
        .build();

    for entry in walker.flatten() {
        let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
        if is_file {
            let path = entry.path();
            if let Ok(content) = fs::read_to_string(path) {
                if content.len() > 1_000_000 { continue; } // Skip huge binaries
                for (i, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        results.push(SearchResult {
                            path: path.to_string_lossy().to_string(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                        if results.len() >= 100 { break; }
                    }
                }
            }
        }
        if results.len() >= 100 { break; }
    }

    Ok(results)
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
pub async fn ai_chat(state: State<'_, EditorState>, mut request: AiRequest) -> Result<String, String> {
    let log_entry = format!("[ai_chat] REQUEST: {:?}\n", request);
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(log_entry.as_bytes()) });

    // Signal Kairos: user is actively using AI — reset idle timer
    state.kairos.report_activity().await;

    // Update MemoryLayer state — agent is now active
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

    let result = state
        .ai_engine
        .clone()
        .autonomous_loop(request, None)
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

    let done_log = format!("[ai_chat] DONE: response_len={}\n", result.len());
    eprintln!("{}", done_log.trim());
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(done_log.as_bytes()) });
        
    // Update MemoryLayer: agent completed the task
    let _ = state.memory_layer.update_state("Idle", "Task completed");

    // Satisfy AiResponse usage warning
    let _response = AiResponse { content: result.clone() };

    Ok(result)
}

/// Trivial-chat fast path. Skips the autonomous loop, the phase
/// machinery, tool catalog construction, system-prompt assembly, and
/// every retry. One HTTP round-trip to the provider and we're done.
/// Used by the frontend for short, action-less prompts like "hello"
/// where running the full agent loop is gross overkill (and was making
/// "hi" take 5+ seconds while the model dutifully ran git_status and
/// grep). Cursor's Agent mode behaves the same way — it doesn't open
/// the codebase index for a greeting.
#[tauri::command]
pub async fn ai_chat_fast(
    state: State<'_, EditorState>,
    request: AiRequest,
) -> Result<String, String> {
    state.kairos.report_activity().await;

    let engine = state.ai_engine.clone();
    // We deliberately do NOT inject the heavy Hades context here; the
    // whole point is sub-second latency. The conversation history the
    // frontend already passed is enough for trivial chat.
    let result = engine
        .single_shot_completion(request)
        .await
        .map_err(|e| e.to_string())?;

    // Keep the existing UI plumbing happy: every other code path
    // delivers the response via the `ai-content` event, so emit it
    // here too. The frontend `ai-content` listener calls
    // `updateLastAgentMessage` and flips `isAgentThinking` off.
    engine.emit_event("ai-content", serde_json::json!({ "content": result.clone() }));

    Ok(result)
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

    // Same context injection as `ai_chat` — background work still needs
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
    let result = engine
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn ai_inline_complete(
    state: State<'_, EditorState>,
    prefix: String,
    suffix: String,
    language: String,
    file_path: String,
) -> Result<String, String> {
    // Use active provider/model from state for completions
    let current_model = state.current_model.lock().await.clone();
    let ollama_url_val = state.ollama_url.lock().await.clone();

    // Detect provider from model string
    let (comp_provider, comp_model, comp_ollama_url) = {
        let m = current_model.as_str();
        if m.contains(':') || (!m.contains('.') && m.contains('/')) || m.to_lowercase().starts_with("llama") || m.to_lowercase().starts_with("qwen") || m.to_lowercase().starts_with("deepseek") || m.to_lowercase().starts_with("gemma") || m.to_lowercase().starts_with("mistral") || m.to_lowercase().starts_with("phi") || m.to_lowercase().starts_with("codellama") {
            ("ollama".to_string(), m.to_string(), Some(ollama_url_val))
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
    };

    let result = state.ai_engine.clone()
        .autonomous_loop(request, None)
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
    _state: State<'_, EditorState>,
    _pr_url: Option<String>,
    diff_content: Option<String>,
    focus_areas: Option<Vec<String>>,
) -> Result<Value, String> {
    let _diff = diff_content.ok_or("diff_content required for PR review")?;
    let _focus = focus_areas.unwrap_or_else(|| vec!["security".to_string(), "performance".to_string(), "style".to_string()]);
    
    Ok(json!({
        "summary": "Review requires AI engine integration",
        "issues": ["Awaiting full AI integration"],
        "suggestions": ["Full PR review coming soon"],
        "security": "Manual review recommended",
        "performance": "Manual review recommended"
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
        "count": unique_files.len()
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
pub async fn ai_execute_command(command: String, cwd: Option<String>, _timeout: Option<u64>) -> Result<String, String> {
    println!("[DEBUG] ai_execute_command: {}", command);
    
    let working_dir = cwd.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.args(&["/C", &command]);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.args(&["-c", &command]);
        c
    };

    cmd.current_dir(working_dir);
    
    let output = cmd.output().map_err(|e| format!("Failed to spawn command: {}", e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    if output.status.success() {
        if stdout.is_empty() && !stderr.is_empty() {
             Ok(format!("Command succeeded (stderr only):\n{}", stderr))
        } else if stdout.is_empty() {
             Ok("Command succeeded (no output)".to_string())
        } else {
             Ok(stdout)
        }
    } else {
        Err(format!("Command failed (exit {}):\nSTDOUT: {}\nSTDERR: {}", 
            output.status.code().unwrap_or(-1),
            stdout,
            stderr))
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
    _state: State<'_, EditorState>,
    path: String,
    content: String,
    description: String,
) -> Result<Value, String> {
    // Check if path is valid (stub for now, needs logic)
    let path_buf = PathBuf::from(&path);
    
    let old_content = if path_buf.exists() {
        fs::read_to_string(&path_buf).unwrap_or_default()
    } else {
        String::new()
    };

    Ok(json!({
        "path": path,
        "oldContent": old_content,
        "newContent": content,
        "description": description
    }))
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

/// Native Ollama GET (`/api/tags`, etc.) from Rust — bypasses browser CORS on locked-down proxies.
#[tauri::command]
pub async fn ollama_native_get(state: State<'_, EditorState>, path: String) -> Result<Value, String> {
    state
        .ai_engine
        .ollama_native_get(path)
        .await
        .map_err(|e| e.to_string())
}

/// Native Ollama POST (`/api/generate`, `/api/chat`, …) from Rust — same CORS bypass as GET.
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
    use std::sync::atomic::Ordering;
    let mut store_messages = state.ai_engine.memory_store.messages.write().await;
    *store_messages = messages;
    state.ai_engine.memory_store.is_dirty.store(true, Ordering::Relaxed);
    Ok(())
}
#[tauri::command]
pub async fn list_chat_sessions(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.ai_engine.memory_store.list_sessions().await))
}

#[tauri::command]
pub async fn load_chat_session(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    state.ai_engine.memory_store.load_from_path(PathBuf::from(path)).await;
    Ok(())
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
