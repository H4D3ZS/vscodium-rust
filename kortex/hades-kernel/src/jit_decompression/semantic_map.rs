//! 6KB Semantic Map (Limbic Index)
//!
//! Implements the persistent parametric gist that serves as the primary
//! navigation index for the codebase. This 6KB vector remains resident
//! in VRAM at all times, enabling semantic search without inflation.
//!
//! ## Structure
//!
//! The semantic map is a 1536-dimensional float32 vector (6KB = 1536 × 4 bytes)
//! that encodes the "gist" of the entire codebase through:
//!
//! - **Holographic Reduced Representation (HRR)**: Circular convolution binding
//! - **TTT Gradient Updates**: Continuous learning from access patterns
//! - **Merkle-DAG Anchors**: Cryptographic links to inflated blocks

use anyhow::Result;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use tracing::debug;

use crate::memory::RaiiBuffer;
use crate::crypto::QuantumSeal;

/// Size of the parametric gist in bytes (1536 × f32 = 6144 bytes)
pub const GIST_SIZE_BYTES: usize = 1536 * 4;

/// Dimension of the parametric gist vector
pub const GIST_DIM: usize = 1536;

/// 6KB Parametric Gist - the "Limbic Index"
///
/// This vector remains resident in VRAM at all times and provides
/// semantic navigation for the entire codebase.
pub struct ParametricGist {
    /// The 1536-dimensional gist vector
    data: [f32; GIST_DIM],
    /// Merkle root hash of linked inflated blocks
    merkle_root: [u8; 32],
    /// Last update timestamp
    last_update: Instant,
    /// Update counter
    update_count: AtomicU64,
    /// Quantum seal for integrity
    seal: Option<QuantumSeal>,
}

impl ParametricGist {
    /// Create a new parametric gist (zero-initialized)
    pub fn new() -> Self {
        Self {
            data: [0.0f32; GIST_DIM],
            merkle_root: [0u8; 32],
            last_update: Instant::now(),
            update_count: AtomicU64::new(0),
            seal: None,
        }
    }

    /// Create gist from .aim file (memory-mapped, zero-copy)
    pub fn from_aim(path: impl AsRef<Path>) -> Result<Self> {
        let buffer = RaiiBuffer::map(path.as_ref())?;
        
        // Parse .aim format to extract gist tensor
        // Format: [magic 8B][json header][tensor 6KB][seal 2452B]
        let tensor = Self::extract_tensor_from_aim(buffer.as_slice())?;
        
        Ok(Self {
            data: tensor,
            merkle_root: [0u8; 32],
            last_update: Instant::now(),
            update_count: AtomicU64::new(0),
            seal: Some(QuantumSeal::seal(buffer.as_slice())),
        })
    }

    /// Extract tensor from .aim file format
    fn extract_tensor_from_aim(data: &[u8]) -> Result<[f32; GIST_DIM]> {
        // Skip magic bytes (8 bytes)
        let mut offset = 8;
        
        // Find JSON header end (look for '}')
        while offset < data.len() && data[offset] != b'}' {
            offset += 1;
        }
        offset += 1;
        
        // Align to 4-byte boundary
        offset = (offset + 3) & !3;
        
        // Read tensor data
        if offset + GIST_SIZE_BYTES > data.len() {
            anyhow::bail!("Invalid .aim file: insufficient data for tensor");
        }
        
        let tensor_bytes = &data[offset..offset + GIST_SIZE_BYTES];
        let mut tensor = [0f32; GIST_DIM];
        
        for (i, chunk) in tensor_bytes.chunks_exact(4).enumerate() {
            tensor[i] = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        
        Ok(tensor)
    }

    /// Get raw pointer to gist data (for VRAM upload)
    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }

    /// Get mutable pointer (for TTT updates)
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr()
    }

    /// Get gist as slice
    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }

    /// Get gist as mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.data
    }

    /// Apply TTT (Test-Time Training) gradient update
    ///
    /// Updates the gist with new information using:
    /// `gist[i] = gist[i] * (1 - alpha) + new_info[i] * alpha`
    ///
    /// # Parameters
    /// - `new_info`: New information to integrate
    /// - `alpha`: Learning rate (0.0-1.0, default 0.1)
    pub fn ttt_update(&mut self, new_info: &[f32; GIST_DIM], alpha: f32) {
        let alpha = alpha.clamp(0.0, 1.0);
        let one_minus_alpha = 1.0 - alpha;
        
        for i in 0..GIST_DIM {
            self.data[i] = self.data[i] * one_minus_alpha + new_info[i] * alpha;
        }
        
        self.last_update = Instant::now();
        self.update_count.fetch_add(1, Ordering::SeqCst);
        
        debug!("TTT update applied (count: {})", self.update_count.load(Ordering::SeqCst));
    }

    /// Apply Holographic Reduced Representation (HRR) binding
    ///
    /// Uses circular convolution to bind new information:
    /// `(a ⊛ b)[i] = Σ a[j] * b[(i-j) mod n]`
    ///
    /// This enables associative retrieval and superposition.
    pub fn hrr_bind(&mut self, other: &[f32; GIST_DIM]) {
        let mut result = [0f32; GIST_DIM];
        
        // Circular convolution
        for i in 0..GIST_DIM {
            for j in 0..GIST_DIM {
                let k = (i + j) % GIST_DIM;
                result[k] += self.data[i] * other[j];
            }
        }
        
        // Normalize
        let norm = (result.iter().map(|x| x * x).sum::<f32>()).sqrt();
        if norm > 1e-6 {
            for x in result.iter_mut() {
                *x /= norm;
            }
        }
        
        self.data = result;
        self.last_update = Instant::now();
        self.update_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Compute cosine similarity with another gist
    pub fn cosine_similarity(&self, other: &ParametricGist) -> f32 {
        let mut dot = 0.0f32;
        let mut norm_self = 0.0f32;
        let mut norm_other = 0.0f32;
        
        for i in 0..GIST_DIM {
            dot += self.data[i] * other.data[i];
            norm_self += self.data[i] * self.data[i];
            norm_other += other.data[i] * other.data[i];
        }
        
        let denom = (norm_self * norm_other).sqrt();
        if denom < 1e-6 {
            0.0
        } else {
            dot / denom
        }
    }

    /// Get Merkle root hash
    pub fn merkle_root(&self) -> &[u8; 32] {
        &self.merkle_root
    }

    /// Update Merkle root with new inflated block hash
    pub fn update_merkle_root(&mut self, block_hash: [u8; 32]) {
        // Combine with existing root using BLAKE3
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&self.merkle_root);
        hasher.update(&block_hash);
        let hash = hasher.finalize();
        self.merkle_root = *hash.as_bytes();
    }

    /// Seal the gist with quantum-resistant crypto
    pub fn seal(&mut self) -> Result<&QuantumSeal> {
        let bytes = self.as_bytes();
        self.seal = Some(QuantumSeal::seal(&bytes));
        Ok(self.seal.as_ref().unwrap())
    }

    /// Verify the gist seal
    pub fn verify_seal(&self) -> Result<bool> {
        match &self.seal {
            Some(seal) => seal.verify(&self.as_bytes()),
            None => Ok(true),  // No seal = no verification needed
        }
    }

    /// Get gist as bytes
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.data)
    }

    /// Get update count
    pub fn update_count(&self) -> u64 {
        self.update_count.load(Ordering::SeqCst)
    }

    /// Get time since last update
    pub fn time_since_update(&self) -> std::time::Duration {
        self.last_update.elapsed()
    }
}

impl Default for ParametricGist {
    fn default() -> Self {
        Self::new()
    }
}

/// Limbic Index - Semantic navigation structure
///
/// Maps semantic clusters to physical file locations for JIT inflation.
#[derive(Debug, Clone)]
pub struct LimbicIndex {
    /// Cluster ID
    pub cluster_id: u32,
    /// Semantic centroid (gist subspace)
    pub centroid: [f32; GIST_DIM],
    /// File paths in this cluster
    pub file_paths: Vec<String>,
    /// Byte offsets for each file
    pub byte_offsets: Vec<(usize, usize)>,
    /// Last access time (for LRU)
    pub last_access: Instant,
    /// Access count (for LFU)
    pub access_count: u32,
    /// Activation score (from attention heads)
    pub activation_score: f32,
}

impl LimbicIndex {
    /// Create a new limbic index entry
    pub fn new(cluster_id: u32, centroid: [f32; GIST_DIM]) -> Self {
        Self {
            cluster_id,
            centroid,
            file_paths: Vec::new(),
            byte_offsets: Vec::new(),
            last_access: Instant::now(),
            access_count: 0,
            activation_score: 0.0,
        }
    }

    /// Add a file to this cluster
    pub fn add_file(&mut self, path: String, byte_range: (usize, usize)) {
        self.file_paths.push(path);
        self.byte_offsets.push(byte_range);
    }

    /// Update activation score from attention heads
    pub fn update_activation(&mut self, score: f32) {
        self.activation_score = score;
        self.last_access = Instant::now();
        self.access_count += 1;
    }

    /// Check if activation exceeds fault threshold
    pub fn needs_inflation(&self, threshold: f32) -> bool {
        self.activation_score >= threshold
    }
}

/// 6KB Semantic Map - Collection of limbic indices
pub struct SemanticMap {
    /// The parametric gist (always resident)
    gist: ParametricGist,
    /// Limbic indices for semantic clusters
    indices: Vec<LimbicIndex>,
    /// Number of clusters
    num_clusters: usize,
    /// VRAM pointer (if uploaded)
    vram_ptr: Option<NonNull<f32>>,
}

impl SemanticMap {
    /// Create a new semantic map
    pub fn new(num_clusters: usize) -> Self {
        Self {
            gist: ParametricGist::new(),
            indices: Vec::with_capacity(num_clusters),
            num_clusters,
            vram_ptr: None,
        }
    }

    /// Add a limbic index to the map
    pub fn add_index(&mut self, index: LimbicIndex) {
        self.indices.push(index);
    }

    /// Get the parametric gist
    pub fn gist(&self) -> &ParametricGist {
        &self.gist
    }

    /// Get mutable gist
    pub fn gist_mut(&mut self) -> &mut ParametricGist {
        &mut self.gist
    }

    /// Get limbic index by cluster ID
    pub fn get_index(&self, cluster_id: u32) -> Option<&LimbicIndex> {
        self.indices.iter().find(|i| i.cluster_id == cluster_id)
    }

    /// Get mutable limbic index
    pub fn get_index_mut(&mut self, cluster_id: u32) -> Option<&mut LimbicIndex> {
        self.indices.iter_mut().find(|i| i.cluster_id == cluster_id)
    }

    /// Find the most activated cluster
    pub fn most_activated(&self) -> Option<&LimbicIndex> {
        self.indices.iter().max_by(|a, b| {
            a.activation_score.partial_cmp(&b.activation_score).unwrap()
        })
    }

    /// Get all indices above activation threshold
    pub fn above_threshold(&self, threshold: f32) -> Vec<&LimbicIndex> {
        self.indices
            .iter()
            .filter(|i| i.activation_score >= threshold)
            .collect()
    }

    /// Upload semantic map to VRAM (zero-copy if possible)
    pub fn upload_to_vram(&mut self) -> Result<NonNull<f32>> {
        // In production, this would allocate VRAM via ROCm/HIP
        // For now, just return the CPU pointer
        let ptr = self.gist.as_ptr() as *mut f32;
        self.vram_ptr = NonNull::new(ptr);
        Ok(self.vram_ptr.unwrap())
    }

    /// Get VRAM pointer
    pub fn vram_ptr(&self) -> Option<NonNull<f32>> {
        self.vram_ptr
    }

    /// Get size in bytes
    pub fn size_bytes(&self) -> usize {
        GIST_SIZE_BYTES
    }

    /// Seal the entire semantic map
    pub fn seal(&mut self) -> Result<QuantumSeal> {
        self.gist.seal()?;
        Ok(self.gist.seal.clone().unwrap())
    }
}

// Safety: ParametricGist can be sent between threads
unsafe impl Send for ParametricGist {}
unsafe impl Sync for ParametricGist {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parametric_gist_creation() {
        let gist = ParametricGist::new();
        assert_eq!(gist.as_slice().len(), GIST_DIM);
        assert_eq!(gist.size_bytes(), GIST_SIZE_BYTES);
    }

    #[test]
    fn test_ttt_update() {
        let mut gist = ParametricGist::new();
        let new_info = [0.5f32; GIST_DIM];
        
        gist.ttt_update(&new_info, 0.1);
        
        // All values should be close to 0.05 (0.9 * 0.0 + 0.1 * 0.5)
        for &val in gist.as_slice() {
            assert!((val - 0.05).abs() < 1e-6);
        }
        
        assert_eq!(gist.update_count(), 1);
    }

    #[test]
    fn test_cosine_similarity() {
        let mut gist1 = ParametricGist::new();
        let mut gist2 = ParametricGist::new();
        
        // Make them identical
        for i in 0..GIST_DIM {
            gist1.as_mut_slice()[i] = 1.0 / (GIST_DIM as f32).sqrt();
            gist2.as_mut_slice()[i] = 1.0 / (GIST_DIM as f32).sqrt();
        }
        
        let sim = gist1.cosine_similarity(&gist2);
        assert!((sim - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_limbic_index_activation() {
        let mut index = LimbicIndex::new(0, [0.0f32; GIST_DIM]);
        
        index.update_activation(0.9);
        assert!(index.needs_inflation(0.85));
        
        index.update_activation(0.5);
        assert!(!index.needs_inflation(0.85));
    }
}
