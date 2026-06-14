//! AArch64 (ARM64) CPU Emulation
//!
//! This module provides a comprehensive, production-grade AArch64 CPU emulator
//! supporting all exception levels (EL0-EL3), full MMU with page table walks,
//! GIC interrupt controller, and all ARMv8/v9 extensions.
//!
//! # Architecture Features
//!
//! - **Exception Levels**: Full EL0-EL3 support with proper privilege separation
//! - **MMU**: Complete 4-level page table walk with all translation granules
//! - **GIC**: GICv3 compatible interrupt controller
//! - **SIMD/FP**: Full NEON and floating-point support
//! - **Extensions**: LSE, PAC, BTI, MTE, RNG, and more
//!
//! # Usage
//!
//! ```ignore
//! use rax::arm::aarch64::{AArch64Cpu, AArch64Config};
//! use rax::arm::memory::FlatMemory;
//!
//! let memory = FlatMemory::new(0, 0x1000_0000);
//! let config = AArch64Config::default();
//! let mut cpu = AArch64Cpu::new(config, Box::new(memory));
//!
//! cpu.reset();
//! loop {
//!     match cpu.step() {
//!         Ok(CpuExit::Continue) => continue,
//!         Ok(exit) => { /* handle exit */ break; }
//!         Err(e) => { /* handle error */ break; }
//!     }
//! }
//! ```

pub mod cpu;
pub mod exceptions;
pub mod gic;
pub mod mmu;
pub mod sysregs;

pub use cpu::{AArch64Config, AArch64Cpu};
pub use exceptions::{ExceptionClass, ExceptionType, SyndromeRegister};
pub use gic::{Gic, GicConfig, GicVersion};
pub use mmu::{
    AddressDescriptor, Mmu, MmuConfig, PageTableEntry, TranslationFault, TranslationGranule,
    TranslationRegime,
};
pub use sysregs::{SystemRegisterBank, SystemRegisters};

// =============================================================================
// Constants
// =============================================================================

/// Number of general-purpose registers (X0-X30).
pub const NUM_GPRS: usize = 31;

/// Number of SIMD/FP registers (V0-V31).
pub const NUM_SIMD_REGS: usize = 32;

/// Number of exception levels.
pub const NUM_ELS: usize = 4;

/// Exception vector offsets from VBAR.
pub mod vector_offsets {
    /// Current EL with SP_EL0 - Synchronous.
    pub const CURR_EL_SP0_SYNC: u64 = 0x000;
    /// Current EL with SP_EL0 - IRQ.
    pub const CURR_EL_SP0_IRQ: u64 = 0x080;
    /// Current EL with SP_EL0 - FIQ.
    pub const CURR_EL_SP0_FIQ: u64 = 0x100;
    /// Current EL with SP_EL0 - SError.
    pub const CURR_EL_SP0_SERROR: u64 = 0x180;

    /// Current EL with SP_ELx - Synchronous.
    pub const CURR_EL_SPX_SYNC: u64 = 0x200;
    /// Current EL with SP_ELx - IRQ.
    pub const CURR_EL_SPX_IRQ: u64 = 0x280;
    /// Current EL with SP_ELx - FIQ.
    pub const CURR_EL_SPX_FIQ: u64 = 0x300;
    /// Current EL with SP_ELx - SError.
    pub const CURR_EL_SPX_SERROR: u64 = 0x380;

    /// Lower EL using AArch64 - Synchronous.
    pub const LOWER_EL_A64_SYNC: u64 = 0x400;
    /// Lower EL using AArch64 - IRQ.
    pub const LOWER_EL_A64_IRQ: u64 = 0x480;
    /// Lower EL using AArch64 - FIQ.
    pub const LOWER_EL_A64_FIQ: u64 = 0x500;
    /// Lower EL using AArch64 - SError.
    pub const LOWER_EL_A64_SERROR: u64 = 0x580;

    /// Lower EL using AArch32 - Synchronous.
    pub const LOWER_EL_A32_SYNC: u64 = 0x600;
    /// Lower EL using AArch32 - IRQ.
    pub const LOWER_EL_A32_IRQ: u64 = 0x680;
    /// Lower EL using AArch32 - FIQ.
    pub const LOWER_EL_A32_FIQ: u64 = 0x700;
    /// Lower EL using AArch32 - SError.
    pub const LOWER_EL_A32_SERROR: u64 = 0x780;
}

/// SCTLR_ELx bit definitions.
pub mod sctlr {
    /// MMU enable.
    pub const M: u64 = 1 << 0;
    /// Alignment check enable.
    pub const A: u64 = 1 << 1;
    /// Data cache enable.
    pub const C: u64 = 1 << 2;
    /// SP alignment check enable.
    pub const SA: u64 = 1 << 3;
    /// SP alignment check enable for EL0.
    pub const SA0: u64 = 1 << 4;
    /// CP15 barrier enable.
    pub const CP15BEN: u64 = 1 << 5;
    /// Non-aligned access (0=trap unaligned).
    pub const NAA: u64 = 1 << 6;
    /// IT disable.
    pub const ITD: u64 = 1 << 7;
    /// SETEND disable.
    pub const SED: u64 = 1 << 8;
    /// User mask access.
    pub const UMA: u64 = 1 << 9;
    /// Enable EL0 access to DC CVAU etc.
    pub const ENRCTX: u64 = 1 << 10;
    /// Exception exit is context synchronizing.
    pub const EOS: u64 = 1 << 11;
    /// Instruction cache enable.
    pub const I: u64 = 1 << 12;
    /// Enable pointer auth (APD).
    pub const ENDA: u64 = 1 << 27;
    /// Write permission implies execute-never (WXN).
    pub const WXN: u64 = 1 << 19;
    /// Exception entry is context synchronizing.
    pub const EIS: u64 = 1 << 22;
    /// Enable PAC for instruction addresses.
    pub const ENIA: u64 = 1 << 31;
}

/// HCR_EL2 bit definitions.
pub mod hcr {
    /// Virtualization enable.
    pub const VM: u64 = 1 << 0;
    /// Set/way invalidation override.
    pub const SWIO: u64 = 1 << 1;
    /// Protected table walk.
    pub const PTW: u64 = 1 << 2;
    /// Physical FIQ routing.
    pub const FMO: u64 = 1 << 3;
    /// Physical IRQ routing.
    pub const IMO: u64 = 1 << 4;
    /// Physical SError routing.
    pub const AMO: u64 = 1 << 5;
    /// Virtual FIQ.
    pub const VF: u64 = 1 << 6;
    /// Virtual IRQ.
    pub const VI: u64 = 1 << 7;
    /// Virtual SError.
    pub const VSE: u64 = 1 << 8;
    /// Force broadcast.
    pub const FB: u64 = 1 << 9;
    /// Barrier shareability upgrade.
    pub const BSU_MASK: u64 = 0b11 << 10;
    /// Default cacheable.
    pub const DC: u64 = 1 << 12;
    /// Trap WFI.
    pub const TWI: u64 = 1 << 13;
    /// Trap WFE.
    pub const TWE: u64 = 1 << 14;
    /// Trap ID.
    pub const TID0: u64 = 1 << 15;
    pub const TID1: u64 = 1 << 16;
    pub const TID2: u64 = 1 << 17;
    pub const TID3: u64 = 1 << 18;
    /// Trap SMC.
    pub const TSC: u64 = 1 << 19;
    /// Trap implementation defined registers.
    pub const TIDCP: u64 = 1 << 20;
    /// Trap cache maintenance.
    pub const TACR: u64 = 1 << 21;
    /// Trap system register accesses.
    pub const TSW: u64 = 1 << 22;
    pub const TPCP: u64 = 1 << 23;
    pub const TPU: u64 = 1 << 24;
    pub const TTLB: u64 = 1 << 25;
    pub const TVM: u64 = 1 << 26;
    pub const TGE: u64 = 1 << 27;
    pub const TDZ: u64 = 1 << 28;
    pub const HCD: u64 = 1 << 29;
    pub const TRVM: u64 = 1 << 30;
    /// Execution state control for lower EL (0=AArch64).
    pub const RW: u64 = 1 << 31;
    /// Stage 2 format indicator.
    pub const CD: u64 = 1 << 32;
    /// Stage 2 translation disabled.
    pub const ID: u64 = 1 << 33;
    /// EL2 Host Enable.
    pub const E2H: u64 = 1 << 34;
    /// Trap LOR registers.
    pub const TLOR: u64 = 1 << 35;
    /// Trap error record access.
    pub const TERR: u64 = 1 << 36;
    /// Trap Extended ERET.
    pub const TEA: u64 = 1 << 37;
    /// Monitor interrupt enable (with VHE).
    pub const MIOCNCE: u64 = 1 << 38;
    /// Trap pointer auth.
    pub const APK: u64 = 1 << 40;
    pub const API: u64 = 1 << 41;
    /// Nested virtualization.
    pub const NV: u64 = 1 << 42;
    pub const NV1: u64 = 1 << 43;
    pub const NV2: u64 = 1 << 45;
}

/// SCR_EL3 bit definitions.
pub mod scr {
    /// Non-secure bit.
    pub const NS: u64 = 1 << 0;
    /// IRQ routing.
    pub const IRQ: u64 = 1 << 1;
    /// FIQ routing.
    pub const FIQ: u64 = 1 << 2;
    /// External abort routing.
    pub const EA: u64 = 1 << 3;
    /// SMC disable.
    pub const SMD: u64 = 1 << 7;
    /// Hypervisor call enable.
    pub const HCE: u64 = 1 << 8;
    /// Secure instruction fetch.
    pub const SIF: u64 = 1 << 9;
    /// Execution state control for EL2 (0=AArch64).
    pub const RW: u64 = 1 << 10;
    /// Secure timer enable.
    pub const ST: u64 = 1 << 11;
    /// Trap WFE.
    pub const TWE: u64 = 1 << 13;
    /// Trap WFI.
    pub const TWI: u64 = 1 << 14;
    /// EL2 enable.
    pub const EEL2: u64 = 1 << 18;
}

/// SPSR bit definitions.
pub mod spsr {
    /// Mode mask (EL + SP select).
    pub const M_MASK: u64 = 0xF;
    /// EL0t.
    pub const EL0T: u64 = 0b0000;
    /// EL1t.
    pub const EL1T: u64 = 0b0100;
    /// EL1h.
    pub const EL1H: u64 = 0b0101;
    /// EL2t.
    pub const EL2T: u64 = 0b1000;
    /// EL2h.
    pub const EL2H: u64 = 0b1001;
    /// EL3t.
    pub const EL3T: u64 = 0b1100;
    /// EL3h.
    pub const EL3H: u64 = 0b1101;
    /// F (FIQ mask).
    pub const F: u64 = 1 << 6;
    /// I (IRQ mask).
    pub const I: u64 = 1 << 7;
    /// A (SError mask).
    pub const A: u64 = 1 << 8;
    /// D (Debug mask).
    pub const D: u64 = 1 << 9;
    /// BTYPE field.
    pub const BTYPE_MASK: u64 = 0b11 << 10;
    /// SSBS (Speculative Store Bypass Safe).
    pub const SSBS: u64 = 1 << 12;
    /// IL (Illegal execution state).
    pub const IL: u64 = 1 << 20;
    /// SS (Software step).
    pub const SS: u64 = 1 << 21;
    /// PAN (Privileged Access Never).
    pub const PAN: u64 = 1 << 22;
    /// UAO (User Access Override).
    pub const UAO: u64 = 1 << 23;
    /// DIT (Data Independent Timing).
    pub const DIT: u64 = 1 << 24;
    /// TCO (Tag Check Override).
    pub const TCO: u64 = 1 << 25;
    /// V (Overflow flag).
    pub const V: u64 = 1 << 28;
    /// C (Carry flag).
    pub const C: u64 = 1 << 29;
    /// Z (Zero flag).
    pub const Z: u64 = 1 << 30;
    /// N (Negative flag).
    pub const N: u64 = 1 << 31;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_offsets() {
        // Verify vector table is 2KB aligned and offsets are correct
        assert_eq!(vector_offsets::CURR_EL_SP0_SYNC, 0x000);
        assert_eq!(vector_offsets::LOWER_EL_A32_SERROR, 0x780);
        assert_eq!(vector_offsets::LOWER_EL_A32_SERROR + 0x80, 0x800); // End of table
    }

    #[test]
    fn test_spsr_modes() {
        assert_eq!(spsr::EL0T, 0b0000);
        assert_eq!(spsr::EL1T, 0b0100);
        assert_eq!(spsr::EL1H, 0b0101);
        assert_eq!(spsr::EL2T, 0b1000);
        assert_eq!(spsr::EL2H, 0b1001);
        assert_eq!(spsr::EL3T, 0b1100);
        assert_eq!(spsr::EL3H, 0b1101);
    }
}
