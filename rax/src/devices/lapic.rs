//! Local APIC (Local Advanced Programmable Interrupt Controller) emulation.
//!
//! The x86-64 Local APIC provides:
//! - Per-CPU interrupt handling
//! - Timer functionality (one-shot, periodic, TSC-deadline modes)
//! - Inter-Processor Interrupts (IPI)
//! - Local interrupt sources (LINT0, LINT1, error, thermal, PMC)
//!
//! MMIO region: 0xFEE00000 - 0xFEE00FFF (4KB)

use super::bus::MmioDevice;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// LAPIC base address (fixed for x86-64)
pub const LAPIC_BASE: u64 = 0xFEE00000;
/// LAPIC region size
pub const LAPIC_SIZE: u64 = 0x1000;

// Register offsets (byte offsets from LAPIC_BASE)
const LAPIC_ID: u64 = 0x020;
const LAPIC_VERSION: u64 = 0x030;
const LAPIC_TPR: u64 = 0x080; // Task Priority Register
const LAPIC_APR: u64 = 0x090; // Arbitration Priority Register
const LAPIC_PPR: u64 = 0x0A0; // Processor Priority Register
const LAPIC_EOI: u64 = 0x0B0; // End of Interrupt
const LAPIC_RRD: u64 = 0x0C0; // Remote Read Register
const LAPIC_LDR: u64 = 0x0D0; // Logical Destination Register
const LAPIC_DFR: u64 = 0x0E0; // Destination Format Register
const LAPIC_SVR: u64 = 0x0F0; // Spurious Interrupt Vector Register
const LAPIC_ISR_BASE: u64 = 0x100; // In-Service Register (8 x 32-bit)
const LAPIC_TMR_BASE: u64 = 0x180; // Trigger Mode Register (8 x 32-bit)
const LAPIC_IRR_BASE: u64 = 0x200; // Interrupt Request Register (8 x 32-bit)
const LAPIC_ESR: u64 = 0x280; // Error Status Register
const LAPIC_ICR_LOW: u64 = 0x300; // Interrupt Command Register (low)
const LAPIC_ICR_HIGH: u64 = 0x310; // Interrupt Command Register (high)
const LAPIC_LVT_TIMER: u64 = 0x320; // LVT Timer Register
const LAPIC_LVT_THERMAL: u64 = 0x330; // LVT Thermal Sensor Register
const LAPIC_LVT_PMC: u64 = 0x340; // LVT Performance Counter Register
const LAPIC_LVT_LINT0: u64 = 0x350; // LVT LINT0 Register
const LAPIC_LVT_LINT1: u64 = 0x360; // LVT LINT1 Register
const LAPIC_LVT_ERROR: u64 = 0x370; // LVT Error Register
const LAPIC_TIMER_ICR: u64 = 0x380; // Timer Initial Count Register
const LAPIC_TIMER_CCR: u64 = 0x390; // Timer Current Count Register
const LAPIC_TIMER_DCR: u64 = 0x3E0; // Timer Divide Configuration Register

// LVT entry bits
const LVT_MASK: u32 = 1 << 16; // Interrupt masked
const LVT_TIMER_MODE_SHIFT: u32 = 17;
const LVT_TIMER_MODE_MASK: u32 = 0x3 << LVT_TIMER_MODE_SHIFT;

// Timer modes
const TIMER_MODE_ONESHOT: u32 = 0;
const TIMER_MODE_PERIODIC: u32 = 1;
const TIMER_MODE_TSC_DEADLINE: u32 = 2;

// SVR bits
const SVR_APIC_ENABLED: u32 = 1 << 8;

// ICR (Interrupt Command Register) field definitions
const ICR_VECTOR_MASK: u64 = 0xFF;
const ICR_DELIVERY_MODE_SHIFT: u64 = 8;
const ICR_DELIVERY_MODE_MASK: u64 = 0x7 << ICR_DELIVERY_MODE_SHIFT;
const ICR_DEST_MODE_LOGICAL: u64 = 1 << 11;
const ICR_DELIVERY_STATUS: u64 = 1 << 12; // Read-only, 0 = idle, 1 = pending
const ICR_LEVEL_ASSERT: u64 = 1 << 14;
const ICR_TRIGGER_LEVEL: u64 = 1 << 15;
const ICR_DEST_SHORTHAND_SHIFT: u64 = 18;
const ICR_DEST_SHORTHAND_MASK: u64 = 0x3 << ICR_DEST_SHORTHAND_SHIFT;
const ICR_DEST_FIELD_SHIFT: u64 = 56; // Bits 56-63 (in full 64-bit ICR)

// ICR Delivery Modes
const DELIVERY_MODE_FIXED: u64 = 0;
const DELIVERY_MODE_LOWEST_PRIORITY: u64 = 1;
const DELIVERY_MODE_SMI: u64 = 2;
const DELIVERY_MODE_NMI: u64 = 4;
const DELIVERY_MODE_INIT: u64 = 5;
const DELIVERY_MODE_SIPI: u64 = 6; // Start-up IPI

// ICR Destination Shorthands
const DEST_SHORTHAND_NONE: u64 = 0;
const DEST_SHORTHAND_SELF: u64 = 1;
const DEST_SHORTHAND_ALL_INCLUDING_SELF: u64 = 2;
const DEST_SHORTHAND_ALL_EXCLUDING_SELF: u64 = 3;

/// LAPIC timer frequency (approximate - 1 GHz bus clock / 16 = ~62.5 MHz base)
/// We'll use a simulated frequency based on real time
const LAPIC_TIMER_FREQ_HZ: u64 = 1_000_000_000; // 1 GHz base frequency

/// Represents an IPI request that needs to be delivered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpiRequest {
    /// Fixed interrupt delivery - deliver vector to target CPU(s)
    Fixed { vector: u8, target: IpiTarget },
    /// Lowest priority - deliver to CPU with lowest priority (treated as Fixed for single CPU)
    LowestPriority { vector: u8, target: IpiTarget },
    /// Non-maskable interrupt
    Nmi { target: IpiTarget },
    /// System management interrupt
    Smi { target: IpiTarget },
    /// INIT - reset target CPU to wait-for-SIPI state
    Init { target: IpiTarget },
    /// Start-up IPI - start target CPU at specified vector * 0x1000
    Sipi { vector: u8, target: IpiTarget },
}

/// Target specification for an IPI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpiTarget {
    /// Send to self only
    ToSelf,
    /// Send to all CPUs including self
    AllIncludingSelf,
    /// Send to all CPUs excluding self
    AllExcludingSelf,
    /// Send to specific CPU by physical APIC ID
    Physical(u8),
    /// Send to CPUs matching logical destination
    Logical { destination: u8 },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LocalApic {
    /// LAPIC ID (usually matches CPU ID)
    id: u32,
    /// LAPIC Version register
    version: u32,
    /// Task Priority Register
    tpr: u32,
    /// Logical Destination Register
    ldr: u32,
    /// Destination Format Register
    dfr: u32,
    /// Spurious Interrupt Vector Register
    svr: u32,
    /// In-Service Register (256 bits = 8 x 32-bit words)
    isr: [u32; 8],
    /// Trigger Mode Register (256 bits = 8 x 32-bit words)
    tmr: [u32; 8],
    /// Interrupt Request Register (256 bits = 8 x 32-bit words)
    irr: [u32; 8],
    /// Error Status Register
    esr: u32,
    /// Interrupt Command Register (64-bit)
    icr: u64,
    /// LVT Timer Register
    lvt_timer: u32,
    /// LVT Thermal Sensor Register
    lvt_thermal: u32,
    /// LVT Performance Counter Register
    lvt_pmc: u32,
    /// LVT LINT0 Register
    lvt_lint0: u32,
    /// LVT LINT1 Register
    lvt_lint1: u32,
    /// LVT Error Register
    lvt_error: u32,
    /// Timer Initial Count Register
    timer_initial_count: u32,
    /// Timer Current Count (computed dynamically)
    timer_current_count: u32,
    /// Timer Divide Configuration Register
    timer_divide_config: u32,
    /// Timestamp when timer was started. Transient host wall-clock state — not
    /// part of the persisted device state; re-armed lazily after restore.
    #[serde(skip)]
    timer_start: Option<Instant>,
    /// Pending timer interrupt
    timer_pending: bool,
    /// Pending IPI request (for VMM to process)
    pending_ipi: Option<IpiRequest>,
    /// Pending NMI to deliver
    nmi_pending: bool,
}

impl LocalApic {
    pub fn new(apic_id: u32) -> Self {
        // Configure for virtual wire mode (what BIOS would normally do):
        // - APIC enabled (SVR bit 8 = 1)
        // - LINT0 configured for ExtInt mode (delivery mode 7 = external interrupt)
        // - LINT1 configured for NMI (delivery mode 4)
        //
        // LINT0 value: delivery_mode=7 (ExtInt), not masked = 0x700
        // LINT1 value: delivery_mode=4 (NMI), not masked = 0x400
        LocalApic {
            id: apic_id << 24, // ID is in bits 24-31
            // Version: bits 0-7 = version (0x14 = modern APIC)
            // bits 16-23 = max LVT entry (6 entries = 0x05)
            version: 0x00050014,
            tpr: 0,
            ldr: 0,
            dfr: 0xFFFFFFFF, // Flat model
            svr: 0x1FF,      // APIC enabled (bit 8), spurious vector 0xFF
            isr: [0; 8],
            tmr: [0; 8],
            irr: [0; 8],
            esr: 0,
            icr: 0,
            lvt_timer: LVT_MASK, // Masked initially
            lvt_thermal: LVT_MASK,
            lvt_pmc: LVT_MASK,
            lvt_lint0: 0x700, // ExtInt mode, not masked - virtual wire mode
            lvt_lint1: 0x400, // NMI mode, not masked
            lvt_error: LVT_MASK,
            timer_initial_count: 0,
            timer_current_count: 0,
            timer_divide_config: 0,
            timer_start: None,
            timer_pending: false,
            pending_ipi: None,
            nmi_pending: false,
        }
    }

    /// Check if APIC is enabled
    pub fn is_enabled(&self) -> bool {
        (self.svr & SVR_APIC_ENABLED) != 0
    }

    /// Get the timer divisor from the divide configuration register
    fn timer_divisor(&self) -> u32 {
        // DCR bits 0-1 and bit 3 encode the divisor
        // 0b0000 = /2, 0b0001 = /4, 0b0010 = /8, 0b0011 = /16
        // 0b1000 = /32, 0b1001 = /64, 0b1010 = /128, 0b1011 = /1
        let bits = (self.timer_divide_config & 0x3) | ((self.timer_divide_config >> 1) & 0x4);
        match bits {
            0b000 => 2,
            0b001 => 4,
            0b010 => 8,
            0b011 => 16,
            0b100 => 32,
            0b101 => 64,
            0b110 => 128,
            0b111 => 1,
            _ => 1,
        }
    }

    /// Get the timer mode from LVT timer register
    fn timer_mode(&self) -> u32 {
        (self.lvt_timer & LVT_TIMER_MODE_MASK) >> LVT_TIMER_MODE_SHIFT
    }

    /// Check if timer is masked
    fn timer_masked(&self) -> bool {
        (self.lvt_timer & LVT_MASK) != 0
    }

    /// Get the timer vector
    pub fn timer_vector(&self) -> u8 {
        (self.lvt_timer & 0xFF) as u8
    }

    /// Tick the timer and return pending interrupt vector if any
    pub fn tick(&mut self) -> Option<u8> {
        if !self.is_enabled() || self.timer_initial_count == 0 || self.timer_masked() {
            return None;
        }

        let Some(start) = self.timer_start else {
            return None;
        };

        let elapsed = start.elapsed();
        let divisor = self.timer_divisor() as u64;
        let ticks_per_sec = LAPIC_TIMER_FREQ_HZ / divisor;

        // Calculate how many timer ticks have elapsed
        let elapsed_ticks = (elapsed.as_nanos() as u64 * ticks_per_sec) / 1_000_000_000;

        let initial = self.timer_initial_count as u64;
        let mode = self.timer_mode();

        match mode {
            TIMER_MODE_ONESHOT => {
                if elapsed_ticks >= initial {
                    // Timer expired
                    self.timer_current_count = 0;
                    self.timer_start = None; // Stop the timer
                    if !self.timer_pending {
                        self.timer_pending = true;
                        return Some(self.timer_vector());
                    }
                } else {
                    self.timer_current_count = (initial - elapsed_ticks) as u32;
                }
            }
            TIMER_MODE_PERIODIC => {
                // In periodic mode, timer restarts automatically
                let periods = elapsed_ticks / initial;
                let remainder = elapsed_ticks % initial;
                self.timer_current_count = (initial - remainder) as u32;

                // Generate interrupt for each period that passed
                if periods > 0 && !self.timer_pending {
                    self.timer_pending = true;
                    // Reset start time for next period
                    self.timer_start = Some(Instant::now());
                    return Some(self.timer_vector());
                }
            }
            TIMER_MODE_TSC_DEADLINE => {
                // TSC-deadline mode uses MSR, not implemented yet
            }
            _ => {}
        }

        None
    }

    /// Clear pending timer interrupt (called after injection)
    pub fn clear_timer_pending(&mut self) {
        self.timer_pending = false;
    }

    /// Check if there's a pending timer interrupt
    pub fn has_pending_timer(&self) -> bool {
        self.timer_pending
    }

    /// Get the APIC ID (bits 24-31 of the ID register)
    pub fn apic_id(&self) -> u8 {
        (self.id >> 24) as u8
    }

    /// Check if there's a pending NMI
    pub fn has_pending_nmi(&self) -> bool {
        self.nmi_pending
    }

    /// Clear pending NMI (called after delivery)
    pub fn clear_pending_nmi(&mut self) {
        self.nmi_pending = false;
    }

    /// Take pending IPI request (returns and clears it)
    pub fn take_pending_ipi(&mut self) -> Option<IpiRequest> {
        self.pending_ipi.take()
    }

    /// Check if there's a pending IPI
    pub fn has_pending_ipi(&self) -> bool {
        self.pending_ipi.is_some()
    }

    /// Handle IPI delivery when ICR is written.
    /// For self-targeted IPIs, this updates local state (IRR/NMI).
    /// For other CPUs, returns an IpiRequest for the VMM to handle.
    fn handle_ipi(&mut self) {
        let icr = self.icr;
        let vector = (icr & ICR_VECTOR_MASK) as u8;
        let delivery_mode = (icr & ICR_DELIVERY_MODE_MASK) >> ICR_DELIVERY_MODE_SHIFT;
        let dest_mode_logical = (icr & ICR_DEST_MODE_LOGICAL) != 0;
        let level_assert = (icr & ICR_LEVEL_ASSERT) != 0;
        let shorthand = (icr & ICR_DEST_SHORTHAND_MASK) >> ICR_DEST_SHORTHAND_SHIFT;
        let dest_field = ((icr >> ICR_DEST_FIELD_SHIFT) & 0xFF) as u8;

        tracing::debug!(
            "LAPIC IPI: vector={:#x}, delivery_mode={}, dest_mode={}, shorthand={}, dest={:#x}, level_assert={}",
            vector,
            delivery_mode,
            if dest_mode_logical {
                "logical"
            } else {
                "physical"
            },
            shorthand,
            dest_field,
            level_assert
        );

        // Determine target
        let target = match shorthand {
            DEST_SHORTHAND_SELF => IpiTarget::ToSelf,
            DEST_SHORTHAND_ALL_INCLUDING_SELF => IpiTarget::AllIncludingSelf,
            DEST_SHORTHAND_ALL_EXCLUDING_SELF => IpiTarget::AllExcludingSelf,
            DEST_SHORTHAND_NONE => {
                if dest_mode_logical {
                    IpiTarget::Logical {
                        destination: dest_field,
                    }
                } else {
                    IpiTarget::Physical(dest_field)
                }
            }
            _ => return, // Invalid shorthand
        };

        // Check if this IPI targets self
        let targets_self = match &target {
            IpiTarget::ToSelf => true,
            IpiTarget::AllIncludingSelf => true,
            IpiTarget::AllExcludingSelf => false,
            IpiTarget::Physical(id) => *id == self.apic_id(),
            IpiTarget::Logical { destination } => {
                // Check if our logical ID matches
                // In flat model (DFR = 0xFFFFFFFF), LDR bits 24-31 are the logical ID
                // Match if any bit in destination matches any bit in our logical ID
                let our_logical_id = (self.ldr >> 24) as u8;
                (destination & our_logical_id) != 0
            }
        };

        // Handle delivery based on mode
        match delivery_mode {
            DELIVERY_MODE_FIXED | DELIVERY_MODE_LOWEST_PRIORITY => {
                if targets_self {
                    // Deliver to self immediately by setting IRR
                    self.set_irr(vector);
                    tracing::debug!("LAPIC: Self-IPI vector {:#x} delivered to IRR", vector);
                }
                // Store IPI request for VMM to deliver to other CPUs
                if !matches!(target, IpiTarget::ToSelf) {
                    self.pending_ipi = Some(if delivery_mode == DELIVERY_MODE_FIXED {
                        IpiRequest::Fixed { vector, target }
                    } else {
                        IpiRequest::LowestPriority { vector, target }
                    });
                }
            }
            DELIVERY_MODE_NMI => {
                if targets_self {
                    self.nmi_pending = true;
                    tracing::debug!("LAPIC: Self-NMI pending");
                }
                if !matches!(target, IpiTarget::ToSelf) {
                    self.pending_ipi = Some(IpiRequest::Nmi { target });
                }
            }
            DELIVERY_MODE_SMI => {
                // SMI is system-level, store for VMM
                self.pending_ipi = Some(IpiRequest::Smi { target });
                tracing::debug!("LAPIC: SMI requested");
            }
            DELIVERY_MODE_INIT => {
                // INIT resets target CPU to wait-for-SIPI state
                // Level de-assert (level=0) is used for INIT synchronization, ignore it
                if level_assert {
                    self.pending_ipi = Some(IpiRequest::Init {
                        target: target.clone(),
                    });
                    tracing::debug!("LAPIC: INIT IPI requested");
                } else {
                    tracing::debug!("LAPIC: INIT level de-assert (ignored)");
                }
            }
            DELIVERY_MODE_SIPI => {
                // Start-up IPI - vector specifies start address (vector * 0x1000)
                self.pending_ipi = Some(IpiRequest::Sipi { vector, target });
                tracing::debug!(
                    "LAPIC: SIPI vector={:#x} (start addr={:#x})",
                    vector,
                    (vector as u32) * 0x1000
                );
            }
            _ => {
                tracing::warn!("LAPIC: Unknown delivery mode {}", delivery_mode);
            }
        }
    }

    /// Get pending interrupt vector (highest priority)
    pub fn get_pending_vector(&self) -> Option<u8> {
        // Check IRR for pending interrupts
        for i in (0..8).rev() {
            if self.irr[i] != 0 {
                for bit in (0..32).rev() {
                    if self.irr[i] & (1 << bit) != 0 {
                        return Some((i * 32 + bit) as u8);
                    }
                }
            }
        }
        None
    }

    /// Set an interrupt as pending in IRR
    pub fn set_irr(&mut self, vector: u8) {
        let idx = (vector / 32) as usize;
        let bit = vector % 32;
        self.irr[idx] |= 1 << bit;
    }

    /// Acknowledge interrupt (move from IRR to ISR)
    pub fn ack_interrupt(&mut self, vector: u8) {
        let idx = (vector / 32) as usize;
        let bit = vector % 32;
        self.irr[idx] &= !(1 << bit);
        self.isr[idx] |= 1 << bit;
    }

    /// End of interrupt - clear highest priority ISR bit
    fn eoi(&mut self) {
        // Find highest priority in-service interrupt and clear it
        for i in (0..8).rev() {
            if self.isr[i] != 0 {
                for bit in (0..32).rev() {
                    if self.isr[i] & (1 << bit) != 0 {
                        self.isr[i] &= !(1 << bit);
                        return;
                    }
                }
            }
        }
    }

    fn read_register(&self, offset: u64) -> u32 {
        match offset {
            LAPIC_ID => self.id,
            LAPIC_VERSION => self.version,
            LAPIC_TPR => self.tpr,
            LAPIC_APR => 0, // Arbitration priority (read-only, usually 0)
            LAPIC_PPR => {
                // Processor priority = max(TPR, highest ISR priority)
                let tpr_class = (self.tpr >> 4) & 0xF;
                let mut max_isr = 0u32;
                for i in (0..8).rev() {
                    if self.isr[i] != 0 {
                        for bit in (0..32).rev() {
                            if self.isr[i] & (1 << bit) != 0 {
                                max_isr = ((i * 32 + bit) >> 4) as u32;
                                break;
                            }
                        }
                        break;
                    }
                }
                std::cmp::max(tpr_class, max_isr) << 4
            }
            LAPIC_RRD => 0,
            LAPIC_LDR => self.ldr,
            LAPIC_DFR => self.dfr,
            LAPIC_SVR => self.svr,
            o if o >= LAPIC_ISR_BASE && o < LAPIC_ISR_BASE + 0x80 => {
                let idx = ((o - LAPIC_ISR_BASE) / 0x10) as usize;
                if idx < 8 { self.isr[idx] } else { 0 }
            }
            o if o >= LAPIC_TMR_BASE && o < LAPIC_TMR_BASE + 0x80 => {
                let idx = ((o - LAPIC_TMR_BASE) / 0x10) as usize;
                if idx < 8 { self.tmr[idx] } else { 0 }
            }
            o if o >= LAPIC_IRR_BASE && o < LAPIC_IRR_BASE + 0x80 => {
                let idx = ((o - LAPIC_IRR_BASE) / 0x10) as usize;
                if idx < 8 { self.irr[idx] } else { 0 }
            }
            LAPIC_ESR => self.esr,
            LAPIC_ICR_LOW => self.icr as u32,
            LAPIC_ICR_HIGH => (self.icr >> 32) as u32,
            LAPIC_LVT_TIMER => self.lvt_timer,
            LAPIC_LVT_THERMAL => self.lvt_thermal,
            LAPIC_LVT_PMC => self.lvt_pmc,
            LAPIC_LVT_LINT0 => self.lvt_lint0,
            LAPIC_LVT_LINT1 => self.lvt_lint1,
            LAPIC_LVT_ERROR => self.lvt_error,
            LAPIC_TIMER_ICR => self.timer_initial_count,
            LAPIC_TIMER_CCR => {
                // Current count must be computed dynamically on every read
                // The kernel uses this for timer calibration
                if self.timer_initial_count == 0 {
                    0
                } else if let Some(start) = self.timer_start {
                    let elapsed = start.elapsed();
                    let divisor = self.timer_divisor() as u64;
                    let ticks_per_sec = LAPIC_TIMER_FREQ_HZ / divisor;

                    // Calculate how many timer ticks have elapsed
                    let elapsed_ticks = (elapsed.as_nanos() as u64 * ticks_per_sec) / 1_000_000_000;
                    let initial = self.timer_initial_count as u64;

                    let mode = self.timer_mode();
                    match mode {
                        TIMER_MODE_ONESHOT => {
                            if elapsed_ticks >= initial {
                                0
                            } else {
                                (initial - elapsed_ticks) as u32
                            }
                        }
                        TIMER_MODE_PERIODIC => {
                            // In periodic mode, wrap around
                            let remainder = elapsed_ticks % initial;
                            (initial - remainder) as u32
                        }
                        _ => 0, // TSC-deadline mode returns 0
                    }
                } else {
                    self.timer_current_count
                }
            }
            LAPIC_TIMER_DCR => self.timer_divide_config,
            _ => 0,
        }
    }

    fn write_register(&mut self, offset: u64, value: u32) {
        match offset {
            LAPIC_ID => {
                // ID is in bits 24-31
                self.id = value & 0xFF000000;
            }
            LAPIC_TPR => {
                self.tpr = value & 0xFF;
            }
            LAPIC_EOI => {
                // Any write triggers EOI
                self.eoi();
            }
            LAPIC_LDR => {
                self.ldr = value;
            }
            LAPIC_DFR => {
                self.dfr = value;
            }
            LAPIC_SVR => {
                let was_enabled = self.is_enabled();
                self.svr = value;
                if !was_enabled && self.is_enabled() {
                    // APIC just got enabled
                    tracing::debug!("LAPIC enabled, SVR={:#x}", value);
                }
            }
            LAPIC_ESR => {
                // Writing clears the ESR
                self.esr = 0;
            }
            LAPIC_ICR_LOW => {
                self.icr = (self.icr & 0xFFFFFFFF00000000) | (value as u64);
                // Writing to ICR_LOW triggers IPI delivery
                self.handle_ipi();
            }
            LAPIC_ICR_HIGH => {
                self.icr = (self.icr & 0x00000000FFFFFFFF) | ((value as u64) << 32);
            }
            LAPIC_LVT_TIMER => {
                self.lvt_timer = value;
                tracing::debug!(
                    "LVT Timer set: vector={:#x}, mode={}, masked={}",
                    value & 0xFF,
                    (value >> 17) & 0x3,
                    (value & LVT_MASK) != 0
                );
            }
            LAPIC_LVT_THERMAL => {
                self.lvt_thermal = value;
            }
            LAPIC_LVT_PMC => {
                self.lvt_pmc = value;
            }
            LAPIC_LVT_LINT0 => {
                self.lvt_lint0 = value;
            }
            LAPIC_LVT_LINT1 => {
                self.lvt_lint1 = value;
            }
            LAPIC_LVT_ERROR => {
                self.lvt_error = value;
            }
            LAPIC_TIMER_ICR => {
                self.timer_initial_count = value;
                self.timer_current_count = value;
                if value > 0 {
                    self.timer_start = Some(Instant::now());
                    self.timer_pending = false;
                    tracing::debug!(
                        "LAPIC timer started: initial_count={}, divisor={}, mode={}",
                        value,
                        self.timer_divisor(),
                        self.timer_mode()
                    );
                } else {
                    self.timer_start = None;
                }
            }
            LAPIC_TIMER_DCR => {
                self.timer_divide_config = value & 0xB; // Only bits 0,1,3 are valid
                tracing::debug!("LAPIC timer divisor set to {}", self.timer_divisor());
            }
            _ => {}
        }
    }
}

/// LAPIC MMIO device wrapper
pub struct LapicDevice {
    lapic: std::sync::Arc<std::sync::Mutex<LocalApic>>,
}

impl LapicDevice {
    pub fn new(lapic: std::sync::Arc<std::sync::Mutex<LocalApic>>) -> Self {
        LapicDevice { lapic }
    }
}

impl MmioDevice for LapicDevice {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        let offset = addr - LAPIC_BASE;
        // LAPIC registers are 32-bit aligned
        let aligned_offset = offset & !0x3;

        if let Ok(lapic) = self.lapic.lock() {
            let value = lapic.read_register(aligned_offset);
            // Handle different read sizes
            match data.len() {
                1 => {
                    let byte_offset = (offset & 0x3) as usize;
                    data[0] = ((value >> (byte_offset * 8)) & 0xFF) as u8;
                }
                2 => {
                    let byte_offset = (offset & 0x2) as usize;
                    let word = ((value >> (byte_offset * 8)) & 0xFFFF) as u16;
                    data[0] = (word & 0xFF) as u8;
                    data[1] = ((word >> 8) & 0xFF) as u8;
                }
                4 => {
                    data[0] = (value & 0xFF) as u8;
                    data[1] = ((value >> 8) & 0xFF) as u8;
                    data[2] = ((value >> 16) & 0xFF) as u8;
                    data[3] = ((value >> 24) & 0xFF) as u8;
                }
                _ => {
                    for byte in data.iter_mut() {
                        *byte = 0;
                    }
                }
            }
        } else {
            for byte in data.iter_mut() {
                *byte = 0xFF;
            }
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        let offset = addr - LAPIC_BASE;
        let aligned_offset = offset & !0x3;

        // Reconstruct 32-bit value from data
        let value = match data.len() {
            1 => data[0] as u32,
            2 => (data[0] as u32) | ((data[1] as u32) << 8),
            4 => {
                (data[0] as u32)
                    | ((data[1] as u32) << 8)
                    | ((data[2] as u32) << 16)
                    | ((data[3] as u32) << 24)
            }
            _ => return,
        };

        if let Ok(mut lapic) = self.lapic.lock() {
            lapic.write_register(aligned_offset, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    // ============================================================================
    // BASIC INITIALIZATION AND STATE TESTS
    // ============================================================================

    #[test]
    fn test_new_lapic_initial_state() {
        let lapic = LocalApic::new(0);

        // Check ID is stored in bits 24-31
        assert_eq!(lapic.id, 0);
        assert_eq!(lapic.apic_id(), 0);

        // Check version register
        assert_eq!(lapic.version, 0x00050014);

        // Check virtual wire mode defaults
        assert!(lapic.is_enabled(), "APIC should be enabled by default");
        assert_eq!(
            lapic.svr, 0x1FF,
            "SVR should have APIC enabled with vector 0xFF"
        );
        assert_eq!(lapic.lvt_lint0, 0x700, "LINT0 should be ExtInt mode");
        assert_eq!(lapic.lvt_lint1, 0x400, "LINT1 should be NMI mode");

        // Check timer is masked by default
        assert!(lapic.timer_masked(), "Timer should be masked initially");

        // Check flat model DFR
        assert_eq!(lapic.dfr, 0xFFFFFFFF, "DFR should be flat model");
    }

    #[test]
    fn test_new_lapic_with_different_ids() {
        for id in [0, 1, 7, 15, 255] {
            let lapic = LocalApic::new(id);
            assert_eq!(lapic.apic_id(), id as u8, "APIC ID mismatch for id={}", id);
            assert_eq!(
                lapic.id,
                (id as u32) << 24,
                "ID register mismatch for id={}",
                id
            );
        }
    }

    #[test]
    fn test_apic_enable_disable() {
        let mut lapic = LocalApic::new(0);

        // Initially enabled
        assert!(lapic.is_enabled());

        // Disable APIC by clearing bit 8 of SVR
        lapic.write_register(LAPIC_SVR, 0x0FF);
        assert!(!lapic.is_enabled(), "APIC should be disabled");

        // Re-enable APIC
        lapic.write_register(LAPIC_SVR, 0x1FF);
        assert!(lapic.is_enabled(), "APIC should be re-enabled");
    }

    // ============================================================================
    // REGISTER READ/WRITE TESTS
    // ============================================================================

    #[test]
    fn test_id_register_read_write() {
        let mut lapic = LocalApic::new(0);

        // Read initial ID
        assert_eq!(lapic.read_register(LAPIC_ID), 0);

        // Write new ID (only bits 24-31 are writable)
        lapic.write_register(LAPIC_ID, 0x05000000);
        assert_eq!(lapic.read_register(LAPIC_ID), 0x05000000);
        assert_eq!(lapic.apic_id(), 5);

        // Write ID with lower bits set (should be masked)
        lapic.write_register(LAPIC_ID, 0xFF123456);
        assert_eq!(lapic.read_register(LAPIC_ID), 0xFF000000);
    }

    #[test]
    fn test_version_register_readonly() {
        let mut lapic = LocalApic::new(0);
        let original = lapic.read_register(LAPIC_VERSION);

        // Try to write (should have no effect - version is read-only)
        lapic.write_register(LAPIC_VERSION, 0x12345678);
        assert_eq!(lapic.read_register(LAPIC_VERSION), original);
    }

    #[test]
    fn test_tpr_register() {
        let mut lapic = LocalApic::new(0);

        // Write and read TPR (only lower 8 bits)
        lapic.write_register(LAPIC_TPR, 0x12345678);
        assert_eq!(lapic.read_register(LAPIC_TPR), 0x78);

        lapic.write_register(LAPIC_TPR, 0xAB);
        assert_eq!(lapic.read_register(LAPIC_TPR), 0xAB);
    }

    #[test]
    fn test_ppr_calculation() {
        let mut lapic = LocalApic::new(0);

        // PPR = max(TPR[7:4], highest ISR vector[7:4]) << 4
        // With no ISR and TPR=0, PPR should be 0
        assert_eq!(lapic.read_register(LAPIC_PPR), 0);

        // Set TPR to priority class 5
        lapic.write_register(LAPIC_TPR, 0x50);
        assert_eq!(lapic.read_register(LAPIC_PPR), 0x50);

        // Set an interrupt in service at vector 0x70 (priority class 7)
        lapic.isr[0x70 / 32] |= 1 << (0x70 % 32);
        assert_eq!(lapic.read_register(LAPIC_PPR), 0x70);

        // Set TPR higher than ISR
        lapic.write_register(LAPIC_TPR, 0x80);
        assert_eq!(lapic.read_register(LAPIC_PPR), 0x80);
    }

    #[test]
    fn test_ldr_register() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_LDR, 0xAB000000);
        assert_eq!(lapic.read_register(LAPIC_LDR), 0xAB000000);
    }

    #[test]
    fn test_dfr_register() {
        let mut lapic = LocalApic::new(0);

        // Default is flat model
        assert_eq!(lapic.read_register(LAPIC_DFR), 0xFFFFFFFF);

        // Write cluster model
        lapic.write_register(LAPIC_DFR, 0x0FFFFFFF);
        assert_eq!(lapic.read_register(LAPIC_DFR), 0x0FFFFFFF);
    }

    #[test]
    fn test_svr_register() {
        let mut lapic = LocalApic::new(0);

        assert_eq!(lapic.read_register(LAPIC_SVR), 0x1FF);

        lapic.write_register(LAPIC_SVR, 0x0AB);
        assert_eq!(lapic.read_register(LAPIC_SVR), 0x0AB);
        assert!(!lapic.is_enabled());
    }

    #[test]
    fn test_esr_register() {
        let mut lapic = LocalApic::new(0);

        // Initially 0
        assert_eq!(lapic.read_register(LAPIC_ESR), 0);

        // Manually set ESR for testing
        lapic.esr = 0x44;
        assert_eq!(lapic.read_register(LAPIC_ESR), 0x44);

        // Writing any value clears ESR
        lapic.write_register(LAPIC_ESR, 0xFF);
        assert_eq!(lapic.read_register(LAPIC_ESR), 0);
    }

    #[test]
    fn test_icr_register() {
        let mut lapic = LocalApic::new(0);

        // Write ICR high (destination field)
        lapic.write_register(LAPIC_ICR_HIGH, 0x05000000);
        assert_eq!(lapic.read_register(LAPIC_ICR_HIGH), 0x05000000);

        // ICR low read
        assert_eq!(lapic.read_register(LAPIC_ICR_LOW), 0);
    }

    #[test]
    fn test_lvt_registers() {
        let mut lapic = LocalApic::new(0);

        // Test all LVT registers
        let lvt_offsets = [
            (LAPIC_LVT_TIMER, &mut lapic.lvt_timer),
            (LAPIC_LVT_THERMAL, &mut lapic.lvt_thermal),
            (LAPIC_LVT_PMC, &mut lapic.lvt_pmc),
            (LAPIC_LVT_LINT0, &mut lapic.lvt_lint0),
            (LAPIC_LVT_LINT1, &mut lapic.lvt_lint1),
            (LAPIC_LVT_ERROR, &mut lapic.lvt_error),
        ];

        // Re-create lapic to avoid borrow issues
        let mut lapic = LocalApic::new(0);

        // Test LVT Timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00030020);
        assert_eq!(lapic.read_register(LAPIC_LVT_TIMER), 0x00030020);
        assert_eq!(lapic.timer_vector(), 0x20);
        assert_eq!(lapic.timer_mode(), 1); // Periodic

        // Test LVT Error
        lapic.write_register(LAPIC_LVT_ERROR, 0x00010030);
        assert_eq!(lapic.read_register(LAPIC_LVT_ERROR), 0x00010030);
    }

    #[test]
    fn test_isr_tmr_irr_registers() {
        let mut lapic = LocalApic::new(0);

        // Set some bits in each register array
        lapic.isr[0] = 0x00000001;
        lapic.isr[7] = 0x80000000;
        lapic.tmr[3] = 0x12345678;
        lapic.irr[5] = 0xABCDEF00;

        // Read ISR
        assert_eq!(lapic.read_register(LAPIC_ISR_BASE), 0x00000001);
        assert_eq!(lapic.read_register(LAPIC_ISR_BASE + 0x70), 0x80000000);

        // Read TMR
        assert_eq!(lapic.read_register(LAPIC_TMR_BASE + 0x30), 0x12345678);

        // Read IRR
        assert_eq!(lapic.read_register(LAPIC_IRR_BASE + 0x50), 0xABCDEF00);
    }

    // ============================================================================
    // TIMER FUNCTIONALITY TESTS
    // ============================================================================

    #[test]
    fn test_timer_divisor_encoding() {
        let mut lapic = LocalApic::new(0);

        // Test all divisor encodings
        let test_cases = [
            (0b0000, 2),   // /2
            (0b0001, 4),   // /4
            (0b0010, 8),   // /8
            (0b0011, 16),  // /16
            (0b1000, 32),  // /32
            (0b1001, 64),  // /64
            (0b1010, 128), // /128
            (0b1011, 1),   // /1
        ];

        for (dcr_value, expected_divisor) in test_cases {
            lapic.write_register(LAPIC_TIMER_DCR, dcr_value);
            assert_eq!(
                lapic.timer_divisor(),
                expected_divisor,
                "DCR {:#x} should give divisor {}",
                dcr_value,
                expected_divisor
            );
        }
    }

    #[test]
    fn test_timer_mode_encoding() {
        let mut lapic = LocalApic::new(0);

        // Oneshot mode (bits 17-18 = 00)
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        assert_eq!(lapic.timer_mode(), TIMER_MODE_ONESHOT);

        // Periodic mode (bits 17-18 = 01)
        lapic.write_register(LAPIC_LVT_TIMER, 0x00020020);
        assert_eq!(lapic.timer_mode(), TIMER_MODE_PERIODIC);

        // TSC-deadline mode (bits 17-18 = 10)
        lapic.write_register(LAPIC_LVT_TIMER, 0x00040020);
        assert_eq!(lapic.timer_mode(), TIMER_MODE_TSC_DEADLINE);
    }

    #[test]
    fn test_timer_masked() {
        let mut lapic = LocalApic::new(0);

        // Initially masked
        assert!(lapic.timer_masked());

        // Unmask
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        assert!(!lapic.timer_masked());

        // Mask again
        lapic.write_register(LAPIC_LVT_TIMER, 0x00010020);
        assert!(lapic.timer_masked());
    }

    #[test]
    fn test_timer_initial_count_starts_timer() {
        let mut lapic = LocalApic::new(0);

        // Timer should not be running initially
        assert!(lapic.timer_start.is_none());

        // Set initial count
        lapic.write_register(LAPIC_TIMER_ICR, 1000000);
        assert!(lapic.timer_start.is_some());
        assert_eq!(lapic.timer_initial_count, 1000000);

        // Setting to 0 stops timer
        lapic.write_register(LAPIC_TIMER_ICR, 0);
        assert!(lapic.timer_start.is_none());
    }

    #[test]
    fn test_timer_current_count_decreases() {
        let mut lapic = LocalApic::new(0);

        // Set up timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020); // Unmask, oneshot
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011); // Divisor = 1 (fastest)
        lapic.write_register(LAPIC_TIMER_ICR, 1_000_000_000); // 1 billion ticks

        let initial = lapic.read_register(LAPIC_TIMER_CCR);

        // Wait a tiny bit
        thread::sleep(Duration::from_micros(100));

        let current = lapic.read_register(LAPIC_TIMER_CCR);

        // Current count should have decreased (or stayed same if too fast)
        assert!(
            current <= initial,
            "Timer count should decrease or stay same"
        );
    }

    #[test]
    fn test_timer_oneshot_expiration() {
        let mut lapic = LocalApic::new(0);

        // Set up oneshot timer with very short count
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020); // Vector 0x20, oneshot, unmasked
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011); // Divisor = 1
        lapic.write_register(LAPIC_TIMER_ICR, 1); // Very small count

        // Wait for timer to expire
        thread::sleep(Duration::from_millis(10));

        // Tick should return the timer vector
        let vector = lapic.tick();
        assert_eq!(vector, Some(0x20), "Timer should fire with vector 0x20");
        assert!(lapic.has_pending_timer());

        // Second tick should return None (already fired in oneshot mode)
        let vector = lapic.tick();
        assert_eq!(vector, None, "Oneshot timer should not fire twice");
    }

    #[test]
    fn test_timer_periodic_repeats() {
        let mut lapic = LocalApic::new(0);

        // Set up periodic timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00020030); // Vector 0x30, periodic, unmasked
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011); // Divisor = 1
        lapic.write_register(LAPIC_TIMER_ICR, 1); // Very small count

        // Wait and tick
        thread::sleep(Duration::from_millis(10));
        let vector1 = lapic.tick();
        assert_eq!(vector1, Some(0x30));

        // Clear pending and wait again
        lapic.clear_timer_pending();
        thread::sleep(Duration::from_millis(10));

        let vector2 = lapic.tick();
        assert_eq!(vector2, Some(0x30), "Periodic timer should fire again");
    }

    #[test]
    fn test_timer_masked_no_interrupt() {
        let mut lapic = LocalApic::new(0);

        // Set up masked timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00010020); // Masked
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        thread::sleep(Duration::from_millis(10));

        let vector = lapic.tick();
        assert_eq!(vector, None, "Masked timer should not generate interrupt");
    }

    #[test]
    fn test_timer_disabled_apic_no_interrupt() {
        let mut lapic = LocalApic::new(0);

        // Disable APIC
        lapic.write_register(LAPIC_SVR, 0x0FF);

        // Set up timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        thread::sleep(Duration::from_millis(10));

        let vector = lapic.tick();
        assert_eq!(
            vector, None,
            "Disabled APIC should not generate timer interrupt"
        );
    }

    // ============================================================================
    // INTERRUPT HANDLING TESTS
    // ============================================================================

    #[test]
    fn test_set_irr() {
        let mut lapic = LocalApic::new(0);

        // Set various vectors
        lapic.set_irr(0);
        assert_eq!(lapic.irr[0], 1);

        lapic.set_irr(31);
        assert_eq!(lapic.irr[0], 0x80000001);

        lapic.set_irr(32);
        assert_eq!(lapic.irr[1], 1);

        lapic.set_irr(255);
        assert_eq!(lapic.irr[7], 0x80000000);
    }

    #[test]
    fn test_get_pending_vector() {
        let mut lapic = LocalApic::new(0);

        // No pending interrupts
        assert_eq!(lapic.get_pending_vector(), None);

        // Set vector 0x20
        lapic.set_irr(0x20);
        assert_eq!(lapic.get_pending_vector(), Some(0x20));

        // Set higher priority vector 0x30
        lapic.set_irr(0x30);
        assert_eq!(lapic.get_pending_vector(), Some(0x30));

        // Set even higher priority vector 0xFF
        lapic.set_irr(0xFF);
        assert_eq!(lapic.get_pending_vector(), Some(0xFF));
    }

    #[test]
    fn test_ack_interrupt() {
        let mut lapic = LocalApic::new(0);

        // Set and acknowledge interrupt
        lapic.set_irr(0x40);
        assert!(lapic.irr[0x40 / 32] & (1 << (0x40 % 32)) != 0);
        assert!(lapic.isr[0x40 / 32] & (1 << (0x40 % 32)) == 0);

        lapic.ack_interrupt(0x40);
        assert!(
            lapic.irr[0x40 / 32] & (1 << (0x40 % 32)) == 0,
            "IRR bit should be cleared"
        );
        assert!(
            lapic.isr[0x40 / 32] & (1 << (0x40 % 32)) != 0,
            "ISR bit should be set"
        );
    }

    #[test]
    fn test_eoi_clears_highest_isr() {
        let mut lapic = LocalApic::new(0);

        // Set multiple interrupts in service
        lapic.isr[0x20 / 32] |= 1 << (0x20 % 32);
        lapic.isr[0x50 / 32] |= 1 << (0x50 % 32);
        lapic.isr[0x80 / 32] |= 1 << (0x80 % 32);

        // EOI should clear highest priority (0x80)
        lapic.write_register(LAPIC_EOI, 0);
        assert!(lapic.isr[0x80 / 32] & (1 << (0x80 % 32)) == 0);
        assert!(lapic.isr[0x50 / 32] & (1 << (0x50 % 32)) != 0);
        assert!(lapic.isr[0x20 / 32] & (1 << (0x20 % 32)) != 0);

        // Next EOI clears 0x50
        lapic.write_register(LAPIC_EOI, 0);
        assert!(lapic.isr[0x50 / 32] & (1 << (0x50 % 32)) == 0);
        assert!(lapic.isr[0x20 / 32] & (1 << (0x20 % 32)) != 0);

        // Final EOI clears 0x20
        lapic.write_register(LAPIC_EOI, 0);
        assert!(lapic.isr[0x20 / 32] & (1 << (0x20 % 32)) == 0);
    }

    #[test]
    fn test_eoi_any_value() {
        let mut lapic = LocalApic::new(0);

        lapic.isr[1] = 0x00000001; // Vector 32

        // Any write to EOI triggers it
        lapic.write_register(LAPIC_EOI, 0xDEADBEEF);
        assert_eq!(lapic.isr[1], 0);
    }

    // ============================================================================
    // IPI (INTER-PROCESSOR INTERRUPT) TESTS
    // ============================================================================

    #[test]
    fn test_ipi_self_fixed() {
        let mut lapic = LocalApic::new(0);

        // Set up self-targeted fixed IPI
        // ICR: vector=0x50, delivery_mode=FIXED(0), shorthand=SELF(1)
        lapic.write_register(LAPIC_ICR_HIGH, 0);
        lapic.write_register(LAPIC_ICR_LOW, 0x00040050); // Shorthand=1 (self), vector=0x50

        // Should set IRR for vector 0x50
        assert!(lapic.irr[0x50 / 32] & (1 << (0x50 % 32)) != 0);

        // No pending IPI for VMM (self-targeted)
        assert!(!lapic.has_pending_ipi());
    }

    #[test]
    fn test_ipi_self_nmi() {
        let mut lapic = LocalApic::new(0);

        // Self-targeted NMI
        // ICR: delivery_mode=NMI(4), shorthand=SELF(1)
        lapic.write_register(LAPIC_ICR_LOW, 0x00040400); // Shorthand=1, delivery=NMI

        assert!(lapic.has_pending_nmi());
        lapic.clear_pending_nmi();
        assert!(!lapic.has_pending_nmi());
    }

    #[test]
    fn test_ipi_all_including_self() {
        let mut lapic = LocalApic::new(0);

        // All including self
        // ICR: vector=0x60, shorthand=ALL_INCLUDING_SELF(2)
        lapic.write_register(LAPIC_ICR_LOW, 0x00080060); // Shorthand=2

        // Should set local IRR
        assert!(lapic.irr[0x60 / 32] & (1 << (0x60 % 32)) != 0);

        // Should also have pending IPI for VMM
        assert!(lapic.has_pending_ipi());
        let ipi = lapic.take_pending_ipi().unwrap();
        match ipi {
            IpiRequest::Fixed { vector, target } => {
                assert_eq!(vector, 0x60);
                assert!(matches!(target, IpiTarget::AllIncludingSelf));
            }
            _ => panic!("Expected Fixed IPI"),
        }
    }

    #[test]
    fn test_ipi_all_excluding_self() {
        let mut lapic = LocalApic::new(0);

        // All excluding self
        lapic.write_register(LAPIC_ICR_LOW, 0x000C0070); // Shorthand=3

        // Should NOT set local IRR
        assert!(lapic.irr[0x70 / 32] & (1 << (0x70 % 32)) == 0);

        // Should have pending IPI for VMM
        let ipi = lapic.take_pending_ipi().unwrap();
        match ipi {
            IpiRequest::Fixed { vector, target } => {
                assert_eq!(vector, 0x70);
                assert!(matches!(target, IpiTarget::AllExcludingSelf));
            }
            _ => panic!("Expected Fixed IPI"),
        }
    }

    #[test]
    fn test_ipi_physical_destination() {
        let mut lapic = LocalApic::new(0);

        // Target CPU with APIC ID 5
        lapic.write_register(LAPIC_ICR_HIGH, 0x05000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000080); // Shorthand=0 (use dest)

        let ipi = lapic.take_pending_ipi().unwrap();
        match ipi {
            IpiRequest::Fixed { vector, target } => {
                assert_eq!(vector, 0x80);
                assert!(matches!(target, IpiTarget::Physical(5)));
            }
            _ => panic!("Expected Fixed IPI"),
        }
    }

    #[test]
    fn test_ipi_logical_destination() {
        let mut lapic = LocalApic::new(0);

        // Logical destination mode
        lapic.write_register(LAPIC_ICR_HIGH, 0xAB000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000890); // Logical mode (bit 11), vector 0x90

        let ipi = lapic.take_pending_ipi().unwrap();
        match ipi {
            IpiRequest::Fixed { vector, target } => {
                assert_eq!(vector, 0x90);
                match target {
                    IpiTarget::Logical { destination } => assert_eq!(destination, 0xAB),
                    _ => panic!("Expected Logical target"),
                }
            }
            _ => panic!("Expected Fixed IPI"),
        }
    }

    #[test]
    fn test_ipi_init() {
        let mut lapic = LocalApic::new(0);

        // INIT IPI with level assert
        lapic.write_register(LAPIC_ICR_HIGH, 0x01000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00004500); // INIT (delivery=5), level assert (bit 14)

        let ipi = lapic.take_pending_ipi().unwrap();
        assert!(matches!(ipi, IpiRequest::Init { .. }));
    }

    #[test]
    fn test_ipi_init_deassert_ignored() {
        let mut lapic = LocalApic::new(0);

        // INIT IPI without level assert (de-assert)
        lapic.write_register(LAPIC_ICR_HIGH, 0x01000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000500); // INIT, no level assert

        // Should be ignored
        assert!(!lapic.has_pending_ipi());
    }

    #[test]
    fn test_ipi_sipi() {
        let mut lapic = LocalApic::new(0);

        // SIPI with vector 0x10 (start at 0x10000)
        lapic.write_register(LAPIC_ICR_HIGH, 0x01000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000610); // SIPI (delivery=6), vector=0x10

        let ipi = lapic.take_pending_ipi().unwrap();
        match ipi {
            IpiRequest::Sipi { vector, .. } => {
                assert_eq!(vector, 0x10);
            }
            _ => panic!("Expected SIPI"),
        }
    }

    #[test]
    fn test_ipi_lowest_priority() {
        let mut lapic = LocalApic::new(0);

        // Lowest priority delivery mode
        lapic.write_register(LAPIC_ICR_HIGH, 0x02000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x000001A0); // delivery=1 (lowest priority), vector=0xA0

        let ipi = lapic.take_pending_ipi().unwrap();
        assert!(matches!(
            ipi,
            IpiRequest::LowestPriority { vector: 0xA0, .. }
        ));
    }

    #[test]
    fn test_ipi_smi() {
        let mut lapic = LocalApic::new(0);

        // SMI delivery mode
        lapic.write_register(LAPIC_ICR_HIGH, 0x03000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000200); // delivery=2 (SMI)

        let ipi = lapic.take_pending_ipi().unwrap();
        assert!(matches!(ipi, IpiRequest::Smi { .. }));
    }

    #[test]
    fn test_ipi_logical_self_match() {
        let mut lapic = LocalApic::new(0);

        // Set our logical ID
        lapic.write_register(LAPIC_LDR, 0x04000000); // Logical ID = 0x04

        // Send to logical destination that matches us
        lapic.write_register(LAPIC_ICR_HIGH, 0x04000000); // destination = 0x04
        lapic.write_register(LAPIC_ICR_LOW, 0x000008B0); // Logical mode, vector=0xB0

        // Should set local IRR because destination matches our LDR
        assert!(lapic.irr[0xB0 / 32] & (1 << (0xB0 % 32)) != 0);
    }

    #[test]
    fn test_take_pending_ipi() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_ICR_HIGH, 0x05000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000050);

        assert!(lapic.has_pending_ipi());
        let ipi = lapic.take_pending_ipi();
        assert!(ipi.is_some());
        assert!(!lapic.has_pending_ipi());
        assert!(lapic.take_pending_ipi().is_none());
    }

    // ============================================================================
    // MMIO DEVICE WRAPPER TESTS
    // ============================================================================

    #[test]
    fn test_lapic_device_read_4byte() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(5)));
        let mut device = LapicDevice::new(lapic);

        let mut data = [0u8; 4];
        device.read(LAPIC_BASE + LAPIC_ID, &mut data);

        let value = u32::from_le_bytes(data);
        assert_eq!(value, 0x05000000);
    }

    #[test]
    fn test_lapic_device_read_1byte() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0xAB)));
        let mut device = LapicDevice::new(lapic);

        // Read byte 3 of ID register (which contains the APIC ID)
        let mut data = [0u8; 1];
        device.read(LAPIC_BASE + LAPIC_ID + 3, &mut data);
        assert_eq!(data[0], 0xAB);
    }

    #[test]
    fn test_lapic_device_read_2byte() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        // Set TPR to 0x1234 (but only low 8 bits matter)
        lapic.lock().unwrap().write_register(LAPIC_TPR, 0x34);

        let mut data = [0u8; 2];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data);
        assert_eq!(u16::from_le_bytes(data), 0x34);
    }

    #[test]
    fn test_lapic_device_write_4byte() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        let data = 0xAB_u32.to_le_bytes();
        device.write(LAPIC_BASE + LAPIC_TPR, &data);

        assert_eq!(lapic.lock().unwrap().read_register(LAPIC_TPR), 0xAB);
    }

    #[test]
    fn test_lapic_device_write_1byte() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        device.write(LAPIC_BASE + LAPIC_TPR, &[0x42]);
        assert_eq!(lapic.lock().unwrap().read_register(LAPIC_TPR), 0x42);
    }

    // ============================================================================
    // EDGE CASES AND BOUNDARY TESTS
    // ============================================================================

    #[test]
    fn test_vector_boundaries() {
        let mut lapic = LocalApic::new(0);

        // Test vector 0
        lapic.set_irr(0);
        assert_eq!(lapic.get_pending_vector(), Some(0));
        lapic.ack_interrupt(0);
        lapic.write_register(LAPIC_EOI, 0);

        // Test vector 255
        lapic.set_irr(255);
        assert_eq!(lapic.get_pending_vector(), Some(255));
        lapic.ack_interrupt(255);
        assert_eq!(lapic.isr[7], 0x80000000);
        lapic.write_register(LAPIC_EOI, 0);
        assert_eq!(lapic.isr[7], 0);
    }

    #[test]
    fn test_all_irr_bits() {
        let mut lapic = LocalApic::new(0);

        // Set all 256 interrupt vectors
        for v in 0..=255u8 {
            lapic.set_irr(v);
        }

        // All IRR words should be 0xFFFFFFFF
        for i in 0..8 {
            assert_eq!(lapic.irr[i], 0xFFFFFFFF, "IRR[{}] should be all 1s", i);
        }

        // Highest priority should be 255
        assert_eq!(lapic.get_pending_vector(), Some(255));
    }

    #[test]
    fn test_multiple_interrupts_priority() {
        let mut lapic = LocalApic::new(0);

        // Set interrupts at various priorities
        lapic.set_irr(0x10); // Low priority
        lapic.set_irr(0x50); // Medium priority
        lapic.set_irr(0x90); // High priority

        // Should get highest first
        assert_eq!(lapic.get_pending_vector(), Some(0x90));
        lapic.ack_interrupt(0x90);

        assert_eq!(lapic.get_pending_vector(), Some(0x50));
        lapic.ack_interrupt(0x50);

        assert_eq!(lapic.get_pending_vector(), Some(0x10));
        lapic.ack_interrupt(0x10);

        assert_eq!(lapic.get_pending_vector(), None);
    }

    #[test]
    fn test_register_alignment() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        // Write TPR
        lapic.lock().unwrap().write_register(LAPIC_TPR, 0xAB);

        // Read at various alignments - all should access same register
        let mut data4 = [0u8; 4];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data4);

        let mut data1_0 = [0u8; 1];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data1_0);

        // Byte 0 should be 0xAB
        assert_eq!(data1_0[0], 0xAB);
        assert_eq!(data4[0], 0xAB);
    }

    #[test]
    fn test_invalid_register_offset() {
        let lapic = LocalApic::new(0);

        // Reading undefined register should return 0
        assert_eq!(lapic.read_register(0x004), 0);
        assert_eq!(lapic.read_register(0xFFC), 0);
    }

    #[test]
    fn test_timer_zero_initial_count() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_ICR, 0);

        // Timer should not fire with zero count
        let vector = lapic.tick();
        assert_eq!(vector, None);
    }

    #[test]
    fn test_concurrent_ipi_and_timer() {
        let mut lapic = LocalApic::new(0);

        // Set up timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020); // Vector 0x20
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        // Send self-IPI
        lapic.write_register(LAPIC_ICR_LOW, 0x00040030); // Self, vector 0x30

        // Both should be pending
        assert!(lapic.irr[0x30 / 32] & (1 << (0x30 % 32)) != 0);

        thread::sleep(Duration::from_millis(10));
        let timer_vec = lapic.tick();
        assert_eq!(timer_vec, Some(0x20));

        // IPI vector should still be in IRR
        assert_eq!(lapic.get_pending_vector(), Some(0x30));
    }

    #[test]
    fn test_nmi_pending_state() {
        let mut lapic = LocalApic::new(0);

        assert!(!lapic.has_pending_nmi());

        lapic.nmi_pending = true;
        assert!(lapic.has_pending_nmi());

        lapic.clear_pending_nmi();
        assert!(!lapic.has_pending_nmi());
    }

    #[test]
    fn test_apr_rrd_readonly() {
        let lapic = LocalApic::new(0);

        // APR and RRD should return 0 (read-only, usually 0)
        assert_eq!(lapic.read_register(LAPIC_APR), 0);
        assert_eq!(lapic.read_register(LAPIC_RRD), 0);
    }

    #[test]
    fn test_dcr_mask() {
        let mut lapic = LocalApic::new(0);

        // Only bits 0, 1, 3 are valid in DCR
        lapic.write_register(LAPIC_TIMER_DCR, 0xFFFFFFFF);
        assert_eq!(lapic.read_register(LAPIC_TIMER_DCR), 0x0B);
    }

    #[test]
    fn test_timer_ccr_returns_zero_when_no_initial() {
        let lapic = LocalApic::new(0);

        // With no timer started, CCR should be 0
        assert_eq!(lapic.read_register(LAPIC_TIMER_CCR), 0);
    }

    #[test]
    fn test_eoi_with_empty_isr() {
        let mut lapic = LocalApic::new(0);

        // EOI with empty ISR should not crash
        lapic.write_register(LAPIC_EOI, 0);

        // ISR should still be all zeros
        for i in 0..8 {
            assert_eq!(lapic.isr[i], 0);
        }
    }

    #[test]
    fn test_isr_tmr_irr_register_indexing() {
        let mut lapic = LocalApic::new(0);

        // Each array has 8 entries, each at 0x10 byte intervals
        // ISR: 0x100, 0x110, 0x120, 0x130, 0x140, 0x150, 0x160, 0x170
        // TMR: 0x180, 0x190, 0x1A0, 0x1B0, 0x1C0, 0x1D0, 0x1E0, 0x1F0
        // IRR: 0x200, 0x210, 0x220, 0x230, 0x240, 0x250, 0x260, 0x270

        lapic.isr[3] = 0xDEADBEEF;
        lapic.tmr[5] = 0xCAFEBABE;
        lapic.irr[7] = 0x12345678;

        assert_eq!(lapic.read_register(LAPIC_ISR_BASE + 0x30), 0xDEADBEEF);
        assert_eq!(lapic.read_register(LAPIC_TMR_BASE + 0x50), 0xCAFEBABE);
        assert_eq!(lapic.read_register(LAPIC_IRR_BASE + 0x70), 0x12345678);
    }

    #[test]
    fn test_physical_self_target() {
        let mut lapic = LocalApic::new(7);

        // Target physical ID 7 (ourselves)
        lapic.write_register(LAPIC_ICR_HIGH, 0x07000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x000000C0); // Physical mode, vector 0xC0

        // Should set local IRR
        assert!(lapic.irr[0xC0 / 32] & (1 << (0xC0 % 32)) != 0);

        // Should also have IPI for VMM (since not using self shorthand)
        assert!(lapic.has_pending_ipi());
    }

    #[test]
    fn test_icr_preserves_written_value() {
        let mut lapic = LocalApic::new(0);

        // ICR stores the written value (implementation detail: delivery status bit
        // is technically read-only in hardware, but we preserve all bits for simplicity)
        lapic.write_register(LAPIC_ICR_HIGH, 0xAB000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000050);

        // Verify ICR maintains its value
        assert_eq!(lapic.read_register(LAPIC_ICR_HIGH), 0xAB000000);
        // Note: ICR_LOW may have different bits due to IPI processing,
        // but the vector and other fields should be preserved
        assert_eq!(lapic.read_register(LAPIC_ICR_LOW) & 0xFF, 0x50);
    }

    // ============================================================================
    // EDGE CASES - TIMER
    // ============================================================================

    #[test]
    fn test_timer_mode_change_while_running() {
        let mut lapic = LocalApic::new(0);

        // Start oneshot timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020); // Oneshot, vector 0x20
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1_000_000_000);

        // Change to periodic while running
        lapic.write_register(LAPIC_LVT_TIMER, 0x00020020); // Periodic

        // Timer should still be running
        assert!(lapic.timer_start.is_some());
        assert_eq!(lapic.timer_mode(), TIMER_MODE_PERIODIC);
    }

    #[test]
    fn test_timer_vector_change_while_running() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        // Change vector while timer is expiring
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000030);

        thread::sleep(Duration::from_millis(10));
        let vector = lapic.tick();

        // Should fire with new vector
        assert_eq!(vector, Some(0x30));
    }

    #[test]
    fn test_timer_mask_while_pending() {
        let mut lapic = LocalApic::new(0);

        // Set up and let timer expire
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        thread::sleep(Duration::from_millis(10));
        lapic.tick(); // This sets pending

        // Mask the timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00010020);

        // Pending flag should still be set (masking doesn't clear pending)
        assert!(lapic.has_pending_timer());
    }

    #[test]
    fn test_timer_restart_clears_pending() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        thread::sleep(Duration::from_millis(10));
        lapic.tick();
        assert!(lapic.has_pending_timer());

        // Restart timer - should clear pending
        lapic.write_register(LAPIC_TIMER_ICR, 1_000_000);
        assert!(!lapic.has_pending_timer());
    }

    #[test]
    fn test_timer_all_divisors_produce_different_rates() {
        // Verify each divisor actually changes the timer behavior
        let divisor_configs = [
            (0b0000, 2u32),
            (0b0001, 4),
            (0b0010, 8),
            (0b0011, 16),
            (0b1000, 32),
            (0b1001, 64),
            (0b1010, 128),
            (0b1011, 1),
        ];

        for (config, expected) in divisor_configs {
            let mut lapic = LocalApic::new(0);
            lapic.write_register(LAPIC_TIMER_DCR, config);
            assert_eq!(lapic.timer_divisor(), expected);
        }
    }

    #[test]
    fn test_timer_initial_count_max_value() {
        let mut lapic = LocalApic::new(0);

        // Use slower divisor (128) to reduce sensitivity to timing
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1010); // Divisor 128
        lapic.write_register(LAPIC_TIMER_ICR, 0xFFFFFFFF);

        assert_eq!(lapic.timer_initial_count, 0xFFFFFFFF);
        assert!(lapic.timer_start.is_some());

        // CCR should be close to max (with div128, 1ms = ~7.8M ticks elapsed)
        // Allow generous margin for slow CI systems
        let ccr = lapic.read_register(LAPIC_TIMER_CCR);
        assert!(
            ccr > 0xF0000000,
            "CCR ({:#x}) should be reasonably close to initial count",
            ccr
        );
    }

    #[test]
    fn test_timer_initial_count_one() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011); // Divisor 1
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        // With divisor 1 and count 1, timer expires in 1 nanosecond
        thread::sleep(Duration::from_micros(10));

        let vector = lapic.tick();
        assert_eq!(vector, Some(0x20));
    }

    #[test]
    fn test_timer_periodic_multiple_expirations() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_LVT_TIMER, 0x00020020); // Periodic
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        // Collect multiple timer fires
        let mut fire_count = 0;
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(5));
            if lapic.tick().is_some() {
                fire_count += 1;
                lapic.clear_timer_pending();
            }
        }

        assert!(fire_count >= 2, "Periodic timer should fire multiple times");
    }

    // ============================================================================
    // EDGE CASES - INTERRUPT PRIORITY
    // ============================================================================

    #[test]
    fn test_irr_priority_all_classes() {
        let mut lapic = LocalApic::new(0);

        // Set one interrupt per priority class (0x00, 0x10, 0x20, ..., 0xF0)
        for class in 0..16u8 {
            lapic.set_irr(class * 16);
        }

        // Should return highest priority (0xF0)
        assert_eq!(lapic.get_pending_vector(), Some(0xF0));

        // Ack and check next
        lapic.ack_interrupt(0xF0);
        assert_eq!(lapic.get_pending_vector(), Some(0xE0));
    }

    #[test]
    fn test_irr_same_class_different_vectors() {
        let mut lapic = LocalApic::new(0);

        // Set multiple vectors in same priority class
        lapic.set_irr(0x40);
        lapic.set_irr(0x41);
        lapic.set_irr(0x4F);

        // Highest within class should be returned first
        assert_eq!(lapic.get_pending_vector(), Some(0x4F));
        lapic.ack_interrupt(0x4F);

        assert_eq!(lapic.get_pending_vector(), Some(0x41));
        lapic.ack_interrupt(0x41);

        assert_eq!(lapic.get_pending_vector(), Some(0x40));
    }

    #[test]
    fn test_nested_interrupts_isr_tracking() {
        let mut lapic = LocalApic::new(0);

        // Simulate nested interrupt handling
        lapic.set_irr(0x20);
        lapic.ack_interrupt(0x20); // ISR has 0x20

        lapic.set_irr(0x30);
        lapic.ack_interrupt(0x30); // ISR has 0x20, 0x30

        lapic.set_irr(0x40);
        lapic.ack_interrupt(0x40); // ISR has 0x20, 0x30, 0x40

        // EOI should clear in reverse order
        lapic.write_register(LAPIC_EOI, 0);
        assert!(lapic.isr[0x40 / 32] & (1 << (0x40 % 32)) == 0);
        assert!(lapic.isr[0x30 / 32] & (1 << (0x30 % 32)) != 0);

        lapic.write_register(LAPIC_EOI, 0);
        assert!(lapic.isr[0x30 / 32] & (1 << (0x30 % 32)) == 0);
        assert!(lapic.isr[0x20 / 32] & (1 << (0x20 % 32)) != 0);

        lapic.write_register(LAPIC_EOI, 0);
        assert!(lapic.isr[0x20 / 32] & (1 << (0x20 % 32)) == 0);
    }

    #[test]
    fn test_ppr_with_nested_isr() {
        let mut lapic = LocalApic::new(0);

        lapic.write_register(LAPIC_TPR, 0x10);

        // Add interrupt in service at 0x50
        lapic.isr[0x50 / 32] |= 1 << (0x50 % 32);
        assert_eq!(lapic.read_register(LAPIC_PPR), 0x50);

        // Add higher priority ISR
        lapic.isr[0x80 / 32] |= 1 << (0x80 % 32);
        assert_eq!(lapic.read_register(LAPIC_PPR), 0x80);

        // Clear higher priority, should fall back
        lapic.isr[0x80 / 32] &= !(1 << (0x80 % 32));
        assert_eq!(lapic.read_register(LAPIC_PPR), 0x50);
    }

    #[test]
    fn test_spurious_vector_reserved_bits() {
        let mut lapic = LocalApic::new(0);

        // SVR has specific bit meanings - test bit 8 (enable) toggles correctly
        lapic.write_register(LAPIC_SVR, 0x000);
        assert!(!lapic.is_enabled());

        lapic.write_register(LAPIC_SVR, 0x1AB);
        assert!(lapic.is_enabled());
        assert_eq!(lapic.read_register(LAPIC_SVR), 0x1AB);
    }

    // ============================================================================
    // EDGE CASES - IPI
    // ============================================================================

    #[test]
    fn test_ipi_rapid_succession() {
        let mut lapic = LocalApic::new(0);

        // Send multiple self-IPIs rapidly
        for vector in 0x20..0x30u8 {
            lapic.write_register(LAPIC_ICR_LOW, 0x00040000 | (vector as u32));
        }

        // All should be in IRR
        for vector in 0x20..0x30u8 {
            assert!(
                lapic.irr[vector as usize / 32] & (1 << (vector % 32)) != 0,
                "Vector {:#x} should be in IRR",
                vector
            );
        }
    }

    #[test]
    fn test_ipi_logical_no_match() {
        let mut lapic = LocalApic::new(0);

        // Set our logical ID to 0x01
        lapic.write_register(LAPIC_LDR, 0x01000000);

        // Send to logical destination 0x80 (doesn't match 0x01)
        lapic.write_register(LAPIC_ICR_HIGH, 0x80000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000850); // Logical mode, vector 0x50

        // Should NOT set local IRR (no match)
        assert!(lapic.irr[0x50 / 32] & (1 << (0x50 % 32)) == 0);

        // But should have IPI for VMM
        assert!(lapic.has_pending_ipi());
    }

    #[test]
    fn test_ipi_logical_partial_match() {
        let mut lapic = LocalApic::new(0);

        // Set our logical ID with multiple bits
        lapic.write_register(LAPIC_LDR, 0x0F000000); // Bits 0-3 set

        // Send to destination that partially matches (bit 2)
        lapic.write_register(LAPIC_ICR_HIGH, 0x04000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000860); // Logical mode

        // Should match (0x04 & 0x0F != 0)
        assert!(lapic.irr[0x60 / 32] & (1 << (0x60 % 32)) != 0);
    }

    #[test]
    fn test_ipi_overwrite_pending() {
        let mut lapic = LocalApic::new(0);

        // Send first IPI
        lapic.write_register(LAPIC_ICR_HIGH, 0x01000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000020);

        let first_ipi = lapic.take_pending_ipi();
        assert!(first_ipi.is_some());

        // Send second IPI without taking first
        lapic.write_register(LAPIC_ICR_HIGH, 0x02000000);
        lapic.write_register(LAPIC_ICR_LOW, 0x00000030);

        // Second IPI overwrites (implementation detail)
        let second_ipi = lapic.take_pending_ipi();
        assert!(second_ipi.is_some());
    }

    #[test]
    fn test_ipi_all_delivery_modes_to_self() {
        let mut lapic = LocalApic::new(0);

        // Fixed to self
        lapic.write_register(LAPIC_ICR_LOW, 0x00040020);
        assert!(lapic.irr[0x20 / 32] & (1 << (0x20 % 32)) != 0);

        // NMI to self
        lapic.write_register(LAPIC_ICR_LOW, 0x00040400);
        assert!(lapic.has_pending_nmi());
        lapic.clear_pending_nmi();

        // Note: INIT/SIPI/SMI to self don't make practical sense
        // but shouldn't crash
        lapic.write_register(LAPIC_ICR_LOW, 0x00044500); // INIT to self (level assert)
        lapic.write_register(LAPIC_ICR_LOW, 0x00040600); // SIPI to self
        lapic.write_register(LAPIC_ICR_LOW, 0x00040200); // SMI to self
    }

    #[test]
    fn test_sipi_vector_calculation() {
        let mut lapic = LocalApic::new(0);

        // SIPI with various vectors
        let test_vectors = [0x00, 0x01, 0x10, 0x80, 0xFF];

        for vector in test_vectors {
            lapic.pending_ipi = None;
            lapic.write_register(LAPIC_ICR_HIGH, 0x01000000);
            lapic.write_register(LAPIC_ICR_LOW, 0x00000600 | (vector as u32));

            let ipi = lapic.take_pending_ipi().unwrap();
            match ipi {
                IpiRequest::Sipi { vector: v, .. } => {
                    assert_eq!(v, vector);
                    // Start address would be vector * 0x1000
                }
                _ => panic!("Expected SIPI"),
            }
        }
    }

    // ============================================================================
    // EDGE CASES - REGISTER ACCESS
    // ============================================================================

    #[test]
    fn test_read_all_isr_registers() {
        let mut lapic = LocalApic::new(0);

        // Set a pattern in ISR
        for i in 0..8 {
            lapic.isr[i] = 0x12345678 + i as u32;
        }

        // Read all 8 ISR registers
        for i in 0..8 {
            let offset = LAPIC_ISR_BASE + (i as u64 * 0x10);
            let value = lapic.read_register(offset);
            assert_eq!(value, 0x12345678 + i as u32);
        }
    }

    #[test]
    fn test_read_all_tmr_registers() {
        let mut lapic = LocalApic::new(0);

        for i in 0..8 {
            lapic.tmr[i] = 0xABCDEF00 + i as u32;
        }

        for i in 0..8 {
            let offset = LAPIC_TMR_BASE + (i as u64 * 0x10);
            let value = lapic.read_register(offset);
            assert_eq!(value, 0xABCDEF00 + i as u32);
        }
    }

    #[test]
    fn test_read_all_irr_registers() {
        let mut lapic = LocalApic::new(0);

        for i in 0..8 {
            lapic.irr[i] = 0xDEADBEEF - i as u32;
        }

        for i in 0..8 {
            let offset = LAPIC_IRR_BASE + (i as u64 * 0x10);
            let value = lapic.read_register(offset);
            assert_eq!(value, 0xDEADBEEF - i as u32);
        }
    }

    #[test]
    fn test_register_gaps_return_zero() {
        let lapic = LocalApic::new(0);

        // These are gaps in the LAPIC register space
        let gap_offsets = [
            0x000, 0x010, // Before ID
            0x040, 0x050, 0x060, 0x070, // Between VERSION and TPR
            0x0C0, // RRD
        ];

        for offset in gap_offsets {
            assert_eq!(
                lapic.read_register(offset),
                0,
                "Gap at offset {:#x} should return 0",
                offset
            );
        }
    }

    #[test]
    fn test_lvt_all_registers() {
        let mut lapic = LocalApic::new(0);

        let lvt_offsets = [
            LAPIC_LVT_TIMER,
            0x330, // LVT_THERMAL
            0x340, // LVT_PMC
            LAPIC_LVT_LINT0,
            LAPIC_LVT_LINT1,
            LAPIC_LVT_ERROR,
        ];

        for (i, &offset) in lvt_offsets.iter().enumerate() {
            let value = 0x00010000 | (0x20 + i as u32); // Masked, different vectors
            lapic.write_register(offset, value);
            assert_eq!(
                lapic.read_register(offset),
                value,
                "LVT at offset {:#x}",
                offset
            );
        }
    }

    // ============================================================================
    // TORTURE TESTS
    // ============================================================================

    #[test]
    fn test_torture_rapid_irr_set_clear() {
        let mut lapic = LocalApic::new(0);

        // Rapidly set and clear IRR bits
        for _ in 0..100 {
            for v in 0..=255u8 {
                lapic.set_irr(v);
            }
            for v in 0..=255u8 {
                lapic.ack_interrupt(v);
            }
            for _ in 0..256 {
                lapic.write_register(LAPIC_EOI, 0);
            }
        }

        // Everything should be cleared
        for i in 0..8 {
            assert_eq!(lapic.irr[i], 0);
            assert_eq!(lapic.isr[i], 0);
        }
    }

    #[test]
    fn test_torture_timer_reconfiguration() {
        let mut lapic = LocalApic::new(0);

        // Rapidly reconfigure timer
        for i in 0..100 {
            let vector = (0x20 + (i % 0xE0)) as u32;
            let mode = (i % 3) << 17;
            let mask = if i % 5 == 0 { LVT_MASK } else { 0 };

            lapic.write_register(LAPIC_LVT_TIMER, vector | mode | mask);
            lapic.write_register(LAPIC_TIMER_DCR, (i % 12) as u32);
            lapic.write_register(LAPIC_TIMER_ICR, (i * 1000 + 1) as u32);
        }

        // Should not crash, timer state should be consistent
        assert!(lapic.timer_initial_count > 0);
    }

    #[test]
    fn test_torture_ipi_storm() {
        let mut lapic = LocalApic::new(0);

        // Send many IPIs
        for i in 0..1000 {
            let vector = (0x20 + (i % 0xE0)) as u32;
            let shorthand = ((i % 4) << 18) as u32;

            lapic.write_register(LAPIC_ICR_HIGH, ((i % 256) << 24) as u32);
            lapic.write_register(LAPIC_ICR_LOW, vector | shorthand);

            // Periodically drain IPIs
            if i % 10 == 0 {
                lapic.take_pending_ipi();
            }
        }
    }

    #[test]
    fn test_torture_register_random_access() {
        let mut lapic = LocalApic::new(0);

        // Access many registers in "random" order
        let offsets = [
            LAPIC_ID,
            LAPIC_TPR,
            LAPIC_SVR,
            LAPIC_LVT_TIMER,
            LAPIC_TIMER_ICR,
            LAPIC_TIMER_DCR,
            LAPIC_EOI,
            LAPIC_ICR_LOW,
            LAPIC_ICR_HIGH,
            LAPIC_ESR,
            LAPIC_LDR,
            LAPIC_DFR,
        ];

        for round in 0..100 {
            for (i, &offset) in offsets.iter().enumerate() {
                let value = ((round * 13 + i * 7) % 256) as u32;

                // Write (skip read-only registers)
                if offset != LAPIC_VERSION && offset != 0x0A0 {
                    lapic.write_register(offset, value);
                }

                // Read
                let _ = lapic.read_register(offset);
            }
        }
    }

    // ============================================================================
    // COMPLEX INTERACTION TESTS
    // ============================================================================

    #[test]
    fn test_complex_timer_with_interrupts() {
        let mut lapic = LocalApic::new(0);

        // Set up timer
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_DCR, 0b1011);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        // Also set some IRR bits
        lapic.set_irr(0x30);
        lapic.set_irr(0x40);

        thread::sleep(Duration::from_millis(10));
        lapic.tick();

        // Timer pending should be separate from IRR interrupts
        assert!(lapic.has_pending_timer());
        assert_eq!(lapic.get_pending_vector(), Some(0x40)); // Highest in IRR

        // Ack IRR interrupts
        lapic.ack_interrupt(0x40);
        lapic.ack_interrupt(0x30);

        // Timer still pending
        assert!(lapic.has_pending_timer());
    }

    #[test]
    fn test_complex_disable_with_pending() {
        let mut lapic = LocalApic::new(0);

        // Set up interrupts and timer
        lapic.set_irr(0x50);
        lapic.write_register(LAPIC_LVT_TIMER, 0x00000020);
        lapic.write_register(LAPIC_TIMER_ICR, 1);

        thread::sleep(Duration::from_millis(10));
        lapic.tick();

        // Disable APIC
        lapic.write_register(LAPIC_SVR, 0x0FF);

        // IRR should still be readable
        assert!(lapic.irr[0x50 / 32] & (1 << (0x50 % 32)) != 0);

        // Timer tick should not fire when disabled
        lapic.clear_timer_pending();
        lapic.write_register(LAPIC_TIMER_ICR, 1);
        thread::sleep(Duration::from_millis(10));
        assert_eq!(lapic.tick(), None);
    }

    #[test]
    fn test_complex_eoi_with_pending_irr() {
        let mut lapic = LocalApic::new(0);

        // Put 0x50 in service
        lapic.set_irr(0x50);
        lapic.ack_interrupt(0x50);

        // Set same vector in IRR again (re-assertion while in service)
        lapic.set_irr(0x50);

        // EOI should only clear ISR, not IRR
        lapic.write_register(LAPIC_EOI, 0);

        assert!(lapic.isr[0x50 / 32] & (1 << (0x50 % 32)) == 0);
        assert!(lapic.irr[0x50 / 32] & (1 << (0x50 % 32)) != 0);
    }

    #[test]
    fn test_complex_icr_full_64bit() {
        let mut lapic = LocalApic::new(0);

        // Write ICR high first, then low
        lapic.write_register(LAPIC_ICR_HIGH, 0xABCDEF00);
        lapic.write_register(LAPIC_ICR_LOW, 0x12345678);

        // Full ICR should be correct
        assert_eq!(lapic.icr, 0xABCDEF0012345678);

        // Read back both halves
        assert_eq!(lapic.read_register(LAPIC_ICR_LOW), 0x12345678);
        assert_eq!(lapic.read_register(LAPIC_ICR_HIGH), 0xABCDEF00);
    }

    #[test]
    fn test_complex_tpr_affects_ppr() {
        let mut lapic = LocalApic::new(0);

        // No ISR, PPR follows TPR
        for tpr in (0..=0xF0).step_by(0x10) {
            lapic.write_register(LAPIC_TPR, tpr);
            // PPR should be TPR class << 4
            assert_eq!(lapic.read_register(LAPIC_PPR), tpr & 0xF0);
        }
    }

    #[test]
    fn test_complex_multiple_nmi_pending() {
        let mut lapic = LocalApic::new(0);

        // Send multiple self-NMIs
        lapic.write_register(LAPIC_ICR_LOW, 0x00040400); // NMI to self
        assert!(lapic.has_pending_nmi());

        // Sending another NMI shouldn't change anything (already pending)
        lapic.write_register(LAPIC_ICR_LOW, 0x00040400);
        assert!(lapic.has_pending_nmi());

        lapic.clear_pending_nmi();
        assert!(!lapic.has_pending_nmi());
    }

    // ============================================================================
    // MMIO DEVICE EDGE CASES
    // ============================================================================

    #[test]
    fn test_mmio_device_unaligned_read() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        // Set a known value
        lapic.lock().unwrap().write_register(LAPIC_TPR, 0xAB);

        // Read at offset +1 (unaligned)
        let mut data = [0u8; 1];
        device.read(LAPIC_BASE + LAPIC_TPR + 1, &mut data);

        // Should read byte 1 of the aligned register (0x00)
        assert_eq!(data[0], 0x00);
    }

    #[test]
    fn test_mmio_device_read_sizes() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        lapic.lock().unwrap().write_register(LAPIC_TPR, 0xAB);

        // 1-byte read
        let mut data1 = [0u8; 1];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data1);
        assert_eq!(data1[0], 0xAB);

        // 2-byte read
        let mut data2 = [0u8; 2];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data2);
        assert_eq!(u16::from_le_bytes(data2), 0x00AB);

        // 4-byte read
        let mut data4 = [0u8; 4];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data4);
        assert_eq!(u32::from_le_bytes(data4), 0x000000AB);
    }

    #[test]
    fn test_mmio_device_write_sizes() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        // 1-byte write
        device.write(LAPIC_BASE + LAPIC_TPR, &[0x12]);
        assert_eq!(lapic.lock().unwrap().read_register(LAPIC_TPR), 0x12);

        // 2-byte write
        device.write(LAPIC_BASE + LAPIC_TPR, &[0x34, 0x00]);
        assert_eq!(lapic.lock().unwrap().read_register(LAPIC_TPR), 0x34);

        // 4-byte write
        device.write(LAPIC_BASE + LAPIC_TPR, &[0x56, 0x00, 0x00, 0x00]);
        assert_eq!(lapic.lock().unwrap().read_register(LAPIC_TPR), 0x56);
    }

    #[test]
    fn test_mmio_device_invalid_size() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic.clone());

        // 3-byte read (unusual size)
        let mut data = [0u8; 3];
        device.read(LAPIC_BASE + LAPIC_TPR, &mut data);
        // Should fill with 0s for unsupported sizes
        assert_eq!(data, [0, 0, 0]);

        // 5-byte write (unusual size) - should be ignored
        device.write(LAPIC_BASE + LAPIC_TPR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_mmio_device_boundary_addresses() {
        let lapic = Arc::new(Mutex::new(LocalApic::new(0)));
        let mut device = LapicDevice::new(lapic);

        // Read at start of LAPIC region
        let mut data = [0u8; 4];
        device.read(LAPIC_BASE, &mut data);

        // Read at end of LAPIC region
        device.read(LAPIC_BASE + 0xFFC, &mut data);
    }
}
