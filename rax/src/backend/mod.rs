//! Backend abstraction layer.
//!
//! This module provides traits and factory functions for VM backends.

pub mod emulator;

#[cfg(all(feature = "kvm", target_os = "linux"))]
pub mod kvm;

// HVF backend for macOS
// - Intel Macs: x86_64 hardware virtualization
// - Apple Silicon: ARM64 hardware virtualization (for aarch64 guests)
#[cfg(all(feature = "hvf", target_os = "macos"))]
pub mod hvf;

use std::any::Any;
use std::sync::Arc;

use vm_memory::GuestMemoryMmap;

use crate::config::{ArchKind, BackendKind, VmConfig};
use crate::cpu::VCpu;
use crate::error::{Error, Result};

/// Abstract VM interface.
///
/// Represents a virtual machine that can create vCPUs and manage interrupts.
pub trait Vm: Send + Sync {
    /// Create a new vCPU.
    fn create_vcpu(&self, id: u32, mem: Arc<GuestMemoryMmap>) -> Result<Box<dyn VCpu>>;

    /// Set IRQ line level.
    fn set_irq_line(&self, irq: u32, level: bool) -> Result<()>;

    /// Get reference as Any for downcasting to concrete types.
    fn as_any(&self) -> &dyn Any;
}

/// Abstract backend interface.
///
/// Represents a virtualization backend (KVM, emulator, etc.).
pub trait Backend: Send + Sync {
    /// Backend name for logging.
    fn name(&self) -> &'static str;

    /// Create a new VM.
    fn create_vm(&self) -> Result<Box<dyn Vm>>;
}

/// Create a backend based on configuration.
pub fn create(config: &VmConfig) -> Result<Box<dyn Backend>> {
    match config.backend {
        #[cfg(all(feature = "kvm", target_os = "linux"))]
        BackendKind::Kvm => {
            if config.arch != ArchKind::X86_64 {
                return Err(Error::InvalidConfig(
                    "KVM backend only supports x86_64".to_string(),
                ));
            }
            Ok(Box::new(kvm::KvmBackend::new()?))
        }
        #[cfg(not(all(feature = "kvm", target_os = "linux")))]
        BackendKind::Kvm => Err(Error::InvalidConfig(
            "KVM backend not available (requires Linux with --features kvm)".to_string(),
        )),
        BackendKind::Emulator => Ok(Box::new(emulator::EmulatorBackend::new(
            config.arch,
            config.hexagon_isa,
            config.hexagon_endian,
        ))),
        // Intel Mac - HVF for x86_64 guests
        #[cfg(all(feature = "hvf", target_os = "macos", target_arch = "x86_64"))]
        BackendKind::Hvf => {
            if config.arch != ArchKind::X86_64 {
                return Err(Error::InvalidConfig(
                    "HVF backend on Intel Mac only supports x86_64 guests".to_string(),
                ));
            }
            Ok(Box::new(hvf::HvfBackend::new()?))
        }
        // Apple Silicon - HVF for ARM64 guests only
        #[cfg(all(feature = "hvf", target_os = "macos", target_arch = "aarch64"))]
        BackendKind::Hvf => match config.arch {
            ArchKind::Aarch64 => Ok(Box::new(hvf::arm64::HvfArm64Backend::new()?)),
            ArchKind::X86_64 => Err(Error::InvalidConfig(
                "HVF backend cannot run x86_64 guests on Apple Silicon. \
                         Apple's Hypervisor.framework only supports ARM64 guests on ARM64 hosts. \
                         Use --backend emulator for x86_64 emulation on Apple Silicon."
                    .to_string(),
            )),
            _ => Err(Error::InvalidConfig(format!(
                "HVF backend on Apple Silicon only supports aarch64 guests, not {:?}",
                config.arch
            ))),
        },
        #[cfg(not(all(feature = "hvf", target_os = "macos")))]
        BackendKind::Hvf => Err(Error::InvalidConfig(
            "HVF backend not available (requires macOS with --features hvf)".to_string(),
        )),
    }
}
