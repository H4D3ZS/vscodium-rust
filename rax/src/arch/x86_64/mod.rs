// On Linux x86 hosts, use linux-loader's native types.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod acpi;

/// El-Torito (BIOS CD) boot support for legacy/real-mode boot (e.g. TempleOS).
pub mod bios_boot;
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
mod native_imports {
    pub use linux_loader::cmdline::Cmdline;
    pub use linux_loader::configurator::linux::LinuxBootConfigurator;
    pub use linux_loader::configurator::{BootConfigurator, BootParams};
    pub use linux_loader::loader::bootparam::{boot_e820_entry, boot_params};
    pub use linux_loader::loader::elf::PvhBootCapability;
    pub use linux_loader::loader::{BzImage, KernelLoader, KernelLoaderResult, load_cmdline};
}

// Off Linux, use our local bootparam module so Windows builds do not pull in
// linux-loader's rawfd-only dependency path.
#[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
mod bootparam;

#[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
mod native_imports {
    pub use super::bootparam::{
        BootE820Entry as boot_e820_entry, BootParams as boot_params, KernelLoaderResult,
        PvhBootCapability, SetupHeader,
    };

    // Wrapper types to match linux-loader API
    use std::ffi::CString;
    use vm_memory::{GuestAddress, GuestMemoryMmap};

    pub struct Cmdline {
        bytes: Vec<u8>,
        max_len: usize,
    }

    impl Cmdline {
        pub fn new(max_len: usize) -> Result<Self, String> {
            if max_len == 0 {
                return Err("max length must be non-zero".to_string());
            }
            Ok(Self {
                bytes: Vec::new(),
                max_len,
            })
        }

        pub fn insert_str(&mut self, s: &str) -> Result<(), String> {
            if s.as_bytes().contains(&0) {
                return Err("command line contains NUL byte".to_string());
            }
            if s.len() + 1 > self.max_len {
                return Err(format!(
                    "command line length {} exceeds max {}",
                    s.len(),
                    self.max_len
                ));
            }
            self.bytes.clear();
            self.bytes.extend_from_slice(s.as_bytes());
            Ok(())
        }

        pub fn as_cstring(&self) -> Result<CString, std::ffi::NulError> {
            CString::new(self.bytes.clone())
        }
    }

    pub fn load_cmdline(
        mem: &GuestMemoryMmap,
        addr: GuestAddress,
        cmdline: &Cmdline,
    ) -> Result<(), crate::error::Error> {
        let s = cmdline
            .as_cstring()
            .map_err(|e| crate::error::Error::KernelLoad(format!("cmdline error: {e}")))?;
        super::bootparam::load_cmdline(mem, addr, s.to_str().unwrap_or(""))
    }

    pub struct BzImage;
    impl BzImage {
        pub fn load<F: std::io::Read + std::io::Seek>(
            mem: &GuestMemoryMmap,
            kernel_offset: Option<GuestAddress>,
            kernel_file: &mut F,
            _highmem_start: Option<GuestAddress>,
        ) -> Result<KernelLoaderResult, crate::error::Error> {
            use std::io::{Read, Seek, SeekFrom};

            let start = kernel_offset.unwrap_or(GuestAddress(0x1000000));

            // Read entire file
            kernel_file
                .seek(SeekFrom::Start(0))
                .map_err(|e| crate::error::Error::Io(e))?;
            let mut data = Vec::new();
            kernel_file
                .read_to_end(&mut data)
                .map_err(|e| crate::error::Error::Io(e))?;

            super::bootparam::load_bzimage_from_bytes(mem, &data, start)
        }
    }

    // Wrapper for boot params writing
    pub struct LinuxBootConfigurator;
    impl LinuxBootConfigurator {
        pub fn write_bootparams(
            params: &BootParams,
            mem: &GuestMemoryMmap,
        ) -> Result<(), crate::error::Error> {
            super::bootparam::write_boot_params(mem, &params.params, params.addr)
        }
    }

    pub struct BootParams {
        pub params: boot_params,
        pub addr: GuestAddress,
    }

    impl BootParams {
        pub fn new(params: &boot_params, addr: GuestAddress) -> Self {
            Self {
                params: *params,
                addr,
            }
        }
    }
}

use std::fs::File;
use std::io::Read;

use goblin::elf::Elf as GoblinElf;
use native_imports::*;
use tracing::{debug, info};
use vm_memory::{Address, Bytes, GuestAddress, GuestMemory, GuestMemoryMmap};

use crate::arch::{Arch, BootInfo, X86_64BootInfo};
#[cfg(all(feature = "kvm", target_os = "linux"))]
use crate::backend::kvm::KvmVm;
use crate::config::VmConfig;
use crate::cpu::{CpuState, DescriptorTable, Registers, Segment, SystemRegisters, X86_64CpuState};
use crate::devices::bus::{IoBus, IoRange, MmioBus, MmioRange, SharedIoDevice};
use crate::devices::debug::DebugPort;
use crate::devices::dma::Dma;
use crate::devices::fdc::Fdc;
use crate::devices::fw_cfg::FwCfg;
use crate::devices::i8042::I8042;
use crate::devices::ide::IdeController;
use crate::devices::ioapic::IoApic;
use crate::devices::map::{X86_DEBUG_PORT_BASE, X86_DEBUG_PORT_LEN};
use crate::devices::rtc::{RTC_ADDRESS, RtcStub};
use crate::devices::sysctl::SystemControl;
use crate::error::{Error, Result};
use crate::memory::{PAGE_SIZE, align_down};
use std::sync::{Arc, Mutex};

/// The primary IDE controller, stashed during device setup so the ISO boot path
/// (which runs later, in `load_kernel`) can attach the CD image as an ATAPI
/// CD-ROM on the primary master. Single-vCPU legacy boot, so a static suffices.
static BOOT_IDE_PRIMARY: Mutex<Option<Arc<Mutex<IdeController>>>> = Mutex::new(None);

/// Kernel load address - must match ELF PhysAddr (typically 16MB for x86_64 Linux)
const KERNEL_LOAD_ADDR: u64 = 0x1000000;
const BOOT_PARAMS_ADDR: u64 = 0x7000;
const CMDLINE_ADDR: u64 = 0x20000;
const GDT_ADDR: u64 = 0x500;
const TSS_ADDR: u64 = 0x1000;
const BOOT_STACK_ADDR: u64 = 0x8ff0;

// Page table addresses for identity mapping and kernel space
const PML4_ADDR: u64 = 0x9000;
const PDPTE_ADDR: u64 = 0xa000; // PDPTE for PML4[0] - identity map first 1GB
const PDE_ADDR: u64 = 0xb000; // PDE for PDPTE[0] - 512 x 2MB pages = 1GB
// Additional page tables for kernel virtual address space
const PDPTE_KERNEL_ADDR: u64 = 0xc000; // PDPTE for PML4[511] - kernel text
const PDE_KERNEL_ADDR: u64 = 0xd000; // PDE for PDPTE_KERNEL[510/511]
const PDPTE_DIRECT_ADDR: u64 = 0xe000; // PDPTE for PML4[273] - direct map

const BOOT_CS: u16 = 0x10;
const BOOT_DS: u16 = 0x18;
const BOOT_TR: u16 = 0x20;

// x86 control register bits
const X86_CR0_PE: u64 = 1 << 0;
const X86_CR0_PG: u64 = 1 << 31;
const X86_CR4_PAE: u64 = 1 << 5;
const EFER_LME: u64 = 1 << 8;
const EFER_LMA: u64 = 1 << 10;

const E820_RAM: u32 = 1;
const E820_RESERVED: u32 = 2;

const LOW_MEM_END: u64 = 0x9fc00;
const BIOS_END: u64 = 0x100000;

const KVM_RESERVED_SIZE: u64 = 0x4000;

pub struct X86_64Arch;

impl X86_64Arch {
    pub fn new() -> Self {
        X86_64Arch
    }

    fn build_cmdline(cmdline: &str) -> Result<Cmdline> {
        let mut cmdline_builder = Cmdline::new(4096)
            .map_err(|e| Error::InvalidConfig(format!("invalid cmdline: {e}")))?;
        cmdline_builder
            .insert_str(cmdline)
            .map_err(|e| Error::InvalidConfig(format!("invalid cmdline: {e}")))?;
        Ok(cmdline_builder)
    }

    /// Load kernel image - just load the binary at KERNEL_LOAD_ADDR
    /// The kernel handles its own decompression if needed
    fn load_kernel_image(
        mem: &GuestMemoryMmap,
        kernel: &VmConfig,
    ) -> Result<(KernelLoaderResult, bool, Option<u64>)> {
        info!(path = %kernel.kernel.display(), "loading kernel image");

        // Read the entire kernel file
        let kernel_data = std::fs::read(&kernel.kernel)?;
        let kernel_size = kernel_data.len() as u64;

        info!(
            size = kernel_size,
            load_addr = format!("{:#x}", KERNEL_LOAD_ADDR),
            "loading kernel binary"
        );

        // Check for bzImage format by looking for "HdrS" magic at offset 0x202
        let is_bzimage =
            kernel_data.len() > 0x206 && kernel_data[0x202..0x206] == [0x48, 0x64, 0x72, 0x53]; // "HdrS"

        if is_bzimage {
            info!("detected bzImage kernel format");
            // Use linux-loader's BzImage loader which properly parses the setup header
            let mut kernel_file = File::open(&kernel.kernel)?;
            // BzImage::load(mem, kernel_offset, kernel_image, highmem_start_address)
            // - kernel_offset: where to load the kernel (None = use code32_start from header)
            // - highmem_start_address: start of high memory (should be 0 for full access)
            let result = BzImage::load(
                mem,
                Some(GuestAddress(KERNEL_LOAD_ADDR)), // kernel_offset
                &mut kernel_file,
                Some(GuestAddress(0)), // highmem_start_address
            )
            .map_err(|e| Error::KernelLoad(format!("failed to load bzImage: {}", e)))?;

            info!(
                kernel_load = format!("{:#x}", result.kernel_load.raw_value()),
                kernel_end = format!("{:#x}", result.kernel_end),
                has_setup_header = result.setup_header.is_some(),
                "bzImage loaded"
            );

            return Ok((result, false, None));
        }

        // Not bzImage - load at KERNEL_LOAD_ADDR as raw binary
        mem.write_slice(&kernel_data, GuestAddress(KERNEL_LOAD_ADDR))?;

        let result = KernelLoaderResult {
            kernel_load: GuestAddress(KERNEL_LOAD_ADDR),
            kernel_end: KERNEL_LOAD_ADDR + kernel_size,
            setup_header: None,
            pvh_boot_cap: PvhBootCapability::PvhEntryNotPresent,
        };

        // Check if it's ELF to get entry point, otherwise use load address
        let mut kernel_file = File::open(&kernel.kernel)?;
        let mut magic = [0u8; 4];
        kernel_file.read_exact(&mut magic)?;

        if magic == [0x7f, b'E', b'L', b'F'] {
            let elf = GoblinElf::parse(&kernel_data)
                .map_err(|e| Error::KernelLoad(format!("failed to parse ELF: {}", e)))?;

            // Load each LOAD segment to its correct physical address
            let mut max_phys_end = KERNEL_LOAD_ADDR;
            for ph in elf.program_headers.iter() {
                if ph.p_type == goblin::elf::program_header::PT_LOAD {
                    let phys_addr = ph.p_paddr;
                    let file_offset = ph.p_offset as usize;
                    let file_size = ph.p_filesz as usize;
                    let mem_size = ph.p_memsz as usize;

                    info!(
                        phys_addr = format!("{:#x}", phys_addr),
                        virt_addr = format!("{:#x}", ph.p_vaddr),
                        file_offset = format!("{:#x}", file_offset),
                        file_size = format!("{:#x}", file_size),
                        mem_size = format!("{:#x}", mem_size),
                        "loading ELF LOAD segment"
                    );

                    // Load file content
                    if file_size > 0 && file_offset + file_size <= kernel_data.len() {
                        mem.write_slice(
                            &kernel_data[file_offset..file_offset + file_size],
                            GuestAddress(phys_addr),
                        )?;
                    }

                    // Zero BSS (mem_size > file_size)
                    if mem_size > file_size {
                        let bss_start = phys_addr + file_size as u64;
                        let bss_size = mem_size - file_size;
                        let zeros = vec![0u8; bss_size];
                        mem.write_slice(&zeros, GuestAddress(bss_start))?;
                    }

                    let seg_end = phys_addr + mem_size as u64;
                    if seg_end > max_phys_end {
                        max_phys_end = seg_end;
                    }
                }
            }

            // Update kernel_end to reflect actual loaded size
            let result = KernelLoaderResult {
                kernel_load: GuestAddress(KERNEL_LOAD_ADDR),
                kernel_end: max_phys_end,
                setup_header: None,
                pvh_boot_cap: PvhBootCapability::PvhEntryNotPresent,
            };

            // The ELF entry point for vmlinux. Modern kernels built with
            // CONFIG_PHYSICAL_START have e_entry as a physical address.
            // The kernel expects to start with identity mapping (phys == virt)
            // and sets up its own virtual address mapping during early boot.
            // The __pi___startup_64 code verifies: (kernel_base >> 46) == 0
            // which requires running at the physical address, not virtual.
            let entry = elf.header.e_entry;

            info!(
                entry = format!("{:#x}", entry),
                kernel_end = format!("{:#x}", max_phys_end),
                "ELF kernel entry point"
            );
            Ok((result, true, Some(entry)))
        } else {
            info!("raw binary kernel, entry at load address");
            Ok((result, false, None))
        }
    }

    fn load_initrd(
        mem: &GuestMemoryMmap,
        initrd_path: &std::path::Path,
        max_addr: u64,
        kernel_end: u64,
    ) -> Result<(GuestAddress, u64)> {
        info!(path = %initrd_path.display(), "loading initrd");
        let buf = std::fs::read(initrd_path)?;
        let size = buf.len() as u64;
        if size == 0 {
            return Err(Error::KernelLoad("initrd is empty".to_string()));
        }
        if max_addr < size {
            return Err(Error::KernelLoad(
                "initrd does not fit in guest memory".to_string(),
            ));
        }
        let start = align_down(max_addr.saturating_sub(size), PAGE_SIZE);
        if start < kernel_end {
            return Err(Error::KernelLoad(
                "initrd overlaps kernel image".to_string(),
            ));
        }
        let start_addr = GuestAddress(start);
        mem.write_slice(&buf, start_addr)?;
        Ok((start_addr, size))
    }

    fn build_e820(mem_size: u64, reserved_start: u64) -> Vec<boot_e820_entry> {
        let mut entries = Vec::new();
        if LOW_MEM_END > 0 {
            entries.push(boot_e820_entry {
                addr: 0,
                size: LOW_MEM_END,
                type_: E820_RAM,
            });
        }
        if BIOS_END > LOW_MEM_END {
            entries.push(boot_e820_entry {
                addr: LOW_MEM_END,
                size: BIOS_END - LOW_MEM_END,
                type_: E820_RESERVED,
            });
        }
        if reserved_start > BIOS_END {
            entries.push(boot_e820_entry {
                addr: BIOS_END,
                size: reserved_start - BIOS_END,
                type_: E820_RAM,
            });
        }
        if mem_size > reserved_start {
            entries.push(boot_e820_entry {
                addr: reserved_start,
                size: mem_size - reserved_start,
                type_: E820_RESERVED,
            });
        }
        entries
    }

    /// Build initial system registers for 64-bit long mode boot.
    fn build_sregs() -> SystemRegisters {
        // 64-bit code segment (L=1, D=0 for long mode)
        let cs = Segment {
            base: 0,
            limit: 0xfffff,
            selector: BOOT_CS,
            type_: 0x0b, // Execute/Read, accessed
            present: true,
            dpl: 0,
            db: false, // Must be 0 for 64-bit code
            s: true,
            l: true, // 64-bit mode
            g: true,
            avl: false,
            unusable: false,
        };

        // Data segment
        let ds = Segment {
            base: 0,
            limit: 0xfffff,
            selector: BOOT_DS,
            type_: 0x03, // Read/Write, accessed
            present: true,
            dpl: 0,
            db: true,
            s: true,
            l: false,
            g: true,
            avl: false,
            unusable: false,
        };

        // TSS segment
        let tr = Segment {
            base: TSS_ADDR,
            limit: 0x67, // Minimum TSS size - 1
            selector: BOOT_TR,
            type_: 0x0b, // 32-bit busy TSS
            present: true,
            dpl: 0,
            db: false,
            s: false, // System segment
            l: false,
            g: false,
            avl: false,
            unusable: false,
        };

        // LDT (not used but needs valid state)
        let ldt = Segment {
            base: 0,
            limit: 0xffff,
            selector: 0,
            type_: 2,
            present: true,
            dpl: 0,
            db: false,
            s: false,
            l: false,
            g: false,
            avl: false,
            unusable: true,
        };

        SystemRegisters {
            cs,
            ds: ds.clone(),
            es: ds.clone(),
            fs: ds.clone(),
            gs: ds.clone(),
            ss: ds,
            tr,
            ldt,
            gdt: DescriptorTable {
                base: GDT_ADDR,
                limit: (5 * 8 - 1) as u16,
            },
            idt: DescriptorTable {
                base: 0,
                limit: 0xffff,
            },
            cr0: X86_CR0_PE | X86_CR0_PG | 0x20, // PE + PG + NE
            cr2: 0,
            cr3: PML4_ADDR,
            cr4: X86_CR4_PAE,
            cr8: 0,
            efer: EFER_LME | EFER_LMA,
            star: 0,
            lstar: 0,
            cstar: 0,
            fmask: 0,
            sysenter_cs: 0,
            sysenter_esp: 0,
            sysenter_eip: 0,
            dr0: 0,
            dr1: 0,
            dr2: 0,
            dr3: 0,
            dr6: 0xFFFF0FF0, // Default value after reset
            dr7: 0x00000400, // Default value after reset
        }
    }

    /// Set up page tables for identity mapping and kernel virtual address space.
    ///
    /// Creates the following mappings:
    /// - PML4[0]: Identity maps first 1GB (virtual 0x0 -> physical 0x0)
    /// - PML4[273]: Direct physical memory map at 0xffff888000000000
    /// - PML4[511]: Kernel text area at 0xffffffff80000000
    fn setup_page_tables(mem: &GuestMemoryMmap) -> Result<()> {
        // Clear all page table pages first
        let zero_page = [0u8; 4096];
        mem.write_slice(&zero_page, GuestAddress(PML4_ADDR))?;
        mem.write_slice(&zero_page, GuestAddress(PDPTE_ADDR))?;
        mem.write_slice(&zero_page, GuestAddress(PDE_ADDR))?;
        mem.write_slice(&zero_page, GuestAddress(PDPTE_KERNEL_ADDR))?;
        mem.write_slice(&zero_page, GuestAddress(PDE_KERNEL_ADDR))?;
        mem.write_slice(&zero_page, GuestAddress(PDPTE_DIRECT_ADDR))?;

        // === PML4 entries ===
        // PML4[0] - Identity map (virtual 0x0 - 0x7FFFFFFFFF -> physical 0x0 - 0x7FFFFFFFFF)
        let pml4_entry_0: u64 = PDPTE_ADDR | 0x3; // Present + Writable
        mem.write_obj(pml4_entry_0, GuestAddress(PML4_ADDR + 0 * 8))?;

        // PML4[273] - Direct physical memory map at 0xffff888000000000
        // Linux uses this for the "direct map" of all physical memory
        let pml4_entry_273: u64 = PDPTE_DIRECT_ADDR | 0x3;
        mem.write_obj(pml4_entry_273, GuestAddress(PML4_ADDR + 273 * 8))?;

        // PML4[511] - Kernel text area at 0xffffffff80000000
        let pml4_entry_511: u64 = PDPTE_KERNEL_ADDR | 0x3;
        mem.write_obj(pml4_entry_511, GuestAddress(PML4_ADDR + 511 * 8))?;

        // === PDPTE for identity mapping (PML4[0]) ===
        // Use 1GB huge pages directly in PDPTE to cover more memory
        // Each entry covers 1GB, we create 8 entries for 8GB coverage
        for i in 0u64..8 {
            let pdpte_entry: u64 = (i << 30) | 0x83; // Present + Writable + Huge (1GB page)
            mem.write_obj(pdpte_entry, GuestAddress(PDPTE_ADDR + i * 8))?;
        }

        // === PDPTE for direct map (PML4[273] at 0xffff888000000000) ===
        // The direct map provides physical memory access at high virtual addresses.
        // Map first 8GB properly (8 entries), rest wrap to physical 0
        for i in 0u64..512 {
            let phys_addr = if i < 8 { i << 30 } else { 0 }; // First 8 entries = 8GB, rest = 0
            let pdpte_entry: u64 = phys_addr | 0x83; // Present + Writable + Huge (1GB page)
            mem.write_obj(pdpte_entry, GuestAddress(PDPTE_DIRECT_ADDR + i * 8))?;
        }

        // === PDPTE for kernel text (PML4[511]) ===
        // PML4[511] covers virtual 0xffffff8000000000 to 0xffffffffffffffff (512 GB).
        // The kernel text area starts at 0xffffffff80000000.
        //
        // Virtual address 0xffffffff80000000 is in PML4[511] at:
        //   offset = 0xffffffff80000000 - 0xffffff8000000000 = 0x7f80000000
        //   PDPTE index = offset / 1GB = 510
        //
        // Linux kernel expects: virt 0xffffffff80000000 + X -> phys X
        // So PDPTE[510] should map to physical 0, PDPTE[511] to physical 1GB, etc.
        //
        // PDPTE[i] for i < 510: not used by kernel, map to 0
        // PDPTE[510]: physical 0 (covers 0xffffffff80000000-0xffffffffbfffffff)
        // PDPTE[511]: physical 1GB (covers 0xffffffffc0000000-0xffffffffffffffff)
        for i in 0u64..512 {
            let phys_addr = if i >= 510 {
                // Kernel text region: PDPTE[510+j] -> physical j*1GB
                ((i - 510) % 8) << 30
            } else {
                // Below kernel text: not normally used, map to 0 for safety
                0
            };
            let pdpte_entry: u64 = phys_addr | 0x83; // Present + Writable + Huge (1GB page)
            mem.write_obj(pdpte_entry, GuestAddress(PDPTE_KERNEL_ADDR + i * 8))?;
        }

        debug!(
            pml4 = format!("{:#x}", PML4_ADDR),
            "setup page tables: identity map + kernel space + direct map"
        );
        Ok(())
    }

    fn write_gdt(mem: &GuestMemoryMmap) -> Result<()> {
        // Build TSS descriptor (16 bytes for 32-bit TSS)
        let tss_base = TSS_ADDR;
        let tss_limit: u32 = 0x67;

        // 32-bit TSS descriptor (type 0x89 = available 32-bit TSS)
        let tss_low = (tss_limit & 0xffff) as u64
            | ((tss_base & 0xffffff) << 16)
            | (0x89u64 << 40)
            | ((tss_limit as u64 >> 16) & 0xf) << 48
            | ((tss_base >> 24) & 0xff) << 56;

        // 64-bit code segment: L=1, D=0
        let code64_entry = gdt_entry_64bit(0x9a);

        let gdt = [
            0u64,            // 0x00: null
            0u64,            // 0x08: null
            code64_entry,    // 0x10: 64-bit code segment
            gdt_entry(0x92), // 0x18: data segment
            tss_low,         // 0x20: TSS descriptor
        ];
        debug!(
            gdt_entries = format!("{:#018x?}", gdt),
            gdt_addr = format!("{:#x}", GDT_ADDR),
            "writing GDT"
        );
        for (index, entry) in gdt.iter().enumerate() {
            let addr = GuestAddress(GDT_ADDR + (index as u64 * 8));
            mem.write_obj(*entry, addr)?;
        }

        // Write a minimal TSS at TSS_ADDR
        let tss = [0u8; 104];
        mem.write_slice(&tss, GuestAddress(TSS_ADDR))?;

        debug!(tss_addr = format!("{:#x}", TSS_ADDR), "wrote TSS");
        Ok(())
    }
}

impl Arch for X86_64Arch {
    fn name(&self) -> &'static str {
        "x86_64"
    }

    fn setup_devices(&self, io_bus: &mut IoBus, mmio_bus: &mut MmioBus) -> Result<()> {
        // The PCI host bridge (config ports 0xCF8-0xCFF) is created and owned by
        // the VMM so it can be shared with the emulator MMU for BAR-mapped MMIO
        // routing; it is no longer registered here.

        // CMOS/RTC
        io_bus.register(
            IoRange {
                base: RTC_ADDRESS,
                len: 2,
            },
            Box::new(RtcStub::new()),
        )?;

        io_bus.register(
            IoRange {
                base: X86_DEBUG_PORT_BASE,
                len: X86_DEBUG_PORT_LEN,
            },
            Box::new(DebugPort::new()),
        )?;

        // QEMU fw_cfg (selector 0x510, data 0x511) - reachable via PIO exits.
        io_bus.register(
            IoRange {
                base: 0x510,
                len: 2,
            },
            Box::new(FwCfg::new()),
        )?;

        // I/O APIC MMIO window at 0xFEC00000 (24 redirection entries).
        mmio_bus.register(
            MmioRange {
                base: 0xFEC0_0000,
                len: 0x20,
            },
            Box::new(IoApic::new()),
        )?;

        // ---------------------------------------------------------------------
        // Legacy ISA devices. These are wired so the guest's drivers can probe
        // and attach to them. Devices that span several non-contiguous port
        // windows share one instance via SharedIoDevice.
        // ---------------------------------------------------------------------

        // 8237A DMA controller pair. Reachable at 0x00-0x0F (controller 1),
        // 0xC0-0xDF (controller 2), and the page-register file 0x80-0x8F (which
        // also covers the 0x80 POST diagnostic port).
        let dma = Arc::new(Mutex::new(Dma::new()));
        for range in [
            IoRange {
                base: 0x00,
                len: 0x10,
            },
            IoRange {
                base: 0x80,
                len: 0x10,
            },
            IoRange {
                base: 0xC0,
                len: 0x20,
            },
        ] {
            io_bus.register(range, Box::new(SharedIoDevice::new(dma.clone())))?;
        }

        // Legacy system-control ports: 0x61 (NMI status / PC speaker / refresh)
        // and 0x92 (Port A: A20 gate + fast reset). 0xCF9 (reset control) is not
        // wired separately because it falls inside the PCI CONFIG_ADDRESS window
        // (0xCF8-0xCFF) owned by the PCI host bridge.
        let sysctl = Arc::new(Mutex::new(SystemControl::new()));
        for base in [0x61u16, 0x92] {
            io_bus.register(
                IoRange { base, len: 1 },
                Box::new(SharedIoDevice::new(sysctl.clone())),
            )?;
        }

        // i8042 PS/2 keyboard/mouse controller: data 0x60, status/command 0x64.
        // 0x61 in between belongs to the system-control block above.
        let i8042 = Arc::new(Mutex::new(I8042::new()));
        io_bus.register(
            IoRange { base: 0x60, len: 1 },
            Box::new(SharedIoDevice::new(i8042.clone())),
        )?;
        io_bus.register(
            IoRange { base: 0x64, len: 1 },
            Box::new(SharedIoDevice::new(i8042.clone())),
        )?;

        // Legacy IDE/ATA controllers, present with no media attached (the guest
        // detects the channel and finds no drive). Primary: 0x1F0-0x1F7 cmd +
        // 0x3F6 control; secondary: 0x170-0x177 cmd + 0x376 control.
        let ide_primary = Arc::new(Mutex::new(IdeController::new(0x1F0, 0x3F6, Vec::new())));
        // Stash for the ISO boot path to attach the CD as an ATAPI CD-ROM later.
        *BOOT_IDE_PRIMARY.lock().unwrap() = Some(ide_primary.clone());
        io_bus.register(
            IoRange {
                base: 0x1F0,
                len: 8,
            },
            Box::new(SharedIoDevice::new(ide_primary.clone())),
        )?;
        io_bus.register(
            IoRange {
                base: 0x3F6,
                len: 1,
            },
            Box::new(SharedIoDevice::new(ide_primary.clone())),
        )?;
        let ide_secondary = Arc::new(Mutex::new(IdeController::new(0x170, 0x376, Vec::new())));
        io_bus.register(
            IoRange {
                base: 0x170,
                len: 8,
            },
            Box::new(SharedIoDevice::new(ide_secondary.clone())),
        )?;
        io_bus.register(
            IoRange {
                base: 0x376,
                len: 1,
            },
            Box::new(SharedIoDevice::new(ide_secondary.clone())),
        )?;

        // 82077AA floppy disk controller, no floppy inserted. The 0x3F0-0x3F7
        // window minus 0x3F6 (which is the IDE primary control port above):
        // 0x3F0-0x3F5 plus 0x3F7.
        let fdc = Arc::new(Mutex::new(Fdc::new()));
        io_bus.register(
            IoRange {
                base: 0x3F0,
                len: 6,
            },
            Box::new(SharedIoDevice::new(fdc.clone())),
        )?;
        io_bus.register(
            IoRange {
                base: 0x3F7,
                len: 1,
            },
            Box::new(SharedIoDevice::new(fdc.clone())),
        )?;

        Ok(())
    }

    fn serial_irq(&self) -> Option<u32> {
        Some(4)
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        // Legacy/El-Torito boot: if the image is an ISO-9660 CD (cheap probe for
        // "CD001" at offset 0x8001), set it up for a 16-bit real-mode BIOS boot
        // (load the boot image at 0x7C00, install the CD for INT 13h) instead of
        // loading a Linux kernel into long mode.
        {
            use std::io::{Read, Seek, SeekFrom};
            let mut magic = [0u8; 5];
            let looks_iso = File::open(&config.kernel)
                .and_then(|mut f| {
                    f.seek(SeekFrom::Start(0x8001))?;
                    f.read_exact(&mut magic)
                })
                .map(|_| bios_boot::is_iso_image_header(&magic))
                .unwrap_or(false);
            if looks_iso {
                info!(path = %config.kernel.display(), "ISO image detected — El-Torito real-mode boot");
                let data = std::fs::read(&config.kernel)?;
                bios_boot::arm_real_mode_boot(mem, data, config.memory.bytes())
                    .map_err(|e| Error::KernelLoad(format!("El-Torito boot setup: {e}")))?;
                // Also expose the ISO as an ATAPI CD-ROM on the primary IDE
                // master, so a guest that probes ATA for its CD/DVD (e.g.
                // TempleOS) finds and mounts it instead of stalling.
                if let (Some(ide), Some(iso)) = (
                    BOOT_IDE_PRIMARY.lock().unwrap().clone(),
                    crate::backend::emulator::x86_64::bios::installed_cd(),
                ) {
                    ide.lock().unwrap().attach_cdrom(iso);
                }
                return Ok(BootInfo::X86_64(X86_64BootInfo {
                    entry_point: 0x7C00,
                    boot_params_addr: GuestAddress(0),
                    tss_addr: 0,
                    identity_map_addr: 0,
                }));
            }
        }

        // Use the configured memory size (what we report to the kernel via e820),
        // NOT the actual allocated size (which includes padding for per-CPU overflow).
        let mem_size = config.memory.bytes();
        if mem_size <= KERNEL_LOAD_ADDR + KVM_RESERVED_SIZE {
            return Err(Error::InvalidConfig(
                "memory is too small for kernel and reserved pages".to_string(),
            ));
        }

        // Calculate reserved_start for e820 and initrd placement (within reported memory)
        let reserved_start = align_down(mem_size - KVM_RESERVED_SIZE, PAGE_SIZE);

        // Place identity map and TSS addresses in the MMIO gap below 4GB.
        // These must NOT overlap with registered guest memory slots.
        // KVM's set_tss_address creates an internal memory slot, which fails
        // with EEXIST if it overlaps user-registered memory.
        // The memory registration leaves a gap at KVM_TSS_IDENTITY_GAP_START for this purpose.
        use crate::memory::KVM_TSS_IDENTITY_GAP_START;
        let identity_map_addr = KVM_TSS_IDENTITY_GAP_START;
        let tss_addr = KVM_TSS_IDENTITY_GAP_START + PAGE_SIZE;

        let (loader_result, is_elf, elf_phys_entry) = Self::load_kernel_image(mem, config)?;

        let kernel_end = loader_result.kernel_end as u64;
        if kernel_end >= reserved_start {
            return Err(Error::KernelLoad(
                "kernel image overlaps reserved KVM region".to_string(),
            ));
        }

        // Build command line
        let cmdline = Self::build_cmdline(&config.cmdline)?;
        info!(cmdline = %config.cmdline, "loading kernel command line");
        load_cmdline(mem, GuestAddress(CMDLINE_ADDR), &cmdline).map_err(Error::from)?;
        let cmdline_size = cmdline
            .as_cstring()
            .map_err(|e| Error::KernelLoad(format!("cmdline error: {e}")))?
            .as_bytes_with_nul()
            .len() as u32;
        if CMDLINE_ADDR + cmdline_size as u64 >= BIOS_END {
            return Err(Error::KernelLoad("cmdline exceeds low memory".to_string()));
        }

        let entry_point = if is_elf {
            // ELF kernel (vmlinux) - simpler boot process
            // The entry point is directly from the ELF header
            let mut params = boot_params::default();

            // Configure VGA text mode console (80x25, mode 3)
            params.screen_info.orig_video_mode = 3; // Standard VGA text mode
            params.screen_info.orig_video_cols = 80; // 80 columns
            params.screen_info.orig_video_lines = 25; // 25 rows
            params.screen_info.orig_video_isVGA = 1; // VGA detected
            params.screen_info.orig_video_points = 16; // 16 scanlines per char

            params.hdr.type_of_loader = 0xff;
            params.hdr.loadflags = 0x1 | 0x40; // LOADED_HIGH + KEEP_SEGMENTS
            params.hdr.cmd_line_ptr = CMDLINE_ADDR as u32;
            params.hdr.cmdline_size = cmdline_size;

            // Load initrd if specified
            if let Some(initrd_path) = &config.initrd {
                let initrd_max = reserved_start - 1;
                let (initrd_addr, initrd_size) =
                    Self::load_initrd(mem, initrd_path, initrd_max, kernel_end)?;
                params.hdr.ramdisk_image = initrd_addr.raw_value() as u32;
                params.hdr.ramdisk_size = initrd_size as u32;
            }

            // Build e820 memory map
            let e820_entries = Self::build_e820(mem_size, reserved_start);
            debug!(entries = e820_entries.len(), "built e820 map");
            params.e820_entries = e820_entries.len() as u8;
            for (index, entry) in e820_entries.iter().enumerate() {
                params.e820_table[index] = *entry;
            }

            let boot_params = BootParams::new(&params, GuestAddress(BOOT_PARAMS_ADDR));
            LinuxBootConfigurator::write_bootparams(&boot_params, mem)?;

            // For ELF vmlinux, use the physical address of the first LOAD segment
            // This is where startup_64 (or a jump to it) is located
            // Note: We use the parsed PhysAddr, NOT the ELF e_entry which may be different
            let entry = elf_phys_entry.unwrap_or(loader_result.kernel_load.raw_value());
            debug!(
                entry = format!("{:#x}", entry),
                kernel_end = format!("{:#x}", kernel_end),
                "ELF kernel entry point (physical start of first LOAD segment)"
            );
            entry
        } else if let Some(setup_header) = loader_result.setup_header {
            // bzImage kernel - needs decompression setup
            // Note: Do NOT patch the bzImage - the compressed kernel data would be corrupted.
            // Any patches should be applied after decompression by the kernel itself.

            let hdr_version = { setup_header.version };
            let hdr_loadflags = { setup_header.loadflags };
            let hdr_code32_start = { setup_header.code32_start };
            debug!(
                version = format!("{:#x}", hdr_version),
                loadflags = format!("{:#x}", hdr_loadflags),
                code32_start = format!("{:#x}", hdr_code32_start),
                "setup header"
            );

            let mut params = boot_params::default();

            // Configure VGA text mode console (80x25, mode 3)
            params.screen_info.orig_video_mode = 3; // Standard VGA text mode
            params.screen_info.orig_video_cols = 80; // 80 columns
            params.screen_info.orig_video_lines = 25; // 25 rows
            params.screen_info.orig_video_isVGA = 1; // VGA detected
            params.screen_info.orig_video_points = 16; // 16 scanlines per char

            params.hdr = setup_header;
            params.hdr.type_of_loader = 0xff;
            params.hdr.loadflags |= 0x1 | 0x40;
            params.hdr.cmd_line_ptr = CMDLINE_ADDR as u32;
            params.hdr.cmdline_size = cmdline_size;
            params.hdr.pref_address = 0x5076000;

            let pref_addr = params.hdr.pref_address;
            let init_sz = params.hdr.init_size;
            debug!(
                pref_address = format!("{:#x}", pref_addr),
                init_size = format!("{:#x}", init_sz),
                "set pref_address close to decompressor bp"
            );

            if let Some(initrd_path) = &config.initrd {
                let initrd_addr_max = if params.hdr.initrd_addr_max == 0 {
                    reserved_start - 1
                } else {
                    params.hdr.initrd_addr_max as u64
                };
                let initrd_max = initrd_addr_max.min(reserved_start - 1);
                let (initrd_addr, initrd_size) =
                    Self::load_initrd(mem, initrd_path, initrd_max, kernel_end)?;
                params.hdr.ramdisk_image = initrd_addr.raw_value() as u32;
                params.hdr.ramdisk_size = initrd_size as u32;
            }

            let e820_entries = Self::build_e820(mem_size, reserved_start);
            debug!(entries = e820_entries.len(), "built e820 map");
            params.e820_entries = e820_entries.len() as u8;
            for (index, entry) in e820_entries.iter().enumerate() {
                params.e820_table[index] = *entry;
            }

            let boot_params = BootParams::new(&params, GuestAddress(BOOT_PARAMS_ADDR));
            LinuxBootConfigurator::write_bootparams(&boot_params, mem)?;

            // Verify kernel is loaded
            let mut first_bytes = [0u8; 16];
            mem.read_slice(
                &mut first_bytes,
                GuestAddress(loader_result.kernel_load.raw_value()),
            )
            .map_err(|e| Error::KernelLoad(format!("failed to read kernel: {e}")))?;
            debug!(
                entry = format!("{:#x}", loader_result.kernel_load.raw_value()),
                kernel_end = format!("{:#x}", loader_result.kernel_end),
                first_bytes = format!("{:02x?}", first_bytes),
                "kernel loaded"
            );

            // For 64-bit boot, use startup_64 at offset 0x200 from the 32-bit entry
            let entry_point_64 = loader_result.kernel_load.raw_value() + 0x200;

            let mut entry64_bytes = [0u8; 16];
            mem.read_slice(&mut entry64_bytes, GuestAddress(entry_point_64))
                .map_err(|e| Error::KernelLoad(format!("failed to read entry64: {e}")))?;

            debug!(
                entry32 = format!("{:#x}", loader_result.kernel_load.raw_value()),
                entry64 = format!("{:#x}", entry_point_64),
                entry64_bytes = format!("{:02x?}", entry64_bytes),
                "kernel entry points"
            );

            entry_point_64
        } else {
            // Raw binary - just load and jump to it
            // Load initrd if specified (place it after the kernel)
            if let Some(initrd_path) = &config.initrd {
                let initrd_max = reserved_start - 1;
                let (initrd_addr, initrd_size) =
                    Self::load_initrd(mem, initrd_path, initrd_max, kernel_end)?;
                info!(
                    initrd_addr = format!("{:#x}", initrd_addr.raw_value()),
                    initrd_size = initrd_size,
                    "initrd loaded for raw binary"
                );
            }

            info!(
                entry = format!("{:#x}", loader_result.kernel_load.raw_value()),
                "raw binary entry point"
            );
            loader_result.kernel_load.raw_value()
        };

        Ok(BootInfo::X86_64(X86_64BootInfo {
            entry_point,
            boot_params_addr: GuestAddress(BOOT_PARAMS_ADDR),
            tss_addr,
            identity_map_addr,
        }))
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, vm: &KvmVm, boot: &BootInfo) -> Result<()> {
        let boot = boot
            .as_x86_64()
            .ok_or_else(|| Error::InvalidConfig("expected x86_64 boot info".to_string()))?;
        debug!("creating IRQ chip");
        vm.create_irq_chip()?;
        debug!("creating PIT2");
        vm.create_pit2()?;
        debug!(
            tss_addr = format!("{:#x}", boot.tss_addr),
            "setting TSS address"
        );
        vm.set_tss_address(boot.tss_addr)?;
        debug!(
            identity_map_addr = format!("{:#x}", boot.identity_map_addr),
            "setting identity map address"
        );
        vm.set_identity_map_address(boot.identity_map_addr)?;
        debug!("init_vm complete");
        Ok(())
    }

    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        // El-Torito real-mode boot: use the armed 16-bit CPU state (CR0.PE=0,
        // CS:IP=0:0x7C00, boot image already loaded) — skip the long-mode page
        // tables / GDT entirely.
        if let Some((sregs, regs)) = bios_boot::armed_real_mode_state() {
            return Ok(CpuState::X86_64(X86_64CpuState { regs, sregs }));
        }

        let boot = boot
            .as_x86_64()
            .ok_or_else(|| Error::InvalidConfig("expected x86_64 boot info".to_string()))?;
        // Setup page tables and GDT in guest memory
        Self::setup_page_tables(mem)?;
        Self::write_gdt(mem)?;

        // Build initial CPU state
        let regs = Registers {
            rip: boot.entry_point,
            rflags: 0x2,
            rsp: BOOT_STACK_ADDR,
            rbp: 0,
            rbx: 0,
            rdi: 0,
            rsi: boot.boot_params_addr.raw_value(),
            ..Default::default()
        };

        let sregs = Self::build_sregs();

        info!(
            rip = format!("{:#x}", regs.rip),
            rsp = format!("{:#x}", regs.rsp),
            rsi = format!("{:#x}", regs.rsi),
            cr0 = format!("{:#x}", sregs.cr0),
            cr3 = format!("{:#x}", sregs.cr3),
            cr4 = format!("{:#x}", sregs.cr4),
            efer = format!("{:#x}", sregs.efer),
            cs_l = sregs.cs.l,
            "initial CPU state built"
        );

        Ok(CpuState::X86_64(X86_64CpuState { regs, sregs }))
    }
}

fn gdt_entry(access: u8) -> u64 {
    let flags: u64 = 0xcf; // G=1, D/B=1, L=0, AVL=1
    let limit: u64 = 0xffff;
    let base: u64 = 0;
    (limit & 0xffff)
        | ((base & 0xffffff) << 16)
        | ((access as u64) << 40)
        | ((flags as u64) << 48)
        | ((base >> 24) << 56)
}

/// Create a 64-bit code segment GDT entry (L=1, D=0)
fn gdt_entry_64bit(access: u8) -> u64 {
    let flags: u64 = 0xaf; // G=1, D/B=0, L=1, AVL=1 (64-bit mode)
    let limit: u64 = 0xffff;
    let base: u64 = 0;
    (limit & 0xffff)
        | ((base & 0xffffff) << 16)
        | ((access as u64) << 40)
        | ((flags as u64) << 48)
        | ((base >> 24) << 56)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_e820_layout() {
        let entries = X86_64Arch::build_e820(512 * 1024 * 1024, 0x1fffc000);
        assert!(entries.len() >= 3);
        let first_type = unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(entries[0].type_)) };
        assert_eq!(first_type, E820_RAM);
    }
}
