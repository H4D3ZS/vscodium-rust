//! ARM PrimeCell PL011 UART.
//!
//! Minimal-but-honest PL011 model for the AArch64 machine: enough for the
//! Linux `amba-pl011` driver (and `earlycon=pl011`) to probe and run an
//! interactive console. TX bytes go straight to host stdout (same as the
//! 16550 model); RX bytes are queued by the VMM's console mux.
//!
//! The transmitter is always ready (FIFO never fills), so the TX interrupt
//! is raised whenever it is unmasked. The RX interrupt is level: asserted
//! while the RX queue is non-empty and unmasked.

use std::collections::VecDeque;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use super::bus::MmioDevice;

// Register offsets (byte offsets from the 4KB APB window base).
const UARTDR: u64 = 0x000;
const UARTRSR_ECR: u64 = 0x004;
const UARTFR: u64 = 0x018;
const UARTILPR: u64 = 0x020;
const UARTIBRD: u64 = 0x024;
const UARTFBRD: u64 = 0x028;
const UARTLCR_H: u64 = 0x02C;
const UARTCR: u64 = 0x030;
const UARTIFLS: u64 = 0x034;
const UARTIMSC: u64 = 0x038;
const UARTRIS: u64 = 0x03C;
const UARTMIS: u64 = 0x040;
const UARTICR: u64 = 0x044;
const UARTDMACR: u64 = 0x048;

// Flag register bits.
const FR_RXFE: u32 = 1 << 4; // RX FIFO empty
const FR_TXFE: u32 = 1 << 7; // TX FIFO empty

// Interrupt bits (RIS/MIS/IMSC/ICR).
const INT_RX: u32 = 1 << 4;
const INT_TX: u32 = 1 << 5;

/// AMBA PrimeCell identification registers at 0xFE0..0xFFF: PL011 (r1p5)
/// peripheral ID followed by the PrimeCell cell ID. The Linux amba bus reads
/// these to match the driver, one byte per word.
const AMBA_IDS: [u8; 8] = [0x11, 0x10, 0x14, 0x00, 0x0D, 0xF0, 0x05, 0xB1];

/// PL011 UART device state.
pub struct Pl011 {
    /// Received bytes waiting for the guest.
    rx_queue: VecDeque<u8>,
    /// Interrupt mask (UARTIMSC).
    imsc: u32,
    /// Control register (UARTCR).
    cr: u32,
    /// Line control (UARTLCR_H).
    lcr_h: u32,
    /// Integer/fractional baud divisors (latched, otherwise unused).
    ibrd: u32,
    fbrd: u32,
    /// FIFO level select.
    ifls: u32,
    /// DMA control (unused).
    dmacr: u32,
}

impl Pl011 {
    pub fn new() -> Self {
        Pl011 {
            rx_queue: VecDeque::new(),
            imsc: 0,
            cr: 0x300, // TXE | RXE (reset value per TRM)
            lcr_h: 0,
            ibrd: 0,
            fbrd: 0,
            ifls: 0x12,
            dmacr: 0,
        }
    }

    /// Queue host console input for the guest.
    pub fn queue_input(&mut self, bytes: &[u8]) {
        self.rx_queue.extend(bytes);
    }

    /// Raw interrupt status.
    fn ris(&self) -> u32 {
        let mut ris = INT_TX; // transmitter is always empty
        if !self.rx_queue.is_empty() {
            ris |= INT_RX;
        }
        ris
    }

    /// Masked interrupt status: drives the UART's SPI line (level).
    pub fn irq_pending(&self) -> bool {
        self.ris() & self.imsc != 0
    }

    fn read_reg(&mut self, offset: u64) -> u32 {
        match offset {
            UARTDR => self.rx_queue.pop_front().map(u32::from).unwrap_or(0),
            UARTRSR_ECR => 0,
            UARTFR => {
                let mut fr = FR_TXFE;
                if self.rx_queue.is_empty() {
                    fr |= FR_RXFE;
                }
                fr
            }
            UARTILPR => 0,
            UARTIBRD => self.ibrd,
            UARTFBRD => self.fbrd,
            UARTLCR_H => self.lcr_h,
            UARTCR => self.cr,
            UARTIFLS => self.ifls,
            UARTIMSC => self.imsc,
            UARTRIS => self.ris(),
            UARTMIS => self.ris() & self.imsc,
            UARTDMACR => self.dmacr,
            0xFE0..=0xFFC => {
                let idx = ((offset - 0xFE0) >> 2) as usize;
                AMBA_IDS.get(idx).copied().map(u32::from).unwrap_or(0)
            }
            _ => 0,
        }
    }

    /// Current value of a state register, without read side effects. Used to
    /// fold partial-width bus writes into the full register.
    fn state_reg(&self, offset: u64) -> u32 {
        match offset {
            UARTIBRD => self.ibrd,
            UARTFBRD => self.fbrd,
            UARTLCR_H => self.lcr_h,
            UARTCR => self.cr,
            UARTIFLS => self.ifls,
            UARTIMSC => self.imsc,
            UARTDMACR => self.dmacr,
            _ => 0,
        }
    }

    fn write_reg(&mut self, offset: u64, value: u32) {
        match offset {
            UARTDR => {
                let byte = value as u8;
                let _ = io::stdout().write_all(&[byte]);
                let _ = io::stdout().flush();
            }
            UARTIBRD => self.ibrd = value & 0xFFFF,
            UARTFBRD => self.fbrd = value & 0x3F,
            UARTLCR_H => self.lcr_h = value & 0xFF,
            UARTCR => self.cr = value & 0xFFFF,
            UARTIFLS => self.ifls = value & 0x3F,
            UARTIMSC => self.imsc = value & 0x7FF,
            // ICR clears edge-latched sources; ours are level-derived, so the
            // write is accepted and status recomputes from current state.
            UARTICR => {}
            UARTDMACR => self.dmacr = value & 0x7,
            _ => {}
        }
    }
}

impl Default for Pl011 {
    fn default() -> Self {
        Self::new()
    }
}

/// MMIO adapter exposing a shared [`Pl011`] on the bus.
pub struct Pl011MmioDevice {
    base: u64,
    inner: Arc<Mutex<Pl011>>,
}

impl Pl011MmioDevice {
    pub fn new(base: u64, inner: Arc<Mutex<Pl011>>) -> Self {
        Pl011MmioDevice { base, inner }
    }
}

impl MmioDevice for Pl011MmioDevice {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        // The bus may dispatch byte-at-a-time; reconstruct the register value
        // and serve the byte lane being asked for. Registers with read side
        // effects (DR) are only popped on the lane-0 access.
        let offset = addr - self.base;
        let reg = offset & !0x3;
        let lane = (offset & 0x3) as usize;
        let Ok(mut uart) = self.inner.lock() else {
            data.fill(0);
            return;
        };
        let value = if reg == UARTDR && lane != 0 {
            0
        } else {
            uart.read_reg(reg)
        };
        let bytes = value.to_le_bytes();
        for (i, b) in data.iter_mut().enumerate() {
            *b = bytes.get(lane + i).copied().unwrap_or(0);
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        let offset = addr - self.base;
        let reg = offset & !0x3;
        let lane = (offset & 0x3) as usize;
        let Ok(mut uart) = self.inner.lock() else {
            return;
        };
        // DR is write-to-transmit: only the low byte lane carries data.
        if reg == UARTDR {
            if lane == 0 {
                if let Some(&byte) = data.first() {
                    uart.write_reg(UARTDR, u32::from(byte));
                }
            }
            return;
        }
        // State registers: fold the written lanes into the current value so
        // both whole-word writes and the bus's byte-split dispatch work.
        let mut value = uart.state_reg(reg);
        for (i, b) in data.iter().enumerate() {
            let l = lane + i;
            if l < 4 {
                value = (value & !(0xFF << (8 * l))) | (u32::from(*b) << (8 * l));
            }
        }
        uart.write_reg(reg, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amba_ids_probe() {
        let mut uart = Pl011::new();
        assert_eq!(uart.read_reg(0xFE0), 0x11);
        assert_eq!(uart.read_reg(0xFE8), 0x14);
        assert_eq!(uart.read_reg(0xFF0), 0x0D);
        assert_eq!(uart.read_reg(0xFFC), 0xB1);
    }

    #[test]
    fn rx_flow_and_interrupt() {
        let mut uart = Pl011::new();
        assert!(!uart.irq_pending());
        assert_eq!(uart.read_reg(UARTFR) & FR_RXFE, FR_RXFE);

        uart.queue_input(b"a");
        // Unmasked: no interrupt yet.
        assert!(!uart.irq_pending());
        uart.write_reg(UARTIMSC, INT_RX);
        assert!(uart.irq_pending());
        assert_eq!(uart.read_reg(UARTFR) & FR_RXFE, 0);

        assert_eq!(uart.read_reg(UARTDR), u32::from(b'a'));
        assert!(!uart.irq_pending());
        assert_eq!(uart.read_reg(UARTFR) & FR_RXFE, FR_RXFE);
    }

    #[test]
    fn tx_interrupt_when_unmasked() {
        let mut uart = Pl011::new();
        assert_eq!(uart.read_reg(UARTRIS) & INT_TX, INT_TX);
        assert_eq!(uart.read_reg(UARTMIS), 0);
        uart.write_reg(UARTIMSC, INT_TX);
        assert_eq!(uart.read_reg(UARTMIS) & INT_TX, INT_TX);
        assert!(uart.irq_pending());
    }
}
