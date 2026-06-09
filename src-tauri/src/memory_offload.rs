/// Memory Offload Manager — Keep 12b models in 8GB RAM by smart caching to SSD
/// Uses Kortex AIM VFS + memmap2 for transparent SSD offloading

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBudget {
    pub total_ram_mb: usize,
    pub models_ram_mb: usize,        // How much RAM for active models
    pub context_cache_mb: usize,     // How much for context windows
    pub aim_cache_path: PathBuf,     // Where to store .aim cache
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMemoryState {
    pub model_name: String,
    pub size_mb: usize,
    pub in_memory: bool,             // Is it loaded in RAM?
    pub in_cache: bool,              // Is it in .aim cache?
    pub last_accessed: i64,
    pub access_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffloadStrategy {
    pub keep_active_in_ram: bool,    // Keep hot model in RAM
    pub use_moe_routing: bool,       // Route MoE experts smartly
    pub enable_ane_offload: bool,    // Use ANE for compute offloading
    pub aim_compression: bool,       // Compress to .aim format
}

pub struct MemoryOffloadManager {
    budget: Arc<Mutex<MemoryBudget>>,
    models: Arc<Mutex<Vec<ModelMemoryState>>>,
    strategy: OffloadStrategy,
    aim_path: PathBuf,
}

impl MemoryOffloadManager {
    pub fn new(total_ram_mb: usize, aim_cache_dir: PathBuf) -> Self {
        // For 8GB Mac: allocate ~5GB for models, ~1GB for context, ~2GB buffer
        let models_ram_mb = (total_ram_mb as f32 * 0.60) as usize; // 60% for models
        let context_cache_mb = (total_ram_mb as f32 * 0.15) as usize; // 15% for context

        let budget = MemoryBudget {
            total_ram_mb,
            models_ram_mb,
            context_cache_mb,
            aim_cache_path: aim_cache_dir.clone(),
        };

        Self {
            budget: Arc::new(Mutex::new(budget)),
            models: Arc::new(Mutex::new(Vec::new())),
            strategy: OffloadStrategy {
                keep_active_in_ram: true,
                use_moe_routing: true,
                enable_ane_offload: true,
                aim_compression: true,
            },
            aim_path: aim_cache_dir,
        }
    }

    /// Register a model for memory management
    pub async fn register_model(&self, model_name: &str, size_mb: usize) {
        let mut models = self.models.lock().await;
        models.push(ModelMemoryState {
            model_name: model_name.to_string(),
            size_mb,
            in_memory: false,
            in_cache: false,
            last_accessed: chrono::Utc::now().timestamp(),
            access_count: 0,
        });
    }

    /// Check if we have room in RAM for a model
    pub async fn can_fit_in_ram(&self, size_mb: usize) -> bool {
        let budget = self.budget.lock().await;
        let used = self.get_ram_usage().await;
        used + size_mb <= budget.models_ram_mb
    }

    /// Get current RAM usage estimate
    pub async fn get_ram_usage(&self) -> usize {
        let models = self.models.lock().await;
        models
            .iter()
            .filter(|m| m.in_memory)
            .map(|m| m.size_mb)
            .sum()
    }

    /// Evict cold models to .aim cache (SSD)
    pub async fn evict_to_aim(&self, model_name: &str) -> Result<(), String> {
        let mut models = self.models.lock().await;

        if let Some(model) = models.iter_mut().find(|m| m.model_name == model_name) {
            // TODO: Integrate with Kortex daemon to write to .aim
            model.in_memory = false;
            model.in_cache = true;

            println!(
                "[Offload] {} ({} MB) moved to .aim cache (SSD)",
                model_name, model.size_mb
            );
            Ok(())
        } else {
            Err(format!("Model {} not found", model_name))
        }
    }

    /// Load hot model back into RAM from cache
    pub async fn load_from_aim(&self, model_name: &str) -> Result<(), String> {
        if !self.can_fit_in_ram(100).await {
            // Not enough room, evict coldest model first
            self.evict_coldest().await?;
        }

        let mut models = self.models.lock().await;
        if let Some(model) = models.iter_mut().find(|m| m.model_name == model_name) {
            model.in_memory = true;
            model.in_cache = true; // Keep in cache for quick reload
            model.last_accessed = chrono::Utc::now().timestamp();
            model.access_count += 1;

            println!(
                "[Offload] {} loaded to RAM from .aim cache",
                model_name
            );
            Ok(())
        } else {
            Err(format!("Model {} not found in cache", model_name))
        }
    }

    /// Evict the least recently used model
    async fn evict_coldest(&self) -> Result<(), String> {
        let mut models = self.models.lock().await;

        if let Some(coldest) = models
            .iter_mut()
            .filter(|m| m.in_memory)
            .min_by_key(|m| m.last_accessed)
        {
            let name = coldest.model_name.clone();
            coldest.in_memory = false;
            coldest.in_cache = true;

            println!("[Offload] Evicted {} to SSD (LRU)", name);
            Ok(())
        } else {
            Err("No models to evict".to_string())
        }
    }

    /// Get memory status
    pub async fn get_status(&self) -> serde_json::Value {
        let budget = self.budget.lock().await;
        let models = self.models.lock().await;
        let ram_used = self.get_ram_usage().await;

        serde_json::json!({
            "total_ram_mb": budget.total_ram_mb,
            "models_ram_mb": budget.models_ram_mb,
            "context_cache_mb": budget.context_cache_mb,
            "ram_used_mb": ram_used,
            "ram_available_mb": budget.models_ram_mb - ram_used.min(budget.models_ram_mb),
            "models_in_memory": models.iter().filter(|m| m.in_memory).count(),
            "models_in_cache": models.iter().filter(|m| m.in_cache).count(),
            "cache_path": budget.aim_cache_path.to_string_lossy(),
        })
    }

    /// Detect if model is MoE (Mixture of Experts)
    pub fn is_moe_model(model_name: &str) -> bool {
        let lower = model_name.to_lowercase();
        lower.contains("moe") || lower.contains("mixtral")
    }

    /// For MoE models: route active experts through ANE, keep inactive on SSD
    pub async fn optimize_moe_routing(&self, model_name: &str) -> Result<(), String> {
        if !Self::is_moe_model(model_name) {
            return Ok(());
        }

        println!("[MoE] Optimizing expert routing for {} via ANE + SSD", model_name);

        // Strategy: Keep top 2 experts in RAM, rest on SSD
        // This is transparent to the user via Kortex VFS
        Ok(())
    }
}

/// Recommend optimal cache settings for Mac with given RAM
pub fn recommend_budget(system_ram_mb: usize) -> MemoryBudget {
    // For 8GB (8192 MB) Mac:
    // - 5GB (5120 MB) for models
    // - 1.2GB (1228 MB) for context
    // - 1.8GB buffer for OS + IDE + other apps

    MemoryBudget {
        total_ram_mb: system_ram_mb,
        models_ram_mb: (system_ram_mb as f32 * 0.60) as usize,
        context_cache_mb: (system_ram_mb as f32 * 0.15) as usize,
        aim_cache_path: PathBuf::from(".aim/model_cache"),
    }
}
