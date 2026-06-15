//! RISC-V (RV64) architecture integration: image loading and boot state.
//!
//! This wires the self-contained [`crate::riscv`] interpreter into the VMM as a
//! bootable architecture. It loads a flat binary or an ELF image into guest
//! memory and produces the initial register file (entry PC, stack pointer), and
//! exposes a 16550 UART at the RISC-V "virt" MMIO address for console output.

use std::fs::File;
use std::io::Read;

use goblin::elf::Elf;
use vm_memory::{Address, Bytes, GuestAddress, GuestMemory, GuestMemoryMmap};

use crate::arch::{Arch, BootInfo, RiscVBootInfo};
use crate::config::VmConfig;
use crate::cpu::{CpuState, RiscVRegisters};
use crate::devices::bus::{IoBus, MmioBus};
use crate::error::{Error, Result};

/// 16550 UART MMIO base (matches the RISC-V "virt" machine convention).
const RISCV_UART_BASE: u64 = 0x1000_0000;
/// `EM_RISCV` machine type.
const EM_RISCV: u16 = 243;

pub struct Riscv64Arch;

impl Riscv64Arch {
    pub fn new() -> Self {
        Riscv64Arch
    }

    fn load_raw(mem: &GuestMemoryMmap, buf: &[u8]) -> Result<RiscVBootInfo> {
        mem.write_slice(buf, GuestAddress(0))?;
        Ok(RiscVBootInfo {
            entry_point: 0,
            load_addr: 0,
            image_size: buf.len() as u64,
        })
    }

    fn load_elf(mem: &GuestMemoryMmap, buf: &[u8]) -> Result<RiscVBootInfo> {
        let elf =
            Elf::parse(buf).map_err(|e| Error::KernelLoad(format!("ELF parse error: {e}")))?;
        if !elf.is_64 {
            return Err(Error::KernelLoad("RISC-V ELF must be 64-bit".to_string()));
        }
        if elf.header.e_machine != EM_RISCV {
            return Err(Error::KernelLoad(format!(
                "not a RISC-V ELF (e_machine={})",
                elf.header.e_machine
            )));
        }

        let mut min_addr = u64::MAX;
        let mut max_addr = 0u64;
        for ph in &elf.program_headers {
            if ph.p_type != goblin::elf::program_header::PT_LOAD {
                continue;
            }
            let file_start = ph.p_offset as usize;
            let file_end = file_start
                .checked_add(ph.p_filesz as usize)
                .ok_or_else(|| Error::KernelLoad("ELF segment overflow".to_string()))?;
            if file_end > buf.len() {
                return Err(Error::KernelLoad("ELF segment out of range".to_string()));
            }
            let load_addr = if ph.p_paddr != 0 {
                ph.p_paddr
            } else {
                ph.p_vaddr
            };
            mem.write_slice(&buf[file_start..file_end], GuestAddress(load_addr))?;
            min_addr = min_addr.min(load_addr);
            max_addr = max_addr.max(load_addr + ph.p_memsz);
        }

        Ok(RiscVBootInfo {
            entry_point: elf.entry,
            load_addr: if min_addr == u64::MAX { 0 } else { min_addr },
            image_size: max_addr.saturating_sub(min_addr),
        })
    }
}

impl Arch for Riscv64Arch {
    fn name(&self) -> &'static str {
        "riscv64"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        Some(RISCV_UART_BASE)
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        let mut file = File::open(&config.kernel)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if buf.len() < 4 {
            return Err(Error::KernelLoad("image is too small".to_string()));
        }
        let info = if buf.starts_with(b"\x7fELF") {
            Self::load_elf(mem, &buf)?
        } else {
            Self::load_raw(mem, &buf)?
        };
        Ok(BootInfo::RiscV(info))
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        Ok(())
    }

    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        let boot = boot
            .as_riscv()
            .ok_or_else(|| Error::InvalidConfig("expected riscv boot info".to_string()))?;

        let mut regs = RiscVRegisters::default();
        regs.pc = boot.entry_point;
        // Stack pointer (x2) at the top of guest RAM, 16-byte aligned.
        let mem_end = mem.last_addr().raw_value().saturating_add(1);
        regs.x[2] = mem_end.saturating_sub(16) & !0xf;
        Ok(CpuState::riscv(regs))
    }
}
