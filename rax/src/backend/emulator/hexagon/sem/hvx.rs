//! HVX (Hexagon Vector eXtensions) instructions — 1024-bit (128-byte) vector
//! ops over V0..V31. This is the bring-up: per-lane vector add/sub for byte,
//! halfword and word lanes (signed/unsigned saturating and average variants).
//! Semantics from the V68 spec (`fVFOREACH(W,i){ Vd.elem[i] = ... }`); verified
//! against the qemu-hexagon HVX oracle (tests/hexagon_hvx_diff.rs).

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

/// 128-byte vector viewed as raw bytes (little-endian within each u32 word).
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
fn get_h(b: &Bytes, i: usize) -> u16 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn get_w(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Apply a per-byte binary op across all 128 byte lanes.
fn map_b(a: &Bytes, c: &Bytes, f: impl Fn(u8, u8) -> u8) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..128 {
        o[i] = f(a[i], c[i]);
    }
    o
}
/// Apply a per-halfword op across all 64 halfword lanes.
fn map_h(a: &Bytes, c: &Bytes, f: impl Fn(u16, u16) -> u16) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..64 {
        set_h(&mut o, i, f(get_h(a, i), get_h(c, i)));
    }
    o
}
/// Apply a per-word op across all 32 word lanes.
fn map_w(a: &Bytes, c: &Bytes, f: impl Fn(u32, u32) -> u32) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..32 {
        set_w(&mut o, i, f(get_w(a, i), get_w(c, i)));
    }
    o
}

/// Execute an HVX opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
    let rd = fld(d, b'd');

    let out = match op {
        // ---- non-saturating add ----
        Opcode::V6_vaddb => map_b(&vu, &vv, |a, b| a.wrapping_add(b)),
        Opcode::V6_vaddh => map_h(&vu, &vv, |a, b| a.wrapping_add(b)),
        Opcode::V6_vaddw => map_w(&vu, &vv, |a, b| a.wrapping_add(b)),
        // ---- non-saturating sub ----
        Opcode::V6_vsubb => map_b(&vu, &vv, |a, b| a.wrapping_sub(b)),
        Opcode::V6_vsubh => map_h(&vu, &vv, |a, b| a.wrapping_sub(b)),
        Opcode::V6_vsubw => map_w(&vu, &vv, |a, b| a.wrapping_sub(b)),
        // ---- signed saturating add/sub ----
        Opcode::V6_vaddbsat => map_b(&vu, &vv, |a, b| {
            (a as i8 as i16 + b as i8 as i16).clamp(-128, 127) as u8
        }),
        Opcode::V6_vaddhsat => map_h(&vu, &vv, |a, b| {
            (a as i16 as i32 + b as i16 as i32).clamp(-32768, 32767) as u16
        }),
        Opcode::V6_vaddwsat => map_w(&vu, &vv, |a, b| {
            (a as i32 as i64 + b as i32 as i64).clamp(i32::MIN as i64, i32::MAX as i64) as u32
        }),
        Opcode::V6_vsubbsat => map_b(&vu, &vv, |a, b| {
            (a as i8 as i16 - b as i8 as i16).clamp(-128, 127) as u8
        }),
        Opcode::V6_vsubhsat => map_h(&vu, &vv, |a, b| {
            (a as i16 as i32 - b as i16 as i32).clamp(-32768, 32767) as u16
        }),
        Opcode::V6_vsubwsat => map_w(&vu, &vv, |a, b| {
            (a as i32 as i64 - b as i32 as i64).clamp(i32::MIN as i64, i32::MAX as i64) as u32
        }),
        // ---- unsigned saturating add/sub ----
        Opcode::V6_vaddubsat => map_b(&vu, &vv, |a, b| (a as u16 + b as u16).min(255) as u8),
        Opcode::V6_vadduhsat => map_h(&vu, &vv, |a, b| (a as u32 + b as u32).min(65535) as u16),
        Opcode::V6_vadduwsat => map_w(&vu, &vv, |a, b| a.checked_add(b).unwrap_or(u32::MAX)),
        Opcode::V6_vsububsat => map_b(&vu, &vv, |a, b| a.saturating_sub(b)),
        Opcode::V6_vsubuhsat => map_h(&vu, &vv, |a, b| a.saturating_sub(b)),
        // ---- bitwise ----
        Opcode::V6_vand => map_w(&vu, &vv, |a, b| a & b),
        Opcode::V6_vor => map_w(&vu, &vv, |a, b| a | b),
        Opcode::V6_vxor => map_w(&vu, &vv, |a, b| a ^ b),

        _ => return false,
    };
    ctx.set_v(rd, from_bytes(&out));
    true
}
