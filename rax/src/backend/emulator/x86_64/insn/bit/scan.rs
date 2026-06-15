//! Bit scan instructions: BSF, BSR.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// BSF r, r/m (0x0F 0xBC)
pub fn bsf(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    if value == 0 {
        vcpu.regs.rflags |= flags::bits::ZF;
        // Destination is undefined when source is 0
    } else {
        vcpu.regs.rflags &= !flags::bits::ZF;
        let bit_index = value.trailing_zeros() as u64;
        vcpu.set_reg(reg, bit_index, op_size);
    }
    // BSF writes ZF eagerly; drop any pending lazy op so a later flag reader
    // (e.g. Jcc) sees this ZF rather than recomputing from a stale prior ALU op.
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// BSR r, r/m (0x0F 0xBD)
pub fn bsr(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    if value == 0 {
        vcpu.regs.rflags |= flags::bits::ZF;
        // Destination is undefined when source is 0
    } else {
        vcpu.regs.rflags &= !flags::bits::ZF;
        // Count leading zeros for the specific operand size
        let bit_index = match op_size {
            2 => 15 - (value as u16).leading_zeros(),
            4 => 31 - (value as u32).leading_zeros(),
            8 => 63 - value.leading_zeros(),
            _ => 63 - value.leading_zeros(),
        };
        vcpu.set_reg(reg, bit_index as u64, op_size);
    }
    // BSR writes ZF eagerly; drop any pending lazy op so a later flag reader
    // (e.g. Jcc) sees this ZF rather than recomputing from a stale prior ALU op.
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
