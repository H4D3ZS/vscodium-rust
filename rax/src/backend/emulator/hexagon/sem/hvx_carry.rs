//! (hvx_carry) HVX carry-chain add/sub with a vector-predicate carry, the
//! unsigned-saturating word subtract, and the V62 set-predicate-v2 op. All taken
//! verbatim from the V68 HVX PRM pseudocode and verified against the
//! qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).
//!
//! Carry forms operate per 32-bit word lane (VELEM(32) = 32 lanes). The carry
//! bits live in a Q vector-predicate, which stores one bit per *vector byte*
//! (128 bits = `[u32; 4]`). For word lane `i` the PRM uses the 4-bit group
//! `QxV[4*i+3 : 4*i]`:
//!
//!   * carry-IN  is `QxV[i*4]` — the LSB bit of word `i`'s 4-bit group.
//!   * carry-OUT is written as `QxV[4*i+3:4*i] = -carry_from(...)` — i.e. all
//!     four bits of the group are set to the carry-out (all-1 if carry, else 0).
//!
//! `carry_from(a, b, cin)` is the unsigned add carry-out of bit 31:
//! `(a as u64 + b as u64 + cin as u64) >> 32 != 0`. Subtract is realised as
//! `Vu + ~Vv + cin` (two's-complement), exactly as the PRM writes it.
//!
//!     Vd.w,Qe4 = vadd(Vu.w,Vv.w):carry        (carryo: cin=0, separate Qe out)
//!     Vd.w,Qe4 = vsub(Vu.w,Vv.w):carry        (subcarryo: cin=1, separate Qe out)
//!     Vd.w     = vadd(Vu.w,Vv.w,Qx4):carry    (carry: cin/out share Qx)
//!     Vd.w     = vsub(Vu.w,Vv.w,Qx4):carry
//!     Vd.w     = vadd(Vu.w,Vv.w,Qs4):carry:sat (carry-in Qs, sat_32, no Q out)
//!     Vd.uw    = vsub(Vu.uw,Vv.uw):sat        (per-lane usat_32(Vu-Vv))
//!     Qd4      = vsetq2(Rt)                    (mask of length Rt, byte-granular)

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

/// Unsigned word `i` (0..31) of the 128-byte vector.
#[inline]
fn uw(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}

#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Read carry-in bit for word lane `i` from a Q predicate: bit position `4*i`.
#[inline]
fn qcarry_in(q: &[u32; 4], i: usize) -> u32 {
    let bit = i * 4;
    (q[bit >> 5] >> (bit & 31)) & 1
}

/// Write carry-out for word lane `i`: set the whole 4-bit group `[4*i+3:4*i]`
/// to `carry` (all-1 if carry, else all-0), matching `QxV[..] = -carry_from`.
#[inline]
fn qcarry_out(q: &mut [u32; 4], i: usize, carry: bool) {
    for byte in 0..4 {
        let bit = i * 4 + byte;
        let m = 1u32 << (bit & 31);
        if carry {
            q[bit >> 5] |= m;
        } else {
            q[bit >> 5] &= !m;
        }
    }
}

/// Add-with-carry of two 32-bit words: returns `(sum32, carry_out)`.
#[inline]
fn addc(a: u32, b: u32, cin: u32) -> (u32, bool) {
    let full = a as u64 + b as u64 + cin as u64;
    (full as u32, (full >> 32) != 0)
}

/// Execute a hvx_carry opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- Vd.w,Qe4 = vadd/vsub(Vu.w,Vv.w):carry (separate carry-out Q) ----
        // carryo: cin=0, b=Vv;  subcarryo: cin=1, b=~Vv.
        Opcode::V6_vaddcarryo | Opcode::V6_vsubcarryo => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let sub = matches!(op, Opcode::V6_vsubcarryo);
            let cin = if sub { 1 } else { 0 };
            let mut out = [0u8; 128];
            let mut qe = [0u32; 4];
            for i in 0..32 {
                let b = if sub { !uw(&vv, i) } else { uw(&vv, i) };
                let (sum, carry) = addc(uw(&vu, i), b, cin);
                set_w(&mut out, i, sum);
                qcarry_out(&mut qe, i, carry);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            ctx.set_q(fld(d, b'e'), qe);
            true
        }

        // ---- Vd.w = vadd/vsub(Vu.w,Vv.w,Qx4):carry (Qx is carry-in AND out) ----
        Opcode::V6_vaddcarry | Opcode::V6_vsubcarry => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let sub = matches!(op, Opcode::V6_vsubcarry);
            // Qx comes from an EARLIER packet (Hexagon does not forward Q within
            // a packet): read the OLD architectural Q.
            let qx = fld(d, b'x');
            let qin = ctx.qread(qx);
            let mut out = [0u8; 128];
            let mut qout = [0u32; 4];
            for i in 0..32 {
                let cin = qcarry_in(&qin, i);
                // sub: Vu + ~Vv + cin   add: Vu + Vv + cin
                let b = if sub { !uw(&vv, i) } else { uw(&vv, i) };
                let (sum, carry) = addc(uw(&vu, i), b, cin);
                set_w(&mut out, i, sum);
                qcarry_out(&mut qout, i, carry);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            ctx.set_q(qx, qout);
            true
        }

        // ---- Vd.w = vadd(Vu.w,Vv.w,Qs4):carry:sat (carry-in only, saturating) ----
        // Vd.w[i] = sat_32(Vu.w[i] + Vv.w[i] + QsV[i*4]); no carry-out written.
        Opcode::V6_vaddcarrysat => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let qs = ctx.qread(fld(d, b's'));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let cin = qcarry_in(&qs, i) as i64;
                let s = uw(&vu, i) as i32 as i64 + uw(&vv, i) as i32 as i64 + cin;
                let sat = ctx.sat_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // ---- Vd.uw = vsub(Vu.uw,Vv.uw):sat (per-lane usat_32(Vu - Vv)) ----
        Opcode::V6_vsubuwsat => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let s = uw(&vu, i) as i64 - uw(&vv, i) as i64;
                let sat = ctx.satu_n(s, 32);
                set_w(&mut out, i, sat as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // ---- Qd4 = vsetq2(Rt): byte mask of length Rt (vector-byte granular) ----
        // QdV[i] = (i <= ((Rt-1) & (VWIDTH-1))) ? 1 : 0, VWIDTH = 128 bytes.
        // Per the qemu idef this is a literal `(Rt-1) & 127` with no Rt==0 guard,
        // so Rt==0 wraps to length 128 (all bytes set).
        Opcode::V6_pred_scalar2v2 => {
            let rt = ctx.r(fld(d, b't'));
            let last = (rt.wrapping_sub(1) & 127) as usize;
            let mut q = [0u32; 4];
            for i in 0..=last {
                q[i >> 5] |= 1u32 << (i & 31);
            }
            ctx.set_q(fld(d, b'd'), q);
            true
        }

        _ => false,
    }
}
