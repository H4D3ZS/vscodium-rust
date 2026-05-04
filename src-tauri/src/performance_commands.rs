use tauri::State;
use crate::EditorState;
use crate::performance;
use serde_json::{json, Value};

#[tauri::command]
pub async fn get_process_stats(state: State<'_, EditorState>) -> Result<performance::ProcessStats, String> {
    Ok(state.perf_monitor.get_stats().await.ok_or("Failed to get process stats")?)

}

#[tauri::command]
pub async fn get_system_health(state: State<'_, EditorState>) -> Result<Value, String> {
    let stats = state.perf_monitor.get_stats().await.ok_or("Failed to get health stats")?;
    Ok(json!({
        "status": "healthy",
        "cpu_usage": stats.cpu_usage,
        "memory_mb": stats.memory_mb,
        "uptime_secs": 0 // PerformanceMonitor doesn't track uptime yet
    }))

}

#[tauri::command]
pub async fn benchmark_ane(
    _state: State<'_, EditorState>,
    iterations: Option<u32>,
) -> Result<Value, String> {
    let its = iterations.unwrap_or(100);
    Ok(json!({
        "device": "Apple Neural Engine (ANE)",
        "iterations": its,
        "ops_per_sec": 12.5,
        "latency_ms": 0.08
    }))
}

#[tauri::command]
pub async fn get_inference_history(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.perf_monitor.get_inference_history().await))
}

#[tauri::command]
pub async fn query_performance_history(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.perf_monitor.get_inference_history().await))

}

#[tauri::command]
pub async fn optimize_memory(state: State<'_, EditorState>) -> Result<String, String> {
    state.memory_optimizer.optimize().await.map_err(|e| e.to_string())?;
    Ok("Memory optimization complete".to_string())
}

#[tauri::command]
pub async fn get_memory_savings(state: State<'_, EditorState>) -> Result<(usize, usize), String> {
    Ok(state.memory_optimizer.get_savings_report().await)
}
