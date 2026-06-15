//! ARM architecture family support.
//!
//! This module provides architecture definitions for the ARM family:
//! - AArch64 (64-bit ARMv8-A)
//! - ARMv7-A (32-bit Cortex-A series)
//! - ARMv8-A AArch32 (32-bit mode on ARMv8)
//! - Cortex-M (Thumb-2 based microcontrollers)
//! - Cortex-R (real-time processors)

use std::fs::File;
use std::io::Read;

use goblin::elf::Elf;
use vm_memory::{Address, Bytes, GuestAddress, GuestMemory, GuestMemoryMmap};

use crate::arch::{Arch, BootInfo};
use crate::config::VmConfig;
use crate::cpu::CpuState;
use crate::devices::bus::{IoBus, MmioBus};
use crate::error::{Error, Result};

// =============================================================================
// ARM Boot Info
// =============================================================================

/// Boot information for ARM platforms.
#[derive(Clone, Debug)]
pub struct ArmBootInfo {
    /// Entry point address
    pub entry_point: u64,
    /// Load address of the image
    pub load_addr: u64,
    /// Size of the loaded image
    pub image_size: u64,
    /// Device tree blob address (if applicable)
    pub dtb_addr: Option<u64>,
    /// Initial stack pointer
    pub initial_sp: Option<u64>,
}

// =============================================================================
// AArch64 Architecture
// =============================================================================

/// UART base address for the PL011 (ARM PrimeCell UART)
pub const AARCH64_UART_BASE: u64 = 0x0900_0000;
/// PL011 interrupt: SPI 1 = GIC INTID 33
pub const AARCH64_UART_IRQ: u32 = 33;
/// GICv3 distributor base
pub const AARCH64_GICD_BASE: u64 = 0x0800_0000;
/// GICv3 redistributor region base
pub const AARCH64_GICR_BASE: u64 = 0x080A_0000;
/// RAM base address
pub const AARCH64_RAM_BASE: u64 = 0x4000_0000;

/// GICv3 distributor frame size advertised in the device tree.
const GICD_SIZE: u64 = 0x1_0000;
/// GICv3 redistributor frame size per CPU (RD + SGI frames).
const GICR_SIZE_PER_CPU: u64 = 0x2_0000;

/// ARM64 `Image` header magic ("ARM\x64") at offset 56.
pub const AARCH64_IMAGE_MAGIC: u32 = 0x644D_5241;

/// Fields of the ARM64 Image header needed for placement (all little-endian).
struct Aarch64ImageHeader {
    /// Image load offset from a 2MB-aligned RAM base.
    text_offset: u64,
    /// Effective kernel memory footprint (incl. BSS); 0 on very old kernels.
    image_size: u64,
}

fn parse_aarch64_image_header(buf: &[u8]) -> Option<Aarch64ImageHeader> {
    if buf.len() < 64 {
        return None;
    }
    let magic = u32::from_le_bytes(buf[56..60].try_into().unwrap());
    if magic != AARCH64_IMAGE_MAGIC {
        return None;
    }
    Some(Aarch64ImageHeader {
        text_offset: u64::from_le_bytes(buf[8..16].try_into().unwrap()),
        image_size: u64::from_le_bytes(buf[16..24].try_into().unwrap()),
    })
}

/// Returns true when the file looks like an ARM64 Linux `Image`.
pub fn is_aarch64_image(buf: &[u8]) -> bool {
    parse_aarch64_image_header(buf).is_some()
}

pub struct Aarch64Arch;

impl Aarch64Arch {
    pub fn new() -> Self {
        Aarch64Arch
    }

    fn load_elf(mem: &GuestMemoryMmap, buf: &[u8]) -> Result<ArmBootInfo> {
        let elf =
            Elf::parse(buf).map_err(|e| Error::KernelLoad(format!("ELF parse error: {e}")))?;

        if !elf.is_64 {
            return Err(Error::KernelLoad("AArch64 ELF must be 64-bit".to_string()));
        }

        // Check for ARM64 machine type
        if elf.header.e_machine != goblin::elf::header::EM_AARCH64 {
            return Err(Error::KernelLoad(format!(
                "Expected AArch64 ELF (e_machine=183), got {}",
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

        Ok(ArmBootInfo {
            entry_point: elf.entry,
            load_addr: min_addr,
            image_size: max_addr.saturating_sub(min_addr),
            dtb_addr: None,
            initial_sp: None,
        })
    }

    fn load_raw(mem: &GuestMemoryMmap, buf: &[u8]) -> Result<ArmBootInfo> {
        // ARM64 boot protocol: load the Image at a 2MB-aligned base plus the
        // header's text_offset. image_size covers BSS, which must not overlap
        // anything we place after the kernel.
        let header = parse_aarch64_image_header(buf);
        let text_offset = header.as_ref().map(|h| h.text_offset).unwrap_or(0);
        let image_size = header
            .as_ref()
            .map(|h| h.image_size)
            .filter(|&s| s != 0)
            .unwrap_or(buf.len() as u64)
            .max(buf.len() as u64);

        let load_addr = AARCH64_RAM_BASE + text_offset;
        mem.write_slice(buf, GuestAddress(load_addr))?;

        Ok(ArmBootInfo {
            entry_point: load_addr,
            load_addr,
            image_size,
            dtb_addr: None,
            initial_sp: None,
        })
    }

    /// Build the device tree for the AArch64 virt machine: RAM, one CPU,
    /// GICv3, the architected timer, PSCI over HVC, and a PL011 console.
    fn build_dtb(
        cmdline: &str,
        ram_base: u64,
        ram_size: u64,
        initrd: Option<(u64, u64)>,
    ) -> Vec<u8> {
        use crate::arch::fdt::FdtBuilder;

        const PHANDLE_GIC: u32 = 1;
        const PHANDLE_CLK: u32 = 2;
        // Device tree interrupt cells: <type intid flags>.
        const GIC_SPI: u32 = 0;
        const GIC_PPI: u32 = 1;
        const IRQ_LEVEL_HI: u32 = 4;

        let mut fdt = FdtBuilder::new();
        fdt.begin_node("");
        fdt.prop_str("compatible", "linux,dummy-virt");
        fdt.prop_u32("#address-cells", 2);
        fdt.prop_u32("#size-cells", 2);
        fdt.prop_u32("interrupt-parent", PHANDLE_GIC);

        fdt.begin_node("chosen");
        fdt.prop_str("bootargs", cmdline);
        fdt.prop_str("stdout-path", &format!("/pl011@{AARCH64_UART_BASE:x}"));
        if let Some((start, end)) = initrd {
            fdt.prop_u64("linux,initrd-start", start);
            fdt.prop_u64("linux,initrd-end", end);
        }
        fdt.end_node();

        fdt.begin_node(&format!("memory@{ram_base:x}"));
        fdt.prop_str("device_type", "memory");
        fdt.prop_cells(
            "reg",
            &[
                (ram_base >> 32) as u32,
                ram_base as u32,
                (ram_size >> 32) as u32,
                ram_size as u32,
            ],
        );
        fdt.end_node();

        fdt.begin_node("cpus");
        fdt.prop_u32("#address-cells", 1);
        fdt.prop_u32("#size-cells", 0);
        fdt.begin_node("cpu@0");
        fdt.prop_str("device_type", "cpu");
        fdt.prop_str("compatible", "arm,arm-v8");
        fdt.prop_u32("reg", 0);
        fdt.end_node();
        fdt.end_node();

        fdt.begin_node("psci");
        fdt.prop_str_list("compatible", &["arm,psci-1.0", "arm,psci-0.2"]);
        fdt.prop_str("method", "hvc");
        fdt.end_node();

        fdt.begin_node(&format!("intc@{AARCH64_GICD_BASE:x}"));
        fdt.prop_str("compatible", "arm,gic-v3");
        fdt.prop_u32("#interrupt-cells", 3);
        fdt.prop_empty("interrupt-controller");
        fdt.prop_cells(
            "reg",
            &[
                (AARCH64_GICD_BASE >> 32) as u32,
                AARCH64_GICD_BASE as u32,
                (GICD_SIZE >> 32) as u32,
                GICD_SIZE as u32,
                (AARCH64_GICR_BASE >> 32) as u32,
                AARCH64_GICR_BASE as u32,
                (GICR_SIZE_PER_CPU >> 32) as u32,
                GICR_SIZE_PER_CPU as u32,
            ],
        );
        fdt.prop_u32("phandle", PHANDLE_GIC);
        fdt.end_node();

        fdt.begin_node("timer");
        fdt.prop_str("compatible", "arm,armv8-timer");
        fdt.prop_cells(
            "interrupts",
            &[
                GIC_PPI, 13, IRQ_LEVEL_HI, // secure physical
                GIC_PPI, 14, IRQ_LEVEL_HI, // non-secure physical
                GIC_PPI, 11, IRQ_LEVEL_HI, // virtual
                GIC_PPI, 10, IRQ_LEVEL_HI, // hypervisor
            ],
        );
        fdt.prop_empty("always-on");
        fdt.end_node();

        fdt.begin_node("apb-pclk");
        fdt.prop_str("compatible", "fixed-clock");
        fdt.prop_u32("#clock-cells", 0);
        fdt.prop_u32("clock-frequency", 24_000_000);
        fdt.prop_str("clock-output-names", "clk24mhz");
        fdt.prop_u32("phandle", PHANDLE_CLK);
        fdt.end_node();

        fdt.begin_node(&format!("pl011@{AARCH64_UART_BASE:x}"));
        fdt.prop_str_list("compatible", &["arm,pl011", "arm,primecell"]);
        fdt.prop_cells(
            "reg",
            &[
                (AARCH64_UART_BASE >> 32) as u32,
                AARCH64_UART_BASE as u32,
                0,
                0x1000,
            ],
        );
        fdt.prop_cells(
            "interrupts",
            &[GIC_SPI, AARCH64_UART_IRQ - 32, IRQ_LEVEL_HI],
        );
        fdt.prop_cells("clocks", &[PHANDLE_CLK, PHANDLE_CLK]);
        fdt.prop_str_list("clock-names", &["uartclk", "apb_pclk"]);
        fdt.end_node();

        fdt.end_node();
        fdt.finish()
    }
}

impl Arch for Aarch64Arch {
    fn name(&self) -> &'static str {
        "aarch64"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        // ARM uses MMIO for everything, no port I/O
        // TODO: Register PL011 UART, GIC, etc.
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        Some(AARCH64_UART_BASE)
    }

    fn serial_irq(&self) -> Option<u32> {
        Some(AARCH64_UART_IRQ) // SPI 1 (first SPI is 32)
    }

    fn ram_base(&self) -> u64 {
        AARCH64_RAM_BASE
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        let mut file = File::open(&config.kernel)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf.len() < 4 {
            return Err(Error::KernelLoad("image is too small".to_string()));
        }

        let mut info = if buf.starts_with(b"\x7fELF") {
            Self::load_elf(mem, &buf)?
        } else {
            Self::load_raw(mem, &buf)?
        };

        let ram_size = config.memory.bytes();
        let ram_end = AARCH64_RAM_BASE + ram_size;
        const ALIGN_2M: u64 = 0x20_0000;
        let align_2m = |addr: u64| (addr + ALIGN_2M - 1) & !(ALIGN_2M - 1);

        // Place the initrd (optional) and DTB above the kernel footprint.
        let mut next = align_2m(info.load_addr + info.image_size);

        let initrd_range = match &config.initrd {
            Some(path) => {
                let mut initrd = Vec::new();
                File::open(path)?.read_to_end(&mut initrd)?;
                let start = next;
                let end = start + initrd.len() as u64;
                if end > ram_end {
                    return Err(Error::KernelLoad(format!(
                        "initrd ({} bytes) does not fit in guest RAM",
                        initrd.len()
                    )));
                }
                mem.write_slice(&initrd, GuestAddress(start))?;
                next = align_2m(end);
                Some((start, end))
            }
            None => None,
        };

        let dtb = Self::build_dtb(&config.cmdline, AARCH64_RAM_BASE, ram_size, initrd_range);
        let dtb_addr = next;
        if dtb_addr + dtb.len() as u64 > ram_end {
            return Err(Error::KernelLoad(
                "no room for device tree in guest RAM".to_string(),
            ));
        }
        mem.write_slice(&dtb, GuestAddress(dtb_addr))?;

        tracing::info!(
            entry = format!("{:#x}", info.entry_point),
            image_size = info.image_size,
            dtb_addr = format!("{:#x}", dtb_addr),
            dtb_size = dtb.len(),
            initrd = ?initrd_range.map(|(s, e)| format!("{s:#x}..{e:#x}")),
            "AArch64 boot layout"
        );

        info.dtb_addr = Some(dtb_addr);
        info.initial_sp = Some(ram_end & !0xF);
        Ok(BootInfo::Arm(info))
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        // TODO: Initialize KVM for ARM
        Err(Error::InvalidConfig(
            "KVM for AArch64 not yet implemented".to_string(),
        ))
    }

    fn initial_cpu_state(&self, _mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        use crate::cpu::{Aarch64Registers, Aarch64SystemRegisters};

        let boot = match boot {
            BootInfo::Arm(info) => info,
            _ => return Err(Error::InvalidConfig("expected ARM boot info".to_string())),
        };

        // ARM64 boot protocol: PC at the image entry, x0 = DTB physical
        // address, x1-x3 zero, MMU off, all interrupts masked at EL1h (the
        // default PSTATE is 0x3C5). The kernel sets up its own stack.
        let mut regs = Aarch64Registers::default();
        regs.pc = boot.entry_point;
        regs.sp = boot.initial_sp.unwrap_or(0) & !0xF;
        regs.x[0] = boot.dtb_addr.unwrap_or(0);

        let mut sregs = Aarch64SystemRegisters::default();
        sregs.sctlr_el1 = 0; // MMU off, caches off initially

        Ok(CpuState::aarch64(regs, sregs))
    }
}

// =============================================================================
// ARMv7-A Architecture
// =============================================================================

/// Default UART base address for ARMv7-A platforms
const ARMV7A_UART_BASE: u64 = 0x1010_0000; // Common for Versatile/RealView

pub struct Armv7aArch;

impl Armv7aArch {
    pub fn new() -> Self {
        Armv7aArch
    }
}

/// Samsung S3C64xx physical RAM base (SMDK6410: 128MB at 0x50000000) — the
/// platform the 32-bit ARMv6 machine models.
pub const ARMV7A_RAM_BASE: u64 = 0x5000_0000;
/// Kernel Image load offset (TEXT_OFFSET 0x8000 for 32-bit ARM).
pub const ARMV7A_TEXT_OFFSET: u64 = 0x8000;
/// Where the DTB is placed.
pub const ARMV7A_DTB_ADDR: u64 = 0x5300_0000;
/// Where the initrd/initramfs is placed.
pub const ARMV7A_INITRD_ADDR: u64 = 0x5400_0000;

// S5L8900 (iPod Touch 1G) firmware load addresses.
const S5L_IBOOT_BASE: u64 = 0x1800_0000;
const S5L_VROM_BASE: u64 = 0x2000_0000;
const S5L_LLB_BASE: u64 = 0x2200_0000;
const S5L_NOR_BASE: u64 = 0x2400_0000;
const S5L_ENGINE_8900_BASE: u32 = 0x3F00_0000;

/// Load the S5L8900 firmware images and apply the bootrom-call patches the
/// QEMU iPod Touch reference uses, then boot iBoot directly.
///
/// `config.kernel` points at the iBoot binary (`iboot_204_n45ap.bin`); the
/// bootrom (`bootrom_s5l8900`) and NOR (`nor_n45ap.bin`) are discovered as
/// siblings in the same directory.
fn load_s5l8900_firmware(mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
    let iboot_path = &config.kernel;
    let dir = iboot_path
        .parent()
        .ok_or_else(|| Error::InvalidConfig("iboot path has no parent directory".to_string()))?;

    let read = |p: &std::path::Path| -> Result<Vec<u8>> {
        let mut v = Vec::new();
        File::open(p)?.read_to_end(&mut v)?;
        Ok(v)
    };

    let iboot = read(iboot_path)?;
    mem.write_slice(&iboot, GuestAddress(S5L_IBOOT_BASE))?;

    // Bootrom (VROM). Optional — iBoot is entered directly, but the bootrom
    // function table is patched so iBoot's image-decrypt calls land on our
    // routines.
    let bootrom_path = dir.join("bootrom_s5l8900");
    if bootrom_path.exists() {
        let bootrom = read(&bootrom_path)?;
        mem.write_slice(&bootrom, GuestAddress(S5L_VROM_BASE))?;
    }

    // NOR flash (holds the device tree / NVRAM / boot images iBoot reads).
    let nor_path = dir.join("nor_n45ap.bin");
    if nor_path.exists() {
        let nor = read(&nor_path)?;
        mem.write_slice(&nor, GuestAddress(S5L_NOR_BASE))?;
    }

    let w32 = |addr: u64, val: u32| -> Result<()> {
        mem.write_slice(&val.to_le_bytes(), GuestAddress(addr))?;
        Ok(())
    };

    // Patch the bootrom function-pointer table to point at our LLB routines.
    w32(0x2000_008c, (S5L_LLB_BASE + 0x80) as u32)?;
    w32(0x2000_0090, (S5L_LLB_BASE + 0x100) as u32)?;

    // LLB+0x80: MOVS R0, #1 ; BX LR  (8900 signature check → success).
    w32(S5L_LLB_BASE + 0x80, 0xe3b0_0001)?;
    w32(S5L_LLB_BASE + 0x84, 0xe12f_ff1e)?;

    // LLB+0x100: LDR r1,[pc,#0x100] ; STR r0,[r1] ; MOVS R0,#1 ; BX lr
    // (writes the image address to the 8900 engine MMIO, triggering decrypt).
    w32(S5L_LLB_BASE + 0x100, 0xe59f_1100)?;
    w32(S5L_LLB_BASE + 0x104, 0xe581_0000)?;
    w32(S5L_LLB_BASE + 0x108, 0xe3b0_0001)?;
    w32(S5L_LLB_BASE + 0x10c, 0xe12f_ff1e)?;

    // LLB+0x208: the 8900 engine base address loaded by the routine above.
    w32(S5L_LLB_BASE + 0x208, S5L_ENGINE_8900_BASE)?;

    // Optional: force the boot path. This iBoot, loaded directly (bypassing
    // the bootrom→LLB chain), drops to its USB-DFU recovery console instead of
    // auto-booting. Redirect the console-entry call (`BL 0x1800a7b8` at
    // 0x1800079a) to `do_fsboot` (0x18004300) so it reads the kernelcache from
    // NAND. Gated by RAX_S5L_FORCE_FSBOOT.
    if std::env::var("RAX_S5L_FORCE_FSBOOT").is_ok() {
        // Thumb BL 0x1800079a -> 0x18004300 : hw1=0xf003, hw2=0xfdb1.
        mem.write_slice(&[0x03, 0xf0, 0xb1, 0xfd], GuestAddress(0x1800_079a))?;
    }

    tracing::info!(
        entry = format!("{:#x}", S5L_IBOOT_BASE),
        iboot = %iboot_path.display(),
        iboot_size = iboot.len(),
        bootrom = bootrom_path.exists(),
        nor = nor_path.exists(),
        "S5L8900 iBoot boot layout"
    );

    Ok(BootInfo::Arm(ArmBootInfo {
        entry_point: S5L_IBOOT_BASE,
        load_addr: S5L_IBOOT_BASE,
        image_size: iboot.len() as u64,
        dtb_addr: None,
        initial_sp: None,
    }))
}

/// Insert `linux,initrd-start`/`linux,initrd-end` into a flattened device
/// tree's `/chosen` node (creating the node if absent), rebuilding the blob.
fn fdt_set_initrd(dtb: &[u8], start: u32, end: u32) -> Result<Vec<u8>> {
    let be32 = |off: usize| -> u32 { u32::from_be_bytes(dtb[off..off + 4].try_into().unwrap()) };
    if dtb.len() < 40 || be32(0) != 0xD00D_FEED {
        return Err(Error::KernelLoad("invalid device tree blob".to_string()));
    }
    let off_struct = be32(8) as usize;
    let off_strings = be32(12) as usize;
    let size_strings = be32(32) as usize;
    let size_struct = be32(36) as usize;

    let structure = &dtb[off_struct..off_struct + size_struct];
    let strings = &dtb[off_strings..off_strings + size_strings];

    // Locate `/chosen`: depth-1 BEGIN_NODE named "chosen". Token stream:
    // BEGIN_NODE(1) name..pad4 | END_NODE(2) | PROP(3) len nameoff data..pad4
    // | NOP(4) | END(9).
    let tok = |p: usize| -> u32 {
        u32::from_be_bytes(structure[p..p + 4].try_into().unwrap())
    };
    let mut p = 0usize;
    let mut depth = 0usize;
    let mut chosen_body: Option<usize> = None; // offset right after the node name
    let mut root_body: Option<usize> = None;
    while p + 4 <= structure.len() {
        match tok(p) {
            1 => {
                let name_start = p + 4;
                let name_end = name_start
                    + structure[name_start..]
                        .iter()
                        .position(|&b| b == 0)
                        .unwrap_or(0);
                let name = &structure[name_start..name_end];
                p = (name_end + 1 + 3) & !3;
                if depth == 0 {
                    root_body = Some(p);
                } else if depth == 1 && name == b"chosen" {
                    chosen_body = Some(p);
                    break;
                }
                depth += 1;
            }
            2 => {
                depth = depth.saturating_sub(1);
                p += 4;
            }
            3 => {
                let len = tok(p + 4) as usize;
                p = (p + 12 + len + 3) & !3;
            }
            4 => p += 4,
            _ => break, // END
        }
    }

    // Property name strings appended to the strings block.
    let mut new_strings = strings.to_vec();
    let start_nameoff = new_strings.len() as u32;
    new_strings.extend_from_slice(b"linux,initrd-start\0");
    let end_nameoff = new_strings.len() as u32;
    new_strings.extend_from_slice(b"linux,initrd-end\0");

    let mut props = Vec::new();
    for (nameoff, value) in [(start_nameoff, start), (end_nameoff, end)] {
        props.extend_from_slice(&3u32.to_be_bytes());
        props.extend_from_slice(&4u32.to_be_bytes());
        props.extend_from_slice(&nameoff.to_be_bytes());
        props.extend_from_slice(&value.to_be_bytes());
    }

    let mut new_struct = Vec::with_capacity(size_struct + 96);
    match (chosen_body, root_body) {
        (Some(at), _) => {
            new_struct.extend_from_slice(&structure[..at]);
            new_struct.extend_from_slice(&props);
            new_struct.extend_from_slice(&structure[at..]);
        }
        (None, Some(at)) => {
            new_struct.extend_from_slice(&structure[..at]);
            new_struct.extend_from_slice(&1u32.to_be_bytes());
            new_struct.extend_from_slice(b"chosen\0\0"); // name + pad to 4
            new_struct.extend_from_slice(&props);
            new_struct.extend_from_slice(&2u32.to_be_bytes());
            new_struct.extend_from_slice(&structure[at..]);
        }
        _ => {
            return Err(Error::KernelLoad(
                "device tree has no root node".to_string(),
            ));
        }
    }

    // Reassemble: header | (everything between header and struct, i.e. the
    // memory reservation map) | struct | strings.
    let mut out = dtb[..off_struct].to_vec();
    out.extend_from_slice(&new_struct);
    let new_off_strings = out.len();
    out.extend_from_slice(&new_strings);
    let total = out.len();
    out[4..8].copy_from_slice(&(total as u32).to_be_bytes());
    out[12..16].copy_from_slice(&(new_off_strings as u32).to_be_bytes());
    out[32..36].copy_from_slice(&(new_strings.len() as u32).to_be_bytes());
    out[36..40].copy_from_slice(&(new_struct.len() as u32).to_be_bytes());
    Ok(out)
}

impl Arch for Armv7aArch {
    fn name(&self) -> &'static str {
        "armv7a"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        // Samsung S3C64xx UART0 (served inside the vCPU's memory bridge).
        Some(0x7F00_5000)
    }

    fn ram_base(&self) -> u64 {
        ARMV7A_RAM_BASE
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        let mut file = File::open(&config.kernel)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf.len() < 4 {
            return Err(Error::KernelLoad("image is too small".to_string()));
        }

        // S5L8900 (iPod Touch 1G) firmware machine: load bootrom/iBoot/NOR and
        // boot iBoot directly, instead of the Linux DT boot protocol.
        if std::env::var("RAX_MACHINE").as_deref() == Ok("s5l8900") {
            return load_s5l8900_firmware(mem, config);
        }

        // 32-bit ARM boot protocol (DT): uncompressed Image at RAM base +
        // TEXT_OFFSET, device tree above it, r2 = DTB physical address.
        let load_addr = ARMV7A_RAM_BASE + ARMV7A_TEXT_OFFSET;
        mem.write_slice(&buf, GuestAddress(load_addr))?;

        // The device tree: --dtb on the command line, with a fallback to a
        // `dtbs/` directory next to the kernel image.
        let dtb_path = match &config.arm_dtb {
            Some(p) => p.clone(),
            None => {
                let fallback = config
                    .kernel
                    .parent()
                    .map(|d| d.join("dtbs/s3c6410-smdk6410.dtb"))
                    .filter(|p| p.exists());
                fallback.ok_or_else(|| {
                    Error::InvalidConfig(
                        "armv7a DT boot requires --dtb <file> (no dtbs/s3c6410-smdk6410.dtb \
                         found next to the kernel)"
                            .to_string(),
                    )
                })?
            }
        };
        let mut dtb = Vec::new();
        File::open(&dtb_path)?.read_to_end(&mut dtb)?;

        // Optional initramfs: loaded high in RAM and advertised through the
        // /chosen node so the kernel picks it up.
        let mut initrd_len = 0usize;
        if let Some(initrd_path) = &config.initrd {
            let mut initrd = Vec::new();
            File::open(initrd_path)?.read_to_end(&mut initrd)?;
            initrd_len = initrd.len();
            mem.write_slice(&initrd, GuestAddress(ARMV7A_INITRD_ADDR))?;
            dtb = fdt_set_initrd(
                &dtb,
                ARMV7A_INITRD_ADDR as u32,
                (ARMV7A_INITRD_ADDR as usize + initrd_len) as u32,
            )?;
        }
        mem.write_slice(&dtb, GuestAddress(ARMV7A_DTB_ADDR))?;

        tracing::info!(
            entry = format!("{:#x}", load_addr),
            image_size = buf.len(),
            dtb = %dtb_path.display(),
            dtb_addr = format!("{:#x}", ARMV7A_DTB_ADDR),
            initrd_len,
            "ARMv6/ARMv7-A boot layout"
        );

        let info = ArmBootInfo {
            entry_point: load_addr,
            load_addr,
            image_size: buf.len() as u64,
            dtb_addr: Some(ARMV7A_DTB_ADDR),
            initial_sp: None,
        };

        Ok(BootInfo::Arm(info))
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        Err(Error::InvalidConfig(
            "KVM for ARMv7-A not supported".to_string(),
        ))
    }

    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        use crate::cpu::{Aarch32Registers, Aarch32SystemRegisters};

        let boot = match boot {
            BootInfo::Arm(info) => info,
            _ => return Err(Error::InvalidConfig("expected ARM boot info".to_string())),
        };

        let mut regs = Aarch32Registers::default();
        regs.pc = boot.entry_point as u32;

        // Set up initial stack at end of memory
        let mem_end = mem.last_addr().raw_value().saturating_add(1);
        let sp = ((mem_end - 16) & !0x7) as u32; // 8-byte aligned
        regs.sp = sp;

        // R0 = 0 (unused), R1 = machine type, R2 = DTB address
        regs.r[0] = 0;
        regs.r[1] = 0xFFFF_FFFF; // Machine type (0xFFFFFFFF = use DTB)
        regs.r[2] = boot.dtb_addr.unwrap_or(0) as u32;

        // Supervisor mode, IRQ/FIQ masked, ARM state.
        regs.cpsr = 0xD3;

        let sregs = Aarch32SystemRegisters::default();

        Ok(CpuState::aarch32(regs, sregs))
    }
}

// =============================================================================
// ARMv8-A AArch32 Mode
// =============================================================================

pub struct Armv8a32Arch;

impl Armv8a32Arch {
    pub fn new() -> Self {
        Armv8a32Arch
    }
}

impl Arch for Armv8a32Arch {
    fn name(&self) -> &'static str {
        "armv8a32"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        Some(ARMV7A_UART_BASE)
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        // Same as ARMv7-A for now
        Armv7aArch::new().load_kernel(mem, config)
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        Err(Error::InvalidConfig(
            "KVM for ARMv8-A AArch32 not supported".to_string(),
        ))
    }

    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        // Same as ARMv7-A for now
        Armv7aArch::new().initial_cpu_state(mem, boot)
    }
}

// =============================================================================
// Cortex-M Architecture
// =============================================================================

/// Default UART base for Cortex-M (varies by vendor, using ARM MPS2 as example)
const CORTEXM_UART_BASE: u64 = 0x4000_4000;
/// Default vector table address
const CORTEXM_VTOR_DEFAULT: u32 = 0x0000_0000;

pub struct CortexMArch;

impl CortexMArch {
    pub fn new() -> Self {
        CortexMArch
    }
}

impl Arch for CortexMArch {
    fn name(&self) -> &'static str {
        "cortex-m"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        Some(CORTEXM_UART_BASE)
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        let mut file = File::open(&config.kernel)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf.len() < 8 {
            return Err(Error::KernelLoad("image is too small".to_string()));
        }

        // Cortex-M vector table starts at load address
        // First word is initial SP, second word is reset handler
        let load_addr = 0u64;
        mem.write_slice(&buf, GuestAddress(load_addr))?;

        // Read initial SP and reset handler from vector table
        let initial_sp = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let reset_handler = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);

        let info = ArmBootInfo {
            entry_point: reset_handler as u64,
            load_addr,
            image_size: buf.len() as u64,
            dtb_addr: None,
            initial_sp: Some(initial_sp as u64),
        };

        Ok(BootInfo::Arm(info))
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        Err(Error::InvalidConfig(
            "KVM for Cortex-M not supported".to_string(),
        ))
    }

    fn initial_cpu_state(&self, _mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        use crate::cpu::{CortexMRegisters, CortexMSystemRegisters};

        let boot = match boot {
            BootInfo::Arm(info) => info,
            _ => return Err(Error::InvalidConfig("expected ARM boot info".to_string())),
        };

        let mut regs = CortexMRegisters::default();
        // PC must have LSB set for Thumb mode (Cortex-M is always Thumb)
        regs.pc = (boot.entry_point as u32) & !1;
        regs.msp = boot.initial_sp.unwrap_or(0x2000_0000) as u32;
        regs.xpsr = 0x0100_0000; // Thumb bit set

        let mut sregs = CortexMSystemRegisters::default();
        sregs.vtor = boot.load_addr as u32;

        Ok(CpuState::cortex_m(regs, sregs))
    }
}

// =============================================================================
// Cortex-R Architecture
// =============================================================================

pub struct CortexRArch;

impl CortexRArch {
    pub fn new() -> Self {
        CortexRArch
    }
}

impl Arch for CortexRArch {
    fn name(&self) -> &'static str {
        "cortex-r"
    }

    fn setup_devices(&self, _io_bus: &mut IoBus, _mmio_bus: &mut MmioBus) -> Result<()> {
        Ok(())
    }

    fn serial_mmio_base(&self) -> Option<u64> {
        Some(0x1010_0000) // Typical for Cortex-R platforms
    }

    fn load_kernel(&self, mem: &GuestMemoryMmap, config: &VmConfig) -> Result<BootInfo> {
        // Similar to ARMv7-A
        Armv7aArch::new().load_kernel(mem, config)
    }

    #[cfg(all(feature = "kvm", target_os = "linux"))]
    fn init_vm(&self, _vm: &crate::backend::kvm::KvmVm, _boot: &BootInfo) -> Result<()> {
        Err(Error::InvalidConfig(
            "KVM for Cortex-R not supported".to_string(),
        ))
    }

    fn initial_cpu_state(&self, mem: &GuestMemoryMmap, boot: &BootInfo) -> Result<CpuState> {
        // Cortex-R uses AArch32 state
        Armv7aArch::new().initial_cpu_state(mem, boot)
    }
}
