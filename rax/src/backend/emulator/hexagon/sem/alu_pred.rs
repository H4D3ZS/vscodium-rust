//! Predicated / conditional scalar ALU & transfer instructions plus leftover
//! scalar `A2_*`/`A4_*` (rounding, combine-with-immediate). Semantics taken
//! verbatim from the Hexagon V68 spec (`semantics_generated.pyinc`), verified
//! against the `qemu-hexagon` reference oracle. See `sem/alu.rs` for the
//! established pattern.
//!
//! Predicated forms read the *old* predicate (`fLSBOLD`/`fLSBOLDNOT` = LSB of
//! `Pu`). When the condition is false the spec executes `CANCEL`, i.e. the
//! destination is *not* written. Because register writes are buffered, simply
//! skipping `set_r`/`set_rp` on the false path realises the cancel.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

/// Execute an `alu_pred`-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    // Field readers, scoped to this instruction.
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let ss = |c: &SemCtx| c.rp(fld(d, b's'));
    let rd = fld(d, b'd');
    // `Pu` LSB (old architectural predicate). Only read for opcodes that
    // actually carry a `Pu4` operand — reading `ctx.p` for unrelated opcodes
    // would index an out-of-range register (their `u` field is a GPR).
    let cond_true = |c: &SemCtx| (c.p(fld(d, b'u')) & 1) != 0;
    // `.new` condition: the in-flight predicate produced earlier in this packet.
    let cond_new = |c: &SemCtx| (c.p_new(fld(d, b'u')) & 1) != 0;

    // Predicated unary value helpers (for the `.new`/old extend & halfword-shift
    // forms): aslh/asrh/sxtb/sxth/zxtb/zxth of Rs.
    let aslh = |c: &SemCtx| s(c) << 16;
    let asrh = |c: &SemCtx| ((s(c) as i32) >> 16) as u32;
    let sxtb = |c: &SemCtx| s(c) as i8 as i32 as u32;
    let sxth = |c: &SemCtx| s(c) as i16 as i32 as u32;
    let zxtb = |c: &SemCtx| s(c) & 0xff;
    let zxth = |c: &SemCtx| s(c) & 0xffff;

    match op {
        // ===================== `.new`-predicate forms =====================
        // Identical to the `.old` forms below but the condition reads the
        // predicate produced earlier in the same packet (fLSBNEW/fLSBNEWNOT).
        Opcode::A2_paddtnew => {
            if cond_new(ctx) {
                let v = s(ctx).wrapping_add(t(ctx));
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_paddfnew => {
            if !cond_new(ctx) {
                let v = s(ctx).wrapping_add(t(ctx));
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_padditnew => {
            if cond_new(ctx) {
                let v = s(ctx).wrapping_add(fimm_s(d, b'i', ctx.immext) as u32);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_paddifnew => {
            if !cond_new(ctx) {
                let v = s(ctx).wrapping_add(fimm_s(d, b'i', ctx.immext) as u32);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_psubtnew => {
            if cond_new(ctx) {
                let v = t(ctx).wrapping_sub(s(ctx));
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_psubfnew => {
            if !cond_new(ctx) {
                let v = t(ctx).wrapping_sub(s(ctx));
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_pandtnew => {
            if cond_new(ctx) {
                let v = s(ctx) & t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_pandfnew => {
            if !cond_new(ctx) {
                let v = s(ctx) & t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_portnew => {
            if cond_new(ctx) {
                let v = s(ctx) | t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_porfnew => {
            if !cond_new(ctx) {
                let v = s(ctx) | t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_pxortnew => {
            if cond_new(ctx) {
                let v = s(ctx) ^ t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_pxorfnew => {
            if !cond_new(ctx) {
                let v = s(ctx) ^ t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::C2_cmovenewit => {
            if cond_new(ctx) {
                ctx.set_r(rd, fimm_s(d, b'i', ctx.immext) as u32);
            }
        }
        Opcode::C2_cmovenewif => {
            if !cond_new(ctx) {
                ctx.set_r(rd, fimm_s(d, b'i', ctx.immext) as u32);
            }
        }
        // Predicated halfword-shift / sign-zero-extend, `.new` condition only
        // (the old-predicate forms live in sem/alu_ext.rs).
        Opcode::A4_paslhtnew => {
            if cond_new(ctx) {
                let v = aslh(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_paslhfnew => {
            if !cond_new(ctx) {
                let v = aslh(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_pasrhtnew => {
            if cond_new(ctx) {
                let v = asrh(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_pasrhfnew => {
            if !cond_new(ctx) {
                let v = asrh(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_psxtbtnew => {
            if cond_new(ctx) {
                let v = sxtb(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_psxtbfnew => {
            if !cond_new(ctx) {
                let v = sxtb(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_psxthtnew => {
            if cond_new(ctx) {
                let v = sxth(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_psxthfnew => {
            if !cond_new(ctx) {
                let v = sxth(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_pzxtbtnew => {
            if cond_new(ctx) {
                let v = zxtb(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_pzxtbfnew => {
            if !cond_new(ctx) {
                let v = zxtb(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_pzxthtnew => {
            if cond_new(ctx) {
                let v = zxth(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A4_pzxthfnew => {
            if !cond_new(ctx) {
                let v = zxth(ctx);
                ctx.set_r(rd, v);
            }
        }

        // ---- predicated add (register) : Rd=if(Pu)add(Rs,Rt) ----
        Opcode::A2_paddt => {
            if cond_true(ctx) {
                let v = s(ctx).wrapping_add(t(ctx));
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_paddf => {
            if !cond_true(ctx) {
                let v = s(ctx).wrapping_add(t(ctx));
                ctx.set_r(rd, v);
            }
        }

        // ---- predicated add (immediate) : Rd=if(Pu)add(Rs,#s8) ----
        Opcode::A2_paddit => {
            if cond_true(ctx) {
                let v = s(ctx).wrapping_add(fimm_s(d, b'i', ctx.immext) as u32);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_paddif => {
            if !cond_true(ctx) {
                let v = s(ctx).wrapping_add(fimm_s(d, b'i', ctx.immext) as u32);
                ctx.set_r(rd, v);
            }
        }

        // ---- predicated sub : Rd=if(Pu)sub(Rt,Rs)  (note operand order) ----
        Opcode::A2_psubt => {
            if cond_true(ctx) {
                let v = t(ctx).wrapping_sub(s(ctx));
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_psubf => {
            if !cond_true(ctx) {
                let v = t(ctx).wrapping_sub(s(ctx));
                ctx.set_r(rd, v);
            }
        }

        // ---- predicated and ----
        Opcode::A2_pandt => {
            if cond_true(ctx) {
                let v = s(ctx) & t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_pandf => {
            if !cond_true(ctx) {
                let v = s(ctx) & t(ctx);
                ctx.set_r(rd, v);
            }
        }

        // ---- predicated or ----
        Opcode::A2_port => {
            if cond_true(ctx) {
                let v = s(ctx) | t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_porf => {
            if !cond_true(ctx) {
                let v = s(ctx) | t(ctx);
                ctx.set_r(rd, v);
            }
        }

        // ---- predicated xor ----
        Opcode::A2_pxort => {
            if cond_true(ctx) {
                let v = s(ctx) ^ t(ctx);
                ctx.set_r(rd, v);
            }
        }
        Opcode::A2_pxorf => {
            if !cond_true(ctx) {
                let v = s(ctx) ^ t(ctx);
                ctx.set_r(rd, v);
            }
        }

        // ---- conditional move of immediate : if(Pu) Rd=#s12 ----
        Opcode::C2_cmoveit => {
            if cond_true(ctx) {
                ctx.set_r(rd, fimm_s(d, b'i', ctx.immext) as u32);
            }
        }
        Opcode::C2_cmoveif => {
            if !cond_true(ctx) {
                ctx.set_r(rd, fimm_s(d, b'i', ctx.immext) as u32);
            }
        }

        // ---- combine register with immediate : Rdd=combine(...) (16:16 words) ----
        // A4_combineri: Rdd=combine(Rs,#s8) -> word1=Rs, word0=#s8 (sign-ext, ext'able)
        Opcode::A4_combineri => {
            let lo = fimm_s(d, b'i', ctx.immext) as u32 as u64;
            let hi = s(ctx) as u64;
            ctx.set_rp(rd, (hi << 32) | lo);
        }
        // A4_combineir: Rdd=combine(#s8,Rs) -> word1=#s8, word0=Rs
        Opcode::A4_combineir => {
            let lo = s(ctx) as u64;
            let hi = fimm_s(d, b'i', ctx.immext) as u32 as u64;
            ctx.set_rp(rd, (hi << 32) | lo);
        }
        // A4_combineii: Rdd=combine(#s8,#U6) -> word0=#U6 (field I, unsigned,
        // extendable), word1=#s8 (field i, signed s8). Mirrors the spec
        // `fIMMEXT(UiV); fSETWORD(0,Rdd,UiV); fSETWORD(1,Rdd,siV)`.
        Opcode::A4_combineii => {
            let lo = fimm_u(d, b'I', ctx.immext) as u64;
            let hi = fimm_s(d, b'i', None) as u32 as u64;
            ctx.set_rp(rd, (hi << 32) | lo);
        }

        // ---- round to nearest, with optional :sat ----
        // fRNDN(A,N) = (N==0) ? A : sxt32_64(A) + (1<<(N-1))
        // A4_round_ri: Rd = fRNDN(Rs,#u5) >> #u5        (arithmetic >> on i64)
        Opcode::A4_round_ri => {
            let n = fimm_u(d, b'i', None) & 0x1f;
            let v = (rndn(s(ctx), n) >> n) as u32;
            ctx.set_r(rd, v);
        }
        // A4_round_ri_sat: Rd = fSAT(fRNDN(Rs,#u5)) >> #u5  (sat to 32 bits, THEN shift)
        Opcode::A4_round_ri_sat => {
            let n = fimm_u(d, b'i', None) & 0x1f;
            let sat = ctx.sat_n(rndn(s(ctx), n), 32);
            let v = (sat >> n) as u32;
            ctx.set_r(rd, v);
        }
        // A4_round_rr: N = Rt & 0x1f
        Opcode::A4_round_rr => {
            let n = t(ctx) & 0x1f;
            let v = (rndn(s(ctx), n) >> n) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A4_round_rr_sat => {
            let n = t(ctx) & 0x1f;
            let sat = ctx.sat_n(rndn(s(ctx), n), 32);
            let v = (sat >> n) as u32;
            ctx.set_r(rd, v);
        }

        // ---- A2_roundsat: Rd=round(Rss):sat ----
        // fADDSAT64(tmp, Rss, 0x80000000); Rd = fGETWORD(1,tmp)  (high word)
        Opcode::A2_roundsat => {
            let tmp = addsat64(ctx, ss(ctx) as i64, 0x8000_0000i64);
            // fGETWORD(1, tmp): bits 63:32 (as a 32-bit value).
            let v = (tmp >> 32) as u32;
            ctx.set_r(rd, v);
        }

        _ => return false,
    }
    true
}

/// `fRNDN(A, N)` — round-to-nearest bias. `A` is a 32-bit register value
/// sign-extended to 64 bits; for `N != 0` add `1 << (N-1)`.
#[inline]
fn rndn(a: u32, n: u32) -> i64 {
    let a = a as i32 as i64;
    if n == 0 { a } else { a + (1i64 << (n - 1)) }
}

/// `fADDSAT64(DST, A, B)` — signed 64-bit saturating add. Sets the sticky
/// overflow flag on saturation. Returns the (possibly saturated) sum.
#[inline]
fn addsat64(ctx: &mut SemCtx, a: i64, b: i64) -> i64 {
    let ua = a as u64;
    let ub = b as u64;
    let sum = ua.wrapping_add(ub);
    let xor = ua ^ ub;
    const MASK: u64 = 0x8000_0000_0000_0000;
    if xor & MASK != 0 {
        // Opposite signs: cannot overflow.
        sum as i64
    } else if (ua ^ sum) & MASK != 0 {
        // Operand signs matched but result sign flipped: overflow.
        ctx.set_ovf();
        if sum & MASK != 0 {
            // Overflowed to negative -> clamp to max positive.
            0x7FFF_FFFF_FFFF_FFFF
        } else {
            // Overflowed to positive -> clamp to max negative.
            i64::MIN
        }
    } else {
        sum as i64
    }
}
