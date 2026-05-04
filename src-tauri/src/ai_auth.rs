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

pub async fn open_login_window(app: AppHandle, provider: String) -> Result<(), String> {
    let url = match provider.as_str() {
        "claude" => "https://claude.ai/login",
        "gemini" => "https://gemini.google.com/app",
        _ => return Err("Unsupported provider".into()),
    };

    let label = format!("login-{}", provider);
    
    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.close();
    }

    let win = WebviewWindowBuilder::new(&app, label, WebviewUrl::App(url.parse().unwrap()))
        .title(format!("Login to {}", provider))
        .inner_size(800.0, 600.0)
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
    
    let js = format!(r#"
        (function() {{
            const data = {{
                provider: "{}",
                cookies: document.cookie,
                userAgent: navigator.userAgent
            }};
            window.__TAURI__.event.emit('session-captured', data);
        }})();
    "#, provider);
    
    win.eval(&js).map_err(|e| e.to_string())?;
    
    Ok(AiSession {
        provider,
        cookies: "CAPTURED_VIA_EVENT".to_string(),
        user_agent: "Mozilla/5.0...".to_string(),
        org_id: None,
    })
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
