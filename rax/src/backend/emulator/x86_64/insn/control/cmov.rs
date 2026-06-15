//! Conditional move instructions: CMOVcc.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// CMOVcc r, r/m (0x0F 0x40-0x4F) - Conditional Move
pub fn cmovcc(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext, cc: u8) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Only perform the move if the condition is true
    if vcpu.check_condition(cc) {
        let value = if is_memory {
            vcpu.read_mem(addr, op_size)?
        } else {
            vcpu.get_reg(rm, op_size)
        };
        vcpu.set_reg(reg, value, op_size);
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
