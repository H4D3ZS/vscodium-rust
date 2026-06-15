//! Legacy system-control I/O ports.
//!
//! This device emulates a handful of small, historically-related "glue" ports
//! that real PCs expose for system control. Grouping them in one device keeps
//! the bus wiring simple and lets a kernel/BIOS perform the usual A20 gating,
//! reset, speaker, and POST-delay operations.
//!
//! Ports handled:
//! - `0x92`   System Control Port A (A20 gate, fast reset)
//! - `0xCF9`  Reset Control Register (RST_CPU / SYS_RST / full reset)
//! - `0x61`   System Control Port B / NMI status & PC speaker control
//! - `0x80`   POST / DMA page register (no-op scratch, often used as an I/O delay)
//!            The full `0x80`-`0x8F` range is treated as scratch storage.

use super::bus::IoDevice;

// ---- Port A (0x92) bit layout ------------------------------------------------
/// Fast reset: writing 1 to this bit requests a CPU reset.
const PORT_A_FAST_RESET: u8 = 1 << 0;
/// A20 gate enable.
const PORT_A_A20: u8 = 1 << 1;

// ---- Reset Control Register (0xCF9) bit layout -------------------------------
/// System reset request (bit 1).
const RCR_SYS_RST: u8 = 1 << 1;
/// CPU reset request (bit 2). Combined with SYS_RST it triggers a reset.
const RCR_RST_CPU: u8 = 1 << 2;
/// Full reset (bit 3) - request a "cold"/full platform reset.
const RCR_FULL_RST: u8 = 1 << 3;

// ---- Port B (0x61) bit layout ------------------------------------------------
/// PIT channel 2 gate enable.
const PORT_B_CH2_GATE: u8 = 1 << 0;
/// PC speaker data enable.
const PORT_B_SPEAKER: u8 = 1 << 1;
/// Refresh request toggle (toggles on every read).
const PORT_B_REFRESH: u8 = 1 << 4;
/// PIT channel 2 OUT status (driven by the PIT).
const PORT_B_CH2_OUT: u8 = 1 << 5;
/// Writable bits in port B (only the low two control bits are stored).
const PORT_B_WRITE_MASK: u8 = PORT_B_CH2_GATE | PORT_B_SPEAKER;

/// Legacy system control ports device.
pub struct SystemControl {
    /// A20 gate enabled (Port A bit 1).
    a20_enabled: bool,
    /// A reset has been requested via Port A (0x92) or the RCR (0xCF9).
    reset_requested: bool,
    /// A full/cold reset was requested via the RCR (0xCF9 bit 3).
    full_reset_requested: bool,
    /// Last value written to the Reset Control Register (0xCF9).
    rcr: u8,
    /// PIT channel 2 gate (Port B bit 0).
    ch2_gate: bool,
    /// PC speaker enable (Port B bit 1).
    speaker_enabled: bool,
    /// Refresh toggle state (Port B bit 4) - flips on every read of 0x61.
    refresh_toggle: bool,
    /// PIT channel 2 OUT level (Port B bit 5), driven by the PIT.
    ch2_out: bool,
    /// Scratch storage for the POST / DMA page register range (0x80-0x8F).
    post_scratch: u8,
}

impl Default for SystemControl {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemControl {
    pub fn new() -> Self {
        SystemControl {
            // Many systems boot with A20 already enabled; the kernel re-checks
            // anyway. Default to enabled to match common firmware behaviour.
            a20_enabled: true,
            reset_requested: false,
            full_reset_requested: false,
            rcr: 0,
            ch2_gate: false,
            speaker_enabled: false,
            refresh_toggle: false,
            ch2_out: false,
            post_scratch: 0,
        }
    }

    /// Whether the A20 gate is currently enabled.
    pub fn a20_enabled(&self) -> bool {
        self.a20_enabled
    }

    /// Whether a reset has been requested (via 0x92 fast reset or 0xCF9).
    pub fn reset_requested(&self) -> bool {
        self.reset_requested
    }

    /// Whether a full/cold reset was requested (0xCF9 bit 3).
    pub fn full_reset_requested(&self) -> bool {
        self.full_reset_requested
    }

    /// Clear any pending reset request after the orchestrator has handled it.
    pub fn clear_reset(&mut self) {
        self.reset_requested = false;
        self.full_reset_requested = false;
    }

    /// Whether the PC speaker is enabled (Port B bit 1).
    pub fn speaker_enabled(&self) -> bool {
        self.speaker_enabled
    }

    /// Whether the PIT channel 2 gate is enabled (Port B bit 0).
    pub fn ch2_gate(&self) -> bool {
        self.ch2_gate
    }

    /// Drive the PIT channel 2 OUT line (reflected in Port B bit 5).
    ///
    /// The PIT calls this so software reading 0x61 sees the speaker output
    /// level (used by some timing / calibration code).
    pub fn set_pit_ch2_out(&mut self, out: bool) {
        self.ch2_out = out;
    }

    // ---- Port A (0x92) ------------------------------------------------------

    fn read_port_a(&self) -> u8 {
        let mut value = 0;
        if self.a20_enabled {
            value |= PORT_A_A20;
        }
        // Fast-reset bit reads back as 0 (it is a momentary action).
        value
    }

    fn write_port_a(&mut self, value: u8) {
        // Bit 1 controls the A20 gate.
        self.a20_enabled = value & PORT_A_A20 != 0;
        // Bit 0 (fast reset) is a write-1-to-trigger action.
        if value & PORT_A_FAST_RESET != 0 {
            self.reset_requested = true;
        }
    }

    // ---- Reset Control Register (0xCF9) -------------------------------------

    fn write_rcr(&mut self, value: u8) {
        self.rcr = value;
        // A reset is triggered when RST_CPU (bit 2) is set. Real hardware also
        // gates this with SYS_RST (bit 1); honour the documented combination.
        if value & RCR_RST_CPU != 0 && value & RCR_SYS_RST != 0 {
            self.reset_requested = true;
            // Bit 3 selects a full ("cold") reset versus a soft reset.
            if value & RCR_FULL_RST != 0 {
                self.full_reset_requested = true;
            }
        }
    }

    // ---- Port B (0x61) ------------------------------------------------------

    fn read_port_b(&mut self) -> u8 {
        // The refresh bit toggles on every read so calibration loops that poll
        // 0x61 waiting for it to flip make progress.
        self.refresh_toggle = !self.refresh_toggle;

        let mut value = 0;
        if self.ch2_gate {
            value |= PORT_B_CH2_GATE;
        }
        if self.speaker_enabled {
            value |= PORT_B_SPEAKER;
        }
        if self.refresh_toggle {
            value |= PORT_B_REFRESH;
        }
        if self.ch2_out {
            value |= PORT_B_CH2_OUT;
        }
        // Bits 6 (IOCHK) and 7 (PARITY) report no error: return 0.
        value
    }

    fn write_port_b(&mut self, value: u8) {
        // Only the low two control bits are software-writable.
        self.ch2_gate = value & PORT_B_CH2_GATE != 0;
        self.speaker_enabled = value & PORT_B_SPEAKER != 0;
        let _ = PORT_B_WRITE_MASK;
    }
}

impl IoDevice for SystemControl {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            0x92 => self.read_port_a(),
            0xCF9 => self.rcr,
            0x61 => self.read_port_b(),
            0x80..=0x8F => self.post_scratch,
            _ => 0xFF,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            0x92 => self.write_port_a(value),
            0xCF9 => self.write_rcr(value),
            0x61 => self.write_port_b(value),
            0x80..=0x8F => self.post_scratch = value,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_defaults() {
        let dev = SystemControl::new();
        assert!(dev.a20_enabled());
        assert!(!dev.reset_requested());
        assert!(!dev.full_reset_requested());
        assert!(!dev.speaker_enabled());
        assert!(!dev.ch2_gate());
    }

    // ---- A20 gate via Port A (0x92) ----------------------------------------

    #[test]
    fn test_a20_toggle_via_port_a() {
        let mut dev = SystemControl::new();

        // Disable A20: write a value with bit 1 clear.
        dev.write(0x92, 0x00);
        assert!(!dev.a20_enabled());
        assert_eq!(dev.read(0x92) & PORT_A_A20, 0);

        // Enable A20: write a value with bit 1 set.
        dev.write(0x92, PORT_A_A20);
        assert!(dev.a20_enabled());
        assert_eq!(dev.read(0x92) & PORT_A_A20, PORT_A_A20);
    }

    // ---- Reset via Port A (0x92) fast reset --------------------------------

    #[test]
    fn test_fast_reset_via_port_a() {
        let mut dev = SystemControl::new();
        assert!(!dev.reset_requested());

        // Writing bit 0 (and keeping A20 set) requests a fast reset.
        dev.write(0x92, PORT_A_A20 | PORT_A_FAST_RESET);
        assert!(dev.reset_requested());
        // A20 should remain enabled by that same write.
        assert!(dev.a20_enabled());

        // Fast-reset bit reads back as 0 (momentary).
        assert_eq!(dev.read(0x92) & PORT_A_FAST_RESET, 0);

        // Clearing the request works.
        dev.clear_reset();
        assert!(!dev.reset_requested());
    }

    #[test]
    fn test_port_a_no_reset_without_bit0() {
        let mut dev = SystemControl::new();
        dev.write(0x92, PORT_A_A20);
        assert!(!dev.reset_requested());
    }

    // ---- Reset via Reset Control Register (0xCF9) --------------------------

    #[test]
    fn test_reset_via_cf9() {
        let mut dev = SystemControl::new();

        // RST_CPU alone (no SYS_RST) should not trigger a reset.
        dev.write(0xCF9, RCR_RST_CPU);
        assert!(!dev.reset_requested());

        // RST_CPU + SYS_RST triggers a soft reset.
        dev.write(0xCF9, RCR_RST_CPU | RCR_SYS_RST);
        assert!(dev.reset_requested());
        assert!(!dev.full_reset_requested());

        // The written value reads back from 0xCF9.
        assert_eq!(dev.read(0xCF9), RCR_RST_CPU | RCR_SYS_RST);
    }

    #[test]
    fn test_full_reset_via_cf9() {
        let mut dev = SystemControl::new();

        // Full reset: RST_CPU + SYS_RST + full-reset bit.
        dev.write(0xCF9, RCR_RST_CPU | RCR_SYS_RST | RCR_FULL_RST);
        assert!(dev.reset_requested());
        assert!(dev.full_reset_requested());

        dev.clear_reset();
        assert!(!dev.reset_requested());
        assert!(!dev.full_reset_requested());
    }

    // ---- Port B (0x61) refresh toggling, gate, and OUT ---------------------

    #[test]
    fn test_port_b_refresh_bit_toggles() {
        let mut dev = SystemControl::new();

        let first = dev.read(0x61) & PORT_B_REFRESH;
        let second = dev.read(0x61) & PORT_B_REFRESH;
        let third = dev.read(0x61) & PORT_B_REFRESH;

        // The refresh bit must flip on every read.
        assert_ne!(first, second);
        assert_ne!(second, third);
        assert_eq!(first, third);
    }

    #[test]
    fn test_port_b_ch2_gate_and_speaker_writes() {
        let mut dev = SystemControl::new();

        // Enable both ch2 gate and speaker.
        dev.write(0x61, PORT_B_CH2_GATE | PORT_B_SPEAKER);
        assert!(dev.ch2_gate());
        assert!(dev.speaker_enabled());
        let v = dev.read(0x61);
        assert_eq!(v & PORT_B_CH2_GATE, PORT_B_CH2_GATE);
        assert_eq!(v & PORT_B_SPEAKER, PORT_B_SPEAKER);

        // Disable both.
        dev.write(0x61, 0x00);
        assert!(!dev.ch2_gate());
        assert!(!dev.speaker_enabled());
    }

    #[test]
    fn test_port_b_ch2_out_setter() {
        let mut dev = SystemControl::new();

        // Default: OUT low, bit 5 clear.
        assert_eq!(dev.read(0x61) & PORT_B_CH2_OUT, 0);

        // PIT drives OUT high.
        dev.set_pit_ch2_out(true);
        assert_eq!(dev.read(0x61) & PORT_B_CH2_OUT, PORT_B_CH2_OUT);

        // PIT drives OUT low again.
        dev.set_pit_ch2_out(false);
        assert_eq!(dev.read(0x61) & PORT_B_CH2_OUT, 0);
    }

    #[test]
    fn test_port_b_iochk_parity_bits_zero() {
        let mut dev = SystemControl::new();
        // Bits 6 and 7 (IOCHK / PARITY) always read as 0.
        let v = dev.read(0x61);
        assert_eq!(v & 0b1100_0000, 0);
    }

    #[test]
    fn test_port_b_write_ignores_high_bits() {
        let mut dev = SystemControl::new();
        // Writing high bits must not affect stored control state.
        dev.write(0x61, 0xFF);
        assert!(dev.ch2_gate());
        assert!(dev.speaker_enabled());
        // OUT and refresh are not software-writable; OUT stays at default low.
        dev.set_pit_ch2_out(false);
        assert_eq!(dev.read(0x61) & PORT_B_CH2_OUT, 0);
    }

    // ---- POST / DMA page scratch (0x80-0x8F) -------------------------------

    #[test]
    fn test_post_scratch_read_write() {
        let mut dev = SystemControl::new();

        dev.write(0x80, 0xAB);
        assert_eq!(dev.read(0x80), 0xAB);

        // Overwrite with a new value.
        dev.write(0x80, 0x5C);
        assert_eq!(dev.read(0x80), 0x5C);
    }

    #[test]
    fn test_post_scratch_range() {
        let mut dev = SystemControl::new();
        // The whole 0x80-0x8F range shares the scratch byte.
        dev.write(0x88, 0x42);
        assert_eq!(dev.read(0x8F), 0x42);
        assert_eq!(dev.read(0x80), 0x42);
    }

    // ---- Unhandled ports ---------------------------------------------------

    #[test]
    fn test_unhandled_port_reads_ff() {
        let mut dev = SystemControl::new();
        assert_eq!(dev.read(0x1234), 0xFF);
        // A write to an unhandled port is silently ignored (no panic).
        dev.write(0x1234, 0x99);
    }
}
