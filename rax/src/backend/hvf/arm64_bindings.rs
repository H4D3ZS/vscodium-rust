//! FFI bindings for Apple's Hypervisor.framework ARM64 support.
//!
//! This module provides ARM64-specific bindings for Hypervisor.framework
//! on Apple Silicon Macs. These APIs are only available on aarch64 macOS.
//!
//! Reference: https://developer.apple.com/documentation/hypervisor

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::ffi::c_void;

// Re-use common types from the x86 bindings
pub use super::bindings::{
    HV_BAD_ARGUMENT, HV_BUSY, HV_DENIED, HV_ERROR, HV_MEMORY_EXEC, HV_MEMORY_READ, HV_MEMORY_WRITE,
    HV_NO_DEVICE, HV_NO_RESOURCES, HV_SUCCESS, HV_UNSUPPORTED, hv_error_string, hv_gpaddr_t,
    hv_memory_flags_t, hv_return_t, hv_uvaddr_t,
};

/// ARM64 vCPU handle type
pub type hv_vcpu_t = *mut c_void;

/// ARM64 vCPU exit information
pub type hv_vcpu_exit_t = *mut hv_vcpu_exit;

/// ARM64 register type
pub type hv_reg_t = u32;

/// ARM64 system register type (encoding as u16)
pub type hv_sys_reg_t = u16;

/// ARM64 SIMD/FP register type
pub type hv_simd_fp_reg_t = u32;

// ARM64 general-purpose register indices (X0-X30, plus SP, PC, CPSR)
pub const HV_REG_X0: hv_reg_t = 0;
pub const HV_REG_X1: hv_reg_t = 1;
pub const HV_REG_X2: hv_reg_t = 2;
pub const HV_REG_X3: hv_reg_t = 3;
pub const HV_REG_X4: hv_reg_t = 4;
pub const HV_REG_X5: hv_reg_t = 5;
pub const HV_REG_X6: hv_reg_t = 6;
pub const HV_REG_X7: hv_reg_t = 7;
pub const HV_REG_X8: hv_reg_t = 8;
pub const HV_REG_X9: hv_reg_t = 9;
pub const HV_REG_X10: hv_reg_t = 10;
pub const HV_REG_X11: hv_reg_t = 11;
pub const HV_REG_X12: hv_reg_t = 12;
pub const HV_REG_X13: hv_reg_t = 13;
pub const HV_REG_X14: hv_reg_t = 14;
pub const HV_REG_X15: hv_reg_t = 15;
pub const HV_REG_X16: hv_reg_t = 16;
pub const HV_REG_X17: hv_reg_t = 17;
pub const HV_REG_X18: hv_reg_t = 18;
pub const HV_REG_X19: hv_reg_t = 19;
pub const HV_REG_X20: hv_reg_t = 20;
pub const HV_REG_X21: hv_reg_t = 21;
pub const HV_REG_X22: hv_reg_t = 22;
pub const HV_REG_X23: hv_reg_t = 23;
pub const HV_REG_X24: hv_reg_t = 24;
pub const HV_REG_X25: hv_reg_t = 25;
pub const HV_REG_X26: hv_reg_t = 26;
pub const HV_REG_X27: hv_reg_t = 27;
pub const HV_REG_X28: hv_reg_t = 28;
pub const HV_REG_X29: hv_reg_t = 29; // Frame pointer (FP)
pub const HV_REG_X30: hv_reg_t = 30; // Link register (LR)
pub const HV_REG_PC: hv_reg_t = 31;
pub const HV_REG_FPCR: hv_reg_t = 32;
pub const HV_REG_FPSR: hv_reg_t = 33;
pub const HV_REG_CPSR: hv_reg_t = 34;

// ARM64 SIMD/FP register indices (Q0-Q31, 128-bit each)
pub const HV_SIMD_FP_REG_Q0: hv_simd_fp_reg_t = 0;
pub const HV_SIMD_FP_REG_Q1: hv_simd_fp_reg_t = 1;
pub const HV_SIMD_FP_REG_Q2: hv_simd_fp_reg_t = 2;
pub const HV_SIMD_FP_REG_Q3: hv_simd_fp_reg_t = 3;
pub const HV_SIMD_FP_REG_Q4: hv_simd_fp_reg_t = 4;
pub const HV_SIMD_FP_REG_Q5: hv_simd_fp_reg_t = 5;
pub const HV_SIMD_FP_REG_Q6: hv_simd_fp_reg_t = 6;
pub const HV_SIMD_FP_REG_Q7: hv_simd_fp_reg_t = 7;
pub const HV_SIMD_FP_REG_Q8: hv_simd_fp_reg_t = 8;
pub const HV_SIMD_FP_REG_Q9: hv_simd_fp_reg_t = 9;
pub const HV_SIMD_FP_REG_Q10: hv_simd_fp_reg_t = 10;
pub const HV_SIMD_FP_REG_Q11: hv_simd_fp_reg_t = 11;
pub const HV_SIMD_FP_REG_Q12: hv_simd_fp_reg_t = 12;
pub const HV_SIMD_FP_REG_Q13: hv_simd_fp_reg_t = 13;
pub const HV_SIMD_FP_REG_Q14: hv_simd_fp_reg_t = 14;
pub const HV_SIMD_FP_REG_Q15: hv_simd_fp_reg_t = 15;
pub const HV_SIMD_FP_REG_Q16: hv_simd_fp_reg_t = 16;
pub const HV_SIMD_FP_REG_Q17: hv_simd_fp_reg_t = 17;
pub const HV_SIMD_FP_REG_Q18: hv_simd_fp_reg_t = 18;
pub const HV_SIMD_FP_REG_Q19: hv_simd_fp_reg_t = 19;
pub const HV_SIMD_FP_REG_Q20: hv_simd_fp_reg_t = 20;
pub const HV_SIMD_FP_REG_Q21: hv_simd_fp_reg_t = 21;
pub const HV_SIMD_FP_REG_Q22: hv_simd_fp_reg_t = 22;
pub const HV_SIMD_FP_REG_Q23: hv_simd_fp_reg_t = 23;
pub const HV_SIMD_FP_REG_Q24: hv_simd_fp_reg_t = 24;
pub const HV_SIMD_FP_REG_Q25: hv_simd_fp_reg_t = 25;
pub const HV_SIMD_FP_REG_Q26: hv_simd_fp_reg_t = 26;
pub const HV_SIMD_FP_REG_Q27: hv_simd_fp_reg_t = 27;
pub const HV_SIMD_FP_REG_Q28: hv_simd_fp_reg_t = 28;
pub const HV_SIMD_FP_REG_Q29: hv_simd_fp_reg_t = 29;
pub const HV_SIMD_FP_REG_Q30: hv_simd_fp_reg_t = 30;
pub const HV_SIMD_FP_REG_Q31: hv_simd_fp_reg_t = 31;

// ARM64 system register encodings (encoded as op0:op1:CRn:CRm:op2)
// These are the register indices used by hv_vcpu_get_sys_reg/hv_vcpu_set_sys_reg

/// Encode system register from op0, op1, CRn, CRm, op2
pub const fn encode_sys_reg(op0: u8, op1: u8, crn: u8, crm: u8, op2: u8) -> hv_sys_reg_t {
    ((op0 as u16) << 14)
        | ((op1 as u16) << 11)
        | ((crn as u16) << 7)
        | ((crm as u16) << 3)
        | (op2 as u16)
}

// Common system registers (EL1)
pub const HV_SYS_REG_MPIDR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 0, 0, 5);
pub const HV_SYS_REG_SCTLR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 1, 0, 0);
pub const HV_SYS_REG_TTBR0_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 2, 0, 0);
pub const HV_SYS_REG_TTBR1_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 2, 0, 1);
pub const HV_SYS_REG_TCR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 2, 0, 2);
pub const HV_SYS_REG_MAIR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 10, 2, 0);
pub const HV_SYS_REG_VBAR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 12, 0, 0);
pub const HV_SYS_REG_ESR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 5, 2, 0);
pub const HV_SYS_REG_FAR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 6, 0, 0);
pub const HV_SYS_REG_ELR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 4, 0, 1);
pub const HV_SYS_REG_SPSR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 4, 0, 0);
pub const HV_SYS_REG_SP_EL0: hv_sys_reg_t = encode_sys_reg(3, 0, 4, 1, 0);
pub const HV_SYS_REG_SP_EL1: hv_sys_reg_t = encode_sys_reg(3, 4, 4, 1, 0);
pub const HV_SYS_REG_TPIDR_EL0: hv_sys_reg_t = encode_sys_reg(3, 3, 13, 0, 2);
pub const HV_SYS_REG_TPIDR_EL1: hv_sys_reg_t = encode_sys_reg(3, 0, 13, 0, 4);
pub const HV_SYS_REG_TPIDRRO_EL0: hv_sys_reg_t = encode_sys_reg(3, 3, 13, 0, 3);

// Timer registers
pub const HV_SYS_REG_CNTP_CTL_EL0: hv_sys_reg_t = encode_sys_reg(3, 3, 14, 2, 1);
pub const HV_SYS_REG_CNTP_CVAL_EL0: hv_sys_reg_t = encode_sys_reg(3, 3, 14, 2, 2);
pub const HV_SYS_REG_CNTV_CTL_EL0: hv_sys_reg_t = encode_sys_reg(3, 3, 14, 3, 1);
pub const HV_SYS_REG_CNTV_CVAL_EL0: hv_sys_reg_t = encode_sys_reg(3, 3, 14, 3, 2);

// ARM64 exit reasons
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hv_exit_reason_t {
    HV_EXIT_REASON_CANCELED = 0,
    HV_EXIT_REASON_EXCEPTION = 1,
    HV_EXIT_REASON_VTIMER_ACTIVATED = 2,
    HV_EXIT_REASON_UNKNOWN = 3,
}

// Exception class (EC) values from ESR_EL2
pub const EC_UNKNOWN: u32 = 0b000000;
pub const EC_WFI_WFE: u32 = 0b000001;
pub const EC_MCR_MRC_CP15: u32 = 0b000011;
pub const EC_MCRR_MRRC_CP15: u32 = 0b000100;
pub const EC_MCR_MRC_CP14: u32 = 0b000101;
pub const EC_LDC_STC_CP14: u32 = 0b000110;
pub const EC_SIMD_FP: u32 = 0b000111;
pub const EC_VMRS: u32 = 0b001000;
pub const EC_PAUTH: u32 = 0b001001;
pub const EC_LDST64B: u32 = 0b001010;
pub const EC_MRRC_CP14: u32 = 0b001100;
pub const EC_BTI: u32 = 0b001101;
pub const EC_ILLEGAL: u32 = 0b001110;
pub const EC_SVC32: u32 = 0b010001;
pub const EC_HVC32: u32 = 0b010010;
pub const EC_SMC32: u32 = 0b010011;
pub const EC_SVC64: u32 = 0b010101;
pub const EC_HVC64: u32 = 0b010110;
pub const EC_SMC64: u32 = 0b010111;
pub const EC_MSR_MRS: u32 = 0b011000;
pub const EC_SVE: u32 = 0b011001;
pub const EC_ERET: u32 = 0b011010;
pub const EC_FPAC: u32 = 0b011100;
pub const EC_SME: u32 = 0b011101;
pub const EC_GRANULE_PROTECTION_CHECK: u32 = 0b011110;
pub const EC_IMPL_DEF: u32 = 0b011111;
pub const EC_INST_ABORT_LOWER: u32 = 0b100000;
pub const EC_INST_ABORT_CURR: u32 = 0b100001;
pub const EC_PC_ALIGN: u32 = 0b100010;
pub const EC_DATA_ABORT_LOWER: u32 = 0b100100;
pub const EC_DATA_ABORT_CURR: u32 = 0b100101;
pub const EC_SP_ALIGN: u32 = 0b100110;
pub const EC_MOP: u32 = 0b100111;
pub const EC_FP_EXC32: u32 = 0b101000;
pub const EC_FP_EXC64: u32 = 0b101100;
pub const EC_SERROR: u32 = 0b101111;
pub const EC_BREAKPOINT_LOWER: u32 = 0b110000;
pub const EC_BREAKPOINT_CURR: u32 = 0b110001;
pub const EC_STEP_LOWER: u32 = 0b110010;
pub const EC_STEP_CURR: u32 = 0b110011;
pub const EC_WATCHPOINT_LOWER: u32 = 0b110100;
pub const EC_WATCHPOINT_CURR: u32 = 0b110101;
pub const EC_BKPT32: u32 = 0b111000;
pub const EC_VECTOR_CATCH32: u32 = 0b111010;
pub const EC_BRK64: u32 = 0b111100;

/// vCPU exit information structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct hv_vcpu_exit {
    pub reason: hv_exit_reason_t,
    pub exception: hv_vcpu_exit_exception,
}

/// Exception details from vCPU exit
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hv_vcpu_exit_exception {
    /// Syndrome register value (ESR_EL2)
    pub syndrome: u64,
    /// Virtual address that caused the exception
    pub virtual_address: u64,
    /// Physical address (for data/instruction aborts)
    pub physical_address: u64,
}

impl hv_vcpu_exit_exception {
    /// Extract exception class (EC) from syndrome
    pub fn exception_class(&self) -> u32 {
        ((self.syndrome >> 26) & 0x3F) as u32
    }

    /// Extract instruction length (IL) from syndrome
    pub fn instruction_length(&self) -> u32 {
        if (self.syndrome >> 25) & 1 != 0 {
            4 // 32-bit instruction
        } else {
            2 // 16-bit instruction (Thumb)
        }
    }

    /// Extract ISS (Instruction Specific Syndrome) from syndrome
    pub fn iss(&self) -> u32 {
        (self.syndrome & 0x1FFFFFF) as u32
    }

    /// For data aborts: is this a write?
    pub fn is_write(&self) -> bool {
        (self.iss() >> 6) & 1 != 0
    }

    /// For data aborts: size of access (0=byte, 1=halfword, 2=word, 3=doubleword)
    pub fn access_size(&self) -> u32 {
        (self.iss() >> 22) & 0x3
    }

    /// For data aborts: sign extend?
    pub fn sign_extend(&self) -> bool {
        (self.iss() >> 21) & 1 != 0
    }

    /// For data aborts: register transfer (Rt)
    pub fn srt(&self) -> u32 {
        (self.iss() >> 16) & 0x1F
    }
}

/// 128-bit SIMD/FP value
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct hv_simd_fp_uchar16_t {
    pub bytes: [u8; 16],
}

impl hv_simd_fp_uchar16_t {
    pub fn as_u64_pair(&self) -> [u64; 2] {
        let low = u64::from_le_bytes(self.bytes[0..8].try_into().unwrap());
        let high = u64::from_le_bytes(self.bytes[8..16].try_into().unwrap());
        [low, high]
    }

    pub fn from_u64_pair(pair: [u64; 2]) -> Self {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&pair[0].to_le_bytes());
        bytes[8..16].copy_from_slice(&pair[1].to_le_bytes());
        Self { bytes }
    }
}

// vCPU configuration options
pub const HV_VCPU_CONFIG_DEFAULT: u64 = 0;

// External function declarations from Hypervisor.framework (ARM64)
#[cfg(target_arch = "aarch64")]
#[link(name = "Hypervisor", kind = "framework")]
unsafe extern "C" {
    /// Create a VM instance for ARM64.
    pub fn hv_vm_create(config: *const c_void) -> hv_return_t;

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
    pub fn hv_vcpu_create(
        vcpu: *mut hv_vcpu_t,
        exit: *mut hv_vcpu_exit_t,
        config: *const c_void,
    ) -> hv_return_t;

    /// Destroy a vCPU.
    pub fn hv_vcpu_destroy(vcpu: hv_vcpu_t) -> hv_return_t;

    /// Run a vCPU until exit.
    pub fn hv_vcpu_run(vcpu: hv_vcpu_t) -> hv_return_t;

    /// Force an immediate exit of a set of vCPUs (callable from any thread).
    pub fn hv_vcpus_exit(vcpus: *mut hv_vcpu_t, vcpu_count: u32) -> hv_return_t;

    /// Get a general-purpose register.
    pub fn hv_vcpu_get_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: *mut u64) -> hv_return_t;

    /// Set a general-purpose register.
    pub fn hv_vcpu_set_reg(vcpu: hv_vcpu_t, reg: hv_reg_t, value: u64) -> hv_return_t;

    /// Get a SIMD/FP register.
    pub fn hv_vcpu_get_simd_fp_reg(
        vcpu: hv_vcpu_t,
        reg: hv_simd_fp_reg_t,
        value: *mut hv_simd_fp_uchar16_t,
    ) -> hv_return_t;

    /// Set a SIMD/FP register.
    pub fn hv_vcpu_set_simd_fp_reg(
        vcpu: hv_vcpu_t,
        reg: hv_simd_fp_reg_t,
        value: hv_simd_fp_uchar16_t,
    ) -> hv_return_t;

    /// Get a system register.
    pub fn hv_vcpu_get_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: *mut u64) -> hv_return_t;

    /// Set a system register.
    pub fn hv_vcpu_set_sys_reg(vcpu: hv_vcpu_t, reg: hv_sys_reg_t, value: u64) -> hv_return_t;

    /// Get pending interrupts.
    pub fn hv_vcpu_get_pending_interrupt(
        vcpu: hv_vcpu_t,
        interrupt_type: u32,
        pending: *mut bool,
    ) -> hv_return_t;

    /// Set pending interrupt.
    pub fn hv_vcpu_set_pending_interrupt(
        vcpu: hv_vcpu_t,
        interrupt_type: u32,
        pending: bool,
    ) -> hv_return_t;

    /// Get VTimer mask.
    pub fn hv_vcpu_get_vtimer_mask(vcpu: hv_vcpu_t, masked: *mut bool) -> hv_return_t;

    /// Set VTimer mask.
    pub fn hv_vcpu_set_vtimer_mask(vcpu: hv_vcpu_t, masked: bool) -> hv_return_t;

    /// Get VTimer offset.
    pub fn hv_vcpu_get_vtimer_offset(vcpu: hv_vcpu_t, offset: *mut u64) -> hv_return_t;

    /// Set VTimer offset.
    pub fn hv_vcpu_set_vtimer_offset(vcpu: hv_vcpu_t, offset: u64) -> hv_return_t;
}

// Interrupt types for ARM64
pub const HV_INTERRUPT_TYPE_IRQ: u32 = 0;
pub const HV_INTERRUPT_TYPE_FIQ: u32 = 1;

// =============================================================================
// In-kernel GICv3 (Hypervisor.framework, macOS 15+)
// =============================================================================

/// Opaque GIC configuration object (`hv_gic_config_t`, an os_object).
pub type hv_gic_config_t = *mut c_void;

/// `hv_gic_redistributor_reg_t` value for GICR_ISPENDR0 (SGI/PPI pending set).
pub const HV_GIC_REDISTRIBUTOR_REG_GICR_ISPENDR0: u32 = 0x10200;

/// GIC INTID of the EL1 virtual timer PPI (delivered on
/// HV_EXIT_REASON_VTIMER_ACTIVATED).
pub const HV_GIC_INT_EL1_VIRTUAL_TIMER: u32 = 27;

#[cfg(target_arch = "aarch64")]
#[link(name = "Hypervisor", kind = "framework")]
unsafe extern "C" {
    /// Create a GIC configuration object (caller owns the returned os_object).
    pub fn hv_gic_config_create() -> hv_gic_config_t;

    /// Set the guest physical address of the GICv3 distributor.
    pub fn hv_gic_config_set_distributor_base(
        config: hv_gic_config_t,
        base: hv_gpaddr_t,
    ) -> hv_return_t;

    /// Set the guest physical address of the GICv3 redistributor region.
    pub fn hv_gic_config_set_redistributor_base(
        config: hv_gic_config_t,
        base: hv_gpaddr_t,
    ) -> hv_return_t;

    /// Create the in-kernel GICv3 for the current VM (before vCPU creation).
    pub fn hv_gic_create(config: hv_gic_config_t) -> hv_return_t;

    /// Assert or deassert an SPI line (intid >= 32).
    pub fn hv_gic_set_spi(intid: u32, level: bool) -> hv_return_t;

    /// Size of the distributor MMIO region.
    pub fn hv_gic_get_distributor_size(size: *mut usize) -> hv_return_t;

    /// Size of one redistributor (per vCPU).
    pub fn hv_gic_get_redistributor_size(size: *mut usize) -> hv_return_t;

    /// Total size of the redistributor region (all vCPUs).
    pub fn hv_gic_get_redistributor_region_size(size: *mut usize) -> hv_return_t;

    /// Write a redistributor register for a vCPU (reg is hv_gic_redistributor_reg_t).
    pub fn hv_gic_set_redistributor_reg(vcpu: hv_vcpu_t, reg: u32, value: u64) -> hv_return_t;

    /// Range of SPI INTIDs supported by the GIC.
    pub fn hv_gic_get_spi_interrupt_range(base: *mut u32, count: *mut u32) -> hv_return_t;

    /// Required alignment for the distributor base.
    pub fn hv_gic_get_distributor_base_alignment(alignment: *mut usize) -> hv_return_t;

    /// Required alignment for the redistributor region base.
    pub fn hv_gic_get_redistributor_base_alignment(alignment: *mut usize) -> hv_return_t;

    /// Actual redistributor base for a vCPU.
    pub fn hv_gic_get_redistributor_base(vcpu: hv_vcpu_t, base: *mut hv_gpaddr_t) -> hv_return_t;
}

/// Check if ARM64 Hypervisor.framework is available
#[cfg(target_arch = "aarch64")]
pub fn hv_arm64_check_available() -> Result<(), &'static str> {
    use std::ptr;

    let ret = unsafe { hv_vm_create(ptr::null()) };
    if ret == HV_SUCCESS {
        unsafe { hv_vm_destroy() };
        Ok(())
    } else if ret == HV_DENIED {
        Err("Hypervisor access denied. Add com.apple.security.hypervisor entitlement.")
    } else if ret == HV_NO_DEVICE {
        Err("Hypervisor not available on this system.")
    } else if ret == HV_UNSUPPORTED {
        Err("ARM64 Hypervisor not supported on this system.")
    } else {
        Err("Failed to initialize ARM64 Hypervisor.framework")
    }
}

#[cfg(not(target_arch = "aarch64"))]
pub fn hv_arm64_check_available() -> Result<(), &'static str> {
    Err("ARM64 Hypervisor requires Apple Silicon")
}
