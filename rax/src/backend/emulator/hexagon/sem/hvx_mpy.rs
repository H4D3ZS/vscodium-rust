//! (hvx_mpy) HVX integer multiplies. The straightforward same-width truncated
//! `vmpyi` forms (16x16->16, 16x8->16, 32x8->32, 32x16->32); the widening /
//! shifting / saturating / vector-pair multiply forms are left for a later pass.
//! Verified against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).

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
fn get_h(b: &Bytes, i: usize) -> i16 {
    i16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn get_w(b: &Bytes, i: usize) -> i32 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}
/// Signed byte `n` (0..3) of a scalar word.
#[inline]
fn rt_byte(rt: u32, n: usize) -> i32 {
    ((rt >> (n * 8)) & 0xff) as u8 as i8 as i32
}
/// Signed halfword `n` (0..1) of a scalar word.
#[inline]
fn rt_half(rt: u32, n: usize) -> i32 {
    ((rt >> (n * 16)) & 0xffff) as u16 as i16 as i32
}

/// Execute an HVX multiply opcode. Returns `false` if not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let rd = fld(d, b'd');
    let mut out = [0u8; 128];

    match op {
        // Vd.h = vmpyi(Vu.h, Vv.h): per-halfword 16x16 -> low 16 bits.
        Opcode::V6_vmpyih => {
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            for i in 0..64 {
                let p = get_h(&vu, i) as i32 * get_h(&vv, i) as i32;
                set_h(&mut out, i, p as u16);
            }
        }
        // Vd.h = vmpyi(Vu.h, Rt.b): scalar byte (lane i uses byte i%4).
        Opcode::V6_vmpyihb => {
            let rt = ctx.r(fld(d, b't'));
            for i in 0..64 {
                let p = get_h(&vu, i) as i32 * rt_byte(rt, i % 4);
                set_h(&mut out, i, p as u16);
            }
        }
        // Vd.w = vmpyi(Vu.w, Rt.b): per-word 32x8 -> low 32 bits.
        Opcode::V6_vmpyiwb => {
            let rt = ctx.r(fld(d, b't'));
            for i in 0..32 {
                let p = (get_w(&vu, i) as i64) * (rt_byte(rt, i % 4) as i64);
                set_w(&mut out, i, p as u32);
            }
        }
        // Vd.w = vmpyi(Vu.w, Rt.h): per-word 32x16 -> low 32 bits.
        Opcode::V6_vmpyiwh => {
            let rt = ctx.r(fld(d, b't'));
            for i in 0..32 {
                let p = (get_w(&vu, i) as i64) * (rt_half(rt, i % 2) as i64);
                set_w(&mut out, i, p as u32);
            }
        }
        _ => return false,
    }
    ctx.set_v(rd, from_bytes(&out));
    true
}
