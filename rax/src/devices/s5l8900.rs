//! Samsung S5L8900 (Apple A-series predecessor) platform devices.
//!
//! The S5L8900 is the SoC in the first-generation iPod Touch / iPhone
//! (ARM1176JZF-S, ARMv6K). These models are ported from the QEMU iPod Touch
//! 1G reference machine and are sufficient to bring up Apple's iBoot
//! second-stage bootloader.

use std::collections::VecDeque;
use std::io::{self, Write};

// =============================================================================
// Clock controller (CLOCK0 / CLOCK1)
// =============================================================================

/// S5L8900 clock/PLL controller. Two instances exist (CLOCK0, CLOCK1); both
/// answer with the same register file. iBoot reads the PLL config to derive
/// the CPU/bus/peripheral frequencies.
pub struct S5lClock {
    config0: u32,
    config1: u32,
    config2: u32,
    pll0con: u32,
    pll1con: u32,
    pll2con: u32,
    pll3con: u32,
    plllock: u32,
}

impl S5lClock {
    pub fn new() -> Self {
        let mut config0 = 0u32;
        config0 |= 1 << 12; // clock PLL index 1
        config0 |= 1 << 24; // has memory divisor
        config0 |= 2 << 16; // memory divisor = 2

        let mut config1 = 0u32;
        config1 |= 1 << 12; // bus PLL index 1
        config1 |= 1 << 24; // has bus divisor
        config1 |= 3 << 16; // bus divisor = 3
        config1 |= 1 << 8; // unknown has divisor
        config1 |= 3; // unknown divisor 1 = 3
        config1 |= 1 << 20; // peripheral factor = 1
        config1 |= 1 << 14;
        config1 |= 1 << 28; // some PLL index = 1
        config1 |= 1 << 30;

        let mut config2 = 0u32;
        config2 |= 3 << 28; // peripheral PLL index 3
        config2 |= 1 << 24; // display has divisor
        config2 |= 1 << 16; // display divisor = 1

        S5lClock {
            config0,
            config1,
            config2,
            // MDIV, PDIV, SDIV per PLL
            pll0con: (80 << 8) | (8 << 24),
            pll1con: (103 << 8) | (6 << 24),
            pll2con: (156 << 8) | (53 << 24) | 2,
            pll3con: (72 << 8) | (8 << 24) | 1,
            plllock: 1 | 2 | 4 | 8,
        }
    }

    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            0x00 => self.config0,
            0x04 => self.config1,
            0x08 => self.config2,
            0x20 => self.pll0con,
            0x24 => self.pll1con,
            0x28 => self.pll2con,
            0x2C => self.pll3con,
            0x40 => self.plllock,
            0x44 => 0x000a_003a, // PLLMODE (captured from real hardware)
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        if offset == 0x40 {
            self.plllock = value;
        }
    }
}

impl Default for S5lClock {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// System timer
// =============================================================================

/// S5L8900 timer block. iBoot uses the free-running tick counter (TICKSHIGH /
/// TICKSLOW) for microsecond delay loops. We drive it from a tick value the
/// vCPU advances with executed instructions, so guest busy-waits terminate.
pub struct S5lTimer {
    /// Free-running microsecond counter. iBoot treats the timer as a µs clock
    /// (it divides by 1_000_000 to get seconds). The value is updated coarsely
    /// (every N instructions) from a host clock so it tracks real time yet
    /// stays stable across a guest's few-instruction atomic counter read
    /// (read-low / read-high / read-low again, which assumes the timer is
    /// slower than the CPU).
    micros: u64,
    status: u32,
    config: u32,
    bcount1: u32,
    bcount2: u32,
    irqstat: u32,
    /// Set when the timer reaches its reload and an IRQ is pending.
    irq: bool,
    /// Whether the periodic timer (timer 4) is running.
    started: bool,
    /// Instruction count at which to next assert the periodic IRQ.
    next_irq_insn: u64,
}

impl S5lTimer {
    pub fn new() -> Self {
        S5lTimer {
            micros: 0,
            status: 0,
            config: 0,
            bcount1: 0,
            bcount2: 0,
            irqstat: 0,
            irq: false,
            started: false,
            next_irq_insn: 0,
        }
    }

    /// Set the µs counter from the host clock (called periodically, NOT every
    /// instruction, so the value is stable across a guest atomic read).
    pub fn set_micros(&mut self, micros: u64) {
        // Monotonic: never let the counter go backwards.
        if micros > self.micros {
            self.micros = micros;
        }
    }

    /// Raise the periodic system-tick IRQ, paced by executed instructions (so
    /// the rate is independent of the µs-counter speedup) and held off until
    /// the guest has run long enough to have created its scheduler tasks.
    pub fn tick_irq(&mut self, insn_count: u64, ready: u64, interval: u64) {
        if !self.started || insn_count < ready {
            return;
        }
        if insn_count >= self.next_irq_insn {
            self.irq = true;
            self.next_irq_insn = insn_count.max(ready) + interval;
        }
    }

    pub fn irq_pending(&self) -> bool {
        self.irq
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        let t = self.micros;
        match offset {
            0x80 => (t >> 32) as u32, // TICKSHIGH
            0x84 => t as u32,          // TICKSLOW
            0x10000 => !0,             // IRQSTAT
            0xF8 => 0xFFFF_FFFF,       // IRQLATCH
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x10000 => self.irqstat = value,
            0xF8 => self.irq = false, // IRQLATCH: acknowledge
            // Timer 4 (0xA0) sub-registers.
            0xA0 => self.config = value, // CONFIG
            0xA4 => {
                // STATE: bit0 = start. Arm/disarm the periodic IRQ.
                self.status = value;
                self.started = value & 1 != 0;
            }
            0xA8 => self.bcount1 = value, // COUNT_BUFFER
            0xAC => self.bcount2 = value, // COUNT_BUFFER2
            _ => {}
        }
    }
}

impl Default for S5lTimer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// PL080 DMA controller (DMAC0 / DMAC1)
// =============================================================================

/// ARM PL080 DMA controller. iBoot uses DMAC0 (0x38200000) to stream NAND
/// pages from the flash-controller FIFO (0x38A00080) into RAM. We model the
/// register file and per-channel descriptors here; the backend performs the
/// actual memory movement when a channel is enabled (it needs access to guest
/// memory and the peripheral read path, which live on the bridge).
pub struct S5lDmac {
    pub src: [u32; 8],
    pub dst: [u32; 8],
    pub lli: [u32; 8],
    pub control: [u32; 8],
    pub config: [u32; 8],
    /// Controller-global DMACConfiguration (0x030).
    pub global_config: u32,
    /// IntTCStatus / RawIntTCStatus (terminal-count interrupt, per channel).
    pub tc_status: u32,
    /// EnbldChns (0x01C): which channels are currently enabled.
    pub enbld: u32,
    /// Channel whose transfer the backend should run, set on a channel-enable
    /// write and consumed via `take_pending()`.
    pending: Option<u8>,
}

impl S5lDmac {
    pub fn new() -> Self {
        S5lDmac {
            src: [0; 8],
            dst: [0; 8],
            lli: [0; 8],
            control: [0; 8],
            config: [0; 8],
            global_config: 0,
            tc_status: 0,
            enbld: 0,
            pending: None,
        }
    }

    /// Take the channel scheduled by the last channel-enable write, if any.
    pub fn take_pending(&mut self) -> Option<u8> {
        self.pending.take()
    }

    /// Mark channel `ch`'s transfer complete: zero the remaining size, disable
    /// the channel, and raise its terminal-count interrupt status.
    pub fn complete(&mut self, ch: usize) {
        self.control[ch] &= !0xFFF;
        self.config[ch] &= !1;
        self.enbld &= !(1 << ch);
        self.tc_status |= 1 << ch;
    }

    /// Channel IRQ line level: any terminal-count status whose channel has the
    /// terminal-count interrupt mask (Config bit15) set.
    pub fn irq_pending(&self) -> bool {
        let mut mask = 0u32;
        for ch in 0..8 {
            if self.config[ch] & (1 << 15) != 0 {
                mask |= 1 << ch;
            }
        }
        self.tc_status & mask != 0
    }

    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            0x000 => self.tc_status,         // IntStatus
            0x004 => self.tc_status,         // IntTCStatus
            0x00C => 0,                      // IntErrorStatus
            0x014 => self.tc_status,         // RawIntTCStatus
            0x018 => 0,                      // RawIntErrorStatus
            0x01C => self.enbld,             // EnbldChns
            0x030 => self.global_config,     // DMACConfiguration
            _ if (0x100..0x200).contains(&offset) => {
                let ch = ((offset - 0x100) / 0x20) as usize;
                match (offset - 0x100) % 0x20 {
                    0x00 => self.src[ch],
                    0x04 => self.dst[ch],
                    0x08 => self.lli[ch],
                    0x0C => self.control[ch],
                    0x10 => self.config[ch],
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, val: u32) {
        match offset {
            0x008 => self.tc_status &= !val, // IntTCClear
            0x010 => {}                      // IntErrClr
            0x030 => self.global_config = val,
            _ if (0x100..0x200).contains(&offset) => {
                let ch = ((offset - 0x100) / 0x20) as usize;
                match (offset - 0x100) % 0x20 {
                    0x00 => self.src[ch] = val,
                    0x04 => self.dst[ch] = val,
                    0x08 => self.lli[ch] = val,
                    0x0C => self.control[ch] = val,
                    0x10 => {
                        self.config[ch] = val;
                        // Channel Enable (bit0): schedule the transfer.
                        if val & 1 != 0 {
                            self.enbld |= 1 << ch;
                            self.pending = Some(ch as u8);
                        } else {
                            self.enbld &= !(1 << ch);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Default for S5lDmac {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Chip ID
// =============================================================================

/// S5L8900 chip-identification block.
pub struct S5lChipId;

impl S5lChipId {
    pub fn read(offset: u32) -> u32 {
        match offset {
            0x4 => 0x2 << 24, // CHIP_REVISION
            _ => 0,
        }
    }
}

// =============================================================================
// GPIO
// =============================================================================

/// S5L8900 GPIO controller. Only the button/level state register at 0x2c4 is
/// meaningfully read by early firmware.
pub struct S5lGpio {
    pub gpio_state: u32,
}

impl S5lGpio {
    pub fn new() -> Self {
        S5lGpio { gpio_state: 0 }
    }

    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            0x2c4 => self.gpio_state,
            _ => 0,
        }
    }

    pub fn write(&mut self, _offset: u32, _value: u32) {}
}

impl Default for S5lGpio {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// System interrupt controller / power (SYSIC)
// =============================================================================

pub const GPIO_NUMINTGROUPS: usize = 7;

/// S5L8900 SYSIC: power management plus the GPIO interrupt aggregation groups.
pub struct S5lSysic {
    power_state: u32,
    pub gpio_int_level: [u32; GPIO_NUMINTGROUPS],
    pub gpio_int_status: [u32; GPIO_NUMINTGROUPS],
    pub gpio_int_enabled: [u32; GPIO_NUMINTGROUPS],
    pub gpio_int_type: [u32; GPIO_NUMINTGROUPS],
}

impl S5lSysic {
    pub fn new() -> Self {
        S5lSysic {
            power_state: 0,
            gpio_int_level: [0; GPIO_NUMINTGROUPS],
            gpio_int_status: [0; GPIO_NUMINTGROUPS],
            gpio_int_enabled: [0; GPIO_NUMINTGROUPS],
            gpio_int_type: [0; GPIO_NUMINTGROUPS],
        }
    }

    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            0x44 => 2 << 0x18,                  // POWER_ID
            0x08 | 0x14 => self.power_state,    // POWER_SETSTATE / POWER_STATE
            0x7a | 0x7c => 1,
            0x80..=0x9C => self.gpio_int_level[((offset - 0x80) / 4) as usize],
            0xA0..=0xBC => self.gpio_int_status[((offset - 0xA0) / 4) as usize],
            0xC0..=0xDC => self.gpio_int_enabled[((offset - 0xC0) / 4) as usize],
            0xE0..=0xFC => self.gpio_int_type[((offset - 0xE0) / 4) as usize],
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x0C => {
                // POWER_ONCTRL: ignore a few devices so they read as already on.
                if value & 0x20 == 0 && value & 0x4 == 0 && value & 0x10 == 0 {
                    self.power_state = value;
                }
            }
            0x10 => self.power_state = value, // POWER_OFFCTRL
            0xA0..=0xBC => {
                let g = ((offset - 0xA0) / 4) as usize;
                self.gpio_int_status[g] &= !value; // write-1-to-clear
            }
            0xC0..=0xDC => self.gpio_int_enabled[((offset - 0xC0) / 4) as usize] = value,
            0xE0..=0xFC => self.gpio_int_type[((offset - 0xE0) / 4) as usize] = value,
            _ => {}
        }
    }
}

impl Default for S5lSysic {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// PL192 Vectored Interrupt Controller
// =============================================================================

const PL192_INT_SOURCES: usize = 32;

/// ARM PrimeCell PL192 VIC, as used by the S5L8900 (two instances chained:
/// VIC1's parent output daisy-chains into VIC0, which drives the CPU).
pub struct Pl192 {
    pub vect_addr: [u32; PL192_INT_SOURCES],
    pub vect_priority: [u32; PL192_INT_SOURCES],
    pub rawintr: u32,
    pub intselect: u32,
    pub intenable: u32,
    pub softint: u32,
    pub protection: u32,
    pub sw_priority_mask: u32,
    pub daisy_priority: u32,
    pub irq_status: u32,
    pub fiq_status: u32,
    /// Address last latched for the acknowledged interrupt.
    pub address: u32,
    /// Daisy input level from the downstream controller (VIC1 → VIC0).
    pub daisy_input: bool,
    pub daisy_vectaddr: u32,
}

impl Pl192 {
    pub fn new() -> Self {
        Pl192 {
            vect_addr: [0; PL192_INT_SOURCES],
            vect_priority: [0xf; PL192_INT_SOURCES],
            rawintr: 0,
            intselect: 0,
            intenable: 0,
            softint: 0,
            protection: 0,
            sw_priority_mask: 0xffff,
            daisy_priority: 0xf,
            irq_status: 0,
            fiq_status: 0,
            address: 0,
            daisy_input: false,
            daisy_vectaddr: 0,
        }
    }

    /// Recompute IRQ/FIQ status from raw/soft inputs and the masks.
    pub fn update(&mut self) {
        let active = (self.rawintr | self.softint) & self.intenable;
        self.irq_status = active & !self.intselect;
        self.fiq_status = active & self.intselect;
        // Latch the vector for the lowest-numbered pending IRQ (priority is
        // not modelled in detail — early firmware does not depend on it).
        if self.irq_status != 0 {
            let n = self.irq_status.trailing_zeros() as usize;
            self.address = self.vect_addr[n];
        } else if self.daisy_input {
            self.address = self.daisy_vectaddr;
        }
    }

    /// CPU IRQ line level contributed by this controller (excluding daisy).
    pub fn irq_asserted(&self) -> bool {
        self.irq_status != 0 || self.daisy_input
    }

    pub fn fiq_asserted(&self) -> bool {
        self.fiq_status != 0
    }

    pub fn set_line(&mut self, irq: u32, level: bool) {
        if irq >= PL192_INT_SOURCES as u32 {
            return;
        }
        if level {
            self.rawintr |= 1 << irq;
        } else {
            self.rawintr &= !(1 << irq);
        }
        self.update();
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        if (0xfe0..0x1000).contains(&offset) {
            let id = [0x92u8, 0x11, 0x04, 0x00, 0x0D, 0xF0, 0x05, 0xB1];
            return id[((offset - 0xfe0) >> 2) as usize] as u32;
        }
        if (0x100..0x180).contains(&offset) {
            return self.vect_addr[((offset - 0x100) >> 2) as usize];
        }
        if (0x200..0x280).contains(&offset) {
            return self.vect_priority[((offset - 0x200) >> 2) as usize];
        }
        match offset {
            0x00 => self.irq_status,
            0x04 => self.fiq_status,
            0x08 => self.rawintr,
            0x0C => self.intselect,
            0x10 => self.intenable,
            0x18 => self.softint,
            0x20 => self.protection,
            0x24 => self.sw_priority_mask,
            0x28 => self.daisy_priority,
            0x14 => 0, // INTENCLEAR
            0xF00 => self.address, // VECTADDR (ack)
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        if (0x100..0x180).contains(&offset) {
            self.vect_addr[((offset - 0x100) >> 2) as usize] = value;
            self.update();
            return;
        }
        if (0x200..0x280).contains(&offset) {
            self.vect_priority[((offset - 0x200) >> 2) as usize] = value & 0xf;
            self.update();
            return;
        }
        match offset {
            0x0C => self.intselect = value,
            0x10 => self.intenable |= value,
            0x14 => self.intenable &= !value, // INTENCLEAR
            0x18 => self.softint |= value,
            0x1C => self.softint &= !value, // SOFTINTCLEAR
            0x20 => self.protection = value & 1,
            0x24 => self.sw_priority_mask = value & 0xffff,
            0x28 => self.daisy_priority = value & 0xf,
            0xF00 => {} // VECTADDR finish: no priority stack modelled
            _ => {}
        }
        self.update();
    }
}

impl Default for Pl192 {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// NAND flash controller (FMSS) + ECC, backed by per-page files
// =============================================================================

pub const NAND_NUM_BANKS: u32 = 8;
pub const NAND_BYTES_PER_PAGE: usize = 2048;
pub const NAND_BYTES_PER_SPARE: usize = 64;
const NAND_CHIP_ID: u32 = 0xA514_D3AD;
const NAND_CMD_ID: u32 = 0x90;
const NAND_CMD_READ: u32 = 0x30;
const NAND_CMD_READSTATUS: u32 = 0x70;

/// S5L8900 NAND controller. iBoot's flash driver programs a bank/page/command
/// then drains the page through the FMFIFO register; the ADM DMA sets those
/// registers up. Page contents come from `<nand>/bank<b>/<page>.page` files
/// (2048 data + 64 spare bytes each); a missing page reads as zeros.
pub struct S5lNand {
    nand_path: Option<std::path::PathBuf>,
    pub fmctrl0: u32,
    fmctrl1: u32,
    fmaddr0: u32,
    fmaddr1: u32,
    fmanum: u32,
    pub fmdnum: u32,
    rsctrl: u32,
    cmd: u32,
    reading_spare: bool,
    page_buffer: Vec<u8>,
    pub spare_buffer: Vec<u8>,
    buffered_bank: i64,
    buffered_page: i64,
    pub reading_multiple_pages: bool,
    pub cur_bank_reading: i64,
    pub banks_to_read: Vec<u32>,
    pub pages_to_read: Vec<u32>,
}

impl S5lNand {
    pub fn new(nand_path: Option<std::path::PathBuf>) -> Self {
        S5lNand {
            nand_path,
            fmctrl0: 0,
            fmctrl1: 0,
            fmaddr0: 0,
            fmaddr1: 0,
            fmanum: 0,
            fmdnum: 0,
            rsctrl: 0,
            cmd: 0,
            reading_spare: false,
            page_buffer: vec![0u8; NAND_BYTES_PER_PAGE],
            spare_buffer: vec![0u8; NAND_BYTES_PER_SPARE],
            buffered_bank: -1,
            buffered_page: -1,
            reading_multiple_pages: false,
            cur_bank_reading: -1,
            banks_to_read: vec![0u32; 512],
            pages_to_read: vec![0u32; 512],
        }
    }

    fn active_bank(&self) -> i64 {
        let bitmap = (self.fmctrl0 >> 1) & 0xFF;
        for b in 0..NAND_NUM_BANKS {
            if bitmap & (1 << b) != 0 {
                return b as i64;
            }
        }
        -1
    }

    /// Activate exactly one bank (clear all bank-select bits, set `bank`).
    pub fn set_bank(&mut self, bank: u32) {
        for b in 0..8 {
            self.fmctrl0 &= !(1 << (b + 1));
        }
        self.fmctrl0 |= 1 << (bank + 1);
    }

    /// Load `<nand>/bank<bank>/<page>.page` into the page+spare buffers for the
    /// currently active bank (no-op if already buffered).
    pub fn set_buffered_page(&mut self, page: u32) {
        let bank = self.active_bank();
        if bank < 0 {
            return;
        }
        if bank == self.buffered_bank && page as i64 == self.buffered_page {
            return;
        }
        self.page_buffer.iter_mut().for_each(|b| *b = 0);
        self.spare_buffer.iter_mut().for_each(|b| *b = 0);
        if let Some(dir) = &self.nand_path {
            let path = dir.join(format!("bank{bank}/{page}.page"));
            if let Ok(data) = std::fs::read(&path) {
                tracing::debug!(bank, page, len = data.len(), "nand page hit");
                let n = data.len().min(NAND_BYTES_PER_PAGE);
                self.page_buffer[..n].copy_from_slice(&data[..n]);
                if data.len() > NAND_BYTES_PER_PAGE {
                    let s = (data.len() - NAND_BYTES_PER_PAGE).min(NAND_BYTES_PER_SPARE);
                    self.spare_buffer[..s]
                        .copy_from_slice(&data[NAND_BYTES_PER_PAGE..NAND_BYTES_PER_PAGE + s]);
                }
            } else {
                tracing::debug!(bank, page, "nand page MISS (erased)");
                // Missing page: mark the FTL byte so empty pages look erased.
                self.spare_buffer[0xA] = 0xFF;
            }
        } else {
            self.spare_buffer[0xA] = 0xFF;
        }
        self.buffered_bank = bank;
        self.buffered_page = page as i64;
    }

    fn page_word(&self, idx: usize) -> u32 {
        let o = idx * 4;
        if o + 4 <= self.page_buffer.len() {
            u32::from_le_bytes(self.page_buffer[o..o + 4].try_into().unwrap())
        } else {
            0
        }
    }

    fn spare_word(&self, idx: usize) -> u32 {
        let o = idx * 4;
        if o + 4 <= self.spare_buffer.len() {
            u32::from_le_bytes(self.spare_buffer[o..o + 4].try_into().unwrap())
        } else {
            0
        }
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            0x0 => self.fmctrl0,
            0x80 => {
                // FMFIFO: drains the buffered page word-by-word.
                if self.cmd == NAND_CMD_ID {
                    return NAND_CHIP_ID;
                }
                if self.cmd == NAND_CMD_READSTATUS {
                    return 1 << 6;
                }
                let read_val;
                if self.reading_multiple_pages {
                    if self.fmdnum % 0x800 == 0 {
                        self.cur_bank_reading += 1;
                        let idx = self.cur_bank_reading.max(0) as usize;
                        let bank = *self.banks_to_read.get(idx).unwrap_or(&0);
                        self.set_bank(bank);
                    }
                    let mut page_offset = self.fmdnum % 0x800;
                    if page_offset == 0 {
                        page_offset = 0x800;
                    }
                    let idx = self.cur_bank_reading.max(0) as usize;
                    let page = *self.pages_to_read.get(idx).unwrap_or(&0);
                    self.set_buffered_page(page);
                    read_val = self.page_word((NAND_BYTES_PER_PAGE - page_offset as usize) / 4);
                } else {
                    let page = (self.fmaddr1 << 16) | (self.fmaddr0 >> 16);
                    self.set_buffered_page(page);
                    if self.reading_spare {
                        read_val = self
                            .spare_word((NAND_BYTES_PER_SPARE - self.fmdnum as usize - 1) / 4);
                    } else {
                        read_val =
                            self.page_word((NAND_BYTES_PER_PAGE - self.fmdnum as usize - 1) / 4);
                    }
                }
                self.fmdnum = self.fmdnum.wrapping_sub(4);
                read_val
            }
            // FMCSTAT: everything ready, all eight banks present.
            0x48 => (1 << 1)
                | (1 << 2)
                | (1 << 3)
                | (1 << 4)
                | (1 << 5)
                | (1 << 6)
                | (1 << 7)
                | (1 << 8)
                | (1 << 9)
                | (1 << 10)
                | (1 << 11)
                | (1 << 12),
            0x100 => self.rsctrl,
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, val: u32) {
        match offset {
            0x0 => self.fmctrl0 = val,
            0x4 => self.fmctrl1 = val,
            0x8 => self.cmd = val,
            0xC => self.fmaddr0 = val,
            0x10 => self.fmaddr1 = val,
            0x2C => self.fmanum = val,
            0x30 => {
                self.reading_spare = val == NAND_BYTES_PER_SPARE as u32 - 1;
                self.fmdnum = val;
            }
            0x100 => self.rsctrl = val,
            _ => {}
        }
    }
}

/// S5L8900 NAND ECC engine. Always reports success; raises an IRQ on START.
pub struct S5lNandEcc {
    irq: bool,
}

impl S5lNandEcc {
    pub fn new() -> Self {
        S5lNandEcc { irq: false }
    }

    pub fn irq_pending(&self) -> bool {
        self.irq
    }

    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            0x10 => 0, // STATUS: success
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, _val: u32) {
        match offset {
            0xC => self.irq = true,  // START
            0x40 => self.irq = false, // CLEARINT
            _ => {}
        }
    }
}

impl Default for S5lNandEcc {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// S5L8900 AES crypto engine (MMIO @ 0x38C00000)
// =============================================================================

/// Which hardware key the AES engine decrypts with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AesKeyType {
    /// Caller-supplied key written to the key registers.
    Custom,
    /// Per-device-class GID key (used for LLB/iBoot/kernelcache images).
    Gid,
    /// Per-device UID key.
    Uid,
}

/// The S5L8900 on-die AES block. iBoot programs the source/destination
/// addresses, the input size, the key selector, the key (for Custom) and the
/// IV through MMIO registers, then writes `AES_GO` to perform an in-memory
/// AES-CBC decrypt. The DMA (reading `inaddr` / writing `outaddr` in guest
/// physical memory) is performed by the vCPU step, which holds memory access;
/// this struct only holds the programmed register state. Register layout and
/// the placeholder UID key match the QEMU `s5l8900` reference port.
pub struct S5lAes {
    pub inaddr: u32,
    pub outaddr: u32,
    pub insize: u32,
    pub outsize: u32,
    pub keytype: AesKeyType,
    pub keylen: u32,
    /// 0 = decrypt, 1 = encrypt (set on the second AES_KEYLEN write).
    pub operation: u32,
    pub custkey: [u8; 32],
    pub ivec: [u8; 16],
    pub status: u32,
    /// Set when AES_GO is written; consumed by the vCPU step.
    pub pending_go: bool,
    keylen_writes: u8,
}

// Register offsets (QEMU s5l8900 reference).
const AES_GO: u32 = 0x4;
const AES_STATUS: u32 = 0xC;
const AES_KEYLEN: u32 = 0x14;
const AES_INSIZE: u32 = 0x18;
const AES_INADDR: u32 = 0x20;
const AES_OUTSIZE: u32 = 0x24;
const AES_OUTADDR: u32 = 0x28;
const AES_KEY_REG: u32 = 0x4C;
const AES_KEYSIZE: u32 = 0x20;
const AES_TYPE: u32 = 0x6C;
const AES_IV_REG: u32 = 0x74;
const AES_IVSIZE: u32 = 0x10;

/// QEMU placeholder UID key. Real per-device UID keys are not recoverable; a
/// real GID key may be supplied via `RAX_S5L_GID_KEY` for the engine.
pub const AES_UID_KEY: [u8; 16] = [
    0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
];

impl S5lAes {
    pub fn new() -> Self {
        S5lAes {
            inaddr: 0,
            outaddr: 0,
            insize: 0,
            outsize: 0,
            keytype: AesKeyType::Custom,
            keylen: 0,
            operation: 0,
            custkey: [0; 32],
            ivec: [0; 16],
            status: 0,
            pending_go: false,
            keylen_writes: 0,
        }
    }

    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            AES_STATUS => self.status,
            AES_OUTSIZE => self.outsize,
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            AES_GO => self.pending_go = true,
            AES_KEYLEN => {
                if self.keylen_writes == 1 {
                    self.operation = value;
                }
                self.keylen_writes = self.keylen_writes.wrapping_add(1);
                self.keylen = value;
            }
            AES_INADDR => self.inaddr = value,
            AES_INSIZE => self.insize = value,
            AES_OUTSIZE => self.outsize = value,
            AES_OUTADDR => self.outaddr = value,
            AES_TYPE => {
                self.keytype = match value {
                    1 => AesKeyType::Gid,
                    2 => AesKeyType::Uid,
                    _ => AesKeyType::Custom,
                };
            }
            o if (AES_KEY_REG..AES_KEY_REG + AES_KEYSIZE).contains(&o) => {
                let idx = ((o - AES_KEY_REG) / 4) as usize * 4;
                let b = value.to_le_bytes();
                for (i, bi) in b.iter().enumerate() {
                    self.custkey[idx + i] |= bi;
                }
            }
            o if (AES_IV_REG..AES_IV_REG + AES_IVSIZE).contains(&o) => {
                let idx = ((o - AES_IV_REG) / 4) as usize * 4;
                let b = value.to_le_bytes();
                for (i, bi) in b.iter().enumerate() {
                    self.ivec[idx + i] |= bi;
                }
            }
            _ => {}
        }
    }

    /// Clear the volatile key/IV state after a GO (matches the hardware, which
    /// wipes the key registers each operation).
    pub fn finish(&mut self) {
        self.custkey = [0; 32];
        self.ivec = [0; 16];
        self.keylen_writes = 0;
        self.status = 0xf;
    }
}

impl Default for S5lAes {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// PCF50633 Power Management Unit (I2C slave at address 0x73)
// =============================================================================

/// PCF50633 PMU. iBoot reads PMU registers over I2C to decide power/boot
/// behaviour — notably register 0x67, which gates the serial console.
pub struct Pcf50633 {
    /// Current register index; auto-increments on each read.
    cmd: u8,
}

impl Pcf50633 {
    pub fn new() -> Self {
        Pcf50633 { cmd: 0 }
    }

    /// I2C write: the first byte selects the register index.
    pub fn send(&mut self, data: u8) {
        self.cmd = data;
    }

    /// I2C read: return the current register's value, then post-increment.
    pub fn recv(&mut self) -> u8 {
        let res: u8 = match self.cmd {
            0x67 => 1, // enable the debug UARTs (serial console)
            _ => 0,    // battery/charge/RTC registers: 0 is acceptable early
        };
        if std::env::var("RAX_S5L_PMULOG").is_ok() {
            eprintln!("PMU recv reg {:#x} -> {res:#x}", self.cmd);
        }
        self.cmd = self.cmd.wrapping_add(1);
        res
    }
}

impl Default for Pcf50633 {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// I2C controller (Samsung S5L IIC)
// =============================================================================

const IICCON_ACKEN: u8 = 1 << 7;
const IICSTAT_START: u8 = 1 << 5;
const IICSTAT_TXRXEN: u8 = 1 << 4;
const IICSTAT_LASTBIT: u8 = 1 << 0;

/// S5L8900 I2C master controller. Models enough of the register protocol for
/// iBoot's polled PMU transactions: the `iicreg20` "transfer active" flag the
/// firmware spins on, and data routing to the attached PMU slave.
pub struct S5lI2c {
    control: u8,
    status: u8,
    address: u8,
    data: u8,
    line_ctrl: u8,
    iicreg20: u32,
    active: bool,
    addressed: u8,
    /// The PMU slave (present on I2C1; None on buses without one).
    pmu: Option<Pcf50633>,
}

impl S5lI2c {
    pub fn new(pmu: bool) -> Self {
        S5lI2c {
            control: 0,
            status: 0,
            address: 0,
            data: 0,
            line_ctrl: 0,
            iicreg20: 0,
            active: false,
            addressed: 0,
            pmu: pmu.then(Pcf50633::new),
        }
    }

    fn slave_recv(&mut self) -> u8 {
        match (self.addressed, self.pmu.as_mut()) {
            (0x73, Some(pmu)) => pmu.recv(),
            _ => 0,
        }
    }

    fn slave_send(&mut self, data: u8) {
        if let (0x73, Some(pmu)) = (self.addressed, self.pmu.as_mut()) {
            pmu.send(data);
        }
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            0x00 => self.control as u32,
            0x04 => self.status as u32,
            0x08 => self.address as u32,
            0x0c => {
                // I2CDS read: fetch the next byte from the slave.
                self.iicreg20 |= 0x100;
                self.data = self.slave_recv();
                self.data as u32
            }
            0x10 => self.line_ctrl as u32,
            0x20 => {
                // IICREG20: transfer-status flags, cleared on read.
                let tmp = self.iicreg20;
                self.iicreg20 &= !0x100;
                self.iicreg20 &= !0x2000;
                tmp
            }
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        let v = value as u8;
        match offset {
            0x00 => {
                // I2CCON
                if value & !(IICCON_ACKEN as u32) != 0 {
                    self.iicreg20 |= 0x100;
                }
                if (value & 0x10) != 0 && self.status == 0x90 {
                    self.iicreg20 |= 0x2000;
                }
                self.control = v;
            }
            0x04 => {
                // I2CSTAT: mode + start/stop control.
                let mode = (self.status >> 6) & 0x3;
                if (value as u8) & IICSTAT_TXRXEN != 0 {
                    match mode {
                        0 | 1 => {
                            // Slave receive/transmit: pull a byte.
                            self.data = self.slave_recv();
                        }
                        2 | 3 => {
                            if (value as u8) & IICSTAT_START != 0 {
                                self.status &= !IICSTAT_LASTBIT;
                                self.iicreg20 |= 0x100;
                                self.active = true;
                                self.addressed = self.data >> 1;
                            } else {
                                self.active = false;
                                self.status |= IICSTAT_TXRXEN;
                            }
                        }
                        _ => {}
                    }
                }
                self.status = v;
            }
            0x08 => self.address = v,
            0x0c => {
                // I2CDS write: send a byte to the slave.
                self.iicreg20 |= 0x100;
                self.data = v;
                self.slave_send(v);
            }
            0x10 => self.line_ctrl = v,
            _ => {}
        }
    }
}

impl Default for S5lI2c {
    fn default() -> Self {
        Self::new(false)
    }
}

// =============================================================================
// SPI controller (Apple/S5L SPI) + attached peripherals
// =============================================================================

/// An SSI peripheral attached to an SPI bus. `transfer` exchanges one byte.
enum SpiPeripheral {
    /// No device — reads back zero.
    None,
    /// The LCD panel (on SPI1): responds to ID/info commands.
    LcdPanel { cur_cmd: u8 },
    /// The multitouch controller (on SPI2): stubbed (returns zero).
    Multitouch,
}

impl SpiPeripheral {
    fn transfer(&mut self, value: u8) -> u8 {
        match self {
            SpiPeripheral::None | SpiPeripheral::Multitouch => 0,
            SpiPeripheral::LcdPanel { cur_cmd } => {
                if *cur_cmd == 0
                    && matches!(value, 0x95 | 0xDA | 0xDB | 0xDC)
                {
                    *cur_cmd = value;
                    return 0;
                }
                if *cur_cmd != 0 {
                    let res = match *cur_cmd {
                        0x95 => 0x01,
                        0xDA => 0x71, // panel ID byte 0
                        0xDB => 0xC2, // panel ID byte 1
                        0xDC => 0x00,
                        _ => 0,
                    };
                    *cur_cmd = 0;
                    return res;
                }
                0
            }
        }
    }
}

const SPI_CTRL: u32 = 0x000;
const SPI_CFG: u32 = 0x004;
const SPI_STATUS: u32 = 0x008;
const SPI_PIN: u32 = 0x00c;
const SPI_TXDATA: u32 = 0x010;
const SPI_RXDATA: u32 = 0x020;
const SPI_RXCNT: u32 = 0x034;

const CTRL_RUN: u32 = 1 << 0;
const CTRL_TX_RESET: u32 = 1 << 2;
const CTRL_RX_RESET: u32 = 1 << 3;
const CFG_AGD: u32 = 1 << 0;
const CFG_IE_RXREADY: u32 = 1 << 7;
const CFG_IE_TXEMPTY: u32 = 1 << 8;
const CFG_IE_COMPLETE: u32 = 1 << 21;
const STATUS_RXREADY: u32 = 1 << 0;
const STATUS_TXEMPTY: u32 = 1 << 1;
const STATUS_COMPLETE: u32 = 1 << 22;
const STATUS_TXFIFO_SHIFT: u32 = 4;
const STATUS_RXFIFO_SHIFT: u32 = 8;
const STATUS_TXFIFO_MASK: u32 = 31 << STATUS_TXFIFO_SHIFT;
const STATUS_RXFIFO_MASK: u32 = 31 << STATUS_RXFIFO_SHIFT;

/// Apple/S5L SPI master. iBoot drives it in polled mode: reset FIFOs, push TX
/// bytes, set RXCNT, RUN, then poll STATUS for COMPLETE and drain RXDATA.
pub struct S5lSpi {
    regs: [u32; 64],
    tx: VecDeque<u8>,
    rx: VecDeque<u8>,
    peripheral: SpiPeripheral,
}

impl S5lSpi {
    /// `index` selects the attached peripheral (0=none, 1=LCD panel,
    /// 2=multitouch), matching the S5L8900 machine wiring.
    pub fn new(index: u8) -> Self {
        let peripheral = match index {
            1 => SpiPeripheral::LcdPanel { cur_cmd: 0 },
            2 => SpiPeripheral::Multitouch,
            _ => SpiPeripheral::None,
        };
        S5lSpi {
            regs: [0; 64],
            tx: VecDeque::new(),
            rx: VecDeque::new(),
            peripheral,
        }
    }

    fn word_size(&self) -> usize {
        match (self.regs[(SPI_CFG >> 2) as usize] >> 13) & 0x3 {
            0 => 1,
            1 => 2,
            2 => 4,
            _ => 1,
        }
    }

    fn run(&mut self) {
        let ctrl_i = (SPI_CTRL >> 2) as usize;
        let status_i = (SPI_STATUS >> 2) as usize;
        let cfg_i = (SPI_CFG >> 2) as usize;
        let rxcnt_i = (SPI_RXCNT >> 2) as usize;
        if self.regs[ctrl_i] & CTRL_RUN == 0 {
            return;
        }
        while let Some(tx) = self.tx.pop_front() {
            let rx = self.peripheral.transfer(tx);
            if self.tx.is_empty() {
                self.regs[status_i] |= STATUS_TXEMPTY;
            }
            if self.regs[rxcnt_i] > 0 {
                self.rx.push_back(rx);
                self.regs[rxcnt_i] -= 1;
                self.regs[status_i] |= STATUS_RXREADY;
            }
        }
        // Auto-get-data: fetch the remaining receive bytes with sentinels.
        while self.regs[rxcnt_i] > 0 && self.regs[cfg_i] & CFG_AGD != 0 {
            let rx = self.peripheral.transfer(0xff);
            self.rx.push_back(rx);
            self.regs[rxcnt_i] -= 1;
            self.regs[status_i] |= STATUS_RXREADY;
        }
        if self.regs[rxcnt_i] == 0 && self.tx.is_empty() {
            self.regs[status_i] |= STATUS_COMPLETE;
            self.regs[ctrl_i] &= !CTRL_RUN;
        }
    }

    /// SPI interrupt line level: asserted when a status bit whose
    /// interrupt-enable bit is set in CFG is active (matches the QEMU
    /// `apple_spi_update_irq`). The panel-init driver does an IRQ-driven blocking
    /// transfer (enables IE_COMPLETE and sleeps until the controller signals),
    /// so this IRQ is what wakes it.
    pub fn irq_pending(&self) -> bool {
        let cfg = self.regs[(SPI_CFG >> 2) as usize];
        let status = self.regs[(SPI_STATUS >> 2) as usize];
        let mut mask = 0;
        if cfg & CFG_IE_RXREADY != 0 {
            mask |= STATUS_RXREADY;
        }
        if cfg & CFG_IE_TXEMPTY != 0 {
            mask |= STATUS_TXEMPTY;
        }
        if cfg & CFG_IE_COMPLETE != 0 {
            mask |= STATUS_COMPLETE;
        }
        status & mask != 0
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        let idx = (offset >> 2) as usize;
        if idx >= self.regs.len() {
            return 0;
        }
        let mut r = self.regs[idx];
        let mut run = false;
        match offset {
            SPI_RXDATA => {
                let ws = self.word_size();
                let mut bytes = [0u8; 4];
                for b in bytes.iter_mut().take(ws) {
                    *b = self.rx.pop_front().unwrap_or(0);
                }
                r = u32::from_le_bytes(bytes);
                if self.rx.is_empty() {
                    run = true;
                }
            }
            SPI_STATUS => {
                let mut val = (self.tx.len() as u32) << STATUS_TXFIFO_SHIFT;
                val |= (self.rx.len() as u32) << STATUS_RXFIFO_SHIFT;
                val &= STATUS_TXFIFO_MASK | STATUS_RXFIFO_MASK;
                r &= !(STATUS_TXFIFO_MASK | STATUS_RXFIFO_MASK);
                r |= val;
            }
            _ => {}
        }
        if run {
            self.run();
        }
        r
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        let idx = (offset >> 2) as usize;
        if idx >= self.regs.len() {
            return;
        }
        let mut run = false;
        match offset {
            SPI_CTRL => {
                if value & CTRL_TX_RESET != 0 {
                    self.tx.clear();
                }
                if value & CTRL_RX_RESET != 0 {
                    self.rx.clear();
                }
                if value & CTRL_RUN != 0 && !self.tx.is_empty() {
                    run = true;
                }
                // The TX/RX FIFO-reset bits are self-clearing: the panel-init
                // SPI driver sets them and then polls for them to clear before
                // proceeding, so they must read back as 0.
                self.regs[idx] = value & !(CTRL_TX_RESET | CTRL_RX_RESET);
            }
            SPI_STATUS => {
                // Write-1-to-clear.
                self.regs[idx] &= !value;
                run = true;
            }
            SPI_TXDATA..=0x013 => {
                let ws = self.word_size();
                for b in value.to_le_bytes().iter().take(ws) {
                    self.tx.push_back(*b);
                }
                self.regs[idx] = value;
            }
            SPI_CFG => {
                self.regs[idx] = value;
                run = true;
            }
            SPI_PIN => self.regs[idx] = value,
            _ => self.regs[idx] = value,
        }
        if run {
            self.run();
        }
    }
}

// =============================================================================
// LCD controller (register file + framebuffer base)
// =============================================================================

/// S5L8900 LCD controller. Mostly a register file; the window-1 framebuffer
/// base (`0x60`) points at the BGRA framebuffer the display scans out.
pub struct S5lLcd {
    regs: [u32; 0x400],
    /// Periodic vsync/refresh interrupt level (raised by `tick`).
    irq: bool,
    tick_acc: u64,
}

impl S5lLcd {
    pub fn new() -> Self {
        S5lLcd {
            regs: [0; 0x400],
            irq: false,
            tick_acc: 0,
        }
    }

    /// Advance the refresh timer; raise the vsync IRQ roughly periodically.
    pub fn tick(&mut self, n: u64) {
        self.tick_acc = self.tick_acc.wrapping_add(n);
        if self.tick_acc >= 200_000 {
            self.tick_acc = 0;
            self.irq = true;
        }
    }

    pub fn irq_pending(&self) -> bool {
        self.irq
    }

    pub fn framebuffer_base(&self) -> u32 {
        self.regs[0x60 >> 2]
    }

    pub fn read(&self, offset: u32) -> u32 {
        let idx = (offset >> 2) as usize;
        self.regs.get(idx).copied().unwrap_or(0)
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        let idx = (offset >> 2) as usize;
        if idx < self.regs.len() {
            self.regs[idx] = value;
        }
        // Writing the interrupt-ack register (0x18) lowers the vsync IRQ.
        if offset == 0x18 {
            self.irq = false;
        }
    }
}

impl Default for S5lLcd {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// UART (Samsung-style, S5L layout — identical register offsets to s3c64xx)
// =============================================================================

/// S5L8900 UART. Register layout matches the Samsung s3c/exynos UART. iBoot's
/// serial console polls UTRSTAT for TX-ready and writes characters to UTXH.
pub struct S5lUart {
    rx: VecDeque<u8>,
    ulcon: u32,
    ucon: u32,
    ufcon: u32,
    umcon: u32,
    ubrdiv: u32,
    udivslot: u32,
    uintp: u32,
    uintm: u32,
}

impl S5lUart {
    pub fn new() -> Self {
        S5lUart {
            rx: VecDeque::new(),
            ulcon: 0,
            ucon: 0,
            ufcon: 0,
            umcon: 0,
            ubrdiv: 0,
            udivslot: 0,
            uintp: 0,
            uintm: 0xF,
        }
    }

    pub fn queue_input(&mut self, bytes: &[u8]) {
        if !bytes.is_empty() {
            self.rx.extend(bytes);
            self.uintp |= 1;
        }
    }

    pub fn irq_pending(&self) -> bool {
        self.uintp & !self.uintm != 0
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            0x00 => self.ulcon,
            0x04 => self.ucon,
            0x08 => self.ufcon,
            0x0C => self.umcon,
            0x10 => 0x6 | u32::from(!self.rx.is_empty()), // UTRSTAT: TX empty, RX ready
            0x14 => 0,                                    // UERSTAT
            0x18 => self.rx.len().min(63) as u32,         // UFSTAT
            0x1C => 0,                                    // UMSTAT
            0x24 => {
                // URXH
                let b = self.rx.pop_front().unwrap_or(0) as u32;
                if self.rx.is_empty() {
                    self.uintp &= !1;
                }
                b
            }
            0x28 => self.ubrdiv,
            0x2C => self.udivslot,
            0x30 | 0x34 => self.uintp, // UINTP / UINTSP
            0x38 => self.uintm,
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x00 => self.ulcon = value,
            0x04 => self.ucon = value,
            0x08 => self.ufcon = value,
            0x0C => self.umcon = value,
            0x20 => {
                // UTXH
                let _ = io::stdout().write_all(&[value as u8]);
                let _ = io::stdout().flush();
            }
            0x28 => self.ubrdiv = value,
            0x2C => self.udivslot = value,
            0x30 | 0x34 => {
                self.uintp &= !value;
                if !self.rx.is_empty() {
                    self.uintp |= 1;
                }
            }
            0x38 => self.uintm = value,
            _ => {}
        }
    }
}

impl Default for S5lUart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod aes_engine_tests {
    use super::*;
    use crate::devices::crypto::{aes_cbc_decrypt, AesKey};

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    /// Program the AES engine through its MMIO registers exactly as iBoot does
    /// (custom key + IV written word-by-word, type/size set), then confirm the
    /// assembled key/IV decrypt a NIST CBC vector — i.e. the register-decode
    /// path feeds the crypto correctly.
    #[test]
    fn aes_engine_register_protocol_cbc() {
        let key = hex("2b7e151628aed2a6abf7158809cf4f3c"); // AES-128
        let iv = hex("000102030405060708090a0b0c0d0e0f");
        let ct = hex("7649abac8119b246cee98e9b12e9197d");
        let pt = hex("6bc1bee22e409f96e93d7e117393172a");

        let mut aes = S5lAes::new();
        // Type = Custom (0).
        aes.write(AES_TYPE, 0);
        // Custom key: 16 bytes written as 4 little-endian words at AES_KEY_REG.
        for (i, w) in key.chunks_exact(4).enumerate() {
            let v = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
            aes.write(AES_KEY_REG + (i as u32) * 4, v);
        }
        // IV: 16 bytes written as 4 words at AES_IV_REG.
        for (i, w) in iv.chunks_exact(4).enumerate() {
            let v = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
            aes.write(AES_IV_REG + (i as u32) * 4, v);
        }
        aes.write(AES_INSIZE, ct.len() as u32);
        aes.write(AES_GO, 1);
        assert!(aes.pending_go, "AES_GO must mark a pending operation");

        // The assembled custkey/IV must round-trip the vector.
        let k = AesKey::new(&aes.custkey[..16]).unwrap();
        let mut iv16 = [0u8; 16];
        iv16.copy_from_slice(&aes.ivec);
        let mut data = ct.clone();
        aes_cbc_decrypt(&k, &iv16, &mut data);
        assert_eq!(data, pt);

        // finish() wipes the volatile key/IV and reports done.
        aes.finish();
        assert_eq!(aes.custkey, [0u8; 32]);
        assert_eq!(aes.ivec, [0u8; 16]);
        assert_eq!(aes.status, 0xf);
    }

    #[test]
    fn aes_engine_uid_keytype() {
        let mut aes = S5lAes::new();
        aes.write(AES_TYPE, 2);
        assert!(matches!(aes.keytype, AesKeyType::Uid));
        aes.write(AES_TYPE, 1);
        assert!(matches!(aes.keytype, AesKeyType::Gid));
    }
}
