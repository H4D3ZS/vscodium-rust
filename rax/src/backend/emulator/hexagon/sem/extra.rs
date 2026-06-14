//! Additional scalar instructions filling gaps left by the per-class effort:
//! nop, halfword shifts, 64-bit pair abs/min/max/saturating-add and and-not/
//! or-not, compare-into-register (rcmp), low-lane byte/halfword compares (cmpb*/
//! cmph*), and conditional word combine. Semantics from the Hexagon V68 spec;
//! verified against the qemu-hexagon oracle.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fimm_s, fimm_u, fld};

/// Execute an "extra" opcode. Returns `false` if `op` is not in this set.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let s = |c: &SemCtx| c.r(fld(d, b's'));
    let t = |c: &SemCtx| c.r(fld(d, b't'));
    let sp = |c: &SemCtx| c.rp(fld(d, b's'));
    let tp = |c: &SemCtx| c.rp(fld(d, b't'));
    let rd = fld(d, b'd');

    // Low-lane signed/unsigned byte and halfword accessors of Rs/Rt.
    let sb = |c: &SemCtx| s(c) as i8 as i32;
    let tb = |c: &SemCtx| t(c) as i8 as i32;
    let sub = |c: &SemCtx| (s(c) & 0xff) as u32;
    let tub = |c: &SemCtx| (t(c) & 0xff) as u32;
    let sh = |c: &SemCtx| s(c) as i16 as i32;
    let th = |c: &SemCtx| t(c) as i16 as i32;
    let suh = |c: &SemCtx| (s(c) & 0xffff) as u32;
    let tuh = |c: &SemCtx| (t(c) & 0xffff) as u32;
    let bit = |b: bool| if b { 0xffu8 } else { 0x00 };

    match op {
        Opcode::A2_nop => {}

        // ---- halfword shifts (32-bit) ----
        Opcode::A2_aslh => {
            let v = s(ctx) << 16;
            ctx.set_r(rd, v);
        }
        Opcode::A2_asrh => {
            let v = ((s(ctx) as i32) >> 16) as u32;
            ctx.set_r(rd, v);
        }

        // ---- 64-bit pair abs / min / max ----
        Opcode::A2_absp => {
            let v = (sp(ctx) as i64).wrapping_abs() as u64;
            ctx.set_rp(rd, v);
        }
        Opcode::A2_maxp => {
            let v = (sp(ctx) as i64).max(tp(ctx) as i64) as u64;
            ctx.set_rp(rd, v);
        }
        Opcode::A2_minp => {
            let v = (tp(ctx) as i64).min(sp(ctx) as i64) as u64; // min(Rtt,Rss)
            ctx.set_rp(rd, v);
        }
        Opcode::A2_maxup => {
            let v = sp(ctx).max(tp(ctx));
            ctx.set_rp(rd, v);
        }
        Opcode::A2_minup => {
            let v = tp(ctx).min(sp(ctx)); // minu(Rtt,Rss)
            ctx.set_rp(rd, v);
        }

        // ---- 64-bit saturating add (fADDSAT64) ----
        Opcode::A2_addpsat => {
            let a = sp(ctx) as i64;
            let b = tp(ctx) as i64;
            let (sum, ovf) = a.overflowing_add(b);
            let r = if ovf {
                ctx.set_ovf();
                if a < 0 { i64::MIN } else { i64::MAX }
            } else {
                sum
            };
            ctx.set_rp(rd, r as u64);
        }

        // ---- pair and-not / or-not ----
        Opcode::A4_andnp => {
            let v = tp(ctx) & !sp(ctx); // and(Rtt,~Rss)
            ctx.set_rp(rd, v);
        }
        Opcode::A4_ornp => {
            let v = tp(ctx) | !sp(ctx); // or(Rtt,~Rss)
            ctx.set_rp(rd, v);
        }

        // ---- compare into register (Rd = cond ? 1 : 0) ----
        Opcode::A4_rcmpeq => {
            let v = (s(ctx) == t(ctx)) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A4_rcmpneq => {
            let v = (s(ctx) != t(ctx)) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A4_rcmpeqi => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = (s(ctx) == imm) as u32;
            ctx.set_r(rd, v);
        }
        Opcode::A4_rcmpneqi => {
            let imm = fimm_s(d, b'i', ctx.immext) as u32;
            let v = (s(ctx) != imm) as u32;
            ctx.set_r(rd, v);
        }

        // ---- low-lane byte compares -> predicate (all 8 bits) ----
        Opcode::A4_cmpbeq => {
            let v = bit((s(ctx) & 0xff) == (t(ctx) & 0xff));
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmpbgt => {
            let v = bit(sb(ctx) > tb(ctx));
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmpbgtu => {
            let v = bit(sub(ctx) > tub(ctx));
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmpbeqi => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = bit(sub(ctx) == imm);
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmpbgti => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bit(sb(ctx) > imm);
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmpbgtui => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = bit(sub(ctx) > imm);
            ctx.set_p(rd, v);
        }

        // ---- low-lane halfword compares -> predicate ----
        Opcode::A4_cmpheq => {
            let v = bit((s(ctx) & 0xffff) == (t(ctx) & 0xffff));
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmphgt => {
            let v = bit(sh(ctx) > th(ctx));
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmphgtu => {
            let v = bit(suh(ctx) > tuh(ctx));
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmpheqi => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bit(sh(ctx) == imm);
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmphgti => {
            let imm = fimm_s(d, b'i', ctx.immext);
            let v = bit(sh(ctx) > imm);
            ctx.set_p(rd, v);
        }
        Opcode::A4_cmphgtui => {
            let imm = fimm_u(d, b'i', ctx.immext);
            let v = bit(suh(ctx) > imm);
            ctx.set_p(rd, v);
        }

        // ---- halfword combine: Rd = (Rt.X << 16) | Rs.Y ----
        Opcode::A2_combine_hh => {
            let v = ((t(ctx) >> 16) << 16) | (s(ctx) >> 16);
            ctx.set_r(rd, v);
        }
        Opcode::A2_combine_hl => {
            let v = ((t(ctx) >> 16) << 16) | (s(ctx) & 0xffff);
            ctx.set_r(rd, v);
        }
        Opcode::A2_combine_lh => {
            let v = ((t(ctx) & 0xffff) << 16) | (s(ctx) >> 16);
            ctx.set_r(rd, v);
        }
        Opcode::A2_combine_ll => {
            let v = ((t(ctx) & 0xffff) << 16) | (s(ctx) & 0xffff);
            ctx.set_r(rd, v);
        }

        // ---- conditional word combine (predicated; old predicate LSB) ----
        Opcode::C2_ccombinewt => {
            if ctx.p(fld(d, b'u')) & 1 != 0 {
                let v = ((s(ctx) as u64) << 32) | (t(ctx) as u64);
                ctx.set_rp(rd, v);
            }
        }
        Opcode::C2_ccombinewf => {
            if ctx.p(fld(d, b'u')) & 1 == 0 {
                let v = ((s(ctx) as u64) << 32) | (t(ctx) as u64);
                ctx.set_rp(rd, v);
            }
        }

        _ => return false,
    }
    true
}
