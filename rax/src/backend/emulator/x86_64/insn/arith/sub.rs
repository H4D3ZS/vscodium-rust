//! SUB and SBB instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// SUB r/m8, r8 (0x28)
pub fn sub_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex);

    if is_memory {
        let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
        let result = dst.wrapping_sub(src) & 0xFF;
        vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
        vcpu.set_lazy_sub(dst, src, result, 1);
    } else {
        let dst = vcpu.get_reg8(rm, has_rex);
        let result = dst.wrapping_sub(src) & 0xFF;
        vcpu.set_reg8(rm, result, has_rex);
        vcpu.set_lazy_sub(dst, src, result, 1);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SUB r/m, r (0x29)
pub fn sub_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);

    if is_memory {
        let dst = vcpu.read_mem(addr, op_size)?;
        let result = dst.wrapping_sub(src);
        vcpu.write_mem(addr, result, op_size)?;
        vcpu.set_lazy_sub(dst, src, result, op_size);
    } else {
        let dst = vcpu.get_reg(rm, op_size);
        let result = dst.wrapping_sub(src);
        vcpu.set_reg(rm, result, op_size);
        vcpu.set_lazy_sub(dst, src, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SUB r8, r/m8 (0x2A)
pub fn sub_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg8(reg, has_rex);

    let src = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
    } else {
        vcpu.get_reg8(rm, has_rex)
    };
    let result = dst.wrapping_sub(src) & 0xFF;
    vcpu.set_reg8(reg, result, has_rex);
    vcpu.set_lazy_sub(dst, src, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SUB r, r/m (0x2B)
pub fn sub_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let result = dst.wrapping_sub(src);
    vcpu.set_reg(reg, result, op_size);
    vcpu.set_lazy_sub(dst, src, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SUB AL, imm8 (0x2C)
pub fn sub_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let imm = ctx.consume_u8()? as u64;
    let al = vcpu.regs.rax & 0xFF;
    let result = al.wrapping_sub(imm);
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (result & 0xFF);
    vcpu.set_lazy_sub(al, imm, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SUB rAX, imm (0x2D)
pub fn sub_rax_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };
    let rax = vcpu.get_reg(0, op_size);
    let result = rax.wrapping_sub(imm);
    vcpu.set_reg(0, result, op_size);
    vcpu.set_lazy_sub(rax, imm, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SBB r/m8, r8 (0x18) - Subtract with Borrow
pub fn sbb_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };

    if is_memory {
        let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
        let result = dst.wrapping_sub(src).wrapping_sub(cf_val) & 0xFF;
        vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
        flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, src, cf_in, result, 1);
    } else {
        let dst = vcpu.get_reg8(rm, has_rex);
        let result = dst.wrapping_sub(src).wrapping_sub(cf_val) & 0xFF;
        vcpu.set_reg8(rm, result, has_rex);
        flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, src, cf_in, result, 1);
    }
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SBB r/m, r (0x19) - Subtract with Borrow
pub fn sbb_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };

    if is_memory {
        let dst = vcpu.read_mem(addr, op_size)?;
        let result = dst.wrapping_sub(src).wrapping_sub(cf_val);
        vcpu.write_mem(addr, result, op_size)?;
        flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, src, cf_in, result, op_size);
    } else {
        let dst = vcpu.get_reg(rm, op_size);
        let result = dst.wrapping_sub(src).wrapping_sub(cf_val);
        vcpu.set_reg(rm, result, op_size);
        flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, src, cf_in, result, op_size);
    }
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SBB r8, r/m8 (0x1A) - Subtract with Borrow
pub fn sbb_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg8(reg, has_rex);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };

    let src = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
    } else {
        vcpu.get_reg8(rm, has_rex)
    };
    let result = dst.wrapping_sub(src).wrapping_sub(cf_val) & 0xFF;
    vcpu.set_reg8(reg, result, has_rex);
    flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, src, cf_in, result, 1);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SBB r, r/m (0x1B) - Subtract with Borrow
pub fn sbb_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };

    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let result = dst.wrapping_sub(src).wrapping_sub(cf_val);
    vcpu.set_reg(reg, result, op_size);
    flags::update_flags_sbb(&mut vcpu.regs.rflags, dst, src, cf_in, result, op_size);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SBB AL, imm8 (0x1C) - Subtract with Borrow
pub fn sbb_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let imm = ctx.consume_u8()? as u64;
    let al = vcpu.regs.rax & 0xFF;
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };
    let result = al.wrapping_sub(imm).wrapping_sub(cf_val);
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (result & 0xFF);
    flags::update_flags_sbb(&mut vcpu.regs.rflags, al, imm, cf_in, result, 1);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SBB rAX, imm (0x1D) - Subtract with Borrow
pub fn sbb_rax_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };
    let rax = vcpu.get_reg(0, op_size);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };
    let result = rax.wrapping_sub(imm).wrapping_sub(cf_val);
    vcpu.set_reg(0, result, op_size);
    flags::update_flags_sbb(&mut vcpu.regs.rflags, rax, imm, cf_in, result, op_size);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
