//! Table lookup translation instruction: XLAT/XLATB.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// XLAT/XLATB (0xD7) - Table lookup translation
pub fn xlat(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // AL = [RBX + AL]
    let index = vcpu.regs.rax & 0xFF;
    let addr = vcpu.regs.rbx.wrapping_add(index);
    let value = vcpu.read_mem(addr, 1)?;
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (value & 0xFF);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
