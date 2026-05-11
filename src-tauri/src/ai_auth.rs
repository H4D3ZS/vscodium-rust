use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl, State, Emitter};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::EditorState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiSession {
    pub provider: String,
    pub cookies: String,
    pub user_agent: String,
    pub org_id: Option<String>,
}

pub struct AuthState {
    pub sessions: tokio::sync::Mutex<HashMap<String, AiSession>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            sessions: tokio::sync::Mutex::new(HashMap::new()),
        }
    }

    pub async fn login(&self, _provider: String) -> Result<(), String> {
        // Mock login successful logic
        Ok(())
    }
}

/// URL of the official API key console for each provider. This is the page we
/// want the user's default browser to land on, because:
///   - You can copy a key in 5 seconds.
///   - It bypasses CAPTCHAs that fire inside an embedded WebView.
///   - We don't have to scrape `document.cookie` (which can't see the
///     httpOnly cookies these providers actually use for auth anyway).
fn api_key_console_url(provider: &str) -> Option<&'static str> {
    match provider {
        "claude" | "anthropic" => Some("https://console.anthropic.com/settings/keys"),
        "gemini" | "google" => Some("https://aistudio.google.com/app/apikey"),
        "openai" => Some("https://platform.openai.com/api-keys"),
        "groq" => Some("https://console.groq.com/keys"),
        "openrouter" => Some("https://openrouter.ai/keys"),
        "mistral" => Some("https://console.mistral.ai/api-keys"),
        "xai" => Some("https://console.x.ai/"),
        _ => None,
    }
}

/// URL of the chat product (cookie-capture path) for each provider. Kept for
/// the experimental "use my paid web subscription" flow.
fn chat_product_url(provider: &str) -> Option<&'static str> {
    match provider {
        "claude" => Some("https://claude.ai/login"),
        "gemini" => Some("https://gemini.google.com/app"),
        _ => None,
    }
}

/// Cross-platform "open this URL in the user's default browser." Used by the
/// new `open_ai_login` Tauri command. We deliberately use the system browser
/// here rather than an in-app WebView because:
///   - The user is more likely to already be logged in to Anthropic/Google
///     in their normal browser, which means zero extra friction.
///   - Modern providers fingerprint embedded WebViews and reject sign-in.
///   - The API key page is a single copy-paste, so we don't need the IDE to
///     intercept anything; we just need to deposit the user there.
fn open_url_in_system_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // `cmd /C start "" "<url>"` ⇒ open with the user's default browser.
        // The empty `""` is the window title arg that `start` requires when
        // the path is itself a quoted URL.
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
            .map_err(|e| format!("cmd /C start failed: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("open command failed: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| format!("xdg-open failed: {}", e))?;
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        return Err(format!("Unsupported platform for opening URL: {}", url));
    }
    Ok(())
}

/// Open a Tauri-managed WebView pointed at a provider's chat product. This is
/// the "scrape a web-subscription session" path — it's experimental, fragile,
/// and won't work for providers that use httpOnly cookies (which is all of
/// them). Kept for completeness; the `open_ai_login` command below uses the
/// system browser path by default.
pub async fn open_login_window(app: AppHandle, provider: String) -> Result<(), String> {
    let url = chat_product_url(&provider)
        .ok_or_else(|| format!("No chat product URL for provider '{}'", provider))?;

    let label = format!("login-{}", provider);

    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.close();
    }

    let parsed = url.parse().map_err(|e| format!("bad URL '{}': {:?}", url, e))?;
    let win = WebviewWindowBuilder::new(&app, label, WebviewUrl::External(parsed))
        .title(format!("Login to {}", provider))
        .inner_size(900.0, 700.0)
        .build()
        .map_err(|e| e.to_string())?;

    win.show().map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn save_session(state: &AuthState, session: AiSession) {
    let mut sessions = state.sessions.lock().await;
    sessions.insert(session.provider.clone(), session);
}

pub async fn get_session(state: &AuthState, provider: &str) -> Option<AiSession> {
    let sessions = state.sessions.lock().await;
    sessions.get(provider).cloned()
}

pub async fn capture_session(app: AppHandle, provider: String) -> Result<AiSession, String> {
    let label = format!("login-{}", provider);
    let win = app.get_webview_window(&label).ok_or("Login window not found")?;

    // Tauri v2: the renderer global lives at `window.__TAURI_INTERNALS__` and
    // we route events via the public IPC `invoke` instead of `__TAURI__.event`.
    // Posting the captured payload back to a dedicated command keeps the
    // capture inside Rust where we can persist it atomically.
    let js = format!(r#"
        (function() {{
            const data = {{
                provider: {provider:?},
                cookies: document.cookie,
                userAgent: navigator.userAgent
            }};
            try {{
                if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {{
                    window.__TAURI_INTERNALS__.invoke('save_ai_session', {{ session: {{
                        provider: data.provider,
                        cookies: data.cookies,
                        user_agent: data.userAgent,
                        org_id: null
                    }} }});
                }} else if (window.__TAURI__ && window.__TAURI__.event) {{
                    window.__TAURI__.event.emit('session-captured', data);
                }}
            }} catch (e) {{
                console.error('Failed to forward session:', e);
            }}
        }})();
    "#, provider = provider);

    win.eval(&js).map_err(|e| e.to_string())?;

    // We don't have synchronous access to `document.cookie`; the renderer JS
    // forwards the payload to `save_ai_session`. Return a placeholder so the
    // caller knows the eval succeeded.
    Ok(AiSession {
        provider,
        cookies: "PENDING_RENDERER_FORWARD".to_string(),
        user_agent: String::new(),
        org_id: None,
    })
}

// ─── Helpers exposed for system_commands::open_ai_login ─────────────────

/// Open the API key console for `provider` in the user's default browser.
/// Returns the URL that was opened so the frontend can surface a "we opened
/// <url> for you" toast.
pub fn open_api_key_console(provider: &str) -> Result<String, String> {
    let provider = provider.to_ascii_lowercase();
    let url = api_key_console_url(&provider).ok_or_else(|| {
        format!(
            "Unknown provider '{}'. Supported: claude, gemini, openai, groq, openrouter, mistral, xai.",
            provider
        )
    })?;
    open_url_in_system_browser(url)?;
    Ok(url.to_string())
}

// ─── Tauri commands ───────────────────────────────────────────────────────

/// Lets the frontend ask Tauri to scrape `document.cookie` out of the
/// open-login WebView. Real cookie capture happens in `capture_session`'s
/// renderer-side JS, which posts back via `save_ai_session`.
#[tauri::command]
pub async fn capture_ai_session_now(
    app: AppHandle,
    provider: String,
) -> Result<AiSession, String> {
    capture_session(app, provider).await
}

/// Tells the frontend which mode is supported for which provider. Surfaces
/// the actual support matrix so the UI can render the correct button label.
#[tauri::command]
pub fn provider_login_capabilities(provider: String) -> Value {
    let provider = provider.to_ascii_lowercase();
    serde_json::json!({
        "provider": provider,
        "api_key_url": api_key_console_url(&provider),
        "webview_url": chat_product_url(&provider),
        "supports_api_key": api_key_console_url(&provider).is_some(),
        "supports_webview": chat_product_url(&provider).is_some(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_console_url_covers_supported_providers() {
        // The set the UI offers. If we add a row in AgentSettingsView's API
        // keys section, we must add the URL here or the Connect button breaks.
        for p in ["claude", "anthropic", "gemini", "google", "openai", "groq", "openrouter", "mistral", "xai"] {
            assert!(api_key_console_url(p).is_some(), "missing api key URL for {p}");
        }
    }

    #[test]
    fn api_key_console_url_returns_none_for_unknown() {
        assert!(api_key_console_url("not-a-provider").is_none());
        assert!(api_key_console_url("").is_none());
    }

    #[test]
    fn chat_product_url_only_covers_browser_login_eligible_providers() {
        // Cookie-scrape login only makes sense for chat products with a
        // user-facing web UI. API-only providers should NOT have a chat URL —
        // surfacing one would suggest the cookie-scrape path could work for
        // them, which it can't.
        assert!(chat_product_url("claude").is_some());
        assert!(chat_product_url("gemini").is_some());
        assert!(chat_product_url("openai").is_none(), "OpenAI has no cookie-scrape product page");
        assert!(chat_product_url("groq").is_none());
    }

    #[test]
    fn provider_login_capabilities_shape() {
        let v = provider_login_capabilities("claude".to_string());
        assert_eq!(v["provider"], "claude");
        assert!(v["supports_api_key"].as_bool().unwrap());
        assert!(v["supports_webview"].as_bool().unwrap());
        assert_eq!(v["api_key_url"], "https://console.anthropic.com/settings/keys");
    }

    #[test]
    fn provider_login_capabilities_handles_unknown() {
        let v = provider_login_capabilities("ufo-cloud".to_string());
        assert_eq!(v["provider"], "ufo-cloud");
        assert_eq!(v["supports_api_key"].as_bool(), Some(false));
        assert_eq!(v["supports_webview"].as_bool(), Some(false));
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ApiKeys {
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub google: Option<String>,
    pub groq: Option<String>,
    pub openrouter: Option<String>,
    pub mistral: Option<String>,
    pub xai: Option<String>,
    pub alibaba: Option<String>,
    pub apiradar: Option<String>,
    pub elevenlabs_api_key: Option<String>,
    pub elevenlabs_voice_id: Option<String>,
}

#[tauri::command]
pub async fn get_api_keys(state: State<'_, EditorState>) -> Result<ApiKeys, String> {
    let path = state.config_dir.join("api_keys.json");
    if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
        Ok(ApiKeys::default())
    }
}

#[tauri::command]
pub async fn save_api_keys(
    state: State<'_, EditorState>,
    keys: Value,
) -> Result<HashMap<String, String>, String> {
    let path = state.config_dir.join("api_keys.json");
    let mut merged = if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str::<ApiKeys>(&content).unwrap_or_default()
    } else {
        ApiKeys::default()
    };

    let incoming: ApiKeys = serde_json::from_value(keys).map_err(|e| e.to_string())?;
    
    if incoming.openai.is_some() { merged.openai = incoming.openai; }
    if incoming.anthropic.is_some() { merged.anthropic = incoming.anthropic; }
    if incoming.google.is_some() { merged.google = incoming.google; }
    if incoming.elevenlabs_api_key.is_some() { merged.elevenlabs_api_key = incoming.elevenlabs_api_key; }

    let content = serde_json::to_string_pretty(&merged).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())?;

    let mut results = HashMap::new();
    results.insert("status".to_string(), "Success".to_string());
    Ok(results)
}

#[tauri::command]
pub async fn save_api_key(
    state: State<'_, EditorState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let mut keys = get_api_keys(state.clone()).await?;
    match key.as_str() {
        "openai" => keys.openai = Some(value),
        "anthropic" => keys.anthropic = Some(value),
        "google" => keys.google = Some(value),
        "elevenlabs_api_key" => keys.elevenlabs_api_key = Some(value),
        _ => return Err(format!("Unsupported key: {}", key)),
    }
    let path = state.config_dir.join("api_keys.json");
    let content = serde_json::to_string_pretty(&keys).map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub async fn hunt_api_keys(
    _state: State<'_, EditorState>,
    app: AppHandle,
    provider: String,
) -> Result<Vec<crate::hunter::HuntResult>, String> {
    use crate::domain::HuntProgress;
    
    let hunter = crate::hunter::ApiRadarHunter::new();
    let _ = app.emit("hunt-progress", HuntProgress { msg: format!("Hunting for {} leaks...", provider) });
    
    let leaks = hunter.fetch_recent_leaks(&provider).await.map_err(|e| e.to_string())?;
    let mut results = Vec::new();

    for leak in leaks {
         results.push(crate::hunter::HuntResult {
             provider: leak.provider,
             key: leak.redacted_key,
             key_type: "Leaked".to_string(),
             source: "ApiRadar".to_string(),
             repo_url: leak.repo_url,
             is_live: false,
             details: format!("Found in file: {}", leak.file_path),
         });
    }

    Ok(results)
}

#[tauri::command]
pub async fn save_ai_session(
    state: State<'_, EditorState>,
    session: AiSession,
) -> Result<(), String> {
    save_session(&state.auth_state, session).await;
    Ok(())
}

#[tauri::command]
pub async fn capture_ai_session(
    _state: State<'_, EditorState>,
    app: AppHandle,
    provider: String,
) -> Result<AiSession, String> {
    capture_session(app, provider).await
}
