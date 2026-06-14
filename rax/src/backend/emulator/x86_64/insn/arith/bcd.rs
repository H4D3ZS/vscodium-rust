//! BCD Adjustment Instructions.
//!
//! These instructions are invalid in 64-bit mode. We reject them when CS.L=1
//! to match the spec.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

fn ensure_not_long_mode(vcpu: &mut X86_64Vcpu) -> Result<bool> {
    if vcpu.sregs.cs.l {
        vcpu.inject_exception(6, None)?;
        return Ok(false);
    }
    Ok(true)
}

/// DAA - Decimal Adjust AL after Addition (0x27)
/// Adjusts AL after BCD addition
pub fn daa(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ensure_not_long_mode(vcpu)? {
        return Ok(None);
    }
    vcpu.materialize_flags();
    let old_al = (vcpu.regs.rax & 0xFF) as u8;
    let old_cf = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let af = (vcpu.regs.rflags & flags::bits::AF) != 0;

    let mut al = old_al;
    let mut cf = false;
    let mut af_new = false;

    // If lower nibble > 9 or AF is set, add 6
    if (al & 0x0F) > 9 || af {
        let (new_al, carry) = al.overflowing_add(6);
        al = new_al;
        cf = old_cf || carry;
        af_new = true;
    }

    // If original AL > 0x99 or CF was set, add 0x60
    if old_al > 0x99 || old_cf {
        al = al.wrapping_add(0x60);
        cf = true;
    }

    // Set flags
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (al as u64);

    // Update flags: CF, AF, SF, ZF, PF
    if cf {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    if af_new {
        vcpu.regs.rflags |= flags::bits::AF;
    } else {
        vcpu.regs.rflags &= !flags::bits::AF;
    }

    // SF, ZF, PF based on result
    flags::update_szp(&mut vcpu.regs.rflags, al as u64, 1);
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// DAS - Decimal Adjust AL after Subtraction (0x2F)
/// Adjusts AL after BCD subtraction
pub fn das(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ensure_not_long_mode(vcpu)? {
        return Ok(None);
    }
    vcpu.materialize_flags();
    let old_al = (vcpu.regs.rax & 0xFF) as u8;
    let old_cf = (vcpu.regs.rflags & flags::bits::CF) != 0;
    let af = (vcpu.regs.rflags & flags::bits::AF) != 0;

    let mut al = old_al;
    let mut cf = false;
    let mut af_new = false;

    // If lower nibble > 9 or AF is set, subtract 6
    if (al & 0x0F) > 9 || af {
        let (new_al, borrow) = al.overflowing_sub(6);
        al = new_al;
        cf = old_cf || borrow;
        af_new = true;
    }

    // If original AL > 0x99 or CF was set, subtract 0x60
    if old_al > 0x99 || old_cf {
        al = al.wrapping_sub(0x60);
        cf = true;
    }

    // Set result
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF) | (al as u64);

    // Update flags: CF, AF, SF, ZF, PF
    if cf {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    if af_new {
        vcpu.regs.rflags |= flags::bits::AF;
    } else {
        vcpu.regs.rflags &= !flags::bits::AF;
    }

    // SF, ZF, PF based on result
    flags::update_szp(&mut vcpu.regs.rflags, al as u64, 1);
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// AAA - ASCII Adjust After Addition (0x37)
/// Adjusts AL and AH after unpacked BCD addition
pub fn aaa(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ensure_not_long_mode(vcpu)? {
        return Ok(None);
    }
    vcpu.materialize_flags();
    let al = (vcpu.regs.rax & 0xFF) as u8;
    let ah = ((vcpu.regs.rax >> 8) & 0xFF) as u8;
    let af = (vcpu.regs.rflags & flags::bits::AF) != 0;

    let (new_al, new_ah, cf, af_new) = if (al & 0x0F) > 9 || af {
        // AX := AX + 0x0106, then AL masked to low nibble.
        let ax = ((ah as u16) << 8) | al as u16;
        let ax = ax.wrapping_add(0x0106);
        ((ax & 0x0F) as u8, (ax >> 8) as u8, true, true)
    } else {
        // No adjustment needed
        (al & 0x0F, ah, false, false)
    };

    // Update AX (only low nibble of AL kept, AH adjusted)
    vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | ((new_ah as u64) << 8) | (new_al as u64);

    // Update CF and AF
    if cf {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    if af_new {
        vcpu.regs.rflags |= flags::bits::AF;
    } else {
        vcpu.regs.rflags &= !flags::bits::AF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// AAS - ASCII Adjust After Subtraction (0x3F)
/// Adjusts AL and AH after unpacked BCD subtraction
pub fn aas(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ensure_not_long_mode(vcpu)? {
        return Ok(None);
    }
    vcpu.materialize_flags();
    let al = (vcpu.regs.rax & 0xFF) as u8;
    let ah = ((vcpu.regs.rax >> 8) & 0xFF) as u8;
    let af = (vcpu.regs.rflags & flags::bits::AF) != 0;

    let (new_al, new_ah, cf, af_new) = if (al & 0x0F) > 9 || af {
        // AX := AX - 6; AH := AH - 1; then AL masked to low nibble.
        let ax = ((ah as u16) << 8) | al as u16;
        let ax = ax.wrapping_sub(0x0006);
        let ah = ((ax >> 8) as u8).wrapping_sub(1);
        ((ax & 0x0F) as u8, ah, true, true)
    } else {
        // No adjustment needed
        (al & 0x0F, ah, false, false)
    };

    // Update AX (only low nibble of AL kept, AH adjusted)
    vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | ((new_ah as u64) << 8) | (new_al as u64);

    // Update CF and AF
    if cf {
        vcpu.regs.rflags |= flags::bits::CF;
    } else {
        vcpu.regs.rflags &= !flags::bits::CF;
    }
    if af_new {
        vcpu.regs.rflags |= flags::bits::AF;
    } else {
        vcpu.regs.rflags &= !flags::bits::AF;
    }
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// AAM - ASCII Adjust AX after Multiply (0xD4 imm8)
/// AH = AL / imm8, AL = AL % imm8
pub fn aam(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ensure_not_long_mode(vcpu)? {
        return Ok(None);
    }
    let imm8 = ctx.consume_u8()?;

    // Division by zero causes #DE
    if imm8 == 0 {
        // #DE exception - don't advance RIP
        vcpu.inject_exception(0, None)?;
        return Ok(None);
    }

    let al = (vcpu.regs.rax & 0xFF) as u8;
    let ah = al / imm8;
    let new_al = al % imm8;

    // Set AH and AL
    vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | ((ah as u64) << 8) | (new_al as u64);

    // Update SF, ZF, PF based on AL
    flags::update_szp(&mut vcpu.regs.rflags, new_al as u64, 1);
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// AAD - ASCII Adjust AX before Division (0xD5 imm8)
/// AL = (AL + (AH * imm8)) & 0xFF, AH = 0
pub fn aad(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if !ensure_not_long_mode(vcpu)? {
        return Ok(None);
    }
    let imm8 = ctx.consume_u8()? as u16;

    let al = (vcpu.regs.rax & 0xFF) as u16;
    let ah = ((vcpu.regs.rax >> 8) & 0xFF) as u16;

    let new_al = ((al + (ah * imm8)) & 0xFF) as u8;

    // Set AL, clear AH
    vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | (new_al as u64);

    // Update SF, ZF, PF based on AL
    flags::update_szp(&mut vcpu.regs.rflags, new_al as u64, 1);
    vcpu.clear_lazy_flags();

    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
