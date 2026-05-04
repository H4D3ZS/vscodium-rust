use crate::EditorState;
use tauri::State;
use serde_json::{Value, json};

#[tauri::command]
pub async fn vector_index_codebase(state: State<'_, EditorState>) -> Result<String, String> {
    state.vector_indexer
        .index_codebase()
        .await
        .map(|_| "Codebase indexing started".to_string())
        .map_err(|e| format!("Failed to start indexing: {}", e))
}

#[tauri::command]
pub async fn vector_search_codebase(
    state: State<'_, EditorState>,
    query: String,
    limit: Option<usize>,
) -> Result<Value, String> {
    let limit = limit.unwrap_or(10);
    let results = state.vector_indexer
        .search_codebase(&query, limit)
        .await
        .map_err(|e| format!("Search failed: {}", e))?;

    Ok(json!({
        "query": query,
        "results": results,
        "count": results.len()
    }))
}

#[tauri::command]
pub async fn vector_find_symbol(
    state: State<'_, EditorState>,
    symbol_name: String,
) -> Result<Value, String> {
    let results = state.vector_indexer
        .find_symbol(&symbol_name)
        .await
        .map_err(|e| format!("Symbol search failed: {}", e))?;

    Ok(json!({
        "symbol": symbol_name,
        "results": results,
        "count": results.len()
    }))
}

#[tauri::command]
pub async fn vector_get_index_stats(state: State<'_, EditorState>) -> Result<Value, String> {
    let stats = state.vector_indexer
        .get_index_stats()
        .await
        .map_err(|e| format!("Failed to get stats: {}", e))?;

    Ok(json!(stats))
}

#[tauri::command]
pub async fn vector_get_file_chunks(
    state: State<'_, EditorState>,
    file_path: String,
) -> Result<Value, String> {
    let chunks = state.vector_indexer
        .get_file_chunks(&file_path)
        .await
        .map_err(|e| format!("Failed to get chunks: {}", e))?;

    Ok(json!({
        "file": file_path,
        "chunks": chunks,
        "count": chunks.len()
    }))
}
