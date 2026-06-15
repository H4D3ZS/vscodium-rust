//! (hvx_shift) HVX per-lane vector shifts and bit ops — verified against the
//! qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs). See sem/hvx.rs for the
//! established 128-byte lane pattern and the SemCtx vector API.
//!
//! Implements: per-lane arithmetic/logical shifts by scalar Rt (vaslh/vaslw,
//! vasrh/vasrw, vlsrh/vlsrw/vlsrb), per-lane bidirectional shifts by vector Vv
//! (vaslhv/vaslwv, vasrhv/vasrwv, vlsrhv/vlsrwv), byte rotate vror, count
//! leading zeros (vcl0h/vcl0w), normalization amount (vnormamth/vnormamtw),
//! popcount (vpopcounth), and the narrowing rounding/saturating right shifts
//! (vasrwh[sat][rndsat], vasrwuh*, vasruwuh*, vasrhub*, vasrhb*, vasruhub*).
//! Semantics from the V68 spec; element/shift macros from imported/macros.def
//! and mmvec/macros.def (fBIDIR_*SHIFT*, fSXTN, fCL1_*, fVROUND, fVSAT*).

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

/// Map a per-byte unary op across all 128 byte lanes.
fn map_b(a: &Bytes, f: impl Fn(u8) -> u8) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..128 {
        o[i] = f(a[i]);
    }
    o
}
/// Map a per-halfword unary op across all 64 halfword lanes.
fn map_h(a: &Bytes, f: impl Fn(u16) -> u16) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..64 {
        set_h(&mut o, i, f(get_h(a, i)));
    }
    o
}
/// Map a per-word unary op across all 32 word lanes.
fn map_w(a: &Bytes, f: impl Fn(u32) -> u32) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..32 {
        set_w(&mut o, i, f(get_w(a, i)));
    }
    o
}
/// Map a per-halfword binary op (lane-paired) across all 64 halfword lanes.
fn map_h2(a: &Bytes, c: &Bytes, f: impl Fn(u16, u16) -> u16) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..64 {
        set_h(&mut o, i, f(get_h(a, i), get_h(c, i)));
    }
    o
}
/// Map a per-word binary op (lane-paired) across all 32 word lanes.
fn map_w2(a: &Bytes, c: &Bytes, f: impl Fn(u32, u32) -> u32) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..32 {
        set_w(&mut o, i, f(get_w(a, i), get_w(c, i)));
    }
    o
}

// --- bidirectional shifts (vector shift amount) ----------------------------
// fSXTN(n,_,shamt) sign-extends the low `n` bits of the lane: positive => left,
// negative => right by |shamt| (fBIDIR_*). Shift-by-(|s|-1)-then-1 avoids the
// UB of a full-width shift when |s| equals the type width.

/// fBIDIR_ASHIFTL on a 16-bit signed lane: +s left, -s arithmetic right.
#[inline]
fn bidir_ashiftl_h(src: u16, shamt: i32) -> u16 {
    let s = src as i16;
    if shamt < 0 {
        let n = (-shamt) as u32;
        (((s >> (n - 1)) >> 1) as u16) // arithmetic right by n
    } else {
        ((s as i32) << shamt) as u16
    }
}
/// fBIDIR_ASHIFTR on a 16-bit signed lane: +s arithmetic right, -s left.
#[inline]
fn bidir_ashiftr_h(src: u16, shamt: i32) -> u16 {
    let s = src as i16;
    if shamt < 0 {
        let n = (-shamt) as u32;
        ((((s as i32) << (n - 1)) << 1) as u16)
    } else {
        ((s >> shamt) as u16)
    }
}
/// fBIDIR_LSHIFTR on a 16-bit unsigned lane: +s logical right, -s left.
#[inline]
fn bidir_lshiftr_h(src: u16, shamt: i32) -> u16 {
    if shamt < 0 {
        let n = (-shamt) as u32;
        (((src as u32) << (n - 1)) << 1) as u16
    } else {
        src >> shamt
    }
}
#[inline]
fn bidir_ashiftl_w(src: u32, shamt: i32) -> u32 {
    let s = src as i32;
    if shamt < 0 {
        let n = (-shamt) as u32;
        (((s >> (n - 1)) >> 1) as u32)
    } else {
        ((s as i64) << shamt) as u32
    }
}
#[inline]
fn bidir_ashiftr_w(src: u32, shamt: i32) -> u32 {
    let s = src as i32;
    if shamt < 0 {
        let n = (-shamt) as u32;
        (((s as i64) << (n - 1)) << 1) as u32
    } else {
        (s >> shamt) as u32
    }
}
#[inline]
fn bidir_lshiftr_w(src: u32, shamt: i32) -> u32 {
    if shamt < 0 {
        let n = (-shamt) as u32;
        (((src as u64) << (n - 1)) << 1) as u32
    } else {
        src >> shamt
    }
}

#[inline]
fn sxtn(val: u32, n: u32) -> i32 {
    let m = val & ((1u32 << n) - 1);
    let shift = 32 - n;
    ((m << shift) as i32) >> shift
}

// --- narrowing rounding/saturating right shifts ----------------------------
// `Vd.w[i]` (or .h[i]) packs two narrowed lanes: even sub-lane from Vv, odd
// from Vu, per NARROWING_SHIFT. The source is sign- or zero-extended per the
// type, an optional round bias (+1<<(shamt-1) when shamt>0) is added, then an
// arithmetic/logical right shift and saturation to the narrow width.

#[inline]
fn vsath(v: i64) -> u16 {
    v.clamp(-32768, 32767) as i16 as u16
}
#[inline]
fn vsatuh(v: i64) -> u16 {
    v.clamp(0, 65535) as u16
}
#[inline]
fn vsatb(v: i64) -> u8 {
    v.clamp(-128, 127) as i8 as u8
}
#[inline]
fn vsatub(v: i64) -> u8 {
    v.clamp(0, 255) as u8
}

/// Signed-source word: arithmetic shift right with optional round bias.
#[inline]
fn shr_round_s_w(src: u32, shamt: u32, round: bool) -> i64 {
    let v = src as i32 as i64;
    let v = if round && shamt > 0 {
        v + (1i64 << (shamt - 1))
    } else {
        v
    };
    v >> shamt
}
/// Unsigned-source word.
#[inline]
fn shr_round_u_w(src: u32, shamt: u32, round: bool) -> i64 {
    let v = src as u64 as i64;
    let v = if round && shamt > 0 {
        v + (1i64 << (shamt - 1))
    } else {
        v
    };
    v >> shamt
}
/// Signed-source half.
#[inline]
fn shr_round_s_h(src: u16, shamt: u32, round: bool) -> i64 {
    let v = src as i16 as i64;
    let v = if round && shamt > 0 {
        v + (1i64 << (shamt - 1))
    } else {
        v
    };
    v >> shamt
}
/// Unsigned-source half.
#[inline]
fn shr_round_u_h(src: u16, shamt: u32, round: bool) -> i64 {
    let v = src as u64 as i64;
    let v = if round && shamt > 0 {
        v + (1i64 << (shamt - 1))
    } else {
        v
    };
    v >> shamt
}

/// Word->half narrow: 32 lanes, each output word = [sat(Vv) | sat(Vu)<<16].
fn narrow_wh(
    vu: &Bytes,
    vv: &Bytes,
    shamt: u32,
    round: bool,
    src_signed: bool,
    sat: impl Fn(i64) -> u16,
) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..32 {
        let (lo, hi) = if src_signed {
            (
                sat(shr_round_s_w(get_w(vv, i), shamt, round)),
                sat(shr_round_s_w(get_w(vu, i), shamt, round)),
            )
        } else {
            (
                sat(shr_round_u_w(get_w(vv, i), shamt, round)),
                sat(shr_round_u_w(get_w(vu, i), shamt, round)),
            )
        };
        set_w(&mut o, i, lo as u32 | ((hi as u32) << 16));
    }
    o
}

/// Half->byte narrow: 64 lanes, each output half = [sat(Vv) | sat(Vu)<<8].
fn narrow_hb(
    vu: &Bytes,
    vv: &Bytes,
    shamt: u32,
    round: bool,
    src_signed: bool,
    sat: impl Fn(i64) -> u8,
) -> Bytes {
    let mut o = [0u8; 128];
    for i in 0..64 {
        let (lo, hi) = if src_signed {
            (
                sat(shr_round_s_h(get_h(vv, i), shamt, round)),
                sat(shr_round_s_h(get_h(vu, i), shamt, round)),
            )
        } else {
            (
                sat(shr_round_u_h(get_h(vv, i), shamt, round)),
                sat(shr_round_u_h(get_h(vu, i), shamt, round)),
            )
        };
        set_h(&mut o, i, lo as u16 | ((hi as u16) << 8));
    }
    o
}

/// Execute a hvx_shift opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
    let rt = ctx.r(fld(d, b't'));
    let rd = fld(d, b'd');

    let out = match op {
        // ---- shift by scalar Rt (amount masked to lane-width bits) ----
        Opcode::V6_vaslh => map_h(&vu, |x| x << (rt & 15)),
        Opcode::V6_vaslw => map_w(&vu, |x| x << (rt & 31)),
        Opcode::V6_vasrh => map_h(&vu, |x| ((x as i16) >> (rt & 15)) as u16),
        Opcode::V6_vasrw => map_w(&vu, |x| ((x as i32) >> (rt & 31)) as u32),
        Opcode::V6_vlsrh => map_h(&vu, |x| x >> (rt & 15)),
        Opcode::V6_vlsrw => map_w(&vu, |x| x >> (rt & 31)),
        Opcode::V6_vlsrb => map_b(&vu, |x| x >> (rt & 7)),

        // ---- bidirectional shift by per-lane vector Vv ----
        Opcode::V6_vaslhv => map_h2(&vu, &vv, |a, b| bidir_ashiftl_h(a, sxtn(b as u32, 5))),
        Opcode::V6_vaslwv => map_w2(&vu, &vv, |a, b| bidir_ashiftl_w(a, sxtn(b, 6))),
        Opcode::V6_vasrhv => map_h2(&vu, &vv, |a, b| bidir_ashiftr_h(a, sxtn(b as u32, 5))),
        Opcode::V6_vasrwv => map_w2(&vu, &vv, |a, b| bidir_ashiftr_w(a, sxtn(b, 6))),
        Opcode::V6_vlsrhv => map_h2(&vu, &vv, |a, b| bidir_lshiftr_h(a, sxtn(b as u32, 5))),
        Opcode::V6_vlsrwv => map_w2(&vu, &vv, |a, b| bidir_lshiftr_w(a, sxtn(b, 6))),

        // ---- rotate whole vector right by Rt bytes ----
        Opcode::V6_vror => {
            let mut o = [0u8; 128];
            let r = rt as usize;
            for k in 0..128 {
                o[k] = vu[(k + r) & 127];
            }
            o
        }

        // ---- count leading zeros per lane ----
        Opcode::V6_vcl0h => map_h(&vu, |x| x.leading_zeros() as u16),
        Opcode::V6_vcl0w => map_w(&vu, |x| x.leading_zeros()),

        // ---- normalization amount: max(clz, clo) - 1 ----
        Opcode::V6_vnormamth => map_h(&vu, |x| {
            let n = x.leading_zeros().max((!x).leading_zeros());
            (n - 1) as u16
        }),
        Opcode::V6_vnormamtw => map_w(&vu, |x| {
            let n = x.leading_zeros().max((!x).leading_zeros());
            n - 1
        }),

        // ---- popcount per halfword ----
        Opcode::V6_vpopcounth => map_h(&vu, |x| x.count_ones() as u16),

        // ---- narrowing word->half shifts (signed source) ----
        Opcode::V6_vasrwh => narrow_wh(&vu, &vv, rt & 0xF, false, true, |v| v as u16),
        Opcode::V6_vasrwhsat => narrow_wh(&vu, &vv, rt & 0xF, false, true, vsath),
        Opcode::V6_vasrwhrndsat => narrow_wh(&vu, &vv, rt & 0xF, true, true, vsath),
        Opcode::V6_vasrwuhsat => narrow_wh(&vu, &vv, rt & 0xF, false, true, vsatuh),
        Opcode::V6_vasrwuhrndsat => narrow_wh(&vu, &vv, rt & 0xF, true, true, vsatuh),
        // unsigned-source word->half
        Opcode::V6_vasruwuhsat => narrow_wh(&vu, &vv, rt & 0xF, false, false, vsatuh),
        Opcode::V6_vasruwuhrndsat => narrow_wh(&vu, &vv, rt & 0xF, true, false, vsatuh),

        // ---- narrowing half->byte shifts (signed source) ----
        Opcode::V6_vasrhubsat => narrow_hb(&vu, &vv, rt & 0x7, false, true, vsatub),
        Opcode::V6_vasrhubrndsat => narrow_hb(&vu, &vv, rt & 0x7, true, true, vsatub),
        Opcode::V6_vasrhbsat => narrow_hb(&vu, &vv, rt & 0x7, false, true, vsatb),
        Opcode::V6_vasrhbrndsat => narrow_hb(&vu, &vv, rt & 0x7, true, true, vsatb),
        // unsigned-source half->byte
        Opcode::V6_vasruhubsat => narrow_hb(&vu, &vv, rt & 0x7, false, false, vsatub),
        Opcode::V6_vasruhubrndsat => narrow_hb(&vu, &vv, rt & 0x7, true, false, vsatub),

        _ => return false,
    };
    ctx.set_v(rd, from_bytes(&out));
    true
}
