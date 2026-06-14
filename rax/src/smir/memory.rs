//! SMIR memory interface.
//!
//! This module defines the memory trait for SMIR execution and related types.

use crate::smir::types::*;

// ============================================================================
// Memory Error
// ============================================================================

/// Memory access error
#[derive(Clone, Debug)]
pub enum MemoryError {
    /// Page fault (page not present)
    PageFault {
        addr: GuestAddr,
        write: bool,
        user: bool,
    },

    /// Access violation (permission denied)
    AccessViolation { addr: GuestAddr, write: bool },

    /// Alignment fault
    Alignment { addr: GuestAddr, required: usize },

    /// MMIO region (needs special handling)
    Mmio { addr: GuestAddr, size: usize },

    /// Exclusive monitor failure
    ExclusiveFailed,

    /// Address out of bounds
    OutOfBounds { addr: GuestAddr },
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::PageFault { addr, write, user } => {
                write!(
                    f,
                    "page fault at {:#x} ({}, {})",
                    addr,
                    if *write { "write" } else { "read" },
                    if *user { "user" } else { "kernel" }
                )
            }
            MemoryError::AccessViolation { addr, write } => {
                write!(
                    f,
                    "access violation at {:#x} ({})",
                    addr,
                    if *write { "write" } else { "read" }
                )
            }
            MemoryError::Alignment { addr, required } => {
                write!(
                    f,
                    "alignment fault at {:#x} (required {} bytes)",
                    addr, required
                )
            }
            MemoryError::Mmio { addr, size } => {
                write!(f, "MMIO access at {:#x} ({} bytes)", addr, size)
            }
            MemoryError::ExclusiveFailed => write!(f, "exclusive monitor failed"),
            MemoryError::OutOfBounds { addr } => write!(f, "address out of bounds: {:#x}", addr),
        }
    }
}

impl std::error::Error for MemoryError {}

// ============================================================================
// Memory Trait
// ============================================================================

/// Memory interface for SMIR execution
pub trait SmirMemory: Send {
    /// Read bytes from memory
    fn read(&mut self, addr: GuestAddr, buf: &mut [u8]) -> Result<(), MemoryError>;

    /// Write bytes to memory
    fn write(&mut self, addr: GuestAddr, data: &[u8]) -> Result<(), MemoryError>;

    /// Atomic load
    fn atomic_load(
        &mut self,
        addr: GuestAddr,
        size: MemWidth,
        order: MemoryOrder,
    ) -> Result<u64, MemoryError>;

    /// Atomic store
    fn atomic_store(
        &mut self,
        addr: GuestAddr,
        value: u64,
        size: MemWidth,
        order: MemoryOrder,
    ) -> Result<(), MemoryError>;

    /// Atomic compare-and-swap
    fn compare_and_swap(
        &mut self,
        addr: GuestAddr,
        expected: u64,
        new: u64,
        size: MemWidth,
        success_order: MemoryOrder,
        failure_order: MemoryOrder,
    ) -> Result<(u64, bool), MemoryError>;

    /// Atomic read-modify-write
    fn atomic_rmw(
        &mut self,
        addr: GuestAddr,
        op: AtomicOp,
        operand: u64,
        size: MemWidth,
        order: MemoryOrder,
    ) -> Result<u64, MemoryError>;

    /// Load-linked / Load-exclusive
    fn load_exclusive(&mut self, addr: GuestAddr, size: MemWidth) -> Result<u64, MemoryError>;

    /// Store-conditional / Store-exclusive (returns true if succeeded)
    fn store_exclusive(
        &mut self,
        addr: GuestAddr,
        value: u64,
        size: MemWidth,
    ) -> Result<bool, MemoryError>;

    /// Clear exclusive monitor
    fn clear_exclusive(&mut self);

    /// Memory fence
    fn fence(&mut self, kind: FenceKind);

    /// Prefetch hint (optional, default is no-op)
    fn prefetch(&mut self, _addr: GuestAddr, _write: bool) {
        // Default: no-op
    }

    /// Check if address range is valid
    fn probe(&self, addr: GuestAddr, size: usize, write: bool) -> Result<(), MemoryError>;
}

/// Memory reader interface for lifting (read-only)
pub trait MemoryReader: Send + Sync {
    fn read(&self, addr: GuestAddr, size: usize) -> Result<Vec<u8>, MemoryError>;
}

// ============================================================================
// Simple Memory Implementation
// ============================================================================

/// Simple flat memory implementation for testing
pub struct FlatMemory {
    /// Memory contents
    data: Vec<u8>,
    /// Base address
    base: GuestAddr,
    /// Exclusive monitor address
    exclusive_addr: Option<GuestAddr>,
    /// Exclusive monitor size
    exclusive_size: MemWidth,
}

impl FlatMemory {
    /// Create new flat memory with given size
    pub fn new(size: usize) -> Self {
        FlatMemory {
            data: vec![0; size],
            base: 0,
            exclusive_addr: None,
            exclusive_size: MemWidth::B8,
        }
    }

    /// Create new flat memory with given base address and size
    pub fn with_base(base: GuestAddr, size: usize) -> Self {
        FlatMemory {
            data: vec![0; size],
            base,
            exclusive_addr: None,
            exclusive_size: MemWidth::B8,
        }
    }

    /// Get offset from address
    fn offset(&self, addr: GuestAddr) -> Result<usize, MemoryError> {
        if addr < self.base {
            return Err(MemoryError::OutOfBounds { addr });
        }
        let off = (addr - self.base) as usize;
        if off >= self.data.len() {
            return Err(MemoryError::OutOfBounds { addr });
        }
        Ok(off)
    }

    /// Load the memory with initial data
    pub fn load(&mut self, offset: usize, data: &[u8]) {
        let end = offset + data.len();
        if end <= self.data.len() {
            self.data[offset..end].copy_from_slice(data);
        }
    }
}

impl SmirMemory for FlatMemory {
    fn read(&mut self, addr: GuestAddr, buf: &mut [u8]) -> Result<(), MemoryError> {
        let off = self.offset(addr)?;
        let end = off + buf.len();
        if end > self.data.len() {
            return Err(MemoryError::OutOfBounds {
                addr: addr + buf.len() as u64,
            });
        }
        buf.copy_from_slice(&self.data[off..end]);
        Ok(())
    }

    fn write(&mut self, addr: GuestAddr, data: &[u8]) -> Result<(), MemoryError> {
        let off = self.offset(addr)?;
        let end = off + data.len();
        if end > self.data.len() {
            return Err(MemoryError::OutOfBounds {
                addr: addr + data.len() as u64,
            });
        }
        self.data[off..end].copy_from_slice(data);

        // Check exclusive monitor
        if let Some(excl_addr) = self.exclusive_addr {
            let excl_end = excl_addr + self.exclusive_size.bytes() as u64;
            let write_end = addr + data.len() as u64;
            if !(write_end <= excl_addr || addr >= excl_end) {
                self.exclusive_addr = None;
            }
        }

        Ok(())
    }

    fn atomic_load(
        &mut self,
        addr: GuestAddr,
        size: MemWidth,
        _order: MemoryOrder,
    ) -> Result<u64, MemoryError> {
        let mut buf = [0u8; 8];
        let bytes = size.bytes() as usize;
        self.read(addr, &mut buf[..bytes])?;
        Ok(u64::from_le_bytes(buf))
    }

    fn atomic_store(
        &mut self,
        addr: GuestAddr,
        value: u64,
        size: MemWidth,
        _order: MemoryOrder,
    ) -> Result<(), MemoryError> {
        let bytes = value.to_le_bytes();
        self.write(addr, &bytes[..size.bytes() as usize])
    }

    fn compare_and_swap(
        &mut self,
        addr: GuestAddr,
        expected: u64,
        new: u64,
        size: MemWidth,
        _success_order: MemoryOrder,
        _failure_order: MemoryOrder,
    ) -> Result<(u64, bool), MemoryError> {
        let old = self.atomic_load(addr, size, MemoryOrder::SeqCst)?;
        let mask = match size {
            MemWidth::B1 => 0xFF,
            MemWidth::B2 => 0xFFFF,
            MemWidth::B4 => 0xFFFF_FFFF,
            _ => u64::MAX,
        };
        if (old & mask) == (expected & mask) {
            self.atomic_store(addr, new, size, MemoryOrder::SeqCst)?;
            Ok((old, true))
        } else {
            Ok((old, false))
        }
    }

    fn atomic_rmw(
        &mut self,
        addr: GuestAddr,
        op: AtomicOp,
        operand: u64,
        size: MemWidth,
        _order: MemoryOrder,
    ) -> Result<u64, MemoryError> {
        let old = self.atomic_load(addr, size, MemoryOrder::SeqCst)?;
        // Operate at the access width: mask both operands, and for signed
        // min/max sign-extend from `size` bits so e.g. AMOMAX.W compares the
        // 32-bit values as signed (not the zero-extended 64-bit values).
        let bits = size.bytes() * 8;
        let mask = if bits >= 64 {
            u64::MAX
        } else {
            (1u64 << bits) - 1
        };
        let o = operand & mask;
        let old_m = old & mask;
        let sext = |v: u64| -> i64 {
            if bits >= 64 {
                v as i64
            } else {
                ((v << (64 - bits)) as i64) >> (64 - bits)
            }
        };
        let new = match op {
            AtomicOp::Add => old_m.wrapping_add(o),
            AtomicOp::Sub => old_m.wrapping_sub(o),
            AtomicOp::And => old_m & o,
            AtomicOp::Or => old_m | o,
            AtomicOp::Xor => old_m ^ o,
            AtomicOp::Nand => !(old_m & o),
            AtomicOp::Max => std::cmp::max(sext(old_m), sext(o)) as u64,
            AtomicOp::Min => std::cmp::min(sext(old_m), sext(o)) as u64,
            AtomicOp::Umax => std::cmp::max(old_m, o),
            AtomicOp::Umin => std::cmp::min(old_m, o),
            AtomicOp::Swap => o,
        } & mask;
        self.atomic_store(addr, new, size, MemoryOrder::SeqCst)?;
        Ok(old_m)
    }

    fn load_exclusive(&mut self, addr: GuestAddr, size: MemWidth) -> Result<u64, MemoryError> {
        let val = self.atomic_load(addr, size, MemoryOrder::Acquire)?;
        self.exclusive_addr = Some(addr);
        self.exclusive_size = size;
        Ok(val)
    }

    fn store_exclusive(
        &mut self,
        addr: GuestAddr,
        value: u64,
        size: MemWidth,
    ) -> Result<bool, MemoryError> {
        let success = self.exclusive_addr == Some(addr) && self.exclusive_size == size;
        if success {
            self.atomic_store(addr, value, size, MemoryOrder::Release)?;
        }
        self.exclusive_addr = None;
        Ok(success)
    }

    fn clear_exclusive(&mut self) {
        self.exclusive_addr = None;
    }

    fn fence(&mut self, _kind: FenceKind) {
        // No-op for single-threaded interpreter
    }

    fn probe(&self, addr: GuestAddr, size: usize, _write: bool) -> Result<(), MemoryError> {
        if addr < self.base {
            return Err(MemoryError::OutOfBounds { addr });
        }
        let off = (addr - self.base) as usize;
        if off + size > self.data.len() {
            return Err(MemoryError::OutOfBounds {
                addr: addr + size as u64,
            });
        }
        Ok(())
    }
}

impl MemoryReader for FlatMemory {
    fn read(&self, addr: GuestAddr, size: usize) -> Result<Vec<u8>, MemoryError> {
        if addr < self.base {
            return Err(MemoryError::OutOfBounds { addr });
        }
        let off = (addr - self.base) as usize;
        if off + size > self.data.len() {
            return Err(MemoryError::OutOfBounds {
                addr: addr + size as u64,
            });
        }
        Ok(self.data[off..off + size].to_vec())
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check alignment for memory access
pub fn check_alignment(addr: GuestAddr, size: MemWidth, strict: bool) -> Result<(), MemoryError> {
    let required = if strict {
        size.bytes() as u64
    } else {
        (size.bytes() as u64).min(8)
    };

    if required > 1 && addr % required != 0 {
        Err(MemoryError::Alignment {
            addr,
            required: required as usize,
        })
    } else {
        Ok(())
    }
}

/// Convert bytes to u64 with specified endianness
pub fn bytes_to_u64(bytes: &[u8], endian: Endian) -> u64 {
    let mut arr = [0u8; 8];
    let len = bytes.len().min(8);
    arr[..len].copy_from_slice(&bytes[..len]);
    match endian {
        Endian::Little => u64::from_le_bytes(arr),
        Endian::Big => u64::from_be_bytes(arr),
    }
}

/// Convert u64 to bytes with specified endianness
pub fn u64_to_bytes(value: u64, endian: Endian) -> [u8; 8] {
    match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    }
}

// ============================================================================
// Exclusive Monitor
// ============================================================================

/// Exclusive monitor state for ARM LL/SC emulation
#[derive(Clone, Debug, Default)]
pub struct ExclusiveMonitor {
    /// Address being monitored (None if no exclusive)
    pub addr: Option<GuestAddr>,
    /// Size of exclusive region
    pub size: MemWidth,
    /// Value loaded (for comparison on store)
    pub value: u64,
}

impl ExclusiveMonitor {
    /// Create a new monitor
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark exclusive access (LDXR)
    pub fn mark_exclusive(&mut self, addr: GuestAddr, size: MemWidth, value: u64) {
        self.addr = Some(addr);
        self.size = size;
        self.value = value;
    }

    /// Check if exclusive is still valid for this address
    pub fn check_exclusive(&self, addr: GuestAddr, size: MemWidth) -> bool {
        match self.addr {
            Some(excl_addr) => excl_addr == addr && self.size == size,
            None => false,
        }
    }

    /// Clear exclusive monitor
    pub fn clear(&mut self) {
        self.addr = None;
    }

    /// Check if a store to this address should clear the monitor
    pub fn should_clear_for_store(&self, addr: GuestAddr, size: usize) -> bool {
        if let Some(excl_addr) = self.addr {
            let excl_end = excl_addr + self.size.bytes() as u64;
            let store_end = addr + size as u64;
            !(store_end <= excl_addr || addr >= excl_end)
        } else {
            false
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_memory() {
        let mut mem = FlatMemory::new(0x1000);

        // Write and read using SmirMemory trait
        SmirMemory::write(&mut mem, 0x100, &[1, 2, 3, 4]).unwrap();
        let mut buf = [0u8; 4];
        SmirMemory::read(&mut mem, 0x100, &mut buf).unwrap();
        assert_eq!(buf, [1, 2, 3, 4]);

        // Atomic operations
        mem.atomic_store(0x200, 42, MemWidth::B8, MemoryOrder::SeqCst)
            .unwrap();
        let val = mem
            .atomic_load(0x200, MemWidth::B8, MemoryOrder::SeqCst)
            .unwrap();
        assert_eq!(val, 42);

        // CAS
        let (old, success) = mem
            .compare_and_swap(
                0x200,
                42,
                100,
                MemWidth::B8,
                MemoryOrder::SeqCst,
                MemoryOrder::Relaxed,
            )
            .unwrap();
        assert_eq!(old, 42);
        assert!(success);

        let val = mem
            .atomic_load(0x200, MemWidth::B8, MemoryOrder::SeqCst)
            .unwrap();
        assert_eq!(val, 100);
    }

    #[test]
    fn test_exclusive_monitor() {
        let mut mem = FlatMemory::new(0x1000);

        // Load exclusive
        mem.atomic_store(0x100, 42, MemWidth::B8, MemoryOrder::SeqCst)
            .unwrap();
        let val = mem.load_exclusive(0x100, MemWidth::B8).unwrap();
        assert_eq!(val, 42);

        // Store exclusive should succeed
        let success = mem.store_exclusive(0x100, 100, MemWidth::B8).unwrap();
        assert!(success);

        // Store exclusive without load should fail
        let success = mem.store_exclusive(0x100, 200, MemWidth::B8).unwrap();
        assert!(!success);
    }

    #[test]
    fn test_alignment_check() {
        assert!(check_alignment(0x100, MemWidth::B4, true).is_ok());
        assert!(check_alignment(0x101, MemWidth::B4, true).is_err());
        assert!(check_alignment(0x101, MemWidth::B1, true).is_ok());
    }

    #[test]
    fn test_bytes_conversion() {
        let val = 0x0102030405060708u64;

        let le_bytes = u64_to_bytes(val, Endian::Little);
        assert_eq!(le_bytes, [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);

        let restored = bytes_to_u64(&le_bytes, Endian::Little);
        assert_eq!(restored, val);
    }
}
