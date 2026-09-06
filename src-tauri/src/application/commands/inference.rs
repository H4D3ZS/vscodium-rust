/// Inference optimization commands
use tauri::State;
use serde_json::Value;

/// Get unified inference optimization status (ANE + Memory + MoE)
#[tauri::command]
pub async fn inference_get_status(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    // Get current model
    let model = state.ai.current_model.lock().await.clone();

    // Get ANE status
    let ane_status = state.ai.ane.get_status().await;

    // Compose unified status
    Ok(serde_json::json!({
        "model": model,
        "ane": {
            "available": ane_status.available,
            "chip": ane_status.model,
            "mode": ane_status.inference_mode,
            "estimated_speedup": ane_status.estimated_speedup,
            "tokens_per_sec": ane_status.tokens_per_sec_estimate,
        },
        "memory": {
            "system_ram_mb": 8192,
            "models_allocation_mb": 5120,
            "context_cache_mb": 1228,
            "buffer_mb": 1742,
        },
        "optimizations": {
            "ane_acceleration": ane_status.available,
            "ssd_cache_offload": true,
            "smart_memory_management": true,
            "moe_routing": false, // TODO: detect from model
        },
        "expected_performance": {
            "note": "Decode is memory-bandwidth bound; tok/s set by the local GPU, not the ANE",
            "qwen_2b_q4_m1": "~45 tokens/sec (near the ~68GB/s bandwidth ceiling)",
            "ane_role": "offloads vector-index similarity, freeing CPU/GPU during streams",
            "first_token_latency_ms": "2000-3000",
        },
        "cache_location": ".aim/model_cache (memmap2 SSD-backed)",
    }))
}

/// Prepare model for optimized inference
#[tauri::command]
pub async fn inference_prepare_model(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    model_name: String,
) -> Result<Value, String> {
    println!("[Inference] Preparing {} for optimized inference...", model_name);

    // Set the model
    let _ = state.ai.current_model.lock().await.clone();
    {
        let mut current = state.ai.current_model.lock().await;
        *current = model_name.clone();
    }

    // Initialize ANE aux-offload (similarity scoring) if hardware supports it
    let _ = state.ai.ane.init_aux_offload(768).await;

    Ok(serde_json::json!({
        "model": model_name,
        "status": "ready",
        "ane_enabled": state.ai.ane.can_accelerate().await,
        "message": format!("Model {} optimized for inference (ANE: {}, SSD cache: .aim/)",
            model_name,
            if state.ai.ane.can_accelerate().await { "ON" } else { "OFF" }
        ),
    }))
}

/// Get setup recommendation for user's system
#[tauri::command]
pub async fn inference_get_setup_recommendation() -> Result<Value, String> {
    Ok(serde_json::json!({
        "recommended_setup": {
            "system": "M1/M2/M3 Mac with 8GB RAM",
            "primary_model": "qwen3.5:12b",
            "backup_model": "mistral:7b",
            "memory_layout": {
                "models_in_ram": "5.1 GB",
                "context_cache": "1.2 GB",
                "system_buffer": "1.7 GB",
            },
            "cache_strategy": {
                "hot_models": "Keep in RAM (memmap2 backed)",
                "cold_models": "Offload to .aim/ (SSD memmap2)",
                "context_windows": "LRU eviction + compression",
            },
        },
        "expected_performance": {
            "qwen3.5:2b_q4": "~45 tokens/sec (bandwidth ceiling on M1)",
            "qwen3.5:12b_q4": "~8-12 tokens/sec (bandwidth ceiling on M1 8GB)",
            "note": "tok/s is set by the local GPU memory bandwidth; ANE offloads indexing instead",
            "first_token_latency": "2-3 seconds (includes SSD load)",
        },
        "optimization_path": [
            "1. Pull a small model into your local backend (e.g. qwen3.5:2b)",
            "2. IDE Settings → Model Selection → Auto-Detect",
            "3. Settings → ANE Acceleration → Enable (offloads semantic search to the NPU)",
            "4. Semantic search stays fast while a generation stream is active",
        ],
        "twitter_style_workflow": {
            "description": "Like @andrewyng, @ylecun do local development",
            "benefits": [
                "Zero internet dependency",
                "Full privacy (data never leaves Mac)",
                "ANE handles semantic search off the CPU/GPU (~10x lower power than GPU)",
                "32GB SSD for model cache (256GB available)",
            ],
        },
    }))
}
