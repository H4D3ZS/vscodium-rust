use tauri::State;
use crate::EditorState;
use serde_json::{json, Value};
use std::path::PathBuf;

#[tauri::command]
pub async fn mount_project(
    state: State<'_, EditorState>,
    path: String,
) -> Result<(), String> {
    let path_buf = PathBuf::from(path);
    state.memory_layer.mount_workspace(path_buf.clone()).await.map_err(|e| e.to_string())?;
    let mut root = state.active_root.lock().await;
    *root = Some(path_buf);
    Ok(())
}

#[tauri::command]
pub async fn unmount_project(
    state: State<'_, EditorState>,
) -> Result<(), String> {
    state.memory_layer.unmount().await;
    let mut root = state.active_root.lock().await;
    *root = None;
    Ok(())
}

#[tauri::command]
pub async fn get_project_memory(
    state: State<'_, EditorState>,
) -> Result<Value, String> {
    let mem = state.memory_layer.get_all_memory().await.map_err(|e| e.to_string())?;
    Ok(json!(mem))
}

#[tauri::command]
pub async fn clear_project_memory(
    state: State<'_, EditorState>,
) -> Result<(), String> {
    state.memory_layer.clear_memory().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_project_memory(
    state: State<'_, EditorState>,
    key: String,
    value: String,
) -> Result<(), String> {
    state.memory_layer.set_memory(&key, &value).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_project(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Value, String> {
    let results = state.memory_layer.search(&query).await.map_err(|e| e.to_string())?;
    Ok(json!(results))
}

#[tauri::command]
pub async fn query_workspace_memory(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Value, String> {
    let results = state.memory_layer.query_context(&query).await.map_err(|e| e.to_string())?;
    Ok(json!(results))
}

#[tauri::command]
pub async fn get_file_context(
    state: State<'_, EditorState>,
    path: String,
) -> Result<Value, String> {
    let ctx = state.memory_layer.get_file_context(&path).await.map_err(|e| e.to_string())?;
    Ok(json!(ctx))
}
