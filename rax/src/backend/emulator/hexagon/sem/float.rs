//! (float) Hexagon IEEE-754 floating-point instructions (`F2_*`).
//!
//! Semantics verified bit-exactly (result **and** USR FP flags) against the
//! `qemu-hexagon` oracle, which is the ground truth for this corpus. The oracle
//! implements these ops with QEMU softfloat configured (see
//! `target/hexagon/cpu.c` / `arch.c`) as:
//!   * `default_nan_mode = 1`, default-NaN pattern = all-ones -> any NaN result
//!     is canonicalised to `0xFFFF_FFFF` (f32) / `0xFFFF_FFFF_FFFF_FFFF` (f64).
//!     This matches the idef `fUNFLOAT`/`fUNDOUBLE` macros.
//!   * **No** denormal flushing (`flush_inputs_to_zero`/`flush_to_zero` are not
//!     set) -> denormals behave per IEEE.
//!   * `detect_tininess_before_rounding`, round-to-nearest-even (USR starts 0).
//!   * FP exception flags accumulate into USR sticky bits:
//!       bit1 FPINVF(invalid), bit2 FPDBZF(divbyzero), bit3 FPOVFF(overflow),
//!       bit4 FPUNFF(underflow), bit5 FPINPF(inexact).
//!     `input_denormal_used` is NOT mapped to USR.
//!
//! Only ops whose result AND USR match the oracle exactly are dispatched here.
//! Implemented: compares (sf/df cmp eq/gt/ge/uo), classify (sf/df), make
//! immediates (sf/df, pos/neg), min/max (sf/df), sfmpy, all int<->float
//! conversions (incl. `:chop`), df2sf, sf2df.
//!
//! Intentionally NOT implemented (fall through): sfadd/sfsub and all f64
//! arithmetic (dfadd/dfsub/...). Their exact (pre-rounding) result cannot be
//! represented with native f32/f64, so the softfloat inexact/underflow flags
//! cannot be derived bit-exactly — and a wrong float op would break the shared
//! regression guard for everyone.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

const USR_FPINVF: u32 = 1 << 1; // invalid operation sticky flag
const USR_FPOVFF: u32 = 1 << 3; // overflow sticky flag
const USR_FPUNFF: u32 = 1 << 4; // underflow sticky flag
const USR_FPINPF: u32 = 1 << 5; // inexact sticky flag

// ---- bit-pattern helpers --------------------------------------------------

#[inline]
fn f32_is_nan(b: u32) -> bool {
    (b & 0x7f80_0000) == 0x7f80_0000 && (b & 0x007f_ffff) != 0
}
#[inline]
fn f32_is_snan(b: u32) -> bool {
    f32_is_nan(b) && (b & 0x0040_0000) == 0 // top mantissa bit clear => signaling
}
#[inline]
fn f64_is_nan(b: u64) -> bool {
    (b & 0x7ff0_0000_0000_0000) == 0x7ff0_0000_0000_0000 && (b & 0x000f_ffff_ffff_ffff) != 0
}
#[inline]
fn f64_is_snan(b: u64) -> bool {
    f64_is_nan(b) && (b & 0x0008_0000_0000_0000) == 0
}

/// Hexagon `fpclassify` category (computed on raw bits — denormals report
/// SUBNORMAL, matching softfloat's `float32_is_denormal` etc.).
#[derive(PartialEq)]
enum FpClass {
    Zero,
    Normal,
    Subnormal,
    Infinite,
    Nan,
}

fn classify_sf(b: u32) -> FpClass {
    let exp = (b >> 23) & 0xff;
    let mant = b & 0x007f_ffff;
    if exp == 0 {
        if mant == 0 {
            FpClass::Zero
        } else {
            FpClass::Subnormal
        }
    } else if exp == 0xff {
        if mant == 0 {
            FpClass::Infinite
        } else {
            FpClass::Nan
        }
    } else {
        FpClass::Normal
    }
}

fn classify_df(b: u64) -> FpClass {
    let exp = (b >> 52) & 0x7ff;
    let mant = b & 0x000f_ffff_ffff_ffff;
    if exp == 0 {
        if mant == 0 {
            FpClass::Zero
        } else {
            FpClass::Subnormal
        }
    } else if exp == 0x7ff {
        if mant == 0 {
            FpClass::Infinite
        } else {
            FpClass::Nan
        }
    } else {
        FpClass::Normal
    }
}

/// QEMU compare relation.
#[derive(PartialEq, Clone, Copy)]
enum Rel {
    Less,
    Equal,
    Greater,
    Unordered,
}

/// `float32_compare_quiet`: ordered IEEE compare; unordered if either is NaN.
/// Raises invalid only on a signaling NaN (quiet variant).
fn cmp_sf_quiet(a: u32, b: u32, ctx: &mut SemCtx) -> Rel {
    if f32_is_nan(a) || f32_is_nan(b) {
        if f32_is_snan(a) || f32_is_snan(b) {
            ctx.usr_or |= USR_FPINVF;
        }
        return Rel::Unordered;
    }
    let (fa, fb) = (f32::from_bits(a), f32::from_bits(b));
    if fa < fb {
        Rel::Less
    } else if fa > fb {
        Rel::Greater
    } else {
        Rel::Equal
    }
}

fn cmp_df_quiet(a: u64, b: u64, ctx: &mut SemCtx) -> Rel {
    if f64_is_nan(a) || f64_is_nan(b) {
        if f64_is_snan(a) || f64_is_snan(b) {
            ctx.usr_or |= USR_FPINVF;
        }
        return Rel::Unordered;
    }
    let (fa, fb) = (f64::from_bits(a), f64::from_bits(b));
    if fa < fb {
        Rel::Less
    } else if fa > fb {
        Rel::Greater
    } else {
        Rel::Equal
    }
}

// ---- min / max (QEMU `*_maximum_number` / `*_minimum_number`) -------------
//
// IEEE-754-2019 minimumNumber/maximumNumber:
//   * one QNaN + a number  -> the number (no invalid).
//   * any SNaN (not both NaN) -> invalid, return the non-NaN operand.
//   * both NaN              -> default NaN (all-ones), invalid iff an SNaN.
//   * otherwise magnitude/sign compare; ties resolved by sign so that
//     max(+0,-0)=+0, min(+0,-0)=-0.

fn sf_minmax(a: u32, b: u32, is_min: bool, ctx: &mut SemCtx) -> u32 {
    let an = f32_is_nan(a);
    let bn = f32_is_nan(b);
    if an || bn {
        let snan = f32_is_snan(a) || f32_is_snan(b);
        if !(an && bn) {
            // exactly one NaN
            if snan {
                ctx.usr_or |= USR_FPINVF;
            }
            return if an { b } else { a };
        }
        // both NaN
        if snan {
            ctx.usr_or |= USR_FPINVF;
        }
        return 0xFFFF_FFFF; // default NaN
    }
    let fa = f32::from_bits(a);
    let fb = f32::from_bits(b);
    // Magnitude+sign compare; handle signed-zero ties explicitly.
    let pick_a = if fa == fb {
        // equal value (covers +0/-0); decide by sign bit.
        let sa = (a >> 31) & 1;
        let sb = (b >> 31) & 1;
        if sa == sb {
            true
        } else if is_min {
            sa == 1 // min picks the negative one
        } else {
            sa == 0 // max picks the positive one
        }
    } else if is_min {
        fa < fb
    } else {
        fa > fb
    };
    if pick_a { a } else { b }
}

fn df_minmax(a: u64, b: u64, is_min: bool, ctx: &mut SemCtx) -> u64 {
    let an = f64_is_nan(a);
    let bn = f64_is_nan(b);
    if an || bn {
        let snan = f64_is_snan(a) || f64_is_snan(b);
        if !(an && bn) {
            if snan {
                ctx.usr_or |= USR_FPINVF;
            }
            return if an { b } else { a };
        }
        if snan {
            ctx.usr_or |= USR_FPINVF;
        }
        return 0xFFFF_FFFF_FFFF_FFFF;
    }
    let fa = f64::from_bits(a);
    let fb = f64::from_bits(b);
    let pick_a = if fa == fb {
        let sa = (a >> 63) & 1;
        let sb = (b >> 63) & 1;
        if sa == sb {
            true
        } else if is_min {
            sa == 1
        } else {
            sa == 0
        }
    } else if is_min {
        fa < fb
    } else {
        fa > fb
    };
    if pick_a { a } else { b }
}

fn class_match(class: &FpClass, imm: u32) -> u8 {
    let bit = |n: u32| (imm >> n) & 1 == 1;
    let hit = (bit(0) && *class == FpClass::Zero)
        || (bit(1) && *class == FpClass::Normal)
        || (bit(2) && *class == FpClass::Subnormal)
        || (bit(3) && *class == FpClass::Infinite)
        || (bit(4) && *class == FpClass::Nan);
    if hit { 0xff } else { 0x00 }
}

// ---- integer -> float conversions -----------------------------------------
//
// QEMU uses softfloat int->float with round-to-nearest-even (USR rounding 0).
// These can only ever raise the *inexact* flag (the source magnitude needs more
// significant bits than the destination mantissa, with nonzero low bits). They
// never produce NaN/inf/overflow/underflow for the in-range integer inputs.

/// `true` if rounding integer `magnitude` to a float with `mant_bits` mantissa
/// bits (including the implicit leading 1) loses precision.
#[inline]
fn int_conv_inexact(magnitude: u128, mant_bits: u32) -> bool {
    if magnitude == 0 {
        return false;
    }
    let sig = 128 - magnitude.leading_zeros();
    if sig <= mant_bits {
        return false;
    }
    // Low (sig - mant_bits) bits must all be zero to be exact.
    let dropped = sig - mant_bits;
    (magnitude & ((1u128 << dropped) - 1)) != 0
}

/// Convert an integer (given as signed magnitude info) to an f32 result with
/// the Hexagon inexact flag, returning the encoded bits.
fn i_to_sf(value_mag: u128, result: f32, ctx: &mut SemCtx) -> u32 {
    if int_conv_inexact(value_mag, 24) {
        ctx.usr_or |= USR_FPINPF;
    }
    result.to_bits()
}

fn i_to_df(value_mag: u128, result: f64, ctx: &mut SemCtx) -> u64 {
    if int_conv_inexact(value_mag, 53) {
        ctx.usr_or |= USR_FPINPF;
    }
    result.to_bits()
}

// ---- float -> integer conversions -----------------------------------------
//
// QEMU's Hexagon helpers special-case NaN (signed: result -1) and negatives
// (unsigned: result 0), then defer to softfloat `float_to_{s,u}int`. Softfloat:
//   * inf / out-of-range  -> invalid, saturate (signed: sign?min:max;
//     unsigned: sign?0:max).
//   * in-range normal      -> round, set inexact iff a fraction was dropped.
// USR exception bits that matter: invalid (bit1) and inexact (bit5).
// Rounding mode: round-to-nearest-even (USR=0) for the base op,
// round-toward-zero for `:chop`.

#[inline]
fn round_for(f: f64, chop: bool) -> f64 {
    if chop { f.trunc() } else { f.round_ties_even() }
}
#[inline]
fn round_for_f32(f: f32, chop: bool) -> f32 {
    if chop { f.trunc() } else { f.round_ties_even() }
}

/// Convert a (non-NaN) integer-valued rounded float `ri` plus the inexact
/// indicator into a signed integer of `min..=max`, raising USR flags. `ri` may
/// be ±inf (treated as out of range).
fn float_to_sint(ri: f64, inexact: bool, min: i128, max: i128, ctx: &mut SemCtx) -> i128 {
    let v = ri as i128; // exact for integer-valued finite |ri| < 2^127; inf saturates
    if v < min || v > max || !ri.is_finite() {
        ctx.usr_or |= USR_FPINVF;
        if ri.is_sign_negative() { min } else { max }
    } else {
        if inexact {
            ctx.usr_or |= USR_FPINPF;
        }
        v
    }
}

/// Convert a non-negative rounded float (caller has already handled NaN and
/// strictly-negative inputs) to an unsigned integer of `0..=max`. `ri` may be
/// +inf (out of range).
fn float_to_uint(ri: f64, inexact: bool, max: u128, ctx: &mut SemCtx) -> u128 {
    if !ri.is_finite() {
        // +inf -> invalid, max.
        ctx.usr_or |= USR_FPINVF;
        return max;
    }
    let v = ri as i128; // ri >= 0 here
    if v < 0 || (v as u128) > max {
        ctx.usr_or |= USR_FPINVF;
        max
    } else {
        if inexact {
            ctx.usr_or |= USR_FPINPF;
        }
        v as u128
    }
}

// ---- single-precision multiply --------------------------------------------
//
// f32*f32 is computed *exactly* in f64: the product needs at most 48 significant
// bits (fits f64's 53-bit significand) and the product exponent stays well
// within f64's range, so `exact` is the true infinite-precision product. We then
// round-to-nearest-even back to f32 (identical to QEMU softfloat with USR
// rounding 0 and no denormal flush) and derive the USR exception flags from the
// exact (pre-rounding) result, matching softfloat's `tininess_before_rounding`
// underflow detection.
//
// (sfadd/sfsub are deliberately not built on this helper: the exact *sum* of two
// f32 with a wide exponent spread needs far more than 53 bits, so an f64
// intermediate silently drops bits and would miss inexact/underflow flags.)

/// Round the *exact* f64 result of an f32 op back to f32 and raise the matching
/// USR flags, returning the canonicalised bit pattern.
fn finish_sf(a: u32, b: u32, exact: f64, ctx: &mut SemCtx) -> u32 {
    // Invalid: a signaling-NaN operand, or a NaN produced from non-NaN inputs
    // (inf-inf / 0*inf -> default-NaN result).
    let res = exact as f32;
    if f32_is_snan(a) || f32_is_snan(b) || (res.is_nan() && !f32_is_nan(a) && !f32_is_nan(b)) {
        ctx.usr_or |= USR_FPINVF;
    }
    if res.is_nan() {
        return 0xFFFF_FFFF; // default NaN
    }
    if res.is_infinite() {
        // Overflow only when the (finite) exact result rounded up to infinity.
        if exact.is_finite() {
            ctx.usr_or |= USR_FPOVFF | USR_FPINPF;
        }
        return res.to_bits();
    }
    // Finite result. Inexact iff rounding f64->f32 changed the value.
    let inexact = (res as f64) != exact;
    if inexact {
        ctx.usr_or |= USR_FPINPF;
    }
    // Underflow (tininess detected before rounding): exact magnitude is in the
    // subnormal range and the result is inexact.
    let min_normal = f32::MIN_POSITIVE as f64;
    if exact != 0.0 && exact.abs() < min_normal && inexact {
        ctx.usr_or |= USR_FPUNFF;
    }
    res.to_bits()
}

/// `conv_df2sf` narrowing (f64 -> f32). A single correctly-rounded narrowing of
/// an exact f64 value; the inexact test has no double-rounding hazard because
/// the input is already exactly representable. Flags: invalid iff sNaN input;
/// inexact iff rounding lost precision; overflow iff a finite input rounds to
/// inf; underflow iff tiny (pre-rounding) and inexact.
fn df_to_sf(b: u64, ctx: &mut SemCtx) -> u32 {
    if f64_is_nan(b) {
        if f64_is_snan(b) {
            ctx.usr_or |= USR_FPINVF;
        }
        return 0xFFFF_FFFF; // default NaN
    }
    let x = f64::from_bits(b);
    let res = x as f32;
    if res.is_infinite() {
        if x.is_finite() {
            ctx.usr_or |= USR_FPOVFF | USR_FPINPF;
        }
        return res.to_bits();
    }
    let inexact = (res as f64) != x;
    if inexact {
        ctx.usr_or |= USR_FPINPF;
    }
    let min_normal = f32::MIN_POSITIVE as f64;
    if x != 0.0 && x.abs() < min_normal && inexact {
        ctx.usr_or |= USR_FPUNFF;
    }
    res.to_bits()
}

/// `conv_sf2{w,d}` (signed): Hexagon returns -1 + invalid for NaN, else softfloat.
fn sf_to_sint(b: u32, chop: bool, min: i128, max: i128, ctx: &mut SemCtx) -> i128 {
    if f32_is_nan(b) {
        ctx.usr_or |= USR_FPINVF;
        return -1; // truncated to the dest width by the caller
    }
    let f = f32::from_bits(b);
    let ri = round_for_f32(f, chop);
    float_to_sint(ri as f64, ri != f, min, max, ctx)
}

/// `conv_sf2{uw,ud}` (unsigned): NaN -> max+invalid; strictly-negative ->
/// 0+invalid (Hexagon checks sign before rounding); else softfloat.
fn sf_to_uint(b: u32, chop: bool, max: u128, ctx: &mut SemCtx) -> u128 {
    if f32_is_nan(b) {
        ctx.usr_or |= USR_FPINVF;
        return max;
    }
    let f = f32::from_bits(b);
    // is_neg && !zero  (covers -0 as non-negative path)
    if (b & 0x8000_0000) != 0 && f != 0.0 {
        ctx.usr_or |= USR_FPINVF;
        return 0;
    }
    let ri = round_for_f32(f, chop);
    float_to_uint(ri as f64, ri != f, max, ctx)
}

fn df_to_sint(b: u64, chop: bool, min: i128, max: i128, ctx: &mut SemCtx) -> i128 {
    if f64_is_nan(b) {
        ctx.usr_or |= USR_FPINVF;
        return -1;
    }
    let f = f64::from_bits(b);
    let ri = round_for(f, chop);
    float_to_sint(ri, ri != f, min, max, ctx)
}

fn df_to_uint(b: u64, chop: bool, max: u128, ctx: &mut SemCtx) -> u128 {
    if f64_is_nan(b) {
        ctx.usr_or |= USR_FPINVF;
        return max;
    }
    let f = f64::from_bits(b);
    if (b & 0x8000_0000_0000_0000) != 0 && f != 0.0 {
        ctx.usr_or |= USR_FPINVF;
        return 0;
    }
    let ri = round_for(f, chop);
    float_to_uint(ri, ri != f, max, ctx)
}

/// Execute a float-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let rd = fld(d, b'd');
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let pred = |c: &mut SemCtx, r: Rel, set: &[Rel]| {
        let v = if set.contains(&r) { 0xff } else { 0x00 };
        c.set_p(rd, v);
    };

    match op {
        // ---- single-precision compares (-> predicate) ----
        Opcode::F2_sfcmpeq => {
            let r = cmp_sf_quiet(s(ctx), t(ctx), ctx);
            pred(ctx, r, &[Rel::Equal]);
        }
        Opcode::F2_sfcmpgt => {
            let r = cmp_sf_quiet(s(ctx), t(ctx), ctx);
            pred(ctx, r, &[Rel::Greater]);
        }
        Opcode::F2_sfcmpge => {
            let r = cmp_sf_quiet(s(ctx), t(ctx), ctx);
            pred(ctx, r, &[Rel::Greater, Rel::Equal]);
        }
        Opcode::F2_sfcmpuo => {
            let r = cmp_sf_quiet(s(ctx), t(ctx), ctx);
            pred(ctx, r, &[Rel::Unordered]);
        }

        // ---- double-precision compares (-> predicate) ----
        Opcode::F2_dfcmpeq => {
            let r = cmp_df_quiet(sp(ctx), tp(ctx), ctx);
            pred(ctx, r, &[Rel::Equal]);
        }
        Opcode::F2_dfcmpgt => {
            let r = cmp_df_quiet(sp(ctx), tp(ctx), ctx);
            pred(ctx, r, &[Rel::Greater]);
        }
        Opcode::F2_dfcmpge => {
            let r = cmp_df_quiet(sp(ctx), tp(ctx), ctx);
            pred(ctx, r, &[Rel::Greater, Rel::Equal]);
        }
        Opcode::F2_dfcmpuo => {
            let r = cmp_df_quiet(sp(ctx), tp(ctx), ctx);
            pred(ctx, r, &[Rel::Unordered]);
        }

        // ---- classify (-> predicate); always clears FP flags (USR stays 0) ----
        Opcode::F2_sfclass => {
            let imm = fld_imm(d);
            let v = class_match(&classify_sf(s(ctx)), imm);
            ctx.set_p(rd, v);
        }
        Opcode::F2_dfclass => {
            let imm = fld_imm(d);
            let v = class_match(&classify_df(sp(ctx)), imm);
            ctx.set_p(rd, v);
        }

        // ---- single-precision multiply ----
        // f32*f32 is computed *exactly* in f64 (the 48-bit product fits f64's
        // 53-bit significand and the exponent is well within f64 range), so the
        // round-to-f32 and the derived USR flags are bit-exact. sfadd/sfsub are
        // intentionally NOT implemented: the exact sum of two f32 with a large
        // exponent spread is not representable in f64, so the inexact/underflow
        // flags cannot be derived reliably with native float types.
        Opcode::F2_sfmpy => {
            let (a, b) = (s(ctx), t(ctx));
            let exact = f32::from_bits(a) as f64 * f32::from_bits(b) as f64;
            let v = finish_sf(a, b, exact, ctx);
            ctx.set_r(rd, v);
        }

        // ---- min / max ----
        Opcode::F2_sfmax => {
            let v = sf_minmax(s(ctx), t(ctx), false, ctx);
            ctx.set_r(rd, v);
        }
        Opcode::F2_sfmin => {
            let v = sf_minmax(s(ctx), t(ctx), true, ctx);
            ctx.set_r(rd, v);
        }
        Opcode::F2_dfmax => {
            let v = df_minmax(sp(ctx), tp(ctx), false, ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::F2_dfmin => {
            let v = df_minmax(sp(ctx), tp(ctx), true, ctx);
            ctx.set_rp(rd, v);
        }

        // ---- immediate make (pure integer; USR stays 0) ----
        Opcode::F2_sfimm_p => {
            let imm = fld_imm(d);
            let v = ((127u32 - 6) << 23).wrapping_add(imm << 17);
            ctx.set_r(rd, v);
        }
        Opcode::F2_sfimm_n => {
            let imm = fld_imm(d);
            let v = ((127u32 - 6) << 23).wrapping_add(imm << 17) | (1 << 31);
            ctx.set_r(rd, v);
        }
        Opcode::F2_dfimm_p => {
            let imm = fld_imm(d) as u64;
            let v = ((1023u64 - 6) << 52).wrapping_add(imm << 46);
            ctx.set_rp(rd, v);
        }
        Opcode::F2_dfimm_n => {
            let imm = fld_imm(d) as u64;
            let v = ((1023u64 - 6) << 52).wrapping_add(imm << 46) | (1u64 << 63);
            ctx.set_rp(rd, v);
        }

        // ---- integer -> float conversions ----
        Opcode::F2_conv_w2sf => {
            let x = s(ctx) as i32;
            let v = i_to_sf(x.unsigned_abs() as u128, x as f32, ctx);
            ctx.set_r(rd, v);
        }
        Opcode::F2_conv_uw2sf => {
            let x = s(ctx);
            let v = i_to_sf(x as u128, x as f32, ctx);
            ctx.set_r(rd, v);
        }
        Opcode::F2_conv_d2sf => {
            let x = sp(ctx) as i64;
            let v = i_to_sf(x.unsigned_abs() as u128, x as f32, ctx);
            ctx.set_r(rd, v);
        }
        Opcode::F2_conv_ud2sf => {
            let x = sp(ctx);
            let v = i_to_sf(x as u128, x as f32, ctx);
            ctx.set_r(rd, v);
        }
        Opcode::F2_conv_w2df => {
            let x = s(ctx) as i32;
            let v = i_to_df(x.unsigned_abs() as u128, x as f64, ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::F2_conv_uw2df => {
            let x = s(ctx);
            let v = i_to_df(x as u128, x as f64, ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::F2_conv_d2df => {
            let x = sp(ctx) as i64;
            let v = i_to_df(x.unsigned_abs() as u128, x as f64, ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::F2_conv_ud2df => {
            let x = sp(ctx);
            let v = i_to_df(x as u128, x as f64, ctx);
            ctx.set_rp(rd, v);
        }

        // ---- sf -> signed int ----
        Opcode::F2_conv_sf2w | Opcode::F2_conv_sf2w_chop => {
            let chop = matches!(op, Opcode::F2_conv_sf2w_chop);
            let v = sf_to_sint(s(ctx), chop, i32::MIN as i128, i32::MAX as i128, ctx);
            ctx.set_r(rd, v as i32 as u32);
        }
        Opcode::F2_conv_sf2d | Opcode::F2_conv_sf2d_chop => {
            let chop = matches!(op, Opcode::F2_conv_sf2d_chop);
            let v = sf_to_sint(s(ctx), chop, i64::MIN as i128, i64::MAX as i128, ctx);
            ctx.set_rp(rd, v as i64 as u64);
        }
        // ---- sf -> unsigned int ----
        Opcode::F2_conv_sf2uw | Opcode::F2_conv_sf2uw_chop => {
            let chop = matches!(op, Opcode::F2_conv_sf2uw_chop);
            let v = sf_to_uint(s(ctx), chop, u32::MAX as u128, ctx);
            ctx.set_r(rd, v as u32);
        }
        Opcode::F2_conv_sf2ud | Opcode::F2_conv_sf2ud_chop => {
            let chop = matches!(op, Opcode::F2_conv_sf2ud_chop);
            let v = sf_to_uint(s(ctx), chop, u64::MAX as u128, ctx);
            ctx.set_rp(rd, v as u64);
        }
        // ---- df -> signed int ----
        Opcode::F2_conv_df2w | Opcode::F2_conv_df2w_chop => {
            let chop = matches!(op, Opcode::F2_conv_df2w_chop);
            let v = df_to_sint(sp(ctx), chop, i32::MIN as i128, i32::MAX as i128, ctx);
            ctx.set_r(rd, v as i32 as u32);
        }
        Opcode::F2_conv_df2d | Opcode::F2_conv_df2d_chop => {
            let chop = matches!(op, Opcode::F2_conv_df2d_chop);
            let v = df_to_sint(sp(ctx), chop, i64::MIN as i128, i64::MAX as i128, ctx);
            ctx.set_rp(rd, v as i64 as u64);
        }
        // ---- df -> unsigned int ----
        Opcode::F2_conv_df2uw | Opcode::F2_conv_df2uw_chop => {
            let chop = matches!(op, Opcode::F2_conv_df2uw_chop);
            let v = df_to_uint(sp(ctx), chop, u32::MAX as u128, ctx);
            ctx.set_r(rd, v as u32);
        }
        Opcode::F2_conv_df2ud | Opcode::F2_conv_df2ud_chop => {
            let chop = matches!(op, Opcode::F2_conv_df2ud_chop);
            let v = df_to_uint(sp(ctx), chop, u64::MAX as u128, ctx);
            ctx.set_rp(rd, v as u64);
        }

        // ---- df -> sf narrowing ----
        Opcode::F2_conv_df2sf => {
            let v = df_to_sf(sp(ctx), ctx);
            ctx.set_r(rd, v);
        }

        // ---- sf -> df widening (always exact; invalid iff sNaN input) ----
        Opcode::F2_conv_sf2df => {
            let b = s(ctx);
            if f32_is_nan(b) {
                if f32_is_snan(b) {
                    ctx.usr_or |= USR_FPINVF;
                }
                ctx.set_rp(rd, 0xFFFF_FFFF_FFFF_FFFF);
            } else {
                let v = f32::from_bits(b) as f64;
                ctx.set_rp(rd, v.to_bits());
            }
        }

        _ => return false,
    }
    true
}

/// Read the immediate field (letter `i`) raw value; these FP immediates are
/// never constant-extended.
#[inline]
fn fld_imm(d: &DecodedOp) -> u32 {
    d.field(b'i').map(|f| f.value).unwrap_or(0)
}
