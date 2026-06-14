//! (hvx_lut) HVX vector lookup tables. Implements the byte/halfword vectorized
//! lookup-table ops: `vlut4` (4-entry table from a scalar pair), `vlut32`
//! (vlutvvb*, byte lookups into 32-entry sub-tables) and `vlut16` (vlutvwh*,
//! halfword lookups producing a vector pair). Verified against the qemu-hexagon
//! vector oracle (tests/hexagon_hvx_diff.rs). See sem/hvx_mpy.rs for the
//! 128-byte lane pattern and the SemCtx vector API.
//!
//! All forms assume the emulator's 1024-bit (128-byte) HVX configuration:
//! VECTOR_SIZE = 128 bytes, so `log2(VECTOR_SIZE) - 6 == 1` and the selector's
//! `oddhalf` bit is `(sel >> 1) & 1`.

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
#[inline]
fn get_uh(b: &Bytes, i: usize) -> u16 {
    u16::from_le_bytes([b[i * 2], b[i * 2 + 1]])
}
#[inline]
fn set_h(b: &mut Bytes, i: usize, val: u16) {
    b[i * 2..i * 2 + 2].copy_from_slice(&val.to_le_bytes());
}

/// `oddhalf` bit of a selector, for the 128-byte (1024-bit) configuration.
#[inline]
fn oddhalf(sel: u32) -> usize {
    ((sel >> 1) & 0x1) as usize
}

/// Execute an HVX lookup-table opcode. Returns `false` if not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // Vd.h = vlut4(Vu.uh, Rtt.h): each halfword of Vu selects (via its top
        // two bits) one of four halfwords packed in scalar pair Rtt.
        Opcode::V6_vlut4 => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rtt = ctx.rp(fld(d, b't'));
            let mut out = [0u8; 128];
            for i in 0..64 {
                let sel = ((get_uh(&vu, i) >> 14) & 0x3) as u32;
                let entry = (rtt >> (sel * 16)) as u16;
                set_h(&mut out, i, entry);
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
            true
        }

        // --- vlut32 (vlutvvb*): byte lookups -------------------------------
        // Vd.b = vlut32(Vu.b, Vv.b, Rt)
        Opcode::V6_vlutvvb => {
            let sel = ctx.r(fld(d, b't'));
            let out = vlut32(ctx, d, sel, false, None);
            ctx.set_v(fld(d, b'd'), out);
            true
        }
        // Vd.b = vlut32(Vu.b, Vv.b, Rt):nomatch
        Opcode::V6_vlutvvb_nm => {
            let sel = ctx.r(fld(d, b't'));
            let out = vlut32(ctx, d, sel, true, None);
            ctx.set_v(fld(d, b'd'), out);
            true
        }
        // Vd.b = vlut32(Vu.b, Vv.b, #u3)
        Opcode::V6_vlutvvbi => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let out = vlut32(ctx, d, imm, false, None);
            ctx.set_v(fld(d, b'd'), out);
            true
        }
        // Vx.b |= vlut32(Vu.b, Vv.b, Rt)
        Opcode::V6_vlutvvb_oracc => {
            let xr = fld(d, b'x');
            let prev = ctx.vread(xr);
            let sel = ctx.r(fld(d, b't'));
            let out = vlut32(ctx, d, sel, false, Some(prev));
            ctx.set_v(xr, out);
            true
        }
        // Vx.b |= vlut32(Vu.b, Vv.b, #u3)
        Opcode::V6_vlutvvb_oracci => {
            let xr = fld(d, b'x');
            let prev = ctx.vread(xr);
            let imm = fimm_u(d, b'i', ctx.immext);
            let out = vlut32(ctx, d, imm, false, Some(prev));
            ctx.set_v(xr, out);
            true
        }

        // --- vlut16 (vlutvwh*): halfword lookups, vector-pair dest ----------
        // Vdd.h = vlut16(Vu.b, Vv.h, Rt)
        Opcode::V6_vlutvwh => {
            let sel = ctx.r(fld(d, b't'));
            let (lo, hi) = vlut16(ctx, d, sel, false, None);
            let dd = fld(d, b'd');
            ctx.set_v(dd, lo);
            ctx.set_v(dd + 1, hi);
            true
        }
        // Vdd.h = vlut16(Vu.b, Vv.h, Rt):nomatch
        Opcode::V6_vlutvwh_nm => {
            let sel = ctx.r(fld(d, b't'));
            let (lo, hi) = vlut16(ctx, d, sel, true, None);
            let dd = fld(d, b'd');
            ctx.set_v(dd, lo);
            ctx.set_v(dd + 1, hi);
            true
        }
        // Vdd.h = vlut16(Vu.b, Vv.h, #u3)
        Opcode::V6_vlutvwhi => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let (lo, hi) = vlut16(ctx, d, imm, false, None);
            let dd = fld(d, b'd');
            ctx.set_v(dd, lo);
            ctx.set_v(dd + 1, hi);
            true
        }
        // Vxx.h |= vlut16(Vu.b, Vv.h, Rt)
        Opcode::V6_vlutvwh_oracc => {
            let xr = fld(d, b'x');
            let prev = (ctx.vread(xr), ctx.vread(xr + 1));
            let sel = ctx.r(fld(d, b't'));
            let (lo, hi) = vlut16(ctx, d, sel, false, Some(prev));
            ctx.set_v(xr, lo);
            ctx.set_v(xr + 1, hi);
            true
        }
        // Vxx.h |= vlut16(Vu.b, Vv.h, #u3)
        Opcode::V6_vlutvwh_oracci => {
            let xr = fld(d, b'x');
            let prev = (ctx.vread(xr), ctx.vread(xr + 1));
            let imm = fimm_u(d, b'i', ctx.immext);
            let (lo, hi) = vlut16(ctx, d, imm, false, Some(prev));
            ctx.set_v(xr, lo);
            ctx.set_v(xr + 1, hi);
            true
        }

        _ => false,
    }
}

/// Shared byte-lookup core for the vlut32 (`vlutvvb*`) family.
///
/// For each output byte `i`:
///   matchval = sel & 0x7; oddhalf = (sel >> 1) & 1; idx = Vu.ub[i];
///   match  : Vd.b[i] = ((idx & 0xE0) == (matchval<<5)) ? Vv.h[idx%64].b[oddhalf] : 0
///   nomatch: idx = (idx & 0x1F) | (matchval<<5); Vd.b[i] = Vv.h[idx%64].b[oddhalf]
/// `acc` (when Some) is OR-accumulated into.
fn vlut32(
    ctx: &SemCtx,
    d: &DecodedOp,
    sel: u32,
    nomatch: bool,
    acc: Option<[u32; 32]>,
) -> [u32; 32] {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
    let matchval = (sel & 0x7) as u8;
    let oh = oddhalf(sel);
    let mut out = match acc {
        Some(a) => to_bytes(&a),
        None => [0u8; 128],
    };
    for i in 0..128 {
        let idx = vu[i];
        let val: u8 = if nomatch {
            let lut_idx = ((idx & 0x1f) | (matchval << 5)) as usize;
            // Vv.h[lut_idx % 64].b[oddhalf]
            vv[(lut_idx % 64) * 2 + oh]
        } else if (idx & 0xe0) == (matchval << 5) {
            let lut_idx = idx as usize;
            vv[(lut_idx % 64) * 2 + oh]
        } else {
            0
        };
        if acc.is_some() {
            out[i] |= val;
        } else {
            out[i] = val;
        }
    }
    from_bytes(&out)
}

/// Shared halfword-lookup core for the vlut16 (`vlutvwh*`) family. Produces a
/// vector pair `(lo, hi)`.
///
/// For each output halfword `i` (0..64):
///   matchval = sel & 0xF; oddhalf = (sel >> 1) & 1;
///   idx0 = Vu.uh[i].ub[0]; idx1 = Vu.uh[i].ub[1];
///   match  : lo.h[i] = ((idx0 & 0xF0)==(matchval<<4)) ? Vv.w[idx0%32].h[oddhalf] : 0
///            hi.h[i] = ((idx1 & 0xF0)==(matchval<<4)) ? Vv.w[idx1%32].h[oddhalf] : 0
///   nomatch: idx = (idx & 0x0F) | (matchval<<4); lo/hi.h[i] = Vv.w[idx%32].h[oddhalf]
/// `acc` (when Some) is OR-accumulated into each half.
fn vlut16(
    ctx: &SemCtx,
    d: &DecodedOp,
    sel: u32,
    nomatch: bool,
    acc: Option<([u32; 32], [u32; 32])>,
) -> ([u32; 32], [u32; 32]) {
    let vu = to_bytes(&ctx.vread(fld(d, b'u')));
    let vv = to_bytes(&ctx.vread(fld(d, b'v')));
    let matchval = (sel & 0xf) as u8;
    let oh = oddhalf(sel);
    let (mut lo, mut hi) = match acc {
        Some((l, h)) => (to_bytes(&l), to_bytes(&h)),
        None => ([0u8; 128], [0u8; 128]),
    };
    // Read Vv as 32 words, each holding two halfwords; halfword `oh` of word w
    // is the halfword at index `w*2 + oh`.
    let lookup = |idx: u8| -> u16 {
        let w = (idx as usize) % 32;
        get_uh(&vv, w * 2 + oh)
    };
    for i in 0..64 {
        let idx0 = vu[i * 2];
        let idx1 = vu[i * 2 + 1];
        let (v0, v1): (u16, u16) = if nomatch {
            let l0 = (idx0 & 0x0f) | (matchval << 4);
            let l1 = (idx1 & 0x0f) | (matchval << 4);
            (lookup(l0), lookup(l1))
        } else {
            let m0 = if (idx0 & 0xf0) == (matchval << 4) {
                lookup(idx0)
            } else {
                0
            };
            let m1 = if (idx1 & 0xf0) == (matchval << 4) {
                lookup(idx1)
            } else {
                0
            };
            (m0, m1)
        };
        if acc.is_some() {
            let o0 = get_uh(&lo, i) | v0;
            let o1 = get_uh(&hi, i) | v1;
            set_h(&mut lo, i, o0);
            set_h(&mut hi, i, o1);
        } else {
            set_h(&mut lo, i, v0);
            set_h(&mut hi, i, v1);
        }
    }
    (from_bytes(&lo), from_bytes(&hi))
}
