//! Apple Hypervisor.framework backend implementation.
//!
//! This backend uses Apple's Hypervisor.framework to provide hardware-accelerated
//! virtualization on macOS:
//! - On Intel Macs: Native x86_64 virtualization
//! - On Apple Silicon: Native ARM64 virtualization for AArch64 guests
//!
//! Note: x86_64 guests on Apple Silicon are NOT supported by HVF.
//! Apple's Hypervisor.framework only supports guests matching the host architecture.
//! For x86_64 emulation on Apple Silicon, use the emulator backend.

mod bindings;
mod convert;
pub mod rosetta;

// ARM64 HVF backend (Apple Silicon only)
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub mod arm64;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod arm64_bindings;

// Re-export ARM64 backend types on Apple Silicon
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub use arm64::{HvfArm64Backend, HvfArm64Vcpu, HvfArm64Vm};

use std::any::Any;
use std::sync::{Arc, Mutex};

use tracing::{debug, info, warn};
use vm_memory::{Address, GuestAddress, GuestMemory, GuestMemoryMmap, GuestMemoryRegion};

use crate::cpu::{CpuState, VCpu, VcpuExit, X86_64CpuState};
use crate::error::{Error, Result};
use crate::memory::GuestMemoryWrapper;

use super::{Backend, Vm};
use bindings::*;
use convert::*;
use rosetta::CpuidConfig;

/// HVF backend.
pub struct HvfBackend {
    /// CPUID configuration for x86_64 guests
    cpuid_config: CpuidConfig,
}

impl HvfBackend {
    pub fn new() -> Result<Self> {
        // This backend only works on Intel Macs - verified at compile time via cfg
        // Check if Hypervisor.framework is available and we have entitlements
        if let Err(msg) = bindings::hv_check_available() {
            return Err(Error::InvalidConfig(msg.to_string()));
        }

        info!("HVF backend initialized for native x86_64 virtualization (Intel Mac)");

        Ok(HvfBackend {
            cpuid_config: CpuidConfig::default(),
        })
    }
}

impl Backend for HvfBackend {
    fn name(&self) -> &'static str {
        "hvf"
    }

    fn create_vm(&self) -> Result<Box<dyn Vm>> {
        // Create VM
        let flags = rosetta::get_vm_creation_flags();
        let ret = unsafe { hv_vm_create(flags) };
        if ret != HV_SUCCESS {
            return Err(Error::Emulator(format!(
                "Failed to create VM: {}",
                hv_error_string(ret)
            )));
        }

        info!("Created HVF VM");

        Ok(Box::new(HvfVm {
            cpuid_config: self.cpuid_config.clone(),
            pending_irqs: Mutex::new(Vec::new()),
            memory_mapped: Mutex::new(false),
        }))
    }
}

impl Drop for HvfBackend {
    fn drop(&mut self) {
        // VM is cleaned up in HvfVm::drop
    }
}

/// HVF VM instance.
pub struct HvfVm {
    /// CPUID configuration
    cpuid_config: CpuidConfig,
    /// Pending IRQs to inject
    pending_irqs: Mutex<Vec<u32>>,
    /// Whether memory has been mapped
    memory_mapped: Mutex<bool>,
}

impl HvfVm {
    /// Register guest memory with the VM.
    pub fn register_memory(&self, mem: &GuestMemoryWrapper) -> Result<()> {
        let mut mapped = self.memory_mapped.lock().unwrap();
        if *mapped {
            return Ok(());
        }

        for region in mem.memory().iter() {
            let guest_addr = region.start_addr().0;
            let size = region.len() as usize;
            let host_addr = region.as_ptr() as *mut std::ffi::c_void;

            debug!(
                guest_addr = format!("{:#x}", guest_addr),
                size = size,
                "Mapping guest memory region"
            );

            let flags = HV_MEMORY_READ | HV_MEMORY_WRITE | HV_MEMORY_EXEC;
            let ret = unsafe { hv_vm_map(host_addr, guest_addr, size, flags) };
            if ret != HV_SUCCESS {
                return Err(Error::Emulator(format!(
                    "Failed to map memory at {:#x}: {}",
                    guest_addr,
                    hv_error_string(ret)
                )));
            }
        }

        *mapped = true;
        info!("Mapped guest memory to HVF VM");
        Ok(())
    }
}

impl Vm for HvfVm {
    fn create_vcpu(&self, id: u32, mem: Arc<GuestMemoryMmap>) -> Result<Box<dyn VCpu>> {
        // Create vCPU
        let mut vcpu_id: hv_vcpuid_t = 0;
        let ret = unsafe { hv_vcpu_create(&mut vcpu_id, 0) };
        if ret != HV_SUCCESS {
            return Err(Error::Emulator(format!(
                "Failed to create vCPU {}: {}",
                id,
                hv_error_string(ret)
            )));
        }

        debug!(id, vcpu_id, "Created HVF vCPU");

        // Initialize vCPU state
        let vcpu = HvfVcpu::new(vcpu_id, id, mem, self.cpuid_config.clone())?;

        Ok(Box::new(vcpu))
    }

    fn set_irq_line(&self, irq: u32, level: bool) -> Result<()> {
        if level {
            let mut irqs = self.pending_irqs.lock().unwrap();
            if !irqs.contains(&irq) {
                irqs.push(irq);
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Drop for HvfVm {
    fn drop(&mut self) {
        let ret = unsafe { hv_vm_destroy() };
        if ret != HV_SUCCESS {
            warn!("Failed to destroy VM: {}", hv_error_string(ret));
        }
    }
}

/// HVF vCPU.
pub struct HvfVcpu {
    /// HVF vCPU ID
    vcpu_id: hv_vcpuid_t,
    /// Our vCPU ID (index)
    id: u32,
    /// Guest memory reference
    mem: Arc<GuestMemoryMmap>,
    /// CPUID configuration
    cpuid_config: CpuidConfig,
    /// Pending I/O in data pointer (port, size)
    pending_io_in: Option<(u16, u8)>,
    /// Whether the vCPU is halted
    halted: bool,
    /// Pending interrupt to inject
    pending_interrupt: Option<u8>,
}

// Safety: HvfVcpu is Send because the vCPU ID is just a u64 handle,
// and we synchronize access appropriately.
unsafe impl Send for HvfVcpu {}

impl HvfVcpu {
    fn new(
        vcpu_id: hv_vcpuid_t,
        id: u32,
        mem: Arc<GuestMemoryMmap>,
        cpuid_config: CpuidConfig,
    ) -> Result<Self> {
        let vcpu = HvfVcpu {
            vcpu_id,
            id,
            mem,
            cpuid_config,
            pending_io_in: None,
            halted: false,
            pending_interrupt: None,
        };

        // Initialize VMCS
        vcpu.init_vmcs()?;

        Ok(vcpu)
    }

    /// Initialize VMCS fields for the vCPU.
    fn init_vmcs(&self) -> Result<()> {
        use hv_vmx_vmcs_field_t::*;

        // Get VMX capabilities to determine which controls we can enable
        let mut pin_cap: u64 = 0;
        let mut proc_cap: u64 = 0;
        let mut proc2_cap: u64 = 0;
        let mut entry_cap: u64 = 0;
        let mut exit_cap: u64 = 0;

        unsafe {
            hv_vmx_get_capability(HV_VMX_CAP_PINBASED, &mut pin_cap);
            hv_vmx_get_capability(HV_VMX_CAP_PROCBASED, &mut proc_cap);
            hv_vmx_get_capability(HV_VMX_CAP_PROCBASED2, &mut proc2_cap);
            hv_vmx_get_capability(HV_VMX_CAP_ENTRY, &mut entry_cap);
            hv_vmx_get_capability(HV_VMX_CAP_EXIT, &mut exit_cap);
        }

        // Pin-based controls: enable external interrupt exiting
        let pin_based = self.cap2ctrl(pin_cap, PIN_BASED_EXT_INTR_EXIT | PIN_BASED_NMI_EXIT);
        vmcs_write(self.vcpu_id, VMCS_CTRL_PIN_BASED, pin_based)?;

        // Primary processor-based controls
        let proc_based = self.cap2ctrl(
            proc_cap,
            CPU_BASED_HLT_EXIT
                | CPU_BASED_CR8_LOAD_EXIT
                | CPU_BASED_CR8_STORE_EXIT
                | CPU_BASED_UNCOND_IO_EXIT
                | CPU_BASED_SECONDARY_CONTROLS,
        );
        vmcs_write(self.vcpu_id, VMCS_CTRL_CPU_BASED, proc_based)?;

        // Secondary processor-based controls: enable EPT, unrestricted guest
        let proc2_based = self.cap2ctrl(
            proc2_cap,
            CPU_BASED2_EPT | CPU_BASED2_UNRESTRICTED_GUEST | CPU_BASED2_RDTSCP,
        );
        vmcs_write(self.vcpu_id, VMCS_CTRL_CPU_BASED2, proc2_based)?;

        // VM-entry controls: load EFER, IA-32e guest mode
        let entry_ctls = self.cap2ctrl(entry_cap, VMENTRY_LOAD_IA32_EFER | VMENTRY_GUEST_IA32E);
        vmcs_write(self.vcpu_id, VMCS_CTRL_VMENTRY_CONTROLS, entry_ctls)?;

        // Exception bitmap: trap no exceptions initially
        vmcs_write(self.vcpu_id, VMCS_CTRL_EXC_BITMAP, 0)?;

        // CR0/CR4 masks and shadows - allow guest to modify most bits
        vmcs_write(self.vcpu_id, VMCS_CTRL_CR0_MASK, CR0_NE)?;
        vmcs_write(self.vcpu_id, VMCS_CTRL_CR0_SHADOW, CR0_NE)?;
        vmcs_write(self.vcpu_id, VMCS_CTRL_CR4_MASK, 0)?;
        vmcs_write(self.vcpu_id, VMCS_CTRL_CR4_SHADOW, 0)?;

        // Guest link pointer (for nested virtualization, set to -1 for none)
        vmcs_write(self.vcpu_id, VMCS_GUEST_LINK_POINTER, 0xFFFFFFFF_FFFFFFFF)?;

        debug!(vcpu_id = self.vcpu_id, "Initialized VMCS");
        Ok(())
    }

    /// Convert capability to control value.
    /// The capability encodes allowed 0->1 and 1->0 transitions.
    fn cap2ctrl(&self, cap: u64, ctrl: u64) -> u64 {
        let allowed_0 = cap & 0xFFFFFFFF; // Bits that may be 0
        let allowed_1 = (cap >> 32) & 0xFFFFFFFF; // Bits that may be 1
        (ctrl | allowed_0) & allowed_1
    }

    /// Handle CPUID exit.
    fn handle_cpuid(&self) -> Result<()> {
        let rax = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RAX)?;
        let rcx = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RCX)?;
        let leaf = rax as u32;
        let subleaf = rcx as u32;

        let (eax, ebx, ecx, edx) = self.cpuid_config.cpuid(leaf, subleaf);

        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RAX, eax as u64)?;
        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RBX, ebx as u64)?;
        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RCX, ecx as u64)?;
        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RDX, edx as u64)?;

        // Advance RIP past the CPUID instruction (2 bytes)
        self.advance_rip(2)?;

        Ok(())
    }

    /// Handle RDMSR exit.
    fn handle_rdmsr(&self) -> Result<Option<VcpuExit>> {
        let ecx = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RCX)? as u32;

        // Handle common MSRs
        let value = match ecx {
            0x1B => 0xFEE00000 | 0x800, // IA32_APIC_BASE - LAPIC at default address, enabled
            0xC0000080 => {
                // IA32_EFER - read from VMCS
                vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_EFER)?
            }
            0xC0000081 => 0, // IA32_STAR
            0xC0000082 => 0, // IA32_LSTAR
            0xC0000083 => 0, // IA32_CSTAR
            0xC0000084 => 0, // IA32_FMASK
            0xC0000100 => {
                // IA32_FS_BASE - read from VMCS
                vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_GUEST_FS_BASE)?
            }
            0xC0000101 => {
                // IA32_GS_BASE - read from VMCS
                vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_GUEST_GS_BASE)?
            }
            0xC0000102 => 0, // IA32_KERNEL_GS_BASE
            0x174 => {
                // IA32_SYSENTER_CS
                vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_SYSENTER_CS,
                )?
            }
            0x175 => {
                // IA32_SYSENTER_ESP
                vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_SYSENTER_ESP,
                )?
            }
            0x176 => {
                // IA32_SYSENTER_EIP
                vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_SYSENTER_EIP,
                )?
            }
            0x1A0 => rosetta::MsrDefaults::default().misc_enable, // IA32_MISC_ENABLE
            0x277 => rosetta::MsrDefaults::default().pat,         // IA32_PAT
            0x10 => 0, // IA32_TIME_STAMP_COUNTER - return 0, will be handled by TSC offsetting
            0x6E0 => 0, // IA32_TSC_DEADLINE
            _ => {
                debug!(msr = format!("{:#x}", ecx), "Unhandled RDMSR");
                0 // Return 0 for unknown MSRs
            }
        };

        // Set result in EDX:EAX
        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RAX, value & 0xFFFFFFFF)?;
        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RDX, value >> 32)?;

        // Advance RIP past the RDMSR instruction (2 bytes)
        self.advance_rip(2)?;

        Ok(None)
    }

    /// Handle WRMSR exit.
    fn handle_wrmsr(&self) -> Result<Option<VcpuExit>> {
        let ecx = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RCX)? as u32;
        let eax = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RAX)? as u32;
        let edx = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RDX)? as u32;
        let value = ((edx as u64) << 32) | (eax as u64);

        match ecx {
            0x1B => {
                // IA32_APIC_BASE - ignore for now
                debug!(value = format!("{:#x}", value), "WRMSR IA32_APIC_BASE");
            }
            0xC0000080 => {
                // IA32_EFER
                vmcs_write(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_EFER,
                    value,
                )?;
            }
            0xC0000081 => {
                // IA32_STAR - store somewhere or handle syscall
                debug!(value = format!("{:#x}", value), "WRMSR IA32_STAR");
            }
            0xC0000082 => {
                // IA32_LSTAR
                debug!(value = format!("{:#x}", value), "WRMSR IA32_LSTAR");
            }
            0xC0000083 => {
                // IA32_CSTAR
                debug!(value = format!("{:#x}", value), "WRMSR IA32_CSTAR");
            }
            0xC0000084 => {
                // IA32_FMASK
                debug!(value = format!("{:#x}", value), "WRMSR IA32_FMASK");
            }
            0xC0000100 => {
                // IA32_FS_BASE
                vmcs_write(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_GUEST_FS_BASE, value)?;
            }
            0xC0000101 => {
                // IA32_GS_BASE
                vmcs_write(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_GUEST_GS_BASE, value)?;
            }
            0xC0000102 => {
                // IA32_KERNEL_GS_BASE
                debug!(value = format!("{:#x}", value), "WRMSR IA32_KERNEL_GS_BASE");
            }
            0x174 => {
                // IA32_SYSENTER_CS
                vmcs_write(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_SYSENTER_CS,
                    value,
                )?;
            }
            0x175 => {
                // IA32_SYSENTER_ESP
                vmcs_write(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_SYSENTER_ESP,
                    value,
                )?;
            }
            0x176 => {
                // IA32_SYSENTER_EIP
                vmcs_write(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_IA32_SYSENTER_EIP,
                    value,
                )?;
            }
            0x6E0 => {
                // IA32_TSC_DEADLINE
                debug!(value = format!("{:#x}", value), "WRMSR IA32_TSC_DEADLINE");
            }
            _ => {
                debug!(
                    msr = format!("{:#x}", ecx),
                    value = format!("{:#x}", value),
                    "Unhandled WRMSR"
                );
            }
        }

        // Advance RIP past the WRMSR instruction (2 bytes)
        self.advance_rip(2)?;

        Ok(None)
    }

    /// Advance RIP by the given number of bytes.
    fn advance_rip(&self, bytes: u64) -> Result<()> {
        let rip = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RIP)?;
        write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RIP, rip + bytes)
    }

    /// Inject a pending interrupt if possible.
    fn try_inject_interrupt(&mut self) -> Result<()> {
        if let Some(vector) = self.pending_interrupt.take() {
            // Check if interrupts are enabled
            let rflags = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RFLAGS)?;
            if (rflags & RFLAGS_IF) == 0 {
                // Interrupts disabled, put it back
                self.pending_interrupt = Some(vector);
                return Ok(());
            }

            // Check interruptibility state
            let interruptibility = vmcs_read(
                self.vcpu_id,
                hv_vmx_vmcs_field_t::VMCS_GUEST_INTERRUPTIBILITY_STATE,
            )?;
            if interruptibility != 0 {
                // Not interruptible (e.g., after STI or MOV SS)
                self.pending_interrupt = Some(vector);
                return Ok(());
            }

            // Inject the interrupt via VM-entry
            let intr_info = (vector as u64)
                | ((INTR_TYPE_EXT_INTR as u64) << INTR_INFO_TYPE_SHIFT)
                | (INTR_INFO_VALID as u64);
            vmcs_write(
                self.vcpu_id,
                hv_vmx_vmcs_field_t::VMCS_CTRL_VMENTRY_IRQ_INFO,
                intr_info,
            )?;

            self.halted = false;
        }
        Ok(())
    }
}

impl VCpu for HvfVcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        // Try to inject any pending interrupt
        self.try_inject_interrupt()?;

        // If halted and no interrupt pending, return Hlt
        if self.halted && self.pending_interrupt.is_none() {
            return Ok(VcpuExit::Hlt);
        }

        // Run the vCPU
        let ret = unsafe { hv_vcpu_run(self.vcpu_id) };
        if ret != HV_SUCCESS {
            return Err(Error::Emulator(format!(
                "hv_vcpu_run failed: {}",
                hv_error_string(ret)
            )));
        }

        // Get exit reason
        let exit_reason = vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_EXIT_REASON)? as u32;
        let exit_reason = exit_reason & 0xFFFF; // Lower 16 bits

        match exit_reason {
            VMX_EXIT_REASON_HLT => {
                self.halted = true;
                Ok(VcpuExit::Hlt)
            }

            VMX_EXIT_REASON_IO => {
                let qual = vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_EXIT_QUALIFICATION)?;
                let io = IoExitQualification::from_qualification(qual);
                let insn_len = vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_EXIT_INSTRUCTION_LENGTH,
                )?;

                match io.direction {
                    IoDirection::In => {
                        self.pending_io_in = Some((io.port, io.size));
                        Ok(VcpuExit::IoIn {
                            port: io.port,
                            size: io.size,
                        })
                    }
                    IoDirection::Out => {
                        // Read data from RAX
                        let rax = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RAX)?;
                        let data = match io.size {
                            1 => vec![rax as u8],
                            2 => (rax as u16).to_le_bytes().to_vec(),
                            4 => (rax as u32).to_le_bytes().to_vec(),
                            _ => vec![rax as u8],
                        };

                        // Advance RIP
                        self.advance_rip(insn_len)?;

                        Ok(VcpuExit::IoOut {
                            port: io.port,
                            data,
                        })
                    }
                }
            }

            VMX_EXIT_REASON_EPT_VIOLATION => {
                let qual = vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_EXIT_QUALIFICATION)?;
                let gpa = vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_GUEST_PHYSICAL_ADDRESS,
                )?;
                let ept = EptViolationQualification::from_qualification(qual);

                if ept.write {
                    // MMIO write
                    // We need to decode the instruction to get the write data
                    // For now, return a placeholder
                    Ok(VcpuExit::MmioWrite {
                        addr: gpa,
                        data: vec![0; 4], // TODO: decode actual data
                    })
                } else {
                    // MMIO read
                    Ok(VcpuExit::MmioRead {
                        addr: gpa,
                        size: 4, // TODO: decode actual size
                    })
                }
            }

            VMX_EXIT_REASON_CPUID => {
                self.handle_cpuid()?;
                // Re-run immediately
                self.run()
            }

            VMX_EXIT_REASON_RDMSR => {
                if let Some(exit) = self.handle_rdmsr()? {
                    Ok(exit)
                } else {
                    // Re-run immediately
                    self.run()
                }
            }

            VMX_EXIT_REASON_WRMSR => {
                if let Some(exit) = self.handle_wrmsr()? {
                    Ok(exit)
                } else {
                    // Re-run immediately
                    self.run()
                }
            }

            VMX_EXIT_REASON_TRIPLE_FAULT => Ok(VcpuExit::Shutdown),

            VMX_EXIT_REASON_EXCEPTION_NMI => {
                let intr_info =
                    vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_IDT_VECTORING_INFO)?;
                let vector = (intr_info & INTR_INFO_VECTOR_MASK as u64) as u8;
                Ok(VcpuExit::Exception(vector))
            }

            VMX_EXIT_REASON_EXT_INTERRUPT => {
                // External interrupt - just continue
                self.run()
            }

            VMX_EXIT_REASON_IRQ_WINDOW => {
                // Interrupt window opened - try to inject
                self.try_inject_interrupt()?;
                self.run()
            }

            VMX_EXIT_REASON_CR_ACCESS => {
                // Handle CR access (usually CR0/CR3/CR4 writes)
                let qual = vmcs_read(self.vcpu_id, hv_vmx_vmcs_field_t::VMCS_EXIT_QUALIFICATION)?;
                let insn_len = vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_EXIT_INSTRUCTION_LENGTH,
                )?;

                // For now, just advance RIP and continue
                // TODO: Proper CR access handling
                self.advance_rip(insn_len)?;
                self.run()
            }

            VMX_EXIT_REASON_XSETBV => {
                // Handle XSETBV (extended control register write)
                let insn_len = vmcs_read(
                    self.vcpu_id,
                    hv_vmx_vmcs_field_t::VMCS_EXIT_INSTRUCTION_LENGTH,
                )?;
                self.advance_rip(insn_len)?;
                self.run()
            }

            _ => {
                let rip = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RIP)?;
                Ok(VcpuExit::Unknown(format!(
                    "Exit reason {} at RIP {:#x}",
                    exit_reason, rip
                )))
            }
        }
    }

    fn get_state(&self) -> Result<CpuState> {
        let regs = regs_from_hvf(self.vcpu_id)?;
        let sregs = sregs_from_hvf(self.vcpu_id)?;
        Ok(CpuState::X86_64(X86_64CpuState { regs, sregs }))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::X86_64(state) => state,
            _ => {
                return Err(Error::InvalidConfig(
                    "HVF backend only supports x86_64 state".to_string(),
                ));
            }
        };

        regs_to_hvf(self.vcpu_id, &state.regs)?;
        sregs_to_hvf(self.vcpu_id, &state.sregs)?;

        Ok(())
    }

    fn complete_io_in(&mut self, data: &[u8]) {
        if let Some((port, size)) = self.pending_io_in.take() {
            // Write data to RAX
            let value = match size {
                1 => data.first().copied().unwrap_or(0) as u64,
                2 => {
                    let bytes: [u8; 2] = data[..2.min(data.len())].try_into().unwrap_or([0, 0]);
                    u16::from_le_bytes(bytes) as u64
                }
                4 => {
                    let bytes: [u8; 4] =
                        data[..4.min(data.len())].try_into().unwrap_or([0, 0, 0, 0]);
                    u32::from_le_bytes(bytes) as u64
                }
                _ => 0,
            };

            if let Err(e) = write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RAX, value) {
                warn!("Failed to write RAX for IO in: {}", e);
            }

            // Advance RIP past the IN instruction
            if let Ok(insn_len) = vmcs_read(
                self.vcpu_id,
                hv_vmx_vmcs_field_t::VMCS_EXIT_INSTRUCTION_LENGTH,
            ) {
                let _ = self.advance_rip(insn_len);
            }

            debug!(port, size, value, "Completed I/O in");
        }
    }

    fn inject_interrupt(&mut self, vector: u8) -> Result<bool> {
        if !self.can_inject_interrupt() {
            self.pending_interrupt = Some(vector);
            return Ok(false);
        }

        // Inject immediately
        let intr_info = (vector as u64)
            | ((INTR_TYPE_EXT_INTR as u64) << INTR_INFO_TYPE_SHIFT)
            | (INTR_INFO_VALID as u64);
        vmcs_write(
            self.vcpu_id,
            hv_vmx_vmcs_field_t::VMCS_CTRL_VMENTRY_IRQ_INFO,
            intr_info,
        )?;

        self.halted = false;
        Ok(true)
    }

    fn can_inject_interrupt(&self) -> bool {
        // Check IF flag
        let rflags = match read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RFLAGS) {
            Ok(f) => f,
            Err(_) => return false,
        };

        if (rflags & RFLAGS_IF) == 0 {
            return false;
        }

        // Check interruptibility state
        let interruptibility = match vmcs_read(
            self.vcpu_id,
            hv_vmx_vmcs_field_t::VMCS_GUEST_INTERRUPTIBILITY_STATE,
        ) {
            Ok(s) => s,
            Err(_) => return false,
        };

        interruptibility == 0
    }

    fn inject_nmi(&mut self) -> Result<bool> {
        let intr_info = 2u64  // NMI vector
            | ((INTR_TYPE_NMI as u64) << INTR_INFO_TYPE_SHIFT)
            | (INTR_INFO_VALID as u64);
        vmcs_write(
            self.vcpu_id,
            hv_vmx_vmcs_field_t::VMCS_CTRL_VMENTRY_IRQ_INFO,
            intr_info,
        )?;

        self.halted = false;
        Ok(true)
    }

    #[cfg(feature = "debug")]
    fn set_single_step(&mut self, enabled: bool) {
        // Enable trap flag for single-stepping
        if let Ok(rflags) = read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RFLAGS) {
            let new_rflags = if enabled {
                rflags | RFLAGS_TF
            } else {
                rflags & !RFLAGS_TF
            };
            let _ = write_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RFLAGS, new_rflags);
        }
    }

    #[cfg(feature = "debug")]
    fn is_single_step(&self) -> bool {
        read_register(self.vcpu_id, hv_x86_reg_t::HV_X86_RFLAGS)
            .map(|f| (f & RFLAGS_TF) != 0)
            .unwrap_or(false)
    }

    #[cfg(feature = "debug")]
    fn invalidate_code_cache(&mut self, _addr: u64) {
        // HVF doesn't have a software decode cache to invalidate
        // Just invalidate TLB to be safe
        unsafe {
            hv_vcpu_invalidate_tlb(self.vcpu_id);
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn instruction_count(&self) -> u64 {
        // HVF doesn't provide instruction count, return 0
        0
    }
}

impl Drop for HvfVcpu {
    fn drop(&mut self) {
        let ret = unsafe { hv_vcpu_destroy(self.vcpu_id) };
        if ret != HV_SUCCESS {
            warn!(
                "Failed to destroy vCPU {}: {}",
                self.id,
                hv_error_string(ret)
            );
        }
    }
}
