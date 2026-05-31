use std::process::Command;
use tauri::{AppHandle, Manager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserSession {
    pub provider: String,
    pub url: String,
    pub cookies: Option<String>,
    pub status: SessionStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionStatus {
    Disconnected,
    Connected,
    Authenticated,
}

pub struct BrowserController {
    app_handle: AppHandle,
}

impl BrowserController {
    pub fn new(app_handle: AppHandle) -> Self {
        BrowserController { app_handle }
    }

    pub async fn open_login_url(&self, provider: &str, redirect_url: &str) -> Result<String, String> {
        let (url, _browser_cmd) = match provider {
            "claude" => {
                let login_url = format!(
                    "https://claude.ai/login?continue={}",
                    urlencoding::encode(redirect_url)
                );
                (login_url, "chrome")
            }
            "gemini" => {
                let login_url = format!(
                    "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=https://www.googleapis.com/auth/generative-language.tuning&prompt=consent",
                    urlencoding::encode("your-client-id"),
                    urlencoding::encode(redirect_url)
                );
                (login_url, "chrome")
            }
            "openai" => {
                let login_url = format!(
                    "https://auth0.openai.com/u/login?redirect_uri={}",
                    urlencoding::encode(redirect_url)
                );
                (login_url, "chrome")
            }
            _ => {
                return Err(format!("Unknown provider: {}", provider));
            }
        };

        // Open browser to login
        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd")
            .args(&["/c", &format!("start {}", url)])
            .output();

        #[cfg(target_os = "linux")]
        let _ = Command::new("xdg-open").arg(&url).output();

        #[cfg(target_os = "macos")]
        let _ = Command::new("open").arg(&url).output();

        Ok(url)
    }

    pub async fn save_cookies(&self, session: &BrowserSession) -> Result<(), String> {
        // Save cookies to storage for future use
        let cookies_path = self.app_handle
            .path()
            .app_data_dir()
            .unwrap_or_default()
            .join(format!("cookies_{}.json", session.provider));
        
        let cookies_json = serde_json::to_string(session)
            .map_err(|e| format!("Failed to serialize cookies: {}", e))?;
        
        std::fs::write(&cookies_path, cookies_json)
            .map_err(|e| format!("Failed to save cookies: {}", e))?;
        
        Ok(())
    }

    pub async fn restore_cookies(&self, provider: &str) -> Result<BrowserSession, String> {
        let cookies_path = self.app_handle
            .path()
            .app_data_dir()
            .unwrap_or_default()
            .join(format!("cookies_{}.json", provider));

        if !cookies_path.exists() {
            return Err("No saved cookies found".to_string());
        }

        let content = std::fs::read_to_string(&cookies_path)
            .map_err(|e| format!("Failed to read cookies: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to deserialize cookies: {}", e))
    }

    pub async fn clear_cookies(&self, provider: &str) -> Result<(), String> {
        let cookies_path = self.app_handle
            .path()
            .app_data_dir()
            .unwrap_or_default()
            .join(format!("cookies_{}.json", provider));

        if cookies_path.exists() {
            std::fs::remove_file(&cookies_path)
                .map_err(|e| format!("Failed to clear cookies: {}", e))?;
        }

        Ok(())
    }

    pub async fn connect_to_openwebui(&self, url: String) -> Result<BrowserSession, String> {
        let session = BrowserSession {
            provider: "openwebui".to_string(),
            url,
            cookies: None,
            status: SessionStatus::Disconnected,
        };

        // Test connection
        let client = reqwest::Client::new();
        let test_url = format!("{}/api/v1/me", session.url.trim_end_matches('/'));
        
        match client.get(&test_url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                Ok(BrowserSession {
                    status: SessionStatus::Connected,
                    ..session
                })
            }
            Ok(_) => Err("OpenWebUI connection failed - check URL".to_string()),
            Err(e) => Err(format!("Connection error: {}", e)),
        }
    }
}
