//! XCHG instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// XCHG r8, r/m8 (0x86)
pub fn xchg_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    if is_memory {
        let reg_val = vcpu.get_reg8(reg, ctx.rex.is_some());
        let mem_val = vcpu.read_mem(addr, 1)?;
        vcpu.set_reg8(reg, mem_val, ctx.rex.is_some());
        vcpu.write_mem(addr, reg_val, 1)?;
    } else {
        let reg_val = vcpu.get_reg8(reg, ctx.rex.is_some());
        let rm_val = vcpu.get_reg8(rm, ctx.rex.is_some());
        vcpu.set_reg8(reg, rm_val, ctx.rex.is_some());
        vcpu.set_reg8(rm, reg_val, ctx.rex.is_some());
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XCHG r, r/m (0x87)
pub fn xchg_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let reg_val = vcpu.get_reg(reg, op_size);

    if is_memory {
        let mem_val = vcpu.read_mem(addr, op_size)?;
        vcpu.set_reg(reg, mem_val, op_size);
        vcpu.write_mem(addr, reg_val, op_size)?;
    } else {
        let rm_val = vcpu.get_reg(rm, op_size);
        vcpu.set_reg(reg, rm_val, op_size);
        vcpu.set_reg(rm, reg_val, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XCHG rAX, r (0x91-0x97)
pub fn xchg_rax_r(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let reg = (opcode - 0x90) | ctx.rex_b();
    let rax_val = vcpu.get_reg(0, op_size);
    let reg_val = vcpu.get_reg(reg, op_size);
    vcpu.set_reg(0, reg_val, op_size);
    vcpu.set_reg(reg, rax_val, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
