//! ARM system register encoding and decoding.
//!
//! This module provides lookup tables and functions for encoding/decoding
//! system registers for both AArch32 (CP15) and AArch64.
//!
//! # AArch32 CP15 Encoding
//!
//! CP15 registers are accessed via MCR/MRC instructions with:
//! - CRn: Primary register number (0-15)
//! - Op1: Opcode 1 (0-7)
//! - CRm: Secondary register number (0-15)
//! - Op2: Opcode 2 (0-7)
//!
//! # AArch64 System Register Encoding
//!
//! System registers are accessed via MSR/MRS instructions with:
//! - Op0: 2 bits (2 or 3)
//! - Op1: 3 bits (0-7)
//! - CRn: 4 bits (0-15)
//! - CRm: 4 bits (0-15)
//! - Op2: 3 bits (0-7)

use std::fmt;

// =============================================================================
// AArch32 CP15 Encoding
// =============================================================================

/// CP15 coprocessor register encoding for AArch32.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cp15Encoding {
    /// CRn - primary register number (0-15).
    pub crn: u8,
    /// Op1 - opcode 1 (0-7).
    pub op1: u8,
    /// CRm - secondary register number (0-15).
    pub crm: u8,
    /// Op2 - opcode 2 (0-7).
    pub op2: u8,
}

impl Cp15Encoding {
    /// Create a new CP15 encoding.
    pub const fn new(crn: u8, op1: u8, crm: u8, op2: u8) -> Self {
        Cp15Encoding { crn, op1, crm, op2 }
    }

    /// Encode as a 16-bit value (for lookup tables).
    pub const fn encode(&self) -> u16 {
        ((self.crn as u16) << 12)
            | ((self.op1 as u16) << 9)
            | ((self.crm as u16) << 5)
            | ((self.op2 as u16) << 2)
    }

    /// Decode from a 16-bit value.
    pub const fn decode(value: u16) -> Self {
        Cp15Encoding {
            crn: ((value >> 12) & 0xF) as u8,
            op1: ((value >> 9) & 0x7) as u8,
            crm: ((value >> 5) & 0xF) as u8,
            op2: ((value >> 2) & 0x7) as u8,
        }
    }
}

impl fmt::Display for Cp15Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CP15 c{}, {}, c{}, {}",
            self.crn, self.op1, self.crm, self.op2
        )
    }
}

// =============================================================================
// AArch64 System Register Encoding
// =============================================================================

/// AArch64 system register encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Aarch64SysRegEncoding {
    /// Op0 (2-3).
    pub op0: u8,
    /// Op1 (0-7).
    pub op1: u8,
    /// CRn (0-15).
    pub crn: u8,
    /// CRm (0-15).
    pub crm: u8,
    /// Op2 (0-7).
    pub op2: u8,
}

impl Aarch64SysRegEncoding {
    /// Create a new system register encoding.
    pub const fn new(op0: u8, op1: u8, crn: u8, crm: u8, op2: u8) -> Self {
        Aarch64SysRegEncoding {
            op0,
            op1,
            crn,
            crm,
            op2,
        }
    }

    /// Encode as a 16-bit value.
    /// Format: op0[15:14] op1[13:11] CRn[10:7] CRm[6:3] op2[2:0]
    pub const fn encode(&self) -> u16 {
        ((self.op0 as u16 & 0x3) << 14)
            | ((self.op1 as u16 & 0x7) << 11)
            | ((self.crn as u16 & 0xF) << 7)
            | ((self.crm as u16 & 0xF) << 3)
            | (self.op2 as u16 & 0x7)
    }

    /// Decode from a 16-bit value.
    pub const fn decode(value: u16) -> Self {
        Aarch64SysRegEncoding {
            op0: ((value >> 14) & 0x3) as u8,
            op1: ((value >> 11) & 0x7) as u8,
            crn: ((value >> 7) & 0xF) as u8,
            crm: ((value >> 3) & 0xF) as u8,
            op2: (value & 0x7) as u8,
        }
    }

    /// Encode for MSR/MRS instruction (bits 20:5 of the instruction).
    pub const fn msr_encoding(&self) -> u32 {
        ((self.op0 as u32 & 0x3) << 19)
            | ((self.op1 as u32 & 0x7) << 16)
            | ((self.crn as u32 & 0xF) << 12)
            | ((self.crm as u32 & 0xF) << 8)
            | ((self.op2 as u32 & 0x7) << 5)
    }
}

impl fmt::Display for Aarch64SysRegEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S{}_{}_C{}_C{}_{}",
            self.op0, self.op1, self.crn, self.crm, self.op2
        )
    }
}

// =============================================================================
// AArch64 System Register Definitions
// =============================================================================

/// AArch64 system register identifiers.
///
/// This enum defines known system registers with their encodings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Aarch64SysReg {
    // =========================================================================
    // ID Registers (Op0=3, Op1=0, CRn=0)
    // =========================================================================
    /// Main ID Register.
    MIDR_EL1,
    /// Multiprocessor Affinity Register.
    MPIDR_EL1,
    /// Revision ID Register.
    REVIDR_EL1,
    /// Processor Feature Register 0.
    ID_AA64PFR0_EL1,
    /// Processor Feature Register 1.
    ID_AA64PFR1_EL1,
    /// Debug Feature Register 0.
    ID_AA64DFR0_EL1,
    /// Debug Feature Register 1.
    ID_AA64DFR1_EL1,
    /// Instruction Set Attribute Register 0.
    ID_AA64ISAR0_EL1,
    /// Instruction Set Attribute Register 1.
    ID_AA64ISAR1_EL1,
    /// Instruction Set Attribute Register 2.
    ID_AA64ISAR2_EL1,
    /// Memory Model Feature Register 0.
    ID_AA64MMFR0_EL1,
    /// Memory Model Feature Register 1.
    ID_AA64MMFR1_EL1,
    /// Memory Model Feature Register 2.
    ID_AA64MMFR2_EL1,

    // =========================================================================
    // Control Registers
    // =========================================================================
    /// System Control Register (EL1).
    SCTLR_EL1,
    /// System Control Register (EL2).
    SCTLR_EL2,
    /// System Control Register (EL3).
    SCTLR_EL3,
    /// Auxiliary Control Register (EL1).
    ACTLR_EL1,
    /// Coprocessor Access Control Register (EL1).
    CPACR_EL1,
    /// Hypervisor Configuration Register.
    HCR_EL2,
    /// Secure Configuration Register.
    SCR_EL3,

    // =========================================================================
    // Translation Registers
    // =========================================================================
    /// Translation Table Base Register 0 (EL1).
    TTBR0_EL1,
    /// Translation Table Base Register 1 (EL1).
    TTBR1_EL1,
    /// Translation Table Base Register 0 (EL2).
    TTBR0_EL2,
    /// Translation Control Register (EL1).
    TCR_EL1,
    /// Translation Control Register (EL2).
    TCR_EL2,
    /// Virtualization Translation Control Register.
    VTCR_EL2,
    /// Virtualization Translation Table Base Register.
    VTTBR_EL2,
    /// Memory Attribute Indirection Register (EL1).
    MAIR_EL1,
    /// Memory Attribute Indirection Register (EL2).
    MAIR_EL2,

    // =========================================================================
    // Exception Handling Registers
    // =========================================================================
    /// Vector Base Address Register (EL1).
    VBAR_EL1,
    /// Vector Base Address Register (EL2).
    VBAR_EL2,
    /// Vector Base Address Register (EL3).
    VBAR_EL3,
    /// Exception Syndrome Register (EL1).
    ESR_EL1,
    /// Exception Syndrome Register (EL2).
    ESR_EL2,
    /// Exception Syndrome Register (EL3).
    ESR_EL3,
    /// Fault Address Register (EL1).
    FAR_EL1,
    /// Fault Address Register (EL2).
    FAR_EL2,
    /// Fault Address Register (EL3).
    FAR_EL3,
    /// Hypervisor IPA Fault Address Register.
    HPFAR_EL2,
    /// Exception Link Register (EL1).
    ELR_EL1,
    /// Exception Link Register (EL2).
    ELR_EL2,
    /// Exception Link Register (EL3).
    ELR_EL3,
    /// Saved Program Status Register (EL1).
    SPSR_EL1,
    /// Saved Program Status Register (EL2).
    SPSR_EL2,
    /// Saved Program Status Register (EL3).
    SPSR_EL3,

    // =========================================================================
    // Stack Pointer Registers
    // =========================================================================
    /// Stack Pointer (EL0).
    SP_EL0,
    /// Stack Pointer (EL1).
    SP_EL1,
    /// Stack Pointer (EL2).
    SP_EL2,

    // =========================================================================
    // Thread Registers
    // =========================================================================
    /// Thread ID Register (EL0, user read/write).
    TPIDR_EL0,
    /// Thread ID Register (EL0, user read-only).
    TPIDRRO_EL0,
    /// Thread ID Register (EL1).
    TPIDR_EL1,
    /// Thread ID Register (EL2).
    TPIDR_EL2,
    /// Thread ID Register (EL3).
    TPIDR_EL3,

    // =========================================================================
    // Timer Registers
    // =========================================================================
    /// Counter-timer Frequency Register.
    CNTFRQ_EL0,
    /// Counter-timer Physical Count Register.
    CNTPCT_EL0,
    /// Counter-timer Virtual Count Register.
    CNTVCT_EL0,
    /// Counter-timer Physical Timer Control Register.
    CNTP_CTL_EL0,
    /// Counter-timer Physical Timer Compare Value Register.
    CNTP_CVAL_EL0,
    /// Counter-timer Physical Timer Timer Value Register.
    CNTP_TVAL_EL0,
    /// Counter-timer Virtual Timer Control Register.
    CNTV_CTL_EL0,
    /// Counter-timer Virtual Timer Compare Value Register.
    CNTV_CVAL_EL0,
    /// Counter-timer Virtual Timer Timer Value Register.
    CNTV_TVAL_EL0,
    /// Counter-timer Virtual Offset Register.
    CNTVOFF_EL2,
    /// Counter-timer Hypervisor Control Register.
    CNTHCTL_EL2,

    // =========================================================================
    // Debug Registers
    // =========================================================================
    /// Monitor Debug Configuration Register (EL2).
    MDCR_EL2,
    /// Monitor Debug Configuration Register (EL3).
    MDCR_EL3,
    /// Debug State and Control Register.
    MDSCR_EL1,
    /// OS Lock Data Transfer Register.
    OSDLR_EL1,
    /// OS Lock Access Register.
    OSLAR_EL1,

    // =========================================================================
    // Performance Monitor Registers
    // =========================================================================
    /// Performance Monitors Control Register.
    PMCR_EL0,
    /// Performance Monitors Count Enable Set Register.
    PMCNTENSET_EL0,
    /// Performance Monitors Count Enable Clear Register.
    PMCNTENCLR_EL0,
    /// Performance Monitors Cycle Count Register.
    PMCCNTR_EL0,
    /// Performance Monitors User Enable Register.
    PMUSERENR_EL0,

    // =========================================================================
    // GIC System Registers (GICv3+)
    // =========================================================================
    /// Interrupt Controller System Register Enable (EL1).
    ICC_SRE_EL1,
    /// Interrupt Controller System Register Enable (EL2).
    ICC_SRE_EL2,
    /// Interrupt Controller System Register Enable (EL3).
    ICC_SRE_EL3,
    /// Interrupt Controller Interrupt Acknowledge Register 0.
    ICC_IAR0_EL1,
    /// Interrupt Controller Interrupt Acknowledge Register 1.
    ICC_IAR1_EL1,
    /// Interrupt Controller End of Interrupt Register 0.
    ICC_EOIR0_EL1,
    /// Interrupt Controller End of Interrupt Register 1.
    ICC_EOIR1_EL1,
    /// Interrupt Controller Running Priority Register.
    ICC_RPR_EL1,
    /// Interrupt Controller Binary Point Register 0.
    ICC_BPR0_EL1,
    /// Interrupt Controller Binary Point Register 1.
    ICC_BPR1_EL1,
    /// Interrupt Controller Interrupt Priority Mask Register.
    ICC_PMR_EL1,
    /// Interrupt Controller Control Register.
    ICC_CTLR_EL1,
    /// Interrupt Controller Interrupt Group 0 Enable Register.
    ICC_IGRPEN0_EL1,
    /// Interrupt Controller Interrupt Group 1 Enable Register.
    ICC_IGRPEN1_EL1,
    /// Interrupt Controller Software Generated Interrupt Group 0 Register.
    ICC_SGI0R_EL1,
    /// Interrupt Controller Software Generated Interrupt Group 1 Register.
    ICC_SGI1R_EL1,

    // =========================================================================
    // Pointer Authentication Registers (ARMv8.3+)
    // =========================================================================
    /// Pointer Authentication Key A for Instruction (low).
    APIAKeyLo_EL1,
    /// Pointer Authentication Key A for Instruction (high).
    APIAKeyHi_EL1,
    /// Pointer Authentication Key B for Instruction (low).
    APIBKeyLo_EL1,
    /// Pointer Authentication Key B for Instruction (high).
    APIBKeyHi_EL1,
    /// Pointer Authentication Key A for Data (low).
    APDAKeyLo_EL1,
    /// Pointer Authentication Key A for Data (high).
    APDAKeyHi_EL1,
    /// Pointer Authentication Key B for Data (low).
    APDBKeyLo_EL1,
    /// Pointer Authentication Key B for Data (high).
    APDBKeyHi_EL1,
    /// Pointer Authentication Key for Generic (low).
    APGAKeyLo_EL1,
    /// Pointer Authentication Key for Generic (high).
    APGAKeyHi_EL1,

    /// Unknown/unsupported register.
    Unknown(Aarch64SysRegEncoding),
}

impl Aarch64SysReg {
    /// Get the encoding for this register.
    pub const fn encoding(&self) -> Aarch64SysRegEncoding {
        use Aarch64SysReg::*;
        match self {
            // ID Registers
            MIDR_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 0, 0),
            MPIDR_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 0, 5),
            REVIDR_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 0, 6),
            ID_AA64PFR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 4, 0),
            ID_AA64PFR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 4, 1),
            ID_AA64DFR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 5, 0),
            ID_AA64DFR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 5, 1),
            ID_AA64ISAR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 6, 0),
            ID_AA64ISAR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 6, 1),
            ID_AA64ISAR2_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 6, 2),
            ID_AA64MMFR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 7, 0),
            ID_AA64MMFR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 7, 1),
            ID_AA64MMFR2_EL1 => Aarch64SysRegEncoding::new(3, 0, 0, 7, 2),

            // Control Registers
            SCTLR_EL1 => Aarch64SysRegEncoding::new(3, 0, 1, 0, 0),
            SCTLR_EL2 => Aarch64SysRegEncoding::new(3, 4, 1, 0, 0),
            SCTLR_EL3 => Aarch64SysRegEncoding::new(3, 6, 1, 0, 0),
            ACTLR_EL1 => Aarch64SysRegEncoding::new(3, 0, 1, 0, 1),
            CPACR_EL1 => Aarch64SysRegEncoding::new(3, 0, 1, 0, 2),
            HCR_EL2 => Aarch64SysRegEncoding::new(3, 4, 1, 1, 0),
            SCR_EL3 => Aarch64SysRegEncoding::new(3, 6, 1, 1, 0),

            // Translation Registers
            TTBR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 0, 0),
            TTBR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 0, 1),
            TTBR0_EL2 => Aarch64SysRegEncoding::new(3, 4, 2, 0, 0),
            TCR_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 0, 2),
            TCR_EL2 => Aarch64SysRegEncoding::new(3, 4, 2, 0, 2),
            VTCR_EL2 => Aarch64SysRegEncoding::new(3, 4, 2, 1, 2),
            VTTBR_EL2 => Aarch64SysRegEncoding::new(3, 4, 2, 1, 0),
            MAIR_EL1 => Aarch64SysRegEncoding::new(3, 0, 10, 2, 0),
            MAIR_EL2 => Aarch64SysRegEncoding::new(3, 4, 10, 2, 0),

            // Exception Handling Registers
            VBAR_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 0, 0),
            VBAR_EL2 => Aarch64SysRegEncoding::new(3, 4, 12, 0, 0),
            VBAR_EL3 => Aarch64SysRegEncoding::new(3, 6, 12, 0, 0),
            ESR_EL1 => Aarch64SysRegEncoding::new(3, 0, 5, 2, 0),
            ESR_EL2 => Aarch64SysRegEncoding::new(3, 4, 5, 2, 0),
            ESR_EL3 => Aarch64SysRegEncoding::new(3, 6, 5, 2, 0),
            FAR_EL1 => Aarch64SysRegEncoding::new(3, 0, 6, 0, 0),
            FAR_EL2 => Aarch64SysRegEncoding::new(3, 4, 6, 0, 0),
            FAR_EL3 => Aarch64SysRegEncoding::new(3, 6, 6, 0, 0),
            HPFAR_EL2 => Aarch64SysRegEncoding::new(3, 4, 6, 0, 4),
            ELR_EL1 => Aarch64SysRegEncoding::new(3, 0, 4, 0, 1),
            ELR_EL2 => Aarch64SysRegEncoding::new(3, 4, 4, 0, 1),
            ELR_EL3 => Aarch64SysRegEncoding::new(3, 6, 4, 0, 1),
            SPSR_EL1 => Aarch64SysRegEncoding::new(3, 0, 4, 0, 0),
            SPSR_EL2 => Aarch64SysRegEncoding::new(3, 4, 4, 0, 0),
            SPSR_EL3 => Aarch64SysRegEncoding::new(3, 6, 4, 0, 0),

            // Stack Pointer Registers
            SP_EL0 => Aarch64SysRegEncoding::new(3, 0, 4, 1, 0),
            SP_EL1 => Aarch64SysRegEncoding::new(3, 4, 4, 1, 0),
            SP_EL2 => Aarch64SysRegEncoding::new(3, 6, 4, 1, 0),

            // Thread Registers
            TPIDR_EL0 => Aarch64SysRegEncoding::new(3, 3, 13, 0, 2),
            TPIDRRO_EL0 => Aarch64SysRegEncoding::new(3, 3, 13, 0, 3),
            TPIDR_EL1 => Aarch64SysRegEncoding::new(3, 0, 13, 0, 4),
            TPIDR_EL2 => Aarch64SysRegEncoding::new(3, 4, 13, 0, 2),
            TPIDR_EL3 => Aarch64SysRegEncoding::new(3, 6, 13, 0, 2),

            // Timer Registers
            CNTFRQ_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 0, 0),
            CNTPCT_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 0, 1),
            CNTVCT_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 0, 2),
            CNTP_CTL_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 2, 1),
            CNTP_CVAL_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 2, 2),
            CNTP_TVAL_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 2, 0),
            CNTV_CTL_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 3, 1),
            CNTV_CVAL_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 3, 2),
            CNTV_TVAL_EL0 => Aarch64SysRegEncoding::new(3, 3, 14, 3, 0),
            CNTVOFF_EL2 => Aarch64SysRegEncoding::new(3, 4, 14, 0, 3),
            CNTHCTL_EL2 => Aarch64SysRegEncoding::new(3, 4, 14, 1, 0),

            // Debug Registers
            MDCR_EL2 => Aarch64SysRegEncoding::new(3, 4, 1, 1, 1),
            MDCR_EL3 => Aarch64SysRegEncoding::new(3, 6, 1, 3, 1),
            MDSCR_EL1 => Aarch64SysRegEncoding::new(2, 0, 0, 2, 2),
            OSDLR_EL1 => Aarch64SysRegEncoding::new(2, 0, 1, 3, 4),
            OSLAR_EL1 => Aarch64SysRegEncoding::new(2, 0, 1, 0, 4),

            // Performance Monitor Registers
            PMCR_EL0 => Aarch64SysRegEncoding::new(3, 3, 9, 12, 0),
            PMCNTENSET_EL0 => Aarch64SysRegEncoding::new(3, 3, 9, 12, 1),
            PMCNTENCLR_EL0 => Aarch64SysRegEncoding::new(3, 3, 9, 12, 2),
            PMCCNTR_EL0 => Aarch64SysRegEncoding::new(3, 3, 9, 13, 0),
            PMUSERENR_EL0 => Aarch64SysRegEncoding::new(3, 3, 9, 14, 0),

            // GIC System Registers
            ICC_SRE_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 5),
            ICC_SRE_EL2 => Aarch64SysRegEncoding::new(3, 4, 12, 9, 5),
            ICC_SRE_EL3 => Aarch64SysRegEncoding::new(3, 6, 12, 12, 5),
            ICC_IAR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 8, 0),
            ICC_IAR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 0),
            ICC_EOIR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 8, 1),
            ICC_EOIR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 1),
            ICC_RPR_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 11, 3),
            ICC_BPR0_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 8, 3),
            ICC_BPR1_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 3),
            ICC_PMR_EL1 => Aarch64SysRegEncoding::new(3, 0, 4, 6, 0),
            ICC_CTLR_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 4),
            ICC_IGRPEN0_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 6),
            ICC_IGRPEN1_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 12, 7),
            ICC_SGI0R_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 11, 7),
            ICC_SGI1R_EL1 => Aarch64SysRegEncoding::new(3, 0, 12, 11, 5),

            // Pointer Authentication Registers
            APIAKeyLo_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 1, 0),
            APIAKeyHi_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 1, 1),
            APIBKeyLo_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 1, 2),
            APIBKeyHi_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 1, 3),
            APDAKeyLo_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 2, 0),
            APDAKeyHi_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 2, 1),
            APDBKeyLo_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 2, 2),
            APDBKeyHi_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 2, 3),
            APGAKeyLo_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 3, 0),
            APGAKeyHi_EL1 => Aarch64SysRegEncoding::new(3, 0, 2, 3, 1),

            Unknown(enc) => *enc,
        }
    }

    /// Get the register name as a string.
    pub fn name(&self) -> &'static str {
        use Aarch64SysReg::*;
        match self {
            MIDR_EL1 => "MIDR_EL1",
            MPIDR_EL1 => "MPIDR_EL1",
            REVIDR_EL1 => "REVIDR_EL1",
            ID_AA64PFR0_EL1 => "ID_AA64PFR0_EL1",
            ID_AA64PFR1_EL1 => "ID_AA64PFR1_EL1",
            ID_AA64DFR0_EL1 => "ID_AA64DFR0_EL1",
            ID_AA64DFR1_EL1 => "ID_AA64DFR1_EL1",
            ID_AA64ISAR0_EL1 => "ID_AA64ISAR0_EL1",
            ID_AA64ISAR1_EL1 => "ID_AA64ISAR1_EL1",
            ID_AA64ISAR2_EL1 => "ID_AA64ISAR2_EL1",
            ID_AA64MMFR0_EL1 => "ID_AA64MMFR0_EL1",
            ID_AA64MMFR1_EL1 => "ID_AA64MMFR1_EL1",
            ID_AA64MMFR2_EL1 => "ID_AA64MMFR2_EL1",
            SCTLR_EL1 => "SCTLR_EL1",
            SCTLR_EL2 => "SCTLR_EL2",
            SCTLR_EL3 => "SCTLR_EL3",
            ACTLR_EL1 => "ACTLR_EL1",
            CPACR_EL1 => "CPACR_EL1",
            HCR_EL2 => "HCR_EL2",
            SCR_EL3 => "SCR_EL3",
            TTBR0_EL1 => "TTBR0_EL1",
            TTBR1_EL1 => "TTBR1_EL1",
            TTBR0_EL2 => "TTBR0_EL2",
            TCR_EL1 => "TCR_EL1",
            TCR_EL2 => "TCR_EL2",
            VTCR_EL2 => "VTCR_EL2",
            VTTBR_EL2 => "VTTBR_EL2",
            MAIR_EL1 => "MAIR_EL1",
            MAIR_EL2 => "MAIR_EL2",
            VBAR_EL1 => "VBAR_EL1",
            VBAR_EL2 => "VBAR_EL2",
            VBAR_EL3 => "VBAR_EL3",
            ESR_EL1 => "ESR_EL1",
            ESR_EL2 => "ESR_EL2",
            ESR_EL3 => "ESR_EL3",
            FAR_EL1 => "FAR_EL1",
            FAR_EL2 => "FAR_EL2",
            FAR_EL3 => "FAR_EL3",
            HPFAR_EL2 => "HPFAR_EL2",
            ELR_EL1 => "ELR_EL1",
            ELR_EL2 => "ELR_EL2",
            ELR_EL3 => "ELR_EL3",
            SPSR_EL1 => "SPSR_EL1",
            SPSR_EL2 => "SPSR_EL2",
            SPSR_EL3 => "SPSR_EL3",
            SP_EL0 => "SP_EL0",
            SP_EL1 => "SP_EL1",
            SP_EL2 => "SP_EL2",
            TPIDR_EL0 => "TPIDR_EL0",
            TPIDRRO_EL0 => "TPIDRRO_EL0",
            TPIDR_EL1 => "TPIDR_EL1",
            TPIDR_EL2 => "TPIDR_EL2",
            TPIDR_EL3 => "TPIDR_EL3",
            CNTFRQ_EL0 => "CNTFRQ_EL0",
            CNTPCT_EL0 => "CNTPCT_EL0",
            CNTVCT_EL0 => "CNTVCT_EL0",
            CNTP_CTL_EL0 => "CNTP_CTL_EL0",
            CNTP_CVAL_EL0 => "CNTP_CVAL_EL0",
            CNTP_TVAL_EL0 => "CNTP_TVAL_EL0",
            CNTV_CTL_EL0 => "CNTV_CTL_EL0",
            CNTV_CVAL_EL0 => "CNTV_CVAL_EL0",
            CNTV_TVAL_EL0 => "CNTV_TVAL_EL0",
            CNTVOFF_EL2 => "CNTVOFF_EL2",
            CNTHCTL_EL2 => "CNTHCTL_EL2",
            MDCR_EL2 => "MDCR_EL2",
            MDCR_EL3 => "MDCR_EL3",
            MDSCR_EL1 => "MDSCR_EL1",
            OSDLR_EL1 => "OSDLR_EL1",
            OSLAR_EL1 => "OSLAR_EL1",
            PMCR_EL0 => "PMCR_EL0",
            PMCNTENSET_EL0 => "PMCNTENSET_EL0",
            PMCNTENCLR_EL0 => "PMCNTENCLR_EL0",
            PMCCNTR_EL0 => "PMCCNTR_EL0",
            PMUSERENR_EL0 => "PMUSERENR_EL0",
            ICC_SRE_EL1 => "ICC_SRE_EL1",
            ICC_SRE_EL2 => "ICC_SRE_EL2",
            ICC_SRE_EL3 => "ICC_SRE_EL3",
            ICC_IAR0_EL1 => "ICC_IAR0_EL1",
            ICC_IAR1_EL1 => "ICC_IAR1_EL1",
            ICC_EOIR0_EL1 => "ICC_EOIR0_EL1",
            ICC_EOIR1_EL1 => "ICC_EOIR1_EL1",
            ICC_RPR_EL1 => "ICC_RPR_EL1",
            ICC_BPR0_EL1 => "ICC_BPR0_EL1",
            ICC_BPR1_EL1 => "ICC_BPR1_EL1",
            ICC_PMR_EL1 => "ICC_PMR_EL1",
            ICC_CTLR_EL1 => "ICC_CTLR_EL1",
            ICC_IGRPEN0_EL1 => "ICC_IGRPEN0_EL1",
            ICC_IGRPEN1_EL1 => "ICC_IGRPEN1_EL1",
            ICC_SGI0R_EL1 => "ICC_SGI0R_EL1",
            ICC_SGI1R_EL1 => "ICC_SGI1R_EL1",
            APIAKeyLo_EL1 => "APIAKeyLo_EL1",
            APIAKeyHi_EL1 => "APIAKeyHi_EL1",
            APIBKeyLo_EL1 => "APIBKeyLo_EL1",
            APIBKeyHi_EL1 => "APIBKeyHi_EL1",
            APDAKeyLo_EL1 => "APDAKeyLo_EL1",
            APDAKeyHi_EL1 => "APDAKeyHi_EL1",
            APDBKeyLo_EL1 => "APDBKeyLo_EL1",
            APDBKeyHi_EL1 => "APDBKeyHi_EL1",
            APGAKeyLo_EL1 => "APGAKeyLo_EL1",
            APGAKeyHi_EL1 => "APGAKeyHi_EL1",
            Unknown(_) => "Unknown",
        }
    }

    /// Decode a system register from its encoding.
    pub fn from_encoding(enc: Aarch64SysRegEncoding) -> Self {
        // Build a lookup key
        let key = enc.encode();

        // Check against known registers
        // This is a simple linear search; could be optimized with a hash map
        let known = [
            Aarch64SysReg::MIDR_EL1,
            Aarch64SysReg::MPIDR_EL1,
            Aarch64SysReg::REVIDR_EL1,
            Aarch64SysReg::ID_AA64PFR0_EL1,
            Aarch64SysReg::ID_AA64PFR1_EL1,
            Aarch64SysReg::ID_AA64DFR0_EL1,
            Aarch64SysReg::ID_AA64DFR1_EL1,
            Aarch64SysReg::ID_AA64ISAR0_EL1,
            Aarch64SysReg::ID_AA64ISAR1_EL1,
            Aarch64SysReg::ID_AA64ISAR2_EL1,
            Aarch64SysReg::ID_AA64MMFR0_EL1,
            Aarch64SysReg::ID_AA64MMFR1_EL1,
            Aarch64SysReg::ID_AA64MMFR2_EL1,
            Aarch64SysReg::SCTLR_EL1,
            Aarch64SysReg::SCTLR_EL2,
            Aarch64SysReg::SCTLR_EL3,
            Aarch64SysReg::ACTLR_EL1,
            Aarch64SysReg::CPACR_EL1,
            Aarch64SysReg::HCR_EL2,
            Aarch64SysReg::SCR_EL3,
            Aarch64SysReg::TTBR0_EL1,
            Aarch64SysReg::TTBR1_EL1,
            Aarch64SysReg::TTBR0_EL2,
            Aarch64SysReg::TCR_EL1,
            Aarch64SysReg::TCR_EL2,
            Aarch64SysReg::VTCR_EL2,
            Aarch64SysReg::VTTBR_EL2,
            Aarch64SysReg::MAIR_EL1,
            Aarch64SysReg::MAIR_EL2,
            Aarch64SysReg::VBAR_EL1,
            Aarch64SysReg::VBAR_EL2,
            Aarch64SysReg::VBAR_EL3,
            Aarch64SysReg::ESR_EL1,
            Aarch64SysReg::ESR_EL2,
            Aarch64SysReg::ESR_EL3,
            Aarch64SysReg::FAR_EL1,
            Aarch64SysReg::FAR_EL2,
            Aarch64SysReg::FAR_EL3,
            Aarch64SysReg::HPFAR_EL2,
            Aarch64SysReg::ELR_EL1,
            Aarch64SysReg::ELR_EL2,
            Aarch64SysReg::ELR_EL3,
            Aarch64SysReg::SPSR_EL1,
            Aarch64SysReg::SPSR_EL2,
            Aarch64SysReg::SPSR_EL3,
            Aarch64SysReg::SP_EL0,
            Aarch64SysReg::SP_EL1,
            Aarch64SysReg::SP_EL2,
            Aarch64SysReg::TPIDR_EL0,
            Aarch64SysReg::TPIDRRO_EL0,
            Aarch64SysReg::TPIDR_EL1,
            Aarch64SysReg::TPIDR_EL2,
            Aarch64SysReg::TPIDR_EL3,
            Aarch64SysReg::CNTFRQ_EL0,
            Aarch64SysReg::CNTPCT_EL0,
            Aarch64SysReg::CNTVCT_EL0,
            Aarch64SysReg::CNTP_CTL_EL0,
            Aarch64SysReg::CNTP_CVAL_EL0,
            Aarch64SysReg::CNTP_TVAL_EL0,
            Aarch64SysReg::CNTV_CTL_EL0,
            Aarch64SysReg::CNTV_CVAL_EL0,
            Aarch64SysReg::CNTV_TVAL_EL0,
            Aarch64SysReg::CNTVOFF_EL2,
            Aarch64SysReg::CNTHCTL_EL2,
            Aarch64SysReg::ICC_PMR_EL1,
        ];

        for reg in known {
            if reg.encoding().encode() == key {
                return reg;
            }
        }

        Aarch64SysReg::Unknown(enc)
    }
}

impl fmt::Display for Aarch64SysReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Decode an AArch64 system register from MSR/MRS instruction encoding.
///
/// The encoding is in bits 20:5 of the instruction.
pub fn decode_msr_encoding(encoding: u32) -> Aarch64SysRegEncoding {
    Aarch64SysRegEncoding {
        op0: ((encoding >> 19) & 0x3) as u8,
        op1: ((encoding >> 16) & 0x7) as u8,
        crn: ((encoding >> 12) & 0xF) as u8,
        crm: ((encoding >> 8) & 0xF) as u8,
        op2: ((encoding >> 5) & 0x7) as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sysreg_encoding() {
        let sctlr = Aarch64SysReg::SCTLR_EL1;
        let enc = sctlr.encoding();
        assert_eq!(enc.op0, 3);
        assert_eq!(enc.op1, 0);
        assert_eq!(enc.crn, 1);
        assert_eq!(enc.crm, 0);
        assert_eq!(enc.op2, 0);
    }

    #[test]
    fn sysreg_decode() {
        let enc = Aarch64SysRegEncoding::new(3, 0, 1, 0, 0);
        let reg = Aarch64SysReg::from_encoding(enc);
        assert_eq!(reg, Aarch64SysReg::SCTLR_EL1);
    }

    #[test]
    fn cp15_encoding() {
        let enc = Cp15Encoding::new(1, 0, 0, 0); // SCTLR
        let encoded = enc.encode();
        let decoded = Cp15Encoding::decode(encoded);
        assert_eq!(enc, decoded);
    }

    #[test]
    fn sysreg_roundtrip() {
        let enc = Aarch64SysRegEncoding::new(3, 3, 14, 2, 1); // CNTP_CTL_EL0
        let encoded = enc.encode();
        let decoded = Aarch64SysRegEncoding::decode(encoded);
        assert_eq!(enc, decoded);
    }
}
