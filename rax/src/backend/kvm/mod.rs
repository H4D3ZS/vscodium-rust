//! KVM backend implementation.

mod convert;

use std::any::Any;
use std::sync::Arc;

use kvm_ioctls::{Kvm, VcpuFd, VmFd};
use vm_memory::GuestMemoryMmap;

use crate::cpu::{CpuState, VCpu, VcpuExit, X86_64CpuState};
use crate::error::{Error, Result};
use crate::memory::GuestMemoryWrapper;

use super::{Backend, Vm};

/// KVM backend.
pub struct KvmBackend {
    kvm: Kvm,
}

impl KvmBackend {
    pub fn new() -> Result<Self> {
        let kvm = Kvm::new()?;
        Ok(KvmBackend { kvm })
    }

    /// Get reference to the Kvm instance (needed for CPUID).
    pub fn kvm(&self) -> &Kvm {
        &self.kvm
    }
}

impl Backend for KvmBackend {
    fn name(&self) -> &'static str {
        "kvm"
    }

    fn create_vm(&self) -> Result<Box<dyn Vm>> {
        let vm_fd = self.kvm.create_vm()?;
        // Create a new Kvm instance for the VM (Kvm doesn't implement Clone)
        let kvm = Kvm::new()?;
        Ok(Box::new(KvmVm { vm_fd, kvm }))
    }
}

/// KVM VM instance.
pub struct KvmVm {
    vm_fd: VmFd,
    kvm: Kvm,
}

impl KvmVm {
    /// Get reference to the VmFd.
    pub fn vm_fd(&self) -> &VmFd {
        &self.vm_fd
    }

    /// Get reference to the Kvm instance.
    pub fn kvm(&self) -> &Kvm {
        &self.kvm
    }

    /// Register guest memory with the VM.
    pub fn register_memory(&self, mem: &GuestMemoryWrapper) -> Result<()> {
        mem.register(&self.vm_fd)
    }

    /// Create IRQ chip.
    pub fn create_irq_chip(&self) -> Result<()> {
        self.vm_fd.create_irq_chip()?;
        Ok(())
    }

    /// Create PIT2.
    pub fn create_pit2(&self) -> Result<()> {
        use kvm_bindings::kvm_pit_config;
        self.vm_fd.create_pit2(kvm_pit_config {
            flags: 0,
            pad: [0; 15],
        })?;
        Ok(())
    }

    /// Set TSS address.
    pub fn set_tss_address(&self, addr: u64) -> Result<()> {
        self.vm_fd.set_tss_address(addr as usize)?;
        Ok(())
    }

    /// Set identity map address.
    pub fn set_identity_map_address(&self, addr: u64) -> Result<()> {
        self.vm_fd.set_identity_map_address(addr)?;
        Ok(())
    }

    /// Create a raw VcpuFd (for setup_vcpu compatibility during transition).
    pub fn create_vcpu_fd(&self, id: u64) -> Result<VcpuFd> {
        Ok(self.vm_fd.create_vcpu(id)?)
    }
}

impl Vm for KvmVm {
    fn create_vcpu(&self, id: u32, _mem: Arc<GuestMemoryMmap>) -> Result<Box<dyn VCpu>> {
        use kvm_bindings::KVM_MAX_CPUID_ENTRIES;
        use tracing::debug;

        debug!(id, "KvmVm::create_vcpu: calling create_vcpu");
        let vcpu_fd = self.vm_fd.create_vcpu(id as u64)?;
        debug!(id, "KvmVm::create_vcpu: vcpu created, getting CPUID");

        // Set CPUID for the vCPU
        let cpuid = self.kvm.get_supported_cpuid(KVM_MAX_CPUID_ENTRIES)?;
        debug!(id, "KvmVm::create_vcpu: setting CPUID");
        vcpu_fd.set_cpuid2(&cpuid)?;
        debug!(id, "KvmVm::create_vcpu: done");

        Ok(Box::new(KvmVcpu {
            vcpu_fd,
            id,
            pending_io_in: None,
        }))
    }

    fn set_irq_line(&self, irq: u32, level: bool) -> Result<()> {
        self.vm_fd.set_irq_line(irq, level)?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// KVM vCPU.
pub struct KvmVcpu {
    vcpu_fd: VcpuFd,
    id: u32,
    /// Pointer and length to pending IoIn/MmioRead data buffer.
    /// This is set during run() when we get an IoIn/MmioRead exit,
    /// and written to in complete_io_in().
    /// Safety: The pointer is valid between run() calls as it points
    /// to the mmap'd kvm_run structure owned by vcpu_fd.
    pending_io_in: Option<(*mut u8, usize)>,
}

// Safety: KvmVcpu is Send because:
// - VcpuFd is Send
// - The raw pointer in pending_io_in points to memory owned by vcpu_fd
//   (the mmap'd kvm_run structure), so it moves with the struct
// - We only access the pointer from methods that take &mut self
unsafe impl Send for KvmVcpu {}

impl KvmVcpu {
    /// Get reference to the VcpuFd.
    pub fn vcpu_fd(&self) -> &VcpuFd {
        &self.vcpu_fd
    }
}

impl VCpu for KvmVcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        // Clear any previous pending I/O
        self.pending_io_in = None;

        match self.vcpu_fd.run()? {
            kvm_ioctls::VcpuExit::Hlt => Ok(VcpuExit::Hlt),
            kvm_ioctls::VcpuExit::Shutdown => Ok(VcpuExit::Shutdown),
            kvm_ioctls::VcpuExit::IoIn(port, data) => {
                // Store pointer to the data buffer so complete_io_in() can write to it
                self.pending_io_in = Some((data.as_mut_ptr(), data.len()));
                Ok(VcpuExit::IoIn {
                    port,
                    size: data.len() as u8,
                })
            }
            kvm_ioctls::VcpuExit::IoOut(port, data) => Ok(VcpuExit::IoOut {
                port,
                data: data.to_vec(),
            }),
            kvm_ioctls::VcpuExit::MmioRead(addr, data) => {
                // Store pointer to the data buffer so complete_io_in() can write to it
                self.pending_io_in = Some((data.as_mut_ptr(), data.len()));
                Ok(VcpuExit::MmioRead {
                    addr,
                    size: data.len() as u8,
                })
            }
            kvm_ioctls::VcpuExit::MmioWrite(addr, data) => Ok(VcpuExit::MmioWrite {
                addr,
                data: data.to_vec(),
            }),
            kvm_ioctls::VcpuExit::SystemEvent(type_, flags) => Ok(VcpuExit::SystemEvent {
                type_,
                flags: flags.first().copied().unwrap_or(0),
            }),
            kvm_ioctls::VcpuExit::FailEntry(reason, _) => Ok(VcpuExit::FailEntry { reason }),
            kvm_ioctls::VcpuExit::InternalError => Ok(VcpuExit::InternalError),
            exit => Ok(VcpuExit::Unknown(format!("{:?}", exit))),
        }
    }

    fn get_state(&self) -> Result<CpuState> {
        let kvm_regs = self.vcpu_fd.get_regs()?;
        let kvm_sregs = self.vcpu_fd.get_sregs()?;
        let regs = convert::regs_from_kvm(&kvm_regs);
        let sregs = convert::sregs_from_kvm(&kvm_sregs);
        Ok(CpuState::X86_64(X86_64CpuState { regs, sregs }))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::X86_64(state) => state,
            _ => {
                return Err(Error::InvalidConfig(
                    "KVM backend only supports x86_64 state".to_string(),
                ));
            }
        };
        let kvm_regs = convert::regs_to_kvm(&state.regs);
        let kvm_sregs = convert::sregs_to_kvm(&state.sregs);
        self.vcpu_fd.set_regs(&kvm_regs)?;
        self.vcpu_fd.set_sregs(&kvm_sregs)?;
        Ok(())
    }

    fn complete_io_in(&mut self, data: &[u8]) {
        // Write the response data to the KVM I/O buffer
        if let Some((ptr, len)) = self.pending_io_in.take() {
            let to_copy = data.len().min(len);
            // Safety: The pointer is valid as it points to the mmap'd kvm_run
            // structure which stays valid between run() calls. We only write
            // up to the buffer length that KVM told us.
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, to_copy);
            }
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
