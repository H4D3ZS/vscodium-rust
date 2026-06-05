use tauri::State;
use crate::EditorState;
use crate::mcp_registry;
use serde_json::{json, Value};

#[tauri::command]
pub async fn list_mcp_servers(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.mcp_registry.list_servers().await))
}

#[tauri::command]
pub async fn add_mcp_server(
    state: State<'_, EditorState>,
    name: String,
    config: mcp_registry::McpServerConfig,
) -> Result<(), String> {
    state
        .mcp_registry
        .add_server(name, config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_mcp_server(state: State<'_, EditorState>, name: String) -> Result<(), String> {
    state
        .mcp_registry
        .remove_server(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_mcp_server_enabled(
    state: State<'_, EditorState>,
    name: String,
    enabled: bool,
) -> Result<(), String> {
    state
        .mcp_registry
        .set_server_enabled(&name, enabled)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_mcp_config_path(state: State<'_, EditorState>) -> Result<String, String> {
    Ok(state
        .mcp_registry
        .config_path()
        .to_string_lossy()
        .into_owned())
}

#[tauri::command]
pub async fn list_mcp_tools(state: State<'_, EditorState>) -> Result<Value, String> {
    state
        .mcp_registry
        .list_tools()
        .await
        .map(|tools| json!({ "tools": tools, "count": tools.len() }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_mcp_config(state: State<'_, EditorState>) -> Result<Value, String> {
    let path = state.mcp_registry.config_path().to_string_lossy().into_owned();
    let text = state
        .mcp_registry
        .read_config_text()
        .map_err(|e| e.to_string())?;
    Ok(json!({ "path": path, "text": text }))
}

#[tauri::command]
pub async fn write_mcp_config(
    state: State<'_, EditorState>,
    text: String,
) -> Result<(), String> {
    state
        .mcp_registry
        .write_config_text(&text)
        .await
        .map_err(|e| e.to_string())
}
