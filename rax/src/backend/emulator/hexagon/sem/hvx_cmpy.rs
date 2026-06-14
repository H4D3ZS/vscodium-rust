//! (hvx_cmpy) HVX even/odd word multiplies (vmpyewuh / vmpyowh and variants),
//! the building blocks of the 32x32 wide and fractional multiplies. Verified
//! against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs). See
//! sem/hvx_mpy.rs for the 128-byte lane pattern and the SemCtx vector API.
//!
//! Spec: V68 HVX PRM `Vd.w=vmpye(Vu.w,Vv.uh)`, `Vd.w=vmpyo(Vu.w,Vv.h):<<1...`.
//!   even (vmpye): prod = Vu.w[i](signed) * Vv.uh[0](unsigned low half)
//!   odd  (vmpyo): prod = Vu.w[i](signed) * Vv.h[1](signed high half)

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
#[inline]
fn get_w(b: &Bytes, i: usize) -> i32 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn get_uw(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}
/// Unsigned low halfword (uh[0]) of word `i`.
#[inline]
fn vv_uh0(b: &Bytes, i: usize) -> u32 {
    (get_uw(b, i) & 0xffff) as u32
}
/// Signed high halfword (h[1]) of word `i`.
#[inline]
fn vv_h1(b: &Bytes, i: usize) -> i32 {
    (get_uw(b, i) >> 16) as u16 as i16 as i32
}

/// Execute an HVX even/odd word multiply opcode. Returns `false` if not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));

    match op {
        // Vd.w = vmpye(Vu.w, Vv.uh): prod = Vu.w[i] * Vv.uh[0]; Vd = prod >> 16.
        Opcode::V6_vmpyewuh => {
            let mut out = [0u8; 128];
            for i in 0..32 {
                let prod = (get_w(&vu, i) as i64) * (vv_uh0(&vv, i) as i64);
                set_w(&mut out, i, (prod >> 16) as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        // Vdd = vmpye(Vu.w, Vv.uh): prod = Vu.w[i] * Vv.uh[0];
        //   Vdd.v[1].w[i] = prod >> 16 (high reg); Vdd.v[0].w[i] = prod << 16 (low reg).
        Opcode::V6_vmpyewuh_64 => {
            let rd = fld(d, b'd');
            let mut lo = [0u8; 128];
            let mut hi = [0u8; 128];
            for i in 0..32 {
                let prod = (get_w(&vu, i) as i64) * (vv_uh0(&vv, i) as i64);
                set_w(&mut hi, i, (prod >> 16) as u32);
                set_w(&mut lo, i, (prod << 16) as u32);
            }
            ctx.set_v(rd, from_bytes(&lo));
            ctx.set_v(rd + 1, from_bytes(&hi));
        }

        // Vd.w = vmpyo(Vu.w, Vv.h):<<1:sat
        //   prod = Vu.w[i] * Vv.h[1]; Vd = sat32(prod >> 15)  (the <<1 folds >>16 into >>15).
        Opcode::V6_vmpyowh => {
            let mut out = [0u8; 128];
            for i in 0..32 {
                let prod = (get_w(&vu, i) as i64) * (vv_h1(&vv, i) as i64);
                let r = ctx.sat_n(prod >> 15, 32);
                set_w(&mut out, i, r as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        // Vd.w = vmpyo(Vu.w, Vv.h):<<1:rnd:sat
        //   prod = Vu.w[i] * Vv.h[1]; Vd = sat32((((prod >> 14) + 1) >> 1)).
        Opcode::V6_vmpyowh_rnd => {
            let mut out = [0u8; 128];
            for i in 0..32 {
                let prod = (get_w(&vu, i) as i64) * (vv_h1(&vv, i) as i64);
                let r = ctx.sat_n(((prod >> 14) + 1) >> 1, 32);
                set_w(&mut out, i, r as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        // Vx.w += vmpyo(Vu.w, Vv.h):<<1:sat:shift
        //   Vx = sat32((((Vx + Vu.w[i]*Vv.h[1]) >> 14)) >> 1)  (no rounding +1).
        Opcode::V6_vmpyowh_sacc => {
            let x = fld(d, b'x');
            let vx = to_bytes(&ctx.vread(x));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let acc = (get_w(&vx, i) as i64) + (get_w(&vu, i) as i64) * (vv_h1(&vv, i) as i64);
                let r = ctx.sat_n(acc >> 15, 32);
                set_w(&mut out, i, r as u32);
            }
            ctx.set_v(x, from_bytes(&out));
        }

        // Vx.w += vmpyo(Vu.w, Vv.h):<<1:rnd:sat:shift
        //   Vx = sat32((((Vx + Vu.w[i]*Vv.h[1]) >> 14) + 1) >> 1).
        Opcode::V6_vmpyowh_rnd_sacc => {
            let x = fld(d, b'x');
            let vx = to_bytes(&ctx.vread(x));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let acc = (get_w(&vx, i) as i64) + (get_w(&vu, i) as i64) * (vv_h1(&vv, i) as i64);
                let r = ctx.sat_n(((acc >> 14) + 1) >> 1, 32);
                set_w(&mut out, i, r as u32);
            }
            ctx.set_v(x, from_bytes(&out));
        }

        // Vxx += vmpyo(Vu.w, Vv.h): wide 64-bit accumulate into a vector pair.
        //   prod = Vu.w[i]*Vv.h[1] + Vxx.v[1].w[i];
        //   Vxx.v[1].w[i]      = prod >> 16;             (high reg)
        //   Vxx.v[0].w[i].h[0] = old Vxx.v[0].w[i] >> 16;
        //   Vxx.v[0].w[i].h[1] = prod & 0xffff;          (low reg repacked)
        Opcode::V6_vmpyowh_64_acc => {
            let x = fld(d, b'x');
            let vlo = to_bytes(&ctx.vread(x));
            let vhi = to_bytes(&ctx.vread(x + 1));
            let mut out_lo = [0u8; 128];
            let mut out_hi = [0u8; 128];
            for i in 0..32 {
                let prod =
                    (get_w(&vu, i) as i64) * (vv_h1(&vv, i) as i64) + (get_w(&vhi, i) as i64);
                set_w(&mut out_hi, i, (prod >> 16) as u32);
                let lo_h0 = (get_uw(&vlo, i) >> 16) & 0xffff;
                let lo_h1 = (prod as u32) & 0xffff;
                set_w(&mut out_lo, i, (lo_h1 << 16) | lo_h0);
            }
            ctx.set_v(x, from_bytes(&out_lo));
            ctx.set_v(x + 1, from_bytes(&out_hi));
        }

        _ => return false,
    }
    true
}
