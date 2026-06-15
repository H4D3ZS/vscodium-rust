//! SSE arithmetic instructions: ADD, SUB, MUL, DIV, SQRT.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::simd_native;

/// SSE packed single/double add (0x58)
pub fn sse_add(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        // ADDSS - scalar single
        let src = if is_memory {
            f32::from_bits(vcpu.read_mem(addr, 4)? as u32)
        } else {
            f32::from_bits(vcpu.regs.xmm[rm as usize][0] as u32)
        };
        let dst = f32::from_bits(vcpu.regs.xmm[xmm_dst][0] as u32);
        let result = dst + src;
        vcpu.regs.xmm[xmm_dst][0] =
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | result.to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        // ADDSD - scalar double
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        vcpu.regs.xmm[xmm_dst][0] = (dst + src).to_bits();
    } else if ctx.operand_size_override {
        // ADDPD - packed double (2 x f64)
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let d0 = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]) + f64::from_bits(src_lo);
        let d1 = f64::from_bits(vcpu.regs.xmm[xmm_dst][1]) + f64::from_bits(src_hi);
        vcpu.regs.xmm[xmm_dst][0] = d0.to_bits();
        vcpu.regs.xmm[xmm_dst][1] = d1.to_bits();
    } else {
        // ADDPS - packed single (4 x f32)
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];
        let r0 = f32::from_bits(dst_lo as u32) + f32::from_bits(src_lo as u32);
        let r1 = f32::from_bits((dst_lo >> 32) as u32) + f32::from_bits((src_lo >> 32) as u32);
        let r2 = f32::from_bits(dst_hi as u32) + f32::from_bits(src_hi as u32);
        let r3 = f32::from_bits((dst_hi >> 32) as u32) + f32::from_bits((src_hi >> 32) as u32);
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SSE packed single/double subtract (0x5C)
pub fn sse_sub(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | (dst - src).to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        vcpu.regs.xmm[xmm_dst][0] = (dst - src).to_bits();
    } else if ctx.operand_size_override {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] =
            (f64::from_bits(vcpu.regs.xmm[xmm_dst][0]) - f64::from_bits(src_lo)).to_bits();
        vcpu.regs.xmm[xmm_dst][1] =
            (f64::from_bits(vcpu.regs.xmm[xmm_dst][1]) - f64::from_bits(src_hi)).to_bits();
    } else {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let (dst_lo, dst_hi) = (vcpu.regs.xmm[xmm_dst][0], vcpu.regs.xmm[xmm_dst][1]);
        let r0 = f32::from_bits(dst_lo as u32) - f32::from_bits(src_lo as u32);
        let r1 = f32::from_bits((dst_lo >> 32) as u32) - f32::from_bits((src_lo >> 32) as u32);
        let r2 = f32::from_bits(dst_hi as u32) - f32::from_bits(src_hi as u32);
        let r3 = f32::from_bits((dst_hi >> 32) as u32) - f32::from_bits((src_hi >> 32) as u32);
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SSE packed single/double multiply (0x59)
pub fn sse_mul(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | (dst * src).to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        vcpu.regs.xmm[xmm_dst][0] = (dst * src).to_bits();
    } else if ctx.operand_size_override {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] =
            (f64::from_bits(vcpu.regs.xmm[xmm_dst][0]) * f64::from_bits(src_lo)).to_bits();
        vcpu.regs.xmm[xmm_dst][1] =
            (f64::from_bits(vcpu.regs.xmm[xmm_dst][1]) * f64::from_bits(src_hi)).to_bits();
    } else {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let (dst_lo, dst_hi) = (vcpu.regs.xmm[xmm_dst][0], vcpu.regs.xmm[xmm_dst][1]);
        let r0 = f32::from_bits(dst_lo as u32) * f32::from_bits(src_lo as u32);
        let r1 = f32::from_bits((dst_lo >> 32) as u32) * f32::from_bits((src_lo >> 32) as u32);
        let r2 = f32::from_bits(dst_hi as u32) * f32::from_bits(src_hi as u32);
        let r3 = f32::from_bits((dst_hi >> 32) as u32) * f32::from_bits((src_hi >> 32) as u32);
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SSE packed single/double divide (0x5E)
pub fn sse_div(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | (dst / src).to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        let dst = f64::from_bits(vcpu.regs.xmm[xmm_dst][0]);
        vcpu.regs.xmm[xmm_dst][0] = (dst / src).to_bits();
    } else if ctx.operand_size_override {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] =
            (f64::from_bits(vcpu.regs.xmm[xmm_dst][0]) / f64::from_bits(src_lo)).to_bits();
        vcpu.regs.xmm[xmm_dst][1] =
            (f64::from_bits(vcpu.regs.xmm[xmm_dst][1]) / f64::from_bits(src_hi)).to_bits();
    } else {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let (dst_lo, dst_hi) = (vcpu.regs.xmm[xmm_dst][0], vcpu.regs.xmm[xmm_dst][1]);
        let r0 = f32::from_bits(dst_lo as u32) / f32::from_bits(src_lo as u32);
        let r1 = f32::from_bits((dst_lo >> 32) as u32) / f32::from_bits((src_lo >> 32) as u32);
        let r2 = f32::from_bits(dst_hi as u32) / f32::from_bits(src_hi as u32);
        let r3 = f32::from_bits((dst_hi >> 32) as u32) / f32::from_bits((src_hi >> 32) as u32);
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Packed Integer Multiply Instructions (SSE2/MMX)
// =============================================================================

/// PMULLW - Multiply Packed Signed Words, Low Result (0x0F 0xD5)
/// SSE2: 66 0F D5 /r  PMULLW xmm1, xmm2/m128
/// MMX:  NP 0F D5 /r  PMULLW mm, mm/m64
pub fn pmullw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        // SSE2: PMULLW xmm1, xmm2/m128
        let xmm_dst = reg as usize;
        let src: simd_native::Xmm = if is_memory {
            [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
        } else {
            vcpu.regs.xmm[rm as usize]
        };
        simd_native::pmullw_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    } else {
        // MMX: PMULLW mm, mm/m64
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..4 {
            let d = ((dst >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src >> (i * 16)) & 0xFFFF) as i16;
            let r = d.wrapping_mul(s) as u16;
            result |= (r as u64) << (i * 16);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMULHW - Multiply Packed Signed Words, High Result (0x0F 0xE5)
/// SSE2: 66 0F E5 /r  PMULHW xmm1, xmm2/m128
/// MMX:  NP 0F E5 /r  PMULHW mm, mm/m64
pub fn pmulhw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..4 {
            let d = ((dst_lo >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
            let r = (((d as i32) * (s as i32)) >> 16) as u16;
            result_lo |= (r as u64) << (i * 16);
        }
        for i in 0..4 {
            let d = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
            let r = (((d as i32) * (s as i32)) >> 16) as u16;
            result_hi |= (r as u64) << (i * 16);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..4 {
            let d = ((dst >> (i * 16)) & 0xFFFF) as i16;
            let s = ((src >> (i * 16)) & 0xFFFF) as i16;
            let r = (((d as i32) * (s as i32)) >> 16) as u16;
            result |= (r as u64) << (i * 16);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMULHUW - Multiply Packed Unsigned Words, High Result (0x0F 0xE4)
/// SSE2: 66 0F E4 /r  PMULHUW xmm1, xmm2/m128
/// MMX:  NP 0F E4 /r  PMULHUW mm, mm/m64
pub fn pmulhuw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];

        let mut result_lo = 0u64;
        let mut result_hi = 0u64;
        for i in 0..4 {
            let d = ((dst_lo >> (i * 16)) & 0xFFFF) as u16;
            let s = ((src_lo >> (i * 16)) & 0xFFFF) as u16;
            let r = (((d as u32) * (s as u32)) >> 16) as u16;
            result_lo |= (r as u64) << (i * 16);
        }
        for i in 0..4 {
            let d = ((dst_hi >> (i * 16)) & 0xFFFF) as u16;
            let s = ((src_hi >> (i * 16)) & 0xFFFF) as u16;
            let r = (((d as u32) * (s as u32)) >> 16) as u16;
            result_hi |= (r as u64) << (i * 16);
        }
        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        let mut result = 0u64;
        for i in 0..4 {
            let d = ((dst >> (i * 16)) & 0xFFFF) as u16;
            let s = ((src >> (i * 16)) & 0xFFFF) as u16;
            let r = (((d as u32) * (s as u32)) >> 16) as u16;
            result |= (r as u64) << (i * 16);
        }
        vcpu.regs.mm[mm_dst] = result;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMULUDQ - Multiply Packed Unsigned Doubleword Integers (0x0F 0xF4)
/// SSE2: 66 0F F4 /r  PMULUDQ xmm1, xmm2/m128
/// MMX:  NP 0F F4 /r  PMULUDQ mm, mm/m64
pub fn pmuludq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        let xmm_dst = reg as usize;
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let dst_lo = vcpu.regs.xmm[xmm_dst][0];
        let dst_hi = vcpu.regs.xmm[xmm_dst][1];
        // Multiply low dword of each qword as unsigned, producing 64-bit result
        let r0 = (dst_lo as u32 as u64) * (src_lo as u32 as u64);
        let r1 = (dst_hi as u32 as u64) * (src_hi as u32 as u64);
        vcpu.regs.xmm[xmm_dst][0] = r0;
        vcpu.regs.xmm[xmm_dst][1] = r1;
    } else {
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        let dst = vcpu.regs.mm[mm_dst];
        // Multiply low dword as unsigned, producing 64-bit result
        let r = (dst as u32 as u64) * (src as u32 as u64);
        vcpu.regs.mm[mm_dst] = r;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PXOR - XOR Packed Integers (0x0F 0xEF)
/// SSE2: 66 0F EF /r  PXOR xmm1, xmm2/m128
/// MMX:  NP 0F EF /r  PXOR mm, mm/m64
pub fn pxor(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if ctx.operand_size_override {
        // SSE2: PXOR xmm1, xmm2/m128
        let xmm_dst = reg as usize;
        let src: simd_native::Xmm = if is_memory {
            [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
        } else {
            vcpu.regs.xmm[rm as usize]
        };
        simd_native::pxor_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    } else {
        // MMX: PXOR mm, mm/m64
        let mm_dst = reg as usize & 0x7;
        let src = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.mm[rm as usize & 0x7]
        };
        vcpu.regs.mm[mm_dst] ^= src;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SSE packed single/double sqrt (0x51)
pub fn sse_sqrt(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        let src = if is_memory {
            f32::from_bits(vcpu.read_mem(addr, 4)? as u32)
        } else {
            f32::from_bits(vcpu.regs.xmm[rm as usize][0] as u32)
        };
        vcpu.regs.xmm[xmm_dst][0] =
            (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | src.sqrt().to_bits() as u64;
    } else if ctx.rep_prefix == Some(0xF2) {
        let src = if is_memory {
            f64::from_bits(vcpu.read_mem(addr, 8)?)
        } else {
            f64::from_bits(vcpu.regs.xmm[rm as usize][0])
        };
        vcpu.regs.xmm[xmm_dst][0] = src.sqrt().to_bits();
    } else if ctx.operand_size_override {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        vcpu.regs.xmm[xmm_dst][0] = f64::from_bits(src_lo).sqrt().to_bits();
        vcpu.regs.xmm[xmm_dst][1] = f64::from_bits(src_hi).sqrt().to_bits();
    } else {
        let (src_lo, src_hi) = if is_memory {
            (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
        } else {
            (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
        };
        let r0 = f32::from_bits(src_lo as u32).sqrt();
        let r1 = f32::from_bits((src_lo >> 32) as u32).sqrt();
        let r2 = f32::from_bits(src_hi as u32).sqrt();
        let r3 = f32::from_bits((src_hi >> 32) as u32).sqrt();
        vcpu.regs.xmm[xmm_dst][0] = r0.to_bits() as u64 | ((r1.to_bits() as u64) << 32);
        vcpu.regs.xmm[xmm_dst][1] = r2.to_bits() as u64 | ((r3.to_bits() as u64) << 32);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
