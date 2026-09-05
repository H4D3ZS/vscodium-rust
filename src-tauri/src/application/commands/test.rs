//! Test discovery/runner — thin Tauri adapters.

use serde_json::json;
use tauri::State;

#[tauri::command]
pub async fn test_sniff_framework(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
) -> Result<serde_json::Value, String> {
    let fw = state.services.test_runner.sniff_framework(&root);
    Ok(json!({ "framework": fw }))
}

#[tauri::command]
pub async fn test_discover(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
) -> Result<serde_json::Value, String> {
    let cases = state.services.test_runner.discover(&root)?;
    Ok(json!({ "tests": cases }))
}

#[tauri::command]
pub async fn test_run_file(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
    path: String,
) -> Result<serde_json::Value, String> {
    let result = state.services.test_runner.run_file(&root, &path)?;
    Ok(json!(result))
}

#[tauri::command]
pub async fn test_run_all(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
) -> Result<serde_json::Value, String> {
    let result = state.services.test_runner.run_all(&root)?;
    Ok(json!(result))
}
