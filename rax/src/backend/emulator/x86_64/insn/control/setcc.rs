//! Set byte on condition instructions: SETcc.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// SETcc r/m8 (0x0F 0x90-0x9F)
pub fn setcc(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext, cc: u8) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (_, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let value = if vcpu.check_condition(cc) { 1u8 } else { 0u8 };

    if is_memory {
        vcpu.mmu.write_u8(addr, value, &vcpu.sregs)?;
    } else {
        vcpu.set_reg8(rm, value as u64, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
