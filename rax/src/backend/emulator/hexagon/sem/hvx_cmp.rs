//! (hvx_cmp) HVX vector compares -> Q vector-predicates, Q-predicate logic,
//! `vmux` select, and `vand` Q<->R / Q<->V bridges. Verified against the
//! qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).
//!
//! The oracle does NOT capture Q registers, so every Q-producer is verified by
//! consuming the produced Q (e.g. in a `vmux` or `vand(Q,...)`) into a V
//! register that the oracle does capture, and comparing that V.
//!
//! Semantics are taken verbatim from the V68 spec
//! (`tools/hexagon/qemu/semantics_generated.pyinc`). A Q vector-predicate stores
//! one bit per *vector byte*: 128 bits = `[u32; 4]`, where bit `b` corresponds
//! to vector byte `b` (`fGETQBIT(REG,b) = (REG.w[b>>5] >> (b&31)) & 1`). A
//! per-word compare therefore sets 4 contiguous bits per word lane, per-halfword
//! 2 bits, per-byte 1 bit (`fSETQBITS(Q, W, mask, byte_index, ...)`).
//!
//! Not implemented here: the accumulating compare forms (`V6_veqb_and` etc.) and
//! the `_acc` bridge forms (`vandqrt_acc`, `vandvrt_acc`, `vandnqrt_acc`), which
//! read-modify their Q/V destination. Those require a Q/V value written by an
//! earlier packet, which cannot be expressed in the single-packet differential
//! harness, so they are left unverified rather than shipped wrong.

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

// --- Q vector-predicate bit access (1 bit per vector byte) -----------------

/// Read vector-byte bit `i` (0..128) from a Q predicate.
#[inline]
fn qbit(q: &[u32; 4], i: usize) -> bool {
    (q[i >> 5] >> (i & 31)) & 1 != 0
}

/// Set vector-byte bit `i` (0..128) of a Q predicate to `v`.
#[inline]
fn set_qbit(q: &mut [u32; 4], i: usize, v: bool) {
    let m = 1u32 << (i & 31);
    if v {
        q[i >> 5] |= m;
    } else {
        q[i >> 5] &= !m;
    }
}

/// Build a Q predicate from a per-lane compare predicate `f(lane)`, where each
/// lane is `w` bytes wide: every vector byte covered by a true lane gets bit 1.
fn compare<F: Fn(usize) -> bool>(w: usize, f: F) -> [u32; 4] {
    let mut q = [0u32; 4];
    let lanes = 128 / w;
    for lane in 0..lanes {
        let v = f(lane);
        for byte in 0..w {
            set_qbit(&mut q, lane * w + byte, v);
        }
    }
    q
}

// --- per-lane element readers ----------------------------------------------

#[inline]
fn sb(b: &Bytes, i: usize) -> i8 {
    b[i] as i8
}
#[inline]
fn ub(b: &Bytes, i: usize) -> u8 {
    b[i]
}
#[inline]
fn sh(b: &Bytes, i: usize) -> i16 {
    i16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
#[inline]
fn uh(b: &Bytes, i: usize) -> u16 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
#[inline]
fn sw(b: &Bytes, i: usize) -> i32 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}
#[inline]
fn uw(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}

/// Execute a hvx_cmp opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vector compares: Qd = vcmp(Vu.<t>, Vv.<t>) ----
        // (eq is identical for signed/unsigned; gt distinguishes signedness).
        Opcode::V6_veqb
        | Opcode::V6_vgtb
        | Opcode::V6_vgtub
        | Opcode::V6_veqh
        | Opcode::V6_vgth
        | Opcode::V6_vgtuh
        | Opcode::V6_veqw
        | Opcode::V6_vgtw
        | Opcode::V6_vgtuw => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let cmp = match op {
                Opcode::V6_veqb => compare(1, |i| ub(&vu, i) == ub(&vv, i)),
                Opcode::V6_vgtb => compare(1, |i| sb(&vu, i) > sb(&vv, i)),
                Opcode::V6_vgtub => compare(1, |i| ub(&vu, i) > ub(&vv, i)),
                Opcode::V6_veqh => compare(2, |i| uh(&vu, i) == uh(&vv, i)),
                Opcode::V6_vgth => compare(2, |i| sh(&vu, i) > sh(&vv, i)),
                Opcode::V6_vgtuh => compare(2, |i| uh(&vu, i) > uh(&vv, i)),
                Opcode::V6_veqw => compare(4, |i| uw(&vu, i) == uw(&vv, i)),
                Opcode::V6_vgtw => compare(4, |i| sw(&vu, i) > sw(&vv, i)),
                // V6_vgtuw
                _ => compare(4, |i| uw(&vu, i) > uw(&vv, i)),
            };
            ctx.set_q(fld(d, b'd'), cmp);
            true
        }

        // ---- Q-predicate logic: Qd = OP(Qs, Qt) (per-bit) ----
        Opcode::V6_pred_and
        | Opcode::V6_pred_or
        | Opcode::V6_pred_xor
        | Opcode::V6_pred_and_n
        | Opcode::V6_pred_or_n
        | Opcode::V6_pred_not => {
            let qs = ctx.qread_new(fld(d, b's'));
            let qt = ctx.qread_new(fld(d, b't'));
            let mut out = [0u32; 4];
            for k in 0..4 {
                out[k] = match op {
                    Opcode::V6_pred_and => qs[k] & qt[k],
                    Opcode::V6_pred_or => qs[k] | qt[k],
                    Opcode::V6_pred_xor => qs[k] ^ qt[k],
                    Opcode::V6_pred_and_n => qs[k] & !qt[k],
                    Opcode::V6_pred_or_n => qs[k] | !qt[k],
                    // V6_pred_not
                    _ => !qs[k],
                };
            }
            ctx.set_q(fld(d, b'd'), out);
            true
        }

        // ---- vmux: Vd.b[i] = Qt.bit[i] ? Vu.b[i] : Vv.b[i] ----
        Opcode::V6_vmux => {
            let qt = ctx.qread_new(fld(d, b't'));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut o = [0u8; 128];
            for i in 0..128 {
                o[i] = if qbit(&qt, i) { vu[i] } else { vv[i] };
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }

        // ---- vandvqv: Vd.b[i] = Qv.bit[i] ? Vu.b[i] : 0  (vandvnqv negates) ----
        Opcode::V6_vandvqv | Opcode::V6_vandvnqv => {
            let qv = ctx.qread_new(fld(d, b'v'));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let neg = matches!(op, Opcode::V6_vandvnqv);
            let mut o = [0u8; 128];
            for i in 0..128 {
                o[i] = if qbit(&qv, i) ^ neg { vu[i] } else { 0 };
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }

        // ---- vandqrt: Vd.ub[i] = Qu.bit[i] ? Rt.byte[i%4] : 0  (n-form negates) ----
        Opcode::V6_vandqrt | Opcode::V6_vandnqrt => {
            let qu = ctx.qread_new(fld(d, b'u'));
            let rt = ctx.r(fld(d, b't')).to_le_bytes();
            let neg = matches!(op, Opcode::V6_vandnqrt);
            let mut o = [0u8; 128];
            for i in 0..128 {
                o[i] = if qbit(&qu, i) ^ neg { rt[i % 4] } else { 0 };
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }

        // ---- vandvrt: Qd.bit[i] = (Vu.ub[i] & Rt.byte[i%4]) != 0 ----
        Opcode::V6_vandvrt => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't')).to_le_bytes();
            let mut q = [0u32; 4];
            for i in 0..128 {
                set_qbit(&mut q, i, (vu[i] & rt[i % 4]) != 0);
            }
            ctx.set_q(fld(d, b'd'), q);
            true
        }

        _ => false,
    }
}
