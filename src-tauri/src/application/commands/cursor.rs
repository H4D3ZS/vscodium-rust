use crate::cursor_compat::{
    append_debug_log, create_worktree, ensure_scaffold, list_worktrees, scan_project,
    CursorWorktreeInfo,
};
use crate::EditorState;
use serde_json::Value;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn cursor_scan_project(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = resolve_root(&state, root).await?;
    scan_project(&root_path)
        .map(|s| serde_json::to_value(s).unwrap_or(Value::Null))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cursor_init_project(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = resolve_root(&state, root).await?;
    ensure_scaffold(&root_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cursor_append_debug_log(
    state: State<'_, EditorState>,
    event: Value,
    root: Option<String>,
) -> Result<(), String> {
    let root_path = resolve_root(&state, root).await?;
    append_debug_log(&root_path, event).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cursor_list_worktrees(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Vec<CursorWorktreeInfo>, String> {
    let root_path = resolve_root(&state, root).await?;
    Ok(list_worktrees(&root_path))
}

#[tauri::command]
pub async fn cursor_create_worktree(
    state: State<'_, EditorState>,
    branch: String,
    name: Option<String>,
    root: Option<String>,
) -> Result<CursorWorktreeInfo, String> {
    let root_path = resolve_root(&state, root).await?;
    create_worktree(&root_path, &branch, name.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cursor_reload_workspace(
    state: State<'_, EditorState>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = resolve_root(&state, root).await?;
    state
        .ai.engine
        .reload_cursor_workspace(&root_path)
        .await
        .map_err(|e| e.to_string())
}

async fn resolve_root(state: &State<'_, EditorState>, root: Option<String>) -> Result<PathBuf, String> {
    if let Some(r) = root.filter(|s| !s.trim().is_empty()) {
        return Ok(PathBuf::from(r));
    }
    state
        .editor.active_root
        .lock()
        .await
        .clone()
        .ok_or_else(|| "No active workspace root".to_string())
}
