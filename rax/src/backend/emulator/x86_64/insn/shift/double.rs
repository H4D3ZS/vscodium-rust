//! Double precision shift instructions: SHLD, SHRD.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// SHLD r/m, r, imm8 (0x0F 0xA4) - Double precision shift left
pub fn shld_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    ctx.rip_relative_offset = 1;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    // Mask count: 6 bits for 64-bit ops, 5 bits for 16/32-bit ops
    let count_mask = if op_size == 8 { 0x3F } else { 0x1F };
    let count = ctx.consume_u8()? & count_mask;

    let dst = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let src = vcpu.get_reg(reg, op_size);

    let result = execute_shld(vcpu, dst, src, count, op_size);

    if is_memory {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SHLD r/m, r, CL (0x0F 0xA5) - Double precision shift left
pub fn shld_cl(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    // Mask count: 6 bits for 64-bit ops, 5 bits for 16/32-bit ops
    let count_mask = if op_size == 8 { 0x3F } else { 0x1F };
    let count = (vcpu.regs.rcx & count_mask) as u8;

    let dst = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let src = vcpu.get_reg(reg, op_size);

    let result = execute_shld(vcpu, dst, src, count, op_size);

    if is_memory {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SHRD r/m, r, imm8 (0x0F 0xAC) - Double precision shift right
pub fn shrd_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    ctx.rip_relative_offset = 1;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    // Mask count: 6 bits for 64-bit ops, 5 bits for 16/32-bit ops
    let count_mask = if op_size == 8 { 0x3F } else { 0x1F };
    let count = ctx.consume_u8()? & count_mask;

    let dst = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let src = vcpu.get_reg(reg, op_size);

    let result = execute_shrd(vcpu, dst, src, count, op_size);

    if is_memory {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SHRD r/m, r, CL (0x0F 0xAD) - Double precision shift right
pub fn shrd_cl(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    // Mask count: 6 bits for 64-bit ops, 5 bits for 16/32-bit ops
    let count_mask = if op_size == 8 { 0x3F } else { 0x1F };
    let count = (vcpu.regs.rcx & count_mask) as u8;

    let dst = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let src = vcpu.get_reg(reg, op_size);

    let result = execute_shrd(vcpu, dst, src, count, op_size);

    if is_memory {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Execute SHLD: shift dst left, filling in from src
fn execute_shld(vcpu: &mut X86_64Vcpu, dst: u64, src: u64, count: u8, size: u8) -> u64 {
    let bits = (size * 8) as u32;
    let mask = if bits == 64 {
        !0u64
    } else {
        (1u64 << bits) - 1
    };
    let count_mask = if bits == 64 { 0x3F } else { 0x1F };
    let count = (count as u32) & count_mask;

    if count == 0 {
        return dst & mask;
    }
    if count > bits {
        return dst & mask;
    }

    // SHLD: shift dst left by count, bring in high bits from src
    // Result = (dst << count) | (src >> (bits - count))
    let result = ((dst << count) | (src >> (bits - count))) & mask;

    // Update flags
    // CF = last bit shifted out of dst
    let cf = if count <= bits {
        (dst >> (bits - count)) & 1 != 0
    } else {
        false
    };

    // OF = sign bit changed (only defined for count == 1)
    let of = if count == 1 {
        ((result >> (bits - 1)) ^ (dst >> (bits - 1))) & 1 != 0
    } else {
        false
    };

    flags::update_flags_logic(&mut vcpu.regs.rflags, result, size);
    if cf {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    if of {
        vcpu.regs.rflags |= flags::bits::OF;
    } else {
        vcpu.regs.rflags &= !flags::bits::OF;
    }
    vcpu.clear_lazy_flags();

    result
}

/// Execute SHRD: shift dst right, filling in from src
fn execute_shrd(vcpu: &mut X86_64Vcpu, dst: u64, src: u64, count: u8, size: u8) -> u64 {
    let bits = (size * 8) as u32;
    let mask = if bits == 64 {
        !0u64
    } else {
        (1u64 << bits) - 1
    };
    let count_mask = if bits == 64 { 0x3F } else { 0x1F };
    let count = (count as u32) & count_mask;

    if count == 0 {
        return dst & mask;
    }
    if count > bits {
        return dst & mask;
    }

    // SHRD: shift dst right by count, bring in low bits from src
    // Result = (dst >> count) | (src << (bits - count))
    let result = ((dst >> count) | ((src & mask) << (bits - count))) & mask;

    // Update flags
    // CF = last bit shifted out of dst
    let cf = if count > 0 && count <= bits {
        (dst >> (count - 1)) & 1 != 0
    } else {
        false
    };

    // OF = sign bit changed (only defined for count == 1)
    let of = if count == 1 {
        ((result >> (bits - 1)) ^ (dst >> (bits - 1))) & 1 != 0
    } else {
        false
    };

    flags::update_flags_logic(&mut vcpu.regs.rflags, result, size);
    if cf {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    if of {
        vcpu.regs.rflags |= flags::bits::OF;
    } else {
        vcpu.regs.rflags &= !flags::bits::OF;
    }
    vcpu.clear_lazy_flags();

    result
}
