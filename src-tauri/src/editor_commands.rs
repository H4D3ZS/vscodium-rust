use tauri::State;
use crate::EditorState;
use crate::domain::{Settings};
use crate::context_key::{ContextValue};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use tree_sitter::Parser;
use tree_sitter_rust::LANGUAGE;

#[tauri::command]
pub async fn get_settings(state: State<'_, EditorState>) -> Result<Settings, String> {
    let settings_path = state.config_dir.join("settings.json");
    if settings_path.exists() {
        if let Ok(raw) = fs::read_to_string(&settings_path) {
            if let Ok(settings) = serde_json::from_str(&raw) {
                return Ok(settings);
            }
        }
    }
    Ok(Settings {
        theme: "vscodium-dark".to_string(),
        font_size: 14,
    })
}

#[tauri::command]
pub async fn update_settings(state: State<'_, EditorState>, settings: Settings) -> Result<(), String> {
    let settings_path = state.config_dir.join("settings.json");
    let content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(settings_path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn switch_to_buffer(_state: State<'_, EditorState>, _path: String) -> Result<(), String> {
    // Logic for switching active editor buffer
    Ok(())
}

#[tauri::command]
pub async fn get_highlights(_state: State<'_, EditorState>, code: String) -> Result<Value, String> {
    let mut parser = Parser::new();
    parser.set_language(&LANGUAGE.into()).map_err(|e| e.to_string())?;
    let _tree = parser.parse(&code, None).ok_or("Failed to parse code")?;
    
    // Simplified highlight logic for demo
    Ok(json!([]))
}

#[tauri::command]
pub fn set_context_key(state: State<'_, EditorState>, key: String, value: ContextValue) {
    state.context_keys.set(key, value);
}

#[tauri::command]
pub fn evaluate_when_clause(state: State<'_, EditorState>, clause: String) -> bool {
    state.context_keys.evaluate(&clause)
}

#[tauri::command]
pub async fn set_active_root(state: State<'_, EditorState>, path: Option<String>) -> Result<(), String> {
    let mut root = state.active_root.lock().await;
    if let Some(p) = path {
        let path_buf = PathBuf::from(p);
        *root = Some(path_buf.clone());
        state.ai_engine.set_root_path(path_buf);
    } else {
        *root = None;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_active_root(state: State<'_, EditorState>) -> Result<Option<String>, String> {
    let root = state.active_root.lock().await;
    Ok(root.as_ref().map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Result<Option<String>, String> {
    let kb = state.keybindings.lock().await;
    Ok(kb.resolve_key(&key, &state.context_keys))
}
