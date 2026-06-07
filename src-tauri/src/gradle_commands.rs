//! Gradle build system — thin Tauri adapters.

use crate::EditorState;
use serde_json::json;
use tauri::State;

#[tauri::command]
pub async fn gradle_detect_project(
    state: State<'_, EditorState>,
    root: String,
) -> Result<serde_json::Value, String> {
    let project = state.gradle_service.detect(&root)?;
    Ok(json!(project))
}

#[tauri::command]
pub async fn gradle_sync_project(
    state: State<'_, EditorState>,
    root: String,
) -> Result<serde_json::Value, String> {
    let project = state.gradle_service.sync(&root)?;
    Ok(json!(project))
}

#[tauri::command]
pub async fn gradle_list_tasks(
    state: State<'_, EditorState>,
    root: String,
) -> Result<serde_json::Value, String> {
    let tasks = state.gradle_service.list_tasks(&root)?;
    Ok(json!({ "tasks": tasks }))
}

#[tauri::command]
pub async fn gradle_run_task(
    state: State<'_, EditorState>,
    root: String,
    task: String,
) -> Result<serde_json::Value, String> {
    let output = state.gradle_service.run_task(&root, &task)?;
    Ok(json!({ "ok": true, "output": output }))
}
