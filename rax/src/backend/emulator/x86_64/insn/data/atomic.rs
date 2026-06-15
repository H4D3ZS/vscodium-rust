//! Atomic instructions: XADD, CMPXCHG.
//!
//! LOCK semantics: these read-modify-write forms may carry a LOCK prefix
//! (0xF0). On a real CPU LOCK guarantees the RMW is atomic w.r.t. other cores;
//! rax is a single-vCPU interpreter, so each instruction runs to completion
//! without interleaving and the RMW is already atomic regardless of LOCK.
//! The only architectural behaviour LOCK adds here is a decode-time legality
//! check (`X86_64Vcpu::enforce_lock_prefix`): a LOCK on a register-destination
//! XADD/CMPXCHG (ModR/M mod == 3) raises #UD before these handlers run, so the
//! register branches below are never reached with a LOCK prefix present.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// XADD r/m8, r8 (0x0F 0xC0) - Exchange and Add
pub fn xadd_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex) as u8;

    if is_memory {
        let dst = vcpu.mmu.read_u8(addr, &vcpu.sregs)?;
        let sum = dst.wrapping_add(src);
        // DEST = DEST + SRC, SRC = old DEST
        vcpu.mmu.write_u8(addr, sum, &vcpu.sregs)?;
        vcpu.set_reg8(reg, dst as u64, has_rex);
        flags::update_flags_add(&mut vcpu.regs.rflags, dst as u64, src as u64, sum as u64, 1);
    } else {
        let dst = vcpu.get_reg8(rm, has_rex) as u8;
        let sum = dst.wrapping_add(src);
        // XADD: TEMP = SRC + DEST; SRC = DEST; DEST = TEMP. When reg == rm refer
        // to the same byte register, writing SRC = old DEST after DEST = sum
        // would clobber the sum back to the old value, so only the DEST = sum
        // write happens (matches hardware: the register ends up 2*old). The
        // AH/AL-style alias (different byte positions of the same GPR) is fine
        // since the two set_reg8 writes touch disjoint byte lanes.
        if reg == rm {
            vcpu.set_reg8(rm, sum as u64, has_rex);
        } else {
            vcpu.set_reg8(rm, sum as u64, has_rex);
            vcpu.set_reg8(reg, dst as u64, has_rex);
        }
        flags::update_flags_add(&mut vcpu.regs.rflags, dst as u64, src as u64, sum as u64, 1);
    }
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// XADD r/m, r (0x0F 0xC1) - Exchange and Add
pub fn xadd_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);

    if is_memory {
        let dst = vcpu.read_mem(addr, op_size)?;
        let sum = dst.wrapping_add(src);
        // DEST = DEST + SRC, SRC = old DEST
        vcpu.write_mem(addr, sum, op_size)?;
        vcpu.set_reg(reg, dst, op_size);
        flags::update_flags_add(&mut vcpu.regs.rflags, dst, src, sum, op_size);
    } else {
        let dst = vcpu.get_reg(rm, op_size);
        let sum = dst.wrapping_add(src);
        // XADD: TEMP = SRC + DEST; SRC = DEST; DEST = TEMP
        // When reg == rm (same register), both SRC and DEST refer to the same register
        // so the result is just DEST = DEST + SRC = 2 * reg (SRC = DEST is a no-op)
        if reg == rm {
            vcpu.set_reg(rm, sum, op_size);
        } else {
            vcpu.set_reg(rm, sum, op_size);
            vcpu.set_reg(reg, dst, op_size);
        }
        flags::update_flags_add(&mut vcpu.regs.rflags, dst, src, sum, op_size);
    }
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMPXCHG r/m8, r8 (0x0F 0xB0) - Compare and Exchange
pub fn cmpxchg_rm8_r8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg8(reg, has_rex) as u8;
    let al = (vcpu.regs.rax & 0xFF) as u8;

    let dst = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg8(rm, has_rex) as u8
    };

    // Compare AL with destination
    let cmp_result = al.wrapping_sub(dst);
    flags::update_flags_sub(
        &mut vcpu.regs.rflags,
        al as u64,
        dst as u64,
        cmp_result as u64,
        1,
    );
    vcpu.clear_lazy_flags();

    if al == dst {
        // ZF is set, store source into destination
        if is_memory {
            vcpu.mmu.write_u8(addr, src, &vcpu.sregs)?;
        } else {
            vcpu.set_reg8(rm, src as u64, has_rex);
        }
    } else {
        // ZF is clear, load destination into AL
        vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (dst as u64);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMPXCHG r/m, r (0x0F 0xB1) - Compare and Exchange
pub fn cmpxchg_rm_r(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;
    let src = vcpu.get_reg(reg, op_size);
    let rax = vcpu.get_reg(0, op_size);

    let dst = if is_memory {
        vcpu.read_mem(addr, op_size)?
    } else {
        vcpu.get_reg(rm, op_size)
    };

    // Compare rAX with destination
    let cmp_result = rax.wrapping_sub(dst);
    flags::update_flags_sub(&mut vcpu.regs.rflags, rax, dst, cmp_result, op_size);
    vcpu.clear_lazy_flags();

    if rax == dst {
        // ZF is set, store source into destination
        if is_memory {
            vcpu.write_mem(addr, src, op_size)?;
        } else {
            vcpu.set_reg(rm, src, op_size);
        }
    } else {
        // ZF is clear, load destination into rAX
        vcpu.set_reg(0, dst, op_size);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
