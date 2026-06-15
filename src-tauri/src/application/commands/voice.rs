use tauri::State;
use crate::EditorState;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ElevenLabsVoice {
    pub voice_id: String,
    pub name: String,
    pub labels: Option<serde_json::Value>,
    pub preview_url: Option<String>,
    pub category: Option<String>,
    pub gender: Option<String>,
    pub age: Option<String>,
    pub accent: Option<String>,
}

#[derive(Deserialize, Default)]
struct ApiKeys {
    elevenlabs_api_key: Option<String>,
}

#[tauri::command]
pub async fn elevenlabs_get_voices(state: State<'_, EditorState>) -> Result<Vec<ElevenLabsVoice>, String> {
    let path = state.config_dir.join("api_keys.json");
    let api_key: String = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let keys: ApiKeys = serde_json::from_str(&content).unwrap_or_default();
        keys.elevenlabs_api_key.unwrap_or_default()
    } else {
        String::new()
    };

    if api_key.is_empty() {
        return Err("ElevenLabs API key not configured".to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.elevenlabs.io/v1/voices")
        .header("xi-api-key", &api_key)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !response.status().is_success() {
        let err = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}", err));
    }

    let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    let voices_val = data.get("voices").ok_or("No 'voices' field in response")?;
    let voices: Vec<ElevenLabsVoice> = serde_json::from_value(voices_val.clone()).map_err(|e| e.to_string())?;
    
    Ok(voices)
}
