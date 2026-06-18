//! SSE packed operations: MOVUPS, MOVAPS, ANDPS, ORPS, XORPS, etc.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::simd_native;

// =============================================================================
// Packed Move Instructions (MOVUPS, MOVAPS, MOVSS, MOVSD)
// =============================================================================

/// MOVUPS/MOVUPD/MOVSS/MOVSD xmm, xmm/m (0F 10)
/// NP: MOVUPS, 66: MOVUPD, F3: MOVSS, F2: MOVSD
pub fn movups_load(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        // MOVSS - move scalar single
        let value = if is_memory {
            vcpu.read_mem(addr, 4)?
        } else {
            vcpu.regs.xmm[rm as usize][0] & 0xFFFFFFFF
        };
        if is_memory {
            vcpu.regs.xmm[xmm_dst][0] = value;
            vcpu.regs.xmm[xmm_dst][1] = 0;
        } else {
            // Reg-to-reg: merge low 32 bits, keep rest
            vcpu.regs.xmm[xmm_dst][0] = (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | value;
        }
    } else if ctx.rep_prefix == Some(0xF2) {
        // MOVSD - move scalar double
        let value = if is_memory {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.regs.xmm[rm as usize][0]
        };
        if is_memory {
            vcpu.regs.xmm[xmm_dst][0] = value;
            vcpu.regs.xmm[xmm_dst][1] = 0;
        } else {
            vcpu.regs.xmm[xmm_dst][0] = value;
        }
    } else {
        // MOVUPS/MOVUPD - move packed (unaligned OK)
        if is_memory {
            vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
            vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
        } else {
            let xmm_src = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVUPS/MOVUPD/MOVSS/MOVSD xmm/m, xmm (0F 11)
/// NP: MOVUPS, 66: MOVUPD, F3: MOVSS, F2: MOVSD
pub fn movups_store(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if ctx.rep_prefix == Some(0xF3) {
        // MOVSS store
        let value = vcpu.regs.xmm[xmm_src][0] & 0xFFFFFFFF;
        if is_memory {
            vcpu.write_mem(addr, value, 4)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = (vcpu.regs.xmm[xmm_dst][0] & !0xFFFFFFFF) | value;
        }
    } else if ctx.rep_prefix == Some(0xF2) {
        // MOVSD store
        let value = vcpu.regs.xmm[xmm_src][0];
        if is_memory {
            vcpu.write_mem(addr, value, 8)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = value;
        }
    } else {
        // MOVUPS/MOVUPD store
        if is_memory {
            vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVAPS/MOVAPD xmm, xmm/m128 (0F 28)
pub fn movaps_load(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if is_memory {
        if addr & 0xF != 0 {
            return Err(Error::Emulator(format!(
                "MOVAPS/MOVAPD: unaligned memory access at {:#x}",
                addr
            )));
        }
        vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
        vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVAPS/MOVAPD xmm/m128, xmm (0F 29)
pub fn movaps_store(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if is_memory {
        if addr & 0xF != 0 {
            return Err(Error::Emulator(format!(
                "MOVAPS/MOVAPD: unaligned memory access at {:#x}",
                addr
            )));
        }
        vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
        vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
    } else {
        let xmm_dst = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Packed Logical Operations (ANDPS, ANDNPS, ORPS, XORPS)
// =============================================================================

/// ANDPS/ANDPD xmm, xmm/m128 (0F 54)
pub fn andps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    vcpu.regs.xmm[xmm_dst][0] &= src_lo;
    vcpu.regs.xmm[xmm_dst][1] &= src_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ANDNPS/ANDNPD xmm, xmm/m128 (0F 55)
pub fn andnps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    vcpu.regs.xmm[xmm_dst][0] = (!vcpu.regs.xmm[xmm_dst][0]) & src_lo;
    vcpu.regs.xmm[xmm_dst][1] = (!vcpu.regs.xmm[xmm_dst][1]) & src_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ORPS/ORPD xmm, xmm/m128 (0F 56)
pub fn orps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    vcpu.regs.xmm[xmm_dst][0] |= src_lo;
    vcpu.regs.xmm[xmm_dst][1] |= src_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XORPS/XORPD xmm, xmm/m128 (0F 57)
pub fn xorps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    vcpu.regs.xmm[xmm_dst][0] ^= src_lo;
    vcpu.regs.xmm[xmm_dst][1] ^= src_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Prefetch Hints (PREFETCHNTA/PREFETCHT0/PREFETCHT1/PREFETCHT2)
// =============================================================================

/// PREFETCHh m8 (0F 18 /0-3) - cache prefetch hints, treated as NOP in emulator
pub fn prefetchh(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let hint = (modrm >> 3) & 0x07;

    if hint > 3 {
        return Err(Error::Emulator(format!(
            "unimplemented PREFETCHh hint /{} at RIP={:#x}",
            hint, vcpu.regs.rip
        )));
    }

    let (_, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PREFETCHW/PREFETCHWT1 m8 (0F 0D /1-2) - prefetch with intent to write, treated as NOP
pub fn prefetchw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let hint = (modrm >> 3) & 0x07;

    if hint != 1 && hint != 2 {
        return Err(Error::Emulator(format!(
            "unimplemented PREFETCHW hint /{} at RIP={:#x}",
            hint, vcpu.regs.rip
        )));
    }

    let (_, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Packed Integer Subtract (PSUB* family)
// =============================================================================

/// PSUB* packed integer subtract (SSE2, 66 0F xx)
pub fn psub_packed(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return psub_packed_mmx(vcpu, ctx, opcode);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    match opcode {
        // Wrapping variants - use native SIMD
        0xF8 => simd_native::psubb_xmm(&mut vcpu.regs.xmm[xmm_dst], &src), // PSUBB
        0xF9 => simd_native::psubw_xmm(&mut vcpu.regs.xmm[xmm_dst], &src), // PSUBW
        0xFA => simd_native::psubd_xmm(&mut vcpu.regs.xmm[xmm_dst], &src), // PSUBD
        0xFB => simd_native::psubq_xmm(&mut vcpu.regs.xmm[xmm_dst], &src), // PSUBQ
        // Saturating variants - keep scalar for now (no native support)
        0xD8 | 0xD9 | 0xE8 | 0xE9 => {
            let dst_lo = vcpu.regs.xmm[xmm_dst][0];
            let dst_hi = vcpu.regs.xmm[xmm_dst][1];
            let (res_lo, res_hi) = match opcode {
                0xD8 => (
                    sub_u8_saturate(dst_lo, src[0]),
                    sub_u8_saturate(dst_hi, src[1]),
                ), // PSUBUSB
                0xD9 => (
                    sub_u16_saturate(dst_lo, src[0]),
                    sub_u16_saturate(dst_hi, src[1]),
                ), // PSUBUSW
                0xE8 => (
                    sub_i8_saturate(dst_lo, src[0]),
                    sub_i8_saturate(dst_hi, src[1]),
                ), // PSUBSB
                0xE9 => (
                    sub_i16_saturate(dst_lo, src[0]),
                    sub_i16_saturate(dst_hi, src[1]),
                ), // PSUBSW
                _ => unreachable!(),
            };
            vcpu.regs.xmm[xmm_dst][0] = res_lo;
            vcpu.regs.xmm[xmm_dst][1] = res_hi;
        }
        _ => {
            return Err(Error::Emulator(format!(
                "unimplemented PSUB opcode {:#x} at RIP={:#x}",
                opcode, vcpu.regs.rip
            )));
        }
    };

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn psub_packed_mmx(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let result = match opcode {
        0xD8 => sub_u8_saturate(dst, src),  // PSUBUSB
        0xD9 => sub_u16_saturate(dst, src), // PSUBUSW
        0xE8 => sub_i8_saturate(dst, src),  // PSUBSB
        0xE9 => sub_i16_saturate(dst, src), // PSUBSW
        0xF8 => sub_u8_wrap(dst, src),      // PSUBB
        0xF9 => sub_u16_wrap(dst, src),     // PSUBW
        0xFA => sub_u32_wrap(dst, src),     // PSUBD
        0xFB => dst.wrapping_sub(src),      // PSUBQ
        _ => {
            return Err(Error::Emulator(format!(
                "unimplemented PSUB opcode {:#x} at RIP={:#x}",
                opcode, vcpu.regs.rip
            )));
        }
    };

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn sub_u8_wrap(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as u8;
        let vb = ((b >> (i * 8)) & 0xFF) as u8;
        let diff = va.wrapping_sub(vb);
        result |= (diff as u64) << (i * 8);
    }
    result
}

fn sub_u16_wrap(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as u16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
        let diff = va.wrapping_sub(vb);
        result |= (diff as u64) << (i * 16);
    }
    result
}

fn sub_u32_wrap(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..2 {
        let va = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
        let vb = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32;
        let diff = va.wrapping_sub(vb);
        result |= (diff as u64) << (i * 32);
    }
    result
}

fn sub_u8_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as u8;
        let vb = ((b >> (i * 8)) & 0xFF) as u8;
        let diff = va.saturating_sub(vb);
        result |= (diff as u64) << (i * 8);
    }
    result
}

fn sub_u16_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as u16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
        let diff = va.saturating_sub(vb);
        result |= (diff as u64) << (i * 16);
    }
    result
}

fn sub_i8_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as i8;
        let vb = ((b >> (i * 8)) & 0xFF) as i8;
        let diff = va.saturating_sub(vb) as u8;
        result |= (diff as u64) << (i * 8);
    }
    result
}

fn sub_i16_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as i16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
        let diff = va.saturating_sub(vb) as u16;
        result |= (diff as u64) << (i * 16);
    }
    result
}

// =============================================================================
// Packed Integer Add (PADD* family)
// =============================================================================

/// PADDB - packed add bytes (0xFC)
pub fn paddb_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        // MMX version
        return paddb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::paddb_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_u8_wrap(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PADDW - packed add words (0xFD)
pub fn paddw_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::paddw_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_u16_wrap(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PADDD - packed add dwords (0xFE)
pub fn paddd_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddd_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::paddd_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddd_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_u32_wrap(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PADDQ - packed add qwords (0xD4)
pub fn paddq_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddq_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::paddq_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddq_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = dst.wrapping_add(src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn add_u8_wrap(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as u8;
        let vb = ((b >> (i * 8)) & 0xFF) as u8;
        let sum = va.wrapping_add(vb);
        result |= (sum as u64) << (i * 8);
    }
    result
}

fn add_u16_wrap(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as u16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
        let sum = va.wrapping_add(vb);
        result |= (sum as u64) << (i * 16);
    }
    result
}

fn add_u32_wrap(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..2 {
        let va = ((a >> (i * 32)) & 0xFFFF_FFFF) as u32;
        let vb = ((b >> (i * 32)) & 0xFFFF_FFFF) as u32;
        let sum = va.wrapping_add(vb);
        result |= (sum as u64) << (i * 32);
    }
    result
}

// =============================================================================
// Packed Integer Saturating Add (PADDS*/PADDUS* family)
// =============================================================================

/// PADDSB - packed add signed saturate bytes (0xEC)
pub fn paddsb_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddsb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = add_i8_saturate(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = add_i8_saturate(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddsb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_i8_saturate(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PADDSW - packed add signed saturate words (0xED)
pub fn paddsw_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddsw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = add_i16_saturate(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = add_i16_saturate(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddsw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_i16_saturate(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PADDUSB - packed add unsigned saturate bytes (0xDC)
pub fn paddusb_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddusb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = add_u8_saturate(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = add_u8_saturate(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddusb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_u8_saturate(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PADDUSW - packed add unsigned saturate words (0xDD)
pub fn paddusw_packed(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return paddusw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = add_u16_saturate(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = add_u16_saturate(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn paddusw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = add_u16_saturate(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn add_i8_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as i8;
        let vb = ((b >> (i * 8)) & 0xFF) as i8;
        let sum = va.saturating_add(vb) as u8;
        result |= (sum as u64) << (i * 8);
    }
    result
}

fn add_i16_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as i16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
        let sum = va.saturating_add(vb) as u16;
        result |= (sum as u64) << (i * 16);
    }
    result
}

fn add_u8_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as u8;
        let vb = ((b >> (i * 8)) & 0xFF) as u8;
        let sum = va.saturating_add(vb);
        result |= (sum as u64) << (i * 8);
    }
    result
}

fn add_u16_saturate(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as u16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as u16;
        let sum = va.saturating_add(vb);
        result |= (sum as u64) << (i * 16);
    }
    result
}

// =============================================================================
// Packed Integer Logical (PAND/POR/PXOR/PANDN)
// =============================================================================

/// PAND - packed logical AND (0xDB)
pub fn pand(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pand_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::pand_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pand_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    vcpu.regs.mm[mm_dst] &= src;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PANDN - packed logical AND NOT (0xDF)
pub fn pandn(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pandn_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::pandn_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pandn_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    vcpu.regs.mm[mm_dst] = !vcpu.regs.mm[mm_dst] & src;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POR - packed logical OR (0xEB)
pub fn por(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return por_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src: simd_native::Xmm = if is_memory {
        [vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?]
    } else {
        vcpu.regs.xmm[rm as usize]
    };

    simd_native::por_xmm(&mut vcpu.regs.xmm[xmm_dst], &src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn por_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    vcpu.regs.mm[mm_dst] |= src;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Packed Integer Compare (PCMPEQ*/PCMPGT*)
// =============================================================================

/// PCMPEQB - packed compare equal bytes (0x74)
pub fn pcmpeqb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pcmpeqb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = cmp_eq_bytes(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = cmp_eq_bytes(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pcmpeqb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = cmp_eq_bytes(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PCMPEQW - packed compare equal words (0x75)
pub fn pcmpeqw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pcmpeqw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = cmp_eq_words(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = cmp_eq_words(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pcmpeqw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = cmp_eq_words(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PCMPEQD - packed compare equal dwords (0x76)
pub fn pcmpeqd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pcmpeqd_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = cmp_eq_dwords(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = cmp_eq_dwords(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pcmpeqd_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = cmp_eq_dwords(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PCMPGTB - packed compare greater than bytes (0x64)
pub fn pcmpgtb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pcmpgtb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = cmp_gt_bytes(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = cmp_gt_bytes(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pcmpgtb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = cmp_gt_bytes(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PCMPGTW - packed compare greater than words (0x65)
pub fn pcmpgtw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pcmpgtw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = cmp_gt_words(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = cmp_gt_words(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pcmpgtw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = cmp_gt_words(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PCMPGTD - packed compare greater than dwords (0x66)
pub fn pcmpgtd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pcmpgtd_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    vcpu.regs.xmm[xmm_dst][0] = cmp_gt_dwords(dst_lo, src_lo);
    vcpu.regs.xmm[xmm_dst][1] = cmp_gt_dwords(dst_hi, src_hi);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pcmpgtd_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];
    vcpu.regs.mm[mm_dst] = cmp_gt_dwords(dst, src);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn cmp_eq_bytes(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = (a >> (i * 8)) & 0xFF;
        let vb = (b >> (i * 8)) & 0xFF;
        let mask = if va == vb { 0xFF } else { 0x00 };
        result |= mask << (i * 8);
    }
    result
}

fn cmp_eq_words(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = (a >> (i * 16)) & 0xFFFF;
        let vb = (b >> (i * 16)) & 0xFFFF;
        let mask = if va == vb { 0xFFFF } else { 0x0000 };
        result |= mask << (i * 16);
    }
    result
}

fn cmp_eq_dwords(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..2 {
        let va = (a >> (i * 32)) & 0xFFFF_FFFF;
        let vb = (b >> (i * 32)) & 0xFFFF_FFFF;
        let mask = if va == vb { 0xFFFF_FFFF } else { 0x0 };
        result |= mask << (i * 32);
    }
    result
}

fn cmp_gt_bytes(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        let va = ((a >> (i * 8)) & 0xFF) as i8;
        let vb = ((b >> (i * 8)) & 0xFF) as i8;
        let mask = if va > vb { 0xFF } else { 0x00 };
        result |= (mask as u64) << (i * 8);
    }
    result
}

fn cmp_gt_words(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let va = ((a >> (i * 16)) & 0xFFFF) as i16;
        let vb = ((b >> (i * 16)) & 0xFFFF) as i16;
        let mask = if va > vb { 0xFFFFu64 } else { 0x0 };
        result |= mask << (i * 16);
    }
    result
}

fn cmp_gt_dwords(a: u64, b: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..2 {
        let va = ((a >> (i * 32)) & 0xFFFF_FFFF) as i32;
        let vb = ((b >> (i * 32)) & 0xFFFF_FFFF) as i32;
        let mask = if va > vb { 0xFFFF_FFFFu64 } else { 0x0 };
        result |= mask << (i * 32);
    }
    result
}

// =============================================================================
// Non-Temporal Store (MOVNTQ)
// =============================================================================

/// MOVNTQ - non-temporal store MMX (0xE7)
pub fn movntq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, _rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if !is_memory {
        return Err(Error::Emulator(
            "MOVNTQ requires memory destination".to_string(),
        ));
    }
    let mm_src = (reg & 0x7) as usize;
    vcpu.write_mem(addr, vcpu.regs.mm[mm_src], 8)?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// Packed Integer Misc (PMADDWD/PMAX*/PMIN*/PMOVMSKB)
// =============================================================================

/// PMADDWD - Multiply and Add Packed Integers (0xF5)
pub fn pmaddwd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        // NP 0F F5 /r: MMX form, operates on 64-bit MM registers.
        return pmaddwd_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    let mut d_words = [0i16; 8];
    let mut s_words = [0i16; 8];
    for i in 0..4 {
        d_words[i] = ((dst_lo >> (i * 16)) & 0xFFFF) as i16;
        s_words[i] = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
    }
    for i in 0..4 {
        d_words[i + 4] = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
        s_words[i + 4] = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
    }

    let mut result_lo = 0u64;
    let mut result_hi = 0u64;
    for i in 0..4 {
        let a0 = d_words[i * 2] as i32;
        let a1 = d_words[i * 2 + 1] as i32;
        let b0 = s_words[i * 2] as i32;
        let b1 = s_words[i * 2 + 1] as i32;
        let sum = a0.wrapping_mul(b0).wrapping_add(a1.wrapping_mul(b1));
        let val = sum as u32 as u64;
        if i < 2 {
            result_lo |= val << (i * 32);
        } else {
            result_hi |= val << ((i - 2) * 32);
        }
    }

    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMADDWD MMX form (NP 0F F5 /r): mm, mm/m64.
/// Multiplies the four signed 16-bit words pairwise then adds adjacent products:
///   DEST[31:0]  := s0*d0 + s1*d1
///   DEST[63:32] := s2*d2 + s3*d3
fn pmaddwd_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut d_words = [0i16; 4];
    let mut s_words = [0i16; 4];
    for i in 0..4 {
        d_words[i] = ((dst >> (i * 16)) & 0xFFFF) as i16;
        s_words[i] = ((src >> (i * 16)) & 0xFFFF) as i16;
    }

    let mut result = 0u64;
    for i in 0..2 {
        let a0 = d_words[i * 2] as i32;
        let a1 = d_words[i * 2 + 1] as i32;
        let b0 = s_words[i * 2] as i32;
        let b1 = s_words[i * 2 + 1] as i32;
        let sum = a0.wrapping_mul(b0).wrapping_add(a1.wrapping_mul(b1));
        result |= (sum as u32 as u64) << (i * 32);
    }

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMAXSW - Maximum of Packed Signed Words (0xEE)
pub fn pmaxsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pmaxsw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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
        result_lo |= ((d.max(s) as u16) as u64) << (i * 16);
    }
    for i in 0..4 {
        let d = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
        let s = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
        result_hi |= ((d.max(s) as u16) as u64) << (i * 16);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pmaxsw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..4 {
        let d = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let s = ((src >> (i * 16)) & 0xFFFF) as i16;
        result |= ((d.max(s) as u16) as u64) << (i * 16);
    }
    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMINSW - Minimum of Packed Signed Words (0xEA)
pub fn pminsw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pminsw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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
        result_lo |= ((d.min(s) as u16) as u64) << (i * 16);
    }
    for i in 0..4 {
        let d = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
        let s = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
        result_hi |= ((d.min(s) as u16) as u64) << (i * 16);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pminsw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..4 {
        let d = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let s = ((src >> (i * 16)) & 0xFFFF) as i16;
        result |= ((d.min(s) as u16) as u64) << (i * 16);
    }
    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMAXUB - Maximum of Packed Unsigned Bytes (0xDE)
pub fn pmaxub(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pmaxub_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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
    for i in 0..8 {
        let d = ((dst_lo >> (i * 8)) & 0xFF) as u8;
        let s = ((src_lo >> (i * 8)) & 0xFF) as u8;
        result_lo |= (d.max(s) as u64) << (i * 8);
    }
    for i in 0..8 {
        let d = ((dst_hi >> (i * 8)) & 0xFF) as u8;
        let s = ((src_hi >> (i * 8)) & 0xFF) as u8;
        result_hi |= (d.max(s) as u64) << (i * 8);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pmaxub_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..8 {
        let d = ((dst >> (i * 8)) & 0xFF) as u8;
        let s = ((src >> (i * 8)) & 0xFF) as u8;
        result |= (d.max(s) as u64) << (i * 8);
    }
    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMINUB - Minimum of Packed Unsigned Bytes (0xDA)
pub fn pminub(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pminub_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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
    for i in 0..8 {
        let d = ((dst_lo >> (i * 8)) & 0xFF) as u8;
        let s = ((src_lo >> (i * 8)) & 0xFF) as u8;
        result_lo |= (d.min(s) as u64) << (i * 8);
    }
    for i in 0..8 {
        let d = ((dst_hi >> (i * 8)) & 0xFF) as u8;
        let s = ((src_hi >> (i * 8)) & 0xFF) as u8;
        result_hi |= (d.min(s) as u64) << (i * 8);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pminub_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..8 {
        let d = ((dst >> (i * 8)) & 0xFF) as u8;
        let s = ((src >> (i * 8)) & 0xFF) as u8;
        result |= (d.min(s) as u64) << (i * 8);
    }
    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVMSKB - Move Byte Mask (0xD7)
pub fn pmovmskb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator(format!(
            "PMOVMSKB requires 66 prefix at RIP={:#x}",
            vcpu.regs.rip
        )));
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let mut mask = 0u64;
    for i in 0..8 {
        let byte = ((src_lo >> (i * 8)) & 0xFF) as u8;
        if byte & 0x80 != 0 {
            mask |= 1u64 << i;
        }
    }
    for i in 0..8 {
        let byte = ((src_hi >> (i * 8)) & 0xFF) as u8;
        if byte & 0x80 != 0 {
            mask |= 1u64 << (i + 8);
        }
    }

    let dst_size = if ctx.rex_w() { 8 } else { 4 };
    vcpu.set_reg(reg, mask, dst_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// MOVLPS/MOVHPS - Move Low/High Packed Single-Precision
// =============================================================================

/// MOVLPS xmm, m64 / MOVHLPS xmm, xmm (0F 12)
pub fn movlps_load(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if is_memory {
        // MOVLPS: Load 64 bits from memory to low qword
        vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
    } else {
        // MOVHLPS: Move high qword to low qword
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][1];
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVLPS m64, xmm (0F 13)
pub fn movlps_store(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if is_memory {
        vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
    } else {
        return Err(Error::Emulator(format!(
            "MOVLPS store requires memory operand at RIP={:#x}",
            vcpu.regs.rip
        )));
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVHPS xmm, m64 / MOVLHPS xmm, xmm (0F 16)
pub fn movhps_load(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if is_memory {
        // MOVHPS: Load 64 bits from memory to high qword
        vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr, 8)?;
    } else {
        // MOVLHPS: Move low qword to high qword
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][0];
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVHPS m64, xmm (0F 17)
pub fn movhps_store(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if is_memory {
        vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][1], 8)?;
    } else {
        return Err(Error::Emulator(format!(
            "MOVHPS store requires memory operand at RIP={:#x}",
            vcpu.regs.rip
        )));
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// SSE3 replicate moves: MOVDDUP / MOVSLDUP / MOVSHDUP (legacy encodings)
// =============================================================================

/// MOVDDUP xmm1, xmm2/m64 (F2 0F 12)
/// Duplicates the low 64 bits of the source into both lanes of the destination:
/// dst.lo = dst.hi = src.lo. The memory form loads 64 bits; the register form
/// uses the source xmm low 64 bits.
pub fn movddup(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    let src_lo = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };

    vcpu.regs.xmm[xmm_dst][0] = src_lo;
    vcpu.regs.xmm[xmm_dst][1] = src_lo;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVSLDUP xmm1, xmm2/m128 (F3 0F 12)
/// Duplicates the even-indexed (low) 32-bit floats:
/// r0=s0, r1=s0, r2=s2, r3=s2.
pub fn movsldup(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let s0 = src_lo & 0xFFFF_FFFF;
    let s2 = src_hi & 0xFFFF_FFFF;
    vcpu.regs.xmm[xmm_dst][0] = s0 | (s0 << 32);
    vcpu.regs.xmm[xmm_dst][1] = s2 | (s2 << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVSHDUP xmm1, xmm2/m128 (F3 0F 16)
/// Duplicates the odd-indexed (high) 32-bit floats:
/// r0=s1, r1=s1, r2=s3, r3=s3.
pub fn movshdup(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    let s1 = (src_lo >> 32) & 0xFFFF_FFFF;
    let s3 = (src_hi >> 32) & 0xFFFF_FFFF;
    vcpu.regs.xmm[xmm_dst][0] = s1 | (s1 << 32);
    vcpu.regs.xmm[xmm_dst][1] = s3 | (s3 << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// MOVNTI - Non-Temporal Store (0F C3)
// =============================================================================

/// MOVNTI m32/m64, r32/r64 (0F C3)
pub fn movnti(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, _rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if !is_memory {
        return Err(Error::Emulator(format!(
            "MOVNTI requires memory operand at RIP={:#x}",
            vcpu.regs.rip
        )));
    }
    let size = if ctx.rex_w() { 8 } else { 4 };
    let value = vcpu.get_reg(reg, size);
    vcpu.write_mem(addr, value, size)?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// LDDQU - Load Unaligned Integer 128 Bits (F2 0F F0)
// =============================================================================

/// LDDQU xmm, m128 (F2 0F F0)
pub fn lddqu(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, _rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if !is_memory {
        return Err(Error::Emulator(format!(
            "LDDQU requires memory operand at RIP={:#x}",
            vcpu.regs.rip
        )));
    }
    let xmm_dst = reg as usize;
    vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
    vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PSADBW - Sum of Absolute Differences (0F F6)
// =============================================================================

/// PSADBW xmm, xmm/m128 (66 0F F6) or mm, mm/m64 (0F F6)
pub fn psadbw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return psadbw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    // Calculate SAD for low and high 64-bit halves
    let mut sad_low = 0u64;
    let mut sad_high = 0u64;

    for i in 0..8 {
        let d = ((dst_lo >> (i * 8)) & 0xFF) as i16;
        let s = ((src_lo >> (i * 8)) & 0xFF) as i16;
        sad_low += (d - s).unsigned_abs() as u64;
    }
    for i in 0..8 {
        let d = ((dst_hi >> (i * 8)) & 0xFF) as i16;
        let s = ((src_hi >> (i * 8)) & 0xFF) as i16;
        sad_high += (d - s).unsigned_abs() as u64;
    }

    // Result: low 16 bits of each qword contain the SAD, upper 48 bits are zero
    vcpu.regs.xmm[xmm_dst][0] = sad_low & 0xFFFF;
    vcpu.regs.xmm[xmm_dst][1] = sad_high & 0xFFFF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn psadbw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut sad = 0u64;
    for i in 0..8 {
        let d = ((dst >> (i * 8)) & 0xFF) as i16;
        let s = ((src >> (i * 8)) & 0xFF) as i16;
        sad += (d - s).unsigned_abs() as u64;
    }

    vcpu.regs.mm[mm_dst] = sad & 0xFFFF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// MASKMOVDQU - Store Selected Bytes of Double Quadword (66 0F F7)
// =============================================================================

/// MASKMOVDQU xmm1, xmm2 (66 0F F7) - stores bytes from xmm1 to DS:RDI based on mask in xmm2
pub fn maskmovdqu(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        // MASKMOVQ - MMX version
        return maskmovq(vcpu, ctx);
    }

    let (reg, rm, is_memory, _, _) = vcpu.decode_modrm(ctx)?;
    if is_memory {
        return Err(Error::Emulator(format!(
            "MASKMOVDQU requires register operands at RIP={:#x}",
            vcpu.regs.rip
        )));
    }

    let xmm_src = reg as usize;
    let xmm_mask = rm as usize;
    let addr = vcpu.regs.rdi;

    let src_lo = vcpu.regs.xmm[xmm_src][0];
    let src_hi = vcpu.regs.xmm[xmm_src][1];
    let mask_lo = vcpu.regs.xmm[xmm_mask][0];
    let mask_hi = vcpu.regs.xmm[xmm_mask][1];

    // Write bytes where mask high bit is set
    for i in 0..8 {
        if (mask_lo >> (i * 8 + 7)) & 1 != 0 {
            let byte = ((src_lo >> (i * 8)) & 0xFF) as u64;
            vcpu.write_mem(addr + i as u64, byte, 1)?;
        }
    }
    for i in 0..8 {
        if (mask_hi >> (i * 8 + 7)) & 1 != 0 {
            let byte = ((src_hi >> (i * 8)) & 0xFF) as u64;
            vcpu.write_mem(addr + 8 + i as u64, byte, 1)?;
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MASKMOVQ mm1, mm2 (0F F7)
fn maskmovq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, _, _) = vcpu.decode_modrm(ctx)?;
    if is_memory {
        return Err(Error::Emulator(format!(
            "MASKMOVQ requires register operands at RIP={:#x}",
            vcpu.regs.rip
        )));
    }

    let mm_src = (reg & 0x7) as usize;
    let mm_mask = (rm & 0x7) as usize;
    let addr = vcpu.regs.rdi;

    let src = vcpu.regs.mm[mm_src];
    let mask = vcpu.regs.mm[mm_mask];

    for i in 0..8 {
        if (mask >> (i * 8 + 7)) & 1 != 0 {
            let byte = ((src >> (i * 8)) & 0xFF) as u64;
            vcpu.write_mem(addr + i as u64, byte, 1)?;
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PAVGB/PAVGW - Packed Average (0F E0/E3)
// =============================================================================

/// PAVGB xmm, xmm/m128 (66 0F E0) or mm, mm/m64 (0F E0)
pub fn pavgb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pavgb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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
    for i in 0..8 {
        let d = ((dst_lo >> (i * 8)) & 0xFF) as u16;
        let s = ((src_lo >> (i * 8)) & 0xFF) as u16;
        let avg = ((d + s + 1) >> 1) as u64;
        result_lo |= avg << (i * 8);
    }
    for i in 0..8 {
        let d = ((dst_hi >> (i * 8)) & 0xFF) as u16;
        let s = ((src_hi >> (i * 8)) & 0xFF) as u16;
        let avg = ((d + s + 1) >> 1) as u64;
        result_hi |= avg << (i * 8);
    }

    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pavgb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..8 {
        let d = ((dst >> (i * 8)) & 0xFF) as u16;
        let s = ((src >> (i * 8)) & 0xFF) as u16;
        let avg = ((d + s + 1) >> 1) as u64;
        result |= avg << (i * 8);
    }

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PAVGW xmm, xmm/m128 (66 0F E3) or mm, mm/m64 (0F E3)
pub fn pavgw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return pavgw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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
        let d = ((dst_lo >> (i * 16)) & 0xFFFF) as u32;
        let s = ((src_lo >> (i * 16)) & 0xFFFF) as u32;
        let avg = ((d + s + 1) >> 1) as u64;
        result_lo |= avg << (i * 16);
    }
    for i in 0..4 {
        let d = ((dst_hi >> (i * 16)) & 0xFFFF) as u32;
        let s = ((src_hi >> (i * 16)) & 0xFFFF) as u32;
        let avg = ((d + s + 1) >> 1) as u64;
        result_hi |= avg << (i * 16);
    }

    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn pavgw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..4 {
        let d = ((dst >> (i * 16)) & 0xFFFF) as u32;
        let s = ((src >> (i * 16)) & 0xFFFF) as u32;
        let avg = ((d + s + 1) >> 1) as u64;
        result |= avg << (i * 16);
    }

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PACKSSWB/PACKSSDW - Pack with Signed Saturation (0F 63/6B)
// =============================================================================

/// PACKSSWB xmm, xmm/m128 (66 0F 63) or mm, mm/m64 (0F 63)
pub fn packsswb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return packsswb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    // Pack 8 words from dest and 8 words from src into 16 bytes
    let mut result_lo = 0u64;
    let mut result_hi = 0u64;

    // First 8 bytes from dst (words -> bytes with signed saturation)
    for i in 0..4 {
        let w = ((dst_lo >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result_lo |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result_lo |= (b as u64) << ((i + 4) * 8);
    }

    // Next 8 bytes from src (words -> bytes with signed saturation)
    for i in 0..4 {
        let w = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result_hi |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result_hi |= (b as u64) << ((i + 4) * 8);
    }

    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn packsswb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    // First 4 bytes from dst
    for i in 0..4 {
        let w = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result |= (b as u64) << (i * 8);
    }
    // Next 4 bytes from src
    for i in 0..4 {
        let w = ((src >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(-128, 127) as i8 as u8;
        result |= (b as u64) << ((i + 4) * 8);
    }

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PACKSSDW xmm, xmm/m128 (66 0F 6B) or mm, mm/m64 (0F 6B)
pub fn packssdw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return packssdw_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    // Pack 4 dwords from dest and 4 dwords from src into 8 words
    let mut result_lo = 0u64;
    let mut result_hi = 0u64;

    // First 4 words from dst (dwords -> words with signed saturation)
    for i in 0..2 {
        let d = ((dst_lo >> (i * 32)) & 0xFFFFFFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result_lo |= (w as u64) << (i * 16);
    }
    for i in 0..2 {
        let d = ((dst_hi >> (i * 32)) & 0xFFFFFFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result_lo |= (w as u64) << ((i + 2) * 16);
    }

    // Next 4 words from src (dwords -> words with signed saturation)
    for i in 0..2 {
        let d = ((src_lo >> (i * 32)) & 0xFFFFFFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result_hi |= (w as u64) << (i * 16);
    }
    for i in 0..2 {
        let d = ((src_hi >> (i * 32)) & 0xFFFFFFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result_hi |= (w as u64) << ((i + 2) * 16);
    }

    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn packssdw_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    // First 2 words from dst
    for i in 0..2 {
        let d = ((dst >> (i * 32)) & 0xFFFFFFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result |= (w as u64) << (i * 16);
    }
    // Next 2 words from src
    for i in 0..2 {
        let d = ((src >> (i * 32)) & 0xFFFFFFFF) as i32;
        let w = d.clamp(-32768, 32767) as i16 as u16;
        result |= (w as u64) << ((i + 2) * 16);
    }

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// PACKUSWB - Pack with Unsigned Saturation (0F 67)
// =============================================================================

/// PACKUSWB xmm, xmm/m128 (66 0F 67) or mm, mm/m64 (0F 67)
pub fn packuswb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return packuswb_mmx(vcpu, ctx);
    }

    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
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

    // First 8 bytes from dst (signed words -> unsigned bytes with saturation)
    for i in 0..4 {
        let w = ((dst_lo >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result_lo |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((dst_hi >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result_lo |= (b as u64) << ((i + 4) * 8);
    }

    // Next 8 bytes from src
    for i in 0..4 {
        let w = ((src_lo >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result_hi |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((src_hi >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result_hi |= (b as u64) << ((i + 4) * 8);
    }

    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn packuswb_mmx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x7) as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.mm[(rm & 0x7) as usize]
    };
    let dst = vcpu.regs.mm[mm_dst];

    let mut result = 0u64;
    for i in 0..4 {
        let w = ((dst >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result |= (b as u64) << (i * 8);
    }
    for i in 0..4 {
        let w = ((src >> (i * 16)) & 0xFFFF) as i16;
        let b = w.clamp(0, 255) as u8;
        result |= (b as u64) << ((i + 4) * 8);
    }

    vcpu.regs.mm[mm_dst] = result;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// MOVQ2DQ/MOVDQ2Q - MMX/XMM conversion (F3/F2 0F D6)
// =============================================================================

/// MOVQ2DQ xmm, mm (F3 0F D6) - Move quadword from MMX to XMM
pub fn movq2dq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, _, _) = vcpu.decode_modrm(ctx)?;
    if is_memory {
        return Err(Error::Emulator(format!(
            "MOVQ2DQ requires register operands at RIP={:#x}",
            vcpu.regs.rip
        )));
    }
    let xmm_dst = reg as usize;
    let mm_src = (rm & 0x7) as usize;
    vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.mm[mm_src];
    vcpu.regs.xmm[xmm_dst][1] = 0;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVDQ2Q mm, xmm (F2 0F D6) - Move low quadword from XMM to MMX
pub fn movdq2q(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, _, _) = vcpu.decode_modrm(ctx)?;
    if is_memory {
        return Err(Error::Emulator(format!(
            "MOVDQ2Q requires register operands at RIP={:#x}",
            vcpu.regs.rip
        )));
    }
    let mm_dst = (reg & 0x7) as usize;
    let xmm_src = rm as usize;
    vcpu.regs.mm[mm_dst] = vcpu.regs.xmm[xmm_src][0];
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Packed integer shift by XMM count
/// PSRLW/D/Q, PSRAW/D, PSLLW/D/Q xmm, xmm/m128
/// Shift count is taken from low 64 bits of xmm/m128
pub fn packed_shift_xmm_count(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // For SSE2 with 66 prefix, use XMM registers
    // For MMX (no prefix), use MM registers
    let use_xmm = ctx.operand_size_override;

    // Get shift count from low 64 bits of source
    let count = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else if use_xmm {
        vcpu.regs.xmm[rm as usize][0]
    } else {
        vcpu.regs.mm[rm as usize & 0x7]
    };

    if use_xmm {
        let xmm_dst = reg as usize;
        let src_lo = vcpu.regs.xmm[xmm_dst][0];
        let src_hi = vcpu.regs.xmm[xmm_dst][1];

        let (result_lo, result_hi) = match opcode {
            // PSRLW: logical right shift words
            0xD1 => {
                if count >= 16 {
                    (0, 0)
                } else {
                    (
                        shift_right_words(src_lo, count as u32),
                        shift_right_words(src_hi, count as u32),
                    )
                }
            }
            // PSRLD: logical right shift dwords
            0xD2 => {
                if count >= 32 {
                    (0, 0)
                } else {
                    (
                        shift_right_dwords(src_lo, count as u32),
                        shift_right_dwords(src_hi, count as u32),
                    )
                }
            }
            // PSRLQ: logical right shift qwords
            0xD3 => {
                if count >= 64 {
                    (0, 0)
                } else {
                    (src_lo >> count, src_hi >> count)
                }
            }
            // PSRAW: arithmetic right shift words
            0xE1 => {
                let shift = count.min(15) as u32;
                (
                    shift_right_arith_words(src_lo, shift),
                    shift_right_arith_words(src_hi, shift),
                )
            }
            // PSRAD: arithmetic right shift dwords
            0xE2 => {
                let shift = count.min(31) as u32;
                (
                    shift_right_arith_dwords(src_lo, shift),
                    shift_right_arith_dwords(src_hi, shift),
                )
            }
            // PSLLW: logical left shift words
            0xF1 => {
                if count >= 16 {
                    (0, 0)
                } else {
                    (
                        shift_left_words(src_lo, count as u32),
                        shift_left_words(src_hi, count as u32),
                    )
                }
            }
            // PSLLD: logical left shift dwords
            0xF2 => {
                if count >= 32 {
                    (0, 0)
                } else {
                    (
                        shift_left_dwords(src_lo, count as u32),
                        shift_left_dwords(src_hi, count as u32),
                    )
                }
            }
            // PSLLQ: logical left shift qwords
            0xF3 => {
                if count >= 64 {
                    (0, 0)
                } else {
                    (src_lo << count, src_hi << count)
                }
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "Unknown packed shift opcode {:#04x}",
                    opcode
                )));
            }
        };

        vcpu.regs.xmm[xmm_dst][0] = result_lo;
        vcpu.regs.xmm[xmm_dst][1] = result_hi;
    } else {
        // MMX version
        let mm_dst = (reg & 0x7) as usize;
        let src = vcpu.regs.mm[mm_dst];

        let result = match opcode {
            0xD1 => {
                if count >= 16 {
                    0
                } else {
                    shift_right_words(src, count as u32)
                }
            }
            0xD2 => {
                if count >= 32 {
                    0
                } else {
                    shift_right_dwords(src, count as u32)
                }
            }
            0xD3 => {
                if count >= 64 {
                    0
                } else {
                    src >> count
                }
            }
            0xE1 => {
                let shift = count.min(15) as u32;
                shift_right_arith_words(src, shift)
            }
            0xE2 => {
                let shift = count.min(31) as u32;
                shift_right_arith_dwords(src, shift)
            }
            0xF1 => {
                if count >= 16 {
                    0
                } else {
                    shift_left_words(src, count as u32)
                }
            }
            0xF2 => {
                if count >= 32 {
                    0
                } else {
                    shift_left_dwords(src, count as u32)
                }
            }
            0xF3 => {
                if count >= 64 {
                    0
                } else {
                    src << count
                }
            }
            _ => {
                return Err(Error::Emulator(format!(
                    "Unknown packed shift opcode {:#04x}",
                    opcode
                )));
            }
        };

        vcpu.regs.mm[mm_dst] = result;
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// Shift helper functions for packed operations
fn shift_right_words(v: u64, shift: u32) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((v >> (i * 16)) & 0xFFFF) as u16;
        let shifted = w >> shift;
        result |= (shifted as u64) << (i * 16);
    }
    result
}

fn shift_left_words(v: u64, shift: u32) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((v >> (i * 16)) & 0xFFFF) as u16;
        let shifted = w << shift;
        result |= (shifted as u64) << (i * 16);
    }
    result
}

fn shift_right_arith_words(v: u64, shift: u32) -> u64 {
    let mut result = 0u64;
    for i in 0..4 {
        let w = ((v >> (i * 16)) & 0xFFFF) as i16;
        let shifted = (w >> shift) as u16;
        result |= (shifted as u64) << (i * 16);
    }
    result
}

fn shift_right_dwords(v: u64, shift: u32) -> u64 {
    let d0 = (v as u32) >> shift;
    let d1 = ((v >> 32) as u32) >> shift;
    (d0 as u64) | ((d1 as u64) << 32)
}

fn shift_left_dwords(v: u64, shift: u32) -> u64 {
    let d0 = (v as u32) << shift;
    let d1 = ((v >> 32) as u32) << shift;
    (d0 as u64) | ((d1 as u64) << 32)
}

fn shift_right_arith_dwords(v: u64, shift: u32) -> u64 {
    let d0 = ((v as u32) as i32 >> shift) as u32;
    let d1 = (((v >> 32) as u32) as i32 >> shift) as u32;
    (d0 as u64) | ((d1 as u64) << 32)
}

/// EMMS - Empty MMX Technology State (0F 77)
/// Marks all x87 FPU tag words as empty (valid for x87 operations)
pub fn emms(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Set all FPU tag words to empty (11b)
    // This indicates that the corresponding x87 FPU data register is empty
    vcpu.fpu.tag_word = 0xFFFF; // All 8 tags set to 11b (empty)
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
