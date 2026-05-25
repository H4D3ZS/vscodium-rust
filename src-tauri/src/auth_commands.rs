use tauri::{Emitter, Manager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Holds active OAuth sessions keyed by session_id
lazy_static::lazy_static! {
    pub static ref OAUTH_SESSIONS: Arc<Mutex<HashMap<String, OAuthSession>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthSession {
    pub provider: String,
    pub redirect_url: String,
    pub state: String,
    pub token: Option<String>,
    pub token_expires: Option<i64>,
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
                .map(|ch| if ch.is_ascii_alphanumeric() { ch.to_ascii_lowercase() } else { '_' })
                .collect::<String>()
                .trim_matches('_')
                .to_string()
        })
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "default".to_string());
    (canonical, account)
}

// ============================================================================
// OAuth Callback Server – runs in background and captures the token
// ============================================================================
pub async fn start_oauth_listener(port: u16, app: tauri::AppHandle) -> Result<(), String> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| format!("Failed to bind OAuth listener: {}", e))?;

    println!("[OAuth] Callback listener running on http://127.0.0.1:{}", port);

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
                                provider_name = session.provider.clone();
                                success = true;
                            }
                        }
                    }

                    if success {
                        use tauri::Manager;
                        if !provider_name.is_empty() {
                            let window_label = format!("login_{}", provider_name);
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
    let provider = webui_canonical_provider(&request.provider);

    // Build provider-specific login URL
    let (login_url, base_url) = match provider.as_str() {
        "claude" => (
            "https://claude.ai/login",
            "https://claude.ai",
        ),
        "gemini" => (
            "https://gemini.google.com/app",
            "https://gemini.google.com",
        ),
        "openai" | "gpt" => (
            "https://chatgpt.com/auth/login",
            "https://chatgpt.com",
        ),
        "openwebui" | "openwebui-claude" | "openwebui-gpt" | "openwebui-gemini" => (
            "http://localhost:8080/auth",
            "http://localhost:8080",
        ),
        "deepseek" => (
            "https://chat.deepseek.com/sign_in",
            "https://chat.deepseek.com",
        ),
        "qwen" => (
            "https://chat.qwen.ai/",
            "https://chat.qwen.ai",
        ),
        other => return Err(format!("Unsupported provider: {}", other)),
    };

    // Generate session
    let session_id = Uuid::new_v4().to_string();
    let redirect_url = format!("{}/oauth/callback?session_id={}&provider={}",
        base_url, session_id, provider);

    // Store session
    {
        let mut sessions = OAUTH_SESSIONS.lock().unwrap();
        sessions.insert(session_id.clone(), OAuthSession {
            provider: provider.clone(),
            redirect_url: redirect_url.clone(),
            state: session_id.clone(),
            token: None,
            token_expires: None,
        });
    }

    // Open popup window instead of default browser
    let window_label = format!("login_{}", provider);
    if let Some(existing) = app.get_webview_window(&window_label) {
        let _ = existing.close();
    }

    let init_script = format!(r#"
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
    "#, session_id, provider);

    let _ = tauri::WebviewWindowBuilder::new(
        &app,
        window_label,
        tauri::WebviewUrl::External(login_url.parse().unwrap()),
    )
    .title(format!("{} Login", provider))
    .inner_size(800.0, 700.0)
    .resizable(true)
    .initialization_script(&init_script)
    .build()
    .map_err(|e| format!("Failed to build popup window: {}", e))?;

    Ok(LoginResponse {
        success: true,
        message: format!("Browser opened for {} login", provider),
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
    let sessions = OAUTH_SESSIONS.lock().unwrap();
    for session in sessions.values() {
        if session.provider == provider {
            if let Some(token) = &session.token {
                return Ok(Some(token.clone()));
            }
        }
    }
    Ok(None)
}

fn webui_chat_url(provider: &str) -> Option<&'static str> {
    match provider {
        "claude" | "claude-webui" | "claude (webui)" => Some("https://claude.ai/new"),
        "gemini" | "gemini-webui" | "gemini (webui)" => Some("https://gemini.google.com/app"),
        "openai" | "gpt" | "chatgpt" | "openai-webui" | "openai (webui)" => Some("https://chatgpt.com/"),
        "deepseek" | "deepseek-webui" | "deepseek (webui)" => Some("https://chat.deepseek.com/"),
        "qwen" | "qwen-webui" | "qwen (webui)" | "qwen-code" | "qwen code" => Some("https://chat.qwen.ai/"),
        _ => None,
    }
}

fn webui_prompt_script(provider: &str, prompt: &str) -> Result<String, String> {
    let prompt_json = serde_json::to_string(prompt)
        .map_err(|e| format!("Failed to encode prompt: {}", e))?;

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

    Ok(format!(r#"
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
    let provider_json = serde_json::to_string(provider)
        .map_err(|e| format!("Failed to encode provider: {}", e))?;
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

    Ok(format!(r#"
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
    let _ = app.emit("webui-response", &payload);
    let provider = payload.provider.clone();
    let window = payload.window.clone();
    let url = payload.url.clone();
    let _ = app.emit("ai-artifact", serde_json::json!({
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
    }));

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
    let sessions = OAUTH_SESSIONS.lock().unwrap();
    for session in sessions.values() {
        if session.provider == provider {
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

#[cfg(not(target_env = "msvc"))]
mod platform;
