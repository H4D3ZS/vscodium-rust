//! AArch64 System Registers
//!
//! This module implements the complete set of AArch64 system registers
//! organized by exception level and functional group.

use std::collections::HashMap;

use crate::arm::sysreg::{Aarch64SysReg, Aarch64SysRegEncoding};

// =============================================================================
// System Register Bank (per-EL storage)
// =============================================================================

/// Per-exception-level system register bank.
#[derive(Clone, Debug)]
pub struct SystemRegisterBank {
    /// System Control Register.
    pub sctlr: u64,
    /// Auxiliary Control Register.
    pub actlr: u64,
    /// Coprocessor Access Control Register (EL1 only).
    pub cpacr: u64,
    /// Translation Table Base Register 0.
    pub ttbr0: u64,
    /// Translation Table Base Register 1 (EL1 only).
    pub ttbr1: u64,
    /// Translation Control Register.
    pub tcr: u64,
    /// Memory Attribute Indirection Register.
    pub mair: u64,
    /// Auxiliary Memory Attribute Indirection Register.
    pub amair: u64,
    /// Vector Base Address Register.
    pub vbar: u64,
    /// Context ID Register (EL1 only).
    pub contextidr: u64,
    /// Exception Syndrome Register.
    pub esr: u64,
    /// Fault Address Register.
    pub far: u64,
    /// Hypervisor IPA Fault Address Register (EL2 only).
    pub hpfar: u64,
    /// Exception Link Register.
    pub elr: u64,
    /// Saved Program Status Register.
    pub spsr: u64,
    /// Software Thread ID Register (EL0 readable).
    pub tpidr: u64,
    /// Thread ID Register (EL0 readable/writable).
    pub tpidrro: u64,
}

impl Default for SystemRegisterBank {
    fn default() -> Self {
        Self {
            sctlr: 0,
            actlr: 0,
            // CPACR_EL1: Enable FP/SIMD (FPEN=0b11) and SVE (ZEN=0b11) by default
            cpacr: (0b11 << 20) | (0b11 << 16),
            ttbr0: 0,
            ttbr1: 0,
            tcr: 0,
            mair: 0,
            amair: 0,
            vbar: 0,
            contextidr: 0,
            esr: 0,
            far: 0,
            hpfar: 0,
            elr: 0,
            spsr: 0,
            tpidr: 0,
            tpidrro: 0,
        }
    }
}

// =============================================================================
// Complete System Registers
// =============================================================================

/// Complete AArch64 system register state.
#[derive(Clone, Debug)]
pub struct SystemRegisters {
    // =========================================================================
    // Per-EL Register Banks
    // =========================================================================
    /// EL1 system registers.
    pub el1: SystemRegisterBank,
    /// EL2 system registers.
    pub el2: SystemRegisterBank,
    /// EL3 system registers.
    pub el3: SystemRegisterBank,

    // =========================================================================
    // EL0 Registers
    // =========================================================================
    /// Thread ID Register (EL0 read/write).
    pub tpidr_el0: u64,
    /// Thread Pointer (EL0 read-only).
    pub tpidrro_el0: u64,

    // =========================================================================
    // EL2 Hypervisor Registers
    // =========================================================================
    /// Hypervisor Configuration Register.
    pub hcr_el2: u64,
    /// Monitor Debug Configuration Register (EL2).
    pub mdcr_el2: u64,
    /// Counter-timer Hypervisor Control Register.
    pub cnthctl_el2: u64,
    /// Virtualization Translation Control Register.
    pub vtcr_el2: u64,
    /// Virtualization Translation Table Base Register.
    pub vttbr_el2: u64,
    /// Virtualization Multiprocessor ID Register.
    pub vmpidr_el2: u64,
    /// Virtualization Processor ID Register.
    pub vpidr_el2: u64,
    /// Counter-timer Virtual Offset.
    pub cntvoff_el2: u64,

    // =========================================================================
    // EL3 Secure Monitor Registers
    // =========================================================================
    /// Secure Configuration Register.
    pub scr_el3: u64,
    /// Monitor Debug Configuration Register (EL3).
    pub mdcr_el3: u64,
    /// Counter-timer Secure Virtual Timer Control Register.
    pub cntps_ctl_el1: u64,
    /// Counter-timer Secure Virtual Timer Compare Value.
    pub cntps_cval_el1: u64,
    /// Counter-timer Secure Virtual Timer Value.
    pub cntps_tval_el1: u64,

    // =========================================================================
    // ID Registers (Read-Only)
    // =========================================================================
    /// Main ID Register.
    pub midr_el1: u64,
    /// Multiprocessor Affinity Register.
    pub mpidr_el1: u64,
    /// Revision ID Register.
    pub revidr_el1: u64,
    /// Processor Feature Register 0.
    pub id_aa64pfr0_el1: u64,
    /// Processor Feature Register 1.
    pub id_aa64pfr1_el1: u64,
    /// Debug Feature Register 0.
    pub id_aa64dfr0_el1: u64,
    /// Debug Feature Register 1.
    pub id_aa64dfr1_el1: u64,
    /// Auxiliary Feature Register 0.
    pub id_aa64afr0_el1: u64,
    /// Auxiliary Feature Register 1.
    pub id_aa64afr1_el1: u64,
    /// Instruction Set Attribute Register 0.
    pub id_aa64isar0_el1: u64,
    /// Instruction Set Attribute Register 1.
    pub id_aa64isar1_el1: u64,
    /// Instruction Set Attribute Register 2.
    pub id_aa64isar2_el1: u64,
    /// Memory Model Feature Register 0.
    pub id_aa64mmfr0_el1: u64,
    /// Memory Model Feature Register 1.
    pub id_aa64mmfr1_el1: u64,
    /// Memory Model Feature Register 2.
    pub id_aa64mmfr2_el1: u64,
    /// Cache Level ID Register.
    pub clidr_el1: u64,
    /// Cache Type Register.
    pub ctr_el0: u64,
    /// Data Cache Zero ID Register.
    pub dczid_el0: u64,

    // =========================================================================
    // Timer Registers
    // =========================================================================
    /// Counter Frequency.
    pub cntfrq_el0: u64,
    /// Physical Count.
    pub cntpct_el0: u64,
    /// Virtual Count.
    pub cntvct_el0: u64,
    /// Physical Timer Control.
    pub cntp_ctl_el0: u64,
    /// Physical Timer Compare Value.
    pub cntp_cval_el0: u64,
    /// Physical Timer Value.
    pub cntp_tval_el0: u64,
    /// Virtual Timer Control.
    pub cntv_ctl_el0: u64,
    /// Virtual Timer Compare Value.
    pub cntv_cval_el0: u64,
    /// Virtual Timer Value.
    pub cntv_tval_el0: u64,
    /// Counter-timer Kernel Control Register.
    pub cntkctl_el1: u64,

    // =========================================================================
    // Debug Registers
    // =========================================================================
    /// Debug Status and Control Register.
    pub mdscr_el1: u64,
    /// OS Lock Data Transfer Register.
    pub osdtrrx_el1: u64,
    /// OS Lock Data Transfer Transmit Register.
    pub osdtrtx_el1: u64,
    /// OS Double Lock Register.
    pub osdlr_el1: u64,
    /// Debug Breakpoint Control Registers.
    pub dbgbcr: [u64; 16],
    /// Debug Breakpoint Value Registers.
    pub dbgbvr: [u64; 16],
    /// Debug Watchpoint Control Registers.
    pub dbgwcr: [u64; 16],
    /// Debug Watchpoint Value Registers.
    pub dbgwvr: [u64; 16],

    // =========================================================================
    // Performance Monitor Registers
    // =========================================================================
    /// Performance Monitors Control Register.
    pub pmcr_el0: u64,
    /// Performance Monitors Count Enable Set Register.
    pub pmcntenset_el0: u64,
    /// Performance Monitors Count Enable Clear Register.
    pub pmcntenclr_el0: u64,
    /// Performance Monitors Overflow Flag Status Register.
    pub pmovsset_el0: u64,
    /// Performance Monitors Overflow Flag Status Clear Register.
    pub pmovsclr_el0: u64,
    /// Performance Monitors Software Increment Register.
    pub pmswinc_el0: u64,
    /// Performance Monitors Event Counter Selection Register.
    pub pmselr_el0: u64,
    /// Performance Monitors Cycle Count Register.
    pub pmccntr_el0: u64,
    /// Performance Monitors Event Type Selection Register.
    pub pmxevtyper_el0: u64,
    /// Performance Monitors Event Count Register.
    pub pmxevcntr_el0: u64,
    /// Performance Monitors User Enable Register.
    pub pmuserenr_el0: u64,
    /// Performance Monitors Interrupt Enable Set Register.
    pub pmintenset_el1: u64,
    /// Performance Monitors Interrupt Enable Clear Register.
    pub pmintenclr_el1: u64,
    /// Performance Monitors Event Type Registers.
    pub pmevtyper: [u64; 31],
    /// Performance Monitors Event Count Registers.
    pub pmevcntr: [u64; 31],

    // =========================================================================
    // Pointer Authentication Registers
    // =========================================================================
    /// Pointer Authentication Key A (Data, low).
    pub apiakeylo_el1: u64,
    /// Pointer Authentication Key A (Data, high).
    pub apiakeyhi_el1: u64,
    /// Pointer Authentication Key B (Data, low).
    pub apibkeylo_el1: u64,
    /// Pointer Authentication Key B (Data, high).
    pub apibkeyhi_el1: u64,
    /// Pointer Authentication Key A (Instruction, low).
    pub apdakeylo_el1: u64,
    /// Pointer Authentication Key A (Instruction, high).
    pub apdakeyhi_el1: u64,
    /// Pointer Authentication Key B (Instruction, low).
    pub apdbkeylo_el1: u64,
    /// Pointer Authentication Key B (Instruction, high).
    pub apdbkeyhi_el1: u64,
    /// Pointer Authentication Key (Generic, low).
    pub apgakeylo_el1: u64,
    /// Pointer Authentication Key (Generic, high).
    pub apgakeyhi_el1: u64,

    // =========================================================================
    // Memory Tagging Extension Registers
    // =========================================================================
    /// Tag Control Register (EL0).
    pub tco: bool,
    /// Tag Check Override (GCR_EL1).
    pub gcr_el1: u64,
    /// Random Allocation Tag (RGSR_EL1).
    pub rgsr_el1: u64,
    /// Tag Fault Status Register (TFSR_EL1).
    pub tfsr_el1: u64,
    /// Tag Fault Status Register (TFSR_EL2).
    pub tfsr_el2: u64,
    /// Tag Fault Status Register (TFSR_EL3).
    pub tfsr_el3: u64,

    // =========================================================================
    // RAS (Reliability, Availability, Serviceability) Registers
    // =========================================================================
    /// Error Record Feature Register.
    pub erridr_el1: u64,
    /// Error Record Select Register.
    pub errselr_el1: u64,
    /// Error Record Primary Record.
    pub erxfr_el1: u64,
    /// Error Record Control.
    pub erxctlr_el1: u64,
    /// Error Record Status.
    pub erxstatus_el1: u64,
    /// Error Record Address.
    pub erxaddr_el1: u64,
    /// Error Record Miscellaneous 0.
    pub erxmisc0_el1: u64,
    /// Error Record Miscellaneous 1.
    pub erxmisc1_el1: u64,

    // =========================================================================
    // Activity Monitors Registers
    // =========================================================================
    /// Activity Monitors Counter Group Configuration Register 0.
    pub amcgcr_el0: u64,
    /// Activity Monitors Counter Group 0 Enable Register.
    pub amcg0en_el0: u64,
    /// Activity Monitors User Enable Register.
    pub amuserenr_el0: u64,
    /// Activity Monitors Event Counter Registers.
    pub amevcntr0: [u64; 16],
    pub amevcntr1: [u64; 16],
    /// Activity Monitors Event Type Registers.
    pub amevtyper0: [u64; 16],
    pub amevtyper1: [u64; 16],

    // =========================================================================
    // Random Number Generator Registers
    // =========================================================================
    /// Random Number.
    pub rndr: u64,
    /// Reseeded Random Number.
    pub rndrrs: u64,

    // =========================================================================
    // Speculative Store Bypass Safe
    // =========================================================================
    /// Speculation restriction.
    pub ssbs: bool,

    // =========================================================================
    // Data Independent Timing
    // =========================================================================
    /// Data independent timing.
    pub dit: bool,

    // =========================================================================
    // Implementation Defined Registers (generic storage)
    // =========================================================================
    /// Implementation defined register storage.
    pub impl_defined: HashMap<u16, u64>,
}

impl Default for SystemRegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemRegisters {
    /// Create a new system register state with reset values.
    pub fn new() -> Self {
        Self {
            el1: SystemRegisterBank::default(),
            el2: SystemRegisterBank::default(),
            el3: SystemRegisterBank::default(),

            tpidr_el0: 0,
            tpidrro_el0: 0,

            hcr_el2: 0,
            mdcr_el2: 0,
            cnthctl_el2: 0,
            vtcr_el2: 0,
            vttbr_el2: 0,
            vmpidr_el2: 0,
            vpidr_el2: 0,
            cntvoff_el2: 0,

            scr_el3: 0,
            mdcr_el3: 0,
            cntps_ctl_el1: 0,
            cntps_cval_el1: 0,
            cntps_tval_el1: 0,

            // ID registers with typical Cortex-A values
            midr_el1: 0x410F_D083,  // Cortex-A53 r0p3
            mpidr_el1: 0x8000_0000, // Uniprocessor, Aff0=0
            revidr_el1: 0,
            // EL0-EL3 (AArch64+AArch32), FP + AdvSIMD (with FP16), GICv3
            // system register interface. SVE/SEL2/RAS deliberately not
            // advertised: their system registers are not implemented, and a
            // booting kernel probes whatever the ID registers claim.
            id_aa64pfr0_el1: 0x0000_0000_0112_2222,
            id_aa64pfr1_el1: 0,
            id_aa64dfr0_el1: 0x0000_0000_0101_0106, // 6 BPs, 4 WPs
            id_aa64dfr1_el1: 0,
            id_aa64afr0_el1: 0,
            id_aa64afr1_el1: 0,
            id_aa64isar0_el1: 0x0011_1111_1111_1111, // AES, SHA, CRC, atomics
            id_aa64isar1_el1: 0x0011_0011_1111_1111, // PAC, BTI, etc.
            id_aa64isar2_el1: 0,
            id_aa64mmfr0_el1: 0x0000_0000_0000_1122, // 4KB/16KB/64KB granules, 48-bit PA
            id_aa64mmfr1_el1: 0,
            id_aa64mmfr2_el1: 0,
            clidr_el1: 0x0A20_0023, // L1 I+D, L2 unified
            ctr_el0: 0x8444_C004,   // 64-byte cache lines
            dczid_el0: 0x0000_0004, // 64-byte DC ZVA block

            cntfrq_el0: 62_500_000, // 62.5 MHz typical
            cntpct_el0: 0,
            cntvct_el0: 0,
            cntp_ctl_el0: 0,
            cntp_cval_el0: 0,
            cntp_tval_el0: 0,
            cntv_ctl_el0: 0,
            cntv_cval_el0: 0,
            cntv_tval_el0: 0,
            cntkctl_el1: 0,

            mdscr_el1: 0,
            osdtrrx_el1: 0,
            osdtrtx_el1: 0,
            osdlr_el1: 0,
            dbgbcr: [0; 16],
            dbgbvr: [0; 16],
            dbgwcr: [0; 16],
            dbgwvr: [0; 16],

            pmcr_el0: 0,
            pmcntenset_el0: 0,
            pmcntenclr_el0: 0,
            pmovsset_el0: 0,
            pmovsclr_el0: 0,
            pmswinc_el0: 0,
            pmselr_el0: 0,
            pmccntr_el0: 0,
            pmxevtyper_el0: 0,
            pmxevcntr_el0: 0,
            pmuserenr_el0: 0,
            pmintenset_el1: 0,
            pmintenclr_el1: 0,
            pmevtyper: [0; 31],
            pmevcntr: [0; 31],

            apiakeylo_el1: 0,
            apiakeyhi_el1: 0,
            apibkeylo_el1: 0,
            apibkeyhi_el1: 0,
            apdakeylo_el1: 0,
            apdakeyhi_el1: 0,
            apdbkeylo_el1: 0,
            apdbkeyhi_el1: 0,
            apgakeylo_el1: 0,
            apgakeyhi_el1: 0,

            tco: false,
            gcr_el1: 0,
            rgsr_el1: 0,
            tfsr_el1: 0,
            tfsr_el2: 0,
            tfsr_el3: 0,

            erridr_el1: 0,
            errselr_el1: 0,
            erxfr_el1: 0,
            erxctlr_el1: 0,
            erxstatus_el1: 0,
            erxaddr_el1: 0,
            erxmisc0_el1: 0,
            erxmisc1_el1: 0,

            amcgcr_el0: 0,
            amcg0en_el0: 0,
            amuserenr_el0: 0,
            amevcntr0: [0; 16],
            amevcntr1: [0; 16],
            amevtyper0: [0; 16],
            amevtyper1: [0; 16],

            rndr: 0,
            rndrrs: 0,

            ssbs: false,
            dit: false,

            impl_defined: HashMap::new(),
        }
    }

    /// Reset all registers to their default values.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get register bank for a given exception level.
    pub fn bank(&self, el: u8) -> &SystemRegisterBank {
        match el {
            1 => &self.el1,
            2 => &self.el2,
            3 => &self.el3,
            _ => &self.el1, // EL0 uses EL1 bank for most registers
        }
    }

    /// Get mutable register bank for a given exception level.
    pub fn bank_mut(&mut self, el: u8) -> &mut SystemRegisterBank {
        match el {
            1 => &mut self.el1,
            2 => &mut self.el2,
            3 => &mut self.el3,
            _ => &mut self.el1,
        }
    }

    /// Read a system register by encoding.
    pub fn read(&self, encoding: Aarch64SysRegEncoding, current_el: u8) -> Option<u64> {
        let enc = encoding.encode();

        // Handle known system registers
        match (
            encoding.op0,
            encoding.op1,
            encoding.crn,
            encoding.crm,
            encoding.op2,
        ) {
            // MIDR_EL1
            (3, 0, 0, 0, 0) => Some(self.midr_el1),
            // MPIDR_EL1
            (3, 0, 0, 0, 5) => Some(self.mpidr_el1),
            // REVIDR_EL1
            (3, 0, 0, 0, 6) => Some(self.revidr_el1),

            // ID_AA64PFR0_EL1
            (3, 0, 0, 4, 0) => Some(self.id_aa64pfr0_el1),
            // ID_AA64PFR1_EL1
            (3, 0, 0, 4, 1) => Some(self.id_aa64pfr1_el1),
            // ID_AA64DFR0_EL1
            (3, 0, 0, 5, 0) => Some(self.id_aa64dfr0_el1),
            // ID_AA64DFR1_EL1
            (3, 0, 0, 5, 1) => Some(self.id_aa64dfr1_el1),
            // ID_AA64ISAR0_EL1
            (3, 0, 0, 6, 0) => Some(self.id_aa64isar0_el1),
            // ID_AA64ISAR1_EL1
            (3, 0, 0, 6, 1) => Some(self.id_aa64isar1_el1),
            // ID_AA64ISAR2_EL1
            (3, 0, 0, 6, 2) => Some(self.id_aa64isar2_el1),
            // ID_AA64MMFR0_EL1
            (3, 0, 0, 7, 0) => Some(self.id_aa64mmfr0_el1),
            // ID_AA64MMFR1_EL1
            (3, 0, 0, 7, 1) => Some(self.id_aa64mmfr1_el1),
            // ID_AA64MMFR2_EL1
            (3, 0, 0, 7, 2) => Some(self.id_aa64mmfr2_el1),

            // SCTLR_EL1
            (3, 0, 1, 0, 0) => Some(self.el1.sctlr),
            // SCTLR_EL2
            (3, 4, 1, 0, 0) => Some(self.el2.sctlr),
            // SCTLR_EL3
            (3, 6, 1, 0, 0) => Some(self.el3.sctlr),

            // CPACR_EL1
            (3, 0, 1, 0, 2) => Some(self.el1.cpacr),

            // TTBR0_EL1
            (3, 0, 2, 0, 0) => Some(self.el1.ttbr0),
            // TTBR1_EL1
            (3, 0, 2, 0, 1) => Some(self.el1.ttbr1),
            // TTBR0_EL2
            (3, 4, 2, 0, 0) => Some(self.el2.ttbr0),
            // TTBR0_EL3
            (3, 6, 2, 0, 0) => Some(self.el3.ttbr0),

            // TCR_EL1
            (3, 0, 2, 0, 2) => Some(self.el1.tcr),
            // TCR_EL2
            (3, 4, 2, 0, 2) => Some(self.el2.tcr),
            // TCR_EL3
            (3, 6, 2, 0, 2) => Some(self.el3.tcr),

            // MAIR_EL1
            (3, 0, 10, 2, 0) => Some(self.el1.mair),
            // MAIR_EL2
            (3, 4, 10, 2, 0) => Some(self.el2.mair),
            // MAIR_EL3
            (3, 6, 10, 2, 0) => Some(self.el3.mair),

            // VBAR_EL1
            (3, 0, 12, 0, 0) => Some(self.el1.vbar),
            // VBAR_EL2
            (3, 4, 12, 0, 0) => Some(self.el2.vbar),
            // VBAR_EL3
            (3, 6, 12, 0, 0) => Some(self.el3.vbar),

            // ESR_EL1
            (3, 0, 5, 2, 0) => Some(self.el1.esr),
            // ESR_EL2
            (3, 4, 5, 2, 0) => Some(self.el2.esr),
            // ESR_EL3
            (3, 6, 5, 2, 0) => Some(self.el3.esr),

            // FAR_EL1
            (3, 0, 6, 0, 0) => Some(self.el1.far),
            // FAR_EL2
            (3, 4, 6, 0, 0) => Some(self.el2.far),
            // FAR_EL3
            (3, 6, 6, 0, 0) => Some(self.el3.far),

            // ELR_EL1
            (3, 0, 4, 0, 1) => Some(self.el1.elr),
            // ELR_EL2
            (3, 4, 4, 0, 1) => Some(self.el2.elr),
            // ELR_EL3
            (3, 6, 4, 0, 1) => Some(self.el3.elr),

            // SPSR_EL1
            (3, 0, 4, 0, 0) => Some(self.el1.spsr),
            // SPSR_EL2
            (3, 4, 4, 0, 0) => Some(self.el2.spsr),
            // SPSR_EL3
            (3, 6, 4, 0, 0) => Some(self.el3.spsr),

            // SP_EL0 (special handling needed in CPU)
            (3, 0, 4, 1, 0) => None, // Handled by CPU

            // TPIDR_EL0
            (3, 3, 13, 0, 2) => Some(self.tpidr_el0),
            // TPIDRRO_EL0
            (3, 3, 13, 0, 3) => Some(self.tpidrro_el0),
            // TPIDR_EL1
            (3, 0, 13, 0, 4) => Some(self.el1.tpidr),

            // HCR_EL2
            (3, 4, 1, 1, 0) => Some(self.hcr_el2),
            // VTCR_EL2
            (3, 4, 2, 1, 2) => Some(self.vtcr_el2),
            // VTTBR_EL2
            (3, 4, 2, 1, 0) => Some(self.vttbr_el2),

            // SCR_EL3
            (3, 6, 1, 1, 0) => Some(self.scr_el3),

            // Timer registers
            // CNTKCTL_EL1
            (3, 0, 14, 1, 0) => Some(self.cntkctl_el1),
            // CNTFRQ_EL0
            (3, 3, 14, 0, 0) => Some(self.cntfrq_el0),
            // CNTPCT_EL0
            (3, 3, 14, 0, 1) => Some(self.cntpct_el0),
            // CNTVCT_EL0
            (3, 3, 14, 0, 2) => Some(self.cntvct_el0),
            // CNTP_CTL_EL0 (ISTATUS computed from the live counter)
            (3, 3, 14, 2, 1) => {
                let mut ctl = self.cntp_ctl_el0 & 0x3;
                if (ctl & 1) != 0 && self.cntpct_el0 >= self.cntp_cval_el0 {
                    ctl |= 4;
                }
                Some(ctl)
            }
            // CNTP_CVAL_EL0
            (3, 3, 14, 2, 2) => Some(self.cntp_cval_el0),
            // CNTP_TVAL_EL0 = bits(32) of (CVAL - count)
            (3, 3, 14, 2, 0) => {
                Some(self.cntp_cval_el0.wrapping_sub(self.cntpct_el0) & 0xFFFF_FFFF)
            }
            // CNTV_CTL_EL0 (ISTATUS computed from the live counter)
            (3, 3, 14, 3, 1) => {
                let mut ctl = self.cntv_ctl_el0 & 0x3;
                if (ctl & 1) != 0 && self.cntvct_el0 >= self.cntv_cval_el0 {
                    ctl |= 4;
                }
                Some(ctl)
            }
            // CNTV_CVAL_EL0
            (3, 3, 14, 3, 2) => Some(self.cntv_cval_el0),
            // CNTV_TVAL_EL0 = bits(32) of (CVAL - count)
            (3, 3, 14, 3, 0) => {
                Some(self.cntv_cval_el0.wrapping_sub(self.cntvct_el0) & 0xFFFF_FFFF)
            }

            // CTR_EL0
            (3, 3, 0, 0, 1) => Some(self.ctr_el0),
            // DCZID_EL0
            (3, 3, 0, 0, 7) => Some(self.dczid_el0),
            // CLIDR_EL1
            (3, 1, 0, 0, 1) => Some(self.clidr_el1),

            // MDSCR_EL1
            (2, 0, 0, 2, 2) => Some(self.mdscr_el1),

            // PAC keys
            (3, 0, 2, 1, 0) => Some(self.apiakeylo_el1),
            (3, 0, 2, 1, 1) => Some(self.apiakeyhi_el1),
            (3, 0, 2, 1, 2) => Some(self.apibkeylo_el1),
            (3, 0, 2, 1, 3) => Some(self.apibkeyhi_el1),
            (3, 0, 2, 2, 0) => Some(self.apdakeylo_el1),
            (3, 0, 2, 2, 1) => Some(self.apdakeyhi_el1),
            (3, 0, 2, 2, 2) => Some(self.apdbkeylo_el1),
            (3, 0, 2, 2, 3) => Some(self.apdbkeyhi_el1),
            (3, 0, 2, 3, 0) => Some(self.apgakeylo_el1),
            (3, 0, 2, 3, 1) => Some(self.apgakeyhi_el1),

            // RNDR
            (3, 3, 2, 4, 0) => Some(self.rndr),
            // RNDRRS
            (3, 3, 2, 4, 1) => Some(self.rndrrs),

            // Check implementation defined storage
            _ => self.impl_defined.get(&enc).copied(),
        }
    }

    /// Write a system register by encoding.
    pub fn write(&mut self, encoding: Aarch64SysRegEncoding, value: u64, current_el: u8) -> bool {
        match (
            encoding.op0,
            encoding.op1,
            encoding.crn,
            encoding.crm,
            encoding.op2,
        ) {
            // SCTLR_EL1
            (3, 0, 1, 0, 0) => {
                self.el1.sctlr = value;
                true
            }
            // SCTLR_EL2
            (3, 4, 1, 0, 0) => {
                self.el2.sctlr = value;
                true
            }
            // SCTLR_EL3
            (3, 6, 1, 0, 0) => {
                self.el3.sctlr = value;
                true
            }

            // CPACR_EL1
            (3, 0, 1, 0, 2) => {
                self.el1.cpacr = value;
                true
            }

            // TTBR0_EL1
            (3, 0, 2, 0, 0) => {
                self.el1.ttbr0 = value;
                true
            }
            // TTBR1_EL1
            (3, 0, 2, 0, 1) => {
                self.el1.ttbr1 = value;
                true
            }
            // TTBR0_EL2
            (3, 4, 2, 0, 0) => {
                self.el2.ttbr0 = value;
                true
            }
            // TTBR0_EL3
            (3, 6, 2, 0, 0) => {
                self.el3.ttbr0 = value;
                true
            }

            // TCR_EL1
            (3, 0, 2, 0, 2) => {
                self.el1.tcr = value;
                true
            }
            // TCR_EL2
            (3, 4, 2, 0, 2) => {
                self.el2.tcr = value;
                true
            }
            // TCR_EL3
            (3, 6, 2, 0, 2) => {
                self.el3.tcr = value;
                true
            }

            // MAIR_EL1
            (3, 0, 10, 2, 0) => {
                self.el1.mair = value;
                true
            }
            // MAIR_EL2
            (3, 4, 10, 2, 0) => {
                self.el2.mair = value;
                true
            }
            // MAIR_EL3
            (3, 6, 10, 2, 0) => {
                self.el3.mair = value;
                true
            }

            // VBAR_EL1
            (3, 0, 12, 0, 0) => {
                self.el1.vbar = value;
                true
            }
            // VBAR_EL2
            (3, 4, 12, 0, 0) => {
                self.el2.vbar = value;
                true
            }
            // VBAR_EL3
            (3, 6, 12, 0, 0) => {
                self.el3.vbar = value;
                true
            }

            // ESR_EL1
            (3, 0, 5, 2, 0) => {
                self.el1.esr = value;
                true
            }
            // ESR_EL2
            (3, 4, 5, 2, 0) => {
                self.el2.esr = value;
                true
            }
            // ESR_EL3
            (3, 6, 5, 2, 0) => {
                self.el3.esr = value;
                true
            }

            // FAR_EL1
            (3, 0, 6, 0, 0) => {
                self.el1.far = value;
                true
            }
            // FAR_EL2
            (3, 4, 6, 0, 0) => {
                self.el2.far = value;
                true
            }
            // FAR_EL3
            (3, 6, 6, 0, 0) => {
                self.el3.far = value;
                true
            }

            // ELR_EL1
            (3, 0, 4, 0, 1) => {
                self.el1.elr = value;
                true
            }
            // ELR_EL2
            (3, 4, 4, 0, 1) => {
                self.el2.elr = value;
                true
            }
            // ELR_EL3
            (3, 6, 4, 0, 1) => {
                self.el3.elr = value;
                true
            }

            // SPSR_EL1
            (3, 0, 4, 0, 0) => {
                self.el1.spsr = value;
                true
            }
            // SPSR_EL2
            (3, 4, 4, 0, 0) => {
                self.el2.spsr = value;
                true
            }
            // SPSR_EL3
            (3, 6, 4, 0, 0) => {
                self.el3.spsr = value;
                true
            }

            // TPIDR_EL0
            (3, 3, 13, 0, 2) => {
                self.tpidr_el0 = value;
                true
            }
            // TPIDRRO_EL0 (usually read-only from EL0)
            (3, 3, 13, 0, 3) if current_el > 0 => {
                self.tpidrro_el0 = value;
                true
            }
            // TPIDR_EL1
            (3, 0, 13, 0, 4) => {
                self.el1.tpidr = value;
                true
            }

            // HCR_EL2
            (3, 4, 1, 1, 0) => {
                self.hcr_el2 = value;
                true
            }
            // VTCR_EL2
            (3, 4, 2, 1, 2) => {
                self.vtcr_el2 = value;
                true
            }
            // VTTBR_EL2
            (3, 4, 2, 1, 0) => {
                self.vttbr_el2 = value;
                true
            }

            // SCR_EL3
            (3, 6, 1, 1, 0) => {
                self.scr_el3 = value;
                true
            }

            // Timer registers
            // CNTKCTL_EL1
            (3, 0, 14, 1, 0) => {
                self.cntkctl_el1 = value;
                true
            }
            (3, 3, 14, 0, 0) if current_el >= 1 => {
                self.cntfrq_el0 = value;
                true
            }
            // CNTP_CTL_EL0: only ENABLE and IMASK are writable
            (3, 3, 14, 2, 1) => {
                self.cntp_ctl_el0 = value & 0x3;
                true
            }
            (3, 3, 14, 2, 2) => {
                self.cntp_cval_el0 = value;
                true
            }
            // CNTP_TVAL_EL0 write: CVAL = count + SignExtend(value<31:0>)
            (3, 3, 14, 2, 0) => {
                self.cntp_cval_el0 =
                    self.cntpct_el0.wrapping_add(value as u32 as i32 as i64 as u64);
                true
            }
            // CNTV_CTL_EL0: only ENABLE and IMASK are writable
            (3, 3, 14, 3, 1) => {
                self.cntv_ctl_el0 = value & 0x3;
                true
            }
            (3, 3, 14, 3, 2) => {
                self.cntv_cval_el0 = value;
                true
            }
            // CNTV_TVAL_EL0 write: CVAL = count + SignExtend(value<31:0>)
            (3, 3, 14, 3, 0) => {
                self.cntv_cval_el0 =
                    self.cntvct_el0.wrapping_add(value as u32 as i32 as i64 as u64);
                true
            }

            // MDSCR_EL1
            (2, 0, 0, 2, 2) => {
                self.mdscr_el1 = value;
                true
            }

            // PAC keys
            (3, 0, 2, 1, 0) => {
                self.apiakeylo_el1 = value;
                true
            }
            (3, 0, 2, 1, 1) => {
                self.apiakeyhi_el1 = value;
                true
            }
            (3, 0, 2, 1, 2) => {
                self.apibkeylo_el1 = value;
                true
            }
            (3, 0, 2, 1, 3) => {
                self.apibkeyhi_el1 = value;
                true
            }
            (3, 0, 2, 2, 0) => {
                self.apdakeylo_el1 = value;
                true
            }
            (3, 0, 2, 2, 1) => {
                self.apdakeyhi_el1 = value;
                true
            }
            (3, 0, 2, 2, 2) => {
                self.apdbkeylo_el1 = value;
                true
            }
            (3, 0, 2, 2, 3) => {
                self.apdbkeyhi_el1 = value;
                true
            }
            (3, 0, 2, 3, 0) => {
                self.apgakeylo_el1 = value;
                true
            }
            (3, 0, 2, 3, 1) => {
                self.apgakeyhi_el1 = value;
                true
            }

            // MTE registers
            (3, 0, 5, 6, 1) => {
                self.tfsr_el1 = value;
                true
            }
            (3, 4, 5, 6, 1) => {
                self.tfsr_el2 = value;
                true
            }
            (3, 6, 5, 6, 1) => {
                self.tfsr_el3 = value;
                true
            }
            (3, 0, 1, 0, 6) => {
                self.gcr_el1 = value;
                true
            }
            (3, 0, 1, 0, 5) => {
                self.rgsr_el1 = value;
                true
            }

            // Store in implementation defined
            _ => {
                let enc = encoding.encode();
                self.impl_defined.insert(enc, value);
                true
            }
        }
    }

    /// Tick timers (call once per cycle or instruction).
    pub fn tick_timers(&mut self, cycles: u64) {
        self.cntpct_el0 = self.cntpct_el0.wrapping_add(cycles);
        self.cntvct_el0 = self.cntpct_el0.wrapping_sub(self.cntvoff_el2);
    }

    /// Check if physical timer interrupt is pending.
    pub fn cntp_interrupt_pending(&self) -> bool {
        let ctl = self.cntp_ctl_el0;
        let enable = (ctl & 1) != 0;
        let imask = (ctl & 2) != 0;
        let istatus = (ctl & 4) != 0;

        enable && !imask && (istatus || self.cntpct_el0 >= self.cntp_cval_el0)
    }

    /// Check if virtual timer interrupt is pending.
    pub fn cntv_interrupt_pending(&self) -> bool {
        let ctl = self.cntv_ctl_el0;
        let enable = (ctl & 1) != 0;
        let imask = (ctl & 2) != 0;
        let istatus = (ctl & 4) != 0;

        enable && !imask && (istatus || self.cntvct_el0 >= self.cntv_cval_el0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysregs_default() {
        let regs = SystemRegisters::new();
        assert_eq!(regs.midr_el1, 0x410F_D083);
        assert_eq!(regs.cntfrq_el0, 62_500_000);
    }

    #[test]
    fn test_sysregs_read_write() {
        let mut regs = SystemRegisters::new();

        // Write SCTLR_EL1
        let enc = Aarch64SysRegEncoding::new(3, 0, 1, 0, 0);
        assert!(regs.write(enc, 0x12345678, 1));
        assert_eq!(regs.read(enc, 1), Some(0x12345678));

        // Write VBAR_EL1
        let enc = Aarch64SysRegEncoding::new(3, 0, 12, 0, 0);
        assert!(regs.write(enc, 0xFFFF_0000_0000_0000, 1));
        assert_eq!(regs.read(enc, 1), Some(0xFFFF_0000_0000_0000));
    }

    #[test]
    fn test_timer_tick() {
        let mut regs = SystemRegisters::new();

        regs.tick_timers(100);
        assert_eq!(regs.cntpct_el0, 100);
        assert_eq!(regs.cntvct_el0, 100);

        regs.cntvoff_el2 = 50;
        regs.tick_timers(100);
        assert_eq!(regs.cntpct_el0, 200);
        assert_eq!(regs.cntvct_el0, 150);
    }

    #[test]
    fn test_timer_interrupt() {
        let mut regs = SystemRegisters::new();

        // Timer disabled
        assert!(!regs.cntp_interrupt_pending());

        // Enable timer, set compare value
        regs.cntp_ctl_el0 = 1; // Enable
        regs.cntp_cval_el0 = 100;

        // Not yet triggered
        regs.cntpct_el0 = 50;
        assert!(!regs.cntp_interrupt_pending());

        // Triggered
        regs.cntpct_el0 = 100;
        assert!(regs.cntp_interrupt_pending());

        // Masked
        regs.cntp_ctl_el0 = 3; // Enable + mask
        assert!(!regs.cntp_interrupt_pending());
    }
}
