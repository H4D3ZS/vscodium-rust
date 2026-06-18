//! Bit count instructions: POPCNT, LZCNT, TZCNT.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// TZCNT r, r/m (F3 0F BC) - Count trailing zeros
pub fn tzcnt(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    let bit_count = (op_size * 8) as u64;
    let result = if value == 0 {
        bit_count
    } else {
        value.trailing_zeros() as u64
    };

    vcpu.set_reg(reg, result, op_size);

    // TZCNT flags: CF=1 if source is 0, ZF=1 if result is 0
    vcpu.regs.rflags &= !(flags::bits::CF | flags::bits::ZF);
    if value == 0 {
        vcpu.regs.rflags |= flags::bits::CF;
    }
    if result == 0 {
        vcpu.regs.rflags |= flags::bits::ZF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LZCNT r, r/m (F3 0F BD) - Count leading zeros
pub fn lzcnt(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    let bit_count = (op_size * 8) as u32;

    // For LZCNT, we count leading zeros within the operand size
    let result = if value == 0 {
        bit_count as u64
    } else {
        // We need to count leading zeros for the specific operand size
        let leading_zeros = match op_size {
            2 => (value as u16).leading_zeros(),
            4 => (value as u32).leading_zeros(),
            8 => value.leading_zeros(),
            _ => value.leading_zeros(),
        };
        leading_zeros as u64
    };

    vcpu.set_reg(reg, result, op_size);

    // LZCNT flags: CF=1 if source is 0, ZF=1 if result is 0
    vcpu.regs.rflags &= !(flags::bits::CF | flags::bits::ZF);
    if value == 0 {
        vcpu.regs.rflags |= flags::bits::CF;
    }
    if result == 0 {
        vcpu.regs.rflags |= flags::bits::ZF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POPCNT r, r/m (F3 0F B8)
pub fn popcnt(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    let count = match op_size {
        2 => (value as u16).count_ones() as u64,
        4 => (value as u32).count_ones() as u64,
        8 => value.count_ones() as u64,
        _ => value.count_ones() as u64,
    };

    vcpu.set_reg(reg, count, op_size);

    // POPCNT clears OF, SF, AF, CF, PF and sets ZF if result is zero
    vcpu.regs.rflags &= !(flags::bits::OF
        | flags::bits::SF
        | flags::bits::AF
        | flags::bits::CF
        | flags::bits::PF
        | flags::bits::ZF);
    if count == 0 {
        vcpu.regs.rflags |= flags::bits::ZF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
