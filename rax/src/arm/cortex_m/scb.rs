//! System Control Block (SCB) for Cortex-M.
//!
//! The SCB provides system implementation information and control, including:
//! - CPU identification (CPUID)
//! - Exception and interrupt control
//! - System control and configuration
//! - Fault status and address
//! - Debug features
//!
//! Registers (offset from SCS base 0xE000E000):
//! - 0xD00: CPUID (CPU ID Base Register)
//! - 0xD04: ICSR (Interrupt Control and State Register)
//! - 0xD08: VTOR (Vector Table Offset Register)
//! - 0xD0C: AIRCR (Application Interrupt and Reset Control Register)
//! - 0xD10: SCR (System Control Register)
//! - 0xD14: CCR (Configuration and Control Register)
//! - 0xD18-0xD20: SHPR1-3 (System Handler Priority Registers)
//! - 0xD24: SHCSR (System Handler Control and State Register)
//! - 0xD28: CFSR (Configurable Fault Status Register)
//! - 0xD2C: HFSR (HardFault Status Register)
//! - 0xD30: DFSR (Debug Fault Status Register)
//! - 0xD34: MMFAR (MemManage Fault Address Register)
//! - 0xD38: BFAR (BusFault Address Register)
//! - 0xD3C: AFSR (Auxiliary Fault Status Register)

/// ICSR register bits.
pub mod icsr {
    /// Active exception number (bits 8:0).
    pub const VECTACTIVE_MASK: u32 = 0x1FF;
    /// Return to base (thread mode).
    pub const RETTOBASE: u32 = 1 << 11;
    /// Pending exception number (bits 20:12).
    pub const VECTPENDING_SHIFT: u32 = 12;
    pub const VECTPENDING_MASK: u32 = 0x1FF << 12;
    /// Interrupt pending flag.
    pub const ISRPENDING: u32 = 1 << 22;
    /// SysTick pending clear.
    pub const PENDSTCLR: u32 = 1 << 25;
    /// SysTick pending set.
    pub const PENDSTSET: u32 = 1 << 26;
    /// PendSV pending clear.
    pub const PENDSVCLR: u32 = 1 << 27;
    /// PendSV pending set.
    pub const PENDSVSET: u32 = 1 << 28;
    /// NMI pending set.
    pub const NMIPENDSET: u32 = 1 << 31;
}

/// AIRCR register bits.
pub mod aircr {
    /// Vector key for writes.
    pub const VECTKEY: u32 = 0x05FA << 16;
    /// Vector key mask.
    pub const VECTKEYSTAT: u32 = 0xFA05 << 16;
    /// Priority grouping (bits 10:8).
    pub const PRIGROUP_SHIFT: u32 = 8;
    pub const PRIGROUP_MASK: u32 = 0x7 << 8;
    /// System reset request.
    pub const SYSRESETREQ: u32 = 1 << 2;
    /// Clear active state (debug only).
    pub const VECTCLRACTIVE: u32 = 1 << 1;
    /// Local reset (debug only).
    pub const VECTRESET: u32 = 1 << 0;
    /// Endianness (read-only).
    pub const ENDIANNESS: u32 = 1 << 15;
}

/// SCR register bits.
pub mod scr {
    /// Sleep on exit from ISR.
    pub const SLEEPONEXIT: u32 = 1 << 1;
    /// Use deep sleep.
    pub const SLEEPDEEP: u32 = 1 << 2;
    /// Send event on pending.
    pub const SEVONPEND: u32 = 1 << 4;
}

/// CCR register bits.
pub mod ccr {
    /// Non-base thread enable.
    pub const NONBASETHRDENA: u32 = 1 << 0;
    /// User set pending.
    pub const USERSETMPEND: u32 = 1 << 1;
    /// Unaligned access trap.
    pub const UNALIGN_TRP: u32 = 1 << 3;
    /// Divide by zero trap.
    pub const DIV_0_TRP: u32 = 1 << 4;
    /// Bus fault on priority escalation.
    pub const BFHFNMIGN: u32 = 1 << 8;
    /// Stack 8-byte alignment.
    pub const STKALIGN: u32 = 1 << 9;
    /// Branch predictor enable (ARMv8-M).
    pub const BP: u32 = 1 << 18;
    /// Instruction cache enable (ARMv8-M).
    pub const IC: u32 = 1 << 17;
    /// Data cache enable (ARMv8-M).
    pub const DC: u32 = 1 << 16;
}

/// SHCSR register bits.
pub mod shcsr {
    /// MemManage fault active.
    pub const MEMFAULTACT: u32 = 1 << 0;
    /// BusFault active.
    pub const BUSFAULTACT: u32 = 1 << 1;
    /// HardFault active (ARMv8-M).
    pub const HARDFAULTACT: u32 = 1 << 2;
    /// UsageFault active.
    pub const USGFAULTACT: u32 = 1 << 3;
    /// SecureFault active (ARMv8-M).
    pub const SECUREFAULTACT: u32 = 1 << 4;
    /// NMI active (ARMv8-M).
    pub const NMIACT: u32 = 1 << 5;
    /// SVCall active.
    pub const SVCALLACT: u32 = 1 << 7;
    /// DebugMonitor active.
    pub const MONITORACT: u32 = 1 << 8;
    /// PendSV active.
    pub const PENDSVACT: u32 = 1 << 10;
    /// SysTick active.
    pub const SYSTICKACT: u32 = 1 << 11;
    /// UsageFault pending.
    pub const USGFAULTPENDED: u32 = 1 << 12;
    /// MemManage fault pending.
    pub const MEMFAULTPENDED: u32 = 1 << 13;
    /// BusFault pending.
    pub const BUSFAULTPENDED: u32 = 1 << 14;
    /// SVCall pending.
    pub const SVCALLPENDED: u32 = 1 << 15;
    /// MemManage fault enable.
    pub const MEMFAULTENA: u32 = 1 << 16;
    /// BusFault enable.
    pub const BUSFAULTENA: u32 = 1 << 17;
    /// UsageFault enable.
    pub const USGFAULTENA: u32 = 1 << 18;
    /// SecureFault enable (ARMv8-M).
    pub const SECUREFAULTENA: u32 = 1 << 19;
    /// SecureFault pending (ARMv8-M).
    pub const SECUREFAULTPENDED: u32 = 1 << 20;
    /// HardFault pending (ARMv8-M).
    pub const HARDFAULTPENDED: u32 = 1 << 21;
}

/// CFSR register bits (UsageFault, BusFault, MemManage combined).
pub mod cfsr {
    // MemManage Fault Status (bits 7:0)
    /// Instruction access violation.
    pub const IACCVIOL: u32 = 1 << 0;
    /// Data access violation.
    pub const DACCVIOL: u32 = 1 << 1;
    /// MemManage on unstacking.
    pub const MUNSTKERR: u32 = 1 << 3;
    /// MemManage on stacking.
    pub const MSTKERR: u32 = 1 << 4;
    /// MemManage during FP lazy stacking.
    pub const MLSPERR: u32 = 1 << 5;
    /// MMFAR valid.
    pub const MMARVALID: u32 = 1 << 7;

    // BusFault Status (bits 15:8)
    /// Instruction bus error.
    pub const IBUSERR: u32 = 1 << 8;
    /// Precise data bus error.
    pub const PRECISERR: u32 = 1 << 9;
    /// Imprecise data bus error.
    pub const IMPRECISERR: u32 = 1 << 10;
    /// BusFault on unstacking.
    pub const UNSTKERR: u32 = 1 << 11;
    /// BusFault on stacking.
    pub const STKERR: u32 = 1 << 12;
    /// BusFault during FP lazy stacking.
    pub const LSPERR: u32 = 1 << 13;
    /// BFAR valid.
    pub const BFARVALID: u32 = 1 << 15;

    // UsageFault Status (bits 31:16)
    /// Undefined instruction.
    pub const UNDEFINSTR: u32 = 1 << 16;
    /// Invalid state (e.g., invalid EPSR.T).
    pub const INVSTATE: u32 = 1 << 17;
    /// Invalid PC load.
    pub const INVPC: u32 = 1 << 18;
    /// No coprocessor.
    pub const NOCP: u32 = 1 << 19;
    /// Stack overflow (ARMv8-M).
    pub const STKOF: u32 = 1 << 20;
    /// Unaligned access.
    pub const UNALIGNED: u32 = 1 << 24;
    /// Divide by zero.
    pub const DIVBYZERO: u32 = 1 << 25;
}

/// HFSR register bits.
pub mod hfsr {
    /// Vector table hard fault.
    pub const VECTTBL: u32 = 1 << 1;
    /// Forced hard fault (escalation).
    pub const FORCED: u32 = 1 << 30;
    /// Debug event caused hard fault.
    pub const DEBUGEVT: u32 = 1 << 31;
}

/// DFSR register bits.
pub mod dfsr {
    /// Halt request.
    pub const HALTED: u32 = 1 << 0;
    /// Breakpoint.
    pub const BKPT: u32 = 1 << 1;
    /// DWT match.
    pub const DWTTRAP: u32 = 1 << 2;
    /// Vector catch.
    pub const VCATCH: u32 = 1 << 3;
    /// External debug request.
    pub const EXTERNAL: u32 = 1 << 4;
}

/// Cortex-M variant for CPUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CortexMVariant {
    CortexM0,
    CortexM0Plus,
    CortexM1,
    CortexM3,
    CortexM4,
    CortexM7,
    CortexM23,
    CortexM33,
    CortexM35P,
    CortexM55,
    CortexM85,
}

impl CortexMVariant {
    /// Get CPUID value for this variant.
    pub fn cpuid(&self) -> u32 {
        // Format: [31:24] Implementer, [23:20] Variant, [19:16] Architecture,
        //         [15:4] Part number, [3:0] Revision
        // Implementer: 0x41 = ARM
        // Architecture: 0xC = ARMv6-M, 0xF = ARMv7-M/ARMv8-M
        match self {
            CortexMVariant::CortexM0 => 0x410C_C200,
            CortexMVariant::CortexM0Plus => 0x410C_C601,
            CortexMVariant::CortexM1 => 0x410C_C210,
            CortexMVariant::CortexM3 => 0x411F_C231,
            CortexMVariant::CortexM4 => 0x410F_C240,
            CortexMVariant::CortexM7 => 0x411F_C270,
            CortexMVariant::CortexM23 => 0x410F_D200,
            CortexMVariant::CortexM33 => 0x410F_D210,
            CortexMVariant::CortexM35P => 0x410F_D310,
            CortexMVariant::CortexM55 => 0x410F_D220,
            CortexMVariant::CortexM85 => 0x410F_D230,
        }
    }

    /// Check if this variant has FPU.
    pub fn has_fpu(&self) -> bool {
        matches!(
            self,
            CortexMVariant::CortexM4
                | CortexMVariant::CortexM7
                | CortexMVariant::CortexM33
                | CortexMVariant::CortexM35P
                | CortexMVariant::CortexM55
                | CortexMVariant::CortexM85
        )
    }

    /// Check if this variant has DSP extension.
    pub fn has_dsp(&self) -> bool {
        matches!(
            self,
            CortexMVariant::CortexM4
                | CortexMVariant::CortexM7
                | CortexMVariant::CortexM33
                | CortexMVariant::CortexM35P
                | CortexMVariant::CortexM55
                | CortexMVariant::CortexM85
        )
    }

    /// Check if this is an ARMv8-M variant.
    pub fn is_v8m(&self) -> bool {
        matches!(
            self,
            CortexMVariant::CortexM23
                | CortexMVariant::CortexM33
                | CortexMVariant::CortexM35P
                | CortexMVariant::CortexM55
                | CortexMVariant::CortexM85
        )
    }

    /// Check if this variant has TrustZone.
    pub fn has_trustzone(&self) -> bool {
        self.is_v8m()
    }

    /// Check if this variant has MVE (Helium).
    pub fn has_mve(&self) -> bool {
        matches!(self, CortexMVariant::CortexM55 | CortexMVariant::CortexM85)
    }
}

/// System Control Block state.
#[derive(Clone, Debug)]
pub struct Scb {
    /// CPU variant.
    variant: CortexMVariant,
    /// CPUID register (read-only).
    cpuid: u32,
    /// Vector Table Offset Register.
    vtor: u32,
    /// Application Interrupt and Reset Control Register.
    aircr: u32,
    /// System Control Register.
    scr: u32,
    /// Configuration and Control Register.
    ccr: u32,
    /// System Handler Control and State Register.
    shcsr: u32,
    /// Configurable Fault Status Register.
    cfsr: u32,
    /// HardFault Status Register.
    hfsr: u32,
    /// Debug Fault Status Register.
    dfsr: u32,
    /// MemManage Fault Address Register.
    mmfar: u32,
    /// BusFault Address Register.
    bfar: u32,
    /// Auxiliary Fault Status Register.
    afsr: u32,
    /// NMI pending.
    nmi_pending: bool,
    /// PendSV pending.
    pendsv_pending: bool,
    /// SysTick pending.
    systick_pending: bool,
    /// Reset request.
    reset_request: bool,
}

impl Scb {
    /// Create a new SCB for the specified variant.
    pub fn new(variant: CortexMVariant) -> Self {
        let cpuid = variant.cpuid();

        // Default CCR value depends on variant
        let ccr = if variant.is_v8m() {
            ccr::STKALIGN | ccr::UNALIGN_TRP
        } else {
            ccr::STKALIGN
        };

        Self {
            variant,
            cpuid,
            vtor: 0,
            aircr: aircr::VECTKEYSTAT,
            scr: 0,
            ccr,
            shcsr: 0,
            cfsr: 0,
            hfsr: 0,
            dfsr: 0,
            mmfar: 0,
            bfar: 0,
            afsr: 0,
            nmi_pending: false,
            pendsv_pending: false,
            systick_pending: false,
            reset_request: false,
        }
    }

    /// Reset the SCB.
    pub fn reset(&mut self) {
        let variant = self.variant;
        *self = Self::new(variant);
    }

    /// Get the CPU variant.
    pub fn variant(&self) -> CortexMVariant {
        self.variant
    }

    // =========================================================================
    // Exception Pending State
    // =========================================================================

    /// Check if NMI is pending.
    pub fn is_nmi_pending(&self) -> bool {
        self.nmi_pending
    }

    /// Set NMI pending.
    pub fn set_nmi_pending(&mut self, pending: bool) {
        self.nmi_pending = pending;
    }

    /// Check if PendSV is pending.
    pub fn is_pendsv_pending(&self) -> bool {
        self.pendsv_pending
    }

    /// Set PendSV pending.
    pub fn set_pendsv_pending(&mut self, pending: bool) {
        self.pendsv_pending = pending;
    }

    /// Check if SysTick is pending.
    pub fn is_systick_pending(&self) -> bool {
        self.systick_pending
    }

    /// Set SysTick pending.
    pub fn set_systick_pending(&mut self, pending: bool) {
        self.systick_pending = pending;
    }

    /// Take reset request (returns true and clears if set).
    pub fn take_reset_request(&mut self) -> bool {
        let was_set = self.reset_request;
        self.reset_request = false;
        was_set
    }

    // =========================================================================
    // Fault Status
    // =========================================================================

    /// Get CFSR value.
    pub fn get_cfsr(&self) -> u32 {
        self.cfsr
    }

    /// Set CFSR bits (OR with existing).
    pub fn set_cfsr_bits(&mut self, bits: u32) {
        self.cfsr |= bits;
    }

    /// Clear CFSR bits (write 1 to clear).
    pub fn clear_cfsr_bits(&mut self, bits: u32) {
        self.cfsr &= !bits;
    }

    /// Set MemManage fault with address.
    pub fn set_memmanage_fault(&mut self, status: u32, address: Option<u32>) {
        self.cfsr |= status & 0xFF;
        if let Some(addr) = address {
            self.mmfar = addr;
            self.cfsr |= cfsr::MMARVALID;
        }
    }

    /// Set BusFault with address.
    pub fn set_bus_fault(&mut self, status: u32, address: Option<u32>) {
        self.cfsr |= (status << 8) & 0xFF00;
        if let Some(addr) = address {
            self.bfar = addr;
            self.cfsr |= cfsr::BFARVALID;
        }
    }

    /// Set UsageFault.
    pub fn set_usage_fault(&mut self, status: u32) {
        self.cfsr |= (status << 16) & 0xFFFF_0000;
    }

    /// Set HardFault status.
    pub fn set_hardfault(&mut self, status: u32) {
        self.hfsr |= status;
    }

    /// Clear HardFault status (write 1 to clear).
    pub fn clear_hardfault(&mut self, bits: u32) {
        self.hfsr &= !bits;
    }

    // =========================================================================
    // Fault Enables
    // =========================================================================

    /// Check if MemManage fault is enabled.
    pub fn is_memmanage_enabled(&self) -> bool {
        self.shcsr & shcsr::MEMFAULTENA != 0
    }

    /// Check if BusFault is enabled.
    pub fn is_busfault_enabled(&self) -> bool {
        self.shcsr & shcsr::BUSFAULTENA != 0
    }

    /// Check if UsageFault is enabled.
    pub fn is_usagefault_enabled(&self) -> bool {
        self.shcsr & shcsr::USGFAULTENA != 0
    }

    // =========================================================================
    // Configuration
    // =========================================================================

    /// Get vector table offset.
    pub fn vtor(&self) -> u32 {
        self.vtor
    }

    /// Set vector table offset.
    pub fn set_vtor(&mut self, value: u32) {
        // VTOR must be aligned to number of exceptions * 4
        // Minimum alignment is 128 bytes (32 exceptions)
        self.vtor = value & 0xFFFF_FF80;
    }

    /// Get priority grouping.
    pub fn priority_group(&self) -> u8 {
        ((self.aircr >> aircr::PRIGROUP_SHIFT) & 0x7) as u8
    }

    /// Check if divide by zero trap is enabled.
    pub fn div_by_zero_trap(&self) -> bool {
        self.ccr & ccr::DIV_0_TRP != 0
    }

    /// Check if unaligned access trap is enabled.
    pub fn unaligned_trap(&self) -> bool {
        self.ccr & ccr::UNALIGN_TRP != 0
    }

    /// Check if stack should be 8-byte aligned.
    pub fn stack_align(&self) -> bool {
        self.ccr & ccr::STKALIGN != 0
    }

    /// Check if sleep-on-exit is enabled.
    pub fn sleep_on_exit(&self) -> bool {
        self.scr & scr::SLEEPONEXIT != 0
    }

    /// Check if deep sleep is requested.
    pub fn sleep_deep(&self) -> bool {
        self.scr & scr::SLEEPDEEP != 0
    }

    // =========================================================================
    // Register Access
    // =========================================================================

    /// Read ICSR register.
    pub fn read_icsr(&self, current_exception: u16, pending_exception: Option<u16>) -> u32 {
        let mut icsr = current_exception as u32 & icsr::VECTACTIVE_MASK;

        if let Some(pending) = pending_exception {
            icsr |= (pending as u32) << icsr::VECTPENDING_SHIFT;
            icsr |= icsr::ISRPENDING;
        }

        if current_exception == 0 {
            icsr |= icsr::RETTOBASE;
        }

        if self.pendsv_pending {
            icsr |= icsr::PENDSVSET;
        }
        if self.systick_pending {
            icsr |= icsr::PENDSTSET;
        }
        if self.nmi_pending {
            icsr |= icsr::NMIPENDSET;
        }

        icsr
    }

    /// Write ICSR register.
    pub fn write_icsr(&mut self, value: u32) {
        // PendSV
        if value & icsr::PENDSVCLR != 0 {
            self.pendsv_pending = false;
        }
        if value & icsr::PENDSVSET != 0 {
            self.pendsv_pending = true;
        }

        // SysTick
        if value & icsr::PENDSTCLR != 0 {
            self.systick_pending = false;
        }
        if value & icsr::PENDSTSET != 0 {
            self.systick_pending = true;
        }

        // NMI (can only set, not clear)
        if value & icsr::NMIPENDSET != 0 {
            self.nmi_pending = true;
        }
    }

    /// Read AIRCR register.
    pub fn read_aircr(&self) -> u32 {
        let mut aircr = aircr::VECTKEYSTAT;
        aircr |= (self.aircr >> aircr::PRIGROUP_SHIFT) & 0x7;
        aircr <<= aircr::PRIGROUP_SHIFT;
        // Add endianness bit if big-endian (we're little-endian)
        aircr
    }

    /// Write AIRCR register.
    pub fn write_aircr(&mut self, value: u32) {
        // Check for correct key
        if value & 0xFFFF_0000 != aircr::VECTKEY {
            return;
        }

        // Priority grouping
        self.aircr = (self.aircr & !aircr::PRIGROUP_MASK) | (value & aircr::PRIGROUP_MASK);

        // System reset request
        if value & aircr::SYSRESETREQ != 0 {
            self.reset_request = true;
        }
    }

    /// Read from SCB register space (offset from 0xE000ED00).
    pub fn read(&self, offset: u32, current_exception: u16, pending_exception: Option<u16>) -> u32 {
        match offset {
            0x00 => self.cpuid,
            0x04 => self.read_icsr(current_exception, pending_exception),
            0x08 => self.vtor,
            0x0C => self.read_aircr(),
            0x10 => self.scr,
            0x14 => self.ccr,
            // SHPR1-3 are handled by NVIC
            0x24 => self.shcsr,
            0x28 => self.cfsr,
            0x2C => self.hfsr,
            0x30 => self.dfsr,
            0x34 => self.mmfar,
            0x38 => self.bfar,
            0x3C => self.afsr,
            _ => 0,
        }
    }

    /// Write to SCB register space.
    pub fn write(&mut self, offset: u32, value: u32) {
        match offset {
            0x00 => {} // CPUID is read-only
            0x04 => self.write_icsr(value),
            0x08 => self.set_vtor(value),
            0x0C => self.write_aircr(value),
            0x10 => self.scr = value & 0x16, // Only bits 1, 2, 4
            0x14 => {
                // CCR - some bits are read-only or implementation-defined
                let writable = ccr::NONBASETHRDENA
                    | ccr::USERSETMPEND
                    | ccr::UNALIGN_TRP
                    | ccr::DIV_0_TRP
                    | ccr::BFHFNMIGN
                    | ccr::STKALIGN;
                self.ccr = (self.ccr & !writable) | (value & writable);
            }
            // SHPR1-3 are handled by NVIC
            0x24 => {
                // SHCSR - only enable bits are writable
                let writable = shcsr::MEMFAULTENA | shcsr::BUSFAULTENA | shcsr::USGFAULTENA;
                self.shcsr = (self.shcsr & !writable) | (value & writable);
            }
            0x28 => self.clear_cfsr_bits(value), // Write 1 to clear
            0x2C => self.clear_hardfault(value), // Write 1 to clear
            0x30 => self.dfsr &= !value,         // Write 1 to clear
            0x34 => self.mmfar = value,
            0x38 => self.bfar = value,
            0x3C => {} // AFSR is implementation-defined
            _ => {}
        }
    }
}

impl Default for Scb {
    fn default() -> Self {
        Self::new(CortexMVariant::CortexM4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scb_cpuid() {
        let scb = Scb::new(CortexMVariant::CortexM4);
        assert_eq!(scb.read(0x00, 0, None), 0x410F_C240);

        let scb = Scb::new(CortexMVariant::CortexM0);
        assert_eq!(scb.read(0x00, 0, None), 0x410C_C200);

        let scb = Scb::new(CortexMVariant::CortexM33);
        assert_eq!(scb.read(0x00, 0, None), 0x410F_D210);
    }

    #[test]
    fn test_scb_vtor() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        scb.write(0x08, 0x0800_0000);
        assert_eq!(scb.vtor(), 0x0800_0000);

        // Should be aligned
        scb.write(0x08, 0x0800_0001);
        assert_eq!(scb.vtor(), 0x0800_0000);
    }

    #[test]
    fn test_scb_pendsv() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        assert!(!scb.is_pendsv_pending());

        // Set pending
        scb.write(0x04, icsr::PENDSVSET);
        assert!(scb.is_pendsv_pending());

        // Clear pending
        scb.write(0x04, icsr::PENDSVCLR);
        assert!(!scb.is_pendsv_pending());
    }

    #[test]
    fn test_scb_systick() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        scb.write(0x04, icsr::PENDSTSET);
        assert!(scb.is_systick_pending());

        scb.write(0x04, icsr::PENDSTCLR);
        assert!(!scb.is_systick_pending());
    }

    #[test]
    fn test_scb_aircr() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        // Try to write without key - should fail
        scb.write(0x0C, 0x0000_0300);
        assert_eq!(scb.priority_group(), 0);

        // Write with correct key
        scb.write(0x0C, aircr::VECTKEY | 0x0000_0300);
        assert_eq!(scb.priority_group(), 3);
    }

    #[test]
    fn test_scb_reset_request() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        assert!(!scb.take_reset_request());

        scb.write(0x0C, aircr::VECTKEY | aircr::SYSRESETREQ);
        assert!(scb.take_reset_request());
        assert!(!scb.take_reset_request()); // Should be cleared
    }

    #[test]
    fn test_scb_cfsr() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        // Set some fault bits
        scb.set_usage_fault(cfsr::UNDEFINSTR >> 16);
        assert_eq!(scb.get_cfsr() & cfsr::UNDEFINSTR, cfsr::UNDEFINSTR);

        // Clear by writing 1
        scb.write(0x28, cfsr::UNDEFINSTR);
        assert_eq!(scb.get_cfsr() & cfsr::UNDEFINSTR, 0);
    }

    #[test]
    fn test_scb_memmanage_fault() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        scb.set_memmanage_fault(cfsr::DACCVIOL, Some(0x2000_1234));
        assert!(scb.get_cfsr() & cfsr::DACCVIOL != 0);
        assert!(scb.get_cfsr() & cfsr::MMARVALID != 0);
        assert_eq!(scb.read(0x34, 0, None), 0x2000_1234); // MMFAR
    }

    #[test]
    fn test_scb_ccr() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        // Default should have STKALIGN
        assert!(scb.stack_align());

        // Enable div by zero trap
        scb.write(0x14, scb.ccr | ccr::DIV_0_TRP);
        assert!(scb.div_by_zero_trap());
    }

    #[test]
    fn test_scb_fault_enables() {
        let mut scb = Scb::new(CortexMVariant::CortexM4);

        assert!(!scb.is_memmanage_enabled());
        assert!(!scb.is_busfault_enabled());
        assert!(!scb.is_usagefault_enabled());

        scb.write(
            0x24,
            shcsr::MEMFAULTENA | shcsr::BUSFAULTENA | shcsr::USGFAULTENA,
        );
        assert!(scb.is_memmanage_enabled());
        assert!(scb.is_busfault_enabled());
        assert!(scb.is_usagefault_enabled());
    }

    #[test]
    fn test_cortex_m_variants() {
        assert!(CortexMVariant::CortexM4.has_fpu());
        assert!(!CortexMVariant::CortexM3.has_fpu());

        assert!(CortexMVariant::CortexM33.is_v8m());
        assert!(!CortexMVariant::CortexM4.is_v8m());

        assert!(CortexMVariant::CortexM55.has_mve());
        assert!(!CortexMVariant::CortexM33.has_mve());
    }
}
