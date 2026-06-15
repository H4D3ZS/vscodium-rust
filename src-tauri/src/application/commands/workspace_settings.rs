use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
use tauri::State;
use crate::EditorState;

fn workspace_settings_path(root: &str) -> PathBuf {
    PathBuf::from(root).join(".vscode").join("settings.json")
}

#[tauri::command]
pub async fn get_workspace_settings(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root = match root {
        Some(r) => r,
        None => state.editor.active_root.lock().await.clone()
            .map(|p| p.to_string_lossy().to_string())
            .ok_or("No workspace open")?,
    };
    let path = workspace_settings_path(&root);
    if !path.exists() {
        return Ok(json!({}));
    }
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let sanitised = raw
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
        .replace(",}", "}")
        .replace(",]", "]");
    serde_json::from_str(&sanitised).map_err(|e| format!("Invalid settings.json: {e}"))
}

#[tauri::command]
pub async fn update_workspace_settings(
    state: State<'_, EditorState>,
    settings: Value,
    root: Option<String>,
) -> Result<(), String> {
    let root = match root {
        Some(r) => r,
        None => state.editor.active_root.lock().await.clone()
            .map(|p| p.to_string_lossy().to_string())
            .ok_or("No workspace open")?,
    };
    let dir = PathBuf::from(&root).join(".vscode");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("settings.json");
    let content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}
