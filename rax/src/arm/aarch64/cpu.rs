//! AArch64 CPU Implementation
//!
//! This module implements a complete AArch64 CPU emulator supporting:
//! - All exception levels (EL0-EL3)
//! - Full system register set
//! - MMU with page table walks
//! - GIC interrupt controller
//! - All ARMv8/v9 instruction categories

use std::collections::HashSet;
use std::fmt::Debug;

use super::exceptions::{
    ExceptionType, SyndromeRegister, build_spsr, exception_target_el, parse_spsr, vector_offset,
};
use super::gic::{Gic, GicConfig};
use super::mmu::{Mmu, MmuConfig, TranslationFault, TranslationGranule};
use super::sysregs::SystemRegisters;
use super::{NUM_ELS, NUM_GPRS, NUM_SIMD_REGS, sctlr};

use crate::arm::cpu_trait::{
    ArmCpu, ArmError, ArmException, ArmProfile, ArmVersion, CpuExit, MemoryFaultInfo,
    MemoryFaultType, ProcessorState, WatchpointKind,
};
use crate::arm::features::ArmFeatures;
use crate::arm::memory::ArmMemory;
use crate::arm::sysreg::Aarch64SysRegEncoding;
use crate::vphone_trace::{TraceEvent, record_runtime_event, runtime_trace_enabled};

/// When set, reads of system registers rax does not model return 0 (RAZ) and
/// writes are dropped (WI) instead of raising `Unimplemented`. OFF by default so
/// the differential oracle tests keep their strict behavior; the iOS/XNU boot
/// harness turns it on, because XNU touches a large set of Apple IMPLEMENTATION
/// DEFINED registers that no public reference models.
static LENIENT_SYSREGS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Enable/disable lenient (RAZ/WI) handling of unmodeled system registers.
pub fn set_lenient_sysregs(enabled: bool) {
    LENIENT_SYSREGS.store(enabled, std::sync::atomic::Ordering::Relaxed);
}

#[inline]
fn lenient_sysregs() -> bool {
    LENIENT_SYSREGS.load(std::sync::atomic::Ordering::Relaxed)
}

// =============================================================================
// CPU Configuration
// =============================================================================

/// AArch64 CPU configuration.
#[derive(Clone, Debug)]
pub struct AArch64Config {
    /// Architecture version.
    pub version: ArmVersion,
    /// Enabled features.
    pub features: ArmFeatures,
    /// Initial exception level (1, 2, or 3).
    pub initial_el: u8,
    /// GIC configuration.
    pub gic_config: Option<GicConfig>,
    /// Number of breakpoint registers.
    pub num_breakpoints: u8,
    /// Number of watchpoint registers.
    pub num_watchpoints: u8,
}

impl Default for AArch64Config {
    fn default() -> Self {
        Self {
            version: ArmVersion::V8_0A,
            features: ArmFeatures::armv8_0_base(),
            initial_el: 1,
            gic_config: Some(GicConfig::default()),
            num_breakpoints: 6,
            num_watchpoints: 4,
        }
    }
}

impl AArch64Config {
    /// Create configuration for ARMv8.0-A.
    pub fn v8_0() -> Self {
        Self {
            version: ArmVersion::V8_0A,
            features: ArmFeatures::armv8_0_base(),
            ..Default::default()
        }
    }

    /// Create configuration for ARMv8.1-A.
    pub fn v8_1() -> Self {
        Self {
            version: ArmVersion::V8_1A,
            features: ArmFeatures::armv8_1_base(),
            ..Default::default()
        }
    }

    /// Create configuration for ARMv8.2-A.
    pub fn v8_2() -> Self {
        Self {
            version: ArmVersion::V8_2A,
            features: ArmFeatures::armv8_2_base(),
            ..Default::default()
        }
    }

    /// Create configuration for ARMv9.0-A.
    pub fn v9_0() -> Self {
        Self {
            version: ArmVersion::V9_0A,
            features: ArmFeatures::armv9_0_base(),
            ..Default::default()
        }
    }
}

// =============================================================================
// AArch64 CPU
// =============================================================================

/// AArch64 CPU emulator.
pub struct AArch64Cpu {
    // Note: Debug derived manually below due to Box<dyn ArmMemory>
    // =========================================================================
    // General Purpose Registers
    // =========================================================================
    /// X0-X30 (64-bit general purpose registers).
    x: [u64; NUM_GPRS],

    /// Stack pointers for each EL.
    sp_el: [u64; NUM_ELS],

    /// Program Counter.
    pc: u64,

    // =========================================================================
    // Processor State (PSTATE)
    // =========================================================================
    /// NZCV condition flags.
    nzcv: u8,

    /// DAIF interrupt masks (D, A, I, F).
    daif: u8,

    /// Current exception level (0-3).
    current_el: u8,

    /// SP selection (false = SP_EL0, true = SP_ELx).
    sp_sel: bool,

    /// PAN (Privileged Access Never).
    pan: bool,

    /// UAO (User Access Override).
    uao: bool,

    /// DIT (Data Independent Timing).
    dit: bool,

    /// SSBS (Speculative Store Bypass Safe).
    ssbs: bool,

    /// TCO (Tag Check Override).
    tco: bool,

    /// BTYPE (Branch Type for BTI).
    btype: u8,

    /// IL (Illegal execution state).
    il: bool,

    /// SS (Software Step).
    ss: bool,

    // =========================================================================
    // SIMD/FP Registers
    // =========================================================================
    /// V0-V31 (128-bit SIMD/FP registers).
    v: [u128; NUM_SIMD_REGS],

    /// Floating-point Control Register.
    fpcr: u32,

    /// Floating-point Status Register.
    fpsr: u32,

    // =========================================================================
    // SVE (Scalable Vector Extension)
    // =========================================================================
    /// SVE Vector Length in bits (must be multiple of 128, min 128, max 2048).
    /// For simplicity, we use VL=128 which makes Z registers equivalent to V registers.
    sve_vl: u16,

    /// SVE Predicate registers P0-P15.
    /// Each bit corresponds to one byte of the vector (VL/8 bits per predicate).
    /// For VL=128: 16 bits, VL=256: 32 bits, etc.
    /// We use u32 to support up to VL=256.
    sve_p: [u32; 16],

    /// First-fault register (FFR) - special predicate for first-fault loads.
    sve_ffr: u32,

    // =========================================================================
    // System Registers
    // =========================================================================
    /// All system registers.
    sysregs: SystemRegisters,

    // =========================================================================
    // MMU
    // =========================================================================
    /// Memory Management Unit.
    mmu: Mmu,

    // =========================================================================
    // GIC
    // =========================================================================
    /// Generic Interrupt Controller, shared with the memory bridge so that
    /// distributor/redistributor MMIO and the CPU's ICC system registers
    /// observe the same state.
    gic: Option<std::sync::Arc<std::sync::Mutex<Gic>>>,
    /// Lock-free mirror of this CPU's GIC IRQ line (published by the GIC on
    /// every state change); checked on the hot path of `step_system`.
    gic_irq_line: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// Cached generic-timer output levels mirrored into GIC PPIs (virt PPI 27,
    /// phys PPI 30); avoids locking the GIC when nothing changed.
    timer_levels: (bool, bool),
    /// Translation level of the most recent MMU fault, for abort syndromes.
    /// (Atomic only for interior mutability behind `&self`; the CPU is
    /// single-threaded.)
    last_fault_level: std::sync::atomic::AtomicU8,
    /// Remaining debug-log quota for delivered faults.
    fault_log_budget: u32,
    /// Ring buffer of recently executed PCs (boot debugging).
    pc_ring: [u64; 64],
    pc_ring_idx: usize,

    // =========================================================================
    // Memory
    // =========================================================================
    /// Physical memory.
    memory: Box<dyn ArmMemory>,

    // =========================================================================
    // Execution State
    // =========================================================================
    /// Instruction count.
    insn_count: u64,

    /// Cycle count.
    cycle_count: u64,

    /// CPU halted.
    halted: bool,

    /// Waiting for interrupt.
    wfi: bool,

    /// Waiting for event.
    wfe: bool,

    /// Event signaled.
    event_register: bool,

    /// Pending exceptions.
    pending_exceptions: Vec<ArmException>,

    // =========================================================================
    // Debug
    // =========================================================================
    /// Breakpoints (PC addresses).
    breakpoints: HashSet<u64>,

    /// Watchpoints (address, size, kind).
    watchpoints: Vec<(u64, usize, WatchpointKind)>,

    // =========================================================================
    // Configuration
    // =========================================================================
    /// CPU configuration.
    config: AArch64Config,
}

impl std::fmt::Debug for AArch64Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AArch64Cpu")
            .field("pc", &format_args!("0x{:016x}", self.pc))
            .field("current_el", &self.current_el)
            .field("sp_sel", &self.sp_sel)
            .field("nzcv", &format_args!("{:04b}", self.nzcv))
            .field("daif", &format_args!("{:04b}", self.daif))
            .field("insn_count", &self.insn_count)
            .field("halted", &self.halted)
            .finish_non_exhaustive()
    }
}

impl AArch64Cpu {
    /// Create a new AArch64 CPU.
    pub fn new(config: AArch64Config, memory: Box<dyn ArmMemory>) -> Self {
        let gic = config
            .gic_config
            .as_ref()
            .map(|gc| std::sync::Arc::new(std::sync::Mutex::new(Gic::new(gc.clone()))));
        let gic_irq_line = gic
            .as_ref()
            .and_then(|g| g.lock().ok().and_then(|g| g.irq_line(0)));

        Self {
            x: [0; NUM_GPRS],
            sp_el: [0; NUM_ELS],
            pc: 0,

            nzcv: 0,
            daif: 0xF, // All exceptions masked on reset
            current_el: config.initial_el,
            sp_sel: true, // Use SP_ELx on reset
            pan: false,
            uao: false,
            dit: false,
            ssbs: false,
            tco: false,
            btype: 0,
            il: false,
            ss: false,

            v: [0; NUM_SIMD_REGS],
            fpcr: 0,
            fpsr: 0,

            // SVE: Default VL=128 bits (16 bytes)
            sve_vl: 128,
            sve_p: [0; 16],
            sve_ffr: 0,

            sysregs: SystemRegisters::new(),
            mmu: Mmu::new(),
            gic,
            gic_irq_line,
            timer_levels: (false, false),
            last_fault_level: std::sync::atomic::AtomicU8::new(0),
            fault_log_budget: 64,
            pc_ring: [0; 64],
            pc_ring_idx: 0,
            memory,

            insn_count: 0,
            cycle_count: 0,
            halted: false,
            wfi: false,
            wfe: false,
            event_register: false,
            pending_exceptions: Vec::new(),

            breakpoints: HashSet::new(),
            watchpoints: Vec::new(),

            config,
        }
    }

    /// Mutable access to the system register banks, for boot-time seeding of
    /// SCTLR_EL1 / VBAR_EL1 / TPIDR_EL1 etc. before the first instruction runs
    /// (e.g. the iOS XNU boot path, which hands the kernel a pre-initialized EL1
    /// state rather than running it through a reset vector).
    pub fn sysregs_mut(&mut self) -> &mut SystemRegisters {
        &mut self.sysregs
    }

    /// Read-only access to the system register banks.
    pub fn sysregs(&self) -> &SystemRegisters {
        &self.sysregs
    }

    // =========================================================================
    // Register Access
    // =========================================================================

    /// Get X register (X0-X30, or XZR if reg == 31).
    pub fn get_x(&self, reg: u8) -> u64 {
        if reg < 31 {
            self.x[reg as usize]
        } else {
            0 // XZR
        }
    }

    /// Set X register (X0-X30, write to XZR is ignored).
    pub fn set_x(&mut self, reg: u8, value: u64) {
        if reg < 31 {
            self.x[reg as usize] = value;
        }
    }

    /// Get W register (lower 32 bits of X).
    pub fn get_w(&self, reg: u8) -> u32 {
        self.get_x(reg) as u32
    }

    /// Set W register (zero-extends to X).
    pub fn set_w(&mut self, reg: u8, value: u32) {
        self.set_x(reg, value as u64);
    }

    /// Get current stack pointer.
    pub fn current_sp(&self) -> u64 {
        if self.sp_sel || self.current_el == 0 {
            if self.current_el == 0 {
                self.sp_el[0]
            } else {
                self.sp_el[self.current_el as usize]
            }
        } else {
            self.sp_el[0]
        }
    }

    /// Read a register encoded in the "Xn|SP" slot: index 31 selects the
    /// current stack pointer rather than XZR.
    fn gpr_or_sp(&self, reg: u8) -> u64 {
        if reg == 31 {
            self.current_sp()
        } else {
            self.get_x(reg)
        }
    }

    /// Write a register encoded in the "Xn|SP" slot.
    fn set_gpr_or_sp(&mut self, reg: u8, value: u64) {
        if reg == 31 {
            self.set_current_sp(value);
        } else {
            self.set_x(reg, value);
        }
    }

    /// Set current stack pointer.
    pub fn set_current_sp(&mut self, value: u64) {
        if self.sp_sel || self.current_el == 0 {
            if self.current_el == 0 {
                self.sp_el[0] = value;
            } else {
                self.sp_el[self.current_el as usize] = value;
            }
        } else {
            self.sp_el[0] = value;
        }
    }

    // =========================================================================
    // Flag Access
    // =========================================================================

    /// Get N flag.
    pub fn get_n(&self) -> bool {
        (self.nzcv >> 3) & 1 != 0
    }

    /// Get Z flag.
    pub fn get_z(&self) -> bool {
        (self.nzcv >> 2) & 1 != 0
    }

    /// Get C flag.
    pub fn get_c(&self) -> bool {
        (self.nzcv >> 1) & 1 != 0
    }

    /// Get V flag.
    pub fn get_v(&self) -> bool {
        self.nzcv & 1 != 0
    }

    /// Set N flag.
    pub fn set_n(&mut self, v: bool) {
        if v {
            self.nzcv |= 0x8;
        } else {
            self.nzcv &= !0x8;
        }
    }

    /// Set Z flag.
    pub fn set_z(&mut self, v: bool) {
        if v {
            self.nzcv |= 0x4;
        } else {
            self.nzcv &= !0x4;
        }
    }

    /// Set C flag.
    pub fn set_c(&mut self, v: bool) {
        if v {
            self.nzcv |= 0x2;
        } else {
            self.nzcv &= !0x2;
        }
    }

    /// Set V flag.
    pub fn set_v(&mut self, v: bool) {
        if v {
            self.nzcv |= 0x1;
        } else {
            self.nzcv &= !0x1;
        }
    }

    /// Set all NZCV flags.
    pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        self.nzcv = ((n as u8) << 3) | ((z as u8) << 2) | ((c as u8) << 1) | (v as u8);
    }

    /// Update N and Z flags based on result.
    pub fn update_nz_64(&mut self, result: u64) {
        self.set_n((result as i64) < 0);
        self.set_z(result == 0);
    }

    /// Update N and Z flags based on 32-bit result.
    pub fn update_nz_32(&mut self, result: u32) {
        self.set_n((result as i32) < 0);
        self.set_z(result == 0);
    }

    // =========================================================================
    // Condition Evaluation
    // =========================================================================

    /// Evaluate condition code.
    pub fn condition_holds(&self, cond: u8) -> bool {
        let result = match cond >> 1 {
            0b000 => self.get_z(),                                  // EQ/NE
            0b001 => self.get_c(),                                  // CS/CC
            0b010 => self.get_n(),                                  // MI/PL
            0b011 => self.get_v(),                                  // VS/VC
            0b100 => self.get_c() && !self.get_z(),                 // HI/LS
            0b101 => self.get_n() == self.get_v(),                  // GE/LT
            0b110 => self.get_n() == self.get_v() && !self.get_z(), // GT/LE
            0b111 => true,                                          // AL
            _ => unreachable!(),
        };

        if cond & 1 != 0 && cond != 0xF {
            !result
        } else {
            result
        }
    }

    // =========================================================================
    // Memory Access
    // =========================================================================

    /// Read byte from memory (with MMU translation).
    pub fn mem_read_u8(&self, va: u64) -> Result<u8, ArmError> {
        let pa = self.translate_address(va, false, false)?;
        self.memory.read_u8(pa).map_err(|e| e.into())
    }

    /// Read halfword from memory.
    pub fn mem_read_u16(&self, va: u64) -> Result<u16, ArmError> {
        let pa = self.translate_address(va, false, false)?;
        self.memory.read_u16(pa).map_err(|e| e.into())
    }

    /// Read word from memory.
    pub fn mem_read_u32(&self, va: u64) -> Result<u32, ArmError> {
        let pa = self.translate_address(va, false, false)?;
        self.memory.read_u32(pa).map_err(|e| e.into())
    }

    /// Read doubleword from memory.
    pub fn mem_read_u64(&self, va: u64) -> Result<u64, ArmError> {
        let pa = self.translate_address(va, false, false)?;
        self.memory.read_u64(pa).map_err(|e| e.into())
    }

    /// Write byte to memory.
    pub fn mem_write_u8(&mut self, va: u64, value: u8) -> Result<(), ArmError> {
        let pa = self.translate_address(va, true, false)?;
        self.memory.write_u8(pa, value).map_err(|e| e.into())
    }

    /// Write halfword to memory.
    pub fn mem_write_u16(&mut self, va: u64, value: u16) -> Result<(), ArmError> {
        let pa = self.translate_address(va, true, false)?;
        self.memory.write_u16(pa, value).map_err(|e| e.into())
    }

    /// Write word to memory.
    pub fn mem_write_u32(&mut self, va: u64, value: u32) -> Result<(), ArmError> {
        let pa = self.translate_address(va, true, false)?;
        self.memory.write_u32(pa, value).map_err(|e| e.into())
    }

    /// Write doubleword to memory.
    pub fn mem_write_u64(&mut self, va: u64, value: u64) -> Result<(), ArmError> {
        let pa = self.translate_address(va, true, false)?;
        self.memory.write_u64(pa, value).map_err(|e| e.into())
    }

    /// Translate virtual address to physical address.
    fn translate_address(
        &self,
        va: u64,
        is_write: bool,
        is_execute: bool,
    ) -> Result<u64, ArmError> {
        // Check alignment for execute
        if is_execute && (va & 3) != 0 {
            return Err(ArmError::MemoryError(MemoryFaultInfo {
                address: va,
                access: if is_write {
                    crate::arm::cpu_trait::AccessType::Write
                } else if is_execute {
                    crate::arm::cpu_trait::AccessType::InstructionFetch
                } else {
                    crate::arm::cpu_trait::AccessType::Read
                },
                fault_type: MemoryFaultType::Alignment,
                stage2: false,
            }));
        }

        // Use MMU if enabled
        let privileged = self.current_el > 0;
        match self.mmu.translate(
            va,
            self.memory.as_ref(),
            is_write,
            is_execute,
            privileged,
            self.current_el,
        ) {
            Ok(desc) => Ok(desc.pa),
            Err(fault) => Err(self.translation_fault_to_error(fault, is_write)),
        }
    }

    /// Convert translation fault to ArmError.
    fn translation_fault_to_error(&self, fault: TranslationFault, is_write: bool) -> ArmError {
        use super::mmu::TranslationFaultType;

        let fault_type = match fault.fault_type {
            TranslationFaultType::Translation => MemoryFaultType::Translation,
            TranslationFaultType::Permission => MemoryFaultType::Permission,
            TranslationFaultType::Alignment => MemoryFaultType::Alignment,
            TranslationFaultType::AccessFlag => MemoryFaultType::AccessFlag,
            TranslationFaultType::AddressSize => MemoryFaultType::AddressSize,
            TranslationFaultType::ExternalAbort => MemoryFaultType::External,
        };

        self.last_fault_level
            .store(fault.level, std::sync::atomic::Ordering::Relaxed);

        ArmError::MemoryError(MemoryFaultInfo {
            address: fault.va,
            access: if is_write {
                crate::arm::cpu_trait::AccessType::Write
            } else {
                crate::arm::cpu_trait::AccessType::Read
            },
            fault_type,
            stage2: fault.stage2,
        })
    }

    /// Shared handle to the GIC (for the platform memory bridge to service
    /// distributor/redistributor MMIO).
    pub fn gic_handle(&self) -> Option<std::sync::Arc<std::sync::Mutex<Gic>>> {
        self.gic.clone()
    }

    /// Recently executed PCs, oldest first (boot debugging).
    pub fn recent_pcs(&self) -> Vec<u64> {
        let mut out = Vec::with_capacity(self.pc_ring.len());
        for i in 0..self.pc_ring.len() {
            let pc = self.pc_ring[(self.pc_ring_idx + i) % self.pc_ring.len()];
            if pc != 0 {
                out.push(pc);
            }
        }
        out
    }

    // =========================================================================
    // Instruction Fetch and Execution
    // =========================================================================

    /// Fetch instruction at PC.
    fn fetch_instruction(&self) -> Result<u32, ArmError> {
        let pa = self.translate_address(self.pc, false, true)?;
        self.memory.read_u32(pa).map_err(|e| e.into())
    }

    /// Execute one instruction.
    fn execute_instruction(&mut self) -> Result<CpuExit, ArmError> {
        // Fetch instruction
        let insn = self.fetch_instruction()?;

        // Check breakpoint
        if self.breakpoints.contains(&self.pc) {
            return Ok(CpuExit::Breakpoint(self.pc as u32));
        }

        // Save PC and advance
        let old_pc = self.pc;
        self.pc = self.pc.wrapping_add(4);

        // Clear BTYPE (set by branches)
        let old_btype = self.btype;
        self.btype = 0;

        // Execute
        let result = self.decode_and_execute(insn);

        match result {
            Ok(exit) => {
                self.insn_count += 1;
                self.cycle_count += 1;
                Ok(exit)
            }
            Err(e) => {
                // Restore PC on error
                self.pc = old_pc;
                self.btype = old_btype;
                Err(e)
            }
        }
    }

    /// Decode and execute an instruction.
    fn decode_and_execute(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Top-level decode by bits [28:25]
        let op0 = (insn >> 25) & 0xF;

        match op0 {
            // Reserved
            0b0000 => Err(ArmError::UndefinedInstruction(insn)),

            // Unallocated
            0b0001 | 0b0011 => Err(ArmError::UndefinedInstruction(insn)),

            // SVE (Scalable Vector Extension)
            0b0010 => self.exec_sve(insn),

            // Data Processing - Immediate
            0b1000 | 0b1001 => self.exec_dp_imm(insn),

            // Branches, Exception Generating, System
            0b1010 | 0b1011 => self.exec_branch_system(insn),

            // Loads and Stores
            0b0100 | 0b0110 | 0b1100 | 0b1110 => self.exec_load_store(insn),

            // Data Processing - Register
            0b0101 | 0b1101 => self.exec_dp_reg(insn),

            // Data Processing - SIMD and FP
            0b0111 | 0b1111 => self.exec_simd_fp(insn),

            _ => Err(ArmError::UndefinedInstruction(insn)),
        }
    }

    // =========================================================================
    // Exception Handling
    // =========================================================================

    /// Take an exception.
    fn take_exception(
        &mut self,
        target_el: u8,
        exc_type: ExceptionType,
        syndrome: SyndromeRegister,
    ) -> Result<(), ArmError> {
        // Build SPSR from current state
        let saved_spsr = build_spsr(
            self.nzcv,
            self.daif,
            self.current_el,
            self.sp_sel,
            self.ssbs,
            self.pan,
            self.uao,
            self.dit,
            self.tco,
            self.btype,
            self.il,
            self.ss,
        );

        // Save state to target EL. Asynchronous exceptions (IRQ/FIQ) do not
        // report a syndrome; leave ESR untouched for them.
        self.sysregs.bank_mut(target_el).spsr = saved_spsr;
        self.sysregs.bank_mut(target_el).elr = self.pc;
        if !matches!(exc_type, ExceptionType::Irq | ExceptionType::Fiq) {
            self.sysregs.bank_mut(target_el).esr = syndrome.value;
        }

        // Calculate vector offset
        let offset = vector_offset(
            exc_type,
            self.current_el,
            target_el,
            true, // from AArch64
            self.sp_sel,
        );

        // Get VBAR
        let vbar = self.sysregs.bank(target_el).vbar;

        // Switch to target EL
        self.current_el = target_el;
        self.sp_sel = true; // Use SP_ELx
        self.daif = 0xF; // Mask all interrupts

        // Clear single-step
        self.ss = false;

        // Clear IL
        self.il = false;

        // Set BTYPE to 0
        self.btype = 0;

        // Branch to handler
        self.pc = vbar.wrapping_add(offset);

        Ok(())
    }

    /// Return from exception (ERET).
    fn exception_return(&mut self) -> Result<CpuExit, ArmError> {
        // Get saved state from current EL
        let spsr = self.sysregs.bank(self.current_el).spsr;
        let elr = self.sysregs.bank(self.current_el).elr;

        // Parse SPSR
        let (nzcv, daif, target_el, sp_sel, ssbs, pan, uao, dit, tco, btype, il, ss) =
            parse_spsr(spsr);

        // Check if return is valid
        if target_el > self.current_el {
            // Cannot return to higher EL
            return Err(ArmError::Internal("ERET to higher EL".to_string()));
        }

        // Restore state
        self.nzcv = nzcv;
        self.daif = daif;
        self.current_el = target_el;
        self.sp_sel = sp_sel;
        self.ssbs = ssbs;
        self.pan = pan;
        self.uao = uao;
        self.dit = dit;
        self.tco = tco;
        self.btype = btype;
        self.il = il;
        self.ss = ss;

        // Set PC
        self.pc = elr;

        Ok(CpuExit::Continue)
    }

    /// Check for pending interrupts.
    fn check_pending_interrupts(&mut self) -> Result<Option<CpuExit>, ArmError> {
        // Check GIC for pending interrupt
        if let Some(ref gic) = self.gic {
            let cpu_id = 0; // Assume single core for now

            if gic.lock().map(|g| g.pending_interrupt(cpu_id)).unwrap_or(false) {
                // Check if IRQ is masked
                let irq_masked = (self.daif & 0x2) != 0;

                if !irq_masked {
                    // Determine target EL
                    let target_el = exception_target_el(
                        ExceptionType::Irq,
                        self.current_el,
                        self.sysregs.hcr_el2,
                        self.sysregs.scr_el3,
                    );

                    return Ok(Some(CpuExit::InterruptPending));
                }
            }
        }

        // Check timer interrupts
        if self.sysregs.cntp_interrupt_pending() {
            let irq_masked = (self.daif & 0x2) != 0;
            if !irq_masked {
                return Ok(Some(CpuExit::InterruptPending));
            }
        }

        Ok(None)
    }

    // =========================================================================
    // System Register Access
    // =========================================================================

    /// Read system register.
    fn read_sysreg(&self, encoding: Aarch64SysRegEncoding) -> Result<u64, ArmError> {
        // Handle special cases first
        match (
            encoding.op0,
            encoding.op1,
            encoding.crn,
            encoding.crm,
            encoding.op2,
        ) {
            // NZCV
            (3, 3, 4, 2, 0) => {
                return Ok((self.nzcv as u64) << 28);
            }
            // DAIF
            (3, 3, 4, 2, 1) => {
                return Ok((self.daif as u64) << 6);
            }
            // CurrentEL
            (3, 0, 4, 2, 2) => {
                return Ok((self.current_el as u64) << 2);
            }
            // SPSel
            (3, 0, 4, 2, 0) => {
                return Ok(self.sp_sel as u64);
            }
            // SP_EL0
            (3, 0, 4, 1, 0) => {
                return Ok(self.sp_el[0]);
            }
            // SP_EL1
            (3, 4, 4, 1, 0) => {
                return Ok(self.sp_el[1]);
            }
            // SP_EL2
            (3, 6, 4, 1, 0) => {
                return Ok(self.sp_el[2]);
            }
            // FPCR
            (3, 3, 4, 4, 0) => {
                return Ok(self.fpcr as u64);
            }
            // FPSR
            (3, 3, 4, 4, 1) => {
                return Ok(self.fpsr as u64);
            }
            _ => {}
        }

        // GICv3 CPU interface (ICC_*)
        if let Some(value) = self.read_icc(encoding) {
            return Ok(value);
        }

        // Unallocated registers in the ID/cache-info space (op0=3, crn=0:
        // ID_*, AIDR, CCSIDR, ...) and the debug ID space (op0=2, crn=0:
        // MDCCSR, DBGDTR, ...) are RAZ by architecture; a booting kernel
        // reads the whole block.
        if (encoding.op0 == 3 || encoding.op0 == 2) && encoding.crn == 0 {
            return Ok(self
                .sysregs
                .read(encoding, self.current_el)
                .unwrap_or(0));
        }

        // Read from sysregs
        match self.sysregs.read(encoding, self.current_el) {
            Some(v) => Ok(v),
            None if lenient_sysregs() => Ok(0), // RAZ for unmodeled regs (iOS boot)
            None => Err(ArmError::Unimplemented(format!("System register {}", encoding))),
        }
    }

    /// Read a GICv3 CPU interface (ICC_*) register. Returns None when the
    /// encoding is not an implemented ICC register.
    fn read_icc(&self, encoding: Aarch64SysRegEncoding) -> Option<u64> {
        let gic = self.gic.as_ref()?;
        let enc = (
            encoding.op0,
            encoding.op1,
            encoding.crn,
            encoding.crm,
            encoding.op2,
        );
        let mut gic = gic.lock().ok()?;
        let value = match enc {
            // ICC_PMR_EL1
            (3, 0, 4, 6, 0) => gic.cpu(0)?.priority_mask as u64,
            // ICC_IAR0_EL1 / ICC_IAR1_EL1 (read acknowledges)
            (3, 0, 12, 8, 0) | (3, 0, 12, 12, 0) => gic.acknowledge(0) as u64,
            // ICC_HPPIR0_EL1 / ICC_HPPIR1_EL1
            (3, 0, 12, 8, 2) | (3, 0, 12, 12, 2) => gic.cpu(0)?.highest_pending_intid as u64,
            // ICC_BPR0_EL1
            (3, 0, 12, 8, 3) => gic.cpu(0)?.bpr0 as u64,
            // ICC_BPR1_EL1
            (3, 0, 12, 12, 3) => gic.cpu(0)?.bpr1 as u64,
            // ICC_AP0R / ICC_AP1R (active priorities: RAZ)
            (3, 0, 12, 8, 4..=7) | (3, 0, 12, 9, 0..=3) => 0,
            // ICC_RPR_EL1
            (3, 0, 12, 11, 3) => gic.cpu(0)?.running_priority as u64,
            // ICC_CTLR_EL1: PRIbits=7 (8 priority bits), IDbits=0 (16-bit)
            (3, 0, 12, 12, 4) => {
                let cpu = gic.cpu(0)?;
                (7 << 8) | (cpu.eoi_mode as u64) << 1
            }
            // ICC_SRE_EL1: system register interface enabled, locked on
            (3, 0, 12, 12, 5) => 0x7,
            // ICC_IGRPEN0_EL1
            (3, 0, 12, 12, 6) => gic.cpu(0)?.igrpen0,
            // ICC_IGRPEN1_EL1
            (3, 0, 12, 12, 7) => gic.cpu(0)?.igrpen1,
            _ => return None,
        };
        Some(value)
    }

    /// Write a GICv3 CPU interface (ICC_*) register. Returns true when the
    /// encoding was handled.
    fn write_icc(&mut self, encoding: Aarch64SysRegEncoding, value: u64) -> bool {
        let Some(gic) = self.gic.as_ref() else {
            return false;
        };
        let enc = (
            encoding.op0,
            encoding.op1,
            encoding.crn,
            encoding.crm,
            encoding.op2,
        );
        let Ok(mut gic) = gic.lock() else {
            return false;
        };
        match enc {
            // ICC_PMR_EL1
            (3, 0, 4, 6, 0) => gic.set_priority_mask(0, value as u8),
            // ICC_EOIR0_EL1 / ICC_EOIR1_EL1
            (3, 0, 12, 8, 1) | (3, 0, 12, 12, 1) => gic.end_of_interrupt(0, value as u32),
            // ICC_BPR0_EL1
            (3, 0, 12, 8, 3) => {
                if let Some(cpu) = gic.cpu_mut(0) {
                    cpu.bpr0 = (value & 0x7) as u8;
                }
            }
            // ICC_BPR1_EL1
            (3, 0, 12, 12, 3) => {
                if let Some(cpu) = gic.cpu_mut(0) {
                    cpu.bpr1 = (value & 0x7) as u8;
                }
            }
            // ICC_AP0R / ICC_AP1R: WI
            (3, 0, 12, 8, 4..=7) | (3, 0, 12, 9, 0..=3) => {}
            // ICC_DIR_EL1
            (3, 0, 12, 11, 1) => gic.deactivate(0, value as u32),
            // ICC_SGI1R_EL1 / ICC_ASGI1R_EL1 / ICC_SGI0R_EL1
            (3, 0, 12, 11, 5) | (3, 0, 12, 11, 6) | (3, 0, 12, 11, 7) => gic.raise_sgi(value),
            // ICC_CTLR_EL1: only EOImode is writable here
            (3, 0, 12, 12, 4) => {
                if let Some(cpu) = gic.cpu_mut(0) {
                    cpu.eoi_mode = (value >> 1) & 1 != 0;
                    cpu.ctlr_el1 = value;
                }
            }
            // ICC_SRE_EL1: WI (system register interface is always on)
            (3, 0, 12, 12, 5) => {}
            // ICC_IGRPEN0_EL1
            (3, 0, 12, 12, 6) => gic.set_group_enable(0, false, value & 1 != 0),
            // ICC_IGRPEN1_EL1
            (3, 0, 12, 12, 7) => gic.set_group_enable(0, true, value & 1 != 0),
            _ => return false,
        }
        true
    }

    /// Write system register.
    fn write_sysreg(
        &mut self,
        encoding: Aarch64SysRegEncoding,
        value: u64,
    ) -> Result<(), ArmError> {
        // Handle special cases first
        match (
            encoding.op0,
            encoding.op1,
            encoding.crn,
            encoding.crm,
            encoding.op2,
        ) {
            // NZCV
            (3, 3, 4, 2, 0) => {
                self.nzcv = ((value >> 28) & 0xF) as u8;
                return Ok(());
            }
            // DAIF
            (3, 3, 4, 2, 1) => {
                self.daif = ((value >> 6) & 0xF) as u8;
                return Ok(());
            }
            // SPSel
            (3, 0, 4, 2, 0) => {
                self.sp_sel = (value & 1) != 0;
                return Ok(());
            }
            // SP_EL0
            (3, 0, 4, 1, 0) => {
                self.sp_el[0] = value;
                return Ok(());
            }
            // SP_EL1
            (3, 4, 4, 1, 0) => {
                self.sp_el[1] = value;
                return Ok(());
            }
            // SP_EL2
            (3, 6, 4, 1, 0) => {
                self.sp_el[2] = value;
                return Ok(());
            }
            // FPCR
            (3, 3, 4, 4, 0) => {
                self.fpcr = value as u32;
                return Ok(());
            }
            // FPSR
            (3, 3, 4, 4, 1) => {
                self.fpsr = value as u32;
                return Ok(());
            }
            // SCTLR_ELx - update MMU config
            (3, 0, 1, 0, 0) | (3, 4, 1, 0, 0) | (3, 6, 1, 0, 0) => {
                let el = encoding.op1 / 2; // 0->EL1, 4->EL2, 6->EL3
                let el = if el == 0 { 1 } else { el };
                self.sysregs.bank_mut(el).sctlr = value;
                self.update_mmu_config();
                return Ok(());
            }
            // TCR_ELx - update MMU config
            (3, 0, 2, 0, 2) | (3, 4, 2, 0, 2) | (3, 6, 2, 0, 2) => {
                let el = encoding.op1 / 2;
                let el = if el == 0 { 1 } else { el };
                self.sysregs.bank_mut(el).tcr = value;
                self.update_mmu_config();
                return Ok(());
            }
            // TTBR0_ELx - update MMU config
            (3, 0, 2, 0, 0) | (3, 4, 2, 0, 0) | (3, 6, 2, 0, 0) => {
                let el = encoding.op1 / 2;
                let el = if el == 0 { 1 } else { el };
                self.sysregs.bank_mut(el).ttbr0 = value;
                self.update_mmu_config();
                return Ok(());
            }
            // TTBR1_EL1
            (3, 0, 2, 0, 1) => {
                self.sysregs.el1.ttbr1 = value;
                self.update_mmu_config();
                return Ok(());
            }
            // MAIR_ELx
            (3, 0, 10, 2, 0) | (3, 4, 10, 2, 0) | (3, 6, 10, 2, 0) => {
                let el = encoding.op1 / 2;
                let el = if el == 0 { 1 } else { el };
                self.sysregs.bank_mut(el).mair = value;
                self.update_mmu_config();
                return Ok(());
            }
            _ => {}
        }

        // GICv3 CPU interface (ICC_*)
        if self.write_icc(encoding, value) {
            return Ok(());
        }

        // Write to sysregs
        if self.sysregs.write(encoding, value, self.current_el) {
            Ok(())
        } else if lenient_sysregs() {
            Ok(()) // WI for unmodeled regs (iOS boot)
        } else {
            Err(ArmError::Unimplemented(format!(
                "System register write {}",
                encoding
            )))
        }
    }

    /// Update MMU configuration from system registers.
    fn update_mmu_config(&mut self) {
        let sctlr = self.sysregs.bank(self.current_el).sctlr;
        let tcr = self.sysregs.bank(self.current_el).tcr;
        let ttbr0 = self.sysregs.bank(self.current_el).ttbr0;
        let ttbr1 = if self.current_el == 1 {
            self.sysregs.el1.ttbr1
        } else {
            0
        };
        let mair = self.sysregs.bank(self.current_el).mair;

        let enabled = (sctlr & sctlr::M) != 0;
        let wxn = (sctlr & sctlr::WXN) != 0;

        let t0sz = (tcr & 0x3F) as u8;
        let t1sz = ((tcr >> 16) & 0x3F) as u8;
        let tg0 = ((tcr >> 14) & 0x3) as u8;
        let tg1 = ((tcr >> 30) & 0x3) as u8;

        let granule0 = TranslationGranule::from_tg0(tg0).unwrap_or(TranslationGranule::Granule4KB);
        let granule1 = TranslationGranule::from_tg1(tg1).unwrap_or(TranslationGranule::Granule4KB);

        self.mmu.set_config(MmuConfig {
            enabled,
            pa_size: 48,
            t0sz,
            t1sz,
            tg0: granule0,
            tg1: granule1,
            ttbr0,
            ttbr1,
            mair,
            wxn,
        });
    }

    // =========================================================================
    // Instruction Execution Stubs
    // =========================================================================

    /// Execute data processing (immediate) instruction.
    fn exec_dp_imm(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op0 = (insn >> 23) & 0x7;

        match op0 {
            0b000 | 0b001 => self.exec_pc_rel(insn),
            0b010 => self.exec_add_sub_imm(insn),
            0b011 => self.exec_add_sub_imm_tags(insn),
            0b100 => self.exec_logical_imm(insn),
            0b101 => self.exec_move_wide(insn),
            0b110 => self.exec_bitfield(insn),
            0b111 => self.exec_extract(insn),
            _ => Err(ArmError::UndefinedInstruction(insn)),
        }
    }

    /// Execute branch and system instruction.
    fn exec_branch_system(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Use bits [31:24] for primary decode
        let bits_31_24 = (insn >> 24) & 0xFF;

        // B.cond: bits[31:24] = 01010100 (0x54)
        if bits_31_24 == 0x54 {
            return self.exec_b_cond(insn);
        }

        // B, BL: bits[31:26] = 00010x or 10010x -> bits[31:24] starts with 000101 or 100101
        // Actually B: 000101, BL: 100101, so check bits[30:26] = 00101
        let bits_30_26 = (insn >> 26) & 0x1F;
        if bits_30_26 == 0b00101 {
            return self.exec_b_bl(insn);
        }

        // CBZ/CBNZ: bits[31:24] = x0110100 or x0110101 -> 0x34/0x35 or 0xB4/0xB5
        if bits_31_24 == 0x34 || bits_31_24 == 0x35 || bits_31_24 == 0xB4 || bits_31_24 == 0xB5 {
            return self.exec_cbz_cbnz(insn);
        }

        // TBZ/TBNZ: bits[31:24] = x0110110 or x0110111 -> 0x36/0x37 or 0xB6/0xB7
        if bits_31_24 == 0x36 || bits_31_24 == 0x37 || bits_31_24 == 0xB6 || bits_31_24 == 0xB7 {
            return self.exec_tbz_tbnz(insn);
        }

        // Exception generation: bits[31:24] = 0xD4
        if bits_31_24 == 0xD4 {
            return self.exec_exception_system(insn);
        }

        // System instructions: bits[31:22] = 1101010100 -> bits[31:24] = 0xD5 and bits[23:22] = 00
        if bits_31_24 == 0xD5 {
            let bits_23_22 = (insn >> 22) & 0x3;
            if bits_23_22 == 0 {
                return self.exec_exception_system(insn);
            }
        }

        // Unconditional branch (register): bits[31:25] = 1101011 -> bits[31:24] = 0xD6
        if bits_31_24 == 0xD6 {
            return self.exec_br_reg(insn);
        }

        Err(ArmError::UndefinedInstruction(insn))
    }

    /// Execute load/store instruction.
    fn exec_load_store(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // SIMD&FP register load/stores (V=1, bit 26) respect the CPACR FP
        // trap: lazy FP context switching depends on the first FP touch
        // (including vector loads in memcpy) trapping.
        if (insn >> 26) & 1 != 0 {
            let fpen = (self.sysregs.el1.cpacr >> 20) & 0x3;
            if (self.current_el == 0 && fpen != 0x3)
                || (self.current_el == 1 && (fpen & 1) == 0)
            {
                return self.take_fp_access_trap();
            }
        }

        // Advanced SIMD load/store multiple structures (LD1-4 / ST1-4).
        // bits[31]=0, bits[29:24] = 001100 (no-offset or post-index variant).
        if (insn >> 31) & 1 == 0 && (insn >> 24) & 0x3F == 0b001100 {
            return self.exec_ldst_structures(insn);
        }
        // Advanced SIMD load/store single structure (LD1-4 element, LD1R-LD4R).
        if (insn >> 31) & 1 == 0 && (insn >> 24) & 0x3F == 0b001101 {
            return self.exec_ldst_single(insn);
        }

        let op0 = (insn >> 28) & 0xF;
        let op1 = (insn >> 26) & 0x1;
        let bits_29_27 = (insn >> 27) & 0x7;
        let bit_24 = (insn >> 24) & 0x1;

        // Load/store exclusive: bits[29:27] = 00x, bit[24] = 0
        if bits_29_27 & 0b110 == 0b000 && bit_24 == 0 && op1 == 0 {
            return self.exec_ldst_exclusive(insn);
        }

        // FEAT_MTE tag load/stores: bits[31:24] = 0xD9. Without tag-capable
        // memory the tag side is a no-op, but the architected data side-
        // effects (granule zeroing, writeback, LDG's register write) are
        // honoured. The 16-byte tag granule is TG.
        if (insn >> 24) & 0xFF == 0xD9 {
            const TG: u64 = 16;
            let opc = (insn >> 22) & 0x3;
            let imm9 = (((insn >> 12) & 0x1FF) as i32) << 23 >> 23;
            let op2 = (insn >> 10) & 0x3;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rt = (insn & 0x1F) as u8;
            let base = self.gpr_or_sp(rn);
            let off = (imm9 as i64).wrapping_mul(TG as i64);

            // Bulk forms have imm9==0 and op2==00: STZGM (opc 00), LDG
            // (opc 01 op2 00 is LDG with any imm), STGM (10), LDGM (11).
            if op2 == 0b00 {
                match opc {
                    0b01 => {
                        // LDG Xt, [Xn, #imm]: load the allocation tag for the
                        // address into Xt's tag field. No tag memory: tag 0.
                        let v = self.get_x(rt) & !(0xFu64 << 56);
                        self.set_x(rt, v);
                        return Ok(CpuExit::Continue);
                    }
                    0b00 => {
                        // STZGM: zero the data of the naturally-aligned
                        // region (DCZID block size).
                        let block = 4u64 << (self.sysregs.dczid_el0 & 0xF);
                        let addr = base & !(block - 1);
                        for o in (0..block).step_by(8) {
                            self.mem_write_u64(addr + o, 0)?;
                        }
                        return Ok(CpuExit::Continue);
                    }
                    _ => {
                        // STGM (10): tag store only — no data effect.
                        // LDGM (11): bulk tag load — no tags, Xt = 0.
                        if opc == 0b11 {
                            self.set_x(rt, 0);
                        }
                        return Ok(CpuExit::Continue);
                    }
                }
            }

            // Indexed forms: op2 01=post-index, 10=signed-offset, 11=pre-index.
            let addr = if op2 == 0b01 {
                base
            } else {
                (base as i64).wrapping_add(off) as u64
            } & !(TG - 1);
            // STG (00) / ST2G (10): tag-only stores, no data effect.
            // STZG (01) / STZ2G (11): also zero the granule(s).
            if opc == 0b01 || opc == 0b11 {
                let granules = if opc == 0b11 { 2 } else { 1 };
                for g in 0..granules {
                    for o in (0..TG).step_by(8) {
                        self.mem_write_u64(addr + g * TG + o, 0)?;
                    }
                }
            }
            if op2 == 0b01 || op2 == 0b11 {
                let nb = (base as i64).wrapping_add(off) as u64;
                self.set_gpr_or_sp(rn, nb);
            }
            return Ok(CpuExit::Continue);
        }

        // Load register (literal), integer and SIMD&FP forms:
        // bits[29:27] = 01x, bits[25:24] = 00.
        if bits_29_27 & 0b110 == 0b010 && (insn >> 24) & 0x3 == 0 {
            return self.exec_ldr_literal(insn);
        }

        // Load/store pair: bits[29:27] = 10x (post-index, offset, pre-index)
        // bit[28] = 0 distinguishes pair from single register
        if bits_29_27 & 0b110 == 0b100 {
            return self.exec_ldst_pair(insn);
        }

        // Load/store single register: bits[29:27] = 11x
        if bits_29_27 & 0b110 == 0b110 {
            return self.exec_ldst_reg(insn);
        }

        // Fallback to single register for any remaining cases
        self.exec_ldst_reg(insn)
    }

    /// Execute data processing (register) instruction.
    fn exec_dp_reg(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op1 = (insn >> 28) & 0x1;
        let op2 = (insn >> 21) & 0xF;
        let _op3 = (insn >> 10) & 0x3F;

        if op1 == 0 {
            if (op2 & 0b1000) == 0 {
                // Logical (shifted register)
                return self.exec_logical_shifted(insn);
            } else {
                // Add/sub (shifted/extended register)
                return self.exec_add_sub_shifted_ext(insn);
            }
        } else {
            // op1 = 1
            match op2 {
                0b0000 => {
                    // Add/sub with carry
                    return self.exec_adc_sbc(insn);
                }
                0b0010 => {
                    // Conditional compare (register)
                    return self.exec_ccmp_ccmn(insn);
                }
                0b0100 => {
                    // Conditional select
                    return self.exec_csel(insn);
                }
                0b0110 => {
                    if ((insn >> 30) & 1) == 0 {
                        // Data processing (2 source)
                        return self.exec_dp_2src(insn);
                    }
                    // Data processing (1 source)
                    return self.exec_dp_1src(insn);
                }
                _ if (op2 & 0b1000) != 0 => {
                    // Data processing (3 source)
                    return self.exec_dp_3src(insn);
                }
                _ => {}
            }
        }

        Err(ArmError::UndefinedInstruction(insn))
    }

    /// Execute SIMD/FP instruction.
    fn exec_simd_fp(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Check if FP/SIMD is enabled
        let cpacr = self.sysregs.el1.cpacr;
        let fpen = (cpacr >> 20) & 0x3;

        if self.current_el == 0 && fpen != 0x3 {
            // FP/SIMD trapped at EL0
            return self.take_fp_access_trap();
        }
        if self.current_el == 1 && (fpen & 1) == 0 {
            // FP/SIMD trapped at EL1
            return self.take_fp_access_trap();
        }

        // Decode SIMD/FP instruction groups
        // Bits [28:25] = 0111 or 1111 for SIMD/FP
        // Bits [31:30] and [24:21] determine the specific group

        let op0 = (insn >> 28) & 0xF;
        let op1 = (insn >> 23) & 0x3;
        let op2 = (insn >> 19) & 0xF;
        let op3 = (insn >> 10) & 0x1FF;

        // Scalar FP data processing (three source): FMADD/FMSUB/FNMADD/FNMSUB.
        // bits[31:24] = 0001_1111
        if (insn >> 24) & 0xFF == 0b00011111 {
            let fp_type = (insn >> 22) & 0x3;
            let o1 = (insn >> 21) & 1;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let o0 = (insn >> 15) & 1;
            let ra = ((insn >> 10) & 0x1F) as usize;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            match fp_type {
                0b00 | 0b01 | 0b11 => {
                    // Route through the canonicalising fused multiply-add. ARM
                    // negates the product (FMSUB/FNMADD) and/or the addend
                    // (FNMADD/FNMSUB) before FPMulAdd, which also flips the
                    // propagated NaN sign.
                    let eb: u32 = match fp_type {
                        0b00 => 32,
                        0b01 => 64,
                        _ => 16,
                    };
                    let m_mask: u64 = if eb == 64 { u64::MAX } else { (1u64 << eb) - 1 };
                    let n = self.v[rn] as u64 & m_mask;
                    let m = self.v[rm] as u64 & m_mask;
                    let a = self.v[ra] as u64 & m_mask;
                    let (nn, aa) = match (o1, o0) {
                        (0, 0) => (n, a),                                   // FMADD
                        (0, 1) => (fp_neg_bits(n, eb), a),                  // FMSUB
                        (1, 0) => (fp_neg_bits(n, eb), fp_neg_bits(a, eb)), // FNMADD
                        _ => (n, fp_neg_bits(a, eb)),                       // FNMSUB
                    };
                    let r = fp_muladd_bits(aa, nn, m, eb);
                    self.v[rd] = (r & m_mask) as u128;
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            }
            return Ok(CpuExit::Continue);
        }

        // Scalar FP immediate (FMOV Sd/Dd/Hd, #imm): 00011110 ptype 1 imm8 100
        // 00000 Rd. imm8=bits[20:13]; the 8-bit float immediate expands per the
        // element size (h/s/d). Writes the low element and zeroes the upper bits.
        if (insn >> 24) & 0xFF == 0b00011110
            && (insn >> 21) & 1 == 1
            && (insn >> 10) & 0x7 == 0b100
            && (insn >> 5) & 0x1F == 0
        {
            let ptype = (insn >> 22) & 0x3;
            let esize = match ptype {
                0b00 => 4usize,
                0b01 => 8,
                0b11 => 2,
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            let imm8 = ((insn >> 13) & 0xFF) as u8;
            let rd = (insn & 0x1F) as usize;
            self.v[rd] = vfp_expand_imm(imm8, esize) as u128;
            return Ok(CpuExit::Continue);
        }

        // Scalar FP data processing (two source)
        // bits[31:24] = 0001_1110
        // bits[23:22] = type (size)
        // bit[21] = 1
        // bits[15:12] = opcode
        // bits[11:10] = 10
        if (insn >> 24) & 0xFF == 0b00011110 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0b10
        {
            let fp_type = (insn >> 22) & 0x3;
            let opcode = (insn >> 12) & 0xF;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rd = (insn & 0x1F) as u8;

            // FMUL/FDIV/FADD/FSUB/FMAX/FMIN/FMAXNM/FMINNM via the canonicalising
            // helper; FNMUL = FPNeg(FPMul) flips the product's (incl. NaN) sign.
            let kind = match opcode {
                0b0000 | 0b1000 => FpKind::Mul,
                0b0001 => FpKind::Div,
                0b0010 => FpKind::Add,
                0b0011 => FpKind::Sub,
                0b0100 => FpKind::Max,
                0b0101 => FpKind::Min,
                0b0110 => FpKind::MaxNm,
                0b0111 => FpKind::MinNm,
                _ => return Err(ArmError::Unimplemented(format!("FP opcode {}", opcode))),
            };
            let nmul = opcode == 0b1000;
            match fp_type {
                0b00 => {
                    let (n, m) = (self.v[rn as usize] as u32, self.v[rm as usize] as u32);
                    let mut r = fp_three_same_f32(kind, n, m, 0);
                    if nmul {
                        r ^= 0x8000_0000;
                    }
                    self.v[rd as usize] = r as u128;
                }
                0b01 => {
                    let (n, m) = (self.v[rn as usize] as u64, self.v[rm as usize] as u64);
                    let mut r = fp_three_same_f64(kind, n, m, 0);
                    if nmul {
                        r ^= 0x8000_0000_0000_0000;
                    }
                    self.v[rd as usize] = r as u128;
                }
                0b11 => {
                    let (n, m) = (self.v[rn as usize] as u16, self.v[rm as usize] as u16);
                    let mut r = sve_fp16_binop(kind, n, m);
                    if nmul {
                        r ^= 0x8000;
                    }
                    self.v[rd as usize] = r as u128;
                }
                _ => return Err(ArmError::Unimplemented("FP16/reserved".to_string())),
            }
            return Ok(CpuExit::Continue);
        }

        // Scalar FP data processing (one source)
        // bits[31:24] = 0001_1110
        // bits[23:22] = type (size)
        // bit[21] = 1
        // bits[20:15] = opcode
        // bits[14:10] = 10000
        if (insn >> 24) & 0xFF == 0b00011110
            && (insn >> 21) & 1 == 1
            && (insn >> 10) & 0x1F == 0b10000
        {
            let fp_type = (insn >> 22) & 0x3;
            let opcode = (insn >> 15) & 0x1F;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rd = (insn & 0x1F) as u8;

            // BFCVT Hd, Sn (FEAT_BF16): single-precision -> bfloat16, RNE.
            // Encoded as ptype=01, opcode bits[20:15]=000110 (bits[19:15]=00110).
            if fp_type == 0b01 && opcode == 0b00110 {
                let bf = f32_to_bf16(self.v[rn as usize] as u32);
                self.v[rd as usize] = bf as u128;
                return Ok(CpuExit::Continue);
            }

            // FCVT (precision change between h/s/d): opcode 0b001xx, bits[16:15]
            // (=opcode&3) select the destination precision (00=s,01=d,11=h),
            // ptype the source. Round-to-nearest-even; NaN via FPConvertNaN.
            if matches!(opcode, 0b00100 | 0b00101 | 0b00111) {
                let dst = opcode & 0x3;
                if dst == fp_type {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let src_prec = match fp_type {
                    0b00 => 4usize,
                    0b01 => 8,
                    0b11 => 2,
                    _ => return Err(ArmError::UndefinedInstruction(insn)),
                };
                let dst_prec = match dst {
                    0b00 => 4usize,
                    0b01 => 8,
                    _ => 2,
                };
                let sb = self.v[rn as usize];
                // Read the source as f64, and detect a NaN at source precision.
                let (val, is_nan) = match src_prec {
                    4 => {
                        let b = sb as u32;
                        (f32::from_bits(b) as f64, is_nan32(b))
                    }
                    8 => (f64::from_bits(sb as u64), is_nan64(sb as u64)),
                    _ => {
                        let b = sb as u16;
                        (fp16_to_f64(b), (b & 0x7C00) == 0x7C00 && (b & 0x3FF) != 0)
                    }
                };
                let r: u64 = if is_nan {
                    fp_convert_nan(sb as u64, src_prec, dst_prec)
                } else {
                    match dst_prec {
                        4 => (val as f32).to_bits() as u64,
                        8 => val.to_bits(),
                        _ => fp16_round(val) as u64,
                    }
                };
                self.v[rd as usize] = r as u128;
                return Ok(CpuExit::Continue);
            }

            // FRINT32Z/X (opcode bits[19:15]=10000/10001) and FRINT64Z/X
            // (10010/10011): scalar FEAT_FRINTTS, f32 (ptype 00) / f64 (ptype 01).
            if matches!(opcode, 0b10000 | 0b10001 | 0b10010 | 0b10011) {
                let intsize = if opcode & 0b10 == 0 { 32 } else { 64 };
                let z = opcode & 1 == 0;
                self.v[rd as usize] = match fp_type {
                    0b00 => frint_ts_f32(self.v[rn as usize] as u32, intsize, z) as u128,
                    0b01 => frint_ts_f64(self.v[rn as usize] as u64, intsize, z) as u128,
                    _ => return Err(ArmError::UndefinedInstruction(insn)),
                };
                return Ok(CpuExit::Continue);
            }

            // FMOV is a plain copy; the FRINT/FABS/FNEG/FSQRT ops share the
            // verified two-reg FP element helpers (correct rounding modes).
            let kind = match opcode {
                0b00000 => None, // FMOV
                0b00001 => Some(TwoRegFp::Fabs),
                0b00010 => Some(TwoRegFp::Fneg),
                0b00011 => Some(TwoRegFp::Fsqrt),
                0b01000 => Some(TwoRegFp::RintN),
                0b01001 => Some(TwoRegFp::RintP),
                0b01010 => Some(TwoRegFp::RintM),
                0b01011 => Some(TwoRegFp::RintZ),
                0b01100 => Some(TwoRegFp::RintA),
                0b01110 => Some(TwoRegFp::RintX),
                0b01111 => Some(TwoRegFp::RintI),
                // 0b001xx with bit2 set are FCVT (precision change) -> handled by
                // the dedicated FCVT block; anything else is unallocated here.
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            match fp_type {
                0b00 => {
                    let a = self.v[rn as usize] as u32;
                    let r = match kind {
                        None => a,
                        Some(k) => fp_two_reg_f32(k, a),
                    };
                    self.v[rd as usize] = r as u128;
                }
                0b01 => {
                    let a = self.v[rn as usize] as u64;
                    let r = match kind {
                        None => a,
                        Some(k) => fp_two_reg_f64(k, a),
                    };
                    self.v[rd as usize] = r as u128;
                }
                0b11 => {
                    let a = self.v[rn as usize] as u16;
                    let r: u16 = match kind {
                        None => a, // FMOV
                        Some(TwoRegFp::Fabs) => a & 0x7FFF,
                        Some(TwoRegFp::Fneg) => a ^ 0x8000,
                        Some(TwoRegFp::Fsqrt) => fp16_sqrt(a),
                        Some(TwoRegFp::RintN) | Some(TwoRegFp::RintX) | Some(TwoRegFp::RintI) => {
                            fp16_frint(a, 0)
                        }
                        Some(TwoRegFp::RintM) => fp16_frint(a, 1),
                        Some(TwoRegFp::RintP) => fp16_frint(a, 2),
                        Some(TwoRegFp::RintZ) => fp16_frint(a, 3),
                        Some(TwoRegFp::RintA) => fp16_frint(a, 4),
                        _ => return Err(ArmError::UndefinedInstruction(insn)),
                    };
                    self.v[rd as usize] = r as u128;
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            }
            return Ok(CpuExit::Continue);
        }

        // FP compare
        // bits[31:24] = 0001_1110
        // bits[23:22] = type
        // bit[21] = 1
        // bits[15:14] = 00
        // bits[13:10] = 1000
        // bits[4:3] = opc
        // bits[2:0] = 0xx
        if (insn >> 24) & 0xFF == 0b00011110
            && (insn >> 21) & 1 == 1
            && (insn >> 14) & 0x3 == 0
            && (insn >> 10) & 0xF == 0b1000
            && (insn & 0x7) == 0
        {
            let fp_type = (insn >> 22) & 0x3;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let opc = (insn >> 3) & 0x3;
            let cmp_with_zero = (insn & 0x8) != 0;

            match fp_type {
                0b00 => {
                    // Single precision
                    let op1 = f32::from_bits(self.v[rn as usize] as u32);
                    let op2 = if cmp_with_zero {
                        0.0f32
                    } else {
                        f32::from_bits(self.v[rm as usize] as u32)
                    };

                    let (n, z, c, v) = if op1.is_nan() || op2.is_nan() {
                        // Unordered
                        if opc & 1 != 0 {
                            // FCMPE - signal exception
                            // For now, just set flags
                        }
                        (false, false, true, true)
                    } else if op1 == op2 {
                        (false, true, true, false)
                    } else if op1 < op2 {
                        (true, false, false, false)
                    } else {
                        (false, false, true, false)
                    };

                    self.set_n(n);
                    self.set_z(z);
                    self.set_c(c);
                    self.set_v(v);
                }
                0b01 => {
                    // Double precision
                    let op1 = f64::from_bits(self.v[rn as usize] as u64);
                    let op2 = if cmp_with_zero {
                        0.0f64
                    } else {
                        f64::from_bits(self.v[rm as usize] as u64)
                    };

                    let (n, z, c, v) = if op1.is_nan() || op2.is_nan() {
                        (false, false, true, true)
                    } else if op1 == op2 {
                        (false, true, true, false)
                    } else if op1 < op2 {
                        (true, false, false, false)
                    } else {
                        (false, false, true, false)
                    };

                    self.set_n(n);
                    self.set_z(z);
                    self.set_c(c);
                    self.set_v(v);
                }
                0b11 => {
                    // Half precision (compared exactly via f64).
                    let op1 = fp16_to_f64(self.v[rn as usize] as u16);
                    let op2 = if cmp_with_zero {
                        0.0f64
                    } else {
                        fp16_to_f64(self.v[rm as usize] as u16)
                    };
                    let (n, z, c, v) = if op1.is_nan() || op2.is_nan() {
                        (false, false, true, true)
                    } else if op1 == op2 {
                        (false, true, true, false)
                    } else if op1 < op2 {
                        (true, false, false, false)
                    } else {
                        (false, false, true, false)
                    };
                    self.set_n(n);
                    self.set_z(z);
                    self.set_c(c);
                    self.set_v(v);
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            }
            return Ok(CpuExit::Continue);
        }

        // Floating-point conditional compare (FCCMP / FCCMPE)
        // bits[31:24]=0001_1110, bit21=1, bits[11:10]=01
        if (insn >> 24) & 0xFF == 0b00011110 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0b01
        {
            let fp_type = (insn >> 22) & 0x3;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let cond = ((insn >> 12) & 0xF) as u8;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let nzcv_imm = (insn & 0xF) as u8;

            let to_f64 = |bits: u128| -> Option<f64> {
                Some(match fp_type {
                    0b00 => f32::from_bits(bits as u32) as f64,
                    0b01 => f64::from_bits(bits as u64),
                    0b11 => Self::fp16_to_f32(bits as u16) as f64,
                    _ => return None,
                })
            };
            let (a, b) = match (to_f64(self.v[rn]), to_f64(self.v[rm])) {
                (Some(a), Some(b)) => (a, b),
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };

            if self.condition_holds(cond) {
                let (n, z, c, v) = if a.is_nan() || b.is_nan() {
                    (false, false, true, true)
                } else if a == b {
                    (false, true, true, false)
                } else if a < b {
                    (true, false, false, false)
                } else {
                    (false, false, true, false)
                };
                self.set_nzcv(n, z, c, v);
            } else {
                self.set_n(nzcv_imm & 0b1000 != 0);
                self.set_z(nzcv_imm & 0b0100 != 0);
                self.set_c(nzcv_imm & 0b0010 != 0);
                self.set_v(nzcv_imm & 0b0001 != 0);
            }
            return Ok(CpuExit::Continue);
        }

        // Floating-point conditional select (FCSEL)
        // bits[31:24]=0001_1110, bit21=1, bits[11:10]=11
        if (insn >> 24) & 0xFF == 0b00011110 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0b11
        {
            let fp_type = (insn >> 22) & 0x3;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let cond = ((insn >> 12) & 0xF) as u8;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;

            let width: u32 = match fp_type {
                0b00 => 32,
                0b01 => 64,
                0b11 => 16,
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            let src = if self.condition_holds(cond) {
                self.v[rn]
            } else {
                self.v[rm]
            };
            let mask = (1u128 << width) - 1;
            self.v[rd] = src & mask; // scalar result, upper bits zeroed
            return Ok(CpuExit::Continue);
        }

        // FMOV (general) - move between FP and general registers
        // bits[31] = sf
        // bits[30:24] = 0011110
        // bits[23:22] = type
        // bit[21] = 1
        // bits[20:19] = rmode
        // bits[18:16] = opcode
        // bits[15:10] = 000000
        if (insn >> 24) & 0x7F == 0b0011110
            && (insn >> 21) & 1 == 1
            && (insn >> 10) & 0x3F == 0
            && (insn >> 16) & 0x7 >= 0b110
        {
            let sf = (insn >> 31) & 1;
            let fp_type = (insn >> 22) & 0x3;
            let rmode = (insn >> 19) & 0x3;
            let opcode = (insn >> 16) & 0x7;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rd = (insn & 0x1F) as u8;

            match (sf, fp_type, rmode, opcode) {
                // FMOV Wd, Sn
                (0, 0b00, 0b00, 0b110) => {
                    let val = self.v[rn as usize] as u32;
                    self.set_w(rd, val);
                }
                // FMOV Sd, Wn
                (0, 0b00, 0b00, 0b111) => {
                    let val = self.get_w(rn);
                    self.v[rd as usize] = val as u128;
                }
                // FMOV Xd, Dn
                (1, 0b01, 0b00, 0b110) => {
                    let val = self.v[rn as usize] as u64;
                    self.set_x(rd, val);
                }
                // FMOV Dn, Xn
                (1, 0b01, 0b00, 0b111) => {
                    let val = self.get_x(rn);
                    self.v[rd as usize] = val as u128;
                }
                // FMOV Xd, Vn.D[1]
                (1, 0b10, 0b01, 0b110) => {
                    let val = (self.v[rn as usize] >> 64) as u64;
                    self.set_x(rd, val);
                }
                // FMOV Vd.D[1], Xn
                (1, 0b10, 0b01, 0b111) => {
                    let val = self.get_x(rn);
                    let lower = self.v[rd as usize] as u64;
                    self.v[rd as usize] = ((val as u128) << 64) | (lower as u128);
                }
                // FMOV Wd, Hn / FMOV Xd, Hn (FEAT_FP16): zero-extend the 16 bits.
                (_, 0b11, 0b00, 0b110) => {
                    let val = self.v[rn as usize] as u16 as u64;
                    if sf == 1 {
                        self.set_x(rd, val);
                    } else {
                        self.set_w(rd, val as u32);
                    }
                }
                // FMOV Hd, Wn / FMOV Hd, Xn (FEAT_FP16): low 16 bits -> Hd.
                (_, 0b11, 0b00, 0b111) => {
                    let val = if sf == 1 {
                        self.get_x(rn)
                    } else {
                        self.get_w(rn) as u64
                    };
                    self.v[rd as usize] = (val & 0xFFFF) as u128;
                }
                // FJCVTZS Wd, Dn (FEAT_JSCVT): JS double->int32 (round toward
                // zero, modulo 2^32). Z=1 iff exact (finite, integral, in range);
                // N=C=V=0.
                (0, 0b01, 0b11, 0b110) => {
                    let x = f64::from_bits(self.v[rn as usize] as u64);
                    let (res, exact): (i32, bool) = if !x.is_finite() {
                        (0, false)
                    } else {
                        let t = x.trunc();
                        // JS ToInt32: reduce trunc(x) modulo 2^32 to a signed 32-bit.
                        let m = t.rem_euclid(4294967296.0); // [0, 2^32)
                        let r = if m >= 2147483648.0 {
                            (m - 4294967296.0) as i64
                        } else {
                            m as i64
                        } as i32;
                        // -0.0 is "inexact" for JavaScript (qemu helper_fjcvtzs).
                        let neg_zero = x == 0.0 && x.is_sign_negative();
                        (r, x == r as f64 && !neg_zero)
                    };
                    self.set_w(rd, res as u32);
                    self.set_nzcv(false, exact, false, false);
                }
                _ => {
                    return Err(ArmError::Unimplemented(format!(
                        "FMOV general variant sf={} type={} rmode={} op={}",
                        sf, fp_type, rmode, opcode
                    )));
                }
            }
            return Ok(CpuExit::Continue);
        }

        // FCVT - floating-point convert precision
        // bits[31:24] = 0001_1110
        // bits[23:22] = type (source)
        // bit[21] = 1
        // bits[20:17] = 0001
        // bits[16:15] = opc (dest)
        // bits[14:10] = 10000
        if (insn >> 24) & 0xFF == 0b00011110
            && (insn >> 21) & 1 == 1
            && (insn >> 17) & 0xF == 0b0001
            && (insn >> 10) & 0x1F == 0b10000
        {
            let src_type = (insn >> 22) & 0x3;
            let dst_type = (insn >> 15) & 0x3;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rd = (insn & 0x1F) as u8;

            match (src_type, dst_type) {
                // FCVT Dd, Sn (single to double)
                (0b00, 0b01) => {
                    let val = f32::from_bits(self.v[rn as usize] as u32);
                    let result = val as f64;
                    self.v[rd as usize] = result.to_bits() as u128;
                }
                // FCVT Sd, Dn (double to single)
                (0b01, 0b00) => {
                    let val = f64::from_bits(self.v[rn as usize] as u64);
                    let result = val as f32;
                    self.v[rd as usize] = result.to_bits() as u128;
                }
                _ => {
                    return Err(ArmError::Unimplemented(format!(
                        "FCVT variant src={} dst={}",
                        src_type, dst_type
                    )));
                }
            }
            return Ok(CpuExit::Continue);
        }

        // Fixed-point conversion between FP and GPR (scalar): sf 0 0 11110 ptype
        // 0 rmode opcode scale Rn Rd, bit21==0. scale=bits[15:10], fbits=64-scale.
        // SCVTF/UCVTF (opcode 010/011, GPR int -> FP scaled by 2^-fbits) and
        // FCVTZS/FCVTZU (opcode 000/001, FP -> GPR int = trunc(FP * 2^fbits)).
        if (insn >> 24) & 0x7F == 0b0011110
            && (insn >> 21) & 1 == 0
            && matches!((insn >> 16) & 0x7, 0b000 | 0b001 | 0b010 | 0b011)
        {
            let sf = (insn >> 31) & 1;
            let ptype = (insn >> 22) & 0x3;
            let opcode = (insn >> 16) & 0x7;
            let scale = (insn >> 10) & 0x3F;
            let fbits = 64 - scale as i32;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rd = (insn & 0x1F) as u8;
            // 32-bit forms only support fbits 1..=32: scale<5> must be 1.
            // The rmode field must also be fixed (11 for FCVTZ*, 00 for *CVTF).
            if sf == 0 && (scale >> 5) & 1 == 0 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let rmode = (insn >> 19) & 0x3;
            let want_rmode = if opcode >= 0b010 { 0b00 } else { 0b11 };
            if rmode != want_rmode || ptype == 0b10 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            if opcode == 0b010 || opcode == 0b011 {
                // GPR int -> FP, value = int / 2^fbits (the 2^-fbits scale is an
                // exact power of two, so a single FPRound of the integer suffices).
                let signed = opcode == 0b010;
                let scale_f = 2f64.powi(-fbits);
                let r: u64 = match ptype {
                    0b00 => {
                        let f = if signed {
                            (if sf == 1 {
                                self.get_x(rn) as i64 as f32
                            } else {
                                self.get_w(rn) as i32 as f32
                            })
                        } else {
                            (if sf == 1 {
                                self.get_x(rn) as f32
                            } else {
                                self.get_w(rn) as f32
                            })
                        };
                        (f * 2f32.powi(-fbits)).to_bits() as u64
                    }
                    0b01 => {
                        let v = if signed {
                            (if sf == 1 {
                                self.get_x(rn) as i64 as f64
                            } else {
                                self.get_w(rn) as i32 as f64
                            })
                        } else {
                            (if sf == 1 {
                                self.get_x(rn) as f64
                            } else {
                                self.get_w(rn) as f64
                            })
                        };
                        (v * scale_f).to_bits()
                    }
                    0b11 => {
                        let v = if signed {
                            (if sf == 1 {
                                self.get_x(rn) as i64 as f64
                            } else {
                                self.get_w(rn) as i32 as f64
                            })
                        } else {
                            (if sf == 1 {
                                self.get_x(rn) as f64
                            } else {
                                self.get_w(rn) as f64
                            })
                        };
                        fp16_round(v * scale_f) as u64
                    }
                    _ => return Err(ArmError::UndefinedInstruction(insn)),
                };
                self.v[rd as usize] = r as u128;
                return Ok(CpuExit::Continue);
            }
            // FP -> GPR int, truncating toward zero: int = sat(trunc(FP * 2^fbits)).
            let signed = (opcode & 1) == 0;
            let fval: f64 = match ptype {
                0b00 => f32::from_bits(self.v[rn as usize] as u32) as f64,
                0b01 => f64::from_bits(self.v[rn as usize] as u64),
                0b11 => fp16_to_f64(self.v[rn as usize] as u16),
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            let scaled = fval * 2f64.powi(fbits);
            let res: u64 = match (sf == 1, signed) {
                (true, true) => scaled as i64 as u64,
                (true, false) => scaled as u64,
                (false, true) => (scaled as i32) as u32 as u64,
                (false, false) => (scaled as u32) as u64,
            };
            if sf == 1 {
                self.set_x(rd, res);
            } else {
                self.set_w(rd, res as u32);
            }
            return Ok(CpuExit::Continue);
        }

        // Conversion between floating-point and integer (scalar, GPR-involved):
        // sf 0 0 11110 ptype 1 rmode opcode 000000 Rn Rd. rmode=bits[20:19],
        // opcode=bits[18:16]. FMOV (opcode 11x) is handled above; here are the
        // FP<->int conversions: SCVTF/UCVTF (opcode 010/011, int->FP) and
        // FCVTNS/NU/PS/PU/MS/MU/ZS/ZU/AS/AU (FP->int, rounding from rmode, or
        // ties-away for opcode 100/101).
        if (insn >> 24) & 0x7F == 0b0011110 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3F == 0 {
            let sf = (insn >> 31) & 1;
            let ptype = (insn >> 22) & 0x3;
            let rmode = (insn >> 19) & 0x3;
            let opcode = (insn >> 16) & 0x7;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rd = (insn & 0x1F) as u8;
            if opcode >= 0b110 {
                // FMOV (general) is handled earlier; anything else is unallocated.
                return Err(ArmError::UndefinedInstruction(insn));
            }
            if opcode == 0b010 || opcode == 0b011 {
                // SCVTF / UCVTF: integer (GPR) -> floating-point.
                let signed = opcode == 0b010;
                let iv = if sf == 1 {
                    self.get_x(rn)
                } else {
                    self.get_w(rn) as u64
                };
                let val: f64 = if signed {
                    if sf == 1 {
                        iv as i64 as f64
                    } else {
                        iv as i32 as f64
                    }
                } else if sf == 1 {
                    iv as f64
                } else {
                    iv as u32 as f64
                };
                let r: u64 = match ptype {
                    0b00 => {
                        // Round the (exact-in-f64) value to f32.
                        let f: f32 = if signed {
                            if sf == 1 {
                                iv as i64 as f32
                            } else {
                                iv as i32 as f32
                            }
                        } else if sf == 1 {
                            iv as f32
                        } else {
                            iv as u32 as f32
                        };
                        f.to_bits() as u64
                    }
                    0b01 => val.to_bits(),
                    0b11 => fp16_round(val) as u64,
                    _ => return Err(ArmError::UndefinedInstruction(insn)),
                };
                self.v[rd as usize] = r as u128;
                return Ok(CpuExit::Continue);
            }
            // FP -> integer. signed = even opcode; rounding from rmode (or
            // ties-away for FCVTA* opcode 100/101).
            let signed = (opcode & 1) == 0;
            let fval: f64 = match ptype {
                0b00 => f32::from_bits(self.v[rn as usize] as u32) as f64,
                0b01 => f64::from_bits(self.v[rn as usize] as u64),
                0b11 => fp16_to_f64(self.v[rn as usize] as u16),
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            let rounded = if opcode >= 0b100 {
                fval.round() // FCVTAS/FCVTAU: round to nearest, ties away
            } else {
                match rmode {
                    0b00 => fval.round_ties_even(),
                    0b01 => fval.ceil(),
                    0b10 => fval.floor(),
                    _ => fval.trunc(),
                }
            };
            // Saturate into the sf-width signed/unsigned range; NaN -> 0 (Rust's
            // float-to-int `as` already truncates/saturates/maps NaN to 0, and the
            // input is already integral).
            let res: u64 = match (sf == 1, signed) {
                (true, true) => rounded as i64 as u64,
                (true, false) => rounded as u64,
                (false, true) => (rounded as i32) as u32 as u64,
                (false, false) => (rounded as u32) as u64,
            };
            if sf == 1 {
                self.set_x(rd, res);
            } else {
                self.set_w(rd, res as u32);
            }
            return Ok(CpuExit::Continue);
        }

        // (FADD/FADDP FP16 fall through to the unified three-same FP16 handler
        // below; the previous dedicated add handler rounded incorrectly.)

        // SM3/SM4 crypto (bits[31:24]=0xCE). This MUST precede every Advanced
        // SIMD dispatch below: 0xCE has bits[28:24]=01110 and bit22=1/bit10=1,
        // so e.g. SM3SS1 would otherwise be captured by the FP16 three-same
        // group and executed as FMLA.
        if (insn >> 24) & 0xFF == 0xCE {
            return self.exec_crypto(insn);
        }

        // Advanced SIMD copy (DUP element/general, INS element/general, SMOV,
        // UMOV). Identified by bits[23:21]==000 (bit22==0 distinguishes it from
        // the FP16 three-same group, which has bit22==1). Must precede FP16.
        // Encoding: 0_Q_op_01110000_imm5_0_imm4_1_Rn_Rd
        // op_bits 11110 (top 0x5E) is the scalar form: DUP <V><d>,<Vn>.<T>[i]
        // (a.k.a. MOV), handled inside exec_simd_copy.
        if matches!((insn >> 24) & 0x1F, 0b01110 | 0b11110)
            && (insn >> 21) & 0x7 == 0
            && (insn >> 15) & 1 == 0
            && (insn >> 10) & 1 == 1
        {
            return self.exec_simd_copy(insn);
        }

        // Advanced SIMD three-same FP16 (vector and scalar)
        // FP16 uses bit[21]=0 (unlike regular three-same which has bit[21]=1)
        // Various FP16 ops use different bits[23:22] values:
        //   - FADD/FSUB/etc: bits[23:22]=11
        //   - FDIV/FRECPS/FRSQRTS: bits[23:22]=01
        let op_bits = (insn >> 24) & 0x1F;
        if (op_bits == 0b01110 || op_bits == 0b11110)
            && (insn >> 22) & 1 == 1       // bit[22]=1 for FP16 three-same
            && (insn >> 21) & 1 == 0       // bit[21]=0 for FP16 three-same
            && (insn >> 14) & 0x3 == 0b00  // bits[15:14]=00 for FP16 three-same
            && (insn >> 10) & 1 == 1
        {
            return self.exec_simd_fp16_three_same(insn);
        }

        // Advanced SIMD three-same (vector and scalar)
        // Vector encoding: 0_Q_U_01110_size_1_Rm_opcode_1_Rn_Rd (bits[28:24]=01110)
        // Scalar encoding: 0_1_U_11110_size_1_Rm_opcode_1_Rn_Rd (bits[28:24]=11110)
        let op_bits = (insn >> 24) & 0x1F;
        if (op_bits == 0b01110 || op_bits == 0b11110)
            && (insn >> 21) & 1 == 1
            && (insn >> 10) & 1 == 1
        {
            return self.exec_simd_three_same(insn);
        }

        // BFCVTN/BFCVTN2 (FEAT_BF16): f32 -> bf16 narrowing. Same two-reg-misc
        // slot as FCVTN (opcode 10110) but selected by size==10 (FCVTN uses
        // size 0x). Intercept before the generic two-reg-misc handler.
        if op_bits == 0b01110
            && (insn >> 29) & 1 == 0
            && (insn >> 22) & 0x3 == 0b10
            && (insn >> 17) & 0x1F == 0b10000
            && (insn >> 12) & 0x1F == 0b10110
            && (insn >> 10) & 0x3 == 0b10
        {
            return self.exec_simd_bfcvtn(insn);
        }

        // Advanced SIMD two-reg misc (vector and scalar)
        // Vector encoding: 0_Q_U_01110_size_10000_opcode_10_Rn_Rd (bits[28:24]=01110)
        // Scalar encoding: 0_1_U_11110_size_10000_opcode_10_Rn_Rd (bits[28:24]=11110)
        if (op_bits == 0b01110 || op_bits == 0b11110)
            && (insn >> 17) & 0x1F == 0b10000
            && (insn >> 10) & 0x3 == 0b10
        {
            return self.exec_simd_two_reg(insn);
        }

        // Advanced SIMD two-reg misc FP16 (vector and scalar)
        // Encoding pattern: bits[21:19]=111 distinguishes FP16 from normal two-reg misc
        // Vector: 0_Q_U_01110_size_111_opcode_10_Rn_Rd
        // Scalar: 0_1_U_11110_size_111_opcode_10_Rn_Rd
        if (op_bits == 0b01110 || op_bits == 0b11110)
            && (insn >> 19) & 0x7 == 0b111  // FP16 distinguishing bits
            && (insn >> 10) & 0x3 == 0b10
        {
            return self.exec_simd_fp16_two_reg(insn);
        }

        // Scalar three-different: SQDMULL/SQDMLAL/SQDMLSL <Dd>,<Sn>,<Sm> (and the
        // S<-H form). Top 0x5E (op_bits==11110), bit21==1, bits[11:10]==00,
        // opcode 1101/1001/1011. Signed doubling widening multiply, then
        // (optionally) a saturating accumulate; one element, rest zeroed.
        if op_bits == 0b11110 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0b00 {
            let size = (insn >> 22) & 0x3;
            let opcode = (insn >> 12) & 0xF;
            let (accum, subtract) = match opcode {
                0b1101 => (false, false), // SQDMULL
                0b1001 => (true, false),  // SQDMLAL
                0b1011 => (true, true),   // SQDMLSL
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            if size != 0b01 && size != 0b10 {
                return Err(ArmError::UndefinedInstruction(insn)); // src H or S only
            }
            let bits = 8u32 << size; // narrow source element size
            let dbits = 2 * bits;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            let nv = sext_elem(self.v[rn] as u64 & elem_mask(bits), bits);
            let mv = sext_elem(self.v[rm] as u64 & elem_mask(bits), bits);
            let dmin = -(1i128 << (dbits - 1));
            let dmax = (1i128 << (dbits - 1)) - 1;
            let prod = (2 * nv * mv).clamp(dmin, dmax);
            let r: u128 = if accum {
                let d = sext_elem(self.v[rd] as u64 & elem_mask(dbits), dbits);
                (sat_signed(d + if subtract { -prod } else { prod }, dbits) as u128)
                    & elem_mask_u128(dbits)
            } else {
                (prod as u128) & elem_mask_u128(dbits)
            };
            self.v[rd] = r;
            return Ok(CpuExit::Continue);
        }

        // Advanced SIMD three different (disparate) - widening/narrowing operations
        // Encoding: 0_Q_U_01110_size_1_Rm_opcode_00_Rn_Rd
        // bits[28:24]=01110, bit[21]=1, bits[11:10]=00
        if op_bits == 0b01110 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0b00 {
            return self.exec_simd_three_different(insn);
        }

        // SDOT/UDOT (FEAT_DotProd, bits[15:10]=100101) and USDOT (FEAT_I8MM,
        // U==0, bits[15:10]=100111): 8-bit -> 32-bit dot product, bit21==0.
        if op_bits == 0b01110 && (insn >> 21) & 1 == 0 {
            let lo6 = (insn >> 10) & 0x3F;
            if lo6 == 0b100101 {
                let signed = (insn >> 29) & 1 == 0; // SDOT (U=0) / UDOT (U=1)
                return self.exec_simd_dot(insn, signed, signed);
            }
            if lo6 == 0b100111 && (insn >> 29) & 1 == 0 {
                // USDOT: Vn unsigned, Vm signed.
                return self.exec_simd_dot(insn, false, true);
            }
        }

        // Three-same-extra (bit21==0): SQRDMLAH/SQRDMLSH (FEAT_RDM, bits[15:10]==
        // 100001/100011, vector + scalar) and SMMLA/UMMLA/USMMLA (FEAT_I8MM,
        // bits[15:10]==101001/101011, vector only).
        if (op_bits == 0b01110 || op_bits == 0b11110) && (insn >> 21) & 1 == 0 {
            let lo6 = (insn >> 10) & 0x3F;
            if matches!(lo6, 0b100001 | 0b100011 | 0b101001 | 0b101011) {
                return self.exec_simd_three_same_extra(insn);
            }
        }

        // FCMLA (vector): 0_Q_1_01110_size_0_Rm_110_rot_1_Rn_Rd
        //   bits[15:13]=110, bit10=1, rot=bits[12:11].
        // FCADD: 0_Q_1_01110_size_0_Rm_111_rot_01_Rn_Rd
        //   bits[15:13]=111, bits[11:10]=01, rot=bit12.
        if op_bits == 0b01110 && (insn >> 29) & 1 == 1 && (insn >> 21) & 1 == 0 {
            if (insn >> 13) & 0x7 == 0b110 && (insn >> 10) & 1 == 1 {
                return self.exec_simd_complex(insn, true);
            }
            if (insn >> 13) & 0x7 == 0b111 && (insn >> 10) & 0x3 == 0b01 {
                return self.exec_simd_complex(insn, false);
            }
            // BF16 three-same-extra: BFDOT/BFMLAL (bits[15:10]=111111) and
            // BFMMLA (bits[15:10]=111011), sub-selected by size bits[23:22].
            let lo6 = (insn >> 10) & 0x3F;
            let size = (insn >> 22) & 0x3;
            if lo6 == 0b111111 {
                if size == 0b01 {
                    return self.exec_simd_bfdot(insn, false); // BFDOT vector
                }
                if size == 0b11 {
                    return self.exec_simd_bfmlal(insn, false); // BFMLALB/T vector
                }
            }
            if lo6 == 0b111011 && size == 0b01 {
                return self.exec_simd_bfmmla(insn); // BFMMLA
            }
        }

        // Cryptographic AES/SHA operations
        // AES: 0100 1110 00 1 01000 0 opcode 10 Rn Rd (bits[31:24]=0x4E)
        // SHA two-reg: 0101 1110 00 1 01000 0 opcode 10 Rn Rd (bits[31:24]=0x5E)
        // The bits[21:17]==10100 marker distinguishes AES/SHA two-register crypto
        // from across-lanes (11000) and two-reg-misc (10000), which share the
        // same bits[31:24] for Q==1.
        if ((insn >> 24) & 0xFF == 0x4E || (insn >> 24) & 0xFF == 0x5E)
            && (insn >> 17) & 0x1F == 0b10100
            && (insn >> 10) & 0x3 == 0b10
        {
            return self.exec_crypto(insn);
        }

        // SHA/SM3/SM4 three-register operations
        // SHA three-reg: 0101 1110 000 Rm 0 opcode 00 Rn Rd (bits[31:24]=0x5E, bits[11:10]=00)
        // SM3/SM4: various encodings with bits[31:24]=0xCE
        if (insn >> 24) & 0xFF == 0x5E && (insn >> 21) & 7 == 0 && (insn >> 10) & 0x3 == 0b00 {
            return self.exec_crypto(insn);
        }

        // Advanced SIMD across lanes (reduction operations like ADDV, SADDLV, etc.)
        // Encoding: 0_Q_U_01110_size_11000_opcode_10_Rn_Rd
        if op_bits == 0b01110 && (insn >> 17) & 0x1F == 0b11000 && (insn >> 10) & 0x3 == 0b10 {
            return self.exec_simd_across_lanes(insn);
        }

        // AdvSIMD scalar pairwise (ADDP/FADDP/FMAXP/FMINP/FMAXNMP/FMINNMP to a
        // scalar): top 0x5E/0x7E, bits[21:17]==11000, bits[11:10]==10.
        if op_bits == 0b11110 && (insn >> 17) & 0x1F == 0b11000 && (insn >> 10) & 0x3 == 0b10 {
            return self.exec_simd_scalar_pairwise(insn);
        }

        // FCMLA by element: 0_Q_1_01111_size_L_M_Rm_0_rot_1_H_0_Rn_Rd. Must
        // precede the generic indexed dispatch below, since its opcode field
        // bits[15:12]=0_rot_1 overlaps FMLA/FMLS-by-element. Discriminated by
        // U==1, bit15==0, bit12==1, bit10==0.
        if op_bits == 0b01111
            && (insn >> 29) & 1 == 1
            && (insn >> 15) & 1 == 0
            && (insn >> 12) & 1 == 1
            && (insn >> 10) & 1 == 0
        {
            return self.exec_simd_complex_indexed(insn);
        }

        // U=0 by-element group with opcode bits[15:12]==1111, bit10==0: the
        // FEAT_I8MM / FEAT_BF16 by-element instructions, sub-selected by the
        // size field bits[23:22]: 00=SUDOT, 01=BFDOT, 10=USDOT, 11=BFMLALB/T.
        // Must precede the generic indexed dispatch below.
        if op_bits == 0b01111
            && (insn >> 29) & 1 == 0
            && (insn >> 12) & 0xF == 0b1111
            && (insn >> 10) & 1 == 0
        {
            match (insn >> 22) & 0x3 {
                0b00 => return self.exec_simd_dot_indexed_mixed(insn, true, false), // SUDOT: Vn signed, Vm unsigned
                0b10 => return self.exec_simd_dot_indexed_mixed(insn, false, true), // USDOT: Vn unsigned, Vm signed
                0b01 => return self.exec_simd_bfdot(insn, true), // BFDOT by element
                0b11 => return self.exec_simd_bfmlal(insn, true), // BFMLALB/T by element
                _ => {}
            }
        }

        // FEAT_FP8FMA FMLALB/FMLALT (by element): 0 Q 0 01111 11 idx Rm 0000
        // idx 0 Rn Rd. FP8 (E5M2: an exact f16 with the mantissa truncated to
        // 2 bits, so widening is a left shift) multiplied into f16 lanes;
        // Q selects bottom (0) / top (1) source bytes.
        if op_bits == 0b01111
            && (insn >> 29) & 1 == 0
            && (insn >> 22) & 0x3 == 0b11
            && (insn >> 12) & 0xF == 0b0000
            && (insn >> 10) & 1 == 0
        {
            let top = (insn >> 30) & 1 == 1;
            let rm = ((insn >> 16) & 0xF) as usize;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            let idx = ((((insn >> 11) & 1) << 3)
                | (((insn >> 21) & 1) << 2)
                | (((insn >> 20) & 1) << 1)
                | ((insn >> 10) & 1)) as usize;
            let f8_to_f16 = |b: u8| (b as u16) << 8;
            let n = self.v[rn].to_le_bytes();
            let m = self.v[rm].to_le_bytes();
            let mut d = self.v[rd].to_le_bytes();
            let mb = f8_to_f16(m[idx & 0xF]);
            for i in 0..8 {
                let nb = f8_to_f16(n[2 * i + usize::from(top)]);
                let prod = fp16_mul(nb, mb);
                let cur = u16::from_le_bytes([d[2 * i], d[2 * i + 1]]);
                let r = fp16_add(cur, prod);
                d[2 * i..2 * i + 2].copy_from_slice(&r.to_le_bytes());
            }
            self.v[rd] = u128::from_le_bytes(d);
            return Ok(CpuExit::Continue);
        }

        // FEAT_FHM FMLAL/FMLSL/FMLAL2/FMLSL2 by element: 0Q U 01111 10 L M Rm
        // (top:sub:00) H 0 Rn Rd. sz==10, bits[13:12]==00, and bit15==U (which
        // distinguishes them from the integer MUL/MLAL-by-element forms that
        // share bits[13:12]==00). Must precede the generic indexed dispatch.
        if op_bits == 0b01111
            && (insn >> 22) & 0x3 == 0b10
            && (insn >> 12) & 0x3 == 0
            && (insn >> 10) & 1 == 0
            && (insn >> 15) & 1 == (insn >> 29) & 1
        {
            return self.exec_fmlal(insn, true);
        }

        // Advanced SIMD vector x indexed element
        // Encoding: 0_Q_U_01111_size_L_M_Rm_opcode_H_0_Rn_Rd  (bit10 = 0)
        if (op_bits == 0b01111 || op_bits == 0b11111) && (insn >> 10) & 1 == 0 {
            return self.exec_simd_indexed(insn);
        }

        // Advanced SIMD modified immediate (MOVI/MVNI/ORR/BIC/FMOV vector)
        // Encoding: 0_Q_op_0111100000_abc_cmode_o2_1_defgh_Rd
        if (insn >> 19) & 0x3FF == 0b0111100000 && (insn >> 10) & 1 == 1 {
            return self.exec_simd_modified_imm(insn);
        }

        // Advanced SIMD shift by immediate (vector: bits[28:23]==011110; scalar:
        // bits[28:23]==111110). bit[10]==1. Both route to exec_simd_shift_imm,
        // which detects scalar via the top byte (0x5F/0x7F).
        if matches!((insn >> 23) & 0x3F, 0b011110 | 0b111110) && (insn >> 10) & 1 == 1 {
            return self.exec_simd_shift_imm(insn);
        }

        // Advanced SIMD permute (ZIP, UZP, TRN)
        // Encoding: 0_Q_0_01110_size_0_Rm_0_opcode_10_Rn_Rd
        if op_bits == 0b01110
            && (insn >> 29) & 1 == 0
            && (insn >> 21) & 1 == 0
            && (insn >> 15) & 1 == 0
            && (insn >> 10) & 0x3 == 0b10
        {
            return self.exec_simd_permute(insn);
        }

        // Advanced SIMD table lookup (TBL, TBX)
        // Encoding: 0_Q_0_01110_00_0_Rm_0_len_op_00_Rn_Rd
        if op_bits == 0b01110
            && (insn >> 29) & 1 == 0
            && (insn >> 22) & 0x3 == 0b00
            && (insn >> 21) & 1 == 0
            && (insn >> 10) & 0x3 == 0b00
        {
            return self.exec_simd_table(insn);
        }

        // Advanced SIMD extract (EXT)
        // Encoding: 0_Q_10_1110_00_0_Rm_0_imm4_0_Rn_Rd
        if op_bits == 0b01110 && (insn >> 29) & 1 == 1 && (insn >> 22) & 0x3 == 0b00 {
            return self.exec_simd_extract(insn);
        }

        // If we get here, it's an unimplemented SIMD/FP instruction
        Err(ArmError::Unimplemented(format!(
            "SIMD/FP insn 0x{:08x}",
            insn
        )))
    }

    /// Execute SIMD FP add (binary uniform add).
    fn exec_simd_fp_add_uniform(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let pair = ((insn >> 29) & 1) != 0;
        let size = (insn >> 22) & 0x3;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        let esize = if size & 1 == 0 { 4 } else { 8 };
        let datasize = if q == 1 { 16 } else { 8 };
        let elements = datasize / esize;

        let src1 = self.v[rn].to_le_bytes();
        let src2 = self.v[rm].to_le_bytes();
        let mut dst = [0u8; 16];
        let mut concat = [0u8; 32];

        if pair {
            concat[..datasize].copy_from_slice(&src1[..datasize]);
            concat[datasize..datasize * 2].copy_from_slice(&src2[..datasize]);
        }

        for e in 0..elements {
            let out_off = e * esize;
            if esize == 4 {
                let (a, b) = if pair {
                    let idx1 = 2 * e;
                    let idx2 = idx1 + 1;
                    let a_off = idx1 * esize;
                    let b_off = idx2 * esize;
                    (
                        f32::from_le_bytes([
                            concat[a_off],
                            concat[a_off + 1],
                            concat[a_off + 2],
                            concat[a_off + 3],
                        ]),
                        f32::from_le_bytes([
                            concat[b_off],
                            concat[b_off + 1],
                            concat[b_off + 2],
                            concat[b_off + 3],
                        ]),
                    )
                } else {
                    let a_off = e * esize;
                    let b_off = e * esize;
                    (
                        f32::from_le_bytes([
                            src1[a_off],
                            src1[a_off + 1],
                            src1[a_off + 2],
                            src1[a_off + 3],
                        ]),
                        f32::from_le_bytes([
                            src2[b_off],
                            src2[b_off + 1],
                            src2[b_off + 2],
                            src2[b_off + 3],
                        ]),
                    )
                };

                let result = a + b;
                let bytes = result.to_le_bytes();
                dst[out_off..out_off + 4].copy_from_slice(&bytes);
            } else {
                let (a, b) = if pair {
                    let idx1 = 2 * e;
                    let idx2 = idx1 + 1;
                    let a_off = idx1 * esize;
                    let b_off = idx2 * esize;
                    (
                        f64::from_le_bytes([
                            concat[a_off],
                            concat[a_off + 1],
                            concat[a_off + 2],
                            concat[a_off + 3],
                            concat[a_off + 4],
                            concat[a_off + 5],
                            concat[a_off + 6],
                            concat[a_off + 7],
                        ]),
                        f64::from_le_bytes([
                            concat[b_off],
                            concat[b_off + 1],
                            concat[b_off + 2],
                            concat[b_off + 3],
                            concat[b_off + 4],
                            concat[b_off + 5],
                            concat[b_off + 6],
                            concat[b_off + 7],
                        ]),
                    )
                } else {
                    let a_off = e * esize;
                    let b_off = e * esize;
                    (
                        f64::from_le_bytes([
                            src1[a_off],
                            src1[a_off + 1],
                            src1[a_off + 2],
                            src1[a_off + 3],
                            src1[a_off + 4],
                            src1[a_off + 5],
                            src1[a_off + 6],
                            src1[a_off + 7],
                        ]),
                        f64::from_le_bytes([
                            src2[b_off],
                            src2[b_off + 1],
                            src2[b_off + 2],
                            src2[b_off + 3],
                            src2[b_off + 4],
                            src2[b_off + 5],
                            src2[b_off + 6],
                            src2[b_off + 7],
                        ]),
                    )
                };

                let result = a + b;
                let bytes = result.to_le_bytes();
                dst[out_off..out_off + 8].copy_from_slice(&bytes);
            }
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD FP16 three-same register instructions.
    fn exec_simd_fp16_three_same(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let a = (insn >> 23) & 1; // Selects between two groups of operations
        let rm = ((insn >> 16) & 0x1F) as usize;
        let opcode = (insn >> 11) & 0x7; // 3 bits for FP16
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        // For scalar (bit28=1) only the low halfword is processed.
        let is_scalar = ((insn >> 28) & 1) == 1;
        let datasize = if is_scalar {
            2
        } else if q == 1 {
            16
        } else {
            8
        };
        let elements = datasize / 2;

        // The scalar three-same FP16 group is a subset: only FMULX, FCMEQ,
        // FRECPS, FRSQRTS, FCMGE, FACGE, FABD, FCMGT and FACGT have scalar
        // encodings. The element-wise arithmetic (FADD/FSUB/FMUL/FMAX/FMIN/
        // FMAXNM/FMINNM/FDIV), the fused FMLA/FMLS and the pairwise forms do
        // not, so reject them in scalar context.
        if is_scalar
            && !matches!(
                (u, a, opcode),
                (0, 0, 0b011)
                    | (0, 0, 0b100)
                    | (0, 0, 0b111)
                    | (0, 1, 0b111)
                    | (1, 0, 0b100)
                    | (1, 0, 0b101)
                    | (1, 1, 0b010)
                    | (1, 1, 0b100)
                    | (1, 1, 0b101)
            )
        {
            return Ok(CpuExit::Undefined(insn));
        }

        // Classify the operation. `Bin` is a per-lane binary op; `Mla`/`Mls`
        // are the fused multiply-accumulate forms (they read the destination);
        // `Pair` is a pairwise-reduction op. See the Arm "Advanced SIMD
        // three-same (FP16)" table indexed by (U, a=bit23, opcode=bits[13:11]).
        enum Fp16Op {
            Bin(fn(u16, u16) -> u16),
            Pair(fn(u16, u16) -> u16),
            Mla,
            Mls,
        }
        let op = match (u, a, opcode) {
            // U=0
            (0, 0, 0b000) => Fp16Op::Bin(fp16_maxnm),
            (0, 1, 0b000) => Fp16Op::Bin(fp16_minnm),
            (0, 0, 0b001) => Fp16Op::Mla,
            (0, 1, 0b001) => Fp16Op::Mls,
            (0, 0, 0b010) => Fp16Op::Bin(fp16_add),
            (0, 1, 0b010) => Fp16Op::Bin(fp16_sub),
            (0, 0, 0b011) => Fp16Op::Bin(fp16_mulx),
            (0, 0, 0b100) => Fp16Op::Bin(|x, y| fp16_cmp(x, y, 0)), // FCMEQ
            (0, 0, 0b110) => Fp16Op::Bin(fp16_max),
            (0, 1, 0b110) => Fp16Op::Bin(fp16_min),
            (0, 0, 0b111) => Fp16Op::Bin(fp16_recps),
            (0, 1, 0b111) => Fp16Op::Bin(fp16_rsqrts),
            // U=1
            (1, 0, 0b000) => Fp16Op::Pair(fp16_maxnm),
            (1, 1, 0b000) => Fp16Op::Pair(fp16_minnm),
            (1, 0, 0b010) => Fp16Op::Pair(fp16_add),
            (1, 1, 0b010) => Fp16Op::Bin(fp16_abd),
            (1, 0, 0b011) => Fp16Op::Bin(fp16_mul),
            (1, 0, 0b100) => Fp16Op::Bin(|x, y| fp16_cmp(x, y, 1)), // FCMGE
            (1, 1, 0b100) => Fp16Op::Bin(|x, y| fp16_cmp(x, y, 2)), // FCMGT
            (1, 0, 0b101) => Fp16Op::Bin(|x, y| fp16_cmp(x, y, 3)), // FACGE
            (1, 1, 0b101) => Fp16Op::Bin(|x, y| fp16_cmp(x, y, 4)), // FACGT
            (1, 0, 0b110) => Fp16Op::Pair(fp16_max),
            (1, 1, 0b110) => Fp16Op::Pair(fp16_min),
            (1, 0, 0b111) => Fp16Op::Bin(fp16_div),
            _ => return Ok(CpuExit::Undefined(insn)),
        };

        let lane = |v: u128, e: usize| -> u16 { (v >> (e * 16)) as u16 };
        let src1 = self.v[rn];
        let src2 = self.v[rm];
        let acc = self.v[rd];
        let mut dst = 0u128;

        match op {
            Fp16Op::Bin(f) => {
                for e in 0..elements {
                    let r = f(lane(src1, e), lane(src2, e));
                    dst |= (r as u128) << (e * 16);
                }
            }
            Fp16Op::Mla => {
                for e in 0..elements {
                    let r = fp16_mla(lane(acc, e), lane(src1, e), lane(src2, e));
                    dst |= (r as u128) << (e * 16);
                }
            }
            Fp16Op::Mls => {
                for e in 0..elements {
                    let r = fp16_mls(lane(acc, e), lane(src1, e), lane(src2, e));
                    dst |= (r as u128) << (e * 16);
                }
            }
            Fp16Op::Pair(f) => {
                // Pairwise: the lower half of the result comes from adjacent
                // pairs of Vn, the upper half from adjacent pairs of Vm.
                let pairs = elements / 2;
                for i in 0..pairs {
                    let r = f(lane(src1, 2 * i), lane(src1, 2 * i + 1));
                    dst |= (r as u128) << (i * 16);
                }
                for i in 0..pairs {
                    let r = f(lane(src2, 2 * i), lane(src2, 2 * i + 1));
                    dst |= (r as u128) << ((pairs + i) * 16);
                }
            }
        }

        self.v[rd] = dst;
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD FP16 two-reg misc instructions.
    fn exec_simd_fp16_two_reg(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let a = (insn >> 23) & 1; // bit23 sub-group selector (the FP16 "sz" low bit)
        let opcode = (insn >> 12) & 0x1F;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        // For scalar, bit[28]=1
        let is_scalar = ((insn >> 28) & 1) == 1;
        let datasize = if is_scalar {
            2
        } else if q == 1 {
            16
        } else {
            8
        };
        let elements = datasize / 2;

        // Validity: FABS/FNEG (01111), FRINT* (11000/11001) and the vector FSQRT
        // (U=1, 11111) have no SIMD-scalar encoding — their scalar variants live
        // in the floating-point data-processing groups. FRECPX (U=0, 11111) is
        // scalar-only and has no vector form. Reject the mismatched cases.
        if is_scalar {
            if opcode == 0b01111
                || opcode == 0b11000
                || opcode == 0b11001
                || (opcode == 0b11111 && u == 1)
            {
                return Ok(CpuExit::Undefined(insn));
            }
        } else if opcode == 0b11111 && u == 0 {
            return Ok(CpuExit::Undefined(insn));
        }

        let lane = |v: u128, e: usize| -> u16 { (v >> (e * 16)) as u16 };
        let src = self.v[rn];
        let mut dst = 0u128;

        for e in 0..elements {
            let s = lane(src, e);
            // Decode by (U, a=bit23, opcode=bits[16:12]) per the Arm "Advanced
            // SIMD two-register miscellaneous (FP16)" table. FCVT* produce a
            // 16-bit integer lane; SCVTF/UCVTF consume one; the rest are FP16.
            let r: u16 = match (u, a, opcode) {
                // Sign manipulation.
                (0, 1, 0b01111) => s & 0x7FFF, // FABS
                (1, 1, 0b01111) => s ^ 0x8000, // FNEG
                // Square root and reciprocal-family estimates.
                (1, 1, 0b11111) => fp16_sqrt(s),   // FSQRT
                (0, 1, 0b11111) => fp16_recpx(s),  // FRECPX (scalar form)
                (0, 1, 0b11101) => fp16_recpe(s),  // FRECPE
                (1, 1, 0b11101) => fp16_rsqrte(s), // FRSQRTE
                // Compare against zero.
                (0, 1, 0b01100) => fp16_cmp0(s, 0), // FCMGT #0
                (0, 1, 0b01101) => fp16_cmp0(s, 2), // FCMEQ #0
                (0, 1, 0b01110) => fp16_cmp0(s, 4), // FCMLT #0
                (1, 1, 0b01100) => fp16_cmp0(s, 1), // FCMGE #0
                (1, 1, 0b01101) => fp16_cmp0(s, 3), // FCMLE #0
                // Round to integral.
                (0, 0, 0b11000) => fp16_frint(s, 0), // FRINTN
                (0, 0, 0b11001) => fp16_frint(s, 1), // FRINTM
                (0, 1, 0b11000) => fp16_frint(s, 2), // FRINTP
                (0, 1, 0b11001) => fp16_frint(s, 3), // FRINTZ
                (1, 0, 0b11000) => fp16_frint(s, 4), // FRINTA
                (1, 0, 0b11001) => fp16_frint(s, 0), // FRINTX (current mode = RNE)
                (1, 1, 0b11001) => fp16_frint(s, 0), // FRINTI (current mode = RNE)
                // Floating-point to integer (signed).
                (0, 0, 0b11010) => fp16_to_int16(s, true, 0), // FCVTNS
                (0, 0, 0b11011) => fp16_to_int16(s, true, 1), // FCVTMS
                (0, 0, 0b11100) => fp16_to_int16(s, true, 4), // FCVTAS
                (0, 1, 0b11010) => fp16_to_int16(s, true, 2), // FCVTPS
                (0, 1, 0b11011) => fp16_to_int16(s, true, 3), // FCVTZS
                // Floating-point to integer (unsigned).
                (1, 0, 0b11010) => fp16_to_int16(s, false, 0), // FCVTNU
                (1, 0, 0b11011) => fp16_to_int16(s, false, 1), // FCVTMU
                (1, 0, 0b11100) => fp16_to_int16(s, false, 4), // FCVTAU
                (1, 1, 0b11010) => fp16_to_int16(s, false, 2), // FCVTPU
                (1, 1, 0b11011) => fp16_to_int16(s, false, 3), // FCVTZU
                // Integer to floating-point.
                (0, 0, 0b11101) => int16_to_fp16(s, true), // SCVTF
                (1, 0, 0b11101) => int16_to_fp16(s, false), // UCVTF
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            dst |= (r as u128) << (e * 16);
        }

        self.v[rd] = dst;
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD three-different (disparate) instructions.
    /// These are widening/narrowing operations like multiply-accumulate long.
    fn exec_simd_three_different(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let opcode = (insn >> 12) & 0xF;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        let bits = 8u32 << size; // source element (or narrowing destination) size
        let esize = (bits / 8) as usize;
        let dbits = 2 * bits; // doubled (wide) element size
        let part = q as usize; // "2" forms use the upper half of the narrow source
        let signed = u == 0;

        let vn = self.v[rn];
        let vm = self.v[rm];
        let vd = self.v[rd];
        let vn_b = vn.to_le_bytes();
        let vm_b = vm.to_le_bytes();

        match opcode {
            // ---- ADDHN/RADDHN (0100), SUBHN/RSUBHN (0110): add/sub then take
            //      the high half, narrowing 2*bits -> bits. ----
            0b0100 | 0b0110 => {
                if size == 0b11 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let rounding = u == 1;
                let add = opcode == 0b0100;
                let elements = 64 / bits as usize;
                let dmask = elem_mask_u128(dbits);
                let mut packed = 0u64;
                for e in 0..elements {
                    let a = (vn >> (e * dbits as usize)) & dmask;
                    let b = (vm >> (e * dbits as usize)) & dmask;
                    let mut s = if add {
                        a.wrapping_add(b) & dmask
                    } else {
                        a.wrapping_sub(b) & dmask
                    };
                    if rounding {
                        s = s.wrapping_add(1u128 << (bits - 1)) & dmask;
                    }
                    let narrowed = ((s >> bits) & elem_mask_u128(bits)) as u64;
                    packed |= (narrowed & elem_mask(bits)) << (e * bits as usize);
                }
                let mut bytes = vd.to_le_bytes();
                bytes[part * 8..part * 8 + 8].copy_from_slice(&packed.to_le_bytes());
                if part == 0 {
                    bytes[8..16].copy_from_slice(&[0u8; 8]);
                }
                self.v[rd] = u128::from_le_bytes(bytes);
                Ok(CpuExit::Continue)
            }
            // ---- SADDW/UADDW (0001), SSUBW/USUBW (0011): Vn is already wide. ----
            0b0001 | 0b0011 => {
                if size == 0b11 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let add = opcode == 0b0001;
                let elements = 64 / bits as usize;
                let mut result = 0u128;
                for e in 0..elements {
                    let aw = (vn >> (e * dbits as usize)) & elem_mask_u128(dbits);
                    let awide: i128 = if signed {
                        sext_elem_wide(aw, dbits)
                    } else {
                        aw as i128
                    };
                    let bn = read_elem(&vm_b, part * 8 + e * esize, esize);
                    let bwide: i128 = if signed {
                        sext_elem(bn, bits)
                    } else {
                        uext_elem(bn, bits) as i128
                    };
                    let r = if add { awide + bwide } else { awide - bwide };
                    result |= ((r as u128) & elem_mask_u128(dbits)) << (e * dbits as usize);
                }
                self.v[rd] = result;
                Ok(CpuExit::Continue)
            }
            // ---- Widening L-forms ----
            _ => {
                // PMULL.1Q (size==11) is the only size-3 form.
                if size == 0b11 && opcode != 0b1110 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                if size == 0b11 && opcode == 0b1110 {
                    // PMULL/PMULL2 of 64-bit -> 128-bit polynomial product.
                    if u == 1 {
                        return Err(ArmError::UndefinedInstruction(insn));
                    }
                    let a = (vn >> (part * 64)) as u64;
                    let b = (vm >> (part * 64)) as u64;
                    self.v[rd] = poly_mul_64(a, b);
                    return Ok(CpuExit::Continue);
                }
                // SQDMLAL/SQDMLSL/SQDMULL need a 16- or 32-bit source.
                if matches!(opcode, 0b1001 | 0b1011 | 0b1101) && size == 0b00 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                // PMULL (vector form here) is 8-bit source only.
                if opcode == 0b1110 && size != 0b00 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let elements = 64 / bits as usize;
                let dmask = elem_mask_u128(dbits);
                let mut result = 0u128;
                for e in 0..elements {
                    let off = part * 8 + e * esize;
                    let an = read_elem(&vn_b, off, esize);
                    let bn = read_elem(&vm_b, off, esize);
                    let (av, bv): (i128, i128) = if signed {
                        (sext_elem(an, bits), sext_elem(bn, bits))
                    } else {
                        (uext_elem(an, bits) as i128, uext_elem(bn, bits) as i128)
                    };
                    let dval = ((vd >> (e * dbits as usize)) & dmask) as u64;
                    let r: u128 = match opcode {
                        0b0000 => ((av + bv) as u128) & dmask,         // SADDL/UADDL
                        0b0010 => ((av - bv) as u128) & dmask,         // SSUBL/USUBL
                        0b0111 => (((av - bv).abs()) as u128) & dmask, // SABDL/UABDL
                        0b0101 => {
                            ((sext_elem_wide(dval as u128, dbits) + (av - bv).abs()) as u128)
                                & dmask
                            // SABAL/UABAL
                        }
                        0b1000 => {
                            ((sext_elem_wide(dval as u128, dbits) + av * bv) as u128) & dmask // SMLAL/UMLAL
                        }
                        0b1010 => {
                            ((sext_elem_wide(dval as u128, dbits) - av * bv) as u128) & dmask // SMLSL/UMLSL
                        }
                        0b1100 => ((av * bv) as u128) & dmask, // SMULL/UMULL
                        0b1110 => {
                            if u == 1 {
                                return Err(ArmError::UndefinedInstruction(insn));
                            }
                            poly_mul_wide(an, bn, bits) as u128 & dmask // PMULL (8->16)
                        }
                        0b1001 | 0b1011 | 0b1101 => {
                            // SQDMLAL / SQDMLSL / SQDMULL (signed only).
                            if u == 1 {
                                return Err(ArmError::UndefinedInstruction(insn));
                            }
                            let dmin = -(1i128 << (dbits - 1));
                            let dmax = (1i128 << (dbits - 1)) - 1;
                            let prod = (2 * av * bv).clamp(dmin, dmax);
                            let acc = match opcode {
                                0b1001 => sext_elem_wide(dval as u128, dbits) + prod,
                                0b1011 => sext_elem_wide(dval as u128, dbits) - prod,
                                _ => prod,
                            };
                            (sat_signed_wide(acc, dbits)) & dmask
                        }
                        _ => return Err(ArmError::UndefinedInstruction(insn)),
                    };
                    result |= r << (e * dbits as usize);
                }
                self.v[rd] = result;
                Ok(CpuExit::Continue)
            }
        }
    }

    /// Execute FCADD / FCMLA: floating-point complex add / fused multiply-add
    /// over interleaved (real, imaginary) element pairs (FEAT_FCMA). `is_fcmla`
    /// selects FCMLA (2-bit rotation) vs FCADD (1-bit rotation).
    fn exec_simd_complex(&mut self, insn: u32, is_fcmla: bool) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let size = (insn >> 22) & 0x3;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        // size: 01=f16, 10=f32, 11=f64. size==00 is reserved.
        if size == 0b00 {
            return Ok(CpuExit::Undefined(insn));
        }
        let esize = 8u32 << size; // 16 / 32 / 64
        if esize == 64 && q == 0 {
            return Ok(CpuExit::Undefined(insn)); // a 64-bit complex pair needs 128 bits
        }
        let datasize = if q == 1 { 128 } else { 64 };
        let pairs = datasize / (2 * esize as usize);
        let mask = elem_mask(esize) as u128;
        let op1 = self.v[rn];
        let op2 = self.v[rm];
        let op3 = self.v[rd];
        let elem = |v: u128, idx: usize| -> u64 { ((v >> (idx * esize as usize)) & mask) as u64 };
        let mut result = 0u128;
        for e in 0..pairs {
            let re = 2 * e;
            let im = 2 * e + 1;
            let (a_re, a_im) = (elem(op1, re), elem(op1, im));
            let (b_re, b_im) = (elem(op2, re), elem(op2, im));
            let (r_re, r_im) = if is_fcmla {
                let rot = (insn >> 11) & 0x3;
                let (d_re, d_im) = (elem(op3, re), elem(op3, im));
                // result_re += x_re * y_re; result_im += x_im * y_im.
                let (xr, yr, xi, yi) = match rot {
                    0b00 => (a_re, b_re, a_re, b_im),
                    0b01 => (a_im, fp_neg_bits(b_im, esize), a_im, b_re),
                    0b10 => (
                        a_re,
                        fp_neg_bits(b_re, esize),
                        a_re,
                        fp_neg_bits(b_im, esize),
                    ),
                    _ => (a_im, b_im, a_im, fp_neg_bits(b_re, esize)),
                };
                (
                    fp_muladd_bits(d_re, xr, yr, esize),
                    fp_muladd_bits(d_im, xi, yi, esize),
                )
            } else {
                // FCADD: rot==0 (90deg): re = a_re + (-b_im), im = a_im + b_re.
                //        rot==1 (270deg): re = a_re + b_im, im = a_im + (-b_re).
                let rot = (insn >> 12) & 1;
                let (add_re, add_im) = if rot == 0 {
                    (fp_neg_bits(b_im, esize), b_re)
                } else {
                    (b_im, fp_neg_bits(b_re, esize))
                };
                (
                    fp_add_bits(a_re, add_re, esize),
                    fp_add_bits(a_im, add_im, esize),
                )
            };
            result |= (r_re as u128 & mask) << (re * esize as usize);
            result |= (r_im as u128 & mask) << (im * esize as usize);
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute FCMLA by element: like vector FCMLA, but the Vm complex pair is
    /// selected once by the H:L (f16) / H (f32) index and reused for every lane.
    fn exec_simd_complex_indexed(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let size = (insn >> 22) & 0x3;
        let rot = (insn >> 13) & 0x3;
        let l = (insn >> 21) & 1;
        let m = (insn >> 20) & 1;
        let h = (insn >> 11) & 1;
        let rm = (((insn >> 16) & 0xF) | (m << 4)) as usize; // Vm = M:Rm
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        // Only f16 (size=01) and f32 (size=10) are allocated.
        if size != 0b01 && size != 0b10 {
            return Ok(CpuExit::Undefined(insn));
        }
        let esize = 8u32 << size; // 16 or 32
        let index = if size == 0b01 {
            ((h << 1) | l) as usize
        } else {
            h as usize
        };
        if size == 0b10 && (l == 1 || q == 0) {
            return Ok(CpuExit::Undefined(insn));
        }
        if size == 0b01 && h == 1 && q == 0 {
            return Ok(CpuExit::Undefined(insn));
        }
        let datasize = if q == 1 { 128 } else { 64 };
        let pairs = datasize / (2 * esize as usize);
        let mask = elem_mask(esize) as u128;
        let es = esize as usize;
        let op1 = self.v[rn];
        let op2 = self.v[rm];
        let op3 = self.v[rd];
        let elem = |v: u128, idx: usize| -> u64 { ((v >> (idx * es)) & mask) as u64 };
        let m_re = elem(op2, index * 2);
        let m_im = elem(op2, index * 2 + 1);
        let mut result = 0u128;
        for e in 0..pairs {
            let (a_re, a_im) = (elem(op1, 2 * e), elem(op1, 2 * e + 1));
            let (d_re, d_im) = (elem(op3, 2 * e), elem(op3, 2 * e + 1));
            let (xr, yr, xi, yi) = match rot {
                0b00 => (a_re, m_re, a_re, m_im),
                0b01 => (a_im, fp_neg_bits(m_im, esize), a_im, m_re),
                0b10 => (
                    a_re,
                    fp_neg_bits(m_re, esize),
                    a_re,
                    fp_neg_bits(m_im, esize),
                ),
                _ => (a_im, m_im, a_im, fp_neg_bits(m_re, esize)),
            };
            let r_re = fp_muladd_bits(d_re, xr, yr, esize);
            let r_im = fp_muladd_bits(d_im, xi, yi, esize);
            result |= (r_re as u128 & mask) << (2 * e * es);
            result |= (r_im as u128 & mask) << ((2 * e + 1) * es);
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute SDOT/UDOT/USDOT: the 8-bit -> 32-bit four-way dot product. Each
    /// 32-bit lane accumulates four byte-wise products of the corresponding
    /// Vn/Vm bytes. `op1_signed`/`op2_signed` give the byte signedness:
    /// SDOT = (s,s), UDOT = (u,u), USDOT = (u,s).
    fn exec_simd_dot(
        &mut self,
        insn: u32,
        op1_signed: bool,
        op2_signed: bool,
    ) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let lanes = if q == 1 { 4 } else { 2 }; // 32-bit accumulator lanes
        let op1 = self.v[rn];
        let op2 = self.v[rm];
        let byte = |v: u128, sh: usize, signed: bool| -> i64 {
            let b = (v >> sh) as u8;
            if signed { b as i8 as i64 } else { b as i64 }
        };
        let mut result = self.v[rd];
        for e in 0..lanes {
            let mut res: i64 = 0;
            for i in 0..4 {
                let sh = (4 * e + i) * 8;
                res += byte(op1, sh, op1_signed) * byte(op2, sh, op2_signed);
            }
            let lane = (result >> (e * 32)) as u32;
            let updated = (lane as i64).wrapping_add(res) as u32;
            result = (result & !(0xFFFF_FFFFu128 << (e * 32))) | ((updated as u128) << (e * 32));
        }
        if q == 0 {
            result &= 0xFFFF_FFFF_FFFF_FFFF;
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute the AdvSIMD "three-same-extra" (bit21==0) ops: SQRDMLAH/SQRDMLSH
    /// (FEAT_RDM; vector + scalar) and SMMLA/UMMLA/USMMLA (FEAT_I8MM int8 2x2
    /// matrix multiply-accumulate; .4s,.16b,.16b, Q==1 only).
    fn exec_simd_three_same_extra(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op_bits = (insn >> 24) & 0x1F;
        let scalar = op_bits == 0b11110;
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let lo6 = (insn >> 10) & 0x3F;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        if lo6 == 0b101001 || lo6 == 0b101011 {
            // SMMLA(U=0)/UMMLA(U=1)/USMMLA(U=0,101011) int8 2x2 matrix MAC.
            if scalar || q == 0 || size != 0b10 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let (n_signed, m_signed) = if lo6 == 0b101011 {
                (false, true) // USMMLA: Vn unsigned, Vm signed
            } else if u == 1 {
                (false, false) // UMMLA
            } else {
                (true, true) // SMMLA
            };
            let n = self.v[rn].to_le_bytes();
            let m = self.v[rm].to_le_bytes();
            let a = self.v[rd];
            let mut res = 0u128;
            for i in 0..2 {
                for j in 0..2 {
                    let mut acc = (a >> ((i * 2 + j) * 32)) as u32 as i32 as i64;
                    for k in 0..8 {
                        let nv = if n_signed {
                            n[i * 8 + k] as i8 as i64
                        } else {
                            n[i * 8 + k] as i64
                        };
                        let mv = if m_signed {
                            m[j * 8 + k] as i8 as i64
                        } else {
                            m[j * 8 + k] as i64
                        };
                        acc += nv * mv;
                    }
                    res |= (acc as u32 as u128) << ((i * 2 + j) * 32);
                }
            }
            self.v[rd] = res;
            return Ok(CpuExit::Continue);
        }

        // SQRDMLAH (100001) / SQRDMLSH (100011): U==1, 16- or 32-bit elements.
        if u != 1 || size == 0b00 || size == 0b11 {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        let sub = lo6 == 0b100011;
        let bits = 8u32 << size;
        let esize = (bits / 8) as usize;
        let elements = if scalar {
            1
        } else if q == 1 {
            16 / esize
        } else {
            8 / esize
        };
        let n = self.v[rn].to_le_bytes();
        let m = self.v[rm].to_le_bytes();
        let a = self.v[rd].to_le_bytes();
        let mut dst = [0u8; 16];
        for e in 0..elements {
            let off = e * esize;
            let prod = sext_elem(read_elem(&n, off, esize), bits)
                * sext_elem(read_elem(&m, off, esize), bits);
            let rounded = (prod * 2 + (1i128 << (bits - 1))) >> bits;
            let acc = sext_elem(read_elem(&a, off, esize), bits);
            let r = sat_signed(if sub { acc - rounded } else { acc + rounded }, bits);
            write_elem(&mut dst, off, esize, r);
        }
        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute USDOT/SUDOT by element (FEAT_I8MM). The index (H:L) selects a
    /// 4-byte group of Vm reused for every lane. `op1_signed`/`op2_signed` give
    /// the Vn/Vm byte signedness (USDOT = (false,true), SUDOT = (true,false)).
    fn exec_simd_dot_indexed_mixed(
        &mut self,
        insn: u32,
        op1_signed: bool,
        op2_signed: bool,
    ) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let l = (insn >> 21) & 1;
        let m = (insn >> 20) & 1;
        let h = (insn >> 11) & 1;
        let rm = (((insn >> 16) & 0xF) | (m << 4)) as usize; // Vm = M:Rm
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let index = ((h << 1) | l) as usize; // H:L, selects a 32-bit group
        let lanes = if q == 1 { 4 } else { 2 };
        let op1 = self.v[rn];
        let op2 = self.v[rm];
        let byte = |v: u128, sh: usize, signed: bool| -> i64 {
            let b = (v >> sh) as u8;
            if signed { b as i8 as i64 } else { b as i64 }
        };
        let base = index * 4;
        let mut result = self.v[rd];
        for e in 0..lanes {
            let mut res: i64 = 0;
            for i in 0..4 {
                res +=
                    byte(op1, (4 * e + i) * 8, op1_signed) * byte(op2, (base + i) * 8, op2_signed);
            }
            let lane = (result >> (e * 32)) as u32;
            let updated = (lane as i64).wrapping_add(res) as u32;
            result = (result & !(0xFFFF_FFFFu128 << (e * 32))) | ((updated as u128) << (e * 32));
        }
        if q == 0 {
            result &= 0xFFFF_FFFF_FFFF_FFFF;
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute BFMLALB/BFMLALT (FEAT_BF16): widening bf16 -> f32 fused
    /// multiply-accumulate. Q (bit30) selects the Bottom (0) or Top (1) bf16 of
    /// each f32 pair. The result is always a full 128-bit, 4-lane f32 vector.
    fn exec_simd_bfmlal(&mut self, insn: u32, is_indexed: bool) -> Result<CpuExit, ArmError> {
        let sel = ((insn >> 30) & 1) as usize; // Q: 0=B (low 16), 1=T (high 16)
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let op1 = self.v[rn];
        let op3 = self.v[rd];
        let bf16 = |v: u128, lane: usize| -> u16 { (v >> (lane * 16)) as u16 };
        let (op2, idx): (u128, Option<usize>) = if is_indexed {
            let l = (insn >> 21) & 1;
            let m = (insn >> 20) & 1;
            let h = (insn >> 11) & 1;
            let rm = ((insn >> 16) & 0xF) as usize; // 4-bit, V0..V15
            (self.v[rm], Some(((h << 2) | (l << 1) | m) as usize)) // index = H:L:M
        } else {
            let rm = ((insn >> 16) & 0x1F) as usize;
            (self.v[rm], None)
        };
        let mut result = 0u128;
        for e in 0..4 {
            let b1 = bf16(op1, 2 * e + sel);
            let b2 = match idx {
                // The by-element form selects a single bf16 (Vm.H[index]); the
                // vector form takes the B/T half of pair e.
                Some(ix) => bf16(op2, ix),
                None => bf16(op2, 2 * e + sel),
            };
            let a = (op3 >> (e * 32)) as u32;
            // Single-rounded fused multiply-add (FPMulAdd) with ARM-correct NaN
            // selection (addend first); bf16 widens to f32 by a 16-bit shift.
            let r = fp_muladd_bits(
                a as u64,
                (b1 as u32 as u64) << 16,
                (b2 as u32 as u64) << 16,
                32,
            ) as u32;
            result |= (r as u128) << (e * 32);
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute BFDOT (FEAT_BF16): 2-way bf16 dot product accumulated into f32
    /// lanes. The two bf16 products and the f32 accumulator are summed in
    /// unrounded precision and rounded once to f32 with round-to-odd (the
    /// standard FPCR.EBF==0 path).
    fn exec_simd_bfdot(&mut self, insn: u32, is_indexed: bool) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let lanes = if q == 1 { 4 } else { 2 };
        let op1 = self.v[rn];
        let op3 = self.v[rd];
        let bf16 = |v: u128, lane: usize| -> u16 { (v >> (lane * 16)) as u16 };
        let (op2, idx): (u128, Option<usize>) = if is_indexed {
            let l = (insn >> 21) & 1;
            let m = (insn >> 20) & 1;
            let h = (insn >> 11) & 1;
            let rm = (((insn >> 16) & 0xF) | (m << 4)) as usize; // Vm = M:Rm
            (self.v[rm], Some(((h << 1) | l) as usize)) // index H:L selects a bf16 pair
        } else {
            let rm = ((insn >> 16) & 0x1F) as usize;
            (self.v[rm], None)
        };
        let _ = &bf16;
        let mut result = self.v[rd];
        for e in 0..lanes {
            let acc_bits = (op3 >> (e * 32)) as u32;
            let n_pair = (op1 >> (e * 32)) as u32;
            let m_pair = match idx {
                Some(ix) => (op2 >> (ix * 32)) as u32,
                None => (op2 >> (e * 32)) as u32,
            };
            let r = bfdotadd_ebf0(acc_bits, n_pair, m_pair);
            result = (result & !(0xFFFF_FFFFu128 << (e * 32))) | ((r as u128) << (e * 32));
        }
        if q == 0 {
            result &= 0xFFFF_FFFF_FFFF_FFFF;
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute BFCVTN/BFCVTN2 (FEAT_BF16): narrow 4 f32 lanes to 4 bf16 lanes
    /// (round-to-nearest-even). BFCVTN (Q=0) writes the low 64 bits and zeroes
    /// the high half; BFCVTN2 (Q=1) writes the high 64 bits, preserving the low.
    fn exec_simd_bfcvtn(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let op = self.v[rn];
        let mut narrowed = 0u64;
        for e in 0..4 {
            let bf = f32_to_bf16((op >> (e * 32)) as u32);
            narrowed |= (bf as u64) << (e * 16);
        }
        if q == 0 {
            self.v[rd] = narrowed as u128;
        } else {
            self.v[rd] = (self.v[rd] & 0xFFFF_FFFF_FFFF_FFFF) | ((narrowed as u128) << 64);
        }
        Ok(CpuExit::Continue)
    }

    /// Execute BFMMLA (FEAT_BF16): 2x4-by-4x2 bf16 matrix multiply accumulating
    /// into a 2x2 f32 matrix, with the same round-to-odd accumulation as BFDOT.
    fn exec_simd_bfmmla(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let op1 = self.v[rn];
        let op2 = self.v[rm];
        let acc = self.v[rd];
        let mut result = 0u128;
        for i in 0..2 {
            for j in 0..2 {
                let lane = 2 * i + j;
                let acc_bits = (acc >> (lane * 32)) as u32;
                // Two bfdotadd steps over the k=0,1 and k=2,3 bf16 pairs, exactly
                // as qemu gvec_bfmmla processes each output lane.
                let n01 = (op1 >> ((4 * i) * 16)) as u32; // bf16 lanes 4i, 4i+1
                let m01 = (op2 >> ((4 * j) * 16)) as u32; // bf16 lanes 4j, 4j+1
                let n23 = (op1 >> ((4 * i + 2) * 16)) as u32; // lanes 4i+2, 4i+3
                let m23 = (op2 >> ((4 * j + 2) * 16)) as u32; // lanes 4j+2, 4j+3
                let s = bfdotadd_ebf0(acc_bits, n01, m01);
                let r = bfdotadd_ebf0(s, n23, m23);
                result |= (r as u128) << (lane * 32);
            }
        }
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute cryptographic operations (AES, SHA, SM3, SM4).
    /// For now, this is a stub that allows the instruction to execute.
    fn exec_crypto(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        // SHA512 (FEAT_SHA512) and SHA3 (FEAT_SHA3) 64-bit-lane crypto: 0xCE.
        if (insn >> 24) & 0xFF == 0xCE {
            let rm = ((insn >> 16) & 0x1F) as usize;
            let ra = ((insn >> 10) & 0x1F) as usize;
            let grp = (insn >> 21) & 0x7; // bits[23:21]
            let o = (insn >> 10) & 0x3F; // bits[15:10]
            let lanes = |v: u128| (v as u64, (v >> 64) as u64);
            let pack = |a: u64, b: u64| (a as u128) | ((b as u128) << 64);
            if grp == 0b011 && o == 0b100000 {
                // SHA512H
                let (d0, d1) = lanes(self.v[rd]);
                let (m0, m1) = lanes(self.v[rm]);
                let (n0, n1) = lanes(self.v[rn]);
                let s1 = |x: u64| x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41);
                let cho = |x: u64, y: u64, z: u64| (x & (y ^ z)) ^ z;
                let nd1 = d1.wrapping_add(s1(m1)).wrapping_add(cho(m1, n0, n1));
                let t = nd1.wrapping_add(m0);
                let nd0 = d0.wrapping_add(s1(t)).wrapping_add(cho(t, m1, n0));
                self.v[rd] = pack(nd0, nd1);
                return Ok(CpuExit::Continue);
            }
            if grp == 0b011 && o == 0b100001 {
                // SHA512H2
                let (d0, d1) = lanes(self.v[rd]);
                let (m0, m1) = lanes(self.v[rm]);
                let (n0, _n1) = lanes(self.v[rn]);
                let s0 = |x: u64| x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39);
                let maj = |x: u64, y: u64, z: u64| (x & y) | ((x | y) & z);
                let nd1 = d1.wrapping_add(s0(m0)).wrapping_add(maj(n0, m1, m0));
                let nd0 = d0.wrapping_add(s0(nd1)).wrapping_add(maj(nd1, m0, m1));
                self.v[rd] = pack(nd0, nd1);
                return Ok(CpuExit::Continue);
            }
            if grp == 0b110 && o == 0b100000 {
                // SHA512SU0
                let (d0, d1) = lanes(self.v[rd]);
                let (n0, _n1) = lanes(self.v[rn]);
                let sig0 = |x: u64| x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7);
                self.v[rd] = pack(d0.wrapping_add(sig0(d1)), d1.wrapping_add(sig0(n0)));
                return Ok(CpuExit::Continue);
            }
            if grp == 0b011 && o == 0b100010 {
                // SHA512SU1
                let (d0, d1) = lanes(self.v[rd]);
                let (m0, m1) = lanes(self.v[rm]);
                let (n0, n1) = lanes(self.v[rn]);
                let sig1 = |x: u64| x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6);
                self.v[rd] = pack(
                    d0.wrapping_add(sig1(n0)).wrapping_add(m0),
                    d1.wrapping_add(sig1(n1)).wrapping_add(m1),
                );
                return Ok(CpuExit::Continue);
            }
            if grp == 0b011 && o == 0b100011 {
                // RAX1: Vd[i] = Vn[i] ^ ROL64(Vm[i], 1)
                let (n0, n1) = lanes(self.v[rn]);
                let (m0, m1) = lanes(self.v[rm]);
                self.v[rd] = pack(n0 ^ m0.rotate_left(1), n1 ^ m1.rotate_left(1));
                return Ok(CpuExit::Continue);
            }
            if grp == 0b100 {
                // XAR: Vd[i] = ROR64(Vn[i] ^ Vm[i], imm6)
                let imm = o; // bits[15:10]
                let (n0, n1) = lanes(self.v[rn]);
                let (m0, m1) = lanes(self.v[rm]);
                self.v[rd] = pack((n0 ^ m0).rotate_right(imm), (n1 ^ m1).rotate_right(imm));
                return Ok(CpuExit::Continue);
            }
            if (grp == 0b000 || grp == 0b001) && (insn >> 15) & 1 == 0 {
                // EOR3 (grp 000): Vd = Vn ^ Vm ^ Va.  BCAX (grp 001): Vn ^ (Vm & ~Va).
                let (n, m, a) = (self.v[rn], self.v[rm], self.v[ra]);
                self.v[rd] = if grp == 0b000 {
                    n ^ m ^ a
                } else {
                    n ^ (m & !a)
                };
                return Ok(CpuExit::Continue);
            }
        }

        // AES single-block operations: bits[31:24]=0x4E, opcode bits[16:12].
        if (insn >> 24) & 0xFF == 0x4E {
            let opcode = (insn >> 12) & 0x1F;
            match opcode {
                0b00100 => {
                    // AESE: ShiftRows(SubBytes(Vd EOR Vn))
                    let st = self.v[rd] ^ self.v[rn];
                    self.v[rd] = aes_sub_bytes(aes_shift_rows(st, false), false);
                    return Ok(CpuExit::Continue);
                }
                0b00101 => {
                    // AESD: InvShiftRows then InvSubBytes of (Vd EOR Vn)
                    let st = self.v[rd] ^ self.v[rn];
                    self.v[rd] = aes_sub_bytes(aes_shift_rows(st, true), true);
                    return Ok(CpuExit::Continue);
                }
                0b00110 => {
                    // AESMC
                    self.v[rd] = aes_mix_columns(self.v[rn], false);
                    return Ok(CpuExit::Continue);
                }
                0b00111 => {
                    // AESIMC
                    self.v[rd] = aes_mix_columns(self.v[rn], true);
                    return Ok(CpuExit::Continue);
                }
                _ => {}
            }
        }

        let rm = ((insn >> 16) & 0x1F) as usize;

        // SHA-1 / SHA-256 (bits[31:24]=0x5E).
        if (insn >> 24) & 0xFF == 0x5E {
            // Two-register SHA: bits[21:17]==10100, opcode at bits[16:12].
            if (insn >> 17) & 0x1F == 0b10100 {
                let opcode = (insn >> 12) & 0x1F;
                match opcode {
                    0b00000 => {
                        // SHA1H Sd, Sn: ROL(Sn, 30) on the low 32 bits.
                        self.v[rd] = (self.v[rn] as u32).rotate_left(30) as u128;
                        return Ok(CpuExit::Continue);
                    }
                    0b00001 => {
                        // SHA1SU1 Vd.4S, Vn.4S
                        let op1 = self.v[rd];
                        let op2 = self.v[rn];
                        let t = op1 ^ (op2 >> 32);
                        let t0 = sha_elem(t, 0).rotate_left(1);
                        let t1 = sha_elem(t, 1).rotate_left(1);
                        let t2 = sha_elem(t, 2).rotate_left(1);
                        let t3 = sha_elem(t, 3).rotate_left(1) ^ sha_elem(t, 0).rotate_left(2);
                        let mut r = 0u128;
                        sha_set_elem(&mut r, 0, t0);
                        sha_set_elem(&mut r, 1, t1);
                        sha_set_elem(&mut r, 2, t2);
                        sha_set_elem(&mut r, 3, t3);
                        self.v[rd] = r;
                        return Ok(CpuExit::Continue);
                    }
                    0b00010 => {
                        // SHA256SU0 Vd.4S, Vn.4S
                        let x = self.v[rd];
                        let y = self.v[rn];
                        let t = (y << 96) | (x >> 32); // Y<31:0> : X<127:32>
                        let mut r = 0u128;
                        for e in 0..4 {
                            let elt = sha_elem(t, e);
                            let s = elt.rotate_right(7) ^ elt.rotate_right(18) ^ (elt >> 3);
                            sha_set_elem(&mut r, e, s.wrapping_add(sha_elem(x, e)));
                        }
                        self.v[rd] = r;
                        return Ok(CpuExit::Continue);
                    }
                    _ => {}
                }
            } else if (insn >> 21) & 7 == 0 && (insn >> 10) & 3 == 0 {
                // Three-register SHA: opcode at bits[14:12].
                let opcode = (insn >> 12) & 0x7;
                match opcode {
                    0b000 => {
                        // SHA1C
                        self.v[rd] =
                            sha1_hash(self.v[rd], self.v[rn] as u32, self.v[rm], sha_choose);
                        return Ok(CpuExit::Continue);
                    }
                    0b001 => {
                        // SHA1P
                        self.v[rd] =
                            sha1_hash(self.v[rd], self.v[rn] as u32, self.v[rm], sha_parity);
                        return Ok(CpuExit::Continue);
                    }
                    0b010 => {
                        // SHA1M
                        self.v[rd] =
                            sha1_hash(self.v[rd], self.v[rn] as u32, self.v[rm], sha_majority);
                        return Ok(CpuExit::Continue);
                    }
                    0b011 => {
                        // SHA1SU0 Vd.4S, Vn.4S, Vm.4S
                        let op1 = self.v[rd];
                        let op2 = self.v[rn];
                        let op3 = self.v[rm];
                        // result = (Vn<63:0> : Vd<127:64>) EOR Vd EOR Vm
                        let r = ((op2 << 64) | (op1 >> 64)) ^ op1 ^ op3;
                        self.v[rd] = r;
                        return Ok(CpuExit::Continue);
                    }
                    0b100 => {
                        // SHA256H Qd, Qn, Vm: SHA256hash(Vd, Vn, Vm, part1=true)
                        self.v[rd] = sha256_hash(self.v[rd], self.v[rn], self.v[rm], true);
                        return Ok(CpuExit::Continue);
                    }
                    0b101 => {
                        // SHA256H2 Qd, Qn, Vm: SHA256hash(Vn, Vd, Vm, part1=false)
                        self.v[rd] = sha256_hash(self.v[rn], self.v[rd], self.v[rm], false);
                        return Ok(CpuExit::Continue);
                    }
                    0b110 => {
                        // SHA256SU1 Vd.4S, Vn.4S, Vm.4S
                        let x = self.v[rd];
                        let y = self.v[rn];
                        let z = self.v[rm];
                        let t0 = (z << 96) | (y >> 32); // Z<31:0> : Y<127:32>
                        let mut r = 0u128;
                        // e = 0,1 use T1 = Z<127:64>
                        for e in 0..2 {
                            let elt = sha_elem(z >> 64, e); // Z<127:64> element e
                            let s = elt.rotate_right(17) ^ elt.rotate_right(19) ^ (elt >> 10);
                            let v = s.wrapping_add(sha_elem(x, e)).wrapping_add(sha_elem(t0, e));
                            sha_set_elem(&mut r, e, v);
                        }
                        // e = 2,3 use T1 = result<63:0>
                        for e in 2..4 {
                            let elt = sha_elem(r, e - 2); // result<63:0> element (e-2)
                            let s = elt.rotate_right(17) ^ elt.rotate_right(19) ^ (elt >> 10);
                            let v = s.wrapping_add(sha_elem(x, e)).wrapping_add(sha_elem(t0, e));
                            sha_set_elem(&mut r, e, v);
                        }
                        self.v[rd] = r;
                        return Ok(CpuExit::Continue);
                    }
                    _ => {}
                }
            }
        }

        // SM4 (bits[31:24]==0xCE).
        if (insn >> 24) & 0xFF == 0xCE {
            // SM4E Vd.4S, Vn.4S: 11001110 11000000 100001 Rn Rd.
            if (insn >> 16) & 0xFF == 0xC0 && (insn >> 10) & 0x3F == 0b100001 {
                self.v[rd] = sm4_rounds(self.v[rd], self.v[rn], true);
                return Ok(CpuExit::Continue);
            }
            // SM4EKEY Vd.4S, Vn.4S, Vm.4S: 11001110 011 Rm 110010 Rn Rd.
            if (insn >> 21) & 0x7 == 0b011 && (insn >> 10) & 0x3F == 0b110010 {
                self.v[rd] = sm4_rounds(self.v[rn], self.v[rm], false);
                return Ok(CpuExit::Continue);
            }

            // SM3 group.
            let grp = (insn >> 21) & 0x7;
            if grp == 0b010 {
                if (insn >> 15) & 1 == 0 {
                    // SM3SS1 Vd.4S, Vn.4S, Vm.4S, Va.4S (Va = Ra at bits[14:10]).
                    let ra = ((insn >> 10) & 0x1F) as usize;
                    let t = (self.v[rn] >> 96) as u32;
                    let val = t
                        .rotate_left(12)
                        .wrapping_add((self.v[rm] >> 96) as u32)
                        .wrapping_add((self.v[ra] >> 96) as u32)
                        .rotate_left(7);
                    self.v[rd] = (val as u128) << 96;
                    return Ok(CpuExit::Continue);
                } else if (insn >> 14) & 0x3 == 0b10 {
                    // SM3TT1A/SM3TT1B/SM3TT2A/SM3TT2B (sel = bits[11:10], i = imm2).
                    let i = (insn >> 12) & 0x3;
                    let sel = (insn >> 10) & 0x3;
                    self.v[rd] = sm3_tt(self.v[rd], self.v[rn], self.v[rm], i, sel);
                    return Ok(CpuExit::Continue);
                }
            } else if grp == 0b011 {
                if (insn >> 10) & 0x3F == 0b110000 {
                    self.v[rd] = sm3_partw1(self.v[rd], self.v[rn], self.v[rm]);
                    return Ok(CpuExit::Continue);
                }
                if (insn >> 10) & 0x3F == 0b110001 {
                    self.v[rd] = sm3_partw2(self.v[rd], self.v[rn], self.v[rm]);
                    return Ok(CpuExit::Continue);
                }
            }
        }

        // Any remaining crypto encoding is unallocated.
        Ok(CpuExit::Undefined(insn))
    }

    /// Execute SIMD across lanes (reduction operations).
    fn exec_simd_across_lanes(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let opcode = (insn >> 12) & 0x1F;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        // ---- Floating-point reductions: FMAXNMV/FMINNMV (0b01100),
        //      FMAXV/FMINV (0b01111). U==1, f32 lanes only. bit23 picks min. ----
        if opcode == 0b01100 || opcode == 0b01111 {
            // FP max/min across lanes: f32 (U==1, 4S) or FP16 (U==0, .4h/.8h).
            // bit23 (size high) selects min; opcode 01100=NM variant. Reduced via
            // the ARM-correct combine (NaN propagation, sign-of-zero, sNaN quiet).
            let nm = opcode == 0b01100;
            let is_min = (size >> 1) & 1 == 1;
            let kind = match (nm, is_min) {
                (false, false) => FpKind::Max,
                (false, true) => FpKind::Min,
                (true, false) => FpKind::MaxNm,
                (true, true) => FpKind::MinNm,
            };
            let vn = self.v[rn];
            // ARM Reduce() is a recursive split-in-half tree (sve_fp_tree_reduce),
            // NOT a sequential fold — the order is observable when a NaN is
            // present (sNaN propagation / which numeric lane survives).
            let (esize, nlanes) = if u == 1 {
                if size & 1 != 0 || q == 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                (4usize, 4usize) // f32, 4S
            } else {
                (2usize, if q == 1 { 8 } else { 4 }) // FP16, .8h/.4h
            };
            let buf: Vec<u64> = (0..nlanes)
                .map(|e| (vn >> (e * esize * 8)) as u64 & elem_mask((esize * 8) as u32))
                .collect();
            let r = sve_fp_tree_reduce(&buf, kind, esize);
            self.v[rd] = (r & elem_mask((esize * 8) as u32)) as u128;
            return Ok(CpuExit::Continue);
        }

        let bits = 8u32 << size;
        let esize = (bits / 8) as usize;
        let datasize = if q == 1 { 16 } else { 8 };
        let elements = datasize / esize;
        let src = self.v[rn].to_le_bytes();

        // Reductions are defined for 8B/16B/4H/8H and (Q==1) 4S; never 64-bit,
        // and 8B/4H also exclude the single-element degenerate cases.
        let valid_size = match size {
            0b00 => true,   // 8B / 16B
            0b01 => true,   // 4H / 8H
            0b10 => q == 1, // 4S only
            _ => false,
        };
        if !valid_size {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let (result, result_bits): (u64, u32) = match opcode {
            0b11011 => {
                // ADDV
                let mut acc = 0u64;
                for e in 0..elements {
                    acc = acc.wrapping_add(read_elem(&src, e * esize, esize));
                }
                (acc & elem_mask(bits), bits)
            }
            0b00011 => {
                // SADDLV (U=0) / UADDLV (U=1) -- widening sum across lanes.
                let mut acc = 0i128;
                for e in 0..elements {
                    let v = read_elem(&src, e * esize, esize);
                    acc += if u == 0 {
                        sext_elem(v, bits)
                    } else {
                        uext_elem(v, bits) as i128
                    };
                }
                ((acc as u64) & elem_mask(2 * bits), 2 * bits)
            }
            0b01010 => {
                // SMAXV (U=0) / UMAXV (U=1)
                let mut acc = read_elem(&src, 0, esize);
                for e in 1..elements {
                    let v = read_elem(&src, e * esize, esize);
                    acc = if u == 0 {
                        if sext_elem(v, bits) > sext_elem(acc, bits) {
                            v
                        } else {
                            acc
                        }
                    } else if uext_elem(v, bits) > uext_elem(acc, bits) {
                        v
                    } else {
                        acc
                    };
                }
                (acc & elem_mask(bits), bits)
            }
            0b11010 => {
                // SMINV (U=0) / UMINV (U=1)
                let mut acc = read_elem(&src, 0, esize);
                for e in 1..elements {
                    let v = read_elem(&src, e * esize, esize);
                    acc = if u == 0 {
                        if sext_elem(v, bits) < sext_elem(acc, bits) {
                            v
                        } else {
                            acc
                        }
                    } else if uext_elem(v, bits) < uext_elem(acc, bits) {
                        v
                    } else {
                        acc
                    };
                }
                (acc & elem_mask(bits), bits)
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };

        self.v[rd] = (result as u128) & elem_mask_u128(result_bits);
        Ok(CpuExit::Continue)
    }

    /// AdvSIMD scalar pairwise: reduce the two elements of a vector to a scalar.
    /// ADDP (int, D only); FADDP/FMAXP/FMINP/FMAXNMP/FMINNMP for f16 (U=0),
    /// f32 (U=1, bit22=0) or f64 (U=1, bit22=1). bit23 selects min for the
    /// max/min forms. Writes lane 0, zeroing the rest.
    fn exec_simd_scalar_pairwise(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let opcode = (insn >> 12) & 0x1F;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let vn = self.v[rn];

        if opcode == 0b11011 {
            // ADDP (scalar, .2d -> Dd).
            if u != 0 || size != 0b11 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            self.v[rd] = (vn as u64).wrapping_add((vn >> 64) as u64) as u128;
            return Ok(CpuExit::Continue);
        }

        let (faddp, nm) = match opcode {
            0b01101 => (true, false),  // FADDP
            0b01100 => (false, true),  // FMAXNMP / FMINNMP
            0b01111 => (false, false), // FMAXP / FMINP
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };
        let min = (size >> 1) & 1 == 1;
        let esize = if u == 0 {
            2usize // FP16
        } else if size & 1 == 0 {
            4 // f32
        } else {
            8 // f64
        };
        let kind = if faddp {
            FpKind::Add
        } else {
            match (nm, min) {
                (false, false) => FpKind::Max,
                (false, true) => FpKind::Min,
                (true, false) => FpKind::MaxNm,
                (true, true) => FpKind::MinNm,
            }
        };
        let mask = elem_mask((esize * 8) as u32);
        let e0 = vn as u64 & mask;
        let e1 = (vn >> (esize * 8)) as u64 & mask;
        let r = sve_fp_combine(kind, esize, e0, e1);
        self.v[rd] = (r & mask) as u128;
        Ok(CpuExit::Continue)
    }

    /// Execute the SIMD modified-immediate group: MOVI, MVNI, ORR (imm),
    /// BIC (imm) and FMOV (vector immediate).
    fn exec_simd_modified_imm(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let op = (insn >> 29) & 1;
        let cmode = (insn >> 12) & 0xF;
        let rd = (insn & 0x1F) as usize;
        // imm8 = abc:defgh
        let abc = (insn >> 16) & 0x7;
        let defgh = (insn >> 5) & 0x1F;
        let imm8 = ((abc << 5) | defgh) as u8;

        // FP16 FMOV vector immediate (FEAT_FP16): cmode==1111, op==0, o2(bit11)==1.
        // Broadcast the 8-bit half-precision immediate to .4h (Q=0) / .8h (Q=1).
        if cmode == 0b1111 && op == 0 && (insn >> 11) & 1 == 1 {
            let h = vfp_expand_imm_f16(imm8) as u128;
            let lane = h | (h << 16) | (h << 32) | (h << 48);
            self.v[rd] = if q == 1 { lane | (lane << 64) } else { lane };
            return Ok(CpuExit::Continue);
        }

        // Some (op, cmode, Q) combinations are UNDEFINED.
        //  - FMOV f64 (op=1, cmode=1111) requires Q==1.
        if op == 1 && cmode == 0b1111 && q == 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        //  - op=1, cmode=1110 is MOVI(64-bit); op=1, cmode=0xx0/10x0 is MVNI;
        //    these are all allocated. The only fully-unallocated case in this
        //    group is handled by the cmode match returning a defined value.

        let imm64 = adv_simd_expand_imm(op, cmode, imm8);

        // ORR/BIC immediate: cmode = 0xx1 or 10x1.
        let orr_bic = (cmode & 1) == 1 && (cmode >> 1) < 0b110;
        if orr_bic {
            let imm128 = (imm64 as u128) | ((imm64 as u128) << 64);
            let cur = self.v[rd];
            let r = if op == 0 { cur | imm128 } else { cur & !imm128 };
            self.v[rd] = if q == 1 { r } else { r & elem_mask_u128(64) };
            return Ok(CpuExit::Continue);
        }

        // MOVI / MVNI / FMOV. MVNI inverts for op=1 except the cmode=1110
        // (MOVI 64-bit) and cmode=1111 (FMOV) special cases.
        let val = if op == 1 && cmode != 0b1110 && cmode != 0b1111 {
            !imm64
        } else {
            imm64
        };
        let result = if q == 1 {
            (val as u128) | ((val as u128) << 64)
        } else {
            val as u128
        };
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD shift by immediate.
    fn exec_simd_shift_imm(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let immh = (insn >> 19) & 0xF;
        let immb = (insn >> 16) & 0x7;
        let opcode = (insn >> 11) & 0x1F;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        // Scalar AdvSIMD shift-by-immediate has top byte 0x5F/0x7F (&0x1F==11111),
        // distinct from the scalar two-reg-misc class (0x5E/0x7E, 11110).
        let scalar = ((insn >> 24) & 0x1F) == 0b11111;

        // immh==0 belongs to the modified-immediate / other encoding.
        if immh == 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        let size_idx = if immh & 0b1000 != 0 {
            3
        } else if immh & 0b0100 != 0 {
            2
        } else if immh & 0b0010 != 0 {
            1
        } else {
            0
        };
        let bits = 8u32 << size_idx; // element size the shift operates on
        let immhimmb = ((immh << 3) | immb) as u32;

        match opcode {
            // ---- Same element-size shifts ----
            0b00000 | 0b00010 | 0b00100 | 0b00110 | 0b01000 | 0b01010 | 0b01100 | 0b01110 => {
                // A few opcode slots are only allocated for one value of U.
                let valid = match opcode {
                    0b01000 => u == 1, // SRI (U==1 only)
                    0b01100 => u == 1, // SQSHLU (U==1 only)
                    _ => true,
                };
                if !valid {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                // 64-bit elements need 2D (Q==1) in the vector form.
                if bits == 64 && q == 0 && !scalar {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let is_left = matches!(opcode, 0b01010 | 0b01100 | 0b01110);
                let shift = if is_left {
                    immhimmb - bits
                } else {
                    2 * bits - immhimmb
                };
                let esize = (bits / 8) as usize;
                let datasize = if scalar {
                    esize
                } else if q == 1 {
                    16
                } else {
                    8
                };
                let elements = datasize / esize;
                let src = self.v[rn].to_le_bytes();
                let old = self.v[rd].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let off = e * esize;
                    let a = read_elem(&src, off, esize);
                    let d = read_elem(&old, off, esize);
                    let r = adv_simd_shift_imm_elem(u, opcode, bits, shift, a, d);
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[rd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }
            // ---- Widening left shift: SSHLL / USHLL (SXTL/UXTL when shift==0) ----
            0b10100 => {
                if bits == 64 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let shift = immhimmb - bits;
                let esize = (bits / 8) as usize;
                let elements = 8 / esize; // source elements per 64-bit half
                let part = q as usize; // SSHLL2 uses the upper half of Vn
                let src = self.v[rn].to_le_bytes();
                let mut result: u128 = 0;
                for e in 0..elements {
                    let off = part * 8 + e * esize;
                    let a = read_elem(&src, off, esize);
                    let widened: u128 = if u == 0 {
                        ((sext_elem(a, bits) << shift) as u128) & elem_mask_u128(2 * bits)
                    } else {
                        (uext_elem(a, bits) << shift) & elem_mask_u128(2 * bits)
                    };
                    result |= widened << (e * 2 * bits as usize);
                }
                self.v[rd] = result;
                Ok(CpuExit::Continue)
            }
            // ---- Narrowing right shift ----
            0b10000 | 0b10001 | 0b10010 | 0b10011 => {
                if bits == 64 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let rounding = opcode == 0b10001 || opcode == 0b10011;
                let src_bits = 2 * bits;
                let shift = 2 * bits - immhimmb;
                let esize = (bits / 8) as usize;
                // Scalar narrowing shift (SQSHRN <Bd>,<Hn>,#imm etc.) writes one
                // element to lane 0, zeroing the rest; the vector "2" form fills
                // the upper 64 bits (part=1).
                let elements = if scalar { 1 } else { 8 / esize };
                let part = if scalar { 0 } else { q as usize };
                let vn = self.v[rn];
                let mut packed: u64 = 0;
                for e in 0..elements {
                    let s = ((vn >> (e * src_bits as usize)) & elem_mask_u128(src_bits)) as u64;
                    let r: u64 = match (u, opcode) {
                        (0, 0b10000) | (0, 0b10001) => {
                            // SHRN / RSHRN: truncating narrow.
                            simd_rshift(s, shift, src_bits, false, rounding) & elem_mask(bits)
                        }
                        (1, 0b10000) | (1, 0b10001) => {
                            // SQSHRUN / SQRSHRUN: signed source, unsigned saturate.
                            sat_unsigned(simd_rshift_full(s, shift, src_bits, true, rounding), bits)
                        }
                        (0, 0b10010) | (0, 0b10011) => {
                            // SQSHRN / SQRSHRN: signed source, signed saturate.
                            sat_signed(simd_rshift_full(s, shift, src_bits, true, rounding), bits)
                        }
                        _ => {
                            // UQSHRN / UQRSHRN: unsigned source, unsigned saturate.
                            sat_unsigned(
                                simd_rshift_full(s, shift, src_bits, false, rounding),
                                bits,
                            )
                        }
                    };
                    packed |= (r & elem_mask(bits)) << (e * bits as usize);
                }
                let mut bytes = self.v[rd].to_le_bytes();
                bytes[part * 8..part * 8 + 8].copy_from_slice(&packed.to_le_bytes());
                if part == 0 {
                    bytes[8..16].copy_from_slice(&[0u8; 8]);
                }
                self.v[rd] = u128::from_le_bytes(bytes);
                Ok(CpuExit::Continue)
            }
            // ---- Fixed-point convert ----
            0b11100 | 0b11111 => {
                if size_idx < 1 {
                    return Err(ArmError::UndefinedInstruction(insn)); // 8-bit not defined
                }
                if bits == 64 && q == 0 && !scalar {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let fbits = 2 * bits - immhimmb;
                let esize = (bits / 8) as usize;
                let datasize = if scalar {
                    esize
                } else if q == 1 {
                    16
                } else {
                    8
                };
                let elements = datasize / esize;
                let src = self.v[rn].to_le_bytes();
                let mut dst = [0u8; 16];
                let scale = (2.0f64).powi(fbits as i32);
                for e in 0..elements {
                    let off = e * esize;
                    let a = read_elem(&src, off, esize);
                    let r = fixed_point_convert(opcode, u, bits, a, scale);
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[rd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }
            _ => Err(ArmError::UndefinedInstruction(insn)),
        }
    }

    /// Execute Advanced SIMD "vector x indexed element" instructions: the second
    /// multiplicand is a single broadcast lane of Vm. Covers integer MUL/MLA/MLS,
    /// the saturating doubling family, the widening L-forms, and FP FMUL/FMLA/
    /// FMLS/FMULX. (FP16-indexed, FMLAL-indexed and FCMLA are not yet handled.)
    fn exec_simd_indexed(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let l = (insn >> 21) & 1;
        let m = (insn >> 20) & 1;
        let opcode = (insn >> 12) & 0xF;
        let h = (insn >> 11) & 1;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let scalar = ((insn >> 24) & 0x1F) == 0b11111;

        // Element size, second-source register and broadcast lane index.
        // size==00 is the half-precision FP form (FMUL/FMLA/FMLS/FMULX by
        // element); it shares the H:L:M index and 4-bit Vm of the integer H form.
        let (bits, vm_reg, index): (u32, usize, usize) = match size {
            0b00 | 0b01 => (
                16,
                ((insn >> 16) & 0xF) as usize,
                ((h << 2) | (l << 1) | m) as usize,
            ),
            0b10 => (
                32,
                ((m << 4) | ((insn >> 16) & 0xF)) as usize,
                ((h << 1) | l) as usize,
            ),
            0b11 => (64, ((m << 4) | ((insn >> 16) & 0xF)) as usize, h as usize),
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };
        let esize = (bits / 8) as usize;
        let emask = elem_mask(bits);
        let vm_elem = ((self.v[vm_reg] >> (index * bits as usize)) & (emask as u128)) as u64;

        // ---- Floating-point indexed: FMLA/FMLS/FMUL/FMULX ----
        let fp_kind = match (u, opcode) {
            (0, 0b0001) => Some(FpKind::Mla),
            (0, 0b0101) => Some(FpKind::Mls),
            (0, 0b1001) => Some(FpKind::Mul),
            (1, 0b1001) => Some(FpKind::Mulx),
            _ => None,
        };
        if let Some(kind) = fp_kind {
            if size == 0b01 {
                // Half precision uses size==00; size==01 is unallocated for FP.
                return Err(ArmError::UndefinedInstruction(insn));
            }
            if bits == 64 && q == 0 && !scalar {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let datasize = if scalar {
                esize
            } else if q == 1 {
                16
            } else {
                8
            };
            let elements = datasize / esize;
            let vn = self.v[rn].to_le_bytes();
            let vd_old = self.v[rd].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                let off = e * esize;
                let a = read_elem(&vn, off, esize);
                let d = read_elem(&vd_old, off, esize);
                let r = if bits == 16 {
                    let an = a as u16;
                    let bn = vm_elem as u16;
                    let dn = d as u16;
                    (match kind {
                        FpKind::Mul => fp16_mul(an, bn),
                        FpKind::Mulx => fp16_mulx(an, bn),
                        FpKind::Mla => fp16_mla(dn, an, bn),
                        FpKind::Mls => fp16_mls(dn, an, bn),
                        _ => return Err(ArmError::UndefinedInstruction(insn)),
                    }) as u64
                } else if bits == 32 {
                    fp_three_same_f32(kind, a as u32, vm_elem as u32, d as u32) as u64
                } else {
                    fp_three_same_f64(kind, a, vm_elem, d)
                };
                write_elem(&mut dst, off, esize, r);
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SDOT/UDOT by element (opcode 1110): the index selects a 32-bit
        // (4-byte) group of Vm that is reused for every output lane.
        if opcode == 0b1110 {
            if size != 0b10 {
                return Ok(CpuExit::Undefined(insn));
            }
            let signed = u == 0;
            let lanes = if q == 1 { 4 } else { 2 };
            let op1 = self.v[rn];
            let vm_bytes = vm_elem as u32; // the selected 4-byte group
            let mut result = self.v[rd];
            for e in 0..lanes {
                let mut res: i64 = 0;
                for i in 0..4 {
                    let b1 = (op1 >> ((4 * e + i) * 8)) as u8;
                    let b2 = (vm_bytes >> (i * 8)) as u8;
                    res += if signed {
                        (b1 as i8 as i64) * (b2 as i8 as i64)
                    } else {
                        (b1 as i64) * (b2 as i64)
                    };
                }
                let lane = (result >> (e * 32)) as u32;
                let updated = (lane as i64).wrapping_add(res) as u32;
                result =
                    (result & !(0xFFFF_FFFFu128 << (e * 32))) | ((updated as u128) << (e * 32));
            }
            if q == 0 {
                result &= 0xFFFF_FFFF_FFFF_FFFF;
            }
            self.v[rd] = result;
            return Ok(CpuExit::Continue);
        }

        // Integer indexed ops use 16- or 32-bit elements only.
        if size != 0b01 && size != 0b10 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        // ---- Widening L-forms: SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL/SQDMULL/SQDMLAL/SQDMLSL ----
        let widening = matches!(opcode, 0b0010 | 0b0011 | 0b0110 | 0b0111 | 0b1010 | 0b1011);
        if widening {
            let dst_bits = 2 * bits;
            // Scalar by-element (SQDMLAL <Dd>,<Sn>,<Vm>.s[i] etc.) produces one
            // widened element in lane 0, zeroing the rest; the vector "2" form
            // reads the upper half of Vn.
            let elements = if scalar { 1 } else { 64 / bits as usize };
            let part = if scalar { 0 } else { q as usize };
            let signed = u == 0;
            let sat_double = matches!(opcode, 0b0011 | 0b0111 | 0b1011);
            let accum = matches!(opcode, 0b0010 | 0b0110 | 0b0011 | 0b0111);
            let subtract = matches!(opcode, 0b0110 | 0b0111);
            // SQDMULL/SQDMLAL/SQDMLSL are signed-only.
            if sat_double && u == 1 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let vn = self.v[rn].to_le_bytes();
            let vd_old = self.v[rd];
            let dmin = -(1i128 << (dst_bits - 1));
            let dmax = (1i128 << (dst_bits - 1)) - 1;
            let mut result: u128 = 0;
            for e in 0..elements {
                let off = part * 8 + e * esize;
                let a = read_elem(&vn, off, esize);
                let (av, bv): (i128, i128) = if signed {
                    (sext_elem(a, bits), sext_elem(vm_elem, bits))
                } else {
                    (uext_elem(a, bits) as i128, uext_elem(vm_elem, bits) as i128)
                };
                let mut prod = av * bv;
                if sat_double {
                    prod = (prod * 2).clamp(dmin, dmax);
                }
                let elem: u128 = if accum {
                    let d = ((vd_old >> (e * dst_bits as usize)) & elem_mask_u128(dst_bits)) as u64;
                    if sat_double {
                        let acc = sext_elem(d, dst_bits) + if subtract { -prod } else { prod };
                        sat_signed(acc, dst_bits) as u128
                    } else {
                        let r = if subtract {
                            (d as i128).wrapping_sub(prod)
                        } else {
                            (d as i128).wrapping_add(prod)
                        };
                        (r as u128) & elem_mask_u128(dst_bits)
                    }
                } else {
                    (prod as u128) & elem_mask_u128(dst_bits)
                };
                result |= elem << (e * dst_bits as usize);
            }
            self.v[rd] = result;
            return Ok(CpuExit::Continue);
        }

        // ---- Same-size: MUL/MLA/MLS and the saturating doubling-high family ----
        if bits == 64 && q == 0 && !scalar {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        let datasize = if scalar {
            esize
        } else if q == 1 {
            16
        } else {
            8
        };
        let elements = datasize / esize;
        let vn = self.v[rn].to_le_bytes();
        let vd_old = self.v[rd].to_le_bytes();
        let mut dst = [0u8; 16];
        for e in 0..elements {
            let off = e * esize;
            let a = read_elem(&vn, off, esize);
            let d = read_elem(&vd_old, off, esize);
            let r = match (u, opcode) {
                (0, 0b1000) => {
                    ((uext_elem(a, bits) * uext_elem(vm_elem, bits)) as u64) & emask // MUL
                }
                (1, 0b0000) => {
                    let p = (uext_elem(a, bits) * uext_elem(vm_elem, bits)) as u64;
                    d.wrapping_add(p) & emask // MLA
                }
                (1, 0b0100) => {
                    let p = (uext_elem(a, bits) * uext_elem(vm_elem, bits)) as u64;
                    d.wrapping_sub(p) & emask // MLS
                }
                (0, 0b1100) => adv_simd_three_same_int(0, 0b10110, bits, a, vm_elem, 0), // SQDMULH
                (0, 0b1101) => adv_simd_three_same_int(1, 0b10110, bits, a, vm_elem, 0), // SQRDMULH
                (1, 0b1101) => {
                    // SQRDMLAH: accumulate the (unsaturated) rounded doubling
                    // product, then saturate once.
                    let prod = sext_elem(a, bits) * sext_elem(vm_elem, bits);
                    let rounded = (prod * 2 + (1i128 << (bits - 1))) >> bits;
                    sat_signed(sext_elem(d, bits) + rounded, bits)
                }
                (1, 0b1111) => {
                    // SQRDMLSH
                    let prod = sext_elem(a, bits) * sext_elem(vm_elem, bits);
                    let rounded = (prod * 2 + (1i128 << (bits - 1))) >> bits;
                    sat_signed(sext_elem(d, bits) - rounded, bits)
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute the Advanced SIMD "copy" group: DUP (element/general), INS
    /// (element/general), SMOV, UMOV. Element size and lane index come from the
    /// `imm5` field (lowest set bit selects the size).
    fn exec_simd_copy(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let op = (insn >> 29) & 1;
        let scalar = (insn >> 24) & 0x1F == 0b11110; // DUP <V><d>,<Vn>.<T>[i] (MOV)
        let imm5 = (insn >> 16) & 0x1F;
        let imm4 = (insn >> 11) & 0xF;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        let size = if imm5 & 1 != 0 {
            0u32
        } else if imm5 & 2 != 0 {
            1
        } else if imm5 & 4 != 0 {
            2
        } else if imm5 & 8 != 0 {
            3
        } else {
            return Err(ArmError::UndefinedInstruction(insn));
        };
        let esize = 8u32 << size; // element size in bits
        let shift = esize as usize;
        let index = (imm5 >> (size + 1)) as usize;
        let emask = elem_mask_u128(esize);

        if op == 1 {
            // INS (element): Vd[index] = Vn[src_index].
            let src_index = (imm4 >> size) as usize;
            let vn = self.v[rn as usize];
            let elem = (vn >> (src_index * shift)) & emask;
            let mut vd = self.v[rd as usize];
            vd &= !(emask << (index * shift));
            vd |= elem << (index * shift);
            self.v[rd as usize] = vd;
            return Ok(CpuExit::Continue);
        }

        match imm4 {
            0b0000 => {
                // DUP (element): broadcast Vn[index]. The scalar form (MOV
                // <V><d>,<Vn>.<T>[i]) extracts a single element into lane 0.
                if !scalar && size == 3 && q == 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let vn = self.v[rn as usize];
                let elem = (vn >> (index * shift)) & emask;
                if scalar {
                    self.v[rd as usize] = elem;
                } else {
                    let datasize = if q == 1 { 128 } else { 64 };
                    let mut result = 0u128;
                    let mut p = 0;
                    while p < datasize {
                        result |= elem << p;
                        p += shift;
                    }
                    self.v[rd as usize] = result;
                }
            }
            0b0001 => {
                // DUP (general): broadcast Xn/Wn.
                if size == 3 && q == 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let v = (self.get_x(rn) as u128) & emask;
                let datasize = if q == 1 { 128 } else { 64 };
                let mut result = 0u128;
                let mut p = 0;
                while p < datasize {
                    result |= v << p;
                    p += shift;
                }
                self.v[rd as usize] = result;
            }
            0b0011 => {
                // INS (general): Vd[index] = Xn/Wn.
                let v = (self.get_x(rn) as u128) & emask;
                let mut vd = self.v[rd as usize];
                vd &= !(emask << (index * shift));
                vd |= v << (index * shift);
                self.v[rd as usize] = vd;
            }
            0b0101 => {
                // SMOV: GPR = sign-extended Vn[index]. Valid: B/H -> W or X,
                // S -> X only; never D.
                if size == 3 || (size == 2 && q == 0) {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let vn = self.v[rn as usize];
                let elem = ((vn >> (index * shift)) & emask) as u64;
                let signed = sext_elem(elem, esize) as u64;
                if q == 1 {
                    self.set_x(rd, signed);
                } else {
                    self.set_w(rd, signed as u32);
                }
            }
            0b0111 => {
                // UMOV: GPR = zero-extended Vn[index]. Valid: B/H/S -> W,
                // D -> X only.
                let valid = (size <= 2 && q == 0) || (size == 3 && q == 1);
                if !valid {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let vn = self.v[rn as usize];
                let elem = ((vn >> (index * shift)) & emask) as u64;
                if q == 1 {
                    self.set_x(rd, elem);
                } else {
                    self.set_w(rd, elem as u32);
                }
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        }
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD permute operations (ZIP, UZP, TRN).
    fn exec_simd_permute(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let size = (insn >> 22) & 0x3;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let opcode = (insn >> 12) & 0x7;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        // 64-bit elements need the 2D (Q==1) arrangement; "1D" is RESERVED.
        if size == 0b11 && q == 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let esize = 1usize << size;
        let datasize = if q == 1 { 16 } else { 8 };
        let elements = datasize / esize;

        let src1 = self.v[rn].to_le_bytes();
        let src2 = self.v[rm].to_le_bytes();
        let mut dst = [0u8; 16];

        match opcode {
            0b001 => {
                // UZP1 - unzip, lower halves
                for e in 0..elements {
                    let src_idx = e * 2;
                    let dst_off = e * esize;
                    if src_idx < elements {
                        let src_off = src_idx * esize;
                        dst[dst_off..dst_off + esize]
                            .copy_from_slice(&src1[src_off..src_off + esize]);
                    } else {
                        let src_off = (src_idx - elements) * esize;
                        dst[dst_off..dst_off + esize]
                            .copy_from_slice(&src2[src_off..src_off + esize]);
                    }
                }
            }
            0b010 => {
                // TRN1 - transpose, lower halves
                for e in 0..(elements / 2) {
                    let dst_off1 = (e * 2) * esize;
                    let dst_off2 = (e * 2 + 1) * esize;
                    let src_off = (e * 2) * esize;
                    dst[dst_off1..dst_off1 + esize]
                        .copy_from_slice(&src1[src_off..src_off + esize]);
                    dst[dst_off2..dst_off2 + esize]
                        .copy_from_slice(&src2[src_off..src_off + esize]);
                }
            }
            0b011 => {
                // ZIP1 - zip, lower halves
                for e in 0..(elements / 2) {
                    let dst_off1 = (e * 2) * esize;
                    let dst_off2 = (e * 2 + 1) * esize;
                    let src_off = e * esize;
                    dst[dst_off1..dst_off1 + esize]
                        .copy_from_slice(&src1[src_off..src_off + esize]);
                    dst[dst_off2..dst_off2 + esize]
                        .copy_from_slice(&src2[src_off..src_off + esize]);
                }
            }
            0b101 => {
                // UZP2 - unzip, upper halves
                for e in 0..elements {
                    let src_idx = e * 2 + 1;
                    let dst_off = e * esize;
                    if src_idx < elements {
                        let src_off = src_idx * esize;
                        dst[dst_off..dst_off + esize]
                            .copy_from_slice(&src1[src_off..src_off + esize]);
                    } else {
                        let src_off = (src_idx - elements) * esize;
                        dst[dst_off..dst_off + esize]
                            .copy_from_slice(&src2[src_off..src_off + esize]);
                    }
                }
            }
            0b110 => {
                // TRN2 - transpose, upper halves
                for e in 0..(elements / 2) {
                    let dst_off1 = (e * 2) * esize;
                    let dst_off2 = (e * 2 + 1) * esize;
                    let src_off = (e * 2 + 1) * esize;
                    dst[dst_off1..dst_off1 + esize]
                        .copy_from_slice(&src1[src_off..src_off + esize]);
                    dst[dst_off2..dst_off2 + esize]
                        .copy_from_slice(&src2[src_off..src_off + esize]);
                }
            }
            0b111 => {
                // ZIP2 - zip, upper halves
                let half = elements / 2;
                for e in 0..half {
                    let dst_off1 = (e * 2) * esize;
                    let dst_off2 = (e * 2 + 1) * esize;
                    let src_off = (half + e) * esize;
                    dst[dst_off1..dst_off1 + esize]
                        .copy_from_slice(&src1[src_off..src_off + esize]);
                    dst[dst_off2..dst_off2 + esize]
                        .copy_from_slice(&src2[src_off..src_off + esize]);
                }
            }
            _ => return Ok(CpuExit::Undefined(insn)),
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD table lookup (TBL, TBX).
    fn exec_simd_table(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let len = ((insn >> 13) & 0x3) as usize;
        let op = (insn >> 12) & 1;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        let datasize = if q == 1 { 16 } else { 8 };

        // Build table from consecutive registers
        let mut table = [0u8; 64];
        for i in 0..=len {
            let reg = (rn + i) % 32;
            let bytes = self.v[reg].to_le_bytes();
            table[i * 16..(i + 1) * 16].copy_from_slice(&bytes);
        }
        let table_size = (len + 1) * 16;

        let indices = self.v[rm].to_le_bytes();
        let mut dst = if op == 1 {
            // TBX: keep original values for out-of-range indices
            self.v[rd].to_le_bytes()
        } else {
            [0u8; 16]
        };

        for i in 0..datasize {
            let idx = indices[i] as usize;
            if idx < table_size {
                dst[i] = table[idx];
            }
            // For TBL (op=0), out-of-range stays 0
            // For TBX (op=1), out-of-range keeps original
        }
        // Q==0 zeroes the upper 64 bits (TBX kept Vd's upper half otherwise).
        if q == 0 {
            for b in 8..16 {
                dst[b] = 0;
            }
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD extract (EXT).
    fn exec_simd_extract(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let imm4 = ((insn >> 11) & 0xF) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        let datasize = if q == 1 { 16 } else { 8 };

        // imm4 with bit 3 set is UNDEFINED for the 64-bit (Q==0) form.
        if q == 0 && imm4 >= 8 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        // Concatenate the low `datasize` bytes of Vn:Vm and extract `datasize`
        // bytes starting at byte `imm4`.
        let src1 = self.v[rn].to_le_bytes();
        let src2 = self.v[rm].to_le_bytes();
        let mut concat = [0u8; 32];
        concat[..datasize].copy_from_slice(&src1[..datasize]);
        concat[datasize..2 * datasize].copy_from_slice(&src2[..datasize]);

        let mut dst = [0u8; 16];
        for i in 0..datasize {
            dst[i] = concat[imm4 + i];
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD three-same register instructions.
    fn exec_simd_three_same(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let opcode = (insn >> 11) & 0x1F;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let scalar = ((insn >> 24) & 0x1F) == 0b11110;

        // Floating-point three-same opcodes (0b11000..=0b11111).
        if opcode >= 0b11000 {
            return self.exec_simd_three_same_fp(insn, scalar);
        }

        // Logical operations (opcode 0b00011) act on the whole register; the
        // `size` field selects the operation rather than the element size.
        if opcode == 0b00011 {
            let n1 = self.v[rn];
            let n2 = self.v[rm];
            let dd = self.v[rd];
            let result = match (u, size) {
                (0, 0b00) => n1 & n2,                // AND
                (0, 0b01) => n1 & !n2,               // BIC
                (0, 0b10) => n1 | n2,                // ORR
                (0, 0b11) => n1 | !n2,               // ORN
                (1, 0b00) => n1 ^ n2,                // EOR
                (1, 0b01) => n2 ^ (dd & (n2 ^ n1)),  // BSL
                (1, 0b10) => dd ^ ((dd ^ n1) & n2),  // BIT
                (1, 0b11) => dd ^ ((dd ^ n1) & !n2), // BIF
                _ => unreachable!(),
            };
            let mask = if q == 1 {
                u128::MAX
            } else {
                0xFFFF_FFFF_FFFF_FFFF
            };
            self.v[rd] = result & mask;
            return Ok(CpuExit::Continue);
        }

        let bits = 8u32 << size; // 8, 16, 32 or 64
        let esize = (bits / 8) as usize;

        if scalar {
            // The scalar form allows only a subset of opcodes. The non-saturating
            // arithmetic/compare/shift ops (ADD/SUB, CMGT/CMGE/CMHI/CMHS,
            // CMTST/CMEQ, SSHL/USHL, SRSHL/URSHL) are defined for 64-bit (D)
            // elements only; the saturating ops allow all sizes; everything else
            // is unallocated as a scalar.
            let scalar_d_only = matches!(
                opcode,
                0b00110 | 0b00111 | 0b01000 | 0b01010 | 0b10000 | 0b10001
            );
            let scalar_any_size = matches!(opcode, 0b00001 | 0b00101 | 0b01001 | 0b01011);
            let scalar_sqdmulh = opcode == 0b10110;
            if scalar_d_only {
                if size != 0b11 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
            } else if !scalar_any_size && !scalar_sqdmulh {
                return Err(ArmError::UndefinedInstruction(insn));
            }
        }

        // Reject UNDEFINED (opcode, size) combinations. These integer opcodes
        // have no 64-bit (size==0b11) vector form.
        let no_64 = matches!(
            opcode,
            0b00000
                | 0b00010
                | 0b00100
                | 0b01100
                | 0b01101
                | 0b01110
                | 0b01111
                | 0b10010
                | 0b10100
                | 0b10101
        );
        if size == 0b11 && no_64 {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        // 64-bit elements need the 2D (Q==1) arrangement; "1D" is not a valid
        // vector form. (Scalar uses a single element and is handled separately.)
        if size == 0b11 && q == 0 && !scalar {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        match opcode {
            0b10011 => {
                // MUL: no 64-bit form; PMUL: 8-bit only.
                if u == 0 && size == 0b11 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                if u == 1 && size != 0b00 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
            }
            0b10110 => {
                // SQDMULH/SQRDMULH: 16- or 32-bit only.
                if size == 0b00 || size == 0b11 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
            }
            0b10111 => {
                // ADDP is U==0 only; U==1 at this opcode is unallocated.
                if u == 1 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
            }
            _ => {}
        }

        let datasize = if scalar {
            esize
        } else if q == 1 {
            16
        } else {
            8
        };
        let elements = datasize / esize;

        // SMAXP/SMINP/ADDP take their operands pairwise from the Vn:Vm concat.
        let pairwise = matches!(opcode, 0b10100 | 0b10101 | 0b10111);

        let src1 = self.v[rn].to_le_bytes();
        let src2 = self.v[rm].to_le_bytes();
        let old_d = self.v[rd].to_le_bytes();
        let mut dst = [0u8; 16];

        let mut concat = [0u8; 32];
        if pairwise {
            concat[..datasize].copy_from_slice(&src1[..datasize]);
            concat[datasize..datasize * 2].copy_from_slice(&src2[..datasize]);
        }

        for e in 0..elements {
            let off = e * esize;
            let (a, b) = if pairwise {
                (
                    read_elem(&concat, (2 * e) * esize, esize),
                    read_elem(&concat, (2 * e + 1) * esize, esize),
                )
            } else {
                (read_elem(&src1, off, esize), read_elem(&src2, off, esize))
            };
            let d = read_elem(&old_d, off, esize);
            let res = adv_simd_three_same_int(u, opcode, bits, a, b, d);
            write_elem(&mut dst, off, esize, res);
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute an Advanced SIMD three-same floating-point instruction.
    fn exec_simd_three_same_fp(&mut self, insn: u32, scalar: bool) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let opcode = (insn >> 11) & 0x1F;
        let rm = ((insn >> 16) & 0x1F) as usize;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;

        let sz = size & 1; // 0 => f32, 1 => f64
        let a_bit = (size >> 1) & 1;

        // FEAT_FHM: FMLAL/FMLSL (U==0, opcode 0b11101) and FMLAL2/FMLSL2
        // (U==1, opcode 0b11001) widen FP16 lanes into FP32 accumulator lanes.
        // These are only defined for the vector (non-scalar) form.
        if !scalar && ((u == 0 && opcode == 0b11101) || (u == 1 && opcode == 0b11001)) {
            return self.exec_fmlal(insn, false);
        }

        let kind = match fp_three_same_decode(u, a_bit, opcode) {
            Some(k) => k,
            None => return Err(ArmError::UndefinedInstruction(insn)),
        };
        let esize = if sz == 0 { 4usize } else { 8 };

        // A 64-bit vector cannot hold a single f64 element (needs 2D / Q==1).
        if sz == 1 && q == 0 && !scalar {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let datasize = if scalar {
            esize
        } else if q == 1 {
            16
        } else {
            8
        };
        let elements = datasize / esize;

        let pairwise = matches!(
            kind,
            FpKind::Addp | FpKind::Maxp | FpKind::Minp | FpKind::MaxNmp | FpKind::MinNmp
        );

        let src1 = self.v[rn].to_le_bytes();
        let src2 = self.v[rm].to_le_bytes();
        let old_d = self.v[rd].to_le_bytes();
        let mut dst = [0u8; 16];

        let mut concat = [0u8; 32];
        if pairwise {
            concat[..datasize].copy_from_slice(&src1[..datasize]);
            concat[datasize..datasize * 2].copy_from_slice(&src2[..datasize]);
        }

        for e in 0..elements {
            let off = e * esize;
            let (a, b) = if pairwise {
                (
                    read_elem(&concat, (2 * e) * esize, esize),
                    read_elem(&concat, (2 * e + 1) * esize, esize),
                )
            } else {
                (read_elem(&src1, off, esize), read_elem(&src2, off, esize))
            };
            let d = read_elem(&old_d, off, esize);
            let res = if sz == 0 {
                fp_three_same_f32(kind, a as u32, b as u32, d as u32) as u64
            } else {
                fp_three_same_f64(kind, a, b, d)
            };
            write_elem(&mut dst, off, esize, res);
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// FMLAL/FMLSL/FMLAL2/FMLSL2 (FEAT_FHM): widening FP16 fused multiply-add.
    /// Each FP32 result lane accumulates the exact product of two FP16 source
    /// lanes. The non-`2` forms take the lower half of the FP16 lanes, the `2`
    /// forms the upper half. `a` (size<1>) selects add vs subtract.
    fn exec_fmlal(&mut self, insn: u32, indexed: bool) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        // Vector form: sub=bit23, "2"=U(bit29), Rm=bits[20:16]. Indexed form:
        // sub=bit13, "2"=bit15, Rm=bits[19:16], index=(H:L:M)=bit11:bit21:bit20.
        let (sub, top, rm, index) = if indexed {
            (
                (insn >> 14) & 1 != 0, // FMLSL/FMLSL2 (op[15:12] bit14)
                (insn >> 15) & 1 != 0, // FMLAL2/FMLSL2 (upper FP16 lanes)
                ((insn >> 16) & 0xF) as usize,
                Some(
                    (((insn >> 11) & 1) << 2 | ((insn >> 21) & 1) << 1 | ((insn >> 20) & 1))
                        as usize,
                ),
            )
        } else {
            (
                (insn >> 23) & 1 != 0,
                (insn >> 29) & 1 != 0,
                ((insn >> 16) & 0x1F) as usize,
                None,
            )
        };
        let elements = if q == 1 { 4 } else { 2 };
        let sel = if top { elements } else { 0 }; // "2" forms read the upper FP16 lanes
        let vn = self.v[rn];
        let vm = self.v[rm];
        let vd = self.v[rd];
        let mut result: u128 = 0;
        for e in 0..elements {
            let lane = e + sel;
            // FMLSL negates the multiplicand (FPNeg op1) before the fused MAC.
            let h1 = (vn >> (16 * lane)) as u16 ^ if sub { 0x8000 } else { 0 };
            let h2 = match index {
                Some(ix) => (vm >> (16 * ix)) as u16,
                None => (vm >> (16 * lane)) as u16,
            };
            let nn = Self::fp16_to_f32(h1).to_bits();
            let mm = Self::fp16_to_f32(h2).to_bits();
            let acc = (vd >> (32 * e)) as u32;
            let r = fp_muladd_bits(acc as u64, nn as u64, mm as u64, 32) as u32;
            result |= (r as u128) << (32 * e);
        }
        // Q==0 leaves the upper 64 bits zero.
        self.v[rd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute SIMD two-register miscellaneous instructions.
    fn exec_simd_two_reg(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let size = (insn >> 22) & 0x3;
        let opcode = (insn >> 12) & 0x1F;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        // Scalar AdvSIMD two-reg-misc (top byte 0x5E/0x7E) operates on a single
        // element of the low lane, zeroing the rest of the destination.
        let scalar = (insn >> 24) & 0x1F == 0b11110;

        let esize = 1usize << size;
        let datasize = if q == 1 { 16 } else { 8 };
        let elements = if scalar { 1 } else { datasize / esize };

        // ---- REV64 / REV32 / REV16: reverse elements within a container. ----
        if (u == 0 && opcode == 0b00000)
            || (u == 1 && opcode == 0b00000)
            || (u == 0 && opcode == 0b00001)
        {
            let container = if opcode == 0b00001 {
                16usize // REV16
            } else if u == 1 {
                32 // REV32
            } else {
                64 // REV64
            };
            let cbytes = container / 8;
            if esize >= cbytes || (8 << size) > container {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let epc = cbytes / esize; // elements per container
            let src = self.v[rn].to_le_bytes();
            let mut dst = [0u8; 16];
            for c in 0..(datasize / cbytes) {
                for i in 0..epc {
                    let from = (c * epc + (epc - 1 - i)) * esize;
                    let to = (c * epc + i) * esize;
                    dst[to..to + esize].copy_from_slice(&src[from..from + esize]);
                }
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // ---- NOT (size==00) / RBIT (size==01): per-byte, U==1 opcode 0b00101. ----
        if u == 1 && opcode == 0b00101 {
            if size > 0b01 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let src = self.v[rn].to_le_bytes();
            let mut dst = [0u8; 16];
            for b in 0..datasize {
                dst[b] = if size == 0b00 {
                    !src[b]
                } else {
                    src[b].reverse_bits()
                };
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // ---- Same-size integer ops (CLS/CLZ/CNT/ABS/NEG/SQABS/SQNEG/CMxx#0/
        //      SUQADD/USQADD). ----
        {
            let bits = (8u32) << size;
            // Probe whether this (u, opcode) is one we handle here.
            if adv_simd_two_reg_int(u, opcode, bits, 0, 0).is_some() {
                // CNT is byte-only; NOT/RBIT handled above.
                if opcode == 0b00101 && size != 0b00 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                // CLS/CLZ have no 64-bit element form.
                if opcode == 0b00100 && size == 0b11 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                // 64-bit elements need the 2D (Q==1) arrangement.
                if size == 0b11 && q == 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                let accumulate = opcode == 0b00011; // SUQADD / USQADD read Vd
                let src = self.v[rn].to_le_bytes();
                let old = self.v[rd].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let off = e * esize;
                    let a = read_elem(&src, off, esize);
                    let d = if accumulate {
                        read_elem(&old, off, esize)
                    } else {
                        0
                    };
                    let r = adv_simd_two_reg_int(u, opcode, bits, a, d).unwrap();
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[rd] = u128::from_le_bytes(dst);
                return Ok(CpuExit::Continue);
            }
        }

        // ---- SADDLP/UADDLP (00010), SADALP/UADALP (00110): pairwise widening. ----
        if opcode == 0b00010 || opcode == 0b00110 {
            if size == 0b11 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let bits = 8u32 << size;
            let dbits = 2 * bits;
            let src_elems = datasize / esize;
            let out_elems = src_elems / 2;
            let signed = u == 0;
            let accumulate = opcode == 0b00110;
            let src = self.v[rn].to_le_bytes();
            let vd = self.v[rd];
            let mut result = 0u128;
            for o in 0..out_elems {
                let a = read_elem(&src, (2 * o) * esize, esize);
                let b = read_elem(&src, (2 * o + 1) * esize, esize);
                let sum: i128 = if signed {
                    sext_elem(a, bits) + sext_elem(b, bits)
                } else {
                    uext_elem(a, bits) as i128 + uext_elem(b, bits) as i128
                };
                let mut val = (sum as u128) & elem_mask_u128(dbits);
                if accumulate {
                    let d = (vd >> (o * dbits as usize)) & elem_mask_u128(dbits);
                    val = val.wrapping_add(d) & elem_mask_u128(dbits);
                }
                result |= val << (o * dbits as usize);
            }
            self.v[rd] = result;
            return Ok(CpuExit::Continue);
        }

        // ---- XTN/SQXTUN (10010), SQXTN/UQXTN (10100): narrowing. ----
        if opcode == 0b10010 || opcode == 0b10100 {
            if size == 0b11 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let bits = 8u32 << size; // destination element size
            let dbits = 2 * bits; // source element size
            // Scalar narrowing (SQXTN/UQXTN/SQXTUN <Bd>,<Hn> etc.) writes a single
            // element into lane 0 and zeroes the rest; the vector form fills the
            // low (part=0) or high (part=1, the "2" variant) 64-bit half.
            let out_elems = if scalar { 1 } else { 8 / esize };
            let part = if scalar { 0 } else { q as usize };
            let vn = self.v[rn];
            let mut packed = 0u64;
            for e in 0..out_elems {
                let s = ((vn >> (e * dbits as usize)) & elem_mask_u128(dbits)) as u64;
                let r: u64 = match (u, opcode) {
                    (0, 0b10010) => s & elem_mask(bits),                     // XTN
                    (1, 0b10010) => sat_unsigned(sext_elem(s, dbits), bits), // SQXTUN
                    (0, 0b10100) => sat_signed(sext_elem(s, dbits), bits),   // SQXTN
                    _ => sat_unsigned(uext_elem(s, dbits) as i128, bits),    // UQXTN
                };
                packed |= (r & elem_mask(bits)) << (e * bits as usize);
            }
            let mut bytes = self.v[rd].to_le_bytes();
            bytes[part * 8..part * 8 + 8].copy_from_slice(&packed.to_le_bytes());
            if part == 0 {
                bytes[8..16].copy_from_slice(&[0u8; 8]);
            }
            self.v[rd] = u128::from_le_bytes(bytes);
            return Ok(CpuExit::Continue);
        }

        // ---- SHLL/SHLL2 (U==1, 10011): shift left long by the element size. ----
        if u == 1 && opcode == 0b10011 {
            if size == 0b11 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            let bits = 8u32 << size;
            let dbits = 2 * bits;
            let part = q as usize;
            let src = self.v[rn].to_le_bytes();
            let mut result = 0u128;
            for e in 0..(8 / esize) {
                let a = read_elem(&src, part * 8 + e * esize, esize);
                let val = (uext_elem(a, bits) << bits) & elem_mask_u128(dbits);
                result |= val << (e * dbits as usize);
            }
            self.v[rd] = result;
            return Ok(CpuExit::Continue);
        }

        // ---- Floating-point two-register-misc (deterministic subset). The
        //      estimate ops (FRECPE/FRSQRTE/URECPE/URSQRTE) and FP narrow/long
        //      fall through to the legacy handling below. ----
        if let Some(r) = self.exec_simd_two_reg_fp(insn) {
            return r;
        }

        let src = self.v[rn].to_le_bytes();
        let mut dst = [0u8; 16];

        for e in 0..elements {
            let offset = e * esize;

            match esize {
                1 => {
                    let a = src[offset];
                    dst[offset] = match (u, opcode) {
                        (1, 0b00101) => !a, // NOT
                        (0, 0b01011) => {
                            if (a as i8) < 0 {
                                a.wrapping_neg()
                            } else {
                                a
                            }
                        } // ABS
                        (1, 0b01011) => a.wrapping_neg(), // NEG
                        _ => a,
                    };
                }
                2 => {
                    let a = i16::from_le_bytes([src[offset], src[offset + 1]]);
                    let result = match (u, opcode) {
                        (0, 0b01011) => a.abs() as u16,
                        (1, 0b01011) => a.wrapping_neg() as u16,
                        _ => a as u16,
                    };
                    let bytes = result.to_le_bytes();
                    dst[offset..offset + 2].copy_from_slice(&bytes);
                }
                4 => {
                    if opcode >= 0b01100 && opcode <= 0b11111 {
                        // FP unary
                        let a = f32::from_le_bytes([
                            src[offset],
                            src[offset + 1],
                            src[offset + 2],
                            src[offset + 3],
                        ]);
                        let result = match (u, opcode) {
                            (0, 0b01111) => a.abs(),   // FABS
                            (1, 0b01111) => -a,        // FNEG
                            (1, 0b10111) => a.sqrt(),  // FSQRT
                            (0, 0b11000) => a.round(), // FRINTN
                            (1, 0b11000) => a.ceil(),  // FRINTP
                            (0, 0b11001) => a.floor(), // FRINTM
                            (1, 0b11001) => a.trunc(), // FRINTZ
                            _ => a,
                        };
                        let bytes = result.to_le_bytes();
                        dst[offset..offset + 4].copy_from_slice(&bytes);
                    } else {
                        // Integer
                        let a = i32::from_le_bytes([
                            src[offset],
                            src[offset + 1],
                            src[offset + 2],
                            src[offset + 3],
                        ]);
                        let result = match (u, opcode) {
                            (0, 0b01011) => a.abs() as u32,
                            (1, 0b01011) => a.wrapping_neg() as u32,
                            (1, 0b00101) => !(a as u32),
                            _ => a as u32,
                        };
                        let bytes = result.to_le_bytes();
                        dst[offset..offset + 4].copy_from_slice(&bytes);
                    }
                }
                8 => {
                    if opcode >= 0b01100 {
                        // FP double
                        let a = f64::from_le_bytes([
                            src[offset],
                            src[offset + 1],
                            src[offset + 2],
                            src[offset + 3],
                            src[offset + 4],
                            src[offset + 5],
                            src[offset + 6],
                            src[offset + 7],
                        ]);
                        let result = match (u, opcode) {
                            (0, 0b01111) => a.abs(),
                            (1, 0b01111) => -a,
                            (1, 0b10111) => a.sqrt(),
                            (0, 0b11000) => a.round(),
                            (1, 0b11000) => a.ceil(),
                            (0, 0b11001) => a.floor(),
                            (1, 0b11001) => a.trunc(),
                            _ => a,
                        };
                        let bytes = result.to_le_bytes();
                        dst[offset..offset + 8].copy_from_slice(&bytes);
                    } else {
                        let a = i64::from_le_bytes([
                            src[offset],
                            src[offset + 1],
                            src[offset + 2],
                            src[offset + 3],
                            src[offset + 4],
                            src[offset + 5],
                            src[offset + 6],
                            src[offset + 7],
                        ]);
                        let result = match (u, opcode) {
                            (0, 0b01011) => a.abs() as u64,
                            (1, 0b01011) => a.wrapping_neg() as u64,
                            _ => a as u64,
                        };
                        let bytes = result.to_le_bytes();
                        dst[offset..offset + 8].copy_from_slice(&bytes);
                    }
                }
                _ => {}
            }
        }

        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Deterministic FP two-register-misc ops (FABS/FNEG/FSQRT, FRINT*, FCVT* to
    /// integer, SCVTF/UCVTF, FCMxx #0). Returns `None` for the estimate ops and
    /// FP narrow/long forms so the caller can fall through.
    fn exec_simd_two_reg_fp(&mut self, insn: u32) -> Option<Result<CpuExit, ArmError>> {
        let q = (insn >> 30) & 1;
        let u = (insn >> 29) & 1;
        let sz_hi = (insn >> 23) & 1;
        let sz = (insn >> 22) & 1; // 0 => f32, 1 => f64
        let opcode = (insn >> 12) & 0x1F;
        let rn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as usize;
        let scalar = ((insn >> 24) & 0x1F) == 0b11110;

        // FRINT32X/Z (opcode 11110) and FRINT64X/Z (opcode 11111), sz_hi==0.
        // U selects X(round per mode)/Z(toward zero); bit22 selects f32/f64.
        if sz_hi == 0 && (opcode == 0b11110 || opcode == 0b11111) {
            let intsize = if opcode == 0b11110 { 32 } else { 64 };
            let z = u == 0;
            let esize = if sz == 0 { 4usize } else { 8 };
            if sz == 1 && q == 0 && !scalar {
                return Some(Err(ArmError::UndefinedInstruction(insn)));
            }
            let datasize = if scalar {
                esize
            } else if q == 1 {
                16
            } else {
                8
            };
            let src = self.v[rn].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..(datasize / esize) {
                let off = e * esize;
                let a = read_elem(&src, off, esize);
                let r = if esize == 4 {
                    frint_ts_f32(a as u32, intsize, z) as u64
                } else {
                    frint_ts_f64(a, intsize, z)
                };
                write_elem(&mut dst, off, esize, r);
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Some(Ok(CpuExit::Continue));
        }

        // FCVTL/FCVTL2 (opcode 10111, U=0), FCVTN/FCVTN2 (10110, U=0) and
        // FCVTXN/FCVTXN2 (10110, U=1): FP convert long/narrow. sz(bit22) selects
        // the f16<->f32 (0) vs f32<->f64 (1) pair. (BFCVTN size==10 is handled
        // before reaching here.)
        if opcode == 0b10110 || opcode == 0b10111 {
            let long = opcode == 0b10111;
            let round_odd = !long && u == 1; // FCVTXN
            let part = q as usize; // FCVTL2/FCVTN2 use the upper half
            if long {
                // Widen: f16->f32 (sz=0) or f32->f64 (sz=1). The source 64-bit
                // half holds 8/sp elements (4 h or 2 s).
                let (sp, dp) = if sz == 0 { (2usize, 4usize) } else { (4, 8) };
                let nelem = 8 / sp;
                let src = self.v[rn].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..nelem {
                    let s = read_elem(&src, part * 8 + e * sp, sp);
                    write_elem(&mut dst, e * dp, dp, fp_cvt_elem(s, sp, dp, false));
                }
                self.v[rd] = u128::from_le_bytes(dst);
            } else {
                // Narrow: f32->f16 (sz=0) or f64->f32 (sz=1, or FCVTX round-odd).
                // The full 128-bit source holds 16/sp elements (4 s or 2 d).
                let (sp, dp) = if sz == 0 { (4usize, 2usize) } else { (8, 4) };
                let nelem = if scalar { 1 } else { 16 / sp };
                let src = self.v[rn].to_le_bytes();
                let mut dst = if scalar || part == 0 {
                    [0u8; 16]
                } else {
                    self.v[rd].to_le_bytes()
                };
                let base = if scalar { 0 } else { part * 8 };
                for e in 0..nelem {
                    let s = read_elem(&src, e * sp, sp);
                    write_elem(
                        &mut dst,
                        base + e * dp,
                        dp,
                        fp_cvt_elem(s, sp, dp, round_odd),
                    );
                }
                self.v[rd] = u128::from_le_bytes(dst);
            }
            return Some(Ok(CpuExit::Continue));
        }

        // SCVTF / UCVTF take an integer source, so they bypass the float helper.
        let cvtf = match (u, sz_hi, opcode) {
            (0, 0, 0b11101) => Some(false), // SCVTF
            (1, 0, 0b11101) => Some(true),  // UCVTF
            _ => None,
        };
        let kind = match (u, sz_hi, opcode) {
            (0, 1, 0b01111) => Some(TwoRegFp::Fabs),
            (1, 1, 0b01111) => Some(TwoRegFp::Fneg),
            (1, 1, 0b11111) => Some(TwoRegFp::Fsqrt),
            (0, 0, 0b11000) => Some(TwoRegFp::RintN),
            (0, 1, 0b11000) => Some(TwoRegFp::RintP),
            (1, 0, 0b11000) => Some(TwoRegFp::RintA),
            (0, 0, 0b11001) => Some(TwoRegFp::RintM),
            (0, 1, 0b11001) => Some(TwoRegFp::RintZ),
            (1, 0, 0b11001) => Some(TwoRegFp::RintX),
            (1, 1, 0b11001) => Some(TwoRegFp::RintI),
            (0, 0, 0b11010) => Some(TwoRegFp::CvtNS),
            (0, 1, 0b11010) => Some(TwoRegFp::CvtPS),
            (1, 0, 0b11010) => Some(TwoRegFp::CvtNU),
            (1, 1, 0b11010) => Some(TwoRegFp::CvtPU),
            (0, 0, 0b11011) => Some(TwoRegFp::CvtMS),
            (0, 1, 0b11011) => Some(TwoRegFp::CvtZS),
            (1, 0, 0b11011) => Some(TwoRegFp::CvtMU),
            (1, 1, 0b11011) => Some(TwoRegFp::CvtZU),
            (0, 0, 0b11100) => Some(TwoRegFp::CvtAS),
            (1, 0, 0b11100) => Some(TwoRegFp::CvtAU),
            (0, 1, 0b01100) => Some(TwoRegFp::CmGt),
            (1, 1, 0b01100) => Some(TwoRegFp::CmGe),
            (0, 1, 0b01101) => Some(TwoRegFp::CmEq),
            (1, 1, 0b01101) => Some(TwoRegFp::CmLe),
            (0, 1, 0b01110) => Some(TwoRegFp::CmLt),
            _ => None,
        };
        // URECPE (U=0) / URSQRTE (U=1): unsigned 32-bit integer estimates,
        // sz_hi=1, opcode 11100.
        if (insn >> 23) & 1 == 1 && opcode == 0b11100 {
            if sz != 0 {
                return Some(Err(ArmError::UndefinedInstruction(insn)));
            }
            let datasize = if scalar {
                4usize
            } else if q == 1 {
                16
            } else {
                8
            };
            let elements = datasize / 4;
            let src = self.v[rn].to_le_bytes();
            let mut dst = [0u8; 16];
            let is_rsqrt = (insn >> 29) & 1 == 1;
            for e in 0..elements {
                let off = e * 4;
                let a = read_elem(&src, off, 4) as u32;
                let r = if is_rsqrt {
                    unsigned_rsqrt_estimate(a)
                } else {
                    unsigned_recip_estimate(a)
                };
                write_elem(&mut dst, off, 4, r as u64);
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Some(Ok(CpuExit::Continue));
        }

        // FRECPE (U=0) / FRSQRTE (U=1): estimate ops, sz_hi=1, opcode 11101.
        if (insn >> 23) & 1 == 1 && opcode == 0b11101 {
            let is_rsqrt = (insn >> 29) & 1 == 1;
            if sz == 1 && q == 0 && !scalar {
                return Some(Err(ArmError::UndefinedInstruction(insn)));
            }
            let esize = if sz == 0 { 4usize } else { 8 };
            let datasize = if scalar {
                esize
            } else if q == 1 {
                16
            } else {
                8
            };
            let elements = datasize / esize;
            let src = self.v[rn].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                let off = e * esize;
                let a = read_elem(&src, off, esize);
                let r = match (is_rsqrt, sz == 0) {
                    (false, true) => fp_recip_estimate_f32(a as u32) as u64,
                    (false, false) => fp_recip_estimate_f64(a),
                    (true, true) => fp_rsqrt_estimate_f32(a as u32) as u64,
                    (true, false) => fp_rsqrt_estimate_f64(a),
                };
                write_elem(&mut dst, off, esize, r);
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Some(Ok(CpuExit::Continue));
        }

        // FRECPX (reciprocal exponent): sz_hi=1, opcode 11111, U=0. Scalar-only
        // in AArch64 (no vector form), so only lane 0 is written and the rest of
        // the register is zeroed.
        if (insn >> 23) & 1 == 1 && opcode == 0b11111 && u == 0 {
            if !scalar {
                return Some(Err(ArmError::UndefinedInstruction(insn)));
            }
            let esize = if sz == 0 { 4usize } else { 8 };
            let a = read_elem(&self.v[rn].to_le_bytes(), 0, esize);
            let mut dst = [0u8; 16];
            write_elem(&mut dst, 0, esize, sve_fp_recpx(esize, a));
            self.v[rd] = u128::from_le_bytes(dst);
            return Some(Ok(CpuExit::Continue));
        }

        if kind.is_none() && cvtf.is_none() {
            return None;
        }

        if sz == 1 && q == 0 && !scalar {
            return Some(Err(ArmError::UndefinedInstruction(insn)));
        }
        let esize = if sz == 0 { 4usize } else { 8 };
        let datasize = if scalar {
            esize
        } else if q == 1 {
            16
        } else {
            8
        };
        let elements = datasize / esize;
        let src = self.v[rn].to_le_bytes();
        let mut dst = [0u8; 16];
        for e in 0..elements {
            let off = e * esize;
            let a = read_elem(&src, off, esize);
            let r = if let Some(unsigned) = cvtf {
                if sz == 0 {
                    let f = if unsigned {
                        a as u32 as f32
                    } else {
                        a as u32 as i32 as f32
                    };
                    f.to_bits() as u64
                } else {
                    let f = if unsigned { a as f64 } else { a as i64 as f64 };
                    f.to_bits()
                }
            } else if sz == 0 {
                fp_two_reg_f32(kind.unwrap(), a as u32) as u64
            } else {
                fp_two_reg_f64(kind.unwrap(), a)
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[rd] = u128::from_le_bytes(dst);
        Some(Ok(CpuExit::Continue))
    }

    // FP helper functions
    fn fp_maxnm_f32(&self, a: f32, b: f32) -> f32 {
        if a.is_nan() {
            b
        } else if b.is_nan() {
            a
        } else {
            a.max(b)
        }
    }

    fn fp_minnm_f32(&self, a: f32, b: f32) -> f32 {
        if a.is_nan() {
            b
        } else if b.is_nan() {
            a
        } else {
            a.min(b)
        }
    }

    fn fp_nmul_f32(&self, a: f32, b: f32) -> f32 {
        -(a * b)
    }

    fn fp_maxnm_f64(&self, a: f64, b: f64) -> f64 {
        if a.is_nan() {
            b
        } else if b.is_nan() {
            a
        } else {
            a.max(b)
        }
    }

    fn fp_minnm_f64(&self, a: f64, b: f64) -> f64 {
        if a.is_nan() {
            b
        } else if b.is_nan() {
            a
        } else {
            a.min(b)
        }
    }

    fn fp_nmul_f64(&self, a: f64, b: f64) -> f64 {
        -(a * b)
    }

    fn fp16_to_f32(h: u16) -> f32 {
        let sign = ((h >> 15) & 1) as u32;
        let exp = ((h >> 10) & 0x1F) as u32;
        let mant = (h & 0x3FF) as u32;

        let f32_bits = if exp == 0 {
            if mant == 0 {
                sign << 31
            } else {
                let mut m = mant;
                let mut e = 0i32;
                while (m & 0x400) == 0 {
                    m <<= 1;
                    e += 1;
                }
                m &= 0x3FF;
                // A binary16 subnormal has value mant*2^-24; once normalised so
                // the implicit 1 sits at bit 10 (after `e` left shifts) the
                // unbiased exponent is -14-e, i.e. biased (127-14-e).
                let new_exp = (127 - 14 - e) as u32;
                (sign << 31) | (new_exp << 23) | (m << 13)
            }
        } else if exp == 0x1F {
            (sign << 31) | (0xFF << 23) | (mant << 13)
        } else {
            let new_exp = exp + 127 - 15;
            (sign << 31) | (new_exp << 23) | (mant << 13)
        };

        f32::from_bits(f32_bits)
    }

    fn f32_to_fp16(f: f32) -> u16 {
        let bits = f.to_bits();
        let sign = ((bits >> 31) & 1) as u16;
        let exp = ((bits >> 23) & 0xFF) as i32;
        let mant = (bits & 0x7FFFFF) as u32;

        if exp == 0xFF {
            if mant == 0 {
                (sign << 15) | (0x1F << 10)
            } else {
                (sign << 15) | (0x1F << 10) | ((mant >> 13) as u16 & 0x3FF).max(1)
            }
        } else {
            // f32 -> f64 is exact, so a single fp16_round is correctly rounded
            // (round-to-nearest-even, with carry into the exponent and the
            // proper overflow/subnormal thresholds). The prior code truncated
            // the mantissa, which lost the rounding bit.
            fp16_round(f as f64)
        }
    }

    // =========================================================================
    // SVE (Scalable Vector Extension) Execution
    // =========================================================================

    /// Execute SVE instruction.
    /// Read SVE predicate register `i` (the low VL/8 bits are meaningful;
    /// 16 bits at VL=128). Exposed for the differential harness.
    pub fn sve_pred(&self, i: usize) -> u32 {
        self.sve_p[i]
    }

    /// Write SVE predicate register `i`. Exposed for the differential harness.
    pub fn set_sve_pred(&mut self, i: usize, v: u32) {
        self.sve_p[i] = v;
    }

    fn exec_sve(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Check if SVE is enabled (CPACR_EL1.ZEN)
        let cpacr = self.sysregs.el1.cpacr;
        let zen = (cpacr >> 16) & 0x3;

        if self.current_el == 0 && zen != 0x3 {
            return Ok(CpuExit::Undefined(insn));
        }
        if self.current_el == 1 && zen == 0x0 {
            return Ok(CpuExit::Undefined(insn));
        }

        // Early rejects for reserved field combinations that broader dispatch
        // arms below would otherwise accept.
        let top = (insn >> 24) & 0xFF;
        // 0x04, bit21==1, bits[15:13]==100, bit12==0: unpredicated shift by
        // wide elements (ASR/LSR/LSL Zd, Zn, Zm.D). The wide operand is .D, so
        // doubleword element size is unallocated.
        if top == 0b0000_0100
            && (insn >> 21) & 1 == 1
            && (insn >> 13) & 0x7 == 0b100
            && (insn >> 12) & 1 == 0
            && (insn >> 22) & 0x3 == 0b11
        {
            return Ok(CpuExit::Undefined(insn));
        }
        // 0x25 integer arith with shifted immediate (bits[21:19]==100,
        // bits[15:14]==11): sh=1 with byte elements is unallocated.
        if top == 0b0010_0101
            && (insn >> 19) & 0x7 == 0b100
            && (insn >> 14) & 0x3 == 0b11
            && (insn >> 13) & 1 == 1
            && (insn >> 22) & 0x3 == 0
        {
            return Ok(CpuExit::Undefined(insn));
        }
        // 0x05 bitwise logical with immediate (AND/EOR/ORR/DUPM space,
        // bits[21:18]==x00000 family): a reserved imm13 pattern is unallocated.
        if top == 0b0000_0101 && (insn >> 18) & 0xF == 0b0000 {
            let n = (insn >> 17) & 1 == 1;
            let immr = (insn >> 11) & 0x3F;
            let imms = (insn >> 5) & 0x3F;
            if decode_bitmask(n, imms, immr, true).is_err() {
                return Ok(CpuExit::Undefined(insn));
            }
        }

        // Extract primary classification bits
        let op0 = (insn >> 29) & 0x7;
        let op1 = (insn >> 23) & 0x3;
        let op2 = (insn >> 17) & 0x1F;
        let op3 = (insn >> 10) & 0x3F;

        // Common register fields
        let zd = (insn & 0x1F) as usize;
        let zn = ((insn >> 5) & 0x1F) as usize;
        let zm = ((insn >> 16) & 0x1F) as usize;
        let pg = ((insn >> 10) & 0x7) as usize;
        let size = (insn >> 22) & 0x3;

        // Element size in bytes
        let esize = 1usize << size; // 1, 2, 4, or 8 bytes

        match op0 {
            // EXT (destructive): 0x05, bits[23:21]==001, bits[15:13]==000.
            // Zdn.B = (Zm:Zdn) extracted at byte offset imm8 (imm8h:imm8l).
            // Must precede the int_unpred arm below (which shares bit21==1 &&
            // bits[15:13]==000 but does not check the op byte). At VL=128 there
            // are 16 byte-elements; if imm8>=16 the offset wraps to 0.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 0x7 == 0b001
                    && (insn >> 13) & 0x7 == 0b000 =>
            {
                let imm8 = (((insn >> 16) & 0x1F) << 3) | ((insn >> 10) & 0x7);
                let low = self.v[zd]; // operand1 (Zdn) = low half of concat
                let high = self.v[zn]; // operand2 (Zm)  = high half of concat
                let pos = if imm8 >= 16 { 0 } else { imm8 };
                let s = pos * 8; // byte offset -> bit offset (0..=120)
                self.v[zd] = if s == 0 {
                    low
                } else {
                    (low >> s) | (high << (128 - s))
                };
                Ok(CpuExit::Continue)
            }

            // SVE2.1 DUPQ (broadcast indexed element within each 128-bit
            // segment): 0x05, bits[23:22]==00, bit21==1, bits[15:10]==001001. The
            // element size and segment index are packed into the tsz field
            // bits[20:16] (lowest set bit selects esize; the index is above it).
            // At VL=128 there is one segment, so Zn[index] fills every lane.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 22) & 0x3 == 0b00
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001001 =>
            {
                let tsz = (insn >> 16) & 0x1F;
                if tsz == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let size = tsz.trailing_zeros() as usize;
                let esz = 1usize << size;
                let index = (tsz >> (size + 1)) as usize;
                let nsegelt = 16 / esz;
                let src = self.v[zn].to_le_bytes();
                let val = if index < nsegelt {
                    read_elem(&src, index * esz, esz)
                } else {
                    0
                };
                let mut dst = [0u8; 16];
                for e in 0..nsegelt {
                    write_elem(&mut dst, e * esz, esz, val);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2.1 EXTQ (extract within each 128-bit segment): 0x05,
            // bits[23:20]==0110, bits[15:10]==001001. imm4=bits[19:16] is a byte
            // offset (0..15). Destructive: the concatenation (Zm:Zdn) of the
            // 128-bit segment is shifted right imm bytes; at VL=128 this matches
            // EXT with a 4-bit immediate (Zdn=Zd field, Zm=Zn field).
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 20) & 0xF == 0b0110
                    && (insn >> 10) & 0x3F == 0b001001 =>
            {
                let imm = ((insn >> 16) & 0xF) as usize; // bytes
                let low = self.v[zd]; // Zdn
                let high = self.v[zn]; // Zm
                let s = imm * 8;
                self.v[zd] = if s == 0 {
                    low
                } else {
                    (low >> s) | (high << (128 - s))
                };
                Ok(CpuExit::Continue)
            }

            // SVE2.1 REVD (reverse 64-bit doublewords within each 128-bit
            // segment, predicated/merging): 0x05, bits[23:16]==0b00101110,
            // bits[15:13]==100. Pg=bits[12:10]; the swap of a 128-bit segment is
            // governed by the predicate bit of its low doubleword. At VL=128 the
            // single segment swaps its two 64-bit halves iff predicate bit 0 set.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 16) & 0xFF == 0b00101110
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                let pg = ((insn >> 10) & 0x7) as usize;
                let pred = self.sve_p[pg];
                if pred & 1 == 1 {
                    let v = self.v[zn];
                    self.v[zd] = (v >> 64) | (v << 64);
                }
                // inactive low-doubleword -> merge (Zd unchanged)
                Ok(CpuExit::Continue)
            }

            // SVE2.1 PMOV (move a bit-plane between a vector and a predicate):
            // 0x05, bits[21:19]==101, bits[15:10]==001110. bit16 selects the
            // direction (0: Zn -> Pd, 1: Pn -> Zd). The element size and bit-plane
            // index are tsz-encoded in bits[23:22]:bits[18:17]. Per qemu pmov_pv/
            // pmov_vp, predicate bit e*esize maps to vector bit elements*idx+e
            // (elements = 16/esize). The vp form zeroes Zd only when idx==0 (else
            // it merges the selected plane).
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 19) & 0x7 == 0b101
                    && (insn >> 10) & 0x3F == 0b001110 =>
            {
                let sh = (insn >> 22) & 0x3;
                let sl = (insn >> 17) & 0x3;
                let (esz, idx): (usize, usize) = if sh == 0b00 && sl == 0b01 {
                    (0, 0)
                } else if sh == 0b00 && sl & 0b10 == 0b10 {
                    (1, (sl & 1) as usize)
                } else if sh == 0b01 {
                    (2, sl as usize)
                } else if sh & 0b10 == 0b10 {
                    (3, ((((sh & 1) << 2) | sl) as usize))
                } else {
                    return Ok(CpuExit::Undefined(insn));
                };
                let esize = 1usize << esz;
                let elements = 16 / esize;
                if (insn >> 16) & 1 == 0 {
                    // PMOV Pd.T, Zn[idx]: Zn -> Pd.
                    let pd = (insn & 0xF) as usize;
                    let z = self.v[((insn >> 5) & 0x1F) as usize];
                    let mut p = 0u32;
                    for e in 0..elements {
                        let bit = (z >> (elements * idx + e)) & 1;
                        p |= (bit as u32) << (e * esize);
                    }
                    self.sve_p[pd] = p;
                } else {
                    // PMOV Zd[idx], Pn.T: Pn -> Zd.
                    let zd = (insn & 0x1F) as usize;
                    let p = self.sve_p[((insn >> 5) & 0xF) as usize];
                    let mut z = if idx == 0 { 0u128 } else { self.v[zd] };
                    for e in 0..elements {
                        let pos = elements * idx + e;
                        let bit = ((p >> (e * esize)) & 1) as u128;
                        z = (z & !(1u128 << pos)) | (bit << pos);
                    }
                    self.v[zd] = z;
                }
                Ok(CpuExit::Continue)
            }

            // Unpredicated integer add/subtract (ADD/SUB/SQADD/UQADD/SQSUB/
            // UQSUB): bit21==1, bits[15:13]==000. Size is the full bits[23:22],
            // so this must NOT be gated on op1 (which folds size's high bit).
            0b000 if (insn >> 21) & 1 == 1 && (insn >> 13) & 0x7 == 0b000 => {
                self.exec_sve_int_unpred(insn, zd, zn, zm, esize)
            }

            // TBL (table lookup, single table): 0x05, bit21==1,
            // bits[15:10]==001100. Shares bits[15:10] with the unpredicated
            // logical arm below (0x04) so it MUST precede it and gate on 0x05.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001100 =>
            {
                self.exec_sve_tbl(zd, zn, zm, esize)
            }

            // TBX (table lookup, keep destination for out-of-range): 0x05,
            // bit21==1, bits[15:10]==001011. Like TBL but unmatched indices
            // preserve the existing Zd element instead of zeroing it.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001011 =>
            {
                self.exec_sve_tbx(zd, zn, zm, esize)
            }

            // SVE2.1 TBXQ (per-128-bit-segment table lookup, keep destination):
            // 0x05, bit21==1, bits[15:10]==001101. At VL=128 (one segment) this
            // is identical to TBX over the whole register.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001101 =>
            {
                self.exec_sve_tbx(zd, zn, zm, esize)
            }

            // TBL2 (two-register table lookup): 0x05, bit21==1,
            // bits[15:10]==001010. The tables are {Zn, Zn+1}; out-of-range
            // indices yield 0.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001010 =>
            {
                let t0 = self.v[zn].to_le_bytes();
                let t1 = self.v[(zn + 1) % 32].to_le_bytes();
                let idx = self.v[zm].to_le_bytes();
                let n = 16 / esize;
                let mut dst = [0u8; 16];
                for e in 0..n {
                    let off = e * esize;
                    let i = read_elem(&idx, off, esize) as usize;
                    let val = if i < n {
                        read_elem(&t0, i * esize, esize)
                    } else if i < 2 * n {
                        read_elem(&t1, (i - n) * esize, esize)
                    } else {
                        0
                    };
                    write_elem(&mut dst, off, esize, val);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // Predicate ZIP/UZP/TRN (ZIP1_p..TRN2_p): 0x05, bits[21:20]==10,
            // bits[15:13]==010. opc=bits[12:10]. Permutes the esize-bit chunks
            // of the byte-granular predicates Pn (bits[8:5]) and Pm (bits[19:16]).
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 20) & 0x3 == 0b10
                    && (insn >> 13) & 0x7 == 0b010 =>
            {
                let opc = (insn >> 10) & 0x7;
                let esize = 1usize << ((insn >> 22) & 0x3);
                let n = 16 / esize;
                let cmask = (1u32 << esize) - 1;
                let pn = self.sve_p[((insn >> 5) & 0xF) as usize];
                let pm = self.sve_p[((insn >> 16) & 0xF) as usize];
                let chunk = |p: u32, i: usize| (p >> (i * esize)) & cmask;
                let mut out = 0u32;
                let h = n / 2;
                for i in 0..h {
                    let (lo, hi) = match opc {
                        0b000 => (chunk(pn, i), chunk(pm, i)),                 // ZIP1
                        0b001 => (chunk(pn, h + i), chunk(pm, h + i)),         // ZIP2
                        0b100 => (chunk(pn, 2 * i), chunk(pm, 2 * i)),         // TRN1
                        0b101 => (chunk(pn, 2 * i + 1), chunk(pm, 2 * i + 1)), // TRN2
                        _ => (0, 0),
                    };
                    if matches!(opc, 0b000 | 0b001 | 0b100 | 0b101) {
                        out |= lo << (2 * i * esize);
                        out |= hi << ((2 * i + 1) * esize);
                    }
                }
                if opc == 0b010 || opc == 0b011 {
                    let odd = opc == 0b011; // UZP2 takes odd elements
                    for i in 0..n {
                        let v = if i < h {
                            chunk(pn, 2 * i + odd as usize)
                        } else {
                            chunk(pm, 2 * (i - h) + odd as usize)
                        };
                        out |= v << (i * esize);
                    }
                } else if opc > 0b101 {
                    return Ok(CpuExit::Undefined(insn));
                }
                self.sve_p[(insn & 0xF) as usize] = out;
                Ok(CpuExit::Continue)
            }

            // Predicate REV (REV_p): 0x05, bits[21:16]==110100, bits[15:10]==
            // 010000. Reverse the esize-bit chunks of the predicate Pn.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 16) & 0x3F == 0b110100
                    && (insn >> 10) & 0x3F == 0b010000 =>
            {
                let esize = 1usize << ((insn >> 22) & 0x3);
                let n = 16 / esize;
                let cmask = (1u32 << esize) - 1;
                let pn = self.sve_p[((insn >> 5) & 0xF) as usize];
                let mut out = 0u32;
                for i in 0..n {
                    out |= ((pn >> ((n - 1 - i) * esize)) & cmask) << (i * esize);
                }
                self.sve_p[(insn & 0xF) as usize] = out;
                Ok(CpuExit::Continue)
            }

            // PUNPKLO/PUNPKHI (predicate unpack): 0x05, bits[23:20]==0011,
            // bits[19:17]==000, bits[15:10]==010000. Each of the low (lo) / high
            // (hi, bit16) 8 source predicate bits expands to bit 2i of the dest.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 22) & 0x3 == 0
                    && (insn >> 20) & 0x3 == 0b11
                    && (insn >> 17) & 0x7 == 0
                    && (insn >> 10) & 0x3F == 0b010000 =>
            {
                let hi = (insn >> 16) & 1 == 1;
                let pn = self.sve_p[((insn >> 5) & 0xF) as usize];
                let pd = (insn & 0xF) as usize;
                let base = if hi { 8 } else { 0 };
                let mut out = 0u32;
                for i in 0..8 {
                    if (pn >> (base + i)) & 1 == 1 {
                        out |= 1 << (2 * i);
                    }
                }
                self.sve_p[pd] = out;
                Ok(CpuExit::Continue)
            }

            // DUP (indexed broadcast) Zd.T, Zn.T[index]: 0x05, bit21==1,
            // bits[15:10]==001000. esize and index come from the tsz:imm2 field
            // (lowest set bit of tsz selects esize).
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001000 =>
            {
                self.exec_sve_dup_indexed(insn, zn, zd)
            }

            // COMPACT (pack active elements down): 0x05, bit23==1,
            // bits[21:16]==100001, bits[15:13]==100. S/D elements only.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 23) & 1 == 1
                    && (insn >> 16) & 0x3F == 0b100001
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                self.exec_sve_compact(insn, zd, zn, pg)
            }

            // SPLICE (destructive): 0x05, bits[21:16]==101100, bits[15:13]==100.
            // Zdn's active span is packed low, the rest filled from Zm.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 16) & 0x3F == 0b101100
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                self.exec_sve_splice(insn, zd, zn, pg)
            }

            // Unpredicated bitwise logical (AND/ORR/EOR/BIC): bits[15:10]=001100.
            0b000 if (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3F == 0b001100 => {
                self.exec_sve_logical_unpred(insn, zd, zn, zm)
            }

            // SVE2 unpredicated multiply: 0x04, bit21==1, bits[15:12]==0110,
            // bits[11:10] opc (00=MUL, 01=PMUL byte-only, 10=SMULH, 11=UMULH).
            // The 0x05 sibling of bits[15:12]==0110 is ZIP/UZP/TRN, so this MUST
            // gate on the op byte. PMUL is defined for byte elements only.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 12) & 0xF == 0b0110
                    && ((insn >> 10) & 0x3 != 0b01 || esize == 1) =>
            {
                let opc = (insn >> 10) & 0x3;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let off = e * esize;
                    let x = read_elem(&a, off, esize);
                    let y = read_elem(&b, off, esize);
                    let r = match opc {
                        0b00 => x.wrapping_mul(y) & mask,
                        0b01 => poly_mul_8(x, y), // PMUL.B (carry-less)
                        0b10 => ((sext_elem(x, bits) * sext_elem(y, bits)) >> bits) as u64 & mask,
                        _ => ((uext_elem(x, bits) * uext_elem(y, bits)) >> bits) as u64 & mask,
                    };
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 bitwise ternary (whole-register): 0x04, bit21==1,
            // bits[15:11]==00111. Zdn=bits[4:0], Zk=bits[9:5], Zm=bits[20:16];
            // opc=bits[23:22], o2=bit10 select the operation.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 11) & 0x1F == 0b00111 =>
            {
                let opc = (insn >> 22) & 0x3;
                let o2 = (insn >> 10) & 1;
                let dn = self.v[zd]; // Zdn (first source + destination)
                let k = self.v[zn]; // Zk (select mask)
                let m = self.v[zm]; // Zm (second source)
                self.v[zd] = match (opc, o2) {
                    (0b00, 0) => dn ^ m ^ k,             // EOR3
                    (0b01, 0) => dn ^ (m & !k),          // BCAX
                    (0b00, 1) => (dn & k) | (m & !k),    // BSL
                    (0b01, 1) => (!dn & k) | (m & !k),   // BSL1N
                    (0b10, 1) => (dn & k) | (!m & !k),   // BSL2N
                    (0b11, 1) => !((dn & k) | (m & !k)), // NBSL
                    _ => return Ok(CpuExit::Undefined(insn)),
                };
                Ok(CpuExit::Continue)
            }

            // SVE FEXPA (exponential accelerator): 0x04, bit21==1,
            // bits[20:16]==00000, bits[15:10]==101110. Unpredicated table lookup.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 16) & 0x1F == 0b00000
                    && (insn >> 10) & 0x3F == 0b101110 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esz = 1usize << size;
                let n = self.v[zn].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..(16 / esz) {
                    let off = e * esz;
                    write_elem(&mut dst, off, esz, sve_fexpa(esz, read_elem(&n, off, esz)));
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE REVB/REVH/REVW/RBIT (predicated, merging): 0x05, bit21==1,
            // bits[20:18]==001, bits[15:13]==100. bits[17:16]: 00=REVB (reverse
            // bytes within each element), 01=REVH (halfwords), 10=REVW (words),
            // 11=RBIT (bits). @rd_pg_rn.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 18) & 0x7 == 0b001
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                let op = (insn >> 16) & 0x3;
                let unit = match op {
                    0b00 => 1usize,          // REVB
                    0b01 if esize >= 4 => 2, // REVH (S/D)
                    0b10 if esize == 8 => 4, // REVW (D)
                    0b11 => 0,               // RBIT
                    _ => return Ok(CpuExit::Undefined(insn)),
                };
                if op == 0b00 && esize < 2 {
                    return Ok(CpuExit::Undefined(insn)); // REVB.b is reserved
                }
                let pg = ((insn >> 10) & 0x7) as usize;
                let rn = ((insn >> 5) & 0x1F) as usize;
                let pred = self.sve_p[pg];
                let mask = elem_mask((esize * 8) as u32);
                let src = self.v[rn].to_le_bytes();
                let mut dst = self.v[zd].to_le_bytes();
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    if (pred >> off) & 1 == 0 {
                        continue;
                    }
                    let v = read_elem(&src, off, esize);
                    let r = if op == 0b11 {
                        (v & mask).reverse_bits() >> (64 - esize * 8)
                    } else {
                        reverse_chunks(v, esize, unit) & mask
                    };
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE INSR (insert scalar, shifting the vector up one element): 0x05,
            // bit21==1, bits[15:10]==001110, bits[20:16]==00100 (GPR) or 10100
            // (SIMD scalar). New Zdn = [scalar, Zdn[0..N-1]].
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001110
                    && matches!((insn >> 16) & 0x1F, 0b00100 | 0b10100) =>
            {
                let insr_f = (insn >> 20) & 1 == 1; // SIMD&FP scalar form
                let rmf = ((insn >> 5) & 0x1F) as usize;
                let esbits = esize * 8;
                let smask: u128 = (1u128 << esbits) - 1;
                let scalar = if insr_f {
                    self.v[rmf] & smask
                } else {
                    (self.get_x(rmf as u8) as u128) & smask
                };
                self.v[zd] = (self.v[zd] << esbits) | scalar;
                Ok(CpuExit::Continue)
            }

            // SVE CLASTA/CLASTB to vector or SIMD&FP scalar: 0x05,
            // bits[21:17]==10100 (vector) / 10101 (scalar), bit16=A(0)/B(1),
            // bits[15:13]==100. The element at (CLASTB) / after (CLASTA) the last
            // active lane of Zm is broadcast to Zdn (vector) or written to Vd
            // (scalar); with no active lane the destination is unchanged.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && matches!((insn >> 17) & 0x1F, 0b10100 | 0b10101)
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                let scalar_form = (insn >> 17) & 1 == 1;
                let before = (insn >> 16) & 1 == 1; // CLASTB
                let pg = ((insn >> 10) & 0x7) as usize;
                let src_reg = ((insn >> 5) & 0x1F) as usize;
                let n = 16 / esize;
                let pred = self.sve_p[pg];
                let mask = elem_mask((esize * 8) as u32);
                let src = self.v[src_reg].to_le_bytes();
                let mut last: i32 = -1;
                for e in (0..n).rev() {
                    if (pred >> (e * esize)) & 1 == 1 {
                        last = e as i32;
                        break;
                    }
                }
                let selected = if last >= 0 {
                    let idx = if before {
                        last as usize
                    } else {
                        let i = (last + 1) as usize;
                        if i >= n { 0 } else { i }
                    };
                    Some(read_elem(&src, idx * esize, esize) & mask)
                } else {
                    None
                };
                if scalar_form {
                    // Writing to a SIMD&FP scalar always zeroes the upper bits;
                    // with no active element the prior low element is preserved.
                    let val = selected.unwrap_or((self.v[zd] as u64) & mask);
                    self.v[zd] = val as u128;
                } else if let Some(val) = selected {
                    // Vector form: broadcast; unchanged if no active element.
                    let mut out = 0u128;
                    for e in 0..n {
                        out |= (val as u128) << (e * esize * 8);
                    }
                    self.v[zd] = out;
                }
                Ok(CpuExit::Continue)
            }

            // SVE FCPY (copy FP immediate into Pg-active lanes, merging): 0x05,
            // bits[21:20]==01, bits[15:13]==110. Pg is 4-bit (bits[19:16]).
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 20) & 0x3 == 0b01
                    && (insn >> 13) & 0x7 == 0b110 =>
            {
                if esize < 2 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let pg = ((insn >> 16) & 0xF) as usize;
                let imm8 = ((insn >> 5) & 0xFF) as u8;
                let val = vfp_expand_imm(imm8, esize);
                let pred = self.sve_p[pg];
                let mut dst = self.v[zd].to_le_bytes();
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    if (pred >> off) & 1 == 1 {
                        write_elem(&mut dst, off, esize, val);
                    }
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE DUPM (broadcast logical-immediate mask): 0x05, bits[23:18]==
            // 110000. The N:immr:imms field decodes a 64-bit mask broadcast to
            // every doubleword lane.
            0b000 if (insn >> 24) & 0xFF == 0b00000101 && (insn >> 18) & 0x3F == 0b110000 => {
                let n = (insn >> 17) & 1 == 1;
                let immr = (insn >> 11) & 0x3F;
                let imms = (insn >> 5) & 0x3F;
                match decode_bitmask(n, imms, immr, true) {
                    Ok(imm) => {
                        self.v[zd] = (imm as u128) | ((imm as u128) << 64);
                        Ok(CpuExit::Continue)
                    }
                    Err(_) => Ok(CpuExit::Undefined(insn)),
                }
            }

            // SVE UNPK (SUNPKHI/LO, UUNPKHI/LO): 0x05, bits[21:18]==1100,
            // bits[15:10]==001110. Unpack the low (h=0) or high (h=1) half of
            // Zn, sign- (u=0) or zero- (u=1) extending each half-width element.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 18) & 0xF == 0b1100
                    && (insn >> 10) & 0x3F == 0b001110 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let mask = elem_mask((d_esize * 8) as u32);
                let unsigned = (insn >> 17) & 1 == 1;
                let hi = (insn >> 16) & 1 == 1;
                let src = self.v[zn].to_le_bytes();
                let n_dst = 16 / d_esize;
                let mut dst = [0u8; 16];
                for e in 0..n_dst {
                    let src_idx = (if hi { n_dst } else { 0 }) + e;
                    let sv = read_elem(&src, src_idx * s_esize, s_esize);
                    let r = if unsigned {
                        uext_elem(sv, s_bits) as u64
                    } else {
                        sext_elem(sv, s_bits) as u64
                    };
                    write_elem(&mut dst, e * d_esize, d_esize, r & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE FTSSEL (trigonometric select coefficient): 0x04, bit21==1,
            // bits[15:10]==101100. Per lane: result = Zm[e]&1 ? 1.0 : Zn[e];
            // then if Zm[e]&2 negate. Unpredicated; Zn=bits[9:5], Zm=bits[20:16].
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b101100 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esize = 1usize << size;
                let one: u64 = match esize {
                    2 => 0x3C00,
                    4 => 0x3F80_0000,
                    _ => 0x3FF0_0000_0000_0000,
                };
                let signbit: u64 = 1 << (esize * 8 - 1);
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    let mm = read_elem(&m, off, esize);
                    let mut nn = read_elem(&n, off, esize);
                    if mm & 1 != 0 {
                        nn = one;
                    }
                    if mm & 2 != 0 {
                        nn ^= signbit;
                    }
                    write_elem(&mut dst, off, esize, nn);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 XAR (exclusive-or and rotate right by immediate): 0x04,
            // bit21==1, bits[15:10]==001101. Zdn=bits[4:0], Zm=bits[9:5]; the
            // tsz:imm3 field gives the element size and rotate amount (1..bits).
            // Destructive: Zdn = ROR(Zdn ^ Zm, amount) per element.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b001101 =>
            {
                let tsz = (((insn >> 22) & 0x3) << 2) | ((insn >> 19) & 0x3);
                if tsz == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let bits: u32 = if tsz & 0b1000 != 0 {
                    64
                } else if tsz & 0b0100 != 0 {
                    32
                } else if tsz & 0b0010 != 0 {
                    16
                } else {
                    8
                };
                let esize = (bits / 8) as usize;
                let tszimm = (tsz << 3) | ((insn >> 16) & 0x7);
                let amount = (2 * bits - tszimm) % bits; // 1..bits, bits == identity
                let a = self.v[zd].to_le_bytes();
                let b = self.v[zn].to_le_bytes();
                let mask = elem_mask(bits);
                let mut dst = [0u8; 16];
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    let x = (read_elem(&a, off, esize) ^ read_elem(&b, off, esize)) & mask;
                    let r = if amount == 0 {
                        x
                    } else {
                        ((x >> amount) | (x << (bits - amount))) & mask
                    };
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 SQDMULH/SQRDMULH (unpredicated saturating doubling multiply
            // high): 0x04, bit21==1, bits[15:11]==01110. R=bit10 adds rounding.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 11) & 0x1F == 0b01110 =>
            {
                let round = (insn >> 10) & 1 == 1;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                let hi = (1i128 << (bits - 1)) - 1;
                let lo = -(1i128 << (bits - 1));
                for e in 0..elements {
                    let off = e * esize;
                    let prod = sext_elem(read_elem(&a, off, esize), bits)
                        * sext_elem(read_elem(&b, off, esize), bits);
                    let high = if round {
                        (prod + (1i128 << (bits - 2))) >> (bits - 1)
                    } else {
                        prod >> (bits - 1)
                    };
                    write_elem(&mut dst, off, esize, high.clamp(lo, hi) as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE element count / inc-dec / stack allocation (0x04). The stack
            // forms (ADDVL/ADDPL/RDVL, bits[15:11]==01010) share bits[15:13]==010
            // with INDEX but differ at bit12; the count forms have bits[15:14]==11.
            // Routed before INDEX so the stack forms are not mis-decoded.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (((insn >> 11) & 0x1F == 0b01010)
                        || ((insn >> 21) & 1 == 1
                            && matches!((insn >> 12) & 0xF, 0b1100 | 0b1110 | 0b1111))) =>
            {
                self.exec_sve_elem_count(insn)
            }

            // INDEX (immediate/scalar variants): bit21==1, bits[15:13]==010.
            0b000 if (insn >> 21) & 1 == 1 && (insn >> 13) & 0x7 == 0b010 => {
                self.exec_sve_index(insn, zd, esize)
            }

            // ZIP/UZP/TRN (unpredicated permute): 0x05, bit21==1, bits[15:13]==011.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x7 == 0b011 =>
            {
                self.exec_sve_zip_uzp_trn(insn, zd, zn, zm, esize)
            }

            // SEL Zd.T, Pg, Zn, Zm: 0x05, bit21==1, bits[15:14]==11. Per-element
            // merge governed by the 4-bit predicate Pg (bits[13:10]).
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 14) & 0x3 == 0b11 =>
            {
                let pg = ((insn >> 10) & 0xF) as usize;
                let pred = self.sve_p[pg];
                let elements = 16 / esize;
                let n_reg = self.v[zn].to_le_bytes();
                let m_reg = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let off = e * esize;
                    let src = if (pred >> (e * esize)) & 1 == 1 {
                        &n_reg
                    } else {
                        &m_reg
                    };
                    write_elem(&mut dst, off, esize, read_elem(src, off, esize));
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // REV Zd.T, Zn.T (reverse all elements): 0x05, bits[20:16]==11000,
            // bits[15:10]==001110.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 16) & 0x1F == 0b11000
                    && (insn >> 10) & 0x3F == 0b001110 =>
            {
                let n = 16 / esize;
                let a = self.v[zn].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..n {
                    write_elem(
                        &mut dst,
                        e * esize,
                        esize,
                        read_elem(&a, (n - 1 - e) * esize, esize),
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // CPY/MOV (predicated copy of immediate / scalar GPR / SIMD scalar),
            // all in the 0x05 space.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 20) & 0x3 == 0b01
                    && (insn >> 15) & 1 == 0 =>
            {
                self.exec_sve_cpy(insn, esize, 0) // CPY immediate
            }
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 16) & 0x3F == 0b101000
                    && (insn >> 13) & 0x7 == 0b101 =>
            {
                self.exec_sve_cpy(insn, esize, 1) // CPY scalar GPR
            }
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 16) & 0x3F == 0b100000
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                self.exec_sve_cpy(insn, esize, 2) // CPY SIMD scalar
            }

            // LASTA/LASTB/CLASTA/CLASTB -> GPR: 0x05, bits[15:13]==101, bit21==1,
            // bits[19:17]==000. bit20: 0=LAST, 1=CLAST; bit16: 0=A (after), 1=B.
            0b000
                if (insn >> 24) & 0xFF == 0b00000101
                    && (insn >> 13) & 0x7 == 0b101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 17) & 0x7 == 0b000 =>
            {
                self.exec_sve_lastx(insn, esize)
            }

            // ADR (vector address generation): 0x04, bit21==1, bits[15:12]==1010.
            // Zd[e] = Zn[e] + offset(Zm[e]) * 2^msz. bits[23:22] selects the
            // form: 00=D+SXTW(Zm<31:0>), 01=D+UXTW(Zm<31:0>), 10=S packed,
            // 11=D packed. msz = bits[11:10].
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 12) & 0xF == 0b1010 =>
            {
                let mode = (insn >> 22) & 0x3;
                let msz = (insn >> 10) & 0x3;
                let esize = if mode == 0b10 { 4 } else { 8 };
                let elements = 16 / esize;
                let m = elem_mask((esize * 8) as u32);
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let off = e * esize;
                    let base = read_elem(&a, off, esize);
                    let zmv = read_elem(&b, off, esize);
                    let offset = match mode {
                        0b00 => (zmv as u32 as i32 as i64 as u64) << msz, // SXTW
                        0b01 => (zmv as u32 as u64) << msz,               // UXTW
                        _ => zmv << msz,                                  // packed S/D
                    };
                    write_elem(&mut dst, off, esize, base.wrapping_add(offset) & m);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // MOVPRFX Zd, Zn (unpredicated whole-register copy): 0x04,
            // bits[23:16]==00100000, bits[15:10]==101111. Standalone it is a
            // plain vector move.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 16) & 0xFF == 0b00100000
                    && (insn >> 10) & 0x3F == 0b101111 =>
            {
                self.v[zd] = self.v[zn];
                Ok(CpuExit::Continue)
            }

            // MOVPRFX Zd.T, Pg/M-or-Z, Zn.T (predicated copy): 0x04,
            // bits[21:18]==0100, bit17==0, bits[15:13]==001. Active lanes copy
            // Zn; inactive lanes merge (M=1, keep Zd) or zero (M=0). Must precede
            // the integer-reduction arm, which shares bit21==0 && bits[15:13]==001.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 18) & 0xF == 0b0100
                    && (insn >> 17) & 1 == 0
                    && (insn >> 13) & 0x7 == 0b001 =>
            {
                let merging = (insn >> 16) & 1 == 1;
                let pred = self.sve_p[pg];
                let elements = 16 / esize;
                let n_reg = self.v[zn].to_le_bytes();
                let mut dst = self.v[zd].to_le_bytes(); // merging base = prior Zd
                for e in 0..elements {
                    let off = e * esize;
                    if (pred >> off) & 1 == 1 {
                        write_elem(&mut dst, off, esize, read_elem(&n_reg, off, esize));
                    } else if !merging {
                        write_elem(&mut dst, off, esize, 0);
                    }
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2.1 integer quadword reductions (ADDQV/SMAXQV/UMAXQV/SMINQV/
            // UMINQV/ANDQV/ORQV/EORQV): 0x04, bit21==0, bits[15:13]==001, and
            // bit18==1 (the QV opcodes set the high bit of bits[18:16], unlike the
            // scalar reductions below). Reduces each element position across the
            // 128-bit segments into a single quadword in Vd. Must precede the
            // scalar-reduction arm, which shares bit21==0 && bits[15:13]==001.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 13) & 0x7 == 0b001
                    && (insn >> 18) & 1 == 1 =>
            {
                self.exec_sve_qv_reduce_int(insn, esize)
            }

            // Integer reductions to a scalar (SADDV/UADDV/SMAXV/.../ANDV/ORV/
            // EORV): bit21==0, bits[15:13]==001.
            0b000 if (insn >> 21) & 1 == 0 && (insn >> 13) & 0x7 == 0b001 => {
                self.exec_sve_int_reduce(insn, esize)
            }

            // Predicated shift by vector (ASR/LSR/LSL Zdn, Pg/M, Zdn, Zm):
            // bits[15:13]==100, bits[21:19]==010.
            0b000
                if (insn >> 13) & 0x7 == 0b100
                    && (insn >> 19) & 0x7 == 0b010
                    && (insn >> 21) & 1 == 0 =>
            {
                self.exec_sve_shift_pred(insn, zd, zn, pg, esize)
            }

            // Predicated shift by immediate: bits[15:13]==100, bits[21:20]==00
            // (bits[21:19] 000 => ASR/LSR/LSL/ASRD/SQSHL/UQSHL, 001 => SRSHR/
            // URSHR/SQSHLU).
            0b000 if (insn >> 13) & 0x7 == 0b100 && (insn >> 20) & 0x3 == 0b00 => {
                self.exec_sve_shift_imm(insn)
            }

            // SVE predicated integer/FP unary (merging): 0x04, bits[15:13]==101,
            // bits[21:19] in {010,011}. opc=bits[21:16] selects SXTB/H/W, UXTB/
            // H/W, ABS, NEG, CLS, CLZ, CNT, CNOT, FABS, FNEG, NOT.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 13) & 0x7 == 0b101
                    && matches!((insn >> 19) & 0x7, 0b010 | 0b011) =>
            {
                let opc = (insn >> 16) & 0x3F;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let signbit = 1u64 << (bits - 1);
                let pred = self.sve_p[pg];
                let src = self.v[zn].to_le_bytes();
                let mut dst = self.v[zd].to_le_bytes();
                // Validity: the extend width must be smaller than the element.
                // Checked up front — an unallocated size is UNDEF even when
                // the governing predicate has no active elements.
                let ext_ok = |w: u32| bits > w;
                let opc_valid = match opc {
                    0b010000 | 0b010001 => ext_ok(8),
                    0b010010 | 0b010011 => ext_ok(16),
                    0b010100 | 0b010101 => ext_ok(32),
                    0b010110..=0b011011 | 0b011110 => true,
                    0b011100 | 0b011101 => esize >= 2,
                    _ => false,
                };
                if !opc_valid {
                    return Ok(CpuExit::Undefined(insn));
                }
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    if (pred >> off) & 1 == 0 {
                        continue;
                    }
                    let v = read_elem(&src, off, esize);
                    let r: u64 = match opc {
                        0b010000 if ext_ok(8) => sext_elem(v, 8) as u64 & mask, // SXTB
                        0b010001 if ext_ok(8) => v & 0xFF,                      // UXTB
                        0b010010 if ext_ok(16) => sext_elem(v, 16) as u64 & mask, // SXTH
                        0b010011 if ext_ok(16) => v & 0xFFFF,                   // UXTH
                        0b010100 if ext_ok(32) => sext_elem(v, 32) as u64 & mask, // SXTW
                        0b010101 if ext_ok(32) => v & 0xFFFF_FFFF,              // UXTW
                        0b010110 => sext_elem(v, bits).unsigned_abs() as u64 & mask, // ABS
                        0b010111 => (-sext_elem(v, bits)) as u64 & mask,        // NEG
                        0b011000 => count_leading_sign(v, bits),                // CLS
                        0b011001 => count_leading_zeros_elem(v, bits),          // CLZ
                        0b011010 => (v & mask).count_ones() as u64,             // CNT
                        0b011011 => u64::from(v & mask == 0),                   // CNOT
                        0b011100 if esize >= 2 => v & !signbit,                 // FABS
                        0b011101 if esize >= 2 => v ^ signbit,                  // FNEG
                        0b011110 => !v & mask,                                  // NOT
                        _ => return Ok(CpuExit::Undefined(insn)),
                    };
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE predicated integer multiply-add: 0x04, bit21==0, bits[15:13]
            // ==010 (MLA: d=Za+Zn*Zm), 011 (MLS: d=Za-Zn*Zm), 110 (MAD:
            // d=Za+Zdn*Zm), 111 (MSB: d=Za-Zdn*Zm). Low-half integer multiply,
            // merging. MLA/MLS keep Za in Zd; MAD/MSB keep a multiplicand in Zd.
            0b000
                if (insn >> 24) & 0xFF == 0b00000100
                    && (insn >> 21) & 1 == 0
                    && matches!((insn >> 13) & 0x7, 0b010 | 0b011 | 0b110 | 0b111) =>
            {
                let op3 = (insn >> 13) & 0x7;
                let sub = op3 & 1 == 1; // MLS/MSB
                let mad = op3 & 0x4 != 0; // MAD/MSB (Zdn is a multiplicand)
                let rm = ((insn >> 16) & 0x1F) as usize;
                let r95 = ((insn >> 5) & 0x1F) as usize;
                let (f1, f2, ar) = if mad { (zd, rm, r95) } else { (r95, rm, zd) };
                let pred = self.sve_p[pg];
                let mask = elem_mask((esize * 8) as u32);
                let fb1 = self.v[f1].to_le_bytes();
                let fb2 = self.v[f2].to_le_bytes();
                let ab = self.v[ar].to_le_bytes();
                let mut dst = self.v[zd].to_le_bytes();
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    if (pred >> off) & 1 == 0 {
                        continue;
                    }
                    let prod =
                        read_elem(&fb1, off, esize).wrapping_mul(read_elem(&fb2, off, esize));
                    let a = read_elem(&ab, off, esize);
                    let r = if sub {
                        a.wrapping_sub(prod)
                    } else {
                        a.wrapping_add(prod)
                    } & mask;
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // Integer predicated binary operations
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x10) == 0 => {
                self.exec_sve_int_pred(insn, zd, zn, zm, pg, esize)
            }

            // Unpredicated arithmetic
            0b000 if op1 == 0b01 => self.exec_sve_int_unpred(insn, zd, zn, zm, esize),

            // SVE2.1 PSEL (predicate select): 0x25, bit21==1, bits[15:14]==01,
            // bit9==0, bit4==0. Pd = Pn if the Pm element at (Wv+imm) mod
            // elements is active, else Pd is all-false. The element size and imm
            // are tsz-encoded in bits[23:18]; Wv = W(bits[17:16]+12).
            0b001
                if (insn >> 24) & 0xFF == 0b00100101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 14) & 0x3 == 0b01
                    && (insn >> 9) & 1 == 0
                    && (insn >> 4) & 1 == 0 =>
            {
                let b2322 = (insn >> 22) & 0x3;
                let b20 = (insn >> 20) & 1;
                let b19 = (insn >> 19) & 1;
                let b18 = (insn >> 18) & 1;
                let (esz, imm): (usize, u64) = if b18 == 1 {
                    (0, ((b2322 << 2) | ((insn >> 19) & 0x3)) as u64)
                } else if b19 == 1 {
                    (1, ((b2322 << 1) | b20) as u64)
                } else if b20 == 1 {
                    (2, b2322 as u64)
                } else if (insn >> 22) & 1 == 1 {
                    (3, ((insn >> 23) & 1) as u64)
                } else {
                    return Ok(CpuExit::Undefined(insn));
                };
                let esize = 1usize << esz;
                let elements = (16 / esize) as u64;
                let rv = 12 + ((insn >> 16) & 0x3) as u8;
                let wv = self.get_x(rv) & 0xFFFF_FFFF;
                let idx = (wv.wrapping_add(imm)) % elements;
                let pm = ((insn >> 5) & 0xF) as usize;
                let pn = ((insn >> 10) & 0xF) as usize;
                let pd = (insn & 0xF) as usize;
                let active = (self.sve_p[pm] >> (idx as usize * esize)) & 1 == 1;
                self.sve_p[pd] = if active { self.sve_p[pn] } else { 0 };
                Ok(CpuExit::Continue)
            }

            // SVE2.1 PEXT (extract a predicate from a predicate-as-counter):
            // 0x25, bit21==1, bits[20:16]==00000, bits[15:12]==0111, bit4==1.
            // PEXT_1 (bits[11:10]==00, imm=bits[9:8]) writes one predicate;
            // PEXT_2 (bits[11:9]==010, imm=bit8) writes the pair {Pd, Pd+1}. The
            // counter source is PN(8+bits[7:5]). Decodes the counter (qemu
            // decode_counter/CounterToPredicate) into the `part`-th VL-sized chunk.
            0b001
                if (insn >> 24) & 0xFF == 0b00100101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 16) & 0x1F == 0b00000
                    && (insn >> 12) & 0xF == 0b0111
                    && (insn >> 4) & 1 == 1 =>
            {
                let v_esz = ((insn >> 22) & 0x3) as usize;
                let (n, imm) = if (insn >> 10) & 0x3 == 0b00 {
                    (1usize, ((insn >> 8) & 0x3) as usize) // PEXT_1
                } else if (insn >> 9) & 0x7 == 0b010 {
                    (2usize, ((insn >> 8) & 1) as usize) // PEXT_2
                } else {
                    return Ok(CpuExit::Undefined(insn));
                };
                let png = self.sve_p[(8 + ((insn >> 5) & 0x7)) as usize] & 0xFFFF;
                let rd_base = (insn & 0xF) as usize;
                const VL_BYTES: usize = 16; // VL=128 -> 16-byte vector, 16-bit predicate
                const PRED_MASKS: [u32; 5] = [0xFFFF, 0x5555, 0x1111, 0x0101, 0x0001];
                // decode_counter (predicate-as-counter -> element count/invert/stride)
                let (count, lg2_stride, invert) = if png & 0xF != 0 {
                    let p_esz = png.trailing_zeros() as usize;
                    let mut count = (png & ((VL_BYTES as u32) * 8 - 1)) as usize; // pow2ceil(16)<<3 -1 = 127
                    count >>= p_esz + 1;
                    let invert = (png >> 15) & 1 == 1;
                    let mut stride = 0usize;
                    if p_esz != v_esz {
                        if p_esz < v_esz {
                            let shift = v_esz - p_esz;
                            let trunc = count >> shift;
                            count = trunc + (count != (trunc << shift)) as usize;
                        } else {
                            let shift = p_esz - v_esz;
                            count <<= shift;
                            stride = shift;
                        }
                    }
                    (count, stride, invert)
                } else {
                    (0, 0, false)
                };
                let esz_mask = PRED_MASKS[v_esz + lg2_stride];
                let oprbits = VL_BYTES; // 16 predicate bit-positions at VL=128
                for i in 0..n {
                    let rd = (rd_base + i) % 16;
                    let part = imm * n + i;
                    let b_count = ((count << v_esz) as i64) - (VL_BYTES * part) as i64;
                    let pd = if invert {
                        if b_count <= 0 {
                            esz_mask // whilel(all)
                        } else if (b_count as usize) < oprbits {
                            // whileg: last (oprbits - b_count) positions active
                            let inv = b_count as usize;
                            esz_mask & !((1u32 << inv) - 1) & ((1u32 << oprbits) - 1)
                        } else {
                            0
                        }
                    } else if b_count > 0 {
                        // whilel: first min(b_count, oprbits) positions active
                        let c = (b_count as usize).min(oprbits);
                        esz_mask & ((1u32 << c) - 1)
                    } else {
                        0
                    };
                    self.sve_p[rd] = pd;
                }
                Ok(CpuExit::Continue)
            }

            // Predicate operations (WHILE, PTRUE, etc.)
            0b001 => self.exec_sve_pred_ops(insn),

            // DUP/MOV/INDEX
            0b000 if op1 == 0b10 || op1 == 0b11 => self.exec_sve_permute(insn, zd, zn, zm, esize),

            // FP predicated operations
            0b011 => self.exec_sve_fp_pred(insn, zd, zn, zm, pg, esize),

            // SVE2 integer add/subtract long/wide and abs-diff long: 0x45,
            // bit21==0, bits[15:13] selects the group — 000 = add/sub LONG (both
            // operands widened from half-width), 001 = ABS-DIFF long (|a-b|,
            // S=bit12 must be 1), 010 = add/sub WIDE (Zn already full width, Zm
            // widened). T (bit10) picks odd/even half-width source elements;
            // U (bit11) unsigned widening; S (bit12) subtract. size=00 reserved.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && matches!((insn >> 13) & 0x7, 0b000 | 0b001 | 0b010) =>
            {
                let group = (insn >> 13) & 0x7;
                let size = (insn >> 22) & 0x3;
                if size == 0 || (group == 0b001 && (insn >> 12) & 1 == 0) {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let sub = (insn >> 12) & 1 == 1;
                let unsigned = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let elements = 16 / d_esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                let mask = elem_mask((d_esize * 8) as u32);
                let widen = |x: u64| -> i128 {
                    if unsigned {
                        uext_elem(x, s_bits) as i128
                    } else {
                        sext_elem(x, s_bits)
                    }
                };
                for d in 0..elements {
                    let s_off = (2 * d + top as usize) * s_esize;
                    let vm = widen(read_elem(&b, s_off, s_esize));
                    let r: i128 = match group {
                        0b000 => {
                            let vn = widen(read_elem(&a, s_off, s_esize));
                            if sub { vn - vm } else { vn + vm }
                        }
                        0b001 => (widen(read_elem(&a, s_off, s_esize)) - vm).abs(),
                        _ => {
                            let vn = read_elem(&a, d * d_esize, d_esize) as i128;
                            if sub { vn - vm } else { vn + vm }
                        }
                    };
                    write_elem(&mut dst, d * d_esize, d_esize, (r as u64) & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 integer add/subtract interleaved long (SADDLBT/SSUBLBT/
            // SSUBLTB): 0x45, bit21==0, bits[15:12]==1000. Signed widening where
            // the two narrow source halves come from DIFFERENT positions:
            // bits[11:10]==00 SADDLBT (Zn bottom + Zm top), 10 SSUBLBT (Zn bottom
            // - Zm top), 11 SSUBLTB (Zn top - Zm bottom); 01 unallocated. size=00
            // reserved. Mirrors qemu's saddl/ssubl helper with sel={2,2,1}.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b1000 =>
            {
                let size = (insn >> 22) & 0x3;
                let op = (insn >> 10) & 0x3;
                if size == 0 || op == 0b01 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let sub = op != 0b00;
                // (seln, selm): which narrow half of Zn/Zm (0=bottom, 1=top).
                let (seln, selm) = if op == 0b11 {
                    (1usize, 0usize)
                } else {
                    (0usize, 1usize)
                };
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let elements = 16 / d_esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                let mask = elem_mask((d_esize * 8) as u32);
                for d in 0..elements {
                    let n_off = (2 * d + seln) * s_esize;
                    let m_off = (2 * d + selm) * s_esize;
                    let vn = sext_elem(read_elem(&a, n_off, s_esize), s_bits);
                    let vm = sext_elem(read_elem(&b, m_off, s_esize), s_bits);
                    let r = if sub { vn - vm } else { vn + vm };
                    write_elem(&mut dst, d * d_esize, d_esize, (r as u64) & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2.1 SCLAMP/UCLAMP (signed/unsigned clamp): 0x44, bit21==0,
            // bits[15:11]==11000. bit10=U. Zd = min(max(Zd, Zn), Zm) per element
            // (Zd is both the clamped value and the destination).
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b11000 =>
            {
                let unsigned = (insn >> 10) & 1 == 1;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let d = self.v[zd].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    let (dv, nv, mv) = (
                        read_elem(&d, off, esize),
                        read_elem(&n, off, esize),
                        read_elem(&m, off, esize),
                    );
                    let r = if unsigned {
                        uext_elem(dv, bits)
                            .max(uext_elem(nv, bits))
                            .min(uext_elem(mv, bits)) as u64
                    } else {
                        sext_elem(dv, bits)
                            .max(sext_elem(nv, bits))
                            .min(sext_elem(mv, bits)) as u64
                    };
                    write_elem(&mut dst, off, esize, r & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2.1 quadword permutes ZIPQ1/ZIPQ2/UZPQ1/UZPQ2/TBLQ: 0x44,
            // bit21==0, bits[15:13]==111. opc=bits[12:10] (000/001 ZIP, 010/011
            // UZP, 110 TBLQ). These permute within each 128-bit segment; at VL=128
            // (a single segment) they coincide with the non-quadword ZIP/UZP/TBL
            // over the whole register. (TBXQ lives in the 0x05 space, handled
            // separately.)
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 13) & 0x7 == 0b111 =>
            {
                let opc = (insn >> 10) & 0x7;
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let nelt = 16 / esize;
                let half = nelt / 2;
                let mut dst = [0u8; 16];
                match opc {
                    0b000 | 0b001 => {
                        // ZIPQ1 (low half) / ZIPQ2 (high half): interleave Zn,Zm.
                        let base = if opc == 0b001 { half } else { 0 };
                        for i in 0..half {
                            write_elem(
                                &mut dst,
                                (2 * i) * esize,
                                esize,
                                read_elem(&n, (base + i) * esize, esize),
                            );
                            write_elem(
                                &mut dst,
                                (2 * i + 1) * esize,
                                esize,
                                read_elem(&m, (base + i) * esize, esize),
                            );
                        }
                    }
                    0b010 | 0b011 => {
                        // UZPQ1 (even) / UZPQ2 (odd): deinterleave Zn:Zm.
                        let start = (opc & 1) as usize; // 0 even, 1 odd
                        for i in 0..nelt {
                            let src_idx = 2 * i + start;
                            let v = if src_idx < nelt {
                                read_elem(&n, src_idx * esize, esize)
                            } else {
                                read_elem(&m, (src_idx - nelt) * esize, esize)
                            };
                            write_elem(&mut dst, i * esize, esize, v);
                        }
                    }
                    0b110 => {
                        // TBLQ: per-segment table lookup of Zn indexed by Zm,
                        // zero-filling out-of-range indices.
                        for i in 0..nelt {
                            let idx = read_elem(&m, i * esize, esize) as usize;
                            let v = if idx < nelt {
                                read_elem(&n, idx * esize, esize)
                            } else {
                                0
                            };
                            write_elem(&mut dst, i * esize, esize, v);
                        }
                    }
                    _ => return Ok(CpuExit::Undefined(insn)),
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 shift right and accumulate: 0x45, bit21==0, bits[15:12]==1110.
            // SSRA/USRA (R=bit11=0) and SRSRA/URSRA (R=1); U=bit10 signedness.
            // Same-size (tsz=tszh:tszl 4 bits); shift = 2*esize - tsz:imm3.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b1110 =>
            {
                let tsize = (((insn >> 22) & 0x3) << 2) | ((insn >> 19) & 0x3);
                if tsize == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let bits = 8 << (31 - tsize.leading_zeros());
                let esize = (bits / 8) as usize;
                let amount = 2 * bits - ((tsize << 3) | ((insn >> 16) & 0x7));
                let round = (insn >> 11) & 1 == 1;
                let unsigned = (insn >> 10) & 1 == 1;
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let acc = self.v[zd].to_le_bytes();
                let n = self.v[zn].to_le_bytes();
                let mut dst = acc;
                for e in 0..elements {
                    let off = e * esize;
                    let x = read_elem(&n, off, esize);
                    let v: i128 = if unsigned {
                        uext_elem(x, bits) as i128
                    } else {
                        sext_elem(x, bits)
                    };
                    let shifted = if round {
                        (v + (1i128 << (amount - 1))) >> amount
                    } else {
                        v >> amount
                    };
                    let cur = read_elem(&acc, off, esize) as i128;
                    write_elem(&mut dst, off, esize, (cur + shifted) as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 shift and insert: 0x45, bit21==0, bits[15:11]==11110. op=bit10
            // selects SLI (shift left, preserve low bits) vs SRI (shift right,
            // preserve high bits). Same-size tsz decode.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b11110 =>
            {
                let tsize = (((insn >> 22) & 0x3) << 2) | ((insn >> 19) & 0x3);
                if tsize == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let bits = 8 << (31 - tsize.leading_zeros());
                let esize = (bits / 8) as usize;
                let tszimm = (tsize << 3) | ((insn >> 16) & 0x7);
                let sli = (insn >> 10) & 1 == 1;
                let amount = if sli {
                    tszimm - bits
                } else {
                    2 * bits - tszimm
                };
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let dn = self.v[zd].to_le_bytes();
                let n = self.v[zn].to_le_bytes();
                let mut dst = dn;
                for e in 0..elements {
                    let off = e * esize;
                    let x = read_elem(&n, off, esize);
                    let d = read_elem(&dn, off, esize);
                    let r = if sli {
                        let keep = (1u64 << amount) - 1; // low `amount` dest bits preserved
                        ((x << amount) & mask) | (d & keep)
                    } else {
                        // SRI shift is 1..=esize; a full-width shift yields 0 (a
                        // u64 `>> bits` would otherwise wrap when bits==64).
                        let shifted = if amount >= bits {
                            0
                        } else {
                            (x >> amount) & mask
                        };
                        let keep = mask & !((1u64 << (bits - amount)).wrapping_sub(1)); // high bits
                        shifted | (d & keep)
                    };
                    write_elem(&mut dst, off, esize, r & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 abs-diff accumulate long: 0x45, bit21==0, bits[15:12]==1100.
            // SABALB/T (U=0) / UABALB/T (U=1): Zda += |widen(Zn) - widen(Zm)| over
            // the half-width even (T=0) / odd (T=1) source elements.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b1100 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let mask = elem_mask((d_esize * 8) as u32);
                let unsigned = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let elements = 16 / d_esize;
                let acc = self.v[zd].to_le_bytes();
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = acc;
                let widen = |x: u64| -> i128 {
                    if unsigned {
                        uext_elem(x, s_bits) as i128
                    } else {
                        sext_elem(x, s_bits)
                    }
                };
                for d in 0..elements {
                    let off = (2 * d + top as usize) * s_esize;
                    let diff = (widen(read_elem(&a, off, s_esize))
                        - widen(read_elem(&b, off, s_esize)))
                    .abs();
                    let cur = read_elem(&acc, d * d_esize, d_esize) as i128;
                    write_elem(&mut dst, d * d_esize, d_esize, (cur + diff) as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 SABA/UABA (abs-diff accumulate, same width): 0x45, bit21==0,
            // bits[15:11]==11111, bit10 selects UABA(1)/SABA(0). The destination
            // accumulates the per-element absolute difference at full width.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b11111 =>
            {
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let unsigned = (insn >> 10) & 1 == 1;
                let elements = 16 / esize;
                let acc = self.v[zd].to_le_bytes();
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = acc;
                for e in 0..elements {
                    let off = e * esize;
                    let (av, bv) = if unsigned {
                        (
                            uext_elem(read_elem(&a, off, esize), bits) as i128,
                            uext_elem(read_elem(&b, off, esize), bits) as i128,
                        )
                    } else {
                        (
                            sext_elem(read_elem(&a, off, esize), bits),
                            sext_elem(read_elem(&b, off, esize), bits),
                        )
                    };
                    let diff = (av - bv).abs() as u64 & mask;
                    let cur = read_elem(&acc, off, esize);
                    write_elem(&mut dst, off, esize, cur.wrapping_add(diff) & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 bit permute (BEXT/BDEP/BGRP): 0x45, bit21==0, bits[15:12]==1011.
            // opc=bits[11:10]: 00=BEXT (gather Zn bits at Zm's set bits to the
            // bottom, like PEXT), 01=BDEP (scatter Zn's low bits to Zm's set bits,
            // like PDEP), 10=BGRP (Zm-selected bits to the bottom, rest on top).
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b1011 =>
            {
                let opc = (insn >> 10) & 0x3;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let off = e * esize;
                    let zn_e = read_elem(&a, off, esize);
                    let zm_e = read_elem(&b, off, esize);
                    let r = match opc {
                        0b00 => {
                            let mut r = 0u64;
                            let mut k = 0;
                            for i in 0..bits {
                                if (zm_e >> i) & 1 == 1 {
                                    r |= ((zn_e >> i) & 1) << k;
                                    k += 1;
                                }
                            }
                            r
                        }
                        0b01 => {
                            let mut r = 0u64;
                            let mut k = 0;
                            for i in 0..bits {
                                if (zm_e >> i) & 1 == 1 {
                                    r |= ((zn_e >> k) & 1) << i;
                                    k += 1;
                                }
                            }
                            r
                        }
                        0b10 => {
                            let (mut low, mut lk, mut high, mut hk) = (0u64, 0u32, 0u64, 0u32);
                            for i in 0..bits {
                                let bit = (zn_e >> i) & 1;
                                if (zm_e >> i) & 1 == 1 {
                                    low |= bit << lk;
                                    lk += 1;
                                } else {
                                    high |= bit << hk;
                                    hk += 1;
                                }
                            }
                            low | (high << lk)
                        }
                        _ => return Ok(CpuExit::Undefined(insn)),
                    };
                    write_elem(&mut dst, off, esize, r & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 shift left long: 010001010 tszh 0 tszl imm3 1010 U T Zn Zd.
            // Widens the half-width source elements (signed U=0 / unsigned U=1,
            // even T=0 / odd T=1) and shifts them left. src esize from highest
            // set bit of tsz, dst 2x; shift = tsz:imm3 - src_bits.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 23) & 1 == 0
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b1010 =>
            {
                let tsize = (((insn >> 22) & 1) << 2) | ((insn >> 19) & 0x3);
                if tsize == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let src_esize = 1usize << (31 - tsize.leading_zeros());
                let dst_esize = src_esize * 2;
                let src_bits = (src_esize * 8) as u32;
                let dst_bits = (dst_esize * 8) as u32;
                let dmask = elem_mask(dst_bits);
                let amount = ((tsize << 3) | ((insn >> 16) & 0x7)) - src_bits;
                let unsigned = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let n_dst = 16 / dst_esize;
                let a = self.v[zn].to_le_bytes();
                let mut dst = [0u8; 16];
                for d in 0..n_dst {
                    let x = read_elem(&a, (2 * d + top as usize) * src_esize, src_esize);
                    let widened: u128 = if unsigned {
                        uext_elem(x, src_bits)
                    } else {
                        sext_elem(x, src_bits) as u128
                    };
                    write_elem(
                        &mut dst,
                        d * dst_esize,
                        dst_esize,
                        (widened << amount) as u64 & dmask,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 integer multiply long: 0x45, bit21==0, bits[15:13]==011.
            // (op=bit12, U=bit11): (1,0)=SMULLB/T, (1,1)=UMULLB/T, (0,0)=
            // SQDMULLB/T (saturating doubling), (0,1)=PMULLB/T (polynomial).
            // Source elements are half-width; T picks odd/even. size=00 reserved;
            // PMULL is only defined for the H form (size=01) in base SVE2.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 13) & 0x7 == 0b011 =>
            {
                let size = (insn >> 22) & 0x3;
                let op = (insn >> 12) & 1;
                let unsigned = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let is_pmull = op == 0 && unsigned;
                // SMULL/UMULL/SQDMULL need a half-width source so size==0 is
                // reserved; PMULL is valid for .q (size==0, 64->128), .h
                // (size==01, 8->16) and .d (size==11, 32->64) but not size==10.
                if (size == 0 && !is_pmull) || (is_pmull && size == 2) {
                    return Ok(CpuExit::Undefined(insn));
                }
                if is_pmull && size == 0 {
                    // PMULLB/T .q <- .d: 64x64 -> 128 carryless. T selects the
                    // odd (high) 64-bit lane of the segment, B the even (low).
                    let lane = top as usize;
                    let xn = (self.v[zn] >> (lane * 64)) as u64;
                    let xm = (self.v[zm] >> (lane * 64)) as u64;
                    self.v[zd] = poly_mul_64(xn, xm);
                    return Ok(CpuExit::Continue);
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let d_bits = (d_esize * 8) as u32;
                let elements = 16 / d_esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                let mask = elem_mask(d_bits);
                for d in 0..elements {
                    let off = (2 * d + top as usize) * s_esize;
                    let xn = read_elem(&a, off, s_esize);
                    let xm = read_elem(&b, off, s_esize);
                    let r: u64 = match (op, unsigned) {
                        (1, false) => (sext_elem(xn, s_bits) * sext_elem(xm, s_bits)) as u64 & mask,
                        (1, true) => (uext_elem(xn, s_bits) * uext_elem(xm, s_bits)) as u64 & mask,
                        (0, false) => {
                            let prod = 2i128 * sext_elem(xn, s_bits) * sext_elem(xm, s_bits);
                            let hi = (1i128 << (d_bits - 1)) - 1;
                            let lo = -(1i128 << (d_bits - 1));
                            prod.clamp(lo, hi) as u64 & mask
                        }
                        _ => poly_mul_wide(xn, xm, s_bits) & mask,
                    };
                    write_elem(&mut dst, d * d_esize, d_esize, r);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 complex integer add (CADD/SQCADD): 0x45, bits[21:17]==00000,
            // bits[15:11]==11011. Treats element pairs as (real, imag); adds Zm
            // rotated by 90 (rot=0) or 270 (rot=1) degrees into Zdn. op=bit16
            // selects the saturating form (SQCADD).
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 17) & 0x1F == 0
                    && (insn >> 11) & 0x1F == 0b11011 =>
            {
                let size = (insn >> 22) & 0x3;
                let esize = 1usize << size;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let sat = (insn >> 16) & 1 == 1;
                let rot = (insn >> 10) & 1;
                let dn = self.v[zd].to_le_bytes(); // Zdn
                let m = self.v[zn].to_le_bytes(); // Zm (bits[9:5])
                let mut dst = dn;
                let pairs = (16 / esize) / 2;
                let hi = (1i128 << (bits - 1)) - 1;
                let lo = -(1i128 << (bits - 1));
                let clamp = |v: i128| if sat { v.clamp(lo, hi) } else { v };
                for p in 0..pairs {
                    let (re, im) = (2 * p * esize, (2 * p + 1) * esize);
                    let dn_re = sext_elem(read_elem(&dn, re, esize), bits);
                    let dn_im = sext_elem(read_elem(&dn, im, esize), bits);
                    let m_re = sext_elem(read_elem(&m, re, esize), bits);
                    let m_im = sext_elem(read_elem(&m, im, esize), bits);
                    let (r_re, r_im) = if rot == 0 {
                        (dn_re - m_im, dn_im + m_re) // rotate Zm by 90 degrees
                    } else {
                        (dn_re + m_im, dn_im - m_re) // rotate Zm by 270 degrees
                    };
                    write_elem(&mut dst, re, esize, clamp(r_re) as u64 & mask);
                    write_elem(&mut dst, im, esize, clamp(r_im) as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 shift right narrow: 010001010 tszh 1 tszl imm3 00 op U R T.
            // (op,U): (0,1)=SHRN/RSHRN, (0,0)=SQSHRUN, (1,0)=SQSHRN, (1,1)=UQSHRN
            // (R=bit11 adds rounding). dst esize from highest set bit of tsz, src
            // 2x; shift amount = 2*dst_bits - (tsz:imm3).
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 23) & 1 == 0
                    && (insn >> 21) & 1 == 1
                    && (insn >> 14) & 0x3 == 0 =>
            {
                let tsize = (((insn >> 22) & 1) << 2) | ((insn >> 19) & 0x3);
                if tsize == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let dst_esize = 1usize << (31 - tsize.leading_zeros());
                let src_esize = dst_esize * 2;
                let dst_bits = (dst_esize * 8) as u32;
                let src_bits = (src_esize * 8) as u32;
                let dmask = elem_mask(dst_bits);
                let tszimm = (tsize << 3) | ((insn >> 16) & 0x7);
                let amount = src_bits - tszimm; // 1..=dst_bits
                let op = (insn >> 13) & 1;
                let u = (insn >> 12) & 1;
                let round = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let n_src = 16 / src_esize;
                let a = self.v[zn].to_le_bytes();
                let mut dst = if top {
                    self.v[zd].to_le_bytes()
                } else {
                    [0u8; 16]
                };
                for d in 0..n_src {
                    let x = read_elem(&a, d * src_esize, src_esize);
                    let narrow: u64 = match (op, u) {
                        (0, 1) => {
                            let v = uext_elem(x, src_bits);
                            let r = if round {
                                (v + (1u128 << (amount - 1))) >> amount
                            } else {
                                v >> amount
                            };
                            r as u64 & dmask
                        }
                        (0, 0) => {
                            let v = sext_elem(x, src_bits);
                            let r = if round {
                                (v + (1i128 << (amount - 1))) >> amount
                            } else {
                                v >> amount
                            };
                            r.clamp(0, dmask as i128) as u64
                        }
                        (1, 0) => {
                            let v = sext_elem(x, src_bits);
                            let r = if round {
                                (v + (1i128 << (amount - 1))) >> amount
                            } else {
                                v >> amount
                            };
                            let hi = (1i128 << (dst_bits - 1)) - 1;
                            let lo = -(1i128 << (dst_bits - 1));
                            r.clamp(lo, hi) as u64 & dmask
                        }
                        _ => {
                            let v = uext_elem(x, src_bits);
                            let r = if round {
                                (v + (1u128 << (amount - 1))) >> amount
                            } else {
                                v >> amount
                            };
                            r.min(dmask as u128) as u64
                        }
                    };
                    write_elem(
                        &mut dst,
                        (2 * d + top as usize) * dst_esize,
                        dst_esize,
                        narrow,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2.1 multi-vector saturating extract narrow SQCVTN/UQCVTN/
            // SQCVTUN (.s pair -> .h): 0x45, bits[23:22]==00, bit21==1,
            // bits[20:16]==10001, bits[15:13]==010. op=bits[12:10] (000 SQCVTN
            // signed->signed, 010 UQCVTN unsigned->unsigned, 100 SQCVTUN
            // signed->unsigned). Reads the register pair {Zn, Zn+1} and
            // interleaves: Zd.h[2i]=sat(Zn.s[i]), Zd.h[2i+1]=sat(Zn+1.s[i]).
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 22) & 0x3 == 0b00
                    && (insn >> 21) & 1 == 1
                    && (insn >> 16) & 0x1F == 0b10001
                    && (insn >> 13) & 0x7 == 0b010 =>
            {
                let (signed_in, signed_out) = match (insn >> 10) & 0x7 {
                    0b000 => (true, true),   // SQCVTN
                    0b010 => (false, false), // UQCVTN
                    0b100 => (true, false),  // SQCVTUN
                    _ => return Ok(CpuExit::Undefined(insn)),
                };
                let zn = ((insn >> 5) & 0x1F) as usize;
                let s0 = self.v[zn].to_le_bytes();
                let s1 = self.v[(zn + 1) % 32].to_le_bytes();
                let mut dst = [0u8; 16];
                let narrow = |bytes: &[u8; 16], i: usize| -> u64 {
                    let v = read_elem(bytes, i * 4, 4);
                    let w = if signed_in {
                        sext_elem(v, 32)
                    } else {
                        uext_elem(v, 32) as i128
                    };
                    if signed_out {
                        sat_signed(w, 16)
                    } else {
                        sat_unsigned(w, 16)
                    }
                };
                for i in 0..4 {
                    write_elem(&mut dst, (2 * i) * 2, 2, narrow(&s0, i));
                    write_elem(&mut dst, (2 * i + 1) * 2, 2, narrow(&s1, i));
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 saturating extract narrow: 010001010 tszh 1 tszl 000010 vv T.
            // (bit12,bit11): 00=SQXTN (signed->signed sat), 01=UQXTN (unsigned->
            // unsigned sat), 10=SQXTUN (signed->unsigned sat). The dest element
            // size comes from the highest set bit of tsz=tszh:tszl, source 2x.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 23) & 1 == 0
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x3F == 0b000010 =>
            {
                let tsz = (((insn >> 22) & 1) << 2) | ((insn >> 19) & 0x3);
                if tsz == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let hsb = 31 - tsz.leading_zeros();
                let dst_esize = 1usize << hsb;
                let src_esize = dst_esize * 2;
                let dst_bits = (dst_esize * 8) as u32;
                let src_bits = (src_esize * 8) as u32;
                let dmask = elem_mask(dst_bits);
                let variant = (insn >> 11) & 0x3;
                let top = (insn >> 10) & 1 == 1;
                let n_src = 16 / src_esize;
                let a = self.v[zn].to_le_bytes();
                let mut dst = if top {
                    self.v[zd].to_le_bytes()
                } else {
                    [0u8; 16]
                };
                for d in 0..n_src {
                    let x = read_elem(&a, d * src_esize, src_esize);
                    let narrow: u64 = match variant {
                        0b00 => {
                            let v = sext_elem(x, src_bits);
                            let hi = (1i128 << (dst_bits - 1)) - 1;
                            let lo = -(1i128 << (dst_bits - 1));
                            v.clamp(lo, hi) as u64 & dmask
                        }
                        0b01 => uext_elem(x, src_bits).min(dmask as u128) as u64,
                        _ => sext_elem(x, src_bits).clamp(0, dmask as i128) as u64,
                    };
                    write_elem(
                        &mut dst,
                        (2 * d + top as usize) * dst_esize,
                        dst_esize,
                        narrow,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 add/subtract high narrow: 0x45, bit21==1, bits[15:13]==011.
            // ADDHN/SUBHN (S=bit12) with optional rounding (R=bit11). The result
            // is the high half of the (full-width) sum/difference, written to the
            // even (T=0, bottom, other half zeroed) or odd (T=1, top, other half
            // preserved) narrow elements. size=00 reserved.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x7 == 0b011 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let src_esize = 1usize << size;
                let dst_esize = src_esize / 2;
                let src_mask = elem_mask((src_esize * 8) as u32);
                let dst_bits = (dst_esize * 8) as u32;
                let dst_mask = elem_mask(dst_bits);
                let sub = (insn >> 12) & 1 == 1;
                let round = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let n_src = 16 / src_esize;
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = if top {
                    self.v[zd].to_le_bytes()
                } else {
                    [0u8; 16]
                };
                for d in 0..n_src {
                    let xn = read_elem(&a, d * src_esize, src_esize);
                    let xm = read_elem(&b, d * src_esize, src_esize);
                    let sum = if sub {
                        xn.wrapping_sub(xm)
                    } else {
                        xn.wrapping_add(xm)
                    };
                    let rounded = if round {
                        sum.wrapping_add(1u64 << (dst_bits - 1))
                    } else {
                        sum
                    } & src_mask;
                    let narrow = (rounded >> dst_bits) & dst_mask;
                    write_elem(
                        &mut dst,
                        (2 * d + top as usize) * dst_esize,
                        dst_esize,
                        narrow,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 integer multiply-add long: 0x44, bit21==0, bits[15:13]==010.
            // S?MLALB/T (S=0) and S?MLSLB/T (S=1); U widening sign; T odd/even.
            // Zda (the destination, bits[4:0]) accumulates widen(Zn)*widen(Zm).
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 13) & 0x7 == 0b010 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let mask = elem_mask((d_esize * 8) as u32);
                let sub = (insn >> 12) & 1 == 1;
                let unsigned = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let elements = 16 / d_esize;
                let acc = self.v[zd].to_le_bytes();
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = acc;
                for d in 0..elements {
                    let off = (2 * d + top as usize) * s_esize;
                    let xn = read_elem(&a, off, s_esize);
                    let xm = read_elem(&b, off, s_esize);
                    let prod: i128 = if unsigned {
                        (uext_elem(xn, s_bits) * uext_elem(xm, s_bits)) as i128
                    } else {
                        sext_elem(xn, s_bits) * sext_elem(xm, s_bits)
                    };
                    let cur = read_elem(&acc, d * d_esize, d_esize) as i128;
                    let r = if sub { cur - prod } else { cur + prod };
                    write_elem(&mut dst, d * d_esize, d_esize, (r as u64) & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 saturating doubling multiply-add long: 0x44, bit21==0,
            // bits[15:12]==0110. SQDMLALB/T (S=0) / SQDMLSLB/T (S=1). The doubled
            // signed product is saturated, then the accumulate is saturated.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b0110 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let d_bits = (d_esize * 8) as u32;
                let mask = elem_mask(d_bits);
                let sub = (insn >> 11) & 1 == 1;
                let top = (insn >> 10) & 1 == 1;
                let elements = 16 / d_esize;
                let hi = (1i128 << (d_bits - 1)) - 1;
                let lo = -(1i128 << (d_bits - 1));
                let acc = self.v[zd].to_le_bytes();
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = acc;
                for d in 0..elements {
                    let off = (2 * d + top as usize) * s_esize;
                    let prod = 2i128
                        * sext_elem(read_elem(&a, off, s_esize), s_bits)
                        * sext_elem(read_elem(&b, off, s_esize), s_bits);
                    let sat = prod.clamp(lo, hi);
                    let cur = sext_elem(read_elem(&acc, d * d_esize, d_esize), d_bits);
                    let r = if sub { cur - sat } else { cur + sat };
                    write_elem(
                        &mut dst,
                        d * d_esize,
                        d_esize,
                        (r.clamp(lo, hi) as u64) & mask,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 saturating doubling multiply-add long (interleaved):
            // SQDMLALBT/SQDMLSLBT. 0x44, bit21==0, bits[15:11]==00001, bit10=S
            // (0=add, 1=sub). The two narrow sources come from DIFFERENT halves
            // (Zn bottom * Zm top, sel=2), unlike the B/T forms above; the doubled
            // product saturates, then the accumulate saturates. size=00 reserved.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b00001 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let d_bits = (d_esize * 8) as u32;
                let mask = elem_mask(d_bits);
                let sub = (insn >> 10) & 1 == 1;
                let elements = 16 / d_esize;
                let hi = (1i128 << (d_bits - 1)) - 1;
                let lo = -(1i128 << (d_bits - 1));
                let acc = self.v[zd].to_le_bytes();
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = acc;
                for d in 0..elements {
                    let n_off = (2 * d) * s_esize; // Zn bottom
                    let m_off = (2 * d + 1) * s_esize; // Zm top
                    let prod = 2i128
                        * sext_elem(read_elem(&a, n_off, s_esize), s_bits)
                        * sext_elem(read_elem(&b, m_off, s_esize), s_bits);
                    let sat = prod.clamp(lo, hi);
                    let cur = sext_elem(read_elem(&acc, d * d_esize, d_esize), d_bits);
                    let r = if sub { cur - sat } else { cur + sat };
                    write_elem(
                        &mut dst,
                        d * d_esize,
                        d_esize,
                        (r.clamp(lo, hi) as u64) & mask,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 SADALP/UADALP (add long pairwise, accumulate): 0x44,
            // bits[21:17]==00010, bits[15:13]==101. U=bit16. Each (active)
            // destination element gains the widened sum of a pair of half-width
            // source elements; inactive lanes keep the prior accumulator.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 17) & 0x1F == 0b00010
                    && (insn >> 13) & 0x7 == 0b101 =>
            {
                let size = (insn >> 22) & 0x3;
                if size == 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let d_esize = 1usize << size;
                let s_esize = d_esize / 2;
                let s_bits = (s_esize * 8) as u32;
                let mask = elem_mask((d_esize * 8) as u32);
                let unsigned = (insn >> 16) & 1 == 1;
                let pred = self.sve_p[pg];
                let elements = 16 / d_esize;
                let acc = self.v[zd].to_le_bytes();
                let n = self.v[zn].to_le_bytes();
                let mut dst = acc;
                let widen = |x: u64| -> i128 {
                    if unsigned {
                        uext_elem(x, s_bits) as i128
                    } else {
                        sext_elem(x, s_bits)
                    }
                };
                for d in 0..elements {
                    if (pred >> (d * d_esize)) & 1 == 0 {
                        continue;
                    }
                    let pair = widen(read_elem(&n, 2 * d * s_esize, s_esize))
                        + widen(read_elem(&n, (2 * d + 1) * s_esize, s_esize));
                    let cur = read_elem(&acc, d * d_esize, d_esize) as i128;
                    write_elem(&mut dst, d * d_esize, d_esize, (cur + pair) as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 predicated integer pairwise: 0x44, bits[21:19]==010,
            // bits[15:13]==101. opc=bits[18:17] (00=ADDP, 10=MAXP, 11=MINP),
            // U=bit16. The pairwise results of Zdn and Zm are INTERLEAVED (even
            // output = Zdn pair, odd = Zm pair); merged into Zdn under Pg.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 19) & 0x7 == 0b010
                    && (insn >> 13) & 0x7 == 0b101 =>
            {
                let opc = (insn >> 17) & 0x3;
                let unsigned = (insn >> 16) & 1 == 1;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let h = elements / 2;
                let pred = self.sve_p[pg];
                let dn = self.v[zd].to_le_bytes(); // Zdn
                let m = self.v[zn].to_le_bytes(); // Zm
                let op = |a: u64, b: u64| -> u64 {
                    match opc {
                        0b00 => a.wrapping_add(b) & mask,
                        0b10 if unsigned => a.max(b),
                        0b10 => (sext_elem(a, bits).max(sext_elem(b, bits)) as u64) & mask,
                        _ if unsigned => a.min(b),
                        _ => (sext_elem(a, bits).min(sext_elem(b, bits)) as u64) & mask,
                    }
                };
                let mut res = [0u8; 16];
                for p in 0..h {
                    let dnv = op(
                        read_elem(&dn, 2 * p * esize, esize),
                        read_elem(&dn, (2 * p + 1) * esize, esize),
                    );
                    let mv = op(
                        read_elem(&m, 2 * p * esize, esize),
                        read_elem(&m, (2 * p + 1) * esize, esize),
                    );
                    write_elem(&mut res, 2 * p * esize, esize, dnv);
                    write_elem(&mut res, (2 * p + 1) * esize, esize, mv);
                }
                let mut dst = dn;
                for e in 0..elements {
                    if (pred >> (e * esize)) & 1 == 1 {
                        write_elem(
                            &mut dst,
                            e * esize,
                            esize,
                            read_elem(&res, e * esize, esize),
                        );
                    }
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 SQRDMLAH/SQRDMLSH (saturating rounding doubling multiply-add):
            // 0x44, bit21==0, bits[15:11]==01110. S=bit10 selects subtract. The
            // rounded doubling-high is unsaturated; only the accumulate saturates.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b01110 =>
            {
                let sub = (insn >> 10) & 1 == 1;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let elements = 16 / esize;
                let acc = self.v[zd].to_le_bytes();
                let a = self.v[zn].to_le_bytes();
                let b = self.v[zm].to_le_bytes();
                let mut dst = acc;
                let hi = (1i128 << (bits - 1)) - 1;
                let lo = -(1i128 << (bits - 1));
                for e in 0..elements {
                    let off = e * esize;
                    let prod = sext_elem(read_elem(&a, off, esize), bits)
                        * sext_elem(read_elem(&b, off, esize), bits);
                    // The Zm factor is negated BEFORE the rounding bias is added
                    // (matching qemu), so the rounding of SQRDMLSH is applied to
                    // -prod rather than negating the rounded SQRDMLAH result —
                    // the two differ at exact rounding ties.
                    let p = if sub { -prod } else { prod };
                    let sdrh = (p + (1i128 << (bits - 2))) >> (bits - 1);
                    let cur = sext_elem(read_elem(&acc, off, esize), bits);
                    write_elem(
                        &mut dst,
                        off,
                        esize,
                        (cur + sdrh).clamp(lo, hi) as u64 & mask,
                    );
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 CDOT (complex integer dot product): 0x44, bit21==0,
            // bits[15:12]==0001. rot=bits[11:10]. Each destination element
            // accumulates two complex products of half-width signed elements
            // (.s from int8, .d from int16): real += r*a, then += i*b*(+/-1).
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 12) & 0xF == 0b0001 =>
            {
                let size = (insn >> 22) & 0x3;
                if size < 2 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esize = 1usize << size; // 4 (.s) or 8 (.d)
                let nb = esize / 4; // narrow element bytes (1 or 2)
                let nbits = (nb * 8) as u32;
                let dbits = (esize * 8) as u32;
                let mask = elem_mask(dbits);
                let rot = (insn >> 10) & 0x3;
                let sel_a = (rot & 1) as usize;
                let sel_b = sel_a ^ 1;
                let sub_i: i128 = if rot == 0 || rot == 3 { -1 } else { 1 };
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let a = self.v[zd].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..(16 / esize) {
                    let mut acc = sext_elem(read_elem(&a, e * esize, esize), dbits);
                    for i in 0..2 {
                        let base = e * esize + i * 2 * nb;
                        let e1r = sext_elem(read_elem(&n, base, nb), nbits);
                        let e1i = sext_elem(read_elem(&n, base + nb, nb), nbits);
                        let e2a = sext_elem(read_elem(&m, base + nb * sel_a, nb), nbits);
                        let e2b = sext_elem(read_elem(&m, base + nb * sel_b, nb), nbits);
                        acc += e1r * e2a + e1i * e2b * sub_i;
                    }
                    write_elem(&mut dst, e * esize, esize, acc as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 CDOT by indexed element: 0x44, bit21==1, bits[15:12]==0100.
            // Like CDOT but the Zm complex element is taken from a fixed index
            // within the 128-bit segment and shared across the segment. .s:
            // index=bits[20:19], Zm=bits[18:16]; .d: index=bit20, Zm=bits[19:16].
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 12) & 0xF == 0b0100 =>
            {
                let size = (insn >> 22) & 0x3;
                if size < 2 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esize = 1usize << size;
                let nb = esize / 4;
                let nbits = (nb * 8) as u32;
                let dbits = (esize * 8) as u32;
                let mask = elem_mask(dbits);
                let rot = (insn >> 10) & 0x3;
                let sel_a = (rot & 1) as usize;
                let sel_b = sel_a ^ 1;
                let sub_i: i128 = if rot == 0 || rot == 3 { -1 } else { 1 };
                let (index, zmr) = if size == 2 {
                    (((insn >> 19) & 0x3) as usize, ((insn >> 16) & 0x7) as usize)
                } else {
                    (((insn >> 20) & 1) as usize, ((insn >> 16) & 0xF) as usize)
                };
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zmr].to_le_bytes();
                let a = self.v[zd].to_le_bytes();
                let m_base = index * esize;
                let mut dst = [0u8; 16];
                for e in 0..(16 / esize) {
                    let mut acc = sext_elem(read_elem(&a, e * esize, esize), dbits);
                    let nbase = e * esize;
                    for i in 0..2 {
                        let e1r = sext_elem(read_elem(&n, nbase + i * 2 * nb, nb), nbits);
                        let e1i = sext_elem(read_elem(&n, nbase + i * 2 * nb + nb, nb), nbits);
                        let e2a =
                            sext_elem(read_elem(&m, m_base + i * 2 * nb + nb * sel_a, nb), nbits);
                        let e2b =
                            sext_elem(read_elem(&m, m_base + i * 2 * nb + nb * sel_b, nb), nbits);
                        acc += e1r * e2a + e1i * e2b * sub_i;
                    }
                    write_elem(&mut dst, e * esize, esize, acc as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE integer dot product (vector): 0x44, bit21==0, with bit23==1
            // and bits[15:11]==00000 (SDOT/UDOT, u=bit10) or bits[23:22]==10 and
            // bits[15:10]==011110 (USDOT).
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (((insn >> 23) & 1 == 1 && (insn >> 11) & 0x1F == 0b00000)
                        || ((insn >> 22) & 0x3 == 0b10 && (insn >> 10) & 0x3F == 0b011110)) =>
            {
                self.exec_sve_dot(insn)
            }

            // SVE integer dot product (indexed): 0x44, bit21==1,
            // bits[15:10] in {SDOT, UDOT, USDOT, SUDOT}.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 1
                    && matches!(
                        (insn >> 10) & 0x3F,
                        0b000000 | 0b000001 | 0b000110 | 0b000111
                    ) =>
            {
                self.exec_sve_dot(insn)
            }

            // SVE2.1 two-way integer dot product SDOT/UDOT (.s <- .h): 0x44,
            // bit21==0, bits[15:11]==11001 (bit10=U). Each .s lane accumulates two
            // 16-bit products. bits[23:22]==00 is the vector form; ==10 is the
            // indexed form (index=bits[20:19], Zm=bits[18:16]) broadcasting the
            // index-th .h pair within the 128-bit segment.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b11001 =>
            {
                let signed = (insn >> 10) & 1 == 0;
                let indexed = (insn >> 22) & 0x3 == 0b10;
                let zd = (insn & 0x1F) as usize;
                let zn = ((insn >> 5) & 0x1F) as usize;
                let (zm, index) = if indexed {
                    (((insn >> 16) & 0x7) as usize, ((insn >> 19) & 0x3) as usize)
                } else {
                    (((insn >> 16) & 0x1F) as usize, 0)
                };
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let a = self.v[zd].to_le_bytes();
                let ext = |b: &[u8; 16], off: usize| -> i64 {
                    if signed {
                        sext_elem(read_elem(b, off, 2), 16) as i64
                    } else {
                        uext_elem(read_elem(b, off, 2), 16) as i64
                    }
                };
                let mut dst = [0u8; 16];
                for i in 0..4 {
                    let mut acc = read_elem(&a, i * 4, 4) as u32 as i64;
                    for k in 0..2 {
                        let n_off = i * 4 + k * 2;
                        let m_off = if indexed { (index * 2 + k) * 2 } else { n_off };
                        acc = acc.wrapping_add(ext(&n, n_off) * ext(&m, m_off));
                    }
                    write_elem(&mut dst, i * 4, 4, acc as u32 as u64);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 predicated integer ALU (saturating/rounding shifts, halving
            // add/sub, saturating add/sub, SQABS/SQNEG): 0x44, bit21==0,
            // bits[15:13]==100, or bits[15:13]==101 with bits[21:19]==001. The
            // pairwise group (bits[15:13]==101, bits[21:19]==010) is handled by
            // its own arm and excluded here.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && ((insn >> 13) & 0x7 == 0b100
                        || ((insn >> 13) & 0x7 == 0b101 && (insn >> 19) & 0x7 == 0b001)
                        || ((insn >> 13) & 0x7 == 0b101
                            && (insn >> 19) & 0x7 == 0b000
                            && matches!((insn >> 16) & 0x7, 0b000 | 0b001))) =>
            {
                self.exec_sve2_pred_alu(insn)
            }

            // SVE2 complex integer multiply-add (CMLA/SQRDCMLAH): 0x44, bit21==0,
            // bits[15:13]==001. op=bit12 picks the saturating-rounding-doubling
            // SQRDCMLAH; rot=bits[11:10] is the 0/90/180/270 rotation. Each
            // complex pair accumulates one selected-component product.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 0
                    && (insn >> 13) & 0x7 == 0b001 =>
            {
                let size = (insn >> 22) & 0x3;
                let esize = 1usize << size;
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let sat = (insn >> 12) & 1 == 1;
                let rot = (insn >> 10) & 0x3;
                let acc = self.v[zd].to_le_bytes();
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let mut dst = acc;
                let hi = (1i128 << (bits - 1)) - 1;
                let lo = -(1i128 << (bits - 1));
                let pairs = (16 / esize) / 2;
                for p in 0..pairs {
                    let (re, im) = (2 * p * esize, (2 * p + 1) * esize);
                    let n_re = sext_elem(read_elem(&n, re, esize), bits);
                    let n_im = sext_elem(read_elem(&n, im, esize), bits);
                    let m_re = sext_elem(read_elem(&m, re, esize), bits);
                    let m_im = sext_elem(read_elem(&m, im, esize), bits);
                    let acc_re = sext_elem(read_elem(&acc, re, esize), bits);
                    let acc_im = sext_elem(read_elem(&acc, im, esize), bits);
                    let zn_sel = if rot == 0 || rot == 2 { n_re } else { n_im };
                    // The signed Zm factor for the real/imag accumulation.
                    let (mfr, mfi): (i128, i128) = match rot {
                        0 => (m_re, m_im),
                        1 => (-m_im, m_re),
                        2 => (-m_re, -m_im),
                        _ => (m_im, -m_re),
                    };
                    let (r_re, r_im) = if sat {
                        // SignedDoublingRoundingHigh: (2*prod + 2^(bits-1)) >> bits,
                        // rewritten as (prod + 2^(bits-2)) >> (bits-1) to avoid the
                        // doubled product overflowing i128 at the 64-bit size. As in
                        // NEON SQRDMLAH the rounded high part is NOT saturated; only
                        // the final accumulate is.
                        let sdrh = |prod: i128| (prod + (1i128 << (bits - 2))) >> (bits - 1);
                        (
                            (acc_re + sdrh(zn_sel * mfr)).clamp(lo, hi),
                            (acc_im + sdrh(zn_sel * mfi)).clamp(lo, hi),
                        )
                    } else {
                        (acc_re + zn_sel * mfr, acc_im + zn_sel * mfi)
                    };
                    write_elem(&mut dst, re, esize, r_re as u64 & mask);
                    write_elem(&mut dst, im, esize, r_im as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 integer multiply / multiply-add (indexed): 0x44, bit21==1.
            // The second factor is a single element Zm[index] broadcast to every
            // lane; the (index, Zm) packing depends on the element size.
            // bits[15:10] selects MUL/SQDMULH/SQRDMULH (1111xx), MLA/MLS
            // (00001x) or SQRDMLAH/SQRDMLSH (00010x). Other op fields (SMLALB,
            // CMLA, ...) fall through to the unimplemented arm.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 1
                    && matches!(
                        (insn >> 10) & 0x3F,
                        0b111110 | 0b111100 | 0b111101 | 0b000010 | 0b000011 | 0b000100 | 0b000101
                    ) =>
            {
                self.exec_sve2_mul_indexed(insn, zn, zd)
            }

            // SVE2 widening multiply-add long by indexed element: 0x44, bit21==1.
            // bits[15:12] selects S/U MULL/MLAL/MLSL and SQDMULL/SQDMLAL/SQDMLSL;
            // the narrow source is half the destination width and bit10 (T) picks
            // the odd/even narrow lane. Distinct op fields from the same-width
            // indexed group above (1111xx / 0000xx), so no overlap.
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 1
                    && matches!(
                        (insn >> 12) & 0xF,
                        0b1000
                            | 0b1001
                            | 0b1010
                            | 0b1011
                            | 0b1100
                            | 0b1101
                            | 0b1110
                            | 0b0010
                            | 0b0011
                    ) =>
            {
                self.exec_sve2_mull_indexed(insn, zn, zd)
            }

            // SVE2 CMLA / SQRDCMLAH by indexed element: 0x44, bit21==1,
            // bits[15:13]==011. bit12 picks SQRDCMLAH (saturating-rounding-
            // doubling) over plain CMLA. rot=bits[11:10]; the indexed Zm complex
            // pair (at 2*index) is broadcast. .h: index=bits[20:19],
            // Zm=bits[18:16]; .s: index=bit20, Zm=bits[19:16]. For SQRDCMLAH the
            // doubled-rounded high product is accumulated then saturated, exactly
            // as the non-indexed SQRDCMLAH (qemu sve2_sqrdcmlah_idx).
            0b010
                if (insn >> 24) & 0xFF == 0b01000100
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x7 == 0b011 =>
            {
                let size = (insn >> 22) & 0x3;
                if size < 2 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let sat = (insn >> 12) & 1 == 1;
                let esize = 1usize << (size - 1); // .h=2, .s=4
                let bits = (esize * 8) as u32;
                let mask = elem_mask(bits);
                let hi = (1i128 << (bits - 1)) - 1;
                let lo = -(1i128 << (bits - 1));
                let sdrh = |prod: i128| (prod + (1i128 << (bits - 2))) >> (bits - 1);
                let rot = (insn >> 10) & 0x3;
                let sel_a = (rot & 1) as usize;
                let sel_b = sel_a ^ 1;
                let sub_r: i128 = if rot == 1 || rot == 2 { -1 } else { 1 };
                let sub_i: i128 = if rot >= 2 { -1 } else { 1 };
                let (index, zm) = if size == 2 {
                    (((insn >> 19) & 0x3) as usize, ((insn >> 16) & 0x7) as usize)
                } else {
                    (((insn >> 20) & 1) as usize, ((insn >> 16) & 0xF) as usize)
                };
                let idx = index * 2;
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let a = self.v[zd].to_le_bytes();
                let e2a = sext_elem(read_elem(&m, (idx + sel_a) * esize, esize), bits);
                let e2b = sext_elem(read_elem(&m, (idx + sel_b) * esize, esize), bits);
                let mut dst = [0u8; 16];
                for p in 0..((16 / esize) / 2) {
                    let (re, im) = (2 * p, 2 * p + 1);
                    let e1 = sext_elem(read_elem(&n, (re + sel_a) * esize, esize), bits);
                    let ar = sext_elem(read_elem(&a, re * esize, esize), bits);
                    let ai = sext_elem(read_elem(&a, im * esize, esize), bits);
                    let (r_re, r_im) = if sat {
                        (
                            (ar + sdrh(e1 * e2a * sub_r)).clamp(lo, hi),
                            (ai + sdrh(e1 * e2b * sub_i)).clamp(lo, hi),
                        )
                    } else {
                        (ar + e1 * e2a * sub_r, ai + e1 * e2b * sub_i)
                    };
                    write_elem(&mut dst, re * esize, esize, r_re as u64 & mask);
                    write_elem(&mut dst, im * esize, esize, r_im as u64 & mask);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 MATCH / NMATCH (character match -> predicate): 0x45, bit21==1,
            // bits[15:13]==100. For each Pg-active Zn element the result bit is
            // set if that element value equals any Zm element in the same
            // 128-bit segment (MATCH) or none of them (NMATCH, bit4==1). The
            // result is zeroing and sets NZCV via PredTest(Pg). size b/h only.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x7 == 0b100 =>
            {
                let size = (insn >> 22) & 0x3;
                if size > 1 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esize = 1usize << size;
                let elements = 16 / esize;
                let nmatch = (insn >> 4) & 1 == 1;
                let pg = ((insn >> 10) & 0x7) as usize;
                let pd = (insn & 0xF) as usize;
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let gov = self.sve_p[pg];
                let mut result = 0u32;
                for e in 0..elements {
                    let off = e * esize;
                    if (gov >> off) & 1 == 0 {
                        continue; // zeroing predication
                    }
                    let ne = read_elem(&n, off, esize);
                    let matched = (0..elements).any(|j| read_elem(&m, j * esize, esize) == ne);
                    if matched ^ nmatch {
                        result |= 1 << off;
                    }
                }
                self.sve_p[pd] = result;
                let (nf, zf, cf, vf) = pred_test(gov, result, elements, esize);
                self.set_n(nf);
                self.set_z(zf);
                self.set_c(cf);
                self.set_v(vf);
                Ok(CpuExit::Continue)
            }

            // SVE2 HISTSEG (histogram segment): 0x45, bit21==1,
            // bits[15:10]==101000, size==b. Each result byte is the number of Zm
            // bytes (in the 128-bit segment) equal to the corresponding Zn byte.
            // Unpredicated.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 10) & 0x3F == 0b101000 =>
            {
                if (insn >> 22) & 0x3 != 0 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..16 {
                    dst[e] = m.iter().filter(|&&b| b == n[e]).count() as u8;
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 HISTCNT (histogram count): 0x45, bit21==1, bits[15:13]==110,
            // size s/d. For each Pg-active element i, the result is the number of
            // active elements j<=i whose Zm value equals Zn[i]; inactive lanes
            // are zeroed.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x7 == 0b110 =>
            {
                let size = (insn >> 22) & 0x3;
                if size < 2 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esize = 1usize << size;
                let elements = 16 / esize;
                let pg = ((insn >> 10) & 0x7) as usize;
                let gov = self.sve_p[pg];
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];
                for i in 0..elements {
                    let off_i = i * esize;
                    if (gov >> off_i) & 1 == 0 {
                        continue; // zeroing
                    }
                    let nn = read_elem(&n, off_i, esize);
                    let mut count = 0u64;
                    for j in 0..=i {
                        let off_j = j * esize;
                        if (gov >> off_j) & 1 == 1 && read_elem(&m, off_j, esize) == nn {
                            count += 1;
                        }
                    }
                    write_elem(&mut dst, off_i, esize, count);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE2 crypto (AES / SM4 / SHA3-RAX1): 0x45, bit21==1,
            // bits[15:13]==111. At VL=128 each op acts on the single 128-bit
            // segment, identical to its NEON counterpart.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 1
                    && (insn >> 13) & 0x7 == 0b111 =>
            {
                self.exec_sve2_crypto(insn)
            }

            // SVE2 ADCLB/ADCLT/SBCLB/SBCLT (long add/subtract with carry): 0x45,
            // bit21==0, bits[15:11]==11010. The carry-in is bit `esize` of each
            // Zm element; bit23 inverts the Zn operand (SBCL = add of the one's
            // complement); bit22 selects .d (1) / .s (0); bit10 (T) the odd/even
            // Zn half. Zda holds the low half; the full sum (with carry-out) is
            // written across the doubled container.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b11010 =>
            {
                let d_form = (insn >> 22) & 1 == 1;
                let top = ((insn >> 10) & 1) as u32;
                let inv = (insn >> 23) & 1 == 1;
                if d_form {
                    let e1 = self.v[zd] as u64;
                    let mut e2 = (self.v[zn] >> (top * 64)) as u64;
                    if inv {
                        e2 = !e2;
                    }
                    let c = ((self.v[zm] >> 64) & 1) as u64;
                    self.v[zd] = (e1 as u128) + (e2 as u128) + (c as u128);
                } else {
                    let mut dst = 0u128;
                    for i in 0..2 {
                        let e1 = (self.v[zd] >> (i * 64)) as u32;
                        let mut e2 = ((self.v[zn] >> (i * 64)) >> (top * 32)) as u32;
                        if inv {
                            e2 = !e2;
                        }
                        let c = ((self.v[zm] >> (i * 64 + 32)) & 1) as u32;
                        let sum = e1 as u64 + e2 as u64 + c as u64; // 33-bit, holds carry-out
                        dst |= (sum as u128) << (i * 64);
                    }
                    self.v[zd] = dst;
                }
                Ok(CpuExit::Continue)
            }

            // SVE2 EORBT/EORTB (interleaving exclusive OR): 0x45, bit21==0,
            // bits[15:11]==10010, bit10 selects EORTB(1)/EORBT(0). EORBT writes
            // the even result lanes as Zn_even ^ Zm_odd (odd lanes keep the prior
            // Zd); EORTB writes the odd lanes as Zn_odd ^ Zm_even.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 11) & 0x1F == 0b10010 =>
            {
                let esize = 1usize << ((insn >> 22) & 0x3);
                let tb = (insn >> 10) & 1 == 1; // EORTB
                let (sel1, sel2) = if tb {
                    (1usize, 0usize)
                } else {
                    (0usize, 1usize)
                };
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let mut dst = self.v[zd].to_le_bytes(); // keep prior Zd in unwritten lanes
                for p in 0..((16 / esize) / 2) {
                    let base = 2 * p * esize;
                    let nn = read_elem(&n, base + sel1 * esize, esize);
                    let mm = read_elem(&m, base + sel2 * esize, esize);
                    write_elem(&mut dst, base + sel1 * esize, esize, nn ^ mm);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // SVE I8MM integer matrix multiply-accumulate (SMMLA/UMMLA/USMMLA):
            // 0x45, bit21==0, bits[15:10]==100110. bit23 = Zn unsigned, bit22 =
            // Zm unsigned (the signed-by-unsigned pair is unallocated). Computes
            // a 2x2 int32 tile: Zda += Zn(2x8 int8) * Zm(2x8 int8)^T, each entry
            // an 8-element dot product accumulated mod 2^32.
            0b010
                if (insn >> 24) & 0xFF == 0b01000101
                    && (insn >> 21) & 1 == 0
                    && (insn >> 10) & 0x3F == 0b100110 =>
            {
                let n_uns = (insn >> 23) & 1 == 1;
                let m_uns = (insn >> 22) & 1 == 1;
                if !n_uns && m_uns {
                    return Ok(CpuExit::Undefined(insn)); // signed-by-unsigned: unallocated
                }
                let n = self.v[zn].to_le_bytes();
                let m = self.v[zm].to_le_bytes();
                let acc = self.v[zd].to_le_bytes();
                let dot = |nrow: usize, mrow: usize| -> u32 {
                    let mut s = 0i64;
                    for k in 0..8 {
                        let nv = n[nrow * 8 + k];
                        let mv = m[mrow * 8 + k];
                        let np = if n_uns { nv as i64 } else { nv as i8 as i64 };
                        let mp = if m_uns { mv as i64 } else { mv as i8 as i64 };
                        s = s.wrapping_add(np * mp);
                    }
                    s as u32
                };
                let mut dst = [0u8; 16];
                for (idx, &(nr, mr)) in [(0, 0), (0, 1), (1, 0), (1, 1)].iter().enumerate() {
                    let a = u32::from_le_bytes(acc[idx * 4..idx * 4 + 4].try_into().unwrap());
                    let r = a.wrapping_add(dot(nr, mr));
                    dst[idx * 4..idx * 4 + 4].copy_from_slice(&r.to_le_bytes());
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // Load/Store
            0b100 | 0b101 | 0b110 | 0b111 => self.exec_sve_ldst(insn),

            _ => Err(ArmError::Unimplemented(format!(
                "SVE op0={:03b} op1={:02b}",
                op0, op1
            ))),
        }
    }

    /// Execute SVE integer dot product (SDOT/UDOT/USDOT/SUDOT), vector and
    /// indexed. Each destination element (S from 8-bit sources, D from 16-bit)
    /// accumulates a 4-element dot product; the indexed form broadcasts the
    /// index-th 4-element group of Zm across the segment. Sign treatment is
    /// per-operand; no saturation.
    fn exec_sve_dot(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let indexed = (insn >> 21) & 1 == 1;
        let d_esize = if (insn >> 22) & 1 == 0 { 4usize } else { 8 };
        let s_esize = d_esize / 4;
        let s_bits = (s_esize * 8) as u32;
        let d_bits = (d_esize * 8) as u32;
        let mask = elem_mask(d_bits);
        let (n_signed, m_signed) = if indexed {
            match (insn >> 10) & 0x3F {
                0b000000 => (true, true),   // SDOT
                0b000001 => (false, false), // UDOT
                0b000110 => (false, true),  // USDOT (Zn unsigned, Zm signed)
                0b000111 => (true, false),  // SUDOT (Zn signed, Zm unsigned)
                _ => return Ok(CpuExit::Undefined(insn)),
            }
        } else if (insn >> 10) & 0x3F == 0b011110 {
            (false, true) // USDOT vector
        } else {
            let u = (insn >> 10) & 1 == 1;
            (!u, !u) // SDOT(u=0) / UDOT(u=1)
        };
        let zd = (insn & 0x1F) as usize;
        let zn = ((insn >> 5) & 0x1F) as usize;
        let (zm, index) = if indexed {
            if d_esize == 4 {
                (((insn >> 16) & 0x7) as usize, ((insn >> 19) & 0x3) as usize)
            } else {
                (((insn >> 16) & 0xF) as usize, ((insn >> 20) & 1) as usize)
            }
        } else {
            (((insn >> 16) & 0x1F) as usize, 0)
        };
        let n = self.v[zn].to_le_bytes();
        let m = self.v[zm].to_le_bytes();
        let a = self.v[zd].to_le_bytes();
        let ext = |b: &[u8; 16], off: usize, s: bool| -> i128 {
            if s {
                sext_elem(read_elem(b, off, s_esize), s_bits)
            } else {
                uext_elem(read_elem(b, off, s_esize), s_bits) as i128
            }
        };
        let mut dst = [0u8; 16];
        for i in 0..(16 / d_esize) {
            let mut acc = sext_elem(read_elem(&a, i * d_esize, d_esize), d_bits);
            for k in 0..4 {
                let n_off = i * d_esize + k * s_esize;
                let m_off = if indexed {
                    (index * 4 + k) * s_esize
                } else {
                    n_off
                };
                acc += ext(&n, n_off, n_signed) * ext(&m, m_off, m_signed);
            }
            write_elem(&mut dst, i * d_esize, d_esize, acc as u64 & mask);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute the SVE2 predicated integer ALU group at 0x44 bits[15:14]==10:
    /// saturating/rounding shifts by vector (SRSHL/URSHL/SQSHL/UQSHL/SQRSHL/
    /// UQRSHL and their reversed forms), halving add/sub (SHADD/UHADD/SHSUB/
    /// UHSUB/SRHADD/URHADD/SHSUBR/UHSUBR), saturating add/sub (SQADD/UQADD/
    /// SQSUB/UQSUB/SUQADD/USQADD/SQSUBR/UQSUBR) at bits[15:13]==100, and the
    /// unary SQABS/SQNEG at bits[15:13]==101 bits[21:19]==001. All merge under
    /// Pg. The op is keyed on bits[21:16]; reversed forms swap the operands.
    fn exec_sve2_pred_alu(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let esize = 1usize << ((insn >> 22) & 0x3);
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let pg = ((insn >> 10) & 0x7) as usize;
        let rd = (insn & 0x1F) as usize;
        let rfield = ((insn >> 5) & 0x1F) as usize;
        let pred = self.sve_p[pg];
        let dst_prior = self.v[rd].to_le_bytes();
        let mut dst = dst_prior;
        let elements = 16 / esize;

        if (insn >> 13) & 0x7 == 0b101 {
            // Unary, source = rfield, dest = rd, merging. bits[21:19]==001 ->
            // SQABS/SQNEG; bits[21:19]==000 -> URECPE/URSQRTE (S-only unsigned
            // reciprocal estimates).
            let src = self.v[rfield].to_le_bytes();
            let recip = (insn >> 19) & 0x7 == 0b000;
            if recip && esize != 4 {
                return Ok(CpuExit::Undefined(insn));
            }
            let sel = (insn >> 16) & 1 == 1; // SQNEG / URSQRTE
            for e in 0..elements {
                let off = e * esize;
                if (pred >> off) & 1 == 0 {
                    continue;
                }
                let r = if recip {
                    let a = read_elem(&src, off, 4) as u32;
                    (if sel {
                        unsigned_rsqrt_estimate(a)
                    } else {
                        unsigned_recip_estimate(a)
                    }) as u64
                } else {
                    let n = sext_elem(read_elem(&src, off, esize), bits);
                    if sel {
                        sat_signed(-n, bits)
                    } else {
                        sat_signed(n.abs(), bits)
                    }
                };
                write_elem(&mut dst, off, esize, r);
            }
            self.v[rd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        let opc6 = (insn >> 16) & 0x3F;
        let reversed = matches!(
            opc6,
            0b000110
                | 0b000111
                | 0b001100
                | 0b001101
                | 0b001110
                | 0b001111
                | 0b010110
                | 0b010111
                | 0b011110
                | 0b011111
        );
        let field = self.v[rfield].to_le_bytes();
        let do_shift = |val: u64, sh: i64, signed: bool, round: bool, sat: bool| -> u64 {
            if bits == 64 {
                if signed {
                    sqrshl_d(val as i64, sh, round, sat) as u64
                } else {
                    uqrshl_d(val, sh, round, sat)
                }
            } else if signed {
                sqrshl_bhs(sext_elem(val, bits) as i32, sh as i32, bits, round, sat) as u64 & mask
            } else {
                uqrshl_bhs((val & mask) as u32, sh as i32, bits, round, sat) as u64 & mask
            }
        };
        for e in 0..elements {
            let off = e * esize;
            if (pred >> off) & 1 == 0 {
                continue;
            }
            let rd_v = read_elem(&dst_prior, off, esize);
            let fv = read_elem(&field, off, esize);
            let (a, b) = if reversed { (fv, rd_v) } else { (rd_v, fv) };
            let (sa, sb) = (sext_elem(a, bits), sext_elem(b, bits));
            let (ua, ub) = (uext_elem(a, bits) as i128, uext_elem(b, bits) as i128);
            let r: u64 = match opc6 {
                0b000010 | 0b000110 => do_shift(a, sb as i64, true, true, false), // SRSHL(R)
                0b000011 | 0b000111 => do_shift(a, sb as i64, false, true, false), // URSHL(R)
                0b001000 | 0b001100 => do_shift(a, sb as i64, true, false, true), // SQSHL(R)
                0b001001 | 0b001101 => do_shift(a, sb as i64, false, false, true), // UQSHL(R)
                0b001010 | 0b001110 => do_shift(a, sb as i64, true, true, true),  // SQRSHL(R)
                0b001011 | 0b001111 => do_shift(a, sb as i64, false, true, true), // UQRSHL(R)
                0b010000 => ((sa + sb) >> 1) as u64 & mask,                       // SHADD
                0b010001 => ((ua + ub) >> 1) as u64 & mask,                       // UHADD
                0b010010 | 0b010110 => ((sa - sb) >> 1) as u64 & mask,            // SHSUB(R)
                0b010011 | 0b010111 => ((ua - ub) >> 1) as u64 & mask,            // UHSUB(R)
                0b010100 => ((sa + sb + 1) >> 1) as u64 & mask,                   // SRHADD
                0b010101 => ((ua + ub + 1) >> 1) as u64 & mask,                   // URHADD
                0b011000 => sat_signed(sa + sb, bits),                            // SQADD
                0b011001 => sat_unsigned(ua + ub, bits),                          // UQADD
                0b011010 | 0b011110 => sat_signed(sa - sb, bits),                 // SQSUB(R)
                0b011011 | 0b011111 => sat_unsigned(ua - ub, bits),               // UQSUB(R)
                0b011100 => sat_signed(sa + ub, bits),                            // SUQADD
                0b011101 => sat_unsigned(ua + sb, bits),                          // USQADD
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[rd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE2 widening multiply-add long by an indexed element. The
    /// narrow source elements (half the destination width) are sign- or
    /// zero-extended; `Zm[index]` is the shared broadcast factor and bit10 (T)
    /// selects the odd/even narrow lane of Zn. SQDMULL doubles-and-saturates;
    /// the SQDMLAL/SQDMLSL accumulate saturates a second time.
    fn exec_sve2_mull_indexed(
        &mut self,
        insn: u32,
        zn: usize,
        zd: usize,
    ) -> Result<CpuExit, ArmError> {
        let size = (insn >> 22) & 0x3; // 2=.s (h src), 3=.d (s src)
        if size < 2 {
            return Ok(CpuExit::Undefined(insn));
        }
        let d_esize = 1usize << size;
        let s_esize = d_esize / 2;
        let s_bits = (s_esize * 8) as u32;
        let d_bits = (d_esize * 8) as u32;
        let top = (insn >> 10) & 1 == 1;
        let op = (insn >> 12) & 0xF;
        // Index and Zm packing differ per size: .s uses a 3-bit index
        // (bit20:bit19:bit11) with Zm in z0-z7; .d a 2-bit index (bit20:bit11)
        // with Zm in z0-z15.
        let (index, zm) = if size == 2 {
            let idx = (((insn >> 20) & 1) << 2) | (((insn >> 19) & 1) << 1) | ((insn >> 11) & 1);
            (idx as usize, ((insn >> 16) & 0x7) as usize)
        } else {
            let idx = (((insn >> 20) & 1) << 1) | ((insn >> 11) & 1);
            (idx as usize, ((insn >> 16) & 0xF) as usize)
        };
        let n = self.v[zn].to_le_bytes();
        let m = self.v[zm].to_le_bytes();
        let acc = self.v[zd].to_le_bytes();
        let mut dst = [0u8; 16];
        let mask = elem_mask(d_bits);
        let hi = (1i128 << (d_bits - 1)) - 1;
        let lo = -(1i128 << (d_bits - 1));
        let m_raw = read_elem(&m, index * s_esize, s_esize);
        let elements = 16 / d_esize;
        for e in 0..elements {
            let n_raw = read_elem(&n, (2 * e + top as usize) * s_esize, s_esize);
            let aa = read_elem(&acc, e * d_esize, d_esize);
            let aa_s = sext_elem(aa, d_bits);
            let nm_s = sext_elem(n_raw, s_bits) * sext_elem(m_raw, s_bits);
            let nm_u = (uext_elem(n_raw, s_bits) * uext_elem(m_raw, s_bits)) as i128;
            let sqdmull = (2 * nm_s).clamp(lo, hi); // saturating doubling product
            let r: u64 = match op {
                0b1100 => nm_s as u64 & mask,                           // SMULLB/T
                0b1101 => nm_u as u64 & mask,                           // UMULLB/T
                0b1000 => (aa_s + nm_s) as u64 & mask,                  // SMLALB/T
                0b1001 => (aa_s + nm_u) as u64 & mask,                  // UMLALB/T
                0b1010 => (aa_s - nm_s) as u64 & mask,                  // SMLSLB/T
                0b1011 => (aa_s - nm_u) as u64 & mask,                  // UMLSLB/T
                0b1110 => sqdmull as u64 & mask,                        // SQDMULLB/T
                0b0010 => (aa_s + sqdmull).clamp(lo, hi) as u64 & mask, // SQDMLALB/T
                0b0011 => (aa_s - sqdmull).clamp(lo, hi) as u64 & mask, // SQDMLSLB/T
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, e * d_esize, d_esize, r);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute the SVE2 crypto group (AES round/mix, SM4, SHA3 RAX1). At
    /// VL=128 every operation works on the single 128-bit segment, so it reuses
    /// the NEON primitives directly. AES family: bits[15:11]==11100, sub-decoded
    /// by bits[23:16] and bit10; SM4EKEY/RAX1: bits[15:11]==11110, bit10.
    fn exec_sve2_crypto(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let zd = (insn & 0x1F) as usize;
        let n = ((insn >> 5) & 0x1F) as usize; // bits[9:5] (Zm for AES/SM4E, Zn for *KEY/RAX1)
        let m = ((insn >> 16) & 0x1F) as usize; // bits[20:16] (Zm for *KEY/RAX1)
        let inv = (insn >> 10) & 1 == 1;
        match (insn >> 11) & 0x1F {
            0b11100 => {
                match (insn >> 16) & 0xFF {
                    // AESMC / AESIMC: Zd = (Inv)MixColumns(Zd). bits[9:5] must be 0.
                    0x20 if n == 0 => self.v[zd] = aes_mix_columns(self.v[zd], inv),
                    // AESE / AESD: Zd = (Inv)SubBytes((Inv)ShiftRows(Zd ^ Zm)).
                    0x22 => {
                        let st = self.v[zd] ^ self.v[n];
                        self.v[zd] = aes_sub_bytes(aes_shift_rows(st, inv), inv);
                    }
                    // SM4E: Zd = SM4E(Zd, Zm).
                    0x23 if !inv => self.v[zd] = sm4_rounds(self.v[zd], self.v[n], true),
                    _ => return Ok(CpuExit::Undefined(insn)),
                }
                Ok(CpuExit::Continue)
            }
            0b11110 => {
                if !inv {
                    // SM4EKEY: Zd = SM4EKEY(Zn, Zm).
                    self.v[zd] = sm4_rounds(self.v[n], self.v[m], false);
                } else {
                    // RAX1: per 64-bit element, Zd = Zn ^ ROL(Zm, 1).
                    let (zn, zm) = (self.v[n], self.v[m]);
                    let lo = (zn as u64) ^ (zm as u64).rotate_left(1);
                    let hi = ((zn >> 64) as u64) ^ ((zm >> 64) as u64).rotate_left(1);
                    self.v[zd] = (lo as u128) | ((hi as u128) << 64);
                }
                Ok(CpuExit::Continue)
            }
            _ => Ok(CpuExit::Undefined(insn)),
        }
    }

    /// Execute SVE2 integer multiply / multiply-add by an indexed element.
    /// `Zm[index]` (selected within the single 128-bit segment for VL=128) is
    /// the shared second factor for every destination lane. MUL/MLA/MLS take
    /// the truncated low half of the product; SQDMULH/SQRDMULH take the
    /// saturating (optionally rounded) doubled high half.
    fn exec_sve2_mul_indexed(
        &mut self,
        insn: u32,
        zn: usize,
        zd: usize,
    ) -> Result<CpuExit, ArmError> {
        // Element size and (index, Zm) packing differ per size: H uses a 3-bit
        // index (bit22:bit20:bit19) with Zm in z0-z7; S a 2-bit index
        // (bit20:bit19) with Zm in z0-z7; D a 1-bit index (bit20), Zm in z0-z15.
        let (esize, index, zm): (usize, usize, usize) = if (insn >> 23) & 1 == 0 {
            let idx = (((insn >> 22) & 1) << 2) | (((insn >> 20) & 1) << 1) | ((insn >> 19) & 1);
            (2, idx as usize, ((insn >> 16) & 0x7) as usize)
        } else if (insn >> 22) & 1 == 0 {
            let idx = (((insn >> 20) & 1) << 1) | ((insn >> 19) & 1);
            (4, idx as usize, ((insn >> 16) & 0x7) as usize)
        } else {
            (
                8,
                ((insn >> 20) & 1) as usize,
                ((insn >> 16) & 0xF) as usize,
            )
        };
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let op = (insn >> 10) & 0x3F;
        let n = self.v[zn].to_le_bytes();
        let m = self.v[zm].to_le_bytes();
        let acc = self.v[zd].to_le_bytes();
        let mut dst = acc;
        let m_val = read_elem(&m, index * esize, esize);
        let m_s = sext_elem(m_val, bits);
        let lo = -(1i128 << (bits - 1));
        let hi = (1i128 << (bits - 1)) - 1;
        let elements = 16 / esize;
        for e in 0..elements {
            let off = e * esize;
            let a = read_elem(&n, off, esize);
            let res: u64 = match op {
                0b111110 => a.wrapping_mul(m_val) & mask, // MUL (low half)
                0b000010 => read_elem(&acc, off, esize).wrapping_add(a.wrapping_mul(m_val)) & mask, // MLA
                0b000011 => read_elem(&acc, off, esize).wrapping_sub(a.wrapping_mul(m_val)) & mask, // MLS
                0b111100 => {
                    // SQDMULH: sat((2*a*b) >> bits) == sat((a*b) >> (bits-1)).
                    let prod = sext_elem(a, bits) * m_s;
                    (prod >> (bits - 1)).clamp(lo, hi) as u64 & mask
                }
                0b111101 => {
                    // SQRDMULH: sat((2*a*b + 2^(bits-1)) >> bits), rewritten as
                    // sat((a*b + 2^(bits-2)) >> (bits-1)) to avoid i128 overflow.
                    let prod = sext_elem(a, bits) * m_s;
                    ((prod + (1i128 << (bits - 2))) >> (bits - 1)).clamp(lo, hi) as u64 & mask
                }
                0b000100 | 0b000101 => {
                    // SQRDMLAH (000100) / SQRDMLSH (000101): Zda + rounded
                    // doubling-high of (+/-)a*Zm[idx]; the accumulate saturates.
                    // The product is negated before the rounding bias, matching
                    // qemu (differs from negating the rounded result at ties).
                    let prod = sext_elem(a, bits) * m_s;
                    let p = if op == 0b000101 { -prod } else { prod };
                    let sdrh = (p + (1i128 << (bits - 2))) >> (bits - 1);
                    let cur = sext_elem(read_elem(&acc, off, esize), bits);
                    (cur + sdrh).clamp(lo, hi) as u64 & mask
                }
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, off, esize, res);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE integer predicated operations.
    fn exec_sve_int_pred(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        _zm: usize,
        pg: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        // SVE predicated integer ALU (destructive): Zdn = op(Zdn, Zm) for active
        // elements, Zdn unchanged for inactive. The governing predicate Pg is
        // BYTE-granular (element e of `esize` bytes is active iff bit e*esize is
        // set). The op is (group=bits[21:19], opc=bits[18:16]):
        //   000: 000 ADD,  001 SUB,  011 SUBR
        //   001: 000 SMAX, 001 UMAX, 010 SMIN, 011 UMIN, 100 SABD, 101 UABD
        //   010: 000 MUL,  010 SMULH,011 UMULH,100 SDIV, 101 UDIV, 110 SDIVR, 111 UDIVR
        //   011: 000 ORR,  001 EOR,  010 AND,  011 BIC
        // The predicated ALU group has bits[15:13]==000; other values (shifts
        // =100, etc.) are handled by dedicated dispatch arms.
        if (insn >> 13) & 0x7 != 0b000 {
            return Ok(CpuExit::Undefined(insn));
        }
        let group = (insn >> 19) & 0x7;
        // SDIV/UDIV/SDIVR/UDIVR only exist for word and doubleword elements;
        // byte/halfword encodings are unallocated regardless of the predicate.
        if group == 0b010 && (insn >> 18) & 1 == 1 && esize < 4 {
            return Ok(CpuExit::Undefined(insn));
        }
        let opc = (insn >> 16) & 0x7;
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let a_reg = self.v[zd].to_le_bytes(); // Zdn (first source, also dest)
        let b_reg = self.v[zn].to_le_bytes(); // Zm (second source)
        let mut dst = a_reg;
        // Signed divide over the (sign-extended) element values. Division by
        // zero yields 0; the MIN/-1 case never overflows i128 for esize<=64 and
        // the subsequent element mask wraps it to the architectural result.
        let sdiv = |n: i128, d: i128| -> i128 { if d == 0 { 0 } else { n / d } };
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 0 {
                continue;
            }
            let off = e * esize;
            let a = read_elem(&a_reg, off, esize);
            let b = read_elem(&b_reg, off, esize);
            let sa = sext_elem(a, bits);
            let sb = sext_elem(b, bits);
            let ua = uext_elem(a, bits);
            let ub = uext_elem(b, bits);
            let r = match (group, opc) {
                (0b000, 0b000) => a.wrapping_add(b),
                (0b000, 0b001) => a.wrapping_sub(b),
                (0b000, 0b011) => b.wrapping_sub(a),
                (0b001, 0b000) => {
                    if sa > sb {
                        a
                    } else {
                        b
                    }
                }
                (0b001, 0b001) => {
                    if ua > ub {
                        a
                    } else {
                        b
                    }
                }
                (0b001, 0b010) => {
                    if sa < sb {
                        a
                    } else {
                        b
                    }
                }
                (0b001, 0b011) => {
                    if ua < ub {
                        a
                    } else {
                        b
                    }
                }
                (0b001, 0b100) => (sa - sb).unsigned_abs() as u64,
                (0b001, 0b101) => (if ua > ub { ua - ub } else { ub - ua }) as u64,
                (0b010, 0b000) => a.wrapping_mul(b),
                (0b010, 0b010) => ((sa * sb) >> bits) as u64,
                (0b010, 0b011) => ((ua * ub) >> bits) as u64,
                (0b010, 0b100) if esize >= 4 => sdiv(sa, sb) as u64,
                (0b010, 0b101) if esize >= 4 => (if ub == 0 { 0 } else { ua / ub }) as u64,
                (0b010, 0b110) if esize >= 4 => sdiv(sb, sa) as u64,
                (0b010, 0b111) if esize >= 4 => (if ua == 0 { 0 } else { ub / ua }) as u64,
                (0b011, 0b000) => a | b,
                (0b011, 0b001) => a ^ b,
                (0b011, 0b010) => a & b,
                (0b011, 0b011) => a & !b,
                _ => return Ok(CpuExit::Undefined(insn)),
            } & mask;
            write_elem(&mut dst, off, esize, r);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE predicated shift by immediate (destructive, merging). The
    /// element size AND shift amount are jointly encoded in tsz:imm: esize is
    /// the lowest set bit of tsize=tszh:tszl; for ASR/LSR amount = 2*esize -
    /// tszimm, for LSL amount = tszimm - esize.
    fn exec_sve_shift_imm(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let tszh = (insn >> 22) & 0x3;
        let tszl = (insn >> 8) & 0x3;
        let imm3 = (insn >> 5) & 0x7;
        let tsize = (tszh << 2) | tszl;
        if tsize == 0 {
            return Ok(CpuExit::Undefined(insn));
        }
        // esize from the highest set bit of tsize (0001->8, 001x->16, 01xx->32,
        // 1xxx->64).
        let bits: u32 = if tsize & 0b1000 != 0 {
            64
        } else if tsize & 0b0100 != 0 {
            32
        } else if tsize & 0b0010 != 0 {
            16
        } else {
            8
        };
        let esize = (bits / 8) as usize;
        let tszimm = (tsize << 3) | imm3;
        // The operation is the full bits[21:16]; the low three bits alone do not
        // distinguish e.g. ASRD (000_100) from SRSHR (001_100).
        let op6 = (insn >> 16) & 0x3F;
        // Shift-left ops take amount = tszimm - bits; shift-right ops take
        // amount = 2*bits - tszimm.
        let is_shl = matches!(op6, 0b000_011 | 0b000_110 | 0b000_111 | 0b001_111);
        let amount = if is_shl {
            tszimm - bits
        } else {
            2 * bits - tszimm
        };
        let pg = ((insn >> 10) & 0x7) as usize;
        let zd = (insn & 0x1F) as usize;
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let mask = elem_mask(bits);
        let a_reg = self.v[zd].to_le_bytes();
        let mut dst = a_reg;
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 0 {
                continue;
            }
            let off = e * esize;
            let v = read_elem(&a_reg, off, esize);
            let r = match op6 {
                0b000_000 => (sext_elem(v, bits) >> amount) as u64 & mask, // ASR
                0b000_001 => (uext_elem(v, bits) >> amount) as u64 & mask, // LSR
                0b000_011 => (uext_elem(v, bits) << amount) as u64 & mask, // LSL
                0b000_100 => {
                    // ASRD: signed shift-right rounding toward zero (divide).
                    let n = sext_elem(v, bits);
                    let bias = if n < 0 { (1i128 << amount) - 1 } else { 0 };
                    ((n + bias) >> amount) as u64 & mask
                }
                0b000_110 => {
                    // SQSHL: signed saturating shift left.
                    if bits == 64 {
                        sqrshl_d(v as i64, amount as i64, false, true) as u64 & mask
                    } else {
                        sqrshl_bhs(sext_elem(v, bits) as i32, amount as i32, bits, false, true)
                            as u64
                            & mask
                    }
                }
                0b000_111 => {
                    // UQSHL: unsigned saturating shift left.
                    if bits == 64 {
                        uqrshl_d(uext_elem(v, bits) as u64, amount as i64, false, true) & mask
                    } else {
                        uqrshl_bhs(uext_elem(v, bits) as u32, amount as i32, bits, false, true)
                            as u64
                            & mask
                    }
                }
                0b001_100 => sve_srshr(sext_elem(v, bits) as i64, amount) as u64 & mask, // SRSHR
                0b001_101 => sve_urshr(uext_elem(v, bits) as u64, amount) & mask,        // URSHR
                0b001_111 => {
                    // SQSHLU: signed shift left, saturating into the unsigned range.
                    let src = sext_elem(v, bits);
                    if src < 0 {
                        0
                    } else if bits == 64 {
                        uqrshl_d(src as u64, amount as i64, false, true) & mask
                    } else {
                        uqrshl_bhs(src as u32, amount as i32, bits, false, true) as u64 & mask
                    }
                }
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE predicated shift by vector (destructive): Zdn = shift(Zdn,
    /// Zm) per active element. opc=bits[18:16]: 000=ASR, 001=LSR, 011=LSL. The
    /// shift amount is the (unsigned) Zm element; out-of-range gives 0 (LSR/LSL)
    /// or a full arithmetic shift (ASR). Pg is byte-granular.
    fn exec_sve_shift_pred(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        pg: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 16) & 0x7;
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let a_reg = self.v[zd].to_le_bytes(); // Zdn
        let b_reg = self.v[zn].to_le_bytes(); // Zm-field
        // bit18 selects the reversed form (ASRR/LSRR/LSLR), which swaps the
        // value and shift-amount operands; base op is bits[17:16].
        let reversed = opc & 0b100 != 0;
        let base_op = opc & 0b011;
        let mut dst = a_reg;
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 0 {
                continue;
            }
            let off = e * esize;
            let za = read_elem(&a_reg, off, esize);
            let zb = read_elem(&b_reg, off, esize);
            let (a, sh) = if reversed { (zb, za) } else { (za, zb) };
            let r = match base_op {
                0b000 => {
                    let s = sh.min((bits - 1) as u64);
                    (sext_elem(a, bits) >> s) as u64 & mask
                }
                0b001 => {
                    if sh >= bits as u64 {
                        0
                    } else {
                        (a >> sh) & mask
                    }
                }
                0b011 => {
                    if sh >= bits as u64 {
                        0
                    } else {
                        (a << sh) & mask
                    }
                }
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE LASTA/LASTB/CLASTA/CLASTB to a GPR. `B` (bit16) takes the
    /// last active element; `A` takes the element after it (wrapping). The
    /// conditional (C) forms keep Rdn when no element is active.
    fn exec_sve_lastx(&mut self, insn: u32, esize: usize) -> Result<CpuExit, ArmError> {
        let before = (insn >> 16) & 1 == 1; // B = take the last active element
        let conditional = (insn >> 20) & 1 == 1; // CLAST
        let pg = ((insn >> 10) & 0x7) as usize;
        let zn = ((insn >> 5) & 0x1F) as usize;
        let rd = (insn & 0x1F) as u8;
        let mask = self.sve_p[pg];
        let n = 16 / esize;
        let op = self.v[zn].to_le_bytes();
        let em = elem_mask((esize * 8) as u32);
        let mut last: i32 = -1;
        for e in (0..n).rev() {
            if (mask >> (e * esize)) & 1 == 1 {
                last = e as i32;
                break;
            }
        }
        let res = if conditional && last < 0 {
            self.get_x(rd) & em
        } else {
            let idx = if before {
                if last < 0 { n - 1 } else { last as usize }
            } else {
                let i = (last + 1) as usize;
                if i >= n { 0 } else { i }
            };
            read_elem(&op, idx * esize, esize) & em
        };
        self.set_x(rd, res);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE CPY/MOV (predicated copy). `mode`: 0=immediate (Pg=4-bit,
    /// merging or zeroing), 1=scalar GPR (Rn, SP if 31, merging), 2=SIMD scalar
    /// Vn (merging). Pg is byte-granular.
    fn exec_sve_cpy(&mut self, insn: u32, esize: usize, mode: u32) -> Result<CpuExit, ArmError> {
        let zd = (insn & 0x1F) as usize;
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let elements = 16 / esize;
        let (pg, merging, elem_val) = match mode {
            0 => {
                // LSL #8 (sh=1) is undefined for byte elements.
                if esize == 1 && (insn >> 13) & 1 == 1 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let pg = ((insn >> 16) & 0xF) as usize; // 4-bit predicate
                let merging = (insn >> 14) & 1 == 1;
                let imm8 = ((insn >> 5) & 0xFF) as u8 as i8 as i64;
                let imm = if (insn >> 13) & 1 == 1 {
                    imm8 << 8
                } else {
                    imm8
                };
                (pg, merging, (imm as u64) & mask)
            }
            1 => {
                let pg = ((insn >> 10) & 0x7) as usize;
                let rn = ((insn >> 5) & 0x1F) as u8;
                let v = if rn == 31 {
                    self.current_sp()
                } else {
                    self.get_x(rn)
                };
                (pg, true, v & mask)
            }
            _ => {
                let pg = ((insn >> 10) & 0x7) as usize;
                let vn = ((insn >> 5) & 0x1F) as usize;
                (pg, true, (self.v[vn] as u64) & mask)
            }
        };
        let pred = self.sve_p[pg];
        let mut dst = if merging {
            self.v[zd].to_le_bytes()
        } else {
            [0u8; 16]
        };
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 1 {
                write_elem(&mut dst, e * esize, esize, elem_val);
            }
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE integer reduction (predicated) to a scalar in Vd. opc6 =
    /// bits[21:16]: SADDV(000000)/UADDV(000001) give a 64-bit sum; SMAXV/UMAXV/
    /// SMINV/UMINV (0010xx) and ANDV/ORV/EORV (0110xx) give an esize result.
    /// Inactive elements use the operation identity. Pg is byte-granular.
    fn exec_sve_int_reduce(&mut self, insn: u32, esize: usize) -> Result<CpuExit, ArmError> {
        let opc6 = (insn >> 16) & 0x3F;
        // SADDV has no 64-bit form (use UADDV.D for that).
        if opc6 == 0b000000 && esize == 8 {
            return Ok(CpuExit::Undefined(insn));
        }
        let pg = ((insn >> 10) & 0x7) as usize;
        let zn = ((insn >> 5) & 0x1F) as usize;
        let vd = (insn & 0x1F) as usize;
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let src = self.v[zn].to_le_bytes();
        let mut act: Vec<u64> = Vec::with_capacity(elements);
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 1 {
                act.push(read_elem(&src, e * esize, esize));
            }
        }
        let result: u128 = match opc6 {
            0b000000 => (act.iter().map(|&x| sext_elem(x, bits)).sum::<i128>() as u64) as u128,
            0b000001 => (act.iter().map(|&x| uext_elem(x, bits)).sum::<u128>() as u64) as u128,
            0b001000 => {
                (act.iter()
                    .map(|&x| sext_elem(x, bits))
                    .max()
                    .unwrap_or(-(1i128 << (bits - 1))) as u64
                    & mask) as u128
            }
            0b001001 => {
                (act.iter().map(|&x| uext_elem(x, bits)).max().unwrap_or(0) as u64 & mask) as u128
            }
            0b001010 => {
                (act.iter()
                    .map(|&x| sext_elem(x, bits))
                    .min()
                    .unwrap_or((1i128 << (bits - 1)) - 1) as u64
                    & mask) as u128
            }
            0b001011 => {
                (act.iter()
                    .map(|&x| uext_elem(x, bits))
                    .min()
                    .unwrap_or(mask as u128) as u64
                    & mask) as u128
            }
            0b011000 => (act.iter().fold(0u64, |a, &x| a | x) & mask) as u128, // ORV
            0b011001 => (act.iter().fold(0u64, |a, &x| a ^ x) & mask) as u128, // EORV
            0b011010 => (act.iter().fold(mask, |a, &x| a & x) & mask) as u128, // ANDV
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        self.v[vd] = result;
        Ok(CpuExit::Continue)
    }

    /// Execute an SVE2.1 integer quadword reduction (ADDQV/SMAXQV/UMAXQV/SMINQV/
    /// UMINQV/ANDQV/ORQV/EORQV) to Vd. opc6=bits[21:16]. Each element position is
    /// reduced across the 128-bit segments of Zn (seeded with the op identity);
    /// at VL=128 (one segment) an active lane keeps Zn's value while an inactive
    /// lane takes the identity. Pg is byte-granular. Mirrors qemu DO_VPQ /
    /// DO_LOGIC_QV (the identity is the reduction of the empty active set).
    fn exec_sve_qv_reduce_int(&mut self, insn: u32, esize: usize) -> Result<CpuExit, ArmError> {
        let opc6 = (insn >> 16) & 0x3F;
        let pg = ((insn >> 10) & 0x7) as usize;
        let zn = ((insn >> 5) & 0x1F) as usize;
        let vd = (insn & 0x1F) as usize;
        let bits = (esize * 8) as u32;
        let mask = elem_mask(bits);
        let ident: u64 = match opc6 {
            0b000101 => 0,                                    // ADDQV
            0b001100 => 1u64 << (bits - 1),                   // SMAXQV  -> INT_MIN
            0b001101 => 0,                                    // UMAXQV
            0b001110 => (1u64 << (bits - 1)).wrapping_sub(1), // SMINQV -> INT_MAX
            0b001111 => mask,                                 // UMINQV  -> UINT_MAX
            0b011100 => 0,                                    // ORQV
            0b011101 => 0,                                    // EORQV
            0b011110 => mask,                                 // ANDQV   -> all-ones
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        let pred = self.sve_p[pg];
        let src = self.v[zn].to_le_bytes();
        let mut dst = [0u8; 16];
        for e in 0..(16 / esize) {
            let off = e * esize;
            let v = if (pred >> off) & 1 == 1 {
                read_elem(&src, off, esize)
            } else {
                ident
            };
            write_elem(&mut dst, off, esize, v & mask);
        }
        self.v[vd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE integer unpredicated operations.
    fn exec_sve_int_unpred(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        zm: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        // bits[12:10]: 000=ADD 001=SUB 100=SQADD 101=UQADD 110=SQSUB 111=UQSUB.
        // Map each to the verified NEON three-same integer core (u, opcode).
        let opc = (insn >> 10) & 0x7;
        let (u, neon_op) = match opc {
            0b000 => (0, 0b10000), // ADD
            0b001 => (1, 0b10000), // SUB
            0b100 => (0, 0b00001), // SQADD
            0b101 => (1, 0b00001), // UQADD
            0b110 => (0, 0b00101), // SQSUB
            0b111 => (1, 0b00101), // UQSUB
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        let bits = (esize * 8) as u32;
        let elements = 16 / esize;
        let src = self.v[zn].to_le_bytes();
        let src2 = self.v[zm].to_le_bytes();
        let mut dst = [0u8; 16];
        for e in 0..elements {
            let off = e * esize;
            let a = read_elem(&src, off, esize);
            let b = read_elem(&src2, off, esize);
            write_elem(
                &mut dst,
                off,
                esize,
                adv_simd_three_same_int(u, neon_op, bits, a, b, 0),
            );
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE unpredicated bitwise logical (AND/ORR/EOR/BIC), selected by
    /// bits[23:22], over the whole vector (element size is irrelevant).
    fn exec_sve_logical_unpred(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        zm: usize,
    ) -> Result<CpuExit, ArmError> {
        let a = self.v[zn];
        let b = self.v[zm];
        self.v[zd] = match (insn >> 22) & 0x3 {
            0b00 => a & b, // AND
            0b01 => a | b, // ORR
            0b10 => a ^ b, // EOR
            _ => a & !b,   // BIC
        };
        Ok(CpuExit::Continue)
    }

    /// Execute SVE predicate-generating operations (PTRUE/PTRUES, PFALSE, the
    /// WHILE family). Predicates are stored BYTE-granular: element `e` (size
    /// `esize` bytes) is governed by bit `e*esize`, matching the architecture
    /// and the differential oracle. The dispatch keys on the real opcode bits
    /// (NOT on op1=bits[24:23], which folds the size field's high bit).
    fn exec_sve_pred_ops(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let size = (insn >> 22) & 0x3;
        let esize = 1usize << size;
        let elements = 16 / esize;
        let pd = (insn & 0xF) as usize;
        let b15_10 = (insn >> 10) & 0x3F;

        // First-fault register (FFR) manipulation — handled first since these
        // are fully-fixed encodings. SETFFR sets every PL bit (16 at VL=128).
        if insn == 0x252C_9000 {
            self.sve_ffr = 0xFFFF;
            return Ok(CpuExit::Continue);
        }
        // WRFFR Pn: FFR = P[Pn].
        if insn & 0xFFFF_FE1F == 0x2528_9000 {
            self.sve_ffr = self.sve_p[((insn >> 5) & 0xF) as usize];
            return Ok(CpuExit::Continue);
        }
        // RDFFR Pd (unpredicated): P[Pd] = FFR.
        if (insn >> 10) & 0x3FFF == 0x67C && (insn >> 4) & 0x3F == 0 {
            self.sve_p[pd] = self.sve_ffr;
            return Ok(CpuExit::Continue);
        }
        // RDFFR/RDFFRS Pd, Pg/Z (predicated): P[Pd] = FFR & P[Pg] (zeroing). The
        // S-bit form (bit22, RDFFRS) also sets NZCV = PredTest(Pg, result). The
        // mask ignores bit22 (==bit12 of the shifted field).
        if (insn >> 10) & 0x2FFF == 0x63C && (insn >> 9) & 1 == 0 && (insn >> 4) & 1 == 0 {
            let pgl = ((insn >> 5) & 0xF) as usize;
            let r = self.sve_ffr & self.sve_p[pgl];
            self.sve_p[pd] = r;
            if (insn >> 22) & 1 == 1 {
                let (n, z, c, v) = pred_test(self.sve_p[pgl], r, 16, 1);
                self.set_n(n);
                self.set_z(z);
                self.set_c(c);
                self.set_v(v);
            }
            return Ok(CpuExit::Continue);
        }

        // PTEST Pg, Pn.B: NZCV = PredTest(Pg, Pn) at byte granularity (the
        // predicate result is unchanged; only flags are written). Encoding
        // 00100101 01 010000 11 pg 0 rn 0 0000.
        if insn & 0xFFFF_C21F == 0x2550_C000 {
            let pgl = ((insn >> 10) & 0xF) as usize;
            let pn = ((insn >> 5) & 0xF) as usize;
            let (n, z, c, v) = pred_test(self.sve_p[pgl], self.sve_p[pn], 16, 1);
            self.set_n(n);
            self.set_z(z);
            self.set_c(c);
            self.set_v(v);
            return Ok(CpuExit::Continue);
        }

        // FDUP: broadcast an FP modified-immediate to all lanes. 0x25,
        // bits[21:13]==111001110. Unpredicated; size 0 reserved.
        if (insn >> 13) & 0x1FF == 0b111001110 {
            if esize < 2 {
                return Ok(CpuExit::Undefined(insn));
            }
            let zd = (insn & 0x1F) as usize;
            let val = vfp_expand_imm(((insn >> 5) & 0xFF) as u8, esize);
            let mut out = 0u128;
            for e in 0..elements {
                out |= (val as u128) << (e * esize * 8);
            }
            self.v[zd] = out;
            return Ok(CpuExit::Continue);
        }

        // SVE integer compare with signed immediate -> predicate: 0x25,
        // bit21==0, condition (bits[15:13], bit4): GE/GT/LT/LE/EQ/NE. imm5 is
        // signed (bits[20:16]). Zeroing under Pg; sets NZCV via PredTest(Pg).
        if (insn >> 24) & 0xFF == 0b00100101
            && (insn >> 21) & 1 == 0
            && matches!(
                ((insn >> 13) & 0x7, (insn >> 4) & 1),
                (0b000, 0) | (0b000, 1) | (0b001, 0) | (0b001, 1) | (0b100, 0) | (0b100, 1)
            )
        {
            let cc = ((insn >> 13) & 0x7, (insn >> 4) & 1);
            let imm = (((insn >> 16) & 0x1F) as i64) << 59 >> 59; // sign-extend imm5
            let pgi = ((insn >> 10) & 0x7) as usize;
            let bits = (esize * 8) as u32;
            let pred = self.sve_p[pgi];
            let n = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
            let mut result = 0u32;
            for e in 0..elements {
                let off = e * esize;
                if (pred >> off) & 1 == 0 {
                    continue;
                }
                let a = sext_elem(read_elem(&n, off, esize), bits) as i64;
                let r = match cc {
                    (0b000, 0) => a >= imm,
                    (0b000, 1) => a > imm,
                    (0b001, 0) => a < imm,
                    (0b001, 1) => a <= imm,
                    (0b100, 0) => a == imm,
                    _ => a != imm,
                };
                if r {
                    result |= 1 << off;
                }
            }
            self.sve_p[pd] = result;
            let (nf, zf, cf, vf) = pred_test(pred, result, elements, esize);
            self.set_n(nf);
            self.set_z(zf);
            self.set_c(cf);
            self.set_v(vf);
            return Ok(CpuExit::Continue);
        }

        // SVE integer compare with unsigned immediate -> predicate: 0x24,
        // bit21==1. imm7=bits[20:14] (unsigned); condition (bit13, bit4):
        // HS/HI/LO/LS. Zeroing under Pg; sets NZCV via PredTest(Pg).
        if (insn >> 24) & 0xFF == 0b00100100 && (insn >> 21) & 1 == 1 {
            let imm = ((insn >> 14) & 0x7F) as u64;
            let lohi = ((insn >> 13) & 1, (insn >> 4) & 1);
            let pgi = ((insn >> 10) & 0x7) as usize;
            let bits = (esize * 8) as u32;
            let pred = self.sve_p[pgi];
            let n = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
            let mut result = 0u32;
            for e in 0..elements {
                let off = e * esize;
                if (pred >> off) & 1 == 0 {
                    continue;
                }
                let a = uext_elem(read_elem(&n, off, esize), bits) as u64;
                let r = match lohi {
                    (0, 0) => a >= imm,
                    (0, 1) => a > imm,
                    (1, 0) => a < imm,
                    _ => a <= imm,
                };
                if r {
                    result |= 1 << off;
                }
            }
            self.sve_p[pd] = result;
            let (nf, zf, cf, vf) = pred_test(pred, result, elements, esize);
            self.set_n(nf);
            self.set_z(zf);
            self.set_c(cf);
            self.set_v(vf);
            return Ok(CpuExit::Continue);
        }

        // DUP Zd.T, #imm{,LSL #8} (unpredicated immediate broadcast): bits[21:16]
        // ==111000, bits[15:14]==11. (Distinct from PTRUE by bit21==1.)
        if (insn >> 16) & 0x3F == 0b111000 && (insn >> 14) & 0x3 == 0b11 {
            // LSL #8 (sh=1) is undefined for byte elements.
            if esize == 1 && (insn >> 13) & 1 == 1 {
                return Ok(CpuExit::Undefined(insn));
            }
            let zd = (insn & 0x1F) as usize;
            let imm8 = ((insn >> 5) & 0xFF) as u8 as i8 as i64;
            let imm = if (insn >> 13) & 1 == 1 {
                imm8 << 8
            } else {
                imm8
            };
            let elem_val = (imm as u64) & elem_mask((esize * 8) as u32);
            let mut dst = [0u8; 16];
            for e in 0..(16 / esize) {
                write_elem(&mut dst, e * esize, esize, elem_val);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // CNTP Rd, Pg, Pn.T: count active Pn elements under Pg -> 64-bit GPR.
        // bits[21:16]==100000, bits[15:14]==10.
        if (insn >> 24) & 0xFF == 0b00100101
            && (insn >> 16) & 0x3F == 0b100000
            && (insn >> 14) & 0x3 == 0b10
        {
            let pgl = ((insn >> 10) & 0xF) as usize;
            let pn = ((insn >> 5) & 0xF) as usize;
            let rd = (insn & 0x1F) as u8;
            let (mask, op) = (self.sve_p[pgl], self.sve_p[pn]);
            let mut sum = 0u64;
            for e in 0..(16 / esize) {
                let b = e * esize;
                if (mask >> b) & 1 == 1 && (op >> b) & 1 == 1 {
                    sum += 1;
                }
            }
            self.set_x(rd, sum);
            return Ok(CpuExit::Continue);
        }

        // INCP/DECP: increment/decrement a GPR (R form, bit11==1) or each Z
        // element (Z form, bit11==0) by the active-element count of Pg.
        // bits[21:17]==10110 (bit16: 0=INC, 1=DEC), bits[15:12]==1000.
        if (insn >> 24) & 0xFF == 0b00100101
            && (insn >> 17) & 0x1F == 0b10110
            && (insn >> 12) & 0xF == 0b1000
        {
            let dec = (insn >> 16) & 1 == 1;
            let is_z = (insn >> 11) & 1 == 0;
            let pgl = ((insn >> 5) & 0xF) as usize;
            let dn = (insn & 0x1F) as usize;
            let mask = self.sve_p[pgl];
            let mut count = 0u64;
            for e in 0..(16 / esize) {
                if (mask >> (e * esize)) & 1 == 1 {
                    count += 1;
                }
            }
            if is_z {
                if esize == 1 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let a = self.v[dn].to_le_bytes();
                let mut dst = a;
                let em = elem_mask((esize * 8) as u32);
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    let v = read_elem(&a, off, esize);
                    let r = if dec {
                        v.wrapping_sub(count)
                    } else {
                        v.wrapping_add(count)
                    } & em;
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[dn] = u128::from_le_bytes(dst);
            } else {
                let cur = self.get_x(dn as u8);
                self.set_x(
                    dn as u8,
                    if dec {
                        cur.wrapping_sub(count)
                    } else {
                        cur.wrapping_add(count)
                    },
                );
            }
            return Ok(CpuExit::Continue);
        }

        // SQINCP/UQINCP/SQDECP/UQDECP (saturating INCP/DECP): bits[21:18]==1010,
        // d=bit17 (0=inc, 1=dec), u=bit16 (0=signed SQ, 1=unsigned UQ),
        // bits[15:11]==10001 (GPR r-form) / 10000 (vector z-form). For the GPR
        // form bit10 selects 64-bit (1) vs 32-bit (0) saturation width. The
        // count of active Pg elements (at this esz) is the saturating delta.
        if (insn >> 24) & 0xFF == 0b00100101
            && (insn >> 18) & 0xF == 0b1010
            && (insn >> 12) & 0xF == 0b1000
        {
            let dec = (insn >> 17) & 1 == 1;
            let uns = (insn >> 16) & 1 == 1;
            let is_z = (insn >> 11) & 1 == 0;
            let pgl = ((insn >> 5) & 0xF) as usize;
            let dn = (insn & 0x1F) as usize;
            let mask = self.sve_p[pgl];
            let mut count = 0u64;
            for e in 0..(16 / esize) {
                if (mask >> (e * esize)) & 1 == 1 {
                    count += 1;
                }
            }
            if is_z {
                // Vector form: per-element saturating add/sub. esz==0 (byte) is
                // unallocated for the z-form.
                if esize == 1 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let a = self.v[dn].to_le_bytes();
                let mut dst = a;
                let bits = (esize * 8) as u32;
                for e in 0..(16 / esize) {
                    let off = e * esize;
                    let v = read_elem(&a, off, esize);
                    let r = sat_addsub_elem(v, count, bits, uns, dec);
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[dn] = u128::from_le_bytes(dst);
            } else {
                // GPR form: bit10 picks 64-bit vs 32-bit saturation width.
                let sf64 = (insn >> 10) & 1 == 1;
                let cur = self.get_x(dn as u8);
                let res = if sf64 {
                    sat_addsub_64(cur, count, uns, dec)
                } else {
                    sat_addsub_32(cur, count, uns, dec)
                };
                self.set_x(dn as u8, res);
            }
            return Ok(CpuExit::Continue);
        }

        // CMP<cc>_P.P.ZZ (bits[31:24]==0x24): predicated vector compare producing
        // a zeroing predicate Pd, then NZCV = PredTest(Pg, result). The compare
        // is (bits[15:13], bit4): (000,0)HS (000,1)HI (100,0)GE (100,1)GT
        // (101,0)EQ (101,1)NE.
        if (insn >> 24) & 0xFF == 0b00100100 && (insn >> 21) & 1 == 0 {
            let cmp_hi = (insn >> 13) & 0x7;
            let cmp_lo = (insn >> 4) & 1;
            let pg = ((insn >> 10) & 0x7) as usize;
            let zn = ((insn >> 5) & 0x1F) as usize;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let n_reg = self.v[zn].to_le_bytes();
            let m_reg = self.v[zm].to_le_bytes();
            let gov = self.sve_p[pg];
            let bits = (esize * 8) as u32;
            let mut result = 0u32;
            for e in 0..elements {
                let b = e * esize;
                if (gov >> b) & 1 == 0 {
                    continue; // inactive -> 0 (zeroing predicate)
                }
                let a = read_elem(&n_reg, b, esize);
                let c = read_elem(&m_reg, b, esize);
                let cond = match (cmp_hi, cmp_lo) {
                    (0b000, 0) => uext_elem(a, bits) >= uext_elem(c, bits),
                    (0b000, 1) => uext_elem(a, bits) > uext_elem(c, bits),
                    (0b100, 0) => sext_elem(a, bits) >= sext_elem(c, bits),
                    (0b100, 1) => sext_elem(a, bits) > sext_elem(c, bits),
                    (0b101, 0) => a == c,
                    (0b101, 1) => a != c,
                    _ => return Ok(CpuExit::Undefined(insn)),
                };
                if cond {
                    result |= 1 << b;
                }
            }
            self.sve_p[pd] = result;
            let (n, z, cf, v) = pred_test(gov, result, elements, esize);
            self.set_n(n);
            self.set_z(z);
            self.set_c(cf);
            self.set_v(v);
            return Ok(CpuExit::Continue);
        }

        // Predicate-on-predicate logical ops (Pd = Pg & op(Pn, Pm), zeroing):
        // 0x25, bits[21:20]==00, bits[15:14]==01. Op selected by (bit23, bit9,
        // bit4). These work on the raw VL/8-bit (16 at VL=128) predicate values,
        // no element size. bits[21:20] MUST be 00 — the BRKA/BRKB/BRKN family
        // shares bits[15:14]==01 but has bits[21:20]==01.
        if (insn >> 24) & 0xFF == 0b00100101
            && (insn >> 20) & 0x3 == 0b00
            && (insn >> 14) & 0x3 == 0b01
        {
            let pm = ((insn >> 16) & 0xF) as usize;
            let pgl = ((insn >> 10) & 0xF) as usize;
            let pn = ((insn >> 5) & 0xF) as usize;
            let s = (insn >> 22) & 1;
            let vg = self.sve_p[pgl];
            let vn = self.sve_p[pn];
            let vm = self.sve_p[pm];
            let sel = (insn >> 23) & 1 == 0 && (insn >> 9) & 1 == 1 && (insn >> 4) & 1 == 1;
            let r = if sel {
                // SEL Pd = Pg ? Pn : Pm (per bit). Not zeroing, never sets flags.
                ((vg & vn) | (!vg & vm)) & 0xFFFF
            } else {
                (match ((insn >> 23) & 1, (insn >> 9) & 1, (insn >> 4) & 1) {
                    (0, 0, 0) => vg & vn & vm,    // AND(S)
                    (0, 0, 1) => vg & vn & !vm,   // BIC(S)
                    (0, 1, 0) => vg & (vn ^ vm),  // EOR(S)
                    (1, 0, 0) => vg & (vn | vm),  // ORR(S)
                    (1, 0, 1) => vg & (vn | !vm), // ORN(S)
                    (1, 1, 0) => vg & !(vn | vm), // NOR(S)
                    (1, 1, 1) => vg & !(vn & vm), // NAND(S)
                    _ => return Ok(CpuExit::Undefined(insn)),
                }) & 0xFFFF
            };
            self.sve_p[pd] = r;
            // The S-bit forms (ANDS/BICS/.../MOVS) set NZCV = PredTest(Pg, Pd);
            // SEL never sets flags.
            if s == 1 && !sel {
                let (n, z, c, v) = pred_test(vg, r, 16, 1);
                self.set_n(n);
                self.set_z(z);
                self.set_c(c);
                self.set_v(v);
            }
            return Ok(CpuExit::Continue);
        }

        // PFALSE Pd: writes an all-false predicate (bits[15:10]==111001).
        if b15_10 == 0b111001 {
            self.sve_p[pd] = 0;
            return Ok(CpuExit::Continue);
        }

        // PTRUE / PTRUES Pd.T, pattern: bits[15:10]==111000, S=bit16. PTRUES
        // sets NZCV = PredTest(result, result) — i.e. the result governs itself,
        // so C = !LastActive collapses to (result == 0).
        if b15_10 == 0b111000 {
            let s = (insn >> 16) & 1;
            let pattern = (insn >> 5) & 0x1F;
            let count = sve_pattern_count(pattern, elements);
            let mut pred = 0u32;
            for e in 0..count {
                pred |= 1 << (e * esize);
            }
            self.sve_p[pd] = pred;
            if s == 1 {
                let empty = pred == 0;
                self.set_n(!empty);
                self.set_z(empty);
                self.set_c(empty);
                self.set_v(false);
            }
            return Ok(CpuExit::Continue);
        }

        // CTERMEQ/CTERMNE: 0x25, bit23==1, bit21==1, bits[15:10]==001000.
        // Compares two GP registers (sf=bit22 -> 64/32-bit); sets N to the
        // comparison result and V=!N&!C, leaving Z and C unchanged. bit4 = NE.
        if (insn >> 23) & 1 == 1 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3F == 0b001000 {
            let sf = (insn >> 22) & 1 == 1;
            let ne = (insn >> 4) & 1 == 1;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let (a, b) = if sf {
                (self.get_x(rn), self.get_x(rm))
            } else {
                (self.get_w(rn) as u64, self.get_w(rm) as u64)
            };
            let cmp = if ne { a != b } else { a == b };
            self.set_n(cmp);
            let c = self.get_c();
            self.set_v(!cmp & !c);
            return Ok(CpuExit::Continue);
        }

        // WHILE family (RR): bit21==1, bits[15:13]==000, bit10==1. Compares a
        // running index against a limit; bits[11:10]: 01=signed, 11=unsigned;
        // bit4: 0=strict (<), 1=inclusive (<=). The result is a contiguous run
        // of active elements from element 0, and NZCV is set from the result.
        if (insn >> 21) & 1 == 1 && (insn >> 13) & 0x7 == 0 && (insn >> 10) & 1 == 1 {
            let sf = (insn >> 12) & 1;
            let unsigned = (insn >> 10) & 0x3 == 0b11;
            let inclusive = (insn >> 4) & 1 == 1;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let mut pred = 0u32;
            for e in 0..elements {
                let active = if unsigned {
                    let a = if sf == 1 {
                        self.get_x(rn)
                    } else {
                        self.get_w(rn) as u64
                    };
                    let b = if sf == 1 {
                        self.get_x(rm)
                    } else {
                        self.get_w(rm) as u64
                    };
                    let idx = a.wrapping_add(e as u64);
                    // Once the running index wraps below the start it stays inactive.
                    if idx < a {
                        false
                    } else if inclusive {
                        idx <= b
                    } else {
                        idx < b
                    }
                } else {
                    let a = if sf == 1 {
                        self.get_x(rn) as i64
                    } else {
                        self.get_w(rn) as i32 as i64
                    };
                    let b = if sf == 1 {
                        self.get_x(rm) as i64
                    } else {
                        self.get_w(rm) as i32 as i64
                    };
                    let idx = a.wrapping_add(e as i64);
                    if inclusive { idx <= b } else { idx < b }
                };
                if active {
                    pred |= 1 << (e * esize);
                }
            }
            self.sve_p[pd] = pred;
            let (n, z, c, v) = pred_test_flags(pred, elements, esize);
            self.set_n(n);
            self.set_z(z);
            self.set_c(c);
            self.set_v(v);
            return Ok(CpuExit::Continue);
        }

        // WHILE gt-family (RR): bit21==1, bits[15:13]==000, bit10==0. The running
        // index decreases from rn toward rm. bit11: 0=signed (GT/GE), 1=unsigned
        // (HI/HS). Per qemu do_WHILE, the "or-equal" sense is inverted vs the
        // lt-family: bit4==0 => GE/HS (inclusive), bit4==1 => GT/HI (strict).
        if (insn >> 21) & 1 == 1 && (insn >> 13) & 0x7 == 0 && (insn >> 10) & 1 == 0 {
            let sf = (insn >> 12) & 1 == 1;
            let unsigned = (insn >> 11) & 1 == 1;
            // a->eq: GT/HI have it set; the comparison "or-equal" flag is its
            // negation for the gt-family (eq = a->eq == lt, lt == false here).
            let eq = (insn >> 4) & 1 == 0;
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let tmax = elements as u64;
            let count: u64 = if unsigned {
                let a = if sf {
                    self.get_x(rn)
                } else {
                    self.get_w(rn) as u64
                };
                let b = if sf {
                    self.get_x(rm)
                } else {
                    self.get_w(rm) as u64
                };
                let cond = if eq { a >= b } else { a > b };
                if !cond {
                    0
                } else if eq && b == 0 {
                    // op1 == maxval(0): produce an all-true predicate.
                    tmax
                } else {
                    let t0 = (a - b) as u128 + if eq { 1 } else { 0 };
                    t0.min(tmax as u128) as u64
                }
            } else {
                let a = if sf {
                    self.get_x(rn) as i64
                } else {
                    self.get_w(rn) as i32 as i64
                };
                let b = if sf {
                    self.get_x(rm) as i64
                } else {
                    self.get_w(rm) as i32 as i64
                };
                let cond = if eq { a >= b } else { a > b };
                let maxval = if sf { i64::MIN } else { i32::MIN as i64 };
                if !cond {
                    0
                } else if eq && b == maxval {
                    tmax
                } else {
                    let t0 = (a as i128 - b as i128) + if eq { 1 } else { 0 };
                    t0.clamp(0, tmax as i128) as u64
                }
            };
            // The gt-family anchors the contiguous active run at the TOP of the
            // predicate (high-numbered elements), per qemu do_whileg.
            let start = elements - count.min(elements as u64) as usize;
            let mut pred = 0u32;
            for e in start..elements {
                pred |= 1 << (e * esize);
            }
            self.sve_p[pd] = pred;
            let (n, z, c, v) = pred_test_flags(pred, elements, esize);
            self.set_n(n);
            self.set_z(z);
            self.set_c(c);
            self.set_v(v);
            return Ok(CpuExit::Continue);
        }

        // WHILERW / WHILEWR (memory-hazard predicate): 0x25, bit21==1,
        // bits[15:10]==001100, bit4 picks WHILERW(1)/WHILEWR(0). Both produce a
        // monotone prefix of `count` active elements, then set NZCV like the
        // WHILE family. The count (clamped to the element count) follows qemu:
        //   WHILEWR: Xn>=Xm -> all; else (Xm - Xn) >> esz   (no WAR hazard)
        //   WHILERW: Xn==Xm -> all; else |Xn - Xm| >> esz   (no RAW hazard)
        if (insn >> 21) & 1 == 1 && b15_10 == 0b001100 {
            let rn = ((insn >> 5) & 0x1F) as u8;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let rw = (insn >> 4) & 1 == 1;
            let xn = self.get_x(rn);
            let xm = self.get_x(rm);
            let tmax = elements as u64;
            let count = if rw {
                if xn == xm {
                    tmax
                } else {
                    let d = if xn >= xm { xn - xm } else { xm - xn };
                    (d >> size).min(tmax)
                }
            } else if xn >= xm {
                tmax
            } else {
                ((xm - xn) >> size).min(tmax)
            };
            let mut pred = 0u32;
            for e in 0..count as usize {
                pred |= 1 << (e * esize);
            }
            self.sve_p[pd] = pred;
            let (n, z, c, v) = pred_test_flags(pred, elements, esize);
            self.set_n(n);
            self.set_z(z);
            self.set_c(c);
            self.set_v(v);
            return Ok(CpuExit::Continue);
        }

        // BRKA / BRKB (break after / before the first true element of Pn,
        // single source): 0x25, bits[21:16]==010000, bits[15:14]==01, bit9==0.
        // bit23 picks BRKA(0)/BRKB(1); bit22 the flag-setting S form; bit4 the
        // merging(1)/zeroing(0) of Pg-inactive elements. esize is always 1 byte.
        if (insn >> 24) & 1 == 1
            && (insn >> 16) & 0x3F == 0b010000
            && (insn >> 14) & 0x3 == 0b01
            && (insn >> 9) & 1 == 0
        {
            let before = (insn >> 23) & 1 == 1; // BRKB
            let setflags = (insn >> 22) & 1 == 1;
            let merging = (insn >> 4) & 1 == 1;
            // The flag-setting form (BRKAS/BRKBS) is always zeroing: M (bit4)
            // must be 0, so S=1 with M=1 is an unallocated encoding.
            if setflags && merging {
                return Ok(CpuExit::Undefined(insn));
            }
            let pg = ((insn >> 10) & 0xF) as usize;
            let pn = ((insn >> 5) & 0xF) as usize;
            let mask = self.sve_p[pg];
            let operand = self.sve_p[pn];
            let prior = self.sve_p[pd];
            let mut result = 0u32;
            let mut brk = false;
            for e in 0..16 {
                let elem = (operand >> e) & 1 == 1;
                if (mask >> e) & 1 == 1 {
                    if before {
                        brk = brk || elem;
                        if !brk {
                            result |= 1 << e;
                        }
                    } else {
                        if !brk {
                            result |= 1 << e;
                        }
                        brk = brk || elem;
                    }
                } else if merging && (prior >> e) & 1 == 1 {
                    result |= 1 << e;
                }
            }
            if setflags {
                let (n, z, c, v) = pred_test(mask, result, 16, 1);
                self.set_n(n);
                self.set_z(z);
                self.set_c(c);
                self.set_v(v);
            }
            self.sve_p[pd] = result;
            return Ok(CpuExit::Continue);
        }

        // BRKN: 0x25, bit23==0, bits[21:16]==011000, bits[15:14]==01. If the
        // last Pg-active element of Pn is true, the result is Pdm unchanged,
        // else all-false. BRKNS (bit22==1) sets NZCV via PredTest(Ones,result).
        if (insn >> 24) & 1 == 1
            && (insn >> 23) & 1 == 0
            && (insn >> 16) & 0x3F == 0b011000
            && (insn >> 14) & 0x3 == 0b01
        {
            let setflags = (insn >> 22) & 1 == 1;
            let pg = ((insn >> 10) & 0xF) as usize;
            let pn = ((insn >> 5) & 0xF) as usize;
            let mask = self.sve_p[pg];
            let operand1 = self.sve_p[pn];
            let operand2 = self.sve_p[pd]; // Pdm (source + dest)
            let result = if last_active(mask, operand1, 16, 1) {
                operand2
            } else {
                0
            };
            if setflags {
                let (n, z, c, v) = pred_test(0xFFFF, result, 16, 1);
                self.set_n(n);
                self.set_z(z);
                self.set_c(c);
                self.set_v(v);
            }
            self.sve_p[pd] = result;
            return Ok(CpuExit::Continue);
        }

        // BRKPA / BRKPB (propagating partition break): 0x25, bit23==0,
        // bits[21:20]==00, bits[15:14]==11, bit9==0. The carry-in is whether
        // the last Pg-active element of Pn is set; within Pg-active elements the
        // result stays true until the Pm break (after for BRKPA, before BRKPB).
        if (insn >> 24) & 1 == 1
            && (insn >> 23) & 1 == 0
            && (insn >> 20) & 0x3 == 0b00
            && (insn >> 14) & 0x3 == 0b11
            && (insn >> 9) & 1 == 0
        {
            let before = (insn >> 4) & 1 == 1; // BRKPB
            let setflags = (insn >> 22) & 1 == 1;
            let pm = ((insn >> 16) & 0xF) as usize;
            let pg = ((insn >> 10) & 0xF) as usize;
            let pn = ((insn >> 5) & 0xF) as usize;
            let mask = self.sve_p[pg];
            let operand1 = self.sve_p[pn];
            let operand2 = self.sve_p[pm];
            let mut last = last_active(mask, operand1, 16, 1);
            let mut result = 0u32;
            for e in 0..16 {
                if (mask >> e) & 1 == 1 {
                    if before {
                        last = last && (operand2 >> e) & 1 == 0;
                        if last {
                            result |= 1 << e;
                        }
                    } else {
                        if last {
                            result |= 1 << e;
                        }
                        last = last && (operand2 >> e) & 1 == 0;
                    }
                }
            }
            if setflags {
                let (n, z, c, v) = pred_test(mask, result, 16, 1);
                self.set_n(n);
                self.set_z(z);
                self.set_c(c);
                self.set_v(v);
            }
            self.sve_p[pd] = result;
            return Ok(CpuExit::Continue);
        }

        // PFIRST Pdn.B, Pg, Pdn.B: bits[23:16]==01011000, bits[15:9]==1100000,
        // bit4==0. Sets the FIRST Pg-active element true in the (unchanged) Pdn.
        // Always operates on byte elements (esize=8 bits), independent of the
        // bits[23:22] field which is fixed to 01 in the opcode.
        if (insn >> 16) & 0xFF == 0b01011000
            && (insn >> 9) & 0x7F == 0b1100000
            && (insn >> 4) & 1 == 0
        {
            let pg = ((insn >> 5) & 0xF) as usize;
            let mask = self.sve_p[pg];
            let mut result = self.sve_p[pd];
            for e in 0..16 {
                if (mask >> e) & 1 == 1 {
                    result |= 1 << e;
                    break;
                }
            }
            let (n, z, c, v) = pred_test(mask, result, 16, 1);
            self.set_n(n);
            self.set_z(z);
            self.set_c(c);
            self.set_v(v);
            self.sve_p[pd] = result;
            return Ok(CpuExit::Continue);
        }

        // PNEXT Pdn.T, Pg, Pdn.T: bits[21:16]==011001, bits[15:9]==1100010,
        // bit4==0. Finds the next Pg-active element strictly after the last
        // active element of the current Pdn, leaving only that element active.
        if (insn >> 16) & 0x3F == 0b011001
            && (insn >> 9) & 0x7F == 0b1100010
            && (insn >> 4) & 1 == 0
        {
            let pg = ((insn >> 5) & 0xF) as usize;
            let mask = self.sve_p[pg];
            let operand = self.sve_p[pd];
            let mut last: i32 = -1;
            for e in 0..elements {
                if (operand >> (e * esize)) & 1 == 1 {
                    last = e as i32;
                }
            }
            let mut next = (last + 1) as usize;
            while next < elements && (mask >> (next * esize)) & 1 == 0 {
                next += 1;
            }
            let mut result = 0u32;
            if next < elements {
                result |= 1 << (next * esize);
            }
            let (n, z, c, v) = pred_test(mask, result, elements, esize);
            self.set_n(n);
            self.set_z(z);
            self.set_c(c);
            self.set_v(v);
            self.sve_p[pd] = result;
            return Ok(CpuExit::Continue);
        }

        Err(ArmError::Unimplemented(format!(
            "SVE predicate op bits[15:10]={:06b}",
            b15_10
        )))
    }

    /// Execute SVE INDEX: Zd[e] = base + e*step, with base/step from either a
    /// signed 5-bit immediate or an X register. bits[11:10]: bit10 picks the
    /// base source (0=imm5 at [9:5], 1=Xn), bit11 the step source (0=imm5 at
    /// [20:16], 1=Xm).
    fn exec_sve_index(&mut self, insn: u32, zd: usize, esize: usize) -> Result<CpuExit, ArmError> {
        let sext5 = |v: u32| -> i64 { (((v & 0x1F) as i32) << 27 >> 27) as i64 };
        let base: i64 = if (insn >> 10) & 1 == 1 {
            self.get_x(((insn >> 5) & 0x1F) as u8) as i64
        } else {
            sext5((insn >> 5) & 0x1F)
        };
        let step: i64 = if (insn >> 11) & 1 == 1 {
            self.get_x(((insn >> 16) & 0x1F) as u8) as i64
        } else {
            sext5((insn >> 16) & 0x1F)
        };
        let bits = (esize * 8) as u32;
        let m = elem_mask(bits) as u128;
        let elements = 16 / esize;
        let mut dst = 0u128;
        for e in 0..elements {
            let v = base.wrapping_add((e as i64).wrapping_mul(step)) as u64 as u128 & m;
            dst |= v << (e * esize * 8);
        }
        self.v[zd] = dst;
        Ok(CpuExit::Continue)
    }

    /// Execute the SVE element-count / inc-dec-by-element-count / stack-
    /// allocation family (all 0x04): ADDVL/ADDPL/RDVL, CNTB/H/W/D, INCB/DECB...
    /// to a GPR or Z register, and the saturating SQINCB/UQINCB.../SQDECB...
    /// forms. The pattern selects how many elements of size esz the predicate
    /// would have, scaled by MUL #(imm4+1).
    fn exec_sve_elem_count(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let rd = (insn & 0x1F) as u8;
        // Stack-allocation forms (ADDVL/ADDPL/RDVL): bits[15:11]==01010. These
        // use the stack-pointer register encoding (reg 31 == SP, not XZR).
        if (insn >> 11) & 0x1F == 0b01010 {
            let imm6 = (((insn >> 5) & 0x3F) as i64) << 58 >> 58; // sign-extend 6
            let vl_bytes = (self.sve_vl / 8) as i64;
            let rn = ((insn >> 16) & 0x1F) as u8;
            let base = if rn == 31 {
                self.get_sp()
            } else {
                self.get_x(rn)
            };
            let val = match (insn >> 21) & 0x7 {
                0b001 => base.wrapping_add((imm6 * vl_bytes) as u64), // ADDVL
                0b011 => base.wrapping_add((imm6 * (vl_bytes / 8)) as u64), // ADDPL
                0b101 => (imm6 * vl_bytes) as u64,                    // RDVL (rn==31)
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            if rd == 31 {
                self.set_sp(val);
            } else {
                self.set_x(rd, val);
            }
            return Ok(CpuExit::Continue);
        }
        // Element-count forms: count = pattern_count(pat, esz) * (imm4 + 1).
        let esz = (insn >> 22) & 0x3;
        let esize_bits = 8u32 << esz; // 8/16/32/64
        let elements = (self.sve_vl as usize) / esize_bits as usize;
        let pattern = (insn >> 5) & 0x1F;
        let mul = (((insn >> 16) & 0xF) + 1) as u64;
        let count = sve_pattern_count(pattern, elements) as u64 * mul;
        match (insn >> 12) & 0xF {
            0b1110 => {
                if (insn >> 20) & 0x3 == 0b10 {
                    // CNT_r: Rd = count.
                    self.set_x(rd, count);
                } else {
                    // INCDEC_r: Rd = Xd +/- count (64-bit wrapping). d = bit10.
                    let cur = self.get_x(rd);
                    let v = if (insn >> 10) & 1 == 1 {
                        cur.wrapping_sub(count)
                    } else {
                        cur.wrapping_add(count)
                    };
                    self.set_x(rd, v);
                }
                Ok(CpuExit::Continue)
            }
            0b1111 => {
                // SINCDEC_r: saturating GPR. bits[21:20]==10 => 32-bit, 11 => 64.
                let sf64 = (insn >> 20) & 0x3 == 0b11;
                let dec = (insn >> 11) & 1 == 1;
                let uns = (insn >> 10) & 1 == 1;
                let cur = self.get_x(rd);
                let res = if sf64 {
                    sat_addsub_64(cur, count, uns, dec)
                } else {
                    sat_addsub_32(cur, count, uns, dec)
                };
                self.set_x(rd, res);
                Ok(CpuExit::Continue)
            }
            0b1100 => {
                // Vector inc/dec. Byte elements (esz==0) are unallocated.
                if esize_bits == 8 {
                    return Ok(CpuExit::Undefined(insn));
                }
                let esize = esize_bits as usize / 8;
                let a = self.v[rd as usize].to_le_bytes();
                let mut dst = a;
                let nlanes = 16 / esize;
                let sat = (insn >> 20) & 0x3 == 0b10; // SINCDEC_v vs INCDEC_v
                let mask = elem_mask(esize_bits);
                for e in 0..nlanes {
                    let off = e * esize;
                    let v = read_elem(&a, off, esize);
                    let r = if sat {
                        let dec = (insn >> 11) & 1 == 1;
                        let uns = (insn >> 10) & 1 == 1;
                        sat_addsub_elem(v, count, esize_bits, uns, dec)
                    } else {
                        let dec = (insn >> 10) & 1 == 1;
                        (if dec {
                            v.wrapping_sub(count)
                        } else {
                            v.wrapping_add(count)
                        }) & mask
                    };
                    write_elem(&mut dst, off, esize, r);
                }
                self.v[rd as usize] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }
            _ => Ok(CpuExit::Undefined(insn)),
        }
    }

    /// Execute SVE ZIP1/ZIP2/UZP1/UZP2/TRN1/TRN2 (unpredicated vector permute).
    /// At VL=128 these match the corresponding NEON permutes over the register.
    fn exec_sve_zip_uzp_trn(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        zm: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 10) & 0x7;
        let n = 16 / esize;
        let half = n / 2;
        let a = self.v[zn].to_le_bytes();
        let b = self.v[zm].to_le_bytes();
        let mut dst = [0u8; 16];
        let get = |buf: &[u8; 16], i: usize| read_elem(buf, i * esize, esize);
        for i in 0..half {
            let (lo, hi): (u64, u64) = match opc {
                0b000 => (get(&a, i), get(&b, i)),                 // ZIP1
                0b001 => (get(&a, half + i), get(&b, half + i)),   // ZIP2
                0b100 => (get(&a, 2 * i), get(&b, 2 * i)),         // TRN1
                0b101 => (get(&a, 2 * i + 1), get(&b, 2 * i + 1)), // TRN2
                _ => (0, 0),
            };
            match opc {
                0b000 | 0b001 | 0b100 | 0b101 => {
                    write_elem(&mut dst, (2 * i) * esize, esize, lo);
                    write_elem(&mut dst, (2 * i + 1) * esize, esize, hi);
                }
                _ => {}
            }
        }
        if opc == 0b010 || opc == 0b011 {
            // UZP1 (even) / UZP2 (odd): concatenated even/odd elements of Zn:Zm.
            let off = if opc == 0b011 { 1 } else { 0 };
            for i in 0..n {
                let v = if i < half {
                    get(&a, 2 * i + off)
                } else {
                    get(&b, 2 * (i - half) + off)
                };
                write_elem(&mut dst, i * esize, esize, v);
            }
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE TBL (table lookup, single source table). For each element e,
    /// `Zd[e] = Zn[Zm[e]]` if the index `Zm[e]` is within range, else 0. The
    /// table Zn is indexed by the unsigned element value of Zm.
    fn exec_sve_tbl(
        &mut self,
        zd: usize,
        zn: usize,
        zm: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        let elements = 16 / esize;
        let table = self.v[zn].to_le_bytes();
        let idxs = self.v[zm].to_le_bytes();
        let mut dst = [0u8; 16];
        for e in 0..elements {
            let idx = read_elem(&idxs, e * esize, esize) as usize;
            if idx < elements {
                let val = read_elem(&table, idx * esize, esize);
                write_elem(&mut dst, e * esize, esize, val);
            }
            // Out-of-range index leaves the destination element as 0.
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE TBX (table lookup with destination preservation). Like TBL,
    /// but an out-of-range index keeps the prior value of the destination
    /// element rather than zeroing it (so Zd is both source and destination).
    fn exec_sve_tbx(
        &mut self,
        zd: usize,
        zn: usize,
        zm: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        let elements = 16 / esize;
        let table = self.v[zn].to_le_bytes();
        let idxs = self.v[zm].to_le_bytes();
        let mut dst = self.v[zd].to_le_bytes(); // preserve existing Zd
        for e in 0..elements {
            let idx = read_elem(&idxs, e * esize, esize) as usize;
            if idx < elements {
                let val = read_elem(&table, idx * esize, esize);
                write_elem(&mut dst, e * esize, esize, val);
            }
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE DUP (indexed): broadcast element `index` of Zn to every lane
    /// of Zd. The esize and index are encoded in tsz:imm2 — the lowest set bit
    /// of tsz selects esize (8<<n bits), the remaining high bits give the index.
    /// An index past the end of the register broadcasts zero.
    fn exec_sve_dup_indexed(
        &mut self,
        insn: u32,
        zn: usize,
        zd: usize,
    ) -> Result<CpuExit, ArmError> {
        let imm2 = (insn >> 22) & 0x3;
        let tsz = (insn >> 16) & 0x1F;
        if tsz == 0 {
            return Ok(CpuExit::Undefined(insn));
        }
        let imm = (imm2 << 5) | tsz; // 7-bit imm2:tsz
        let tz = tsz.trailing_zeros(); // 0..=4
        let esize = 1usize << tz; // bytes: 1,2,4,8,16
        let index = (imm >> (tz + 1)) as usize;
        if esize == 16 {
            // Quadword element (VL=128 -> a single element): index 0 selects the
            // whole register, anything beyond broadcasts zero.
            self.v[zd] = if index == 0 { self.v[zn] } else { 0 };
            return Ok(CpuExit::Continue);
        }
        let elements = 16 / esize;
        let src = self.v[zn].to_le_bytes();
        let element = if index >= elements {
            0u64
        } else {
            read_elem(&src, index * esize, esize)
        };
        let mut dst = [0u8; 16];
        for e in 0..elements {
            write_elem(&mut dst, e * esize, esize, element);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE COMPACT: pack the active (per Pg) elements of Zn contiguously
    /// into the low elements of Zd, zeroing the remaining high elements. Only
    /// 32-bit (S) and 64-bit (D) element sizes are defined (esize = 32 << sz).
    fn exec_sve_compact(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        pg: usize,
    ) -> Result<CpuExit, ArmError> {
        let esize = 4usize << ((insn >> 22) & 1); // bytes: 4 (S) or 8 (D)
        let elements = 16 / esize;
        let pred = self.sve_p[pg];
        let src = self.v[zn].to_le_bytes();
        let mut dst = [0u8; 16];
        let mut x = 0;
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 1 {
                let val = read_elem(&src, e * esize, esize);
                write_elem(&mut dst, x * esize, esize, val);
                x += 1;
            }
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE SPLICE (destructive): copy the elements of Zdn spanning from
    /// the first to the last active element (inclusive, regardless of the
    /// predicate value of elements in between) into the low part of the result,
    /// then fill the remaining elements from the low elements of Zm. With no
    /// active element the result is Zm unchanged. `zd`=Zdn, `zn`=Zm.
    fn exec_sve_splice(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        pg: usize,
    ) -> Result<CpuExit, ArmError> {
        let esize = 1usize << ((insn >> 22) & 0x3); // bytes
        let elements = 16 / esize;
        let pred = self.sve_p[pg];
        let op1 = self.v[zd].to_le_bytes(); // Zdn
        let op2 = self.v[zn].to_le_bytes(); // Zm
        let mut dst = [0u8; 16];
        let mut x = 0usize;
        let mut lastnum: i32 = -1;
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 1 {
                lastnum = e as i32;
            }
        }
        if lastnum >= 0 {
            let mut active = false;
            for e in 0..=(lastnum as usize) {
                if (pred >> (e * esize)) & 1 == 1 {
                    active = true;
                }
                if active {
                    let val = read_elem(&op1, e * esize, esize);
                    write_elem(&mut dst, x * esize, esize, val);
                    x += 1;
                }
            }
        }
        // Fill the remaining (elements - x) destination slots from Zm's low part.
        for e in 0..(elements - x) {
            let val = read_elem(&op2, e * esize, esize);
            write_elem(&mut dst, x * esize, esize, val);
            x += 1;
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE permute operations (DUP, INDEX, REV, etc.).
    fn exec_sve_permute(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        zm: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        let op1 = (insn >> 23) & 0x3;
        let op3 = (insn >> 10) & 0x3F;

        match op1 {
            // INDEX
            0b11 if (insn >> 17) & 0xF == 0 => {
                let rn = ((insn >> 5) & 0x1F) as u8;
                let rm = ((insn >> 16) & 0x1F) as u8;
                let elements = 16 / esize;

                let start = self.get_x(rn) as i64;
                let incr = self.get_x(rm) as i64;

                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let val = start.wrapping_add((e as i64).wrapping_mul(incr));
                    let offset = e * esize;
                    match esize {
                        1 => dst[offset] = val as u8,
                        2 => {
                            let bytes = (val as u16).to_le_bytes();
                            dst[offset..offset + 2].copy_from_slice(&bytes);
                        }
                        4 => {
                            let bytes = (val as u32).to_le_bytes();
                            dst[offset..offset + 4].copy_from_slice(&bytes);
                        }
                        8 => {
                            let bytes = (val as u64).to_le_bytes();
                            dst[offset..offset + 8].copy_from_slice(&bytes);
                        }
                        _ => {}
                    }
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // DUP (scalar)
            0b10 if (op3 & 0x3E) == 0x20 => {
                let rn = ((insn >> 5) & 0x1F) as u8;
                let val = self.get_x(rn);
                let elements = 16 / esize;

                let mut dst = [0u8; 16];
                for e in 0..elements {
                    let offset = e * esize;
                    match esize {
                        1 => dst[offset] = val as u8,
                        2 => {
                            let bytes = (val as u16).to_le_bytes();
                            dst[offset..offset + 2].copy_from_slice(&bytes);
                        }
                        4 => {
                            let bytes = (val as u32).to_le_bytes();
                            dst[offset..offset + 4].copy_from_slice(&bytes);
                        }
                        8 => {
                            let bytes = val.to_le_bytes();
                            dst[offset..offset + 8].copy_from_slice(&bytes);
                        }
                        _ => {}
                    }
                }
                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // ZIP/UZP/TRN
            0b10 if (op3 & 0x30) == 0x00 => {
                let opc = (insn >> 10) & 0x7;
                let elements = 16 / esize;
                let src1 = self.v[zn].to_le_bytes();
                let src2 = self.v[zm].to_le_bytes();
                let mut dst = [0u8; 16];

                match opc {
                    // ZIP1 - interleave lower halves
                    0b000 => {
                        let half = elements / 2;
                        for e in 0..half {
                            for b in 0..esize {
                                dst[e * 2 * esize + b] = src1[e * esize + b];
                                dst[(e * 2 + 1) * esize + b] = src2[e * esize + b];
                            }
                        }
                    }
                    // ZIP2 - interleave upper halves
                    0b001 => {
                        let half = elements / 2;
                        for e in 0..half {
                            let src_off = (half + e) * esize;
                            for b in 0..esize {
                                dst[e * 2 * esize + b] = src1[src_off + b];
                                dst[(e * 2 + 1) * esize + b] = src2[src_off + b];
                            }
                        }
                    }
                    // UZP1 - even elements
                    0b010 => {
                        let half = elements / 2;
                        for e in 0..half {
                            for b in 0..esize {
                                dst[e * esize + b] = src1[e * 2 * esize + b];
                                dst[(half + e) * esize + b] = src2[e * 2 * esize + b];
                            }
                        }
                    }
                    // UZP2 - odd elements
                    0b011 => {
                        let half = elements / 2;
                        for e in 0..half {
                            for b in 0..esize {
                                dst[e * esize + b] = src1[(e * 2 + 1) * esize + b];
                                dst[(half + e) * esize + b] = src2[(e * 2 + 1) * esize + b];
                            }
                        }
                    }
                    // TRN1 - transpose even elements
                    0b100 => {
                        for e in 0..elements / 2 {
                            for b in 0..esize {
                                dst[e * 2 * esize + b] = src1[e * 2 * esize + b];
                                dst[(e * 2 + 1) * esize + b] = src2[e * 2 * esize + b];
                            }
                        }
                    }
                    // TRN2 - transpose odd elements
                    0b101 => {
                        for e in 0..elements / 2 {
                            for b in 0..esize {
                                dst[e * 2 * esize + b] = src1[(e * 2 + 1) * esize + b];
                                dst[(e * 2 + 1) * esize + b] = src2[(e * 2 + 1) * esize + b];
                            }
                        }
                    }
                    _ => {
                        return Err(ArmError::Unimplemented(format!(
                            "SVE ZIP/UZP/TRN opc={}",
                            opc
                        )));
                    }
                }

                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // REV
            0b10 if (op3 & 0x38) == 0x18 => {
                let elements = 16 / esize;
                let src = self.v[zn].to_le_bytes();
                let mut dst = [0u8; 16];

                for e in 0..elements {
                    let src_e = elements - 1 - e;
                    for b in 0..esize {
                        dst[e * esize + b] = src[src_e * esize + b];
                    }
                }

                self.v[zd] = u128::from_le_bytes(dst);
                Ok(CpuExit::Continue)
            }

            // RDVL - read vector length
            0b11 if (insn >> 17) & 0x1F == 0x1F && (op3 & 0x3E) == 0x10 => {
                let rd = (insn & 0x1F) as u8;
                let imm6 = ((insn >> 5) & 0x3F) as i64;
                let imm = if imm6 & 0x20 != 0 { imm6 | !0x3F } else { imm6 };
                // VL in bytes
                let vl_bytes = (self.sve_vl / 8) as i64;
                let result = (vl_bytes * imm) as u64;
                self.set_x(rd, result);
                Ok(CpuExit::Continue)
            }

            // CNTx - count elements
            0b11 if (insn >> 17) & 0x18 == 0x10 => {
                let rd = (insn & 0x1F) as u8;
                let opc = (insn >> 16) & 0x7;
                let pattern = (insn >> 5) & 0x1F;
                let imm4 = ((insn >> 16) & 0xF) as u64;

                let esize_bits = match opc {
                    0b000 => 8,  // CNTB
                    0b001 => 16, // CNTH
                    0b010 => 32, // CNTW
                    0b011 => 64, // CNTD
                    _ => 8,
                };

                let elements = (self.sve_vl as u64) / esize_bits;
                let count = match pattern {
                    0b11111 => elements, // ALL
                    _ => elements,
                };

                self.set_x(rd, count * imm4.max(1));
                Ok(CpuExit::Continue)
            }

            _ => Err(ArmError::Unimplemented(format!(
                "SVE permute op1={:02b} op3={:06b}",
                op1, op3
            ))),
        }
    }

    /// Execute SVE FP predicated operations.
    /// FEAT_SVE_B16B16 bf16 data-processing. Returns `Some(result)` if `insn` is
    /// a recognised bf16 op, else `None` (so the caller continues its normal
    /// f16/f32/f64 dispatch). All forms are 8-bit-exponent bf16 with FPCR-default
    /// (round-to-nearest, no flush, propagate-NaN) handling via `bf16_binop` /
    /// `bf16_fma`. The bf16 ops occupy the size==00 encoding slots (and, for the
    /// indexed forms, use bit22 as the high index bit).
    fn try_exec_sve_bf16(&mut self, insn: u32) -> Option<Result<CpuExit, ArmError>> {
        let top = (insn >> 24) & 0xFF;
        let zd = (insn & 0x1F) as usize;
        // ---- 0x65: unpredicated 3-same, predicated binary, predicated FMA ----
        if top == 0b01100101 && (insn >> 22) & 0x3 == 0b00 {
            let bit21 = (insn >> 21) & 1;
            // Unpredicated BFADD/BFSUB/BFMUL: bit21==0, bits[15:12]==0000,
            // opc=bits[11:10]. Zm=bits[20:16], Zn=bits[9:5].
            if bit21 == 0 && (insn >> 12) & 0xF == 0b0000 {
                let kind = match (insn >> 10) & 0x3 {
                    0b00 => FpKind::Add,
                    0b01 => FpKind::Sub,
                    0b10 => FpKind::Mul,
                    _ => return Some(Ok(CpuExit::Undefined(insn))),
                };
                let n = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
                let m = self.v[((insn >> 16) & 0x1F) as usize].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..8 {
                    let r = bf16_binop(
                        kind,
                        read_elem(&n, e * 2, 2) as u16,
                        read_elem(&m, e * 2, 2) as u16,
                    );
                    write_elem(&mut dst, e * 2, 2, r as u64);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                return Some(Ok(CpuExit::Continue));
            }
            // Predicated BFADD/.../BFMIN (merging): bit21==0, bits[15:13]==100,
            // opc=bits[20:16]. Zdn=Zd, Zm=bits[9:5], Pg=bits[12:10].
            if bit21 == 0 && (insn >> 13) & 0x7 == 0b100 {
                let kind = match (insn >> 16) & 0x1F {
                    0b00000 => FpKind::Add,
                    0b00001 => FpKind::Sub,
                    0b00010 => FpKind::Mul,
                    0b00100 => FpKind::MaxNm,
                    0b00101 => FpKind::MinNm,
                    0b00110 => FpKind::Max,
                    0b00111 => FpKind::Min,
                    _ => return Some(Ok(CpuExit::Undefined(insn))),
                };
                let pred = self.sve_p[((insn >> 10) & 0x7) as usize];
                let dn = self.v[zd].to_le_bytes();
                let m = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
                let mut dst = dn;
                for e in 0..8 {
                    if (pred >> (e * 2)) & 1 == 0 {
                        continue;
                    }
                    let r = bf16_binop(
                        kind,
                        read_elem(&dn, e * 2, 2) as u16,
                        read_elem(&m, e * 2, 2) as u16,
                    );
                    write_elem(&mut dst, e * 2, 2, r as u64);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                return Some(Ok(CpuExit::Continue));
            }
            // Predicated BFMLA (bit13==0) / BFMLS (bit13==1) (merging): bit21==1,
            // bits[15:14]==00. Zda=Zd, Zn=bits[9:5], Zm=bits[20:16], Pg=bits[12:10].
            if bit21 == 1 && (insn >> 14) & 0x3 == 0b00 {
                let sub = (insn >> 13) & 1 == 1;
                let pred = self.sve_p[((insn >> 10) & 0x7) as usize];
                let a = self.v[zd].to_le_bytes();
                let n = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
                let m = self.v[((insn >> 16) & 0x1F) as usize].to_le_bytes();
                let mut dst = a;
                for e in 0..8 {
                    if (pred >> (e * 2)) & 1 == 0 {
                        continue;
                    }
                    let r = bf16_fma(
                        read_elem(&a, e * 2, 2) as u16,
                        read_elem(&n, e * 2, 2) as u16,
                        read_elem(&m, e * 2, 2) as u16,
                        sub,
                    );
                    write_elem(&mut dst, e * 2, 2, r as u64);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                return Some(Ok(CpuExit::Continue));
            }
            return None;
        }
        // ---- 0x64: BFCLAMP and indexed BFMUL/BFMLA/BFMLS ----
        if top == 0b01100100 && (insn >> 23) & 1 == 0 && (insn >> 21) & 1 == 1 {
            let op6 = (insn >> 10) & 0x3F;
            // BFCLAMP (size==00): Zd = bf16 minnum(maxnum(Zn, Zd), Zm).
            if op6 == 0b001001 && (insn >> 22) & 0x3 == 0b00 {
                let n = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
                let m = self.v[((insn >> 16) & 0x1F) as usize].to_le_bytes();
                let d = self.v[zd].to_le_bytes();
                let mut dst = [0u8; 16];
                for e in 0..8 {
                    let lo = bf16_binop(
                        FpKind::MaxNm,
                        read_elem(&n, e * 2, 2) as u16,
                        read_elem(&d, e * 2, 2) as u16,
                    );
                    let r = bf16_binop(FpKind::MinNm, lo, read_elem(&m, e * 2, 2) as u16);
                    write_elem(&mut dst, e * 2, 2, r as u64);
                }
                self.v[zd] = u128::from_le_bytes(dst);
                return Some(Ok(CpuExit::Continue));
            }
            // Indexed BFMUL/BFMLA/BFMLS (.h): index=(bit22<<2)|bits[20:19],
            // Zm=bits[18:16], Zn=bits[9:5]. The index-th bf16 of each 128-bit
            // segment is the broadcast second factor (Zm.h[index] at VL=128).
            let (mul, sub) = match op6 {
                0b001010 => (true, false),  // BFMUL
                0b000010 => (false, false), // BFMLA
                0b000011 => (false, true),  // BFMLS
                _ => return None,
            };
            let index = ((((insn >> 22) & 1) << 2) | ((insn >> 19) & 0x3)) as usize;
            let n = self.v[((insn >> 5) & 0x1F) as usize].to_le_bytes();
            let m = self.v[((insn >> 16) & 0x7) as usize].to_le_bytes();
            let a = self.v[zd].to_le_bytes();
            let mb = read_elem(&m, index * 2, 2) as u16; // Zm.h[index]
            let mut dst = [0u8; 16];
            for e in 0..8 {
                let ne = read_elem(&n, e * 2, 2) as u16;
                let r = if mul {
                    bf16_binop(FpKind::Mul, ne, mb)
                } else {
                    bf16_fma(read_elem(&a, e * 2, 2) as u16, ne, mb, sub)
                };
                write_elem(&mut dst, e * 2, 2, r as u64);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Some(Ok(CpuExit::Continue));
        }
        None
    }

    fn exec_sve_fp_pred(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        zm: usize,
        pg: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        // FEAT_SVE_B16B16 bf16 arithmetic (BFADD/BFSUB/BFMUL/BFMAX/BFMIN/
        // BFMAXNM/BFMINNM, BFMLA/BFMLS, indexed BFMUL/BFMLA/BFMLS, BFCLAMP).
        // These use the size==00 encoding slots, distinct from the f16/f32/f64
        // ops handled below, so intercept them first.
        if let Some(r) = self.try_exec_sve_bf16(insn) {
            return r;
        }

        // SVE BFDOT (bf16 dot product, round-to-odd): 0x64, bits[23:22]==01,
        // bit21==1, bits[15:10]==100000 (zzzz) or 010000 (zzxw indexed). Each
        // f32 lane sums two bf16 products; the indexed form broadcasts Zm's
        // 32-bit group at `index` (bits[20:19], Zm in bits[18:16]).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 22) & 0x3 == 0b01
            && (insn >> 21) & 1 == 1
            && matches!((insn >> 10) & 0x3F, 0b100000 | 0b010000)
        {
            let indexed = (insn >> 10) & 0x3F == 0b010000;
            let (m, a) = (
                if indexed {
                    self.v[((insn >> 16) & 0x7) as usize]
                } else {
                    self.v[zm]
                },
                self.v[zd],
            );
            let n = self.v[zn];
            let m_idx = if indexed {
                (m >> (((insn >> 19) & 0x3) * 32)) as u32
            } else {
                0
            };
            let mut r = 0u128;
            for e in 0..4 {
                let m_pair = if indexed {
                    m_idx
                } else {
                    (m >> (e * 32)) as u32
                };
                let res = sve_bfdot_lane((a >> (e * 32)) as u32, (n >> (e * 32)) as u32, m_pair);
                r |= (res as u128) << (e * 32);
            }
            self.v[zd] = r;
            return Ok(CpuExit::Continue);
        }

        // SVE2.1 FDOT (FP 2-way dot product, f16 -> f32): 0x64, bits[23:22]==00,
        // bit21==1, bits[15:10]==100000 (zzzz) or 010000 (zzxz indexed). Each f32
        // lane sums two f16 products (single-rounded) into the accumulator; the
        // indexed form broadcasts Zm's 32-bit group at index=bits[20:19]
        // (Zm=bits[18:16]).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 22) & 0x3 == 0b00
            && (insn >> 21) & 1 == 1
            && matches!((insn >> 10) & 0x3F, 0b100000 | 0b010000)
        {
            let indexed = (insn >> 10) & 0x3F == 0b010000;
            let n = self.v[zn];
            let (m, a) = if indexed {
                (self.v[((insn >> 16) & 0x7) as usize], self.v[zd])
            } else {
                (self.v[zm], self.v[zd])
            };
            let m_idx = if indexed {
                (m >> (((insn >> 19) & 0x3) * 32)) as u32
            } else {
                0
            };
            let mut r = 0u128;
            for e in 0..4 {
                let m_pair = if indexed {
                    m_idx
                } else {
                    (m >> (e * 32)) as u32
                };
                let res = f16_dotadd((a >> (e * 32)) as u32, (n >> (e * 32)) as u32, m_pair);
                r |= (res as u128) << (e * 32);
            }
            self.v[zd] = r;
            return Ok(CpuExit::Continue);
        }

        // SVE2 FMLAL/FMLSL (f16) and BFMLALB/T, BFMLSLB/T (bf16) widening fused
        // multiply-add into f32: 0x64, bit21==1, bits[15:11]==10000 (add) or
        // 10100 (sub); bit10 picks the odd(T)/even(B) lane. bits[23:22] selects
        // the source format: 10=f16, 11=bf16 (the bf16 subtract form BFMLSL is
        // FEAT_SVE2p1). The narrow inputs are widened to f32 (bf16 by a 16-bit
        // left shift, f16 by the standard widening) and accumulated with a
        // single ARM-correct fused multiply-add (`fp_muladd_bits` = float32_
        // muladd, processing the addend NaN first); FMLSL/BFMLSL negate the Zn
        // input (the FPCR.AH=0 form, matching the oracle).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 21) & 1 == 1
            && matches!((insn >> 22) & 0x3, 0b10 | 0b11)
            && matches!((insn >> 11) & 0x1F, 0b10000 | 0b10100)
        {
            let bf = (insn >> 22) & 0x3 == 0b11;
            let sub = (insn >> 13) & 1 == 1;
            let top = (insn >> 10) & 1 == 1;
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zm].to_le_bytes();
            let acc = self.v[zd].to_le_bytes();
            let widen = |b: u16| -> u32 {
                if bf {
                    (b as u32) << 16
                } else {
                    Self::fp16_to_f32(b).to_bits()
                }
            };
            let mut dst = acc;
            for j in 0..4 {
                let h_off = (2 * j + top as usize) * 2;
                let nbits = read_elem(&n, h_off, 2) as u16 ^ if sub { 0x8000 } else { 0 };
                let nn = widen(nbits);
                let mm = widen(read_elem(&m, h_off, 2) as u16);
                let aa = read_elem(&acc, j * 4, 4) as u32;
                let r = fp_muladd_bits(aa as u64, nn as u64, mm as u64, 32) as u32;
                write_elem(&mut dst, j * 4, 4, r as u64);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE2 FMLAL/FMLSL (f16) and BFMLALB/T, BFMLSLB/T (bf16) by indexed
        // element: 0x64, bit21==1, bits[23:22]==10(f16)/11(bf16),
        // bits[15:14]==01, bit12==0. sub=bit13, T=bit10, Zm=bits[18:16],
        // index=(bits[20:19]<<1)|bit11. Like the non-indexed form but Zm.h[index]
        // is the broadcast second factor; the FMA uses the ARM-correct
        // float32_muladd (addend NaN processed first).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 21) & 1 == 1
            && matches!((insn >> 22) & 0x3, 0b10 | 0b11)
            && (insn >> 14) & 0x3 == 0b01
            && (insn >> 12) & 1 == 0
        {
            let bf = (insn >> 22) & 0x3 == 0b11;
            let sub = (insn >> 13) & 1 == 1; // FMLSL / BFMLSL
            let top = (insn >> 10) & 1 == 1; // odd half of Zn
            let index = ((((insn >> 19) & 0x3) << 1) | ((insn >> 11) & 1)) as usize;
            let zmr = ((insn >> 16) & 0x7) as usize;
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zmr].to_le_bytes();
            let acc = self.v[zd].to_le_bytes();
            let widen = |b: u16| -> u32 {
                if bf {
                    (b as u32) << 16
                } else {
                    Self::fp16_to_f32(b).to_bits()
                }
            };
            let mm = widen(read_elem(&m, index * 2, 2) as u16); // Zm.h[index]
            let mut dst = acc;
            for j in 0..4 {
                let h_off = (2 * j + top as usize) * 2;
                let nbits = read_elem(&n, h_off, 2) as u16 ^ if sub { 0x8000 } else { 0 };
                let nn = widen(nbits);
                let aa = read_elem(&acc, j * 4, 4) as u32;
                let r = fp_muladd_bits(aa as u64, nn as u64, mm as u64, 32) as u32;
                write_elem(&mut dst, j * 4, 4, r as u64);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE FMMLA / BFMMLA (FP matrix multiply-accumulate): 0x64, bit21==1,
        // bits[15:10]==111001. bits[23:22]: 01=BFMMLA, 10=FMMLA.s, 11=FMMLA.d.
        // The 2x2 f32 tile is N(row i) . M(row j) with plain (non-fused) mul/add.
        // FMMLA.d needs a 256-bit segment (VL >= 4*8 bytes), so at VL=128 it is
        // an unallocated encoding. BFMMLA reuses the NEON path (same semantics).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 21) & 1 == 1
            && (insn >> 10) & 0x3F == 0b111001
        {
            match (insn >> 22) & 0x3 {
                0b01 => return self.exec_simd_bfmmla(insn),
                0b10 => {
                    let (n, m, a) = (self.v[zn], self.v[zm], self.v[zd]);
                    let f = |v: u128, i: u32| f32::from_bits((v >> (i * 32)) as u32);
                    let (n00, n01, n10, n11) = (f(n, 0), f(n, 1), f(n, 2), f(n, 3));
                    let (m00, m01, m10, m11) = (f(m, 0), f(m, 1), f(m, 2), f(m, 3));
                    let d = [
                        f(a, 0) + (n00 * m00 + n01 * m01),
                        f(a, 1) + (n00 * m10 + n01 * m11),
                        f(a, 2) + (n10 * m00 + n11 * m01),
                        f(a, 3) + (n10 * m10 + n11 * m11),
                    ];
                    let mut r = 0u128;
                    for (i, v) in d.iter().enumerate() {
                        r |= (v.to_bits() as u128) << (i * 32);
                    }
                    self.v[zd] = r;
                    return Ok(CpuExit::Continue);
                }
                // FMMLA.d (esz=3): VL=128 < 4*8 bytes, so it is unallocated.
                _ => return Ok(CpuExit::Undefined(insn)),
            }
        }

        // SVE FCADD (FP complex add, predicated): 0x64, bits[21:17]==00000,
        // bits[15:13]==100. rot=bit16 (0=90,1=270). Per complex pair, merging:
        // re = Zdn_re + (rot? Zm_im : -Zm_im); im = Zdn_im + (rot? -Zm_re : Zm_re).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 17) & 0x1F == 0
            && (insn >> 13) & 0x7 == 0b100
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let bits = 8u32 << size;
            let esz = (bits / 8) as usize;
            let rot = (insn >> 16) & 1;
            let (dn, zmv) = (self.v[zd], self.v[zn]); // Zdn, Zm
            let pred = self.sve_p[pg];
            let mask = elem_mask(bits) as u128;
            let elem = |v: u128, idx: usize| ((v >> (idx * bits as usize)) & mask) as u64;
            let mut result = dn;
            for e in 0..(16 / (2 * esz)) {
                let (re, im) = (2 * e, 2 * e + 1);
                let (add_re, add_im) = if rot == 0 {
                    (fp_neg_bits(elem(zmv, im), bits), elem(zmv, re))
                } else {
                    (elem(zmv, im), fp_neg_bits(elem(zmv, re), bits))
                };
                if (pred >> (re * esz)) & 1 == 1 {
                    let r = fp_add_bits(elem(dn, re), add_re, bits) as u128 & mask;
                    result =
                        (result & !(mask << (re * bits as usize))) | (r << (re * bits as usize));
                }
                if (pred >> (im * esz)) & 1 == 1 {
                    let r = fp_add_bits(elem(dn, im), add_im, bits) as u128 & mask;
                    result =
                        (result & !(mask << (im * bits as usize))) | (r << (im * bits as usize));
                }
            }
            self.v[zd] = result;
            return Ok(CpuExit::Continue);
        }

        // SVE FCMLA (FP complex multiply-add, predicated): 0x64, bit21==0,
        // bit15==0. rot=bits[14:13]; Zn=bits[9:5], Zm=bits[20:16], Zda=Zd. Per
        // complex pair, merging, same operand selection as NEON FCMLA.
        if (insn >> 24) & 0xFF == 0b01100100 && (insn >> 21) & 1 == 0 && (insn >> 15) & 1 == 0 {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let bits = 8u32 << size;
            let esz = (bits / 8) as usize;
            let rot = (insn >> 13) & 0x3;
            let (n, mv, acc) = (self.v[zn], self.v[zm], self.v[zd]);
            let pred = self.sve_p[pg];
            let mask = elem_mask(bits) as u128;
            let elem = |v: u128, idx: usize| ((v >> (idx * bits as usize)) & mask) as u64;
            let mut result = acc;
            for e in 0..(16 / (2 * esz)) {
                let (re, im) = (2 * e, 2 * e + 1);
                let (a_re, a_im) = (elem(n, re), elem(n, im));
                let (b_re, b_im) = (elem(mv, re), elem(mv, im));
                let (xr, yr, xi, yi) = match rot {
                    0b00 => (a_re, b_re, a_re, b_im),
                    0b01 => (a_im, fp_neg_bits(b_im, bits), a_im, b_re),
                    0b10 => (a_re, fp_neg_bits(b_re, bits), a_re, fp_neg_bits(b_im, bits)),
                    _ => (a_im, b_im, a_im, fp_neg_bits(b_re, bits)),
                };
                if (pred >> (re * esz)) & 1 == 1 {
                    let r = fp_muladd_bits(elem(acc, re), xr, yr, bits) as u128 & mask;
                    result =
                        (result & !(mask << (re * bits as usize))) | (r << (re * bits as usize));
                }
                if (pred >> (im * esz)) & 1 == 1 {
                    let r = fp_muladd_bits(elem(acc, im), xi, yi, bits) as u128 & mask;
                    result =
                        (result & !(mask << (im * bits as usize))) | (r << (im * bits as usize));
                }
            }
            self.v[zd] = result;
            return Ok(CpuExit::Continue);
        }

        // SVE FP multiply / multiply-add by indexed element: 0x64, bit21==1,
        // bits[15:11]==00000 (FMLA=000000 / FMLS=000001) or bits[15:10]==001000
        // (FMUL). The indexed Zm element is broadcast. Size: bit23==0 -> .h
        // (fp16, bit22 is the index MSB), bits[23:22]==10 -> .s, ==11 -> .d.
        // FMLA/FMLS are fused; FMUL is a plain multiply (unpredicated).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 21) & 1 == 1
            && ((insn >> 11) & 0x1F == 0b00000 || (insn >> 10) & 0x3F == 0b001000)
        {
            let (esz, index, zmr): (usize, usize, usize) = if (insn >> 23) & 1 == 0 {
                // .h: index = bit22:bits[20:19], Zm = bits[18:16].
                let idx = (((insn >> 22) & 1) << 2) | ((insn >> 19) & 0x3);
                (2, idx as usize, ((insn >> 16) & 0x7) as usize)
            } else if (insn >> 22) & 1 == 0 {
                (
                    4,
                    ((insn >> 19) & 0x3) as usize,
                    ((insn >> 16) & 0x7) as usize,
                )
            } else {
                (
                    8,
                    ((insn >> 20) & 1) as usize,
                    ((insn >> 16) & 0xF) as usize,
                )
            };
            let ebits = (esz * 8) as u32;
            let is_fmul = (insn >> 10) & 0x3F == 0b001000;
            let is_fmls = !is_fmul && (insn >> 10) & 1 == 1;
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zmr].to_le_bytes();
            let acc = self.v[zd].to_le_bytes();
            let mm = read_elem(&m, index * esz, esz); // Zm[index]
            let mut dst = acc;
            for e in 0..(16 / esz) {
                let off = e * esz;
                let ne = read_elem(&n, off, esz);
                let r = if is_fmul {
                    match esz {
                        2 => fp16_mul(ne as u16, mm as u16) as u64,
                        4 => fp_three_same_f32(FpKind::Mul, ne as u32, mm as u32, 0) as u64,
                        _ => fp_three_same_f64(FpKind::Mul, ne, mm, 0),
                    }
                } else {
                    let nn = if is_fmls { fp_neg_bits(ne, ebits) } else { ne };
                    fp_muladd_bits(read_elem(&acc, off, esz), nn, mm, ebits)
                };
                write_elem(&mut dst, off, esz, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE FCMLA by indexed element: 0x64, bit21==1, bits[15:12]==0001.
        // rot=bits[11:10]; the indexed Zm complex pair (2*index) is broadcast.
        // Unpredicated, fused. .h: index=bits[20:19], Zm=bits[18:16]; .s:
        // index=bit20, Zm=bits[19:16]. Same flip/negate math as FCMLA.
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 21) & 1 == 1
            && (insn >> 12) & 0xF == 0b0001
        {
            let size = (insn >> 22) & 0x3;
            if size < 2 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esize = 1usize << (size - 1); // .h=2, .s=4
            let bits = (esize * 8) as u32;
            let mask = elem_mask(bits) as u128;
            let rot = (insn >> 10) & 0x3;
            let flip = rot & 1;
            let negf_imag = (rot >> 1) & 1;
            let negf_real = flip ^ negf_imag;
            let (index, zmr) = if size == 2 {
                (((insn >> 19) & 0x3) as usize, ((insn >> 16) & 0x7) as usize)
            } else {
                (((insn >> 20) & 1) as usize, ((insn >> 16) & 0xF) as usize)
            };
            let (n, mv, acc) = (self.v[zn], self.v[zmr], self.v[zd]);
            let elem = |v: u128, e: usize| ((v >> (e * bits as usize)) & mask) as u64;
            let (mr, mi) = (elem(mv, 2 * index), elem(mv, 2 * index + 1));
            let e1b = if flip == 1 { mi } else { mr };
            let e3b = if flip == 1 { mr } else { mi };
            let e1 = if negf_real == 1 {
                fp_neg_bits(e1b, bits)
            } else {
                e1b
            };
            let e3 = if negf_imag == 1 {
                fp_neg_bits(e3b, bits)
            } else {
                e3b
            };
            let mut result = acc;
            for p in 0..((16 / esize) / 2) {
                let (re, im) = (2 * p, 2 * p + 1);
                let e2 = if flip == 1 { elem(n, im) } else { elem(n, re) };
                let dr = fp_muladd_bits(elem(acc, re), e2, e1, bits) as u128 & mask;
                let di = fp_muladd_bits(elem(acc, im), e2, e3, bits) as u128 & mask;
                result = (result & !(mask << (re * bits as usize))) | (dr << (re * bits as usize));
                result = (result & !(mask << (im * bits as usize))) | (di << (im * bits as usize));
            }
            self.v[zd] = result;
            return Ok(CpuExit::Continue);
        }

        // SVE predicated FP fused multiply-add: 0x65, bit21==1. bits[14:13]
        // select FMLA(00)/FMLS(01)/FNMLA(10)/FNMLS(11); bit15 picks the form
        // (0: Zd is the addend Za, multiplicands Zn/Zm; 1: Zd is a multiplicand
        // Zdn with addend Za). neg_prod=bit13^bit14, neg_addend=bit14 (FPCR.AH=0
        // negates via the sign bit). Single fused multiply-add; merging.
        if (insn >> 24) & 0xFF == 0b01100101 && (insn >> 21) & 1 == 1 {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let ebits = (esz * 8) as u32;
            let neg_prod = ((insn >> 13) & 1) ^ ((insn >> 14) & 1) == 1;
            let neg_add = (insn >> 14) & 1 == 1;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let r95 = ((insn >> 5) & 0x1F) as usize;
            // mad form (bit15==1): Zdn=zd is a multiplicand, Zm=bits[9:5],
            // addend=bits[20:16]. else: Zn=bits[9:5], Zm=bits[20:16], addend=Za=zd.
            let (n_reg, m_reg, a_reg) = if (insn >> 15) & 1 == 1 {
                (zd, r95, rm)
            } else {
                (r95, rm, zd)
            };
            let pred = self.sve_p[pg];
            let nb = self.v[n_reg].to_le_bytes();
            let mb = self.v[m_reg].to_le_bytes();
            let ab = self.v[a_reg].to_le_bytes();
            let mut dst = self.v[zd].to_le_bytes();
            for e in 0..(16 / esz) {
                let off = e * esz;
                if (pred >> off) & 1 == 0 {
                    continue;
                }
                let mut n = read_elem(&nb, off, esz);
                if neg_prod {
                    n = fp_neg_bits(n, ebits);
                }
                let mut a = read_elem(&ab, off, esz);
                if neg_add {
                    a = fp_neg_bits(a, ebits);
                }
                let r = fp_muladd_bits(a, n, read_elem(&mb, off, esz), ebits);
                write_elem(&mut dst, off, esz, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE FRECPE / FRSQRTE (reciprocal / reciprocal-sqrt estimate,
        // unpredicated): 0x65, bits[21:16]==001110 (FRECPE) / 001111 (FRSQRTE),
        // bits[15:10]==001100. Reuses the FP estimate helpers.
        if (insn >> 24) & 0xFF == 0b01100101
            && matches!((insn >> 16) & 0x3F, 0b001110 | 0b001111)
            && (insn >> 10) & 0x3F == 0b001100
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let rsqrt = (insn >> 16) & 1 == 1;
            let n = self.v[zn].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..(16 / esz) {
                let off = e * esz;
                let x = read_elem(&n, off, esz);
                let r = match esz {
                    2 => {
                        (if rsqrt {
                            fp16_rsqrte(x as u16)
                        } else {
                            fp16_recpe(x as u16)
                        }) as u64
                    }
                    4 => {
                        (if rsqrt {
                            fp_rsqrt_estimate_f32(x as u32)
                        } else {
                            fp_recip_estimate_f32(x as u32)
                        }) as u64
                    }
                    _ => {
                        if rsqrt {
                            fp_rsqrt_estimate_f64(x)
                        } else {
                            fp_recip_estimate_f64(x)
                        }
                    }
                };
                write_elem(&mut dst, off, esz, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE FP compare (register) -> predicate: 0x65, bit21==0, the condition
        // is (bits[15:13], bit4): FCMGE/FCMGT/FCMEQ/FCMNE/FCMUO/FACGE/FACGT.
        // Zeroing under Pg; sets NZCV via PredTest(Pg).
        if (insn >> 24) & 0xFF == 0b01100101
            && (insn >> 21) & 1 == 0
            && matches!(
                ((insn >> 13) & 0x7, (insn >> 4) & 1),
                (0b010, 0)
                    | (0b010, 1)
                    | (0b011, 0)
                    | (0b011, 1)
                    | (0b110, 0)
                    | (0b110, 1)
                    | (0b111, 1)
            )
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let cc = ((insn >> 13) & 0x7, (insn >> 4) & 1);
            let pred = self.sve_p[pg];
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zm].to_le_bytes();
            let mut pd = 0u32;
            for e in 0..(16 / esz) {
                let off = e * esz;
                if (pred >> off) & 1 == 1
                    && sve_fp_compare(esz, cc, read_elem(&n, off, esz), read_elem(&m, off, esz))
                {
                    pd |= 1 << off;
                }
            }
            // FP compares write only the predicate; they do not set NZCV.
            self.sve_p[(insn & 0xF) as usize] = pd;
            return Ok(CpuExit::Continue);
        }

        // SVE FP compare with zero -> predicate: 0x65, bits[21:18]==0100,
        // bits[15:13]==001. bits[17:16] select GE/GT (00), LT/LE (01), EQ/NE
        // (10); bit4 picks within. Zeroing; sets NZCV via PredTest(Pg).
        if (insn >> 24) & 0xFF == 0b01100101
            && (insn >> 18) & 0xF == 0b0100
            && (insn >> 13) & 0x7 == 0b001
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let sub = (insn >> 16) & 0x3;
            let bit4 = (insn >> 4) & 1;
            // Valid: GE/GT (00,0/1), LT/LE (01,0/1), EQ (10,0), NE (11,0).
            // (10,1) and (11,1) are unallocated and fault on hardware.
            if (sub == 0b10 || sub == 0b11) && bit4 == 1 {
                return Ok(CpuExit::Undefined(insn));
            }
            let pred = self.sve_p[pg];
            let n = self.v[zn].to_le_bytes();
            let mut pd = 0u32;
            for e in 0..(16 / esz) {
                let off = e * esz;
                if (pred >> off) & 1 == 1
                    && sve_fp_compare_zero(esz, sub, bit4, read_elem(&n, off, esz))
                {
                    pd |= 1 << off;
                }
            }
            // FP compares write only the predicate; they do not set NZCV.
            self.sve_p[(insn & 0xF) as usize] = pd;
            return Ok(CpuExit::Continue);
        }

        // SVE FRECPS / FRSQRTS (reciprocal / reciprocal-sqrt step, unpredicated):
        // 0x65, bit21==0, bits[15:10]==000110 (FRECPS) / 000111 (FRSQRTS).
        // Fused step with the inf*0 special (2.0 / 1.5).
        if (insn >> 24) & 0xFF == 0b01100101
            && (insn >> 21) & 1 == 0
            && matches!((insn >> 10) & 0x3F, 0b000110 | 0b000111)
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let rsqrt = (insn >> 10) & 1 == 1;
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zm].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..(16 / esz) {
                let off = e * esz;
                let (x, y) = (read_elem(&n, off, esz), read_elem(&m, off, esz));
                let r = if rsqrt {
                    sve_rsqrts(esz, x, y)
                } else {
                    sve_recps(esz, x, y)
                };
                write_elem(&mut dst, off, esz, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE FTSMUL (trigonometric starting value): 0x65, bit21==0,
        // bits[15:10]==000011. result = Zn[e]^2 with sign from Zm[e] bit0.
        if (insn >> 24) & 0xFF == 0b01100101
            && (insn >> 21) & 1 == 0
            && (insn >> 10) & 0x3F == 0b000011
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zm].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..(16 / esz) {
                let off = e * esz;
                let r = sve_ftsmul(esz, read_elem(&n, off, esz), read_elem(&m, off, esz) & 1);
                write_elem(&mut dst, off, esz, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE FTMAD (trigonometric multiply-add coefficient): 0x65,
        // bits[21:19]==010, bits[15:10]==100000. Destructive: Zdn = fused(Zdn,
        // |Zm|, coeff[imm + 8*(Zm<0)]); Zm is at bits[9:5], imm at bits[18:16].
        if (insn >> 24) & 0xFF == 0b01100101
            && (insn >> 19) & 0x7 == 0b010
            && (insn >> 10) & 0x3F == 0b100000
        {
            let size = (insn >> 22) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let imm = ((insn >> 16) & 0x7) as usize;
            let dn = self.v[zd].to_le_bytes(); // Zdn
            let m = self.v[zn].to_le_bytes(); // Zm at bits[9:5]
            let mut dst = [0u8; 16];
            for e in 0..(16 / esz) {
                let off = e * esz;
                let r = sve_ftmad(esz, read_elem(&dn, off, esz), read_elem(&m, off, esz), imm);
                write_elem(&mut dst, off, esz, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE2.1 FCLAMP (FP clamp): 0x64, bit21==1, bits[15:10]==001001.
        // Zd = fminnum(fmaxnum(Zn, Zd), Zm) per element (Zd is the clamped
        // value). Must precede the reduce/unary dispatch (which keys on
        // bits[15:13], that FCLAMP's 001001 would otherwise hit as 001).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 21) & 1 == 1
            && (insn >> 10) & 0x3F == 0b001001
        {
            let n = self.v[zn].to_le_bytes();
            let m = self.v[zm].to_le_bytes();
            let d = self.v[zd].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..(16 / esize) {
                let off = e * esize;
                let lo = sve_fp_combine(
                    FpKind::MaxNm,
                    esize,
                    read_elem(&n, off, esize),
                    read_elem(&d, off, esize),
                );
                let r = sve_fp_combine(FpKind::MinNm, esize, lo, read_elem(&m, off, esize));
                write_elem(&mut dst, off, esize, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // SVE2.1 FP quadword reductions (FADDQV/FMAXNMQV/FMINNMQV/FMAXQV/
        // FMINQV): 0x64, bits[21:19]==010, bits[15:13]==101. opc=bits[18:16].
        // Each element position is reduced across the 128-bit segments seeded
        // with the op identity; at VL=128 an active lane is combined with the
        // identity (so FADD normalises -0.0/quiets NaN) and an inactive lane is
        // the raw identity. Must precede the unary dispatch (bits[15:13]==101).
        if (insn >> 24) & 0xFF == 0b01100100
            && (insn >> 19) & 0x7 == 0b010
            && (insn >> 13) & 0x7 == 0b101
        {
            let kind = match (insn >> 16) & 0x7 {
                0b000 => FpKind::Add,
                0b100 => FpKind::MaxNm,
                0b101 => FpKind::MinNm,
                0b110 => FpKind::Max,
                0b111 => FpKind::Min,
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            // At VL=128 each element position reduces a single-element column,
            // so an active lane is the raw Zn element (no combine with the
            // identity -> -0.0 and NaN payloads are preserved, matching hw); an
            // inactive lane is the op identity (the empty reduction).
            let ident = sve_fp_identity(kind, esize);
            let pred = self.sve_p[pg];
            let src = self.v[zn].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..(16 / esize) {
                let off = e * esize;
                let v = if (pred >> off) & 1 == 1 {
                    read_elem(&src, off, esize)
                } else {
                    ident
                };
                write_elem(&mut dst, off, esize, v);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // FP fast reductions / FADDA live at bits[15:13]==001; FP unary at
        // bits[15:13]==101; predicated binary arith at bits[15:13]==100.
        match (insn >> 13) & 0x7 {
            0b001 => return self.exec_sve_fp_reduce(insn, esize),
            0b101 => return self.exec_sve_fp_unary(insn, zd, zn, pg, esize),
            0b100 => {}
            _ => return Ok(CpuExit::Undefined(insn)),
        }
        // FP pairwise (FADDP/FMAXNMP/FMINNMP/FMAXP/FMINP): 0x64, bits[21:19]==010.
        // Interleaves the pairwise results of Zdn and Zm (even = Zdn pair, odd =
        // Zm pair), merged into Zdn under Pg. opc=bits[18:16].
        if (insn >> 24) & 0xFF == 0b01100100 && (insn >> 19) & 0x7 == 0b010 {
            let kind = match (insn >> 16) & 0x7 {
                0b000 => FpKind::Add,
                0b100 => FpKind::MaxNm,
                0b101 => FpKind::MinNm,
                0b110 => FpKind::Max,
                _ => FpKind::Min,
            };
            let pred = self.sve_p[pg];
            let elements = 16 / esize;
            let h = elements / 2;
            let dn = self.v[zd].to_le_bytes(); // Zdn
            let m = self.v[zn].to_le_bytes(); // Zm
            let mut res = [0u8; 16];
            for p in 0..h {
                let dnv = sve_fp_combine(
                    kind,
                    esize,
                    read_elem(&dn, 2 * p * esize, esize),
                    read_elem(&dn, (2 * p + 1) * esize, esize),
                );
                let mv = sve_fp_combine(
                    kind,
                    esize,
                    read_elem(&m, 2 * p * esize, esize),
                    read_elem(&m, (2 * p + 1) * esize, esize),
                );
                write_elem(&mut res, 2 * p * esize, esize, dnv);
                write_elem(&mut res, (2 * p + 1) * esize, esize, mv);
            }
            let mut dst = dn;
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 1 {
                    write_elem(
                        &mut dst,
                        e * esize,
                        esize,
                        read_elem(&res, e * esize, esize),
                    );
                }
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }
        let opc5 = (insn >> 16) & 0x1F;
        // FSCALE (opc5==01001): Zdn = Zdn * 2^(signed Zm element), merging. The
        // Zm element is a signed integer exponent, not a float.
        if opc5 == 0b01001 {
            let pred = self.sve_p[pg];
            let a = self.v[zd].to_le_bytes(); // Zdn
            let b = self.v[zn].to_le_bytes(); // Zm
            let mut dst = a;
            let ibits = (esize * 8) as u32;
            for e in 0..(16 / esize) {
                let off = e * esize;
                if (pred >> off) & 1 == 0 {
                    continue;
                }
                let n = sext_elem(read_elem(&b, off, esize), ibits) as i64;
                let r = sve_fscale(esize, read_elem(&a, off, esize), n);
                write_elem(&mut dst, off, esize, r);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }
        let (kind, swap) = match opc5 {
            0b00000 => (FpKind::Add, false),
            0b00001 => (FpKind::Sub, false),
            0b00010 => (FpKind::Mul, false),
            0b00011 => (FpKind::Sub, true), // FSUBR
            0b00100 => (FpKind::MaxNm, false),
            0b00101 => (FpKind::MinNm, false),
            0b00110 => (FpKind::Max, false),
            0b00111 => (FpKind::Min, false),
            0b01000 => (FpKind::Abd, false),
            0b01010 => (FpKind::Mulx, false), // FMULX
            0b01100 => (FpKind::Div, true),   // FDIVR
            0b01101 => (FpKind::Div, false),
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let a_reg = self.v[zd].to_le_bytes(); // Zdn (first source, dest)
        let b_reg = self.v[zn].to_le_bytes(); // Zm (second source)
        let mut dst = a_reg;
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 0 {
                continue;
            }
            let off = e * esize;
            let a = read_elem(&a_reg, off, esize);
            let b = read_elem(&b_reg, off, esize);
            let (x, y) = if swap { (b, a) } else { (a, b) };
            let r = match esize {
                2 => sve_fp16_binop(kind, x as u16, y as u16) as u64,
                4 => fp_three_same_f32(kind, x as u32, y as u32, 0) as u64,
                8 => fp_three_same_f64(kind, x, y, 0),
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE FP reduction to a scalar in Vd: the "fast" tree reductions
    /// FADDV/FMAXNMV/FMINNMV/FMAXV/FMINV (opc=bits[18:16]) and the strictly
    /// ordered FADDA (bits[20:16]==11000). Pg is byte-granular.
    fn exec_sve_fp_reduce(&mut self, insn: u32, esize: usize) -> Result<CpuExit, ArmError> {
        if esize < 2 {
            return Ok(CpuExit::Undefined(insn));
        }
        let pg = ((insn >> 10) & 0x7) as usize;
        let zn = ((insn >> 5) & 0x1F) as usize;
        let vd = (insn & 0x1F) as usize;
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let mask = elem_mask((esize * 8) as u32) as u128;
        // FADDA: strict left-to-right accumulate seeded by Vdn[0]; skip inactive.
        if (insn >> 16) & 0x1F == 0b11000 {
            let m_reg = self.v[zn].to_le_bytes(); // Zm
            let vd_bytes = self.v[vd].to_le_bytes();
            let mut acc = read_elem(&vd_bytes, 0, esize);
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 1 {
                    acc = sve_fp_combine(
                        FpKind::Add,
                        esize,
                        acc,
                        read_elem(&m_reg, e * esize, esize),
                    );
                }
            }
            self.v[vd] = (acc as u128) & mask;
            return Ok(CpuExit::Continue);
        }
        let kind = match (insn >> 16) & 0x7 {
            0b000 => FpKind::Add,   // FADDV
            0b100 => FpKind::MaxNm, // FMAXNMV
            0b101 => FpKind::MinNm, // FMINNMV
            0b110 => FpKind::Max,   // FMAXV
            0b111 => FpKind::Min,   // FMINV
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        let ident = sve_fp_identity(kind, esize);
        let src = self.v[zn].to_le_bytes();
        let buf: Vec<u64> = (0..elements)
            .map(|e| {
                if (pred >> (e * esize)) & 1 == 1 {
                    read_elem(&src, e * esize, esize)
                } else {
                    ident
                }
            })
            .collect();
        self.v[vd] = (sve_fp_tree_reduce(&buf, kind, esize) as u128) & mask;
        Ok(CpuExit::Continue)
    }

    /// Execute SVE predicated FP unary (merging): FSQRT, FRECPX and FRINT*
    /// (bits[20:16] selects the op). Inactive lanes keep their prior Zd value.
    fn exec_sve_fp_unary(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        pg: usize,
        esize: usize,
    ) -> Result<CpuExit, ArmError> {
        // FCVT (FP precision conversion): 0x65, bits[21:18]==0010. bits[23:22]
        // (opc) and bits[17:16] (opc2) select the src/dst float widths, NOT the
        // element size, so it bypasses the size-derived esize path entirely. The
        // 0x65 gate excludes the 0x64 FCVTNT/FCVTLT/FCVTXNT top/bottom variants.
        if (insn >> 24) & 0xFF == 0b01100101 && (insn >> 18) & 0xF == 0b0010 {
            return self.exec_sve_fcvt(insn, zd, zn, pg);
        }
        // FLOGB (find exponent): 0x65, bits[23:19]==0b00011, size in bits[18:17].
        // The element size is not bits[23:22] (those are 0), so it is computed
        // locally. Result is floor(log2|x|) as a signed integer, merging.
        if (insn >> 24) & 0xFF == 0b01100101 && (insn >> 19) & 0x1F == 0b00011 {
            let size = (insn >> 17) & 0x3;
            if size == 0 {
                return Ok(CpuExit::Undefined(insn));
            }
            let esz = 1usize << size;
            let pred = self.sve_p[pg];
            let src = self.v[zn].to_le_bytes();
            let mut dst = self.v[zd].to_le_bytes();
            let elements = 16 / esz;
            for e in 0..elements {
                let off = e * esz;
                if (pred >> off) & 1 == 0 {
                    continue;
                }
                let r = sve_flogb(esz, read_elem(&src, off, esz));
                write_elem(&mut dst, off, esz, r as u64);
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }
        // FP<->int conversions: 0x65, bits[21:19]==011 (FCVTZS/FCVTZU, FP->int)
        // or ==010 (SCVTF/UCVTF, int->FP). bits[23:22]/bits[18:17] pick the
        // widths and bit16 the signedness, so this also bypasses the esize path.
        // FLOGB (bits[23:22]==00) is intercepted above before reaching here.
        if (insn >> 24) & 0xFF == 0b01100101
            && ((insn >> 19) & 0x7 == 0b011 || (insn >> 19) & 0x7 == 0b010)
        {
            return self.exec_sve_fp_int_cvt(insn, zd, zn, pg);
        }
        // FCVTNT/FCVTXNT (narrow, top) and FCVTLT (long, top): 0x64,
        // bits[21:18]==0010. The wider element is the container; for narrowing
        // the converted result goes to the top (odd) half (bottom preserved),
        // for widening the source is read from the top (odd) half. FCVTXNT uses
        // round-to-odd. Predication is at the container (wider) granularity.
        if (insn >> 24) & 0xFF == 0b01100100 && (insn >> 18) & 0xF == 0b0010 {
            let opc = (insn >> 22) & 0x3;
            let opc2 = (insn >> 16) & 0x3;
            // (src,dst,round_odd,narrow,bf): bf marks the bf16 destination
            // (BFCVTNT), distinguished from FCVTNT s->h only by opc2 (10 vs 00).
            let (src_sz, dst_sz, round_odd, narrow, bf): (usize, usize, bool, bool, bool) =
                match (opc, opc2) {
                    (0b00, 0b10) => (8, 4, true, true, false),  // FCVTXNT d->s
                    (0b10, 0b00) => (4, 2, false, true, false), // FCVTNT  s->h
                    (0b10, 0b10) => (4, 2, false, true, true),  // BFCVTNT s->bf16
                    (0b11, 0b10) => (8, 4, false, true, false), // FCVTNT  d->s
                    (0b10, 0b01) => (2, 4, false, false, false), // FCVTLT h->s
                    (0b11, 0b11) => (4, 8, false, false, false), // FCVTLT s->d
                    _ => return Ok(CpuExit::Undefined(insn)),
                };
            let cont = src_sz.max(dst_sz);
            let elements = 16 / cont;
            let pred = self.sve_p[pg];
            let operand = self.v[zn].to_le_bytes();
            let mut dst = self.v[zd].to_le_bytes();
            for c in 0..elements {
                let coff = c * cont;
                if (pred >> coff) & 1 == 0 {
                    continue;
                }
                let convert = |x: u64| -> u64 {
                    match (src_sz, dst_sz, round_odd) {
                        (4, 2, _) if bf => f32_to_bf16(x as u32) as u64,
                        (4, 2, _) => Self::f32_to_fp16(f32::from_bits(x as u32)) as u64,
                        (8, 4, false) => (f64::from_bits(x) as f32).to_bits() as u64,
                        (8, 4, true) => round_odd_f64_to_f32(f64::from_bits(x)) as u64,
                        (2, 4, _) => Self::fp16_to_f32(x as u16).to_bits() as u64,
                        _ => (f32::from_bits(x as u32) as f64).to_bits(),
                    }
                };
                if narrow {
                    let res = convert(read_elem(&operand, coff, src_sz));
                    write_elem(&mut dst, coff + dst_sz, dst_sz, res); // top half
                } else {
                    let res = convert(read_elem(&operand, coff + src_sz, src_sz)); // top half
                    write_elem(&mut dst, coff, dst_sz, res);
                }
            }
            self.v[zd] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }
        if esize < 2 {
            return Ok(CpuExit::Undefined(insn));
        }
        let b20_16 = (insn >> 16) & 0x1F;
        // FRINT* rounding -> (TwoRegFp variant, fp16 mode).
        let rint = |m: u32| -> Option<(TwoRegFp, u8)> {
            Some(match m {
                0b000 => (TwoRegFp::RintN, 0),
                0b001 => (TwoRegFp::RintP, 2),
                0b010 => (TwoRegFp::RintM, 1),
                0b011 => (TwoRegFp::RintZ, 3),
                0b100 => (TwoRegFp::RintA, 4),
                0b110 => (TwoRegFp::RintX, 0),
                0b111 => (TwoRegFp::RintI, 0),
                _ => return None,
            })
        };
        let pred = self.sve_p[pg];
        let elements = 16 / esize;
        let src = self.v[zn].to_le_bytes();
        let mut dst = self.v[zd].to_le_bytes(); // merging: start from Zd
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 == 0 {
                continue;
            }
            let off = e * esize;
            let lane = read_elem(&src, off, esize);
            let r = match b20_16 {
                0b01101 => match esize {
                    2 => fp16_sqrt(lane as u16) as u64,
                    4 => fp_two_reg_f32(TwoRegFp::Fsqrt, lane as u32) as u64,
                    _ => fp_two_reg_f64(TwoRegFp::Fsqrt, lane),
                },
                0b01100 => sve_fp_recpx(esize, lane),
                m if m < 0b01000 => {
                    let Some((trk, fp16m)) = rint(m) else {
                        return Ok(CpuExit::Undefined(insn));
                    };
                    match esize {
                        2 => fp16_frint(lane as u16, fp16m) as u64,
                        4 => fp_two_reg_f32(trk, lane as u32) as u64,
                        _ => fp_two_reg_f64(trk, lane),
                    }
                }
                _ => return Ok(CpuExit::Undefined(insn)),
            };
            write_elem(&mut dst, off, esize, r);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE FCVT (predicated FP precision conversion between fp16/fp32/
    /// fp64). The per-element container size is the larger of the source and
    /// destination widths; the source value occupies the low bits of its
    /// container and the (zero-extended) result is written back. Predication is
    /// byte-granular at the container size and merges (inactive lanes keep Zd).
    fn exec_sve_fcvt(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        pg: usize,
    ) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 22) & 0x3;
        let opc2 = (insn >> 16) & 0x3;
        // round_odd marks FCVTX (double->single, round-to-odd) which shares the
        // (8,4) widths with regular FCVT double->single but uses RO rounding.
        let (src_sz, dst_sz, round_odd, bf): (usize, usize, bool, bool) = match (opc, opc2) {
            (0b10, 0b01) => (2, 4, false, false), // half   -> single
            (0b11, 0b01) => (2, 8, false, false), // half   -> double
            (0b10, 0b00) => (4, 2, false, false), // single -> half
            (0b11, 0b11) => (4, 8, false, false), // single -> double
            (0b11, 0b00) => (8, 2, false, false), // double -> half
            (0b11, 0b10) => (8, 4, false, false), // double -> single
            (0b00, 0b10) => (8, 4, true, false),  // FCVTX  double -> single (round-to-odd)
            (0b10, 0b10) => (4, 2, false, true),  // BFCVT  single -> bf16
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        let cont = src_sz.max(dst_sz);
        let elements = 16 / cont;
        let pred = self.sve_p[pg];
        let operand = self.v[zn].to_le_bytes();
        let mut dst = self.v[zd].to_le_bytes(); // merging: start from Zd
        for e in 0..elements {
            let off = e * cont;
            if (pred >> off) & 1 == 0 {
                continue;
            }
            let x = read_elem(&operand, off, src_sz);
            let res = match (src_sz, dst_sz) {
                (2, 4) => Self::fp16_to_f32(x as u16).to_bits() as u64,
                (2, 8) => fp16_to_f64(x as u16).to_bits(),
                (4, 2) if bf => f32_to_bf16(x as u32) as u64, // BFCVT
                (4, 2) => Self::f32_to_fp16(f32::from_bits(x as u32)) as u64,
                (4, 8) => (f32::from_bits(x as u32) as f64).to_bits(),
                (8, 2) => fp16_round(f64::from_bits(x)) as u64,
                _ if round_odd => round_odd_f64_to_f32(f64::from_bits(x)) as u64, // FCVTX
                _ => (f64::from_bits(x) as f32).to_bits() as u64, // double -> single
            };
            write_elem(&mut dst, off, cont, res);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE FCVTZS/FCVTZU (FP -> integer, round toward zero, saturating)
    /// and SCVTF/UCVTF (integer -> FP, round to nearest even). The per-element
    /// container is the larger of the FP and integer widths; the source occupies
    /// the low bits of its container and the result is zero-extended back.
    /// Predication is byte-granular at the container size and merges.
    fn exec_sve_fp_int_cvt(
        &mut self,
        insn: u32,
        zd: usize,
        zn: usize,
        pg: usize,
    ) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 22) & 0x3;
        let opc2 = (insn >> 17) & 0x3;
        let signed = (insn >> 16) & 1 == 0; // int_U: 0=signed, 1=unsigned
        let to_int = (insn >> 19) & 0x7 == 0b011; // FCVTZ; else SCVTF/UCVTF
        let (fp_sz, int_sz): (usize, usize) = match (opc, opc2) {
            (0b01, 0b01) => (2, 2), // fp16 <-> int16
            (0b01, 0b10) => (2, 4), // fp16 <-> int32
            (0b01, 0b11) => (2, 8), // fp16 <-> int64
            (0b10, 0b10) => (4, 4), // f32  <-> int32
            (0b11, 0b00) => (8, 4), // f64  <-> int32
            (0b11, 0b10) => (4, 8), // f32  <-> int64
            (0b11, 0b11) => (8, 8), // f64  <-> int64
            _ => return Ok(CpuExit::Undefined(insn)),
        };
        let cont = fp_sz.max(int_sz);
        let elements = 16 / cont;
        let pred = self.sve_p[pg];
        let operand = self.v[zn].to_le_bytes();
        let mut dst = self.v[zd].to_le_bytes(); // merging: start from Zd
        for e in 0..elements {
            let off = e * cont;
            if (pred >> off) & 1 == 0 {
                continue;
            }
            let res = if to_int {
                sve_fcvtz(fp_sz, int_sz, signed, read_elem(&operand, off, fp_sz))
            } else {
                sve_cvtf(int_sz, fp_sz, signed, read_elem(&operand, off, int_sz))
            };
            write_elem(&mut dst, off, cont, res);
        }
        self.v[zd] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    /// Execute SVE load/store instructions. Currently models the contiguous
    /// LD1{B,H,W,D}/LD1S{B,H,W} and ST1{B,H,W,D} forms with a scalar base plus a
    /// VL-scaled immediate (the `_Z.P.BI_` encodings). Predication is
    /// byte-granular; loads zero inactive elements, stores skip them.
    fn exec_sve_ldst(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // SVE prefetch (PRF*) instructions are architectural hints with no
        // register or memory effect -> no-op. They share the load/store space;
        // detect the PRF encodings (all have bit4==0) per the ARM decode.
        if (insn >> 4) & 1 == 0 {
            let b3125 = (insn >> 25) & 0x7F;
            let (b2423, b2221, b1513) =
                ((insn >> 23) & 0x3, (insn >> 21) & 0x3, (insn >> 13) & 0x7);
            let is_prf = if b3125 == 0b1000010 {
                (b2423 == 0 && (insn >> 21) & 1 == 1 && (insn >> 15) & 1 == 0)
                    || (b2221 == 0 && b1513 == 0b111)
                    || (b2423 == 0b11 && (insn >> 22) & 1 == 1 && (insn >> 15) & 1 == 0)
                    || (b2221 == 0 && b1513 == 0b110)
            } else if b3125 == 0b1100010 {
                (b2423 == 0 && b2221 == 0b11 && (insn >> 15) & 1 == 1)
                    || (b2423 == 0 && (insn >> 21) & 1 == 1 && (insn >> 15) & 1 == 0)
                    || (b2221 == 0 && b1513 == 0b111)
            } else {
                false
            };
            if is_prf {
                return Ok(CpuExit::Continue);
            }
        }
        let pg = ((insn >> 10) & 0x7) as usize;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let zt = (insn & 0x1F) as usize;
        let imm4 = ((((insn >> 16) & 0xF) as i32) << 28 >> 28) as i64; // signed 4-bit
        let pred = self.sve_p[pg];
        let base = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };
        let is_store = (insn >> 30) & 1 == 1;
        let b15_13 = (insn >> 13) & 0x7;
        // imm9 = SInt(imm9h:imm9l) for the whole-register LDR/STR forms.
        let imm9 = (((((insn >> 16) & 0x3F) << 3) | ((insn >> 10) & 0x7)) as i32) << 23 >> 23;
        let imm9 = imm9 as i64;

        // LDR/STR whole-register fill/spill (unpredicated). Zt loads/stores the
        // full VL/8 (=16) bytes; Pt loads/stores PL/8 (=2) bytes. bits[15:13]:
        // 010 = vector register, 000 = predicate register. The immediate is
        // scaled by the register's byte size.
        if insn >> 22 == 0b1000010110 && b15_13 == 0b010 {
            let addr = (base as i64 + imm9 * 16) as u64;
            let mut bytes = [0u8; 16];
            for (i, b) in bytes.iter_mut().enumerate() {
                *b = self.memory.read_u8(self.translate_address(
                    addr + i as u64,
                    false,
                    false,
                )?)?;
            }
            self.v[zt] = u128::from_le_bytes(bytes);
            return Ok(CpuExit::Continue);
        }
        if insn >> 22 == 0b1110010110 && b15_13 == 0b010 {
            let addr = (base as i64 + imm9 * 16) as u64;
            let bytes = self.v[zt].to_le_bytes();
            for (i, b) in bytes.iter().enumerate() {
                self.memory
                    .write_u8(self.translate_address(addr + i as u64, true, false)?, *b)?;
            }
            return Ok(CpuExit::Continue);
        }
        if insn >> 22 == 0b1000010110 && b15_13 == 0b000 {
            let pt = (insn & 0xF) as usize;
            let addr = (base as i64 + imm9 * 2) as u64;
            let b0 = self
                .memory
                .read_u8(self.translate_address(addr, false, false)?)? as u32;
            let b1 = self
                .memory
                .read_u8(self.translate_address(addr + 1, false, false)?)?
                as u32;
            self.sve_p[pt] = b0 | (b1 << 8);
            return Ok(CpuExit::Continue);
        }
        if insn >> 22 == 0b1110010110 && b15_13 == 0b000 {
            let pt = (insn & 0xF) as usize;
            let addr = (base as i64 + imm9 * 2) as u64;
            let p = self.sve_p[pt];
            self.memory
                .write_u8(self.translate_address(addr, true, false)?, p as u8)?;
            self.memory.write_u8(
                self.translate_address(addr + 1, true, false)?,
                (p >> 8) as u8,
            )?;
            return Ok(CpuExit::Continue);
        }

        // LD1R (load and replicate): 1000010 dtypeh 1 imm6 1 dtypel Pg Rn Zt.
        // Reads one element at base + imm6*mbytes, extends it to the element
        // width and broadcasts it to every active lane (zeroing the inactive).
        if insn >> 25 == 0b1000010 && (insn >> 22) & 1 == 1 && (insn >> 15) & 1 == 1 {
            let dtype = (((insn >> 23) & 0x3) << 2) | ((insn >> 13) & 0x3);
            let (esize, mbytes, signed) = sve_ld1_dtype(dtype);
            let imm6 = (insn >> 16) & 0x3F; // unsigned
            let elements = 16 / esize;
            let addr = base + (imm6 as u64) * (mbytes as u64);
            let any_active = (0..elements).any(|e| (pred >> (e * esize)) & 1 == 1);
            let val = if any_active {
                let pa = self.translate_address(addr, false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                if signed {
                    (sext_elem(raw, (mbytes * 8) as u32) as u64) & elem_mask((esize * 8) as u32)
                } else {
                    raw
                }
            } else {
                0
            };
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 1 {
                    write_elem(&mut dst, e * esize, esize, val);
                }
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // Contiguous LD1 (scalar + immediate): 1010010 dtype 0 imm4 101 Pg Rn Zt.
        if !is_store && insn >> 25 == 0b1010010 && b15_13 == 0b101 && (insn >> 20) & 1 == 0 {
            let dtype = (insn >> 21) & 0xF;
            let (esize, mbytes, signed) = sve_ld1_dtype(dtype);
            let elements = 16 / esize;
            let addr0 = (base as i64 + imm4 * (elements * mbytes) as i64) as u64;
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue; // inactive -> zero (LD1 is zeroing)
                }
                let ea = addr0 + (e * mbytes) as u64;
                let pa = self.translate_address(ea, false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                let val = if signed {
                    (sext_elem(raw, (mbytes * 8) as u32) as u64) & elem_mask((esize * 8) as u32)
                } else {
                    raw
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // Contiguous ST1 (scalar + immediate): 1110010 msz size 0 imm4 111 Pg Rn Zt.
        // msz=bits[24:23] memory width, size=bits[22:21] element width (>= msz).
        if is_store && insn >> 25 == 0b1110010 && b15_13 == 0b111 && (insn >> 20) & 1 == 0 {
            let msz = (insn >> 23) & 0x3;
            let size = (insn >> 21) & 0x3;
            if size < msz {
                return Ok(CpuExit::Undefined(insn)); // element must be >= memory size
            }
            let esize = 1usize << size;
            let mbytes = 1usize << msz;
            let elements = 16 / esize;
            let addr0 = (base as i64 + imm4 * (elements * mbytes) as i64) as u64;
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue; // inactive -> leave memory unchanged
                }
                let ea = addr0 + (e * mbytes) as u64;
                let pa = self.translate_address(ea, true, false)?;
                let val = read_elem(&src, e * esize, esize); // low msize bytes stored
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    4 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // LD1 (scalar + scalar register offset): 1010010 dtype Rm 010 Pg Rn Zt.
        // addr = base + (Xm + e) * mbytes. Rm==31 is UNDEFINED.
        if !is_store && insn >> 25 == 0b1010010 && b15_13 == 0b010 {
            let rm = ((insn >> 16) & 0x1F) as u8;
            if rm == 31 {
                return Ok(CpuExit::Undefined(insn));
            }
            let dtype = (insn >> 21) & 0xF;
            let (esize, mbytes, signed) = sve_ld1_dtype(dtype);
            let elements = 16 / esize;
            let addr0 = base.wrapping_add(self.get_x(rm).wrapping_mul(mbytes as u64));
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let pa = self.translate_address(addr0 + (e * mbytes) as u64, false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                let val = if signed {
                    (sext_elem(raw, (mbytes * 8) as u32) as u64) & elem_mask((esize * 8) as u32)
                } else {
                    raw
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // LDFF1 (first-fault contiguous, scalar+scalar): 1010010 dtype Rm 011 Pg
        // Rn Zt. Like LD1 (addr=base+(Xm+e)*mbytes) but the first active element
        // faults normally while later elements are suppressed (FFR cleared).
        if !is_store && insn >> 25 == 0b1010010 && b15_13 == 0b011 {
            let dtype = (insn >> 21) & 0xF;
            let (esize, mbytes, signed) = sve_ld1_dtype(dtype);
            let elements = 16 / esize;
            let rm = ((insn >> 16) & 0x1F) as u8;
            let addr0 = base.wrapping_add(self.get_x(rm).wrapping_mul(mbytes as u64));
            return self.exec_sve_ff_load(addr0, mbytes, esize, signed, elements, pred, zt, false);
        }

        // LDNF1 (non-fault contiguous, scalar+imm): 1010010 dtype 1 imm4 101 Pg
        // Rn Zt (bit20==1 separates it from LD1's bit20==0). No access faults;
        // any element that would fault is suppressed (FFR cleared).
        if !is_store && insn >> 25 == 0b1010010 && b15_13 == 0b101 && (insn >> 20) & 1 == 1 {
            let dtype = (insn >> 21) & 0xF;
            let (esize, mbytes, signed) = sve_ld1_dtype(dtype);
            let elements = 16 / esize;
            let addr0 = (base as i64 + imm4 * (elements * mbytes) as i64) as u64;
            return self.exec_sve_ff_load(addr0, mbytes, esize, signed, elements, pred, zt, true);
        }

        // ST1 (scalar + scalar register offset): 1110010 msz size Rm 010 Pg Rn Zt.
        if is_store && insn >> 25 == 0b1110010 && b15_13 == 0b010 {
            let rm = ((insn >> 16) & 0x1F) as u8;
            if rm == 31 {
                return Ok(CpuExit::Undefined(insn));
            }
            let msz = (insn >> 23) & 0x3;
            let size = (insn >> 21) & 0x3;
            if size < msz {
                return Ok(CpuExit::Undefined(insn));
            }
            let esize = 1usize << size;
            let mbytes = 1usize << msz;
            let elements = 16 / esize;
            let addr0 = base.wrapping_add(self.get_x(rm).wrapping_mul(mbytes as u64));
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let pa = self.translate_address(addr0 + (e * mbytes) as u64, true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    4 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // LD1 gather (64-bit scalar base + vector offset, D elements):
        // 1100010 msz ig1 Zm 1 U ff Pg Rn Zt. esize=64; addr[e] = Xn +
        // (Zm[e] << scale); scale = msz when scaled (bits[22:21]==11) else 0;
        // load msize bytes and sign(U=0)/zero(U=1)-extend; inactive lanes zero.
        // bit22==1 (ig1 high) separates it from the vector-base form (ig1==01).
        // ff=bit13 is free: ff=1 is the first-fault LDFF1 gather, which on the
        // (untestable in this harness) fault path would suppress + clear FFR;
        // the no-fault path is identical to the plain gather modelled here.
        if insn >> 25 == 0b1100010 && (insn >> 22) & 1 == 1 && (insn >> 15) & 1 == 1 {
            let msz = (insn >> 23) & 0x3;
            let scaled = (insn >> 21) & 0x3 == 0b11;
            let unsigned = (insn >> 14) & 1 == 1;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let scale = if scaled { msz } else { 0 };
            let esize = 8usize; // D
            let elements = 16 / esize;
            let offs = self.v[zm].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let off = read_elem(&offs, e * esize, esize); // 64-bit unsigned offset
                let pa = self.translate_address(base.wrapping_add(off << scale), false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                let val = if unsigned {
                    raw
                } else {
                    sext_elem(raw, (mbytes * 8) as u32) as u64
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // LD1 gather (unpacked: D elements, 32-bit vector offset): 1100010 msz
        // xs scaled Zm 0 U ff Pg Rn Zt (bit15==0 vs the D.64 form's bit15==1).
        // esize=64; offset[e] = extend(Zm[e]<31:0>, xs) << scale. ff=bit13 free
        // (first-fault variant; no-fault path identical).
        if insn >> 25 == 0b1100010 && (insn >> 15) & 1 == 0 {
            let msz = (insn >> 23) & 0x3;
            let xs_signed = (insn >> 22) & 1 == 1;
            let scaled = (insn >> 21) & 1 == 1;
            let unsigned = (insn >> 14) & 1 == 1;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let scale = if scaled { msz } else { 0 };
            let esize = 8usize; // D
            let elements = 16 / esize;
            let offs = self.v[zm].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let off32 = read_elem(&offs, e * esize, 4) as u32; // low 32 bits
                let off = if xs_signed {
                    off32 as i32 as i64 as u64
                } else {
                    off32 as u64
                };
                let pa = self.translate_address(base.wrapping_add(off << scale), false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                let val = if unsigned {
                    raw
                } else {
                    sext_elem(raw, (mbytes * 8) as u32) as u64
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // LD1 gather (32-bit scalar base + vector offset, S elements): 1000010
        // msz xs scaled Zm 0 U ff Pg Rn Zt. esize=32; offset[e] = extend(Zm[e]
        // <31:0>, xs) << scale (xs=1 SXTW signed, 0 UXTW unsigned). Checked after
        // LDR/STR/LD1R (which share the 1000010 prefix but have bits[24:23]==11
        // or bit15==1), so those win first for their encodings.
        if insn >> 25 == 0b1000010 && (insn >> 15) & 1 == 0 {
            let msz = (insn >> 23) & 0x3;
            if msz == 3 {
                return Ok(CpuExit::Undefined(insn)); // no doubleword in S-form
            }
            let xs_signed = (insn >> 22) & 1 == 1;
            let scaled = (insn >> 21) & 1 == 1;
            let unsigned = (insn >> 14) & 1 == 1;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let scale = if scaled { msz } else { 0 };
            let esize = 4usize; // S
            let elements = 16 / esize;
            let offs = self.v[zm].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let off32 = read_elem(&offs, e * esize, esize) as u32;
                let off = if xs_signed {
                    off32 as i32 as i64 as u64
                } else {
                    off32 as u64
                };
                let pa = self.translate_address(base.wrapping_add(off << scale), false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    _ => self.memory.read_u32(pa)? as u64,
                };
                let val = if unsigned {
                    raw
                } else {
                    (sext_elem(raw, (mbytes * 8) as u32) as u64) & elem_mask(32)
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // ST1 scatter (64-bit scalar base + vector offset, D elements):
        // 1110010 msz ig1 Zm 101 Pg Rn Zt. addr[e] = Xn + (Zm[e] << scale);
        // scale = msz when scaled (bits[22:21]==01) else 0; store the low msize
        // bytes of each active D element (inactive lanes leave memory unchanged).
        // bit22==0 separates it from the vector-base scatter (ig1==10).
        if insn >> 25 == 0b1110010 && (insn >> 22) & 1 == 0 && (insn >> 13) & 0x7 == 0b101 {
            let msz = (insn >> 23) & 0x3;
            let scaled = (insn >> 21) & 0x3 == 0b01;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let scale = if scaled { msz } else { 0 };
            let esize = 8usize; // D
            let elements = 16 / esize;
            let offs = self.v[zm].to_le_bytes();
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let off = read_elem(&offs, e * esize, esize);
                let pa = self.translate_address(base.wrapping_add(off << scale), true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    4 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // ST1 scatter (32-bit scalar base + vector offset, S elements): 1110010
        // msz ig1 Zm 1 xs 0 Pg Rn Zt. esize=32; offset[e] = extend(Zm[e]<31:0>,
        // xs) << scale; scale = msz when scaled (bits[22:21]==11) else 0. bit13==0
        // separates this from the D-form scatter (bits[15:13]==101); bit22==1
        // (ig1 high) separates it from the unpacked x32 D-form scatter below.
        if insn >> 25 == 0b1110010
            && (insn >> 22) & 1 == 1
            && (insn >> 15) & 1 == 1
            && (insn >> 13) & 1 == 0
        {
            let msz = (insn >> 23) & 0x3;
            if msz == 3 {
                return Ok(CpuExit::Undefined(insn));
            }
            let scaled = (insn >> 21) & 0x3 == 0b11;
            let xs_signed = (insn >> 14) & 1 == 1;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let scale = if scaled { msz } else { 0 };
            let esize = 4usize; // S
            let elements = 16 / esize;
            let offs = self.v[zm].to_le_bytes();
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let off32 = read_elem(&offs, e * esize, esize) as u32;
                let off = if xs_signed {
                    off32 as i32 as i64 as u64
                } else {
                    off32 as u64
                };
                let pa = self.translate_address(base.wrapping_add(off << scale), true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    _ => self.memory.write_u32(pa, val as u32)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // ST1 scatter (unpacked: D elements, 32-bit vector offset): 1110010 msz
        // ig1 Zm 1 xs 0 Pg Rn Zt with bit22==0 (ig1 high clear). esize=64;
        // offset[e] = extend(Zm[e]<31:0>, xs) << scale.
        if insn >> 25 == 0b1110010
            && (insn >> 22) & 1 == 0
            && (insn >> 15) & 1 == 1
            && (insn >> 13) & 1 == 0
        {
            let msz = (insn >> 23) & 0x3;
            let scaled = (insn >> 21) & 1 == 1;
            let xs_signed = (insn >> 14) & 1 == 1;
            let zm = ((insn >> 16) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let scale = if scaled { msz } else { 0 };
            let esize = 8usize; // D
            let elements = 16 / esize;
            let offs = self.v[zm].to_le_bytes();
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let off32 = read_elem(&offs, e * esize, 4) as u32;
                let off = if xs_signed {
                    off32 as i32 as i64 as u64
                } else {
                    off32 as u64
                };
                let pa = self.translate_address(base.wrapping_add(off << scale), true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    4 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // LD1 gather (vector base + immediate, D elements): 1100010 msz 01 imm5
        // 1 U ff Pg Zn Zt. Each element's base IS Zn[e]; addr[e] = Zn[e] +
        // imm5 * mbytes. esize=64; load msize bytes, sign/zero-extend; zeroing.
        // ff=bit13 free (first-fault variant; no-fault path identical).
        if insn >> 25 == 0b1100010 && (insn >> 21) & 0x3 == 0b01 && (insn >> 15) & 1 == 1 {
            let msz = (insn >> 23) & 0x3;
            let unsigned = (insn >> 14) & 1 == 1;
            let imm5 = (insn >> 16) & 0x1F;
            let zn_base = ((insn >> 5) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let esize = 8usize; // D
            let elements = 16 / esize;
            let bases = self.v[zn_base].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let elem_base = read_elem(&bases, e * esize, esize);
                let ea = elem_base.wrapping_add((imm5 as u64) * (mbytes as u64));
                let pa = self.translate_address(ea, false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                let val = if unsigned {
                    raw
                } else {
                    sext_elem(raw, (mbytes * 8) as u32) as u64
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // ST1 scatter (vector base + immediate, D elements): 1110010 msz 10 imm5
        // 101 Pg Zn Zt. addr[e] = Zn[e] + imm5 * mbytes; store low msize bytes.
        if insn >> 25 == 0b1110010 && (insn >> 21) & 0x3 == 0b10 && (insn >> 13) & 0x7 == 0b101 {
            let msz = (insn >> 23) & 0x3;
            let imm5 = (insn >> 16) & 0x1F;
            let zn_base = ((insn >> 5) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let esize = 8usize; // D
            let elements = 16 / esize;
            let bases = self.v[zn_base].to_le_bytes();
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let elem_base = read_elem(&bases, e * esize, esize);
                let ea = elem_base.wrapping_add((imm5 as u64) * (mbytes as u64));
                let pa = self.translate_address(ea, true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    4 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // LD1RQ (load and replicate quadword): 1010010 msz 00 ... with
        // bits[15:13]==001 (scalar+imm: addr=base+imm4*16) or ==000 (scalar+Xm:
        // addr=base+(Xm+e)*mbytes, Rm==31 UNDEFINED). At VL=128 the quadword is
        // the whole register, so this is a packed contiguous load (zeroing).
        if !is_store
            && insn >> 25 == 0b1010010
            && (insn >> 21) & 0x3 == 0b00
            && ((b15_13 == 0b001 && (insn >> 20) & 1 == 0) || b15_13 == 0b000)
        {
            let esize = 1usize << ((insn >> 23) & 0x3);
            let elements = 16 / esize;
            let addr0 = if b15_13 == 0b001 {
                (base as i64 + imm4 * 16) as u64
            } else {
                let rm = ((insn >> 16) & 0x1F) as u8;
                if rm == 31 {
                    return Ok(CpuExit::Undefined(insn));
                }
                base.wrapping_add(self.get_x(rm).wrapping_mul(esize as u64))
            };
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let pa = self.translate_address(addr0 + (e * esize) as u64, false, false)?;
                let val: u64 = match esize {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // LDNT1 (non-temporal contiguous load): 1010010 msz 000 imm4 111 Pg Rn
        // Zt. The non-temporal hint has no architectural effect, so this is a
        // packed LD1 (esize=msize, no extension, zeroing inactive).
        if !is_store && insn >> 25 == 0b1010010 && b15_13 == 0b111 && (insn >> 20) & 0x7 == 0b000 {
            let esize = 1usize << ((insn >> 23) & 0x3);
            let elements = 16 / esize;
            let addr0 = (base as i64 + imm4 * (elements * esize) as i64) as u64;
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let pa = self.translate_address(addr0 + (e * esize) as u64, false, false)?;
                let val: u64 = match esize {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    4 => self.memory.read_u32(pa)? as u64,
                    _ => self.memory.read_u64(pa)?,
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // STNT1 (non-temporal contiguous store): 1110010 msz 001 imm4 111 Pg Rn
        // Zt (bits[22:20]==001). A packed ST1.
        if is_store && insn >> 25 == 0b1110010 && b15_13 == 0b111 && (insn >> 20) & 0x7 == 0b001 {
            let esize = 1usize << ((insn >> 23) & 0x3);
            let elements = 16 / esize;
            let addr0 = (base as i64 + imm4 * (elements * esize) as i64) as u64;
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let pa = self.translate_address(addr0 + (e * esize) as u64, true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match esize {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    4 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // LD2/LD3/LD4 (contiguous, de-interleaving): 1010010 msz opc 0 imm4 111
        // Pg Rn Zt. opc=bits[22:21] in {01,10,11} -> nreg in {2,3,4}. Reads
        // nreg*elements consecutive structures and de-interleaves them so that
        // Z[(t+r)%32][e] = Mem[base + (e*nreg + r)*mbytes]; zeroes inactive lanes.
        if !is_store && insn >> 25 == 0b1010010 && b15_13 == 0b111 && (insn >> 21) & 0x3 != 0b00 {
            let nreg = (((insn >> 21) & 0x3) + 1) as usize;
            let msz = (insn >> 23) & 0x3;
            let esize = 1usize << msz;
            let elements = 16 / esize;
            let mbytes = esize;
            let addr0 = (base as i64 + imm4 * (elements * nreg * mbytes) as i64) as u64;
            let mut regs = [[0u8; 16]; 4];
            let mut a = addr0;
            for e in 0..elements {
                let active = (pred >> (e * esize)) & 1 == 1;
                for reg in regs.iter_mut().take(nreg) {
                    if active {
                        let pa = self.translate_address(a, false, false)?;
                        let val: u64 = match mbytes {
                            1 => self.memory.read_u8(pa)? as u64,
                            2 => self.memory.read_u16(pa)? as u64,
                            4 => self.memory.read_u32(pa)? as u64,
                            _ => self.memory.read_u64(pa)?,
                        };
                        write_elem(reg, e * esize, esize, val);
                    }
                    a = a.wrapping_add(mbytes as u64);
                }
            }
            for r in 0..nreg {
                self.v[(zt + r) % 32] = u128::from_le_bytes(regs[r]);
            }
            return Ok(CpuExit::Continue);
        }

        // ST2/ST3/ST4 (contiguous, interleaving): 1110010 msz opc 1 imm4 111 Pg
        // Rn Zt. bit20==1 separates it from ST1 (bit20==0). Interleaves the nreg
        // source registers: Mem[base + (e*nreg + r)*mbytes] = Z[(t+r)%32][e].
        if is_store
            && insn >> 25 == 0b1110010
            && b15_13 == 0b111
            && (insn >> 20) & 1 == 1
            && (insn >> 21) & 0x3 != 0b00
        {
            let nreg = (((insn >> 21) & 0x3) + 1) as usize;
            let msz = (insn >> 23) & 0x3;
            let esize = 1usize << msz;
            let elements = 16 / esize;
            let mbytes = esize;
            let addr0 = (base as i64 + imm4 * (elements * nreg * mbytes) as i64) as u64;
            let mut srcs = [[0u8; 16]; 4];
            for r in 0..nreg {
                srcs[r] = self.v[(zt + r) % 32].to_le_bytes();
            }
            let mut a = addr0;
            for e in 0..elements {
                let active = (pred >> (e * esize)) & 1 == 1;
                for src in srcs.iter().take(nreg) {
                    if active {
                        let pa = self.translate_address(a, true, false)?;
                        let val = read_elem(src, e * esize, esize);
                        match mbytes {
                            1 => self.memory.write_u8(pa, val as u8)?,
                            2 => self.memory.write_u16(pa, val as u16)?,
                            4 => self.memory.write_u32(pa, val as u32)?,
                            _ => self.memory.write_u64(pa, val)?,
                        }
                    }
                    a = a.wrapping_add(mbytes as u64);
                }
            }
            return Ok(CpuExit::Continue);
        }

        // LD1 gather (S-form vector base + immediate): 1000010 msz 01 imm5 1 U
        // ff Pg Zn Zt. esize=32; the per-element base is the 32-bit Zn[e]
        // (zero-extended); addr[e] = Zn[e] + imm5*mbytes. bit22==0 (bits[22:21]
        // ==01) separates it from LD1R (bit22==1). ff=bit13 free (first-fault).
        if insn >> 25 == 0b1000010 && (insn >> 21) & 0x3 == 0b01 && (insn >> 15) & 1 == 1 {
            let msz = (insn >> 23) & 0x3;
            if msz == 3 {
                return Ok(CpuExit::Undefined(insn)); // no doubleword in S-form
            }
            let unsigned = (insn >> 14) & 1 == 1;
            let imm5 = (insn >> 16) & 0x1F;
            let zn_base = ((insn >> 5) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let esize = 4usize; // S
            let elements = 16 / esize;
            let bases = self.v[zn_base].to_le_bytes();
            let mut dst = [0u8; 16];
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let elem_base = read_elem(&bases, e * esize, esize); // 32-bit base
                let ea = elem_base.wrapping_add((imm5 as u64) * (mbytes as u64));
                let pa = self.translate_address(ea, false, false)?;
                let raw: u64 = match mbytes {
                    1 => self.memory.read_u8(pa)? as u64,
                    2 => self.memory.read_u16(pa)? as u64,
                    _ => self.memory.read_u32(pa)? as u64,
                };
                let val = if unsigned {
                    raw
                } else {
                    (sext_elem(raw, (mbytes * 8) as u32) as u64) & elem_mask(32)
                };
                write_elem(&mut dst, e * esize, esize, val);
            }
            self.v[zt] = u128::from_le_bytes(dst);
            return Ok(CpuExit::Continue);
        }

        // ST1 scatter (S-form vector base + immediate): 1110010 msz 11 imm5 101
        // Pg Zn Zt. esize=32; addr[e] = Zn[e]<31:0> + imm5*mbytes. bits[22:21]
        // ==11 separates it from the D.64 (00/01) and D vector-base (10) forms.
        if insn >> 25 == 0b1110010 && (insn >> 21) & 0x3 == 0b11 && (insn >> 13) & 0x7 == 0b101 {
            let msz = (insn >> 23) & 0x3;
            if msz == 3 {
                return Ok(CpuExit::Undefined(insn));
            }
            let imm5 = (insn >> 16) & 0x1F;
            let zn_base = ((insn >> 5) & 0x1F) as usize;
            let mbytes = 1usize << msz;
            let esize = 4usize; // S
            let elements = 16 / esize;
            let bases = self.v[zn_base].to_le_bytes();
            let src = self.v[zt].to_le_bytes();
            for e in 0..elements {
                if (pred >> (e * esize)) & 1 == 0 {
                    continue;
                }
                let elem_base = read_elem(&bases, e * esize, esize);
                let ea = elem_base.wrapping_add((imm5 as u64) * (mbytes as u64));
                let pa = self.translate_address(ea, true, false)?;
                let val = read_elem(&src, e * esize, esize);
                match mbytes {
                    1 => self.memory.write_u8(pa, val as u8)?,
                    2 => self.memory.write_u16(pa, val as u16)?,
                    _ => self.memory.write_u32(pa, val as u32)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // Other SVE memory forms (gather/vector first-fault) are not yet modelled.
        Ok(CpuExit::Undefined(insn))
    }

    /// Shared body for the contiguous first-fault (LDFF1) and non-fault (LDNF1)
    /// loads. Loads each active element; on an access that cannot be performed
    /// the access is suppressed: for LDFF1 the very first active element still
    /// faults normally, but any later element (and every element for LDNF1) is
    /// suppressed, the FFR is cleared from that element onward, and the
    /// suppressed/inactive lanes are zeroed. With no fault this is exactly LD1.
    #[allow(clippy::too_many_arguments)]
    fn exec_sve_ff_load(
        &mut self,
        addr0: u64,
        mbytes: usize,
        esize: usize,
        signed: bool,
        elements: usize,
        pred: u32,
        zt: usize,
        nonfault: bool,
    ) -> Result<CpuExit, ArmError> {
        let mut dst = [0u8; 16];
        let mut first = true;
        let mut faulted = false;
        for e in 0..elements {
            if (pred >> (e * esize)) & 1 != 1 {
                continue; // inactive -> zero
            }
            if faulted {
                self.sve_ffr &= !(1u32 << (e * esize));
                continue;
            }
            let ea = addr0 + (e * mbytes) as u64;
            let read: Result<u64, ArmError> = match self.translate_address(ea, false, false) {
                Ok(pa) => match mbytes {
                    1 => self
                        .memory
                        .read_u8(pa)
                        .map(|v| v as u64)
                        .map_err(Into::into),
                    2 => self
                        .memory
                        .read_u16(pa)
                        .map(|v| v as u64)
                        .map_err(Into::into),
                    4 => self
                        .memory
                        .read_u32(pa)
                        .map(|v| v as u64)
                        .map_err(Into::into),
                    _ => self.memory.read_u64(pa).map_err(Into::into),
                },
                Err(err) => Err(err),
            };
            match read {
                Ok(raw) => {
                    let val = if signed {
                        (sext_elem(raw, (mbytes * 8) as u32) as u64) & elem_mask((esize * 8) as u32)
                    } else {
                        raw
                    };
                    write_elem(&mut dst, e * esize, esize, val);
                    first = false;
                }
                Err(err) => {
                    if first && !nonfault {
                        return Err(err); // LDFF1's first active element faults normally
                    }
                    faulted = true;
                    self.sve_ffr &= !(1u32 << (e * esize));
                }
            }
        }
        self.v[zt] = u128::from_le_bytes(dst);
        Ok(CpuExit::Continue)
    }

    // =========================================================================
    // Instruction Implementations (stubs - to be filled in)
    // =========================================================================

    fn exec_pc_rel(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op = (insn >> 31) & 1;
        let rd = (insn & 0x1F) as u8;
        let immhi = ((insn >> 5) & 0x7FFFF) as i64;
        let immlo = ((insn >> 29) & 0x3) as i64;
        let imm = (immhi << 2) | immlo;
        let imm = (imm << 43) >> 43; // Sign extend from 21 bits

        // PC was already incremented, use the address of this instruction
        let current_pc = self.pc.wrapping_sub(4);

        let result = if op == 0 {
            // ADR
            (current_pc as i64).wrapping_add(imm) as u64
        } else {
            // ADRP
            let base = current_pc & !0xFFF;
            (base as i64).wrapping_add(imm << 12) as u64
        };

        self.set_x(rd, result);
        Ok(CpuExit::Continue)
    }

    fn exec_add_sub_imm(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op = (insn >> 30) & 1; // 0=ADD, 1=SUB
        let s = (insn >> 29) & 1; // Set flags
        let sh = (insn >> 22) & 1; // Shift imm by 12
        let imm12 = ((insn >> 10) & 0xFFF) as u64;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        let imm = if sh != 0 { imm12 << 12 } else { imm12 };

        if sf != 0 {
            // 64-bit
            let rn_val = if rn == 31 {
                self.current_sp()
            } else {
                self.get_x(rn)
            };

            let (result, carry, overflow) = if op == 0 {
                let (r, c) = rn_val.overflowing_add(imm);
                let v = (!(rn_val ^ imm) & (rn_val ^ r)) >> 63 != 0;
                (r, c, v)
            } else {
                let (r, c) = rn_val.overflowing_sub(imm);
                let v = ((rn_val ^ imm) & (rn_val ^ r)) >> 63 != 0;
                (r, !c, v)
            };

            if s != 0 {
                self.update_nz_64(result);
                self.set_c(carry);
                self.set_v(overflow);
            }

            if rd == 31 {
                if s == 0 {
                    self.set_current_sp(result);
                }
            } else {
                self.set_x(rd, result);
            }
        } else {
            // 32-bit
            let rn_val = if rn == 31 {
                self.current_sp() as u32
            } else {
                self.get_w(rn)
            };
            let imm = imm as u32;

            let (result, carry, overflow) = if op == 0 {
                let (r, c) = rn_val.overflowing_add(imm);
                let v = (!(rn_val ^ imm) & (rn_val ^ r)) >> 31 != 0;
                (r, c, v)
            } else {
                let (r, c) = rn_val.overflowing_sub(imm);
                let v = ((rn_val ^ imm) & (rn_val ^ r)) >> 31 != 0;
                (r, !c, v)
            };

            if s != 0 {
                self.update_nz_32(result);
                self.set_c(carry);
                self.set_v(overflow);
            }

            if rd == 31 {
                if s == 0 {
                    self.set_current_sp(result as u64);
                }
            } else {
                self.set_w(rd, result);
            }
        }

        Ok(CpuExit::Continue)
    }

    /// Execute Add/Sub Immediate with Tags (ADDG/SUBG - MTE instructions).
    ///
    /// Encoding:
    /// 31:31 sf (must be 1 for 64-bit)
    /// 30:30 op (0=ADD, 1=SUB)
    /// 29:29 S (must be 0)
    /// 28:23 100011
    /// 22:22 o2 (must be 0)
    /// 21:16 uimm6 (offset in 16-byte granules)
    /// 15:14 op3
    /// 13:10 uimm4 (tag offset)
    /// 9:5   Xn (source register)
    /// 4:0   Xd (destination register)
    fn exec_add_sub_imm_tags(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op = (insn >> 30) & 1; // 0=ADDG, 1=SUBG
        let uimm6 = ((insn >> 16) & 0x3F) as u64;
        let uimm4 = ((insn >> 10) & 0xF) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        // TAG_GRANULE is 16 bytes (LOG2_TAG_GRANULE = 4)
        const TAG_GRANULE: u64 = 16;
        let offset = uimm6 * TAG_GRANULE;

        // Get source operand
        let operand1 = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };

        // Extract the current allocation tag from address bits [59:56]
        let start_tag = ((operand1 >> 56) & 0xF) as u8;

        // Compute new tag (simplified - in full MTE, this uses GCR_EL1.Exclude)
        // The tag is modified by uimm4, wrapping at 16
        let rtag = if self.config.features.has_mte() {
            // MTE enabled - compute new tag
            (start_tag.wrapping_add(uimm4)) & 0xF
        } else {
            // MTE disabled - tag is 0
            0
        };

        // Compute result address
        let result = if op == 0 {
            // ADDG
            operand1.wrapping_add(offset)
        } else {
            // SUBG
            operand1.wrapping_sub(offset)
        };

        // Insert the new allocation tag into the result address
        // Tags are stored in bits [59:56] (top byte, lower nibble)
        let result = (result & !0x0F00_0000_0000_0000u64) | ((rtag as u64) << 56);

        // Write result
        if rd == 31 {
            self.set_current_sp(result);
        } else {
            self.set_x(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_logical_imm(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let opc = (insn >> 29) & 0x3;
        let n = (insn >> 22) & 1;
        let immr = ((insn >> 16) & 0x3F) as u32;
        let imms = ((insn >> 10) & 0x3F) as u32;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        // Decode bitmask immediate
        let imm = decode_bitmask(n != 0, imms, immr, sf != 0)?;

        if sf != 0 {
            // 64-bit
            let rn_val = self.get_x(rn);

            let result = match opc {
                0b00 => rn_val & imm, // AND
                0b01 => rn_val | imm, // ORR
                0b10 => rn_val ^ imm, // EOR
                0b11 => rn_val & imm, // ANDS
                _ => unreachable!(),
            };

            if opc == 0b11 {
                self.update_nz_64(result);
                self.set_c(false);
                self.set_v(false);
            }

            if rd == 31 && opc != 0b11 {
                self.set_current_sp(result);
            } else {
                self.set_x(rd, result);
            }
        } else {
            // 32-bit
            let rn_val = self.get_w(rn);
            let imm = imm as u32;

            let result = match opc {
                0b00 => rn_val & imm,
                0b01 => rn_val | imm,
                0b10 => rn_val ^ imm,
                0b11 => rn_val & imm,
                _ => unreachable!(),
            };

            if opc == 0b11 {
                self.update_nz_32(result);
                self.set_c(false);
                self.set_v(false);
            }

            if rd == 31 && opc != 0b11 {
                self.set_current_sp(result as u64);
            } else {
                self.set_w(rd, result);
            }
        }

        Ok(CpuExit::Continue)
    }

    fn exec_move_wide(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let opc = (insn >> 29) & 0x3;
        let hw = ((insn >> 21) & 0x3) as u32;
        let imm16 = ((insn >> 5) & 0xFFFF) as u64;
        let rd = (insn & 0x1F) as u8;

        let shift = hw * 16;

        let result = match opc {
            0b00 => {
                // MOVN
                let val = imm16 << shift;
                if sf != 0 { !val } else { (!val) & 0xFFFF_FFFF }
            }
            0b10 => {
                // MOVZ
                imm16 << shift
            }
            0b11 => {
                // MOVK
                let old = if sf != 0 {
                    self.get_x(rd)
                } else {
                    self.get_w(rd) as u64
                };
                let mask = !(0xFFFFu64 << shift);
                (old & mask) | (imm16 << shift)
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };

        if sf != 0 {
            self.set_x(rd, result);
        } else {
            self.set_w(rd, result as u32);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_bitfield(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let opc = (insn >> 29) & 0x3;
        let n = (insn >> 22) & 1;
        let immr = ((insn >> 16) & 0x3F) as u32;
        let imms = ((insn >> 10) & 0x3F) as u32;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        let datasize = if sf != 0 { 64u32 } else { 32 };

        // Decode wmask and tmask
        let (wmask, tmask) = decode_bitmasks(n != 0, imms, immr, false, datasize)?;

        let src = if sf != 0 {
            self.get_x(rn)
        } else {
            self.get_w(rn) as u64
        };

        let dst = if sf != 0 {
            self.get_x(rd)
        } else {
            self.get_w(rd) as u64
        };

        // Rotate right
        let bot = if immr == 0 {
            src
        } else {
            (src >> immr) | (src << (datasize - immr))
        };

        // Per the ARM pseudocode: bot = ROR(src, immr) AND wmask, and the
        // destination combines top/bot under TMASK (not wmask — using wmask
        // here turns e.g. `asr xD, xN, #32` into a rotate).
        let bot = bot & wmask;
        let result = match opc {
            0b00 => {
                // SBFM
                // Sign-extend based on imms
                let top = if (src >> imms) & 1 != 0 { !0u64 } else { 0u64 };
                (top & !tmask) | (bot & tmask)
            }
            0b01 => {
                // BFM
                let merged = (dst & !wmask) | bot;
                (dst & !tmask) | (merged & tmask)
            }
            0b10 => {
                // UBFM
                bot & tmask
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };

        if sf != 0 {
            self.set_x(rd, result);
        } else {
            self.set_w(rd, result as u32);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_extract(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let n = (insn >> 22) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let imms = ((insn >> 10) & 0x3F) as u32;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        let datasize = if sf != 0 { 64u32 } else { 32 };
        let lsb = imms;

        let operand1 = if sf != 0 {
            self.get_x(rn)
        } else {
            self.get_w(rn) as u64
        };

        let operand2 = if sf != 0 {
            self.get_x(rm)
        } else {
            self.get_w(rm) as u64
        };

        let result = if lsb == 0 {
            operand2
        } else {
            (operand1 << (datasize - lsb)) | (operand2 >> lsb)
        };

        if sf != 0 {
            self.set_x(rd, result);
        } else {
            self.set_w(rd, result as u32);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_b_cond(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let imm19 = ((insn >> 5) & 0x7FFFF) as i64;
        let cond = (insn & 0xF) as u8;

        let offset = ((imm19 << 45) >> 43) as i64; // Sign extend and multiply by 4

        if self.condition_holds(cond) {
            self.pc = ((self.pc as i64).wrapping_sub(4).wrapping_add(offset)) as u64;
        }

        Ok(CpuExit::Continue)
    }

    fn exec_exception_system(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Check bits [31:24] to distinguish exception generation from system instructions
        let bits_31_24 = (insn >> 24) & 0xFF;

        if bits_31_24 == 0xD4 {
            // Exception generation: opc (bits 23:21) selects the group and
            // LL (bits 1:0) the target level — SVC/HVC/SMC share opc=000 and
            // differ only in LL.
            let opc = (insn >> 21) & 0x7;
            let ll = insn & 0x3;
            let imm16 = ((insn >> 5) & 0xFFFF) as u16;

            return match (opc, ll) {
                (0b000, 0b01) => Ok(CpuExit::Svc(imm16 as u32)),
                (0b000, 0b10) => Ok(CpuExit::Hvc(imm16)),
                (0b000, 0b11) => Ok(CpuExit::Smc(imm16)),
                (0b001, 0b00) => Ok(CpuExit::Breakpoint(imm16 as u32)),
                (0b010, 0b00) | (0b101, 0b01..=0b11) => {
                    // HLT / DCPS1-3: halt into (emulated) debug state.
                    self.halted = true;
                    Ok(CpuExit::Halt)
                }
                _ => Err(ArmError::UndefinedInstruction(insn)),
            };
        }

        // bits [31:22] = 0x354 (1101_0101_00) = system instructions
        // This covers hints, barriers, MSR, MRS, etc.
        let l = (insn >> 21) & 1;
        let op0 = (insn >> 19) & 0x3;

        if l == 0 && op0 == 0 {
            // System instructions with L=0, op0=00 (hints, barriers, MSR imm)
            return self.exec_system(insn);
        }

        // SYS/SYSL (op0=01): cache/TLB maintenance, DC ZVA, AT
        if op0 == 1 {
            return self.exec_sys_insn(insn, l == 1);
        }

        // MSR/MRS (system register access)
        // L=0: MSR (write), L=1: MRS (read)
        // op0 = 10 or 11 for different register categories
        if op0 != 0 || l == 1 {
            return self.exec_msr_mrs(insn);
        }

        Err(ArmError::UndefinedInstruction(insn))
    }

    /// Execute SYS/SYSL (op0=01): DC/IC/TLBI/AT space. Maintenance operations
    /// are no-ops for this memory model, with the exception of DC ZVA which
    /// architecturally zeroes a block of memory and is relied on by kernels
    /// for page clearing.
    fn exec_sys_insn(&mut self, insn: u32, is_read: bool) -> Result<CpuExit, ArmError> {
        let op1 = ((insn >> 16) & 0x7) as u8;
        let crn = ((insn >> 12) & 0xF) as u8;
        let crm = ((insn >> 8) & 0xF) as u8;
        let op2 = ((insn >> 5) & 0x7) as u8;
        let rt = (insn & 0x1F) as u8;

        if is_read {
            // SYSL: nothing implemented reads back state
            if rt != 31 {
                self.set_x(rt, 0);
            }
            return Ok(CpuExit::Continue);
        }

        // DC ZVA: zero a block of memory at X[rt]
        if (op1, crn, crm, op2) == (3, 7, 4, 1) {
            let block = 4usize << (self.sysregs.dczid_el0 & 0xF);
            let va = self.get_x(rt) & !(block as u64 - 1);
            for off in (0..block as u64).step_by(8) {
                self.mem_write_u64(va + off, 0)?;
            }
            return Ok(CpuExit::Continue);
        }

        // AT S1E1R/S1E1W/S1E0R/S1E0W: stage-1 address translation probe;
        // result lands in PAR_EL1.
        if (op1, crn, crm) == (0, 7, 8) && op2 < 4 {
            let va = self.get_x(rt);
            let privileged = op2 < 2;
            let is_write = op2 & 1 != 0;
            let par = match self.mmu.translate(
                va,
                self.memory.as_ref(),
                is_write,
                false,
                privileged,
                self.current_el,
            ) {
                // F=0, PA in bits 51:12, outer/inner WB cacheable attrs.
                Ok(desc) => (desc.pa & 0x000F_FFFF_FFFF_F000) | (0xFFu64 << 56),
                // F=1, fault status in bits 6:1.
                Err(fault) => {
                    let fsc =
                        fsc_for_fault(translation_fault_type_of(&fault), fault.level) as u64;
                    1 | (fsc << 1)
                }
            };
            let enc = Aarch64SysRegEncoding::new(3, 0, 7, 4, 0); // PAR_EL1
            let _ = self.sysregs.write(enc, par, self.current_el);
            return Ok(CpuExit::Continue);
        }

        // Everything else (DC/IC/TLBI/...) is a no-op: there are no caches,
        // and translations are walked on every access.
        Ok(CpuExit::Continue)
    }

    fn exec_system(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let crn = ((insn >> 12) & 0xF) as u8;
        let op1 = ((insn >> 16) & 0x7) as u8;
        let op2 = ((insn >> 5) & 0x7) as u8;

        if crn == 4 {
            // PSTATE space: MSR (immediate) writes of PSTATE fields (CRm
            // carries the immediate) plus the FEAT_FlagM flag-format ops.
            // Kernels lean on DAIFSet/DAIFClr for interrupt masking, so these
            // must not fall through as hints.
            let imm = ((insn >> 8) & 0xF) as u8;
            match (op1, op2) {
                // CFINV
                (0, 0b000) => {
                    let c = self.get_c();
                    self.set_nzcv(self.get_n(), self.get_z(), !c, self.get_v());
                }
                // XAFLAG
                (0, 0b001) => {
                    let (z, c) = (self.get_z(), self.get_c());
                    self.set_nzcv(!c && z, z && c, c || z, !c && z);
                }
                // AXFLAG
                (0, 0b010) => {
                    let (z, c, v) = (self.get_z(), self.get_c(), self.get_v());
                    self.set_nzcv(false, z || v, c && !v, false);
                }
                // UAO
                (0, 0b011) => self.uao = imm & 1 != 0,
                // PAN
                (0, 0b100) => self.pan = imm & 1 != 0,
                // SPSel
                (0, 0b101) => self.sp_sel = imm & 1 != 0,
                // SSBS
                (3, 0b001) => self.ssbs = imm & 1 != 0,
                // DIT
                (3, 0b010) => self.dit = imm & 1 != 0,
                // TCO
                (3, 0b100) => self.tco = imm & 1 != 0,
                // DAIFSet
                (3, 0b110) => self.daif |= imm,
                // DAIFClr
                (3, 0b111) => self.daif &= !imm,
                // Unallocated PSTATE ops behave as NOPs here (lenient, like
                // the pre-existing fall-through).
                _ => {}
            }
            return Ok(CpuExit::Continue);
        }

        if crn == 2 && op1 == 3 {
            // Hints: HINT #imm7 where imm7 = CRm:op2. Only CRm=0 carries the
            // classic NOP/YIELD/WFE/WFI/SEV/SEVL group; higher CRm values are
            // BTI landing pads, pointer-auth hints (PACIASP/AUTIASP), etc.,
            // which behave as NOPs here.
            let crm = ((insn >> 8) & 0xF) as u8;
            if crm != 0 {
                return Ok(CpuExit::Continue);
            }
            match op2 {
                0b000 => Ok(CpuExit::Continue), // NOP
                0b001 => Ok(CpuExit::Continue), // YIELD
                0b010 => {
                    // WFE
                    if self.event_register {
                        self.event_register = false;
                        Ok(CpuExit::Continue)
                    } else {
                        self.wfe = true;
                        Ok(CpuExit::Wfe)
                    }
                }
                0b011 => {
                    // WFI
                    self.wfi = true;
                    Ok(CpuExit::Wfi)
                }
                0b100 => {
                    // SEV
                    self.event_register = true;
                    Ok(CpuExit::Continue)
                }
                0b101 => {
                    // SEVL
                    self.event_register = true;
                    Ok(CpuExit::Continue)
                }
                _ => Ok(CpuExit::Continue),
            }
        } else if crn == 3 {
            // Barriers
            match op2 {
                0b010 => Ok(CpuExit::Continue), // CLREX
                0b100 => Ok(CpuExit::Continue), // DSB
                0b101 => Ok(CpuExit::Continue), // DMB
                0b110 => Ok(CpuExit::Continue), // ISB
                _ => Ok(CpuExit::Continue),
            }
        } else {
            // Other system instructions (cache maintenance, etc.)
            Ok(CpuExit::Continue)
        }
    }

    fn exec_msr_mrs(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let l = (insn >> 21) & 1; // 0 = MSR, 1 = MRS
        let o0 = ((insn >> 19) & 0x1) as u8 + 2;
        let op1 = ((insn >> 16) & 0x7) as u8;
        let crn = ((insn >> 12) & 0xF) as u8;
        let crm = ((insn >> 8) & 0xF) as u8;
        let op2 = ((insn >> 5) & 0x7) as u8;
        let rt = (insn & 0x1F) as u8;

        let encoding = Aarch64SysRegEncoding::new(o0, op1, crn, crm, op2);

        if l != 0 {
            // MRS
            let value = self.read_sysreg(encoding)?;
            self.set_x(rt, value);
            self.record_sysreg_trace(encoding, "read", value);
        } else {
            // MSR
            let value = self.get_x(rt);
            self.write_sysreg(encoding, value)?;
            self.record_sysreg_trace(encoding, "write", value);
        }

        Ok(CpuExit::Continue)
    }

    fn record_sysreg_trace(&self, encoding: Aarch64SysRegEncoding, op: &str, value: u64) {
        if !runtime_trace_enabled() {
            return;
        }
        record_runtime_event(&TraceEvent::Sysreg {
            pc: self.pc,
            name: encoding.to_string(),
            op: op.to_string(),
            value,
        });
    }

    fn exec_br_reg(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 21) & 0xF;
        let op2 = (insn >> 16) & 0x1F;
        let op3 = (insn >> 10) & 0x3F;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let op4 = insn & 0x1F;

        if op2 != 0x1F {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        // PAC-authenticated branch family (arm64e): op3 = 0b00001x. We model PAC
        // signing (PACIxSP etc.) as a no-op, so the matching authenticated branch
        // behaves as its plain counterpart on the unmodified pointer. This boots
        // arm64e kernels without enforcing pointer-auth security.
        if op3 == 0b000010 || op3 == 0b000011 {
            match opc {
                0b0000 | 0b1000 => {
                    // BRAA/BRAB/BRAAZ/BRABZ
                    self.pc = self.get_x(rn);
                    self.btype = 0b01;
                }
                0b0001 | 0b1001 => {
                    // BLRAA/BLRAB/BLRAAZ/BLRABZ
                    self.set_x(30, self.pc);
                    self.pc = self.get_x(rn);
                    self.btype = 0b10;
                }
                0b0010 => {
                    // RETAA/RETAB: return to X30 (auth modifier ignored)
                    self.pc = self.get_x(30);
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            }
            return Ok(CpuExit::Continue);
        }

        if op3 != 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let target = self.get_x(rn);

        match (opc, op4) {
            (0b0000, 0) => {
                // BR
                self.pc = target;
                self.btype = 0b01;
            }
            (0b0001, 0) => {
                // BLR
                self.set_x(30, self.pc);
                self.pc = target;
                self.btype = 0b10;
            }
            (0b0010, 0) => {
                // RET
                let lr = if rn == 31 { 30 } else { rn };
                self.pc = self.get_x(lr);
            }
            (0b0100, 0) => {
                // ERET
                return self.exception_return();
            }
            (0b0101, 0) => {
                // DRPS: debug restore process state. Outside real debug state
                // this behaves as a NOP in our model.
                return Ok(CpuExit::Continue);
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_cbz_cbnz(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op = (insn >> 24) & 1; // 0=CBZ, 1=CBNZ
        let imm19 = ((insn >> 5) & 0x7FFFF) as i64;
        let rt = (insn & 0x1F) as u8;

        let offset = ((imm19 << 45) >> 43) as i64;
        let operand = if sf != 0 {
            self.get_x(rt)
        } else {
            self.get_w(rt) as u64
        };

        let take_branch = if op == 0 { operand == 0 } else { operand != 0 };

        if take_branch {
            self.pc = ((self.pc as i64).wrapping_sub(4).wrapping_add(offset)) as u64;
        }

        Ok(CpuExit::Continue)
    }

    fn exec_tbz_tbnz(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let b5 = (insn >> 31) & 1;
        let op = (insn >> 24) & 1; // 0=TBZ, 1=TBNZ
        let b40 = ((insn >> 19) & 0x1F) as u32;
        let imm14 = ((insn >> 5) & 0x3FFF) as i64;
        let rt = (insn & 0x1F) as u8;

        let bit_pos = (b5 << 5) | b40;
        let offset = ((imm14 << 50) >> 48) as i64;
        let operand = self.get_x(rt);
        let bit_set = (operand >> bit_pos) & 1 != 0;

        let take_branch = if op == 0 { !bit_set } else { bit_set };

        if take_branch {
            self.pc = ((self.pc as i64).wrapping_sub(4).wrapping_add(offset)) as u64;
        }

        Ok(CpuExit::Continue)
    }

    fn exec_b_bl(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op = (insn >> 31) & 1; // 0=B, 1=BL
        let imm26 = (insn & 0x03FF_FFFF) as i64;

        let offset = ((imm26 << 38) >> 36) as i64; // Sign extend and multiply by 4

        if op != 0 {
            // BL - save return address
            self.set_x(30, self.pc);
            self.btype = 0b10;
        }

        self.pc = ((self.pc as i64).wrapping_sub(4).wrapping_add(offset)) as u64;

        Ok(CpuExit::Continue)
    }

    // Load/Store implementations
    /// Execute Load/Store Exclusive instructions (LDXR, STXR, LDAXR, STLXR, etc.)
    ///
    /// Encoding (from ASL):
    /// 31:30 size (00=8-bit, 01=16-bit, 10=32-bit, 11=64-bit)
    /// 29:24 001000
    /// 23:23 o2 (pair indicator)
    /// 22:22 L (0=store, 1=load)
    /// 21:21 o1
    /// 20:16 Rs (status register for store)
    /// 15:15 o0 (1=acquire/release semantics)
    /// 14:10 Rt2 (for pair)
    /// 9:5   Rn (base register)
    /// 4:0   Rt (data register)
    fn exec_ldst_exclusive(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let size = (insn >> 30) & 0x3;
        let o2 = (insn >> 23) & 0x1; // 1 = pair
        let l = (insn >> 22) & 0x1; // 1 = load, 0 = store
        let o1 = (insn >> 21) & 0x1;
        let rs = ((insn >> 16) & 0x1F) as u8;
        let o0 = (insn >> 15) & 0x1; // 1 = acquire/release
        let rt2 = ((insn >> 10) & 0x1F) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rt = (insn & 0x1F) as u8;

        // CAS/CASA/CASL/CASAL (FEAT_LSE): o2==1 (bit23) and o1==1 (bit21).
        // A single compare-and-swap RMW (no exclusive monitor needed).
        if o2 == 1 && o1 == 1 {
            let bits = 8u32 << size;
            let m = elem_mask(bits);
            let addr = if rn == 31 {
                self.current_sp()
            } else {
                self.get_x(rn)
            };
            let old = match size {
                0 => self.mem_read_u8(addr)? as u64,
                1 => self.mem_read_u16(addr)? as u64,
                2 => self.mem_read_u32(addr)? as u64,
                _ => self.mem_read_u64(addr)?,
            };
            let compare = self.get_x(rs) & m;
            if (old & m) == compare {
                let newval = self.get_x(rt) & m;
                match size {
                    0 => self.mem_write_u8(addr, newval as u8)?,
                    1 => self.mem_write_u16(addr, newval as u16)?,
                    2 => self.mem_write_u32(addr, newval as u32)?,
                    _ => self.mem_write_u64(addr, newval)?,
                }
            }
            if size == 3 {
                self.set_x(rs, old);
            } else {
                self.set_w(rs, old as u32);
            }
            return Ok(CpuExit::Continue);
        }

        // CASP/CASPA/CASPL/CASPAL (FEAT_LSE): compare-and-swap pair.
        // Encoding: 0 sz 001000 0 L 1 Rs o0 11111 Rn Rt (bit31==0, o2==0, o1==1).
        // sz==0 -> 32-bit pair, sz==1 -> 64-bit pair. Rs/Rt must be even.
        if o2 == 0 && o1 == 1 && (insn >> 31) & 1 == 0 {
            let sz = (insn >> 30) & 1; // 0 = 32-bit pair, 1 = 64-bit pair
            let addr = if rn == 31 {
                self.current_sp()
            } else {
                self.get_x(rn)
            };
            let s = rs as usize;
            let t = rt as usize;
            if sz == 0 {
                // 32-bit pair: low element at addr, high element at addr+4.
                let lo = self.mem_read_u32(addr)?;
                let hi = self.mem_read_u32(addr + 4)?;
                let s1 = self.get_x(rs) as u32; // compare low
                let s2 = self.get_x((s + 1) as u8) as u32; // compare high
                if lo == s1 && hi == s2 {
                    let t1 = self.get_x(rt) as u32;
                    let t2 = self.get_x((t + 1) as u8) as u32;
                    self.mem_write_u32(addr, t1)?;
                    self.mem_write_u32(addr + 4, t2)?;
                }
                self.set_w(rs, lo);
                self.set_w((s + 1) as u8, hi);
            } else {
                // 64-bit pair: low element at addr, high element at addr+8.
                let lo = self.mem_read_u64(addr)?;
                let hi = self.mem_read_u64(addr + 8)?;
                let s1 = self.get_x(rs);
                let s2 = self.get_x((s + 1) as u8);
                if lo == s1 && hi == s2 {
                    let t1 = self.get_x(rt);
                    let t2 = self.get_x((t + 1) as u8);
                    self.mem_write_u64(addr, t1)?;
                    self.mem_write_u64(addr + 8, t2)?;
                }
                self.set_x(rs, lo);
                self.set_x((s + 1) as u8, hi);
            }
            return Ok(CpuExit::Continue);
        }

        // LDAR/STLR (and the LDARB/LDARH/STLRB/STLRH byte/halfword forms,
        // plus the LDLAR/STLLR LORegion variants): ordered but NOT exclusive
        // — plain accesses with acquire/release semantics. They carry
        // Rs=Rt2=11111 and must not consult the exclusive monitor: a
        // spin-unlock's STLRB would otherwise be silently dropped.
        if o2 == 1 && o1 == 0 {
            let address = if rn == 31 {
                self.current_sp()
            } else {
                self.get_x(rn)
            };
            // Ordered accesses are alignment-checked regardless of SCTLR.A.
            let elsize = 1u64 << size;
            if address & (elsize - 1) != 0 {
                return Err(ArmError::MemoryError(MemoryFaultInfo {
                    address,
                    access: if l == 1 {
                        crate::arm::cpu_trait::AccessType::Read
                    } else {
                        crate::arm::cpu_trait::AccessType::Write
                    },
                    fault_type: MemoryFaultType::Alignment,
                    stage2: false,
                }));
            }
            let pa = self.translate_address(address, l == 0, false)?;
            if l == 1 {
                match size {
                    0 => {
                        let val = self.memory.read_u8(pa)?;
                        self.set_w(rt, val as u32);
                    }
                    1 => {
                        let val = self.memory.read_u16(pa)?;
                        self.set_w(rt, val as u32);
                    }
                    2 => {
                        let val = self.memory.read_u32(pa)?;
                        self.set_w(rt, val);
                    }
                    _ => {
                        let val = self.memory.read_u64(pa)?;
                        self.set_x(rt, val);
                    }
                }
            } else {
                let val = if rt == 31 { 0 } else { self.get_x(rt) };
                match size {
                    0 => self.memory.write_u8(pa, val as u8)?,
                    1 => self.memory.write_u16(pa, val as u16)?,
                    2 => self.memory.write_u32(pa, val as u32)?,
                    _ => self.memory.write_u64(pa, val)?,
                }
            }
            return Ok(CpuExit::Continue);
        }

        // Pair exclusive ops (LDXP/STXP/LDAXP/STLXP) are flagged by o1 (bit21);
        // single LDXR/STXR have o1==0.
        let is_pair = o1 == 1;
        let is_load = l == 1;
        let is_ordered = o0 == 1; // acquire/release semantics (LDAXR/STLXR)

        // Element size in bytes
        let elsize = 1usize << size; // 1, 2, 4, or 8 bytes
        let datasize = if is_pair { elsize * 2 } else { elsize };

        // Get address from base register
        let address = if rn == 31 {
            // SP - check alignment
            let sp = self.current_sp();
            // SP must be aligned to 16 bytes for stack access
            if sp & 0xF != 0 {
                return Err(ArmError::MemoryError(MemoryFaultInfo {
                    address: sp,
                    access: crate::arm::cpu_trait::AccessType::Read,
                    fault_type: MemoryFaultType::Alignment,
                    stage2: false,
                }));
            }
            sp
        } else {
            self.get_x(rn)
        };

        // Translate address (for physical memory access)
        let pa = self.translate_address(address, !is_load, false)?;

        if is_load {
            // Load exclusive: LDXR, LDAXR, LDXP, LDAXP

            // Set exclusive monitors for this address range
            self.memory.mark_exclusive(pa, datasize as u8);

            if is_pair {
                // Load pair (LDXP, LDAXP)
                if elsize == 4 {
                    // 32-bit pair - atomic 64-bit load
                    let data = self.memory.read_u64(pa)?;
                    // Little-endian: lower register gets lower bits
                    self.set_w(rt, data as u32);
                    self.set_w(rt2, (data >> 32) as u32);
                } else {
                    // 64-bit pair - two 64-bit loads (128-bit aligned)
                    if pa & 0xF != 0 {
                        return Err(ArmError::MemoryError(MemoryFaultInfo {
                            address,
                            access: crate::arm::cpu_trait::AccessType::Read,
                            fault_type: MemoryFaultType::Alignment,
                            stage2: false,
                        }));
                    }
                    let val1 = self.memory.read_u64(pa)?;
                    let val2 = self.memory.read_u64(pa + 8)?;
                    self.set_x(rt, val1);
                    self.set_x(rt2, val2);
                }
            } else {
                // Single register load (LDXR, LDAXR, LDXRB, LDXRH)
                match elsize {
                    1 => {
                        let val = self.memory.read_u8(pa)?;
                        self.set_w(rt, val as u32);
                    }
                    2 => {
                        let val = self.memory.read_u16(pa)?;
                        self.set_w(rt, val as u32);
                    }
                    4 => {
                        let val = self.memory.read_u32(pa)?;
                        self.set_w(rt, val);
                    }
                    8 => {
                        let val = self.memory.read_u64(pa)?;
                        self.set_x(rt, val);
                    }
                    _ => unreachable!(),
                }
            }

            // Memory barrier for acquire semantics
            if is_ordered {
                // LDAXR has acquire semantics - barrier is implicit
                // In our single-threaded emulator, this is a no-op
            }
        } else {
            // Store exclusive: STXR, STLXR, STXP, STLXP

            // Memory barrier for release semantics
            if is_ordered {
                // STLXR has release semantics - barrier is implicit
                // In our single-threaded emulator, this is a no-op
            }

            // Check if exclusive monitors pass
            let exclusive_held = self.memory.check_exclusive(pa, datasize as u8);

            if exclusive_held {
                // Exclusive access succeeded - perform the store
                if is_pair {
                    if elsize == 4 {
                        // 32-bit pair - atomic 64-bit store
                        let val1 = self.get_w(rt) as u64;
                        let val2 = self.get_w(rt2) as u64;
                        let data = val1 | (val2 << 32);
                        self.memory.write_u64(pa, data)?;
                    } else {
                        // 64-bit pair
                        if pa & 0xF != 0 {
                            return Err(ArmError::MemoryError(MemoryFaultInfo {
                                address,
                                access: crate::arm::cpu_trait::AccessType::Write,
                                fault_type: MemoryFaultType::Alignment,
                                stage2: false,
                            }));
                        }
                        let val1 = self.get_x(rt);
                        let val2 = self.get_x(rt2);
                        self.memory.write_u64(pa, val1)?;
                        self.memory.write_u64(pa + 8, val2)?;
                    }
                } else {
                    // Single register store
                    match elsize {
                        1 => {
                            let val = self.get_w(rt) as u8;
                            self.memory.write_u8(pa, val)?;
                        }
                        2 => {
                            let val = self.get_w(rt) as u16;
                            self.memory.write_u16(pa, val)?;
                        }
                        4 => {
                            let val = self.get_w(rt);
                            self.memory.write_u32(pa, val)?;
                        }
                        8 => {
                            let val = self.get_x(rt);
                            self.memory.write_u64(pa, val)?;
                        }
                        _ => unreachable!(),
                    }
                }

                // Store succeeded - write 0 to status register
                self.set_w(rs, 0);
            } else {
                // Exclusive access failed - write 1 to status register
                self.set_w(rs, 1);
            }
        }

        Ok(CpuExit::Continue)
    }

    fn exec_ldr_literal(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 30) & 0x3;
        let v = (insn >> 26) & 1;
        let imm19 = ((insn >> 5) & 0x7FFFF) as i64;
        let rt = (insn & 0x1F) as u8;

        let offset = ((imm19 << 45) >> 43) as i64;
        let address = ((self.pc as i64).wrapping_sub(4).wrapping_add(offset)) as u64;

        if v != 0 {
            // LDR (literal, SIMD&FP): opc selects S/D/Q; opc=11 unallocated.
            let bytes = match opc {
                0b00 => 4usize,
                0b01 => 8,
                0b10 => 16,
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            let mut buf = [0u8; 16];
            for (i, b) in buf.iter_mut().enumerate().take(bytes) {
                *b = self.mem_read_u8(address + i as u64)?;
            }
            self.v[rt as usize] = u128::from_le_bytes(buf);
            return Ok(CpuExit::Continue);
        }

        match opc {
            0b00 => {
                // LDR (32-bit)
                let value = self.mem_read_u32(address)?;
                self.set_w(rt, value);
            }
            0b01 => {
                // LDR (64-bit)
                let value = self.mem_read_u64(address)?;
                self.set_x(rt, value);
            }
            0b10 => {
                // LDRSW
                let value = self.mem_read_u32(address)? as i32 as i64 as u64;
                self.set_x(rt, value);
            }
            0b11 => {
                // PRFM - prefetch, NOP
                return Ok(CpuExit::Continue);
            }
            _ => unreachable!(),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_ldst_pair(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let opc = (insn >> 30) & 0x3;
        let v = (insn >> 26) & 1;
        let mode = (insn >> 23) & 0x3; // 00=no-alloc, 01=post, 10=signed, 11=pre
        let l = (insn >> 22) & 1; // 0=store, 1=load
        let imm7 = ((insn >> 15) & 0x7F) as i32;
        let rt2 = ((insn >> 10) & 0x1F) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rt = (insn & 0x1F) as u8;

        // Element (per-register) size in bytes and whether LDPSW sign-extends.
        let (bytes, ldpsw) = if v != 0 {
            let b = match opc {
                0b00 => 4usize, // S
                0b01 => 8,      // D
                0b10 => 16,     // Q
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            (b, false)
        } else {
            match opc {
                0b00 => (4usize, false), // 32-bit
                0b01 => (4, true),       // LDPSW (load only)
                0b10 => (8, false),      // 64-bit
                // STTP/LDTP/STTNP/LDTNP (FEAT_LRCPC3 unprivileged pair):
                // privilege-checking aside, plain 64-bit pair semantics.
                0b11 => (8, false),
                _ => unreachable!(),
            }
        };
        // opc=01, V=0 splits by the L bit: L=1 is LDPSW, L=0 is STGP
        // (FEAT_MTE store-allocation-tag pair). STGP stores two 64-bit
        // registers; the tag write is a no-op in our flat memory model, and
        // its immediate is scaled by the 16-byte tag granule (LSL #4).
        // STGP has no no-allocate (STNP-style) form.
        let stgp = v == 0 && opc == 0b01 && l == 0 && mode != 0b00;
        if stgp {
            let off = (((imm7 << 25) >> 25) as i64) * 16;
            let base = if rn == 31 {
                self.current_sp()
            } else {
                self.get_x(rn)
            };
            let addr = if mode == 0b01 {
                base
            } else {
                (base as i64).wrapping_add(off) as u64
            };
            self.mem_write_u64(addr, self.get_x(rt))?;
            self.mem_write_u64(addr.wrapping_add(8), self.get_x(rt2))?;
            if mode == 0b01 || mode == 0b11 {
                let nb = (base as i64).wrapping_add(off) as u64;
                if rn == 31 {
                    self.set_current_sp(nb);
                } else {
                    self.set_x(rn, nb);
                }
            }
            return Ok(CpuExit::Continue);
        }
        if ldpsw && l == 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let offset = (((imm7 << 25) >> 25) as i64) * (bytes as i64);
        let wback = mode == 0b01 || mode == 0b11;
        let postindex = mode == 0b01;

        let base = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };
        let address = if postindex {
            base
        } else {
            (base as i64).wrapping_add(offset) as u64
        };
        let addr2 = address.wrapping_add(bytes as u64);

        if v != 0 {
            if l != 0 {
                let mut b1 = [0u8; 16];
                let mut b2 = [0u8; 16];
                for i in 0..bytes {
                    b1[i] = self.mem_read_u8(address + i as u64)?;
                    b2[i] = self.mem_read_u8(addr2 + i as u64)?;
                }
                self.v[rt as usize] = u128::from_le_bytes(b1);
                self.v[rt2 as usize] = u128::from_le_bytes(b2);
            } else {
                let v1 = self.v[rt as usize].to_le_bytes();
                let v2 = self.v[rt2 as usize].to_le_bytes();
                for i in 0..bytes {
                    self.mem_write_u8(address + i as u64, v1[i])?;
                    self.mem_write_u8(addr2 + i as u64, v2[i])?;
                }
            }
        } else if bytes == 4 {
            if l != 0 {
                let val1 = self.mem_read_u32(address)?;
                let val2 = self.mem_read_u32(addr2)?;
                if ldpsw {
                    self.set_x(rt, val1 as i32 as i64 as u64);
                    self.set_x(rt2, val2 as i32 as i64 as u64);
                } else {
                    self.set_w(rt, val1);
                    self.set_w(rt2, val2);
                }
            } else {
                self.mem_write_u32(address, self.get_w(rt))?;
                self.mem_write_u32(addr2, self.get_w(rt2))?;
            }
        } else if l != 0 {
            self.set_x(rt, self.mem_read_u64(address)?);
            self.set_x(rt2, self.mem_read_u64(addr2)?);
        } else {
            self.mem_write_u64(address, self.get_x(rt))?;
            self.mem_write_u64(addr2, self.get_x(rt2))?;
        }

        if wback {
            let new_base = (base as i64).wrapping_add(offset) as u64;
            if rn == 31 {
                self.set_current_sp(new_base);
            } else {
                self.set_x(rn, new_base);
            }
        }

        Ok(CpuExit::Continue)
    }

    /// Advanced SIMD load/store single structure: one element to/from a lane of
    /// `selem` consecutive registers (LD1-LD4 by element), and the replicating
    /// loads LD1R-LD4R (broadcast one element across all lanes).
    fn exec_ldst_single(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let post = (insn >> 23) & 1;
        let l = (insn >> 22) & 1;
        let r = (insn >> 21) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let opcode = (insn >> 13) & 0x7;
        let s_bit = (insn >> 12) & 1;
        let size = (insn >> 10) & 0x3;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rt = (insn & 0x1F) as usize;

        let scale = opcode >> 1; // bits[15:14]
        let selem = (((opcode & 1) << 1) | r) as usize + 1;

        let (esize, index, replicate) = match scale {
            0b00 => (8u32, ((q << 3) | (s_bit << 2) | size) as usize, false),
            0b01 => {
                if size & 1 != 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                (16, ((q << 2) | (s_bit << 1) | (size >> 1)) as usize, false)
            }
            0b10 => {
                if size & 2 != 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                if size & 1 == 0 {
                    (32, ((q << 1) | s_bit) as usize, false)
                } else {
                    if s_bit != 0 {
                        return Err(ArmError::UndefinedInstruction(insn));
                    }
                    (64, q as usize, false)
                }
            }
            _ => {
                // Replicate (LD1R-LD4R): load-only, S must be 0.
                if l == 0 || s_bit != 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
                (8u32 << size, 0usize, true)
            }
        };
        let ebytes = (esize / 8) as u64;
        let datasize = if q == 1 { 16usize } else { 8 };
        let emask = elem_mask_u128(esize);

        let base = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };
        let mut addr = base;

        for sct in 0..selem {
            let reg = (rt + sct) % 32;
            if replicate {
                let mut bytes = [0u8; 8];
                for (b, slot) in bytes.iter_mut().enumerate().take(ebytes as usize) {
                    *slot = self.mem_read_u8(addr + b as u64)?;
                }
                let val = u64::from_le_bytes(bytes) as u128 & emask;
                let elements = datasize / ebytes as usize;
                let mut result = 0u128;
                for e in 0..elements {
                    result |= val << (e * esize as usize);
                }
                self.v[reg] = result;
            } else {
                let shift = index * esize as usize;
                if l != 0 {
                    let mut bytes = [0u8; 8];
                    for (b, slot) in bytes.iter_mut().enumerate().take(ebytes as usize) {
                        *slot = self.mem_read_u8(addr + b as u64)?;
                    }
                    let val = u64::from_le_bytes(bytes) as u128 & emask;
                    self.v[reg] = (self.v[reg] & !(emask << shift)) | (val << shift);
                } else {
                    let val = (self.v[reg] >> shift) & emask;
                    for b in 0..ebytes as usize {
                        self.mem_write_u8(addr + b as u64, (val >> (b * 8)) as u8)?;
                    }
                }
            }
            addr += ebytes;
        }

        if post != 0 {
            let inc = if rm == 31 {
                selem as u64 * ebytes
            } else {
                self.get_x(rm)
            };
            let new = base.wrapping_add(inc);
            if rn == 31 {
                self.set_current_sp(new);
            } else {
                self.set_x(rn, new);
            }
        }
        Ok(CpuExit::Continue)
    }

    /// Advanced SIMD load/store multiple structures: LD1/ST1 (1-4 registers),
    /// LD2/ST2, LD3/ST3, LD4/ST4 (de-interleaving). Contiguous, optional
    /// post-index writeback.
    fn exec_ldst_structures(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let q = (insn >> 30) & 1;
        let post = (insn >> 23) & 1;
        let l = (insn >> 22) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let opcode = (insn >> 12) & 0xF;
        let size = (insn >> 10) & 0x3;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rt = (insn & 0x1F) as usize;

        // (rpt, selem): number of register groups and structure size.
        let (rpt, selem): (usize, usize) = match opcode {
            0b0000 => (1, 4), // LD4/ST4
            0b0010 => (4, 1), // LD1 x4
            0b0100 => (1, 3), // LD3/ST3
            0b0110 => (3, 1), // LD1 x3
            0b0111 => (1, 1), // LD1 x1
            0b1000 => (1, 2), // LD2/ST2
            0b1010 => (2, 1), // LD1 x2
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };
        // A single 64-bit element (1D, size=11 with Q=0) is only valid when the
        // structure spans a single register per group.
        if size == 0b11 && q == 0 && selem != 1 && rpt == 1 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let esize = 8u32 << size; // bits
        let ebytes = (esize / 8) as u64;
        let datasize = if q == 1 { 16usize } else { 8 };
        let elements = datasize / ebytes as usize;
        let nregs = rpt * selem;

        let base = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };
        let mut addr = base;

        // Loads rewrite each touched register fully (upper bits zeroed for Q=0).
        if l != 0 {
            for i in 0..nregs {
                self.v[(rt + i) % 32] = 0;
            }
        }
        let emask = elem_mask_u128(esize);
        for r in 0..rpt {
            for e in 0..elements {
                for sct in 0..selem {
                    let reg = (rt + r * selem + sct) % 32;
                    let shift = e * esize as usize;
                    if l != 0 {
                        let mut bytes = [0u8; 8];
                        for (b, slot) in bytes.iter_mut().enumerate().take(ebytes as usize) {
                            *slot = self.mem_read_u8(addr + b as u64)?;
                        }
                        let val = u64::from_le_bytes(bytes) as u128 & emask;
                        self.v[reg] = (self.v[reg] & !(emask << shift)) | (val << shift);
                    } else {
                        let val = (self.v[reg] >> shift) & emask;
                        for b in 0..ebytes as usize {
                            self.mem_write_u8(addr + b as u64, (val >> (b * 8)) as u8)?;
                        }
                    }
                    addr += ebytes;
                }
            }
        }

        if post != 0 {
            let inc = if rm == 31 {
                (nregs * elements) as u64 * ebytes
            } else {
                self.get_x(rm)
            };
            let new = base.wrapping_add(inc);
            if rn == 31 {
                self.set_current_sp(new);
            } else {
                self.set_x(rn, new);
            }
        }
        Ok(CpuExit::Continue)
    }

    /// Atomic memory operations (FEAT_LSE): LDADD/LDCLR/LDEOR/LDSET/LDSMAX/
    /// LDSMIN/LDUMAX/LDUMIN and SWP. Single-core, so the load-op-store is just
    /// sequential. Rt receives the pre-operation value (discarded if Rt==31).
    fn exec_atomic_memop(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let size = (insn >> 30) & 0x3;
        let rs = ((insn >> 16) & 0x1F) as u8;
        let o3 = (insn >> 15) & 1;
        let opc = (insn >> 12) & 0x7;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rt = (insn & 0x1F) as u8;
        let bits = 8u32 << size;
        let m = elem_mask(bits);

        let addr = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };
        let old = match size {
            0 => self.mem_read_u8(addr)? as u64,
            1 => self.mem_read_u16(addr)? as u64,
            2 => self.mem_read_u32(addr)? as u64,
            _ => self.mem_read_u64(addr)?,
        };
        let operand = self.get_x(rs) & m;

        let new = if o3 == 1 {
            if opc == 0 {
                operand // SWP
            } else if opc == 0b100 && rs == 31 && (insn >> 23) & 1 == 1 && (insn >> 22) & 1 == 0
            {
                // LDAPR/LDAPRB/LDAPRH (FEAT_LRCPC): load-acquire RCpc. In a
                // single-threaded model this is a plain load.
                if size == 3 {
                    self.set_x(rt, old);
                } else {
                    self.set_w(rt, old as u32);
                }
                return Ok(CpuExit::Continue);
            } else {
                return Err(ArmError::UndefinedInstruction(insn));
            }
        } else {
            match opc {
                0b000 => old.wrapping_add(operand), // LDADD
                0b001 => old & !operand,            // LDCLR
                0b010 => old ^ operand,             // LDEOR
                0b011 => old | operand,             // LDSET
                0b100 => (sext_elem(old, bits).max(sext_elem(operand, bits)) as u64) & m, // LDSMAX
                0b101 => (sext_elem(old, bits).min(sext_elem(operand, bits)) as u64) & m, // LDSMIN
                0b110 => (old & m).max(operand & m), // LDUMAX
                0b111 => (old & m).min(operand & m), // LDUMIN
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            }
        };
        let new = new & m;
        match size {
            0 => self.mem_write_u8(addr, new as u8)?,
            1 => self.mem_write_u16(addr, new as u16)?,
            2 => self.mem_write_u32(addr, new as u32)?,
            _ => self.mem_write_u64(addr, new)?,
        }
        if rt != 31 {
            if size == 3 {
                self.set_x(rt, old);
            } else {
                self.set_w(rt, old as u32);
            }
        }
        Ok(CpuExit::Continue)
    }

    fn exec_ldst_reg(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let size = (insn >> 30) & 0x3;
        let v = (insn >> 26) & 1;
        let opc = (insn >> 22) & 0x3;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rt = (insn & 0x1F) as u8;

        // Atomic memory operations (FEAT_LSE): bit24=0, bit21=1, bits[11:10]=00.
        if v == 0 && (insn >> 24) & 1 == 0 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0 {
            return self.exec_atomic_memop(insn);
        }

        if v != 0 {
            // SIMD/FP load/store: access size is 1 << ((opc<1>:size)).
            let scale = (((opc >> 1) & 1) << 2) | size;
            if scale > 4 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
            // Register-offset form: the extend option must have bit 1 set
            // (UXTW/LSL/SXTW/SXTX); other options are unallocated.
            if (insn >> 24) & 1 == 0 && (insn >> 21) & 1 == 1 && (insn >> 10) & 0x3 == 0b10 {
                let option = (insn >> 13) & 0x7;
                if option & 0b010 == 0 {
                    return Err(ArmError::UndefinedInstruction(insn));
                }
            }
            let access = 1usize << scale;
            let is_load = (opc & 1) == 1;
            let (address, wback, wback_value) = self.decode_address(insn, rn, scale)?;
            if is_load {
                let mut bytes = [0u8; 16];
                for (i, b) in bytes.iter_mut().enumerate().take(access) {
                    *b = self.mem_read_u8(address + i as u64)?;
                }
                self.v[rt as usize] = u128::from_le_bytes(bytes);
            } else {
                let val = self.v[rt as usize].to_le_bytes();
                for (i, b) in val.iter().enumerate().take(access) {
                    self.mem_write_u8(address + i as u64, *b)?;
                }
            }
            if wback {
                if rn == 31 {
                    self.set_current_sp(wback_value);
                } else {
                    self.set_x(rn, wback_value);
                }
            }
            return Ok(CpuExit::Continue);
        }

        let bit24 = (insn >> 24) & 1;
        let bit21 = (insn >> 21) & 1;
        let op2 = (insn >> 10) & 0x3;
        if size == 0b11 && opc >= 0b10 {
            let is_unsigned_offset = bit24 == 1;
            let is_signed_offset = bit24 == 0 && bit21 == 0 && op2 == 0b00;
            let is_register_offset = bit24 == 0 && bit21 == 1 && op2 == 0b10;

            if opc == 0b10 && (is_unsigned_offset || is_signed_offset || is_register_offset) {
                if is_register_offset {
                    let option = ((insn >> 13) & 0x7) as u8;
                    if option & 0b010 == 0 {
                        return Err(ArmError::UndefinedInstruction(insn));
                    }
                }
                return Ok(CpuExit::Continue);
            }

            return Err(ArmError::UndefinedInstruction(insn));
        }
        if size == 0b10 && opc == 0b11 {
            return Err(ArmError::UndefinedInstruction(insn));
        }
        if bit24 == 0 && bit21 == 1 && op2 == 0b10 {
            let option = ((insn >> 13) & 0x7) as u8;
            if option & 0b010 == 0 {
                return Err(ArmError::UndefinedInstruction(insn));
            }
        }

        // Determine addressing mode
        let (address, wback, wback_value) = self.decode_address(insn, rn, size)?;

        let is_load = (opc & 1) != 0 || opc == 0b10;
        let is_signed = opc >= 0b10;

        if is_load {
            let value = match size {
                0b00 => {
                    let v = self.mem_read_u8(address)?;
                    if is_signed && opc == 0b11 {
                        v as i8 as i64 as u64
                    } else if is_signed {
                        v as i8 as i32 as u64
                    } else {
                        v as u64
                    }
                }
                0b01 => {
                    let v = self.mem_read_u16(address)?;
                    if is_signed && opc == 0b11 {
                        v as i16 as i64 as u64
                    } else if is_signed {
                        v as i16 as i32 as u64
                    } else {
                        v as u64
                    }
                }
                0b10 => {
                    let v = self.mem_read_u32(address)?;
                    if is_signed {
                        v as i32 as i64 as u64
                    } else {
                        v as u64
                    }
                }
                0b11 => self.mem_read_u64(address)?,
                _ => unreachable!(),
            };

            if size == 0b11 || (is_signed && opc == 0b10) {
                self.set_x(rt, value);
            } else {
                self.set_w(rt, value as u32);
            }
        } else {
            // Store
            match size {
                0b00 => self.mem_write_u8(address, self.get_w(rt) as u8)?,
                0b01 => self.mem_write_u16(address, self.get_w(rt) as u16)?,
                0b10 => self.mem_write_u32(address, self.get_w(rt))?,
                0b11 => self.mem_write_u64(address, self.get_x(rt))?,
                _ => unreachable!(),
            }
        }

        // Writeback
        if wback {
            if rn == 31 {
                self.set_current_sp(wback_value);
            } else {
                self.set_x(rn, wback_value);
            }
        }

        Ok(CpuExit::Continue)
    }

    /// Decode addressing mode for load/store. `scale` is the log2 of the access
    /// size in bytes (used to scale the unsigned/register offsets).
    fn decode_address(&self, insn: u32, rn: u8, scale: u32) -> Result<(u64, bool, u64), ArmError> {
        let base = if rn == 31 {
            self.current_sp()
        } else {
            self.get_x(rn)
        };

        // Unsigned offset form: selected by bit 24 alone. Bit 21 is the top
        // bit of imm12 here, NOT a mode selector — gating on it sent every
        // access with an offset >= 0x800 << scale down the indexed path,
        // where the imm12 payload was misread as a post-index writeback.
        if (insn >> 24) & 1 != 0 {
            let imm12 = ((insn >> 10) & 0xFFF) as u64;
            let offset = imm12 << scale;
            return Ok((base.wrapping_add(offset), false, 0));
        }

        // Check addressing mode
        let bit21 = (insn >> 21) & 1;
        let op4 = (insn >> 10) & 0x3;

        match op4 {
            0b00 => {
                // Unscaled immediate
                let imm9 = ((insn >> 12) & 0x1FF) as i32;
                let offset = ((imm9 << 23) >> 23) as i64;
                Ok(((base as i64).wrapping_add(offset) as u64, false, 0))
            }
            0b01 => {
                // Immediate post-indexed
                let imm9 = ((insn >> 12) & 0x1FF) as i32;
                let offset = ((imm9 << 23) >> 23) as i64;
                Ok((base, true, (base as i64).wrapping_add(offset) as u64))
            }
            0b10 if bit21 == 0 => {
                // Unprivileged signed immediate offset
                let imm9 = ((insn >> 12) & 0x1FF) as i32;
                let offset = ((imm9 << 23) >> 23) as i64;
                Ok(((base as i64).wrapping_add(offset) as u64, false, 0))
            }
            0b10 => {
                // Register offset
                let rm = ((insn >> 16) & 0x1F) as u8;
                let option = ((insn >> 13) & 0x7) as u8;
                let s = ((insn >> 12) & 1) != 0;

                let offset = self.extend_reg(rm, option, if s { scale } else { 0 })?;
                Ok((base.wrapping_add(offset), false, 0))
            }
            0b11 => {
                // Immediate pre-indexed
                let imm9 = ((insn >> 12) & 0x1FF) as i32;
                let offset = ((imm9 << 23) >> 23) as i64;
                let addr = (base as i64).wrapping_add(offset) as u64;
                Ok((addr, true, addr))
            }
            _ => unreachable!(),
        }
    }

    /// Extend register with optional shift.
    fn extend_reg(&self, rm: u8, option: u8, shift: u32) -> Result<u64, ArmError> {
        let val = self.get_x(rm);

        let extended = match option {
            0b000 => (val as u8) as u64,                // UXTB
            0b001 => (val as u16) as u64,               // UXTH
            0b010 => (val as u32) as u64,               // UXTW
            0b011 => val,                               // UXTX
            0b100 => (val as u8 as i8 as i64) as u64,   // SXTB
            0b101 => (val as u16 as i16 as i64) as u64, // SXTH
            0b110 => (val as u32 as i32 as i64) as u64, // SXTW
            0b111 => val,                               // SXTX
            _ => return Err(ArmError::UndefinedInstruction(0)),
        };

        Ok(extended << shift)
    }

    // Data processing (register) implementations
    fn exec_logical_shifted(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let opc = (insn >> 29) & 0x3;
        let shift = ((insn >> 22) & 0x3) as u32;
        let n = (insn >> 21) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let imm6 = ((insn >> 10) & 0x3F) as u32;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        // For 32-bit forms a shift amount with bit 5 set is UNDEFINED.
        if sf == 0 && (imm6 & 0x20) != 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let operand1 = if sf != 0 {
            self.get_x(rn)
        } else {
            self.get_w(rn) as u64
        };

        let mut operand2 = if sf != 0 {
            self.get_x(rm)
        } else {
            self.get_w(rm) as u64
        };

        // Apply shift at the correct datasize (32 or 64 bits).
        operand2 = if sf != 0 {
            match shift {
                0b00 => operand2 << imm6,                   // LSL
                0b01 => operand2 >> imm6,                   // LSR
                0b10 => ((operand2 as i64) >> imm6) as u64, // ASR
                0b11 => operand2.rotate_right(imm6),        // ROR
                _ => unreachable!(),
            }
        } else {
            let v = operand2 as u32;
            (match shift {
                0b00 => v << imm6,                   // LSL
                0b01 => v >> imm6,                   // LSR
                0b10 => ((v as i32) >> imm6) as u32, // ASR
                0b11 => v.rotate_right(imm6),        // ROR
                _ => unreachable!(),
            }) as u64
        };

        if sf == 0 {
            operand2 &= 0xFFFF_FFFF;
        }

        // Invert if N bit set
        if n != 0 {
            operand2 = !operand2;
            if sf == 0 {
                operand2 &= 0xFFFF_FFFF;
            }
        }

        let result = match opc {
            0b00 => operand1 & operand2, // AND / BIC
            0b01 => operand1 | operand2, // ORR / ORN
            0b10 => operand1 ^ operand2, // EOR / EON
            0b11 => operand1 & operand2, // ANDS / BICS
            _ => unreachable!(),
        };

        if opc == 0b11 {
            if sf != 0 {
                self.update_nz_64(result);
            } else {
                self.update_nz_32(result as u32);
            }
            self.set_c(false);
            self.set_v(false);
        }

        if sf != 0 {
            self.set_x(rd, result);
        } else {
            self.set_w(rd, result as u32);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_add_sub_shifted_ext(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op = (insn >> 30) & 1;
        let s = (insn >> 29) & 1;
        let extended = (insn >> 21) & 1; // bit 21 distinguishes shifted (0) from extended (1)
        let rm = ((insn >> 16) & 0x1F) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        if extended == 0 {
            // Shifted register
            let shift = ((insn >> 22) & 0x3) as u32;
            let imm6 = ((insn >> 10) & 0x3F) as u32;

            // ROR is not a valid shift for add/sub, and 32-bit forms with bit 5
            // of the shift amount set are UNDEFINED.
            if shift == 0b11 || (sf == 0 && (imm6 & 0x20) != 0) {
                return Err(ArmError::UndefinedInstruction(insn));
            }

            let operand1 = if sf != 0 {
                self.get_x(rn)
            } else {
                self.get_w(rn) as u64
            };

            let mut operand2 = if sf != 0 {
                self.get_x(rm)
            } else {
                self.get_w(rm) as u64
            };

            operand2 = match shift {
                0b00 => operand2 << imm6,
                0b01 => {
                    if sf != 0 {
                        operand2 >> imm6
                    } else {
                        // 32-bit LSR: shift the 32-bit value, not the zero-extended u64.
                        ((operand2 as u32) >> imm6) as u64
                    }
                }
                0b10 => {
                    if sf != 0 {
                        ((operand2 as i64) >> imm6) as u64
                    } else {
                        // 32-bit ASR: sign-extend from bit 31 before shifting.
                        (((operand2 as u32 as i32 as i64) >> imm6) as u64)
                    }
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };

            if sf == 0 {
                operand2 &= 0xFFFF_FFFF;
            }

            let (result, carry, overflow) = if op == 0 {
                // ADD
                if sf != 0 {
                    let (r, c) = operand1.overflowing_add(operand2);
                    let v = (!(operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                    (r, c, v)
                } else {
                    let o1 = operand1 as u32;
                    let o2 = operand2 as u32;
                    let (r, c) = o1.overflowing_add(o2);
                    let v = (!(o1 ^ o2) & (o1 ^ r)) >> 31 != 0;
                    (r as u64, c, v)
                }
            } else {
                // SUB
                if sf != 0 {
                    let (r, c) = operand1.overflowing_sub(operand2);
                    let v = ((operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                    (r, !c, v)
                } else {
                    let o1 = operand1 as u32;
                    let o2 = operand2 as u32;
                    let (r, c) = o1.overflowing_sub(o2);
                    let v = ((o1 ^ o2) & (o1 ^ r)) >> 31 != 0;
                    (r as u64, !c, v)
                }
            };

            if s != 0 {
                if sf != 0 {
                    self.update_nz_64(result);
                } else {
                    self.update_nz_32(result as u32);
                }
                self.set_c(carry);
                self.set_v(overflow);
            }

            if rd == 31 && s == 0 {
                self.set_current_sp(result);
            } else if sf != 0 {
                self.set_x(rd, result);
            } else {
                self.set_w(rd, result as u32);
            }
        } else {
            // Extended register
            let option = ((insn >> 13) & 0x7) as u8;
            let imm3 = ((insn >> 10) & 0x7) as u32;

            let operand1 = if rn == 31 {
                self.current_sp()
            } else {
                self.get_x(rn)
            };

            let operand2 = self.extend_reg(rm, option, imm3)?;

            let (result, carry, overflow) = if op == 0 {
                let (r, c) = operand1.overflowing_add(operand2);
                let v = (!(operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                (r, c, v)
            } else {
                let (r, c) = operand1.overflowing_sub(operand2);
                let v = ((operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                (r, !c, v)
            };

            if s != 0 {
                if sf != 0 {
                    self.update_nz_64(result);
                } else {
                    self.update_nz_32(result as u32);
                }
                self.set_c(carry);
                self.set_v(overflow);
            }

            if rd == 31 && s == 0 {
                self.set_current_sp(result);
            } else if sf != 0 {
                self.set_x(rd, result);
            } else {
                self.set_w(rd, result as u32);
            }
        }

        Ok(CpuExit::Continue)
    }

    fn exec_adc_sbc(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op = (insn >> 30) & 1;
        let s = (insn >> 29) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        // RMIF Xn, #imm6, #mask (FEAT_FlagM): rotate Xn right by imm6 and move
        // the low four bits into the NZCV flags selected by mask.
        if sf == 1 && op == 0 && s == 1 && (insn >> 10) & 0x1F == 0b00001 && (insn >> 4) & 1 == 0 {
            let imm6 = (insn >> 15) & 0x3F;
            let mask = insn & 0xF;
            let val = self.get_x(rn).rotate_right(imm6);
            let mut n = self.get_n();
            let mut z = self.get_z();
            let mut c = self.get_c();
            let mut v = self.get_v();
            if mask & 0b1000 != 0 {
                n = (val >> 3) & 1 == 1;
            }
            if mask & 0b0100 != 0 {
                z = (val >> 2) & 1 == 1;
            }
            if mask & 0b0010 != 0 {
                c = (val >> 1) & 1 == 1;
            }
            if mask & 0b0001 != 0 {
                v = val & 1 == 1;
            }
            self.set_nzcv(n, z, c, v);
            return Ok(CpuExit::Continue);
        }

        // SETF8/SETF16 Wn (FEAT_FlagM): set NZV from a narrow value, C
        // unchanged. N = sign bit, Z = narrow value == 0, V = bit(width) XOR
        // bit(width-1).
        if sf == 0
            && op == 0
            && s == 1
            && (insn >> 10) & 0xF == 0b0010
            && (insn >> 15) & 0x3F == 0
            && insn & 0x1F == 0b01101
        {
            let width = if (insn >> 14) & 1 == 1 { 16 } else { 8 };
            let w = self.get_w(rn);
            let n = (w >> (width - 1)) & 1 == 1;
            let z = w & ((1u32 << width) - 1) == 0;
            let v = ((w >> width) & 1) != ((w >> (width - 1)) & 1);
            let c = self.get_c();
            self.set_nzcv(n, z, c, v);
            return Ok(CpuExit::Continue);
        }

        // ADC/ADCS/SBC/SBCS require bits[15:10] == 0; anything else in this
        // space is unallocated.
        if (insn >> 10) & 0x3F != 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let c_in = if self.get_c() { 1u64 } else { 0 };

        if sf != 0 {
            let operand1 = self.get_x(rn);
            let operand2 = self.get_x(rm);

            let (result, carry, overflow) = if op == 0 {
                // ADC
                let (r1, c1) = operand1.overflowing_add(operand2);
                let (r, c2) = r1.overflowing_add(c_in);
                let v = (!(operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                (r, c1 || c2, v)
            } else {
                // SBC
                let not_c = if self.get_c() { 0u64 } else { 1 };
                let (r1, c1) = operand1.overflowing_sub(operand2);
                let (r, c2) = r1.overflowing_sub(not_c);
                let v = ((operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                (r, !(c1 || c2), v)
            };

            if s != 0 {
                self.update_nz_64(result);
                self.set_c(carry);
                self.set_v(overflow);
            }

            self.set_x(rd, result);
        } else {
            let operand1 = self.get_w(rn);
            let operand2 = self.get_w(rm);
            let c_in = c_in as u32;

            let (result, carry, overflow) = if op == 0 {
                let (r1, c1) = operand1.overflowing_add(operand2);
                let (r, c2) = r1.overflowing_add(c_in);
                let v = (!(operand1 ^ operand2) & (operand1 ^ r)) >> 31 != 0;
                (r, c1 || c2, v)
            } else {
                let not_c = if self.get_c() { 0u32 } else { 1 };
                let (r1, c1) = operand1.overflowing_sub(operand2);
                let (r, c2) = r1.overflowing_sub(not_c);
                let v = ((operand1 ^ operand2) & (operand1 ^ r)) >> 31 != 0;
                (r, !(c1 || c2), v)
            };

            if s != 0 {
                self.update_nz_32(result);
                self.set_c(carry);
                self.set_v(overflow);
            }

            self.set_w(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_ccmp_ccmn(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op = (insn >> 30) & 1; // 0=CCMN, 1=CCMP
        let imm_or_reg = (insn >> 11) & 1;
        let rm_imm5 = ((insn >> 16) & 0x1F) as u8;
        let cond = ((insn >> 12) & 0xF) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let nzcv = (insn & 0xF) as u8;

        if self.condition_holds(cond) {
            let operand2 = if imm_or_reg != 0 {
                rm_imm5 as u64
            } else {
                if sf != 0 {
                    self.get_x(rm_imm5)
                } else {
                    self.get_w(rm_imm5) as u64
                }
            };

            if sf != 0 {
                let operand1 = self.get_x(rn);
                let (result, carry, overflow) = if op == 0 {
                    // CCMN (add)
                    let (r, c) = operand1.overflowing_add(operand2);
                    let v = (!(operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                    (r, c, v)
                } else {
                    // CCMP (sub)
                    let (r, c) = operand1.overflowing_sub(operand2);
                    let v = ((operand1 ^ operand2) & (operand1 ^ r)) >> 63 != 0;
                    (r, !c, v)
                };
                self.update_nz_64(result);
                self.set_c(carry);
                self.set_v(overflow);
            } else {
                let operand1 = self.get_w(rn);
                let operand2 = operand2 as u32;
                let (result, carry, overflow) = if op == 0 {
                    let (r, c) = operand1.overflowing_add(operand2);
                    let v = (!(operand1 ^ operand2) & (operand1 ^ r)) >> 31 != 0;
                    (r, c, v)
                } else {
                    let (r, c) = operand1.overflowing_sub(operand2);
                    let v = ((operand1 ^ operand2) & (operand1 ^ r)) >> 31 != 0;
                    (r, !c, v)
                };
                self.update_nz_32(result);
                self.set_c(carry);
                self.set_v(overflow);
            }
        } else {
            self.nzcv = nzcv;
        }

        Ok(CpuExit::Continue)
    }

    fn exec_csel(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op = (insn >> 30) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let cond = ((insn >> 12) & 0xF) as u8;
        let op2 = (insn >> 10) & 0x3;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        let cond_met = self.condition_holds(cond);

        if sf != 0 {
            let operand1 = self.get_x(rn);
            let operand2 = self.get_x(rm);

            let result = if cond_met {
                operand1
            } else {
                match (op, op2) {
                    (0, 0) => operand2,                 // CSEL
                    (0, 1) => operand2.wrapping_add(1), // CSINC
                    (1, 0) => !operand2,                // CSINV
                    (1, 1) => operand2.wrapping_neg(),  // CSNEG
                    _ => unreachable!(),
                }
            };

            self.set_x(rd, result);
        } else {
            let operand1 = self.get_w(rn);
            let operand2 = self.get_w(rm);

            let result = if cond_met {
                operand1
            } else {
                match (op, op2) {
                    (0, 0) => operand2,
                    (0, 1) => operand2.wrapping_add(1),
                    (1, 0) => !operand2,
                    (1, 1) => operand2.wrapping_neg(),
                    _ => unreachable!(),
                }
            };

            self.set_w(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_dp_1src(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let s = (insn >> 29) & 1;
        let opcode2 = (insn >> 16) & 0x1F;
        let opcode = (insn >> 10) & 0x3F;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        if s != 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        // FEAT_PAuth one-source (opcode2 = 00001). No real key schedule:
        // PAC*/AUT* leave the pointer logically intact (authenticate is the
        // inverse of sign), XPAC strips the auth field. The opcode field
        // (bits[15:10]) selects the variant: 0b000xxx take a modifier in Rn,
        // 0b001xxx are the "Z" forms (modifier 0, require Rn=11111), and
        // 0b0100xx are XPACI/XPACD (require Rn=11111).
        if sf == 1 && opcode2 == 0b00001 {
            let strip = |v: u64| v & 0x007F_FFFF_FFFF_FFFF;
            match opcode {
                // PACIA/IB/DA/DB, AUTIA/IB/DA/DB Xd, Xn|SP
                0b000000..=0b000111 => {
                    let v = self.get_x(rd);
                    self.set_x(rd, v);
                    return Ok(CpuExit::Continue);
                }
                // PACIZA/.../AUTDZB Xd (Rn must be 11111)
                0b001000..=0b001111 => {
                    if rn != 31 {
                        return Ok(CpuExit::Undefined(insn));
                    }
                    let v = self.get_x(rd);
                    self.set_x(rd, v);
                    return Ok(CpuExit::Continue);
                }
                // XPACI / XPACD Xd (Rn must be 11111)
                0b010000 | 0b010001 => {
                    if rn != 31 {
                        return Ok(CpuExit::Undefined(insn));
                    }
                    let v = strip(self.get_x(rd));
                    self.set_x(rd, v);
                    return Ok(CpuExit::Continue);
                }
                _ => {}
            }
        }

        if sf != 0 {
            let operand = self.get_x(rn);
            let result = match opcode {
                0b000000 => operand.reverse_bits(), // RBIT
                0b000001 => {
                    // REV16
                    ((operand & 0x00ff_00ff_00ff_00ff) << 8)
                        | ((operand & 0xff00_ff00_ff00_ff00) >> 8)
                }
                0b000010 => {
                    // REV32
                    ((operand & 0x0000_00ff) << 24)
                        | ((operand & 0x0000_ff00) << 8)
                        | ((operand & 0x00ff_0000) >> 8)
                        | ((operand & 0xff00_0000) >> 24)
                        | ((operand & 0x0000_00ff_0000_0000) << 24)
                        | ((operand & 0x0000_ff00_0000_0000) << 8)
                        | ((operand & 0x00ff_0000_0000_0000) >> 8)
                        | ((operand & 0xff00_0000_0000_0000) >> 24)
                }
                0b000011 => operand.swap_bytes(), // REV
                0b000100 => u64::from(operand.leading_zeros()), // CLZ
                0b000101 => count_leading_sign(operand, 64), // CLS
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            self.set_x(rd, result);
        } else {
            let operand = self.get_w(rn);
            let result = match opcode {
                0b000000 => operand.reverse_bits(), // RBIT
                0b000001 => {
                    // REV16
                    ((operand & 0x00ff_00ff) << 8) | ((operand & 0xff00_ff00) >> 8)
                }
                0b000010 => operand.swap_bytes(), // REV
                0b000100 => operand.leading_zeros(), // CLZ
                0b000101 => count_leading_sign(u64::from(operand), 32) as u32, // CLS
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };
            self.set_w(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_dp_2src(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let s = (insn >> 29) & 1;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let opcode = (insn >> 10) & 0x3F;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        // FEAT_MTE / FEAT_PAuth data-processing (2-source). These all require
        // sf=1 (64-bit). The flag-setting SUBPS aside, the rest leave NZCV.
        if sf == 1 {
            match (s, opcode) {
                // SUBP Xd, Xn|SP, Xm|SP — subtract address tags ignored
                // (bits[63:56] of each operand are sign-extended from bit 55).
                (0, 0b000000) => {
                    let a = sign_extend_56(self.gpr_or_sp(rn));
                    let b = sign_extend_56(self.gpr_or_sp(rm));
                    self.set_x(rd, a.wrapping_sub(b));
                    return Ok(CpuExit::Continue);
                }
                // SUBPS Xd, Xn|SP, Xm|SP — as SUBP, sets NZCV.
                (1, 0b000000) => {
                    let a = sign_extend_56(self.gpr_or_sp(rn));
                    let b = sign_extend_56(self.gpr_or_sp(rm));
                    let (res, n, z, c, v) = sub_with_flags_64(a, b);
                    self.set_nzcv(n, z, c, v);
                    self.set_x(rd, res);
                    return Ok(CpuExit::Continue);
                }
                // IRG Xd|SP, Xn|SP, Xm — insert random (here: deterministic 0)
                // tag, honouring nothing in GCR_EL1. Tag bits are [59:56].
                (0, 0b000100) => {
                    let v = self.gpr_or_sp(rn) & !(0xFu64 << 56);
                    self.set_gpr_or_sp(rd, v);
                    return Ok(CpuExit::Continue);
                }
                // GMI Xd, Xn|SP, Xm — tag mask insert: Xd = Xm | (1 << tag(Xn)).
                (0, 0b000101) => {
                    let tag = (self.gpr_or_sp(rn) >> 56) & 0xF;
                    self.set_x(rd, self.get_x(rm) | (1u64 << tag));
                    return Ok(CpuExit::Continue);
                }
                // PACGA Xd, Xn, Xm|SP — generic pointer-auth MAC in bits[63:32].
                // No real key schedule; produce a deterministic non-zero MAC so
                // the destination is written.
                (0, 0b001100) => {
                    let mac = pacga_stub(self.get_x(rn), self.gpr_or_sp(rm));
                    self.set_x(rd, mac);
                    return Ok(CpuExit::Continue);
                }
                _ => {}
            }
        }

        if s != 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        if sf != 0 {
            let operand1 = self.get_x(rn);
            let operand2 = self.get_x(rm);

            let result = match opcode {
                0b000010 => {
                    // UDIV
                    if operand2 == 0 {
                        0
                    } else {
                        operand1 / operand2
                    }
                }
                0b000011 => {
                    // SDIV
                    if operand2 == 0 {
                        0
                    } else {
                        (operand1 as i64).wrapping_div(operand2 as i64) as u64
                    }
                }
                0b001000 => {
                    // LSLV
                    let shift = (operand2 & 0x3F) as u32;
                    operand1 << shift
                }
                0b001001 => {
                    // LSRV
                    let shift = (operand2 & 0x3F) as u32;
                    operand1 >> shift
                }
                0b001010 => {
                    // ASRV
                    let shift = (operand2 & 0x3F) as u32;
                    ((operand1 as i64) >> shift) as u64
                }
                0b001011 => {
                    // RORV
                    let shift = (operand2 & 0x3F) as u32;
                    operand1.rotate_right(shift)
                }
                0b010000 => {
                    // CRC32B
                    crc32(operand1, operand2 as u8 as u64, 8)
                }
                0b010001 => {
                    // CRC32H
                    crc32(operand1, operand2 as u16 as u64, 16)
                }
                0b010010 => {
                    // CRC32W
                    crc32(operand1, operand2 as u32 as u64, 32)
                }
                0b010011 => {
                    // CRC32X
                    crc32(operand1, operand2, 64)
                }
                0b010100 => {
                    // CRC32CB
                    crc32c(operand1, operand2 as u8 as u64, 8)
                }
                0b010101 => {
                    // CRC32CH
                    crc32c(operand1, operand2 as u16 as u64, 16)
                }
                0b010110 => {
                    // CRC32CW
                    crc32c(operand1, operand2 as u32 as u64, 32)
                }
                0b010111 => {
                    // CRC32CX
                    crc32c(operand1, operand2, 64)
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };

            self.set_x(rd, result);
        } else {
            let operand1 = self.get_w(rn);
            let operand2 = self.get_w(rm);

            let result = match opcode {
                0b000010 => {
                    // UDIV
                    if operand2 == 0 {
                        0
                    } else {
                        operand1 / operand2
                    }
                }
                0b000011 => {
                    // SDIV
                    if operand2 == 0 {
                        0
                    } else {
                        (operand1 as i32).wrapping_div(operand2 as i32) as u32
                    }
                }
                0b001000 => {
                    // LSLV
                    let shift = (operand2 & 0x1F) as u32;
                    operand1 << shift
                }
                0b001001 => {
                    // LSRV
                    let shift = (operand2 & 0x1F) as u32;
                    operand1 >> shift
                }
                0b001010 => {
                    // ASRV
                    let shift = (operand2 & 0x1F) as u32;
                    ((operand1 as i32) >> shift) as u32
                }
                0b001011 => {
                    // RORV
                    let shift = (operand2 & 0x1F) as u32;
                    operand1.rotate_right(shift)
                }
                0b010000 => {
                    // CRC32B
                    crc32(operand1 as u64, operand2 as u8 as u64, 8) as u32
                }
                0b010001 => {
                    // CRC32H
                    crc32(operand1 as u64, operand2 as u16 as u64, 16) as u32
                }
                0b010010 => {
                    // CRC32W
                    crc32(operand1 as u64, operand2 as u64, 32) as u32
                }
                0b010100 => {
                    // CRC32CB
                    crc32c(operand1 as u64, operand2 as u8 as u64, 8) as u32
                }
                0b010101 => {
                    // CRC32CH
                    crc32c(operand1 as u64, operand2 as u16 as u64, 16) as u32
                }
                0b010110 => {
                    // CRC32CW
                    crc32c(operand1 as u64, operand2 as u64, 32) as u32
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };

            self.set_w(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_dp_3src(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let sf = (insn >> 31) & 1;
        let op54 = (insn >> 29) & 0x3;
        let op31 = (insn >> 21) & 0x7;
        let rm = ((insn >> 16) & 0x1F) as u8;
        let o0 = (insn >> 15) & 1;
        let ra = ((insn >> 10) & 0x1F) as u8;
        let rn = ((insn >> 5) & 0x1F) as u8;
        let rd = (insn & 0x1F) as u8;

        if op54 != 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        if sf != 0 {
            // 64-bit
            let operand1 = self.get_x(rn);
            let operand2 = self.get_x(rm);
            let addend = self.get_x(ra);

            let result = match (op31, o0) {
                (0b000, 0) => {
                    // MADD
                    addend.wrapping_add(operand1.wrapping_mul(operand2))
                }
                (0b000, 1) => {
                    // MSUB
                    addend.wrapping_sub(operand1.wrapping_mul(operand2))
                }
                (0b001, 0) => {
                    // SMADDL
                    let p = (operand1 as i32 as i64).wrapping_mul(operand2 as i32 as i64);
                    (addend as i64).wrapping_add(p) as u64
                }
                (0b001, 1) => {
                    // SMSUBL
                    let p = (operand1 as i32 as i64).wrapping_mul(operand2 as i32 as i64);
                    (addend as i64).wrapping_sub(p) as u64
                }
                (0b010, 0) => {
                    // SMULH
                    let a = operand1 as i64 as i128;
                    let b = operand2 as i64 as i128;
                    ((a * b) >> 64) as u64
                }
                (0b101, 0) => {
                    // UMADDL
                    let p = (operand1 as u32 as u64).wrapping_mul(operand2 as u32 as u64);
                    addend.wrapping_add(p)
                }
                (0b101, 1) => {
                    // UMSUBL
                    let p = (operand1 as u32 as u64).wrapping_mul(operand2 as u32 as u64);
                    addend.wrapping_sub(p)
                }
                (0b110, 0) => {
                    // UMULH
                    let a = operand1 as u128;
                    let b = operand2 as u128;
                    ((a * b) >> 64) as u64
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };

            self.set_x(rd, result);
        } else {
            // 32-bit
            let operand1 = self.get_w(rn);
            let operand2 = self.get_w(rm);
            let addend = self.get_w(ra);

            let result = match (op31, o0) {
                (0b000, 0) => {
                    // MADD
                    addend.wrapping_add(operand1.wrapping_mul(operand2))
                }
                (0b000, 1) => {
                    // MSUB
                    addend.wrapping_sub(operand1.wrapping_mul(operand2))
                }
                _ => return Err(ArmError::UndefinedInstruction(insn)),
            };

            self.set_w(rd, result);
        }

        Ok(CpuExit::Continue)
    }
}

// =============================================================================
// ArmCpu Trait Implementation
// =============================================================================

// =============================================================================
// System-mode execution (full-machine emulation)
// =============================================================================
//
// `ArmCpu::step` surfaces faults and exception-generating instructions as
// errors/exits — the right behavior for the instruction-level oracle tests.
// Booting an OS instead requires architectural delivery: SVC vectors to the
// EL1 handler, page faults become data aborts with a syndrome, and GIC/timer
// interrupts asynchronously enter the vector table. `step_system` wraps the
// same execution core with that delivery layer.
impl AArch64Cpu {
    /// Cycles the counter advances per emulated instruction.
    const TIMER_TICKS_PER_INSN: u64 = 16;

    /// Execute one instruction with full system semantics.
    pub fn step_system(&mut self) -> Result<CpuExit, ArmError> {
        self.tick_system(Self::TIMER_TICKS_PER_INSN);

        if self.halted {
            return Ok(CpuExit::Halt);
        }

        let irq_line = self
            .gic_irq_line
            .as_ref()
            .map(|l| l.load(std::sync::atomic::Ordering::Acquire))
            .unwrap_or(false);

        // WFE may complete spuriously (real hardware wakes it via the timer
        // event stream, which is not modelled): never actually sleep on it,
        // just yield once so spin loops (LDXR; WFE) keep making progress.
        if self.wfe {
            self.wfe = false;
            self.event_register = false;
            return Ok(CpuExit::Wfe);
        }

        if self.wfi {
            if irq_line {
                self.wfi = false;
            } else {
                // Idle: skip the counter ahead to the next timer deadline so
                // a sleeping guest doesn't burn host time waiting for ticks.
                self.fast_forward_timers();
                return Ok(CpuExit::Wfi);
            }
        }

        // Deliver a pending IRQ if PSTATE.I allows.
        if irq_line && (self.daif & 0x2) == 0 {
            self.take_irq()?;
            return Ok(CpuExit::Continue);
        }

        self.pc_ring[self.pc_ring_idx] = self.pc;
        self.pc_ring_idx = (self.pc_ring_idx + 1) % self.pc_ring.len();

        match self.execute_instruction() {
            Ok(CpuExit::Svc(imm)) => {
                // PC already points past the SVC: that is the preferred
                // return address.
                self.enter_sync_exception(SyndromeRegister::svc(imm as u16), None)?;
                Ok(CpuExit::Continue)
            }
            Ok(CpuExit::Breakpoint(imm)) if !self.breakpoints.contains(&self.pc) => {
                // Guest BRK instruction (not a host debugger breakpoint):
                // the preferred return address is the BRK itself.
                self.pc = self.pc.wrapping_sub(4);
                self.enter_sync_exception(SyndromeRegister::brk(imm as u16), None)?;
                Ok(CpuExit::Continue)
            }
            Ok(exit) => Ok(exit),
            Err(err) => self.deliver_fault(err),
        }
    }

    /// Advance the generic timer and mirror its output lines into GIC PPIs
    /// (27 = virtual timer, 30 = non-secure physical timer).
    fn tick_system(&mut self, cycles: u64) {
        self.sysregs.tick_timers(cycles);

        let levels = (
            self.sysregs.cntv_interrupt_pending(),
            self.sysregs.cntp_interrupt_pending(),
        );
        if levels != self.timer_levels {
            if let Some(ref gic) = self.gic {
                if let Ok(mut gic) = gic.lock() {
                    gic.set_ppi_level(0, 27, levels.0);
                    gic.set_ppi_level(0, 30, levels.1);
                }
            }
            self.timer_levels = levels;
        }
    }

    /// During WFI, jump the counter to the nearest armed timer deadline (or
    /// nudge it forward when no timer is armed).
    fn fast_forward_timers(&mut self) {
        let cntpct = self.sysregs.cntpct_el0;
        let cntvoff = self.sysregs.cntvoff_el2;
        let mut target: Option<u64> = None;

        // CNTP deadline in physical-counter terms.
        if self.sysregs.cntp_ctl_el0 & 0x3 == 0x1 && self.sysregs.cntp_cval_el0 > cntpct {
            target = Some(self.sysregs.cntp_cval_el0);
        }
        // CNTV deadline converted to physical-counter terms.
        if self.sysregs.cntv_ctl_el0 & 0x3 == 0x1 {
            let phys = self.sysregs.cntv_cval_el0.wrapping_add(cntvoff);
            if phys > cntpct {
                target = Some(target.map_or(phys, |t| t.min(phys)));
            }
        }

        let jump = match target {
            Some(t) => t.saturating_sub(cntpct),
            // No armed timer: advance ~1ms of counter time per idle pass.
            None => self.sysregs.cntfrq_el0 / 1000,
        };
        self.tick_system(jump);
    }

    /// Take an IRQ exception now.
    fn take_irq(&mut self) -> Result<(), ArmError> {
        let target = exception_target_el(
            ExceptionType::Irq,
            self.current_el,
            self.sysregs.hcr_el2,
            self.sysregs.scr_el3,
        );
        self.take_exception(target, ExceptionType::Irq, SyndromeRegister::new())
    }

    /// Take a synchronous exception with the given syndrome (and FAR, for
    /// aborts).
    fn enter_sync_exception(
        &mut self,
        syndrome: SyndromeRegister,
        far: Option<u64>,
    ) -> Result<(), ArmError> {
        let target = exception_target_el(
            ExceptionType::Synchronous,
            self.current_el,
            self.sysregs.hcr_el2,
            self.sysregs.scr_el3,
        );
        if let Some(addr) = far {
            self.sysregs.bank_mut(target).far = addr;
        }
        self.take_exception(target, ExceptionType::Synchronous, syndrome)
    }

    /// FP/SIMD access trap (CPACR.FPEN): vector to the EL1 handler with
    /// EC=0x07 so the kernel can do its lazy FP context switch. Called from
    /// inside instruction execution, where PC has already been advanced.
    fn take_fp_access_trap(&mut self) -> Result<CpuExit, ArmError> {
        self.pc = self.pc.wrapping_sub(4);
        self.enter_sync_exception(SyndromeRegister::simd_fp_trap(), None)?;
        Ok(CpuExit::Continue)
    }

    /// Convert an execution error into the corresponding guest exception.
    /// PC has been restored to the faulting instruction by
    /// `execute_instruction`.
    fn deliver_fault(&mut self, err: ArmError) -> Result<CpuExit, ArmError> {
        use crate::arm::cpu_trait::AccessType;

        // Boot debugging: surface the first faults (and any fault storm).
        self.fault_log_budget = self.fault_log_budget.saturating_sub(1);
        if self.fault_log_budget > 0 {
            tracing::debug!(
                pc = format!("{:#x}", self.pc),
                el = self.current_el,
                insns = self.insn_count,
                err = ?err,
                level = self.last_fault_level.load(std::sync::atomic::Ordering::Relaxed),
                sctlr = format!("{:#x}", self.sysregs.el1.sctlr),
                tcr = format!("{:#x}", self.sysregs.el1.tcr),
                ttbr0 = format!("{:#x}", self.sysregs.el1.ttbr0),
                ttbr1 = format!("{:#x}", self.sysregs.el1.ttbr1),
                "guest fault"
            );
        }

        match err {
            ArmError::MemoryError(info) => {
                let level = self
                    .last_fault_level
                    .load(std::sync::atomic::Ordering::Relaxed);
                let fsc = fsc_for_fault(info.fault_type, level);
                let from_lower = self.current_el == 0;
                let syndrome = if info.access == AccessType::InstructionFetch {
                    SyndromeRegister::instruction_abort(from_lower, fsc, false)
                } else {
                    SyndromeRegister::data_abort(
                        from_lower,
                        fsc,
                        info.access == AccessType::Write || info.access == AccessType::Atomic,
                        false, // cm
                        false, // s1ptw
                        false, // isv
                        0,     // sas
                        false, // sse
                        0,     // srt
                        false, // sf
                        false, // ar
                        false, // vncr
                        false, // fnv
                        false, // ea
                        0,     // set
                    )
                };
                self.enter_sync_exception(syndrome, Some(info.address))?;
                Ok(CpuExit::Continue)
            }
            ArmError::UndefinedInstruction(_) => {
                self.enter_sync_exception(SyndromeRegister::unknown(), None)?;
                Ok(CpuExit::Continue)
            }
            ArmError::Unimplemented(what) => {
                // Trap-on-unknown: report it once at debug level, then let
                // the guest's undef handler decide.
                tracing::debug!(what, pc = format!("{:#x}", self.pc), "UNDEF injection");
                self.enter_sync_exception(SyndromeRegister::unknown(), None)?;
                Ok(CpuExit::Continue)
            }
            other => Err(other),
        }
    }
}

/// Map an MMU walk fault to the internal fault-type enum (for AT/PAR).
fn translation_fault_type_of(fault: &TranslationFault) -> MemoryFaultType {
    use super::mmu::TranslationFaultType as T;
    match fault.fault_type {
        T::Translation => MemoryFaultType::Translation,
        T::Permission => MemoryFaultType::Permission,
        T::Alignment => MemoryFaultType::Alignment,
        T::AccessFlag => MemoryFaultType::AccessFlag,
        T::AddressSize => MemoryFaultType::AddressSize,
        T::ExternalAbort => MemoryFaultType::External,
    }
}

/// Map an internal fault type + translation level to the architectural fault
/// status code.
fn fsc_for_fault(
    fault_type: MemoryFaultType,
    level: u8,
) -> super::exceptions::FaultStatusCode {
    use super::exceptions::FaultStatusCode as F;
    let level = level.min(3);
    match fault_type {
        MemoryFaultType::Translation => match level {
            0 => F::TranslationL0,
            1 => F::TranslationL1,
            2 => F::TranslationL2,
            _ => F::TranslationL3,
        },
        MemoryFaultType::AccessFlag => match level {
            1 => F::AccessFlagL1,
            2 => F::AccessFlagL2,
            _ => F::AccessFlagL3,
        },
        MemoryFaultType::Permission => match level {
            1 => F::PermissionL1,
            2 => F::PermissionL2,
            _ => F::PermissionL3,
        },
        MemoryFaultType::Alignment => F::Alignment,
        MemoryFaultType::AddressSize => match level {
            0 => F::AddressSizeL0,
            1 => F::AddressSizeL1,
            2 => F::AddressSizeL2,
            _ => F::AddressSizeL3,
        },
        _ => F::SyncExternal,
    }
}

impl ArmCpu for AArch64Cpu {
    fn step(&mut self) -> Result<CpuExit, ArmError> {
        if self.halted {
            return Ok(CpuExit::Halt);
        }

        // Check for WFI/WFE completion
        if self.wfi {
            if let Some(_) = self.check_pending_interrupts()? {
                self.wfi = false;
            } else {
                return Ok(CpuExit::Wfi);
            }
        }

        if self.wfe {
            if self.event_register {
                self.event_register = false;
                self.wfe = false;
            } else {
                return Ok(CpuExit::Wfe);
            }
        }

        // Check for pending interrupts
        if let Some(exit) = self.check_pending_interrupts()? {
            return Ok(exit);
        }

        // Execute one instruction
        self.execute_instruction()
    }

    fn reset(&mut self) {
        // Reset all registers
        self.x = [0; NUM_GPRS];
        self.sp_el = [0; NUM_ELS];
        self.pc = 0;

        self.nzcv = 0;
        self.daif = 0xF; // All exceptions masked
        self.current_el = self.config.initial_el;
        self.sp_sel = true;
        self.pan = false;
        self.uao = false;
        self.dit = false;
        self.ssbs = false;
        self.tco = false;
        self.btype = 0;
        self.il = false;
        self.ss = false;

        self.v = [0; NUM_SIMD_REGS];
        self.fpcr = 0;
        self.fpsr = 0;

        self.sysregs.reset();
        self.mmu = Mmu::new();
        if let Some(ref gic) = self.gic {
            if let Ok(mut gic) = gic.lock() {
                gic.reset();
            }
        }
        self.timer_levels = (false, false);

        self.insn_count = 0;
        self.cycle_count = 0;
        self.halted = false;
        self.wfi = false;
        self.wfe = false;
        self.event_register = false;
        self.pending_exceptions.clear();
        self.breakpoints.clear();
        self.watchpoints.clear();
    }

    fn get_gpr(&self, reg: u8) -> u64 {
        self.get_x(reg)
    }

    fn set_gpr(&mut self, reg: u8, value: u64) {
        self.set_x(reg, value);
    }

    fn get_pc(&self) -> u64 {
        self.pc
    }

    fn set_pc(&mut self, value: u64) {
        self.pc = value;
    }

    fn get_sp(&self) -> u64 {
        self.current_sp()
    }

    fn set_sp(&mut self, value: u64) {
        self.set_current_sp(value);
    }

    fn get_lr(&self) -> u64 {
        self.get_x(30) // X30 is the link register in AArch64
    }

    fn set_lr(&mut self, value: u64) {
        self.set_x(30, value);
    }

    fn get_pstate(&self) -> ProcessorState {
        ProcessorState {
            n: self.get_n(),
            z: self.get_z(),
            c: self.get_c(),
            v: self.get_v(),
            q: false,
            ge: 0,
            el: self.current_el,
            sp_sel: self.sp_sel,
            t: false, // Not applicable to AArch64
            i: (self.daif & 0x2) != 0,
            f: (self.daif & 0x1) != 0,
            a: (self.daif & 0x4) != 0,
            d: (self.daif & 0x8) != 0,
            e: false, // Little endian
            it_state: 0,
            mode: 0,
        }
    }

    fn set_pstate(&mut self, state: ProcessorState) {
        self.set_nzcv(state.n, state.z, state.c, state.v);
        self.current_el = state.el;
        self.sp_sel = state.sp_sel;
        self.daif = ((state.d as u8) << 3)
            | ((state.a as u8) << 2)
            | ((state.i as u8) << 1)
            | (state.f as u8);
    }

    fn is_privileged(&self) -> bool {
        self.current_el > 0
    }

    fn is_secure(&self) -> bool {
        // Check SCR_EL3.NS bit
        (self.sysregs.scr_el3 & 1) == 0
    }

    fn current_el(&self) -> u8 {
        self.current_el
    }

    fn read_memory(&self, addr: u64, size: usize) -> Result<Vec<u8>, ArmError> {
        let mut data = vec![0u8; size];
        for i in 0..size {
            data[i] = self.mem_read_u8(addr + i as u64)?;
        }
        Ok(data)
    }

    fn write_memory(&mut self, addr: u64, data: &[u8]) -> Result<(), ArmError> {
        for (i, &byte) in data.iter().enumerate() {
            self.mem_write_u8(addr + i as u64, byte)?;
        }
        Ok(())
    }

    fn arch_version(&self) -> ArmVersion {
        self.config.version
    }

    fn profile(&self) -> ArmProfile {
        ArmProfile::A
    }

    fn features(&self) -> ArmFeatures {
        self.config.features
    }

    fn pending_exceptions(&self) -> Vec<ArmException> {
        self.pending_exceptions.clone()
    }

    fn inject_exception(&mut self, exception: ArmException) -> Result<(), ArmError> {
        self.pending_exceptions.push(exception);
        Ok(())
    }

    fn set_breakpoint(&mut self, addr: u64) -> Result<(), ArmError> {
        self.breakpoints.insert(addr);
        Ok(())
    }

    fn clear_breakpoint(&mut self, addr: u64) -> Result<(), ArmError> {
        self.breakpoints.remove(&addr);
        Ok(())
    }

    fn set_watchpoint(
        &mut self,
        addr: u64,
        size: usize,
        kind: WatchpointKind,
    ) -> Result<(), ArmError> {
        // Check if watchpoint already exists
        if !self
            .watchpoints
            .iter()
            .any(|(a, s, k)| *a == addr && *s == size && *k == kind)
        {
            self.watchpoints.push((addr, size, kind));
        }
        Ok(())
    }

    fn clear_watchpoint(&mut self, addr: u64) -> Result<(), ArmError> {
        self.watchpoints.retain(|(a, _, _)| *a != addr);
        Ok(())
    }

    fn instruction_count(&self) -> u64 {
        self.insn_count
    }

    fn cycle_count(&self) -> Option<u64> {
        Some(self.cycle_count)
    }

    fn has_fpu(&self) -> bool {
        true // AArch64 always has FP
    }

    fn get_simd_reg(&self, reg: u8) -> Option<(u64, u64)> {
        if reg < 32 {
            let val = self.v[reg as usize];
            Some((val as u64, (val >> 64) as u64))
        } else {
            None
        }
    }

    fn set_simd_reg(&mut self, reg: u8, low: u64, high: u64) -> Result<(), ArmError> {
        if reg < 32 {
            self.v[reg as usize] = (high as u128) << 64 | (low as u128);
            Ok(())
        } else {
            Err(ArmError::InvalidRegister(reg))
        }
    }

    fn get_fpcr(&self) -> Option<u32> {
        Some(self.fpcr)
    }

    fn set_fpcr(&mut self, value: u32) -> Result<(), ArmError> {
        self.fpcr = value;
        Ok(())
    }

    fn get_fpsr(&self) -> Option<u32> {
        Some(self.fpsr)
    }

    fn set_fpsr(&mut self, value: u32) -> Result<(), ArmError> {
        self.fpsr = value;
        Ok(())
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Decode bitmask immediate for logical instructions.
fn decode_bitmask(n: bool, imms: u32, immr: u32, is_64bit: bool) -> Result<u64, ArmError> {
    // For 64-bit (sf=1): N must be 1
    // For 32-bit (sf=0): N must be 0, and highest set bit in ~imms[5:0] determines element size
    let len = if n {
        6 // 64-bit elements
    } else {
        // Find highest set bit position in ~imms[5:0] (6-bit value)
        let not_imms = !imms & 0x3F;
        if not_imms == 0 {
            return Err(ArmError::UndefinedInstruction(0));
        }
        // len = HighestSetBit(immN:NOT(imms)) per the A64 DecodeBitMasks
        // pseudocode. For N=0 this is the highest set bit position of
        // ~imms[5:0] (0-5); the element size is 1<<len.
        let pos = 31 - not_imms.leading_zeros();
        if pos > 5 {
            return Err(ArmError::UndefinedInstruction(0));
        }
        pos
    };

    if len < 1 || len > 6 {
        return Err(ArmError::UndefinedInstruction(0));
    }

    let levels = (1u32 << len) - 1;
    let s = imms & levels;
    let r = immr & levels;
    let esize = 1u64 << len;

    if s == levels {
        return Err(ArmError::UndefinedInstruction(0));
    }

    // Create the pattern - a run of (s+1) ones
    let welem = if s + 1 >= 64 {
        u64::MAX
    } else {
        (1u64 << (s + 1)) - 1
    };

    // Create mask for element size
    let esize_mask = if esize >= 64 {
        u64::MAX
    } else {
        (1u64 << esize) - 1
    };

    // Rotate right by r
    let rotated = if r == 0 {
        welem
    } else {
        ((welem >> r) | (welem << (esize as u32 - r))) & esize_mask
    };

    // Replicate to fill the register
    let mut result = 0u64;
    let replications = 64 / esize;
    for i in 0..replications {
        result |= rotated << (i * esize);
    }

    if !is_64bit {
        result &= 0xFFFF_FFFF;
    }

    Ok(result)
}

/// Decode bitmasks for bitfield instructions.
fn decode_bitmasks(
    n: bool,
    imms: u32,
    immr: u32,
    _immediate: bool,
    datasize: u32,
) -> Result<(u64, u64), ArmError> {
    // len = HighestSetBit(immN:NOT(imms<5:0>))
    // For N=1: the 7-bit value is 1:xxxxxx, so highest bit is at position 6 -> len=6
    // For N=0: the 7-bit value is 0:NOT(imms), we find highest bit of NOT(imms)
    let len = if n {
        6
    } else {
        let not_imms = !imms & 0x3F;
        if not_imms == 0 {
            // All bits of imms are 1, which is reserved
            return Err(ArmError::UndefinedInstruction(0));
        }
        // Find position of highest set bit in not_imms (0-5)
        // leading_zeros for u32 counts from bit 31, so position = 31 - leading_zeros
        // But not_imms is only 6 bits, so we need: 5 - (not_imms as u8).leading_zeros() after masking
        // Actually simpler: 31 - not_imms.leading_zeros() gives us the position in the u32
        let pos = 31 - not_imms.leading_zeros();
        if pos > 5 {
            return Err(ArmError::UndefinedInstruction(0));
        }
        pos // len = position of highest set bit (not pos + 1!)
    };

    if len < 1 || len > 6 || (1 << len) > datasize {
        return Err(ArmError::UndefinedInstruction(0));
    }

    let levels = (1u32 << len) - 1;
    let s = imms & levels;
    let r = immr & levels;
    let diff = ((s as i32).wrapping_sub(r as i32)) as u32;
    let esize = 1u64 << len;

    // Create element masks, handling potential overflow
    let welem = if s + 1 >= 64 {
        u64::MAX
    } else {
        (1u64 << (s + 1)) - 1
    };

    let telem_bits = (diff & levels) + 1;
    let telem = if telem_bits >= 64 {
        u64::MAX
    } else {
        (1u64 << telem_bits) - 1
    };

    let esize_mask = if esize >= 64 {
        u64::MAX
    } else {
        (1u64 << esize) - 1
    };

    // Rotate welem right by R within element size
    let wmask_elem = if r == 0 {
        welem
    } else {
        ((welem >> r) | (welem << (esize as u32 - r))) & esize_mask
    };

    // Replicate
    let mut wmask = 0u64;
    let mut tmask = 0u64;
    let replications = 64 / esize;
    for i in 0..replications {
        wmask |= wmask_elem << (i * esize);
        tmask |= (telem & esize_mask) << (i * esize);
    }

    if datasize == 32 {
        wmask &= 0xFFFF_FFFF;
        tmask &= 0xFFFF_FFFF;
    }

    Ok((wmask, tmask))
}

/// CRC32 calculation (ISO 3309 polynomial).
fn crc32(crc: u64, data: u64, size: u32) -> u64 {
    const POLY: u32 = 0xEDB8_8320;
    let mut crc = crc as u32;
    let bytes = size / 8;

    for i in 0..bytes {
        let byte = ((data >> (i * 8)) & 0xFF) as u8;
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ POLY
            } else {
                crc >> 1
            };
        }
    }

    crc as u64
}

/// CRC32C calculation (Castagnoli polynomial).
fn crc32c(crc: u64, data: u64, size: u32) -> u64 {
    const POLY: u32 = 0x82F6_3B78;
    let mut crc = crc as u32;
    let bytes = size / 8;

    for i in 0..bytes {
        let byte = ((data >> (i * 8)) & 0xFF) as u8;
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ POLY
            } else {
                crc >> 1
            };
        }
    }

    crc as u64
}

// =============================================================================
// Advanced SIMD (NEON) element helpers
//
// These operate on a single vector element whose value occupies the low
// `bits` bits of a u64 (`bits` in {8, 16, 32, 64}). They implement the exact
// per-element semantics from the ARM Architecture Reference Manual and are
// verified differentially against qemu-user (tests/arm_diff.rs).
// =============================================================================

/// Mask covering the low `bits` bits.
#[inline]
fn elem_mask(bits: u32) -> u64 {
    if bits >= 64 {
        u64::MAX
    } else {
        (1u64 << bits) - 1
    }
}

/// Saturating signed/unsigned add/subtract of a small non-negative `val` into
/// the low 32 bits of a 64-bit GPR (SQINCP/UQINCP/SQDECP/UQDECP, 32-bit form).
/// Matches qemu do_sat_addsub_32: the signed result is sign-extended to 64
/// bits, the unsigned result zero-extended.
fn sat_addsub_32(reg: u64, val: u64, u: bool, d: bool) -> u64 {
    if u {
        let r = (reg as u32) as i64; // zero-extend
        let res = if d {
            (r - val as i64).max(0)
        } else {
            (r + val as i64).min(u32::MAX as i64)
        };
        res as u64 // in [0, UINT32_MAX] -> zero-extended
    } else {
        let r = (reg as u32) as i32 as i64; // sign-extend
        let res = if d {
            (r - val as i64).max(i32::MIN as i64)
        } else {
            (r + val as i64).min(i32::MAX as i64)
        };
        res as u64 // sign-extended 64-bit pattern
    }
}

/// Saturating signed/unsigned add/subtract of a small non-negative `val` into a
/// full 64-bit GPR (SQINCP/UQINCP/SQDECP/UQDECP, 64-bit form). Matches qemu
/// do_sat_addsub_64.
fn sat_addsub_64(reg: u64, val: u64, u: bool, d: bool) -> u64 {
    if u {
        if d {
            reg.saturating_sub(val)
        } else {
            reg.saturating_add(val)
        }
    } else {
        let r = reg as i64;
        let v = val as i64;
        (if d {
            r.saturating_sub(v)
        } else {
            r.saturating_add(v)
        }) as u64
    }
}

/// Per-element saturating add/sub of `count` for the SINCDECP vector form.
/// `bits` ∈ {16,32,64}; increments add `count`, decrements subtract it,
/// saturating into the element's signed (`u`=false) or unsigned (`u`=true)
/// range. Matches qemu sve_{s,u}q{add,sub}i_{h,s,d}.
fn sat_addsub_elem(elem: u64, count: u64, bits: u32, u: bool, dec: bool) -> u64 {
    let mask = elem_mask(bits);
    if u {
        if bits >= 64 {
            return if dec {
                elem.saturating_sub(count)
            } else {
                elem.saturating_add(count)
            };
        }
        let n = (elem & mask) as i128;
        let r = if dec {
            n - count as i128
        } else {
            n + count as i128
        };
        (r.clamp(0, mask as i128) as u64) & mask
    } else {
        let n = sext_elem(elem, bits);
        if bits >= 64 {
            let r = if dec {
                (n as i64).saturating_sub(count as i64)
            } else {
                (n as i64).saturating_add(count as i64)
            };
            return r as u64;
        }
        let r = if dec {
            n - count as i128
        } else {
            n + count as i128
        };
        let smax = (1i128 << (bits - 1)) - 1;
        let smin = -(1i128 << (bits - 1));
        (r.clamp(smin, smax) as u64) & mask
    }
}

/// Sign-extend the low `bits` bits of `v` to i128.
#[inline]
fn sext_elem(v: u64, bits: u32) -> i128 {
    let v = v & elem_mask(bits);
    let shift = 64 - bits;
    (((v << shift) as i64) >> shift) as i128
}

/// Zero-extend the low `bits` bits of `v` to u128.
#[inline]
fn uext_elem(v: u64, bits: u32) -> u128 {
    (v & elem_mask(bits)) as u128
}

/// Signed rounding shift right (SRSHR), 64-bit. Port of qemu do_srshr; the
/// element is sign-extended to 64 bits before the rounding add.
#[inline]
fn sve_srshr(x: i64, sh: u32) -> i64 {
    if sh < 64 {
        (x >> sh) + ((x >> (sh - 1)) & 1)
    } else {
        // Rounding the sign bit always produces 0.
        0
    }
}

/// Unsigned rounding shift right (URSHR), 64-bit. Port of qemu do_urshr.
#[inline]
fn sve_urshr(x: u64, sh: u32) -> u64 {
    if sh < 64 {
        (x >> sh) + ((x >> (sh - 1)) & 1)
    } else if sh == 64 {
        x >> 63
    } else {
        0
    }
}

/// Saturate a signed value to the `bits`-bit signed range, returned as raw bits.
#[inline]
/// Signed saturating/rounding shift-left by a signed amount, for bits in
/// {8,16,32}. Faithful port of qemu do_sqrshl_bhs (vec_internal.h).
fn sqrshl_bhs(src: i32, shift: i32, bits: u32, round: bool, sat: bool) -> i32 {
    if shift <= -(bits as i32) {
        return if round { 0 } else { src >> 31 };
    } else if shift < 0 {
        if round {
            let s = src >> (-shift - 1);
            return (s >> 1) + (s & 1);
        }
        return src >> (-shift);
    } else if shift < bits as i32 {
        let val = src.wrapping_shl(shift as u32);
        if bits == 32 {
            if !sat || (val >> shift) == src {
                return val;
            }
        } else {
            let extval = (val << (32 - bits)) >> (32 - bits); // sextract32(val,0,bits)
            if !sat || val == extval {
                return extval;
            }
        }
    } else if !sat || src == 0 {
        return 0;
    }
    (1i32 << (bits - 1)) - i32::from(src >= 0)
}

/// Unsigned saturating/rounding shift-left, bits in {8,16,32}. Port of qemu
/// do_uqrshl_bhs.
fn uqrshl_bhs(src: u32, shift: i32, bits: u32, round: bool, sat: bool) -> u32 {
    if shift <= -(bits as i32 + round as i32) {
        return 0;
    } else if shift < 0 {
        if round {
            let s = src >> (-shift - 1);
            return (s >> 1) + (s & 1);
        }
        return src >> (-shift);
    } else if shift < bits as i32 {
        let val = src.wrapping_shl(shift as u32);
        if bits == 32 {
            if !sat || (val >> shift) == src {
                return val;
            }
        } else {
            let extval = val & ((1u32 << bits) - 1);
            if !sat || val == extval {
                return extval;
            }
        }
    } else if !sat || src == 0 {
        return 0;
    }
    if bits == 32 {
        u32::MAX
    } else {
        (1u32 << bits) - 1
    }
}

/// Signed saturating/rounding shift-left for 64-bit elements. Port of qemu
/// do_sqrshl_d.
fn sqrshl_d(src: i64, shift: i64, round: bool, sat: bool) -> i64 {
    if shift <= -64 {
        return if round { 0 } else { src >> 63 };
    } else if shift < 0 {
        if round {
            let s = src >> (-shift - 1);
            return (s >> 1) + (s & 1);
        }
        return src >> (-shift);
    } else if shift < 64 {
        let val = src.wrapping_shl(shift as u32);
        if !sat || (val >> shift) == src {
            return val;
        }
    } else if !sat || src == 0 {
        return 0;
    }
    if src < 0 { i64::MIN } else { i64::MAX }
}

/// Unsigned saturating/rounding shift-left for 64-bit elements. Port of qemu
/// do_uqrshl_d.
fn uqrshl_d(src: u64, shift: i64, round: bool, sat: bool) -> u64 {
    if shift <= -(64 + round as i64) {
        return 0;
    } else if shift < 0 {
        if round {
            let s = src >> (-shift - 1);
            return (s >> 1) + (s & 1);
        }
        return src >> (-shift);
    } else if shift < 64 {
        let val = src.wrapping_shl(shift as u32);
        if !sat || (val >> shift) == src {
            return val;
        }
    } else if !sat || src == 0 {
        return 0;
    }
    u64::MAX
}

fn sat_signed(v: i128, bits: u32) -> u64 {
    let max = (1i128 << (bits - 1)) - 1;
    let min = -(1i128 << (bits - 1));
    (v.clamp(min, max) as u64) & elem_mask(bits)
}

/// Saturate a value to the `bits`-bit unsigned range, returned as raw bits.
#[inline]
fn sat_unsigned(v: i128, bits: u32) -> u64 {
    let max = (1i128 << bits) - 1;
    (v.clamp(0, max) as u64) & elem_mask(bits)
}

/// All-ones if `cond`, else 0, in the low `bits` bits (comparison result).
#[inline]
fn bool_mask(cond: bool, bits: u32) -> u64 {
    if cond { elem_mask(bits) } else { 0 }
}

/// Shift `a` (the low `bits` bits) by the signed amount `sh` per the ARM
/// register-shift family. `signed` selects SSHL vs USHL; `rounding` adds the
/// round constant on right shifts (SRSHL/URSHL); `saturating` clamps left-shift
/// overflow to the element range (SQSHL/UQSHL etc.). Returns the raw result.
fn adv_simd_shift_reg(
    a: u64,
    sh: i32,
    bits: u32,
    signed: bool,
    rounding: bool,
    saturating: bool,
) -> u64 {
    let m = elem_mask(bits);
    if signed {
        let sval = sext_elem(a, bits);
        if sh >= 0 {
            // Left shift.
            let s = sh as u32;
            if s >= bits || s >= 64 {
                if saturating {
                    if sval == 0 {
                        0
                    } else {
                        sat_signed(if sval > 0 { i128::MAX } else { i128::MIN }, bits)
                    }
                } else {
                    0
                }
            } else {
                let res = sval << s;
                if saturating {
                    sat_signed(res, bits)
                } else {
                    (res as u64) & m
                }
            }
        } else {
            // Right shift (arithmetic), optionally rounded.
            let rsh = (-sh) as u32;
            if rsh > bits {
                // Round constant dominates: rounded -> 0, unrounded -> sign.
                if rounding {
                    0
                } else if sval < 0 {
                    m
                } else {
                    0
                }
            } else {
                let round = if rounding { 1i128 << (rsh - 1) } else { 0 };
                let res = (sval + round) >> rsh;
                (res as u64) & m
            }
        }
    } else {
        let uval = uext_elem(a, bits) as i128;
        if sh >= 0 {
            let s = sh as u32;
            if s >= bits || s >= 64 {
                if saturating {
                    if uval == 0 { 0 } else { m }
                } else {
                    0
                }
            } else {
                let res = uval << s;
                if saturating {
                    sat_unsigned(res, bits)
                } else {
                    (res as u64) & m
                }
            }
        } else {
            let rsh = (-sh) as u32;
            if rsh > bits {
                0
            } else {
                let round = if rounding { 1i128 << (rsh - 1) } else { 0 };
                let res = (uval + round) >> rsh;
                (res as u64) & m
            }
        }
    }
}

/// Polynomial (carry-less) multiply of two 8-bit values, low 8 bits of result.
#[inline]
fn poly_mul_8(a: u64, b: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 0..8 {
        if (a >> i) & 1 != 0 {
            result ^= b << i;
        }
    }
    result & 0xFF
}

/// Widening polynomial multiply: `bits`-bit operands -> full `2*bits` product.
#[inline]
fn poly_mul_wide(a: u64, b: u64, bits: u32) -> u64 {
    let mut result: u64 = 0;
    for i in 0..bits {
        if (a >> i) & 1 != 0 {
            result ^= b << i;
        }
    }
    result
}

/// 64x64 -> 128-bit polynomial (carry-less) multiply (PMULL.1Q).
#[inline]
fn poly_mul_64(a: u64, b: u64) -> u128 {
    let mut result: u128 = 0;
    for i in 0..64 {
        if (a >> i) & 1 != 0 {
            result ^= (b as u128) << i;
        }
    }
    result
}

/// Sign-extend the low `bits` bits of a u128 (`bits` up to 64) to i128.
#[inline]
fn sext_elem_wide(v: u128, bits: u32) -> i128 {
    let v = v & elem_mask_u128(bits);
    let shift = 128 - bits;
    ((v << shift) as i128) >> shift
}

/// Saturate a signed value to the `bits`-bit signed range (`bits` up to 64),
/// returned as raw bits in a u128.
#[inline]
fn sat_signed_wide(v: i128, bits: u32) -> u128 {
    let max = (1i128 << (bits - 1)) - 1;
    let min = -(1i128 << (bits - 1));
    (v.clamp(min, max) as u128) & elem_mask_u128(bits)
}

/// Compute one element of an Advanced SIMD three-same *integer* operation.
///
/// `a`, `b` are the source elements (low `bits` bits); `d` is the current
/// destination element (used by accumulating ops MLA/MLS/SABA/UABA). `u` is the
/// U bit and `opcode` the 5-bit opcode. For pairwise opcodes (SMAXP/SMINP/ADDP)
/// the caller supplies the adjacent pair as `(a, b)`.
fn adv_simd_three_same_int(u: u32, opcode: u32, bits: u32, a: u64, b: u64, d: u64) -> u64 {
    let m = elem_mask(bits);
    let sa = sext_elem(a, bits);
    let sb = sext_elem(b, bits);
    let ua = uext_elem(a, bits) as i128;
    let ub = uext_elem(b, bits) as i128;
    let ud = uext_elem(d, bits);

    match opcode {
        0b00000 => {
            // SHADD / UHADD
            if u == 0 {
                ((sa + sb) >> 1) as u64 & m
            } else {
                ((ua + ub) >> 1) as u64 & m
            }
        }
        0b00010 => {
            // SRHADD / URHADD
            if u == 0 {
                ((sa + sb + 1) >> 1) as u64 & m
            } else {
                ((ua + ub + 1) >> 1) as u64 & m
            }
        }
        0b00100 => {
            // SHSUB / UHSUB
            if u == 0 {
                ((sa - sb) >> 1) as u64 & m
            } else {
                ((ua - ub) >> 1) as u64 & m
            }
        }
        0b00001 => {
            // SQADD / UQADD
            if u == 0 {
                sat_signed(sa + sb, bits)
            } else {
                sat_unsigned(ua + ub, bits)
            }
        }
        0b00101 => {
            // SQSUB / UQSUB
            if u == 0 {
                sat_signed(sa - sb, bits)
            } else {
                sat_unsigned(ua - ub, bits)
            }
        }
        0b00110 => {
            // CMGT / CMHI
            let c = if u == 0 { sa > sb } else { ua > ub };
            bool_mask(c, bits)
        }
        0b00111 => {
            // CMGE / CMHS
            let c = if u == 0 { sa >= sb } else { ua >= ub };
            bool_mask(c, bits)
        }
        0b01000 | 0b01001 | 0b01010 | 0b01011 => {
            // SSHL/USHL (1000), SQSHL/UQSHL (1001), SRSHL/URSHL (1010),
            // SQRSHL/UQRSHL (1011). Shift amount is the low byte of b, signed.
            let sh = (b as u8 as i8) as i32;
            let rounding = opcode == 0b01010 || opcode == 0b01011;
            let saturating = opcode == 0b01001 || opcode == 0b01011;
            adv_simd_shift_reg(a, sh, bits, u == 0, rounding, saturating)
        }
        0b01100 => {
            // SMAX / UMAX  (also SMAXP/UMAXP share this op via pairwise sourcing)
            if u == 0 {
                (sa.max(sb) as u64) & m
            } else {
                (ua.max(ub) as u64) & m
            }
        }
        0b01101 => {
            // SMIN / UMIN
            if u == 0 {
                (sa.min(sb) as u64) & m
            } else {
                (ua.min(ub) as u64) & m
            }
        }
        0b01110 => {
            // SABD / UABD
            if u == 0 {
                ((sa - sb).abs() as u64) & m
            } else {
                ((ua - ub).abs() as u64) & m
            }
        }
        0b01111 => {
            // SABA / UABA  (accumulate absolute difference)
            let abd = if u == 0 {
                (sa - sb).abs()
            } else {
                (ua - ub).abs()
            };
            ((ud as i128 + abd) as u64) & m
        }
        0b10000 => {
            // ADD / SUB
            if u == 0 {
                ((ua + ub) as u64) & m
            } else {
                ((ua - ub) as u64) & m
            }
        }
        0b10001 => {
            // CMTST / CMEQ
            let c = if u == 0 { (ua & ub) != 0 } else { ua == ub };
            bool_mask(c, bits)
        }
        0b10010 => {
            // MLA / MLS
            let prod = (ua * ub) as u64;
            if u == 0 {
                (ud as u64).wrapping_add(prod) & m
            } else {
                (ud as u64).wrapping_sub(prod) & m
            }
        }
        0b10011 => {
            // MUL / PMUL
            if u == 0 {
                ((ua * ub) as u64) & m
            } else {
                poly_mul_8(a, b)
            }
        }
        0b10100 => {
            // SMAXP / UMAXP (pairwise max -- same kernel as SMAX/UMAX)
            if u == 0 {
                (sa.max(sb) as u64) & m
            } else {
                (ua.max(ub) as u64) & m
            }
        }
        0b10101 => {
            // SMINP / UMINP
            if u == 0 {
                (sa.min(sb) as u64) & m
            } else {
                (ua.min(ub) as u64) & m
            }
        }
        0b10110 => {
            // SQDMULH / SQRDMULH (signed saturating doubling multiply high)
            let prod = sa * sb;
            let rounded = if u == 1 {
                prod * 2 + (1i128 << (bits - 1))
            } else {
                prod * 2
            };
            sat_signed(rounded >> bits, bits)
        }
        0b10111 => {
            // ADDP (pairwise add)
            ((ua + ub) as u64) & m
        }
        _ => a & m,
    }
}

/// VFPExpandImm for single precision: 8-bit immediate -> f32 bit pattern.
/// VFP modified FP immediate expansion for half precision.
fn vfp_expand_imm_f16(imm8: u8) -> u16 {
    let sign = ((imm8 >> 7) & 1) as u16;
    let frac = (imm8 & 0x3F) as u16;
    (sign << 15) | (if (imm8 >> 6) & 1 == 1 { 0x3000 } else { 0x4000 }) | (frac << 6)
}

/// VFP modified FP immediate expanded for an `esize`-byte element.
fn vfp_expand_imm(imm8: u8, esize: usize) -> u64 {
    match esize {
        2 => vfp_expand_imm_f16(imm8) as u64,
        4 => vfp_expand_imm_f32(imm8) as u64,
        _ => vfp_expand_imm_f64(imm8),
    }
}

/// Reverse the `unit`-byte chunks within an `esize`-byte little-endian value
/// (REVB unit=1, REVH unit=2, REVW unit=4).
fn reverse_chunks(val: u64, esize: usize, unit: usize) -> u64 {
    let bytes = val.to_le_bytes();
    let mut out = [0u8; 8];
    let n = esize / unit;
    for c in 0..n {
        let dst = (n - 1 - c) * unit;
        out[dst..dst + unit].copy_from_slice(&bytes[c * unit..c * unit + unit]);
    }
    u64::from_le_bytes(out)
}

fn vfp_expand_imm_f32(imm8: u8) -> u32 {
    let imm8 = imm8 as u32;
    let sign = (imm8 >> 7) & 1;
    let b6 = (imm8 >> 6) & 1;
    // exp(8) = NOT(b6) : b6*5 : imm8<5:4>
    let exp = ((!b6 & 1) << 7) | (if b6 != 0 { 0b11111 } else { 0 } << 2) | ((imm8 >> 4) & 0x3);
    let mant = (imm8 & 0xF) << 19;
    (sign << 31) | (exp << 23) | mant
}

/// VFPExpandImm for double precision: 8-bit immediate -> f64 bit pattern.
fn vfp_expand_imm_f64(imm8: u8) -> u64 {
    let imm8 = imm8 as u64;
    let sign = (imm8 >> 7) & 1;
    let b6 = (imm8 >> 6) & 1;
    // exp(11) = NOT(b6) : b6*8 : imm8<5:4>
    let exp = ((!b6 & 1) << 10) | (if b6 != 0 { 0xFF } else { 0 } << 2) | ((imm8 >> 4) & 0x3);
    let mant = (imm8 & 0xF) << 48;
    (sign << 63) | (exp << 52) | mant
}

/// AdvSIMDExpandImm: expand an 8-bit immediate to a 64-bit value per `cmode`/`op`
/// (ARM Architecture Reference Manual). Used by the SIMD modified-immediate group.
fn adv_simd_expand_imm(op: u32, cmode: u32, imm8: u8) -> u64 {
    let imm8 = imm8 as u64;
    let rep32 = |x: u64| (x & 0xFFFF_FFFF) | ((x & 0xFFFF_FFFF) << 32);
    let rep16 = |x: u64| {
        let x = x & 0xFFFF;
        x | (x << 16) | (x << 32) | (x << 48)
    };
    let rep8 = |x: u64| (x & 0xFF).wrapping_mul(0x0101_0101_0101_0101);
    match cmode {
        0b0000 | 0b0001 => rep32(imm8),
        0b0010 | 0b0011 => rep32(imm8 << 8),
        0b0100 | 0b0101 => rep32(imm8 << 16),
        0b0110 | 0b0111 => rep32(imm8 << 24),
        0b1000 | 0b1001 => rep16(imm8),
        0b1010 | 0b1011 => rep16(imm8 << 8),
        0b1100 => rep32((imm8 << 8) | 0xFF),
        0b1101 => rep32((imm8 << 16) | 0xFFFF),
        0b1110 => {
            if op == 0 {
                rep8(imm8)
            } else {
                // MOVI 64-bit: each bit of imm8 expands to a 0x00/0xFF byte.
                let mut r = 0u64;
                for i in 0..8 {
                    if (imm8 >> i) & 1 != 0 {
                        r |= 0xFFu64 << (i * 8);
                    }
                }
                r
            }
        }
        0b1111 => {
            if op == 0 {
                rep32(vfp_expand_imm_f32(imm8 as u8) as u64)
            } else {
                vfp_expand_imm_f64(imm8 as u8)
            }
        }
        _ => 0,
    }
}

/// Mask covering the low `bits` bits, as u128 (`bits` up to 128).
#[inline]
fn elem_mask_u128(bits: u32) -> u128 {
    if bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bits) - 1
    }
}

/// Like `simd_rshift` but returns the full (untruncated, signed) shifted value
/// so a narrowing op can saturate it to a smaller destination element.
fn simd_rshift_full(a: u64, shift: u32, bits: u32, signed: bool, rounding: bool) -> i128 {
    let round: i128 = if rounding { 1i128 << (shift - 1) } else { 0 };
    if signed {
        (sext_elem(a, bits) + round) >> shift
    } else {
        ((uext_elem(a, bits) as i128) + round) >> shift
    }
}

/// One element of a NEON fixed-point <-> floating-point conversion (`bits` is
/// 16, 32 or 64, `scale` is 2^fbits). Returns the raw result element.
fn fixed_point_convert(opcode: u32, u: u32, bits: u32, a: u64, scale: f64) -> u64 {
    if bits == 16 {
        // FP16 variants (FEAT_FP16).
        if opcode == 0b11100 {
            let f = if u == 0 {
                (a as u16 as i16 as f64) / scale
            } else {
                (a as u16 as f64) / scale
            };
            AArch64Cpu::f32_to_fp16(f as f32) as u64
        } else {
            let f = (AArch64Cpu::fp16_to_f32(a as u16) as f64) * scale;
            let t = f.trunc();
            if u == 0 {
                (t.clamp(i16::MIN as f64, i16::MAX as f64) as i16 as u16) as u64
            } else {
                t.clamp(0.0, u16::MAX as f64) as u16 as u64
            }
        }
    } else if opcode == 0b11100 {
        // SCVTF / UCVTF: integer * 2^-fbits -> float
        if bits == 32 {
            let f = if u == 0 {
                (a as u32 as i32 as f64) / scale
            } else {
                (a as u32 as f64) / scale
            };
            (f as f32).to_bits() as u64
        } else {
            let f = if u == 0 {
                (a as i64 as f64) / scale
            } else {
                (a as f64) / scale
            };
            f.to_bits()
        }
    } else {
        // FCVTZS / FCVTZU: float * 2^fbits -> integer (round toward zero)
        if bits == 32 {
            let f = (f32::from_bits(a as u32) as f64) * scale;
            let t = f.trunc();
            if u == 0 {
                (t.clamp(i32::MIN as f64, i32::MAX as f64) as i32 as u32) as u64
            } else {
                t.clamp(0.0, u32::MAX as f64) as u32 as u64
            }
        } else {
            let f = f64::from_bits(a) * scale;
            let t = f.trunc();
            if u == 0 {
                (t.clamp(i64::MIN as f64, i64::MAX as f64) as i64) as u64
            } else {
                t.clamp(0.0, u64::MAX as f64) as u64
            }
        }
    }
}

/// Right-shift the low `bits` bits of `a` by `shift` (1..=bits), arithmetic if
/// `signed`, with optional rounding (SRSHR/URSHR). Result in the low `bits` bits.
fn simd_rshift(a: u64, shift: u32, bits: u32, signed: bool, rounding: bool) -> u64 {
    let m = elem_mask(bits);
    let round: i128 = if rounding { 1i128 << (shift - 1) } else { 0 };
    if signed {
        let v = sext_elem(a, bits);
        (((v + round) >> shift) as u64) & m
    } else {
        let v = uext_elem(a, bits) as i128;
        (((v + round) >> shift) as u64) & m
    }
}

/// One element of a same-size Advanced SIMD shift-by-immediate. `a` is the
/// source element, `d` the current destination element (for the accumulating
/// and insert forms). Returns the raw result element.
fn adv_simd_shift_imm_elem(u: u32, opcode: u32, bits: u32, shift: u32, a: u64, d: u64) -> u64 {
    let m = elem_mask(bits);
    let signed = u == 0;
    match opcode {
        0b00000 => simd_rshift(a, shift, bits, signed, false), // SSHR / USHR
        0b00010 => {
            // SSRA / USRA: accumulate shifted value into destination.
            (d.wrapping_add(simd_rshift(a, shift, bits, signed, false))) & m
        }
        0b00100 => simd_rshift(a, shift, bits, signed, true), // SRSHR / URSHR
        0b00110 => {
            // SRSRA / URSRA
            (d.wrapping_add(simd_rshift(a, shift, bits, signed, true))) & m
        }
        0b01000 => {
            // SRI (u==1): shift right and insert.
            let low_mask = if shift >= bits {
                0
            } else {
                (1u64 << (bits - shift)) - 1
            };
            let shifted = (uext_elem(a, bits) >> shift) as u64 & low_mask;
            shifted | (d & !low_mask & m)
        }
        0b01010 => {
            if u == 0 {
                // SHL
                ((uext_elem(a, bits) << shift) as u64) & m
            } else {
                // SLI: shift left and insert.
                let low_mask = (1u64 << shift) - 1;
                let shifted = ((uext_elem(a, bits) << shift) as u64) & m & !low_mask;
                shifted | (d & low_mask)
            }
        }
        0b01100 => {
            // SQSHLU: signed value, saturating left shift to unsigned range.
            sat_unsigned(sext_elem(a, bits) << shift, bits)
        }
        0b01110 => {
            // SQSHL / UQSHL: saturating left shift.
            if signed {
                sat_signed(sext_elem(a, bits) << shift, bits)
            } else {
                sat_unsigned((uext_elem(a, bits) as i128) << shift, bits)
            }
        }
        _ => a & m,
    }
}

/// Reverse the low `bits` bits of each byte, returning a value with `bits/8`
/// bit-reversed bytes (RBIT operates per byte).
#[inline]
fn rbit_bytes(a: u64, bits: u32) -> u64 {
    let mut out = 0u64;
    for byte in 0..(bits / 8) {
        let b = ((a >> (byte * 8)) & 0xFF) as u8;
        out |= (b.reverse_bits() as u64) << (byte * 8);
    }
    out
}

/// Count leading sign bits (CLS): number of consecutive bits after the sign bit
/// that equal the sign bit, within an element of `bits`.
#[inline]
fn count_leading_sign(a: u64, bits: u32) -> u64 {
    let v = a & elem_mask(bits);
    let sign = (v >> (bits - 1)) & 1;
    let mut count = 0u64;
    let mut i = bits as i32 - 2;
    while i >= 0 {
        if (v >> i) & 1 == sign {
            count += 1;
            i -= 1;
        } else {
            break;
        }
    }
    count
}

/// Count leading zeros (CLZ) within an element of `bits`.
#[inline]
fn count_leading_zeros_elem(a: u64, bits: u32) -> u64 {
    let v = a & elem_mask(bits);
    if v == 0 {
        return bits as u64;
    }
    let mut count = 0u64;
    let mut i = bits as i32 - 1;
    while i >= 0 {
        if (v >> i) & 1 == 0 {
            count += 1;
            i -= 1;
        } else {
            break;
        }
    }
    count
}

/// One element of an Advanced SIMD two-register-miscellaneous *integer* op that
/// preserves element size (not REV / widening / narrowing / FP). `a` is the
/// source element and `d` the current destination (for SUQADD/USQADD). Returns
/// `Some(result)` or `None` if the opcode is handled elsewhere.
fn adv_simd_two_reg_int(u: u32, opcode: u32, bits: u32, a: u64, d: u64) -> Option<u64> {
    let m = elem_mask(bits);
    let sa = sext_elem(a, bits);
    Some(match (u, opcode) {
        (0, 0b00011) => sat_signed(sext_elem(d, bits) + uext_elem(a, bits) as i128, bits), // SUQADD
        (1, 0b00011) => sat_unsigned(uext_elem(d, bits) as i128 + sext_elem(a, bits), bits), // USQADD
        (0, 0b00100) => count_leading_sign(a, bits) & m,                                     // CLS
        (1, 0b00100) => count_leading_zeros_elem(a, bits) & m,                               // CLZ
        (0, 0b00101) => (a & 0xFF).count_ones() as u64, // CNT (per byte; bits==8)
        (0, 0b00111) => sat_signed(sext_elem(a, bits).abs(), bits), // SQABS
        (1, 0b00111) => sat_signed(-sext_elem(a, bits), bits), // SQNEG
        (0, 0b01000) => bool_mask(sa > 0, bits),        // CMGT #0
        (1, 0b01000) => bool_mask(sa >= 0, bits),       // CMGE #0
        (0, 0b01001) => bool_mask(sa == 0, bits),       // CMEQ #0
        (1, 0b01001) => bool_mask(sa <= 0, bits),       // CMLE #0
        (0, 0b01010) => bool_mask(sa < 0, bits),        // CMLT #0
        (0, 0b01011) => (sa.unsigned_abs() as u64) & m, // ABS
        (1, 0b01011) => ((-sa) as u64) & m,             // NEG
        _ => return None,
    })
}

/// Read an `esize`-byte little-endian element from `bytes` at `off`.
#[inline]
fn read_elem(bytes: &[u8], off: usize, esize: usize) -> u64 {
    let mut v = 0u64;
    for i in 0..esize {
        v |= (bytes[off + i] as u64) << (8 * i);
    }
    v
}

/// Write the low `esize` bytes of `val` little-endian into `bytes` at `off`.
#[inline]
fn write_elem(bytes: &mut [u8], off: usize, esize: usize, val: u64) {
    for i in 0..esize {
        bytes[off + i] = (val >> (8 * i)) as u8;
    }
}

/// Advanced SIMD three-same floating-point operation kind.
#[derive(Clone, Copy, PartialEq)]
enum FpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mulx,
    Mla,
    Mls,
    Max,
    Min,
    MaxNm,
    MinNm,
    CmEq,
    CmGe,
    CmGt,
    AcGe,
    AcGt,
    Abd,
    Recps,
    Rsqrts,
    Addp,
    Maxp,
    Minp,
    MaxNmp,
    MinNmp,
}

/// Decode the FP three-same opcode from (U, size<1>, opcode) into an `FpKind`.
fn fp_three_same_decode(u: u32, a: u32, opcode: u32) -> Option<FpKind> {
    use FpKind::*;
    Some(match (u, a, opcode) {
        (0, 0, 0b11000) => MaxNm,
        (0, 0, 0b11001) => Mla,
        (0, 0, 0b11010) => Add,
        (0, 0, 0b11011) => Mulx,
        (0, 0, 0b11100) => CmEq,
        (0, 0, 0b11110) => Max,
        (0, 0, 0b11111) => Recps,
        (0, 1, 0b11000) => MinNm,
        (0, 1, 0b11001) => Mls,
        (0, 1, 0b11010) => Sub,
        (0, 1, 0b11110) => Min,
        (0, 1, 0b11111) => Rsqrts,
        (1, 0, 0b11000) => MaxNmp,
        (1, 0, 0b11010) => Addp,
        (1, 0, 0b11011) => Mul,
        (1, 0, 0b11100) => CmGe,
        (1, 0, 0b11101) => AcGe,
        (1, 0, 0b11110) => Maxp,
        (1, 0, 0b11111) => Div,
        (1, 1, 0b11000) => MinNmp,
        (1, 1, 0b11010) => Abd,
        (1, 1, 0b11100) => CmGt,
        (1, 1, 0b11101) => AcGt,
        (1, 1, 0b11110) => Minp,
        _ => return None,
    })
}

/// FMAX per ARM: NaN propagates; +0 is greater than -0.
#[inline]
/// One element of an FP precision conversion (FCVTL/FCVTN and the scalar FCVT):
/// `src_prec`/`dst_prec` are byte widths (2=f16, 4=f32, 8=f64). NaN goes through
/// FPConvertNaN; `round_odd` selects FCVTX (f64->f32 round-to-odd, which carries
/// its own NaN handling). Round-to-nearest-even otherwise.
fn fp_cvt_elem(bits: u64, src_prec: usize, dst_prec: usize, round_odd: bool) -> u64 {
    if round_odd {
        return round_odd_f64_to_f32(f64::from_bits(bits)) as u64;
    }
    let is_nan = match src_prec {
        4 => is_nan32(bits as u32),
        8 => is_nan64(bits),
        _ => (bits as u16) & 0x7C00 == 0x7C00 && (bits as u16) & 0x3FF != 0,
    };
    if is_nan {
        return fp_convert_nan(bits, src_prec, dst_prec);
    }
    let val = match src_prec {
        4 => f32::from_bits(bits as u32) as f64,
        8 => f64::from_bits(bits),
        _ => fp16_to_f64(bits as u16),
    };
    match dst_prec {
        4 => (val as f32).to_bits() as u64,
        8 => val.to_bits(),
        _ => fp16_round(val) as u64,
    }
}

/// FRINT32/FRINT64 (FEAT_FRINTTS): round an f32 to an integral value within the
/// signed `intsize`-bit range. `z`=round-toward-zero (FRINT*Z), else round per
/// the current mode (FPCR default = ties-even). Out-of-range / NaN / Inf yield
/// INT{intsize}_MIN as a float (qemu frint_s, vfp_helper.c).
fn frint_ts_f32(bits: u32, intsize: u32, z: bool) -> u32 {
    let overflow = (0x100 + 126 + intsize) << 23;
    if (bits >> 23) & 0xFF == 0xFF {
        return overflow; // NaN / Inf
    }
    let x = f32::from_bits(bits);
    let r = if z { x.trunc() } else { x.round_ties_even() };
    let rb = r.to_bits();
    let rexp = (rb >> 23) & 0xFF;
    if rexp < 126 + intsize {
        return rb;
    }
    if rexp == 126 + intsize && (rb >> 31) & 1 == 1 && rb & 0x7F_FFFF == 0 {
        return rb; // exactly INT{intsize}_MIN
    }
    overflow
}

/// FRINT32/FRINT64 for f64 (qemu frint_d).
fn frint_ts_f64(bits: u64, intsize: u32, z: bool) -> u64 {
    let overflow = (0x800u64 + 1022 + intsize as u64) << 52;
    if (bits >> 52) & 0x7FF == 0x7FF {
        return overflow; // NaN / Inf
    }
    let x = f64::from_bits(bits);
    let r = if z { x.trunc() } else { x.round_ties_even() };
    let rb = r.to_bits();
    let rexp = ((rb >> 52) & 0x7FF) as u32;
    if rexp < 1022 + intsize {
        return rb;
    }
    if rexp == 1022 + intsize && (rb >> 63) & 1 == 1 && rb & 0xF_FFFF_FFFF_FFFF == 0 {
        return rb;
    }
    overflow
}

fn fp_max_f32(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        f32::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_positive() { a } else { b }
    } else {
        a.max(b)
    }
}
#[inline]
fn fp_min_f32(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        f32::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_negative() { a } else { b }
    } else {
        a.min(b)
    }
}
#[inline]
fn fp_max_f64(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_positive() { a } else { b }
    } else {
        a.max(b)
    }
}
#[inline]
fn fp_min_f64(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else if a == 0.0 && b == 0.0 {
        if a.is_sign_negative() { a } else { b }
    } else {
        a.min(b)
    }
}

/// Deterministic two-register-misc floating-point unary operation kind.
#[derive(Clone, Copy, PartialEq)]
enum TwoRegFp {
    Fabs,
    Fneg,
    Fsqrt,
    RintN,
    RintP,
    RintM,
    RintZ,
    RintA,
    RintX,
    RintI,
    CvtNS,
    CvtMS,
    CvtPS,
    CvtZS,
    CvtAS,
    CvtNU,
    CvtMU,
    CvtPU,
    CvtZU,
    CvtAU,
    CmGt,
    CmGe,
    CmEq,
    CmLe,
    CmLt,
}

/// Apply a two-reg-misc FP op to one f32 element (raw bits in/out).
fn fp_two_reg_f32(kind: TwoRegFp, bits: u32) -> u32 {
    use TwoRegFp::*;
    let x = f32::from_bits(bits);
    let mask = |c: bool| if c { u32::MAX } else { 0 };
    match kind {
        Fabs => x.abs().to_bits(),
        Fneg => (-x).to_bits(),
        Fsqrt => {
            if x.is_sign_negative() && x != 0.0 && !x.is_nan() {
                0x7FC0_0000 // sqrt of negative/-Inf -> default NaN (positive)
            } else {
                x.sqrt().to_bits()
            }
        }
        RintN | RintX | RintI => x.round_ties_even().to_bits(),
        RintP => x.ceil().to_bits(),
        RintM => x.floor().to_bits(),
        RintZ => x.trunc().to_bits(),
        RintA => x.round().to_bits(),
        CmGt => mask(x > 0.0),
        CmGe => mask(x >= 0.0),
        CmEq => mask(x == 0.0),
        CmLe => mask(x <= 0.0),
        CmLt => mask(x < 0.0),
        CvtNS | CvtMS | CvtPS | CvtZS | CvtAS => {
            let r = match kind {
                CvtNS => x.round_ties_even(),
                CvtMS => x.floor(),
                CvtPS => x.ceil(),
                CvtZS => x.trunc(),
                _ => x.round(),
            };
            (r as i32) as u32
        }
        CvtNU | CvtMU | CvtPU | CvtZU | CvtAU => {
            let r = match kind {
                CvtNU => x.round_ties_even(),
                CvtMU => x.floor(),
                CvtPU => x.ceil(),
                CvtZU => x.trunc(),
                _ => x.round(),
            };
            r as u32
        }
    }
}

/// Apply a two-reg-misc FP op to one f64 element (raw bits in/out).
fn fp_two_reg_f64(kind: TwoRegFp, bits: u64) -> u64 {
    use TwoRegFp::*;
    let x = f64::from_bits(bits);
    let mask = |c: bool| if c { u64::MAX } else { 0 };
    match kind {
        Fabs => x.abs().to_bits(),
        Fneg => (-x).to_bits(),
        Fsqrt => {
            if x.is_sign_negative() && x != 0.0 && !x.is_nan() {
                0x7FF8_0000_0000_0000 // sqrt of negative/-Inf -> default NaN
            } else {
                x.sqrt().to_bits()
            }
        }
        RintN | RintX | RintI => x.round_ties_even().to_bits(),
        RintP => x.ceil().to_bits(),
        RintM => x.floor().to_bits(),
        RintZ => x.trunc().to_bits(),
        RintA => x.round().to_bits(),
        CmGt => mask(x > 0.0),
        CmGe => mask(x >= 0.0),
        CmEq => mask(x == 0.0),
        CmLe => mask(x <= 0.0),
        CmLt => mask(x < 0.0),
        CvtNS | CvtMS | CvtPS | CvtZS | CvtAS => {
            let r = match kind {
                CvtNS => x.round_ties_even(),
                CvtMS => x.floor(),
                CvtPS => x.ceil(),
                CvtZS => x.trunc(),
                _ => x.round(),
            };
            (r as i64) as u64
        }
        CvtNU | CvtMU | CvtPU | CvtZU | CvtAU => {
            let r = match kind {
                CvtNU => x.round_ties_even(),
                CvtMU => x.floor(),
                CvtPU => x.ceil(),
                CvtZU => x.trunc(),
                _ => x.round(),
            };
            r as u64
        }
    }
}

/// Compute one f32 element of an Advanced SIMD three-same FP operation.
#[inline]
fn is_nan32(x: u32) -> bool {
    x & 0x7F80_0000 == 0x7F80_0000 && x & 0x007F_FFFF != 0
}
#[inline]
fn is_snan32(x: u32) -> bool {
    is_nan32(x) && x & 0x0040_0000 == 0
}
#[inline]
fn is_nan64(x: u64) -> bool {
    x & 0x7FF0_0000_0000_0000 == 0x7FF0_0000_0000_0000 && x & 0x000F_FFFF_FFFF_FFFF != 0
}
#[inline]
fn is_snan64(x: u64) -> bool {
    is_nan64(x) && x & 0x0008_0000_0000_0000 == 0
}

/// ARM FPProcessNaNs for two f32 operands (FPCR.DN=0): a signaling NaN is
/// quieted (sign+payload preserved), a quiet NaN is returned as-is; sNaN takes
/// priority, then operand order.
fn fp32_nan2(a: u32, b: u32) -> Option<u32> {
    if is_snan32(a) {
        Some(a | 0x0040_0000)
    } else if is_snan32(b) {
        Some(b | 0x0040_0000)
    } else if is_nan32(a) {
        Some(a)
    } else if is_nan32(b) {
        Some(b)
    } else {
        None
    }
}
/// FPProcessNaNs over three f32 operands (for the fused multiply-add forms),
/// processed in (addend, op1, op2) order as ARM FPMulAdd does.
fn fp32_nan3(a: u32, b: u32, c: u32) -> Option<u32> {
    for &x in &[a, b, c] {
        if is_snan32(x) {
            return Some(x | 0x0040_0000);
        }
    }
    for &x in &[a, b, c] {
        if is_nan32(x) {
            return Some(x);
        }
    }
    None
}
fn fp64_nan2(a: u64, b: u64) -> Option<u64> {
    if is_snan64(a) {
        Some(a | 0x0008_0000_0000_0000)
    } else if is_snan64(b) {
        Some(b | 0x0008_0000_0000_0000)
    } else if is_nan64(a) {
        Some(a)
    } else if is_nan64(b) {
        Some(b)
    } else {
        None
    }
}
fn fp64_nan3(a: u64, b: u64, c: u64) -> Option<u64> {
    for &x in &[a, b, c] {
        if is_snan64(x) {
            return Some(x | 0x0008_0000_0000_0000);
        }
    }
    for &x in &[a, b, c] {
        if is_nan64(x) {
            return Some(x);
        }
    }
    None
}

fn fp_three_same_f32(kind: FpKind, a: u32, b: u32, d: u32) -> u32 {
    use FpKind::*;
    // ARM NaN handling (FPCR.DN=0, the qemu-user default): a NaN input
    // propagates quieted; an invalid operation on non-NaN inputs yields the
    // default NaN 0x7FC00000 (native x86 arithmetic would give 0xFFC00000).
    match kind {
        // FABD = FPAbs(FPSub(a,b)): a propagated NaN has its sign cleared.
        Abd => {
            if let Some(n) = fp32_nan2(a, b) {
                return n & 0x7FFF_FFFF;
            }
        }
        Add | Sub | Mul | Div | Mulx | Addp | Max | Maxp | Min | Minp => {
            if let Some(n) = fp32_nan2(a, b) {
                return n;
            }
        }
        Mla => {
            if let Some(n) = fp32_nan3(d, a, b) {
                return n;
            }
        }
        // FMLS negates op1 (a) before the fused multiply-add, flipping its NaN
        // sign for propagation.
        Mls => {
            if let Some(n) = fp32_nan3(d, a ^ 0x8000_0000, b) {
                return n;
            }
        }
        MaxNm | MaxNmp | MinNm | MinNmp => {
            // sNaN propagates (quieted); a lone qNaN loses to the number below.
            if is_snan32(a) {
                return a | 0x0040_0000;
            }
            if is_snan32(b) {
                return b | 0x0040_0000;
            }
        }
        // Recps/Rsqrts delegate to sve_recps/sve_rsqrts (fused + FPNeg-first NaN).
        _ => {}
    }
    let x = f32::from_bits(a);
    let y = f32::from_bits(b);
    let acc = f32::from_bits(d);
    let mask = |c: bool| if c { u32::MAX } else { 0 };
    let canon = |r: f32| -> u32 { if r.is_nan() { 0x7FC0_0000 } else { r.to_bits() } };
    let inf0 = |p: f32, q: f32| (p.is_infinite() && q == 0.0) || (p == 0.0 && q.is_infinite());
    match kind {
        Add => canon(x + y),
        Sub => canon(x - y),
        Mul => canon(x * y),
        Div => canon(x / y),
        Mulx => {
            if inf0(x, y) {
                // FMULX(inf,0)=+/-2.0 with sign = sign(x) XOR sign(y).
                let neg = x.is_sign_negative() ^ y.is_sign_negative();
                (if neg { -2.0f32 } else { 2.0f32 }).to_bits()
            } else {
                canon(x * y)
            }
        }
        Mla => canon(x.mul_add(y, acc)),
        Mls => canon((-x).mul_add(y, acc)),
        Max | Maxp => fp_max_f32(x, y).to_bits(),
        Min | Minp => fp_min_f32(x, y).to_bits(),
        MaxNm | MaxNmp => {
            let (aq, bq) = (is_nan32(a), is_nan32(b));
            if aq && bq {
                a
            } else if aq {
                b
            } else if bq {
                a
            } else {
                fp_max_f32(x, y).to_bits()
            }
        }
        MinNm | MinNmp => {
            let (aq, bq) = (is_nan32(a), is_nan32(b));
            if aq && bq {
                a
            } else if aq {
                b
            } else if bq {
                a
            } else {
                fp_min_f32(x, y).to_bits()
            }
        }
        CmEq => mask(x == y),
        CmGe => mask(x >= y),
        CmGt => mask(x > y),
        AcGe => mask(x.abs() >= y.abs()),
        AcGt => mask(x.abs() > y.abs()),
        Abd => canon((x - y).abs()),
        Recps => sve_recps(4, a as u64, b as u64) as u32,
        Rsqrts => sve_rsqrts(4, a as u64, b as u64) as u32,
        Addp => canon(x + y),
    }
}

/// Compute one f64 element of an Advanced SIMD three-same FP operation.
fn fp_three_same_f64(kind: FpKind, a: u64, b: u64, d: u64) -> u64 {
    use FpKind::*;
    match kind {
        Abd => {
            if let Some(n) = fp64_nan2(a, b) {
                return n & 0x7FFF_FFFF_FFFF_FFFF;
            }
        }
        Add | Sub | Mul | Div | Mulx | Addp | Max | Maxp | Min | Minp => {
            if let Some(n) = fp64_nan2(a, b) {
                return n;
            }
        }
        Mla => {
            if let Some(n) = fp64_nan3(d, a, b) {
                return n;
            }
        }
        Mls => {
            if let Some(n) = fp64_nan3(d, a ^ 0x8000_0000_0000_0000, b) {
                return n;
            }
        }
        MaxNm | MaxNmp | MinNm | MinNmp => {
            if is_snan64(a) {
                return a | 0x0008_0000_0000_0000;
            }
            if is_snan64(b) {
                return b | 0x0008_0000_0000_0000;
            }
        }
        _ => {}
    }
    let x = f64::from_bits(a);
    let y = f64::from_bits(b);
    let acc = f64::from_bits(d);
    let mask = |c: bool| if c { u64::MAX } else { 0 };
    let canon = |r: f64| -> u64 {
        if r.is_nan() {
            0x7FF8_0000_0000_0000
        } else {
            r.to_bits()
        }
    };
    let inf0 = |p: f64, q: f64| (p.is_infinite() && q == 0.0) || (p == 0.0 && q.is_infinite());
    match kind {
        Add => canon(x + y),
        Sub => canon(x - y),
        Mul => canon(x * y),
        Div => canon(x / y),
        Mulx => {
            if inf0(x, y) {
                let neg = x.is_sign_negative() ^ y.is_sign_negative();
                (if neg { -2.0f64 } else { 2.0f64 }).to_bits()
            } else {
                canon(x * y)
            }
        }
        Mla => canon(x.mul_add(y, acc)),
        Mls => canon((-x).mul_add(y, acc)),
        Max | Maxp => fp_max_f64(x, y).to_bits(),
        Min | Minp => fp_min_f64(x, y).to_bits(),
        MaxNm | MaxNmp => {
            let (aq, bq) = (is_nan64(a), is_nan64(b));
            if aq && bq {
                a
            } else if aq {
                b
            } else if bq {
                a
            } else {
                fp_max_f64(x, y).to_bits()
            }
        }
        MinNm | MinNmp => {
            let (aq, bq) = (is_nan64(a), is_nan64(b));
            if aq && bq {
                a
            } else if aq {
                b
            } else if bq {
                a
            } else {
                fp_min_f64(x, y).to_bits()
            }
        }
        CmEq => mask(x == y),
        CmGe => mask(x >= y),
        CmGt => mask(x > y),
        AcGe => mask(x.abs() >= y.abs()),
        AcGt => mask(x.abs() > y.abs()),
        Abd => canon((x - y).abs()),
        Recps => sve_recps(8, a, b),
        Rsqrts => sve_rsqrts(8, a, b),
        Addp => canon(x + y),
    }
}

/// ARM RecipEstimate integer core (input a in [256,512)).
fn recip_estimate(a: u32) -> u32 {
    let a = a * 2 + 1;
    let b = (1u32 << 19) / a;
    (b + 1) >> 1
}

/// FRECPE for f32 (normal inputs).
fn fp_recip_estimate_f32(bits: u32) -> u32 {
    let sign = bits >> 31;
    let exp = (bits >> 23) & 0xFF;
    let frac = bits & 0x7F_FFFF;
    if exp == 0xFF {
        return if frac != 0 {
            bits | 0x40_0000
        } else {
            sign << 31
        }; // NaN->qNaN, inf->0
    }
    if exp == 0 && frac == 0 {
        return (sign << 31) | (0xFF << 23); // zero -> infinity
    }
    // |value| < 2^-128 -> the reciprocal overflows -> +/-inf (round-to-nearest).
    if exp == 0 && frac < 0x20_0000 {
        return (sign << 31) | (0xFF << 23);
    }
    // Work with a 52-bit fraction (ASL maps the f32 significand to f64 width).
    let mut fraction: u64 = (frac as u64) << 29;
    let mut e = exp as i32;
    if e == 0 {
        // Normalise a denormal input (value >= 2^-128).
        if (fraction >> 51) & 1 == 0 {
            e = -1;
            fraction = (fraction << 2) & ((1u64 << 52) - 1);
        } else {
            fraction = (fraction << 1) & ((1u64 << 52) - 1);
        }
    }
    let scaled = 0x100 | ((fraction >> 44) & 0xFF) as u32;
    let estimate = recip_estimate(scaled);
    let mut result_exp = 253i32 - e;
    let mut out_frac: u64 = ((estimate & 0xFF) as u64) << 44;
    if result_exp == 0 {
        out_frac = (1u64 << 51) | (out_frac >> 1); // denormal output
    } else if result_exp == -1 {
        out_frac = (1u64 << 50) | (out_frac >> 2);
        result_exp = 0;
    }
    (sign << 31) | (((result_exp as u32) & 0xFF) << 23) | ((out_frac >> 29) as u32 & 0x7F_FFFF)
}

/// FRECPE for f64 (normal inputs).
fn fp_recip_estimate_f64(bits: u64) -> u64 {
    let sign = bits >> 63;
    let exp = ((bits >> 52) & 0x7FF) as i32;
    let frac = bits & 0xF_FFFF_FFFF_FFFF;
    if exp == 0x7FF {
        return if frac != 0 {
            bits | 0x8_0000_0000_0000
        } else {
            sign << 63
        };
    }
    if exp == 0 && frac == 0 {
        return (sign << 63) | (0x7FFu64 << 52);
    }
    // |value| < 2^-1024 -> overflow -> +/-inf.
    if exp == 0 && frac < (1u64 << 50) {
        return (sign << 63) | (0x7FFu64 << 52);
    }
    let mut fraction = frac;
    let mut e = exp;
    if e == 0 {
        if (fraction >> 51) & 1 == 0 {
            e = -1;
            fraction = (fraction << 2) & ((1u64 << 52) - 1);
        } else {
            fraction = (fraction << 1) & ((1u64 << 52) - 1);
        }
    }
    let scaled = 0x100 | ((fraction >> 44) & 0xFF) as u32;
    let estimate = recip_estimate(scaled);
    let mut result_exp = 2045i32 - e;
    let mut out_frac: u64 = ((estimate & 0xFF) as u64) << 44;
    if result_exp == 0 {
        out_frac = (1u64 << 51) | (out_frac >> 1);
    } else if result_exp == -1 {
        out_frac = (1u64 << 50) | (out_frac >> 2);
        result_exp = 0;
    }
    (sign << 63) | (((result_exp as u64) & 0x7FF) << 52) | (out_frac & 0xF_FFFF_FFFF_FFFF)
}

/// ARM RecipSqrtEstimate integer core (input a in [128,512)).
fn recip_sqrt_estimate(mut a: u32) -> u32 {
    if a < 256 {
        a = a * 2 + 1;
    } else {
        a = (a >> 1) << 1;
        a = (a + 1) * 2;
    }
    let a = a as u64;
    let mut b: u64 = 512;
    while a * (b + 1) * (b + 1) < (1u64 << 28) {
        b += 1;
    }
    ((b + 1) >> 1) as u32
}

/// FRSQRTE for f32.
fn fp_rsqrt_estimate_f32(bits: u32) -> u32 {
    let sign = bits >> 31;
    let exp = (bits >> 23) & 0xFF;
    let frac = bits & 0x7F_FFFF;
    if exp == 0xFF && frac != 0 {
        return bits | 0x40_0000;
    } // NaN -> qNaN
    if exp == 0 && frac == 0 {
        return (sign << 31) | (0xFF << 23);
    } // zero -> inf
    if sign == 1 {
        return 0x7FC0_0000;
    } // negative -> default NaN
    if exp == 0xFF {
        return 0;
    } // +inf -> +0
    let mut fraction: u64 = (frac as u64) << 29; // bits<51:29>
    let mut e = exp as i32;
    if e == 0 {
        while (fraction >> 51) & 1 == 0 {
            fraction = (fraction << 1) & 0xF_FFFF_FFFF_FFFF;
            e -= 1;
        }
        fraction = (fraction << 1) & 0xF_FFFF_FFFF_FFFF;
    }
    let scaled = if e & 1 == 0 {
        0x100 | ((fraction >> 44) & 0xFF) as u32
    } else {
        0x80 | ((fraction >> 45) & 0x7F) as u32
    };
    let result_exp = (((380 - e) / 2) as u32) & 0xFF;
    let est = recip_sqrt_estimate(scaled);
    (sign << 31) | (result_exp << 23) | ((est & 0xFF) << 15)
}

/// FRSQRTE for f64.
fn fp_rsqrt_estimate_f64(bits: u64) -> u64 {
    let sign = bits >> 63;
    let exp = ((bits >> 52) & 0x7FF) as i32;
    let frac = bits & 0xF_FFFF_FFFF_FFFF;
    if exp == 0x7FF && frac != 0 {
        return bits | 0x8_0000_0000_0000;
    }
    if exp == 0 && frac == 0 {
        return (sign << 63) | (0x7FFu64 << 52);
    }
    if sign == 1 {
        return 0x7FF8_0000_0000_0000;
    }
    if exp == 0x7FF {
        return 0;
    }
    let mut fraction: u64 = frac;
    let mut e = exp;
    if e == 0 {
        while (fraction >> 51) & 1 == 0 {
            fraction = (fraction << 1) & 0xF_FFFF_FFFF_FFFF;
            e -= 1;
        }
        fraction = (fraction << 1) & 0xF_FFFF_FFFF_FFFF;
    }
    let scaled = if e & 1 == 0 {
        0x100 | ((fraction >> 44) & 0xFF) as u32
    } else {
        0x80 | ((fraction >> 45) & 0x7F) as u32
    };
    let result_exp = (((3068 - e) / 2) as u64) & 0x7FF;
    let est = recip_sqrt_estimate(scaled);
    (sign << 63) | (result_exp << 52) | (((est & 0xFF) as u64) << 44)
}

/// UnsignedRecipEstimate (N=32): estimate of 1/x for a fixed-point value.
fn unsigned_recip_estimate(op: u32) -> u32 {
    if op & 0x8000_0000 == 0 {
        return 0xFFFF_FFFF;
    }
    let est = recip_estimate((op >> 23) & 0x1FF);
    (est & 0x1FF) << 23
}

/// UnsignedRSqrtEstimate (N=32).
fn unsigned_rsqrt_estimate(op: u32) -> u32 {
    if op & 0xC000_0000 == 0 {
        return 0xFFFF_FFFF;
    }
    let est = recip_sqrt_estimate((op >> 23) & 0x1FF);
    (est & 0x1FF) << 23
}

// ---- Precision-generic FP element helpers (esize in bits: 16/32/64) ----

/// Flip the sign bit of a floating-point element.
fn fp_neg_bits(b: u64, esize: u32) -> u64 {
    b ^ (1u64 << (esize - 1))
}

/// FPAdd over a binary16/32/64 element.
fn fp_add_bits(a: u64, b: u64, esize: u32) -> u64 {
    match esize {
        16 => fp16_add(a as u16, b as u16) as u64,
        32 => fp_three_same_f32(FpKind::Add, a as u32, b as u32, 0) as u64,
        _ => fp_three_same_f64(FpKind::Add, a, b, 0),
    }
}

/// FPMulAdd (fused): `acc + x*y` over a binary16/32/64 element.
fn fp_muladd_bits(acc: u64, x: u64, y: u64, esize: u32) -> u64 {
    match esize {
        16 => fp16_mla(acc as u16, x as u16, y as u16) as u64,
        32 => fp_three_same_f32(FpKind::Mla, x as u32, y as u32, acc as u32) as u64,
        _ => fp_three_same_f64(FpKind::Mla, x, y, acc),
    }
}

// ---- SVE predicate helpers ----

/// Number of leading active elements selected by an SVE predicate `pattern`
/// (POW2/VL1..VL256/MUL3/MUL4/ALL) given the element count. Unallocated
/// patterns select zero elements.
fn sve_pattern_count(pattern: u32, elements: usize) -> usize {
    match pattern {
        0b00000 => {
            // POW2: largest power of two <= elements.
            let mut p = 1;
            while p * 2 <= elements {
                p *= 2;
            }
            p
        }
        0b00001..=0b00111 => {
            let c = pattern as usize; // VL1..VL7
            if c <= elements { c } else { 0 }
        }
        0b01000 => (8 <= elements).then_some(8).unwrap_or(0),
        0b01001 => (16 <= elements).then_some(16).unwrap_or(0),
        0b01010 => (32 <= elements).then_some(32).unwrap_or(0),
        0b01011 => (64 <= elements).then_some(64).unwrap_or(0),
        0b01100 => (128 <= elements).then_some(128).unwrap_or(0),
        0b01101 => (256 <= elements).then_some(256).unwrap_or(0),
        0b11101 => (elements / 4) * 4, // MUL4
        0b11110 => (elements / 3) * 3, // MUL3
        0b11111 => elements,           // ALL
        _ => 0,
    }
}

/// NZCV produced by an SVE predicate-setting op (PTEST convention with an
/// all-true governing predicate): N=First active, Z=None active, C=!Last
/// active, V=0. `pred` is byte-granular; element `e` is bit `e*esize`.
/// Sign-extend an MTE-tagged address from bit 55 (the address part is bits
/// [55:0]; the logical/physical tags above are ignored for SUBP/SUBPS).
fn sign_extend_56(v: u64) -> u64 {
    ((v << 8) as i64 >> 8) as u64
}

/// 64-bit subtract returning (result, N, Z, C, V).
fn sub_with_flags_64(a: u64, b: u64) -> (u64, bool, bool, bool, bool) {
    let (res, borrow) = a.overflowing_sub(b);
    let c = !borrow; // ARM carry = NOT borrow
    let v = (((a ^ b) & (a ^ res)) >> 63) & 1 == 1;
    (res, (res >> 63) & 1 == 1, res == 0, c, v)
}

/// Deterministic stand-in for the PACGA generic MAC (bits[63:32]); good
/// enough for tests that only check the destination is written.
fn pacga_stub(x: u64, y: u64) -> u64 {
    let mut h = x ^ y.rotate_left(32) ^ 0x9E37_79B9_7F4A_7C15;
    h ^= h >> 29;
    h & 0xFFFF_FFFF_0000_0000
}

fn pred_test_flags(pred: u32, elements: usize, esize: usize) -> (bool, bool, bool, bool) {
    let first = pred & 1 != 0;
    let none = pred == 0;
    let last = (pred >> ((elements - 1) * esize)) & 1 != 0;
    (first, none, !last, false)
}

/// General SVE PredTest(mask, result): N=is the first mask-active element set in
/// result, Z=no mask-active element is set, C=!is the last mask-active element
/// set, V=0. Both predicates are byte-granular.
fn pred_test(mask: u32, result: u32, elements: usize, esize: usize) -> (bool, bool, bool, bool) {
    let mut n = false;
    let mut first = true;
    let mut z = true;
    let mut last_r = false;
    for e in 0..elements {
        let b = e * esize;
        if (mask >> b) & 1 == 1 {
            let r = (result >> b) & 1 == 1;
            if first {
                n = r;
                first = false;
            }
            if r {
                z = false;
            }
            last_r = r;
        }
    }
    (n, z, !last_r, false)
}

/// Decode the 4-bit SVE contiguous-load `dtype` field into the destination
/// element size, the memory access size (both in bytes) and whether the loaded
/// value is sign-extended. msize <= esize always; signed loads sign-extend.
fn sve_ld1_dtype(dtype: u32) -> (usize, usize, bool) {
    match dtype {
        0b0000 => (1, 1, false), // LD1B  -> 8
        0b0001 => (2, 1, false), // LD1B  -> 16
        0b0010 => (4, 1, false), // LD1B  -> 32
        0b0011 => (8, 1, false), // LD1B  -> 64
        0b0100 => (8, 4, true),  // LD1SW -> 64
        0b0101 => (2, 2, false), // LD1H  -> 16
        0b0110 => (4, 2, false), // LD1H  -> 32
        0b0111 => (8, 2, false), // LD1H  -> 64
        0b1000 => (8, 2, true),  // LD1SH -> 64
        0b1001 => (4, 2, true),  // LD1SH -> 32
        0b1010 => (4, 4, false), // LD1W  -> 32
        0b1011 => (8, 4, false), // LD1W  -> 64
        0b1100 => (8, 1, true),  // LD1SB -> 64
        0b1101 => (4, 1, true),  // LD1SB -> 32
        0b1110 => (2, 1, true),  // LD1SB -> 16
        _ => (8, 8, false),      // 1111: LD1D -> 64
    }
}

/// SVE LastActive(mask, operand): true iff the highest-indexed mask-active
/// element is set in `operand`. Both predicates are byte-granular (element `e`
/// of size `esize` bytes is governed by bit `e*esize`).
fn last_active(mask: u32, operand: u32, elements: usize, esize: usize) -> bool {
    for e in (0..elements).rev() {
        let b = e * esize;
        if (mask >> b) & 1 == 1 {
            return (operand >> b) & 1 == 1;
        }
    }
    false
}

/// Combine two FP element bit-values with an `FpKind` op at the given esize,
/// reusing the verified binary16/32/64 helpers (for SVE FP reductions/FADDA).
fn sve_fp_combine(kind: FpKind, esize: usize, x: u64, y: u64) -> u64 {
    match esize {
        2 => sve_fp16_binop(kind, x as u16, y as u16) as u64,
        4 => fp_three_same_f32(kind, x as u32, y as u32, 0) as u64,
        _ => fp_three_same_f64(kind, x, y, 0),
    }
}

/// Recursive split-in-half binary-tree reduction (the SVE "fast" reduction
/// order): combine(reduce(low half), reduce(high half)). The low-index half is
/// ALWAYS the first operand — this exact order is required for FP bit-exactness
/// (FPAdd is non-associative; FPMax/FPMin sign-of-zero depends on position).
fn sve_fp_tree_reduce(buf: &[u64], kind: FpKind, esize: usize) -> u64 {
    if buf.len() == 1 {
        return buf[0];
    }
    let h = buf.len() / 2;
    let lo = sve_fp_tree_reduce(&buf[..h], kind, esize);
    let hi = sve_fp_tree_reduce(&buf[h..], kind, esize);
    sve_fp_combine(kind, esize, lo, hi)
}

/// Identity element used to pad inactive lanes in an SVE FP reduction.
fn sve_fp_identity(kind: FpKind, esize: usize) -> u64 {
    use FpKind::*;
    match kind {
        Add => 0, // +0.0
        // The max identity must never win the max -> -Inf; the min identity -> +Inf.
        Max => match esize {
            2 => 0xFC00,
            4 => 0xFF80_0000,
            _ => 0xFFF0_0000_0000_0000,
        }, // -Inf
        Min => match esize {
            2 => 0x7C00,
            4 => 0x7F80_0000,
            _ => 0x7FF0_0000_0000_0000,
        }, // +Inf
        _ => match esize {
            2 => 0x7E00,
            4 => 0x7FC0_0000,
            _ => 0x7FF8_0000_0000_0000,
        }, // default NaN (FMAXNM/FMINNM)
    }
}

/// FRECPX (reciprocal exponent) over an f32/f64 element bit-value.
fn sve_fp_recpx(esize: usize, lane: u64) -> u64 {
    match esize {
        2 => fp16_recpx(lane as u16) as u64,
        4 => {
            let x = lane as u32;
            if (x & 0x7F80_0000) == 0x7F80_0000 && (x & 0x7F_FFFF) != 0 {
                return (x | 0x40_0000) as u64; // NaN -> quiet
            }
            let sign = x & 0x8000_0000;
            let exp = (x >> 23) & 0xFF;
            (if exp == 0 {
                sign | (0xFE << 23)
            } else {
                sign | ((!exp & 0xFF) << 23)
            }) as u64
        }
        _ => {
            let x = lane;
            if (x & 0x7FF0_0000_0000_0000) == 0x7FF0_0000_0000_0000 && (x & 0xF_FFFF_FFFF_FFFF) != 0
            {
                return x | 0x8_0000_0000_0000; // NaN -> quiet
            }
            let sign = x & 0x8000_0000_0000_0000;
            let exp = (x >> 52) & 0x7FF;
            if exp == 0 {
                sign | (0x7FE << 52)
            } else {
                sign | ((!exp & 0x7FF) << 52)
            }
        }
    }
}

// ---- BFloat16 (bf16) helpers (FEAT_BF16) ----

/// Widen a bf16 to f32 — exact (bf16 is the top 16 bits of an f32).
#[inline]
fn bf16_to_f32(b: u16) -> f32 {
    f32::from_bits((b as u32) << 16)
}

/// Convert an f32 (raw bits) to bf16 with round-to-nearest-even (the rounding
/// used by BFCVT/BFCVTN; FPCR rounding mode is ignored). NaN is quieted.
fn f32_to_bf16(x: u32) -> u16 {
    if (x & 0x7F80_0000) == 0x7F80_0000 {
        // Inf or NaN.
        if (x & 0x007F_FFFF) != 0 {
            // NaN: quiet it (set bf16 quiet bit), preserve sign.
            return ((x >> 16) as u16) | 0x0040;
        }
        return (x >> 16) as u16; // +/- Inf -> 0x7F80 / 0xFF80
    }
    // Round-to-nearest-even on the dropped low 16 mantissa bits. The add-bias
    // trick also carries correctly into the exponent (overflow -> bf16 Inf) and
    // handles subnormals/zero.
    let lsb = (x >> 16) & 1;
    let rounded = x.wrapping_add(0x7FFF + lsb);
    (rounded >> 16) as u16
}

/// FEAT_SVE_B16B16 bf16 binary op (BFADD/BFSUB/BFMUL/BFMAX/BFMIN/BFMAXNM/
/// BFMINNM), ARM-correct at FPCR defaults. bf16 widens to f32 exactly (16-bit
/// left shift). NaN/Inf operands and the MUL/MAX/MIN family resolve through the
/// verified ARM f32 op then narrow round-to-nearest-even (a bf16 product is
/// exact in f32, and MAX/MIN return an operand, so a single narrow is exact).
/// Finite ADD/SUB accumulate the exact sum in f64 and narrow once (round-to-odd
/// to f32 then RNE to bf16) to avoid the f32-then-bf16 double rounding.
fn bf16_binop(kind: FpKind, a: u16, b: u16) -> u16 {
    let af = (a as u32) << 16;
    let bf = (b as u32) << 16;
    let non_finite = |x: u32| x & 0x7F80_0000 == 0x7F80_0000;
    if !matches!(kind, FpKind::Add | FpKind::Sub) || non_finite(af) || non_finite(bf) {
        return f32_to_bf16(fp_three_same_f32(kind, af, bf, 0));
    }
    let x = f32::from_bits(af) as f64;
    let y = f32::from_bits(bf) as f64;
    let s = if matches!(kind, FpKind::Sub) {
        x - y
    } else {
        x + y
    };
    f32_to_bf16(round_odd_f64_to_f32(s))
}

/// FEAT_SVE_B16B16 bf16 fused multiply-add (BFMLA: Zda+Zn*Zm, BFMLS: negate Zn
/// first). Finite operands accumulate the exact product in f64 then narrow once
/// (the bf16*bf16 product is exact in f64); NaN/Inf use the verified f32 fused
/// multiply-add. The negate-input form matches the FPCR.AH=0 oracle behaviour.
fn bf16_fma(a: u16, n: u16, m: u16, sub: bool) -> u16 {
    let af = (a as u32) << 16;
    let nf = ((n ^ if sub { 0x8000 } else { 0 }) as u32) << 16;
    let mf = (m as u32) << 16;
    let non_finite = |x: u32| x & 0x7F80_0000 == 0x7F80_0000;
    if non_finite(af) || non_finite(nf) || non_finite(mf) {
        return f32_to_bf16(fp_muladd_bits(af as u64, nf as u64, mf as u64, 32) as u32);
    }
    let p = (f32::from_bits(nf) as f64) * (f32::from_bits(mf) as f64);
    let s = (f32::from_bits(af) as f64) + p;
    f32_to_bf16(round_odd_f64_to_f32(s))
}

/// One lane of the FEAT_SVE2p1 FP 2-way dot product FDOT (f16 -> f32): the f32
/// accumulator `sum` plus the two f16 products of the 32-bit groups e1, e2,
/// computed with a single rounding (ARM FPDot) then a separate (non-fused) f32
/// accumulate. Faithful port of qemu f16_dotadd: NaN inputs follow
/// FPProcessNaNs4 (first signalling, else first quiet, widened+quieted); finite
/// products are exact in f64, summed via a Knuth 2Sum and rounded once to f32
/// (round-to-odd of the exact sum -> RNE-to-f32 is double-rounding-safe).
fn f16_dotadd(sum: u32, e1: u32, e2: u32) -> u32 {
    let lanes = [e1 as u16, (e1 >> 16) as u16, e2 as u16, (e2 >> 16) as u16];
    let is_nan = |h: u16| (h & 0x7C00) == 0x7C00 && (h & 0x3FF) != 0;
    let is_snan = |h: u16| is_nan(h) && (h & 0x0200) == 0; // f16 quiet bit = bit 9
    let t32: u32 = if lanes.iter().any(|&h| is_nan(h)) {
        let pick = lanes
            .iter()
            .copied()
            .find(|&h| is_snan(h))
            .or_else(|| lanes.iter().copied().find(|&h| is_nan(h)))
            .unwrap();
        AArch64Cpu::fp16_to_f32(pick).to_bits() | 0x0040_0000 // quieted
    } else {
        let f = |h: u16| fp16_to_f64(h);
        let p1 = f(lanes[0]) * f(lanes[2]); // h1r*h2r (exact in f64)
        let p2 = f(lanes[1]) * f(lanes[3]); // h1c*h2c (exact in f64)
        // Knuth 2Sum: hi + lo == p1 + p2 exactly.
        let hi = p1 + p2;
        let v = hi - p1;
        let lo = (p1 - (hi - v)) + (p2 - v);
        // Round the exact sum once to f32: round-to-odd in f64 (force the
        // mantissa odd when inexact) then RNE narrow is double-rounding-safe.
        let s_odd = if lo != 0.0 {
            f64::from_bits(hi.to_bits() | 1)
        } else {
            hi
        };
        (s_odd as f32).to_bits()
    };
    fp_three_same_f32(FpKind::Add, sum, t32, 0)
}

/// One round-to-odd f32 add step (`a + b` rounded once to f32, returned widened
/// to f64 for chaining). The BF16 dot/matrix instructions accumulate as a
/// sequence of these per-pair round-to-odd adds (matching the hardware), NOT a
/// single round of the exact multi-term sum.
#[inline]
fn bf_odd_add(a: f64, b: f64) -> f64 {
    f32::from_bits(round_odd_f64_to_f32(a + b)) as f64
}

/// Round an f64 to f32 with round-to-odd (Von Neumann): truncate toward zero,
/// and if any bits were discarded force the result mantissa LSB to 1. Used for
/// the unrounded BF16 dot-product accumulation (FPCR.EBF==0). The f64 input is
/// assumed to be the exact value (callers keep the exponent span small enough
/// that the f64 sum is exact).
fn round_odd_f64_to_f32(x: f64) -> u32 {
    if x.is_nan() {
        // FPConvertNaN (FPCR.DN=0): preserve sign and the top 23 fraction bits,
        // forcing the quiet bit (an sNaN is quieted, signalling InvalidOp which
        // the oracle does not compare). FCVTX/FCVTXNT are NOT default-NaN ops.
        let b = x.to_bits();
        let sign = ((b >> 63) as u32) << 31;
        let frac = ((b >> 29) as u32 & 0x7F_FFFF) | 0x40_0000;
        return sign | 0x7F80_0000 | frac;
    }
    let sign = ((x.is_sign_negative()) as u32) << 31;
    let a = x.abs();
    if a == 0.0 {
        return sign;
    }
    if a.is_infinite() {
        return sign | 0x7F80_0000;
    }
    let bits = a.to_bits();
    let exp = ((bits >> 52) & 0x7FF) as i64 - 1023; // unbiased, `a` is normal f64
    let mant = bits & 0x000F_FFFF_FFFF_FFFF; // 52-bit fraction
    if exp > 127 {
        return sign | 0x7F7F_FFFF; // round-to-odd never overflows to Inf
    }
    if exp >= -126 {
        // Normal f32: keep the top 23 fraction bits, OR in sticky for round-odd.
        let frac = (mant >> 29) as u32;
        let dropped = mant & ((1u64 << 29) - 1);
        let f = if dropped != 0 { frac | 1 } else { frac };
        let e = (exp + 127) as u32;
        return sign | (e << 23) | f;
    }
    // Subnormal f32: value = 1.mant * 2^exp, exp <= -127.
    let sig = (1u64 << 52) | mant;
    let shift = (-(exp + 97)) as u32; // value * 2^149 == sig >> shift
    if shift >= 64 {
        return sign | 1; // tiny nonzero -> smallest subnormal under round-odd
    }
    let frac = (sig >> shift) as u32 & 0x7F_FFFF;
    let dropped = sig & ((1u64 << shift) - 1);
    let f = if dropped != 0 { frac | 1 } else { frac };
    sign | f
}

/// Round an f64 to f32 with round-to-odd-INF (the BF16 dot-product rounding,
/// FPCR.EBF==0): like `round_odd_f64_to_f32` but overflow rounds to infinity
/// (not the max finite), and any NaN collapses to the default NaN 0x7FC00000
/// (default-NaN mode is forced for these instructions). The f64 input is the
/// exact value to round (the add path pre-rounds to odd at f64 precision so the
/// final f32 round-to-odd is double-rounding-safe).
fn round_odd_inf_f64_to_f32(x: f64) -> u32 {
    if x.is_nan() {
        return 0x7FC0_0000; // default NaN
    }
    let sign = (x.is_sign_negative() as u32) << 31;
    let a = x.abs();
    if a == 0.0 {
        return sign;
    }
    if a.is_infinite() {
        return sign | 0x7F80_0000;
    }
    let bits = a.to_bits();
    let exp = ((bits >> 52) & 0x7FF) as i64 - 1023;
    let mant = bits & 0x000F_FFFF_FFFF_FFFF;
    if exp > 127 {
        return sign | 0x7F80_0000; // round-to-odd-INF: overflow -> infinity
    }
    if exp >= -126 {
        let frac = (mant >> 29) as u32;
        let dropped = mant & ((1u64 << 29) - 1);
        let f = if dropped != 0 { frac | 1 } else { frac };
        let e = (exp + 127) as u32;
        return sign | (e << 23) | f;
    }
    // Subnormal f32: value = 1.mant * 2^exp, exp <= -127.
    let sig = (1u64 << 52) | mant;
    let shift = (-(exp + 97)) as u32;
    if shift >= 64 {
        return sign | 1;
    }
    let frac = (sig >> shift) as u32 & 0x7F_FFFF;
    let dropped = sig & ((1u64 << shift) - 1);
    let f = if dropped != 0 { frac | 1 } else { frac };
    sign | f
}

/// Flush an f32 denormal (raw bits) to a sign-preserving zero, per FPCR.FZ /
/// FZ-of-inputs. Used by the EBF==0 BF16 dot path. Inf/NaN pass through.
#[inline]
fn ftz_f32_bits(bits: u32) -> u32 {
    let exp = (bits >> 23) & 0xFF;
    let mant = bits & 0x007F_FFFF;
    if exp == 0 && mant != 0 {
        bits & 0x8000_0000
    } else {
        bits
    }
}

/// Move a finite, nonzero f64 one ULP toward +inf (`up`) or -inf.
#[inline]
fn nextafter_f64(x: f64, up: bool) -> f64 {
    let bits = x.to_bits();
    let neg = (bits >> 63) == 1;
    let nb = if up == !neg { bits + 1 } else { bits - 1 };
    f64::from_bits(nb)
}

/// One f32 multiply for the EBF==0 BF16 dot path: flush-to-zero inputs, exact
/// product in f64 (the widened bf16 operands have <=8-bit significands so the
/// product is exact), round-to-odd-INF to f32, flush-to-zero result. Mirrors
/// qemu float32_mul under the is_ebf(EBF=0) float_status.
fn bf_f32_mul(a: u32, b: u32) -> u32 {
    let af = f32::from_bits(ftz_f32_bits(a)) as f64;
    let bf = f32::from_bits(ftz_f32_bits(b)) as f64;
    ftz_f32_bits(round_odd_inf_f64_to_f32(af * bf))
}

/// One f32 add for the EBF==0 BF16 dot path: flush-to-zero inputs, EXACT sum via
/// 2Sum, round-to-odd at f64, then round-to-odd-INF to f32, flush-to-zero
/// result. The 2Sum + f64 round-to-odd captures the sticky bit even when the
/// operands differ by more than f64 precision (where a plain f64 add would lose
/// it), so the final f32 round-to-odd is exact. Mirrors qemu float32_add.
fn bf_f32_add(a: u32, b: u32) -> u32 {
    let af = f32::from_bits(ftz_f32_bits(a)) as f64;
    let bf = f32::from_bits(ftz_f32_bits(b)) as f64;
    let hi = af + bf;
    if hi.is_nan() {
        return 0x7FC0_0000;
    }
    if hi.is_infinite() {
        return round_odd_inf_f64_to_f32(hi);
    }
    // Knuth 2Sum: lo is the exact rounding error, hi + lo == af + bf exactly.
    let bb = hi - af;
    let lo = (af - (hi - bb)) + (bf - bb);
    let v64 = if lo == 0.0 {
        hi
    } else if hi.to_bits() & 1 == 1 {
        hi // hi already has an odd mantissa LSB
    } else {
        nextafter_f64(hi, lo > 0.0)
    };
    ftz_f32_bits(round_odd_inf_f64_to_f32(v64))
}

/// BFloat16 2-way dot product accumulate (FPCR.EBF==0), the qemu-user default.
/// Mirrors qemu bfdotadd: two products and two adds, each an f32 op under the
/// round-to-odd-INF / flush-to-zero / default-NaN float_status. `e1`/`e2` are
/// the 32-bit source slots (two bf16 each); `sum_bits` is the f32 accumulator.
fn bfdotadd_ebf0(sum_bits: u32, e1: u32, e2: u32) -> u32 {
    let t1 = bf_f32_mul(e1 << 16, e2 << 16);
    let t2 = bf_f32_mul(e1 & 0xFFFF_0000, e2 & 0xFFFF_0000);
    let t = bf_f32_add(t1, t2);
    bf_f32_add(sum_bits, t)
}

/// SVE2 FLOGB: floor(log2(|x|)) of an `esize`-byte IEEE float as a signed
/// integer of the same width. Finite non-zero values yield their unbiased
/// base-2 exponent (normal: biased_exp - bias; subnormal: normalized away
/// from the implicit-bit boundary). Infinity yields the most-positive integer
/// `2^(N-1)-1` (log2|inf| = +inf); zero and NaN yield the most-negative
/// integer `-(2^(N-1))`. The special-case results are verified vs qemu.
fn sve_flogb(esize: usize, bits: u64) -> i64 {
    let (expbits, fracbits): (u32, u32) = match esize {
        2 => (5, 10),
        4 => (8, 23),
        _ => (11, 52),
    };
    let bias = (1i64 << (expbits - 1)) - 1;
    let exp_mask = (1u64 << expbits) - 1;
    let exp = (bits >> fracbits) & exp_mask;
    let mant = bits & ((1u64 << fracbits) - 1);
    let int_bits = (esize as u32) * 8;
    let most_neg = -(1i64 << (int_bits - 1));
    let most_pos = (1i64 << (int_bits - 1)) - 1;
    if exp == exp_mask {
        // mant==0 is +/-infinity (most-positive); otherwise NaN (most-negative).
        return if mant == 0 { most_pos } else { most_neg };
    }
    if exp == 0 {
        if mant == 0 {
            return most_neg; // zero: invalid
        }
        // Subnormal: value = mant * 2^(Emin - fracbits), Emin = 1 - bias. The
        // unbiased exponent is Emin shifted down by the leading-zero count of
        // the fraction (relative to the implicit-bit position).
        let emin = 1 - bias;
        let msb = 63 - mant.leading_zeros() as i64; // floor(log2(mant))
        return emin - fracbits as i64 + msb;
    }
    exp as i64 - bias // normal: 1 <= significand < 2, so floor(log2) == exponent
}

/// One f32 lane of SVE BFDOT (FPCR.EBF==0, the qemu-user default): a 2-way bf16
/// dot product accumulated into the f32 lane. Delegates to the canonical
/// bfdotadd_ebf0 (round-to-odd-INF / flush-to-zero / default-NaN).
fn sve_bfdot_lane(acc_bits: u32, n: u32, m: u32) -> u32 {
    bfdotadd_ebf0(acc_bits, n, m)
}

/// FEXPA coefficient tables (ARM pseudocode): the low bits of Zn index a
/// significand, the next bits supply the result exponent.
const FEXPA_H: [u16; 32] = [
    0x0000, 0x0016, 0x002d, 0x0045, 0x005d, 0x0075, 0x008e, 0x00a8, 0x00c2, 0x00dc, 0x00f8, 0x0114,
    0x0130, 0x014d, 0x016b, 0x0189, 0x01a8, 0x01c8, 0x01e8, 0x0209, 0x022b, 0x024e, 0x0271, 0x0295,
    0x02ba, 0x02e0, 0x0306, 0x032e, 0x0356, 0x037f, 0x03a9, 0x03d4,
];
const FEXPA_S: [u32; 64] = [
    0x000000, 0x0164d2, 0x02cd87, 0x043a29, 0x05aac3, 0x071f62, 0x08980f, 0x0a14d5, 0x0b95c2,
    0x0d1adf, 0x0ea43a, 0x1031dc, 0x11c3d3, 0x135a2b, 0x14f4f0, 0x16942d, 0x1837f0, 0x19e046,
    0x1b8d3a, 0x1d3eda, 0x1ef532, 0x20b051, 0x227043, 0x243516, 0x25fed7, 0x27cd94, 0x29a15b,
    0x2b7a3a, 0x2d583f, 0x2f3b79, 0x3123f6, 0x3311c4, 0x3504f3, 0x36fd92, 0x38fbaf, 0x3aff5b,
    0x3d08a4, 0x3f179a, 0x412c4d, 0x4346cd, 0x45672a, 0x478d75, 0x49b9be, 0x4bec15, 0x4e248c,
    0x506334, 0x52a81e, 0x54f35b, 0x5744fd, 0x599d16, 0x5bfbb8, 0x5e60f5, 0x60ccdf, 0x633f89,
    0x65b907, 0x68396a, 0x6ac0c7, 0x6d4f30, 0x6fe4ba, 0x728177, 0x75257d, 0x77d0df, 0x7a83b3,
    0x7d3e0c,
];
const FEXPA_D: [u64; 64] = [
    0x0000000000000,
    0x02C9A3E778061,
    0x059B0D3158574,
    0x0874518759BC8,
    0x0B5586CF9890F,
    0x0E3EC32D3D1A2,
    0x11301D0125B51,
    0x1429AAEA92DE0,
    0x172B83C7D517B,
    0x1A35BEB6FCB75,
    0x1D4873168B9AA,
    0x2063B88628CD6,
    0x2387A6E756238,
    0x26B4565E27CDD,
    0x29E9DF51FDEE1,
    0x2D285A6E4030B,
    0x306FE0A31B715,
    0x33C08B26416FF,
    0x371A7373AA9CB,
    0x3A7DB34E59FF7,
    0x3DEA64C123422,
    0x4160A21F72E2A,
    0x44E086061892D,
    0x486A2B5C13CD0,
    0x4BFDAD5362A27,
    0x4F9B2769D2CA7,
    0x5342B569D4F82,
    0x56F4736B527DA,
    0x5AB07DD485429,
    0x5E76F15AD2148,
    0x6247EB03A5585,
    0x6623882552225,
    0x6A09E667F3BCD,
    0x6DFB23C651A2F,
    0x71F75E8EC5F74,
    0x75FEB564267C9,
    0x7A11473EB0187,
    0x7E2F336CF4E62,
    0x82589994CCE13,
    0x868D99B4492ED,
    0x8ACE5422AA0DB,
    0x8F1AE99157736,
    0x93737B0CDC5E5,
    0x97D829FDE4E50,
    0x9C49182A3F090,
    0xA0C667B5DE565,
    0xA5503B23E255D,
    0xA9E6B5579FDBF,
    0xAE89F995AD3AD,
    0xB33A2B84F15FB,
    0xB7F76F2FB5E47,
    0xBCC1E904BC1D2,
    0xC199BDD85529C,
    0xC67F12E57D14B,
    0xCB720DCEF9069,
    0xD072D4A07897C,
    0xD5818DCFBA487,
    0xDA9E603DB3285,
    0xDFC97337B9B5F,
    0xE502EE78B3FF6,
    0xEA4AFA2A490DA,
    0xEFA1BEE615A27,
    0xF50765B6E4540,
    0xFA7C1819E90D8,
];

/// SVE FEXPA (exponential accelerator): build a float from a table significand
/// indexed by the low bits of Zn and an exponent from the next bits.
fn sve_fexpa(esize: usize, nn: u64) -> u64 {
    match esize {
        2 => FEXPA_H[(nn & 0x1F) as usize] as u64 | (((nn >> 5) & 0x1F) << 10),
        4 => FEXPA_S[(nn & 0x3F) as usize] as u64 | (((nn >> 6) & 0xFF) << 23),
        _ => FEXPA_D[(nn & 0x3F) as usize] | (((nn >> 6) & 0x7FF) << 52),
    }
}

/// scalbn for f32: x * 2^n, correctly rounded (musl port, avoids double rounding).
fn scalbn_f32(x: f32, mut n: i32) -> f32 {
    let mut y = x;
    if n > 127 {
        y *= f32::from_bits(0x7F00_0000); // 2^127
        n -= 127;
        if n > 127 {
            y *= f32::from_bits(0x7F00_0000);
            n -= 127;
            if n > 127 {
                n = 127;
            }
        }
    } else if n < -126 {
        y *= f32::from_bits(0x0080_0000) * f32::from_bits(0x4B80_0000); // 2^-126 * 2^24
        n += 126 - 24;
        if n < -126 {
            y *= f32::from_bits(0x0080_0000) * f32::from_bits(0x4B80_0000);
            n += 126 - 24;
            if n < -126 {
                n = -126;
            }
        }
    }
    y * f32::from_bits(((0x7F + n) as u32) << 23)
}

/// scalbn for f64 (musl port).
fn scalbn_f64(x: f64, mut n: i64) -> f64 {
    let mut y = x;
    if n > 1023 {
        y *= f64::from_bits(0x7FE0_0000_0000_0000); // 2^1023
        n -= 1023;
        if n > 1023 {
            y *= f64::from_bits(0x7FE0_0000_0000_0000);
            n -= 1023;
            if n > 1023 {
                n = 1023;
            }
        }
    } else if n < -1022 {
        y *= f64::from_bits(0x0010_0000_0000_0000) * f64::from_bits(0x4340_0000_0000_0000); // 2^-1022*2^53
        n += 1022 - 53;
        if n < -1022 {
            y *= f64::from_bits(0x0010_0000_0000_0000) * f64::from_bits(0x4340_0000_0000_0000);
            n += 1022 - 53;
            if n < -1022 {
                n = -1022;
            }
        }
    }
    y * f64::from_bits(((0x3FF + n) as u64) << 52)
}

/// SVE FSCALE: multiply `x` by 2^(signed Zm element). fp16 via an exact f64
/// intermediate; f32/f64 via the correctly-rounded scalbn.
fn sve_fscale(esize: usize, x: u64, n: i64) -> u64 {
    match esize {
        2 => fp16_round(fp16_to_f64(x as u16) * exp2_f64(n.clamp(-1023, 1023) as i32)) as u64,
        4 => scalbn_f32(
            f32::from_bits(x as u32),
            n.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        )
        .to_bits() as u64,
        _ => scalbn_f64(f64::from_bits(x), n).to_bits(),
    }
}

/// 2^n as an f64, exact for |n| <= 1023.
fn exp2_f64(n: i32) -> f64 {
    f64::from_bits(((0x3FF + n) as u64) << 52)
}

/// Widen an `esize`-byte IEEE float to f64 (exact) for SVE compares.
fn sve_fp_to_f64(esize: usize, x: u64) -> f64 {
    match esize {
        2 => fp16_to_f64(x as u16),
        4 => f32::from_bits(x as u32) as f64,
        _ => f64::from_bits(x),
    }
}

/// SVE FP compare (register) condition, keyed on (bits[15:13], bit4). Rust's
/// native comparisons already give the IEEE unordered (NaN) behaviour.
fn sve_fp_compare(esize: usize, cc: (u32, u32), a: u64, b: u64) -> bool {
    let (av, bv) = (sve_fp_to_f64(esize, a), sve_fp_to_f64(esize, b));
    match cc {
        (0b010, 0) => av >= bv,                   // FCMGE
        (0b010, 1) => av > bv,                    // FCMGT
        (0b011, 0) => av == bv,                   // FCMEQ
        (0b011, 1) => av != bv,                   // FCMNE
        (0b110, 0) => av.is_nan() || bv.is_nan(), // FCMUO
        (0b110, 1) => av.abs() >= bv.abs(),       // FACGE
        (0b111, 1) => av.abs() > bv.abs(),        // FACGT
        _ => false,
    }
}

/// SVE FP compare with zero, keyed on (bits[17:16], bit4).
fn sve_fp_compare_zero(esize: usize, sub: u32, bit4: u32, a: u64) -> bool {
    let av = sve_fp_to_f64(esize, a);
    match (sub, bit4) {
        (0b00, 0) => av >= 0.0, // FCMGE
        (0b00, 1) => av > 0.0,  // FCMGT
        (0b01, 0) => av < 0.0,  // FCMLT
        (0b01, 1) => av <= 0.0, // FCMLE
        (0b10, 0) => av == 0.0, // FCMEQ
        _ => av != 0.0,         // FCMNE
    }
}

/// SVE FRECPS reciprocal step: fused (2.0 - x*y), with inf*0 -> 2.0. Matches
/// qemu recpsf (FPCR.AH=0, FZ=0).
fn sve_recps(esize: usize, x: u64, y: u64) -> u64 {
    match esize {
        2 => fp16_recps(x as u16, y as u16) as u64,
        4 => {
            // FPRecipStepFused negates op1 first, so the propagated NaN sign
            // flips relative to the original op1.
            if let Some(n) = fp32_nan2((x as u32) ^ 0x8000_0000, y as u32) {
                return n as u64;
            }
            let (a, b) = (f32::from_bits(x as u32), f32::from_bits(y as u32));
            let r = if (a.is_infinite() && b == 0.0) || (b.is_infinite() && a == 0.0) {
                2.0
            } else {
                (-a).mul_add(b, 2.0)
            };
            (if r.is_nan() { 0x7FC0_0000 } else { r.to_bits() }) as u64
        }
        _ => {
            if let Some(n) = fp64_nan2(x ^ 0x8000_0000_0000_0000, y) {
                return n;
            }
            let (a, b) = (f64::from_bits(x), f64::from_bits(y));
            let r = if (a.is_infinite() && b == 0.0) || (b.is_infinite() && a == 0.0) {
                2.0
            } else {
                (-a).mul_add(b, 2.0)
            };
            if r.is_nan() {
                0x7FF8_0000_0000_0000
            } else {
                r.to_bits()
            }
        }
    }
}

/// SVE FRSQRTS reciprocal-sqrt step: fused (3.0 - x*y)/2, with inf*0 -> 1.5.
fn sve_rsqrts(esize: usize, x: u64, y: u64) -> u64 {
    match esize {
        2 => fp16_rsqrts(x as u16, y as u16) as u64,
        4 => {
            if let Some(n) = fp32_nan2((x as u32) ^ 0x8000_0000, y as u32) {
                return n as u64;
            }
            let (a, b) = (f32::from_bits(x as u32), f32::from_bits(y as u32));
            let r = if (a.is_infinite() && b == 0.0) || (b.is_infinite() && a == 0.0) {
                1.5
            } else {
                (-a).mul_add(b, 3.0) * 0.5
            };
            (if r.is_nan() { 0x7FC0_0000 } else { r.to_bits() }) as u64
        }
        _ => {
            if let Some(n) = fp64_nan2(x ^ 0x8000_0000_0000_0000, y) {
                return n;
            }
            let (a, b) = (f64::from_bits(x), f64::from_bits(y));
            let r = if (a.is_infinite() && b == 0.0) || (b.is_infinite() && a == 0.0) {
                1.5
            } else {
                (-a).mul_add(b, 3.0) * 0.5
            };
            if r.is_nan() {
                0x7FF8_0000_0000_0000
            } else {
                r.to_bits()
            }
        }
    }
}

/// SVE FTSMUL: square `x` and set the result sign to `sgn` (bit0 of Zm),
/// unless the squared value is NaN (then the sign is left as produced).
fn sve_ftsmul(esize: usize, x: u64, sgn: u64) -> u64 {
    match esize {
        2 => {
            let s = fp16_mul(x as u16, x as u16);
            if (s & 0x7C00) == 0x7C00 && (s & 0x03FF) != 0 {
                s as u64 // NaN
            } else {
                ((s & 0x7FFF) | ((sgn as u16) << 15)) as u64
            }
        }
        4 => {
            let r = f32::from_bits(x as u32) * f32::from_bits(x as u32);
            if r.is_nan() {
                r.to_bits() as u64
            } else {
                ((r.to_bits() & 0x7FFF_FFFF) | ((sgn as u32) << 31)) as u64
            }
        }
        _ => {
            let r = f64::from_bits(x) * f64::from_bits(x);
            if r.is_nan() {
                r.to_bits()
            } else {
                (r.to_bits() & 0x7FFF_FFFF_FFFF_FFFF) | (sgn << 63)
            }
        }
    }
}

/// SVE FTMAD coefficient tables (ARM ASL): index = imm + (8 if Zm<0). The
/// fused multiply-add is Zdn*|Zm| + coeff[index] (FPCR.AH=0 default).
const FTMAD_COEFF_H: [u16; 16] = [
    0x3c00, 0xb155, 0x2030, 0, 0, 0, 0, 0, 0x3c00, 0xb800, 0x293a, 0, 0, 0, 0, 0,
];
const FTMAD_COEFF_S: [u32; 16] = [
    0x3f80_0000,
    0xbe2a_aaab,
    0x3c08_8886,
    0xb950_08b9,
    0x3636_9d6d,
    0,
    0,
    0,
    0x3f80_0000,
    0xbf00_0000,
    0x3d2a_aaa6,
    0xbab6_0705,
    0x37cd_37cc,
    0,
    0,
    0,
];
const FTMAD_COEFF_D: [u64; 16] = [
    0x3ff0_0000_0000_0000,
    0xbfc5_5555_5555_5543,
    0x3f81_1111_1110_f30c,
    0xbf2a_01a0_19b9_2fc6,
    0x3ec7_1de3_51f3_d22b,
    0xbe5a_e5e2_b60f_7b91,
    0x3de5_d840_8868_552f,
    0,
    0x3ff0_0000_0000_0000,
    0xbfe0_0000_0000_0000,
    0x3fa5_5555_5555_5536,
    0xbf56_c16c_16c1_3a0b,
    0x3efa_01a0_19b1_e8d8,
    0xbe92_7e4f_7282_f468,
    0x3e21_ee96_d264_1b13,
    0xbda8_f763_80fb_b401,
];

/// SVE FTMAD: Zdn = fused(Zdn, |Zm|, coeff[imm + 8*(Zm<0)]). The product is
/// against the absolute value of Zm; a negative Zm selects the upper coefficient
/// block (FPCR.AH=0 default — no product negation).
fn sve_ftmad(esize: usize, nn: u64, mm: u64, imm: usize) -> u64 {
    match esize {
        2 => {
            let neg = mm & 0x8000 != 0;
            let m = if neg { mm & 0x7FFF } else { mm } as u16;
            let coeff = FTMAD_COEFF_H[imm + if neg { 8 } else { 0 }];
            fp16_round(fp16_to_f64(nn as u16) * fp16_to_f64(m) + fp16_to_f64(coeff)) as u64
        }
        4 => {
            let neg = mm & 0x8000_0000 != 0;
            let m = if neg { mm & 0x7FFF_FFFF } else { mm };
            let coeff = f32::from_bits(FTMAD_COEFF_S[imm + if neg { 8 } else { 0 }]);
            f32::from_bits(nn as u32)
                .mul_add(f32::from_bits(m as u32), coeff)
                .to_bits() as u64
        }
        _ => {
            let neg = mm & 0x8000_0000_0000_0000 != 0;
            let m = if neg { mm & 0x7FFF_FFFF_FFFF_FFFF } else { mm };
            let coeff = f64::from_bits(FTMAD_COEFF_D[imm + if neg { 8 } else { 0 }]);
            f64::from_bits(nn)
                .mul_add(f64::from_bits(m), coeff)
                .to_bits()
        }
    }
}

// ---- SHA-1 / SHA-256 primitives (FIPS-180, per ARM ASL) ----

/// Extract 32-bit element `e` from a 128-bit vector.
#[inline]
fn sha_elem(v: u128, e: u32) -> u32 {
    (v >> (e * 32)) as u32
}

/// Insert 32-bit element `e` into a 128-bit vector.
#[inline]
fn sha_set_elem(v: &mut u128, e: u32, x: u32) {
    let sh = e * 32;
    *v = (*v & !(0xFFFF_FFFFu128 << sh)) | ((x as u128) << sh);
}

/// SHAchoose: ((y EOR z) AND x) EOR z
#[inline]
fn sha_choose(x: u32, y: u32, z: u32) -> u32 {
    ((y ^ z) & x) ^ z
}

/// SHAmajority: (x AND y) OR ((x OR y) AND z)
#[inline]
fn sha_majority(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | ((x | y) & z)
}

/// SHAparity: x EOR y EOR z
#[inline]
fn sha_parity(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

/// SHA256 compression hash update (4 rounds). `part1` selects which 128-bit
/// half (X for SHA256H, Y for SHA256H2) is returned, per the ASL SHA256hash.
fn sha256_hash(x_in: u128, y_in: u128, w: u128, part1: bool) -> u128 {
    let mut x = x_in;
    let mut y = y_in;
    for e in 0..4 {
        let chs = sha_choose(sha_elem(y, 0), sha_elem(y, 1), sha_elem(y, 2));
        let maj = sha_majority(sha_elem(x, 0), sha_elem(x, 1), sha_elem(x, 2));
        // SIGMA1(Y<31:0>) = ROR(y0,6) ^ ROR(y0,11) ^ ROR(y0,25)
        let y0 = sha_elem(y, 0);
        let sigma1 = y0.rotate_right(6) ^ y0.rotate_right(11) ^ y0.rotate_right(25);
        let t = sha_elem(y, 3)
            .wrapping_add(sigma1)
            .wrapping_add(chs)
            .wrapping_add(sha_elem(w, e));
        // X<127:96> = t + X<127:96>
        let x3 = t.wrapping_add(sha_elem(x, 3));
        sha_set_elem(&mut x, 3, x3);
        // SIGMA0(X<31:0>) = ROR(x0,2) ^ ROR(x0,13) ^ ROR(x0,22)
        let x0 = sha_elem(x, 0);
        let sigma0 = x0.rotate_right(2) ^ x0.rotate_right(13) ^ x0.rotate_right(22);
        // Y<127:96> = t + SIGMA0(X<31:0>) + maj
        sha_set_elem(&mut y, 3, t.wrapping_add(sigma0).wrapping_add(maj));
        // <Y, X> = ROL(Y : X, 32) over the 256-bit concatenation (Y high, X low).
        let carry = (y >> 96) as u32; // Y<127:96>
        let new_y = (y << 32) | (x >> 96);
        let new_x = (x << 32) | (carry as u128);
        x = new_x;
        y = new_y;
    }
    if part1 { x } else { y }
}

/// SHA1 hash update (4 rounds) for SHA1C/SHA1P/SHA1M. `f` is the round
/// function (choose / parity / majority). Returns the new X (V[d]).
fn sha1_hash(x_in: u128, y_in: u32, w: u128, f: fn(u32, u32, u32) -> u32) -> u128 {
    let mut x = x_in;
    let mut y = y_in;
    for e in 0..4 {
        let t = f(sha_elem(x, 1), sha_elem(x, 2), sha_elem(x, 3));
        y = y
            .wrapping_add(sha_elem(x, 0).rotate_left(5))
            .wrapping_add(t)
            .wrapping_add(sha_elem(w, e));
        // X<63:32> = ROL(X<63:32>, 30)
        let x1 = sha_elem(x, 1).rotate_left(30);
        sha_set_elem(&mut x, 1, x1);
        // <Y, X> = ROL(Y : X, 32): Y is 32 bits, X is 128 bits (160-bit rotate).
        let new_y = sha_elem(x, 3); // X<127:96>
        let new_x = ((x & ((1u128 << 96) - 1)) << 32) | (y as u128); // X<95:0> : Y
        y = new_y;
        x = new_x;
    }
    x
}

// ---- Software half-precision (IEEE binary16) for AdvSIMD FP16 ----
//
// All operations follow the Arm ASL with the default FPCR (round-to-nearest
// even, no flush-to-zero, DN=0 so input NaNs propagate quieted). Arithmetic is
// evaluated in f64 — exact for binary16 add/sub/mul and the fused step/estimate
// forms — then rounded once to binary16 with `fp16_round`.

#[inline]
/// ARM FPConvertNaN (FPCR.DN=0): convert a NaN between FP precisions, preserving
/// the sign, quieting, and aligning the payload to the MSB of the destination
/// mantissa. `prec` is the byte width: 2=f16(10-bit mantissa), 4=f32(23), 8=f64(52).
fn fp_convert_nan(src: u64, src_prec: usize, dst_prec: usize) -> u64 {
    let fields = |p: usize| -> (u32, u32) {
        match p {
            2 => (10, 5),
            4 => (23, 8),
            _ => (52, 11),
        }
    };
    let (sm, se) = fields(src_prec);
    let (dm, de) = fields(dst_prec);
    let sign = (src >> (sm + se)) & 1;
    let payload = src & ((1u64 << sm) - 1);
    let aligned = if dm >= sm {
        payload << (dm - sm)
    } else {
        payload >> (sm - dm)
    };
    let quiet = 1u64 << (dm - 1);
    let exp_all = (1u64 << de) - 1;
    (sign << (dm + de)) | (exp_all << dm) | quiet | aligned
}

fn fp16_to_f64(h: u16) -> f64 {
    AArch64Cpu::fp16_to_f32(h) as f64
}

#[inline]
fn fp16_is_nan(h: u16) -> bool {
    (h & 0x7C00) == 0x7C00 && (h & 0x03FF) != 0
}

#[inline]
fn fp16_is_inf(h: u16) -> bool {
    (h & 0x7FFF) == 0x7C00
}

#[inline]
fn fp16_is_zero(h: u16) -> bool {
    (h & 0x7FFF) == 0
}

/// FPProcessNaNs over two operands (DN=0): propagate a NaN if present,
/// quieting signaling NaNs and giving them priority. Returns None if neither
/// operand is a NaN.
fn fp16_nan2(a: u16, b: u16) -> Option<u16> {
    let a_nan = fp16_is_nan(a);
    let b_nan = fp16_is_nan(b);
    if a_nan && (a & 0x0200) == 0 {
        Some(a | 0x0200)
    } else if b_nan && (b & 0x0200) == 0 {
        Some(b | 0x0200)
    } else if a_nan {
        Some(a)
    } else if b_nan {
        Some(b)
    } else {
        None
    }
}

/// FPProcessNaNs over three operands (for the fused multiply-add forms).
fn fp16_nan3(a: u16, b: u16, c: u16) -> Option<u16> {
    for &x in &[a, b, c] {
        if fp16_is_nan(x) && (x & 0x0200) == 0 {
            return Some(x | 0x0200);
        }
    }
    for &x in &[a, b, c] {
        if fp16_is_nan(x) {
            return Some(x);
        }
    }
    None
}

/// Round `v / 2^shift` to nearest, ties to even.
fn round_shift_u64(v: u64, shift: u32) -> u64 {
    if shift == 0 {
        return v;
    }
    if shift >= 64 {
        return 0;
    }
    let result = v >> shift;
    let rem = v & ((1u64 << shift) - 1);
    let half = 1u64 << (shift - 1);
    if rem > half || (rem == half && (result & 1) == 1) {
        result + 1
    } else {
        result
    }
}

/// One element of an SVE FP -> integer conversion (FCVTZS/FCVTZU): round the
/// `fp_sz`-byte float toward zero into an `int_sz`-byte integer, saturating
/// out-of-range magnitudes and mapping NaN to 0. Rust's float-to-int `as`
/// already truncates toward zero, saturates and maps NaN to 0, matching ARM.
fn sve_fcvtz(fp_sz: usize, int_sz: usize, signed: bool, x: u64) -> u64 {
    let f: f64 = match fp_sz {
        2 => fp16_to_f64(x as u16),
        4 => f32::from_bits(x as u32) as f64,
        _ => f64::from_bits(x),
    };
    // A signed result is sign-extended to the (possibly wider) container; an
    // unsigned result is zero-extended. The caller's write_elem masks back down
    // to the container width, so extending to 64 bits here is always correct.
    match (int_sz, signed) {
        (2, true) => (f as i16) as i64 as u64,
        (2, false) => (f as u16) as u64,
        (4, true) => (f as i32) as i64 as u64,
        (4, false) => (f as u32) as u64,
        (8, true) => (f as i64) as u64,
        _ => f as u64,
    }
}

/// One element of an SVE integer -> FP conversion (SCVTF/UCVTF): convert the
/// `int_sz`-byte integer (signed or unsigned) to an `fp_sz`-byte float with
/// round-to-nearest-even. The integer is cast directly to the destination type
/// to avoid a double rounding through an intermediate wider float.
fn sve_cvtf(int_sz: usize, fp_sz: usize, signed: bool, x: u64) -> u64 {
    match fp_sz {
        4 => {
            let f: f32 = if signed {
                match int_sz {
                    2 => (x as u16 as i16) as f32,
                    4 => (x as u32 as i32) as f32,
                    _ => (x as i64) as f32,
                }
            } else {
                match int_sz {
                    2 => (x as u16) as f32,
                    4 => (x as u32) as f32,
                    _ => x as f32,
                }
            };
            f.to_bits() as u64
        }
        8 => {
            let f: f64 = if signed {
                match int_sz {
                    2 => (x as u16 as i16) as f64,
                    4 => (x as u32 as i32) as f64,
                    _ => (x as i64) as f64,
                }
            } else {
                match int_sz {
                    2 => (x as u16) as f64,
                    4 => (x as u32) as f64,
                    _ => x as f64,
                }
            };
            f.to_bits()
        }
        _ => {
            // fp16 destination: an integer large enough to round when widened to
            // f64 (|x| >= 2^53) is far beyond fp16's range and saturates to Inf,
            // so routing through an exact f64 then fp16_round is single-rounded.
            let f: f64 = if signed {
                match int_sz {
                    2 => (x as u16 as i16) as f64,
                    4 => (x as u32 as i32) as f64,
                    _ => (x as i64) as f64,
                }
            } else {
                match int_sz {
                    2 => (x as u16) as f64,
                    4 => (x as u32) as f64,
                    _ => x as f64,
                }
            };
            fp16_round(f) as u64
        }
    }
}

/// Round an f64 to IEEE binary16 (round-to-nearest even, no flush-to-zero).
/// A NaN input maps to the default binary16 NaN; callers that must preserve an
/// operand NaN handle propagation before calling this.
fn fp16_round(x: f64) -> u16 {
    if x.is_nan() {
        return 0x7E00;
    }
    let sign: u16 = if x.is_sign_negative() { 0x8000 } else { 0 };
    let a = x.abs();
    if a == 0.0 {
        return sign;
    }
    if a.is_infinite() || a >= 65520.0 {
        // 65520 is the round-to-nearest overflow threshold (halfway to 2^16).
        return sign | 0x7C00;
    }
    let bits = a.to_bits();
    let exp = ((bits >> 52) & 0x7FF) as i32 - 1023; // `a` is a normal f64 here
    let mant52 = bits & 0x000F_FFFF_FFFF_FFFF;
    if exp < -14 {
        // Subnormal binary16 (or rounding up into the smallest normal).
        let sig = (1u64 << 52) | mant52; // 1.mant52 scaled by 2^52
        let shift = (28 - exp) as u32; // value * 2^24 == sig >> (28 - exp)
        let m = round_shift_u64(sig, shift);
        if m >= 1024 {
            return sign | (1 << 10) | ((m as u16) & 0x3FF);
        }
        return sign | (m as u16 & 0x3FF);
    }
    let e16 = (exp + 15) as u16; // biased binary16 exponent in [1, 30]
    let m = round_shift_u64(mant52, 42); // round the 52-bit fraction to 10 bits
    if m >= 1024 {
        let e2 = e16 + 1;
        if e2 >= 0x1F {
            return sign | 0x7C00;
        }
        return sign | (e2 << 10);
    }
    sign | (e16 << 10) | (m as u16 & 0x3FF)
}

fn fp16_add(a: u16, b: u16) -> u16 {
    if let Some(n) = fp16_nan2(a, b) {
        return n;
    }
    fp16_round(fp16_to_f64(a) + fp16_to_f64(b))
}

fn fp16_sub(a: u16, b: u16) -> u16 {
    if let Some(n) = fp16_nan2(a, b) {
        return n;
    }
    fp16_round(fp16_to_f64(a) - fp16_to_f64(b))
}

fn fp16_mul(a: u16, b: u16) -> u16 {
    if let Some(n) = fp16_nan2(a, b) {
        return n;
    }
    fp16_round(fp16_to_f64(a) * fp16_to_f64(b))
}

fn fp16_div(a: u16, b: u16) -> u16 {
    if let Some(n) = fp16_nan2(a, b) {
        return n;
    }
    fp16_round(fp16_to_f64(a) / fp16_to_f64(b))
}

fn fp16_mulx(a: u16, b: u16) -> u16 {
    if let Some(n) = fp16_nan2(a, b) {
        return n;
    }
    if (fp16_is_zero(a) && fp16_is_inf(b)) || (fp16_is_inf(a) && fp16_is_zero(b)) {
        let sign = ((a >> 15) ^ (b >> 15)) & 1;
        return (sign << 15) | 0x4000; // ±2.0
    }
    fp16_round(fp16_to_f64(a) * fp16_to_f64(b))
}

fn fp16_max_min(a: u16, b: u16, is_min: bool) -> u16 {
    if let Some(n) = fp16_nan2(a, b) {
        return n;
    }
    let x = fp16_to_f64(a);
    let y = fp16_to_f64(b);
    if x == 0.0 && y == 0.0 {
        // Both zero: FMAX prefers +0, FMIN prefers -0.
        let s = if is_min {
            ((a | b) >> 15) & 1
        } else {
            ((a & b) >> 15) & 1
        };
        return s << 15;
    }
    let pick_a = if is_min { x < y } else { x > y };
    let pick_b = if is_min { y < x } else { y > x };
    if pick_a {
        a
    } else if pick_b {
        b
    } else {
        a
    }
}

fn fp16_max(a: u16, b: u16) -> u16 {
    fp16_max_min(a, b, false)
}

fn fp16_min(a: u16, b: u16) -> u16 {
    fp16_max_min(a, b, true)
}

fn fp16_maxnum_minnum(a: u16, b: u16, is_min: bool) -> u16 {
    // ARM FPMaxNum/FPMinNum (mirrors the verified fp_three_same_f32 path): a
    // signalling NaN propagates quieted; otherwise a lone quiet NaN loses to the
    // numeric operand, and two quiet NaNs return the first.
    let snan = |v: u16| (v & 0x7C00) == 0x7C00 && (v & 0x3FF) != 0 && (v & 0x0200) == 0;
    if snan(a) {
        return a | 0x0200;
    }
    if snan(b) {
        return b | 0x0200;
    }
    let aq = fp16_is_nan(a);
    let bq = fp16_is_nan(b);
    if aq && bq {
        return a;
    }
    if aq {
        return b;
    }
    if bq {
        return a;
    }
    fp16_max_min(a, b, is_min)
}

/// Dispatch an `FpKind` binary op to the verified binary16 helpers (for SVE
/// predicated FP). Only the arithmetic/min/max/abd kinds are used here.
fn sve_fp16_binop(kind: FpKind, x: u16, y: u16) -> u16 {
    use FpKind::*;
    match kind {
        Add => fp16_add(x, y),
        Sub => fp16_sub(x, y),
        Mul => fp16_mul(x, y),
        Div => fp16_div(x, y),
        Max => fp16_max(x, y),
        Min => fp16_min(x, y),
        MaxNm => fp16_maxnm(x, y),
        MinNm => fp16_minnm(x, y),
        Abd => fp16_abd(x, y),
        _ => x,
    }
}

fn fp16_maxnm(a: u16, b: u16) -> u16 {
    fp16_maxnum_minnum(a, b, false)
}

fn fp16_minnm(a: u16, b: u16) -> u16 {
    fp16_maxnum_minnum(a, b, true)
}

fn fp16_abd(a: u16, b: u16) -> u16 {
    fp16_sub(a, b) & 0x7FFF
}

fn fp16_recps(a: u16, b: u16) -> u16 {
    // FPRecipStepFused negates op1 first, flipping the propagated NaN sign.
    if let Some(n) = fp16_nan2(a ^ 0x8000, b) {
        return n;
    }
    if (fp16_is_zero(a) && fp16_is_inf(b)) || (fp16_is_inf(a) && fp16_is_zero(b)) {
        return 0x4000; // 2.0
    }
    fp16_round(2.0 - fp16_to_f64(a) * fp16_to_f64(b))
}

fn fp16_rsqrts(a: u16, b: u16) -> u16 {
    if let Some(n) = fp16_nan2(a ^ 0x8000, b) {
        return n;
    }
    if (fp16_is_zero(a) && fp16_is_inf(b)) || (fp16_is_inf(a) && fp16_is_zero(b)) {
        return 0x3E00; // 1.5
    }
    fp16_round((3.0 - fp16_to_f64(a) * fp16_to_f64(b)) / 2.0)
}

fn fp16_mla(acc: u16, a: u16, b: u16) -> u16 {
    // ARM FPMulAdd processes NaNs in (addend, op1, op2) order.
    if let Some(n) = fp16_nan3(acc, a, b) {
        return n;
    }
    fp16_round(fp16_to_f64(acc) + fp16_to_f64(a) * fp16_to_f64(b))
}

fn fp16_mls(acc: u16, a: u16, b: u16) -> u16 {
    // FMLS = FPMulAdd(acc, FPNeg(a), b): the multiplicand is negated BEFORE NaN
    // processing, so a propagated NaN from `a` carries the flipped sign.
    let na = a ^ 0x8000;
    if let Some(n) = fp16_nan3(acc, na, b) {
        return n;
    }
    fp16_round(fp16_to_f64(acc) + fp16_to_f64(na) * fp16_to_f64(b))
}

/// FP16 comparisons returning an all-ones (true) / all-zeros (false) lane.
/// `kind`: 0=EQ, 1=GE, 2=GT, 3=ACGE (abs), 4=ACGT (abs).
fn fp16_cmp(a: u16, b: u16, kind: u8) -> u16 {
    if fp16_is_nan(a) || fp16_is_nan(b) {
        return 0; // unordered compares are false
    }
    let x = fp16_to_f64(a);
    let y = fp16_to_f64(b);
    let r = match kind {
        0 => x == y,
        1 => x >= y,
        2 => x > y,
        3 => x.abs() >= y.abs(),
        _ => x.abs() > y.abs(),
    };
    if r { 0xFFFF } else { 0 }
}

/// FP16 comparison against zero (two-reg-misc forms).
/// `kind`: 0=GT, 1=GE, 2=EQ, 3=LE, 4=LT.
fn fp16_cmp0(a: u16, kind: u8) -> u16 {
    if fp16_is_nan(a) {
        return 0;
    }
    let x = fp16_to_f64(a);
    let r = match kind {
        0 => x > 0.0,
        1 => x >= 0.0,
        2 => x == 0.0,
        3 => x <= 0.0,
        _ => x < 0.0,
    };
    if r { 0xFFFF } else { 0 }
}

/// FP16 square root (provably correctly rounded via f64: 53 >= 2*11+2).
fn fp16_sqrt(a: u16) -> u16 {
    if fp16_is_nan(a) {
        return a | 0x0200;
    }
    fp16_round(fp16_to_f64(a).sqrt())
}

/// FP16 round-to-integral. `mode`: 0=TIEEVEN, 1=NEGINF, 2=POSINF, 3=ZERO,
/// 4=TIEAWAY. The result is an integral binary16 value.
fn fp16_frint(a: u16, mode: u8) -> u16 {
    if fp16_is_nan(a) {
        return a | 0x0200;
    }
    let x = fp16_to_f64(a);
    if x == 0.0 || x.is_infinite() {
        return a; // ±0 and ±inf are returned unchanged
    }
    let r = match mode {
        0 => x.round_ties_even(),
        1 => x.floor(),
        2 => x.ceil(),
        3 => x.trunc(),
        _ => x.round(), // ties away from zero
    };
    // Preserve the sign of a zero result (e.g. round(-0.3) == -0.0).
    if r == 0.0 {
        return (a & 0x8000) | 0;
    }
    fp16_round(r)
}

/// FPRecipEstimate for binary16 (FPCR default: RNE, FZ16=0). Ported from the
/// Arm ASL using the shared `recip_estimate` 8-bit core.
fn fp16_recpe(op: u16) -> u16 {
    let sign = (op >> 15) as u64 & 1;
    let exp = ((op >> 10) & 0x1F) as i32;
    let frac = (op & 0x3FF) as u64;
    if exp == 0x1F {
        return if frac != 0 {
            op | 0x0200
        } else {
            (sign << 15) as u16
        };
    }
    if exp == 0 && frac == 0 {
        return ((sign << 15) as u16) | 0x7C00; // zero -> infinity
    }
    if exp == 0 && frac < 256 {
        // |value| < 2^-16: overflow to infinity (RNE).
        return ((sign << 15) as u16) | 0x7C00;
    }
    let mut fraction: u64 = frac << 42; // operand<9:0> : Zeros(42)
    let mut e = exp;
    if exp == 0 {
        if (fraction >> 51) & 1 == 0 {
            e = -1;
            fraction = (fraction & ((1u64 << 50) - 1)) << 2;
        } else {
            fraction = (fraction & ((1u64 << 51) - 1)) << 1;
        }
    }
    let scaled = 0x100u32 | ((fraction >> 44) & 0xFF) as u32;
    let mut result_exp = 29 - e;
    let estimate = (recip_estimate(scaled) & 0xFF) as u64;
    let mut frac2: u64 = estimate << 44; // estimate<7:0> : Zeros(44)
    if result_exp == 0 {
        frac2 = (1u64 << 51) | (frac2 >> 1);
    } else if result_exp == -1 {
        frac2 = (1u64 << 50) | (frac2 >> 2);
        result_exp = 0;
    }
    ((sign as u16) << 15) | (((result_exp as u16) & 0x1F) << 10) | ((frac2 >> 42) & 0x3FF) as u16
}

/// FPRSqrtEstimate for binary16. Ported from the Arm ASL.
fn fp16_rsqrte(op: u16) -> u16 {
    let sign = (op >> 15) as u64 & 1;
    let exp = ((op >> 10) & 0x1F) as i32;
    let frac = (op & 0x3FF) as u64;
    if exp == 0x1F && frac != 0 {
        return op | 0x0200; // NaN -> quiet
    }
    if exp == 0 && frac == 0 {
        return ((sign << 15) as u16) | 0x7C00; // zero -> +/-inf
    }
    if sign == 1 {
        return 0x7E00; // negative -> default NaN
    }
    if exp == 0x1F {
        return 0; // +inf -> +0
    }
    let mut fraction: u64 = frac << 42;
    let mut e = exp;
    if exp == 0 {
        while (fraction >> 51) & 1 == 0 {
            fraction = (fraction & ((1u64 << 51) - 1)) << 1;
            e -= 1;
        }
        fraction = (fraction & ((1u64 << 51) - 1)) << 1;
    }
    let scaled = if e & 1 == 0 {
        0x100u32 | ((fraction >> 44) & 0xFF) as u32 // '1':fraction<51:44>
    } else {
        0x080u32 | ((fraction >> 45) & 0x7F) as u32 // '01':fraction<51:45>
    };
    let result_exp = (44 - e).div_euclid(2);
    let estimate = (recip_sqrt_estimate(scaled) & 0xFF) as u16;
    (((result_exp as u16) & 0x1F) << 10) | (estimate << 2)
}

/// FPRecpX (reciprocal exponent) for binary16.
fn fp16_recpx(op: u16) -> u16 {
    if fp16_is_nan(op) {
        return op | 0x0200;
    }
    let sign = op & 0x8000;
    let exp = (op >> 10) & 0x1F;
    if exp == 0 {
        sign | (30 << 10) // max_exp = Ones(5) - 1
    } else {
        sign | ((!exp & 0x1F) << 10)
    }
}

/// Convert binary16 to a 16-bit integer lane with saturation.
/// `mode`: 0=TIEEVEN, 1=NEGINF, 2=POSINF, 3=ZERO, 4=TIEAWAY.
fn fp16_to_int16(a: u16, signed: bool, mode: u8) -> u16 {
    if fp16_is_nan(a) {
        return 0;
    }
    let x = fp16_to_f64(a);
    let r = match mode {
        0 => x.round_ties_even(),
        1 => x.floor(),
        2 => x.ceil(),
        3 => x.trunc(),
        _ => x.round(),
    };
    if signed {
        if r >= 32767.0 {
            return 32767i16 as u16;
        }
        if r <= -32768.0 {
            return -32768i16 as u16;
        }
        (r as i64 as i16) as u16
    } else {
        if r >= 65535.0 {
            return 0xFFFF;
        }
        if r <= 0.0 {
            return 0;
        }
        r as i64 as u16
    }
}

/// Convert a 16-bit integer lane to binary16 (round to nearest even).
fn int16_to_fp16(lane: u16, signed: bool) -> u16 {
    let v = if signed {
        (lane as i16) as f64
    } else {
        lane as f64
    };
    fp16_round(v)
}

/// SM4 S-box (GB/T 32907-2016).
const SM4_SBOX: [u8; 256] = [
    0xd6, 0x90, 0xe9, 0xfe, 0xcc, 0xe1, 0x3d, 0xb7, 0x16, 0xb6, 0x14, 0xc2, 0x28, 0xfb, 0x2c, 0x05,
    0x2b, 0x67, 0x9a, 0x76, 0x2a, 0xbe, 0x04, 0xc3, 0xaa, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99,
    0x9c, 0x42, 0x50, 0xf4, 0x91, 0xef, 0x98, 0x7a, 0x33, 0x54, 0x0b, 0x43, 0xed, 0xcf, 0xac, 0x62,
    0xe4, 0xb3, 0x1c, 0xa9, 0xc9, 0x08, 0xe8, 0x95, 0x80, 0xdf, 0x94, 0xfa, 0x75, 0x8f, 0x3f, 0xa6,
    0x47, 0x07, 0xa7, 0xfc, 0xf3, 0x73, 0x17, 0xba, 0x83, 0x59, 0x3c, 0x19, 0xe6, 0x85, 0x4f, 0xa8,
    0x68, 0x6b, 0x81, 0xb2, 0x71, 0x64, 0xda, 0x8b, 0xf8, 0xeb, 0x0f, 0x4b, 0x70, 0x56, 0x9d, 0x35,
    0x1e, 0x24, 0x0e, 0x5e, 0x63, 0x58, 0xd1, 0xa2, 0x25, 0x22, 0x7c, 0x3b, 0x01, 0x21, 0x78, 0x87,
    0xd4, 0x00, 0x46, 0x57, 0x9f, 0xd3, 0x27, 0x52, 0x4c, 0x36, 0x02, 0xe7, 0xa0, 0xc4, 0xc8, 0x9e,
    0xea, 0xbf, 0x8a, 0xd2, 0x40, 0xc7, 0x38, 0xb5, 0xa3, 0xf7, 0xf2, 0xce, 0xf9, 0x61, 0x15, 0xa1,
    0xe0, 0xae, 0x5d, 0xa4, 0x9b, 0x34, 0x1a, 0x55, 0xad, 0x93, 0x32, 0x30, 0xf5, 0x8c, 0xb1, 0xe3,
    0x1d, 0xf6, 0xe2, 0x2e, 0x82, 0x66, 0xca, 0x60, 0xc0, 0x29, 0x23, 0xab, 0x0d, 0x53, 0x4e, 0x6f,
    0xd5, 0xdb, 0x37, 0x45, 0xde, 0xfd, 0x8e, 0x2f, 0x03, 0xff, 0x6a, 0x72, 0x6d, 0x6c, 0x5b, 0x51,
    0x8d, 0x1b, 0xaf, 0x92, 0xbb, 0xdd, 0xbc, 0x7f, 0x11, 0xd9, 0x5c, 0x41, 0x1f, 0x10, 0x5a, 0xd8,
    0x0a, 0xc1, 0x31, 0x88, 0xa5, 0xcd, 0x7b, 0xbd, 0x2d, 0x74, 0xd0, 0x12, 0xb8, 0xe5, 0xb4, 0xb0,
    0x89, 0x69, 0x97, 0x4a, 0x0c, 0x96, 0x77, 0x7e, 0x65, 0xb9, 0xf1, 0x09, 0xc5, 0x6e, 0xc6, 0x84,
    0x18, 0xf0, 0x7d, 0xec, 0x3a, 0xdc, 0x4d, 0x20, 0x79, 0xee, 0x5f, 0x3e, 0xd7, 0xcb, 0x39, 0x48,
];

/// Apply the SM4 S-box to each of the four bytes of a 32-bit word.
fn sm4_sub(x: u32) -> u32 {
    let b = x.to_le_bytes();
    u32::from_le_bytes([
        SM4_SBOX[b[0] as usize],
        SM4_SBOX[b[1] as usize],
        SM4_SBOX[b[2] as usize],
        SM4_SBOX[b[3] as usize],
    ])
}

/// One SM4 round transform (4 sub-rounds). `key_or_const` supplies the four
/// 32-bit round inputs (round keys for SM4E, constants for SM4EKEY). `enc`
/// selects the encryption linear transform (ROL 2/10/18/24) vs the key
/// expansion transform (ROL 13/23).
fn sm4_rounds(mut rr: u128, key_or_const: u128, enc: bool) -> u128 {
    for index in 0..4 {
        let k = (key_or_const >> (index * 32)) as u32;
        let mut intval = (rr >> 96) as u32 ^ (rr >> 64) as u32 ^ (rr >> 32) as u32 ^ k;
        intval = sm4_sub(intval);
        intval = if enc {
            intval
                ^ intval.rotate_left(2)
                ^ intval.rotate_left(10)
                ^ intval.rotate_left(18)
                ^ intval.rotate_left(24)
        } else {
            intval ^ intval.rotate_left(13) ^ intval.rotate_left(23)
        };
        intval ^= rr as u32; // EOR roundresult<31:0>
        rr = (rr >> 32) | ((intval as u128) << 96);
    }
    rr
}

/// SM3 TT1/TT2 round transforms. `sel`: 0=TT1A, 1=TT1B, 2=TT2A, 3=TT2B.
/// `i` is the immediate lane index selecting the word of Vm.
fn sm3_tt(vd: u128, vn: u128, vm: u128, i: u32, sel: u32) -> u128 {
    let word = |v: u128, k: u32| (v >> (32 * k)) as u32;
    let d0 = word(vd, 0);
    let d1 = word(vd, 1);
    let d2 = word(vd, 2);
    let d3 = word(vd, 3);
    let wj = word(vm, i);
    let vn3 = word(vn, 3);
    let (tt, rot, mix) = match sel {
        0b00 => {
            // SM3TT1A
            let ss2 = vn3 ^ d3.rotate_left(12);
            let tt1 = d1 ^ (d3 ^ d2);
            (
                tt1.wrapping_add(d0).wrapping_add(ss2).wrapping_add(wj),
                9u32,
                false,
            )
        }
        0b01 => {
            // SM3TT1B (majority)
            let ss2 = vn3 ^ d3.rotate_left(12);
            let tt1 = (d3 & d1) | (d3 & d2) | (d1 & d2);
            (
                tt1.wrapping_add(d0).wrapping_add(ss2).wrapping_add(wj),
                9,
                false,
            )
        }
        0b10 => {
            // SM3TT2A
            let tt2 = d1 ^ (d3 ^ d2);
            (
                tt2.wrapping_add(d0).wrapping_add(vn3).wrapping_add(wj),
                19,
                true,
            )
        }
        _ => {
            // SM3TT2B
            let tt2 = (d3 & d2) | ((!d3) & d1);
            (
                tt2.wrapping_add(d0).wrapping_add(vn3).wrapping_add(wj),
                19,
                true,
            )
        }
    };
    let r0 = d1;
    let r1 = d2.rotate_left(rot);
    let r2 = d3;
    let r3 = if mix {
        tt ^ tt.rotate_left(9) ^ tt.rotate_left(17)
    } else {
        tt
    };
    (r0 as u128) | ((r1 as u128) << 32) | ((r2 as u128) << 64) | ((r3 as u128) << 96)
}

/// SM3PARTW1 message expansion.
fn sm3_partw1(vd: u128, vn: u128, vm: u128) -> u128 {
    let word = |v: u128, k: u32| (v >> (32 * k)) as u32;
    let vdn = vd ^ vn;
    let mut w = [0u32; 4];
    w[0] = word(vdn, 0) ^ word(vm, 1).rotate_left(15);
    w[1] = word(vdn, 1) ^ word(vm, 2).rotate_left(15);
    w[2] = word(vdn, 2) ^ word(vm, 3).rotate_left(15);
    for i in 0..4 {
        if i == 3 {
            w[3] = word(vdn, 3) ^ w[0].rotate_left(15);
        }
        w[i] = w[i] ^ w[i].rotate_left(15) ^ w[i].rotate_left(23);
    }
    (w[0] as u128) | ((w[1] as u128) << 32) | ((w[2] as u128) << 64) | ((w[3] as u128) << 96)
}

/// SM3PARTW2 message expansion.
fn sm3_partw2(vd: u128, vn: u128, vm: u128) -> u128 {
    let word = |v: u128, k: u32| (v >> (32 * k)) as u32;
    let mut tmp = [0u32; 4];
    for k in 0..4 {
        tmp[k as usize] = word(vn, k) ^ word(vm, k).rotate_left(7);
    }
    let mut r = [0u32; 4];
    for k in 0..4 {
        r[k] = word(vd, k as u32) ^ tmp[k];
    }
    let mut tmp2 = tmp[0].rotate_left(15);
    tmp2 = tmp2 ^ tmp2.rotate_left(15) ^ tmp2.rotate_left(23);
    r[3] ^= tmp2;
    (r[0] as u128) | ((r[1] as u128) << 32) | ((r[2] as u128) << 64) | ((r[3] as u128) << 96)
}

/// AES S-box and inverse S-box (FIPS-197).
const AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];
const AES_INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// GF(2^8) multiply with the AES reduction polynomial (0x11b).
#[inline]
fn aes_gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

#[inline]
fn aes_sub_bytes(state: u128, inverse: bool) -> u128 {
    let table = if inverse { &AES_INV_SBOX } else { &AES_SBOX };
    let mut b = state.to_le_bytes();
    for x in b.iter_mut() {
        *x = table[*x as usize];
    }
    u128::from_le_bytes(b)
}

/// AES ShiftRows on the column-major 16-byte state (or InvShiftRows).
#[inline]
fn aes_shift_rows(state: u128, inverse: bool) -> u128 {
    let s = state.to_le_bytes();
    let mut out = [0u8; 16];
    for r in 0..4usize {
        for c in 0..4usize {
            let src_c = if inverse {
                (c + 4 - r) % 4
            } else {
                (c + r) % 4
            };
            out[c * 4 + r] = s[src_c * 4 + r];
        }
    }
    u128::from_le_bytes(out)
}

/// AES MixColumns (or InvMixColumns) on the column-major 16-byte state.
#[inline]
fn aes_mix_columns(state: u128, inverse: bool) -> u128 {
    let s = state.to_le_bytes();
    let mut out = [0u8; 16];
    for c in 0..4usize {
        let a = [s[c * 4], s[c * 4 + 1], s[c * 4 + 2], s[c * 4 + 3]];
        let col = if inverse {
            [
                aes_gmul(a[0], 14) ^ aes_gmul(a[1], 11) ^ aes_gmul(a[2], 13) ^ aes_gmul(a[3], 9),
                aes_gmul(a[0], 9) ^ aes_gmul(a[1], 14) ^ aes_gmul(a[2], 11) ^ aes_gmul(a[3], 13),
                aes_gmul(a[0], 13) ^ aes_gmul(a[1], 9) ^ aes_gmul(a[2], 14) ^ aes_gmul(a[3], 11),
                aes_gmul(a[0], 11) ^ aes_gmul(a[1], 13) ^ aes_gmul(a[2], 9) ^ aes_gmul(a[3], 14),
            ]
        } else {
            [
                aes_gmul(a[0], 2) ^ aes_gmul(a[1], 3) ^ a[2] ^ a[3],
                a[0] ^ aes_gmul(a[1], 2) ^ aes_gmul(a[2], 3) ^ a[3],
                a[0] ^ a[1] ^ aes_gmul(a[2], 2) ^ aes_gmul(a[3], 3),
                aes_gmul(a[0], 3) ^ a[1] ^ a[2] ^ aes_gmul(a[3], 2),
            ]
        };
        out[c * 4..c * 4 + 4].copy_from_slice(&col);
    }
    u128::from_le_bytes(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm::memory::FlatMemory;

    fn create_test_cpu() -> AArch64Cpu {
        let memory = FlatMemory::new(0, 0x1000_0000);
        AArch64Cpu::new(AArch64Config::default(), Box::new(memory))
    }

    #[test]
    fn test_cpu_creation() {
        let cpu = create_test_cpu();
        assert_eq!(cpu.get_pc(), 0);
        assert_eq!(cpu.current_el(), 1);
    }

    #[test]
    fn test_register_access() {
        let mut cpu = create_test_cpu();

        cpu.set_x(0, 0x1234_5678_9ABC_DEF0);
        assert_eq!(cpu.get_x(0), 0x1234_5678_9ABC_DEF0);

        cpu.set_w(1, 0xDEAD_BEEF);
        assert_eq!(cpu.get_w(1), 0xDEAD_BEEF);
        assert_eq!(cpu.get_x(1), 0xDEAD_BEEF); // Zero-extended

        // XZR always reads 0
        assert_eq!(cpu.get_x(31), 0);
        cpu.set_x(31, 0xFFFF); // Write to XZR is ignored
        assert_eq!(cpu.get_x(31), 0);
    }

    #[test]
    fn test_condition_flags() {
        let mut cpu = create_test_cpu();

        cpu.set_nzcv(true, false, true, false);
        assert!(cpu.get_n());
        assert!(!cpu.get_z());
        assert!(cpu.get_c());
        assert!(!cpu.get_v());

        cpu.update_nz_64(0);
        assert!(!cpu.get_n());
        assert!(cpu.get_z());

        cpu.update_nz_64(0x8000_0000_0000_0000);
        assert!(cpu.get_n());
        assert!(!cpu.get_z());
    }

    #[test]
    fn test_condition_evaluation() {
        let mut cpu = create_test_cpu();

        // Test EQ (Z=1)
        cpu.set_z(true);
        assert!(cpu.condition_holds(0b0000)); // EQ
        assert!(!cpu.condition_holds(0b0001)); // NE

        // Test CS (C=1)
        cpu.set_c(true);
        assert!(cpu.condition_holds(0b0010)); // CS
        assert!(!cpu.condition_holds(0b0011)); // CC

        // Test AL (always)
        assert!(cpu.condition_holds(0b1110)); // AL
    }

    #[test]
    fn test_stack_pointer() {
        let mut cpu = create_test_cpu();

        cpu.set_current_sp(0x8000_0000);
        assert_eq!(cpu.current_sp(), 0x8000_0000);
    }

    #[test]
    fn test_bitmask_decode() {
        // Test 64-bit mode with N=1 (64-bit elements)
        // imms=0 means a single 1 bit, immr=0 means no rotation
        let mask = decode_bitmask(true, 0, 0, true).unwrap();
        assert_eq!(mask, 0x0000_0000_0000_0001);

        // imms=62 means 63 ones (all except MSB), immr=0
        let mask = decode_bitmask(true, 62, 0, true).unwrap();
        assert_eq!(mask, 0x7FFF_FFFF_FFFF_FFFF);

        // Test N=0 (smaller element sizes)
        // ~imms[5:0] = 0x20 = 0b100000, highest bit at position 5, so len=6 (invalid for N=0)
        // Let's use imms=0b011111, so ~imms[5:0]=0b100000, but that's still len=6

        // imms=0b111100, ~imms[5:0]=0b000011, highest bit at position 1, len=1
        // (2-bit elements). s = imms & 0b1 = 0, so element = 0b01.
        let mask = decode_bitmask(false, 0b111100, 0, true).unwrap();
        // 2-bit element 0b01 replicated: 0x5555555555555555
        assert_eq!(mask, 0x5555_5555_5555_5555);

        // 32-bit mode should mask result
        let mask = decode_bitmask(false, 0b111100, 0, false).unwrap();
        assert_eq!(mask, 0x0000_0000_5555_5555);
    }

    #[test]
    fn test_crc32() {
        // Test basic CRC32 functionality
        let crc = crc32(0, 0x12, 8);
        assert_ne!(crc, 0);

        let crc = crc32c(0, 0x12, 8);
        assert_ne!(crc, 0);
    }

    #[test]
    fn test_arm_cpu_trait() {
        let mut cpu = create_test_cpu();

        assert_eq!(cpu.arch_version(), ArmVersion::V8_0A);
        assert_eq!(cpu.profile(), ArmProfile::A);
        assert!(cpu.is_privileged()); // EL1 is privileged

        cpu.reset();
        assert_eq!(cpu.get_pc(), 0);

        // Test PSTATE
        let pstate = cpu.get_pstate();
        assert_eq!(pstate.el, 1);

        // Test register access via trait
        cpu.set_gpr(5, 0xDEAD_BEEF);
        assert_eq!(cpu.get_gpr(5), 0xDEAD_BEEF);

        // Test LR
        cpu.set_lr(0x1234);
        assert_eq!(cpu.get_lr(), 0x1234);
    }

    #[test]
    fn test_breakpoint() {
        let mut cpu = create_test_cpu();

        assert!(cpu.set_breakpoint(0x1000).is_ok());
        // set_breakpoint always succeeds (idempotent)
        assert!(cpu.set_breakpoint(0x1000).is_ok());

        assert!(cpu.clear_breakpoint(0x1000).is_ok());
        // clear_breakpoint is also idempotent
        assert!(cpu.clear_breakpoint(0x1000).is_ok());
    }

    // =========================================================================
    // Instruction Execution Tests
    // =========================================================================

    /// Helper to create a CPU and write an instruction at PC
    fn create_cpu_with_insn(insn: u32) -> AArch64Cpu {
        let mut cpu = create_test_cpu();
        cpu.write_memory(0, &insn.to_le_bytes()).unwrap();
        cpu
    }

    /// Helper to write instruction at specific address
    fn write_insn(cpu: &mut AArch64Cpu, addr: u64, insn: u32) {
        cpu.write_memory(addr, &insn.to_le_bytes()).unwrap();
    }

    // -------------------------------------------------------------------------
    // Data Processing Immediate - PC-relative addressing
    // -------------------------------------------------------------------------

    #[test]
    fn test_adr() {
        // ADR X0, #0x100 (PC + 0x100)
        // ADR: [0 immlo[1:0] 10000 immhi[18:0] Rd[4:0]]
        // PC=0, imm=0x100 -> immhi=0x40, immlo=0
        let insn = 0x10000800; // ADR X0, #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x100);
        assert_eq!(cpu.get_pc(), 4);
    }

    #[test]
    fn test_adrp() {
        // ADRP X1, #0x1000 (page-aligned, PC + 0x1000)
        // ADRP: [1 immlo[1:0] 10000 immhi[18:0] Rd[4:0]]
        let insn = 0x90000001; // ADRP X1, #0 (current page)
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(1), 0); // Page of PC=0
        assert_eq!(cpu.get_pc(), 4);
    }

    // -------------------------------------------------------------------------
    // Data Processing Immediate - Add/Subtract
    // -------------------------------------------------------------------------

    #[test]
    fn test_add_imm_64() {
        // ADD X0, X1, #0x123
        // sf=1, op=0, S=0, shift=0, imm12=0x123, Rn=1, Rd=0
        // [1 0 0 10001 00 imm12 Rn Rd]
        let insn = 0x91048C20; // ADD X0, X1, #0x123
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1123);
    }

    #[test]
    fn test_add_imm_32() {
        // ADD W0, W1, #0x50
        // sf=0
        let insn = 0x11014020; // ADD W0, W1, #0x50
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_0000_0100);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x150); // 32-bit result, zero-extended
    }

    #[test]
    fn test_adds_imm_sets_flags() {
        // ADDS X0, X1, #1 (result will be 0, sets Z flag)
        let insn = 0xB1000420; // ADDS X0, X1, #1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_FFFF_FFFF);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z()); // Zero flag
        assert!(cpu.get_c()); // Carry flag (overflow from addition)
    }

    #[test]
    fn test_sub_imm() {
        // SUB X0, X1, #0x100
        let insn = 0xD1040020; // SUB X0, X1, #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x500);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x400);
    }

    #[test]
    fn test_subs_imm_negative() {
        // SUBS X0, X1, #0x100 (result negative)
        let insn = 0xF1040020; // SUBS X0, X1, #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x50);
        cpu.step().unwrap();
        assert!(cpu.get_n()); // Negative
        assert!(!cpu.get_c()); // No borrow = C clear
    }

    #[test]
    fn test_add_imm_shifted() {
        // ADD X0, X1, #0x1, LSL #12
        // shift=1 means LSL #12
        let insn = 0x91400420; // ADD X0, X1, #1, LSL #12
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x2000);
    }

    // -------------------------------------------------------------------------
    // Data Processing Immediate - Logical
    // -------------------------------------------------------------------------

    #[test]
    fn test_and_imm() {
        // AND X0, X1, #0xFF (bitmask for low 8 bits)
        // For AND imm, the immediate is encoded as bitmask
        // N=1, immr=0, imms=7 gives 0xFF mask for 64-bit
        let insn = 0x92401C20; // AND X0, X1, #0xFF
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1234_5678);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x78);
    }

    #[test]
    fn test_orr_imm() {
        // ORR X0, X1, #0x1
        // N=1, immr=0, imms=0 -> single bit pattern
        // sf=1, opc=01, 100100, N=1, immr=000000, imms=000000, Rn=1, Rd=0
        // = 1 01 100100 1 000000 000000 00001 00000
        // = 0xB2400020
        let insn = 0xB2400020; // ORR X0, X1, #0x1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1234_5678);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1234_5679); // 0x1234_5678 | 0x1 = 0x1234_5679
    }

    #[test]
    fn test_eor_imm() {
        // EOR X0, X1, #1
        let insn = 0xD2400020; // EOR X0, X1, #1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xAAAA);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xAAAB);
    }

    #[test]
    fn test_ands_imm() {
        // ANDS X0, X1, #0xFF (sets flags)
        let insn = 0xF2401C20; // ANDS X0, X1, #0xFF
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z()); // Zero flag set
    }

    // -------------------------------------------------------------------------
    // Data Processing Immediate - Move Wide
    // -------------------------------------------------------------------------

    #[test]
    fn test_movz() {
        // MOVZ X0, #0x1234
        let insn = 0xD2824680; // MOVZ X0, #0x1234
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1234);
    }

    #[test]
    fn test_movz_shifted() {
        // MOVZ X0, #0xABCD, LSL #16 (hw=01)
        // Encoding: 1 10 100101 01 imm16 Rd = 0xD2B579A0
        let insn = 0xD2B579A0; // MOVZ X0, #0xABCD, LSL #16
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xABCD_0000);
    }

    #[test]
    fn test_movn() {
        // MOVN X0, #0 (result is ~0 = 0xFFFF_FFFF_FFFF_FFFF)
        let insn = 0x92800000; // MOVN X0, #0
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn test_movk() {
        // MOVK X0, #0x5678, LSL #16 (keep other bits)
        let insn = 0xF2AACF00; // MOVK X0, #0x5678, LSL #16
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x0000_0000_0000_1234);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x0000_0000_5678_1234);
    }

    // -------------------------------------------------------------------------
    // Data Processing Immediate - Bitfield
    // -------------------------------------------------------------------------

    #[test]
    fn test_ubfm_lsr() {
        // UBFM can implement LSR: LSR X0, X1, #4 = UBFM X0, X1, #4, #63
        let insn = 0xD344FC20; // UBFM X0, X1, #4, #63 (LSR #4)
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xF0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x0F);
    }

    #[test]
    fn test_ubfm_lsr_zero_fills_rotated_high_bits() {
        // UBFM X0, X1, #5, #63 is the LSR #5 alias.
        let insn = 0xD345FC20;
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1234_5678_9ABC_DEF0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x0091_A2B3_C4D5_E6F7);
    }

    #[test]
    fn test_ubfm_lsl_discards_rotated_out_high_bit() {
        // UBFM X0, X1, #63, #62 is the LSL #1 alias.
        let insn = 0xD37EFC20;
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x8000_0000_0000_0001);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x2);
    }

    #[test]
    fn test_ubfm_uxtb() {
        // UXTB W0, W1 = UBFM W0, W1, #0, #7
        let insn = 0x53001C20; // UBFM W0, W1, #0, #7
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_1234);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x34);
    }

    #[test]
    fn test_sbfm_asr() {
        // SBFM can implement ASR: ASR X0, X1, #4 = SBFM X0, X1, #4, #63
        let insn = 0x9344FC20; // SBFM X0, X1, #4, #63 (ASR #4)
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x8000_0000_0000_00F0u64);
        cpu.step().unwrap();
        // Sign-extended shift right
        assert_eq!(cpu.get_x(0), 0xF800_0000_0000_000F);
    }

    #[test]
    fn test_sbfm_sxtb() {
        // SXTB W0, W1 = SBFM W0, W1, #0, #7
        let insn = 0x13001C20; // SBFM W0, W1, #0, #7
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x80); // Negative byte
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FF80); // Sign-extended to 32-bit
    }

    #[test]
    fn test_bfm() {
        // BFM X0, X1, #4, #7 - insert bits
        let insn = 0xB344_1C20; // BFM X0, X1, #4, #7
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0xFFFF_FFFF_FFFF_0000);
        cpu.set_x(1, 0x00AB);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_000A);
    }

    #[test]
    fn test_bfm_bfi_inserts_without_clearing_lower_bits() {
        // BFI W0, W1, #8, #8 inserts the low byte of W1 into bits 15:8.
        let insn = 0x3318_1C20;
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_w(0, 0xA5A5_1234);
        cpu.set_w(1, 0x0000_00CC);
        cpu.step().unwrap();
        assert_eq!(cpu.get_w(0), 0xA5A5_CC34);
    }

    #[test]
    fn test_bfm_self_bfi_replicates_byte() {
        let mut cpu = create_test_cpu();
        write_insn(&mut cpu, 0, 0x3318_1C00); // BFI W0, W0, #8, #8
        write_insn(&mut cpu, 4, 0x3310_1C00); // BFI W0, W0, #16, #8
        write_insn(&mut cpu, 8, 0x3308_1C00); // BFI W0, W0, #24, #8
        cpu.set_w(0, 0x81);
        cpu.step().unwrap();
        cpu.step().unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_w(0), 0x8181_8181);
    }

    // -------------------------------------------------------------------------
    // Data Processing Immediate - Extract
    // -------------------------------------------------------------------------

    #[test]
    fn test_extr() {
        // EXTR X0, X1, X2, #8 - extract bits from concatenation
        // result = (X1 << (64-8)) | (X2 >> 8)
        let insn = 0x93C22020; // EXTR X0, X1, X2, #8
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x0000_0000_0000_00FF);
        cpu.set_x(2, 0xFF00_0000_0000_0000);
        cpu.step().unwrap();
        // (0xFF << 56) | (0xFF00... >> 8) = 0xFF00... | 0x00FF... = 0xFFFF...
        assert_eq!(cpu.get_x(0), 0xFFFF_0000_0000_0000);
    }

    #[test]
    fn test_ror_via_extr() {
        // ROR X0, X1, #4 = EXTR X0, X1, X1, #4
        let insn = 0x93C11020; // EXTR X0, X1, X1, #4
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xF);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xF000_0000_0000_0000);
    }

    // -------------------------------------------------------------------------
    // Branch Instructions - Conditional
    // -------------------------------------------------------------------------

    #[test]
    fn test_b_cond_taken() {
        // B.EQ #0x100 (taken when Z=1)
        let insn = 0x54000800; // B.EQ #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_z(true);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x100);
    }

    #[test]
    fn test_b_cond_not_taken() {
        // B.EQ #0x100 (not taken when Z=0)
        let insn = 0x54000800; // B.EQ #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_z(false);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 4); // Falls through
    }

    #[test]
    fn test_b_ne() {
        // B.NE #0x20
        let insn = 0x54000101; // B.NE #0x20
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_z(false);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x20);
    }

    // -------------------------------------------------------------------------
    // Branch Instructions - Unconditional
    // -------------------------------------------------------------------------

    #[test]
    fn test_b() {
        // B #0x1000
        let insn = 0x14000400; // B #0x1000
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x1000);
    }

    #[test]
    fn test_b_negative() {
        // B #-0x100 (backward branch)
        // imm26 = -0x40 (in instruction words) = 0x3FFFFC0
        let insn = 0x17FFFFC0; // B #-0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_pc(0x1000);
        write_insn(&mut cpu, 0x1000, insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0xF00);
    }

    #[test]
    fn test_bl() {
        // BL #0x100 (saves return address in X30)
        let insn = 0x94000040; // BL #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x100);
        assert_eq!(cpu.get_x(30), 4); // Return address
    }

    // -------------------------------------------------------------------------
    // Branch Instructions - Compare and Branch
    // -------------------------------------------------------------------------

    #[test]
    fn test_cbz_taken() {
        // CBZ X0, #0x100
        let insn = 0xB4000800; // CBZ X0, #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x100);
    }

    #[test]
    fn test_cbz_not_taken() {
        // CBZ X0, #0x100
        let insn = 0xB4000800; // CBZ X0, #0x100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 1);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 4);
    }

    #[test]
    fn test_cbnz_taken() {
        // CBNZ X1, #0x80
        let insn = 0xB5000401; // CBNZ X1, #0x80
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1234);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x80);
    }

    #[test]
    fn test_cbz_32bit() {
        // CBZ W0, #0x20
        let insn = 0x34000100; // CBZ W0, #0x20
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0xFFFF_FFFF_0000_0000); // Upper bits set but W0 is 0
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x20);
    }

    // -------------------------------------------------------------------------
    // Branch Instructions - Test and Branch
    // -------------------------------------------------------------------------

    #[test]
    fn test_tbz_taken() {
        // TBZ X0, #0, #0x40 (branch if bit 0 is 0)
        let insn = 0x36000200; // TBZ X0, #0, #0x40
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0xFFFE); // Bit 0 is 0
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x40);
    }

    #[test]
    fn test_tbz_not_taken() {
        // TBZ X0, #0, #0x40
        let insn = 0x36000200; // TBZ X0, #0, #0x40
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0xFFFF); // Bit 0 is 1
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 4);
    }

    #[test]
    fn test_tbnz_taken() {
        // TBNZ X0, #4, #0x80 (branch if bit 4 is 1)
        let insn = 0x37200400; // TBNZ X0, #4, #0x80
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x10); // Bit 4 is 1
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x80);
    }

    #[test]
    fn test_tbz_high_bit() {
        // TBZ X0, #63, #0x20 (test highest bit)
        let insn = 0xB6F80100; // TBZ X0, #63, #0x20
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x7FFF_FFFF_FFFF_FFFF); // Bit 63 is 0
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x20);
    }

    // -------------------------------------------------------------------------
    // Branch Instructions - Register
    // -------------------------------------------------------------------------

    #[test]
    fn test_br() {
        // BR X1
        let insn = 0xD61F0020; // BR X1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x2000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x2000);
    }

    #[test]
    fn test_blr() {
        // BLR X5
        let insn = 0xD63F00A0; // BLR X5
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(5, 0x4000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x4000);
        assert_eq!(cpu.get_x(30), 4); // Return address
    }

    #[test]
    fn test_ret() {
        // RET (uses X30 by default)
        let insn = 0xD65F03C0; // RET
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(30, 0x8000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x8000);
    }

    #[test]
    fn test_ret_xn() {
        // RET X5
        let insn = 0xD65F00A0; // RET X5
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(5, 0x3000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x3000);
    }

    // -------------------------------------------------------------------------
    // Load/Store Instructions - LDR Literal
    // -------------------------------------------------------------------------

    #[test]
    fn test_ldr_literal_64() {
        // LDR X0, #0x100 (load from PC+0x100)
        let insn = 0x58000800; // LDR X0, #0x100
        let mut cpu = create_cpu_with_insn(insn);
        // Write test value at offset 0x100
        cpu.write_memory(0x100, &0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xDEAD_BEEF_CAFE_BABE);
    }

    #[test]
    fn test_ldr_literal_32() {
        // LDR W0, #0x80
        let insn = 0x18000400; // LDR W0, #0x80
        let mut cpu = create_cpu_with_insn(insn);
        cpu.write_memory(0x80, &0x1234_5678u32.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1234_5678);
    }

    #[test]
    fn test_ldrsw_literal() {
        // LDRSW X0, #0x40 (sign-extended 32-bit load)
        let insn = 0x98000200; // LDRSW X0, #0x40
        let mut cpu = create_cpu_with_insn(insn);
        cpu.write_memory(0x40, &0x8000_0001u32.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_8000_0001); // Sign-extended
    }

    // -------------------------------------------------------------------------
    // Load/Store Instructions - Load/Store Pair
    // -------------------------------------------------------------------------

    #[test]
    fn test_stp_64() {
        // STP X0, X1, [X2]
        let insn = 0xA9000440; // STP X0, X1, [X2]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x1111_1111_1111_1111);
        cpu.set_x(1, 0x2222_2222_2222_2222);
        cpu.set_x(2, 0x1000);
        cpu.step().unwrap();

        let data = cpu.read_memory(0x1000, 8).unwrap();
        assert_eq!(
            u64::from_le_bytes(data[..8].try_into().unwrap()),
            0x1111_1111_1111_1111
        );

        let data = cpu.read_memory(0x1008, 8).unwrap();
        assert_eq!(
            u64::from_le_bytes(data[..8].try_into().unwrap()),
            0x2222_2222_2222_2222
        );
    }

    #[test]
    fn test_ldp_64() {
        // LDP X0, X1, [X2]
        let insn = 0xA9400440; // LDP X0, X1, [X2]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(2, 0x1000);
        cpu.write_memory(0x1000, &0xAAAA_BBBB_CCCC_DDDDu64.to_le_bytes())
            .unwrap();
        cpu.write_memory(0x1008, &0x1234_5678_9ABC_DEF0u64.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xAAAA_BBBB_CCCC_DDDD);
        assert_eq!(cpu.get_x(1), 0x1234_5678_9ABC_DEF0);
    }

    #[test]
    fn test_ldp_post_index() {
        // LDP X0, X1, [X2], #16
        let insn = 0xA8C10440; // LDP X0, X1, [X2], #16
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(2, 0x1000);
        cpu.write_memory(0x1000, &1u64.to_le_bytes()).unwrap();
        cpu.write_memory(0x1008, &2u64.to_le_bytes()).unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 1);
        assert_eq!(cpu.get_x(1), 2);
        assert_eq!(cpu.get_x(2), 0x1010); // Post-indexed
    }

    #[test]
    fn test_stp_pre_index() {
        // STP X0, X1, [X2, #-16]!
        let insn = 0xA9BF0440; // STP X0, X1, [X2, #-16]!
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x1111);
        cpu.set_x(1, 0x2222);
        cpu.set_x(2, 0x1010);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(2), 0x1000); // Pre-indexed
    }

    #[test]
    fn test_ldp_32() {
        // LDP W0, W1, [X2]
        let insn = 0x29400440; // LDP W0, W1, [X2]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(2, 0x1000);
        cpu.write_memory(0x1000, &0xDEAD_BEEFu32.to_le_bytes())
            .unwrap();
        cpu.write_memory(0x1004, &0xCAFE_BABEu32.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xDEAD_BEEF);
        assert_eq!(cpu.get_x(1), 0xCAFE_BABE);
    }

    // -------------------------------------------------------------------------
    // Load/Store Instructions - Register Offset
    // -------------------------------------------------------------------------

    #[test]
    fn test_str_imm() {
        // STR X0, [X1, #8]
        let insn = 0xF9000420; // STR X0, [X1, #8]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0xDEAD_BEEF_1234_5678);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();

        let data = cpu.read_memory(0x1008, 8).unwrap();
        assert_eq!(
            u64::from_le_bytes(data[..8].try_into().unwrap()),
            0xDEAD_BEEF_1234_5678
        );
    }

    #[test]
    fn test_ldr_imm() {
        // LDR X0, [X1, #16]
        let insn = 0xF9400820; // LDR X0, [X1, #16]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1010, &0xCAFE_BABE_DEAD_BEEFu64.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xCAFE_BABE_DEAD_BEEF);
    }

    #[test]
    fn test_strb() {
        // STRB W0, [X1]
        let insn = 0x39000020; // STRB W0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x1234_5678);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();

        let data = cpu.read_memory(0x1000, 1).unwrap();
        assert_eq!(data[0], 0x78);
    }

    #[test]
    fn test_ldrb() {
        // LDRB W0, [X1]
        let insn = 0x39400020; // LDRB W0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &[0xAB]).unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xAB);
    }

    #[test]
    fn test_ldrsb() {
        // LDRSB X0, [X1] (sign-extend byte to 64-bit)
        let insn = 0x39800020; // LDRSB X0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &[0x80]).unwrap(); // Negative byte
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_FF80);
    }

    #[test]
    fn test_strh() {
        // STRH W0, [X1]
        let insn = 0x79000020; // STRH W0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x1234_5678);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();

        let data = cpu.read_memory(0x1000, 2).unwrap();
        assert_eq!(u16::from_le_bytes(data[..2].try_into().unwrap()), 0x5678);
    }

    #[test]
    fn test_ldrh() {
        // LDRH W0, [X1]
        let insn = 0x79400020; // LDRH W0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &0xABCDu16.to_le_bytes()).unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xABCD);
    }

    #[test]
    fn test_ldrsh() {
        // LDRSH X0, [X1] (sign-extend halfword to 64-bit)
        let insn = 0x79800020; // LDRSH X0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &0x8001u16.to_le_bytes()).unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_8001);
    }

    #[test]
    fn test_str_32() {
        // STR W0, [X1]
        let insn = 0xB9000020; // STR W0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0xDEAD_BEEF);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();

        let data = cpu.read_memory(0x1000, 4).unwrap();
        assert_eq!(
            u32::from_le_bytes(data[..4].try_into().unwrap()),
            0xDEAD_BEEF
        );
    }

    #[test]
    fn test_ldr_32() {
        // LDR W0, [X1]
        let insn = 0xB9400020; // LDR W0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &0x1234_5678u32.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1234_5678);
    }

    #[test]
    fn test_ldrsw() {
        // LDRSW X0, [X1] (sign-extend word to 64-bit)
        let insn = 0xB9800020; // LDRSW X0, [X1]
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &0x8000_0001u32.to_le_bytes())
            .unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_8000_0001);
    }

    #[test]
    fn test_ldr_post_index() {
        // LDR X0, [X1], #8
        let insn = 0xF8408420; // LDR X0, [X1], #8
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.write_memory(0x1000, &0x1234u64.to_le_bytes()).unwrap();
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1234);
        assert_eq!(cpu.get_x(1), 0x1008); // Post-indexed
    }

    #[test]
    fn test_str_pre_index() {
        // STR X0, [X1, #8]!
        let insn = 0xF8008C20; // STR X0, [X1, #8]!
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 0x5678);
        cpu.set_x(1, 0x1000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(1), 0x1008); // Pre-indexed

        let data = cpu.read_memory(0x1008, 8).unwrap();
        assert_eq!(u64::from_le_bytes(data[..8].try_into().unwrap()), 0x5678);
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - Logical Shifted Register
    // -------------------------------------------------------------------------

    #[test]
    fn test_and_shifted() {
        // AND X0, X1, X2
        let insn = 0x8A020020; // AND X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFF00_FF00);
        cpu.set_x(2, 0x0FF0_0FF0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x0F00_0F00);
    }

    #[test]
    fn test_and_lsl() {
        // AND X0, X1, X2, LSL #4
        let insn = 0x8A021020; // AND X0, X1, X2, LSL #4
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF);
        cpu.set_x(2, 0x00FF);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x0FF0);
    }

    #[test]
    fn test_orr_reg() {
        // ORR X0, X1, X2
        let insn = 0xAA020020; // ORR X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xF0F0);
        cpu.set_x(2, 0x0F0F);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF);
    }

    #[test]
    fn test_eor_reg() {
        // EOR X0, X1, X2
        let insn = 0xCA020020; // EOR X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF);
        cpu.set_x(2, 0x0F0F);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xF0F0);
    }

    #[test]
    fn test_bic() {
        // BIC X0, X1, X2 (bit clear: X1 AND NOT X2)
        let insn = 0x8A220020; // BIC X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF);
        cpu.set_x(2, 0x00FF);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFF00);
    }

    #[test]
    fn test_orn() {
        // ORN X0, X1, X2 (X1 OR NOT X2)
        let insn = 0xAA220020; // ORN X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.set_x(2, 0xFF);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), !0xFFu64);
    }

    #[test]
    fn test_eon() {
        // EON X0, X1, X2 (X1 XOR NOT X2)
        let insn = 0xCA220020; // EON X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.set_x(2, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), !0u64);
    }

    #[test]
    fn test_ands_reg() {
        // ANDS X0, X1, X2 (sets flags)
        let insn = 0xEA020020; // ANDS X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000);
        cpu.set_x(2, 0x0001);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z()); // Result is zero
    }

    #[test]
    fn test_tst() {
        // TST X1, X2 (ANDS XZR, X1, X2)
        let insn = 0xEA02003F; // TST X1, X2 (Rd=XZR)
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x8000_0000_0000_0000);
        cpu.set_x(2, 0x8000_0000_0000_0000);
        cpu.step().unwrap();
        assert!(cpu.get_n()); // Negative (bit 63 set)
        assert!(!cpu.get_z()); // Not zero
    }

    #[test]
    fn test_mov_reg() {
        // MOV X0, X1 (alias for ORR X0, XZR, X1)
        let insn = 0xAA0103E0; // MOV X0, X1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xDEAD_BEEF);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xDEAD_BEEF);
    }

    #[test]
    fn test_mvn() {
        // MVN X0, X1 (alias for ORN X0, XZR, X1)
        let insn = 0xAA2103E0; // MVN X0, X1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), !0u64);
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - Add/Subtract Shifted/Extended
    // -------------------------------------------------------------------------

    #[test]
    fn test_add_shifted() {
        // ADD X0, X1, X2
        let insn = 0x8B020020; // ADD X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 200);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 300);
    }

    #[test]
    fn test_add_lsl() {
        // ADD X0, X1, X2, LSL #2
        let insn = 0x8B020820; // ADD X0, X1, X2, LSL #2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 25);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 200);
    }

    #[test]
    fn test_sub_shifted() {
        // SUB X0, X1, X2
        let insn = 0xCB020020; // SUB X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 500);
        cpu.set_x(2, 200);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 300);
    }

    #[test]
    fn test_adds_shifted() {
        // ADDS X0, X1, X2 (sets flags)
        let insn = 0xAB020020; // ADDS X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_FFFF_FFFF);
        cpu.set_x(2, 1);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z()); // Zero
        assert!(cpu.get_c()); // Carry
    }

    #[test]
    fn test_subs_shifted() {
        // SUBS X0, X1, X2 (CMP alias when Rd=XZR)
        let insn = 0xEB020020; // SUBS X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 100);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z());
        assert!(cpu.get_c()); // No borrow = C set
    }

    #[test]
    fn test_cmp() {
        // CMP X1, X2 (SUBS XZR, X1, X2)
        let insn = 0xEB02003F; // CMP X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 50);
        cpu.set_x(2, 100);
        cpu.step().unwrap();
        assert!(cpu.get_n()); // Negative
        assert!(!cpu.get_c()); // Borrow = C clear
    }

    #[test]
    fn test_cmn() {
        // CMN X1, X2 (ADDS XZR, X1, X2)
        let insn = 0xAB02003F; // CMN X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_FFFF_FFFF);
        cpu.set_x(2, 1);
        cpu.step().unwrap();
        assert!(cpu.get_z()); // Result is zero
        assert!(cpu.get_c()); // Carry out
    }

    #[test]
    fn test_neg() {
        // NEG X0, X1 (SUB X0, XZR, X1)
        let insn = 0xCB0103E0; // NEG X0, X1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 1);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn test_add_extended() {
        // ADD X0, X1, W2, UXTW (zero-extend W2 to 64-bit)
        let insn = 0x8B224020; // ADD X0, X1, W2, UXTW
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1000_0000_0000_0000);
        cpu.set_x(2, 0xFFFF_FFFF_0000_0100);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1000_0000_0000_0100);
    }

    #[test]
    fn test_add_extended_sxtw() {
        // ADD X0, X1, W2, SXTW (sign-extend W2 to 64-bit)
        let insn = 0x8B22C020; // ADD X0, X1, W2, SXTW
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.set_x(2, 0x8000_0000); // Negative when sign-extended
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_8000_0000);
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - ADC/SBC
    // -------------------------------------------------------------------------

    #[test]
    fn test_adc() {
        // ADC X0, X1, X2 (add with carry)
        let insn = 0x9A020020; // ADC X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 200);
        cpu.set_c(true);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 301); // 100 + 200 + 1
    }

    #[test]
    fn test_adc_no_carry() {
        // ADC X0, X1, X2 (no carry in)
        let insn = 0x9A020020; // ADC X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 200);
        cpu.set_c(false);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 300);
    }

    #[test]
    fn test_adcs() {
        // ADCS X0, X1, X2 (sets flags)
        let insn = 0xBA020020; // ADCS X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_FFFF_FFFF);
        cpu.set_x(2, 0);
        cpu.set_c(true);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z());
        assert!(cpu.get_c()); // Overflow
    }

    #[test]
    fn test_sbc() {
        // SBC X0, X1, X2 (subtract with carry/borrow)
        let insn = 0xDA020020; // SBC X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 500);
        cpu.set_x(2, 200);
        cpu.set_c(true); // No borrow
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 300);
    }

    #[test]
    fn test_sbc_borrow() {
        // SBC X0, X1, X2 (with borrow)
        let insn = 0xDA020020; // SBC X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 500);
        cpu.set_x(2, 200);
        cpu.set_c(false); // Borrow
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 299);
    }

    #[test]
    fn test_sbcs() {
        // SBCS X0, X1, X2 (sets flags)
        let insn = 0xFA020020; // SBCS X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 100);
        cpu.set_c(true);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
        assert!(cpu.get_z());
    }

    #[test]
    fn test_ngc() {
        // NGC X0, X1 (SBC X0, XZR, X1)
        let insn = 0xDA0103E0; // NGC X0, X1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.set_c(true);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - Conditional Compare
    // -------------------------------------------------------------------------

    #[test]
    fn test_ccmp_true() {
        // CCMP X1, X2, #0, EQ (compare if Z=1)
        // Encoding: sf=1 11 11010010 Rm cond 00 Rn 0 nzcv
        // = 111 11010010 00010 0000 00 00001 0 0000
        // = 0xFA420020
        let insn = 0xFA420020; // CCMP X1, X2, #0, EQ
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 100);
        cpu.set_z(true); // Condition true (EQ)
        cpu.step().unwrap();
        assert!(cpu.get_z()); // Result of comparison (100-100=0)
        assert!(cpu.get_c()); // No borrow
    }

    #[test]
    fn test_ccmp_false() {
        // CCMP X1, X2, #0b0100, EQ (use nzcv if Z=0)
        // Encoding: 111 11010010 00010 0000 00 00001 0 0100
        // = 0xFA420024
        let insn = 0xFA420024; // CCMP X1, X2, #4, EQ (nzcv=0100)
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_z(false); // Condition false
        cpu.step().unwrap();
        assert!(cpu.get_z()); // nzcv bit 2 = Z
        assert!(!cpu.get_c()); // nzcv bit 1 = C (clear)
    }

    #[test]
    fn test_ccmn() {
        // CCMN X1, X2, #0, NE (add comparison if Z=0)
        // Encoding: sf=1 01 11010010 Rm cond 00 Rn 0 nzcv (note: op=0 for CCMN)
        // = 101 11010010 00010 0001 00 00001 0 0000
        // = 0xBA421020
        let insn = 0xBA421020; // CCMN X1, X2, #0, NE
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_FFFF_FFFF);
        cpu.set_x(2, 1);
        cpu.set_z(false); // NE is true
        cpu.step().unwrap();
        assert!(cpu.get_z()); // Result is zero
        assert!(cpu.get_c()); // Carry out
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - Conditional Select
    // -------------------------------------------------------------------------

    #[test]
    fn test_csel_true() {
        // CSEL X0, X1, X2, EQ (select X1 if Z=1)
        let insn = 0x9A820020; // CSEL X0, X1, X2, EQ
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1111);
        cpu.set_x(2, 0x2222);
        cpu.set_z(true);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1111);
    }

    #[test]
    fn test_csel_false() {
        // CSEL X0, X1, X2, EQ (select X2 if Z=0)
        let insn = 0x9A820020; // CSEL X0, X1, X2, EQ
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1111);
        cpu.set_x(2, 0x2222);
        cpu.set_z(false);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x2222);
    }

    #[test]
    fn test_csinc_true() {
        // CSINC X0, X1, X2, NE (select X1 if Z=0)
        let insn = 0x9A821420; // CSINC X0, X1, X2, NE
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 200);
        cpu.set_z(false); // NE is true
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 100);
    }

    #[test]
    fn test_csinc_false() {
        // CSINC X0, X1, X2, NE (select X2+1 if Z=1)
        let insn = 0x9A821420; // CSINC X0, X1, X2, NE
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 200);
        cpu.set_z(true); // NE is false
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 201);
    }

    #[test]
    fn test_csinv() {
        // CSINV X0, X1, X2, EQ (select X1 if Z=1, else ~X2)
        let insn = 0xDA820020; // CSINV X0, X1, X2, EQ
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1111);
        cpu.set_x(2, 0);
        cpu.set_z(false);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), !0u64);
    }

    #[test]
    fn test_csneg() {
        // CSNEG X0, X1, X2, EQ (select X1 if Z=1, else -X2)
        let insn = 0xDA820420; // CSNEG X0, X1, X2, EQ
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0);
        cpu.set_x(2, 5);
        cpu.set_z(false);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_FFFB); // -5
    }

    #[test]
    fn test_cinc() {
        // CINC X0, X1, NE = CSINC X0, X1, X1, EQ
        // If EQ is true: X0 = X1
        // If EQ is false (NE is true): X0 = X1 + 1
        let insn = 0x9A810420; // CINC X0, X1, NE
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_z(false); // EQ is false, so NE is true -> X0 = X1 + 1
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 101);
    }

    #[test]
    fn test_cset() {
        // CSET X0, EQ (CSINC X0, XZR, XZR, NE)
        let insn = 0x9A9F17E0; // CSET X0, EQ
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_z(true); // EQ is true
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 1);
    }

    #[test]
    fn test_csetm() {
        // CSETM X0, EQ = CSINV X0, XZR, XZR, NE
        // If NE (Z=0): X0 = XZR = 0
        // If EQ (Z=1): X0 = NOT(XZR) = !0
        // Encoding: sf=1 op=1 S=0 11010100 Rm=11111 cond=0001(NE) op2=00 Rn=11111 Rd=00000
        // = 110 11010100 11111 0001 00 11111 00000 = 0xDA9F13E0
        let insn = 0xDA9F13E0; // CSETM X0, EQ (encoded as CSINV X0, XZR, XZR, NE)
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_z(true); // EQ is true, so NE is false -> X0 = !0
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), !0u64);
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - 2-source
    // -------------------------------------------------------------------------

    #[test]
    fn test_udiv() {
        // UDIV X0, X1, X2
        let insn = 0x9AC20820; // UDIV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 7);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 14);
    }

    #[test]
    fn test_udiv_by_zero() {
        // UDIV X0, X1, X2 (divide by zero returns 0)
        let insn = 0x9AC20820; // UDIV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
    }

    #[test]
    fn test_sdiv() {
        // SDIV X0, X1, X2
        let insn = 0x9AC20C20; // SDIV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, (-100i64) as u64);
        cpu.set_x(2, 7);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0) as i64, -14);
    }

    #[test]
    fn test_sdiv_by_zero() {
        // SDIV X0, X1, X2 (divide by zero returns 0)
        let insn = 0x9AC20C20; // SDIV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, (-100i64) as u64);
        cpu.set_x(2, 0);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0);
    }

    #[test]
    fn test_lslv() {
        // LSLV X0, X1, X2 (logical shift left variable)
        let insn = 0x9AC22020; // LSLV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFF);
        cpu.set_x(2, 4);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFF0);
    }

    #[test]
    fn test_lsrv() {
        // LSRV X0, X1, X2 (logical shift right variable)
        let insn = 0x9AC22420; // LSRV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFF0);
        cpu.set_x(2, 4);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xFF);
    }

    #[test]
    fn test_asrv() {
        // ASRV X0, X1, X2 (arithmetic shift right variable)
        let insn = 0x9AC22820; // ASRV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x8000_0000_0000_0000);
        cpu.set_x(2, 4);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xF800_0000_0000_0000);
    }

    #[test]
    fn test_rorv() {
        // RORV X0, X1, X2 (rotate right variable)
        let insn = 0x9AC22C20; // RORV X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xF);
        cpu.set_x(2, 4);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0xF000_0000_0000_0000);
    }

    // -------------------------------------------------------------------------
    // Data Processing Register - 3-source
    // -------------------------------------------------------------------------

    #[test]
    fn test_madd() {
        // MADD X0, X1, X2, X3 (X0 = X1*X2 + X3)
        let insn = 0x9B020C20; // MADD X0, X1, X2, X3
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 10);
        cpu.set_x(2, 20);
        cpu.set_x(3, 5);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 205);
    }

    #[test]
    fn test_mul() {
        // MUL X0, X1, X2 (MADD X0, X1, X2, XZR)
        let insn = 0x9B027C20; // MUL X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 100);
        cpu.set_x(2, 200);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 20000);
    }

    #[test]
    fn test_msub() {
        // MSUB X0, X1, X2, X3 (X0 = X3 - X1*X2)
        let insn = 0x9B028C20; // MSUB X0, X1, X2, X3
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 10);
        cpu.set_x(2, 20);
        cpu.set_x(3, 500);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 300);
    }

    #[test]
    fn test_mneg() {
        // MNEG X0, X1, X2 (MSUB X0, X1, X2, XZR)
        let insn = 0x9B02FC20; // MNEG X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 10);
        cpu.set_x(2, 20);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0) as i64, -200);
    }

    #[test]
    fn test_smaddl() {
        // SMADDL X0, W1, W2, X3 (signed widening multiply-add)
        let insn = 0x9B220C20; // SMADDL X0, W1, W2, X3
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF); // -1 as W
        cpu.set_x(2, 10);
        cpu.set_x(3, 100);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0) as i64, 90); // 100 + (-1 * 10)
    }

    #[test]
    fn test_smull() {
        // SMULL X0, W1, W2 (SMADDL X0, W1, W2, XZR)
        let insn = 0x9B227C20; // SMULL X0, W1, W2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF); // -1 as W
        cpu.set_x(2, 100);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0) as i64, -100);
    }

    #[test]
    fn test_umaddl() {
        // UMADDL X0, W1, W2, X3 (unsigned widening multiply-add)
        let insn = 0x9BA20C20; // UMADDL X0, W1, W2, X3
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF); // Max u32
        cpu.set_x(2, 2);
        cpu.set_x(3, 1);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1_FFFF_FFFF); // 2 * 0xFFFF_FFFF + 1
    }

    #[test]
    fn test_umull() {
        // UMULL X0, W1, W2 (UMADDL X0, W1, W2, XZR)
        let insn = 0x9BA27C20; // UMULL X0, W1, W2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x1_0000);
        cpu.set_x(2, 0x1_0000);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0x1_0000_0000);
    }

    #[test]
    fn test_smulh() {
        // SMULH X0, X1, X2 (signed high multiply)
        let insn = 0x9B427C20; // SMULH X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x8000_0000_0000_0000); // Large negative
        cpu.set_x(2, 2);
        cpu.step().unwrap();
        // Result is high 64 bits of signed 128-bit product
        assert_eq!(cpu.get_x(0), 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn test_umulh() {
        // UMULH X0, X1, X2 (unsigned high multiply)
        let insn = 0x9BC27C20; // UMULH X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x8000_0000_0000_0000);
        cpu.set_x(2, 2);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 1);
    }

    // -------------------------------------------------------------------------
    // System Instructions
    // -------------------------------------------------------------------------

    #[test]
    fn test_nop() {
        // NOP
        let insn = 0xD503201F; // NOP
        let mut cpu = create_cpu_with_insn(insn);
        let old_pc = cpu.get_pc();
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), old_pc + 4);
    }

    #[test]
    fn test_dmb() {
        // DMB SY
        let insn = 0xD5033FBF; // DMB SY
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 4);
    }

    #[test]
    fn test_dsb() {
        // DSB SY
        let insn = 0xD5033F9F; // DSB SY
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 4);
    }

    #[test]
    fn test_isb() {
        // ISB
        let insn = 0xD5033FDF; // ISB
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 4);
    }

    // -------------------------------------------------------------------------
    // Multi-instruction sequences
    // -------------------------------------------------------------------------

    #[test]
    fn test_simple_program() {
        // Simple program: MOV X0, #1; ADD X0, X0, #1; ADD X0, X0, #1
        let mut cpu = create_test_cpu();
        write_insn(&mut cpu, 0, 0xD2800020); // MOV X0, #1
        write_insn(&mut cpu, 4, 0x91000400); // ADD X0, X0, #1
        write_insn(&mut cpu, 8, 0x91000400); // ADD X0, X0, #1

        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 1);

        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 2);

        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 3);
    }

    #[test]
    fn test_loop() {
        // Simple countdown loop
        // 0x0000: MOV X0, #5
        // 0x0004: SUBS X0, X0, #1
        // 0x0008: B.NE #-4
        let mut cpu = create_test_cpu();
        write_insn(&mut cpu, 0, 0xD28000A0); // MOV X0, #5
        write_insn(&mut cpu, 4, 0xF1000400); // SUBS X0, X0, #1
        write_insn(&mut cpu, 8, 0x54FFFFE1); // B.NE #-4

        // Execute MOV
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 5);

        // Execute loop 5 times
        for expected in (0..5).rev() {
            cpu.step().unwrap(); // SUBS
            assert_eq!(cpu.get_x(0), expected);
            cpu.step().unwrap(); // B.NE or fall through
        }

        // After loop, PC should be at 0x0C (fell through)
        assert_eq!(cpu.get_pc(), 0x0C);
    }

    #[test]
    fn test_function_call() {
        // Test function call and return
        // 0x0000: MOV X0, #42
        // 0x0004: BL #0x100
        // 0x0008: ADD X0, X0, #1  (after return)
        // ...
        // 0x0104: ADD X0, X0, X0
        // 0x0108: RET
        let mut cpu = create_test_cpu();
        write_insn(&mut cpu, 0x0000, 0xD2800540); // MOV X0, #42
        write_insn(&mut cpu, 0x0004, 0x94000040); // BL #0x100
        write_insn(&mut cpu, 0x0008, 0x91000400); // ADD X0, X0, #1

        write_insn(&mut cpu, 0x0104, 0x8B000000); // ADD X0, X0, X0
        write_insn(&mut cpu, 0x0108, 0xD65F03C0); // RET

        // MOV X0, #42
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 42);

        // BL #0x100
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 0x104);
        assert_eq!(cpu.get_x(30), 8); // Return address

        // ADD X0, X0, X0
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 84);

        // RET
        cpu.step().unwrap();
        assert_eq!(cpu.get_pc(), 8);

        // ADD X0, X0, #1
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 85);
    }

    #[test]
    fn test_memory_operations() {
        // Test store and load sequence
        // MOV X0, #0xABCD
        // MOV X1, #0x1000
        // STR X0, [X1]
        // MOV X0, #0
        // LDR X2, [X1]
        let mut cpu = create_test_cpu();
        write_insn(&mut cpu, 0x00, 0xD29579A0); // MOV X0, #0xABCD (imm16=0xABCD, hw=0)
        write_insn(&mut cpu, 0x04, 0xD2820001); // MOV X1, #0x1000
        write_insn(&mut cpu, 0x08, 0xF9000020); // STR X0, [X1]
        write_insn(&mut cpu, 0x0C, 0xD2800000); // MOV X0, #0
        write_insn(&mut cpu, 0x10, 0xF9400022); // LDR X2, [X1]

        for _ in 0..5 {
            cpu.step().unwrap();
        }

        assert_eq!(cpu.get_x(0), 0);
        assert_eq!(cpu.get_x(2), 0xABCD);
    }

    // -------------------------------------------------------------------------
    // Edge cases and special values
    // -------------------------------------------------------------------------

    #[test]
    fn test_max_values() {
        // ADD with maximum 64-bit value
        let insn = 0x91000400; // ADD X0, X0, #1
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, u64::MAX);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 0); // Wraps around
    }

    #[test]
    fn test_signed_overflow() {
        // ADDS with signed overflow
        let insn = 0xAB020020; // ADDS X0, X1, X2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0x7FFF_FFFF_FFFF_FFFF); // Max positive
        cpu.set_x(2, 1);
        cpu.step().unwrap();
        assert!(cpu.get_v()); // Overflow flag set
        assert!(cpu.get_n()); // Result is negative
    }

    #[test]
    fn test_zero_register_as_source() {
        // ADD X0, XZR, #100 (XZR as source)
        // imm12 = 100 = 0x64, Rn = 31 (XZR), Rd = 0
        let insn = 0x910193E0; // ADD X0, XZR, #100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.step().unwrap();
        assert_eq!(cpu.get_x(0), 100);
    }

    #[test]
    fn test_zero_register_as_dest() {
        // ADD XZR, X1, #100 (XZR as destination, discards result)
        let insn = 0x9119003F; // ADD XZR, X0, #100
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(0, 50);
        cpu.step().unwrap();
        // Result discarded, XZR still reads 0
        assert_eq!(cpu.get_x(31), 0);
    }

    #[test]
    fn test_32bit_operations() {
        // 32-bit operations should zero-extend
        let insn = 0x0B020020; // ADD W0, W1, W2
        let mut cpu = create_cpu_with_insn(insn);
        cpu.set_x(1, 0xFFFF_FFFF_0000_0001);
        cpu.set_x(2, 0xFFFF_FFFF_0000_0001);
        cpu.step().unwrap();
        // Result is 32-bit, zero-extended to 64
        assert_eq!(cpu.get_x(0), 2);
    }
}
