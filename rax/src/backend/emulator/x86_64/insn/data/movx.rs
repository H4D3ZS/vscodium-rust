//! MOVZX, MOVSX, MOVSXD instructions.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// MOVSXD r, r/m (0x63). Operand-size–dependent:
///   - REX.W: sign-extend r/m32 -> r64.
///   - 32-bit (default, no REX.W): the source is r/m32 and the destination is r32;
///     the value is written 32-bit (zero-extended into the full 64-bit register,
///     the standard 64-bit-mode write behavior). No actual sign extension occurs
///     because source and destination are the same width.
///   - 16-bit (66 prefix, no REX.W): the source is r/m16 written to r16.
/// Hardware/KVM accept all of these without faulting, so we must too.
pub fn movsxd(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    if ctx.rex_w() {
        // Sign-extend r/m32 -> r64.
        let value = if is_memory {
            vcpu.mmu.read_u32(addr, &vcpu.sregs)?
        } else {
            vcpu.get_reg(rm, 4) as u32
        };
        let extended = value as i32 as i64 as u64;
        vcpu.set_reg(reg, extended, 8);
    } else if ctx.op_size == 2 {
        // 16-bit form: move r/m16 -> r16 (only the low 16 bits change).
        let value = if is_memory {
            vcpu.mmu.read_u16(addr, &vcpu.sregs)? as u64
        } else {
            vcpu.get_reg(rm, 2) & 0xFFFF
        };
        vcpu.set_reg(reg, value, 2);
    } else {
        // 32-bit form: move r/m32 -> r32, zero-extending into the 64-bit register.
        let value = if is_memory {
            vcpu.mmu.read_u32(addr, &vcpu.sregs)? as u64
        } else {
            vcpu.get_reg(rm, 4) & 0xFFFF_FFFF
        };
        vcpu.set_reg(reg, value, 4);
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVZX r, r/m8 (0x0F 0xB6)
pub fn movzx_r_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)? as u64
    } else {
        // Use get_reg8 to properly handle high-byte registers (AH, BH, CH, DH)
        vcpu.get_reg8(rm, has_rex)
    };

    vcpu.set_reg(reg, value, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVZX r, r/m16 (0x0F 0xB7)
pub fn movzx_r_rm16(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.mmu.read_u16(addr, &vcpu.sregs)? as u64
    } else {
        vcpu.get_reg(rm, 2) & 0xFFFF
    };
    vcpu.set_reg(reg, value, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVSX r, r/m8 (0x0F 0xBE)
pub fn movsx_r_rm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let has_rex = ctx.rex.is_some();
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.mmu.read_u8(addr, &vcpu.sregs)?
    } else {
        // Use get_reg8 to properly handle high-byte registers (AH, BH, CH, DH)
        vcpu.get_reg8(rm, has_rex) as u8
    };
    // Sign-extend
    let extended = value as i8 as i64 as u64;
    vcpu.set_reg(reg, extended, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MOVSX r, r/m16 (0x0F 0xBF)
pub fn movsx_r_rm16(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let op_size = ctx.op_size;
    let (reg, rm, is_memory, addr, _) = vcpu.decode_modrm(ctx)?;

    let value = if is_memory {
        vcpu.mmu.read_u16(addr, &vcpu.sregs)?
    } else {
        vcpu.get_reg(rm, 2) as u16
    };
    // Sign-extend
    let extended = value as i16 as i64 as u64;
    vcpu.set_reg(reg, extended, op_size);
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
