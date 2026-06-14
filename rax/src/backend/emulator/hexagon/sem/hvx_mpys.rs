//! (hvx_mpys) HVX scalar multiply-add / piecewise (vmpa*) and integer
//! multiply-accumulate scalar forms (vmpyi*_acc, vmpyieoh/iewh/iewuh/iowh/iwub,
//! vmpsuhuhsat). Verified against the qemu-hexagon vector oracle
//! (tests/hexagon_hvx_diff.rs). See sem/hvx_mpy.rs for the 128-byte lane pattern
//! and the SemCtx vector API.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

type Bytes = [u8; 128];

#[inline]
fn to_bytes(v: &[u32; 32]) -> Bytes {
    let mut b = [0u8; 128];
    for i in 0..32 {
        b[i * 4..i * 4 + 4].copy_from_slice(&v[i].to_le_bytes());
    }
    b
}
#[inline]
fn from_bytes(b: &Bytes) -> [u32; 32] {
    let mut v = [0u32; 32];
    for i in 0..32 {
        v[i] = u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]);
    }
    v
}

// --- per-element little-endian accessors over a 128-byte vector --------------

/// Unsigned byte `i` (0..127).
#[inline]
fn ub(b: &Bytes, i: usize) -> i64 {
    b[i] as i64
}
/// Signed byte `i` (0..127).
#[inline]
fn sb(b: &Bytes, i: usize) -> i64 {
    b[i] as i8 as i64
}
/// Signed halfword `i` (0..63).
#[inline]
fn sh(b: &Bytes, i: usize) -> i64 {
    i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i64
}
/// Unsigned halfword `i` (0..63).
#[inline]
fn uh(b: &Bytes, i: usize) -> i64 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i64
}
/// Raw unsigned halfword `i` (0..63), as u16 (for bit-field extraction).
#[inline]
fn raw_h(b: &Bytes, i: usize) -> u16 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
/// Signed word `i` (0..31).
#[inline]
fn sw(b: &Bytes, i: usize) -> i64 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]) as i64
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn get_h_signed(b: &Bytes, i: usize) -> i64 {
    sh(b, i)
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn get_w_signed(b: &Bytes, i: usize) -> i64 {
    sw(b, i)
}

/// Signed byte `n` (0..3) of a scalar word.
#[inline]
fn rt_sb(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 8)) & 0xff) as u8 as i8 as i64
}
/// Unsigned byte `n` (0..3) of a scalar word.
#[inline]
fn rt_ub(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 8)) & 0xff) as i64
}
/// Signed halfword `n` (0..1) of a scalar word.
#[inline]
fn rt_sh(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 16)) & 0xffff) as u16 as i16 as i64
}
/// Signed halfword `n` (0..3) of a 64-bit scalar pair.
#[inline]
fn rtt_sh(rtt: u64, n: usize) -> i64 {
    ((rtt >> (n * 16)) & 0xffff) as u16 as i16 as i64
}
/// Unsigned halfword `n` (0..3) of a 64-bit scalar pair.
#[inline]
fn rtt_uh(rtt: u64, n: usize) -> i64 {
    ((rtt >> (n * 16)) & 0xffff) as i64
}

/// Read a vector register pair `(low=R, high=R+1)` as two 128-byte buffers.
#[inline]
fn vread_pair(ctx: &SemCtx, reg: u8) -> (Bytes, Bytes) {
    (to_bytes(&ctx.vread(reg)), to_bytes(&ctx.vread(reg + 1)))
}

/// Write a vector register pair (low half -> R, high half -> R+1).
#[inline]
fn set_pair(ctx: &mut SemCtx, reg: u8, lo: &Bytes, hi: &Bytes) {
    ctx.set_v(reg, from_bytes(lo));
    ctx.set_v(reg + 1, from_bytes(hi));
}

/// Execute a hvx_mpys opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // --- vmpa: byte multiply-add, pair source Vuu, scalar Rt (halfword out) ---
        // Vdd.h = vmpa(Vuu.ub, Rt.b)   /   Vxx.h += ...
        Opcode::V6_vmpabus | Opcode::V6_vmpabus_acc => {
            let pair = matches!(op, Opcode::V6_vmpabus_acc);
            let (uu0, uu1) = vread_pair(ctx, fld(d, b'u'));
            let rt = ctx.r(fld(d, b't'));
            let dreg = if pair { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if pair {
                vread_pair(ctx, dreg)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let v0 = (ub(&uu0, 2 * i) * rt_sb(rt, 0)) + (ub(&uu1, 2 * i) * rt_sb(rt, 1));
                let v1 =
                    (ub(&uu0, 2 * i + 1) * rt_sb(rt, 2)) + (ub(&uu1, 2 * i + 1) * rt_sb(rt, 3));
                if pair {
                    let (o0, o1) = (get_h_signed(&lo, i), get_h_signed(&hi, i));
                    set_h(&mut lo, i, (o0 + v0) as u16);
                    set_h(&mut hi, i, (o1 + v1) as u16);
                } else {
                    set_h(&mut lo, i, v0 as u16);
                    set_h(&mut hi, i, v1 as u16);
                }
            }
            set_pair(ctx, dreg, &lo, &hi);
        }
        // Vdd.h = vmpa(Vuu.ub, Rt.ub)  /  Vxx.h += ...  (unsigned scalar bytes)
        Opcode::V6_vmpabuu | Opcode::V6_vmpabuu_acc => {
            let pair = matches!(op, Opcode::V6_vmpabuu_acc);
            let (uu0, uu1) = vread_pair(ctx, fld(d, b'u'));
            let rt = ctx.r(fld(d, b't'));
            let dreg = if pair { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if pair {
                vread_pair(ctx, dreg)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let v0 = (ub(&uu0, 2 * i) * rt_ub(rt, 0)) + (ub(&uu1, 2 * i) * rt_ub(rt, 1));
                let v1 =
                    (ub(&uu0, 2 * i + 1) * rt_ub(rt, 2)) + (ub(&uu1, 2 * i + 1) * rt_ub(rt, 3));
                if pair {
                    // .uh accumulate: wraps in 16 bits.
                    let (o0, o1) = (raw_h(&lo, i) as i64, raw_h(&hi, i) as i64);
                    set_h(&mut lo, i, (o0 + v0) as u16);
                    set_h(&mut hi, i, (o1 + v1) as u16);
                } else {
                    set_h(&mut lo, i, v0 as u16);
                    set_h(&mut hi, i, v1 as u16);
                }
            }
            set_pair(ctx, dreg, &lo, &hi);
        }
        // Vdd.w = vmpa(Vuu.h, Rt.b)   /   Vxx.w += ...  (signed halfword * signed byte -> word)
        Opcode::V6_vmpahb | Opcode::V6_vmpahb_acc => {
            let pair = matches!(op, Opcode::V6_vmpahb_acc);
            let (uu0, uu1) = vread_pair(ctx, fld(d, b'u'));
            let rt = ctx.r(fld(d, b't'));
            let dreg = if pair { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if pair {
                vread_pair(ctx, dreg)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                // word i; h[0] = halfword 2i, h[1] = halfword 2i+1
                let v0 = (sh(&uu0, 2 * i) * rt_sb(rt, 0)) + (sh(&uu1, 2 * i) * rt_sb(rt, 1));
                let v1 =
                    (sh(&uu0, 2 * i + 1) * rt_sb(rt, 2)) + (sh(&uu1, 2 * i + 1) * rt_sb(rt, 3));
                if pair {
                    let (o0, o1) = (get_w_signed(&lo, i), get_w_signed(&hi, i));
                    set_w(&mut lo, i, (o0 + v0) as u32);
                    set_w(&mut hi, i, (o1 + v1) as u32);
                } else {
                    set_w(&mut lo, i, v0 as u32);
                    set_w(&mut hi, i, v1 as u32);
                }
            }
            set_pair(ctx, dreg, &lo, &hi);
        }
        // Vdd.w = vmpa(Vuu.uh, Rt.b)  /  Vxx.w += ...  (unsigned halfword * signed byte -> word)
        Opcode::V6_vmpauhb | Opcode::V6_vmpauhb_acc => {
            let pair = matches!(op, Opcode::V6_vmpauhb_acc);
            let (uu0, uu1) = vread_pair(ctx, fld(d, b'u'));
            let rt = ctx.r(fld(d, b't'));
            let dreg = if pair { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if pair {
                vread_pair(ctx, dreg)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                let v0 = (uh(&uu0, 2 * i) * rt_sb(rt, 0)) + (uh(&uu1, 2 * i) * rt_sb(rt, 1));
                let v1 =
                    (uh(&uu0, 2 * i + 1) * rt_sb(rt, 2)) + (uh(&uu1, 2 * i + 1) * rt_sb(rt, 3));
                if pair {
                    let (o0, o1) = (get_w_signed(&lo, i), get_w_signed(&hi, i));
                    set_w(&mut lo, i, (o0 + v0) as u32);
                    set_w(&mut hi, i, (o1 + v1) as u32);
                } else {
                    set_w(&mut lo, i, v0 as u32);
                    set_w(&mut hi, i, v1 as u32);
                }
            }
            set_pair(ctx, dreg, &lo, &hi);
        }
        // Vdd.h = vmpa(Vuu.ub, Vvv.b): pair source Vuu (unsigned byte) * pair source Vvv (signed byte).
        Opcode::V6_vmpabusv => {
            let (uu0, uu1) = vread_pair(ctx, fld(d, b'u'));
            let (vv0, vv1) = vread_pair(ctx, fld(d, b'v'));
            let dreg = fld(d, b'd');
            let mut lo = [0u8; 128];
            let mut hi = [0u8; 128];
            for i in 0..64 {
                let v0 = (ub(&uu0, 2 * i) * sb(&vv0, 2 * i)) + (ub(&uu1, 2 * i) * sb(&vv1, 2 * i));
                let v1 = (ub(&uu0, 2 * i + 1) * sb(&vv0, 2 * i + 1))
                    + (ub(&uu1, 2 * i + 1) * sb(&vv1, 2 * i + 1));
                set_h(&mut lo, i, v0 as u16);
                set_h(&mut hi, i, v1 as u16);
            }
            set_pair(ctx, dreg, &lo, &hi);
        }
        // Vdd.h = vmpa(Vuu.ub, Vvv.ub): pair source Vuu (ub) * pair source Vvv (ub).
        Opcode::V6_vmpabuuv => {
            let (uu0, uu1) = vread_pair(ctx, fld(d, b'u'));
            let (vv0, vv1) = vread_pair(ctx, fld(d, b'v'));
            let dreg = fld(d, b'd');
            let mut lo = [0u8; 128];
            let mut hi = [0u8; 128];
            for i in 0..64 {
                let v0 = (ub(&uu0, 2 * i) * ub(&vv0, 2 * i)) + (ub(&uu1, 2 * i) * ub(&vv1, 2 * i));
                let v1 = (ub(&uu0, 2 * i + 1) * ub(&vv0, 2 * i + 1))
                    + (ub(&uu1, 2 * i + 1) * ub(&vv1, 2 * i + 1));
                set_h(&mut lo, i, v0 as u16);
                set_h(&mut hi, i, v1 as u16);
            }
            set_pair(ctx, dreg, &lo, &hi);
        }
        // --- piecewise multiply with 64-bit scalar, saturating to 16 bits ---
        // Vx.h = vmpa(Vx.h, Vu.h, Rtt.h):sat
        Opcode::V6_vmpahhsat => {
            let vx_in = to_bytes(&ctx.vread(fld(d, b'x')));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rtt = ctx.rp(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = [0u8; 128];
            for i in 0..64 {
                let x = sh(&vx_in, i);
                let u = sh(&vu, i);
                let idx = ((raw_h(&vu, i) >> 14) & 0x3) as usize;
                let prod = ((x * u) << 1) + (rtt_sh(rtt, idx) << 15);
                let r = ctx.sat_n(prod >> 16, 16);
                set_h(&mut out, i, r as u16);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vx.h = vmpa(Vx.h, Vu.uh, Rtt.uh):sat
        Opcode::V6_vmpauhuhsat => {
            let vx_in = to_bytes(&ctx.vread(fld(d, b'x')));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rtt = ctx.rp(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = [0u8; 128];
            for i in 0..64 {
                let x = sh(&vx_in, i);
                let u = uh(&vu, i);
                let idx = ((raw_h(&vu, i) >> 14) & 0x3) as usize;
                let prod = (x * u) + (rtt_uh(rtt, idx) << 15);
                let r = ctx.sat_n(prod >> 16, 16);
                set_h(&mut out, i, r as u16);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vx.h = vmps(Vx.h, Vu.uh, Rtt.uh):sat
        Opcode::V6_vmpsuhuhsat => {
            let vx_in = to_bytes(&ctx.vread(fld(d, b'x')));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rtt = ctx.rp(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = [0u8; 128];
            for i in 0..64 {
                let x = sh(&vx_in, i);
                let u = uh(&vu, i);
                let idx = ((raw_h(&vu, i) >> 14) & 0x3) as usize;
                let prod = (x * u) - (rtt_uh(rtt, idx) << 15);
                let r = ctx.sat_n(prod >> 16, 16);
                set_h(&mut out, i, r as u16);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // --- integer multiply-accumulate scalar / mixed-half forms ---
        // Vx.h += vmpyi(Vu.h, Vv.h)
        Opcode::V6_vmpyih_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..64 {
                let acc = get_h_signed(&out, i) + sh(&vu, i) * sh(&vv, i);
                set_h(&mut out, i, acc as u16);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vx.h += vmpyi(Vu.h, Rt.b)
        Opcode::V6_vmpyihb_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..64 {
                let acc = get_h_signed(&out, i) + sh(&vu, i) * rt_sb(rt, i % 4);
                set_h(&mut out, i, acc as u16);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vx.w += vmpyi(Vu.w, Rt.b)
        Opcode::V6_vmpyiwb_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..32 {
                let acc = get_w_signed(&out, i) + sw(&vu, i) * rt_sb(rt, i % 4);
                set_w(&mut out, i, acc as u32);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vx.w += vmpyi(Vu.w, Rt.h)
        Opcode::V6_vmpyiwh_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..32 {
                let acc = get_w_signed(&out, i) + sw(&vu, i) * rt_sh(rt, i % 2);
                set_w(&mut out, i, acc as u32);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vd.w = vmpyi(Vu.w, Rt.ub)
        Opcode::V6_vmpyiwub => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let dreg = fld(d, b'd');
            let mut out = [0u8; 128];
            for i in 0..32 {
                let p = sw(&vu, i) * rt_ub(rt, i % 4);
                set_w(&mut out, i, p as u32);
            }
            ctx.set_v(dreg, from_bytes(&out));
        }
        // Vx.w += vmpyi(Vu.w, Rt.ub)
        Opcode::V6_vmpyiwub_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..32 {
                let acc = get_w_signed(&out, i) + sw(&vu, i) * rt_ub(rt, i % 4);
                set_w(&mut out, i, acc as u32);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vd.w = vmpyieo(Vu.h, Vv.h): (Vu.w[i].h[0] * Vv.w[i].h[1]) << 16
        Opcode::V6_vmpyieoh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let dreg = fld(d, b'd');
            let mut out = [0u8; 128];
            for i in 0..32 {
                // word i: h[0] = halfword 2i, h[1] = halfword 2i+1
                let p = (sh(&vu, 2 * i) * sh(&vv, 2 * i + 1)) << 16;
                set_w(&mut out, i, p as u32);
            }
            ctx.set_v(dreg, from_bytes(&out));
        }
        // Vd.w = vmpyie(Vu.w, Vv.uh): Vu.w[i] * Vv.w[i].uh[0]
        Opcode::V6_vmpyiewuh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let dreg = fld(d, b'd');
            let mut out = [0u8; 128];
            for i in 0..32 {
                // uh[0] = low (even) halfword of word i = halfword 2i
                let p = sw(&vu, i) * uh(&vv, 2 * i);
                set_w(&mut out, i, p as u32);
            }
            ctx.set_v(dreg, from_bytes(&out));
        }
        // Vx.w += vmpyie(Vu.w, Vv.uh): Vx.w[i] + (Vu.w[i] * Vv.w[i].uh[0])
        Opcode::V6_vmpyiewuh_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..32 {
                let acc = get_w_signed(&out, i) + sw(&vu, i) * uh(&vv, 2 * i);
                set_w(&mut out, i, acc as u32);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vx.w += vmpyie(Vu.w, Vv.h): Vx.w[i] + (Vu.w[i] * Vv.w[i].h[0])
        Opcode::V6_vmpyiewh_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let xreg = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(xreg));
            for i in 0..32 {
                // h[0] = low (even) signed halfword of word i = halfword 2i
                let acc = get_w_signed(&out, i) + sw(&vu, i) * sh(&vv, 2 * i);
                set_w(&mut out, i, acc as u32);
            }
            ctx.set_v(xreg, from_bytes(&out));
        }
        // Vd.w = vmpyio(Vu.w, Vv.h): Vu.w[i] * Vv.w[i].h[1]
        Opcode::V6_vmpyiowh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let dreg = fld(d, b'd');
            let mut out = [0u8; 128];
            for i in 0..32 {
                // h[1] = high (odd) signed halfword of word i = halfword 2i+1
                let p = sw(&vu, i) * sh(&vv, 2 * i + 1);
                set_w(&mut out, i, p as u32);
            }
            ctx.set_v(dreg, from_bytes(&out));
        }
        _ => return false,
    }
    true
}
