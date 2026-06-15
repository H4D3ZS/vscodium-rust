//! Compare / predicate instructions (`C2_*`, `C4_*`): scalar & pair compares
//! producing predicates, predicate logic, mux selection, and bit-test compares.
//! Semantics taken verbatim from the Hexagon V68 spec (semantics_generated.pyinc,
//! expanded via gen_semantics.c) and verified against the qemu-hexagon oracle.
//!
//! Predicate registers are 8 bits. A scalar compare's boolean result is splatted
//! to all eight bits (`f8BITSOF(x)` = `x ? 0xff : 0x00`); predicate-logic ops
//! operate bitwise across the 8 bits. `set_p` already takes a `u8`, so any wider
//! intermediate (e.g. `~Ps`) is truncated to 8 bits on write, matching hardware.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

/// Execute a compare-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    // Field readers, scoped to this instruction.
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let ps = |c: &SemCtx| c.p(fld(d, b's'));
    let pt = |c: &SemCtx| c.p(fld(d, b't'));
    let pu = |c: &SemCtx| c.p(fld(d, b'u'));
    let pd = fld(d, b'd');
    let rd = fld(d, b'd');

    // f8BITSOF(x): splat a boolean to all 8 predicate bits.
    let bits8 = |cond: bool| -> u8 { if cond { 0xff } else { 0x00 } };

    match op {
        // ---- scalar register compares (-> predicate) ----
        Opcode::C2_cmpeq => {
            let v = bits8((s(ctx) as i32) == (t(ctx) as i32));
            ctx.set_p(pd, v);
        }
        Opcode::C2_cmpgt => {
            let v = bits8((s(ctx) as i32) > (t(ctx) as i32));
            ctx.set_p(pd, v);
        }
        Opcode::C2_cmpgtu => {
            let v = bits8(s(ctx) > t(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C4_cmpneq => {
            let v = bits8((s(ctx) as i32) != (t(ctx) as i32));
            ctx.set_p(pd, v);
        }
        Opcode::C4_cmplte => {
            let v = bits8((s(ctx) as i32) <= (t(ctx) as i32));
            ctx.set_p(pd, v);
        }
        Opcode::C4_cmplteu => {
            let v = bits8(s(ctx) <= t(ctx));
            ctx.set_p(pd, v);
        }

        // ---- 64-bit pair compares (-> predicate) ----
        Opcode::C2_cmpeqp => {
            let v = bits8(sp(ctx) == tp(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C2_cmpgtp => {
            let v = bits8((sp(ctx) as i64) > (tp(ctx) as i64));
            ctx.set_p(pd, v);
        }
        Opcode::C2_cmpgtup => {
            let v = bits8(sp(ctx) > tp(ctx));
            ctx.set_p(pd, v);
        }

        // ---- scalar immediate compares (-> predicate) ----
        // RsV is signed 32-bit; siV is the sign-extended (extendable) immediate.
        Opcode::C2_cmpeqi => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bits8((s(ctx) as i32) == imm);
            ctx.set_p(pd, v);
        }
        Opcode::C2_cmpgti => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bits8((s(ctx) as i32) > imm);
            ctx.set_p(pd, v);
        }
        Opcode::C2_cmpgtui => {
            // fCAST4u(RsV) > fCAST4u(uiV): unsigned 32-bit compare, #u9 immediate.
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = bits8(s(ctx) > imm);
            ctx.set_p(pd, v);
        }
        Opcode::C4_cmpneqi => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bits8((s(ctx) as i32) != imm);
            ctx.set_p(pd, v);
        }
        Opcode::C4_cmpltei => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bits8((s(ctx) as i32) <= imm);
            ctx.set_p(pd, v);
        }
        Opcode::C4_cmplteui => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = bits8(s(ctx) <= imm);
            ctx.set_p(pd, v);
        }

        // ---- bit-test compares (-> predicate) ----
        Opcode::C2_bitsset => {
            let v = bits8((s(ctx) & t(ctx)) == t(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C2_bitsclr => {
            let v = bits8((s(ctx) & t(ctx)) == 0);
            ctx.set_p(pd, v);
        }
        Opcode::C2_bitsclri => {
            let imm = fimm_u(d, b'i', ctx.immext); // #u6, not extendable
            let v = bits8((s(ctx) & imm) == 0);
            ctx.set_p(pd, v);
        }
        Opcode::C4_nbitsset => {
            let v = bits8((s(ctx) & t(ctx)) != t(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C4_nbitsclr => {
            let v = bits8((s(ctx) & t(ctx)) != 0);
            ctx.set_p(pd, v);
        }
        Opcode::C4_nbitsclri => {
            let imm = fimm_u(d, b'i', ctx.immext); // #u6
            let v = bits8((s(ctx) & imm) != 0);
            ctx.set_p(pd, v);
        }

        // ---- mux: Rd = (P.lsb ? a : b) ----
        Opcode::C2_mux => {
            let v = if pu(ctx) & 1 != 0 { s(ctx) } else { t(ctx) };
            ctx.set_r(rd, v);
        }
        Opcode::C2_muxii => {
            // fLSBOLD(Pu) ? siV(#s8, extendable) : SiV(#S8)
            let a = fimm_s(d, b'i', ctx.immext) as u32;
            let b = fimm_s(d, b'I', None) as u32;
            let v = if pu(ctx) & 1 != 0 { a } else { b };
            ctx.set_r(rd, v);
        }
        Opcode::C2_muxir => {
            // fLSBOLD(Pu) ? RsV : siV(#s8, extendable)
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = if pu(ctx) & 1 != 0 { s(ctx) } else { imm };
            ctx.set_r(rd, v);
        }
        Opcode::C2_muxri => {
            // fLSBOLD(Pu) ? siV(#s8, extendable) : RsV
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = if pu(ctx) & 1 != 0 { imm } else { s(ctx) };
            ctx.set_r(rd, v);
        }

        // ---- predicate logic (8-bit bitwise) ----
        Opcode::C2_and => {
            // Pd = Ps & Pt
            let v = ps(ctx) & pt(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C2_or => {
            let v = ps(ctx) | pt(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C2_xor => {
            let v = ps(ctx) ^ pt(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C2_not => {
            let v = !ps(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C2_andn => {
            // Pd = Pt & ~Ps
            let v = pt(ctx) & !ps(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C2_orn => {
            // Pd = Pt | ~Ps
            let v = pt(ctx) | !ps(ctx);
            ctx.set_p(pd, v);
        }

        // ---- compound predicate logic (Ps, Pt, Pu) ----
        Opcode::C4_and_and => {
            let v = ps(ctx) & pt(ctx) & pu(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C4_and_or => {
            let v = ps(ctx) & (pt(ctx) | pu(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C4_or_and => {
            let v = ps(ctx) | (pt(ctx) & pu(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C4_or_or => {
            let v = ps(ctx) | pt(ctx) | pu(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C4_and_andn => {
            let v = ps(ctx) & pt(ctx) & !pu(ctx);
            ctx.set_p(pd, v);
        }
        Opcode::C4_and_orn => {
            let v = ps(ctx) & (pt(ctx) | !pu(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C4_or_andn => {
            let v = ps(ctx) | (pt(ctx) & !pu(ctx));
            ctx.set_p(pd, v);
        }
        Opcode::C4_or_orn => {
            let v = ps(ctx) | pt(ctx) | !pu(ctx);
            ctx.set_p(pd, v);
        }

        // ---- any8 / all8 ----
        Opcode::C2_any8 => {
            // Pd = (Ps ? 0xff : 0x00)
            let v = bits8(ps(ctx) != 0);
            ctx.set_p(pd, v);
        }
        Opcode::C2_all8 => {
            // Pd = (Ps == 0xff ? 0xff : 0x00)
            let v = bits8(ps(ctx) == 0xff);
            ctx.set_p(pd, v);
        }

        // ---- vitpack: Rd = (Ps & 0x55) | (Pt & 0xAA) ----
        Opcode::C2_vitpack => {
            let v = (ps(ctx) & 0x55) | (pt(ctx) & 0xaa);
            ctx.set_r(rd, v as u32);
        }

        // ---- mask: Rdd byte[i] = (Pt bit i) ? 0xff : 0x00, i=0..7 ----
        Opcode::C2_mask => {
            let p = pt(ctx);
            let mut rddv: u64 = 0;
            for i in 0..8u32 {
                let byte: u64 = if (p >> i) & 1 != 0 { 0xff } else { 0x00 };
                rddv |= byte << (i * 8);
            }
            ctx.set_rp(rd, rddv);
        }

        // ---- transfers between predicate and GPR ----
        Opcode::C2_tfrpr => {
            // Rd = fZXTN(8,32,Ps): zero-extend the 8-bit predicate.
            let v = ps(ctx) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::C2_tfrrp => {
            // Pd = fGETUBYTE(0,Rs): low byte of Rs.
            let v = (s(ctx) & 0xff) as u8;
            ctx.set_p(pd, v);
        }

        // ---- fastcorner9 ----
        // tmp = halfword[(Ps<<8)|Pt] replicated into both halves of a 32-bit word;
        // AND-fold tmp with (tmp>>1) eight times; result nonzero -> true.
        Opcode::C4_fastcorner9 => {
            let v = bits8(fastcorner9(ps(ctx), pt(ctx)));
            ctx.set_p(pd, v);
        }
        Opcode::C4_fastcorner9_not => {
            let v = bits8(!fastcorner9(ps(ctx), pt(ctx)));
            ctx.set_p(pd, v);
        }

        _ => return false,
    }
    true
}

/// fastcorner9 core: returns true when the AND-folded replicated halfword is
/// nonzero (the "9 consecutive set bits" detector). Mirrors the spec exactly:
///   fSETHALF(0,tmp,(Ps<<8)|Pt); fSETHALF(1,tmp,(Ps<<8)|Pt);
///   for (i=1;i<9;i++) tmp &= tmp>>1;
fn fastcorner9(ps: u8, pt: u8) -> bool {
    let half: u32 = ((ps as u32) << 8) | (pt as u32);
    // fSETHALF writes the low 16 bits of `half` into halfword 0 and halfword 1.
    let h = half & 0xffff;
    let mut tmp: u32 = h | (h << 16);
    for _ in 1..9 {
        tmp &= tmp >> 1;
    }
    tmp != 0
}
