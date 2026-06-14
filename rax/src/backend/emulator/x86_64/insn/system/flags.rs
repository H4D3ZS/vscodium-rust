//! Flag manipulation instructions: CLI, STI, CLC, STC, CLD, STD, CMC, LAHF, SAHF.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;
use super::control_regs::{current_cpl, raise_gp0};

/// CLI/STI fault with #GP(0) when the current privilege level is numerically
/// greater (less privileged) than the I/O privilege level (IOPL, bits 12-13 of
/// RFLAGS). When CPL <= IOPL the instruction is permitted.
#[inline]
fn iopl_blocks(vcpu: &X86_64Vcpu) -> bool {
    let iopl = ((vcpu.regs.rflags & flags::bits::IOPL_MASK) >> 12) as u8;
    current_cpl(vcpu) > iopl
}

/// CLI - Clear Interrupt Flag (0xFA)
pub fn cli(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    use super::super::super::cpu::log_if_transition;
    // #GP(0) when CPL > IOPL (insufficient privilege to modify IF).
    if iopl_blocks(vcpu) {
        return raise_gp0(vcpu);
    }
    let old_if = (vcpu.regs.rflags & flags::bits::IF) != 0;
    vcpu.regs.rflags &= !flags::bits::IF;
    log_if_transition(vcpu.regs.rip, old_if, false, "CLI");
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// STI - Set Interrupt Flag (0xFB)
pub fn sti(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    use super::super::super::cpu::log_if_transition;
    // #GP(0) when CPL > IOPL (insufficient privilege to modify IF).
    if iopl_blocks(vcpu) {
        return raise_gp0(vcpu);
    }
    let old_if = (vcpu.regs.rflags & flags::bits::IF) != 0;
    vcpu.regs.rflags |= flags::bits::IF;
    log_if_transition(vcpu.regs.rip, old_if, true, "STI");
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CLC - Clear Carry Flag (0xF8)
pub fn clc(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    vcpu.regs.rflags &= !flags::bits::CF;
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// STC - Set Carry Flag (0xF9)
pub fn stc(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    vcpu.regs.rflags |= flags::bits::CF;
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CLD - Clear Direction Flag (0xFC)
pub fn cld(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.regs.rflags &= !flags::bits::DF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// STD - Set Direction Flag (0xFD)
pub fn std(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.regs.rflags |= flags::bits::DF;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// CMC - Complement Carry Flag (0xF5)
pub fn cmc(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    vcpu.regs.rflags ^= flags::bits::CF;
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LAHF - Load AH from Flags (0x9F)
/// Loads SF, ZF, AF, PF, CF from RFLAGS into AH
pub fn lahf(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    vcpu.materialize_flags();
    // AH = SF:ZF:0:AF:0:PF:1:CF (bits 7:6:5:4:3:2:1:0)
    let mut flags_byte = (vcpu.regs.rflags & 0xD5) as u8;
    flags_byte |= 0x02;
    // Set AH (bits 8-15 of RAX)
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF00) | ((flags_byte as u64) << 8);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SAHF - Store AH into Flags (0x9E)
/// Stores AH into SF, ZF, AF, PF, CF of RFLAGS
pub fn sahf(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // AH contains SF:ZF:0:AF:0:PF:1:CF
    let ah = ((vcpu.regs.rax >> 8) & 0xFF) as u64;
    // Mask for SF, ZF, AF, PF, CF (bits 7, 6, 4, 2, 0)
    let mask = 0xD5u64; // 1101_0101
    vcpu.regs.rflags = (vcpu.regs.rflags & !mask) | (ah & mask);
    // Bit 1 is always set
    vcpu.regs.rflags |= 0x2;
    vcpu.clear_lazy_flags();
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
