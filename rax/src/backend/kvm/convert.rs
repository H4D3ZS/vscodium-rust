//! Conversion between CpuState and KVM types.

use kvm_bindings::{kvm_dtable, kvm_regs, kvm_segment, kvm_sregs};

use crate::cpu::{DescriptorTable, Registers, Segment, SystemRegisters};

/// Convert KVM registers to our Registers type.
pub fn regs_from_kvm(kvm: &kvm_regs) -> Registers {
    Registers {
        rax: kvm.rax,
        rbx: kvm.rbx,
        rcx: kvm.rcx,
        rdx: kvm.rdx,
        rsi: kvm.rsi,
        rdi: kvm.rdi,
        rsp: kvm.rsp,
        rbp: kvm.rbp,
        r8: kvm.r8,
        r9: kvm.r9,
        r10: kvm.r10,
        r11: kvm.r11,
        r12: kvm.r12,
        r13: kvm.r13,
        r14: kvm.r14,
        r15: kvm.r15,
        // APX Extended GPRs (R16-R31) - not supported by KVM yet, initialize to 0
        r16: 0,
        r17: 0,
        r18: 0,
        r19: 0,
        r20: 0,
        r21: 0,
        r22: 0,
        r23: 0,
        r24: 0,
        r25: 0,
        r26: 0,
        r27: 0,
        r28: 0,
        r29: 0,
        r30: 0,
        r31: 0,
        rip: kvm.rip,
        rflags: kvm.rflags,
        // Note: MMX/XMM/YMM registers are part of FPU state in KVM, initialized to defaults here
        xmm: [[0; 2]; 16],
        ymm_high: [[0; 2]; 16],
        zmm_high: [[0; 4]; 16],
        zmm_ext: [[0; 8]; 16],
        k: [0; 8],
        mm: [0; 8],
    }
}

/// Convert our Registers type to KVM registers.
pub fn regs_to_kvm(regs: &Registers) -> kvm_regs {
    kvm_regs {
        rax: regs.rax,
        rbx: regs.rbx,
        rcx: regs.rcx,
        rdx: regs.rdx,
        rsi: regs.rsi,
        rdi: regs.rdi,
        rsp: regs.rsp,
        rbp: regs.rbp,
        r8: regs.r8,
        r9: regs.r9,
        r10: regs.r10,
        r11: regs.r11,
        r12: regs.r12,
        r13: regs.r13,
        r14: regs.r14,
        r15: regs.r15,
        rip: regs.rip,
        rflags: regs.rflags,
    }
}

/// Convert KVM segment to our Segment type.
fn segment_from_kvm(kvm: &kvm_segment) -> Segment {
    Segment {
        base: kvm.base,
        limit: kvm.limit,
        selector: kvm.selector,
        type_: kvm.type_,
        present: kvm.present != 0,
        dpl: kvm.dpl,
        db: kvm.db != 0,
        s: kvm.s != 0,
        l: kvm.l != 0,
        g: kvm.g != 0,
        avl: kvm.avl != 0,
        unusable: kvm.unusable != 0,
    }
}

/// Convert our Segment type to KVM segment.
fn segment_to_kvm(seg: &Segment) -> kvm_segment {
    kvm_segment {
        base: seg.base,
        limit: seg.limit,
        selector: seg.selector,
        type_: seg.type_,
        present: seg.present as u8,
        dpl: seg.dpl,
        db: seg.db as u8,
        s: seg.s as u8,
        l: seg.l as u8,
        g: seg.g as u8,
        avl: seg.avl as u8,
        unusable: seg.unusable as u8,
        padding: 0,
    }
}

/// Convert KVM descriptor table to our DescriptorTable type.
fn dtable_from_kvm(kvm: &kvm_dtable) -> DescriptorTable {
    DescriptorTable {
        base: kvm.base,
        limit: kvm.limit,
    }
}

/// Convert our DescriptorTable type to KVM descriptor table.
fn dtable_to_kvm(dt: &DescriptorTable) -> kvm_dtable {
    kvm_dtable {
        base: dt.base,
        limit: dt.limit,
        padding: [0; 3],
    }
}

/// Convert KVM system registers to our SystemRegisters type.
pub fn sregs_from_kvm(kvm: &kvm_sregs) -> SystemRegisters {
    SystemRegisters {
        cs: segment_from_kvm(&kvm.cs),
        ds: segment_from_kvm(&kvm.ds),
        es: segment_from_kvm(&kvm.es),
        fs: segment_from_kvm(&kvm.fs),
        gs: segment_from_kvm(&kvm.gs),
        ss: segment_from_kvm(&kvm.ss),
        tr: segment_from_kvm(&kvm.tr),
        ldt: segment_from_kvm(&kvm.ldt),
        gdt: dtable_from_kvm(&kvm.gdt),
        idt: dtable_from_kvm(&kvm.idt),
        cr0: kvm.cr0,
        cr2: kvm.cr2,
        cr3: kvm.cr3,
        cr4: kvm.cr4,
        cr8: kvm.cr8,
        efer: kvm.efer,
        star: 0,
        lstar: 0,
        cstar: 0,
        fmask: 0,
        sysenter_cs: 0,
        sysenter_esp: 0,
        sysenter_eip: 0,
        // Debug registers (not part of kvm_sregs, initialized to defaults)
        dr0: 0,
        dr1: 0,
        dr2: 0,
        dr3: 0,
        dr6: 0xFFFF0FF0, // Default value after reset
        dr7: 0x00000400, // Default value after reset
    }
}

/// Convert our SystemRegisters type to KVM system registers.
pub fn sregs_to_kvm(sregs: &SystemRegisters) -> kvm_sregs {
    kvm_sregs {
        cs: segment_to_kvm(&sregs.cs),
        ds: segment_to_kvm(&sregs.ds),
        es: segment_to_kvm(&sregs.es),
        fs: segment_to_kvm(&sregs.fs),
        gs: segment_to_kvm(&sregs.gs),
        ss: segment_to_kvm(&sregs.ss),
        tr: segment_to_kvm(&sregs.tr),
        ldt: segment_to_kvm(&sregs.ldt),
        gdt: dtable_to_kvm(&sregs.gdt),
        idt: dtable_to_kvm(&sregs.idt),
        cr0: sregs.cr0,
        cr2: sregs.cr2,
        cr3: sregs.cr3,
        cr4: sregs.cr4,
        cr8: sregs.cr8,
        efer: sregs.efer,
        apic_base: 0,
        interrupt_bitmap: [0; 4],
    }
}
