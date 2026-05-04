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
