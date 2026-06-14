//! CMP instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// CMP r/m8, r8 (0x38)
pub fn cmp_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex) as u8;

    let dst = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg8(rm, has_rex) as u8
    };
    let result = dst.wrapping_sub(src) as u64;
    vcpu.set_lazy_sub(dst as u64, src as u64, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMP r/m, r (0x39)
pub fn cmp_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);

    let dst = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let result = dst.wrapping_sub(src);
    vcpu.set_lazy_sub(dst, src, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMP r8, r/m8 (0x3A)
pub fn cmp_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg8(reg, has_rex) as u8;

    let src = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg8(rm, has_rex) as u8
    };
    let result = dst.wrapping_sub(src) as u64;
    vcpu.set_lazy_sub(dst as u64, src as u64, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMP r, r/m (0x3B)
pub fn cmp_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let result = dst.wrapping_sub(src);
    vcpu.set_lazy_sub(dst, src, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMP AL, imm8 (0x3C)
pub fn cmp_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let imm = ctx.consume_u8()? as u64;
    let al = vcpu.regs.rax & 0xFF;
    let result = al.wrapping_sub(imm);
    vcpu.set_lazy_sub(al, imm, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMP rAX, imm (0x3D)
pub fn cmp_rax_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };
    let rax = vcpu.get_reg(0, op_size);
    let result = rax.wrapping_sub(imm);
    vcpu.set_lazy_sub(rax, imm, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
