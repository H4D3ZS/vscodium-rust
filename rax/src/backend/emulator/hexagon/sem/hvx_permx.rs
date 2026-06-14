//! (hvx_permx) HVX extended permutes — verified against the qemu-hexagon vector
//! oracle (tests/hexagon_hvx_diff.rs). Semantics taken verbatim from the V60/V69
//! HVX Programmer's Reference Manual pseudocode.
//!
//! Members:
//!  - `vshuff`/`vdeal` — in-place dual-register full shuffle/deal `OP(Vy,Vx,Rt)`:
//!    a sequence of conditional cross-register byte swaps controlled by Rt, where
//!    Vy (field 'y') and Vx (field 'x') are BOTH read and written.
//!  - `vdealvdd` — pair-dest deal `Vdd=vdeal(Vu,Vv,Rt)` (v[0]=Vv, v[1]=Vu).
//!  - `vshufoeb`/`vshufoeh` — odd/even shuffle into a pair (v[0]=even, v[1]=odd).
//!  - `vdelta`/`vrdelta` — log-stage byte permute networks controlled by Vv.
//!  - `vswap` — Q-selected byte swap into a pair (v[0]=Q?Vu:Vv, v[1]=Q?Vv:Vu).
//!  - `vrotr` — per-word bit rotate-right of Vu.uw by Vv.uw (mod 32).
//!  - `vassign_tmp`/`vcombine_tmp` — `.tmp` register moves (see note below).
//!
//! Lane conventions match sem/hvx.rs: a vector is 128 bytes, little-endian within
//! each u32 word. `fVELEM(8)=128`, the full-vector byte count (VWIDTH).

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

/// 128-byte vector viewed as raw bytes (little-endian within each u32 word).
type Bytes = [u8; 128];

const VWIDTH: usize = 128;

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

/// Read vector-byte bit `i` (0..128) of a Q predicate (`fGETQBIT`).
#[inline]
fn qbit(q: &[u32; 4], i: usize) -> bool {
    (q[i >> 5] >> (i & 31)) & 1 != 0
}

/// Write the (even, odd) halves of a 256-byte vector pair to `Vdd`.
#[inline]
fn set_pair(ctx: &mut SemCtx, rd: u8, lo: &Bytes, hi: &Bytes) {
    ctx.set_v(rd, from_bytes(lo));
    ctx.set_v(rd + 1, from_bytes(hi));
}

/// Execute a hvx_permx opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // ---- vshuff(Vy,Vx,Rt): in-place dual-register shuffle network --------
        // for (offset=1; offset<VWIDTH; offset<<=1)
        //   if (Rt & offset) for k: if !(k & offset) SWAP(Vy.ub[k], Vx.ub[k+offset])
        Opcode::V6_vshuff => {
            let ry = fld(d, b'y');
            let rx = fld(d, b'x');
            let rt = ctx.r(fld(d, b't')) as usize;
            let mut vy = to_bytes(&ctx.vread(ry));
            let mut vx = to_bytes(&ctx.vread(rx));
            let mut offset = 1usize;
            while offset < VWIDTH {
                if rt & offset != 0 {
                    for k in 0..VWIDTH {
                        if k & offset == 0 {
                            std::mem::swap(&mut vy[k], &mut vx[k + offset]);
                        }
                    }
                }
                offset <<= 1;
            }
            ctx.set_v(ry, from_bytes(&vy));
            ctx.set_v(rx, from_bytes(&vx));
        }

        // ---- vdeal(Vy,Vx,Rt): in-place dual-register deal network ------------
        // for (offset=VWIDTH>>1; offset>0; offset>>=1)
        //   if (Rt & offset) for k: if !(k & offset) SWAP(Vy.ub[k], Vx.ub[k+offset])
        Opcode::V6_vdeal => {
            let ry = fld(d, b'y');
            let rx = fld(d, b'x');
            let rt = ctx.r(fld(d, b't')) as usize;
            let mut vy = to_bytes(&ctx.vread(ry));
            let mut vx = to_bytes(&ctx.vread(rx));
            let mut offset = VWIDTH >> 1;
            while offset > 0 {
                if rt & offset != 0 {
                    for k in 0..VWIDTH {
                        if k & offset == 0 {
                            std::mem::swap(&mut vy[k], &mut vx[k + offset]);
                        }
                    }
                }
                offset >>= 1;
            }
            ctx.set_v(ry, from_bytes(&vy));
            ctx.set_v(rx, from_bytes(&vx));
        }

        // ---- vdealvdd: Vdd=vdeal(Vu,Vv,Rt) -----------------------------------
        // v[0]=Vv, v[1]=Vu, then the deal network swaps v[1].ub[k] <-> v[0].ub[k+offset]
        Opcode::V6_vdealvdd => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let rt = ctx.r(fld(d, b't')) as usize;
            let mut lo = vv; // v[0]
            let mut hi = vu; // v[1]
            let mut offset = VWIDTH >> 1;
            while offset > 0 {
                if rt & offset != 0 {
                    for k in 0..VWIDTH {
                        if k & offset == 0 {
                            std::mem::swap(&mut hi[k], &mut lo[k + offset]);
                        }
                    }
                }
                offset >>= 1;
            }
            set_pair(ctx, fld(d, b'd'), &lo, &hi);
        }

        // ---- vshufoeb: Vdd.b = vshuffoe(Vu.b, Vv.b) --------------------------
        // v[0] = vshuffe(Vu,Vv) (even bytes), v[1] = vshuffo (odd bytes):
        //   v[0].uh[i].b[0]=Vv.ub[2i]; v[0].uh[i].b[1]=Vu.ub[2i]
        //   v[1].uh[i].b[0]=Vv.ub[2i+1]; v[1].uh[i].b[1]=Vu.ub[2i+1]
        Opcode::V6_vshufoeb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..64 {
                lo[i * 2] = vv[i * 2];
                lo[i * 2 + 1] = vu[i * 2];
                hi[i * 2] = vv[i * 2 + 1];
                hi[i * 2 + 1] = vu[i * 2 + 1];
            }
            set_pair(ctx, fld(d, b'd'), &lo, &hi);
        }

        // ---- vshufoeh: Vdd.h = vshuffoe(Vu.h, Vv.h) --------------------------
        //   v[0].uw[i].h[0]=Vv.uh[2i]; v[0].uw[i].h[1]=Vu.uh[2i]
        //   v[1].uw[i].h[0]=Vv.uh[2i+1]; v[1].uw[i].h[1]=Vu.uh[2i+1]
        Opcode::V6_vshufoeh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..32 {
                set_h(&mut lo, i * 2, get_h(&vv, i * 2));
                set_h(&mut lo, i * 2 + 1, get_h(&vu, i * 2));
                set_h(&mut hi, i * 2, get_h(&vv, i * 2 + 1));
                set_h(&mut hi, i * 2 + 1, get_h(&vu, i * 2 + 1));
            }
            set_pair(ctx, fld(d, b'd'), &lo, &hi);
        }

        // ---- vdelta: Vd = vdelta(Vu, Vv) -------------------------------------
        // for (offset=VWIDTH; (offset>>=1)>0; )
        //   for k: Vd.ub[k] = (Vv.ub[k]&offset) ? Vu.ub[k^offset] : Vu.ub[k];
        //   then Vu = Vd  (each stage feeds the next).
        Opcode::V6_vdelta => {
            let mut vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            let mut offset = VWIDTH;
            loop {
                offset >>= 1;
                if offset == 0 {
                    break;
                }
                for k in 0..VWIDTH {
                    out[k] = if vv[k] as usize & offset != 0 {
                        vu[k ^ offset]
                    } else {
                        vu[k]
                    };
                }
                vu = out;
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&vu));
        }

        // ---- vrdelta: Vd = vrdelta(Vu, Vv) (delta with strides ascending) ----
        // for (offset=1; offset<VWIDTH; offset<<=1) { ... same body ... }
        Opcode::V6_vrdelta => {
            let mut vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            let mut offset = 1usize;
            while offset < VWIDTH {
                for k in 0..VWIDTH {
                    out[k] = if vv[k] as usize & offset != 0 {
                        vu[k ^ offset]
                    } else {
                        vu[k]
                    };
                }
                vu = out;
                offset <<= 1;
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&vu));
        }

        // ---- vswap: Vdd = vswap(Qt, Vu, Vv) ----------------------------------
        // v[0].ub[i] = Qt[i] ? Vu.ub[i] : Vv.ub[i];
        // v[1].ub[i] = Qt[i] ? Vv.ub[i] : Vu.ub[i];
        Opcode::V6_vswap => {
            let qt = ctx.qread_new(fld(d, b't'));
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..VWIDTH {
                if qbit(&qt, i) {
                    lo[i] = vu[i];
                    hi[i] = vv[i];
                } else {
                    lo[i] = vv[i];
                    hi[i] = vu[i];
                }
            }
            set_pair(ctx, fld(d, b'd'), &lo, &hi);
        }

        // ---- vrotr: Vd.uw = vrotr(Vu.uw, Vv.uw) ------------------------------
        // Vd.uw[i] = rotate_right(Vu.uw[i], Vv.uw[i] & 0x1f)
        Opcode::V6_vrotr => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let amt = get_w(&vv, i) & 0x1f;
                set_w(&mut out, i, get_w(&vu, i).rotate_right(amt));
            }
            ctx.set_v(fld(d, b'd'), from_bytes(&out));
        }

        // ---- vassign_tmp: Vd.tmp = Vu ----------------------------------------
        // Temporary assignment: the value is forwarded within the packet but the
        // PRM specifies it is NOT committed to the register file. The current sem
        // layer cannot express "forward-without-commit" (set_v both forwards via
        // .new and commits), and llvm-mc v68 has no assembly syntax for this
        // register-move .tmp form, so it is unverifiable. We compute the correct
        // value (Vd = Vu) so any in-packet .new consumer observes the right data.
        Opcode::V6_vassign_tmp => {
            let vu = ctx.vread(fld(d, b'u'));
            ctx.set_v(fld(d, b'd'), vu);
        }

        // ---- vcombine_tmp: Vdd.tmp = vcombine(Vu, Vv) ------------------------
        // Same .tmp caveat as vassign_tmp. v[0]=Vv, v[1]=Vu.
        Opcode::V6_vcombine_tmp => {
            let vu = ctx.vread(fld(d, b'u'));
            let vv = ctx.vread(fld(d, b'v'));
            let rd = fld(d, b'd');
            ctx.set_v(rd, vv);
            ctx.set_v(rd + 1, vu);
        }

        _ => return false,
    }
    true
}
