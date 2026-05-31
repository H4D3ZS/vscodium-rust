//! KV-Cache Injection Module
//!
//! Provides C++/HIP logic for injecting inflated code blocks into the
//! llama.cpp KV-cache mid-inference using neural pointers.
//!
//! ## Architecture
//!
//! ```text
//! Inflated Block (VRAM Scratchpad)
//!         │
//!         ▼
//! ┌───────────────────┐
//! │ Neural Pointer    │ → Links to 6KB gist centroid
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │ HIP Kernel        │ → Parallel token embedding
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │ KV-Cache Insert   │ → Mid-inference injection
//! └───────────────────┘
//! ```

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::ffi::c_void;
use tracing::{debug, info};

use super::semantic_map::ParametricGist;

/// Neural pointer - maintains continuity between gist and inflated block
#[derive(Debug, Clone)]
pub struct NeuralPointer {
    /// Pointer to inflated block data in VRAM
    pub block_ptr: u64,  // VRAM address
    /// Pointer to gist centroid
    pub gist_centroid: [f32; 1536],
    /// Cluster ID
    pub cluster_id: u32,
    /// Byte offset in scratchpad
    pub offset: usize,
    /// Block size in bytes
    pub size: usize,
    /// ML-DSA seal hash
    pub seal_hash: [u8; 32],
    /// Creation timestamp
    pub created_at: Instant,
}

impl NeuralPointer {
    /// Create a new neural pointer
    pub fn new(
        block_ptr: u64,
        gist: &ParametricGist,
        cluster_id: u32,
        offset: usize,
        size: usize,
    ) -> Self {
        Self {
            block_ptr,
            gist_centroid: *gist.as_slice().try_into().unwrap_or(&[0.0f32; 1536]),
            cluster_id,
            offset,
            size,
            seal_hash: [0u8; 32],
            created_at: Instant::now(),
        }
    }

    /// Set ML-DSA seal hash
    pub fn set_seal(&mut self, hash: [u8; 32]) {
        self.seal_hash = hash;
    }

    /// Verify pointer validity
    pub fn is_valid(&self) -> bool {
        self.block_ptr != 0 && self.size > 0
    }
}

/// Injected block in KV-cache
#[derive(Debug)]
pub struct InjectedBlock {
    /// Neural pointer
    pub pointer: NeuralPointer,
    /// KV-cache slot indices
    pub kv_slots: Vec<u32>,
    /// Number of tokens injected
    pub token_count: u32,
    /// Injection timestamp
    pub injected_at: Instant,
    /// Access count (for LRU)
    pub access_count: AtomicU64,
    /// Last access timestamp
    pub last_access: Instant,
}

impl InjectedBlock {
    /// Create a new injected block
    pub fn new(pointer: NeuralPointer, kv_slots: Vec<u32>, token_count: u32) -> Self {
        Self {
            pointer,
            kv_slots,
            token_count,
            injected_at: Instant::now(),
            access_count: AtomicU64::new(1),
            last_access: Instant::now(),
        }
    }

    /// Record access
    pub fn record_access(&mut self) {
        self.access_count.fetch_add(1, Ordering::SeqCst);
        self.last_access = Instant::now();
    }

    /// Get access count
    pub fn access_count(&self) -> u64 {
        self.access_count.load(Ordering::SeqCst)
    }
}

/// KV-Cache injector for HIP integration
pub struct KVCacheInjector {
    /// Injected blocks
    blocks: Arc<tokio::sync::RwLock<Vec<InjectedBlock>>>,
    /// Current KV-cache position
    kv_position: AtomicUsize,
    /// Maximum KV-cache size
    max_kv_size: usize,
    /// Total injections
    total_injections: AtomicU64,
    /// Total tokens injected
    total_tokens_injected: AtomicU64,
    /// Enabled flag
    enabled: AtomicBool,
}

// Safety: KVCacheInjector can be shared across threads
unsafe impl Send for KVCacheInjector {}
unsafe impl Sync for KVCacheInjector {}

impl KVCacheInjector {
    /// Create a new KV-cache injector
    pub fn new(max_kv_size: usize) -> Self {
        Self {
            blocks: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            kv_position: AtomicUsize::new(0),
            max_kv_size,
            total_injections: AtomicU64::new(0),
            total_tokens_injected: AtomicU64::new(0),
            enabled: AtomicBool::new(true),
        }
    }

    /// Inject an inflated block into KV-cache
    pub async fn inject(
        &self,
        pointer: NeuralPointer,
        token_count: u32,
    ) -> Result<(), KVCacheError> {
        if !self.enabled.load(Ordering::SeqCst) {
            return Err(KVCacheError::InjectorDisabled);
        }

        let current_pos = self.kv_position.load(Ordering::SeqCst);
        
        // Check if we have space
        if current_pos + token_count as usize > self.max_kv_size {
            return Err(KVCacheError::CacheFull);
        }

        // Allocate KV slots
        let kv_slots: Vec<u32> = (current_pos as u32..(current_pos + token_count as usize) as u32).collect();
        let kv_start = kv_slots.first().copied().unwrap_or(0);
        let kv_end = kv_slots.last().copied().unwrap_or(0);
        let cluster_id = pointer.cluster_id;

        // Create injected block
        let block = InjectedBlock::new(pointer, kv_slots, token_count);

        // Add to blocks list
        {
            let mut blocks = self.blocks.write().await;
            blocks.push(block);
        }

        // Update position
        self.kv_position.fetch_add(token_count as usize, Ordering::SeqCst);
        self.total_injections.fetch_add(1, Ordering::SeqCst);
        self.total_tokens_injected.fetch_add(token_count as u64, Ordering::SeqCst);

        info!(
            "Injected block: cluster {}, {} tokens, KV slots {}-{}",
            cluster_id,
            token_count,
            kv_start,
            kv_end
        );

        Ok(())
    }

    /// Get all injected blocks
    pub async fn get_blocks(&self) -> Vec<InjectedBlock> {
        let blocks = self.blocks.read().await;
        blocks.iter().map(|b| {
            InjectedBlock {
                pointer: b.pointer.clone(),
                kv_slots: b.kv_slots.clone(),
                token_count: b.token_count,
                injected_at: b.injected_at,
                access_count: AtomicU64::new(b.access_count()),
                last_access: b.last_access,
            }
        }).collect()
    }

    /// Remove a block by cluster ID
    pub async fn remove_block(&self, cluster_id: u32) -> Option<InjectedBlock> {
        let mut blocks = self.blocks.write().await;
        
        if let Some(pos) = blocks.iter().position(|b| b.pointer.cluster_id == cluster_id) {
            let block = blocks.remove(pos);
            
            // Free KV slots (simplified - production needs proper slot management)
            self.kv_position.fetch_sub(block.token_count as usize, Ordering::SeqCst);
            
            debug!("Removed block for cluster {}", cluster_id);
            return Some(block);
        }
        
        None
    }

    /// Get current KV position
    pub fn kv_position(&self) -> usize {
        self.kv_position.load(Ordering::SeqCst)
    }

    /// Get total injections
    pub fn total_injections(&self) -> u64 {
        self.total_injections.load(Ordering::SeqCst)
    }

    /// Get total tokens injected
    pub fn total_tokens_injected(&self) -> u64 {
        self.total_tokens_injected.load(Ordering::SeqCst)
    }

    /// Reset injector
    pub async fn reset(&self) {
        let mut blocks = self.blocks.write().await;
        blocks.clear();
        self.kv_position.store(0, Ordering::SeqCst);
    }

    /// Enable/disable injector
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }
}

/// KV-cache injection error types
#[derive(Debug)]
pub enum KVCacheError {
    /// Injector disabled
    InjectorDisabled,
    /// KV-cache full
    CacheFull,
    /// Invalid neural pointer
    InvalidPointer,
    /// HIP kernel error
    HipError(String),
}

impl std::fmt::Display for KVCacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KVCacheError::InjectorDisabled => write!(f, "KV-cache injector disabled"),
            KVCacheError::CacheFull => write!(f, "KV-cache full"),
            KVCacheError::InvalidPointer => write!(f, "Invalid neural pointer"),
            KVCacheError::HipError(e) => write!(f, "HIP error: {}", e),
        }
    }
}

impl std::error::Error for KVCacheError {}

/// C-compatible FFI for HIP integration
#[no_mangle]
pub extern "C" fn hades_kv_cache_inject(
    injector_ptr: *mut c_void,
    block_ptr: u64,
    gist_ptr: *const f32,
    cluster_id: u32,
    offset: usize,
    size: usize,
    token_count: u32,
) -> *mut c_void {
    if injector_ptr.is_null() || gist_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let injector = unsafe { &*(injector_ptr as *const KVCacheInjector) };
    
    // Create neural pointer
    let gist_slice = unsafe { std::slice::from_raw_parts(gist_ptr, 1536) };
    let mut gist = ParametricGist::new();
    for (i, &val) in gist_slice.iter().enumerate() {
        gist.as_mut_slice()[i] = val;
    }
    
    let pointer = NeuralPointer::new(block_ptr, &gist, cluster_id, offset, size);
    
    // Inject (blocking call for FFI)
    let runtime = tokio::runtime::Handle::current();
    match runtime.block_on(injector.inject(pointer, token_count)) {
        Ok(block) => Box::into_raw(Box::new(block)) as *mut c_void,
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kv_cache_injection() {
        let injector = KVCacheInjector::new(4096);
        
        let gist = ParametricGist::new();
        let pointer = NeuralPointer::new(0x1000, &gist, 0, 0, 1024);
        
        let block = injector.inject(pointer, 100).await.unwrap();
        
        assert_eq!(block.token_count, 100);
        assert_eq!(injector.kv_position(), 100);
        assert_eq!(injector.total_injections(), 1);
    }

    #[tokio::test]
    async fn test_neural_pointer_creation() {
        let gist = ParametricGist::new();
        let pointer = NeuralPointer::new(0x1000, &gist, 42, 512, 2048);
        
        assert_eq!(pointer.cluster_id, 42);
        assert_eq!(pointer.offset, 512);
        assert_eq!(pointer.size, 2048);
        assert!(pointer.is_valid());
    }
}
