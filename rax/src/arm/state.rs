//! ARM execution state and PSTATE/CPSR/xPSR definitions.
//!
//! This module provides detailed bit field definitions for the processor
//! state registers across all ARM profiles.

// =============================================================================
// AArch64 PSTATE Definitions
// =============================================================================

/// PSTATE (Process State) bit definitions for AArch64.
///
/// PSTATE is not a single register but a collection of fields accessed
/// through special-purpose registers (NZCV, DAIF, CurrentEL, etc.).
pub mod pstate {
    // Condition flags (bits 31:28)
    /// Negative condition flag.
    pub const N: u64 = 1 << 31;
    /// Zero condition flag.
    pub const Z: u64 = 1 << 30;
    /// Carry condition flag.
    pub const C: u64 = 1 << 29;
    /// Overflow condition flag.
    pub const V: u64 = 1 << 28;

    /// Mask for all condition flags (NZCV).
    pub const NZCV_MASK: u64 = N | Z | C | V;

    // Exception mask bits (bits 9:6)
    /// Debug exception mask.
    pub const D: u64 = 1 << 9;
    /// SError (System Error) exception mask.
    pub const A: u64 = 1 << 8;
    /// IRQ exception mask.
    pub const I: u64 = 1 << 7;
    /// FIQ exception mask.
    pub const F: u64 = 1 << 6;

    /// Mask for all exception masks (DAIF).
    pub const DAIF_MASK: u64 = D | A | I | F;

    // Execution state and mode
    /// Software Step bit (for debug).
    pub const SS: u64 = 1 << 21;
    /// Illegal Execution state bit.
    pub const IL: u64 = 1 << 20;
    /// Execution state (0 = AArch64, 1 = AArch32). Read-only.
    pub const NRW: u64 = 1 << 4;
    /// Exception level mask (EL[1:0]).
    pub const EL_MASK: u64 = 0x3 << 2;
    /// Stack pointer select (0 = SP_EL0, 1 = SP_ELx).
    pub const SP: u64 = 1 << 0;

    // Security/Speculation control
    /// Privileged Access Never (ARMv8.1+).
    pub const PAN: u64 = 1 << 22;
    /// User Access Override (ARMv8.2+).
    pub const UAO: u64 = 1 << 23;
    /// Data Independent Timing (ARMv8.4+).
    pub const DIT: u64 = 1 << 24;
    /// Tag Check Override (ARMv8.5+ MTE).
    pub const TCO: u64 = 1 << 25;

    // Branch type for BTI (bits 11:10)
    /// Branch type mask (ARMv8.5+ BTI).
    pub const BTYPE_MASK: u64 = 0x3 << 10;
    /// Speculative Store Bypass Safe (ARMv8.0+).
    pub const SSBS: u64 = 1 << 12;

    // ARMv8.5+ additional
    /// All Exceptions masked (NMI, ARMv8.8+).
    pub const ALLINT: u64 = 1 << 13;

    // ARMv8.7+ additional
    /// Pointer authentication enabled for code.
    pub const PACM: u64 = 1 << 26;

    /// Extract exception level from PSTATE value.
    #[inline]
    pub const fn get_el(pstate: u64) -> u8 {
        ((pstate >> 2) & 0x3) as u8
    }

    /// Set exception level in PSTATE value.
    #[inline]
    pub const fn set_el(pstate: u64, el: u8) -> u64 {
        (pstate & !EL_MASK) | (((el & 0x3) as u64) << 2)
    }

    /// Check if using SP_ELx (vs SP_EL0).
    #[inline]
    pub const fn using_sp_elx(pstate: u64) -> bool {
        (pstate & SP) != 0
    }

    /// Get PSTATE suffix string (e.g., "EL1h", "EL0t").
    pub fn mode_string(pstate: u64) -> &'static str {
        let el = get_el(pstate);
        let sp = using_sp_elx(pstate);
        match (el, sp) {
            (0, _) => "EL0t",
            (1, false) => "EL1t",
            (1, true) => "EL1h",
            (2, false) => "EL2t",
            (2, true) => "EL2h",
            (3, false) => "EL3t",
            (3, true) => "EL3h",
            _ => "???",
        }
    }
}

// =============================================================================
// AArch32 CPSR Definitions
// =============================================================================

/// CPSR (Current Program Status Register) bit definitions for AArch32.
pub mod cpsr {
    // Condition flags (bits 31:27)
    /// Negative condition flag.
    pub const N: u32 = 1 << 31;
    /// Zero condition flag.
    pub const Z: u32 = 1 << 30;
    /// Carry condition flag.
    pub const C: u32 = 1 << 29;
    /// Overflow condition flag.
    pub const V: u32 = 1 << 28;
    /// Saturation flag (sticky, for DSP instructions).
    pub const Q: u32 = 1 << 27;

    /// Mask for condition flags (NZCV).
    pub const NZCV_MASK: u32 = N | Z | C | V;

    // IT block state (Thumb-2, bits 26:25 and 15:10)
    /// IT state mask (IT[7:2] in bits 15:10, IT[1:0] in bits 26:25).
    pub const IT_MASK: u32 = 0x0600_FC00;

    // SIMD/DSP flags (bits 19:16)
    /// Greater-than-or-equal flags for SIMD byte operations.
    pub const GE_MASK: u32 = 0xF << 16;

    // Endianness (bit 9)
    /// Endianness state (0 = little, 1 = big).
    pub const E: u32 = 1 << 9;

    // Exception mask bits (bits 8:6)
    /// Asynchronous abort mask.
    pub const A: u32 = 1 << 8;
    /// IRQ mask.
    pub const I: u32 = 1 << 7;
    /// FIQ mask.
    pub const F: u32 = 1 << 6;

    // Execution state (bit 5)
    /// Thumb state (0 = ARM, 1 = Thumb).
    pub const T: u32 = 1 << 5;

    // Mode bits (bits 4:0)
    /// Mode mask.
    pub const MODE_MASK: u32 = 0x1F;

    // Processor modes
    /// User mode.
    pub const MODE_USR: u32 = 0b10000;
    /// FIQ mode.
    pub const MODE_FIQ: u32 = 0b10001;
    /// IRQ mode.
    pub const MODE_IRQ: u32 = 0b10010;
    /// Supervisor mode.
    pub const MODE_SVC: u32 = 0b10011;
    /// Monitor mode (TrustZone).
    pub const MODE_MON: u32 = 0b10110;
    /// Abort mode.
    pub const MODE_ABT: u32 = 0b10111;
    /// Hypervisor mode (ARMv7-A Virt).
    pub const MODE_HYP: u32 = 0b11010;
    /// Undefined mode.
    pub const MODE_UND: u32 = 0b11011;
    /// System mode.
    pub const MODE_SYS: u32 = 0b11111;

    /// Extract mode from CPSR value.
    #[inline]
    pub const fn get_mode(cpsr: u32) -> u32 {
        cpsr & MODE_MASK
    }

    /// Set mode in CPSR value.
    #[inline]
    pub const fn set_mode(cpsr: u32, mode: u32) -> u32 {
        (cpsr & !MODE_MASK) | (mode & MODE_MASK)
    }

    /// Check if in Thumb state.
    #[inline]
    pub const fn is_thumb(cpsr: u32) -> bool {
        (cpsr & T) != 0
    }

    /// Check if in privileged mode.
    #[inline]
    pub const fn is_privileged(cpsr: u32) -> bool {
        get_mode(cpsr) != MODE_USR
    }

    /// Get GE flags (bits 19:16).
    #[inline]
    pub const fn get_ge(cpsr: u32) -> u8 {
        ((cpsr >> 16) & 0xF) as u8
    }

    /// Set GE flags.
    #[inline]
    pub const fn set_ge(cpsr: u32, ge: u8) -> u32 {
        (cpsr & !GE_MASK) | (((ge & 0xF) as u32) << 16)
    }

    /// Get mode name string.
    pub fn mode_string(cpsr: u32) -> &'static str {
        match get_mode(cpsr) {
            MODE_USR => "USR",
            MODE_FIQ => "FIQ",
            MODE_IRQ => "IRQ",
            MODE_SVC => "SVC",
            MODE_MON => "MON",
            MODE_ABT => "ABT",
            MODE_HYP => "HYP",
            MODE_UND => "UND",
            MODE_SYS => "SYS",
            _ => "???",
        }
    }
}

// =============================================================================
// Cortex-M xPSR Definitions
// =============================================================================

/// xPSR (combined APSR/IPSR/EPSR) bit definitions for Cortex-M.
pub mod xpsr {
    // APSR - Application Program Status Register (condition flags)
    /// Negative flag.
    pub const N: u32 = 1 << 31;
    /// Zero flag.
    pub const Z: u32 = 1 << 30;
    /// Carry flag.
    pub const C: u32 = 1 << 29;
    /// Overflow flag.
    pub const V: u32 = 1 << 28;
    /// Saturation flag (DSP, ARMv7E-M+).
    pub const Q: u32 = 1 << 27;

    /// Mask for condition flags.
    pub const NZCV_MASK: u32 = N | Z | C | V;

    // GE flags (ARMv7E-M DSP extension)
    /// Greater-than-or-equal flags.
    pub const GE_MASK: u32 = 0xF << 16;

    // IPSR - Interrupt Program Status Register
    /// Exception number mask (bits 8:0).
    pub const EXCEPTION_MASK: u32 = 0x1FF;

    // EPSR - Execution Program Status Register
    /// Thumb bit (always 1 on Cortex-M).
    pub const T: u32 = 1 << 24;
    /// ICI/IT state mask (for interrupt-continuable and IT blocks).
    pub const ICI_IT_MASK: u32 = 0x0600_FC00;

    // Exception numbers
    /// Thread mode (no exception).
    pub const EXC_THREAD: u32 = 0;
    /// Reset.
    pub const EXC_RESET: u32 = 1;
    /// NMI.
    pub const EXC_NMI: u32 = 2;
    /// HardFault.
    pub const EXC_HARDFAULT: u32 = 3;
    /// MemManage fault (ARMv7-M+).
    pub const EXC_MEMMANAGE: u32 = 4;
    /// BusFault (ARMv7-M+).
    pub const EXC_BUSFAULT: u32 = 5;
    /// UsageFault (ARMv7-M+).
    pub const EXC_USAGEFAULT: u32 = 6;
    /// SecureFault (ARMv8-M+).
    pub const EXC_SECUREFAULT: u32 = 7;
    /// SVCall.
    pub const EXC_SVCALL: u32 = 11;
    /// Debug Monitor (ARMv7-M+).
    pub const EXC_DEBUGMON: u32 = 12;
    /// PendSV.
    pub const EXC_PENDSV: u32 = 14;
    /// SysTick.
    pub const EXC_SYSTICK: u32 = 15;
    /// First external IRQ.
    pub const EXC_IRQ0: u32 = 16;

    /// Get exception number from xPSR.
    #[inline]
    pub const fn get_exception(xpsr: u32) -> u32 {
        xpsr & EXCEPTION_MASK
    }

    /// Check if in handler mode.
    #[inline]
    pub const fn in_handler_mode(xpsr: u32) -> bool {
        get_exception(xpsr) != 0
    }

    /// Check if in thread mode.
    #[inline]
    pub const fn in_thread_mode(xpsr: u32) -> bool {
        get_exception(xpsr) == 0
    }

    /// Get exception name.
    pub fn exception_name(exc: u32) -> &'static str {
        match exc {
            EXC_THREAD => "Thread",
            EXC_RESET => "Reset",
            EXC_NMI => "NMI",
            EXC_HARDFAULT => "HardFault",
            EXC_MEMMANAGE => "MemManage",
            EXC_BUSFAULT => "BusFault",
            EXC_USAGEFAULT => "UsageFault",
            EXC_SECUREFAULT => "SecureFault",
            EXC_SVCALL => "SVCall",
            EXC_DEBUGMON => "DebugMon",
            EXC_PENDSV => "PendSV",
            EXC_SYSTICK => "SysTick",
            n if n >= EXC_IRQ0 => "IRQ",
            _ => "Reserved",
        }
    }
}

/// CONTROL register bits for Cortex-M.
pub mod control {
    /// Thread mode privilege level (0 = privileged, 1 = unprivileged).
    pub const NPRIV: u32 = 1 << 0;
    /// Stack pointer selection (0 = MSP, 1 = PSP).
    pub const SPSEL: u32 = 1 << 1;
    /// Floating-point context active (ARMv7-M with FPU).
    pub const FPCA: u32 = 1 << 2;
    /// Secure Floating-point context active (ARMv8-M with FPU).
    pub const SFPA: u32 = 1 << 3;

    /// Check if using PSP.
    #[inline]
    pub const fn using_psp(control: u32) -> bool {
        (control & SPSEL) != 0
    }

    /// Check if privileged in thread mode.
    #[inline]
    pub const fn is_privileged(control: u32) -> bool {
        (control & NPRIV) == 0
    }
}

/// EXC_RETURN values for Cortex-M exception return.
pub mod exc_return {
    /// Return to Handler mode, use MSP.
    pub const HANDLER_MSP: u32 = 0xFFFF_FFF1;
    /// Return to Thread mode, use MSP.
    pub const THREAD_MSP: u32 = 0xFFFF_FFF9;
    /// Return to Thread mode, use PSP.
    pub const THREAD_PSP: u32 = 0xFFFF_FFFD;

    // With FPU (ARMv7-M+)
    /// Return to Handler mode, use MSP, extended frame.
    pub const HANDLER_MSP_FP: u32 = 0xFFFF_FFE1;
    /// Return to Thread mode, use MSP, extended frame.
    pub const THREAD_MSP_FP: u32 = 0xFFFF_FFE9;
    /// Return to Thread mode, use PSP, extended frame.
    pub const THREAD_PSP_FP: u32 = 0xFFFF_FFED;

    // ARMv8-M Secure/Non-secure
    /// Return to Secure Handler mode, use MSP_S.
    pub const S_HANDLER_MSP: u32 = 0xFFFF_FFF1;
    /// Return to Secure Thread mode, use MSP_S.
    pub const S_THREAD_MSP: u32 = 0xFFFF_FFF9;
    /// Return to Secure Thread mode, use PSP_S.
    pub const S_THREAD_PSP: u32 = 0xFFFF_FFFD;
    /// Return to Non-secure Handler mode, use MSP_NS.
    pub const NS_HANDLER_MSP: u32 = 0xFFFF_FFB1;
    /// Return to Non-secure Thread mode, use MSP_NS.
    pub const NS_THREAD_MSP: u32 = 0xFFFF_FFB9;
    /// Return to Non-secure Thread mode, use PSP_NS.
    pub const NS_THREAD_PSP: u32 = 0xFFFF_FFBD;

    /// Check if EXC_RETURN value.
    #[inline]
    pub const fn is_exc_return(value: u32) -> bool {
        (value & 0xFFFF_FF00) == 0xFFFF_FF00
    }

    /// Check if returning to thread mode.
    #[inline]
    pub const fn is_thread_mode(value: u32) -> bool {
        (value & 0x8) != 0
    }

    /// Check if using PSP.
    #[inline]
    pub const fn is_psp(value: u32) -> bool {
        (value & 0x4) != 0
    }

    /// Check if extended frame (FPU state saved).
    #[inline]
    pub const fn is_extended_frame(value: u32) -> bool {
        (value & 0x10) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pstate_el() {
        let p = pstate::set_el(0, 1);
        assert_eq!(pstate::get_el(p), 1);

        let p = pstate::set_el(p, 3);
        assert_eq!(pstate::get_el(p), 3);
    }

    #[test]
    fn cpsr_mode() {
        let c = cpsr::set_mode(0, cpsr::MODE_SVC);
        assert_eq!(cpsr::get_mode(c), cpsr::MODE_SVC);
        assert!(cpsr::is_privileged(c));

        let c = cpsr::set_mode(c, cpsr::MODE_USR);
        assert!(!cpsr::is_privileged(c));
    }

    #[test]
    fn xpsr_exception() {
        let x = 0x0100_0000 | xpsr::EXC_HARDFAULT;
        assert!(xpsr::in_handler_mode(x));
        assert_eq!(xpsr::get_exception(x), xpsr::EXC_HARDFAULT);

        let x = 0x0100_0000; // Thread mode
        assert!(xpsr::in_thread_mode(x));
    }

    #[test]
    fn exc_return_check() {
        assert!(exc_return::is_exc_return(exc_return::THREAD_PSP));
        assert!(exc_return::is_thread_mode(exc_return::THREAD_PSP));
        assert!(exc_return::is_psp(exc_return::THREAD_PSP));
        assert!(!exc_return::is_psp(exc_return::THREAD_MSP));
    }
}
