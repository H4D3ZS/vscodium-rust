//! FFI bindings for Apple's Hypervisor.framework with Rosetta x86_64 support.
//!
//! This module provides safe Rust wrappers around the Hypervisor.framework C API
//! for creating and managing virtual machines on macOS. On Apple Silicon Macs,
//! Rosetta 2 is used to translate x86_64 guest code.
//!
//! Reference: https://developer.apple.com/documentation/hypervisor

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::ffi::c_void;

/// Return type for Hypervisor.framework functions
pub type hv_return_t = i32;

/// VM handle type
pub type hv_vm_t = *mut c_void;

/// vCPU handle type (opaque, represented as u64 in the API)
pub type hv_vcpuid_t = u64;

/// Guest physical address
pub type hv_gpaddr_t = u64;

/// User virtual address (host)
pub type hv_uvaddr_t = *mut c_void;

/// Memory flags for mapping
pub type hv_memory_flags_t = u64;

/// VMX capability type
pub type hv_vmx_capability_t = u32;

// HV return codes
pub const HV_SUCCESS: hv_return_t = 0;
pub const HV_ERROR: hv_return_t = -85377023; // 0xfae94001
pub const HV_BUSY: hv_return_t = -85377022;
pub const HV_BAD_ARGUMENT: hv_return_t = -85377021;
pub const HV_NO_RESOURCES: hv_return_t = -85377020;
pub const HV_NO_DEVICE: hv_return_t = -85377019;
pub const HV_DENIED: hv_return_t = -85377018;
pub const HV_UNSUPPORTED: hv_return_t = -85377017;

// Memory mapping flags
pub const HV_MEMORY_READ: hv_memory_flags_t = 1 << 0;
pub const HV_MEMORY_WRITE: hv_memory_flags_t = 1 << 1;
pub const HV_MEMORY_EXEC: hv_memory_flags_t = 1 << 2;

// x86_64 register definitions for Rosetta
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hv_x86_reg_t {
    // General purpose registers
    HV_X86_RIP = 0,
    HV_X86_RFLAGS = 1,
    HV_X86_RAX = 2,
    HV_X86_RCX = 3,
    HV_X86_RDX = 4,
    HV_X86_RBX = 5,
    HV_X86_RSI = 6,
    HV_X86_RDI = 7,
    HV_X86_RSP = 8,
    HV_X86_RBP = 9,
    HV_X86_R8 = 10,
    HV_X86_R9 = 11,
    HV_X86_R10 = 12,
    HV_X86_R11 = 13,
    HV_X86_R12 = 14,
    HV_X86_R13 = 15,
    HV_X86_R14 = 16,
    HV_X86_R15 = 17,

    // Segment registers
    HV_X86_CS = 18,
    HV_X86_SS = 19,
    HV_X86_DS = 20,
    HV_X86_ES = 21,
    HV_X86_FS = 22,
    HV_X86_GS = 23,
    HV_X86_LDT = 24,
    HV_X86_TR = 25,

    // Descriptor table registers
    HV_X86_IDTR = 26,
    HV_X86_GDTR = 27,

    // Control registers
    HV_X86_CR0 = 28,
    HV_X86_CR1 = 29,
    HV_X86_CR2 = 30,
    HV_X86_CR3 = 31,
    HV_X86_CR4 = 32,

    // Debug registers
    HV_X86_DR0 = 33,
    HV_X86_DR1 = 34,
    HV_X86_DR2 = 35,
    HV_X86_DR3 = 36,
    HV_X86_DR4 = 37,
    HV_X86_DR5 = 38,
    HV_X86_DR6 = 39,
    HV_X86_DR7 = 40,

    // Extended feature enable register
    HV_X86_EFER = 41,

    // XCR0 (extended control register)
    HV_X86_XCR0 = 42,
    // FPU registers will be handled via FXSAVE/FXRSTOR
}

// VMCS field encodings for VMX (Intel-style, used by Rosetta)
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hv_vmx_vmcs_field_t {
    // 16-bit guest state fields
    VMCS_GUEST_ES_SELECTOR = 0x00000800,
    VMCS_GUEST_CS_SELECTOR = 0x00000802,
    VMCS_GUEST_SS_SELECTOR = 0x00000804,
    VMCS_GUEST_DS_SELECTOR = 0x00000806,
    VMCS_GUEST_FS_SELECTOR = 0x00000808,
    VMCS_GUEST_GS_SELECTOR = 0x0000080a,
    VMCS_GUEST_LDTR_SELECTOR = 0x0000080c,
    VMCS_GUEST_TR_SELECTOR = 0x0000080e,

    // 64-bit guest state fields
    VMCS_GUEST_LINK_POINTER = 0x00002800,
    VMCS_GUEST_IA32_DEBUGCTL = 0x00002802,
    VMCS_GUEST_IA32_PAT = 0x00002804,
    VMCS_GUEST_IA32_EFER = 0x00002806,
    VMCS_GUEST_IA32_PERF_GLOBAL_CTRL = 0x00002808,
    VMCS_GUEST_PDPTE0 = 0x0000280a,
    VMCS_GUEST_PDPTE1 = 0x0000280c,
    VMCS_GUEST_PDPTE2 = 0x0000280e,
    VMCS_GUEST_PDPTE3 = 0x00002810,

    // 32-bit guest state fields
    VMCS_GUEST_ES_LIMIT = 0x00004800,
    VMCS_GUEST_CS_LIMIT = 0x00004802,
    VMCS_GUEST_SS_LIMIT = 0x00004804,
    VMCS_GUEST_DS_LIMIT = 0x00004806,
    VMCS_GUEST_FS_LIMIT = 0x00004808,
    VMCS_GUEST_GS_LIMIT = 0x0000480a,
    VMCS_GUEST_LDTR_LIMIT = 0x0000480c,
    VMCS_GUEST_TR_LIMIT = 0x0000480e,
    VMCS_GUEST_GDTR_LIMIT = 0x00004810,
    VMCS_GUEST_IDTR_LIMIT = 0x00004812,
    VMCS_GUEST_ES_ACCESS_RIGHTS = 0x00004814,
    VMCS_GUEST_CS_ACCESS_RIGHTS = 0x00004816,
    VMCS_GUEST_SS_ACCESS_RIGHTS = 0x00004818,
    VMCS_GUEST_DS_ACCESS_RIGHTS = 0x0000481a,
    VMCS_GUEST_FS_ACCESS_RIGHTS = 0x0000481c,
    VMCS_GUEST_GS_ACCESS_RIGHTS = 0x0000481e,
    VMCS_GUEST_LDTR_ACCESS_RIGHTS = 0x00004820,
    VMCS_GUEST_TR_ACCESS_RIGHTS = 0x00004822,
    VMCS_GUEST_INTERRUPTIBILITY_STATE = 0x00004824,
    VMCS_GUEST_ACTIVITY_STATE = 0x00004826,
    VMCS_GUEST_SMBASE = 0x00004828,
    VMCS_GUEST_IA32_SYSENTER_CS = 0x0000482a,

    // Natural-width guest state fields
    VMCS_GUEST_CR0 = 0x00006800,
    VMCS_GUEST_CR3 = 0x00006802,
    VMCS_GUEST_CR4 = 0x00006804,
    VMCS_GUEST_ES_BASE = 0x00006806,
    VMCS_GUEST_CS_BASE = 0x00006808,
    VMCS_GUEST_SS_BASE = 0x0000680a,
    VMCS_GUEST_DS_BASE = 0x0000680c,
    VMCS_GUEST_FS_BASE = 0x0000680e,
    VMCS_GUEST_GS_BASE = 0x00006810,
    VMCS_GUEST_LDTR_BASE = 0x00006812,
    VMCS_GUEST_TR_BASE = 0x00006814,
    VMCS_GUEST_GDTR_BASE = 0x00006816,
    VMCS_GUEST_IDTR_BASE = 0x00006818,
    VMCS_GUEST_DR7 = 0x0000681a,
    VMCS_GUEST_RSP = 0x0000681c,
    VMCS_GUEST_RIP = 0x0000681e,
    VMCS_GUEST_RFLAGS = 0x00006820,
    VMCS_GUEST_PENDING_DBG_EXCEPTIONS = 0x00006822,
    VMCS_GUEST_IA32_SYSENTER_ESP = 0x00006824,
    VMCS_GUEST_IA32_SYSENTER_EIP = 0x00006826,

    // VM-exit information fields
    VMCS_EXIT_REASON = 0x00004402,
    VMCS_EXIT_QUALIFICATION = 0x00006400,
    VMCS_GUEST_LINEAR_ADDRESS = 0x0000640a,
    VMCS_GUEST_PHYSICAL_ADDRESS = 0x00002400,
    VMCS_EXIT_INSTRUCTION_LENGTH = 0x0000440c,
    VMCS_EXIT_INSTRUCTION_INFO = 0x0000440e,
    VMCS_IDT_VECTORING_INFO = 0x00004408,
    VMCS_IDT_VECTORING_ERROR_CODE = 0x0000440a,

    // VM-entry control fields
    VMCS_CTRL_VMENTRY_CONTROLS = 0x00004012,
    VMCS_CTRL_VMENTRY_IRQ_INFO = 0x00004016,
    VMCS_CTRL_VMENTRY_EXCEPTION_ERROR = 0x00004018,
    VMCS_CTRL_VMENTRY_INSTRUCTION_LENGTH = 0x0000401a,

    // Primary/secondary processor-based controls
    VMCS_CTRL_PIN_BASED = 0x00004000,
    VMCS_CTRL_CPU_BASED = 0x00004002,
    VMCS_CTRL_CPU_BASED2 = 0x0000401e,
    VMCS_CTRL_EXC_BITMAP = 0x00004004,
    VMCS_CTRL_CR0_MASK = 0x00006000,
    VMCS_CTRL_CR4_MASK = 0x00006002,
    VMCS_CTRL_CR0_SHADOW = 0x00006004,
    VMCS_CTRL_CR4_SHADOW = 0x00006006,

    // I/O bitmap addresses
    VMCS_CTRL_IO_BITMAP_A = 0x00002000,
    VMCS_CTRL_IO_BITMAP_B = 0x00002002,

    // MSR bitmaps
    VMCS_CTRL_MSR_BITMAPS = 0x00002004,

    // EPT pointer
    VMCS_CTRL_EPTP = 0x0000201a,
}

// VMX exit reasons
pub const VMX_EXIT_REASON_EXCEPTION_NMI: u32 = 0;
pub const VMX_EXIT_REASON_EXT_INTERRUPT: u32 = 1;
pub const VMX_EXIT_REASON_TRIPLE_FAULT: u32 = 2;
pub const VMX_EXIT_REASON_INIT_SIGNAL: u32 = 3;
pub const VMX_EXIT_REASON_SIPI: u32 = 4;
pub const VMX_EXIT_REASON_IO_SMI: u32 = 5;
pub const VMX_EXIT_REASON_OTHER_SMI: u32 = 6;
pub const VMX_EXIT_REASON_IRQ_WINDOW: u32 = 7;
pub const VMX_EXIT_REASON_NMI_WINDOW: u32 = 8;
pub const VMX_EXIT_REASON_TASK_SWITCH: u32 = 9;
pub const VMX_EXIT_REASON_CPUID: u32 = 10;
pub const VMX_EXIT_REASON_GETSEC: u32 = 11;
pub const VMX_EXIT_REASON_HLT: u32 = 12;
pub const VMX_EXIT_REASON_INVD: u32 = 13;
pub const VMX_EXIT_REASON_INVLPG: u32 = 14;
pub const VMX_EXIT_REASON_RDPMC: u32 = 15;
pub const VMX_EXIT_REASON_RDTSC: u32 = 16;
pub const VMX_EXIT_REASON_RSM: u32 = 17;
pub const VMX_EXIT_REASON_VMCALL: u32 = 18;
pub const VMX_EXIT_REASON_VMCLEAR: u32 = 19;
pub const VMX_EXIT_REASON_VMLAUNCH: u32 = 20;
pub const VMX_EXIT_REASON_VMPTRLD: u32 = 21;
pub const VMX_EXIT_REASON_VMPTRST: u32 = 22;
pub const VMX_EXIT_REASON_VMREAD: u32 = 23;
pub const VMX_EXIT_REASON_VMRESUME: u32 = 24;
pub const VMX_EXIT_REASON_VMWRITE: u32 = 25;
pub const VMX_EXIT_REASON_VMOFF: u32 = 26;
pub const VMX_EXIT_REASON_VMON: u32 = 27;
pub const VMX_EXIT_REASON_CR_ACCESS: u32 = 28;
pub const VMX_EXIT_REASON_DR_ACCESS: u32 = 29;
pub const VMX_EXIT_REASON_IO: u32 = 30;
pub const VMX_EXIT_REASON_RDMSR: u32 = 31;
pub const VMX_EXIT_REASON_WRMSR: u32 = 32;
pub const VMX_EXIT_REASON_ENTRY_GUEST_STATE: u32 = 33;
pub const VMX_EXIT_REASON_ENTRY_MSR_LOAD: u32 = 34;
pub const VMX_EXIT_REASON_MWAIT: u32 = 36;
pub const VMX_EXIT_REASON_MONITOR_TRAP: u32 = 37;
pub const VMX_EXIT_REASON_MONITOR: u32 = 39;
pub const VMX_EXIT_REASON_PAUSE: u32 = 40;
pub const VMX_EXIT_REASON_ENTRY_MCE: u32 = 41;
pub const VMX_EXIT_REASON_TPR_THRESHOLD: u32 = 43;
pub const VMX_EXIT_REASON_APIC_ACCESS: u32 = 44;
pub const VMX_EXIT_REASON_VIRTUALIZED_EOI: u32 = 45;
pub const VMX_EXIT_REASON_GDTR_IDTR_ACCESS: u32 = 46;
pub const VMX_EXIT_REASON_LDTR_TR_ACCESS: u32 = 47;
pub const VMX_EXIT_REASON_EPT_VIOLATION: u32 = 48;
pub const VMX_EXIT_REASON_EPT_MISCONFIGURATION: u32 = 49;
pub const VMX_EXIT_REASON_INVEPT: u32 = 50;
pub const VMX_EXIT_REASON_RDTSCP: u32 = 51;
pub const VMX_EXIT_REASON_VMX_PREEMPTION_TIMER: u32 = 52;
pub const VMX_EXIT_REASON_INVVPID: u32 = 53;
pub const VMX_EXIT_REASON_WBINVD: u32 = 54;
pub const VMX_EXIT_REASON_XSETBV: u32 = 55;
pub const VMX_EXIT_REASON_APIC_WRITE: u32 = 56;
pub const VMX_EXIT_REASON_RDRAND: u32 = 57;
pub const VMX_EXIT_REASON_INVPCID: u32 = 58;
pub const VMX_EXIT_REASON_VMFUNC: u32 = 59;
pub const VMX_EXIT_REASON_ENCLS: u32 = 60;
pub const VMX_EXIT_REASON_RDSEED: u32 = 61;
pub const VMX_EXIT_REASON_PML_FULL: u32 = 62;
pub const VMX_EXIT_REASON_XSAVES: u32 = 63;
pub const VMX_EXIT_REASON_XRSTORS: u32 = 64;

// VMX capability flags
pub const HV_VMX_CAP_PINBASED: hv_vmx_capability_t = 0;
pub const HV_VMX_CAP_PROCBASED: hv_vmx_capability_t = 1;
pub const HV_VMX_CAP_PROCBASED2: hv_vmx_capability_t = 2;
pub const HV_VMX_CAP_ENTRY: hv_vmx_capability_t = 3;
pub const HV_VMX_CAP_EXIT: hv_vmx_capability_t = 4;
pub const HV_VMX_CAP_PREEMPTION_TIMER: hv_vmx_capability_t = 32;

// Pin-based VM-execution controls
pub const PIN_BASED_EXT_INTR_EXIT: u64 = 1 << 0;
pub const PIN_BASED_NMI_EXIT: u64 = 1 << 3;
pub const PIN_BASED_VIRTUAL_NMIS: u64 = 1 << 5;
pub const PIN_BASED_VMX_PREEMPTION_TIMER: u64 = 1 << 6;

// Primary processor-based VM-execution controls
pub const CPU_BASED_IRQ_WINDOW_EXIT: u64 = 1 << 2;
pub const CPU_BASED_TSC_OFFSET: u64 = 1 << 3;
pub const CPU_BASED_HLT_EXIT: u64 = 1 << 7;
pub const CPU_BASED_INVLPG_EXIT: u64 = 1 << 9;
pub const CPU_BASED_MWAIT_EXIT: u64 = 1 << 10;
pub const CPU_BASED_RDPMC_EXIT: u64 = 1 << 11;
pub const CPU_BASED_RDTSC_EXIT: u64 = 1 << 12;
pub const CPU_BASED_CR3_LOAD_EXIT: u64 = 1 << 15;
pub const CPU_BASED_CR3_STORE_EXIT: u64 = 1 << 16;
pub const CPU_BASED_CR8_LOAD_EXIT: u64 = 1 << 19;
pub const CPU_BASED_CR8_STORE_EXIT: u64 = 1 << 20;
pub const CPU_BASED_TPR_SHADOW: u64 = 1 << 21;
pub const CPU_BASED_NMI_WINDOW_EXIT: u64 = 1 << 22;
pub const CPU_BASED_MOV_DR_EXIT: u64 = 1 << 23;
pub const CPU_BASED_UNCOND_IO_EXIT: u64 = 1 << 24;
pub const CPU_BASED_USE_IO_BITMAPS: u64 = 1 << 25;
pub const CPU_BASED_MONITOR_TRAP: u64 = 1 << 27;
pub const CPU_BASED_USE_MSR_BITMAPS: u64 = 1 << 28;
pub const CPU_BASED_MONITOR_EXIT: u64 = 1 << 29;
pub const CPU_BASED_PAUSE_EXIT: u64 = 1 << 30;
pub const CPU_BASED_SECONDARY_CONTROLS: u64 = 1 << 31;

// Secondary processor-based VM-execution controls
pub const CPU_BASED2_VIRTUAL_APIC: u64 = 1 << 0;
pub const CPU_BASED2_EPT: u64 = 1 << 1;
pub const CPU_BASED2_DESC_TABLE_EXIT: u64 = 1 << 2;
pub const CPU_BASED2_RDTSCP: u64 = 1 << 3;
pub const CPU_BASED2_VIRTUAL_X2APIC: u64 = 1 << 4;
pub const CPU_BASED2_VPID: u64 = 1 << 5;
pub const CPU_BASED2_WBINVD_EXIT: u64 = 1 << 6;
pub const CPU_BASED2_UNRESTRICTED_GUEST: u64 = 1 << 7;
pub const CPU_BASED2_APIC_REGISTER_VIRT: u64 = 1 << 8;
pub const CPU_BASED2_VIRTUAL_INTR_DELIVERY: u64 = 1 << 9;
pub const CPU_BASED2_PAUSE_LOOP_EXIT: u64 = 1 << 10;
pub const CPU_BASED2_RDRAND_EXIT: u64 = 1 << 11;
pub const CPU_BASED2_INVPCID: u64 = 1 << 12;
pub const CPU_BASED2_VMFUNC: u64 = 1 << 13;
pub const CPU_BASED2_VMCS_SHADOW: u64 = 1 << 14;
pub const CPU_BASED2_ENCLS_EXIT: u64 = 1 << 15;
pub const CPU_BASED2_RDSEED_EXIT: u64 = 1 << 16;
pub const CPU_BASED2_PML: u64 = 1 << 17;
pub const CPU_BASED2_EPT_VIOLATION_VE: u64 = 1 << 18;
pub const CPU_BASED2_PT_CONCEAL: u64 = 1 << 19;
pub const CPU_BASED2_XSAVES_XRSTORS: u64 = 1 << 20;
pub const CPU_BASED2_TSC_SCALING: u64 = 1 << 25;

// VM-entry controls
pub const VMENTRY_LOAD_DBG_CONTROLS: u64 = 1 << 2;
pub const VMENTRY_GUEST_IA32E: u64 = 1 << 9;
pub const VMENTRY_SMM_ENTRY: u64 = 1 << 10;
pub const VMENTRY_DEACTIVATE_DUAL_MONITOR: u64 = 1 << 11;
pub const VMENTRY_LOAD_IA32_PERF_GLOBAL_CTRL: u64 = 1 << 13;
pub const VMENTRY_LOAD_IA32_PAT: u64 = 1 << 14;
pub const VMENTRY_LOAD_IA32_EFER: u64 = 1 << 15;

// VM-exit controls
pub const VMEXIT_SAVE_DBG_CONTROLS: u64 = 1 << 2;
pub const VMEXIT_HOST_ADDR_SPACE_SIZE: u64 = 1 << 9;
pub const VMEXIT_LOAD_IA32_PERF_GLOBAL_CTRL: u64 = 1 << 12;
pub const VMEXIT_ACK_INTERRUPT: u64 = 1 << 15;
pub const VMEXIT_SAVE_IA32_PAT: u64 = 1 << 18;
pub const VMEXIT_LOAD_IA32_PAT: u64 = 1 << 19;
pub const VMEXIT_SAVE_IA32_EFER: u64 = 1 << 20;
pub const VMEXIT_LOAD_IA32_EFER: u64 = 1 << 21;
pub const VMEXIT_SAVE_VMX_PREEMPTION_TIMER: u64 = 1 << 22;

// Interrupt types for VM-entry interrupt info
pub const INTR_TYPE_EXT_INTR: u32 = 0;
pub const INTR_TYPE_NMI: u32 = 2;
pub const INTR_TYPE_HARD_EXCEPTION: u32 = 3;
pub const INTR_TYPE_SOFT_INTR: u32 = 4;
pub const INTR_TYPE_PRIV_SW_EXCEPTION: u32 = 5;
pub const INTR_TYPE_SOFT_EXCEPTION: u32 = 6;

// Interrupt info field bit positions
pub const INTR_INFO_VECTOR_MASK: u32 = 0xff;
pub const INTR_INFO_TYPE_SHIFT: u32 = 8;
pub const INTR_INFO_TYPE_MASK: u32 = 0x7 << INTR_INFO_TYPE_SHIFT;
pub const INTR_INFO_DELIVER_ERR_CODE: u32 = 1 << 11;
pub const INTR_INFO_VALID: u32 = 1 << 31;

// EFER bits
pub const EFER_SCE: u64 = 1 << 0; // System Call Extensions
pub const EFER_LME: u64 = 1 << 8; // Long Mode Enable
pub const EFER_LMA: u64 = 1 << 10; // Long Mode Active
pub const EFER_NXE: u64 = 1 << 11; // No-Execute Enable

// CR0 bits
pub const CR0_PE: u64 = 1 << 0; // Protection Enable
pub const CR0_MP: u64 = 1 << 1; // Monitor Coprocessor
pub const CR0_EM: u64 = 1 << 2; // Emulation
pub const CR0_TS: u64 = 1 << 3; // Task Switched
pub const CR0_ET: u64 = 1 << 4; // Extension Type
pub const CR0_NE: u64 = 1 << 5; // Numeric Error
pub const CR0_WP: u64 = 1 << 16; // Write Protect
pub const CR0_AM: u64 = 1 << 18; // Alignment Mask
pub const CR0_NW: u64 = 1 << 29; // Not Write-through
pub const CR0_CD: u64 = 1 << 30; // Cache Disable
pub const CR0_PG: u64 = 1 << 31; // Paging

// CR4 bits
pub const CR4_VME: u64 = 1 << 0; // Virtual-8086 Mode Extensions
pub const CR4_PVI: u64 = 1 << 1; // Protected-Mode Virtual Interrupts
pub const CR4_TSD: u64 = 1 << 2; // Time Stamp Disable
pub const CR4_DE: u64 = 1 << 3; // Debugging Extensions
pub const CR4_PSE: u64 = 1 << 4; // Page Size Extensions
pub const CR4_PAE: u64 = 1 << 5; // Physical Address Extension
pub const CR4_MCE: u64 = 1 << 6; // Machine-Check Enable
pub const CR4_PGE: u64 = 1 << 7; // Page Global Enable
pub const CR4_PCE: u64 = 1 << 8; // Performance-Monitoring Counter Enable
pub const CR4_OSFXSR: u64 = 1 << 9; // OS FXSAVE/FXRSTOR Support
pub const CR4_OSXMMEXCPT: u64 = 1 << 10; // OS Unmasked Exception Support
pub const CR4_UMIP: u64 = 1 << 11; // User-Mode Instruction Prevention
pub const CR4_VMXE: u64 = 1 << 13; // VMX Enable
pub const CR4_SMXE: u64 = 1 << 14; // SMX Enable
pub const CR4_FSGSBASE: u64 = 1 << 16; // FSGSBASE Enable
pub const CR4_PCIDE: u64 = 1 << 17; // PCID Enable
pub const CR4_OSXSAVE: u64 = 1 << 18; // XSAVE and Processor Extended States Enable
pub const CR4_SMEP: u64 = 1 << 20; // SMEP Enable
pub const CR4_SMAP: u64 = 1 << 21; // SMAP Enable
pub const CR4_PKE: u64 = 1 << 22; // Protection Key Enable

// RFLAGS bits
pub const RFLAGS_CF: u64 = 1 << 0; // Carry Flag
pub const RFLAGS_PF: u64 = 1 << 2; // Parity Flag
pub const RFLAGS_AF: u64 = 1 << 4; // Auxiliary Carry Flag
pub const RFLAGS_ZF: u64 = 1 << 6; // Zero Flag
pub const RFLAGS_SF: u64 = 1 << 7; // Sign Flag
pub const RFLAGS_TF: u64 = 1 << 8; // Trap Flag
pub const RFLAGS_IF: u64 = 1 << 9; // Interrupt Enable Flag
pub const RFLAGS_DF: u64 = 1 << 10; // Direction Flag
pub const RFLAGS_OF: u64 = 1 << 11; // Overflow Flag
pub const RFLAGS_IOPL_MASK: u64 = 3 << 12; // I/O Privilege Level
pub const RFLAGS_NT: u64 = 1 << 14; // Nested Task
pub const RFLAGS_RF: u64 = 1 << 16; // Resume Flag
pub const RFLAGS_VM: u64 = 1 << 17; // Virtual-8086 Mode
pub const RFLAGS_AC: u64 = 1 << 18; // Alignment Check
pub const RFLAGS_VIF: u64 = 1 << 19; // Virtual Interrupt Flag
pub const RFLAGS_VIP: u64 = 1 << 20; // Virtual Interrupt Pending
pub const RFLAGS_ID: u64 = 1 << 21; // ID Flag
pub const RFLAGS_RESERVED_1: u64 = 1 << 1; // Always 1

// External function declarations from Hypervisor.framework
#[link(name = "Hypervisor", kind = "framework")]
unsafe extern "C" {
    /// Create a VM instance.
    pub fn hv_vm_create(flags: u64) -> hv_return_t;

    /// Destroy the VM instance.
    pub fn hv_vm_destroy() -> hv_return_t;

    /// Map memory from host to guest physical address space.
    pub fn hv_vm_map(
        uva: hv_uvaddr_t,
        gpa: hv_gpaddr_t,
        size: usize,
        flags: hv_memory_flags_t,
    ) -> hv_return_t;

    /// Unmap memory from guest physical address space.
    pub fn hv_vm_unmap(gpa: hv_gpaddr_t, size: usize) -> hv_return_t;

    /// Protect memory region with new flags.
    pub fn hv_vm_protect(gpa: hv_gpaddr_t, size: usize, flags: hv_memory_flags_t) -> hv_return_t;

    /// Create a vCPU.
    pub fn hv_vcpu_create(vcpu: *mut hv_vcpuid_t, flags: u64) -> hv_return_t;

    /// Destroy a vCPU.
    pub fn hv_vcpu_destroy(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Run a vCPU until exit.
    pub fn hv_vcpu_run(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Run a vCPU until exit or timeout.
    pub fn hv_vcpu_run_until(vcpu: hv_vcpuid_t, deadline: u64) -> hv_return_t;

    /// Interrupt a running vCPU.
    pub fn hv_vcpu_interrupt(vcpus: *const hv_vcpuid_t, vcpu_count: u32) -> hv_return_t;

    /// Read a VMCS field.
    pub fn hv_vmx_vcpu_read_vmcs(vcpu: hv_vcpuid_t, field: u32, value: *mut u64) -> hv_return_t;

    /// Write a VMCS field.
    pub fn hv_vmx_vcpu_write_vmcs(vcpu: hv_vcpuid_t, field: u32, value: u64) -> hv_return_t;

    /// Get VMX capabilities.
    pub fn hv_vmx_get_capability(field: hv_vmx_capability_t, value: *mut u64) -> hv_return_t;

    /// Read vCPU register.
    pub fn hv_vcpu_read_register(vcpu: hv_vcpuid_t, reg: u32, value: *mut u64) -> hv_return_t;

    /// Write vCPU register.
    pub fn hv_vcpu_write_register(vcpu: hv_vcpuid_t, reg: u32, value: u64) -> hv_return_t;

    /// Read vCPU floating point state.
    pub fn hv_vcpu_read_fpstate(vcpu: hv_vcpuid_t, buffer: *mut c_void, size: usize)
    -> hv_return_t;

    /// Write vCPU floating point state.
    pub fn hv_vcpu_write_fpstate(
        vcpu: hv_vcpuid_t,
        buffer: *const c_void,
        size: usize,
    ) -> hv_return_t;

    /// Enable native vCPU capabilities (needed for x86 on ARM).
    pub fn hv_vcpu_set_trap_debug_exceptions(vcpu: hv_vcpuid_t, enable: bool) -> hv_return_t;

    /// Set trap for debug register access.
    pub fn hv_vcpu_set_trap_debug_reg_accesses(vcpu: hv_vcpuid_t, enable: bool) -> hv_return_t;

    /// Force flush vCPU state.
    pub fn hv_vcpu_flush(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Invalidate TLB.
    pub fn hv_vcpu_invalidate_tlb(vcpu: hv_vcpuid_t) -> hv_return_t;

    /// Sync TSC across vCPUs.
    pub fn hv_vm_sync_tsc(tsc: u64) -> hv_return_t;
}

/// Convert HV return code to human-readable string.
pub fn hv_error_string(ret: hv_return_t) -> &'static str {
    match ret {
        HV_SUCCESS => "Success",
        HV_ERROR => "Error",
        HV_BUSY => "Busy",
        HV_BAD_ARGUMENT => "Bad argument",
        HV_NO_RESOURCES => "No resources",
        HV_NO_DEVICE => "No device",
        HV_DENIED => "Permission denied (check entitlements)",
        HV_UNSUPPORTED => "Unsupported operation",
        _ => "Unknown error",
    }
}

/// Check if Hypervisor.framework is available and we have permission to use it.
pub fn hv_check_available() -> Result<(), &'static str> {
    // Try to create and immediately destroy a VM to check availability
    let ret = unsafe { hv_vm_create(0) };
    if ret == HV_SUCCESS {
        unsafe { hv_vm_destroy() };
        Ok(())
    } else if ret == HV_DENIED {
        Err("Hypervisor access denied. Add com.apple.security.hypervisor entitlement.")
    } else if ret == HV_NO_DEVICE {
        Err("Hypervisor not available on this system.")
    } else if ret == HV_UNSUPPORTED {
        Err("Hypervisor not supported (Rosetta x86 translation may not be available).")
    } else {
        Err("Failed to initialize Hypervisor.framework")
    }
}
