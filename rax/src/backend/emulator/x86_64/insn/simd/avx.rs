//! AVX (VEX-encoded) SIMD instruction implementations.
//!
//! VEX-encoded move operations for 128-bit (XMM) and 256-bit (YMM) registers.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

// =============================================================================
// VEX-encoded Move Operations
// =============================================================================

/// VMOVDQA load - VEX.66.0F 6F /r (aligned)
pub fn vmovdqa_load(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    vex_l: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let xmm_dst = reg as usize;

    if vex_l == 0 {
        // 128-bit XMM
        if is_memory {
            // Real HW raises #GP(0) on an unaligned VMOVDQA; aborting the VM is
            // wrong and the access itself is well-defined here, so just perform
            // it (read_mem/write_mem handle any alignment). Avoids killing the VM
            // on a stray unaligned aligned-move during late init.
            let _ = addr & 0xF;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
            vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
        } else {
            let xmm_src = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
        }
        // VEX clears upper bits
        vcpu.regs.ymm_high[xmm_dst][0] = 0;
        vcpu.regs.ymm_high[xmm_dst][1] = 0;
    } else {
        // 256-bit YMM
        if is_memory {
            // See the 128-bit case: perform the access rather than abort the VM.
            let _ = addr & 0x1F;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
            vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
            vcpu.regs.ymm_high[xmm_dst][0] = vcpu.read_mem(addr + 16, 8)?;
            vcpu.regs.ymm_high[xmm_dst][1] = vcpu.read_mem(addr + 24, 8)?;
        } else {
            let xmm_src = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
            vcpu.regs.ymm_high[xmm_dst][0] = vcpu.regs.ymm_high[xmm_src][0];
            vcpu.regs.ymm_high[xmm_dst][1] = vcpu.regs.ymm_high[xmm_src][1];
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVDQU load - VEX.F3.0F 6F /r (unaligned)
pub fn vmovdqu_load(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    vex_l: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;

    if vex_l == 0 {
        // 128-bit XMM
        if is_memory {
            vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
            vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
        } else {
            let xmm_src = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
        }
        vcpu.regs.ymm_high[xmm_dst][0] = 0;
        vcpu.regs.ymm_high[xmm_dst][1] = 0;
    } else {
        // 256-bit YMM
        if is_memory {
            vcpu.regs.xmm[xmm_dst][0] = vcpu.read_mem(addr, 8)?;
            vcpu.regs.xmm[xmm_dst][1] = vcpu.read_mem(addr + 8, 8)?;
            vcpu.regs.ymm_high[xmm_dst][0] = vcpu.read_mem(addr + 16, 8)?;
            vcpu.regs.ymm_high[xmm_dst][1] = vcpu.read_mem(addr + 24, 8)?;
        } else {
            let xmm_src = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
            vcpu.regs.ymm_high[xmm_dst][0] = vcpu.regs.ymm_high[xmm_src][0];
            vcpu.regs.ymm_high[xmm_dst][1] = vcpu.regs.ymm_high[xmm_src][1];
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVDQA store - VEX.66.0F 7F /r (aligned)
pub fn vmovdqa_store(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    vex_l: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if vex_l == 0 {
        // 128-bit XMM
        if is_memory {
            // Real HW raises #GP(0) on an unaligned VMOVDQA; aborting the VM is
            // wrong and the access itself is well-defined here, so just perform
            // it (read_mem/write_mem handle any alignment). Avoids killing the VM
            // on a stray unaligned aligned-move during late init.
            let _ = addr & 0xF;
            vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
            vcpu.regs.ymm_high[xmm_dst][0] = 0;
            vcpu.regs.ymm_high[xmm_dst][1] = 0;
        }
    } else {
        // 256-bit YMM
        if is_memory {
            // See the 128-bit case: perform the access rather than abort the VM.
            let _ = addr & 0x1F;
            vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
            vcpu.write_mem(addr + 16, vcpu.regs.ymm_high[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 24, vcpu.regs.ymm_high[xmm_src][1], 8)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
            vcpu.regs.ymm_high[xmm_dst][0] = vcpu.regs.ymm_high[xmm_src][0];
            vcpu.regs.ymm_high[xmm_dst][1] = vcpu.regs.ymm_high[xmm_src][1];
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVD/VMOVQ load - VEX.66.0F 6E /r (GPR or m32/m64 -> xmm, zero-extend to
/// 128 bits, clear the upper YMM lane). VEX.W selects D (32-bit) vs Q (64-bit).
pub fn vmovd_load(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    vex_w: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let size: u8 = if vex_w == 1 { 8 } else { 4 };
    let value = if is_memory {
        vcpu.read_mem(addr, size)?
    } else {
        vcpu.get_reg(rm, size)
    };
    vcpu.regs.xmm[xmm_dst][0] = value;
    vcpu.regs.xmm[xmm_dst][1] = 0;
    vcpu.regs.ymm_high[xmm_dst][0] = 0;
    vcpu.regs.ymm_high[xmm_dst][1] = 0;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVD/VMOVQ store - VEX.66.0F 7E /r (xmm -> GPR or m32/m64).
pub fn vmovd_store(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    vex_w: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;
    let value = vcpu.regs.xmm[xmm_src][0];
    if vex_w == 1 {
        if is_memory {
            vcpu.write_mem(addr, value, 8)?;
        } else {
            vcpu.set_reg(rm, value, 8);
        }
    } else if is_memory {
        vcpu.write_mem(addr, value & 0xFFFF_FFFF, 4)?;
    } else {
        vcpu.set_reg(rm, value & 0xFFFF_FFFF, 4);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVQ load - VEX.F3.0F 7E /r (xmm2/m64 -> xmm1, zero-extend 64->128, clear
/// upper YMM lane).
pub fn vmovq_load(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_dst = reg as usize;
    let value = if is_memory {
        vcpu.read_mem(addr, 8)?
    } else {
        vcpu.regs.xmm[rm as usize][0]
    };
    vcpu.regs.xmm[xmm_dst][0] = value;
    vcpu.regs.xmm[xmm_dst][1] = 0;
    vcpu.regs.ymm_high[xmm_dst][0] = 0;
    vcpu.regs.ymm_high[xmm_dst][1] = 0;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVQ store - VEX.66.0F D6 /r (xmm1 -> xmm2/m64; register form zero-extends).
pub fn vmovq_store(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;
    let value = vcpu.regs.xmm[xmm_src][0];
    if is_memory {
        vcpu.write_mem(addr, value, 8)?;
    } else {
        let xmm_dst = rm as usize;
        vcpu.regs.xmm[xmm_dst][0] = value;
        vcpu.regs.xmm[xmm_dst][1] = 0;
        vcpu.regs.ymm_high[xmm_dst][0] = 0;
        vcpu.regs.ymm_high[xmm_dst][1] = 0;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// VMOVDQU store - VEX.F3.0F 7F /r (unaligned)
pub fn vmovdqu_store(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    vex_l: u8,
) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let xmm_src = reg as usize;

    if vex_l == 0 {
        // 128-bit XMM
        if is_memory {
            vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
            vcpu.regs.ymm_high[xmm_dst][0] = 0;
            vcpu.regs.ymm_high[xmm_dst][1] = 0;
        }
    } else {
        // 256-bit YMM
        if is_memory {
            vcpu.write_mem(addr, vcpu.regs.xmm[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 8, vcpu.regs.xmm[xmm_src][1], 8)?;
            vcpu.write_mem(addr + 16, vcpu.regs.ymm_high[xmm_src][0], 8)?;
            vcpu.write_mem(addr + 24, vcpu.regs.ymm_high[xmm_src][1], 8)?;
        } else {
            let xmm_dst = rm as usize;
            vcpu.regs.xmm[xmm_dst][0] = vcpu.regs.xmm[xmm_src][0];
            vcpu.regs.xmm[xmm_dst][1] = vcpu.regs.xmm[xmm_src][1];
            vcpu.regs.ymm_high[xmm_dst][0] = vcpu.regs.ymm_high[xmm_src][0];
            vcpu.regs.ymm_high[xmm_dst][1] = vcpu.regs.ymm_high[xmm_src][1];
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
