//! S5L8900 (iPod Touch 1G / iPhone 1) vCPU: boots Apple's iBoot on the
//! software ARMv6 emulator.
//!
//! Drives the AArch32 [`Armv7Cpu`] + [`Executor`] over guest memory with the
//! S5L8900 platform device set (clock, timer, PL192 VIC pair, SYSIC, GPIO,
//! chip-ID, UART, 8900 crypto engine) served at their physical MMIO windows.
//! Firmware (bootrom, iBoot, NOR) and the bootrom-call patches are placed in
//! guest RAM by the arch boot setup; the CPU resets to IBOOT_BASE.

use std::cell::RefCell;
use std::sync::{Arc, Mutex, OnceLock};

use tracing::debug;
use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use crate::arm::execution::{ArmMemory, MemoryError};
use crate::arm::mmu_v6::{self, V6Access, V6Fault, V6MmuConfig};
use crate::arm::{
    Armv7Cpu, Decoder, ExceptionType, ExecResult, ExecutionState, Executor, ProcessorMode,
};
use crate::cpu::{
    Aarch32CpuState, Aarch32Registers, Aarch32SystemRegisters, CpuState, VCpu, VcpuExit,
};
use crate::devices::s3c64xx::S3cUart;
use crate::devices::crypto::{aes_cbc_decrypt, sha1, AesKey};
use crate::devices::s5l8900::{
    AesKeyType, Pl192, S5lAes, S5lChipId, S5lClock, S5lDmac, S5lGpio, S5lI2c, S5lLcd, S5lNand,
    S5lNandEcc, S5lSpi, S5lSysic, S5lTimer, AES_UID_KEY, NAND_BYTES_PER_SPARE,
};
use crate::error::{Error, Result};

// Device MMIO base addresses (see the QEMU iPod Touch 1G reference).
const CLOCK0_BASE: u32 = 0x3810_0000;
const CLOCK1_BASE: u32 = 0x3C50_0000;
const VIC0_BASE: u32 = 0x38E0_0000;
const VIC1_BASE: u32 = 0x38E0_1000;
const SYSIC_BASE: u32 = 0x39A0_0000;
const TIMER1_BASE: u32 = 0x3E20_0000;
const GPIO_BASE: u32 = 0x3E40_0000;
const CHIPID_BASE: u32 = 0x3E50_0000;
const UART0_BASE: u32 = 0x3CC0_0000;
const I2C0_BASE: u32 = 0x3C60_0000;
const I2C1_BASE: u32 = 0x3C90_0000;
const SPI0_BASE: u32 = 0x3C30_0000;
const SPI1_BASE: u32 = 0x3CE0_0000;
const SPI2_BASE: u32 = 0x3D20_0000;
const LCD_BASE: u32 = 0x3890_0000;
const NAND_BASE: u32 = 0x38A0_0000;
const NAND_ECC_BASE: u32 = 0x38F0_0000;
const ADM_BASE: u32 = 0x3880_0000;
const DMAC0_BASE: u32 = 0x3820_0000;
const ENGINE_8900_BASE: u32 = 0x3F00_0000;
const AES_BASE: u32 = 0x38C0_0000;

const IBOOT_BASE: u32 = 0x1800_0000;
const LLB_BASE: u32 = 0x2200_0000;

// IRQ line numbers on VIC0.
const TIMER1_IRQ: u32 = 0x7;
const SPI0_IRQ: u32 = 0x9;
const SPI1_IRQ: u32 = 0xA;
const SPI2_IRQ: u32 = 0xB;
const DMAC0_IRQ: u32 = 0x10;
/// NAND ECC engine: global IRQ 0x2B, i.e. VIC1 line (0x2B - 32) = 11.
const NAND_ECC_VIC1_LINE: u32 = 0x2B - 32;
const LCD_IRQ: u32 = 0xD;
const UART0_IRQ: u32 = 24;

/// Instructions per `run()` batch before yielding to the VMM loop.
const BATCH: u32 = 65_536;

/// Default guest-time speedup for the µs timer: firmware delays elapse this
/// many times faster than real wall-clock time (keeps multi-second boot waits
/// short). Overridable with RAX_S5L_TIMER_SPEEDUP. Stability of the guest's
/// atomic counter read is unaffected — that depends only on the update cadence
/// (every 256 instructions), not the magnitude of each step.
fn timer_speedup() -> u64 {
    std::env::var("RAX_S5L_TIMER_SPEEDUP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(256)
}

/// Parsed-once write-watchpoint range (addr, len) from RAX_S5L_WATCH=hex:hex.
fn watch_range() -> &'static Option<(u32, u32)> {
    static WATCH: std::sync::OnceLock<Option<(u32, u32)>> = std::sync::OnceLock::new();
    WATCH.get_or_init(|| {
        let v = std::env::var("RAX_S5L_WATCH").ok()?;
        let (a, l) = v.split_once(':')?;
        let addr = u32::from_str_radix(a.trim_start_matches("0x"), 16).ok()?;
        let len = u32::from_str_radix(l.trim_start_matches("0x"), 16).ok()?;
        Some((addr, len))
    })
}

/// The directory holding the NAND `bank<n>/<page>.page` dumps. From
/// RAX_S5L_NAND, else the iPod Touch reference dump next to the firmware.
fn nand_dir() -> Option<std::path::PathBuf> {
    if let Ok(p) = std::env::var("RAX_S5L_NAND") {
        return Some(std::path::PathBuf::from(p));
    }
    let default = std::path::PathBuf::from("docs/hardware/apple/iPodTouch1/nand");
    default.is_dir().then_some(default)
}

/// Last memory fault recorded by the bridge (for DFSR/DFAR reporting).
#[derive(Clone, Copy, Default)]
struct LastFault {
    addr: u32,
    fsr: u32,
    domain: u32,
    access: u32,
}

/// Mutable bridge internals (RefCell: the executor's read path is `&self`).
struct BridgeInner {
    mmu: V6MmuConfig,
    privileged: bool,
    /// PC of the instruction currently executing, for the write-watchpoint log.
    cur_pc: u32,
    last_fault: LastFault,
    clock0: S5lClock,
    clock1: S5lClock,
    vic0: Pl192,
    vic1: Pl192,
    sysic: S5lSysic,
    gpio: S5lGpio,
    timer: S5lTimer,
    i2c0: S5lI2c,
    i2c1: S5lI2c,
    spi0: S5lSpi,
    spi1: S5lSpi,
    spi2: S5lSpi,
    lcd: S5lLcd,
    nand: S5lNand,
    nand_ecc: S5lNandEcc,
    aes: S5lAes,
    dmac0: S5lDmac,
    /// ADM (Apple Data Mover) DMA data-section addresses.
    adm_data2: u32,
    adm_data3: u32,
    /// Pending ADM operation (ctrl value written), serviced by the vCPU step.
    adm_req: Option<u32>,
    uart: Arc<OnceLock<Arc<Mutex<S3cUart>>>>,
    /// Pending 8900-engine decryption request: the physical address of the
    /// image header written to the engine MMIO. Serviced by the vCPU step.
    engine_8900_req: Option<u32>,
    /// Budget-limited log of accesses to unmapped MMIO windows.
    openbus_log_budget: u32,
    /// Log all handled device reads too (RAX_S5L_DEVLOG).
    devlog: bool,
}

/// Memory bridge: VA→PA via the v6 MMU, S5L8900 device windows at their
/// physical addresses, everything else through the flat guest memory (which
/// backs RAM, iBoot, VROM, LLB and NOR).
struct S5lBridge {
    mem: Arc<GuestMemoryMmap>,
    inner: RefCell<BridgeInner>,
}

impl S5lBridge {
    fn translate(
        mem: &GuestMemoryMmap,
        inner: &mut BridgeInner,
        va: u32,
        access: V6Access,
    ) -> std::result::Result<u32, MemoryError> {
        let walk = |pa: u32| -> Option<u32> {
            let mut buf = [0u8; 4];
            mem.read_slice(&mut buf, GuestAddress(pa as u64)).ok()?;
            Some(u32::from_le_bytes(buf))
        };
        match mmu_v6::translate_v6(&inner.mmu, va, inner.privileged, access, walk) {
            Ok(t) => Ok(t.pa),
            Err(V6Fault { fsr, domain }) => {
                inner.last_fault = LastFault {
                    addr: va,
                    fsr,
                    domain,
                    access: match access {
                        V6Access::Read => 0,
                        V6Access::Write => 1,
                        V6Access::Execute => 2,
                    } | if inner.privileged { 4 } else { 0 },
                };
                Err(MemoryError::PermissionDenied(va))
            }
        }
    }
}

impl BridgeInner {
    /// Device-window read at a PHYSICAL address. Returns None when the address
    /// is plain memory (RAM/firmware) and should fall through to the mmap.
    fn dev_read(&mut self, pa: u32) -> Option<u32> {
        let off = |base: u32| pa - base;
        match pa {
            _ if (CLOCK0_BASE..CLOCK0_BASE + 0x1000).contains(&pa) => {
                Some(self.clock0.read(off(CLOCK0_BASE)))
            }
            _ if (CLOCK1_BASE..CLOCK1_BASE + 0x1000).contains(&pa) => {
                Some(self.clock1.read(off(CLOCK1_BASE)))
            }
            _ if (VIC0_BASE..VIC0_BASE + 0x1000).contains(&pa) => {
                Some(self.vic0.read(off(VIC0_BASE)))
            }
            _ if (VIC1_BASE..VIC1_BASE + 0x1000).contains(&pa) => {
                Some(self.vic1.read(off(VIC1_BASE)))
            }
            _ if (SYSIC_BASE..SYSIC_BASE + 0x1000).contains(&pa) => {
                Some(self.sysic.read(off(SYSIC_BASE)))
            }
            _ if (TIMER1_BASE..TIMER1_BASE + 0x10004).contains(&pa) => {
                Some(self.timer.read(off(TIMER1_BASE)))
            }
            _ if (GPIO_BASE..GPIO_BASE + 0x10000).contains(&pa) => {
                Some(self.gpio.read(off(GPIO_BASE)))
            }
            _ if (CHIPID_BASE..CHIPID_BASE + 0x10).contains(&pa) => {
                Some(S5lChipId::read(off(CHIPID_BASE)))
            }
            _ if (I2C0_BASE..I2C0_BASE + 0x1000).contains(&pa) => {
                Some(self.i2c0.read(off(I2C0_BASE)))
            }
            _ if (I2C1_BASE..I2C1_BASE + 0x1000).contains(&pa) => {
                Some(self.i2c1.read(off(I2C1_BASE)))
            }
            _ if (SPI0_BASE..SPI0_BASE + 0x100).contains(&pa) => {
                Some(self.spi0.read(off(SPI0_BASE)))
            }
            _ if (SPI1_BASE..SPI1_BASE + 0x100).contains(&pa) => {
                Some(self.spi1.read(off(SPI1_BASE)))
            }
            _ if (SPI2_BASE..SPI2_BASE + 0x100).contains(&pa) => {
                Some(self.spi2.read(off(SPI2_BASE)))
            }
            _ if (LCD_BASE..LCD_BASE + 0x1000).contains(&pa) => {
                Some(self.lcd.read(off(LCD_BASE)))
            }
            _ if (NAND_BASE..NAND_BASE + 0x1000).contains(&pa) => {
                Some(self.nand.read(off(NAND_BASE)))
            }
            _ if (NAND_ECC_BASE..NAND_ECC_BASE + 0x1000).contains(&pa) => {
                Some(self.nand_ecc.read(off(NAND_ECC_BASE)))
            }
            _ if (DMAC0_BASE..DMAC0_BASE + 0x1000).contains(&pa) => {
                Some(self.dmac0.read(off(DMAC0_BASE)))
            }
            _ if (ADM_BASE..ADM_BASE + 0x1000).contains(&pa) => Some(match off(ADM_BASE) {
                0x0 => 0x2,  // CTRL: device ready
                0x4 => 0x10, // CTRL2: upload event finished
                _ => 0,
            }),
            _ if (UART0_BASE..UART0_BASE + 0x1000).contains(&pa) => Some(
                self.uart
                    .get()
                    .and_then(|u| u.lock().ok().map(|mut u| u.read(off(UART0_BASE))))
                    .unwrap_or(0),
            ),
            _ if (ENGINE_8900_BASE..ENGINE_8900_BASE + 0x100).contains(&pa) => Some(0),
            _ if (AES_BASE..AES_BASE + 0x100).contains(&pa) => Some(self.aes.read(off(AES_BASE))),
            // Plain memory.
            _ if is_memory(pa) => None,
            // Unmapped MMIO: open bus (read as zero).
            _ => {
                if self.openbus_log_budget > 0 {
                    self.openbus_log_budget -= 1;
                    debug!(pa = format!("{pa:#x}"), "openbus read");
                }
                Some(0)
            }
        }
    }

    /// Device-window write. Returns true if the write was consumed by a device
    /// (or open bus); false if it should fall through to plain memory.
    fn dev_write(&mut self, pa: u32, value: u32) -> bool {
        let off = |base: u32| pa - base;
        match pa {
            _ if (CLOCK0_BASE..CLOCK0_BASE + 0x1000).contains(&pa) => {
                self.clock0.write(off(CLOCK0_BASE), value);
                true
            }
            _ if (CLOCK1_BASE..CLOCK1_BASE + 0x1000).contains(&pa) => {
                self.clock1.write(off(CLOCK1_BASE), value);
                true
            }
            _ if (VIC0_BASE..VIC0_BASE + 0x1000).contains(&pa) => {
                self.vic0.write(off(VIC0_BASE), value);
                true
            }
            _ if (VIC1_BASE..VIC1_BASE + 0x1000).contains(&pa) => {
                self.vic1.write(off(VIC1_BASE), value);
                true
            }
            _ if (SYSIC_BASE..SYSIC_BASE + 0x1000).contains(&pa) => {
                self.sysic.write(off(SYSIC_BASE), value);
                true
            }
            _ if (TIMER1_BASE..TIMER1_BASE + 0x10004).contains(&pa) => {
                self.timer.write(off(TIMER1_BASE), value);
                true
            }
            _ if (GPIO_BASE..GPIO_BASE + 0x10000).contains(&pa) => {
                self.gpio.write(off(GPIO_BASE), value);
                true
            }
            _ if (CHIPID_BASE..CHIPID_BASE + 0x10).contains(&pa) => true,
            _ if (I2C0_BASE..I2C0_BASE + 0x1000).contains(&pa) => {
                self.i2c0.write(off(I2C0_BASE), value);
                true
            }
            _ if (I2C1_BASE..I2C1_BASE + 0x1000).contains(&pa) => {
                self.i2c1.write(off(I2C1_BASE), value);
                true
            }
            _ if (SPI0_BASE..SPI0_BASE + 0x100).contains(&pa) => {
                self.spi0.write(off(SPI0_BASE), value);
                true
            }
            _ if (SPI1_BASE..SPI1_BASE + 0x100).contains(&pa) => {
                self.spi1.write(off(SPI1_BASE), value);
                true
            }
            _ if (SPI2_BASE..SPI2_BASE + 0x100).contains(&pa) => {
                self.spi2.write(off(SPI2_BASE), value);
                true
            }
            _ if (LCD_BASE..LCD_BASE + 0x1000).contains(&pa) => {
                self.lcd.write(off(LCD_BASE), value);
                true
            }
            _ if (NAND_BASE..NAND_BASE + 0x1000).contains(&pa) => {
                self.nand.write(off(NAND_BASE), value);
                true
            }
            _ if (NAND_ECC_BASE..NAND_ECC_BASE + 0x1000).contains(&pa) => {
                self.nand_ecc.write(off(NAND_ECC_BASE), value);
                true
            }
            _ if (DMAC0_BASE..DMAC0_BASE + 0x1000).contains(&pa) => {
                self.dmac0.write(off(DMAC0_BASE), value);
                true
            }
            _ if (ADM_BASE..ADM_BASE + 0x1000).contains(&pa) => {
                match off(ADM_BASE) {
                    0x88 => self.adm_data2 = value,
                    0x8C => self.adm_data3 = value,
                    0x0 | 0x4 => self.adm_req = Some((off(ADM_BASE) << 28) | (value & 0xFF)),
                    _ => {}
                }
                true
            }
            _ if (UART0_BASE..UART0_BASE + 0x1000).contains(&pa) => {
                if let Some(u) = self.uart.get() {
                    if let Ok(mut u) = u.lock() {
                        u.write(off(UART0_BASE), value);
                    }
                }
                true
            }
            _ if (ENGINE_8900_BASE..ENGINE_8900_BASE + 0x100).contains(&pa) => {
                // Writing the image header address to offset 0 triggers
                // in-place AES-CBC decryption (serviced by the vCPU step).
                if off(ENGINE_8900_BASE) == 0 {
                    self.engine_8900_req = Some(value);
                }
                true
            }
            _ if (AES_BASE..AES_BASE + 0x100).contains(&pa) => {
                self.aes.write(off(AES_BASE), value);
                true
            }
            _ if is_memory(pa) => false,
            _ => {
                if self.openbus_log_budget > 0 {
                    self.openbus_log_budget -= 1;
                    debug!(pa = format!("{pa:#x}"), val = format!("{value:#x}"), "openbus write");
                }
                true
            }
        }
    }

    fn read_pa(
        &mut self,
        mem: &GuestMemoryMmap,
        pa: u32,
        buf: &mut [u8],
    ) -> std::result::Result<(), MemoryError> {
        if buf.len() <= 4 && !is_memory(pa) {
            if let Some(v) = self.dev_read(pa & !0x3) {
                if self.openbus_log_budget > 0 && self.devlog {
                    self.openbus_log_budget -= 1;
                    debug!(pa = format!("{pa:#x}"), val = format!("{v:#x}"), "dev read");
                }
                let lane = (pa & 0x3) as usize;
                let bytes = v.to_le_bytes();
                for (i, b) in buf.iter_mut().enumerate() {
                    *b = *bytes.get(lane + i).unwrap_or(&0);
                }
                return Ok(());
            }
        }
        mem.read_slice(buf, GuestAddress(pa as u64))
            .map_err(|_| MemoryError::BusError(pa))
    }

    fn write_pa(
        &mut self,
        mem: &GuestMemoryMmap,
        pa: u32,
        data: &[u8],
    ) -> std::result::Result<(), MemoryError> {
        // Optional write-watchpoint (RAX_S5L_WATCH=hexaddr:hexlen): log writes
        // that land in [addr, addr+len) so heap/free-list corruption can be
        // traced to the offending store.
        if let Some((wa, wl)) = *watch_range() {
            if pa >= wa && pa < wa.wrapping_add(wl) {
                let mut v = [0u8; 4];
                v[..data.len().min(4)].copy_from_slice(&data[..data.len().min(4)]);
                debug!(
                    pa = format!("{pa:#x}"),
                    val = format!("{:#x}", u32::from_le_bytes(v)),
                    pc = format!("{:#x}", self.cur_pc),
                    "watch write"
                );
            }
        }
        if data.len() <= 4 && !is_memory(pa) {
            let reg = pa & !0x3;
            let lane = (pa & 0x3) as usize;
            let mut cur = self.dev_read(reg).unwrap_or(0).to_le_bytes();
            for (i, b) in data.iter().enumerate() {
                if lane + i < 4 {
                    cur[lane + i] = *b;
                }
            }
            if self.dev_write(reg, u32::from_le_bytes(cur)) {
                // A channel-enable write to DMAC0 schedules a transfer; run it
                // now (here we have the guest-memory handle the transfer needs).
                if let Some(ch) = self.dmac0.take_pending() {
                    self.dma_run(mem, ch as usize);
                }
                return Ok(());
            }
        }
        mem.write_slice(data, GuestAddress(pa as u64))
            .map_err(|_| MemoryError::BusError(pa))
    }

    /// Perform a PL080 channel transfer synchronously. iBoot's use is a
    /// peripheral-to-memory stream (NAND FIFO at a fixed source address into an
    /// incrementing RAM buffer); we also handle memory-to-memory. The transfer
    /// honours the source/destination increment flags and the transfer width.
    fn dma_run(&mut self, mem: &GuestMemoryMmap, ch: usize) {
        let src = self.dmac0.src[ch];
        let dst = self.dmac0.dst[ch];
        let ctl = self.dmac0.control[ch];
        let count = ctl & 0xFFF; // number of transfers
        let swidth = 1u32 << ((ctl >> 18) & 7).min(2); // src transfer width (bytes)
        let s_inc = (ctl >> 26) & 1 != 0;
        let d_inc = (ctl >> 27) & 1 != 0;
        let total = (count * swidth) as usize; // total bytes to move
        if total == 0 {
            self.dmac0.complete(ch);
            return;
        }

        // Gather the source bytes.
        let mut buf = vec![0u8; total];
        if !s_inc && !is_memory(src) {
            // Peripheral FIFO (e.g. NAND data register): drain it word-by-word.
            let mut i = 0;
            while i < total {
                let mut word = [0u8; 4];
                let _ = self.read_pa(mem, src, &mut word);
                let n = (total - i).min(4);
                buf[i..i + n].copy_from_slice(&word[..n]);
                i += 4;
            }
        } else {
            for (i, b) in buf.iter_mut().enumerate() {
                let a = if s_inc { src.wrapping_add(i as u32) } else { src };
                let mut byte = [0u8; 1];
                let _ = self.read_pa(mem, a, &mut byte);
                *b = byte[0];
            }
        }

        // Scatter to the destination.
        if d_inc && is_memory(dst) {
            let _ = mem.write_slice(&buf, GuestAddress(dst as u64));
        } else {
            for (i, b) in buf.iter().enumerate() {
                let a = if d_inc { dst.wrapping_add(i as u32) } else { dst };
                let _ = self.write_pa(mem, a, &[*b]);
            }
        }

        debug!(
            ch = ch,
            src = format!("{src:#x}"),
            dst = format!("{dst:#x}"),
            bytes = total,
            "dma_run"
        );
        if std::env::var("RAX_S5L_DMADUMP").is_ok() {
            if let Ok(path) = std::env::var("RAX_S5L_DMADUMP") {
                use std::io::Write;
                if let Ok(mut f) =
                    std::fs::OpenOptions::new().create(true).append(true).open(&path)
                {
                    let _ = writeln!(f, "--- ch{ch} src={src:#x} dst={dst:#x} bytes={total}");
                    let _ = f.write_all(&buf);
                    let _ = writeln!(f);
                }
            }
        }
        self.dmac0.complete(ch);
    }
}

/// Whether a physical address is plain memory (RAM/firmware/NOR), as opposed
/// to a device MMIO window. The S5L8900 devices all live in 0x38000000..
/// 0x40000000; everything below 0x38000000 and the NOR window are memory.
fn is_memory(pa: u32) -> bool {
    !(0x3800_0000..0x4000_0000).contains(&pa)
}

impl S5lBridge {
    fn access(
        &self,
        addr: u32,
        access: V6Access,
        data: Option<&[u8]>,
        out: Option<&mut [u8]>,
    ) -> std::result::Result<(), MemoryError> {
        let mut inner = self.inner.borrow_mut();
        let pa = Self::translate(&self.mem, &mut inner, addr, access)?;
        if let Some(buf) = out {
            inner.read_pa(&self.mem, pa, buf)
        } else if let Some(d) = data {
            inner.write_pa(&self.mem, pa, d)
        } else {
            Ok(())
        }
    }
}

impl ArmMemory for S5lBridge {
    fn read_word(&self, addr: u32) -> std::result::Result<u32, MemoryError> {
        let mut b = [0u8; 4];
        self.access(addr, V6Access::Read, None, Some(&mut b))?;
        Ok(u32::from_le_bytes(b))
    }

    fn read_halfword(&self, addr: u32) -> std::result::Result<u16, MemoryError> {
        let mut b = [0u8; 2];
        self.access(addr, V6Access::Read, None, Some(&mut b))?;
        Ok(u16::from_le_bytes(b))
    }

    fn read_byte(&self, addr: u32) -> std::result::Result<u8, MemoryError> {
        let mut b = [0u8; 1];
        self.access(addr, V6Access::Read, None, Some(&mut b))?;
        Ok(b[0])
    }

    fn write_word(&mut self, addr: u32, value: u32) -> std::result::Result<(), MemoryError> {
        self.access(addr, V6Access::Write, Some(&value.to_le_bytes()), None)
    }

    fn write_halfword(&mut self, addr: u32, value: u16) -> std::result::Result<(), MemoryError> {
        self.access(addr, V6Access::Write, Some(&value.to_le_bytes()), None)
    }

    fn write_byte(&mut self, addr: u32, value: u8) -> std::result::Result<(), MemoryError> {
        self.access(addr, V6Access::Write, Some(&[value]), None)
    }
}

/// S5L8900 vCPU backed by the AArch32 interpreter.
pub struct S5L8900Vcpu {
    id: u32,
    cpu: Armv7Cpu,
    bridge: S5lBridge,
    decoder: Decoder,
    uart: Arc<OnceLock<Arc<Mutex<S3cUart>>>>,
    insn_count: u64,
    excl: crate::arm::instructions::ExclusiveMonitor,
    pc_ring: [u32; 512],
    pc_ring_idx: usize,
    zero_slide: u32,
    trace_pcs: Vec<u32>,
    trace_log_budget: u32,
    trace_start_insn: u64,
    fault_log_budget: u32,
    last_heartbeat: std::time::Instant,
    boot_instant: std::time::Instant,
    timer_speedup: u64,
    /// When set, derive guest µs from the instruction count instead of the
    /// host clock so that boots are fully deterministic (reproducible for
    /// debugging). `det_timer` holds the instructions-per-µs divisor.
    det_timer: u64,
    /// Multiplier on the µs counter — iBoot treats the timer as a 12 MHz tick
    /// counter (it reads freq=12000000 and computes elapsed = ticks*1e6/freq),
    /// so providing 12 ticks/µs makes its time math correct. From RAX_S5L_TIMER_MUL.
    timer_mul: u64,
    input_seeded: bool,
    timer_irq: bool,
    timer_irq_ready: u64,
    timer_irq_interval: u64,
    shutdown: bool,
}

enum StepOutcome {
    Progress,
    Idle,
}

impl S5L8900Vcpu {
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>) -> Self {
        // The console UART is provided by the VMM via attach_s3c_uart() so that
        // host stdin feeds the same instance the bridge serves.
        let uart = Arc::new(OnceLock::new());
        let bridge = S5lBridge {
            mem,
            inner: RefCell::new(BridgeInner {
                mmu: V6MmuConfig::default(),
                privileged: true,
                cur_pc: 0,
                last_fault: LastFault::default(),
                clock0: S5lClock::new(),
                clock1: S5lClock::new(),
                vic0: Pl192::new(),
                vic1: Pl192::new(),
                sysic: S5lSysic::new(),
                gpio: S5lGpio::new(),
                timer: S5lTimer::new(),
                i2c0: S5lI2c::new(false),
                i2c1: S5lI2c::new(true), // PMU (pcf50633) lives on I2C1
                spi0: S5lSpi::new(0),
                spi1: S5lSpi::new(1), // LCD panel
                spi2: S5lSpi::new(2), // multitouch
                lcd: S5lLcd::new(),
                nand: S5lNand::new(nand_dir()),
                nand_ecc: S5lNandEcc::new(),
                aes: S5lAes::new(),
                dmac0: S5lDmac::new(),
                adm_data2: 0,
                adm_data3: 0,
                adm_req: None,
                uart: uart.clone(),
                engine_8900_req: None,
                openbus_log_budget: std::env::var("RAX_S5L_OPENBUS_LOG")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(256),
                devlog: std::env::var("RAX_S5L_DEVLOG").is_ok(),
            }),
        };
        let mut cpu = Armv7Cpu::new();
        // ARM1176JZF-S (ARMv6K), the S5L8900 core.
        cpu.cp15.midr = 0x410F_B767;
        cpu.cp15.ctr = 0x1D15_2152;
        S5L8900Vcpu {
            id,
            cpu,
            bridge,
            decoder: Decoder::new_aarch32(),
            uart,
            insn_count: 0,
            excl: crate::arm::instructions::ExclusiveMonitor::default(),
            pc_ring: [0; 512],
            pc_ring_idx: 0,
            zero_slide: 0,
            trace_pcs: std::env::var("RAX_TRACE_PC")
                .ok()
                .map(|v| {
                    v.split(',')
                        .filter_map(|a| u32::from_str_radix(a.trim_start_matches("0x"), 16).ok())
                        .collect()
                })
                .unwrap_or_default(),
            trace_log_budget: std::env::var("RAX_S5L_TRACE_BUDGET")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(400),
            trace_start_insn: std::env::var("RAX_S5L_TRACE_START")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            fault_log_budget: 64,
            last_heartbeat: std::time::Instant::now(),
            boot_instant: std::time::Instant::now(),
            timer_speedup: timer_speedup(),
            timer_mul: std::env::var("RAX_S5L_TIMER_MUL")
                .ok()
                .and_then(|v| v.parse().ok())
                .filter(|&n| n > 0)
                .unwrap_or(1),
            det_timer: std::env::var("RAX_S5L_DET_TIMER")
                .ok()
                .map(|v| v.parse().ok().filter(|&n| n > 0).unwrap_or(32))
                .unwrap_or(0),
            input_seeded: false,
            // The system-tick IRQ drives iBoot's cooperative scheduler (it wakes
            // tasks blocked in task_sleep). It self-gates on the timer's `started`
            // flag, which iBoot sets by writing START to TIMER_4 STATE only after
            // the scheduler is up — so it is safe to enable by default. Opt out
            // with RAX_S5L_NO_TIMER_IRQ for regression isolation.
            timer_irq: !std::env::var("RAX_S5L_NO_TIMER_IRQ").is_ok(),
            timer_irq_ready: std::env::var("RAX_S5L_IRQ_READY").ok().and_then(|v| v.parse().ok()).unwrap_or(0),
            timer_irq_interval: std::env::var("RAX_S5L_IRQ_INTERVAL").ok().and_then(|v| v.parse().ok()).unwrap_or(50_000),
            shutdown: false,
        }
    }

    fn sync_mmu(&mut self) {
        let cp = &self.cpu.cp15;
        let mut inner = self.bridge.inner.borrow_mut();
        inner.mmu = V6MmuConfig {
            enabled: cp.sctlr.m(),
            ttbr0: cp.ttbr0 as u32,
            ttbr1: cp.ttbr1 as u32,
            ttbcr_n: cp.ttbcr & 0x7,
            dacr: cp.dacr,
            afe: false,
        };
        inner.privileged = self.cpu.cpsr.mode != ProcessorMode::User as u8;
    }

    /// Recompute VIC line levels from device state; return CPU IRQ assertion.
    fn sync_irqs(&mut self) -> bool {
        let uart_lvl = self
            .uart
            .get()
            .and_then(|u| u.lock().ok().map(|u| u.irq_pending()))
            .unwrap_or(false);
        let mut inner = self.bridge.inner.borrow_mut();
        let timer_lvl = inner.timer.irq_pending();
        let lcd_lvl = inner.lcd.irq_pending();
        let spi0_lvl = inner.spi0.irq_pending();
        let spi1_lvl = inner.spi1.irq_pending();
        let spi2_lvl = inner.spi2.irq_pending();
        let dmac0_lvl = inner.dmac0.irq_pending();
        let nand_ecc_lvl = inner.nand_ecc.irq_pending();
        inner.vic0.set_line(TIMER1_IRQ, timer_lvl);
        inner.vic0.set_line(LCD_IRQ, lcd_lvl);
        inner.vic0.set_line(UART0_IRQ, uart_lvl);
        inner.vic0.set_line(SPI0_IRQ, spi0_lvl);
        inner.vic0.set_line(SPI1_IRQ, spi1_lvl);
        inner.vic0.set_line(SPI2_IRQ, spi2_lvl);
        inner.vic0.set_line(DMAC0_IRQ, dmac0_lvl);
        inner.vic1.set_line(NAND_ECC_VIC1_LINE, nand_ecc_lvl);
        // VIC1 daisy-chains into VIC0.
        let daisy = inner.vic1.irq_asserted();
        inner.vic0.daisy_input = daisy;
        inner.vic0.update();
        inner.vic0.irq_asserted()
    }

    fn take_exception(&mut self, exc: ExceptionType) {
        let vbar = self.cpu.cp15.sctlr.vector_base();
        let mut exec = Executor::with_vbar(&mut self.cpu, &mut self.bridge, vbar);
        exec.take_exception(exc);
    }

    /// Service a pending 8900-engine decryption request, if any.
    fn service_engine_8900(&mut self) {
        let req = self.bridge.inner.borrow_mut().engine_8900_req.take();
        if let Some(addr) = req {
            self.decrypt_8900(addr);
        }
    }

    fn phys_r32(&self, pa: u32) -> u32 {
        let mut b = [0u8; 4];
        let _ = self.bridge.mem.read_slice(&mut b, GuestAddress(pa as u64));
        u32::from_le_bytes(b)
    }

    fn phys_w(&self, pa: u32, data: &[u8]) {
        let _ = self.bridge.mem.write_slice(data, GuestAddress(pa as u64));
    }

    /// Service a pending ADM (Apple Data Mover) command. The ADM is the
    /// high-level NAND DMA: iBoot writes a command structure into a data
    /// section in RAM, then pokes a control register; the ADM reads it, sets
    /// the NAND controller up for the page read, and deposits the page spare.
    fn service_adm(&mut self) {
        let req = self.bridge.inner.borrow_mut().adm_req.take();
        let Some(req) = req else { return };
        let off = req >> 28;
        let value = req & 0xFF;
        let (data2, data3) = {
            let i = self.bridge.inner.borrow();
            (i.adm_data2, i.adm_data3)
        };

        if off == 0 {
            // ADM_CTRL: start-up — mark device started and banks present.
            if value == 0x3 {
                self.phys_w(data2, &0x50u32.to_le_bytes());
                let mut chip = Vec::with_capacity(32);
                for _ in 0..8 {
                    chip.extend_from_slice(&0xA514_D3ADu32.to_le_bytes());
                }
                self.phys_w(data3, &chip);
            }
            return;
        }
        // ADM_CTRL2: execute the queued command.
        if value != 0x2 {
            return;
        }
        let cmd = self.phys_r32(data2 + 0x1104 + 0x24);
        let num_pages = {
            let v = self.phys_r32(data2 + 0x1104 + 0x28) as u16;
            v.swap_bytes()
        };
        let bswap = |p: u32| p.swap_bytes();

        match cmd {
            0x300 if num_pages == 1 => {
                let bank = (self.phys_r32(data2 + 0x1104 + 0x44) & 0xFF) as u32;
                let page = bswap(self.phys_r32(data2 + 0x1104 + 0x244));
                let mut inner = self.bridge.inner.borrow_mut();
                inner.nand.reading_multiple_pages = false;
                inner.nand.set_bank(bank);
                inner.nand.write(0x30, 0x800 - 1); // FMDNUM
                inner.nand.write(0xC, page << 16); // FMADDR0
                inner.nand.write(0x10, (page >> 16) & 0xFF); // FMADDR1
                inner.nand.write(0x8, 0x30); // CMD = READ
                inner.nand.set_buffered_page(page);
                let spare = inner.nand.spare_buffer.clone();
                drop(inner);
                self.phys_w(data3, &spare[..NAND_BYTES_PER_SPARE]);
            }
            0x300 | 0x200 => {
                // Scattered / multi-page read: queue the page/bank list.
                let n = num_pages as usize;
                let mut inner = self.bridge.inner.borrow_mut();
                inner.nand.reading_multiple_pages = true;
                for i in 0..n.min(512) {
                    let page = if cmd == 0x200 {
                        // Same starting page across all 8 banks per group.
                        let base = bswap(self.phys_r32(data2 + 0x1104 + 0x244));
                        base + (i / 8) as u32
                    } else {
                        bswap(self.phys_r32(data2 + 0x1104 + 0x244 + 4 * i as u32))
                    };
                    let bank = if cmd == 0x200 {
                        (i % 8) as u32
                    } else {
                        (self.phys_r32(data2 + 0x1104 + 0x44 + i as u32) & 0xFF) as u32
                    };
                    inner.nand.pages_to_read[i] = page;
                    inner.nand.banks_to_read[i] = bank;
                }
                inner.nand.fmdnum = num_pages as u32 * 0x800;
                inner.nand.cur_bank_reading = -1;
                drop(inner);
                // Mark each scattered page's spare slot as readable.
                for i in 0..n {
                    let mut sbuf = [0u8; 0xC];
                    sbuf[10] = 0xFF;
                    self.phys_w(data3 + i as u32 * 0xC, &sbuf);
                }
            }
            _ => {}
        }
    }

    fn step(&mut self) -> StepOutcome {
        self.sync_mmu();
        {
            let mut inner = self.bridge.inner.borrow_mut();
            inner.lcd.tick(1);
            // The synthesized system-tick IRQ drives iBoot's cooperative
            // scheduler. tick_irq self-gates on the timer's `started` flag, so
            // it only fires once iBoot has armed TIMER_4 (i.e. the scheduler is
            // up). This is what wakes tasks blocked in task_sleep.
            if self.timer_irq {
                inner.timer.tick_irq(self.insn_count, self.timer_irq_ready, self.timer_irq_interval);
            }
            // Update the µs timer from the host clock periodically (not every
            // instruction) so it tracks real time but stays stable across a
            // guest atomic counter read. The speedup factor makes firmware
            // delays elapse in a fraction of real time.
            if self.det_timer != 0 {
                inner.timer.set_micros(self.insn_count / self.det_timer * self.timer_mul);
            } else if self.insn_count & 0xFF == 0 {
                let us = self.boot_instant.elapsed().as_micros() as u64;
                inner.timer.set_micros(us.wrapping_mul(self.timer_speedup) * self.timer_mul);
            }
        }

        let irq_pending = self.sync_irqs();
        if irq_pending {
            self.cpu.is_halted = false;
        }
        // Only deliver an IRQ once the guest has actually installed its
        // exception vectors at the active vector base. iBoot runs from
        // 0x18000000 and maps virtual 0 -> its in-place vector table only after
        // early init; before that the IRQ vector reads back as zeros and the
        // CPU would walk off into low memory. The real IRQ instruction is
        // `LDR pc, [pc, #0x18]` (0xe59ff018), so gate on seeing it. This
        // replaces the fragile fixed-instruction-count readiness heuristic.
        let vectors_ready = {
            let vbar = self.cpu.cp15.sctlr.vector_base();
            matches!(self.bridge.read_word(vbar.wrapping_add(0x18)), Ok(0xe59f_f018))
        };
        if irq_pending && !self.cpu.cpsr.i && vectors_ready {
            let from = self.cpu.regs[15];
            let from_cpsr = self.cpu.cpsr.to_u32();
            let vbar = self.cpu.cp15.sctlr.vector_base();
            self.take_exception(ExceptionType::Irq);
            if self.fault_log_budget > 0 {
                self.fault_log_budget -= 1;
                debug!(
                    from = format!("{from:#x}"),
                    from_cpsr = format!("{from_cpsr:#x}"),
                    spsr_irq = format!("{:#x}", self.cpu.spsr_irq.to_u32()),
                    vbar = format!("{vbar:#x}"),
                    to = format!("{:#x}", self.cpu.regs[15]),
                    irq_sp = format!("{:#x}", self.cpu.regs[13]),
                    insns = self.insn_count,
                    "irq delivered"
                );
            }
            return StepOutcome::Progress;
        }

        if self.cpu.is_halted {
            let micros = if self.det_timer != 0 {
                self.insn_count / self.det_timer * self.timer_mul
            } else {
                let us = self.boot_instant.elapsed().as_micros() as u64;
                us.wrapping_mul(self.timer_speedup)
            };
            self.bridge.inner.borrow_mut().timer.set_micros(micros);
            return StepOutcome::Idle;
        }

        // Debug aid: once iBoot has reached its console wait, inject serial
        // input (RAX_S5L_INPUT) into the UART RX so its recovery console can be
        // driven (e.g. a boot command). Seeded once.
        if !self.input_seeded && self.insn_count > 50_000_000 {
            if let Ok(s) = std::env::var("RAX_S5L_INPUT") {
                if let Some(u) = self.uart.get() {
                    if let Ok(mut u) = u.lock() {
                        let mut bytes = s.replace("\\r", "\r").replace("\\n", "\n").into_bytes();
                        bytes.push(b'\r');
                        u.queue_input(&bytes);
                    }
                }
                self.input_seeded = true;
            }
        }

        let pc = self.cpu.regs[15];
        self.pc_ring[self.pc_ring_idx] = pc;
        self.pc_ring_idx = (self.pc_ring_idx + 1) % self.pc_ring.len();
        let is_thumb = self.cpu.cpsr.t;

        let mut insn_len = 4u32;
        // Decode bytes in MEMORY order (the decoder reads hw1 from bytes[0..2],
        // hw2 from bytes[2..4]). For ARM and 16-bit Thumb this is just the
        // little-endian word/halfword.
        let mut decode_bytes = [0u8; 4];
        let raw = if is_thumb {
            let hw1 = match self.bridge.read_halfword(pc) {
                Ok(v) => v,
                Err(_) => return self.prefetch_abort(pc),
            };
            decode_bytes[0..2].copy_from_slice(&hw1.to_le_bytes());
            if (hw1 >> 11) >= 0x1D {
                let hw2 = match self.bridge.read_halfword(pc.wrapping_add(2)) {
                    Ok(v) => v,
                    Err(_) => return self.prefetch_abort(pc),
                };
                decode_bytes[2..4].copy_from_slice(&hw2.to_le_bytes());
                ((hw1 as u32) << 16) | hw2 as u32
            } else {
                insn_len = 2;
                hw1 as u32
            }
        } else {
            let w = match self.bridge.read_word(pc) {
                Ok(v) => v,
                Err(_) => return self.prefetch_abort(pc),
            };
            decode_bytes.copy_from_slice(&w.to_le_bytes());
            w
        };

        // Derail detector: a long run of all-zero instruction words means the
        // CPU has branched into zeroed memory. Dump the recent PC history so
        // the bad branch can be located, then stop.
        if raw == 0 && pc < 0x1800_0000 {
            self.zero_slide += 1;
            if self.zero_slide == 48 && std::env::var("RAX_S5L_DERAIL").is_ok() {
                let mut trail = Vec::new();
                for i in 0..self.pc_ring.len() {
                    let idx = (self.pc_ring_idx + i) % self.pc_ring.len();
                    let p = self.pc_ring[idx];
                    if p >= 0x1800_0000 {
                        trail.push(format!("{p:#x}"));
                    }
                }
                debug!(trail = trail.join(" "), insns = self.insn_count, "derail trail");
                self.shutdown = true;
                return StepOutcome::Idle;
            }
        } else {
            self.zero_slide = 0;
        }

        if std::env::var("RAX_S5L_TRACE").is_ok()
            && self.insn_count >= self.trace_start_insn
            && self.trace_log_budget > 0
        {
            self.trace_log_budget -= 1;
            debug!(
                pc = format!("{pc:#x}"),
                raw = format!("{raw:#010x}"),
                lr = format!("{:#x}", self.cpu.regs[14]),
                cpsr = format!("{:#x}", self.cpu.cpsr.to_u32()),
                spsr_irq = format!("{:#x}", self.cpu.spsr_irq.to_u32()),
                insns = self.insn_count,
                "insn"
            );
        }
        if self.trace_pcs.contains(&pc)
            && self.trace_log_budget > 0
            && self.insn_count >= self.trace_start_insn
        {
            self.trace_log_budget -= 1;
            let regs: Vec<String> =
                (0..16).map(|i| format!("r{i}={:#x}", self.cpu.regs[i])).collect();
            debug!(
                regs = regs.join(" "),
                cpsr = format!("{:#x}", self.cpu.cpsr.to_u32()),
                insns = self.insn_count,
                "trace-pc hit"
            );
        }

        let state = if is_thumb {
            ExecutionState::Thumb
        } else {
            ExecutionState::Aarch32
        };
        let slice: &[u8] = if insn_len == 2 {
            &decode_bytes[..2]
        } else {
            &decode_bytes
        };
        self.decoder.set_state(state);
        let insn = match self.decoder.decode(slice) {
            Ok(i) => i,
            Err(_) => {
                self.take_exception(ExceptionType::UndefinedInstruction);
                return StepOutcome::Progress;
            }
        };

        if watch_range().is_some() {
            self.bridge.inner.borrow_mut().cur_pc = pc;
        }
        let vbar = self.cpu.cp15.sctlr.vector_base();
        let mut exec = Executor::with_vbar(&mut self.cpu, &mut self.bridge, vbar);
        exec.exclusive_monitor = self.excl.clone();
        let result = exec.execute(&insn);
        self.excl = exec.exclusive_monitor.clone();
        self.insn_count += 1;

        // Service any decryption request the just-executed store triggered.
        if self.bridge.inner.borrow().engine_8900_req.is_some() {
            self.service_engine_8900();
        }
        // Service an AES-engine GO the just-executed store triggered.
        if self.bridge.inner.borrow().aes.pending_go {
            self.service_aes();
        }
        // Service any ADM/NAND DMA command the just-executed store triggered.
        if self.bridge.inner.borrow().adm_req.is_some() {
            self.service_adm();
        }

        match result {
            ExecResult::Continue => {
                self.cpu.regs[15] = self.cpu.regs[15].wrapping_add(insn_len);
                if self.cpu.cpsr.in_it_block() {
                    self.cpu.cpsr.advance_it_state();
                }
                StepOutcome::Progress
            }
            ExecResult::Branch(target) => {
                if target & 1 != 0 {
                    self.cpu.cpsr.t = true;
                    self.cpu.regs[15] = target & !1;
                } else {
                    self.cpu.regs[15] = target;
                }
                StepOutcome::Progress
            }
            ExecResult::Exception(exc) => {
                self.take_exception(exc);
                StepOutcome::Progress
            }
            ExecResult::Halt => {
                self.cpu.is_halted = true;
                StepOutcome::Idle
            }
            ExecResult::Undefined => {
                self.take_exception(ExceptionType::UndefinedInstruction);
                StepOutcome::Progress
            }
            ExecResult::MemoryFault(_) => {
                let f = self.bridge.inner.borrow().last_fault;
                if self.fault_log_budget > 0 {
                    self.fault_log_budget -= 1;
                    debug!(
                        pc = format!("{:#x}", self.cpu.regs[15]),
                        addr = format!("{:#x}", f.addr),
                        fsr = format!("{:#x}", f.fsr),
                        insns = self.insn_count,
                        "data abort"
                    );
                }
                self.cpu.cp15.dfsr =
                    f.fsr | (f.domain << 4) | if f.access & 3 == 1 { 1 << 11 } else { 0 };
                self.cpu.cp15.dfar = f.addr;
                self.take_exception(ExceptionType::DataAbort(f.addr));
                StepOutcome::Progress
            }
        }
    }

    fn prefetch_abort(&mut self, pc: u32) -> StepOutcome {
        let f = self.bridge.inner.borrow().last_fault;
        if self.fault_log_budget > 0 {
            self.fault_log_budget -= 1;
            debug!(
                pc = format!("{pc:#x}"),
                fsr = format!("{:#x}", f.fsr),
                lr = format!("{:#x}", self.cpu.regs[14]),
                insns = self.insn_count,
                "prefetch abort"
            );
        }
        self.cpu.cp15.ifsr = f.fsr | (f.domain << 4);
        self.cpu.cp15.ifar = pc;
        self.take_exception(ExceptionType::PrefetchAbort(pc));
        StepOutcome::Progress
    }

    fn heartbeat(&mut self) {
        if self.last_heartbeat.elapsed() >= std::time::Duration::from_secs(2) {
            debug!(
                insns = self.insn_count,
                pc = format!("{:#x}", self.cpu.regs[15]),
                lr = format!("{:#x}", self.cpu.regs[14]),
                cpsr = format!("{:#x}", self.cpu.cpsr.to_u32()),
                mmu = self.bridge.inner.borrow().mmu.enabled,
                "s5l8900 emulator heartbeat"
            );
            // Debug aid: dump the call stack (likely iBoot return addresses
            // in 0x18xxxxxx found on the stack) to reveal the active loop.
            if std::env::var("RAX_S5L_STACKDUMP").is_ok() {
                let sp = self.cpu.regs[13];
                let mut frames = Vec::new();
                for i in 0..256u32 {
                    if let Ok(w) = self.bridge.read_word(sp.wrapping_add(i * 4)) {
                        let a = w & !1; // strip Thumb bit
                        if (0x1800_0000..0x1802_0000).contains(&a) {
                            frames.push(format!("{a:#x}"));
                        }
                    }
                }
                debug!(
                    sp = format!("{sp:#x}"),
                    pc = format!("{:#x}", self.cpu.regs[15]),
                    stack = frames.join(" "),
                    "stack dump"
                );
            }
            // Debug aid: dump an arbitrary physical region (RAX_S5L_MEMDUMP=
            // <hexaddr>:<hexlen>:<path>) — e.g. iBoot's log/heap area.
            if let Ok(spec) = std::env::var("RAX_S5L_MEMDUMP") {
                let parts: Vec<&str> = spec.split(':').collect();
                if parts.len() == 3 {
                    if let (Ok(addr), Ok(len)) = (
                        u64::from_str_radix(parts[0].trim_start_matches("0x"), 16),
                        usize::from_str_radix(parts[1].trim_start_matches("0x"), 16),
                    ) {
                        let mut buf = vec![0u8; len];
                        if self.bridge.mem.read_slice(&mut buf, GuestAddress(addr)).is_ok() {
                            let _ = std::fs::write(parts[2], &buf);
                        }
                    }
                }
            }
            // Debug aid: dump the LCD framebuffer (physical 0x0fe00000,
            // 320x480 BGRA) to a file so the rendered boot image can be
            // inspected. Gated by RAX_S5L_FBDUMP=<path>.
            if let Ok(path) = std::env::var("RAX_S5L_FBDUMP") {
                let mut buf = vec![0u8; 320 * 480 * 4];
                if self
                    .bridge
                    .mem
                    .read_slice(&mut buf, GuestAddress(0x0fe0_0000))
                    .is_ok()
                {
                    let _ = std::fs::write(&path, &buf);
                }
            }
            self.last_heartbeat = std::time::Instant::now();
        }
    }
}

impl VCpu for S5L8900Vcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        if self.shutdown {
            return Ok(VcpuExit::Shutdown);
        }
        for _ in 0..BATCH {
            match self.step() {
                StepOutcome::Progress => {}
                StepOutcome::Idle => {
                    self.heartbeat();
                    return Ok(VcpuExit::Hlt);
                }
            }
        }
        self.heartbeat();
        Ok(VcpuExit::Hlt)
    }

    fn get_state(&self) -> Result<CpuState> {
        let mut regs = Aarch32Registers::default();
        for i in 0..13 {
            regs.r[i] = self.cpu.regs[i];
        }
        regs.sp = self.cpu.regs[13];
        regs.lr = self.cpu.regs[14];
        regs.pc = self.cpu.regs[15];
        regs.cpsr = self.cpu.cpsr.to_u32();
        Ok(CpuState::Aarch32(Aarch32CpuState {
            regs,
            sregs: Aarch32SystemRegisters::default(),
        }))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::Aarch32(s) => s,
            _ => {
                return Err(Error::Emulator(
                    "expected aarch32 state for s5l8900 vCPU".to_string(),
                ));
            }
        };
        let cpsr = crate::arm::Psr::from_u32(state.regs.cpsr);
        if let Some(mode) = ProcessorMode::from_bits(cpsr.mode) {
            self.cpu.change_mode(mode);
        }
        for i in 0..13 {
            self.cpu.regs[i] = state.regs.r[i];
        }
        self.cpu.regs[13] = state.regs.sp;
        self.cpu.regs[14] = state.regs.lr;
        self.cpu.regs[15] = state.regs.pc;
        self.cpu.cpsr = cpsr;
        Ok(())
    }

    fn complete_io_in(&mut self, _data: &[u8]) {}

    fn attach_s3c_uart(
        &mut self,
        uart: std::sync::Arc<std::sync::Mutex<crate::devices::s3c64xx::S3cUart>>,
    ) {
        // The S5L8900 UART is register-compatible with the Samsung s3c UART,
        // so it reuses the VMM's console plumbing (host stdin → guest serial).
        let _ = self.uart.set(uart);
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn instruction_count(&self) -> u64 {
        self.insn_count
    }
}

impl S5L8900Vcpu {
    /// Service an 8900-engine in-place AES-CBC decryption request. `addr` is
    /// the physical address of the 8900 image header; the body that follows
    /// is decrypted in place. Currently a logged no-op — the bootrom-call
    /// patch already reports success, so iBoot's verification proceeds; real
    /// decryption is added when an encrypted image is actually loaded.
    fn decrypt_8900(&mut self, addr: u32) {
        if self.fault_log_budget > 0 {
            self.fault_log_budget -= 1;
            debug!(addr = format!("{addr:#x}"), "8900 decrypt request (stub)");
        }
    }

    /// Service an AES-engine `AES_GO`: DMA `insize` bytes from `inaddr`, AES-CBC
    /// decrypt them with the selected key, and write the plaintext to `outaddr`
    /// (in guest physical memory). The GID key, which is not in the QEMU
    /// reference, may be supplied as 16/24/32 hex bytes via `RAX_S5L_GID_KEY`.
    fn service_aes(&mut self) {
        let (inaddr, outaddr, insize, keytype, custkey, ivec) = {
            let inner = self.bridge.inner.borrow();
            let a = &inner.aes;
            (a.inaddr, a.outaddr, a.insize, a.keytype, a.custkey, a.ivec)
        };

        // Resolve the decryption key.
        let key_bytes: Option<Vec<u8>> = match keytype {
            AesKeyType::Uid => Some(AES_UID_KEY.to_vec()),
            AesKeyType::Custom => {
                // Custom key length follows the AES key-length register; default
                // to AES-256 (the engine's widest) using all 32 bytes.
                Some(custkey.to_vec())
            }
            AesKeyType::Gid => std::env::var("RAX_S5L_GID_KEY").ok().and_then(|h| {
                let h = h.trim().trim_start_matches("0x");
                if h.len() % 2 != 0 {
                    return None;
                }
                (0..h.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&h[i..i + 2], 16).ok())
                    .collect::<Option<Vec<u8>>>()
            }),
        };

        let len = insize as usize;
        let mut ok = false;
        if let Some(kb) = key_bytes {
            if let Some(key) = AesKey::new(&kb) {
                if len > 0 && len % 16 == 0 {
                    let mut buf = vec![0u8; len];
                    if self
                        .bridge
                        .mem
                        .read_slice(&mut buf, GuestAddress(inaddr as u64))
                        .is_ok()
                    {
                        aes_cbc_decrypt(&key, &ivec, &mut buf);
                        ok = self
                            .bridge
                            .mem
                            .write_slice(&buf, GuestAddress(outaddr as u64))
                            .is_ok();
                    }
                }
            }
        }

        if self.fault_log_budget > 0 {
            self.fault_log_budget -= 1;
            debug!(
                inaddr = format!("{inaddr:#x}"),
                outaddr = format!("{outaddr:#x}"),
                insize = len,
                keytype = match keytype {
                    AesKeyType::Uid => "uid",
                    AesKeyType::Gid => "gid",
                    AesKeyType::Custom => "custom",
                },
                ok,
                "aes engine decrypt"
            );
        }

        let mut inner = self.bridge.inner.borrow_mut();
        inner.aes.pending_go = false;
        inner.aes.outsize = insize;
        inner.aes.finish();
    }

    /// Compute the SHA-1 digest of a guest physical region (for the SHA engine
    /// / image-hash verification path). Returns the 20-byte digest.
    #[allow(dead_code)]
    fn sha1_region(&self, addr: u32, len: u32) -> Option<[u8; 20]> {
        let mut buf = vec![0u8; len as usize];
        self.bridge.mem.read_slice(&mut buf, GuestAddress(addr as u64)).ok()?;
        Some(sha1(&buf))
    }
}
