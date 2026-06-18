//! Loop instructions: LOOP, LOOPZ, LOOPNZ, JRCXZ.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

/// LOOPNZ/LOOPNE rel8 (0xE0) - Decrement ECX/RCX; jump if not zero and ZF=0
pub fn loopnz(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    let disp = ctx.consume_u8()? as i8 as i64;
    let next_rip = vcpu.regs.rip + ctx.cursor as u64;

    // Use address size to determine counter (RCX in 64-bit mode)
    let counter = if ctx.address_size_override {
        let ecx = (vcpu.regs.rcx as u32).wrapping_sub(1);
        vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF_FFFF) | (ecx as u64);
        ecx as u64
    } else {
        vcpu.regs.rcx = vcpu.regs.rcx.wrapping_sub(1);
        vcpu.regs.rcx
    };

    let zf = (vcpu.regs.rflags & flags::bits::ZF) != 0;

    if counter != 0 && !zf {
        vcpu.regs.rip = (next_rip as i64 + disp) as u64;
    } else {
        vcpu.regs.rip = next_rip;
    }
    Ok(None)
}

/// LOOPZ/LOOPE rel8 (0xE1) - Decrement ECX/RCX; jump if not zero and ZF=1
pub fn loopz(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    let disp = ctx.consume_u8()? as i8 as i64;
    let next_rip = vcpu.regs.rip + ctx.cursor as u64;

    let counter = if ctx.address_size_override {
        let ecx = (vcpu.regs.rcx as u32).wrapping_sub(1);
        vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF_FFFF) | (ecx as u64);
        ecx as u64
    } else {
        vcpu.regs.rcx = vcpu.regs.rcx.wrapping_sub(1);
        vcpu.regs.rcx
    };

    let zf = (vcpu.regs.rflags & flags::bits::ZF) != 0;

    if counter != 0 && zf {
        vcpu.regs.rip = (next_rip as i64 + disp) as u64;
    } else {
        vcpu.regs.rip = next_rip;
    }
    Ok(None)
}

/// LOOP rel8 (0xE2) - Decrement ECX/RCX; jump if not zero
pub fn loop_rel8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let disp = ctx.consume_u8()? as i8 as i64;
    let next_rip = vcpu.regs.rip + ctx.cursor as u64;

    let counter = if ctx.address_size_override {
        let ecx = (vcpu.regs.rcx as u32).wrapping_sub(1);
        vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF_FFFF) | (ecx as u64);
        ecx as u64
    } else {
        vcpu.regs.rcx = vcpu.regs.rcx.wrapping_sub(1);
        vcpu.regs.rcx
    };

    if counter != 0 {
        vcpu.regs.rip = (next_rip as i64 + disp) as u64;
    } else {
        vcpu.regs.rip = next_rip;
    }
    Ok(None)
}

/// JRCXZ/JECXZ rel8 (0xE3) - Jump if RCX/ECX is zero
pub fn jrcxz(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let disp = ctx.consume_u8()? as i8 as i64;
    let next_rip = vcpu.regs.rip + ctx.cursor as u64;

    // Use address size to determine counter (RCX in 64-bit mode, ECX with override)
    let counter = if ctx.address_size_override {
        vcpu.regs.rcx as u32 as u64
    } else {
        vcpu.regs.rcx
    };

    if counter == 0 {
        vcpu.regs.rip = (next_rip as i64 + disp) as u64;
    } else {
        vcpu.regs.rip = next_rip;
    }
    Ok(None)
}
