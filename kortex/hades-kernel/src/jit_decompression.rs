//! Infinite-Fidelity JIT Decompression Engine
//!
//! This module implements the semantic-gated inflation system that enables
//! 8GB VRAM to handle arbitrarily large codebases through:
//!
//! 1. **6KB Semantic Map (Limbic Index)**: Persistent parametric gist in VRAM
//! 2. **Semantic Fault Handler**: Monitors attention activations ≥0.85
//! 3. **JIT Inflation via io_uring**: Zero-copy SSD→VRAM streaming
//! 4. **KV-Cache Injection**: Mid-inference raw code injection
//! 5. **LRU Eviction**: Hardware empathy at 6.5GB cap
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    INFERENCE ENGINE                          │
//! │  Attention Heads → [Activation Scores] → Fault Handler      │
//! └────────────────────────────┬────────────────────────────────┘
//!                              │ Activation ≥ 0.85
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │              SEMANTIC FAULT HANDLER                          │
//! │  - Cluster ID extraction                                    │
//! │  - Inflation request queue                                  │
//! │  - Thermal back-pressure check                              │
//! └────────────────────────────┬────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │              JIT INFLATION ENGINE                            │
//! │  ┌─────────────────────────────────────────────────────┐    │
//! │  │  io_uring Ring                                      │    │
//! │  │  - Zero-copy SSD read                               │    │
//! │  │  - Direct to VRAM scratchpad                        │    │
//! │  └─────────────────────────────────────────────────────┘    │
//! └────────────────────────────┬────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │              VRAM SCRATCHPAD (512MB)                        │
//! │  - Inflated raw code blocks                                 │
//! │  - LRU eviction queue                                       │
//! │  - Neural pointers to 6KB gist                              │
//! └────────────────────────────┬────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │              KV-CACHE INJECTION                             │
//! │  - HIP kernels for mid-inference injection                  │
//! │  - Neural pointer continuity                                │
//! │  - ML-DSA sealed blocks                                     │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod semantic_map;
pub mod fault_handler;
pub mod inflation;
pub mod kv_cache;
pub mod lru;

pub use semantic_map::{SemanticMap, LimbicIndex, ParametricGist};
pub use fault_handler::{FaultHandler, AttentionCluster, InflationRequest};
pub use inflation::{InflationEngine, ScratchpadBuffer, InflationResult};
pub use kv_cache::{KVCacheInjector, NeuralPointer, InjectedBlock};
pub use lru::{LruEvictionQueue, EvictionPolicy, EvictedNode};

/// Configuration for the JIT decompression engine
#[derive(Debug, Clone)]
pub struct JitDecompressionConfig {
    /// Size of the parametric gist in VRAM (default: 6KB)
    pub gist_size_bytes: usize,
    /// Size of the VRAM scratchpad for inflated blocks (default: 512MB)
    pub scratchpad_size_bytes: usize,
    /// Maximum VRAM usage before LRU eviction triggers (default: 6.5GB)
    pub vram_empathy_cap_bytes: u64,
    /// Attention activation threshold for fault triggering (default: 0.85)
    pub fault_threshold: f32,
    /// Thermal throttle temperature (default: 72°C)
    pub thermal_throttle_temp_c: f32,
    /// Enable io_uring for zero-copy I/O (Linux only)
    pub use_io_uring: bool,
    /// Number of io_uring entries (default: 256)
    pub io_uring_depth: usize,
}

impl Default for JitDecompressionConfig {
    fn default() -> Self {
        Self {
            gist_size_bytes: 6 * 1024,  // 6KB semantic map
            scratchpad_size_bytes: 512 * 1024 * 1024,  // 512MB scratchpad
            vram_empathy_cap_bytes: 6 * 1024 * 1024 * 1024,  // 6.5GB cap
            fault_threshold: 0.85,
            thermal_throttle_temp_c: 72.0,
            use_io_uring: cfg!(target_os = "linux"),
            io_uring_depth: 256,
        }
    }
}

/// Status of the JIT decompression engine
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JitStatus {
    /// Engine idle, waiting for faults
    Idle,
    /// Processing inflation request
    Inflating { cluster_id: u32 },
    /// Waiting for thermal throttle
    ThermalWait { temp_c: f32 },
    /// LRU eviction in progress
    Evicting { bytes_freed: usize },
    /// Error state
    Error { code: JitErrorCode },
}

/// Error codes for JIT decompression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JitErrorCode {
    /// VRAM scratchpad full, eviction failed
    ScratchpadFull,
    /// io_uring submission failed
    IoUringSubmitFailed,
    /// Thermal emergency (temp > 80°C)
    ThermalEmergency,
    /// KV-cache injection failed
    KVCacheInjectionFailed,
    /// ML-DSA seal verification failed
    SealVerificationFailed,
    /// SSD read timeout
    SsdReadTimeout,
}

/// Metrics for the JIT decompression engine
#[derive(Debug, Clone, Default)]
pub struct JitMetrics {
    /// Total inflation requests processed
    pub total_inflations: u64,
    /// Total bytes inflated from SSD
    pub total_bytes_inflated: u64,
    /// Total LRU evictions
    pub total_evictions: u64,
    /// Bytes freed by eviction
    pub total_bytes_evicted: u64,
    /// Thermal throttle events
    pub thermal_throttle_count: u64,
    /// Average inflation latency (microseconds)
    pub avg_inflation_latency_us: f64,
    /// Current VRAM usage (gist + scratchpad)
    pub current_vram_usage_bytes: u64,
    /// Current scratchpad usage
    pub scratchpad_usage_bytes: u64,
    /// Hit rate (gist-only vs inflated)
    pub gist_hit_rate: f64,
}

impl JitMetrics {
    /// Record an inflation event
    pub fn record_inflation(&mut self, bytes: usize, latency_us: u64) {
        self.total_inflations += 1;
        self.total_bytes_inflated += bytes as u64;
        
        // Exponential moving average for latency
        let alpha = 0.1;
        self.avg_inflation_latency_us = 
            (1.0 - alpha) * self.avg_inflation_latency_us + 
            alpha * latency_us as f64;
    }
    
    /// Record an eviction event
    pub fn record_eviction(&mut self, bytes: usize) {
        self.total_evictions += 1;
        self.total_bytes_evicted += bytes as u64;
    }
    
    /// Record a thermal throttle event
    pub fn record_thermal_throttle(&mut self) {
        self.thermal_throttle_count += 1;
    }
}
