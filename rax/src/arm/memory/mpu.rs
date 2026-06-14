//! Memory Protection Unit (MPU) implementation.
//!
//! This module implements the ARM MPU for Cortex-M and Cortex-R processors:
//! - ARMv6-M: Optional 8-region MPU (Cortex-M0+)
//! - ARMv7-M: Optional 8 or 16-region MPU (Cortex-M3/M4/M7)
//! - ARMv8-M: Optional 8 or 16-region MPU with enhanced security (Cortex-M23/M33/M55/M85)
//! - ARMv7-R/ARMv8-R: 12-24 region MPU (Cortex-R series)
//!
//! The MPU provides:
//! - Memory region definition with base address and size
//! - Access permissions (privileged/unprivileged, read/write)
//! - Memory attributes (cacheable, bufferable, shareable)
//! - Execute-never (XN) control
//! - Subregion disable (ARMv7-M)
//! - TrustZone security attribution (ARMv8-M)

use super::regions::{AccessPermission, MemoryAttributes, MemoryRegionType};
use super::{MemResult, MemoryError};
use crate::arm::cpu_trait::AccessType;

/// MPU type/variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MpuType {
    /// ARMv6-M MPU (8 regions, no subregions).
    V6M,
    /// ARMv7-M MPU (8 regions with subregions).
    V7M8,
    /// ARMv7-M MPU (16 regions with subregions).
    V7M16,
    /// ARMv8-M MPU (8 regions, no subregions, with limit address).
    V8M8,
    /// ARMv8-M MPU (16 regions, no subregions, with limit address).
    V8M16,
    /// ARMv7-R MPU (12 regions).
    V7R12,
    /// ARMv7-R MPU (16 regions).
    V7R16,
    /// ARMv8-R MPU (up to 24 regions).
    V8R,
}

impl MpuType {
    /// Get the number of regions for this MPU type.
    pub fn num_regions(&self) -> usize {
        match self {
            MpuType::V6M | MpuType::V7M8 | MpuType::V8M8 => 8,
            MpuType::V7M16 | MpuType::V8M16 | MpuType::V7R16 => 16,
            MpuType::V7R12 => 12,
            MpuType::V8R => 24,
        }
    }

    /// Check if this MPU type supports subregions.
    pub fn supports_subregions(&self) -> bool {
        matches!(
            self,
            MpuType::V7M8 | MpuType::V7M16 | MpuType::V7R12 | MpuType::V7R16
        )
    }

    /// Check if this is an ARMv8-M MPU (uses RBAR/RLAR format).
    pub fn is_v8m(&self) -> bool {
        matches!(self, MpuType::V8M8 | MpuType::V8M16)
    }

    /// Check if this is an ARMv8-R MPU.
    pub fn is_v8r(&self) -> bool {
        matches!(self, MpuType::V8R)
    }
}

/// MPU region attributes (ARMv7-M format).
#[derive(Clone, Copy, Debug, Default)]
pub struct MpuRegionAttr {
    /// Execute Never (XN) bit.
    pub xn: bool,
    /// Access Permission bits.
    pub ap: u8,
    /// Type Extension bits (TEX).
    pub tex: u8,
    /// Shareable bit.
    pub s: bool,
    /// Cacheable bit.
    pub c: bool,
    /// Bufferable bit.
    pub b: bool,
    /// Subregion disable bits (8 bits, one per subregion).
    pub srd: u8,
}

impl MpuRegionAttr {
    /// Create from RASR register value (ARMv7-M).
    pub fn from_rasr(rasr: u32) -> Self {
        Self {
            xn: (rasr >> 28) & 1 != 0,
            ap: ((rasr >> 24) & 0x7) as u8,
            tex: ((rasr >> 19) & 0x7) as u8,
            s: (rasr >> 18) & 1 != 0,
            c: (rasr >> 17) & 1 != 0,
            b: (rasr >> 16) & 1 != 0,
            srd: ((rasr >> 8) & 0xFF) as u8,
        }
    }

    /// Convert to RASR register value (ARMv7-M).
    pub fn to_rasr(&self, size_bits: u8, enabled: bool) -> u32 {
        let mut rasr = 0u32;
        if self.xn {
            rasr |= 1 << 28;
        }
        rasr |= (self.ap as u32 & 0x7) << 24;
        rasr |= (self.tex as u32 & 0x7) << 19;
        if self.s {
            rasr |= 1 << 18;
        }
        if self.c {
            rasr |= 1 << 17;
        }
        if self.b {
            rasr |= 1 << 16;
        }
        rasr |= (self.srd as u32) << 8;
        rasr |= ((size_bits as u32) & 0x1F) << 1;
        if enabled {
            rasr |= 1;
        }
        rasr
    }

    /// Get access permissions from AP bits.
    pub fn access_permission(&self) -> AccessPermission {
        AccessPermission::from_ap_bits_v7m(self.ap)
    }

    /// Get memory attributes.
    pub fn memory_attributes(&self) -> MemoryAttributes {
        // Decode TEX/C/B to memory type and cacheability
        let mem_type = match (self.tex, self.c, self.b) {
            (0b000, false, false) => MemoryRegionType::StronglyOrdered,
            (0b000, false, true) => MemoryRegionType::Device,
            (0b000, true, _) => MemoryRegionType::Normal,
            (0b001, false, false) => MemoryRegionType::Normal, // Non-cacheable
            (0b001, false, true) => MemoryRegionType::Normal,  // Reserved -> Normal
            (0b001, true, false) => MemoryRegionType::Normal,  // Write-back, write-allocate
            (0b001, true, true) => MemoryRegionType::Normal,   // Write-back, write-allocate
            (0b010, false, false) => MemoryRegionType::Device, // Non-shareable device
            (0b010, _, _) => MemoryRegionType::Normal,         // Reserved -> Normal
            _ => MemoryRegionType::Normal,                     // Cacheable normal memory
        };

        MemoryAttributes {
            mem_type,
            execute_never: self.xn,
            ..Default::default()
        }
    }
}

/// MPU region definition.
#[derive(Clone, Debug)]
pub struct MpuRegion {
    /// Region number (0-7/15/23 depending on MPU type).
    pub number: u8,
    /// Base address (aligned to region size).
    pub base: u32,
    /// Region size in bytes (must be power of 2, min 32).
    pub size: u32,
    /// Limit address for ARMv8-M (base + size - 1, aligned to 32 bytes).
    pub limit: u32,
    /// Region enabled.
    pub enabled: bool,
    /// Region attributes.
    pub attr: MpuRegionAttr,
    /// Access permissions (ARMv8-M style).
    pub permissions: AccessPermission,
    /// Execute never (separate for ARMv8-M).
    pub xn: bool,
    /// Privileged execute never (ARMv8-M).
    pub pxn: bool,
    /// Shareable (ARMv8-M).
    pub sh: u8,
    /// Attribute index (ARMv8-M/R).
    pub attr_idx: u8,
}

impl MpuRegion {
    /// Create a new disabled region.
    pub fn new(number: u8) -> Self {
        Self {
            number,
            base: 0,
            size: 0,
            limit: 0,
            enabled: false,
            attr: MpuRegionAttr::default(),
            permissions: AccessPermission::NONE,
            xn: false,
            pxn: false,
            sh: 0,
            attr_idx: 0,
        }
    }

    /// Check if an address falls within this region (ARMv7-M).
    pub fn contains_v7m(&self, addr: u32) -> bool {
        if !self.enabled || self.size == 0 {
            return false;
        }
        addr >= self.base && addr < self.base + self.size
    }

    /// Check if an address falls within this region (ARMv8-M).
    pub fn contains_v8m(&self, addr: u32) -> bool {
        if !self.enabled {
            return false;
        }
        addr >= self.base && addr <= self.limit
    }

    /// Check if address is in a disabled subregion (ARMv7-M).
    pub fn is_subregion_disabled(&self, addr: u32) -> bool {
        if self.size < 256 || self.attr.srd == 0 {
            return false;
        }

        let subregion_size = self.size / 8;
        let offset = addr - self.base;
        let subregion_num = (offset / subregion_size) as u8;

        (self.attr.srd >> subregion_num) & 1 != 0
    }

    /// Configure from RBAR/RASR (ARMv7-M).
    pub fn configure_v7m(&mut self, rbar: u32, rasr: u32) {
        // RBAR format: [31:5] ADDR, [4] VALID, [3:0] REGION
        self.base = rbar & 0xFFFF_FFE0;

        // RASR format: [31:29] reserved, [28] XN, [26:24] AP, [21:19] TEX,
        // [18] S, [17] C, [16] B, [15:8] SRD, [5:1] SIZE, [0] ENABLE
        self.enabled = (rasr & 1) != 0;
        let size_bits = ((rasr >> 1) & 0x1F) as u8;
        self.size = if size_bits >= 4 {
            1u32 << (size_bits + 1)
        } else {
            0
        };
        self.attr = MpuRegionAttr::from_rasr(rasr);
        self.xn = self.attr.xn;
    }

    /// Configure from RBAR/RLAR (ARMv8-M).
    pub fn configure_v8m(&mut self, rbar: u32, rlar: u32) {
        // RBAR format: [31:5] BASE, [4:3] SH, [2:1] AP, [0] XN
        self.base = rbar & 0xFFFF_FFE0;
        self.sh = ((rbar >> 3) & 0x3) as u8;
        let ap = ((rbar >> 1) & 0x3) as u8;
        self.xn = (rbar & 1) != 0;

        // RLAR format: [31:5] LIMIT, [4] PXN, [3:1] AttrIndx, [0] EN
        self.limit = rlar | 0x1F; // Limit is inclusive and aligned to 32 bytes
        self.pxn = (rlar >> 4) & 1 != 0;
        self.attr_idx = ((rlar >> 1) & 0x7) as u8;
        self.enabled = (rlar & 1) != 0;

        // Compute size from base and limit
        if self.enabled {
            self.size = self.limit - self.base + 1;
        }

        // Decode AP bits for ARMv8-M
        self.permissions = match ap {
            0b00 => AccessPermission::PRIV_RW,
            0b01 => AccessPermission::FULL_ACCESS,
            0b10 => AccessPermission::PRIV_RO,
            0b11 => AccessPermission::RO,
            _ => AccessPermission::NONE,
        };
    }

    /// Get RBAR value (ARMv7-M).
    pub fn get_rbar_v7m(&self, include_region: bool) -> u32 {
        let mut rbar = self.base & 0xFFFF_FFE0;
        if include_region {
            rbar |= 0x10 | (self.number as u32 & 0xF);
        }
        rbar
    }

    /// Get RASR value (ARMv7-M).
    pub fn get_rasr_v7m(&self) -> u32 {
        let size_bits = if self.size > 0 {
            (31 - self.size.leading_zeros()) as u8
        } else {
            0
        };
        self.attr.to_rasr(size_bits, self.enabled)
    }

    /// Get RBAR value (ARMv8-M).
    pub fn get_rbar_v8m(&self) -> u32 {
        let mut rbar = self.base & 0xFFFF_FFE0;
        rbar |= (self.sh as u32 & 0x3) << 3;
        let ap: u8 = match self.permissions {
            AccessPermission {
                priv_read: true,
                priv_write: true,
                unpriv_read: false,
                ..
            } => 0b00,
            AccessPermission {
                priv_read: true,
                priv_write: true,
                unpriv_read: true,
                unpriv_write: true,
            } => 0b01,
            AccessPermission {
                priv_read: true,
                priv_write: false,
                unpriv_read: false,
                ..
            } => 0b10,
            AccessPermission {
                priv_read: true,
                priv_write: false,
                unpriv_read: true,
                ..
            } => 0b11,
            _ => 0b00,
        };
        rbar |= (ap as u32 & 0x3) << 1;
        if self.xn {
            rbar |= 1;
        }
        rbar
    }

    /// Get RLAR value (ARMv8-M).
    pub fn get_rlar_v8m(&self) -> u32 {
        let mut rlar = self.limit & 0xFFFF_FFE0;
        if self.pxn {
            rlar |= 1 << 4;
        }
        rlar |= (self.attr_idx as u32 & 0x7) << 1;
        if self.enabled {
            rlar |= 1;
        }
        rlar
    }
}

impl Default for MpuRegion {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Memory Protection Unit.
#[derive(Clone, Debug)]
pub struct Mpu {
    /// MPU type.
    pub mpu_type: MpuType,
    /// MPU enabled.
    pub enabled: bool,
    /// Enable MPU during HardFault and NMI handlers.
    pub hfnmiena: bool,
    /// Enable default memory map as background region.
    pub privdefena: bool,
    /// MPU regions.
    pub regions: Vec<MpuRegion>,
    /// Currently selected region (for register access).
    pub selected_region: u8,
    /// Memory Attribute Indirection Registers (ARMv8-M).
    pub mair: [u8; 8],
}

impl Mpu {
    /// Create a new MPU of the specified type.
    pub fn new(mpu_type: MpuType) -> Self {
        let num_regions = mpu_type.num_regions();
        let regions = (0..num_regions as u8).map(MpuRegion::new).collect();

        Self {
            mpu_type,
            enabled: false,
            hfnmiena: false,
            privdefena: false,
            regions,
            selected_region: 0,
            mair: [0; 8],
        }
    }

    /// Reset the MPU to initial state.
    pub fn reset(&mut self) {
        self.enabled = false;
        self.hfnmiena = false;
        self.privdefena = false;
        self.selected_region = 0;
        self.mair = [0; 8];
        for region in &mut self.regions {
            *region = MpuRegion::new(region.number);
        }
    }

    /// Get the MPU_TYPE register value.
    pub fn get_type_register(&self) -> u32 {
        let dregion = self.regions.len() as u32;
        // [15:8] DREGION, [0] SEPARATE (always 0 for unified)
        dregion << 8
    }

    /// Get the MPU_CTRL register value.
    pub fn get_ctrl(&self) -> u32 {
        let mut ctrl = 0u32;
        if self.enabled {
            ctrl |= 1;
        }
        if self.hfnmiena {
            ctrl |= 1 << 1;
        }
        if self.privdefena {
            ctrl |= 1 << 2;
        }
        ctrl
    }

    /// Set the MPU_CTRL register.
    pub fn set_ctrl(&mut self, value: u32) {
        self.enabled = (value & 1) != 0;
        self.hfnmiena = (value >> 1) & 1 != 0;
        self.privdefena = (value >> 2) & 1 != 0;
    }

    /// Get the MPU_RNR register value.
    pub fn get_rnr(&self) -> u32 {
        self.selected_region as u32
    }

    /// Set the MPU_RNR register.
    pub fn set_rnr(&mut self, value: u32) {
        let region = (value & 0xFF) as u8;
        if (region as usize) < self.regions.len() {
            self.selected_region = region;
        }
    }

    /// Get the MPU_RBAR register value for the selected region (ARMv7-M).
    pub fn get_rbar_v7m(&self) -> u32 {
        if let Some(region) = self.regions.get(self.selected_region as usize) {
            region.get_rbar_v7m(false)
        } else {
            0
        }
    }

    /// Set the MPU_RBAR register (ARMv7-M).
    pub fn set_rbar_v7m(&mut self, value: u32) {
        // Check VALID bit to see if we should switch regions
        if (value >> 4) & 1 != 0 {
            let region_num = (value & 0xF) as u8;
            if (region_num as usize) < self.regions.len() {
                self.selected_region = region_num;
            }
        }

        if let Some(region) = self.regions.get_mut(self.selected_region as usize) {
            region.base = value & 0xFFFF_FFE0;
        }
    }

    /// Get the MPU_RASR register value for the selected region (ARMv7-M).
    pub fn get_rasr_v7m(&self) -> u32 {
        if let Some(region) = self.regions.get(self.selected_region as usize) {
            region.get_rasr_v7m()
        } else {
            0
        }
    }

    /// Set the MPU_RASR register (ARMv7-M).
    pub fn set_rasr_v7m(&mut self, value: u32) {
        if let Some(region) = self.regions.get_mut(self.selected_region as usize) {
            let rbar = region.base;
            region.configure_v7m(rbar, value);
        }
    }

    /// Get the MPU_RBAR register value for the selected region (ARMv8-M).
    pub fn get_rbar_v8m(&self) -> u32 {
        if let Some(region) = self.regions.get(self.selected_region as usize) {
            region.get_rbar_v8m()
        } else {
            0
        }
    }

    /// Set the MPU_RBAR register (ARMv8-M).
    pub fn set_rbar_v8m(&mut self, value: u32) {
        if let Some(region) = self.regions.get_mut(self.selected_region as usize) {
            let rlar = region.get_rlar_v8m();
            region.configure_v8m(value, rlar);
        }
    }

    /// Get the MPU_RLAR register value for the selected region (ARMv8-M).
    pub fn get_rlar_v8m(&self) -> u32 {
        if let Some(region) = self.regions.get(self.selected_region as usize) {
            region.get_rlar_v8m()
        } else {
            0
        }
    }

    /// Set the MPU_RLAR register (ARMv8-M).
    pub fn set_rlar_v8m(&mut self, value: u32) {
        if let Some(region) = self.regions.get_mut(self.selected_region as usize) {
            let rbar = region.get_rbar_v8m();
            region.configure_v8m(rbar, value);
        }
    }

    /// Get MAIR0 register (ARMv8-M).
    pub fn get_mair0(&self) -> u32 {
        u32::from_le_bytes([self.mair[0], self.mair[1], self.mair[2], self.mair[3]])
    }

    /// Set MAIR0 register (ARMv8-M).
    pub fn set_mair0(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.mair[0..4].copy_from_slice(&bytes);
    }

    /// Get MAIR1 register (ARMv8-M).
    pub fn get_mair1(&self) -> u32 {
        u32::from_le_bytes([self.mair[4], self.mair[5], self.mair[6], self.mair[7]])
    }

    /// Set MAIR1 register (ARMv8-M).
    pub fn set_mair1(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.mair[4..8].copy_from_slice(&bytes);
    }

    /// Find the highest-priority region that matches an address.
    pub fn find_region(&self, addr: u32) -> Option<&MpuRegion> {
        if !self.enabled {
            return None;
        }

        // Higher region numbers have higher priority
        if self.mpu_type.is_v8m() || self.mpu_type.is_v8r() {
            self.regions.iter().rev().find(|r| r.contains_v8m(addr))
        } else {
            self.regions
                .iter()
                .rev()
                .find(|r| r.contains_v7m(addr) && !r.is_subregion_disabled(addr))
        }
    }

    /// Check if an access is allowed.
    pub fn check_access(
        &self,
        addr: u32,
        size: u32,
        access: AccessType,
        privileged: bool,
    ) -> MemResult<()> {
        if !self.enabled {
            return Ok(());
        }

        // Check if any byte in the access range would fault
        // For simplicity, we check the start address
        if let Some(region) = self.find_region(addr) {
            // Check permissions
            let allowed = match access {
                AccessType::InstructionFetch => {
                    if region.xn || (privileged && region.pxn) {
                        false
                    } else {
                        region.attr.access_permission().can_read(privileged)
                            || region.permissions.can_read(privileged)
                    }
                }
                AccessType::Read | AccessType::Atomic => {
                    region.attr.access_permission().can_read(privileged)
                        || region.permissions.can_read(privileged)
                }
                AccessType::Write => {
                    region.attr.access_permission().can_write(privileged)
                        || region.permissions.can_write(privileged)
                }
            };

            if allowed {
                Ok(())
            } else {
                Err(MemoryError::MpuFault {
                    addr: addr as u64,
                    access,
                })
            }
        } else if self.privdefena && privileged {
            // Background region allows privileged access
            Ok(())
        } else {
            Err(MemoryError::MpuFault {
                addr: addr as u64,
                access,
            })
        }
    }

    /// Configure a region (helper method).
    pub fn configure_region(
        &mut self,
        number: u8,
        base: u32,
        size: u32,
        permissions: AccessPermission,
        xn: bool,
        enabled: bool,
    ) {
        if let Some(region) = self.regions.get_mut(number as usize) {
            region.base = base;
            region.size = size;
            region.limit = base + size - 1;
            region.permissions = permissions;
            region.xn = xn;
            region.enabled = enabled;

            // Also set v7m-style attributes
            region.attr.ap = match permissions {
                AccessPermission {
                    priv_read: true,
                    priv_write: true,
                    unpriv_read: false,
                    unpriv_write: false,
                } => 0b001,
                AccessPermission {
                    priv_read: true,
                    priv_write: true,
                    unpriv_read: true,
                    unpriv_write: true,
                } => 0b011,
                AccessPermission {
                    priv_read: true,
                    priv_write: false,
                    unpriv_read: false,
                    unpriv_write: false,
                } => 0b101,
                AccessPermission {
                    priv_read: true,
                    priv_write: false,
                    unpriv_read: true,
                    unpriv_write: false,
                } => 0b110,
                _ => 0b000,
            };
            region.attr.xn = xn;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpu_type() {
        assert_eq!(MpuType::V7M8.num_regions(), 8);
        assert_eq!(MpuType::V7M16.num_regions(), 16);
        assert_eq!(MpuType::V8R.num_regions(), 24);

        assert!(MpuType::V7M8.supports_subregions());
        assert!(!MpuType::V8M8.supports_subregions());
    }

    #[test]
    fn test_mpu_region_v7m() {
        let mut region = MpuRegion::new(0);

        // Configure: base=0x20000000, size=64KB, AP=full access, enabled
        // RASR format: [28] XN, [26:24] AP, [21:19] TEX, [18] S, [17] C, [16] B,
        //              [15:8] SRD, [5:1] SIZE, [0] ENABLE
        // SIZE=15 means 2^(15+1) = 2^16 = 64KB
        // SIZE field in bits [5:1]: 15 << 1 = 0x1E
        // AP=011 in bits [26:24]: 0x03 << 24 = 0x0300_0000
        // ENABLE=1 in bit [0]: 0x01
        let rbar = 0x2000_0000;
        let rasr = 0x0300_001F; // AP=011, SIZE=15 (64KB=2^16), ENABLE=1

        region.configure_v7m(rbar, rasr);

        assert!(region.enabled);
        assert_eq!(region.base, 0x2000_0000);
        assert_eq!(region.size, 0x1_0000); // 64KB
        assert!(region.contains_v7m(0x2000_0000));
        assert!(region.contains_v7m(0x2000_FFFF));
        assert!(!region.contains_v7m(0x2001_0000));
    }

    #[test]
    fn test_mpu_region_v8m() {
        let mut region = MpuRegion::new(0);

        // Configure: base=0x20000000, limit=0x2000FFFF (64KB)
        let rbar = 0x2000_0000 | (0b01 << 1); // Base + AP=01 (full access)
        let rlar = 0x2000_FFFF | 1; // Limit + EN=1

        region.configure_v8m(rbar, rlar);

        assert!(region.enabled);
        assert_eq!(region.base, 0x2000_0000);
        assert!(region.contains_v8m(0x2000_0000));
        assert!(region.contains_v8m(0x2000_FFFF));
        assert!(!region.contains_v8m(0x2001_0000));
    }

    #[test]
    fn test_mpu_access_check() {
        let mut mpu = Mpu::new(MpuType::V7M8);
        mpu.enabled = true;
        mpu.privdefena = false;

        // Configure region 0: 0x20000000-0x2000FFFF, full access
        mpu.configure_region(
            0,
            0x2000_0000,
            0x1_0000,
            AccessPermission::FULL_ACCESS,
            false,
            true,
        );

        // Access within region should succeed
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Read, true)
                .is_ok()
        );
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Write, false)
                .is_ok()
        );

        // Access outside region should fail
        assert!(
            mpu.check_access(0x3000_0000, 4, AccessType::Read, true)
                .is_err()
        );
    }

    #[test]
    fn test_mpu_privileged_only() {
        let mut mpu = Mpu::new(MpuType::V7M8);
        mpu.enabled = true;

        // Configure region 0: privileged RW only
        mpu.configure_region(
            0,
            0x2000_0000,
            0x1_0000,
            AccessPermission::PRIV_RW,
            false,
            true,
        );

        // Privileged access should succeed
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Read, true)
                .is_ok()
        );
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Write, true)
                .is_ok()
        );

        // Unprivileged access should fail
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Read, false)
                .is_err()
        );
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Write, false)
                .is_err()
        );
    }

    #[test]
    fn test_mpu_execute_never() {
        let mut mpu = Mpu::new(MpuType::V7M8);
        mpu.enabled = true;

        // Configure region 0: full access but XN
        mpu.configure_region(
            0,
            0x2000_0000,
            0x1_0000,
            AccessPermission::FULL_ACCESS,
            true,
            true,
        );

        // Data access should succeed
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Read, true)
                .is_ok()
        );

        // Instruction fetch should fail
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::InstructionFetch, true)
                .is_err()
        );
    }

    #[test]
    fn test_mpu_background_region() {
        let mut mpu = Mpu::new(MpuType::V7M8);
        mpu.enabled = true;
        mpu.privdefena = true;

        // No regions configured, but background region enabled
        // Privileged access should succeed
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Read, true)
                .is_ok()
        );

        // Unprivileged access should still fail
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Read, false)
                .is_err()
        );
    }

    #[test]
    fn test_mpu_region_priority() {
        let mut mpu = Mpu::new(MpuType::V7M8);
        mpu.enabled = true;

        // Configure overlapping regions
        // Region 0: 0x20000000-0x2000FFFF, RO
        mpu.configure_region(0, 0x2000_0000, 0x1_0000, AccessPermission::RO, false, true);
        // Region 1: 0x20000000-0x20001FFF, RW (higher priority)
        mpu.configure_region(
            1,
            0x2000_0000,
            0x2000,
            AccessPermission::FULL_ACCESS,
            false,
            true,
        );

        // Access to first 8KB should use region 1 (RW)
        assert!(
            mpu.check_access(0x2000_0000, 4, AccessType::Write, true)
                .is_ok()
        );

        // Access beyond first 8KB should use region 0 (RO)
        assert!(
            mpu.check_access(0x2000_2000, 4, AccessType::Write, true)
                .is_err()
        );
        assert!(
            mpu.check_access(0x2000_2000, 4, AccessType::Read, true)
                .is_ok()
        );
    }

    #[test]
    fn test_mpu_registers() {
        let mut mpu = Mpu::new(MpuType::V7M8);

        // Check type register
        let type_reg = mpu.get_type_register();
        assert_eq!((type_reg >> 8) & 0xFF, 8); // 8 regions

        // Set and get CTRL
        mpu.set_ctrl(0x7);
        assert!(mpu.enabled);
        assert!(mpu.hfnmiena);
        assert!(mpu.privdefena);
        assert_eq!(mpu.get_ctrl(), 0x7);

        // Set and get RNR
        mpu.set_rnr(3);
        assert_eq!(mpu.get_rnr(), 3);
    }
}
