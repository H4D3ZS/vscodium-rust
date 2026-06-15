//! MOV instructions (GPR data movement).

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// MOV r8, imm8 (0xB0-0xB7)
pub fn mov_r8_imm8(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let reg = (opcode - 0xB0) | ctx.rex_b();
    let imm = ctx.consume_u8()?;
    let has_rex = ctx.rex.is_some();
    vcpu.set_reg8(reg, imm as u64, has_rex);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r16/32/64, imm16/32/64 (0xB8-0xBF)
pub fn mov_r_imm(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let reg = (opcode - 0xB8) | ctx.rex_b();
    let imm = ctx.consume_imm(ctx.op_size)?;
    vcpu.set_reg(reg, imm, ctx.op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Read the absolute offset operand of a `MOV moffs` instruction (its width is
/// the effective address size: 16-bit in real/16-bit mode, 32-bit in 32-bit
/// mode, 64-bit in long mode; toggled by a 0x67 prefix) and add the segment
/// base (DS by default, or an override). In long mode DS.base is 0, so the base
/// add is a no-op there.
fn moffs_addr(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<u64> {
    let cs = &vcpu.sregs.cs;
    let off = if cs.l {
        if ctx.address_size_override {
            ctx.consume_u32()? as u64
        } else {
            ctx.consume_u64()?
        }
    } else if cs.db {
        if ctx.address_size_override {
            ctx.consume_u16()? as u64
        } else {
            ctx.consume_u32()? as u64
        }
    } else if ctx.address_size_override {
        ctx.consume_u32()? as u64
    } else {
        ctx.consume_u16()? as u64
    };
    Ok(vcpu
        .get_segment_base(ctx.segment_override)
        .wrapping_add(off))
}

/// MOV AL, moffs8 (0xA0) - Load byte from absolute address
pub fn mov_al_moffs(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let addr = moffs_addr(vcpu, ctx)?;
    let value = vcpu.mmu.read_u8(addr, &vcpu.sregs)?;
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (value as u64);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV rAX, moffs (0xA1) - Load word/dword/qword from absolute address
pub fn mov_rax_moffs(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let addr = moffs_addr(vcpu, ctx)?;
    let value = vcpu.read_mem(addr, ctx.op_size)?;
    vcpu.set_reg(0, value, ctx.op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV moffs8, AL (0xA2) - Store byte to absolute address
pub fn mov_moffs_al(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let addr = moffs_addr(vcpu, ctx)?;
    vcpu.mmu.write_u8(addr, vcpu.regs.rax as u8, &vcpu.sregs)?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV moffs, rAX (0xA3) - Store word/dword/qword to absolute address
pub fn mov_moffs_rax(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let addr = moffs_addr(vcpu, ctx)?;
    vcpu.write_mem(addr, vcpu.get_reg(0, ctx.op_size), ctx.op_size)?;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r/m8, r8 (0x88)
pub fn mov_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let value = vcpu.get_reg8(reg, has_rex);

    if is_memory {
        vcpu.mmu.write_u8(addr, value as u8, &vcpu.sregs)?;
    } else {
        vcpu.set_reg8(rm, value, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r/m, r (0x89)
pub fn mov_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let value = vcpu.get_reg(reg, op_size);

    if is_memory {
        vcpu.write_mem(addr, value, op_size)?;
    } else {
        vcpu.set_reg(rm, value, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r8, r/m8 (0x8A)
pub fn mov_r8_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
    } else {
        vcpu.get_reg8(rm, has_rex)
    };
    vcpu.set_reg8(reg, value, has_rex);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r, r/m (0x8B)
pub fn mov_r_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    vcpu.set_reg(reg, value, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r/m, Sreg (0x8C)
pub fn mov_rm_sreg(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (sreg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let value = vcpu.get_sreg(sreg);

    if is_memory {
        vcpu.mmu.write_u16(addr, value, &vcpu.sregs)?;
    } else {
        let reg_size = if op_size == 8 {
            8
        } else if op_size == 4 {
            4
        } else {
            2
        };
        vcpu.set_reg(rm, value as u64, reg_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV Sreg, r/m16 (0x8E)
pub fn mov_sreg_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (sreg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.mmu.read_u16(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg(rm, 2) as u16
    };
    vcpu.set_sreg(sreg, value);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r/m8, imm8 (0xC6 /0) or XABORT (0xC6 F8 imm8)
pub fn mov_rm8_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    // Check for XABORT (C6 F8 imm8) - ModRM F8 has reg=7
    let modrm = ctx.peek_u8()?;
    let reg = (modrm >> 3) & 0x07;

    if reg == 7 {
        // XABORT - abort transaction with status
        ctx.consume_u8()?; // consume ModRM
        let _status = ctx.consume_u8()?; // status code
        // TSX not supported - XABORT has no effect outside transaction
        // In a real transaction, this would jump to fallback
        vcpu.regs.rip += ctx.cursor as u64;
        return Ok(None);
    }

    ctx.rip_relative_offset = 1;
    let (_, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let imm = ctx.consume_u8()?;

    if is_memory {
        vcpu.mmu.write_u8(addr, imm, &vcpu.sregs)?;
    } else {
        vcpu.set_reg8(rm, imm as u64, has_rex);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOV r/m, imm (0xC7 /0) or XBEGIN (0xC7 F8 rel32)
pub fn mov_rm_imm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Check for XBEGIN (C7 F8 rel32) - ModRM F8 has mod=11, reg=7, r/m=0
    let modrm = ctx.peek_u8()?;
    let reg = (modrm >> 3) & 0x07;

    if reg == 7 {
        // XBEGIN - begin transaction
        ctx.consume_u8()?; // consume ModRM
        let rel32 = ctx.consume_u32()? as i32;
        // TSX not supported - always abort immediately (this is valid behavior)
        // Set EAX to abort status code: we use 0 (capacity abort, retry may help)
        vcpu.regs.rax = 0;
        // Jump to fallback address
        let fallback = (vcpu.regs.rip as i64)
            .wrapping_add(ctx.cursor as i64)
            .wrapping_add(rel32 as i64) as u64;
        vcpu.regs.rip = fallback;
        return Ok(None);
    }

    let op_size = ctx.op_size;
    let imm_size = if op_size == 8 { 4 } else { op_size };
    ctx.rip_relative_offset = imm_size as usize;
    let (_, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let imm = ctx.consume_imm(imm_size)?;
    let imm = if op_size == 8 {
        imm as i32 as i64 as u64
    } else {
        imm
    };

    // Tolerate MOV r64, imm64 encoded with C7 /0 when the upper dword is sign-extension.
    if op_size == 8 && (modrm >> 6) == 3 && ctx.cursor + 4 <= ctx.bytes_len {
        let sign = if (imm as i64) < 0 { 0xFF } else { 0x00 };
        if ctx.bytes[ctx.cursor..ctx.cursor + 4]
            .iter()
            .all(|b| *b == sign)
        {
            ctx.cursor += 4;
        }
    }

    if is_memory {
        vcpu.write_mem(addr, imm, op_size)?;
    } else {
        vcpu.set_reg(rm, imm, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
