use tauri::State;
use serde_json::Value;
use crate::ane_inference::AneStatus;

/// Get ANE (Apple Neural Engine) status and availability
#[tauri::command]
pub async fn ane_get_status(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<AneStatus, String> {
    Ok(state.ai.ane.get_status().await)
}

/// Initialize ANE aux-offload (batched similarity scoring for the vector index).
/// Token generation stays on the local GPU — the ANE cannot reach into the local backend.
#[tauri::command]
pub async fn ane_init_inference(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<(), String> {
    // 768 = nomic-embed-text dimension; kernel recompiles lazily if dims differ.
    state.ai.ane.init_aux_offload(768).await
}

/// Check if ANE can handle current inference workload
#[tauri::command]
pub async fn ane_can_accelerate(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<bool, String> {
    Ok(state.ai.ane.can_accelerate().await)
}

/// Update ANE performance metrics after inference
#[tauri::command]
pub async fn ane_update_metrics(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    tokens_processed: u32,
    elapsed_secs: f32,
) -> Result<(), String> {
    state.ai.ane.update_status(tokens_processed, elapsed_secs).await;
    Ok(())
}

/// Get real-time ANE diagnostics
#[tauri::command]
pub async fn ane_diagnostics(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    let status = state.ai.ane.get_status().await;
    Ok(serde_json::json!({
        "available": status.available,
        "chip": status.model,
        "mode": status.inference_mode,
        "estimated_speedup": status.estimated_speedup,
        "tokens_per_sec": status.tokens_per_sec_estimate,
        "can_accelerate": state.ai.ane.can_accelerate().await,
        "token_generation": "metal_decode", // decode is bandwidth-bound; ANE can't enter the inference process
        "ane_workload": "vector_index_similarity",
    }))
}
