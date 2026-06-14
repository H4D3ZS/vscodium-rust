//! (hvx_misc) Miscellaneous HVX bridge / housekeeping ops:
//!   * `vnot`             — bitwise NOT of a whole vector.
//!   * `vinsertwr`        — insert scalar Rt into word 0 of Vx (rest preserved).
//!   * `extractw`         — extract a vector word (selected by Rs) into a GPR.
//!   * `vandqrt_acc` / `vandnqrt_acc` — Q -> V byte bridge, OR-accumulated.
//!   * `vandvrt_acc`      — V -> Q byte bridge, OR-accumulated into Qx.
//!   * `vprefixq{b,h,w}`  — parallel prefix sum of a Q predicate into byte/h/w lanes.
//!   * `pred_scalar2`     — `vsetq`: set the low `Rt & 127` Q bits.
//!   * `shuffeqh` / `shuffeqw` — predicate shrink/shuffle of a Q pair.
//!
//! Semantics are taken verbatim from the V68 HVX spec
//! (`tools/hexagon/qemu/imported/mmvec/ext.idef`) and verified against the
//! qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs, `diff_hvx_misc`).

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
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
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

/// Execute a hvx_misc opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- Vd = vnot(Vu): bitwise NOT of the whole 128-byte vector ----
        Opcode::V6_vnot => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut o = [0u8; 128];
            for i in 0..128 {
                o[i] = !vu[i];
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }

        // ---- Vx.w = vinsert(Rt): VxV.uw[0] = RtV (other words preserved) ----
        Opcode::V6_vinsertwr => {
            let dst = fld(d, b'x');
            let rt = ctx.r(fld(d, b't'));
            let mut o = to_bytes(&ctx.vread(dst));
            set_w(&mut o, 0, rt);
            ctx.set_v(dst, from_bytes(&o));
            true
        }

        // ---- Rd = vextract(Vu, Rs): RdV = VuV.uw[(Rs & 127) >> 2] ----
        Opcode::V6_extractw => {
            let vu = ctx.vread(fld(d, b'u'));
            let rs = ctx.r(fld(d, b's'));
            let idx = ((rs & 127) >> 2) as usize;
            ctx.set_r(fld(d, b'd'), vu[idx]);
            true
        }

        // ---- Vx.ub |= vand(Qu.ub, Rt.ub) / vand(!Qu.ub, Rt.ub) ----
        // VxV.ub[i] |= (Qu.bit[i] ^ neg) ? Rt.byte[i%4] : 0
        Opcode::V6_vandqrt_acc | Opcode::V6_vandnqrt_acc => {
            let qu = ctx.qread_new(fld(d, b'u'));
            let rt = ctx.r(fld(d, b't')).to_le_bytes();
            let neg = matches!(op, Opcode::V6_vandnqrt_acc);
            let dst = fld(d, b'x');
            let mut o = to_bytes(&ctx.vread(dst));
            for i in 0..128 {
                if qbit(&qu, i) ^ neg {
                    o[i] |= rt[i % 4];
                }
            }
            ctx.set_v(dst, from_bytes(&o));
            true
        }

        // ---- Qx.ub |= vand(Vu.ub, Rt.ub) ----
        // QxV.bit[i] |= ((Vu.ub[i] & Rt.byte[i%4]) != 0)
        Opcode::V6_vandvrt_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't')).to_le_bytes();
            let dst = fld(d, b'x');
            let mut q = ctx.qread_new(dst);
            for i in 0..128 {
                if (vu[i] & rt[i % 4]) != 0 {
                    set_qbit(&mut q, i, true);
                }
            }
            ctx.set_q(dst, q);
            true
        }

        // ---- Vd.b = prefixsum(Qv): running count of Q bits, byte lanes ----
        Opcode::V6_vprefixqb => {
            let qv = ctx.qread_new(fld(d, b'v'));
            let mut o = [0u8; 128];
            let mut acc: u8 = 0;
            for i in 0..128 {
                acc = acc.wrapping_add(qbit(&qv, i) as u8);
                o[i] = acc;
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }
        // ---- Vd.h = prefixsum(Qv): running count over halfword lanes ----
        Opcode::V6_vprefixqh => {
            let qv = ctx.qread_new(fld(d, b'v'));
            let mut o = [0u8; 128];
            let mut acc: u16 = 0;
            for i in 0..64 {
                acc = acc.wrapping_add(qbit(&qv, i * 2) as u16);
                acc = acc.wrapping_add(qbit(&qv, i * 2 + 1) as u16);
                set_h(&mut o, i, acc);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }
        // ---- Vd.w = prefixsum(Qv): running count over word lanes ----
        Opcode::V6_vprefixqw => {
            let qv = ctx.qread_new(fld(d, b'v'));
            let mut o = [0u8; 128];
            let mut acc: u32 = 0;
            for i in 0..32 {
                for k in 0..4 {
                    acc = acc.wrapping_add(qbit(&qv, i * 4 + k) as u32);
                }
                set_w(&mut o, i, acc);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&o));
            true
        }

        // ---- Qd = vsetq(Rt): set the low `Rt & 127` byte-bits ----
        Opcode::V6_pred_scalar2 => {
            let rt = ctx.r(fld(d, b't'));
            let n = (rt & 127) as usize;
            let mut q = [0u32; 4];
            for i in 0..128 {
                set_qbit(&mut q, i, i < n);
            }
            ctx.set_q(fld(d, b'd'), q);
            true
        }

        // ---- Qd.b = vshuffe(Qs.h, Qt.h): Qd.bit[i] = (i&1)?Qs[i-1]:Qt[i] ----
        Opcode::V6_shuffeqh => {
            let qs = ctx.qread_new(fld(d, b's'));
            let qt = ctx.qread_new(fld(d, b't'));
            let mut q = [0u32; 4];
            for i in 0..128 {
                let v = if i & 1 != 0 {
                    qbit(&qs, i - 1)
                } else {
                    qbit(&qt, i)
                };
                set_qbit(&mut q, i, v);
            }
            ctx.set_q(fld(d, b'd'), q);
            true
        }
        // ---- Qd.h = vshuffe(Qs.w, Qt.w): Qd.bit[i] = (i&2)?Qs[i-2]:Qt[i] ----
        Opcode::V6_shuffeqw => {
            let qs = ctx.qread_new(fld(d, b's'));
            let qt = ctx.qread_new(fld(d, b't'));
            let mut q = [0u32; 4];
            for i in 0..128 {
                let v = if i & 2 != 0 {
                    qbit(&qs, i - 2)
                } else {
                    qbit(&qt, i)
                };
                set_qbit(&mut q, i, v);
            }
            ctx.set_q(fld(d, b'd'), q);
            true
        }

        _ => false,
    }
}
