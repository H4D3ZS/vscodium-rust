use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeys {
    pub anthropic: Option<String>,
    pub anthropic_base_url: Option<String>,
    pub openai: Option<String>,
    pub openai_base_url: Option<String>,
    pub google: Option<String>,
    pub groq: Option<String>,
    pub openrouter: Option<String>,
    pub mistral: Option<String>,
    pub deepseek: Option<String>,
    pub mimo: Option<String>,
    pub highwayapi: Option<String>,
    pub highwayapi_base_url: Option<String>,
    pub cyberifrit: Option<String>,
    pub cyberifrit_base_url: Option<String>,
}

fn api_keys_path(config_dir: &PathBuf) -> PathBuf {
    config_dir.join("api_keys.json")
}

#[tauri::command]
pub fn get_api_keys(config_dir: PathBuf) -> Result<ApiKeys, String> {
    let path = api_keys_path(&config_dir);
    if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    } else {
        Ok(ApiKeys::default())
    }
}

#[tauri::command]
pub fn save_api_keys(config_dir: PathBuf, keys: ApiKeys) -> Result<(), String> {
    let path = api_keys_path(&config_dir);
    let content = serde_json::to_string_pretty(&keys).map_err(|e| e.to_string())?;
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_api_key(config_dir: PathBuf, provider: String, key: String) -> Result<(), String> {
    let mut keys = get_api_keys(config_dir.clone())?;
    match provider.as_str() {
        "anthropic" => keys.anthropic = Some(key),
        "openai" => keys.openai = Some(key),
        "google" => keys.google = Some(key),
        "groq" => keys.groq = Some(key),
        "openrouter" => keys.openrouter = Some(key),
        "mistral" => keys.mistral = Some(key),
        "deepseek" => keys.deepseek = Some(key),
        "mimo" => keys.mimo = Some(key),
        "highwayapi" => keys.highwayapi = Some(key),
        "cyberifrit" => keys.cyberifrit = Some(key),
        _ => return Err(format!("Unknown provider: {}", provider)),
    }
    save_api_keys(config_dir, keys)
}
