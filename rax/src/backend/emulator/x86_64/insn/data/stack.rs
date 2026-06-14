//! Stack instructions: PUSH, POP, PUSHA, POPA.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// PUSH r64 (0x50-0x57)
pub fn push_r64(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let reg = (opcode - 0x50) | ctx.any_rex_b();
    let op_size = stack_op_size(vcpu, ctx);
    let value = vcpu.get_reg(reg, op_size);
    match op_size {
        2 => vcpu.push16(value as u16)?,
        4 => vcpu.push32(value as u32)?,
        8 => vcpu.push64(value)?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid PUSH r op size: {}",
                op_size
            )));
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

fn segment_op_size(vcpu: &X86_64Vcpu, ctx: &InsnContext) -> u8 {
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0;
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode {
        if ctx.operand_size_override { 2 } else { 8 }
    } else {
        let default_16bit = !vcpu.sregs.cs.db;
        let is_16bit = default_16bit ^ ctx.operand_size_override;
        if is_16bit { 2 } else { 4 }
    }
}

fn stack_op_size(vcpu: &X86_64Vcpu, ctx: &InsnContext) -> u8 {
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0;
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode {
        if ctx.operand_size_override { 2 } else { 8 }
    } else {
        let default_16bit = !vcpu.sregs.cs.db;
        let is_16bit = default_16bit ^ ctx.operand_size_override;
        if is_16bit { 2 } else { 4 }
    }
}

fn segment_invalid_in_64bit(sreg: u8) -> bool {
    matches!(sreg, 0 | 1 | 2 | 3)
}

fn segment_name(sreg: u8) -> &'static str {
    match sreg {
        0 => "ES",
        1 => "CS",
        2 => "SS",
        3 => "DS",
        4 => "FS",
        5 => "GS",
        _ => "UNKNOWN",
    }
}

/// PUSH Sreg (ES/CS/SS/DS/FS/GS)
pub fn push_sreg(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    sreg: u8,
) -> Result<Option<VcpuExit>> {
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0;
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode && segment_invalid_in_64bit(sreg) {
        // PUSH ES/CS/SS/DS invalid in 64-bit mode - inject #UD
        // Don't advance RIP - exception should point to faulting instruction
        vcpu.inject_exception(6, None)?; // #UD = vector 6
        return Ok(None);
    }

    let op_size = segment_op_size(vcpu, ctx);
    let value = vcpu.get_sreg(sreg) as u64;
    match op_size {
        2 => vcpu.push16(value as u16)?,
        4 => vcpu.push32(value as u32)?,
        8 => vcpu.push64(value)?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid PUSH Sreg size: {}",
                op_size
            )));
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POP Sreg (ES/SS/DS/FS/GS)
pub fn pop_sreg(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    sreg: u8,
) -> Result<Option<VcpuExit>> {
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0;
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;

    if in_64bit_mode && segment_invalid_in_64bit(sreg) {
        // POP ES/CS/SS/DS invalid in 64-bit mode - inject #UD
        // Don't advance RIP - exception should point to faulting instruction
        vcpu.inject_exception(6, None)?; // #UD = vector 6
        return Ok(None);
    }

    let op_size = segment_op_size(vcpu, ctx);
    let value = match op_size {
        2 => vcpu.pop16()? as u64,
        4 => vcpu.pop32()? as u64,
        8 => vcpu.pop64()?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid POP Sreg size: {}",
                op_size
            )));
        }
    };
    vcpu.set_sreg(sreg, (value & 0xFFFF) as u16);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PUSH imm8 (0x6A)
pub fn push_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = stack_op_size(vcpu, ctx);
    let imm = ctx.consume_u8()? as i8 as i64 as u64;
    match op_size {
        2 => vcpu.push16(imm as u16)?,
        4 => vcpu.push32(imm as u32)?,
        8 => vcpu.push64(imm)?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid PUSH imm8 op size: {}",
                op_size
            )));
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PUSH imm32 (0x68)
pub fn push_imm32(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = stack_op_size(vcpu, ctx);
    let imm = if op_size == 2 {
        ctx.consume_u16()? as i16 as i64 as u64
    } else {
        ctx.consume_u32()? as i32 as i64 as u64
    };
    match op_size {
        2 => vcpu.push16(imm as u16)?,
        4 => vcpu.push32(imm as u32)?,
        8 => vcpu.push64(imm)?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid PUSH imm32 op size: {}",
                op_size
            )));
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POP r64 (0x58-0x5F)
pub fn pop_r64(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    opcode: u8,
) -> Result<Option<VcpuExit>> {
    let reg = (opcode - 0x58) | ctx.any_rex_b();
    let op_size = stack_op_size(vcpu, ctx);

    // Special handling for POP RSP (reg 4)
    // Intel manual: "The POP ESP instruction increments the stack pointer (ESP)
    // before data at the old top of stack is written into the destination."
    // But since DEST is RSP itself, the increment to old RSP is effectively discarded.
    // Final RSP = value read from [old_RSP]
    if reg == 4 {
        // POP RSP - special case: just read value and set RSP to it
        let value = match op_size {
            2 => vcpu.mmu.read_u16(vcpu.regs.rsp, &vcpu.sregs)? as u64,
            4 => vcpu.mmu.read_u32(vcpu.regs.rsp, &vcpu.sregs)? as u64,
            8 => vcpu.mmu.read_u64(vcpu.regs.rsp, &vcpu.sregs)?,
            _ => {
                return Err(Error::Emulator(format!(
                    "invalid POP RSP op size: {}",
                    op_size
                )));
            }
        };
        // For POP RSP, the final RSP is just the value read from stack
        // The normal +8 increment is discarded because we're writing to RSP
        vcpu.regs.rsp = value;
        vcpu.regs.rip += ctx.cursor as u64;
        return Ok(None);
    }

    let value = match op_size {
        2 => vcpu.pop16()? as u64,
        4 => vcpu.pop32()? as u64,
        8 => vcpu.pop64()?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid POP r op size: {}",
                op_size
            )));
        }
    };
    vcpu.set_reg(reg, value, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POP r/m64 (0x8F /0)
pub fn pop_rm(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = stack_op_size(vcpu, ctx);

    let (_reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    // Pop value based on operand size
    let value = match op_size {
        2 => vcpu.pop16()? as u64,
        4 => vcpu.pop32()? as u64,
        8 => vcpu.pop64()?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid POP r/m op size: {}",
                op_size
            )));
        }
    };

    if is_memory {
        vcpu.write_mem(addr, value, op_size)?;
    } else {
        vcpu.set_reg(rm, value, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// PUSHA/PUSHAD (0x60) - Push all general-purpose registers
pub fn pusha(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Check if we're in 64-bit mode - PUSHA/PUSHAD is invalid in 64-bit mode
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0; // EFER.LMA = bit 10
    let cs_l = vcpu.sregs.cs.l; // CS.L indicates 64-bit code segment

    if in_long_mode && cs_l {
        // PUSHA/PUSHAD invalid in 64-bit mode - inject #UD
        // Don't advance RIP - exception should point to faulting instruction
        vcpu.inject_exception(6, None)?; // #UD = vector 6
        return Ok(None);
    }

    // Determine operand size: 0x66 prefix TOGGLES the default operand size
    // CS.D (db flag) determines default: D=0 means 16-bit default, D=1 means 32-bit default
    // The 0x66 prefix inverts the default
    let default_16bit = !vcpu.sregs.cs.db;
    let is_16bit = default_16bit ^ ctx.operand_size_override;

    // Save original SP/ESP before any pushes
    let original_sp = vcpu.regs.rsp;

    if is_16bit {
        // PUSHA - push 16-bit registers: AX, CX, DX, BX, SP, BP, SI, DI
        let ax = (vcpu.regs.rax & 0xFFFF) as u16;
        let cx = (vcpu.regs.rcx & 0xFFFF) as u16;
        let dx = (vcpu.regs.rdx & 0xFFFF) as u16;
        let bx = (vcpu.regs.rbx & 0xFFFF) as u16;
        let sp = (original_sp & 0xFFFF) as u16;
        let bp = (vcpu.regs.rbp & 0xFFFF) as u16;
        let si = (vcpu.regs.rsi & 0xFFFF) as u16;
        let di = (vcpu.regs.rdi & 0xFFFF) as u16;

        vcpu.push16(ax)?;
        vcpu.push16(cx)?;
        vcpu.push16(dx)?;
        vcpu.push16(bx)?;
        vcpu.push16(sp)?;
        vcpu.push16(bp)?;
        vcpu.push16(si)?;
        vcpu.push16(di)?;
    } else {
        // PUSHAD - push 32-bit registers: EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI
        let eax = (vcpu.regs.rax & 0xFFFFFFFF) as u32;
        let ecx = (vcpu.regs.rcx & 0xFFFFFFFF) as u32;
        let edx = (vcpu.regs.rdx & 0xFFFFFFFF) as u32;
        let ebx = (vcpu.regs.rbx & 0xFFFFFFFF) as u32;
        let esp = (original_sp & 0xFFFFFFFF) as u32;
        let ebp = (vcpu.regs.rbp & 0xFFFFFFFF) as u32;
        let esi = (vcpu.regs.rsi & 0xFFFFFFFF) as u32;
        let edi = (vcpu.regs.rdi & 0xFFFFFFFF) as u32;

        vcpu.push32(eax)?;
        vcpu.push32(ecx)?;
        vcpu.push32(edx)?;
        vcpu.push32(ebx)?;
        vcpu.push32(esp)?;
        vcpu.push32(ebp)?;
        vcpu.push32(esi)?;
        vcpu.push32(edi)?;
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// POPA/POPAD (0x61) - Pop all general-purpose registers
pub fn popa(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Check if we're in 64-bit mode - POPA/POPAD is invalid in 64-bit mode
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0; // EFER.LMA = bit 10
    let cs_l = vcpu.sregs.cs.l; // CS.L indicates 64-bit code segment

    if in_long_mode && cs_l {
        // POPA/POPAD invalid in 64-bit mode - inject #UD
        // Don't advance RIP - exception should point to faulting instruction
        vcpu.inject_exception(6, None)?; // #UD = vector 6
        return Ok(None);
    }

    // Determine operand size: 0x66 prefix TOGGLES the default operand size
    // CS.D (db flag) determines default: D=0 means 16-bit default, D=1 means 32-bit default
    let default_16bit = !vcpu.sregs.cs.db;
    let is_16bit = default_16bit ^ ctx.operand_size_override;

    if is_16bit {
        // POPA - pop 16-bit registers: DI, SI, BP, skip SP, BX, DX, CX, AX
        let di = vcpu.pop16()?;
        let si = vcpu.pop16()?;
        let bp = vcpu.pop16()?;
        let _ = vcpu.pop16()?; // Skip SP value on stack
        let bx = vcpu.pop16()?;
        let dx = vcpu.pop16()?;
        let cx = vcpu.pop16()?;
        let ax = vcpu.pop16()?;

        // Update only the lower 16 bits of registers
        vcpu.regs.rdi = (vcpu.regs.rdi & !0xFFFF) | (di as u64);
        vcpu.regs.rsi = (vcpu.regs.rsi & !0xFFFF) | (si as u64);
        vcpu.regs.rbp = (vcpu.regs.rbp & !0xFFFF) | (bp as u64);
        vcpu.regs.rbx = (vcpu.regs.rbx & !0xFFFF) | (bx as u64);
        vcpu.regs.rdx = (vcpu.regs.rdx & !0xFFFF) | (dx as u64);
        vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | (cx as u64);
        vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | (ax as u64);
    } else {
        // POPAD - pop 32-bit registers: EDI, ESI, EBP, skip ESP, EBX, EDX, ECX, EAX
        let edi = vcpu.pop32()?;
        let esi = vcpu.pop32()?;
        let ebp = vcpu.pop32()?;
        let _ = vcpu.pop32()?; // Skip ESP value on stack
        let ebx = vcpu.pop32()?;
        let edx = vcpu.pop32()?;
        let ecx = vcpu.pop32()?;
        let eax = vcpu.pop32()?;

        // Update only the lower 32 bits of registers, preserving upper 32 bits
        vcpu.regs.rdi = (vcpu.regs.rdi & 0xFFFFFFFF00000000) | (edi as u64);
        vcpu.regs.rsi = (vcpu.regs.rsi & 0xFFFFFFFF00000000) | (esi as u64);
        vcpu.regs.rbp = (vcpu.regs.rbp & 0xFFFFFFFF00000000) | (ebp as u64);
        vcpu.regs.rbx = (vcpu.regs.rbx & 0xFFFFFFFF00000000) | (ebx as u64);
        vcpu.regs.rdx = (vcpu.regs.rdx & 0xFFFFFFFF00000000) | (edx as u64);
        vcpu.regs.rcx = (vcpu.regs.rcx & 0xFFFFFFFF00000000) | (ecx as u64);
        vcpu.regs.rax = (vcpu.regs.rax & 0xFFFFFFFF00000000) | (eax as u64);
    }

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
