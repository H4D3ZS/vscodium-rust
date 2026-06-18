//! Unified ARM CPU trait for polymorphic execution.
//!
//! This module defines the core `ArmCpu` trait that all ARM profile implementations
//! (Cortex-M, Cortex-R, Cortex-A, AArch64) must implement. This allows for:
//! - Polymorphic CPU handling in tests and VMM
//! - Consistent interface across all ARM variants
//! - Profile-specific optimizations while maintaining compatibility

use std::fmt::Debug;

use crate::arm::features::ArmFeatures;

/// ARM architecture version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArmVersion {
    /// ARMv6-M (Cortex-M0, M0+, M1)
    V6M,
    /// ARMv7-M (Cortex-M3)
    V7M,
    /// ARMv7E-M (Cortex-M4, M7)
    V7EM,
    /// ARMv8-M Baseline (Cortex-M23)
    V8MBaseline,
    /// ARMv8-M Mainline (Cortex-M33)
    V8MMainline,
    /// ARMv8.1-M (Cortex-M55, M85)
    V8_1M,
    /// ARMv7-R (Cortex-R4, R5, R7, R8)
    V7R,
    /// ARMv8-R (Cortex-R52, R82)
    V8R,
    /// ARMv7-A (Cortex-A5, A7, A8, A9, A15, A17)
    V7A,
    /// ARMv8.0-A (Cortex-A32, A35, A53, A55, A57, A72, A73)
    V8_0A,
    /// ARMv8.1-A
    V8_1A,
    /// ARMv8.2-A (Cortex-A55, A75, A76, A77, A78)
    V8_2A,
    /// ARMv8.3-A
    V8_3A,
    /// ARMv8.4-A (Neoverse N1)
    V8_4A,
    /// ARMv8.5-A
    V8_5A,
    /// ARMv8.6-A
    V8_6A,
    /// ARMv8.7-A
    V8_7A,
    /// ARMv8.8-A
    V8_8A,
    /// ARMv9.0-A (Cortex-A510, A710, A715, X2, X3)
    V9_0A,
    /// ARMv9.1-A
    V9_1A,
    /// ARMv9.2-A (Cortex-A520, A720, X4)
    V9_2A,
    /// ARMv9.3-A
    V9_3A,
    /// ARMv9.4-A
    V9_4A,
}

impl ArmVersion {
    /// Check if this version supports AArch64.
    pub fn supports_aarch64(&self) -> bool {
        matches!(
            self,
            ArmVersion::V8_0A
                | ArmVersion::V8_1A
                | ArmVersion::V8_2A
                | ArmVersion::V8_3A
                | ArmVersion::V8_4A
                | ArmVersion::V8_5A
                | ArmVersion::V8_6A
                | ArmVersion::V8_7A
                | ArmVersion::V8_8A
                | ArmVersion::V9_0A
                | ArmVersion::V9_1A
                | ArmVersion::V9_2A
                | ArmVersion::V9_3A
                | ArmVersion::V9_4A
                | ArmVersion::V8R
        )
    }

    /// Check if this is an M-profile version.
    pub fn is_m_profile(&self) -> bool {
        matches!(
            self,
            ArmVersion::V6M
                | ArmVersion::V7M
                | ArmVersion::V7EM
                | ArmVersion::V8MBaseline
                | ArmVersion::V8MMainline
                | ArmVersion::V8_1M
        )
    }

    /// Check if this is an R-profile version.
    pub fn is_r_profile(&self) -> bool {
        matches!(self, ArmVersion::V7R | ArmVersion::V8R)
    }

    /// Check if this is an A-profile version.
    pub fn is_a_profile(&self) -> bool {
        !self.is_m_profile() && !self.is_r_profile()
    }

    /// Check if this version supports TrustZone.
    pub fn supports_trustzone(&self) -> bool {
        matches!(
            self,
            ArmVersion::V8MBaseline
                | ArmVersion::V8MMainline
                | ArmVersion::V8_1M
                | ArmVersion::V8_0A
                | ArmVersion::V8_1A
                | ArmVersion::V8_2A
                | ArmVersion::V8_3A
                | ArmVersion::V8_4A
                | ArmVersion::V8_5A
                | ArmVersion::V8_6A
                | ArmVersion::V8_7A
                | ArmVersion::V8_8A
                | ArmVersion::V9_0A
                | ArmVersion::V9_1A
                | ArmVersion::V9_2A
                | ArmVersion::V9_3A
                | ArmVersion::V9_4A
        )
    }

    /// Check if this version supports SVE.
    pub fn supports_sve(&self) -> bool {
        matches!(
            self,
            ArmVersion::V8_2A
                | ArmVersion::V8_3A
                | ArmVersion::V8_4A
                | ArmVersion::V8_5A
                | ArmVersion::V8_6A
                | ArmVersion::V8_7A
                | ArmVersion::V8_8A
                | ArmVersion::V9_0A
                | ArmVersion::V9_1A
                | ArmVersion::V9_2A
                | ArmVersion::V9_3A
                | ArmVersion::V9_4A
        )
    }

    /// Check if this version has mandatory SVE2.
    pub fn has_mandatory_sve2(&self) -> bool {
        matches!(
            self,
            ArmVersion::V9_0A
                | ArmVersion::V9_1A
                | ArmVersion::V9_2A
                | ArmVersion::V9_3A
                | ArmVersion::V9_4A
        )
    }

    /// Check if MVE (Helium) is supported.
    pub fn supports_mve(&self) -> bool {
        matches!(self, ArmVersion::V8_1M)
    }
}

/// ARM processor profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmProfile {
    /// Application profile (Cortex-A series)
    A,
    /// Real-time profile (Cortex-R series)
    R,
    /// Microcontroller profile (Cortex-M series)
    M,
}

/// Processor state (PSTATE/CPSR abstraction).
#[derive(Clone, Copy, Debug, Default)]
pub struct ProcessorState {
    /// Negative flag
    pub n: bool,
    /// Zero flag
    pub z: bool,
    /// Carry flag
    pub c: bool,
    /// Overflow flag
    pub v: bool,
    /// Saturation flag (DSP)
    pub q: bool,
    /// Greater-than-or-equal flags (GE[3:0])
    pub ge: u8,
    /// Current exception level (0-3 for AArch64, mode for AArch32)
    pub el: u8,
    /// Stack pointer selection (0=SP_EL0, 1=SP_ELx)
    pub sp_sel: bool,
    /// Thumb state (AArch32)
    pub t: bool,
    /// IRQ mask
    pub i: bool,
    /// FIQ mask
    pub f: bool,
    /// Asynchronous abort mask
    pub a: bool,
    /// Debug mask (AArch64)
    pub d: bool,
    /// Endianness (0=little, 1=big)
    pub e: bool,
    /// IT block state (Thumb)
    pub it_state: u8,
    /// Mode bits (AArch32)
    pub mode: u8,
}

impl ProcessorState {
    /// Create from AArch32 CPSR value.
    pub fn from_cpsr(cpsr: u32) -> Self {
        let it_lo = ((cpsr >> 25) & 3) as u8;
        let it_hi = ((cpsr >> 10) & 0x3F) as u8;
        Self {
            n: (cpsr >> 31) != 0,
            z: ((cpsr >> 30) & 1) != 0,
            c: ((cpsr >> 29) & 1) != 0,
            v: ((cpsr >> 28) & 1) != 0,
            q: ((cpsr >> 27) & 1) != 0,
            ge: ((cpsr >> 16) & 0xF) as u8,
            e: ((cpsr >> 9) & 1) != 0,
            a: ((cpsr >> 8) & 1) != 0,
            i: ((cpsr >> 7) & 1) != 0,
            f: ((cpsr >> 6) & 1) != 0,
            t: ((cpsr >> 5) & 1) != 0,
            mode: (cpsr & 0x1F) as u8,
            it_state: (it_hi << 2) | it_lo,
            el: 0,
            sp_sel: false,
            d: false,
        }
    }

    /// Convert to AArch32 CPSR value.
    pub fn to_cpsr(&self) -> u32 {
        let mut cpsr = self.mode as u32;
        if self.t {
            cpsr |= 1 << 5;
        }
        if self.f {
            cpsr |= 1 << 6;
        }
        if self.i {
            cpsr |= 1 << 7;
        }
        if self.a {
            cpsr |= 1 << 8;
        }
        if self.e {
            cpsr |= 1 << 9;
        }
        cpsr |= ((self.it_state & 3) as u32) << 25;
        cpsr |= (((self.it_state >> 2) & 0x3F) as u32) << 10;
        cpsr |= (self.ge as u32) << 16;
        if self.q {
            cpsr |= 1 << 27;
        }
        if self.v {
            cpsr |= 1 << 28;
        }
        if self.c {
            cpsr |= 1 << 29;
        }
        if self.z {
            cpsr |= 1 << 30;
        }
        if self.n {
            cpsr |= 1 << 31;
        }
        cpsr
    }

    /// Create from AArch64 PSTATE value.
    pub fn from_pstate(pstate: u64) -> Self {
        Self {
            n: ((pstate >> 31) & 1) != 0,
            z: ((pstate >> 30) & 1) != 0,
            c: ((pstate >> 29) & 1) != 0,
            v: ((pstate >> 28) & 1) != 0,
            d: ((pstate >> 9) & 1) != 0,
            a: ((pstate >> 8) & 1) != 0,
            i: ((pstate >> 7) & 1) != 0,
            f: ((pstate >> 6) & 1) != 0,
            el: ((pstate >> 2) & 3) as u8,
            sp_sel: (pstate & 1) != 0,
            ..Default::default()
        }
    }

    /// Convert to AArch64 PSTATE value.
    pub fn to_pstate(&self) -> u64 {
        let mut pstate = 0u64;
        if self.sp_sel {
            pstate |= 1;
        }
        pstate |= (self.el as u64) << 2;
        if self.f {
            pstate |= 1 << 6;
        }
        if self.i {
            pstate |= 1 << 7;
        }
        if self.a {
            pstate |= 1 << 8;
        }
        if self.d {
            pstate |= 1 << 9;
        }
        if self.v {
            pstate |= 1 << 28;
        }
        if self.c {
            pstate |= 1 << 29;
        }
        if self.z {
            pstate |= 1 << 30;
        }
        if self.n {
            pstate |= 1 << 31;
        }
        pstate
    }
}

/// CPU exit reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CpuExit {
    /// Instruction executed normally, continue.
    Continue,
    /// CPU halted (HLT, WFI, WFE).
    Halt,
    /// Waiting for interrupt (WFI).
    Wfi,
    /// Waiting for event (WFE).
    Wfe,
    /// Supervisor call (SVC).
    Svc(u32),
    /// Hypervisor call (HVC).
    Hvc(u16),
    /// Secure monitor call (SMC).
    Smc(u16),
    /// Breakpoint hit (BKPT/BRK).
    Breakpoint(u32),
    /// Undefined instruction.
    Undefined(u32),
    /// Memory fault.
    MemoryFault(MemoryFaultInfo),
    /// External abort.
    ExternalAbort(u64),
    /// Debug event.
    Debug(DebugEvent),
    /// Interrupt pending.
    InterruptPending,
    /// Exception taken.
    ExceptionTaken(ArmException),
    /// Maximum instruction count reached.
    MaxInstructions,
    /// Shutdown requested.
    Shutdown,
}

/// Memory fault information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryFaultInfo {
    /// Faulting address.
    pub address: u64,
    /// Access type that caused the fault.
    pub access: AccessType,
    /// Fault type.
    pub fault_type: MemoryFaultType,
    /// Is this a stage 2 fault? (virtualization)
    pub stage2: bool,
}

/// Type of memory access.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessType {
    /// Instruction fetch.
    InstructionFetch,
    /// Data read.
    Read,
    /// Data write.
    Write,
    /// Atomic read-modify-write.
    Atomic,
}

/// Type of memory fault.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryFaultType {
    /// Address translation fault.
    Translation,
    /// Access flag fault.
    AccessFlag,
    /// Permission fault.
    Permission,
    /// Alignment fault.
    Alignment,
    /// External abort.
    External,
    /// Address size fault.
    AddressSize,
    /// TLB conflict.
    TlbConflict,
    /// Unsupported atomic operation.
    UnsupportedAtomic,
    /// Lockdown fault.
    Lockdown,
    /// Synchronous external abort.
    SyncExternal,
}

/// Debug event types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugEvent {
    /// Software breakpoint.
    SoftwareBreakpoint(u64),
    /// Hardware breakpoint.
    HardwareBreakpoint(u64),
    /// Watchpoint (address, is_write).
    Watchpoint(u64, bool),
    /// Single step.
    Step,
    /// Vector catch.
    VectorCatch,
    /// External debug request.
    ExternalDebug,
}

/// ARM exception types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArmException {
    // Cortex-M exceptions
    /// Reset
    Reset,
    /// Non-maskable interrupt
    Nmi,
    /// Hard fault
    HardFault,
    /// Memory management fault
    MemManage(u32),
    /// Bus fault
    BusFault(u32),
    /// Usage fault
    UsageFault(u32),
    /// Secure fault (ARMv8-M)
    SecureFault,
    /// Supervisor call
    SvCall(u32),
    /// Debug monitor
    DebugMonitor,
    /// PendSV
    PendSv,
    /// SysTick
    SysTick,
    /// External interrupt
    Irq(u16),

    // A-profile exceptions
    /// Synchronous exception (ESR value)
    Synchronous(u64),
    /// IRQ interrupt
    IrqA,
    /// FIQ interrupt
    FiqA,
    /// System error (SError)
    SError(u64),

    // AArch32 specific
    /// Undefined instruction
    UndefinedInstruction,
    /// Prefetch abort
    PrefetchAbort(u32),
    /// Data abort
    DataAbort(u32),
    /// Hypervisor trap
    HypTrap,
    /// Hypervisor call
    HypCall,

    // Virtual exceptions
    /// Virtual IRQ
    VIrq,
    /// Virtual FIQ
    VFiq,
    /// Virtual SError
    VSError,
}

impl ArmException {
    /// Get exception number for Cortex-M.
    pub fn cortex_m_exception_number(&self) -> Option<u16> {
        match self {
            ArmException::Reset => Some(1),
            ArmException::Nmi => Some(2),
            ArmException::HardFault => Some(3),
            ArmException::MemManage(_) => Some(4),
            ArmException::BusFault(_) => Some(5),
            ArmException::UsageFault(_) => Some(6),
            ArmException::SecureFault => Some(7),
            ArmException::SvCall(_) => Some(11),
            ArmException::DebugMonitor => Some(12),
            ArmException::PendSv => Some(14),
            ArmException::SysTick => Some(15),
            ArmException::Irq(n) => Some(16 + n),
            _ => None,
        }
    }

    /// Get exception priority (lower = higher priority) for Cortex-M.
    pub fn default_priority(&self) -> i16 {
        match self {
            ArmException::Reset => -3,
            ArmException::Nmi => -2,
            ArmException::HardFault => -1,
            _ => 0, // Configurable
        }
    }
}

/// Watchpoint kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchpointKind {
    /// Trigger on read.
    Read,
    /// Trigger on write.
    Write,
    /// Trigger on read or write.
    ReadWrite,
}

/// ARM CPU error types.
#[derive(Clone, Debug)]
pub enum ArmError {
    /// Undefined instruction.
    UndefinedInstruction(u32),
    /// Unimplemented feature.
    Unimplemented(String),
    /// Memory access error.
    MemoryError(MemoryFaultInfo),
    /// Invalid register access.
    InvalidRegister(u8),
    /// Invalid exception level.
    InvalidExceptionLevel(u8),
    /// Configuration error.
    Configuration(String),
    /// Internal error.
    Internal(String),
}

impl std::fmt::Display for ArmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmError::UndefinedInstruction(insn) => {
                write!(f, "undefined instruction: 0x{:08x}", insn)
            }
            ArmError::Unimplemented(msg) => write!(f, "unimplemented: {}", msg),
            ArmError::MemoryError(info) => write!(f, "memory error at 0x{:x}", info.address),
            ArmError::InvalidRegister(reg) => write!(f, "invalid register: {}", reg),
            ArmError::InvalidExceptionLevel(el) => write!(f, "invalid exception level: {}", el),
            ArmError::Configuration(msg) => write!(f, "configuration error: {}", msg),
            ArmError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for ArmError {}

/// Unified trait for all ARM CPU implementations.
///
/// This trait provides a consistent interface across all ARM profiles:
/// - Cortex-M (M0, M0+, M3, M4, M7, M23, M33, M55, M85)
/// - Cortex-R (R4, R5, R7, R8, R52, R82)
/// - Cortex-A / AArch64 (A32, A53, A55, A72, A76, X1, etc.)
pub trait ArmCpu: Send + Sync + Debug {
    /// Execute one instruction.
    ///
    /// Returns the exit reason which may indicate:
    /// - Continue: instruction executed normally
    /// - Halt: CPU halted (WFI/WFE/HLT)
    /// - Exception: exception needs to be taken
    /// - Fault: memory or other fault occurred
    fn step(&mut self) -> Result<CpuExit, ArmError>;

    /// Execute until an exit condition.
    ///
    /// Continues executing instructions until:
    /// - An exit-worthy condition occurs (halt, exception, fault)
    /// - Maximum instruction count is reached
    fn run(&mut self) -> Result<CpuExit, ArmError> {
        loop {
            match self.step()? {
                CpuExit::Continue => continue,
                exit => return Ok(exit),
            }
        }
    }

    /// Execute up to `max_instructions` instructions.
    fn run_for(&mut self, max_instructions: u64) -> Result<CpuExit, ArmError> {
        for _ in 0..max_instructions {
            match self.step()? {
                CpuExit::Continue => continue,
                exit => return Ok(exit),
            }
        }
        Ok(CpuExit::MaxInstructions)
    }

    /// Reset the CPU to initial state.
    fn reset(&mut self);

    // =========================================================================
    // General Purpose Registers
    // =========================================================================

    /// Get a general-purpose register value.
    ///
    /// For AArch32: reg 0-15 (R0-R15, where R15=PC)
    /// For AArch64: reg 0-30 (X0-X30), 31 is XZR/SP depending on context
    fn get_gpr(&self, reg: u8) -> u64;

    /// Set a general-purpose register value.
    fn set_gpr(&mut self, reg: u8, value: u64);

    /// Get the program counter.
    fn get_pc(&self) -> u64;

    /// Set the program counter.
    fn set_pc(&mut self, value: u64);

    /// Get the stack pointer.
    fn get_sp(&self) -> u64;

    /// Set the stack pointer.
    fn set_sp(&mut self, value: u64);

    /// Get the link register.
    fn get_lr(&self) -> u64;

    /// Set the link register.
    fn set_lr(&mut self, value: u64);

    // =========================================================================
    // Processor State
    // =========================================================================

    /// Get current processor state (flags, mode, etc).
    fn get_pstate(&self) -> ProcessorState;

    /// Set processor state.
    fn set_pstate(&mut self, state: ProcessorState);

    /// Check if processor is in privileged mode.
    fn is_privileged(&self) -> bool;

    /// Check if processor is in secure state (TrustZone).
    fn is_secure(&self) -> bool {
        false // Default for non-TrustZone implementations
    }

    /// Get current exception level (AArch64) or mode (AArch32).
    fn current_el(&self) -> u8;

    // =========================================================================
    // Memory Access
    // =========================================================================

    /// Read from memory.
    fn read_memory(&self, addr: u64, size: usize) -> Result<Vec<u8>, ArmError>;

    /// Write to memory.
    fn write_memory(&mut self, addr: u64, data: &[u8]) -> Result<(), ArmError>;

    /// Read a byte from memory.
    fn read_u8(&self, addr: u64) -> Result<u8, ArmError> {
        let data = self.read_memory(addr, 1)?;
        Ok(data[0])
    }

    /// Read a halfword from memory.
    fn read_u16(&self, addr: u64) -> Result<u16, ArmError> {
        let data = self.read_memory(addr, 2)?;
        Ok(u16::from_le_bytes([data[0], data[1]]))
    }

    /// Read a word from memory.
    fn read_u32(&self, addr: u64) -> Result<u32, ArmError> {
        let data = self.read_memory(addr, 4)?;
        Ok(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
    }

    /// Read a doubleword from memory.
    fn read_u64(&self, addr: u64) -> Result<u64, ArmError> {
        let data = self.read_memory(addr, 8)?;
        Ok(u64::from_le_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]))
    }

    /// Write a byte to memory.
    fn write_u8(&mut self, addr: u64, value: u8) -> Result<(), ArmError> {
        self.write_memory(addr, &[value])
    }

    /// Write a halfword to memory.
    fn write_u16(&mut self, addr: u64, value: u16) -> Result<(), ArmError> {
        self.write_memory(addr, &value.to_le_bytes())
    }

    /// Write a word to memory.
    fn write_u32(&mut self, addr: u64, value: u32) -> Result<(), ArmError> {
        self.write_memory(addr, &value.to_le_bytes())
    }

    /// Write a doubleword to memory.
    fn write_u64(&mut self, addr: u64, value: u64) -> Result<(), ArmError> {
        self.write_memory(addr, &value.to_le_bytes())
    }

    // =========================================================================
    // Architecture Information
    // =========================================================================

    /// Get the architecture version.
    fn arch_version(&self) -> ArmVersion;

    /// Get the processor profile.
    fn profile(&self) -> ArmProfile;

    /// Get enabled features.
    fn features(&self) -> ArmFeatures;

    /// Check if a feature is enabled.
    fn has_feature(&self, feature: ArmFeatures) -> bool {
        self.features().contains(feature)
    }

    // =========================================================================
    // Exception Handling
    // =========================================================================

    /// Get pending exceptions.
    fn pending_exceptions(&self) -> Vec<ArmException>;

    /// Inject an exception.
    fn inject_exception(&mut self, exception: ArmException) -> Result<(), ArmError>;

    /// Check if an exception is pending.
    fn has_pending_exception(&self) -> bool {
        !self.pending_exceptions().is_empty()
    }

    // =========================================================================
    // Debug Support
    // =========================================================================

    /// Set a software breakpoint at address.
    fn set_breakpoint(&mut self, addr: u64) -> Result<(), ArmError>;

    /// Clear a breakpoint at address.
    fn clear_breakpoint(&mut self, addr: u64) -> Result<(), ArmError>;

    /// Set a watchpoint.
    fn set_watchpoint(
        &mut self,
        addr: u64,
        size: usize,
        kind: WatchpointKind,
    ) -> Result<(), ArmError>;

    /// Clear a watchpoint.
    fn clear_watchpoint(&mut self, addr: u64) -> Result<(), ArmError>;

    /// Get instruction count since reset.
    fn instruction_count(&self) -> u64;

    /// Get cycle count (if available).
    fn cycle_count(&self) -> Option<u64> {
        None
    }

    // =========================================================================
    // SIMD/FPU State (Optional)
    // =========================================================================

    /// Check if FPU/SIMD is available.
    fn has_fpu(&self) -> bool {
        false
    }

    /// Get a SIMD/FP register (128-bit as two u64).
    fn get_simd_reg(&self, reg: u8) -> Option<(u64, u64)> {
        let _ = reg;
        None
    }

    /// Set a SIMD/FP register.
    fn set_simd_reg(&mut self, reg: u8, low: u64, high: u64) -> Result<(), ArmError> {
        let _ = (reg, low, high);
        Err(ArmError::Unimplemented("SIMD not supported".to_string()))
    }

    /// Get FPCR (Floating-Point Control Register).
    fn get_fpcr(&self) -> Option<u32> {
        None
    }

    /// Set FPCR.
    fn set_fpcr(&mut self, value: u32) -> Result<(), ArmError> {
        let _ = value;
        Err(ArmError::Unimplemented("FPCR not supported".to_string()))
    }

    /// Get FPSR (Floating-Point Status Register).
    fn get_fpsr(&self) -> Option<u32> {
        None
    }

    /// Set FPSR.
    fn set_fpsr(&mut self, value: u32) -> Result<(), ArmError> {
        let _ = value;
        Err(ArmError::Unimplemented("FPSR not supported".to_string()))
    }

    // =========================================================================
    // SVE State (Optional - AArch64 only)
    // =========================================================================

    /// Check if SVE is available.
    fn has_sve(&self) -> bool {
        false
    }

    /// Get SVE vector length in bits.
    fn sve_vl(&self) -> Option<u32> {
        None
    }

    /// Set SVE vector length in bits.
    fn set_sve_vl(&mut self, vl: u32) -> Result<(), ArmError> {
        let _ = vl;
        Err(ArmError::Unimplemented("SVE not supported".to_string()))
    }

    /// Get SVE Z register (up to 256 bytes).
    fn get_sve_z(&self, reg: u8) -> Option<Vec<u8>> {
        let _ = reg;
        None
    }

    /// Set SVE Z register.
    fn set_sve_z(&mut self, reg: u8, data: &[u8]) -> Result<(), ArmError> {
        let _ = (reg, data);
        Err(ArmError::Unimplemented("SVE not supported".to_string()))
    }

    /// Get SVE P register (up to 32 bytes).
    fn get_sve_p(&self, reg: u8) -> Option<Vec<u8>> {
        let _ = reg;
        None
    }

    /// Set SVE P register.
    fn set_sve_p(&mut self, reg: u8, data: &[u8]) -> Result<(), ArmError> {
        let _ = (reg, data);
        Err(ArmError::Unimplemented("SVE not supported".to_string()))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm_version_properties() {
        // M-profile
        assert!(ArmVersion::V6M.is_m_profile());
        assert!(ArmVersion::V7M.is_m_profile());
        assert!(ArmVersion::V7EM.is_m_profile());
        assert!(ArmVersion::V8MBaseline.is_m_profile());
        assert!(ArmVersion::V8MMainline.is_m_profile());
        assert!(ArmVersion::V8_1M.is_m_profile());

        // R-profile
        assert!(ArmVersion::V7R.is_r_profile());
        assert!(ArmVersion::V8R.is_r_profile());

        // A-profile
        assert!(ArmVersion::V7A.is_a_profile());
        assert!(ArmVersion::V8_0A.is_a_profile());
        assert!(ArmVersion::V9_0A.is_a_profile());
    }

    #[test]
    fn test_aarch64_support() {
        assert!(!ArmVersion::V7A.supports_aarch64());
        assert!(ArmVersion::V8_0A.supports_aarch64());
        assert!(ArmVersion::V9_0A.supports_aarch64());
        assert!(ArmVersion::V8R.supports_aarch64());
    }

    #[test]
    fn test_sve_support() {
        assert!(!ArmVersion::V8_0A.supports_sve());
        assert!(ArmVersion::V8_2A.supports_sve());
        assert!(ArmVersion::V9_0A.supports_sve());
        assert!(ArmVersion::V9_0A.has_mandatory_sve2());
    }

    #[test]
    fn test_mve_support() {
        assert!(!ArmVersion::V8MMainline.supports_mve());
        assert!(ArmVersion::V8_1M.supports_mve());
    }

    #[test]
    fn test_processor_state_cpsr_roundtrip() {
        let original = 0xF00000D3u32; // NZCV=1111, SVC mode, I+F masked
        let state = ProcessorState::from_cpsr(original);

        assert!(state.n);
        assert!(state.z);
        assert!(state.c);
        assert!(state.v);
        assert!(state.i);
        assert!(state.f);
        assert_eq!(state.mode, 0x13); // SVC

        let reconstructed = state.to_cpsr();
        assert_eq!(reconstructed, original);
    }

    #[test]
    fn test_processor_state_pstate_roundtrip() {
        let original = 0xE000_0205u64; // NZCV=1110, EL1h
        let state = ProcessorState::from_pstate(original);

        assert!(state.n);
        assert!(state.z);
        assert!(state.c);
        assert!(!state.v);
        assert_eq!(state.el, 1);
        assert!(state.sp_sel);

        let reconstructed = state.to_pstate();
        assert_eq!(reconstructed, original);
    }

    #[test]
    fn test_exception_numbers() {
        assert_eq!(ArmException::Reset.cortex_m_exception_number(), Some(1));
        assert_eq!(ArmException::Nmi.cortex_m_exception_number(), Some(2));
        assert_eq!(ArmException::HardFault.cortex_m_exception_number(), Some(3));
        assert_eq!(ArmException::SysTick.cortex_m_exception_number(), Some(15));
        assert_eq!(ArmException::Irq(0).cortex_m_exception_number(), Some(16));
        assert_eq!(ArmException::Irq(10).cortex_m_exception_number(), Some(26));
    }

    #[test]
    fn test_exception_priorities() {
        assert_eq!(ArmException::Reset.default_priority(), -3);
        assert_eq!(ArmException::Nmi.default_priority(), -2);
        assert_eq!(ArmException::HardFault.default_priority(), -1);
        assert_eq!(ArmException::SvCall(0).default_priority(), 0);
    }
}
