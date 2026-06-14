//! SSE4.1 and SSE4.2 instruction implementations.
//!
//! SSE4.1 (0x0F 0x38): PBLENDVB, BLENDVPS, BLENDVPD, PTEST, PMOVSXBW, etc.
//! SSE4.1 (0x0F 0x3A): ROUNDPS, ROUNDPD, BLENDPS, BLENDPD, PEXTR*, PINSR*, etc.
//! SSE4.2 (0x0F 0x38): PCMPGTQ
//! SSE4.2 (0x0F 0x3A): PCMP*STR* string comparison

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

// =============================================================================
// SSE4.1 Blend Instructions (0x0F 0x38)
// =============================================================================

/// PBLENDVB - Variable Blend Packed Bytes (0x10)
pub fn pblendvb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PBLENDVB requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    // Mask is implicitly XMM0
    let mask_lo = vcpu.regs.xmm[0][0];
    let mask_hi = vcpu.regs.xmm[0][1];
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    let mut result_lo = 0u64;
    let mut result_hi = 0u64;
    for i in 0..8 {
        let m = ((mask_lo >> (i * 8)) & 0x80) != 0;
        let s = (src_lo >> (i * 8)) & 0xFF;
        let d = (dst_lo >> (i * 8)) & 0xFF;
        result_lo |= (if m { s } else { d }) << (i * 8);
    }
    for i in 0..8 {
        let m = ((mask_hi >> (i * 8)) & 0x80) != 0;
        let s = (src_hi >> (i * 8)) & 0xFF;
        let d = (dst_hi >> (i * 8)) & 0xFF;
        result_hi |= (if m { s } else { d }) << (i * 8);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// BLENDVPS - Variable Blend Packed Single-FP (0x14)
pub fn blendvps(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("BLENDVPS requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let mask_lo = vcpu.regs.xmm[0][0];
    let mask_hi = vcpu.regs.xmm[0][1];
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    // Blend based on high bit of each dword in mask
    let m0 = (mask_lo & 0x80000000) != 0;
    let m1 = (mask_lo & 0x8000000000000000) != 0;
    let m2 = (mask_hi & 0x80000000) != 0;
    let m3 = (mask_hi & 0x8000000000000000) != 0;

    let s0 = src_lo & 0xFFFFFFFF;
    let s1 = src_lo >> 32;
    let s2 = src_hi & 0xFFFFFFFF;
    let s3 = src_hi >> 32;
    let d0 = dst_lo & 0xFFFFFFFF;
    let d1 = dst_lo >> 32;
    let d2 = dst_hi & 0xFFFFFFFF;
    let d3 = dst_hi >> 32;

    vcpu.regs.xmm[xmm_dst][0] = (if m0 { s0 } else { d0 }) | ((if m1 { s1 } else { d1 }) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (if m2 { s2 } else { d2 }) | ((if m3 { s3 } else { d3 }) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// BLENDVPD - Variable Blend Packed Double-FP (0x15)
pub fn blendvpd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("BLENDVPD requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };
    let mask_lo = vcpu.regs.xmm[0][0];
    let mask_hi = vcpu.regs.xmm[0][1];
    let dst_lo = vcpu.regs.xmm[xmm_dst][0];
    let dst_hi = vcpu.regs.xmm[xmm_dst][1];

    let m0 = (mask_lo & 0x8000000000000000) != 0;
    let m1 = (mask_hi & 0x8000000000000000) != 0;

    vcpu.regs.xmm[xmm_dst][0] = if m0 { src_lo } else { dst_lo };
    vcpu.regs.xmm[xmm_dst][1] = if m1 { src_hi } else { dst_hi };
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PTEST - Logical Compare (0x17)
pub fn ptest(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PTEST requires 66 prefix".to_string()));
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

    // ZF = (src AND dst) == 0
    let and_result = (src_lo & dst_lo) | (src_hi & dst_hi);
    // CF = (src AND NOT dst) == 0
    let andn_result = (src_lo & !dst_lo) | (src_hi & !dst_hi);

    // Clear AF, OF, PF, SF and set ZF, CF appropriately
    vcpu.regs.rflags &= !(flags::bits::AF
        | flags::bits::OF
        | flags::bits::PF
        | flags::bits::SF
        | flags::bits::ZF
        | flags::bits::CF);
    if and_result == 0 {
        vcpu.regs.rflags |= flags::bits::ZF;
    }
    if andn_result == 0 {
        vcpu.regs.rflags |= flags::bits::CF;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// SSE4.1 Sign/Zero Extend Instructions (0x0F 0x38)
// =============================================================================

/// PMOVSXBW - Packed Move with Sign Extend Byte to Word (0x20)
pub fn pmovsxbw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVSXBW requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    // Sign extend 8 bytes to 8 words
    let mut result_lo = 0u64;
    let mut result_hi = 0u64;
    for i in 0..4 {
        let b = ((src >> (i * 8)) & 0xFF) as i8;
        result_lo |= ((b as i16 as u16) as u64) << (i * 16);
    }
    for i in 0..4 {
        let b = ((src >> ((i + 4) * 8)) & 0xFF) as i8;
        result_hi |= ((b as i16 as u16) as u64) << (i * 16);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVSXBD - Packed Move with Sign Extend Byte to Dword (0x21)
pub fn pmovsxbd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVSXBD requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 4)?
    } else {
        vcpu.regs.xmm[rm as usize][0] & 0xFFFFFFFF
    };
    let b0 = (src & 0xFF) as i8 as i32 as u32;
    let b1 = ((src >> 8) & 0xFF) as i8 as i32 as u32;
    let b2 = ((src >> 16) & 0xFF) as i8 as i32 as u32;
    let b3 = ((src >> 24) & 0xFF) as i8 as i32 as u32;
    vcpu.regs.xmm[xmm_dst][0] = (b0 as u64) | ((b1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (b2 as u64) | ((b3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVSXBQ - Packed Move with Sign Extend Byte to Qword (0x22)
pub fn pmovsxbq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVSXBQ requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 2)?
    } else {
        vcpu.regs.xmm[rm as usize][0] & 0xFFFF
    };
    let b0 = (src & 0xFF) as i8 as i64 as u64;
    let b1 = ((src >> 8) & 0xFF) as i8 as i64 as u64;
    vcpu.regs.xmm[xmm_dst][0] = b0;
    vcpu.regs.xmm[xmm_dst][1] = b1;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVSXWD - Packed Move with Sign Extend Word to Dword (0x23)
pub fn pmovsxwd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVSXWD requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    let w0 = (src & 0xFFFF) as i16 as i32 as u32;
    let w1 = ((src >> 16) & 0xFFFF) as i16 as i32 as u32;
    let w2 = ((src >> 32) & 0xFFFF) as i16 as i32 as u32;
    let w3 = ((src >> 48) & 0xFFFF) as i16 as i32 as u32;
    vcpu.regs.xmm[xmm_dst][0] = (w0 as u64) | ((w1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (w2 as u64) | ((w3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVSXWQ - Packed Move with Sign Extend Word to Qword (0x24)
pub fn pmovsxwq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVSXWQ requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 4)?
    } else {
        vcpu.regs.xmm[rm as usize][0] & 0xFFFFFFFF
    };
    let w0 = (src & 0xFFFF) as i16 as i64 as u64;
    let w1 = ((src >> 16) & 0xFFFF) as i16 as i64 as u64;
    vcpu.regs.xmm[xmm_dst][0] = w0;
    vcpu.regs.xmm[xmm_dst][1] = w1;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVSXDQ - Packed Move with Sign Extend Dword to Qword (0x25)
pub fn pmovsxdq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVSXDQ requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    let d0 = (src as u32) as i32 as i64 as u64;
    let d1 = ((src >> 32) as u32) as i32 as i64 as u64;
    vcpu.regs.xmm[xmm_dst][0] = d0;
    vcpu.regs.xmm[xmm_dst][1] = d1;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVZXBW - Packed Move with Zero Extend Byte to Word (0x30)
pub fn pmovzxbw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVZXBW requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    let mut result_lo = 0u64;
    let mut result_hi = 0u64;
    for i in 0..4 {
        let b = (src >> (i * 8)) & 0xFF;
        result_lo |= b << (i * 16);
    }
    for i in 0..4 {
        let b = (src >> ((i + 4) * 8)) & 0xFF;
        result_hi |= b << (i * 16);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVZXBD - Packed Move with Zero Extend Byte to Dword (0x31)
pub fn pmovzxbd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVZXBD requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 4)?
    } else {
        vcpu.regs.xmm[rm as usize][0] & 0xFFFFFFFF
    };
    let b0 = src & 0xFF;
    let b1 = (src >> 8) & 0xFF;
    let b2 = (src >> 16) & 0xFF;
    let b3 = (src >> 24) & 0xFF;
    vcpu.regs.xmm[xmm_dst][0] = b0 | (b1 << 32);
    vcpu.regs.xmm[xmm_dst][1] = b2 | (b3 << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVZXBQ - Packed Move with Zero Extend Byte to Qword (0x32)
pub fn pmovzxbq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVZXBQ requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 2)?
    } else {
        vcpu.regs.xmm[rm as usize][0] & 0xFFFF
    };
    vcpu.regs.xmm[xmm_dst][0] = src & 0xFF;
    vcpu.regs.xmm[xmm_dst][1] = (src >> 8) & 0xFF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVZXWD - Packed Move with Zero Extend Word to Dword (0x33)
pub fn pmovzxwd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVZXWD requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    let w0 = src & 0xFFFF;
    let w1 = (src >> 16) & 0xFFFF;
    let w2 = (src >> 32) & 0xFFFF;
    let w3 = (src >> 48) & 0xFFFF;
    vcpu.regs.xmm[xmm_dst][0] = w0 | (w1 << 32);
    vcpu.regs.xmm[xmm_dst][1] = w2 | (w3 << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVZXWQ - Packed Move with Zero Extend Word to Qword (0x34)
pub fn pmovzxwq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVZXWQ requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 4)?
    } else {
        vcpu.regs.xmm[rm as usize][0] & 0xFFFFFFFF
    };
    vcpu.regs.xmm[xmm_dst][0] = src & 0xFFFF;
    vcpu.regs.xmm[xmm_dst][1] = (src >> 16) & 0xFFFF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMOVZXDQ - Packed Move with Zero Extend Dword to Qword (0x35)
pub fn pmovzxdq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMOVZXDQ requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let src = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    vcpu.regs.xmm[xmm_dst][0] = src & 0xFFFFFFFF;
    vcpu.regs.xmm[xmm_dst][1] = (src >> 32) & 0xFFFFFFFF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// SSE4.1 Multiply/Compare Instructions (0x0F 0x38)
// =============================================================================

/// PMULDQ - Multiply Packed Signed Dword (0x28)
pub fn pmuldq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMULDQ requires 66 prefix".to_string()));
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
    // Multiply low dwords of each qword, result is 64-bit
    let r0 = (dst_lo as i32 as i64) * (src_lo as i32 as i64);
    let r1 = (dst_hi as i32 as i64) * (src_hi as i32 as i64);
    vcpu.regs.xmm[xmm_dst][0] = r0 as u64;
    vcpu.regs.xmm[xmm_dst][1] = r1 as u64;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PCMPEQQ - Compare Packed Qword for Equal (0x29)
pub fn pcmpeqq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PCMPEQQ requires 66 prefix".to_string()));
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
    vcpu.regs.xmm[xmm_dst][0] = if dst_lo == src_lo { u64::MAX } else { 0 };
    vcpu.regs.xmm[xmm_dst][1] = if dst_hi == src_hi { u64::MAX } else { 0 };
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVNTDQA - Load Double Quadword Non-Temporal (0x2A)
pub fn movntdqa(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("MOVNTDQA requires 66 prefix".to_string()));
    }
    let (reg, _rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    if !is_memory {
        return Err(Error::Emulator(
            "MOVNTDQA requires memory operand".to_string(),
        ));
    }
    let xmm_dst = reg as usize;
    // Non-temporal hint ignored in emulation, just load normally
    vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
    vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PACKUSDW - Pack with Unsigned Saturation (0x2B)
pub fn packusdw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PACKUSDW requires 66 prefix".to_string()));
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

    fn sat_u16(v: i32) -> u16 {
        v.clamp(0, 65535) as u16
    }
    let d = [
        sat_u16(dst_lo as i32),
        sat_u16((dst_lo >> 32) as i32),
        sat_u16(dst_hi as i32),
        sat_u16((dst_hi >> 32) as i32),
        sat_u16(src_lo as i32),
        sat_u16((src_lo >> 32) as i32),
        sat_u16(src_hi as i32),
        sat_u16((src_hi >> 32) as i32),
    ];
    vcpu.regs.xmm[xmm_dst][0] =
        (d[0] as u64) | ((d[1] as u64) << 16) | ((d[2] as u64) << 32) | ((d[3] as u64) << 48);
    vcpu.regs.xmm[xmm_dst][1] =
        (d[4] as u64) | ((d[5] as u64) << 16) | ((d[6] as u64) << 32) | ((d[7] as u64) << 48);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMULLD - Multiply Packed Signed Dword (0x40)
pub fn pmulld(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMULLD requires 66 prefix".to_string()));
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

    let r0 = (dst_lo as i32).wrapping_mul(src_lo as i32) as u32;
    let r1 = ((dst_lo >> 32) as i32).wrapping_mul((src_lo >> 32) as i32) as u32;
    let r2 = (dst_hi as i32).wrapping_mul(src_hi as i32) as u32;
    let r3 = ((dst_hi >> 32) as i32).wrapping_mul((src_hi >> 32) as i32) as u32;

    vcpu.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PHMINPOSUW - Packed Horizontal Word Minimum (0x41)
pub fn phminposuw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PHMINPOSUW requires 66 prefix".to_string()));
    }
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let (src_lo, src_hi) = if is_memory {
        (vcpu.read_mem(addr, 8)?, vcpu.read_mem(addr + 8, 8)?)
    } else {
        (vcpu.regs.xmm[rm as usize][0], vcpu.regs.xmm[rm as usize][1])
    };

    // Find minimum unsigned word and its index
    let words = [
        (src_lo & 0xFFFF) as u16,
        ((src_lo >> 16) & 0xFFFF) as u16,
        ((src_lo >> 32) & 0xFFFF) as u16,
        ((src_lo >> 48) & 0xFFFF) as u16,
        (src_hi & 0xFFFF) as u16,
        ((src_hi >> 16) & 0xFFFF) as u16,
        ((src_hi >> 32) & 0xFFFF) as u16,
        ((src_hi >> 48) & 0xFFFF) as u16,
    ];

    let mut min_val = words[0];
    let mut min_idx = 0u16;
    for i in 1..8 {
        if words[i] < min_val {
            min_val = words[i];
            min_idx = i as u16;
        }
    }

    // Result: low word = min value, word 1 = index, rest = 0
    vcpu.regs.xmm[xmm_dst][0] = (min_val as u64) | ((min_idx as u64) << 16);
    vcpu.regs.xmm[xmm_dst][1] = 0;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// SSE4.2 Compare Instructions (0x0F 0x38)
// =============================================================================

/// PCMPGTQ - Compare Packed Qword for Greater Than (0x37)
pub fn pcmpgtq(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PCMPGTQ requires 66 prefix".to_string()));
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
    vcpu.regs.xmm[xmm_dst][0] = if (dst_lo as i64) > (src_lo as i64) {
        u64::MAX
    } else {
        0
    };
    vcpu.regs.xmm[xmm_dst][1] = if (dst_hi as i64) > (src_hi as i64) {
        u64::MAX
    } else {
        0
    };
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// SSE4.1/4.2 Min/Max Instructions (0x0F 0x38)
// =============================================================================

/// PMINSB - Minimum of Packed Signed Bytes (0x38)
pub fn pminsb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMINSB requires 66 prefix".to_string()));
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
        let d = ((dst_lo >> (i * 8)) & 0xFF) as i8;
        let s = ((src_lo >> (i * 8)) & 0xFF) as i8;
        result_lo |= ((d.min(s) as u8) as u64) << (i * 8);
    }
    for i in 0..8 {
        let d = ((dst_hi >> (i * 8)) & 0xFF) as i8;
        let s = ((src_hi >> (i * 8)) & 0xFF) as i8;
        result_hi |= ((d.min(s) as u8) as u64) << (i * 8);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMINSD - Minimum of Packed Signed Dwords (0x39)
pub fn pminsd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMINSD requires 66 prefix".to_string()));
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

    let r0 = (dst_lo as i32).min(src_lo as i32) as u32;
    let r1 = ((dst_lo >> 32) as i32).min((src_lo >> 32) as i32) as u32;
    let r2 = (dst_hi as i32).min(src_hi as i32) as u32;
    let r3 = ((dst_hi >> 32) as i32).min((src_hi >> 32) as i32) as u32;

    vcpu.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMINUW - Minimum of Packed Unsigned Words (0x3A)
pub fn pminuw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMINUW requires 66 prefix".to_string()));
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
        let d = ((dst_lo >> (i * 16)) & 0xFFFF) as u16;
        let s = ((src_lo >> (i * 16)) & 0xFFFF) as u16;
        result_lo |= (d.min(s) as u64) << (i * 16);
    }
    for i in 0..4 {
        let d = ((dst_hi >> (i * 16)) & 0xFFFF) as u16;
        let s = ((src_hi >> (i * 16)) & 0xFFFF) as u16;
        result_hi |= (d.min(s) as u64) << (i * 16);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMINUD - Minimum of Packed Unsigned Dwords (0x3B)
pub fn pminud(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMINUD requires 66 prefix".to_string()));
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

    let r0 = (dst_lo as u32).min(src_lo as u32);
    let r1 = ((dst_lo >> 32) as u32).min((src_lo >> 32) as u32);
    let r2 = (dst_hi as u32).min(src_hi as u32);
    let r3 = ((dst_hi >> 32) as u32).min((src_hi >> 32) as u32);

    vcpu.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMAXSB - Maximum of Packed Signed Bytes (0x3C)
pub fn pmaxsb(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMAXSB requires 66 prefix".to_string()));
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
        let d = ((dst_lo >> (i * 8)) & 0xFF) as i8;
        let s = ((src_lo >> (i * 8)) & 0xFF) as i8;
        result_lo |= ((d.max(s) as u8) as u64) << (i * 8);
    }
    for i in 0..8 {
        let d = ((dst_hi >> (i * 8)) & 0xFF) as i8;
        let s = ((src_hi >> (i * 8)) & 0xFF) as i8;
        result_hi |= ((d.max(s) as u8) as u64) << (i * 8);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMAXSD - Maximum of Packed Signed Dwords (0x3D)
pub fn pmaxsd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMAXSD requires 66 prefix".to_string()));
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

    let r0 = (dst_lo as i32).max(src_lo as i32) as u32;
    let r1 = ((dst_lo >> 32) as i32).max((src_lo >> 32) as i32) as u32;
    let r2 = (dst_hi as i32).max(src_hi as i32) as u32;
    let r3 = ((dst_hi >> 32) as i32).max((src_hi >> 32) as i32) as u32;

    vcpu.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMAXUW - Maximum of Packed Unsigned Words (0x3E)
pub fn pmaxuw(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMAXUW requires 66 prefix".to_string()));
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
        let d = ((dst_lo >> (i * 16)) & 0xFFFF) as u16;
        let s = ((src_lo >> (i * 16)) & 0xFFFF) as u16;
        result_lo |= (d.max(s) as u64) << (i * 16);
    }
    for i in 0..4 {
        let d = ((dst_hi >> (i * 16)) & 0xFFFF) as u16;
        let s = ((src_hi >> (i * 16)) & 0xFFFF) as u16;
        result_hi |= (d.max(s) as u64) << (i * 16);
    }
    vcpu.regs.xmm[xmm_dst][0] = result_lo;
    vcpu.regs.xmm[xmm_dst][1] = result_hi;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PMAXUD - Maximum of Packed Unsigned Dwords (0x3F)
pub fn pmaxud(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ctx.operand_size_override {
        return Err(Error::Emulator("PMAXUD requires 66 prefix".to_string()));
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

    let r0 = (dst_lo as u32).max(src_lo as u32);
    let r1 = ((dst_lo >> 32) as u32).max((src_lo >> 32) as u32);
    let r2 = (dst_hi as u32).max(src_hi as u32);
    let r3 = ((dst_hi >> 32) as u32).max((src_hi >> 32) as u32);

    vcpu.regs.xmm[xmm_dst][0] = (r0 as u64) | ((r1 as u64) << 32);
    vcpu.regs.xmm[xmm_dst][1] = (r2 as u64) | ((r3 as u64) << 32);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
