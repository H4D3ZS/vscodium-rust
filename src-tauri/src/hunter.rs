use serde::{Deserialize, Serialize};
use reqwest::Client;
use regex::Regex;
use anyhow::{Result, anyhow};
use std::time::Duration;
use std::path::Path;

const ALLOWED_EXTENSIONS: &[&str] = &[
    "xml", "json", "properties", "sql", "txt", "log", "tmp", "backup", "bak", "enc",
    "yml", "yaml", "toml", "ini", "config", "conf", "cfg", "env", "envrc", "prod",
    "secret", "private", "key"
];

const KEYNAME_INDICATORS: &[&str] = &[
    "access_key", "secret_key", "access_token", "api_key", "apikey", "api_secret",
    "apiSecret", "app_secret", "application_key", "app_key", "appkey", "auth_token",
    "authsecret", "bearer", "token", "secret", "credentials"
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiRadarLeak {
    pub id: String,
    pub provider: String,
    #[serde(rename = "redactedKey")]
    pub redacted_key: String,
    #[serde(rename = "repoUrl")]
    pub repo_url: String,
    #[serde(rename = "filePath")]
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HuntResult {
    pub provider: String,
    pub key: String,
    pub key_type: String,
    pub source: String,
    pub repo_url: String,
    pub is_live: bool,
    pub details: String,
}

#[derive(Deserialize)]
struct LeaksResponse {
    leaks: Vec<ApiRadarLeak>,
}

pub struct ApiRadarHunter {
    client: Client,
    branches: Vec<&'static str>,
}

impl ApiRadarHunter {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("AIRI-Hunter/1.0")
                .build()
                .unwrap_or_default(),
            branches: vec!["main", "master", "dev", "prod", "develop", "staging", "v1"],
        }
    }

    pub async fn fetch_recent_leaks(&self, provider: &str) -> Result<Vec<ApiRadarLeak>> {
        let url = if provider == "all" {
            "https://apiradar.live/api/leaks".to_string()
        } else {
            format!("https://apiradar.live/api/leaks?provider={}&limit=10", provider)
        };

        let resp = self.client.get(&url).send().await?;
        if resp.status().is_success() {
            let data: LeaksResponse = resp.json().await?;
            Ok(data.leaks)
        } else {
            Err(anyhow!("Failed to fetch leaks: {}", resp.status()))
        }
    }

    pub async fn fetch_raw_content(&self, repo_path: &str, file_path: &str) -> Option<String> {
        let clean_repo = repo_path.replace("https://github.com/", "");
        for branch in &self.branches {
            let url = format!("https://raw.githubusercontent.com/{}/{}/{}", clean_repo, branch, file_path);
            if let Ok(resp) = self.client.get(&url).send().await {
                if resp.status().is_success() {
                    if let Ok(content) = resp.text().await {
                        return Some(content);
                    }
                }
            }
        }
        None
    }

    pub fn is_relevant_file(&self, file_path: &str) -> bool {
        let path = Path::new(file_path);
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let lower_ext = ext.to_lowercase();
            return ALLOWED_EXTENSIONS.iter().any(|&e| e == lower_ext);
        }
        false
    }

    pub fn contains_key_indicator(&self, content: &str) -> bool {
        let lower_content = content.to_lowercase();
        KEYNAME_INDICATORS.iter().any(|&indicator| lower_content.contains(indicator))
    }

    pub fn extract_keys(&self, content: &str) -> Vec<(String, String)> {
        // Evaluate all regexes directly without premature keyword optimization

        let patterns: Vec<(&str, &str, Vec<&str>, bool)> = vec![
            // (type, regex, context_tags, requires_tag_match)
            // Keys with unique prefixes don't need tag matching — the prefix IS the identifier
            ("openai_key", r"sk-[a-zA-Z0-9]{20,}", vec!["openai", "gpt", "api"], false),
            ("anthropic_api_key", r"sk-ant-api03-[a-zA-Z0-9\-_]{80,}", vec!["anthropic", "claude"], false),
            ("google_api_key", r"AIza[0-9A-Za-z\-_]{35}", vec!["google", "gemini", "firebase", "api"], false),
            ("openrouter_key", r"sk-or-v1-[a-fA-F0-9]{64}", vec!["openrouter", "router"], false),
            ("groq_key", r"gsk_[a-zA-Z0-9]{48,}", vec!["groq"], false),
            ("perplexity_key", r"pplx-[a-zA-Z0-9]{44,}", vec!["perplexity"], false),
            ("github_token", r"gh[pousr]_[a-zA-Z0-9]{36,}", vec!["github", "oauth"], false),
            ("xai_key", r"xai-[a-zA-Z0-9]{48,}", vec!["xai", "grok"], false),
            // Mistral keys don't have a unique prefix — require tag context
            ("mistral_api_key", r"[a-zA-Z0-9]{32}", vec!["mistral", "pixtral", "codestral"], true),
        ];

        let mut results = Vec::new();
        let lower_content = content.to_lowercase();

        for (key_type, pattern, tags, requires_tag) in patterns {
            // Skip tag-required patterns if no relevant tags found in content
            if requires_tag {
                let has_tag = tags.iter().any(|t| lower_content.contains(&t.to_lowercase()));
                if !has_tag { continue; }
            }

            if let Ok(re) = Regex::new(pattern) {
                for cap in re.captures_iter(content) {
                    let key_value = cap[0].to_string();
                    // Deduplicate
                    if !results.iter().any(|(_, v): &(String, String)| v == &key_value) {
                        results.push((key_type.to_string(), key_value));
                    }
                }
            }
        }
        results
    }

    pub async fn validate_key(&self, key_type: &str, key: &str) -> (bool, String) {
        match key_type {
            "openai_key" | "openrouter_key" | "xai_key" | "groq_key" | "cerebras_key" | "mistral_api_key" => {
                let url = match key_type {
                    "openrouter_key" => "https://openrouter.ai/api/v1/models",
                    "xai_key" => "https://api.x.ai/v1/models",
                    "groq_key" => "https://api.groq.com/openai/v1/models",
                    "cerebras_key" => "https://api.cerebras.ai/v1/models",
                    "mistral_api_key" => "https://api.mistral.ai/v1/models",
                    _ => "https://api.openai.com/v1/models",
                };
                let resp = self.client.get(url)
                    .header("Authorization", format!("Bearer {}", key))
                    .send().await;
                match resp {
                    Ok(r) if r.status().is_success() => (true, "Key is live (models list success)".to_string()),
                    Ok(r) => (false, format!("Failed: {}", r.status())),
                    Err(e) => (false, format!("Error: {}", e)),
                }
            },
            "anthropic_api_key" => {
                let resp = self.client.get("https://api.anthropic.com/v1/models")
                    .header("x-api-key", key)
                    .header("anthropic-version", "2023-06-01")
                    .send().await;
                match resp {
                    Ok(r) if r.status().is_success() => (true, "Key is live".to_string()),
                    Ok(r) => (false, format!("Failed: {}", r.status())),
                    Err(e) => (false, format!("Error: {}", e)),
                }
            },
            "gemini_api_key" | "google_api_key" => {
                let url = format!("https://generativelanguage.googleapis.com/v1/models?key={}", key);
                let resp = self.client.get(&url).send().await;
                match resp {
                    Ok(r) if r.status().is_success() => (true, "Key is live (v1 success)".to_string()),
                    Ok(r) => (false, format!("Failed: {} at v1", r.status())),
                    Err(e) => (false, format!("Error: {}", e)),
                }
            },
            _ => (false, "Unknown key type".to_string()),
        }
    }
}
