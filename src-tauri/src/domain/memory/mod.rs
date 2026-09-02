//! Memory domain: the `.aim` binary Neural Weight-Map format (memmap2), the
//! layered memory store/optimizer, RAM-tier offloading, and context quantization.

pub mod aim_store;
pub mod context_quantizer;
pub mod memory_layer;
pub mod memory_offload;
pub mod memory_optimizer;
pub mod memory_store;
