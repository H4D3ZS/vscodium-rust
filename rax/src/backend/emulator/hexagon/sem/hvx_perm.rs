//! (hvx_perm) HVX permute / shuffle / deal / pack / widen instructions —
//! verified against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).
//! See sem/hvx.rs for the established 128-byte lane pattern and the SemCtx
//! vector API (vread/set_v). Semantics taken verbatim from the V68 spec
//! (`fVFOREACH(W,i){ Vd.elem[i] = ... }`, expanded via gen_semantics.c).
//!
//! Lane conventions (matching hvx.rs): a vector is 128 bytes, little-endian
//! within each u32 word. `fVELEM(W) = 1024/W` is the number of W-bit lanes
//! (8->128, 16->64, 32->32); element index `i` addresses lane `i` of that
//! width. `fGETUBYTE(n,x)`/`fGETUHALF(n,x)` extract sub-element `n` of a wider
//! lane (byte/half within a half/word).

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

// ---- saturation helpers (fVSAT*) -------------------------------------------
#[inline]
fn sat_ub(x: i32) -> u8 {
    x.clamp(0, 255) as u8
}
#[inline]
fn sat_b(x: i32) -> u8 {
    x.clamp(-128, 127) as u8
}
#[inline]
fn sat_uh(x: i32) -> u16 {
    x.clamp(0, 65535) as u16
}
#[inline]
fn sat_h(x: i32) -> u16 {
    x.clamp(-32768, 32767) as u16
}

/// Write the (even, odd) halves of a 256-byte vector pair to `Vdd`.
#[inline]
fn set_pair(ctx: &mut SemCtx, rd: u8, lo: &Bytes, hi: &Bytes) {
    ctx.set_v(rd, from_bytes(lo));
    ctx.set_v(rd + 1, from_bytes(hi));
}

/// Execute a hvx_perm opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let rd = fld(d, b'd');

    match op {
        // ---- vassign: Vd = Vu --------------------------------------------
        Opcode::V6_vassign => {
            let vu = ctx.vread(fld(d, b'u'));
            ctx.set_v(rd, vu);
        }

        // ---- vcombine: Vdd = combine(Vu, Vv)  (v[0]=Vv, v[1]=Vu) ----------
        Opcode::V6_vcombine => {
            let vu = ctx.vread(fld(d, b'u'));
            let vv = ctx.vread(fld(d, b'v'));
            ctx.set_v(rd, vv);
            ctx.set_v(rd + 1, vu);
        }

        // ---- splat scalar to all lanes -----------------------------------
        Opcode::V6_lvsplatw => {
            let rt = ctx.r(fld(d, b't'));
            ctx.set_v(rd, [rt; 32]);
        }
        Opcode::V6_lvsplath => {
            let rt = ctx.r(fld(d, b't')) as u16;
            let w = (rt as u32) | ((rt as u32) << 16);
            ctx.set_v(rd, [w; 32]);
        }
        Opcode::V6_lvsplatb => {
            let rt = ctx.r(fld(d, b't')) as u8 as u32;
            let w = rt | (rt << 8) | (rt << 16) | (rt << 24);
            ctx.set_v(rd, [w; 32]);
        }

        // ---- vror: rotate vector right by Rt bytes (byte k <- u[(k+Rt)&127])
        Opcode::V6_vror => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let rt = ctx.r(fld(d, b't')) as usize;
            let mut out = [0u8; 128];
            for k in 0..128 {
                out[k] = vu[(k + rt) & 127];
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // ---- valign/vlalign: byte-align two vectors --------------------
        // valignb: shift = Rt & 127; out[i] = (i+shift>=128)? Vu[i+shift-128] : Vv[i+shift]
        Opcode::V6_valignb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let shift = (ctx.r(fld(d, b't')) as usize) & 127;
            ctx.set_v(rd, from_bytes(&align(&vu, &vv, shift)));
        }
        // vlalignb: shift = 128 - (Rt & 127)
        Opcode::V6_vlalignb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let shift = 128 - ((ctx.r(fld(d, b't')) as usize) & 127);
            ctx.set_v(rd, from_bytes(&align(&vu, &vv, shift)));
        }
        // valignbi: shift = #u3 (immediate)
        Opcode::V6_valignbi => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let shift = imm_u3(d) as usize;
            ctx.set_v(rd, from_bytes(&align(&vu, &vv, shift)));
        }
        // vlalignbi: shift = 128 - #u3
        Opcode::V6_vlalignbi => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let shift = 128 - imm_u3(d) as usize;
            ctx.set_v(rd, from_bytes(&align(&vu, &vv, shift)));
        }

        // ---- vshuffvdd: Vdd = vshuff(Vu,Vv,Rt) -----------------------------
        // v[0]=Vv, v[1]=Vu; for each power-of-two offset selected by Rt bits,
        // swap byte k of v[1] with byte k+offset of v[0] when !(k & offset).
        Opcode::V6_vshuffvdd => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let rt = ctx.r(fld(d, b't')) as usize;
            let mut lo = vv; // v[0]
            let mut hi = vu; // v[1]
            let mut offset = 1usize;
            while offset < 128 {
                if rt & offset != 0 {
                    for k in 0..128 {
                        if k & offset == 0 {
                            std::mem::swap(&mut hi[k], &mut lo[k + offset]);
                        }
                    }
                }
                offset <<= 1;
            }
            set_pair(ctx, rd, &lo, &hi);
        }

        // ---- per-byte single-vector shuffle (vshuffb) ----------------------
        // fVFOREACH(16,i): byte0 of half i <- Vu.ub[i]; byte1 <- Vu.ub[i+64]
        Opcode::V6_vshuffb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i * 2] = vu[i];
                out[i * 2 + 1] = vu[i + 64];
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vshuffh: fVFOREACH(32,i): half0 of word i <- Vu.uh[i]; half1 <- Vu.uh[i+32]
        Opcode::V6_vshuffh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i * 2, get_h(&vu, i));
                set_h(&mut out, i * 2 + 1, get_h(&vu, i + 32));
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // ---- per-byte single-vector deal (vdealb) --------------------------
        // fVFOREACH(16,i): Vd.ub[i] <- byte0 of Vu half i; Vd.ub[i+64] <- byte1
        Opcode::V6_vdealb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i] = vu[i * 2];
                out[i + 64] = vu[i * 2 + 1];
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vdealh: fVFOREACH(32,i): Vd.uh[i] <- half0 of Vu word i; Vd.uh[i+32] <- half1
        Opcode::V6_vdealh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i, get_h(&vu, i * 2));
                set_h(&mut out, i + 32, get_h(&vu, i * 2 + 1));
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vdealb4w: Vd.b = vdeale(Vu.b,Vv.b)
        // fVFOREACH(32,i): ub[i]=byte0(Vv.w[i]); ub[32+i]=byte2(Vv.w[i]);
        //                  ub[64+i]=byte0(Vu.w[i]); ub[96+i]=byte2(Vu.w[i])
        Opcode::V6_vdealb4w => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                let wv = get_w(&vv, i);
                let wu = get_w(&vu, i);
                out[i] = wv as u8;
                out[32 + i] = (wv >> 16) as u8;
                out[64 + i] = wu as u8;
                out[96 + i] = (wu >> 16) as u8;
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // ---- two-vector shuffle even/odd (vshuffeb / vshuffob) ------------
        // vshuffeb: fVFOREACH(16,i): half i = byte0(Vv.uh[i]) | byte0(Vu.uh[i])<<8
        Opcode::V6_vshuffeb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i * 2] = vv[i * 2]; // byte0 of Vv half i
                out[i * 2 + 1] = vu[i * 2]; // byte0 of Vu half i
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vshuffob: half i = byte1(Vv.uh[i]) | byte1(Vu.uh[i])<<8
        Opcode::V6_vshuffob => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i * 2] = vv[i * 2 + 1]; // byte1 of Vv half i
                out[i * 2 + 1] = vu[i * 2 + 1]; // byte1 of Vu half i
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vshufeh (Vd.h=vshuffe(Vu.h,Vv.h)): word i = half0(Vv.w[i]) | half0(Vu.w[i])<<16
        Opcode::V6_vshufeh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i * 2, get_h(&vv, i * 2));
                set_h(&mut out, i * 2 + 1, get_h(&vu, i * 2));
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vshufoh (Vd.h=vshuffo): word i = half1(Vv.w[i]) | half1(Vu.w[i])<<16
        Opcode::V6_vshufoh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i * 2, get_h(&vv, i * 2 + 1));
                set_h(&mut out, i * 2 + 1, get_h(&vu, i * 2 + 1));
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // ---- pack even/odd (Vd half-width from two full-width sources) ----
        // vpackeb: fVFOREACH(16,i): ub[i]=byte0(Vv.uh[i]); ub[i+64]=byte0(Vu.uh[i])
        Opcode::V6_vpackeb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i] = vv[i * 2];
                out[i + 64] = vu[i * 2];
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vpackob: ub[i]=byte1(Vv.uh[i]); ub[i+64]=byte1(Vu.uh[i])
        Opcode::V6_vpackob => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i] = vv[i * 2 + 1];
                out[i + 64] = vu[i * 2 + 1];
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vpackeh: fVFOREACH(32,i): uh[i]=half0(Vv.w[i]); uh[i+32]=half0(Vu.w[i])
        Opcode::V6_vpackeh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i, get_h(&vv, i * 2));
                set_h(&mut out, i + 32, get_h(&vu, i * 2));
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vpackoh: uh[i]=half1(Vv.w[i]); uh[i+32]=half1(Vu.w[i])
        Opcode::V6_vpackoh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i, get_h(&vv, i * 2 + 1));
                set_h(&mut out, i + 32, get_h(&vu, i * 2 + 1));
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // ---- saturating pack (full-width signed source -> half-width) ----
        // vpackhub_sat: ub[i]=satUB(Vv.h[i]); ub[i+64]=satUB(Vu.h[i])
        Opcode::V6_vpackhub_sat => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i] = sat_ub(get_h(&vv, i) as i16 as i32);
                out[i + 64] = sat_ub(get_h(&vu, i) as i16 as i32);
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vpackhb_sat: b[i]=satB(Vv.h[i]); b[i+64]=satB(Vu.h[i])
        Opcode::V6_vpackhb_sat => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..64 {
                out[i] = sat_b(get_h(&vv, i) as i16 as i32);
                out[i + 64] = sat_b(get_h(&vu, i) as i16 as i32);
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vpackwuh_sat: uh[i]=satUH(Vv.w[i]); uh[i+32]=satUH(Vu.w[i])
        Opcode::V6_vpackwuh_sat => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i, sat_uh(get_w(&vv, i) as i32));
                set_h(&mut out, i + 32, sat_uh(get_w(&vu, i) as i32));
            }
            ctx.set_v(rd, from_bytes(&out));
        }
        // vpackwh_sat: h[i]=satH(Vv.w[i]); h[i+32]=satH(Vu.w[i])
        Opcode::V6_vpackwh_sat => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let vv = to_bytes(&ctx.vread(fld(d, b'v')));
            let mut out = [0u8; 128];
            for i in 0..32 {
                set_h(&mut out, i, sat_h(get_w(&vv, i) as i32));
                set_h(&mut out, i + 32, sat_h(get_w(&vu, i) as i32));
            }
            ctx.set_v(rd, from_bytes(&out));
        }

        // ---- zero/sign extend a vector into a vector PAIR -----------------
        // vzb (Vdd.uh = vzxt(Vu.ub)): v[0].uh[i]=ZE(byte0 of half i);
        //                             v[1].uh[i]=ZE(byte1 of half i)
        Opcode::V6_vzb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..64 {
                set_h(&mut lo, i, vu[i * 2] as u16);
                set_h(&mut hi, i, vu[i * 2 + 1] as u16);
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vsb (Vdd.h = vsxt(Vu.b)): sign-extend byte0/byte1 of each half
        Opcode::V6_vsb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..64 {
                set_h(&mut lo, i, vu[i * 2] as i8 as i16 as u16);
                set_h(&mut hi, i, vu[i * 2 + 1] as i8 as i16 as u16);
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vzh (Vdd.uw = vzxt(Vu.uh)): zero-extend half0/half1 of each word
        Opcode::V6_vzh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..32 {
                set_w(&mut lo, i, get_h(&vu, i * 2) as u32);
                set_w(&mut hi, i, get_h(&vu, i * 2 + 1) as u32);
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vsh (Vdd.w = vsxt(Vu.h)): sign-extend half0/half1 of each word
        Opcode::V6_vsh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..32 {
                set_w(&mut lo, i, get_h(&vu, i * 2) as i16 as i32 as u32);
                set_w(&mut hi, i, get_h(&vu, i * 2 + 1) as i16 as i32 as u32);
            }
            set_pair(ctx, rd, &lo, &hi);
        }

        // ---- unpack a vector into a vector PAIR (interleaved widen) -------
        // vunpackub (Vdd.uh = vunpack(Vu.ub)): fVFOREACH(8,i) Vdd.uh[i]=ZE(Vu.ub[i])
        // The pair is 256 bytes => 128 uh lanes; even lanes -> v[0], odd -> v[1].
        Opcode::V6_vunpackub => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..128 {
                let val = vu[i] as u16;
                if i < 64 {
                    set_h(&mut lo, i, val);
                } else {
                    set_h(&mut hi, i - 64, val);
                }
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vunpackb (Vdd.h = vunpack(Vu.b)): sign-extend each byte
        Opcode::V6_vunpackb => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..128 {
                let val = vu[i] as i8 as i16 as u16;
                if i < 64 {
                    set_h(&mut lo, i, val);
                } else {
                    set_h(&mut hi, i - 64, val);
                }
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vunpackuh (Vdd.uw = vunpack(Vu.uh)): fVFOREACH(16,i) Vdd.uw[i]=ZE(Vu.uh[i])
        Opcode::V6_vunpackuh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..64 {
                let val = get_h(&vu, i) as u32;
                if i < 32 {
                    set_w(&mut lo, i, val);
                } else {
                    set_w(&mut hi, i - 32, val);
                }
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vunpackh (Vdd.w = vunpack(Vu.h)): sign-extend each half
        Opcode::V6_vunpackh => {
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let (mut lo, mut hi) = ([0u8; 128], [0u8; 128]);
            for i in 0..64 {
                let val = get_h(&vu, i) as i16 as i32 as u32;
                if i < 32 {
                    set_w(&mut lo, i, val);
                } else {
                    set_w(&mut hi, i - 32, val);
                }
            }
            set_pair(ctx, rd, &lo, &hi);
        }
        // vunpackob (Vxx.h |= vunpacko(Vu.ub)): Vxx.uh[i] |= ZE8_16(Vu.ub[i])<<8
        // Read-modify the existing pair, OR the high byte of each unpacked half.
        Opcode::V6_vunpackob => {
            let rx = fld(d, b'x');
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut lo = to_bytes(&ctx.vread(rx));
            let mut hi = to_bytes(&ctx.vread(rx + 1));
            for i in 0..128 {
                let add = (vu[i] as u16) << 8;
                if i < 64 {
                    let cur = get_h(&lo, i);
                    set_h(&mut lo, i, cur | add);
                } else {
                    let cur = get_h(&hi, i - 64);
                    set_h(&mut hi, i - 64, cur | add);
                }
            }
            set_pair(ctx, rx, &lo, &hi);
        }
        // vunpackoh (Vxx.w |= vunpacko(Vu.uh)): Vxx.uw[i] |= ZE16_32(Vu.uh[i])<<16
        Opcode::V6_vunpackoh => {
            let rx = fld(d, b'x');
            let vu = to_bytes(&ctx.vread(fld(d, b'u')));
            let mut lo = to_bytes(&ctx.vread(rx));
            let mut hi = to_bytes(&ctx.vread(rx + 1));
            for i in 0..64 {
                let add = (get_h(&vu, i) as u32) << 16;
                if i < 32 {
                    let cur = get_w(&lo, i);
                    set_w(&mut lo, i, cur | add);
                } else {
                    let cur = get_w(&hi, i - 32);
                    set_w(&mut hi, i - 32, cur | add);
                }
            }
            set_pair(ctx, rx, &lo, &hi);
        }

        _ => return false,
    }
    true
}

// ---- helpers ---------------------------------------------------------------

/// Byte-align: out[i] = (i+shift >= 128) ? Vu[i+shift-128] : Vv[i+shift].
#[inline]
fn align(vu: &Bytes, vv: &Bytes, shift: usize) -> Bytes {
    let mut out = [0u8; 128];
    for i in 0..128 {
        out[i] = if i + shift >= 128 {
            vu[i + shift - 128]
        } else {
            vv[i + shift]
        };
    }
    out
}

#[inline]
fn set_w(b: &mut Bytes, i: usize, val: u32) {
    b[i * 4..i * 4 + 4].copy_from_slice(&val.to_le_bytes());
}

/// Read the 3-bit unsigned align immediate (field letter `i`).
#[inline]
fn imm_u3(d: &DecodedOp) -> u32 {
    d.field(b'i').map(|f| f.value).unwrap_or(0) & 0x7
}
