//! Bit-manipulation instructions (`S2_*`, `S4_*`, `S5_*`, `S6_*`, `A4_bitsplit*`,
//! `A2_swiz`): set/clear/toggle/test bit, extract/insert, mask, bit counting,
//! popcount/parity, bit-reverse, splat, packhl, bitsplit, swiz. Semantics taken
//! verbatim from the Hexagon V68 spec (`semantics_generated.pyinc`), with the
//! f-macros (`fEXTRACTU`/`fINSERT`/`fSXTN`/`fZXTN`/`fBIDIR_*`/`fCL1_*`/`fBREV_*`)
//! expanded from `imported/macros.def`.
//!
//! See `sem/alu.rs` for the established style. Verified against `qemu-hexagon`.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_u, fld};

// --- macro helpers ----------------------------------------------------------

/// `fZXTN(n, _, val)`: zero-extend the low `n` bits. `n == 0` yields 0; `n >= 64`
/// is the identity (the masking would overflow).
#[inline]
fn zxtn(n: u32, val: u64) -> u64 {
    if n == 0 {
        0
    } else if n >= 64 {
        val
    } else {
        val & ((1u64 << n) - 1)
    }
}

/// `fSXTN(n, _, val)`: sign-extend the low `n` bits to 64 bits. `n == 0` -> 0,
/// `n >= 64` is the identity.
#[inline]
fn sxtn(n: u32, val: u64) -> i64 {
    if n == 0 {
        0
    } else if n >= 64 {
        val as i64
    } else {
        let z = val & ((1u64 << n) - 1);
        ((z ^ (1u64 << (n - 1))) as i64) - (1i64 << (n - 1))
    }
}

/// `fBIDIR_LSHIFTL(1, shamt, ...)`: bidirectional left shift of value 1.
/// shamt >= 0: `1 << shamt`; shamt < 0: `(1 >> (-shamt - 1)) >> 1`.
#[inline]
fn bidir_lshiftl_one(shamt: i64) -> u64 {
    if shamt < 0 {
        let s = (-shamt) - 1;
        // (1u64 >> s) >> 1 ; any positive shift of 1 yields 0
        if s >= 63 { 0 } else { (1u64 >> s) >> 1 }
    } else if shamt >= 64 {
        0
    } else {
        1u64 << shamt
    }
}

/// `fBIDIR_LSHIFTR(src, shamt, ...)`: bidirectional right shift.
/// shamt >= 0: `src >> shamt`; shamt < 0: `(src << (-shamt - 1)) << 1`.
#[inline]
fn bidir_lshiftr(src: u64, shamt: i64) -> u64 {
    if shamt < 0 {
        let s = (-shamt) - 1;
        if s >= 63 {
            0
        } else {
            (src << s).wrapping_shl(1)
        }
    } else if shamt >= 64 {
        0
    } else {
        src >> shamt
    }
}

/// Signed 7-bit shift amount taken from `Rt` (`fSXTN(7, 32, Rt)`).
#[inline]
fn shift_amt7(rt: u32) -> i64 {
    sxtn(7, rt as u64)
}

/// `count_leading_ones` over the top `bits` of `val`.
#[inline]
fn cl1(val: u64, bits: u32) -> u32 {
    let mut n = 0u32;
    for i in (0..bits).rev() {
        if (val >> i) & 1 == 1 {
            n += 1;
        } else {
            break;
        }
    }
    n
}

/// `reverse_bits` over the low `bits` of `val`.
#[inline]
fn brev(val: u64, bits: u32) -> u64 {
    let mut out = 0u64;
    for i in 0..bits {
        if (val >> i) & 1 == 1 {
            out |= 1u64 << (bits - 1 - i);
        }
    }
    out
}

/// `fGETWORD(n, src)`: extract 32-bit word `n` from a 64-bit value (sign-relevant
/// for callers that interpret it as signed; we return the raw bits).
#[inline]
fn getword(n: u32, src: u64) -> u32 {
    (src >> (n * 32)) as u32
}

/// `fGETBYTE(n, src)`: signed byte `n`.
#[inline]
fn getbyte(n: u32, src: u32) -> i32 {
    ((src >> (n * 8)) & 0xff) as i8 as i32
}

/// `fGETHALF(n, src)`: signed halfword `n`.
#[inline]
fn gethalf(n: u32, src: u32) -> i32 {
    ((src >> (n * 16)) & 0xffff) as i16 as i32
}

/// `fSETBYTE(n, dst, val)`.
#[inline]
fn setbyte(n: u32, dst: u64, val: u32) -> u64 {
    let m = 0xffu64 << (n * 8);
    (dst & !m) | (((val as u64) & 0xff) << (n * 8))
}

/// `fSETHALF(n, dst, val)`.
#[inline]
fn sethalf(n: u32, dst: u64, val: u32) -> u64 {
    let m = 0xffffu64 << (n * 16);
    (dst & !m) | (((val as u64) & 0xffff) << (n * 16))
}

/// `fSETWORD(n, dst, val)`.
#[inline]
fn setword(n: u32, dst: u64, val: u32) -> u64 {
    let m = 0xffff_ffffu64 << (n * 32);
    (dst & !m) | (((val as u64) & 0xffff_ffff) << (n * 32))
}

// --- exec --------------------------------------------------------------------

/// Execute a bitmanip-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let rd = fld(d, b'd');
    // Width is field `i` (#u5/#u6), offset is field `I` (#U5/#U6). Unextended.
    let ui = |_c: &SemCtx| fimm_u(d, b'i', None);
    let big_ui = |_c: &SemCtx| fimm_u(d, b'I', None);

    match op {
        // ---- set / clear / toggle bit (immediate, write GPR) ----
        Opcode::S2_setbit_i => {
            let v = s(ctx) | (1u32 << ui(ctx));
            ctx.set_r(rd, v);
        }
        Opcode::S2_clrbit_i => {
            let v = s(ctx) & !(1u32 << ui(ctx));
            ctx.set_r(rd, v);
        }
        Opcode::S2_togglebit_i => {
            let v = s(ctx) ^ (1u32 << ui(ctx));
            ctx.set_r(rd, v);
        }

        // ---- set / clear / toggle bit (register, write GPR) ----
        Opcode::S2_setbit_r => {
            let bit = bidir_lshiftl_one(shift_amt7(t(ctx))) as u32;
            let v = s(ctx) | bit;
            ctx.set_r(rd, v);
        }
        Opcode::S2_clrbit_r => {
            let bit = bidir_lshiftl_one(shift_amt7(t(ctx))) as u32;
            let v = s(ctx) & !bit;
            ctx.set_r(rd, v);
        }
        Opcode::S2_togglebit_r => {
            let bit = bidir_lshiftl_one(shift_amt7(t(ctx))) as u32;
            let v = s(ctx) ^ bit;
            ctx.set_r(rd, v);
        }

        // ---- test bit (immediate / register, write PREDICATE) ----
        Opcode::S2_tstbit_i => {
            let set = (s(ctx) & (1u32 << ui(ctx))) != 0;
            ctx.set_p(rd, if set { 0xff } else { 0x00 });
        }
        Opcode::S4_ntstbit_i => {
            let clear = (s(ctx) & (1u32 << ui(ctx))) == 0;
            ctx.set_p(rd, if clear { 0xff } else { 0x00 });
        }
        Opcode::S2_tstbit_r => {
            // fCAST4_8u(Rs) & fBIDIR_LSHIFTL(1, sxtn7(Rt), 4_8) — done in 64-bit.
            let bit = bidir_lshiftl_one(shift_amt7(t(ctx)));
            let set = ((s(ctx) as u64) & bit) != 0;
            ctx.set_p(rd, if set { 0xff } else { 0x00 });
        }
        Opcode::S4_ntstbit_r => {
            let bit = bidir_lshiftl_one(shift_amt7(t(ctx)));
            let clear = ((s(ctx) as u64) & bit) == 0;
            ctx.set_p(rd, if clear { 0xff } else { 0x00 });
        }

        // ---- extractu / extract (immediate) ----
        Opcode::S2_extractu => {
            // RdV = fZXTN(width, 32, (u32)Rs >> offset)
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let shifted = (s(ctx) >> offset) as u64;
            let v = zxtn(width, shifted) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::S2_extractup => {
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let shifted = sp(ctx) >> offset;
            let v = zxtn(width, shifted);
            ctx.set_rp(rd, v);
        }
        Opcode::S4_extract => {
            // RdV = fSXTN(width, 32, (u32)Rs >> offset)
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let shifted = (s(ctx) >> offset) as u64;
            let v = sxtn(width, shifted) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::S4_extractp => {
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let shifted = sp(ctx) >> offset;
            let v = sxtn(width, shifted) as u64;
            ctx.set_rp(rd, v);
        }

        // ---- extractu / extract (register-pair width:offset) ----
        Opcode::S2_extractu_rp => {
            let width = zxtn(6, getword(1, tp(ctx)) as u64) as u32;
            let offset = sxtn(7, getword(0, tp(ctx)) as u64);
            let shifted = bidir_lshiftr(s(ctx) as u64, offset);
            let v = zxtn(width, shifted) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::S2_extractup_rp => {
            let width = zxtn(6, getword(1, tp(ctx)) as u64) as u32;
            let offset = sxtn(7, getword(0, tp(ctx)) as u64);
            let shifted = bidir_lshiftr(sp(ctx), offset);
            let v = zxtn(width, shifted);
            ctx.set_rp(rd, v);
        }
        Opcode::S4_extract_rp => {
            let width = zxtn(6, getword(1, tp(ctx)) as u64) as u32;
            let offset = sxtn(7, getword(0, tp(ctx)) as u64);
            let shifted = bidir_lshiftr(s(ctx) as u64, offset);
            let v = sxtn(width, shifted) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::S4_extractp_rp => {
            let width = zxtn(6, getword(1, tp(ctx)) as u64) as u32;
            let offset = sxtn(7, getword(0, tp(ctx)) as u64);
            let shifted = bidir_lshiftr(sp(ctx), offset);
            let v = sxtn(width, shifted) as u64;
            ctx.set_rp(rd, v);
        }

        // ---- insert (immediate) ----
        Opcode::S2_insert => {
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let mask = if width >= 64 {
                u64::MAX
            } else {
                (1u64 << width) - 1
            };
            let mut x = ctx.r(fld(d, b'x')) as u64;
            x &= !(mask << offset);
            x |= ((s(ctx) as u64) & mask) << offset;
            ctx.set_r(fld(d, b'x'), x as u32);
        }
        Opcode::S2_insertp => {
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let mask = if width >= 64 {
                u64::MAX
            } else {
                (1u64 << width) - 1
            };
            let mut x = ctx.rp(fld(d, b'x'));
            x &= !(mask << offset);
            x |= (sp(ctx) & mask) << offset;
            ctx.set_rp(fld(d, b'x'), x);
        }

        // ---- insert (register-pair width:offset) ----
        Opcode::S2_insert_rp => {
            let width = zxtn(6, getword(1, tp(ctx)) as u64) as u32;
            let offset = sxtn(7, getword(0, tp(ctx)) as u64);
            let mask = if width >= 64 {
                u64::MAX
            } else {
                (1u64 << width) - 1
            };
            if offset < 0 {
                ctx.set_r(fld(d, b'x'), 0);
            } else {
                let off = offset as u32;
                let mut x = ctx.r(fld(d, b'x')) as u64;
                x &= !(mask << off);
                x |= ((s(ctx) as u64) & mask) << off;
                ctx.set_r(fld(d, b'x'), x as u32);
            }
        }
        Opcode::S2_insertp_rp => {
            let width = zxtn(6, getword(1, tp(ctx)) as u64) as u32;
            let offset = sxtn(7, getword(0, tp(ctx)) as u64);
            let mask = if width >= 64 {
                u64::MAX
            } else {
                (1u64 << width) - 1
            };
            if offset < 0 {
                ctx.set_rp(fld(d, b'x'), 0);
            } else {
                let off = offset as u32;
                let mut x = ctx.rp(fld(d, b'x'));
                x &= !(mask << off);
                x |= (sp(ctx) & mask) << off;
                ctx.set_rp(fld(d, b'x'), x);
            }
        }

        // ---- mask ----
        Opcode::S2_mask => {
            // RdV = ((1<<width)-1) << offset ; width=#u5(i), offset=#U5(I)
            let width = ui(ctx);
            let offset = big_ui(ctx);
            let m = ((1u32 << width) - 1) << offset;
            ctx.set_r(rd, m);
        }

        // ---- count leading bits ----
        Opcode::S2_cl1 => {
            ctx.set_r(rd, cl1(s(ctx) as u64, 32));
        }
        Opcode::S2_cl0 => {
            ctx.set_r(rd, cl1(!s(ctx) as u64, 32));
        }
        Opcode::S2_clb => {
            let v = cl1(s(ctx) as u64, 32).max(cl1(!s(ctx) as u64, 32));
            ctx.set_r(rd, v);
        }
        Opcode::S2_cl1p => {
            ctx.set_r(rd, cl1(sp(ctx), 64));
        }
        Opcode::S2_cl0p => {
            ctx.set_r(rd, cl1(!sp(ctx), 64));
        }
        Opcode::S2_clbp => {
            let v = cl1(sp(ctx), 64).max(cl1(!sp(ctx), 64));
            ctx.set_r(rd, v);
        }
        Opcode::S2_clbnorm => {
            let rs = s(ctx);
            let v = if rs == 0 {
                0
            } else {
                cl1(rs as u64, 32).max(cl1(!rs as u64, 32)) - 1
            };
            ctx.set_r(rd, v);
        }

        // ---- count trailing bits (via bit-reverse + count-leading) ----
        Opcode::S2_ct1 => {
            ctx.set_r(rd, cl1(brev(s(ctx) as u64, 32), 32));
        }
        Opcode::S2_ct0 => {
            ctx.set_r(rd, cl1(brev(!s(ctx) as u64, 32), 32));
        }
        Opcode::S2_ct1p => {
            ctx.set_r(rd, cl1(brev(sp(ctx), 64), 64));
        }
        Opcode::S2_ct0p => {
            ctx.set_r(rd, cl1(brev(!sp(ctx), 64), 64));
        }

        // ---- popcount / parity ----
        Opcode::S5_popcountp => {
            ctx.set_r(rd, sp(ctx).count_ones());
        }
        Opcode::S4_parity => {
            let v = 1u32 & (s(ctx) & t(ctx)).count_ones();
            ctx.set_r(rd, v);
        }
        Opcode::S2_parityp => {
            let v = 1u32 & (sp(ctx) & tp(ctx)).count_ones();
            ctx.set_r(rd, v);
        }

        // ---- bit reverse ----
        Opcode::S2_brev => {
            ctx.set_r(rd, brev(s(ctx) as u64, 32) as u32);
        }
        Opcode::S2_brevp => {
            ctx.set_rp(rd, brev(sp(ctx), 64));
        }

        // ---- splat ----
        Opcode::S2_vsplatrb => {
            let b = (s(ctx) & 0xff) as u32;
            let mut out = 0u64;
            for i in 0..4 {
                out = setbyte(i, out, b);
            }
            ctx.set_r(rd, out as u32);
        }
        Opcode::S2_vsplatrh => {
            let h = (s(ctx) & 0xffff) as u32;
            let mut out = 0u64;
            for i in 0..4 {
                out = sethalf(i, out, h);
            }
            ctx.set_rp(rd, out);
        }
        Opcode::S6_vsplatrbp => {
            let b = (s(ctx) & 0xff) as u32;
            let mut out = 0u64;
            for i in 0..8 {
                out = setbyte(i, out, b);
            }
            ctx.set_rp(rd, out);
        }

        // ---- packhl ----
        Opcode::S2_packhl => {
            let rs = s(ctx);
            let rt = t(ctx);
            let mut out = 0u64;
            out = sethalf(0, out, gethalf(0, rt) as u32);
            out = sethalf(1, out, gethalf(0, rs) as u32);
            out = sethalf(2, out, gethalf(1, rt) as u32);
            out = sethalf(3, out, gethalf(1, rs) as u32);
            ctx.set_rp(rd, out);
        }

        // ---- bitsplit ----
        Opcode::A4_bitspliti => {
            let rs = s(ctx);
            let sh = ui(ctx);
            let mut out = 0u64;
            out = setword(1, out, rs >> sh);
            out = setword(0, out, zxtn(sh, rs as u64) as u32);
            ctx.set_rp(rd, out);
        }
        Opcode::A4_bitsplit => {
            let rs = s(ctx);
            let sh = zxtn(5, t(ctx) as u64) as u32;
            let mut out = 0u64;
            out = setword(1, out, rs >> sh);
            out = setword(0, out, zxtn(sh, rs as u64) as u32);
            ctx.set_rp(rd, out);
        }

        // ---- swiz (byte reverse) ----
        Opcode::A2_swiz => {
            let rs = s(ctx);
            let mut out = 0u64;
            out = setbyte(0, out, getbyte(3, rs) as u32);
            out = setbyte(1, out, getbyte(2, rs) as u32);
            out = setbyte(2, out, getbyte(1, rs) as u32);
            out = setbyte(3, out, getbyte(0, rs) as u32);
            ctx.set_r(rd, out as u32);
        }

        _ => return false,
    }
    true
}
