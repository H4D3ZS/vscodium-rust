//! XOR instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// XOR r/m8, r8 (0x30)
pub fn xor_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex);

    if is_memory {
        let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
        let result = (dst ^ src) & 0xFF;
        vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
        vcpu.set_lazy_logic(result, 1);
    } else {
        let dst = vcpu.get_reg8(rm, has_rex);
        let result = (dst ^ src) & 0xFF;
        vcpu.set_reg8(rm, result, has_rex);
        vcpu.set_lazy_logic(result, 1);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XOR r/m, r (0x31) memory-operand path (cold). Kept out-of-line so the
/// register-direct hot path keeps a minimal stack frame.
#[cold]
#[inline(never)]
fn xor_rm_r_mem(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    addr: u64,
    src: u64,
    op_size: u8,
) -> Result<Option<VcpuExit>> {
    let dst = vcpu.read_mem(addr, op_size)?;
    let result = dst ^ src;
    vcpu.write_mem(addr, result, op_size)?;
    vcpu.set_lazy_logic(result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XOR r/m, r (0x31)
pub fn xor_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);

    if is_memory {
        return xor_rm_r_mem(vcpu, ctx, addr, src, op_size);
    }

    let dst = vcpu.get_reg(rm, op_size);
    let result = dst ^ src;
    vcpu.set_reg(rm, result, op_size);
    vcpu.set_lazy_logic(result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XOR r8, r/m8 (0x32)
pub fn xor_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg8(reg, has_rex);

    let src = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
    } else {
        vcpu.get_reg8(rm, has_rex)
    };
    let result = (dst ^ src) & 0xFF;
    vcpu.set_reg8(reg, result, has_rex);
    vcpu.set_lazy_logic(result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XOR r, r/m (0x33)
pub fn xor_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let result = dst ^ src;
    vcpu.set_reg(reg, result, op_size);
    vcpu.set_lazy_logic(result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XOR AL, imm8 (0x34)
pub fn xor_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let imm = ctx.consume_u8()? as u64;
    let result = (vcpu.regs.rax & 0xFF) ^ imm;
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | result;
    vcpu.set_lazy_logic(result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XOR rAX, imm (0x35)
pub fn xor_rax_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };
    let result = vcpu.get_reg(0, op_size) ^ imm;
    vcpu.set_reg(0, result, op_size);
    vcpu.set_lazy_logic(result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
