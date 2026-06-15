//! Samsung S3C64xx platform devices for the ARMv6 machine.
//!
//! Enough of the SoC for an `s3c6400_defconfig` kernel booting the
//! `s3c6410-smdk6410` device tree on the software emulator:
//!
//! - [`S3cUart`]: the Samsung SoC UART (`samsung,s3c6400-uart`), console
//!   `ttySAC0`. TX goes to host stdout; RX is fed by the VMM console mux and
//!   raises a VIC line.
//! - [`Pl192Vic`]: ARM PL192 vectored interrupt controller (the kernel's
//!   `irq-vic` driver only uses the status/enable register interface).
//! - [`S3cPwmTimer`]: the PWM timer block used by `samsung_pwm_timer` as
//!   clocksource + clockevents (timer 4 events, timer 3 source).
//! - [`S3cSyscon`]: RAM-backed system controller (PLL/clock registers with
//!   sane reset values, chip ID) for the `samsung,s3c6410-clock` driver.

use std::collections::VecDeque;
use std::io::{self, Write};

// =============================================================================
// Samsung UART (S3C6400 style)
// =============================================================================

const ULCON: u32 = 0x00;
const UCON: u32 = 0x04;
const UFCON: u32 = 0x08;
const UMCON: u32 = 0x0C;
const UTRSTAT: u32 = 0x10;
const UERSTAT: u32 = 0x14;
const UFSTAT: u32 = 0x18;
const UMSTAT: u32 = 0x1C;
const UTXH: u32 = 0x20;
const URXH: u32 = 0x24;
const UBRDIV: u32 = 0x28;
const UDIVSLOT: u32 = 0x2C;
const UINTP: u32 = 0x30;
const UINTSP: u32 = 0x34;
const UINTM: u32 = 0x38;

/// Samsung SoC UART. Interrupt model is the S3C64xx style with the
/// per-UART UINTP/UINTM (pending/mask) registers feeding one VIC line.
pub struct S3cUart {
    rx: VecDeque<u8>,
    ulcon: u32,
    ucon: u32,
    ufcon: u32,
    umcon: u32,
    ubrdiv: u32,
    udivslot: u32,
    /// Interrupt pending (bit0 RXD, bit1 ERROR, bit2 TXD, bit3 MODEM).
    uintp: u32,
    uintm: u32,
}

impl S3cUart {
    pub fn new() -> Self {
        S3cUart {
            rx: VecDeque::new(),
            ulcon: 0,
            ucon: 0,
            ufcon: 0,
            umcon: 0,
            ubrdiv: 0,
            udivslot: 0,
            // TXD pending is level-like: our TX path completes instantly,
            // so "FIFO below trigger" is always true. The driver gates it
            // with UINTM (start_tx unmasks, stop_tx masks).
            uintp: 1 << 2,
            uintm: 0xF,
        }
    }

    pub fn queue_input(&mut self, bytes: &[u8]) {
        if !bytes.is_empty() {
            self.rx.extend(bytes);
            self.uintp |= 1; // RXD pending
        }
    }

    /// Level of the UART's VIC line.
    pub fn irq_pending(&self) -> bool {
        self.uintp & !self.uintm != 0
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            ULCON => self.ulcon,
            UCON => self.ucon,
            UFCON => self.ufcon,
            UMCON => self.umcon,
            // TX buffer + transmitter always empty; RX ready when queued.
            UTRSTAT => 0x6 | u32::from(!self.rx.is_empty()),
            UERSTAT => 0,
            // UFSTAT: RX FIFO count in [5:0] (s3c64xx layout: [6] full).
            UFSTAT => self.rx.len().min(63) as u32,
            UMSTAT => 0,
            URXH => {
                let b = self.rx.pop_front().unwrap_or(0) as u32;
                if self.rx.is_empty() {
                    self.uintp &= !1;
                } else {
                    self.uintp |= 1;
                }
                b
            }
            UBRDIV => self.ubrdiv,
            UDIVSLOT => self.udivslot,
            UINTP => self.uintp,
            UINTSP => self.uintp,
            UINTM => self.uintm,
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            ULCON => self.ulcon = value,
            UCON => self.ucon = value,
            UFCON => self.ufcon = value,
            UMCON => self.umcon = value,
            UTXH => {
                let _ = io::stdout().write_all(&[value as u8]);
                let _ = io::stdout().flush();
                // TX completes instantly; latch TXD pending so irq-driven
                // transmit makes progress when unmasked.
                self.uintp |= 1 << 2;
            }
            UBRDIV => self.ubrdiv = value,
            UDIVSLOT => self.udivslot = value,
            // UINTP/UINTSP: write-1-to-clear; level-style sources re-latch
            // immediately (RX while the queue is non-empty, TX always since
            // the FIFO is always empty).
            UINTP | UINTSP => {
                self.uintp &= !value;
                if !self.rx.is_empty() {
                    self.uintp |= 1;
                }
                self.uintp |= 1 << 2;
            }
            UINTM => self.uintm = value,
            _ => {}
        }
    }
}

impl Default for S3cUart {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// PL192 Vectored Interrupt Controller
// =============================================================================

/// ARM PL192 VIC. The Linux `irq-vic` driver uses IRQSTATUS + the enable
/// registers; vectored operation is not required.
pub struct Pl192Vic {
    raw: u32,
    soft: u32,
    select: u32, // 1 = FIQ
    enable: u32,
}

impl Pl192Vic {
    pub fn new() -> Self {
        Pl192Vic {
            raw: 0,
            soft: 0,
            select: 0,
            enable: 0,
        }
    }

    /// Drive a peripheral input line level.
    pub fn set_line(&mut self, line: u32, level: bool) {
        if level {
            self.raw |= 1 << line;
        } else {
            self.raw &= !(1 << line);
        }
    }

    pub fn irq_asserted(&self) -> bool {
        (self.raw | self.soft) & self.enable & !self.select != 0
    }

    pub fn fiq_asserted(&self) -> bool {
        (self.raw | self.soft) & self.enable & self.select != 0
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            0x000 => (self.raw | self.soft) & self.enable & !self.select, // IRQSTATUS
            0x004 => (self.raw | self.soft) & self.enable & self.select,  // FIQSTATUS
            0x008 => self.raw | self.soft,                                // RAWINTR
            0x00C => self.select,
            0x010 => self.enable,
            0x018 => self.soft,
            // Peripheral/PrimeCell IDs (PL192).
            0xFE0 => 0x92,
            0xFE4 => 0x11,
            0xFE8 => 0x04,
            0xFEC => 0x00,
            0xFF0 => 0x0D,
            0xFF4 => 0xF0,
            0xFF8 => 0x05,
            0xFFC => 0xB1,
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x00C => self.select = value,
            0x010 => self.enable |= value,  // INTENABLE: set bits
            0x014 => self.enable &= !value, // INTENCLEAR
            0x018 => self.soft |= value,
            0x01C => self.soft &= !value,
            _ => {}
        }
    }
}

impl Default for Pl192Vic {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Samsung PWM timer block (clocksource / clockevents)
// =============================================================================

const TCFG0: u32 = 0x00;
const TCFG1: u32 = 0x04;
const TCON: u32 = 0x08;
const TINT_CSTAT: u32 = 0x44;

/// PWM timer block. Timers count DOWN from TCNTB at a rate derived from
/// pclk/prescaler/divider; we advance them from the vCPU's instruction
/// clock via [`Self::tick`]. Timer 4 raises its TINT_CSTAT status (the
/// clockevent); the others mostly serve as the free-running clocksource.
pub struct S3cPwmTimer {
    tcfg0: u32,
    tcfg1: u32,
    tcon: u32,
    tcntb: [u32; 5],
    tcmpb: [u32; 4],
    /// Current down-counters (fixed point: 32.16 fractional ticks).
    tcnt: [u64; 5],
    tint_cstat: u32,
}

impl S3cPwmTimer {
    pub fn new() -> Self {
        S3cPwmTimer {
            tcfg0: 0x0101,
            tcfg1: 0,
            tcon: 0,
            tcntb: [0; 5],
            tcmpb: [0; 4],
            tcnt: [0; 5],
            tint_cstat: 0,
        }
    }

    #[inline]
    fn start_bit(timer: usize) -> u32 {
        match timer {
            0 => 0,
            1 => 8,
            2 => 12,
            3 => 16,
            _ => 20,
        }
    }

    /// Advance all running timers by `ticks` timer-input cycles (we use a
    /// fixed ratio from the instruction clock; precise PLL math is not
    /// needed for a functional clocksource).
    pub fn tick(&mut self, ticks: u64) {
        for t in 0..5 {
            let start = Self::start_bit(t);
            if (self.tcon >> start) & 1 == 0 {
                continue; // not started
            }
            let reload = (self.tcon >> (start + 3)) & 1 == 1
                || (t == 0 && (self.tcon >> 3) & 1 == 1);
            let mut remaining = self.tcnt[t] >> 16;
            let mut budget = ticks;
            loop {
                if remaining >= budget {
                    remaining -= budget;
                    break;
                }
                budget -= remaining;
                // expiry
                if t == 4 {
                    self.tint_cstat |= 1 << (5 + 4); // status bit for timer 4
                }
                if t < 4 {
                    self.tint_cstat |= 1 << (5 + t as u32);
                }
                let rl = self.tcntb[t] as u64;
                if !reload || rl == 0 {
                    remaining = 0;
                    // auto-stop without reload
                    self.tcon &= !(1 << start);
                    break;
                }
                remaining = rl;
            }
            self.tcnt[t] = remaining << 16;
        }
    }

    /// Any enabled timer interrupt pending? (One VIC line per timer; the
    /// caller maps them.)
    pub fn irq_pending(&self, timer: usize) -> bool {
        let ena = (self.tint_cstat >> timer) & 1 == 1;
        let sta = (self.tint_cstat >> (5 + timer)) & 1 == 1;
        ena && sta
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            TCFG0 => self.tcfg0,
            TCFG1 => self.tcfg1,
            TCON => self.tcon,
            // TCNTB / TCMPB / TCNTO per timer.
            0x0C => self.tcntb[0],
            0x10 => self.tcmpb[0],
            0x14 => (self.tcnt[0] >> 16) as u32,
            0x18 => self.tcntb[1],
            0x1C => self.tcmpb[1],
            0x20 => (self.tcnt[1] >> 16) as u32,
            0x24 => self.tcntb[2],
            0x28 => self.tcmpb[2],
            0x2C => (self.tcnt[2] >> 16) as u32,
            0x30 => self.tcntb[3],
            0x34 => self.tcmpb[3],
            0x38 => (self.tcnt[3] >> 16) as u32,
            0x3C => self.tcntb[4],
            0x40 => (self.tcnt[4] >> 16) as u32,
            TINT_CSTAT => self.tint_cstat,
            _ => 0,
        }
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            TCFG0 => self.tcfg0 = value,
            TCFG1 => self.tcfg1 = value,
            TCON => {
                // Manual update bits copy TCNTB into the live counter.
                for t in 0..5usize {
                    let start = Self::start_bit(t);
                    let manual = match t {
                        0 => (value >> 1) & 1 == 1,
                        _ => (value >> (start + 1)) & 1 == 1,
                    };
                    if manual {
                        self.tcnt[t] = (self.tcntb[t] as u64) << 16;
                    }
                }
                self.tcon = value;
            }
            0x0C => self.tcntb[0] = value,
            0x10 => self.tcmpb[0] = value,
            0x18 => self.tcntb[1] = value,
            0x1C => self.tcmpb[1] = value,
            0x24 => self.tcntb[2] = value,
            0x28 => self.tcmpb[2] = value,
            0x30 => self.tcntb[3] = value,
            0x34 => self.tcmpb[3] = value,
            0x3C => self.tcntb[4] = value,
            TINT_CSTAT => {
                // [4:0] enables (RW), [9:5] status (write-1-to-clear).
                let ena = value & 0x1F;
                let clr = (value >> 5) & 0x1F;
                self.tint_cstat = (self.tint_cstat & !(0x1F) & !(clr << 5)) | ena;
            }
            _ => {}
        }
    }
}

impl Default for S3cPwmTimer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// System controller (clock/PLL block + chip ID)
// =============================================================================

/// RAM-backed SYSCON at 0x7E00F000 with reset values the
/// `samsung,s3c6410-clock` driver can work with (fin = 12 MHz).
pub struct S3cSyscon {
    regs: Vec<u32>,
}

impl S3cSyscon {
    pub fn new() -> Self {
        let mut regs = vec![0u32; 0x1000 / 4];
        // APLL/MPLL: enabled, 532 MHz from 12 MHz fin (m=266, p=3, s=1).
        regs[0x0C / 4] = 0x810A_0301;
        regs[0x10 / 4] = 0x810A_0301;
        // EPLL: ~96 MHz (m=32, p=1, s=2) + EPLL_CON1 = 0.
        regs[0x14 / 4] = 0x8020_0102;
        regs[0x18 / 4] = 0;
        // CLK_SRC: APLL/MPLL/EPLL selected as PLL outputs.
        regs[0x1C / 4] = 0x7;
        // CLK_DIV0: sensible post-dividers (ARM /1, HCLKx2 /2, HCLK /2,
        // PCLK /4 in the fields the driver reads).
        regs[0x20 / 4] = 0x0105_1000;
        // Chip ID (S3C6410 rev 1).
        regs[0x118 / 4] = 0x3641_0101;
        S3cSyscon { regs }
    }

    pub fn read(&mut self, offset: u32) -> u32 {
        *self.regs.get((offset / 4) as usize).unwrap_or(&0)
    }

    pub fn write(&mut self, offset: u32, value: u32) {
        // Chip ID is read-only; everything else is writable RAM.
        if offset == 0x118 {
            return;
        }
        if let Some(slot) = self.regs.get_mut((offset / 4) as usize) {
            *slot = value;
        }
    }
}

impl Default for S3cSyscon {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uart_rx_irq_flow() {
        let mut u = S3cUart::new();
        assert!(!u.irq_pending());
        u.queue_input(b"a");
        assert!(u.irq_pending() == (u.uintm & 1 == 0));
        u.write(UINTM, 0xE); // unmask RXD
        assert!(u.irq_pending());
        assert_eq!(u.read(URXH), u32::from(b'a'));
        assert!(!u.irq_pending());
        assert_eq!(u.read(UTRSTAT) & 1, 0);
    }

    #[test]
    fn vic_enable_and_status() {
        let mut v = Pl192Vic::new();
        v.set_line(5, true);
        assert!(!v.irq_asserted());
        v.write(0x010, 1 << 5);
        assert!(v.irq_asserted());
        assert_eq!(v.read(0x000), 1 << 5);
        v.write(0x014, 1 << 5);
        assert!(!v.irq_asserted());
    }

    #[test]
    fn pwm_timer4_event() {
        let mut t = S3cPwmTimer::new();
        t.write(0x3C, 100); // TCNTB4
        t.write(TCON, 1 << 21); // manual update timer4
        t.write(TCON, (1 << 20) | (1 << 22)); // start + auto-reload
        t.write(TINT_CSTAT, 1 << 4); // enable timer4 int
        t.tick(50);
        assert!(!t.irq_pending(4));
        t.tick(60);
        assert!(t.irq_pending(4));
        // ack
        t.write(TINT_CSTAT, (1 << 4) | (1 << 9));
        assert!(!t.irq_pending(4));
    }
}
