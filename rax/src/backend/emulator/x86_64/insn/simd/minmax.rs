//! SSE min/max instructions: MINPS, MAXPS, MINPD, MAXPD, MINSS, MAXSS, MINSD, MAXSD.
//!
//! x86 MIN/MAX semantics differ from IEEE/Rust: MIN returns `(dst < src) ? dst : src`
//! and MAX returns `(dst > src) ? dst : src`. The comparison is false whenever
//! either operand is NaN (or for ±0 ties), so the SECOND operand (src) is returned
//! in those cases — unlike Rust's `f32::min`/`max`, which ignore NaN.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

#[inline(always)]
fn min32(dst: f32, src: f32) -> f32 {
    if dst < src { dst } else { src }
}
#[inline(always)]
fn max32(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}
#[inline(always)]
fn min64(dst: f64, src: f64) -> f64 {
    if dst < src { dst } else { src }
}
#[inline(always)]
fn max64(dst: f64, src: f64) -> f64 {
    if dst > src { dst } else { src }
}

/// SSE packed single/double min (0x5D)
pub fn sse_min(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        let src = if is_memory {
            f32::from_bits(vcpu.read_mem(addr, 4)? as u32)
        } else {
            f32::from_bits(vcpu.regs.xmm[rm as usize][0] as u32)
        };
        let dst = f32::from_bits(vcpu.regs.xmm[xmm_dst][0] as u32);
        vcpu.regs.xmm[xmm_dst][0] =
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | min32(dst, src).to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        vcpu.regs.xmm[xmm_dst][0] = min64(dst, src).to_bits();
    } else if ctx.operand_size_override {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] = min64(
            f64::from_bits(vcpu.regs.xmm[xmm_dst][0]),
            f64::from_bits(src_lo),
        )
        .to_bits();
        vcpu.regs.xmm[xmm_dst][1] = min64(
            f64::from_bits(vcpu.regs.xmm[xmm_dst][1]),
            f64::from_bits(src_hi),
        )
        .to_bits();
    } else {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let (dst_lo, dst_hi) = (vcpu.regs.xmm[xmm_dst][0], vcpu.regs.xmm[xmm_dst][1]);
        let r0 = min32(f32::from_bits(dst_lo as u32), f32::from_bits(src_lo as u32));
        let r1 = min32(
            f32::from_bits((dst_lo >> 32) as u32),
            f32::from_bits((src_lo >> 32) as u32),
        );
        let r2 = min32(f32::from_bits(dst_hi as u32), f32::from_bits(src_hi as u32));
        let r3 = min32(
            f32::from_bits((dst_hi >> 32) as u32),
            f32::from_bits((src_hi >> 32) as u32),
        );
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SSE packed single/double max (0x5F)
pub fn sse_max(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        let src = if is_memory {
            f32::from_bits(vcpu.read_mem(addr, 4)? as u32)
        } else {
            f32::from_bits(vcpu.regs.xmm[rm as usize][0] as u32)
        };
        let dst = f32::from_bits(vcpu.regs.xmm[xmm_dst][0] as u32);
        vcpu.regs.xmm[xmm_dst][0] =
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | max32(dst, src).to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        vcpu.regs.xmm[xmm_dst][0] = max64(dst, src).to_bits();
    } else if ctx.operand_size_override {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] = max64(
            f64::from_bits(vcpu.regs.xmm[xmm_dst][0]),
            f64::from_bits(src_lo),
        )
        .to_bits();
        vcpu.regs.xmm[xmm_dst][1] = max64(
            f64::from_bits(vcpu.regs.xmm[xmm_dst][1]),
            f64::from_bits(src_hi),
        )
        .to_bits();
    } else {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let (dst_lo, dst_hi) = (vcpu.regs.xmm[xmm_dst][0], vcpu.regs.xmm[xmm_dst][1]);
        let r0 = max32(f32::from_bits(dst_lo as u32), f32::from_bits(src_lo as u32));
        let r1 = max32(
            f32::from_bits((dst_lo >> 32) as u32),
            f32::from_bits((src_lo >> 32) as u32),
        );
        let r2 = max32(f32::from_bits(dst_hi as u32), f32::from_bits(src_hi as u32));
        let r3 = max32(
            f32::from_bits((dst_hi >> 32) as u32),
            f32::from_bits((src_hi >> 32) as u32),
        );
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
