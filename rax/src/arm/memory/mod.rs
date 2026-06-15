//! Production-grade ARM memory subsystem.
//!
//! This module provides a comprehensive memory model for ARM emulation including:
//! - Memory Protection Unit (MPU) for Cortex-M/R
//! - Memory-Mapped I/O (MMIO) support
//! - Exclusive monitor for LDREX/STREX
//! - Memory attributes and cacheability
//! - Alignment checking
//! - Endianness handling

pub mod mpu;
pub mod regions;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::arm::cpu_trait::{AccessType, ArmError, MemoryFaultInfo, MemoryFaultType};

pub use mpu::{Mpu, MpuRegion, MpuRegionAttr, MpuType};
pub use regions::{AccessPermission, MemoryAttributes, MemoryRegion, MemoryRegionType};

/// Memory access result.
pub type MemResult<T> = Result<T, MemoryError>;

/// Memory error types.
#[derive(Clone, Debug)]
pub enum MemoryError {
    /// Address is outside valid memory.
    OutOfBounds { addr: u64, size: usize },
    /// Alignment error.
    Alignment { addr: u64, required: usize },
    /// Permission denied.
    Permission {
        addr: u64,
        access: AccessType,
        reason: String,
    },
    /// MPU fault.
    MpuFault { addr: u64, access: AccessType },
    /// Bus error (external abort).
    BusError { addr: u64 },
    /// Device memory access error.
    DeviceError { addr: u64, message: String },
    /// Exclusive monitor failure.
    ExclusiveFailed { addr: u64 },
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::OutOfBounds { addr, size } => {
                write!(
                    f,
                    "memory access out of bounds at 0x{:x} (size {})",
                    addr, size
                )
            }
            MemoryError::Alignment { addr, required } => {
                write!(
                    f,
                    "unaligned access at 0x{:x} (required {} byte alignment)",
                    addr, required
                )
            }
            MemoryError::Permission {
                addr,
                access,
                reason,
            } => {
                write!(
                    f,
                    "permission denied for {:?} at 0x{:x}: {}",
                    access, addr, reason
                )
            }
            MemoryError::MpuFault { addr, access } => {
                write!(f, "MPU fault for {:?} at 0x{:x}", access, addr)
            }
            MemoryError::BusError { addr } => {
                write!(f, "bus error at 0x{:x}", addr)
            }
            MemoryError::DeviceError { addr, message } => {
                write!(f, "device error at 0x{:x}: {}", addr, message)
            }
            MemoryError::ExclusiveFailed { addr } => {
                write!(f, "exclusive access failed at 0x{:x}", addr)
            }
        }
    }
}

impl std::error::Error for MemoryError {}

impl From<MemoryError> for ArmError {
    fn from(e: MemoryError) -> Self {
        let (addr, access, fault_type) = match &e {
            MemoryError::OutOfBounds { addr, .. } => {
                (*addr, AccessType::Read, MemoryFaultType::Translation)
            }
            MemoryError::Alignment { addr, .. } => {
                (*addr, AccessType::Read, MemoryFaultType::Alignment)
            }
            MemoryError::Permission { addr, access, .. } => {
                (*addr, *access, MemoryFaultType::Permission)
            }
            MemoryError::MpuFault { addr, access } => (*addr, *access, MemoryFaultType::Permission),
            MemoryError::BusError { addr } => (*addr, AccessType::Read, MemoryFaultType::External),
            MemoryError::DeviceError { addr, .. } => {
                (*addr, AccessType::Read, MemoryFaultType::External)
            }
            MemoryError::ExclusiveFailed { addr } => (
                *addr,
                AccessType::Atomic,
                MemoryFaultType::UnsupportedAtomic,
            ),
        };
        ArmError::MemoryError(MemoryFaultInfo {
            address: addr,
            access,
            fault_type,
            stage2: false,
        })
    }
}

/// MMIO handler trait for memory-mapped peripherals.
pub trait MmioHandler: Send + Sync + Debug {
    /// Read from the peripheral.
    fn read(&mut self, offset: u64, size: usize) -> MemResult<u64>;

    /// Write to the peripheral.
    fn write(&mut self, offset: u64, size: usize, value: u64) -> MemResult<()>;

    /// Get the name of this handler (for debugging).
    fn name(&self) -> &str;
}

/// Registered MMIO region.
#[derive(Debug)]
struct MmioRegion {
    base: u64,
    size: u64,
    handler: Box<dyn MmioHandler>,
}

/// Exclusive monitor state for LDREX/STREX.
#[derive(Clone, Debug, Default)]
pub struct ExclusiveMonitor {
    /// Currently monitored address (None if not monitoring).
    address: Option<u64>,
    /// Size of monitored region.
    size: u8,
    /// Process ID / context for multiprocessor support.
    context_id: u32,
}

impl ExclusiveMonitor {
    /// Create a new exclusive monitor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark an address as exclusively accessed.
    pub fn mark_exclusive(&mut self, addr: u64, size: u8, context_id: u32) {
        self.address = Some(addr & !((size as u64) - 1)); // Align to size
        self.size = size;
        self.context_id = context_id;
    }

    /// Check and clear exclusive access.
    /// Returns true if the address was exclusively held by this context.
    pub fn check_and_clear(&mut self, addr: u64, size: u8, context_id: u32) -> bool {
        let aligned_addr = addr & !((size as u64) - 1);
        if self.address == Some(aligned_addr) && self.size == size && self.context_id == context_id
        {
            self.clear();
            true
        } else {
            self.clear();
            false
        }
    }

    /// Clear the exclusive monitor.
    pub fn clear(&mut self) {
        self.address = None;
    }

    /// Check if an address is being monitored.
    pub fn is_exclusive(&self, addr: u64, size: u8) -> bool {
        let aligned_addr = addr & !((size as u64) - 1);
        self.address == Some(aligned_addr) && self.size == size
    }
}

/// Memory barrier types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierKind {
    /// Full system barrier (all memory, all observers)
    FullSystem,
    /// Outer shareable barrier
    OuterShareable,
    /// Inner shareable barrier
    InnerShareable,
    /// Non-shareable barrier
    NonShareable,
    /// Load barrier (DMB LD)
    Load,
    /// Store barrier (DMB ST)
    Store,
}

/// Production-grade ARM memory interface.
pub trait ArmMemory: Send + Sync + Debug {
    // =========================================================================
    // Basic Access
    // =========================================================================

    /// Read bytes from memory.
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()>;

    /// Write bytes to memory.
    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()>;

    // =========================================================================
    // Typed Access with Alignment
    // =========================================================================

    /// Read an 8-bit value.
    fn read_u8(&self, addr: u64) -> MemResult<u8> {
        let mut buf = [0u8; 1];
        self.read(addr, &mut buf)?;
        Ok(buf[0])
    }

    /// Read a 16-bit value (with alignment check).
    fn read_u16(&self, addr: u64) -> MemResult<u16> {
        if self.requires_alignment() && (addr & 1) != 0 {
            return Err(MemoryError::Alignment { addr, required: 2 });
        }
        let mut buf = [0u8; 2];
        self.read(addr, &mut buf)?;
        Ok(if self.is_big_endian() {
            u16::from_be_bytes(buf)
        } else {
            u16::from_le_bytes(buf)
        })
    }

    /// Read a 32-bit value (with alignment check).
    fn read_u32(&self, addr: u64) -> MemResult<u32> {
        if self.requires_alignment() && (addr & 3) != 0 {
            return Err(MemoryError::Alignment { addr, required: 4 });
        }
        let mut buf = [0u8; 4];
        self.read(addr, &mut buf)?;
        Ok(if self.is_big_endian() {
            u32::from_be_bytes(buf)
        } else {
            u32::from_le_bytes(buf)
        })
    }

    /// Read a 64-bit value (with alignment check).
    fn read_u64(&self, addr: u64) -> MemResult<u64> {
        if self.requires_alignment() && (addr & 7) != 0 {
            return Err(MemoryError::Alignment { addr, required: 8 });
        }
        let mut buf = [0u8; 8];
        self.read(addr, &mut buf)?;
        Ok(if self.is_big_endian() {
            u64::from_be_bytes(buf)
        } else {
            u64::from_le_bytes(buf)
        })
    }

    /// Write an 8-bit value.
    fn write_u8(&mut self, addr: u64, value: u8) -> MemResult<()> {
        self.write(addr, &[value])
    }

    /// Write a 16-bit value (with alignment check).
    fn write_u16(&mut self, addr: u64, value: u16) -> MemResult<()> {
        if self.requires_alignment() && (addr & 1) != 0 {
            return Err(MemoryError::Alignment { addr, required: 2 });
        }
        let buf = if self.is_big_endian() {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write(addr, &buf)
    }

    /// Write a 32-bit value (with alignment check).
    fn write_u32(&mut self, addr: u64, value: u32) -> MemResult<()> {
        if self.requires_alignment() && (addr & 3) != 0 {
            return Err(MemoryError::Alignment { addr, required: 4 });
        }
        let buf = if self.is_big_endian() {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write(addr, &buf)
    }

    /// Write a 64-bit value (with alignment check).
    fn write_u64(&mut self, addr: u64, value: u64) -> MemResult<()> {
        if self.requires_alignment() && (addr & 7) != 0 {
            return Err(MemoryError::Alignment { addr, required: 8 });
        }
        let buf = if self.is_big_endian() {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        self.write(addr, &buf)
    }

    // =========================================================================
    // Exclusive Access (LDREX/STREX, LDXR/STXR)
    // =========================================================================

    /// Read with exclusive access.
    fn read_exclusive(&mut self, addr: u64, size: usize) -> MemResult<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read(addr, &mut buf)?;
        self.mark_exclusive(addr, size as u8);
        Ok(buf)
    }

    /// Write with exclusive access check.
    /// Returns true if the write succeeded (exclusive was held).
    fn write_exclusive(&mut self, addr: u64, data: &[u8]) -> MemResult<bool> {
        if self.check_exclusive(addr, data.len() as u8) {
            self.write(addr, data)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Mark an address as exclusive.
    fn mark_exclusive(&mut self, addr: u64, size: u8);

    /// Check if exclusive access is held and clear it.
    fn check_exclusive(&mut self, addr: u64, size: u8) -> bool;

    /// Clear all exclusive monitors.
    fn clear_exclusive(&mut self);

    // =========================================================================
    // Instruction Fetch
    // =========================================================================

    /// Fetch instructions from memory.
    /// This may have different permissions than data access.
    fn fetch(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        self.read(addr, buf)
    }

    /// Fetch a 16-bit instruction (Thumb).
    fn fetch_u16(&self, addr: u64) -> MemResult<u16> {
        let mut buf = [0u8; 2];
        self.fetch(addr, &mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Fetch a 32-bit instruction (ARM/Thumb-2).
    fn fetch_u32(&self, addr: u64) -> MemResult<u32> {
        let mut buf = [0u8; 4];
        self.fetch(addr, &mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    // =========================================================================
    // Memory Attributes
    // =========================================================================

    /// Get memory attributes for an address.
    fn attributes(&self, addr: u64) -> MemoryAttributes {
        let _ = addr;
        MemoryAttributes::default()
    }

    /// Check if memory requires alignment.
    fn requires_alignment(&self) -> bool {
        true
    }

    /// Check if memory is big endian.
    fn is_big_endian(&self) -> bool {
        false
    }

    // =========================================================================
    // Barriers
    // =========================================================================

    /// Execute a data memory barrier.
    fn data_barrier(&mut self, _kind: BarrierKind) {
        // Default: no-op in single-core emulation
    }

    /// Execute an instruction synchronization barrier.
    fn instruction_barrier(&mut self) {
        // Default: no-op in single-core emulation
    }

    // =========================================================================
    // MMIO
    // =========================================================================

    /// Register an MMIO handler.
    fn register_mmio(&mut self, base: u64, size: u64, handler: Box<dyn MmioHandler>);

    /// Unregister an MMIO handler.
    fn unregister_mmio(&mut self, base: u64);

    // =========================================================================
    // MPU
    // =========================================================================

    /// Get MPU reference (if available).
    fn mpu(&self) -> Option<&Mpu> {
        None
    }

    /// Get mutable MPU reference (if available).
    fn mpu_mut(&mut self) -> Option<&mut Mpu> {
        None
    }

    /// Check MPU permissions for an access.
    fn check_mpu(&self, addr: u64, size: usize, access: AccessType) -> MemResult<()> {
        let _ = (addr, size, access);
        Ok(()) // Default: no MPU
    }
}

/// Standard memory implementation for ARM emulation.
#[derive(Debug)]
pub struct StandardMemory {
    /// RAM regions.
    ram: BTreeMap<u64, Vec<u8>>,
    /// MMIO regions.
    mmio: Vec<MmioRegion>,
    /// Exclusive monitor.
    exclusive: ExclusiveMonitor,
    /// MPU (optional).
    mpu: Option<Mpu>,
    /// Require alignment for accesses.
    require_alignment: bool,
    /// Big endian mode.
    big_endian: bool,
    /// MPU enabled.
    mpu_enabled: bool,
    /// Privileged mode (for MPU checks).
    privileged: bool,
}

impl StandardMemory {
    /// Create a new standard memory with no regions.
    pub fn new() -> Self {
        Self {
            ram: BTreeMap::new(),
            mmio: Vec::new(),
            exclusive: ExclusiveMonitor::new(),
            mpu: None,
            require_alignment: true,
            big_endian: false,
            mpu_enabled: false,
            privileged: true,
        }
    }

    /// Create memory with a single RAM region.
    pub fn with_ram(base: u64, size: usize) -> Self {
        let mut mem = Self::new();
        mem.add_ram(base, size);
        mem
    }

    /// Add a RAM region.
    pub fn add_ram(&mut self, base: u64, size: usize) {
        self.ram.insert(base, vec![0u8; size]);
    }

    /// Add RAM with initial contents.
    pub fn add_ram_with_data(&mut self, base: u64, data: Vec<u8>) {
        self.ram.insert(base, data);
    }

    /// Initialize MPU.
    pub fn init_mpu(&mut self, mpu_type: MpuType) {
        self.mpu = Some(Mpu::new(mpu_type));
    }

    /// Enable/disable MPU.
    pub fn set_mpu_enabled(&mut self, enabled: bool) {
        self.mpu_enabled = enabled;
    }

    /// Set privileged mode for MPU checks.
    pub fn set_privileged(&mut self, privileged: bool) {
        self.privileged = privileged;
    }

    /// Set alignment requirement.
    pub fn set_require_alignment(&mut self, require: bool) {
        self.require_alignment = require;
    }

    /// Set endianness.
    pub fn set_big_endian(&mut self, big_endian: bool) {
        self.big_endian = big_endian;
    }

    /// Find RAM region containing address.
    fn find_ram(&self, addr: u64) -> Option<(&u64, &Vec<u8>)> {
        self.ram
            .range(..=addr)
            .next_back()
            .and_then(|(base, data)| {
                if addr >= *base && addr < base + data.len() as u64 {
                    Some((base, data))
                } else {
                    None
                }
            })
    }

    /// Find mutable RAM region containing address.
    fn find_ram_mut(&mut self, addr: u64) -> Option<(u64, &mut Vec<u8>)> {
        let base = self
            .ram
            .range(..=addr)
            .next_back()
            .and_then(|(base, data)| {
                if addr >= *base && addr < base + data.len() as u64 {
                    Some(*base)
                } else {
                    None
                }
            })?;
        self.ram.get_mut(&base).map(|data| (base, data))
    }

    /// Find MMIO handler for address.
    fn find_mmio(&self, addr: u64) -> Option<usize> {
        self.mmio
            .iter()
            .position(|r| addr >= r.base && addr < r.base + r.size)
    }

    /// Perform MPU check if enabled.
    fn do_mpu_check(&self, addr: u64, size: usize, access: AccessType) -> MemResult<()> {
        if !self.mpu_enabled {
            return Ok(());
        }

        if let Some(mpu) = &self.mpu {
            mpu.check_access(addr as u32, size as u32, access, self.privileged)?;
        }

        Ok(())
    }
}

impl Default for StandardMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl ArmMemory for StandardMemory {
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        // MPU check
        self.do_mpu_check(addr, buf.len(), AccessType::Read)?;

        // Try RAM first
        if let Some((base, data)) = self.find_ram(addr) {
            let offset = (addr - base) as usize;
            if offset + buf.len() <= data.len() {
                buf.copy_from_slice(&data[offset..offset + buf.len()]);
                return Ok(());
            }
        }

        // Check if it's an MMIO region - need mutable access for handlers
        // For read-only trait method, we can't call MMIO handlers
        // Return bus error for unmapped addresses
        Err(MemoryError::OutOfBounds {
            addr,
            size: buf.len(),
        })
    }

    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()> {
        // MPU check
        self.do_mpu_check(addr, data.len(), AccessType::Write)?;

        // Try RAM first
        if let Some((base, ram_data)) = self.find_ram_mut(addr) {
            let offset = (addr - base) as usize;
            if offset + data.len() <= ram_data.len() {
                ram_data[offset..offset + data.len()].copy_from_slice(data);
                return Ok(());
            }
        }

        // Try MMIO
        if let Some(idx) = self.find_mmio(addr) {
            let region = &mut self.mmio[idx];
            let offset = addr - region.base;
            // Write each byte
            for (i, &byte) in data.iter().enumerate() {
                region.handler.write(offset + i as u64, 1, byte as u64)?;
            }
            return Ok(());
        }

        Err(MemoryError::OutOfBounds {
            addr,
            size: data.len(),
        })
    }

    fn mark_exclusive(&mut self, addr: u64, size: u8) {
        self.exclusive.mark_exclusive(addr, size, 0);
    }

    fn check_exclusive(&mut self, addr: u64, size: u8) -> bool {
        self.exclusive.check_and_clear(addr, size, 0)
    }

    fn clear_exclusive(&mut self) {
        self.exclusive.clear();
    }

    fn requires_alignment(&self) -> bool {
        self.require_alignment
    }

    fn is_big_endian(&self) -> bool {
        self.big_endian
    }

    fn register_mmio(&mut self, base: u64, size: u64, handler: Box<dyn MmioHandler>) {
        // Remove any existing handler at this base
        self.mmio.retain(|r| r.base != base);
        self.mmio.push(MmioRegion {
            base,
            size,
            handler,
        });
    }

    fn unregister_mmio(&mut self, base: u64) {
        self.mmio.retain(|r| r.base != base);
    }

    fn mpu(&self) -> Option<&Mpu> {
        self.mpu.as_ref()
    }

    fn mpu_mut(&mut self) -> Option<&mut Mpu> {
        self.mpu.as_mut()
    }

    fn check_mpu(&self, addr: u64, size: usize, access: AccessType) -> MemResult<()> {
        self.do_mpu_check(addr, size, access)
    }
}

/// Simple flat memory for testing (no MPU, no MMIO).
#[derive(Debug)]
pub struct FlatMemory {
    data: Vec<u8>,
    base: u64,
    exclusive: ExclusiveMonitor,
}

impl FlatMemory {
    /// Create flat memory with given size at base address.
    pub fn new(base: u64, size: usize) -> Self {
        Self {
            data: vec![0u8; size],
            base,
            exclusive: ExclusiveMonitor::new(),
        }
    }

    /// Create flat memory with initial data.
    pub fn with_data(base: u64, data: Vec<u8>) -> Self {
        Self {
            data,
            base,
            exclusive: ExclusiveMonitor::new(),
        }
    }

    /// Get the size of the memory.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Get the base address.
    pub fn base(&self) -> u64 {
        self.base
    }

    /// Get raw data slice.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable raw data slice.
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    fn translate(&self, addr: u64, size: usize) -> MemResult<usize> {
        if addr < self.base {
            return Err(MemoryError::OutOfBounds { addr, size });
        }
        let offset = (addr - self.base) as usize;
        // checked_add: a huge synthetic address must fault, not overflow the
        // bounds check and panic on the slice index.
        if offset
            .checked_add(size)
            .map(|end| end > self.data.len())
            .unwrap_or(true)
        {
            return Err(MemoryError::OutOfBounds { addr, size });
        }
        Ok(offset)
    }
}

impl ArmMemory for FlatMemory {
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        let offset = self.translate(addr, buf.len())?;
        buf.copy_from_slice(&self.data[offset..offset + buf.len()]);
        Ok(())
    }

    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()> {
        let offset = self.translate(addr, data.len())?;
        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn mark_exclusive(&mut self, addr: u64, size: u8) {
        self.exclusive.mark_exclusive(addr, size, 0);
    }

    fn check_exclusive(&mut self, addr: u64, size: u8) -> bool {
        self.exclusive.check_and_clear(addr, size, 0)
    }

    fn clear_exclusive(&mut self) {
        self.exclusive.clear();
    }

    fn requires_alignment(&self) -> bool {
        false // Flat memory allows unaligned access
    }

    fn is_big_endian(&self) -> bool {
        false
    }

    fn register_mmio(&mut self, _base: u64, _size: u64, _handler: Box<dyn MmioHandler>) {
        // Flat memory doesn't support MMIO
    }

    fn unregister_mmio(&mut self, _base: u64) {
        // Flat memory doesn't support MMIO
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_memory_basic() {
        let mut mem = FlatMemory::new(0x2000_0000, 0x1000);

        // Write and read
        mem.write_u32(0x2000_0000, 0xDEADBEEF).unwrap();
        assert_eq!(mem.read_u32(0x2000_0000).unwrap(), 0xDEADBEEF);

        // Byte access
        mem.write_u8(0x2000_0100, 0x42).unwrap();
        assert_eq!(mem.read_u8(0x2000_0100).unwrap(), 0x42);

        // Out of bounds
        assert!(mem.read_u32(0x2000_1000).is_err());
        assert!(mem.read_u32(0x1FFF_FFFF).is_err());
    }

    #[test]
    fn test_exclusive_monitor() {
        let mut monitor = ExclusiveMonitor::new();

        // Mark exclusive
        monitor.mark_exclusive(0x1000, 4, 0);
        assert!(monitor.is_exclusive(0x1000, 4));

        // Check and clear - should succeed
        assert!(monitor.check_and_clear(0x1000, 4, 0));
        assert!(!monitor.is_exclusive(0x1000, 4));

        // Check again - should fail (already cleared)
        monitor.mark_exclusive(0x1000, 4, 0);
        assert!(monitor.check_and_clear(0x1000, 4, 0));
        assert!(!monitor.check_and_clear(0x1000, 4, 0));
    }

    #[test]
    fn test_exclusive_memory() {
        let mut mem = FlatMemory::new(0, 0x1000);

        // LDREX/STREX pattern
        let data = mem.read_exclusive(0x100, 4).unwrap();
        assert_eq!(data, vec![0, 0, 0, 0]);

        // STREX should succeed
        assert!(mem.write_exclusive(0x100, &[1, 2, 3, 4]).unwrap());

        // Second STREX should fail
        assert!(!mem.write_exclusive(0x100, &[5, 6, 7, 8]).unwrap());

        // Verify first write took effect
        assert_eq!(mem.read_u32(0x100).unwrap(), 0x04030201);
    }

    #[test]
    fn test_standard_memory() {
        let mut mem = StandardMemory::with_ram(0x2000_0000, 0x10000);

        // Basic access
        mem.write_u32(0x2000_0000, 0x12345678).unwrap();
        assert_eq!(mem.read_u32(0x2000_0000).unwrap(), 0x12345678);

        // Add another region
        mem.add_ram(0x0800_0000, 0x1000);
        mem.write_u32(0x0800_0000, 0xABCD1234).unwrap();
        assert_eq!(mem.read_u32(0x0800_0000).unwrap(), 0xABCD1234);
    }
}
