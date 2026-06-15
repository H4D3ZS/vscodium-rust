//! (alu_ext) Hexagon instruction gap-fill — direct opcode-dispatch handlers
//! for the remaining `A2_*`/`A4_*`/`A5_*`/`A6_*` ALU instructions: halfword
//! add/sub with placement (`addh_*`/`subh_*`), pair add with raw word
//! (`addsph`/`addspl`), half-register transfer immediate (`tfrih`/`tfril`),
//! vector lane compares into predicates (`vcmp{b,h,w}*`, `vcmpbeq_any`,
//! `vcmpbeq_notany`), sum-of-absolute-differences (`vrsadub`), vector reduce
//! max/min with index (`vrmax*`/`vrmin*`), bounds check, modwrap, convergent
//! rounding (`cround`), add/sub-with-carry-predicate (`addp_c`/`subp_c`),
//! predicated shift/extend (`paslh`/`pasrh`/`psxt*`/`pzxt*`), `vaddhubs`, and
//! `vminub` with predicate output.
//!
//! Semantics are taken verbatim from the Hexagon V68 spec
//! (`semantics_generated.pyinc`) and the f-macros in `imported/macros.def`,
//! verified bit-exact against the qemu-hexagon oracle (`tests/hexagon_diff.rs`).
//! See `sem/alu.rs` and `sem/mpy.rs` for the established pattern.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

// --- macro-equivalent lane helpers (mirror imported/macros.def) -------------
//   fGETHALF(N,SRC)  = (size2s_t)((SRC>>(N*16))&0xffff)   signed 16
//   fGETUHALF(N,SRC) = (size2u_t)((SRC>>(N*16))&0xffff)   unsigned 16
//   fGETBYTE(N,SRC)  = (size1s_t)((SRC>>(N*8))&0xff)      signed 8
//   fGETUBYTE(N,SRC) = (size1u_t)((SRC>>(N*8))&0xff)      unsigned 8
//   fGETWORD(N,SRC)  = (size4s_t)((SRC>>(N*32))&0xffffffff) sign-ext to 64
//   fGETUWORD(N,SRC) = (size4u_t)((SRC>>(N*32))&0xffffffff) zero-ext to 64
//   fSETHALF/fSETWORD/fSETBYTE insert a lane (masking to width).

#[inline]
fn get_half(src: u64, n: u32) -> i64 {
    (((src >> (n * 16)) & 0xffff) as u16 as i16) as i64
}
#[inline]
fn get_uhalf(src: u64, n: u32) -> i64 {
    ((src >> (n * 16)) & 0xffff) as i64
}
#[inline]
fn get_byte(src: u64, n: u32) -> i64 {
    (((src >> (n * 8)) & 0xff) as u8 as i8) as i64
}
#[inline]
fn get_ubyte(src: u64, n: u32) -> i64 {
    ((src >> (n * 8)) & 0xff) as i64
}
/// fGETWORD: signed 32 -> 64.
#[inline]
fn get_word(src: u64, n: u32) -> i64 {
    ((src >> (n * 32)) as u32 as i32) as i64
}
/// fGETUWORD: unsigned 32 -> 64.
#[inline]
fn get_uword(src: u64, n: u32) -> u64 {
    ((src >> (n * 32)) as u32) as u64
}
#[inline]
fn set_half(dst: u64, n: u32, val: i64) -> u64 {
    let sh = n * 16;
    (dst & !(0xffffu64 << sh)) | (((val as u64) & 0xffff) << sh)
}
#[inline]
fn set_word(dst: u64, n: u32, val: i64) -> u64 {
    let sh = n * 32;
    (dst & !(0xffff_ffffu64 << sh)) | (((val as u64) & 0xffff_ffff) << sh)
}
#[inline]
fn set_byte(dst: u64, n: u32, val: i64) -> u64 {
    let sh = n * 8;
    (dst & !(0xffu64 << sh)) | (((val as u64) & 0xff) << sh)
}
/// f8BITSOF(VAL) = VAL ? 0xff : 0x00.
#[inline]
fn bits8of(cond: bool) -> u8 {
    if cond { 0xff } else { 0x00 }
}

/// `conv_round(a, n)` — convergent (".5 rounds toward even") rounding to a
/// 2^n boundary, exactly as the Hexagon PRM `convround(Rs, 2**(n-1)) >> n`.
/// `a` is a signed 32-bit value sign-extended to 64 bits.
#[inline]
fn conv_round(a: u32, n: u32) -> u32 {
    let src = a as i32 as i64;
    if n == 0 {
        return a;
    }
    // Round bias: a full half-bit normally, but at an exact tie (the low n-1
    // bits are zero) only round up when bit n is set — i.e. toward even.
    let rndbit: i64 = if (a & ((1u32 << (n - 1)).wrapping_sub(1))) == 0 {
        ((1i64 << n) & src) >> 1
    } else {
        1i64 << (n - 1)
    };
    ((src + rndbit) >> n) as u32
}

/// `fCLIP(DST,SRC,U)` — clamp `src` to the signed range `[-(1<<U), (1<<U)-1]`.
/// The bounds are computed as C `size4s_t`, so `U==31` wraps exactly like the
/// idef (`1<<31` overflows to `i32::MIN`, giving `maxv=0x7fffffff`,
/// `minv=i32::MIN`). No saturation flag is raised — this is a plain clamp.
#[inline]
fn clip32(src: i32, u: u32) -> i32 {
    let maxv = (1i32.wrapping_shl(u)).wrapping_sub(1);
    let minv = (1i32.wrapping_shl(u)).wrapping_neg();
    src.max(minv).min(maxv)
}

/// 64-bit convergent rounding (`CROUND` macro). Rounds the signed 64-bit `src`
/// to a 2^n boundary, biasing toward even at an exact tie (low `n-1` bits zero).
#[inline]
fn conv_round64(src: i64, n: u32) -> i64 {
    if n == 0 {
        return src;
    }
    // i128 keeps full precision through the +rndbit and >>n (matches fSHIFTR128).
    let s = src as i128;
    let rndbit: i128 = if (src & ((1i64 << (n - 1)) - 1)) == 0 {
        // Tie: round up only when bit n of src is set (toward even).
        ((1i128 << n) & s) >> 1
    } else {
        1i128 << (n - 1)
    };
    ((s + rndbit) >> n) as i64
}

/// Execute an `alu_ext`-class opcode. Returns `false` if `op` is not handled.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    // Scalar/pair field readers. Behavior strings reference RsV/RtV/RssV/RttV;
    // these map to the decoded `s`/`t` fields (mirroring sem/alu.rs, oracle-verified).
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let u = |c: &SemCtx| c.r(fld(d, b'u'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let rd = fld(d, b'd');
    let rx = fld(d, b'x');

    match op {
        // ============ halfword add with placement (A2_addh_*) ============
        // l16: RdV = fSXTN(16,32, half(Rt) + half(Rs))   (result in low half, sign-ext)
        Opcode::A2_addh_l16_ll => {
            let v =
                ((get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 0)) as i16 as i32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_l16_hl => {
            let v =
                ((get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 1)) as i16 as i32) as u32;
            ctx.set_r(rd, v);
        }
        // l16:sat: RdV = fSATH(half(Rt) + half(Rs))
        Opcode::A2_addh_l16_sat_ll => {
            let v = ctx.sat_n(get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 0), 16) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_l16_sat_hl => {
            let v = ctx.sat_n(get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 1), 16) as u32;
            ctx.set_r(rd, v);
        }
        // h16: RdV = (half(Rt) + half(Rs)) << 16   (result placed in high half)
        Opcode::A2_addh_h16_ll => {
            let v = ((get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 0)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_h16_lh => {
            let v = ((get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 1)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_h16_hl => {
            let v = ((get_half(t(ctx) as u64, 1) + get_half(s(ctx) as u64, 0)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_h16_hh => {
            let v = ((get_half(t(ctx) as u64, 1) + get_half(s(ctx) as u64, 1)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        // h16:sat: RdV = fSATH(half(Rt) + half(Rs)) << 16
        Opcode::A2_addh_h16_sat_ll => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 0), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_h16_sat_lh => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 0) + get_half(s(ctx) as u64, 1), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_h16_sat_hl => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 1) + get_half(s(ctx) as u64, 0), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_addh_h16_sat_hh => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 1) + get_half(s(ctx) as u64, 1), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }

        // ============ halfword sub with placement (A2_subh_*) ============
        // l16: RdV = fSXTN(16,32, half(Rt) - half(Rs))
        Opcode::A2_subh_l16_ll => {
            let v =
                ((get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 0)) as i16 as i32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_l16_hl => {
            let v =
                ((get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 1)) as i16 as i32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_l16_sat_ll => {
            let v = ctx.sat_n(get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 0), 16) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_l16_sat_hl => {
            let v = ctx.sat_n(get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 1), 16) as u32;
            ctx.set_r(rd, v);
        }
        // h16: RdV = (half(Rt) - half(Rs)) << 16
        Opcode::A2_subh_h16_ll => {
            let v = ((get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 0)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_lh => {
            let v = ((get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 1)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_hl => {
            let v = ((get_half(t(ctx) as u64, 1) - get_half(s(ctx) as u64, 0)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_hh => {
            let v = ((get_half(t(ctx) as u64, 1) - get_half(s(ctx) as u64, 1)) as u32) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_sat_ll => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 0), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_sat_lh => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 0) - get_half(s(ctx) as u64, 1), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_sat_hl => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 1) - get_half(s(ctx) as u64, 0), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subh_h16_sat_hh => {
            let v = (ctx.sat_n(get_half(t(ctx) as u64, 1) - get_half(s(ctx) as u64, 1), 16) as u32)
                << 16;
            ctx.set_r(rd, v);
        }

        // ============ pair add with raw word (A2_addsph/addspl) ============
        // Rdd = Rtt + fSXTN(32,64, word(N, Rss))
        Opcode::A2_addsph => {
            let v = (tp(ctx) as i64).wrapping_add(get_word(sp(ctx), 1)) as u64;
            ctx.set_rp(rd, v);
        }
        Opcode::A2_addspl => {
            let v = (tp(ctx) as i64).wrapping_add(get_word(sp(ctx), 0)) as u64;
            ctx.set_rp(rd, v);
        }

        // ============ transfer immediate into half (A2_tfrih/tfril) ============
        // Rx.H32=#u16 -> fSETHALF(1,Rx,u16); Rx.L32=#u16 -> fSETHALF(0,Rx,u16).
        Opcode::A2_tfrih => {
            let imm = fimm_u(d, b'i', None) as i64;
            let v = set_half(ctx.r(rx) as u64, 1, imm);
            ctx.set_r(rx, v as u32);
        }
        Opcode::A2_tfril => {
            let imm = fimm_u(d, b'i', None) as i64;
            let v = set_half(ctx.r(rx) as u64, 0, imm);
            ctx.set_r(rx, v as u32);
        }

        // ============ vector byte/half/word compares -> predicate ============
        // Byte: 8 lanes, 1 bit each.
        Opcode::A2_vcmpbeq => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            for i in 0..8 {
                if get_byte(a, i) == get_byte(b, i) {
                    p |= 1 << i;
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A2_vcmpbgtu => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            for i in 0..8 {
                if get_ubyte(a, i) > get_ubyte(b, i) {
                    p |= 1 << i;
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpbgt => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            for i in 0..8 {
                if get_byte(a, i) > get_byte(b, i) {
                    p |= 1 << i;
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpbeqi => {
            let a = sp(ctx);
            let imm = fimm_u(d, b'i', None) as i64; // #u8, compared to unsigned byte
            let mut p = 0u8;
            for i in 0..8 {
                if get_ubyte(a, i) == imm {
                    p |= 1 << i;
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpbgti => {
            let a = sp(ctx);
            let imm = fimm_s(d, b'i', None) as i64; // #s8, compared to signed byte
            let mut p = 0u8;
            for i in 0..8 {
                if get_byte(a, i) > imm {
                    p |= 1 << i;
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpbgtui => {
            let a = sp(ctx);
            let imm = fimm_u(d, b'i', None) as i64; // #u7
            let mut p = 0u8;
            for i in 0..8 {
                if get_ubyte(a, i) > imm {
                    p |= 1 << i;
                }
            }
            ctx.set_p(rd, p);
        }
        // Half: 4 lanes, 2 bits each.
        Opcode::A2_vcmpheq => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            for i in 0..4 {
                if get_half(a, i) == get_half(b, i) {
                    p |= 0b11 << (i * 2);
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A2_vcmphgt => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            for i in 0..4 {
                if get_half(a, i) > get_half(b, i) {
                    p |= 0b11 << (i * 2);
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A2_vcmphgtu => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            for i in 0..4 {
                if get_uhalf(a, i) > get_uhalf(b, i) {
                    p |= 0b11 << (i * 2);
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpheqi => {
            let a = sp(ctx);
            let imm = fimm_s(d, b'i', None) as i64; // #s8
            let mut p = 0u8;
            for i in 0..4 {
                if get_half(a, i) == imm {
                    p |= 0b11 << (i * 2);
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmphgti => {
            let a = sp(ctx);
            let imm = fimm_s(d, b'i', None) as i64; // #s8
            let mut p = 0u8;
            for i in 0..4 {
                if get_half(a, i) > imm {
                    p |= 0b11 << (i * 2);
                }
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmphgtui => {
            let a = sp(ctx);
            let imm = fimm_u(d, b'i', None) as i64; // #u7
            let mut p = 0u8;
            for i in 0..4 {
                if get_uhalf(a, i) > imm {
                    p |= 0b11 << (i * 2);
                }
            }
            ctx.set_p(rd, p);
        }
        // Word: 2 lanes, 4 bits each.
        Opcode::A2_vcmpweq => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            if get_word(a, 0) == get_word(b, 0) {
                p |= 0x0f;
            }
            if get_word(a, 1) == get_word(b, 1) {
                p |= 0xf0;
            }
            ctx.set_p(rd, p);
        }
        Opcode::A2_vcmpwgt => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            if get_word(a, 0) > get_word(b, 0) {
                p |= 0x0f;
            }
            if get_word(a, 1) > get_word(b, 1) {
                p |= 0xf0;
            }
            ctx.set_p(rd, p);
        }
        Opcode::A2_vcmpwgtu => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut p = 0u8;
            if get_uword(a, 0) > get_uword(b, 0) {
                p |= 0x0f;
            }
            if get_uword(a, 1) > get_uword(b, 1) {
                p |= 0xf0;
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpweqi => {
            let a = sp(ctx);
            let imm = fimm_s(d, b'i', None) as i64; // #s8
            let mut p = 0u8;
            if get_word(a, 0) == imm {
                p |= 0x0f;
            }
            if get_word(a, 1) == imm {
                p |= 0xf0;
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpwgti => {
            let a = sp(ctx);
            let imm = fimm_s(d, b'i', None) as i64; // #s8
            let mut p = 0u8;
            if get_word(a, 0) > imm {
                p |= 0x0f;
            }
            if get_word(a, 1) > imm {
                p |= 0xf0;
            }
            ctx.set_p(rd, p);
        }
        Opcode::A4_vcmpwgtui => {
            let a = sp(ctx);
            let imm = fimm_u(d, b'i', None) as u64; // #u7, fCAST4u
            let mut p = 0u8;
            if get_uword(a, 0) > imm {
                p |= 0x0f;
            }
            if get_uword(a, 1) > imm {
                p |= 0xf0;
            }
            ctx.set_p(rd, p);
        }
        // any8 / !any8 of byte-eq compare.
        Opcode::A4_vcmpbeq_any => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut hit = false;
            for i in 0..8 {
                if get_byte(a, i) == get_byte(b, i) {
                    hit = true;
                }
            }
            ctx.set_p(rd, if hit { 0xff } else { 0x00 });
        }
        Opcode::A6_vcmpbeq_notany => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut hit = false;
            for i in 0..8 {
                if get_byte(a, i) == get_byte(b, i) {
                    hit = true;
                }
            }
            // PdV = 0xff if any, then PdV = ~PdV.
            let v: u8 = if hit { 0xff } else { 0x00 };
            ctx.set_p(rd, !v);
        }

        // ============ sum of absolute byte differences (A2_vrsadub) ============
        Opcode::A2_vrsadub => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut w0: i64 = 0;
            let mut w1: i64 = 0;
            for i in 0..4 {
                w0 += (get_ubyte(a, i) - get_ubyte(b, i)).abs();
            }
            for i in 4..8 {
                w1 += (get_ubyte(a, i) - get_ubyte(b, i)).abs();
            }
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::A2_vrsadub_acc => {
            let (a, b) = (sp(ctx), tp(ctx));
            let acc = ctx.rp(rx);
            let mut w0 = get_word(acc, 0);
            let mut w1 = get_word(acc, 1);
            for i in 0..4 {
                w0 += (get_ubyte(a, i) - get_ubyte(b, i)).abs();
            }
            for i in 4..8 {
                w1 += (get_ubyte(a, i) - get_ubyte(b, i)).abs();
            }
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }

        // ============ vector reduce max/min with index (A4_vrmax*/vrmin*) ====
        // Rxx = vrmaxh(Rss, Ru): seed max/addr from old Rxx, scan Rss halves,
        // result word0 = winning value, word1 = address (Ru | i<<shift).
        Opcode::A4_vrmaxh => vr_minmax_half(ctx, fld(d, b's'), u(ctx), rx, true, false),
        Opcode::A4_vrmaxuh => vr_minmax_half(ctx, fld(d, b's'), u(ctx), rx, true, true),
        Opcode::A4_vrminh => vr_minmax_half(ctx, fld(d, b's'), u(ctx), rx, false, false),
        Opcode::A4_vrminuh => vr_minmax_half(ctx, fld(d, b's'), u(ctx), rx, false, true),
        Opcode::A4_vrmaxw => vr_minmax_word(ctx, fld(d, b's'), u(ctx), rx, true, false),
        Opcode::A4_vrmaxuw => vr_minmax_word(ctx, fld(d, b's'), u(ctx), rx, true, true),
        Opcode::A4_vrminw => vr_minmax_word(ctx, fld(d, b's'), u(ctx), rx, false, false),
        Opcode::A4_vrminuw => vr_minmax_word(ctx, fld(d, b's'), u(ctx), rx, false, true),

        // ============ bounds check (A4_boundscheck_hi/lo) ============
        // Pd = f8BITSOF( (src >= word0(Rtt)) && (src < word1(Rtt)) ), src unsigned.
        Opcode::A4_boundscheck_hi => {
            let src = get_uword(sp(ctx), 1);
            let lo = get_uword(tp(ctx), 0);
            let hi = get_uword(tp(ctx), 1);
            ctx.set_p(rd, bits8of(src >= lo && src < hi));
        }
        Opcode::A4_boundscheck_lo => {
            let src = get_uword(sp(ctx), 0);
            let lo = get_uword(tp(ctx), 0);
            let hi = get_uword(tp(ctx), 1);
            ctx.set_p(rd, bits8of(src >= lo && src < hi));
        }

        // ============ modulo wrap unsigned (A4_modwrapu) ============
        // if (Rs<0) Rd=Rs+(u)Rt; else if ((u)Rs>=(u)Rt) Rd=Rs-(u)Rt; else Rd=Rs.
        Opcode::A4_modwrapu => {
            let rs = s(ctx);
            let rt = t(ctx);
            let v = if (rs as i32) < 0 {
                rs.wrapping_add(rt)
            } else if rs >= rt {
                rs.wrapping_sub(rt)
            } else {
                rs
            };
            ctx.set_r(rd, v);
        }

        // ============ convergent rounding (A4_cround_ri/rr) ============
        Opcode::A4_cround_ri => {
            let n = fimm_u(d, b'i', None) & 0x1f;
            ctx.set_r(rd, conv_round(s(ctx), n));
        }
        Opcode::A4_cround_rr => {
            let n = t(ctx) & 0x1f; // fZXTN(5,32,Rt)
            ctx.set_r(rd, conv_round(s(ctx), n));
        }

        // ============ add/sub with carry predicate (A4_addp_c/subp_c) ========
        // Rdd = Rss + Rtt + P.lsb; P = f8BITSOF(carry-out).  sub: Rtt -> ~Rtt.
        Opcode::A4_addp_c => {
            let rss = sp(ctx);
            let rtt = tp(ctx);
            let px = fld(d, b'x');
            let cin = (ctx.p(px) & 1) as u128;
            let sum = rss as u128 + rtt as u128 + cin;
            ctx.set_rp(rd, sum as u64);
            ctx.set_p(px, bits8of(sum > u64::MAX as u128));
        }
        Opcode::A4_subp_c => {
            let rss = sp(ctx);
            let rtt = !tp(ctx);
            let px = fld(d, b'x');
            let cin = (ctx.p(px) & 1) as u128;
            let sum = rss as u128 + rtt as u128 + cin;
            ctx.set_rp(rd, sum as u64);
            ctx.set_p(px, bits8of(sum > u64::MAX as u128));
        }

        // ============ predicated shift/extend (A4_p{aslh,asrh,sxt,zxt}{t,f}) ==
        // Pu (field u) gates the write; CANCEL = leave Rd unwritten.
        Opcode::A4_paslht => {
            if cond(ctx, d, true) {
                ctx.set_r(rd, s(ctx) << 16);
            }
        }
        Opcode::A4_paslhf => {
            if cond(ctx, d, false) {
                ctx.set_r(rd, s(ctx) << 16);
            }
        }
        Opcode::A4_pasrht => {
            if cond(ctx, d, true) {
                ctx.set_r(rd, ((s(ctx) as i32) >> 16) as u32);
            }
        }
        Opcode::A4_pasrhf => {
            if cond(ctx, d, false) {
                ctx.set_r(rd, ((s(ctx) as i32) >> 16) as u32);
            }
        }
        Opcode::A4_psxtbt => {
            if cond(ctx, d, true) {
                ctx.set_r(rd, (s(ctx) as i8 as i32) as u32);
            }
        }
        Opcode::A4_psxtbf => {
            if cond(ctx, d, false) {
                ctx.set_r(rd, (s(ctx) as i8 as i32) as u32);
            }
        }
        Opcode::A4_psxtht => {
            if cond(ctx, d, true) {
                ctx.set_r(rd, (s(ctx) as i16 as i32) as u32);
            }
        }
        Opcode::A4_psxthf => {
            if cond(ctx, d, false) {
                ctx.set_r(rd, (s(ctx) as i16 as i32) as u32);
            }
        }
        Opcode::A4_pzxtbt => {
            if cond(ctx, d, true) {
                ctx.set_r(rd, s(ctx) & 0xff);
            }
        }
        Opcode::A4_pzxtbf => {
            if cond(ctx, d, false) {
                ctx.set_r(rd, s(ctx) & 0xff);
            }
        }
        Opcode::A4_pzxtht => {
            if cond(ctx, d, true) {
                ctx.set_r(rd, s(ctx) & 0xffff);
            }
        }
        Opcode::A4_pzxthf => {
            if cond(ctx, d, false) {
                ctx.set_r(rd, s(ctx) & 0xffff);
            }
        }

        // ============ vaddhub:sat (A5_vaddhubs) ============
        // Rd: 4 bytes, byte i = fSATUB(half(i,Rss) + half(i,Rtt)).
        Opcode::A5_vaddhubs => {
            let (a, b) = (sp(ctx), tp(ctx));
            let mut v: u64 = 0;
            for i in 0..4 {
                let sum = get_half(a, i) + get_half(b, i);
                v = set_byte(v, i, ctx.satu_n(sum, 8));
            }
            ctx.set_r(rd, v as u32);
        }

        // ============ clip to signed (#u+1)-bit range (A7_clip/vclip) =========
        // fCLIP(DST,SRC,U): maxv=(1<<U)-1, minv=-(1<<U), DST=min(maxv,max(SRC,minv)).
        // Plain clamp (size4s_t arithmetic), no USR:OVF side effect.
        Opcode::A7_clip => {
            let u_imm = fimm_u(d, b'i', None);
            ctx.set_r(rd, clip32(s(ctx) as i32, u_imm) as u32);
        }
        Opcode::A7_vclip => {
            let u_imm = fimm_u(d, b'i', None);
            let src = sp(ctx);
            let w0 = clip32(get_word(src, 0) as i32, u_imm);
            let w1 = clip32(get_word(src, 1) as i32, u_imm);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0 as i64), 1, w1 as i64));
        }

        // ============ 64-bit convergent rounding (A7_croundd_ri/rr) ===========
        // CROUND over the full signed 64-bit Rss; shift is #u6 / fZXTN(6,32,Rt).
        Opcode::A7_croundd_ri => {
            let n = fimm_u(d, b'i', None) & 0x3f;
            ctx.set_rp(rd, conv_round64(sp(ctx) as i64, n) as u64);
        }
        Opcode::A7_croundd_rr => {
            let n = t(ctx) & 0x3f; // fZXTN(6,32,Rt)
            ctx.set_rp(rd, conv_round64(sp(ctx) as i64, n) as u64);
        }

        // ============ conditional .new combine (C2_ccombinewnew{t,f}) =========
        // if (Pu.new[!]) { Rdd.w[0]=Rt; Rdd.w[1]=Rs; } else CANCEL (leave unwritten).
        Opcode::C2_ccombinewnewt => {
            if (ctx.p_new(fld(d, b'u')) & 1) != 0 {
                let v = ((s(ctx) as u64) << 32) | (t(ctx) as u64);
                ctx.set_rp(rd, v);
            }
        }
        Opcode::C2_ccombinewnewf => {
            if (ctx.p_new(fld(d, b'u')) & 1) == 0 {
                let v = ((s(ctx) as u64) << 32) | (t(ctx) as u64);
                ctx.set_rp(rd, v);
            }
        }

        // ============ add immediate to PC (C4_addipc) =========================
        // Rd = fREAD_PC() + #u6 (extendable). fREAD_PC is this packet's start PC.
        Opcode::C4_addipc => {
            let imm = fimm_u(d, b'i', ctx.immext);
            ctx.set_r(rd, ctx.regs.pc().wrapping_add(imm));
        }

        // ============ vminub with predicate output (A6_vminub_RdP) ============
        // Rdd,Pe = vminub(Rtt,Rss): per byte min, Pe bit i = (Rtt[i] > Rss[i]).
        Opcode::A6_vminub_RdP => {
            let rtt = tp(ctx);
            let rss = sp(ctx);
            let pe = fld(d, b'e');
            let mut p = 0u8;
            let mut v: u64 = 0;
            for i in 0..8 {
                let bt = get_ubyte(rtt, i);
                let bs = get_ubyte(rss, i);
                if bt > bs {
                    p |= 1 << i;
                }
                v = set_byte(v, i, bt.min(bs));
            }
            ctx.set_rp(rd, v);
            ctx.set_p(pe, p);
        }

        _ => return false,
    }
    true
}

/// Old-predicate condition (`fLSBOLD`/`fLSBOLDNOT`) for the `Pu` operand.
#[inline]
fn cond(ctx: &SemCtx, d: &DecodedOp, want_true: bool) -> bool {
    let lsb = (ctx.p(fld(d, b'u')) & 1) != 0;
    lsb == want_true
}

/// Vector reduce max/min over the four halfwords of `Rss`, seeded from the old
/// `Rxx` accumulator. Writes word0 = winning value, word1 = address.
fn vr_minmax_half(ctx: &mut SemCtx, rss: u8, ru: u32, rx: u8, is_max: bool, uns: bool) {
    let src = ctx.rp(rss);
    let acc = ctx.rp(rx);
    // Seed value from word0 low half of Rxx, addr from word1 of Rxx.
    let mut best: i64 = if uns {
        get_uhalf(acc, 0)
    } else {
        get_half(acc, 0)
    };
    let mut addr: i64 = get_word(acc, 1);
    for i in 0..4u32 {
        let lane = if uns {
            get_uhalf(src, i)
        } else {
            get_half(src, i)
        };
        let better = if is_max { best < lane } else { best > lane };
        if better {
            best = lane;
            addr = (ru | (i << 1)) as i32 as i64;
        }
    }
    let v = set_word(set_word(0, 0, best), 1, addr);
    ctx.set_rp(rx, v);
}

/// Vector reduce max/min over the two words of `Rss`, seeded from old `Rxx`.
fn vr_minmax_word(ctx: &mut SemCtx, rss: u8, ru: u32, rx: u8, is_max: bool, uns: bool) {
    let src = ctx.rp(rss);
    let acc = ctx.rp(rx);
    let mut best: i128 = if uns {
        get_uword(acc, 0) as i128
    } else {
        get_word(acc, 0) as i128
    };
    let mut addr: i64 = get_word(acc, 1);
    for i in 0..2u32 {
        let lane: i128 = if uns {
            get_uword(src, i) as i128
        } else {
            get_word(src, i) as i128
        };
        let better = if is_max { best < lane } else { best > lane };
        if better {
            best = lane;
            addr = (ru | (i << 2)) as i32 as i64;
        }
    }
    let v = set_word(set_word(0, 0, best as i64), 1, addr);
    ctx.set_rp(rx, v);
}
