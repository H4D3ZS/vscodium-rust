//! (hvx_mpyv) HVX widening multiplies: vector-by-vector and vector-by-scalar
//! forms (vmpyb/h/ub/uh, plus the `:<<1:[rnd:]sat` halfword forms and the
//! `vmpyuhe` even-halfword form). Most forms widen each pair of sub-elements
//! into a destination register pair (Vdd/Vxx), placing the even sub-products in
//! the even (low) vector register and the odd sub-products in the odd (high)
//! register. Verified against the qemu-hexagon vector oracle
//! (tests/hexagon_hvx_diff.rs).

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
/// Signed byte at byte index `i` (0..128).
#[inline]
fn get_b(b: &Bytes, i: usize) -> i32 {
    b[i] as i8 as i32
}
/// Unsigned byte at byte index `i` (0..128).
#[inline]
fn get_ub(b: &Bytes, i: usize) -> i32 {
    b[i] as i32
}
/// Signed halfword at halfword index `i` (0..64).
#[inline]
fn get_h(b: &Bytes, i: usize) -> i32 {
    i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i32
}
/// Unsigned halfword at halfword index `i` (0..64).
#[inline]
fn get_uh(b: &Bytes, i: usize) -> i32 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i32
}
/// Signed word at word index `i` (0..32).
#[inline]
fn get_w(b: &Bytes, i: usize) -> i32 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
/// Unsigned word at word index `i` (0..32).
#[inline]
fn get_uw(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
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
/// Unsigned byte `n` (0..3) of a scalar word.
#[inline]
fn rt_ubyte(rt: u32, n: usize) -> i32 {
    ((rt >> (n * 8)) & 0xff) as i32
}
/// Signed halfword `n` (0..1) of a scalar word.
#[inline]
fn rt_half(rt: u32, n: usize) -> i32 {
    ((rt >> (n * 16)) & 0xffff) as u16 as i16 as i32
}
/// Unsigned halfword `n` (0..1) of a scalar word.
#[inline]
fn rt_uhalf(rt: u32, n: usize) -> i32 {
    ((rt >> (n * 16)) & 0xffff) as i32
}

/// Read a vector-register pair (even base `reg`, odd `reg+1`) as 256 bytes split
/// into (low, high) 128-byte halves.
#[inline]
fn read_pair(ctx: &SemCtx, reg: u8) -> (Bytes, Bytes) {
    (to_bytes(&ctx.vread(reg)), to_bytes(&ctx.vread(reg + 1)))
}
/// Write a vector-register pair: `lo` -> Vd[reg] (even), `hi` -> Vd[reg+1] (odd).
#[inline]
fn write_pair(ctx: &mut SemCtx, reg: u8, lo: &Bytes, hi: &Bytes) {
    ctx.set_v(reg, from_bytes(lo));
    ctx.set_v(reg + 1, from_bytes(hi));
}

/// Execute a hvx_mpyv opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vector-by-vector, widening byte multiplies -> halfword pair ----
        // Vdd.h=vmpy(Vu.b,Vv.b): v[0].h[i]=Vu.b[2i]*Vv.b[2i]; v[1]=odd bytes.
        Opcode::V6_vmpybv | Opcode::V6_vmpybv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = matches!(op, Opcode::V6_vmpybv_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let p0 = get_b(&vu, 2 * i) * get_b(&vv, 2 * i);
                let p1 = get_b(&vu, 2 * i + 1) * get_b(&vv, 2 * i + 1);
                let a0 = if acc { get_h(&lo, i) } else { 0 };
                let a1 = if acc { get_h(&hi, i) } else { 0 };
                set_h(&mut lo, i, (a0 + p0) as u16);
                set_h(&mut hi, i, (a1 + p1) as u16);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.h=vmpy(Vu.ub,Vv.b): unsigned byte by signed byte -> halfword pair.
        Opcode::V6_vmpybusv | Opcode::V6_vmpybusv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = matches!(op, Opcode::V6_vmpybusv_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let p0 = get_ub(&vu, 2 * i) * get_b(&vv, 2 * i);
                let p1 = get_ub(&vu, 2 * i + 1) * get_b(&vv, 2 * i + 1);
                let a0 = if acc { get_h(&lo, i) } else { 0 };
                let a1 = if acc { get_h(&hi, i) } else { 0 };
                set_h(&mut lo, i, (a0 + p0) as u16);
                set_h(&mut hi, i, (a1 + p1) as u16);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.uh=vmpy(Vu.ub,Vv.ub): unsigned byte by unsigned byte -> uh pair.
        Opcode::V6_vmpyubv | Opcode::V6_vmpyubv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = matches!(op, Opcode::V6_vmpyubv_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let p0 = get_ub(&vu, 2 * i) * get_ub(&vv, 2 * i);
                let p1 = get_ub(&vu, 2 * i + 1) * get_ub(&vv, 2 * i + 1);
                let a0 = if acc { get_uh(&lo, i) } else { 0 };
                let a1 = if acc { get_uh(&hi, i) } else { 0 };
                set_h(&mut lo, i, (a0 + p0) as u16);
                set_h(&mut hi, i, (a1 + p1) as u16);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.w=vmpy(Vu.h,Vv.h): signed halfword by signed halfword -> word pair.
        Opcode::V6_vmpyhv | Opcode::V6_vmpyhv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = matches!(op, Opcode::V6_vmpyhv_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                let p0 = get_h(&vu, 2 * i) as i64 * get_h(&vv, 2 * i) as i64;
                let p1 = get_h(&vu, 2 * i + 1) as i64 * get_h(&vv, 2 * i + 1) as i64;
                let a0 = if acc { get_w(&lo, i) as i64 } else { 0 };
                let a1 = if acc { get_w(&hi, i) as i64 } else { 0 };
                set_w(&mut lo, i, (a0 + p0) as u32);
                set_w(&mut hi, i, (a1 + p1) as u32);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.w=vmpy(Vu.h,Vv.uh): signed halfword by unsigned halfword -> word pair.
        Opcode::V6_vmpyhus | Opcode::V6_vmpyhus_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = matches!(op, Opcode::V6_vmpyhus_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                let p0 = get_h(&vu, 2 * i) as i64 * get_uh(&vv, 2 * i) as i64;
                let p1 = get_h(&vu, 2 * i + 1) as i64 * get_uh(&vv, 2 * i + 1) as i64;
                let a0 = if acc { get_w(&lo, i) as i64 } else { 0 };
                let a1 = if acc { get_w(&hi, i) as i64 } else { 0 };
                set_w(&mut lo, i, (a0 + p0) as u32);
                set_w(&mut hi, i, (a1 + p1) as u32);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.uw=vmpy(Vu.uh,Vv.uh): unsigned halfword by unsigned halfword -> uw pair.
        Opcode::V6_vmpyuhv | Opcode::V6_vmpyuhv_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let acc = matches!(op, Opcode::V6_vmpyuhv_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                let p0 = get_uh(&vu, 2 * i) as i64 * get_uh(&vv, 2 * i) as i64;
                let p1 = get_uh(&vu, 2 * i + 1) as i64 * get_uh(&vv, 2 * i + 1) as i64;
                let a0 = if acc { get_uw(&lo, i) as i64 } else { 0 };
                let a1 = if acc { get_uw(&hi, i) as i64 } else { 0 };
                set_w(&mut lo, i, (a0 + p0) as u32);
                set_w(&mut hi, i, (a1 + p1) as u32);
            }
            write_pair(ctx, rd, &lo, &hi);
        }

        // ---- vector-by-vector, halfword <<1:rnd:sat -> single halfword vector --
        // Vd.h=vmpy(Vu.h,Vv.h):<<1:rnd:sat
        Opcode::V6_vmpyhvsrs => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                let prod = (get_h(&vu, i) as i64 * get_h(&vv, i) as i64) << 1;
                let rnd = prod + 0x8000;
                let s32 = ctx.sat_n(rnd, 32);
                let h1 = (s32 >> 16) & 0xffff;
                set_h(&mut out, i, h1 as u16);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        // ---- vector-by-scalar, widening byte multiplies -> halfword pair ------
        // Vdd.h=vmpy(Vu.ub,Rt.b): unsigned byte by signed scalar byte.
        // even lane i uses Rt.b[(2i+0)%4], odd uses Rt.b[(2i+1)%4].
        Opcode::V6_vmpybus | Opcode::V6_vmpybus_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = matches!(op, Opcode::V6_vmpybus_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let p0 = get_ub(&vu, 2 * i) * rt_byte(rt, (2 * i) % 4);
                let p1 = get_ub(&vu, 2 * i + 1) * rt_byte(rt, (2 * i + 1) % 4);
                let a0 = if acc { get_h(&lo, i) } else { 0 };
                let a1 = if acc { get_h(&hi, i) } else { 0 };
                set_h(&mut lo, i, (a0 + p0) as u16);
                set_h(&mut hi, i, (a1 + p1) as u16);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.uh=vmpy(Vu.ub,Rt.ub): unsigned byte by unsigned scalar byte.
        Opcode::V6_vmpyub | Opcode::V6_vmpyub_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = matches!(op, Opcode::V6_vmpyub_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..64 {
                let p0 = get_ub(&vu, 2 * i) * rt_ubyte(rt, (2 * i) % 4);
                let p1 = get_ub(&vu, 2 * i + 1) * rt_ubyte(rt, (2 * i + 1) % 4);
                let a0 = if acc { get_uh(&lo, i) } else { 0 };
                let a1 = if acc { get_uh(&hi, i) } else { 0 };
                set_h(&mut lo, i, (a0 + p0) as u16);
                set_h(&mut hi, i, (a1 + p1) as u16);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.w=vmpy(Vu.h,Rt.h): signed halfword by signed scalar halfword.
        // v[0].w[i]=Vu.w[i].h[0]*Rt.h[0]; v[1].w[i]=Vu.w[i].h[1]*Rt.h[1].
        Opcode::V6_vmpyh | Opcode::V6_vmpyh_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = matches!(op, Opcode::V6_vmpyh_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                let p0 = get_h(&vu, 2 * i) as i64 * rt_half(rt, 0) as i64;
                let p1 = get_h(&vu, 2 * i + 1) as i64 * rt_half(rt, 1) as i64;
                let a0 = if acc { get_w(&lo, i) as i64 } else { 0 };
                let a1 = if acc { get_w(&hi, i) as i64 } else { 0 };
                set_w(&mut lo, i, (a0 + p0) as u32);
                set_w(&mut hi, i, (a1 + p1) as u32);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vxx.w+=vmpy(Vu.h,Rt.h):sat: saturating word accumulate (no non-acc form).
        Opcode::V6_vmpyhsat_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let rd = fld(d, b'x');
            let (mut lo, mut hi) = read_pair(ctx, rd);
            for i in 0..32 {
                let p0 = get_h(&vu, 2 * i) as i64 * rt_half(rt, 0) as i64;
                let p1 = get_h(&vu, 2 * i + 1) as i64 * rt_half(rt, 1) as i64;
                let s0 = ctx.sat_n(get_w(&lo, i) as i64 + p0, 32);
                let s1 = ctx.sat_n(get_w(&hi, i) as i64 + p1, 32);
                set_w(&mut lo, i, s0 as u32);
                set_w(&mut hi, i, s1 as u32);
            }
            write_pair(ctx, rd, &lo, &hi);
        }
        // Vdd.uw=vmpy(Vu.uh,Rt.uh): unsigned halfword by unsigned scalar halfword.
        Opcode::V6_vmpyuh | Opcode::V6_vmpyuh_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let acc = matches!(op, Opcode::V6_vmpyuh_acc);
            let rd = if acc { fld(d, b'x') } else { fld(d, b'd') };
            let (mut lo, mut hi) = if acc {
                read_pair(ctx, rd)
            } else {
                ([0u8; 128], [0u8; 128])
            };
            for i in 0..32 {
                let p0 = get_uh(&vu, 2 * i) as i64 * rt_uhalf(rt, 0) as i64;
                let p1 = get_uh(&vu, 2 * i + 1) as i64 * rt_uhalf(rt, 1) as i64;
                let a0 = if acc { get_uw(&lo, i) as i64 } else { 0 };
                let a1 = if acc { get_uw(&hi, i) as i64 } else { 0 };
                set_w(&mut lo, i, (a0 + p0) as u32);
                set_w(&mut hi, i, (a1 + p1) as u32);
            }
            write_pair(ctx, rd, &lo, &hi);
        }

        // ---- vector-by-scalar, halfword <<1 sat / rnd:sat -> single vector ----
        // Vd.h=vmpy(Vu.h,Rt.h):<<1:sat : per word lane, both halfwords.
        Opcode::V6_vmpyhss => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let p0 = ((get_h(&vu, 2 * i) as i64) * (rt_half(rt, 0) as i64)) << 1;
                let p1 = ((get_h(&vu, 2 * i + 1) as i64) * (rt_half(rt, 1) as i64)) << 1;
                let h0 = (ctx.sat_n(p0, 32) >> 16) & 0xffff;
                let h1 = (ctx.sat_n(p1, 32) >> 16) & 0xffff;
                set_h(&mut out, 2 * i, h0 as u16);
                set_h(&mut out, 2 * i + 1, h1 as u16);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }
        // Vd.h=vmpy(Vu.h,Rt.h):<<1:rnd:sat : as above with +0x8000 round.
        Opcode::V6_vmpyhsrs => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let p0 = (((get_h(&vu, 2 * i) as i64) * (rt_half(rt, 0) as i64)) << 1) + 0x8000;
                let p1 = (((get_h(&vu, 2 * i + 1) as i64) * (rt_half(rt, 1) as i64)) << 1) + 0x8000;
                let h0 = (ctx.sat_n(p0, 32) >> 16) & 0xffff;
                let h1 = (ctx.sat_n(p1, 32) >> 16) & 0xffff;
                set_h(&mut out, 2 * i, h0 as u16);
                set_h(&mut out, 2 * i + 1, h1 as u16);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        // ---- vmpye: even unsigned-halfword by scalar unsigned-halfword -------
        // Vd.uw[i] = Vu.uw[i].uh[0] * Rt.uh[0]  (low halfword of each word lane).
        Opcode::V6_vmpyuhe => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let p = get_uh(&vu, 2 * i) as u64 * (rt_uhalf(rt, 0) as u64);
                set_w(&mut out, i, p as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }
        // Vx.uw[i] += Vu.uw[i].uh[0] * Rt.uh[0]  (wrapping unsigned word accumulate).
        Opcode::V6_vmpyuhe_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let rd = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(rd));
            for i in 0..32 {
                let p = get_uh(&vu, 2 * i) as u64 * (rt_uhalf(rt, 0) as u64);
                let acc = get_uw(&out, i).wrapping_add(p as u32);
                set_w(&mut out, i, acc);
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // Vd.uh=vmpy(Vu.uh,Vv.uh):>>16 (V69): per-halfword unsigned 16x16 product,
        // keep the high 16 bits — `Vd.uh[i] = (Vu.uh[i]*Vv.uh[i]).uh[1]`.
        Opcode::V6_vmpyuhvs => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                let p = get_uh(&vu, i) as u64 * get_uh(&vv, i) as u64;
                set_h(&mut out, i, ((p >> 16) & 0xffff) as u16);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        _ => return false,
    }
    true
}
