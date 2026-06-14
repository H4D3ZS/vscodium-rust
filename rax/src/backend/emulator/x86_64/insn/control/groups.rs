//! Opcode group instructions: Group 4 (0xFE), Group 5 (0xFF).

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::call::validate_far_selector;

/// Group 4: INC/DEC r/m8 (0xFE)
pub fn group4(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();

    match op {
        0 => {
            // INC r/m8 - preserves CF (lazy OF/SF/ZF/AF/PF).
            vcpu.resolve_lazy_cf();
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg8(rm, has_rex);
                let result = (val as u8).wrapping_add(1) as u64;
                vcpu.set_reg8(rm, result, has_rex);
                vcpu.set_lazy_inc(val, result, 1);
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                let val = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
                let result = (val as u8).wrapping_add(1) as u64;
                vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
                vcpu.set_lazy_inc(val, result, 1);
            }
            vcpu.regs.rip += ctx.cursor as u64;
        }
        1 => {
            // DEC r/m8 - preserves CF (lazy OF/SF/ZF/AF/PF).
            vcpu.resolve_lazy_cf();
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg8(rm, has_rex);
                let result = (val as u8).wrapping_sub(1) as u64;
                vcpu.set_reg8(rm, result, has_rex);
                vcpu.set_lazy_dec(val, result, 1);
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                let val = vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64;
                let result = (val as u8).wrapping_sub(1) as u64;
                vcpu.mmu.write_u8(addr, result as u8, &vcpu.sregs)?;
                vcpu.set_lazy_dec(val, result, 1);
            }
            vcpu.regs.rip += ctx.cursor as u64;
        }
        _ => {
            // 0xFE /2-7 are undefined - inject #UD exception
            eprintln!(
                "[#UD] 0xFE /{} at RIP={:#x} (undefined opcode)",
                op, vcpu.regs.rip
            );
            vcpu.inject_exception(6, None)?; // #UD = vector 6
            return Ok(None);
        }
    }
    Ok(None)
}

/// INC r/m (0xFF /0) memory-operand path (cold). CF is already locked in by the
/// caller's `resolve_lazy_cf`. Kept out-of-line so the register-direct INC/DEC
/// hot path keeps a minimal stack frame.
#[cold]
#[inline(never)]
fn group5_inc_mem(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    op_size: u8,
) -> Result<()> {
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    let val = vcpu.read_mem(addr, op_size)?;
    let result = val.wrapping_add(1);
    vcpu.write_mem(addr, result, op_size)?;
    vcpu.set_lazy_inc(val, result, op_size);
    Ok(())
}

/// DEC r/m (0xFF /1) memory-operand path (cold). See `group5_inc_mem`.
#[cold]
#[inline(never)]
fn group5_dec_mem(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    op_size: u8,
) -> Result<()> {
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;
    let val = vcpu.read_mem(addr, op_size)?;
    let result = val.wrapping_sub(1);
    vcpu.write_mem(addr, result, op_size)?;
    vcpu.set_lazy_dec(val, result, op_size);
    Ok(())
}

/// CALL FAR m16:16/m16:32/m16:64 (0xFF /3), cold path.
#[cold]
#[inline(never)]
fn group5_call_far(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    modrm: u8,
) -> Result<Option<VcpuExit>> {
    if modrm >> 6 == 3 {
        // CALL FAR with register operand is undefined - inject #UD
        // Don't advance RIP - exception should point to faulting instruction
        vcpu.inject_exception(6, None)?;
        return Ok(None);
    }
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;

    let offset_size = ctx.op_size;

    // Read offset and selector from memory
    let offset = vcpu.read_mem(addr, offset_size)?;
    let selector = vcpu.mmu.read_u16(addr + offset_size as u64, &vcpu.sregs)?;
    validate_far_selector(vcpu, selector)?;
    let old_cpl = vcpu.sregs.cs.selector & 0x3;
    let new_cpl = selector & 0x3;
    if new_cpl != old_cpl {
        return Err(Error::Emulator(
            "JMP FAR privilege change not supported".to_string(),
        ));
    }

    // Push return CS:IP
    let old_cs = vcpu.get_sreg(1);
    let ret_addr = vcpu.regs.rip + ctx.cursor as u64;

    match ctx.op_size {
        2 => {
            vcpu.push16(old_cs)?;
            vcpu.push16(ret_addr as u16)?;
        }
        4 => {
            vcpu.push32(old_cs as u32)?;
            vcpu.push32(ret_addr as u32)?;
        }
        8 => {
            vcpu.push64(old_cs as u64)?;
            vcpu.push64(ret_addr)?;
        }
        _ => {
            return Err(Error::Emulator(format!(
                "CALL FAR m16:16/m16:32 invalid return size: {}",
                ctx.op_size
            )));
        }
    }

    // Load new CS:IP
    vcpu.set_sreg(1, selector);
    vcpu.regs.rip = offset;
    Ok(None)
}

/// JMP FAR m16:16/m16:32/m16:64 (0xFF /5), cold path.
#[cold]
#[inline(never)]
fn group5_jmp_far(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    modrm: u8,
) -> Result<Option<VcpuExit>> {
    if modrm >> 6 == 3 {
        // JMP FAR with register operand is undefined - inject #UD
        // Don't advance RIP - exception should point to faulting instruction
        vcpu.inject_exception(6, None)?;
        return Ok(None);
    }
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;

    let offset_size = ctx.op_size;

    // Read offset and selector from memory
    let offset = vcpu.read_mem(addr, offset_size)?;
    let selector = vcpu.mmu.read_u16(addr + offset_size as u64, &vcpu.sregs)?;
    validate_far_selector(vcpu, selector)?;

    // Load new CS:IP
    vcpu.set_sreg(1, selector);
    vcpu.regs.rip = offset;
    Ok(None)
}

/// PUSH r/m16/32/64 (0xFF /6), cold path.
#[cold]
#[inline(never)]
fn group5_push(
    vcpu: &mut X86_64Vcpu,
    ctx: &mut InsnContext,
    modrm_start: usize,
    modrm: u8,
    rm: u8,
) -> Result<Option<VcpuExit>> {
    let in_long_mode = (vcpu.sregs.efer & 0x400) != 0;
    let in_64bit_mode = in_long_mode && vcpu.sregs.cs.l;
    let op_size = if in_64bit_mode {
        if ctx.operand_size_override { 2 } else { 8 }
    } else {
        let default_16bit = !vcpu.sregs.cs.db;
        let is_16bit = default_16bit ^ ctx.operand_size_override;
        if is_16bit { 2 } else { 4 }
    };

    let val = if modrm >> 6 == 3 {
        vcpu.get_reg(rm, op_size)
    } else {
        let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
        ctx.cursor = modrm_start + 1 + extra;
        vcpu.read_mem(addr, op_size)?
    };
    match op_size {
        2 => vcpu.push16(val as u16)?,
        4 => vcpu.push32(val as u32)?,
        8 => vcpu.push64(val)?,
        _ => {
            return Err(Error::Emulator(format!(
                "invalid PUSH r/m op size: {}",
                op_size
            )));
        }
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// Group 5: INC/DEC/CALL/JMP/PUSH (0xFF)
pub fn group5(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let modrm = ctx.consume_u8()?;
    let op = (modrm >> 3) & 0x07;
    let rm = (modrm & 0x07) | ctx.rex_b();
    let op_size = ctx.op_size;

    match op {
        0 => {
            // INC r/m - preserves CF. Lazy: lock in the pending op's CF, then defer
            // the OF/SF/ZF/AF/PF computation to the next flag reader.
            vcpu.resolve_lazy_cf();
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg(rm, op_size);
                let result = val.wrapping_add(1);
                vcpu.set_reg(rm, result, op_size);
                vcpu.set_lazy_inc(val, result, op_size);
            } else {
                group5_inc_mem(vcpu, ctx, modrm_start, op_size)?;
            }
            vcpu.regs.rip += ctx.cursor as u64;
        }
        1 => {
            // DEC r/m - preserves CF. Lazy: lock in the pending op's CF, then defer
            // the OF/SF/ZF/AF/PF computation to the next flag reader.
            vcpu.resolve_lazy_cf();
            if modrm >> 6 == 3 {
                let val = vcpu.get_reg(rm, op_size);
                let result = val.wrapping_sub(1);
                vcpu.set_reg(rm, result, op_size);
                vcpu.set_lazy_dec(val, result, op_size);
            } else {
                group5_dec_mem(vcpu, ctx, modrm_start, op_size)?;
            }
            vcpu.regs.rip += ctx.cursor as u64;
        }
        2 => {
            // CALL r/m64
            let target = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, 8)
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, 8)?
            };
            let ret_addr = vcpu.regs.rip + ctx.cursor as u64;

            vcpu.push64(ret_addr)?;
            vcpu.regs.rip = target;
        }
        3 => {
            // CALL FAR m16:16/m16:32/m16:64 (cold; far transfers are rare).
            return group5_call_far(vcpu, ctx, modrm_start, modrm);
        }
        4 => {
            // JMP r/m64
            let target = if modrm >> 6 == 3 {
                vcpu.get_reg(rm, 8)
            } else {
                let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
                ctx.cursor = modrm_start + 1 + extra;
                vcpu.read_mem(addr, 8)?
            };
            vcpu.regs.rip = target;
        }
        5 => {
            // JMP FAR m16:16/m16:32/m16:64 (cold; far transfers are rare).
            return group5_jmp_far(vcpu, ctx, modrm_start, modrm);
        }
        6 => {
            // PUSH r/m16/32/64 (cold; stack push pulls in mode/op-size logic).
            return group5_push(vcpu, ctx, modrm_start, modrm, rm);
        }
        _ => {
            // 0xFF /7 is undefined - inject #UD exception
            // Don't advance RIP - exception should point to faulting instruction
            vcpu.inject_exception(6, None)?; // #UD = vector 6
        }
    }
    Ok(None)
}
