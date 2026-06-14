use std::fs::File;
use std::io::Read;

use goblin::elf::Elf;
use vm_memory::{Address, Bytes, GuestAddress, GuestMemory, GuestMemoryMmap};

use crate::arch::{Arch, BootInfo, HexagonBootInfo};
use crate::config::{Endianness, VmConfig};
use crate::cpu::{CpuState, HexagonRegisters};
use crate::devices::bus::{IoBus, MmioBus};
use crate::error::{Error, Result};

const HEXAGON_UART_BASE: u64 = 0xf000_0000;

pub struct HexagonArch;

impl HexagonArch {
    pub fn new() -> Self {
        HexagonArch
    }

    fn load_raw(mem: &GuestMemoryMmap, config: &VmConfig, buf: &[u8]) -> Result<HexagonBootInfo> {
        let load_addr = config.hexagon_load_addr.map(|addr| addr.raw()).unwrap_or(0);
        let entry = config
            .hexagon_entry
            .map(|addr| addr.raw())
            .unwrap_or(load_addr);

        mem.write_slice(buf, GuestAddress(load_addr))?;

        Ok(HexagonBootInfo {
            entry_point: entry,
            load_addr,
            image_size: buf.len() as u64,
        })
    }

    fn load_elf(mem: &GuestMemoryMmap, config: &VmConfig, buf: &[u8]) -> Result<HexagonBootInfo> {
        let elf =
            Elf::parse(buf).map_err(|e| Error::KernelLoad(format!("ELF parse error: {e}")))?;

        if elf.is_64 {
            return Err(Error::KernelLoad("Hexagon ELF must be 32-bit".to_string()));
        }

        let elf_little = elf.little_endian;
        let config_little = config.hexagon_endian == Endianness::Little;
        if elf_little != config_little {
            return Err(Error::KernelLoad(format!(
                "ELF endianness mismatch (file={}, config={})",
                if elf_little { "little" } else { "big" },
                if config_little { "little" } else { "big" }
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

        let entry = config
            .hexagon_entry
            .map(|addr| addr.raw())
            .unwrap_or(elf.entry);

        Ok(HexagonBootInfo {
            entry_point: entry,
            load_addr: min_addr,
            image_size: max_addr.saturating_sub(min_addr),
        })
    }
}

impl Arch for HexagonArch {
    fn name(&self) -> &'static str {
        "hexagon"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        Some(HEXAGON_UART_BASE)
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        let mut file = File::open(&config.kernel)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if buf.len() < 4 {
            return Err(Error::KernelLoad("image is too small".to_string()));
        }

        let info = if buf.starts_with(b"\x7fELF") {
            Self::load_elf(mem, config, &buf)?
        } else {
            Self::load_raw(mem, config, &buf)?
        };

        Ok(BootInfo::Hexagon(info))
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        Ok(())
    }

    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        let boot = boot
            .as_hexagon()
            .ok_or_else(|| Error::InvalidConfig("expected hexagon boot info".to_string()))?;

        let mut regs = HexagonRegisters::default();
        regs.set_pc(boot.entry_point as u32);

        let mem_end = mem.last_addr().raw_value().saturating_add(1);
        let sp = (mem_end.saturating_sub(16) & 0xffff_fff0) as u32;
        regs.r[29] = sp;
        regs.r[30] = sp;

        Ok(CpuState::hexagon(regs))
    }
}
