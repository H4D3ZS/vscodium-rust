//! Intel 8253/8254 Programmable Interval Timer (PIT) emulation.
//!
//! The PIT provides three independent 16-bit counters (channels 0-2):
//! - Channel 0: System timer, generates IRQ 0
//! - Channel 1: Originally DRAM refresh (not used in modern systems)
//! - Channel 2: PC speaker (gated by port 0x61 bit 0, OUT readable at bit 5)
//!
//! I/O ports:
//! - 0x40: Channel 0 data
//! - 0x41: Channel 1 data
//! - 0x42: Channel 2 data
//! - 0x43: Mode/Command register
//!
//! Timing is based on wall-clock time for real-time behavior: each channel's
//! current count and OUT level are derived from the number of PIT oscillator
//! ticks elapsed since the counter was (re)loaded.
//!
//! ## Operating modes implemented
//! - Mode 0 (interrupt on terminal count): OUT low after reload, goes high when
//!   the count reaches 0 and stays high.
//! - Mode 1 (hardware retriggerable one-shot): gate-triggered; OUT modelled but
//!   triggering is partial (the PIT only sees the gate level, not edges).
//! - Mode 2 (rate generator): OUT high, pulses low for one tick at terminal
//!   count, reloads and repeats.
//! - Mode 3 (square wave): OUT toggles, ~50% duty cycle, reloads each half period.
//! - Mode 4 (software-triggered strobe): OUT high, strobes low for one tick when
//!   the count reaches 0, then stays high until reprogrammed.
//! - Mode 5 (hardware-triggered strobe): gate-triggered strobe; partial, see Mode 1.

use crate::timing;
use serde::{Deserialize, Serialize};

use super::bus::IoDevice;

/// PIT oscillator frequency (1.193182 MHz). Re-exported from [`timing`] so the
/// constant is documented alongside the device; the actual nanos<->tick math
/// lives in [`timing::nanos_to_pit_ticks`].
#[cfg(test)]
const PIT_FREQUENCY: u64 = timing::PIT_FREQUENCY_HZ;

/// Default reload value for ~100 Hz (10ms period)
const DEFAULT_RELOAD: u16 = 11932;

/// Counter access mode (control-word bits 5:4). The latch command (RW field 0)
/// is handled as a transient action and never becomes a persistent access mode,
/// so it is not represented here.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum AccessMode {
    LowByteOnly,
    HighByteOnly,
    LowHighByte,
}

impl AccessMode {
    /// Encode this access mode into the 2-bit RW field used in the control
    /// word / read-back status byte (bits 5:4).
    fn rw_bits(self) -> u8 {
        match self {
            AccessMode::LowByteOnly => 1,
            AccessMode::HighByteOnly => 2,
            AccessMode::LowHighByte => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum OperatingMode {
    InterruptOnTerminalCount,     // Mode 0
    HardwareRetriggerableOneShot, // Mode 1
    RateGenerator,                // Mode 2
    SquareWaveGenerator,          // Mode 3
    SoftwareTriggeredStrobe,      // Mode 4
    HardwareTriggeredStrobe,      // Mode 5
}

impl OperatingMode {
    /// The 3-bit mode field as it appears in the control / status byte.
    /// Modes 6/7 are aliases of 2/3 and are normalised to 2/3 here.
    fn mode_bits(self) -> u8 {
        match self {
            OperatingMode::InterruptOnTerminalCount => 0,
            OperatingMode::HardwareRetriggerableOneShot => 1,
            OperatingMode::RateGenerator => 2,
            OperatingMode::SquareWaveGenerator => 3,
            OperatingMode::SoftwareTriggeredStrobe => 4,
            OperatingMode::HardwareTriggeredStrobe => 5,
        }
    }
}

/// Tracks which byte of a lo/hi access the read or write flip-flop expects next.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum BytePhase {
    Low,
    High,
}

#[derive(Clone, Serialize, Deserialize)]
struct Channel {
    reload_value: u16,
    count: u16,
    access_mode: AccessMode,
    operating_mode: OperatingMode,
    /// BCD counting selected (control word bit 0).
    bcd: bool,
    /// Read latch for counter values. Uses u32 to allow bit 16 as a marker
    /// for "high byte only" state after first read in LowHighByte mode.
    read_latch: Option<u32>,
    write_latch: Option<u8>,
    /// Read flip-flop phase for LowHighByte access (which byte is next).
    read_phase: BytePhase,
    gate: bool,
    output: bool,
    /// Null-count flag: set when a new reload value is written into the control
    /// logic but not yet transferred into the actual counting element. Cleared
    /// once counting actually starts. Reported by the read-back status command.
    null_count: bool,
    /// Wall-clock nanoseconds at which the current count period started. The
    /// live count/OUT level is derived from the ticks elapsed since this point.
    loaded_at_nanos: u64,
}

impl Default for Channel {
    fn default() -> Self {
        Channel {
            reload_value: DEFAULT_RELOAD,
            count: DEFAULT_RELOAD,
            access_mode: AccessMode::LowHighByte,
            operating_mode: OperatingMode::RateGenerator,
            bcd: false,
            read_latch: None,
            write_latch: None,
            read_phase: BytePhase::Low,
            gate: true,
            output: false,
            null_count: false,
            loaded_at_nanos: 0,
        }
    }
}

impl Channel {
    /// Effective reload period in PIT ticks (0 means the full 0x10000).
    fn period(&self) -> u32 {
        if self.reload_value == 0 {
            0x10000
        } else {
            self.reload_value as u32
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Pit {
    channels: [Channel; 3],
    /// Wall-clock nanoseconds at last tick
    last_tick_nanos: u64,
    irq_pending: bool,
    tick_count: u64,
}

impl Pit {
    pub fn new() -> Self {
        let now = timing::elapsed_nanos();
        let mut channels = [Channel::default(), Channel::default(), Channel::default()];
        for ch in &mut channels {
            ch.loaded_at_nanos = now;
        }
        // Channel 2's gate is driven externally (port 0x61 bit 0) and starts low.
        channels[2].gate = false;
        Pit {
            channels,
            last_tick_nanos: now,
            irq_pending: false,
            tick_count: 0,
        }
    }

    /// Check if a timer interrupt is pending
    pub fn has_pending_interrupt(&self) -> bool {
        self.irq_pending
    }

    /// Clear the pending interrupt
    pub fn clear_interrupt(&mut self) {
        self.irq_pending = false;
    }

    // ---- Channel 2 gate / OUT wiring (driven by the 0x61 sysctl device) -----

    /// Set channel 2's GATE input (port 0x61 bit 0). When the gate transitions
    /// the channel's timing reference is reloaded so the count restarts.
    pub fn set_channel2_gate(&mut self, level: bool) {
        let ch = &mut self.channels[2];
        if ch.gate != level {
            ch.gate = level;
            // Reload the timing reference on a gate change so counting resumes
            // from the reload value (sufficient for the gate-triggered modes and
            // correct for the level-gated modes 2/3).
            ch.loaded_at_nanos = timing::elapsed_nanos();
            ch.count = ch.reload_value;
        }
    }

    /// Current channel 2 GATE input level.
    pub fn channel2_gate(&self) -> bool {
        self.channels[2].gate
    }

    /// Current channel 2 OUT level (readable via port 0x61 bit 5).
    ///
    /// Computed from elapsed wall-clock time so polling reflects the live
    /// square-wave / strobe output that calibration loops expect.
    pub fn channel2_out(&self) -> bool {
        self.compute(2).1
    }

    /// Tick the timer - should be called periodically.
    /// Returns true if a (channel 0) timer interrupt should be generated.
    pub fn tick(&mut self) -> bool {
        let now = timing::elapsed_nanos();

        // Refresh the live count/OUT for every channel from elapsed time.
        for channel in 0..3 {
            let (count, output) = self.compute(channel);
            let ch = &mut self.channels[channel];
            ch.count = count;
            // Once counting has progressed past the initial load the new count
            // has been transferred into the counting element.
            if ch.loaded_at_nanos != now {
                ch.null_count = false;
            }
            ch.output = output;
        }

        // Channel 0 drives IRQ 0. Determine whether the OUT line produced a
        // rising edge (mode 0) or a terminal-count event (modes 2/3/4) since the
        // last tick, using the number of completed periods.
        let fired = self.channels[0].gate && self.channel0_interrupt(now);

        self.last_tick_nanos = now;

        if fired {
            self.irq_pending = true;
            self.tick_count += 1;
        }
        fired
    }

    /// Decide whether channel 0 should raise IRQ 0 this tick.
    ///
    /// We compare the elapsed time at the previous tick and at `now`: if at
    /// least one terminal-count boundary was crossed, the timer fired.
    fn channel0_interrupt(&self, now: u64) -> bool {
        let ch = &self.channels[0];
        let period = ch.period() as u64;
        if period == 0 {
            return false;
        }

        let start = ch.loaded_at_nanos;
        let prev = self.last_tick_nanos.max(start);
        if now <= prev {
            return false;
        }

        let ticks_prev = timing::nanos_to_pit_ticks(prev.saturating_sub(start));
        let ticks_now = timing::nanos_to_pit_ticks(now.saturating_sub(start));

        match ch.operating_mode {
            OperatingMode::InterruptOnTerminalCount
            | OperatingMode::HardwareRetriggerableOneShot => {
                // One-shot: fires exactly once, when the count first reaches 0.
                ticks_prev < period && ticks_now >= period
            }
            OperatingMode::RateGenerator
            | OperatingMode::SquareWaveGenerator
            | OperatingMode::SoftwareTriggeredStrobe
            | OperatingMode::HardwareTriggeredStrobe => {
                // Periodic: count how many terminal counts elapsed in (prev, now].
                (ticks_now / period) > (ticks_prev / period)
            }
        }
    }

    /// Compute the live (count, OUT) pair for a channel from elapsed wall-clock
    /// time, without mutating state. This is the heart of the timing model.
    fn compute(&self, channel: usize) -> (u16, bool) {
        let ch = &self.channels[channel];
        let now = timing::elapsed_nanos();
        let elapsed = now.saturating_sub(ch.loaded_at_nanos);
        let elapsed_ticks = timing::nanos_to_pit_ticks(elapsed);
        self.compute_from_ticks(channel, elapsed_ticks)
    }

    /// Pure (count, OUT) computation for a channel given an explicit number of
    /// elapsed PIT oscillator ticks since the counter was loaded. Split out from
    /// [`Self::compute`] so the mode behavior is deterministically testable
    /// without depending on real wall-clock time.
    fn compute_from_ticks(&self, channel: usize, elapsed_ticks: u64) -> (u16, bool) {
        let ch = &self.channels[channel];
        let period = ch.period();

        // With the gate low, level-gated modes (2/3) freeze counting and force a
        // defined OUT level; the count holds at its last value.
        if !ch.gate {
            match ch.operating_mode {
                // For the rate/square modes a low gate forces OUT high.
                OperatingMode::RateGenerator | OperatingMode::SquareWaveGenerator => {
                    return (ch.count, true);
                }
                _ => return (ch.count, ch.output),
            }
        }

        match ch.operating_mode {
            OperatingMode::InterruptOnTerminalCount
            | OperatingMode::HardwareRetriggerableOneShot => {
                // Counts down once to 0; OUT starts low, goes high at 0 and holds.
                if elapsed_ticks >= period as u64 {
                    (0, true)
                } else {
                    let remaining = period - elapsed_ticks as u32;
                    (Self::to_count(remaining, period, ch.bcd), false)
                }
            }
            OperatingMode::RateGenerator => {
                // OUT is high except for the single tick where the count is 1->0.
                let pos = (elapsed_ticks % period as u64) as u32;
                let remaining = period - pos; // counts period..1
                let out = remaining != 1;
                (Self::to_count(remaining, period, ch.bcd), out)
            }
            OperatingMode::SquareWaveGenerator => {
                // Square wave: OUT high for the first half, low for the second.
                // The counter is reloaded each half period.
                let half = period / 2;
                let in_period = (elapsed_ticks % period as u64) as u32;
                let out = in_period < half.max(1);
                // Count decrements by 2 each tick (approximated to the half-period
                // window for the purpose of reads).
                let pos_in_half = if out { in_period } else { in_period - half };
                let span = if out { half.max(1) } else { period - half };
                let remaining = span.saturating_sub(pos_in_half).max(1);
                (Self::to_count(remaining * 2, period, ch.bcd), out)
            }
            OperatingMode::SoftwareTriggeredStrobe | OperatingMode::HardwareTriggeredStrobe => {
                // OUT high until terminal count, strobes low for exactly one tick.
                if elapsed_ticks >= period as u64 {
                    let pos = (elapsed_ticks % period as u64) as u32;
                    let out = pos != 0; // low only on the exact terminal-count tick
                    let remaining = if pos == 0 { period } else { period - pos };
                    (Self::to_count(remaining, period, ch.bcd), out)
                } else {
                    let remaining = period - elapsed_ticks as u32;
                    (Self::to_count(remaining, period, ch.bcd), true)
                }
            }
        }
    }

    /// Clamp a raw binary count into the 16-bit counter, honoring BCD when set.
    fn to_count(value: u32, period: u32, bcd: bool) -> u16 {
        let v = value.min(period).max(if value == 0 { 0 } else { 1 });
        if bcd {
            Self::bin_to_bcd(v as u16 % 10000)
        } else {
            v as u16
        }
    }

    /// Convert a binary value (0..=9999) into packed BCD.
    fn bin_to_bcd(mut v: u16) -> u16 {
        v %= 10000;
        ((v / 1000) << 12) | (((v / 100) % 10) << 8) | (((v / 10) % 10) << 4) | (v % 10)
    }

    /// Convert packed BCD back into binary (for written reload values).
    fn bcd_to_bin(v: u16) -> u16 {
        (v >> 12) * 1000 + ((v >> 8) & 0xF) * 100 + ((v >> 4) & 0xF) * 10 + (v & 0xF)
    }

    /// Marker bit to indicate high-byte-only latch (bit 16 set in u32)
    /// Valid count values are 0x0000-0xFFFF, so bit 16 is safe to use as marker
    const HIGH_BYTE_ONLY_MARKER: u32 = 0x10000;

    /// Build the read-back status byte for a channel:
    /// bit7 = OUT, bit6 = null-count, bits5:4 = RW mode, bits3:1 = mode, bit0 = BCD.
    fn status_byte(&self, channel: usize) -> u8 {
        let (_, out) = self.compute(channel);
        let ch = &self.channels[channel];
        let mut status = 0u8;
        if out {
            status |= 1 << 7;
        }
        if ch.null_count {
            status |= 1 << 6;
        }
        status |= ch.access_mode.rw_bits() << 4;
        status |= ch.operating_mode.mode_bits() << 1;
        if ch.bcd {
            status |= 1;
        }
        status
    }

    /// Latch the current (time-computed) count of a channel for reading, unless
    /// a latch is already pending (spec: a second latch before reading is
    /// ignored).
    fn latch_count(&mut self, channel: usize) {
        let count = self.compute(channel).0;
        let ch = &mut self.channels[channel];
        if ch.read_latch.is_none() {
            ch.read_latch = Some(count as u32);
            ch.read_phase = BytePhase::Low;
        }
    }

    fn read_channel(&mut self, channel: usize) -> u8 {
        // Check if we have a latched value first
        if let Some(latch) = self.channels[channel].read_latch {
            let ch = &mut self.channels[channel];
            match ch.access_mode {
                AccessMode::LowByteOnly => {
                    ch.read_latch = None;
                    (latch & 0xFF) as u8
                }
                AccessMode::LowHighByte => {
                    // Check if this is a high-byte-only latch (after first read)
                    // Bit 16 is set for high-byte-only, clear for full 16-bit latch
                    if latch & Self::HIGH_BYTE_ONLY_MARKER != 0 {
                        // Second read: return the high byte, clear latch
                        ch.read_latch = None;
                        ch.read_phase = BytePhase::Low;
                        (latch & 0xFF) as u8
                    } else {
                        // First read: return low byte, keep high byte for next read
                        // Mark with HIGH_BYTE_ONLY_MARKER to indicate second read pending
                        ch.read_latch = Some(Self::HIGH_BYTE_ONLY_MARKER | (latch >> 8));
                        ch.read_phase = BytePhase::High;
                        (latch & 0xFF) as u8
                    }
                }
                AccessMode::HighByteOnly => {
                    ch.read_latch = None;
                    ((latch >> 8) & 0xFF) as u8
                }
            }
        } else {
            // Calculate current count based on wall-clock time
            let count = self.compute(channel).0;
            let ch = &mut self.channels[channel];
            match ch.access_mode {
                AccessMode::LowByteOnly => (count & 0xFF) as u8,
                AccessMode::HighByteOnly => (count >> 8) as u8,
                AccessMode::LowHighByte => match ch.read_phase {
                    BytePhase::Low => {
                        ch.read_phase = BytePhase::High;
                        (count & 0xFF) as u8
                    }
                    BytePhase::High => {
                        ch.read_phase = BytePhase::Low;
                        (count >> 8) as u8
                    }
                },
            }
        }
    }

    fn write_channel(&mut self, channel: usize, value: u8) {
        let ch = &mut self.channels[channel];

        let mut reloaded = false;
        match ch.access_mode {
            AccessMode::LowByteOnly => {
                // Per spec: "the respective other 8 bits are zeroed"
                ch.reload_value = value as u16;
                reloaded = true;
            }
            AccessMode::HighByteOnly => {
                // Per spec: "the respective other 8 bits are zeroed"
                ch.reload_value = (value as u16) << 8;
                reloaded = true;
            }
            AccessMode::LowHighByte => {
                if let Some(low) = ch.write_latch {
                    // Second byte (high)
                    ch.reload_value = (low as u16) | ((value as u16) << 8);
                    ch.write_latch = None;
                    reloaded = true;
                } else {
                    // First byte (low). Writing the first byte of a two-byte load
                    // marks the count as not-yet-loaded (null count is set).
                    ch.write_latch = Some(value);
                    ch.null_count = true;
                }
            }
        }

        if reloaded {
            // Decode BCD reload values into a binary divisor for the timing math.
            let mut reload = ch.reload_value;
            if ch.bcd {
                reload = Self::bcd_to_bin(reload);
            }
            ch.reload_value = reload;
            ch.count = ch.reload_value;
            ch.loaded_at_nanos = timing::elapsed_nanos();
            // The count is now loaded into the counting element.
            ch.null_count = false;
            // Establish the initial OUT level for the new mode.
            ch.output = matches!(
                ch.operating_mode,
                OperatingMode::RateGenerator
                    | OperatingMode::SquareWaveGenerator
                    | OperatingMode::SoftwareTriggeredStrobe
                    | OperatingMode::HardwareTriggeredStrobe
            );
        }
    }

    fn write_command(&mut self, value: u8) {
        let channel = ((value >> 6) & 0x03) as usize;

        if channel == 3 {
            self.read_back(value);
            return;
        }

        let access = (value >> 4) & 0x03;
        let mode = (value >> 1) & 0x07;
        let bcd = value & 0x01 != 0;

        // Access == 0 is the counter-latch command and does NOT reprogram mode.
        if access == 0 {
            self.latch_count(channel);
            return;
        }

        let ch = &mut self.channels[channel];

        ch.access_mode = match access {
            1 => AccessMode::LowByteOnly,
            2 => AccessMode::HighByteOnly,
            3 => AccessMode::LowHighByte,
            _ => unreachable!(),
        };

        ch.operating_mode = match mode {
            0 => OperatingMode::InterruptOnTerminalCount,
            1 => OperatingMode::HardwareRetriggerableOneShot,
            2 | 6 => OperatingMode::RateGenerator,
            3 | 7 => OperatingMode::SquareWaveGenerator,
            4 => OperatingMode::SoftwareTriggeredStrobe,
            5 => OperatingMode::HardwareTriggeredStrobe,
            _ => unreachable!(),
        };

        ch.bcd = bcd;
        ch.write_latch = None;
        ch.read_phase = BytePhase::Low;
        // Programming the control word arms the counter: null count is set until
        // a reload value is written, and OUT assumes its mode's initial level.
        ch.null_count = true;
        ch.output = !matches!(ch.operating_mode, OperatingMode::InterruptOnTerminalCount);
    }

    /// Read-back command (8254): control word 0b11xxxxxx.
    /// bit5 = !latch-count, bit4 = !latch-status, bits3:1 = channel select.
    fn read_back(&mut self, value: u8) {
        let latch_count = value & (1 << 5) == 0;
        let latch_status = value & (1 << 4) == 0;

        for (channel, bit) in [(0usize, 1u8 << 1), (1, 1 << 2), (2, 1 << 3)] {
            if value & bit == 0 {
                continue;
            }
            // Spec: with both count and status requested, the status byte is
            // returned by the first read and the count bytes by the following
            // reads. Modelling that combined sequence needs a separate status
            // slot; we DEFER it. Behaviour here:
            //   - status-only  -> latch the status byte for the next read
            //   - count (with or without status) -> latch the count
            // i.e. when both bits are set we currently prioritise the count.
            if latch_status && !latch_count {
                let status = self.status_byte(channel) as u32;
                let ch = &mut self.channels[channel];
                if ch.read_latch.is_none() {
                    ch.read_latch = Some(status);
                    ch.read_phase = BytePhase::Low;
                }
            } else if latch_count {
                self.latch_count(channel);
            }
        }
    }
}

impl IoDevice for Pit {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            0x40 => self.read_channel(0),
            0x41 => self.read_channel(1),
            0x42 => self.read_channel(2),
            0x43 => 0xFF, // Command register is write-only
            _ => 0xFF,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            0x40 => self.write_channel(0, value),
            0x41 => self.write_channel(1, value),
            0x42 => self.write_channel(2, value),
            0x43 => self.write_command(value),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a fresh PIT for testing
    fn make_pit() -> Pit {
        // Initialize timing to ensure consistent behavior
        crate::timing::init();
        Pit::new()
    }

    // ========== Basic Construction and Defaults ==========

    #[test]
    fn test_pit_new_default_state() {
        let pit = make_pit();
        assert!(!pit.irq_pending);
        assert_eq!(pit.tick_count, 0);
        assert_eq!(pit.channels[0].reload_value, DEFAULT_RELOAD);
        assert_eq!(pit.channels[0].count, DEFAULT_RELOAD);
    }

    #[test]
    fn test_default_channel_settings() {
        let pit = make_pit();
        // Channels 0 and 1 default gate high; channel 2's gate is externally
        // driven (port 0x61) and starts low.
        for (i, ch) in pit.channels.iter().enumerate() {
            assert_eq!(ch.access_mode, AccessMode::LowHighByte);
            assert!(matches!(ch.operating_mode, OperatingMode::RateGenerator));
            if i == 2 {
                assert!(!ch.gate);
            } else {
                assert!(ch.gate);
            }
            assert!(!ch.output);
            assert!(ch.read_latch.is_none());
            assert!(ch.write_latch.is_none());
        }
    }

    // ========== I/O Port Mapping ==========

    #[test]
    fn test_io_port_mapping() {
        let mut pit = make_pit();
        assert_eq!(pit.read(0x43), 0xFF);
        assert_eq!(pit.read(0x44), 0xFF);
        assert_eq!(pit.read(0x50), 0xFF);
    }

    // ========== Command Register Parsing ==========

    #[test]
    fn test_command_channel_selection() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36);
        assert!(matches!(
            pit.channels[0].operating_mode,
            OperatingMode::SquareWaveGenerator
        ));
        pit.write(0x43, 0x74);
        assert!(matches!(
            pit.channels[1].operating_mode,
            OperatingMode::RateGenerator
        ));
        pit.write(0x43, 0xB0);
        assert!(matches!(
            pit.channels[2].operating_mode,
            OperatingMode::InterruptOnTerminalCount
        ));
    }

    #[test]
    fn test_command_access_mode_parsing() {
        let mut pit = make_pit();
        pit.write(0x43, 0x10);
        assert_eq!(pit.channels[0].access_mode, AccessMode::LowByteOnly);
        pit.write(0x43, 0x20);
        assert_eq!(pit.channels[0].access_mode, AccessMode::HighByteOnly);
        pit.write(0x43, 0x30);
        assert_eq!(pit.channels[0].access_mode, AccessMode::LowHighByte);
    }

    #[test]
    fn test_command_operating_modes() {
        let mut pit = make_pit();
        let cases = [
            (0x30u8, OperatingMode::InterruptOnTerminalCount),
            (0x32, OperatingMode::HardwareRetriggerableOneShot),
            (0x34, OperatingMode::RateGenerator),
            (0x36, OperatingMode::SquareWaveGenerator),
            (0x38, OperatingMode::SoftwareTriggeredStrobe),
            (0x3A, OperatingMode::HardwareTriggeredStrobe),
            (0x3C, OperatingMode::RateGenerator), // mode 6 -> 2
            (0x3E, OperatingMode::SquareWaveGenerator), // mode 7 -> 3
        ];
        for (cmd, expected) in cases {
            pit.write(0x43, cmd);
            assert_eq!(pit.channels[0].operating_mode, expected, "cmd {cmd:#x}");
        }
    }

    #[test]
    fn test_bcd_bit_decoded() {
        let mut pit = make_pit();
        // Mode 3, lo/hi, BCD set.
        pit.write(0x43, 0x37);
        assert!(pit.channels[0].bcd);
        // Mode 3, lo/hi, binary.
        pit.write(0x43, 0x36);
        assert!(!pit.channels[0].bcd);
    }

    // ========== Counter Latch Command ==========

    #[test]
    fn test_counter_latch_command() {
        let mut pit = make_pit();
        pit.channels[0].count = 0x1234;
        pit.channels[0].reload_value = 0x1234;
        pit.channels[0].access_mode = AccessMode::LowHighByte;
        pit.channels[0].operating_mode = OperatingMode::InterruptOnTerminalCount;
        pit.channels[0].gate = false; // freeze so compute() returns count
        pit.write(0x43, 0x00);
        assert_eq!(pit.channels[0].read_latch, Some(0x1234));
    }

    #[test]
    fn test_counter_latch_preserves_until_read() {
        let mut pit = make_pit();
        pit.channels[0].count = 0xABCD;
        pit.channels[0].reload_value = 0xABCD;
        pit.channels[0].access_mode = AccessMode::LowHighByte;
        pit.channels[0].gate = false;

        pit.write(0x43, 0x00);
        assert_eq!(pit.channels[0].read_latch, Some(0xABCD));

        // Change actual count; latched value must persist.
        pit.channels[0].count = 0x1234;

        let low = pit.read(0x40);
        assert_eq!(low, 0xCD);
        let high = pit.read(0x40);
        assert_eq!(high, 0xAB);
        assert!(pit.channels[0].read_latch.is_none());
    }

    #[test]
    fn test_multiple_latch_commands_ignored() {
        let mut pit = make_pit();
        pit.channels[0].count = 0x1111;
        pit.channels[0].reload_value = 0x1111;
        pit.channels[0].access_mode = AccessMode::LowHighByte;
        pit.channels[0].gate = false;
        pit.write(0x43, 0x00); // latch

        pit.channels[0].count = 0x2222;
        pit.write(0x43, 0x00); // ignored

        let low = pit.read(0x40);
        let high = pit.read(0x40);
        assert_eq!((high as u16) << 8 | low as u16, 0x1111);
    }

    #[test]
    fn test_latch_does_not_change_mode() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36); // mode 3
        pit.write(0x43, 0x00); // latch command on channel 0
        assert!(matches!(
            pit.channels[0].operating_mode,
            OperatingMode::SquareWaveGenerator
        ));
    }

    // ========== Access Modes / flip-flop ==========

    #[test]
    fn test_low_byte_only_write() {
        let mut pit = make_pit();
        pit.write(0x43, 0x10);
        pit.write(0x40, 0x42);
        assert_eq!(pit.channels[0].reload_value, 0x0042);
    }

    #[test]
    fn test_high_byte_only_write() {
        let mut pit = make_pit();
        pit.write(0x43, 0x20);
        pit.write(0x40, 0x42);
        assert_eq!(pit.channels[0].reload_value & 0xFF00, 0x4200);
    }

    #[test]
    fn test_lobyte_hibyte_write_sequence() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36);
        pit.write(0x40, 0x34);
        assert_eq!(pit.channels[0].write_latch, Some(0x34));
        // Null count set after first byte of a two-byte load.
        assert!(pit.channels[0].null_count);
        pit.write(0x40, 0x12);
        assert!(pit.channels[0].write_latch.is_none());
        assert_eq!(pit.channels[0].reload_value, 0x1234);
        // Once loaded, null count clears.
        assert!(!pit.channels[0].null_count);
    }

    #[test]
    fn test_read_flip_flop_unlatched_lohi() {
        let mut pit = make_pit();
        pit.channels[0].reload_value = 0xBEEF;
        pit.channels[0].count = 0xBEEF;
        pit.channels[0].access_mode = AccessMode::LowHighByte;
        pit.channels[0].gate = false; // freeze count at 0xBEEF
        // Two consecutive unlatched reads return low then high byte.
        let lo = pit.read(0x40);
        let hi = pit.read(0x40);
        assert_eq!(lo, 0xEF);
        assert_eq!(hi, 0xBE);
        // Flip-flop reset back to low for the next pair.
        let lo2 = pit.read(0x40);
        assert_eq!(lo2, 0xEF);
    }

    #[test]
    fn test_low_byte_only_read() {
        let mut pit = make_pit();
        pit.channels[0].reload_value = 0xABCD;
        pit.channels[0].count = 0xABCD;
        pit.channels[0].access_mode = AccessMode::LowByteOnly;
        pit.channels[0].gate = false;
        assert_eq!(pit.read(0x40), 0xCD);
    }

    #[test]
    fn test_high_byte_only_read() {
        let mut pit = make_pit();
        pit.channels[0].reload_value = 0xABCD;
        pit.channels[0].count = 0xABCD;
        pit.channels[0].access_mode = AccessMode::HighByteOnly;
        pit.channels[0].gate = false;
        assert_eq!(pit.read(0x40), 0xAB);
    }

    // ========== Reload Value of 0 = 65536 ==========

    #[test]
    fn test_reload_value_zero_means_65536() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36);
        pit.write(0x40, 0x00);
        pit.write(0x40, 0x00);
        assert_eq!(pit.channels[0].reload_value, 0);
        assert_eq!(pit.channels[0].period(), 0x10000);
    }

    // ========== Channel 2 Data Port ==========

    #[test]
    fn test_channel_2_read_write() {
        let mut pit = make_pit();
        pit.write(0x43, 0xB6);
        pit.write(0x42, 0xEF);
        pit.write(0x42, 0xBE);
        assert_eq!(pit.channels[2].reload_value, 0xBEEF);
    }

    // ========== Read-Back Command ==========

    #[test]
    fn test_readback_latch_count() {
        let mut pit = make_pit();
        pit.write(0x43, 0x30); // ch0 mode 0, lo/hi
        pit.channels[0].reload_value = 0x1234;
        pit.channels[0].count = 0x1234;
        pit.channels[0].gate = false; // freeze count

        // Read-back: latch count for channel 0.
        // 0b11 channel, bit5=0 latch-count, bit4=1 no status, bit1=1 select ch0.
        // = 1100_0010 = 0xC2
        pit.write(0x43, 0xC2);
        let lo = pit.read(0x40);
        let hi = pit.read(0x40);
        assert_eq!((hi as u16) << 8 | lo as u16, 0x1234);
    }

    #[test]
    fn test_readback_latch_status() {
        let mut pit = make_pit();
        // Configure channel 0: mode 3 (square wave), lo/hi, binary.
        pit.write(0x43, 0x36);
        pit.channels[0].gate = false; // forces OUT high for mode 3
        pit.channels[0].null_count = false;

        // Read-back status only for channel 0:
        // 0b11 channel, bit5=1 no count, bit4=0 latch-status, bit1=1 select ch0.
        // = 1110_0010 = 0xE2
        pit.write(0x43, 0xE2);
        let status = pit.read(0x40);

        // OUT high (gate low forces square-wave OUT high) -> bit7 set.
        assert_eq!(status & 0x80, 0x80, "OUT bit");
        // RW = lo/hi = 11 in bits 5:4.
        assert_eq!((status >> 4) & 0x03, 0x03, "RW bits");
        // Mode = 3 in bits 3:1.
        assert_eq!((status >> 1) & 0x07, 3, "mode bits");
        // BCD clear.
        assert_eq!(status & 0x01, 0, "BCD bit");
    }

    #[test]
    fn test_readback_status_null_count_bit() {
        let mut pit = make_pit();
        // Program control word but do not write a reload value: null count set.
        pit.write(0x43, 0x34); // ch0 mode 2 lo/hi
        assert!(pit.channels[0].null_count);

        pit.write(0x43, 0xE2); // status only, ch0
        let status = pit.read(0x40);
        assert_eq!(status & 0x40, 0x40, "null-count bit should be set");
    }

    #[test]
    fn test_readback_multiple_channels_count() {
        let mut pit = make_pit();
        for (cmd, ch) in [(0x30u8, 0usize), (0x70, 1), (0xB0, 2)] {
            pit.write(0x43, cmd); // mode 0 lo/hi per channel
            pit.channels[ch].reload_value = 0x1000 + ch as u16;
            pit.channels[ch].count = 0x1000 + ch as u16;
            pit.channels[ch].gate = false;
        }
        // Latch count for all three channels at once.
        // bits: 0b11, bit5=0 count, bit4=1 no status, select ch0|ch1|ch2 = bits1,2,3
        // = 1100_1110 = 0xCE
        pit.write(0x43, 0xCE);
        for (port, ch) in [(0x40u16, 0usize), (0x41, 1), (0x42, 2)] {
            let lo = pit.read(port);
            let hi = pit.read(port);
            assert_eq!((hi as u16) << 8 | lo as u16, 0x1000 + ch as u16);
        }
    }

    // ========== Status byte construction ==========

    #[test]
    fn test_status_byte_fields() {
        let mut pit = make_pit();
        pit.write(0x43, 0x77); // ch1, lo/hi, mode 3, BCD
        pit.channels[1].null_count = false;
        pit.channels[1].gate = false; // mode 3 OUT high when gate low
        let s = pit.status_byte(1);
        assert_eq!(s & 0x80, 0x80); // OUT high
        assert_eq!((s >> 4) & 0x03, 3); // lo/hi
        assert_eq!((s >> 1) & 0x07, 3); // mode 3
        assert_eq!(s & 1, 1); // BCD
    }

    // ========== Interrupt Generation ==========

    #[test]
    fn test_has_pending_interrupt() {
        let mut pit = make_pit();
        assert!(!pit.has_pending_interrupt());
        pit.irq_pending = true;
        assert!(pit.has_pending_interrupt());
    }

    #[test]
    fn test_clear_interrupt() {
        let mut pit = make_pit();
        pit.irq_pending = true;
        pit.clear_interrupt();
        assert!(!pit.irq_pending);
    }

    // ========== Mode OUT-pin behavior ==========

    #[test]
    fn test_mode0_out_low_then_high() {
        let mut pit = make_pit();
        // Mode 0: OUT starts low, goes high at terminal count and holds.
        pit.channels[0].operating_mode = OperatingMode::InterruptOnTerminalCount;
        pit.channels[0].gate = true;
        let period: u16 = 100;
        pit.channels[0].reload_value = period;
        pit.channels[0].count = period;

        // Just loaded (0 ticks elapsed) -> OUT low, count == reload.
        let (c0, out0) = pit.compute_from_ticks(0, 0);
        assert!(!out0, "mode 0 OUT starts low");
        assert_eq!(c0, period);

        // Half way -> still low, count decreased.
        let (c1, out1) = pit.compute_from_ticks(0, 50);
        assert!(!out1);
        assert_eq!(c1, 50);

        // At/after terminal count -> OUT high, count 0.
        let (c2, out2) = pit.compute_from_ticks(0, period as u64);
        assert!(out2, "mode 0 OUT high at terminal count");
        assert_eq!(c2, 0);
        // Stays high afterwards.
        assert!(pit.compute_from_ticks(0, period as u64 + 500).1);
    }

    #[test]
    fn test_mode3_square_wave_out_toggles() {
        let mut pit = make_pit();
        pit.channels[0].operating_mode = OperatingMode::SquareWaveGenerator;
        pit.channels[0].gate = true;
        let period: u64 = 1000; // half = 500
        pit.channels[0].reload_value = period as u16;
        pit.channels[0].count = period as u16;

        // First half (0..500) -> OUT high.
        assert!(
            pit.compute_from_ticks(0, 0).1,
            "start of first half OUT high"
        );
        assert!(
            pit.compute_from_ticks(0, 250).1,
            "middle of first half OUT high"
        );
        assert!(
            pit.compute_from_ticks(0, 499).1,
            "end of first half OUT high"
        );

        // Second half (500..1000) -> OUT low.
        assert!(
            !pit.compute_from_ticks(0, 500).1,
            "start of second half OUT low"
        );
        assert!(
            !pit.compute_from_ticks(0, 750).1,
            "middle of second half OUT low"
        );
        assert!(
            !pit.compute_from_ticks(0, 999).1,
            "end of second half OUT low"
        );

        // Next cycle wraps back to high.
        assert!(
            pit.compute_from_ticks(0, 1000).1,
            "next cycle first half OUT high"
        );
        assert!(
            pit.compute_from_ticks(0, 1250).1,
            "next cycle first half OUT high"
        );
        assert!(
            !pit.compute_from_ticks(0, 1500).1,
            "next cycle second half OUT low"
        );
    }

    #[test]
    fn test_mode2_rate_generator_out_pulse() {
        let mut pit = make_pit();
        pit.channels[0].operating_mode = OperatingMode::RateGenerator;
        pit.channels[0].gate = true;
        let period: u64 = 1000;
        pit.channels[0].reload_value = period as u16;
        pit.channels[0].count = period as u16;

        // OUT is high for almost the whole period.
        let (count, out) = pit.compute_from_ticks(0, 0);
        assert!(out, "rate generator OUT high at start");
        assert_eq!(count, period as u16);

        // OUT high in the middle.
        assert!(pit.compute_from_ticks(0, 500).1);

        // OUT drops low for exactly the single tick where the count would reach 1.
        // remaining == 1 occurs at pos == period - 1.
        let (count_lo, out_lo) = pit.compute_from_ticks(0, period - 1);
        assert!(!out_lo, "rate generator OUT low at terminal count");
        assert_eq!(count_lo, 1);

        // Then reloads and OUT is high again next cycle.
        assert!(
            pit.compute_from_ticks(0, period).1,
            "OUT high again after reload"
        );
    }

    #[test]
    fn test_mode4_strobe_out_high_then_one_tick_low() {
        let mut pit = make_pit();
        pit.channels[0].operating_mode = OperatingMode::SoftwareTriggeredStrobe;
        pit.channels[0].gate = true;
        let period: u64 = 100;
        pit.channels[0].reload_value = period as u16;
        pit.channels[0].count = period as u16;

        // OUT high while counting down.
        assert!(pit.compute_from_ticks(0, 0).1, "mode 4 OUT high at start");
        assert!(pit.compute_from_ticks(0, 50).1, "mode 4 OUT high mid-count");

        // Strobes low for exactly the terminal-count tick.
        assert!(
            !pit.compute_from_ticks(0, period).1,
            "mode 4 strobes low at TC"
        );

        // Returns high on the following tick.
        assert!(
            pit.compute_from_ticks(0, period + 1).1,
            "mode 4 OUT high after strobe"
        );
    }

    // ========== Channel 2 gate / OUT wiring ==========

    #[test]
    fn test_channel2_gate_getter_setter() {
        let mut pit = make_pit();
        assert!(!pit.channel2_gate());
        pit.set_channel2_gate(true);
        assert!(pit.channel2_gate());
        pit.set_channel2_gate(false);
        assert!(!pit.channel2_gate());
    }

    #[test]
    fn test_channel2_out_readable() {
        let mut pit = make_pit();
        // Configure channel 2 as a square wave generator with a known period.
        pit.write(0x43, 0xB6); // ch2, lo/hi, mode 3
        pit.write(0x42, 0x00);
        pit.write(0x42, 0x10); // reload 0x1000
        // With the gate enabled, OUT is computed live; with gate disabled the
        // square-wave OUT is forced high.
        pit.set_channel2_gate(false);
        assert!(pit.channel2_out(), "square-wave OUT high while gated off");
        // Re-enabling the gate restarts counting; OUT begins in the high half.
        pit.set_channel2_gate(true);
        assert!(pit.channel2_out(), "first half of square wave OUT high");
    }

    #[test]
    fn test_channel2_gate_change_reloads() {
        let mut pit = make_pit();
        pit.write(0x43, 0xB4); // ch2 mode 2 lo/hi
        pit.write(0x42, 0x00);
        pit.write(0x42, 0x20); // reload 0x2000
        pit.set_channel2_gate(true);
        // Count was reset to the reload value on the gate rising edge.
        assert_eq!(pit.channels[2].count, 0x2000);
    }

    // ========== Standard Timer Configuration (like BIOS) ==========

    #[test]
    fn test_standard_100hz_configuration() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36);
        pit.write(0x40, 0x9C);
        pit.write(0x40, 0x2E);
        assert_eq!(pit.channels[0].reload_value, 0x2E9C);
        assert!(matches!(
            pit.channels[0].operating_mode,
            OperatingMode::SquareWaveGenerator
        ));
    }

    #[test]
    fn test_bios_default_18hz_configuration() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36);
        pit.write(0x40, 0x00);
        pit.write(0x40, 0x00);
        assert_eq!(pit.channels[0].reload_value, 0);
    }

    // ========== All Three Channels Independent ==========

    #[test]
    fn test_channels_independent() {
        let mut pit = make_pit();
        pit.write(0x43, 0x30); // Ch 0: mode 0
        pit.write(0x43, 0x76); // Ch 1: mode 3
        pit.write(0x43, 0xB4); // Ch 2: mode 2

        pit.write(0x40, 0x11);
        pit.write(0x40, 0x11);
        pit.write(0x41, 0x22);
        pit.write(0x41, 0x22);
        pit.write(0x42, 0x33);
        pit.write(0x42, 0x33);

        assert!(matches!(
            pit.channels[0].operating_mode,
            OperatingMode::InterruptOnTerminalCount
        ));
        assert!(matches!(
            pit.channels[1].operating_mode,
            OperatingMode::SquareWaveGenerator
        ));
        assert!(matches!(
            pit.channels[2].operating_mode,
            OperatingMode::RateGenerator
        ));

        assert_eq!(pit.channels[0].reload_value, 0x1111);
        assert_eq!(pit.channels[1].reload_value, 0x2222);
        assert_eq!(pit.channels[2].reload_value, 0x3333);
    }

    // ========== Write Latch Reset on Mode Change ==========

    #[test]
    fn test_write_latch_cleared_on_mode_change() {
        let mut pit = make_pit();
        pit.write(0x43, 0x36);
        pit.write(0x40, 0x12);
        assert!(pit.channels[0].write_latch.is_some());
        pit.write(0x43, 0x34);
        assert!(pit.channels[0].write_latch.is_none());
    }

    // ========== PIT Frequency Constant ==========

    #[test]
    fn test_pit_frequency_constant() {
        assert_eq!(PIT_FREQUENCY, 1193182);
    }

    // ========== BCD reload decode ==========

    #[test]
    fn test_bcd_reload_decode() {
        let mut pit = make_pit();
        // Mode 0, lo/hi, BCD. Write BCD value 0x1234 -> decimal 1234.
        pit.write(0x43, 0x31);
        pit.write(0x40, 0x34);
        pit.write(0x40, 0x12);
        assert_eq!(pit.channels[0].reload_value, 1234);
        assert!(pit.channels[0].bcd);
    }

    #[test]
    fn test_bcd_roundtrip_helpers() {
        assert_eq!(Pit::bin_to_bcd(1234), 0x1234);
        assert_eq!(Pit::bcd_to_bin(0x1234), 1234);
        assert_eq!(Pit::bcd_to_bin(0x9999), 9999);
        assert_eq!(Pit::bin_to_bcd(9999), 0x9999);
    }

    // ========== tick() drives IRQ on channel 0 ==========

    #[test]
    fn test_tick_fires_after_terminal_count_mode0() {
        let mut pit = make_pit();
        pit.channels[0].operating_mode = OperatingMode::InterruptOnTerminalCount;
        pit.channels[0].gate = true;
        pit.channels[0].reload_value = 100;
        pit.channels[0].count = 100;
        let now = timing::elapsed_nanos();
        pit.channels[0].loaded_at_nanos = now;
        pit.last_tick_nanos = now;

        // Back-date the load so the 100-tick period has fully elapsed by "now".
        let span = (200u128 * 1_000_000_000u128 / PIT_FREQUENCY as u128) as u64;
        pit.channels[0].loaded_at_nanos = now.saturating_sub(span);
        pit.last_tick_nanos = now.saturating_sub(span);

        assert!(pit.tick(), "mode 0 should fire once after terminal count");
        assert!(pit.has_pending_interrupt());
    }
}
