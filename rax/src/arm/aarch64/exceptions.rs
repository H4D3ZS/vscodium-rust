//! AArch64 Exception Handling
//!
//! This module implements the complete AArch64 exception model including:
//! - Synchronous exceptions (SVC, data abort, instruction abort, etc.)
//! - Asynchronous exceptions (IRQ, FIQ, SError)
//! - Exception entry and return
//! - Exception syndrome generation

use super::{spsr, vector_offsets};

// =============================================================================
// Exception Class (EC field of ESR_ELx)
// =============================================================================

/// Exception class values for ESR_ELx.EC field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExceptionClass {
    /// Unknown reason.
    Unknown = 0x00,
    /// Trapped WFI or WFE.
    WfiWfe = 0x01,
    /// Trapped MCR/MRC (CP15).
    McrrMrrc15 = 0x03,
    /// Trapped MCRR/MRRC (CP15).
    McrrMrrc14 = 0x04,
    /// Trapped MCR/MRC (CP14).
    McrMrc14 = 0x05,
    /// Trapped LDC/STC.
    LdcStc = 0x06,
    /// Access to SVE, SIMD, FP disabled.
    SveSmdFp = 0x07,
    /// Trapped VMRS.
    Vmrs = 0x08,
    /// Pointer authentication failure.
    Pac = 0x09,
    /// Trapped LD64B/ST64B*.
    Ld64bSt64b = 0x0A,
    /// Trapped MRRC (CP14).
    Mrrc14 = 0x0C,
    /// Branch Target Exception.
    Bti = 0x0D,
    /// Illegal Execution state.
    IllegalState = 0x0E,
    /// SVC in AArch32.
    SvcAarch32 = 0x11,
    /// HVC in AArch32.
    HvcAarch32 = 0x12,
    /// SMC in AArch32.
    SmcAarch32 = 0x13,
    /// SVC in AArch64.
    SvcAarch64 = 0x15,
    /// HVC in AArch64.
    HvcAarch64 = 0x16,
    /// SMC in AArch64.
    SmcAarch64 = 0x17,
    /// Trapped MSR/MRS/system instruction.
    MsrMrsSys = 0x18,
    /// Access to SVE disabled.
    Sve = 0x19,
    /// Trapped ERET/ERETAA/ERETAB.
    Eret = 0x1A,
    /// Exception from Pointer Authentication.
    PacFail = 0x1C,
    /// Implementation defined (EL3).
    ImplDefined = 0x1F,
    /// Instruction Abort from lower EL.
    InsnAbortLowerEl = 0x20,
    /// Instruction Abort from same EL.
    InsnAbortSameEl = 0x21,
    /// PC alignment fault.
    PcAlignment = 0x22,
    /// Data Abort from lower EL.
    DataAbortLowerEl = 0x24,
    /// Data Abort from same EL.
    DataAbortSameEl = 0x25,
    /// SP alignment fault.
    SpAlignment = 0x26,
    /// Trapped FP exception (AArch32).
    FpExcAarch32 = 0x28,
    /// Trapped FP exception (AArch64).
    FpExcAarch64 = 0x2C,
    /// SError interrupt.
    SError = 0x2F,
    /// Breakpoint from lower EL.
    BreakpointLowerEl = 0x30,
    /// Breakpoint from same EL.
    BreakpointSameEl = 0x31,
    /// Software Step from lower EL.
    SoftwareStepLowerEl = 0x32,
    /// Software Step from same EL.
    SoftwareStepSameEl = 0x33,
    /// Watchpoint from lower EL.
    WatchpointLowerEl = 0x34,
    /// Watchpoint from same EL.
    WatchpointSameEl = 0x35,
    /// BKPT in AArch32.
    BkptAarch32 = 0x38,
    /// Vector Catch (AArch32).
    VectorCatch = 0x3A,
    /// BRK in AArch64.
    BrkAarch64 = 0x3C,
}

impl ExceptionClass {
    /// Create from u8 value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::Unknown),
            0x01 => Some(Self::WfiWfe),
            0x03 => Some(Self::McrrMrrc15),
            0x04 => Some(Self::McrrMrrc14),
            0x05 => Some(Self::McrMrc14),
            0x06 => Some(Self::LdcStc),
            0x07 => Some(Self::SveSmdFp),
            0x08 => Some(Self::Vmrs),
            0x09 => Some(Self::Pac),
            0x0A => Some(Self::Ld64bSt64b),
            0x0C => Some(Self::Mrrc14),
            0x0D => Some(Self::Bti),
            0x0E => Some(Self::IllegalState),
            0x11 => Some(Self::SvcAarch32),
            0x12 => Some(Self::HvcAarch32),
            0x13 => Some(Self::SmcAarch32),
            0x15 => Some(Self::SvcAarch64),
            0x16 => Some(Self::HvcAarch64),
            0x17 => Some(Self::SmcAarch64),
            0x18 => Some(Self::MsrMrsSys),
            0x19 => Some(Self::Sve),
            0x1A => Some(Self::Eret),
            0x1C => Some(Self::PacFail),
            0x1F => Some(Self::ImplDefined),
            0x20 => Some(Self::InsnAbortLowerEl),
            0x21 => Some(Self::InsnAbortSameEl),
            0x22 => Some(Self::PcAlignment),
            0x24 => Some(Self::DataAbortLowerEl),
            0x25 => Some(Self::DataAbortSameEl),
            0x26 => Some(Self::SpAlignment),
            0x28 => Some(Self::FpExcAarch32),
            0x2C => Some(Self::FpExcAarch64),
            0x2F => Some(Self::SError),
            0x30 => Some(Self::BreakpointLowerEl),
            0x31 => Some(Self::BreakpointSameEl),
            0x32 => Some(Self::SoftwareStepLowerEl),
            0x33 => Some(Self::SoftwareStepSameEl),
            0x34 => Some(Self::WatchpointLowerEl),
            0x35 => Some(Self::WatchpointSameEl),
            0x38 => Some(Self::BkptAarch32),
            0x3A => Some(Self::VectorCatch),
            0x3C => Some(Self::BrkAarch64),
            _ => None,
        }
    }
}

// =============================================================================
// Exception Type
// =============================================================================

/// High-level exception type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExceptionType {
    /// Synchronous exception (SVC, abort, etc.).
    Synchronous,
    /// IRQ interrupt.
    Irq,
    /// FIQ interrupt.
    Fiq,
    /// SError (System Error).
    SError,
}

// =============================================================================
// Data Abort / Instruction Abort Fault Status Codes
// =============================================================================

/// Instruction/Data Fault Status Code (DFSC/IFSC in ESR_ELx.ISS).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FaultStatusCode {
    /// Address size fault, level 0.
    AddressSizeL0 = 0b000000,
    /// Address size fault, level 1.
    AddressSizeL1 = 0b000001,
    /// Address size fault, level 2.
    AddressSizeL2 = 0b000010,
    /// Address size fault, level 3.
    AddressSizeL3 = 0b000011,
    /// Translation fault, level 0.
    TranslationL0 = 0b000100,
    /// Translation fault, level 1.
    TranslationL1 = 0b000101,
    /// Translation fault, level 2.
    TranslationL2 = 0b000110,
    /// Translation fault, level 3.
    TranslationL3 = 0b000111,
    /// Access flag fault, level 1.
    AccessFlagL1 = 0b001001,
    /// Access flag fault, level 2.
    AccessFlagL2 = 0b001010,
    /// Access flag fault, level 3.
    AccessFlagL3 = 0b001011,
    /// Permission fault, level 1.
    PermissionL1 = 0b001101,
    /// Permission fault, level 2.
    PermissionL2 = 0b001110,
    /// Permission fault, level 3.
    PermissionL3 = 0b001111,
    /// Synchronous External abort, not on table walk.
    SyncExternal = 0b010000,
    /// Synchronous External abort, level 0.
    SyncExternalL0 = 0b010100,
    /// Synchronous External abort, level 1.
    SyncExternalL1 = 0b010101,
    /// Synchronous External abort, level 2.
    SyncExternalL2 = 0b010110,
    /// Synchronous External abort, level 3.
    SyncExternalL3 = 0b010111,
    /// Synchronous parity or ECC error, not on table walk.
    SyncParity = 0b011000,
    /// Synchronous parity or ECC error, level 0.
    SyncParityL0 = 0b011100,
    /// Synchronous parity or ECC error, level 1.
    SyncParityL1 = 0b011101,
    /// Synchronous parity or ECC error, level 2.
    SyncParityL2 = 0b011110,
    /// Synchronous parity or ECC error, level 3.
    SyncParityL3 = 0b011111,
    /// Alignment fault.
    Alignment = 0b100001,
    /// TLB conflict abort.
    TlbConflict = 0b110000,
    /// Unsupported atomic hardware update.
    AtomicHwUpdate = 0b110001,
    /// Implementation defined fault (Lockdown).
    ImplDefLockdown = 0b110100,
    /// Implementation defined fault (Unsupported exclusive).
    ImplDefExclusive = 0b110101,
}

// =============================================================================
// Syndrome Register (ESR_ELx)
// =============================================================================

/// Exception Syndrome Register builder/parser.
#[derive(Clone, Copy, Debug, Default)]
pub struct SyndromeRegister {
    /// The raw ESR value.
    pub value: u64,
}

impl SyndromeRegister {
    /// Create a new empty syndrome.
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    /// Create from raw value.
    pub const fn from_raw(value: u64) -> Self {
        Self { value }
    }

    /// Get the Exception Class (EC) field.
    pub fn ec(&self) -> u8 {
        ((self.value >> 26) & 0x3F) as u8
    }

    /// Get the Instruction Length (IL) field.
    pub fn il(&self) -> bool {
        (self.value >> 25) & 1 != 0
    }

    /// Get the Instruction Specific Syndrome (ISS) field.
    pub fn iss(&self) -> u32 {
        (self.value & 0x01FF_FFFF) as u32
    }

    /// Get ISS2 field (for Data Abort).
    pub fn iss2(&self) -> u8 {
        ((self.value >> 32) & 0x1F) as u8
    }

    /// Set the Exception Class.
    pub fn set_ec(&mut self, ec: ExceptionClass) {
        self.value = (self.value & !(0x3F << 26)) | ((ec as u64) << 26);
    }

    /// Set the Instruction Length (32-bit = true).
    pub fn set_il(&mut self, is_32bit: bool) {
        if is_32bit {
            self.value |= 1 << 25;
        } else {
            self.value &= !(1 << 25);
        }
    }

    /// Set the ISS field.
    pub fn set_iss(&mut self, iss: u32) {
        self.value = (self.value & !0x01FF_FFFF) | (iss as u64 & 0x01FF_FFFF);
    }

    // =========================================================================
    // Syndrome Builders
    // =========================================================================

    /// Create syndrome for SVC instruction.
    pub fn svc(imm16: u16) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::SvcAarch64);
        esr.set_il(true);
        esr.set_iss(imm16 as u32);
        esr
    }

    /// Create syndrome for HVC instruction.
    pub fn hvc(imm16: u16) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::HvcAarch64);
        esr.set_il(true);
        esr.set_iss(imm16 as u32);
        esr
    }

    /// Create syndrome for SMC instruction.
    pub fn smc(imm16: u16) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::SmcAarch64);
        esr.set_il(true);
        esr.set_iss(imm16 as u32);
        esr
    }

    /// Create syndrome for BRK instruction.
    pub fn brk(imm16: u16) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::BrkAarch64);
        esr.set_il(true);
        esr.set_iss(imm16 as u32);
        esr
    }

    /// Create syndrome for illegal execution state.
    pub fn illegal_state() -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::IllegalState);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for PC alignment fault.
    pub fn pc_alignment() -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::PcAlignment);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for SP alignment fault.
    pub fn sp_alignment() -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::SpAlignment);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for instruction abort.
    pub fn instruction_abort(from_lower_el: bool, fault: FaultStatusCode, s1ptw: bool) -> Self {
        let mut esr = Self::new();
        let ec = if from_lower_el {
            ExceptionClass::InsnAbortLowerEl
        } else {
            ExceptionClass::InsnAbortSameEl
        };
        esr.set_ec(ec);
        esr.set_il(true);

        let mut iss = fault as u32;
        if s1ptw {
            iss |= 1 << 7; // S1PTW bit
        }
        esr.set_iss(iss);
        esr
    }

    /// Create syndrome for data abort.
    pub fn data_abort(
        from_lower_el: bool,
        fault: FaultStatusCode,
        wnr: bool, // Write not Read
        cm: bool,  // Cache maintenance
        s1ptw: bool,
        isv: bool,  // Instruction syndrome valid
        sas: u8,    // Syndrome Access Size
        sse: bool,  // Syndrome Sign Extend
        srt: u8,    // Syndrome Register Transfer
        sf: bool,   // 64-bit register
        ar: bool,   // Acquire/Release
        vncr: bool, // VNCR_EL2 access
        fnv: bool,  // FAR not valid
        ea: bool,   // External abort
        set: u8,    // Synchronous Error Type
    ) -> Self {
        let mut esr = Self::new();
        let ec = if from_lower_el {
            ExceptionClass::DataAbortLowerEl
        } else {
            ExceptionClass::DataAbortSameEl
        };
        esr.set_ec(ec);
        esr.set_il(true);

        let mut iss = fault as u32;
        if wnr {
            iss |= 1 << 6;
        }
        if cm {
            iss |= 1 << 8;
        }
        if s1ptw {
            iss |= 1 << 7;
        }
        if isv {
            iss |= 1 << 24;
            iss |= (sas as u32 & 0x3) << 22;
            if sse {
                iss |= 1 << 21;
            }
            iss |= (srt as u32 & 0x1F) << 16;
            if sf {
                iss |= 1 << 15;
            }
            if ar {
                iss |= 1 << 14;
            }
        }
        if vncr {
            iss |= 1 << 13;
        }
        if fnv {
            iss |= 1 << 10;
        }
        if ea {
            iss |= 1 << 9;
        }
        iss |= (set as u32 & 0x3) << 11;

        esr.set_iss(iss);
        esr
    }

    /// Create syndrome for MSR/MRS/system instruction trap.
    pub fn msr_mrs_trap(
        op0: u8,
        op1: u8,
        crn: u8,
        crm: u8,
        op2: u8,
        rt: u8,
        direction: bool, // true = MRS (read), false = MSR (write)
    ) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::MsrMrsSys);
        esr.set_il(true);

        let iss = ((op0 as u32 & 0x3) << 20)
            | ((op2 as u32 & 0x7) << 17)
            | ((op1 as u32 & 0x7) << 14)
            | ((crn as u32 & 0xF) << 10)
            | ((rt as u32 & 0x1F) << 5)
            | ((crm as u32 & 0xF) << 1)
            | (direction as u32);
        esr.set_iss(iss);
        esr
    }

    /// Create syndrome for WFI/WFE trap.
    pub fn wfi_wfe_trap(is_wfe: bool, cv: bool, cond: u8) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::WfiWfe);
        esr.set_il(true);

        let mut iss = 0u32;
        if is_wfe {
            iss |= 1; // TI bit
        }
        if cv {
            iss |= 1 << 24;
            iss |= (cond as u32 & 0xF) << 20;
        }
        esr.set_iss(iss);
        esr
    }

    /// Create syndrome for SVE/SIMD/FP trap.
    pub fn simd_fp_trap() -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::SveSmdFp);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for BTI exception.
    pub fn bti() -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::Bti);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for unknown reason.
    pub fn unknown() -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::Unknown);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for SError.
    pub fn serror(aet: u8, ea: bool, dfsc: u8) -> Self {
        let mut esr = Self::new();
        esr.set_ec(ExceptionClass::SError);
        esr.set_il(true);

        let iss = ((aet as u32 & 0x7) << 10) | ((ea as u32) << 9) | (dfsc as u32 & 0x3F);
        esr.set_iss(iss);
        esr
    }

    /// Create syndrome for breakpoint.
    pub fn breakpoint(from_lower_el: bool) -> Self {
        let mut esr = Self::new();
        let ec = if from_lower_el {
            ExceptionClass::BreakpointLowerEl
        } else {
            ExceptionClass::BreakpointSameEl
        };
        esr.set_ec(ec);
        esr.set_il(true);
        esr
    }

    /// Create syndrome for watchpoint.
    pub fn watchpoint(from_lower_el: bool, wnr: bool, cm: bool) -> Self {
        let mut esr = Self::new();
        let ec = if from_lower_el {
            ExceptionClass::WatchpointLowerEl
        } else {
            ExceptionClass::WatchpointSameEl
        };
        esr.set_ec(ec);
        esr.set_il(true);

        let mut iss = 0u32;
        if wnr {
            iss |= 1 << 6;
        }
        if cm {
            iss |= 1 << 8;
        }
        esr.set_iss(iss);
        esr
    }

    /// Create syndrome for software step.
    pub fn software_step(from_lower_el: bool, isv: bool, ex: bool) -> Self {
        let mut esr = Self::new();
        let ec = if from_lower_el {
            ExceptionClass::SoftwareStepLowerEl
        } else {
            ExceptionClass::SoftwareStepSameEl
        };
        esr.set_ec(ec);
        esr.set_il(true);

        let iss = ((isv as u32) << 24) | ((ex as u32) << 6);
        esr.set_iss(iss);
        esr
    }
}

// =============================================================================
// Vector Table Offset Calculation
// =============================================================================

/// Calculate the vector table offset for an exception.
pub fn vector_offset(
    exc_type: ExceptionType,
    from_el: u8,
    to_el: u8,
    from_aarch64: bool,
    use_sp_elx: bool,
) -> u64 {
    if from_el == to_el {
        // Same EL
        let base = if use_sp_elx {
            vector_offsets::CURR_EL_SPX_SYNC
        } else {
            vector_offsets::CURR_EL_SP0_SYNC
        };

        base + match exc_type {
            ExceptionType::Synchronous => 0x000,
            ExceptionType::Irq => 0x080,
            ExceptionType::Fiq => 0x100,
            ExceptionType::SError => 0x180,
        }
    } else {
        // Lower EL
        let base = if from_aarch64 {
            vector_offsets::LOWER_EL_A64_SYNC
        } else {
            vector_offsets::LOWER_EL_A32_SYNC
        };

        base + match exc_type {
            ExceptionType::Synchronous => 0x000,
            ExceptionType::Irq => 0x080,
            ExceptionType::Fiq => 0x100,
            ExceptionType::SError => 0x180,
        }
    }
}

/// Build SPSR value from current processor state.
pub fn build_spsr(
    nzcv: u8,
    daif: u8,
    current_el: u8,
    sp_sel: bool,
    ssbs: bool,
    pan: bool,
    uao: bool,
    dit: bool,
    tco: bool,
    btype: u8,
    il: bool,
    ss: bool,
) -> u64 {
    let mut spsr = 0u64;

    // Mode field
    let mode = match (current_el, sp_sel) {
        (0, _) => spsr::EL0T,
        (1, false) => spsr::EL1T,
        (1, true) => spsr::EL1H,
        (2, false) => spsr::EL2T,
        (2, true) => spsr::EL2H,
        (3, false) => spsr::EL3T,
        (3, true) => spsr::EL3H,
        _ => spsr::EL0T,
    };
    spsr |= mode;

    // DAIF bits
    if daif & 0x8 != 0 {
        spsr |= spsr::D;
    }
    if daif & 0x4 != 0 {
        spsr |= spsr::A;
    }
    if daif & 0x2 != 0 {
        spsr |= spsr::I;
    }
    if daif & 0x1 != 0 {
        spsr |= spsr::F;
    }

    // NZCV
    if nzcv & 0x8 != 0 {
        spsr |= spsr::N;
    }
    if nzcv & 0x4 != 0 {
        spsr |= spsr::Z;
    }
    if nzcv & 0x2 != 0 {
        spsr |= spsr::C;
    }
    if nzcv & 0x1 != 0 {
        spsr |= spsr::V;
    }

    // Other bits
    if ssbs {
        spsr |= spsr::SSBS;
    }
    if pan {
        spsr |= spsr::PAN;
    }
    if uao {
        spsr |= spsr::UAO;
    }
    if dit {
        spsr |= spsr::DIT;
    }
    if tco {
        spsr |= spsr::TCO;
    }
    spsr |= ((btype as u64) & 0x3) << 10;
    if il {
        spsr |= spsr::IL;
    }
    if ss {
        spsr |= spsr::SS;
    }

    spsr
}

/// Extract processor state from SPSR value.
pub fn parse_spsr(
    spsr: u64,
) -> (
    u8,
    u8,
    u8,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    u8,
    bool,
    bool,
) {
    let mode = (spsr & spsr::M_MASK) as u8;
    let (el, sp_sel) = match mode {
        0b0000 => (0, false), // EL0t
        0b0100 => (1, false), // EL1t
        0b0101 => (1, true),  // EL1h
        0b1000 => (2, false), // EL2t
        0b1001 => (2, true),  // EL2h
        0b1100 => (3, false), // EL3t
        0b1101 => (3, true),  // EL3h
        _ => (0, false),
    };

    let mut nzcv = 0u8;
    if spsr & spsr::N != 0 {
        nzcv |= 0x8;
    }
    if spsr & spsr::Z != 0 {
        nzcv |= 0x4;
    }
    if spsr & spsr::C != 0 {
        nzcv |= 0x2;
    }
    if spsr & spsr::V != 0 {
        nzcv |= 0x1;
    }

    let mut daif = 0u8;
    if spsr & spsr::D != 0 {
        daif |= 0x8;
    }
    if spsr & spsr::A != 0 {
        daif |= 0x4;
    }
    if spsr & spsr::I != 0 {
        daif |= 0x2;
    }
    if spsr & spsr::F != 0 {
        daif |= 0x1;
    }

    let ssbs = spsr & spsr::SSBS != 0;
    let pan = spsr & spsr::PAN != 0;
    let uao = spsr & spsr::UAO != 0;
    let dit = spsr & spsr::DIT != 0;
    let tco = spsr & spsr::TCO != 0;
    let btype = ((spsr >> 10) & 0x3) as u8;
    let il = spsr & spsr::IL != 0;
    let ss = spsr & spsr::SS != 0;

    (
        nzcv, daif, el, sp_sel, ssbs, pan, uao, dit, tco, btype, il, ss,
    )
}

/// Determine target EL for an exception.
pub fn exception_target_el(
    exc_type: ExceptionType,
    current_el: u8,
    hcr_el2: u64,
    scr_el3: u64,
) -> u8 {
    use super::{hcr, scr};

    match exc_type {
        ExceptionType::Synchronous => {
            // Synchronous exceptions go to current EL or higher
            current_el.max(1)
        }
        ExceptionType::Irq => {
            // Check routing
            if current_el == 3 {
                3
            } else if scr_el3 & scr::IRQ != 0 {
                3
            } else if current_el < 2 && hcr_el2 & hcr::IMO != 0 {
                2
            } else {
                1.max(current_el)
            }
        }
        ExceptionType::Fiq => {
            if current_el == 3 {
                3
            } else if scr_el3 & scr::FIQ != 0 {
                3
            } else if current_el < 2 && hcr_el2 & hcr::FMO != 0 {
                2
            } else {
                1.max(current_el)
            }
        }
        ExceptionType::SError => {
            if current_el == 3 {
                3
            } else if scr_el3 & scr::EA != 0 {
                3
            } else if current_el < 2 && hcr_el2 & hcr::AMO != 0 {
                2
            } else {
                1.max(current_el)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syndrome_svc() {
        let esr = SyndromeRegister::svc(0x1234);
        assert_eq!(esr.ec(), ExceptionClass::SvcAarch64 as u8);
        assert!(esr.il());
        assert_eq!(esr.iss(), 0x1234);
    }

    #[test]
    fn test_syndrome_data_abort() {
        let esr = SyndromeRegister::data_abort(
            true,                           // from lower EL
            FaultStatusCode::TranslationL3, // translation fault
            true,                           // write
            false,                          // not cache maintenance
            false,                          // not S1PTW
            false,                          // no ISV
            0,
            false,
            0,
            false,
            false,
            false,
            false,
            false,
            0,
        );
        assert_eq!(esr.ec(), ExceptionClass::DataAbortLowerEl as u8);
        assert!(esr.il());
        assert_eq!(esr.iss() & 0x3F, FaultStatusCode::TranslationL3 as u32);
        assert!(esr.iss() & (1 << 6) != 0); // WnR bit
    }

    #[test]
    fn test_vector_offset() {
        // Same EL, SP_ELx, synchronous
        assert_eq!(
            vector_offset(ExceptionType::Synchronous, 1, 1, true, true),
            vector_offsets::CURR_EL_SPX_SYNC
        );

        // Same EL, SP_ELx, IRQ
        assert_eq!(
            vector_offset(ExceptionType::Irq, 1, 1, true, true),
            vector_offsets::CURR_EL_SPX_IRQ
        );

        // Lower EL, AArch64, synchronous
        assert_eq!(
            vector_offset(ExceptionType::Synchronous, 0, 1, true, true),
            vector_offsets::LOWER_EL_A64_SYNC
        );

        // Lower EL, AArch32, FIQ
        assert_eq!(
            vector_offset(ExceptionType::Fiq, 0, 1, false, true),
            vector_offsets::LOWER_EL_A32_FIQ
        );
    }

    #[test]
    fn test_spsr_build_parse() {
        let spsr = build_spsr(
            0b1010, // NZCV = NZ
            0b1111, // DAIF all masked
            1,      // EL1
            true,   // SP_EL1
            false,  // SSBS
            true,   // PAN
            false,  // UAO
            true,   // DIT
            false,  // TCO
            0,      // BTYPE
            false,  // IL
            false,  // SS
        );

        let (nzcv, daif, el, sp_sel, ssbs, pan, uao, dit, tco, btype, il, ss) = parse_spsr(spsr);

        assert_eq!(nzcv, 0b1010);
        assert_eq!(daif, 0b1111);
        assert_eq!(el, 1);
        assert!(sp_sel);
        assert!(!ssbs);
        assert!(pan);
        assert!(!uao);
        assert!(dit);
        assert!(!tco);
        assert_eq!(btype, 0);
        assert!(!il);
        assert!(!ss);
    }
}
