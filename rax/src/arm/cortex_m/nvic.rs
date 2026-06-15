//! Nested Vectored Interrupt Controller (NVIC) for Cortex-M.
//!
//! The NVIC provides:
//! - Up to 496 external interrupts (implementation defined, typically 32-240)
//! - 8-bit priority levels with grouping
//! - Interrupt enable/disable/pending control
//! - Active interrupt tracking
//! - Software interrupt generation
//!
//! Memory map (offset from SCS base 0xE000E000):
//! - 0x100-0x17F: NVIC_ISER0-ISER15 (Interrupt Set-Enable)
//! - 0x180-0x1FF: NVIC_ICER0-ICER15 (Interrupt Clear-Enable)
//! - 0x200-0x27F: NVIC_ISPR0-ISPR15 (Interrupt Set-Pending)
//! - 0x280-0x2FF: NVIC_ICPR0-ICPR15 (Interrupt Clear-Pending)
//! - 0x300-0x37F: NVIC_IABR0-IABR15 (Interrupt Active Bit)
//! - 0x380-0x3FF: NVIC_ITNS0-ITNS15 (Interrupt Target Non-Secure, ARMv8-M)
//! - 0x400-0x4EF: NVIC_IPR0-IPR123 (Interrupt Priority)
//! - 0xE00: STIR (Software Trigger Interrupt Register)

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Maximum number of external interrupts supported.
pub const MAX_INTERRUPTS: usize = 496;

/// Number of system exceptions (exception numbers 1-15).
pub const NUM_SYSTEM_EXCEPTIONS: usize = 16;

/// Interrupt priority with exception number for comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingInterrupt {
    /// Effective priority (lower = higher priority).
    priority: i16,
    /// Exception number.
    exception_num: u16,
}

impl Ord for PendingInterrupt {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower priority value = higher priority
        // If priorities equal, lower exception number wins
        match other.priority.cmp(&self.priority) {
            Ordering::Equal => other.exception_num.cmp(&self.exception_num),
            other => other,
        }
    }
}

impl PartialOrd for PendingInterrupt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// NVIC state.
#[derive(Clone, Debug)]
pub struct Nvic {
    /// Number of implemented interrupts.
    num_interrupts: usize,
    /// Priority bits implemented (3-8).
    priority_bits: u8,
    /// Interrupt enable bits (one bit per interrupt).
    enabled: [u32; 16],
    /// Interrupt pending bits.
    pending: [u32; 16],
    /// Interrupt active bits.
    active: [u32; 16],
    /// Interrupt target non-secure (ARMv8-M).
    target_ns: [u32; 16],
    /// Interrupt priorities (8-bit per interrupt).
    priority: [u8; MAX_INTERRUPTS],
    /// System handler priorities (for exceptions 4-15).
    /// Index 0 = MemManage (exception 4), index 11 = SysTick (exception 15)
    system_priority: [u8; 12],
    /// Priority grouping (PRIGROUP field from AIRCR).
    priority_group: u8,
    /// PRIMASK register (1 = mask all configurable exceptions).
    primask: bool,
    /// FAULTMASK register (1 = mask all exceptions except NMI).
    faultmask: bool,
    /// BASEPRI register (mask exceptions with priority >= BASEPRI).
    basepri: u8,
    /// Currently executing exception number (0 = Thread mode).
    current_exception: u16,
    /// Exception return value (EXC_RETURN).
    exc_return: u32,
    /// Secure state (ARMv8-M).
    secure: bool,
}

impl Nvic {
    /// Create a new NVIC with the specified number of interrupts.
    pub fn new(num_interrupts: usize, priority_bits: u8) -> Self {
        assert!(num_interrupts <= MAX_INTERRUPTS);
        assert!((2..=8).contains(&priority_bits)); // M0/M0+ have 2 bits

        Self {
            num_interrupts,
            priority_bits,
            enabled: [0; 16],
            pending: [0; 16],
            active: [0; 16],
            target_ns: [0; 16],
            priority: [0; MAX_INTERRUPTS],
            system_priority: [0; 12],
            priority_group: 0,
            primask: false,
            faultmask: false,
            basepri: 0,
            current_exception: 0,
            exc_return: 0xFFFF_FFFF,
            secure: true,
        }
    }

    /// Create NVIC for a specific Cortex-M variant.
    pub fn for_cortex_m0() -> Self {
        Self::new(32, 2) // M0 has 2 priority bits
    }

    pub fn for_cortex_m0plus() -> Self {
        Self::new(32, 2)
    }

    pub fn for_cortex_m3() -> Self {
        Self::new(240, 8)
    }

    pub fn for_cortex_m4() -> Self {
        Self::new(240, 8)
    }

    pub fn for_cortex_m7() -> Self {
        Self::new(240, 8)
    }

    pub fn for_cortex_m23() -> Self {
        Self::new(240, 8)
    }

    pub fn for_cortex_m33() -> Self {
        Self::new(480, 8)
    }

    pub fn for_cortex_m55() -> Self {
        Self::new(480, 8)
    }

    pub fn for_cortex_m85() -> Self {
        Self::new(480, 8)
    }

    /// Reset the NVIC to initial state.
    pub fn reset(&mut self) {
        self.enabled = [0; 16];
        self.pending = [0; 16];
        self.active = [0; 16];
        self.priority = [0; MAX_INTERRUPTS];
        self.system_priority = [0; 12];
        self.priority_group = 0;
        self.primask = false;
        self.faultmask = false;
        self.basepri = 0;
        self.current_exception = 0;
        self.exc_return = 0xFFFF_FFFF;
    }

    // =========================================================================
    // Interrupt Enable/Disable
    // =========================================================================

    /// Check if an interrupt is enabled.
    pub fn is_enabled(&self, irq: u16) -> bool {
        if irq as usize >= self.num_interrupts {
            return false;
        }
        let idx = irq as usize / 32;
        let bit = irq as usize % 32;
        (self.enabled[idx] >> bit) & 1 != 0
    }

    /// Enable an interrupt.
    pub fn enable(&mut self, irq: u16) {
        if (irq as usize) < self.num_interrupts {
            let idx = irq as usize / 32;
            let bit = irq as usize % 32;
            self.enabled[idx] |= 1 << bit;
        }
    }

    /// Disable an interrupt.
    pub fn disable(&mut self, irq: u16) {
        if (irq as usize) < self.num_interrupts {
            let idx = irq as usize / 32;
            let bit = irq as usize % 32;
            self.enabled[idx] &= !(1 << bit);
        }
    }

    /// Read ISER register.
    pub fn read_iser(&self, reg: usize) -> u32 {
        if reg < 16 { self.enabled[reg] } else { 0 }
    }

    /// Write ISER register (set-enable, write 1 to enable).
    pub fn write_iser(&mut self, reg: usize, value: u32) {
        if reg < 16 {
            self.enabled[reg] |= value;
        }
    }

    /// Write ICER register (clear-enable, write 1 to disable).
    pub fn write_icer(&mut self, reg: usize, value: u32) {
        if reg < 16 {
            self.enabled[reg] &= !value;
        }
    }

    // =========================================================================
    // Interrupt Pending
    // =========================================================================

    /// Check if an interrupt is pending.
    pub fn is_pending(&self, irq: u16) -> bool {
        if irq as usize >= self.num_interrupts {
            return false;
        }
        let idx = irq as usize / 32;
        let bit = irq as usize % 32;
        (self.pending[idx] >> bit) & 1 != 0
    }

    /// Set an interrupt pending.
    pub fn set_pending(&mut self, irq: u16) {
        if (irq as usize) < self.num_interrupts {
            let idx = irq as usize / 32;
            let bit = irq as usize % 32;
            self.pending[idx] |= 1 << bit;
        }
    }

    /// Clear a pending interrupt.
    pub fn clear_pending(&mut self, irq: u16) {
        if (irq as usize) < self.num_interrupts {
            let idx = irq as usize / 32;
            let bit = irq as usize % 32;
            self.pending[idx] &= !(1 << bit);
        }
    }

    /// Read ISPR register.
    pub fn read_ispr(&self, reg: usize) -> u32 {
        if reg < 16 { self.pending[reg] } else { 0 }
    }

    /// Write ISPR register (set-pending).
    pub fn write_ispr(&mut self, reg: usize, value: u32) {
        if reg < 16 {
            self.pending[reg] |= value;
        }
    }

    /// Write ICPR register (clear-pending).
    pub fn write_icpr(&mut self, reg: usize, value: u32) {
        if reg < 16 {
            self.pending[reg] &= !value;
        }
    }

    // =========================================================================
    // Interrupt Active
    // =========================================================================

    /// Check if an interrupt is active.
    pub fn is_active(&self, irq: u16) -> bool {
        if irq as usize >= self.num_interrupts {
            return false;
        }
        let idx = irq as usize / 32;
        let bit = irq as usize % 32;
        (self.active[idx] >> bit) & 1 != 0
    }

    /// Set an interrupt active (internal use during exception entry).
    pub fn set_active(&mut self, irq: u16) {
        if (irq as usize) < self.num_interrupts {
            let idx = irq as usize / 32;
            let bit = irq as usize % 32;
            self.active[idx] |= 1 << bit;
        }
    }

    /// Clear an interrupt active (internal use during exception return).
    pub fn clear_active(&mut self, irq: u16) {
        if (irq as usize) < self.num_interrupts {
            let idx = irq as usize / 32;
            let bit = irq as usize % 32;
            self.active[idx] &= !(1 << bit);
        }
    }

    /// Read IABR register.
    pub fn read_iabr(&self, reg: usize) -> u32 {
        if reg < 16 { self.active[reg] } else { 0 }
    }

    // =========================================================================
    // Interrupt Priority
    // =========================================================================

    /// Get interrupt priority.
    pub fn get_priority(&self, irq: u16) -> u8 {
        if (irq as usize) < self.num_interrupts {
            self.priority[irq as usize]
        } else {
            0
        }
    }

    /// Set interrupt priority.
    pub fn set_priority(&mut self, irq: u16, priority: u8) {
        if (irq as usize) < self.num_interrupts {
            // Mask to implemented priority bits (MSB-aligned)
            let mask = 0xFF << (8 - self.priority_bits);
            self.priority[irq as usize] = priority & mask;
        }
    }

    /// Read IPR register (4 priorities per register).
    pub fn read_ipr(&self, reg: usize) -> u32 {
        let base = reg * 4;
        if base + 3 < self.num_interrupts {
            (self.priority[base] as u32)
                | ((self.priority[base + 1] as u32) << 8)
                | ((self.priority[base + 2] as u32) << 16)
                | ((self.priority[base + 3] as u32) << 24)
        } else {
            0
        }
    }

    /// Write IPR register.
    pub fn write_ipr(&mut self, reg: usize, value: u32) {
        let base = reg * 4;
        let mask = 0xFF << (8 - self.priority_bits);
        if base < self.num_interrupts {
            self.priority[base] = (value as u8) & mask;
        }
        if base + 1 < self.num_interrupts {
            self.priority[base + 1] = ((value >> 8) as u8) & mask;
        }
        if base + 2 < self.num_interrupts {
            self.priority[base + 2] = ((value >> 16) as u8) & mask;
        }
        if base + 3 < self.num_interrupts {
            self.priority[base + 3] = ((value >> 24) as u8) & mask;
        }
    }

    // =========================================================================
    // System Exception Priority
    // =========================================================================

    /// Get system exception priority (exception numbers 4-15).
    pub fn get_system_priority(&self, exception_num: u16) -> i16 {
        match exception_num {
            1 => -3, // Reset (fixed)
            2 => -2, // NMI (fixed)
            3 => -1, // HardFault (fixed, or configurable in ARMv8-M)
            4..=15 => self.system_priority[(exception_num - 4) as usize] as i16,
            _ => 0,
        }
    }

    /// Set system exception priority.
    pub fn set_system_priority(&mut self, exception_num: u16, priority: u8) {
        if (4..=15).contains(&exception_num) {
            let mask = 0xFF << (8 - self.priority_bits);
            self.system_priority[(exception_num - 4) as usize] = priority & mask;
        }
    }

    /// Read SHPR register (System Handler Priority).
    /// SHPR1: exceptions 4-7 (MemManage, BusFault, UsageFault, SecureFault)
    /// SHPR2: exceptions 8-11 (reserved, reserved, reserved, SVCall)
    /// SHPR3: exceptions 12-15 (DebugMonitor, reserved, PendSV, SysTick)
    pub fn read_shpr(&self, reg: usize) -> u32 {
        let base = reg * 4;
        if base < 12 {
            (self.system_priority[base] as u32)
                | ((self.system_priority.get(base + 1).copied().unwrap_or(0) as u32) << 8)
                | ((self.system_priority.get(base + 2).copied().unwrap_or(0) as u32) << 16)
                | ((self.system_priority.get(base + 3).copied().unwrap_or(0) as u32) << 24)
        } else {
            0
        }
    }

    /// Write SHPR register.
    pub fn write_shpr(&mut self, reg: usize, value: u32) {
        let base = reg * 4;
        let mask = 0xFF << (8 - self.priority_bits);
        for i in 0..4 {
            if base + i < 12 {
                self.system_priority[base + i] = ((value >> (i * 8)) as u8) & mask;
            }
        }
    }

    // =========================================================================
    // Priority Masking
    // =========================================================================

    /// Get PRIMASK.
    pub fn get_primask(&self) -> bool {
        self.primask
    }

    /// Set PRIMASK.
    pub fn set_primask(&mut self, value: bool) {
        self.primask = value;
    }

    /// Get FAULTMASK.
    pub fn get_faultmask(&self) -> bool {
        self.faultmask
    }

    /// Set FAULTMASK.
    pub fn set_faultmask(&mut self, value: bool) {
        self.faultmask = value;
    }

    /// Get BASEPRI.
    pub fn get_basepri(&self) -> u8 {
        self.basepri
    }

    /// Set BASEPRI.
    pub fn set_basepri(&mut self, value: u8) {
        let mask = 0xFF << (8 - self.priority_bits);
        self.basepri = value & mask;
    }

    /// Get current execution priority.
    pub fn execution_priority(&self) -> i16 {
        if self.faultmask {
            return -1; // Only NMI can preempt
        }
        if self.primask {
            return 0; // Only HardFault and NMI can preempt
        }
        if self.basepri != 0 {
            return self.basepri as i16;
        }
        if self.current_exception == 0 {
            return 256; // Thread mode, lowest priority
        }
        self.get_exception_priority(self.current_exception)
    }

    // =========================================================================
    // Exception Priority Calculation
    // =========================================================================

    /// Get effective priority for an exception number.
    pub fn get_exception_priority(&self, exception_num: u16) -> i16 {
        match exception_num {
            1 => -3, // Reset
            2 => -2, // NMI
            3 => -1, // HardFault
            4..=15 => self.system_priority[(exception_num - 4) as usize] as i16,
            16.. => {
                let irq = exception_num - 16;
                if (irq as usize) < self.num_interrupts {
                    self.priority[irq as usize] as i16
                } else {
                    256
                }
            }
            _ => 256,
        }
    }

    /// Get group priority and subpriority from priority value.
    pub fn split_priority(&self, priority: u8) -> (u8, u8) {
        let subgroup_bits = self.priority_group.min(7);
        let group_bits = 8 - subgroup_bits;
        let group_mask = 0xFF << subgroup_bits;
        let subgroup_mask = !group_mask;
        (
            (priority & group_mask) >> subgroup_bits,
            priority & subgroup_mask,
        )
    }

    /// Set priority grouping (PRIGROUP from AIRCR).
    pub fn set_priority_group(&mut self, group: u8) {
        self.priority_group = group & 0x7;
    }

    /// Get priority grouping.
    pub fn get_priority_group(&self) -> u8 {
        self.priority_group
    }

    // =========================================================================
    // Exception Entry/Return
    // =========================================================================

    /// Get current exception number.
    pub fn current_exception(&self) -> u16 {
        self.current_exception
    }

    /// Set current exception (for exception entry).
    pub fn enter_exception(&mut self, exception_num: u16) {
        self.current_exception = exception_num;

        // Set active bit for external interrupts
        if exception_num >= 16 {
            let irq = exception_num - 16;
            self.set_active(irq);
            self.clear_pending(irq);
        }
    }

    /// Return from exception.
    pub fn return_from_exception(&mut self) {
        if self.current_exception >= 16 {
            let irq = self.current_exception - 16;
            self.clear_active(irq);
        }
        self.current_exception = 0;
    }

    /// Get EXC_RETURN value.
    pub fn get_exc_return(&self) -> u32 {
        self.exc_return
    }

    /// Set EXC_RETURN value.
    pub fn set_exc_return(&mut self, value: u32) {
        self.exc_return = value;
    }

    // =========================================================================
    // Pending Exception Detection
    // =========================================================================

    /// Find highest priority pending exception that can preempt.
    pub fn get_pending_exception(&self) -> Option<u16> {
        let exec_priority = self.execution_priority();

        // Check system exceptions first (higher priority numbers = higher priority)
        // NMI (exception 2)
        // Note: NMI pending is handled separately via SCB

        // Check external interrupts
        let mut best: Option<(i16, u16)> = None;

        for irq in 0..self.num_interrupts as u16 {
            if self.is_pending(irq) && self.is_enabled(irq) {
                let exception_num = irq + 16;
                let priority = self.get_exception_priority(exception_num);

                if priority < exec_priority {
                    match best {
                        None => best = Some((priority, exception_num)),
                        Some((best_pri, best_exc)) => {
                            if priority < best_pri
                                || (priority == best_pri && exception_num < best_exc)
                            {
                                best = Some((priority, exception_num));
                            }
                        }
                    }
                }
            }
        }

        best.map(|(_, exc)| exc)
    }

    /// Check if any exception is pending and can preempt.
    pub fn has_pending_exception(&self) -> bool {
        self.get_pending_exception().is_some()
    }

    // =========================================================================
    // Software Trigger
    // =========================================================================

    /// Write STIR (Software Trigger Interrupt Register).
    pub fn write_stir(&mut self, value: u32) {
        let irq = (value & 0x1FF) as u16;
        if (irq as usize) < self.num_interrupts {
            self.set_pending(irq);
        }
    }

    // =========================================================================
    // Register Access
    // =========================================================================

    /// Read from NVIC register space (offset from 0xE000E100).
    pub fn read(&self, offset: u32) -> u32 {
        match offset {
            // ISER0-ISER15
            0x000..=0x03C => self.read_iser(((offset - 0x000) / 4) as usize),
            // ICER0-ICER15
            0x080..=0x0BC => self.read_iser(((offset - 0x080) / 4) as usize),
            // ISPR0-ISPR15
            0x100..=0x13C => self.read_ispr(((offset - 0x100) / 4) as usize),
            // ICPR0-ICPR15
            0x180..=0x1BC => self.read_ispr(((offset - 0x180) / 4) as usize),
            // IABR0-IABR15
            0x200..=0x23C => self.read_iabr(((offset - 0x200) / 4) as usize),
            // IPR0-IPR123
            0x300..=0x4EF => self.read_ipr(((offset - 0x300) / 4) as usize),
            _ => 0,
        }
    }

    /// Write to NVIC register space.
    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            // ISER0-ISER15
            0x000..=0x03C => self.write_iser(((offset - 0x000) / 4) as usize, value),
            // ICER0-ICER15
            0x080..=0x0BC => self.write_icer(((offset - 0x080) / 4) as usize, value),
            // ISPR0-ISPR15
            0x100..=0x13C => self.write_ispr(((offset - 0x100) / 4) as usize, value),
            // ICPR0-ICPR15
            0x180..=0x1BC => self.write_icpr(((offset - 0x180) / 4) as usize, value),
            // IPR0-IPR123
            0x300..=0x4EF => self.write_ipr(((offset - 0x300) / 4) as usize, value),
            // STIR
            0xE00 => self.write_stir(value),
            _ => {}
        }
    }
}

impl Default for Nvic {
    fn default() -> Self {
        Self::for_cortex_m4()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvic_enable_disable() {
        let mut nvic = Nvic::new(64, 8);

        assert!(!nvic.is_enabled(0));
        nvic.enable(0);
        assert!(nvic.is_enabled(0));
        nvic.disable(0);
        assert!(!nvic.is_enabled(0));

        // Test high interrupt number
        nvic.enable(63);
        assert!(nvic.is_enabled(63));
        assert!(!nvic.is_enabled(64)); // Out of range
    }

    #[test]
    fn test_nvic_pending() {
        let mut nvic = Nvic::new(64, 8);

        assert!(!nvic.is_pending(5));
        nvic.set_pending(5);
        assert!(nvic.is_pending(5));
        nvic.clear_pending(5);
        assert!(!nvic.is_pending(5));
    }

    #[test]
    fn test_nvic_priority() {
        let mut nvic = Nvic::new(64, 8);

        nvic.set_priority(0, 0x80);
        assert_eq!(nvic.get_priority(0), 0x80);

        nvic.set_priority(1, 0x40);
        assert_eq!(nvic.get_priority(1), 0x40);

        // Test priority masking (2 bits = only top 2 bits implemented)
        let mut nvic2 = Nvic::new(32, 2);
        nvic2.set_priority(0, 0xFF);
        assert_eq!(nvic2.get_priority(0), 0xC0); // Only top 2 bits
    }

    #[test]
    fn test_nvic_ipr_register() {
        let mut nvic = Nvic::new(64, 8);

        // Write 4 priorities at once
        nvic.write_ipr(0, 0x80_40_20_10);
        assert_eq!(nvic.get_priority(0), 0x10);
        assert_eq!(nvic.get_priority(1), 0x20);
        assert_eq!(nvic.get_priority(2), 0x40);
        assert_eq!(nvic.get_priority(3), 0x80);

        // Read back
        assert_eq!(nvic.read_ipr(0), 0x80_40_20_10);
    }

    #[test]
    fn test_nvic_pending_exception() {
        let mut nvic = Nvic::new(64, 8);

        // No pending exceptions initially
        assert!(nvic.get_pending_exception().is_none());

        // Set up interrupt 5 with priority 0x80, enabled and pending
        nvic.set_priority(5, 0x80);
        nvic.enable(5);
        nvic.set_pending(5);

        // Should find exception 21 (5 + 16)
        assert_eq!(nvic.get_pending_exception(), Some(21));

        // Add higher priority interrupt
        nvic.set_priority(10, 0x40);
        nvic.enable(10);
        nvic.set_pending(10);

        // Should find exception 26 (10 + 16) due to higher priority
        assert_eq!(nvic.get_pending_exception(), Some(26));
    }

    #[test]
    fn test_nvic_priority_masking() {
        let mut nvic = Nvic::new(64, 8);

        nvic.set_priority(0, 0x80);
        nvic.enable(0);
        nvic.set_pending(0);

        // Without masking, should be pending
        assert!(nvic.get_pending_exception().is_some());

        // With PRIMASK, configurable exceptions masked
        nvic.set_primask(true);
        assert!(nvic.get_pending_exception().is_none());
        nvic.set_primask(false);

        // With BASEPRI at 0x40, priority 0x80 is masked
        nvic.set_basepri(0x40);
        assert!(nvic.get_pending_exception().is_none());

        // Priority 0x20 should not be masked
        nvic.set_priority(0, 0x20);
        assert!(nvic.get_pending_exception().is_some());
    }

    #[test]
    fn test_nvic_system_priority() {
        let mut nvic = Nvic::new(64, 8);

        // Fixed priorities
        assert_eq!(nvic.get_system_priority(1), -3); // Reset
        assert_eq!(nvic.get_system_priority(2), -2); // NMI
        assert_eq!(nvic.get_system_priority(3), -1); // HardFault

        // Configurable priorities
        nvic.set_system_priority(11, 0x80); // SVCall
        assert_eq!(nvic.get_system_priority(11), 0x80);

        nvic.set_system_priority(15, 0x40); // SysTick
        assert_eq!(nvic.get_system_priority(15), 0x40);
    }

    #[test]
    fn test_nvic_exception_entry_return() {
        let mut nvic = Nvic::new(64, 8);

        assert_eq!(nvic.current_exception(), 0);

        // Enter exception 21 (IRQ 5)
        nvic.enter_exception(21);
        assert_eq!(nvic.current_exception(), 21);
        assert!(nvic.is_active(5));

        // Return from exception
        nvic.return_from_exception();
        assert_eq!(nvic.current_exception(), 0);
        assert!(!nvic.is_active(5));
    }

    #[test]
    fn test_nvic_register_access() {
        let mut nvic = Nvic::new(64, 8);

        // Write ISER0 to enable interrupts 0-31
        nvic.write(0x000, 0xFFFF_FFFF);
        assert_eq!(nvic.read(0x000), 0xFFFF_FFFF);
        assert!(nvic.is_enabled(0));
        assert!(nvic.is_enabled(31));

        // Write ICER0 to disable interrupt 0
        nvic.write(0x080, 0x0000_0001);
        assert!(!nvic.is_enabled(0));
        assert!(nvic.is_enabled(31));

        // Test STIR
        nvic.write(0xE00, 5);
        assert!(nvic.is_pending(5));
    }
}
