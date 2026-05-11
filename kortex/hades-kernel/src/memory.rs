//! RAII Memory Management for 8GB VRAM Environment
//! 
//! Zero-copy memory-mapped I/O with strict ownership semantics.
//! No garbage collector, no bloat - pure Rust RAII.

use anyhow::{Result, Context};
use memmap2::Mmap;
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Arc;
use tracing::{debug, trace};

/// Resource Acquisition Is Initialization buffer
/// Automatically unmaps on drop - no manual cleanup
#[derive(Debug)]
pub struct RaiiBuffer {
    mmap: Mmap,
    path: String,
    len: usize,
}

impl RaiiBuffer {
    /// Memory-map a file with zero-copy semantics
    pub fn map<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let file = std::fs::File::open(path_ref)
            .with_context(|| format!("Failed to open: {}", path_ref.display()))?;
        
        let mmap = unsafe { Mmap::map(&file) }
            .with_context(|| format!("Failed to mmap: {}", path_ref.display()))?;
        
        let len = mmap.len();
        let path_str = path_ref.display().to_string();
        
        debug!("Mapped {} bytes from {}", len, path_str);
        
        Ok(Self {
            mmap,
            path: path_str,
            len,
        })
    }
    
    /// Get raw pointer to data (for FFI with llama.cpp/Candle)
    pub fn as_ptr(&self) -> *const u8 {
        self.mmap.as_ptr()
    }
    
    /// Get data slice
    pub fn as_slice(&self) -> &[u8] {
        &self.mmap
    }
    
    /// Get length in bytes
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Get source path
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for RaiiBuffer {
    fn drop(&mut self) {
        trace!("Unmapping {} bytes from {}", self.len, self.path);
        // Mmap drops automatically - RAII handles cleanup
    }
}

/// Memory-mapped tensor guard for 1536-dim float32 vectors
/// Provides safe access to .aim tensor data
pub struct AimMemoryGuard {
    #[allow(dead_code)]
    mmap: Mmap,
    tensor_ptr: NonNull<f32>,
    tensor_len: usize,
}

impl AimMemoryGuard {
    /// Map .aim file and locate tensor (after magic + JSON header)
    /// 
    /// .aim format:
    /// - Magic bytes (8 bytes)
    /// - JSON header (variable, terminated by '}')
    /// - Tensor data (1536 × f32 = 6144 bytes)
    /// - Optional KV-cache
    /// - ML-DSA seal (2420 bytes)
    pub fn map_aim<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let file = std::fs::File::open(path_ref)
            .with_context(|| format!("Failed to open .aim: {}", path_ref.display()))?;
        
        let mmap = unsafe { Mmap::map(&file) }
            .with_context(|| format!("Failed to mmap .aim: {}", path_ref.display()))?;
        
        // Parse header to find tensor offset
        let header_end = Self::find_json_header_end(&mmap)?;
        let tensor_offset = header_end;
        
        // Safety: tensor data is aligned to 4 bytes in .aim format
        let tensor_ptr = unsafe {
            let ptr = mmap.as_ptr().add(tensor_offset) as *const f32;
            NonNull::new(ptr as *mut f32)
                .context("Null tensor pointer in .aim file")?
        };
        
        let tensor_len = 1536; // Fixed for neuraldaredevil embeddings
        
        debug!(
            "Mapped .aim tensor: {} dims @ offset {} ({} bytes)",
            tensor_len, tensor_offset, tensor_len * 4
        );
        
        Ok(Self {
            mmap,
            tensor_ptr,
            tensor_len,
        })
    }
    
    /// Find end of JSON header (look for '}')
    fn find_json_header_end(mmap: &Mmap) -> Result<usize> {
        // Skip magic bytes (8 bytes)
        let start = 8;
        
        // Find closing brace
        for i in start..mmap.len() {
            if mmap[i] == b'}' {
                // Align to 4-byte boundary for f32
                return Ok((i + 1 + 3) & !3);
            }
        }
        
        anyhow::bail!("No JSON header terminator found in .aim")
    }
    
    /// Get tensor as f32 slice
    pub fn as_tensor(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(self.tensor_ptr.as_ptr(), self.tensor_len)
        }
    }
    
    /// Get mutable tensor (for TTT updates)
    pub fn as_tensor_mut(&mut self) -> &mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(self.tensor_ptr.as_ptr(), self.tensor_len)
        }
    }
    
    /// Get raw pointer for FFI
    pub fn tensor_ptr(&self) -> *const f32 {
        self.tensor_ptr.as_ptr()
    }
}

impl Drop for AimMemoryGuard {
    fn drop(&mut self) {
        trace!(
            "Unmapping .aim tensor: {} dims @ {:?}",
            self.tensor_len,
            self.tensor_ptr
        );
    }
}

/// Memory-mapped tensor with shared ownership (Arc-backed)
/// For multi-threaded access without copying
pub struct MappedTensor {
    inner: Arc<AimMemoryGuard>,
}

impl MappedTensor {
    /// Create shared tensor mapping
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let guard = AimMemoryGuard::map_aim(path)?;
        Ok(Self {
            inner: Arc::new(guard),
        })
    }
    
    /// Get tensor reference (thread-safe read)
    pub fn as_tensor(&self) -> &[f32] {
        self.inner.as_tensor()
    }
    
    /// Clone is cheap (Arc increment)
    pub fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Clone for MappedTensor {
    fn clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_raii_buffer_lifecycle() {
        // Create temp file for testing
        let temp_path = std::env::temp_dir().join("hades_test.bin");
        std::fs::write(&temp_path, b"test data").unwrap();
        
        {
            let buf = RaiiBuffer::map(&temp_path).unwrap();
            assert_eq!(buf.as_slice(), b"test data");
            assert_eq!(buf.len(), 9);
            // Drop happens here - RAII cleanup
        }
        
        std::fs::remove_file(temp_path).unwrap();
    }
}
