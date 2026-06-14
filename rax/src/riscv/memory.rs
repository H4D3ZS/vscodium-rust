//! Memory abstraction for the RISC-V interpreter.
//!
//! All guest memory is little-endian (RISC-V is a little-endian architecture in
//! every configuration we model). The [`Memory`] trait provides byte-granular
//! read/write plus width-typed little-endian helpers; [`FlatMemory`] is a
//! contiguous backing store used by tests and the differential oracle.

use std::fmt::Debug;

/// Result of a memory access.
pub type MemResult<T> = Result<T, MemError>;

/// Reasons a memory access can fail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemError {
    /// Access fell outside any mapped region.
    OutOfBounds {
        /// Faulting address.
        addr: u64,
        /// Access size in bytes.
        size: usize,
    },
    /// Access violated alignment requirements of the backing device.
    Misaligned {
        /// Faulting address.
        addr: u64,
        /// Access size in bytes.
        size: usize,
    },
}

/// Backend-agnostic guest memory interface.
///
/// Implementors only need to provide [`read`](Memory::read) and
/// [`write`](Memory::write); the little-endian width helpers are derived.
/// The `Send` bound lets a [`RiscVCpu`](crate::riscv::RiscVCpu) be moved across
/// threads (required by the VMM's `VCpu` interface).
pub trait Memory: Debug + Send {
    /// Read exactly `buf.len()` bytes starting at `addr`.
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()>;

    /// Write `data` starting at `addr`.
    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()>;

    /// Read an unsigned byte.
    #[inline]
    fn read_u8(&self, addr: u64) -> MemResult<u8> {
        let mut b = [0u8; 1];
        self.read(addr, &mut b)?;
        Ok(b[0])
    }

    /// Read a little-endian halfword.
    #[inline]
    fn read_u16(&self, addr: u64) -> MemResult<u16> {
        let mut b = [0u8; 2];
        self.read(addr, &mut b)?;
        Ok(u16::from_le_bytes(b))
    }

    /// Read a little-endian word.
    #[inline]
    fn read_u32(&self, addr: u64) -> MemResult<u32> {
        let mut b = [0u8; 4];
        self.read(addr, &mut b)?;
        Ok(u32::from_le_bytes(b))
    }

    /// Read a little-endian doubleword.
    #[inline]
    fn read_u64(&self, addr: u64) -> MemResult<u64> {
        let mut b = [0u8; 8];
        self.read(addr, &mut b)?;
        Ok(u64::from_le_bytes(b))
    }

    /// Write a byte.
    #[inline]
    fn write_u8(&mut self, addr: u64, value: u8) -> MemResult<()> {
        self.write(addr, &[value])
    }

    /// Write a little-endian halfword.
    #[inline]
    fn write_u16(&mut self, addr: u64, value: u16) -> MemResult<()> {
        self.write(addr, &value.to_le_bytes())
    }

    /// Write a little-endian word.
    #[inline]
    fn write_u32(&mut self, addr: u64, value: u32) -> MemResult<()> {
        self.write(addr, &value.to_le_bytes())
    }

    /// Write a little-endian doubleword.
    #[inline]
    fn write_u64(&mut self, addr: u64, value: u64) -> MemResult<()> {
        self.write(addr, &value.to_le_bytes())
    }
}

/// Contiguous, flat backing store covering `[base, base + size)`.
#[derive(Clone)]
pub struct FlatMemory {
    data: Vec<u8>,
    base: u64,
}

impl Debug for FlatMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlatMemory")
            .field("base", &self.base)
            .field("size", &self.data.len())
            .finish()
    }
}

impl FlatMemory {
    /// Create zeroed memory of `size` bytes mapped at `base`.
    pub fn new(base: u64, size: usize) -> Self {
        FlatMemory {
            data: vec![0u8; size],
            base,
        }
    }

    /// Create memory initialized with `data` mapped at `base`.
    pub fn with_data(base: u64, data: Vec<u8>) -> Self {
        FlatMemory { data, base }
    }

    /// Base address of the mapping.
    pub fn base(&self) -> u64 {
        self.base
    }

    /// Size of the mapping in bytes.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Raw backing slice.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Mutable raw backing slice.
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    #[inline]
    fn offset(&self, addr: u64, size: usize) -> MemResult<usize> {
        if addr < self.base {
            return Err(MemError::OutOfBounds { addr, size });
        }
        let off = (addr - self.base) as usize;
        match off.checked_add(size) {
            Some(end) if end <= self.data.len() => Ok(off),
            _ => Err(MemError::OutOfBounds { addr, size }),
        }
    }
}

impl Memory for FlatMemory {
    #[inline]
    fn read(&self, addr: u64, buf: &mut [u8]) -> MemResult<()> {
        let off = self.offset(addr, buf.len())?;
        buf.copy_from_slice(&self.data[off..off + buf.len()]);
        Ok(())
    }

    #[inline]
    fn write(&mut self, addr: u64, data: &[u8]) -> MemResult<()> {
        let off = self.offset(addr, data.len())?;
        self.data[off..off + data.len()].copy_from_slice(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_roundtrip_widths() {
        let mut m = FlatMemory::new(0x1000, 0x1000);
        m.write_u64(0x1000, 0x0123_4567_89ab_cdef).unwrap();
        assert_eq!(m.read_u64(0x1000).unwrap(), 0x0123_4567_89ab_cdef);
        assert_eq!(m.read_u32(0x1000).unwrap(), 0x89ab_cdef);
        assert_eq!(m.read_u16(0x1000).unwrap(), 0xcdef);
        assert_eq!(m.read_u8(0x1000).unwrap(), 0xef);
        // little-endian byte order
        assert_eq!(m.read_u8(0x1007).unwrap(), 0x01);
    }

    #[test]
    fn out_of_bounds() {
        let m = FlatMemory::new(0, 16);
        assert!(m.read_u32(14).is_err());
        assert!(m.read_u8(16).is_err());
        assert!(m.read_u8(15).is_ok());
    }
}
