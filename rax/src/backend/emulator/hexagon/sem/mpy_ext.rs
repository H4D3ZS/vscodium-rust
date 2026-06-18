//! (mpy_ext) Hexagon halfword/word integer-multiply matrix.
//!
//! Implements the systematic `M2_mpy*` 16x16 multiply family
//! (`M2_mpy_{hh,hl,lh,ll}_s{0,1}` and the `_acc`/`_nac`/`_rnd`/`_sat`/
//! `_sat_rnd`/`_acc_sat`/`_nac_sat` variants), the 64-bit-result `M2_mpyd_*`
//! family, the unsigned `M2_mpyu_*`/`M2_mpyud_*` families, the
//! `M2_hmmpy{h,l}_{s1,rs1}` 32x16 high-half multiplies, and the
//! `M6_vabsdiff{b,ub}` byte absolute-difference ops.
//!
//! Semantics taken verbatim from the Hexagon V68 spec
//! (semantics_generated.pyinc) and the f-macros in `imported/macros.def`:
//!   fMPY16SS(a,b)   = fSE32_64(fSE16_32(a)*fSE16_32(b))    (signed 16x16 -> i32 -> i64)
//!   fMPY16UU(a,b)   = fZE32_64(fZE16_32(a)*fZE16_32(b))    (unsigned 16x16 -> u32 -> i64)
//!   fMPY3216SS(a,b) = fSE32_64(a) * fSXTN(16,64,b)         (i32 x signed-16 -> i64)
//!   fSCALE(N,A)     = ((size8s_t)A) << N
//!   fROUND(A)       = A + 0x8000
//!   fSAT(A)         = fSATN(32,A)   (signed-32 saturate, sets USR:OVF)
//!   fGETHALF(N,S)   = (size2s_t)((S>>(N*16))&0xffff)       (signed half)
//!   fGETUHALF(N,S)  = (size2u_t)((S>>(N*16))&0xffff)       (unsigned half)
//!   fGETBYTE/fGETUBYTE/fSETBYTE                            (byte lanes)
//!   fABS(A)         = (A<0)?-A:A
//!
//! Verified against the qemu-hexagon oracle (tests/hexagon_diff.rs).

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

/// Accumulate mode for the 16x16 multiply matrix.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Acc {
    /// `Rd = product` (no read-modify).
    Set,
    /// `Rx += product`.
    Add,
    /// `Rx -= product`.
    Sub,
}

/// fGETWORD(n, src): signed 32-bit lane of a 64-bit pair, sign-extended to i64.
#[inline]
fn get_word(src: u64, n: u32) -> i64 {
    ((src >> (n * 32)) as u32 as i32) as i64
}

/// M7 complex-multiply word selection per the `CMPY64`/`CMPY128` idef macros:
/// `prod = (Rss.w[w0] * Rtt.w[w1]) {+,-} (Rss.w[w2] * Rtt.w[w3])`, each term a
/// full signed 32x32 product (fMPY32SS) widened to 128 bits before the add/sub.
/// Returned as i128 so the wcmpy shift/saturate path keeps full precision.
#[inline]
fn cmpy_terms(rss: u64, rtt: u64, w0: u32, w1: u32, w2: u32, w3: u32, add: bool) -> i128 {
    let tmp = (get_word(rss, w0) as i128) * (get_word(rtt, w1) as i128);
    let acc = (get_word(rss, w2) as i128) * (get_word(rtt, w3) as i128);
    if add { tmp + acc } else { tmp - acc }
}

/// fGETHALF(n, src): signed 16-bit lane, sign-extended to i64.
#[inline]
fn get_half(src: u32, n: u32) -> i64 {
    (((src >> (n * 16)) & 0xffff) as u16 as i16) as i64
}
/// fGETUHALF(n, src): unsigned 16-bit lane, zero-extended to i64.
#[inline]
fn get_uhalf(src: u32, n: u32) -> i64 {
    ((src >> (n * 16)) & 0xffff) as i64
}
/// fGETBYTE(n, src): signed 8-bit lane, sign-extended to i64.
#[inline]
fn get_byte(src: u64, n: u32) -> i64 {
    (((src >> (n * 8)) & 0xff) as u8 as i8) as i64
}
/// fGETUBYTE(n, src): unsigned 8-bit lane, zero-extended to i64.
#[inline]
fn get_ubyte(src: u64, n: u32) -> i64 {
    ((src >> (n * 8)) & 0xff) as i64
}
/// fSETBYTE(n, dst, val): insert the low 8 bits of `val` into byte lane `n`.
#[inline]
fn set_byte(dst: u64, n: u32, val: i64) -> u64 {
    let sh = n * 8;
    (dst & !(0xffu64 << sh)) | (((val as u64) & 0xff) << sh)
}

/// Parametrised 16x16 multiply (the `M2_mpy*` matrix).
///
/// * `s_high`/`t_high`: select Rs.H/Rs.L and Rt.H/Rt.L (`1` = high half).
/// * `unsigned`: use fMPY16UU (`mpyu`/`mpyud`) instead of fMPY16SS.
/// * `s1`: fSCALE(1,..) — shift the product left by one (the `:<<1` form).
/// * `acc`: Set (`Rd=`), Add (`Rx+=`), or Sub (`Rx-=`).
/// * `rnd`: fROUND — add 0x8000 (only ever combined with `Acc::Set`).
/// * `sat`: fSAT — saturate the result to signed 32 (non-wide only).
/// * `wide`: 64-bit Rdd/Rxx result (`mpyd`/`mpyud`) vs 32-bit Rd/Rx.
#[allow(clippy::too_many_arguments)]
#[inline]
fn mpy16(
    ctx: &mut SemCtx,
    d: &DecodedOp,
    s_high: u32,
    t_high: u32,
    unsigned: bool,
    s1: bool,
    acc: Acc,
    rnd: bool,
    sat: bool,
    wide: bool,
) {
    let rs = ctx.r(fld(d, b's'));
    let rt = ctx.r(fld(d, b't'));

    // 16x16 product, sign- or zero-extended into i64 (always fits in i32/u32).
    let prod: i64 = if unsigned {
        let a = get_uhalf(rs, s_high) as u32;
        let b = get_uhalf(rt, t_high) as u32;
        (a.wrapping_mul(b) as u64) as i64
    } else {
        let a = get_half(rs, s_high) as i32;
        let b = get_half(rt, t_high) as i32;
        (a.wrapping_mul(b)) as i64
    };

    // fSCALE(1, product) when the `:<<1` form is selected.
    let scaled = if s1 { prod << 1 } else { prod };

    // Combine with the accumulator (read OLD Rx) per the acc/nac/set rule.
    // fROUND (+0x8000) only ever appears on the `Acc::Set` rnd forms.
    let val: i64 = match acc {
        Acc::Set => {
            if rnd {
                scaled + 0x8000
            } else {
                scaled
            }
        }
        Acc::Add => {
            let old = if wide {
                ctx.rp(fld(d, b'x')) as i64
            } else {
                ctx.r(fld(d, b'x')) as i32 as i64
            };
            old.wrapping_add(scaled)
        }
        Acc::Sub => {
            let old = if wide {
                ctx.rp(fld(d, b'x')) as i64
            } else {
                ctx.r(fld(d, b'x')) as i32 as i64
            };
            old.wrapping_sub(scaled)
        }
    };

    // fSAT (signed-32 saturate) is applied to the final value when present.
    let val = if sat { ctx.sat_n(val, 32) } else { val };

    let dst = match acc {
        Acc::Set => fld(d, b'd'),
        Acc::Add | Acc::Sub => fld(d, b'x'),
    };
    if wide {
        ctx.set_rp(dst, val as u64);
    } else {
        ctx.set_r(dst, val as u32);
    }
}

/// Execute a mpy_ext opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let rd = fld(d, b'd');
    match op {
        // ============ 16x16 multiply matrix (M2_mpy* / mpyd / mpyu / mpyud) ============
        Opcode::M2_mpy_acc_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpy_acc_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Add, false, false, false),
        Opcode::M2_mpy_acc_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpy_acc_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Add, false, false, false),
        Opcode::M2_mpy_acc_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpy_acc_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Add, false, false, false),
        Opcode::M2_mpy_acc_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpy_acc_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Add, false, false, false),
        Opcode::M2_mpy_acc_sat_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_hh_s1 => {
            mpy16(ctx, d, 1, 1, false, true, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_hl_s1 => {
            mpy16(ctx, d, 1, 0, false, true, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_lh_s1 => {
            mpy16(ctx, d, 0, 1, false, true, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Add, false, true, false)
        }
        Opcode::M2_mpy_acc_sat_ll_s1 => {
            mpy16(ctx, d, 0, 0, false, true, Acc::Add, false, true, false)
        }
        Opcode::M2_mpyd_acc_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyd_acc_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Add, false, false, true),
        Opcode::M2_mpyd_acc_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyd_acc_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Add, false, false, true),
        Opcode::M2_mpyd_acc_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyd_acc_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Add, false, false, true),
        Opcode::M2_mpyd_acc_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyd_acc_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Add, false, false, true),
        Opcode::M2_mpyd_hh_s0 => mpy16(ctx, d, 1, 1, false, false, Acc::Set, false, false, true),
        Opcode::M2_mpyd_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Set, false, false, true),
        Opcode::M2_mpyd_hl_s0 => mpy16(ctx, d, 1, 0, false, false, Acc::Set, false, false, true),
        Opcode::M2_mpyd_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Set, false, false, true),
        Opcode::M2_mpyd_lh_s0 => mpy16(ctx, d, 0, 1, false, false, Acc::Set, false, false, true),
        Opcode::M2_mpyd_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Set, false, false, true),
        Opcode::M2_mpyd_ll_s0 => mpy16(ctx, d, 0, 0, false, false, Acc::Set, false, false, true),
        Opcode::M2_mpyd_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Set, false, false, true),
        Opcode::M2_mpyd_nac_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyd_nac_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyd_nac_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyd_nac_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyd_nac_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyd_nac_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyd_nac_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyd_nac_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyd_rnd_hh_s0 => mpy16(ctx, d, 1, 1, false, false, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_hl_s0 => mpy16(ctx, d, 1, 0, false, false, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_lh_s0 => mpy16(ctx, d, 0, 1, false, false, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_ll_s0 => mpy16(ctx, d, 0, 0, false, false, Acc::Set, true, false, true),
        Opcode::M2_mpyd_rnd_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Set, true, false, true),
        Opcode::M2_mpy_hh_s0 => mpy16(ctx, d, 1, 1, false, false, Acc::Set, false, false, false),
        Opcode::M2_mpy_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Set, false, false, false),
        Opcode::M2_mpy_hl_s0 => mpy16(ctx, d, 1, 0, false, false, Acc::Set, false, false, false),
        Opcode::M2_mpy_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Set, false, false, false),
        Opcode::M2_mpy_lh_s0 => mpy16(ctx, d, 0, 1, false, false, Acc::Set, false, false, false),
        Opcode::M2_mpy_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Set, false, false, false),
        Opcode::M2_mpy_ll_s0 => mpy16(ctx, d, 0, 0, false, false, Acc::Set, false, false, false),
        Opcode::M2_mpy_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Set, false, false, false),
        Opcode::M2_mpy_nac_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpy_nac_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Sub, false, false, false),
        Opcode::M2_mpy_nac_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpy_nac_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Sub, false, false, false),
        Opcode::M2_mpy_nac_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpy_nac_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Sub, false, false, false),
        Opcode::M2_mpy_nac_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpy_nac_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Sub, false, false, false),
        Opcode::M2_mpy_nac_sat_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_hh_s1 => {
            mpy16(ctx, d, 1, 1, false, true, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_hl_s1 => {
            mpy16(ctx, d, 1, 0, false, true, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_lh_s1 => {
            mpy16(ctx, d, 0, 1, false, true, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_nac_sat_ll_s1 => {
            mpy16(ctx, d, 0, 0, false, true, Acc::Sub, false, true, false)
        }
        Opcode::M2_mpy_rnd_hh_s0 => mpy16(ctx, d, 1, 1, false, false, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_hl_s0 => mpy16(ctx, d, 1, 0, false, false, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_lh_s0 => mpy16(ctx, d, 0, 1, false, false, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_ll_s0 => mpy16(ctx, d, 0, 0, false, false, Acc::Set, true, false, false),
        Opcode::M2_mpy_rnd_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Set, true, false, false),
        Opcode::M2_mpy_sat_hh_s0 => mpy16(ctx, d, 1, 1, false, false, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_hh_s1 => mpy16(ctx, d, 1, 1, false, true, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_hl_s0 => mpy16(ctx, d, 1, 0, false, false, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_hl_s1 => mpy16(ctx, d, 1, 0, false, true, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_lh_s0 => mpy16(ctx, d, 0, 1, false, false, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_lh_s1 => mpy16(ctx, d, 0, 1, false, true, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_ll_s0 => mpy16(ctx, d, 0, 0, false, false, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_ll_s1 => mpy16(ctx, d, 0, 0, false, true, Acc::Set, false, true, false),
        Opcode::M2_mpy_sat_rnd_hh_s0 => {
            mpy16(ctx, d, 1, 1, false, false, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_hh_s1 => {
            mpy16(ctx, d, 1, 1, false, true, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_hl_s0 => {
            mpy16(ctx, d, 1, 0, false, false, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_hl_s1 => {
            mpy16(ctx, d, 1, 0, false, true, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_lh_s0 => {
            mpy16(ctx, d, 0, 1, false, false, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_lh_s1 => {
            mpy16(ctx, d, 0, 1, false, true, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_ll_s0 => {
            mpy16(ctx, d, 0, 0, false, false, Acc::Set, true, true, false)
        }
        Opcode::M2_mpy_sat_rnd_ll_s1 => {
            mpy16(ctx, d, 0, 0, false, true, Acc::Set, true, true, false)
        }
        Opcode::M2_mpyu_acc_hh_s0 => {
            mpy16(ctx, d, 1, 1, true, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpyu_acc_hh_s1 => mpy16(ctx, d, 1, 1, true, true, Acc::Add, false, false, false),
        Opcode::M2_mpyu_acc_hl_s0 => {
            mpy16(ctx, d, 1, 0, true, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpyu_acc_hl_s1 => mpy16(ctx, d, 1, 0, true, true, Acc::Add, false, false, false),
        Opcode::M2_mpyu_acc_lh_s0 => {
            mpy16(ctx, d, 0, 1, true, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpyu_acc_lh_s1 => mpy16(ctx, d, 0, 1, true, true, Acc::Add, false, false, false),
        Opcode::M2_mpyu_acc_ll_s0 => {
            mpy16(ctx, d, 0, 0, true, false, Acc::Add, false, false, false)
        }
        Opcode::M2_mpyu_acc_ll_s1 => mpy16(ctx, d, 0, 0, true, true, Acc::Add, false, false, false),
        Opcode::M2_mpyud_acc_hh_s0 => {
            mpy16(ctx, d, 1, 1, true, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyud_acc_hh_s1 => mpy16(ctx, d, 1, 1, true, true, Acc::Add, false, false, true),
        Opcode::M2_mpyud_acc_hl_s0 => {
            mpy16(ctx, d, 1, 0, true, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyud_acc_hl_s1 => mpy16(ctx, d, 1, 0, true, true, Acc::Add, false, false, true),
        Opcode::M2_mpyud_acc_lh_s0 => {
            mpy16(ctx, d, 0, 1, true, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyud_acc_lh_s1 => mpy16(ctx, d, 0, 1, true, true, Acc::Add, false, false, true),
        Opcode::M2_mpyud_acc_ll_s0 => {
            mpy16(ctx, d, 0, 0, true, false, Acc::Add, false, false, true)
        }
        Opcode::M2_mpyud_acc_ll_s1 => mpy16(ctx, d, 0, 0, true, true, Acc::Add, false, false, true),
        Opcode::M2_mpyud_hh_s0 => mpy16(ctx, d, 1, 1, true, false, Acc::Set, false, false, true),
        Opcode::M2_mpyud_hh_s1 => mpy16(ctx, d, 1, 1, true, true, Acc::Set, false, false, true),
        Opcode::M2_mpyud_hl_s0 => mpy16(ctx, d, 1, 0, true, false, Acc::Set, false, false, true),
        Opcode::M2_mpyud_hl_s1 => mpy16(ctx, d, 1, 0, true, true, Acc::Set, false, false, true),
        Opcode::M2_mpyud_lh_s0 => mpy16(ctx, d, 0, 1, true, false, Acc::Set, false, false, true),
        Opcode::M2_mpyud_lh_s1 => mpy16(ctx, d, 0, 1, true, true, Acc::Set, false, false, true),
        Opcode::M2_mpyud_ll_s0 => mpy16(ctx, d, 0, 0, true, false, Acc::Set, false, false, true),
        Opcode::M2_mpyud_ll_s1 => mpy16(ctx, d, 0, 0, true, true, Acc::Set, false, false, true),
        Opcode::M2_mpyud_nac_hh_s0 => {
            mpy16(ctx, d, 1, 1, true, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyud_nac_hh_s1 => mpy16(ctx, d, 1, 1, true, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyud_nac_hl_s0 => {
            mpy16(ctx, d, 1, 0, true, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyud_nac_hl_s1 => mpy16(ctx, d, 1, 0, true, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyud_nac_lh_s0 => {
            mpy16(ctx, d, 0, 1, true, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyud_nac_lh_s1 => mpy16(ctx, d, 0, 1, true, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyud_nac_ll_s0 => {
            mpy16(ctx, d, 0, 0, true, false, Acc::Sub, false, false, true)
        }
        Opcode::M2_mpyud_nac_ll_s1 => mpy16(ctx, d, 0, 0, true, true, Acc::Sub, false, false, true),
        Opcode::M2_mpyu_hh_s0 => mpy16(ctx, d, 1, 1, true, false, Acc::Set, false, false, false),
        Opcode::M2_mpyu_hh_s1 => mpy16(ctx, d, 1, 1, true, true, Acc::Set, false, false, false),
        Opcode::M2_mpyu_hl_s0 => mpy16(ctx, d, 1, 0, true, false, Acc::Set, false, false, false),
        Opcode::M2_mpyu_hl_s1 => mpy16(ctx, d, 1, 0, true, true, Acc::Set, false, false, false),
        Opcode::M2_mpyu_lh_s0 => mpy16(ctx, d, 0, 1, true, false, Acc::Set, false, false, false),
        Opcode::M2_mpyu_lh_s1 => mpy16(ctx, d, 0, 1, true, true, Acc::Set, false, false, false),
        Opcode::M2_mpyu_ll_s0 => mpy16(ctx, d, 0, 0, true, false, Acc::Set, false, false, false),
        Opcode::M2_mpyu_ll_s1 => mpy16(ctx, d, 0, 0, true, true, Acc::Set, false, false, false),
        Opcode::M2_mpyu_nac_hh_s0 => {
            mpy16(ctx, d, 1, 1, true, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpyu_nac_hh_s1 => mpy16(ctx, d, 1, 1, true, true, Acc::Sub, false, false, false),
        Opcode::M2_mpyu_nac_hl_s0 => {
            mpy16(ctx, d, 1, 0, true, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpyu_nac_hl_s1 => mpy16(ctx, d, 1, 0, true, true, Acc::Sub, false, false, false),
        Opcode::M2_mpyu_nac_lh_s0 => {
            mpy16(ctx, d, 0, 1, true, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpyu_nac_lh_s1 => mpy16(ctx, d, 0, 1, true, true, Acc::Sub, false, false, false),
        Opcode::M2_mpyu_nac_ll_s0 => {
            mpy16(ctx, d, 0, 0, true, false, Acc::Sub, false, false, false)
        }
        Opcode::M2_mpyu_nac_ll_s1 => mpy16(ctx, d, 0, 0, true, true, Acc::Sub, false, false, false),

        // ============ hmmpy: 32 x signed-16 high-half multiply (:<<1[:rnd]:sat) ============
        // RdV = fSAT( (fSCALE(1, fMPY3216SS(RsV, Rt.{H|L})) [+0x8000]) >> 16 )
        Opcode::M2_hmmpyh_s1 => {
            let rs = ctx.r(fld(d, b's')) as i32 as i64;
            let rt = get_half(ctx.r(fld(d, b't')), 1);
            let v = ctx.sat_n((rs.wrapping_mul(rt) << 1) >> 16, 32);
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_hmmpyl_s1 => {
            let rs = ctx.r(fld(d, b's')) as i32 as i64;
            let rt = get_half(ctx.r(fld(d, b't')), 0);
            let v = ctx.sat_n((rs.wrapping_mul(rt) << 1) >> 16, 32);
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_hmmpyh_rs1 => {
            let rs = ctx.r(fld(d, b's')) as i32 as i64;
            let rt = get_half(ctx.r(fld(d, b't')), 1);
            let v = ctx.sat_n(((rs.wrapping_mul(rt) << 1) + 0x8000) >> 16, 32);
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_hmmpyl_rs1 => {
            let rs = ctx.r(fld(d, b's')) as i32 as i64;
            let rt = get_half(ctx.r(fld(d, b't')), 0);
            let v = ctx.sat_n(((rs.wrapping_mul(rt) << 1) + 0x8000) >> 16, 32);
            ctx.set_r(rd, v as u32);
        }

        // ============ M7 complex multiply real/imaginary 32-bit (V73 audio) ====
        // CMPY64: Rdd = (Rss.w[w0]*Rtt.w[w1]) OP (Rss.w[w2]*Rtt.w[w3]); _acc: Rxx += .
        //   dcmpyrw  Real    (-,0,0,1,1)   dcmpyrwc Real conj (+,0,0,1,1)
        //   dcmpyiw  Imag    (+,0,1,1,0)   dcmpyiwc Imag conj (-,1,0,0,1)
        Opcode::M7_dcmpyrw => m7_dcmpy(ctx, d, false, 0, 0, 1, 1, false),
        Opcode::M7_dcmpyrwc => m7_dcmpy(ctx, d, true, 0, 0, 1, 1, false),
        Opcode::M7_dcmpyiw => m7_dcmpy(ctx, d, true, 0, 1, 1, 0, false),
        Opcode::M7_dcmpyiwc => m7_dcmpy(ctx, d, false, 1, 0, 0, 1, false),
        Opcode::M7_dcmpyrw_acc => m7_dcmpy(ctx, d, false, 0, 0, 1, 1, true),
        Opcode::M7_dcmpyrwc_acc => m7_dcmpy(ctx, d, true, 0, 0, 1, 1, true),
        Opcode::M7_dcmpyiw_acc => m7_dcmpy(ctx, d, true, 0, 1, 1, 0, true),
        Opcode::M7_dcmpyiwc_acc => m7_dcmpy(ctx, d, false, 1, 0, 0, 1, true),

        // CMPY128/CMPY128RND: tmp=Rss.w[w0]*Rtt.w[w1]; acc=Rss.w[w2]*Rtt.w[w3];
        //   acc = OP(tmp,acc) [+0x40000000 when :rnd]; acc>>=31; Rd = sat32(acc).
        //   wcmpyrw  Real    (SUB,0,0,1,1)   wcmpyrwc Real conj (ADD,0,0,1,1)
        //   wcmpyiw  Imag    (ADD,0,1,1,0)   wcmpyiwc Imag conj (SUB,1,0,0,1)
        // OP here is the macro's first-arg-relative op: fADD128 -> add=true.
        Opcode::M7_wcmpyrw => m7_wcmpy(ctx, d, false, 0, 0, 1, 1, false),
        Opcode::M7_wcmpyrwc => m7_wcmpy(ctx, d, true, 0, 0, 1, 1, false),
        Opcode::M7_wcmpyiw => m7_wcmpy(ctx, d, true, 0, 1, 1, 0, false),
        Opcode::M7_wcmpyiwc => m7_wcmpy(ctx, d, false, 1, 0, 0, 1, false),
        Opcode::M7_wcmpyrw_rnd => m7_wcmpy(ctx, d, false, 0, 0, 1, 1, true),
        Opcode::M7_wcmpyrwc_rnd => m7_wcmpy(ctx, d, true, 0, 0, 1, 1, true),
        Opcode::M7_wcmpyiw_rnd => m7_wcmpy(ctx, d, true, 0, 1, 1, 0, true),
        Opcode::M7_wcmpyiwc_rnd => m7_wcmpy(ctx, d, false, 1, 0, 0, 1, true),

        // ============ vabsdiff: per-byte |Rtt[i] - Rss[i]| (M6) ============
        // Note: operands are (Rtt, Rss) — the difference is Rtt-byte minus Rss-byte.
        Opcode::M6_vabsdiffb => {
            let rss = ctx.rp(fld(d, b's'));
            let rtt = ctx.rp(fld(d, b't'));
            let mut v: u64 = 0;
            for i in 0..8 {
                v = set_byte(v, i, (get_byte(rtt, i) - get_byte(rss, i)).abs());
            }
            ctx.set_rp(rd, v);
        }
        Opcode::M6_vabsdiffub => {
            let rss = ctx.rp(fld(d, b's'));
            let rtt = ctx.rp(fld(d, b't'));
            let mut v: u64 = 0;
            for i in 0..8 {
                v = set_byte(v, i, (get_ubyte(rtt, i) - get_ubyte(rss, i)).abs());
            }
            ctx.set_rp(rd, v);
        }

        _ => return false,
    }
    true
}

/// `M7_dcmpy*` — 64-bit complex multiply (optionally accumulating). The `Rdd`
/// (or `Rxx +=`) result wraps in 64 bits, exactly matching the idef's int64.
#[allow(clippy::too_many_arguments)]
#[inline]
fn m7_dcmpy(
    ctx: &mut SemCtx,
    d: &DecodedOp,
    add: bool,
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
    acc: bool,
) {
    let rss = ctx.rp(fld(d, b's'));
    let rtt = ctx.rp(fld(d, b't'));
    let prod = cmpy_terms(rss, rtt, w0, w1, w2, w3, add) as i64;
    if acc {
        let rx = fld(d, b'x');
        let v = (ctx.rp(rx) as i64).wrapping_add(prod) as u64;
        ctx.set_rp(rx, v);
    } else {
        let v = prod as u64;
        ctx.set_rp(fld(d, b'd'), v);
    }
}

/// `M7_wcmpy*` — 32-bit complex multiply with `:<<1` scale and signed-32
/// saturation (optionally `:rnd`). Mirrors the `CMPY128`/`CMPY128RND` macros:
/// the 128-bit accumulator is shifted right by 31 (the `<<1` then `>>32`) before
/// saturating to a word; `:rnd` adds 0x40000000 before the shift.
#[allow(clippy::too_many_arguments)]
#[inline]
fn m7_wcmpy(
    ctx: &mut SemCtx,
    d: &DecodedOp,
    add: bool,
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
    rnd: bool,
) {
    let rss = ctx.rp(fld(d, b's'));
    let rtt = ctx.rp(fld(d, b't'));
    let mut acc = cmpy_terms(rss, rtt, w0, w1, w2, w3, add);
    if rnd {
        acc += 0x4000_0000i128;
    }
    let shifted = acc >> 31; // arithmetic shift on the signed 128-bit accumulator
    let v = ctx.sat_n(shifted as i64, 32);
    ctx.set_r(fld(d, b'd'), v as u32);
}
