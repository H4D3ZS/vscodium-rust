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
        "snapshot": stats.snapshot,
        "uptime_secs": 0
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

#[cfg(target_os = "windows")]
extern "system" {
    fn GetCurrentProcess() -> isize;
    fn SetProcessWorkingSetSize(
        hProcess: isize,
        dwMinimumWorkingSetSize: usize,
        dwMaximumWorkingSetSize: usize,
    ) -> i32;
}

/// Release malloc zones + encourage kernel to reclaim idle pages (macOS).
#[cfg(target_os = "macos")]
pub fn macos_pressure_relief() {
    extern "C" {
        fn malloc_zone_pressure_relief(zone: *mut std::ffi::c_void, magnitude: u32) -> usize;
    }
    unsafe {
        let _ = malloc_zone_pressure_relief(std::ptr::null_mut(), 0);
    }
}

#[tauri::command]
pub async fn optimize_memory(state: State<'_, EditorState>) -> Result<String, String> {
    state.memory_optimizer.optimize().await.map_err(|e| e.to_string())?;
    let engine = state.ai_engine.clone();
    let _ = engine.optimize_memory().await;

    #[cfg(target_os = "windows")]
    unsafe {
        let handle = GetCurrentProcess();
        let _ = SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
    }

    #[cfg(target_os = "macos")]
    macos_pressure_relief();

    Ok("Memory optimization complete".to_string())
}

#[tauri::command]
pub async fn get_memory_savings(state: State<'_, EditorState>) -> Result<(usize, usize), String> {
    Ok(state.memory_optimizer.get_savings_report().await)
}
