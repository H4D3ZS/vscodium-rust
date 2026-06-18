//! (hvx_v6mpy) HVX V69 byte-matrix multiply v6mpy (vertical/horizontal phases),
//! with packed sign-extended 10-bit coefficients and a 2-bit phase immediate.
//!
//! For each 32-bit lane `i`, six signed ~10-bit coefficients are unpacked from
//! the Vvv vector pair: c0j (j=0..2) from Vvv.v[0] and c1j from Vvv.v[1]. The
//! low 8 bits come from `Vvv.v[k].uw[i].ub[j]` and the high 2 bits from
//! `Vvv.v[k].uw[i].ub[3] >> (2*j)`; the 10-bit value is sign-extended to i32.
//!
//! The `#u2` phase selects which unsigned bytes of the Vuu pair multiply which
//! coefficients, and whether each product lands in Vdd.v[0] or Vdd.v[1]. The
//! `:h` (horizontal) and `:v` (vertical) forms use different byte selections.
//! The `_vxx` forms accumulate into the existing destination pair.
//!
//! Verified against the qemu-hexagon v69 vector oracle (tests/hexagon_hvx_diff.rs).

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
/// Unsigned byte `k` (0..3) of word `i` (0..31) of a 128-byte vector.
#[inline]
fn ub(b: &Bytes, i: usize, k: usize) -> i64 {
    b[i * 4 + k] as i64
}
/// Signed word `i` (0..31).
#[inline]
fn get_w(b: &Bytes, i: usize) -> i64 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]) as i64
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Unpack the signed 10-bit coefficient `cXj` for lane `i` from one Vvv vector.
/// Low 8 bits from `ub[j]`; high 2 bits from `ub[3] >> (2*j)`; sign-extend 10->i64.
#[inline]
fn coeff(b: &Bytes, i: usize, j: usize) -> i64 {
    let hi2 = (b[i * 4 + 3] as i64 >> (2 * j)) & 3;
    let lo8 = b[i * 4 + j] as i64;
    let v10 = (hi2 << 8) | lo8;
    // sign-extend a 10-bit value: ((v << 6) >> 6) in i64 arithmetic, mask to 10 bits first.
    ((v10 & 0x3ff) << 54) >> 54
}

/// A single product term: which Vuu vector (0=lo,1=hi), which byte (0..3) of the
/// word lane, which coefficient (0=c00..2=c02, 3=c10..5=c12), and which output
/// vector (0=lo,1=hi) it accumulates into.
type Term = (u8, u8, u8, u8);

/// :h (horizontal) phase term tables, indexed by `#u2`.
const H_TERMS: [&[Term]; 4] = [
    // u == 0
    &[
        (1, 3, 3, 1),
        (1, 1, 4, 1),
        (0, 3, 5, 1),
        (1, 2, 0, 1),
        (1, 0, 1, 1),
        (0, 2, 2, 1),
        (1, 2, 3, 0),
        (1, 0, 4, 0),
        (0, 2, 5, 0),
    ],
    // u == 1
    &[
        (1, 3, 0, 1),
        (1, 1, 1, 1),
        (0, 3, 2, 1),
        (1, 3, 3, 0),
        (1, 1, 4, 0),
        (0, 3, 5, 0),
        (1, 2, 0, 0),
        (1, 0, 1, 0),
        (0, 2, 2, 0),
    ],
    // u == 2
    &[
        (1, 1, 3, 1),
        (0, 3, 4, 1),
        (0, 1, 5, 1),
        (1, 0, 0, 1),
        (0, 2, 1, 1),
        (0, 0, 2, 1),
        (1, 0, 3, 0),
        (0, 2, 4, 0),
        (0, 0, 5, 0),
    ],
    // u == 3
    &[
        (1, 1, 0, 1),
        (0, 3, 1, 1),
        (0, 1, 2, 1),
        (1, 1, 3, 0),
        (0, 3, 4, 0),
        (0, 1, 5, 0),
        (1, 0, 0, 0),
        (0, 2, 1, 0),
        (0, 0, 2, 0),
    ],
];

/// :v (vertical) phase term tables, indexed by `#u2`.
const V_TERMS: [&[Term]; 4] = [
    // u == 0
    &[
        (0, 3, 3, 1),
        (1, 2, 4, 1),
        (1, 3, 5, 1),
        (0, 1, 0, 1),
        (1, 0, 1, 1),
        (1, 1, 2, 1),
        (0, 1, 3, 0),
        (1, 0, 4, 0),
        (1, 1, 5, 0),
    ],
    // u == 1
    &[
        (0, 3, 0, 1),
        (1, 2, 1, 1),
        (1, 3, 2, 1),
        (0, 3, 3, 0),
        (1, 2, 4, 0),
        (1, 3, 5, 0),
        (0, 1, 0, 0),
        (1, 0, 1, 0),
        (1, 1, 2, 0),
    ],
    // u == 2
    &[
        (0, 2, 3, 1),
        (0, 3, 4, 1),
        (1, 2, 5, 1),
        (0, 0, 0, 1),
        (0, 1, 1, 1),
        (1, 0, 2, 1),
        (0, 0, 3, 0),
        (0, 1, 4, 0),
        (1, 0, 5, 0),
    ],
    // u == 3
    &[
        (0, 2, 0, 1),
        (0, 3, 1, 1),
        (1, 2, 2, 1),
        (0, 2, 3, 0),
        (0, 3, 4, 0),
        (1, 2, 5, 0),
        (0, 0, 0, 0),
        (0, 1, 1, 0),
        (1, 0, 2, 0),
    ],
];

/// Execute a hvx_v6mpy opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let (terms_tbl, acc) = match op {
        Opcode::V6_v6mpyhubs10 => (&H_TERMS, false),
        Opcode::V6_v6mpyhubs10_vxx => (&H_TERMS, true),
        Opcode::V6_v6mpyvubs10 => (&V_TERMS, false),
        Opcode::V6_v6mpyvubs10_vxx => (&V_TERMS, true),
        _ => return false,
    };

    let ubase = fld(d, b'u');
    let vbase = fld(d, b'v');
    let u0 = to_bytes(&ctx.vread(ubase)); // Vuu.v[0]
    let u1 = to_bytes(&ctx.vread(ubase + 1)); // Vuu.v[1]
    let cv0 = to_bytes(&ctx.vread(vbase)); // Vvv.v[0] -> c0j
    let cv1 = to_bytes(&ctx.vread(vbase + 1)); // Vvv.v[1] -> c1j
    let phase = (fimm_u(d, b'i', ctx.immext) & 3) as usize;
    let terms = terms_tbl[phase];

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
        // coefficients: index 0..2 = c00..c02 (from Vvv.v[0]), 3..5 = c10..c12 (Vvv.v[1]).
        let c = [
            coeff(&cv0, i, 0),
            coeff(&cv0, i, 1),
            coeff(&cv0, i, 2),
            coeff(&cv1, i, 0),
            coeff(&cv1, i, 1),
            coeff(&cv1, i, 2),
        ];
        let mut s0 = if acc { get_w(&o0, i) } else { 0 };
        let mut s1 = if acc { get_w(&o1, i) } else { 0 };
        for &(vsel, byte, ci, osel) in terms {
            let uv = if vsel == 0 { &u0 } else { &u1 };
            let prod = ub(uv, i, byte as usize) * c[ci as usize];
            if osel == 0 {
                s0 = s0.wrapping_add(prod);
            } else {
                s1 = s1.wrapping_add(prod);
            }
        }
        set_w(&mut o0, i, s0 as u32);
        set_w(&mut o1, i, s1 as u32);
    }

    ctx.set_v(dbase, from_bytes(&o0));
    ctx.set_v(dbase + 1, from_bytes(&o1));
    true
}
