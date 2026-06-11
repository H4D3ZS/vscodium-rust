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
    state.ext.context_keys.set(key, value);
}

#[tauri::command]
pub fn evaluate_when_clause(state: State<'_, EditorState>, clause: String) -> bool {
    state.ext.context_keys.evaluate(&clause)
}

#[tauri::command]
pub async fn set_active_root(
    state: State<'_, EditorState>,
    path: Option<String>,
) -> Result<Option<String>, String> {
    let mut root = state.editor.active_root.lock().await;
    match path {
        None => {
            *root = None;
            Ok(None)
        }
        Some(raw) => {
            // Strip embedded NUL bytes — a corrupted localStorage entry once
            // smuggled `"…manus_source_code\0"` in here, which then fed
            // straight into CreateProcessW and broke every new terminal.
            let cleaned: String = raw.split('\0').next().unwrap_or("").trim().to_string();
            if cleaned.is_empty() {
                *root = None;
                return Ok(None);
            }
            let path_buf = PathBuf::from(&cleaned);
            if !path_buf.is_dir() {
                // Stale activeRoot from a previous session — drop it so the
                // frontend can fall back to the backend's cwd instead of
                // sending CreateProcessW into a non-existent folder.
                *root = None;
                return Err(format!("active root does not exist: {}", cleaned));
            }
            *root = Some(path_buf.clone());
            state.ai.engine.set_root_path(path_buf.clone());
            if let Err(e) = state.memory.vector_indexer.set_workspace(path_buf.clone()).await {
                eprintln!("[set_active_root] vector indexer rebind failed: {e}");
            }
            let root_for_ctx = path_buf.clone();
            let ctx = state.memory.context_indexer.clone();
            tauri::async_runtime::spawn(async move {
                let _ = ctx.trigger_index_cycle().await;
                let _ = ctx.reindex_if_needed(&root_for_ctx);
            });
            Ok(Some(cleaned))
        }
    }
}

#[tauri::command]
pub async fn get_active_root(state: State<'_, EditorState>) -> Result<Option<String>, String> {
    let root = state.editor.active_root.lock().await;
    Ok(root.as_ref().map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
pub fn path_exists(path: String) -> bool {
    let cleaned: String = path.split('\0').next().unwrap_or("").trim().to_string();
    if cleaned.is_empty() {
        return false;
    }
    std::path::Path::new(&cleaned).exists()
}

#[tauri::command]
pub async fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Result<Option<String>, String> {
    let kb = state.ext.keybindings.lock().await;
    Ok(kb.resolve_key(&key, &state.ext.context_keys))
}

/// Snapshot every registered keybinding. Powers the Keybindings panel in
/// the settings UI; sorting / filtering happens client-side.
#[tauri::command]
pub async fn list_keybindings(state: State<'_, EditorState>) -> Result<Vec<crate::keybindings::Keybinding>, String> {
    let kb = state.ext.keybindings.lock().await;
    Ok(kb.list())
}

/// Add or replace a binding. Pass an empty `key` to delete an existing
/// (command, when) tuple. The when-clause is optional and matches the
/// VS Code grammar.
#[tauri::command]
pub async fn update_keybinding(
    state: State<'_, EditorState>,
    key: String,
    command: String,
    when: Option<String>,
) -> Result<(), String> {
    let mut kb = state.ext.keybindings.lock().await;
    kb.upsert(key, command, when);
    Ok(())
}
