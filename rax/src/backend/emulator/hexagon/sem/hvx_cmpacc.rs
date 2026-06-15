//! (hvx_cmpacc) compare-accumulate vector predicates:
//! `Qx {&=,|=,^=} vcmp.{gt,eq}(Vu.<t>, Vv.<t>)` for b/h/w (signed) and
//! ub/uh/uw (unsigned, gt only). Verified against the qemu-hexagon vector
//! oracle (tests/hexagon_hvx_diff.rs).
//!
//! Each op reads the existing architectural `Qx` vector-predicate (written by an
//! earlier packet — Hexagon does not forward Q within a packet), recomputes a
//! per-element compare into a per-vector-byte mask (`fSETQBITS`: a true element
//! sets every Q bit covering that element's bytes), and combines it bit-wise into
//! `Qx` with `&` / `|` / `^`. Semantics taken verbatim from the V69 HVX spec:
//!
//!     for( i = 0; i < VWIDTH; i += W ) {
//!         QxV[i+W-1:i] = QxV[i+W-1:i] [&|^] ((Vu.<t>[i/W] CMP Vv.<t>[i/W]) ? MASK : 0);
//!     }
//!
//! A Q vector-predicate stores one bit per vector byte: 128 bits = `[u32; 4]`,
//! bit `b` => `(Q.w[b>>5] >> (b&31)) & 1`. `vcmp.eq` is identical for signed and
//! unsigned operands, so the assembler maps the `eq` unsigned forms (ub/uh/uw)
//! onto the signed (b/h/w) encodings; only the `gt` forms have distinct
//! signed/unsigned opcodes.

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

// --- Q vector-predicate bit access (1 bit per vector byte) -----------------

/// Build a Q vector-predicate from a per-lane compare predicate `f(lane)`, where
/// each lane is `w` bytes wide: every vector byte covered by a true lane gets bit
/// 1, all others 0 (`fSETQBITS`).
fn compare<F: Fn(usize) -> bool>(w: usize, f: F) -> [u32; 4] {
    let mut q = [0u32; 4];
    let lanes = 128 / w;
    for lane in 0..lanes {
        if f(lane) {
            for byte in 0..w {
                let i = lane * w + byte;
                q[i >> 5] |= 1u32 << (i & 31);
            }
        }
    }
    q
}

/// Combine kind for the accumulate (`&=`, `|=`, `^=`).
#[derive(Clone, Copy)]
enum Acc {
    And,
    Or,
    Xor,
}

/// Execute a hvx_cmpacc opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    // Decode (compare-mask builder, accumulate kind) from the opcode; return
    // `false` for anything not in this class.
    let (cmp, acc): (fn(&Bytes, &Bytes) -> [u32; 4], Acc) = match op {
        // ---- signed/eq byte ----
        Opcode::V6_veqb_and => (|u, v| compare(1, |i| ub(u, i) == ub(v, i)), Acc::And),
        Opcode::V6_veqb_or => (|u, v| compare(1, |i| ub(u, i) == ub(v, i)), Acc::Or),
        Opcode::V6_veqb_xor => (|u, v| compare(1, |i| ub(u, i) == ub(v, i)), Acc::Xor),
        // ---- eq halfword ----
        Opcode::V6_veqh_and => (|u, v| compare(2, |i| uh(u, i) == uh(v, i)), Acc::And),
        Opcode::V6_veqh_or => (|u, v| compare(2, |i| uh(u, i) == uh(v, i)), Acc::Or),
        Opcode::V6_veqh_xor => (|u, v| compare(2, |i| uh(u, i) == uh(v, i)), Acc::Xor),
        // ---- eq word ----
        Opcode::V6_veqw_and => (|u, v| compare(4, |i| uw(u, i) == uw(v, i)), Acc::And),
        Opcode::V6_veqw_or => (|u, v| compare(4, |i| uw(u, i) == uw(v, i)), Acc::Or),
        Opcode::V6_veqw_xor => (|u, v| compare(4, |i| uw(u, i) == uw(v, i)), Acc::Xor),
        // ---- gt signed byte ----
        Opcode::V6_vgtb_and => (|u, v| compare(1, |i| sb(u, i) > sb(v, i)), Acc::And),
        Opcode::V6_vgtb_or => (|u, v| compare(1, |i| sb(u, i) > sb(v, i)), Acc::Or),
        Opcode::V6_vgtb_xor => (|u, v| compare(1, |i| sb(u, i) > sb(v, i)), Acc::Xor),
        // ---- gt signed halfword ----
        Opcode::V6_vgth_and => (|u, v| compare(2, |i| sh(u, i) > sh(v, i)), Acc::And),
        Opcode::V6_vgth_or => (|u, v| compare(2, |i| sh(u, i) > sh(v, i)), Acc::Or),
        Opcode::V6_vgth_xor => (|u, v| compare(2, |i| sh(u, i) > sh(v, i)), Acc::Xor),
        // ---- gt signed word ----
        Opcode::V6_vgtw_and => (|u, v| compare(4, |i| sw(u, i) > sw(v, i)), Acc::And),
        Opcode::V6_vgtw_or => (|u, v| compare(4, |i| sw(u, i) > sw(v, i)), Acc::Or),
        Opcode::V6_vgtw_xor => (|u, v| compare(4, |i| sw(u, i) > sw(v, i)), Acc::Xor),
        // ---- gt unsigned byte ----
        Opcode::V6_vgtub_and => (|u, v| compare(1, |i| ub(u, i) > ub(v, i)), Acc::And),
        Opcode::V6_vgtub_or => (|u, v| compare(1, |i| ub(u, i) > ub(v, i)), Acc::Or),
        Opcode::V6_vgtub_xor => (|u, v| compare(1, |i| ub(u, i) > ub(v, i)), Acc::Xor),
        // ---- gt unsigned halfword ----
        Opcode::V6_vgtuh_and => (|u, v| compare(2, |i| uh(u, i) > uh(v, i)), Acc::And),
        Opcode::V6_vgtuh_or => (|u, v| compare(2, |i| uh(u, i) > uh(v, i)), Acc::Or),
        Opcode::V6_vgtuh_xor => (|u, v| compare(2, |i| uh(u, i) > uh(v, i)), Acc::Xor),
        // ---- gt unsigned word ----
        Opcode::V6_vgtuw_and => (|u, v| compare(4, |i| uw(u, i) > uw(v, i)), Acc::And),
        Opcode::V6_vgtuw_or => (|u, v| compare(4, |i| uw(u, i) > uw(v, i)), Acc::Or),
        Opcode::V6_vgtuw_xor => (|u, v| compare(4, |i| uw(u, i) > uw(v, i)), Acc::Xor),
        _ => return false,
    };

    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
    let new = cmp(&vu, &vv);
    // Read-modify-write the architectural Qx (written by an earlier packet).
    let qx = fld(d, b'x');
    let old = ctx.qread(qx);
    let mut out = [0u32; 4];
    for k in 0..4 {
        out[k] = match acc {
            Acc::And => old[k] & new[k],
            Acc::Or => old[k] | new[k],
            Acc::Xor => old[k] ^ new[k],
        };
    }
    ctx.set_q(qx, out);
    true
}
