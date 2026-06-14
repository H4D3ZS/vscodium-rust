//! SysTick Timer for Cortex-M.
//!
//! The SysTick timer provides a simple 24-bit countdown timer that can generate
//! periodic interrupts. It is typically used for:
//! - RTOS tick generation
//! - Time delays
//! - System timing
//!
//! Registers (offset from SCS base 0xE000E000):
//! - 0x010: SYST_CSR (Control and Status Register)
//! - 0x014: SYST_RVR (Reload Value Register)
//! - 0x018: SYST_CVR (Current Value Register)
//! - 0x01C: SYST_CALIB (Calibration Register)

/// SysTick Control and Status Register bits.
pub mod csr {
    /// Counter enable.
    pub const ENABLE: u32 = 1 << 0;
    /// Interrupt enable.
    pub const TICKINT: u32 = 1 << 1;
    /// Clock source: 0=external, 1=processor clock.
    pub const CLKSOURCE: u32 = 1 << 2;
    /// Count flag (set when counter reaches 0).
    pub const COUNTFLAG: u32 = 1 << 16;
}

/// SysTick timer state.
#[derive(Clone, Debug)]
pub struct SysTick {
    /// Control and Status Register.
    csr: u32,
    /// Reload Value Register (24-bit).
    rvr: u32,
    /// Current Value Register (24-bit).
    cvr: u32,
    /// Calibration Register.
    calib: u32,
    /// Count flag (sticky, cleared on CSR read).
    count_flag: bool,
    /// External clock frequency (Hz).
    external_clock: u64,
    /// Processor clock frequency (Hz).
    processor_clock: u64,
    /// Cycles since last tick.
    cycles_accumulated: u64,
    /// Pending interrupt flag.
    pending: bool,
}

impl SysTick {
    /// Create a new SysTick timer.
    ///
    /// # Arguments
    /// * `processor_clock` - Processor clock frequency in Hz
    /// * `external_clock` - External reference clock frequency (or 0 if not available)
    /// * `tenms_value` - 10ms calibration value (0 if unknown)
    pub fn new(processor_clock: u64, external_clock: u64, tenms_value: u32) -> Self {
        // Build calibration register:
        // [31] NOREF: 1 if no external reference clock
        // [30] SKEW: 1 if calibration value is not exactly 10ms
        // [23:0] TENMS: reload value for 10ms ticks
        let noref = if external_clock == 0 { 1u32 << 31 } else { 0 };
        let skew = if tenms_value == 0 { 1u32 << 30 } else { 0 };
        let tenms = tenms_value & 0x00FF_FFFF;
        let calib = noref | skew | tenms;

        Self {
            csr: 0,
            rvr: 0,
            cvr: 0,
            calib,
            count_flag: false,
            external_clock,
            processor_clock,
            cycles_accumulated: 0,
            pending: false,
        }
    }

    /// Create SysTick with default settings for common frequencies.
    pub fn with_frequency(processor_clock: u64) -> Self {
        // Calculate 10ms calibration value
        let tenms = (processor_clock / 100) as u32;
        Self::new(processor_clock, 0, tenms.min(0x00FF_FFFF))
    }

    /// Reset the SysTick timer.
    pub fn reset(&mut self) {
        self.csr = 0;
        self.rvr = 0;
        self.cvr = 0;
        self.count_flag = false;
        self.cycles_accumulated = 0;
        self.pending = false;
    }

    /// Check if timer is enabled.
    pub fn is_enabled(&self) -> bool {
        self.csr & csr::ENABLE != 0
    }

    /// Check if interrupt is enabled.
    pub fn is_interrupt_enabled(&self) -> bool {
        self.csr & csr::TICKINT != 0
    }

    /// Check if using processor clock.
    pub fn uses_processor_clock(&self) -> bool {
        self.csr & csr::CLKSOURCE != 0
    }

    /// Get current clock frequency.
    pub fn clock_frequency(&self) -> u64 {
        if self.uses_processor_clock() {
            self.processor_clock
        } else {
            self.external_clock
        }
    }

    /// Check and clear pending interrupt.
    pub fn take_pending(&mut self) -> bool {
        let was_pending = self.pending;
        self.pending = false;
        was_pending
    }

    /// Check if interrupt is pending.
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// Advance the timer by the specified number of processor clock cycles.
    ///
    /// Returns true if the timer underflowed (and potentially generated an interrupt).
    pub fn tick(&mut self, processor_cycles: u64) -> bool {
        if !self.is_enabled() || self.rvr == 0 {
            return false;
        }

        // Convert processor cycles to timer ticks
        let timer_ticks = if self.uses_processor_clock() {
            processor_cycles
        } else if self.external_clock > 0 {
            // Scale cycles from processor clock to external clock
            (processor_cycles * self.external_clock) / self.processor_clock
        } else {
            return false; // No external clock available
        };

        self.cycles_accumulated += timer_ticks;
        let mut underflowed = false;

        // If CVR is 0 at start, reload without setting count flag
        // (This happens after writing to CVR)
        if self.cvr == 0 {
            self.cvr = self.rvr;
        }

        // Process timer decrements
        while self.cycles_accumulated > 0 && self.cvr > 0 {
            let decrement = self.cycles_accumulated.min(self.cvr as u64);
            self.cvr -= decrement as u32;
            self.cycles_accumulated -= decrement;

            if self.cvr == 0 {
                // Counter reached 0 - set count flag
                self.count_flag = true;
                underflowed = true;

                if self.is_interrupt_enabled() {
                    self.pending = true;
                }

                // If more cycles to process, reload and continue
                if self.cycles_accumulated > 0 {
                    self.cvr = self.rvr;
                }
            }
        }

        underflowed
    }

    /// Advance timer by one tick (for simple stepping).
    pub fn step(&mut self) -> bool {
        self.tick(1)
    }

    // =========================================================================
    // Register Access
    // =========================================================================

    /// Read CSR register.
    pub fn read_csr(&mut self) -> u32 {
        let mut value = self.csr & 0x0001_0007;
        if self.count_flag {
            value |= csr::COUNTFLAG;
            self.count_flag = false; // Cleared on read
        }
        value
    }

    /// Write CSR register.
    pub fn write_csr(&mut self, value: u32) {
        self.csr = value & 0x0000_0007; // Only bits 0-2 are writable
    }

    /// Read RVR register.
    pub fn read_rvr(&self) -> u32 {
        self.rvr
    }

    /// Write RVR register.
    pub fn write_rvr(&mut self, value: u32) {
        self.rvr = value & 0x00FF_FFFF; // 24-bit
    }

    /// Read CVR register.
    pub fn read_cvr(&self) -> u32 {
        self.cvr
    }

    /// Write CVR register (any write clears to 0 and clears COUNTFLAG).
    pub fn write_cvr(&mut self, _value: u32) {
        self.cvr = 0;
        self.count_flag = false;
    }

    /// Read CALIB register.
    pub fn read_calib(&self) -> u32 {
        self.calib
    }

    /// Read from SysTick register space (offset from 0xE000E010).
    pub fn read(&mut self, offset: u32) -> u32 {
        match offset {
            0x00 => self.read_csr(),
            0x04 => self.read_rvr(),
            0x08 => self.read_cvr(),
            0x0C => self.read_calib(),
            _ => 0,
        }
    }

    /// Write to SysTick register space.
    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x00 => self.write_csr(value),
            0x04 => self.write_rvr(value),
            0x08 => self.write_cvr(value),
            0x0C => {} // CALIB is read-only
            _ => {}
        }
    }

    // =========================================================================
    // Time Calculations
    // =========================================================================

    /// Get remaining time until next tick in processor cycles.
    pub fn cycles_until_tick(&self) -> Option<u64> {
        if !self.is_enabled() || self.rvr == 0 {
            return None;
        }

        let timer_ticks_remaining = if self.cvr == 0 {
            self.rvr as u64
        } else {
            self.cvr as u64
        };

        // Convert timer ticks to processor cycles
        if self.uses_processor_clock() {
            Some(timer_ticks_remaining)
        } else if self.external_clock > 0 {
            Some((timer_ticks_remaining * self.processor_clock) / self.external_clock)
        } else {
            None
        }
    }

    /// Get tick period in processor cycles.
    pub fn tick_period_cycles(&self) -> Option<u64> {
        if self.rvr == 0 {
            return None;
        }

        let timer_ticks = self.rvr as u64 + 1;

        if self.uses_processor_clock() {
            Some(timer_ticks)
        } else if self.external_clock > 0 {
            Some((timer_ticks * self.processor_clock) / self.external_clock)
        } else {
            None
        }
    }

    /// Get tick frequency in Hz.
    pub fn tick_frequency(&self) -> Option<f64> {
        if self.rvr == 0 {
            return None;
        }

        let clock = self.clock_frequency() as f64;
        let reload = self.rvr as f64 + 1.0;
        Some(clock / reload)
    }
}

impl Default for SysTick {
    fn default() -> Self {
        // Default to 100 MHz processor clock
        Self::with_frequency(100_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systick_basic() {
        let mut systick = SysTick::with_frequency(100_000_000);

        assert!(!systick.is_enabled());
        assert_eq!(systick.read_cvr(), 0);
        assert_eq!(systick.read_rvr(), 0);
    }

    #[test]
    fn test_systick_countdown() {
        let mut systick = SysTick::with_frequency(1_000_000); // 1 MHz

        // Set reload value to 1000 (1ms at 1MHz)
        systick.write_rvr(1000);
        assert_eq!(systick.read_rvr(), 1000);

        // Enable timer with processor clock
        systick.write_csr(csr::ENABLE | csr::CLKSOURCE);
        assert!(systick.is_enabled());
        assert!(systick.uses_processor_clock());

        // Write to CVR to clear and start from reload value
        systick.write_cvr(0);

        // Tick 500 cycles
        assert!(!systick.tick(500));
        assert_eq!(systick.read_cvr(), 500);

        // Tick 500 more cycles - should underflow
        assert!(systick.tick(500));
        assert!(systick.count_flag);

        // Read CSR should clear count flag
        let csr = systick.read_csr();
        assert!(csr & csr::COUNTFLAG != 0);
        assert!(!systick.count_flag);
    }

    #[test]
    fn test_systick_interrupt() {
        let mut systick = SysTick::with_frequency(1_000_000);

        systick.write_rvr(100);
        systick.write_csr(csr::ENABLE | csr::CLKSOURCE | csr::TICKINT);
        systick.write_cvr(0);

        assert!(!systick.is_pending());

        // Tick to underflow
        systick.tick(100);
        assert!(systick.is_pending());

        // Take pending should clear it
        assert!(systick.take_pending());
        assert!(!systick.is_pending());
    }

    #[test]
    fn test_systick_disabled() {
        let mut systick = SysTick::with_frequency(1_000_000);

        systick.write_rvr(100);
        // Not enabled
        systick.write_cvr(0);

        // Ticking should have no effect
        assert!(!systick.tick(1000));
    }

    #[test]
    fn test_systick_calib() {
        // With external clock
        let systick = SysTick::new(100_000_000, 32768, 327);
        let calib = systick.read_calib();
        assert_eq!(calib & 0x00FF_FFFF, 327);
        assert!(calib & (1 << 31) == 0); // NOREF = 0 (has external clock)

        // Without external clock
        let systick = SysTick::new(100_000_000, 0, 0);
        let calib = systick.read_calib();
        assert!(calib & (1 << 31) != 0); // NOREF = 1
        assert!(calib & (1 << 30) != 0); // SKEW = 1 (no calibration)
    }

    #[test]
    fn test_systick_timing() {
        let mut systick = SysTick::with_frequency(100_000_000); // 100 MHz

        // 1ms tick at 100MHz = 100,000 cycles
        systick.write_rvr(99_999); // Reload value is N-1 for N cycles
        systick.write_csr(csr::ENABLE | csr::CLKSOURCE);
        systick.write_cvr(0);

        // Tick frequency should be 1000 Hz (1ms period)
        let freq = systick.tick_frequency().unwrap();
        assert!((freq - 1000.0).abs() < 0.01);

        // Period should be 100,000 cycles
        let period = systick.tick_period_cycles().unwrap();
        assert_eq!(period, 100_000);
    }

    #[test]
    fn test_systick_multiple_underflows() {
        let mut systick = SysTick::with_frequency(1_000);

        systick.write_rvr(10); // Very short period
        systick.write_csr(csr::ENABLE | csr::CLKSOURCE | csr::TICKINT);
        systick.write_cvr(0);

        // Tick enough cycles for multiple underflows
        systick.tick(35);

        // Should have pending interrupt (at least one underflow)
        assert!(systick.is_pending());
    }

    #[test]
    fn test_systick_register_access() {
        let mut systick = SysTick::with_frequency(1_000_000);

        // Write through register interface
        systick.write(0x04, 0x00FF_FFFF); // RVR
        assert_eq!(systick.read(0x04), 0x00FF_FFFF);

        systick.write(0x00, 0x07); // CSR
        assert_eq!(systick.read(0x00) & 0x07, 0x07);

        // CVR write clears to 0
        systick.write(0x08, 0x12345678);
        assert_eq!(systick.read(0x08), 0);
    }
}
