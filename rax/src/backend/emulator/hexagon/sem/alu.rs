//! Scalar/pair ALU instructions (`A2_*`, `A4_*`): add/sub/logical, min/max,
//! abs/neg, saturate, sign-extend, combine. Semantics from the Hexagon V68 spec.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fld};

/// Execute an ALU-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    // Field readers, scoped to this instruction.
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let rd = fld(d, b'd');

    match op {
        // ---- 64-bit pair add/sub/logical ----
        Opcode::A2_addp => {
            let v = sp(ctx).wrapping_add(tp(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::A2_subp => {
            let v = tp(ctx).wrapping_sub(sp(ctx)); // sub(Rtt,Rss)
            ctx.set_rp(rd, v);
        }
        Opcode::A2_andp => {
            let v = sp(ctx) & tp(ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::A2_orp => {
            let v = sp(ctx) | tp(ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::A2_xorp => {
            let v = sp(ctx) ^ tp(ctx);
            ctx.set_rp(rd, v);
        }
        Opcode::A2_negp => {
            let v = (sp(ctx) as i64).wrapping_neg() as u64;
            ctx.set_rp(rd, v);
        }
        Opcode::A2_notp => {
            let v = !sp(ctx);
            ctx.set_rp(rd, v);
        }

        // ---- abs / neg (32-bit) ----
        Opcode::A2_abs => {
            let v = (s(ctx) as i32).wrapping_abs() as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_abssat => {
            let a = (s(ctx) as i32 as i64).abs();
            let v = ctx.sat_n(a, 32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_negsat => {
            let v = ctx.sat_n(-(s(ctx) as i32 as i64), 32) as u32;
            ctx.set_r(rd, v);
        }

        // ---- min / max (32-bit) ----  (note operand order differs per spec)
        Opcode::A2_max => {
            let v = (s(ctx) as i32).max(t(ctx) as i32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_maxu => {
            let v = s(ctx).max(t(ctx));
            ctx.set_r(rd, v);
        }
        Opcode::A2_min => {
            let v = (t(ctx) as i32).min(s(ctx) as i32) as u32; // min(Rt,Rs)
            ctx.set_r(rd, v);
        }
        Opcode::A2_minu => {
            let v = t(ctx).min(s(ctx));
            ctx.set_r(rd, v);
        }

        // ---- saturating add/sub (32-bit, computed in 64-bit) ----
        Opcode::A2_addsat => {
            let v = ctx.sat_n(s(ctx) as i32 as i64 + t(ctx) as i32 as i64, 32) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subsat => {
            let v = ctx.sat_n(t(ctx) as i32 as i64 - s(ctx) as i32 as i64, 32) as u32; // sub(Rt,Rs)
            ctx.set_r(rd, v);
        }

        // ---- saturate / sign-extend ----
        Opcode::A2_sat => {
            let v = ctx.sat_n(sp(ctx) as i64, 32) as u32; // sat(Rss) 64->32
            ctx.set_r(rd, v);
        }
        Opcode::A2_satb => {
            let v = ctx.sat_n(s(ctx) as i32 as i64, 8) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_sath => {
            let v = ctx.sat_n(s(ctx) as i32 as i64, 16) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_satub => {
            let v = ctx.satu_n(s(ctx) as i32 as i64, 8) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_satuh => {
            let v = ctx.satu_n(s(ctx) as i32 as i64, 16) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_sxtw => {
            let v = s(ctx) as i32 as i64 as u64;
            ctx.set_rp(rd, v);
        }

        // ---- logical with negated operand ----
        Opcode::A4_andn => {
            let v = t(ctx) & !s(ctx); // and(Rt,~Rs)
            ctx.set_r(rd, v);
        }
        Opcode::A4_orn => {
            let v = t(ctx) | !s(ctx); // or(Rt,~Rs)
            ctx.set_r(rd, v);
        }

        // ---- immediate or/sub-reverse ----
        Opcode::A2_orir => {
            let v = s(ctx) | fimm_s(d, b'i', ctx.immext) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A2_subri => {
            let v = (fimm_s(d, b'i', ctx.immext) as u32).wrapping_sub(s(ctx));
            ctx.set_r(rd, v);
        }

        // ---- combine ----
        Opcode::A2_combinew => {
            // combine(Rs,Rt): high word = Rs, low word = Rt.
            let v = ((s(ctx) as u64) << 32) | (t(ctx) as u64);
            ctx.set_rp(rd, v);
        }
        Opcode::A2_combineii => {
            // combine(#s8,#S8): high = siV (field i, extendable), low = SiV (field I).
            let hi = fimm_s(d, b'i', ctx.immext) as u32 as u64;
            let lo = fimm_s(d, b'I', None) as u32 as u64;
            ctx.set_rp(rd, (hi << 32) | lo);
        }

        _ => return false,
    }
    true
}
