//! Chat-panel bridge to the Claude Code CLI, running against local Lemonade.
//!
//! The IDE has two agent loops that share one inference server: its own
//! (`Sentient::autonomous_loop`) and Claude Code's. This module lets the chat
//! panel drive the latter, so a prompt typed in the IDE gets Claude Code's
//! harness — its tool-call discipline, hooks, skills and subagents — instead of
//! the in-house loop, while inference still runs on the local model.
//!
//! Transport: `claude -p --output-format stream-json --include-partial-messages`.
//! Streamed text is pushed into `Sentient::chat_stream_buf`, the same buffer the
//! panel already polls via `chat_stream_drain` (the `ai-content-delta` event does
//! not reach the webview), so the existing rendering path works unchanged.

use tauri::{AppHandle, Emitter, State};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Stdio;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Outcome of one Claude Code turn.
#[derive(serde::Serialize, Clone, Debug, Default)]
pub struct ClaudeCodeReply {
    /// Assistant text for the turn, concatenated across content blocks.
    pub content: String,
    /// Claude Code session id — pass back as `resume` to continue the thread.
    pub session_id: String,
    /// Tool names invoked this turn, in call order.
    pub tools_used: Vec<String>,
    /// Number of assistant/tool round-trips Claude Code took.
    pub num_turns: u32,
    /// `false` when the CLI reported an error result.
    pub ok: bool,
    /// Error text when `ok` is false.
    pub error: String,
}

/// Tokens Claude Code sends before any conversation content: its system prompt
/// plus tool definitions. Measured ~21k on this setup. It is charged against the
/// same window as the replayed history, which is why a 97k transcript does not
/// fit a 98k window.
const AGENT_OVERHEAD_TOKENS: u32 = 21_000;

/// Chars per token for mixed code+prose on this tokenizer, measured against real
/// counts. Raw byte length over 4.0 was ~40% optimistic and would let an
/// overflowing session through.
const CHARS_PER_TOKEN: f32 = 3.6;

/// Claude Code's on-disk key for a working directory: path separators and the
/// drive colon each become `-` (so `C:\Users\x\p` → `C--Users-x-p`).
fn claude_project_key(dir: &std::path::Path) -> String {
    let s = dir.to_string_lossy();
    let mapped: String = s
        .chars()
        .map(|c| if c == '\\' || c == '/' || c == ':' { '-' } else { c })
        .collect();
    mapped.trim_start_matches('-').to_string()
}

/// Approximate tokens of conversation history in a stored transcript.
///
/// `None` when the transcript cannot be found or read — callers must treat that
/// as "unknown" and proceed, never as "empty".
fn transcript_history_tokens(dir: &std::path::Path, session_id: &str) -> Option<u32> {
    let home = dirs::home_dir()?;
    let path = home
        .join(".claude")
        .join("projects")
        .join(claude_project_key(dir))
        .join(format!("{session_id}.jsonl"));
    let text = std::fs::read_to_string(path).ok()?;

    let mut chars: usize = 0;
    for line in text.lines().filter(|l| !l.is_empty()) {
        let Ok(obj) = serde_json::from_str::<Value>(line) else { continue };
        let Some(msg) = obj.get("message") else { continue };
        if msg.get("role").is_none() {
            continue;
        }
        match msg.get("content") {
            Some(Value::String(s)) => chars += s.len(),
            Some(Value::Array(blocks)) => {
                for b in blocks {
                    match b.get("type").and_then(|t| t.as_str()) {
                        Some("text") => {
                            chars += b.get("text").and_then(|t| t.as_str()).map_or(0, str::len)
                        }
                        Some("tool_use") => {
                            chars += b.get("input").map_or(0, |i| i.to_string().len())
                        }
                        Some("tool_result") => {
                            chars += match b.get("content") {
                                Some(Value::String(s)) => s.len(),
                                Some(v) => v.to_string().len(),
                                None => 0,
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Some((chars as f32 / CHARS_PER_TOKEN) as u32)
}

/// Refuse a `--resume` that cannot fit the served context window.
///
/// `--resume` replays the ENTIRE conversation in one request, and Claude Code
/// does not know the local model's window, so it never trims. Past `n_ctx`,
/// llama.cpp returns 400 and Lemonade's Anthropic adapter turns that into an
/// HTTP 200 with empty content — no error reaches the caller, the turn simply
/// never produces text. Inside the IDE that is an unexplained dead chat panel.
///
/// Failing here with a number the user can act on is strictly better than
/// spawning a process that is guaranteed to hang.
fn check_resume_fits(
    dir: &std::path::Path,
    session_id: &str,
    ctx_size: u32,
) -> Result<(), String> {
    let Some(history) = transcript_history_tokens(dir, session_id) else {
        return Ok(()); // Unknown size — let it try rather than block on a guess.
    };
    let needed = history + AGENT_OVERHEAD_TOKENS;
    if needed <= ctx_size {
        return Ok(());
    }
    Err(format!(
        "Cannot resume session {}: it replays ~{} tokens of history, and with Claude \
         Code's ~{}-token overhead that needs ~{} against a {}-token window (over by \
         ~{}). Resuming would return an empty response with no error and hang. \
         Start a new session, or /compact this one first — usable history is ~{} tokens.",
        &session_id[..session_id.len().min(8)],
        history,
        AGENT_OVERHEAD_TOKENS,
        needed,
        ctx_size,
        needed - ctx_size,
        ctx_size.saturating_sub(AGENT_OVERHEAD_TOKENS),
    ))
}

/// Push a chunk into the shared chat-stream buffer the panel polls.
fn push_stream(engine: &Arc<crate::ai_engine::Sentient>, chunk: &str) {
    if chunk.is_empty() {
        return;
    }
    if let Ok(mut b) = engine.chat_stream_buf.lock() {
        b.push_str(chunk);
    }
}

/// Run one Claude Code turn against the local Lemonade model.
///
/// `resume` continues an existing Claude Code session (pass the `session_id`
/// from a previous reply); omit it to start a fresh thread. `model` defaults to
/// the IDE's selected model. `allow_net` and `skip_permissions` mirror
/// `spawn_claude_terminal` — airgapped and permission-skipping by default.
#[tauri::command]
pub async fn claude_code_chat(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    app: AppHandle,
    prompt: String,
    resume: Option<String>,
    model: Option<String>,
    skip_permissions: Option<bool>,
    allow_net: Option<bool>,
    images: Option<Vec<String>>,
) -> Result<ClaudeCodeReply, String> {
    let mut prompt = prompt.trim().to_string();
    if prompt.is_empty() {
        return Err("Empty prompt".to_string());
    }

    let workspace_root = {
        let root = state.editor.active_root.lock().await;
        root.clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    };

    // The CLI's `-p` takes text only, so attached images are written to disk and
    // referenced by path — Claude Code's Read tool handles image files. Without
    // this they were silently dropped from the turn.
    let saved = materialize_images(images.as_deref().unwrap_or(&[]));
    if !saved.is_empty() {
        prompt.push_str("

Attached image(s) — use the Read tool to view:
");
        for p in &saved {
            prompt.push_str(&format!("- {}
", p));
        }
    }

    let model = match model.map(|m| m.trim().to_string()).filter(|m| !m.is_empty()) {
        Some(m) => m,
        None => state.ai.current_model.lock().await.clone(),
    };
    // A `…:latest` id persisted from Lemonade's Ollama-compat tag list is not
    // what the server or the Anthropic endpoint answers to.
    let model = super::ai::canonical_model_id(model.trim()).to_string();
    if model.trim().is_empty() {
        return Err("No model selected. Pick a Lemonade model in Settings → Inference Backend.".into());
    }

    let lemonade_base = state.ai.engine.lemonade_base().await;
    let lemonade_base = lemonade_base.trim_end_matches('/').to_string();

    // Make sure the backend is actually up before spawning anything.
    //
    // The served-model guard below cannot distinguish "Lemonade does not serve
    // this" from "Lemonade is not running" — an unreachable server returns an
    // empty list, which it treats as "unknown" and waves through. So on a fresh
    // IDE launch the first prompt used to spawn a Claude Code process with no
    // backend to talk to, fail, and then work on the second attempt once the
    // server had come up on its own.
    //
    // This starts the server if needed and preloads the model, so the first
    // prompt behaves like every other one.
    let ready = super::ai::lemonade_ensure_ready(&lemonade_base, &model).await;

    // Same two guards as the terminal path: a model Lemonade doesn't serve would
    // 404 every request, and one that can't call tools makes the agent loop spin.
    let served = super::ai::lemonade_served_models(&lemonade_base).await;
    if served.is_empty() {
        return Err(format!(
            "Cannot reach Lemonade at {lemonade_base}. {ready}\n\
             Start it (or set LEMONADE_URL) and try again."
        ));
    }
    // `served` is non-empty here, so a miss really means "not served".
    if !served.iter().any(|m| m == &model) {
        let mut available = served.clone();
        available.sort();
        return Err(format!(
            "Lemonade does not serve '{}'. Available: {}",
            model,
            available.join(", ")
        ));
    }
    if !super::ai::supports_tool_calling(&model) {
        return Err(format!(
            "Model '{}' cannot reliably call tools (measured 0/6) — pick a tool-calling model.",
            model
        ));
    }

    super::ai::apply_lemonade_tuning(&lemonade_base, &model).await;

    let (exe, base_args) = super::terminal::resolve_claude_launch(&workspace_root);
    let mut cmd = std::process::Command::new(&exe);
    // Without CREATE_NO_WINDOW a console flashes up on every chat message: the
    // launcher is claude.cmd, and cmd.exe always allocates a window. It looks
    // like something is shelling out behind the user's back, which is alarming
    // in a product even though nothing is wrong.
    {
        use crate::process_ext::CommandExtHidden as _;
        cmd.hidden();
    }
    cmd.args(&base_args);

    cmd.arg("-p")
        .arg("--output-format").arg("stream-json")
        .arg("--include-partial-messages")
        // stream-json output requires --verbose in print mode.
        .arg("--verbose");

    if skip_permissions.unwrap_or(true) {
        cmd.arg("--dangerously-skip-permissions");
    }

    // Carry the offensive-security operator context into Claude Code's harness.
    //
    // Claude Code supplies its own system prompt, so without this the IDE's
    // `cyber_instruction` — which only reaches the in-house `autonomous_loop` —
    // was silently dropped for every chat message. Since the Claude Code harness
    // is the DEFAULT, that meant the security framing was absent exactly where
    // most work happens, and VAPT prompts fell back to whatever the base prompt
    // allowed. Appended (not replaced) so Claude Code keeps its tool discipline.
    cmd.arg("--append-system-prompt")
        .arg(crate::ai_prompts::OFFENSIVE_SECURITY_MODE);
    if let Some(sid) = resume.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        // Preflight before spawning: an overflowing resume hangs silently, and
        // once the process is running there is no signal to distinguish that
        // from a slow first turn.
        let (ctx_size, _, _) = super::ai::lemonade_tuning(&model);
        check_resume_fits(&workspace_root, sid, ctx_size)?;
        cmd.arg("--resume").arg(sid);
    }
    cmd.arg(&prompt);

    // Identical env contract to `spawn_claude_terminal` — see the comments there.
    cmd.env("ANTHROPIC_BASE_URL", &lemonade_base);
    cmd.env("ANTHROPIC_AUTH_TOKEN", "lemonade");
    cmd.env("ANTHROPIC_API_KEY", "lemonade");
    // Recent Claude Code builds validate ANTHROPIC_MODEL against a known list and
    // abort with `[claude-code:unrecognized_model]` on anything that isn't a real
    // Claude id — a raw GGUF name like `Escha-…-ROCmFP2.gguf` never passes. The
    // local server (llama-server, directly or via the Kortex proxy) ignores the
    // model field and serves whatever it loaded, so we hand Claude Code a
    // recognised alias and let the backend do the right thing.
    const CC_MODEL_ALIAS: &str = "claude-sonnet-4-20250514";
    for var in [
        "ANTHROPIC_MODEL",
        "ANTHROPIC_SMALL_FAST_MODEL",
        "ANTHROPIC_DEFAULT_HAIKU_MODEL",
        "ANTHROPIC_DEFAULT_SONNET_MODEL",
        "ANTHROPIC_DEFAULT_OPUS_MODEL",
    ] {
        cmd.env(var, CC_MODEL_ALIAS);
    }
    let (ctx_size, _, _) = super::ai::lemonade_tuning(&model);
    // Small on purpose. llama.cpp requires `prompt + max_tokens <= n_ctx`, so a
    // large output budget is headroom stolen from the prompt on every request —
    // and worst of all on the compaction request, which has the biggest prompt.
    cmd.env(
        "CLAUDE_CODE_MAX_OUTPUT_TOKENS",
        super::ai::claude_max_output_tokens().to_string(),
    );
    // Tell Claude Code the REAL context window.
    //
    // It does not recognise local model ids, so it assumes 200k and sizes
    // auto-compact to that. The served window here is `ctx_size` (98304), so a
    // session would sail past the real limit and start getting empty HTTP 200s
    // — llama.cpp 400s that Lemonade rewrites as success — long before Claude
    // Code decided to compact. That presents as the agent going silent
    // mid-session with no error, which is the worst failure mode on this stack.
    //
    // Setting this makes auto-compact fire while there is still room.
    //
    // Declaring the TRUE window is not enough and was measured to fail: Claude Code
    // compacts at ~80% of what it is told, so at 98,304 it would wait until ~78,600
    // — and with ~36,700 tokens of system prompt and tool schemas on top, the
    // compaction request itself overflows and dies with "summarization produced
    // empty response". `claude_max_context_tokens` derates it so compaction starts
    // while it can still succeed.
    cmd.env(
        "CLAUDE_CODE_MAX_CONTEXT_TOKENS",
        super::ai::claude_max_context_tokens(ctx_size).to_string(),
    );
    // Without this the client gives up while the model is still generating, then
    // retries into the queue behind its own unfinished request.
    let (_, _, expected_tps) = super::ai::lemonade_tuning(&model);
    cmd.env(
        "API_TIMEOUT_MS",
        super::ai::claude_api_timeout_ms(expected_tps, super::ai::claude_max_output_tokens())
            .to_string(),
    );
    cmd.env("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1");
    if !allow_net.unwrap_or(false) {
        cmd.env("HTTPS_PROXY", "http://127.0.0.1:1");
        cmd.env("https_proxy", "http://127.0.0.1:1");
        cmd.env("HTTP_PROXY", "http://127.0.0.1:1");
        cmd.env("http_proxy", "http://127.0.0.1:1");
        cmd.env("NO_PROXY", "localhost,127.0.0.1,::1");
        cmd.env("no_proxy", "localhost,127.0.0.1,::1");
    }

    if workspace_root.is_dir() {
        cmd.current_dir(&workspace_root);
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Own process group on Unix so `process_registry::kill_all` can signal the
    // whole tree with a negative PID. Windows uses `taskkill /T` and needs no
    // equivalent — which matters here, because the launcher is `claude.cmd`
    // shelling out to `claude.exe`, so killing only the direct child would
    // orphan the process that is actually writing files.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    let mut child = cmd.spawn().map_err(|e| {
        format!("Claude Code not found ({}). Install: npm i -g @anthropic-ai/claude-code", e)
    })?;

    // Register before any await so the Stop button can kill a runaway turn.
    // Without this the subprocess keeps running — and keeps editing files —
    // after the user has stopped the agent.
    let child_pid = child.id();
    crate::process_registry::register(child_pid);

    let stdout = child.stdout.take().ok_or("failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("failed to capture stderr")?;

    let engine = state.ai.engine.clone();
    let reply = Arc::new(Mutex::new(ClaudeCodeReply { ok: true, ..Default::default() }));

    // stderr on a side thread so a chatty CLI can't fill the pipe and deadlock.
    let errbuf = Arc::new(Mutex::new(String::new()));
    let errbuf_t = errbuf.clone();
    let err_thread = std::thread::spawn(move || {
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            if let Ok(mut b) = errbuf_t.lock() {
                b.push_str(&line);
                b.push('\n');
            }
        }
    });

    let reply_t = reply.clone();
    let app_t = app.clone();
    // `call_id -> (path, content-before-the-edit)`. Claude Code emits the
    // `assistant` message carrying `tool_use` BEFORE it runs the tool, so the
    // file on disk is still the pre-edit version at that moment — that is what
    // makes a real before/after diff possible.
    let snapshots: Arc<Mutex<HashMap<String, (String, String)>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let root_t = workspace_root.clone();
    let out_thread = std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            let line = line.trim();
            if !line.starts_with('{') {
                continue;
            }
            let Ok(ev) = serde_json::from_str::<Value>(line) else { continue };
            handle_event(&ev, &engine, &app_t, &reply_t, &snapshots, &root_t);
        }
    });

    let status = tokio::task::spawn_blocking(move || child.wait())
        .await
        .map_err(|e| {
            crate::process_registry::unregister(child_pid);
            format!("join error: {e}")
        })?
        .map_err(|e| {
            crate::process_registry::unregister(child_pid);
            format!("claude wait failed: {e}")
        })?;
    crate::process_registry::unregister(child_pid);
    let _ = out_thread.join();
    let _ = err_thread.join();

    let mut out = reply.lock().map(|r| r.clone()).unwrap_or_default();

    // Claude Code can exit 0 and still report a failed turn via result.subtype
    // (e.g. "error_during_execution"), which on its own names no cause. The
    // actual reason — model refused, context overflow, tool crash — is on
    // stderr, so attach it rather than surfacing a bare subtype nobody can act on.
    if !out.ok {
        let stderr_text = errbuf.lock().map(|b| b.clone()).unwrap_or_default();
        let lines: Vec<&str> = stderr_text.lines().collect();
        let tail = lines[lines.len().saturating_sub(6)..].join("
");
        if !tail.trim().is_empty() {
            out.error = format!("{}
{}", out.error.trim(), tail.trim());
        }
        if out.error.trim().is_empty() || out.error.trim() == "error_during_execution" {
            out.error = format!(
                "Claude Code ended the turn without a result ({}). Most often the local model                  returned an empty reply — check that Lemonade is running at {} with the model                  loaded.",
                out.error.trim(),
                lemonade_base
            );
        }
    }

    if !status.success() && out.ok {
        out.ok = false;
        let stderr_text = errbuf.lock().map(|b| b.clone()).unwrap_or_default();
        out.error = format!(
            "claude exited with {}: {}",
            status.code().map(|c| c.to_string()).unwrap_or_else(|| "signal".into()),
            stderr_text.trim().chars().take(400).collect::<String>()
        );
    }
    Ok(out)
}

/// Translate one stream-json event onto the IDE's existing chat surface.
fn handle_event(
    ev: &Value,
    engine: &Arc<crate::ai_engine::Sentient>,
    app: &AppHandle,
    reply: &Arc<Mutex<ClaudeCodeReply>>,
    snapshots: &Arc<Mutex<HashMap<String, (String, String)>>>,
    root: &PathBuf,
) {
    match ev.get("type").and_then(|t| t.as_str()).unwrap_or("") {
        // Token-level deltas — the only thing the panel needs for live text.
        "stream_event" => {
            let inner = ev.get("event").unwrap_or(&Value::Null);
            if inner.get("type").and_then(|t| t.as_str()) == Some("content_block_delta") {
                if let Some(text) = inner
                    .get("delta")
                    .and_then(|d| d.get("text"))
                    .and_then(|t| t.as_str())
                {
                    push_stream(engine, text);
                    let _ = app.emit("ai-content-delta", json!({ "delta": text }));
                }
            }
        }
        // Completed assistant message: accumulate final text, surface tool calls.
        "assistant" => {
            let blocks = ev
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_array())
                .cloned()
                .unwrap_or_default();
            for b in blocks {
                match b.get("type").and_then(|t| t.as_str()).unwrap_or("") {
                    "text" => {
                        if let Some(t) = b.get("text").and_then(|t| t.as_str()) {
                            if let Ok(mut r) = reply.lock() {
                                r.content.push_str(t);
                            }
                        }
                    }
                    "tool_use" => {
                        let name = b.get("name").and_then(|n| n.as_str()).unwrap_or("tool");
                        let call_id = b.get("id").and_then(|i| i.as_str()).unwrap_or_default();
                        if let Ok(mut r) = reply.lock() {
                            r.tools_used.push(name.to_string());
                        }
                        // Same activity contract the native loop uses, so Claude
                        // Code's calls render as the normal Cursor-style tool
                        // cards. Tauri events do not reach this webview — the
                        // panel polls `agent_activity_drain` instead.
                        crate::domain::tools::registry::push_activity(
                            &engine.activity_log,
                            "ai-tool-call",
                            json!({
                                "name": name,
                                "args": b.get("input").cloned().unwrap_or(Value::Null),
                                "call_id": call_id,
                            }),
                        );
                        // This event precedes execution, so the file on disk is
                        // still the pre-edit version. Snapshot it now — it is the
                        // only moment the "before" side of the diff exists.
                        if is_edit_tool(name) {
                            if let Some(fp) = b
                                .get("input")
                                .and_then(|i| i.get("file_path"))
                                .and_then(|f| f.as_str())
                            {
                                let full = resolve_path(root, fp);
                                let before = std::fs::read_to_string(&full).unwrap_or_default();
                                if let Ok(mut m) = snapshots.lock() {
                                    m.insert(call_id.to_string(), (fp.to_string(), before));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        // Tool results come back as a synthetic user message; close the matching
        // card so it stops spinning and shows its output.
        "user" => {
            let blocks = ev
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_array())
                .cloned()
                .unwrap_or_default();
            for b in blocks {
                if b.get("type").and_then(|t| t.as_str()) != Some("tool_result") {
                    continue;
                }
                let call_id = b.get("tool_use_id").and_then(|i| i.as_str()).unwrap_or_default();
                // `content` is either a plain string or a list of text blocks.
                let result = match b.get("content") {
                    Some(Value::String(t)) => t.clone(),
                    Some(Value::Array(parts)) => parts
                        .iter()
                        .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
                        .collect::<Vec<_>>()
                        .join("
"),
                    other => other.map(|o| o.to_string()).unwrap_or_default(),
                };
                let is_err = b.get("is_error").and_then(|e| e.as_bool()).unwrap_or(false);
                crate::domain::tools::registry::push_activity(
                    &engine.activity_log,
                    "ai-tool-result",
                    json!({
                        "name": "",
                        "call_id": call_id,
                        "result": if is_err { format!("Error: {result}") } else { result.clone() },
                    }),
                );

                // The edit has landed — pair it with the snapshot and queue a
                // reviewable proposal, same contract the native loop uses.
                if !is_err {
                    let snap = snapshots.lock().ok().and_then(|mut m| m.remove(call_id));
                    if let Some((path, before)) = snap {
                        queue_proposal(engine, root, &path, &before);
                    }
                }
            }
        }
        "result" => {
            if let Ok(mut r) = reply.lock() {
                if let Some(sid) = ev.get("session_id").and_then(|s| s.as_str()) {
                    r.session_id = sid.to_string();
                }
                if let Some(n) = ev.get("num_turns").and_then(|n| n.as_u64()) {
                    r.num_turns = n as u32;
                }
                let subtype = ev.get("subtype").and_then(|s| s.as_str()).unwrap_or("");
                if subtype != "success" {
                    r.ok = false;
                    r.error = ev
                        .get("result")
                        .and_then(|x| x.as_str())
                        .unwrap_or(subtype)
                        .to_string();
                }
                // `result.result` is the authoritative final text; prefer it when
                // the assistant blocks produced nothing (e.g. tool-only turns).
                if r.content.trim().is_empty() {
                    if let Some(t) = ev.get("result").and_then(|x| x.as_str()) {
                        r.content = t.to_string();
                    }
                }
            }
        }
        _ => {}
    }
}

/// Write attached images to disk and return their paths.
///
/// Accepts `data:` URLs (what the chat panel produces for pasted/attached
/// images). Plain `http(s)` URLs are returned unchanged so the model can still
/// see the reference, though the airgap blocks fetching them by default.
/// Anything undecodable is skipped rather than failing the turn.
fn materialize_images(urls: &[String]) -> Vec<String> {
    use base64::Engine;

    if urls.is_empty() {
        return Vec::new();
    }
    let dir = std::env::temp_dir().join("vscodium-rust-attachments");
    if std::fs::create_dir_all(&dir).is_err() {
        return Vec::new();
    }

    let mut out = Vec::new();
    for url in urls {
        let url = url.trim();
        if url.starts_with("http://") || url.starts_with("https://") {
            out.push(url.to_string());
            continue;
        }
        let Some(rest) = url.strip_prefix("data:") else { continue };
        let Some((meta, payload)) = rest.split_once(',') else { continue };
        if !meta.contains("base64") {
            continue;
        }
        let ext = match meta.split(';').next().unwrap_or("") {
            "image/png" => "png",
            "image/jpeg" | "image/jpg" => "jpg",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => "png",
        };
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(payload) else { continue };
        let path = dir.join(format!("{}.{}", uuid::Uuid::new_v4().simple(), ext));
        if std::fs::write(&path, &bytes).is_ok() {
            out.push(path.to_string_lossy().to_string());
        }
    }
    out
}

/// Claude Code tools that mutate a file on disk. `MultiEdit` and `NotebookEdit`
/// also carry `file_path`, so the same snapshot/diff path covers them.
fn is_edit_tool(name: &str) -> bool {
    matches!(name, "Edit" | "Write" | "MultiEdit" | "NotebookEdit")
}

/// Claude Code reports absolute paths, but the panel keys proposals on the
/// workspace-relative path the rest of the IDE uses.
fn resolve_path(root: &PathBuf, p: &str) -> PathBuf {
    let path = PathBuf::from(p);
    if path.is_absolute() { path } else { root.join(path) }
}

/// Queue a reviewable before/after diff for the edit-review panel, matching
/// `agent_proposals_drain`'s contract exactly.
///
/// Repeated edits to one file collapse into a single entry that keeps the
/// EARLIEST snapshot as `oldContent`, so the diff spans the whole turn and
/// rejecting it reverts the file completely.
fn queue_proposal(
    engine: &Arc<crate::ai_engine::Sentient>,
    root: &PathBuf,
    path: &str,
    before: &str,
) {
    let full = resolve_path(root, path);
    let after = std::fs::read_to_string(&full).unwrap_or_default();
    if after == before {
        return;
    }
    // Key on the workspace-relative path so a later absolute-path edit to the
    // same file collapses onto the same entry.
    let rel = full
        .strip_prefix(root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string());

    let Ok(mut q) = engine.pending_proposals.lock() else { return };
    let base_old = q
        .iter()
        .find(|p| p.get("path").and_then(|v| v.as_str()) == Some(rel.as_str()))
        .and_then(|p| p.get("oldContent").and_then(|v| v.as_str()))
        .map(|s| s.to_string())
        .unwrap_or_else(|| before.to_string());

    let patch = diffy::create_patch(&base_old, &after);
    let mut additions = 0u32;
    let mut deletions = 0u32;
    for l in patch.to_string().lines() {
        if l.starts_with('+') && !l.starts_with("+++") { additions += 1; }
        else if l.starts_with('-') && !l.starts_with("---") { deletions += 1; }
    }

    q.retain(|p| p.get("path").and_then(|v| v.as_str()) != Some(rel.as_str()));
    q.push(json!({
        "path": rel,
        "oldContent": base_old,
        "newContent": after,
        "description": "Claude Code edit",
        "additions": additions,
        "deletions": deletions,
    }));
    if q.len() > 200 {
        let drop = q.len() - 200;
        q.drain(0..drop);
    }
}

/// Whether the Claude Code CLI is resolvable — the chat panel greys out the
/// "route through Claude Code" toggle when this is false.
#[tauri::command]
pub fn claude_code_available() -> bool {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let (exe, _) = super::terminal::resolve_claude_launch(&root);
    exe != "claude" || which::which("claude").is_ok()
}


#[cfg(test)]
mod edit_review_tests {
    use super::*;

    /// A 1x1 PNG as a data URL — the shape the chat panel produces for a pasted
    /// image. It must land on disk as a real file, because the CLI's `-p` takes
    /// text only and can reach an image solely by path.
    #[test]
    fn data_urls_are_written_to_disk() {
        const PNG_1X1: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";
        let out = materialize_images(&[PNG_1X1.to_string()]);
        assert_eq!(out.len(), 1, "one image in, one path out");
        let p = PathBuf::from(&out[0]);
        assert!(p.exists(), "path must be a real file the Read tool can open");
        assert_eq!(p.extension().and_then(|e| e.to_str()), Some("png"));
        assert!(std::fs::metadata(&p).unwrap().len() > 0);
        let _ = std::fs::remove_file(&p);
    }

    /// Remote URLs pass through untouched; junk is skipped rather than
    /// failing the whole turn.
    #[test]
    fn http_passes_through_and_junk_is_skipped() {
        let out = materialize_images(&[
            "https://example.com/a.png".to_string(),
            "data:image/png,not-base64".to_string(),
            "garbage".to_string(),
        ]);
        assert_eq!(out, vec!["https://example.com/a.png".to_string()]);
    }

    #[test]
    fn only_file_mutating_tools_are_snapshotted() {
        for t in ["Edit", "Write", "MultiEdit", "NotebookEdit"] {
            assert!(is_edit_tool(t), "{t} mutates a file and must be reviewable");
        }
        // Snapshotting these would queue empty diffs on every read/search.
        for t in ["Bash", "Read", "Grep", "Glob", "WebFetch", "TodoWrite"] {
            assert!(!is_edit_tool(t), "{t} does not write a file");
        }
    }

    /// Claude Code reports absolute paths; the review panel keys on the
    /// workspace-relative path, so both spellings must collapse to one entry.
    #[test]
    fn absolute_and_relative_paths_resolve_alike() {
        let root = PathBuf::from("/work");
        assert_eq!(resolve_path(&root, "src/a.rs"), PathBuf::from("/work/src/a.rs"));
        assert_eq!(resolve_path(&root, "/work/src/a.rs"), PathBuf::from("/work/src/a.rs"));
    }

    #[test]
    fn diff_counts_added_and_removed_lines() {
        let before = "one
two
three
";
        let after = "one
two CHANGED
three
four
";
        let patch = diffy::create_patch(before, after).to_string();
        let adds = patch.lines().filter(|l| l.starts_with('+') && !l.starts_with("+++")).count();
        let dels = patch.lines().filter(|l| l.starts_with('-') && !l.starts_with("---")).count();
        assert_eq!(adds, 2, "changed line + new line");
        assert_eq!(dels, 1, "the replaced line");
    }

    /// Claude Code keys transcripts by working directory with separators and the
    /// drive colon flattened to `-`. Getting this wrong means the preflight finds
    /// no transcript and silently waves every resume through.
    #[test]
    fn project_key_matches_claude_codes_on_disk_layout() {
        use std::path::Path;
        assert_eq!(
            claude_project_key(Path::new(r"C:\Users\HADES\Desktop\bh")),
            "C--Users-HADES-Desktop-bh"
        );
        assert_eq!(
            claude_project_key(Path::new("/home/u/proj")),
            "home-u-proj",
            "leading separator must not produce a leading dash"
        );
    }

    /// A missing transcript is "unknown", not "empty" — the guard must let the
    /// spawn proceed rather than block a resume it cannot measure.
    #[test]
    fn unmeasurable_resume_is_allowed_through() {
        let dir = std::env::temp_dir().join("vscodium-rust-no-such-project-xyz");
        assert!(check_resume_fits(&dir, "deadbeef-0000", 98_304).is_ok());
    }

    /// The arithmetic that decides hang vs proceed. History alone fitting the
    /// window is not enough — the ~21k agent overhead is charged against the same
    /// budget, which is exactly the case that was hanging in practice (a 97k
    /// transcript against a 98k window).
    #[test]
    fn overhead_is_charged_against_the_same_window() {
        const CTX: u32 = 98_304;
        let usable = CTX - AGENT_OVERHEAD_TOKENS;
        assert_eq!(usable, 77_304);

        // A 97k history "fits" 98304 on its own, but must NOT be allowed.
        assert!(
            97_001 + AGENT_OVERHEAD_TOKENS > CTX,
            "a 97k transcript plus overhead must overflow a 98k window"
        );
        // And it must clear comfortably once compacted under the budget.
        assert!(50_000 + AGENT_OVERHEAD_TOKENS < CTX);
    }
}
