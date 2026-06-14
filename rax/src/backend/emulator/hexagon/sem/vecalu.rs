//! Vector (SIMD-within-register) sub-word ALU instructions (`A2_v*`, `A2_sv*`).
//!
//! Operands are split into byte (8b), halfword (16b) or word (32b) lanes; the
//! per-lane operation is applied independently, with optional signed/unsigned
//! per-lane saturation (`:sat`) and rounding (`:rnd`/`:crnd`). 64-bit pair
//! operands use `ctx.rp()`/`ctx.set_rp()`; the paired-halfword `sv*` forms
//! operate on 32-bit registers. Semantics taken verbatim from the Hexagon V68
//! spec (`semantics_generated.pyinc`); lane macros from `imported/macros.def`.

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

// ---- lane extraction helpers (mirror fGET*/fSET* macros) ------------------

/// Signed halfword lane `i` of a 64-bit value (`fGETHALF`).
#[inline]
fn geth(src: u64, i: u32) -> i64 {
    ((src >> (i * 16)) & 0xffff) as u16 as i16 as i64
}
/// Unsigned halfword lane `i` (`fGETUHALF`).
#[inline]
fn getuh(src: u64, i: u32) -> i64 {
    ((src >> (i * 16)) & 0xffff) as i64
}
/// Signed byte lane `i` (`fGETBYTE`).
#[inline]
fn getb(src: u64, i: u32) -> i64 {
    ((src >> (i * 8)) & 0xff) as u8 as i8 as i64
}
/// Unsigned byte lane `i` (`fGETUBYTE`).
#[inline]
fn getub(src: u64, i: u32) -> i64 {
    ((src >> (i * 8)) & 0xff) as i64
}
/// Signed word lane `i` (`fGETWORD`).
#[inline]
fn getw(src: u64, i: u32) -> i64 {
    ((src >> (i * 32)) & 0xffff_ffff) as u32 as i32 as i64
}
/// Unsigned word lane `i` (`fGETUWORD`).
#[inline]
fn getuw(src: u64, i: u32) -> i64 {
    ((src >> (i * 32)) & 0xffff_ffff) as i64
}

/// Insert `val` into halfword lane `i` (`fSETHALF`).
#[inline]
fn seth(dst: &mut u64, i: u32, val: i64) {
    let m = 0xffffu64 << (i * 16);
    *dst = (*dst & !m) | (((val as u64) & 0xffff) << (i * 16));
}
/// Insert `val` into byte lane `i` (`fSETBYTE`).
#[inline]
fn setb(dst: &mut u64, i: u32, val: i64) {
    let m = 0xffu64 << (i * 8);
    *dst = (*dst & !m) | (((val as u64) & 0xff) << (i * 8));
}
/// Insert `val` into word lane `i` (`fSETWORD`).
#[inline]
fn setw(dst: &mut u64, i: u32, val: i64) {
    let m = 0xffff_ffffu64 << (i * 32);
    *dst = (*dst & !m) | (((val as u64) & 0xffff_ffff) << (i * 32));
}

/// Convergent rounding (`fCRND`): if low two bits are 0b11, add 1.
#[inline]
fn crnd(a: i64) -> i64 {
    if (a & 0x3) == 0x3 { a + 1 } else { a }
}

/// Execute a vecalu-class opcode. Returns `false` if `op` is not in this class.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let ss = |c: &SemCtx| c.rp(fld(d, b's'));
    let tt = |c: &SemCtx| c.rp(fld(d, b't'));
    let s = |c: &SemCtx| c.r(fld(d, b's')) as u64;
    let t = |c: &SemCtx| c.r(fld(d, b't')) as u64;
    let rd = fld(d, b'd');

    match op {
        // ---- vector add (halfword) ----
        Opcode::A2_vaddh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, geth(a, i) + geth(b, i));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vaddhs => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.sat_n(geth(a, i) + geth(b, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vadduhs => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.satu_n(getuh(a, i) + getuh(b, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector add (word) ----
        Opcode::A2_vaddw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getw(a, i).wrapping_add(getw(b, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vaddws => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(getw(a, i) + getw(b, i), 32);
                setw(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector add (byte) ----
        Opcode::A2_vaddub => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, getub(a, i) + getub(b, i));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vaddubs => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                let v = ctx.satu_n(getub(a, i) + getub(b, i), 8);
                setb(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector sub (halfword) — note operand order vsub(Rtt,Rss) ----
        Opcode::A2_vsubh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, geth(b, i) - geth(a, i));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vsubhs => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.sat_n(geth(b, i) - geth(a, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vsubuhs => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.satu_n(getuh(b, i) - getuh(a, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector sub (word) ----
        Opcode::A2_vsubw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getw(b, i).wrapping_sub(getw(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vsubws => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(getw(b, i) - getw(a, i), 32);
                setw(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector sub (byte) ----
        Opcode::A2_vsubub => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, getub(b, i) - getub(a, i));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vsububs => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                let v = ctx.satu_n(getub(b, i) - getub(a, i), 8);
                setb(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector average (halfword, signed) ----
        Opcode::A2_vavgh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, (geth(a, i) + geth(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavghr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, (geth(a, i) + geth(b, i) + 1) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavghcr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, crnd(geth(a, i) + geth(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector average (word, signed) ----
        Opcode::A2_vavgw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, (getw(a, i) + getw(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavgwr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, (getw(a, i) + getw(b, i) + 1) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavgwcr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, crnd(getw(a, i) + getw(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector average (byte, unsigned) ----
        Opcode::A2_vavgub => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, (getub(a, i) + getub(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavgubr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, (getub(a, i) + getub(b, i) + 1) >> 1);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector average (halfword, unsigned) ----
        Opcode::A2_vavguh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, (getuh(a, i) + getuh(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavguhr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, (getuh(a, i) + getuh(b, i) + 1) >> 1);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector average (word, unsigned) ----
        Opcode::A2_vavguw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, (getuw(a, i) + getuw(b, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vavguwr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, (getuw(a, i) + getuw(b, i) + 1) >> 1);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector negative average (halfword) — vnavg(Rtt,Rss) ----
        Opcode::A2_vnavgh => {
            let (a, b) = (ss(ctx), tt(ctx)); // a=Rss, b=Rtt
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, (geth(b, i) - geth(a, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vnavghr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.sat_n((geth(b, i) - geth(a, i) + 1) >> 1, 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vnavghcr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.sat_n(crnd(geth(b, i) - geth(a, i)) >> 1, 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector negative average (word) ----
        Opcode::A2_vnavgw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, (getw(b, i) - getw(a, i)) >> 1);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vnavgwr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n((getw(b, i) - getw(a, i) + 1) >> 1, 32);
                setw(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vnavgwcr => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(crnd(getw(b, i) - getw(a, i)) >> 1, 32);
                setw(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector max — fMAX(Rtt_lane, Rss_lane) ----
        Opcode::A2_vmaxh => {
            let (a, b) = (ss(ctx), tt(ctx)); // a=Rss, b=Rtt
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, geth(b, i).max(geth(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vmaxuh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, getuh(b, i).max(getuh(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vmaxw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getw(b, i).max(getw(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vmaxuw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getuw(b, i).max(getuw(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vmaxb => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, getb(b, i).max(getb(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vmaxub => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, getub(b, i).max(getub(a, i)));
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector min — fMIN(Rtt_lane, Rss_lane) ----
        Opcode::A2_vminh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, geth(b, i).min(geth(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vminuh => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, getuh(b, i).min(getuh(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vminw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getw(b, i).min(getw(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vminuw => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getuw(b, i).min(getuw(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vminb => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, getb(b, i).min(getb(a, i)));
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vminub => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            for i in 0..8 {
                setb(&mut r, i, getub(b, i).min(getub(a, i)));
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector abs (halfword/word) ----
        Opcode::A2_vabsh => {
            let a = ss(ctx);
            let mut r = 0u64;
            for i in 0..4 {
                seth(&mut r, i, geth(a, i).abs());
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vabshsat => {
            let a = ss(ctx);
            let mut r = 0u64;
            for i in 0..4 {
                let v = ctx.sat_n(geth(a, i).abs(), 16);
                seth(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vabsw => {
            let a = ss(ctx);
            let mut r = 0u64;
            for i in 0..2 {
                setw(&mut r, i, getw(a, i).abs());
            }
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vabswsat => {
            let a = ss(ctx);
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(getw(a, i).abs(), 32);
                setw(&mut r, i, v);
            }
            ctx.set_rp(rd, r);
        }

        // ---- vector reduce add of unsigned bytes ----
        Opcode::A2_vraddub => {
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = 0u64;
            let mut w0 = 0i64;
            for i in 0..4 {
                w0 += getub(a, i) + getub(b, i);
            }
            setw(&mut r, 0, w0);
            let mut w1 = 0i64;
            for i in 4..8 {
                w1 += getub(a, i) + getub(b, i);
            }
            setw(&mut r, 1, w1);
            ctx.set_rp(rd, r);
        }
        Opcode::A2_vraddub_acc => {
            let rx = fld(d, b'x');
            let (a, b) = (ss(ctx), tt(ctx));
            let mut r = ctx.rp(rx);
            let mut w0 = getw(r, 0);
            for i in 0..4 {
                w0 += getub(a, i) + getub(b, i);
            }
            setw(&mut r, 0, w0);
            let mut w1 = getw(r, 1);
            for i in 4..8 {
                w1 += getub(a, i) + getub(b, i);
            }
            setw(&mut r, 1, w1);
            ctx.set_rp(rx, r);
        }

        // ---- vector complex conjugate (saturating negate of odd halves) ----
        Opcode::A2_vconj => {
            let a = ss(ctx);
            let mut r = 0u64;
            seth(&mut r, 0, geth(a, 0));
            let v1 = ctx.sat_n(-geth(a, 1), 16);
            seth(&mut r, 1, v1);
            seth(&mut r, 2, geth(a, 2));
            let v3 = ctx.sat_n(-geth(a, 3), 16);
            seth(&mut r, 3, v3);
            ctx.set_rp(rd, r);
        }

        // ---- paired-halfword (sv*) forms: 32-bit registers, 2 halfword lanes ----
        Opcode::A2_svaddh => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                seth(&mut r, i, geth(a, i) + geth(b, i));
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svaddhs => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(geth(a, i) + geth(b, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svadduhs => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.satu_n(getuh(a, i) + getuh(b, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svsubh => {
            let (a, b) = (s(ctx), t(ctx)); // vsubh(Rt,Rs): a=Rs, b=Rt
            let mut r = 0u64;
            for i in 0..2 {
                seth(&mut r, i, geth(b, i) - geth(a, i));
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svsubhs => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.sat_n(geth(b, i) - geth(a, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svsubuhs => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                let v = ctx.satu_n(getuh(b, i) - getuh(a, i), 16);
                seth(&mut r, i, v);
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svavgh => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                seth(&mut r, i, (geth(a, i) + geth(b, i)) >> 1);
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svavghs => {
            let (a, b) = (s(ctx), t(ctx));
            let mut r = 0u64;
            for i in 0..2 {
                seth(&mut r, i, (geth(a, i) + geth(b, i) + 1) >> 1);
            }
            ctx.set_r(rd, r as u32);
        }
        Opcode::A2_svnavgh => {
            let (a, b) = (s(ctx), t(ctx)); // vnavgh(Rt,Rs): a=Rs, b=Rt
            let mut r = 0u64;
            for i in 0..2 {
                seth(&mut r, i, (geth(b, i) - geth(a, i)) >> 1);
            }
            ctx.set_r(rd, r as u32);
        }

        _ => return false,
    }
    true
}
