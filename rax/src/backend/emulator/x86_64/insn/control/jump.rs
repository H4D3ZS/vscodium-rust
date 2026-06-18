//! Jump instructions: JMP, Jcc.

use crate::cpu::VcpuExit;
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::call::{near_branch_op_size, validate_far_selector};

/// Truncate an instruction pointer to the operand-size width: a 16-bit jump
/// wraps IP within 64 KiB (real/16-bit mode), 32-bit within 4 GiB; 64-bit is
/// unchanged. Relative jumps use the operand size for the displacement width
/// and the IP wrap.
fn mask_ip(ip: u64, op_size: u8) -> u64 {
    match op_size {
        2 => ip & 0xFFFF,
        4 => ip & 0xFFFF_FFFF,
        _ => ip,
    }
}

/// JMP rel8 (0xEB)
pub fn jmp_rel8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = near_branch_op_size(vcpu, ctx);
    let disp = ctx.consume_u8()? as i8 as i64;
    let target = (vcpu.regs.rip as i64 + ctx.cursor as i64 + disp) as u64;
    vcpu.regs.rip = mask_ip(target, op_size);
    Ok(None)
}

/// JMP rel16/rel32 (0xE9). The displacement is rel16 for a 16-bit operand size,
/// else rel32 (sign-extended, including in 64-bit mode).
pub fn jmp_rel32(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = near_branch_op_size(vcpu, ctx);
    let disp = if op_size == 2 {
        ctx.consume_u16()? as i16 as i64
    } else {
        ctx.consume_u32()? as i32 as i64
    };
    let target = (vcpu.regs.rip as i64 + ctx.cursor as i64 + disp) as u64;
    vcpu.regs.rip = mask_ip(target, op_size);
    Ok(None)
}

/// JMPABS imm64 (APX REX2 + 0xA1)
pub fn jmp_abs(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.regs.rip = ctx.consume_u64()?;
    Ok(None)
}

/// Jcc rel8 (0x70-0x7F)
pub fn jcc_rel8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext, cc: u8) -> Result<Option<VcpuExit>> {
    let op_size = near_branch_op_size(vcpu, ctx);
    let disp = ctx.consume_u8()? as i8 as i64;
    let taken_target = (vcpu.regs.rip as i64 + ctx.cursor as i64 + disp) as u64;
    let fall_through = vcpu.regs.rip + ctx.cursor as u64;

    // Evaluate condition and branch
    if vcpu.check_condition(cc) {
        vcpu.regs.rip = mask_ip(taken_target, op_size);
    } else {
        vcpu.regs.rip = mask_ip(fall_through, op_size);
    }
    Ok(None)
}

/// Jcc rel16/rel32 (0x0F 0x80-0x8F)
pub fn jcc_rel32(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext, cc: u8) -> Result<Option<VcpuExit>> {
    let op_size = near_branch_op_size(vcpu, ctx);
    let disp = if op_size == 2 {
        ctx.consume_u16()? as i16 as i64
    } else {
        ctx.consume_u32()? as i32 as i64
    };
    let taken_target = (vcpu.regs.rip as i64 + ctx.cursor as i64 + disp) as u64;
    let fall_through = vcpu.regs.rip + ctx.cursor as u64;

    // Evaluate condition and branch
    if vcpu.check_condition(cc) {
        vcpu.regs.rip = mask_ip(taken_target, op_size);
    } else {
        vcpu.regs.rip = mask_ip(fall_through, op_size);
    }
    Ok(None)
}

/// JMP FAR ptr16:16/ptr16:32 (0xEA)
/// Far jump with immediate pointer - loads segment:offset from instruction.
/// Note: This opcode is invalid in 64-bit mode, but we emulate it for compatibility.
pub fn jmp_far_ptr(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let offset = match ctx.op_size {
        2 => ctx.consume_u16()? as u64,
        4 => ctx.consume_u32()? as u64,
        _ => {
            return Err(Error::Emulator(format!(
                "JMP FAR ptr16:16/ptr16:32 invalid operand size: {}",
                ctx.op_size
            )));
        }
    };
    let selector = ctx.consume_u16()?;
    validate_far_selector(vcpu, selector)?;
    // A far JMP reloads CS from the target descriptor; the CPL is unchanged
    // (it does not switch privilege the way a far CALL/interrupt through a gate
    // can). The mode-establishing flush jump after `mov cr0` (real→protected,
    // or the protected→long handoff) lands here — just load the descriptor.

    // Load CS:IP from the real descriptor (lenient: flat fallback for a sparse
    // descriptor table so legacy flat-segment code keeps working).
    vcpu.load_code_segment_lenient(selector);
    vcpu.regs.rip = offset;
    Ok(None)
}

/// JMP FAR m16:16/m16:32/m16:64 (0xFF /5)
/// Far jump with memory indirect - loads segment:offset from memory.
/// Offset size follows the operand-size attribute (16/32 in non-64-bit, 16/32/64 in 64-bit).
pub fn jmp_far_mem(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let modrm_start = ctx.cursor;
    let _modrm = ctx.consume_u8()?;

    // Get memory address
    let (addr, extra) = vcpu.decode_modrm_addr(ctx, modrm_start)?;
    ctx.cursor = modrm_start + 1 + extra;

    let offset_size = ctx.op_size;

    // Read offset and selector from memory
    let offset = vcpu.read_mem(addr, offset_size)?;
    let selector = vcpu.mmu.read_u16(addr + offset_size as u64, &vcpu.sregs)?;
    validate_far_selector(vcpu, selector)?;
    // A far JMP reloads CS from the target descriptor; the CPL is unchanged
    // (it does not switch privilege the way a far CALL/interrupt through a gate
    // can). The mode-establishing flush jump after `mov cr0` (real→protected,
    // or the protected→long handoff) lands here — just load the descriptor.

    // Load CS:IP from the real descriptor (lenient: flat fallback).
    vcpu.load_code_segment_lenient(selector);
    vcpu.regs.rip = offset;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use vm_memory::{GuestAddress, GuestMemoryMmap};

    use crate::backend::emulator::x86_64::cpu::MAX_INSN_LEN;
    use crate::backend::emulator::x86_64::flags;

    const HIGH_RIP: u64 = 0xFFFF_FFFF_8124_9EE0;

    fn long_mode_vcpu(rflags: u64) -> X86_64Vcpu {
        let mem =
            Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x10000)]).unwrap());
        let mut vcpu = X86_64Vcpu::new(0, mem);
        vcpu.regs.rip = HIGH_RIP;
        vcpu.regs.rflags = rflags;
        vcpu.sregs.cs.l = true;
        vcpu.sregs.cs.db = false;
        vcpu
    }

    fn context(instruction: &[u8], opcode: u8) -> InsnContext {
        let mut bytes = [0; MAX_INSN_LEN];
        bytes[..instruction.len()].copy_from_slice(instruction);
        InsnContext {
            bytes,
            bytes_len: instruction.len(),
            cursor: 1,
            rex: None,
            rex2: None,
            operand_size_override: false,
            address_size_override: false,
            rep_prefix: None,
            op_size: 4,
            rip_relative_offset: 0,
            segment_override: None,
            evex: None,
            opcode,
        }
    }

    #[test]
    fn long_mode_jmp_rel8_preserves_high_rip_bits() {
        let mut vcpu = long_mode_vcpu(0x2);
        let mut ctx = context(&[0xEB, 0x05], 0xEB);

        jmp_rel8(&mut vcpu, &mut ctx).unwrap();

        assert_eq!(vcpu.regs.rip, HIGH_RIP + 2 + 5);
    }

    #[test]
    fn long_mode_jmp_rel32_preserves_high_rip_bits() {
        let mut vcpu = long_mode_vcpu(0x2);
        let mut ctx = context(&[0xE9, 0x05, 0x00, 0x00, 0x00], 0xE9);

        jmp_rel32(&mut vcpu, &mut ctx).unwrap();

        assert_eq!(vcpu.regs.rip, HIGH_RIP + 5 + 5);
    }

    #[test]
    fn long_mode_jcc_not_taken_preserves_high_rip_bits() {
        let mut vcpu = long_mode_vcpu(0x2 | flags::bits::ZF);
        let mut ctx = context(&[0x75, 0x05], 0x75);

        jcc_rel8(&mut vcpu, &mut ctx, 0x5).unwrap();

        assert_eq!(vcpu.regs.rip, HIGH_RIP + 2);
    }
}
