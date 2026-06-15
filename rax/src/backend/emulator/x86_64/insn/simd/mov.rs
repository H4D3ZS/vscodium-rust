//! SIMD data movement instructions: MOVD, MOVQ, MOVDQA, MOVDQU.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

// =============================================================================
// MOVD/MOVQ Instructions (XMM ↔ GPR/memory)
// =============================================================================

/// MOVD xmm, r/m32 or MOVQ xmm, r/m64 (66 0F 6E /r)
/// Move doubleword/quadword from GPR/memory to XMM register
pub fn movd_xmm_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_idx = reg as usize;

    // Check REX.W for MOVQ (64-bit) vs MOVD (32-bit)
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    let value = if is_memory {
        if is_64bit {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.read_mem(addr, 4)?
        }
    } else {
        if is_64bit {
            vcpu.get_reg(rm, 8)
        } else {
            vcpu.get_reg(rm, 4)
        }
    };

    // Store in low part of XMM, zero-extend to 128 bits
    vcpu.regs.xmm[xmm_idx][0] = value;
    vcpu.regs.xmm[xmm_idx][1] = 0;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVD r/m32, xmm or MOVQ r/m64, xmm (66 0F 7E /r)
/// Move doubleword/quadword from XMM register to GPR/memory
pub fn movd_rm_xmm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_idx = reg as usize;

    // Check REX.W for MOVQ (64-bit) vs MOVD (32-bit)
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    let value = vcpu.regs.xmm[xmm_idx][0];

    if is_memory {
        if is_64bit {
            vcpu.write_mem(addr, value, 8)?;
        } else {
            vcpu.write_mem(addr, value as u32 as u64, 4)?;
        }
    } else {
        if is_64bit {
            vcpu.set_reg(rm, value, 8);
        } else {
            vcpu.set_reg(rm, value as u32 as u64, 4);
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVD mm, r/m32 or MOVQ mm, r/m64 (NP 0F 6E /r)
/// Move doubleword/quadword from GPR/memory to MMX register
pub fn movd_mm_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_idx = (reg & 0x07) as usize;

    // Check REX.W for MOVQ (64-bit) vs MOVD (32-bit)
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    let value = if is_memory {
        if is_64bit {
            vcpu.read_mem(addr, 8)?
        } else {
            vcpu.read_mem(addr, 4)?
        }
    } else {
        if is_64bit {
            vcpu.get_reg(rm, 8)
        } else {
            vcpu.get_reg(rm, 4)
        }
    };

    // Store in MMX register, zero-extended for MOVD
    vcpu.regs.mm[mm_idx] = value;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVD r/m32, mm or MOVQ r/m64, mm (NP 0F 7E /r)
/// Move doubleword/quadword from MMX register to GPR/memory
pub fn movd_rm_mm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_idx = (reg & 0x07) as usize;

    // Check REX.W for MOVQ (64-bit) vs MOVD (32-bit)
    let is_64bit = ctx.rex.map_or(false, |r| r & 0x08 != 0);

    let value = vcpu.regs.mm[mm_idx];

    if is_memory {
        if is_64bit {
            vcpu.write_mem(addr, value, 8)?;
        } else {
            vcpu.write_mem(addr, value as u32 as u64, 4)?;
        }
    } else {
        if is_64bit {
            vcpu.set_reg(rm, value, 8);
        } else {
            vcpu.set_reg(rm, value as u32 as u64, 4);
        }
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// MOVQ Instructions (XMM ↔ XMM/memory)
// =============================================================================

/// MOVQ xmm1, xmm2/m64 (F3 0F 7E /r)
/// Move quadword from xmm2/m64 to xmm1
pub fn movq_xmm_xmm_m64(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    let value = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_src][0]
    };

    // Store in low part of XMM, zero high part
    vcpu.regs.xmm[xmm_dst][0] = value;
    vcpu.regs.xmm[xmm_dst][1] = 0;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVQ xmm2/m64, xmm1 (66 0F D6 /r)
/// Move quadword from xmm1 to xmm2/m64
pub fn movq_xmm_m64_xmm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    let value = vcpu.regs.xmm[xmm_src][0];

    if is_memory {
        vcpu.write_mem(addr, value, 8)?;
    } else {
        let xmm_dst = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = value;
        vcpu.regs.xmm[xmm_dst][1] = 0;
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVQ mm, mm/m64 (NP 0F 6F /r)
/// Move quadword from mm/m64 to MMX register
pub fn movq_mm_mm_m64(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_dst = (reg & 0x07) as usize;

    let value = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        let mm_src = (rm & 0x07) as usize;
        vcpu.regs.mm[mm_src]
    };

    vcpu.regs.mm[mm_dst] = value;

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVQ mm/m64, mm (NP 0F 7F /r)
/// Store quadword from MMX register to mm/m64
pub fn movq_mm_m64_mm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let mm_src = (reg & 0x07) as usize;

    let value = vcpu.regs.mm[mm_src];

    if is_memory {
        vcpu.write_mem(addr, value, 8)?;
    } else {
        let mm_dst = (rm & 0x07) as usize;
        vcpu.regs.mm[mm_dst] = value;
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

// =============================================================================
// 128-bit Aligned/Unaligned Movement
// =============================================================================

/// MOVDQA xmm1, xmm2/m128 (66 0F 6F /r)
/// Move aligned packed integer values from xmm2/m128 to xmm1
/// Memory operand must be 16-byte aligned, otherwise #GP exception
pub fn movdqa_xmm_xmm_m128(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if is_memory {
        // MOVDQA requires 16-byte alignment for memory operands
        if addr & 0xF != 0 {
            return Err(Error::Emulator(format!(
                "MOVDQA: unaligned memory access at {:#x} (must be 16-byte aligned)",
                addr
            )));
        }
        // Read 128 bits from memory
        let low = vcpu.read_mem(addr, 8)?;
        let high = vcpu.read_mem(addr + 8, 8)?;
        vcpu.regs.xmm[xmm_dst][0] = low;
        vcpu.regs.xmm[xmm_dst][1] = high;
    } else {
        // XMM to XMM move
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVDQU xmm1, xmm2/m128 (F3 0F 6F /r)
/// Move unaligned packed integer values from xmm2/m128 to xmm1
pub fn movdqu_xmm_xmm_m128(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if is_memory {
        // MOVDQU allows unaligned memory access
        let low = vcpu.read_mem(addr, 8)?;
        let high = vcpu.read_mem(addr + 8, 8)?;
        vcpu.regs.xmm[xmm_dst][0] = low;
        vcpu.regs.xmm[xmm_dst][1] = high;
    } else {
        // XMM to XMM move
        let xmm_src = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVDQA xmm2/m128, xmm1 (66 0F 7F /r)
/// Store aligned packed integer values from xmm1 to xmm2/m128
/// Memory operand must be 16-byte aligned, otherwise #GP exception
pub fn movdqa_xmm_m128_xmm(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if is_memory {
        // MOVDQA requires 16-byte alignment for memory operands
        if addr & 0xF != 0 {
            return Err(Error::Emulator(format!(
                "MOVDQA: unaligned memory access at {:#x} (must be 16-byte aligned)",
                addr
            )));
        }
        // Write 128 bits to memory
        vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
        vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
    } else {
        // XMM to XMM move
        let xmm_dst = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVDQU xmm2/m128, xmm1 (F3 0F 7F /r)
/// Store unaligned packed integer values from xmm1 to xmm2/m128
pub fn movdqu_xmm_m128_xmm(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if is_memory {
        // MOVDQU allows unaligned memory access
        vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
        vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
    } else {
        // XMM to XMM move
        let xmm_dst = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
        vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
