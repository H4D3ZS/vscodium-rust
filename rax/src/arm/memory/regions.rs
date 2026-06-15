//! Memory region definitions and attributes.
//!
//! This module defines memory regions, their types, and attributes
//! following the ARM architecture memory model.

use std::fmt;

/// Memory region type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryRegionType {
    /// Normal memory (RAM/ROM).
    Normal,
    /// Device memory (MMIO).
    Device,
    /// Strongly-ordered memory.
    StronglyOrdered,
}

impl Default for MemoryRegionType {
    fn default() -> Self {
        MemoryRegionType::Normal
    }
}

/// Memory cacheability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Cacheability {
    /// Non-cacheable.
    #[default]
    NonCacheable,
    /// Write-through, no write-allocate.
    WriteThrough,
    /// Write-back, no write-allocate.
    WriteBackNoAllocate,
    /// Write-back, write-allocate.
    WriteBackAllocate,
}

/// Memory shareability domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Shareability {
    /// Non-shareable (private to this PE).
    #[default]
    NonShareable,
    /// Inner shareable (shared within inner domain).
    InnerShareable,
    /// Outer shareable (shared within outer domain).
    OuterShareable,
}

/// Device memory ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DeviceType {
    /// Device-nGnRnE (non-Gathering, non-Reordering, no Early Write Acknowledgement).
    #[default]
    DeviceNGnRnE,
    /// Device-nGnRE (non-Gathering, non-Reordering, Early Write Acknowledgement).
    DeviceNGnRE,
    /// Device-nGRE (non-Gathering, Reordering, Early Write Acknowledgement).
    DeviceNGRE,
    /// Device-GRE (Gathering, Reordering, Early Write Acknowledgement).
    DeviceGRE,
}

/// Memory attributes for a region.
#[derive(Clone, Copy, Debug, Default)]
pub struct MemoryAttributes {
    /// Memory type.
    pub mem_type: MemoryRegionType,
    /// Inner cacheability (for Normal memory).
    pub inner_cache: Cacheability,
    /// Outer cacheability (for Normal memory).
    pub outer_cache: Cacheability,
    /// Shareability.
    pub shareability: Shareability,
    /// Device type (for Device memory).
    pub device_type: DeviceType,
    /// Execute-never (XN).
    pub execute_never: bool,
    /// Privileged execute-never (PXN).
    pub privileged_execute_never: bool,
}

impl MemoryAttributes {
    /// Create attributes for normal cacheable memory.
    pub fn normal_cached() -> Self {
        Self {
            mem_type: MemoryRegionType::Normal,
            inner_cache: Cacheability::WriteBackAllocate,
            outer_cache: Cacheability::WriteBackAllocate,
            shareability: Shareability::InnerShareable,
            ..Default::default()
        }
    }

    /// Create attributes for normal non-cacheable memory.
    pub fn normal_non_cached() -> Self {
        Self {
            mem_type: MemoryRegionType::Normal,
            inner_cache: Cacheability::NonCacheable,
            outer_cache: Cacheability::NonCacheable,
            shareability: Shareability::NonShareable,
            ..Default::default()
        }
    }

    /// Create attributes for device memory.
    pub fn device() -> Self {
        Self {
            mem_type: MemoryRegionType::Device,
            device_type: DeviceType::DeviceNGnRnE,
            execute_never: true,
            ..Default::default()
        }
    }

    /// Create attributes for strongly-ordered memory.
    pub fn strongly_ordered() -> Self {
        Self {
            mem_type: MemoryRegionType::StronglyOrdered,
            execute_never: true,
            ..Default::default()
        }
    }

    /// Check if this is device or strongly-ordered memory.
    pub fn is_device_or_strongly_ordered(&self) -> bool {
        matches!(
            self.mem_type,
            MemoryRegionType::Device | MemoryRegionType::StronglyOrdered
        )
    }

    /// Check if memory is cacheable.
    pub fn is_cacheable(&self) -> bool {
        self.mem_type == MemoryRegionType::Normal
            && (self.inner_cache != Cacheability::NonCacheable
                || self.outer_cache != Cacheability::NonCacheable)
    }
}

/// Access permissions for a memory region.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccessPermission {
    /// Privileged read access.
    pub priv_read: bool,
    /// Privileged write access.
    pub priv_write: bool,
    /// Unprivileged read access.
    pub unpriv_read: bool,
    /// Unprivileged write access.
    pub unpriv_write: bool,
}

impl AccessPermission {
    /// No access.
    pub const NONE: Self = Self {
        priv_read: false,
        priv_write: false,
        unpriv_read: false,
        unpriv_write: false,
    };

    /// Privileged read-only.
    pub const PRIV_RO: Self = Self {
        priv_read: true,
        priv_write: false,
        unpriv_read: false,
        unpriv_write: false,
    };

    /// Privileged read-write.
    pub const PRIV_RW: Self = Self {
        priv_read: true,
        priv_write: true,
        unpriv_read: false,
        unpriv_write: false,
    };

    /// Privileged read-write, unprivileged read-only.
    pub const PRIV_RW_UNPRIV_RO: Self = Self {
        priv_read: true,
        priv_write: true,
        unpriv_read: true,
        unpriv_write: false,
    };

    /// Full access (privileged and unprivileged read-write).
    pub const FULL_ACCESS: Self = Self {
        priv_read: true,
        priv_write: true,
        unpriv_read: true,
        unpriv_write: true,
    };

    /// Read-only for both privileged and unprivileged.
    pub const RO: Self = Self {
        priv_read: true,
        priv_write: false,
        unpriv_read: true,
        unpriv_write: false,
    };

    /// Check if read access is allowed.
    pub fn can_read(&self, privileged: bool) -> bool {
        if privileged {
            self.priv_read
        } else {
            self.unpriv_read
        }
    }

    /// Check if write access is allowed.
    pub fn can_write(&self, privileged: bool) -> bool {
        if privileged {
            self.priv_write
        } else {
            self.unpriv_write
        }
    }

    /// Create from ARMv7-M AP bits (3-bit encoding).
    pub fn from_ap_bits_v7m(ap: u8) -> Self {
        match ap & 0x7 {
            0b000 => Self::NONE,
            0b001 => Self::PRIV_RW,
            0b010 => Self::PRIV_RW_UNPRIV_RO,
            0b011 => Self::FULL_ACCESS,
            0b100 => Self::NONE, // Reserved, treated as no access
            0b101 => Self::PRIV_RO,
            0b110 => Self::RO,
            0b111 => Self::RO, // Reserved, treated as RO
            _ => unreachable!(),
        }
    }

    /// Create from ARMv8-M AP bits (2-bit encoding with separate RO).
    pub fn from_ap_bits_v8m(ap: u8, ro: bool) -> Self {
        let base = match ap & 0x3 {
            0b00 => Self::PRIV_RW,
            0b01 => Self::FULL_ACCESS,
            0b10 => Self::PRIV_RO,
            0b11 => Self::RO,
            _ => unreachable!(),
        };
        if ro {
            Self {
                priv_write: false,
                unpriv_write: false,
                ..base
            }
        } else {
            base
        }
    }
}

impl Default for AccessPermission {
    fn default() -> Self {
        Self::FULL_ACCESS
    }
}

/// A memory region definition.
#[derive(Clone, Debug)]
pub struct MemoryRegion {
    /// Base address.
    pub base: u64,
    /// Size in bytes.
    pub size: u64,
    /// Region name (for debugging).
    pub name: String,
    /// Memory attributes.
    pub attributes: MemoryAttributes,
    /// Access permissions.
    pub permissions: AccessPermission,
    /// Region enabled.
    pub enabled: bool,
}

impl MemoryRegion {
    /// Create a new memory region.
    pub fn new(base: u64, size: u64, name: impl Into<String>) -> Self {
        Self {
            base,
            size,
            name: name.into(),
            attributes: MemoryAttributes::default(),
            permissions: AccessPermission::default(),
            enabled: true,
        }
    }

    /// Create a RAM region.
    pub fn ram(base: u64, size: u64, name: impl Into<String>) -> Self {
        Self {
            base,
            size,
            name: name.into(),
            attributes: MemoryAttributes::normal_cached(),
            permissions: AccessPermission::FULL_ACCESS,
            enabled: true,
        }
    }

    /// Create a ROM region.
    pub fn rom(base: u64, size: u64, name: impl Into<String>) -> Self {
        Self {
            base,
            size,
            name: name.into(),
            attributes: MemoryAttributes::normal_cached(),
            permissions: AccessPermission::RO,
            enabled: true,
        }
    }

    /// Create a device/peripheral region.
    pub fn device(base: u64, size: u64, name: impl Into<String>) -> Self {
        Self {
            base,
            size,
            name: name.into(),
            attributes: MemoryAttributes::device(),
            permissions: AccessPermission::PRIV_RW,
            enabled: true,
        }
    }

    /// Check if an address falls within this region.
    pub fn contains(&self, addr: u64) -> bool {
        self.enabled && addr >= self.base && addr < self.base + self.size
    }

    /// Check if a range overlaps with this region.
    pub fn overlaps(&self, addr: u64, size: u64) -> bool {
        self.enabled && addr < self.base + self.size && addr + size > self.base
    }

    /// Get the end address (exclusive).
    pub fn end(&self) -> u64 {
        self.base + self.size
    }

    /// Set attributes.
    pub fn with_attributes(mut self, attrs: MemoryAttributes) -> Self {
        self.attributes = attrs;
        self
    }

    /// Set permissions.
    pub fn with_permissions(mut self, perms: AccessPermission) -> Self {
        self.permissions = perms;
        self
    }
}

impl fmt::Display for MemoryRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: 0x{:08x}-0x{:08x} ({} bytes, {:?})",
            self.name,
            self.base,
            self.base + self.size,
            self.size,
            self.attributes.mem_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_region_contains() {
        let region = MemoryRegion::ram(0x2000_0000, 0x1_0000, "SRAM");

        assert!(region.contains(0x2000_0000));
        assert!(region.contains(0x2000_FFFF));
        assert!(!region.contains(0x2001_0000));
        assert!(!region.contains(0x1FFF_FFFF));
    }

    #[test]
    fn test_memory_region_overlaps() {
        let region = MemoryRegion::ram(0x2000_0000, 0x1_0000, "SRAM");

        // Fully contained
        assert!(region.overlaps(0x2000_1000, 0x100));
        // Partial overlap at start
        assert!(region.overlaps(0x1FFF_F000, 0x2000));
        // Partial overlap at end
        assert!(region.overlaps(0x2000_F000, 0x2000));
        // No overlap before
        assert!(!region.overlaps(0x1000_0000, 0x1000));
        // No overlap after
        assert!(!region.overlaps(0x3000_0000, 0x1000));
    }

    #[test]
    fn test_access_permission() {
        assert!(AccessPermission::FULL_ACCESS.can_read(true));
        assert!(AccessPermission::FULL_ACCESS.can_read(false));
        assert!(AccessPermission::FULL_ACCESS.can_write(true));
        assert!(AccessPermission::FULL_ACCESS.can_write(false));

        assert!(AccessPermission::PRIV_RW.can_read(true));
        assert!(!AccessPermission::PRIV_RW.can_read(false));
        assert!(AccessPermission::PRIV_RW.can_write(true));
        assert!(!AccessPermission::PRIV_RW.can_write(false));

        assert!(!AccessPermission::NONE.can_read(true));
        assert!(!AccessPermission::NONE.can_write(true));
    }

    #[test]
    fn test_ap_bits_v7m() {
        let none = AccessPermission::from_ap_bits_v7m(0b000);
        assert!(!none.can_read(true));

        let priv_rw = AccessPermission::from_ap_bits_v7m(0b001);
        assert!(priv_rw.can_read(true));
        assert!(priv_rw.can_write(true));
        assert!(!priv_rw.can_read(false));

        let full = AccessPermission::from_ap_bits_v7m(0b011);
        assert!(full.can_read(true));
        assert!(full.can_write(true));
        assert!(full.can_read(false));
        assert!(full.can_write(false));
    }

    #[test]
    fn test_memory_attributes() {
        let cached = MemoryAttributes::normal_cached();
        assert!(cached.is_cacheable());
        assert!(!cached.is_device_or_strongly_ordered());

        let device = MemoryAttributes::device();
        assert!(!device.is_cacheable());
        assert!(device.is_device_or_strongly_ordered());
        assert!(device.execute_never);
    }
}
