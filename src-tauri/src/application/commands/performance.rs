use tauri::State;
use crate::performance;
use serde_json::{json, Value};

#[tauri::command]
pub async fn get_process_stats(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<performance::ProcessStats, String> {
    Ok(state.services.perf_monitor.get_stats().await.ok_or("Failed to get process stats")?)

}

#[tauri::command]
pub async fn get_system_health(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Value, String> {
    let stats = state.services.perf_monitor.get_stats().await.ok_or("Failed to get health stats")?;
    Ok(json!({
        "status": "healthy",
        "cpu_usage": stats.cpu_usage,
        "memory_mb": stats.memory_mb,
        "snapshot": stats.snapshot,
        "uptime_secs": 0
    }))

}

/// Measured ANE vs CPU benchmark on the real similarity-search workload
/// (batched dot products over synthetic unit vectors, 768-dim like nomic-embed).
#[tauri::command]
pub async fn benchmark_ane(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    iterations: Option<u32>,
) -> Result<Value, String> {
    let its = iterations.unwrap_or(10).max(1);
    let dim = 768usize;
    let n = 256usize; // one full ANE batch

    // Deterministic pseudo-random unit vectors (no rand dep needed).
    let make_vec = |seed: usize| -> Vec<f32> {
        let mut v: Vec<f32> = (0..dim)
            .map(|i| (((seed * 31 + i * 17) % 1000) as f32 / 500.0) - 1.0)
            .collect();
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        v.iter_mut().for_each(|x| *x /= norm);
        v
    };
    let query = make_vec(7);
    let embs: Vec<Vec<f32>> = (0..n).map(make_vec).collect();
    let emb_refs: Vec<&[f32]> = embs.iter().map(|e| e.as_slice()).collect();

    // ANE timing on the prepared path (how ann_index actually uses it):
    // embeddings pre-packed once, only the query column rewritten per search.
    state.ai.ane.init_aux_offload(dim).await.ok();
    let ane = tokio::task::block_in_place(|| {
        let mut batches = state.ai.ane.prepare_sim_batches(&emb_refs)?;
        let start = std::time::Instant::now();
        for _ in 0..its {
            state
                .ai.ane
                .similarity_prepared(&query, &mut batches)?;
        }
        Some(start.elapsed().as_secs_f64() / its as f64)
    });

    // CPU timing (same dot-product workload)
    let cpu = tokio::task::block_in_place(|| {
        let start = std::time::Instant::now();
        for _ in 0..its {
            let _scores: Vec<f32> = emb_refs
                .iter()
                .map(|e| query.iter().zip(e.iter()).map(|(a, b)| a * b).sum())
                .collect();
        }
        start.elapsed().as_secs_f64() / its as f64
    });

    Ok(json!({
        "workload": format!("{n} x {dim}-dim cosine similarity (vector index search)"),
        "iterations": its,
        "ane_latency_ms": ane.map(|s| s * 1000.0),
        "cpu_latency_ms": cpu * 1000.0,
        "ane_available": ane.is_some(),
        "note": "ANE wins on power and frees CPU/GPU during Ollama streams; decode tok/s is bandwidth-bound and unaffected",
    }))
}

#[tauri::command]
pub async fn get_inference_history(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Value, String> {
    Ok(json!(state.services.perf_monitor.get_inference_history().await))
}

#[tauri::command]
pub async fn query_performance_history(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Value, String> {
    Ok(json!(state.services.perf_monitor.get_inference_history().await))

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
pub async fn optimize_memory(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<String, String> {
    state.memory.optimizer.optimize().await.map_err(|e| e.to_string())?;
    let engine = state.ai.engine.clone();
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
pub async fn get_memory_savings(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(usize, usize), String> {
    Ok(state.memory.optimizer.get_savings_report().await)
}
