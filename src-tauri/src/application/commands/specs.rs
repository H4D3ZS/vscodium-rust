use tauri::State;
use crate::{EditorState, specs_db};

#[tauri::command]
pub async fn cmd_specs_create_project(state: State<'_, EditorState>, name: String, specs: String, provider: Option<String>) -> Result<i64, String> {
    state.services.specs_db.create_project(&name, &specs, provider.as_deref()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_delete_project(state: State<'_, EditorState>, id: i64) -> Result<(), String> {
    state.services.specs_db.delete_project(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_clear_history(state: State<'_, EditorState>) -> Result<(), String> {
    state.services.specs_db.clear_history().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_delete_task(state: State<'_, EditorState>, id: i64) -> Result<(), String> {
    state.services.specs_db.delete_work_item(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_get_projects(state: State<'_, EditorState>) -> Result<Vec<specs_db::Project>, String> {
    state.services.specs_db.get_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_generate_layout(state: State<'_, EditorState>, project_id: i64) -> Result<(), String> {
    state.services.specs_db.add_work_item("GenerateLayout", project_id).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_get_project_tasks(state: State<'_, EditorState>, project_id: i64) -> Result<Vec<specs_db::WorkItem>, String> {
    state.services.specs_db.get_work_items_for_project(project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_get_project_by_name(state: State<'_, EditorState>, name: String) -> Result<Option<specs_db::Project>, String> {
    state.services.specs_db.get_projects().map(|projects| {
        projects.into_iter().find(|p| p.name == name)
    }).map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn cmd_specs_get_project_files(state: State<'_, EditorState>, project_id: i64) -> Result<Vec<serde_json::Value>, String> {
    state.services.specs_db.get_project_files(project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cmd_specs_retry_task(
    state: tauri::State<'_, EditorState>,
    task_id: i64,
) -> Result<(), String> {
    state
        .services.specs_db
        .update_work_status(task_id, "Pending", "")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn cmd_specs_set_project_provider(state: State<'_, EditorState>, project_id: i64, provider: String) -> Result<(), String> {
    state.services.specs_db.set_project_provider(project_id, &provider).map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn cmd_specs_get_extended_project_layout(state: State<'_, EditorState>, project_id: i64) -> Result<serde_json::Value, String> {
    state.services.specs_db.get_extended_project_layout(project_id).map_err(|e| e.to_string())
}
