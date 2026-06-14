//! (hvx_round) HVX rounding / saturating narrowing converts (`vround*`, `vsat*`),
//! the per-element variable-shift narrows (`vasrv*sat`), the accumulating shifts
//! (`v{asr,asl}{h,w}_acc`) and the shift-into overlay (`vasr_into`).
//!
//! The narrowing converts INTERLEAVE their two source vectors: output element
//! `2*i` comes from `Vv` element `i`, output element `2*i+1` from `Vu` element
//! `i` (matching `fSETHALF(0/1, VdV.w[i], …)` / `fSETBYTE(0/1, VdV.h[i], …)`).
//!
//! Saturation uses the Hexagon `fVSAT{B,UB,H,UH,W,DW}` primitives, which clamp
//! and flag USR overflow via `ctx.sat_n` / `ctx.satu_n`. Rounding (`:rnd`) adds
//! the round bias `1 << (shamt-1)` (or `0x80` / `0x8000` for the fixed-width
//! rounds) BEFORE the arithmetic right shift, matching `fVROUND`.
//!
//! Verified against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).
//! The four `vasrv*sat` per-element variable narrows are V69+ instructions: they
//! do not assemble under the harness `-mcpu=hexagonv68`, so they are implemented
//! here for completeness but cannot be differentially verified at v68.

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

// --- per-lane element readers (signed/unsigned) ----------------------------

#[inline]
fn get_h(b: &Bytes, i: usize) -> i64 {
    i16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i64
}
#[inline]
fn get_uh(b: &Bytes, i: usize) -> i64 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]) as i64
}
#[inline]
fn get_w(b: &Bytes, i: usize) -> i64 {
    i32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]) as i64
}
#[inline]
fn get_uw(b: &Bytes, i: usize) -> i64 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]) as i64
}
#[inline]
fn set_b(b: &mut Bytes, i: usize, val: u8) {
    b[i] = val;
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}
#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Narrowing kind: which element width / signedness the saturation targets.
#[derive(Clone, Copy)]
enum NarrowKind {
    /// half -> signed byte (fVSATB)
    Hb,
    /// half -> unsigned byte (fVSATUB)
    Hub,
    /// word -> signed half (fVSATH)
    Wh,
    /// word -> unsigned half (fVSATUH)
    Wuh,
}

/// Execute a hvx_round opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vround*: round (+bias) then >>w/2, saturate, interleave Vv/Vu ----
        // half->byte (16-bit iter, bias 0x80, >>8); word->half (32-bit iter,
        // bias 0x8000, >>16). Vu/Vv read signed or unsigned per the mnemonic.
        Opcode::V6_vroundhb
        | Opcode::V6_vroundhub
        | Opcode::V6_vrounduhub
        | Opcode::V6_vroundwh
        | Opcode::V6_vroundwuh
        | Opcode::V6_vrounduwuh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            match op {
                // half -> byte
                Opcode::V6_vroundhb => round_hb(ctx, &vu, &vv, &mut out, get_h, NarrowKind::Hb),
                Opcode::V6_vroundhub => round_hb(ctx, &vu, &vv, &mut out, get_h, NarrowKind::Hub),
                Opcode::V6_vrounduhub => round_hb(ctx, &vu, &vv, &mut out, get_uh, NarrowKind::Hub),
                // word -> half
                Opcode::V6_vroundwh => round_wh(ctx, &vu, &vv, &mut out, get_w, NarrowKind::Wh),
                Opcode::V6_vroundwuh => round_wh(ctx, &vu, &vv, &mut out, get_w, NarrowKind::Wuh),
                // V6_vrounduwuh
                _ => round_wh(ctx, &vu, &vv, &mut out, get_uw, NarrowKind::Wuh),
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // ---- vsat*: saturate (no shift) and interleave Vv/Vu ----
        Opcode::V6_vsathub | Opcode::V6_vsatwh | Opcode::V6_vsatuwuh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            match op {
                // half -> unsigned byte
                Opcode::V6_vsathub => {
                    for i in 0..64 {
                        set_b(&mut out, 2 * i, satb(ctx, get_h(&vv, i), NarrowKind::Hub));
                        set_b(
                            &mut out,
                            2 * i + 1,
                            satb(ctx, get_h(&vu, i), NarrowKind::Hub),
                        );
                    }
                }
                // word -> signed half
                Opcode::V6_vsatwh => {
                    for i in 0..32 {
                        set_h(
                            &mut out,
                            2 * i,
                            sath(ctx, get_w(&vv, i), NarrowKind::Wh) as u16,
                        );
                        set_h(
                            &mut out,
                            2 * i + 1,
                            sath(ctx, get_w(&vu, i), NarrowKind::Wh) as u16,
                        );
                    }
                }
                // V6_vsatuwuh: unsigned word -> unsigned half
                _ => {
                    for i in 0..32 {
                        set_h(
                            &mut out,
                            2 * i,
                            sath(ctx, get_uw(&vv, i), NarrowKind::Wuh) as u16,
                        );
                        set_h(
                            &mut out,
                            2 * i + 1,
                            sath(ctx, get_uw(&vu, i), NarrowKind::Wuh) as u16,
                        );
                    }
                }
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // ---- vsatdw: saturate {Vu.w (hi32):Vv.w (lo32)} 64-bit -> signed 32 ----
        Opcode::V6_vsatdw => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let hi = get_w(&vu, i); // sign of the 64-bit value
                let lo = get_uw(&vv, i); // low 32 bits, zero-extended
                let val = (hi << 32) | lo;
                let s = ctx.sat_n(val, 32);
                set_w(&mut out, i, s as u32);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // ---- vasrv*sat: per-element variable shift narrow (V69+, pair source) ----
        // Vd.uh = vasr(Vuu.w,  Vv.uh)[:rnd]:sat  (word -> unsigned half)
        // Vd.ub = vasr(Vuu.uh, Vv.ub)[:rnd]:sat  (uhalf -> unsigned byte)
        Opcode::V6_vasrvwuhsat
        | Opcode::V6_vasrvwuhrndsat
        | Opcode::V6_vasrvuhubsat
        | Opcode::V6_vasrvuhubrndsat => {
            let ubase = fld(d, b'u');
            let v0 = to_bytes(&ctx.vread(ubase)); // Vuu.v[0]
            let v1 = to_bytes(&ctx.vread(ubase + 1)); // Vuu.v[1]
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            match op {
                // word -> unsigned half, shamt = Vv.uh[2i+{0,1}] & 0xF
                Opcode::V6_vasrvwuhsat | Opcode::V6_vasrvwuhrndsat => {
                    let rnd = op == Opcode::V6_vasrvwuhrndsat;
                    for i in 0..32 {
                        let s0 = (get_uh(&vv, 2 * i) & 0xF) as u32;
                        let r0 = shift_round(get_w(&v0, i), s0, rnd);
                        set_h(&mut out, 2 * i, sath(ctx, r0, NarrowKind::Wuh) as u16);
                        let s1 = (get_uh(&vv, 2 * i + 1) & 0xF) as u32;
                        let r1 = shift_round(get_w(&v1, i), s1, rnd);
                        set_h(&mut out, 2 * i + 1, sath(ctx, r1, NarrowKind::Wuh) as u16);
                    }
                }
                // unsigned half -> unsigned byte, shamt = Vv.ub[2i+{0,1}] & 0x7
                _ => {
                    let rnd = op == Opcode::V6_vasrvuhubrndsat;
                    for i in 0..64 {
                        let s0 = (vv[2 * i] & 0x7) as u32;
                        let r0 = shift_round(get_uh(&v0, i), s0, rnd);
                        set_b(&mut out, 2 * i, satb(ctx, r0, NarrowKind::Hub));
                        let s1 = (vv[2 * i + 1] & 0x7) as u32;
                        let r1 = shift_round(get_uh(&v1, i), s1, rnd);
                        set_b(&mut out, 2 * i + 1, satb(ctx, r1, NarrowKind::Hub));
                    }
                }
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // ---- accumulating shifts: Vx.<w> += (Vu.<w> {<<,>>} (Rt & (W-1))) ----
        // Arithmetic shifts; the accumulate wraps at the element width.
        Opcode::V6_vaslw_acc
        | Opcode::V6_vasrw_acc
        | Opcode::V6_vaslh_acc
        | Opcode::V6_vasrh_acc => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't'));
            let dst = fld(d, b'x');
            let mut out = to_bytes(&ctx.vread(dst));
            match op {
                Opcode::V6_vaslw_acc => {
                    let sh = rt & 31;
                    for i in 0..32 {
                        let s = (get_w(&vu, i) as i32).wrapping_shl(sh);
                        let acc = (get_w(&out, i) as i32).wrapping_add(s);
                        set_w(&mut out, i, acc as u32);
                    }
                }
                Opcode::V6_vasrw_acc => {
                    let sh = rt & 31;
                    for i in 0..32 {
                        let s = (get_w(&vu, i) as i32) >> sh;
                        let acc = (get_w(&out, i) as i32).wrapping_add(s);
                        set_w(&mut out, i, acc as u32);
                    }
                }
                Opcode::V6_vaslh_acc => {
                    let sh = rt & 15;
                    for i in 0..64 {
                        let s = (get_h(&vu, i) as i16).wrapping_shl(sh);
                        let acc = (get_h(&out, i) as i16).wrapping_add(s);
                        set_h(&mut out, i, acc as u16);
                    }
                }
                // V6_vasrh_acc
                _ => {
                    let sh = rt & 15;
                    for i in 0..64 {
                        let s = (get_h(&vu, i) as i16) >> sh;
                        let acc = (get_h(&out, i) as i16).wrapping_add(s);
                        set_h(&mut out, i, acc as u16);
                    }
                }
            }
            ctx.set_v(dst, from_bytes(&out));
            true
        }

        // ---- vasr_into: ASR Vu.w into the running pair Vxx, overlaying bits ----
        // For each word lane: shift Vu.w[i] (sign-extended, placed in the high 32
        // bits) into a 64-bit accumulator whose initial value is Vxx.v[0].w[i]
        // (sign-extended high, zero-extended low). count = bidirectional shift
        // amount from Vv.w[i] ([-0x40,0x3f]); the dropped low bits of Vu spill
        // into the MSBs of Vxx.v[0]. Result split: hi32 -> Vxx.v[1], lo32 ->
        // Vxx.v[0].
        Opcode::V6_vasr_into => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let xbase = fld(d, b'x');
            let mut x0 = to_bytes(&ctx.vread(xbase)); // Vxx.v[0]
            let mut x1 = to_bytes(&ctx.vread(xbase + 1)); // Vxx.v[1]
            for i in 0..32 {
                let shift = (get_w(&vu, i)) << 32; // fSE32_64(Vu.w[i]) << 32
                let xlo = x0_word(&x0, i);
                let mask = (get_w(&x0, i) << 32) | (xlo as i64); // SE hi | ZE lo
                let lomask: i64 = ((1i64) << 32) - 1; // (fSE32_64(1) << 32) - 1
                let vvw = get_w(&vv, i) as i32;
                let count = -(0x40 & vvw) + (vvw & 0x3f);
                let result: i64 = if count == -0x40 {
                    0
                } else if count < 0 {
                    let n = (-count) as u32;
                    ((shift as i64) << n) | (mask & (lomask << n))
                } else {
                    let n = count as u32;
                    ((shift as i64) >> n) | (mask & ((lomask as u64 >> n) as i64))
                };
                set_w(&mut x1, i, ((result >> 32) & 0xffff_ffff) as u32);
                set_w(&mut x0, i, (result & 0xffff_ffff) as u32);
            }
            ctx.set_v(xbase, from_bytes(&x0));
            ctx.set_v(xbase + 1, from_bytes(&x1));
            true
        }

        _ => false,
    }
}

/// Zero-extended low word `i` of a vector (for the `vasr_into` mask).
#[inline]
fn x0_word(b: &Bytes, i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}

/// Saturate to a byte (signed `b` / unsigned `ub`), flagging USR overflow.
#[inline]
fn satb(ctx: &mut SemCtx, v: i64, k: NarrowKind) -> u8 {
    match k {
        NarrowKind::Hb => ctx.sat_n(v, 8) as u8,
        // Hub
        _ => ctx.satu_n(v, 8) as u8,
    }
}

/// Saturate to a half (signed `h` / unsigned `uh`), flagging USR overflow.
#[inline]
fn sath(ctx: &mut SemCtx, v: i64, k: NarrowKind) -> i16 {
    match k {
        NarrowKind::Wh => ctx.sat_n(v, 16) as i16,
        // Wuh
        _ => ctx.satu_n(v, 16) as i16,
    }
}

/// Add the round bias `1 << (shamt-1)` (when `rnd` and shamt>0) then arithmetic
/// shift right by `shamt`. Matches `fVROUND` / `fVNOROUND` followed by `>> shamt`.
#[inline]
fn shift_round(val: i64, shamt: u32, rnd: bool) -> i64 {
    let biased = if rnd && shamt > 0 {
        val + (1i64 << (shamt - 1))
    } else {
        val
    };
    biased >> shamt
}

/// half -> byte round: out[2i]=Vv, out[2i+1]=Vu; bias 0x80, >>8, saturate.
fn round_hb(
    ctx: &mut SemCtx,
    vu: &Bytes,
    vv: &Bytes,
    out: &mut Bytes,
    rd: fn(&Bytes, usize) -> i64,
    k: NarrowKind,
) {
    for i in 0..64 {
        let lo = (rd(vv, i) + 0x80) >> 8;
        let hi = (rd(vu, i) + 0x80) >> 8;
        set_b(out, 2 * i, satb(ctx, lo, k));
        set_b(out, 2 * i + 1, satb(ctx, hi, k));
    }
}

/// word -> half round: out[2i]=Vv, out[2i+1]=Vu; bias 0x8000, >>16, saturate.
fn round_wh(
    ctx: &mut SemCtx,
    vu: &Bytes,
    vv: &Bytes,
    out: &mut Bytes,
    rd: fn(&Bytes, usize) -> i64,
    k: NarrowKind,
) {
    for i in 0..32 {
        let lo = (rd(vv, i) + 0x8000) >> 16;
        let hi = (rd(vu, i) + 0x8000) >> 16;
        set_h(out, 2 * i, sath(ctx, lo, k) as u16);
        set_h(out, 2 * i + 1, sath(ctx, hi, k) as u16);
    }
}
