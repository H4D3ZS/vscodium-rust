//! (hvx_hist) HVX histogram instructions: vhist / vhist(Qv) / vwhist128* /
//! vwhist256*. Each tallies byte/halfword values from an input vector into
//! histogram bins held across the whole vector register file V0..V31. Matched
//! element-for-element to the V68 HVX PRM pseudocode and verified live against
//! the qemu-hexagon memory+vector oracle (tests/hexagon_hvx_mem_diff.rs).
//!
//! ## Input vector
//! These instructions are unusual: their input is the data from a `.tmp` vector
//! load that must appear earlier in the SAME packet (qemu asserts exactly one
//! `.tmp` load is present; a bare `{ vhist }` faults). The canonical idiom is
//! `{ v0.tmp = vmem(Rx+#0); vhist }`. rax forwards a `.tmp` load into the
//! per-packet scratch buffer (`new_v_tmp`, never committed), so `vread_new(0)`
//! returns the loaded bytes — matching qemu's `tmp_VRegs[0]` for the standard
//! `v0.tmp` form.
//!
//! ## Histogram model (PRM)
//! The register file is treated as 8 independent 128-bit "slices" (lanes), one
//! per 16-byte group of the 128-byte input. `VELEM(128) = 1024/128 = 8`.
//!
//! * vhist: for each of the 8 lanes, each of its 16 input bytes `value` selects
//!   `regno = value>>3` and `element = value&7`; `V[regno].uh[8*lane+element]`
//!   is incremented by 1 (wrapping at 16 bits). The Qv form gates each increment
//!   on the per-vector-byte predicate bit `Qv[16*lane+i]` (old Q, produced in an
//!   earlier packet — Hexagon does not forward Q within a packet).
//! * vwhist128: VELEM(16)=64 halfwords; the even byte is the bucket and the odd
//!   byte the weight. `vindex = (bucket>>3)&0x1F` selects the register;
//!   `elindex = ((i>>1)&~3) | ((bucket>>1)&3)` selects a 32-bit bin;
//!   `V[vindex].uw[elindex] += weight` (wrapping). Variants gate the add on the
//!   bucket LSB (`#u1`: `(bucket&1)==#u`) and/or the predicate bit `Qv[2*i]`.
//! * vwhist256: same bucket/weight split, `vindex = (bucket>>3)&0x1F`, but
//!   `elindex = (i&~7) | (bucket&7)` selects a 16-bit bin;
//!   `V[vindex].uh[elindex] += weight`. The `:sat` forms saturate the 16-bit bin
//!   unsigned (usat_16); the Qv forms gate on `Qv[2*i]`.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_u, fld};

/// Working register file as 32 * 128 bytes (little-endian within each word).
type File = [[u8; 128]; 32];

#[inline]
fn load_file(ctx: &SemCtx) -> File {
    let mut f = [[0u8; 128]; 32];
    for r in 0..32 {
        let v = ctx.vread(r as u8);
        for w in 0..32 {
            f[r][w * 4..w * 4 + 4].copy_from_slice(&v[w].to_le_bytes());
        }
    }
    f
}

#[inline]
fn store_file(ctx: &mut SemCtx, f: &File) {
    for r in 0..32 {
        let mut v = [0u32; 32];
        for w in 0..32 {
            v[w] = u32::from_le_bytes([
                f[r][w * 4],
                f[r][w * 4 + 1],
                f[r][w * 4 + 2],
                f[r][w * 4 + 3],
            ]);
        }
        ctx.set_v(r as u8, v);
    }
}

/// The histogram input: 128 bytes of the `.tmp`-loaded vector. The standard
/// idiom loads into V0, which rax forwards into the packet scratch buffer, so
/// `vread_new(0)` returns the loaded bytes (qemu's `tmp_VRegs[0]`).
#[inline]
fn input_bytes(ctx: &SemCtx) -> [u8; 128] {
    let v = ctx.vread_new(0);
    let mut b = [0u8; 128];
    for w in 0..32 {
        b[w * 4..w * 4 + 4].copy_from_slice(&v[w].to_le_bytes());
    }
    b
}

/// Read vector-byte predicate bit `i` (0..128) from a Q register value.
#[inline]
fn qbit(q: &[u32; 4], i: usize) -> bool {
    (q[i >> 5] >> (i & 31)) & 1 != 0
}

#[inline]
fn get_uh(f: &File, reg: usize, i: usize) -> u32 {
    u16::from_le_bytes([f[reg][i * 2], f[reg][i * 2 + 1]]) as u32
}
#[inline]
fn set_uh(f: &mut File, reg: usize, i: usize, val: u32) {
    f[reg][i * 2..i * 2 + 2].copy_from_slice(&(val as u16).to_le_bytes());
}
#[inline]
fn get_uw(f: &File, reg: usize, i: usize) -> u32 {
    u32::from_le_bytes([
        f[reg][i * 4],
        f[reg][i * 4 + 1],
        f[reg][i * 4 + 2],
        f[reg][i * 4 + 3],
    ])
}
#[inline]
fn set_uw(f: &mut File, reg: usize, i: usize, val: u32) {
    f[reg][i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Execute a hvx_hist opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vhist / vhist(Qv): 8 lanes x 16 bytes -> uh bins, +1 ----
        Opcode::V6_vhist | Opcode::V6_vhistq => {
            let inp = input_bytes(ctx);
            let qv = if op == Opcode::V6_vhistq {
                Some(ctx.qread(fld(d, b'v')))
            } else {
                None
            };
            let mut f = load_file(ctx);
            for lane in 0..8 {
                for i in 0..16 {
                    if let Some(ref q) = qv {
                        if !qbit(q, 16 * lane + i) {
                            continue;
                        }
                    }
                    let value = inp[16 * lane + i] as usize;
                    let regno = value >> 3;
                    let element = value & 7;
                    let idx = 8 * lane + element;
                    let cur = get_uh(&f, regno, idx);
                    set_uh(&mut f, regno, idx, cur.wrapping_add(1) & 0xffff);
                }
            }
            store_file(ctx, &f);
        }

        // ---- vwhist128 family: 64 halfwords -> uw bins, += weight ----
        // bucket = even byte, weight = odd byte; bits 2:1 of bucket pick the bin.
        Opcode::V6_vwhist128
        | Opcode::V6_vwhist128m
        | Opcode::V6_vwhist128q
        | Opcode::V6_vwhist128qm => {
            let inp = input_bytes(ctx);
            let qv = matches!(op, Opcode::V6_vwhist128q | Opcode::V6_vwhist128qm)
                .then(|| ctx.qread(fld(d, b'v')));
            let imm = matches!(op, Opcode::V6_vwhist128m | Opcode::V6_vwhist128qm)
                .then(|| (fimm_u(d, b'i', ctx.immext) & 1) as u8);
            let mut f = load_file(ctx);
            for i in 0..64 {
                let bucket = inp[2 * i] as usize;
                let weight = inp[2 * i + 1] as u32;
                let vindex = (bucket >> 3) & 0x1f;
                let elindex = ((i >> 1) & !3) | ((bucket >> 1) & 3);
                let mut cond = true;
                if let Some(u) = imm {
                    cond &= (bucket & 1) as u8 == u;
                }
                if let Some(ref q) = qv {
                    cond &= qbit(q, 2 * i);
                }
                if cond {
                    let cur = get_uw(&f, vindex, elindex);
                    set_uw(&mut f, vindex, elindex, cur.wrapping_add(weight));
                }
            }
            store_file(ctx, &f);
        }

        // ---- vwhist256 family: 64 halfwords -> uh bins, += weight (opt sat) ----
        // bucket = even byte, weight = odd byte; low 3 bits of bucket pick the bin.
        Opcode::V6_vwhist256
        | Opcode::V6_vwhist256_sat
        | Opcode::V6_vwhist256q
        | Opcode::V6_vwhist256q_sat => {
            let inp = input_bytes(ctx);
            let qv = matches!(op, Opcode::V6_vwhist256q | Opcode::V6_vwhist256q_sat)
                .then(|| ctx.qread(fld(d, b'v')));
            let sat = matches!(op, Opcode::V6_vwhist256_sat | Opcode::V6_vwhist256q_sat);
            let mut f = load_file(ctx);
            for i in 0..64 {
                let bucket = inp[2 * i] as usize;
                let weight = inp[2 * i + 1] as u32;
                let vindex = (bucket >> 3) & 0x1f;
                let elindex = (i & !7) | (bucket & 7);
                let cond = match qv {
                    Some(ref q) => qbit(q, 2 * i),
                    None => true,
                };
                if cond {
                    let sum = get_uh(&f, vindex, elindex).wrapping_add(weight);
                    let val = if sat { sum.min(0xffff) } else { sum & 0xffff };
                    set_uh(&mut f, vindex, elindex, val);
                }
            }
            store_file(ctx, &f);
        }

        _ => return false,
    }
    true
}
