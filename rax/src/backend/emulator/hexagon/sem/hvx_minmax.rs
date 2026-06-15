//! (hvx_minmax) HVX per-lane min/max, average, negative-average, absolute value
//! and absolute-difference. Verified against the qemu-hexagon vector oracle.
//! Semantics: fVAVGS=(a+b)>>1 signed, fVAVGSRND=(a+b+1)>>1, fVAVGU unsigned,
//! fVNAVGS=(a-b)>>1, fABS, fVSAT*.

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
fn gh(b: &Bytes, i: usize) -> u16 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
#[inline]
fn sh(b: &mut Bytes, i: usize, v: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&v.to_le_bytes());
}
#[inline]
fn gw(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn sw(b: &mut Bytes, i: usize, v: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
}

fn mb(a: &Bytes, c: &Bytes, f: impl Fn(u8, u8) -> u8) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..128 {
        o[i] = f(a[i], c[i]);
    }
    o
}
fn mh(a: &Bytes, c: &Bytes, f: impl Fn(u16, u16) -> u16) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..64 {
        sh(&mut o, i, f(gh(a, i), gh(c, i)));
    }
    o
}
fn mw(a: &Bytes, c: &Bytes, f: impl Fn(u32, u32) -> u32) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..32 {
        sw(&mut o, i, f(gw(a, i), gw(c, i)));
    }
    o
}
fn ub(a: &Bytes, f: impl Fn(u8) -> u8) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..128 {
        o[i] = f(a[i]);
    }
    o
}
fn uh(a: &Bytes, f: impl Fn(u16) -> u16) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..64 {
        sh(&mut o, i, f(gh(a, i)));
    }
    o
}
fn uw(a: &Bytes, f: impl Fn(u32) -> u32) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..32 {
        sw(&mut o, i, f(gw(a, i)));
    }
    o
}

/// signed average: `(a + b + rnd) >> 1` in wide precision.
#[inline]
fn avg(a: i64, b: i64, rnd: i64) -> i64 {
    (a + b + rnd) >> 1
}

pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let rd = fld(d, b'd');
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));

    let out = match op {
        // ---- signed min/max ----
        Opcode::V6_vmaxb => mb(&vu, &vv, |a, b| (a as i8).max(b as i8) as u8),
        Opcode::V6_vmaxh => mh(&vu, &vv, |a, b| (a as i16).max(b as i16) as u16),
        Opcode::V6_vmaxw => mw(&vu, &vv, |a, b| (a as i32).max(b as i32) as u32),
        Opcode::V6_vminb => mb(&vu, &vv, |a, b| (a as i8).min(b as i8) as u8),
        Opcode::V6_vminh => mh(&vu, &vv, |a, b| (a as i16).min(b as i16) as u16),
        Opcode::V6_vminw => mw(&vu, &vv, |a, b| (a as i32).min(b as i32) as u32),
        // ---- unsigned min/max ----
        Opcode::V6_vmaxub => mb(&vu, &vv, |a, b| a.max(b)),
        Opcode::V6_vmaxuh => mh(&vu, &vv, |a, b| a.max(b)),
        Opcode::V6_vminub => mb(&vu, &vv, |a, b| a.min(b)),
        Opcode::V6_vminuh => mh(&vu, &vv, |a, b| a.min(b)),
        // ---- signed average (a+b)>>1 ----
        Opcode::V6_vavgb => mb(&vu, &vv, |a, b| {
            avg(a as i8 as i64, b as i8 as i64, 0) as u8
        }),
        Opcode::V6_vavgh => mh(&vu, &vv, |a, b| {
            avg(a as i16 as i64, b as i16 as i64, 0) as u16
        }),
        Opcode::V6_vavgw => mw(&vu, &vv, |a, b| {
            avg(a as i32 as i64, b as i32 as i64, 0) as u32
        }),
        Opcode::V6_vavgbrnd => mb(&vu, &vv, |a, b| {
            avg(a as i8 as i64, b as i8 as i64, 1) as u8
        }),
        Opcode::V6_vavghrnd => mh(&vu, &vv, |a, b| {
            avg(a as i16 as i64, b as i16 as i64, 1) as u16
        }),
        Opcode::V6_vavgwrnd => mw(&vu, &vv, |a, b| {
            avg(a as i32 as i64, b as i32 as i64, 1) as u32
        }),
        // ---- unsigned average ----
        Opcode::V6_vavgub => mb(&vu, &vv, |a, b| avg(a as i64, b as i64, 0) as u8),
        Opcode::V6_vavguh => mh(&vu, &vv, |a, b| avg(a as i64, b as i64, 0) as u16),
        Opcode::V6_vavguw => mw(&vu, &vv, |a, b| avg(a as i64, b as i64, 0) as u32),
        Opcode::V6_vavgubrnd => mb(&vu, &vv, |a, b| avg(a as i64, b as i64, 1) as u8),
        Opcode::V6_vavguhrnd => mh(&vu, &vv, |a, b| avg(a as i64, b as i64, 1) as u16),
        Opcode::V6_vavguwrnd => mw(&vu, &vv, |a, b| avg(a as i64, b as i64, 1) as u32),
        // ---- negative average (a-b)>>1 ----
        Opcode::V6_vnavgb => mb(&vu, &vv, |a, b| {
            ((a as i8 as i64 - b as i8 as i64) >> 1) as u8
        }),
        Opcode::V6_vnavgh => mh(&vu, &vv, |a, b| {
            ((a as i16 as i64 - b as i16 as i64) >> 1) as u16
        }),
        Opcode::V6_vnavgw => mw(&vu, &vv, |a, b| {
            ((a as i32 as i64 - b as i32 as i64) >> 1) as u32
        }),
        Opcode::V6_vnavgub => mb(&vu, &vv, |a, b| ((a as i64 - b as i64) >> 1) as u8),
        // ---- absolute value ----
        Opcode::V6_vabsb => ub(&vu, |a| (a as i8).wrapping_abs() as u8),
        Opcode::V6_vabsh => uh(&vu, |a| (a as i16).wrapping_abs() as u16),
        Opcode::V6_vabsw => uw(&vu, |a| (a as i32).wrapping_abs() as u32),
        Opcode::V6_vabsb_sat => ub(&vu, |a| (a as i8 as i32).abs().min(127) as u8),
        Opcode::V6_vabsh_sat => uh(&vu, |a| (a as i16 as i32).abs().min(32767) as u16),
        Opcode::V6_vabsw_sat => uw(&vu, |a| (a as i32 as i64).abs().min(i32::MAX as i64) as u32),
        // ---- absolute difference (result is the magnitude, unsigned lane) ----
        Opcode::V6_vabsdiffh => mh(&vu, &vv, |a, b| {
            (a as i16 as i32 - b as i16 as i32).unsigned_abs() as u16
        }),
        Opcode::V6_vabsdiffw => mw(&vu, &vv, |a, b| {
            (a as i32 as i64 - b as i32 as i64).unsigned_abs() as u32
        }),
        Opcode::V6_vabsdiffub => mb(&vu, &vv, |a, b| if a > b { a - b } else { b - a }),
        Opcode::V6_vabsdiffuh => mh(&vu, &vv, |a, b| if a > b { a - b } else { b - a }),
        _ => return false,
    };
    ctx.set_v(rd, from_bytes(&out));
    true
}
