//! Shift / rotate instructions (`S2_*`, `S4_*`, `S5_*`, `S6_*`): immediate and
//! register-amount logical/arithmetic shifts (single-word and pair),
//! shift-accumulate (`+ - & | ^`), saturating/rounding shifts, the bidirectional
//! register-amount shifts, `S4_lsli`, `S2_addasl_rrri`, and `S6_rol_*` rotates.
//! Semantics taken verbatim from the Hexagon V68 spec (gen_semantics expansion
//! of `imported/shift.idef` + `imported/macros.def`).

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

// ---- core shift primitives (mirror the spec f-macros) ---------------------

/// `fASHIFTL(SRC,SHAMT,4_4)`: arithmetic left shift, 32-bit signed source.
/// (SHAMT is a u5 here, always < 64, so the `>=64 -> 0` guard never triggers.)
#[inline]
fn ashiftl_32(src: u32, shamt: u32) -> u32 {
    ((src as i32) << shamt) as u32
}
/// `fASHIFTR(SRC,SHAMT,4_4)`: arithmetic right shift, 32-bit signed source.
#[inline]
fn ashiftr_32(src: u32, shamt: u32) -> u32 {
    ((src as i32) >> shamt) as u32
}
/// `fLSHIFTR(SRC,SHAMT,4_4)`: logical right shift, 32-bit unsigned source.
#[inline]
fn lshiftr_32(src: u32, shamt: u32) -> u32 {
    src >> shamt
}
/// `fROTL(SRC,SHAMT,4_4)`: rotate-left of a 32-bit value.
#[inline]
fn rotl_32(src: u32, shamt: u32) -> u32 {
    if shamt == 0 {
        src
    } else {
        (src << shamt) | (src >> (32 - shamt))
    }
}

/// `fASHIFTL(SRC,SHAMT,8_8)`: arithmetic left shift, 64-bit signed source.
#[inline]
fn ashiftl_64(src: u64, shamt: u32) -> u64 {
    ((src as i64) << shamt) as u64
}
/// `fASHIFTR(SRC,SHAMT,8_8)`: arithmetic right shift, 64-bit signed source.
#[inline]
fn ashiftr_64(src: u64, shamt: u32) -> u64 {
    ((src as i64) >> shamt) as u64
}
/// `fLSHIFTR(SRC,SHAMT,8_8)`: logical right shift, 64-bit unsigned source.
#[inline]
fn lshiftr_64(src: u64, shamt: u32) -> u64 {
    src >> shamt
}
/// `fROTL(SRC,SHAMT,8_8)`: rotate-left of a 64-bit value.
#[inline]
fn rotl_64(src: u64, shamt: u32) -> u64 {
    if shamt == 0 {
        src
    } else {
        (src << shamt) | (src >> (64 - shamt))
    }
}

// ---- bidirectional register-amount shifts ---------------------------------
// `shamt = fSXTN(7,32,RtV)` -> sign-extend the low 7 bits of Rt to i32; a
// negative amount reverses the shift direction.  Evaluate in i64/u64.

/// `fBIDIR_ASHIFTL(SRC,shamt,4_8)`: signed 32-bit source widened to i64.
#[inline]
fn bidir_ashiftl_4_8(src: u32, shamt: i32) -> u64 {
    let s = src as i32 as i64;
    (if shamt < 0 {
        (s >> ((-shamt) - 1)) >> 1
    } else {
        s << shamt
    }) as u64
}
/// `fBIDIR_ASHIFTR(SRC,shamt,4_8)`.
#[inline]
fn bidir_ashiftr_4_8(src: u32, shamt: i32) -> u64 {
    let s = src as i32 as i64;
    (if shamt < 0 {
        (s << ((-shamt) - 1)) << 1
    } else {
        s >> shamt
    }) as u64
}
/// `fBIDIR_LSHIFTL(SRC,shamt,4_8)`: unsigned 32-bit source widened to u64.
#[inline]
fn bidir_lshiftl_4_8(src: u32, shamt: i32) -> u64 {
    let u = src as u64;
    if shamt < 0 {
        (u >> ((-shamt) - 1)) >> 1
    } else {
        u << shamt
    }
}
/// `fBIDIR_LSHIFTR(SRC,shamt,4_8)`.
#[inline]
fn bidir_lshiftr_4_8(src: u32, shamt: i32) -> u64 {
    let u = src as u64;
    if shamt < 0 {
        (u << ((-shamt) - 1)) << 1
    } else {
        u >> shamt
    }
}

/// `fBIDIR_ASHIFTL(SRC,shamt,8_8)`.
#[inline]
fn bidir_ashiftl_8_8(src: u64, shamt: i32) -> u64 {
    let s = src as i64;
    (if shamt < 0 {
        (s >> ((-shamt) - 1)) >> 1
    } else {
        s << shamt
    }) as u64
}
/// `fBIDIR_ASHIFTR(SRC,shamt,8_8)`.
#[inline]
fn bidir_ashiftr_8_8(src: u64, shamt: i32) -> u64 {
    let s = src as i64;
    (if shamt < 0 {
        (s << ((-shamt) - 1)) << 1
    } else {
        s >> shamt
    }) as u64
}
/// `fBIDIR_LSHIFTL(SRC,shamt,8_8)`.
#[inline]
fn bidir_lshiftl_8_8(src: u64, shamt: i32) -> u64 {
    if shamt < 0 {
        (src >> ((-shamt) - 1)) >> 1
    } else {
        src << shamt
    }
}
/// `fBIDIR_LSHIFTR(SRC,shamt,8_8)`.
#[inline]
fn bidir_lshiftr_8_8(src: u64, shamt: i32) -> u64 {
    if shamt < 0 {
        (src << ((-shamt) - 1)) << 1
    } else {
        src >> shamt
    }
}

/// `fSAT_ORIG_SHL(A, ORIG)` — saturate a left-shift result to 32 bits, taking
/// the original register's sign into account so an overflowing shift saturates
/// toward the correct extreme.  `a` is the (untruncated) i64 shift result.
#[inline]
fn sat_orig_shl(ctx: &mut SemCtx, a: i64, orig: u32) -> u32 {
    let orig_s = orig as i32;
    let sat = ctx.sat_n(a, 32) as i32;
    if (sat ^ orig_s) < 0 {
        // sign flipped -> saturate toward ORIG's extreme
        let v = if orig_s < 0 {
            -(1i64 << 31)
        } else {
            (1i64 << 31) - 1
        };
        ctx.set_ovf();
        v as u32
    } else if orig_s > 0 && a == 0 {
        let v = (1i64 << 31) - 1;
        ctx.set_ovf();
        v as u32
    } else {
        sat as u32
    }
}

/// Execute a shift-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let rd = fld(d, b'd');
    let rx = fld(d, b'x');
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let ui = || fimm_u(d, b'i', ctx.immext);
    // Bidirectional shift amount: sign-extend low 7 bits of Rt to i32.
    let shamt = |c: &SemCtx| ((c.r(fld(d, b't')) as i32) << 25) >> 25;

    match op {
        // ---- immediate single-word shifts ----
        Opcode::S2_asl_i_r => {
            let v = ashiftl_32(s(ctx), ui());
            ctx.set_r(rd, v);
        }
        Opcode::S2_asr_i_r => {
            let v = ashiftr_32(s(ctx), ui());
            ctx.set_r(rd, v);
        }
        Opcode::S2_lsr_i_r => {
            let v = lshiftr_32(s(ctx), ui());
            ctx.set_r(rd, v);
        }

        // ---- immediate pair shifts ----
        Opcode::S2_asl_i_p => {
            let v = ashiftl_64(sp(ctx), ui());
            ctx.set_rp(rd, v);
        }
        Opcode::S2_asr_i_p => {
            let v = ashiftr_64(sp(ctx), ui());
            ctx.set_rp(rd, v);
        }
        Opcode::S2_lsr_i_p => {
            let v = lshiftr_64(sp(ctx), ui());
            ctx.set_rp(rd, v);
        }

        // ---- immediate single-word shift-accumulate (RxV op asl/asr/lsr) ----
        Opcode::S2_asl_i_r_acc => {
            let v = ctx.r(rx).wrapping_add(ashiftl_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_i_r_nac => {
            let v = ctx.r(rx).wrapping_sub(ashiftl_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_i_r_and => {
            let v = ctx.r(rx) & ashiftl_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_i_r_or => {
            let v = ctx.r(rx) | ashiftl_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_i_r_xacc => {
            let v = ctx.r(rx) ^ ashiftl_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_i_r_acc => {
            let v = ctx.r(rx).wrapping_add(ashiftr_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_i_r_nac => {
            let v = ctx.r(rx).wrapping_sub(ashiftr_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_i_r_and => {
            let v = ctx.r(rx) & ashiftr_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_i_r_or => {
            let v = ctx.r(rx) | ashiftr_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_i_r_acc => {
            let v = ctx.r(rx).wrapping_add(lshiftr_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_i_r_nac => {
            let v = ctx.r(rx).wrapping_sub(lshiftr_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_i_r_and => {
            let v = ctx.r(rx) & lshiftr_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_i_r_or => {
            let v = ctx.r(rx) | lshiftr_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_i_r_xacc => {
            let v = ctx.r(rx) ^ lshiftr_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }

        // ---- immediate pair shift-accumulate (RxxV op asl/asr/lsr) ----
        Opcode::S2_asl_i_p_acc => {
            let v = ctx.rp(rx).wrapping_add(ashiftl_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_i_p_nac => {
            let v = ctx.rp(rx).wrapping_sub(ashiftl_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_i_p_and => {
            let v = ctx.rp(rx) & ashiftl_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_i_p_or => {
            let v = ctx.rp(rx) | ashiftl_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_i_p_xacc => {
            let v = ctx.rp(rx) ^ ashiftl_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_i_p_acc => {
            let v = ctx.rp(rx).wrapping_add(ashiftr_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_i_p_nac => {
            let v = ctx.rp(rx).wrapping_sub(ashiftr_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_i_p_and => {
            let v = ctx.rp(rx) & ashiftr_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_i_p_or => {
            let v = ctx.rp(rx) | ashiftr_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_i_p_acc => {
            let v = ctx.rp(rx).wrapping_add(lshiftr_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_i_p_nac => {
            let v = ctx.rp(rx).wrapping_sub(lshiftr_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_i_p_and => {
            let v = ctx.rp(rx) & lshiftr_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_i_p_or => {
            let v = ctx.rp(rx) | lshiftr_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_i_p_xacc => {
            let v = ctx.rp(rx) ^ lshiftr_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }

        // ---- saturating / rounding immediate shifts ----
        Opcode::S2_asl_i_r_sat => {
            // fSAT(fASHIFTL(RsV,uiV,4_8)) — left-shift i32->i64, saturate to 32.
            let a = (s(ctx) as i32 as i64) << ui();
            let v = ctx.sat_n(a, 32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::S2_asr_i_r_rnd => {
            // fASHIFTR(((fASHIFTR(RsV,uiV,4_8))+1),1,8_8)
            let inner = (s(ctx) as i32 as i64) >> ui();
            let v = ((inner + 1) >> 1) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::S2_asr_i_svw_trun => {
            // For each word i of Rss: halfword i of Rd = low16( sxt(word_i) >> u5 ).
            let rss = sp(ctx);
            let u = ui();
            let mut out: u32 = 0;
            for i in 0..2u32 {
                let word = ((rss >> (i * 32)) & 0xffff_ffff) as u32 as i32 as i64;
                let half = ((word >> u) & 0xffff) as u32;
                out |= half << (i * 16);
            }
            ctx.set_r(rd, out);
        }

        // ---- S4_lsli: Rd = lsl(#s6, Rt) (bidirectional, immediate source) ----
        Opcode::S4_lsli => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = bidir_lshiftl_4_8(imm, shamt(ctx)) as u32;
            ctx.set_r(rd, v);
        }

        // ---- S2_addasl_rrri: Rd = Rt + asl(Rs, #u3) ----
        Opcode::S2_addasl_rrri => {
            let v = t(ctx).wrapping_add(ashiftl_32(s(ctx), ui()));
            ctx.set_r(rd, v);
        }

        // ---- register-amount pair shifts (bidirectional) ----
        Opcode::S2_asl_r_p => {
            let v = bidir_ashiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::S2_asr_r_p => {
            let v = bidir_ashiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::S2_lsr_r_p => {
            let v = bidir_lshiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::S2_lsl_r_p => {
            let v = bidir_lshiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rd, v);
        }

        // ---- register-amount pair shift-accumulate ----
        Opcode::S2_asl_r_p_acc => {
            let v = ctx
                .rp(rx)
                .wrapping_add(bidir_ashiftl_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_r_p_nac => {
            let v = ctx
                .rp(rx)
                .wrapping_sub(bidir_ashiftl_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_r_p_and => {
            let v = ctx.rp(rx) & bidir_ashiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_r_p_or => {
            let v = ctx.rp(rx) | bidir_ashiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asl_r_p_xor => {
            let v = ctx.rp(rx) ^ bidir_ashiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_r_p_acc => {
            let v = ctx
                .rp(rx)
                .wrapping_add(bidir_ashiftr_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_r_p_nac => {
            let v = ctx
                .rp(rx)
                .wrapping_sub(bidir_ashiftr_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_r_p_and => {
            let v = ctx.rp(rx) & bidir_ashiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_r_p_or => {
            let v = ctx.rp(rx) | bidir_ashiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_asr_r_p_xor => {
            let v = ctx.rp(rx) ^ bidir_ashiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_r_p_acc => {
            let v = ctx
                .rp(rx)
                .wrapping_add(bidir_lshiftr_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_r_p_nac => {
            let v = ctx
                .rp(rx)
                .wrapping_sub(bidir_lshiftr_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_r_p_and => {
            let v = ctx.rp(rx) & bidir_lshiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_r_p_or => {
            let v = ctx.rp(rx) | bidir_lshiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsr_r_p_xor => {
            let v = ctx.rp(rx) ^ bidir_lshiftr_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsl_r_p_acc => {
            let v = ctx
                .rp(rx)
                .wrapping_add(bidir_lshiftl_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsl_r_p_nac => {
            let v = ctx
                .rp(rx)
                .wrapping_sub(bidir_lshiftl_8_8(sp(ctx), shamt(ctx)));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsl_r_p_and => {
            let v = ctx.rp(rx) & bidir_lshiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsl_r_p_or => {
            let v = ctx.rp(rx) | bidir_lshiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }
        Opcode::S2_lsl_r_p_xor => {
            let v = ctx.rp(rx) ^ bidir_lshiftl_8_8(sp(ctx), shamt(ctx));
            ctx.set_rp(rx, v);
        }

        // ---- register-amount single-word shift-accumulate (bidirectional) ----
        Opcode::S2_asl_r_r_acc => {
            let v = ctx
                .r(rx)
                .wrapping_add(bidir_ashiftl_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_r_r_nac => {
            let v = ctx
                .r(rx)
                .wrapping_sub(bidir_ashiftl_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_r_r_and => {
            let v = ctx.r(rx) & bidir_ashiftl_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_asl_r_r_or => {
            let v = ctx.r(rx) | bidir_ashiftl_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_r_r_acc => {
            let v = ctx
                .r(rx)
                .wrapping_add(bidir_ashiftr_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_r_r_nac => {
            let v = ctx
                .r(rx)
                .wrapping_sub(bidir_ashiftr_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_r_r_and => {
            let v = ctx.r(rx) & bidir_ashiftr_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_asr_r_r_or => {
            let v = ctx.r(rx) | bidir_ashiftr_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_r_r_acc => {
            let v = ctx
                .r(rx)
                .wrapping_add(bidir_lshiftr_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_r_r_nac => {
            let v = ctx
                .r(rx)
                .wrapping_sub(bidir_lshiftr_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_r_r_and => {
            let v = ctx.r(rx) & bidir_lshiftr_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsr_r_r_or => {
            let v = ctx.r(rx) | bidir_lshiftr_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsl_r_r_acc => {
            let v = ctx
                .r(rx)
                .wrapping_add(bidir_lshiftl_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsl_r_r_nac => {
            let v = ctx
                .r(rx)
                .wrapping_sub(bidir_lshiftl_4_8(s(ctx), shamt(ctx)) as u32);
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsl_r_r_and => {
            let v = ctx.r(rx) & bidir_lshiftl_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }
        Opcode::S2_lsl_r_r_or => {
            let v = ctx.r(rx) | bidir_lshiftl_4_8(s(ctx), shamt(ctx)) as u32;
            ctx.set_r(rx, v);
        }

        // ---- register-amount single-word saturating shifts ----
        Opcode::S2_asl_r_r_sat => {
            // fBIDIR_ASHIFTL_SAT(RsV,shamt,4_8)
            let sh = shamt(ctx);
            let src = s(ctx);
            let v = if sh < 0 {
                bidir_ashiftl_4_8(src, sh) as u32
            } else {
                let a = (src as i32 as i64) << sh;
                sat_orig_shl(ctx, a, src)
            };
            ctx.set_r(rd, v);
        }
        Opcode::S2_asr_r_r_sat => {
            // fBIDIR_ASHIFTR_SAT(RsV,shamt,4_8)
            let sh = shamt(ctx);
            let src = s(ctx);
            let v = if sh < 0 {
                let a = ((src as i32 as i64) << ((-sh) - 1)) << 1;
                sat_orig_shl(ctx, a, src)
            } else {
                bidir_ashiftr_4_8(src, sh) as u32
            };
            ctx.set_r(rd, v);
        }

        // ---- rotates ----
        Opcode::S6_rol_i_r => {
            let v = rotl_32(s(ctx), ui());
            ctx.set_r(rd, v);
        }
        Opcode::S6_rol_i_r_acc => {
            let v = ctx.r(rx).wrapping_add(rotl_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S6_rol_i_r_nac => {
            let v = ctx.r(rx).wrapping_sub(rotl_32(s(ctx), ui()));
            ctx.set_r(rx, v);
        }
        Opcode::S6_rol_i_r_and => {
            let v = ctx.r(rx) & rotl_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S6_rol_i_r_or => {
            let v = ctx.r(rx) | rotl_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S6_rol_i_r_xacc => {
            let v = ctx.r(rx) ^ rotl_32(s(ctx), ui());
            ctx.set_r(rx, v);
        }
        Opcode::S6_rol_i_p => {
            let v = rotl_64(sp(ctx), ui());
            ctx.set_rp(rd, v);
        }
        Opcode::S6_rol_i_p_acc => {
            let v = ctx.rp(rx).wrapping_add(rotl_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S6_rol_i_p_nac => {
            let v = ctx.rp(rx).wrapping_sub(rotl_64(sp(ctx), ui()));
            ctx.set_rp(rx, v);
        }
        Opcode::S6_rol_i_p_and => {
            let v = ctx.rp(rx) & rotl_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S6_rol_i_p_or => {
            let v = ctx.rp(rx) | rotl_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }
        Opcode::S6_rol_i_p_xacc => {
            let v = ctx.rp(rx) ^ rotl_64(sp(ctx), ui());
            ctx.set_rp(rx, v);
        }

        _ => return false,
    }
    true
}
