//! Software CPU emulator backend.
//!
//! This module provides a software-based x86_64 CPU emulator for cross-platform support.

pub mod aarch64;
pub mod armv6;
pub mod hexagon;
pub mod riscv;
pub mod s5l8900;
pub mod x86_64;

use std::any::Any;
use std::sync::Arc;

use vm_memory::GuestMemoryMmap;

use crate::config::{ArchKind, Endianness, HexagonIsa};
use crate::cpu::VCpu;
use crate::error::{Error, Result};

use super::{Backend, Vm};

/// Software emulator backend.
pub struct EmulatorBackend {
    arch: ArchKind,
    hexagon_isa: HexagonIsa,
    hexagon_endian: Endianness,
}

impl EmulatorBackend {
    pub fn new(arch: ArchKind, hexagon_isa: HexagonIsa, hexagon_endian: Endianness) -> Self {
        EmulatorBackend {
            arch,
            hexagon_isa,
            hexagon_endian,
        }
    }
}

impl Backend for EmulatorBackend {
    fn name(&self) -> &'static str {
        "emulator"
    }

    fn create_vm(&self) -> Result<Box<dyn Vm>> {
        Ok(Box::new(EmulatorVm::new(
            self.arch,
            self.hexagon_isa,
            self.hexagon_endian,
        )))
    }
}

/// Emulated VM instance.
pub struct EmulatorVm {
    irq_pending: std::sync::Mutex<Vec<u32>>,
    arch: ArchKind,
    hexagon_isa: HexagonIsa,
    hexagon_endian: Endianness,
}

impl EmulatorVm {
    pub fn new(arch: ArchKind, hexagon_isa: HexagonIsa, hexagon_endian: Endianness) -> Self {
        EmulatorVm {
            irq_pending: std::sync::Mutex::new(Vec::new()),
            arch,
            hexagon_isa,
            hexagon_endian,
        }
    }
}

impl Vm for EmulatorVm {
    fn create_vcpu(&self, id: u32, mem: Arc<GuestMemoryMmap>) -> Result<Box<dyn VCpu>> {
        match self.arch {
            ArchKind::X86_64 => Ok(Box::new(x86_64::X86_64Vcpu::new(id, mem))),
            ArchKind::Hexagon => Ok(Box::new(hexagon::HexagonVcpu::new(
                id,
                mem,
                self.hexagon_isa,
                self.hexagon_endian,
            ))),
            ArchKind::Riscv64 => Ok(Box::new(riscv::RiscVVcpu::new(id, mem))),
            ArchKind::Aarch64 => Ok(Box::new(aarch64::Aarch64Vcpu::new(id, mem))),
            ArchKind::Armv7a => {
                if std::env::var("RAX_MACHINE").as_deref() == Ok("s5l8900") {
                    Ok(Box::new(s5l8900::S5L8900Vcpu::new(id, mem)))
                } else {
                    Ok(Box::new(armv6::Armv6Vcpu::new(id, mem)))
                }
            }
            _ => Err(Error::Emulator(format!(
                "Unsupported architecture: {:?}",
                self.arch
            ))),
        }
    }

    fn set_irq_line(&self, irq: u32, level: bool) -> Result<()> {
        if level {
            let mut pending = self.irq_pending.lock().unwrap();
            if !pending.contains(&irq) {
                pending.push(irq);
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
