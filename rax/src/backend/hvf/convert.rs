//! Conversion between CpuState and HVF/VMCS types.
//!
//! This module handles translation between our internal CPU state representation
//! and the Hypervisor.framework VMCS fields and register values.

use super::bindings::*;
use crate::cpu::{DescriptorTable, Registers, Segment, SystemRegisters};
use crate::error::{Error, Result};

/// Read a VMCS field, returning an error if the operation fails.
#[inline]
pub fn vmcs_read(vcpu: hv_vcpuid_t, field: hv_vmx_vmcs_field_t) -> Result<u64> {
    let mut value: u64 = 0;
    let ret = unsafe { hv_vmx_vcpu_read_vmcs(vcpu, field as u32, &mut value) };
    if ret != HV_SUCCESS {
        return Err(Error::Emulator(format!(
            "Failed to read VMCS field {:?}: {}",
            field,
            hv_error_string(ret)
        )));
    }
    Ok(value)
}

/// Write a VMCS field, returning an error if the operation fails.
#[inline]
pub fn vmcs_write(vcpu: hv_vcpuid_t, field: hv_vmx_vmcs_field_t, value: u64) -> Result<()> {
    let ret = unsafe { hv_vmx_vcpu_write_vmcs(vcpu, field as u32, value) };
    if ret != HV_SUCCESS {
        return Err(Error::Emulator(format!(
            "Failed to write VMCS field {:?}={:#x}: {}",
            field,
            value,
            hv_error_string(ret)
        )));
    }
    Ok(())
}

/// Read a vCPU register.
#[inline]
pub fn read_register(vcpu: hv_vcpuid_t, reg: hv_x86_reg_t) -> Result<u64> {
    let mut value: u64 = 0;
    let ret = unsafe { hv_vcpu_read_register(vcpu, reg as u32, &mut value) };
    if ret != HV_SUCCESS {
        return Err(Error::Emulator(format!(
            "Failed to read register {:?}: {}",
            reg,
            hv_error_string(ret)
        )));
    }
    Ok(value)
}

/// Write a vCPU register.
#[inline]
pub fn write_register(vcpu: hv_vcpuid_t, reg: hv_x86_reg_t, value: u64) -> Result<()> {
    let ret = unsafe { hv_vcpu_write_register(vcpu, reg as u32, value) };
    if ret != HV_SUCCESS {
        return Err(Error::Emulator(format!(
            "Failed to write register {:?}={:#x}: {}",
            reg,
            value,
            hv_error_string(ret)
        )));
    }
    Ok(())
}

/// Read general-purpose registers from HVF vCPU.
pub fn regs_from_hvf(vcpu: hv_vcpuid_t) -> Result<Registers> {
    use hv_x86_reg_t::*;

    Ok(Registers {
        rax: read_register(vcpu, HV_X86_RAX)?,
        rbx: read_register(vcpu, HV_X86_RBX)?,
        rcx: read_register(vcpu, HV_X86_RCX)?,
        rdx: read_register(vcpu, HV_X86_RDX)?,
        rsi: read_register(vcpu, HV_X86_RSI)?,
        rdi: read_register(vcpu, HV_X86_RDI)?,
        rsp: read_register(vcpu, HV_X86_RSP)?,
        rbp: read_register(vcpu, HV_X86_RBP)?,
        r8: read_register(vcpu, HV_X86_R8)?,
        r9: read_register(vcpu, HV_X86_R9)?,
        r10: read_register(vcpu, HV_X86_R10)?,
        r11: read_register(vcpu, HV_X86_R11)?,
        r12: read_register(vcpu, HV_X86_R12)?,
        r13: read_register(vcpu, HV_X86_R13)?,
        r14: read_register(vcpu, HV_X86_R14)?,
        r15: read_register(vcpu, HV_X86_R15)?,
        rip: read_register(vcpu, HV_X86_RIP)?,
        rflags: read_register(vcpu, HV_X86_RFLAGS)?,
        // SIMD registers will be read via FP state; APX EGPRs are not
        // exposed by HVF.
        ..Default::default()
    })
}

/// Write general-purpose registers to HVF vCPU.
pub fn regs_to_hvf(vcpu: hv_vcpuid_t, regs: &Registers) -> Result<()> {
    use hv_x86_reg_t::*;

    write_register(vcpu, HV_X86_RAX, regs.rax)?;
    write_register(vcpu, HV_X86_RBX, regs.rbx)?;
    write_register(vcpu, HV_X86_RCX, regs.rcx)?;
    write_register(vcpu, HV_X86_RDX, regs.rdx)?;
    write_register(vcpu, HV_X86_RSI, regs.rsi)?;
    write_register(vcpu, HV_X86_RDI, regs.rdi)?;
    write_register(vcpu, HV_X86_RSP, regs.rsp)?;
    write_register(vcpu, HV_X86_RBP, regs.rbp)?;
    write_register(vcpu, HV_X86_R8, regs.r8)?;
    write_register(vcpu, HV_X86_R9, regs.r9)?;
    write_register(vcpu, HV_X86_R10, regs.r10)?;
    write_register(vcpu, HV_X86_R11, regs.r11)?;
    write_register(vcpu, HV_X86_R12, regs.r12)?;
    write_register(vcpu, HV_X86_R13, regs.r13)?;
    write_register(vcpu, HV_X86_R14, regs.r14)?;
    write_register(vcpu, HV_X86_R15, regs.r15)?;
    write_register(vcpu, HV_X86_RIP, regs.rip)?;
    write_register(vcpu, HV_X86_RFLAGS, regs.rflags)?;

    Ok(())
}

/// Convert VMCS segment access rights to our Segment format.
/// VMX access rights format:
/// Bits 0-3: Type
/// Bit 4: S (descriptor type)
/// Bits 5-6: DPL
/// Bit 7: Present
/// Bits 8-11: Reserved
/// Bit 12: Available
/// Bit 13: L (64-bit mode)
/// Bit 14: D/B
/// Bit 15: G (granularity)
/// Bit 16: Unusable
fn segment_from_vmcs(selector: u64, base: u64, limit: u64, access_rights: u64) -> Segment {
    Segment {
        base,
        limit: limit as u32,
        selector: selector as u16,
        type_: (access_rights & 0xF) as u8,
        s: (access_rights & (1 << 4)) != 0,
        dpl: ((access_rights >> 5) & 0x3) as u8,
        present: (access_rights & (1 << 7)) != 0,
        avl: (access_rights & (1 << 12)) != 0,
        l: (access_rights & (1 << 13)) != 0,
        db: (access_rights & (1 << 14)) != 0,
        g: (access_rights & (1 << 15)) != 0,
        unusable: (access_rights & (1 << 16)) != 0,
    }
}

/// Convert our Segment format to VMCS access rights.
fn segment_to_access_rights(seg: &Segment) -> u64 {
    let mut ar: u64 = seg.type_ as u64;
    if seg.s {
        ar |= 1 << 4;
    }
    ar |= (seg.dpl as u64) << 5;
    if seg.present {
        ar |= 1 << 7;
    }
    if seg.avl {
        ar |= 1 << 12;
    }
    if seg.l {
        ar |= 1 << 13;
    }
    if seg.db {
        ar |= 1 << 14;
    }
    if seg.g {
        ar |= 1 << 15;
    }
    if seg.unusable {
        ar |= 1 << 16;
    }
    ar
}

/// Read system registers from HVF vCPU via VMCS.
pub fn sregs_from_hvf(vcpu: hv_vcpuid_t) -> Result<SystemRegisters> {
    use hv_vmx_vmcs_field_t::*;
    use hv_x86_reg_t::*;

    // Read segment registers from VMCS
    let cs = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_CS_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_CS_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_CS_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_CS_ACCESS_RIGHTS)?,
    );
    let ds = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_DS_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_DS_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_DS_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_DS_ACCESS_RIGHTS)?,
    );
    let es = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_ES_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_ES_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_ES_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_ES_ACCESS_RIGHTS)?,
    );
    let fs = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_FS_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_FS_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_FS_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_FS_ACCESS_RIGHTS)?,
    );
    let gs = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_GS_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_GS_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_GS_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_GS_ACCESS_RIGHTS)?,
    );
    let ss = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_SS_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_SS_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_SS_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_SS_ACCESS_RIGHTS)?,
    );
    let tr = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_TR_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_TR_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_TR_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_TR_ACCESS_RIGHTS)?,
    );
    let ldt = segment_from_vmcs(
        vmcs_read(vcpu, VMCS_GUEST_LDTR_SELECTOR)?,
        vmcs_read(vcpu, VMCS_GUEST_LDTR_BASE)?,
        vmcs_read(vcpu, VMCS_GUEST_LDTR_LIMIT)?,
        vmcs_read(vcpu, VMCS_GUEST_LDTR_ACCESS_RIGHTS)?,
    );

    // Descriptor tables
    let gdt = DescriptorTable {
        base: vmcs_read(vcpu, VMCS_GUEST_GDTR_BASE)?,
        limit: vmcs_read(vcpu, VMCS_GUEST_GDTR_LIMIT)? as u16,
    };
    let idt = DescriptorTable {
        base: vmcs_read(vcpu, VMCS_GUEST_IDTR_BASE)?,
        limit: vmcs_read(vcpu, VMCS_GUEST_IDTR_LIMIT)? as u16,
    };

    // Control registers
    let cr0 = vmcs_read(vcpu, VMCS_GUEST_CR0)?;
    let cr3 = vmcs_read(vcpu, VMCS_GUEST_CR3)?;
    let cr4 = vmcs_read(vcpu, VMCS_GUEST_CR4)?;
    let cr2 = read_register(vcpu, HV_X86_CR2)?;

    // EFER is accessible via VMCS
    let efer = vmcs_read(vcpu, VMCS_GUEST_IA32_EFER)?;

    // Debug registers via direct register access
    let dr0 = read_register(vcpu, HV_X86_DR0)?;
    let dr1 = read_register(vcpu, HV_X86_DR1)?;
    let dr2 = read_register(vcpu, HV_X86_DR2)?;
    let dr3 = read_register(vcpu, HV_X86_DR3)?;
    let dr6 = read_register(vcpu, HV_X86_DR6)?;
    let dr7 = vmcs_read(vcpu, VMCS_GUEST_DR7)?;

    // SYSENTER MSRs from VMCS
    let sysenter_cs = vmcs_read(vcpu, VMCS_GUEST_IA32_SYSENTER_CS)?;
    let sysenter_esp = vmcs_read(vcpu, VMCS_GUEST_IA32_SYSENTER_ESP)?;
    let sysenter_eip = vmcs_read(vcpu, VMCS_GUEST_IA32_SYSENTER_EIP)?;

    Ok(SystemRegisters {
        cs,
        ds,
        es,
        fs,
        gs,
        ss,
        tr,
        ldt,
        gdt,
        idt,
        cr0,
        cr2,
        cr3,
        cr4,
        cr8: 0, // CR8 (TPR) - not directly accessible via basic VMCS
        efer,
        star: 0,  // These MSRs require MSR load/store areas
        lstar: 0, // which we'll handle separately
        cstar: 0,
        fmask: 0,
        sysenter_cs,
        sysenter_esp,
        sysenter_eip,
        dr0,
        dr1,
        dr2,
        dr3,
        dr6,
        dr7,
    })
}

/// Write system registers to HVF vCPU via VMCS.
pub fn sregs_to_hvf(vcpu: hv_vcpuid_t, sregs: &SystemRegisters) -> Result<()> {
    use hv_vmx_vmcs_field_t::*;
    use hv_x86_reg_t::*;

    // CS segment
    vmcs_write(vcpu, VMCS_GUEST_CS_SELECTOR, sregs.cs.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_CS_BASE, sregs.cs.base)?;
    vmcs_write(vcpu, VMCS_GUEST_CS_LIMIT, sregs.cs.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_CS_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.cs),
    )?;

    // DS segment
    vmcs_write(vcpu, VMCS_GUEST_DS_SELECTOR, sregs.ds.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_DS_BASE, sregs.ds.base)?;
    vmcs_write(vcpu, VMCS_GUEST_DS_LIMIT, sregs.ds.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_DS_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.ds),
    )?;

    // ES segment
    vmcs_write(vcpu, VMCS_GUEST_ES_SELECTOR, sregs.es.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_ES_BASE, sregs.es.base)?;
    vmcs_write(vcpu, VMCS_GUEST_ES_LIMIT, sregs.es.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_ES_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.es),
    )?;

    // FS segment
    vmcs_write(vcpu, VMCS_GUEST_FS_SELECTOR, sregs.fs.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_FS_BASE, sregs.fs.base)?;
    vmcs_write(vcpu, VMCS_GUEST_FS_LIMIT, sregs.fs.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_FS_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.fs),
    )?;

    // GS segment
    vmcs_write(vcpu, VMCS_GUEST_GS_SELECTOR, sregs.gs.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_GS_BASE, sregs.gs.base)?;
    vmcs_write(vcpu, VMCS_GUEST_GS_LIMIT, sregs.gs.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_GS_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.gs),
    )?;

    // SS segment
    vmcs_write(vcpu, VMCS_GUEST_SS_SELECTOR, sregs.ss.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_SS_BASE, sregs.ss.base)?;
    vmcs_write(vcpu, VMCS_GUEST_SS_LIMIT, sregs.ss.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_SS_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.ss),
    )?;

    // TR segment
    vmcs_write(vcpu, VMCS_GUEST_TR_SELECTOR, sregs.tr.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_TR_BASE, sregs.tr.base)?;
    vmcs_write(vcpu, VMCS_GUEST_TR_LIMIT, sregs.tr.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_TR_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.tr),
    )?;

    // LDT segment
    vmcs_write(vcpu, VMCS_GUEST_LDTR_SELECTOR, sregs.ldt.selector as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_LDTR_BASE, sregs.ldt.base)?;
    vmcs_write(vcpu, VMCS_GUEST_LDTR_LIMIT, sregs.ldt.limit as u64)?;
    vmcs_write(
        vcpu,
        VMCS_GUEST_LDTR_ACCESS_RIGHTS,
        segment_to_access_rights(&sregs.ldt),
    )?;

    // Descriptor tables
    vmcs_write(vcpu, VMCS_GUEST_GDTR_BASE, sregs.gdt.base)?;
    vmcs_write(vcpu, VMCS_GUEST_GDTR_LIMIT, sregs.gdt.limit as u64)?;
    vmcs_write(vcpu, VMCS_GUEST_IDTR_BASE, sregs.idt.base)?;
    vmcs_write(vcpu, VMCS_GUEST_IDTR_LIMIT, sregs.idt.limit as u64)?;

    // Control registers
    vmcs_write(vcpu, VMCS_GUEST_CR0, sregs.cr0)?;
    vmcs_write(vcpu, VMCS_GUEST_CR3, sregs.cr3)?;
    vmcs_write(vcpu, VMCS_GUEST_CR4, sregs.cr4)?;
    write_register(vcpu, HV_X86_CR2, sregs.cr2)?;

    // EFER
    vmcs_write(vcpu, VMCS_GUEST_IA32_EFER, sregs.efer)?;

    // Debug registers
    write_register(vcpu, HV_X86_DR0, sregs.dr0)?;
    write_register(vcpu, HV_X86_DR1, sregs.dr1)?;
    write_register(vcpu, HV_X86_DR2, sregs.dr2)?;
    write_register(vcpu, HV_X86_DR3, sregs.dr3)?;
    write_register(vcpu, HV_X86_DR6, sregs.dr6)?;
    vmcs_write(vcpu, VMCS_GUEST_DR7, sregs.dr7)?;

    // SYSENTER MSRs
    vmcs_write(vcpu, VMCS_GUEST_IA32_SYSENTER_CS, sregs.sysenter_cs)?;
    vmcs_write(vcpu, VMCS_GUEST_IA32_SYSENTER_ESP, sregs.sysenter_esp)?;
    vmcs_write(vcpu, VMCS_GUEST_IA32_SYSENTER_EIP, sregs.sysenter_eip)?;

    Ok(())
}

/// Parse VMX exit qualification for I/O exits.
pub struct IoExitQualification {
    pub size: u8, // 1, 2, or 4 bytes
    pub direction: IoDirection,
    pub string: bool, // String I/O (INS/OUTS)
    pub rep: bool,    // REP prefix
    pub port: u16,
}

pub enum IoDirection {
    Out,
    In,
}

impl IoExitQualification {
    pub fn from_qualification(qual: u64) -> Self {
        let size = match qual & 0x7 {
            0 => 1,
            1 => 2,
            3 => 4,
            _ => 1,
        };
        let direction = if (qual & (1 << 3)) != 0 {
            IoDirection::In
        } else {
            IoDirection::Out
        };
        let string = (qual & (1 << 4)) != 0;
        let rep = (qual & (1 << 5)) != 0;
        let port = ((qual >> 16) & 0xFFFF) as u16;

        IoExitQualification {
            size,
            direction,
            string,
            rep,
            port,
        }
    }
}

/// Parse VMX exit qualification for EPT violations (memory faults).
pub struct EptViolationQualification {
    pub read: bool,
    pub write: bool,
    pub fetch: bool,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub guest_linear_valid: bool,
    pub guest_physical_valid: bool,
}

impl EptViolationQualification {
    pub fn from_qualification(qual: u64) -> Self {
        EptViolationQualification {
            read: (qual & (1 << 0)) != 0,
            write: (qual & (1 << 1)) != 0,
            fetch: (qual & (1 << 2)) != 0,
            readable: (qual & (1 << 3)) != 0,
            writable: (qual & (1 << 4)) != 0,
            executable: (qual & (1 << 5)) != 0,
            guest_linear_valid: (qual & (1 << 7)) != 0,
            guest_physical_valid: (qual & (1 << 8)) != 0,
        }
    }
}
