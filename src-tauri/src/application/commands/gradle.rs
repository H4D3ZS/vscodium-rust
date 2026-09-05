//! Gradle build system — thin Tauri adapters.

use serde_json::json;
use tauri::State;

#[tauri::command]
pub async fn gradle_detect_project(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
) -> Result<serde_json::Value, String> {
    let project = state.mobile.gradle.detect(&root)?;
    Ok(json!(project))
}

#[tauri::command]
pub async fn gradle_sync_project(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
) -> Result<serde_json::Value, String> {
    let project = state.mobile.gradle.sync(&root)?;
    Ok(json!(project))
}

#[tauri::command]
pub async fn gradle_list_tasks(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
) -> Result<serde_json::Value, String> {
    let tasks = state.mobile.gradle.list_tasks(&root)?;
    Ok(json!({ "tasks": tasks }))
}

#[tauri::command]
pub async fn gradle_run_task(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    root: String,
    task: String,
) -> Result<serde_json::Value, String> {
    let output = state.mobile.gradle.run_task(&root, &task)?;
    Ok(json!({ "ok": true, "output": output }))
}
