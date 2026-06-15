//! Integer multiply instructions (`M2_*`, `M4_*`, `M5_*`): scalar 32x32->64
//! (`dpmpy`), high-half multiply (`mpy_up`), `mpyi` immediate multiply-add,
//! halfword/byte vector multiplies (`vmpyh`, `vmpybu`, ...), complex multiply
//! (`cmpy`, `vrcmpy`, ...), wide vector multiply (`vmpyweh`/`vmpywoh`, the
//! `mmpy*`/`mmac*` family), polynomial multiply (`pmpyw`/`vpmpyh`), and the
//! `M4` accumulating logical ops. Semantics are taken verbatim from the
//! Hexagon V68 spec (semantics_generated.pyinc) and the f-macros in
//! `imported/macros.def`. Verified against the qemu-hexagon oracle.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

// --- macro-equivalent field/extension helpers ------------------------------
//
// These mirror the C macros used by the spec:
//   fGETHALF(N,SRC)  = (size2s_t)((SRC>>(N*16))&0xffff)   -> signed 16
//   fGETUHALF(N,SRC) = (size2u_t)((SRC>>(N*16))&0xffff)   -> unsigned 16
//   fGETBYTE(N,SRC)  = (size1s_t)((SRC>>(N*8))&0xff)      -> signed 8
//   fGETUBYTE(N,SRC) = (size1u_t)((SRC>>(N*8))&0xff)      -> unsigned 8
//   fGETWORD(N,SRC)  = (size8s_t)(size4s_t)((SRC>>(N*32))&0xffffffff) -> signed 32->64
//   fSETHALF/fSETWORD insert a half/word lane (masking to the lane width).
//   fMPY16SS(a,b)    = fSE32_64(fSE16_32(a)*fSE16_32(b))  (16x16 signed, fits in i32)
//   fMPY16SU(a,b)    = fSE32_64(fSE16_32(a)*fZE16_32(b))  (signed*unsigned 16)
//   fMPY3216SS(a,b)  = fSE32_64(a) * fSXTN(16,64,b)       (32 x signed-16)
//   fMPY3216SU(a,b)  = fSE32_64(a) * fZXTN(16,64,b)       (32 x unsigned-16)
//   fSCALE(N,A)      = ((size8s_t)A) << N
//   fSAT(A)          = fSATN(32,A) (saturate signed-32, sets USR:OVF)

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
#[inline]
fn get_word(src: u64, n: u32) -> i64 {
    ((src >> (n * 32)) as u32 as i32) as i64
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

/// fMPY16SS: signed 16 x signed 16 (the 32-bit product always fits, then s.ext).
#[inline]
fn mpy16ss(a: i64, b: i64) -> i64 {
    ((a as i16 as i32).wrapping_mul(b as i16 as i32)) as i64
}
/// fMPY16SU: signed 16 x unsigned 16.
#[inline]
fn mpy16su(a: i64, b: i64) -> i64 {
    ((a as i16 as i32).wrapping_mul((b as u16) as i32)) as i64
}
/// fMPY3216SS: signed 32 x signed 16, full 64-bit product.
#[inline]
fn mpy3216ss(a: i64, b: i64) -> i64 {
    (a as i32 as i64).wrapping_mul(b as i16 as i64)
}
/// fMPY3216SU: signed 32 x unsigned 16, full 64-bit product.
#[inline]
fn mpy3216su(a: i64, b: i64) -> i64 {
    (a as i32 as i64).wrapping_mul((b as u16) as i64)
}

/// Execute a mpy-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let u = |c: &SemCtx| c.r(fld(d, b'u'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let rd = fld(d, b'd');
    let rx = fld(d, b'x');
    let ry = fld(d, b'y');
    let xx = |c: &SemCtx| c.rp(fld(d, b'x'));
    let x = |c: &SemCtx| c.r(fld(d, b'x'));

    match op {
        // ============ 32x32 -> 64 multiply (dpmpy) ============
        Opcode::M2_dpmpyss_s0 => {
            let v = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_dpmpyuu_s0 => {
            let v = (s(ctx) as u64).wrapping_mul(t(ctx) as u64);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_dpmpyss_acc_s0 => {
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_dpmpyss_nac_s0 => {
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            let v = (xx(ctx) as i64).wrapping_sub(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_dpmpyuu_acc_s0 => {
            let p = (s(ctx) as u64).wrapping_mul(t(ctx) as u64);
            let v = xx(ctx).wrapping_add(p);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_dpmpyuu_nac_s0 => {
            let p = (s(ctx) as u64).wrapping_mul(t(ctx) as u64);
            let v = xx(ctx).wrapping_sub(p);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_dpmpyss_rnd_s0 => {
            // RdV=(fMPY32SS(RsV,RtV)+0x80000000LL)>>32;
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            let v = (p.wrapping_add(0x8000_0000) >> 32) as u32;
            ctx.set_r(rd, v);
        }

        // ============ high-half 32x32 multiply (mpy_up) ============
        Opcode::M2_mpy_up => {
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            ctx.set_r(rd, (p >> 32) as u32);
        }
        Opcode::M2_mpy_up_s1 => {
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            ctx.set_r(rd, (p >> 31) as u32);
        }
        Opcode::M2_mpy_up_s1_sat => {
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            let v = ctx.sat_n(p >> 31, 32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::M2_mpyu_up => {
            let p = (s(ctx) as u64).wrapping_mul(t(ctx) as u64);
            ctx.set_r(rd, (p >> 32) as u32);
        }
        Opcode::M2_mpysu_up => {
            // fMPY32SU(RsV, (u)RtV) >> 32  = s.ext(Rs) * z.ext(Rt)
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as u64 as i64);
            ctx.set_r(rd, (p >> 32) as u32);
        }

        // ============ mpyi immediate multiply-(add) ============
        Opcode::M2_mpysip => {
            // RdV = Rs * #u8 (extendable)
            let imm = fimm_u(d, b'i', ctx.immext);
            ctx.set_r(rd, s(ctx).wrapping_mul(imm));
        }
        Opcode::M2_mpysin => {
            // RdV = Rs * -#u8  (not extendable)
            let imm = fimm_u(d, b'i', None);
            ctx.set_r(rd, s(ctx).wrapping_mul(imm.wrapping_neg()));
        }
        Opcode::M2_macsip => {
            // RxV += Rs * #u8 (extendable)
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = x(ctx).wrapping_add(s(ctx).wrapping_mul(imm));
            ctx.set_r(rx, v);
        }
        Opcode::M2_macsin => {
            // RxV -= Rs * #u8 (extendable)
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = x(ctx).wrapping_sub(s(ctx).wrapping_mul(imm));
            ctx.set_r(rx, v);
        }
        Opcode::M2_maci => {
            // RxV += Rs * Rt
            let v = x(ctx).wrapping_add(s(ctx).wrapping_mul(t(ctx)));
            ctx.set_r(rx, v);
        }
        Opcode::M2_mnaci => {
            // RxV -= Rs * Rt
            let v = x(ctx).wrapping_sub(s(ctx).wrapping_mul(t(ctx)));
            ctx.set_r(rx, v);
        }

        // ============ add/sub accumulate (acci family) ============
        Opcode::M2_acci => {
            // RxV += Rs + Rt
            let v = x(ctx).wrapping_add(s(ctx)).wrapping_add(t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M2_accii => {
            // RxV += Rs + #s8 (extendable)
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = x(ctx).wrapping_add(s(ctx)).wrapping_add(imm);
            ctx.set_r(rx, v);
        }
        Opcode::M2_nacci => {
            // RxV -= (Rs + Rt)
            let v = x(ctx).wrapping_sub(s(ctx).wrapping_add(t(ctx)));
            ctx.set_r(rx, v);
        }
        Opcode::M2_naccii => {
            // RxV -= (Rs + #s8) (extendable)
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = x(ctx).wrapping_sub(s(ctx).wrapping_add(imm));
            ctx.set_r(rx, v);
        }
        Opcode::M2_subacc => {
            // RxV += Rt - Rs
            let v = x(ctx).wrapping_add(t(ctx)).wrapping_sub(s(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M2_xor_xacc => {
            // RxV ^= Rs ^ Rt
            let v = x(ctx) ^ s(ctx) ^ t(ctx);
            ctx.set_r(rx, v);
        }
        Opcode::M4_xor_xacc => {
            // RxxV ^= Rss ^ Rtt
            let v = xx(ctx) ^ sp(ctx) ^ tp(ctx);
            ctx.set_rp(rx, v);
        }

        // ============ M4 accumulating logical ============
        Opcode::M4_and_and => {
            let v = x(ctx) & (s(ctx) & t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_and_or => {
            let v = x(ctx) & (s(ctx) | t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_and_xor => {
            let v = x(ctx) & (s(ctx) ^ t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_and_andn => {
            let v = x(ctx) & (s(ctx) & !t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_or_and => {
            let v = x(ctx) | (s(ctx) & t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_or_or => {
            let v = x(ctx) | (s(ctx) | t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_or_xor => {
            let v = x(ctx) | (s(ctx) ^ t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_or_andn => {
            let v = x(ctx) | (s(ctx) & !t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_xor_and => {
            let v = x(ctx) ^ (s(ctx) & t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_xor_or => {
            let v = x(ctx) ^ (s(ctx) | t(ctx));
            ctx.set_r(rx, v);
        }
        Opcode::M4_xor_andn => {
            let v = x(ctx) ^ (s(ctx) & !t(ctx));
            ctx.set_r(rx, v);
        }

        // ============ mpyi + add (M4) ============
        Opcode::M4_mpyrr_addr => {
            // RyV = Ru + Rs * Ry
            let v = u(ctx).wrapping_add(s(ctx).wrapping_mul(ctx.r(ry)));
            ctx.set_r(ry, v);
        }
        Opcode::M4_mpyrr_addi => {
            // RdV = #u6 + Rs * Rt (extendable)
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = imm.wrapping_add(s(ctx).wrapping_mul(t(ctx)));
            ctx.set_r(rd, v);
        }
        Opcode::M4_mpyri_addr => {
            // RdV = Ru + Rs * #u6 (extendable)
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = u(ctx).wrapping_add(s(ctx).wrapping_mul(imm));
            ctx.set_r(rd, v);
        }
        Opcode::M4_mpyri_addi => {
            // RdV = #u6 + Rs * #U6 (u extendable)
            let imm_u = fimm_u(d, b'i', ctx.immext);
            let imm_uu = fimm_u(d, b'I', None);
            let v = imm_u.wrapping_add(s(ctx).wrapping_mul(imm_uu));
            ctx.set_r(rd, v);
        }
        Opcode::M4_mpyri_addr_u2 => {
            // RdV = Ru + Rs * #u6:2 (not extendable). The ":2" suffix means the
            // 6-bit field encodes the immediate with two implicit low zero bits,
            // so scale the raw field by 4.
            let imm = fimm_u(d, b'i', None) << 2;
            let v = u(ctx).wrapping_add(s(ctx).wrapping_mul(imm));
            ctx.set_r(rd, v);
        }

        // ============ mpy:<<1:sat with 64-bit accumulate (M4) ============
        Opcode::M4_mac_up_s1_sat => {
            // RxV = fSAT( s.ext(RxV) + (fMPY32SS(Rs,Rt) >> 31) )
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            let acc = (x(ctx) as i32 as i64).wrapping_add(p >> 31);
            let v = ctx.sat_n(acc, 32) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::M4_nac_up_s1_sat => {
            let p = (s(ctx) as i32 as i64).wrapping_mul(t(ctx) as i32 as i64);
            let acc = (x(ctx) as i32 as i64).wrapping_sub(p >> 31);
            let v = ctx.sat_n(acc, 32) as u32;
            ctx.set_r(rx, v);
        }

        // ============ polynomial multiply ============
        Opcode::M4_pmpyw => {
            let v = pmpyw(s(ctx), t(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::M4_pmpyw_acc => {
            let v = xx(ctx) ^ pmpyw(s(ctx), t(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::M4_vpmpyh => {
            let v = vpmpyh(s(ctx), t(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::M4_vpmpyh_acc => {
            // XOR each 16-bit lane of the accumulator with the product's lanes.
            let v = xx(ctx) ^ vpmpyh(s(ctx), t(ctx));
            ctx.set_rp(rx, v);
        }

        // ============ vmpyh: 2x (16x16) -> 2x32 (vmpy2s) ============
        Opcode::M2_vmpy2s_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w0 = ctx.sat_n(mpy16ss(get_half(rs, 0), get_half(rt, 0)), 32);
            let w1 = ctx.sat_n(mpy16ss(get_half(rs, 1), get_half(rt, 1)), 32);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vmpy2s_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w0 = ctx.sat_n(mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1, 32);
            let w1 = ctx.sat_n(mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1, 32);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vmpy2su_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w0 = ctx.sat_n(mpy16su(get_half(rs, 0), get_uhalf(rt, 0)), 32);
            let w1 = ctx.sat_n(mpy16su(get_half(rs, 1), get_uhalf(rt, 1)), 32);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vmpy2su_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w0 = ctx.sat_n(mpy16su(get_half(rs, 0), get_uhalf(rt, 0)) << 1, 32);
            let w1 = ctx.sat_n(mpy16su(get_half(rs, 1), get_uhalf(rt, 1)) << 1, 32);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vmpy2s_s0pack => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let h1 = ctx.sat_n(mpy16ss(get_half(rs, 1), get_half(rt, 1)) + 0x8000, 32);
            let h0 = ctx.sat_n(mpy16ss(get_half(rs, 0), get_half(rt, 0)) + 0x8000, 32);
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_vmpy2s_s1pack => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let h1 = ctx.sat_n(
                (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1) + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1) + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_vmac2 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w0 = get_word(acc, 0).wrapping_add(mpy16ss(get_half(rs, 0), get_half(rt, 0)));
            let w1 = get_word(acc, 1).wrapping_add(mpy16ss(get_half(rs, 1), get_half(rt, 1)));
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2s_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0) + mpy16ss(get_half(rs, 0), get_half(rt, 0)),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1) + mpy16ss(get_half(rs, 1), get_half(rt, 1)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2s_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0) + (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1) + (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2su_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0) + mpy16su(get_half(rs, 0), get_uhalf(rt, 0)),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1) + mpy16su(get_half(rs, 1), get_uhalf(rt, 1)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2su_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0) + (mpy16su(get_half(rs, 0), get_uhalf(rt, 0)) << 1),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1) + (mpy16su(get_half(rs, 1), get_uhalf(rt, 1)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }

        // ============ vmpyeh: even-halfword 16x16 -> 2x32 (vmpy2es) ============
        Opcode::M2_vmpy2es_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(mpy16ss(get_half(rss, 0), get_half(rtt, 0)), 32);
            let w1 = ctx.sat_n(mpy16ss(get_half(rss, 2), get_half(rtt, 2)), 32);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vmpy2es_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(mpy16ss(get_half(rss, 0), get_half(rtt, 0)) << 1, 32);
            let w1 = ctx.sat_n(mpy16ss(get_half(rss, 2), get_half(rtt, 2)) << 1, 32);
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2es => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = get_word(acc, 0).wrapping_add(mpy16ss(get_half(rss, 0), get_half(rtt, 0)));
            let w1 = get_word(acc, 1).wrapping_add(mpy16ss(get_half(rss, 2), get_half(rtt, 2)));
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2es_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0) + mpy16ss(get_half(rss, 0), get_half(rtt, 0)),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1) + mpy16ss(get_half(rss, 2), get_half(rtt, 2)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vmac2es_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0) + (mpy16ss(get_half(rss, 0), get_half(rtt, 0)) << 1),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1) + (mpy16ss(get_half(rss, 2), get_half(rtt, 2)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }

        // ============ vdmpy: vector dual-multiply (pairs of halves) ============
        Opcode::M2_vdmpys_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(
                mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                    + mpy16ss(get_half(rss, 1), get_half(rtt, 1)),
                32,
            );
            let w1 = ctx.sat_n(
                mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                    + mpy16ss(get_half(rss, 3), get_half(rtt, 3)),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vdmpys_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(rtt, 0)) << 1)
                    + (mpy16ss(get_half(rss, 1), get_half(rtt, 1)) << 1),
                32,
            );
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 2), get_half(rtt, 2)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(rtt, 3)) << 1),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vdmacs_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                    + mpy16ss(get_half(rss, 1), get_half(rtt, 1)),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                    + mpy16ss(get_half(rss, 3), get_half(rtt, 3)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vdmacs_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + (mpy16ss(get_half(rss, 0), get_half(rtt, 0)) << 1)
                    + (mpy16ss(get_half(rss, 1), get_half(rtt, 1)) << 1),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + (mpy16ss(get_half(rss, 2), get_half(rtt, 2)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(rtt, 3)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vdmpyrs_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let s0 = ctx.sat_n(
                mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                    + mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                    + 0x8000,
                32,
            );
            let s1 = ctx.sat_n(
                mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                    + mpy16ss(get_half(rss, 3), get_half(rtt, 3))
                    + 0x8000,
                32,
            );
            // fSETHALF(0,Rd,fGETHALF(1,s0)); fSETHALF(1,Rd,fGETHALF(1,s1));
            let v = set_half(
                set_half(0, 0, get_half(s0 as u64, 1)),
                1,
                get_half(s1 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_vdmpyrs_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let s0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(rtt, 0)) << 1)
                    + (mpy16ss(get_half(rss, 1), get_half(rtt, 1)) << 1)
                    + 0x8000,
                32,
            );
            let s1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 2), get_half(rtt, 2)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(rtt, 3)) << 1)
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 0, get_half(s0 as u64, 1)),
                1,
                get_half(s1 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }

        // ============ vraddh / vradduh / vabsdiff ============
        Opcode::M2_vraddh => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut acc: i64 = 0;
            for i in 0..4 {
                acc = acc.wrapping_add(get_half(rss, i) + get_half(rtt, i));
            }
            ctx.set_r(rd, acc as u32);
        }
        Opcode::M2_vradduh => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut acc: i64 = 0;
            for i in 0..4 {
                acc = acc.wrapping_add(get_uhalf(rss, i) + get_uhalf(rtt, i));
            }
            ctx.set_r(rd, acc as u32);
        }
        Opcode::M2_vabsdiffh => {
            // operands: Rdd = vabsdiffh(Rtt, Rss)  -> abs(half(Rtt) - half(Rss))
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut v: u64 = 0;
            for i in 0..4 {
                v = set_half(v, i, (get_half(rtt, i) - get_half(rss, i)).abs());
            }
            ctx.set_rp(rd, v);
        }
        Opcode::M2_vabsdiffw => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut v: u64 = 0;
            for i in 0..2 {
                v = set_word(v, i, (get_word(rtt, i) - get_word(rss, i)).abs());
            }
            ctx.set_rp(rd, v);
        }

        // ============ cmpy: complex multiply, halfword ============
        Opcode::M2_cmpyi_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            // imaginary: Rs.H*Rt.L + Rs.L*Rt.H
            let v = mpy16ss(get_half(rs, 1), get_half(rt, 0))
                + mpy16ss(get_half(rs, 0), get_half(rt, 1));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_cmpyr_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            // real: Rs.L*Rt.L - Rs.H*Rt.H
            let v = mpy16ss(get_half(rs, 0), get_half(rt, 0))
                - mpy16ss(get_half(rs, 1), get_half(rt, 1));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_cmpys_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w1 = ctx.sat_n(
                mpy16ss(get_half(rs, 1), get_half(rt, 0))
                    + mpy16ss(get_half(rs, 0), get_half(rt, 1)),
                32,
            );
            let w0 = ctx.sat_n(
                mpy16ss(get_half(rs, 0), get_half(rt, 0))
                    - mpy16ss(get_half(rs, 1), get_half(rt, 1)),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 1, w1), 0, w0));
        }
        Opcode::M2_cmpys_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                    + (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                    - (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 1, w1), 0, w0));
        }
        Opcode::M2_cmpysc_s0 => {
            // conjugate: imag uses '-', real uses '+'
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w1 = ctx.sat_n(
                mpy16ss(get_half(rs, 1), get_half(rt, 0))
                    - mpy16ss(get_half(rs, 0), get_half(rt, 1)),
                32,
            );
            let w0 = ctx.sat_n(
                mpy16ss(get_half(rs, 0), get_half(rt, 0))
                    + mpy16ss(get_half(rs, 1), get_half(rt, 1)),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 1, w1), 0, w0));
        }
        Opcode::M2_cmpysc_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                    - (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                    + (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 1, w1), 0, w0));
        }
        Opcode::M2_cmacs_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + mpy16ss(get_half(rs, 1), get_half(rt, 0))
                    + mpy16ss(get_half(rs, 0), get_half(rt, 1)),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0) + mpy16ss(get_half(rs, 0), get_half(rt, 0))
                    - mpy16ss(get_half(rs, 1), get_half(rt, 1)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cmacs_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + (mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                    + (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0) + (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                    - (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cmacsc_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1) + mpy16ss(get_half(rs, 1), get_half(rt, 0))
                    - mpy16ss(get_half(rs, 0), get_half(rt, 1)),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + mpy16ss(get_half(rs, 0), get_half(rt, 0))
                    + mpy16ss(get_half(rs, 1), get_half(rt, 1)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cmacsc_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1) + (mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                    - (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                    + (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cnacs_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    - (mpy16ss(get_half(rs, 1), get_half(rt, 0))
                        + mpy16ss(get_half(rs, 0), get_half(rt, 1))),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    - (mpy16ss(get_half(rs, 0), get_half(rt, 0))
                        - mpy16ss(get_half(rs, 1), get_half(rt, 1))),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cnacs_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    - ((mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                        + (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1)),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    - ((mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                        - (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cnacsc_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    - (mpy16ss(get_half(rs, 1), get_half(rt, 0))
                        - mpy16ss(get_half(rs, 0), get_half(rt, 1))),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    - (mpy16ss(get_half(rs, 0), get_half(rt, 0))
                        + mpy16ss(get_half(rs, 1), get_half(rt, 1))),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cnacsc_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    - ((mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                        - (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1)),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    - ((mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                        + (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1)),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_cmaci_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let p = mpy16ss(get_half(rs, 1), get_half(rt, 0))
                + mpy16ss(get_half(rs, 0), get_half(rt, 1));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_cmacr_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let p = mpy16ss(get_half(rs, 0), get_half(rt, 0))
                - mpy16ss(get_half(rs, 1), get_half(rt, 1));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }

        // ---- cmpy round+sat -> packed single (cmpyrs / cmpyrsc) ----
        Opcode::M2_cmpyrs_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let h1 = ctx.sat_n(
                mpy16ss(get_half(rs, 1), get_half(rt, 0))
                    + mpy16ss(get_half(rs, 0), get_half(rt, 1))
                    + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                mpy16ss(get_half(rs, 0), get_half(rt, 0))
                    - mpy16ss(get_half(rs, 1), get_half(rt, 1))
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_cmpyrs_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let h1 = ctx.sat_n(
                (mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                    + (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1)
                    + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                    - (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1)
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_cmpyrsc_s0 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let h1 = ctx.sat_n(
                mpy16ss(get_half(rs, 1), get_half(rt, 0))
                    - mpy16ss(get_half(rs, 0), get_half(rt, 1))
                    + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                mpy16ss(get_half(rs, 0), get_half(rt, 0))
                    + mpy16ss(get_half(rs, 1), get_half(rt, 1))
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_cmpyrsc_s1 => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let h1 = ctx.sat_n(
                (mpy16ss(get_half(rs, 1), get_half(rt, 0)) << 1)
                    - (mpy16ss(get_half(rs, 0), get_half(rt, 1)) << 1)
                    + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                (mpy16ss(get_half(rs, 0), get_half(rt, 0)) << 1)
                    + (mpy16ss(get_half(rs, 1), get_half(rt, 1)) << 1)
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }

        // ============ vrmpyh / vrcmpy: reduce-multiply of 4 halves -> 64 ============
        Opcode::M2_vrmpy_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                + mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 3));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_vrmac_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                + mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 3));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_vrcmpyi_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                + mpy16ss(get_half(rss, 0), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 3));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_vrcmpyr_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                - mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                - mpy16ss(get_half(rss, 3), get_half(rtt, 3));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_vrcmpyi_s0c => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                - mpy16ss(get_half(rss, 0), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                - mpy16ss(get_half(rss, 2), get_half(rtt, 3));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_vrcmpyr_s0c => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                + mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 3));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M2_vrcmaci_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                + mpy16ss(get_half(rss, 0), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 3));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_vrcmacr_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                - mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                - mpy16ss(get_half(rss, 3), get_half(rtt, 3));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_vrcmaci_s0c => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                - mpy16ss(get_half(rss, 0), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                - mpy16ss(get_half(rss, 2), get_half(rtt, 3));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M2_vrcmacr_s0c => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                + mpy16ss(get_half(rss, 1), get_half(rtt, 1))
                + mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                + mpy16ss(get_half(rss, 3), get_half(rtt, 3));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }

        // ============ vcmpy: complex vector multiply (2 lanes) :sat ============
        Opcode::M2_vcmpy_s0_sat_r => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(
                mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                    - mpy16ss(get_half(rss, 1), get_half(rtt, 1)),
                32,
            );
            let w1 = ctx.sat_n(
                mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                    - mpy16ss(get_half(rss, 3), get_half(rtt, 3)),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vcmpy_s1_sat_r => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                    - mpy16ss(get_half(rss, 1), get_half(rtt, 1)))
                    << 1,
                32,
            );
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                    - mpy16ss(get_half(rss, 3), get_half(rtt, 3)))
                    << 1,
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vcmpy_s0_sat_i => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(
                mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                    + mpy16ss(get_half(rss, 0), get_half(rtt, 1)),
                32,
            );
            let w1 = ctx.sat_n(
                mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                    + mpy16ss(get_half(rss, 2), get_half(rtt, 3)),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vcmpy_s1_sat_i => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                    + mpy16ss(get_half(rss, 0), get_half(rtt, 1)))
                    << 1,
                32,
            );
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                    + mpy16ss(get_half(rss, 2), get_half(rtt, 3)))
                    << 1,
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M2_vcmac_s0_sat_r => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + (mpy16ss(get_half(rss, 0), get_half(rtt, 0))
                        - mpy16ss(get_half(rss, 1), get_half(rtt, 1))),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + (mpy16ss(get_half(rss, 2), get_half(rtt, 2))
                        - mpy16ss(get_half(rss, 3), get_half(rtt, 3))),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M2_vcmac_s0_sat_i => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + (mpy16ss(get_half(rss, 1), get_half(rtt, 0))
                        + mpy16ss(get_half(rss, 0), get_half(rtt, 1))),
                32,
            );
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + (mpy16ss(get_half(rss, 3), get_half(rtt, 2))
                        + mpy16ss(get_half(rss, 2), get_half(rtt, 3))),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }

        // ============ mmpy/mmac: wide 32x16 vector multiply (even/odd) ============
        // even = vmpyweh (mmpyl/mmacls), odd = vmpywoh (mmpyh/mmachs).
        // signed*signed:
        Opcode::M2_mmpyl_s0 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, false, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyl_s1 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, false, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyl_rs0 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, false, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyl_rs1 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, false, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyh_s0 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, true, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyh_s1 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, true, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyh_rs0 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, true, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyh_rs1 => {
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, true, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmacls_s0 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, false, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacls_s1 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, false, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacls_rs0 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, false, true, acc, true);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacls_rs1 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, false, true, acc, true);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmachs_s0 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, true, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmachs_s1 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, true, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmachs_rs0 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 0, true, true, acc, true);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmachs_rs1 => {
            let acc = xx(ctx);
            let v = mmpy_ss(ctx, sp(ctx), tp(ctx), 1, true, true, acc, true);
            ctx.set_rp(rx, v);
        }
        // signed*unsigned (vmpyweuh / vmpywouh):
        Opcode::M2_mmpyul_s0 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, false, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyul_s1 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, false, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyul_rs0 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, false, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyul_rs1 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, false, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyuh_s0 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, true, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyuh_s1 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, true, false, 0, false);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyuh_rs0 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, true, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmpyuh_rs1 => {
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, true, false, 0, true);
            ctx.set_rp(rd, v);
        }
        Opcode::M2_mmaculs_s0 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, false, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmaculs_s1 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, false, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmaculs_rs0 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, false, true, acc, true);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmaculs_rs1 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, false, true, acc, true);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacuhs_s0 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, true, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacuhs_s1 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, true, true, acc, false);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacuhs_rs0 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 0, true, true, acc, true);
            ctx.set_rp(rx, v);
        }
        Opcode::M2_mmacuhs_rs1 => {
            let acc = xx(ctx);
            let v = mmpy_su(ctx, sp(ctx), tp(ctx), 1, true, true, acc, true);
            ctx.set_rp(rx, v);
        }

        // ============ vrmpyweh / vrmpywoh: 2x (32x16) -> 64 ============
        Opcode::M4_vrmpyeh_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy3216ss(get_word(rss, 1), get_half(rtt, 2))
                + mpy3216ss(get_word(rss, 0), get_half(rtt, 0));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M4_vrmpyeh_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = (mpy3216ss(get_word(rss, 1), get_half(rtt, 2)) << 1)
                + (mpy3216ss(get_word(rss, 0), get_half(rtt, 0)) << 1);
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M4_vrmpyoh_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = mpy3216ss(get_word(rss, 1), get_half(rtt, 3))
                + mpy3216ss(get_word(rss, 0), get_half(rtt, 1));
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M4_vrmpyoh_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let v = (mpy3216ss(get_word(rss, 1), get_half(rtt, 3)) << 1)
                + (mpy3216ss(get_word(rss, 0), get_half(rtt, 1)) << 1);
            ctx.set_rp(rd, v as u64);
        }
        Opcode::M4_vrmpyeh_acc_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy3216ss(get_word(rss, 1), get_half(rtt, 2))
                + mpy3216ss(get_word(rss, 0), get_half(rtt, 0));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M4_vrmpyeh_acc_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = (mpy3216ss(get_word(rss, 1), get_half(rtt, 2)) << 1)
                + (mpy3216ss(get_word(rss, 0), get_half(rtt, 0)) << 1);
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M4_vrmpyoh_acc_s0 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = mpy3216ss(get_word(rss, 1), get_half(rtt, 3))
                + mpy3216ss(get_word(rss, 0), get_half(rtt, 1));
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }
        Opcode::M4_vrmpyoh_acc_s1 => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let p = (mpy3216ss(get_word(rss, 1), get_half(rtt, 3)) << 1)
                + (mpy3216ss(get_word(rss, 0), get_half(rtt, 1)) << 1);
            let v = (xx(ctx) as i64).wrapping_add(p);
            ctx.set_rp(rx, v as u64);
        }

        // ============ cmpyiwh / cmpyrwh: complex 32x16 -> :<<1:rnd:sat ============
        Opcode::M4_cmpyi_wh => {
            let (rss, rt) = (sp(ctx), t(ctx) as u64);
            let v = ctx.sat_n(
                (mpy3216ss(get_word(rss, 0), get_half(rt, 1))
                    + mpy3216ss(get_word(rss, 1), get_half(rt, 0))
                    + 0x4000)
                    >> 15,
                32,
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M4_cmpyi_whc => {
            let (rss, rt) = (sp(ctx), t(ctx) as u64);
            let v = ctx.sat_n(
                (mpy3216ss(get_word(rss, 1), get_half(rt, 0))
                    - mpy3216ss(get_word(rss, 0), get_half(rt, 1))
                    + 0x4000)
                    >> 15,
                32,
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M4_cmpyr_wh => {
            let (rss, rt) = (sp(ctx), t(ctx) as u64);
            let v = ctx.sat_n(
                (mpy3216ss(get_word(rss, 0), get_half(rt, 0))
                    - mpy3216ss(get_word(rss, 1), get_half(rt, 1))
                    + 0x4000)
                    >> 15,
                32,
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M4_cmpyr_whc => {
            let (rss, rt) = (sp(ctx), t(ctx) as u64);
            let v = ctx.sat_n(
                (mpy3216ss(get_word(rss, 0), get_half(rt, 0))
                    + mpy3216ss(get_word(rss, 1), get_half(rt, 1))
                    + 0x4000)
                    >> 15,
                32,
            );
            ctx.set_r(rd, v as u32);
        }

        // ============ vrcmpys: complex reduce with replicated halfword ============
        Opcode::M2_vrcmpys_s1_h => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let h = get_word(rtt, 1) as u64;
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 1), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(h, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 2), get_half(h, 1)) << 1),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 1, w1), 0, w0));
        }
        Opcode::M2_vrcmpys_s1_l => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let h = get_word(rtt, 0) as u64;
            let w1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 1), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(h, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 2), get_half(h, 1)) << 1),
                32,
            );
            ctx.set_rp(rd, set_word(set_word(0, 1, w1), 0, w0));
        }
        Opcode::M2_vrcmpys_acc_s1_h => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let h = get_word(rtt, 1) as u64;
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + (mpy16ss(get_half(rss, 1), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(h, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + (mpy16ss(get_half(rss, 0), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 2), get_half(h, 1)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_vrcmpys_acc_s1_l => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let h = get_word(rtt, 0) as u64;
            let w1 = ctx.sat_n(
                get_word(acc, 1)
                    + (mpy16ss(get_half(rss, 1), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(h, 1)) << 1),
                32,
            );
            let w0 = ctx.sat_n(
                get_word(acc, 0)
                    + (mpy16ss(get_half(rss, 0), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 2), get_half(h, 1)) << 1),
                32,
            );
            ctx.set_rp(rx, set_word(set_word(acc, 1, w1), 0, w0));
        }
        Opcode::M2_vrcmpys_s1rp_h => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let h = get_word(rtt, 1) as u64;
            let h1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 1), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(h, 1)) << 1)
                    + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 2), get_half(h, 1)) << 1)
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }
        Opcode::M2_vrcmpys_s1rp_l => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let h = get_word(rtt, 0) as u64;
            let h1 = ctx.sat_n(
                (mpy16ss(get_half(rss, 1), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 3), get_half(h, 1)) << 1)
                    + 0x8000,
                32,
            );
            let h0 = ctx.sat_n(
                (mpy16ss(get_half(rss, 0), get_half(h, 0)) << 1)
                    + (mpy16ss(get_half(rss, 2), get_half(h, 1)) << 1)
                    + 0x8000,
                32,
            );
            let v = set_half(
                set_half(0, 1, get_half(h1 as u64, 1)),
                0,
                get_half(h0 as u64, 1),
            );
            ctx.set_r(rd, v as u32);
        }

        // ============ M5: byte multiplies ============
        Opcode::M5_vmpybuu => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let mut v: u64 = 0;
            for i in 0..4 {
                v = set_half(v, i, mpy16ss(get_ubyte(rs, i), get_ubyte(rt, i)));
            }
            ctx.set_rp(rd, v);
        }
        Opcode::M5_vmpybsu => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let mut v: u64 = 0;
            for i in 0..4 {
                v = set_half(v, i, mpy16ss(get_byte(rs, i), get_ubyte(rt, i)));
            }
            ctx.set_rp(rd, v);
        }
        Opcode::M5_vmacbuu => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let mut v: u64 = 0;
            for i in 0..4 {
                v = set_half(
                    v,
                    i,
                    get_half(acc, i) + mpy16ss(get_ubyte(rs, i), get_ubyte(rt, i)),
                );
            }
            ctx.set_rp(rx, v);
        }
        Opcode::M5_vmacbsu => {
            let (rs, rt) = (s(ctx) as u64, t(ctx) as u64);
            let acc = xx(ctx);
            let mut v: u64 = 0;
            for i in 0..4 {
                v = set_half(
                    v,
                    i,
                    get_half(acc, i) + mpy16ss(get_byte(rs, i), get_ubyte(rt, i)),
                );
            }
            ctx.set_rp(rx, v);
        }
        Opcode::M5_vdmpybsu => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut v: u64 = 0;
            for i in 0..4 {
                let sum = mpy16ss(get_byte(rss, 2 * i), get_ubyte(rtt, 2 * i))
                    + mpy16ss(get_byte(rss, 2 * i + 1), get_ubyte(rtt, 2 * i + 1));
                let h = ctx.sat_n(sum, 16);
                v = set_half(v, i, h);
            }
            ctx.set_rp(rd, v);
        }
        Opcode::M5_vdmacbsu => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let mut v: u64 = 0;
            for i in 0..4 {
                let sum = get_half(acc, i)
                    + mpy16ss(get_byte(rss, 2 * i), get_ubyte(rtt, 2 * i))
                    + mpy16ss(get_byte(rss, 2 * i + 1), get_ubyte(rtt, 2 * i + 1));
                let h = ctx.sat_n(sum, 16);
                v = set_half(v, i, h);
            }
            ctx.set_rp(rx, v);
        }
        Opcode::M5_vrmpybuu => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut w0: i64 = 0;
            let mut w1: i64 = 0;
            for i in 0..4 {
                w0 += mpy16ss(get_ubyte(rss, i), get_ubyte(rtt, i));
                w1 += mpy16ss(get_ubyte(rss, i + 4), get_ubyte(rtt, i + 4));
            }
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M5_vrmpybsu => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let mut w0: i64 = 0;
            let mut w1: i64 = 0;
            for i in 0..4 {
                w0 += mpy16ss(get_byte(rss, i), get_ubyte(rtt, i));
                w1 += mpy16ss(get_byte(rss, i + 4), get_ubyte(rtt, i + 4));
            }
            ctx.set_rp(rd, set_word(set_word(0, 0, w0), 1, w1));
        }
        Opcode::M5_vrmacbuu => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let mut w0: i64 = get_word(acc, 0);
            let mut w1: i64 = get_word(acc, 1);
            for i in 0..4 {
                w0 += mpy16ss(get_ubyte(rss, i), get_ubyte(rtt, i));
                w1 += mpy16ss(get_ubyte(rss, i + 4), get_ubyte(rtt, i + 4));
            }
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }
        Opcode::M5_vrmacbsu => {
            let (rss, rtt) = (sp(ctx), tp(ctx));
            let acc = xx(ctx);
            let mut w0: i64 = get_word(acc, 0);
            let mut w1: i64 = get_word(acc, 1);
            for i in 0..4 {
                w0 += mpy16ss(get_byte(rss, i), get_ubyte(rtt, i));
                w1 += mpy16ss(get_byte(rss, i + 4), get_ubyte(rtt, i + 4));
            }
            ctx.set_rp(rx, set_word(set_word(acc, 0, w0), 1, w1));
        }

        _ => return false,
    }
    true
}

/// Polynomial (carry-less) multiply of two 32-bit words -> 64-bit.
fn pmpyw(rs: u32, rt: u32) -> u64 {
    let x = rs as u64;
    let y = rt as u64;
    let mut prod: u64 = 0;
    for i in 0..32 {
        if (y >> i) & 1 == 1 {
            prod ^= x << i;
        }
    }
    prod
}

/// Polynomial multiply of two pairs of 16-bit halves (`vpmpyh`), interleaved.
fn vpmpyh(rs: u32, rt: u32) -> u64 {
    let x0 = (rs & 0xffff) as u64;
    let x1 = ((rs >> 16) & 0xffff) as u64;
    let y0 = (rt & 0xffff) as u64;
    let y1 = ((rt >> 16) & 0xffff) as u64;
    let mut prod0: u32 = 0;
    let mut prod1: u32 = 0;
    for i in 0..16 {
        if (y0 >> i) & 1 == 1 {
            prod0 ^= (x0 << i) as u32;
        }
        if (y1 >> i) & 1 == 1 {
            prod1 ^= (x1 << i) as u32;
        }
    }
    // half0=prod0.lo, half1=prod1.lo, half2=prod0.hi, half3=prod1.hi
    let mut v: u64 = 0;
    v = set_half(v, 0, (prod0 & 0xffff) as i64);
    v = set_half(v, 1, (prod1 & 0xffff) as i64);
    v = set_half(v, 2, ((prod0 >> 16) & 0xffff) as i64);
    v = set_half(v, 3, ((prod1 >> 16) & 0xffff) as i64);
    v
}

/// Wide vector signed 32x16 multiply (`vmpyweh`/`vmpywoh`) with `:sat`/`:rnd`,
/// optionally accumulating into `acc`. `odd` selects vmpywoh (odd halves),
/// `shift` is the `:<<1` flag, `rnd` adds 0x8000 before the `>>16`.
#[allow(clippy::too_many_arguments)]
fn mmpy_ss(
    ctx: &mut SemCtx,
    rss: u64,
    rtt: u64,
    shift: u32,
    odd: bool,
    accumulate: bool,
    acc: u64,
    rnd: bool,
) -> u64 {
    // even -> halves (0,2); odd -> halves (1,3); word lane wi of Rss with that half.
    let (h0, h1) = if odd { (1u32, 3u32) } else { (0u32, 2u32) };
    let lanes = [(0u32, h0), (1u32, h1)];
    let mut out: u64 = 0;
    for &(wi, hi) in lanes.iter() {
        let prod = mpy3216ss(get_word(rss, wi), get_half(rtt, hi)) << shift;
        let r = if rnd { prod + 0x8000 } else { prod };
        let shifted = r >> 16;
        let sum = if accumulate {
            get_word(acc, wi) + shifted
        } else {
            shifted
        };
        let w = ctx.sat_n(sum, 32);
        out = set_word(out, wi, w);
    }
    out
}

/// Wide vector signed*unsigned 32x16 multiply (`vmpyweuh`/`vmpywouh`).
#[allow(clippy::too_many_arguments)]
fn mmpy_su(
    ctx: &mut SemCtx,
    rss: u64,
    rtt: u64,
    shift: u32,
    odd: bool,
    accumulate: bool,
    acc: u64,
    rnd: bool,
) -> u64 {
    let (h0, h1) = if odd { (1u32, 3u32) } else { (0u32, 2u32) };
    let lanes = [(0u32, h0), (1u32, h1)];
    let mut out: u64 = 0;
    for &(wi, hi) in lanes.iter() {
        let prod = mpy3216su(get_word(rss, wi), get_uhalf(rtt, hi)) << shift;
        let r = if rnd { prod + 0x8000 } else { prod };
        let shifted = r >> 16;
        let sum = if accumulate {
            get_word(acc, wi) + shifted
        } else {
            shifted
        };
        let w = ctx.sat_n(sum, 32);
        out = set_word(out, wi, w);
    }
    out
}
