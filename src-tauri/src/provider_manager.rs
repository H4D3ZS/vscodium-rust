use tauri::AppHandle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub auth_type: AuthType,
    pub model_mapping: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthType {
    OpenWebUI { url: String },
    OpenAI,
    Anthropic,
    Google,
    Custom { headers: HashMap<String, String> },
}

pub struct ProviderManager {
    providers: HashMap<String, ProviderConfig>,
    browser_controller: crate::browser_actuation::BrowserController,
}

impl ProviderManager {
    pub fn new(app_handle: AppHandle) -> Self {
        let browser_controller = crate::browser_actuation::BrowserController::new(app_handle.clone());
        
        let mut providers = HashMap::new();
        
        // OpenWebUI providers
        providers.insert("claude-openwebui".to_string(), ProviderConfig {
            name: "Claude (OpenWebUI)".to_string(),
            base_url: "http://localhost:8080".to_string(),
            auth_type: AuthType::OpenWebUI { url: "http://localhost:8080".to_string() },
            model_mapping: vec![
                ("claude-3-5-sonnet-20241022".to_string(), "claude-3-5-sonnet".to_string()),
                ("claude-3-haiku-20240307".to_string(), "claude-3-haiku".to_string()),
            ].into_iter().collect(),
        });

        providers.insert("gemini-openwebui".to_string(), ProviderConfig {
            name: "Gemini (OpenWebUI)".to_string(),
            base_url: "http://localhost:8080".to_string(),
            auth_type: AuthType::OpenWebUI { url: "http://localhost:8080".to_string() },
            model_mapping: vec![
                ("gemini-1.5-pro".to_string(), "gemini-1.5-pro-exp-0827".to_string()),
                ("gemini-1.5-flash".to_string(), "gemini-1.5-flash-exp-0827".to_string()),
            ].into_iter().collect(),
        });

        providers.insert("gpt-openwebui".to_string(), ProviderConfig {
            name: "GPT (OpenWebUI)".to_string(),
            base_url: "http://localhost:8080".to_string(),
            auth_type: AuthType::OpenWebUI { url: "http://localhost:8080".to_string() },
            model_mapping: vec![
                ("gpt-4o".to_string(), "gpt-4o".to_string()),
                ("gpt-4o-mini".to_string(), "gpt-4o-mini".to_string()),
            ].into_iter().collect(),
        });

        providers.insert("deepseek-webui".to_string(), ProviderConfig {
            name: "DeepSeek (WebUI)".to_string(),
            base_url: "https://chat.deepseek.com".to_string(),
            auth_type: AuthType::OpenWebUI { url: "https://chat.deepseek.com".to_string() },
            model_mapping: vec![
                ("deepseek-chat".to_string(), "deepseek-chat".to_string()),
                ("deepseek-reasoner".to_string(), "deepseek-reasoner".to_string()),
            ].into_iter().collect(),
        });

        providers.insert("qwen-webui".to_string(), ProviderConfig {
            name: "Qwen (WebUI)".to_string(),
            base_url: "https://chat.qwen.ai".to_string(),
            auth_type: AuthType::OpenWebUI { url: "https://chat.qwen.ai".to_string() },
            model_mapping: vec![
                ("qwen-code".to_string(), "qwen-code".to_string()),
                ("qwen-chat".to_string(), "qwen-chat".to_string()),
            ].into_iter().collect(),
        });

        Self {
            providers,
            browser_controller,
        }
    }

    pub async fn login_to_provider(&self, provider_key: &str) -> Result<String, String> {
        let provider = self.providers.get(provider_key)
            .ok_or(format!("Provider not found: {}", provider_key))?;

        match &provider.auth_type {
            AuthType::OpenWebUI { url } => {
                self.browser_controller.open_login_url(provider_key, url).await
            },
            _ => Err("Unsupported auth type".to_string()),
        }
    }

    pub async fn get_provider_config(&self, provider_key: &str) -> Option<&ProviderConfig> {
        self.providers.get(provider_key)
    }

    pub fn list_providers(&self) -> Vec<&ProviderConfig> {
        self.providers.values().collect()
    }
}
