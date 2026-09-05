use tauri::State;
use crate::visual_lab;
use crate::memory_store::SemanticSlot;
use serde_json::Value;

#[tauri::command]
pub async fn get_visual_graph(data: Value, format: String) -> Result<visual_lab::VisualGraph, String> {
    match format.as_str() {
        "json" => Ok(visual_lab::parse_json_to_graph(data)),
        "sql" => {
            let sql = data.as_str().unwrap_or("");
            Ok(visual_lab::parse_sql_to_graph(sql))
        }
        "mongodb" => {
            let mongo = data.as_str().unwrap_or("");
            Ok(visual_lab::parse_mongo_to_graph(mongo))
        }
        _ => Err("Unsupported format".to_string()),
    }
}

#[tauri::command]
pub async fn get_neural_omni_graph(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<visual_lab::VisualGraph, String> {
    state.ai.engine.memory_store.generate_knowledge_graph().await
        .map_err(|e: anyhow::Error| e.to_string())
}


#[tauri::command]
pub async fn get_all_memory_slots(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Vec<SemanticSlot>, String> {
    Ok(state.ai.engine.memory_store.get_all_slots().await)

}

#[tauri::command]
pub async fn generate_visual_graph(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    data: Value,
    format: String,
) -> Result<visual_lab::VisualGraph, String> {
    get_visual_graph(data, format).await
}
