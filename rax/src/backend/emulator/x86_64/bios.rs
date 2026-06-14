//! Minimal real-mode BIOS for legacy/El-Torito boot (e.g. TempleOS).
//!
//! rax has no firmware: the Linux path enters long mode directly. A CD boot
//! image instead runs in 16-bit real mode and calls BIOS services via `INT`.
//! This module services the small set the TempleOS bootloader needs — INT 10h
//! (video/teletype) and INT 13h (extended LBA disk read from the boot CD) —
//! natively, reading sectors out of the retained ISO image. It is consulted
//! from the `INT imm8` handler only when running in real mode with a boot CD
//! installed via [`install_cd`].
//!
//! The CD image is held in a module-level static because legacy boot is
//! inherently single-vCPU (the bootloader runs before any SMP bring-up).

use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::error::Result;

use super::cpu::X86_64Vcpu;

/// CD-ROM logical sector size used by INT 13h extended reads.
const CD_SECTOR: usize = 2048;

static BIOS_CD: Mutex<Option<Arc<Vec<u8>>>> = Mutex::new(None);

/// Total guest RAM in bytes, reported via the INT 15h memory-detection calls
/// (E820/E801/88h). Set by [`install_mem`] at boot. 0 means "unknown".
static BIOS_MEM_BYTES: AtomicU64 = AtomicU64::new(0);

/// Install the boot CD image (the El-Torito ISO) so INT 13h reads can serve it.
/// Passing this enables the real-mode mini-BIOS.
pub fn install_cd(iso: Arc<Vec<u8>>) {
    *BIOS_CD.lock().unwrap() = Some(iso);
}

/// Record the total guest RAM size so INT 15h memory detection can report it.
pub fn install_mem(bytes: u64) {
    BIOS_MEM_BYTES.store(bytes, Ordering::Relaxed);
}

/// The E820 memory map for `total` bytes of RAM: standard low-memory layout
/// (conventional + EBDA/ROM holes) plus one usable extended region. Each entry
/// is (base, length, type) where type 1 = usable, 2 = reserved.
fn e820_map(total: u64) -> Vec<(u64, u64, u32)> {
    let mut v = vec![
        (0x0000_0000, 0x0009_FC00, 1), // 639 KiB conventional, usable
        (0x0009_FC00, 0x0000_0400, 2), // EBDA, reserved
        (0x000E_0000, 0x0002_0000, 2), // BIOS ROM area, reserved
    ];
    if total > 0x10_0000 {
        v.push((0x10_0000, total - 0x10_0000, 1)); // extended memory, usable
    }
    v
}

/// Whether a boot CD (and thus the mini-BIOS) is installed.
pub fn active() -> bool {
    BIOS_CD.lock().unwrap().is_some()
}

fn cd() -> Option<Arc<Vec<u8>>> {
    BIOS_CD.lock().unwrap().clone()
}

/// The installed boot CD image, if any (shared with the IDE/ATAPI controller so
/// the same ISO that INT 13h serves is also the CD-ROM medium).
pub fn installed_cd() -> Option<Arc<Vec<u8>>> {
    BIOS_CD.lock().unwrap().clone()
}

#[inline]
fn set_cf(vcpu: &mut X86_64Vcpu, set: bool) {
    if set {
        vcpu.regs.rflags |= 1;
    } else {
        vcpu.regs.rflags &= !1;
    }
}

#[inline]
fn set_ah(vcpu: &mut X86_64Vcpu, ah: u8) {
    vcpu.regs.rax = (vcpu.regs.rax & !0xFF00) | ((ah as u64) << 8);
}

#[inline]
fn ah(vcpu: &X86_64Vcpu) -> u8 {
    (vcpu.regs.rax >> 8) as u8
}

/// Service a real-mode BIOS software interrupt. Returns `Ok(true)` if the vector
/// was handled (the caller has already advanced past the `INT` instruction).
pub fn service(vcpu: &mut X86_64Vcpu, vector: u8) -> Result<bool> {
    match vector {
        0x10 => {
            int10(vcpu);
            Ok(true)
        }
        0x13 => {
            int13(vcpu)?;
            Ok(true)
        }
        0x15 => {
            int15(vcpu);
            Ok(true)
        }
        0x16 => {
            int16(vcpu);
            Ok(true)
        }
        0x1A => {
            int1a(vcpu);
            Ok(true)
        }
        // Equipment list / base memory size — harmless stubs.
        0x11 => {
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | 0x0021; // basic equipment
            Ok(true)
        }
        0x12 => {
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | 640; // 640 KiB base memory
            Ok(true)
        }
        _ => {
            if std::env::var_os("RAX_RM_TRACE").is_some() {
                eprintln!("[INT-UNH] vec={vector:#x} ax={:#06x}", vcpu.regs.rax as u16);
            }
            Ok(false)
        }
    }
}

/// INT 1Ah — time / RTC / PCI-BIOS.
fn int1a(vcpu: &mut X86_64Vcpu) {
    if std::env::var_os("RAX_RM_TRACE").is_some() {
        eprintln!("[INT1A] ax={:#06x}", vcpu.regs.rax as u16);
    }
    match ah(vcpu) {
        // AH=00h: read system-timer tick count → CX:DX = ticks, AL = midnight flag.
        0x00 => {
            vcpu.regs.rcx &= !0xFFFF;
            vcpu.regs.rdx &= !0xFFFF;
            vcpu.regs.rax &= !0xFF;
            set_cf(vcpu, false);
        }
        // AH=02h: read RTC time (BCD CH=hr CL=min DH=sec) → midnight.
        0x02 => {
            vcpu.regs.rcx &= !0xFFFF;
            vcpu.regs.rdx &= !0xFF00;
            set_cf(vcpu, false);
        }
        // AH=04h: read RTC date (BCD CH=century CL=year DH=month DL=day).
        0x04 => {
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | 0x2026;
            vcpu.regs.rdx = (vcpu.regs.rdx & !0xFFFF) | 0x0101;
            set_cf(vcpu, false);
        }
        // AH=B1h: PCI BIOS — report not installed; the OS falls back to direct
        // PCI config I/O (ports 0xCF8/0xCFC).
        0xB1 => {
            set_ah(vcpu, 0xFF);
            set_cf(vcpu, true);
        }
        _ => set_cf(vcpu, true),
    }
}

/// INT 10h — video services (the subset bootloaders use).
fn int10(vcpu: &mut X86_64Vcpu) {
    match ah(vcpu) {
        // AH=0Eh: teletype output — write AL to the console.
        0x0E => {
            let ch = vcpu.regs.rax as u8;
            let mut out = std::io::stdout().lock();
            let _ = out.write_all(&[ch]);
            let _ = out.flush();
        }
        // AH=0Fh: get video mode → AL=mode (3 = 80x25 text), AH=columns (80),
        // BH=active page (0).
        0x0F => {
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | 0x5003; // AH=0x50(80), AL=3
            vcpu.regs.rbx &= !0xFF00; // BH=0
        }
        // AH=00h (set mode), 02h (set cursor), 03h (get cursor), 01h, 05h, 06h,
        // 07h, 09h/0Ah (write char): accept as no-ops so progress output flows.
        0x03 => {
            // get cursor position/size → report (0,0), default shape.
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | 0x0607;
            vcpu.regs.rdx &= !0xFFFF;
        }
        _ => {}
    }
}

/// INT 13h — disk services. Only the El-Torito extended-read path is meaningful;
/// it reads 2048-byte CD sectors out of the boot ISO.
fn int13(vcpu: &mut X86_64Vcpu) -> Result<()> {
    match ah(vcpu) {
        // AH=00h: reset disk system.
        0x00 => {
            set_ah(vcpu, 0);
            set_cf(vcpu, false);
        }
        // AH=41h: check extensions present → CF=0, BX=0xAA55, AH=0x21 (EDD v2.1),
        // CX bit0 = fixed-disk extended access supported.
        0x41 => {
            vcpu.regs.rbx = (vcpu.regs.rbx & !0xFFFF) | 0xAA55;
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | 0x0001;
            set_ah(vcpu, 0x21);
            set_cf(vcpu, false);
        }
        // AH=42h: extended read using a Disk Address Packet at DS:SI.
        0x42 => {
            let dap = vcpu.sregs.ds.base.wrapping_add(vcpu.regs.rsi & 0xFFFF);
            let count = vcpu.mmu.read_u16(dap + 2, &vcpu.sregs)? as usize;
            let buf_off = vcpu.mmu.read_u16(dap + 4, &vcpu.sregs)? as u64;
            let buf_seg = vcpu.mmu.read_u16(dap + 6, &vcpu.sregs)? as u64;
            let lba = vcpu.mmu.read_u64(dap + 8, &vcpu.sregs)?;
            let buf_lin = (buf_seg << 4).wrapping_add(buf_off);

            let cd = match cd() {
                Some(c) => c,
                None => {
                    set_ah(vcpu, 0x01);
                    set_cf(vcpu, true);
                    return Ok(());
                }
            };
            let start = (lba as usize).saturating_mul(CD_SECTOR);
            let len = count * CD_SECTOR;
            if start.saturating_add(len) > cd.len() {
                // Read past end of medium → error (AH=0x04 sector not found).
                set_ah(vcpu, 0x04);
                set_cf(vcpu, true);
                return Ok(());
            }
            if std::env::var_os("RAX_RM_TRACE").is_some() {
                eprintln!(
                    "[INT13] read lba={lba} count={count} buf={buf_seg:#x}:{buf_off:#x}={buf_lin:#x} ({len}B)"
                );
            }
            vcpu.write_bytes(buf_lin, &cd[start..start + len])?;
            set_ah(vcpu, 0);
            set_cf(vcpu, false);
        }
        // AH=08h: get drive parameters (minimal success).
        0x08 => {
            set_ah(vcpu, 0);
            set_cf(vcpu, false);
        }
        // Unknown function: report failure.
        _ => {
            set_ah(vcpu, 0x01);
            set_cf(vcpu, true);
        }
    }
    Ok(())
}

/// INT 15h — miscellaneous system services. Reported unsupported for now
/// (callers fall back to defaults); a real E820 map can be added when needed.
fn int15(vcpu: &mut X86_64Vcpu) {
    if std::env::var_os("RAX_RM_TRACE").is_some() {
        eprintln!(
            "[INT15] eax={:#x} ebx={:#x} ecx={:#x} edx={:#x} es:di={:#x}:{:#x}",
            vcpu.regs.rax as u32,
            vcpu.regs.rbx as u32,
            vcpu.regs.rcx as u32,
            vcpu.regs.rdx as u32,
            vcpu.sregs.es.selector,
            vcpu.regs.rdi as u16
        );
    }
    let total = BIOS_MEM_BYTES.load(Ordering::Relaxed);
    let eax = vcpu.regs.rax as u32;
    match eax {
        // EAX=0xE820: query system address map (one E820 entry per call). EBX is
        // the continuation/index (0 to start); the call returning the final entry
        // sets EBX=0. EDX must be the 'SMAP' signature; ES:DI points at a >=20-byte
        // buffer that receives (base u64, length u64, type u32).
        0xE820 if total != 0 && (vcpu.regs.rdx as u32) == 0x534D_4150 => {
            let map = e820_map(total);
            let idx = (vcpu.regs.rbx as u32) as usize;
            if idx >= map.len() {
                // Past the last entry → error/done.
                set_cf(vcpu, true);
                return;
            }
            let (base, len, typ) = map[idx];
            let dst = vcpu.sregs.es.base.wrapping_add(vcpu.regs.rdi & 0xFFFF);
            let mut buf = [0u8; 20];
            buf[0..8].copy_from_slice(&base.to_le_bytes());
            buf[8..16].copy_from_slice(&len.to_le_bytes());
            buf[16..20].copy_from_slice(&typ.to_le_bytes());
            if vcpu.write_bytes(dst, &buf).is_err() {
                set_cf(vcpu, true);
                return;
            }
            // EAX = 'SMAP', ECX = bytes written, EBX = next index (0 if last).
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF_FFFF) | 0x534D_4150;
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF_FFFF) | 20;
            let next = if idx + 1 < map.len() {
                (idx + 1) as u64
            } else {
                0
            };
            vcpu.regs.rbx = (vcpu.regs.rbx & !0xFFFF_FFFF) | next;
            set_cf(vcpu, false);
        }
        // AX=0xE801: extended memory size. AX=CX = KiB in the 1–16 MiB window
        // (capped at 0x3C00 = 15 MiB), BX=DX = number of 64 KiB blocks above
        // 16 MiB. Both register pairs carry the same values per the ABI.
        0xE801 if total != 0 => {
            let kib_1_16 = ((total.min(0x100_0000).saturating_sub(0x10_0000)) / 1024) as u16;
            let blocks_above_16 = (total.saturating_sub(0x100_0000) / 0x1_0000) as u16;
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | kib_1_16 as u64;
            vcpu.regs.rcx = (vcpu.regs.rcx & !0xFFFF) | kib_1_16 as u64;
            vcpu.regs.rbx = (vcpu.regs.rbx & !0xFFFF) | blocks_above_16 as u64;
            vcpu.regs.rdx = (vcpu.regs.rdx & !0xFFFF) | blocks_above_16 as u64;
            set_cf(vcpu, false);
        }
        // AH=0x88: extended memory size in KiB above 1 MiB (legacy; capped at
        // 0xFFFF = 64 MiB - 1 KiB).
        _ if (eax >> 8) as u8 == 0x88 && total != 0 => {
            let kib = (total.saturating_sub(0x10_0000) / 1024).min(0xFFFF) as u16;
            vcpu.regs.rax = (vcpu.regs.rax & !0xFFFF) | kib as u64;
            set_cf(vcpu, false);
        }
        _ => {
            set_ah(vcpu, 0x86); // unsupported function
            set_cf(vcpu, true);
        }
    }
}

/// INT 16h — keyboard services. Report "no key available".
fn int16(vcpu: &mut X86_64Vcpu) {
    match ah(vcpu) {
        // AH=01h: check for keystroke → ZF=1 (none). ZF is bit 6 of RFLAGS.
        0x01 => {
            vcpu.regs.rflags |= 1 << 6;
        }
        // AH=00h: read keystroke → return 0 (no key); avoids blocking the boot.
        _ => {
            vcpu.regs.rax &= !0xFFFF;
        }
    }
}
