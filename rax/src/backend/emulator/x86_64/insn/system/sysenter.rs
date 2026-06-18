//! SYSENTER/SYSEXIT instructions.

use crate::cpu::{Segment, VcpuExit};
use crate::error::{Error, Result};

use super::super::super::cpu::{InsnContext, X86_64Vcpu};
use super::super::super::flags;

const CR0_PE: u64 = 1 << 0;
const EFER_LMA: u64 = 1 << 10;

fn build_cs(selector: u16, dpl: u8, l: bool, db: bool) -> Segment {
    Segment {
        base: 0,
        limit: 0xFFFFF,
        selector,
        type_: 0x0B, // Execute/Read, accessed
        present: true,
        dpl,
        db,
        s: true,
        l,
        g: true,
        avl: false,
        unusable: false,
    }
}

fn build_ss(selector: u16, dpl: u8) -> Segment {
    Segment {
        base: 0,
        limit: 0xFFFFF,
        selector,
        type_: 0x03, // Read/Write, accessed
        present: true,
        dpl,
        db: true,
        s: true,
        l: false,
        g: true,
        avl: false,
        unusable: false,
    }
}

/// SYSENTER (0x0F 0x34)
pub fn sysenter(vcpu: &mut X86_64Vcpu, _ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if (vcpu.sregs.cr0 & CR0_PE) == 0 {
        return Err(Error::Emulator("SYSENTER requires CR0.PE=1".to_string()));
    }

    let cs_msr = vcpu.sregs.sysenter_cs as u16;
    let cs_selector = cs_msr & 0xFFFC;
    if cs_selector == 0 {
        return Err(Error::Emulator(
            "SYSENTER requires IA32_SYSENTER_CS[15:2] != 0".to_string(),
        ));
    }

    // Clear VM and IF.
    vcpu.regs.rflags &= !(flags::bits::VM | flags::bits::IF);

    let in_long_mode = (vcpu.sregs.efer & EFER_LMA) != 0;
    if in_long_mode {
        vcpu.regs.rsp = vcpu.sregs.sysenter_esp;
        vcpu.regs.rip = vcpu.sregs.sysenter_eip;
    } else {
        vcpu.regs.rsp = (vcpu.sregs.sysenter_esp as u32) as u64;
        vcpu.regs.rip = (vcpu.sregs.sysenter_eip as u32) as u64;
    }

    let ss_selector = cs_selector.wrapping_add(8);
    vcpu.sregs.cs = build_cs(cs_selector, 0, in_long_mode, !in_long_mode);
    vcpu.sregs.ss = build_ss(ss_selector, 0);

    Ok(None)
}

/// SYSEXIT (0x0F 0x35)
pub fn sysexit(vcpu: &mut X86_64Vcpu, ctx: &mut InsnContext) -> Result<Option<VcpuExit>> {
    if (vcpu.sregs.cr0 & CR0_PE) == 0 {
        return Err(Error::Emulator("SYSEXIT requires CR0.PE=1".to_string()));
    }

    let cs_base = (vcpu.sregs.sysenter_cs as u16) & 0xFFFC;
    if cs_base == 0 {
        return Err(Error::Emulator(
            "SYSEXIT requires IA32_SYSENTER_CS[15:2] != 0".to_string(),
        ));
    }

    let cpl = (vcpu.sregs.cs.selector & 0x3) as u8;
    if cpl != 0 {
        return Err(Error::Emulator("SYSEXIT requires CPL=0".to_string()));
    }

    let is_64 = ctx.rex_w();
    if is_64 {
        vcpu.regs.rsp = vcpu.regs.rcx;
        vcpu.regs.rip = vcpu.regs.rdx;
    } else {
        vcpu.regs.rsp = (vcpu.regs.rcx as u32) as u64;
        vcpu.regs.rip = (vcpu.regs.rdx as u32) as u64;
    }

    let cs_selector = if is_64 {
        cs_base.wrapping_add(32)
    } else {
        cs_base.wrapping_add(16)
    } | 0x3;
    let ss_selector = cs_selector.wrapping_add(8);
    vcpu.sregs.cs = build_cs(cs_selector, 3, is_64, !is_64);
    vcpu.sregs.ss = build_ss(ss_selector, 3);

    Ok(None)
}
