//! AArch64 Memory Management Unit (MMU)
//!
//! This module implements the complete AArch64 MMU including:
//! - 4KB, 16KB, and 64KB translation granules
//! - 4-level page table walks
//! - Stage 1 and Stage 2 translation
//! - Memory attributes (MAIR)
//! - Access permissions
//! - Translation regimes for all ELs

use crate::arm::memory::{ArmMemory, MemoryError};

// =============================================================================
// Translation Granule
// =============================================================================

/// Translation granule size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslationGranule {
    /// 4KB pages.
    Granule4KB,
    /// 16KB pages.
    Granule16KB,
    /// 64KB pages.
    Granule64KB,
}

impl TranslationGranule {
    /// Get the page size in bytes.
    pub fn page_size(&self) -> u64 {
        match self {
            Self::Granule4KB => 4 * 1024,
            Self::Granule16KB => 16 * 1024,
            Self::Granule64KB => 64 * 1024,
        }
    }

    /// Get the page offset bits.
    pub fn page_offset_bits(&self) -> u32 {
        match self {
            Self::Granule4KB => 12,
            Self::Granule16KB => 14,
            Self::Granule64KB => 16,
        }
    }

    /// Get the number of bits per table level.
    pub fn bits_per_level(&self) -> u32 {
        match self {
            Self::Granule4KB => 9,   // 512 entries
            Self::Granule16KB => 11, // 2048 entries
            Self::Granule64KB => 13, // 8192 entries
        }
    }

    /// Get number of entries per table.
    pub fn entries_per_table(&self) -> u64 {
        1 << self.bits_per_level()
    }

    /// Decode from TG0 field of TCR.
    pub fn from_tg0(tg0: u8) -> Option<Self> {
        match tg0 {
            0b00 => Some(Self::Granule4KB),
            0b01 => Some(Self::Granule64KB),
            0b10 => Some(Self::Granule16KB),
            _ => None,
        }
    }

    /// Decode from TG1 field of TCR.
    pub fn from_tg1(tg1: u8) -> Option<Self> {
        match tg1 {
            0b01 => Some(Self::Granule16KB),
            0b10 => Some(Self::Granule4KB),
            0b11 => Some(Self::Granule64KB),
            _ => None,
        }
    }
}

// =============================================================================
// Translation Regime
// =============================================================================

/// Translation regime (defines which registers control translation).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslationRegime {
    /// EL1&0 stage 1 translation.
    EL10,
    /// EL2 stage 1 translation (when E2H=0).
    EL2,
    /// EL2&0 stage 1 translation (when E2H=1).
    EL20,
    /// EL3 stage 1 translation.
    EL3,
    /// EL1&0 stage 2 translation (controlled by EL2).
    Stage2,
}

// =============================================================================
// Translation Fault
// =============================================================================

/// Translation fault information.
#[derive(Clone, Copy, Debug)]
pub struct TranslationFault {
    /// Fault type.
    pub fault_type: TranslationFaultType,
    /// Level at which fault occurred (0-3).
    pub level: u8,
    /// Whether fault occurred during stage 2 translation.
    pub stage2: bool,
    /// Whether fault occurred during stage 1 table walk for stage 2.
    pub s1ptw: bool,
    /// Faulting virtual address.
    pub va: u64,
    /// Faulting IPA (for stage 2 faults).
    pub ipa: Option<u64>,
}

/// Translation fault type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslationFaultType {
    /// Address size fault.
    AddressSize,
    /// Translation fault (unmapped).
    Translation,
    /// Access flag fault.
    AccessFlag,
    /// Permission fault.
    Permission,
    /// Alignment fault.
    Alignment,
    /// External abort.
    ExternalAbort,
}

// =============================================================================
// Page Table Entry
// =============================================================================

/// Page table entry descriptor.
#[derive(Clone, Copy, Debug)]
pub struct PageTableEntry {
    /// Raw descriptor value.
    pub value: u64,
}

impl PageTableEntry {
    /// Create from raw value.
    pub const fn from_raw(value: u64) -> Self {
        Self { value }
    }

    /// Check if entry is valid.
    pub fn is_valid(&self) -> bool {
        self.value & 1 != 0
    }

    /// Check if this is a table descriptor (not a block/page).
    pub fn is_table(&self) -> bool {
        self.is_valid() && (self.value & 2) != 0
    }

    /// Check if this is a block or page descriptor.
    pub fn is_block_or_page(&self) -> bool {
        self.is_valid() && (self.value & 2) == 0
    }

    /// Get the output address (for table, block, or page).
    pub fn output_address(&self, granule: TranslationGranule, level: u8) -> u64 {
        if self.is_table() {
            // Table descriptor: bits [47:12] for 4KB, [47:14] for 16KB, [47:16] for 64KB
            let mask = match granule {
                TranslationGranule::Granule4KB => 0x0000_FFFF_FFFF_F000,
                TranslationGranule::Granule16KB => 0x0000_FFFF_FFFF_C000,
                TranslationGranule::Granule64KB => 0x0000_FFFF_FFFF_0000,
            };
            self.value & mask
        } else {
            // Block/page descriptor
            let page_bits = granule.page_offset_bits();
            let level_bits = granule.bits_per_level();
            let output_bits = page_bits + (3 - level as u32) * level_bits;
            let mask = ((1u64 << 48) - 1) & !((1u64 << output_bits) - 1);
            self.value & mask
        }
    }

    /// Get Access Permissions (AP[2:1]).
    pub fn ap(&self) -> u8 {
        ((self.value >> 6) & 0x3) as u8
    }

    /// Check if unprivileged access is allowed (AP[2:1] = x1).
    pub fn user_accessible(&self) -> bool {
        (self.value >> 6) & 1 != 0
    }

    /// Check if read-only (AP[2:1] = 1x).
    pub fn read_only(&self) -> bool {
        (self.value >> 7) & 1 != 0
    }

    /// Get Shareability (SH[1:0]).
    pub fn shareability(&self) -> u8 {
        ((self.value >> 8) & 0x3) as u8
    }

    /// Get Access Flag (AF).
    pub fn access_flag(&self) -> bool {
        (self.value >> 10) & 1 != 0
    }

    /// Get Not Global (nG).
    pub fn not_global(&self) -> bool {
        (self.value >> 11) & 1 != 0
    }

    /// Get the memory attribute index (AttrIndx[2:0]).
    pub fn attr_index(&self) -> u8 {
        ((self.value >> 2) & 0x7) as u8
    }

    /// Get Execute-Never for unprivileged (UXN/XN for EL0).
    pub fn uxn(&self) -> bool {
        (self.value >> 54) & 1 != 0
    }

    /// Get Privileged Execute-Never (PXN).
    pub fn pxn(&self) -> bool {
        (self.value >> 53) & 1 != 0
    }

    /// Get Contiguous bit.
    pub fn contiguous(&self) -> bool {
        (self.value >> 52) & 1 != 0
    }

    /// Get Dirty Bit Modifier (DBM).
    pub fn dbm(&self) -> bool {
        (self.value >> 51) & 1 != 0
    }

    /// Get GP (Guarded Page for BTI).
    pub fn gp(&self) -> bool {
        (self.value >> 50) & 1 != 0
    }

    // Stage 2 specific attributes

    /// Get Stage 2 memory attributes (MemAttr[3:0]).
    pub fn s2_memattr(&self) -> u8 {
        ((self.value >> 2) & 0xF) as u8
    }

    /// Get Stage 2 access permissions (S2AP[1:0]).
    pub fn s2ap(&self) -> u8 {
        ((self.value >> 6) & 0x3) as u8
    }

    /// Get Stage 2 Execute-Never (XN[1:0]).
    pub fn s2xn(&self) -> u8 {
        ((self.value >> 53) & 0x3) as u8
    }

    /// Get Address Fault (AF) for Stage 2.
    pub fn s2af(&self) -> bool {
        (self.value >> 10) & 1 != 0
    }

    // Table descriptor attributes

    /// Get NSTable (for Secure state).
    pub fn ns_table(&self) -> bool {
        (self.value >> 63) & 1 != 0
    }

    /// Get APTable[1:0].
    pub fn ap_table(&self) -> u8 {
        ((self.value >> 61) & 0x3) as u8
    }

    /// Get UXNTable / XNTable.
    pub fn uxn_table(&self) -> bool {
        (self.value >> 60) & 1 != 0
    }

    /// Get PXNTable.
    pub fn pxn_table(&self) -> bool {
        (self.value >> 59) & 1 != 0
    }
}

// =============================================================================
// Address Descriptor (result of translation)
// =============================================================================

/// Result of successful address translation.
#[derive(Clone, Copy, Debug)]
pub struct AddressDescriptor {
    /// Physical address.
    pub pa: u64,
    /// Memory attributes.
    pub attributes: MemoryAttributes,
    /// Whether the page is contiguous.
    pub contiguous: bool,
    /// Guarded page (for BTI).
    pub gp: bool,
}

/// Memory attributes for a translated address.
#[derive(Clone, Copy, Debug)]
pub struct MemoryAttributes {
    /// Memory type.
    pub mem_type: MemoryType,
    /// Shareability.
    pub shareability: Shareability,
    /// Outer cacheability.
    pub outer_cacheable: Cacheability,
    /// Inner cacheability.
    pub inner_cacheable: Cacheability,
}

/// Memory type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryType {
    /// Device memory.
    Device(DeviceType),
    /// Normal memory.
    Normal,
}

/// Device memory type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceType {
    /// Device-nGnRnE (most restrictive).
    NGnRnE,
    /// Device-nGnRE.
    NGnRE,
    /// Device-nGRE.
    NGRE,
    /// Device-GRE (least restrictive).
    GRE,
}

/// Shareability domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shareability {
    /// Non-shareable.
    NonShareable,
    /// Outer Shareable.
    OuterShareable,
    /// Inner Shareable.
    InnerShareable,
}

/// Cacheability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheability {
    /// Non-cacheable.
    NonCacheable,
    /// Write-Through.
    WriteThrough,
    /// Write-Back.
    WriteBack,
}

impl Default for MemoryAttributes {
    fn default() -> Self {
        Self {
            mem_type: MemoryType::Normal,
            shareability: Shareability::NonShareable,
            outer_cacheable: Cacheability::NonCacheable,
            inner_cacheable: Cacheability::NonCacheable,
        }
    }
}

// =============================================================================
// MMU Configuration
// =============================================================================

/// MMU configuration.
#[derive(Clone, Debug)]
pub struct MmuConfig {
    /// Whether MMU is enabled.
    pub enabled: bool,
    /// Physical address size (bits).
    pub pa_size: u8,
    /// Input address size for TTBR0 (T0SZ).
    pub t0sz: u8,
    /// Input address size for TTBR1 (T1SZ).
    pub t1sz: u8,
    /// Translation granule for TTBR0.
    pub tg0: TranslationGranule,
    /// Translation granule for TTBR1.
    pub tg1: TranslationGranule,
    /// TTBR0 value.
    pub ttbr0: u64,
    /// TTBR1 value.
    pub ttbr1: u64,
    /// MAIR value.
    pub mair: u64,
    /// Whether WXN is enabled.
    pub wxn: bool,
}

impl Default for MmuConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            pa_size: 48,
            t0sz: 16,
            t1sz: 16,
            tg0: TranslationGranule::Granule4KB,
            tg1: TranslationGranule::Granule4KB,
            ttbr0: 0,
            ttbr1: 0,
            mair: 0,
            wxn: false,
        }
    }
}

// =============================================================================
// MMU
// =============================================================================

/// AArch64 Memory Management Unit.
pub struct Mmu {
    /// Configuration.
    config: MmuConfig,
}

impl Mmu {
    /// Create a new MMU.
    pub fn new() -> Self {
        Self {
            config: MmuConfig::default(),
        }
    }

    /// Create with configuration.
    pub fn with_config(config: MmuConfig) -> Self {
        Self { config }
    }

    /// Get configuration.
    pub fn config(&self) -> &MmuConfig {
        &self.config
    }

    /// Update configuration.
    pub fn set_config(&mut self, config: MmuConfig) {
        self.config = config;
    }

    /// Check if MMU is enabled.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Translate a virtual address.
    pub fn translate(
        &self,
        va: u64,
        memory: &dyn ArmMemory,
        is_write: bool,
        is_execute: bool,
        privileged: bool,
        el: u8,
    ) -> Result<AddressDescriptor, TranslationFault> {
        if !self.config.enabled {
            // MMU disabled - identity map
            return Ok(AddressDescriptor {
                pa: va,
                attributes: MemoryAttributes::default(),
                contiguous: false,
                gp: false,
            });
        }

        // Determine which TTBR to use based on VA
        let (ttbr, tsz, granule) = self.select_ttbr(va)?;

        // Calculate starting level and number of levels
        let input_size = 64 - tsz as u32;
        let page_bits = granule.page_offset_bits();
        let bits_per_level = granule.bits_per_level();

        // Calculate starting level (can be 0, 1, 2, or 3)
        let levels_needed = (input_size - page_bits + bits_per_level - 1) / bits_per_level;
        let start_level = (4 - levels_needed) as u8;

        // Walk the page tables
        let mut table_addr = ttbr & 0x0000_FFFF_FFFF_F000; // Extract base address
        let mut level = start_level;
        let mut ap_from_table: u8 = 0;
        let mut uxn_from_table = false;
        let mut pxn_from_table = false;

        loop {
            // Calculate index for this level
            let index = self.level_index(va, level, granule);

            // Read page table entry
            let entry_addr = table_addr + (index * 8);
            let entry_value = memory.read_u64(entry_addr).map_err(|_| TranslationFault {
                fault_type: TranslationFaultType::ExternalAbort,
                level,
                stage2: false,
                s1ptw: false,
                va,
                ipa: None,
            })?;

            let entry = PageTableEntry::from_raw(entry_value);

            // Check validity
            if !entry.is_valid() {
                return Err(TranslationFault {
                    fault_type: TranslationFaultType::Translation,
                    level,
                    stage2: false,
                    s1ptw: false,
                    va,
                    ipa: None,
                });
            }

            if entry.is_table() && level < 3 {
                // Table descriptor - continue walk
                // Accumulate hierarchical attributes
                ap_from_table |= entry.ap_table();
                uxn_from_table |= entry.uxn_table();
                pxn_from_table |= entry.pxn_table();

                table_addr = entry.output_address(granule, level);
                level += 1;
            } else {
                // Block or page descriptor
                // Check access flag
                if !entry.access_flag() {
                    return Err(TranslationFault {
                        fault_type: TranslationFaultType::AccessFlag,
                        level,
                        stage2: false,
                        s1ptw: false,
                        va,
                        ipa: None,
                    });
                }

                // Check permissions
                self.check_permissions(
                    &entry,
                    is_write,
                    is_execute,
                    privileged,
                    el,
                    ap_from_table,
                    uxn_from_table,
                    pxn_from_table,
                    level,
                    va,
                )?;

                // Calculate physical address
                let pa = self.compute_pa(va, &entry, level, granule);

                // Decode memory attributes
                let attributes = self.decode_attributes(&entry);

                return Ok(AddressDescriptor {
                    pa,
                    attributes,
                    contiguous: entry.contiguous(),
                    gp: entry.gp(),
                });
            }
        }
    }

    /// Select TTBR based on virtual address.
    fn select_ttbr(&self, va: u64) -> Result<(u64, u8, TranslationGranule), TranslationFault> {
        // Check if address is in TTBR0 or TTBR1 range
        // TTBR0: VA[63:64-T0SZ] == 0
        // TTBR1: VA[63:64-T1SZ] == 1s

        let t0sz = self.config.t0sz;
        let t1sz = self.config.t1sz;

        // Check TTBR0 range
        let t0_max = 1u64 << (64 - t0sz);
        if va < t0_max {
            return Ok((self.config.ttbr0, t0sz, self.config.tg0));
        }

        // Check TTBR1 range
        let t1_min = !((1u64 << (64 - t1sz)) - 1);
        if va >= t1_min {
            return Ok((self.config.ttbr1, t1sz, self.config.tg1));
        }

        // Address not in either range - translation fault at level 0
        Err(TranslationFault {
            fault_type: TranslationFaultType::Translation,
            level: 0,
            stage2: false,
            s1ptw: false,
            va,
            ipa: None,
        })
    }

    /// Calculate index for a page table level.
    fn level_index(&self, va: u64, level: u8, granule: TranslationGranule) -> u64 {
        let page_bits = granule.page_offset_bits();
        let bits_per_level = granule.bits_per_level();

        let shift = page_bits + (3 - level as u32) * bits_per_level;
        let mask = (1u64 << bits_per_level) - 1;

        (va >> shift) & mask
    }

    /// Compute physical address from VA and entry.
    fn compute_pa(
        &self,
        va: u64,
        entry: &PageTableEntry,
        level: u8,
        granule: TranslationGranule,
    ) -> u64 {
        let page_bits = granule.page_offset_bits();
        let bits_per_level = granule.bits_per_level();

        // Calculate number of bits from VA to include
        let va_bits = page_bits + (3 - level as u32) * bits_per_level;
        let va_mask = (1u64 << va_bits) - 1;

        let base = entry.output_address(granule, level);
        base | (va & va_mask)
    }

    /// Check access permissions.
    fn check_permissions(
        &self,
        entry: &PageTableEntry,
        is_write: bool,
        is_execute: bool,
        privileged: bool,
        el: u8,
        ap_from_table: u8,
        uxn_from_table: bool,
        pxn_from_table: bool,
        level: u8,
        va: u64,
    ) -> Result<(), TranslationFault> {
        let ap = entry.ap();

        // Apply hierarchical permissions
        let effective_ap = ap | ap_from_table;
        let effective_uxn = entry.uxn() || uxn_from_table;
        let effective_pxn = entry.pxn() || pxn_from_table;

        // Check read/write permissions
        // AP[2:1]:
        // 00 = EL0: No access, ELx: RW
        // 01 = EL0: RW, ELx: RW
        // 10 = EL0: No access, ELx: RO
        // 11 = EL0: RO, ELx: RO

        let read_ok;
        let write_ok;

        if privileged {
            // ELx access
            read_ok = true; // Always readable at ELx
            write_ok = (effective_ap & 0b10) == 0; // Writable if AP[2] == 0
        } else {
            // EL0 access
            read_ok = (effective_ap & 0b01) != 0; // Readable if AP[1] == 1
            write_ok = effective_ap == 0b01; // Writable only if AP == 01
        }

        if is_write && !write_ok {
            return Err(TranslationFault {
                fault_type: TranslationFaultType::Permission,
                level,
                stage2: false,
                s1ptw: false,
                va,
                ipa: None,
            });
        }

        if !is_write && !is_execute && !read_ok {
            return Err(TranslationFault {
                fault_type: TranslationFaultType::Permission,
                level,
                stage2: false,
                s1ptw: false,
                va,
                ipa: None,
            });
        }

        // Check execute permissions
        if is_execute {
            let xn = if privileged {
                effective_pxn
            } else {
                effective_uxn
            };

            if xn {
                return Err(TranslationFault {
                    fault_type: TranslationFaultType::Permission,
                    level,
                    stage2: false,
                    s1ptw: false,
                    va,
                    ipa: None,
                });
            }

            // Check WXN (Write implies eXecute Never)
            if self.config.wxn && write_ok {
                return Err(TranslationFault {
                    fault_type: TranslationFaultType::Permission,
                    level,
                    stage2: false,
                    s1ptw: false,
                    va,
                    ipa: None,
                });
            }
        }

        Ok(())
    }

    /// Decode memory attributes from MAIR.
    fn decode_attributes(&self, entry: &PageTableEntry) -> MemoryAttributes {
        let attr_index = entry.attr_index();
        let mair_byte = ((self.config.mair >> (attr_index * 8)) & 0xFF) as u8;

        let (mem_type, outer, inner) = decode_mair_byte(mair_byte);

        let sh = entry.shareability();
        let shareability = match sh {
            0b00 => Shareability::NonShareable,
            0b10 => Shareability::OuterShareable,
            0b11 => Shareability::InnerShareable,
            _ => Shareability::NonShareable,
        };

        MemoryAttributes {
            mem_type,
            shareability,
            outer_cacheable: outer,
            inner_cacheable: inner,
        }
    }
}

impl Default for Mmu {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode a MAIR byte into memory attributes.
fn decode_mair_byte(mair: u8) -> (MemoryType, Cacheability, Cacheability) {
    let outer = (mair >> 4) & 0xF;
    let inner = mair & 0xF;

    if outer == 0 {
        // Device memory
        let device_type = match inner {
            0b0000 => DeviceType::NGnRnE,
            0b0100 => DeviceType::NGnRE,
            0b1000 => DeviceType::NGRE,
            0b1100 => DeviceType::GRE,
            _ => DeviceType::NGnRnE,
        };
        (
            MemoryType::Device(device_type),
            Cacheability::NonCacheable,
            Cacheability::NonCacheable,
        )
    } else {
        // Normal memory
        let outer_cache = decode_cacheability(outer);
        let inner_cache = decode_cacheability(inner);
        (MemoryType::Normal, outer_cache, inner_cache)
    }
}

/// Decode cacheability from MAIR nibble.
fn decode_cacheability(nibble: u8) -> Cacheability {
    match nibble & 0xC {
        0b0000 => Cacheability::NonCacheable,
        0b0100 => Cacheability::NonCacheable, // Device or unpredictable
        0b1000 => Cacheability::WriteThrough,
        0b1100 => Cacheability::WriteBack,
        _ => Cacheability::NonCacheable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_granule_sizes() {
        assert_eq!(TranslationGranule::Granule4KB.page_size(), 4096);
        assert_eq!(TranslationGranule::Granule16KB.page_size(), 16384);
        assert_eq!(TranslationGranule::Granule64KB.page_size(), 65536);

        assert_eq!(TranslationGranule::Granule4KB.page_offset_bits(), 12);
        assert_eq!(TranslationGranule::Granule16KB.page_offset_bits(), 14);
        assert_eq!(TranslationGranule::Granule64KB.page_offset_bits(), 16);

        assert_eq!(TranslationGranule::Granule4KB.bits_per_level(), 9);
        assert_eq!(TranslationGranule::Granule16KB.bits_per_level(), 11);
        assert_eq!(TranslationGranule::Granule64KB.bits_per_level(), 13);
    }

    #[test]
    fn test_pte_valid() {
        let valid = PageTableEntry::from_raw(0x0000_0000_0000_0003);
        assert!(valid.is_valid());
        assert!(valid.is_table());

        let invalid = PageTableEntry::from_raw(0x0000_0000_0000_0000);
        assert!(!invalid.is_valid());

        let block = PageTableEntry::from_raw(0x0000_0000_0000_0401);
        assert!(block.is_valid());
        assert!(!block.is_table());
        assert!(block.is_block_or_page());
    }

    #[test]
    fn test_pte_permissions() {
        // AP = 00, ELx RW, EL0 no access
        let entry = PageTableEntry::from_raw(0x0000_0000_0000_0401);
        assert_eq!(entry.ap(), 0b00);
        assert!(!entry.user_accessible());
        assert!(!entry.read_only());

        // AP = 01, ELx RW, EL0 RW
        let entry = PageTableEntry::from_raw(0x0000_0000_0000_0441);
        assert_eq!(entry.ap(), 0b01);
        assert!(entry.user_accessible());
        assert!(!entry.read_only());

        // AP = 11, ELx RO, EL0 RO
        let entry = PageTableEntry::from_raw(0x0000_0000_0000_04C1);
        assert_eq!(entry.ap(), 0b11);
        assert!(entry.user_accessible());
        assert!(entry.read_only());
    }

    #[test]
    fn test_mmu_disabled() {
        let mmu = Mmu::new();
        assert!(!mmu.is_enabled());
    }

    #[test]
    fn test_decode_mair() {
        // Device-nGnRnE
        let (mem_type, _, _) = decode_mair_byte(0x00);
        assert_eq!(mem_type, MemoryType::Device(DeviceType::NGnRnE));

        // Normal, outer WB, inner WB
        let (mem_type, outer, inner) = decode_mair_byte(0xFF);
        assert_eq!(mem_type, MemoryType::Normal);
        assert_eq!(outer, Cacheability::WriteBack);
        assert_eq!(inner, Cacheability::WriteBack);

        // Normal, outer WT, inner WT
        let (mem_type, outer, inner) = decode_mair_byte(0xBB);
        assert_eq!(mem_type, MemoryType::Normal);
        assert_eq!(outer, Cacheability::WriteThrough);
        assert_eq!(inner, Cacheability::WriteThrough);
    }
}
