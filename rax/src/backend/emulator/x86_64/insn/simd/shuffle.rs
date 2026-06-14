//! SSE shuffle and unpack instructions: PSHUFD, UNPCKLPS, UNPCKHPS.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

// =============================================================================
// SSE Unpack Instructions
// =============================================================================

/// SSE UNPCKLPS/UNPCKLPD (0x14) - unpack and interleave low
pub fn sse_unpcklps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, _src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];

    if ctx.operand_size_override {
        // UNPCKLPD - interleave low doubles
        vcpu.regs.xmm[xmm_dst][1] = src_lo;
    } else {
        // UNPCKLPS - interleave low singles
        let d0 = dst_lo as u32;
        let d1 = (dst_lo >> 32) as u32;
        let s0 = src_lo as u32;
        let s1 = (src_lo >> 32) as u32;
        vcpu.regs.xmm[xmm_dst][0] = d0 as u64 | ((s0 as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = d1 as u64 | ((s1 as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SSE UNPCKHPS/UNPCKHPD (0x15) - unpack and interleave high
pub fn sse_unpckhps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (_src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    if ctx.operand_size_override {
        // UNPCKHPD - interleave high doubles
        vcpu.regs.xmm[xmm_dst][0] = dst_hi;
        vcpu.regs.xmm[xmm_dst][1] = src_hi;
    } else {
        // UNPCKHPS - interleave high singles
        let d2 = dst_hi as u32;
        let d3 = (dst_hi >> 32) as u32;
        let s2 = src_hi as u32;
        let s3 = (src_hi >> 32) as u32;
        vcpu.regs.xmm[xmm_dst][0] = d2 as u64 | ((s2 as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = d3 as u64 | ((s3 as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// SSE Shuffle Instructions
// =============================================================================

/// SSE PSHUFD/PSHUFHW/PSHUFLW (0x0F 0x70)
pub fn pshufd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let imm8 = ctx.consume_u8()?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        // PSHUFHW: shuffle high words, preserve low qword
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] = src_lo;
        let w0 = (src_hi >> (((imm8 >> 0) & 3) * 16)) as u16;
        let w1 = (src_hi >> (((imm8 >> 2) & 3) * 16)) as u16;
        let w2 = (src_hi >> (((imm8 >> 4) & 3) * 16)) as u16;
        let w3 = (src_hi >> (((imm8 >> 6) & 3) * 16)) as u16;
        vcpu.regs.xmm[xmm_dst][1] =
            (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);
    } else if ctx.rep_prefix == Some(0xF2) {
        // PSHUFLW: shuffle low words, preserve high qword
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let w0 = (src_lo >> (((imm8 >> 0) & 3) * 16)) as u16;
        let w1 = (src_lo >> (((imm8 >> 2) & 3) * 16)) as u16;
        let w2 = (src_lo >> (((imm8 >> 4) & 3) * 16)) as u16;
        let w3 = (src_lo >> (((imm8 >> 6) & 3) * 16)) as u16;
        vcpu.regs.xmm[xmm_dst][0] =
            (w0 as u64) | ((w1 as u64) << 16) | ((w2 as u64) << 32) | ((w3 as u64) << 48);
        vcpu.regs.xmm[xmm_dst][1] = src_hi;
    } else if ctx.operand_size_override {
        // PSHUFD: shuffle all 4 dwords
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dwords: [u32; 4] = [
            src_lo as u32,
            (src_lo >> 32) as u32,
            src_hi as u32,
            (src_hi >> 32) as u32,
        ];
        let d0 = dwords[((imm8 >> 0) & 3) as usize];
        let d1 = dwords[((imm8 >> 2) & 3) as usize];
        let d2 = dwords[((imm8 >> 4) & 3) as usize];
        let d3 = dwords[((imm8 >> 6) & 3) as usize];
        vcpu.regs.xmm[xmm_dst][0] = (d0 as u64) | ((d1 as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = (d2 as u64) | ((d3 as u64) << 32);
    } else {
        return Err(Error::Emulator("PSHUFW (MMX) not implemented".to_string()));
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
