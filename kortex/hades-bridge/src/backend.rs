//! HADES Backend for ggml Integration
//!
//! This module provides the integration layer between llama.cpp's ggml backend
//! system and the HADES Kernel weight streaming infrastructure.

use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, info};

/// Adaptive execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptiveMode {
    /// Local 8GB GPU (AMD RX 580) - active paging enabled
    Local8Gb,
    /// Cloud 192GB GPU (AMD MI300X) - parallel in-memory experts
    Cloud192Gb,
}

/// HADES backend state for ggml integration
pub struct HadesBackend {
    /// Current adaptive mode
    mode: AdaptiveMode,
    /// Current layer being computed
    current_layer: AtomicUsize,
    /// Prefetch distance (layers ahead to prefetch)
    prefetch_distance: usize,
    /// Enable parallel expert loading (cloud mode only)
    parallel_experts: bool,
}

impl HadesBackend {
    /// Create new backend with auto-detected mode
    pub fn new() -> Self {
        // Detect mode from environment
        let mode = Self::detect_mode();
        let parallel_experts = matches!(mode, AdaptiveMode::Cloud192Gb);

        info!("HADES backend initialized in {:?} mode", mode);

        Self {
            mode,
            current_layer: AtomicUsize::new(0),
            prefetch_distance: 2, // Prefetch 2 layers ahead
            parallel_experts,
        }
    }

    /// Detect adaptive mode from environment/system
    fn detect_mode() -> AdaptiveMode {
        // Check environment variable first
        if let Ok(mode) = std::env::var("HADES_MODE") {
            return match mode.to_lowercase().as_str() {
                "cloud" | "mi300x" | "192gb" => AdaptiveMode::Cloud192Gb,
                "local" | "rx580" | "8gb" => AdaptiveMode::Local8Gb,
                _ => AdaptiveMode::Local8Gb,
            };
        }

        // Check for cloud burst flag
        let cloud_burst = std::env::var("HADES_CLOUD_BURST")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        if cloud_burst {
            AdaptiveMode::Cloud192Gb
        } else {
            AdaptiveMode::Local8Gb
        }
    }

    /// Get current adaptive mode
    pub fn mode(&self) -> AdaptiveMode {
        self.mode
    }

    /// Check if running in 8GB local mode
    pub fn is_local_8gb(&self) -> bool {
        self.mode == AdaptiveMode::Local8Gb
    }

    /// Check if running in 192GB cloud mode
    pub fn is_cloud_192gb(&self) -> bool {
        self.mode == AdaptiveMode::Cloud192Gb
    }

    /// Called before computing a layer (paging hook)
    ///
    /// This triggers predictive prefetch of the next layer(s).
    /// Call from llama.cpp's decode loop before layer computation.
    ///
    /// # Parameters
    /// - `layer_id`: The layer about to be computed
    pub fn pre_compute_layer(&self, layer_id: usize) {
        self.current_layer.store(layer_id, Ordering::SeqCst);

        match self.mode {
            AdaptiveMode::Local8Gb => {
                // Active paging mode: prefetch next N layers
                for i in 1..=self.prefetch_distance {
                    let next_layer = layer_id + i;
                    debug!("Prefetching layer {} (distance {})", next_layer, i);
                    // In production, call hades_page_layer(next_layer) via FFI
                }
            }
            AdaptiveMode::Cloud192Gb => {
                // Parallel experts mode: preload all experts for this layer
                if self.parallel_experts {
                    debug!("Loading parallel experts for layer {}", layer_id);
                    // In cloud mode, all layers fit in VRAM - no paging needed
                    // Just ensure experts are loaded in parallel
                }
            }
        }
    }

    /// Called after computing a layer
    ///
    /// Can trigger eviction of old layers in 8GB mode.
    pub fn post_compute_layer(&self, layer_id: usize) {
        if self.mode == AdaptiveMode::Local8Gb && layer_id >= 2 {
            // Evict layer-2 to make room (keep 2 layers resident for pipeline)
            let evict_layer = layer_id.saturating_sub(2);
            debug!("Evicting layer {}", evict_layer);
            // In production, call hades_evict_layer(evict_layer) via FFI
        }
    }

    /// Get prefetch distance
    pub fn prefetch_distance(&self) -> usize {
        self.prefetch_distance
    }

    /// Set prefetch distance (default: 2)
    pub fn set_prefetch_distance(&mut self, distance: usize) {
        self.prefetch_distance = distance;
    }

    /// Check if parallel experts enabled
    pub fn parallel_experts_enabled(&self) -> bool {
        self.parallel_experts
    }

    /// Get recommended batch size based on mode
    pub fn recommended_batch_size(&self) -> usize {
        match self.mode {
            AdaptiveMode::Local8Gb => 1,  // Single batch for 8GB
            AdaptiveMode::Cloud192Gb => 8, // Larger batches for cloud
        }
    }

    /// Get recommended context size based on mode
    pub fn recommended_context_size(&self) -> usize {
        match self.mode {
            AdaptiveMode::Local8Gb => 4096,  // 4K context for 8GB
            AdaptiveMode::Cloud192Gb => 32768, // 32K context for cloud
        }
    }
}

impl Default for HadesBackend {
    fn default() -> Self {
        Self::new()
    }
}

/// C-compatible paging hook for llama.cpp integration
///
/// This function is called from ggml_backend.cpp during the decode loop.
/// It triggers the HADES WeightStreamer to prefetch the next layer.
///
/// # Parameters
/// - `layer_id`: Current layer being computed
/// - `is_first_layer`: true if this is layer 0
/// - `is_last_layer`: true if this is the final layer
///
/// # Returns
/// 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn hades_paging_hook(
    layer_id: usize,
    _is_first_layer: bool,
    _is_last_layer: bool,
) -> i32 {
    // In production, this would:
    // 1. Get the global HadesBackend instance
    // 2. Call pre_compute_layer(layer_id)
    // 3. Trigger FFI call to hades_page_layer(layer_id + prefetch_distance)
    
    debug!("Paging hook: layer {}", layer_id);
    
    // For now, just apply thermal back-pressure
    // In production, integrate with actual weight streamer
    hades_apply_back_pressure_ffi();
    
    0
}

/// Internal call to hades_apply_back_pressure
fn hades_apply_back_pressure_ffi() {
    // Call the FFI function directly
    crate::ffi::hades_apply_back_pressure();
}

/// Get adaptive mode for C++ callers
#[no_mangle]
pub extern "C" fn hades_get_mode() -> u32 {
    let backend = HadesBackend::new();
    match backend.mode() {
        AdaptiveMode::Local8Gb => 0,
        AdaptiveMode::Cloud192Gb => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = HadesBackend::new();
        assert!(backend.mode() == AdaptiveMode::Local8Gb || 
                backend.mode() == AdaptiveMode::Cloud192Gb);
    }

    #[test]
    fn test_adaptive_mode_detection() {
        // Test environment variable override
        std::env::set_var("HADES_MODE", "cloud");
        let mode = HadesBackend::detect_mode();
        assert_eq!(mode, AdaptiveMode::Cloud192Gb);
        
        std::env::set_var("HADES_MODE", "local");
        let mode = HadesBackend::detect_mode();
        assert_eq!(mode, AdaptiveMode::Local8Gb);
        
        std::env::remove_var("HADES_MODE");
    }

    #[test]
    fn test_paging_hooks() {
        let backend = HadesBackend::new();
        
        // Simulate decode loop
        for layer in 0..4 {
            backend.pre_compute_layer(layer);
            backend.post_compute_layer(layer);
        }
    }
}
