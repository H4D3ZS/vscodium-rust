//! (hvx_predop) HVX vector-predicated add/sub and scalar-predicated moves.
//!
//! Two families, both taken verbatim from the V68 HVX PRM pseudocode and
//! verified against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs):
//!
//! * Vector-predicate add/sub (`if (Qv[!]) Vx{+,-}= Vu`): a read-modify-write of
//!   `Vx`, masked element-wise by a Q vector-predicate `Qv` produced in an
//!   EARLIER packet (Hexagon does not forward Q within a packet, so `Qv` is read
//!   from the OLD architectural Q file). The spec applies the condition per
//!   *vector byte* via `fCONDMASK{8,16,32}` / `fGENMASK{H,W}`, selecting each
//!   result byte from either the computed `Vx op Vu` or the unchanged `Vx`. For
//!   bytes (`fCONDMASK8`) this is a plain per-byte select; for halfword/word the
//!   mask is built from the per-byte Q bits, so we replicate it byte-by-byte.
//!
//! * Scalar-predicated move / combine (`if (Ps) Vd = Vu`, `vccombine`): gated by
//!   the LSB of the OLD scalar predicate `Ps` (`fLSBOLD`). When the condition is
//!   false the instruction CANCELs (no write), so we leave the destination
//!   untouched. `vccombine` writes the pair `Vdd.v[0] = Vv`, `Vdd.v[1] = Vu`.

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

/// Read vector-byte bit `i` (0..128) from a Q predicate.
#[inline]
fn qbit(q: &[u32; 4], i: usize) -> bool {
    (q[i >> 5] >> (i & 31)) & 1 != 0
}

/// Conditional add/sub: each element `i` is `w` bytes wide. The result element
/// is `Vx op Vu` (add or sub at the element width); then each byte of that
/// element is selected from the operation result or the unchanged `Vx` byte
/// according to the corresponding Q bit (`fCONDMASK{8,16,32}` semantics).
/// `q_selects_op` is the Q-bit polarity that takes the operation result: `true`
/// for the q-form (`if (Qv) ...`), `false` for the nq-form (`if (!Qv) ...`).
fn cond_addsub(
    vx: &Bytes,
    vu: &Bytes,
    qv: &[u32; 4],
    w: usize,
    add: bool,
    q_selects_op: bool,
) -> Bytes {
    let mut out = *vx;
    let lanes = 128 / w;
    for lane in 0..lanes {
        let base = lane * w;
        // Compute the full-width element result of `Vx op Vu` (wrapping; these
        // forms are non-saturating).
        let opres: [u8; 4] = match w {
            1 => {
                let a = vx[base];
                let b = vu[base];
                let r = if add {
                    a.wrapping_add(b)
                } else {
                    a.wrapping_sub(b)
                };
                [r, 0, 0, 0]
            }
            2 => {
                let a = u16::from_le_bytes([vx[base], vx[base + 1]]);
                let b = u16::from_le_bytes([vu[base], vu[base + 1]]);
                let r = if add {
                    a.wrapping_add(b)
                } else {
                    a.wrapping_sub(b)
                };
                let rb = r.to_le_bytes();
                [rb[0], rb[1], 0, 0]
            }
            // w == 4
            _ => {
                let a = u32::from_le_bytes([vx[base], vx[base + 1], vx[base + 2], vx[base + 3]]);
                let b = u32::from_le_bytes([vu[base], vu[base + 1], vu[base + 2], vu[base + 3]]);
                let r = if add {
                    a.wrapping_add(b)
                } else {
                    a.wrapping_sub(b)
                };
                r.to_le_bytes()
            }
        };
        for byte in 0..w {
            let take_op = qbit(qv, base + byte) == q_selects_op;
            if take_op {
                out[base + byte] = opres[byte];
            }
        }
    }
    out
}

/// Execute a hvx_predop opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vector-predicated add/sub: if (Qv[!]) Vx {+,-}= Vu ----
        // `Qv` comes from an earlier packet (no in-packet Q forwarding): old Q.
        Opcode::V6_vaddbq
        | Opcode::V6_vaddbnq
        | Opcode::V6_vaddhq
        | Opcode::V6_vaddhnq
        | Opcode::V6_vaddwq
        | Opcode::V6_vaddwnq
        | Opcode::V6_vsubbq
        | Opcode::V6_vsubbnq
        | Opcode::V6_vsubhq
        | Opcode::V6_vsubhnq
        | Opcode::V6_vsubwq
        | Opcode::V6_vsubwnq => {
            let xreg = fld(d, b'x');
            let vx = to_bytes(&ctx.vread(xreg));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let qv = ctx.qread(fld(d, b'v'));
            let (w, add, q_selects_op) = match op {
                Opcode::V6_vaddbq => (1, true, true),
                Opcode::V6_vaddbnq => (1, true, false),
                Opcode::V6_vaddhq => (2, true, true),
                Opcode::V6_vaddhnq => (2, true, false),
                Opcode::V6_vaddwq => (4, true, true),
                Opcode::V6_vaddwnq => (4, true, false),
                Opcode::V6_vsubbq => (1, false, true),
                Opcode::V6_vsubbnq => (1, false, false),
                Opcode::V6_vsubhq => (2, false, true),
                Opcode::V6_vsubhnq => (2, false, false),
                Opcode::V6_vsubwq => (4, false, true),
                // V6_vsubwnq
                _ => (4, false, false),
            };
            let out = cond_addsub(&vx, &vu, &qv, w, add, q_selects_op);
            ctx.set_v(xreg, from_bytes(&out));
            true
        }

        // ---- scalar-predicated move: if (Ps) Vd = Vu  (vncmov: if (!Ps)) ----
        // CANCEL (no write) when the condition is false.
        Opcode::V6_vcmov | Opcode::V6_vncmov => {
            let ps = ctx.p(fld(d, b's')) & 1;
            let take = if matches!(op, Opcode::V6_vncmov) {
                ps == 0
            } else {
                ps != 0
            };
            if take {
                let vu = ctx.vread(fld(d, b'u'));
                ctx.set_v(fld(d, b'd'), vu);
            }
            true
        }

        // ---- scalar-predicated combine: if (Ps) Vdd = vcombine(Vu, Vv) ----
        // Vdd.v[0] = Vv (low), Vdd.v[1] = Vu (high). CANCEL when false.
        Opcode::V6_vccombine | Opcode::V6_vnccombine => {
            let ps = ctx.p(fld(d, b's')) & 1;
            let take = if matches!(op, Opcode::V6_vnccombine) {
                ps == 0
            } else {
                ps != 0
            };
            if take {
                let vu = ctx.vread(fld(d, b'u'));
                let vv = ctx.vread(fld(d, b'v'));
                let dbase = fld(d, b'd');
                ctx.set_v(dbase, vv);
                ctx.set_v(dbase + 1, vu);
            }
            true
        }

        _ => false,
    }
}
