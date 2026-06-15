//! Cortex-M CPU implementation.
//!
//! This module provides a complete Cortex-M processor implementation including:
//! - All Cortex-M variants (M0, M0+, M3, M4, M7, M23, M33, M55, M85)
//! - Thumb/Thumb-2 instruction execution
//! - Exception handling and NVIC integration
//! - Memory protection (MPU)
//! - FPU support (M4, M7, M33+)
//! - DSP extension support

use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;

use crate::arm::cpu_trait::{
    AccessType, ArmCpu, ArmError, ArmException, ArmProfile, ArmVersion, CpuExit, DebugEvent,
    MemoryFaultInfo, MemoryFaultType, ProcessorState, WatchpointKind,
};
use crate::arm::features::ArmFeatures;
use crate::arm::memory::{ArmMemory, FlatMemory, MemoryError, StandardMemory};
use crate::arm::vfp::VfpState;

use super::nvic::Nvic;
use super::scb::{CortexMVariant, Scb, cfsr, hfsr};
use super::systick::SysTick;

/// Exception frame size in bytes.
const EXCEPTION_FRAME_SIZE: u32 = 32; // 8 registers * 4 bytes
const EXCEPTION_FRAME_SIZE_FPU: u32 = 104; // Standard + 16 FP regs + FPSCR + reserved

/// EXC_RETURN values.
pub mod exc_return {
    /// Return to Handler mode, use MSP.
    pub const HANDLER_MSP: u32 = 0xFFFF_FFF1;
    /// Return to Thread mode, use MSP.
    pub const THREAD_MSP: u32 = 0xFFFF_FFF9;
    /// Return to Thread mode, use PSP.
    pub const THREAD_PSP: u32 = 0xFFFF_FFFD;
    /// Return with FPU state (ARMv7-M+).
    pub const FPU_ACTIVE: u32 = 0xFFFF_FFE1;

    /// Check if returning to Thread mode.
    pub fn is_thread_mode(exc_return: u32) -> bool {
        exc_return & 0x8 != 0
    }

    /// Check if using PSP.
    pub fn uses_psp(exc_return: u32) -> bool {
        exc_return & 0x4 != 0
    }

    /// Check if FPU context was saved.
    pub fn fpu_active(exc_return: u32) -> bool {
        exc_return & 0x10 == 0
    }
}

/// Cortex-M CPU state.
pub struct CortexMCpu {
    /// CPU variant.
    variant: CortexMVariant,
    /// General purpose registers R0-R12.
    regs: [u32; 13],
    /// Stack Pointer (banked: MSP and PSP).
    sp_main: u32,
    sp_process: u32,
    /// Link Register.
    lr: u32,
    /// Program Counter.
    pc: u32,
    /// Program Status Register (xPSR).
    xpsr: u32,
    /// PRIMASK special register.
    primask: bool,
    /// FAULTMASK special register (ARMv7-M+).
    faultmask: bool,
    /// BASEPRI special register (ARMv7-M+).
    basepri: u8,
    /// CONTROL special register.
    control: u8,
    /// Currently in Thread mode (false = Handler mode).
    thread_mode: bool,
    /// Current exception number (0 = Thread mode).
    current_exception: u16,

    /// Memory subsystem.
    memory: Box<dyn ArmMemory>,
    /// NVIC.
    nvic: Nvic,
    /// System Control Block.
    scb: Scb,
    /// SysTick timer.
    systick: SysTick,
    /// VFP/FPU state (optional).
    vfp: Option<VfpState>,

    /// Instruction count.
    insn_count: u64,
    /// Cycle count.
    cycle_count: u64,
    /// Breakpoints.
    breakpoints: HashSet<u64>,
    /// Watchpoints (address, size, kind).
    watchpoints: Vec<(u64, usize, WatchpointKind)>,
    /// Halted state.
    halted: bool,
    /// Sleep state.
    sleeping: bool,
    /// Pending exceptions queue.
    pending_exceptions: Vec<ArmException>,

    /// ARM features.
    features: ArmFeatures,
    /// Architecture version.
    version: ArmVersion,
}

impl CortexMCpu {
    /// Create a new Cortex-M CPU.
    pub fn new(variant: CortexMVariant, memory: Box<dyn ArmMemory>) -> Self {
        let (nvic, version, features) = match variant {
            CortexMVariant::CortexM0 => {
                (Nvic::for_cortex_m0(), ArmVersion::V6M, ArmFeatures::THUMB)
            }
            CortexMVariant::CortexM0Plus => (
                Nvic::for_cortex_m0plus(),
                ArmVersion::V6M,
                ArmFeatures::THUMB,
            ),
            CortexMVariant::CortexM1 => {
                (Nvic::for_cortex_m0(), ArmVersion::V6M, ArmFeatures::THUMB)
            }
            CortexMVariant::CortexM3 => (
                Nvic::for_cortex_m3(),
                ArmVersion::V7M,
                ArmFeatures::THUMB | ArmFeatures::THUMB2,
            ),
            CortexMVariant::CortexM4 => (
                Nvic::for_cortex_m4(),
                ArmVersion::V7EM,
                ArmFeatures::THUMB | ArmFeatures::THUMB2 | ArmFeatures::DSP | ArmFeatures::VFP,
            ),
            CortexMVariant::CortexM7 => (
                Nvic::for_cortex_m7(),
                ArmVersion::V7EM,
                ArmFeatures::THUMB
                    | ArmFeatures::THUMB2
                    | ArmFeatures::DSP
                    | ArmFeatures::VFP
                    | ArmFeatures::VFP_D32,
            ),
            CortexMVariant::CortexM23 => (
                Nvic::for_cortex_m23(),
                ArmVersion::V8MBaseline,
                ArmFeatures::THUMB | ArmFeatures::TRUSTZONE,
            ),
            CortexMVariant::CortexM33 => (
                Nvic::for_cortex_m33(),
                ArmVersion::V8MMainline,
                ArmFeatures::THUMB
                    | ArmFeatures::THUMB2
                    | ArmFeatures::DSP
                    | ArmFeatures::VFP
                    | ArmFeatures::TRUSTZONE,
            ),
            CortexMVariant::CortexM35P => (
                Nvic::for_cortex_m33(),
                ArmVersion::V8MMainline,
                ArmFeatures::THUMB
                    | ArmFeatures::THUMB2
                    | ArmFeatures::DSP
                    | ArmFeatures::VFP
                    | ArmFeatures::TRUSTZONE,
            ),
            CortexMVariant::CortexM55 => (
                Nvic::for_cortex_m55(),
                ArmVersion::V8_1M,
                ArmFeatures::THUMB
                    | ArmFeatures::THUMB2
                    | ArmFeatures::DSP
                    | ArmFeatures::VFP
                    | ArmFeatures::MVE
                    | ArmFeatures::TRUSTZONE,
            ),
            CortexMVariant::CortexM85 => (
                Nvic::for_cortex_m85(),
                ArmVersion::V8_1M,
                ArmFeatures::THUMB
                    | ArmFeatures::THUMB2
                    | ArmFeatures::DSP
                    | ArmFeatures::VFP
                    | ArmFeatures::MVE
                    | ArmFeatures::TRUSTZONE,
            ),
        };

        let vfp = if variant.has_fpu() {
            Some(VfpState::new())
        } else {
            None
        };

        Self {
            variant,
            regs: [0; 13],
            sp_main: 0,
            sp_process: 0,
            lr: 0xFFFF_FFFF, // Initial LR is all ones
            pc: 0,
            xpsr: 0x0100_0000, // T bit set (Thumb mode)
            primask: false,
            faultmask: false,
            basepri: 0,
            control: 0,
            thread_mode: true,
            current_exception: 0,
            memory,
            nvic,
            scb: Scb::new(variant),
            systick: SysTick::with_frequency(100_000_000), // 100 MHz default
            vfp,
            insn_count: 0,
            cycle_count: 0,
            breakpoints: HashSet::new(),
            watchpoints: Vec::new(),
            halted: false,
            sleeping: false,
            pending_exceptions: Vec::new(),
            features,
            version,
        }
    }

    /// Create a new Cortex-M4 with flat memory.
    pub fn new_m4(memory_size: usize) -> Self {
        let memory = Box::new(FlatMemory::new(0, memory_size));
        Self::new(CortexMVariant::CortexM4, memory)
    }

    /// Create a new Cortex-M with standard memory.
    pub fn with_standard_memory(variant: CortexMVariant, ram_base: u64, ram_size: usize) -> Self {
        let memory = Box::new(StandardMemory::with_ram(ram_base, ram_size));
        Self::new(variant, memory)
    }

    /// Get the CPU variant.
    pub fn variant(&self) -> CortexMVariant {
        self.variant
    }

    /// Get reference to NVIC.
    pub fn nvic(&self) -> &Nvic {
        &self.nvic
    }

    /// Get mutable reference to NVIC.
    pub fn nvic_mut(&mut self) -> &mut Nvic {
        &mut self.nvic
    }

    /// Get reference to SCB.
    pub fn scb(&self) -> &Scb {
        &self.scb
    }

    /// Get mutable reference to SCB.
    pub fn scb_mut(&mut self) -> &mut Scb {
        &mut self.scb
    }

    /// Get reference to SysTick.
    pub fn systick(&self) -> &SysTick {
        &self.systick
    }

    /// Get mutable reference to SysTick.
    pub fn systick_mut(&mut self) -> &mut SysTick {
        &mut self.systick
    }

    /// Get reference to memory.
    pub fn memory(&self) -> &dyn ArmMemory {
        self.memory.as_ref()
    }

    /// Get mutable reference to memory.
    pub fn memory_mut(&mut self) -> &mut dyn ArmMemory {
        self.memory.as_mut()
    }

    // =========================================================================
    // Stack Pointer Management
    // =========================================================================

    /// Get current stack pointer.
    fn current_sp(&self) -> u32 {
        if self.uses_psp() {
            self.sp_process
        } else {
            self.sp_main
        }
    }

    /// Set current stack pointer.
    fn set_current_sp(&mut self, value: u32) {
        if self.uses_psp() {
            self.sp_process = value;
        } else {
            self.sp_main = value;
        }
    }

    /// Check if using PSP.
    fn uses_psp(&self) -> bool {
        self.thread_mode && (self.control & 0x2) != 0
    }

    /// Check if in privileged mode.
    fn is_privileged_mode(&self) -> bool {
        !self.thread_mode || (self.control & 0x1) == 0
    }

    // =========================================================================
    // xPSR Access
    // =========================================================================

    /// Get N flag.
    pub fn get_n(&self) -> bool {
        (self.xpsr >> 31) & 1 != 0
    }

    /// Get Z flag.
    pub fn get_z(&self) -> bool {
        (self.xpsr >> 30) & 1 != 0
    }

    /// Get C flag.
    pub fn get_c(&self) -> bool {
        (self.xpsr >> 29) & 1 != 0
    }

    /// Get V flag.
    pub fn get_v(&self) -> bool {
        (self.xpsr >> 28) & 1 != 0
    }

    /// Get Q flag.
    pub fn get_q(&self) -> bool {
        (self.xpsr >> 27) & 1 != 0
    }

    /// Set N flag.
    pub fn set_n(&mut self, value: bool) {
        if value {
            self.xpsr |= 1 << 31;
        } else {
            self.xpsr &= !(1 << 31);
        }
    }

    /// Set Z flag.
    pub fn set_z(&mut self, value: bool) {
        if value {
            self.xpsr |= 1 << 30;
        } else {
            self.xpsr &= !(1 << 30);
        }
    }

    /// Set C flag.
    pub fn set_c(&mut self, value: bool) {
        if value {
            self.xpsr |= 1 << 29;
        } else {
            self.xpsr &= !(1 << 29);
        }
    }

    /// Set V flag.
    pub fn set_v(&mut self, value: bool) {
        if value {
            self.xpsr |= 1 << 28;
        } else {
            self.xpsr &= !(1 << 28);
        }
    }

    /// Set Q flag (sticky).
    pub fn set_q(&mut self) {
        self.xpsr |= 1 << 27;
    }

    /// Update N and Z flags from result.
    pub fn update_nz(&mut self, result: u32) {
        self.set_n((result as i32) < 0);
        self.set_z(result == 0);
    }

    // =========================================================================
    // Memory Access
    // =========================================================================

    fn mem_read_u8(&self, addr: u32) -> Result<u8, ArmError> {
        self.memory.read_u8(addr as u64).map_err(|e| e.into())
    }

    fn mem_read_u16(&self, addr: u32) -> Result<u16, ArmError> {
        self.memory.read_u16(addr as u64).map_err(|e| e.into())
    }

    fn mem_read_u32(&self, addr: u32) -> Result<u32, ArmError> {
        self.memory.read_u32(addr as u64).map_err(|e| e.into())
    }

    fn mem_write_u8(&mut self, addr: u32, value: u8) -> Result<(), ArmError> {
        self.memory
            .write_u8(addr as u64, value)
            .map_err(|e| e.into())
    }

    fn mem_write_u16(&mut self, addr: u32, value: u16) -> Result<(), ArmError> {
        self.memory
            .write_u16(addr as u64, value)
            .map_err(|e| e.into())
    }

    fn mem_write_u32(&mut self, addr: u32, value: u32) -> Result<(), ArmError> {
        self.memory
            .write_u32(addr as u64, value)
            .map_err(|e| e.into())
    }

    // =========================================================================
    // Exception Handling
    // =========================================================================

    /// Take an exception.
    fn take_exception(&mut self, exception_num: u16) -> Result<(), ArmError> {
        // Save context to stack
        let frame_size = if self.vfp.is_some() && self.is_fpu_active() {
            EXCEPTION_FRAME_SIZE_FPU
        } else {
            EXCEPTION_FRAME_SIZE
        };

        let sp = self.current_sp();
        let new_sp = (sp - frame_size) & !0x7; // 8-byte aligned

        // Push exception frame: R0, R1, R2, R3, R12, LR, PC, xPSR
        self.mem_write_u32(new_sp, self.regs[0])?;
        self.mem_write_u32(new_sp + 4, self.regs[1])?;
        self.mem_write_u32(new_sp + 8, self.regs[2])?;
        self.mem_write_u32(new_sp + 12, self.regs[3])?;
        self.mem_write_u32(new_sp + 16, self.regs[12])?;
        self.mem_write_u32(new_sp + 20, self.lr)?;
        self.mem_write_u32(new_sp + 24, self.pc)?;
        self.mem_write_u32(new_sp + 28, self.xpsr)?;

        self.set_current_sp(new_sp);

        // Determine EXC_RETURN value
        let exc_return = if self.thread_mode {
            if self.uses_psp() {
                exc_return::THREAD_PSP
            } else {
                exc_return::THREAD_MSP
            }
        } else {
            exc_return::HANDLER_MSP
        };

        self.lr = exc_return;
        self.nvic.set_exc_return(exc_return);

        // Switch to Handler mode
        self.thread_mode = false;
        self.current_exception = exception_num;

        // Clear FAULTMASK on return from any exception except NMI
        if exception_num != 2 {
            self.faultmask = false;
        }

        // Get vector address
        let vtor = self.scb.vtor();
        let vector_addr = vtor + (exception_num as u32 * 4);
        let handler = self.mem_read_u32(vector_addr)?;

        // Set PC to handler (clear thumb bit for PC)
        self.pc = handler & !1;

        // Set exception number in IPSR
        self.xpsr = (self.xpsr & 0xFFFF_FE00) | (exception_num as u32);

        // Update NVIC state
        self.nvic.enter_exception(exception_num);

        Ok(())
    }

    /// Return from exception.
    fn exception_return(&mut self, exc_return: u32) -> Result<CpuExit, ArmError> {
        let uses_psp = exc_return::uses_psp(exc_return);
        let to_thread = exc_return::is_thread_mode(exc_return);

        // Get stack pointer
        let sp = if uses_psp {
            self.sp_process
        } else {
            self.sp_main
        };

        // Pop exception frame
        self.regs[0] = self.mem_read_u32(sp)?;
        self.regs[1] = self.mem_read_u32(sp + 4)?;
        self.regs[2] = self.mem_read_u32(sp + 8)?;
        self.regs[3] = self.mem_read_u32(sp + 12)?;
        self.regs[12] = self.mem_read_u32(sp + 16)?;
        self.lr = self.mem_read_u32(sp + 20)?;
        self.pc = self.mem_read_u32(sp + 24)?;
        self.xpsr = self.mem_read_u32(sp + 28)?;

        // Restore stack pointer
        let frame_size = if self.vfp.is_some() && exc_return::fpu_active(exc_return) {
            EXCEPTION_FRAME_SIZE_FPU
        } else {
            EXCEPTION_FRAME_SIZE
        };

        let new_sp = sp + frame_size;
        if uses_psp {
            self.sp_process = new_sp;
        } else {
            self.sp_main = new_sp;
        }

        // Update mode
        self.thread_mode = to_thread;

        // Clear IPSR if returning to Thread mode
        if to_thread {
            self.xpsr &= 0xFFFF_FE00;
        }

        // Update NVIC state
        self.nvic.return_from_exception();
        self.current_exception = if to_thread {
            0
        } else {
            (self.xpsr & 0x1FF) as u16
        };

        Ok(CpuExit::Continue)
    }

    /// Check if FPU context is active.
    fn is_fpu_active(&self) -> bool {
        // FPU is active if we have VFP and it's enabled
        self.vfp.as_ref().map_or(false, |v| v.is_enabled())
    }

    /// Check for pending exceptions and take the highest priority one.
    fn check_pending_exceptions(&mut self) -> Result<Option<CpuExit>, ArmError> {
        // Check SysTick
        if self.systick.is_pending() {
            self.scb.set_systick_pending(true);
        }

        // Check system exceptions first
        if self.scb.is_nmi_pending() {
            self.scb.set_nmi_pending(false);
            self.take_exception(2)?;
            return Ok(Some(CpuExit::ExceptionTaken(ArmException::Nmi)));
        }

        if self.scb.is_pendsv_pending() {
            let priority = self.nvic.get_system_priority(14);
            if priority < self.nvic.execution_priority() {
                self.scb.set_pendsv_pending(false);
                self.take_exception(14)?;
                return Ok(Some(CpuExit::ExceptionTaken(ArmException::PendSv)));
            }
        }

        if self.scb.is_systick_pending() {
            let priority = self.nvic.get_system_priority(15);
            if priority < self.nvic.execution_priority() {
                self.scb.set_systick_pending(false);
                self.systick.take_pending();
                self.take_exception(15)?;
                return Ok(Some(CpuExit::ExceptionTaken(ArmException::SysTick)));
            }
        }

        // Check external interrupts
        if let Some(exception_num) = self.nvic.get_pending_exception() {
            self.take_exception(exception_num)?;
            let irq = exception_num - 16;
            return Ok(Some(CpuExit::ExceptionTaken(ArmException::Irq(irq))));
        }

        Ok(None)
    }

    // =========================================================================
    // Instruction Execution
    // =========================================================================

    /// Fetch and decode instruction.
    fn fetch_instruction(&self) -> Result<(u32, usize), ArmError> {
        let pc = self.pc;

        // Fetch first halfword
        let hw1 = self
            .memory
            .fetch_u16(pc as u64)
            .map_err(|e| ArmError::from(e))?;

        // Check if this is a 32-bit instruction
        let is_32bit = (hw1 & 0xF800) >= 0xE800;

        if is_32bit {
            let hw2 = self
                .memory
                .fetch_u16((pc + 2) as u64)
                .map_err(|e| ArmError::from(e))?;
            let insn = ((hw1 as u32) << 16) | (hw2 as u32);
            Ok((insn, 4))
        } else {
            Ok((hw1 as u32, 2))
        }
    }

    /// Execute one instruction.
    fn execute_instruction(&mut self) -> Result<CpuExit, ArmError> {
        let (insn, size) = self.fetch_instruction()?;

        // Check for breakpoint
        if self.breakpoints.contains(&(self.pc as u64)) {
            return Ok(CpuExit::Breakpoint(self.pc));
        }

        // Advance PC before execution (for proper exception handling)
        let old_pc = self.pc;
        self.pc += size as u32;

        // Execute instruction
        let result = if size == 2 {
            self.execute_thumb16(insn as u16)
        } else {
            self.execute_thumb32(insn)
        };

        // Handle execution result
        match result {
            Ok(exit) => {
                self.insn_count += 1;
                self.cycle_count += 1;
                Ok(exit)
            }
            Err(e) => {
                // Restore PC on error
                self.pc = old_pc;
                Err(e)
            }
        }
    }

    /// Execute a 16-bit Thumb instruction.
    fn execute_thumb16(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        // Basic instruction decoding
        match insn >> 10 {
            0b000000..=0b000011 => self.exec_shift_imm(insn),
            0b000100..=0b000111 => self.exec_add_sub(insn),
            0b001000..=0b001111 => self.exec_mov_cmp_add_sub_imm(insn),
            0b010000 => self.exec_data_processing(insn),
            0b010001 => self.exec_special_data(insn),
            0b010010..=0b010011 => self.exec_ldr_literal(insn),
            0b010100..=0b100111 => self.exec_load_store(insn),
            0b101000..=0b101001 => self.exec_adr(insn),
            0b101010..=0b101011 => self.exec_add_sp_imm(insn),
            0b101100..=0b101111 => self.exec_misc(insn),
            0b110000..=0b110001 => self.exec_stm(insn),
            0b110010..=0b110011 => self.exec_ldm(insn),
            0b110100..=0b110111 => self.exec_conditional_branch(insn),
            0b111000..=0b111001 => self.exec_unconditional_branch(insn),
            _ => Err(ArmError::UndefinedInstruction(insn as u32)),
        }
    }

    /// Execute a 32-bit Thumb-2 instruction.
    fn execute_thumb32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op1 = (insn >> 27) & 0x3;
        let op2 = (insn >> 20) & 0x7F;
        let _op = (insn >> 15) & 0x1;

        match op1 {
            0b01 => {
                // Load/store, data processing
                if op2 & 0x64 == 0x00 {
                    self.exec_load_store_multiple_32(insn)
                } else if op2 & 0x64 == 0x04 {
                    self.exec_load_store_dual_32(insn)
                } else if op2 & 0x60 == 0x20 {
                    self.exec_data_processing_shifted_32(insn)
                } else {
                    self.exec_coprocessor_32(insn)
                }
            }
            0b10 => {
                if (insn >> 15) & 1 == 0 {
                    // Data processing (modified immediate)
                    self.exec_data_processing_imm_32(insn)
                } else {
                    // Branches and misc
                    self.exec_branches_misc_32(insn)
                }
            }
            0b11 => {
                // Load/store single, data processing register
                if op2 & 0x71 == 0x00 {
                    self.exec_store_single_32(insn)
                } else if op2 & 0x67 == 0x01 {
                    self.exec_load_byte_32(insn)
                } else if op2 & 0x67 == 0x03 {
                    self.exec_load_halfword_32(insn)
                } else if op2 & 0x67 == 0x05 {
                    self.exec_load_word_32(insn)
                } else if op2 & 0x70 == 0x20 {
                    self.exec_data_processing_reg_32(insn)
                } else if op2 & 0x78 == 0x30 {
                    self.exec_multiply_32(insn)
                } else if op2 & 0x78 == 0x38 {
                    self.exec_long_multiply_32(insn)
                } else {
                    Err(ArmError::UndefinedInstruction(insn))
                }
            }
            _ => Err(ArmError::UndefinedInstruction(insn)),
        }
    }

    // =========================================================================
    // Thumb-16 Instruction Handlers
    // =========================================================================

    fn exec_shift_imm(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 11) & 0x3;
        let imm5 = ((insn >> 6) & 0x1F) as u32;
        let rm = ((insn >> 3) & 0x7) as usize;
        let rd = (insn & 0x7) as usize;

        let rm_val = self.regs[rm];
        let (result, carry) = match op {
            0b00 => {
                // LSL
                if imm5 == 0 {
                    (rm_val, self.get_c())
                } else {
                    let c = (rm_val >> (32 - imm5)) & 1 != 0;
                    (rm_val << imm5, c)
                }
            }
            0b01 => {
                // LSR
                let shift = if imm5 == 0 { 32 } else { imm5 };
                let c = if shift == 32 {
                    (rm_val >> 31) & 1 != 0
                } else {
                    (rm_val >> (shift - 1)) & 1 != 0
                };
                (rm_val >> shift, c)
            }
            0b10 => {
                // ASR
                let shift = if imm5 == 0 { 32 } else { imm5 };
                let c = if shift >= 32 {
                    (rm_val as i32) < 0
                } else {
                    (rm_val >> (shift - 1)) & 1 != 0
                };
                (((rm_val as i32) >> shift.min(31)) as u32, c)
            }
            _ => return Err(ArmError::UndefinedInstruction(insn as u32)),
        };

        self.regs[rd] = result;
        self.update_nz(result);
        self.set_c(carry);

        Ok(CpuExit::Continue)
    }

    fn exec_add_sub(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 9) & 0x3;
        let rn_imm = ((insn >> 6) & 0x7) as u32;
        let rm = ((insn >> 3) & 0x7) as usize;
        let rd = (insn & 0x7) as usize;

        let rm_val = self.regs[rm];
        let operand = if op & 0x2 != 0 {
            rn_imm // immediate
        } else {
            self.regs[rn_imm as usize] // register
        };

        let (result, carry, overflow) = if op & 0x1 != 0 {
            // SUB
            let (r, c) = rm_val.overflowing_sub(operand);
            let v = ((rm_val ^ operand) & (rm_val ^ r)) >> 31 != 0;
            (r, !c, v) // Note: ARM carry is inverted for subtraction
        } else {
            // ADD
            let (r, c) = rm_val.overflowing_add(operand);
            let v = (!(rm_val ^ operand) & (rm_val ^ r)) >> 31 != 0;
            (r, c, v)
        };

        self.regs[rd] = result;
        self.update_nz(result);
        self.set_c(carry);
        self.set_v(overflow);

        Ok(CpuExit::Continue)
    }

    fn exec_mov_cmp_add_sub_imm(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 11) & 0x3;
        let rd = ((insn >> 8) & 0x7) as usize;
        let imm8 = (insn & 0xFF) as u32;

        match op {
            0b00 => {
                // MOV
                self.regs[rd] = imm8;
                self.update_nz(imm8);
            }
            0b01 => {
                // CMP
                let rn_val = self.regs[rd];
                let (result, borrow) = rn_val.overflowing_sub(imm8);
                let v = ((rn_val ^ imm8) & (rn_val ^ result)) >> 31 != 0;
                self.update_nz(result);
                self.set_c(!borrow);
                self.set_v(v);
            }
            0b10 => {
                // ADD
                let rd_val = self.regs[rd];
                let (result, carry) = rd_val.overflowing_add(imm8);
                let v = (!(rd_val ^ imm8) & (rd_val ^ result)) >> 31 != 0;
                self.regs[rd] = result;
                self.update_nz(result);
                self.set_c(carry);
                self.set_v(v);
            }
            0b11 => {
                // SUB
                let rd_val = self.regs[rd];
                let (result, borrow) = rd_val.overflowing_sub(imm8);
                let v = ((rd_val ^ imm8) & (rd_val ^ result)) >> 31 != 0;
                self.regs[rd] = result;
                self.update_nz(result);
                self.set_c(!borrow);
                self.set_v(v);
            }
            _ => unreachable!(),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_data_processing(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 6) & 0xF;
        let rm = ((insn >> 3) & 0x7) as usize;
        let rdn = (insn & 0x7) as usize;

        let rdn_val = self.regs[rdn];
        let rm_val = self.regs[rm];

        let result = match op {
            0b0000 => {
                // AND
                let r = rdn_val & rm_val;
                self.update_nz(r);
                r
            }
            0b0001 => {
                // EOR
                let r = rdn_val ^ rm_val;
                self.update_nz(r);
                r
            }
            0b0010 => {
                // LSL
                let shift = rm_val & 0xFF;
                let (r, c) = if shift == 0 {
                    (rdn_val, self.get_c())
                } else if shift < 32 {
                    let c = (rdn_val >> (32 - shift)) & 1 != 0;
                    (rdn_val << shift, c)
                } else if shift == 32 {
                    (0, rdn_val & 1 != 0)
                } else {
                    (0, false)
                };
                self.update_nz(r);
                self.set_c(c);
                r
            }
            0b0011 => {
                // LSR
                let shift = rm_val & 0xFF;
                let (r, c) = if shift == 0 {
                    (rdn_val, self.get_c())
                } else if shift < 32 {
                    let c = (rdn_val >> (shift - 1)) & 1 != 0;
                    (rdn_val >> shift, c)
                } else if shift == 32 {
                    (0, (rdn_val >> 31) & 1 != 0)
                } else {
                    (0, false)
                };
                self.update_nz(r);
                self.set_c(c);
                r
            }
            0b0100 => {
                // ASR
                let shift = rm_val & 0xFF;
                let (r, c) = if shift == 0 {
                    (rdn_val, self.get_c())
                } else if shift < 32 {
                    let c = (rdn_val >> (shift - 1)) & 1 != 0;
                    (((rdn_val as i32) >> shift) as u32, c)
                } else {
                    let sign = (rdn_val as i32) < 0;
                    (if sign { 0xFFFF_FFFF } else { 0 }, sign)
                };
                self.update_nz(r);
                self.set_c(c);
                r
            }
            0b0101 => {
                // ADC
                let carry_in = if self.get_c() { 1u32 } else { 0 };
                let (t1, c1) = rdn_val.overflowing_add(rm_val);
                let (r, c2) = t1.overflowing_add(carry_in);
                let v = (!(rdn_val ^ rm_val) & (rdn_val ^ r)) >> 31 != 0;
                self.update_nz(r);
                self.set_c(c1 || c2);
                self.set_v(v);
                r
            }
            0b0110 => {
                // SBC
                let borrow_in = if self.get_c() { 0u32 } else { 1 };
                let (t1, b1) = rdn_val.overflowing_sub(rm_val);
                let (r, b2) = t1.overflowing_sub(borrow_in);
                let v = ((rdn_val ^ rm_val) & (rdn_val ^ r)) >> 31 != 0;
                self.update_nz(r);
                self.set_c(!(b1 || b2));
                self.set_v(v);
                r
            }
            0b0111 => {
                // ROR
                let shift = rm_val & 0x1F;
                let (r, c) = if rm_val & 0xFF == 0 {
                    (rdn_val, self.get_c())
                } else if shift == 0 {
                    (rdn_val, (rdn_val >> 31) & 1 != 0)
                } else {
                    let c = (rdn_val >> (shift - 1)) & 1 != 0;
                    (rdn_val.rotate_right(shift), c)
                };
                self.update_nz(r);
                self.set_c(c);
                r
            }
            0b1000 => {
                // TST
                let r = rdn_val & rm_val;
                self.update_nz(r);
                return Ok(CpuExit::Continue); // Don't update register
            }
            0b1001 => {
                // RSB (NEG)
                let (r, borrow) = 0u32.overflowing_sub(rm_val);
                let v = (rm_val & r) >> 31 != 0;
                self.update_nz(r);
                self.set_c(!borrow);
                self.set_v(v);
                r
            }
            0b1010 => {
                // CMP
                let (r, borrow) = rdn_val.overflowing_sub(rm_val);
                let v = ((rdn_val ^ rm_val) & (rdn_val ^ r)) >> 31 != 0;
                self.update_nz(r);
                self.set_c(!borrow);
                self.set_v(v);
                return Ok(CpuExit::Continue);
            }
            0b1011 => {
                // CMN
                let (r, carry) = rdn_val.overflowing_add(rm_val);
                let v = (!(rdn_val ^ rm_val) & (rdn_val ^ r)) >> 31 != 0;
                self.update_nz(r);
                self.set_c(carry);
                self.set_v(v);
                return Ok(CpuExit::Continue);
            }
            0b1100 => {
                // ORR
                let r = rdn_val | rm_val;
                self.update_nz(r);
                r
            }
            0b1101 => {
                // MUL
                let r = rdn_val.wrapping_mul(rm_val);
                self.update_nz(r);
                r
            }
            0b1110 => {
                // BIC
                let r = rdn_val & !rm_val;
                self.update_nz(r);
                r
            }
            0b1111 => {
                // MVN
                let r = !rm_val;
                self.update_nz(r);
                r
            }
            _ => return Err(ArmError::UndefinedInstruction(insn as u32)),
        };

        self.regs[rdn] = result;
        Ok(CpuExit::Continue)
    }

    fn exec_special_data(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 8) & 0x3;
        let d = (insn >> 7) & 1;
        let rm = ((insn >> 3) & 0xF) as usize;
        let rdn = ((insn & 0x7) | (d << 3)) as usize;

        match op {
            0b00 => {
                // ADD (high registers)
                let result = self.reg(rdn).wrapping_add(self.reg(rm));
                if rdn == 15 {
                    self.pc = result & !1;
                } else {
                    self.set_reg(rdn, result);
                }
            }
            0b01 => {
                // CMP (high registers)
                let rdn_val = self.reg(rdn);
                let rm_val = self.reg(rm);
                let (result, borrow) = rdn_val.overflowing_sub(rm_val);
                let v = ((rdn_val ^ rm_val) & (rdn_val ^ result)) >> 31 != 0;
                self.update_nz(result);
                self.set_c(!borrow);
                self.set_v(v);
            }
            0b10 => {
                // MOV (high registers)
                let value = self.reg(rm);
                if rdn == 15 {
                    self.pc = value & !1;
                } else {
                    self.set_reg(rdn, value);
                }
            }
            0b11 => {
                // BX/BLX
                let rm_val = self.reg(rm);
                if (insn >> 7) & 1 != 0 {
                    // BLX
                    self.lr = (self.pc - 2) | 1;
                }
                // Check for exception return
                if rm_val >= 0xF000_0000 {
                    return self.exception_return(rm_val);
                }
                self.pc = rm_val & !1;
            }
            _ => unreachable!(),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_ldr_literal(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let rt = ((insn >> 8) & 0x7) as usize;
        let imm8 = (insn & 0xFF) as u32;
        let addr = ((self.pc + 2) & !3) + (imm8 << 2);
        self.regs[rt] = self.mem_read_u32(addr)?;
        Ok(CpuExit::Continue)
    }

    fn exec_load_store(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 12) & 0xF;

        match op {
            0b0101 => {
                // Load/store register offset
                let opb = (insn >> 9) & 0x7;
                let rm = ((insn >> 6) & 0x7) as usize;
                let rn = ((insn >> 3) & 0x7) as usize;
                let rt = (insn & 0x7) as usize;
                let addr = self.regs[rn].wrapping_add(self.regs[rm]);

                match opb {
                    0b000 => self.mem_write_u32(addr, self.regs[rt])?, // STR
                    0b001 => self.mem_write_u16(addr, self.regs[rt] as u16)?, // STRH
                    0b010 => self.mem_write_u8(addr, self.regs[rt] as u8)?, // STRB
                    0b011 => self.regs[rt] = self.mem_read_u8(addr)? as i8 as i32 as u32, // LDRSB
                    0b100 => self.regs[rt] = self.mem_read_u32(addr)?, // LDR
                    0b101 => self.regs[rt] = self.mem_read_u16(addr)? as u32, // LDRH
                    0b110 => self.regs[rt] = self.mem_read_u8(addr)? as u32, // LDRB
                    0b111 => self.regs[rt] = self.mem_read_u16(addr)? as i16 as i32 as u32, // LDRSH
                    _ => unreachable!(),
                }
            }
            0b0110 => {
                // STR/LDR (imm, word)
                let l = (insn >> 11) & 1;
                let imm5 = ((insn >> 6) & 0x1F) as u32;
                let rn = ((insn >> 3) & 0x7) as usize;
                let rt = (insn & 0x7) as usize;
                let addr = self.regs[rn].wrapping_add(imm5 << 2);

                if l != 0 {
                    self.regs[rt] = self.mem_read_u32(addr)?;
                } else {
                    self.mem_write_u32(addr, self.regs[rt])?;
                }
            }
            0b0111 => {
                // STRB/LDRB (imm)
                let l = (insn >> 11) & 1;
                let imm5 = ((insn >> 6) & 0x1F) as u32;
                let rn = ((insn >> 3) & 0x7) as usize;
                let rt = (insn & 0x7) as usize;
                let addr = self.regs[rn].wrapping_add(imm5);

                if l != 0 {
                    self.regs[rt] = self.mem_read_u8(addr)? as u32;
                } else {
                    self.mem_write_u8(addr, self.regs[rt] as u8)?;
                }
            }
            0b1000 => {
                // STRH/LDRH (imm)
                let l = (insn >> 11) & 1;
                let imm5 = ((insn >> 6) & 0x1F) as u32;
                let rn = ((insn >> 3) & 0x7) as usize;
                let rt = (insn & 0x7) as usize;
                let addr = self.regs[rn].wrapping_add(imm5 << 1);

                if l != 0 {
                    self.regs[rt] = self.mem_read_u16(addr)? as u32;
                } else {
                    self.mem_write_u16(addr, self.regs[rt] as u16)?;
                }
            }
            0b1001 => {
                // STR/LDR (SP relative)
                let l = (insn >> 11) & 1;
                let rt = ((insn >> 8) & 0x7) as usize;
                let imm8 = (insn & 0xFF) as u32;
                let addr = self.current_sp().wrapping_add(imm8 << 2);

                if l != 0 {
                    self.regs[rt] = self.mem_read_u32(addr)?;
                } else {
                    self.mem_write_u32(addr, self.regs[rt])?;
                }
            }
            _ => return Err(ArmError::UndefinedInstruction(insn as u32)),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_adr(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let rd = ((insn >> 8) & 0x7) as usize;
        let imm8 = (insn & 0xFF) as u32;
        self.regs[rd] = ((self.pc + 2) & !3).wrapping_add(imm8 << 2);
        Ok(CpuExit::Continue)
    }

    fn exec_add_sp_imm(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let sp = (insn >> 11) & 1;
        let rd = ((insn >> 8) & 0x7) as usize;
        let imm8 = (insn & 0xFF) as u32;

        if sp != 0 {
            self.regs[rd] = self.current_sp().wrapping_add(imm8 << 2);
        } else {
            self.regs[rd] = ((self.pc + 2) & !3).wrapping_add(imm8 << 2);
        }
        Ok(CpuExit::Continue)
    }

    fn exec_misc(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let op = (insn >> 5) & 0x7F;

        match op {
            0b0000000..=0b0000011 => {
                // ADD/SUB SP
                let s = (insn >> 7) & 1;
                let imm7 = (insn & 0x7F) as u32;
                let offset = imm7 << 2;
                let sp = self.current_sp();
                if s != 0 {
                    self.set_current_sp(sp.wrapping_sub(offset));
                } else {
                    self.set_current_sp(sp.wrapping_add(offset));
                }
                Ok(CpuExit::Continue)
            }
            0b0001000..=0b0001111 => {
                // SXTH/SXTB/UXTH/UXTB
                let op = (insn >> 6) & 0x3;
                let rm = ((insn >> 3) & 0x7) as usize;
                let rd = (insn & 0x7) as usize;
                let rm_val = self.regs[rm];

                self.regs[rd] = match op {
                    0b00 => (rm_val as i16 as i32) as u32, // SXTH
                    0b01 => (rm_val as i8 as i32) as u32,  // SXTB
                    0b10 => rm_val & 0xFFFF,               // UXTH
                    0b11 => rm_val & 0xFF,                 // UXTB
                    _ => unreachable!(),
                };
                Ok(CpuExit::Continue)
            }
            0b0100000..=0b0100111 => self.exec_push(insn), // PUSH (0100xxx, includes M bit)
            0b0110110 => {
                // CPS
                if self.is_privileged_mode() {
                    let im = (insn >> 4) & 1;
                    let _a = (insn >> 2) & 1; // FAULTMASK not supported on all variants
                    let i = (insn >> 1) & 1;
                    let _f = insn & 1;

                    if i != 0 {
                        self.primask = im != 0;
                    }
                }
                Ok(CpuExit::Continue)
            }
            0b0101000..=0b0101111 => {
                // REV/REV16/REVSH
                let op = (insn >> 6) & 0x3;
                let rm = ((insn >> 3) & 0x7) as usize;
                let rd = (insn & 0x7) as usize;
                let rm_val = self.regs[rm];

                self.regs[rd] = match op {
                    0b00 => rm_val.swap_bytes(), // REV
                    0b01 => ((rm_val >> 8) & 0x00FF_00FF) | ((rm_val << 8) & 0xFF00_FF00), // REV16
                    0b11 => ((rm_val & 0xFF) << 8 | (rm_val >> 8) & 0xFF) as i16 as i32 as u32, // REVSH
                    _ => return Err(ArmError::UndefinedInstruction(insn as u32)),
                };
                Ok(CpuExit::Continue)
            }
            0b1100000..=0b1101111 => self.exec_pop(insn), // POP (110xxxx)
            0b1110000..=0b1110111 => {
                // BKPT (1110xxx)
                let imm8 = insn & 0xFF;
                Ok(CpuExit::Breakpoint(imm8 as u32))
            }
            0b1111000..=0b1111111 => {
                // NOP hints (1111xxx)
                let op = (insn >> 4) & 0xF;
                match op {
                    0b0000 => Ok(CpuExit::Continue), // NOP
                    0b0001 => Ok(CpuExit::Continue), // YIELD
                    0b0010 => Ok(CpuExit::Wfe),      // WFE
                    0b0011 => Ok(CpuExit::Wfi),      // WFI
                    0b0100 => Ok(CpuExit::Continue), // SEV
                    _ => Ok(CpuExit::Continue),
                }
            }
            _ => Err(ArmError::UndefinedInstruction(insn as u32)),
        }
    }

    fn exec_push(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let m = (insn >> 8) & 1;
        let register_list = (insn & 0xFF) | ((m as u16) << 14);
        let count = register_list.count_ones();

        let mut addr = self.current_sp().wrapping_sub(count * 4);
        self.set_current_sp(addr);

        for i in 0..15 {
            if (register_list >> i) & 1 != 0 {
                let value = if i == 14 {
                    self.lr
                } else {
                    self.regs[i as usize]
                };
                self.mem_write_u32(addr, value)?;
                addr = addr.wrapping_add(4);
            }
        }

        Ok(CpuExit::Continue)
    }

    fn exec_pop(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let p = (insn >> 8) & 1;
        let register_list = (insn & 0xFF) | ((p as u16) << 15);

        let mut addr = self.current_sp();

        for i in 0..16 {
            if (register_list >> i) & 1 != 0 {
                let value = self.mem_read_u32(addr)?;
                if i == 15 {
                    // Check for exception return
                    if value >= 0xF000_0000 {
                        self.set_current_sp(addr.wrapping_add(4));
                        return self.exception_return(value);
                    }
                    self.pc = value & !1;
                } else {
                    self.regs[i as usize] = value;
                }
                addr = addr.wrapping_add(4);
            }
        }

        self.set_current_sp(addr);
        Ok(CpuExit::Continue)
    }

    fn exec_stm(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let rn = ((insn >> 8) & 0x7) as usize;
        let register_list = insn & 0xFF;

        let mut addr = self.regs[rn];

        for i in 0..8 {
            if (register_list >> i) & 1 != 0 {
                self.mem_write_u32(addr, self.regs[i as usize])?;
                addr = addr.wrapping_add(4);
            }
        }

        self.regs[rn] = addr;
        Ok(CpuExit::Continue)
    }

    fn exec_ldm(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let rn = ((insn >> 8) & 0x7) as usize;
        let register_list = insn & 0xFF;

        let mut addr = self.regs[rn];
        let writeback = (register_list >> rn) & 1 == 0;

        for i in 0..8 {
            if (register_list >> i) & 1 != 0 {
                self.regs[i as usize] = self.mem_read_u32(addr)?;
                addr = addr.wrapping_add(4);
            }
        }

        if writeback {
            self.regs[rn] = addr;
        }
        Ok(CpuExit::Continue)
    }

    fn exec_conditional_branch(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let cond = (insn >> 8) & 0xF;
        let imm8 = (insn & 0xFF) as i8 as i32;

        if cond == 0xE {
            return Err(ArmError::UndefinedInstruction(insn as u32)); // UDF
        }
        if cond == 0xF {
            // SVC
            return Ok(CpuExit::Svc((insn & 0xFF) as u32));
        }

        if self.condition_passed(cond as u8) {
            // In ARM, PC during execution = current_instruction + 4
            // We've already advanced PC by 2, so add 2 more for ARM PC
            let offset = imm8 << 1;
            self.pc = ((self.pc + 2) as i32).wrapping_add(offset) as u32;
        }

        Ok(CpuExit::Continue)
    }

    fn exec_unconditional_branch(&mut self, insn: u16) -> Result<CpuExit, ArmError> {
        let imm11 = (insn & 0x7FF) as i32;
        let offset = ((imm11 << 21) >> 20) as i32; // Sign extend and shift left 1
        // In ARM, PC during execution = current_instruction + 4
        // We've already advanced PC by 2, so add 2 more for ARM PC
        self.pc = ((self.pc + 2) as i32).wrapping_add(offset) as u32;
        Ok(CpuExit::Continue)
    }

    // =========================================================================
    // Thumb-32 Instruction Handlers (Stubs)
    // =========================================================================

    fn exec_load_store_multiple_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Encoding: 1110 100 op:2 w:1 l:1 rn:4 | pm:1 m:1 register_list:14
        let op = (insn >> 23) & 0x3;
        let w = (insn >> 21) & 1 != 0; // Writeback
        let l = (insn >> 20) & 1 != 0; // Load (vs Store)
        let rn = ((insn >> 16) & 0xF) as usize;
        let register_list = (insn & 0xFFFF) as u16;

        if register_list == 0 {
            return Err(ArmError::UndefinedInstruction(insn));
        }

        let rn_val = self.reg(rn);
        let reg_count = register_list.count_ones();

        // Determine addressing mode
        let (start_addr, end_addr, wb_addr) = match op {
            0b00 => {
                // STMDB / LDMDB (decrement before, full descending)
                let start = rn_val.wrapping_sub(reg_count * 4);
                (start, rn_val.wrapping_sub(4), start)
            }
            0b01 => {
                // STM / LDM (increment after, empty ascending)
                (
                    rn_val,
                    rn_val.wrapping_add((reg_count - 1) * 4),
                    rn_val.wrapping_add(reg_count * 4),
                )
            }
            0b10 => {
                // STMDB / LDMDB (decrement before)
                let start = rn_val.wrapping_sub(reg_count * 4);
                (start, rn_val.wrapping_sub(4), start)
            }
            0b11 => {
                // STMIA / LDMIA (increment after)
                (
                    rn_val,
                    rn_val.wrapping_add((reg_count - 1) * 4),
                    rn_val.wrapping_add(reg_count * 4),
                )
            }
            _ => unreachable!(),
        };

        let mut addr = start_addr;

        if l {
            // Load multiple
            for i in 0..15 {
                if (register_list >> i) & 1 != 0 {
                    let value = self.mem_read_u32(addr)?;
                    self.set_reg(i, value);
                    addr = addr.wrapping_add(4);
                }
            }
            // Check for PC in list
            if (register_list >> 15) & 1 != 0 {
                let value = self.mem_read_u32(addr)?;
                self.pc = value & !1;
            }
        } else {
            // Store multiple
            for i in 0..16 {
                if (register_list >> i) & 1 != 0 {
                    let value = self.reg(i);
                    self.mem_write_u32(addr, value)?;
                    addr = addr.wrapping_add(4);
                }
            }
        }

        // Writeback
        if w && !(l && (register_list >> rn) & 1 != 0) {
            self.set_reg(rn, wb_addr);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_load_store_dual_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Various dual/exclusive load/store instructions
        // Encoding: 1110 100P UDWL nnnn tttt dddd iiiiiiii
        // op1 = PU (bits 24:23), op2 = WL (bits 21:20), op3 = bits 7:4
        let op1 = (insn >> 23) & 0x3;
        let op2 = (insn >> 20) & 0x3;
        let op3 = (insn >> 4) & 0xF;
        let rn = ((insn >> 16) & 0xF) as usize;

        // Extract common fields for LDRD/STRD
        let p = (insn >> 24) & 1 != 0;
        let u = (insn >> 23) & 1 != 0;
        let w = (insn >> 21) & 1 != 0;
        let l = (insn >> 20) & 1 != 0;

        match (op1, op2, op3) {
            (0b00 | 0b01, 0b00, _) if op3 != 0 => {
                // STREX
                let rd = ((insn >> 8) & 0xF) as usize;
                let rt = ((insn >> 12) & 0xF) as usize;
                let imm8 = (insn & 0xFF) as u32;
                let addr = self.reg(rn).wrapping_add(imm8 << 2);

                // Simplified exclusive: always succeed
                // Real implementation would check exclusive monitors
                self.mem_write_u32(addr, self.reg(rt))?;
                self.set_reg(rd, 0); // Success
            }
            (0b00 | 0b01, 0b01, _) if op3 != 0 => {
                // LDREX
                let rt = ((insn >> 12) & 0xF) as usize;
                let imm8 = (insn & 0xFF) as u32;
                let addr = self.reg(rn).wrapping_add(imm8 << 2);

                let value = self.mem_read_u32(addr)?;
                self.set_reg(rt, value);
                // Set exclusive monitor (simplified)
            }
            // STRD/LDRD: Match all op1 values (any P, U combination)
            // L=0 for store, L=1 for load (bit 20)
            (_, _, _) if !l && op3 == 0 => {
                // STRD (immediate) - L=0
                let rt = ((insn >> 12) & 0xF) as usize;
                let rt2 = ((insn >> 8) & 0xF) as usize;
                let imm8 = (insn & 0xFF) as u32;

                let offset = if u {
                    imm8 << 2
                } else {
                    (imm8 << 2).wrapping_neg()
                };
                let rn_val = self.reg(rn);
                let addr = if p {
                    rn_val.wrapping_add(offset)
                } else {
                    rn_val
                };

                self.mem_write_u32(addr, self.reg(rt))?;
                self.mem_write_u32(addr.wrapping_add(4), self.reg(rt2))?;

                if w || !p {
                    self.set_reg(rn, rn_val.wrapping_add(offset));
                }
            }
            (_, _, _) if l && op3 == 0 => {
                // LDRD (immediate) - L=1
                let rt = ((insn >> 12) & 0xF) as usize;
                let rt2 = ((insn >> 8) & 0xF) as usize;
                let imm8 = (insn & 0xFF) as u32;

                let offset = if u {
                    imm8 << 2
                } else {
                    (imm8 << 2).wrapping_neg()
                };
                let rn_val = self.reg(rn);
                let addr = if p {
                    rn_val.wrapping_add(offset)
                } else {
                    rn_val
                };

                let val1 = self.mem_read_u32(addr)?;
                let val2 = self.mem_read_u32(addr.wrapping_add(4))?;
                self.set_reg(rt, val1);
                self.set_reg(rt2, val2);

                if w || !p {
                    self.set_reg(rn, rn_val.wrapping_add(offset));
                }
            }
            (0b00, 0b00, 0b0100) => {
                // STREXB
                let rd = ((insn >> 8) & 0xF) as usize;
                let rt = ((insn >> 12) & 0xF) as usize;
                let addr = self.reg(rn);
                self.mem_write_u8(addr, self.reg(rt) as u8)?;
                self.set_reg(rd, 0);
            }
            (0b00, 0b00, 0b0101) => {
                // STREXH
                let rd = ((insn >> 8) & 0xF) as usize;
                let rt = ((insn >> 12) & 0xF) as usize;
                let addr = self.reg(rn);
                self.mem_write_u16(addr, self.reg(rt) as u16)?;
                self.set_reg(rd, 0);
            }
            (0b00, 0b01, 0b0100) => {
                // LDREXB
                let rt = ((insn >> 12) & 0xF) as usize;
                let addr = self.reg(rn);
                let value = self.mem_read_u8(addr)?;
                self.set_reg(rt, value as u32);
            }
            (0b00, 0b01, 0b0101) => {
                // LDREXH
                let rt = ((insn >> 12) & 0xF) as usize;
                let addr = self.reg(rn);
                let value = self.mem_read_u16(addr)?;
                self.set_reg(rt, value as u32);
            }
            (0b01, 0b00, 0b0111) => {
                // STREXD
                let rd = ((insn >> 8) & 0xF) as usize;
                let rt = ((insn >> 12) & 0xF) as usize;
                let rt2 = (insn & 0xF) as usize;
                let addr = self.reg(rn);
                self.mem_write_u32(addr, self.reg(rt))?;
                self.mem_write_u32(addr.wrapping_add(4), self.reg(rt2))?;
                self.set_reg(rd, 0);
            }
            (0b01, 0b01, 0b0111) => {
                // LDREXD
                let rt = ((insn >> 12) & 0xF) as usize;
                let rt2 = (insn & 0xF) as usize;
                let addr = self.reg(rn);
                let val1 = self.mem_read_u32(addr)?;
                let val2 = self.mem_read_u32(addr.wrapping_add(4))?;
                self.set_reg(rt, val1);
                self.set_reg(rt2, val2);
            }
            (0b10, _, _) | (0b11, _, _) => {
                // Table branch (TBB, TBH)
                let h = (insn >> 4) & 1 != 0;
                let rm = (insn & 0xF) as usize;
                let rn_val = self.reg(rn);
                let rm_val = self.reg(rm);

                let addr = if h {
                    rn_val.wrapping_add(rm_val << 1)
                } else {
                    rn_val.wrapping_add(rm_val)
                };

                let offset = if h {
                    self.mem_read_u16(addr)? as u32
                } else {
                    self.mem_read_u8(addr)? as u32
                };

                self.pc = self.pc.wrapping_add(offset << 1);
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_data_processing_shifted_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Encoding: 1110 101 op:4 s rn:4 | 0 imm3 rd:4 imm2 type:2 rm:4
        let op = (insn >> 21) & 0xF;
        let set_flags = (insn >> 20) & 1 != 0;
        let rn = ((insn >> 16) & 0xF) as usize;
        let rd = ((insn >> 8) & 0xF) as usize;
        let imm3 = ((insn >> 12) & 0x7) as u32;
        let imm2 = ((insn >> 6) & 0x3) as u32;
        let shift_type = ((insn >> 4) & 0x3) as u32;
        let rm = (insn & 0xF) as usize;

        let rn_val = self.reg(rn);
        let rm_val = self.reg(rm);
        let shift_n = (imm3 << 2) | imm2;

        // Apply shift to Rm
        let (shifted, shift_carry) = self.shift_c(rm_val, shift_type, shift_n);

        let (result, update_cv) = match op {
            0b0000 => {
                // AND / TST
                let r = rn_val & shifted;
                if rd == 15 && set_flags {
                    self.update_nz(r);
                    self.set_c(shift_carry);
                    return Ok(CpuExit::Continue);
                }
                (r, false)
            }
            0b0001 => {
                // BIC
                (rn_val & !shifted, false)
            }
            0b0010 => {
                // ORR / MOV (if rn == 15)
                if rn == 15 {
                    (shifted, false)
                } else {
                    (rn_val | shifted, false)
                }
            }
            0b0011 => {
                // ORN / MVN (if rn == 15)
                if rn == 15 {
                    (!shifted, false)
                } else {
                    (rn_val | !shifted, false)
                }
            }
            0b0100 => {
                // EOR / TEQ
                let r = rn_val ^ shifted;
                if rd == 15 && set_flags {
                    self.update_nz(r);
                    self.set_c(shift_carry);
                    return Ok(CpuExit::Continue);
                }
                (r, false)
            }
            0b0110 => {
                // PKH (pack halfword)
                let tbform = (insn >> 5) & 1 != 0;
                if tbform {
                    // PKHTB: Rn[31:16], shifted[15:0]
                    ((rn_val & 0xFFFF0000) | (shifted & 0x0000FFFF), false)
                } else {
                    // PKHBT: shifted[31:16], Rn[15:0]
                    ((shifted & 0xFFFF0000) | (rn_val & 0x0000FFFF), false)
                }
            }
            0b1000 => {
                // ADD / CMN
                let (r, c) = rn_val.overflowing_add(shifted);
                let v = (!(rn_val ^ shifted) & (rn_val ^ r)) >> 31 != 0;
                if rd == 15 && set_flags {
                    self.update_nz(r);
                    self.set_c(c);
                    self.set_v(v);
                    return Ok(CpuExit::Continue);
                }
                if set_flags {
                    self.set_c(c);
                    self.set_v(v);
                }
                (r, true)
            }
            0b1010 => {
                // ADC
                let c_in = if self.get_c() { 1u32 } else { 0 };
                let (r1, c1) = rn_val.overflowing_add(shifted);
                let (r, c2) = r1.overflowing_add(c_in);
                let v = (!(rn_val ^ shifted) & (rn_val ^ r)) >> 31 != 0;
                if set_flags {
                    self.set_c(c1 || c2);
                    self.set_v(v);
                }
                (r, true)
            }
            0b1011 => {
                // SBC
                let c_in = if self.get_c() { 0u32 } else { 1 };
                let (r1, b1) = rn_val.overflowing_sub(shifted);
                let (r, b2) = r1.overflowing_sub(c_in);
                let v = ((rn_val ^ shifted) & (rn_val ^ r)) >> 31 != 0;
                if set_flags {
                    self.set_c(!(b1 || b2));
                    self.set_v(v);
                }
                (r, true)
            }
            0b1101 => {
                // SUB / CMP
                let (r, b) = rn_val.overflowing_sub(shifted);
                let v = ((rn_val ^ shifted) & (rn_val ^ r)) >> 31 != 0;
                if rd == 15 && set_flags {
                    self.update_nz(r);
                    self.set_c(!b);
                    self.set_v(v);
                    return Ok(CpuExit::Continue);
                }
                if set_flags {
                    self.set_c(!b);
                    self.set_v(v);
                }
                (r, true)
            }
            0b1110 => {
                // RSB
                let (r, b) = shifted.overflowing_sub(rn_val);
                let v = ((shifted ^ rn_val) & (shifted ^ r)) >> 31 != 0;
                if set_flags {
                    self.set_c(!b);
                    self.set_v(v);
                }
                (r, true)
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };

        if set_flags && !update_cv {
            self.update_nz(result);
            self.set_c(shift_carry);
        } else if set_flags {
            self.update_nz(result);
        }

        if rd == 15 {
            self.pc = result & !1;
        } else {
            self.set_reg(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    /// Perform shift with carry out.
    fn shift_c(&self, value: u32, shift_type: u32, shift_n: u32) -> (u32, bool) {
        if shift_n == 0 {
            return (value, self.get_c());
        }

        match shift_type {
            0b00 => {
                // LSL
                if shift_n < 32 {
                    let c = (value >> (32 - shift_n)) & 1 != 0;
                    (value << shift_n, c)
                } else if shift_n == 32 {
                    (0, value & 1 != 0)
                } else {
                    (0, false)
                }
            }
            0b01 => {
                // LSR
                if shift_n < 32 {
                    let c = (value >> (shift_n - 1)) & 1 != 0;
                    (value >> shift_n, c)
                } else if shift_n == 32 {
                    (0, (value >> 31) != 0)
                } else {
                    (0, false)
                }
            }
            0b10 => {
                // ASR
                let signed = value as i32;
                if shift_n < 32 {
                    let c = (value >> (shift_n - 1)) & 1 != 0;
                    ((signed >> shift_n) as u32, c)
                } else {
                    let c = signed < 0;
                    (if c { 0xFFFFFFFF } else { 0 }, c)
                }
            }
            0b11 => {
                // ROR
                let effective = shift_n & 31;
                if effective == 0 {
                    // RRX when shift_n was 0 handled above
                    (value, (value >> 31) != 0)
                } else {
                    let r = value.rotate_right(effective);
                    (r, (r >> 31) != 0)
                }
            }
            _ => unreachable!(),
        }
    }

    fn exec_coprocessor_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Coprocessor instructions (CP10/CP11 = FPU)
        let coproc = ((insn >> 8) & 0xF) as u8;

        match coproc {
            10 | 11 => self.exec_vfp_32(insn),
            _ => Err(ArmError::Unimplemented(format!(
                "Coprocessor {} not implemented",
                coproc
            ))),
        }
    }

    fn exec_vfp_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        if self.vfp.is_none() {
            return Err(ArmError::Unimplemented("FPU not available".to_string()));
        }

        let op1 = (insn >> 20) & 0x3F;
        let op = (insn >> 4) & 0xF;

        if (op1 & 0x30) == 0x20 && (op & 0x1) == 0 {
            // VFP data processing
            let d = ((insn >> 22) & 1) as u8;
            let vn = ((insn >> 16) & 0xF) as u8;
            let vd = ((insn >> 12) & 0xF) as u8;
            let sz = (insn >> 8) & 1 != 0; // 1 = double, 0 = single
            let n = ((insn >> 7) & 1) as u8;
            let m = ((insn >> 5) & 1) as u8;
            let vm = (insn & 0xF) as u8;
            let opc1 = (insn >> 20) & 0xF;
            let opc2 = (insn >> 16) & 0xF;
            let opc3 = (insn >> 6) & 0x3;

            let vfp = self.vfp.as_mut().unwrap();

            if sz {
                // Double precision
                let d_reg = (d << 4) | vd;
                let n_reg = (n << 4) | vn;
                let m_reg = (m << 4) | vm;

                let vn_val = vfp.read_d(n_reg);
                let vm_val = vfp.read_d(m_reg);

                let result = match (opc1, opc2, opc3) {
                    (0b0000, _, 0b00) => {
                        // VMLA.F64
                        let vd_val = vfp.read_d(d_reg);
                        vd_val + vn_val * vm_val
                    }
                    (0b0000, _, 0b01) => {
                        // VMLS.F64
                        let vd_val = vfp.read_d(d_reg);
                        vd_val - vn_val * vm_val
                    }
                    (0b0001, _, 0b01) => {
                        // VNMLS.F64
                        let vd_val = vfp.read_d(d_reg);
                        -(vd_val - vn_val * vm_val)
                    }
                    (0b0010, _, 0b00) => vn_val * vm_val, // VMUL.F64
                    (0b0011, _, 0b00) => vn_val + vm_val, // VADD.F64
                    (0b0011, _, 0b01) => vn_val - vm_val, // VSUB.F64
                    (0b1000, _, 0b00) => vn_val / vm_val, // VDIV.F64
                    _ => {
                        return Err(ArmError::Unimplemented(format!(
                            "VFP64 op {:04b} {:04b} {:02b}",
                            opc1, opc2, opc3
                        )));
                    }
                };

                vfp.write_d(d_reg, result);
            } else {
                // Single precision
                let d_reg = (vd << 1) | d;
                let n_reg = (vn << 1) | n;
                let m_reg = (vm << 1) | m;

                let vn_val = vfp.read_s(n_reg);
                let vm_val = vfp.read_s(m_reg);

                let result = match (opc1, opc2, opc3) {
                    (0b0000, _, 0b00) => {
                        // VMLA.F32
                        let vd_val = vfp.read_s(d_reg);
                        vd_val + vn_val * vm_val
                    }
                    (0b0000, _, 0b01) => {
                        // VMLS.F32
                        let vd_val = vfp.read_s(d_reg);
                        vd_val - vn_val * vm_val
                    }
                    (0b0010, _, 0b00) => vn_val * vm_val, // VMUL.F32
                    (0b0011, _, 0b00) => vn_val + vm_val, // VADD.F32
                    (0b0011, _, 0b01) => vn_val - vm_val, // VSUB.F32
                    (0b1000, _, 0b00) => vn_val / vm_val, // VDIV.F32
                    (0b1011, 0b0000, 0b01) => vm_val.sqrt(), // VSQRT.F32
                    (0b1011, 0b0000, 0b00) => vm_val,     // VMOV (reg)
                    (0b1011, 0b0001, 0b00) => vm_val.abs(), // VABS.F32
                    (0b1011, 0b0001, 0b01) => -vm_val,    // VNEG.F32
                    _ => {
                        return Err(ArmError::Unimplemented(format!(
                            "VFP32 op {:04b} {:04b} {:02b}",
                            opc1, opc2, opc3
                        )));
                    }
                };

                vfp.write_s(d_reg, result);
            }

            return Ok(CpuExit::Continue);
        }

        if (op1 & 0x38) == 0x10 {
            // VLDR/VSTR
            let u = (insn >> 23) & 1 != 0;
            let d = ((insn >> 22) & 1) as u8;
            let rn = ((insn >> 16) & 0xF) as usize;
            let vd = ((insn >> 12) & 0xF) as u8;
            let sz = (insn >> 8) & 1 != 0;
            let imm8 = (insn & 0xFF) as u32;

            let offset = if u {
                imm8 << 2
            } else {
                (imm8 << 2).wrapping_neg()
            };
            let addr = self.reg(rn).wrapping_add(offset);

            if (op1 & 0x01) != 0 {
                // VLDR
                if sz {
                    let d_reg = (d << 4) | vd;
                    let low = self.mem_read_u32(addr)?;
                    let high = self.mem_read_u32(addr.wrapping_add(4))?;
                    let value = f64::from_bits(((high as u64) << 32) | (low as u64));
                    let vfp = self.vfp.as_mut().unwrap();
                    vfp.write_d(d_reg, value);
                } else {
                    let s_reg = (vd << 1) | d;
                    let bits = self.mem_read_u32(addr)?;
                    let value = f32::from_bits(bits);
                    let vfp = self.vfp.as_mut().unwrap();
                    vfp.write_s(s_reg, value);
                }
            } else {
                // VSTR
                if sz {
                    let d_reg = (d << 4) | vd;
                    let vfp = self.vfp.as_ref().unwrap();
                    let value = vfp.read_d(d_reg).to_bits();
                    self.mem_write_u32(addr, value as u32)?;
                    self.mem_write_u32(addr.wrapping_add(4), (value >> 32) as u32)?;
                } else {
                    let s_reg = (vd << 1) | d;
                    let vfp = self.vfp.as_ref().unwrap();
                    let value = vfp.read_s(s_reg).to_bits();
                    self.mem_write_u32(addr, value)?;
                }
            }

            return Ok(CpuExit::Continue);
        }

        if (op1 & 0x20) == 0 && (op & 0x01) == 0x01 {
            // VMOV (between ARM core register and single-precision)
            let vmov_op = (insn >> 20) & 1;
            let vn = ((insn >> 16) & 0xF) as u8;
            let rt = ((insn >> 12) & 0xF) as usize;
            let n = ((insn >> 7) & 1) as u8;
            let s_reg = (vn << 1) | n;

            if vmov_op != 0 {
                // VMOV Rt, Sn
                let vfp = self.vfp.as_ref().unwrap();
                let value = vfp.read_s(s_reg).to_bits();
                self.set_reg(rt, value);
            } else {
                // VMOV Sn, Rt
                let rt_val = self.reg(rt);
                let value = f32::from_bits(rt_val);
                let vfp = self.vfp.as_mut().unwrap();
                vfp.write_s(s_reg, value);
            }

            return Ok(CpuExit::Continue);
        }

        Err(ArmError::Unimplemented(format!(
            "VFP instruction {:08X}",
            insn
        )))
    }

    fn exec_data_processing_imm_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Encoding: 1111 0x0 op:4 s rn:4 | 0 imm3 rd:4 imm8
        let op = (insn >> 21) & 0xF;
        let set_flags = (insn >> 20) & 1 != 0;
        let rn = ((insn >> 16) & 0xF) as usize;
        let rd = ((insn >> 8) & 0xF) as usize;

        let rn_val = self.reg(rn);
        let (imm32, carry_out) = self.thumb_expand_imm_c(insn);

        let (result, update_cv) = match op {
            0b0000 => {
                // AND / TST (if rd == 15 && s)
                let r = rn_val & imm32;
                if rd == 15 && set_flags {
                    // TST - only update flags
                    self.update_nz(r);
                    self.set_c(carry_out);
                    return Ok(CpuExit::Continue);
                }
                (r, false)
            }
            0b0001 => {
                // BIC
                (rn_val & !imm32, false)
            }
            0b0010 => {
                // ORR / MOV (if rn == 15)
                let r = if rn == 15 { imm32 } else { rn_val | imm32 };
                (r, false)
            }
            0b0011 => {
                // ORN / MVN (if rn == 15)
                let r = if rn == 15 { !imm32 } else { rn_val | !imm32 };
                (r, false)
            }
            0b0100 => {
                // EOR / TEQ (if rd == 15 && s)
                let r = rn_val ^ imm32;
                if rd == 15 && set_flags {
                    // TEQ - only update flags
                    self.update_nz(r);
                    self.set_c(carry_out);
                    return Ok(CpuExit::Continue);
                }
                (r, false)
            }
            0b1000 => {
                // ADD / CMN (if rd == 15 && s)
                let (r, c) = rn_val.overflowing_add(imm32);
                let v = (!(rn_val ^ imm32) & (rn_val ^ r)) >> 31 != 0;
                if rd == 15 && set_flags {
                    // CMN - only update flags
                    self.update_nz(r);
                    self.set_c(c);
                    self.set_v(v);
                    return Ok(CpuExit::Continue);
                }
                if set_flags {
                    self.set_c(c);
                    self.set_v(v);
                }
                (r, true)
            }
            0b1010 => {
                // ADC
                let c_in = if self.get_c() { 1u32 } else { 0 };
                let (r1, c1) = rn_val.overflowing_add(imm32);
                let (r, c2) = r1.overflowing_add(c_in);
                let v = (!(rn_val ^ imm32) & (rn_val ^ r)) >> 31 != 0;
                if set_flags {
                    self.set_c(c1 || c2);
                    self.set_v(v);
                }
                (r, true)
            }
            0b1011 => {
                // SBC
                let c_in = if self.get_c() { 0u32 } else { 1 };
                let (r1, b1) = rn_val.overflowing_sub(imm32);
                let (r, b2) = r1.overflowing_sub(c_in);
                let v = ((rn_val ^ imm32) & (rn_val ^ r)) >> 31 != 0;
                if set_flags {
                    self.set_c(!(b1 || b2));
                    self.set_v(v);
                }
                (r, true)
            }
            0b1101 => {
                // SUB / CMP (if rd == 15 && s)
                let (r, b) = rn_val.overflowing_sub(imm32);
                let v = ((rn_val ^ imm32) & (rn_val ^ r)) >> 31 != 0;
                if rd == 15 && set_flags {
                    // CMP - only update flags
                    self.update_nz(r);
                    self.set_c(!b);
                    self.set_v(v);
                    return Ok(CpuExit::Continue);
                }
                if set_flags {
                    self.set_c(!b);
                    self.set_v(v);
                }
                (r, true)
            }
            0b1110 => {
                // RSB
                let (r, b) = imm32.overflowing_sub(rn_val);
                let v = ((imm32 ^ rn_val) & (imm32 ^ r)) >> 31 != 0;
                if set_flags {
                    self.set_c(!b);
                    self.set_v(v);
                }
                (r, true)
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };

        if set_flags && !update_cv {
            self.update_nz(result);
            self.set_c(carry_out);
        } else if set_flags {
            self.update_nz(result);
        }

        if rd == 15 {
            self.pc = result & !1;
        } else {
            self.set_reg(rd, result);
        }

        Ok(CpuExit::Continue)
    }

    /// Expand a Thumb-2 modified immediate constant with carry out.
    fn thumb_expand_imm_c(&self, insn: u32) -> (u32, bool) {
        let i = ((insn >> 26) & 1) as u32;
        let imm3 = ((insn >> 12) & 0x7) as u32;
        let imm8 = (insn & 0xFF) as u32;
        let imm12 = (i << 11) | (imm3 << 8) | imm8;

        if (imm12 >> 10) == 0 {
            // Unrotated value
            let value = match (imm12 >> 8) & 0x3 {
                0b00 => imm8,
                0b01 => (imm8 << 16) | imm8,
                0b10 => (imm8 << 24) | (imm8 << 8),
                0b11 => (imm8 << 24) | (imm8 << 16) | (imm8 << 8) | imm8,
                _ => unreachable!(),
            };
            (value, self.get_c())
        } else {
            // Rotated value: 1ccccccc rotated right by imm12[11:7]
            let unrotated = 0x80 | (imm12 & 0x7F);
            let rotation = (imm12 >> 7) & 0x1F;
            let value = unrotated.rotate_right(rotation);
            let carry_out = (value >> 31) != 0;
            (value, carry_out)
        }
    }

    fn exec_branches_misc_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        let op1 = (insn >> 20) & 0x7F;
        let op2 = (insn >> 12) & 0x7;

        if op2 & 0x5 == 0 {
            // Conditional branch or misc
            if op1 & 0x38 != 0x38 {
                // B<cond>.W
                let cond = ((insn >> 22) & 0xF) as u8;
                if self.condition_passed(cond) {
                    let s = ((insn >> 26) & 1) as i32;
                    let j1 = ((insn >> 13) & 1) as i32;
                    let j2 = ((insn >> 11) & 1) as i32;
                    let imm6 = ((insn >> 16) & 0x3F) as i32;
                    let imm11 = (insn & 0x7FF) as i32;

                    let offset = (s << 20)
                        | ((j2 ^ !s) << 19)
                        | ((j1 ^ !s) << 18)
                        | (imm6 << 12)
                        | (imm11 << 1);
                    let offset = (offset << 11) >> 11; // Sign extend
                    self.pc = (self.pc as i32).wrapping_add(offset) as u32;
                }
                return Ok(CpuExit::Continue);
            }
        }

        if op2 & 0x5 == 0x1 {
            // B.W
            let s = ((insn >> 26) & 1) as i32;
            let j1 = ((insn >> 13) & 1) as i32;
            let j2 = ((insn >> 11) & 1) as i32;
            let imm10 = ((insn >> 16) & 0x3FF) as i32;
            let imm11 = (insn & 0x7FF) as i32;

            let i1 = !(j1 ^ s) & 1;
            let i2 = !(j2 ^ s) & 1;
            let offset = (s << 24) | (i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1);
            let offset = (offset << 7) >> 7; // Sign extend from 25 bits
            self.pc = (self.pc as i32).wrapping_add(offset) as u32;
            return Ok(CpuExit::Continue);
        }

        if op2 & 0x5 == 0x5 {
            // BL
            let s = ((insn >> 26) & 1) as i32;
            let j1 = ((insn >> 13) & 1) as i32;
            let j2 = ((insn >> 11) & 1) as i32;
            let imm10 = ((insn >> 16) & 0x3FF) as i32;
            let imm11 = (insn & 0x7FF) as i32;

            let i1 = !(j1 ^ s) & 1;
            let i2 = !(j2 ^ s) & 1;
            let offset = (s << 24) | (i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1);
            let offset = (offset << 7) >> 7;

            self.lr = self.pc | 1;
            self.pc = (self.pc as i32).wrapping_add(offset) as u32;
            return Ok(CpuExit::Continue);
        }

        Err(ArmError::Unimplemented("32-bit branch/misc".to_string()))
    }

    fn exec_store_single_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Encoding: 1111 1000 size:2 0 rn:4 | rt:4 imm12/mode
        let size = (insn >> 21) & 0x3;
        let rn = ((insn >> 16) & 0xF) as usize;
        let rt = ((insn >> 12) & 0xF) as usize;
        let rt_val = self.reg(rt);

        let (addr, writeback) = self.compute_addr_32(insn, rn)?;

        match size {
            0b00 => self.mem_write_u8(addr, rt_val as u8)?, // STRB
            0b01 => self.mem_write_u16(addr, rt_val as u16)?, // STRH
            0b10 => self.mem_write_u32(addr, rt_val)?,      // STR
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        }

        if let Some(new_rn) = writeback {
            self.set_reg(rn, new_rn);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_load_byte_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // LDRB, LDRSB variants
        let rn = ((insn >> 16) & 0xF) as usize;
        let rt = ((insn >> 12) & 0xF) as usize;
        let sign_extend = (insn >> 24) & 1 != 0;

        let (addr, writeback) = self.compute_addr_32(insn, rn)?;
        let value = self.mem_read_u8(addr)?;

        self.set_reg(
            rt,
            if sign_extend {
                value as i8 as i32 as u32
            } else {
                value as u32
            },
        );

        if let Some(new_rn) = writeback {
            self.set_reg(rn, new_rn);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_load_halfword_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // LDRH, LDRSH variants
        let rn = ((insn >> 16) & 0xF) as usize;
        let rt = ((insn >> 12) & 0xF) as usize;
        let sign_extend = (insn >> 24) & 1 != 0;

        let (addr, writeback) = self.compute_addr_32(insn, rn)?;
        let value = self.mem_read_u16(addr)?;

        self.set_reg(
            rt,
            if sign_extend {
                value as i16 as i32 as u32
            } else {
                value as u32
            },
        );

        if let Some(new_rn) = writeback {
            self.set_reg(rn, new_rn);
        }

        Ok(CpuExit::Continue)
    }

    fn exec_load_word_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // LDR variants
        let rn = ((insn >> 16) & 0xF) as usize;
        let rt = ((insn >> 12) & 0xF) as usize;

        let (addr, writeback) = self.compute_addr_32(insn, rn)?;
        let value = self.mem_read_u32(addr)?;

        if rt == 15 {
            // Loading to PC - interworking branch
            self.pc = value & !1;
        } else {
            self.set_reg(rt, value);
        }

        if let Some(new_rn) = writeback {
            self.set_reg(rn, new_rn);
        }

        Ok(CpuExit::Continue)
    }

    /// Compute address for 32-bit load/store instructions.
    /// Returns (address, optional writeback value for Rn).
    fn compute_addr_32(&self, insn: u32, rn: usize) -> Result<(u32, Option<u32>), ArmError> {
        let rn_val = self.reg(rn);

        // Check if this is register offset or immediate
        if (insn >> 23) & 1 != 0 {
            // Immediate offset (positive, 12-bit)
            let imm12 = (insn & 0xFFF) as u32;
            Ok((rn_val.wrapping_add(imm12), None))
        } else {
            let op2 = (insn >> 8) & 0xF;

            if (insn >> 11) & 1 != 0 {
                // Immediate offset variants (8-bit)
                let imm8 = (insn & 0xFF) as u32;
                let u = (insn >> 9) & 1 != 0; // Add/subtract
                let p = (insn >> 10) & 1 != 0; // Pre/post index
                let w = (insn >> 8) & 1 != 0; // Writeback

                let offset = if u { imm8 } else { imm8.wrapping_neg() };

                if p {
                    // Pre-indexed
                    let addr = rn_val.wrapping_add(offset);
                    let wb = if w { Some(addr) } else { None };
                    Ok((addr, wb))
                } else {
                    // Post-indexed
                    Ok((rn_val, Some(rn_val.wrapping_add(offset))))
                }
            } else if op2 == 0 {
                // Register offset: [Rn, Rm, LSL #imm2]
                let rm = (insn & 0xF) as usize;
                let imm2 = ((insn >> 4) & 0x3) as u32;
                let offset = self.reg(rm) << imm2;
                Ok((rn_val.wrapping_add(offset), None))
            } else {
                Err(ArmError::UndefinedInstruction(insn))
            }
        }
    }

    fn exec_data_processing_reg_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Various register-based data processing
        let op1 = (insn >> 20) & 0xF;
        let rn = ((insn >> 16) & 0xF) as usize;
        let op2 = (insn >> 4) & 0xF;

        match op1 {
            0b0000 if op2 == 0 => {
                // LSL (register)
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let rn_val = self.reg(rn);
                let shift = self.reg(rm) & 0xFF;
                let set_flags = (insn >> 20) & 1 != 0;

                let (result, carry) = if shift == 0 {
                    (rn_val, self.get_c())
                } else if shift < 32 {
                    let c = (rn_val >> (32 - shift)) & 1 != 0;
                    (rn_val << shift, c)
                } else if shift == 32 {
                    (0, rn_val & 1 != 0)
                } else {
                    (0, false)
                };

                self.set_reg(rd, result);
                if set_flags {
                    self.update_nz(result);
                    self.set_c(carry);
                }
            }
            0b0001 if op2 == 0 => {
                // LSR (register)
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let rn_val = self.reg(rn);
                let shift = self.reg(rm) & 0xFF;
                let set_flags = (insn >> 20) & 1 != 0;

                let (result, carry) = if shift == 0 {
                    (rn_val, self.get_c())
                } else if shift < 32 {
                    let c = (rn_val >> (shift - 1)) & 1 != 0;
                    (rn_val >> shift, c)
                } else if shift == 32 {
                    (0, (rn_val >> 31) != 0)
                } else {
                    (0, false)
                };

                self.set_reg(rd, result);
                if set_flags {
                    self.update_nz(result);
                    self.set_c(carry);
                }
            }
            0b0010 if op2 == 0 => {
                // ASR (register)
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let rn_val = self.reg(rn) as i32;
                let shift = self.reg(rm) & 0xFF;
                let set_flags = (insn >> 20) & 1 != 0;

                let (result, carry) = if shift == 0 {
                    (rn_val as u32, self.get_c())
                } else if shift < 32 {
                    let c = ((rn_val >> (shift - 1)) & 1) != 0;
                    ((rn_val >> shift) as u32, c)
                } else {
                    let c = rn_val < 0;
                    (if c { 0xFFFFFFFF } else { 0 }, c)
                };

                self.set_reg(rd, result);
                if set_flags {
                    self.update_nz(result);
                    self.set_c(carry);
                }
            }
            0b0011 if op2 == 0 => {
                // ROR (register)
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let rn_val = self.reg(rn);
                let shift = self.reg(rm) & 0xFF;
                let set_flags = (insn >> 20) & 1 != 0;

                let (result, carry) = if shift == 0 {
                    (rn_val, self.get_c())
                } else {
                    let effective = shift & 31;
                    if effective == 0 {
                        (rn_val, (rn_val >> 31) != 0)
                    } else {
                        let r = rn_val.rotate_right(effective);
                        (r, (r >> 31) != 0)
                    }
                };

                self.set_reg(rd, result);
                if set_flags {
                    self.update_nz(result);
                    self.set_c(carry);
                }
            }
            0b0000..=0b0011 if (op2 >> 3) == 1 => {
                // SXTH, SXTB, UXTH, UXTB (with rotation)
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let rotation = ((insn >> 4) & 0x3) * 8;
                let rm_val = self.reg(rm).rotate_right(rotation);

                let result = match op1 {
                    0b0000 => {
                        // SXTH / SXTAH
                        let ext = (rm_val as i16 as i32) as u32;
                        if rn != 15 {
                            self.reg(rn).wrapping_add(ext)
                        } else {
                            ext
                        }
                    }
                    0b0001 => {
                        // UXTH / UXTAH
                        let ext = rm_val & 0xFFFF;
                        if rn != 15 {
                            self.reg(rn).wrapping_add(ext)
                        } else {
                            ext
                        }
                    }
                    0b0010 => {
                        // SXTB / SXTAB
                        let ext = (rm_val as i8 as i32) as u32;
                        if rn != 15 {
                            self.reg(rn).wrapping_add(ext)
                        } else {
                            ext
                        }
                    }
                    0b0011 => {
                        // UXTB / UXTAB
                        let ext = rm_val & 0xFF;
                        if rn != 15 {
                            self.reg(rn).wrapping_add(ext)
                        } else {
                            ext
                        }
                    }
                    _ => unreachable!(),
                };

                self.set_reg(rd, result);
            }
            0b1000 if op2 == 0b0100 => {
                // REV
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                self.set_reg(rd, self.reg(rm).swap_bytes());
            }
            0b1000 if op2 == 0b1000 => {
                // REV16
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let v = self.reg(rm);
                let result = ((v >> 8) & 0x00FF00FF) | ((v << 8) & 0xFF00FF00);
                self.set_reg(rd, result);
            }
            0b1001 if op2 == 0b1010 => {
                // RBIT: 1111 1010 1001 Rn | 1111 Rd 1010 Rm
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                self.set_reg(rd, self.reg(rm).reverse_bits());
            }
            0b1001 if op2 == 0b1000 => {
                // REVSH
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let v = self.reg(rm) as u16;
                let swapped = ((v >> 8) | (v << 8)) as i16 as i32 as u32;
                self.set_reg(rd, swapped);
            }
            0b1010 if op2 == 0b0000 => {
                // SEL (packed byte select based on GE flags)
                // Note: GE flags are set by SIMD instructions not yet implemented
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                let rn_val = self.reg(rn);
                let rm_val = self.reg(rm);
                // Without GE flags, default to selecting from Rn
                // TODO: Implement GE flags properly
                self.set_reg(rd, rn_val & 0xFF00FF00 | rm_val & 0x00FF00FF);
            }
            0b1011 if op2 == 0b1000 => {
                // CLZ: 1111 1010 1011 Rn | 1111 Rd 1000 Rm
                let rd = ((insn >> 8) & 0xF) as usize;
                let rm = (insn & 0xF) as usize;
                self.set_reg(rd, self.reg(rm).leading_zeros());
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        }

        Ok(CpuExit::Continue)
    }

    fn exec_multiply_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Encoding: 1111 1011 0 op1:3 rn:4 | ra:4 rd:4 op2:2 rm:4
        let op1 = (insn >> 20) & 0x7;
        let rn = ((insn >> 16) & 0xF) as usize;
        let ra = ((insn >> 12) & 0xF) as usize;
        let rd = ((insn >> 8) & 0xF) as usize;
        let op2 = (insn >> 4) & 0x3;
        let rm = (insn & 0xF) as usize;

        let rn_val = self.reg(rn);
        let rm_val = self.reg(rm);
        let ra_val = self.reg(ra);

        let result = match (op1, op2) {
            (0b000, 0b00) => {
                // MUL (ra == 15) or MLA
                let product = rn_val.wrapping_mul(rm_val);
                if ra == 15 {
                    product
                } else {
                    product.wrapping_add(ra_val)
                }
            }
            (0b000, 0b01) => {
                // MLS
                ra_val.wrapping_sub(rn_val.wrapping_mul(rm_val))
            }
            (0b001, _) => {
                // SMLA* - signed multiply accumulate (half)
                let n = if op2 & 0x2 != 0 {
                    (rn_val >> 16) as i16 as i32
                } else {
                    rn_val as i16 as i32
                };
                let m = if op2 & 0x1 != 0 {
                    (rm_val >> 16) as i16 as i32
                } else {
                    rm_val as i16 as i32
                };
                let product = n.wrapping_mul(m);
                (ra_val as i32).wrapping_add(product) as u32
            }
            (0b010, 0b00 | 0b01) => {
                // SMUAD (op2=0) / SMUSD (op2=1)
                let n_lo = rn_val as i16 as i32;
                let n_hi = (rn_val >> 16) as i16 as i32;
                let m_lo = rm_val as i16 as i32;
                let m_hi = (rm_val >> 16) as i16 as i32;

                if op2 == 0 {
                    // SMUAD: (Rn[15:0] * Rm[15:0]) + (Rn[31:16] * Rm[31:16])
                    (n_lo
                        .wrapping_mul(m_lo)
                        .wrapping_add(n_hi.wrapping_mul(m_hi))) as u32
                } else {
                    // SMUSD: (Rn[15:0] * Rm[15:0]) - (Rn[31:16] * Rm[31:16])
                    (n_lo
                        .wrapping_mul(m_lo)
                        .wrapping_sub(n_hi.wrapping_mul(m_hi))) as u32
                }
            }
            (0b011, 0b00) => {
                // SMUL* - signed multiply (half)
                let n = if (insn >> 5) & 1 != 0 {
                    (rn_val >> 16) as i16 as i32
                } else {
                    rn_val as i16 as i32
                };
                let m = if (insn >> 4) & 1 != 0 {
                    (rm_val >> 16) as i16 as i32
                } else {
                    rm_val as i16 as i32
                };
                n.wrapping_mul(m) as u32
            }
            (0b100, 0b00) => {
                // SDIV
                if rm_val == 0 {
                    0 // Division by zero returns 0 on Cortex-M
                } else {
                    (rn_val as i32).wrapping_div(rm_val as i32) as u32
                }
            }
            (0b101, 0b00) => {
                // UDIV
                if rm_val == 0 {
                    0
                } else {
                    rn_val.wrapping_div(rm_val)
                }
            }
            (0b110, _) => {
                // SMLAD/SMLSD - signed multiply accumulate dual
                let n_lo = rn_val as i16 as i32;
                let n_hi = (rn_val >> 16) as i16 as i32;
                let m_lo = rm_val as i16 as i32;
                let m_hi = (rm_val >> 16) as i16 as i32;

                let product = if op2 & 1 == 0 {
                    n_lo.wrapping_mul(m_lo)
                        .wrapping_add(n_hi.wrapping_mul(m_hi))
                } else {
                    n_lo.wrapping_mul(m_lo)
                        .wrapping_sub(n_hi.wrapping_mul(m_hi))
                };
                (ra_val as i32).wrapping_add(product) as u32
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        };

        self.set_reg(rd, result);
        Ok(CpuExit::Continue)
    }

    fn exec_long_multiply_32(&mut self, insn: u32) -> Result<CpuExit, ArmError> {
        // Encoding: 1111 1011 1 op1:3 rn:4 | rdlo:4 rdhi:4 op2:4 rm:4
        let op1 = (insn >> 20) & 0x7;
        let rn = ((insn >> 16) & 0xF) as usize;
        let rd_lo = ((insn >> 12) & 0xF) as usize;
        let rd_hi = ((insn >> 8) & 0xF) as usize;
        let op2 = (insn >> 4) & 0xF;
        let rm = (insn & 0xF) as usize;

        let rn_val = self.reg(rn);
        let rm_val = self.reg(rm);

        match (op1, op2) {
            (0b000, 0b0000) => {
                // SMULL
                let result = (rn_val as i32 as i64).wrapping_mul(rm_val as i32 as i64) as u64;
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b000, 0b1111) => {
                // SMLAL
                let rd_lo_val = self.reg(rd_lo) as u64;
                let rd_hi_val = self.reg(rd_hi) as u64;
                let acc = (rd_hi_val << 32) | rd_lo_val;
                let product = (rn_val as i32 as i64).wrapping_mul(rm_val as i32 as i64) as u64;
                let result = acc.wrapping_add(product);
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b010, 0b0000) => {
                // UMULL
                let result = (rn_val as u64).wrapping_mul(rm_val as u64);
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b010, 0b1111) => {
                // UMLAL
                let rd_lo_val = self.reg(rd_lo) as u64;
                let rd_hi_val = self.reg(rd_hi) as u64;
                let acc = (rd_hi_val << 32) | rd_lo_val;
                let product = (rn_val as u64).wrapping_mul(rm_val as u64);
                let result = acc.wrapping_add(product);
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b100, 0b0000) => {
                // SMLAL* (half * half)
                let n = if (insn >> 5) & 1 != 0 {
                    (rn_val >> 16) as i16 as i64
                } else {
                    rn_val as i16 as i64
                };
                let m = if (insn >> 4) & 1 != 0 {
                    (rm_val >> 16) as i16 as i64
                } else {
                    rm_val as i16 as i64
                };
                let rd_lo_val = self.reg(rd_lo) as u64;
                let rd_hi_val = self.reg(rd_hi) as u64;
                let acc = ((rd_hi_val << 32) | rd_lo_val) as i64;
                let result = acc.wrapping_add(n.wrapping_mul(m)) as u64;
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b110, 0b0000 | 0b0001) => {
                // SMLALD / SMLSLD
                let n_lo = rn_val as i16 as i64;
                let n_hi = (rn_val >> 16) as i16 as i64;
                let m_lo = rm_val as i16 as i64;
                let m_hi = (rm_val >> 16) as i16 as i64;

                let product = if op2 == 0 {
                    n_lo.wrapping_mul(m_lo)
                        .wrapping_add(n_hi.wrapping_mul(m_hi))
                } else {
                    n_lo.wrapping_mul(m_lo)
                        .wrapping_sub(n_hi.wrapping_mul(m_hi))
                };

                let rd_lo_val = self.reg(rd_lo) as u64;
                let rd_hi_val = self.reg(rd_hi) as u64;
                let acc = ((rd_hi_val << 32) | rd_lo_val) as i64;
                let result = acc.wrapping_add(product) as u64;
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b110, 0b1100) => {
                // UMAAL
                let rd_lo_val = self.reg(rd_lo) as u64;
                let rd_hi_val = self.reg(rd_hi) as u64;
                let product = (rn_val as u64).wrapping_mul(rm_val as u64);
                let result = product.wrapping_add(rd_lo_val).wrapping_add(rd_hi_val);
                self.set_reg(rd_lo, result as u32);
                self.set_reg(rd_hi, (result >> 32) as u32);
            }
            (0b001, 0b1111) => {
                // SDIV: 1111 1011 1001 Rn | 1111 Rd 1111 Rm
                // In this encoding: bits 15:12 = 0xF, bits 11:8 = Rd, bits 7:4 = 0xF, bits 3:0 = Rm
                // So Rd is at position rd_hi (bits 11:8)
                let rd = rd_hi;
                if rm_val == 0 {
                    self.set_reg(rd, 0); // Division by zero returns 0
                } else {
                    let result = (rn_val as i32).wrapping_div(rm_val as i32) as u32;
                    self.set_reg(rd, result);
                }
            }
            (0b011, 0b1111) => {
                // UDIV: 1111 1011 1011 Rn | 1111 Rd 1111 Rm
                let rd = rd_hi;
                if rm_val == 0 {
                    self.set_reg(rd, 0);
                } else {
                    self.set_reg(rd, rn_val.wrapping_div(rm_val));
                }
            }
            _ => return Err(ArmError::UndefinedInstruction(insn)),
        }

        Ok(CpuExit::Continue)
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Check if condition is passed.
    fn condition_passed(&self, cond: u8) -> bool {
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

    /// Get register value (handling PC and SP specially).
    fn reg(&self, reg: usize) -> u32 {
        match reg {
            0..=12 => self.regs[reg],
            13 => self.current_sp(),
            14 => self.lr,
            15 => self.pc,
            _ => 0,
        }
    }

    /// Set register value.
    fn set_reg(&mut self, reg: usize, value: u32) {
        match reg {
            0..=12 => self.regs[reg] = value,
            13 => self.set_current_sp(value),
            14 => self.lr = value,
            15 => self.pc = value & !1,
            _ => {}
        }
    }
}

impl Debug for CortexMCpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CortexMCpu")
            .field("variant", &self.variant)
            .field("pc", &format_args!("0x{:08x}", self.pc))
            .field("sp", &format_args!("0x{:08x}", self.current_sp()))
            .field("lr", &format_args!("0x{:08x}", self.lr))
            .field("xpsr", &format_args!("0x{:08x}", self.xpsr))
            .field("insn_count", &self.insn_count)
            .finish()
    }
}

impl ArmCpu for CortexMCpu {
    fn step(&mut self) -> Result<CpuExit, ArmError> {
        if self.halted {
            return Ok(CpuExit::Halt);
        }

        if self.sleeping {
            // Check for wake-up events
            if let Some(exit) = self.check_pending_exceptions()? {
                self.sleeping = false;
                return Ok(exit);
            }
            return Ok(CpuExit::Wfi);
        }

        // Check for pending exceptions
        if let Some(exit) = self.check_pending_exceptions()? {
            return Ok(exit);
        }

        // Advance SysTick
        self.systick.tick(1);

        // Execute one instruction
        self.execute_instruction()
    }

    fn reset(&mut self) {
        // Reset registers
        self.regs = [0; 13];
        self.lr = 0xFFFF_FFFF;
        self.xpsr = 0x0100_0000; // T bit set

        // Reset special registers
        self.primask = false;
        self.faultmask = false;
        self.basepri = 0;
        self.control = 0;
        self.thread_mode = true;
        self.current_exception = 0;

        // Reset peripherals
        self.nvic.reset();
        self.scb.reset();
        self.systick.reset();

        // Load initial SP and PC from vector table
        if let Ok(sp) = self.mem_read_u32(0) {
            self.sp_main = sp;
        }
        if let Ok(pc) = self.mem_read_u32(4) {
            self.pc = pc & !1;
        }

        // Clear state
        self.insn_count = 0;
        self.cycle_count = 0;
        self.halted = false;
        self.sleeping = false;
        self.pending_exceptions.clear();
    }

    fn get_gpr(&self, reg: u8) -> u64 {
        self.reg(reg as usize) as u64
    }

    fn set_gpr(&mut self, reg: u8, value: u64) {
        self.set_reg(reg as usize, value as u32);
    }

    fn get_pc(&self) -> u64 {
        self.pc as u64
    }

    fn set_pc(&mut self, value: u64) {
        self.pc = (value as u32) & !1;
    }

    fn get_sp(&self) -> u64 {
        self.current_sp() as u64
    }

    fn set_sp(&mut self, value: u64) {
        self.set_current_sp(value as u32);
    }

    fn get_lr(&self) -> u64 {
        self.lr as u64
    }

    fn set_lr(&mut self, value: u64) {
        self.lr = value as u32;
    }

    fn get_pstate(&self) -> ProcessorState {
        ProcessorState {
            n: self.get_n(),
            z: self.get_z(),
            c: self.get_c(),
            v: self.get_v(),
            q: self.get_q(),
            ge: ((self.xpsr >> 16) & 0xF) as u8,
            t: true, // Always in Thumb mode
            i: self.primask,
            f: self.faultmask,
            mode: if self.thread_mode { 0x10 } else { 0x1F },
            ..Default::default()
        }
    }

    fn set_pstate(&mut self, state: ProcessorState) {
        self.set_n(state.n);
        self.set_z(state.z);
        self.set_c(state.c);
        self.set_v(state.v);
        if state.q {
            self.set_q();
        }
        self.primask = state.i;
        self.faultmask = state.f;
    }

    fn is_privileged(&self) -> bool {
        self.is_privileged_mode()
    }

    fn current_el(&self) -> u8 {
        if self.thread_mode { 0 } else { 1 }
    }

    fn read_memory(&self, addr: u64, size: usize) -> Result<Vec<u8>, ArmError> {
        let mut buf = vec![0u8; size];
        self.memory.read(addr, &mut buf).map_err(ArmError::from)?;
        Ok(buf)
    }

    fn write_memory(&mut self, addr: u64, data: &[u8]) -> Result<(), ArmError> {
        self.memory.write(addr, data).map_err(ArmError::from)
    }

    fn arch_version(&self) -> ArmVersion {
        self.version
    }

    fn profile(&self) -> ArmProfile {
        ArmProfile::M
    }

    fn features(&self) -> ArmFeatures {
        self.features
    }

    fn pending_exceptions(&self) -> Vec<ArmException> {
        self.pending_exceptions.clone()
    }

    fn inject_exception(&mut self, exception: ArmException) -> Result<(), ArmError> {
        match exception {
            ArmException::Irq(irq) => {
                self.nvic.set_pending(irq);
            }
            ArmException::Nmi => {
                self.scb.set_nmi_pending(true);
            }
            ArmException::PendSv => {
                self.scb.set_pendsv_pending(true);
            }
            ArmException::SysTick => {
                self.scb.set_systick_pending(true);
            }
            _ => {
                self.pending_exceptions.push(exception);
            }
        }
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
        self.watchpoints.push((addr, size, kind));
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
        self.vfp.is_some()
    }

    fn get_simd_reg(&self, reg: u8) -> Option<(u64, u64)> {
        self.vfp.as_ref().map(|v| {
            let low = v.read_d(reg * 2);
            let high = v.read_d(reg * 2 + 1);
            (low.to_bits(), high.to_bits())
        })
    }

    fn set_simd_reg(&mut self, reg: u8, low: u64, high: u64) -> Result<(), ArmError> {
        if let Some(ref mut vfp) = self.vfp {
            vfp.write_d(reg * 2, f64::from_bits(low));
            vfp.write_d(reg * 2 + 1, f64::from_bits(high));
            Ok(())
        } else {
            Err(ArmError::Unimplemented("FPU not available".to_string()))
        }
    }

    fn get_fpcr(&self) -> Option<u32> {
        self.vfp.as_ref().map(|v| v.fpscr.bits())
    }

    fn set_fpcr(&mut self, value: u32) -> Result<(), ArmError> {
        if let Some(ref mut vfp) = self.vfp {
            vfp.fpscr = crate::arm::vfp::Fpscr::from_bits(value);
            Ok(())
        } else {
            Err(ArmError::Unimplemented("FPU not available".to_string()))
        }
    }

    fn get_fpsr(&self) -> Option<u32> {
        self.vfp.as_ref().map(|v| v.fpscr.bits() & 0x0000_009F)
    }

    fn set_fpsr(&mut self, value: u32) -> Result<(), ArmError> {
        if let Some(ref mut vfp) = self.vfp {
            let fpscr = vfp.fpscr.bits();
            vfp.fpscr =
                crate::arm::vfp::Fpscr::from_bits((fpscr & !0x0000_009F) | (value & 0x0000_009F));
            Ok(())
        } else {
            Err(ArmError::Unimplemented("FPU not available".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm::memory::ArmMemory;

    fn create_test_cpu() -> CortexMCpu {
        let mut memory = FlatMemory::new(0, 0x10000);
        // Set up minimal vector table
        memory.write_u32(0, 0x1000).unwrap(); // Initial SP
        memory.write_u32(4, 0x101).unwrap(); // Reset vector (Thumb)
        CortexMCpu::new(CortexMVariant::CortexM4, Box::new(memory))
    }

    #[test]
    fn test_cpu_creation() {
        let cpu = create_test_cpu();
        assert_eq!(cpu.variant(), CortexMVariant::CortexM4);
        assert_eq!(cpu.profile(), ArmProfile::M);
        assert!(cpu.has_fpu());
    }

    #[test]
    fn test_cpu_reset() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        assert_eq!(cpu.get_sp(), 0x1000);
        assert_eq!(cpu.get_pc(), 0x100);
        assert!(cpu.is_privileged());
    }

    #[test]
    fn test_mov_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // MOV R0, #42 (0x202A)
        cpu.memory_mut().write_u16(0x100, 0x202A).unwrap();
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 42);
    }

    #[test]
    fn test_add_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // MOV R0, #10
        cpu.memory_mut().write_u16(0x100, 0x200A).unwrap();
        // ADD R0, #5
        cpu.memory_mut().write_u16(0x102, 0x3005).unwrap();

        cpu.step().unwrap();
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 15);
    }

    #[test]
    fn test_push_pop() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        let initial_sp = cpu.get_sp();

        // Set up some register values
        cpu.set_gpr(0, 0x1234);
        cpu.set_gpr(1, 0x5678);

        // PUSH {R0, R1} (0xB403)
        cpu.memory_mut().write_u16(0x100, 0xB403).unwrap();
        cpu.step().unwrap();

        assert_eq!(cpu.get_sp(), initial_sp - 8);

        // Clear registers
        cpu.set_gpr(0, 0);
        cpu.set_gpr(1, 0);

        // POP {R0, R1} (0xBC03)
        cpu.memory_mut().write_u16(0x102, 0xBC03).unwrap();
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0x1234);
        assert_eq!(cpu.get_gpr(1), 0x5678);
        assert_eq!(cpu.get_sp(), initial_sp);
    }

    #[test]
    fn test_conditional_branch() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // Set Z flag
        cpu.set_z(true);

        // BEQ +4 (0xD001) - should branch
        cpu.memory_mut().write_u16(0x100, 0xD001).unwrap();
        cpu.step().unwrap();

        assert_eq!(cpu.get_pc(), 0x106); // 0x102 + 4

        // Reset and try BNE
        cpu.set_pc(0x100);
        cpu.set_z(true);

        // BNE +4 (0xD101) - should not branch
        cpu.memory_mut().write_u16(0x100, 0xD101).unwrap();
        cpu.step().unwrap();

        assert_eq!(cpu.get_pc(), 0x102); // No branch
    }

    #[test]
    fn test_breakpoint() {
        let mut cpu = create_test_cpu();
        cpu.reset();
        cpu.set_breakpoint(0x100).unwrap();

        // Any instruction at 0x100
        cpu.memory_mut().write_u16(0x100, 0xBF00).unwrap(); // NOP

        let exit = cpu.step().unwrap();
        assert!(matches!(exit, CpuExit::Breakpoint(0x100)));
    }

    #[test]
    fn test_svc() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // SVC #42 (0xDF2A)
        cpu.memory_mut().write_u16(0x100, 0xDF2A).unwrap();

        let exit = cpu.step().unwrap();
        assert!(matches!(exit, CpuExit::Svc(42)));
    }

    #[test]
    fn test_wfi() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // WFI (0xBF30)
        cpu.memory_mut().write_u16(0x100, 0xBF30).unwrap();

        let exit = cpu.step().unwrap();
        assert!(matches!(exit, CpuExit::Wfi));
    }

    #[test]
    fn test_flags() {
        let mut cpu = create_test_cpu();

        cpu.set_n(true);
        assert!(cpu.get_n());

        cpu.set_z(true);
        assert!(cpu.get_z());

        cpu.set_c(true);
        assert!(cpu.get_c());

        cpu.set_v(true);
        assert!(cpu.get_v());

        cpu.update_nz(0);
        assert!(cpu.get_z());
        assert!(!cpu.get_n());

        cpu.update_nz(0x8000_0000);
        assert!(!cpu.get_z());
        assert!(cpu.get_n());
    }

    // =========================================================================
    // Thumb-32 Instruction Tests
    // =========================================================================

    /// Helper to write a Thumb-32 instruction (little-endian half-words)
    fn write_thumb32(cpu: &mut CortexMCpu, addr: u32, insn: u32) {
        let hw1 = ((insn >> 16) & 0xFFFF) as u16;
        let hw2 = (insn & 0xFFFF) as u16;
        cpu.memory_mut().write_u16(addr as u64, hw1).unwrap();
        cpu.memory_mut().write_u16((addr + 2) as u64, hw2).unwrap();
    }

    #[test]
    fn test_thumb32_bl() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // BL to offset +0x100 from PC
        // BL encoding: 1111 0xxx xxxx xxxx 11x1 xxxx xxxx xxxx
        // For offset 0x100: S=0, imm10=0, J1=1, J2=1, imm11=0x80
        // PC at 0x100, target = 0x204
        // offset = 0x100, so imm10 = 0, imm11 = 0x80, S = 0
        // hw1 = 1111 0S00 0000 0000 = F000
        // hw2 = 11D1 Jimm11 = F880 (J1=1, J2=1, imm11=0x80)
        write_thumb32(&mut cpu, 0x100, 0xF000_F880);
        cpu.step().unwrap();

        assert_eq!(cpu.get_pc(), 0x204);
        assert_eq!(cpu.get_lr() & !1, 0x104); // Return address
    }

    #[test]
    fn test_thumb32_mov_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // MOVW R0, #0x1234
        // Encoding: 1111 0i10 0100 imm4 | 0 imm3 Rd imm8
        // imm16 = 0x1234: imm4=1, i=0, imm3=2, imm8=0x34
        // hw1 = 1111 0010 0100 0001 = F241
        // hw2 = 0 010 0000 0011 0100 = 0034 (imm3=0x2, rd=0, imm8=0x34)
        // Actually: MOV.W Rd, #const where const uses ThumbExpandImm
        // Let's use a simpler constant: MOV R0, #0xFF
        // Encoding for MOV.W R0, #0xFF:
        // op=0010, S=0, Rn=1111, i=0, imm3=0, Rd=0, imm8=0xFF
        // hw1 = 1111 0x00 010x 1111 = F04F
        // hw2 = 0 000 0000 1111 1111 = 00FF
        write_thumb32(&mut cpu, 0x100, 0xF04F_00FF);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0xFF);
    }

    #[test]
    fn test_thumb32_add_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 100);

        // ADD.W R0, R1, #50
        // Encoding: op=1000, S=0, Rn=1, i=0, imm3=0, Rd=0, imm8=50
        // hw1 = 1111 0x01 000x 0001 = F101
        // hw2 = 0 000 0000 0011 0010 = 0032
        write_thumb32(&mut cpu, 0x100, 0xF101_0032);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 150);
    }

    #[test]
    fn test_thumb32_sub_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 100);

        // SUB.W R0, R1, #30
        // Encoding: op=1101, S=0, Rn=1
        // hw1 = 1111 0x01 101x 0001 = F1A1
        // hw2 = 0 000 0000 0001 1110 = 001E
        write_thumb32(&mut cpu, 0x100, 0xF1A1_001E);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 70);
    }

    #[test]
    fn test_thumb32_and_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 0xFF);

        // AND.W R0, R1, #0x0F
        // Encoding: op=0000, S=0, Rn=1
        // hw1 = 1111 0x00 000x 0001 = F001
        // hw2 = 0 000 0000 0000 1111 = 000F
        write_thumb32(&mut cpu, 0x100, 0xF001_000F);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0x0F);
    }

    #[test]
    fn test_thumb32_orr_imm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 0xF0);

        // ORR.W R0, R1, #0x0F
        // Encoding: op=0010, S=0, Rn=1
        // hw1 = 1111 0x00 010x 0001 = F041
        // hw2 = 0 000 0000 0000 1111 = 000F
        write_thumb32(&mut cpu, 0x100, 0xF041_000F);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0xFF);
    }

    #[test]
    fn test_thumb32_mul() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 7);
        cpu.set_gpr(2, 6);

        // MUL R0, R1, R2
        // Encoding: 1111 1011 0000 Rn | Ra Rd 0000 Rm
        // Ra = 1111 (no accumulate), Rn=1, Rm=2, Rd=0
        // hw1 = 1111 1011 0000 0001 = FB01
        // hw2 = 1111 0000 0000 0010 = F002
        write_thumb32(&mut cpu, 0x100, 0xFB01_F002);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 42);
    }

    #[test]
    fn test_thumb32_mla() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 7);
        cpu.set_gpr(2, 6);
        cpu.set_gpr(3, 10);

        // MLA R0, R1, R2, R3
        // Encoding: 1111 1011 0000 Rn | Ra Rd 0000 Rm
        // Ra = 3, Rn=1, Rm=2, Rd=0
        // hw1 = 1111 1011 0000 0001 = FB01
        // hw2 = 0011 0000 0000 0010 = 3002
        write_thumb32(&mut cpu, 0x100, 0xFB01_3002);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 52); // 7*6 + 10 = 52
    }

    #[test]
    fn test_thumb32_sdiv() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 100);
        cpu.set_gpr(2, 7);

        // SDIV R0, R1, R2
        // Encoding: 1111 1011 1001 Rn | 1111 Rd 1111 Rm
        // hw1 = 1111 1011 1001 0001 = 0xFB91
        // hw2 = 1111 0000 1111 0010 = 0xF0F2
        write_thumb32(&mut cpu, 0x100, 0xFB91_F0F2);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 14); // 100 / 7 = 14
    }

    #[test]
    fn test_thumb32_udiv() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 100);
        cpu.set_gpr(2, 3);

        // UDIV R0, R1, R2
        // Encoding: 1111 1011 1011 Rn | 1111 Rd 1111 Rm
        // hw1 = 1111 1011 1011 0001 = 0xFBB1
        // hw2 = 1111 0000 1111 0010 = 0xF0F2
        write_thumb32(&mut cpu, 0x100, 0xFBB1_F0F2);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 33); // 100 / 3 = 33
    }

    #[test]
    fn test_thumb32_smull() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(2, 0x12345);
        cpu.set_gpr(3, 0x6789A);

        // SMULL R0, R1, R2, R3
        // Encoding: 1111 1011 1000 Rn | RdLo RdHi 0000 Rm
        // Rn=2, Rm=3, RdLo=0, RdHi=1
        // hw1 = 1111 1011 1000 0010 = FB82
        // hw2 = 0000 0001 0000 0011 = 0103
        write_thumb32(&mut cpu, 0x100, 0xFB82_0103);
        cpu.step().unwrap();

        let result = ((cpu.get_gpr(1) as u64) << 32) | (cpu.get_gpr(0) as u64);
        assert_eq!(result, 0x12345u64 * 0x6789Au64);
    }

    #[test]
    fn test_thumb32_umull() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(2, 0xFFFFFFFF);
        cpu.set_gpr(3, 2);

        // UMULL R0, R1, R2, R3
        // Encoding: 1111 1011 1010 Rn | RdLo RdHi 0000 Rm
        // Rn=2, Rm=3, RdLo=0, RdHi=1
        // hw1 = 1111 1011 1010 0010 = FBA2
        // hw2 = 0000 0001 0000 0011 = 0103
        write_thumb32(&mut cpu, 0x100, 0xFBA2_0103);
        cpu.step().unwrap();

        let result = ((cpu.get_gpr(1) as u64) << 32) | (cpu.get_gpr(0) as u64);
        assert_eq!(result, 0xFFFFFFFFu64 * 2);
    }

    #[test]
    fn test_thumb32_ldr_imm12() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // Store a value in memory
        cpu.memory_mut().write_u32(0x200, 0xDEADBEEF).unwrap();
        cpu.set_gpr(1, 0x200);

        // LDR.W R0, [R1, #0]
        // Encoding: 1111 1000 1101 Rn | Rt imm12
        // Rn=1, Rt=0, imm12=0
        // hw1 = 1111 1000 1101 0001 = F8D1
        // hw2 = 0000 0000 0000 0000 = 0000
        write_thumb32(&mut cpu, 0x100, 0xF8D1_0000);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0xDEADBEEF);
    }

    #[test]
    fn test_thumb32_str_imm12() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(0, 0x12345678);
        cpu.set_gpr(1, 0x300);

        // STR.W R0, [R1, #0]
        // Encoding: 1111 1000 1100 Rn | Rt imm12
        // Rn=1, Rt=0, imm12=0
        // hw1 = 1111 1000 1100 0001 = F8C1
        // hw2 = 0000 0000 0000 0000 = 0000
        write_thumb32(&mut cpu, 0x100, 0xF8C1_0000);
        cpu.step().unwrap();

        let stored = cpu.memory().read_u32(0x300).unwrap();
        assert_eq!(stored, 0x12345678);
    }

    #[test]
    fn test_thumb32_ldm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        // Set up memory
        cpu.memory_mut().write_u32(0x300, 0x11111111).unwrap();
        cpu.memory_mut().write_u32(0x304, 0x22222222).unwrap();
        cpu.memory_mut().write_u32(0x308, 0x33333333).unwrap();
        cpu.set_gpr(5, 0x300);

        // LDM R5!, {R0, R1, R2}
        // Encoding: 1110 1000 1011 Rn | PM register_list
        // W=1, Rn=5, register_list = 0x0007
        // hw1 = 1110 1000 1011 0101 = E8B5
        // hw2 = 0000 0000 0000 0111 = 0007
        write_thumb32(&mut cpu, 0x100, 0xE8B5_0007);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0x11111111);
        assert_eq!(cpu.get_gpr(1), 0x22222222);
        assert_eq!(cpu.get_gpr(2), 0x33333333);
        assert_eq!(cpu.get_gpr(5), 0x30C); // Writeback
    }

    #[test]
    fn test_thumb32_stm() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(0, 0xAAAAAAAA);
        cpu.set_gpr(1, 0xBBBBBBBB);
        cpu.set_gpr(5, 0x400);

        // STMIA R5!, {R0, R1}
        // Encoding: 1110 1000 1010 Rn | 0M register_list
        // W=1, Rn=5, register_list = 0x0003
        // hw1 = 1110 1000 1010 0101 = E8A5
        // hw2 = 0000 0000 0000 0011 = 0003
        write_thumb32(&mut cpu, 0x100, 0xE8A5_0003);
        cpu.step().unwrap();

        assert_eq!(cpu.memory().read_u32(0x400).unwrap(), 0xAAAAAAAA);
        assert_eq!(cpu.memory().read_u32(0x404).unwrap(), 0xBBBBBBBB);
        assert_eq!(cpu.get_gpr(5), 0x408); // Writeback
    }

    #[test]
    fn test_thumb32_ldrd() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.memory_mut().write_u32(0x500, 0x11112222).unwrap();
        cpu.memory_mut().write_u32(0x504, 0x33334444).unwrap();
        cpu.set_gpr(5, 0x500);

        // LDRD Rt, Rt2, [Rn, #imm8*4]
        // Encoding: 1110 100P U1W1 Rn | Rt Rt2 imm8
        // For LDRD: P=1, U=1, W=0 (no writeback), L=1 (load)
        // Bits: 1110 1001 0101 Rn | Rt Rt2 imm8
        // With Rn=5, Rt=0, Rt2=1, imm8=0:
        // hw1 = 1110 1001 0101 0101 = E955
        // hw2 = 0000 0001 0000 0000 = 0100
        // But this goes to load_store_dual which checks op1, op2, op3
        // op1 = (insn >> 23) & 0x3 = 0b01
        // op2 = (insn >> 20) & 0x3 = 0b01 for LDRD, 0b00 for STRD? No...
        // Let me check: E955 = 1110 1001 0101 0101
        // bits [24:23] = 01, bits [21:20] = 01
        // But handler checks (0b00|0b01, 0b11, _) for LDRD
        // So op2 needs to be 0b11 for LDRD
        // LDRD is: 1110 100P U D W L Rn...
        // For load dual (LDRD): L=1, so bit 20=1
        // Wait the encoding shows: 1110 100 P U 1 W 1 for LDRD
        // So bits [21:20] = 1 1 = 0b11
        // hw1 should be: 1110 1001 1101 0101 = E9D5
        write_thumb32(&mut cpu, 0x100, 0xE9D5_0100);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0x11112222);
        assert_eq!(cpu.get_gpr(1), 0x33334444);
    }

    #[test]
    fn test_thumb32_strd() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(0, 0xAAAABBBB);
        cpu.set_gpr(1, 0xCCCCDDDD);
        cpu.set_gpr(5, 0x600);

        // STRD Rt, Rt2, [Rn, #imm8*4]
        // Encoding: 1110 100P U D W 0 Rn | Rt Rt2 imm8
        // For STRD: P=1, U=1, W=0, L=0 (store)
        // bits [21:20] = 1 0 = 0b10
        // hw1 = 1110 1001 1100 0101 = E9C5
        // hw2 = 0000 0001 0000 0000 = 0100
        write_thumb32(&mut cpu, 0x100, 0xE9C5_0100);
        cpu.step().unwrap();

        assert_eq!(cpu.memory().read_u32(0x600).unwrap(), 0xAAAABBBB);
        assert_eq!(cpu.memory().read_u32(0x604).unwrap(), 0xCCCCDDDD);
    }

    #[test]
    fn test_thumb32_clz() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 0x00010000); // CLZ = 15

        // CLZ R0, R1
        // Encoding: 1111 1010 1011 Rn | 1111 Rd 10 00 Rm
        // For CLZ: op1=0b1011, op2=0b1000
        // Rn=1, Rd=0, Rm=1 (same as Rn)
        // hw1 = 1111 1010 1011 0001 = FAB1
        // hw2 = 1111 0000 1000 0001 = F081
        // This goes to exec_data_processing_reg_32
        // op1 = (insn >> 20) & 0xF = 0xB
        // op2 = (insn >> 4) & 0xF = 0x8 (but CLZ expects 0)
        // Actually CLZ encoding is: 1111 1010 1011 nnnn | 1111 dddd 1000 mmmm
        // op2 should be checked as == 0b0000
        // The issue is my handler checks op2 == 0b0000 but we have 0b1000
        // CLZ is FA B n F d 8 m = FAB1 F081
        // Wait, let me re-read the handler...
        // 0b1011 if op2 == 0b0000 => CLZ
        // But we need op2 from bits [7:4] of hw2 = 0b1000
        // That's wrong - CLZ should have op2=0b1000
        // Let me fix this in the handler instead
        write_thumb32(&mut cpu, 0x100, 0xFAB1_F081);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 15);
    }

    #[test]
    fn test_thumb32_rbit() {
        let mut cpu = create_test_cpu();
        cpu.reset();

        cpu.set_gpr(1, 0x80000000);

        // RBIT R0, R1
        // Encoding: 1111 1010 1001 Rn | 1111 Rd 1010 Rm
        // op1=0b1001, op2=0b1010
        // But handler expects op1=0b1000 and op2=0b1100 for RBIT
        // Actual RBIT: 1111 1010 100 1 nnnn | 1111 dddd 1010 mmmm
        // Let me check ARM reference... RBIT is:
        // 1111 1010 1001 nnnn | 1111 dddd 1010 mmmm
        // So op1 = (insn >> 20) & 0xF = 0x9
        // But our handler checks op1=0b1000=8 with op2=0b1100=12
        // Need to fix handler
        write_thumb32(&mut cpu, 0x100, 0xFA91_F0A1);
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 1);
    }
}
