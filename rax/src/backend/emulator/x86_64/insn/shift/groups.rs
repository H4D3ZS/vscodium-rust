//! Group 2 shift/rotate instructions: SHL, SHR, SAL, SAR, ROL, ROR, RCL, RCR.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::core::{execute_shift, execute_shift8};

/// Group 2: r/m8, imm8 (0xC0)
pub fn group2_rm8_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    ctx.rip_relative_offset = 1;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let has_rex = ctx.rex.is_some();
    let rm = (modrm & 0x07) | ctx.rex_b();

    let (val, addr_opt) = if modrm >> 6 == 3 {
        (vcpu.get_reg8(rm, has_rex) as u8, None)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        (vcpu.mmu.read_u8(addr, &vcpu.sregs)?, Some(addr))
    };

    let count = ctx.consume_u8()? & 0x1F;
    let result = execute_shift8(vcpu, op, val, count)?;

    if let Some(addr) = addr_opt {
        vcpu.mmu.write_u8(addr, result, &vcpu.sregs)?;
    } else {
        vcpu.set_reg8(rm, result as u64, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 2: r/m, imm8 (0xC1)
pub fn group2_rm_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    ctx.rip_relative_offset = 1;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();

    let (val, addr_opt) = if modrm >> 6 == 3 {
        (vcpu.get_reg(rm, op_size), None)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        (vcpu.read_mem(addr, op_size)?, Some(addr))
    };

    // Mask count: 6 bits for 64-bit ops, 5 bits for 16/32-bit ops
    let count_mask = if op_size == 8 { 0x3F } else { 0x1F };
    let count = ctx.consume_u8()? & count_mask;
    let result = execute_shift(vcpu, op, val, count, op_size)?;

    if let Some(addr) = addr_opt {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 2: r/m8, 1 (0xD0)
pub fn group2_rm8_1(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let has_rex = ctx.rex.is_some();
    let rm = (modrm & 0x07) | ctx.rex_b();

    let (val, addr_opt) = if modrm >> 6 == 3 {
        (vcpu.get_reg8(rm, has_rex) as u8, None)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        (vcpu.mmu.read_u8(addr, &vcpu.sregs)?, Some(addr))
    };

    let result = execute_shift8(vcpu, op, val, 1)?;

    if let Some(addr) = addr_opt {
        vcpu.mmu.write_u8(addr, result, &vcpu.sregs)?;
    } else {
        vcpu.set_reg8(rm, result as u64, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 2: r/m, 1 (0xD1)
pub fn group2_rm_1(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();

    let (val, addr_opt) = if modrm >> 6 == 3 {
        (vcpu.get_reg(rm, op_size), None)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        (vcpu.read_mem(addr, op_size)?, Some(addr))
    };

    let result = execute_shift(vcpu, op, val, 1, op_size)?;

    if let Some(addr) = addr_opt {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 2: r/m8, CL (0xD2)
pub fn group2_rm8_cl(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let has_rex = ctx.rex.is_some();
    let rm = (modrm & 0x07) | ctx.rex_b();
    let count = (vcpu.regs.rcx & 0x1F) as u8;

    let (val, addr_opt) = if modrm >> 6 == 3 {
        (vcpu.get_reg8(rm, has_rex) as u8, None)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        (vcpu.mmu.read_u8(addr, &vcpu.sregs)?, Some(addr))
    };

    let result = execute_shift8(vcpu, op, val, count)?;

    if let Some(addr) = addr_opt {
        vcpu.mmu.write_u8(addr, result, &vcpu.sregs)?;
    } else {
        vcpu.set_reg8(rm, result as u64, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 2: r/m, CL (0xD3)
pub fn group2_rm_cl(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();
    // Mask count: 6 bits for 64-bit ops, 5 bits for 16/32-bit ops
    let count_mask = if op_size == 8 { 0x3F } else { 0x1F };
    let count = (vcpu.regs.rcx & count_mask) as u8;

    let (val, addr_opt) = if modrm >> 6 == 3 {
        (vcpu.get_reg(rm, op_size), None)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        (vcpu.read_mem(addr, op_size)?, Some(addr))
    };

    let result = execute_shift(vcpu, op, val, count, op_size)?;

    if let Some(addr) = addr_opt {
        vcpu.write_mem(addr, result, op_size)?;
    } else {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
