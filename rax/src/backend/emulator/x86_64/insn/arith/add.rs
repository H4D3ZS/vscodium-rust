//! ADD and ADC instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

fn add_with_carry_unsigned(dest: u64, src: u64, carry_in: bool, op_size: u8) -> (u64, bool) {
    let bits = (op_size as u32) * 8;
    let full_sum = (dest as u128) + (src as u128) + if carry_in { 1 } else { 0 };
    let mask = (1u128 << bits) - 1;
    let result = (full_sum & mask) as u64;
    let carry_out = full_sum > mask;
    (result, carry_out)
}

/// ADD r/m8, r8 (0x00)
pub fn add_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex);

    if is_memory {
        let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
        let result = dst.wrapping_add(src) & 0xFF;
        vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
        vcpu.set_lazy_add(dst, src, result, 1);
    } else {
        let dst = vcpu.get_reg8(rm, has_rex);
        let result = dst.wrapping_add(src) & 0xFF;
        vcpu.set_reg8(rm, result, has_rex);
        vcpu.set_lazy_add(dst, src, result, 1);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADD r/m, r (0x01)
pub fn add_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);

    if is_memory {
        let dst = vcpu.read_mem(addr, op_size)?;
        let result = dst.wrapping_add(src);
        vcpu.write_mem(addr, result, op_size)?;
        vcpu.set_lazy_add(dst, src, result, op_size);
    } else {
        let dst = vcpu.get_reg(rm, op_size);
        let result = dst.wrapping_add(src);
        vcpu.set_reg(rm, result, op_size);
        vcpu.set_lazy_add(dst, src, result, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADD r8, r/m8 (0x02)
pub fn add_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg8(reg, has_rex);

    let src = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
    } else {
        vcpu.get_reg8(rm, has_rex)
    };
    let result = dst.wrapping_add(src) & 0xFF;
    vcpu.set_reg8(reg, result, has_rex);
    vcpu.set_lazy_add(dst, src, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADD r, r/m (0x03)
pub fn add_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);
    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    let result = dst.wrapping_add(src);
    vcpu.set_reg(reg, result, op_size);
    vcpu.set_lazy_add(dst, src, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADD AL, imm8 (0x04)
pub fn add_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let imm = ctx.consume_u8()? as u64;
    let al = vcpu.regs.rax & 0xFF;
    let result = al.wrapping_add(imm);
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (result & 0xFF);
    vcpu.set_lazy_add(al, imm, result, 1);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADD rAX, imm (0x05)
pub fn add_rax_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };
    let rax = vcpu.get_reg(0, op_size);
    let result = rax.wrapping_add(imm);
    vcpu.set_reg(0, result, op_size);
    vcpu.set_lazy_add(rax, imm, result, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADC r/m8, r8 (0x10) - Add with Carry
pub fn adc_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };

    if is_memory {
        let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
        let result = dst.wrapping_add(src).wrapping_add(cf_val) & 0xFF;
        vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
        flags::update_flags_adc(&mut vcpu.regs.rflags, dst, src, cf_in, result, 1);
    } else {
        let dst = vcpu.get_reg8(rm, has_rex);
        let result = dst.wrapping_add(src).wrapping_add(cf_val) & 0xFF;
        vcpu.set_reg8(rm, result, has_rex);
        flags::update_flags_adc(&mut vcpu.regs.rflags, dst, src, cf_in, result, 1);
    }
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADC r/m, r (0x11) - Add with Carry
pub fn adc_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };

    if is_memory {
        let dst = vcpu.read_mem(addr, op_size)?;
        let result = dst.wrapping_add(src).wrapping_add(cf_val);
        vcpu.write_mem(addr, result, op_size)?;
        flags::update_flags_adc(&mut vcpu.regs.rflags, dst, src, cf_in, result, op_size);
    } else {
        let dst = vcpu.get_reg(rm, op_size);
        let result = dst.wrapping_add(src).wrapping_add(cf_val);
        vcpu.set_reg(rm, result, op_size);
        flags::update_flags_adc(&mut vcpu.regs.rflags, dst, src, cf_in, result, op_size);
    }
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADC r8, r/m8 (0x12) - Add with Carry
pub fn adc_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
    let result = dst.wrapping_add(src).wrapping_add(cf_val) & 0xFF;
    vcpu.set_reg8(reg, result, has_rex);
    flags::update_flags_adc(&mut vcpu.regs.rflags, dst, src, cf_in, result, 1);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADC r, r/m (0x13) - Add with Carry
pub fn adc_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
    let result = dst.wrapping_add(src).wrapping_add(cf_val);
    vcpu.set_reg(reg, result, op_size);
    flags::update_flags_adc(&mut vcpu.regs.rflags, dst, src, cf_in, result, op_size);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADCX r32/r64, r/m32/r/m64 (0x0F 0x38 0xF6 /r with 0x66 prefix)
pub fn adcx_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = if ctx.rex_w() { 8 } else { 4 };
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);
    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let (result, carry_out) = add_with_carry_unsigned(dst, src, cf_in, op_size);
    vcpu.set_reg(reg, result, op_size);

    if carry_out {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADOX r32/r64, r/m32/r/m64 (0x0F 0x38 0xF6 /r with 0xF3 prefix)
pub fn adox_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = if ctx.rex_w() { 8 } else { 4 };
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let dst = vcpu.get_reg(reg, op_size);
    let src = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };
    vcpu.materialize_flags(); // Need OF
    let of_in = (vcpu.regs.rflags & flags::bits::OF) != 0;
    let (result, carry_out) = add_with_carry_unsigned(dst, src, of_in, op_size);
    vcpu.set_reg(reg, result, op_size);

    if carry_out {
        vcpu.regs.rflags |= flags::bits::OF;
    } else {
        vcpu.regs.rflags &= !flags::bits::OF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADC AL, imm8 (0x14) - Add with Carry
pub fn adc_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let imm = ctx.consume_u8()? as u64;
    let al = vcpu.regs.rax & 0xFF;
    vcpu.materialize_flags(); // Need CF
    let cf_in = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let cf_val = if cf_in { 1u64 } else { 0 };
    let result = al.wrapping_add(imm).wrapping_add(cf_val);
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (result & 0xFF);
    flags::update_flags_adc(&mut vcpu.regs.rflags, al, imm, cf_in, result, 1);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// ADC rAX, imm (0x15) - Add with Carry
pub fn adc_rax_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
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
    let result = rax.wrapping_add(imm).wrapping_add(cf_val);
    vcpu.set_reg(0, result, op_size);
    flags::update_flags_adc(&mut vcpu.regs.rflags, rax, imm, cf_in, result, op_size);
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
