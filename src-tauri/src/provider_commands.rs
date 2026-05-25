use tauri::State;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AiProviderType {
    WebUI, // Browser-based OpenWebUI login
    ApiKey, // Traditional API key
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderOption {
    pub id: String,
    pub name: String,
    pub provider_type: AiProviderType,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebUIProvider {
    pub id: String,
    pub name: String,
    pub url: String,
    pub description: String,
}

impl ProviderOption {
    pub fn new(id: String, name: String, provider_type: AiProviderType, description: String) -> Self {
        Self {
            id,
            name,
            provider_type,
            description,
        }
    }
}

pub struct ProviderManager {
    pub options: Vec<ProviderOption>,
    pub webui_providers: Vec<WebUIProvider>,
}

impl ProviderManager {
    pub fn new() -> Self {
        let options = vec![
            ProviderOption::new("claude-webui".to_string(), "Claude (Free WebUI)".to_string(), AiProviderType::WebUI, "Login via browser to Claude's free web interface".to_string()),
            ProviderOption::new("gemini-webui".to_string(), "Gemini (Free WebUI)".to_string(), AiProviderType::WebUI, "Login via browser to Gemini's free web interface".to_string()),
            ProviderOption::new("gpt-webui".to_string(), "GPT (Free WebUI)".to_string(), AiProviderType::WebUI, "Login via browser to OpenAI's free web interface".to_string()),
            ProviderOption::new("deepseek-webui".to_string(), "DeepSeek (Free WebUI)".to_string(), AiProviderType::WebUI, "Login via browser to DeepSeek's free web interface".to_string()),
            ProviderOption::new("qwen-webui".to_string(), "Qwen (Free WebUI)".to_string(), AiProviderType::WebUI, "Login via browser to Qwen's free web interface".to_string()),
            ProviderOption::new("claude-api".to_string(), "Claude (API)".to_string(), AiProviderType::ApiKey, "Use API key for unlimited access".to_string()),
            ProviderOption::new("gemini-api".to_string(), "Gemini (API)".to_string(), AiProviderType::ApiKey, "Use API key for unlimited access".to_string()),
            ProviderOption::new("gpt-api".to_string(), "GPT (API)".to_string(), AiProviderType::ApiKey, "Use API key for unlimited access".to_string()),
        ];
        
        let webui_providers = vec![
            WebUIProvider {
                id: "claude-webui".to_string(),
                name: "Claude WebUI".to_string(),
                url: "https://claude.ai".to_string(),
                description: "Free Claude access via browser login".to_string(),
            },
            WebUIProvider {
                id: "gemini-webui".to_string(),
                name: "Gemini WebUI".to_string(),
                url: "https://gemini.google.com".to_string(),
                description: "Free Gemini access via browser login".to_string(),
            },
            WebUIProvider {
                id: "gpt-webui".to_string(),
                name: "GPT WebUI".to_string(),
                url: "https://chatgpt.com".to_string(),
                description: "Free GPT access via browser login".to_string(),
            },
            WebUIProvider {
                id: "deepseek-webui".to_string(),
                name: "DeepSeek WebUI".to_string(),
                url: "https://chat.deepseek.com".to_string(),
                description: "Free DeepSeek access via browser login".to_string(),
            },
            WebUIProvider {
                id: "qwen-webui".to_string(),
                name: "Qwen WebUI".to_string(),
                url: "https://chat.qwen.ai".to_string(),
                description: "Free Qwen access via browser login".to_string(),
            },
        ];
        
        Self {
            options,
            webui_providers,
        }
    }
    
    pub fn get_provider_options(&self) -> Vec<ProviderOption> {
        self.options.clone()
    }
    
    pub fn get_webui_provider(&self, id: &str) -> Option<&WebUIProvider> {
        self.webui_providers.iter().find(|p| p.id == id)
    }
}

#[tauri::command]
pub async fn get_provider_options() -> Vec<ProviderOption> {
    let manager = ProviderManager::new();
    manager.get_provider_options()
}

#[tauri::command]
pub async fn start_webui_login(
    webui_id: String,
) -> Result<serde_json::Value, String> {
    let manager = ProviderManager::new();
    
    if let Some(provider) = manager.get_webui_provider(&webui_id) {
        // Open browser to the provider's login page
        let login_url = format!("{}/login", provider.url);
        
        #[cfg(target_os = "windows")]
        let _ = std::process::Command::new("cmd")
            .args(&["/c", &format!("start {}", login_url)])
            .output();
            
        #[cfg(target_os = "linux")]
        let _ = std::process::Command::new("xdg-open").arg(&login_url).output();
            
        #[cfg(target_os = "macos")]
        let _ = std::process::Command::new("open").arg(&login_url).output();
        
        Ok(serde_json::json!({
            "success": true,
            "provider": provider.name,
            "message": "Browser opened for login",
            "url": provider.url,
            "login_url": login_url
        }))
    } else {
        Err("Provider not found".to_string())
    }
}
