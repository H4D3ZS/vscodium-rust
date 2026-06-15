//! (hvx_rmpy) HVX reduction multiplies: vrmpy / vdmpy / vtmpy / vdsad / vrsad.
//! Each output lane is a dot-product / sliding-window reduction of several
//! sub-products. Matched element-for-element to the V68 HVX PRM pseudocode and
//! verified against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_u, fld};

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
/// Unsigned byte `i` (0..127) of the 128-byte vector.
#[inline]
fn ub(b: &Bytes, i: usize) -> i64 {
    b[i] as i64
}
/// Signed byte `i` (0..127) of the 128-byte vector.
#[inline]
fn sb(b: &Bytes, i: usize) -> i64 {
    b[i] as i8 as i64
}
/// Signed halfword `i` (0..63).
#[inline]
fn get_h(b: &Bytes, i: usize) -> i64 {
    i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i64
}
/// Unsigned halfword `i` (0..63).
#[inline]
fn get_uh(b: &Bytes, i: usize) -> i64 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i64
}
/// Signed word `i` (0..31).
#[inline]
fn get_w(b: &Bytes, i: usize) -> i64 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]) as i64
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}
/// Unsigned byte `n` (0..3) of a scalar word.
#[inline]
fn rt_ub(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 8)) & 0xff) as i64
}
/// Signed byte `n` (0..3) of a scalar word.
#[inline]
fn rt_sb(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 8)) & 0xff) as u8 as i8 as i64
}
/// Signed halfword `n` (0..1) of a scalar word.
#[inline]
fn rt_h(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 16)) & 0xffff) as u16 as i16 as i64
}
/// Unsigned halfword `n` (0..1) of a scalar word.
#[inline]
fn rt_uh(rt: u32, n: usize) -> i64 {
    ((rt >> (n * 16)) & 0xffff) as i64
}

/// Execute a hvx_rmpy opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vrmpy: scalar, single vector, 4-wide byte dot product (uw/w lane) ----
        Opcode::V6_vrmpyub | Opcode::V6_vrmpyub_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = op == Opcode::V6_vrmpyub_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = if acc { get_w(&out, i) } else { 0 };
                for k in 0..4 {
                    s += ub(&vu, i * 4 + k) * rt_ub(rt, k);
                }
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        Opcode::V6_vrmpybus | Opcode::V6_vrmpybus_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = op == Opcode::V6_vrmpybus_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = if acc { get_w(&out, i) } else { 0 };
                for k in 0..4 {
                    s += ub(&vu, i * 4 + k) * rt_sb(rt, k);
                }
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vrmpy: vector-vector, 4-wide byte dot product ----
        Opcode::V6_vrmpyubv | Opcode::V6_vrmpyubv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = op == Opcode::V6_vrmpyubv_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = if acc { get_w(&out, i) } else { 0 };
                for k in 0..4 {
                    s += ub(&vu, i * 4 + k) * ub(&vv, i * 4 + k);
                }
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        Opcode::V6_vrmpybv | Opcode::V6_vrmpybv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = op == Opcode::V6_vrmpybv_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = if acc { get_w(&out, i) } else { 0 };
                for k in 0..4 {
                    s += sb(&vu, i * 4 + k) * sb(&vv, i * 4 + k);
                }
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        Opcode::V6_vrmpybusv | Opcode::V6_vrmpybusv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = op == Opcode::V6_vrmpybusv_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = if acc { get_w(&out, i) } else { 0 };
                for k in 0..4 {
                    s += ub(&vu, i * 4 + k) * sb(&vv, i * 4 + k);
                }
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vrmpy: pair source/dest, scalar byte, with #u1 byte-lane rotate ----
        Opcode::V6_vrmpyubi | Opcode::V6_vrmpyubi_acc => {
            rmpy_pair_imm(op, d, ctx, Opcode::V6_vrmpyubi_acc, true);
        }
        Opcode::V6_vrmpybusi | Opcode::V6_vrmpybusi_acc => {
            rmpy_pair_imm(op, d, ctx, Opcode::V6_vrmpybusi_acc, false);
        }
        // ---- vrsad: pair, sum of absolute differences, #u1 byte-lane rotate ----
        Opcode::V6_vrsadubi | Opcode::V6_vrsadubi_acc => {
            let acc = op == Opcode::V6_vrsadubi_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let imm = fimm_u(d, b'i', ctx.immext) as usize & 1;
            let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut o0 = if acc {
                to_bytes(&ctx.vread(dbase))
            } else {
                [0u8; 128]
            };
            let mut o1 = if acc {
                to_bytes(&ctx.vread(dbase + 1))
            } else {
                [0u8; 128]
            };
            let lo = |sel: usize| if sel == 0 { &v0 } else { &v1 };
            for i in 0..32 {
                let base = i * 4;
                // Vdd.v[0]
                let mut s0 = if acc { get_w(&o0, i) } else { 0 };
                s0 += (ub(lo(imm), base + 0) - rt_ub(rt, (0usize.wrapping_sub(imm)) & 3)).abs();
                s0 += (ub(&v0, base + 1) - rt_ub(rt, (1usize.wrapping_sub(imm)) & 3)).abs();
                s0 += (ub(&v0, base + 2) - rt_ub(rt, (2usize.wrapping_sub(imm)) & 3)).abs();
                s0 += (ub(&v0, base + 3) - rt_ub(rt, (3usize.wrapping_sub(imm)) & 3)).abs();
                set_w(&mut o0, i, s0 as u32);
                // Vdd.v[1]
                let mut s1 = if acc { get_w(&o1, i) } else { 0 };
                s1 += (ub(&v1, base + 0) - rt_ub(rt, (2usize.wrapping_sub(imm)) & 3)).abs();
                s1 += (ub(&v1, base + 1) - rt_ub(rt, (3usize.wrapping_sub(imm)) & 3)).abs();
                s1 += (ub(lo(imm), base + 2) - rt_ub(rt, (0usize.wrapping_sub(imm)) & 3)).abs();
                s1 += (ub(&v0, base + 3) - rt_ub(rt, (1usize.wrapping_sub(imm)) & 3)).abs();
                set_w(&mut o1, i, s1 as u32);
            }
            ctx.set_v(dbase, from_bytes(&o0));
            ctx.set_v(dbase + 1, from_bytes(&o1));
        }
        // ---- vdsad: pair, sum-of-abs-diff of halfwords, sliding window ----
        Opcode::V6_vdsaduh | Opcode::V6_vdsaduh_acc => {
            let acc = op == Opcode::V6_vdsaduh_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let r0 = rt_uh(rt, 0);
            let r1 = rt_uh(rt, 1);
            let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut o0 = if acc {
                to_bytes(&ctx.vread(dbase))
            } else {
                [0u8; 128]
            };
            let mut o1 = if acc {
                to_bytes(&ctx.vread(dbase + 1))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                // each word lane holds two unsigned halfwords (h[0], h[1])
                let a0 = get_uh(&v0, i * 2 + 0);
                let a1 = get_uh(&v0, i * 2 + 1);
                let b0 = get_uh(&v1, i * 2 + 0);
                let mut s0 = if acc { get_w(&o0, i) } else { 0 };
                s0 += (a0 - r0).abs();
                s0 += (a1 - r1).abs();
                set_w(&mut o0, i, s0 as u32);
                let mut s1 = if acc { get_w(&o1, i) } else { 0 };
                s1 += (a1 - r0).abs();
                s1 += (b0 - r1).abs();
                set_w(&mut o1, i, s1 as u32);
            }
            ctx.set_v(dbase, from_bytes(&o0));
            ctx.set_v(dbase + 1, from_bytes(&o1));
        }
        // ---- vdmpybus: single vector, 2-wide, ub*b -> h ----
        Opcode::V6_vdmpybus | Opcode::V6_vdmpybus_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = op == Opcode::V6_vdmpybus_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..64 {
                let mut s = if acc { get_h(&out, i) } else { 0 };
                s += ub(&vu, i * 2 + 0) * rt_sb(rt, (2 * i) % 4);
                s += ub(&vu, i * 2 + 1) * rt_sb(rt, (2 * i + 1) % 4);
                set_h(&mut out, i, s as u16);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vdmpybus_dv: pair, sliding-window 2-wide, ub*b -> h ----
        Opcode::V6_vdmpybus_dv | Opcode::V6_vdmpybus_dv_acc => {
            let acc = op == Opcode::V6_vdmpybus_dv_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut o0 = if acc {
                to_bytes(&ctx.vread(dbase))
            } else {
                [0u8; 128]
            };
            let mut o1 = if acc {
                to_bytes(&ctx.vread(dbase + 1))
            } else {
                [0u8; 128]
            };
            for i in 0..64 {
                let mut s0 = if acc { get_h(&o0, i) } else { 0 };
                s0 += ub(&v0, i * 2 + 0) * rt_sb(rt, (2 * i) % 4);
                s0 += ub(&v0, i * 2 + 1) * rt_sb(rt, (2 * i + 1) % 4);
                set_h(&mut o0, i, s0 as u16);
                let mut s1 = if acc { get_h(&o1, i) } else { 0 };
                s1 += ub(&v0, i * 2 + 1) * rt_sb(rt, (2 * i) % 4);
                s1 += ub(&v1, i * 2 + 0) * rt_sb(rt, (2 * i + 1) % 4);
                set_h(&mut o1, i, s1 as u16);
            }
            ctx.set_v(dbase, from_bytes(&o0));
            ctx.set_v(dbase + 1, from_bytes(&o1));
        }
        // ---- vdmpyhb: single vector, 2-wide, h*b -> w ----
        Opcode::V6_vdmpyhb | Opcode::V6_vdmpyhb_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = op == Opcode::V6_vdmpyhb_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = if acc { get_w(&out, i) } else { 0 };
                s += get_h(&vu, i * 2 + 0) * rt_sb(rt, (2 * i) % 4);
                s += get_h(&vu, i * 2 + 1) * rt_sb(rt, (2 * i + 1) % 4);
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vdmpyhb_dv: pair, sliding-window 2-wide, h*b -> w ----
        Opcode::V6_vdmpyhb_dv | Opcode::V6_vdmpyhb_dv_acc => {
            let acc = op == Opcode::V6_vdmpyhb_dv_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut o0 = if acc {
                to_bytes(&ctx.vread(dbase))
            } else {
                [0u8; 128]
            };
            let mut o1 = if acc {
                to_bytes(&ctx.vread(dbase + 1))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s0 = if acc { get_w(&o0, i) } else { 0 };
                s0 += get_h(&v0, i * 2 + 0) * rt_sb(rt, (2 * i) % 4);
                s0 += get_h(&v0, i * 2 + 1) * rt_sb(rt, (2 * i + 1) % 4);
                set_w(&mut o0, i, s0 as u32);
                let mut s1 = if acc { get_w(&o1, i) } else { 0 };
                s1 += get_h(&v0, i * 2 + 1) * rt_sb(rt, (2 * i) % 4);
                s1 += get_h(&v1, i * 2 + 0) * rt_sb(rt, (2 * i + 1) % 4);
                set_w(&mut o1, i, s1 as u32);
            }
            ctx.set_v(dbase, from_bytes(&o0));
            ctx.set_v(dbase + 1, from_bytes(&o1));
        }
        // ---- vdmpyhsat: single vector, h*h:sat -> w ----
        Opcode::V6_vdmpyhsat | Opcode::V6_vdmpyhsat_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = op == Opcode::V6_vdmpyhsat_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let prev = if acc { get_w(&out, i) } else { 0 };
                let mut s = prev;
                s += get_h(&vu, i * 2 + 0) * rt_h(rt, 0);
                s += get_h(&vu, i * 2 + 1) * rt_h(rt, 1);
                let sat = ctx.sat_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vdmpyhsusat: single vector, h*uh:sat -> w ----
        Opcode::V6_vdmpyhsusat | Opcode::V6_vdmpyhsusat_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = op == Opcode::V6_vdmpyhsusat_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let prev = if acc { get_w(&out, i) } else { 0 };
                let mut s = prev;
                s += get_h(&vu, i * 2 + 0) * rt_uh(rt, 0);
                s += get_h(&vu, i * 2 + 1) * rt_uh(rt, 1);
                let sat = ctx.sat_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vdmpyhisat: pair source -> single, h*h:sat, sliding window ----
        Opcode::V6_vdmpyhisat | Opcode::V6_vdmpyhisat_acc => {
            let acc = op == Opcode::V6_vdmpyhisat_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let prev = if acc { get_w(&out, i) } else { 0 };
                let mut s = prev;
                // Vuu.v[0].w[i].h[1] and Vuu.v[1].w[i].h[0]
                s += get_h(&v0, i * 2 + 1) * rt_h(rt, 0);
                s += get_h(&v1, i * 2 + 0) * rt_h(rt, 1);
                let sat = ctx.sat_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vdmpyhsuisat: pair source -> single, h*uh:sat, sliding window ----
        Opcode::V6_vdmpyhsuisat | Opcode::V6_vdmpyhsuisat_acc => {
            let acc = op == Opcode::V6_vdmpyhsuisat_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let prev = if acc { get_w(&out, i) } else { 0 };
                let mut s = prev;
                s += get_h(&v0, i * 2 + 1) * rt_uh(rt, 0);
                s += get_h(&v1, i * 2 + 0) * rt_uh(rt, 1);
                let sat = ctx.sat_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vdmpyhvsat: vector-vector, h*h:sat -> w ----
        Opcode::V6_vdmpyhvsat | Opcode::V6_vdmpyhvsat_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = op == Opcode::V6_vdmpyhvsat_acc;
            let dst = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut out = if acc {
                to_bytes(&ctx.vread(dst))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                let mut s = get_h(&vu, i * 2 + 0) * get_h(&vv, i * 2 + 0);
                s += get_h(&vu, i * 2 + 1) * get_h(&vv, i * 2 + 1);
                if acc {
                    s += get_w(&out, i);
                }
                let sat = ctx.sat_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(dst, from_bytes(&out));
        }
        // ---- vtmpyb: pair, 3-wide sliding-window byte multiply -> h ----
        Opcode::V6_vtmpyb | Opcode::V6_vtmpyb_acc => {
            tmpy_b(op, d, ctx, Opcode::V6_vtmpyb_acc, false);
        }
        Opcode::V6_vtmpybus | Opcode::V6_vtmpybus_acc => {
            tmpy_b(op, d, ctx, Opcode::V6_vtmpybus_acc, true);
        }
        // ---- vtmpyhb: pair, 3-wide sliding-window halfword*byte -> w ----
        Opcode::V6_vtmpyhb | Opcode::V6_vtmpyhb_acc => {
            let acc = op == Opcode::V6_vtmpyhb_acc;
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase));
            let v1 = to_bytes(&ctx.vread(ubase + 1));
            let rt = ctx.r(fld(d, b't'));
            let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let mut o0 = if acc {
                to_bytes(&ctx.vread(dbase))
            } else {
                [0u8; 128]
            };
            let mut o1 = if acc {
                to_bytes(&ctx.vread(dbase + 1))
            } else {
                [0u8; 128]
            };
            for i in 0..32 {
                // each word holds two halfwords h[0],h[1]
                let v0h0 = get_h(&v0, i * 2 + 0);
                let v0h1 = get_h(&v0, i * 2 + 1);
                let v1h0 = get_h(&v1, i * 2 + 0);
                let v1h1 = get_h(&v1, i * 2 + 1);
                let rb0 = rt_sb(rt, (2 * i) % 4);
                let rb1 = rt_sb(rt, (2 * i + 1) % 4);
                let mut s0 = if acc { get_w(&o0, i) } else { 0 };
                s0 += v0h0 * rb0;
                s0 += v0h1 * rb1;
                s0 += v1h0;
                set_w(&mut o0, i, s0 as u32);
                let mut s1 = if acc { get_w(&o1, i) } else { 0 };
                s1 += v0h1 * rb0;
                s1 += v1h0 * rb1;
                s1 += v1h1;
                set_w(&mut o1, i, s1 as u32);
            }
            ctx.set_v(dbase, from_bytes(&o0));
            ctx.set_v(dbase + 1, from_bytes(&o1));
        }
        _ => return false,
    }
    true
}

/// vrmpy pair-source/pair-dest with `#u1` byte-lane rotate (ub*ub->uw or ub*b->w).
/// `unsigned_rt` selects whether Rt bytes are unsigned (ubi) or signed (busi).
fn rmpy_pair_imm(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx, acc_op: Opcode, unsigned_rt: bool) {
    let acc = op == acc_op;
    let ubase = fld(d, b'u');
    let v0 = to_bytes(&ctx.vread(ubase));
    let v1 = to_bytes(&ctx.vread(ubase + 1));
    let rt = ctx.r(fld(d, b't'));
    let imm = fimm_u(d, b'i', ctx.immext) as usize & 1;
    let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
    let mut o0 = if acc {
        to_bytes(&ctx.vread(dbase))
    } else {
        [0u8; 128]
    };
    let mut o1 = if acc {
        to_bytes(&ctx.vread(dbase + 1))
    } else {
        [0u8; 128]
    };
    let rb = |n: usize| {
        if unsigned_rt {
            rt_ub(rt, n)
        } else {
            rt_sb(rt, n)
        }
    };
    // Vuu.v[#u ? 1:0]
    let sel = if imm != 0 { &v1 } else { &v0 };
    for i in 0..32 {
        let base = i * 4;
        let mut s0 = if acc { get_w(&o0, i) } else { 0 };
        s0 += ub(sel, base + 0) * rb((0usize.wrapping_sub(imm)) & 3);
        s0 += ub(&v0, base + 1) * rb((1usize.wrapping_sub(imm)) & 3);
        s0 += ub(&v0, base + 2) * rb((2usize.wrapping_sub(imm)) & 3);
        s0 += ub(&v0, base + 3) * rb((3usize.wrapping_sub(imm)) & 3);
        set_w(&mut o0, i, s0 as u32);
        let mut s1 = if acc { get_w(&o1, i) } else { 0 };
        s1 += ub(&v1, base + 0) * rb((2usize.wrapping_sub(imm)) & 3);
        s1 += ub(&v1, base + 1) * rb((3usize.wrapping_sub(imm)) & 3);
        s1 += ub(sel, base + 2) * rb((0usize.wrapping_sub(imm)) & 3);
        s1 += ub(&v0, base + 3) * rb((1usize.wrapping_sub(imm)) & 3);
        set_w(&mut o1, i, s1 as u32);
    }
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}

/// vtmpyb / vtmpybus: pair, 3-wide sliding-window byte multiply -> halfword pair.
/// `unsigned_vu` selects whether Vuu bytes are unsigned (vtmpybus) or signed (vtmpyb).
fn tmpy_b(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx, acc_op: Opcode, unsigned_vu: bool) {
    let acc = op == acc_op;
    let ubase = fld(d, b'u');
    let v0 = to_bytes(&ctx.vread(ubase));
    let v1 = to_bytes(&ctx.vread(ubase + 1));
    let rt = ctx.r(fld(d, b't'));
    let dbase = if acc { fld(d, b'x') } else { fld(d, b'd') };
    let mut o0 = if acc {
        to_bytes(&ctx.vread(dbase))
    } else {
        [0u8; 128]
    };
    let mut o1 = if acc {
        to_bytes(&ctx.vread(dbase + 1))
    } else {
        [0u8; 128]
    };
    let vb = |b: &Bytes, i: usize| if unsigned_vu { ub(b, i) } else { sb(b, i) };
    for i in 0..64 {
        // each halfword holds two bytes b[0],b[1]
        let v0b0 = vb(&v0, i * 2 + 0);
        let v0b1 = vb(&v0, i * 2 + 1);
        let v1b0 = vb(&v1, i * 2 + 0);
        let v1b1 = vb(&v1, i * 2 + 1);
        let rb0 = rt_sb(rt, (2 * i) % 4);
        let rb1 = rt_sb(rt, (2 * i + 1) % 4);
        let mut s0 = if acc { get_h(&o0, i) } else { 0 };
        s0 += v0b0 * rb0;
        s0 += v0b1 * rb1;
        s0 += v1b0;
        set_h(&mut o0, i, s0 as u16);
        let mut s1 = if acc { get_h(&o1, i) } else { 0 };
        s1 += v0b1 * rb0;
        s1 += v1b0 * rb1;
        s1 += v1b1;
        set_h(&mut o1, i, s1 as u16);
    }
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}
