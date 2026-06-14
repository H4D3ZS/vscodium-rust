//! SSE comparison instructions: CMPPS, CMPPD, CMPSS, CMPSD.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

// =============================================================================
// Helper functions for float comparison predicates
// =============================================================================

/// Float comparison predicate for f32 (bits 4:0 for AVX, 2:0 for SSE)
pub fn cmp_predicate_f32(a: f32, b: f32, pred: u8) -> bool {
    match pred & 0x1F {
        0x00 => a == b,                                // EQ_OQ
        0x01 => a < b,                                 // LT_OS
        0x02 => a <= b,                                // LE_OS
        0x03 => a.is_nan() || b.is_nan(),              // UNORD_Q
        0x04 => a != b || a.is_nan() || b.is_nan(),    // NEQ_UQ
        0x05 => !(a < b),                              // NLT_US
        0x06 => !(a <= b),                             // NLE_US
        0x07 => !a.is_nan() && !b.is_nan(),            // ORD_Q
        0x08 => a == b || a.is_nan() || b.is_nan(),    // EQ_UQ
        0x09 => !(a >= b),                             // NGE_US
        0x0A => !(a > b),                              // NGT_US
        0x0B => false,                                 // FALSE_OQ
        0x0C => a != b,                                // NEQ_OQ
        0x0D => a >= b,                                // GE_OS
        0x0E => a > b,                                 // GT_OS
        0x0F => true,                                  // TRUE_UQ
        0x10 => a == b,                                // EQ_OS
        0x11 => a < b || a.is_nan() || b.is_nan(),     // LT_OQ
        0x12 => a <= b || a.is_nan() || b.is_nan(),    // LE_OQ
        0x13 => a.is_nan() || b.is_nan(),              // UNORD_S
        0x14 => a != b,                                // NEQ_US
        0x15 => !(a < b) || a.is_nan() || b.is_nan(),  // NLT_UQ
        0x16 => !(a <= b) || a.is_nan() || b.is_nan(), // NLE_UQ
        0x17 => !a.is_nan() && !b.is_nan(),            // ORD_S
        0x18 => a == b,                                // EQ_US
        0x19 => !(a >= b) || a.is_nan() || b.is_nan(), // NGE_UQ
        0x1A => !(a > b) || a.is_nan() || b.is_nan(),  // NGT_UQ
        0x1B => false,                                 // FALSE_OS
        0x1C => a != b || a.is_nan() || b.is_nan(),    // NEQ_OS
        0x1D => a >= b || a.is_nan() || b.is_nan(),    // GE_OQ
        0x1E => a > b || a.is_nan() || b.is_nan(),     // GT_OQ
        0x1F => true,                                  // TRUE_US
        _ => false,
    }
}

/// Float comparison predicate for f64
pub fn cmp_predicate_f64(a: f64, b: f64, pred: u8) -> bool {
    match pred & 0x1F {
        0x00 => a == b,
        0x01 => a < b,
        0x02 => a <= b,
        0x03 => a.is_nan() || b.is_nan(),
        0x04 => a != b || a.is_nan() || b.is_nan(),
        0x05 => !(a < b),
        0x06 => !(a <= b),
        0x07 => !a.is_nan() && !b.is_nan(),
        0x08 => a == b || a.is_nan() || b.is_nan(),
        0x09 => !(a >= b),
        0x0A => !(a > b),
        0x0B => false,
        0x0C => a != b,
        0x0D => a >= b,
        0x0E => a > b,
        0x0F => true,
        0x10 => a == b,
        0x11 => a < b || a.is_nan() || b.is_nan(),
        0x12 => a <= b || a.is_nan() || b.is_nan(),
        0x13 => a.is_nan() || b.is_nan(),
        0x14 => a != b,
        0x15 => !(a < b) || a.is_nan() || b.is_nan(),
        0x16 => !(a <= b) || a.is_nan() || b.is_nan(),
        0x17 => !a.is_nan() && !b.is_nan(),
        0x18 => a == b,
        0x19 => !(a >= b) || a.is_nan() || b.is_nan(),
        0x1A => !(a > b) || a.is_nan() || b.is_nan(),
        0x1B => false,
        0x1C => a != b || a.is_nan() || b.is_nan(),
        0x1D => a >= b || a.is_nan() || b.is_nan(),
        0x1E => a > b || a.is_nan() || b.is_nan(),
        0x1F => true,
        _ => false,
    }
}

// =============================================================================
// SSE Compare Instructions
// =============================================================================

/// SSE CMPPS/CMPPD/CMPSS/CMPSD (0x0F 0xC2)
pub fn cmpps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let imm8 = ctx.consume_u8()?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        // CMPSS - scalar single
        let src = if is_memory {
            f32::from_bits(vcpu.read_mem(addr, 4)? as u32)
        } else {
            f32::from_bits(vcpu.regs.xmm[rm as usize][0] as u32)
        };
        let dst = f32::from_bits(vcpu.regs.xmm[xmm_dst][0] as u32);
        let result = if cmp_predicate_f32(dst, src, imm8) {
            0xFFFFFFFFu32
        } else {
            0u32
        };
        vcpu.regs.xmm[xmm_dst][0] = (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | result as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        // CMPSD - scalar double
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        let result = if cmp_predicate_f64(dst, src, imm8) {
            !0u64
        } else {
            0u64
        };
        vcpu.regs.xmm[xmm_dst][0] = result;
    } else if ctx.operand_size_override {
        // CMPPD - packed double
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let r0 = if cmp_predicate_f64(
            f64::from_bits(vcpu.regs.xmm[xmm_dst][0]),
            f64::from_bits(src_lo),
            imm8,
        ) {
            !0u64
        } else {
            0u64
        };
        let r1 = if cmp_predicate_f64(
            f64::from_bits(vcpu.regs.xmm[xmm_dst][1]),
            f64::from_bits(src_hi),
            imm8,
        ) {
            !0u64
        } else {
            0u64
        };
        vcpu.regs.xmm[xmm_dst][0] = r0;
        vcpu.regs.xmm[xmm_dst][1] = r1;
    } else {
        // CMPPS - packed single
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];
        let r0 = if cmp_predicate_f32(
            f32::from_bits(dst_lo as u32),
            f32::from_bits(src_lo as u32),
            imm8,
        ) {
            0xFFFFFFFFu32
        } else {
            0
        };
        let r1 = if cmp_predicate_f32(
            f32::from_bits((dst_lo >> 32) as u32),
            f32::from_bits((src_lo >> 32) as u32),
            imm8,
        ) {
            0xFFFFFFFFu32
        } else {
            0
        };
        let r2 = if cmp_predicate_f32(
            f32::from_bits(dst_hi as u32),
            f32::from_bits(src_hi as u32),
            imm8,
        ) {
            0xFFFFFFFFu32
        } else {
            0
        };
        let r3 = if cmp_predicate_f32(
            f32::from_bits((dst_hi >> 32) as u32),
            f32::from_bits((src_hi >> 32) as u32),
            imm8,
        ) {
            0xFFFFFFFFu32
        } else {
            0
        };
        vcpu.regs.xmm[xmm_dst][0] = r0 as u64 | ((r1 as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2 as u64 | ((r3 as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// UCOMISS/UCOMISD - Unordered Compare Scalar FP and set EFLAGS (0x0F 0x2E)
pub fn ucomiss_ucomisd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    let (unordered, greater, less) = if ctx.operand_size_override {
        let a = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        let b = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        (a.is_nan() || b.is_nan(), a > b, a < b)
    } else {
        let a = f32::from_bits(vcpu.regs.xmm[xmm_dst][0] as u32);
        let b = if is_memory {
            f32::from_bits(vcpu.read_mem(addr, 4)? as u32)
        } else {
            f32::from_bits(vcpu.regs.xmm[rm as usize][0] as u32)
        };
        (a.is_nan() || b.is_nan(), a > b, a < b)
    };

    let clear_mask = flags::bits::ZF
        | flags::bits::PF
        | flags::bits::CF
        | flags::bits::OF
        | flags::bits::AF
        | flags::bits::SF;
    vcpu.regs.rflags &= !clear_mask;

    if unordered {
        vcpu.regs.rflags |= flags::bits::ZF | flags::bits::PF | flags::bits::CF;
    } else if greater {
        // ZF=PF=CF=0
    } else if less {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags |= flags::bits::ZF;
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
