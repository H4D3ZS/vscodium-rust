//! IMUL instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// IMUL r, r/m (0x0F 0xAF)
pub fn imul_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    let (result, overflow) = match op_size {
        2 => {
            let r = (dst as i16 as i32).wrapping_mul(src as i16 as i32);
            (r as u16 as u64, r as i16 as i32 != r)
        }
        4 => {
            let r = (dst as i32 as i64).wrapping_mul(src as i32 as i64);
            (r as u32 as u64, r as i32 as i64 != r)
        }
        8 => {
            let r = (dst as i64 as i128).wrapping_mul(src as i64 as i128);
            (r as u64, r as i64 as i128 != r)
        }
        _ => (dst.wrapping_mul(src), false),
    };
    vcpu.set_reg(reg, result, op_size);
    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// IMUL r, r/m, imm (0x69) - 3-operand with 16/32-bit immediate
pub fn imul_r_rm_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    ctx.rip_relative_offset = imm_size as usize;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    // Immediate is sign-extended to operand size
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };

    let (result, overflow) = match op_size {
        2 => {
            let r = (src as i16 as i32).wrapping_mul(imm as i16 as i32);
            (r as u16 as u64, r as i16 as i32 != r)
        }
        4 => {
            let r = (src as i32 as i64).wrapping_mul(imm as i32 as i64);
            (r as u32 as u64, r as i32 as i64 != r)
        }
        8 => {
            let r = (src as i64 as i128).wrapping_mul(imm as i64 as i128);
            (r as u64, r as i64 as i128 != r)
        }
        _ => (src.wrapping_mul(imm), false),
    };
    vcpu.set_reg(reg, result, op_size);
    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// IMUL r, r/m, imm8 (0x6B) - 3-operand with 8-bit immediate
pub fn imul_r_rm_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    ctx.rip_relative_offset = 1;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    // Immediate is 8-bit, sign-extended to operand size
    let imm = ctx.consume_imm(1)? as i8 as i64 as u64;

    let (result, overflow) = match op_size {
        2 => {
            let r = (src as i16 as i32).wrapping_mul(imm as i16 as i32);
            (r as u16 as u64, r as i16 as i32 != r)
        }
        4 => {
            let r = (src as i32 as i64).wrapping_mul(imm as i32 as i64);
            (r as u32 as u64, r as i32 as i64 != r)
        }
        8 => {
            let r = (src as i64 as i128).wrapping_mul(imm as i64 as i128);
            (r as u64, r as i64 as i128 != r)
        }
        _ => (src.wrapping_mul(imm), false),
    };
    vcpu.set_reg(reg, result, op_size);
    flags::set_cf_of(&mut vcpu.regs.rflags, overflow, overflow);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
