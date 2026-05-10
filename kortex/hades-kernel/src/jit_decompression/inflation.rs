//! JIT Inflation Engine via io_uring
//!
//! Implements zero-copy SSD→VRAM streaming for inflated code blocks.
//! Uses io_uring (Linux) or overlapped I/O (Windows) for non-blocking operations.
//!
//! ## Flow
//!
//! 1. Receive `InflationRequest` from fault handler
//! 2. Check thermal throttle state
//! 3. Submit io_uring read request for source file
//! 4. Stream directly to VRAM scratchpad (zero-copy)
//! 5. Notify KV-cache injector when complete

#[cfg(target_os = "linux")]
use io_uring::{IoUring, IoUringProbe};
use std::path::Path;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::memory::RaiiBuffer;
use crate::thermal::ThermalGovernor;
use super::fault_handler::InflationRequest;
use super::{JitMetrics, JitStatus, JitErrorCode};

/// Size of the VRAM scratchpad buffer (default: 512MB)
pub const DEFAULT_SCRATCHPAD_SIZE: usize = 512 * 1024 * 1024;

/// Maximum inflation request timeout
pub const INFLATION_TIMEOUT: Duration = Duration::from_secs(5);

/// Result of an inflation operation
#[derive(Debug, Clone)]
pub struct InflationResult {
    /// Cluster ID that was inflated
    pub cluster_id: u32,
    /// Bytes inflated from SSD
    pub bytes_inflated: usize,
    /// VRAM offset where data was placed
    pub vram_offset: usize,
    /// Operation latency in microseconds
    pub latency_us: u64,
    /// Whether thermal throttle was applied
    pub thermal_throttled: bool,
}

/// VRAM scratchpad buffer for inflated blocks
pub struct ScratchpadBuffer {
    /// Raw pointer to VRAM buffer
    ptr: NonNull<u8>,
    /// Total size in bytes
    size: usize,
    /// Current usage
    usage: AtomicUsize,
    /// Memory-mapped file (for zero-copy)
    #[allow(dead_code)]
    mmap: Option<RaiiBuffer>,
}

// Safety: ScratchpadBuffer can be shared with proper synchronization
unsafe impl Send for ScratchpadBuffer {}
unsafe impl Sync for ScratchpadBuffer {}

impl ScratchpadBuffer {
    /// Create a new scratchpad buffer (CPU-backed for now)
    pub fn new(size: usize) -> Self {
        // In production, this would allocate VRAM via ROCm/HIP
        // For now, we use a Vec as a placeholder
        let data = vec![0u8; size];
        let ptr = NonNull::new(data.as_ptr() as *mut u8).unwrap();
        
        // Leak the Vec to prevent deallocation
        // In production, VRAM allocation handles this
        std::mem::forget(data);
        
        Self {
            ptr,
            size,
            usage: AtomicUsize::new(0),
            mmap: None,
        }
    }

    /// Get pointer to a region in the scratchpad
    pub fn get_region(&self, offset: usize, size: usize) -> Option<NonNull<u8>> {
        if offset + size > self.size {
            return None;
        }
        
        let ptr = unsafe { self.ptr.as_ptr().add(offset) };
        NonNull::new(ptr)
    }

    /// Get current usage
    pub fn usage(&self) -> usize {
        self.usage.load(Ordering::SeqCst)
    }

    /// Get available space
    pub fn available(&self) -> usize {
        self.size - self.usage()
    }

    /// Allocate a region in the scratchpad
    pub fn allocate(&self, size: usize) -> Option<usize> {
        let current = self.usage.load(Ordering::SeqCst);
        if current + size > self.size {
            return None;  // Not enough space
        }
        
        // Simple bump allocator (production needs proper fragmentation handling)
        let offset = current;
        self.usage.fetch_add(size, Ordering::SeqCst);
        Some(offset)
    }

    /// Free a region in the scratchpad
    pub fn free(&self, offset: usize, size: usize) {
        // In production, this would track free regions
        // For now, just decrease usage (simplified)
        self.usage.fetch_sub(size.min(self.usage()), Ordering::SeqCst);
    }

    /// Get total size
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for ScratchpadBuffer {
    fn drop(&mut self) {
        // In production, free VRAM allocation
        debug!("Dropping scratchpad buffer ({} bytes)", self.size);
    }
}

/// io_uring wrapper for zero-copy I/O
#[cfg(target_os = "linux")]
struct IoUringEngine {
    ring: IoUring,
    depth: usize,
    enabled: bool,
}

#[cfg(target_os = "linux")]
impl IoUringEngine {
    fn new(depth: usize) -> Option<Self> {
        // Check if io_uring is available
        if let Ok(probe) = IoUringProbe::new() {
            for op in probe.iter() {
                if op.op == io_uring::opcode::Read::CODE {
                    if let Ok(mut ring) = IoUring::new(depth as u32) {
                        return Some(Self {
                            ring,
                            depth,
                            enabled: true,
                        });
                    }
                }
            }
        }
        None
    }

    /// Submit async read request
    async fn read_at(&mut self, fd: i32, offset: u64, buf: *mut u8, len: usize) -> io_uring::Result<usize> {
        use io_uring::opcode;
        
        let (submitter, mut queue) = self.ring.submitter().split();
        
        unsafe {
            let entry = opcode::Read::new(
                opcode::types::Fd(fd),
                buf,
                len as u32,
            )
            .offset(offset as i64)
            .build();
            
            queue.push(&entry)?;
            submitter.submit()?;
        }
        
        // Wait for completion (simplified - production uses async)
        self.ring.submit_and_wait(1)?;
        
        let cqe = self.ring.completion().next().expect("Completion expected");
        let result = cqe.result();
        
        if result < 0 {
            Err(std::io::Error::from_raw_os_error(-result))
        } else {
            Ok(result as usize)
        }
    }
}

/// JIT Inflation Engine
pub struct InflationEngine {
    /// Scratchpad buffer
    scratchpad: Arc<ScratchpadBuffer>,
    /// Thermal governor
    thermal_governor: Arc<ThermalGovernor>,
    /// io_uring engine (Linux only)
    #[cfg(target_os = "linux")]
    io_uring: Option<IoUringEngine>,
    /// Use io_uring flag
    use_io_uring: bool,
    /// Status
    status: AtomicU32,  // JitStatus as u32
    /// Current inflation cluster
    current_cluster: AtomicU32,
    /// Metrics
    metrics: Arc<RwLock<JitMetrics>>,
    /// Enabled flag
    enabled: AtomicBool,
}

impl InflationEngine {
    /// Create a new inflation engine
    pub fn new(
        scratchpad_size: usize,
        thermal_governor: Arc<ThermalGovernor>,
        use_io_uring: bool,
        io_uring_depth: usize,
    ) -> Self {
        #[cfg(target_os = "linux")]
        let io_uring = if use_io_uring {
            IoUringEngine::new(io_uring_depth)
        } else {
            None
        };
        
        Self {
            scratchpad: Arc::new(ScratchpadBuffer::new(scratchpad_size)),
            thermal_governor,
            #[cfg(target_os = "linux")]
            io_uring,
            use_io_uring,
            status: AtomicU32::new(0),  // JitStatus::Idle = 0
            current_cluster: AtomicU32::new(u32::MAX),
            metrics: Arc::new(RwLock::new(JitMetrics::default())),
            enabled: AtomicBool::new(true),
        }
    }

    /// Process an inflation request
    pub async fn inflate(&self, request: InflationRequest, file_path: &Path) -> Result<InflationResult, JitInflationError> {
        if !self.enabled.load(Ordering::SeqCst) {
            return Err(JitInflationError::EngineDisabled);
        }

        let start = Instant::now();
        
        // Check thermal state
        let throttle_ratio = self.thermal_governor.throttle_ratio();
        let thermal_throttled = throttle_ratio < 1.0;
        
        if thermal_throttled {
            info!("Thermal throttle active, applying back-pressure");
            self.thermal_governor.apply_back_pressure().await;
            
            // Update metrics
            let mut metrics = self.metrics.write().await;
            metrics.record_thermal_throttle();
        }

        // Check scratchpad space
        let file_size = std::fs::metadata(file_path)
            .map(|m| m.len() as usize)
            .unwrap_or(0);

        if self.scratchpad.available() < file_size {
            warn!("Scratchpad full, eviction needed");
            self.status.store(3, Ordering::SeqCst);  // JitStatus::Evicting
            // In production, trigger LRU eviction here
        }

        // Set status
        self.status.store(1, Ordering::SeqCst);  // JitStatus::Inflating
        self.current_cluster.store(request.cluster_id, Ordering::SeqCst);

        // Allocate scratchpad region
        let vram_offset = self.scratchpad.allocate(file_size)
            .ok_or(JitInflationError::ScratchpadFull)?;

        // Perform zero-copy read
        let bytes_read = self.zero_copy_read(file_path, vram_offset, file_size).await?;

        let latency_us = start.elapsed().as_micros() as u64;

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.record_inflation(bytes_read, latency_us);
            metrics.scratchpad_usage_bytes = self.scratchpad.usage() as u64;
        }

        // Set status back to idle
        self.status.store(0, Ordering::SeqCst);  // JitStatus::Idle
        self.current_cluster.store(u32::MAX, Ordering::SeqCst);
        
        Ok(InflationResult {
            cluster_id: request.cluster_id,
            bytes_inflated: bytes_read,
            vram_offset,
            latency_us,
            thermal_throttled,
        })
    }

    /// Zero-copy read from SSD to scratchpad
    async fn zero_copy_read(&self, file_path: &Path, vram_offset: usize, size: usize) -> Result<usize, JitInflationError> {
        #[cfg(target_os = "linux")]
        if self.use_io_uring && self.io_uring.is_some() {
            return self.io_uring_read(file_path, vram_offset, size).await;
        }
        
        // Fallback: standard async read
        self.tokio_read(file_path, vram_offset, size).await
    }

    /// io_uring-based read (Linux only)
    #[cfg(target_os = "linux")]
    async fn io_uring_read(&self, file_path: &Path, vram_offset: usize, size: usize) -> Result<usize, JitInflationError> {
        use std::os::unix::fs::FileExt;
        
        let file = std::fs::File::open(file_path)
            .map_err(|e| JitInflationError::IoError(e))?;
        
        let fd = file.as_raw_fd();
        
        // Get scratchpad pointer
        let scratchpad_ptr = self.scratchpad
            .get_region(vram_offset, size)
            .ok_or(JitInflationError::ScratchpadFull)?
            .as_ptr();
        
        // Submit io_uring read
        if let Some(mut io_uring) = &self.io_uring {
            match tokio::time::timeout(
                INFLATION_TIMEOUT,
                io_uring.read_at(fd, 0, scratchpad_ptr, size),
            ).await {
                Ok(Ok(bytes)) => Ok(bytes),
                Ok(Err(e)) => Err(JitInflationError::IoUringError(e)),
                Err(_) => Err(JitInflationError::Timeout),
            }
        } else {
            self.tokio_read(file_path, vram_offset, size).await
        }
    }

    /// Tokio-based read (fallback)
    async fn tokio_read(&self, file_path: &Path, vram_offset: usize, size: usize) -> Result<usize, JitInflationError> {
        // In production, use tokio::fs with proper async I/O
        // For now, use blocking read in spawn_blocking
        let file_path = file_path.to_path_buf();
        
        let result = tokio::task::spawn_blocking(move || {
            let file = std::fs::File::open(&file_path)?;
            let mut reader = std::io::BufReader::new(file);
            
            // Read into buffer (in production, this goes directly to VRAM)
            let mut buffer = vec![0u8; size];
            let bytes_read = std::io::Read::read(&mut reader, &mut buffer)?;
            
            Ok::<usize, std::io::Error>(bytes_read)
        })
        .await
        .map_err(|e| JitInflationError::TaskError(e))?
        .map_err(JitInflationError::IoError)?;
        
        Ok(result)
    }

    /// Get scratchpad usage
    pub fn scratchpad_usage(&self) -> usize {
        self.scratchpad.usage()
    }

    /// Get scratchpad available space
    pub fn scratchpad_available(&self) -> usize {
        self.scratchpad.available()
    }

    /// Get current status
    pub fn status(&self) -> JitStatus {
        match self.status.load(Ordering::SeqCst) {
            0 => JitStatus::Idle,
            1 => JitStatus::Inflating { cluster_id: self.current_cluster.load(Ordering::SeqCst) },
            2 => JitStatus::ThermalWait { temp_c: self.thermal_governor.telemetry().temperature_c },
            3 => JitStatus::Evicting { bytes_freed: 0 },
            _ => JitStatus::Error { code: JitErrorCode::ScratchpadFull },
        }
    }

    /// Get metrics
    pub async fn metrics(&self) -> JitMetrics {
        self.metrics.read().await.clone()
    }

    /// Enable/disable engine
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }
}

/// Inflation error types
#[derive(Debug)]
pub enum JitInflationError {
    /// Engine disabled
    EngineDisabled,
    /// Scratchpad full
    ScratchpadFull,
    /// io_uring error (Linux)
    #[cfg(target_os = "linux")]
    IoUringError(io_uring::Result<usize>),
    /// I/O error
    IoError(std::io::Error),
    /// Timeout
    Timeout,
    /// Task join error
    TaskError(tokio::task::JoinError),
}

impl std::fmt::Display for JitInflationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitInflationError::EngineDisabled => write!(f, "Inflation engine disabled"),
            JitInflationError::ScratchpadFull => write!(f, "Scratchpad buffer full"),
            #[cfg(target_os = "linux")]
            JitInflationError::IoUringError(e) => write!(f, "io_uring error: {:?}", e),
            JitInflationError::IoError(e) => write!(f, "I/O error: {}", e),
            JitInflationError::Timeout => write!(f, "Inflation timeout"),
            JitInflationError::TaskError(e) => write!(f, "Task error: {}", e),
        }
    }
}

impl std::error::Error for JitInflationError {}

// Safety: InflationEngine can be shared across threads
unsafe impl Send for InflationEngine {}
unsafe impl Sync for InflationEngine {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thermal::ThermalPolicy;

    #[tokio::test]
    async fn test_scratchpad_allocation() {
        let thermal = Arc::new(ThermalGovernor::default());
        let engine = InflationEngine::new(512 * 1024 * 1024, thermal, false, 256);
        
        assert_eq!(engine.scratchpad_available(), 512 * 1024 * 1024);
        
        // Allocate some space
        let offset = engine.scratchpad.allocate(1024);
        assert!(offset.is_some());
        
        assert!(engine.scratchpad_usage() >= 1024);
    }

    #[tokio::test]
    async fn test_inflation_engine_creation() {
        let thermal = Arc::new(ThermalGovernor::default());
        let engine = InflationEngine::new(512 * 1024 * 1024, thermal, cfg!(target_os = "linux"), 256);
        
        assert_eq!(engine.status(), JitStatus::Idle);
    }
}
