//! (hvx_addsub) HVX widening, dual-vector and saturating-special integer add/sub.
//!
//! Three families live here:
//!  * Dual-vector (`Vdd=op(Vuu,Vvv)`): the byte/half/word/(u)sat variants of vadd
//!    and vsub applied independently to each vector of a register pair.
//!  * Widening (`Vdd.h=vadd(Vu.ub,Vv.ub)`, `Vdd.w=vadd(Vu.h,Vv.h)`, ...): a single
//!    vector add/sub whose narrow lanes widen into a pair of wider lanes; even
//!    sub-lanes land in `Vdd.v[0]`, odd sub-lanes in `Vdd.v[1]`. Has `+=` (acc)
//!    forms whose dest pair `Vxx` is read-modify-write.
//!  * Special: `Vd.ub=v(add|sub)(Vu.ub,Vv.b):sat` (mixed signed/unsigned byte,
//!    unsigned-saturated) and `Vd.(h|w)=vadd(vclb(Vu),Vv)` (count-leading-sign-bits
//!    plus a vector addend).
//!
//! Matched element-for-element to the V68 HVX PRM pseudocode and verified against
//! the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).

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

// ---- per-lane scalar ops (no USR side effects: wrapping or simple min/checked) ----

fn add_b(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}
fn sub_b(a: u8, b: u8) -> u8 {
    a.wrapping_sub(b)
}
fn add_h(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
fn sub_h(a: u16, b: u16) -> u16 {
    a.wrapping_sub(b)
}
fn add_w(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}
fn sub_w(a: u32, b: u32) -> u32 {
    a.wrapping_sub(b)
}
fn add_bsat(a: u8, b: u8) -> u8 {
    (a as i8 as i16 + b as i8 as i16).clamp(-128, 127) as u8
}
fn sub_bsat(a: u8, b: u8) -> u8 {
    (a as i8 as i16 - b as i8 as i16).clamp(-128, 127) as u8
}
fn add_hsat(a: u16, b: u16) -> u16 {
    (a as i16 as i32 + b as i16 as i32).clamp(-32768, 32767) as u16
}
fn sub_hsat(a: u16, b: u16) -> u16 {
    (a as i16 as i32 - b as i16 as i32).clamp(-32768, 32767) as u16
}
fn add_wsat(a: u32, b: u32) -> u32 {
    (a as i32 as i64 + b as i32 as i64).clamp(i32::MIN as i64, i32::MAX as i64) as u32
}
fn sub_wsat(a: u32, b: u32) -> u32 {
    (a as i32 as i64 - b as i32 as i64).clamp(i32::MIN as i64, i32::MAX as i64) as u32
}
fn add_ubsat(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}
fn sub_ubsat(a: u8, b: u8) -> u8 {
    a.saturating_sub(b)
}
fn add_uhsat(a: u16, b: u16) -> u16 {
    a.saturating_add(b)
}
fn sub_uhsat(a: u16, b: u16) -> u16 {
    a.saturating_sub(b)
}
fn add_uwsat(a: u32, b: u32) -> u32 {
    a.saturating_add(b)
}
fn sub_uwsat(a: u32, b: u32) -> u32 {
    a.saturating_sub(b)
}

/// Apply a per-byte op to both vectors of a source pair, producing a dest pair.
fn dv_b(
    u0: &Bytes,
    u1: &Bytes,
    v0: &Bytes,
    v1: &Bytes,
    f: impl Fn(u8, u8) -> u8,
) -> (Bytes, Bytes) {
    let mut o0 = [0u8; 128];
    let mut o1 = [0u8; 128];
    for i in 0..128 {
        o0[i] = f(u0[i], v0[i]);
        o1[i] = f(u1[i], v1[i]);
    }
    (o0, o1)
}
/// Apply a per-halfword op to both vectors of a source pair.
fn dv_h(
    u0: &Bytes,
    u1: &Bytes,
    v0: &Bytes,
    v1: &Bytes,
    f: impl Fn(u16, u16) -> u16,
) -> (Bytes, Bytes) {
    let mut o0 = [0u8; 128];
    let mut o1 = [0u8; 128];
    for i in 0..64 {
        set_h(&mut o0, i, f(get_h(u0, i), get_h(v0, i)));
        set_h(&mut o1, i, f(get_h(u1, i), get_h(v1, i)));
    }
    (o0, o1)
}
/// Apply a per-word op to both vectors of a source pair.
fn dv_w(
    u0: &Bytes,
    u1: &Bytes,
    v0: &Bytes,
    v1: &Bytes,
    f: impl Fn(u32, u32) -> u32,
) -> (Bytes, Bytes) {
    let mut o0 = [0u8; 128];
    let mut o1 = [0u8; 128];
    for i in 0..32 {
        set_w(&mut o0, i, f(get_w(u0, i), get_w(v0, i)));
        set_w(&mut o1, i, f(get_w(u1, i), get_w(v1, i)));
    }
    (o0, o1)
}

/// Count of leading identical bits in an `nbits`-wide value (capped at nbits):
/// `max(count_leading_ones(~x), count_leading_ones(x))`. count_leading_ones(~x)
/// is the count of leading zeros of x; count_leading_ones(x) is the leading-1
/// run. Both are measured within the `nbits`-wide element only.
fn clb(x: u64, nbits: u32) -> u32 {
    let shift = 64 - nbits;
    let xj = x << shift; // left-justify the element into the top `nbits`
    // Leading-zero/one runs of the full 64-bit value, but the run can never
    // exceed the element width (the low `shift` bits are zero padding, which
    // would otherwise inflate the leading-zero count for an all-zero element).
    let zeros = xj.leading_zeros().min(nbits);
    let ones = xj.leading_ones().min(nbits);
    zeros.max(ones)
}

/// Execute a hvx_addsub opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ================= dual-vector (Vdd = op(Vuu, Vvv)) =================
        Opcode::V6_vaddb_dv => dv_b_dispatch(d, ctx, add_b),
        Opcode::V6_vaddh_dv => dv_h_dispatch(d, ctx, add_h),
        Opcode::V6_vaddw_dv => dv_w_dispatch(d, ctx, add_w),
        Opcode::V6_vaddbsat_dv => dv_b_dispatch(d, ctx, add_bsat),
        Opcode::V6_vaddhsat_dv => dv_h_dispatch(d, ctx, add_hsat),
        Opcode::V6_vaddwsat_dv => dv_w_dispatch(d, ctx, add_wsat),
        Opcode::V6_vaddubsat_dv => dv_b_dispatch(d, ctx, add_ubsat),
        Opcode::V6_vadduhsat_dv => dv_h_dispatch(d, ctx, add_uhsat),
        Opcode::V6_vadduwsat_dv => dv_w_dispatch(d, ctx, add_uwsat),
        Opcode::V6_vsubb_dv => dv_b_dispatch(d, ctx, sub_b),
        Opcode::V6_vsubh_dv => dv_h_dispatch(d, ctx, sub_h),
        Opcode::V6_vsubw_dv => dv_w_dispatch(d, ctx, sub_w),
        Opcode::V6_vsubbsat_dv => dv_b_dispatch(d, ctx, sub_bsat),
        Opcode::V6_vsubhsat_dv => dv_h_dispatch(d, ctx, sub_hsat),
        Opcode::V6_vsubwsat_dv => dv_w_dispatch(d, ctx, sub_wsat),
        Opcode::V6_vsububsat_dv => dv_b_dispatch(d, ctx, sub_ubsat),
        Opcode::V6_vsubuhsat_dv => dv_h_dispatch(d, ctx, sub_uhsat),
        Opcode::V6_vsubuwsat_dv => dv_w_dispatch(d, ctx, sub_uwsat),

        // ================= widening ub*ub -> h pair =================
        // Vdd.v[0].h[i] = Vu.ub[2i] (+/-) Vv.ub[2i]; Vdd.v[1].h[i] uses byte 2i+1.
        Opcode::V6_vaddubh => widen_ubh(d, ctx, |a, b| a + b, false),
        Opcode::V6_vaddubh_acc => widen_ubh(d, ctx, |a, b| a + b, true),
        Opcode::V6_vsububh => widen_ubh(d, ctx, |a, b| a - b, false),

        // ================= widening h*h -> w pair (signed) =================
        Opcode::V6_vaddhw => widen_hw(d, ctx, |a, b| a + b, false, false),
        Opcode::V6_vaddhw_acc => widen_hw(d, ctx, |a, b| a + b, true, false),
        Opcode::V6_vsubhw => widen_hw(d, ctx, |a, b| a - b, false, false),

        // ================= widening uh*uh -> w pair (unsigned) =================
        Opcode::V6_vadduhw => widen_hw(d, ctx, |a, b| a + b, false, true),
        Opcode::V6_vadduhw_acc => widen_hw(d, ctx, |a, b| a + b, true, true),
        Opcode::V6_vsubuhw => widen_hw(d, ctx, |a, b| a - b, false, true),

        // ================= special: ub (+/-) b, unsigned-saturated byte =================
        Opcode::V6_vaddububb_sat => ububb_sat(d, ctx, true),
        Opcode::V6_vsubububb_sat => ububb_sat(d, ctx, false),

        // ================= special: count-leading-sign-bits + addend =================
        Opcode::V6_vaddclbh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                let c = clb(get_h(&vu, i) as u64, 16) as u16;
                set_h(&mut out, i, c.wrapping_add(get_h(&vv, i)));
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            return true;
        }
        Opcode::V6_vaddclbw => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let c = clb(get_w(&vu, i) as u64, 32);
                set_w(&mut out, i, c.wrapping_add(get_w(&vv, i)));
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            return true;
        }

        _ => return false,
    }
    true
}

/// Read the source pair bases (`u`/`v`) and dest pair base (`d`) for a
/// dual-vector op, returning `(u0, u1, v0, v1, dbase)`.
fn dv_inputs(d: &DecodedOp, ctx: &mut SemCtx) -> (Bytes, Bytes, Bytes, Bytes, u8) {
    let ubase = fld(d, b'u');
    let vbase = fld(d, b'v');
    let dbase = fld(d, b'd');
    let u0 = to_bytes(&ctx.vread(ubase));
    let u1 = to_bytes(&ctx.vread(ubase + 1));
    let v0 = to_bytes(&ctx.vread(vbase));
    let v1 = to_bytes(&ctx.vread(vbase + 1));
    (u0, u1, v0, v1, dbase)
}

fn dv_b_dispatch(d: &DecodedOp, ctx: &mut SemCtx, f: impl Fn(u8, u8) -> u8) {
    let (u0, u1, v0, v1, dbase) = dv_inputs(d, ctx);
    let (o0, o1) = dv_b(&u0, &u1, &v0, &v1, f);
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}
fn dv_h_dispatch(d: &DecodedOp, ctx: &mut SemCtx, f: impl Fn(u16, u16) -> u16) {
    let (u0, u1, v0, v1, dbase) = dv_inputs(d, ctx);
    let (o0, o1) = dv_h(&u0, &u1, &v0, &v1, f);
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}
fn dv_w_dispatch(d: &DecodedOp, ctx: &mut SemCtx, f: impl Fn(u32, u32) -> u32) {
    let (u0, u1, v0, v1, dbase) = dv_inputs(d, ctx);
    let (o0, o1) = dv_w(&u0, &u1, &v0, &v1, f);
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}

/// Widening byte->halfword: `f` combines the two i32-extended bytes; even byte
/// lane -> Vdd.v[0], odd byte lane -> Vdd.v[1]. Optional accumulate into Vxx.
fn widen_ubh(d: &DecodedOp, ctx: &mut SemCtx, f: impl Fn(i32, i32) -> i32, acc: bool) {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
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
        let lo = f(vu[i * 2] as i32, vv[i * 2] as i32);
        let hi = f(vu[i * 2 + 1] as i32, vv[i * 2 + 1] as i32);
        let p0 = if acc { get_h(&o0, i) as i16 as i32 } else { 0 };
        let p1 = if acc { get_h(&o1, i) as i16 as i32 } else { 0 };
        set_h(&mut o0, i, (p0 + lo) as u16);
        set_h(&mut o1, i, (p1 + hi) as u16);
    }
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}

/// Widening halfword->word: `f` combines two i64-extended halfwords; even
/// halfword lane -> Vdd.v[0], odd -> Vdd.v[1]. `unsigned` zero-extends the
/// halfword sources (uh) instead of sign-extending (h). Optional accumulate.
fn widen_hw(
    d: &DecodedOp,
    ctx: &mut SemCtx,
    f: impl Fn(i64, i64) -> i64,
    acc: bool,
    unsigned: bool,
) {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
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
    let ext = |h: u16| -> i64 { if unsigned { h as i64 } else { h as i16 as i64 } };
    for i in 0..32 {
        let lo = f(ext(get_h(&vu, i * 2)), ext(get_h(&vv, i * 2)));
        let hi = f(ext(get_h(&vu, i * 2 + 1)), ext(get_h(&vv, i * 2 + 1)));
        let p0 = if acc { get_w(&o0, i) as i32 as i64 } else { 0 };
        let p1 = if acc { get_w(&o1, i) as i32 as i64 } else { 0 };
        set_w(&mut o0, i, (p0 + lo) as u32);
        set_w(&mut o1, i, (p1 + hi) as u32);
    }
    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
}

/// `Vd.ub = v(add|sub)(Vu.ub, Vv.b):sat`: per byte, unsigned Vu, signed Vv,
/// result unsigned-saturated to 8 bits.
fn ububb_sat(d: &DecodedOp, ctx: &mut SemCtx, add: bool) {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
    let mut out = [0u8; 128];
    for i in 0..128 {
        let a = vu[i] as i32; // unsigned byte
        let b = vv[i] as i8 as i32; // signed byte
        let r = if add { a + b } else { a - b };
        out[i] = ctx.satu_n(r as i64, 8) as u8;
    }
    ctx.set_v(fld(d, b'd'), from_bytes(&out));
}
