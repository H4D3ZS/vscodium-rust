use crate::workspace_compat::{
    delete_kiro_hook, dispatch_hooks, ensure_unified_scaffold, format_steering_for_prompt,
    list_agent_runs, load_kiro_hooks, load_steering_docs, save_agent_run, save_kiro_hook,
    scan_workspace, AgentRunRecord, HookDispatchResult, KiroHook, SteeringDoc,
};
use crate::EditorState;
use serde_json::Value;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn workspace_scan(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = resolve_root(&state, root).await?;
    scan_workspace(&root_path)
        .map(|s| serde_json::to_value(s).unwrap_or(Value::Null))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_init(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = resolve_root(&state, root).await?;
    ensure_unified_scaffold(&root_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_reload(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = resolve_root(&state, root).await?;
    state
        .ai_engine
        .reload_workspace(&root_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_get_steering(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Vec<SteeringDoc>, String> {
    let root_path = resolve_root(&state, root).await?;
    Ok(load_steering_docs(&root_path))
}

#[tauri::command]
pub async fn workspace_steering_prompt(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<String, String> {
    let root_path = resolve_root(&state, root).await?;
    Ok(format_steering_for_prompt(&load_steering_docs(&root_path)))
}

#[tauri::command]
pub async fn workspace_dispatch_hooks(
    state: State<'_, EditorState>,
    event: String,
    file_path: String,
    root: Option<String>,
) -> Result<Vec<HookDispatchResult>, String> {
    let root_path = resolve_root(&state, root).await?;
    dispatch_hooks(&root_path, &event, &file_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_list_hooks(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Vec<KiroHook>, String> {
    let root_path = resolve_root(&state, root).await?;
    load_kiro_hooks(&root_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_delete_hook(
    state: State<'_, EditorState>,
    file_path: String,
    root: Option<String>,
) -> Result<(), String> {
    let root_path = resolve_root(&state, root).await?;
    delete_kiro_hook(&root_path, &file_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_save_hook(
    state: State<'_, EditorState>,
    filename: String,
    hook: KiroHook,
    root: Option<String>,
) -> Result<String, String> {
    let root_path = resolve_root(&state, root).await?;
    save_kiro_hook(&root_path, &filename, &hook)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn workspace_list_agent_runs(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Vec<AgentRunRecord>, String> {
    let root_path = resolve_root(&state, root).await?;
    Ok(list_agent_runs(&root_path))
}

#[tauri::command]
pub async fn workspace_save_agent_run(
    state: State<'_, EditorState>,
    run: AgentRunRecord,
    root: Option<String>,
) -> Result<String, String> {
    let root_path = resolve_root(&state, root).await?;
    save_agent_run(&root_path, &run)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

async fn resolve_root(state: &State<'_, EditorState>, root: Option<String>) -> Result<PathBuf, String> {
    if let Some(r) = root.filter(|s| !s.trim().is_empty()) {
        return Ok(PathBuf::from(r));
    }
    state
        .active_root
        .lock()
        .await
        .clone()
        .ok_or_else(|| "No active workspace root".to_string())
}
