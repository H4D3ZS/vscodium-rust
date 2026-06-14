//! LEA instruction.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// LEA r, m (0x8D)
pub fn lea(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let reg = ((modrm >> 3) & 0x07) | ctx.rex_r();

    // LEA yields the segment OFFSET and must ignore any FS/GS override.
    let (addr, extra) = vcpu.decode_lea_addr(ctx, modrm_start)?;

    ctx.cursor = modrm_start + 1 + extra;
    vcpu.set_reg(reg, addr, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
