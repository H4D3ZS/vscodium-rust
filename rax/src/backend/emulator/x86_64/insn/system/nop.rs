//! NOP-like instructions: ENDBR, multi-byte NOP.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// ENDBR64/ENDBR32 (0x0F 0x1E) - CET instructions, treat as NOP
pub fn endbr(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Skip the ModR/M byte (FA=ENDBR64, FB=ENDBR32)
    ctx.cursor += 1;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Multi-byte NOP (0x0F 0x1F)
pub fn nop_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    // Skip any additional bytes for memory operand
    if modrm >> 6 != 3 {
        let (_, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
