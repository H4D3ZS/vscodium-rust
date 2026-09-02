//! API-key persistence (`api_keys.json`).
//!
//! The engine reads keys from `brain_dir.parent()/api_keys.json`, where
//! `brain_dir == config_dir/brain` and `config_dir` is the Tauri
//! `app_config_dir()` held in `EditorState.config_dir` (derived from the bundle
//! identifier). To guarantee every get/save command writes to the SAME file the
//! engine reads, these commands resolve the path through `State<EditorState>`
//! rather than guessing a directory from a hardcoded string. The previous build
//! guessed `vscodium-rust` while the engine used `app_config_dir()`, and any
//! drift between the two silently split keys across two files — keys were
//! saved to one file and read from another, so BYOB providers never appeared in
//! the model picker.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiKeys {
    pub anthropic: Option<String>,
    pub anthropic_base_url: Option<String>,
    pub openai: Option<String>,
    pub openai_base_url: Option<String>,
    pub google: Option<String>,
    pub google_base_url: Option<String>,
    pub groq: Option<String>,
    pub openrouter: Option<String>,
    pub mistral: Option<String>,
    pub deepseek: Option<String>,
    pub mimo: Option<String>,
    pub mimo_base_url: Option<String>,
    pub highwayapi: Option<String>,
    pub highwayapi_base_url: Option<String>,
    pub cyberifrit: Option<String>,
    pub cyberifrit_base_url: Option<String>,
    pub openmodel: Option<String>,
    pub openmodel_base_url: Option<String>,
    pub xai: Option<String>,
    pub cerebras: Option<String>,
    pub alibaba: Option<String>,
    pub nvidia: Option<String>,
    pub huggingface: Option<String>,
    pub huggingface_base_url: Option<String>,
    pub modelscope: Option<String>,
    pub modelscope_base_url: Option<String>,
}

fn api_keys_path(config_dir: &Path) -> PathBuf {
    config_dir.join("api_keys.json")
}

/// Resolve the config dir, preferring the live `EditorState.config_dir` (the
/// exact path the engine reads) so get/save can never drift apart. Falls back
/// to the Tauri-style default only when called outside a managed state
/// (headless/tests).
fn resolve_config_dir(state: Option<&std::sync::Arc<crate::EditorState>>, override_dir: Option<PathBuf>) -> PathBuf {
    if let Some(d) = override_dir {
        return d;
    }
    if let Some(s) = state {
        return s.config_dir.clone();
    }
    // Headless fallback — mirrors `app_config_dir()` for identifier `vscodium-rust`.
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("vscodium-rust")
}

/// Read & parse `api_keys.json`, tolerating a missing file (returns empty keys)
/// and a corrupt file (also empty keys) so the picker never hard-fails.
fn read_keys(config_dir: &Path) -> ApiKeys {
    let path = api_keys_path(config_dir);
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return ApiKeys::default(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

fn write_keys(config_dir: &Path, keys: &ApiKeys) -> Result<(), String> {
    let path = api_keys_path(config_dir);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(keys).map_err(|e| e.to_string())?;
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_api_keys(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    config_dir: Option<PathBuf>,
) -> Result<ApiKeys, String> {
    let dir = resolve_config_dir(Some(&state), config_dir);
    Ok(read_keys(&dir))
}

#[tauri::command]
pub async fn save_api_keys(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    config_dir: Option<PathBuf>,
    keys: ApiKeys,
) -> Result<(), String> {
    let dir = resolve_config_dir(Some(&state), config_dir);
    write_keys(&dir, &keys)
}

/// Persist a single key (`{ key: "xai", value: "..." }`). Used by the Settings
/// panel's per-field inputs. Accepts every field name the frontend sends; an
/// unknown key is an explicit error instead of a silent no-op.
#[tauri::command]
pub async fn save_api_key(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    config_dir: Option<PathBuf>,
    key: String,
    value: String,
) -> Result<(), String> {
    let dir = resolve_config_dir(Some(&state), config_dir);
    println!("[API_KEYS] save_api_key: key={}, dir={}", key, dir.display());
    let mut keys = read_keys(&dir);
    match key.as_str() {
        "anthropic" => keys.anthropic = Some(value),
        "anthropic_base_url" => keys.anthropic_base_url = Some(value),
        "openai" => keys.openai = Some(value),
        "openai_base_url" => keys.openai_base_url = Some(value),
        "google" => keys.google = Some(value),
        "google_base_url" => keys.google_base_url = Some(value),
        "groq" => keys.groq = Some(value),
        "openrouter" => keys.openrouter = Some(value),
        "mistral" => keys.mistral = Some(value),
        "deepseek" => keys.deepseek = Some(value),
        "mimo" => keys.mimo = Some(value),
        "mimo_base_url" => keys.mimo_base_url = Some(value),
        "highwayapi" => keys.highwayapi = Some(value),
        "highwayapi_base_url" => keys.highwayapi_base_url = Some(value),
        "cyberifrit" => keys.cyberifrit = Some(value),
        "cyberifrit_base_url" => keys.cyberifrit_base_url = Some(value),
        "openmodel" => keys.openmodel = Some(value),
        "openmodel_base_url" => keys.openmodel_base_url = Some(value),
        "xai" => keys.xai = Some(value),
        "cerebras" => keys.cerebras = Some(value),
        "alibaba" => keys.alibaba = Some(value),
        "nvidia" => keys.nvidia = Some(value),
        "huggingface" => keys.huggingface = Some(value),
        "huggingface_base_url" => keys.huggingface_base_url = Some(value),
        _ => return Err(format!("Unknown provider: {}", key)),
    }
    write_keys(&dir, &keys)
}

// ── Internal helpers used by other commands (account/terminal/voice) ──────
// These take an explicit dir because they already resolve the path themselves;
// keeping them synchronous avoids an async boundary where none is needed.

pub fn get_api_keys_at(config_dir: &Path) -> ApiKeys {
    read_keys(config_dir)
}

pub fn save_api_keys_at(config_dir: &Path, keys: &ApiKeys) -> Result<(), String> {
    write_keys(config_dir, keys)
}
