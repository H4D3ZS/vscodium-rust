//! IEEE-754 floating-point support for the F and D extensions.
//!
//! The arithmetic uses the host FPU for the correctly-rounded round-to-nearest
//! result and then, for the directed rounding modes, corrects it using the
//! *exact residual* `delta = true_result - rne_result`. For `+`, `-`, `*`,
//! `/` and `sqrt` the residual is itself representable and is recovered with a
//! fused multiply-add (or 2Sum), giving correctly-rounded results in every RISC-V
//! rounding mode without depending on the host rounding mode. Accrued exception
//! flags (`fflags`) are computed alongside each operation.
//!
//! All results follow RISC-V conventions: any NaN result is the canonical quiet
//! NaN, signaling-NaN inputs and invalid operations raise NV, and single-
//! precision values are NaN-boxed by the caller in [`crate::riscv::cpu`].

/// RISC-V floating-point rounding modes (`frm` field, also the instruction
/// `rm` field; `Dyn` selects the CSR `frm`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundingMode {
    /// Round to nearest, ties to even.
    Rne,
    /// Round towards zero.
    Rtz,
    /// Round down (towards -inf).
    Rdn,
    /// Round up (towards +inf).
    Rup,
    /// Round to nearest, ties to max magnitude.
    Rmm,
    /// Use the dynamic rounding mode from `fcsr.frm`.
    Dyn,
}

impl RoundingMode {
    /// Decode a 3-bit rounding-mode field.
    pub fn from_bits(bits: u8) -> Option<RoundingMode> {
        Some(match bits & 0x7 {
            0 => RoundingMode::Rne,
            1 => RoundingMode::Rtz,
            2 => RoundingMode::Rdn,
            3 => RoundingMode::Rup,
            4 => RoundingMode::Rmm,
            7 => RoundingMode::Dyn,
            _ => return None, // 5,6 reserved
        })
    }
}

/// Accrued floating-point exception flag bits (`fcsr.fflags`).
pub mod fflags {
    /// Inexact.
    pub const NX: u32 = 0x01;
    /// Underflow.
    pub const UF: u32 = 0x02;
    /// Overflow.
    pub const OF: u32 = 0x04;
    /// Divide by zero.
    pub const DZ: u32 = 0x08;
    /// Invalid operation.
    pub const NV: u32 = 0x10;
}

/// Canonical quiet NaN bit pattern for single precision.
pub const CANONICAL_NAN_F32: u32 = 0x7fc0_0000;
/// Canonical quiet NaN for double precision.
pub const CANONICAL_NAN_F64: u64 = 0x7ff8_0000_0000_0000;

// ===========================================================================
// Float abstraction shared by f32 and f64.
// ===========================================================================

/// Operations needed by the generic arithmetic core, implemented for `f32`
/// and `f64`.
pub trait Sf: Copy + PartialOrd {
    /// Underlying bit representation (`u32` / `u64`).
    type Bits: Copy + Eq;

    const CANON_NAN: Self::Bits;
    const ZERO: Self;
    const PINF: Self;
    const NINF: Self;
    /// Mantissa width (including the implicit bit): 24 / 53.
    const MANT_BITS: u32;
    /// Width in bits: 32 / 64.
    const WIDTH: u32;

    fn from_bits(b: Self::Bits) -> Self;
    fn to_bits(self) -> Self::Bits;
    fn add(self, o: Self) -> Self;
    fn sub(self, o: Self) -> Self;
    fn mul(self, o: Self) -> Self;
    fn div(self, o: Self) -> Self;
    fn sqrt(self) -> Self;
    fn mul_add(self, b: Self, c: Self) -> Self;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn abs(self) -> Self;
    fn neg(self) -> Self;
    fn is_zero(self) -> bool;
    /// Largest finite magnitude.
    fn max_finite() -> Self;
    /// Smallest positive normal.
    fn min_normal() -> Self;
    fn signaling_nan(self) -> bool;
    /// Convert an integer to this float, rounding to nearest-even.
    fn from_i128(v: i128) -> Self;
    /// Truncate an integral float to an integer (saturating).
    fn to_i128(self) -> i128;
    /// Round to an integral value per `mode` (the value stays in this format).
    fn round_integral(self, mode: RoundingMode) -> Self;
    fn canon() -> Self {
        Self::from_bits(Self::CANON_NAN)
    }
}

macro_rules! impl_sf {
    ($t:ty, $b:ty, $canon:expr, $mant:expr, $width:expr, $maxf:expr, $minn:expr) => {
        impl Sf for $t {
            type Bits = $b;
            const CANON_NAN: $b = $canon;
            const ZERO: $t = 0.0;
            const PINF: $t = <$t>::INFINITY;
            const NINF: $t = <$t>::NEG_INFINITY;
            const MANT_BITS: u32 = $mant;
            const WIDTH: u32 = $width;
            #[inline]
            fn from_bits(b: $b) -> $t {
                <$t>::from_bits(b)
            }
            #[inline]
            fn to_bits(self) -> $b {
                <$t>::to_bits(self)
            }
            #[inline]
            fn add(self, o: $t) -> $t {
                self + o
            }
            #[inline]
            fn sub(self, o: $t) -> $t {
                self - o
            }
            #[inline]
            fn mul(self, o: $t) -> $t {
                self * o
            }
            #[inline]
            fn div(self, o: $t) -> $t {
                self / o
            }
            #[inline]
            fn sqrt(self) -> $t {
                <$t>::sqrt(self)
            }
            #[inline]
            fn mul_add(self, b: $t, c: $t) -> $t {
                <$t>::mul_add(self, b, c)
            }
            #[inline]
            fn is_nan(self) -> bool {
                <$t>::is_nan(self)
            }
            #[inline]
            fn is_infinite(self) -> bool {
                <$t>::is_infinite(self)
            }
            #[inline]
            fn is_finite(self) -> bool {
                <$t>::is_finite(self)
            }
            #[inline]
            fn is_sign_negative(self) -> bool {
                <$t>::is_sign_negative(self)
            }
            #[inline]
            fn abs(self) -> $t {
                <$t>::abs(self)
            }
            #[inline]
            fn neg(self) -> $t {
                -self
            }
            #[inline]
            fn is_zero(self) -> bool {
                self == 0.0
            }
            #[inline]
            fn max_finite() -> $t {
                $maxf
            }
            #[inline]
            fn min_normal() -> $t {
                $minn
            }
            #[inline]
            fn signaling_nan(self) -> bool {
                // NaN with the quiet bit (MSB of mantissa) clear.
                if !self.is_nan() {
                    return false;
                }
                let qbit: $b = 1 << ($mant - 2);
                (self.to_bits() & qbit) == 0
            }
            #[inline]
            fn from_i128(v: i128) -> $t {
                v as $t
            }
            #[inline]
            fn to_i128(self) -> i128 {
                self as i128
            }
            #[inline]
            fn round_integral(self, mode: RoundingMode) -> $t {
                match mode {
                    RoundingMode::Rtz => self.trunc(),
                    RoundingMode::Rdn => self.floor(),
                    RoundingMode::Rup => self.ceil(),
                    RoundingMode::Rmm => self.round(),
                    _ => self.round_ties_even(),
                }
            }
        }
    };
}

impl_sf!(f32, u32, 0x7fc0_0000, 24, 32, f32::MAX, f32::MIN_POSITIVE);
impl_sf!(
    f64,
    u64,
    0x7ff8_0000_0000_0000,
    53,
    64,
    f64::MAX,
    f64::MIN_POSITIVE
);

// ---------------------------------------------------------------------------
// ULP neighbours (bit-level next-representable).
// ---------------------------------------------------------------------------

fn next_up<F: Sf>(x: F) -> F {
    if x.is_nan() || (x.is_infinite() && !x.is_sign_negative()) {
        return x;
    }
    if x.is_zero() {
        // smallest positive subnormal
        let mut b = F::ZERO.to_bits();
        b = inc_bits::<F>(b);
        return F::from_bits(b);
    }
    let b = x.to_bits();
    let nb = if x.is_sign_negative() {
        dec_bits::<F>(b)
    } else {
        inc_bits::<F>(b)
    };
    F::from_bits(nb)
}

fn next_down<F: Sf>(x: F) -> F {
    next_up(x.neg()).neg()
}

#[inline]
fn inc_bits<F: Sf>(b: F::Bits) -> F::Bits {
    // Generic +1 on the bit pattern via u64 round-trip is not possible without
    // numeric ops on the associated type; specialize through the width.
    add_bits::<F>(b, 1)
}
#[inline]
fn dec_bits<F: Sf>(b: F::Bits) -> F::Bits {
    add_bits::<F>(b, u64::MAX) // -1
}

/// Add `delta` (mod 2^WIDTH) to a raw bit pattern.
fn add_bits<F: Sf>(b: F::Bits, delta: u64) -> F::Bits {
    // Round-trip through u64 using the float to recover an integer is awkward;
    // instead reinterpret via to/from on the concrete widths.
    if F::WIDTH == 32 {
        let v = bits_to_u64::<F>(b) as u32;
        u64_to_bits::<F>(v.wrapping_add(delta as u32) as u64)
    } else {
        let v = bits_to_u64::<F>(b);
        u64_to_bits::<F>(v.wrapping_add(delta))
    }
}

// Helpers to move between the associated Bits type and u64 generically.
fn bits_to_u64<F: Sf>(b: F::Bits) -> u64 {
    // SAFETY: Bits is u32 or u64 (Copy, no padding); read its bytes.
    let mut out: u64 = 0;
    let n = (F::WIDTH / 8) as usize;
    unsafe {
        std::ptr::copy_nonoverlapping(
            &b as *const F::Bits as *const u8,
            &mut out as *mut u64 as *mut u8,
            n,
        );
    }
    out
}
fn u64_to_bits<F: Sf>(v: u64) -> F::Bits {
    let mut out: F::Bits = F::CANON_NAN;
    let n = (F::WIDTH / 8) as usize;
    unsafe {
        std::ptr::copy_nonoverlapping(
            &v as *const u64 as *const u8,
            &mut out as *mut F::Bits as *mut u8,
            n,
        );
    }
    out
}

// ---------------------------------------------------------------------------
// Directed-rounding corrector.
// ---------------------------------------------------------------------------

/// Given the round-to-nearest result `rne` and the exact residual
/// `delta = true - rne`, produce the value for `mode`, setting NX/OF/UF.
/// `inputs_finite` indicates whether both operands were finite (for overflow).
fn correct<F: Sf>(rne: F, delta: F, mode: RoundingMode, inputs_finite: bool, flags: &mut u32) -> F {
    // Overflow: native produced infinity from finite inputs.
    if rne.is_infinite() && inputs_finite {
        *flags |= fflags::OF | fflags::NX;
        let neg = rne.is_sign_negative();
        return match mode {
            RoundingMode::Rtz => {
                if neg {
                    F::max_finite().neg()
                } else {
                    F::max_finite()
                }
            }
            RoundingMode::Rdn => {
                if neg {
                    rne
                } else {
                    F::max_finite()
                }
            }
            RoundingMode::Rup => {
                if neg {
                    F::max_finite().neg()
                } else {
                    rne
                }
            }
            _ => rne, // Rne / Rmm -> inf
        };
    }

    if delta.is_zero() {
        return rne; // exact
    }
    *flags |= fflags::NX;

    let up = delta > F::ZERO; // true result above rne
    let (lo, hi) = if up {
        (rne, next_up(rne))
    } else {
        (next_down(rne), rne)
    };

    let result = match mode {
        RoundingMode::Rne | RoundingMode::Dyn => rne,
        RoundingMode::Rtz => min_mag(lo, hi),
        RoundingMode::Rdn => lo,
        RoundingMode::Rup => hi,
        RoundingMode::Rmm => {
            // Tie iff 2*|delta| == (hi - lo).
            let two_d = delta.abs().add(delta.abs());
            let ulp = hi.sub(lo);
            if two_d == ulp { max_mag(lo, hi) } else { rne }
        }
    };

    // Overflow produced by pushing to infinity in a directed mode.
    if result.is_infinite() && inputs_finite {
        *flags |= fflags::OF;
    }
    // Underflow: tiny (subnormal) and inexact.
    if result.is_finite() && !result.is_zero() && result.abs() < F::min_normal() {
        *flags |= fflags::UF;
    }
    result
}

fn min_mag<F: Sf>(a: F, b: F) -> F {
    if a.abs() <= b.abs() { a } else { b }
}
fn max_mag<F: Sf>(a: F, b: F) -> F {
    if a.abs() >= b.abs() { a } else { b }
}

// ---------------------------------------------------------------------------
// Arithmetic operations (return canonical-NaN bits on NaN results).
// ---------------------------------------------------------------------------

fn nan_result<F: Sf>(flags: &mut u32, sig: bool) -> F {
    if sig {
        *flags |= fflags::NV;
    }
    F::canon()
}

/// Was any operand a signaling NaN?
fn any_snan<F: Sf>(ops: &[F]) -> bool {
    ops.iter().any(|o| o.signaling_nan())
}

/// `a + b`.
pub fn add<F: Sf>(a: F, b: F, mode: RoundingMode, flags: &mut u32) -> F {
    if a.is_nan() || b.is_nan() {
        return nan_result::<F>(flags, any_snan(&[a, b]));
    }
    // inf + (-inf) is invalid.
    if a.is_infinite() && b.is_infinite() && (a.is_sign_negative() != b.is_sign_negative()) {
        *flags |= fflags::NV;
        return F::canon();
    }
    if a.is_infinite() || b.is_infinite() {
        return a.add(b);
    }
    let r = a.add(b);
    if r.is_infinite() {
        return correct(r, F::ZERO, mode, true, flags);
    }
    // Sign of an exact-zero sum: +0 in every mode except round-down, where it is
    // -0 -- unless both addends are +0 (a zero with that same sign is retained).
    if r.is_zero() && !r.is_sign_negative() && mode == RoundingMode::Rdn {
        let both_pos_zero =
            a.is_zero() && !a.is_sign_negative() && b.is_zero() && !b.is_sign_negative();
        if !both_pos_zero {
            return F::ZERO.neg();
        }
    }
    // 2Sum exact residual: a + b == r + e.
    let bv = r.sub(a);
    let e = a.sub(r.sub(bv)).add(b.sub(bv));
    correct(r, e, mode, true, flags)
}

/// `a - b`.
pub fn sub<F: Sf>(a: F, b: F, mode: RoundingMode, flags: &mut u32) -> F {
    add(a, b.neg(), mode, flags)
}

// Multiply, divide, square-root and fused-multiply-add are provided by the
// integer-significand soft-float (`sf_mul`/`sf_div`/`sf_sqrt`/`sf_fma`) below,
// which is correctly rounded in every mode including for subnormal results.

// ---------------------------------------------------------------------------
// Min / Max, comparisons, classify, sign-injection.
// ---------------------------------------------------------------------------

/// RISC-V `fmin` (returns the smaller; NaN handling: quiet NaN ignored, sNaN
/// raises NV; both NaN -> canonical; -0 < +0).
pub fn fmin<F: Sf>(a: F, b: F, flags: &mut u32) -> F {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() && b.is_nan() {
        return F::canon();
    }
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a.is_zero() && b.is_zero() {
        // -0 is less than +0
        return if a.is_sign_negative() { a } else { b };
    }
    if a < b { a } else { b }
}

/// RISC-V `fmax`.
pub fn fmax<F: Sf>(a: F, b: F, flags: &mut u32) -> F {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() && b.is_nan() {
        return F::canon();
    }
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a.is_zero() && b.is_zero() {
        return if a.is_sign_negative() { b } else { a };
    }
    if a > b { a } else { b }
}

/// `feq` (quiet: only sNaN raises NV).
pub fn feq<F: Sf>(a: F, b: F, flags: &mut u32) -> bool {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() || b.is_nan() {
        return false;
    }
    a == b
}

/// `flt` (signaling: any NaN raises NV).
pub fn flt<F: Sf>(a: F, b: F, flags: &mut u32) -> bool {
    if a.is_nan() || b.is_nan() {
        *flags |= fflags::NV;
        return false;
    }
    a < b
}

/// `fle` (signaling: any NaN raises NV).
pub fn fle<F: Sf>(a: F, b: F, flags: &mut u32) -> bool {
    if a.is_nan() || b.is_nan() {
        *flags |= fflags::NV;
        return false;
    }
    a <= b
}

/// `fclass` 10-bit classification mask.
pub fn fclass<F: Sf>(x: F) -> u64 {
    let neg = x.is_sign_negative();
    if x.is_nan() {
        return if x.signaling_nan() { 1 << 8 } else { 1 << 9 };
    }
    if x.is_infinite() {
        return if neg { 1 << 0 } else { 1 << 7 };
    }
    if x.is_zero() {
        return if neg { 1 << 3 } else { 1 << 4 };
    }
    let subnormal = x.abs() < F::min_normal();
    if subnormal {
        if neg { 1 << 2 } else { 1 << 5 }
    } else if neg {
        1 << 1
    } else {
        1 << 6
    }
}

// ---------------------------------------------------------------------------
// Zfa: NaN-propagating min/max, round-to-integral, quiet compares, fli, fcvtmod.
// ---------------------------------------------------------------------------

/// Zfa `fminm`: like `fmin` but a NaN operand yields the canonical NaN.
pub fn fminm<F: Sf>(a: F, b: F, flags: &mut u32) -> F {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() || b.is_nan() {
        return F::canon();
    }
    if a.is_zero() && b.is_zero() {
        return if a.is_sign_negative() { a } else { b };
    }
    if a < b { a } else { b }
}

/// Zfa `fmaxm`: like `fmax` but a NaN operand yields the canonical NaN.
pub fn fmaxm<F: Sf>(a: F, b: F, flags: &mut u32) -> F {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() || b.is_nan() {
        return F::canon();
    }
    if a.is_zero() && b.is_zero() {
        return if a.is_sign_negative() { b } else { a };
    }
    if a > b { a } else { b }
}

/// Zfa `fleq` — quiet `<=` (only a signaling NaN raises NV).
pub fn fleq<F: Sf>(a: F, b: F, flags: &mut u32) -> bool {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() || b.is_nan() {
        return false;
    }
    a <= b
}

/// Zfa `fltq` — quiet `<` (only a signaling NaN raises NV).
pub fn fltq<F: Sf>(a: F, b: F, flags: &mut u32) -> bool {
    if a.signaling_nan() || b.signaling_nan() {
        *flags |= fflags::NV;
    }
    if a.is_nan() || b.is_nan() {
        return false;
    }
    a < b
}

/// Zfa `fround`/`froundnx` — round to an integral value in the given mode.
/// `set_nx` (froundnx) raises NX when the result differs from the input.
pub fn fround<F: Sf>(x: F, mode: RoundingMode, set_nx: bool, flags: &mut u32) -> F {
    if x.is_nan() {
        if x.signaling_nan() {
            *flags |= fflags::NV;
        }
        return F::canon();
    }
    if x.is_infinite() || x.is_zero() {
        return x;
    }
    let r = x.round_integral(mode);
    if set_nx && r.to_bits() != x.to_bits() {
        *flags |= fflags::NX;
    }
    r
}

/// Zfa `fli` constant for table index `idx` (0..31) in format `fmt`.
pub fn fli(fmt: Fmt, idx: u8) -> u64 {
    // f64 values for indices 2..29; 0/1/30/31 are handled specially.
    const TBL: [f64; 32] = [
        -1.0,
        0.0, // 0: -1.0, 1: min normal (special)
        0.0000152587890625,
        0.000030517578125,
        0.00390625,
        0.0078125,
        0.0625,
        0.125, // 2^-16..2^-3
        0.25,
        0.3125,
        0.375,
        0.4375,
        0.5,
        0.625,
        0.75,
        0.875, // 8..15
        1.0,
        1.25,
        1.5,
        1.75,
        2.0,
        2.5,
        3.0,
        4.0, // 16..23
        8.0,
        16.0,
        128.0,
        256.0,
        32768.0,
        65536.0, // 24..29 (incl 2^15, 2^16)
        0.0,
        0.0, // 30: +inf, 31: qNaN (special)
    ];
    match idx & 31 {
        1 => fmt.pack(false, 1, 0), // minimum positive normal
        30 => fmt.pack_inf(false),  // +inf
        31 => fmt.canon(),          // canonical qNaN
        i => {
            let v = TBL[i as usize];
            match fmt.p {
                p if p == F64.p => v.to_bits(),
                p if p == F32.p => (v as f32).to_bits() as u64,
                _ => fcvt_round(F64, fmt, v.to_bits(), RoundingMode::Rne, &mut 0u32),
            }
        }
    }
}

/// Zfa `fcvtmod.w.d` — truncate a double toward zero, take the result modulo
/// 2^32 as a signed 32-bit value, sign-extended. NaN/inf -> 0 with NV; a dropped
/// fraction raises NX.
pub fn fcvtmod_w_d(x: f64, flags: &mut u32) -> u64 {
    if x.is_nan() || x.is_infinite() {
        *flags |= fflags::NV;
        return 0;
    }
    match decompose(F64, x.to_bits()) {
        Dec::Zero(_) => 0,
        Dec::Finite { sign, mant, exp } => {
            // Integer value V = mant * 2^exp; report V mod 2^32 sign-extended.
            // The fraction is inexact when any bit below 2^0 is dropped.
            let inexact = exp < 0 && (mant & ((1u64 << (-exp).min(63)) - 1)) != 0;
            // Invalid when the truncated value does not fit a signed 32-bit int.
            let msb_exp = (63 - (mant as u64).leading_zeros()) as i32 + exp;
            let out_of_range = if msb_exp >= 32 {
                true
            } else {
                let m = mant as i128;
                let v = if exp >= 0 {
                    m << exp
                } else {
                    m >> ((-exp) as u32).min(127)
                };
                let v = if sign { -v } else { v };
                !(v >= -(1i128 << 31) && v < (1i128 << 31))
            };
            // Invalid suppresses inexact (as with the saturating FCVT forms).
            if out_of_range {
                *flags |= fflags::NV;
            } else if inexact {
                *flags |= fflags::NX;
            }
            let m = mant as u128;
            let vmod: u32 = if exp >= 32 {
                0
            } else if exp >= 0 {
                ((m << exp) & 0xffff_ffff) as u32
            } else {
                let s = (-exp) as u32;
                if s >= 128 {
                    0
                } else {
                    ((m >> s) & 0xffff_ffff) as u32
                }
            };
            let r32 = if sign { vmod.wrapping_neg() } else { vmod };
            r32 as i32 as i64 as u64
        }
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// Conversions: float -> integer, integer -> float, and f64 <-> f32.
// ---------------------------------------------------------------------------

/// Convert a float to an integer. `signed`/`width` (32 or 64) select the
/// destination type; 32-bit results are sign-extended to 64 bits per the RISC-V
/// convention (including unsigned forms). Out-of-range and NaN inputs saturate
/// and raise NV; in-range inexact results raise NX.
pub fn ftoi<F: Sf>(x: F, signed: bool, width: u32, mode: RoundingMode, flags: &mut u32) -> u64 {
    // NaN saturates to the maximum of the destination type.
    if x.is_nan() {
        *flags |= fflags::NV;
        return match (signed, width) {
            (true, 16) => i16::MAX as u64,
            (false, 16) => u16::MAX as u64,
            (true, 32) => i32::MAX as i64 as u64,
            (false, 32) => u32::MAX as i32 as i64 as u64, // sign-extend 0xffffffff
            (true, 64) => i64::MAX as u64,
            (false, 64) => u64::MAX,
            _ => 0,
        };
    }
    let intg = x.round_integral(mode);
    let inexact = (intg.to_bits() != x.to_bits()) && !(intg.is_zero() && x.is_zero());
    let v = intg.to_i128(); // saturates at i128 bounds for huge magnitudes

    macro_rules! finish {
        ($ok:expr) => {{
            if inexact {
                *flags |= fflags::NX;
            }
            $ok
        }};
    }

    match (signed, width) {
        (true, 16) => {
            if v < i16::MIN as i128 {
                *flags |= fflags::NV;
                i16::MIN as u64
            } else if v > i16::MAX as i128 {
                *flags |= fflags::NV;
                i16::MAX as u64
            } else {
                finish!(v as i16 as u64)
            }
        }
        (false, 16) => {
            if v < 0 {
                *flags |= fflags::NV;
                0
            } else if v > u16::MAX as i128 {
                *flags |= fflags::NV;
                u16::MAX as u64
            } else {
                finish!(v as u16 as u64)
            }
        }
        (true, 32) => {
            if v < i32::MIN as i128 {
                *flags |= fflags::NV;
                i32::MIN as i64 as u64
            } else if v > i32::MAX as i128 {
                *flags |= fflags::NV;
                i32::MAX as i64 as u64
            } else {
                finish!(v as i32 as i64 as u64)
            }
        }
        (false, 32) => {
            if v < 0 {
                *flags |= fflags::NV;
                0
            } else if v > u32::MAX as i128 {
                *flags |= fflags::NV;
                u32::MAX as i32 as i64 as u64
            } else {
                finish!((v as u32) as i32 as i64 as u64)
            }
        }
        (true, 64) => {
            if v < i64::MIN as i128 {
                *flags |= fflags::NV;
                i64::MIN as u64
            } else if v > i64::MAX as i128 {
                *flags |= fflags::NV;
                i64::MAX as u64
            } else {
                finish!(v as i64 as u64)
            }
        }
        (false, 64) => {
            if v < 0 {
                *flags |= fflags::NV;
                0
            } else if v > u64::MAX as i128 {
                *flags |= fflags::NV;
                u64::MAX
            } else {
                finish!(v as u64)
            }
        }
        _ => 0,
    }
}

/// Convert an integer (already sign/zero-extended into `i128`) to a float,
/// rounding per `mode`. Integer-to-float never overflows the F/D exponent
/// range, so only NX can be raised.
pub fn itof<F: Sf>(v: i128, mode: RoundingMode, flags: &mut u32) -> F {
    if v == 0 {
        return F::ZERO; // +0.0
    }
    let r = F::from_i128(v); // round-to-nearest-even
    let r_int = r.to_i128();
    let delta = v - r_int; // exact integer residual
    if delta == 0 {
        return r;
    }
    *flags |= fflags::NX;
    let up = delta > 0;
    let (lo, hi) = if up {
        (r, next_up(r))
    } else {
        (next_down(r), r)
    };
    match mode {
        RoundingMode::Rtz => min_mag(lo, hi),
        RoundingMode::Rdn => lo,
        RoundingMode::Rup => hi,
        RoundingMode::Rmm => {
            let ulp = hi.to_i128() - lo.to_i128();
            if 2 * delta.unsigned_abs() as i128 == ulp {
                max_mag(lo, hi)
            } else {
                r
            }
        }
        _ => r,
    }
}

/// Narrow an `f64` to `f32` with rounding and flags.
pub fn f64_to_f32(x: f64, mode: RoundingMode, flags: &mut u32) -> u32 {
    if x.is_nan() {
        if x.signaling_nan() {
            *flags |= fflags::NV;
        }
        return CANONICAL_NAN_F32;
    }
    if x.is_infinite() {
        return if x.is_sign_negative() {
            f32::NEG_INFINITY.to_bits()
        } else {
            f32::INFINITY.to_bits()
        };
    }
    let r = x as f32; // round-to-nearest-even
    let inputs_finite = true;
    // Overflow.
    if r.is_infinite() {
        let mut f = 0u32;
        let res = correct::<f32>(r, 0.0, mode, inputs_finite, &mut f);
        *flags |= f;
        return res.to_bits();
    }
    // Exact residual in f64 (f32 widened to f64 is exact).
    let delta = x - (r as f64);
    if delta == 0.0 {
        return r.to_bits();
    }
    *flags |= fflags::NX;
    let up = delta > 0.0;
    let (lo, hi) = if up {
        (r, next_up(r))
    } else {
        (next_down(r), r)
    };
    let result = match mode {
        RoundingMode::Rtz => min_mag(lo, hi),
        RoundingMode::Rdn => lo,
        RoundingMode::Rup => hi,
        RoundingMode::Rmm => {
            let two_d = (delta.abs()) * 2.0;
            let ulp = (hi as f64) - (lo as f64);
            if two_d == ulp { max_mag(lo, hi) } else { r }
        }
        _ => r,
    };
    if result.is_infinite() {
        *flags |= fflags::OF;
    }
    // Tiny + inexact -> underflow (covers both subnormal and flush-to-zero).
    if result.is_finite() && result.abs() < f32::MIN_POSITIVE {
        *flags |= fflags::UF;
    }
    result.to_bits()
}

/// Widen an `f32` to `f64` (always exact; only NV on sNaN).
pub fn f32_to_f64(x: f32, flags: &mut u32) -> u64 {
    if x.is_nan() {
        if x.signaling_nan() {
            *flags |= fflags::NV;
        }
        return CANONICAL_NAN_F64;
    }
    (x as f64).to_bits()
}

// ===========================================================================
// Format-generic helpers (used by all of F16/F32/F64, notably half precision).
// ===========================================================================

/// Generic add: `a + b` in `fmt`.
pub fn sf_add(fmt: Fmt, a: u64, b: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    let (da, db) = (decompose(fmt, a), decompose(fmt, b));
    if matches!(da, Dec::Nan(_)) || matches!(db, Dec::Nan(_)) {
        if matches!(da, Dec::Nan(true)) || matches!(db, Dec::Nan(true)) {
            *flags |= fflags::NV;
        }
        return fmt.canon();
    }
    match (da, db) {
        (Dec::Inf(s1), Dec::Inf(s2)) => {
            if s1 != s2 {
                *flags |= fflags::NV;
                fmt.canon()
            } else {
                fmt.pack_inf(s1)
            }
        }
        (Dec::Inf(s), _) | (_, Dec::Inf(s)) => fmt.pack_inf(s),
        (Dec::Zero(s1), Dec::Zero(s2)) => {
            if s1 == s2 {
                fmt.pack_zero(s1)
            } else {
                fmt.pack_zero(mode == RoundingMode::Rdn)
            }
        }
        (Dec::Zero(_), _) => b,
        (_, Dec::Zero(_)) => a,
        (
            Dec::Finite {
                sign: s1,
                mant: m1,
                exp: e1,
            },
            Dec::Finite {
                sign: s2,
                mant: m2,
                exp: e2,
            },
        ) => add_wide(fmt, s1, m1 as u128, e1, s2, m2 as u128, e2, mode, flags),
        _ => unreachable!(),
    }
}

/// Generic subtract: `a - b` in `fmt`.
pub fn sf_sub(fmt: Fmt, a: u64, b: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    sf_add(fmt, a, b ^ fmt.sign_bit(), mode, flags)
}

/// Generic float-to-float conversion (rounds when narrowing, exact when
/// widening). Replaces the bespoke `f64<->f32` helpers and supports half.
pub fn fcvt_round(src: Fmt, dst: Fmt, bits: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    match decompose(src, bits) {
        Dec::Nan(sig) => {
            if sig {
                *flags |= fflags::NV;
            }
            dst.canon()
        }
        Dec::Inf(s) => dst.pack_inf(s),
        Dec::Zero(s) => dst.pack_zero(s),
        Dec::Finite { sign, mant, exp } => {
            round_pack(dst, sign, mant as u128, exp, false, mode, flags)
        }
    }
}

/// Generic integer-to-float conversion into `fmt`.
pub fn itof_fmt(fmt: Fmt, v: i128, mode: RoundingMode, flags: &mut u32) -> u64 {
    if v == 0 {
        return 0;
    }
    round_pack(fmt, v < 0, v.unsigned_abs(), 0, false, mode, flags)
}

/// Format-generic 10-bit classification mask.
pub fn fclass_bits(fmt: Fmt, bits: u64) -> u64 {
    match decompose(fmt, bits) {
        Dec::Nan(sig) => {
            if sig {
                1 << 8
            } else {
                1 << 9
            }
        }
        Dec::Inf(s) => {
            if s {
                1 << 0
            } else {
                1 << 7
            }
        }
        Dec::Zero(s) => {
            if s {
                1 << 3
            } else {
                1 << 4
            }
        }
        Dec::Finite { sign, mant, .. } => {
            let subnormal = mant < (1u64 << (fmt.p - 1));
            match (sign, subnormal) {
                (true, true) => 1 << 2,
                (false, true) => 1 << 5,
                (true, false) => 1 << 1,
                (false, false) => 1 << 6,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Half-precision (Zfh) min/max, compares and round-to-integral. These widen to
// f32 (exact, since f16 ⊂ f32) for ordering, with signaling detection on the
// raw 16-bit operands.
// ---------------------------------------------------------------------------

/// Widen a half to f32 exactly.
pub fn h_widen(bits: u16) -> f32 {
    f32::from_bits(fcvt_round(F16, F32, bits as u64, RoundingMode::Rne, &mut 0u32) as u32)
}
fn h_snan(bits: u16) -> bool {
    matches!(decompose(F16, bits as u64), Dec::Nan(true))
}
fn h_nan(bits: u16) -> bool {
    matches!(decompose(F16, bits as u64), Dec::Nan(_))
}
const H_CANON: u16 = 0x7e00;

/// `fmin.h` / `fminm.h` (NaN-propagating when `propagate`).
fn hmin(a: u16, b: u16, propagate: bool, flags: &mut u32) -> u16 {
    if h_snan(a) || h_snan(b) {
        *flags |= fflags::NV;
    }
    let (an, bn) = (h_nan(a), h_nan(b));
    if propagate && (an || bn) {
        return H_CANON;
    }
    if an && bn {
        return H_CANON;
    }
    if an {
        return b;
    }
    if bn {
        return a;
    }
    let (af, bf) = (h_widen(a), h_widen(b));
    if af == 0.0 && bf == 0.0 {
        return if (a >> 15) & 1 == 1 { a } else { b };
    }
    if af < bf { a } else { b }
}
/// `fmax.h` / `fmaxm.h`.
fn hmax(a: u16, b: u16, propagate: bool, flags: &mut u32) -> u16 {
    if h_snan(a) || h_snan(b) {
        *flags |= fflags::NV;
    }
    let (an, bn) = (h_nan(a), h_nan(b));
    if propagate && (an || bn) {
        return H_CANON;
    }
    if an && bn {
        return H_CANON;
    }
    if an {
        return b;
    }
    if bn {
        return a;
    }
    let (af, bf) = (h_widen(a), h_widen(b));
    if af == 0.0 && bf == 0.0 {
        return if (a >> 15) & 1 == 1 { b } else { a };
    }
    if af > bf { a } else { b }
}
pub fn fmin_h(a: u16, b: u16, flags: &mut u32) -> u16 {
    hmin(a, b, false, flags)
}
pub fn fmax_h(a: u16, b: u16, flags: &mut u32) -> u16 {
    hmax(a, b, false, flags)
}
pub fn fminm_h(a: u16, b: u16, flags: &mut u32) -> u16 {
    hmin(a, b, true, flags)
}
pub fn fmaxm_h(a: u16, b: u16, flags: &mut u32) -> u16 {
    hmax(a, b, true, flags)
}

/// Half compares. `signaling` raises NV on any NaN; otherwise only on sNaN.
fn hcmp_eq(a: u16, b: u16, signaling: bool, flags: &mut u32) -> bool {
    if h_snan(a) || h_snan(b) || (signaling && (h_nan(a) || h_nan(b))) {
        *flags |= fflags::NV;
    }
    if h_nan(a) || h_nan(b) {
        return false;
    }
    h_widen(a) == h_widen(b)
}
fn hcmp_lt(a: u16, b: u16, signaling: bool, flags: &mut u32) -> bool {
    if h_snan(a) || h_snan(b) || (signaling && (h_nan(a) || h_nan(b))) {
        *flags |= fflags::NV;
    }
    if h_nan(a) || h_nan(b) {
        return false;
    }
    h_widen(a) < h_widen(b)
}
fn hcmp_le(a: u16, b: u16, signaling: bool, flags: &mut u32) -> bool {
    if h_snan(a) || h_snan(b) || (signaling && (h_nan(a) || h_nan(b))) {
        *flags |= fflags::NV;
    }
    if h_nan(a) || h_nan(b) {
        return false;
    }
    h_widen(a) <= h_widen(b)
}
pub fn feq_h(a: u16, b: u16, flags: &mut u32) -> bool {
    hcmp_eq(a, b, false, flags)
}
pub fn flt_h(a: u16, b: u16, flags: &mut u32) -> bool {
    hcmp_lt(a, b, true, flags)
}
pub fn fle_h(a: u16, b: u16, flags: &mut u32) -> bool {
    hcmp_le(a, b, true, flags)
}
pub fn fleq_h(a: u16, b: u16, flags: &mut u32) -> bool {
    hcmp_le(a, b, false, flags)
}
pub fn fltq_h(a: u16, b: u16, flags: &mut u32) -> bool {
    hcmp_lt(a, b, false, flags)
}

/// `fround.h` / `froundnx.h`.
pub fn fround_h(bits: u16, mode: RoundingMode, set_nx: bool, flags: &mut u32) -> u16 {
    match decompose(F16, bits as u64) {
        Dec::Nan(sig) => {
            if sig {
                *flags |= fflags::NV;
            }
            H_CANON
        }
        Dec::Inf(_) | Dec::Zero(_) => bits,
        Dec::Finite { .. } => {
            let r = h_widen(bits).round_integral(mode);
            let rbits = fcvt_round(F32, F16, r.to_bits() as u64, mode, &mut 0u32) as u16;
            if set_nx && rbits != bits {
                *flags |= fflags::NX;
            }
            rbits
        }
    }
}

// ===========================================================================
// Integer-significand soft-float for mul / div / sqrt / fma.
//
// These operations need exact intermediate significands (up to 2x precision for
// products) to round correctly in every mode and to detect inexactness and
// underflow for subnormal results -- something the host f32/f64 (round-to-
// nearest only) cannot provide. Add/sub instead stay on the host FPU with an
// exact 2Sum residual, which is exact even for subnormal results.
// ===========================================================================

/// IEEE binary format parameters.
#[derive(Clone, Copy)]
pub struct Fmt {
    /// Precision in bits (significand including the implicit bit): 24 / 53.
    p: u32,
    /// Exponent field width: 8 / 11.
    exp_bits: u32,
}

/// Half precision (binary16).
pub const F16: Fmt = Fmt { p: 11, exp_bits: 5 };
/// Single precision (binary32).
pub const F32: Fmt = Fmt { p: 24, exp_bits: 8 };
/// Double precision (binary64).
pub const F64: Fmt = Fmt {
    p: 53,
    exp_bits: 11,
};

/// `vfrsqrt7.v` lookup table: 7 MSBs of the output significand indexed by
/// (exp[0] << 6) | sig[MSB-:6]. (RISC-V V-spec Table 58.)
#[rustfmt::skip]
const RSQRT7_TABLE: [u8; 128] = [
    52, 51, 50, 48, 47, 46, 44, 43, 42, 41, 40, 39, 38, 36, 35, 34,
    33, 32, 31, 30, 30, 29, 28, 27, 26, 25, 24, 23, 23, 22, 21, 20,
    19, 19, 18, 17, 16, 16, 15, 14, 14, 13, 12, 12, 11, 10, 10,  9,
     9,  8,  7,  7,  6,  6,  5,  4,  4,  3,  3,  2,  2,  1,  1,  0,
    127, 125, 123, 121, 119, 118, 116, 114, 113, 111, 109, 108, 106, 105, 103, 102,
    100, 99, 97, 96, 95, 93, 92, 91, 90, 88, 87, 86, 85, 84, 83, 82,
    80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70, 70, 69, 68, 67, 66,
    65, 64, 63, 63, 62, 61, 60, 59, 59, 58, 57, 56, 56, 55, 54, 53,
];

/// `vfrec7.v` lookup table: 7 MSBs of the output significand indexed by the
/// 7 MSBs of the normalized input significand. (RISC-V V-spec Table 59.)
#[rustfmt::skip]
const REC7_TABLE: [u8; 128] = [
    127, 125, 123, 121, 119, 117, 116, 114, 112, 110, 109, 107, 105, 104, 102, 100,
    99, 97, 96, 94, 93, 91, 90, 88, 87, 85, 84, 83, 81, 80, 79, 77,
    76, 75, 74, 72, 71, 70, 69, 68, 66, 65, 64, 63, 62, 61, 60, 59,
    58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43,
    42, 41, 40, 40, 39, 38, 37, 36, 35, 35, 34, 33, 32, 31, 31, 30,
    29, 28, 28, 27, 26, 25, 25, 24, 23, 23, 22, 21, 21, 20, 19, 19,
    18, 17, 17, 16, 15, 15, 14, 14, 13, 12, 12, 11, 11, 10, 9, 9,
    8, 8, 7, 7, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0,
];

/// `vfrsqrt7.v`: 7-bit estimate of 1/sqrt(x). Independent of rounding mode.
pub fn vfrsqrt7(fmt: Fmt, bits: u64, flags: &mut u32) -> u64 {
    let m = fmt.p - 1; // significand fraction bits
    let eb = fmt.exp_bits;
    let bias = (1i64 << (eb - 1)) - 1;
    let exp_mask = (1u64 << eb) - 1;
    let sig_mask = (1u64 << m) - 1;
    let sign = (bits >> (m + eb)) & 1;
    let exp = (bits >> m) & exp_mask;
    let sig = bits & sig_mask;
    let canon_nan = (exp_mask << m) | (1u64 << (m - 1));
    if exp == exp_mask {
        if sig == 0 {
            // +/-inf
            if sign == 1 {
                *flags |= fflags::NV;
                return canon_nan;
            }
            return 0; // +inf -> +0
        }
        if sig & (1u64 << (m - 1)) == 0 {
            *flags |= fflags::NV; // sNaN
        }
        return canon_nan;
    }
    if exp == 0 && sig == 0 {
        *flags |= fflags::DZ;
        return (sign << (m + eb)) | (exp_mask << m); // +/-inf
    }
    if sign == 1 {
        *flags |= fflags::NV; // negative -> canonical NaN
        return canon_nan;
    }
    let (norm_exp, norm_sig) = if exp != 0 {
        (exp as i64, sig)
    } else {
        let lz = (sig << (64 - m)).leading_zeros() as i64;
        (-lz, (sig << (1 + lz)) & sig_mask)
    };
    let idx = (((norm_exp & 1) as u64) << 6) | (norm_sig >> (m - 6));
    let sig_out = RSQRT7_TABLE[idx as usize] as u64;
    let out_exp = (3 * bias - 1 - norm_exp) / 2;
    ((out_exp as u64) << m) | (sig_out << (m - 7))
}

/// `vfrec7.v`: 7-bit estimate of 1/x. Overflow result depends on rounding mode.
pub fn vfrec7(fmt: Fmt, bits: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    let m = fmt.p - 1;
    let eb = fmt.exp_bits;
    let bias = (1i64 << (eb - 1)) - 1;
    let exp_mask = (1u64 << eb) - 1;
    let sig_mask = (1u64 << m) - 1;
    let sign = (bits >> (m + eb)) & 1;
    let exp = (bits >> m) & exp_mask;
    let sig = bits & sig_mask;
    let sbit = sign << (m + eb);
    let canon_nan = (exp_mask << m) | (1u64 << (m - 1));
    if exp == exp_mask {
        if sig == 0 {
            return sbit; // +/-inf -> +/-0
        }
        if sig & (1u64 << (m - 1)) == 0 {
            *flags |= fflags::NV;
        }
        return canon_nan;
    }
    if exp == 0 && sig == 0 {
        *flags |= fflags::DZ;
        return sbit | (exp_mask << m); // +/-inf
    }
    let (norm_exp, norm_sig) = if exp != 0 {
        (exp as i64, sig)
    } else {
        let lz = (sig << (64 - m)).leading_zeros() as i64;
        (-lz, (sig << (1 + lz)) & sig_mask)
    };
    let out_exp = 2 * bias - 1 - norm_exp;
    if out_exp > 2 * bias {
        // Overflow: result depends on sign and rounding mode.
        *flags |= fflags::NX | fflags::OF;
        let max_finite = ((exp_mask - 1) << m) | sig_mask;
        let inf = exp_mask << m;
        use RoundingMode::*;
        let mag = if sign == 0 {
            match mode {
                Rup | Rne | Rmm => inf,
                _ => max_finite, // Rdn, Rtz
            }
        } else {
            match mode {
                Rup | Rtz => max_finite,
                _ => inf, // Rdn, Rne, Rmm
            }
        };
        return sbit | mag;
    }
    let idx = norm_sig >> (m - 7);
    let sig_out = REC7_TABLE[idx as usize] as u64;
    if out_exp >= 1 {
        sbit | ((out_exp as u64) << m) | (sig_out << (m - 7))
    } else {
        // Subnormal output: denormalize the 1.sig_out significand.
        let full = (1u64 << m) | (sig_out << (m - 7));
        let shift = (1 - out_exp) as u32;
        sbit | (full >> shift)
    }
}

impl Fmt {
    #[inline]
    fn width(&self) -> u32 {
        self.p + self.exp_bits
    }
    #[inline]
    fn bias(&self) -> i32 {
        (1 << (self.exp_bits - 1)) - 1
    }
    #[inline]
    fn mant_bits(&self) -> u32 {
        self.p - 1
    }
    #[inline]
    fn max_field(&self) -> u32 {
        (1 << self.exp_bits) - 1
    }
    #[inline]
    fn frac_mask(&self) -> u64 {
        (1u64 << self.mant_bits()) - 1
    }
    #[inline]
    fn sign_bit(&self) -> u64 {
        1u64 << (self.width() - 1)
    }
    /// Canonical quiet NaN.
    fn canon(&self) -> u64 {
        ((self.max_field() as u64) << self.mant_bits()) | (1u64 << (self.mant_bits() - 1))
    }
    fn pack_zero(&self, sign: bool) -> u64 {
        if sign { self.sign_bit() } else { 0 }
    }
    fn pack_inf(&self, sign: bool) -> u64 {
        self.pack_zero(sign) | ((self.max_field() as u64) << self.mant_bits())
    }
    fn pack_max(&self, sign: bool) -> u64 {
        self.pack_zero(sign)
            | (((self.max_field() - 1) as u64) << self.mant_bits())
            | self.frac_mask()
    }
    fn pack(&self, sign: bool, field: u64, frac: u64) -> u64 {
        self.pack_zero(sign) | (field << self.mant_bits()) | (frac & self.frac_mask())
    }
}

/// A decomposed float operand.
enum Dec {
    Zero(bool),
    Inf(bool),
    /// `signaling` true for an sNaN.
    Nan(bool),
    /// Finite nonzero: value = (-1)^sign * mant * 2^exp.
    Finite {
        sign: bool,
        mant: u64,
        exp: i32,
    },
}

fn decompose(fmt: Fmt, bits: u64) -> Dec {
    let sign = (bits >> (fmt.width() - 1)) & 1 != 0;
    let field = ((bits >> fmt.mant_bits()) & (fmt.max_field() as u64)) as u32;
    let frac = bits & fmt.frac_mask();
    if field == fmt.max_field() {
        if frac == 0 {
            Dec::Inf(sign)
        } else {
            // signaling if the top fraction bit is clear
            Dec::Nan((frac >> (fmt.mant_bits() - 1)) & 1 == 0)
        }
    } else if field == 0 {
        if frac == 0 {
            Dec::Zero(sign)
        } else {
            Dec::Finite {
                sign,
                mant: frac,
                exp: 1 - fmt.bias() - fmt.mant_bits() as i32,
            }
        }
    } else {
        Dec::Finite {
            sign,
            mant: frac | (1u64 << fmt.mant_bits()),
            exp: field as i32 - fmt.bias() - fmt.mant_bits() as i32,
        }
    }
}

fn overflow_value(fmt: Fmt, sign: bool, mode: RoundingMode) -> u64 {
    match mode {
        RoundingMode::Rtz => fmt.pack_max(sign),
        RoundingMode::Rdn => {
            if sign {
                fmt.pack_inf(sign)
            } else {
                fmt.pack_max(sign)
            }
        }
        RoundingMode::Rup => {
            if sign {
                fmt.pack_max(sign)
            } else {
                fmt.pack_inf(sign)
            }
        }
        _ => fmt.pack_inf(sign),
    }
}

/// Round the value `(-1)^sign * mag * 2^exp` (plus extra `sticky_in` bits below
/// the LSB of `mag`) to `fmt`, setting NX/UF/OF.
fn round_pack(
    fmt: Fmt,
    sign: bool,
    mut mag: u128,
    mut exp: i32,
    sticky_in: bool,
    mode: RoundingMode,
    flags: &mut u32,
) -> u64 {
    if mag == 0 {
        if sticky_in {
            // an exact cancellation already produced zero with a tiny residual;
            // treat as inexact underflow to signed zero.
            *flags |= fflags::NX;
        }
        return fmt.pack_zero(sign);
    }
    let p = fmt.p as i32;
    let bias = fmt.bias();
    // Normalize the MSB of mag to bit 127.
    let lz = mag.leading_zeros();
    mag <<= lz;
    exp -= lz as i32;
    let e = 127 + exp; // unbiased exponent of the leading 1
    let mut biased = e + bias;
    let mut rshift = 128 - p;
    if biased < 1 {
        rshift += 1 - biased;
        biased = 0;
    }
    let (sig, guard, sticky) = if rshift > 128 {
        (0u128, 0u128, sticky_in || mag != 0)
    } else if rshift == 128 {
        // The guard bit is the MSB; everything below is sticky.
        let g = (mag >> 127) & 1;
        let low = (mag & ((1u128 << 127) - 1)) != 0;
        (0u128, g, sticky_in || low)
    } else {
        let g = (mag >> (rshift - 1)) & 1;
        let low = (mag & ((1u128 << (rshift - 1)) - 1)) != 0;
        (mag >> rshift, g, sticky_in || low)
    };
    let mut sigm = sig as u64;
    let inexact = guard != 0 || sticky;
    let round_up = match mode {
        RoundingMode::Rtz => false,
        RoundingMode::Rdn => sign && inexact,
        RoundingMode::Rup => !sign && inexact,
        RoundingMode::Rmm => guard != 0,
        _ => guard != 0 && (sticky || (sigm & 1) != 0),
    };
    if round_up {
        sigm += 1;
    }
    if inexact {
        *flags |= fflags::NX;
    }

    if biased == 0 {
        // Subnormal candidate.
        if sigm >> fmt.mant_bits() != 0 {
            // Rounded up to the smallest normal: not tiny, so no underflow flag.
            return fmt.pack(sign, 1, 0);
        }
        if inexact {
            *flags |= fflags::UF;
        }
        return fmt.pack(sign, 0, sigm);
    }
    // Normal; a rounding carry can bump the significand to 2^p.
    if sigm >> fmt.p != 0 {
        sigm >>= 1;
        biased += 1;
    }
    if biased >= fmt.max_field() as i32 {
        *flags |= fflags::OF | fflags::NX;
        return overflow_value(fmt, sign, mode);
    }
    fmt.pack(sign, biased as u64, sigm & fmt.frac_mask())
}

/// Integer square root of a u128, returning `(floor(sqrt(n)), remainder != 0)`.
fn isqrt_u128(n: u128) -> (u128, bool) {
    if n == 0 {
        return (0, false);
    }
    let mut bit: u128 = 1u128 << (((127 - n.leading_zeros()) & !1) as u32);
    let mut res: u128 = 0;
    let mut rem = n;
    while bit != 0 {
        if rem >= res + bit {
            rem -= res + bit;
            res = (res >> 1) + bit;
        } else {
            res >>= 1;
        }
        bit >>= 2;
    }
    (res, rem != 0)
}

/// `a * b` (soft-float).
pub fn sf_mul(fmt: Fmt, a: u64, b: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    let (da, db) = (decompose(fmt, a), decompose(fmt, b));
    if matches!(da, Dec::Nan(_)) || matches!(db, Dec::Nan(_)) {
        if matches!(da, Dec::Nan(true)) || matches!(db, Dec::Nan(true)) {
            *flags |= fflags::NV;
        }
        return fmt.canon();
    }
    let sign = dec_sign(&da) ^ dec_sign(&db);
    match (da, db) {
        (Dec::Inf(_), Dec::Zero(_)) | (Dec::Zero(_), Dec::Inf(_)) => {
            *flags |= fflags::NV;
            fmt.canon()
        }
        (Dec::Inf(_), _) | (_, Dec::Inf(_)) => fmt.pack_inf(sign),
        (Dec::Zero(_), _) | (_, Dec::Zero(_)) => fmt.pack_zero(sign),
        (
            Dec::Finite {
                mant: ma, exp: ea, ..
            },
            Dec::Finite {
                mant: mb, exp: eb, ..
            },
        ) => {
            let mag = (ma as u128) * (mb as u128);
            round_pack(fmt, sign, mag, ea + eb, false, mode, flags)
        }
        (Dec::Nan(_), _) | (_, Dec::Nan(_)) => unreachable!(),
    }
}

fn dec_sign(d: &Dec) -> bool {
    match d {
        Dec::Zero(s) | Dec::Inf(s) | Dec::Nan(s) => *s,
        Dec::Finite { sign, .. } => *sign,
    }
}

/// `a / b` (soft-float).
pub fn sf_div(fmt: Fmt, a: u64, b: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    let (da, db) = (decompose(fmt, a), decompose(fmt, b));
    if let Dec::Nan(s) = da {
        if s {
            *flags |= fflags::NV;
        }
        if let Dec::Nan(true) = db {
            *flags |= fflags::NV;
        }
        return fmt.canon();
    }
    if let Dec::Nan(s) = db {
        if s {
            *flags |= fflags::NV;
        }
        return fmt.canon();
    }
    let sign = dec_sign(&da) ^ dec_sign(&db);
    match (da, db) {
        (Dec::Inf(_), Dec::Inf(_)) | (Dec::Zero(_), Dec::Zero(_)) => {
            *flags |= fflags::NV;
            fmt.canon()
        }
        (Dec::Inf(_), _) => fmt.pack_inf(sign),
        (_, Dec::Inf(_)) => fmt.pack_zero(sign),
        (Dec::Zero(_), _) => fmt.pack_zero(sign),
        (_, Dec::Zero(_)) => {
            *flags |= fflags::DZ;
            fmt.pack_inf(sign)
        }
        (
            Dec::Finite {
                mant: ma,
                exp: mut ea,
                ..
            },
            Dec::Finite {
                mant: mb,
                exp: mut eb,
                ..
            },
        ) => {
            // Normalize both significands so their MSBs sit at bit 63; otherwise
            // a small (e.g. subnormal) dividend over a wide divisor yields a
            // quotient with too few significant bits to round correctly.
            let na = (ma as u64).leading_zeros();
            let nb = (mb as u64).leading_zeros();
            let ma = (ma as u128) << na;
            let mb = (mb as u128) << nb;
            ea -= na as i32;
            eb -= nb as i32;
            let num = ma << 64; // ma MSB at 63 -> num MSB at 127
            let q = num / mb;
            let sticky = (num % mb) != 0;
            round_pack(fmt, sign, q, ea - eb - 64, sticky, mode, flags)
        }
        _ => unreachable!(),
    }
}

/// `sqrt(a)` (soft-float).
pub fn sf_sqrt(fmt: Fmt, a: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    match decompose(fmt, a) {
        Dec::Nan(s) => {
            if s {
                *flags |= fflags::NV;
            }
            fmt.canon()
        }
        Dec::Zero(s) => fmt.pack_zero(s),
        Dec::Inf(false) => fmt.pack_inf(false),
        Dec::Inf(true) => {
            *flags |= fflags::NV;
            fmt.canon()
        }
        Dec::Finite { sign: true, .. } => {
            *flags |= fflags::NV;
            fmt.canon()
        }
        Dec::Finite {
            mant, exp: mut e, ..
        } => {
            let mut m = mant as u128;
            if e & 1 != 0 {
                m <<= 1;
                e -= 1;
            }
            // Scale up so the radicand has >= 2p+4 bits (so the root has >= p+2).
            let want = 2 * fmt.p + 4;
            let cur = 128 - m.leading_zeros();
            if cur < want {
                let mut s = want - cur;
                if s & 1 != 0 {
                    s += 1;
                }
                m <<= s;
                e -= s as i32;
            }
            let (root, rem) = isqrt_u128(m);
            round_pack(fmt, false, root, e / 2, rem, mode, flags)
        }
    }
}

/// `a * b + c` fused (soft-float, single rounding).
pub fn sf_fma(fmt: Fmt, a: u64, b: u64, c: u64, mode: RoundingMode, flags: &mut u32) -> u64 {
    let (da, db, dc) = (decompose(fmt, a), decompose(fmt, b), decompose(fmt, c));
    let a_zero = matches!(da, Dec::Zero(_));
    let b_zero = matches!(db, Dec::Zero(_));
    let a_inf = matches!(da, Dec::Inf(_));
    let b_inf = matches!(db, Dec::Inf(_));
    // A signaling NaN operand, or a 0*inf product, signals invalid -- the latter
    // even when the addend is a quiet NaN (IEEE 754-2008 7.2).
    let snan = matches!(da, Dec::Nan(true))
        || matches!(db, Dec::Nan(true))
        || matches!(dc, Dec::Nan(true));
    if snan || (a_zero && b_inf) || (b_zero && a_inf) {
        *flags |= fflags::NV;
    }
    if matches!(da, Dec::Nan(_)) || matches!(db, Dec::Nan(_)) || matches!(dc, Dec::Nan(_)) {
        return fmt.canon();
    }
    let psign = dec_sign(&da) ^ dec_sign(&db);
    // 0 * inf is invalid (NV already set above).
    if (a_zero && b_inf) || (b_zero && a_inf) {
        return fmt.canon();
    }
    if a_inf || b_inf {
        // product is +/-inf
        if let Dec::Inf(sc) = dc {
            if sc != psign {
                *flags |= fflags::NV;
                return fmt.canon();
            }
        }
        return fmt.pack_inf(psign);
    }
    if let Dec::Inf(sc) = dc {
        return fmt.pack_inf(sc);
    }
    // product is finite (possibly zero).
    if a_zero || b_zero {
        // product is +/-0; result is c (+ signed zero).
        return match dc {
            Dec::Zero(sc) => {
                // zero + zero
                if sc == psign {
                    fmt.pack_zero(sc)
                } else if mode == RoundingMode::Rdn {
                    fmt.pack_zero(true)
                } else {
                    fmt.pack_zero(false)
                }
            }
            _ => c,
        };
    }
    // Finite nonzero product + c.
    let (ma, ea) = fin(&da);
    let (mb, eb) = fin(&db);
    let pmant = (ma as u128) * (mb as u128);
    let pexp = ea + eb;
    match dc {
        Dec::Zero(_) => round_pack(fmt, psign, pmant, pexp, false, mode, flags),
        Dec::Finite {
            sign: sc,
            mant: mc,
            exp: ec,
        } => add_wide(fmt, psign, pmant, pexp, sc, mc as u128, ec, mode, flags),
        _ => unreachable!(),
    }
}

fn fin(d: &Dec) -> (u64, i32) {
    match d {
        Dec::Finite { mant, exp, .. } => (*mant, *exp),
        _ => (0, 0),
    }
}

/// Add two scaled magnitudes `(s1, m1*2^e1)` and `(s2, m2*2^e2)` exactly, then
/// round to `fmt`. `m1`/`m2` may be up to ~106-bit (product) significands.
#[allow(clippy::too_many_arguments)]
fn add_wide(
    fmt: Fmt,
    s1: bool,
    m1: u128,
    e1: i32,
    s2: bool,
    m2: u128,
    e2: i32,
    mode: RoundingMode,
    flags: &mut u32,
) -> u64 {
    // Order by value-exponent of the MSB so `hi` is the larger magnitude term.
    let v1 = e1 + (127 - m1.leading_zeros()) as i32;
    let v2 = e2 + (127 - m2.leading_zeros()) as i32;
    let ((sh, mh, eh), (sl, ml, el)) = if (v1, m1) >= (v2, m2) {
        ((s1, m1, e1), (s2, m2, e2))
    } else {
        ((s2, m2, e2), (s1, m1, e1))
    };
    // Normalize hi so its MSB sits at bit 120 (headroom for carry + guard bits).
    const TP: i32 = 120;
    let mh_msb = (127 - mh.leading_zeros()) as i32;
    let up = TP - mh_msb;
    let hm = if up >= 0 { mh << up } else { mh >> (-up) };
    let h_lsb = eh - up;
    // Align lo to hi's LSB exponent.
    let shift = el - h_lsb;
    let mut sticky = false;
    let lm = if shift >= 0 {
        if shift >= 128 {
            sticky = ml != 0;
            0
        } else {
            ml << shift
        }
    } else {
        let rs = (-shift) as u32;
        if rs >= 128 {
            sticky = ml != 0;
            0
        } else {
            sticky = (ml & ((1u128 << rs) - 1)) != 0;
            ml >> rs
        }
    };

    if sh == sl {
        round_pack(fmt, sh, hm + lm, h_lsb, sticky, mode, flags)
    } else {
        // Subtraction: embed the sticky as a borrow using two guard bits so the
        // result's low bits reflect "just below" the integer difference.
        let hm2 = hm << 2;
        let lm2 = (lm << 2) | (sticky as u128);
        if hm2 >= lm2 {
            let mag = hm2 - lm2;
            if mag == 0 {
                let neg = mode == RoundingMode::Rdn;
                return fmt.pack_zero(neg);
            }
            round_pack(fmt, sh, mag, h_lsb - 2, false, mode, flags)
        } else {
            round_pack(fmt, sl, lm2 - hm2, h_lsb - 2, false, mode, flags)
        }
    }
}

// ===========================================================================
// Single-source scalar FP evaluation (shared by the RISC-V SMIR lift).
// ===========================================================================

use crate::riscv::Op as RvFpInsn;

/// Does this OP-FP / FMA op write its result to an **integer** (x) register?
/// (compares, fp->int conversions, `fclass`, `fcvtmod`, `fmv.x.*`). Everything
/// else writes an f-register.
pub fn fp_writes_int_dst(op: RvFpInsn) -> bool {
    use RvFpInsn::*;
    matches!(
        op,
        FeqS | FltS
            | FleS
            | FeqD
            | FltD
            | FleD
            | FeqH
            | FltH
            | FleH
            | FleqS
            | FltqS
            | FleqD
            | FltqD
            | FleqH
            | FltqH
            | FclassS
            | FclassD
            | FclassH
            | FcvtWS
            | FcvtWuS
            | FcvtLS
            | FcvtLuS
            | FcvtWD
            | FcvtWuD
            | FcvtLD
            | FcvtLuD
            | FcvtWH
            | FcvtWuH
            | FcvtLH
            | FcvtLuH
            | FcvtmodWD
            | FmvXW
            | FmvXD
            | FmvXH
    )
}

/// Does this OP-FP op take its first source operand from an **integer** (x)
/// register? (int->fp conversions and `fmv.*.x`). Everything else reads an
/// f-register.
pub fn fp_uses_int_src1(op: RvFpInsn) -> bool {
    use RvFpInsn::*;
    matches!(
        op,
        FcvtSW
            | FcvtSWu
            | FcvtSL
            | FcvtSLu
            | FcvtDW
            | FcvtDWu
            | FcvtDL
            | FcvtDLu
            | FcvtHW
            | FcvtHWu
            | FcvtHL
            | FcvtHLu
            | FmvWX
            | FmvDX
            | FmvHX
    )
}

/// Pure evaluation of a scalar OP-FP / FMA RISC-V instruction. `a`/`b`/`c` carry
/// the raw 64-bit source register values — the raw f-register bits for
/// FP-source operands (NaN-boxed exactly as stored), or the raw x-register value
/// in `a` for integer-source ops ([`fp_uses_int_src1`]). Returns the raw value
/// to write into the destination register — NaN-boxed for an f-register
/// destination, or the integer result for an x-register destination
/// ([`fp_writes_int_dst`]) — together with the input `fcsr` with this op's
/// accrued exception flags OR'd in. Returns `None` for an illegal rounding-mode
/// field (hardware raises an illegal-instruction trap: no state change), and for
/// any op outside the OP-FP / FMA arithmetic set this function covers
/// (loads/stores, sign-injection and the moves/classify already lifted to plain
/// SMIR are intentionally excluded).
///
/// This calls the same soft-float primitives ([`add`], [`sf_mul`], [`sf_fma`],
/// …) used by `RiscVCpu::exec_fp`, so a lifted SMIR `RvFp` op is bit-identical to
/// the qemu-verified interpreter; the thin operand wrapper below mirrors
/// `exec_fp` exactly and is independently checked by `tests/riscv_smir_lift.rs`.
pub fn eval_scalar_fp(
    op: RvFpInsn,
    rm_field: u8,
    fcsr: u32,
    a: u64,
    b: u64,
    c: u64,
) -> Option<(u64, u32)> {
    use RvFpInsn::*;

    // Resolve the effective rounding mode (mirrors `RiscVCpu::eff_rm`). Ops whose
    // funct3 does not encode a rounding mode never consult it.
    let needs_rm = !matches!(
        op,
        FminS
            | FmaxS
            | FminD
            | FmaxD
            | FminH
            | FmaxH
            | FeqS
            | FltS
            | FleS
            | FeqD
            | FltD
            | FleD
            | FeqH
            | FltH
            | FleH
            | FminmS
            | FmaxmS
            | FminmD
            | FmaxmD
            | FminmH
            | FmaxmH
            | FleqS
            | FltqS
            | FleqD
            | FltqD
            | FleqH
            | FltqH
            | FliS
            | FliD
            | FliH
            | FcvtmodWD
    );
    let rm = if needs_rm {
        let m = RoundingMode::from_bits(rm_field)?;
        let m = if m == RoundingMode::Dyn {
            RoundingMode::from_bits(((fcsr >> 5) & 0x7) as u8)?
        } else {
            m
        };
        if m == RoundingMode::Dyn {
            return None; // dynamic field itself selecting dynamic is illegal
        }
        m
    } else {
        RoundingMode::Rne
    };

    let mut flags = 0u32;

    // Operand unboxing (mirrors RiscVCpu::rf32/rf16/s32/h).
    let rf32 = |bits: u64| -> f32 {
        if (bits >> 32) == 0xffff_ffff {
            f32::from_bits(bits as u32)
        } else {
            f32::from_bits(CANONICAL_NAN_F32)
        }
    };
    let rf64 = |bits: u64| -> f64 { f64::from_bits(bits) };
    let s32 = |bits: u64| -> u64 { rf32(bits).to_bits() as u64 };
    let rf16 = |bits: u64| -> u16 {
        if (bits >> 16) == 0xffff_ffff_ffff {
            bits as u16
        } else {
            0x7e00
        }
    };
    let h = |bits: u64| -> u64 { rf16(bits) as u64 };
    // Result NaN-boxing (mirrors wf32/wf16/wf64).
    let box_s = |bits: u32| -> u64 { 0xffff_ffff_0000_0000u64 | bits as u64 };
    let box_h = |bits: u16| -> u64 { 0xffff_ffff_ffff_0000u64 | bits as u64 };
    // Sign flips for the FMA variants.
    let neg32 = |bits: u64| bits ^ 0x8000_0000;
    let neg64 = |bits: u64| bits ^ 0x8000_0000_0000_0000;
    let neg16 = |bits: u64| bits ^ 0x8000;

    let result: u64 = match op {
        // ---- single-precision arithmetic ----
        FaddS => box_s(add(rf32(a), rf32(b), rm, &mut flags).to_bits()),
        FsubS => box_s(sub(rf32(a), rf32(b), rm, &mut flags).to_bits()),
        FmulS => box_s(sf_mul(F32, s32(a), s32(b), rm, &mut flags) as u32),
        FdivS => box_s(sf_div(F32, s32(a), s32(b), rm, &mut flags) as u32),
        FsqrtS => box_s(sf_sqrt(F32, s32(a), rm, &mut flags) as u32),
        FmaddS => box_s(sf_fma(F32, s32(a), s32(b), s32(c), rm, &mut flags) as u32),
        FmsubS => box_s(sf_fma(F32, s32(a), s32(b), neg32(s32(c)), rm, &mut flags) as u32),
        FnmsubS => box_s(sf_fma(F32, neg32(s32(a)), s32(b), s32(c), rm, &mut flags) as u32),
        FnmaddS => box_s(sf_fma(F32, neg32(s32(a)), s32(b), neg32(s32(c)), rm, &mut flags) as u32),

        // ---- double-precision arithmetic ----
        FaddD => add(rf64(a), rf64(b), rm, &mut flags).to_bits(),
        FsubD => sub(rf64(a), rf64(b), rm, &mut flags).to_bits(),
        FmulD => sf_mul(F64, a, b, rm, &mut flags),
        FdivD => sf_div(F64, a, b, rm, &mut flags),
        FsqrtD => sf_sqrt(F64, a, rm, &mut flags),
        FmaddD => sf_fma(F64, a, b, c, rm, &mut flags),
        FmsubD => sf_fma(F64, a, b, neg64(c), rm, &mut flags),
        FnmsubD => sf_fma(F64, neg64(a), b, c, rm, &mut flags),
        FnmaddD => sf_fma(F64, neg64(a), b, neg64(c), rm, &mut flags),

        // ---- half-precision arithmetic ----
        FaddH => box_h(sf_add(F16, h(a), h(b), rm, &mut flags) as u16),
        FsubH => box_h(sf_sub(F16, h(a), h(b), rm, &mut flags) as u16),
        FmulH => box_h(sf_mul(F16, h(a), h(b), rm, &mut flags) as u16),
        FdivH => box_h(sf_div(F16, h(a), h(b), rm, &mut flags) as u16),
        FsqrtH => box_h(sf_sqrt(F16, h(a), rm, &mut flags) as u16),
        FmaddH => box_h(sf_fma(F16, h(a), h(b), h(c), rm, &mut flags) as u16),
        FmsubH => box_h(sf_fma(F16, h(a), h(b), neg16(h(c)), rm, &mut flags) as u16),
        FnmsubH => box_h(sf_fma(F16, neg16(h(a)), h(b), h(c), rm, &mut flags) as u16),
        FnmaddH => box_h(sf_fma(F16, neg16(h(a)), h(b), neg16(h(c)), rm, &mut flags) as u16),

        // ---- min / max ----
        FminS => box_s(fmin(rf32(a), rf32(b), &mut flags).to_bits()),
        FmaxS => box_s(fmax(rf32(a), rf32(b), &mut flags).to_bits()),
        FminD => fmin(rf64(a), rf64(b), &mut flags).to_bits(),
        FmaxD => fmax(rf64(a), rf64(b), &mut flags).to_bits(),
        FminH => box_h(fmin_h(rf16(a), rf16(b), &mut flags)),
        FmaxH => box_h(fmax_h(rf16(a), rf16(b), &mut flags)),
        FminmS => box_s(fminm(rf32(a), rf32(b), &mut flags).to_bits()),
        FmaxmS => box_s(fmaxm(rf32(a), rf32(b), &mut flags).to_bits()),
        FminmD => fminm(rf64(a), rf64(b), &mut flags).to_bits(),
        FmaxmD => fmaxm(rf64(a), rf64(b), &mut flags).to_bits(),
        FminmH => box_h(fminm_h(rf16(a), rf16(b), &mut flags)),
        FmaxmH => box_h(fmaxm_h(rf16(a), rf16(b), &mut flags)),

        // ---- comparisons (write an integer register) ----
        FeqS => feq(rf32(a), rf32(b), &mut flags) as u64,
        FltS => flt(rf32(a), rf32(b), &mut flags) as u64,
        FleS => fle(rf32(a), rf32(b), &mut flags) as u64,
        FeqD => feq(rf64(a), rf64(b), &mut flags) as u64,
        FltD => flt(rf64(a), rf64(b), &mut flags) as u64,
        FleD => fle(rf64(a), rf64(b), &mut flags) as u64,
        FeqH => feq_h(rf16(a), rf16(b), &mut flags) as u64,
        FltH => flt_h(rf16(a), rf16(b), &mut flags) as u64,
        FleH => fle_h(rf16(a), rf16(b), &mut flags) as u64,
        FleqS => fleq(rf32(a), rf32(b), &mut flags) as u64,
        FltqS => fltq(rf32(a), rf32(b), &mut flags) as u64,
        FleqD => fleq(rf64(a), rf64(b), &mut flags) as u64,
        FltqD => fltq(rf64(a), rf64(b), &mut flags) as u64,
        FleqH => fleq_h(rf16(a), rf16(b), &mut flags) as u64,
        FltqH => fltq_h(rf16(a), rf16(b), &mut flags) as u64,

        // ---- round to integer ----
        FroundS => box_s(fround(rf32(a), rm, false, &mut flags).to_bits()),
        FroundnxS => box_s(fround(rf32(a), rm, true, &mut flags).to_bits()),
        FroundD => fround(rf64(a), rm, false, &mut flags).to_bits(),
        FroundnxD => fround(rf64(a), rm, true, &mut flags).to_bits(),
        FroundH => box_h(fround_h(rf16(a), rm, false, &mut flags)),
        FroundnxH => box_h(fround_h(rf16(a), rm, true, &mut flags)),

        // ---- float -> integer conversions ----
        FcvtWS => ftoi(rf32(a), true, 32, rm, &mut flags),
        FcvtWuS => ftoi(rf32(a), false, 32, rm, &mut flags),
        FcvtLS => ftoi(rf32(a), true, 64, rm, &mut flags),
        FcvtLuS => ftoi(rf32(a), false, 64, rm, &mut flags),
        FcvtWD => ftoi(rf64(a), true, 32, rm, &mut flags),
        FcvtWuD => ftoi(rf64(a), false, 32, rm, &mut flags),
        FcvtLD => ftoi(rf64(a), true, 64, rm, &mut flags),
        FcvtLuD => ftoi(rf64(a), false, 64, rm, &mut flags),
        FcvtWH => ftoi(h_widen(rf16(a)), true, 32, rm, &mut flags),
        FcvtWuH => ftoi(h_widen(rf16(a)), false, 32, rm, &mut flags),
        FcvtLH => ftoi(h_widen(rf16(a)), true, 64, rm, &mut flags),
        FcvtLuH => ftoi(h_widen(rf16(a)), false, 64, rm, &mut flags),
        FcvtmodWD => fcvtmod_w_d(rf64(a), &mut flags),

        // ---- integer -> float conversions ----
        FcvtSW => box_s({
            let r: f32 = itof(a as i32 as i128, rm, &mut flags);
            r.to_bits()
        }),
        FcvtSWu => box_s({
            let r: f32 = itof(a as u32 as i128, rm, &mut flags);
            r.to_bits()
        }),
        FcvtSL => box_s({
            let r: f32 = itof(a as i64 as i128, rm, &mut flags);
            r.to_bits()
        }),
        FcvtSLu => box_s({
            let r: f32 = itof(a as i128, rm, &mut flags);
            r.to_bits()
        }),
        FcvtDW => {
            let r: f64 = itof(a as i32 as i128, rm, &mut flags);
            r.to_bits()
        }
        FcvtDWu => {
            let r: f64 = itof(a as u32 as i128, rm, &mut flags);
            r.to_bits()
        }
        FcvtDL => {
            let r: f64 = itof(a as i64 as i128, rm, &mut flags);
            r.to_bits()
        }
        FcvtDLu => {
            let r: f64 = itof(a as i128, rm, &mut flags);
            r.to_bits()
        }
        FcvtHW => box_h(itof_fmt(F16, a as i32 as i128, rm, &mut flags) as u16),
        FcvtHWu => box_h(itof_fmt(F16, a as u32 as i128, rm, &mut flags) as u16),
        FcvtHL => box_h(itof_fmt(F16, a as i64 as i128, rm, &mut flags) as u16),
        FcvtHLu => box_h(itof_fmt(F16, a as i128, rm, &mut flags) as u16),

        // ---- float <-> float conversions ----
        FcvtSD => box_s(f64_to_f32(rf64(a), rm, &mut flags)),
        FcvtDS => f32_to_f64(rf32(a), &mut flags),
        FcvtSH => box_s(fcvt_round(F16, F32, h(a), rm, &mut flags) as u32),
        FcvtHS => box_h(fcvt_round(F32, F16, s32(a), rm, &mut flags) as u16),
        FcvtDH => fcvt_round(F16, F64, h(a), rm, &mut flags),
        FcvtHD => box_h(fcvt_round(F64, F16, a, rm, &mut flags) as u16),

        // FliS/FliD/FliH (Zfa load-immediate) are NOT handled here: their `rs1`
        // field is an immediate table index, not a register value, so the lift
        // materialises the constant directly (see the lifter).
        _ => return None,
    };

    Some((result, fcsr | (flags & 0x1f)))
}
