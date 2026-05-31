//! # HADES Bridge - FFI Layer for llama.cpp Integration
//!
//! This library provides a C-compatible FFI interface between the HADES Kernel
//! Rust substrate and the llama.cpp C++ inference engine.
//!
//! ## Key Features
//!
//! - **Zero-Copy Tensor Exchange**: Safe pointer passing between ggml_tensor and MappedTensor
//! - **ThermalGovernor Hooks**: C-accessible throttle checking
//! - **WeightStreamer Integration**: Paging hooks for llama.cpp decode loop
//! - **Adaptive Infrastructure**: Detects 8GB vs 192GB GPU memory
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────┐      FFI      ┌──────────────────────┐
//! │   llama.cpp (C++)   │◄─────────────►│  HADES Bridge (Rust) │
//! │  - ggml_backend.cpp │               │  - hades-bridge     │
//! │  - decode loop      │               │  - memory.rs        │
//! │  - layer compute    │               │  - thermal.rs       │
//! └─────────────────────┘               │  - paging.rs        │
//!                                       └──────────────────────┘
//!                                                │
//!                                       ┌────────▼──────────┐
//!                                       │  HADES Kernel     │
//!                                       │  - WeightStreamer │
//!                                       │  - ThermalGov     │
//!                                       └───────────────────┘
//! ```

pub mod ffi;
pub mod tensor;
pub mod backend;

pub use ffi::*;
pub use tensor::*;
pub use backend::*;

/// Bridge version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default VRAM threshold for 8GB detection (6.5GB usable)
pub const VRAM_8GB_THRESHOLD: u64 = 6 * 1024 * 1024 * 1024;

/// AMD MI300X VRAM (192GB)
pub const VRAM_MI300X: u64 = 192 * 1024 * 1024 * 1024;
