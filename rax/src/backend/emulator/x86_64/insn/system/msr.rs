//! MSR instructions: RDMSR, WRMSR.

use crate::cpu::VcpuExit;
use crate::error::Result;

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::control_regs::{is_cpl0, raise_gp0};

/// WRMSR (0x0F 0x30)
pub fn wrmsr(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Privileged: WRMSR requires CPL 0.
    if !is_cpl0(vcpu) {
        return raise_gp0(vcpu);
    }
    let ecx = vcpu.regs.rcx as u32;
    let value = ((vcpu.regs.rdx & 0xFFFF_FFFF) << 32) | (vcpu.regs.rax & 0xFFFF_FFFF);

    match ecx {
        0xC0000080 => vcpu.sregs.efer = value,    // EFER
        0xC0000081 => vcpu.sregs.star = value,    // STAR
        0xC0000082 => vcpu.sregs.lstar = value,   // LSTAR
        0xC0000083 => vcpu.sregs.cstar = value,   // CSTAR
        0xC0000084 => vcpu.sregs.fmask = value,   // FMASK
        0x174 => vcpu.sregs.sysenter_cs = value,  // IA32_SYSENTER_CS
        0x175 => vcpu.sregs.sysenter_esp = value, // IA32_SYSENTER_ESP
        0x176 => vcpu.sregs.sysenter_eip = value, // IA32_SYSENTER_EIP
        0xC0000100 => vcpu.sregs.fs.base = value, // FS.base (TLS)
        0xC0000101 => {
            vcpu.sregs.gs.base = value; // GS.base (per-CPU data)
            // WORKAROUND: When gs.base is set to a non-zero value, update the per-CPU
            // CR0 shadow with the current CR0 value. This fixes the case where CR0 was
            // written before per-CPU was set up, and the shadow was copied with garbage.
            if value != 0 {
                let percpu_offset = 0xffffffff836ee018u64;
                let instance_addr = value.wrapping_add(percpu_offset);
                // Flush TLB to ensure clean state
                vcpu.mmu.flush_tlb();
                // Write current CR0 to per-CPU shadow (ignore errors)
                let _ = vcpu
                    .mmu
                    .write_u64(instance_addr, vcpu.sregs.cr0, &vcpu.sregs);
            }
        }
        0xC0000102 => vcpu.kernel_gs_base = value, // KernelGSbase
        _ => {}                                    // Ignore unknown MSRs
    }
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}

/// RDMSR (0x0F 0x32)
pub fn rdmsr(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    // Privileged: RDMSR requires CPL 0.
    if !is_cpl0(vcpu) {
        return raise_gp0(vcpu);
    }
    let ecx = vcpu.regs.rcx as u32;

    let value = match ecx {
        0x10 => {
            // IA32_TIME_STAMP_COUNTER. Return the same real-time value as the
            // RDTSC instruction (vcpu.tsc()) so reads via the MSR and via the
            // instruction agree; previously this used host wall-clock directly,
            // which disagreed with RDTSC and made boots nondeterministic.
            vcpu.tsc()
        }
        0x1B => {
            // IA32_APIC_BASE - APIC base address
            // Bit 8: BSP flag (this is the bootstrap processor)
            // Bit 11: APIC global enable
            // Bits 12-35: APIC base physical address (default 0xFEE00000)
            (1u64 << 8) | (1u64 << 11) | 0xFEE00000u64
        }
        0xC0000080 => vcpu.sregs.efer,     // EFER
        0xC0000081 => vcpu.sregs.star,     // STAR
        0xC0000082 => vcpu.sregs.lstar,    // LSTAR
        0xC0000083 => vcpu.sregs.cstar,    // CSTAR
        0xC0000084 => vcpu.sregs.fmask,    // FMASK
        0x174 => vcpu.sregs.sysenter_cs,   // IA32_SYSENTER_CS
        0x175 => vcpu.sregs.sysenter_esp,  // IA32_SYSENTER_ESP
        0x176 => vcpu.sregs.sysenter_eip,  // IA32_SYSENTER_EIP
        0xC0000100 => vcpu.sregs.fs.base,  // FS.base
        0xC0000101 => vcpu.sregs.gs.base,  // GS.base
        0xC0000102 => vcpu.kernel_gs_base, // KernelGSbase
        _ => 0,                            // Return 0 for unknown MSRs
    };

    vcpu.regs.rax = (value & 0xFFFF_FFFF) as u64;
    vcpu.regs.rdx = (value >> 32) as u64;
    vcpu.regs.rip += ctx.cursor as u64;
    Ok(None)
}
