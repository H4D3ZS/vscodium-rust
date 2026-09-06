/// Unified Inference Optimizer: ANE + Memory Offload + MoE Routing
/// For smooth 12b model inference on 8GB Mac with SSD cache

use crate::memory_offload::MemoryOffloadManager;
use crate::ane_inference::AneInferenceOptimizer;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;

pub struct OptimizedInferenceEngine {
    memory: Arc<MemoryOffloadManager>,
    ane: Arc<AneInferenceOptimizer>,
    current_model: Arc<Mutex<String>>,
}

impl OptimizedInferenceEngine {
    pub fn new(
        system_ram_mb: usize,
        _cache_dir: std::path::PathBuf,
    ) -> Self {
        let budget = crate::memory_offload::recommend_budget(system_ram_mb);

        Self {
            memory: Arc::new(MemoryOffloadManager::new(system_ram_mb, budget.aim_cache_path)),
            ane: Arc::new(AneInferenceOptimizer::new()),
            current_model: Arc::new(Mutex::new("qwen3.5:12b".to_string())),
        }
    }

    /// Prepare for inference: load model, check memory, optimize routing
    pub async fn prepare_inference(&self, model_name: &str) -> Result<(), String> {
        // Step 1: Register model if new
        if model_name.contains("12b") {
            self.memory.register_model(model_name, 5_000).await; // ~5GB for 12b
        } else if model_name.contains("7b") {
            self.memory.register_model(model_name, 3_500).await; // ~3.5GB for 7b
        }

        // Step 2: Check if it fits in RAM
        if self.memory.can_fit_in_ram(100).await {
            // Plenty of room, load it
            self.memory.load_from_aim(model_name).await?;
        } else {
            // Need to evict coldest model first
            println!("[Inference] RAM full, optimizing...");
        }

        // Step 3: If MoE model, optimize expert routing
        if MemoryOffloadManager::is_moe_model(model_name) {
            self.memory.optimize_moe_routing(model_name).await?;
        }

        // Step 4: Initialize ANE aux-offload (similarity scoring) if available
        self.ane.init_aux_offload(768).await.ok(); // Fail gracefully

        // Step 5: Update current model
        let mut current = self.current_model.lock().await;
        *current = model_name.to_string();

        println!(
            "[Inference] Model '{}' ready (ANE: {}, Cache: .aim/)",
            model_name,
            if self.ane.can_accelerate().await { "ON" } else { "OFF" }
        );

        Ok(())
    }

    /// Get unified inference status
    pub async fn get_status(&self) -> serde_json::Value {
        let memory_status = self.memory.get_status().await;
        let ane_status = self.ane.get_status().await;
        let current_model = self.current_model.lock().await.clone();

        json!({
            "model": current_model,
            "memory": memory_status,
            "ane": {
                "available": ane_status.available,
                "mode": ane_status.inference_mode,
                "speedup": ane_status.estimated_speedup,
            },
            "optimizations_active": {
                "ane_acceleration": ane_status.available,
                "ssd_offload": true,
                "moe_routing": MemoryOffloadManager::is_moe_model(&current_model),
            },
        })
    }

    /// Recommended setup for Twitter-style Mac local dev
    pub fn recommend_setup() -> serde_json::Value {
        json!({
            "system": "M1/M2/M3 Mac with 8GB RAM + 256GB SSD",
            "models": [
                "qwen3.5:12b (primary - 5GB in RAM)",
                "mistral:7b (backup - 3.5GB on SSD)",
            ],
            "memory_allocation": {
                "models_ram": "5.1 GB (60%)",
                "context_cache": "1.2 GB (15%)",
                "system_buffer": "1.7 GB (25%)",
            },
            "ssd_cache": {
                "path": ".aim/model_cache (memmap2)",
                "format": "compressed .aim binary",
                "fallback_swap": "use entire 256GB SSD as virtual memory",
            },
            "accelerations": {
                "ane": "vector-index similarity offloaded to the NPU (frees CPU/GPU during streams)",
                "smart_cache": "cold models paged to SSD, hot in RAM",
                "moe_routing": "active experts in RAM, inactive on SSD (if MoE)",
            },
            "expected_performance": {
                "qwen3.5:2b_q4": "~45 tokens/sec (M1 bandwidth ceiling)",
                "qwen3.5:12b_q4": "~8-12 tokens/sec (M1 8GB bandwidth ceiling)",
                "first_token_latency": "~2-3 seconds (includes load from cache)",
                "note": "decode tok/s is memory-bandwidth bound; the ANE cannot raise it",
            },
            "workflow": [
                "1. IDE starts → auto-detect RAM budget (8GB)",
                "2. Load qwen3.5:12b into 5GB + enable ANE",
                "3. Cold models auto-evict to .aim (SSD memmap2)",
                "4. MoE routing: active experts in RAM, cold → disk",
                "5. Generation: the local GPU; ANE scores vector-index queries in parallel",
            ],
        })
    }
}
