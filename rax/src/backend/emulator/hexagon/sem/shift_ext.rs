//! (shift_ext) Hexagon shift / bit / vector-shift gap-fill — direct
//! opcode-dispatch handlers for the `S2_*` / `S4_*` / `S5_*` / `S6_*` forms that
//! the base `shift` class does not cover: per-lane vector shifts (halfword and
//! word, immediate and bidirectional register), rounded pair shift, vector
//! saturate/pack, sign/zero extend, truncate/pack, shuffles, interleave /
//! deinterleave, vector align, conditional negate / complex rotate, table
//! index, the `S4_*` compound (add/sub/and/or with embedded shift), `clb`
//! add/norm, and the cross add/sub forms.
//!
//! Semantics taken verbatim from the Hexagon V68 spec (gen_semantics expansion
//! of `imported/shift.idef` + `imported/macros.def`).  Verified against the
//! `qemu-hexagon` reference oracle (`tests/hexagon_diff.rs`).  See `sem/alu.rs`
//! and `sem/shift.rs` for the established pattern.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

// ---- lane accessors (mirror fGET*/fSET* macros) ---------------------------

/// `fGETHALF(N,SRC)`: signed 16-bit lane `N` of a 64-bit value.
#[inline]
fn get_half(src: u64, n: u32) -> i64 {
    ((src >> (n * 16)) & 0xffff) as u16 as i16 as i64
}
/// `fGETUHALF(N,SRC)`: unsigned 16-bit lane `N`.
#[inline]
fn get_uhalf(src: u64, n: u32) -> u64 {
    (src >> (n * 16)) & 0xffff
}
/// `fGETWORD(N,SRC)`: signed 32-bit lane `N`.
#[inline]
fn get_word(src: u64, n: u32) -> i64 {
    ((src >> (n * 32)) & 0xffff_ffff) as u32 as i32 as i64
}
/// `fGETUWORD(N,SRC)`: unsigned 32-bit lane `N`.
#[inline]
fn get_uword(src: u64, n: u32) -> u64 {
    (src >> (n * 32)) & 0xffff_ffff
}
/// `fGETBYTE(N,SRC)`: signed 8-bit lane `N`.
#[inline]
fn get_byte(src: u64, n: u32) -> i64 {
    ((src >> (n * 8)) & 0xff) as u8 as i8 as i64
}
/// `fGETUBYTE(N,SRC)`: unsigned 8-bit lane `N`.
#[inline]
fn get_ubyte(src: u64, n: u32) -> u64 {
    (src >> (n * 8)) & 0xff
}
/// `fSETHALF(N,DST,VAL)`: deposit the low 16 bits of `val` into lane `N`.
#[inline]
fn set_half(dst: u64, n: u32, val: u64) -> u64 {
    let sh = n * 16;
    (dst & !(0xffffu64 << sh)) | ((val & 0xffff) << sh)
}
/// `fSETWORD(N,DST,VAL)`: deposit the low 32 bits of `val` into lane `N`.
#[inline]
fn set_word(dst: u64, n: u32, val: u64) -> u64 {
    let sh = n * 32;
    (dst & !(0xffff_ffffu64 << sh)) | ((val & 0xffff_ffff) << sh)
}
/// `fSETBYTE(N,DST,VAL)`: deposit the low 8 bits of `val` into lane `N`.
#[inline]
fn set_byte(dst: u64, n: u32, val: u64) -> u64 {
    let sh = n * 8;
    (dst & !(0xffu64 << sh)) | ((val & 0xff) << sh)
}

// ---- bidirectional per-lane shifts ----------------------------------------
// `shamt = fSXTN(7,32,RtV)`: sign-extend the low 7 bits of Rt; a negative
// amount reverses the shift direction.  `fBIDIR_*SHIFT*` first widens the lane
// per its REGSTYPE cast, then evaluates in 64-bit.

/// `fSXTN(7,32,RtV)`: sign-extend the low 7 bits of Rt to a signed amount.
#[inline]
fn sxt7(rt: u32) -> i32 {
    ((rt as i32) << 25) >> 25
}

/// `fBIDIR_ASHIFTL(src,shamt,W_8)`: signed source, arithmetic left/right.
#[inline]
fn bidir_ashiftl(src: i64, shamt: i32) -> u64 {
    (if shamt < 0 {
        (src >> ((-shamt) - 1)) >> 1
    } else {
        ((src as u64) << shamt) as i64
    }) as u64
}
/// `fBIDIR_ASHIFTR(src,shamt,W_8)`.
#[inline]
fn bidir_ashiftr(src: i64, shamt: i32) -> u64 {
    (if shamt < 0 {
        ((src as u64) << ((-shamt) - 1) << 1) as i64
    } else {
        src >> shamt
    }) as u64
}
/// `fBIDIR_LSHIFTL(src,shamt,W_8)`: unsigned source.
#[inline]
fn bidir_lshiftl(src: u64, shamt: i32) -> u64 {
    if shamt < 0 {
        (src >> ((-shamt) - 1)) >> 1
    } else {
        src << shamt
    }
}
/// `fBIDIR_LSHIFTR(src,shamt,W_8)`.
#[inline]
fn bidir_lshiftr(src: u64, shamt: i32) -> u64 {
    if shamt < 0 {
        (src << ((-shamt) - 1)) << 1
    } else {
        src >> shamt
    }
}

/// `fASHIFTL(SRC,SHAMT,8_8)`: 64-bit signed left shift, `>=64 -> 0`.
#[inline]
fn ashiftl_64(src: u64, shamt: u32) -> u64 {
    if shamt >= 64 {
        0
    } else {
        ((src as i64) << shamt) as u64
    }
}
/// `fASHIFTR(SRC,SHAMT,8_8)`: 64-bit arithmetic right shift.
#[inline]
fn ashiftr_64(src: u64, shamt: u32) -> u64 {
    ((src as i64) >> shamt) as u64
}
/// `fLSHIFTR(SRC,SHAMT,8_8)`: 64-bit logical right shift, `>=64 -> 0`.
#[inline]
fn lshiftr_64(src: u64, shamt: u32) -> u64 {
    if shamt >= 64 { 0 } else { src >> shamt }
}

// ---- count leading sign bits (clb) ----------------------------------------

/// `count_leading_ones` over the top `bits` of `val`.
#[inline]
fn cl1(val: u64, bits: u32) -> u32 {
    let mut n = 0u32;
    for i in (0..bits).rev() {
        if (val >> i) & 1 == 1 {
            n += 1;
        } else {
            break;
        }
    }
    n
}

// ---- bit interleave / deinterleave (qemu-hexagon `interleave`) ------------

/// `interleave(odd,even)`: even bits of the result come from `even`, odd bits
/// from `odd` (matches qemu `target/hexagon/arch.c`).
#[inline]
fn interleave(odd: u32, even: u32) -> u64 {
    let mut r = 0u64;
    for i in 0..32 {
        r |= (((even >> i) & 1) as u64) << (2 * i);
        r |= (((odd >> i) & 1) as u64) << (2 * i + 1);
    }
    r
}
/// `deinterleave(mixed)`: gather even bits into the low word and odd bits into
/// the high word.
#[inline]
fn deinterleave(mixed: u64) -> u64 {
    let mut even = 0u32;
    let mut odd = 0u32;
    for i in 0..32 {
        even |= (((mixed >> (2 * i)) & 1) as u32) << i;
        odd |= (((mixed >> (2 * i + 1)) & 1) as u32) << i;
    }
    ((odd as u64) << 32) | (even as u64)
}

/// Execute a shift_ext opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let rd = fld(d, b'd');
    let rx = fld(d, b'x');
    let rs = || fld(d, b's');
    let rt = || fld(d, b't');
    let ui = || fimm_u(d, b'i', ctx.immext);

    match op {
        // ---- per-halfword immediate vector shifts (4 lanes) ----
        Opcode::S2_asl_i_vh => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, (get_half(src, i) << sh) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_asr_i_vh => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, (get_half(src, i) >> sh) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_lsr_i_vh => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, get_uhalf(src, i) >> sh);
            }
            ctx.set_rp(rd, r);
        }

        // ---- per-word immediate vector shifts (2 lanes) ----
        Opcode::S2_asl_i_vw => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, (get_word(src, i) << sh) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_asr_i_vw => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, (get_word(src, i) >> sh) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_lsr_i_vw => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, get_uword(src, i) >> sh);
            }
            ctx.set_rp(rd, r);
        }

        // ---- per-halfword bidirectional register vector shifts ----
        Opcode::S2_asl_r_vh => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, bidir_ashiftl(get_half(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_asr_r_vh => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, bidir_ashiftr(get_half(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_lsr_r_vh => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, bidir_lshiftr(get_uhalf(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_lsl_r_vh => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, bidir_lshiftl(get_uhalf(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }

        // ---- per-word bidirectional register vector shifts ----
        Opcode::S2_asl_r_vw => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, bidir_ashiftl(get_word(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_asr_r_vw => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, bidir_ashiftr(get_word(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_lsr_r_vw => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, bidir_lshiftr(get_uword(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_lsl_r_vw => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, bidir_lshiftl(get_uword(src, i), sh));
            }
            ctx.set_rp(rd, r);
        }

        // ---- vasrw to single word, halfword-truncated ----
        // Rd32=vasrw(Rss,Rt): for each of 2 words, bidir-asr (64-bit), keep
        // its low halfword, deposit into halfword `i` of Rd.
        Opcode::S2_asr_r_svw_trun => {
            let src = ctx.rp(rs());
            let sh = sxt7(ctx.r(rt()));
            let mut r = 0u32;
            for i in 0..2 {
                let shifted = bidir_ashiftr(get_word(src, i), sh);
                let half0 = get_half(shifted, 0) as u64;
                r = set_half(r as u64, i, half0) as u32;
            }
            ctx.set_r(rd, r);
        }

        // ---- rounded arithmetic right shift of a pair ----
        Opcode::S2_asr_i_p_rnd => {
            let src = ctx.rp(rs());
            let sh = ui();
            let tmp = ashiftr_64(src, sh);
            let rnd = tmp & 1;
            let v = ashiftr_64(tmp, 1).wrapping_add(rnd);
            ctx.set_rp(rd, v);
        }

        // ---- S2_lsl_r_r: Rd = bidir-lshift-left of Rs by sxt7(Rt) ----
        Opcode::S2_lsl_r_r => {
            let sh = sxt7(ctx.r(rt()));
            let v = bidir_lshiftl(ctx.r(rs()) as u64, sh) as u32;
            ctx.set_r(rd, v);
        }

        // ---- S2_lfsp: linear feedback shift (pair) ----
        Opcode::S2_lfsp => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let parity = ((rss & rtt).count_ones() & 1) as u64;
            let v = (rss >> 1) | (parity << 63);
            ctx.set_rp(rd, v);
        }

        // ---- vector saturate + pack to bytes/halves ----
        Opcode::S2_vsathb => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..4 {
                let v = ctx.sat_n(get_half(src, i), 8) as u64;
                r = set_byte(r as u64, i, v) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S2_vsathub => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..4 {
                let v = ctx.satu_n(get_half(src, i), 8) as u64;
                r = set_byte(r as u64, i, v) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S2_vsatwh => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..2 {
                let v = ctx.sat_n(get_word(src, i), 16) as u64;
                r = set_half(r as u64, i, v) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S2_vsatwuh => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..2 {
                let v = ctx.satu_n(get_word(src, i), 16) as u64;
                r = set_half(r as u64, i, v) as u32;
            }
            ctx.set_r(rd, r);
        }

        // ---- vector saturate, no pack (lanes stay full width) ----
        Opcode::S2_vsathb_nopack => {
            let src = ctx.rp(rs());
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.sat_n(get_half(src, i), 8) as u64;
                r = set_half(r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vsathub_nopack => {
            let src = ctx.rp(rs());
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.satu_n(get_half(src, i), 8) as u64;
                r = set_half(r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vsatwh_nopack => {
            let src = ctx.rp(rs());
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(get_word(src, i), 16) as u64;
                r = set_word(r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vsatwuh_nopack => {
            let src = ctx.rp(rs());
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.satu_n(get_word(src, i), 16) as u64;
                r = set_word(r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- scalar (single-register) saturate + pack of 2 halves ----
        Opcode::S2_svsathb => {
            let src = ctx.r(rs());
            let b0 = ctx.sat_n(get_half(src as u64, 0), 8) as u64;
            let b1 = ctx.sat_n(get_half(src as u64, 1), 8) as u64;
            let mut r = 0u64;
            r = set_byte(r, 0, b0);
            r = set_byte(r, 1, b1);
            ctx.set_r(rd, r as u32);
        }
        Opcode::S2_svsathub => {
            let src = ctx.r(rs());
            let b0 = ctx.satu_n(get_half(src as u64, 0), 8) as u64;
            let b1 = ctx.satu_n(get_half(src as u64, 1), 8) as u64;
            let mut r = 0u64;
            r = set_byte(r, 0, b0);
            r = set_byte(r, 1, b1);
            ctx.set_r(rd, r as u32);
        }

        // ---- asr halves then saturate to unsigned byte ----
        Opcode::S5_asrhub_sat => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u32;
            for i in 0..4 {
                let v = ctx.satu_n(get_half(src, i) >> sh, 8) as u64;
                r = set_byte(r as u64, i, v) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S5_asrhub_rnd_sat => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u32;
            for i in 0..4 {
                let v = ctx.satu_n(((get_half(src, i) >> sh) + 1) >> 1, 8) as u64;
                r = set_byte(r as u64, i, v) as u32;
            }
            ctx.set_r(rd, r);
        }

        // ---- asr halves with rounding (no saturate) ----
        Opcode::S5_vasrhrnd => {
            let src = ctx.rp(rs());
            let sh = ui();
            let mut r = 0u64;
            for i in 0..4 {
                let v = (((get_half(src, i) >> sh) + 1) >> 1) as u64;
                r = set_half(r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- round + pack words to halves ----
        Opcode::S2_vrndpackwh => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..2 {
                let w = get_word(src, i).wrapping_add(0x0_8000);
                let half1 = get_half(w as u64, 1) as u64;
                r = set_half(r as u64, i, half1) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S2_vrndpackwhs => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..2 {
                let w = ctx.sat_n(get_word(src, i).wrapping_add(0x0_8000), 32);
                let half1 = get_half(w as u64, 1) as u64;
                r = set_half(r as u64, i, half1) as u32;
            }
            ctx.set_r(rd, r);
        }

        // ---- sign / zero extend ----
        Opcode::S2_vsxtbh => {
            let src = ctx.r(rs()) as u64;
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, get_byte(src, i) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vzxtbh => {
            let src = ctx.r(rs()) as u64;
            let mut r = 0u64;
            for i in 0..4 {
                r = set_half(r, i, get_ubyte(src, i));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vsxthw => {
            let src = ctx.r(rs()) as u64;
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, get_half(src, i) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vzxthw => {
            let src = ctx.r(rs()) as u64;
            let mut r = 0u64;
            for i in 0..2 {
                r = set_word(r, i, get_uhalf(src, i));
            }
            ctx.set_rp(rd, r);
        }

        // ---- truncate / pack ----
        Opcode::S2_vtrunehb => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..4 {
                r = set_byte(r as u64, i, get_byte(src, i * 2) as u64) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S2_vtrunohb => {
            let src = ctx.rp(rs());
            let mut r = 0u32;
            for i in 0..4 {
                r = set_byte(r as u64, i, get_byte(src, i * 2 + 1) as u64) as u32;
            }
            ctx.set_r(rd, r);
        }
        Opcode::S2_vtrunewh => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            r = set_half(r, 0, get_half(rtt, 0) as u64);
            r = set_half(r, 1, get_half(rtt, 2) as u64);
            r = set_half(r, 2, get_half(rss, 0) as u64);
            r = set_half(r, 3, get_half(rss, 2) as u64);
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vtrunowh => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            r = set_half(r, 0, get_half(rtt, 1) as u64);
            r = set_half(r, 1, get_half(rtt, 3) as u64);
            r = set_half(r, 2, get_half(rss, 1) as u64);
            r = set_half(r, 3, get_half(rss, 3) as u64);
            ctx.set_rp(rd, r);
        }
        Opcode::S6_vtrunehb_ppp => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            for i in 0..4 {
                r = set_byte(r, i, get_byte(rtt, i * 2) as u64);
                r = set_byte(r, i + 4, get_byte(rss, i * 2) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S6_vtrunohb_ppp => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            for i in 0..4 {
                r = set_byte(r, i, get_byte(rtt, i * 2 + 1) as u64);
                r = set_byte(r, i + 4, get_byte(rss, i * 2 + 1) as u64);
            }
            ctx.set_rp(rd, r);
        }

        // ---- shuffles ----
        Opcode::S2_shuffeb => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            for i in 0..4 {
                r = set_byte(r, i * 2, get_byte(rtt, i * 2) as u64);
                r = set_byte(r, i * 2 + 1, get_byte(rss, i * 2) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_shuffob => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            for i in 0..4 {
                r = set_byte(r, i * 2, get_byte(rss, i * 2 + 1) as u64);
                r = set_byte(r, i * 2 + 1, get_byte(rtt, i * 2 + 1) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_shuffeh => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            for i in 0..2 {
                r = set_half(r, i * 2, get_half(rtt, i * 2) as u64);
                r = set_half(r, i * 2 + 1, get_half(rss, i * 2) as u64);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_shuffoh => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let mut r = 0u64;
            for i in 0..2 {
                r = set_half(r, i * 2, get_half(rss, i * 2 + 1) as u64);
                r = set_half(r, i * 2 + 1, get_half(rtt, i * 2 + 1) as u64);
            }
            ctx.set_rp(rd, r);
        }

        // ---- interleave / deinterleave ----
        Opcode::S2_interleave => {
            let src = ctx.rp(rs());
            let odd = get_uword(src, 1) as u32;
            let even = get_uword(src, 0) as u32;
            ctx.set_rp(rd, interleave(odd, even));
        }
        Opcode::S2_deinterleave => {
            let src = ctx.rp(rs());
            ctx.set_rp(rd, deinterleave(src));
        }

        // ---- vector byte align ----
        // Rdd = (Rss >> shift*8) | (Rtt << (8-shift)*8).
        Opcode::S2_valignib => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let sh = ui();
            let v = lshiftr_64(rss, sh * 8) | ashiftl_64(rtt, (8 - sh) * 8);
            ctx.set_rp(rd, v);
        }
        Opcode::S2_valignrb => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let pu = (ctx.p(fld(d, b'u')) & 0x7) as u32;
            let v = lshiftr_64(rss, pu * 8) | ashiftl_64(rtt, (8 - pu) * 8);
            ctx.set_rp(rd, v);
        }

        // ---- conditional negate / complex rotate ----
        Opcode::S2_vcnegh => {
            let rss = ctx.rp(rs());
            let rtv = ctx.r(rt());
            let mut r = 0u64;
            for i in 0..4 {
                let v = if (rtv >> i) & 1 == 1 {
                    ctx.sat_n(-get_half(rss, i), 16) as u64
                } else {
                    get_half(rss, i) as u64
                };
                r = set_half(r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::S2_vrcnegh => {
            let rss = ctx.rp(rs());
            let rtv = ctx.r(rt());
            let mut acc = ctx.rp(rx);
            for i in 0..4 {
                let term = if (rtv >> i) & 1 == 1 {
                    (-get_half(rss, i)) as u64
                } else {
                    get_half(rss, i) as u64
                };
                acc = acc.wrapping_add(term);
            }
            ctx.set_rp(rx, acc);
        }
        Opcode::S2_vcrotate => {
            let rss = ctx.rp(rs());
            let rtv = ctx.r(rt());
            let mut r = 0u64;
            // low pair, control = Rt[1:0]
            let tmp = rtv & 0x3;
            let (h0, h1) = match tmp {
                0 => (get_half(rss, 0), get_half(rss, 1)),
                1 => (get_half(rss, 1), ctx.sat_n(-get_half(rss, 0), 16)),
                2 => (ctx.sat_n(-get_half(rss, 1), 16), get_half(rss, 0)),
                _ => (
                    ctx.sat_n(-get_half(rss, 0), 16),
                    ctx.sat_n(-get_half(rss, 1), 16),
                ),
            };
            r = set_half(r, 0, h0 as u64);
            r = set_half(r, 1, h1 as u64);
            // high pair, control = Rt[3:2]
            let tmp = (rtv >> 2) & 0x3;
            let (h2, h3) = match tmp {
                0 => (get_half(rss, 2), get_half(rss, 3)),
                1 => (get_half(rss, 3), ctx.sat_n(-get_half(rss, 2), 16)),
                2 => (ctx.sat_n(-get_half(rss, 3), 16), get_half(rss, 2)),
                _ => (
                    ctx.sat_n(-get_half(rss, 2), 16),
                    ctx.sat_n(-get_half(rss, 3), 16),
                ),
            };
            r = set_half(r, 2, h2 as u64);
            r = set_half(r, 3, h3 as u64);
            ctx.set_rp(rd, r);
        }

        // ---- complex byte rotate-accumulate ----
        Opcode::S4_vrcrotate => {
            let rss = ctx.rp(rs());
            let rtv = ctx.r(rt());
            let (sumr, sumi) = vrcrotate(rss, rtv, ui());
            let mut r = 0u64;
            r = set_word(r, 0, sumr as u32 as u64);
            r = set_word(r, 1, sumi as u32 as u64);
            ctx.set_rp(rd, r);
        }
        Opcode::S4_vrcrotate_acc => {
            let rss = ctx.rp(rs());
            let rtv = ctx.r(rt());
            let acc = ctx.rp(rx);
            let (sumr, sumi) = vrcrotate(rss, rtv, ui());
            let w0 = (get_word(acc, 0) as i32).wrapping_add(sumr);
            let w1 = (get_word(acc, 1) as i32).wrapping_add(sumi);
            let mut r = 0u64;
            r = set_word(r, 0, w0 as u32 as u64);
            r = set_word(r, 1, w1 as u32 as u64);
            ctx.set_rp(rx, r);
        }

        // ---- table index (extract a field, deposit at width*idx) ----
        Opcode::S2_tableidxb => tableidx(ctx, d, rx, 0),
        Opcode::S2_tableidxh => tableidx(ctx, d, rx, 1),
        Opcode::S2_tableidxw => tableidx(ctx, d, rx, 2),
        Opcode::S2_tableidxd => tableidx(ctx, d, rx, 3),

        // ---- S4 compound add/sub with immediate ----
        Opcode::S4_addaddi => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = ctx
                .r(rs())
                .wrapping_add(ctx.r(fld(d, b'u')))
                .wrapping_add(imm);
            ctx.set_r(rd, v);
        }
        Opcode::S4_subaddi => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = ctx
                .r(rs())
                .wrapping_sub(ctx.r(fld(d, b'u')))
                .wrapping_add(imm);
            ctx.set_r(rd, v);
        }

        // ---- S4 compound: Rx = imm OP (Rx SHIFT shamt) ----
        // `i` = u8 immediate (extendable), `I` = U5 shift amount.
        Opcode::S4_addi_asl_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm.wrapping_add(ctx.r(rx) << sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_addi_lsr_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm.wrapping_add(ctx.r(rx) >> sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_subi_asl_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm.wrapping_sub(ctx.r(rx) << sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_subi_lsr_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm.wrapping_sub(ctx.r(rx) >> sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_andi_asl_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm & (ctx.r(rx) << sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_andi_lsr_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm & (ctx.r(rx) >> sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_ori_asl_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm | (ctx.r(rx) << sh);
            ctx.set_r(rx, v);
        }
        Opcode::S4_ori_lsr_ri => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let sh = fimm_u(d, b'I', None);
            let v = imm | (ctx.r(rx) >> sh);
            ctx.set_r(rx, v);
        }

        // ---- S4 or-and / or-or compound ----
        Opcode::S4_or_andi => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = ctx.r(rx) | (ctx.r(rs()) & imm);
            ctx.set_r(rx, v);
        }
        Opcode::S4_or_andix => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = ctx.r(fld(d, b'u')) | (ctx.r(rx) & imm);
            ctx.set_r(rx, v);
        }
        Opcode::S4_or_ori => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = ctx.r(rx) | (ctx.r(rs()) | imm);
            ctx.set_r(rx, v);
        }

        // ---- count-leading-bits add / norm ----
        Opcode::S4_clbaddi => {
            let s = ctx.r(rs());
            let clb = cl1(s as u64, 32).max(cl1(!s as u64, 32)) as i32;
            let imm = fimm_s(d, b'i', ctx.immext);
            ctx.set_r(rd, clb.wrapping_add(imm) as u32);
        }
        Opcode::S4_clbpaddi => {
            let s = ctx.rp(rs());
            let clb = cl1(s, 64).max(cl1(!s, 64)) as i32;
            let imm = fimm_s(d, b'i', ctx.immext);
            ctx.set_r(rd, clb.wrapping_add(imm) as u32);
        }
        Opcode::S4_clbpnorm => {
            let s = ctx.rp(rs());
            let v = if s == 0 {
                0
            } else {
                cl1(s, 64).max(cl1(!s, 64)) - 1
            };
            ctx.set_r(rd, v);
        }

        // ---- cross add/sub (halfword, saturating) ----
        Opcode::S4_vxaddsubh => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let h0 = ctx.sat_n(get_half(rss, 0) + get_half(rtt, 1), 16) as u64;
            let h1 = ctx.sat_n(get_half(rss, 1) - get_half(rtt, 0), 16) as u64;
            let h2 = ctx.sat_n(get_half(rss, 2) + get_half(rtt, 3), 16) as u64;
            let h3 = ctx.sat_n(get_half(rss, 3) - get_half(rtt, 2), 16) as u64;
            let mut r = 0u64;
            r = set_half(r, 0, h0);
            r = set_half(r, 1, h1);
            r = set_half(r, 2, h2);
            r = set_half(r, 3, h3);
            ctx.set_rp(rd, r);
        }
        Opcode::S4_vxsubaddh => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let h0 = ctx.sat_n(get_half(rss, 0) - get_half(rtt, 1), 16) as u64;
            let h1 = ctx.sat_n(get_half(rss, 1) + get_half(rtt, 0), 16) as u64;
            let h2 = ctx.sat_n(get_half(rss, 2) - get_half(rtt, 3), 16) as u64;
            let h3 = ctx.sat_n(get_half(rss, 3) + get_half(rtt, 2), 16) as u64;
            let mut r = 0u64;
            r = set_half(r, 0, h0);
            r = set_half(r, 1, h1);
            r = set_half(r, 2, h2);
            r = set_half(r, 3, h3);
            ctx.set_rp(rd, r);
        }
        Opcode::S4_vxaddsubhr => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let h0 = ctx.sat_n((get_half(rss, 0) + get_half(rtt, 1) + 1) >> 1, 16) as u64;
            let h1 = ctx.sat_n((get_half(rss, 1) - get_half(rtt, 0) + 1) >> 1, 16) as u64;
            let h2 = ctx.sat_n((get_half(rss, 2) + get_half(rtt, 3) + 1) >> 1, 16) as u64;
            let h3 = ctx.sat_n((get_half(rss, 3) - get_half(rtt, 2) + 1) >> 1, 16) as u64;
            let mut r = 0u64;
            r = set_half(r, 0, h0);
            r = set_half(r, 1, h1);
            r = set_half(r, 2, h2);
            r = set_half(r, 3, h3);
            ctx.set_rp(rd, r);
        }
        Opcode::S4_vxsubaddhr => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let h0 = ctx.sat_n((get_half(rss, 0) - get_half(rtt, 1) + 1) >> 1, 16) as u64;
            let h1 = ctx.sat_n((get_half(rss, 1) + get_half(rtt, 0) + 1) >> 1, 16) as u64;
            let h2 = ctx.sat_n((get_half(rss, 2) - get_half(rtt, 3) + 1) >> 1, 16) as u64;
            let h3 = ctx.sat_n((get_half(rss, 3) + get_half(rtt, 2) + 1) >> 1, 16) as u64;
            let mut r = 0u64;
            r = set_half(r, 0, h0);
            r = set_half(r, 1, h1);
            r = set_half(r, 2, h2);
            r = set_half(r, 3, h3);
            ctx.set_rp(rd, r);
        }

        // ---- cross add/sub (word, saturating to 32 bits) ----
        Opcode::S4_vxaddsubw => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let w0 = ctx.sat_n(get_word(rss, 0) + get_word(rtt, 1), 32) as u64;
            let w1 = ctx.sat_n(get_word(rss, 1) - get_word(rtt, 0), 32) as u64;
            let mut r = 0u64;
            r = set_word(r, 0, w0);
            r = set_word(r, 1, w1);
            ctx.set_rp(rd, r);
        }
        Opcode::S4_vxsubaddw => {
            let rss = ctx.rp(rs());
            let rtt = ctx.rp(rt());
            let w0 = ctx.sat_n(get_word(rss, 0) - get_word(rtt, 1), 32) as u64;
            let w1 = ctx.sat_n(get_word(rss, 1) + get_word(rtt, 0), 32) as u64;
            let mut r = 0u64;
            r = set_word(r, 0, w0);
            r = set_word(r, 1, w1);
            ctx.set_rp(rd, r);
        }

        _ => return false,
    }
    true
}

/// `S4_vrcrotate`: 4 complex byte pairs (real,imag) accumulated with a
/// per-pair rotation selected by the control byte `Rt[uiV]`.
#[inline]
fn vrcrotate(rss: u64, rtv: u32, ui: u32) -> (i32, i32) {
    let mut sumr: i32 = 0;
    let mut sumi: i32 = 0;
    let mut control = get_ubyte(rtv as u64, ui) as u32;
    let mut i = 0u32;
    while i < 8 {
        let tmpr = get_byte(rss, i) as i32;
        let tmpi = get_byte(rss, i + 1) as i32;
        match control & 3 {
            0 => {
                sumr += tmpr;
                sumi += tmpi;
            }
            1 => {
                sumr += tmpi;
                sumi -= tmpr;
            }
            2 => {
                sumr -= tmpi;
                sumi += tmpr;
            }
            _ => {
                sumr -= tmpr;
                sumi -= tmpi;
            }
        }
        control >>= 2;
        i += 2;
    }
    (sumr, sumi)
}

/// `S2_tableidx{b,h,w,d}`: extract a `width`-bit field from `Rs` at a
/// bidirectional offset, then insert it into `Rx` at bit offset `n` (the
/// element-size log2: 0=b, 1=h, 2=w, 3=d) per `fINSERT_BITS(.,width,n,.)`.
#[inline]
fn tableidx(ctx: &mut SemCtx, d: &DecodedOp, rx: u8, n: u32) {
    let width = fimm_u(d, b'i', None); // u4
    let offset = fimm_s(d, b'I', None).wrapping_add(n as i32); // S6 + N
    let rsv = ctx.r(fld(d, b's')) as u64;
    // fEXTRACTU_BIDIR(RsV, width, offset) = fZXTN(width, 32, bidir_lshiftr(RsV, offset, 4_8))
    let shifted = bidir_lshiftr(rsv, offset);
    let field = if width == 0 {
        0u32
    } else if width >= 32 {
        shifted as u32
    } else {
        (shifted & ((1u64 << width) - 1)) as u32
    };
    // fINSERT_BITS(RxV, width, N, field): deposit `width` bits of `field` at
    // bit N.  width==0 yields a zero mask, leaving Rx unchanged.
    let mask: u32 = if width == 0 {
        0
    } else if width >= 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    let rxv = (ctx.r(rx) & !(mask << n)) | ((field & mask) << n);
    ctx.set_r(rx, rxv);
}
