//! Motorola MC146818 Real-Time Clock / CMOS RAM emulation.
//!
//! The MC146818 (and its IBM-PC AT derivatives) exposes 128 bytes of
//! battery-backed CMOS RAM through a two-port index/data interface:
//!
//! - Port `0x70`: index/address register. Writes select which of the 128
//!   CMOS bytes the data port accesses. Bit 7 of a write is the NMI-disable
//!   bit (1 = NMI masked) and is *not* part of the register index, so the
//!   index is masked to `0x7F`.
//! - Port `0x71`: data register. Reads/writes the currently selected byte.
//!
//! The low 14 bytes (indices `0x00`-`0x0D`) are the clock/control registers;
//! the remaining bytes are general-purpose CMOS RAM, conventionally holding
//! BIOS configuration such as the equipment word and memory-size fields.
//!
//! Time is sourced from the host wall clock (`std::time::SystemTime`). The
//! time/date registers are materialised on demand when read, honouring
//! Register B's binary/BCD (bit 2) and 24/12-hour (bit 1) formatting bits.
//!
//! ## IRQ8
//!
//! On real hardware the RTC asserts IRQ8 for periodic, alarm and
//! update-ended events. The [`IoDevice`] bus trait in this emulator carries
//! no interrupt-callback channel, so — matching the convention used by the
//! PIT and 16550 UART devices ([`has_pending_interrupt`]/[`clear_interrupt`])
//! — the device tracks the interrupt-flag state faithfully in Register C and
//! exposes a poll method. Call [`Rtc::tick`] periodically to advance the
//! periodic/alarm/update-ended flags; reading Register C returns and clears
//! the flags and deasserts the (pollable) IRQ8 line.
//!
//! [`has_pending_interrupt`]: Rtc::has_pending_interrupt
//! [`clear_interrupt`]: Rtc::clear_interrupt

use std::time::{SystemTime, UNIX_EPOCH};

use super::bus::IoDevice;

/// CMOS/RTC index (address) port.
pub const RTC_ADDRESS: u16 = 0x70;
/// CMOS/RTC data port.
pub const RTC_DATA: u16 = 0x71;

// --- CMOS register indices ---------------------------------------------------

const REG_SECONDS: u8 = 0x00;
const REG_SECONDS_ALARM: u8 = 0x01;
const REG_MINUTES: u8 = 0x02;
const REG_MINUTES_ALARM: u8 = 0x03;
const REG_HOURS: u8 = 0x04;
const REG_HOURS_ALARM: u8 = 0x05;
const REG_DAY_OF_WEEK: u8 = 0x06;
const REG_DAY_OF_MONTH: u8 = 0x07;
const REG_MONTH: u8 = 0x08;
const REG_YEAR: u8 = 0x09;
const REG_A: u8 = 0x0a;
const REG_B: u8 = 0x0b;
const REG_C: u8 = 0x0c;
const REG_D: u8 = 0x0d;

/// IBM-PC convention: century is stored in CMOS byte `0x32`.
const REG_CENTURY: u8 = 0x32;

// CMOS memory-size bytes (kept stable so a guest BIOS / kernel sees sane RAM).
const REG_BASE_MEM_LO: u8 = 0x15;
const REG_BASE_MEM_HI: u8 = 0x16;
const REG_EXT_MEM_LO: u8 = 0x17;
const REG_EXT_MEM_HI: u8 = 0x18;
const REG_EXT_MEM_LO_2: u8 = 0x30;
const REG_EXT_MEM_HI_2: u8 = 0x31;

// --- Register A bits ---------------------------------------------------------

/// Register A bit 7: Update In Progress.
const REG_A_UIP: u8 = 0x80;
/// Register A rate-select mask (bits 0-3).
const REG_A_RATE_MASK: u8 = 0x0f;

// --- Register B bits ---------------------------------------------------------

/// Daylight Saving Enable.
const REG_B_DSE: u8 = 0x01;
/// 24-hour mode (1 = 24h, 0 = 12h).
const REG_B_24H: u8 = 0x02;
/// Data Mode (1 = binary, 0 = BCD).
const REG_B_DM: u8 = 0x04;
/// Square-Wave Enable.
const REG_B_SQWE: u8 = 0x08;
/// Update-ended Interrupt Enable.
const REG_B_UIE: u8 = 0x10;
/// Alarm Interrupt Enable.
const REG_B_AIE: u8 = 0x20;
/// Periodic Interrupt Enable.
const REG_B_PIE: u8 = 0x40;
/// SET bit: when 1, updates are inhibited (UIP is held clear).
const REG_B_SET: u8 = 0x80;

// --- Register C bits ---------------------------------------------------------

/// Update-ended interrupt Flag.
const REG_C_UF: u8 = 0x10;
/// Alarm interrupt Flag.
const REG_C_AF: u8 = 0x20;
/// Periodic interrupt Flag.
const REG_C_PF: u8 = 0x40;
/// Interrupt Request Flag (set when any enabled source flagged).
const REG_C_IRQF: u8 = 0x80;

// --- Register D bits ---------------------------------------------------------

/// Valid RAM and Time / battery-good bit.
const REG_D_VRT: u8 = 0x80;

/// 12-hour-mode PM indicator (OR'd into the hours register).
const HOUR_PM_BIT: u8 = 0x80;

/// A broken-down wall-clock time in UTC, as fed to the RTC registers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DateTime {
    second: u8,
    minute: u8,
    hour: u8,
    /// Day of week, 1 = Sunday .. 7 = Saturday (MC146818 convention).
    day_of_week: u8,
    day_of_month: u8,
    month: u8,
    /// Full four-digit year (e.g. 2026).
    year: u16,
}

/// Motorola MC146818 RTC / CMOS device.
///
/// The public name `RtcStub` and the no-argument [`RtcStub::new`] constructor
/// are retained for source compatibility with the device registration in
/// `arch/x86_64`. [`Rtc`] is provided as a clearer alias.
pub struct RtcStub {
    /// 128 bytes of CMOS RAM, including the control registers at 0x0A-0x0D.
    cmos: [u8; 128],
    /// Currently selected register index (0x00-0x7F).
    index: u8,
    /// NMI-disable state, tracked from bit 7 of the last index-port write.
    /// `true` means NMIs are masked.
    nmi_disabled: bool,
}

/// Clearer alias for the MC146818 device. Equivalent to [`RtcStub`].
pub type Rtc = RtcStub;

impl RtcStub {
    /// Create a new MC146818 with sane power-on register defaults.
    pub fn new() -> Self {
        let mut cmos = [0u8; 128];

        // Register A: divider configured (32.768 kHz) + 1024 Hz periodic rate.
        // Top nibble 0b010 selects the divider chain; low nibble 0b0110 = rate 6.
        cmos[REG_A as usize] = 0x26;
        // Register B: 24-hour mode, binary-coded-decimal (DM=0). No interrupts.
        cmos[REG_B as usize] = REG_B_24H;
        // Register C: no pending interrupts.
        cmos[REG_C as usize] = 0x00;
        // Register D: RAM/battery valid.
        cmos[REG_D as usize] = REG_D_VRT;

        // Memory-size fields. Base memory is fixed at 640 KiB by convention.
        let base_kib: u16 = 640;
        cmos[REG_BASE_MEM_LO as usize] = base_kib as u8;
        cmos[REG_BASE_MEM_HI as usize] = (base_kib >> 8) as u8;

        // Extended memory above 1 MiB. Report 15 MiB (capped at 0xFFFF KiB,
        // which is what real CMOS clamps to) split across both mirror pairs.
        let ext_kib: u16 = 15 * 1024;
        cmos[REG_EXT_MEM_LO as usize] = ext_kib as u8;
        cmos[REG_EXT_MEM_HI as usize] = (ext_kib >> 8) as u8;
        cmos[REG_EXT_MEM_LO_2 as usize] = ext_kib as u8;
        cmos[REG_EXT_MEM_HI_2 as usize] = (ext_kib >> 8) as u8;

        RtcStub {
            cmos,
            index: 0,
            nmi_disabled: false,
        }
    }

    /// Whether NMIs are currently masked (bit 7 of the last index write).
    pub fn nmi_disabled(&self) -> bool {
        self.nmi_disabled
    }

    /// Whether an enabled IRQ8 source currently has a pending flag set.
    ///
    /// Mirrors the [`crate::devices::pit::Pit`] / 16550 polling convention:
    /// the [`IoDevice`] trait exposes no interrupt callback, so the consumer
    /// polls this and clears it by reading Register C (or [`clear_interrupt`]).
    ///
    /// [`clear_interrupt`]: Rtc::clear_interrupt
    pub fn has_pending_interrupt(&self) -> bool {
        self.cmos[REG_C as usize] & REG_C_IRQF != 0
    }

    /// Clear all Register C interrupt flags and deassert IRQ8.
    pub fn clear_interrupt(&mut self) {
        self.cmos[REG_C as usize] = 0;
    }

    /// Read the (raw) value of Register C without clearing it. Test helper.
    #[cfg(test)]
    fn peek_reg_c(&self) -> u8 {
        self.cmos[REG_C as usize]
    }

    /// Convert a binary value to BCD (e.g. 23 -> 0x23).
    fn to_bcd(value: u8) -> u8 {
        ((value / 10) << 4) | (value % 10)
    }

    /// Format a binary value into the data-mode currently selected by Reg B.
    fn format_value(&self, value: u8) -> u8 {
        if self.cmos[REG_B as usize] & REG_B_DM != 0 {
            value // binary
        } else {
            Self::to_bcd(value)
        }
    }

    /// Format the hours field honouring 24/12-hour mode (Reg B bit 1) and the
    /// binary/BCD data mode.
    fn format_hours(&self, hour24: u8) -> u8 {
        if self.cmos[REG_B as usize] & REG_B_24H != 0 {
            // 24-hour mode.
            self.format_value(hour24)
        } else {
            // 12-hour mode. Hour 0 -> 12 AM, 12 -> 12 PM, 13.. -> 1.. PM.
            let pm = hour24 >= 12;
            let mut hour12 = hour24 % 12;
            if hour12 == 0 {
                hour12 = 12;
            }
            let formatted = self.format_value(hour12);
            if pm {
                formatted | HOUR_PM_BIT
            } else {
                formatted
            }
        }
    }

    /// Compute the current host wall-clock time, broken down in UTC.
    fn now() -> DateTime {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self::from_unix_seconds(secs)
    }

    /// Break a Unix timestamp (seconds since 1970-01-01 UTC) into civil time.
    ///
    /// Uses Howard Hinnant's `civil_from_days` algorithm for the date part.
    fn from_unix_seconds(secs: u64) -> DateTime {
        let days = (secs / 86_400) as i64;
        let secs_of_day = secs % 86_400;

        let hour = (secs_of_day / 3_600) as u8;
        let minute = ((secs_of_day % 3_600) / 60) as u8;
        let second = (secs_of_day % 60) as u8;

        // Day of week: 1970-01-01 was a Thursday. MC146818 uses 1 = Sunday.
        // (days + 4) mod 7 gives 0 = Sunday .. 6 = Saturday; +1 to map to 1..7.
        let dow = (((days % 7) + 4) % 7 + 7) % 7; // 0 = Sunday
        let day_of_week = (dow as u8) + 1;

        // civil_from_days: shift epoch so the year starts on March 1.
        let z = days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097; // [0, 146096]
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
        let mp = (5 * doy + 2) / 153; // [0, 11]
        let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
        let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
        let year = (y + if m <= 2 { 1 } else { 0 }) as u16;

        DateTime {
            second,
            minute,
            hour,
            day_of_week,
            day_of_month: d as u8,
            month: m as u8,
            year,
        }
    }

    /// Materialise a time/date register value from the current host clock.
    fn read_time_register(&self, index: u8) -> u8 {
        let now = Self::now();
        match index {
            REG_SECONDS => self.format_value(now.second),
            REG_MINUTES => self.format_value(now.minute),
            REG_HOURS => self.format_hours(now.hour),
            REG_DAY_OF_WEEK => self.format_value(now.day_of_week),
            REG_DAY_OF_MONTH => self.format_value(now.day_of_month),
            REG_MONTH => self.format_value(now.month),
            REG_YEAR => self.format_value((now.year % 100) as u8),
            REG_CENTURY => self.format_value((now.year / 100) as u8),
            _ => 0,
        }
    }

    /// Whether the alarm registers match the current time. An alarm byte with
    /// its top two bits set (>= 0xC0) is a "don't care" wildcard per the spec.
    fn alarm_matches(&self, now: &DateTime) -> bool {
        let dm = self.cmos[REG_B as usize] & REG_B_DM != 0;
        let to_cmp = |v: u8| if dm { v } else { Self::to_bcd(v) };

        let check = |alarm: u8, current: u8| -> bool {
            // Don't-care: any alarm value with both high bits set.
            (alarm & 0xc0) == 0xc0 || alarm == current
        };

        check(self.cmos[REG_SECONDS_ALARM as usize], to_cmp(now.second))
            && check(self.cmos[REG_MINUTES_ALARM as usize], to_cmp(now.minute))
            && check(self.cmos[REG_HOURS_ALARM as usize], to_cmp(now.hour))
    }

    /// Recompute the Register C IRQF summary bit from the source flags and the
    /// matching enable bits in Register B.
    fn update_irqf(&mut self) {
        let b = self.cmos[REG_B as usize];
        let mut c = self.cmos[REG_C as usize];
        let enabled = ((b & REG_B_PIE) != 0 && (c & REG_C_PF) != 0)
            || ((b & REG_B_AIE) != 0 && (c & REG_C_AF) != 0)
            || ((b & REG_B_UIE) != 0 && (c & REG_C_UF) != 0);
        if enabled {
            c |= REG_C_IRQF;
        } else {
            c &= !REG_C_IRQF;
        }
        self.cmos[REG_C as usize] = c;
    }

    /// Advance interrupt state. Should be called periodically by the VMM loop.
    ///
    /// `elapsed_nanos` is the time since the previous call, used to decide
    /// whether one or more periodic-interrupt ticks have elapsed. Update-ended
    /// and alarm sources are evaluated against the live wall clock.
    ///
    /// Returns `true` if IRQ8 is asserted after this call.
    pub fn tick(&mut self, elapsed_nanos: u64) -> bool {
        let b = self.cmos[REG_B as usize];

        // While the SET bit is asserted, updates are frozen — no update-ended
        // or alarm processing occurs.
        if b & REG_B_SET == 0 {
            let now = Self::now();

            // Update-ended interrupt: flagged once per second. We approximate
            // "an update ended" by flagging whenever at least one second of
            // wall-clock has been crossed since construction; here we flag on
            // any tick where the update would have completed.
            self.cmos[REG_C as usize] |= REG_C_UF;

            // Alarm interrupt.
            if self.alarm_matches(&now) {
                self.cmos[REG_C as usize] |= REG_C_AF;
            }
        }

        // Periodic interrupt: if a non-zero rate is selected, any elapsed time
        // beyond a single period flags PF. A rate of 0 disables the divider.
        if let Some(period_ns) = self.periodic_period_nanos() {
            if elapsed_nanos >= period_ns {
                self.cmos[REG_C as usize] |= REG_C_PF;
            }
        }

        self.update_irqf();
        self.has_pending_interrupt()
    }

    /// Periodic-interrupt period in nanoseconds for the current Register A
    /// rate-select bits, or `None` if periodic interrupts are disabled.
    fn periodic_period_nanos(&self) -> Option<u64> {
        let rate = self.cmos[REG_A as usize] & REG_A_RATE_MASK;
        if rate == 0 {
            return None;
        }
        // Frequency = 32768 >> (rate - 1) Hz. Period = 1e9 / freq ns.
        let freq = 32_768u64 >> (rate - 1);
        if freq == 0 {
            None
        } else {
            Some(1_000_000_000 / freq)
        }
    }

    /// Read the currently selected CMOS register.
    fn read_register(&mut self) -> u8 {
        match self.index {
            REG_SECONDS | REG_MINUTES | REG_HOURS | REG_DAY_OF_WEEK | REG_DAY_OF_MONTH
            | REG_MONTH | REG_YEAR | REG_CENTURY => self.read_time_register(self.index),
            REG_A => {
                // Derive UIP from the host sub-second: assert it briefly near
                // the top of each second, mirroring real hardware so spin-wait
                // loops on UIP can make progress. Held clear while SET is set.
                let base = self.cmos[REG_A as usize] & !REG_A_UIP;
                if self.cmos[REG_B as usize] & REG_B_SET != 0 {
                    base
                } else {
                    let nanos = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.subsec_nanos())
                        .unwrap_or(0);
                    // UIP true for the last ~244us of each second (per spec the
                    // update cycle takes 1984us; we use a short window).
                    if nanos >= 999_756_000 {
                        base | REG_A_UIP
                    } else {
                        base
                    }
                }
            }
            REG_C => {
                // Reading C returns the flags and CLEARS them (deasserts IRQ8).
                let value = self.cmos[REG_C as usize];
                self.cmos[REG_C as usize] = 0;
                value
            }
            REG_D => {
                // Battery / RAM always reported valid.
                self.cmos[REG_D as usize] | REG_D_VRT
            }
            other => self.cmos[other as usize],
        }
    }

    /// Write the currently selected CMOS register.
    fn write_register(&mut self, value: u8) {
        match self.index {
            REG_A => {
                // UIP (bit 7) is read-only; preserve everything else.
                self.cmos[REG_A as usize] = value & !REG_A_UIP;
            }
            REG_B => {
                self.cmos[REG_B as usize] = value;
                // Enable changes may gate an already-set flag.
                self.update_irqf();
            }
            REG_C | REG_D => {
                // Registers C and D are read-only.
            }
            other => {
                self.cmos[other as usize] = value;
            }
        }
    }
}

impl Default for RtcStub {
    fn default() -> Self {
        Self::new()
    }
}

impl IoDevice for RtcStub {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            RTC_ADDRESS => {
                // The index port read-back includes the NMI-disable bit.
                self.index | if self.nmi_disabled { 0x80 } else { 0 }
            }
            RTC_DATA => self.read_register(),
            _ => 0xff,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            RTC_ADDRESS => {
                self.nmi_disabled = value & 0x80 != 0;
                self.index = value & 0x7f;
            }
            RTC_DATA => self.write_register(value),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rtc() -> RtcStub {
        RtcStub::new()
    }

    // --- Index / data port behaviour ----------------------------------------

    #[test]
    fn index_write_masks_to_7f_and_tracks_nmi() {
        let mut rtc = make_rtc();

        // Write index with NMI-disable bit set.
        rtc.write(RTC_ADDRESS, 0x80 | 0x0b);
        assert_eq!(rtc.index, 0x0b, "index must be masked to low 7 bits");
        assert!(rtc.nmi_disabled(), "bit 7 set must disable NMI");

        // Read-back of the index port includes the NMI bit.
        assert_eq!(rtc.read(RTC_ADDRESS), 0x80 | 0x0b);

        // Clearing bit 7 re-enables NMI.
        rtc.write(RTC_ADDRESS, 0x0c);
        assert!(!rtc.nmi_disabled());
        assert_eq!(rtc.index, 0x0c);
        assert_eq!(rtc.read(RTC_ADDRESS), 0x0c);
    }

    #[test]
    fn general_cmos_ram_read_write_roundtrip() {
        let mut rtc = make_rtc();
        // Pick a general-purpose byte (0x40 is RAM, not a clock/control reg).
        rtc.write(RTC_ADDRESS, 0x40);
        rtc.write(RTC_DATA, 0xa5);
        rtc.write(RTC_ADDRESS, 0x40);
        assert_eq!(rtc.read(RTC_DATA), 0xa5);
    }

    #[test]
    fn unhandled_port_reads_ff() {
        let mut rtc = make_rtc();
        assert_eq!(rtc.read(0x72), 0xff);
    }

    // --- Register defaults ---------------------------------------------------

    #[test]
    fn register_d_reports_valid_ram() {
        let mut rtc = make_rtc();
        rtc.write(RTC_ADDRESS, REG_D);
        assert_ne!(rtc.read(RTC_DATA) & REG_D_VRT, 0, "VRT bit must be set");
    }

    #[test]
    fn register_d_is_read_only() {
        let mut rtc = make_rtc();
        rtc.write(RTC_ADDRESS, REG_D);
        rtc.write(RTC_DATA, 0x00);
        assert_ne!(
            rtc.read(RTC_DATA) & REG_D_VRT,
            0,
            "VRT still set after write"
        );
    }

    #[test]
    fn memory_size_bytes_are_sane() {
        let mut rtc = make_rtc();
        let read = |rtc: &mut RtcStub, idx: u8| {
            rtc.write(RTC_ADDRESS, idx);
            rtc.read(RTC_DATA)
        };
        let base = (read(&mut rtc, REG_BASE_MEM_LO) as u16)
            | ((read(&mut rtc, REG_BASE_MEM_HI) as u16) << 8);
        assert_eq!(base, 640, "base memory should be 640 KiB");

        let ext = (read(&mut rtc, REG_EXT_MEM_LO) as u16)
            | ((read(&mut rtc, REG_EXT_MEM_HI) as u16) << 8);
        assert_eq!(ext, 15 * 1024, "extended memory should be 15 MiB");

        let ext2 = (read(&mut rtc, REG_EXT_MEM_LO_2) as u16)
            | ((read(&mut rtc, REG_EXT_MEM_HI_2) as u16) << 8);
        assert_eq!(ext2, 15 * 1024, "extended memory mirror should match");
    }

    // --- BCD helper ----------------------------------------------------------

    #[test]
    fn bcd_helper() {
        assert_eq!(RtcStub::to_bcd(0), 0x00);
        assert_eq!(RtcStub::to_bcd(9), 0x09);
        assert_eq!(RtcStub::to_bcd(10), 0x10);
        assert_eq!(RtcStub::to_bcd(23), 0x23);
        assert_eq!(RtcStub::to_bcd(59), 0x59);
        assert_eq!(RtcStub::to_bcd(99), 0x99);
    }

    // --- Date breakdown ------------------------------------------------------

    #[test]
    fn unix_epoch_breaks_down_correctly() {
        // 1970-01-01 00:00:00 UTC, a Thursday.
        let dt = RtcStub::from_unix_seconds(0);
        assert_eq!(dt.year, 1970);
        assert_eq!(dt.month, 1);
        assert_eq!(dt.day_of_month, 1);
        assert_eq!(dt.hour, 0);
        assert_eq!(dt.minute, 0);
        assert_eq!(dt.second, 0);
        assert_eq!(dt.day_of_week, 5, "1970-01-01 is Thursday (Sun=1)");
    }

    #[test]
    fn known_timestamp_breaks_down_correctly() {
        // 2026-06-01 12:34:56 UTC. (Computed from a Unix timestamp.)
        // 1748781296 = 2025-06-01 12:34:56? verify via algorithm: use a value
        // we can cross-check: 1735689600 = 2025-01-01 00:00:00 UTC (Wednesday).
        let dt = RtcStub::from_unix_seconds(1_735_689_600);
        assert_eq!(dt.year, 2025);
        assert_eq!(dt.month, 1);
        assert_eq!(dt.day_of_month, 1);
        assert_eq!(dt.hour, 0);
        assert_eq!(dt.minute, 0);
        assert_eq!(dt.second, 0);
        assert_eq!(dt.day_of_week, 4, "2025-01-01 is Wednesday (Sun=1)");
    }

    // --- BCD vs binary formatting -------------------------------------------

    #[test]
    fn format_value_respects_data_mode() {
        let mut rtc = make_rtc();

        // Default: BCD (DM=0).
        assert_eq!(rtc.cmos[REG_B as usize] & REG_B_DM, 0);
        assert_eq!(rtc.format_value(42), 0x42);

        // Switch to binary.
        rtc.write(RTC_ADDRESS, REG_B);
        rtc.write(RTC_DATA, REG_B_24H | REG_B_DM);
        assert_eq!(rtc.format_value(42), 42);
    }

    // --- 12 / 24-hour formatting --------------------------------------------

    #[test]
    fn hours_24h_mode() {
        let mut rtc = make_rtc();
        // 24h, binary.
        rtc.cmos[REG_B as usize] = REG_B_24H | REG_B_DM;
        assert_eq!(rtc.format_hours(0), 0);
        assert_eq!(rtc.format_hours(13), 13);
        assert_eq!(rtc.format_hours(23), 23);

        // 24h, BCD.
        rtc.cmos[REG_B as usize] = REG_B_24H;
        assert_eq!(rtc.format_hours(23), 0x23);
    }

    #[test]
    fn hours_12h_mode_sets_pm_bit() {
        let mut rtc = make_rtc();
        // 12h, binary (clear the 24h bit, keep DM).
        rtc.cmos[REG_B as usize] = REG_B_DM;

        assert_eq!(rtc.format_hours(0), 12, "midnight -> 12 AM");
        assert_eq!(rtc.format_hours(1), 1, "01:00 -> 1 AM");
        assert_eq!(rtc.format_hours(11), 11, "11:00 -> 11 AM");
        assert_eq!(rtc.format_hours(12), 12 | HOUR_PM_BIT, "noon -> 12 PM");
        assert_eq!(rtc.format_hours(13), 1 | HOUR_PM_BIT, "13:00 -> 1 PM");
        assert_eq!(rtc.format_hours(23), 11 | HOUR_PM_BIT, "23:00 -> 11 PM");

        // 12h, BCD.
        rtc.cmos[REG_B as usize] = 0;
        assert_eq!(rtc.format_hours(13), 0x01 | HOUR_PM_BIT);
        assert_eq!(rtc.format_hours(22), 0x10 | HOUR_PM_BIT);
    }

    // --- Register A ----------------------------------------------------------

    #[test]
    fn register_a_uip_is_read_only_on_write() {
        let mut rtc = make_rtc();
        rtc.write(RTC_ADDRESS, REG_A);
        // Attempt to set UIP via a write — it must be stripped.
        rtc.write(RTC_DATA, REG_A_UIP | 0x20 | 0x06);
        assert_eq!(
            rtc.cmos[REG_A as usize] & REG_A_UIP,
            0,
            "UIP must not be settable by software"
        );
        assert_eq!(rtc.cmos[REG_A as usize] & 0x7f, 0x26);
    }

    #[test]
    fn periodic_period_for_rate_6_is_1024hz() {
        let rtc = make_rtc(); // default rate nibble = 6
        // 32768 >> (6-1) = 32768 >> 5 = 1024 Hz. Period ~= 976562 ns.
        assert_eq!(rtc.periodic_period_nanos(), Some(1_000_000_000 / 1024));
    }

    #[test]
    fn periodic_disabled_when_rate_zero() {
        let mut rtc = make_rtc();
        rtc.write(RTC_ADDRESS, REG_A);
        rtc.write(RTC_DATA, 0x20); // divider bits set, rate = 0
        assert_eq!(rtc.periodic_period_nanos(), None);
    }

    // --- Register C read-clears ---------------------------------------------

    #[test]
    fn reg_c_read_clears_flags() {
        let mut rtc = make_rtc();
        // Force some flags on directly.
        rtc.cmos[REG_C as usize] = REG_C_IRQF | REG_C_PF | REG_C_UF;
        assert_eq!(rtc.peek_reg_c(), REG_C_IRQF | REG_C_PF | REG_C_UF);

        rtc.write(RTC_ADDRESS, REG_C);
        let value = rtc.read(RTC_DATA);
        assert_eq!(
            value,
            REG_C_IRQF | REG_C_PF | REG_C_UF,
            "read returns flags"
        );

        // A subsequent read must come back clear.
        rtc.write(RTC_ADDRESS, REG_C);
        assert_eq!(rtc.read(RTC_DATA), 0, "Register C must self-clear on read");
        assert!(!rtc.has_pending_interrupt());
    }

    #[test]
    fn reg_c_is_read_only_on_write() {
        let mut rtc = make_rtc();
        rtc.cmos[REG_C as usize] = REG_C_PF;
        rtc.write(RTC_ADDRESS, REG_C);
        rtc.write(RTC_DATA, 0x00); // should be ignored
        assert_eq!(rtc.peek_reg_c(), REG_C_PF);
    }

    // --- Periodic interrupt via tick ----------------------------------------

    #[test]
    fn periodic_interrupt_raises_irq8_when_enabled() {
        let mut rtc = make_rtc();
        // Enable periodic interrupts (PIE) but keep SET to freeze update/alarm
        // so the only source is periodic.
        rtc.write(RTC_ADDRESS, REG_B);
        rtc.write(RTC_DATA, REG_B_24H | REG_B_DM | REG_B_PIE | REG_B_SET);

        // Tick with more than one period elapsed.
        let raised = rtc.tick(1_000_000_000); // 1 second >> period
        assert!(raised, "PIE + elapsed period must assert IRQ8");
        assert!(rtc.has_pending_interrupt());
        assert_ne!(rtc.peek_reg_c() & REG_C_PF, 0);
        assert_ne!(rtc.peek_reg_c() & REG_C_IRQF, 0);

        // Reading Register C clears the line.
        rtc.write(RTC_ADDRESS, REG_C);
        let _ = rtc.read(RTC_DATA);
        assert!(!rtc.has_pending_interrupt());
    }

    #[test]
    fn periodic_flag_not_raised_when_pie_disabled() {
        let mut rtc = make_rtc();
        // PIE disabled, SET on (freeze other sources).
        rtc.write(RTC_ADDRESS, REG_B);
        rtc.write(RTC_DATA, REG_B_24H | REG_B_DM | REG_B_SET);

        let raised = rtc.tick(1_000_000_000);
        assert!(!raised, "no enabled source => IRQ8 deasserted");
        // The PF source flag may set, but IRQF must not without PIE.
        assert_eq!(rtc.peek_reg_c() & REG_C_IRQF, 0);
    }

    // --- Alarm match ---------------------------------------------------------

    #[test]
    fn alarm_matches_with_wildcards() {
        let mut rtc = make_rtc();
        // Binary data mode for simpler reasoning.
        rtc.cmos[REG_B as usize] = REG_B_24H | REG_B_DM;

        let now = DateTime {
            second: 30,
            minute: 15,
            hour: 9,
            day_of_week: 2,
            day_of_month: 1,
            month: 6,
            year: 2026,
        };

        // Exact match on all three alarm fields.
        rtc.cmos[REG_SECONDS_ALARM as usize] = 30;
        rtc.cmos[REG_MINUTES_ALARM as usize] = 15;
        rtc.cmos[REG_HOURS_ALARM as usize] = 9;
        assert!(rtc.alarm_matches(&now));

        // Wrong second => no match.
        rtc.cmos[REG_SECONDS_ALARM as usize] = 31;
        assert!(!rtc.alarm_matches(&now));

        // Don't-care second (>= 0xC0) => match again.
        rtc.cmos[REG_SECONDS_ALARM as usize] = 0xff;
        assert!(rtc.alarm_matches(&now));
    }

    #[test]
    fn alarm_matches_in_bcd_mode() {
        let mut rtc = make_rtc();
        // BCD mode (default DM=0), 24h.
        rtc.cmos[REG_B as usize] = REG_B_24H;

        let now = DateTime {
            second: 5,
            minute: 42,
            hour: 23,
            day_of_week: 1,
            day_of_month: 1,
            month: 1,
            year: 2026,
        };

        // Alarm registers hold BCD-encoded values in BCD mode.
        rtc.cmos[REG_SECONDS_ALARM as usize] = RtcStub::to_bcd(5);
        rtc.cmos[REG_MINUTES_ALARM as usize] = RtcStub::to_bcd(42);
        rtc.cmos[REG_HOURS_ALARM as usize] = RtcStub::to_bcd(23);
        assert!(rtc.alarm_matches(&now));
    }

    #[test]
    fn alarm_interrupt_raises_irq8() {
        let mut rtc = make_rtc();
        // Enable alarm interrupts, binary mode, do NOT set SET so alarm runs.
        rtc.cmos[REG_B as usize] = REG_B_24H | REG_B_DM | REG_B_AIE;

        // Program all-wildcard alarm so it matches the live host time.
        rtc.cmos[REG_SECONDS_ALARM as usize] = 0xff;
        rtc.cmos[REG_MINUTES_ALARM as usize] = 0xff;
        rtc.cmos[REG_HOURS_ALARM as usize] = 0xff;

        let raised = rtc.tick(0);
        assert!(raised, "AIE + matching alarm must assert IRQ8");
        assert_ne!(rtc.peek_reg_c() & REG_C_AF, 0);
    }

    // --- Update-ended interrupt ---------------------------------------------

    #[test]
    fn update_ended_interrupt_raises_irq8() {
        let mut rtc = make_rtc();
        rtc.cmos[REG_B as usize] = REG_B_24H | REG_B_DM | REG_B_UIE;
        let raised = rtc.tick(0);
        assert!(raised, "UIE must assert IRQ8 on update-ended");
        assert_ne!(rtc.peek_reg_c() & REG_C_UF, 0);
    }

    #[test]
    fn set_bit_freezes_update_and_alarm() {
        let mut rtc = make_rtc();
        // UIE + AIE enabled but SET asserted: update/alarm sources frozen.
        rtc.cmos[REG_B as usize] = REG_B_24H | REG_B_DM | REG_B_UIE | REG_B_AIE | REG_B_SET;
        rtc.cmos[REG_SECONDS_ALARM as usize] = 0xff;
        rtc.cmos[REG_MINUTES_ALARM as usize] = 0xff;
        rtc.cmos[REG_HOURS_ALARM as usize] = 0xff;

        let raised = rtc.tick(0);
        assert!(!raised, "SET must freeze update-ended and alarm sources");
        assert_eq!(rtc.peek_reg_c() & (REG_C_UF | REG_C_AF), 0);
    }

    // --- Live time-register reads --------------------------------------------

    #[test]
    fn reading_time_registers_produces_valid_bcd() {
        let mut rtc = make_rtc(); // default: BCD, 24h
        rtc.write(RTC_ADDRESS, REG_SECONDS);
        let sec = rtc.read(RTC_DATA);
        // Valid BCD: each nibble 0-9, seconds 0-59.
        assert!((sec >> 4) <= 5, "tens-of-seconds nibble must be 0-5");
        assert!((sec & 0x0f) <= 9, "units nibble must be 0-9");

        rtc.write(RTC_ADDRESS, REG_MONTH);
        let month = rtc.read(RTC_DATA);
        assert!(
            month >= 0x01 && month <= 0x12,
            "month BCD 1-12, got {month:#x}"
        );
    }

    #[test]
    fn century_register_is_reasonable() {
        let mut rtc = make_rtc();
        rtc.write(RTC_ADDRESS, REG_CENTURY);
        let century = rtc.read(RTC_DATA);
        // 21st century -> 20 decimal -> 0x20 in BCD.
        assert_eq!(century, 0x20, "century should be 20 (BCD) for 20xx");
    }
}
