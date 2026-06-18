//! Memory fence instructions: LFENCE, MFENCE, SFENCE, PAUSE.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};

/// PAUSE - Spin-Loop Hint (F3 90)
pub fn pause(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Hint to the processor that this is a spin-wait loop
    // In emulation, we just treat it as NOP
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// LFENCE - Load Fence (0x0F 0xAE /5)
pub fn lfence(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Memory fence - treat as NOP in emulation
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// MFENCE - Memory Fence (0x0F 0xAE /6)
pub fn mfence(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Memory fence - treat as NOP in emulation
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// SFENCE - Store Fence (0x0F 0xAE /7)
pub fn sfence(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Memory fence - treat as NOP in emulation
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
