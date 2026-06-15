//! ARM CP15 System Control Coprocessor.
//!
//! CP15 is the system control coprocessor in ARMv7 processors. It provides
//! access to:
//! - System control and configuration (SCTLR)
//! - MMU/MPU configuration
//! - Cache control
//! - TLB control
//! - Performance monitors
//! - ID registers
//!
//! # Register Access
//!
//! CP15 registers are accessed via MCR (write) and MRC (read) instructions:
//! ```text
//! MCR p15, Op1, Rt, CRn, CRm, Op2  ; Write Rt to CP15 register
//! MRC p15, Op1, Rt, CRn, CRm, Op2  ; Read CP15 register to Rt
//! ```
//!
//! # Important Registers
//!
//! - SCTLR (c1, 0, c0, 0): System Control Register
//! - CPACR (c1, 0, c0, 2): Coprocessor Access Control Register
//! - TTBR0 (c2, 0, c0, 0): Translation Table Base Register 0
//! - TTBR1 (c2, 0, c0, 1): Translation Table Base Register 1
//! - TTBCR (c2, 0, c0, 2): Translation Table Base Control Register
//! - DACR (c3, 0, c0, 0): Domain Access Control Register
//! - DFSR (c5, 0, c0, 0): Data Fault Status Register
//! - IFSR (c5, 0, c0, 1): Instruction Fault Status Register
//! - DFAR (c6, 0, c0, 0): Data Fault Address Register
//! - IFAR (c6, 0, c0, 2): Instruction Fault Address Register
//! - CONTEXTIDR (c13, 0, c0, 1): Context ID Register
//! - TPIDRURW (c13, 0, c0, 2): User Read/Write Thread ID Register
//! - TPIDRURO (c13, 0, c0, 3): User Read-Only Thread ID Register
//! - TPIDRPRW (c13, 0, c0, 4): PL1 Only Thread ID Register

use crate::arm::sysreg::Cp15Encoding;
use std::fmt;

/// CP15 coprocessor state.
#[derive(Clone)]
pub struct Cp15State {
    // =========================================================================
    // ID Registers (read-only) - CRn=0
    // =========================================================================
    /// Main ID Register (MIDR).
    pub midr: u32,
    /// Cache Type Register (CTR).
    pub ctr: u32,
    /// TCM Type Register.
    pub tcmtr: u32,
    /// TLB Type Register.
    pub tlbtr: u32,
    /// Multiprocessor Affinity Register (MPIDR).
    pub mpidr: u32,
    /// Processor Feature Register 0 (ID_PFR0).
    pub id_pfr0: u32,
    /// Processor Feature Register 1 (ID_PFR1).
    pub id_pfr1: u32,
    /// Debug Feature Register 0 (ID_DFR0).
    pub id_dfr0: u32,
    /// Auxiliary Feature Register 0 (ID_AFR0).
    pub id_afr0: u32,
    /// Memory Model Feature Register 0-3.
    pub id_mmfr: [u32; 4],
    /// Instruction Set Attribute Register 0-5.
    pub id_isar: [u32; 6],

    // =========================================================================
    // System Control Registers - CRn=1
    // =========================================================================
    /// System Control Register (SCTLR).
    pub sctlr: Sctlr,
    /// Auxiliary Control Register (ACTLR).
    pub actlr: u32,
    /// Coprocessor Access Control Register (CPACR).
    pub cpacr: Cpacr,
    /// Secure Configuration Register (SCR) - if Security Extensions.
    pub scr: u32,
    /// Secure Debug Enable Register (SDER).
    pub sder: u32,
    /// Non-Secure Access Control Register (NSACR).
    pub nsacr: u32,

    // =========================================================================
    // Translation Table Registers - CRn=2
    // =========================================================================
    /// Translation Table Base Register 0 (TTBR0).
    pub ttbr0: u64,
    /// Translation Table Base Register 1 (TTBR1).
    pub ttbr1: u64,
    /// Translation Table Base Control Register (TTBCR).
    pub ttbcr: u32,

    // =========================================================================
    // Domain Access Control - CRn=3
    // =========================================================================
    /// Domain Access Control Register (DACR).
    pub dacr: u32,

    // =========================================================================
    // Fault Status Registers - CRn=5
    // =========================================================================
    /// Data Fault Status Register (DFSR).
    pub dfsr: u32,
    /// Instruction Fault Status Register (IFSR).
    pub ifsr: u32,
    /// Auxiliary Data Fault Status Register (ADFSR).
    pub adfsr: u32,
    /// Auxiliary Instruction Fault Status Register (AIFSR).
    pub aifsr: u32,

    // =========================================================================
    // Fault Address Registers - CRn=6
    // =========================================================================
    /// Data Fault Address Register (DFAR).
    pub dfar: u32,
    /// Instruction Fault Address Register (IFAR).
    pub ifar: u32,

    // =========================================================================
    // Cache Operations - CRn=7
    // =========================================================================
    // Most are write-only operations, no state needed

    // =========================================================================
    // TLB Operations - CRn=8
    // =========================================================================
    // All are write-only operations, no state needed

    // =========================================================================
    // Performance Monitors - CRn=9
    // =========================================================================
    /// Performance Monitor Control Register (PMCR).
    pub pmcr: u32,
    /// Count Enable Set Register (PMCNTENSET).
    pub pmcntenset: u32,
    /// Cycle Count Register (PMCCNTR).
    pub pmccntr: u32,
    /// Performance counter registers.
    pub pmevcntr: [u32; 6],
    /// Event type selection registers.
    pub pmevtyper: [u32; 6],
    /// User Enable Register (PMUSERENR).
    pub pmuserenr: u32,

    // =========================================================================
    // Memory Barrier Operations - CRn=7, various
    // =========================================================================
    // Write-only operations, no state

    // =========================================================================
    // Context ID - CRn=13
    // =========================================================================
    /// FCSE PID Register (deprecated).
    pub fcseidr: u32,
    /// Context ID Register (CONTEXTIDR).
    pub contextidr: u32,
    /// User Read/Write Thread ID Register (TPIDRURW).
    pub tpidrurw: u32,
    /// User Read-Only Thread ID Register (TPIDRURO).
    pub tpidruro: u32,
    /// PL1 Only Thread ID Register (TPIDRPRW).
    pub tpidrprw: u32,

    // =========================================================================
    // Generic Timer - CRn=14
    // =========================================================================
    /// Counter Frequency Register (CNTFRQ).
    pub cntfrq: u32,
    /// Physical Timer Value Register (CNTP_TVAL).
    pub cntp_tval: u32,
    /// Physical Timer Control Register (CNTP_CTL).
    pub cntp_ctl: u32,
    /// Virtual Timer Value Register (CNTV_TVAL).
    pub cntv_tval: u32,
    /// Virtual Timer Control Register (CNTV_CTL).
    pub cntv_ctl: u32,
}

impl Default for Cp15State {
    fn default() -> Self {
        Self {
            // ARM Cortex-A9 like MIDR
            midr: 0x410FC090,
            // Cache type: Separate I/D caches, 32-byte lines
            ctr: 0x83338003,
            tcmtr: 0,
            tlbtr: 0,
            // MPIDR: CPU 0, cluster 0, MP extensions
            mpidr: 0x80000000,
            // Processor features
            id_pfr0: 0x00001131, // Thumb, ThumbEE, Jazelle
            id_pfr1: 0x00011011, // Security, Virtualization
            id_dfr0: 0x02010555, // Debug
            id_afr0: 0,
            id_mmfr: [0x10101105, 0x40000000, 0x01260000, 0x02102211],
            id_isar: [
                0x02101110, 0x13112111, 0x21232041, 0x11112131, 0x10011142, 0,
            ],

            // System control - reset values
            sctlr: Sctlr::default(),
            actlr: 0,
            cpacr: Cpacr::default(),
            scr: 0,
            sder: 0,
            nsacr: 0,

            // Translation tables
            ttbr0: 0,
            ttbr1: 0,
            ttbcr: 0,

            // Domain access
            dacr: 0,

            // Fault status
            dfsr: 0,
            ifsr: 0,
            adfsr: 0,
            aifsr: 0,

            // Fault address
            dfar: 0,
            ifar: 0,

            // Performance monitors
            pmcr: 0x41093000, // Implementer ARM, 6 counters
            pmcntenset: 0,
            pmccntr: 0,
            pmevcntr: [0; 6],
            pmevtyper: [0; 6],
            pmuserenr: 0,

            // Context
            fcseidr: 0,
            contextidr: 0,
            tpidrurw: 0,
            tpidruro: 0,
            tpidrprw: 0,

            // Timer
            cntfrq: 0x0177_0000, // ~24.5 MHz
            cntp_tval: 0,
            cntp_ctl: 0,
            cntv_tval: 0,
            cntv_ctl: 0,
        }
    }
}

impl Cp15State {
    /// Create new CP15 state with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read a CP15 register.
    pub fn read(&self, enc: Cp15Encoding) -> Option<u32> {
        match (enc.crn, enc.op1, enc.crm, enc.op2) {
            // CRn=0: ID registers
            (0, 0, 0, 0) => Some(self.midr),
            (0, 0, 0, 1) => Some(self.ctr),
            (0, 0, 0, 2) => Some(self.tcmtr),
            (0, 0, 0, 3) => Some(self.tlbtr),
            (0, 0, 0, 5) => Some(self.mpidr),

            (0, 0, 1, 0) => Some(self.id_pfr0),
            (0, 0, 1, 1) => Some(self.id_pfr1),
            (0, 0, 1, 2) => Some(self.id_dfr0),
            (0, 0, 1, 3) => Some(self.id_afr0),
            (0, 0, 1, 4) => Some(self.id_mmfr[0]),
            (0, 0, 1, 5) => Some(self.id_mmfr[1]),
            (0, 0, 1, 6) => Some(self.id_mmfr[2]),
            (0, 0, 1, 7) => Some(self.id_mmfr[3]),
            (0, 0, 2, 0) => Some(self.id_isar[0]),
            (0, 0, 2, 1) => Some(self.id_isar[1]),
            (0, 0, 2, 2) => Some(self.id_isar[2]),
            (0, 0, 2, 3) => Some(self.id_isar[3]),
            (0, 0, 2, 4) => Some(self.id_isar[4]),
            (0, 0, 2, 5) => Some(self.id_isar[5]),

            // CRn=1: System control
            (1, 0, 0, 0) => Some(self.sctlr.bits()),
            (1, 0, 0, 1) => Some(self.actlr),
            (1, 0, 0, 2) => Some(self.cpacr.bits()),
            (1, 0, 1, 0) => Some(self.scr),
            (1, 0, 1, 1) => Some(self.sder),
            (1, 0, 1, 2) => Some(self.nsacr),

            // CRn=2: Translation table
            (2, 0, 0, 0) => Some(self.ttbr0 as u32),
            (2, 0, 0, 1) => Some(self.ttbr1 as u32),
            (2, 0, 0, 2) => Some(self.ttbcr),

            // CRn=3: Domain access
            (3, 0, 0, 0) => Some(self.dacr),

            // CRn=5: Fault status
            (5, 0, 0, 0) => Some(self.dfsr),
            (5, 0, 0, 1) => Some(self.ifsr),
            (5, 0, 1, 0) => Some(self.adfsr),
            (5, 0, 1, 1) => Some(self.aifsr),

            // CRn=6: Fault address
            (6, 0, 0, 0) => Some(self.dfar),
            (6, 0, 0, 2) => Some(self.ifar),

            // CRn=9: Performance monitors
            (9, 0, 12, 0) => Some(self.pmcr),
            (9, 0, 12, 1) => Some(self.pmcntenset),
            (9, 0, 13, 0) => Some(self.pmccntr),
            (9, 0, 14, 0) => Some(self.pmuserenr),

            // CRn=13: Context
            (13, 0, 0, 0) => Some(self.fcseidr),
            (13, 0, 0, 1) => Some(self.contextidr),
            (13, 0, 0, 2) => Some(self.tpidrurw),
            (13, 0, 0, 3) => Some(self.tpidruro),
            (13, 0, 0, 4) => Some(self.tpidrprw),

            // CRn=14: Timer
            (14, 0, 0, 0) => Some(self.cntfrq),

            _ => None, // Unknown register
        }
    }

    /// Write a CP15 register.
    pub fn write(&mut self, enc: Cp15Encoding, value: u32) -> bool {
        match (enc.crn, enc.op1, enc.crm, enc.op2) {
            // CRn=0: ID registers are read-only
            (0, _, _, _) => false,

            // CRn=1: System control
            (1, 0, 0, 0) => {
                self.sctlr = Sctlr::from_bits(value);
                true
            }
            (1, 0, 0, 1) => {
                self.actlr = value;
                true
            }
            (1, 0, 0, 2) => {
                self.cpacr = Cpacr::from_bits(value);
                true
            }
            (1, 0, 1, 0) => {
                self.scr = value;
                true
            }
            (1, 0, 1, 1) => {
                self.sder = value;
                true
            }
            (1, 0, 1, 2) => {
                self.nsacr = value;
                true
            }

            // CRn=2: Translation table
            (2, 0, 0, 0) => {
                self.ttbr0 = value as u64;
                true
            }
            (2, 0, 0, 1) => {
                self.ttbr1 = value as u64;
                true
            }
            (2, 0, 0, 2) => {
                self.ttbcr = value;
                true
            }

            // CRn=3: Domain access
            (3, 0, 0, 0) => {
                self.dacr = value;
                true
            }

            // CRn=5: Fault status
            (5, 0, 0, 0) => {
                self.dfsr = value;
                true
            }
            (5, 0, 0, 1) => {
                self.ifsr = value;
                true
            }
            (5, 0, 1, 0) => {
                self.adfsr = value;
                true
            }
            (5, 0, 1, 1) => {
                self.aifsr = value;
                true
            }

            // CRn=6: Fault address
            (6, 0, 0, 0) => {
                self.dfar = value;
                true
            }
            (6, 0, 0, 2) => {
                self.ifar = value;
                true
            }

            // CRn=7: Cache operations (write-only, perform operation)
            (7, 0, 1, 0) => true,  // ICIALLUIS - Invalidate all I-caches
            (7, 0, 5, 0) => true,  // ICIALLU - Invalidate all I-cache
            (7, 0, 5, 1) => true,  // ICIMVAU - Invalidate I-cache by MVA
            (7, 0, 5, 6) => true,  // BPIALL - Invalidate branch predictors
            (7, 0, 5, 7) => true,  // BPIMVA - Invalidate branch predictor by MVA
            (7, 0, 6, 1) => true,  // DCIMVAC - Invalidate D-cache by MVA
            (7, 0, 6, 2) => true,  // DCISW - Invalidate D-cache by set/way
            (7, 0, 10, 1) => true, // DCCMVAC - Clean D-cache by MVA
            (7, 0, 10, 2) => true, // DCCSW - Clean D-cache by set/way
            (7, 0, 11, 1) => true, // DCCMVAU - Clean D-cache by MVA (PoU)
            (7, 0, 14, 1) => true, // DCCIMVAC - Clean and invalidate D-cache
            (7, 0, 14, 2) => true, // DCCISW - Clean and invalidate by set/way

            // CRn=7: Barriers (write-only, perform operation)
            (7, 0, 4, 4) => true,  // CP15DSB - Data Synchronization Barrier
            (7, 0, 4, 5) => true,  // CP15DMB - Data Memory Barrier
            (7, 0, 5, 4) => true,  // CP15ISB - Instruction Synchronization Barrier
            (7, 0, 10, 4) => true, // DSB
            (7, 0, 10, 5) => true, // DMB

            // CRn=8: TLB operations (write-only)
            (8, 0, 3, 0) => true, // TLBIALLIS - Invalidate entire TLB (IS)
            (8, 0, 3, 1) => true, // TLBIMVAIS - Invalidate by MVA (IS)
            (8, 0, 3, 2) => true, // TLBIASIDIS - Invalidate by ASID (IS)
            (8, 0, 5, 0) => true, // ITLBIALL - Invalidate entire I-TLB
            (8, 0, 5, 1) => true, // ITLBIMVA - Invalidate I-TLB by MVA
            (8, 0, 5, 2) => true, // ITLBIASID - Invalidate I-TLB by ASID
            (8, 0, 6, 0) => true, // DTLBIALL - Invalidate entire D-TLB
            (8, 0, 6, 1) => true, // DTLBIMVA - Invalidate D-TLB by MVA
            (8, 0, 6, 2) => true, // DTLBIASID - Invalidate D-TLB by ASID
            (8, 0, 7, 0) => true, // TLBIALL - Invalidate unified TLB
            (8, 0, 7, 1) => true, // TLBIMVA - Invalidate unified TLB by MVA
            (8, 0, 7, 2) => true, // TLBIASID - Invalidate unified TLB by ASID

            // CRn=9: Performance monitors
            (9, 0, 12, 0) => {
                self.pmcr = value;
                true
            }
            (9, 0, 12, 1) => {
                self.pmcntenset |= value;
                true
            }
            (9, 0, 12, 2) => {
                self.pmcntenset &= !value;
                true
            } // PMCNTENCLR
            (9, 0, 13, 0) => {
                self.pmccntr = value;
                true
            }
            (9, 0, 14, 0) => {
                self.pmuserenr = value;
                true
            }

            // CRn=13: Context
            (13, 0, 0, 0) => {
                self.fcseidr = value;
                true
            }
            (13, 0, 0, 1) => {
                self.contextidr = value;
                true
            }
            (13, 0, 0, 2) => {
                self.tpidrurw = value;
                true
            }
            (13, 0, 0, 3) => {
                self.tpidruro = value;
                true
            }
            (13, 0, 0, 4) => {
                self.tpidrprw = value;
                true
            }

            // CRn=14: Timer
            (14, 0, 0, 0) => {
                self.cntfrq = value;
                true
            }
            (14, 0, 2, 0) => {
                self.cntp_tval = value;
                true
            }
            (14, 0, 2, 1) => {
                self.cntp_ctl = value;
                true
            }
            (14, 0, 3, 0) => {
                self.cntv_tval = value;
                true
            }
            (14, 0, 3, 1) => {
                self.cntv_ctl = value;
                true
            }

            _ => false, // Unknown or read-only register
        }
    }

    /// Check if MMU is enabled.
    pub fn mmu_enabled(&self) -> bool {
        self.sctlr.m()
    }

    /// Check if data cache is enabled.
    pub fn dcache_enabled(&self) -> bool {
        self.sctlr.c()
    }

    /// Check if instruction cache is enabled.
    pub fn icache_enabled(&self) -> bool {
        self.sctlr.i()
    }

    /// Check if alignment checking is enabled.
    pub fn alignment_check(&self) -> bool {
        self.sctlr.a()
    }

    /// Check if big-endian mode is enabled.
    pub fn big_endian(&self) -> bool {
        self.sctlr.b()
    }

    /// Check if VFP/NEON is accessible (from CPACR).
    pub fn vfp_accessible(&self) -> bool {
        // CP10 and CP11 must both be enabled
        let cp10 = self.cpacr.cp_access(10);
        let cp11 = self.cpacr.cp_access(11);
        cp10 == 3 && cp11 == 3 // Full access
    }
}

impl fmt::Debug for Cp15State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cp15State {{")?;
        writeln!(f, "  MIDR:    {:08x}", self.midr)?;
        writeln!(f, "  MPIDR:   {:08x}", self.mpidr)?;
        writeln!(f, "  SCTLR:   {:?}", self.sctlr)?;
        writeln!(f, "  CPACR:   {:08x}", self.cpacr.bits())?;
        writeln!(f, "  TTBR0:   {:016x}", self.ttbr0)?;
        writeln!(f, "  TTBR1:   {:016x}", self.ttbr1)?;
        writeln!(f, "  TTBCR:   {:08x}", self.ttbcr)?;
        writeln!(f, "  DACR:    {:08x}", self.dacr)?;
        writeln!(f, "  CONTEXTIDR: {:08x}", self.contextidr)?;
        write!(f, "}}")
    }
}

// =============================================================================
// System Control Register (SCTLR)
// =============================================================================

/// System Control Register (SCTLR).
#[derive(Clone, Copy, Default)]
pub struct Sctlr {
    bits: u32,
}

impl Sctlr {
    /// Create from raw bits.
    pub fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get raw bits.
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// M bit: MMU enable.
    pub fn m(&self) -> bool {
        (self.bits & (1 << 0)) != 0
    }

    /// Set MMU enable.
    pub fn set_m(&mut self, enable: bool) {
        if enable {
            self.bits |= 1 << 0;
        } else {
            self.bits &= !(1 << 0);
        }
    }

    /// A bit: Alignment check enable.
    pub fn a(&self) -> bool {
        (self.bits & (1 << 1)) != 0
    }

    /// C bit: Data/unified cache enable.
    pub fn c(&self) -> bool {
        (self.bits & (1 << 2)) != 0
    }

    /// Set data cache enable.
    pub fn set_c(&mut self, enable: bool) {
        if enable {
            self.bits |= 1 << 2;
        } else {
            self.bits &= !(1 << 2);
        }
    }

    /// B bit: Big-endian mode (deprecated in ARMv7).
    pub fn b(&self) -> bool {
        (self.bits & (1 << 7)) != 0
    }

    /// Z bit: Branch prediction enable.
    pub fn z(&self) -> bool {
        (self.bits & (1 << 11)) != 0
    }

    /// I bit: Instruction cache enable.
    pub fn i(&self) -> bool {
        (self.bits & (1 << 12)) != 0
    }

    /// Set instruction cache enable.
    pub fn set_i(&mut self, enable: bool) {
        if enable {
            self.bits |= 1 << 12;
        } else {
            self.bits &= !(1 << 12);
        }
    }

    /// V bit: High vectors (0xFFFF0000 vs 0x00000000).
    pub fn v(&self) -> bool {
        (self.bits & (1 << 13)) != 0
    }

    /// Get exception vector base address.
    pub fn vector_base(&self) -> u32 {
        if self.v() { 0xFFFF0000 } else { 0x00000000 }
    }

    /// RR bit: Round-robin replacement for caches.
    pub fn rr(&self) -> bool {
        (self.bits & (1 << 14)) != 0
    }

    /// EE bit: Exception endianness.
    pub fn ee(&self) -> bool {
        (self.bits & (1 << 25)) != 0
    }

    /// TRE bit: TEX remap enable.
    pub fn tre(&self) -> bool {
        (self.bits & (1 << 28)) != 0
    }

    /// AFE bit: Access flag enable.
    pub fn afe(&self) -> bool {
        (self.bits & (1 << 29)) != 0
    }

    /// TE bit: Thumb exception enable.
    pub fn te(&self) -> bool {
        (self.bits & (1 << 30)) != 0
    }
}

impl fmt::Debug for Sctlr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SCTLR {{ M:{} A:{} C:{} Z:{} I:{} V:{} EE:{} TE:{} bits:{:08x} }}",
            self.m() as u8,
            self.a() as u8,
            self.c() as u8,
            self.z() as u8,
            self.i() as u8,
            self.v() as u8,
            self.ee() as u8,
            self.te() as u8,
            self.bits
        )
    }
}

// =============================================================================
// Coprocessor Access Control Register (CPACR)
// =============================================================================

/// Coprocessor Access Control Register (CPACR).
#[derive(Clone, Copy, Default)]
pub struct Cpacr {
    bits: u32,
}

impl Cpacr {
    /// Create from raw bits.
    pub fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get raw bits.
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// Get access permissions for a coprocessor (0-13).
    /// Returns: 0=denied, 1=PL1 only, 2=reserved, 3=full access
    pub fn cp_access(&self, cp: u8) -> u8 {
        debug_assert!(cp <= 13);
        ((self.bits >> (cp * 2)) & 3) as u8
    }

    /// Set access permissions for a coprocessor.
    pub fn set_cp_access(&mut self, cp: u8, access: u8) {
        debug_assert!(cp <= 13);
        debug_assert!(access <= 3);
        let shift = cp * 2;
        self.bits = (self.bits & !(3 << shift)) | ((access as u32) << shift);
    }

    /// Check if VFP (CP10/CP11) is fully enabled.
    pub fn vfp_enabled(&self) -> bool {
        self.cp_access(10) == 3 && self.cp_access(11) == 3
    }

    /// Enable VFP (CP10/CP11) full access.
    pub fn enable_vfp(&mut self) {
        self.set_cp_access(10, 3);
        self.set_cp_access(11, 3);
    }

    /// ASEDIS bit: Disable Advanced SIMD.
    pub fn asedis(&self) -> bool {
        (self.bits & (1 << 31)) != 0
    }

    /// D32DIS bit: Disable D16-D31.
    pub fn d32dis(&self) -> bool {
        (self.bits & (1 << 30)) != 0
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cp15_id_registers() {
        let cp15 = Cp15State::new();

        // Read MIDR
        let midr = cp15.read(Cp15Encoding::new(0, 0, 0, 0));
        assert_eq!(midr, Some(0x410FC090));

        // Read MPIDR
        let mpidr = cp15.read(Cp15Encoding::new(0, 0, 0, 5));
        assert_eq!(mpidr, Some(0x80000000));
    }

    #[test]
    fn test_cp15_sctlr() {
        let mut cp15 = Cp15State::new();

        // Write SCTLR with MMU and caches enabled
        let value = (1 << 0) | (1 << 2) | (1 << 12); // M, C, I
        cp15.write(Cp15Encoding::new(1, 0, 0, 0), value);

        assert!(cp15.mmu_enabled());
        assert!(cp15.dcache_enabled());
        assert!(cp15.icache_enabled());

        // Read back
        let read = cp15.read(Cp15Encoding::new(1, 0, 0, 0));
        assert_eq!(read, Some(value));
    }

    #[test]
    fn test_cp15_ttbr() {
        let mut cp15 = Cp15State::new();

        cp15.write(Cp15Encoding::new(2, 0, 0, 0), 0x12345000);
        assert_eq!(cp15.ttbr0, 0x12345000);

        cp15.write(Cp15Encoding::new(2, 0, 0, 1), 0x67890000);
        assert_eq!(cp15.ttbr1, 0x67890000);
    }

    #[test]
    fn test_cp15_context() {
        let mut cp15 = Cp15State::new();

        cp15.write(Cp15Encoding::new(13, 0, 0, 2), 0xDEADBEEF);
        assert_eq!(cp15.tpidrurw, 0xDEADBEEF);

        let read = cp15.read(Cp15Encoding::new(13, 0, 0, 2));
        assert_eq!(read, Some(0xDEADBEEF));
    }

    #[test]
    fn test_cp15_cache_ops() {
        let mut cp15 = Cp15State::new();

        // Cache invalidate operations should succeed (no-op in emulation)
        assert!(cp15.write(Cp15Encoding::new(7, 0, 5, 0), 0)); // ICIALLU
        assert!(cp15.write(Cp15Encoding::new(7, 0, 6, 1), 0x1000)); // DCIMVAC
        assert!(cp15.write(Cp15Encoding::new(7, 0, 14, 1), 0x2000)); // DCCIMVAC
    }

    #[test]
    fn test_cp15_tlb_ops() {
        let mut cp15 = Cp15State::new();

        // TLB invalidate operations should succeed
        assert!(cp15.write(Cp15Encoding::new(8, 0, 7, 0), 0)); // TLBIALL
        assert!(cp15.write(Cp15Encoding::new(8, 0, 7, 1), 0x1000)); // TLBIMVA
    }

    #[test]
    fn test_sctlr_bits() {
        let mut sctlr = Sctlr::default();

        sctlr.set_m(true);
        assert!(sctlr.m());

        sctlr.set_c(true);
        assert!(sctlr.c());

        sctlr.set_i(true);
        assert!(sctlr.i());

        // V bit for high vectors
        sctlr = Sctlr::from_bits(1 << 13);
        assert_eq!(sctlr.vector_base(), 0xFFFF0000);

        sctlr = Sctlr::from_bits(0);
        assert_eq!(sctlr.vector_base(), 0x00000000);
    }

    #[test]
    fn test_cpacr_vfp() {
        let mut cpacr = Cpacr::default();

        assert!(!cpacr.vfp_enabled());

        cpacr.enable_vfp();
        assert!(cpacr.vfp_enabled());
        assert_eq!(cpacr.cp_access(10), 3);
        assert_eq!(cpacr.cp_access(11), 3);
    }

    #[test]
    fn test_id_register_readonly() {
        let mut cp15 = Cp15State::new();
        let original_midr = cp15.midr;

        // Attempt to write to MIDR should fail
        let result = cp15.write(Cp15Encoding::new(0, 0, 0, 0), 0x12345678);
        assert!(!result);
        assert_eq!(cp15.midr, original_midr);
    }
}
