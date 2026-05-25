use crate::ai_engine::{AiRequest, ChatMessage, MessageContent};
use crate::provider_manager::ProviderManager;
use crate::browser_actuation::BrowserController;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenWebUIClient {
    provider_key: String,
    session_id: String,
    base_url: String,
    auth_token: Option<String>,
}

impl OpenWebUIClient {
    pub async fn new(provider_key: &str, manager: &ProviderManager) -> Result<Self, String> {
        let config = manager.get_provider_config(provider_key)
            .ok_or("Provider not found")?;
        
        let session_id = uuid::Uuid::new_v4().to_string();
        
        Ok(Self {
            provider_key: provider_key.to_string(),
            session_id,
            base_url: config.base_url.clone(),
            auth_token: None,
        })
    }

    pub async fn login(&mut self, browser_controller: &BrowserController) -> Result<(), String> {
        // Open login page
        let login_url = format!("{}/login", self.base_url);
        
        #[cfg(target_os = "windows")]
        let _ = std::process::Command::new("cmd")
            .args(&["/c", &format!("start {}", login_url)])
            .output();

        #[cfg(target_os = "linux")]
        let _ = std::process::Command::new("xdg-open").arg(&login_url).output();

        #[cfg(target_os = "macos")]
        let _ = std::process::Command::new("open").arg(&login_url).output();

        // Wait for user to login (in a real implementation, you'd have a WebSocket or polling)
        println!("Please complete login in the browser...");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        // Try to get auth token
        if let Ok(token) = self.get_auth_token().await {
            self.auth_token = Some(token);
            Ok(())
        } else {
            Err("Login failed - please check credentials")
        }
    }

    async fn get_auth_token(&self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let auth_url = format!("{}/api/v1/auth/me", self.base_url);
        
        match client.get(&auth_url)
            .header("Authorization", "Bearer test-token") // This would be dynamic
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                // Extract token from response
                Ok("mock-auth-token".to_string()) // Replace with actual token extraction
            },
            Ok(_) => Err("Authentication failed".to_string()),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn chat_completion(&self, messages: &[ChatMessage]) -> Result<String, String> {
        let client = reqwest::Client::new();
        let chat_url = format!("{}/api/v1/chat/completions", self.base_url);
        
        let request = serde_json::json!({
            "model": "claude-3-5-sonnet", // Default model
            "messages": messages.iter().map(|m| {
                json!({
                    "role": m.role,
                    "content": m.content.as_ref().map(|c| c.to_text()).unwrap_or_default()
                })
            }).collect::<Vec<_>>(),
            "temperature": 0.7,
            "stream": false,
        });

        let auth_header = if let Some(token) = &self.auth_token {
            format!("Bearer {}", token)
        } else {
            "Bearer test-token".to_string()
        };

        let response = client.post(&chat_url)
            .header("Authorization", auth_header)
            .json(&request)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.status().is_success() {
            let response_data: serde_json::Value = response.json()
                .await
                .map_err(|e| format!("Response parsing failed: {}", e))?;
            
            if let Some(content) = response_data["choices"][0]["message"]["content"].as_str() {
                Ok(content.to_string())
            } else {
                Err("No content in response".to_string())
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", response.status(), error_text))
        }
    }
}