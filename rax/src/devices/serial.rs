//! UART 16550 Serial Port Emulation
//!
//! Fully compliant implementation of the industry-standard 16550 UART.
//! Reference: OpenCores UART 16550 IP Datasheet and Verilog implementation.

use std::collections::VecDeque;
use std::io::{self, Write};

use crate::devices::bus::{IoDevice, MmioDevice};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Register offsets
const DATA_REG: u16 = 0; // RBR (read) / THR (write) / DLL (DLAB=1)
const IER_REG: u16 = 1; // IER / DLM (DLAB=1)
const IIR_REG: u16 = 2; // IIR (read) / FCR (write)
const LCR_REG: u16 = 3; // Line Control Register
const MCR_REG: u16 = 4; // Modem Control Register
const LSR_REG: u16 = 5; // Line Status Register (read) / PSD (write, DLAB=1)
const MSR_REG: u16 = 6; // Modem Status Register
const SCR_REG: u16 = 7; // Scratch Register

// IER bits (Interrupt Enable Register)
const IER_RDA: u8 = 0x01; // Received Data Available interrupt
const IER_THRE: u8 = 0x02; // Transmitter Holding Register Empty interrupt
const IER_RLS: u8 = 0x04; // Receiver Line Status interrupt
const IER_MS: u8 = 0x08; // Modem Status interrupt
const IER_MASK: u8 = 0x0F; // Valid IER bits (bits 4-7 always 0 per spec)

// IIR bits (Interrupt Identification Register)
const IIR_NO_INT: u8 = 0x01; // No interrupt pending (bit 0 = 1)
const IIR_ID_MASK: u8 = 0x0E; // Interrupt ID bits (3:1)
const IIR_FIFO_ENABLED: u8 = 0xC0; // Bits 7:6 = 11 when FIFOs enabled

// IIR interrupt identification codes (bits 3:1)
const IIR_RLS: u8 = 0x06; // Receiver Line Status (priority 1 - highest)
const IIR_RDA: u8 = 0x04; // Received Data Available (priority 2)
const IIR_CTI: u8 = 0x0C; // Character Timeout Indication (priority 2)
const IIR_THRE: u8 = 0x02; // THR Empty (priority 3)
const IIR_MS: u8 = 0x00; // Modem Status (priority 4 - lowest)

// FCR bits (FIFO Control Register)
const FCR_FIFO_ENABLE: u8 = 0x01; // Enable FIFOs
const FCR_RX_RESET: u8 = 0x02; // Reset RX FIFO
const FCR_TX_RESET: u8 = 0x04; // Reset TX FIFO
const FCR_DMA_MODE: u8 = 0x08; // DMA mode select
const FCR_TRIGGER_MASK: u8 = 0xC0; // RX FIFO trigger level

// LCR bits (Line Control Register)
const LCR_WLS_MASK: u8 = 0x03; // Word length select
const LCR_STB: u8 = 0x04; // Stop bits (0=1, 1=1.5/2)
const LCR_PEN: u8 = 0x08; // Parity enable
const LCR_EPS: u8 = 0x10; // Even parity select
const LCR_STICK: u8 = 0x20; // Stick parity
const LCR_BREAK: u8 = 0x40; // Break control
const LCR_DLAB: u8 = 0x80; // Divisor Latch Access Bit

// MCR bits (Modem Control Register)
const MCR_DTR: u8 = 0x01; // Data Terminal Ready
const MCR_RTS: u8 = 0x02; // Request To Send
const MCR_OUT1: u8 = 0x04; // General purpose output 1
const MCR_OUT2: u8 = 0x08; // General purpose output 2 / Global interrupt enable
const MCR_LOOP: u8 = 0x10; // Loopback mode
const MCR_MASK: u8 = 0x1F; // Valid MCR bits (bits 5-7 always 0)

// LSR bits (Line Status Register)
const LSR_DR: u8 = 0x01; // Data Ready
const LSR_OE: u8 = 0x02; // Overrun Error
const LSR_PE: u8 = 0x04; // Parity Error
const LSR_FE: u8 = 0x08; // Framing Error
const LSR_BI: u8 = 0x10; // Break Interrupt
const LSR_THRE: u8 = 0x20; // Transmitter Holding Register Empty
const LSR_TEMT: u8 = 0x40; // Transmitter Empty
const LSR_FIFO_ERR: u8 = 0x80; // Error in RX FIFO

// MSR bits (Modem Status Register)
const MSR_DCTS: u8 = 0x01; // Delta CTS
const MSR_DDSR: u8 = 0x02; // Delta DSR
const MSR_TERI: u8 = 0x04; // Trailing Edge RI
const MSR_DDCD: u8 = 0x08; // Delta DCD
const MSR_CTS: u8 = 0x10; // Clear To Send
const MSR_DSR: u8 = 0x20; // Data Set Ready
const MSR_RI: u8 = 0x40; // Ring Indicator
const MSR_DCD: u8 = 0x80; // Data Carrier Detect

// FIFO size
const FIFO_SIZE: usize = 16;

// Trigger levels (number of bytes in RX FIFO to trigger interrupt)
const TRIGGER_1: usize = 1;
const TRIGGER_4: usize = 4;
const TRIGGER_8: usize = 8;
const TRIGGER_14: usize = 14;

/// Entry in the RX FIFO with associated error flags
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
struct FifoEntry {
    data: u8,
    parity_error: bool,
    framing_error: bool,
    break_indicator: bool,
}

// State for filtering cursor position responses (ESC[n;nR) on input
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum CprFilterState {
    Normal,
    GotEsc,
    GotBracket,
    InNumber,
}

// State for filtering cursor position queries (ESC[6n) on output
#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum CpqFilterState {
    Normal,
    Buffering(Vec<u8>),
}

/// UART 16550 Serial Port Implementation
///
/// Fully compliant with the 16550 specification including:
/// - 16-byte TX and RX FIFOs with configurable trigger levels
/// - All interrupt types with proper priority encoding
/// - Loopback mode
/// - Modem control/status with delta tracking
/// - Line status with all error flags
#[derive(Clone, Serialize, Deserialize)]
pub struct Serial16550 {
    base_port: Option<u16>,
    base_mmio: Option<u64>,

    // Registers
    ier: u8, // Interrupt Enable Register
    lcr: u8, // Line Control Register
    mcr: u8, // Modem Control Register
    scr: u8, // Scratch Register
    fcr: u8, // FIFO Control Register (write-only, but we track state)

    // Divisor latch
    dll: u8, // Divisor Latch Low
    dlm: u8, // Divisor Latch High

    // FIFO state
    fifo_enabled: bool,
    rx_fifo: VecDeque<FifoEntry>,
    tx_fifo: VecDeque<u8>,
    rx_trigger: usize,

    // Line Status error flags (cleared on LSR read)
    overrun_error: bool,

    // Modem status
    msr: u8,       // Current modem status (bits 7:4)
    msr_delta: u8, // Delta bits (bits 3:0), cleared on MSR read

    // Interrupt state
    thre_interrupt: bool, // THRE interrupt pending
    timeout_counter: u32, // Character timeout counter
    timeout_active: bool, // Timeout condition active

    // Input handling. Host bytes are staged in `input_buffer` (unbounded) and
    // metered into the 16-byte hardware RX FIFO by `pump_input`. The stdin
    // reader thread and host escape mux live in `crate::console`, which feeds
    // bytes in via `queue_input`.
    input_buffer: VecDeque<u8>,
    cpr_state: CprFilterState,
    cpq_state: CpqFilterState,
}

impl Serial16550 {
    pub fn new(base: u16) -> Self {
        Serial16550 {
            base_port: Some(base),
            base_mmio: None,
            ier: 0,
            lcr: 0,
            mcr: 0,
            scr: 0,
            fcr: 0,
            dll: 0,
            dlm: 0,
            fifo_enabled: false,
            rx_fifo: VecDeque::with_capacity(FIFO_SIZE),
            tx_fifo: VecDeque::with_capacity(FIFO_SIZE),
            rx_trigger: TRIGGER_1,
            overrun_error: false,
            msr: 0,
            msr_delta: 0,
            thre_interrupt: false,
            timeout_counter: 0,
            timeout_active: false,
            input_buffer: VecDeque::new(),
            cpr_state: CprFilterState::Normal,
            cpq_state: CpqFilterState::Normal,
        }
    }

    pub fn new_mmio(base: u64) -> Self {
        Serial16550 {
            base_port: None,
            base_mmio: Some(base),
            ier: 0,
            lcr: 0,
            mcr: 0,
            scr: 0,
            fcr: 0,
            dll: 0,
            dlm: 0,
            fifo_enabled: false,
            rx_fifo: VecDeque::with_capacity(FIFO_SIZE),
            tx_fifo: VecDeque::with_capacity(FIFO_SIZE),
            rx_trigger: TRIGGER_1,
            overrun_error: false,
            msr: 0,
            msr_delta: 0,
            thre_interrupt: false,
            timeout_counter: 0,
            timeout_active: false,
            input_buffer: VecDeque::new(),
            cpr_state: CprFilterState::Normal,
            cpq_state: CpqFilterState::Normal,
        }
    }

    pub fn set_mmio_base(&mut self, base: u64) {
        self.base_mmio = Some(base);
    }

    /// Filter cursor position query (ESC[6n) on output
    fn filter_cpq(&mut self, byte: u8) -> Option<Vec<u8>> {
        let state = std::mem::replace(&mut self.cpq_state, CpqFilterState::Normal);
        match state {
            CpqFilterState::Normal => {
                if byte == 0x1b {
                    self.cpq_state = CpqFilterState::Buffering(vec![byte]);
                    None
                } else {
                    Some(vec![byte])
                }
            }
            CpqFilterState::Buffering(mut buf) => {
                buf.push(byte);
                match buf.as_slice() {
                    [0x1b] | [0x1b, b'['] | [0x1b, b'[', b'6'] => {
                        self.cpq_state = CpqFilterState::Buffering(buf);
                        None
                    }
                    [0x1b, b'[', b'6', b'n'] => None, // Complete CPQ - suppress
                    _ => Some(buf),                   // Not a CPQ, output buffered bytes
                }
            }
        }
    }

    /// Queue host input bytes for delivery to the guest.
    ///
    /// Bytes arrive from `crate::console` (which owns the stdin reader thread
    /// and the host escape mux). They are filtered for terminal cursor-position
    /// responses, staged in the unbounded `input_buffer`, then metered into the
    /// 16-byte RX FIFO by `pump_input` — so a burst/paste larger than the FIFO
    /// is delivered without loss as the guest drains it.
    pub fn queue_input(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            // Filter cursor position responses
            let (filtered, extra_bytes) = match (self.cpr_state, byte) {
                (CprFilterState::Normal, 0x1b) => {
                    self.cpr_state = CprFilterState::GotEsc;
                    (true, None)
                }
                (CprFilterState::GotEsc, b'[') => {
                    self.cpr_state = CprFilterState::GotBracket;
                    (true, None)
                }
                (CprFilterState::GotEsc, _) => {
                    self.cpr_state = CprFilterState::Normal;
                    (false, Some(vec![0x1b]))
                }
                (CprFilterState::GotBracket, b'0'..=b'9') => {
                    self.cpr_state = CprFilterState::InNumber;
                    (true, None)
                }
                (CprFilterState::GotBracket, _) => {
                    self.cpr_state = CprFilterState::Normal;
                    (false, Some(vec![0x1b, b'[']))
                }
                (CprFilterState::InNumber, b'0'..=b'9' | b';') => (true, None),
                (CprFilterState::InNumber, b'R') => {
                    self.cpr_state = CprFilterState::Normal;
                    (true, None)
                }
                (CprFilterState::InNumber, _) => {
                    self.cpr_state = CprFilterState::Normal;
                    (false, None)
                }
                (CprFilterState::Normal, _) => (false, None),
            };

            // Stage bytes in the unbounded input buffer rather than pushing
            // straight into the 16-byte hardware FIFO. A burst/paste larger than
            // the FIFO would otherwise overrun and silently drop bytes (e.g. the
            // trailing newline of a command). pump_input() then meters them into
            // the FIFO as the guest drains it.
            if let Some(bytes) = extra_bytes {
                self.input_buffer.extend(bytes);
            }
            if !filtered {
                self.input_buffer.push_back(byte);
            }
        }

        self.pump_input();
    }

    /// Meter staged host input into the 16-byte RX FIFO up to its capacity.
    /// Called after staging and after each RBR read, so a large burst is fed to
    /// the guest a FIFO-full at a time without loss.
    fn pump_input(&mut self) {
        while self.rx_fifo.len() < FIFO_SIZE {
            match self.input_buffer.pop_front() {
                Some(b) => self.receive_byte(b),
                None => break,
            }
        }
    }

    /// Receive a byte into the RX FIFO
    fn receive_byte(&mut self, data: u8) {
        self.receive_byte_with_errors(data, false, false, false);
    }

    fn receive_byte_with_errors(
        &mut self,
        data: u8,
        parity_error: bool,
        framing_error: bool,
        break_indicator: bool,
    ) {
        if self.fifo_enabled {
            if self.rx_fifo.len() >= FIFO_SIZE {
                // Overrun - new data overwrites shift register (per spec, FIFO data preserved)
                self.overrun_error = true;
            } else {
                self.rx_fifo.push_back(FifoEntry {
                    data,
                    parity_error,
                    framing_error,
                    break_indicator,
                });
            }
        } else {
            // Non-FIFO mode: single character buffer
            if !self.rx_fifo.is_empty() {
                // Overrun - new data overwrites old (per spec)
                self.overrun_error = true;
                self.rx_fifo.clear();
            }
            self.rx_fifo.push_back(FifoEntry {
                data,
                parity_error,
                framing_error,
                break_indicator,
            });
        }
        // Reset timeout counter on new data
        self.timeout_counter = 0;
        self.timeout_active = false;
    }

    /// Check if any interrupt is pending (considering global mask)
    pub fn has_pending_interrupt(&self) -> bool {
        // MCR bit 3 (OUT2) acts as global interrupt enable per spec
        if (self.mcr & MCR_OUT2) == 0 {
            return false;
        }
        self.get_pending_interrupt().is_some()
    }

    /// Get the highest priority pending interrupt
    fn get_pending_interrupt(&self) -> Option<u8> {
        // Priority order (highest to lowest):
        // 1. Receiver Line Status
        // 2. Received Data Available / Character Timeout
        // 3. THR Empty
        // 4. Modem Status

        // Check RLS (Receiver Line Status)
        if (self.ier & IER_RLS) != 0 {
            let lsr = self.compute_lsr();
            if (lsr & (LSR_OE | LSR_PE | LSR_FE | LSR_BI)) != 0 {
                return Some(IIR_RLS);
            }
        }

        // Check RDA (Received Data Available) or Character Timeout.
        if (self.ier & IER_RDA) != 0 {
            let trigger_met = if self.fifo_enabled {
                self.rx_fifo.len() >= self.rx_trigger
            } else {
                !self.rx_fifo.is_empty()
            };
            if trigger_met {
                return Some(IIR_RDA);
            }
            // Receiver character-timeout (IIR_CTI). A real 16550 raises this when
            // the RX FIFO holds data BELOW the programmed trigger level and the
            // line then goes idle (~4 character-times with no new byte). We have
            // no per-character bit clock, and host input arrives in discrete
            // poll() bursts after which the line is immediately idle — so any
            // non-empty, sub-trigger FIFO is treated as timed-out. WITHOUT this,
            // a single typed keystroke (1 byte, vs the Linux 8250 driver's
            // 14-byte trigger from FCR=0xC7) never raises IRQ 4: the byte sits in
            // the FIFO (LSR.DR set) but the HLT-idle, interrupt-driven guest is
            // never woken to read it, so the interactive console silently
            // swallows all typed input. (This replaces the never-set
            // `timeout_active` stub that left IIR_CTI permanently dead.)
            if self.fifo_enabled && !self.rx_fifo.is_empty() {
                return Some(IIR_CTI);
            }
        }

        // Check THRE
        if (self.ier & IER_THRE) != 0 && self.thre_interrupt {
            return Some(IIR_THRE);
        }

        // Check MS (Modem Status)
        if (self.ier & IER_MS) != 0 && (self.msr_delta & 0x0F) != 0 {
            return Some(IIR_MS);
        }

        None
    }

    /// Returns true if there's input data available
    pub fn has_input(&self) -> bool {
        !self.rx_fifo.is_empty() || !self.input_buffer.is_empty()
    }

    /// Inject input data directly into the buffer (for testing)
    pub fn inject_input(&mut self, data: &[u8]) {
        for &byte in data {
            self.receive_byte(byte);
        }
    }

    /// Compute the current LSR value
    fn compute_lsr(&self) -> u8 {
        let mut lsr = 0u8;

        // Bit 0: Data Ready
        if !self.rx_fifo.is_empty() {
            lsr |= LSR_DR;
        }

        // Bit 1: Overrun Error (cleared on LSR read)
        if self.overrun_error {
            lsr |= LSR_OE;
        }

        // Bits 2-4: Errors from top of FIFO
        if let Some(entry) = self.rx_fifo.front() {
            if entry.parity_error {
                lsr |= LSR_PE;
            }
            if entry.framing_error {
                lsr |= LSR_FE;
            }
            if entry.break_indicator {
                lsr |= LSR_BI;
            }
        }

        // Bit 5: THR Empty (in FIFO mode, empty when FIFO is completely empty)
        if self.fifo_enabled {
            if self.tx_fifo.is_empty() {
                lsr |= LSR_THRE;
            }
        } else {
            // In non-FIFO mode, we're always "empty" (immediate transmit)
            lsr |= LSR_THRE;
        }

        // Bit 6: Transmitter Empty (both THR/FIFO and shift register empty)
        // In emulation, we always transmit immediately
        if (lsr & LSR_THRE) != 0 {
            lsr |= LSR_TEMT;
        }

        // Bit 7: FIFO Error (any data in FIFO has error)
        if self.fifo_enabled {
            for entry in &self.rx_fifo {
                if entry.parity_error || entry.framing_error || entry.break_indicator {
                    lsr |= LSR_FIFO_ERR;
                    break;
                }
            }
        }

        lsr
    }

    /// Update modem status with delta tracking
    fn update_modem_status(&mut self, new_status: u8) {
        let old_status = self.msr & 0xF0;
        let new_status_bits = new_status & 0xF0;

        // Track changes in status bits
        if (old_status ^ new_status_bits) & MSR_CTS != 0 {
            self.msr_delta |= MSR_DCTS;
        }
        if (old_status ^ new_status_bits) & MSR_DSR != 0 {
            self.msr_delta |= MSR_DDSR;
        }
        // RI trailing edge (0->1 transition)
        if (old_status & MSR_RI) != 0 && (new_status_bits & MSR_RI) == 0 {
            self.msr_delta |= MSR_TERI;
        }
        if (old_status ^ new_status_bits) & MSR_DCD != 0 {
            self.msr_delta |= MSR_DDCD;
        }

        self.msr = (self.msr & 0x0F) | new_status_bits;
    }

    /// Handle FCR write
    fn write_fcr(&mut self, value: u8) {
        self.fcr = value;

        // Bit 0: FIFO enable
        let new_fifo_enabled = (value & FCR_FIFO_ENABLE) != 0;
        if new_fifo_enabled != self.fifo_enabled {
            // Changing FIFO enable resets both FIFOs
            self.rx_fifo.clear();
            self.tx_fifo.clear();
            self.fifo_enabled = new_fifo_enabled;
        }

        // Bit 1: RX FIFO reset
        if (value & FCR_RX_RESET) != 0 {
            self.rx_fifo.clear();
            self.overrun_error = false;
            self.timeout_counter = 0;
            self.timeout_active = false;
        }

        // Bit 2: TX FIFO reset
        if (value & FCR_TX_RESET) != 0 {
            self.tx_fifo.clear();
        }

        // Bits 7:6: Trigger level
        self.rx_trigger = match (value & FCR_TRIGGER_MASK) >> 6 {
            0 => TRIGGER_1,
            1 => TRIGGER_4,
            2 => TRIGGER_8,
            3 => TRIGGER_14,
            _ => unreachable!(),
        };
    }

    /// Transmit a byte (write to THR)
    fn transmit_byte(&mut self, data: u8) {
        // Check for loopback mode
        if (self.mcr & MCR_LOOP) != 0 {
            // In loopback mode, transmitted data goes to receiver
            self.receive_byte(data);
        } else {
            // Normal mode: output to stdout
            if let Some(bytes) = self.filter_cpq(data) {
                let _ = io::stdout().write_all(&bytes);
                let _ = io::stdout().flush();
            }
        }

        // Set THRE interrupt (transmitter became empty)
        self.thre_interrupt = true;
    }

    fn port_offset(&self, port: u16) -> Option<u16> {
        let base = self.base_port?;
        if port >= base && port < base + 8 {
            Some(port - base)
        } else {
            None
        }
    }

    fn mmio_offset(&self, addr: u64) -> Option<u16> {
        let base = self.base_mmio?;
        if addr >= base && addr < base + 8 {
            Some((addr - base) as u16)
        } else {
            None
        }
    }

    fn write_reg(&mut self, offset: u16, value: u8) {
        let dlab = (self.lcr & LCR_DLAB) != 0;

        match offset {
            DATA_REG => {
                if dlab {
                    self.dll = value;
                } else {
                    // Write to THR
                    self.thre_interrupt = false; // Clear THRE interrupt on write
                    self.transmit_byte(value);
                }
            }
            IER_REG => {
                if dlab {
                    self.dlm = value;
                } else {
                    self.ier = value & IER_MASK;
                    // Re-evaluate THRE interrupt when IER changes
                    if (self.ier & IER_THRE) != 0 {
                        // If THRE is now enabled and THR is empty, set interrupt
                        let lsr = self.compute_lsr();
                        if (lsr & LSR_THRE) != 0 {
                            self.thre_interrupt = true;
                        }
                    }
                }
            }
            IIR_REG => {
                // IIR is read-only, writes go to FCR
                self.write_fcr(value);
            }
            LCR_REG => {
                self.lcr = value;
            }
            MCR_REG => {
                let old_mcr = self.mcr;
                self.mcr = value & MCR_MASK;

                // In loopback mode, modem outputs connect to modem inputs
                if (self.mcr & MCR_LOOP) != 0 {
                    let mut new_msr = 0u8;
                    // MCR[0] (DTR) -> MSR[5] (DSR)
                    if (self.mcr & MCR_DTR) != 0 {
                        new_msr |= MSR_DSR;
                    }
                    // MCR[1] (RTS) -> MSR[4] (CTS)
                    if (self.mcr & MCR_RTS) != 0 {
                        new_msr |= MSR_CTS;
                    }
                    // MCR[2] (OUT1) -> MSR[6] (RI)
                    if (self.mcr & MCR_OUT1) != 0 {
                        new_msr |= MSR_RI;
                    }
                    // MCR[3] (OUT2) -> MSR[7] (DCD)
                    if (self.mcr & MCR_OUT2) != 0 {
                        new_msr |= MSR_DCD;
                    }
                    self.update_modem_status(new_msr);
                } else if (old_mcr & MCR_LOOP) != 0 {
                    // Exiting loopback mode - reset modem status
                    self.update_modem_status(0);
                }
            }
            LSR_REG => {
                // LSR is read-only (writes to PSD when DLAB=1, but we don't implement PSD)
            }
            MSR_REG => {
                // MSR is read-only
            }
            SCR_REG => {
                self.scr = value;
            }
            _ => {}
        }
    }

    fn read_reg(&mut self, offset: u16) -> u8 {
        let dlab = (self.lcr & LCR_DLAB) != 0;

        match offset {
            DATA_REG => {
                if dlab {
                    self.dll
                } else {
                    // Read from RBR
                    self.timeout_counter = 0;
                    self.timeout_active = false;

                    let byte = self.rx_fifo.pop_front().map(|e| e.data).unwrap_or(0);
                    // Refill the FIFO from any staged host input as the guest
                    // drains it, so a >16-byte burst is consumed within one ISR.
                    self.pump_input();
                    byte
                }
            }
            IER_REG => {
                if dlab {
                    self.dlm
                } else {
                    self.ier
                }
            }
            IIR_REG => {
                let mut iir = if let Some(int_id) = self.get_pending_interrupt() {
                    // Clear THRE interrupt on IIR read if THRE is identified
                    if int_id == IIR_THRE {
                        self.thre_interrupt = false;
                    }
                    int_id // Bit 0 = 0 (interrupt pending)
                } else {
                    IIR_NO_INT // Bit 0 = 1 (no interrupt pending)
                };

                // Bits 7:6 indicate FIFO status
                if self.fifo_enabled {
                    iir |= IIR_FIFO_ENABLED;
                }

                iir
            }
            LCR_REG => self.lcr,
            MCR_REG => self.mcr,
            LSR_REG => {
                let lsr = self.compute_lsr();
                // Clear overrun error on read
                self.overrun_error = false;
                lsr
            }
            MSR_REG => {
                let msr = (self.msr & 0xF0) | self.msr_delta;
                // Clear delta bits on read
                self.msr_delta = 0;
                msr
            }
            SCR_REG => self.scr,
            _ => 0,
        }
    }

    fn read_mmio(&mut self, addr: u64, data: &mut [u8]) {
        if let Some(offset) = self.mmio_offset(addr) {
            for (idx, byte) in data.iter_mut().enumerate() {
                let reg = offset.saturating_add(idx as u16);
                if reg < 8 {
                    *byte = self.read_reg(reg);
                } else {
                    *byte = 0;
                }
            }
        } else {
            for byte in data {
                *byte = 0;
            }
        }
    }

    fn write_mmio(&mut self, addr: u64, data: &[u8]) {
        if let Some(offset) = self.mmio_offset(addr) {
            for (idx, byte) in data.iter().enumerate() {
                let reg = offset.saturating_add(idx as u16);
                if reg < 8 {
                    self.write_reg(reg, *byte);
                }
            }
        }
    }

    /// Get the current divisor value
    pub fn divisor(&self) -> u16 {
        ((self.dlm as u16) << 8) | (self.dll as u16)
    }
}

impl IoDevice for Serial16550 {
    fn read(&mut self, port: u16) -> u8 {
        self.port_offset(port)
            .map(|offset| self.read_reg(offset))
            .unwrap_or(0)
    }

    fn write(&mut self, port: u16, value: u8) {
        if let Some(offset) = self.port_offset(port) {
            self.write_reg(offset, value);
        }
    }
}

impl MmioDevice for Serial16550 {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        self.read_mmio(addr, data);
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        self.write_mmio(addr, data);
    }
}

pub struct SerialMmioDevice {
    inner: Arc<Mutex<Serial16550>>,
}

impl SerialMmioDevice {
    pub fn new(inner: Arc<Mutex<Serial16550>>) -> Self {
        SerialMmioDevice { inner }
    }
}

impl MmioDevice for SerialMmioDevice {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        if let Ok(mut serial) = self.inner.lock() {
            serial.read_mmio(addr, data);
        } else {
            for byte in data {
                *byte = 0;
            }
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        if let Ok(mut serial) = self.inner.lock() {
            serial.write_mmio(addr, data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::bus::IoDevice;

    const BASE: u16 = 0x3F8;
    const THR: u16 = BASE;
    const RBR: u16 = BASE;
    const DLL: u16 = BASE;
    const IER: u16 = BASE + 1;
    const DLM: u16 = BASE + 1;
    const IIR: u16 = BASE + 2;
    const FCR: u16 = BASE + 2;
    const LCR: u16 = BASE + 3;
    const MCR: u16 = BASE + 4;
    const LSR: u16 = BASE + 5;
    const MSR: u16 = BASE + 6;
    const SCR: u16 = BASE + 7;

    // ============================================================================
    // DLAB and Divisor Tests
    // ============================================================================

    #[test]
    fn serial_tracks_dlab_and_divisor() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, LCR, 0x80);
        IoDevice::write(&mut serial, DLL, 0x34);
        IoDevice::write(&mut serial, DLM, 0x12);
        assert_eq!(serial.divisor(), 0x1234);
        IoDevice::write(&mut serial, LCR, 0x00);
        assert_eq!(serial.lcr & LCR_DLAB, 0);
    }

    #[test]
    fn serial_divisor_read_back() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, LCR, 0x80);
        IoDevice::write(&mut serial, DLL, 0xAB);
        IoDevice::write(&mut serial, DLM, 0xCD);
        assert_eq!(IoDevice::read(&mut serial, DLL), 0xAB);
        assert_eq!(IoDevice::read(&mut serial, DLM), 0xCD);
    }

    // ============================================================================
    // LSR Tests
    // ============================================================================

    #[test]
    fn serial_lsr_initial_state() {
        let mut serial = Serial16550::new(BASE);
        let lsr = IoDevice::read(&mut serial, LSR);
        assert_eq!(lsr, LSR_THRE | LSR_TEMT);
    }

    #[test]
    fn serial_lsr_data_ready_with_input() {
        let mut serial = Serial16550::new(BASE);
        serial.inject_input(b"A");
        let lsr = IoDevice::read(&mut serial, LSR);
        assert_eq!(lsr & LSR_DR, LSR_DR);
    }

    #[test]
    fn serial_lsr_overrun_error() {
        let mut serial = Serial16550::new(BASE);
        // In non-FIFO mode, second byte causes overrun
        serial.inject_input(b"A");
        serial.inject_input(b"B");
        let lsr = IoDevice::read(&mut serial, LSR);
        assert_eq!(lsr & LSR_OE, LSR_OE);
        // Overrun cleared on read
        let lsr2 = IoDevice::read(&mut serial, LSR);
        assert_eq!(lsr2 & LSR_OE, 0);
    }

    #[test]
    fn serial_lsr_fifo_no_overrun_until_full() {
        let mut serial = Serial16550::new(BASE);
        // Enable FIFO
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE);
        // Fill FIFO (16 bytes)
        for i in 0..16 {
            serial.inject_input(&[i]);
        }
        let lsr = IoDevice::read(&mut serial, LSR);
        assert_eq!(lsr & LSR_OE, 0); // No overrun yet
        // 17th byte causes overrun
        serial.inject_input(b"X");
        let lsr2 = IoDevice::read(&mut serial, LSR);
        assert_eq!(lsr2 & LSR_OE, LSR_OE);
    }

    // ============================================================================
    // FIFO Tests
    // ============================================================================

    #[test]
    fn serial_fifo_enable() {
        let mut serial = Serial16550::new(BASE);
        assert!(!serial.fifo_enabled);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE);
        assert!(serial.fifo_enabled);
    }

    #[test]
    fn serial_fifo_trigger_levels() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE | 0x00);
        assert_eq!(serial.rx_trigger, 1);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE | 0x40);
        assert_eq!(serial.rx_trigger, 4);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE | 0x80);
        assert_eq!(serial.rx_trigger, 8);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE | 0xC0);
        assert_eq!(serial.rx_trigger, 14);
    }

    #[test]
    fn serial_fifo_reset() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE);
        serial.inject_input(b"ABC");
        assert_eq!(serial.rx_fifo.len(), 3);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE | FCR_RX_RESET);
        assert_eq!(serial.rx_fifo.len(), 0);
    }

    // ============================================================================
    // IIR Tests
    // ============================================================================

    #[test]
    fn serial_iir_no_interrupt() {
        let mut serial = Serial16550::new(BASE);
        let iir = IoDevice::read(&mut serial, IIR);
        assert_eq!(iir & IIR_NO_INT, IIR_NO_INT);
    }

    #[test]
    fn serial_iir_fifo_enabled_bits() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE);
        let iir = IoDevice::read(&mut serial, IIR);
        assert_eq!(iir & IIR_FIFO_ENABLED, IIR_FIFO_ENABLED);
    }

    #[test]
    fn serial_iir_thre_interrupt() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, MCR_OUT2); // Enable global interrupts
        IoDevice::write(&mut serial, IER, IER_THRE);
        IoDevice::write(&mut serial, THR, 0x41);
        let iir = IoDevice::read(&mut serial, IIR);
        assert_eq!(iir & IIR_ID_MASK, IIR_THRE);
        assert_eq!(iir & IIR_NO_INT, 0);
    }

    #[test]
    fn serial_iir_rda_interrupt() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, MCR_OUT2);
        IoDevice::write(&mut serial, IER, IER_RDA);
        serial.inject_input(b"X");
        let iir = IoDevice::read(&mut serial, IIR);
        assert_eq!(iir & IIR_ID_MASK, IIR_RDA);
    }

    #[test]
    fn serial_iir_rda_priority_over_thre() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, MCR_OUT2);
        IoDevice::write(&mut serial, IER, IER_RDA | IER_THRE);
        IoDevice::write(&mut serial, THR, 0x41); // Trigger THRE
        serial.inject_input(b"X"); // Trigger RDA
        let iir = IoDevice::read(&mut serial, IIR);
        // RDA has higher priority
        assert_eq!(iir & IIR_ID_MASK, IIR_RDA);
    }

    #[test]
    fn serial_iir_global_interrupt_mask() {
        let mut serial = Serial16550::new(BASE);
        // Don't set MCR_OUT2 (global mask)
        IoDevice::write(&mut serial, IER, IER_THRE);
        IoDevice::write(&mut serial, THR, 0x41);
        // Should not report interrupt due to global mask
        assert!(!serial.has_pending_interrupt());
    }

    // ============================================================================
    // MCR and Loopback Tests
    // ============================================================================

    #[test]
    fn serial_mcr_bits_masked() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, 0xFF);
        assert_eq!(IoDevice::read(&mut serial, MCR), MCR_MASK);
    }

    #[test]
    fn serial_loopback_mode() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, MCR_LOOP);
        // In loopback mode, writing to THR should appear in RBR
        IoDevice::write(&mut serial, THR, 0x42);
        assert_eq!(IoDevice::read(&mut serial, RBR), 0x42);
    }

    #[test]
    fn serial_loopback_modem_signals() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, MCR_LOOP | MCR_DTR | MCR_RTS);
        let msr = IoDevice::read(&mut serial, MSR);
        // DTR -> DSR, RTS -> CTS
        assert_eq!(msr & (MSR_DSR | MSR_CTS), MSR_DSR | MSR_CTS);
    }

    // ============================================================================
    // MSR Tests
    // ============================================================================

    #[test]
    fn serial_msr_delta_bits() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, MCR, MCR_LOOP);
        // Set CTS via loopback (RTS -> CTS)
        IoDevice::write(&mut serial, MCR, MCR_LOOP | MCR_RTS);
        let msr = IoDevice::read(&mut serial, MSR);
        assert_eq!(msr & MSR_DCTS, MSR_DCTS); // Delta CTS set
        // Delta cleared on read
        let msr2 = IoDevice::read(&mut serial, MSR);
        assert_eq!(msr2 & MSR_DCTS, 0);
    }

    // ============================================================================
    // IER Tests
    // ============================================================================

    #[test]
    fn serial_ier_bits_masked() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, IER, 0xFF);
        assert_eq!(IoDevice::read(&mut serial, IER), IER_MASK);
    }

    // ============================================================================
    // LCR Tests
    // ============================================================================

    #[test]
    fn serial_lcr_write_read() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, LCR, 0x1B);
        assert_eq!(IoDevice::read(&mut serial, LCR), 0x1B);
    }

    // ============================================================================
    // SCR Tests
    // ============================================================================

    #[test]
    fn serial_scratch_write_read() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, SCR, 0xA5);
        assert_eq!(IoDevice::read(&mut serial, SCR), 0xA5);
    }

    // ============================================================================
    // RBR/THR Tests
    // ============================================================================

    #[test]
    fn serial_read_multiple_chars() {
        let mut serial = Serial16550::new(BASE);
        IoDevice::write(&mut serial, FCR, FCR_FIFO_ENABLE);
        serial.inject_input(b"ABC");
        assert_eq!(IoDevice::read(&mut serial, RBR), b'A');
        assert_eq!(IoDevice::read(&mut serial, RBR), b'B');
        assert_eq!(IoDevice::read(&mut serial, RBR), b'C');
        assert_eq!(IoDevice::read(&mut serial, RBR), 0);
    }

    // ============================================================================
    // MMIO Tests
    // ============================================================================

    #[test]
    fn serial_mmio_basic() {
        use crate::devices::bus::MmioDevice;
        let mut serial = Serial16550::new_mmio(0x1000);
        MmioDevice::write(&mut serial, 0x1003, &[0x03]);
        let mut buf = [0u8; 1];
        MmioDevice::read(&mut serial, 0x1003, &mut buf);
        assert_eq!(buf[0], 0x03);
    }

    #[test]
    fn serial_mmio_lsr() {
        use crate::devices::bus::MmioDevice;
        let mut serial = Serial16550::new_mmio(0x1000);
        let mut buf = [0u8; 1];
        MmioDevice::read(&mut serial, 0x1005, &mut buf);
        assert_eq!(buf[0], LSR_THRE | LSR_TEMT);
    }

    // ============================================================================
    // Typical Usage Sequence
    // ============================================================================

    #[test]
    fn serial_typical_init_sequence() {
        let mut serial = Serial16550::new(BASE);

        // 1. Disable interrupts
        IoDevice::write(&mut serial, IER, 0x00);

        // 2. Set DLAB, configure baud rate
        IoDevice::write(&mut serial, LCR, LCR_DLAB);
        IoDevice::write(&mut serial, DLL, 0x01);
        IoDevice::write(&mut serial, DLM, 0x00);

        // 3. Set 8N1, clear DLAB
        IoDevice::write(&mut serial, LCR, 0x03);

        // 4. Enable FIFO with 14-byte trigger
        IoDevice::write(&mut serial, FCR, 0xC7);

        // 5. Set MCR (DTR, RTS, OUT2)
        IoDevice::write(&mut serial, MCR, 0x0B);

        // 6. Enable interrupts
        IoDevice::write(&mut serial, IER, 0x03);

        // Verify
        assert_eq!(IoDevice::read(&mut serial, LCR), 0x03);
        assert_eq!(IoDevice::read(&mut serial, MCR), 0x0B);
        assert_eq!(IoDevice::read(&mut serial, IER), 0x03);
        assert_eq!(serial.divisor(), 0x0001);
        assert!(serial.fifo_enabled);
        assert_eq!(serial.rx_trigger, 14);
    }

    /// A single sub-trigger RX byte must still raise an interrupt (via the
    /// receiver character-timeout, IIR_CTI). The Linux 8250 driver programs a
    /// 14-byte RX trigger (FCR=0xC7); a lone keystroke is 1 byte (< 14), so
    /// without the timeout fallback it never raises IRQ 4 and the HLT-idle,
    /// interrupt-driven guest never reads it — the interactive console silently
    /// swallows all typed input. This guards the fix.
    #[test]
    fn serial_single_byte_below_trigger_interrupts() {
        let mut serial = Serial16550::new(BASE);
        // Kernel-typical init: 14-byte trigger, OUT2 set, interrupts enabled.
        IoDevice::write(&mut serial, IER, 0x00);
        IoDevice::write(&mut serial, LCR, LCR_DLAB);
        IoDevice::write(&mut serial, DLL, 0x01);
        IoDevice::write(&mut serial, DLM, 0x00);
        IoDevice::write(&mut serial, LCR, 0x03);
        IoDevice::write(&mut serial, FCR, 0xC7);
        IoDevice::write(&mut serial, MCR, 0x0B);
        IoDevice::write(&mut serial, IER, 0x03);
        assert_eq!(serial.rx_trigger, 14);

        // Idle shell waiting on a keystroke: RX interrupt armed, TX idle.
        IoDevice::write(&mut serial, IER, IER_RDA);
        serial.thre_interrupt = false;
        assert!(
            !serial.has_pending_interrupt(),
            "no data yet → no interrupt"
        );

        // One typed byte (1 < 14-byte trigger) MUST still interrupt.
        serial.queue_input(b"x");
        assert!(
            serial.has_pending_interrupt(),
            "a sub-trigger RX byte must raise IRQ via the character-timeout (IIR_CTI)"
        );

        // And it is delivered as IIR_CTI (character timeout), not IIR_RDA.
        let iir = IoDevice::read(&mut serial, IIR);
        assert_eq!(iir & 0x0F, IIR_CTI, "sub-trigger RX → IIR_CTI");
    }

    // ============================================================================
    // Port Range Tests
    // ============================================================================

    #[test]
    fn serial_port_out_of_range() {
        let mut serial = Serial16550::new(BASE);
        assert_eq!(IoDevice::read(&mut serial, BASE - 1), 0);
        assert_eq!(IoDevice::read(&mut serial, BASE + 8), 0);
    }
}
