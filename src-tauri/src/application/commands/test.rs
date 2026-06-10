//! Test discovery/runner — thin Tauri adapters.

use crate::EditorState;
use serde_json::json;
use tauri::State;

#[tauri::command]
pub async fn test_sniff_framework(
    state: State<'_, EditorState>,
    root: String,
) -> Result<serde_json::Value, String> {
    let fw = state.test_runner_service.sniff_framework(&root);
    Ok(json!({ "framework": fw }))
}

#[tauri::command]
pub async fn test_discover(
    state: State<'_, EditorState>,
    root: String,
) -> Result<serde_json::Value, String> {
    let cases = state.test_runner_service.discover(&root)?;
    Ok(json!({ "tests": cases }))
}

#[tauri::command]
pub async fn test_run_file(
    state: State<'_, EditorState>,
    root: String,
    path: String,
) -> Result<serde_json::Value, String> {
    let result = state.test_runner_service.run_file(&root, &path)?;
    Ok(json!(result))
}

#[tauri::command]
pub async fn test_run_all(
    state: State<'_, EditorState>,
    root: String,
) -> Result<serde_json::Value, String> {
    let result = state.test_runner_service.run_all(&root)?;
    Ok(json!(result))
}
