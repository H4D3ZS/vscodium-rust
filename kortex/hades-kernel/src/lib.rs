//! # Hades Kernel - Sovereign Substrate for 8GB VRAM-Constrained Neural Paging
//! 
//! ## Core Principles
//! 
//! 1. **Zero-Copy Substrate**: Data streams directly from NVMe to VRAM via mmap/io_uring
//! 2. **Thermal Back-Pressure**: Autonomous throttling when GPU temp > 72°C or power > 150W
//! 3. **RAII Memory Mastery**: Strict ownership for 8GB environment longevity
//! 4. **Lazy Loading**: Merkle-DAG leaves loaded on-demand via hardware-level triggers
//! 5. **Quantum-Secure Integrity**: ML-DSA/ML-KEM hybrid seals on all .aim drives
//! 
//! ## Architecture
//! 
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    HADES KERNEL SUBSTRATE                    │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Thermal Governor  │  Weight Streamer  │  Lazy Leaf Loader  │
//! │  (AMDGPU/NVAPI)    │  (8GB VRAM Pager) │  (Merkle-DAG mmap) │
//! ├─────────────────────────────────────────────────────────────┤
//! │         JIT DECOMPRESSION ENGINE (Infinite-Fidelity)         │
//! │  - 6KB Semantic Map (Limbic Index)                          │
//! │  - Semantic Fault Handler (≥0.85 activation)                │
//! │  - io_uring Inflation (Zero-copy SSD→VRAM)                  │
//! │  - KV-Cache Injection (HIP kernels)                         │
//! │  - LRU Eviction (6.5GB Hardware Empathy)                    │
//! ├─────────────────────────────────────────────────────────────┤
//! │              Zero-Copy mmap Interface                        │
//! ├─────────────────────────────────────────────────────────────┤
//! │         NVMe Storage ←→ ROCm/HIP ←→ AMD RX 580 8GB          │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod memory;
pub mod thermal;
pub mod paging;
pub mod lazy_dag;
pub mod crypto;
pub mod jit_decompression;

pub use memory::{AimMemoryGuard, MappedTensor, RaiiBuffer};
pub use thermal::{ThermalGovernor, GpuTelemetry, ThermalPolicy};
pub use paging::{WeightStreamer, LayerPage, VramAllocator};
pub use lazy_dag::{MerkleDagLoader, DagLeaf, DagNode};
pub use crypto::{QuantumSeal, IntegrityVerifier};
pub use jit_decompression::{
    JitDecompressionConfig, JitStatus, JitMetrics, JitErrorCode,
    SemanticMap, LimbicIndex, ParametricGist,
    FaultHandler, AttentionCluster, InflationRequest,
    InflationEngine, ScratchpadBuffer, InflationResult,
    KVCacheInjector, NeuralPointer, InjectedBlock,
    LruEvictionQueue, EvictionPolicy, EvictedNode,
};

/// Kernel version following semantic versioning
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Target VRAM budget for AMD RX 580 (8GB total, ~6.5GB usable for weights)
pub const VRAM_BUDGET_BYTES: usize = 6 * 1024 * 1024 * 1024;

/// Thermal throttling thresholds
pub const THERMAL_THROTTLE_TEMP_C: f32 = 72.0;
pub const THERMAL_CRITICAL_TEMP_C: f32 = 80.0;
pub const POWER_THROTTLE_WATTS: f32 = 150.0;

/// io_uring queue depth for async I/O
pub const IO_URING_DEPTH: usize = 256;
