//! IA-PC HPET (High Precision Event Timer) emulation.
//!
//! Implements the High Precision Event Timer as described in the
//! "IA-PC HPET (High Precision Event Timers) Specification, Revision 1.0a".
//!
//! The HPET exposes a 1KB MMIO register block (the standard base for the
//! first event timer block is `0xFED00000`). It contains:
//! - A free-running up-counter (the "main counter") driven by a fixed-period
//!   oscillator. We derive its value from host wall-clock time so that guest
//!   delays elapse in real time, matching the LAPIC/PIT convention in this crate.
//! - `N` independent comparators ("timers"). Each can run in one-shot or
//!   periodic mode and raises a per-timer interrupt when the main counter
//!   matches its comparator value.
//!
//! Register layout (offsets from the MMIO base):
//! - 0x000  General Capabilities and ID (RO, 64-bit)
//! - 0x010  General Configuration       (RW, 64-bit)
//! - 0x020  General Interrupt Status     (RW1C, 64-bit)
//! - 0x0F0  Main Counter Value           (RW, 64-bit)
//! - 0x100 + 0x20*n  Timer n Config/Cap  (RW, 64-bit)
//! - 0x108 + 0x20*n  Timer n Comparator  (RW, 64-bit)
//! - 0x110 + 0x20*n  Timer n FSB Route   (RW, 64-bit)
//!
//! Interrupt delivery follows the polling convention used by the PIT/LAPIC
//! devices: this device does not invoke a callback. Instead the orchestrator
//! polls [`HpetDevice::has_pending_interrupt`] / [`HpetDevice::pending_timers`]
//! after calling [`HpetDevice::tick`], injects the appropriate interrupt, and
//! the guest clears the level by writing the matching RW1C status bit.

use super::bus::MmioDevice;
use std::time::Instant;

/// Standard MMIO base of the first HPET block (fixed by the IA-PC platform).
pub const HPET_BASE: u64 = 0xFED0_0000;
/// Size of the HPET MMIO region (1KB, as defined by the spec).
pub const HPET_SIZE: u64 = 0x400;

/// Number of comparators (event timers) we implement.
pub const NUM_TIMERS: usize = 3;

/// Counter period in femtoseconds per tick.
///
/// 10^8 fs = 100 ns => a 10 MHz main-counter frequency. This is a legal value
/// per the spec (which requires a period no larger than 100 ns / COUNTER_CLK_PERIOD
/// in the top 32 bits of the capabilities register).
pub const COUNTER_CLK_PERIOD_FS: u64 = 100_000_000;

/// Femtoseconds in one nanosecond.
const FS_PER_NS: u64 = 1_000_000;

/// Vendor ID reported in the capabilities register (bits 16-31). Use an
/// Intel-like value; the guest does not depend on a specific vendor.
const VENDOR_ID: u64 = 0x8086;

// ---- Register offsets ------------------------------------------------------

const REG_GEN_CAP_ID: u64 = 0x000;
const REG_GEN_CONFIG: u64 = 0x010;
const REG_GEN_INT_STATUS: u64 = 0x020;
const REG_MAIN_COUNTER: u64 = 0x0F0;
const REG_TIMER_BASE: u64 = 0x100;
const TIMER_STRIDE: u64 = 0x20;

// ---- General Capabilities and ID bits --------------------------------------

const CAP_REV_ID: u64 = 0x01; // revision id (bits 0-7)
const CAP_NUM_TIM_SHIFT: u64 = 8; // bits 8-12: number of timers minus one
const CAP_COUNT_SIZE_CAP: u64 = 1 << 13; // main counter is 64-bit capable
const CAP_LEG_RT_CAP: u64 = 1 << 15; // legacy replacement routing capable
const CAP_VENDOR_SHIFT: u64 = 16; // bits 16-31: vendor id
const CAP_CLK_PERIOD_SHIFT: u64 = 32; // bits 32-63: clock period (fs)

// ---- General Configuration bits --------------------------------------------

const CFG_ENABLE_CNF: u64 = 1 << 0; // overall enable / main counter run
const CFG_LEG_RT_CNF: u64 = 1 << 1; // legacy replacement routing
const CFG_MASK: u64 = CFG_ENABLE_CNF | CFG_LEG_RT_CNF;

// ---- Per-timer Config/Capabilities bits ------------------------------------

const TN_INT_TYPE_CNF: u64 = 1 << 1; // 0=edge, 1=level
const TN_INT_ENB_CNF: u64 = 1 << 2; // interrupt enable
const TN_TYPE_CNF: u64 = 1 << 3; // 0=one-shot, 1=periodic
const TN_PER_INT_CAP: u64 = 1 << 4; // RO: periodic capable
const TN_SIZE_CAP: u64 = 1 << 5; // RO: 64-bit capable
const TN_VAL_SET_CNF: u64 = 1 << 6; // periodic accumulator load
const TN_32MODE_CNF: u64 = 1 << 8; // force 32-bit operation
const TN_INT_ROUTE_SHIFT: u64 = 9; // bits 9-13: IOAPIC routing
const TN_INT_ROUTE_MASK: u64 = 0x1F << TN_INT_ROUTE_SHIFT;
const TN_FSB_EN_CNF: u64 = 1 << 14; // FSB interrupt enable
const TN_FSB_INT_DEL_CAP: u64 = 1 << 15; // RO: FSB delivery capable
const TN_INT_ROUTE_CAP_SHIFT: u64 = 32; // bits 32-63: allowed routing bitmap

/// Writable mask for a per-timer configuration register. Capability and
/// reserved bits are read-only / preserved.
const TN_CFG_WRITE_MASK: u64 = TN_INT_TYPE_CNF
    | TN_INT_ENB_CNF
    | TN_TYPE_CNF
    | TN_VAL_SET_CNF
    | TN_32MODE_CNF
    | TN_INT_ROUTE_MASK
    | TN_FSB_EN_CNF;

/// Allowed IOAPIC input routing for every timer (GSIs 2, 8, 20..23 in the
/// classic chipset map). Exposed so the orchestrator can program valid GSIs.
const TN_INT_ROUTE_CAP: u64 = 0x00F0_0104;

/// State of a single comparator / event timer.
#[derive(Clone, Copy, Debug)]
struct Timer {
    /// Configuration and capability register (the capability bits are fixed).
    config: u64,
    /// Comparator value: the main-counter value at which the timer fires next.
    comparator: u64,
    /// Periodic interval (period mode). Loaded from comparator writes while
    /// `TN_VAL_SET_CNF` is set; used to re-arm the comparator.
    period: u64,
    /// FSB interrupt route register (value/address). Stored, not delivered.
    fsb_route: u64,
}

impl Timer {
    fn new() -> Self {
        // Each timer is periodic-capable, 64-bit capable, and FSB-capable.
        let config = TN_PER_INT_CAP | TN_SIZE_CAP | TN_FSB_INT_DEL_CAP;
        Timer {
            config,
            comparator: u64::MAX,
            period: 0,
            fsb_route: 0,
        }
    }

    fn is_periodic(&self) -> bool {
        self.config & TN_TYPE_CNF != 0
    }

    fn int_enabled(&self) -> bool {
        self.config & TN_INT_ENB_CNF != 0
    }

    fn level_triggered(&self) -> bool {
        self.config & TN_INT_TYPE_CNF != 0
    }
}

/// The HPET core state. The MMIO wrapper [`HpetDevice`] forwards register
/// accesses to this type so that the core can be unit-tested directly.
pub struct Hpet {
    /// General configuration register (ENABLE_CNF / LEG_RT_CNF).
    config: u64,
    /// General interrupt status register (per-timer level status, RW1C).
    int_status: u64,
    /// Comparators.
    timers: [Timer; NUM_TIMERS],
    /// Reference instant used to derive the main counter while enabled.
    epoch: Instant,
    /// Main counter value accumulated while the counter was last halted, plus
    /// any value written by the guest. The live counter is this offset plus
    /// the elapsed ticks since the counter was (re)started.
    counter_offset: u64,
    /// Whether the main counter is currently running (mirrors ENABLE_CNF, but
    /// recorded so enable/disable transitions latch the counter correctly).
    running: bool,
    /// Last computed counter value, used to detect comparator matches in
    /// [`Hpet::tick`] without missing a crossing.
    last_counter: u64,
}

impl Default for Hpet {
    fn default() -> Self {
        Self::new()
    }
}

impl Hpet {
    pub fn new() -> Self {
        Hpet {
            config: 0,
            int_status: 0,
            timers: [Timer::new(); NUM_TIMERS],
            epoch: Instant::now(),
            counter_offset: 0,
            running: false,
            last_counter: 0,
        }
    }

    /// Convert elapsed host nanoseconds into HPET ticks using the configured
    /// femtosecond period. Uses 128-bit math to avoid overflow.
    fn nanos_to_ticks(nanos: u64) -> u64 {
        // ticks = nanos * (FS_PER_NS / period_fs)
        // period is in fs/tick, so ticks = elapsed_fs / period_fs.
        let elapsed_fs = (nanos as u128) * (FS_PER_NS as u128);
        (elapsed_fs / COUNTER_CLK_PERIOD_FS as u128) as u64
    }

    /// Ticks elapsed since the epoch (only meaningful while running).
    fn elapsed_ticks(&self) -> u64 {
        let nanos = self.epoch.elapsed().as_nanos() as u64;
        Self::nanos_to_ticks(nanos)
    }

    /// Current value of the main counter.
    pub fn main_counter(&self) -> u64 {
        if self.running {
            self.counter_offset.wrapping_add(self.elapsed_ticks())
        } else {
            self.counter_offset
        }
    }

    /// Build the read-only General Capabilities and ID register value.
    fn capabilities(&self) -> u64 {
        CAP_REV_ID
            | (((NUM_TIMERS as u64) - 1) << CAP_NUM_TIM_SHIFT)
            | CAP_COUNT_SIZE_CAP
            | CAP_LEG_RT_CAP
            | (VENDOR_ID << CAP_VENDOR_SHIFT)
            | (COUNTER_CLK_PERIOD_FS << CAP_CLK_PERIOD_SHIFT)
    }

    /// Returns true when the HPET is globally enabled (ENABLE_CNF set).
    pub fn is_enabled(&self) -> bool {
        self.config & CFG_ENABLE_CNF != 0
    }

    /// Returns true when legacy replacement routing is enabled.
    ///
    /// When set, timer 0 is routed to IRQ0 (PIT) / IRQ2 and timer 1 to IRQ8
    /// (RTC) instead of through the IOAPIC, per the spec.
    pub fn legacy_routing(&self) -> bool {
        self.config & CFG_LEG_RT_CNF != 0
    }

    /// Latch / start the main counter to reflect a new running state.
    fn set_running(&mut self, run: bool) {
        if run == self.running {
            return;
        }
        if run {
            // Re-anchor the epoch so elapsed ticks accumulate from `counter_offset`.
            self.epoch = Instant::now();
            self.running = true;
        } else {
            // Latch the current value into the offset and stop accumulating.
            self.counter_offset = self.main_counter();
            self.running = false;
        }
        self.last_counter = self.main_counter();
    }

    /// Read a 64-bit register value at an aligned offset.
    fn read_reg(&self, offset: u64) -> u64 {
        match offset {
            REG_GEN_CAP_ID => self.capabilities(),
            REG_GEN_CONFIG => self.config,
            REG_GEN_INT_STATUS => self.int_status,
            REG_MAIN_COUNTER => self.main_counter(),
            o if o >= REG_TIMER_BASE => {
                let idx = ((o - REG_TIMER_BASE) / TIMER_STRIDE) as usize;
                let sub = (o - REG_TIMER_BASE) % TIMER_STRIDE;
                if idx >= NUM_TIMERS {
                    return 0;
                }
                let timer = &self.timers[idx];
                match sub {
                    0x00 => timer.config | (TN_INT_ROUTE_CAP << TN_INT_ROUTE_CAP_SHIFT),
                    0x08 => timer.comparator,
                    0x10 => timer.fsb_route,
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    /// Write a 64-bit register value at an aligned offset.
    fn write_reg(&mut self, offset: u64, value: u64) {
        match offset {
            REG_GEN_CAP_ID => { /* read-only */ }
            REG_GEN_CONFIG => {
                let new = value & CFG_MASK;
                self.config = new;
                self.set_running(new & CFG_ENABLE_CNF != 0);
            }
            REG_GEN_INT_STATUS => {
                // Write-1-to-clear: clear every status bit set in `value`.
                self.int_status &= !value;
            }
            REG_MAIN_COUNTER => {
                // Per spec, the main counter should only be written while halted.
                // We honor the write regardless, re-anchoring the epoch so the
                // new base takes effect immediately.
                self.counter_offset = value;
                self.last_counter = value;
                if self.running {
                    self.epoch = Instant::now();
                }
            }
            o if o >= REG_TIMER_BASE => {
                let idx = ((o - REG_TIMER_BASE) / TIMER_STRIDE) as usize;
                let sub = (o - REG_TIMER_BASE) % TIMER_STRIDE;
                if idx >= NUM_TIMERS {
                    return;
                }
                match sub {
                    0x00 => {
                        let timer = &mut self.timers[idx];
                        // Preserve capability/reserved bits; apply writable bits.
                        timer.config =
                            (timer.config & !TN_CFG_WRITE_MASK) | (value & TN_CFG_WRITE_MASK);
                    }
                    0x08 => {
                        let timer = &mut self.timers[idx];
                        if timer.is_periodic() && timer.config & TN_VAL_SET_CNF != 0 {
                            // Periodic accumulator load: set the interval and the
                            // first comparator value, then clear VAL_SET.
                            timer.period = value;
                            timer.comparator = value;
                            timer.config &= !TN_VAL_SET_CNF;
                        } else if timer.is_periodic() {
                            // Writing the comparator in periodic mode (without
                            // VAL_SET) also updates the interval per the spec.
                            timer.period = value;
                            timer.comparator = value;
                        } else {
                            timer.comparator = value;
                        }
                    }
                    0x10 => {
                        self.timers[idx].fsb_route = value;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Advance the timer state to the current host time, raising interrupt
    /// status bits for any comparators whose match the counter has crossed.
    ///
    /// Returns true if any timer interrupt became (or remained) pending.
    /// Should be polled periodically by the orchestrator.
    pub fn tick(&mut self) -> bool {
        if !self.running {
            return false;
        }

        let now = self.main_counter();
        let prev = self.last_counter;
        self.last_counter = now;

        for (idx, timer) in self.timers.iter_mut().enumerate() {
            if !timer.int_enabled() {
                continue;
            }

            // Detect a comparator crossing in [prev, now]. The counter is
            // monotonic between ticks (no wrap expected within a tick window),
            // so a simple range check suffices.
            let fired = crossed(prev, now, timer.comparator);
            if fired {
                self.int_status |= 1 << idx;

                if timer.is_periodic() && timer.period != 0 {
                    // Re-arm: advance the comparator by whole periods until it
                    // is strictly ahead of the current counter.
                    let mut next = timer.comparator;
                    loop {
                        next = next.wrapping_add(timer.period);
                        if crossed(prev, now, next) {
                            // Multiple periods elapsed in one tick window; keep
                            // advancing but we only latch a single pending bit.
                            continue;
                        }
                        break;
                    }
                    timer.comparator = next;
                }
            }
        }

        self.int_status != 0
    }

    /// True if any per-timer interrupt status bit is set.
    pub fn has_pending_interrupt(&self) -> bool {
        self.int_status != 0
    }

    /// Bitmask of timers (bit `n` => timer `n`) with a pending interrupt.
    pub fn pending_timers(&self) -> u32 {
        self.int_status as u32
    }

    /// Clear a timer's pending status (equivalent to a guest RW1C write).
    /// Useful for orchestrators that ack edge-triggered interrupts directly.
    pub fn clear_interrupt(&mut self, timer: usize) {
        if timer < NUM_TIMERS {
            self.int_status &= !(1 << timer);
        }
    }
}

/// Returns true if `target` lies in the half-open crossing window such that a
/// counter advancing from `prev` to `now` reaches or passes it. We treat the
/// window as inclusive of `now` and exclusive of `prev` so each match fires
/// exactly once across successive ticks.
fn crossed(prev: u64, now: u64, target: u64) -> bool {
    if now >= prev {
        target > prev && target <= now
    } else {
        // Counter wrapped (extremely unlikely for a 64-bit counter): the valid
        // region is (prev, MAX] U [0, now].
        target > prev || target <= now
    }
}

/// MMIO wrapper that adapts [`Hpet`] to the [`MmioDevice`] trait. Handles
/// little-endian assembly/disassembly for 1/2/4/8-byte (and partial) accesses.
pub struct HpetDevice {
    hpet: Hpet,
}

impl HpetDevice {
    pub fn new() -> Self {
        HpetDevice { hpet: Hpet::new() }
    }

    /// Access the underlying core (for polling / orchestrator wiring).
    pub fn core(&self) -> &Hpet {
        &self.hpet
    }

    /// Mutable access to the underlying core.
    pub fn core_mut(&mut self) -> &mut Hpet {
        &mut self.hpet
    }

    /// Convenience: advance timers and report pending interrupt state.
    pub fn tick(&mut self) -> bool {
        self.hpet.tick()
    }

    /// Convenience: are any timer interrupts pending?
    pub fn has_pending_interrupt(&self) -> bool {
        self.hpet.has_pending_interrupt()
    }

    /// Convenience: bitmask of timers with pending interrupts.
    pub fn pending_timers(&self) -> u32 {
        self.hpet.pending_timers()
    }
}

impl Default for HpetDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl MmioDevice for HpetDevice {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        let offset = addr.wrapping_sub(HPET_BASE);
        // All HPET registers are 64-bit; compute the aligned register offset
        // and the byte position of this access within it.
        let aligned = offset & !0x7;
        let byte_in_reg = (offset & 0x7) as usize;
        let value = self.hpet.read_reg(aligned);
        let bytes = value.to_le_bytes();

        for (i, out) in data.iter_mut().enumerate() {
            let pos = byte_in_reg + i;
            *out = if pos < 8 { bytes[pos] } else { 0 };
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        let offset = addr.wrapping_sub(HPET_BASE);
        let aligned = offset & !0x7;
        let byte_in_reg = (offset & 0x7) as usize;

        // Read-modify-write so partial / sub-register accesses preserve the
        // bytes outside the written range (e.g. a 32-bit write to the low half
        // of a 64-bit register must not clobber the high half).
        let mut bytes = self.hpet.read_reg(aligned).to_le_bytes();
        for (i, byte) in data.iter().enumerate() {
            let pos = byte_in_reg + i;
            if pos < 8 {
                bytes[pos] = *byte;
            }
        }
        self.hpet.write_reg(aligned, u64::from_le_bytes(bytes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    fn dev() -> HpetDevice {
        HpetDevice::new()
    }

    /// Helper: write `value` to a 64-bit register via the MMIO interface.
    fn write64(d: &mut HpetDevice, off: u64, value: u64) {
        d.write(HPET_BASE + off, &value.to_le_bytes());
    }

    /// Helper: read a 64-bit register via the MMIO interface.
    fn read64(d: &mut HpetDevice, off: u64) -> u64 {
        let mut buf = [0u8; 8];
        d.read(HPET_BASE + off, &mut buf);
        u64::from_le_bytes(buf)
    }

    // ---- Capabilities / ID --------------------------------------------------

    #[test]
    fn test_capabilities_layout() {
        let mut d = dev();
        let cap = read64(&mut d, REG_GEN_CAP_ID);

        // Period in upper 32 bits.
        assert_eq!(cap >> 32, COUNTER_CLK_PERIOD_FS);
        // Vendor id bits 16-31.
        assert_eq!((cap >> 16) & 0xFFFF, VENDOR_ID);
        // 64-bit counter capable.
        assert_ne!(cap & CAP_COUNT_SIZE_CAP, 0);
        // Legacy replacement capable.
        assert_ne!(cap & CAP_LEG_RT_CAP, 0);
        // NUM_TIM_CAP = N - 1.
        assert_eq!((cap >> CAP_NUM_TIM_SHIFT) & 0x1F, (NUM_TIMERS as u64) - 1);
        // Revision id non-zero.
        assert_eq!(cap & 0xFF, CAP_REV_ID);
    }

    #[test]
    fn test_capabilities_readonly() {
        let mut d = dev();
        let before = read64(&mut d, REG_GEN_CAP_ID);
        write64(&mut d, REG_GEN_CAP_ID, 0xDEAD_BEEF_DEAD_BEEF);
        assert_eq!(read64(&mut d, REG_GEN_CAP_ID), before);
    }

    // ---- Enable / counter ---------------------------------------------------

    #[test]
    fn test_counter_halted_by_default() {
        let mut d = dev();
        assert!(!d.core().is_enabled());
        let a = read64(&mut d, REG_MAIN_COUNTER);
        sleep(Duration::from_millis(2));
        let b = read64(&mut d, REG_MAIN_COUNTER);
        assert_eq!(a, b, "counter must not advance while halted");
    }

    #[test]
    fn test_enable_starts_counter_and_is_monotonic() {
        let mut d = dev();
        write64(&mut d, REG_GEN_CONFIG, CFG_ENABLE_CNF);
        assert!(d.core().is_enabled());

        let mut prev = read64(&mut d, REG_MAIN_COUNTER);
        for _ in 0..5 {
            sleep(Duration::from_millis(2));
            let now = read64(&mut d, REG_MAIN_COUNTER);
            assert!(now >= prev, "counter must be monotonic: {now} < {prev}");
            prev = now;
        }
        // Over ~10ms at 10MHz we expect well over 10k ticks of progress.
        assert!(
            prev > 1000,
            "counter should advance meaningfully, got {prev}"
        );
    }

    #[test]
    fn test_counter_write_while_halted() {
        let mut d = dev();
        write64(&mut d, REG_MAIN_COUNTER, 0x1234_5678);
        assert_eq!(read64(&mut d, REG_MAIN_COUNTER), 0x1234_5678);
    }

    #[test]
    fn test_disable_latches_counter() {
        let mut d = dev();
        write64(&mut d, REG_GEN_CONFIG, CFG_ENABLE_CNF);
        sleep(Duration::from_millis(2));
        write64(&mut d, REG_GEN_CONFIG, 0); // disable
        let a = read64(&mut d, REG_MAIN_COUNTER);
        sleep(Duration::from_millis(2));
        let b = read64(&mut d, REG_MAIN_COUNTER);
        assert_eq!(a, b, "counter must freeze when disabled");
    }

    // ---- General configuration ---------------------------------------------

    #[test]
    fn test_config_legacy_routing_bit() {
        let mut d = dev();
        write64(&mut d, REG_GEN_CONFIG, CFG_LEG_RT_CNF);
        assert!(d.core().legacy_routing());
        assert_eq!(read64(&mut d, REG_GEN_CONFIG) & CFG_MASK, CFG_LEG_RT_CNF);
    }

    #[test]
    fn test_config_reserved_bits_ignored() {
        let mut d = dev();
        write64(&mut d, REG_GEN_CONFIG, 0xFFFF_FFFF_FFFF_FFFF);
        // Only ENABLE_CNF and LEG_RT_CNF should stick.
        assert_eq!(read64(&mut d, REG_GEN_CONFIG), CFG_MASK);
    }

    // ---- One-shot comparator + RW1C status ----------------------------------

    #[test]
    fn test_oneshot_fires_and_status_rw1c_clears() {
        let mut d = dev();
        // Enable timer 0 interrupt, one-shot (TYPE_CNF clear).
        write64(&mut d, REG_TIMER_BASE, TN_INT_ENB_CNF);
        // Comparator just ahead of current counter.
        let target = d.core().main_counter() + 50; // 50 ticks = 5us at 10MHz
        write64(&mut d, REG_TIMER_BASE + 0x08, target);

        // Start the counter.
        write64(&mut d, REG_GEN_CONFIG, CFG_ENABLE_CNF);

        // Wait past the comparator, then tick.
        sleep(Duration::from_millis(2));
        assert!(d.tick());
        assert!(d.has_pending_interrupt());
        assert_eq!(d.pending_timers() & 1, 1);

        // Status reflects timer 0.
        assert_eq!(read64(&mut d, REG_GEN_INT_STATUS) & 1, 1);

        // RW1C: writing 1 clears the bit; writing 0 to other bits is a no-op.
        write64(&mut d, REG_GEN_INT_STATUS, 1);
        assert_eq!(read64(&mut d, REG_GEN_INT_STATUS) & 1, 0);
        assert!(!d.has_pending_interrupt());
    }

    #[test]
    fn test_disabled_interrupt_does_not_fire() {
        let mut d = dev();
        // Timer 0 with INT_ENB clear.
        let target = d.core().main_counter() + 50;
        write64(&mut d, REG_TIMER_BASE + 0x08, target);
        write64(&mut d, REG_GEN_CONFIG, CFG_ENABLE_CNF);
        sleep(Duration::from_millis(2));
        assert!(!d.tick());
        assert!(!d.has_pending_interrupt());
    }

    // ---- Periodic re-arm ----------------------------------------------------

    #[test]
    fn test_periodic_rearm() {
        let mut d = dev();
        // Enable timer 1, periodic, level-triggered.
        let cfg_off = REG_TIMER_BASE + TIMER_STRIDE;
        write64(
            &mut d,
            cfg_off,
            TN_INT_ENB_CNF | TN_TYPE_CNF | TN_INT_TYPE_CNF | TN_VAL_SET_CNF,
        );
        // Load period/comparator (small period so it re-arms quickly).
        write64(&mut d, cfg_off + 0x08, 100);
        // VAL_SET should have been consumed.
        assert_eq!(read64(&mut d, cfg_off) & TN_VAL_SET_CNF, 0);

        write64(&mut d, REG_GEN_CONFIG, CFG_ENABLE_CNF);

        sleep(Duration::from_millis(2));
        assert!(d.tick());
        assert_eq!(d.pending_timers() & 0b10, 0b10);

        // Comparator must have advanced past the current counter (re-armed).
        let comp = read64(&mut d, cfg_off + 0x08);
        assert!(comp > d.core().main_counter() - 100 * 2);
        assert!(comp > 100, "comparator should have advanced by >=1 period");

        // Clear status, advance again, and confirm it fires once more.
        write64(&mut d, REG_GEN_INT_STATUS, 0b10);
        assert_eq!(read64(&mut d, REG_GEN_INT_STATUS) & 0b10, 0);
        sleep(Duration::from_millis(2));
        assert!(d.tick());
        assert_eq!(d.pending_timers() & 0b10, 0b10);
    }

    // ---- Timer config capability bits ---------------------------------------

    #[test]
    fn test_timer_config_capability_bits_preserved() {
        let mut d = dev();
        // Read default config: capability bits set, route cap in upper bits.
        let cfg = read64(&mut d, REG_TIMER_BASE);
        assert_ne!(cfg & TN_PER_INT_CAP, 0);
        assert_ne!(cfg & TN_SIZE_CAP, 0);
        assert_ne!(cfg & TN_FSB_INT_DEL_CAP, 0);
        assert_eq!(cfg >> TN_INT_ROUTE_CAP_SHIFT, TN_INT_ROUTE_CAP);

        // Attempt to clear capability bits via write; they must remain.
        write64(&mut d, REG_TIMER_BASE, 0);
        let cfg2 = read64(&mut d, REG_TIMER_BASE);
        assert_ne!(cfg2 & TN_PER_INT_CAP, 0);
        assert_ne!(cfg2 & TN_SIZE_CAP, 0);

        // Writable bits do take effect.
        write64(&mut d, REG_TIMER_BASE, TN_INT_ENB_CNF | TN_TYPE_CNF);
        let cfg3 = read64(&mut d, REG_TIMER_BASE);
        assert_ne!(cfg3 & TN_INT_ENB_CNF, 0);
        assert_ne!(cfg3 & TN_TYPE_CNF, 0);
    }

    // ---- 32-bit / partial access splitting ----------------------------------

    #[test]
    fn test_32bit_access_split_of_capabilities() {
        let mut d = dev();
        let full = read64(&mut d, REG_GEN_CAP_ID);

        // Low 32 bits.
        let mut lo = [0u8; 4];
        d.read(HPET_BASE + REG_GEN_CAP_ID, &mut lo);
        assert_eq!(u32::from_le_bytes(lo) as u64, full & 0xFFFF_FFFF);

        // High 32 bits (offset +4) must be the clock period.
        let mut hi = [0u8; 4];
        d.read(HPET_BASE + REG_GEN_CAP_ID + 4, &mut hi);
        assert_eq!(u32::from_le_bytes(hi) as u64, full >> 32);
        assert_eq!(u32::from_le_bytes(hi) as u64, COUNTER_CLK_PERIOD_FS);
    }

    #[test]
    fn test_32bit_write_preserves_other_half() {
        let mut d = dev();
        // Seed a known 64-bit comparator.
        write64(&mut d, REG_TIMER_BASE + 0x08, 0xAAAA_AAAA_BBBB_BBBB);

        // 32-bit write to the low half only.
        d.write(
            HPET_BASE + REG_TIMER_BASE + 0x08,
            &0x1234_5678u32.to_le_bytes(),
        );
        let v = read64(&mut d, REG_TIMER_BASE + 0x08);
        assert_eq!(v & 0xFFFF_FFFF, 0x1234_5678);
        assert_eq!(v >> 32, 0xAAAA_AAAA, "high half must be preserved");

        // 32-bit write to the high half only.
        d.write(
            HPET_BASE + REG_TIMER_BASE + 0x08 + 4,
            &0xCAFE_BABEu32.to_le_bytes(),
        );
        let v2 = read64(&mut d, REG_TIMER_BASE + 0x08);
        assert_eq!(v2 >> 32, 0xCAFE_BABE);
        assert_eq!(v2 & 0xFFFF_FFFF, 0x1234_5678, "low half must be preserved");
    }

    #[test]
    fn test_byte_access_reassembles() {
        let mut d = dev();
        write64(&mut d, REG_MAIN_COUNTER, 0x0011_2233_4455_6677);
        // Read byte by byte (simulating the MmioBus byte-wise dispatch).
        let mut assembled = [0u8; 8];
        for (i, b) in assembled.iter_mut().enumerate() {
            let mut one = [0u8; 1];
            d.read(HPET_BASE + REG_MAIN_COUNTER + i as u64, &mut one);
            *b = one[0];
        }
        assert_eq!(u64::from_le_bytes(assembled), 0x0011_2233_4455_6677);
    }

    #[test]
    fn test_byte_wise_write_reassembles() {
        let mut d = dev();
        let value: u64 = 0x8899_AABB_CCDD_EEFF;
        for (i, b) in value.to_le_bytes().iter().enumerate() {
            d.write(HPET_BASE + REG_MAIN_COUNTER + i as u64, &[*b]);
        }
        assert_eq!(read64(&mut d, REG_MAIN_COUNTER), value);
    }

    // ---- Independent timers -------------------------------------------------

    #[test]
    fn test_timers_independent() {
        let mut d = dev();
        for n in 0..NUM_TIMERS as u64 {
            let off = REG_TIMER_BASE + n * TIMER_STRIDE;
            write64(&mut d, off + 0x08, 0x1000 + n);
        }
        for n in 0..NUM_TIMERS as u64 {
            let off = REG_TIMER_BASE + n * TIMER_STRIDE;
            assert_eq!(read64(&mut d, off + 0x08), 0x1000 + n);
        }
    }

    #[test]
    fn test_fsb_route_register_stored() {
        let mut d = dev();
        write64(&mut d, REG_TIMER_BASE + 0x10, 0xDEAD_0000_BEEF_0000);
        assert_eq!(read64(&mut d, REG_TIMER_BASE + 0x10), 0xDEAD_0000_BEEF_0000);
    }

    // ---- crossing helper ----------------------------------------------------

    #[test]
    fn test_crossed_helper() {
        assert!(crossed(10, 20, 15));
        assert!(crossed(10, 20, 20)); // inclusive of now
        assert!(!crossed(10, 20, 10)); // exclusive of prev
        assert!(!crossed(10, 20, 25));
        // wrap case
        assert!(crossed(u64::MAX - 5, 3, 0));
        assert!(crossed(u64::MAX - 5, 3, u64::MAX));
        assert!(!crossed(u64::MAX - 5, 3, 10));
    }
}
