//! ARM instruction execution handlers.
//!
//! This module implements the execution semantics for ARMv7 instructions,
//! providing handlers that operate on the Armv7Cpu state and memory.
//!
//! # Organization
//!
//! Instructions are grouped by category:
//! - Data processing (arithmetic, logical, shift, compare)
//! - Multiply operations
//! - Load/Store operations (including halfword, signed, exclusive)
//! - Branch operations
//! - System operations
//! - Coprocessor operations
//!
//! # Execution Pattern
//!
//! Each instruction handler follows this pattern:
//! 1. Decode operands from the instruction
//! 2. Read source operands (handling PC+8 for R15)
//! 3. Perform the operation
//! 4. Write destination (handling branch for R15)
//! 5. Optionally update flags if S bit is set

use crate::arm::decoder::{Condition, DecodeError, DecodedInsn, Mnemonic, ShiftType};
use crate::arm::execution::{
    ArmMemory, Armv7Cpu, MemoryError, ProcessorMode, Psr, add_with_carry, compute_n_flag,
    compute_z_flag, condition_passed, expand_imm_c, shift_c, sign_extend,
};
use crate::arm::vfp::{
    Fpscr, NeonSize, RoundingMode, vabs_f16_bits, vabs_f32, vabs_f64, vadd_f16_bits, vadd_f32,
    vadd_f64, vadd_i, vand, vbic, vcls_i, vclz_i, vcmp_f16_bits_with_exception,
    vcmp_f32_with_exception, vcmp_f64_with_exception, vcnt_i8, vcvt_f16_bits_f32,
    vcvt_f32_f16_bits, vcvt_f32_f64, vcvt_f32_s32, vcvt_f32_s32_fixed, vcvt_f32_u32,
    vcvt_f32_u32_fixed, vcvt_f64_f32, vcvt_f64_s32, vcvt_f64_s32_fixed, vcvt_f64_u32,
    vcvt_f64_u32_fixed, vcvt_s32_f32, vcvt_s32_f32_fixed, vcvt_s32_f32_round, vcvt_s32_f64,
    vcvt_s32_f64_fixed, vcvt_s32_f64_round, vcvt_u32_f32, vcvt_u32_f32_fixed, vcvt_u32_f32_round,
    vcvt_u32_f64, vcvt_u32_f64_fixed, vcvt_u32_f64_round, vcvtr_s32_f32, vcvtr_s32_f64,
    vcvtr_u32_f32, vcvtr_u32_f64, vdiv_f16_bits, vdiv_f32, vdiv_f64, veor, vfma_f16_bits, vfma_f32,
    vfma_f64, vfms_f16_bits, vfms_f32, vfms_f64, vfnma_f16_bits, vfnma_f32, vfnma_f64,
    vfnms_f16_bits, vfnms_f32, vfnms_f64, vfp_expand_imm_f16, vfp_expand_imm_f32,
    vfp_expand_imm_f64, vmaxnm_f16_bits, vmaxnm_f32, vmaxnm_f64, vminnm_f16_bits, vminnm_f32,
    vminnm_f64, vmla_f16_bits, vmla_f32, vmla_f64, vmls_f16_bits, vmls_f32, vmls_f64,
    vmul_f16_bits, vmul_f32, vmul_f64, vmvn, vneg_f16_bits, vneg_f32, vneg_f64, vnmla_f16_bits,
    vnmla_f32, vnmla_f64, vnmls_f16_bits, vnmls_f32, vnmls_f64, vnmul_f16_bits, vnmul_f32,
    vnmul_f64, vorn, vorr, vrev, vrint_f16_bits, vrint_f32, vrint_f64, vsqrt_f16_bits, vsqrt_f32,
    vsqrt_f64, vsub_f16_bits, vsub_f32, vsub_f64, vsub_i,
};

/// Result of instruction execution.
#[derive(Clone, Debug)]
pub enum ExecResult {
    /// Instruction executed successfully, advance to next instruction.
    Continue,
    /// Branch taken to specified address.
    Branch(u32),
    /// Exception raised (SVC, UDF, etc.).
    Exception(ExceptionType),
    /// CPU halted (WFI, WFE).
    Halt,
    /// Undefined instruction.
    Undefined,
    /// Memory error during execution.
    MemoryFault(MemoryError),
}

#[derive(Clone, Copy, Debug)]
struct NeonStructMem {
    addr: u32,
    regs: u8,
    first: u8,
    inc: u8,
    ebytes: u8,
    writeback: bool,
    rn: usize,
    rm: usize,
}

#[derive(Clone, Copy, Debug)]
struct NeonAllLanesMem {
    addr: u32,
    streams: u8,
    regs: u8,
    first: u8,
    inc: u8,
    ebytes: u8,
    writeback: bool,
    rn: usize,
    rm: usize,
}

#[derive(Clone, Copy, Debug)]
struct NeonSingleLaneMem {
    addr: u32,
    streams: u8,
    first: u8,
    inc: u8,
    ebytes: u8,
    index: u8,
    writeback: bool,
    rn: usize,
    rm: usize,
}

/// Exception types that can be raised during execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExceptionType {
    /// Supervisor call (SVC/SWI).
    SupervisorCall(u32),
    /// Undefined instruction.
    UndefinedInstruction,
    /// Prefetch abort.
    PrefetchAbort(u32),
    /// Data abort.
    DataAbort(u32),
    /// IRQ interrupt.
    Irq,
    /// FIQ fast interrupt.
    Fiq,
    /// Breakpoint (BKPT).
    Breakpoint(u16),
    /// Reset.
    Reset,
}

impl ExceptionType {
    /// Get the exception vector offset for this exception.
    pub fn vector_offset(&self) -> u32 {
        match self {
            ExceptionType::Reset => 0x00,
            ExceptionType::UndefinedInstruction => 0x04,
            ExceptionType::SupervisorCall(_) => 0x08,
            ExceptionType::PrefetchAbort(_) => 0x0C,
            ExceptionType::DataAbort(_) => 0x10,
            ExceptionType::Irq => 0x18,
            ExceptionType::Fiq => 0x1C,
            ExceptionType::Breakpoint(_) => 0x0C, // Uses prefetch abort vector
        }
    }

    /// Get the mode to enter for this exception.
    pub fn target_mode(&self) -> ProcessorMode {
        match self {
            ExceptionType::Reset | ExceptionType::SupervisorCall(_) => ProcessorMode::Supervisor,
            ExceptionType::UndefinedInstruction => ProcessorMode::Undefined,
            ExceptionType::PrefetchAbort(_) | ExceptionType::Breakpoint(_) => ProcessorMode::Abort,
            ExceptionType::DataAbort(_) => ProcessorMode::Abort,
            ExceptionType::Irq => ProcessorMode::Irq,
            ExceptionType::Fiq => ProcessorMode::Fiq,
        }
    }
}

/// Exclusive monitor state for LDREX/STREX.
#[derive(Clone, Debug, Default)]
pub struct ExclusiveMonitor {
    /// Address being monitored (None if not monitoring).
    pub address: Option<u32>,
    /// Size of the monitored region (1, 2, 4, or 8 bytes).
    pub size: u8,
}

impl ExclusiveMonitor {
    pub fn new() -> Self {
        ExclusiveMonitor {
            address: None,
            size: 0,
        }
    }

    /// Mark an address as exclusive.
    pub fn mark_exclusive(&mut self, addr: u32, size: u8) {
        self.address = Some(addr);
        self.size = size;
    }

    /// Check if address is still exclusive and clear the monitor.
    pub fn check_and_clear(&mut self, addr: u32, size: u8) -> bool {
        if self.address == Some(addr) && self.size == size {
            self.address = None;
            true
        } else {
            self.address = None;
            false
        }
    }

    /// Clear the exclusive monitor.
    pub fn clear(&mut self) {
        self.address = None;
    }
}

/// Coprocessor interface for MRC/MCR instructions.
pub trait Coprocessor {
    /// Read from coprocessor register.
    fn read(&self, crn: u8, crm: u8, opc1: u8, opc2: u8) -> Option<u32>;
    /// Write to coprocessor register.
    fn write(&mut self, crn: u8, crm: u8, opc1: u8, opc2: u8, value: u32) -> bool;
}

/// Null coprocessor (returns all zeros, ignores writes).
pub struct NullCoprocessor;

impl Coprocessor for NullCoprocessor {
    fn read(&self, _crn: u8, _crm: u8, _opc1: u8, _opc2: u8) -> Option<u32> {
        Some(0)
    }
    fn write(&mut self, _crn: u8, _crm: u8, _opc1: u8, _opc2: u8, _value: u32) -> bool {
        true
    }
}

/// Instruction executor that ties together CPU state, memory, and decoded instructions.
pub struct Executor<'a, M: ArmMemory> {
    pub cpu: &'a mut Armv7Cpu,
    pub mem: &'a mut M,
    /// Exclusive monitor for LDREX/STREX.
    pub exclusive_monitor: ExclusiveMonitor,
    /// Vector base address register (VBAR).
    pub vbar: u32,
}

impl<'a, M: ArmMemory> Executor<'a, M> {
    /// Create a new executor.
    pub fn new(cpu: &'a mut Armv7Cpu, mem: &'a mut M) -> Self {
        Executor {
            cpu,
            mem,
            exclusive_monitor: ExclusiveMonitor::new(),
            vbar: 0,
        }
    }

    /// Create a new executor with custom VBAR.
    pub fn with_vbar(cpu: &'a mut Armv7Cpu, mem: &'a mut M, vbar: u32) -> Self {
        Executor {
            cpu,
            mem,
            exclusive_monitor: ExclusiveMonitor::new(),
            vbar,
        }
    }

    /// Execute a single decoded instruction.
    pub fn execute(&mut self, insn: &DecodedInsn) -> ExecResult {
        // Check condition code
        if let Some(cond) = insn.cond {
            if !self.condition_passed(cond) {
                return ExecResult::Continue;
            }
        }

        // Dispatch based on mnemonic
        match insn.mnemonic {
            // Data Processing - Arithmetic
            Mnemonic::ADD | Mnemonic::ADDS => self.exec_add(insn),
            Mnemonic::ADC | Mnemonic::ADCS => self.exec_adc(insn),
            Mnemonic::SUB | Mnemonic::SUBS => self.exec_sub(insn),
            Mnemonic::SBC | Mnemonic::SBCS => self.exec_sbc(insn),
            Mnemonic::RSB | Mnemonic::RSBS => self.exec_rsb(insn),
            Mnemonic::RSC | Mnemonic::RSCS => self.exec_rsc(insn),
            Mnemonic::NEG | Mnemonic::NEGS => self.exec_neg(insn),

            // Data Processing - Logical
            Mnemonic::AND | Mnemonic::ANDS => self.exec_and(insn),
            Mnemonic::ORR | Mnemonic::ORRS => self.exec_orr(insn),
            Mnemonic::EOR | Mnemonic::EORS => self.exec_eor(insn),
            Mnemonic::BIC | Mnemonic::BICS => self.exec_bic(insn),
            Mnemonic::ORN | Mnemonic::ORNS => self.exec_orn(insn),

            // Data Processing - Move
            Mnemonic::MOV | Mnemonic::MOVS => self.exec_mov(insn),
            Mnemonic::MVN | Mnemonic::MVNS => self.exec_mvn(insn),
            Mnemonic::MOVZ => self.exec_movw(insn),
            Mnemonic::MOVK => self.exec_movt(insn),

            // Data Processing - Compare
            Mnemonic::CMP => self.exec_cmp(insn),
            Mnemonic::CMN => self.exec_cmn(insn),
            Mnemonic::TST => self.exec_tst(insn),
            Mnemonic::TEQ => self.exec_teq(insn),

            // Data Processing - Shift
            Mnemonic::LSL | Mnemonic::LSLS => self.exec_lsl(insn),
            Mnemonic::LSR | Mnemonic::LSRS => self.exec_lsr(insn),
            Mnemonic::ASR | Mnemonic::ASRS => self.exec_asr(insn),
            Mnemonic::ROR | Mnemonic::RORS => self.exec_ror(insn),
            Mnemonic::RRX | Mnemonic::RRXS => self.exec_rrx(insn),

            // Multiply
            Mnemonic::MUL | Mnemonic::MULS => self.exec_mul(insn),
            Mnemonic::MLA => self.exec_mla(insn),
            Mnemonic::MLS => self.exec_mls(insn),
            Mnemonic::UMULL | Mnemonic::UMULLS => self.exec_umull(insn),
            Mnemonic::SMULL | Mnemonic::SMULLS => self.exec_smull(insn),
            Mnemonic::UMLAL => self.exec_umlal(insn),
            Mnemonic::SMLAL => self.exec_smlal(insn),
            Mnemonic::UMAAL => self.exec_umaal(insn),
            Mnemonic::SDIV => self.exec_sdiv(insn),
            Mnemonic::UDIV => self.exec_udiv(insn),

            // Branch
            Mnemonic::B | Mnemonic::BCC => self.exec_b(insn),
            Mnemonic::BL => self.exec_bl(insn),
            Mnemonic::BX => self.exec_bx(insn),
            Mnemonic::BLX => self.exec_blx(insn),
            Mnemonic::CBZ => self.exec_cbz(insn),
            Mnemonic::CBNZ => self.exec_cbnz(insn),
            Mnemonic::TBB => self.exec_tbb(insn),
            Mnemonic::TBH => self.exec_tbh(insn),

            // Load/Store Word/Byte
            Mnemonic::LDR => self.exec_ldr(insn),
            Mnemonic::LDRB => self.exec_ldrb(insn),
            Mnemonic::STR => self.exec_str(insn),
            Mnemonic::STRB => self.exec_strb(insn),

            // Load/Store Halfword/Signed
            Mnemonic::LDRH => self.exec_ldrh(insn),
            Mnemonic::LDRSH => self.exec_ldrsh(insn),
            Mnemonic::LDRSB => self.exec_ldrsb(insn),
            Mnemonic::STRH => self.exec_strh(insn),

            // Load/Store Double (LDP/STP are the AArch64 names; A32/T32 LDRD/STRD)
            Mnemonic::LDP => self.exec_ldrd(insn),
            Mnemonic::STP => self.exec_strd(insn),

            // Load/Store Exclusive
            Mnemonic::LDXR => self.exec_ldrex(insn),
            Mnemonic::STXR => self.exec_strex(insn),
            Mnemonic::LDXRB => self.exec_ldrexb(insn),
            Mnemonic::STXRB => self.exec_strexb(insn),
            Mnemonic::LDXRH => self.exec_ldrexh(insn),
            Mnemonic::STXRH => self.exec_strexh(insn),
            Mnemonic::LDXP => self.exec_ldrexd(insn),
            Mnemonic::STXP => self.exec_strexd(insn),
            Mnemonic::CLREX => self.exec_clrex(insn),

            // Load/Store Multiple
            Mnemonic::LDM | Mnemonic::LDMIA => self.exec_ldm_stm(insn, true, false, true),
            Mnemonic::LDMIB => self.exec_ldm_stm(insn, true, true, true),
            Mnemonic::LDMDA => self.exec_ldm_stm(insn, true, false, false),
            Mnemonic::LDMDB => self.exec_ldm_stm(insn, true, true, false),
            Mnemonic::STM | Mnemonic::STMIA => self.exec_ldm_stm(insn, false, false, true),
            Mnemonic::STMIB => self.exec_ldm_stm(insn, false, true, true),
            Mnemonic::STMDA => self.exec_ldm_stm(insn, false, false, false),
            Mnemonic::STMDB => self.exec_ldm_stm(insn, false, true, false),
            Mnemonic::PUSH => self.exec_push(insn),
            Mnemonic::POP => self.exec_pop(insn),

            // System
            Mnemonic::SVC | Mnemonic::SWI => self.exec_svc(insn),
            Mnemonic::NOP | Mnemonic::YIELD | Mnemonic::SEV | Mnemonic::SEVL => {
                ExecResult::Continue
            }
            Mnemonic::WFI | Mnemonic::WFE => ExecResult::Halt,
            Mnemonic::CPS => self.exec_cps(insn),
            Mnemonic::SRS => self.exec_srs(insn),
            Mnemonic::RFE => self.exec_rfe(insn),
            Mnemonic::SWP => self.exec_swp(insn),
            Mnemonic::BKPT => self.exec_bkpt(insn),
            Mnemonic::UDF => ExecResult::Exception(ExceptionType::UndefinedInstruction),
            Mnemonic::MRS => self.exec_mrs(insn),
            Mnemonic::MSR => self.exec_msr(insn),
            Mnemonic::DMB | Mnemonic::DSB | Mnemonic::ISB => ExecResult::Continue, // Memory barriers
            Mnemonic::IT => self.exec_it(insn),

            // Coprocessor
            Mnemonic::MCR => self.exec_mcr(insn),
            Mnemonic::MRC => self.exec_mrc(insn),
            Mnemonic::VMSR => self.exec_mcr(insn),
            Mnemonic::VMRS => self.exec_mrc(insn),
            Mnemonic::VLDR => self.exec_vldr(insn),
            Mnemonic::VSTR => self.exec_vstr(insn),
            Mnemonic::VLDM | Mnemonic::VPOP => self.exec_vldm(insn),
            Mnemonic::VSTM | Mnemonic::VPUSH => self.exec_vstm(insn),
            Mnemonic::VLD1 => self.exec_vld1_multiple(insn),
            Mnemonic::VST1 => self.exec_vst1_multiple(insn),
            Mnemonic::VLD2 => self.exec_vld2_multiple(insn),
            Mnemonic::VST2 => self.exec_vst2_multiple(insn),
            Mnemonic::VLD3 => self.exec_vld3_multiple(insn),
            Mnemonic::VST3 => self.exec_vst3_multiple(insn),
            Mnemonic::VLD4 => self.exec_vld4_multiple(insn),
            Mnemonic::VST4 => self.exec_vst4_multiple(insn),
            Mnemonic::VMOV => self.exec_vmov(insn),
            Mnemonic::VMOVL => self.exec_neon_widen_move(insn),
            Mnemonic::VMOVN | Mnemonic::VQMOVN | Mnemonic::VQMOVUN => {
                self.exec_neon_narrow_move(insn)
            }
            Mnemonic::VAND
            | Mnemonic::VBIC
            | Mnemonic::VORR
            | Mnemonic::VORN
            | Mnemonic::VEOR
            | Mnemonic::VBSL
            | Mnemonic::VBIT
            | Mnemonic::VBIF => self.exec_neon_logical_register(insn),
            Mnemonic::VMVN => self.exec_neon_vmvn_register(insn),
            Mnemonic::VREV16 | Mnemonic::VREV32 | Mnemonic::VREV64 => {
                self.exec_neon_vrev_register(insn)
            }
            Mnemonic::VSWP => self.exec_neon_vswp(insn),
            Mnemonic::VDUP => self.exec_neon_vdup(insn),
            Mnemonic::VSHL => self.exec_vshl(insn),
            Mnemonic::VQSHL => self.exec_vqshl(insn),
            Mnemonic::VRSHL | Mnemonic::VQRSHL => self.exec_neon_shift_register(insn),
            Mnemonic::VQSHLU => self.exec_neon_saturating_shift_left_immediate(insn),
            Mnemonic::VSHR
            | Mnemonic::VRSHR
            | Mnemonic::VSRA
            | Mnemonic::VRSRA
            | Mnemonic::VSLI
            | Mnemonic::VSRI => self.exec_neon_shift_immediate(insn),
            Mnemonic::VSHRN
            | Mnemonic::VRSHRN
            | Mnemonic::VQSHRN
            | Mnemonic::VQRSHRN
            | Mnemonic::VQSHRUN
            | Mnemonic::VQRSHRUN => self.exec_neon_shift_narrow_immediate(insn),
            Mnemonic::VTRN | Mnemonic::VUZP | Mnemonic::VZIP => {
                self.exec_neon_pairwise_permute(insn)
            }
            Mnemonic::VPADD | Mnemonic::VPMAX | Mnemonic::VPMIN => {
                self.exec_neon_pairwise_integer(insn)
            }
            Mnemonic::VPADDL | Mnemonic::VPADAL => self.exec_neon_pairwise_add_long(insn),
            Mnemonic::VHADD | Mnemonic::VRHADD | Mnemonic::VHSUB => {
                self.exec_neon_halving_add_sub(insn)
            }
            Mnemonic::VCEQ | Mnemonic::VCGT | Mnemonic::VCGE | Mnemonic::VTST => {
                self.exec_neon_compare(insn)
            }
            Mnemonic::VCLE | Mnemonic::VCLT => self.exec_neon_compare_zero(insn),
            Mnemonic::VACGT | Mnemonic::VACGE => self.exec_neon_fp_compare(insn),
            Mnemonic::VQADD | Mnemonic::VQSUB => self.exec_neon_saturating_add_sub(insn),
            Mnemonic::VQDMULH | Mnemonic::VQRDMULH => self.exec_neon_saturating_doubling_mulh(insn),
            Mnemonic::VQABS | Mnemonic::VQNEG => self.exec_neon_saturating_abs_neg(insn),
            Mnemonic::VRECPE | Mnemonic::VRSQRTE => self.exec_neon_recip_estimate(insn),
            Mnemonic::VRECPS | Mnemonic::VRSQRTS => self.exec_neon_recip_step(insn),
            Mnemonic::VADDL | Mnemonic::VADDW | Mnemonic::VSUBL | Mnemonic::VSUBW => {
                self.exec_neon_long_wide_add_sub(insn)
            }
            Mnemonic::VADDHN | Mnemonic::VRADDHN | Mnemonic::VSUBHN | Mnemonic::VRSUBHN => {
                self.exec_neon_narrow_add_sub(insn)
            }
            Mnemonic::VMULL
            | Mnemonic::VMLAL
            | Mnemonic::VMLSL
            | Mnemonic::VQDMULL
            | Mnemonic::VQDMLAL
            | Mnemonic::VQDMLSL => self.exec_neon_long_multiply(insn),
            Mnemonic::VCLS | Mnemonic::VCLZ | Mnemonic::VCNT => self.exec_neon_count_register(insn),
            Mnemonic::VEXT => self.exec_neon_vext(insn),
            Mnemonic::VTBL | Mnemonic::VTBX => self.exec_neon_table_lookup(insn),
            Mnemonic::VMAX | Mnemonic::VMIN => self.exec_neon_minmax(insn),
            Mnemonic::VABD => self.exec_neon_absdiff(insn),
            Mnemonic::VABA => self.exec_neon_integer_absdiff_accum(insn),
            Mnemonic::VABDL | Mnemonic::VABAL => self.exec_neon_integer_absdiff_long(insn),
            Mnemonic::VADD | Mnemonic::VSUB => self.exec_vadd_vsub(insn),
            Mnemonic::VMUL => self.exec_vmul(insn),
            Mnemonic::VDIV => self.exec_vfp_binop(insn),
            Mnemonic::VNMUL => self.exec_vfp_binop(insn),
            Mnemonic::VMAXNM_F32
            | Mnemonic::VMAXNM_F64
            | Mnemonic::VMAXNM_F16
            | Mnemonic::VMINNM_F32
            | Mnemonic::VMINNM_F64
            | Mnemonic::VMINNM_F16 => self.exec_vfp_binop(insn),
            Mnemonic::VSELEQ | Mnemonic::VSELGE | Mnemonic::VSELGT | Mnemonic::VSELVS => {
                self.exec_vsel(insn)
            }
            Mnemonic::VMLA | Mnemonic::VMLS => self.exec_vmla_vmls(insn),
            Mnemonic::VFMAL | Mnemonic::VFMLS => self.exec_neon_fp16_fused_multiply_long(insn),
            Mnemonic::VFMA
            | Mnemonic::VFMS
            | Mnemonic::VNMLA
            | Mnemonic::VNMLS
            | Mnemonic::VFNMA
            | Mnemonic::VFNMS => self.exec_vfp_accop(insn),
            Mnemonic::VABS | Mnemonic::VNEG if Self::is_neon_abs_neg(insn.raw) => {
                self.exec_neon_abs_neg(insn)
            }
            Mnemonic::VABS | Mnemonic::VNEG => self.exec_vfp_unop(insn),
            Mnemonic::VSQRT => self.exec_vfp_unop(insn),
            Mnemonic::VRINTM_F16
            | Mnemonic::VRINTM_F32
            | Mnemonic::VRINTN_F16
            | Mnemonic::VRINTN_F32
            | Mnemonic::VRINTP_F16
            | Mnemonic::VRINTP_F32
            | Mnemonic::VRINTX_F16
            | Mnemonic::VRINTX_F32
            | Mnemonic::VRINTZ_F16
            | Mnemonic::VRINTZ_F32
                if Self::is_neon_vrint_shape(insn.raw) =>
            {
                self.exec_neon_vrint(insn)
            }
            Mnemonic::VRINTA_F32
            | Mnemonic::VRINTA_F64
            | Mnemonic::VRINTM_F32
            | Mnemonic::VRINTM_F64
            | Mnemonic::VRINTN_F32
            | Mnemonic::VRINTN_F64
            | Mnemonic::VRINTP_F32
            | Mnemonic::VRINTP_F64
            | Mnemonic::VRINTP_F16
            | Mnemonic::VRINTR_F32
            | Mnemonic::VRINTR_F64
            | Mnemonic::VRINTR_F16
            | Mnemonic::VRINTX_F32
            | Mnemonic::VRINTX_F64
            | Mnemonic::VRINTX_F16
            | Mnemonic::VRINTZ_F32
            | Mnemonic::VRINTZ_F64
            | Mnemonic::VRINTZ_F16
            | Mnemonic::VRINTA_F16
            | Mnemonic::VRINTM_F16
            | Mnemonic::VRINTN_F16 => self.exec_vrint(insn),
            Mnemonic::VCMP | Mnemonic::VCMPE => self.exec_vcmp(insn),
            Mnemonic::VCVT_S32_F32
            | Mnemonic::VCVT_U32_F32
            | Mnemonic::VCVT_S32_F16
            | Mnemonic::VCVT_U32_F16
            | Mnemonic::VCVTM_S32_F32
            | Mnemonic::VCVTM_U32_F32
            | Mnemonic::VCVTM_S32_F16
            | Mnemonic::VCVTM_U32_F16
            | Mnemonic::VCVTN_S32_F32
            | Mnemonic::VCVTN_U32_F32
            | Mnemonic::VCVTN_S32_F16
            | Mnemonic::VCVTN_U32_F16
            | Mnemonic::VCVTP_S32_F16
            | Mnemonic::VCVTP_U32_F16
            | Mnemonic::VCVTP_S32_F32
            | Mnemonic::VCVTP_U32_F32
                if Self::is_neon_directed_convert_shape(insn.raw) =>
            {
                self.exec_neon_directed_convert(insn)
            }
            Mnemonic::VCVT_F32_S32
            | Mnemonic::VCVT_F32_U32
            | Mnemonic::VCVT_F16_S32
            | Mnemonic::VCVT_F16_U32
            | Mnemonic::VCVT_S32_F32
            | Mnemonic::VCVT_U32_F32
            | Mnemonic::VCVT_S32_F16
            | Mnemonic::VCVT_U32_F16
            | Mnemonic::VCVT_F64_S32
            | Mnemonic::VCVT_F64_U32
            | Mnemonic::VCVT_S32_F64
            | Mnemonic::VCVT_U32_F64
            | Mnemonic::VCVT_F64_F32
            | Mnemonic::VCVT_F32_F64
            | Mnemonic::VCVT_F16_F32
            | Mnemonic::VCVT_F32_F16
            | Mnemonic::VCVTB_F32_F16
            | Mnemonic::VCVTT_F32_F16
            | Mnemonic::VCVTB_F16_F32
            | Mnemonic::VCVTT_F16_F32
            | Mnemonic::VCVT_F32_S32_FIXED
            | Mnemonic::VCVT_F32_U32_FIXED
            | Mnemonic::VCVT_S32_F32_FIXED
            | Mnemonic::VCVT_U32_F32_FIXED
            | Mnemonic::VCVT_F64_S32_FIXED
            | Mnemonic::VCVT_F64_U32_FIXED
            | Mnemonic::VCVT_S32_F64_FIXED
            | Mnemonic::VCVT_U32_F64_FIXED
            | Mnemonic::VCVTA_S32_F32
            | Mnemonic::VCVTA_U32_F32
            | Mnemonic::VCVTA_S32_F16
            | Mnemonic::VCVTA_U32_F16
            | Mnemonic::VCVTA_S32_F64
            | Mnemonic::VCVTA_U32_F64
            | Mnemonic::VCVTM_S32_F32
            | Mnemonic::VCVTM_U32_F32
            | Mnemonic::VCVTM_S32_F16
            | Mnemonic::VCVTM_U32_F16
            | Mnemonic::VCVTM_S32_F64
            | Mnemonic::VCVTM_U32_F64
            | Mnemonic::VCVTN_S32_F32
            | Mnemonic::VCVTN_U32_F32
            | Mnemonic::VCVTN_S32_F16
            | Mnemonic::VCVTN_U32_F16
            | Mnemonic::VCVTN_S32_F64
            | Mnemonic::VCVTN_U32_F64
            | Mnemonic::VCVTP_S32_F16
            | Mnemonic::VCVTP_U32_F16
            | Mnemonic::VCVTP_S32_F32
            | Mnemonic::VCVTP_U32_F32
            | Mnemonic::VCVTP_S32_F64
            | Mnemonic::VCVTP_U32_F64
            | Mnemonic::VCVTR_S32_F16
            | Mnemonic::VCVTR_U32_F16
            | Mnemonic::VCVTR_S32_F32
            | Mnemonic::VCVTR_U32_F32
            | Mnemonic::VCVTR_S32_F64
            | Mnemonic::VCVTR_U32_F64 => self.exec_vcvt(insn),

            // Bit manipulation
            Mnemonic::CLZ => self.exec_clz(insn),
            Mnemonic::REV => self.exec_rev(insn),
            Mnemonic::REV16 => self.exec_rev16(insn),
            Mnemonic::REVSH => self.exec_revsh(insn),
            Mnemonic::RBIT => self.exec_rbit(insn),

            // Bit field
            Mnemonic::BFC => self.exec_bfc(insn),
            Mnemonic::BFI => self.exec_bfi(insn),
            Mnemonic::UBFX => self.exec_ubfx(insn),
            Mnemonic::SBFX => self.exec_sbfx(insn),

            // Extension
            Mnemonic::SXTB => self.exec_sxtb(insn),
            Mnemonic::SXTH => self.exec_sxth(insn),
            Mnemonic::UXTB => self.exec_uxtb(insn),
            Mnemonic::UXTH => self.exec_uxth(insn),

            // Saturating arithmetic
            Mnemonic::USAT => self.exec_usat(insn),
            Mnemonic::SSAT => self.exec_ssat(insn),

            // AArch32 media / DSP
            Mnemonic::A32_PARALLEL => self.exec_a32_parallel(insn),
            Mnemonic::A32_PKH => self.exec_a32_pkh(insn),
            Mnemonic::A32_EXTEND => self.exec_a32_extend(insn),
            Mnemonic::A32_SAT16 => self.exec_a32_sat16(insn),
            Mnemonic::A32_SAT_ADDSUB => self.exec_a32_sat_addsub(insn),
            Mnemonic::A32_HMUL => self.exec_a32_hmul(insn),
            Mnemonic::A32_DUAL => self.exec_a32_dual(insn),
            Mnemonic::A32_SMLALD => self.exec_a32_smlald(insn),
            Mnemonic::A32_SMMUL => self.exec_a32_smmul(insn),
            Mnemonic::A32_USAD => self.exec_a32_usad(insn),
            Mnemonic::A32_SEL => self.exec_a32_sel(insn),

            // Undefined/Unknown
            Mnemonic::UNDEFINED | Mnemonic::UNKNOWN => ExecResult::Undefined,

            // Not yet implemented
            _ => ExecResult::Undefined,
        }
    }

    // =========================================================================
    // Exception Handling
    // =========================================================================

    /// Take an exception and switch to the appropriate mode.
    pub fn take_exception(&mut self, exception: ExceptionType) {
        let target_mode = exception.target_mode();
        let vector_offset = exception.vector_offset();

        // Save CPSR to SPSR of target mode
        let cpsr_value = self.cpu.cpsr.to_u32();

        // Calculate return address based on exception type
        let return_addr = match &exception {
            ExceptionType::SupervisorCall(_) => self.cpu.regs[15].wrapping_add(4),
            ExceptionType::UndefinedInstruction => self.cpu.regs[15].wrapping_add(4),
            ExceptionType::PrefetchAbort(_) => self.cpu.regs[15].wrapping_add(4),
            ExceptionType::DataAbort(_) => self.cpu.regs[15].wrapping_add(8),
            ExceptionType::Irq => self.cpu.regs[15].wrapping_add(4),
            ExceptionType::Fiq => self.cpu.regs[15].wrapping_add(4),
            ExceptionType::Breakpoint(_) => self.cpu.regs[15].wrapping_add(4),
            ExceptionType::Reset => 0,
        };

        // Switch mode
        self.cpu.change_mode(target_mode);

        // Set SPSR
        if let Some(spsr) = self.cpu.get_current_spsr_mut() {
            *spsr = Psr::from_u32(cpsr_value);
        }

        // Set LR to return address
        self.cpu.regs[14] = return_addr;

        // Update CPSR
        self.cpu.cpsr.i = true; // Disable IRQ
        if matches!(exception, ExceptionType::Fiq | ExceptionType::Reset) {
            self.cpu.cpsr.f = true; // Disable FIQ
        }
        self.cpu.cpsr.t = false; // Enter ARM mode

        // Branch to vector
        self.cpu.regs[15] = self.vbar.wrapping_add(vector_offset);
    }

    /// Return from exception (MOVS PC, LR or SUBS PC, LR, #imm with S bit).
    pub fn exception_return(&mut self) {
        if let Some(spsr) = self.cpu.get_current_spsr() {
            let spsr_value = spsr.to_u32();

            if let Some(mode) = ProcessorMode::from_bits(spsr.mode) {
                // Bank-switch first: change_mode reads the OLD mode from
                // cpsr.mode to save the outgoing SP/LR, so the full CPSR
                // restore must happen after it.
                self.cpu.change_mode(mode);
                self.cpu.cpsr = Psr::from_u32(spsr_value);
            }
        }
    }

    /// Check if condition is passed.
    fn condition_passed(&self, cond: Condition) -> bool {
        condition_passed(
            cond as u8,
            self.cpu.cpsr.n,
            self.cpu.cpsr.z,
            self.cpu.cpsr.c,
            self.cpu.cpsr.v,
        )
    }

    /// Get register value with PC+8 handling.
    #[inline]
    fn reg(&self, r: usize) -> u32 {
        self.cpu.reg(r)
    }

    /// Set register value, handling PC writes as branches.
    #[inline]
    fn set_reg(&mut self, r: usize, value: u32) -> ExecResult {
        if r == 15 {
            ExecResult::Branch(value)
        } else {
            self.cpu.regs[r] = value;
            ExecResult::Continue
        }
    }

    /// Set register value, with S bit handling for PC (exception return).
    fn set_reg_with_s(&mut self, r: usize, value: u32, s_bit: bool) -> ExecResult {
        let r = crate::arm::execution::aarch32_reg_index(r);
        if r == 15 {
            if s_bit && !self.cpu.is_user_or_system() {
                // Exception return
                self.exception_return();
            }
            ExecResult::Branch(value)
        } else {
            self.cpu.regs[r] = value;
            ExecResult::Continue
        }
    }

    /// Update APSR flags for logical operations (N, Z, C from shifter).
    fn set_flags_logical(&mut self, result: u32) {
        self.cpu.cpsr.n = compute_n_flag(result);
        self.cpu.cpsr.z = compute_z_flag(result);
        self.cpu.cpsr.c = self.cpu.carry_out;
    }

    /// Update APSR flags for arithmetic operations (N, Z, C, V).
    fn set_flags_arithmetic(&mut self, result: u32) {
        self.cpu.cpsr.n = compute_n_flag(result);
        self.cpu.cpsr.z = compute_z_flag(result);
        self.cpu.cpsr.c = self.cpu.carry_out;
        self.cpu.cpsr.v = self.cpu.overflow;
    }

    // =========================================================================
    // Data Processing - Arithmetic
    // =========================================================================

    fn exec_add(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.cpu.add_with_carry(self.reg(n), operand2, false);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_adc(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self
            .cpu
            .add_with_carry(self.reg(n), operand2, self.cpu.cpsr.c);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.cpu.add_with_carry(self.reg(n), !operand2, true);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_sbc(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self
            .cpu
            .add_with_carry(self.reg(n), !operand2, self.cpu.cpsr.c);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_rsb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.cpu.add_with_carry(!self.reg(n), operand2, true);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_rsc(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self
            .cpu
            .add_with_carry(!self.reg(n), operand2, self.cpu.cpsr.c);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_neg(&mut self, insn: &DecodedInsn) -> ExecResult {
        // NEG Rd, Rm is RSB Rd, Rm, #0
        let (d, m) = if insn.state.is_thumb() {
            let (r, _) = Self::thumb_reg_ops(insn, 2);
            (r[0], r[1])
        } else {
            (((insn.raw >> 12) & 0xF) as usize, (insn.raw & 0xF) as usize)
        };
        let result = self.cpu.add_with_carry(!self.reg(m), 0, true);

        if insn.sets_flags && d != 15 {
            self.set_flags_arithmetic(result);
        }
        self.set_reg(d, result)
    }

    // =========================================================================
    // Data Processing - Logical
    // =========================================================================

    fn exec_and(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) & operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_orr(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) | operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_eor(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) ^ operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_bic(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) & !operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_orn(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) | !operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    // =========================================================================
    // Data Processing - Move
    // =========================================================================

    fn exec_mov(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, _, operand2) = self.decode_dp_operands(insn);
        let result = operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg_with_s(d, result, insn.sets_flags)
    }

    fn exec_mvn(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, _, operand2) = self.decode_dp_operands(insn);
        let result = !operand2;

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_movw(&mut self, insn: &DecodedInsn) -> ExecResult {
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let imm4 = (insn.raw >> 16) & 0xF;
        let imm12 = insn.raw & 0xFFF;
        let imm16 = (imm4 << 12) | imm12;
        self.cpu.regs[d] = imm16;
        ExecResult::Continue
    }

    fn exec_movt(&mut self, insn: &DecodedInsn) -> ExecResult {
        use crate::arm::decoder::Operand;
        let (d, imm16) = if insn.state.is_thumb() {
            let (r, _) = Self::thumb_reg_ops(insn, 1);
            let imm = match insn.operands.last() {
                Some(Operand::Imm(i)) => i.value as u32 & 0xFFFF,
                _ => 0,
            };
            (r[0], imm)
        } else {
            let imm4 = (insn.raw >> 16) & 0xF;
            let imm12 = insn.raw & 0xFFF;
            (((insn.raw >> 12) & 0xF) as usize, (imm4 << 12) | imm12)
        };
        self.cpu.regs[d] = (self.cpu.regs[d] & 0xFFFF) | (imm16 << 16);
        ExecResult::Continue
    }

    // =========================================================================
    // Data Processing - Compare
    // =========================================================================

    fn exec_cmp(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (_, n, operand2) = self.decode_dp_operands(insn);
        let rn = self.reg(n);
        let result = self.cpu.add_with_carry(rn, !operand2, true);
        self.set_flags_arithmetic(result);
        ExecResult::Continue
    }

    fn exec_cmn(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (_, n, operand2) = self.decode_dp_operands(insn);
        let result = self.cpu.add_with_carry(self.reg(n), operand2, false);
        self.set_flags_arithmetic(result);
        ExecResult::Continue
    }

    fn exec_tst(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (_, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) & operand2;
        self.set_flags_logical(result);
        ExecResult::Continue
    }

    fn exec_teq(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (_, n, operand2) = self.decode_dp_operands(insn);
        let result = self.reg(n) ^ operand2;
        self.set_flags_logical(result);
        ExecResult::Continue
    }

    // =========================================================================
    // Data Processing - Shift
    // =========================================================================

    fn exec_lsl(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m, shift_amount) = self.decode_shift_operands(insn);
        let result = self.cpu.shift_c(self.reg(m), ShiftType::LSL, shift_amount);

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_lsr(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m, shift_amount) = self.decode_shift_operands(insn);
        let result = self.cpu.shift_c(self.reg(m), ShiftType::LSR, shift_amount);

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_asr(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m, shift_amount) = self.decode_shift_operands(insn);
        let result = self.cpu.shift_c(self.reg(m), ShiftType::ASR, shift_amount);

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_ror(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m, shift_amount) = self.decode_shift_operands(insn);
        let result = self.cpu.shift_c(self.reg(m), ShiftType::ROR, shift_amount);

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    fn exec_rrx(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m, _) = self.decode_shift_operands(insn);
        let result = self.cpu.shift_c(self.reg(m), ShiftType::RRX, 1);

        if insn.sets_flags && d != 15 {
            self.set_flags_logical(result);
        }
        self.set_reg(d, result)
    }

    // =========================================================================
    // Multiply Operations
    // =========================================================================

    fn exec_mul(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, m) = self.decode_mul_operands(insn);
        let result = self.reg(n).wrapping_mul(self.reg(m));

        if insn.sets_flags {
            self.cpu.cpsr.n = compute_n_flag(result);
            self.cpu.cpsr.z = compute_z_flag(result);
        }
        self.set_reg(d, result)
    }

    fn exec_mla(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, m, a) = self.decode_mla_operands(insn);
        let result = self
            .reg(n)
            .wrapping_mul(self.reg(m))
            .wrapping_add(self.reg(a));
        if insn.sets_flags {
            self.cpu.cpsr.n = compute_n_flag(result);
            self.cpu.cpsr.z = compute_z_flag(result);
        }
        self.set_reg(d, result)
    }

    fn exec_mls(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, m, a) = self.decode_mla_operands(insn);
        let result = self
            .reg(a)
            .wrapping_sub(self.reg(n).wrapping_mul(self.reg(m)));
        self.set_reg(d, result)
    }

    fn exec_umull(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (dlo, dhi, n, m) = self.decode_mull_operands(insn);
        let result = (self.reg(n) as u64).wrapping_mul(self.reg(m) as u64);

        self.cpu.regs[dlo] = result as u32;
        self.cpu.regs[dhi] = (result >> 32) as u32;

        if insn.sets_flags {
            self.cpu.cpsr.n = (result >> 63) != 0;
            self.cpu.cpsr.z = result == 0;
        }
        ExecResult::Continue
    }

    fn exec_smull(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (dlo, dhi, n, m) = self.decode_mull_operands(insn);
        let result = (self.reg(n) as i32 as i64).wrapping_mul(self.reg(m) as i32 as i64) as u64;

        self.cpu.regs[dlo] = result as u32;
        self.cpu.regs[dhi] = (result >> 32) as u32;

        if insn.sets_flags {
            self.cpu.cpsr.n = (result >> 63) != 0;
            self.cpu.cpsr.z = result == 0;
        }
        ExecResult::Continue
    }

    fn exec_umlal(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (dlo, dhi, n, m) = self.decode_mull_operands(insn);
        let addend = ((self.cpu.regs[dhi] as u64) << 32) | (self.cpu.regs[dlo] as u64);
        let result = (self.reg(n) as u64)
            .wrapping_mul(self.reg(m) as u64)
            .wrapping_add(addend);

        self.cpu.regs[dlo] = result as u32;
        self.cpu.regs[dhi] = (result >> 32) as u32;
        if insn.sets_flags {
            self.cpu.cpsr.n = (result >> 63) != 0;
            self.cpu.cpsr.z = result == 0;
        }
        ExecResult::Continue
    }

    fn exec_smlal(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (dlo, dhi, n, m) = self.decode_mull_operands(insn);
        let addend = ((self.cpu.regs[dhi] as u64) << 32) | (self.cpu.regs[dlo] as u64);
        let result = ((self.reg(n) as i32 as i64).wrapping_mul(self.reg(m) as i32 as i64) as u64)
            .wrapping_add(addend);

        self.cpu.regs[dlo] = result as u32;
        self.cpu.regs[dhi] = (result >> 32) as u32;
        if insn.sets_flags {
            self.cpu.cpsr.n = (result >> 63) != 0;
            self.cpu.cpsr.z = result == 0;
        }
        ExecResult::Continue
    }

    /// UMAAL: RdHi:RdLo = Rn*Rm + RdHi + RdLo (all unsigned). No flags.
    fn exec_umaal(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (dlo, dhi, n, m) = self.decode_mull_operands(insn);
        let result = (self.reg(n) as u64)
            .wrapping_mul(self.reg(m) as u64)
            .wrapping_add(self.cpu.regs[dhi] as u64)
            .wrapping_add(self.cpu.regs[dlo] as u64);
        self.cpu.regs[dlo] = result as u32;
        self.cpu.regs[dhi] = (result >> 32) as u32;
        ExecResult::Continue
    }

    fn exec_sdiv(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, m) = self.decode_mul_operands(insn);

        let dividend = self.reg(n) as i32;
        let divisor = self.reg(m) as i32;

        let result = if divisor == 0 {
            0 // Division by zero returns 0 in ARM
        } else if dividend == i32::MIN && divisor == -1 {
            i32::MIN as u32 // Overflow case
        } else {
            (dividend / divisor) as u32
        };

        self.set_reg(d, result)
    }

    fn exec_udiv(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, m) = self.decode_mul_operands(insn);

        let dividend = self.reg(n);
        let divisor = self.reg(m);

        let result = if divisor == 0 { 0 } else { dividend / divisor };

        self.set_reg(d, result)
    }

    // =========================================================================
    // Branch Operations
    // =========================================================================

    fn exec_b(&mut self, insn: &DecodedInsn) -> ExecResult {
        if let Some(target) = self.decode_branch_target(insn) {
            ExecResult::Branch(target)
        } else {
            ExecResult::Undefined
        }
    }

    fn exec_bl(&mut self, insn: &DecodedInsn) -> ExecResult {
        // The return address is the instruction after this BL. In Thumb state
        // LR must carry bit0 = 1 so the eventual `BX LR` resumes in Thumb.
        if insn.state.is_thumb() {
            self.cpu.regs[14] = self.cpu.regs[15].wrapping_add(4) | 1;
        } else {
            self.cpu.regs[14] = self.cpu.regs[15].wrapping_add(4);
        }

        if let Some(target) = self.decode_branch_target(insn) {
            ExecResult::Branch(target)
        } else {
            ExecResult::Undefined
        }
    }

    fn exec_bx(&mut self, insn: &DecodedInsn) -> ExecResult {
        if let Some(m) = self.decode_reg_operand(insn, 0) {
            let target = self.reg(m);
            self.cpu.cpsr.t = (target & 1) != 0;
            ExecResult::Branch(target & !1)
        } else {
            ExecResult::Undefined
        }
    }

    fn exec_blx(&mut self, insn: &DecodedInsn) -> ExecResult {
        let thumb = insn.state.is_thumb();

        if let Some(m) = self.decode_reg_operand(insn, 0) {
            // BLX Rm: target state comes from bit0 of Rm. Read the target
            // BEFORE writing LR (`blx lr`). The register form is 2 bytes in
            // Thumb, 4 in ARM.
            let target = self.reg(m);
            let ret = self.cpu.regs[15].wrapping_add(if thumb { 2 } else { 4 });
            self.cpu.regs[14] = if thumb { ret | 1 } else { ret };
            self.cpu.cpsr.t = (target & 1) != 0;
            ExecResult::Branch(target & !1)
        } else if let Some(target) = self.decode_branch_target(insn) {
            // BLX (immediate): always toggles instruction set. The 4-byte
            // encoding exists in both states.
            let ret = self.cpu.regs[15].wrapping_add(4);
            self.cpu.regs[14] = if thumb { ret | 1 } else { ret };
            if thumb {
                // Thumb → ARM: the branch target is word-aligned.
                self.cpu.cpsr.t = false;
                let align = self.cpu.regs[15].wrapping_add(4) & 3;
                ExecResult::Branch(target.wrapping_sub(align))
            } else {
                // ARM → Thumb.
                self.cpu.cpsr.t = true;
                ExecResult::Branch(target)
            }
        } else {
            ExecResult::Undefined
        }
    }

    fn exec_cbz(&mut self, insn: &DecodedInsn) -> ExecResult {
        // Thumb-2 only
        let n = (insn.raw & 0x7) as usize;
        if self.reg(n) == 0 {
            if let Some(target) = self.decode_branch_target(insn) {
                return ExecResult::Branch(target);
            }
        }
        ExecResult::Continue
    }

    fn exec_cbnz(&mut self, insn: &DecodedInsn) -> ExecResult {
        // Thumb-2 only
        let n = (insn.raw & 0x7) as usize;
        if self.reg(n) != 0 {
            if let Some(target) = self.decode_branch_target(insn) {
                return ExecResult::Branch(target);
            }
        }
        ExecResult::Continue
    }

    /// Table Branch Byte (TBB) - Thumb-2.
    ///
    /// TBB [Rn, Rm]
    ///
    /// Reads a byte from memory[Rn + Rm] and branches forward by 2*byte.
    fn exec_tbb(&mut self, insn: &DecodedInsn) -> ExecResult {
        // TBB encoding: 11101000 1101nnnn 1111 0000 0000mmmm
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let m = (insn.raw & 0xF) as usize;

        let base = self.reg(n);
        let index = self.reg(m);
        let address = base.wrapping_add(index);

        match self.mem.read_byte(address) {
            Ok(offset) => {
                // Branch forward by 2 * offset from PC
                let pc = self.cpu.regs[15];
                let target = pc.wrapping_add(4).wrapping_add((offset as u32) * 2);
                ExecResult::Branch(target)
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    /// Table Branch Halfword (TBH) - Thumb-2.
    ///
    /// TBH [Rn, Rm, LSL #1]
    ///
    /// Reads a halfword from memory[Rn + Rm*2] and branches forward by 2*halfword.
    fn exec_tbh(&mut self, insn: &DecodedInsn) -> ExecResult {
        // TBH encoding: 11101000 1101nnnn 1111 0000 0001mmmm
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let m = (insn.raw & 0xF) as usize;

        let base = self.reg(n);
        let index = self.reg(m);
        let address = base.wrapping_add(index << 1);

        match self.mem.read_halfword(address) {
            Ok(offset) => {
                // Branch forward by 2 * offset from PC
                let pc = self.cpu.regs[15];
                let target = pc.wrapping_add(4).wrapping_add((offset as u32) * 2);
                ExecResult::Branch(target)
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    // =========================================================================
    // Load/Store Operations
    // =========================================================================

    fn exec_ldr(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.read_word(address) {
            Ok(data) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                if t == 15 {
                    // LDR pc is an interworking branch (ARMv5+): bit0 of the
                    // loaded value selects ARM (0) or Thumb (1).
                    self.cpu.cpsr.t = (data & 1) != 0;
                    ExecResult::Branch(data & !1)
                } else {
                    self.set_reg(t, data)
                }
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_ldrb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.read_byte(address) {
            Ok(data) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                self.cpu.regs[t] = data as u32;
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_ldrh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_halfword_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.read_halfword(address) {
            Ok(data) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                self.cpu.regs[t] = data as u32;
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_ldrsb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_halfword_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.read_byte(address) {
            Ok(data) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                self.cpu.regs[t] = sign_extend(data as u32, 8);
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_ldrsh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_halfword_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.read_halfword(address) {
            Ok(data) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                self.cpu.regs[t] = sign_extend(data as u32, 16);
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_str(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.write_word(address, self.reg(t)) {
            Ok(()) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_strb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.write_byte(address, self.reg(t) as u8) {
            Ok(()) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_strh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_halfword_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        match self.mem.write_halfword(address, self.reg(t) as u16) {
            Ok(()) => {
                if let Some((n, addr)) = writeback {
                    self.cpu.regs[n] = addr;
                }
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    // =========================================================================
    // Load/Store Double
    // =========================================================================

    fn exec_ldrd(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_halfword_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };
        let t2 = (t + 1) & 0xF;

        match self.mem.read_word(address) {
            Ok(data1) => match self.mem.read_word(address.wrapping_add(4)) {
                Ok(data2) => {
                    self.cpu.regs[t] = data1;
                    self.cpu.regs[t2] = data2;
                    if let Some((n, addr)) = writeback {
                        self.cpu.regs[n] = addr;
                    }
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            },
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_strd(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (t, address, writeback) = match self.decode_ldst_halfword_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };
        let t2 = (t + 1) & 0xF;

        match self.mem.write_word(address, self.reg(t)) {
            Ok(()) => match self.mem.write_word(address.wrapping_add(4), self.reg(t2)) {
                Ok(()) => {
                    if let Some((n, addr)) = writeback {
                        self.cpu.regs[n] = addr;
                    }
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            },
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    // =========================================================================
    // Load/Store Exclusive
    // =========================================================================

    fn exec_ldrex(&mut self, insn: &DecodedInsn) -> ExecResult {
        let t = ((insn.raw >> 12) & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let address = self.reg(n);

        self.exclusive_monitor.mark_exclusive(address, 4);

        match self.mem.read_word(address) {
            Ok(data) => {
                self.cpu.regs[t] = data;
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_strex(&mut self, insn: &DecodedInsn) -> ExecResult {
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let t = (insn.raw & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let address = self.reg(n);

        if self.exclusive_monitor.check_and_clear(address, 4) {
            match self.mem.write_word(address, self.reg(t)) {
                Ok(()) => {
                    self.cpu.regs[d] = 0; // Success
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            }
        } else {
            self.cpu.regs[d] = 1; // Failure
            ExecResult::Continue
        }
    }

    /// LDREXD: doubleword exclusive load into an even/odd register pair.
    fn exec_ldrexd(&mut self, insn: &DecodedInsn) -> ExecResult {
        let t = ((insn.raw >> 12) & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        if t & 1 != 0 || t == 14 {
            return ExecResult::Undefined;
        }
        let address = self.reg(n);

        self.exclusive_monitor.mark_exclusive(address, 8);

        let lo = match self.mem.read_word(address) {
            Ok(d) => d,
            Err(e) => return ExecResult::MemoryFault(e),
        };
        let hi = match self.mem.read_word(address.wrapping_add(4)) {
            Ok(d) => d,
            Err(e) => return ExecResult::MemoryFault(e),
        };
        self.cpu.regs[t] = lo;
        self.cpu.regs[t + 1] = hi;
        ExecResult::Continue
    }

    /// STREXD: doubleword exclusive store from an even/odd register pair.
    fn exec_strexd(&mut self, insn: &DecodedInsn) -> ExecResult {
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let t = (insn.raw & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        if t & 1 != 0 || t == 14 {
            return ExecResult::Undefined;
        }
        let address = self.reg(n);

        if self.exclusive_monitor.check_and_clear(address, 8) {
            if let Err(e) = self.mem.write_word(address, self.reg(t)) {
                return ExecResult::MemoryFault(e);
            }
            if let Err(e) = self.mem.write_word(address.wrapping_add(4), self.reg(t + 1)) {
                return ExecResult::MemoryFault(e);
            }
            self.cpu.regs[d] = 0; // Success
        } else {
            self.cpu.regs[d] = 1; // Failure
        }
        ExecResult::Continue
    }

    fn exec_ldrexb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let t = ((insn.raw >> 12) & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let address = self.reg(n);

        self.exclusive_monitor.mark_exclusive(address, 1);

        match self.mem.read_byte(address) {
            Ok(data) => {
                self.cpu.regs[t] = data as u32;
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_strexb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let t = (insn.raw & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let address = self.reg(n);

        if self.exclusive_monitor.check_and_clear(address, 1) {
            match self.mem.write_byte(address, self.reg(t) as u8) {
                Ok(()) => {
                    self.cpu.regs[d] = 0;
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            }
        } else {
            self.cpu.regs[d] = 1;
            ExecResult::Continue
        }
    }

    fn exec_ldrexh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let t = ((insn.raw >> 12) & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let address = self.reg(n);

        self.exclusive_monitor.mark_exclusive(address, 2);

        match self.mem.read_halfword(address) {
            Ok(data) => {
                self.cpu.regs[t] = data as u32;
                ExecResult::Continue
            }
            Err(e) => ExecResult::MemoryFault(e),
        }
    }

    fn exec_strexh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let t = (insn.raw & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let address = self.reg(n);

        if self.exclusive_monitor.check_and_clear(address, 2) {
            match self.mem.write_halfword(address, self.reg(t) as u16) {
                Ok(()) => {
                    self.cpu.regs[d] = 0;
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            }
        } else {
            self.cpu.regs[d] = 1;
            ExecResult::Continue
        }
    }

    fn exec_clrex(&mut self, _insn: &DecodedInsn) -> ExecResult {
        self.exclusive_monitor.clear();
        ExecResult::Continue
    }

    // =========================================================================
    // Load/Store Multiple
    // =========================================================================

    /// Unified LDM/STM for all four addressing modes (IA/IB/DA/DB), A32 and T32.
    /// The lowest-numbered register always maps to the lowest address.
    fn exec_ldm_stm(&mut self, insn: &DecodedInsn, is_load: bool, p: bool, u: bool) -> ExecResult {
        let (n, reglist, wback) = match self.decode_ldstm_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };
        let count = reglist.count_ones();
        let base = self.reg(n);
        let low = if u {
            if p { base.wrapping_add(4) } else { base }
        } else if p {
            base.wrapping_sub(count * 4)
        } else {
            base.wrapping_sub(count * 4).wrapping_add(4)
        };
        let wb_val = if u {
            base.wrapping_add(count * 4)
        } else {
            base.wrapping_sub(count * 4)
        };

        // A32 S bit (the `^` forms): without PC in an LDM list it selects the
        // USER bank for the transfer; an LDM with PC additionally restores
        // CPSR from the current SPSR (exception return).
        let s_bit = insn.state == crate::arm::ExecutionState::Aarch32 && (insn.raw >> 22) & 1 == 1;
        let exception_return = s_bit && is_load && reglist & 0x8000 != 0;
        let user_bank = s_bit && !exception_return && !self.cpu.is_user_or_system();

        let mut addr = low;
        let mut branch_target = None;
        for i in 0..16 {
            if reglist & (1 << i) == 0 {
                continue;
            }
            if is_load {
                match self.mem.read_word(addr) {
                    Ok(d) => {
                        if i == 15 {
                            branch_target = Some(d);
                        } else if user_bank && (i == 13 || i == 14) {
                            self.cpu.regs_usr[i - 13] = d;
                        } else {
                            self.cpu.regs[i] = d;
                        }
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            } else {
                let val = if i == 15 {
                    self.cpu.get_pc()
                } else if user_bank && (i == 13 || i == 14) {
                    self.cpu.regs_usr[i - 13]
                } else {
                    self.reg(i)
                };
                if let Err(e) = self.mem.write_word(addr, val) {
                    return ExecResult::MemoryFault(e);
                }
            }
            addr = addr.wrapping_add(4);
        }

        // Writeback (suppressed for LDM when the base is in the loaded list).
        if wback && !(is_load && reglist & (1 << n) != 0) {
            self.cpu.regs[n] = wb_val;
        }

        if exception_return {
            let spsr = self.current_spsr_bits();
            self.write_cpsr_all(spsr);
        }

        if let Some(target) = branch_target {
            if exception_return {
                // CPSR (incl. T) was restored from SPSR; PC is the saved value.
                ExecResult::Branch(target)
            } else {
                // LDM {pc} is an interworking branch (ARMv5+): bit0 selects the
                // instruction set.
                self.cpu.cpsr.t = (target & 1) != 0;
                ExecResult::Branch(target & !1)
            }
        } else {
            ExecResult::Continue
        }
    }

    #[allow(dead_code)]
    fn exec_ldm(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (n, reglist, wback) = match self.decode_ldstm_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        let mut address = self.reg(n);
        let mut branch_target = None;

        for i in 0..16 {
            if (reglist & (1 << i)) != 0 {
                match self.mem.read_word(address) {
                    Ok(data) => {
                        if i == 15 {
                            branch_target = Some(data);
                        } else {
                            self.cpu.regs[i] = data;
                        }
                        address = address.wrapping_add(4);
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            }
        }

        if wback {
            self.cpu.regs[n] = address;
        }

        if let Some(target) = branch_target {
            ExecResult::Branch(target)
        } else {
            ExecResult::Continue
        }
    }

    fn exec_ldmdb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (n, reglist, wback) = match self.decode_ldstm_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        let count = reglist.count_ones();
        let mut address = self.reg(n).wrapping_sub(count * 4);
        let start_address = address;
        let mut branch_target = None;

        for i in 0..16 {
            if (reglist & (1 << i)) != 0 {
                match self.mem.read_word(address) {
                    Ok(data) => {
                        if i == 15 {
                            branch_target = Some(data);
                        } else {
                            self.cpu.regs[i] = data;
                        }
                        address = address.wrapping_add(4);
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            }
        }

        if wback {
            self.cpu.regs[n] = start_address;
        }

        if let Some(target) = branch_target {
            ExecResult::Branch(target)
        } else {
            ExecResult::Continue
        }
    }

    fn exec_stm(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (n, reglist, wback) = match self.decode_ldstm_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        let mut address = self.reg(n);

        for i in 0..16 {
            if (reglist & (1 << i)) != 0 {
                match self.mem.write_word(address, self.reg(i)) {
                    Ok(()) => {
                        address = address.wrapping_add(4);
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            }
        }

        if wback {
            self.cpu.regs[n] = address;
        }

        ExecResult::Continue
    }

    fn exec_stmdb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (n, reglist, wback) = match self.decode_ldstm_operands(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        let count = reglist.count_ones();
        let mut address = self.reg(n).wrapping_sub(count * 4);
        let start_address = address;

        for i in 0..16 {
            if (reglist & (1 << i)) != 0 {
                match self.mem.write_word(address, self.reg(i)) {
                    Ok(()) => {
                        address = address.wrapping_add(4);
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            }
        }

        if wback {
            self.cpu.regs[n] = start_address;
        }

        ExecResult::Continue
    }

    fn exec_push(&mut self, insn: &DecodedInsn) -> ExecResult {
        let reglist = match self.decode_reglist(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        let count = reglist.count_ones();
        let mut address = self.cpu.regs[13].wrapping_sub(count * 4);
        let start_address = address;

        for i in 0..16 {
            if (reglist & (1 << i)) != 0 {
                match self.mem.write_word(address, self.reg(i)) {
                    Ok(()) => {
                        address = address.wrapping_add(4);
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            }
        }

        self.cpu.regs[13] = start_address;
        ExecResult::Continue
    }

    fn exec_pop(&mut self, insn: &DecodedInsn) -> ExecResult {
        let reglist = match self.decode_reglist(insn) {
            Some(v) => v,
            None => return ExecResult::Undefined,
        };

        let mut address = self.cpu.regs[13];
        let mut branch_target = None;

        for i in 0..16 {
            if (reglist & (1 << i)) != 0 {
                match self.mem.read_word(address) {
                    Ok(data) => {
                        if i == 15 {
                            branch_target = Some(data);
                        } else {
                            self.cpu.regs[i] = data;
                        }
                        address = address.wrapping_add(4);
                    }
                    Err(e) => return ExecResult::MemoryFault(e),
                }
            }
        }

        self.cpu.regs[13] = address;

        // A32 `LDM sp!, {..., pc}^` (the S-bit form, e.g. an IRQ handler's
        // exception-return `e8fd900f`) is decoded here as a POP. With PC in the
        // list and the S bit set, it additionally restores CPSR from the
        // current mode's SPSR — without this the CPU stays in the handler's
        // mode (e.g. IRQ) on return, corrupting the resumed context.
        let s_bit =
            insn.state == crate::arm::ExecutionState::Aarch32 && (insn.raw >> 22) & 1 == 1;
        let exception_return =
            s_bit && (reglist & 0x8000) != 0 && !self.cpu.is_user_or_system();

        if let Some(target) = branch_target {
            if exception_return {
                // CPSR (including the T bit) is restored from SPSR; PC is the
                // loaded value verbatim.
                let spsr = self.current_spsr_bits();
                self.write_cpsr_all(spsr);
                ExecResult::Branch(target)
            } else {
                // POP {pc} is an interworking branch (ARMv5+): the loaded PC's
                // bit0 selects ARM (0) or Thumb (1). Set the state explicitly so
                // a Thumb→ARM return (bit0 = 0) actually leaves Thumb.
                self.cpu.cpsr.t = (target & 1) != 0;
                ExecResult::Branch(target & !1)
            }
        } else {
            ExecResult::Continue
        }
    }

    // =========================================================================
    // System Operations
    // =========================================================================

    fn exec_svc(&mut self, insn: &DecodedInsn) -> ExecResult {
        let imm = insn.raw & 0x00FFFFFF;
        ExecResult::Exception(ExceptionType::SupervisorCall(imm))
    }

    fn exec_bkpt(&mut self, insn: &DecodedInsn) -> ExecResult {
        let imm = ((insn.raw >> 8) & 0xFFF0) | (insn.raw & 0xF);
        ExecResult::Exception(ExceptionType::Breakpoint(imm as u16))
    }

    fn exec_mrs(&mut self, insn: &DecodedInsn) -> ExecResult {
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let r = (insn.raw >> 22) & 1;

        let value = if r != 0 {
            if let Some(spsr) = self.cpu.get_current_spsr() {
                spsr.to_u32()
            } else {
                return ExecResult::Undefined;
            }
        } else {
            self.cpu.cpsr.to_u32()
        };

        self.cpu.regs[d] = value;
        ExecResult::Continue
    }

    fn exec_msr(&mut self, insn: &DecodedInsn) -> ExecResult {
        let r = (insn.raw >> 22) & 1;
        let mask = (insn.raw >> 16) & 0xF;

        let value = if (insn.raw >> 25) & 1 != 0 {
            let imm12 = insn.raw & 0xFFF;
            expand_imm_c(imm12, self.cpu.cpsr.c).0
        } else {
            let n = (insn.raw & 0xF) as usize;
            self.reg(n)
        };

        if r != 0 {
            self.write_current_spsr_by_mask(value, mask);
        } else {
            self.write_cpsr_by_mask(value, mask);
        }

        ExecResult::Continue
    }

    fn write_current_spsr_by_mask(&mut self, value: u32, mask: u32) {
        if let Some(spsr) = self.cpu.get_current_spsr_mut() {
            if (mask & 8) != 0 {
                spsr.n = (value >> 31) != 0;
                spsr.z = ((value >> 30) & 1) != 0;
                spsr.c = ((value >> 29) & 1) != 0;
                spsr.v = ((value >> 28) & 1) != 0;
                spsr.q = ((value >> 27) & 1) != 0;
            }
            if (mask & 2) != 0 {
                spsr.e = ((value >> 9) & 1) != 0;
                spsr.a = ((value >> 8) & 1) != 0;
            }
            if (mask & 1) != 0 {
                spsr.i = ((value >> 7) & 1) != 0;
                spsr.f = ((value >> 6) & 1) != 0;
                spsr.t = ((value >> 5) & 1) != 0;
                spsr.mode = (value & 0x1F) as u8;
            }
        }
    }

    fn write_cpsr_by_mask(&mut self, value: u32, mask: u32) {
        if (mask & 8) != 0 {
            self.cpu.cpsr.n = (value >> 31) != 0;
            self.cpu.cpsr.z = ((value >> 30) & 1) != 0;
            self.cpu.cpsr.c = ((value >> 29) & 1) != 0;
            self.cpu.cpsr.v = ((value >> 28) & 1) != 0;
            self.cpu.cpsr.q = ((value >> 27) & 1) != 0;
        }
        if (mask & 2) != 0 {
            self.cpu.cpsr.e = ((value >> 9) & 1) != 0;
            if self.cpu.is_privileged() {
                self.cpu.cpsr.a = ((value >> 8) & 1) != 0;
            }
        }
        if (mask & 1) != 0 && self.cpu.is_privileged() {
            self.cpu.cpsr.i = ((value >> 7) & 1) != 0;
            self.cpu.cpsr.f = ((value >> 6) & 1) != 0;
            self.cpu.cpsr.t = ((value >> 5) & 1) != 0;

            let new_mode = value & 0x1F;
            if let Some(mode) = ProcessorMode::from_bits(new_mode as u8) {
                if self.cpu.cpsr.mode != mode as u8 {
                    self.cpu.change_mode(mode);
                }
            }
        }
    }

    /// Execute IT (If-Then) instruction (Thumb-2).
    ///
    /// IT{x{y{z}}} cond
    ///
    /// Sets up IT state for conditional execution of up to 4 following instructions.
    /// The condition and mask determine which instructions execute and which are skipped.
    fn exec_it(&mut self, insn: &DecodedInsn) -> ExecResult {
        // IT instruction encoding (16-bit Thumb):
        // Bits 7:4 = firstcond (base condition code)
        // Bits 3:0 = mask (determines T/E pattern)
        let firstcond = ((insn.raw >> 4) & 0xF) as u8;
        let mask = (insn.raw & 0xF) as u8;

        // Mask of 0 is not allowed (would be NOP)
        if mask == 0 {
            return ExecResult::Undefined;
        }

        // Set IT state in CPSR
        self.cpu.cpsr.set_it_state(firstcond, mask);

        ExecResult::Continue
    }

    // =========================================================================
    // Coprocessor Operations
    // =========================================================================

    /// CPS: change processor state (ARMv6). NOP in user mode.
    fn exec_cps(&mut self, insn: &DecodedInsn) -> ExecResult {
        if self.cpu.is_user_or_system() && self.cpu.cpsr.mode == ProcessorMode::User as u8 {
            return ExecResult::Continue;
        }
        let raw = insn.raw;
        let imod = (raw >> 18) & 0x3;
        let m = (raw >> 17) & 1;
        let (a, i, f) = ((raw >> 8) & 1, (raw >> 7) & 1, (raw >> 6) & 1);
        match imod {
            0b10 => {
                // CPSIE: enable = clear mask bits
                if a == 1 { self.cpu.cpsr.a = false; }
                if i == 1 { self.cpu.cpsr.i = false; }
                if f == 1 { self.cpu.cpsr.f = false; }
            }
            0b11 => {
                // CPSID: disable = set mask bits
                if a == 1 { self.cpu.cpsr.a = true; }
                if i == 1 { self.cpu.cpsr.i = true; }
                if f == 1 { self.cpu.cpsr.f = true; }
            }
            _ => {}
        }
        if m == 1 {
            if let Some(mode) = ProcessorMode::from_bits((raw & 0x1F) as u8) {
                self.cpu.change_mode(mode);
            }
        }
        ExecResult::Continue
    }

    /// SRS: store return state (LR and SPSR of the current mode) to the
    /// stack of the mode given in the instruction.
    fn exec_srs(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let p = (raw >> 24) & 1 == 1;
        let u = (raw >> 23) & 1 == 1;
        let w = (raw >> 21) & 1 == 1;
        let mode_bits = (raw & 0x1F) as u8;
        let cur_mode = self.cpu.cpsr.mode;
        let sp = self.banked_sp(mode_bits);
        let low = match (p, u) {
            (false, true) => sp,                      // IA
            (true, true) => sp.wrapping_add(4),       // IB
            (false, false) => sp.wrapping_sub(4),     // DA
            (true, false) => sp.wrapping_sub(8),      // DB
        };
        let lr = self.cpu.regs[14];
        let spsr = self.current_spsr_bits();
        if let Err(e) = self.mem.write_word(low, lr) {
            return ExecResult::MemoryFault(e);
        }
        if let Err(e) = self.mem.write_word(low.wrapping_add(4), spsr) {
            return ExecResult::MemoryFault(e);
        }
        if w {
            let nb = if u { sp.wrapping_add(8) } else { sp.wrapping_sub(8) };
            self.set_banked_sp(mode_bits, nb);
        }
        let _ = cur_mode;
        ExecResult::Continue
    }

    /// RFE: return from exception — load PC and CPSR from [Rn].
    fn exec_rfe(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let p = (raw >> 24) & 1 == 1;
        let u = (raw >> 23) & 1 == 1;
        let w = (raw >> 21) & 1 == 1;
        let n = ((raw >> 16) & 0xF) as usize;
        let base = self.cpu.regs[n];
        let low = match (p, u) {
            (false, true) => base,
            (true, true) => base.wrapping_add(4),
            (false, false) => base.wrapping_sub(4),
            (true, false) => base.wrapping_sub(8),
        };
        let new_pc = match self.mem.read_word(low) {
            Ok(v) => v,
            Err(e) => return ExecResult::MemoryFault(e),
        };
        let new_cpsr = match self.mem.read_word(low.wrapping_add(4)) {
            Ok(v) => v,
            Err(e) => return ExecResult::MemoryFault(e),
        };
        if w {
            let nb = if u { base.wrapping_add(8) } else { base.wrapping_sub(8) };
            self.cpu.regs[n] = nb;
        }
        self.write_cpsr_all(new_cpsr);
        ExecResult::Branch(new_pc)
    }

    /// SWP/SWPB: atomic swap (ARMv6-deprecated but still used).
    fn exec_swp(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let byte = (raw >> 22) & 1 == 1;
        let n = ((raw >> 16) & 0xF) as usize;
        let d = ((raw >> 12) & 0xF) as usize;
        let m = (raw & 0xF) as usize;
        let addr = self.cpu.regs[n];
        if byte {
            let old = match self.mem.read_byte(addr) {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            if let Err(e) = self.mem.write_byte(addr, self.cpu.regs[m] as u8) {
                return ExecResult::MemoryFault(e);
            }
            self.cpu.regs[d] = old as u32;
        } else {
            let old = match self.mem.read_word(addr) {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            if let Err(e) = self.mem.write_word(addr, self.cpu.regs[m]) {
                return ExecResult::MemoryFault(e);
            }
            self.cpu.regs[d] = old;
        }
        ExecResult::Continue
    }

    /// SP of an arbitrary mode (live register if it is the current mode).
    fn banked_sp(&self, mode_bits: u8) -> u32 {
        if mode_bits == self.cpu.cpsr.mode {
            return self.cpu.regs[13];
        }
        match ProcessorMode::from_bits(mode_bits) {
            Some(ProcessorMode::User) | Some(ProcessorMode::System) => self.cpu.regs_usr[0],
            Some(ProcessorMode::Fiq) => self.cpu.regs_fiq[5],
            Some(ProcessorMode::Irq) => self.cpu.regs_irq[0],
            Some(ProcessorMode::Supervisor) => self.cpu.regs_svc[0],
            Some(ProcessorMode::Monitor) => self.cpu.regs_mon[0],
            Some(ProcessorMode::Abort) => self.cpu.regs_abt[0],
            Some(ProcessorMode::Undefined) => self.cpu.regs_und[0],
            _ => self.cpu.regs[13],
        }
    }

    fn set_banked_sp(&mut self, mode_bits: u8, value: u32) {
        if mode_bits == self.cpu.cpsr.mode {
            self.cpu.regs[13] = value;
            return;
        }
        match ProcessorMode::from_bits(mode_bits) {
            Some(ProcessorMode::User) | Some(ProcessorMode::System) => {
                self.cpu.regs_usr[0] = value;
            }
            Some(ProcessorMode::Fiq) => self.cpu.regs_fiq[5] = value,
            Some(ProcessorMode::Irq) => self.cpu.regs_irq[0] = value,
            Some(ProcessorMode::Supervisor) => self.cpu.regs_svc[0] = value,
            Some(ProcessorMode::Monitor) => self.cpu.regs_mon[0] = value,
            Some(ProcessorMode::Abort) => self.cpu.regs_abt[0] = value,
            Some(ProcessorMode::Undefined) => self.cpu.regs_und[0] = value,
            _ => self.cpu.regs[13] = value,
        }
    }

    /// Raw bits of the current mode's SPSR (CPSR if none).
    fn current_spsr_bits(&self) -> u32 {
        match ProcessorMode::from_bits(self.cpu.cpsr.mode) {
            Some(ProcessorMode::Fiq) => self.cpu.spsr_fiq.to_u32(),
            Some(ProcessorMode::Irq) => self.cpu.spsr_irq.to_u32(),
            Some(ProcessorMode::Supervisor) => self.cpu.spsr_svc.to_u32(),
            Some(ProcessorMode::Monitor) => self.cpu.spsr_mon.to_u32(),
            Some(ProcessorMode::Abort) => self.cpu.spsr_abt.to_u32(),
            Some(ProcessorMode::Undefined) => self.cpu.spsr_und.to_u32(),
            _ => self.cpu.cpsr.to_u32(),
        }
    }

    /// Write CPSR including the mode field (exception-return semantics).
    fn write_cpsr_all(&mut self, value: u32) {
        let new_mode = (value & 0x1F) as u8;
        if let Some(mode) = ProcessorMode::from_bits(new_mode) {
            self.cpu.change_mode(mode);
        }
        self.cpu.cpsr = Psr::from_u32(value);
    }

    fn exec_mcr(&mut self, insn: &DecodedInsn) -> ExecResult {
        let t = ((insn.raw >> 12) & 0xF) as usize;
        let cp = ((insn.raw >> 8) & 0xF) as u8;
        let opc1 = ((insn.raw >> 21) & 7) as u8;
        let reg = ((insn.raw >> 16) & 0xF) as u8;

        if cp == 10 && opc1 == 0b111 {
            if t == 15 {
                return ExecResult::Undefined;
            }
            let value = self.reg(t);
            return match reg {
                0 => ExecResult::Continue,
                1 => {
                    if !self.cpu.vfp.is_enabled() {
                        ExecResult::Exception(ExceptionType::UndefinedInstruction)
                    } else {
                        self.cpu.vfp.fpscr = Fpscr::from_bits(value);
                        ExecResult::Continue
                    }
                }
                8 => {
                    self.cpu.vfp.fpexc = value;
                    ExecResult::Continue
                }
                _ => ExecResult::Undefined,
            };
        }

        if cp == 15 {
            let crm = (insn.raw & 0xF) as u8;
            let opc2 = ((insn.raw >> 5) & 0x7) as u8;
            let value = self.reg(t);
            // WFI (MCR p15, 0, Rt, c7, c0, 4): ARMv6 wait-for-interrupt.
            if opc1 == 0 && reg == 7 && crm == 0 && opc2 == 4 {
                self.cpu.is_halted = true;
                return ExecResult::Halt;
            }
            let enc = crate::arm::sysreg::Cp15Encoding::new(reg, opc1, crm, opc2);
            // Cache/TLB maintenance (CRn 7/8) and unmodelled registers are
            // accepted as no-ops; everything modelled lands in Cp15State.
            let _ = self.cpu.cp15.write(enc, value);
            return ExecResult::Continue;
        }

        // For now, just consume the value (would write to coprocessor)
        let _value = self.reg(t);

        ExecResult::Continue
    }

    fn exec_mrc(&mut self, insn: &DecodedInsn) -> ExecResult {
        let t = ((insn.raw >> 12) & 0xF) as usize;
        let cp = ((insn.raw >> 8) & 0xF) as u8;
        let opc1 = ((insn.raw >> 21) & 7) as u8;
        let reg = ((insn.raw >> 16) & 0xF) as u8;

        if cp == 10 && opc1 == 0b111 {
            if t == 15 && reg != 1 {
                return ExecResult::Undefined;
            }
            let value = match reg {
                0 => self.cpu.vfp.fpsid,
                1 => {
                    if !self.cpu.vfp.is_enabled() {
                        return ExecResult::Exception(ExceptionType::UndefinedInstruction);
                    }
                    self.cpu.vfp.fpscr.bits()
                }
                5 => self.cpu.vfp.mvfr2,
                6 => self.cpu.vfp.mvfr1,
                7 => self.cpu.vfp.mvfr0,
                8 => self.cpu.vfp.fpexc,
                _ => return ExecResult::Undefined,
            };
            if t == 15 && reg == 1 {
                self.cpu.cpsr.n = (value & (1 << 31)) != 0;
                self.cpu.cpsr.z = (value & (1 << 30)) != 0;
                self.cpu.cpsr.c = (value & (1 << 29)) != 0;
                self.cpu.cpsr.v = (value & (1 << 28)) != 0;
            } else if t != 15 {
                self.cpu.regs[t] = value;
            }
            return ExecResult::Continue;
        }

        if cp == 15 {
            let crm = (insn.raw & 0xF) as u8;
            let opc2 = ((insn.raw >> 5) & 0x7) as u8;
            let enc = crate::arm::sysreg::Cp15Encoding::new(reg, opc1, crm, opc2);
            let value = self.cpu.cp15.read(enc).unwrap_or(0);
            if t != 15 {
                self.cpu.regs[t] = value;
            } else {
                // MRC with Rt=15 moves bits[31:28] into the CPSR flags.
                self.cpu.cpsr.n = (value & (1 << 31)) != 0;
                self.cpu.cpsr.z = (value & (1 << 30)) != 0;
                self.cpu.cpsr.c = (value & (1 << 29)) != 0;
                self.cpu.cpsr.v = (value & (1 << 28)) != 0;
            }
            return ExecResult::Continue;
        }

        // For now, return 0 (would read from coprocessor)
        if t != 15 {
            self.cpu.regs[t] = 0;
        }

        ExecResult::Continue
    }

    fn exec_vldr(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((addr, size, d)) = self.decode_vfp_mem(insn) else {
            return ExecResult::Undefined;
        };
        match size {
            16 => match self.mem.read_halfword(addr) {
                Ok(bits) => {
                    self.cpu.vfp.write_s_bits(d, bits as u32);
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            },
            32 => match self.mem.read_word(addr) {
                Ok(bits) => {
                    self.cpu.vfp.write_s_bits(d, bits);
                    ExecResult::Continue
                }
                Err(e) => ExecResult::MemoryFault(e),
            },
            64 => {
                let lo = match self.mem.read_word(addr) {
                    Ok(v) => v,
                    Err(e) => return ExecResult::MemoryFault(e),
                };
                let hi = match self.mem.read_word(addr.wrapping_add(4)) {
                    Ok(v) => v,
                    Err(e) => return ExecResult::MemoryFault(e),
                };
                self.cpu
                    .vfp
                    .write_d_bits(d, ((hi as u64) << 32) | lo as u64);
                ExecResult::Continue
            }
            _ => ExecResult::Undefined,
        }
    }

    fn exec_vstr(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((addr, size, d)) = self.decode_vfp_mem(insn) else {
            return ExecResult::Undefined;
        };
        match size {
            16 => match self
                .mem
                .write_halfword(addr, self.cpu.vfp.read_s_bits(d) as u16)
            {
                Ok(()) => ExecResult::Continue,
                Err(e) => ExecResult::MemoryFault(e),
            },
            32 => match self.mem.write_word(addr, self.cpu.vfp.read_s_bits(d)) {
                Ok(()) => ExecResult::Continue,
                Err(e) => ExecResult::MemoryFault(e),
            },
            64 => {
                let bits = self.cpu.vfp.read_d_bits(d);
                if let Err(e) = self.mem.write_word(addr, bits as u32) {
                    return ExecResult::MemoryFault(e);
                }
                if let Err(e) = self
                    .mem
                    .write_word(addr.wrapping_add(4), (bits >> 32) as u32)
                {
                    return ExecResult::MemoryFault(e);
                }
                ExecResult::Continue
            }
            _ => ExecResult::Undefined,
        }
    }

    fn exec_vldm(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((addr, final_addr, size, first, count, writeback, rn)) =
            self.decode_vfp_block_mem(insn)
        else {
            return ExecResult::Undefined;
        };

        let mut current = addr;
        for index in 0..count {
            let reg = first.wrapping_add(index);
            match size {
                32 => {
                    let bits = match self.mem.read_word(current) {
                        Ok(v) => v,
                        Err(e) => return ExecResult::MemoryFault(e),
                    };
                    self.cpu.vfp.write_s_bits(reg, bits);
                    current = current.wrapping_add(4);
                }
                64 => {
                    let lo = match self.mem.read_word(current) {
                        Ok(v) => v,
                        Err(e) => return ExecResult::MemoryFault(e),
                    };
                    let hi = match self.mem.read_word(current.wrapping_add(4)) {
                        Ok(v) => v,
                        Err(e) => return ExecResult::MemoryFault(e),
                    };
                    self.cpu
                        .vfp
                        .write_d_bits(reg, ((hi as u64) << 32) | lo as u64);
                    current = current.wrapping_add(8);
                }
                _ => return ExecResult::Undefined,
            }
        }

        if writeback {
            self.cpu.regs[rn] = final_addr;
        }
        ExecResult::Continue
    }

    fn exec_vstm(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((addr, final_addr, size, first, count, writeback, rn)) =
            self.decode_vfp_block_mem(insn)
        else {
            return ExecResult::Undefined;
        };

        let mut current = addr;
        for index in 0..count {
            let reg = first.wrapping_add(index);
            match size {
                32 => {
                    if let Err(e) = self.mem.write_word(current, self.cpu.vfp.read_s_bits(reg)) {
                        return ExecResult::MemoryFault(e);
                    }
                    current = current.wrapping_add(4);
                }
                64 => {
                    let bits = self.cpu.vfp.read_d_bits(reg);
                    if let Err(e) = self.mem.write_word(current, bits as u32) {
                        return ExecResult::MemoryFault(e);
                    }
                    if let Err(e) = self
                        .mem
                        .write_word(current.wrapping_add(4), (bits >> 32) as u32)
                    {
                        return ExecResult::MemoryFault(e);
                    }
                    current = current.wrapping_add(8);
                }
                _ => return ExecResult::Undefined,
            }
        }

        if writeback {
            self.cpu.regs[rn] = final_addr;
        }
        ExecResult::Continue
    }

    fn exec_vadd_vsub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if Self::is_neon_fp_add_sub_shape(insn.raw) {
            self.exec_neon_fp_add_sub(insn)
        } else if Self::neon_integer_add_sub_size(insn).is_some() {
            self.exec_neon_integer_add_sub(insn)
        } else {
            self.exec_vfp_binop(insn)
        }
    }

    fn is_neon_fp_add_sub_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 24) & 1) == 0
            && ((raw >> 23) & 1) == 0
            && ((raw >> 8) & 0xF) == 0b1101
            && ((raw >> 4) & 1) == 0
    }

    fn exec_neon_fp_add_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_fp_add_sub_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let size = if ((insn.raw >> 20) & 1) == 0 {
            NeonSize::S32
        } else {
            NeonSize::H16
        };
        let ebytes = (size.bits() / 8) as u8;

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match size {
                    NeonSize::S32 => {
                        let n_val = f32::from_bits(n_elem as u32);
                        let m_val = f32::from_bits(m_elem as u32);
                        u64::from(
                            match insn.mnemonic {
                                Mnemonic::VADD => vadd_f32(n_val, m_val, fpscr),
                                Mnemonic::VSUB => vsub_f32(n_val, m_val, fpscr),
                                _ => return ExecResult::Undefined,
                            }
                            .to_bits(),
                        )
                    }
                    NeonSize::H16 => {
                        let n_val = n_elem as u16;
                        let m_val = m_elem as u16;
                        u64::from(match insn.mnemonic {
                            Mnemonic::VADD => vadd_f16_bits(n_val, m_val, fpscr),
                            Mnemonic::VSUB => vsub_f16_bits(n_val, m_val, fpscr),
                            _ => return ExecResult::Undefined,
                        })
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn neon_integer_add_sub_size(insn: &DecodedInsn) -> Option<NeonSize> {
        if !matches!(insn.mnemonic, Mnemonic::VADD | Mnemonic::VSUB)
            || ((insn.raw >> 28) & 0xF) != 0xF
            || !matches!((insn.raw >> 24) & 0xFF, 0xF2 | 0xF3)
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 8) & 0xF) != 0b1000
            || ((insn.raw >> 4) & 1) != 0
        {
            return None;
        }

        match (insn.raw >> 20) & 0x3 {
            0 => Some(NeonSize::B8),
            1 => Some(NeonSize::H16),
            2 => Some(NeonSize::S32),
            3 => Some(NeonSize::D64),
            _ => None,
        }
    }

    fn exec_neon_integer_add_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some(size) = Self::neon_integer_add_sub_size(insn) else {
            return ExecResult::Undefined;
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let n_bits = self.cpu.vfp.read_d_bits(n + index);
            let m_bits = self.cpu.vfp.read_d_bits(m + index);
            let result = match insn.mnemonic {
                Mnemonic::VADD => vadd_i(n_bits, m_bits, size),
                Mnemonic::VSUB => vsub_i(n_bits, m_bits, size),
                _ => return ExecResult::Undefined,
            };
            self.cpu.vfp.write_d_bits(d + index, result);
        }

        ExecResult::Continue
    }

    fn exec_neon_logical_register(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if Self::is_neon_modified_immediate_shape(insn.raw) {
            return self.exec_neon_modified_immediate(insn);
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let d_bits = self.cpu.vfp.read_d_bits(d + index);
            let n_bits = self.cpu.vfp.read_d_bits(n + index);
            let m_bits = self.cpu.vfp.read_d_bits(m + index);
            let result = match insn.mnemonic {
                Mnemonic::VAND => vand(n_bits, m_bits),
                Mnemonic::VBIC => vbic(n_bits, m_bits),
                Mnemonic::VORR => vorr(n_bits, m_bits),
                Mnemonic::VORN => vorn(n_bits, m_bits),
                Mnemonic::VEOR => veor(n_bits, m_bits),
                Mnemonic::VBSL => (n_bits & d_bits) | (m_bits & !d_bits),
                Mnemonic::VBIT => (n_bits & m_bits) | (d_bits & !m_bits),
                Mnemonic::VBIF => (d_bits & m_bits) | (n_bits & !m_bits),
                _ => return ExecResult::Undefined,
            };
            self.cpu.vfp.write_d_bits(d + index, result);
        }

        ExecResult::Continue
    }

    fn exec_neon_vmvn_register(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if Self::is_neon_modified_immediate_shape(insn.raw) {
            return self.exec_neon_modified_immediate(insn);
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let bits = self.cpu.vfp.read_d_bits(m + index);
            self.cpu.vfp.write_d_bits(d + index, vmvn(bits));
        }

        ExecResult::Continue
    }

    fn exec_neon_vrev_register(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let op = (insn.raw >> 7) & 0x3;
        let container_bits = match insn.mnemonic {
            Mnemonic::VREV64 if op == 0b00 => 64,
            Mnemonic::VREV32 if op == 0b01 => 32,
            Mnemonic::VREV16 if op == 0b10 => 16,
            _ => return ExecResult::Undefined,
        };
        if op + ((insn.raw >> 18) & 0x3) >= 3 {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let bits = self.cpu.vfp.read_d_bits(m + index);
            self.cpu
                .vfp
                .write_d_bits(d + index, vrev(bits, size, container_bits));
        }

        ExecResult::Continue
    }

    fn exec_neon_vswp(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        if ((insn.raw >> 18) & 0x3) != 0 {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }
        if d == m {
            return ExecResult::Continue;
        }

        for index in 0..regs {
            let d_bits = self.cpu.vfp.read_d_bits(d + index);
            let m_bits = self.cpu.vfp.read_d_bits(m + index);
            self.cpu.vfp.write_d_bits(d + index, m_bits);
            self.cpu.vfp.write_d_bits(m + index, d_bits);
        }

        ExecResult::Continue
    }

    fn exec_neon_vdup(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let scalar_form = (insn.raw >> 28) == 0xF;
        let (d, regs, ebytes, scalar) = if scalar_form {
            let imm4 = ((insn.raw >> 16) & 0xF) as u8;
            let q = ((insn.raw >> 6) & 1) != 0;
            let d_bit = ((insn.raw >> 22) & 1) as u8;
            let vd = ((insn.raw >> 12) & 0xF) as u8;
            let m_bit = ((insn.raw >> 5) & 1) as u8;
            let vm = (insn.raw & 0xF) as u8;
            let (ebytes, lane) = match imm4 {
                imm if (imm & 0b0001) != 0 => (1, imm >> 1),
                imm if (imm & 0b0011) == 0b0010 => (2, imm >> 2),
                imm if (imm & 0b0111) == 0b0100 => (4, imm >> 3),
                _ => return ExecResult::Undefined,
            };

            let d = (d_bit << 4) | vd;
            let m = (m_bit << 4) | vm;
            let regs = if q { 2 } else { 1 };
            if (q && (d & 1) != 0) || d + regs > 32 || lane as usize >= 8 / ebytes as usize {
                return ExecResult::Undefined;
            }
            (d, regs, ebytes, self.neon_read_d_elem_u64(m, lane, ebytes))
        } else {
            let b = (insn.raw >> 22) & 1;
            let e = (insn.raw >> 5) & 1;
            let q = ((insn.raw >> 21) & 1) != 0;
            let d_bit = ((insn.raw >> 7) & 1) as u8;
            let vd = ((insn.raw >> 16) & 0xF) as u8;
            let rt = ((insn.raw >> 12) & 0xF) as u8;
            let ebytes = match (b, e) {
                (0, 0) => 4,
                (0, 1) => 2,
                (1, 0) => 1,
                _ => return ExecResult::Undefined,
            };

            let d = (d_bit << 4) | vd;
            let regs = if q { 2 } else { 1 };
            if rt == 15 || (q && (d & 1) != 0) || d + regs > 32 {
                return ExecResult::Undefined;
            }
            (d, regs, ebytes, self.cpu.regs[rt as usize] as u64)
        };

        let lane_count = 8 / ebytes;
        for reg in 0..regs {
            for lane in 0..lane_count {
                self.neon_write_d_elem_u64(d + reg, lane as u8, ebytes, scalar);
            }
        }

        ExecResult::Continue
    }

    fn exec_neon_pairwise_permute(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }
        if d == m {
            return ExecResult::Continue;
        }

        match insn.mnemonic {
            Mnemonic::VTRN => {
                let elements = size.elements_per_d();
                let half = elements / 2;
                for index in 0..regs {
                    let d_elems = self.neon_read_vector_elements(d + index, 1, ebytes);
                    let m_elems = self.neon_read_vector_elements(m + index, 1, ebytes);
                    let mut out_d = d_elems.clone();
                    let mut out_m = m_elems.clone();
                    for elem in 0..half {
                        out_d[(2 * elem) + 1] = m_elems[2 * elem];
                        out_m[2 * elem] = d_elems[(2 * elem) + 1];
                    }
                    self.neon_write_vector_elements(d + index, 1, ebytes, &out_d);
                    self.neon_write_vector_elements(m + index, 1, ebytes, &out_m);
                }
            }
            Mnemonic::VUZP => {
                if !q && size == NeonSize::S32 {
                    return ExecResult::Undefined;
                }
                let d_elems = self.neon_read_vector_elements(d, regs, ebytes);
                let m_elems = self.neon_read_vector_elements(m, regs, ebytes);
                let mut zipped = Vec::with_capacity(d_elems.len() + m_elems.len());
                zipped.extend_from_slice(&d_elems);
                zipped.extend_from_slice(&m_elems);

                let elements = d_elems.len();
                let mut out_d = Vec::with_capacity(elements);
                let mut out_m = Vec::with_capacity(elements);
                for elem in 0..elements {
                    out_d.push(zipped[2 * elem]);
                    out_m.push(zipped[(2 * elem) + 1]);
                }
                self.neon_write_vector_elements(d, regs, ebytes, &out_d);
                self.neon_write_vector_elements(m, regs, ebytes, &out_m);
            }
            Mnemonic::VZIP => {
                if !q && size == NeonSize::S32 {
                    return ExecResult::Undefined;
                }
                let d_elems = self.neon_read_vector_elements(d, regs, ebytes);
                let m_elems = self.neon_read_vector_elements(m, regs, ebytes);
                let elements = d_elems.len();
                let mut zipped = Vec::with_capacity(elements * 2);
                for elem in 0..elements {
                    zipped.push(d_elems[elem]);
                    zipped.push(m_elems[elem]);
                }

                self.neon_write_vector_elements(d, regs, ebytes, &zipped[..elements]);
                self.neon_write_vector_elements(m, regs, ebytes, &zipped[elements..]);
            }
            _ => return ExecResult::Undefined,
        }

        ExecResult::Continue
    }

    fn exec_neon_pairwise_integer(&mut self, insn: &DecodedInsn) -> ExecResult {
        if Self::is_neon_fp_pairwise_shape(insn.raw) {
            return self.exec_neon_fp_pairwise(insn);
        }

        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001 || ((insn.raw >> 23) & 1) != 0 {
            return ExecResult::Undefined;
        }

        let valid_shape = matches!(
            (
                ((insn.raw >> 8) & 0xF),
                ((insn.raw >> 4) & 1),
                insn.mnemonic
            ),
            (0b1010, 0, Mnemonic::VPMAX)
                | (0b1010, 1, Mnemonic::VPMIN)
                | (0b1011, 1, Mnemonic::VPADD)
        );
        if !valid_shape || ((insn.raw >> 6) & 1) != 0 {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if d >= 32 || n >= 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let n_elements = self.neon_read_vector_elements_u64(n, 1, ebytes);
        let m_elements = self.neon_read_vector_elements_u64(m, 1, ebytes);
        let half = n_elements.len() / 2;
        let mut out = Vec::with_capacity(n_elements.len());

        for elements in [&n_elements, &m_elements] {
            for pair in 0..half {
                let lhs = elements[2 * pair];
                let rhs = elements[(2 * pair) + 1];
                let result = match insn.mnemonic {
                    Mnemonic::VPADD => lhs.wrapping_add(rhs),
                    Mnemonic::VPMAX if unsigned => lhs.max(rhs),
                    Mnemonic::VPMIN if unsigned => lhs.min(rhs),
                    Mnemonic::VPMAX => {
                        let lhs = Self::neon_sign_extend_elem_u64(lhs, size.bits());
                        let rhs = Self::neon_sign_extend_elem_u64(rhs, size.bits());
                        Self::neon_pack_signed_elem_i128(lhs.max(rhs), size.bits())
                    }
                    Mnemonic::VPMIN => {
                        let lhs = Self::neon_sign_extend_elem_u64(lhs, size.bits());
                        let rhs = Self::neon_sign_extend_elem_u64(rhs, size.bits());
                        Self::neon_pack_signed_elem_i128(lhs.min(rhs), size.bits())
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
        }

        self.neon_write_vector_elements_u64(d, 1, ebytes, &out);
        ExecResult::Continue
    }

    fn is_neon_fp_pairwise_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 24) & 1) == 1
            && ((raw >> 23) & 1) == 0
            && matches!(
                ((raw >> 8) & 0xF, (raw >> 21) & 1),
                (0b1101, 0) | (0b1111, 0 | 1)
            )
            && ((raw >> 4) & 1) == 0
    }

    fn exec_neon_fp_pairwise(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !matches!(
            insn.mnemonic,
            Mnemonic::VPADD | Mnemonic::VPMAX | Mnemonic::VPMIN
        ) || !Self::is_neon_fp_pairwise_shape(insn.raw)
        {
            return ExecResult::Undefined;
        }
        if ((insn.raw >> 6) & 1) != 0 {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if d >= 32 || n >= 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let size = if ((insn.raw >> 20) & 1) == 0 {
            NeonSize::S32
        } else {
            NeonSize::H16
        };
        let ebytes = (size.bits() / 8) as u8;
        let n_elements = self.neon_read_vector_elements_u64(n, 1, ebytes);
        let m_elements = self.neon_read_vector_elements_u64(m, 1, ebytes);
        let fpscr = &mut self.cpu.vfp.fpscr;
        let mut out = Vec::with_capacity(n_elements.len());
        for elements in [&n_elements, &m_elements] {
            for pair in elements.chunks_exact(2) {
                let result = match size {
                    NeonSize::S32 => {
                        let lhs = f32::from_bits(pair[0] as u32);
                        let rhs = f32::from_bits(pair[1] as u32);
                        u64::from(match insn.mnemonic {
                            Mnemonic::VPADD => vadd_f32(lhs, rhs, fpscr).to_bits(),
                            Mnemonic::VPMAX => Self::neon_fpmax_f32_bits(lhs, rhs),
                            Mnemonic::VPMIN => Self::neon_fpmin_f32_bits(lhs, rhs),
                            _ => return ExecResult::Undefined,
                        })
                    }
                    NeonSize::H16 => {
                        let lhs = pair[0] as u16;
                        let rhs = pair[1] as u16;
                        u64::from(match insn.mnemonic {
                            Mnemonic::VPADD => vadd_f16_bits(lhs, rhs, fpscr),
                            Mnemonic::VPMAX => {
                                let lhs_f = vcvt_f32_f16_bits(lhs);
                                let rhs_f = vcvt_f32_f16_bits(rhs);
                                vcvt_f16_bits_f32(
                                    f32::from_bits(Self::neon_fpmax_f32_bits(lhs_f, rhs_f)),
                                    fpscr,
                                )
                            }
                            Mnemonic::VPMIN => {
                                let lhs_f = vcvt_f32_f16_bits(lhs);
                                let rhs_f = vcvt_f32_f16_bits(rhs);
                                vcvt_f16_bits_f32(
                                    f32::from_bits(Self::neon_fpmin_f32_bits(lhs_f, rhs_f)),
                                    fpscr,
                                )
                            }
                            _ => return ExecResult::Undefined,
                        })
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
        }

        self.neon_write_vector_elements_u64(d, 1, ebytes, &out);
        ExecResult::Continue
    }

    fn exec_neon_pairwise_add_long(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 23) != 0b111100111
            || ((insn.raw >> 20) & 0x3) != 0b11
            || ((insn.raw >> 16) & 0x3) != 0
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let op = (insn.raw >> 7) & 0x1F;
        let (accumulate, unsigned) = match (insn.mnemonic, op & 0x1E) {
            (Mnemonic::VPADDL, 0b00100) => (false, (op & 1) != 0),
            (Mnemonic::VPADAL, 0b01100) => (true, (op & 1) != 0),
            _ => return ExecResult::Undefined,
        };

        let narrow_size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let narrow_ebytes = (narrow_size.bits() / 8) as u8;
        let wide_bits = narrow_size.bits() * 2;
        let wide_ebytes = narrow_ebytes * 2;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let elements = self.neon_read_vector_elements_u64(m, regs, narrow_ebytes);
        let old_elements = if accumulate {
            self.neon_read_vector_elements_u64(d, regs, wide_ebytes)
        } else {
            vec![0; elements.len() / 2]
        };
        let mut out = Vec::with_capacity(elements.len() / 2);
        for (pair, old) in elements.chunks_exact(2).zip(old_elements.into_iter()) {
            let lhs = pair[0];
            let rhs = pair[1];
            let sum = if unsigned {
                lhs as i128 + rhs as i128
            } else {
                Self::neon_sign_extend_elem_u64(lhs, narrow_size.bits())
                    + Self::neon_sign_extend_elem_u64(rhs, narrow_size.bits())
            };
            out.push(old.wrapping_add(Self::neon_pack_signed_elem_i128(sum, wide_bits)));
        }
        self.neon_write_vector_elements_u64(d, regs, wide_ebytes, &out);

        ExecResult::Continue
    }

    fn exec_neon_shift_immediate(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 4) & 1) != 1
        {
            return ExecResult::Undefined;
        }

        let imm = (insn.raw >> 16) & 0x3F;
        let size = match imm {
            8..=15 => NeonSize::B8,
            16..=31 => NeonSize::H16,
            32..=63 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;
        let op = match insn.mnemonic {
            Mnemonic::VSHR => 0,
            Mnemonic::VRSHR => 1,
            Mnemonic::VSRA => 2,
            Mnemonic::VRSRA => 3,
            Mnemonic::VSHL => 4,
            Mnemonic::VSLI => 5,
            Mnemonic::VSRI => 6,
            _ => return ExecResult::Undefined,
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };
        let right_shift = (size.bits() * 2) - imm;
        let left_shift = imm - size.bits();
        if matches!(
            insn.mnemonic,
            Mnemonic::VSHR | Mnemonic::VRSHR | Mnemonic::VSRA | Mnemonic::VRSRA | Mnemonic::VSRI
        ) && (right_shift == 0 || right_shift > size.bits())
        {
            return ExecResult::Undefined;
        }
        if matches!(insn.mnemonic, Mnemonic::VSHL | Mnemonic::VSLI)
            && (left_shift == 0 || left_shift > size.bits())
        {
            return ExecResult::Undefined;
        }
        let round_const = if matches!(insn.mnemonic, Mnemonic::VRSHR | Mnemonic::VRSRA) {
            1i128 << (right_shift - 1)
        } else {
            0
        };
        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let old_elements = if matches!(
                insn.mnemonic,
                Mnemonic::VSRA | Mnemonic::VRSRA | Mnemonic::VSLI | Mnemonic::VSRI
            ) {
                self.neon_read_vector_elements_u64(d + reg, 1, ebytes)
            } else {
                vec![0; elements.len()]
            };
            let mut out = Vec::with_capacity(elements.len());
            for (elem, old_elem) in elements.into_iter().zip(old_elements.into_iter()) {
                let result = match op {
                    0..=3 => {
                        if unsigned {
                            let shifted = ((elem as i128 + round_const) >> right_shift) as u64;
                            if matches!(op, 2 | 3) {
                                old_elem.wrapping_add(shifted) & mask
                            } else {
                                shifted
                            }
                        } else {
                            let value =
                                Self::neon_sign_extend_elem_u64(elem, size.bits()) + round_const;
                            let shifted =
                                Self::neon_pack_signed_elem_i128(value >> right_shift, size.bits());
                            if matches!(op, 2 | 3) {
                                old_elem.wrapping_add(shifted) & mask
                            } else {
                                shifted
                            }
                        }
                    }
                    4 => (elem << left_shift) & mask,
                    5 => {
                        let insert_mask = (mask << left_shift) & mask;
                        (old_elem & !insert_mask) | ((elem << left_shift) & insert_mask)
                    }
                    6 => {
                        let insert_mask = mask >> right_shift;
                        (old_elem & !insert_mask) | ((elem >> right_shift) & insert_mask)
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_vshl(&mut self, insn: &DecodedInsn) -> ExecResult {
        if (insn.raw >> 25) == 0b1111001
            && ((insn.raw >> 23) & 1) == 0
            && ((insn.raw >> 8) & 0xF) == 0b0100
            && ((insn.raw >> 4) & 1) == 0
        {
            return self.exec_neon_shift_register(insn);
        }

        self.exec_neon_shift_immediate(insn)
    }

    fn exec_vqshl(&mut self, insn: &DecodedInsn) -> ExecResult {
        if (insn.raw >> 25) == 0b1111001
            && ((insn.raw >> 23) & 1) == 0
            && ((insn.raw >> 8) & 0xF) == 0b0100
            && ((insn.raw >> 4) & 1) == 1
        {
            return self.exec_neon_shift_register(insn);
        }

        self.exec_neon_saturating_shift_left_immediate(insn)
    }

    fn exec_neon_saturating_shift_left_immediate(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 4) & 1) != 1
        {
            return ExecResult::Undefined;
        }

        let op8 = (insn.raw >> 8) & 0xF;
        let unsigned_bit = ((insn.raw >> 24) & 1) != 0;
        let signed_to_unsigned = match (insn.mnemonic, op8, unsigned_bit) {
            (Mnemonic::VQSHL, 0b0111, _) => false,
            (Mnemonic::VQSHLU, 0b0110, true) => true,
            _ => return ExecResult::Undefined,
        };

        let imm = (insn.raw >> 16) & 0x3F;
        let size = match imm {
            8..=15 => NeonSize::B8,
            16..=31 => NeonSize::H16,
            32..=63 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let shift = imm - size.bits();
        if shift == 0 || shift > size.bits() {
            return ExecResult::Undefined;
        }
        let ebytes = (size.bits() / 8) as u8;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let (result, saturated) = if signed_to_unsigned {
                    let value = Self::neon_sign_extend_elem_u64(elem, size.bits()) << shift;
                    Self::neon_unsigned_saturate(value, size.bits())
                } else if unsigned_bit {
                    Self::neon_unsigned_saturate((elem as i128) << shift, size.bits())
                } else {
                    let value = Self::neon_sign_extend_elem_u64(elem, size.bits()) << shift;
                    let (value, saturated) = Self::neon_signed_saturate_i128(value, size.bits());
                    (
                        Self::neon_pack_signed_elem_i128(value, size.bits()),
                        saturated,
                    )
                };
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_shift_register(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001 || ((insn.raw >> 23) & 1) != 0 {
            return ExecResult::Undefined;
        }

        let saturating = ((insn.raw >> 4) & 1) != 0;
        let rounding = match (insn.mnemonic, (insn.raw >> 8) & 0xF, saturating) {
            (Mnemonic::VSHL, 0b0100, false) => false,
            (Mnemonic::VRSHL, 0b0101, false) => true,
            (Mnemonic::VQSHL, 0b0100, true) => false,
            (Mnemonic::VQRSHL, 0b0101, true) => true,
            _ => return ExecResult::Undefined,
        };
        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };
        for reg in 0..regs {
            let shift_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let value_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(value_elements.len());
            for (shift_elem, value_elem) in
                shift_elements.into_iter().zip(value_elements.into_iter())
            {
                let shift = Self::neon_sign_extend_elem_u64(shift_elem, size.bits());
                let result = if shift >= size.bits() as i128 {
                    if saturating {
                        self.cpu.vfp.fpscr.set_qc(true);
                        if unsigned {
                            mask
                        } else if Self::neon_sign_extend_elem_u64(value_elem, size.bits()) < 0 {
                            Self::neon_pack_signed_elem_i128(
                                -(1i128 << (size.bits() - 1)),
                                size.bits(),
                            )
                        } else {
                            Self::neon_pack_signed_elem_i128(
                                (1i128 << (size.bits() - 1)) - 1,
                                size.bits(),
                            )
                        }
                    } else {
                        0
                    }
                } else if shift >= 0 {
                    if saturating {
                        if unsigned {
                            let value = (value_elem as i128) << (shift as u32);
                            let (result, saturated) =
                                Self::neon_unsigned_saturate(value, size.bits());
                            if saturated {
                                self.cpu.vfp.fpscr.set_qc(true);
                            }
                            result
                        } else {
                            let value = Self::neon_sign_extend_elem_u64(value_elem, size.bits())
                                << (shift as u32);
                            let (result, saturated) =
                                Self::neon_signed_saturate_i128(value, size.bits());
                            if saturated {
                                self.cpu.vfp.fpscr.set_qc(true);
                            }
                            Self::neon_pack_signed_elem_i128(result, size.bits())
                        }
                    } else {
                        (value_elem << (shift as u32)) & mask
                    }
                } else {
                    let rshift = (-shift) as u32;
                    if rshift > size.bits() {
                        if unsigned {
                            0
                        } else if Self::neon_sign_extend_elem_u64(value_elem, size.bits()) < 0 {
                            mask
                        } else {
                            0
                        }
                    } else if unsigned {
                        let add = if rounding && rshift > 0 {
                            1u64 << (rshift - 1)
                        } else {
                            0
                        };
                        ((value_elem.wrapping_add(add)) >> rshift) & mask
                    } else {
                        let add = if rounding && rshift > 0 {
                            1i128 << (rshift - 1)
                        } else {
                            0
                        };
                        let value = Self::neon_sign_extend_elem_u64(value_elem, size.bits()) + add;
                        Self::neon_pack_signed_elem_i128(value >> rshift, size.bits())
                    }
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_shift_narrow_immediate(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 4) & 1) != 1
        {
            return ExecResult::Undefined;
        }

        let imm = (insn.raw >> 16) & 0x3F;
        let dest_size = match imm {
            8..=15 => NeonSize::B8,
            16..=31 => NeonSize::H16,
            32..=63 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let source_bits = dest_size.bits() * 2;
        let shift = source_bits - imm;
        if shift == 0 || shift > source_bits {
            return ExecResult::Undefined;
        }
        let dest_ebytes = (dest_size.bits() / 8) as u8;
        let source_ebytes = dest_ebytes * 2;
        let op8 = (insn.raw >> 8) & 0xF;
        let unsigned_bit = ((insn.raw >> 24) & 1) != 0;
        let rounding_bit = ((insn.raw >> 6) & 1) != 0;
        let (rounding, saturating, unsigned_source, unsigned_dest) = match insn.mnemonic {
            Mnemonic::VSHRN if op8 == 0b1000 && !unsigned_bit && !rounding_bit => {
                (false, false, true, true)
            }
            Mnemonic::VRSHRN if op8 == 0b1000 && !unsigned_bit && rounding_bit => {
                (true, false, true, true)
            }
            Mnemonic::VQSHRUN if op8 == 0b1000 && unsigned_bit && !rounding_bit => {
                (false, true, false, true)
            }
            Mnemonic::VQRSHRUN if op8 == 0b1000 && unsigned_bit && rounding_bit => {
                (true, true, false, true)
            }
            Mnemonic::VQSHRN if op8 == 0b1001 && !rounding_bit => {
                (false, true, unsigned_bit, unsigned_bit)
            }
            Mnemonic::VQRSHRN if op8 == 0b1001 && rounding_bit => {
                (true, true, unsigned_bit, unsigned_bit)
            }
            _ => return ExecResult::Undefined,
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if d >= 32 || (m & 1) != 0 || m + 2 > 32 {
            return ExecResult::Undefined;
        }

        let round_const = if rounding { 1i128 << (shift - 1) } else { 0 };
        let elements = self.neon_read_vector_elements_u64(m, 2, source_ebytes);
        let mut out = Vec::with_capacity(elements.len());
        for elem in elements {
            let result = if saturating {
                let shifted = if unsigned_source {
                    ((elem as i128) + round_const) >> shift
                } else {
                    (Self::neon_sign_extend_elem_u64(elem, source_bits) + round_const) >> shift
                };
                let (result, saturated) = if unsigned_dest {
                    Self::neon_unsigned_saturate(shifted, dest_size.bits())
                } else {
                    let (result, saturated) =
                        Self::neon_signed_saturate_i128(shifted, dest_size.bits());
                    (
                        Self::neon_pack_signed_elem_i128(result, dest_size.bits()),
                        saturated,
                    )
                };
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                result
            } else {
                ((elem as u128).wrapping_add(round_const as u128) >> shift) as u64
            };
            out.push(result);
        }
        self.neon_write_vector_elements_u64(d, 1, dest_ebytes, &out);

        ExecResult::Continue
    }

    fn exec_neon_widen_move(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 8) & 0xF) != 0b1010
            || ((insn.raw >> 4) & 1) != 1
        {
            return ExecResult::Undefined;
        }

        let narrow_size = match (insn.raw >> 16) & 0x3F {
            8 => NeonSize::B8,
            16 => NeonSize::H16,
            32 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let narrow_ebytes = (narrow_size.bits() / 8) as u8;
        let wide_bits = narrow_size.bits() * 2;
        let wide_ebytes = narrow_ebytes * 2;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if (d & 1) != 0 || d + 2 > 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let elements = self.neon_read_vector_elements_u64(m, 1, narrow_ebytes);
        let mut out = Vec::with_capacity(elements.len());
        for elem in elements {
            let result = if unsigned {
                elem
            } else {
                Self::neon_pack_signed_elem_i128(
                    Self::neon_sign_extend_elem_u64(elem, narrow_size.bits()),
                    wide_bits,
                )
            };
            out.push(result);
        }
        self.neon_write_vector_elements_u64(d, 2, wide_ebytes, &out);
        ExecResult::Continue
    }

    fn exec_neon_narrow_move(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 23) != 0b111100111
            || ((insn.raw >> 20) & 0x3) != 0b11
            || ((insn.raw >> 16) & 0x3) != 0b10
            || ((insn.raw >> 10) & 0x3) != 0
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let op = (insn.raw >> 7) & 0xF;
        let unsigned = ((insn.raw >> 6) & 1) != 0;
        let (saturating, unsigned_source, unsigned_dest) = match (insn.mnemonic, op) {
            (Mnemonic::VMOVN, 0b0100) if !unsigned => (false, true, true),
            (Mnemonic::VQMOVN, 0b0101) if !unsigned => (true, false, false),
            (Mnemonic::VQMOVN, 0b0101) => (true, true, true),
            (Mnemonic::VQMOVUN, 0b0100) if unsigned => (true, false, true),
            _ => return ExecResult::Undefined,
        };

        let dest_size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let dest_bits = dest_size.bits();
        let source_bits = dest_bits * 2;
        let dest_ebytes = (dest_bits / 8) as u8;
        let source_ebytes = dest_ebytes * 2;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if d >= 32 || (m & 1) != 0 || m + 2 > 32 {
            return ExecResult::Undefined;
        }

        let elements = self.neon_read_vector_elements_u64(m, 2, source_ebytes);
        let mut out = Vec::with_capacity(elements.len());
        for elem in elements {
            let result = if saturating {
                let value = if unsigned_source {
                    elem as i128
                } else {
                    Self::neon_sign_extend_elem_u64(elem, source_bits)
                };
                let (result, saturated) = if unsigned_dest {
                    Self::neon_unsigned_saturate(value, dest_bits)
                } else {
                    let (value, saturated) = Self::neon_signed_saturate_i128(value, dest_bits);
                    (
                        Self::neon_pack_signed_elem_i128(value, dest_bits),
                        saturated,
                    )
                };
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                result
            } else {
                elem
            };
            out.push(result);
        }
        self.neon_write_vector_elements_u64(d, 1, dest_ebytes, &out);
        ExecResult::Continue
    }

    fn exec_neon_saturating_abs_neg(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let elements = self.neon_read_vector_elements(m + index, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let value = Self::neon_sign_extend_elem(elem, size.bits());
                let (result, saturated) = match insn.mnemonic {
                    Mnemonic::VQABS => Self::neon_signed_saturate(value.abs(), size.bits()),
                    Mnemonic::VQNEG => Self::neon_signed_saturate(-value, size.bits()),
                    _ => return ExecResult::Undefined,
                };
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                out.push(Self::neon_pack_signed_elem(result, size.bits()));
            }
            self.neon_write_vector_elements(d + index, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_abs_neg(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_abs_neg(insn.raw) {
            return ExecResult::Undefined;
        }

        let size_bits = match (insn.raw >> 18) & 0x3 {
            0b00 => 8,
            0b01 => 16,
            0b10 => 32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size_bits / 8) as u8;
        let op = (insn.raw >> 7) & 0xF;
        let fp = op >= 0b1110;
        if fp && !matches!(size_bits, 16 | 32) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + index, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let result = if fp {
                    let sign_mask = 1u64 << (size_bits - 1);
                    let value_mask = if size_bits == 32 {
                        u64::from(u32::MAX)
                    } else {
                        u64::from(u16::MAX)
                    };
                    match insn.mnemonic {
                        Mnemonic::VABS => elem & !sign_mask & value_mask,
                        Mnemonic::VNEG => (elem ^ sign_mask) & value_mask,
                        _ => return ExecResult::Undefined,
                    }
                } else {
                    let value = Self::neon_sign_extend_elem_u64(elem, size_bits);
                    let result = match insn.mnemonic {
                        Mnemonic::VABS => value.abs(),
                        Mnemonic::VNEG => -value,
                        _ => return ExecResult::Undefined,
                    };
                    Self::neon_pack_signed_elem_i128(result, size_bits)
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + index, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn is_neon_abs_neg(raw: u32) -> bool {
        if (raw >> 23) != 0b111100111
            || ((raw >> 20) & 0x3) != 0b11
            || ((raw >> 16) & 0x3) != 0b01
            || ((raw >> 11) & 1) != 0
            || ((raw >> 4) & 1) != 0
        {
            return false;
        }

        let size = (raw >> 18) & 0x3;
        match (raw >> 7) & 0xF {
            0b0110 | 0b0111 => size != 0b11,
            0b1110 | 0b1111 => matches!(size, 0b01 | 0b10),
            _ => false,
        }
    }

    fn is_neon_vrint_shape(raw: u32) -> bool {
        (raw >> 24) == 0xF3
            && ((raw >> 23) & 1) == 1
            && ((raw >> 21) & 1) == 1
            && ((raw >> 20) & 1) == 1
            && ((raw >> 16) & 0x3) == 0b10
            && ((raw >> 10) & 0x3) == 0b01
            && matches!(
                (raw >> 7) & 0x7,
                0b000 | 0b001 | 0b010 | 0b011 | 0b101 | 0b111
            )
            && ((raw >> 4) & 1) == 0
    }

    fn exec_neon_vrint(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_vrint_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let Some((mode, exact)) = self.vrint_rounding(insn.mnemonic) else {
            return ExecResult::Undefined;
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let ebytes = (size.bits() / 8) as u8;
        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let result = match size {
                    NeonSize::H16 => u64::from(vrint_f16_bits(
                        elem as u16,
                        mode,
                        exact,
                        &mut self.cpu.vfp.fpscr,
                    )),
                    NeonSize::S32 => u64::from(
                        vrint_f32(
                            f32::from_bits(elem as u32),
                            mode,
                            exact,
                            &mut self.cpu.vfp.fpscr,
                        )
                        .to_bits(),
                    ),
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_halving_add_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let result = if unsigned {
                    let lhs = n_elem;
                    let rhs = m_elem;
                    let value = match insn.mnemonic {
                        Mnemonic::VHADD => (lhs + rhs) >> 1,
                        Mnemonic::VRHADD => (lhs + rhs + 1) >> 1,
                        Mnemonic::VHSUB => ((lhs.wrapping_sub(rhs)) & mask) >> 1,
                        _ => return ExecResult::Undefined,
                    };
                    value
                } else {
                    let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                    let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                    let value = match insn.mnemonic {
                        Mnemonic::VHADD => (lhs + rhs) >> 1,
                        Mnemonic::VRHADD => (lhs + rhs + 1) >> 1,
                        Mnemonic::VHSUB => (lhs - rhs) >> 1,
                        _ => return ExecResult::Undefined,
                    };
                    Self::neon_pack_signed_elem_i128(value, size.bits())
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_saturating_add_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            0b11 => NeonSize::D64,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + index, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + index, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let (packed, saturated) = if unsigned {
                    let lhs = n_elem as i128;
                    let rhs = m_elem as i128;
                    let value = match insn.mnemonic {
                        Mnemonic::VQADD => lhs + rhs,
                        Mnemonic::VQSUB => lhs - rhs,
                        _ => return ExecResult::Undefined,
                    };
                    Self::neon_unsigned_saturate(value, size.bits())
                } else {
                    let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                    let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                    let value = match insn.mnemonic {
                        Mnemonic::VQADD => lhs + rhs,
                        Mnemonic::VQSUB => lhs - rhs,
                        _ => return ExecResult::Undefined,
                    };
                    let (result, saturated) = Self::neon_signed_saturate_i128(value, size.bits());
                    (
                        Self::neon_pack_signed_elem_i128(result, size.bits()),
                        saturated,
                    )
                };
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                out.push(packed);
            }
            self.neon_write_vector_elements_u64(d + index, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_long_wide_add_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let narrow_size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let narrow_ebytes = (narrow_size.bits() / 8) as u8;
        let wide_ebytes = narrow_ebytes * 2;
        let wide_bits = narrow_size.bits() * 2;
        let unsigned = ((insn.raw >> 24) & 1) != 0;
        let add = match insn.mnemonic {
            Mnemonic::VADDL | Mnemonic::VADDW => true,
            Mnemonic::VSUBL | Mnemonic::VSUBW => false,
            _ => return ExecResult::Undefined,
        };
        let wide_n = matches!(insn.mnemonic, Mnemonic::VADDW | Mnemonic::VSUBW);

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if (d & 1) != 0 || (wide_n && (n & 1) != 0) {
            return ExecResult::Undefined;
        }
        if d + 2 > 32 || m >= 32 || (!wide_n && n >= 32) || (wide_n && n + 2 > 32) {
            return ExecResult::Undefined;
        }

        let n_elements = if wide_n {
            self.neon_read_vector_elements_u64(n, 2, wide_ebytes)
        } else {
            self.neon_read_vector_elements_u64(n, 1, narrow_ebytes)
        };
        let m_elements = self.neon_read_vector_elements_u64(m, 1, narrow_ebytes);
        if n_elements.len() != m_elements.len() {
            return ExecResult::Undefined;
        }

        let mut out = Vec::with_capacity(n_elements.len());
        for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
            let lhs = if unsigned {
                n_elem as i128
            } else {
                let bits = if wide_n {
                    wide_bits
                } else {
                    narrow_size.bits()
                };
                Self::neon_sign_extend_elem_u64(n_elem, bits) as i128
            };
            let rhs = if unsigned {
                m_elem as i128
            } else {
                Self::neon_sign_extend_elem_u64(m_elem, narrow_size.bits()) as i128
            };
            let value = if add { lhs + rhs } else { lhs - rhs };
            out.push(Self::neon_pack_signed_elem_i128(value, wide_bits));
        }

        self.neon_write_vector_elements_u64(d, 2, wide_ebytes, &out);
        ExecResult::Continue
    }

    fn exec_neon_narrow_add_sub(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 6) & 1) != 0
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let dest_size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let dest_ebytes = (dest_size.bits() / 8) as u8;
        let source_ebytes = dest_ebytes * 2;
        let source_bits = dest_size.bits() * 2;
        let round = ((insn.raw >> 24) & 1) != 0;
        let add = match insn.mnemonic {
            Mnemonic::VADDHN | Mnemonic::VRADDHN => true,
            Mnemonic::VSUBHN | Mnemonic::VRSUBHN => false,
            _ => return ExecResult::Undefined,
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if (n & 1) != 0 || (m & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d >= 32 || n + 2 > 32 || m + 2 > 32 {
            return ExecResult::Undefined;
        }

        let mask = if source_bits == 64 {
            u64::MAX as u128
        } else {
            (1u128 << source_bits) - 1
        };
        let round_const = if round {
            1u128 << (dest_size.bits() - 1)
        } else {
            0
        };

        let n_elements = self.neon_read_vector_elements_u64(n, 2, source_ebytes);
        let m_elements = self.neon_read_vector_elements_u64(m, 2, source_ebytes);
        if n_elements.len() != m_elements.len() {
            return ExecResult::Undefined;
        }

        let mut out = Vec::with_capacity(n_elements.len());
        for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
            let value = if add {
                (n_elem as u128)
                    .wrapping_add(m_elem as u128)
                    .wrapping_add(round_const)
            } else {
                (n_elem as u128)
                    .wrapping_sub(m_elem as u128)
                    .wrapping_add(round_const)
            } & mask;
            out.push((value >> dest_size.bits()) as u64);
        }

        self.neon_write_vector_elements_u64(d, 1, dest_ebytes, &out);
        ExecResult::Continue
    }

    fn exec_vmul(&mut self, insn: &DecodedInsn) -> ExecResult {
        if Self::is_neon_fp_multiply_shape(insn.raw)
            || Self::is_neon_fp_multiply_scalar_shape(insn.raw)
        {
            return self.exec_neon_fp_multiply(insn);
        }
        if Self::is_neon_polynomial_multiply_shape(insn.raw) {
            return self.exec_neon_polynomial_multiply(insn);
        }
        if Self::is_neon_integer_multiply_shape(insn.raw)
            || Self::is_neon_integer_multiply_scalar_shape(insn.raw)
        {
            return self.exec_neon_integer_multiply(insn);
        }

        self.exec_vfp_binop(insn)
    }

    fn exec_vmla_vmls(&mut self, insn: &DecodedInsn) -> ExecResult {
        if Self::is_neon_fp_multiply_shape(insn.raw)
            || Self::is_neon_fp_multiply_scalar_shape(insn.raw)
            || Self::is_neon_fp_fma_shape(insn.raw)
        {
            return self.exec_neon_fp_multiply(insn);
        }
        if Self::is_neon_integer_multiply_shape(insn.raw)
            || Self::is_neon_integer_multiply_scalar_shape(insn.raw)
        {
            return self.exec_neon_integer_multiply(insn);
        }
        if Self::is_neon_long_multiply_shape(insn.raw) {
            return self.exec_neon_long_multiply(insn);
        }
        if Self::is_neon_long_multiply_scalar_shape(insn.raw) {
            return self.exec_neon_long_multiply(insn);
        }

        self.exec_vfp_accop(insn)
    }

    fn is_neon_integer_multiply_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 23) & 1) == 0
            && ((raw >> 8) & 0xF) == 0b1001
            && (((raw >> 4) & 1) == 0 || ((raw >> 24) & 1) == 0)
    }

    fn is_neon_fp_multiply_shape(raw: u32) -> bool {
        if (raw >> 25) != 0b1111001
            || ((raw >> 23) & 1) != 0
            || ((raw >> 8) & 0xF) != 0b1101
            || ((raw >> 4) & 1) != 1
        {
            return false;
        }

        matches!(
            (((raw >> 24) & 1) != 0, ((raw >> 21) & 1) != 0),
            (true, false) | (false, false) | (false, true)
        )
    }

    fn is_neon_fp_multiply_scalar_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 23) & 1) == 1
            && matches!((raw >> 20) & 0x3, 0b01 | 0b10)
            && ((raw >> 6) & 1) == 1
            && ((raw >> 4) & 1) == 0
            && matches!((raw >> 8) & 0xF, 0b0001 | 0b0101 | 0b1001)
    }

    fn is_neon_fp_fma_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 24) & 1) == 0
            && ((raw >> 23) & 1) == 0
            && ((raw >> 20) & 1) == 0
            && ((raw >> 8) & 0xF) == 0b1100
            && ((raw >> 4) & 1) == 1
    }

    fn is_neon_fp16_fused_multiply_long_shape(raw: u32) -> bool {
        ((raw >> 24) == 0xFC
            && ((raw >> 21) & 1) == 1
            && ((raw >> 20) & 1) == 0
            && ((raw >> 8) & 0xF) == 0b1000
            && ((raw >> 4) & 1) == 1)
            || ((raw >> 24) == 0xFE
                && ((raw >> 23) & 1) == 0
                && ((raw >> 21) & 1) == 0
                && ((raw >> 8) & 0xF) == 0b1000
                && ((raw >> 4) & 1) == 1)
    }

    fn is_neon_polynomial_multiply_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 24) & 1) == 1
            && ((raw >> 23) & 1) == 0
            && ((raw >> 20) & 0x3) == 0
            && ((raw >> 8) & 0xF) == 0b1001
            && ((raw >> 4) & 1) == 1
    }

    fn is_neon_integer_multiply_scalar_shape(raw: u32) -> bool {
        if (raw >> 25) != 0b1111001
            || ((raw >> 23) & 1) != 1
            || ((raw >> 6) & 1) != 1
            || ((raw >> 4) & 1) != 0
        {
            return false;
        }

        matches!((raw >> 8) & 0xF, 0b0000 | 0b0100 | 0b1000)
    }

    fn is_neon_long_multiply_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 23) & 1) == 1
            && ((raw >> 6) & 1) == 0
            && ((raw >> 4) & 1) == 0
            && matches!(
                (raw >> 8) & 0xF,
                0b1000 | 0b1001 | 0b1010 | 0b1011 | 0b1100 | 0b1101
            )
    }

    fn is_neon_long_multiply_scalar_shape(raw: u32) -> bool {
        if (raw >> 25) != 0b1111001
            || ((raw >> 23) & 1) != 1
            || ((raw >> 6) & 1) != 1
            || ((raw >> 4) & 1) != 0
        {
            return false;
        }

        matches!(
            (raw >> 8) & 0xF,
            0b0010 | 0b0011 | 0b0110 | 0b0111 | 0b1010 | 0b1011
        )
    }

    fn is_neon_polynomial_multiply_long_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 23) & 1) == 1
            && ((raw >> 20) & 0x3) == 0
            && ((raw >> 8) & 0xF) == 0b1110
            && ((raw >> 6) & 1) == 0
            && ((raw >> 4) & 1) == 0
    }

    fn exec_neon_fp_multiply(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let scalar = Self::is_neon_fp_multiply_scalar_shape(insn.raw);
        if !Self::is_neon_fp_multiply_shape(insn.raw)
            && !scalar
            && !Self::is_neon_fp_fma_shape(insn.raw)
        {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = if scalar {
            ((insn.raw >> 24) & 1) != 0
        } else {
            ((insn.raw >> 6) & 1) != 0
        };
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | if scalar { 0 } else { m }) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || (!scalar && m + regs > 32) {
            return ExecResult::Undefined;
        }
        let size = if scalar {
            match (insn.raw >> 20) & 0x3 {
                0b01 => NeonSize::H16,
                0b10 => NeonSize::S32,
                _ => return ExecResult::Undefined,
            }
        } else if !Self::is_neon_fp_fma_shape(insn.raw) && ((insn.raw >> 20) & 1) != 0 {
            NeonSize::H16
        } else {
            NeonSize::S32
        };
        let ebytes = (size.bits() / 8) as u8;
        let scalar_elem = if scalar {
            let (scalar_reg, scalar_index) = match size {
                NeonSize::H16 => (vm & 0x7, (m_bit << 1) | (vm >> 3)),
                NeonSize::S32 => (vm, m_bit),
                _ => return ExecResult::Undefined,
            };
            if scalar_reg >= 32 || scalar_index as usize >= size.elements_per_d() {
                return ExecResult::Undefined;
            }
            Some(self.neon_read_d_elem_u64(scalar_reg, scalar_index, ebytes))
        } else {
            None
        };

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = if let Some(elem) = scalar_elem {
                vec![elem; n_elements.len()]
            } else {
                self.neon_read_vector_elements_u64(m + reg, 1, ebytes)
            };
            let d_elements = if matches!(
                insn.mnemonic,
                Mnemonic::VMLA | Mnemonic::VMLS | Mnemonic::VFMA | Mnemonic::VFMS
            ) {
                self.neon_read_vector_elements_u64(d + reg, 1, ebytes)
            } else {
                vec![0; n_elements.len()]
            };
            let mut out = Vec::with_capacity(n_elements.len());
            for ((n_elem, m_elem), d_elem) in n_elements
                .into_iter()
                .zip(m_elements.into_iter())
                .zip(d_elements.into_iter())
            {
                let mut fpscr = self.cpu.vfp.fpscr;
                let result = match size {
                    NeonSize::S32 => {
                        let n_val = f32::from_bits(n_elem as u32);
                        let m_val = f32::from_bits(m_elem as u32);
                        u64::from(
                            match insn.mnemonic {
                                Mnemonic::VMUL => vmul_f32(n_val, m_val, &mut fpscr),
                                Mnemonic::VMLA => vmla_f32(
                                    f32::from_bits(d_elem as u32),
                                    n_val,
                                    m_val,
                                    &mut fpscr,
                                ),
                                Mnemonic::VMLS => vmls_f32(
                                    f32::from_bits(d_elem as u32),
                                    n_val,
                                    m_val,
                                    &mut fpscr,
                                ),
                                Mnemonic::VFMA => vfma_f32(
                                    f32::from_bits(d_elem as u32),
                                    n_val,
                                    m_val,
                                    &mut fpscr,
                                ),
                                Mnemonic::VFMS => vfms_f32(
                                    f32::from_bits(d_elem as u32),
                                    n_val,
                                    m_val,
                                    &mut fpscr,
                                ),
                                _ => return ExecResult::Undefined,
                            }
                            .to_bits(),
                        )
                    }
                    NeonSize::H16 => {
                        let n_val = n_elem as u16;
                        let m_val = m_elem as u16;
                        u64::from(match insn.mnemonic {
                            Mnemonic::VMUL => vmul_f16_bits(n_val, m_val, &mut fpscr),
                            Mnemonic::VMLA => {
                                vmla_f16_bits(d_elem as u16, n_val, m_val, &mut fpscr)
                            }
                            Mnemonic::VMLS => {
                                vmls_f16_bits(d_elem as u16, n_val, m_val, &mut fpscr)
                            }
                            _ => return ExecResult::Undefined,
                        })
                    }
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.fpscr = fpscr;
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_fp16_fused_multiply_long(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_fp16_fused_multiply_long_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let vector = (insn.raw >> 24) == 0xFC;
        let subtract = if vector {
            ((insn.raw >> 23) & 1) != 0
        } else {
            ((insn.raw >> 20) & 1) != 0
        };
        match (insn.mnemonic, subtract) {
            (Mnemonic::VFMAL, false) | (Mnemonic::VFMLS, true) => {}
            _ => return ExecResult::Undefined,
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        if q && (d & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 {
            return ExecResult::Undefined;
        }

        let n = if q {
            (n_bit << 4) | vn
        } else {
            (vn << 1) | n_bit
        };
        let m = if vector {
            if q {
                (m_bit << 4) | vm
            } else {
                (vm << 1) | m_bit
            }
        } else if q {
            vm & 0x7
        } else {
            ((vm & 0x7) << 1) | m_bit
        };
        if n >= 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let index = if !vector {
            Some(if q { (m_bit << 1) | (vm >> 3) } else { vm >> 3 })
        } else {
            None
        };
        let scalar = index.map(|lane| {
            if q {
                self.neon_read_d_elem_u64(m, lane, 2) as u16
            } else {
                ((self.cpu.vfp.read_s_bits(m) >> (lane * 16)) & 0xFFFF) as u16
            }
        });

        for reg in 0..regs {
            let operand1 = if q {
                self.cpu.vfp.read_d_bits(n)
            } else {
                u64::from(self.cpu.vfp.read_s_bits(n))
            };
            let operand2 = if scalar.is_some() {
                0
            } else if q {
                self.cpu.vfp.read_d_bits(m)
            } else {
                u64::from(self.cpu.vfp.read_s_bits(m))
            };
            let acc = self.cpu.vfp.read_d_bits(d + reg);
            let mut out = 0u64;
            for lane in 0..2 {
                let source_lane = if q { 2 * reg + lane } else { lane };
                let shift = source_lane * 16;
                let mut lhs = ((operand1 >> shift) & 0xFFFF) as u16;
                let rhs = scalar.unwrap_or_else(|| ((operand2 >> shift) & 0xFFFF) as u16);
                if subtract {
                    lhs ^= 0x8000;
                }
                let acc_lane = f32::from_bits(((acc >> (lane * 32)) & 0xFFFF_FFFF) as u32);
                let result = vcvt_f32_f16_bits(lhs).mul_add(vcvt_f32_f16_bits(rhs), acc_lane);
                out |= u64::from(result.to_bits()) << (lane * 32);
            }
            self.cpu.vfp.write_d_bits(d + reg, out);
        }

        ExecResult::Continue
    }

    fn exec_neon_polynomial_multiply(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_polynomial_multiply_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, 1);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, 1);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                out.push(u64::from(
                    Self::neon_polynomial_mul_u8(n_elem as u8, m_elem as u8) as u8,
                ));
            }
            self.neon_write_vector_elements_u64(d + reg, 1, 1, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_integer_multiply(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let scalar = Self::is_neon_integer_multiply_scalar_shape(insn.raw);
        if !Self::is_neon_integer_multiply_shape(insn.raw) && !scalar {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let (accumulate, subtract) = if scalar {
            match (insn.raw >> 8) & 0xF {
                0b0000 => (true, false),
                0b0100 => (true, true),
                0b1000 => (false, false),
                _ => return ExecResult::Undefined,
            }
        } else {
            let accumulate = ((insn.raw >> 4) & 1) == 0;
            let subtract = ((insn.raw >> 24) & 1) != 0;
            if !accumulate && subtract {
                return ExecResult::Undefined;
            }
            (accumulate, subtract)
        };

        match (insn.mnemonic, accumulate, subtract) {
            (Mnemonic::VMUL, false, false)
            | (Mnemonic::VMLA, true, false)
            | (Mnemonic::VMLS, true, true) => {}
            _ => return ExecResult::Undefined,
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = if scalar {
            ((insn.raw >> 24) & 1) != 0
        } else {
            ((insn.raw >> 6) & 1) != 0
        };
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | if scalar { 0 } else { m }) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || (!scalar && m + regs > 32) {
            return ExecResult::Undefined;
        }

        let scalar_elem = if scalar {
            let (scalar_reg, scalar_index) = match size {
                NeonSize::H16 => (vm & 0x7, (m_bit << 1) | (vm >> 3)),
                NeonSize::S32 => (vm, m_bit),
                _ => return ExecResult::Undefined,
            };
            if scalar_reg >= 32 || scalar_index as usize >= size.elements_per_d() {
                return ExecResult::Undefined;
            }
            Some(self.neon_read_d_elem_u64(scalar_reg, scalar_index, ebytes))
        } else {
            None
        };

        let mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = if let Some(elem) = scalar_elem {
                vec![elem; n_elements.len()]
            } else {
                self.neon_read_vector_elements_u64(m + reg, 1, ebytes)
            };
            let d_elements = if accumulate {
                self.neon_read_vector_elements_u64(d + reg, 1, ebytes)
            } else {
                vec![0; n_elements.len()]
            };
            let mut out = Vec::with_capacity(n_elements.len());
            for ((n_elem, m_elem), d_elem) in n_elements
                .into_iter()
                .zip(m_elements.into_iter())
                .zip(d_elements.into_iter())
            {
                let product = n_elem.wrapping_mul(m_elem) & mask;
                let result = if accumulate {
                    if subtract {
                        d_elem.wrapping_sub(product)
                    } else {
                        d_elem.wrapping_add(product)
                    }
                } else {
                    product
                } & mask;
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_long_multiply(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if Self::is_neon_polynomial_multiply_long_shape(insn.raw) {
            return self.exec_neon_polynomial_multiply_long(insn);
        }
        let scalar = Self::is_neon_long_multiply_scalar_shape(insn.raw);
        if !Self::is_neon_long_multiply_shape(insn.raw) && !scalar {
            return ExecResult::Undefined;
        }

        let narrow_size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let narrow_ebytes = (narrow_size.bits() / 8) as u8;
        let wide_ebytes = narrow_ebytes * 2;
        let wide_bits = narrow_size.bits() * 2;
        let saturating_doubling = matches!(
            insn.mnemonic,
            Mnemonic::VQDMULL | Mnemonic::VQDMLAL | Mnemonic::VQDMLSL
        );
        if scalar && narrow_size == NeonSize::B8 {
            return ExecResult::Undefined;
        }
        if saturating_doubling && narrow_size == NeonSize::B8 {
            return ExecResult::Undefined;
        }
        let unsigned = ((insn.raw >> 24) & 1) != 0 && !saturating_doubling;
        if scalar && saturating_doubling && ((insn.raw >> 24) & 1) != 0 {
            return ExecResult::Undefined;
        }
        let (accumulate, subtract) = match (insn.mnemonic, scalar, (insn.raw >> 8) & 0xF) {
            (Mnemonic::VMLAL, true, 0b0010) => (true, false),
            (Mnemonic::VQDMLAL, true, 0b0011) => (true, false),
            (Mnemonic::VMLSL, true, 0b0110) => (true, true),
            (Mnemonic::VQDMLSL, true, 0b0111) => (true, true),
            (Mnemonic::VMULL, true, 0b1010) => (false, false),
            (Mnemonic::VQDMULL, true, 0b1011) => (false, false),
            (Mnemonic::VMULL | Mnemonic::VQDMULL, false, _) => (false, false),
            (Mnemonic::VMLAL | Mnemonic::VQDMLAL, false, _) => (true, false),
            (Mnemonic::VMLSL | Mnemonic::VQDMLSL, false, _) => (true, true),
            _ => return ExecResult::Undefined,
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if (d & 1) != 0 || d + 2 > 32 || n >= 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let scalar_elem = if scalar {
            let (scalar_reg, scalar_index) = match narrow_size {
                NeonSize::H16 => (vm & 0x7, (m_bit << 1) | (vm >> 3)),
                NeonSize::S32 => (vm, m_bit),
                _ => return ExecResult::Undefined,
            };
            if scalar_reg >= 32 || scalar_index as usize >= narrow_size.elements_per_d() {
                return ExecResult::Undefined;
            }
            Some(self.neon_read_d_elem_u64(scalar_reg, scalar_index, narrow_ebytes))
        } else {
            None
        };

        let n_elements = self.neon_read_vector_elements_u64(n, 1, narrow_ebytes);
        let m_elements = if let Some(elem) = scalar_elem {
            vec![elem; n_elements.len()]
        } else {
            self.neon_read_vector_elements_u64(m, 1, narrow_ebytes)
        };
        let d_elements = if accumulate {
            self.neon_read_vector_elements_u64(d, 2, wide_ebytes)
        } else {
            vec![0; n_elements.len()]
        };
        if n_elements.len() != m_elements.len() || n_elements.len() != d_elements.len() {
            return ExecResult::Undefined;
        }

        let mut out = Vec::with_capacity(n_elements.len());
        for ((n_elem, m_elem), d_elem) in n_elements
            .into_iter()
            .zip(m_elements.into_iter())
            .zip(d_elements.into_iter())
        {
            let mut product = if unsigned {
                (n_elem as i128) * (m_elem as i128)
            } else {
                let lhs = Self::neon_sign_extend_elem_u64(n_elem, narrow_size.bits());
                let rhs = Self::neon_sign_extend_elem_u64(m_elem, narrow_size.bits());
                lhs * rhs
            };
            if saturating_doubling {
                product <<= 1;
            }
            let acc = if unsigned {
                d_elem as i128
            } else {
                Self::neon_sign_extend_elem_u64(d_elem, wide_bits)
            };
            let value = if accumulate {
                if subtract {
                    acc - product
                } else {
                    acc + product
                }
            } else {
                product
            };
            if saturating_doubling {
                let (value, saturated) = Self::neon_signed_saturate_i128(value, wide_bits);
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                out.push(Self::neon_pack_signed_elem_i128(value, wide_bits));
            } else {
                out.push(Self::neon_pack_signed_elem_i128(value, wide_bits));
            }
        }

        self.neon_write_vector_elements_u64(d, 2, wide_ebytes, &out);
        ExecResult::Continue
    }

    fn exec_neon_polynomial_multiply_long(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !Self::is_neon_polynomial_multiply_long_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if (d & 1) != 0 || d + 2 > 32 || n >= 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let n_elements = self.neon_read_vector_elements_u64(n, 1, 1);
        let m_elements = self.neon_read_vector_elements_u64(m, 1, 1);
        let mut out = Vec::with_capacity(n_elements.len());
        for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
            out.push(u64::from(Self::neon_polynomial_mul_u8(
                n_elem as u8,
                m_elem as u8,
            )));
        }
        self.neon_write_vector_elements_u64(d, 2, 2, &out);
        ExecResult::Continue
    }

    fn neon_polynomial_mul_u8(lhs: u8, rhs: u8) -> u16 {
        let mut product = 0u16;
        for bit in 0..8 {
            if ((rhs >> bit) & 1) != 0 {
                product ^= (lhs as u16) << bit;
            }
        }
        product
    }

    fn exec_neon_saturating_doubling_mulh(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let scalar = ((insn.raw >> 23) & 1) != 0;
        let size = match (insn.raw >> 20) & 0x3 {
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = if scalar {
            ((insn.raw >> 24) & 1) != 0
        } else {
            ((insn.raw >> 6) & 1) != 0
        };
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | if scalar { 0 } else { m }) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || (!scalar && m + regs > 32) {
            return ExecResult::Undefined;
        }

        let scalar_elem = if scalar {
            let (scalar_reg, scalar_index) = match size {
                NeonSize::H16 => (vm & 0x7, (m_bit << 1) | (vm >> 3)),
                NeonSize::S32 => (vm, m_bit),
                _ => return ExecResult::Undefined,
            };
            if scalar_reg >= 32 || scalar_index as usize >= size.elements_per_d() {
                return ExecResult::Undefined;
            }
            Some(self.neon_read_d_elem_u64(scalar_reg, scalar_index, ebytes))
        } else {
            None
        };

        for index in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + index, 1, ebytes);
            let m_elements = if let Some(elem) = scalar_elem {
                vec![elem; n_elements.len()]
            } else {
                self.neon_read_vector_elements_u64(m + index, 1, ebytes)
            };
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let (packed, saturated) = Self::neon_doubling_mulh_elem(
                    n_elem,
                    m_elem,
                    size.bits(),
                    insn.mnemonic == Mnemonic::VQRDMULH,
                );
                if saturated {
                    self.cpu.vfp.fpscr.set_qc(true);
                }
                out.push(packed);
            }
            self.neon_write_vector_elements_u64(d + index, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_count_register(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        if insn.mnemonic == Mnemonic::VCNT && size != NeonSize::B8 {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let bits = self.cpu.vfp.read_d_bits(m + index);
            let result = match insn.mnemonic {
                Mnemonic::VCLS => vcls_i(bits, size),
                Mnemonic::VCLZ => vclz_i(bits, size),
                Mnemonic::VCNT => vcnt_i8(bits),
                _ => return ExecResult::Undefined,
            };
            self.cpu.vfp.write_d_bits(d + index, result);
        }

        ExecResult::Continue
    }

    fn exec_neon_recip_estimate(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 23) != 0b111100111
            || ((insn.raw >> 20) & 0x3) != 0b11
            || ((insn.raw >> 16) & 0x3) != 0b11
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let size = (insn.raw >> 18) & 0x3;
        let fp = match ((insn.raw >> 7) & 0x1F, insn.mnemonic) {
            (0b01000, Mnemonic::VRECPE) | (0b01001, Mnemonic::VRSQRTE) => false,
            (0b01010, Mnemonic::VRECPE) | (0b01011, Mnemonic::VRSQRTE) => true,
            _ => return ExecResult::Undefined,
        };
        if (!fp && size != 0b10) || (fp && !matches!(size, 0b01 | 0b10)) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let ebytes = if fp && size == 0b01 { 2 } else { 4 };
        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let result = match (insn.mnemonic, fp) {
                    (Mnemonic::VRECPE, false) => {
                        u64::from(Self::neon_unsigned_recip_estimate(elem as u32))
                    }
                    (Mnemonic::VRSQRTE, false) => {
                        u64::from(Self::neon_unsigned_rsqrt_estimate(elem as u32))
                    }
                    (Mnemonic::VRECPE, true) if size == 0b01 => {
                        let input = vcvt_f32_f16_bits(elem as u16).to_bits();
                        let result = f32::from_bits(Self::neon_fp_recip_estimate_f32(input));
                        u64::from(vcvt_f16_bits_f32(result, &mut self.cpu.vfp.fpscr))
                    }
                    (Mnemonic::VRSQRTE, true) if size == 0b01 => {
                        let input = vcvt_f32_f16_bits(elem as u16).to_bits();
                        let result = f32::from_bits(Self::neon_fp_rsqrt_estimate_f32(input));
                        u64::from(vcvt_f16_bits_f32(result, &mut self.cpu.vfp.fpscr))
                    }
                    (Mnemonic::VRECPE, true) => {
                        u64::from(Self::neon_fp_recip_estimate_f32(elem as u32))
                    }
                    (Mnemonic::VRSQRTE, true) => {
                        u64::from(Self::neon_fp_rsqrt_estimate_f32(elem as u32))
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn neon_recip_estimate(a: u32) -> u32 {
        let a = a * 2 + 1;
        let b = (1u32 << 19) / a;
        (b + 1) >> 1
    }

    fn neon_recip_sqrt_estimate(mut a: u32) -> u32 {
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

    fn neon_unsigned_recip_estimate(op: u32) -> u32 {
        if op & 0x8000_0000 == 0 {
            return u32::MAX;
        }
        let estimate = Self::neon_recip_estimate((op >> 23) & 0x1FF);
        (estimate & 0x1FF) << 23
    }

    fn neon_unsigned_rsqrt_estimate(op: u32) -> u32 {
        if op & 0xC000_0000 == 0 {
            return u32::MAX;
        }
        let estimate = Self::neon_recip_sqrt_estimate((op >> 23) & 0x1FF);
        (estimate & 0x1FF) << 23
    }

    fn neon_fp_recip_estimate_f32(bits: u32) -> u32 {
        let sign = bits >> 31;
        let exp = (bits >> 23) & 0xFF;
        let frac = bits & 0x7F_FFFF;
        if exp == 0xFF {
            return if frac != 0 {
                bits | 0x40_0000
            } else {
                sign << 31
            };
        }
        if exp == 0 && frac == 0 {
            return (sign << 31) | (0xFF << 23);
        }
        if exp == 0 && frac < 0x20_0000 {
            return (sign << 31) | (0xFF << 23);
        }

        let mut fraction: u64 = (frac as u64) << 29;
        let mut e = exp as i32;
        if e == 0 {
            if (fraction >> 51) & 1 == 0 {
                e = -1;
                fraction = (fraction << 2) & ((1u64 << 52) - 1);
            } else {
                fraction = (fraction << 1) & ((1u64 << 52) - 1);
            }
        }
        let scaled = 0x100 | ((fraction >> 44) & 0xFF) as u32;
        let estimate = Self::neon_recip_estimate(scaled);
        let mut result_exp = 253i32 - e;
        let mut out_frac: u64 = ((estimate & 0xFF) as u64) << 44;
        if result_exp == 0 {
            out_frac = (1u64 << 51) | (out_frac >> 1);
        } else if result_exp == -1 {
            out_frac = (1u64 << 50) | (out_frac >> 2);
            result_exp = 0;
        }
        (sign << 31) | (((result_exp as u32) & 0xFF) << 23) | ((out_frac >> 29) as u32 & 0x7F_FFFF)
    }

    fn neon_fp_rsqrt_estimate_f32(bits: u32) -> u32 {
        let sign = bits >> 31;
        let exp = (bits >> 23) & 0xFF;
        let frac = bits & 0x7F_FFFF;
        if exp == 0xFF && frac != 0 {
            return bits | 0x40_0000;
        }
        if exp == 0 && frac == 0 {
            return (sign << 31) | (0xFF << 23);
        }
        if sign == 1 {
            return 0x7FC0_0000;
        }
        if exp == 0xFF {
            return 0;
        }

        let mut fraction: u64 = (frac as u64) << 29;
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
        let estimate = Self::neon_recip_sqrt_estimate(scaled);
        (sign << 31) | (result_exp << 23) | ((estimate & 0xFF) << 15)
    }

    fn exec_neon_vext(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let imm = ((insn.raw >> 8) & 0xF) as usize;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if (!q && imm > 7) || (q && ((d | n | m) & 1) != 0) {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let bytes = regs as usize * 8;
        let mut source = [0u8; 32];
        for index in 0..regs {
            let offset = index as usize * 8;
            source[offset..offset + 8]
                .copy_from_slice(&self.cpu.vfp.read_d_bits(n + index).to_le_bytes());
            source[bytes + offset..bytes + offset + 8]
                .copy_from_slice(&self.cpu.vfp.read_d_bits(m + index).to_le_bytes());
        }

        for index in 0..regs {
            let offset = index as usize * 8;
            let mut out = [0u8; 8];
            out.copy_from_slice(&source[imm + offset..imm + offset + 8]);
            self.cpu
                .vfp
                .write_d_bits(d + index, u64::from_le_bytes(out));
        }

        ExecResult::Continue
    }

    fn exec_neon_table_lookup(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let length = (((insn.raw >> 8) & 0x3) as u8) + 1;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if d >= 32 || m >= 32 || n + length > 32 {
            return ExecResult::Undefined;
        }

        let mut table = [0u8; 32];
        for reg in 0..length {
            let offset = reg as usize * 8;
            table[offset..offset + 8]
                .copy_from_slice(&self.cpu.vfp.read_d_bits(n + reg).to_le_bytes());
        }

        let indexes = self.cpu.vfp.read_d_bits(m).to_le_bytes();
        let mut out = self.cpu.vfp.read_d_bits(d).to_le_bytes();
        let table_len = length as usize * 8;
        for lane in 0..8 {
            let index = indexes[lane] as usize;
            if index < table_len {
                out[lane] = table[index];
            } else if insn.mnemonic == Mnemonic::VTBL {
                out[lane] = 0;
            }
        }
        self.cpu.vfp.write_d_bits(d, u64::from_le_bytes(out));

        ExecResult::Continue
    }

    fn exec_neon_minmax(&mut self, insn: &DecodedInsn) -> ExecResult {
        if (insn.raw >> 25) == 0b1111001
            && ((insn.raw >> 23) & 1) == 0
            && ((insn.raw >> 8) & 0xF) == 0b0110
        {
            return self.exec_neon_integer_minmax(insn);
        }

        self.exec_neon_fp_minmax(insn)
    }

    fn exec_neon_fp_minmax(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 24) != 0xF2
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 8) & 0xF) != 0b1111
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let size = if ((insn.raw >> 20) & 1) == 0 {
            NeonSize::S32
        } else {
            NeonSize::H16
        };
        let ebytes = (size.bits() / 8) as u8;

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let result = match size {
                    NeonSize::S32 => {
                        let n_val = f32::from_bits(n_elem as u32);
                        let m_val = f32::from_bits(m_elem as u32);
                        (match insn.mnemonic {
                            Mnemonic::VMAX => Self::neon_fpmax_f32_bits(n_val, m_val),
                            Mnemonic::VMIN => Self::neon_fpmin_f32_bits(n_val, m_val),
                            _ => return ExecResult::Undefined,
                        }) as u64
                    }
                    NeonSize::H16 => {
                        let n_val = vcvt_f32_f16_bits(n_elem as u16);
                        let m_val = vcvt_f32_f16_bits(m_elem as u16);
                        let mut fpscr = self.cpu.vfp.fpscr;
                        match insn.mnemonic {
                            Mnemonic::VMAX => vcvt_f16_bits_f32(
                                f32::from_bits(Self::neon_fpmax_f32_bits(n_val, m_val)),
                                &mut fpscr,
                            ) as u64,
                            Mnemonic::VMIN => vcvt_f16_bits_f32(
                                f32::from_bits(Self::neon_fpmin_f32_bits(n_val, m_val)),
                                &mut fpscr,
                            ) as u64,
                            _ => return ExecResult::Undefined,
                        }
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_integer_minmax(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 8) & 0xF) != 0b0110
        {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let result = if unsigned {
                    match insn.mnemonic {
                        Mnemonic::VMAX => n_elem.max(m_elem),
                        Mnemonic::VMIN => n_elem.min(m_elem),
                        _ => return ExecResult::Undefined,
                    }
                } else {
                    let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                    let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                    let value = match insn.mnemonic {
                        Mnemonic::VMAX => lhs.max(rhs),
                        Mnemonic::VMIN => lhs.min(rhs),
                        _ => return ExecResult::Undefined,
                    };
                    Self::neon_pack_signed_elem_i128(value, size.bits())
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_integer_compare(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001 || ((insn.raw >> 23) & 1) != 0 {
            return ExecResult::Undefined;
        }

        let op8 = (insn.raw >> 8) & 0xF;
        let bit4 = (insn.raw >> 4) & 1;
        let bit24 = (insn.raw >> 24) & 1;
        match (insn.mnemonic, op8, bit4, bit24) {
            (Mnemonic::VTST, 0b1000, 1, 0)
            | (Mnemonic::VCEQ, 0b1000, 1, 1)
            | (Mnemonic::VCGT, 0b0011, 0, _)
            | (Mnemonic::VCGE, 0b0011, 1, _) => {}
            _ => return ExecResult::Undefined,
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = bit24 != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let true_mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };
        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let condition = match insn.mnemonic {
                    Mnemonic::VTST => (n_elem & m_elem) != 0,
                    Mnemonic::VCEQ => n_elem == m_elem,
                    Mnemonic::VCGT if unsigned => n_elem > m_elem,
                    Mnemonic::VCGE if unsigned => n_elem >= m_elem,
                    Mnemonic::VCGT => {
                        let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                        let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                        lhs > rhs
                    }
                    Mnemonic::VCGE => {
                        let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                        let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                        lhs >= rhs
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(if condition { true_mask } else { 0 });
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_compare(&mut self, insn: &DecodedInsn) -> ExecResult {
        if (insn.raw >> 23) == 0b111100111
            && ((insn.raw >> 20) & 0x3) == 0b11
            && ((insn.raw >> 16) & 0x3) == 0b01
            && matches!((insn.raw >> 10) & 0x3, 0 | 1)
            && ((insn.raw >> 4) & 1) == 0
        {
            return self.exec_neon_compare_zero(insn);
        }

        if (insn.raw >> 25) == 0b1111001
            && ((insn.raw >> 23) & 1) == 0
            && ((insn.raw >> 8) & 0xF) == 0b1110
        {
            return self.exec_neon_fp_compare(insn);
        }

        self.exec_neon_integer_compare(insn)
    }

    fn exec_neon_compare_zero(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 23) != 0b111100111
            || ((insn.raw >> 20) & 0x3) != 0b11
            || ((insn.raw >> 16) & 0x3) != 0b01
            || !matches!((insn.raw >> 10) & 0x3, 0 | 1)
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let op = (insn.raw >> 7) & 0x7;
        let fp = ((insn.raw >> 8) & 0x7) >= 0b100;
        match (insn.mnemonic, op) {
            (Mnemonic::VCGT, 0b000)
            | (Mnemonic::VCGE, 0b001)
            | (Mnemonic::VCEQ, 0b010)
            | (Mnemonic::VCLE, 0b011)
            | (Mnemonic::VCLT, 0b100) => {}
            _ => return ExecResult::Undefined,
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        if fp && !matches!(size, NeonSize::H16 | NeonSize::S32) {
            return ExecResult::Undefined;
        }
        let ebytes = (size.bits() / 8) as u8;
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let true_mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };
        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let condition = if fp {
                    let value = match size {
                        NeonSize::H16 => vcvt_f32_f16_bits(elem as u16),
                        NeonSize::S32 => f32::from_bits(elem as u32),
                        _ => return ExecResult::Undefined,
                    };
                    match insn.mnemonic {
                        Mnemonic::VCGT => value > 0.0,
                        Mnemonic::VCGE => value >= 0.0,
                        Mnemonic::VCEQ => value == 0.0,
                        Mnemonic::VCLE => value <= 0.0,
                        Mnemonic::VCLT => value < 0.0,
                        _ => return ExecResult::Undefined,
                    }
                } else {
                    let value = Self::neon_sign_extend_elem_u64(elem, size.bits());
                    match insn.mnemonic {
                        Mnemonic::VCGT => value > 0,
                        Mnemonic::VCGE => value >= 0,
                        Mnemonic::VCEQ => value == 0,
                        Mnemonic::VCLE => value <= 0,
                        Mnemonic::VCLT => value < 0,
                        _ => return ExecResult::Undefined,
                    }
                };
                out.push(if condition { true_mask } else { 0 });
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_fp_compare(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 8) & 0xF) != 0b1110
        {
            return ExecResult::Undefined;
        }

        let bit24 = (insn.raw >> 24) & 1;
        let bit21 = (insn.raw >> 21) & 1;
        let bit20 = (insn.raw >> 20) & 1;
        let absolute = ((insn.raw >> 4) & 1) != 0;
        match (insn.mnemonic, absolute, bit24, bit21, bit20) {
            (Mnemonic::VCEQ, false, 0, 0, 0 | 1)
            | (Mnemonic::VCGE, false, 1, 0, 0 | 1)
            | (Mnemonic::VCGT, false, 1, 1, 0 | 1)
            | (Mnemonic::VACGE, true, 1, 0, 0 | 1)
            | (Mnemonic::VACGT, true, 1, 1, 0 | 1) => {}
            _ => return ExecResult::Undefined,
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let size = if bit20 == 0 {
            NeonSize::S32
        } else {
            NeonSize::H16
        };
        let ebytes = (size.bits() / 8) as u8;
        let true_mask = if size == NeonSize::S32 {
            u64::from(u32::MAX)
        } else {
            u64::from(u16::MAX)
        };

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let mut lhs = match size {
                    NeonSize::S32 => f32::from_bits(n_elem as u32),
                    NeonSize::H16 => vcvt_f32_f16_bits(n_elem as u16),
                    _ => return ExecResult::Undefined,
                };
                let mut rhs = match size {
                    NeonSize::S32 => f32::from_bits(m_elem as u32),
                    NeonSize::H16 => vcvt_f32_f16_bits(m_elem as u16),
                    _ => return ExecResult::Undefined,
                };
                if absolute {
                    lhs = lhs.abs();
                    rhs = rhs.abs();
                }
                let condition = match insn.mnemonic {
                    Mnemonic::VCEQ => lhs == rhs,
                    Mnemonic::VCGT | Mnemonic::VACGT => lhs > rhs,
                    Mnemonic::VCGE | Mnemonic::VACGE => lhs >= rhs,
                    _ => return ExecResult::Undefined,
                };
                out.push(if condition { true_mask } else { 0 });
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_recip_step(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 24) & 1) != 0
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 8) & 0xF) != 0b1111
            || ((insn.raw >> 4) & 1) != 1
        {
            return ExecResult::Undefined;
        }

        match (insn.mnemonic, (insn.raw >> 21) & 1) {
            (Mnemonic::VRECPS, 0) | (Mnemonic::VRSQRTS, 1) => {}
            _ => return ExecResult::Undefined,
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let size = if ((insn.raw >> 20) & 1) == 0 {
            NeonSize::S32
        } else {
            NeonSize::H16
        };
        let ebytes = (size.bits() / 8) as u8;

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let lhs = match size {
                    NeonSize::S32 => f32::from_bits(n_elem as u32),
                    NeonSize::H16 => vcvt_f32_f16_bits(n_elem as u16),
                    _ => return ExecResult::Undefined,
                };
                let rhs = match size {
                    NeonSize::S32 => f32::from_bits(m_elem as u32),
                    NeonSize::H16 => vcvt_f32_f16_bits(m_elem as u16),
                    _ => return ExecResult::Undefined,
                };
                let result = if lhs.is_nan() || rhs.is_nan() {
                    f32::NAN
                } else if insn.mnemonic == Mnemonic::VRECPS {
                    if (lhs.is_infinite() && rhs == 0.0) || (rhs.is_infinite() && lhs == 0.0) {
                        2.0
                    } else {
                        (-lhs).mul_add(rhs, 2.0)
                    }
                } else if (lhs.is_infinite() && rhs == 0.0) || (rhs.is_infinite() && lhs == 0.0) {
                    1.5
                } else {
                    (-lhs).mul_add(rhs, 3.0) * 0.5
                };
                let result = match size {
                    NeonSize::S32 => u64::from(result.to_bits()),
                    NeonSize::H16 => u64::from(vcvt_f16_bits_f32(result, &mut self.cpu.vfp.fpscr)),
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn neon_fpmax_f32_bits(a: f32, b: f32) -> u32 {
        if a.is_nan() || b.is_nan() {
            return f32::NAN.to_bits();
        }
        if a == b {
            if a.is_sign_positive() || b.is_sign_positive() {
                0.0f32.to_bits()
            } else {
                a.to_bits()
            }
        } else {
            a.max(b).to_bits()
        }
    }

    fn neon_fpmin_f32_bits(a: f32, b: f32) -> u32 {
        if a.is_nan() || b.is_nan() {
            return f32::NAN.to_bits();
        }
        if a == b {
            if a.is_sign_negative() || b.is_sign_negative() {
                (-0.0f32).to_bits()
            } else {
                a.to_bits()
            }
        } else {
            a.min(b).to_bits()
        }
    }

    fn exec_neon_absdiff(&mut self, insn: &DecodedInsn) -> ExecResult {
        if (insn.raw >> 25) == 0b1111001
            && ((insn.raw >> 23) & 1) == 0
            && ((insn.raw >> 8) & 0xF) == 0b0111
        {
            return self.exec_neon_integer_absdiff_accum(insn);
        }

        self.exec_neon_fp_absdiff(insn)
    }

    fn exec_neon_fp_absdiff(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 24) != 0xF3
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 21) & 1) != 1
            || ((insn.raw >> 8) & 0xF) != 0b1101
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let size = if ((insn.raw >> 20) & 1) == 0 {
            NeonSize::S32
        } else {
            NeonSize::H16
        };
        let ebytes = (size.bits() / 8) as u8;

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(n_elements.len());
            for (n_elem, m_elem) in n_elements.into_iter().zip(m_elements.into_iter()) {
                let result = match size {
                    NeonSize::S32 => {
                        let n_val = f32::from_bits(n_elem as u32);
                        let m_val = f32::from_bits(m_elem as u32);
                        (n_val - m_val).abs().to_bits() as u64
                    }
                    NeonSize::H16 => {
                        let n_val = vcvt_f32_f16_bits(n_elem as u16);
                        let m_val = vcvt_f32_f16_bits(m_elem as u16);
                        let mut fpscr = self.cpu.vfp.fpscr;
                        vcvt_f16_bits_f32((n_val - m_val).abs(), &mut fpscr) as u64
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_integer_absdiff_accum(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 0
            || ((insn.raw >> 8) & 0xF) != 0b0111
        {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let ebytes = (size.bits() / 8) as u8;
        let unsigned = ((insn.raw >> 24) & 1) != 0;
        let accumulate = match insn.mnemonic {
            Mnemonic::VABD => false,
            Mnemonic::VABA => true,
            _ => return ExecResult::Undefined,
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if q && ((d | n | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || n + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let mask = if size.bits() == 32 {
            u64::from(u32::MAX)
        } else {
            (1u64 << size.bits()) - 1
        };

        for reg in 0..regs {
            let n_elements = self.neon_read_vector_elements_u64(n + reg, 1, ebytes);
            let m_elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let d_elements = if accumulate {
                self.neon_read_vector_elements_u64(d + reg, 1, ebytes)
            } else {
                vec![0; n_elements.len()]
            };
            let mut out = Vec::with_capacity(n_elements.len());
            for ((n_elem, m_elem), d_elem) in n_elements
                .into_iter()
                .zip(m_elements.into_iter())
                .zip(d_elements.into_iter())
            {
                let diff = if unsigned {
                    n_elem.abs_diff(m_elem)
                } else {
                    let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                    let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                    lhs.abs_diff(rhs) as u64
                };
                let result = if accumulate {
                    d_elem.wrapping_add(diff) & mask
                } else {
                    diff & mask
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_integer_absdiff_long(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if (insn.raw >> 25) != 0b1111001
            || ((insn.raw >> 23) & 1) != 1
            || ((insn.raw >> 4) & 1) != 0
        {
            return ExecResult::Undefined;
        }

        let accumulate = match ((insn.raw >> 8) & 0xF, insn.mnemonic) {
            (0b0111, Mnemonic::VABDL) => false,
            (0b0101, Mnemonic::VABAL) => true,
            _ => return ExecResult::Undefined,
        };

        let size = match (insn.raw >> 20) & 0x3 {
            0b00 => NeonSize::B8,
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let src_ebytes = (size.bits() / 8) as u8;
        let dest_ebytes = src_ebytes * 2;
        let dest_bits = size.bits() * 2;
        let unsigned = ((insn.raw >> 24) & 1) != 0;

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;

        let d = (d_bit << 4) | vd;
        let n = (n_bit << 4) | vn;
        let m = (m_bit << 4) | vm;
        if (d & 1) != 0 || ((insn.raw >> 6) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + 2 > 32 || n >= 32 || m >= 32 {
            return ExecResult::Undefined;
        }

        let n_elements = self.neon_read_vector_elements_u64(n, 1, src_ebytes);
        let m_elements = self.neon_read_vector_elements_u64(m, 1, src_ebytes);
        let d_elements = if accumulate {
            self.neon_read_vector_elements_u64(d, 2, dest_ebytes)
        } else {
            vec![0; n_elements.len()]
        };
        let mask = if dest_bits == 64 {
            u64::MAX
        } else {
            (1u64 << dest_bits) - 1
        };

        let mut out = Vec::with_capacity(n_elements.len());
        for ((n_elem, m_elem), d_elem) in n_elements
            .into_iter()
            .zip(m_elements.into_iter())
            .zip(d_elements.into_iter())
        {
            let diff = if unsigned {
                n_elem.abs_diff(m_elem)
            } else {
                let lhs = Self::neon_sign_extend_elem_u64(n_elem, size.bits());
                let rhs = Self::neon_sign_extend_elem_u64(m_elem, size.bits());
                lhs.abs_diff(rhs) as u64
            };
            let result = if accumulate {
                d_elem.wrapping_add(diff)
            } else {
                diff
            };
            out.push(result & mask);
        }
        self.neon_write_vector_elements_u64(d, 2, dest_ebytes, &out);

        ExecResult::Continue
    }

    fn exec_vld1_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_all_lanes(insn) {
            return self.exec_vld_all_lanes(info);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vld_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let NeonStructMem {
            addr,
            regs,
            first,
            writeback,
            rn,
            rm,
            ..
        } = info;

        let mut current = addr;
        for index in 0..regs {
            let mut bits = 0u64;
            for byte in 0..8 {
                let value = match self.mem.read_byte(current) {
                    Ok(v) => v,
                    Err(e) => return ExecResult::MemoryFault(e),
                };
                bits |= (value as u64) << (byte * 8);
                current = current.wrapping_add(1);
            }
            self.cpu.vfp.write_d_bits(first + index, bits);
        }

        if writeback {
            self.cpu.regs[rn] = self.neon_struct_writeback(addr, regs, 1, rm);
        }
        ExecResult::Continue
    }

    fn exec_vst1_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vst_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let NeonStructMem {
            addr,
            regs,
            first,
            writeback,
            rn,
            rm,
            ..
        } = info;

        let mut current = addr;
        for index in 0..regs {
            let bits = self.cpu.vfp.read_d_bits(first + index);
            for byte in 0..8 {
                if let Err(e) = self.mem.write_byte(current, (bits >> (byte * 8)) as u8) {
                    return ExecResult::MemoryFault(e);
                }
                current = current.wrapping_add(1);
            }
        }

        if writeback {
            self.cpu.regs[rn] = self.neon_struct_writeback(addr, regs, 1, rm);
        }
        ExecResult::Continue
    }

    fn exec_vld2_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_all_lanes(insn) {
            return self.exec_vld_all_lanes(info);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vld_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let second = info.first + info.inc;
        let elements = 8 / info.ebytes;
        let mut current = info.addr;

        for r in 0..info.regs {
            for element in 0..elements {
                let first = match self.neon_read_mem_elem(current, info.ebytes) {
                    Ok(v) => v,
                    Err(e) => return ExecResult::MemoryFault(e),
                };
                let second_value = match self
                    .neon_read_mem_elem(current.wrapping_add(info.ebytes as u32), info.ebytes)
                {
                    Ok(v) => v,
                    Err(e) => return ExecResult::MemoryFault(e),
                };
                self.neon_write_d_elem(info.first + r, element, info.ebytes, first);
                self.neon_write_d_elem(second + r, element, info.ebytes, second_value);
                current = current.wrapping_add((info.ebytes * 2) as u32);
            }
        }

        if info.writeback {
            self.cpu.regs[info.rn] = self.neon_struct_writeback(info.addr, info.regs, 2, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vst2_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vst_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let second = info.first + info.inc;
        let elements = 8 / info.ebytes;
        let mut current = info.addr;

        for r in 0..info.regs {
            for element in 0..elements {
                let first = self.neon_read_d_elem(info.first + r, element, info.ebytes);
                let second_value = self.neon_read_d_elem(second + r, element, info.ebytes);
                if let Err(e) = self.neon_write_mem_elem(current, info.ebytes, first) {
                    return ExecResult::MemoryFault(e);
                }
                if let Err(e) = self.neon_write_mem_elem(
                    current.wrapping_add(info.ebytes as u32),
                    info.ebytes,
                    second_value,
                ) {
                    return ExecResult::MemoryFault(e);
                }
                current = current.wrapping_add((info.ebytes * 2) as u32);
            }
        }

        if info.writeback {
            self.cpu.regs[info.rn] = self.neon_struct_writeback(info.addr, info.regs, 2, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vld3_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_all_lanes(insn) {
            return self.exec_vld_all_lanes(info);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vld_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let second = info.first + info.inc;
        let third = second + info.inc;
        let elements = 8 / info.ebytes;
        let mut current = info.addr;

        for element in 0..elements {
            let first = match self.neon_read_mem_elem(current, info.ebytes) {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            let second_value = match self
                .neon_read_mem_elem(current.wrapping_add(info.ebytes as u32), info.ebytes)
            {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            let third_value = match self
                .neon_read_mem_elem(current.wrapping_add((info.ebytes * 2) as u32), info.ebytes)
            {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            self.neon_write_d_elem(info.first, element, info.ebytes, first);
            self.neon_write_d_elem(second, element, info.ebytes, second_value);
            self.neon_write_d_elem(third, element, info.ebytes, third_value);
            current = current.wrapping_add((info.ebytes * 3) as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] = self.neon_struct_writeback(info.addr, info.regs, 3, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vst3_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vst_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let second = info.first + info.inc;
        let third = second + info.inc;
        let elements = 8 / info.ebytes;
        let mut current = info.addr;

        for element in 0..elements {
            let first = self.neon_read_d_elem(info.first, element, info.ebytes);
            let second_value = self.neon_read_d_elem(second, element, info.ebytes);
            let third_value = self.neon_read_d_elem(third, element, info.ebytes);
            if let Err(e) = self.neon_write_mem_elem(current, info.ebytes, first) {
                return ExecResult::MemoryFault(e);
            }
            if let Err(e) = self.neon_write_mem_elem(
                current.wrapping_add(info.ebytes as u32),
                info.ebytes,
                second_value,
            ) {
                return ExecResult::MemoryFault(e);
            }
            if let Err(e) = self.neon_write_mem_elem(
                current.wrapping_add((info.ebytes * 2) as u32),
                info.ebytes,
                third_value,
            ) {
                return ExecResult::MemoryFault(e);
            }
            current = current.wrapping_add((info.ebytes * 3) as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] = self.neon_struct_writeback(info.addr, info.regs, 3, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vld4_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_all_lanes(insn) {
            return self.exec_vld_all_lanes(info);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vld_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let second = info.first + info.inc;
        let third = second + info.inc;
        let fourth = third + info.inc;
        let elements = 8 / info.ebytes;
        let mut current = info.addr;

        for element in 0..elements {
            let first = match self.neon_read_mem_elem(current, info.ebytes) {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            let second_value = match self
                .neon_read_mem_elem(current.wrapping_add(info.ebytes as u32), info.ebytes)
            {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            let third_value = match self
                .neon_read_mem_elem(current.wrapping_add((info.ebytes * 2) as u32), info.ebytes)
            {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            let fourth_value = match self
                .neon_read_mem_elem(current.wrapping_add((info.ebytes * 3) as u32), info.ebytes)
            {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            self.neon_write_d_elem(info.first, element, info.ebytes, first);
            self.neon_write_d_elem(second, element, info.ebytes, second_value);
            self.neon_write_d_elem(third, element, info.ebytes, third_value);
            self.neon_write_d_elem(fourth, element, info.ebytes, fourth_value);
            current = current.wrapping_add((info.ebytes * 4) as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] = self.neon_struct_writeback(info.addr, info.regs, 4, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vld_single_lane(&mut self, info: NeonSingleLaneMem) -> ExecResult {
        let mut current = info.addr;
        for stream in 0..info.streams {
            let value = match self.neon_read_mem_elem(current, info.ebytes) {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            self.neon_write_d_elem(
                info.first + stream * info.inc,
                info.index,
                info.ebytes,
                value,
            );
            current = current.wrapping_add(info.ebytes as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] =
                self.neon_lane_writeback(info.addr, info.streams, info.ebytes, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vld_all_lanes(&mut self, info: NeonAllLanesMem) -> ExecResult {
        let mut current = info.addr;
        for stream in 0..info.streams {
            let value = match self.neon_read_mem_elem(current, info.ebytes) {
                Ok(v) => v,
                Err(e) => return ExecResult::MemoryFault(e),
            };
            let bits = Self::neon_replicate_elem(value, info.ebytes);
            let first = info.first + stream * info.inc;
            for reg in 0..info.regs {
                self.cpu.vfp.write_d_bits(first + reg, bits);
            }
            current = current.wrapping_add(info.ebytes as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] = if info.rm == 13 {
                info.addr
                    .wrapping_add((info.streams as u32) * (info.ebytes as u32))
            } else {
                info.addr.wrapping_add(self.reg(info.rm))
            };
        }
        ExecResult::Continue
    }

    fn exec_vst4_multiple(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if let Some(info) = self.decode_neon_vld_vst_single_lane(insn) {
            return self.exec_vst_single_lane(info);
        }
        let Some(info) = self.decode_neon_vld_vst_multiple(insn) else {
            return ExecResult::Undefined;
        };
        let second = info.first + info.inc;
        let third = second + info.inc;
        let fourth = third + info.inc;
        let elements = 8 / info.ebytes;
        let mut current = info.addr;

        for element in 0..elements {
            let first = self.neon_read_d_elem(info.first, element, info.ebytes);
            let second_value = self.neon_read_d_elem(second, element, info.ebytes);
            let third_value = self.neon_read_d_elem(third, element, info.ebytes);
            let fourth_value = self.neon_read_d_elem(fourth, element, info.ebytes);
            if let Err(e) = self.neon_write_mem_elem(current, info.ebytes, first) {
                return ExecResult::MemoryFault(e);
            }
            if let Err(e) = self.neon_write_mem_elem(
                current.wrapping_add(info.ebytes as u32),
                info.ebytes,
                second_value,
            ) {
                return ExecResult::MemoryFault(e);
            }
            if let Err(e) = self.neon_write_mem_elem(
                current.wrapping_add((info.ebytes * 2) as u32),
                info.ebytes,
                third_value,
            ) {
                return ExecResult::MemoryFault(e);
            }
            if let Err(e) = self.neon_write_mem_elem(
                current.wrapping_add((info.ebytes * 3) as u32),
                info.ebytes,
                fourth_value,
            ) {
                return ExecResult::MemoryFault(e);
            }
            current = current.wrapping_add((info.ebytes * 4) as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] = self.neon_struct_writeback(info.addr, info.regs, 4, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vst_single_lane(&mut self, info: NeonSingleLaneMem) -> ExecResult {
        let mut current = info.addr;
        for stream in 0..info.streams {
            let value =
                self.neon_read_d_elem(info.first + stream * info.inc, info.index, info.ebytes);
            if let Err(e) = self.neon_write_mem_elem(current, info.ebytes, value) {
                return ExecResult::MemoryFault(e);
            }
            current = current.wrapping_add(info.ebytes as u32);
        }

        if info.writeback {
            self.cpu.regs[info.rn] =
                self.neon_lane_writeback(info.addr, info.streams, info.ebytes, info.rm);
        }
        ExecResult::Continue
    }

    fn exec_vmov(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if Self::is_neon_modified_immediate_shape(insn.raw) {
            return self.exec_neon_modified_immediate(insn);
        }
        if ((insn.raw >> 4) & 1) == 1
            && matches!((insn.raw >> 8) & 0xF, 0b1010 | 0b1011)
            && ((insn.raw >> 21) & 0x7) == 0b010
            && (insn.raw & 0xC0) == 0
        {
            let rt = ((insn.raw >> 12) & 0xF) as usize;
            let rt2 = ((insn.raw >> 16) & 0xF) as usize;
            if rt == 15 || rt2 == 15 {
                return ExecResult::Undefined;
            }
            let coproc = (insn.raw >> 8) & 0xF;
            let to_core = ((insn.raw >> 20) & 1) == 1;
            if coproc == 0b1010 {
                let sreg = ((insn.raw & 0xF) as u8) << 1;
                if to_core {
                    self.cpu.regs[rt] = self.cpu.vfp.read_s_bits(sreg);
                    self.cpu.regs[rt2] = self.cpu.vfp.read_s_bits(sreg + 1);
                } else {
                    let lo = self.reg(rt);
                    let hi = self.reg(rt2);
                    self.cpu.vfp.write_s_bits(sreg, lo);
                    self.cpu.vfp.write_s_bits(sreg + 1, hi);
                }
            } else {
                let dreg = (((insn.raw >> 5) & 1) << 4) as u8 | (insn.raw & 0xF) as u8;
                if to_core {
                    let bits = self.cpu.vfp.read_d_bits(dreg);
                    self.cpu.regs[rt] = bits as u32;
                    self.cpu.regs[rt2] = (bits >> 32) as u32;
                } else {
                    let bits = (self.reg(rt) as u64) | ((self.reg(rt2) as u64) << 32);
                    self.cpu.vfp.write_d_bits(dreg, bits);
                }
            }
            return ExecResult::Continue;
        }
        if ((insn.raw >> 4) & 1) == 1 && matches!((insn.raw >> 8) & 0xF, 0b1010 | 0b1011) {
            let rt = ((insn.raw >> 12) & 0xF) as usize;
            if rt == 15 {
                return ExecResult::Undefined;
            }
            let to_core = ((insn.raw >> 20) & 1) == 1;
            let coproc = (insn.raw >> 8) & 0xF;
            let opc1 = ((insn.raw >> 21) & 0x3) as u8;
            let opc2 = ((insn.raw >> 5) & 0x3) as u8;
            let v = ((insn.raw >> 16) & 0xF) as u8;
            if coproc == 0b1011 {
                let u = ((insn.raw >> 23) & 1) != 0;
                let dreg = ((((insn.raw >> 7) & 1) << 4) as u8) | v;
                let shape = if (opc1 & 0b10) != 0 {
                    Some((1, ((opc1 & 1) << 2) | opc2))
                } else if (opc2 & 1) != 0 {
                    Some((2, ((opc1 & 1) << 1) | (opc2 >> 1)))
                } else if !u && opc2 == 0 {
                    Some((4, opc1 & 1))
                } else {
                    None
                };
                let Some((ebytes, lane)) = shape else {
                    return ExecResult::Undefined;
                };

                if to_core {
                    let elem = self.neon_read_d_elem_u64(dreg, lane, ebytes);
                    self.cpu.regs[rt] = if u {
                        elem as u32
                    } else if ebytes < 4 {
                        Self::neon_sign_extend_elem_u64(elem, ebytes as u32 * 8) as u32
                    } else {
                        elem as u32
                    };
                } else {
                    if u {
                        return ExecResult::Undefined;
                    }
                    self.neon_write_d_elem_u64(dreg, lane, ebytes, self.reg(rt) as u64);
                }
            } else if coproc == 0b1010 {
                if opc2 != 0 || (opc1 & 0b10) != 0 {
                    return ExecResult::Undefined;
                }
                if opc1 != 0 || (insn.raw & 0xF) != 0 {
                    return ExecResult::Undefined;
                }
                let sreg = (v << 1) | (((insn.raw >> 7) & 1) as u8);
                if to_core {
                    self.cpu.regs[rt] = self.cpu.vfp.read_s_bits(sreg);
                } else {
                    let value = self.reg(rt);
                    self.cpu.vfp.write_s_bits(sreg, value);
                }
            } else {
                return ExecResult::Undefined;
            }
            return ExecResult::Continue;
        }

        if ((insn.raw >> 4) & 1) == 0
            && ((insn.raw >> 23) & 1) == 1
            && ((insn.raw >> 21) & 1) == 1
            && ((insn.raw >> 20) & 1) == 1
            && ((insn.raw >> 7) & 1) == 0
            && ((insn.raw >> 6) & 1) == 0
        {
            let size = (insn.raw >> 8) & 0x3;
            let vd = ((insn.raw >> 12) & 0xF) as u8;
            let d_bit = ((insn.raw >> 22) & 1) as u8;
            let imm8 = ((((insn.raw >> 16) & 0xF) << 4) | (insn.raw & 0xF)) as u8;
            return match size {
                1 => {
                    self.cpu
                        .vfp
                        .write_h_bits((vd << 1) | d_bit, vfp_expand_imm_f16(imm8));
                    ExecResult::Continue
                }
                2 => {
                    self.cpu
                        .vfp
                        .write_s_bits((vd << 1) | d_bit, vfp_expand_imm_f32(imm8));
                    ExecResult::Continue
                }
                3 => {
                    self.cpu
                        .vfp
                        .write_d_bits((d_bit << 4) | vd, vfp_expand_imm_f64(imm8));
                    ExecResult::Continue
                }
                _ => ExecResult::Undefined,
            };
        }

        let Some((d, m, size)) = self.decode_vfp_unary_regs(insn) else {
            return ExecResult::Undefined;
        };
        match size {
            16 => {
                let bits = self.cpu.vfp.read_h_bits(m);
                self.cpu.vfp.write_h_bits(d, bits);
            }
            32 => {
                let bits = self.cpu.vfp.read_s_bits(m);
                self.cpu.vfp.write_s_bits(d, bits);
            }
            64 => {
                let bits = self.cpu.vfp.read_d_bits(m);
                self.cpu.vfp.write_d_bits(d, bits);
            }
            _ => return ExecResult::Undefined,
        }
        ExecResult::Continue
    }

    fn is_neon_modified_immediate_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 23) & 1) == 1
            && ((raw >> 7) & 1) == 0
            && ((raw >> 4) & 1) == 1
            && (((raw >> 8) & 0xF) != 0b1111 || ((raw >> 5) & 1) == 0)
    }

    fn neon_expand_modified_immediate(raw: u32) -> Option<u64> {
        let cmode = (raw >> 8) & 0xF;
        let imm8 = ((((raw >> 24) & 1) << 7) | (((raw >> 16) & 0x7) << 4) | (raw & 0xF)) as u64;

        let imm32 = match cmode {
            0b0000 | 0b0001 => imm8 as u32,
            0b0010 | 0b0011 => (imm8 << 8) as u32,
            0b0100 | 0b0101 => (imm8 << 16) as u32,
            0b0110 | 0b0111 => (imm8 << 24) as u32,
            0b1000 | 0b1001 => {
                let imm16 = imm8 as u32;
                imm16 | (imm16 << 16)
            }
            0b1010 | 0b1011 => {
                let imm16 = (imm8 << 8) as u32;
                imm16 | (imm16 << 16)
            }
            0b1100 => ((imm8 << 8) | 0xFF) as u32,
            0b1101 => ((imm8 << 16) | 0xFFFF) as u32,
            0b1110 if ((raw >> 5) & 1) == 0 => {
                let byte = imm8 as u32;
                byte | (byte << 8) | (byte << 16) | (byte << 24)
            }
            0b1110 => {
                let mut imm64 = 0u64;
                for byte in 0..8 {
                    if ((imm8 >> byte) & 1) != 0 {
                        imm64 |= 0xFFu64 << (byte * 8);
                    }
                }
                return Some(imm64);
            }
            0b1111 if ((raw >> 5) & 1) == 0 => vfp_expand_imm_f32(imm8 as u8),
            _ => return None,
        };

        Some(u64::from(imm32) | (u64::from(imm32) << 32))
    }

    fn exec_neon_modified_immediate(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }

        let Some(imm) = Self::neon_expand_modified_immediate(insn.raw) else {
            return ExecResult::Undefined;
        };
        let d = (((insn.raw >> 22) & 1) << 4 | ((insn.raw >> 12) & 0xF)) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };
        if q && (d & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 {
            return ExecResult::Undefined;
        }

        for index in 0..regs {
            let old = self.cpu.vfp.read_d_bits(d + index);
            let result = match insn.mnemonic {
                Mnemonic::VMOV => imm,
                Mnemonic::VMVN => !imm,
                Mnemonic::VORR => old | imm,
                Mnemonic::VBIC => old & !imm,
                _ => return ExecResult::Undefined,
            };
            self.cpu.vfp.write_d_bits(d + index, result);
        }

        ExecResult::Continue
    }

    fn exec_vfp_binop(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, n, m, size)) = self.decode_vfp_ternary_regs(insn) else {
            return ExecResult::Undefined;
        };
        match size {
            16 => {
                let n_val = self.cpu.vfp.read_h_bits(n);
                let m_val = self.cpu.vfp.read_h_bits(m);
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match insn.mnemonic {
                    Mnemonic::VADD => vadd_f16_bits(n_val, m_val, fpscr),
                    Mnemonic::VSUB => vsub_f16_bits(n_val, m_val, fpscr),
                    Mnemonic::VMUL => vmul_f16_bits(n_val, m_val, fpscr),
                    Mnemonic::VDIV => vdiv_f16_bits(n_val, m_val, fpscr),
                    Mnemonic::VNMUL => vnmul_f16_bits(n_val, m_val, fpscr),
                    Mnemonic::VMAXNM_F16 => vmaxnm_f16_bits(n_val, m_val, fpscr),
                    Mnemonic::VMINNM_F16 => vminnm_f16_bits(n_val, m_val, fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_h_bits(d, result);
            }
            32 => {
                let n_val = self.cpu.vfp.read_s(n);
                let m_val = self.cpu.vfp.read_s(m);
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match insn.mnemonic {
                    Mnemonic::VADD => vadd_f32(n_val, m_val, fpscr),
                    Mnemonic::VSUB => vsub_f32(n_val, m_val, fpscr),
                    Mnemonic::VMUL => vmul_f32(n_val, m_val, fpscr),
                    Mnemonic::VDIV => vdiv_f32(n_val, m_val, fpscr),
                    Mnemonic::VNMUL => vnmul_f32(n_val, m_val, fpscr),
                    Mnemonic::VMAXNM_F32 => vmaxnm_f32(n_val, m_val, fpscr),
                    Mnemonic::VMINNM_F32 => vminnm_f32(n_val, m_val, fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_s(d, result);
            }
            64 => {
                let n_val = self.cpu.vfp.read_d(n);
                let m_val = self.cpu.vfp.read_d(m);
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match insn.mnemonic {
                    Mnemonic::VADD => vadd_f64(n_val, m_val, fpscr),
                    Mnemonic::VSUB => vsub_f64(n_val, m_val, fpscr),
                    Mnemonic::VMUL => vmul_f64(n_val, m_val, fpscr),
                    Mnemonic::VDIV => vdiv_f64(n_val, m_val, fpscr),
                    Mnemonic::VNMUL => vnmul_f64(n_val, m_val, fpscr),
                    Mnemonic::VMAXNM_F64 => vmaxnm_f64(n_val, m_val, fpscr),
                    Mnemonic::VMINNM_F64 => vminnm_f64(n_val, m_val, fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_d(d, result);
            }
            _ => return ExecResult::Undefined,
        }
        ExecResult::Continue
    }

    fn exec_vsel(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, n, m, size)) = self.decode_vfp_cond_select_regs(insn) else {
            return ExecResult::Undefined;
        };
        let fpscr = &self.cpu.vfp.fpscr;
        let take_n = match insn.mnemonic {
            Mnemonic::VSELEQ => fpscr.z(),
            Mnemonic::VSELVS => fpscr.v(),
            Mnemonic::VSELGE => fpscr.n() == fpscr.v(),
            Mnemonic::VSELGT => !fpscr.z() && fpscr.n() == fpscr.v(),
            _ => return ExecResult::Undefined,
        };

        match size {
            16 => {
                let value = if take_n {
                    self.cpu.vfp.read_h_bits(n)
                } else {
                    self.cpu.vfp.read_h_bits(m)
                };
                self.cpu.vfp.write_h_bits(d, value);
            }
            32 => {
                let value = if take_n {
                    self.cpu.vfp.read_s_bits(n)
                } else {
                    self.cpu.vfp.read_s_bits(m)
                };
                self.cpu.vfp.write_s_bits(d, value);
            }
            64 => {
                let value = if take_n {
                    self.cpu.vfp.read_d_bits(n)
                } else {
                    self.cpu.vfp.read_d_bits(m)
                };
                self.cpu.vfp.write_d_bits(d, value);
            }
            _ => return ExecResult::Undefined,
        }

        ExecResult::Continue
    }

    fn exec_vfp_accop(&mut self, insn: &DecodedInsn) -> ExecResult {
        if Self::is_neon_fp_fma_shape(insn.raw) {
            return self.exec_neon_fp_multiply(insn);
        }

        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, n, m, size)) = self.decode_vfp_ternary_regs(insn) else {
            return ExecResult::Undefined;
        };
        match size {
            16 => {
                let acc = self.cpu.vfp.read_h_bits(d);
                let n_val = self.cpu.vfp.read_h_bits(n);
                let m_val = self.cpu.vfp.read_h_bits(m);
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match insn.mnemonic {
                    Mnemonic::VMLA => vmla_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VMLS => vmls_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VFMA => vfma_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VFMS => vfms_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VNMLA => vnmla_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VNMLS => vnmls_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VFNMA => vfnma_f16_bits(acc, n_val, m_val, fpscr),
                    Mnemonic::VFNMS => vfnms_f16_bits(acc, n_val, m_val, fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_h_bits(d, result);
            }
            32 => {
                let acc = self.cpu.vfp.read_s(d);
                let n_val = self.cpu.vfp.read_s(n);
                let m_val = self.cpu.vfp.read_s(m);
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match insn.mnemonic {
                    Mnemonic::VMLA => vmla_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VMLS => vmls_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VFMA => vfma_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VFMS => vfms_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VNMLA => vnmla_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VNMLS => vnmls_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VFNMA => vfnma_f32(acc, n_val, m_val, fpscr),
                    Mnemonic::VFNMS => vfnms_f32(acc, n_val, m_val, fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_s(d, result);
            }
            64 => {
                let acc = self.cpu.vfp.read_d(d);
                let n_val = self.cpu.vfp.read_d(n);
                let m_val = self.cpu.vfp.read_d(m);
                let fpscr = &mut self.cpu.vfp.fpscr;
                let result = match insn.mnemonic {
                    Mnemonic::VMLA => vmla_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VMLS => vmls_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VFMA => vfma_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VFMS => vfms_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VNMLA => vnmla_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VNMLS => vnmls_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VFNMA => vfnma_f64(acc, n_val, m_val, fpscr),
                    Mnemonic::VFNMS => vfnms_f64(acc, n_val, m_val, fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_d(d, result);
            }
            _ => return ExecResult::Undefined,
        }
        ExecResult::Continue
    }

    fn exec_vfp_unop(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, m, size)) = self.decode_vfp_unary_regs(insn) else {
            return ExecResult::Undefined;
        };
        match size {
            16 => {
                let m_val = self.cpu.vfp.read_h_bits(m);
                let result = match insn.mnemonic {
                    Mnemonic::VABS => vabs_f16_bits(m_val),
                    Mnemonic::VNEG => vneg_f16_bits(m_val),
                    Mnemonic::VSQRT => vsqrt_f16_bits(m_val, &mut self.cpu.vfp.fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_h_bits(d, result);
            }
            32 => {
                let m_val = self.cpu.vfp.read_s(m);
                let result = match insn.mnemonic {
                    Mnemonic::VABS => vabs_f32(m_val),
                    Mnemonic::VNEG => vneg_f32(m_val),
                    Mnemonic::VSQRT => vsqrt_f32(m_val, &mut self.cpu.vfp.fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_s(d, result);
            }
            64 => {
                let m_val = self.cpu.vfp.read_d(m);
                let result = match insn.mnemonic {
                    Mnemonic::VABS => vabs_f64(m_val),
                    Mnemonic::VNEG => vneg_f64(m_val),
                    Mnemonic::VSQRT => vsqrt_f64(m_val, &mut self.cpu.vfp.fpscr),
                    _ => return ExecResult::Undefined,
                };
                self.cpu.vfp.write_d(d, result);
            }
            _ => return ExecResult::Undefined,
        }
        ExecResult::Continue
    }

    fn exec_vrint(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, m, size)) = self.decode_vfp_unary_regs(insn) else {
            return ExecResult::Undefined;
        };
        let Some((mode, exact)) = self.vrint_rounding(insn.mnemonic) else {
            return ExecResult::Undefined;
        };

        match size {
            16 => {
                let value = self.cpu.vfp.read_h_bits(m);
                let result = vrint_f16_bits(value, mode, exact, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_h_bits(d, result);
            }
            32 => {
                let value = self.cpu.vfp.read_s(m);
                let result = vrint_f32(value, mode, exact, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s(d, result);
            }
            64 => {
                let value = self.cpu.vfp.read_d(m);
                let result = vrint_f64(value, mode, exact, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_d(d, result);
            }
            _ => return ExecResult::Undefined,
        }

        ExecResult::Continue
    }

    fn exec_vcmp(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, m, size)) = self.decode_vfp_unary_regs(insn) else {
            return ExecResult::Undefined;
        };
        let with_zero = ((insn.raw >> 16) & 0xF) == 5;
        let signal_all_nans = insn.mnemonic == Mnemonic::VCMPE;
        match size {
            16 => {
                let rhs = if with_zero {
                    0
                } else {
                    self.cpu.vfp.read_h_bits(m)
                };
                vcmp_f16_bits_with_exception(
                    self.cpu.vfp.read_h_bits(d),
                    rhs,
                    signal_all_nans,
                    &mut self.cpu.vfp.fpscr,
                );
            }
            32 => {
                let rhs = if with_zero {
                    0.0
                } else {
                    self.cpu.vfp.read_s(m)
                };
                vcmp_f32_with_exception(
                    self.cpu.vfp.read_s(d),
                    rhs,
                    signal_all_nans,
                    &mut self.cpu.vfp.fpscr,
                );
            }
            64 => {
                let rhs = if with_zero {
                    0.0
                } else {
                    self.cpu.vfp.read_d(m)
                };
                vcmp_f64_with_exception(
                    self.cpu.vfp.read_d(d),
                    rhs,
                    signal_all_nans,
                    &mut self.cpu.vfp.fpscr,
                );
            }
            _ => return ExecResult::Undefined,
        }
        ExecResult::Continue
    }

    fn exec_vcvt(&mut self, insn: &DecodedInsn) -> ExecResult {
        if Self::is_neon_fp16_convert_shape(insn.raw) {
            return self.exec_neon_fp16_convert(insn);
        }

        if Self::is_neon_fp_fixed_convert_shape(insn.raw) {
            return self.exec_neon_fp_fixed_convert(insn);
        }

        if Self::is_neon_fp_convert_shape(insn.raw) {
            return self.exec_neon_fp_convert(insn);
        }

        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        let Some((d, m)) = self.decode_vcvt_regs(insn) else {
            return ExecResult::Undefined;
        };

        match insn.mnemonic {
            Mnemonic::VCVT_F32_S32 => {
                let value = self.cpu.vfp.read_s_bits(m) as i32;
                self.cpu.vfp.write_s(d, vcvt_f32_s32(value));
            }
            Mnemonic::VCVT_F32_U32 => {
                let value = self.cpu.vfp.read_s_bits(m);
                self.cpu.vfp.write_s(d, vcvt_f32_u32(value));
            }
            Mnemonic::VCVT_F16_S32 => {
                let value = self.cpu.vfp.read_s_bits(m) as i32;
                let bits = vcvt_f16_bits_f32(vcvt_f32_s32(value), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_h_bits(d, bits);
            }
            Mnemonic::VCVT_F16_U32 => {
                let value = self.cpu.vfp.read_s_bits(m);
                let bits = vcvt_f16_bits_f32(vcvt_f32_u32(value), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_h_bits(d, bits);
            }
            Mnemonic::VCVT_S32_F32 => {
                let value = vcvt_s32_f32(self.cpu.vfp.read_s(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVT_U32_F32 => {
                let value = vcvt_u32_f32(self.cpu.vfp.read_s(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVT_S32_F16 => {
                let value = vcvt_s32_f32(
                    vcvt_f32_f16_bits(self.cpu.vfp.read_h_bits(m)),
                    &mut self.cpu.vfp.fpscr,
                );
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVT_U32_F16 => {
                let value = vcvt_u32_f32(
                    vcvt_f32_f16_bits(self.cpu.vfp.read_h_bits(m)),
                    &mut self.cpu.vfp.fpscr,
                );
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVTR_S32_F32 => {
                let value = vcvtr_s32_f32(self.cpu.vfp.read_s(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVTR_U32_F32 => {
                let value = vcvtr_u32_f32(self.cpu.vfp.read_s(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVTR_S32_F16 => {
                let value = vcvtr_s32_f32(
                    vcvt_f32_f16_bits(self.cpu.vfp.read_h_bits(m)),
                    &mut self.cpu.vfp.fpscr,
                );
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVTR_U32_F16 => {
                let value = vcvtr_u32_f32(
                    vcvt_f32_f16_bits(self.cpu.vfp.read_h_bits(m)),
                    &mut self.cpu.vfp.fpscr,
                );
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVT_F64_S32 => {
                let value = self.cpu.vfp.read_s_bits(m) as i32;
                self.cpu.vfp.write_d(d, vcvt_f64_s32(value));
            }
            Mnemonic::VCVT_F64_U32 => {
                let value = self.cpu.vfp.read_s_bits(m);
                self.cpu.vfp.write_d(d, vcvt_f64_u32(value));
            }
            Mnemonic::VCVT_S32_F64 => {
                let value = vcvt_s32_f64(self.cpu.vfp.read_d(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVT_U32_F64 => {
                let value = vcvt_u32_f64(self.cpu.vfp.read_d(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVTR_S32_F64 => {
                let value = vcvtr_s32_f64(self.cpu.vfp.read_d(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVTR_U32_F64 => {
                let value = vcvtr_u32_f64(self.cpu.vfp.read_d(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVTA_S32_F32
            | Mnemonic::VCVTM_S32_F32
            | Mnemonic::VCVTN_S32_F32
            | Mnemonic::VCVTP_S32_F32 => {
                let Some(mode) = Self::directed_vcvt_rounding(insn.mnemonic) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_s32_f32_round(self.cpu.vfp.read_s(m), mode, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVTA_S32_F16
            | Mnemonic::VCVTM_S32_F16
            | Mnemonic::VCVTN_S32_F16
            | Mnemonic::VCVTP_S32_F16 => {
                let Some(mode) = Self::directed_vcvt_rounding(insn.mnemonic) else {
                    return ExecResult::Undefined;
                };
                let value = vcvt_s32_f32_round(
                    vcvt_f32_f16_bits(self.cpu.vfp.read_h_bits(m)),
                    mode,
                    &mut self.cpu.vfp.fpscr,
                );
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVTA_U32_F32
            | Mnemonic::VCVTM_U32_F32
            | Mnemonic::VCVTN_U32_F32
            | Mnemonic::VCVTP_U32_F32 => {
                let Some(mode) = Self::directed_vcvt_rounding(insn.mnemonic) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_u32_f32_round(self.cpu.vfp.read_s(m), mode, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVTA_U32_F16
            | Mnemonic::VCVTM_U32_F16
            | Mnemonic::VCVTN_U32_F16
            | Mnemonic::VCVTP_U32_F16 => {
                let Some(mode) = Self::directed_vcvt_rounding(insn.mnemonic) else {
                    return ExecResult::Undefined;
                };
                let value = vcvt_u32_f32_round(
                    vcvt_f32_f16_bits(self.cpu.vfp.read_h_bits(m)),
                    mode,
                    &mut self.cpu.vfp.fpscr,
                );
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVTA_S32_F64
            | Mnemonic::VCVTM_S32_F64
            | Mnemonic::VCVTN_S32_F64
            | Mnemonic::VCVTP_S32_F64 => {
                let Some(mode) = Self::directed_vcvt_rounding(insn.mnemonic) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_s32_f64_round(self.cpu.vfp.read_d(m), mode, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVTA_U32_F64
            | Mnemonic::VCVTM_U32_F64
            | Mnemonic::VCVTN_U32_F64
            | Mnemonic::VCVTP_U32_F64 => {
                let Some(mode) = Self::directed_vcvt_rounding(insn.mnemonic) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_u32_f64_round(self.cpu.vfp.read_d(m), mode, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVT_F64_F32 => {
                self.cpu
                    .vfp
                    .write_d(d, vcvt_f64_f32(self.cpu.vfp.read_s(m)));
            }
            Mnemonic::VCVT_F32_F64 => {
                let value = vcvt_f32_f64(self.cpu.vfp.read_d(m), &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s(d, value);
            }
            Mnemonic::VCVTB_F32_F16 | Mnemonic::VCVTT_F32_F16 => {
                let shift = if insn.mnemonic == Mnemonic::VCVTT_F32_F16 {
                    16
                } else {
                    0
                };
                let bits = (self.cpu.vfp.read_s_bits(m) >> shift) as u16;
                self.cpu.vfp.write_s(d, vcvt_f32_f16_bits(bits));
            }
            Mnemonic::VCVTB_F16_F32 | Mnemonic::VCVTT_F16_F32 => {
                let shift = if insn.mnemonic == Mnemonic::VCVTT_F16_F32 {
                    16
                } else {
                    0
                };
                let value = vcvt_f16_bits_f32(self.cpu.vfp.read_s(m), &mut self.cpu.vfp.fpscr);
                let old = self.cpu.vfp.read_s_bits(d);
                let mask = 0xFFFFu32 << shift;
                self.cpu
                    .vfp
                    .write_s_bits(d, (old & !mask) | ((value as u32) << shift));
            }
            Mnemonic::VCVT_F32_S32_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value = self.cpu.vfp.read_s_bits(d) as i32;
                self.cpu.vfp.write_s(d, vcvt_f32_s32_fixed(value, fbits));
            }
            Mnemonic::VCVT_F32_U32_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value = self.cpu.vfp.read_s_bits(d);
                self.cpu.vfp.write_s(d, vcvt_f32_u32_fixed(value, fbits));
            }
            Mnemonic::VCVT_S32_F32_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_s32_f32_fixed(self.cpu.vfp.read_s(d), fbits, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value as u32);
            }
            Mnemonic::VCVT_U32_F32_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_u32_f32_fixed(self.cpu.vfp.read_s(d), fbits, &mut self.cpu.vfp.fpscr);
                self.cpu.vfp.write_s_bits(d, value);
            }
            Mnemonic::VCVT_F64_S32_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value = self.cpu.vfp.read_d_bits(d) as u32 as i32;
                self.cpu.vfp.write_d(d, vcvt_f64_s32_fixed(value, fbits));
            }
            Mnemonic::VCVT_F64_U32_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value = self.cpu.vfp.read_d_bits(d) as u32;
                self.cpu.vfp.write_d(d, vcvt_f64_u32_fixed(value, fbits));
            }
            Mnemonic::VCVT_S32_F64_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_s32_f64_fixed(self.cpu.vfp.read_d(d), fbits, &mut self.cpu.vfp.fpscr);
                let old = self.cpu.vfp.read_d_bits(d) & 0xFFFF_FFFF_0000_0000;
                self.cpu.vfp.write_d_bits(d, old | (value as u32 as u64));
            }
            Mnemonic::VCVT_U32_F64_FIXED => {
                let Some(fbits) = Self::decode_vcvt_fixed_fbits(insn) else {
                    return ExecResult::Undefined;
                };
                let value =
                    vcvt_u32_f64_fixed(self.cpu.vfp.read_d(d), fbits, &mut self.cpu.vfp.fpscr);
                let old = self.cpu.vfp.read_d_bits(d) & 0xFFFF_FFFF_0000_0000;
                self.cpu.vfp.write_d_bits(d, old | value as u64);
            }
            _ => return ExecResult::Undefined,
        }

        ExecResult::Continue
    }

    fn is_neon_fp_convert_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 24) & 1) == 1
            && ((raw >> 23) & 1) == 1
            && ((raw >> 21) & 0x7) == 0b101
            && ((raw >> 16) & 0xF) == 0b1011
            && ((raw >> 8) & 0xE) == 0b0110
            && ((raw >> 5) & 1) == 0
            && ((raw >> 4) & 1) == 0
    }

    fn is_neon_fp16_convert_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 24) & 1) == 1
            && ((raw >> 23) & 1) == 1
            && ((raw >> 20) & 0x7) == 0b011
            && ((raw >> 16) & 0xF) == 0b0110
            && matches!((raw >> 8) & 0xF, 0b0110 | 0b0111)
            && ((raw >> 7) & 1) == 0
            && ((raw >> 6) & 1) == 0
            && ((raw >> 5) & 1) == 0
            && ((raw >> 4) & 1) == 0
    }

    fn is_neon_fp_fixed_convert_shape(raw: u32) -> bool {
        (raw >> 25) == 0b1111001
            && ((raw >> 23) & 1) == 1
            && ((raw >> 8) & 0xE) == 0b1110
            && ((raw >> 7) & 1) == 0
            && ((raw >> 4) & 1) == 1
            && ((raw >> 16) & 0x3F) >= 32
    }

    fn is_neon_directed_convert_shape(raw: u32) -> bool {
        (raw >> 24) == 0xF3
            && ((raw >> 23) & 1) == 1
            && ((raw >> 21) & 1) == 1
            && ((raw >> 20) & 1) == 1
            && ((raw >> 16) & 0x3) == 0b11
            && ((raw >> 10) & 0x3) == 0
            && ((raw >> 4) & 1) == 0
    }

    fn exec_neon_fp_convert(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_fp_convert_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, 4);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let result = match insn.mnemonic {
                    Mnemonic::VCVT_F32_S32 => u64::from(vcvt_f32_s32(elem as u32 as i32).to_bits()),
                    Mnemonic::VCVT_F32_U32 => u64::from(vcvt_f32_u32(elem as u32).to_bits()),
                    Mnemonic::VCVT_S32_F32 => {
                        let value =
                            vcvt_s32_f32(f32::from_bits(elem as u32), &mut self.cpu.vfp.fpscr);
                        u64::from(value as u32)
                    }
                    Mnemonic::VCVT_U32_F32 => {
                        let value =
                            vcvt_u32_f32(f32::from_bits(elem as u32), &mut self.cpu.vfp.fpscr);
                        u64::from(value)
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, 4, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_fp16_convert(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_fp16_convert_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let d = ((((insn.raw >> 22) & 1) << 4) | ((insn.raw >> 12) & 0xF)) as u8;
        let m = (insn.raw & 0xF) as u8;
        match insn.mnemonic {
            Mnemonic::VCVT_F16_F32 => {
                if (m & 1) != 0 || m + 1 >= 32 {
                    return ExecResult::Undefined;
                }
                let values = self.neon_read_vector_elements_u64(m, 2, 4);
                let mut out = Vec::with_capacity(values.len());
                for elem in values {
                    let value =
                        vcvt_f16_bits_f32(f32::from_bits(elem as u32), &mut self.cpu.vfp.fpscr);
                    out.push(u64::from(value));
                }
                self.neon_write_vector_elements_u64(d, 1, 2, &out);
            }
            Mnemonic::VCVT_F32_F16 => {
                if (d & 1) != 0 || d + 1 >= 32 {
                    return ExecResult::Undefined;
                }
                let values = self.neon_read_vector_elements_u64(m, 1, 2);
                let mut out = Vec::with_capacity(values.len());
                for elem in values {
                    out.push(u64::from(vcvt_f32_f16_bits(elem as u16).to_bits()));
                }
                self.neon_write_vector_elements_u64(d, 2, 4, &out);
            }
            _ => return ExecResult::Undefined,
        }

        ExecResult::Continue
    }

    fn exec_neon_fp_fixed_convert(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_fp_fixed_convert_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let fbits = 64 - ((insn.raw >> 16) & 0x3F);
        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, 4);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let result = match insn.mnemonic {
                    Mnemonic::VCVT_F32_S32_FIXED => {
                        u64::from(vcvt_f32_s32_fixed(elem as u32 as i32, fbits).to_bits())
                    }
                    Mnemonic::VCVT_F32_U32_FIXED => {
                        u64::from(vcvt_f32_u32_fixed(elem as u32, fbits).to_bits())
                    }
                    Mnemonic::VCVT_S32_F32_FIXED => {
                        let value = vcvt_s32_f32_fixed(
                            f32::from_bits(elem as u32),
                            fbits,
                            &mut self.cpu.vfp.fpscr,
                        );
                        u64::from(value as u32)
                    }
                    Mnemonic::VCVT_U32_F32_FIXED => {
                        let value = vcvt_u32_f32_fixed(
                            f32::from_bits(elem as u32),
                            fbits,
                            &mut self.cpu.vfp.fpscr,
                        );
                        u64::from(value)
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, 4, &out);
        }

        ExecResult::Continue
    }

    fn exec_neon_directed_convert(&mut self, insn: &DecodedInsn) -> ExecResult {
        if !self.cpu.vfp.is_enabled() {
            return ExecResult::Exception(ExceptionType::UndefinedInstruction);
        }
        if !Self::is_neon_directed_convert_shape(insn.raw) {
            return ExecResult::Undefined;
        }

        let size = match (insn.raw >> 18) & 0x3 {
            0b01 => NeonSize::H16,
            0b10 => NeonSize::S32,
            _ => return ExecResult::Undefined,
        };
        let unsigned = ((insn.raw >> 7) & 1) != 0;
        let mode = match (insn.raw >> 8) & 0x3 {
            0b00 => RoundingMode::RoundNearest,
            0b01 => RoundingMode::RoundPlusInf,
            0b10 => RoundingMode::RoundMinusInf,
            0b11 => RoundingMode::RoundZero,
            _ => unreachable!(),
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let q = ((insn.raw >> 6) & 1) != 0;
        let regs = if q { 2 } else { 1 };
        let d = (d_bit << 4) | vd;
        let m = (m_bit << 4) | vm;
        if q && ((d | m) & 1) != 0 {
            return ExecResult::Undefined;
        }
        if d + regs > 32 || m + regs > 32 {
            return ExecResult::Undefined;
        }

        let ebytes = (size.bits() / 8) as u8;
        for reg in 0..regs {
            let elements = self.neon_read_vector_elements_u64(m + reg, 1, ebytes);
            let mut out = Vec::with_capacity(elements.len());
            for elem in elements {
                let result = match size {
                    NeonSize::H16 => {
                        let value = vcvt_f32_f16_bits(elem as u16);
                        u64::from(Self::neon_float_to_int_lane(
                            value,
                            16,
                            unsigned,
                            mode,
                            &mut self.cpu.vfp.fpscr,
                        ))
                    }
                    NeonSize::S32 if unsigned => {
                        let value = vcvt_u32_f32_round(
                            f32::from_bits(elem as u32),
                            mode,
                            &mut self.cpu.vfp.fpscr,
                        );
                        u64::from(value)
                    }
                    NeonSize::S32 => {
                        let value = vcvt_s32_f32_round(
                            f32::from_bits(elem as u32),
                            mode,
                            &mut self.cpu.vfp.fpscr,
                        );
                        u64::from(value as u32)
                    }
                    _ => return ExecResult::Undefined,
                };
                out.push(result);
            }
            self.neon_write_vector_elements_u64(d + reg, 1, ebytes, &out);
        }

        ExecResult::Continue
    }

    fn neon_float_to_int_lane(
        value: f32,
        bits: u32,
        unsigned: bool,
        mode: RoundingMode,
        fpscr: &mut Fpscr,
    ) -> u32 {
        let rounded = match mode {
            RoundingMode::RoundNearest => value.round_ties_even(),
            RoundingMode::RoundPlusInf => value.ceil(),
            RoundingMode::RoundMinusInf => value.floor(),
            RoundingMode::RoundZero => value.trunc(),
            RoundingMode::RoundTiesAway => value.round(),
        };

        if unsigned {
            let max = (1u32 << bits) - 1;
            if rounded.is_nan() || rounded < 0.0 {
                fpscr.set_ioc(true);
                0
            } else if rounded >= max as f32 {
                fpscr.set_ioc(true);
                max
            } else {
                rounded as u32
            }
        } else {
            let min = -(1i32 << (bits - 1));
            let max = (1i32 << (bits - 1)) - 1;
            if rounded.is_nan() {
                fpscr.set_ioc(true);
                0
            } else if rounded >= max as f32 {
                fpscr.set_ioc(true);
                max as u32
            } else if rounded <= min as f32 {
                fpscr.set_ioc(true);
                (min as u32) & ((1u32 << bits) - 1)
            } else {
                (rounded as i32 as u32) & ((1u32 << bits) - 1)
            }
        }
    }

    fn directed_vcvt_rounding(mnemonic: Mnemonic) -> Option<RoundingMode> {
        match mnemonic {
            Mnemonic::VCVTA_S32_F32
            | Mnemonic::VCVTA_S32_F16
            | Mnemonic::VCVTA_U32_F32
            | Mnemonic::VCVTA_U32_F16
            | Mnemonic::VCVTA_S32_F64
            | Mnemonic::VCVTA_U32_F64 => Some(RoundingMode::RoundTiesAway),
            Mnemonic::VCVTN_S32_F32
            | Mnemonic::VCVTN_S32_F16
            | Mnemonic::VCVTN_U32_F32
            | Mnemonic::VCVTN_U32_F16
            | Mnemonic::VCVTN_S32_F64
            | Mnemonic::VCVTN_U32_F64 => Some(RoundingMode::RoundNearest),
            Mnemonic::VCVTP_S32_F32
            | Mnemonic::VCVTP_S32_F16
            | Mnemonic::VCVTP_U32_F32
            | Mnemonic::VCVTP_U32_F16
            | Mnemonic::VCVTP_S32_F64
            | Mnemonic::VCVTP_U32_F64 => Some(RoundingMode::RoundPlusInf),
            Mnemonic::VCVTM_S32_F32
            | Mnemonic::VCVTM_S32_F16
            | Mnemonic::VCVTM_U32_F32
            | Mnemonic::VCVTM_U32_F16
            | Mnemonic::VCVTM_S32_F64
            | Mnemonic::VCVTM_U32_F64 => Some(RoundingMode::RoundMinusInf),
            _ => None,
        }
    }

    fn vrint_rounding(&self, mnemonic: Mnemonic) -> Option<(RoundingMode, bool)> {
        match mnemonic {
            Mnemonic::VRINTA_F16 | Mnemonic::VRINTA_F32 | Mnemonic::VRINTA_F64 => {
                Some((RoundingMode::RoundTiesAway, false))
            }
            Mnemonic::VRINTN_F16 | Mnemonic::VRINTN_F32 | Mnemonic::VRINTN_F64 => {
                Some((RoundingMode::RoundNearest, false))
            }
            Mnemonic::VRINTP_F16 | Mnemonic::VRINTP_F32 | Mnemonic::VRINTP_F64 => {
                Some((RoundingMode::RoundPlusInf, false))
            }
            Mnemonic::VRINTM_F16 | Mnemonic::VRINTM_F32 | Mnemonic::VRINTM_F64 => {
                Some((RoundingMode::RoundMinusInf, false))
            }
            Mnemonic::VRINTZ_F16 | Mnemonic::VRINTZ_F32 | Mnemonic::VRINTZ_F64 => {
                Some((RoundingMode::RoundZero, false))
            }
            Mnemonic::VRINTR_F16 | Mnemonic::VRINTR_F32 | Mnemonic::VRINTR_F64 => {
                Some((self.cpu.vfp.fpscr.rmode(), false))
            }
            Mnemonic::VRINTX_F16 | Mnemonic::VRINTX_F32 | Mnemonic::VRINTX_F64 => {
                Some((self.cpu.vfp.fpscr.rmode(), true))
            }
            _ => None,
        }
    }

    fn decode_vfp_mem(&mut self, insn: &DecodedInsn) -> Option<(u32, u32, u8)> {
        let u = (insn.raw >> 23) & 1;
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let rn = ((insn.raw >> 16) & 0xF) as usize;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let size = (insn.raw >> 8) & 0x3;
        let size = (insn.raw >> 8) & 0x3;
        let scale = if size == 1 { 2 } else { 4 };
        let imm = (insn.raw & 0xFF).wrapping_mul(scale);
        let base = if rn == 15 {
            self.cpu.get_pc() & !3
        } else {
            self.reg(rn)
        };
        let addr = if u == 1 {
            base.wrapping_add(imm)
        } else {
            base.wrapping_sub(imm)
        };
        match size {
            1 => Some((addr, 16, (vd << 1) | d_bit)),
            2 => Some((addr, 32, (vd << 1) | d_bit)),
            3 => Some((addr, 64, (d_bit << 4) | vd)),
            _ => None,
        }
    }

    fn decode_vfp_block_mem(
        &mut self,
        insn: &DecodedInsn,
    ) -> Option<(u32, u32, u32, u8, u8, bool, usize)> {
        let p = (insn.raw >> 24) & 1;
        let u = (insn.raw >> 23) & 1;
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let w = ((insn.raw >> 21) & 1) != 0;
        let rn = ((insn.raw >> 16) & 0xF) as usize;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let size = (insn.raw >> 8) & 0x3;
        let words = (insn.raw & 0xFF) as u8;
        if words == 0 || !matches!((p, u, w), (0, 1, _) | (1, 0, true)) {
            return None;
        }

        let (elem_size, first, count) = match size {
            2 => (32, (vd << 1) | d_bit, words),
            3 if (words & 1) == 0 => (64, (d_bit << 4) | vd, words / 2),
            _ => return None,
        };
        if count == 0 || first.checked_add(count - 1)? >= 32 {
            return None;
        }

        let byte_count = (words as u32).wrapping_mul(4);
        let base = if rn == 15 {
            self.cpu.get_pc() & !3
        } else {
            self.reg(rn)
        };
        let start = match (p, u) {
            (0, 1) => base,
            (1, 0) => base.wrapping_sub(byte_count),
            _ => return None,
        };
        let final_addr = if u == 1 {
            base.wrapping_add(byte_count)
        } else {
            base.wrapping_sub(byte_count)
        };

        Some((start, final_addr, elem_size, first, count, w, rn))
    }

    fn decode_neon_vld_vst_multiple(&self, insn: &DecodedInsn) -> Option<NeonStructMem> {
        let ty = (insn.raw >> 8) & 0xF;
        let size = ((insn.raw >> 6) & 0x3) as u8;
        let (regs, inc, streams) = match insn.mnemonic {
            Mnemonic::VLD1 | Mnemonic::VST1 => match ty {
                0b0111 => (1, 1, 1),
                0b1010 => (2, 1, 1),
                0b0110 => (3, 1, 1),
                0b0010 => (4, 1, 1),
                _ => return None,
            },
            Mnemonic::VLD2 | Mnemonic::VST2 => match ty {
                0b1000 => (1, 1, 2),
                0b1001 => (1, 2, 2),
                0b0011 => (2, 2, 2),
                _ => return None,
            },
            Mnemonic::VLD3 | Mnemonic::VST3 => match ty {
                0b0100 => (1, 1, 3),
                0b0101 => (1, 2, 3),
                _ => return None,
            },
            Mnemonic::VLD4 | Mnemonic::VST4 => match ty {
                0b0000 => (1, 1, 4),
                0b0001 => (1, 2, 4),
                _ => return None,
            },
            _ => return None,
        };

        let align = (insn.raw >> 4) & 0x3;
        match insn.mnemonic {
            Mnemonic::VLD1 | Mnemonic::VST1 => {
                if (regs == 1 || regs == 3) && (align & 0b10) != 0 {
                    return None;
                }
                if regs == 2 && align == 0b11 {
                    return None;
                }
            }
            Mnemonic::VLD2 | Mnemonic::VST2 => {
                if size == 0b11 || (regs == 1 && align == 0b11) {
                    return None;
                }
            }
            Mnemonic::VLD3 | Mnemonic::VST3 => {
                if size == 0b11 || (align & 0b10) != 0 {
                    return None;
                }
            }
            Mnemonic::VLD4 | Mnemonic::VST4 => {
                if size == 0b11 {
                    return None;
                }
            }
            _ => return None,
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let first = (d_bit << 4) | vd;
        let last = first
            .checked_add((streams - 1) * inc)?
            .checked_add(regs - 1)?;
        if last >= 32 {
            return None;
        }

        let rn = ((insn.raw >> 16) & 0xF) as usize;
        if rn == 15 {
            return None;
        }
        let rm = (insn.raw & 0xF) as usize;
        let writeback = rm != 15;
        Some(NeonStructMem {
            addr: self.reg(rn),
            regs,
            first,
            inc,
            ebytes: 1 << size,
            writeback,
            rn,
            rm,
        })
    }

    fn decode_neon_vld_all_lanes(&self, insn: &DecodedInsn) -> Option<NeonAllLanesMem> {
        if ((insn.raw >> 23) & 1) != 1 || ((insn.raw >> 21) & 1) != 1 {
            return None;
        }

        let ty = (insn.raw >> 8) & 0xF;
        let size = ((insn.raw >> 6) & 0x3) as u8;
        let t = ((insn.raw >> 5) & 1) as u8;
        let a = ((insn.raw >> 4) & 1) as u8;
        let (streams, regs, inc, ebytes) = match insn.mnemonic {
            Mnemonic::VLD1 if ty == 0b1100 => {
                if size == 0b11 || (size == 0 && a == 1) {
                    return None;
                }
                (1, if t == 0 { 1 } else { 2 }, 1, 1 << size)
            }
            Mnemonic::VLD2 if ty == 0b1101 => {
                if size == 0b11 {
                    return None;
                }
                (2, 1, if t == 0 { 1 } else { 2 }, 1 << size)
            }
            Mnemonic::VLD3 if ty == 0b1110 => {
                if size == 0b11 || a == 1 {
                    return None;
                }
                (3, 1, if t == 0 { 1 } else { 2 }, 1 << size)
            }
            Mnemonic::VLD4 if ty == 0b1111 => {
                if size == 0b11 && a == 0 {
                    return None;
                }
                (
                    4,
                    1,
                    if t == 0 { 1 } else { 2 },
                    if size == 0b11 { 4 } else { 1 << size },
                )
            }
            _ => return None,
        };

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let first = (d_bit << 4) | vd;
        let last = first
            .checked_add((streams - 1) * inc)?
            .checked_add(regs - 1)?;
        if last >= 32 {
            return None;
        }

        let rn = ((insn.raw >> 16) & 0xF) as usize;
        if rn == 15 {
            return None;
        }
        let rm = (insn.raw & 0xF) as usize;
        Some(NeonAllLanesMem {
            addr: self.reg(rn),
            streams,
            regs,
            first,
            inc,
            ebytes,
            writeback: rm != 15,
            rn,
            rm,
        })
    }

    fn decode_neon_vld_vst_single_lane(&self, insn: &DecodedInsn) -> Option<NeonSingleLaneMem> {
        if ((insn.raw >> 23) & 1) != 1 {
            return None;
        }
        let l = (insn.raw >> 21) & 1;
        if (l == 1)
            != matches!(
                insn.mnemonic,
                Mnemonic::VLD1 | Mnemonic::VLD2 | Mnemonic::VLD3 | Mnemonic::VLD4
            )
        {
            return None;
        }

        let size = ((insn.raw >> 10) & 0x3) as u8;
        let streams = (((insn.raw >> 8) & 0x3) + 1) as u8;
        let index_align = ((insn.raw >> 4) & 0xF) as u8;
        let (ebytes, index, inc) = Self::decode_neon_single_lane_shape(streams, size, index_align)?;

        let expected = match insn.mnemonic {
            Mnemonic::VLD1 | Mnemonic::VST1 => 1,
            Mnemonic::VLD2 | Mnemonic::VST2 => 2,
            Mnemonic::VLD3 | Mnemonic::VST3 => 3,
            Mnemonic::VLD4 | Mnemonic::VST4 => 4,
            _ => return None,
        };
        if streams != expected {
            return None;
        }

        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let first = (d_bit << 4) | vd;
        let last = first.checked_add((streams - 1) * inc)?;
        if last >= 32 {
            return None;
        }

        let rn = ((insn.raw >> 16) & 0xF) as usize;
        if rn == 15 {
            return None;
        }
        let rm = (insn.raw & 0xF) as usize;
        Some(NeonSingleLaneMem {
            addr: self.reg(rn),
            streams,
            first,
            inc,
            ebytes,
            index,
            writeback: rm != 15,
            rn,
            rm,
        })
    }

    fn decode_neon_single_lane_shape(
        streams: u8,
        size: u8,
        index_align: u8,
    ) -> Option<(u8, u8, u8)> {
        match (streams, size) {
            (1, 0) if (index_align & 0b0001) == 0 => Some((1, index_align >> 1, 1)),
            (1, 1) if (index_align & 0b0010) == 0 => Some((2, index_align >> 2, 1)),
            (1, 2)
                if (index_align & 0b0100) == 0 && matches!(index_align & 0b0011, 0b00 | 0b11) =>
            {
                Some((4, index_align >> 3, 1))
            }
            (2, 0) => Some((1, index_align >> 1, 1)),
            (2, 1) => Some((
                2,
                index_align >> 2,
                if (index_align & 0b0010) == 0 { 1 } else { 2 },
            )),
            (2, 2) if (index_align & 0b0010) == 0 => Some((
                4,
                index_align >> 3,
                if (index_align & 0b0100) == 0 { 1 } else { 2 },
            )),
            (3, 0) if (index_align & 0b0001) == 0 => Some((1, index_align >> 1, 1)),
            (3, 1) if (index_align & 0b0001) == 0 => Some((
                2,
                index_align >> 2,
                if (index_align & 0b0010) == 0 { 1 } else { 2 },
            )),
            (3, 2) if (index_align & 0b0011) == 0 => Some((
                4,
                index_align >> 3,
                if (index_align & 0b0100) == 0 { 1 } else { 2 },
            )),
            (4, 0) => Some((1, index_align >> 1, 1)),
            (4, 1) => Some((
                2,
                index_align >> 2,
                if (index_align & 0b0010) == 0 { 1 } else { 2 },
            )),
            (4, 2) if (index_align & 0b0011) != 0b0011 => Some((
                4,
                index_align >> 3,
                if (index_align & 0b0100) == 0 { 1 } else { 2 },
            )),
            _ => None,
        }
    }

    fn neon_struct_writeback(&self, base: u32, regs: u8, streams: u8, rm: usize) -> u32 {
        if rm == 13 {
            base.wrapping_add((regs as u32) * (streams as u32) * 8)
        } else {
            base.wrapping_add(self.reg(rm))
        }
    }

    fn neon_lane_writeback(&self, base: u32, streams: u8, ebytes: u8, rm: usize) -> u32 {
        if rm == 13 {
            base.wrapping_add((streams as u32) * (ebytes as u32))
        } else {
            base.wrapping_add(self.reg(rm))
        }
    }

    fn neon_replicate_elem(value: u32, ebytes: u8) -> u64 {
        let bits = (ebytes * 8) as u32;
        let mask = if ebytes == 4 {
            u32::MAX as u64
        } else {
            (1u64 << bits) - 1
        };
        let elem = (value as u64) & mask;
        let mut out = 0u64;
        let lanes = 8 / ebytes;
        for lane in 0..lanes {
            out |= elem << ((lane * ebytes * 8) as u32);
        }
        out
    }

    fn neon_read_d_elem(&self, dreg: u8, element: u8, ebytes: u8) -> u32 {
        let shift = (element * ebytes * 8) as u32;
        let mask = if ebytes == 4 {
            u32::MAX as u64
        } else {
            (1u64 << (ebytes * 8)) - 1
        };
        ((self.cpu.vfp.read_d_bits(dreg) >> shift) & mask) as u32
    }

    fn neon_read_d_elem_u64(&self, dreg: u8, element: u8, ebytes: u8) -> u64 {
        let shift = (element * ebytes * 8) as u32;
        let mask = if ebytes == 8 {
            u64::MAX
        } else {
            (1u64 << (ebytes * 8)) - 1
        };
        (self.cpu.vfp.read_d_bits(dreg) >> shift) & mask
    }

    fn neon_sign_extend_elem(value: u32, bits: u32) -> i64 {
        let shift = 64 - bits;
        (((value as u64) << shift) as i64) >> shift
    }

    fn neon_sign_extend_elem_u64(value: u64, bits: u32) -> i128 {
        let shift = 128 - bits;
        ((value as i128) << shift) >> shift
    }

    fn neon_signed_saturate(value: i64, bits: u32) -> (i64, bool) {
        let min = -(1i64 << (bits - 1));
        let max = (1i64 << (bits - 1)) - 1;
        if value < min {
            (min, true)
        } else if value > max {
            (max, true)
        } else {
            (value, false)
        }
    }

    fn neon_signed_saturate_i128(value: i128, bits: u32) -> (i128, bool) {
        let min = -(1i128 << (bits - 1));
        let max = (1i128 << (bits - 1)) - 1;
        if value < min {
            (min, true)
        } else if value > max {
            (max, true)
        } else {
            (value, false)
        }
    }

    fn neon_unsigned_saturate(value: i128, bits: u32) -> (u64, bool) {
        let max = if bits == 64 {
            u64::MAX as i128
        } else {
            (1i128 << bits) - 1
        };
        if value < 0 {
            (0, true)
        } else if value > max {
            (max as u64, true)
        } else {
            (value as u64, false)
        }
    }

    fn neon_pack_signed_elem(value: i64, bits: u32) -> u32 {
        let mask = if bits == 32 {
            u32::MAX as u64
        } else {
            (1u64 << bits) - 1
        };
        (value as u64 & mask) as u32
    }

    fn neon_pack_signed_elem_i128(value: i128, bits: u32) -> u64 {
        let mask = if bits == 64 {
            u64::MAX as u128
        } else {
            (1u128 << bits) - 1
        };
        (value as u128 & mask) as u64
    }

    fn neon_doubling_mulh_elem(lhs: u64, rhs: u64, bits: u32, rounding: bool) -> (u64, bool) {
        let lhs = Self::neon_sign_extend_elem_u64(lhs, bits);
        let rhs = Self::neon_sign_extend_elem_u64(rhs, bits);
        let round_const = if rounding { 1i128 << (bits - 1) } else { 0 };
        let product = (2 * lhs * rhs) + round_const;
        let shifted = product >> bits;
        let (result, saturated) = Self::neon_signed_saturate_i128(shifted, bits);
        (Self::neon_pack_signed_elem_i128(result, bits), saturated)
    }

    fn neon_read_vector_elements(&self, first: u8, regs: u8, ebytes: u8) -> Vec<u32> {
        let elements_per_d = 8 / ebytes;
        let mut elements = Vec::with_capacity(regs as usize * elements_per_d as usize);
        for reg in 0..regs {
            for element in 0..elements_per_d {
                elements.push(self.neon_read_d_elem(first + reg, element, ebytes));
            }
        }
        elements
    }

    fn neon_read_vector_elements_u64(&self, first: u8, regs: u8, ebytes: u8) -> Vec<u64> {
        let elements_per_d = 8 / ebytes;
        let mut elements = Vec::with_capacity(regs as usize * elements_per_d as usize);
        for reg in 0..regs {
            for element in 0..elements_per_d {
                elements.push(self.neon_read_d_elem_u64(first + reg, element, ebytes));
            }
        }
        elements
    }

    fn neon_write_vector_elements(&mut self, first: u8, regs: u8, ebytes: u8, elements: &[u32]) {
        let elements_per_d = 8 / ebytes;
        debug_assert_eq!(elements.len(), regs as usize * elements_per_d as usize);
        let mut next = 0;
        for reg in 0..regs {
            for element in 0..elements_per_d {
                self.neon_write_d_elem(first + reg, element, ebytes, elements[next]);
                next += 1;
            }
        }
    }

    fn neon_write_vector_elements_u64(
        &mut self,
        first: u8,
        regs: u8,
        ebytes: u8,
        elements: &[u64],
    ) {
        let elements_per_d = 8 / ebytes;
        debug_assert_eq!(elements.len(), regs as usize * elements_per_d as usize);
        let mut next = 0;
        for reg in 0..regs {
            for element in 0..elements_per_d {
                self.neon_write_d_elem_u64(first + reg, element, ebytes, elements[next]);
                next += 1;
            }
        }
    }

    fn neon_write_d_elem(&mut self, dreg: u8, element: u8, ebytes: u8, value: u32) {
        let shift = (element * ebytes * 8) as u32;
        let mask = if ebytes == 4 {
            u32::MAX as u64
        } else {
            (1u64 << (ebytes * 8)) - 1
        };
        let old = self.cpu.vfp.read_d_bits(dreg);
        let bits = (old & !(mask << shift)) | (((value as u64) & mask) << shift);
        self.cpu.vfp.write_d_bits(dreg, bits);
    }

    fn neon_write_d_elem_u64(&mut self, dreg: u8, element: u8, ebytes: u8, value: u64) {
        let shift = (element * ebytes * 8) as u32;
        let mask = if ebytes == 8 {
            u64::MAX
        } else {
            (1u64 << (ebytes * 8)) - 1
        };
        let old = self.cpu.vfp.read_d_bits(dreg);
        let bits = (old & !(mask << shift)) | ((value & mask) << shift);
        self.cpu.vfp.write_d_bits(dreg, bits);
    }

    fn neon_read_mem_elem(&self, addr: u32, ebytes: u8) -> Result<u32, MemoryError> {
        match ebytes {
            1 => self.mem.read_byte(addr).map(|v| v as u32),
            2 => self.mem.read_halfword(addr).map(|v| v as u32),
            4 => self.mem.read_word(addr),
            _ => Err(MemoryError::OutOfBounds(addr)),
        }
    }

    fn neon_write_mem_elem(
        &mut self,
        addr: u32,
        ebytes: u8,
        value: u32,
    ) -> Result<(), MemoryError> {
        match ebytes {
            1 => self.mem.write_byte(addr, value as u8),
            2 => self.mem.write_halfword(addr, value as u16),
            4 => self.mem.write_word(addr, value),
            _ => Err(MemoryError::OutOfBounds(addr)),
        }
    }

    fn decode_vcvt_regs(&self, insn: &DecodedInsn) -> Option<(u8, u8)> {
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        let d_s = (vd << 1) | d_bit;
        let d_d = (d_bit << 4) | vd;
        let m_s = (vm << 1) | m_bit;
        let m_d = (m_bit << 4) | vm;

        match insn.mnemonic {
            Mnemonic::VCVT_F32_S32
            | Mnemonic::VCVT_F32_U32
            | Mnemonic::VCVT_F16_S32
            | Mnemonic::VCVT_F16_U32
            | Mnemonic::VCVT_S32_F32
            | Mnemonic::VCVT_U32_F32
            | Mnemonic::VCVT_S32_F16
            | Mnemonic::VCVT_U32_F16
            | Mnemonic::VCVTR_S32_F32
            | Mnemonic::VCVTR_U32_F32
            | Mnemonic::VCVTR_S32_F16
            | Mnemonic::VCVTR_U32_F16
            | Mnemonic::VCVTA_S32_F32
            | Mnemonic::VCVTA_U32_F32
            | Mnemonic::VCVTA_S32_F16
            | Mnemonic::VCVTA_U32_F16
            | Mnemonic::VCVTM_S32_F32
            | Mnemonic::VCVTM_U32_F32
            | Mnemonic::VCVTM_S32_F16
            | Mnemonic::VCVTM_U32_F16
            | Mnemonic::VCVTN_S32_F32
            | Mnemonic::VCVTN_U32_F32
            | Mnemonic::VCVTN_S32_F16
            | Mnemonic::VCVTN_U32_F16
            | Mnemonic::VCVTP_S32_F32
            | Mnemonic::VCVTP_U32_F32
            | Mnemonic::VCVTP_S32_F16
            | Mnemonic::VCVTP_U32_F16
            | Mnemonic::VCVTB_F32_F16
            | Mnemonic::VCVTT_F32_F16
            | Mnemonic::VCVTB_F16_F32
            | Mnemonic::VCVTT_F16_F32 => Some((d_s, m_s)),
            Mnemonic::VCVT_F32_S32_FIXED
            | Mnemonic::VCVT_F32_U32_FIXED
            | Mnemonic::VCVT_S32_F32_FIXED
            | Mnemonic::VCVT_U32_F32_FIXED => Some((d_s, d_s)),
            Mnemonic::VCVT_F64_S32 | Mnemonic::VCVT_F64_U32 | Mnemonic::VCVT_F64_F32 => {
                Some((d_d, m_s))
            }
            Mnemonic::VCVT_F64_S32_FIXED
            | Mnemonic::VCVT_F64_U32_FIXED
            | Mnemonic::VCVT_S32_F64_FIXED
            | Mnemonic::VCVT_U32_F64_FIXED => Some((d_d, d_d)),
            Mnemonic::VCVT_S32_F64
            | Mnemonic::VCVT_U32_F64
            | Mnemonic::VCVT_F32_F64
            | Mnemonic::VCVTR_S32_F64
            | Mnemonic::VCVTR_U32_F64
            | Mnemonic::VCVTA_S32_F64
            | Mnemonic::VCVTA_U32_F64
            | Mnemonic::VCVTM_S32_F64
            | Mnemonic::VCVTM_U32_F64
            | Mnemonic::VCVTN_S32_F64
            | Mnemonic::VCVTN_U32_F64
            | Mnemonic::VCVTP_S32_F64
            | Mnemonic::VCVTP_U32_F64 => Some((d_s, m_d)),
            _ => None,
        }
    }

    fn decode_vcvt_fixed_fbits(insn: &DecodedInsn) -> Option<u32> {
        if ((insn.raw >> 7) & 1) == 0 {
            return None;
        }
        let imm5 = ((insn.raw & 0xF) << 1) | ((insn.raw >> 5) & 1);
        Some(32 - imm5)
    }

    fn decode_vfp_cond_select_regs(&self, insn: &DecodedInsn) -> Option<(u8, u8, u8, u32)> {
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        match (insn.raw >> 8) & 0x3 {
            1 => Some(((vd << 1) | d_bit, (vn << 1) | n_bit, (vm << 1) | m_bit, 16)),
            2 => Some(((vd << 1) | d_bit, (vn << 1) | n_bit, (vm << 1) | m_bit, 32)),
            3 => Some(((d_bit << 4) | vd, (n_bit << 4) | vn, (m_bit << 4) | vm, 64)),
            _ => None,
        }
    }

    fn decode_vfp_ternary_regs(&self, insn: &DecodedInsn) -> Option<(u8, u8, u8, u32)> {
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let n_bit = ((insn.raw >> 7) & 1) as u8;
        let vn = ((insn.raw >> 16) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        match (insn.raw >> 8) & 0x3 {
            1 => Some(((vd << 1) | d_bit, (vn << 1) | n_bit, (vm << 1) | m_bit, 16)),
            2 => Some(((vd << 1) | d_bit, (vn << 1) | n_bit, (vm << 1) | m_bit, 32)),
            3 => Some(((d_bit << 4) | vd, (n_bit << 4) | vn, (m_bit << 4) | vm, 64)),
            _ => None,
        }
    }

    fn decode_vfp_unary_regs(&self, insn: &DecodedInsn) -> Option<(u8, u8, u32)> {
        let d_bit = ((insn.raw >> 22) & 1) as u8;
        let vd = ((insn.raw >> 12) & 0xF) as u8;
        let m_bit = ((insn.raw >> 5) & 1) as u8;
        let vm = (insn.raw & 0xF) as u8;
        match (insn.raw >> 8) & 0x3 {
            1 => Some(((vd << 1) | d_bit, (vm << 1) | m_bit, 16)),
            2 => Some(((vd << 1) | d_bit, (vm << 1) | m_bit, 32)),
            3 => Some(((d_bit << 4) | vd, (m_bit << 4) | vm, 64)),
            _ => None,
        }
    }

    // =========================================================================
    // Bit Manipulation
    // =========================================================================

    fn exec_clz(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let result = self.reg(m).leading_zeros();
        self.set_reg(d, result)
    }

    fn exec_rev(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let result = self.reg(m).swap_bytes();
        self.set_reg(d, result)
    }

    fn exec_rev16(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let val = self.reg(m);
        let result = ((val >> 8) & 0x00FF00FF) | ((val << 8) & 0xFF00FF00);
        self.set_reg(d, result)
    }

    fn exec_revsh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let val = self.reg(m);
        // Byte-reverse the low halfword and sign-extend
        let lo = ((val & 0xFF) << 8) | ((val >> 8) & 0xFF);
        let result = sign_extend(lo & 0xFFFF, 16);
        self.set_reg(d, result)
    }

    fn exec_rbit(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let result = self.reg(m).reverse_bits();
        self.set_reg(d, result)
    }

    // =========================================================================
    // Bit Field Operations
    // =========================================================================

    /// Bitfield instruction fields (Rd, Rn, lsb, five) where `five` is the
    /// width-minus-1 (SBFX/UBFX) or msb (BFI/BFC) field. Handles A32 and T32.
    fn bitfield_fields(&self, insn: &DecodedInsn) -> (usize, usize, u32, u32) {
        let raw = insn.raw;
        if insn.state.is_thumb() {
            let d = ((raw >> 8) & 0xF) as usize;
            let n = ((raw >> 16) & 0xF) as usize;
            let lsb = (((raw >> 12) & 0x7) << 2) | ((raw >> 6) & 0x3);
            (d, n, lsb, raw & 0x1F)
        } else {
            let d = ((raw >> 12) & 0xF) as usize;
            let n = (raw & 0xF) as usize;
            (d, n, (raw >> 7) & 0x1F, (raw >> 16) & 0x1F)
        }
    }

    fn exec_bfc(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, _, lsb, msb) = self.bitfield_fields(insn);
        if msb < lsb {
            return ExecResult::Continue;
        }
        let width = msb - lsb + 1;
        let mask = (((1u64 << width) - 1) as u32) << lsb;
        self.cpu.regs[d] &= !mask;
        ExecResult::Continue
    }

    fn exec_bfi(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, lsb, msb) = self.bitfield_fields(insn);
        if msb < lsb {
            return ExecResult::Continue;
        }
        let width = msb - lsb + 1;
        let mask = (((1u64 << width) - 1) as u32) << lsb;
        let src = (self.reg(n) << lsb) & mask;
        self.cpu.regs[d] = (self.cpu.regs[d] & !mask) | src;
        ExecResult::Continue
    }

    fn exec_ubfx(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, lsb, w) = self.bitfield_fields(insn);
        let width = w + 1;
        let mask = ((1u64 << width) - 1) as u32;
        let result = (self.reg(n) >> lsb) & mask;
        self.set_reg(d, result)
    }

    fn exec_sbfx(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, lsb, w) = self.bitfield_fields(insn);
        let width = w + 1;
        let mask = ((1u64 << width) - 1) as u32;
        let extracted = (self.reg(n) >> lsb) & mask;
        let result = sign_extend(extracted, width);
        self.set_reg(d, result)
    }

    // =========================================================================
    // Extension Operations
    // =========================================================================

    fn exec_sxtb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let rotation = if insn.state.is_thumb() {
            0
        } else {
            ((insn.raw >> 10) & 3) * 8
        };
        let rotated = self.reg(m).rotate_right(rotation);
        let result = sign_extend(rotated & 0xFF, 8);
        self.set_reg(d, result)
    }

    fn exec_sxth(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let rotation = if insn.state.is_thumb() {
            0
        } else {
            ((insn.raw >> 10) & 3) * 8
        };
        let rotated = self.reg(m).rotate_right(rotation);
        let result = sign_extend(rotated & 0xFFFF, 16);
        self.set_reg(d, result)
    }

    fn exec_uxtb(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let rotation = if insn.state.is_thumb() {
            0
        } else {
            ((insn.raw >> 10) & 3) * 8
        };
        let rotated = self.reg(m).rotate_right(rotation);
        let result = rotated & 0xFF;
        self.set_reg(d, result)
    }

    fn exec_uxth(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, m) = self.dm_ops(insn);
        let rotation = if insn.state.is_thumb() {
            0
        } else {
            ((insn.raw >> 10) & 3) * 8
        };
        let rotated = self.reg(m).rotate_right(rotation);
        let result = rotated & 0xFFFF;
        self.set_reg(d, result)
    }

    // =========================================================================
    // Saturating Arithmetic
    // =========================================================================

    /// Saturate instruction fields: (Rd, Rn, sat_imm5, sh, imm5). A32/T32.
    fn sat_fields(&self, insn: &DecodedInsn) -> (usize, usize, u32, bool, u32) {
        let raw = insn.raw;
        if insn.state.is_thumb() {
            let d = ((raw >> 8) & 0xF) as usize;
            let n = ((raw >> 16) & 0xF) as usize;
            let imm5 = (((raw >> 12) & 0x7) << 2) | ((raw >> 6) & 0x3);
            (d, n, raw & 0x1F, (raw >> 21) & 1 != 0, imm5)
        } else {
            let d = ((raw >> 12) & 0xF) as usize;
            let n = (raw & 0xF) as usize;
            (
                d,
                n,
                (raw >> 16) & 0x1F,
                (raw >> 6) & 1 != 0,
                (raw >> 7) & 0x1F,
            )
        }
    }

    fn exec_usat(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, sat_imm, sh, imm5) = self.sat_fields(insn);

        let shift_amount = if imm5 == 0 && sh { 32 } else { imm5 };
        let shift_type = if sh { ShiftType::ASR } else { ShiftType::LSL };
        let operand = shift_c(self.reg(n), shift_type, shift_amount, false).0;

        let max_val = (1u32 << sat_imm).saturating_sub(1);
        let signed_operand = operand as i32;

        let result = if signed_operand < 0 {
            self.cpu.cpsr.q = true;
            0
        } else if operand > max_val {
            self.cpu.cpsr.q = true;
            max_val
        } else {
            operand
        };

        self.set_reg(d, result)
    }

    fn exec_ssat(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (d, n, sat_imm0, sh, imm5) = self.sat_fields(insn);
        let sat_imm = sat_imm0 + 1;

        let shift_amount = if imm5 == 0 && sh { 32 } else { imm5 };
        let shift_type = if sh { ShiftType::ASR } else { ShiftType::LSL };
        let operand = shift_c(self.reg(n), shift_type, shift_amount, false).0 as i32;

        let max_val = (1i32 << (sat_imm - 1)) - 1;
        let min_val = -(1i32 << (sat_imm - 1));

        let result = if operand > max_val {
            self.cpu.cpsr.q = true;
            max_val as u32
        } else if operand < min_val {
            self.cpu.cpsr.q = true;
            min_val as u32
        } else {
            operand as u32
        };

        self.set_reg(d, result)
    }

    // =========================================================================
    // AArch32 media / DSP (A32 encodings; operation derived from the raw word)
    // =========================================================================

    /// (Rd, Rn, Rm) for 3-register media ops (A32 / T32 layouts).
    fn media_regs(&self, insn: &DecodedInsn) -> (usize, usize, usize) {
        let raw = insn.raw;
        if insn.state.is_thumb() {
            (
                ((raw >> 8) & 0xF) as usize,
                ((raw >> 16) & 0xF) as usize,
                (raw & 0xF) as usize,
            )
        } else {
            (
                ((raw >> 12) & 0xF) as usize,
                ((raw >> 16) & 0xF) as usize,
                (raw & 0xF) as usize,
            )
        }
    }

    /// (Rd, Ra, Rm, Rn) for 4-register DSP multiplies (A32 / T32 layouts).
    fn dsp4_regs(&self, insn: &DecodedInsn) -> (usize, usize, usize, usize) {
        let raw = insn.raw;
        if insn.state.is_thumb() {
            (
                ((raw >> 8) & 0xF) as usize,  // Rd = hw2[11:8]
                ((raw >> 12) & 0xF) as usize, // Ra = hw2[15:12]
                (raw & 0xF) as usize,         // Rm = hw2[3:0]
                ((raw >> 16) & 0xF) as usize, // Rn = hw1[3:0]
            )
        } else {
            (
                ((raw >> 16) & 0xF) as usize, // Rd = bits[19:16]
                ((raw >> 12) & 0xF) as usize, // Ra = bits[15:12]
                ((raw >> 8) & 0xF) as usize,  // Rm = bits[11:8]
                (raw & 0xF) as usize,         // Rn = bits[3:0]
            )
        }
    }

    /// Signed-saturate a value to 32 bits, setting the Q flag on saturation.
    fn ssat32(&mut self, x: i64) -> u32 {
        if x > i32::MAX as i64 {
            self.cpu.cpsr.q = true;
            i32::MAX as u32
        } else if x < i32::MIN as i64 {
            self.cpu.cpsr.q = true;
            i32::MIN as u32
        } else {
            x as u32
        }
    }

    /// QADD / QSUB / QDADD / QDSUB.
    fn exec_a32_sat_addsub(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, rn, rm) = self.media_regs(insn);
        let n = self.reg(rn) as i32 as i64;
        let m = self.reg(rm) as i32 as i64;
        // Canonical kind: 0=QADD 1=QSUB 2=QDADD 3=QDSUB.
        let kind = if insn.state.is_thumb() {
            match (raw >> 4) & 0x3 {
                0 => 0,
                1 => 2,
                2 => 1,
                _ => 3,
            }
        } else {
            (raw >> 21) & 0x3
        };
        let result = match kind {
            0b00 => self.ssat32(m + n),
            0b01 => self.ssat32(m - n),
            0b10 => {
                let dbl = self.ssat32(2 * n) as i32 as i64;
                self.ssat32(m + dbl)
            }
            _ => {
                let dbl = self.ssat32(2 * n) as i32 as i64;
                self.ssat32(m - dbl)
            }
        };
        self.set_reg(rd, result)
    }

    /// SMUL/SMLA/SMULW/SMLAW/SMLAL <x><y> (halfword and word multiplies).
    fn exec_a32_hmul(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, ra, rm, rn) = self.dsp4_regs(insn);
        let rn_v = self.reg(rn);
        let rm_v = self.reg(rm);
        let half = |v: u32, top: bool| -> i64 {
            if top {
                (v >> 16) as u16 as i16 as i64
            } else {
                v as u16 as i16 as i64
            }
        };
        // Normalized kind: 0=SMLA 1=SMLAW 2=SMULW 3=SMLAL 4=SMUL.
        let (kind, n_top, m_top) = if insn.state.is_thumb() {
            let op1 = (raw >> 20) & 0x7; // hw1[6:4]
            let nt = (raw >> 5) & 1 != 0;
            let mt = (raw >> 4) & 1 != 0;
            if op1 == 0b001 {
                (if ra == 15 { 4 } else { 0 }, nt, mt) // SMUL / SMLA
            } else {
                (if ra == 15 { 2 } else { 1 }, false, mt) // SMULW / SMLAW
            }
        } else {
            let nt = (raw >> 5) & 1 != 0;
            let mt = (raw >> 6) & 1 != 0;
            match (raw >> 21) & 0x3 {
                0b00 => (0, nt, mt),
                0b01 => (if (raw >> 5) & 1 != 0 { 2 } else { 1 }, false, mt),
                0b10 => (3, nt, mt),
                _ => (4, nt, mt),
            }
        };
        match kind {
            0 => {
                // SMLA<x><y>: Rd = Rn.x * Rm.y + Ra (Q on signed overflow)
                let result = half(rn_v, n_top) * half(rm_v, m_top) + self.reg(ra) as i32 as i64;
                let r32 = result as i32;
                if result != r32 as i64 {
                    self.cpu.cpsr.q = true;
                }
                self.set_reg(rd, r32 as u32)
            }
            1 => {
                // SMLAW<y>: Rd = (Rn * Rm.y)[47:16] + Ra (Q on overflow)
                let prod = (rn_v as i32 as i64) * half(rm_v, m_top);
                let result = (prod >> 16) + self.reg(ra) as i32 as i64;
                let r32 = result as i32;
                if result != r32 as i64 {
                    self.cpu.cpsr.q = true;
                }
                self.set_reg(rd, r32 as u32)
            }
            2 => {
                // SMULW<y>: Rd = (Rn * Rm.y)[47:16]
                let prod = (rn_v as i32 as i64) * half(rm_v, m_top);
                self.set_reg(rd, (prod >> 16) as i32 as u32)
            }
            3 => {
                // SMLAL<x><y>: RdHi:RdLo += Rn.x * Rm.y (RdHi=rd, RdLo=ra)
                let acc = (((self.cpu.regs[rd] as u64) << 32) | self.cpu.regs[ra] as u64) as i64;
                let result = acc.wrapping_add(half(rn_v, n_top) * half(rm_v, m_top)) as u64;
                self.cpu.regs[ra] = result as u32;
                self.cpu.regs[rd] = (result >> 32) as u32;
                ExecResult::Continue
            }
            _ => {
                // SMUL<x><y>: Rd = Rn.x * Rm.y
                self.set_reg(rd, (half(rn_v, n_top) * half(rm_v, m_top)) as i32 as u32)
            }
        }
    }

    /// SMUAD / SMUSD / SMLAD / SMLSD.
    fn exec_a32_dual(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, ra, rm, rn) = self.dsp4_regs(insn);
        // X (swap Rm halves) and sub flags differ by encoding.
        let (swap, sub) = if insn.state.is_thumb() {
            ((raw >> 4) & 1 != 0, (raw >> 20) & 0x7 == 0b100)
        } else {
            ((raw >> 5) & 1 != 0, (raw >> 6) & 1 != 0)
        };
        let rn_v = self.reg(rn);
        let mut rm_v = self.reg(rm);
        if swap {
            rm_v = rm_v.rotate_right(16);
        }
        let p1 = (rn_v as u16 as i16 as i64) * (rm_v as u16 as i16 as i64);
        let p2 = ((rn_v >> 16) as u16 as i16 as i64) * ((rm_v >> 16) as u16 as i16 as i64);
        let mut result = if sub { p1 - p2 } else { p1 + p2 };
        if ra != 15 {
            result += self.reg(ra) as i32 as i64;
        }
        let r32 = result as i32;
        if result != r32 as i64 {
            self.cpu.cpsr.q = true;
        }
        self.set_reg(rd, r32 as u32)
    }

    /// SMLALD / SMLSLD.
    fn exec_a32_smlald(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (dhi, dlo, rm, rn, swap, sub) = if insn.state.is_thumb() {
            (
                ((raw >> 8) & 0xF) as usize,  // RdHi = hw2[11:8]
                ((raw >> 12) & 0xF) as usize, // RdLo = hw2[15:12]
                (raw & 0xF) as usize,         // Rm = hw2[3:0]
                ((raw >> 16) & 0xF) as usize, // Rn = hw1[3:0]
                (raw >> 4) & 1 != 0,
                (raw >> 20) & 0x7 == 0b101, // op1==101 -> SMLSLD
            )
        } else {
            (
                ((raw >> 16) & 0xF) as usize,
                ((raw >> 12) & 0xF) as usize,
                ((raw >> 8) & 0xF) as usize,
                (raw & 0xF) as usize,
                (raw >> 5) & 1 != 0,
                (raw >> 6) & 1 != 0,
            )
        };
        let rn_v = self.reg(rn);
        let mut rm_v = self.reg(rm);
        if swap {
            rm_v = rm_v.rotate_right(16);
        }
        let p1 = (rn_v as u16 as i16 as i64) * (rm_v as u16 as i16 as i64);
        let p2 = ((rn_v >> 16) as u16 as i16 as i64) * ((rm_v >> 16) as u16 as i16 as i64);
        let prod = if sub { p1 - p2 } else { p1 + p2 };
        let acc = (((self.cpu.regs[dhi] as u64) << 32) | self.cpu.regs[dlo] as u64) as i64;
        let result = acc.wrapping_add(prod) as u64;
        self.cpu.regs[dlo] = result as u32;
        self.cpu.regs[dhi] = (result >> 32) as u32;
        ExecResult::Continue
    }

    /// SMMUL / SMMLA / SMMLS (signed most-significant-word multiply).
    fn exec_a32_smmul(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, ra, rm, rn) = self.dsp4_regs(insn);
        let (round, sub) = if insn.state.is_thumb() {
            ((raw >> 4) & 1 != 0, (raw >> 20) & 0x7 == 0b110)
        } else {
            ((raw >> 5) & 1 != 0, (raw >> 6) & 1 != 0)
        };
        let prod = (self.reg(rn) as i32 as i64) * (self.reg(rm) as i32 as i64);
        let acc = if ra == 15 {
            0i64
        } else {
            (self.reg(ra) as i32 as i64) << 32
        };
        let mut result = if sub { acc - prod } else { acc + prod };
        if round {
            result += 0x8000_0000; // rounding
        }
        self.set_reg(rd, (result >> 32) as u32)
    }

    /// USAD8 / USADA8 (sum of absolute differences).
    fn exec_a32_usad(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (rd, ra, rm, rn) = self.dsp4_regs(insn);
        let n = self.reg(rn);
        let m = self.reg(rm);
        let mut sum: u32 = 0;
        for i in 0..4 {
            let a = ((n >> (i * 8)) & 0xFF) as i32;
            let b = ((m >> (i * 8)) & 0xFF) as i32;
            sum = sum.wrapping_add((a - b).unsigned_abs());
        }
        if ra != 15 {
            sum = sum.wrapping_add(self.reg(ra));
        }
        self.set_reg(rd, sum)
    }

    /// PKHBT / PKHTB (pack halfword).
    fn exec_a32_pkh(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, rn, rm) = self.media_regs(insn);
        let (tbform, imm5) = if insn.state.is_thumb() {
            (
                (raw >> 5) & 1 != 0,
                (((raw >> 12) & 0x7) << 2) | ((raw >> 6) & 0x3),
            )
        } else {
            ((raw >> 6) & 1 != 0, (raw >> 7) & 0x1F)
        };
        let n = self.reg(rn);
        let m = self.reg(rm);
        let result = if tbform {
            // PKHTB: top from Rn, bottom from (Rm ASR imm5; imm5==0 => 32)
            let op2 = if imm5 == 0 {
                ((m as i32) >> 31) as u32
            } else {
                ((m as i32) >> imm5) as u32
            };
            (n & 0xFFFF_0000) | (op2 & 0xFFFF)
        } else {
            // PKHBT: bottom from Rn, top from (Rm LSL imm5)
            let op2 = m.wrapping_shl(imm5);
            (op2 & 0xFFFF_0000) | (n & 0xFFFF)
        };
        self.set_reg(rd, result)
    }

    /// (U|S)XT(A)(B|H|B16) sign/zero extend, with optional add and rotate.
    fn exec_a32_extend(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, rn, rm) = self.media_regs(insn);
        // size: 00=B16, 10=B, 11=H ; unsigned ; rotation.
        let (unsigned, size, rotation) = if insn.state.is_thumb() {
            let ty = (raw >> 20) & 0x7; // hw1[6:4]: 0SXTH 1UXTH 2SXTB16 3UXTB16 4SXTB 5UXTB
            let size = match ty >> 1 {
                0 => 0b11, // H
                1 => 0b00, // B16
                _ => 0b10, // B
            };
            (ty & 1 != 0, size, ((raw >> 4) & 0x3) * 8)
        } else {
            (
                (raw >> 22) & 1 != 0,
                (raw >> 20) & 0x3,
                ((raw >> 10) & 0x3) * 8,
            )
        };
        let rotated = self.reg(rm).rotate_right(rotation);
        let add = rn != 15;
        let n = self.reg(rn);
        let extb = |b: u32, u: bool| -> u32 {
            if u {
                b & 0xFF
            } else {
                (b & 0xFF) as u8 as i8 as i32 as u32
            }
        };
        let result = match size {
            0b10 => {
                let ext = extb(rotated, unsigned);
                if add { n.wrapping_add(ext) } else { ext }
            }
            0b11 => {
                let h = rotated & 0xFFFF;
                let ext = if unsigned {
                    h
                } else {
                    h as u16 as i16 as i32 as u32
                };
                if add { n.wrapping_add(ext) } else { ext }
            }
            _ => {
                let lo = extb(rotated, unsigned) & 0xFFFF;
                let hi = extb(rotated >> 16, unsigned) & 0xFFFF;
                if add {
                    let l = (n & 0xFFFF).wrapping_add(lo) & 0xFFFF;
                    let h = ((n >> 16) & 0xFFFF).wrapping_add(hi) & 0xFFFF;
                    l | (h << 16)
                } else {
                    lo | (hi << 16)
                }
            }
        };
        self.set_reg(rd, result)
    }

    /// SSAT16 / USAT16 (parallel halfword saturate).
    fn exec_a32_sat16(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, rn, sat, unsigned) = if insn.state.is_thumb() {
            (
                ((raw >> 8) & 0xF) as usize,
                ((raw >> 16) & 0xF) as usize,
                raw & 0xF,
                (raw >> 23) & 1 != 0,
            )
        } else {
            (
                ((raw >> 12) & 0xF) as usize,
                (raw & 0xF) as usize,
                (raw >> 16) & 0xF,
                (raw >> 22) & 1 != 0,
            )
        };
        let n = self.reg(rn);
        let mut out: u32 = 0;
        for i in 0..2u32 {
            let h = ((n >> (i * 16)) & 0xFFFF) as u16 as i16 as i32;
            let clamped = if unsigned {
                let max = ((1u32 << sat) - 1) as i32;
                if h < 0 {
                    self.cpu.cpsr.q = true;
                    0
                } else if h > max {
                    self.cpu.cpsr.q = true;
                    max
                } else {
                    h
                }
            } else {
                let bits = sat + 1;
                let max = (1i32 << (bits - 1)) - 1;
                let min = -(1i32 << (bits - 1));
                if h > max {
                    self.cpu.cpsr.q = true;
                    max
                } else if h < min {
                    self.cpu.cpsr.q = true;
                    min
                } else {
                    h
                }
            };
            out |= ((clamped as u32) & 0xFFFF) << (i * 16);
        }
        self.set_reg(rd, out)
    }

    /// SEL (select bytes by GE flags).
    fn exec_a32_sel(&mut self, insn: &DecodedInsn) -> ExecResult {
        let (rd, rn, rm) = self.media_regs(insn);
        let n = self.reg(rn);
        let m = self.reg(rm);
        let ge = self.cpu.cpsr.ge;
        let mut result: u32 = 0;
        for i in 0..4u32 {
            let byte = if (ge >> i) & 1 != 0 {
                (n >> (i * 8)) & 0xFF
            } else {
                (m >> (i * 8)) & 0xFF
            };
            result |= byte << (i * 8);
        }
        self.set_reg(rd, result)
    }

    /// Signed/unsigned parallel add/sub (SADD8/QADD16/UHASX/...). Sets GE for
    /// the plain signed (S) and unsigned (U) prefixes.
    fn exec_a32_parallel(&mut self, insn: &DecodedInsn) -> ExecResult {
        let raw = insn.raw;
        let (rd, rn, rm) = self.media_regs(insn);
        // Normalize to the A32 codes: prefix 001=S 010=Q 011=SH 101=U 110=UQ
        // 111=UH ; op2 000=add16 001=asx 010=sax 011=sub16 100=add8 111=sub8.
        let (prefix, op2) = if insn.state.is_thumb() {
            let prefix = match (raw >> 4) & 0x7 {
                // hw2[6:4]: 0=S 1=Q 2=SH 4=U 5=UQ 6=UH
                0 => 0b001,
                1 => 0b010,
                2 => 0b011,
                4 => 0b101,
                5 => 0b110,
                _ => 0b111,
            };
            let op2 = match (raw >> 20) & 0x7 {
                // hw1[6:4]: 0=add8 1=add16 2=asx 4=sub8 5=sub16 6=sax
                0 => 0b100,
                1 => 0b000,
                2 => 0b001,
                4 => 0b111,
                5 => 0b011,
                _ => 0b010,
            };
            (prefix, op2)
        } else {
            ((raw >> 20) & 0x7, (raw >> 5) & 0x7)
        };
        let n = self.reg(rn);
        let m = self.reg(rm);

        let eight = op2 == 0b100 || op2 == 0b111;
        let width: u32 = if eight { 8 } else { 16 };
        let lane = |v: u32, idx: u32, w: u32| (v >> (idx * w)) & ((1u32 << w) - 1);

        // (a, b, sub) per lane.
        let mut lanes: [(u32, u32, bool); 4] = [(0, 0, false); 4];
        let nlanes: usize = match op2 {
            0b000 => {
                for i in 0..2 {
                    lanes[i] = (lane(n, i as u32, 16), lane(m, i as u32, 16), false);
                }
                2
            }
            0b011 => {
                for i in 0..2 {
                    lanes[i] = (lane(n, i as u32, 16), lane(m, i as u32, 16), true);
                }
                2
            }
            0b001 => {
                // ASX: lane0 = n.lo - m.hi ; lane1 = n.hi + m.lo
                lanes[0] = (lane(n, 0, 16), lane(m, 1, 16), true);
                lanes[1] = (lane(n, 1, 16), lane(m, 0, 16), false);
                2
            }
            0b010 => {
                // SAX: lane0 = n.lo + m.hi ; lane1 = n.hi - m.lo
                lanes[0] = (lane(n, 0, 16), lane(m, 1, 16), false);
                lanes[1] = (lane(n, 1, 16), lane(m, 0, 16), true);
                2
            }
            0b100 => {
                for i in 0..4 {
                    lanes[i] = (lane(n, i as u32, 8), lane(m, i as u32, 8), false);
                }
                4
            }
            0b111 => {
                for i in 0..4 {
                    lanes[i] = (lane(n, i as u32, 8), lane(m, i as u32, 8), true);
                }
                4
            }
            _ => return ExecResult::Undefined,
        };

        let sign_ext = |v: u32, w: u32| -> i64 {
            let sh = 64 - w;
            ((v as i64) << sh) >> sh
        };
        let maskw: u32 = if width == 32 {
            u32::MAX
        } else {
            (1u32 << width) - 1
        };
        let smax = (1i64 << (width - 1)) - 1;
        let smin = -(1i64 << (width - 1));
        let umax = (1i64 << width) - 1;

        let mut result: u32 = 0;
        let mut ge: u8 = 0;
        let mut set_ge = false;
        for (idx, &(a, b, sub)) in lanes.iter().take(nlanes).enumerate() {
            let avs = sign_ext(a, width);
            let bvs = sign_ext(b, width);
            let avu = a as i64;
            let bvu = b as i64;
            let (val, ge_opt): (u32, Option<bool>) = match prefix {
                0b001 => {
                    let r = if sub { avs - bvs } else { avs + bvs };
                    (r as u32, Some(r >= 0))
                }
                0b101 => {
                    if sub {
                        ((avu - bvu) as u32, Some(avu >= bvu))
                    } else {
                        let r = avu + bvu;
                        (r as u32, Some(r >= (1i64 << width)))
                    }
                }
                0b010 => {
                    let r = if sub { avs - bvs } else { avs + bvs };
                    (r.clamp(smin, smax) as u32, None)
                }
                0b110 => {
                    let r = if sub { avu - bvu } else { avu + bvu };
                    (r.clamp(0, umax) as u32, None)
                }
                0b011 => {
                    let r = if sub { avs - bvs } else { avs + bvs };
                    ((r >> 1) as u32, None)
                }
                0b111 => {
                    let r = if sub { avu - bvu } else { avu + bvu };
                    ((r >> 1) as u32, None)
                }
                _ => return ExecResult::Undefined,
            };
            result |= (val & maskw) << (idx as u32 * width);
            if let Some(g) = ge_opt {
                set_ge = true;
                if g {
                    if eight {
                        ge |= 1 << idx;
                    } else {
                        ge |= 0b11 << (idx * 2);
                    }
                }
            }
        }

        if set_ge {
            self.cpu.cpsr.ge = ge;
        }
        self.set_reg(rd, result)
    }

    // =========================================================================
    // Operand Decoding Helpers
    // =========================================================================

    /// Collect up to `max` GPR numbers from the decoded operand list, in order.
    fn thumb_reg_ops(insn: &DecodedInsn, max: usize) -> ([usize; 4], usize) {
        use crate::arm::decoder::Operand;
        let mut regs = [0usize; 4];
        let mut cnt = 0;
        for o in &insn.operands {
            if let Operand::Reg(r) = o {
                if cnt < max && cnt < 4 {
                    regs[cnt] = r.num as usize;
                    cnt += 1;
                }
            }
        }
        (regs, cnt)
    }

    /// (Rd, Rm) for two-register ops: from operands in Thumb, from raw in A32.
    fn dm_ops(&self, insn: &DecodedInsn) -> (usize, usize) {
        if insn.state.is_thumb() {
            let (r, _) = Self::thumb_reg_ops(insn, 2);
            (r[0], r[1])
        } else {
            (((insn.raw >> 12) & 0xF) as usize, (insn.raw & 0xF) as usize)
        }
    }

    /// Carry-out of a Thumb data-processing immediate (ThumbExpandImm_C). The
    /// rotated forms produce carry = result[31]; plain forms leave C unchanged.
    fn thumb_imm_carry(&self, insn: &DecodedInsn, value: u32) -> bool {
        if insn.state == crate::arm::ExecutionState::Thumb2 {
            let raw = insn.raw;
            let imm12 = (((raw >> 26) & 1) << 11) | (((raw >> 12) & 0x7) << 8) | (raw & 0xFF);
            if (imm12 >> 8) >= 4 {
                return (value >> 31) & 1 != 0;
            }
        }
        self.cpu.cpsr.c
    }

    /// Thumb (T16/T32) data-processing operand decode using the decoded operands.
    fn decode_dp_operands_thumb(&mut self, insn: &DecodedInsn) -> (usize, usize, u32) {
        use crate::arm::decoder::Operand;
        let (operand2, carry) = match insn.operands.last() {
            Some(Operand::Imm(imm)) => {
                let v = imm.value as u32;
                (v, self.thumb_imm_carry(insn, v))
            }
            Some(Operand::Reg(r)) => (self.reg(r.num as usize), self.cpu.cpsr.c),
            Some(Operand::ShiftedReg(sr)) => shift_c(
                self.reg(sr.reg.num as usize),
                sr.shift_type,
                sr.amount as u32,
                self.cpu.cpsr.c,
            ),
            _ => (0, self.cpu.cpsr.c),
        };
        self.cpu.carry_out = carry;

        // Leading register operands (those before operand2).
        let nlead = insn.operands.len().saturating_sub(1);
        let mut lead = [0usize; 2];
        let mut cnt = 0;
        for o in &insn.operands[..nlead] {
            if let Operand::Reg(r) = o {
                if cnt < 2 {
                    lead[cnt] = r.num as usize;
                    cnt += 1;
                }
            }
        }
        let is_test = matches!(
            insn.mnemonic,
            Mnemonic::CMP | Mnemonic::CMN | Mnemonic::TST | Mnemonic::TEQ
        );
        let (d, n) = match cnt {
            2 => (lead[0], lead[1]),
            1 => {
                if is_test {
                    (15, lead[0])
                } else {
                    (lead[0], 0)
                }
            }
            _ => (0, 0),
        };
        (d, n, operand2)
    }

    /// Decode data processing operands: (Rd, Rn, operand2)
    fn decode_dp_operands(&mut self, insn: &DecodedInsn) -> (usize, usize, u32) {
        if insn.state.is_thumb() {
            return self.decode_dp_operands_thumb(insn);
        }
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let n = ((insn.raw >> 16) & 0xF) as usize;

        let operand2 = if (insn.raw >> 25) & 1 != 0 {
            let imm12 = insn.raw & 0xFFF;
            let (value, carry) = expand_imm_c(imm12, self.cpu.cpsr.c);
            self.cpu.carry_out = carry;
            value
        } else {
            let m = (insn.raw & 0xF) as usize;
            let mut shift_type = ShiftType::from_bits(((insn.raw >> 5) & 3) as u8);

            let shift_amount = if (insn.raw >> 4) & 1 != 0 {
                // Register-controlled shift: amount is Rs[7:0]; RRX is not
                // encodable in this form.
                let s = ((insn.raw >> 8) & 0xF) as usize;
                self.reg(s) & 0xFF
            } else {
                let imm5 = ((insn.raw >> 7) & 0x1F) as u32;
                match shift_type {
                    ShiftType::LSR | ShiftType::ASR if imm5 == 0 => 32,
                    // type==ROR with imm5==0 encodes RRX (rotate right with
                    // extend through carry), not ROR #1.
                    ShiftType::ROR if imm5 == 0 => {
                        shift_type = ShiftType::RRX;
                        1
                    }
                    _ => imm5,
                }
            };

            let (result, carry) = shift_c(self.reg(m), shift_type, shift_amount, self.cpu.cpsr.c);
            self.cpu.carry_out = carry;
            result
        };

        (d, n, operand2)
    }

    /// Decode shift instruction operands: (Rd, Rm, shift_amount)
    fn decode_shift_operands(&self, insn: &DecodedInsn) -> (usize, usize, u32) {
        if insn.state.is_thumb() {
            use crate::arm::decoder::Operand;
            let (regs, _) = Self::thumb_reg_ops(insn, 2);
            let d = regs[0];
            let m = regs[1];
            let amount = match insn.operands.last() {
                Some(Operand::Imm(imm)) => imm.value as u32,
                // Register-controlled shift (e.g. T16 LSLS Rdn, Rm).
                Some(Operand::Reg(r)) => self.reg(r.num as usize) & 0xFF,
                _ => 0,
            };
            return (d, m, amount);
        }
        let d = ((insn.raw >> 12) & 0xF) as usize;
        let m = (insn.raw & 0xF) as usize;

        let shift_amount = if (insn.raw >> 4) & 1 != 0 {
            let s = ((insn.raw >> 8) & 0xF) as usize;
            self.reg(s) & 0xFF
        } else {
            let imm5 = ((insn.raw >> 7) & 0x1F) as u32;
            if imm5 == 0 { 32 } else { imm5 }
        };

        (d, m, shift_amount)
    }

    /// Decode multiply operands: (Rd, Rn, Rm)
    fn decode_mul_operands(&self, insn: &DecodedInsn) -> (usize, usize, usize) {
        if insn.state.is_thumb() {
            let (r, _) = Self::thumb_reg_ops(insn, 3);
            return (r[0], r[1], r[2]);
        }
        let d = ((insn.raw >> 16) & 0xF) as usize;
        let n = (insn.raw & 0xF) as usize;
        let m = ((insn.raw >> 8) & 0xF) as usize;
        (d, n, m)
    }

    /// Decode MLA operands: (Rd, Rn, Rm, Ra)
    fn decode_mla_operands(&self, insn: &DecodedInsn) -> (usize, usize, usize, usize) {
        if insn.state.is_thumb() {
            let (r, _) = Self::thumb_reg_ops(insn, 4);
            return (r[0], r[1], r[2], r[3]);
        }
        let d = ((insn.raw >> 16) & 0xF) as usize;
        let a = ((insn.raw >> 12) & 0xF) as usize;
        let m = ((insn.raw >> 8) & 0xF) as usize;
        let n = (insn.raw & 0xF) as usize;
        (d, n, m, a)
    }

    /// Decode long multiply operands: (RdLo, RdHi, Rn, Rm)
    fn decode_mull_operands(&self, insn: &DecodedInsn) -> (usize, usize, usize, usize) {
        if insn.state.is_thumb() {
            let (r, _) = Self::thumb_reg_ops(insn, 4);
            return (r[0], r[1], r[2], r[3]);
        }
        let dhi = ((insn.raw >> 16) & 0xF) as usize;
        let dlo = ((insn.raw >> 12) & 0xF) as usize;
        let m = ((insn.raw >> 8) & 0xF) as usize;
        let n = (insn.raw & 0xF) as usize;
        (dlo, dhi, n, m)
    }

    /// Decode branch target from instruction.
    fn decode_branch_target(&self, insn: &DecodedInsn) -> Option<u32> {
        // Thumb B/BL/BLX/BCC: the (already scaled and sign-extended) byte
        // offset is in the Label operand and the branch base is PC+4.
        if insn.state.is_thumb() {
            let off = insn.operands.iter().find_map(|o| match o {
                crate::arm::decoder::Operand::Label(l) => Some(*l),
                _ => None,
            })?;
            let base = self.cpu.regs[15].wrapping_add(4);
            return Some((base as i64).wrapping_add(off) as u32);
        }
        // ARM B/BL: 24-bit signed offset scaled by 4, relative to PC+8.
        let imm24 = insn.raw & 0x00FFFFFF;
        let imm26 = imm24 << 2;
        let imm32 = if (imm26 & 0x02000000) != 0 {
            imm26 | 0xFC000000
        } else {
            imm26
        };
        Some(self.cpu.get_pc().wrapping_add(imm32))
    }

    /// Decode register operand at given position.
    fn decode_reg_operand(&self, insn: &DecodedInsn, pos: usize) -> Option<usize> {
        if pos < insn.operands.len() {
            match &insn.operands[pos] {
                crate::arm::decoder::Operand::Reg(reg) => Some(reg.num as usize),
                _ => None,
            }
        } else {
            Some((insn.raw & 0xF) as usize)
        }
    }

    /// Decode load/store operands for word/byte: (Rt, address, writeback)
    /// Compute (Rt, address, writeback) from the decoded operands (Thumb path):
    /// the first Reg operand is Rt and the Mem operand gives base/offset/mode.
    fn decode_mem_thumb(&self, insn: &DecodedInsn) -> Option<(usize, u32, Option<(usize, u32)>)> {
        use crate::arm::decoder::{AddressingMode, MemOffset, Operand};
        let t = insn.operands.iter().find_map(|o| match o {
            Operand::Reg(r) => Some(crate::arm::execution::aarch32_reg_index(r.num as usize)),
            _ => None,
        })?;
        // Thumb LDR (literal): `LDR Rt, [PC, #imm]` is decoded as a Label
        // operand holding the byte offset. The base is Align(PC+4, 4).
        if let Some(off) = insn.operands.iter().find_map(|o| match o {
            Operand::Label(l) => Some(*l),
            _ => None,
        }) {
            let base = self.cpu.regs[15].wrapping_add(4) & !0x3;
            let address = base.wrapping_add(off as u32);
            return Some((t, address, None));
        }
        let mem = insn.operands.iter().find_map(|o| match o {
            Operand::Mem(m) => Some(m),
            _ => None,
        })?;
        let n = mem.base.num as usize;
        let base = self.reg(n);
        let offset: i64 = match &mem.offset {
            MemOffset::None => 0,
            MemOffset::Imm(i) => *i,
            MemOffset::Reg(r) => self.reg(r.num as usize) as i64,
            MemOffset::ShiftedReg(sr) => {
                shift_c(
                    self.reg(sr.reg.num as usize),
                    sr.shift_type,
                    sr.amount as u32,
                    false,
                )
                .0 as i64
            }
            MemOffset::ExtendedReg(_) => return None,
        };
        let offset_addr = (base as i64).wrapping_add(offset) as u32;
        let (address, wb_addr) = match mem.mode {
            AddressingMode::Offset => (offset_addr, None),
            AddressingMode::PreIndex => (offset_addr, Some(offset_addr)),
            AddressingMode::PostIndex => (base, Some(offset_addr)),
        };
        Some((t, address, wb_addr.filter(|_| n != 15).map(|a| (n, a))))
    }

    fn decode_ldst_operands(
        &self,
        insn: &DecodedInsn,
    ) -> Option<(usize, u32, Option<(usize, u32)>)> {
        if insn.state.is_thumb() {
            return self.decode_mem_thumb(insn);
        }
        let p = (insn.raw >> 24) & 1;
        let u = (insn.raw >> 23) & 1;
        let w = (insn.raw >> 21) & 1;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let t = ((insn.raw >> 12) & 0xF) as usize;

        let base = self.reg(n);

        let offset = if (insn.raw >> 25) & 1 != 0 {
            let m = (insn.raw & 0xF) as usize;
            let shift_type = ShiftType::from_bits(((insn.raw >> 5) & 3) as u8);
            let imm5 = ((insn.raw >> 7) & 0x1F) as u32;
            let shift_amount = match shift_type {
                ShiftType::LSR | ShiftType::ASR if imm5 == 0 => 32,
                _ => imm5,
            };
            shift_c(self.reg(m), shift_type, shift_amount, false).0
        } else {
            insn.raw & 0xFFF
        };

        let is_add = u != 0;
        let is_index = p != 0;
        let is_wback = p == 0 || w != 0;

        let offset_addr = if is_add {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };

        let address = if is_index { offset_addr } else { base };
        let writeback = if is_wback && n != 15 {
            Some((n, offset_addr))
        } else {
            None
        };

        Some((t, address, writeback))
    }

    /// Decode load/store operands for halfword/signed: (Rt, address, writeback)
    /// Uses different encoding: bits[11:8] and bits[3:0] for immediate
    fn decode_ldst_halfword_operands(
        &self,
        insn: &DecodedInsn,
    ) -> Option<(usize, u32, Option<(usize, u32)>)> {
        if insn.state.is_thumb() {
            return self.decode_mem_thumb(insn);
        }
        let p = (insn.raw >> 24) & 1;
        let u = (insn.raw >> 23) & 1;
        let i = (insn.raw >> 22) & 1; // Immediate vs register
        let w = (insn.raw >> 21) & 1;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let t = ((insn.raw >> 12) & 0xF) as usize;

        let base = self.reg(n);

        let offset = if i != 0 {
            // Immediate: bits[11:8] and bits[3:0]
            let imm4h = (insn.raw >> 8) & 0xF;
            let imm4l = insn.raw & 0xF;
            (imm4h << 4) | imm4l
        } else {
            // Register
            let m = (insn.raw & 0xF) as usize;
            self.reg(m)
        };

        let is_add = u != 0;
        let is_index = p != 0;
        let is_wback = p == 0 || w != 0;

        let offset_addr = if is_add {
            base.wrapping_add(offset)
        } else {
            base.wrapping_sub(offset)
        };

        let address = if is_index { offset_addr } else { base };
        let writeback = if is_wback && n != 15 {
            Some((n, offset_addr))
        } else {
            None
        };

        Some((t, address, writeback))
    }

    /// Decode load/store multiple operands: (Rn, reglist, wback)
    fn decode_ldstm_operands(&self, insn: &DecodedInsn) -> Option<(usize, u16, bool)> {
        if insn.state.is_thumb() {
            use crate::arm::decoder::Operand;
            let n = insn.operands.iter().find_map(|o| match o {
                Operand::Reg(r) => Some(r.num as usize),
                _ => None,
            })?;
            let reglist = insn.operands.iter().find_map(|o| match o {
                Operand::RegList(rl) => Some(rl.mask),
                _ => None,
            })?;
            // T16 LDM/STM always write back; T32 has an explicit W bit (bit21).
            let wback = if insn.state == crate::arm::ExecutionState::Thumb2 {
                (insn.raw >> 21) & 1 != 0
            } else {
                true
            };
            return Some((n, reglist, wback));
        }
        let w = (insn.raw >> 21) & 1;
        let n = ((insn.raw >> 16) & 0xF) as usize;
        let reglist = (insn.raw & 0xFFFF) as u16;
        Some((n, reglist, w != 0))
    }

    /// Decode register list for PUSH/POP.
    fn decode_reglist(&self, insn: &DecodedInsn) -> Option<u16> {
        // Prefer the decoded register-list operand: for 16-bit Thumb PUSH/POP
        // the raw word's high bits are opcode, not list bits (e.g. POP
        // {r2,r3,r4} = 0xbc1c, whose bit15 would wrongly read as PC).
        if let Some(mask) = insn.operands.iter().find_map(|o| match o {
            crate::arm::decoder::Operand::RegList(rl) => Some(rl.mask),
            _ => None,
        }) {
            return Some(mask as u16);
        }
        Some((insn.raw & 0xFFFF) as u16)
    }
}

// =============================================================================
// Full Execution Loop
// =============================================================================

/// Run the ARM emulator in a fetch-decode-execute loop.
///
/// Returns when:
/// - An exception is raised
/// - CPU is halted (WFI/WFE)
/// - max_instructions is reached
/// - A memory fault occurs
pub fn run_emulator<M: ArmMemory>(
    cpu: &mut Armv7Cpu,
    mem: &mut M,
    decoder: &crate::arm::decoder::Decoder,
    max_instructions: u64,
) -> Result<ExecResult, DecodeError> {
    let mut executor = Executor::new(cpu, mem);
    let mut instructions_executed = 0u64;

    while instructions_executed < max_instructions {
        // Fetch instruction
        let pc = executor.cpu.regs[15];
        let insn_size = if executor.cpu.cpsr.t { 2 } else { 4 };

        // Read instruction bytes
        let mut bytes = [0u8; 4];
        for i in 0..insn_size {
            match executor.mem.read_byte(pc.wrapping_add(i as u32)) {
                Ok(b) => bytes[i] = b,
                Err(e) => return Ok(ExecResult::MemoryFault(e)),
            }
        }

        // Decode instruction
        let insn = decoder.decode(&bytes[..insn_size as usize])?;

        // Execute instruction
        let result = executor.execute(&insn);
        instructions_executed += 1;

        match result {
            ExecResult::Continue => {
                // Advance PC
                executor.cpu.regs[15] = executor.cpu.regs[15].wrapping_add(insn.size as u32);
            }
            ExecResult::Branch(target) => {
                executor.cpu.regs[15] = target;
            }
            ExecResult::Halt
            | ExecResult::Exception(_)
            | ExecResult::Undefined
            | ExecResult::MemoryFault(_) => {
                return Ok(result);
            }
        }
    }

    Ok(ExecResult::Continue)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm::ExecutionState;
    use crate::arm::execution::FlatMemory;

    fn make_cpu() -> Armv7Cpu {
        Armv7Cpu::new()
    }

    fn make_mem() -> FlatMemory {
        FlatMemory::new(0x10000, 0)
    }

    fn make_insn(mnemonic: Mnemonic, raw: u32, sets_flags: bool) -> DecodedInsn {
        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Arm, raw, 4);
        if sets_flags {
            insn = insn.with_flags();
        }
        insn
    }

    #[test]
    fn test_add_immediate() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[1] = 100;

        let insn = make_insn(Mnemonic::ADD, 0xE2810032, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 150);
    }

    #[test]
    fn test_adds_sets_flags() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[1] = 0xFFFFFFFF;

        let insn = make_insn(Mnemonic::ADDS, 0xE2910001, true);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 0);
        assert!(cpu.cpsr.z);
        assert!(cpu.cpsr.c);
    }

    #[test]
    fn test_sub_immediate() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[1] = 100;

        let insn = make_insn(Mnemonic::SUB, 0xE241001E, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 70);
    }

    #[test]
    fn test_mov_immediate() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        let insn = make_insn(Mnemonic::MOV, 0xE3A000FF, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 0xFF);
    }

    #[test]
    fn test_cmp_sets_flags() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[0] = 50;

        let insn = make_insn(Mnemonic::CMP, 0xE3500032, true);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert!(cpu.cpsr.z);
        assert!(cpu.cpsr.c);
    }

    #[test]
    fn test_branch() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[15] = 0x1000;

        let insn = make_insn(Mnemonic::B, 0xEA000040, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        if let ExecResult::Branch(target) = result {
            assert_eq!(target, 0x1000 + 8 + 0x100);
        } else {
            panic!("Expected Branch result");
        }
    }

    #[test]
    fn test_ldr_str() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        mem.write_word(0x100, 0xDEADBEEF).unwrap();

        cpu.regs[1] = 0x100;

        let insn = make_insn(Mnemonic::LDR, 0xE5910000, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 0xDEADBEEF);
    }

    #[test]
    fn test_mul() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[1] = 7;
        cpu.regs[2] = 6;

        let insn = make_insn(Mnemonic::MUL, 0xE0000291, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 42);
    }

    #[test]
    fn test_condition_ne() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.cpsr.z = true;
        cpu.regs[0] = 0;

        let mut insn = make_insn(Mnemonic::MOV, 0x13A00001, false);
        insn.cond = Some(Condition::NE);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 0);
    }

    #[test]
    fn test_svc() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        let insn = make_insn(Mnemonic::SVC, 0xEF00007B, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&insn);

        if let ExecResult::Exception(ExceptionType::SupervisorCall(imm)) = result {
            assert_eq!(imm, 123);
        } else {
            panic!("Expected SupervisorCall exception");
        }
    }

    #[test]
    fn test_ldrex_strex() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        mem.write_word(0x100, 0x12345678).unwrap();
        cpu.regs[1] = 0x100;
        cpu.regs[3] = 0xDEADBEEF; // Set this before creating executor

        // LDREX R0, [R1] followed by STREX R2, R3, [R1]
        // Must use same executor to maintain exclusive monitor state
        let ldrex = make_insn(Mnemonic::LDXR, 0xE1910F9F, false);
        let strex = make_insn(Mnemonic::STXR, 0xE1812F93, false);

        let mut exec = Executor::new(&mut cpu, &mut mem);

        // Execute LDREX
        let result = exec.execute(&ldrex);
        assert!(matches!(result, ExecResult::Continue));

        // Execute STREX - should succeed because LDREX was just done
        let result = exec.execute(&strex);
        assert!(matches!(result, ExecResult::Continue));

        // Drop executor to check cpu/mem state
        drop(exec);

        assert_eq!(cpu.regs[0], 0x12345678); // LDREX loaded value
        assert_eq!(cpu.regs[2], 0); // STREX success
        assert_eq!(mem.read_word(0x100).unwrap(), 0xDEADBEEF); // Memory updated
    }

    #[test]
    fn test_strex_fails_without_ldrex() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        mem.write_word(0x100, 0x12345678).unwrap();
        cpu.regs[1] = 0x100;
        cpu.regs[3] = 0xDEADBEEF;

        // STREX without LDREX should fail
        let strex = make_insn(Mnemonic::STXR, 0xE1812F93, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&strex);
        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[2], 1); // Failure

        // Memory should be unchanged
        assert_eq!(mem.read_word(0x100).unwrap(), 0x12345678);
    }

    #[test]
    fn test_sdiv_udiv() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[1] = 100;
        cpu.regs[2] = 7;

        // SDIV R0, R1, R2
        let sdiv = make_insn(Mnemonic::SDIV, 0xE710F211, false);
        {
            let mut exec = Executor::new(&mut cpu, &mut mem);
            let result = exec.execute(&sdiv);
            assert!(matches!(result, ExecResult::Continue));
        }
        assert_eq!(cpu.regs[0], 14);

        // Test division by zero
        cpu.regs[2] = 0;
        {
            let mut exec = Executor::new(&mut cpu, &mut mem);
            let result = exec.execute(&sdiv);
            assert!(matches!(result, ExecResult::Continue));
        }
        assert_eq!(cpu.regs[0], 0);
    }

    #[test]
    fn test_exception_handling() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();
        cpu.regs[15] = 0x1000;

        let mut exec = Executor::new(&mut cpu, &mut mem);
        exec.take_exception(ExceptionType::SupervisorCall(0));

        // Should be in SVC mode
        assert_eq!(cpu.cpsr.mode, ProcessorMode::Supervisor as u8);
        // IRQ should be disabled
        assert!(cpu.cpsr.i);
        // Should be in ARM mode
        assert!(!cpu.cpsr.t);
        // PC should be at SVC vector
        assert_eq!(cpu.regs[15], 0x08);
    }

    #[test]
    fn test_bfc_bfi() {
        let mut cpu = make_cpu();
        let mut mem = make_mem();

        cpu.regs[0] = 0xFFFFFFFF;

        // BFC R0, #4, #8 - clear bits 4-11
        let bfc = make_insn(Mnemonic::BFC, 0xE7CB021F, false);
        let mut exec = Executor::new(&mut cpu, &mut mem);
        let result = exec.execute(&bfc);
        assert!(matches!(result, ExecResult::Continue));
        assert_eq!(cpu.regs[0], 0xFFFFF00F);
    }
}
