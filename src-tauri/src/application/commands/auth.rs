use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

// Holds active OAuth sessions keyed by session_id
lazy_static::lazy_static! {
    pub static ref OAUTH_SESSIONS: Arc<Mutex<HashMap<String, OAuthSession>>> = Arc::new(Mutex::new(HashMap::new()));
    /// Tracks the currently "active" session_id per provider (key = provider string).
    pub static ref ACTIVE_SESSIONS: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    /// WebUI agentic bridge: window_label -> sender for captured response text.
    /// The browser observer (save_webui_response) delivers each captured (growing)
    /// response here; the agent loop awaits the *stable* final text. This is the
    /// keystone that makes a subscription WebUI session drive the IDE's tool loop.
    pub static ref WEBUI_WAITERS: Arc<Mutex<HashMap<String, tokio::sync::mpsc::UnboundedSender<String>>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Register a waiter for a window's next responses. Returns the receiver the loop
/// reads from; the sender is stored in WEBUI_WAITERS for save_webui_response to push to.
fn webui_register_waiter(label: &str) -> tokio::sync::mpsc::UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    WEBUI_WAITERS.lock().unwrap().insert(label.to_string(), tx);
    rx
}

fn webui_unregister_waiter(label: &str) {
    WEBUI_WAITERS.lock().unwrap().remove(label);
}

/// Deliver a captured response to a window's waiter (if the loop is listening).
fn webui_deliver_response(label: &str, text: String) {
    if let Some(tx) = WEBUI_WAITERS.lock().unwrap().get(label) {
        let _ = tx.send(text);
    }
}

/// Await the *stable* final response: keep the latest text, return once no new
/// capture arrives for `quiet` (streaming finished) or the overall `total` deadline
/// is hit. The observer streams growing snapshots, so "no change for ~3s" ≈ "done".
async fn webui_await_stable(
    rx: &mut tokio::sync::mpsc::UnboundedReceiver<String>,
    quiet: std::time::Duration,
    total: std::time::Duration,
) -> Option<String> {
    let deadline = tokio::time::Instant::now() + total;
    let mut latest: Option<String> = None;
    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            return latest;
        }
        match tokio::time::timeout(quiet.min(remaining), rx.recv()).await {
            Ok(Some(text)) => latest = Some(text), // new snapshot — keep waiting for quiet
            Ok(None) => return latest,             // channel closed
            Err(_) => {
                // Quiet window elapsed. If we have something, it's stable → done.
                if latest.is_some() {
                    return latest;
                }
                // Nothing captured yet; keep waiting until the total deadline.
            }
        }
    }
}

/// Tool protocol injected into the WebUI prompt so a subscription chat model can
/// drive the IDE. The model emits fenced JSON tool calls; the loop executes them
/// locally and feeds results back. This is how a sandboxed claude.ai page "acts"
/// in the IDE — the model is the brain, the loop is the hands.
const WEBUI_TOOL_PROTOCOL: &str = r#"You are an autonomous coding agent operating a REAL IDE on the user's machine through a tool bridge. You cannot touch files directly — instead you EMIT TOOL CALLS and I execute them and return the results, then you continue.

## How to call a tool
Emit one or more fenced JSON blocks, each exactly like:
```json
{"tool": "view_file", "args": {"path": "src/main.rs"}}
```
I run them, then reply with the results. You then emit the next call(s). Act — don't over-explain.

## Tools (name → args)
- view_file {path}                              — read a file
- list_files {path}                             — list a directory
- grep {pattern, path?}                         — search code
- write_to_file {path, content}                 — create/overwrite a file
- search_replace_edit {path, search, replace}   — surgical edit
- run_command {command}                         — run a shell command
- dev_cargo_diagnostics {}                       — cargo check (Rust)
- deep_security_audit {path}                    — structured CWE security audit

## Rules
- One reply = the next tool call(s). Keep going until the whole task is done.
- After any code edit, verify (dev_cargo_diagnostics for Rust, or run_command for typecheck/tests).
- When the task is fully complete AND verified, write the single token TASK_COMPLETE on its own line."#;

/// Open (or reuse) the provider's WebUI window and inject the response observer +
/// the prompt. Shared by send_webui_prompt and the agentic loop.
fn webui_open_and_inject(
    app: &tauri::AppHandle,
    provider_key: &str,
    label: &str,
    url: &str,
    prompt: &str,
) -> Result<(), String> {
    let window = if let Some(existing) = app.get_webview_window(label) {
        existing
    } else {
        tauri::WebviewWindowBuilder::new(
            app,
            label.to_string(),
            tauri::WebviewUrl::External(url.parse().map_err(|e| format!("Bad WebUI URL: {}", e))?),
        )
        .title(format!("{} Web Session", provider_key))
        .inner_size(1100.0, 800.0)
        .resizable(true)
        .visible(false)
        .build()
        .map_err(|e| format!("Failed to open WebUI window: {}", e))?
    };
    let observer_script = webui_response_observer_script(provider_key, label)?;
    let script = webui_prompt_script(provider_key, prompt)?;
    let window_for_eval = window.clone();
    tauri::async_runtime::spawn(async move {
        // Give a freshly-opened page time to render the input box; re-injection into
        // an existing window is harmless (the observer guards against double-install).
        tokio::time::sleep(tokio::time::Duration::from_millis(900)).await;
        let _ = window_for_eval.eval(&observer_script);
        let _ = window_for_eval.eval(&script);
    });
    Ok(())
}

/// Parse fenced JSON tool calls (`{"tool":..,"args":..}`) out of the model's prose.
fn parse_webui_tool_calls(text: &str) -> Vec<(String, serde_json::Value)> {
    let mut calls = Vec::new();
    let mut search = text;
    while let Some(start) = search.find("```") {
        let after = &search[start + 3..];
        let nl = after.find('\n').unwrap_or(after.len());
        let rest = if nl < after.len() { &after[nl + 1..] } else { "" };
        if let Some(end) = rest.find("```") {
            let block = &rest[..end];
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(block.trim()) {
                if let (Some(name), Some(args)) =
                    (v.get("tool").and_then(|x| x.as_str()), v.get("args"))
                {
                    calls.push((name.to_string(), args.clone()));
                }
            }
            search = &rest[end + 3..];
        } else {
            break;
        }
    }
    calls
}

/// Agentic loop over a subscription WebUI session: inject prompt → await the stable
/// response → parse tool calls → execute locally → feed results back → repeat until
/// TASK_COMPLETE, no tool calls, or max_steps. Emits ai-content/ai-tool-call/
/// ai-tool-result so the existing UI renders it like any agent turn.
#[tauri::command]
pub async fn webui_agent_run(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::state::EditorState>,
    provider: String,
    prompt: String,
    max_steps: Option<u32>,
) -> Result<serde_json::Value, String> {
    let max_steps = max_steps.unwrap_or(12).min(40);
    let (provider_key, account_key) = webui_provider_and_account(&provider);
    let url = webui_chat_url(&provider_key)
        .ok_or_else(|| format!("Unsupported WebUI provider: {}", provider))?;
    let label = format!(
        "webui_{}_{}",
        provider_key.replace(" (webui)", "").replace("-webui", "").replace(' ', "_"),
        account_key
    );
    let ai_tools = state.ai.tools.clone();

    let mut next_prompt = format!("{}\n\n# Task\n{}", WEBUI_TOOL_PROTOCOL, prompt);
    let mut final_text = String::new();
    let mut steps_done = 0u32;

    for step in 0..max_steps {
        steps_done = step + 1;
        let mut rx = webui_register_waiter(&label);
        if let Err(e) = webui_open_and_inject(&app, &provider_key, &label, url, &next_prompt) {
            webui_unregister_waiter(&label);
            return Err(e);
        }
        let _ = app.emit(
            "ai-action",
            serde_json::json!({ "action": format!("WebUI step {} — awaiting {} reply", steps_done, provider_key) }),
        );

        let response = webui_await_stable(
            &mut rx,
            std::time::Duration::from_millis(3000),
            std::time::Duration::from_secs(180),
        )
        .await;
        webui_unregister_waiter(&label);

        let response = match response {
            Some(r) => r,
            None => return Err(format!("WebUI: no response captured within timeout (step {})", steps_done)),
        };
        final_text = response.clone();
        let _ = app.emit("ai-content", serde_json::json!({ "content": response }));

        if response.to_uppercase().contains("TASK_COMPLETE") {
            break;
        }
        let calls = parse_webui_tool_calls(&response);
        if calls.is_empty() {
            // Prose answer with no actionable tool call → treat as done.
            break;
        }
        let mut results = String::new();
        for (name, args) in calls {
            let _ = app.emit("ai-tool-call", serde_json::json!({ "name": name, "args": args }));
            let res = ai_tools
                .call_tool(&name, args)
                .await
                .map(|v| v.to_string())
                .unwrap_or_else(|e| format!("Error: {}", e));
            let _ = app.emit("ai-tool-result", serde_json::json!({ "name": name, "result": res }));
            let clipped: String = res.chars().take(2000).collect();
            results.push_str(&format!("\n## Result of {}\n{}\n", name, clipped));
        }
        next_prompt = format!(
            "{}\n\n# Tool results\n{}\n\nContinue: emit the next tool call(s) in the same JSON format, or write TASK_COMPLETE when the task is fully done and verified.",
            WEBUI_TOOL_PROTOCOL, results
        );
    }

    Ok(serde_json::json!({
        "success": true,
        "steps": steps_done,
        "final": final_text,
        "provider": provider_key,
    }))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthSession {
    pub provider: String,
    pub account: String,
    pub redirect_url: String,
    pub state: String,
    pub token: Option<String>,
    pub token_expires: Option<i64>,
    /// Human-readable account label (e.g. "account1", user email, or "default")
    pub display_name: String,
}

/// Public summary of a stored session (safe to send to the frontend)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionSummary {
    pub session_id: String,
    pub provider: String,
    pub display_name: String,
    pub has_token: bool,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub provider: String,
    pub redirect_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub session_id: Option<String>,
    pub url: String,
    pub callback_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebUiResponsePayload {
    pub provider: String,
    pub window: String,
    pub text: String,
    pub url: Option<String>,
}

fn webui_canonical_provider(provider: &str) -> String {
    let provider = provider.split(':').next().unwrap_or(provider);
    let p_lower = provider.to_ascii_lowercase();
    if p_lower.contains("openwebui") || p_lower.contains("open-webui") {
        return "openwebui".to_string();
    }
    let normalized = provider
        .to_ascii_lowercase()
        .replace(" (webui)", "")
        .replace("-webui", "")
        .replace("webui", "")
        .trim()
        .to_string();

    match normalized.as_str() {
        "gpt" | "chatgpt" | "open ai" => "openai".to_string(),
        "qwen code" | "qwen-code" => "qwen".to_string(),
        "" => provider.to_ascii_lowercase(),
        other => other.to_string(),
    }
}

fn webui_provider_and_account(provider: &str) -> (String, String) {
    let mut parts = provider.splitn(2, ':');
    let canonical = webui_canonical_provider(parts.next().unwrap_or(provider));
    let account = parts
        .next()
        .map(|value| {
            value
                .chars()
                .map(|ch| {
                    if ch.is_ascii_alphanumeric() {
                        ch.to_ascii_lowercase()
                    } else {
                        '_'
                    }
                })
                .collect::<String>()
                .trim_matches('_')
                .to_string()
        })
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "default".to_string());
    (canonical, account)
}

fn webui_session_key(provider: &str, account: &str) -> String {
    format!("{}:{}", provider, account)
}

// ============================================================================
// OAuth Callback Server – runs in background and captures the token
// ============================================================================
pub async fn start_oauth_listener(port: u16, app: tauri::AppHandle) -> Result<(), String> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| format!("Failed to bind OAuth listener: {}", e))?;

    println!(
        "[OAuth] Callback listener running on http://127.0.0.1:{}",
        port
    );

    loop {
        match listener.accept().await {
            Ok((mut stream, addr)) => {
                let app_clone = app.clone();
                tokio::spawn(async move {
                    handle_oauth_callback(&mut stream, addr, app_clone).await;
                });
            }
            Err(e) => {
                eprintln!("[OAuth] Accept error: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}

async fn handle_oauth_callback(
    stream: &mut TcpStream,
    _addr: std::net::SocketAddr,
    app: tauri::AppHandle,
) {
    let mut buffer = [0u8; 4096];
    match stream.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            let (method, path) = parse_request(&request);

            if method == "GET" && path.contains("/oauth/callback") {
                // Extract session_id and provider from the path
                if let Some(session_id) = extract_session_id(&path) {
                    let mut success = false;
                    let mut provider_name = String::new();

                    // Scope for MutexGuard
                    {
                        let mut session_map = OAUTH_SESSIONS.lock().unwrap();
                        if let Some(session) = session_map.get_mut(&session_id) {
                            if let Some(token) = extract_token_from_url(&path) {
                                session.token = Some(token);
                                session.token_expires = Some(
                                    chrono::Utc::now()
                                        .checked_add_signed(chrono::Duration::hours(1))
                                        .map(|dt| dt.timestamp())
                                        .unwrap_or(0),
                                );
                                provider_name =
                                    webui_session_key(&session.provider, &session.account);
                                success = true;
                            }
                        }
                    }
                    // Auto-promote: make this session the active one.
                    // (Only sets it if not already overridden by the user.)
                    if success && !provider_name.is_empty() {
                        let mut active_map = ACTIVE_SESSIONS.lock().unwrap();
                        active_map
                            .entry(provider_name.clone())
                            .or_insert_with(|| session_id.clone());
                    }

                    if success {
                        use tauri::Manager;
                        if !provider_name.is_empty() {
                            let window_label = format!("login_{}", provider_name.replace(':', "_"));
                            if let Some(window) = app.get_webview_window(&window_label) {
                                let _ = window.close();
                            }
                        }

                        // Send success response
                        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Authentication successful!</h1><p>You can close this window now.</p><script>window.close();</script></body></html>";
                        let _ = stream.write_all(response.as_bytes()).await;
                        println!("[OAuth] Token captured for session: {}", session_id);
                    } else {
                        let response = "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<html><body>Token not found or invalid session</body></html>";
                        let _ = stream.write_all(response.as_bytes()).await;
                    }
                }
            } else {
                let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<html><body>404</body></html>";
                let _ = stream.write_all(response.as_bytes()).await;
            }
        }
        _ => {}
    }
}

fn parse_request(request: &str) -> (String, String) {
    let lines: Vec<&str> = request.lines().collect();
    if lines.is_empty() {
        return ("GET".to_string(), "/".to_string());
    }
    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    if parts.len() >= 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        ("GET".to_string(), "/".to_string())
    }
}

fn extract_session_id(path: &str) -> Option<String> {
    if let Some(pos) = path.find("session_id=") {
        let rest = &path[pos + 11..];
        if let Some(end) = rest.find('&') {
            return Some(rest[..end].to_string());
        }
        return Some(rest.to_string());
    }
    None
}

fn extract_token_from_url(path: &str) -> Option<String> {
    if let Some(pos) = path.find('?') {
        let query = &path[pos + 1..];
        for part in query.split('&') {
            if let Some((key, value)) = part.split_once('=') {
                if key == "access_token" {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
pub async fn start_webui_login(
    app: tauri::AppHandle,
    request: LoginRequest,
) -> Result<LoginResponse, String> {
    let (provider, account) = webui_provider_and_account(&request.provider);
    let session_key = webui_session_key(&provider, &account);

    // Build provider-specific login URL
    let (login_url, base_url) = match provider.as_str() {
        "claude" => ("https://claude.ai/login", "https://claude.ai"),
        "gemini" => ("https://gemini.google.com/app", "https://gemini.google.com"),
        "openai" | "gpt" => ("https://chatgpt.com/auth/login", "https://chatgpt.com"),
        "openwebui" | "openwebui-claude" | "openwebui-gpt" | "openwebui-gemini" => {
            ("http://localhost:8080/auth", "http://localhost:8080")
        }
        "deepseek" => (
            "https://chat.deepseek.com/sign_in",
            "https://chat.deepseek.com",
        ),
        "qwen" => ("https://chat.qwen.ai/", "https://chat.qwen.ai"),
        other => return Err(format!("Unsupported provider: {}", other)),
    };

    // Generate session
    let session_id = Uuid::new_v4().to_string();
    let redirect_url = format!(
        "{}/oauth/callback?session_id={}&provider={}",
        base_url, session_id, provider
    );

    // Store session
    {
        let display_name = format!("Account {}", &session_id[..6]);
        let mut sessions = OAUTH_SESSIONS.lock().unwrap();
        sessions.insert(
            session_id.clone(),
            OAuthSession {
                provider: provider.clone(),
                account: account.clone(),
                redirect_url: redirect_url.clone(),
                state: session_id.clone(),
                token: None,
                token_expires: None,
                display_name: if account == "default" { display_name } else { account.clone() },
            },
        );
    }

    // Open popup window instead of default browser
    let window_label = format!("login_{}", session_key.replace(':', "_"));
    if let Some(existing) = app.get_webview_window(&window_label) {
        let _ = existing.close();
    }

    let init_script = format!(
        r#"
        setInterval(async () => {{
            let url = window.location.href;
            let success = false;
            let token = "active_session";

            if (url.includes('localhost:8080') || url.includes('openwebui')) {{
                let storedToken = localStorage.getItem('token');
                if (storedToken) {{
                    success = true;
                    token = storedToken;
                }}
            }} else if (url.includes('openai.com') || url.includes('chatgpt.com')) {{
                // Check if we are on the main chat page (pathname / or /c/ or /g/) AND a textarea is present
                if ((window.location.pathname === '/' || window.location.pathname.startsWith('/c/') || window.location.pathname.startsWith('/g/')) && document.querySelector('textarea')) {{
                    success = true;
                    try {{
                        let controller = new AbortController();
                        let timeoutId = setTimeout(() => controller.abort(), 1500);
                        let res = await fetch('/api/auth/session', {{ signal: controller.signal }});
                        clearTimeout(timeoutId);
                        if (res.ok) {{
                            let data = await res.json();
                            if (data && data.accessToken) {{
                                token = data.accessToken;
                            }}
                        }}
                    }} catch(e) {{
                        console.error('Fetch session error:', e);
                    }}
                }}
            }} else if (url.includes('claude.ai')) {{
                if (window.location.pathname === '/' || window.location.pathname.startsWith('/chat') || window.location.pathname.startsWith('/new')) {{
                    success = true;
                }}
            }} else if (url.includes('gemini.google.com')) {{
                if (window.location.pathname.startsWith('/app')) {{
                    success = true;
                }}
            }} else if (url.includes('chat.deepseek.com')) {{
                if (document.querySelector('textarea, div[contenteditable="true"]')) {{
                    success = true;
                }}
            }} else if (url.includes('chat.qwen.ai')) {{
                if (document.querySelector('textarea, div[contenteditable="true"]')) {{
                    success = true;
                }}
            }}

            if (success && !window.__loginRedirected) {{
                window.__loginRedirected = true;
                window.location.href = 'http://127.0.0.1:14285/oauth/callback?session_id={}&access_token=' + token + '&provider={}';
            }}
        }}, 2000);
    "#,
        session_id, session_key
    );

    let _ = tauri::WebviewWindowBuilder::new(
        &app,
        window_label,
        tauri::WebviewUrl::External(login_url.parse().unwrap()),
    )
    .title(format!("{} Login ({})", provider, account))
    .inner_size(800.0, 700.0)
    .resizable(true)
    .initialization_script(&init_script)
    .build()
    .map_err(|e| format!("Failed to build popup window: {}", e))?;

    Ok(LoginResponse {
        success: true,
        message: format!("Browser opened for {} login ({})", provider, account),
        session_id: Some(session_id),
        url: login_url.to_string(),
        callback_url: redirect_url,
    })
}

#[tauri::command]
pub async fn check_login_status(session_id: String) -> Result<TokenResponse, String> {
    let sessions = OAUTH_SESSIONS.lock().unwrap();
    if let Some(session) = sessions.get(&session_id) {
        if let Some(token) = &session.token {
            return Ok(TokenResponse {
                success: true,
                message: "Login complete".to_string(),
                token: Some(token.clone()),
            });
        }
        return Ok(TokenResponse {
            success: false,
            message: "Login in progress...".to_string(),
            token: None,
        });
    }
    Err("Session not found".to_string())
}

#[tauri::command]
pub async fn get_stored_token(provider: String) -> Result<Option<String>, String> {
    Ok(get_stored_token_sync(&provider))
}

/// List all stored sessions as lightweight summaries for the frontend account switcher.
#[tauri::command]
pub async fn list_webui_sessions(provider: Option<String>) -> Result<Vec<SessionSummary>, String> {
    let sessions = OAUTH_SESSIONS.lock().unwrap();
    let active_map = ACTIVE_SESSIONS.lock().unwrap();
    let provider_filter = provider.map(|p| webui_provider_and_account(&p).0);
    let mut result = Vec::new();
    for (sid, session) in sessions.iter() {
        if let Some(ref p) = provider_filter {
            if &session.provider != p {
                continue;
            }
        }
        let key = webui_session_key(&session.provider, &session.account);
        let active_sid = active_map.get(&key);
        result.push(SessionSummary {
            session_id: sid.clone(),
            provider: session.provider.clone(),
            display_name: session.display_name.clone(),
            has_token: session.token.is_some(),
            is_active: active_sid.map(|s| s == sid).unwrap_or(false),
        });
    }
    result.sort_by(|a, b| {
        a.provider
            .cmp(&b.provider)
            .then(a.display_name.cmp(&b.display_name))
    });
    Ok(result)
}

/// Switch the active session for a provider (used by the account switcher UI).
#[tauri::command]
pub async fn switch_webui_session(session_id: String) -> Result<(), String> {
    let session_key = {
        let sessions = OAUTH_SESSIONS.lock().unwrap();
        sessions
            .get(&session_id)
            .map(|s| webui_session_key(&s.provider, &s.account))
            .ok_or_else(|| format!("Session not found: {}", session_id))?
    };
    let mut active_map = ACTIVE_SESSIONS.lock().unwrap();
    active_map.insert(session_key, session_id);
    Ok(())
}

/// Delete a stored session (logout from that account).
#[tauri::command]
pub async fn delete_webui_session(session_id: String) -> Result<(), String> {
    let removed_key = {
        let mut sessions = OAUTH_SESSIONS.lock().unwrap();
        sessions
            .remove(&session_id)
            .map(|s| webui_session_key(&s.provider, &s.account))
    };
    if let Some(key) = removed_key {
        let mut active_map = ACTIVE_SESSIONS.lock().unwrap();
        if active_map.get(&key).map(|s| s == &session_id).unwrap_or(false) {
            active_map.remove(&key);
        }
    }
    Ok(())
}

fn webui_chat_url(provider: &str) -> Option<&'static str> {
    match provider {
        "claude" | "claude-webui" | "claude (webui)" => Some("https://claude.ai/new"),
        "gemini" | "gemini-webui" | "gemini (webui)" => Some("https://gemini.google.com/app"),
        "openai" | "gpt" | "chatgpt" | "openai-webui" | "openai (webui)" => {
            Some("https://chatgpt.com/")
        }
        "deepseek" | "deepseek-webui" | "deepseek (webui)" => Some("https://chat.deepseek.com/"),
        "qwen" | "qwen-webui" | "qwen (webui)" | "qwen-code" | "qwen code" => {
            Some("https://chat.qwen.ai/")
        }
        _ => None,
    }
}

fn webui_prompt_script(provider: &str, prompt: &str) -> Result<String, String> {
    let prompt_json =
        serde_json::to_string(prompt).map_err(|e| format!("Failed to encode prompt: {}", e))?;

    let selectors = match provider {
        "claude" | "claude-webui" | "claude (webui)" => (
            "div[contenteditable=\"true\"]",
            "button[aria-label=\"Send Message\"], button[aria-label=\"Send message\"], button[data-testid=\"send-button\"]",
        ),
        "gemini" | "gemini-webui" | "gemini (webui)" => (
            "div[contenteditable=\"true\"], rich-textarea div[contenteditable=\"true\"]",
            "button[aria-label=\"Send message\"], button[aria-label=\"Submit\"], button.send-button",
        ),
        "openai" | "gpt" | "chatgpt" | "openai-webui" | "openai (webui)" => (
            "textarea, div[contenteditable=\"true\"]",
            "button[data-testid=\"send-button\"], button[aria-label=\"Send prompt\"], button[aria-label=\"Send message\"]",
        ),
        "deepseek" | "deepseek-webui" | "deepseek (webui)" => (
            "textarea, div[contenteditable=\"true\"]",
            "button[type=\"submit\"], button[aria-label*=\"Send\"], button:has(svg)",
        ),
        "qwen" | "qwen-webui" | "qwen (webui)" | "qwen-code" | "qwen code" => (
            "textarea, div[contenteditable=\"true\"]",
            "button[type=\"submit\"], button[aria-label*=\"Send\"], button:has(svg)",
        ),
        _ => return Err(format!("Unsupported WebUI provider: {}", provider)),
    };

    Ok(format!(
        r#"
        (() => {{
            const prompt = {prompt_json};
            const input = document.querySelector({input_selector:?});
            if (!input) {{
                throw new Error('Prompt input not found for {provider}');
            }}
            input.focus();
            if ('value' in input) {{
                input.value = prompt;
                input.dispatchEvent(new InputEvent('input', {{ bubbles: true, inputType: 'insertText', data: prompt }}));
                input.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }} else {{
                input.textContent = '';
                document.execCommand('insertText', false, prompt);
                input.dispatchEvent(new InputEvent('input', {{ bubbles: true, inputType: 'insertText', data: prompt }}));
            }}
            setTimeout(() => {{
                const send = document.querySelector({send_selector:?});
                if (send && !send.disabled) {{
                    send.click();
                }}
            }}, 250);
            window.__hadesLastPrompt = prompt;
            true;
        }})();
    "#,
        prompt_json = prompt_json,
        provider = provider,
        input_selector = selectors.0,
        send_selector = selectors.1,
    ))
}

fn webui_response_observer_script(provider: &str, window_label: &str) -> Result<String, String> {
    let provider_json =
        serde_json::to_string(provider).map_err(|e| format!("Failed to encode provider: {}", e))?;
    let label_json = serde_json::to_string(window_label)
        .map_err(|e| format!("Failed to encode window label: {}", e))?;

    let selectors = match provider {
        "claude" | "claude-webui" | "claude (webui)" => {
            "[data-testid*=\"message\"], div.font-claude-message, div.prose"
        }
        "gemini" | "gemini-webui" | "gemini (webui)" => {
            "message-content, model-response, .model-response-text, .markdown"
        }
        "openai" | "gpt" | "chatgpt" | "openai-webui" | "openai (webui)" => {
            "[data-message-author-role=\"assistant\"], article div.markdown, .markdown"
        }
        "deepseek" | "deepseek-webui" | "deepseek (webui)" => {
            "[class*=\"message\"], [class*=\"answer\"], .markdown, article"
        }
        "qwen" | "qwen-webui" | "qwen (webui)" | "qwen-code" | "qwen code" => {
            "[class*=\"message\"], [class*=\"answer\"], .markdown, article"
        }
        _ => return Err(format!("Unsupported WebUI provider: {}", provider)),
    };

    Ok(format!(
        r#"
        (() => {{
            if (window.__hadesWebUiObserverInstalled) return true;
            window.__hadesWebUiObserverInstalled = true;
            const provider = {provider_json};
            const windowLabel = {label_json};
            const selectors = {selectors:?};
            let lastText = '';
            let lastSentAt = 0;

            const pickResponse = () => {{
                const nodes = Array.from(document.querySelectorAll(selectors));
                const texts = nodes
                    .map((node) => (node.innerText || node.textContent || '').trim())
                    .filter((text) => text.length > 20 && text !== window.__hadesLastPrompt);
                return texts[texts.length - 1] || '';
            }};

            const forward = () => {{
                const text = pickResponse();
                const now = Date.now();
                if (!text || text === lastText || now - lastSentAt < 900) return;
                lastText = text;
                lastSentAt = now;
                try {{
                    if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {{
                        window.__TAURI_INTERNALS__.invoke('save_webui_response', {{
                            payload: {{
                                provider,
                                window: windowLabel,
                                text,
                                url: window.location.href,
                            }}
                        }});
                    }}
                }} catch (e) {{
                    console.error('Failed to forward WebUI response:', e);
                }}
            }};

            const observer = new MutationObserver(() => {{
                clearTimeout(window.__hadesWebUiResponseTimer);
                window.__hadesWebUiResponseTimer = setTimeout(forward, 700);
            }});
            observer.observe(document.body, {{ childList: true, subtree: true, characterData: true }});
            setInterval(forward, 2500);
            true;
        }})();
    "#,
        provider_json = provider_json,
        label_json = label_json,
        selectors = selectors,
    ))
}

#[tauri::command]
pub async fn save_webui_response(
    app: tauri::AppHandle,
    payload: WebUiResponsePayload,
) -> Result<serde_json::Value, String> {
    let trimmed = payload.text.trim();
    if trimmed.is_empty() {
        return Err("Empty WebUI response".to_string());
    }
    // Deliver to the agent loop if one is awaiting this window's response.
    webui_deliver_response(&payload.window, trimmed.to_string());
    let _ = app.emit("webui-response", &payload);
    let provider = payload.provider.clone();
    let window = payload.window.clone();
    let url = payload.url.clone();
    let _ = app.emit(
        "ai-artifact",
        serde_json::json!({
            "type": "record",
            "title": format!("{} WebUI Response", provider),
            "description": format!("Captured {} characters from {}", trimmed.len(), provider),
            "path": url.clone().unwrap_or_else(|| window.clone()),
            "provider": provider,
            "window": window,
            "url": url,
            "metadata": {
                "kind": "webui_response",
                "content": trimmed,
            },
        }),
    );

    Ok(serde_json::json!({
        "success": true,
        "provider": payload.provider,
        "window": payload.window,
        "chars": trimmed.len(),
    }))
}

#[tauri::command]
pub async fn send_webui_prompt(
    app: tauri::AppHandle,
    provider: String,
    prompt: String,
) -> Result<serde_json::Value, String> {
    let (provider_key, account_key) = webui_provider_and_account(&provider);
    let url = webui_chat_url(&provider_key)
        .ok_or_else(|| format!("Unsupported WebUI provider: {}", provider))?;
    let label = format!(
        "webui_{}_{}",
        provider_key
            .replace(" (webui)", "")
            .replace("-webui", "")
            .replace(' ', "_"),
        account_key
    );

    let window = if let Some(existing) = app.get_webview_window(&label) {
        existing
    } else {
        tauri::WebviewWindowBuilder::new(
            &app,
            label.clone(),
            tauri::WebviewUrl::External(url.parse().map_err(|e| format!("Bad WebUI URL: {}", e))?),
        )
        .title(format!("{} Web Session ({})", provider_key, account_key))
        .inner_size(1100.0, 800.0)
        .resizable(true)
        .visible(false)
        .build()
        .map_err(|e| format!("Failed to open WebUI window: {}", e))?
    };

    let observer_script = webui_response_observer_script(&provider_key, &label)?;
    let script = webui_prompt_script(&provider_key, &prompt)?;
    let window_for_eval = window.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(900)).await;
        let _ = window_for_eval.eval(&observer_script);
        let _ = window_for_eval.eval(&script);
    });

    Ok(serde_json::json!({
        "success": true,
        "provider": provider_key,
        "account": account_key,
        "window": label,
        "url": url,
        "message": "Prompt sent to background WebUI session. The response will stream back into the VSCodium-Rust AI panel."
    }))
}

pub fn get_stored_token_sync(provider: &str) -> Option<String> {
    let (provider_key, account_key) = webui_provider_and_account(provider);
    let session_key = webui_session_key(&provider_key, &account_key);
    let sessions = OAUTH_SESSIONS.lock().unwrap();
    // Prefer the active session for this provider+account slot.
    let active_map = ACTIVE_SESSIONS.lock().unwrap();
    if let Some(active_sid) = active_map.get(&session_key) {
        if let Some(session) = sessions.get(active_sid) {
            if let Some(token) = &session.token {
                return Some(token.clone());
            }
        }
    }
    // Fallback: find any session with a token for this exact provider+account.
    for session in sessions.values() {
        if session.provider == provider_key && session.account == account_key {
            if let Some(token) = &session.token {
                return Some(token.clone());
            }
        }
    }
    None
}

// ============================================================================
// Helpers
// ============================================================================
#[allow(dead_code)]
fn open_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", &format!("start {}", url)])
            .output()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .output()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .output()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn toggle_webui_window_visibility(
    app: tauri::AppHandle,
    session_id: String,
) -> Result<bool, String> {
    let (provider_key, account_key) = {
        let sessions = OAUTH_SESSIONS.lock().unwrap();
        if let Some(session) = sessions.get(&session_id) {
            (session.provider.clone(), session.account.clone())
        } else {
            return Err(format!("Session not found: {}", session_id));
        }
    };

    let label = format!(
        "webui_{}_{}",
        provider_key
            .replace(" (webui)", "")
            .replace("-webui", "")
            .replace(' ', "_"),
        account_key
    );

    if let Some(window) = app.get_webview_window(&label) {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            let _ = window.hide();
            Ok(false)
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            Ok(true)
        }
    } else {
        Err(format!("No active WebUI window found for session: {}", session_id))
    }
}

// Removed legacy platform module decoration
