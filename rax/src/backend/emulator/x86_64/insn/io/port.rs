//! Port I/O instructions: IN, OUT.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// IN AL, imm8 (0xE4)
pub fn in_al_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = ctx.consume_u8()? as u16;
    vcpu.regs.rip += ctx.cursor as u64;
    vcpu.set_io_pending_reg(1);
    Ok(Some(VcpuExit::IoIn { port, size: 1 }))
}

/// IN AX/EAX, imm8 (0xE5)
pub fn in_ax_imm8(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = ctx.consume_u8()? as u16;
    let size = if ctx.operand_size_override { 2 } else { 4 };
    vcpu.regs.rip += ctx.cursor as u64;
    vcpu.set_io_pending_reg(size);
    Ok(Some(VcpuExit::IoIn { port, size }))
}

/// IN AL, DX (0xEC)
pub fn in_al_dx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = vcpu.regs.rdx as u16;
    vcpu.regs.rip += ctx.cursor as u64;
    vcpu.set_io_pending_reg(1);
    Ok(Some(VcpuExit::IoIn { port, size: 1 }))
}

/// IN AX/EAX, DX (0xED)
pub fn in_ax_dx(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = vcpu.regs.rdx as u16;
    let size = if ctx.operand_size_override { 2 } else { 4 };
    vcpu.regs.rip += ctx.cursor as u64;
    vcpu.set_io_pending_reg(size);
    Ok(Some(VcpuExit::IoIn { port, size }))
}

/// OUT imm8, AL (0xE6)
pub fn out_imm8_al(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = ctx.consume_u8()? as u16;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(Some(VcpuExit::IoOut {
        port,
        data: vec![vcpu.regs.rax as u8],
    }))
}

/// OUT imm8, AX/EAX (0xE7)
pub fn out_imm8_ax(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = ctx.consume_u8()? as u16;
    let data = if ctx.operand_size_override {
        (vcpu.regs.rax as u16).to_le_bytes().to_vec()
    } else {
        (vcpu.regs.rax as u32).to_le_bytes().to_vec()
    };
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(Some(VcpuExit::IoOut { port, data }))
}

/// OUT DX, AL (0xEE)
pub fn out_dx_al(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = vcpu.regs.rdx as u16;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(Some(VcpuExit::IoOut {
        port,
        data: vec![vcpu.regs.rax as u8],
    }))
}

/// OUT DX, AX/EAX (0xEF)
pub fn out_dx_ax(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    let port = vcpu.regs.rdx as u16;
    let data = if ctx.operand_size_override {
        (vcpu.regs.rax as u16).to_le_bytes().to_vec()
    } else {
        (vcpu.regs.rax as u32).to_le_bytes().to_vec()
    };
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(Some(VcpuExit::IoOut { port, data }))
}
