//! Control and status register (CSR) definitions.
//!
//! Only addresses and human-readable names live here; the live CSR *state* is
//! held by [`crate::riscv::cpu::RiscVCpu`] so that floating-point control bits
//! (`fcsr`/`frm`/`fflags`) stay coherent with the FP execution path.

/// Well-known CSR addresses (12-bit).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum Csr {
    /// Accrued floating-point exception flags (NV/DZ/OF/UF/NX).
    Fflags = 0x001,
    /// Floating-point dynamic rounding mode.
    Frm = 0x002,
    /// Unified floating-point control/status (`frm` || `fflags`).
    Fcsr = 0x003,
    /// Cycle counter (low XLEN bits).
    Cycle = 0xC00,
    /// Wall-clock time.
    Time = 0xC01,
    /// Instructions-retired counter.
    Instret = 0xC02,
    /// Upper 32 bits of `cycle` (RV32 only).
    CycleH = 0xC80,
    /// Upper 32 bits of `time` (RV32 only).
    TimeH = 0xC81,
    /// Upper 32 bits of `instret` (RV32 only).
    InstretH = 0xC82,
    /// Machine status.
    Mstatus = 0x300,
    /// Machine ISA register.
    Misa = 0x301,
    /// Machine exception delegation.
    Medeleg = 0x302,
    /// Machine interrupt delegation.
    Mideleg = 0x303,
    /// Machine interrupt-enable.
    Mie = 0x304,
    /// Machine trap-vector base address.
    Mtvec = 0x305,
    /// Machine counter-enable.
    Mcounteren = 0x306,
    /// Machine scratch register.
    Mscratch = 0x340,
    /// Machine exception program counter.
    Mepc = 0x341,
    /// Machine trap cause.
    Mcause = 0x342,
    /// Machine trap value.
    Mtval = 0x343,
    /// Machine interrupt-pending.
    Mip = 0x344,
    /// Vendor ID.
    Mvendorid = 0xF11,
    /// Architecture ID.
    Marchid = 0xF12,
    /// Implementation ID.
    Mimpid = 0xF13,
    /// Hart ID.
    Mhartid = 0xF14,
    /// Vector start position.
    Vstart = 0x008,
    /// Vector fixed-point saturation flag.
    Vxsat = 0x009,
    /// Vector fixed-point rounding mode.
    Vxrm = 0x00A,
    /// Vector control/status (`vxrm` || `vxsat`).
    Vcsr = 0x00F,
    /// Vector length.
    Vl = 0xC20,
    /// Vector data type.
    Vtype = 0xC21,
    /// Vector register length in bytes.
    Vlenb = 0xC22,
}

impl Csr {
    /// Resolve a raw 12-bit address to a known CSR, if recognized.
    pub fn from_addr(addr: u16) -> Option<Csr> {
        Some(match addr & 0xfff {
            0x001 => Csr::Fflags,
            0x002 => Csr::Frm,
            0x003 => Csr::Fcsr,
            0xC00 => Csr::Cycle,
            0xC01 => Csr::Time,
            0xC02 => Csr::Instret,
            0xC80 => Csr::CycleH,
            0xC81 => Csr::TimeH,
            0xC82 => Csr::InstretH,
            0x300 => Csr::Mstatus,
            0x301 => Csr::Misa,
            0x302 => Csr::Medeleg,
            0x303 => Csr::Mideleg,
            0x304 => Csr::Mie,
            0x305 => Csr::Mtvec,
            0x306 => Csr::Mcounteren,
            0x340 => Csr::Mscratch,
            0x341 => Csr::Mepc,
            0x342 => Csr::Mcause,
            0x343 => Csr::Mtval,
            0x344 => Csr::Mip,
            0xF11 => Csr::Mvendorid,
            0xF12 => Csr::Marchid,
            0xF13 => Csr::Mimpid,
            0xF14 => Csr::Mhartid,
            0x008 => Csr::Vstart,
            0x009 => Csr::Vxsat,
            0x00A => Csr::Vxrm,
            0x00F => Csr::Vcsr,
            0xC20 => Csr::Vl,
            0xC21 => Csr::Vtype,
            0xC22 => Csr::Vlenb,
            _ => return None,
        })
    }

    /// `true` if the CSR address encodes a read-only register (top two bits
    /// `0b11`). Writes to such CSRs raise an illegal-instruction exception.
    #[inline]
    pub fn is_read_only(addr: u16) -> bool {
        (addr >> 10) & 0b11 == 0b11
    }
}

/// Human-readable name for a CSR address (falls back to `csr@0xNNN`).
pub fn csr_name(addr: u16) -> String {
    match Csr::from_addr(addr) {
        Some(c) => format!("{c:?}").to_lowercase(),
        None => format!("csr@{:#05x}", addr & 0xfff),
    }
}
