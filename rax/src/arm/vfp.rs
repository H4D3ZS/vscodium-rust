//! ARM VFP (Vector Floating Point) and NEON SIMD support.
//!
//! This module provides floating-point execution support for ARM processors,
//! including:
//! - VFPv2/VFPv3/VFPv4 scalar floating-point operations
//! - Basic NEON SIMD operations
//! - Floating-point state management (FPSCR, registers)
//!
//! # Register Organization
//!
//! VFP/NEON registers are aliased:
//! - S0-S31: 32-bit single-precision registers
//! - D0-D31: 64-bit double-precision registers (D0 = {S1, S0}, D1 = {S3, S2}, etc.)
//! - Q0-Q15: 128-bit quadword registers (Q0 = {D1, D0}, Q1 = {D3, D2}, etc.)
//!
//! # FPSCR (Floating-Point Status and Control Register)
//!
//! - Bits 31-28: N, Z, C, V condition flags
//! - Bits 26-25: Stride (deprecated in VFPv3+)
//! - Bit 24: FZ (flush-to-zero mode)
//! - Bits 23-22: Rounding mode
//! - Bits 21-20: Len (deprecated in VFPv3+)
//! - Bits 19-16: IDE, IXE, UFE, OFE, DZE, IOE (exception enable bits)
//! - Bits 8-4: IDC, IXC, UFC, OFC, DZC, IOC (cumulative exception flags)

use std::fmt;

/// VFP/NEON state for ARM processor.
#[derive(Clone)]
pub struct VfpState {
    /// 64-bit floating-point registers (D0-D31).
    /// S registers are accessed as low/high 32 bits of D registers.
    pub dregs: [u64; 32],

    /// Floating-Point Status and Control Register.
    pub fpscr: Fpscr,

    /// Floating-Point Exception Register (VFPv3+).
    pub fpexc: u32,

    /// Floating-Point System ID Register.
    pub fpsid: u32,

    /// Media and VFP Feature Registers.
    pub mvfr0: u32,
    pub mvfr1: u32,
    pub mvfr2: u32,
}

impl Default for VfpState {
    fn default() -> Self {
        Self {
            dregs: [0; 32],
            fpscr: Fpscr::default(),
            fpexc: 0x4000_0000, // EN bit set by default
            fpsid: 0x4103_3070, // Implementation ID for a VFPv3-style unit.
            // MVFR values indicating VFPv3 + NEON support
            mvfr0: 0x10110222, // VFPv3 with single and double precision
            mvfr1: 0x11111111, // NEON support
            mvfr2: 0x00000000,
        }
    }
}

impl VfpState {
    /// Create a new VFP state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if VFP is enabled.
    pub fn is_enabled(&self) -> bool {
        (self.fpexc & 0x4000_0000) != 0
    }

    /// Read a single-precision register (S0-S31).
    pub fn read_s(&self, reg: u8) -> f32 {
        let dreg = (reg / 2) as usize;
        let is_high = (reg & 1) != 0;
        let bits = if is_high {
            (self.dregs[dreg] >> 32) as u32
        } else {
            self.dregs[dreg] as u32
        };
        f32::from_bits(bits)
    }

    /// Write a single-precision register (S0-S31).
    pub fn write_s(&mut self, reg: u8, value: f32) {
        let dreg = (reg / 2) as usize;
        let is_high = (reg & 1) != 0;
        let bits = value.to_bits() as u64;
        if is_high {
            self.dregs[dreg] = (self.dregs[dreg] & 0xFFFF_FFFF) | (bits << 32);
        } else {
            self.dregs[dreg] = (self.dregs[dreg] & 0xFFFF_FFFF_0000_0000) | bits;
        }
    }

    /// Read a single-precision register as raw bits.
    pub fn read_s_bits(&self, reg: u8) -> u32 {
        let dreg = (reg / 2) as usize;
        let is_high = (reg & 1) != 0;
        if is_high {
            (self.dregs[dreg] >> 32) as u32
        } else {
            self.dregs[dreg] as u32
        }
    }

    /// Write a single-precision register as raw bits.
    pub fn write_s_bits(&mut self, reg: u8, bits: u32) {
        let dreg = (reg / 2) as usize;
        let is_high = (reg & 1) != 0;
        if is_high {
            self.dregs[dreg] = (self.dregs[dreg] & 0xFFFF_FFFF) | ((bits as u64) << 32);
        } else {
            self.dregs[dreg] = (self.dregs[dreg] & 0xFFFF_FFFF_0000_0000) | (bits as u64);
        }
    }

    /// Read a scalar half-precision value from the low half of an S register.
    pub fn read_h_bits(&self, reg: u8) -> u16 {
        (self.read_s_bits(reg) & 0xFFFF) as u16
    }

    /// Write a scalar half-precision value to the low half of an S register.
    pub fn write_h_bits(&mut self, reg: u8, bits: u16) {
        let old = self.read_s_bits(reg);
        self.write_s_bits(reg, (old & 0xFFFF_0000) | bits as u32);
    }

    /// Read a double-precision register (D0-D31).
    pub fn read_d(&self, reg: u8) -> f64 {
        f64::from_bits(self.dregs[reg as usize])
    }

    /// Write a double-precision register (D0-D31).
    pub fn write_d(&mut self, reg: u8, value: f64) {
        self.dregs[reg as usize] = value.to_bits();
    }

    /// Read a double-precision register as raw bits.
    pub fn read_d_bits(&self, reg: u8) -> u64 {
        self.dregs[reg as usize]
    }

    /// Write a double-precision register as raw bits.
    pub fn write_d_bits(&mut self, reg: u8, bits: u64) {
        self.dregs[reg as usize] = bits;
    }

    /// Read a quadword register (Q0-Q15) as two 64-bit values.
    pub fn read_q(&self, reg: u8) -> (u64, u64) {
        let dreg = (reg * 2) as usize;
        (self.dregs[dreg], self.dregs[dreg + 1])
    }

    /// Write a quadword register (Q0-Q15) from two 64-bit values.
    pub fn write_q(&mut self, reg: u8, low: u64, high: u64) {
        let dreg = (reg * 2) as usize;
        self.dregs[dreg] = low;
        self.dregs[dreg + 1] = high;
    }
}

impl fmt::Debug for VfpState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "VfpState {{")?;
        writeln!(f, "  FPSCR: {:08x}", self.fpscr.bits())?;
        writeln!(f, "  FPEXC: {:08x}", self.fpexc)?;
        writeln!(f, "  FPSID: {:08x}", self.fpsid)?;
        for i in 0..32 {
            if self.dregs[i] != 0 {
                writeln!(f, "  D{}: {:016x}", i, self.dregs[i])?;
            }
        }
        write!(f, "}}")
    }
}

/// Floating-Point Status and Control Register.
#[derive(Clone, Copy, Default)]
pub struct Fpscr {
    /// Raw 32-bit value.
    bits: u32,
}

impl Fpscr {
    /// Create FPSCR from raw bits.
    pub fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get raw bits.
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// Negative flag.
    pub fn n(&self) -> bool {
        (self.bits & (1 << 31)) != 0
    }

    /// Zero flag.
    pub fn z(&self) -> bool {
        (self.bits & (1 << 30)) != 0
    }

    /// Carry flag.
    pub fn c(&self) -> bool {
        (self.bits & (1 << 29)) != 0
    }

    /// Overflow flag.
    pub fn v(&self) -> bool {
        (self.bits & (1 << 28)) != 0
    }

    /// Set condition flags.
    pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.bits = (self.bits & 0x0FFF_FFFF)
            | ((n as u32) << 31)
            | ((z as u32) << 30)
            | ((c as u32) << 29)
            | ((v as u32) << 28);
    }

    /// Cumulative saturation flag.
    pub fn qc(&self) -> bool {
        (self.bits & (1 << 27)) != 0
    }

    /// Set cumulative saturation flag.
    pub fn set_qc(&mut self, qc: bool) {
        if qc {
            self.bits |= 1 << 27;
        } else {
            self.bits &= !(1 << 27);
        }
    }

    /// Flush-to-zero mode.
    pub fn fz(&self) -> bool {
        (self.bits & (1 << 24)) != 0
    }

    /// Set flush-to-zero mode.
    pub fn set_fz(&mut self, fz: bool) {
        if fz {
            self.bits |= 1 << 24;
        } else {
            self.bits &= !(1 << 24);
        }
    }

    /// Default NaN mode.
    pub fn dn(&self) -> bool {
        (self.bits & (1 << 25)) != 0
    }

    /// Set default NaN mode.
    pub fn set_dn(&mut self, dn: bool) {
        if dn {
            self.bits |= 1 << 25;
        } else {
            self.bits &= !(1 << 25);
        }
    }

    /// Rounding mode (0=RN, 1=RP, 2=RM, 3=RZ).
    pub fn rmode(&self) -> RoundingMode {
        match (self.bits >> 22) & 3 {
            0 => RoundingMode::RoundNearest,
            1 => RoundingMode::RoundPlusInf,
            2 => RoundingMode::RoundMinusInf,
            3 => RoundingMode::RoundZero,
            _ => unreachable!(),
        }
    }

    /// Set rounding mode.
    pub fn set_rmode(&mut self, mode: RoundingMode) {
        self.bits = (self.bits & !(3 << 22)) | ((mode as u32) << 22);
    }

    /// Invalid operation cumulative flag (IOC).
    pub fn ioc(&self) -> bool {
        (self.bits & 1) != 0
    }

    /// Set invalid operation cumulative flag.
    pub fn set_ioc(&mut self, ioc: bool) {
        if ioc {
            self.bits |= 1;
        } else {
            self.bits &= !1;
        }
    }

    /// Division by zero cumulative flag (DZC).
    pub fn dzc(&self) -> bool {
        (self.bits & (1 << 1)) != 0
    }

    /// Set division by zero cumulative flag.
    pub fn set_dzc(&mut self, dzc: bool) {
        if dzc {
            self.bits |= 1 << 1;
        } else {
            self.bits &= !(1 << 1);
        }
    }

    /// Overflow cumulative flag (OFC).
    pub fn ofc(&self) -> bool {
        (self.bits & (1 << 2)) != 0
    }

    /// Set overflow cumulative flag.
    pub fn set_ofc(&mut self, ofc: bool) {
        if ofc {
            self.bits |= 1 << 2;
        } else {
            self.bits &= !(1 << 2);
        }
    }

    /// Underflow cumulative flag (UFC).
    pub fn ufc(&self) -> bool {
        (self.bits & (1 << 3)) != 0
    }

    /// Set underflow cumulative flag.
    pub fn set_ufc(&mut self, ufc: bool) {
        if ufc {
            self.bits |= 1 << 3;
        } else {
            self.bits &= !(1 << 3);
        }
    }

    /// Inexact cumulative flag (IXC).
    pub fn ixc(&self) -> bool {
        (self.bits & (1 << 4)) != 0
    }

    /// Set inexact cumulative flag.
    pub fn set_ixc(&mut self, ixc: bool) {
        if ixc {
            self.bits |= 1 << 4;
        } else {
            self.bits &= !(1 << 4);
        }
    }

    /// Input denormal cumulative flag (IDC).
    pub fn idc(&self) -> bool {
        (self.bits & (1 << 7)) != 0
    }

    /// Set input denormal cumulative flag.
    pub fn set_idc(&mut self, idc: bool) {
        if idc {
            self.bits |= 1 << 7;
        } else {
            self.bits &= !(1 << 7);
        }
    }
}

impl fmt::Debug for Fpscr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FPSCR {{ N:{} Z:{} C:{} V:{} FZ:{} DN:{} RM:{:?} IOC:{} DZC:{} OFC:{} UFC:{} IXC:{} IDC:{} }}",
            self.n() as u8,
            self.z() as u8,
            self.c() as u8,
            self.v() as u8,
            self.fz() as u8,
            self.dn() as u8,
            self.rmode(),
            self.ioc() as u8,
            self.dzc() as u8,
            self.ofc() as u8,
            self.ufc() as u8,
            self.ixc() as u8,
            self.idc() as u8
        )
    }
}

/// IEEE 754 rounding modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RoundingMode {
    /// Round to nearest, ties to even.
    RoundNearest = 0,
    /// Round toward positive infinity.
    RoundPlusInf = 1,
    /// Round toward negative infinity.
    RoundMinusInf = 2,
    /// Round toward zero (truncate).
    RoundZero = 3,
    /// Round to nearest, ties away from zero.
    RoundTiesAway = 4,
}

/// VFP data type for operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VfpDataType {
    /// 32-bit single precision.
    F32,
    /// 64-bit double precision.
    F64,
}

// =============================================================================
// VFP Execution Helpers
// =============================================================================

/// Execute single-precision addition.
pub fn vadd_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = a + b;
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision addition.
pub fn vadd_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = a + b;
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision addition.
pub fn vadd_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vadd_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Execute single-precision subtraction.
pub fn vsub_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = a - b;
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision subtraction.
pub fn vsub_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = a - b;
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision subtraction.
pub fn vsub_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vsub_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Execute single-precision multiplication.
pub fn vmul_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = a * b;
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision multiplication.
pub fn vmul_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = a * b;
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision multiplication.
pub fn vmul_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vmul_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Execute single-precision division.
pub fn vdiv_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    if b == 0.0 {
        fpscr.set_dzc(true);
    }
    let result = a / b;
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision division.
pub fn vdiv_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    if b == 0.0 {
        fpscr.set_dzc(true);
    }
    let result = a / b;
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision division.
pub fn vdiv_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vdiv_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Execute single-precision negation.
pub fn vneg_f32(a: f32) -> f32 {
    -a
}

/// Execute double-precision negation.
pub fn vneg_f64(a: f64) -> f64 {
    -a
}

/// Execute half-precision negation.
pub fn vneg_f16_bits(a: u16) -> u16 {
    a ^ 0x8000
}

/// Execute single-precision absolute value.
pub fn vabs_f32(a: f32) -> f32 {
    a.abs()
}

/// Execute double-precision absolute value.
pub fn vabs_f64(a: f64) -> f64 {
    a.abs()
}

/// Execute half-precision absolute value.
pub fn vabs_f16_bits(a: u16) -> u16 {
    a & 0x7FFF
}

/// Execute single-precision square root.
pub fn vsqrt_f32(a: f32, fpscr: &mut Fpscr) -> f32 {
    if a < 0.0 {
        fpscr.set_ioc(true);
    }
    let result = a.sqrt();
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision square root.
pub fn vsqrt_f64(a: f64, fpscr: &mut Fpscr) -> f64 {
    if a < 0.0 {
        fpscr.set_ioc(true);
    }
    let result = a.sqrt();
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision square root.
pub fn vsqrt_f16_bits(a: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(vsqrt_f32(vcvt_f32_f16_bits(a), fpscr), fpscr)
}

/// Execute single-precision multiply-accumulate.
pub fn vmla_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = acc + (a * b);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision multiply-accumulate.
pub fn vmla_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = acc + (a * b);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision multiply-accumulate.
pub fn vmla_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vmla_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision multiply-subtract.
pub fn vmls_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = acc - (a * b);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision multiply-subtract.
pub fn vmls_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = acc - (a * b);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision multiply-subtract.
pub fn vmls_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vmls_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision fused multiply-accumulate.
pub fn vfma_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = a.mul_add(b, acc);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision fused multiply-accumulate.
pub fn vfma_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = a.mul_add(b, acc);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision fused multiply-accumulate.
pub fn vfma_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vfma_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision fused multiply-subtract.
pub fn vfms_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = (-a).mul_add(b, acc);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision fused multiply-subtract.
pub fn vfms_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = (-a).mul_add(b, acc);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision fused multiply-subtract.
pub fn vfms_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vfms_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision negated multiply-accumulate.
pub fn vnmla_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = -(acc + (a * b));
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision negated multiply-accumulate.
pub fn vnmla_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = -(acc + (a * b));
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision negated multiply-accumulate.
pub fn vnmla_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vnmla_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision negated multiply-subtract.
pub fn vnmls_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = -(acc - (a * b));
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision negated multiply-subtract.
pub fn vnmls_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = -(acc - (a * b));
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision negated multiply-subtract.
pub fn vnmls_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vnmls_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision fused negated multiply-accumulate.
pub fn vfnma_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = (-a).mul_add(b, -acc);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision fused negated multiply-accumulate.
pub fn vfnma_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = (-a).mul_add(b, -acc);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision fused negated multiply-accumulate.
pub fn vfnma_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vfnma_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision fused negated multiply-subtract.
pub fn vfnms_f32(acc: f32, a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = a.mul_add(b, -acc);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision fused negated multiply-subtract.
pub fn vfnms_f64(acc: f64, a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = a.mul_add(b, -acc);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision fused negated multiply-subtract.
pub fn vfnms_f16_bits(acc: u16, a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vfnms_f32(
            vcvt_f32_f16_bits(acc),
            vcvt_f32_f16_bits(a),
            vcvt_f32_f16_bits(b),
            fpscr,
        ),
        fpscr,
    )
}

/// Execute single-precision negated multiply.
pub fn vnmul_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    let result = -(a * b);
    update_fpscr_after_op(result, fpscr);
    result
}

/// Execute double-precision negated multiply.
pub fn vnmul_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    let result = -(a * b);
    update_fpscr_after_op_f64(result, fpscr);
    result
}

/// Execute half-precision negated multiply.
pub fn vnmul_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vnmul_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Execute single-precision maxNum.
pub fn vmaxnm_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    if a.is_nan() || b.is_nan() {
        fpscr.set_ioc(true);
        return if a.is_nan() && b.is_nan() {
            f32::NAN
        } else if a.is_nan() {
            b
        } else {
            a
        };
    }
    if a == b {
        if a.is_sign_positive() || b.is_sign_positive() {
            0.0
        } else {
            a
        }
    } else {
        a.max(b)
    }
}

/// Execute double-precision maxNum.
pub fn vmaxnm_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    if a.is_nan() || b.is_nan() {
        fpscr.set_ioc(true);
        return if a.is_nan() && b.is_nan() {
            f64::NAN
        } else if a.is_nan() {
            b
        } else {
            a
        };
    }
    if a == b {
        if a.is_sign_positive() || b.is_sign_positive() {
            0.0
        } else {
            a
        }
    } else {
        a.max(b)
    }
}

/// Execute half-precision maxNum.
pub fn vmaxnm_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vmaxnm_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Execute single-precision minNum.
pub fn vminnm_f32(a: f32, b: f32, fpscr: &mut Fpscr) -> f32 {
    if a.is_nan() || b.is_nan() {
        fpscr.set_ioc(true);
        return if a.is_nan() && b.is_nan() {
            f32::NAN
        } else if a.is_nan() {
            b
        } else {
            a
        };
    }
    if a == b {
        if a.is_sign_negative() || b.is_sign_negative() {
            -0.0
        } else {
            a
        }
    } else {
        a.min(b)
    }
}

/// Execute double-precision minNum.
pub fn vminnm_f64(a: f64, b: f64, fpscr: &mut Fpscr) -> f64 {
    if a.is_nan() || b.is_nan() {
        fpscr.set_ioc(true);
        return if a.is_nan() && b.is_nan() {
            f64::NAN
        } else if a.is_nan() {
            b
        } else {
            a
        };
    }
    if a == b {
        if a.is_sign_negative() || b.is_sign_negative() {
            -0.0
        } else {
            a
        }
    } else {
        a.min(b)
    }
}

/// Execute half-precision minNum.
pub fn vminnm_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) -> u16 {
    vcvt_f16_bits_f32(
        vminnm_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr),
        fpscr,
    )
}

/// Compare single-precision values, updating FPSCR flags.
pub fn vcmp_f32(a: f32, b: f32, fpscr: &mut Fpscr) {
    vcmp_f32_with_exception(a, b, false, fpscr);
}

/// Compare single-precision values, optionally signaling invalid operation for any NaN.
pub fn vcmp_f32_with_exception(a: f32, b: f32, signal_all_nans: bool, fpscr: &mut Fpscr) {
    let (n, z, c, v) = if a.is_nan() || b.is_nan() {
        if signal_all_nans || is_snan_f32(a) || is_snan_f32(b) {
            fpscr.set_ioc(true);
        }
        (false, false, true, true)
    } else if a == b {
        (false, true, true, false)
    } else if a < b {
        (true, false, false, false)
    } else {
        (false, false, true, false)
    };
    fpscr.set_nzcv(n, z, c, v);
}

/// Compare double-precision values, updating FPSCR flags.
pub fn vcmp_f64(a: f64, b: f64, fpscr: &mut Fpscr) {
    vcmp_f64_with_exception(a, b, false, fpscr);
}

/// Compare double-precision values, optionally signaling invalid operation for any NaN.
pub fn vcmp_f64_with_exception(a: f64, b: f64, signal_all_nans: bool, fpscr: &mut Fpscr) {
    let (n, z, c, v) = if a.is_nan() || b.is_nan() {
        if signal_all_nans || is_snan_f64(a) || is_snan_f64(b) {
            fpscr.set_ioc(true);
        }
        (false, false, true, true)
    } else if a == b {
        (false, true, true, false)
    } else if a < b {
        (true, false, false, false)
    } else {
        (false, false, true, false)
    };
    fpscr.set_nzcv(n, z, c, v);
}

/// Compare half-precision values, updating FPSCR flags.
pub fn vcmp_f16_bits(a: u16, b: u16, fpscr: &mut Fpscr) {
    vcmp_f16_bits_with_exception(a, b, false, fpscr);
}

/// Compare half-precision values, optionally signaling invalid operation for any NaN.
pub fn vcmp_f16_bits_with_exception(a: u16, b: u16, signal_all_nans: bool, fpscr: &mut Fpscr) {
    let a_nan = is_nan_f16_bits(a);
    let b_nan = is_nan_f16_bits(b);
    if a_nan || b_nan {
        if signal_all_nans || is_snan_f16_bits(a) || is_snan_f16_bits(b) {
            fpscr.set_ioc(true);
        }
        fpscr.set_nzcv(false, false, true, true);
        return;
    }

    vcmp_f32(vcvt_f32_f16_bits(a), vcvt_f32_f16_bits(b), fpscr);
}

/// Compare single-precision value with zero.
pub fn vcmpz_f32(a: f32, fpscr: &mut Fpscr) {
    vcmp_f32(a, 0.0, fpscr);
}

/// Compare double-precision value with zero.
pub fn vcmpz_f64(a: f64, fpscr: &mut Fpscr) {
    vcmp_f64(a, 0.0, fpscr);
}

fn is_nan_f16_bits(bits: u16) -> bool {
    (bits & 0x7C00) == 0x7C00 && (bits & 0x03FF) != 0
}

fn is_snan_f16_bits(bits: u16) -> bool {
    is_nan_f16_bits(bits) && (bits & 0x0200) == 0
}

fn is_snan_f32(value: f32) -> bool {
    let bits = value.to_bits();
    (bits & 0x7F80_0000) == 0x7F80_0000 && (bits & 0x007F_FFFF) != 0 && (bits & 0x0040_0000) == 0
}

fn is_snan_f64(value: f64) -> bool {
    let bits = value.to_bits();
    (bits & 0x7FF0_0000_0000_0000) == 0x7FF0_0000_0000_0000
        && (bits & 0x000F_FFFF_FFFF_FFFF) != 0
        && (bits & 0x0008_0000_0000_0000) == 0
}

/// Convert signed 32-bit integer to single-precision float.
pub fn vcvt_f32_s32(val: i32) -> f32 {
    val as f32
}

/// Convert unsigned 32-bit integer to single-precision float.
pub fn vcvt_f32_u32(val: u32) -> f32 {
    val as f32
}

/// Convert signed 32-bit fixed-point value to single-precision float.
pub fn vcvt_f32_s32_fixed(val: i32, fbits: u32) -> f32 {
    (val as f32) * 2.0f32.powi(-(fbits as i32))
}

/// Convert unsigned 32-bit fixed-point value to single-precision float.
pub fn vcvt_f32_u32_fixed(val: u32, fbits: u32) -> f32 {
    (val as f32) * 2.0f32.powi(-(fbits as i32))
}

/// Convert single-precision float to signed 32-bit integer (round toward zero).
pub fn vcvt_s32_f32(val: f32, fpscr: &mut Fpscr) -> i32 {
    saturate_f32_to_i32(val.trunc(), fpscr)
}

/// Convert single-precision float to unsigned 32-bit integer (round toward zero).
pub fn vcvt_u32_f32(val: f32, fpscr: &mut Fpscr) -> u32 {
    saturate_f32_to_u32(val.trunc(), fpscr)
}

/// Convert single-precision float to signed 32-bit fixed-point (round toward zero).
pub fn vcvt_s32_f32_fixed(val: f32, fbits: u32, fpscr: &mut Fpscr) -> i32 {
    let scaled = val * 2.0f32.powi(fbits as i32);
    saturate_f32_to_i32(scaled.trunc(), fpscr)
}

/// Convert single-precision float to unsigned 32-bit fixed-point (round toward zero).
pub fn vcvt_u32_f32_fixed(val: f32, fbits: u32, fpscr: &mut Fpscr) -> u32 {
    let scaled = val * 2.0f32.powi(fbits as i32);
    saturate_f32_to_u32(scaled.trunc(), fpscr)
}

/// Convert signed 32-bit integer to double-precision float.
pub fn vcvt_f64_s32(val: i32) -> f64 {
    val as f64
}

/// Convert unsigned 32-bit integer to double-precision float.
pub fn vcvt_f64_u32(val: u32) -> f64 {
    val as f64
}

/// Convert signed 32-bit fixed-point value to double-precision float.
pub fn vcvt_f64_s32_fixed(val: i32, fbits: u32) -> f64 {
    (val as f64) * 2.0f64.powi(-(fbits as i32))
}

/// Convert unsigned 32-bit fixed-point value to double-precision float.
pub fn vcvt_f64_u32_fixed(val: u32, fbits: u32) -> f64 {
    (val as f64) * 2.0f64.powi(-(fbits as i32))
}

/// Convert double-precision float to signed 32-bit integer (round toward zero).
pub fn vcvt_s32_f64(val: f64, fpscr: &mut Fpscr) -> i32 {
    saturate_f64_to_i32(val.trunc(), fpscr)
}

/// Convert double-precision float to unsigned 32-bit integer (round toward zero).
pub fn vcvt_u32_f64(val: f64, fpscr: &mut Fpscr) -> u32 {
    saturate_f64_to_u32(val.trunc(), fpscr)
}

/// Convert double-precision float to signed 32-bit fixed-point (round toward zero).
pub fn vcvt_s32_f64_fixed(val: f64, fbits: u32, fpscr: &mut Fpscr) -> i32 {
    let scaled = val * 2.0f64.powi(fbits as i32);
    saturate_f64_to_i32(scaled.trunc(), fpscr)
}

/// Convert double-precision float to unsigned 32-bit fixed-point (round toward zero).
pub fn vcvt_u32_f64_fixed(val: f64, fbits: u32, fpscr: &mut Fpscr) -> u32 {
    let scaled = val * 2.0f64.powi(fbits as i32);
    saturate_f64_to_u32(scaled.trunc(), fpscr)
}

/// Convert single-precision float to signed 32-bit integer using FPSCR rounding mode.
pub fn vcvtr_s32_f32(val: f32, fpscr: &mut Fpscr) -> i32 {
    let rounded = round_f32_for_int(val, fpscr.rmode());
    saturate_f32_to_i32(rounded, fpscr)
}

/// Convert single-precision float to signed 32-bit integer using an explicit rounding mode.
pub fn vcvt_s32_f32_round(val: f32, mode: RoundingMode, fpscr: &mut Fpscr) -> i32 {
    let rounded = round_f32_for_int(val, mode);
    saturate_f32_to_i32(rounded, fpscr)
}

/// Convert single-precision float to unsigned 32-bit integer using FPSCR rounding mode.
pub fn vcvtr_u32_f32(val: f32, fpscr: &mut Fpscr) -> u32 {
    let rounded = round_f32_for_int(val, fpscr.rmode());
    saturate_f32_to_u32(rounded, fpscr)
}

/// Convert single-precision float to unsigned 32-bit integer using an explicit rounding mode.
pub fn vcvt_u32_f32_round(val: f32, mode: RoundingMode, fpscr: &mut Fpscr) -> u32 {
    let rounded = round_f32_for_int(val, mode);
    saturate_f32_to_u32(rounded, fpscr)
}

/// Convert double-precision float to signed 32-bit integer using FPSCR rounding mode.
pub fn vcvtr_s32_f64(val: f64, fpscr: &mut Fpscr) -> i32 {
    let rounded = round_f64_for_int(val, fpscr.rmode());
    saturate_f64_to_i32(rounded, fpscr)
}

/// Convert double-precision float to signed 32-bit integer using an explicit rounding mode.
pub fn vcvt_s32_f64_round(val: f64, mode: RoundingMode, fpscr: &mut Fpscr) -> i32 {
    let rounded = round_f64_for_int(val, mode);
    saturate_f64_to_i32(rounded, fpscr)
}

/// Convert double-precision float to unsigned 32-bit integer using FPSCR rounding mode.
pub fn vcvtr_u32_f64(val: f64, fpscr: &mut Fpscr) -> u32 {
    let rounded = round_f64_for_int(val, fpscr.rmode());
    saturate_f64_to_u32(rounded, fpscr)
}

/// Convert double-precision float to unsigned 32-bit integer using an explicit rounding mode.
pub fn vcvt_u32_f64_round(val: f64, mode: RoundingMode, fpscr: &mut Fpscr) -> u32 {
    let rounded = round_f64_for_int(val, mode);
    saturate_f64_to_u32(rounded, fpscr)
}

/// Round single-precision to an integral floating-point value.
pub fn vrint_f32(val: f32, mode: RoundingMode, exact: bool, fpscr: &mut Fpscr) -> f32 {
    if val.is_nan() {
        fpscr.set_ioc(true);
        return val;
    }
    if val.is_infinite() || val == 0.0 {
        return val;
    }
    let rounded = round_f32_for_int(val, mode);
    if exact && rounded != val {
        fpscr.set_ixc(true);
    }
    rounded
}

/// Round half-precision bits to an integral half-precision floating-point value.
pub fn vrint_f16_bits(bits: u16, mode: RoundingMode, exact: bool, fpscr: &mut Fpscr) -> u16 {
    let value = vcvt_f32_f16_bits(bits);
    let rounded = vrint_f32(value, mode, exact, fpscr);
    vcvt_f16_bits_f32(rounded, fpscr)
}

/// Round double-precision to an integral floating-point value.
pub fn vrint_f64(val: f64, mode: RoundingMode, exact: bool, fpscr: &mut Fpscr) -> f64 {
    if val.is_nan() {
        fpscr.set_ioc(true);
        return val;
    }
    if val.is_infinite() || val == 0.0 {
        return val;
    }
    let rounded = round_f64_for_int(val, mode);
    if exact && rounded != val {
        fpscr.set_ixc(true);
    }
    rounded
}

/// Convert single-precision to double-precision.
pub fn vcvt_f64_f32(val: f32) -> f64 {
    val as f64
}

/// Convert double-precision to single-precision.
pub fn vcvt_f32_f64(val: f64, fpscr: &mut Fpscr) -> f32 {
    let result = val as f32;
    update_fpscr_after_op(result, fpscr);
    result
}

/// Convert IEEE-754 binary16 bits to single-precision.
pub fn vcvt_f32_f16_bits(bits: u16) -> f32 {
    let sign = ((bits as u32) & 0x8000) << 16;
    let exp = (bits >> 10) & 0x1F;
    let frac = (bits & 0x03FF) as u32;

    let out = match exp {
        0 if frac == 0 => sign,
        0 => {
            let value = (frac as f32) * 2.0f32.powi(-24);
            return if sign != 0 { -value } else { value };
        }
        0x1F => sign | 0x7F80_0000 | (frac << 13),
        _ => {
            let exp32 = ((exp as u32) + (127 - 15)) << 23;
            sign | exp32 | (frac << 13)
        }
    };
    f32::from_bits(out)
}

/// Convert single-precision to IEEE-754 binary16 bits.
pub fn vcvt_f16_bits_f32(val: f32, fpscr: &mut Fpscr) -> u16 {
    let bits = val.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let frac = bits & 0x7F_FFFF;

    if exp == 0xFF {
        if frac == 0 {
            return sign | 0x7C00;
        }
        fpscr.set_ioc(true);
        return sign | 0x7E00 | ((frac >> 13) as u16);
    }

    let exp16 = exp - 127 + 15;
    if exp16 >= 0x1F {
        fpscr.set_ofc(true);
        fpscr.set_ixc(true);
        return sign | 0x7C00;
    }

    if exp16 <= 0 {
        if exp16 < -10 {
            if frac != 0 || exp != 0 {
                fpscr.set_ufc(true);
                fpscr.set_ixc(true);
            }
            return sign;
        }
        let mant = frac | 0x80_0000;
        let rounded = round_shift_right_ties_even(mant, (14 - exp16) as u32);
        if rounded != 0 {
            fpscr.set_ufc(true);
        }
        return sign | rounded as u16;
    }

    let mut rounded_frac = round_shift_right_ties_even(frac, 13);
    let mut final_exp = exp16;
    if rounded_frac == 0x400 {
        rounded_frac = 0;
        final_exp += 1;
        if final_exp >= 0x1F {
            fpscr.set_ofc(true);
            fpscr.set_ixc(true);
            return sign | 0x7C00;
        }
    }
    sign | ((final_exp as u16) << 10) | (rounded_frac as u16)
}

/// Expand an 8-bit VFP immediate to a single-precision bit pattern.
pub fn vfp_expand_imm_f32(imm8: u8) -> u32 {
    let imm8 = imm8 as u32;
    let sign = (imm8 >> 7) & 1;
    let b6 = (imm8 >> 6) & 1;
    let exp = ((!b6 & 1) << 7) | ((if b6 != 0 { 0b11111 } else { 0 }) << 2) | ((imm8 >> 4) & 0x3);
    let mant = (imm8 & 0xF) << 19;
    (sign << 31) | (exp << 23) | mant
}

/// Expand an 8-bit VFP immediate to a half-precision bit pattern.
pub fn vfp_expand_imm_f16(imm8: u8) -> u16 {
    let imm8 = imm8 as u16;
    let sign = (imm8 >> 7) & 1;
    let b6 = (imm8 >> 6) & 1;
    let exp = ((!b6 & 1) << 4) | ((if b6 != 0 { 0b11 } else { 0 }) << 2) | ((imm8 >> 4) & 0x3);
    let mant = (imm8 & 0xF) << 6;
    (sign << 15) | (exp << 10) | mant
}

/// Expand an 8-bit VFP immediate to a double-precision bit pattern.
pub fn vfp_expand_imm_f64(imm8: u8) -> u64 {
    let imm8 = imm8 as u64;
    let sign = (imm8 >> 7) & 1;
    let b6 = (imm8 >> 6) & 1;
    let exp = ((!b6 & 1) << 10) | ((if b6 != 0 { 0xFF } else { 0 }) << 2) | ((imm8 >> 4) & 0x3);
    let mant = (imm8 & 0xF) << 48;
    (sign << 63) | (exp << 52) | mant
}

fn round_f32_for_int(val: f32, mode: RoundingMode) -> f32 {
    match mode {
        RoundingMode::RoundNearest => val.round_ties_even(),
        RoundingMode::RoundPlusInf => val.ceil(),
        RoundingMode::RoundMinusInf => val.floor(),
        RoundingMode::RoundZero => val.trunc(),
        RoundingMode::RoundTiesAway => val.round(),
    }
}

fn round_f64_for_int(val: f64, mode: RoundingMode) -> f64 {
    match mode {
        RoundingMode::RoundNearest => val.round_ties_even(),
        RoundingMode::RoundPlusInf => val.ceil(),
        RoundingMode::RoundMinusInf => val.floor(),
        RoundingMode::RoundZero => val.trunc(),
        RoundingMode::RoundTiesAway => val.round(),
    }
}

fn round_shift_right_ties_even(value: u32, shift: u32) -> u32 {
    if shift == 0 {
        return value;
    }
    let quotient = value >> shift;
    let remainder = value & ((1u32 << shift) - 1);
    let halfway = 1u32 << (shift - 1);
    if remainder > halfway || (remainder == halfway && (quotient & 1) != 0) {
        quotient + 1
    } else {
        quotient
    }
}

fn saturate_f32_to_i32(val: f32, fpscr: &mut Fpscr) -> i32 {
    if val.is_nan() {
        fpscr.set_ioc(true);
        0
    } else if val >= i32::MAX as f32 {
        fpscr.set_ioc(true);
        i32::MAX
    } else if val <= i32::MIN as f32 {
        fpscr.set_ioc(true);
        i32::MIN
    } else {
        val as i32
    }
}

fn saturate_f32_to_u32(val: f32, fpscr: &mut Fpscr) -> u32 {
    if val.is_nan() || val < 0.0 {
        fpscr.set_ioc(true);
        0
    } else if val >= u32::MAX as f32 {
        fpscr.set_ioc(true);
        u32::MAX
    } else {
        val as u32
    }
}

fn saturate_f64_to_i32(val: f64, fpscr: &mut Fpscr) -> i32 {
    if val.is_nan() {
        fpscr.set_ioc(true);
        0
    } else if val >= i32::MAX as f64 {
        fpscr.set_ioc(true);
        i32::MAX
    } else if val <= i32::MIN as f64 {
        fpscr.set_ioc(true);
        i32::MIN
    } else {
        val as i32
    }
}

fn saturate_f64_to_u32(val: f64, fpscr: &mut Fpscr) -> u32 {
    if val.is_nan() || val < 0.0 {
        fpscr.set_ioc(true);
        0
    } else if val >= u32::MAX as f64 {
        fpscr.set_ioc(true);
        u32::MAX
    } else {
        val as u32
    }
}

/// Update FPSCR exception flags after a single-precision operation.
fn update_fpscr_after_op(result: f32, fpscr: &mut Fpscr) {
    if result.is_nan() {
        fpscr.set_ioc(true);
    }
    if result.is_infinite() {
        fpscr.set_ofc(true);
    }
    // Note: Proper underflow/inexact detection requires more sophisticated
    // tracking than Rust's f32/f64 types provide. This is a simplified version.
}

/// Update FPSCR exception flags after a double-precision operation.
fn update_fpscr_after_op_f64(result: f64, fpscr: &mut Fpscr) {
    if result.is_nan() {
        fpscr.set_ioc(true);
    }
    if result.is_infinite() {
        fpscr.set_ofc(true);
    }
}

// =============================================================================
// NEON SIMD Operations
// =============================================================================

/// NEON vector element size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeonSize {
    /// 8-bit elements.
    B8,
    /// 16-bit elements.
    H16,
    /// 32-bit elements.
    S32,
    /// 64-bit elements.
    D64,
}

impl NeonSize {
    /// Number of elements in a 64-bit register.
    pub fn elements_per_d(&self) -> usize {
        match self {
            NeonSize::B8 => 8,
            NeonSize::H16 => 4,
            NeonSize::S32 => 2,
            NeonSize::D64 => 1,
        }
    }

    /// Size in bits.
    pub fn bits(&self) -> u32 {
        match self {
            NeonSize::B8 => 8,
            NeonSize::H16 => 16,
            NeonSize::S32 => 32,
            NeonSize::D64 => 64,
        }
    }
}

/// NEON integer add (element-wise).
pub fn vadd_i(a: u64, b: u64, size: NeonSize) -> u64 {
    match size {
        NeonSize::B8 => {
            let mut result = 0u64;
            for i in 0..8 {
                let va = ((a >> (i * 8)) & 0xFF) as u8;
                let vb = ((b >> (i * 8)) & 0xFF) as u8;
                result |= (va.wrapping_add(vb) as u64) << (i * 8);
            }
            result
        }
        NeonSize::H16 => {
            let mut result = 0u64;
            for i in 0..4 {
                let va = ((a >> (i * 16)) & 0xFFFF) as u16;
                let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
                result |= (va.wrapping_add(vb) as u64) << (i * 16);
            }
            result
        }
        NeonSize::S32 => {
            let mut result = 0u64;
            for i in 0..2 {
                let va = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
                let vb = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32;
                result |= (va.wrapping_add(vb) as u64) << (i * 32);
            }
            result
        }
        NeonSize::D64 => a.wrapping_add(b),
    }
}

/// NEON integer subtract (element-wise).
pub fn vsub_i(a: u64, b: u64, size: NeonSize) -> u64 {
    match size {
        NeonSize::B8 => {
            let mut result = 0u64;
            for i in 0..8 {
                let va = ((a >> (i * 8)) & 0xFF) as u8;
                let vb = ((b >> (i * 8)) & 0xFF) as u8;
                result |= (va.wrapping_sub(vb) as u64) << (i * 8);
            }
            result
        }
        NeonSize::H16 => {
            let mut result = 0u64;
            for i in 0..4 {
                let va = ((a >> (i * 16)) & 0xFFFF) as u16;
                let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
                result |= (va.wrapping_sub(vb) as u64) << (i * 16);
            }
            result
        }
        NeonSize::S32 => {
            let mut result = 0u64;
            for i in 0..2 {
                let va = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
                let vb = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32;
                result |= (va.wrapping_sub(vb) as u64) << (i * 32);
            }
            result
        }
        NeonSize::D64 => a.wrapping_sub(b),
    }
}

/// NEON bitwise AND.
pub fn vand(a: u64, b: u64) -> u64 {
    a & b
}

/// NEON bitwise OR.
pub fn vorr(a: u64, b: u64) -> u64 {
    a | b
}

/// NEON bitwise XOR.
pub fn veor(a: u64, b: u64) -> u64 {
    a ^ b
}

/// NEON bitwise AND NOT (BIC).
pub fn vbic(a: u64, b: u64) -> u64 {
    a & !b
}

/// NEON bitwise OR NOT (ORN).
pub fn vorn(a: u64, b: u64) -> u64 {
    a | !b
}

/// NEON bitwise NOT (MVN).
pub fn vmvn(a: u64) -> u64 {
    !a
}

fn count_leading_sign_bits(value: u64, bits: u32) -> u64 {
    let sign_bit = 1u64 << (bits - 1);
    let mask = if bits == 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    };
    let value = value & mask;
    let inverted = if (value & sign_bit) == 0 {
        value
    } else {
        (!value) & mask
    };
    if inverted == 0 {
        return (bits - 1) as u64;
    }
    let leading = if bits == 64 {
        inverted.leading_zeros()
    } else {
        (inverted << (64 - bits)).leading_zeros()
    };
    leading.saturating_sub(1) as u64
}

/// NEON count leading sign bits (element-wise).
pub fn vcls_i(a: u64, size: NeonSize) -> u64 {
    match size {
        NeonSize::B8 => {
            let mut result = 0u64;
            for i in 0..8 {
                let value = (a >> (i * 8)) & 0xFF;
                result |= count_leading_sign_bits(value, 8) << (i * 8);
            }
            result
        }
        NeonSize::H16 => {
            let mut result = 0u64;
            for i in 0..4 {
                let value = (a >> (i * 16)) & 0xFFFF;
                result |= count_leading_sign_bits(value, 16) << (i * 16);
            }
            result
        }
        NeonSize::S32 => {
            let mut result = 0u64;
            for i in 0..2 {
                let value = (a >> (i * 32)) & 0xFFFF_FFFF;
                result |= count_leading_sign_bits(value, 32) << (i * 32);
            }
            result
        }
        NeonSize::D64 => count_leading_sign_bits(a, 64),
    }
}

/// NEON count leading zeros (element-wise).
pub fn vclz_i(a: u64, size: NeonSize) -> u64 {
    match size {
        NeonSize::B8 => {
            let mut result = 0u64;
            for i in 0..8 {
                let value = ((a >> (i * 8)) & 0xFF) as u8;
                result |= (value.leading_zeros() as u64) << (i * 8);
            }
            result
        }
        NeonSize::H16 => {
            let mut result = 0u64;
            for i in 0..4 {
                let value = ((a >> (i * 16)) & 0xFFFF) as u16;
                result |= (value.leading_zeros() as u64) << (i * 16);
            }
            result
        }
        NeonSize::S32 => {
            let mut result = 0u64;
            for i in 0..2 {
                let value = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
                result |= (value.leading_zeros() as u64) << (i * 32);
            }
            result
        }
        NeonSize::D64 => a.leading_zeros() as u64,
    }
}

/// NEON population count (8-bit element-wise).
pub fn vcnt_i8(a: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let value = ((a >> (i * 8)) & 0xFF) as u8;
        result |= (value.count_ones() as u64) << (i * 8);
    }
    result
}

/// NEON reverse elements within 16/32/64-bit containers.
pub fn vrev(a: u64, size: NeonSize, container_bits: u32) -> u64 {
    let esize = size.bits();
    debug_assert!(container_bits == 16 || container_bits == 32 || container_bits == 64);
    debug_assert!(container_bits % esize == 0);

    let elements_per_container = container_bits / esize;
    let containers = 64 / container_bits;
    let element_mask = if esize == 64 {
        u64::MAX
    } else {
        (1u64 << esize) - 1
    };
    let mut result = 0u64;

    for container in 0..containers {
        let base = container * elements_per_container;
        for element in 0..elements_per_container {
            let src_index = base + element;
            let dst_index = base + (elements_per_container - 1 - element);
            let value = (a >> (src_index * esize)) & element_mask;
            result |= value << (dst_index * esize);
        }
    }

    result
}

/// NEON duplicate scalar to all elements.
pub fn vdup(scalar: u64, size: NeonSize) -> u64 {
    match size {
        NeonSize::B8 => {
            let val = scalar as u8;
            u64::from_ne_bytes([val; 8])
        }
        NeonSize::H16 => {
            let val = scalar as u16;
            let bytes = val.to_ne_bytes();
            let mut result = [0u8; 8];
            for i in 0..4 {
                result[i * 2] = bytes[0];
                result[i * 2 + 1] = bytes[1];
            }
            u64::from_ne_bytes(result)
        }
        NeonSize::S32 => {
            let val = scalar as u32;
            ((val as u64) << 32) | (val as u64)
        }
        NeonSize::D64 => scalar,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfp_state_s_registers() {
        let mut vfp = VfpState::new();

        // Write to S0 (low half of D0)
        vfp.write_s(0, 1.5);
        assert_eq!(vfp.read_s(0), 1.5);

        // Write to S1 (high half of D0)
        vfp.write_s(1, 2.5);
        assert_eq!(vfp.read_s(1), 2.5);

        // S0 should be unchanged
        assert_eq!(vfp.read_s(0), 1.5);
    }

    #[test]
    fn test_vfp_state_d_registers() {
        let mut vfp = VfpState::new();

        vfp.write_d(0, 3.14159);
        assert_eq!(vfp.read_d(0), 3.14159);

        vfp.write_d(15, -2.71828);
        assert_eq!(vfp.read_d(15), -2.71828);
    }

    #[test]
    fn test_vfp_state_q_registers() {
        let mut vfp = VfpState::new();

        vfp.write_q(0, 0x1234_5678_9ABC_DEF0, 0xFEDC_BA98_7654_3210);
        let (low, high) = vfp.read_q(0);
        assert_eq!(low, 0x1234_5678_9ABC_DEF0);
        assert_eq!(high, 0xFEDC_BA98_7654_3210);
    }

    #[test]
    fn test_fpscr_flags() {
        let mut fpscr = Fpscr::default();

        fpscr.set_nzcv(true, false, true, false);
        assert!(fpscr.n());
        assert!(!fpscr.z());
        assert!(fpscr.c());
        assert!(!fpscr.v());

        fpscr.set_nzcv(false, true, false, true);
        assert!(!fpscr.n());
        assert!(fpscr.z());
        assert!(!fpscr.c());
        assert!(fpscr.v());
    }

    #[test]
    fn test_fpscr_rounding_mode() {
        let mut fpscr = Fpscr::default();

        fpscr.set_rmode(RoundingMode::RoundZero);
        assert_eq!(fpscr.rmode(), RoundingMode::RoundZero);

        fpscr.set_rmode(RoundingMode::RoundPlusInf);
        assert_eq!(fpscr.rmode(), RoundingMode::RoundPlusInf);
    }

    #[test]
    fn test_vadd_f32() {
        let mut fpscr = Fpscr::default();
        assert_eq!(vadd_f32(1.0, 2.0, &mut fpscr), 3.0);
        assert_eq!(vadd_f32(-1.5, 2.5, &mut fpscr), 1.0);
    }

    #[test]
    fn test_vsub_f32() {
        let mut fpscr = Fpscr::default();
        assert_eq!(vsub_f32(5.0, 3.0, &mut fpscr), 2.0);
        assert_eq!(vsub_f32(1.0, 2.0, &mut fpscr), -1.0);
    }

    #[test]
    fn test_vmul_f32() {
        let mut fpscr = Fpscr::default();
        assert_eq!(vmul_f32(3.0, 4.0, &mut fpscr), 12.0);
        assert_eq!(vmul_f32(-2.0, 3.0, &mut fpscr), -6.0);
    }

    #[test]
    fn test_vdiv_f32() {
        let mut fpscr = Fpscr::default();
        assert_eq!(vdiv_f32(10.0, 2.0, &mut fpscr), 5.0);
        assert!(!fpscr.dzc());

        // Division by zero
        let _ = vdiv_f32(1.0, 0.0, &mut fpscr);
        assert!(fpscr.dzc());
    }

    #[test]
    fn test_vcmp_f32() {
        let mut fpscr = Fpscr::default();

        // Equal
        vcmp_f32(1.0, 1.0, &mut fpscr);
        assert!(fpscr.z());
        assert!(fpscr.c());
        assert!(!fpscr.n());
        assert!(!fpscr.v());

        // Less than
        vcmp_f32(1.0, 2.0, &mut fpscr);
        assert!(!fpscr.z());
        assert!(!fpscr.c());
        assert!(fpscr.n());
        assert!(!fpscr.v());

        // Greater than
        vcmp_f32(3.0, 2.0, &mut fpscr);
        assert!(!fpscr.z());
        assert!(fpscr.c());
        assert!(!fpscr.n());
        assert!(!fpscr.v());

        // NaN
        vcmp_f32(f32::NAN, 1.0, &mut fpscr);
        assert!(!fpscr.z());
        assert!(fpscr.c());
        assert!(!fpscr.n());
        assert!(fpscr.v());
    }

    #[test]
    fn test_vcvt_f32_s32() {
        assert_eq!(vcvt_f32_s32(42), 42.0);
        assert_eq!(vcvt_f32_s32(-100), -100.0);
    }

    #[test]
    fn test_vcvt_s32_f32() {
        let mut fpscr = Fpscr::default();
        assert_eq!(vcvt_s32_f32(42.7, &mut fpscr), 42);
        assert_eq!(vcvt_s32_f32(-3.9, &mut fpscr), -3);
    }

    #[test]
    fn test_neon_vadd_i() {
        // 8-bit elements
        let a = 0x0102030405060708u64;
        let b = 0x0101010101010101u64;
        let result = vadd_i(a, b, NeonSize::B8);
        assert_eq!(result, 0x0203040506070809u64);

        // 32-bit elements
        let a = 0x0000000100000002u64;
        let b = 0x0000000300000004u64;
        let result = vadd_i(a, b, NeonSize::S32);
        assert_eq!(result, 0x0000000400000006u64);
    }

    #[test]
    fn test_neon_bitwise() {
        let a = 0xFFFF_0000_FFFF_0000u64;
        let b = 0xFF00_FF00_FF00_FF00u64;

        assert_eq!(vand(a, b), 0xFF00_0000_FF00_0000u64);
        assert_eq!(vorr(a, b), 0xFFFF_FF00_FFFF_FF00u64);
        assert_eq!(veor(a, b), 0x00FF_FF00_00FF_FF00u64);
        assert_eq!(vbic(a, b), 0x00FF_0000_00FF_0000u64);
    }

    #[test]
    fn test_vdup() {
        // Duplicate byte
        let result = vdup(0x42, NeonSize::B8);
        assert_eq!(result, 0x4242_4242_4242_4242u64);

        // Duplicate halfword
        let result = vdup(0x1234, NeonSize::H16);
        assert_eq!(result, 0x1234_1234_1234_1234u64);

        // Duplicate word
        let result = vdup(0xDEAD_BEEF, NeonSize::S32);
        assert_eq!(result, 0xDEAD_BEEF_DEAD_BEEFu64);
    }
}
