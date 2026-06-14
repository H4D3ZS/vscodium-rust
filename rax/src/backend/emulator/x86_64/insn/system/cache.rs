//! Cache hint instructions: CLDEMOTE, CLWB.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// CLDEMOTE m8 (0F 1C /0) - cache line demote hint (treated as NOP).
pub fn cldemote(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (_reg, _rm, is_memory, _addr, _) = vcpu.decode_modrm(ctx)?;
    if !is_memory {
        return Err(Error::Emulator(
            "CLDEMOTE requires memory operand".to_string(),
        ));
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
