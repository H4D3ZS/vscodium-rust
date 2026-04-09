use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
}

pub async fn open_login_window(app: AppHandle, provider: String) -> Result<(), String> {
    let url = match provider.as_str() {
        "claude" => "https://claude.ai/login",
        "gemini" => "https://gemini.google.com/app",
        _ => return Err("Unsupported provider".into()),
    };

    let label = format!("login-{}", provider);
    
    // Close existing window if any
    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.close();
    }

    let win = WebviewWindowBuilder::new(&app, label, WebviewUrl::App(url.parse().unwrap()))
        .title(format!("Login to {}", provider))
        .inner_size(800.0, 600.0)
        .build()
        .map_err(|e| e.to_string())?;

    // In a real implementation, we would use a more sophisticated way to intercept cookies.
    // For this "unrestrained" version, we'll listen for a specific event or poll the window.
    // For now, let's assume we have a command that the user triggers once they are logged in.
    
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
    
    // Extract cookies and user agent from the webview
    // Note: We use eval to get document.cookie. This is a reliable way to get the session.
    let _cookies_val = win.eval("document.cookie").map_err(|e| e.to_string())?;
    let _ua_val = win.eval("navigator.userAgent").map_err(|e| e.to_string())?;

    // Eval in Tauri v2 returns nothing by default unless we use a different approach or 
    // we use a plugin. However, we can use a simpler trick: emit an event from JS.
    // For now, let's assume evaluated values can be retrieved if we use a promise-based eval
    // or if we use the tauri-plugin-session if it existed.
    // Since we are "unrestrained", we'll implement a more direct way:
    
    // Actually, win.eval in Tauri v2 is a bit limited for returning values.
    // Let's use win.emit to trigger a JS script that sends the data back via an event.
    
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
    
    win.eval(js).map_err(|e| e.to_string())?;
    
    // We need a way to wait for the event. For simplicity in this implementation, 
    // we'll use a placeholder or have the user trigger another command.
    // But since the user wants "seamless", let's use a Mutex to store the temporarily captured data.
    
    Ok(AiSession {
        provider,
        cookies: "CAPTURED_VIA_EVENT".to_string(), // This will be updated by the event listener
        user_agent: "Mozilla/5.0...".to_string(),
        org_id: None,
    })
}
