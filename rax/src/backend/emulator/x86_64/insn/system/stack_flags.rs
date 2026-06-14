//! Stack-based flag instructions: PUSHF, POPF.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

fn pushf_popf_op_size(vcpu: &X86_64Vcpu, ctx: &InsnContext) -> u8 {
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0;
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode {
        // In 64-bit mode: 8 bytes default, 2 bytes with 66h prefix
        if ctx.operand_size_override { 2 } else { 8 }
    } else {
        // In 32-bit/16-bit mode: depends on D/B flag and prefix
        let default_16bit = !vcpu.sregs.cs.db;
        let is_16bit = default_16bit ^ ctx.operand_size_override;
        if is_16bit { 2 } else { 4 }
    }
}

/// PUSHF (0x9C)
pub fn pushf(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    let flags_val = vcpu.regs.rflags;
    let op_size = pushf_popf_op_size(vcpu, ctx);

    match op_size {
        2 => vcpu.push16(flags_val as u16)?,
        4 => vcpu.push32(flags_val as u32)?,
        8 => vcpu.push64(flags_val)?,
        _ => unreachable!(),
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POPF (0x9D)
pub fn popf(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = pushf_popf_op_size(vcpu, ctx);
    let flags_val = match op_size {
        2 => vcpu.pop16()? as u64,
        4 => vcpu.pop32()? as u64,
        8 => vcpu.pop64()?,
        _ => unreachable!(),
    };

    // Apply the appropriate mask based on operand size
    match op_size {
        2 => {
            // 16-bit POPF: only update low 16 bits (except reserved bits)
            let mask = 0xFFFFu64 & 0x00257FD5u64;
            vcpu.regs.rflags = (vcpu.regs.rflags & !0xFFFF) | (flags_val & mask) | 0x2;
        }
        4 | 8 => {
            // 32-bit/64-bit: update full flags (with mask for modifiable bits)
            vcpu.regs.rflags = (flags_val & 0x00000000_00257FD5) | 0x2;
        }
        _ => unreachable!(),
    }

    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
