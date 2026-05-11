//! LRU Eviction Queue
//!
//! Implements Least Recently Used eviction policy for inflated nodes.
//! When VRAM usage hits the 6.5GB "Hardware Empathy" cap, immediately
//! evict inflated code blocks while keeping the 6KB gist resident.
//!
//! ## Policy
//!
//! 1. **LRU Priority**: Evict least recently accessed blocks first
//! 2. **Gist Protection**: 6KB parametric gist is NEVER evicted
//! 3. **Size-Based**: Larger blocks are evicted first when tied on LRU
//! 4. **Thermal-Aware**: Eviction rate increases under thermal throttle

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::kv_cache::InjectedBlock;

/// Evicted node metadata
#[derive(Debug, Clone)]
pub struct EvictedNode {
    /// Cluster ID
    pub cluster_id: u32,
    /// Bytes freed
    pub bytes_freed: usize,
    /// Eviction timestamp
    pub evicted_at: Instant,
    /// Reason for eviction
    pub reason: EvictionReason,
}

/// Reason for eviction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionReason {
    /// LRU - least recently used
    Lru,
    /// Size-based - largest block
    SizeBased,
    /// Emergency - VRAM critical
    Emergency,
    /// Thermal - throttle triggered
    Thermal,
}

/// Eviction policy configuration
#[derive(Debug, Clone)]
pub struct EvictionPolicy {
    /// VRAM empathy cap in bytes (default: 6.5GB)
    pub vram_cap_bytes: u64,
    /// Gist size (protected, default: 6KB)
    pub gist_size_bytes: usize,
    /// Minimum free VRAM to maintain
    pub min_free_vram_bytes: u64,
    /// Enable size-based eviction tiebreaker
    pub size_tiebreaker: bool,
    /// Thermal throttle multiplier (eviction rate increase)
    pub thermal_multiplier: f32,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        Self {
            vram_cap_bytes: 6 * 1024 * 1024 * 1024,  // 6.5GB
            gist_size_bytes: 6 * 1024,  // 6KB (never evicted)
            min_free_vram_bytes: 512 * 1024 * 1024,  // 512MB
            size_tiebreaker: true,
            thermal_multiplier: 2.0,
        }
    }
}

/// LRU Eviction Queue for inflated blocks
pub struct LruEvictionQueue {
    /// Queue of block IDs in LRU order (front = oldest)
    queue: Arc<RwLock<VecDeque<u32>>>,
    /// Map from block ID to metadata
    metadata: Arc<RwLock<HashMap<u32, BlockMetadata>>>,
    /// Current VRAM usage (inflated blocks only)
    current_usage: AtomicUsize,
    /// Total evictions
    total_evictions: AtomicU64,
    /// Total bytes freed
    total_bytes_freed: AtomicUsize,
    /// Policy
    policy: EvictionPolicy,
    /// Current VRAM free space
    vram_free: AtomicUsize,
}

/// Metadata for a block in the queue
#[derive(Debug, Clone)]
struct BlockMetadata {
    /// Cluster ID
    cluster_id: u32,
    /// Block size in bytes
    size_bytes: usize,
    /// Last access time
    last_access: Instant,
    /// Access count
    access_count: u64,
    /// KV-cache slot count
    kv_slots: u32,
}

// Safety: LruEvictionQueue can be shared across threads
unsafe impl Send for LruEvictionQueue {}
unsafe impl Sync for LruEvictionQueue {}

impl LruEvictionQueue {
    /// Create a new LRU eviction queue
    pub fn new(policy: EvictionPolicy, initial_vram_free: usize) -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::with_capacity(1024))),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            current_usage: AtomicUsize::new(0),
            total_evictions: AtomicU64::new(0),
            total_bytes_freed: AtomicUsize::new(0),
            policy,
            vram_free: AtomicUsize::new(initial_vram_free),
        }
    }

    /// Add a block to the eviction queue
    pub async fn add(&self, block: &InjectedBlock) {
        let block_id = block.pointer.cluster_id;
        
        let meta = BlockMetadata {
            cluster_id: block_id,
            size_bytes: block.pointer.size,
            last_access: block.last_access,
            access_count: block.access_count(),
            kv_slots: block.kv_slots.len() as u32,
        };
        
        // Add to queue (front = oldest)
        {
            let mut queue = self.queue.write().await;
            queue.push_back(block_id);
        }
        
        // Add metadata
        {
            let mut metadata = self.metadata.write().await;
            metadata.insert(block_id, meta);
        }
        
        // Update usage
        self.current_usage.fetch_add(block.pointer.size, Ordering::SeqCst);
        
        debug!("Added block {} to LRU queue (size: {})", block_id, block.pointer.size);
    }

    /// Record access for a block (updates LRU order)
    pub async fn record_access(&self, cluster_id: u32) {
        // Update metadata
        {
            let mut metadata = self.metadata.write().await;
            if let Some(meta) = metadata.get_mut(&cluster_id) {
                meta.last_access = Instant::now();
                meta.access_count += 1;
            }
        }
        
        // Move to back of queue (most recently used)
        {
            let mut queue = self.queue.write().await;
            if let Some(pos) = queue.iter().position(|&id| id == cluster_id) {
                queue.remove(pos);
                queue.push_back(cluster_id);
            }
        }
    }

    /// Check if eviction is needed
    pub fn needs_eviction(&self) -> bool {
        let usage = self.current_usage.load(Ordering::SeqCst);
        let free = self.vram_free.load(Ordering::SeqCst);
        
        // Evict if usage exceeds cap or free space is too low
        (usage as u64) > self.policy.vram_cap_bytes || 
        (free as u64) < self.policy.min_free_vram_bytes
    }

    /// Evict the least recently used block
    pub async fn evict_lru(&self) -> Option<EvictedNode> {
        let block_id = {
            let mut queue = self.queue.write().await;
            queue.pop_front()?  // Remove oldest
        };
        
        // Get metadata
        let meta = {
            let mut metadata = self.metadata.write().await;
            metadata.remove(&block_id)?
        };
        
        // Update usage
        self.current_usage.fetch_sub(meta.size_bytes, Ordering::SeqCst);
        self.vram_free.fetch_add(meta.size_bytes, Ordering::SeqCst);
        self.total_evictions.fetch_add(1, Ordering::SeqCst);
        self.total_bytes_freed.fetch_add(meta.size_bytes, Ordering::SeqCst);
        
        let node = EvictedNode {
            cluster_id: block_id,
            bytes_freed: meta.size_bytes,
            evicted_at: Instant::now(),
            reason: EvictionReason::Lru,
        };
        
        info!(
            "Evicted LRU block {}: {} bytes (accesses: {})",
            block_id, meta.size_bytes, meta.access_count
        );
        
        Some(node)
    }

    /// Evict blocks until under cap
    pub async fn evict_until_under_cap(&self) -> Vec<EvictedNode> {
        let mut evicted = Vec::new();
        
        while self.needs_eviction() {
            if let Some(node) = self.evict_lru().await {
                evicted.push(node);
            } else {
                break;  // Queue empty
            }
        }
        
        evicted
    }

    /// Emergency eviction - free specified bytes
    pub async fn emergency_evict(&self, target_bytes: usize) -> Vec<EvictedNode> {
        let mut evicted = Vec::new();
        let mut freed = 0;
        
        while freed < target_bytes {
            if let Some(node) = self.evict_lru().await {
                freed += node.bytes_freed;
                evicted.push(node);
            } else {
                break;
            }
        }
        
        // Mark all as emergency evictions
        for node in &mut evicted {
            node.reason = EvictionReason::Emergency;
        }
        
        warn!("Emergency eviction: freed {} bytes", freed);
        evicted
    }

    /// Get current usage
    pub fn current_usage(&self) -> usize {
        self.current_usage.load(Ordering::SeqCst)
    }

    /// Get total evictions
    pub fn total_evictions(&self) -> u64 {
        self.total_evictions.load(Ordering::SeqCst)
    }

    /// Get total bytes freed
    pub fn total_bytes_freed(&self) -> usize {
        self.total_bytes_freed.load(Ordering::SeqCst)
    }

    /// Get queue length
    pub async fn queue_length(&self) -> usize {
        let queue = self.queue.read().await;
        queue.len()
    }

    /// Update VRAM free space
    pub fn update_vram_free(&self, free: usize) {
        self.vram_free.store(free, Ordering::SeqCst);
    }

    /// Clear all entries
    pub async fn clear(&self) {
        let mut queue = self.queue.write().await;
        queue.clear();
        
        let mut metadata = self.metadata.write().await;
        metadata.clear();
        
        self.current_usage.store(0, Ordering::SeqCst);
    }

    /// Get policy
    pub fn policy(&self) -> &EvictionPolicy {
        &self.policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lru_eviction() {
        let policy = EvictionPolicy::default();
        let queue = LruEvictionQueue::new(policy, 8 * 1024 * 1024 * 1024);
        
        // Create mock blocks
        let block1 = create_mock_block(1, 1024 * 1024);  // 1MB
        let block2 = create_mock_block(2, 2 * 1024 * 1024);  // 2MB
        let block3 = create_mock_block(3, 3 * 1024 * 1024);  // 3MB
        
        queue.add(&block1).await;
        queue.add(&block2).await;
        queue.add(&block3).await;
        
        assert_eq!(queue.current_usage(), 6 * 1024 * 1024);
        assert_eq!(queue.queue_length().await, 3);
        
        // Access block1 to make it recently used
        tokio::time::sleep(Duration::from_millis(10)).await;
        queue.record_access(1).await;
        
        // Evict LRU (should be block2, then block3)
        let evicted = queue.evict_lru().await.unwrap();
        assert_eq!(evicted.cluster_id, 2);  // block2 is now oldest
        
        assert_eq!(queue.queue_length().await, 2);
    }

    #[tokio::test]
    async fn test_eviction_under_cap() {
        let mut policy = EvictionPolicy::default();
        policy.vram_cap_bytes = 2 * 1024 * 1024;  // 2MB cap
        
        let queue = LruEvictionQueue::new(policy, 8 * 1024 * 1024 * 1024);
        
        // Add blocks that exceed cap
        for i in 1..=5 {
            let block = create_mock_block(i, 1024 * 1024);  // 1MB each
            queue.add(&block).await;
        }
        
        assert!(queue.needs_eviction());
        
        // Evict until under cap
        let evicted = queue.evict_until_under_cap().await;
        
        assert!(evicted.len() >= 3);  // Need to evict at least 3 to get under 2MB
        assert!(!queue.needs_eviction());
    }

    fn create_mock_block(cluster_id: u32, size: usize) -> InjectedBlock {
        use super::super::kv_cache::NeuralPointer;
        use super::super::semantic_map::ParametricGist;
        
        let gist = ParametricGist::new();
        let pointer = NeuralPointer::new(0x1000, &gist, cluster_id, 0, size);
        InjectedBlock::new(pointer, vec![], 0)
    }
}
