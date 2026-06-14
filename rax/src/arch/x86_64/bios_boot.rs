//! El-Torito (BIOS CD) boot support for legacy/real-mode boot — e.g. TempleOS.
//!
//! rax's Linux path loads an ELF kernel straight into long mode. An El-Torito
//! bootable ISO instead expects the BIOS to load a "no-emulation" boot image at
//! `0x7C00` and start it in 16-bit real mode (`CS:IP = 0:0x7C00`, `DL` = boot
//! drive). This module parses the El-Torito boot catalog and extracts the boot
//! image + real-mode entry parameters; the VM then runs it in real mode with a
//! minimal BIOS (INT 10h/13h) servicing the bootloader's requests.

use std::sync::Mutex;

use crate::cpu::{DescriptorTable, Registers, Segment, SystemRegisters};
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

/// Armed real-mode CPU state for an El-Torito ISO boot, consumed once by the
/// arch's `initial_cpu_state`. Set by [`arm_real_mode_boot`]. A static keeps the
/// legacy-boot plumbing out of the (single-vCPU) boot path's data structures.
static REAL_MODE_STATE: Mutex<Option<(SystemRegisters, Registers)>> = Mutex::new(None);

/// Cheap probe: does this look like an ISO-9660 image? (The primary volume
/// descriptor's "CD001" identifier lives at offset 0x8001.)
pub fn is_iso_image_header(magic_at_0x8001: &[u8]) -> bool {
    magic_at_0x8001.len() >= 5 && &magic_at_0x8001[..5] == b"CD001"
}

/// Parse + load an El-Torito ISO's boot image into `mem`, install its CD image
/// for the mini-BIOS (INT 13h), record the guest RAM size for INT 15h memory
/// detection, and arm the 16-bit real-mode CPU state to be returned by
/// [`armed_real_mode_state`]. `mem_bytes` is the *reported* RAM size (what the
/// guest should see), not the padded allocation.
pub fn arm_real_mode_boot(
    mem: &GuestMemoryMmap,
    iso: Vec<u8>,
    mem_bytes: u64,
) -> Result<(), String> {
    let boot = setup_real_mode_boot(mem, iso)?;
    crate::backend::emulator::x86_64::bios::install_cd(std::sync::Arc::new(boot.iso));
    crate::backend::emulator::x86_64::bios::install_mem(mem_bytes);
    *REAL_MODE_STATE.lock().unwrap() = Some((boot.sregs, boot.regs));
    Ok(())
}

/// The armed real-mode CPU state (sregs, regs), if an ISO boot was set up.
pub fn armed_real_mode_state() -> Option<(SystemRegisters, Registers)> {
    REAL_MODE_STATE.lock().unwrap().clone()
}

/// CD-ROM logical sector size (ISO-9660 / El-Torito LBAs are in these units).
pub const CD_SECTOR: usize = 2048;

/// Boot drive number reported to the bootloader in DL. El-Torito CD boot uses a
/// BIOS-assigned drive number; 0xE0 is a common choice for an emulated CD and is
/// what our mini-BIOS keys INT 13h CD reads on.
pub const BOOT_DRIVE_CD: u8 = 0xE0;

/// Fully prepared real-mode boot: the loaded image is in guest memory, `sregs`
/// and `regs` start the bootloader, and `iso` is retained so the mini-BIOS can
/// serve INT 13h CD reads from it.
pub struct RealModeBoot {
    pub sregs: SystemRegisters,
    pub regs: Registers,
    pub iso: Vec<u8>,
    pub boot_drive: u8,
}

/// Parse an El-Torito ISO, load its no-emulation boot image into guest memory at
/// its load address, and produce the 16-bit real-mode CPU state to start it
/// (`CS:IP = 0:load_addr`, `DL` = boot drive, CR0.PE=0, no paging). The ISO bytes
/// are retained for the mini-BIOS to serve `INT 13h` reads.
pub fn setup_real_mode_boot(mem: &GuestMemoryMmap, iso: Vec<u8>) -> Result<RealModeBoot, String> {
    let boot = parse_el_torito(&iso)?;
    if boot.media_type != 0 {
        return Err(format!(
            "unsupported El-Torito media type {} (only no-emulation is supported)",
            boot.media_type
        ));
    }
    mem.write_slice(&boot.boot_image, GuestAddress(boot.load_addr as u64))
        .map_err(|e| format!("loading boot image at {:#x}: {e:?}", boot.load_addr))?;

    let sregs = real_mode_sregs();
    let mut regs = Registers::default();
    // CS.base = 0, so CS:IP = 0:load_addr — the linear entry is load_addr (0x7C00).
    regs.rip = boot.load_addr as u64;
    regs.rdx = BOOT_DRIVE_CD as u64; // DL = boot drive
    regs.rflags = 0x2; // reserved bit set; IF=0 (the bootloader STIs itself)

    Ok(RealModeBoot {
        sregs,
        regs,
        iso,
        boot_drive: BOOT_DRIVE_CD,
    })
}

/// Initial 16-bit real-mode segment/control register state: CR0.PE=0, no paging,
/// EFER=0; every segment base 0, limit 0xFFFF, 16-bit; IDT = real-mode IVT.
fn real_mode_sregs() -> SystemRegisters {
    let code = Segment {
        base: 0,
        limit: 0xFFFF,
        selector: 0,
        type_: 0x0B, // execute/read, accessed
        present: true,
        dpl: 0,
        db: false, // 16-bit
        s: true,
        l: false,
        g: false,
        avl: false,
        unusable: false,
    };
    let data = Segment {
        type_: 0x03, // read/write, accessed
        ..code.clone()
    };
    SystemRegisters {
        cs: code,
        ds: data.clone(),
        es: data.clone(),
        fs: data.clone(),
        gs: data.clone(),
        ss: data,
        tr: Segment::default(),
        ldt: Segment::default(),
        gdt: DescriptorTable {
            base: 0,
            limit: 0xFFFF,
        },
        idt: DescriptorTable {
            base: 0,
            limit: 0x3FF, // real-mode IVT: 256 vectors * 4 bytes
        },
        cr0: 0,
        cr2: 0,
        cr3: 0,
        cr4: 0,
        cr8: 0,
        efer: 0,
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
        dr6: 0xFFFF_0FF0,
        dr7: 0x0000_0400,
    }
}

/// Parsed El-Torito initial (default) boot entry plus the extracted boot image.
#[derive(Debug, Clone)]
pub struct ElToritoBoot {
    /// Media emulation type: 0 = no emulation (the only mode we support).
    pub media_type: u8,
    /// Real-mode load segment. A catalog value of 0 means the BIOS default
    /// `0x07C0`, i.e. load at linear `0x7C00`.
    pub load_segment: u16,
    /// Linear load address (`load_segment << 4`).
    pub load_addr: u32,
    /// Number of 512-byte virtual sectors the BIOS loads from the image.
    pub sector_count: u16,
    /// CD LBA (2048-byte sectors) of the boot image within the ISO.
    pub image_lba: u32,
    /// The boot image bytes (`sector_count * 512`), to be placed at `load_addr`.
    pub boot_image: Vec<u8>,
}

/// Parse the El-Torito boot catalog of an ISO-9660 image and extract the
/// initial (default) no-emulation boot image.
pub fn parse_el_torito(iso: &[u8]) -> Result<ElToritoBoot, String> {
    // Boot Record Volume Descriptor lives at ISO sector 17 (offset 0x8800):
    //   [0]      = descriptor type 0 (boot record)
    //   [1..6]   = "CD001"
    //   [6]      = version 1
    //   [7..0x1F]= "EL TORITO SPECIFICATION"
    //   [0x47..] = absolute pointer (LE u32) to the boot catalog sector.
    let brvd = 17 * CD_SECTOR;
    if iso.len() < brvd + 0x4B {
        return Err("ISO too small for an El-Torito boot record".into());
    }
    if &iso[brvd + 1..brvd + 6] != b"CD001" {
        return Err("no CD001 at sector 17 (not a boot-record volume descriptor)".into());
    }
    if &iso[brvd + 7..brvd + 7 + 23] != b"EL TORITO SPECIFICATION" {
        return Err("sector 17 is not an El-Torito boot record".into());
    }
    let cat_sector = u32::from_le_bytes(iso[brvd + 0x47..brvd + 0x4B].try_into().unwrap());
    let cat = cat_sector as usize * CD_SECTOR;
    if iso.len() < cat + 64 {
        return Err("El-Torito boot catalog is out of range".into());
    }

    // Validation entry (first 32 bytes): [0]=header id 1, [0x1E..0x20]=0x55 0xAA.
    if iso[cat] != 0x01 {
        return Err(format!(
            "bad El-Torito validation header id 0x{:02x}",
            iso[cat]
        ));
    }
    if iso[cat + 0x1E] != 0x55 || iso[cat + 0x1F] != 0xAA {
        return Err("bad El-Torito validation entry signature (0x55AA)".into());
    }

    // Initial/default entry at catalog + 32 (32 bytes):
    //   [0]      = boot indicator (0x88 = bootable)
    //   [1]      = media type (0 = no emulation)
    //   [2..4]   = load segment (LE, 0 => default 0x07C0)
    //   [6..8]   = sector count (LE, 512-byte virtual sectors)
    //   [8..12]  = image LBA (LE, 2048-byte CD sectors)
    let e = cat + 32;
    if iso[e] != 0x88 {
        return Err(format!(
            "initial boot entry not bootable (0x{:02x})",
            iso[e]
        ));
    }
    let media_type = iso[e + 1];
    let raw_seg = u16::from_le_bytes([iso[e + 2], iso[e + 3]]);
    let load_segment = if raw_seg == 0 { 0x07C0 } else { raw_seg };
    let sector_count = u16::from_le_bytes([iso[e + 6], iso[e + 7]]);
    let image_lba = u32::from_le_bytes(iso[e + 8..e + 12].try_into().unwrap());
    let load_addr = (load_segment as u32) << 4;

    let img_off = image_lba as usize * CD_SECTOR;
    let img_len = sector_count as usize * 512;
    if iso.len() < img_off + img_len {
        return Err("El-Torito boot image is out of range".into());
    }
    let boot_image = iso[img_off..img_off + img_len].to_vec();

    Ok(ElToritoBoot {
        media_type,
        load_segment,
        load_addr,
        sector_count,
        image_lba,
        boot_image,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal in-memory ISO carrying an El-Torito no-emulation boot
    /// entry that mirrors TempleOS's catalog (boot image starts `fc b8`, like
    /// TempleOS's `cld; mov ax,imm16`).
    fn synthetic_iso() -> Vec<u8> {
        let cat_sector = 0x14u32; // boot catalog at LBA 0x14
        let image_lba = 0x15u32; // boot image at LBA 0x15
        let sector_count = 4u16; // 4 * 512 = 2048 bytes
        let mut iso = vec![0u8; (image_lba as usize + 1) * CD_SECTOR];

        let brvd = 17 * CD_SECTOR;
        iso[brvd] = 0; // boot record
        iso[brvd + 1..brvd + 6].copy_from_slice(b"CD001");
        iso[brvd + 6] = 1;
        iso[brvd + 7..brvd + 7 + 23].copy_from_slice(b"EL TORITO SPECIFICATION");
        iso[brvd + 0x47..brvd + 0x4B].copy_from_slice(&cat_sector.to_le_bytes());

        let cat = cat_sector as usize * CD_SECTOR;
        iso[cat] = 0x01;
        iso[cat + 0x1E] = 0x55;
        iso[cat + 0x1F] = 0xAA;
        let e = cat + 32;
        iso[e] = 0x88; // bootable
        iso[e + 1] = 0x00; // no emulation
        iso[e + 2..e + 4].copy_from_slice(&0u16.to_le_bytes()); // load seg 0 => 0x7C0
        iso[e + 6..e + 8].copy_from_slice(&sector_count.to_le_bytes());
        iso[e + 8..e + 12].copy_from_slice(&image_lba.to_le_bytes());

        let img = image_lba as usize * CD_SECTOR;
        iso[img] = 0xFC; // cld
        iso[img + 1] = 0xB8; // mov ax, imm16
        iso
    }

    #[test]
    fn parse_no_emulation_boot_image() {
        let iso = synthetic_iso();
        let boot = parse_el_torito(&iso).expect("parse El-Torito");
        assert_eq!(boot.media_type, 0);
        assert_eq!(boot.load_segment, 0x07C0);
        assert_eq!(boot.load_addr, 0x7C00);
        assert_eq!(boot.sector_count, 4);
        assert_eq!(boot.image_lba, 0x15);
        assert_eq!(boot.boot_image.len(), 2048);
        assert_eq!(&boot.boot_image[..2], &[0xFC, 0xB8]);
    }

    #[test]
    fn rejects_non_boot_iso() {
        let iso = vec![0u8; 20 * CD_SECTOR];
        assert!(parse_el_torito(&iso).is_err());
    }

    #[test]
    fn real_mode_setup_loads_image_and_state() {
        let iso = synthetic_iso();
        let mem =
            GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 0x20_0000)]).expect("guest mem");
        let boot = setup_real_mode_boot(&mem, iso).expect("setup real-mode boot");

        // Boot image landed at 0x7C00.
        let mut b = [0u8; 2];
        mem.read_slice(&mut b, GuestAddress(0x7C00)).unwrap();
        assert_eq!(b, [0xFC, 0xB8]);

        // 16-bit real mode: PE off, no paging, 16-bit CS, segments base 0.
        assert_eq!(boot.sregs.cr0 & 1, 0, "CR0.PE must be 0 (real mode)");
        assert_eq!(boot.sregs.efer, 0, "EFER must be 0 (no long mode)");
        assert!(!boot.sregs.cs.db, "CS must be 16-bit");
        assert!(!boot.sregs.cs.l, "CS must not be 64-bit");
        assert_eq!(boot.sregs.cs.base, 0);
        assert_eq!(boot.sregs.cs.limit, 0xFFFF);
        assert_eq!(boot.sregs.idt.limit, 0x3FF, "real-mode IVT");

        // Entry: CS:IP = 0:0x7C00, DL = boot drive.
        assert_eq!(boot.regs.rip, 0x7C00);
        assert_eq!(boot.regs.rdx as u8, BOOT_DRIVE_CD);
    }
}
