//! Group 1 instructions (0x80, 0x81, 0x83).
//!
//! These opcodes handle multiple operations (ADD, OR, ADC, SBB, AND, SUB, XOR, CMP)
//! based on the ModR/M reg field.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// ADC dst, imm (cold). Materializes pending flags, applies carry-in, sets
/// rflags directly, and clears the lazy state. Kept out-of-line so the hot
/// Group 1 dispatch (ADD/OR/AND/SUB/XOR/CMP) keeps a tiny stack frame.
#[cold]
#[inline(never)]
fn adc_cold(vcpu: &mut X86_64Vcpu, dst: u64, imm: u64, size: u8) -> u64 {
    vcpu.materialize_flags();
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = cf_in as u64;
    let r = dst.wrapping_add(imm).wrapping_add(cf_val);
    flags::update_flags_adc(&mut vcpu.regs.rflags, dst, imm, cf_in, r, size);
    vcpu.clear_lazy_flags();
    r
}

/// SBB dst, imm (cold). See `adc_cold`.
#[cold]
#[inline(never)]
fn sbb_cold(vcpu: &mut X86_64Vcpu, dst: u64, imm: u64, size: u8) -> u64 {
    vcpu.materialize_flags();
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = cf_in as u64;
    let r = dst.wrapping_sub(imm).wrapping_sub(cf_val);
    flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, imm, cf_in, r, size);
    vcpu.clear_lazy_flags();
    r
}

/// Apply a Group 1 ALU op (`op` = ModR/M reg field 0..7) to `dst`/`imm` at the
/// given operand size, set the appropriate lazy flags, and return
/// `(result, update_dest)`. Shared by the register and memory operand paths so
/// the per-handler body stays a thin decode + dispatch wrapper. The hot ops
/// (ADD/OR/AND/SUB/XOR/CMP) are pure register arithmetic; ADC/SBB defer to the
/// out-of-line cold helpers.
#[inline(always)]
fn apply_group1(vcpu: &mut X86_64Vcpu, op: u8, dst: u64, imm: u64, op_size: u8) -> (u64, bool) {
    match op {
        0 => {
            let r = dst.wrapping_add(imm);
            vcpu.set_lazy_add(dst, imm, r, op_size);
            (r, true)
        }
        1 => {
            let r = dst | imm;
            vcpu.set_lazy_logic(r, op_size);
            (r, true)
        }
        2 => (adc_cold(vcpu, dst, imm, op_size), true),
        3 => (sbb_cold(vcpu, dst, imm, op_size), true),
        4 => {
            let r = dst & imm;
            vcpu.set_lazy_logic(r, op_size);
            (r, true)
        }
        5 => {
            let r = dst.wrapping_sub(imm);
            vcpu.set_lazy_sub(dst, imm, r, op_size);
            (r, true)
        }
        6 => {
            let r = dst ^ imm;
            vcpu.set_lazy_logic(r, op_size);
            (r, true)
        }
        _ => {
            // 7 => CMP (no writeback). The decode tables only ever feed 0..=7
            // here, so the unreachable 8+ case collapses into the CMP arm.
            let r = dst.wrapping_sub(imm);
            vcpu.set_lazy_sub(dst, imm, r, op_size);
            (r, false)
        }
    }
}

/// Cold path for Group 1 r/m,imm8 (0x83) when the operand is in memory.
#[cold]
#[inline(never)]
fn group1_rm_imm8_mem(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    op: u8,
    op_size: u8,
) -> Result<Option<VcpuExit>> {
    ctx.rip_relative_offset = 1;
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    let dst = vcpu.read_mem(addr, op_size)?;
    let imm = ctx.consume_u8()? as i8 as i64 as u64;
    let (result, update_dest) = apply_group1(vcpu, op, dst, imm, op_size);
    if update_dest {
        vcpu.write_mem(addr, result, op_size)?;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Cold path for Group 1 r/m8,imm8 (0x80) when the operand is in memory.
#[cold]
#[inline(never)]
fn group1_rm8_imm8_mem(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    op: u8,
) -> Result<Option<VcpuExit>> {
    ctx.rip_relative_offset = 1;
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
    let imm = ctx.consume_u8()? as u64;
    let (result, update_dest) = apply_group1(vcpu, op, dst, imm, 1);
    if update_dest {
        vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 1: r/m8, imm8 (0x80)
pub fn group1_rm8_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;

    if modrm >> 6 != 3 {
        return group1_rm8_imm8_mem(vcpu, ctx, modrm_start, op);
    }

    let has_rex = ctx.rex.is_some();
    let rm = (modrm & 0x07) | ctx.rex_b();
    let dst = vcpu.get_reg8(rm, has_rex);
    let imm = ctx.consume_u8()? as u64;

    let (result, update_dest) = apply_group1(vcpu, op, dst, imm, 1);
    if update_dest {
        vcpu.set_reg8(rm, result, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Cold path for Group 1 r/m,imm32 (0x81) when the operand is in memory.
#[cold]
#[inline(never)]
fn group1_rm_imm32_mem(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    op: u8,
    op_size: u8,
    imm_size: u8,
) -> Result<Option<VcpuExit>> {
    ctx.rip_relative_offset = imm_size as usize;
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    let dst = vcpu.read_mem(addr, op_size)?;
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };
    let (result, update_dest) = apply_group1(vcpu, op, dst, imm, op_size);
    if update_dest {
        vcpu.write_mem(addr, result, op_size)?;
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 1: r/m, imm32 (0x81)
pub fn group1_rm_imm32(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;

    if modrm >> 6 != 3 {
        return group1_rm_imm32_mem(vcpu, ctx, modrm_start, op, op_size, imm_size);
    }

    let rm = (modrm & 0x07) | ctx.rex_b();
    let dst = vcpu.get_reg(rm, op_size);

    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };

    let (result, update_dest) = apply_group1(vcpu, op, dst, imm, op_size);
    if update_dest {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 1: r/m, imm8 sign-extended (0x83)
pub fn group1_rm_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;

    // Memory operand: hand off to the out-of-line cold path so the (dominant)
    // register-direct path keeps a small stack frame and prologue.
    if modrm >> 6 != 3 {
        return group1_rm_imm8_mem(vcpu, ctx, modrm_start, op, op_size);
    }

    let rm = (modrm & 0x07) | ctx.rex_b();
    let dst = vcpu.get_reg(rm, op_size);
    let imm = ctx.consume_u8()? as i8 as i64 as u64;

    let (result, update_dest) = apply_group1(vcpu, op, dst, imm, op_size);
    if update_dest {
        vcpu.set_reg(rm, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
