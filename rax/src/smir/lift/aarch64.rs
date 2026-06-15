//! AArch64 instruction lifter.
//!
//! This module lifts AArch64 machine code to SMIR using the existing ARM decoder.

use std::collections::HashSet;

use crate::arm::decoder::{
    AddressingMode, Condition as ArmCondition, DecodedInsn, ExtendType, FpRegSize, FpRegister,
    MemOffset, MemOperand, Mnemonic, Operand, Register, ShiftType,
};
use crate::smir::flags::FlagUpdate;
use crate::smir::ir::{
    CallTarget, CallingConv, FunctionAttrs, SmirBlock, SmirFunction, Terminator, TrapKind,
};
use crate::smir::lift::{ControlFlow, LiftContext, LiftError, LiftResult, MemoryReader};
use crate::smir::memory::MemoryError;
use crate::smir::ops::{OpKind, SmirOp};
use crate::smir::types::*;

const NZCV_N: i64 = 1_i64 << 31;
const NZCV_Z: i64 = 1_i64 << 30;
const NZCV_C: i64 = 1_i64 << 29;
const NZCV_V: i64 = 1_i64 << 28;
const NZCV_MASK: i64 = NZCV_N | NZCV_Z | NZCV_C | NZCV_V;
const FP_SYSREG_MASK: i64 = 0xffff_ffff;
const MTE_TAG_CLEAR_MASK: i64 = (!0x0f00_0000_0000_0000u64) as i64;
const SYSREG_NZCV: u16 = (3 << 14) | (3 << 11) | (4 << 7) | (2 << 3);
const SYSREG_FPCR: u16 = (3 << 14) | (3 << 11) | (4 << 7) | (4 << 3);
const SYSREG_FPSR: u16 = (3 << 14) | (3 << 11) | (4 << 7) | (4 << 3) | 1;

// ============================================================================
// AArch64 Lifter
// ============================================================================

#[derive(Clone, Copy)]
enum CondSelectFalseOp {
    Identity,
    Increment,
    Invert,
    Negate,
}

#[derive(Clone, Copy)]
enum RevKind {
    Full,
    Halfwords,
    Words,
}

#[derive(Clone, Copy)]
struct SysRegAccess {
    reg: ArmReg,
    mask: i64,
    read_width: OpWidth,
    write_width: OpWidth,
}

#[derive(Clone, Copy)]
enum BitfieldKind {
    Extract { sign_extend: bool },
    Insert,
    InsertLow,
    InsertZero { sign_extend: bool },
}

/// AArch64 instruction lifter
pub struct Aarch64Lifter {
    /// Whether to use strict mode (fail on unsupported instructions)
    strict: bool,
}

impl Aarch64Lifter {
    /// Create a new AArch64 lifter
    pub fn new() -> Self {
        Aarch64Lifter { strict: false }
    }

    /// Create a lifter in strict mode
    pub fn strict() -> Self {
        Aarch64Lifter { strict: true }
    }

    // ========================================================================
    // Register Conversion
    // ========================================================================

    /// Convert ARM register to VReg
    fn arm_reg(&self, reg: &Register) -> VReg {
        if reg.is_sp {
            VReg::Arch(ArchReg::Arm(ArmReg::Sp))
        } else if reg.num == 31 && !reg.is_sp {
            // XZR/WZR reads as zero
            VReg::Imm(0)
        } else {
            VReg::Arch(ArchReg::Arm(ArmReg::X(reg.num)))
        }
    }

    fn supported_sysreg(sysreg: u16) -> Option<SysRegAccess> {
        match sysreg {
            SYSREG_NZCV => Some(SysRegAccess {
                reg: ArmReg::Nzcv,
                mask: NZCV_MASK,
                read_width: OpWidth::W32,
                write_width: OpWidth::W32,
            }),
            SYSREG_FPCR => Some(SysRegAccess {
                reg: ArmReg::Fpcr,
                mask: FP_SYSREG_MASK,
                read_width: OpWidth::W64,
                write_width: OpWidth::W64,
            }),
            SYSREG_FPSR => Some(SysRegAccess {
                reg: ArmReg::Fpsr,
                mask: FP_SYSREG_MASK,
                read_width: OpWidth::W64,
                write_width: OpWidth::W64,
            }),
            _ => None,
        }
    }

    fn lift_add_sub_tags(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rd, rn, offset) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
            insn.operands.get(3),
        ) {
            (
                Some(Operand::Reg(rd)),
                Some(Operand::Reg(rn)),
                Some(Operand::Imm(offset)),
                Some(Operand::Imm(_tag_offset)),
            ) => (rd, rn, offset.effective_value()),
            _ => {
                return Err(LiftError::Internal(
                    "invalid ADDG/SUBG operands".to_string(),
                ));
            }
        };

        let adjusted = ctx.alloc_vreg();
        let src2 = SrcOperand::Imm(offset);
        let kind = if insn.mnemonic == Mnemonic::ADDG {
            OpKind::Add {
                dst: adjusted,
                src1: self.arm_reg(rn),
                src2,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            }
        } else {
            OpKind::Sub {
                dst: adjusted,
                src1: self.arm_reg(rn),
                src2,
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            }
        };
        Self::push_lifted_op(ops, pc, kind);

        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: self.dst_reg(rd, ctx),
                src1: adjusted,
                src2: SrcOperand::Imm64(MTE_TAG_CLEAR_MASK),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        Ok(())
    }

    /// Get the width for an ARM register operand
    fn reg_width(&self, reg: &Register) -> OpWidth {
        if reg.is_64bit {
            OpWidth::W64
        } else {
            OpWidth::W32
        }
    }

    /// Convert ARM condition to SMIR condition
    fn arm_cond(&self, cond: ArmCondition) -> Condition {
        match cond {
            ArmCondition::EQ => Condition::Eq,
            ArmCondition::NE => Condition::Ne,
            ArmCondition::CS => Condition::Uge,
            ArmCondition::CC => Condition::Ult,
            ArmCondition::MI => Condition::Negative,
            ArmCondition::PL => Condition::Positive,
            ArmCondition::VS => Condition::Overflow,
            ArmCondition::VC => Condition::NoOverflow,
            ArmCondition::HI => Condition::Ugt,
            ArmCondition::LS => Condition::Ule,
            ArmCondition::GE => Condition::Sge,
            ArmCondition::LT => Condition::Slt,
            ArmCondition::GT => Condition::Sgt,
            ArmCondition::LE => Condition::Sle,
            ArmCondition::AL | ArmCondition::NV => Condition::Always,
        }
    }

    /// Convert shift type
    fn arm_shift(&self, shift: ShiftType) -> ShiftOp {
        match shift {
            ShiftType::LSL => ShiftOp::Lsl,
            ShiftType::LSR => ShiftOp::Lsr,
            ShiftType::ASR => ShiftOp::Asr,
            ShiftType::ROR | ShiftType::RRX => ShiftOp::Ror,
        }
    }

    fn arm_extend(&self, extend: ExtendType) -> ExtendOp {
        match extend {
            ExtendType::UXTB => ExtendOp::Uxtb,
            ExtendType::UXTH => ExtendOp::Uxth,
            ExtendType::UXTW => ExtendOp::Uxtw,
            ExtendType::UXTX => ExtendOp::Uxtx,
            ExtendType::SXTB => ExtendOp::Sxtb,
            ExtendType::SXTH => ExtendOp::Sxth,
            ExtendType::SXTW => ExtendOp::Sxtw,
            ExtendType::SXTX => ExtendOp::Sxtx,
        }
    }

    // ========================================================================
    // Operand Helpers
    // ========================================================================

    /// Convert memory operand to SMIR address
    fn mem_to_addr(&self, mem: &MemOperand, ctx: &mut LiftContext) -> (Address, Vec<SmirOp>) {
        let mut pre_ops = Vec::new();
        let pc = ctx.guest_pc;

        let addr = match &mem.offset {
            MemOffset::None => Address::Direct(self.arm_reg(&mem.base)),
            MemOffset::Imm(off) => Address::BaseOffset {
                base: self.arm_reg(&mem.base),
                offset: *off,
                disp_size: DispSize::Auto,
            },
            MemOffset::Reg(idx) => {
                let tmp = ctx.alloc_vreg();
                let width = self.reg_width(&mem.base);
                pre_ops.push(SmirOp::new(
                    OpId(0),
                    pc,
                    OpKind::Add {
                        dst: tmp,
                        src1: self.arm_reg(&mem.base),
                        src2: SrcOperand::Reg(self.arm_reg(idx)),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));
                Address::Direct(tmp)
            }
            MemOffset::ShiftedReg(sr) => {
                let tmp_shift = ctx.alloc_vreg();
                let tmp_addr = ctx.alloc_vreg();
                let width = self.reg_width(&mem.base);

                pre_ops.push(SmirOp::new(
                    OpId(0),
                    pc,
                    OpKind::Shl {
                        dst: tmp_shift,
                        src: self.arm_reg(&sr.reg),
                        amount: SrcOperand::Imm(sr.amount as i64),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));

                pre_ops.push(SmirOp::new(
                    OpId(1),
                    pc,
                    OpKind::Add {
                        dst: tmp_addr,
                        src1: self.arm_reg(&mem.base),
                        src2: SrcOperand::Reg(tmp_shift),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));

                Address::Direct(tmp_addr)
            }
            MemOffset::ExtendedReg(er) => {
                let tmp_ext = ctx.alloc_vreg();
                let tmp_addr = ctx.alloc_vreg();
                let width = self.reg_width(&mem.base);

                let (from_width, signed) = match er.extend_type {
                    ExtendType::UXTB => (OpWidth::W8, false),
                    ExtendType::UXTH => (OpWidth::W16, false),
                    ExtendType::UXTW => (OpWidth::W32, false),
                    ExtendType::UXTX => (OpWidth::W64, false),
                    ExtendType::SXTB => (OpWidth::W8, true),
                    ExtendType::SXTH => (OpWidth::W16, true),
                    ExtendType::SXTW => (OpWidth::W32, true),
                    ExtendType::SXTX => (OpWidth::W64, true),
                };

                if signed {
                    pre_ops.push(SmirOp::new(
                        OpId(0),
                        pc,
                        OpKind::SignExtend {
                            dst: tmp_ext,
                            src: self.arm_reg(&er.reg),
                            from_width,
                            to_width: width,
                        },
                    ));
                } else {
                    pre_ops.push(SmirOp::new(
                        OpId(0),
                        pc,
                        OpKind::ZeroExtend {
                            dst: tmp_ext,
                            src: self.arm_reg(&er.reg),
                            from_width,
                            to_width: width,
                        },
                    ));
                }

                if er.shift > 0 {
                    let tmp_shift = ctx.alloc_vreg();
                    pre_ops.push(SmirOp::new(
                        OpId(1),
                        pc,
                        OpKind::Shl {
                            dst: tmp_shift,
                            src: tmp_ext,
                            amount: SrcOperand::Imm(er.shift as i64),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                    pre_ops.push(SmirOp::new(
                        OpId(2),
                        pc,
                        OpKind::Add {
                            dst: tmp_addr,
                            src1: self.arm_reg(&mem.base),
                            src2: SrcOperand::Reg(tmp_shift),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                } else {
                    pre_ops.push(SmirOp::new(
                        OpId(1),
                        pc,
                        OpKind::Add {
                            dst: tmp_addr,
                            src1: self.arm_reg(&mem.base),
                            src2: SrcOperand::Reg(tmp_ext),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                }

                Address::Direct(tmp_addr)
            }
        };

        (addr, pre_ops)
    }

    /// Handle pre/post-index writeback for memory operand
    fn handle_writeback(
        &self,
        mem: &MemOperand,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        _ctx: &mut LiftContext,
    ) {
        let offset = match &mem.offset {
            MemOffset::Imm(off) => *off,
            _ => return,
        };

        match mem.mode {
            AddressingMode::PreIndex | AddressingMode::PostIndex => {
                let width = self.reg_width(&mem.base);
                let base_reg = self.arm_reg(&mem.base);

                if matches!(base_reg, VReg::Imm(_)) {
                    return;
                }

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Add {
                        dst: base_reg,
                        src1: base_reg,
                        src2: SrcOperand::Imm(offset),
                        width,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            AddressingMode::Offset => {}
        }
    }

    fn indexed_access_addr(&self, mem: &MemOperand, addr: Address) -> Address {
        if matches!(
            mem.mode,
            AddressingMode::PreIndex | AddressingMode::PostIndex
        ) {
            Address::Direct(self.arm_reg(&mem.base))
        } else {
            addr
        }
    }

    fn materialize_src_operand(
        &self,
        src: SrcOperand,
        width: OpWidth,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> VReg {
        match src {
            SrcOperand::Reg(reg) => reg,
            SrcOperand::Imm(value) | SrcOperand::Imm64(value) => VReg::Imm(value),
            src => {
                let tmp = ctx.alloc_vreg();
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Mov {
                        dst: tmp,
                        src,
                        width,
                    },
                );
                tmp
            }
        }
    }

    /// Get destination VReg from operand, handling XZR/WZR writes
    fn dst_reg(&self, reg: &Register, ctx: &mut LiftContext) -> VReg {
        if reg.num == 31 && !reg.is_sp {
            ctx.alloc_vreg()
        } else if reg.is_sp {
            VReg::Arch(ArchReg::Arm(ArmReg::Sp))
        } else {
            VReg::Arch(ArchReg::Arm(ArmReg::X(reg.num)))
        }
    }

    fn push_lifted_op(ops: &mut Vec<SmirOp>, pc: u64, kind: OpKind) {
        ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));
    }

    fn push_mul_op(
        ops: &mut Vec<SmirOp>,
        pc: u64,
        dst_lo: VReg,
        dst_hi: Option<VReg>,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
        signed: bool,
    ) {
        let src2 = SrcOperand::Reg(src2);
        let flags = FlagUpdate::None;
        if signed {
            Self::push_lifted_op(
                ops,
                pc,
                OpKind::MulS {
                    dst_lo,
                    dst_hi,
                    src1,
                    src2,
                    width,
                    flags,
                },
            );
        } else {
            Self::push_lifted_op(
                ops,
                pc,
                OpKind::MulU {
                    dst_lo,
                    dst_hi,
                    src1,
                    src2,
                    width,
                    flags,
                },
            );
        }
    }

    fn widen_w_to_x(
        &self,
        ops: &mut Vec<SmirOp>,
        pc: u64,
        ctx: &mut LiftContext,
        src: &Register,
        signed: bool,
    ) -> VReg {
        let dst = ctx.alloc_vreg();
        let kind = if signed {
            OpKind::SignExtend {
                dst,
                src: self.arm_reg(src),
                from_width: OpWidth::W32,
                to_width: OpWidth::W64,
            }
        } else {
            OpKind::ZeroExtend {
                dst,
                src: self.arm_reg(src),
                from_width: OpWidth::W32,
                to_width: OpWidth::W64,
            }
        };
        Self::push_lifted_op(ops, pc, kind);
        dst
    }

    fn lift_crc32(
        &self,
        insn: &DecodedInsn,
        ops: &mut Vec<SmirOp>,
        pc: u64,
        ctx: &mut LiftContext,
    ) {
        if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            let dst = self.dst_reg(rd, ctx);
            let crc = ctx.alloc_vreg();
            Self::push_lifted_op(
                ops,
                pc,
                OpKind::Mov {
                    dst: crc,
                    src: SrcOperand::Reg(self.arm_reg(rn)),
                    width: OpWidth::W32,
                },
            );

            let data_bits = match insn.mnemonic {
                Mnemonic::CRC32B | Mnemonic::CRC32CB => 8,
                Mnemonic::CRC32H | Mnemonic::CRC32CH => 16,
                Mnemonic::CRC32W | Mnemonic::CRC32CW => 32,
                Mnemonic::CRC32X | Mnemonic::CRC32CX => 64,
                _ => return,
            };
            let poly = if matches!(
                insn.mnemonic,
                Mnemonic::CRC32CB | Mnemonic::CRC32CH | Mnemonic::CRC32CW | Mnemonic::CRC32CX
            ) {
                0x82f6_3b78
            } else {
                0xedb8_8320u32
            };
            let source_width = if data_bits == 64 {
                OpWidth::W64
            } else {
                OpWidth::W32
            };

            for byte in 0..(data_bits / 8) {
                let data_byte = ctx.alloc_vreg();
                if byte == 0 {
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Mov {
                            dst: data_byte,
                            src: SrcOperand::Reg(self.arm_reg(rm)),
                            width: source_width,
                        },
                    );
                } else {
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Shr {
                            dst: data_byte,
                            src: self.arm_reg(rm),
                            amount: SrcOperand::Imm((byte * 8) as i64),
                            width: source_width,
                            flags: FlagUpdate::None,
                        },
                    );
                }
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::And {
                        dst: data_byte,
                        src1: data_byte,
                        src2: SrcOperand::Imm(0xff),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                );
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Xor {
                        dst: crc,
                        src1: crc,
                        src2: SrcOperand::Reg(data_byte),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                );

                for _ in 0..8 {
                    let bit = ctx.alloc_vreg();
                    let mask = ctx.alloc_vreg();
                    let shifted = ctx.alloc_vreg();
                    let poly_bits = ctx.alloc_vreg();

                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::And {
                            dst: bit,
                            src1: crc,
                            src2: SrcOperand::Imm(1),
                            width: OpWidth::W32,
                            flags: FlagUpdate::None,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Sub {
                            dst: mask,
                            src1: VReg::Imm(0),
                            src2: SrcOperand::Reg(bit),
                            width: OpWidth::W32,
                            flags: FlagUpdate::None,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Shr {
                            dst: shifted,
                            src: crc,
                            amount: SrcOperand::Imm(1),
                            width: OpWidth::W32,
                            flags: FlagUpdate::None,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::And {
                            dst: poly_bits,
                            src1: mask,
                            src2: SrcOperand::Imm(poly as i64),
                            width: OpWidth::W32,
                            flags: FlagUpdate::None,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Xor {
                            dst: crc,
                            src1: shifted,
                            src2: SrcOperand::Reg(poly_bits),
                            width: OpWidth::W32,
                            flags: FlagUpdate::None,
                        },
                    );
                }
            }

            Self::push_lifted_op(
                ops,
                pc,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Reg(crc),
                    width: OpWidth::W32,
                },
            );
        }
    }

    // ========================================================================
    // FP Helpers
    // ========================================================================

    fn fp_vreg(reg: &FpRegister) -> VReg {
        VReg::Arch(ArchReg::Arm(ArmReg::V(reg.num)))
    }

    fn fp_precision(size: &FpRegSize) -> FpPrecision {
        match size {
            FpRegSize::S => FpPrecision::F32,
            FpRegSize::D => FpPrecision::F64,
            FpRegSize::H => FpPrecision::F16,
            _ => FpPrecision::F32,
        }
    }

    // ========================================================================
    // Instruction Lifting
    // ========================================================================

    /// Lift a single instruction to SMIR ops
    fn lift_insn_inner(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let mut ops = Vec::new();
        let mut control = ControlFlow::Fallthrough;

        macro_rules! push_op {
            ($kind:expr) => {{
                ops.push(SmirOp::new(OpId(ops.len() as u16), pc, $kind));
            }};
        }

        match insn.mnemonic {
            // =================================================================
            // Arithmetic
            // =================================================================
            Mnemonic::ADD | Mnemonic::ADDS => {
                let (dst, src1, src2, width) = self.parse_arith_operands(insn, ctx)?;
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::Add {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::SUB | Mnemonic::SUBS => {
                let (dst, src1, src2, width) = self.parse_arith_operands(insn, ctx)?;
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::Sub {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::ADDG | Mnemonic::SUBG => {
                self.lift_add_sub_tags(insn, pc, &mut ops, ctx)?;
            }

            Mnemonic::ADC | Mnemonic::ADCS => {
                let (dst, src1, src2, width) = self.parse_arith_operands(insn, ctx)?;
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::Adc {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::SBC | Mnemonic::SBCS => {
                let (dst, src1, src2, width) =
                    if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rm)), None) = (
                        insn.operands.get(0),
                        insn.operands.get(1),
                        insn.operands.get(2),
                    ) {
                        (
                            self.dst_reg(rd, ctx),
                            VReg::Imm(0),
                            SrcOperand::Reg(self.arm_reg(rm)),
                            self.reg_width(rd),
                        )
                    } else {
                        self.parse_arith_operands(insn, ctx)?
                    };
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::Sbb {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::NEG | Mnemonic::NEGS => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rm))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    let flags = if insn.sets_flags {
                        FlagUpdate::All
                    } else {
                        FlagUpdate::None
                    };
                    push_op!(OpKind::Neg {
                        dst,
                        src: self.arm_reg(rm),
                        width,
                        flags,
                    });
                }
            }

            Mnemonic::MUL | Mnemonic::MNEG => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    if insn.mnemonic == Mnemonic::MUL {
                        Self::push_mul_op(
                            &mut ops,
                            pc,
                            dst,
                            None,
                            self.arm_reg(rn),
                            self.arm_reg(rm),
                            width,
                            false,
                        );
                    } else {
                        push_op!(OpKind::MulSub {
                            dst,
                            acc: VReg::Imm(0),
                            src1: self.arm_reg(rn),
                            src2: self.arm_reg(rm),
                            width,
                        });
                    }
                }
            }

            Mnemonic::MADD | Mnemonic::MSUB => {
                if let (
                    Some(Operand::Reg(rd)),
                    Some(Operand::Reg(rn)),
                    Some(Operand::Reg(rm)),
                    Some(Operand::Reg(ra)),
                ) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                    insn.operands.get(3),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);

                    if insn.mnemonic == Mnemonic::MADD {
                        push_op!(OpKind::MulAdd {
                            dst,
                            acc: self.arm_reg(ra),
                            src1: self.arm_reg(rn),
                            src2: self.arm_reg(rm),
                            width,
                        });
                    } else {
                        push_op!(OpKind::MulSub {
                            dst,
                            acc: self.arm_reg(ra),
                            src1: self.arm_reg(rn),
                            src2: self.arm_reg(rm),
                            width,
                        });
                    }
                }
            }

            Mnemonic::SMADDL | Mnemonic::SMSUBL | Mnemonic::UMADDL | Mnemonic::UMSUBL => {
                if let (
                    Some(Operand::Reg(rd)),
                    Some(Operand::Reg(rn)),
                    Some(Operand::Reg(rm)),
                    Some(Operand::Reg(ra)),
                ) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                    insn.operands.get(3),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let src1 = self.widen_w_to_x(
                        &mut ops,
                        pc,
                        ctx,
                        rn,
                        matches!(insn.mnemonic, Mnemonic::SMADDL | Mnemonic::SMSUBL),
                    );
                    let src2 = self.widen_w_to_x(
                        &mut ops,
                        pc,
                        ctx,
                        rm,
                        matches!(insn.mnemonic, Mnemonic::SMADDL | Mnemonic::SMSUBL),
                    );
                    let product = ctx.alloc_vreg();
                    let signed = matches!(insn.mnemonic, Mnemonic::SMADDL | Mnemonic::SMSUBL);
                    Self::push_mul_op(
                        &mut ops,
                        pc,
                        product,
                        None,
                        src1,
                        src2,
                        OpWidth::W64,
                        signed,
                    );
                    if matches!(insn.mnemonic, Mnemonic::SMADDL | Mnemonic::UMADDL) {
                        push_op!(OpKind::Add {
                            dst,
                            src1: self.arm_reg(ra),
                            src2: SrcOperand::Reg(product),
                            width: OpWidth::W64,
                            flags: FlagUpdate::None,
                        });
                    } else {
                        push_op!(OpKind::Sub {
                            dst,
                            src1: self.arm_reg(ra),
                            src2: SrcOperand::Reg(product),
                            width: OpWidth::W64,
                            flags: FlagUpdate::None,
                        });
                    }
                }
            }

            Mnemonic::SMULL | Mnemonic::UMULL => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let signed = insn.mnemonic == Mnemonic::SMULL;
                    let src1 = self.widen_w_to_x(&mut ops, pc, ctx, rn, signed);
                    let src2 = self.widen_w_to_x(&mut ops, pc, ctx, rm, signed);
                    Self::push_mul_op(&mut ops, pc, dst, None, src1, src2, OpWidth::W64, signed);
                }
            }

            Mnemonic::SMULH | Mnemonic::UMULH => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let lo = ctx.alloc_vreg();
                    Self::push_mul_op(
                        &mut ops,
                        pc,
                        lo,
                        Some(dst),
                        self.arm_reg(rn),
                        self.arm_reg(rm),
                        OpWidth::W64,
                        insn.mnemonic == Mnemonic::SMULH,
                    );
                }
            }

            Mnemonic::UDIV => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    push_op!(OpKind::DivU {
                        quot: dst,
                        rem: None,
                        src1: self.arm_reg(rn),
                        src2: SrcOperand::Reg(self.arm_reg(rm)),
                        width,
                        flags: FlagUpdate::None,
                    });
                }
            }

            Mnemonic::SDIV => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    push_op!(OpKind::DivS {
                        quot: dst,
                        rem: None,
                        src1: self.arm_reg(rn),
                        src2: SrcOperand::Reg(self.arm_reg(rm)),
                        width,
                        flags: FlagUpdate::None,
                    });
                }
            }

            Mnemonic::CRC32B
            | Mnemonic::CRC32H
            | Mnemonic::CRC32W
            | Mnemonic::CRC32X
            | Mnemonic::CRC32CB
            | Mnemonic::CRC32CH
            | Mnemonic::CRC32CW
            | Mnemonic::CRC32CX => self.lift_crc32(insn, &mut ops, pc, ctx),

            // =================================================================
            // Logical
            // =================================================================
            Mnemonic::AND | Mnemonic::ANDS => {
                let (dst, src1, src2, width) = self.parse_arith_operands(insn, ctx)?;
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::And {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::ORR | Mnemonic::ORRS => {
                let (dst, src1, src2, width) = self.parse_arith_operands(insn, ctx)?;
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::Or {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::EOR | Mnemonic::EORS => {
                let (dst, src1, src2, width) = self.parse_arith_operands(insn, ctx)?;
                let flags = if insn.sets_flags {
                    FlagUpdate::All
                } else {
                    FlagUpdate::None
                };
                push_op!(OpKind::Xor {
                    dst,
                    src1,
                    src2,
                    width,
                    flags,
                });
            }

            Mnemonic::BIC | Mnemonic::BICS | Mnemonic::ORN | Mnemonic::EON => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let src1 = self.arm_reg(rn);
                    let width = self.reg_width(rd);
                    let flags = if insn.sets_flags {
                        FlagUpdate::All
                    } else {
                        FlagUpdate::None
                    };

                    let src2 = self.parse_operand2(insn, 2, ctx)?;

                    match insn.mnemonic {
                        Mnemonic::BIC | Mnemonic::BICS => {
                            push_op!(OpKind::AndNot {
                                dst,
                                src1,
                                src2,
                                width,
                                flags,
                            });
                        }
                        Mnemonic::ORN => {
                            let src2_reg =
                                self.materialize_src_operand(src2, width, pc, &mut ops, ctx);
                            let inverted = ctx.alloc_vreg();
                            push_op!(OpKind::Not {
                                dst: inverted,
                                src: src2_reg,
                                width,
                            });
                            push_op!(OpKind::Or {
                                dst,
                                src1,
                                src2: SrcOperand::Reg(inverted),
                                width,
                                flags,
                            });
                        }
                        Mnemonic::EON => {
                            let src2_reg =
                                self.materialize_src_operand(src2, width, pc, &mut ops, ctx);
                            let inverted = ctx.alloc_vreg();
                            push_op!(OpKind::Not {
                                dst: inverted,
                                src: src2_reg,
                                width,
                            });
                            push_op!(OpKind::Xor {
                                dst,
                                src1,
                                src2: SrcOperand::Reg(inverted),
                                width,
                                flags,
                            });
                        }
                        _ => unreachable!(),
                    }
                }
            }

            Mnemonic::MVN | Mnemonic::MVNS => {
                if let (Some(Operand::Reg(rd)), Some(src_op)) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);

                    let src = match src_op {
                        Operand::Reg(rm) => self.arm_reg(rm),
                        Operand::Imm(imm) => VReg::Imm(imm.effective_value()),
                        _ => return Err(LiftError::Internal("invalid MVN operand".to_string())),
                    };

                    push_op!(OpKind::Not { dst, src, width });
                }
            }

            Mnemonic::TST => {
                if let (Some(Operand::Reg(rn)), Some(op2)) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let tmp = ctx.alloc_vreg();
                    let src1 = self.arm_reg(rn);
                    let width = self.reg_width(rn);
                    let src2 = self.operand_to_src(op2, ctx)?;

                    push_op!(OpKind::And {
                        dst: tmp,
                        src1,
                        src2,
                        width,
                        flags: FlagUpdate::All,
                    });
                }
            }

            // =================================================================
            // Compare
            // =================================================================
            Mnemonic::CMP => {
                if let (Some(Operand::Reg(rn)), Some(op2)) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let tmp = ctx.alloc_vreg();
                    let src1 = self.arm_reg(rn);
                    let width = self.reg_width(rn);
                    let src2 = self.operand_to_src(op2, ctx)?;

                    push_op!(OpKind::Sub {
                        dst: tmp,
                        src1,
                        src2,
                        width,
                        flags: FlagUpdate::All,
                    });
                }
            }

            Mnemonic::CMN => {
                if let (Some(Operand::Reg(rn)), Some(op2)) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let tmp = ctx.alloc_vreg();
                    let src1 = self.arm_reg(rn);
                    let width = self.reg_width(rn);
                    let src2 = self.operand_to_src(op2, ctx)?;

                    push_op!(OpKind::Add {
                        dst: tmp,
                        src1,
                        src2,
                        width,
                        flags: FlagUpdate::All,
                    });
                }
            }

            // =================================================================
            // Move
            // =================================================================
            Mnemonic::MOV | Mnemonic::MOVS => {
                if let (Some(Operand::Reg(rd)), Some(src_op)) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    let src = self.operand_to_src(src_op, ctx)?;

                    push_op!(OpKind::Mov { dst, src, width });
                }
            }

            Mnemonic::MOVZ => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Imm(imm))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    let val = imm.effective_value();

                    push_op!(OpKind::Mov {
                        dst,
                        src: SrcOperand::Imm(val),
                        width,
                    });
                }
            }

            Mnemonic::MOVN => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Imm(imm))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    let val = !imm.effective_value();

                    push_op!(OpKind::Mov {
                        dst,
                        src: SrcOperand::Imm(val),
                        width,
                    });
                }
            }

            Mnemonic::MOVK => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Imm(imm))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    let shift = imm.shift;
                    let mask = !(0xFFFFu64 << shift);
                    let insert_val = (imm.value as u64) << shift;

                    let tmp = ctx.alloc_vreg();

                    push_op!(OpKind::And {
                        dst: tmp,
                        src1: self.arm_reg(rd),
                        src2: SrcOperand::Imm(mask as i64),
                        width,
                        flags: FlagUpdate::None,
                    });

                    push_op!(OpKind::Or {
                        dst,
                        src1: tmp,
                        src2: SrcOperand::Imm(insert_val as i64),
                        width,
                        flags: FlagUpdate::None,
                    });
                }
            }

            // =================================================================
            // Address Calculation
            // =================================================================
            Mnemonic::ADR => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Label(offset))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let target = (pc as i64).wrapping_add(*offset) as u64;

                    push_op!(OpKind::Mov {
                        dst,
                        src: SrcOperand::Imm(target as i64),
                        width: OpWidth::W64,
                    });
                }
            }

            Mnemonic::ADRP => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Label(offset))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let page = pc & !0xFFF;
                    let target = (page as i64).wrapping_add(*offset) as u64;

                    push_op!(OpKind::Mov {
                        dst,
                        src: SrcOperand::Imm(target as i64),
                        width: OpWidth::W64,
                    });
                }
            }

            // =================================================================
            // Shifts
            // =================================================================
            Mnemonic::LSL | Mnemonic::LSLS => {
                self.lift_shift(insn, ShiftOp::Lsl, pc, &mut ops, ctx)?;
            }

            Mnemonic::LSR | Mnemonic::LSRS => {
                self.lift_shift(insn, ShiftOp::Lsr, pc, &mut ops, ctx)?;
            }

            Mnemonic::ASR | Mnemonic::ASRS => {
                self.lift_shift(insn, ShiftOp::Asr, pc, &mut ops, ctx)?;
            }

            Mnemonic::ROR | Mnemonic::RORS => {
                self.lift_shift(insn, ShiftOp::Ror, pc, &mut ops, ctx)?;
            }

            Mnemonic::EXTR => {
                self.lift_extract(insn, pc, &mut ops, ctx)?;
            }

            Mnemonic::UBFX => {
                self.lift_bitfield(
                    insn,
                    BitfieldKind::Extract { sign_extend: false },
                    pc,
                    &mut ops,
                    ctx,
                )?;
            }

            Mnemonic::SBFX => {
                self.lift_bitfield(
                    insn,
                    BitfieldKind::Extract { sign_extend: true },
                    pc,
                    &mut ops,
                    ctx,
                )?;
            }

            Mnemonic::BFI | Mnemonic::BFC => {
                self.lift_bitfield(insn, BitfieldKind::Insert, pc, &mut ops, ctx)?;
            }

            Mnemonic::BFXIL => {
                self.lift_bitfield(insn, BitfieldKind::InsertLow, pc, &mut ops, ctx)?;
            }

            Mnemonic::UBFIZ => {
                self.lift_bitfield(
                    insn,
                    BitfieldKind::InsertZero { sign_extend: false },
                    pc,
                    &mut ops,
                    ctx,
                )?;
            }

            Mnemonic::SBFIZ => {
                self.lift_bitfield(
                    insn,
                    BitfieldKind::InsertZero { sign_extend: true },
                    pc,
                    &mut ops,
                    ctx,
                )?;
            }

            // =================================================================
            // Conditional Compare
            // =================================================================
            Mnemonic::CCMP | Mnemonic::CCMN => {
                self.lift_cond_compare(insn, pc, &mut ops, ctx)?;
            }

            // =================================================================
            // Extend
            // =================================================================
            Mnemonic::SXTB => {
                self.lift_extend(insn, OpWidth::W8, true, pc, &mut ops, ctx)?;
            }

            Mnemonic::SXTH => {
                self.lift_extend(insn, OpWidth::W16, true, pc, &mut ops, ctx)?;
            }

            Mnemonic::SXTW => {
                self.lift_extend(insn, OpWidth::W32, true, pc, &mut ops, ctx)?;
            }

            Mnemonic::UXTB => {
                self.lift_extend(insn, OpWidth::W8, false, pc, &mut ops, ctx)?;
            }

            Mnemonic::UXTH => {
                self.lift_extend(insn, OpWidth::W16, false, pc, &mut ops, ctx)?;
            }

            // =================================================================
            // Conditional Select
            // =================================================================
            Mnemonic::CSEL
            | Mnemonic::CSINC
            | Mnemonic::CSINV
            | Mnemonic::CSNEG
            | Mnemonic::CSET
            | Mnemonic::CSETM
            | Mnemonic::CINC
            | Mnemonic::CINV
            | Mnemonic::CNEG => {
                self.lift_cond_select(insn, pc, &mut ops, ctx)?;
            }

            // =================================================================
            // Bit manipulation
            // =================================================================
            Mnemonic::CLZ => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    push_op!(OpKind::Clz {
                        dst,
                        src: self.arm_reg(rn),
                        width,
                    });
                }
            }

            Mnemonic::CLS => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let src = self.arm_reg(rn);
                    let width = self.reg_width(rd);

                    let sign_mask = ctx.alloc_vreg();
                    push_op!(OpKind::Sar {
                        dst: sign_mask,
                        src,
                        amount: SrcOperand::Imm(i64::from(width.bits() - 1)),
                        width,
                        flags: FlagUpdate::None,
                    });

                    let normalized = ctx.alloc_vreg();
                    push_op!(OpKind::Xor {
                        dst: normalized,
                        src1: src,
                        src2: SrcOperand::Reg(sign_mask),
                        width,
                        flags: FlagUpdate::None,
                    });

                    let leading = ctx.alloc_vreg();
                    push_op!(OpKind::Clz {
                        dst: leading,
                        src: normalized,
                        width,
                    });

                    push_op!(OpKind::Sub {
                        dst,
                        src1: leading,
                        src2: SrcOperand::Imm(1),
                        width,
                        flags: FlagUpdate::None,
                    });
                }
            }

            Mnemonic::RBIT => {
                if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let dst = self.dst_reg(rd, ctx);
                    let width = self.reg_width(rd);
                    push_op!(OpKind::Rbit {
                        dst,
                        src: self.arm_reg(rn),
                        width,
                    });
                }
            }

            Mnemonic::REV => {
                self.lift_rev(insn, RevKind::Full, pc, &mut ops, ctx)?;
            }

            Mnemonic::REV16 => {
                self.lift_rev(insn, RevKind::Halfwords, pc, &mut ops, ctx)?;
            }

            Mnemonic::REV32 => {
                self.lift_rev(insn, RevKind::Words, pc, &mut ops, ctx)?;
            }

            // =================================================================
            // Load/Store
            // =================================================================
            Mnemonic::LDR => {
                let width = match insn.operands.first() {
                    Some(Operand::Reg(r)) if !r.is_64bit => MemWidth::B4,
                    _ => MemWidth::B8,
                };
                self.lift_load(insn, width, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDRB => {
                self.lift_load(insn, MemWidth::B1, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDRH => {
                self.lift_load(insn, MemWidth::B2, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDRSB => {
                self.lift_load(insn, MemWidth::B1, SignExtend::Sign, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDRSH => {
                self.lift_load(insn, MemWidth::B2, SignExtend::Sign, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDRSW => {
                self.lift_load(insn, MemWidth::B4, SignExtend::Sign, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDXR | Mnemonic::LDAXR => {
                let width = match insn.operands.first() {
                    Some(Operand::Reg(r)) if !r.is_64bit => MemWidth::B4,
                    _ => MemWidth::B8,
                };
                self.lift_load_exclusive(insn, width, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDXRB | Mnemonic::LDAXRB => {
                self.lift_load_exclusive(insn, MemWidth::B1, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDXRH | Mnemonic::LDAXRH => {
                self.lift_load_exclusive(insn, MemWidth::B2, pc, &mut ops, ctx)?;
            }

            Mnemonic::STXR | Mnemonic::STLXR => {
                let width = match insn.operands.get(1) {
                    Some(Operand::Reg(r)) if !r.is_64bit => MemWidth::B4,
                    _ => MemWidth::B8,
                };
                self.lift_store_exclusive(insn, width, pc, &mut ops, ctx)?;
            }

            Mnemonic::STXRB | Mnemonic::STLXRB => {
                self.lift_store_exclusive(insn, MemWidth::B1, pc, &mut ops, ctx)?;
            }

            Mnemonic::STXRH | Mnemonic::STLXRH => {
                self.lift_store_exclusive(insn, MemWidth::B2, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDAR => {
                let width = match insn.operands.first() {
                    Some(Operand::Reg(r)) if !r.is_64bit => MemWidth::B4,
                    _ => MemWidth::B8,
                };
                self.lift_load(insn, width, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDARB => {
                self.lift_load(insn, MemWidth::B1, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDARH => {
                self.lift_load(insn, MemWidth::B2, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::STR => {
                let width = match insn.operands.first() {
                    Some(Operand::Reg(r)) if !r.is_64bit => MemWidth::B4,
                    _ => MemWidth::B8,
                };
                self.lift_store(insn, width, pc, &mut ops, ctx)?;
            }

            Mnemonic::STRB => {
                self.lift_store(insn, MemWidth::B1, pc, &mut ops, ctx)?;
            }

            Mnemonic::STRH => {
                self.lift_store(insn, MemWidth::B2, pc, &mut ops, ctx)?;
            }

            Mnemonic::STLR => {
                let width = match insn.operands.first() {
                    Some(Operand::Reg(r)) if !r.is_64bit => MemWidth::B4,
                    _ => MemWidth::B8,
                };
                self.lift_store(insn, width, pc, &mut ops, ctx)?;
            }

            Mnemonic::STLRB => {
                self.lift_store(insn, MemWidth::B1, pc, &mut ops, ctx)?;
            }

            Mnemonic::STLRH => {
                self.lift_store(insn, MemWidth::B2, pc, &mut ops, ctx)?;
            }

            Mnemonic::SWP
            | Mnemonic::SWPA
            | Mnemonic::SWPAL
            | Mnemonic::SWPL
            | Mnemonic::CAS
            | Mnemonic::CASA
            | Mnemonic::CASAL
            | Mnemonic::CASL
            | Mnemonic::LDADD
            | Mnemonic::LDADDA
            | Mnemonic::LDADDAL
            | Mnemonic::LDADDL
            | Mnemonic::LDCLR
            | Mnemonic::LDEOR
            | Mnemonic::LDSET
            | Mnemonic::LDSMAX
            | Mnemonic::LDSMIN
            | Mnemonic::LDUMAX
            | Mnemonic::LDUMIN => {
                if matches!(
                    insn.mnemonic,
                    Mnemonic::CAS | Mnemonic::CASA | Mnemonic::CASAL | Mnemonic::CASL
                ) {
                    self.lift_cas(insn, pc, &mut ops, ctx)?;
                } else {
                    self.lift_atomic_rmw(insn, pc, &mut ops, ctx)?;
                }
            }

            Mnemonic::LDP | Mnemonic::LDNP => {
                self.lift_load_pair(insn, SignExtend::Zero, pc, &mut ops, ctx)?;
            }

            Mnemonic::LDPSW => {
                self.lift_load_pair(insn, SignExtend::Sign, pc, &mut ops, ctx)?;
            }

            Mnemonic::STP | Mnemonic::STNP => {
                self.lift_store_pair(insn, pc, &mut ops, ctx)?;
            }

            Mnemonic::PRFM => {
                push_op!(OpKind::Nop);
            }

            // =================================================================
            // Branches
            // =================================================================
            Mnemonic::B => {
                if let Some(Operand::Label(offset)) = insn.operands.get(0) {
                    let target = (pc as i64).wrapping_add(*offset) as u64;
                    control = ControlFlow::Branch { target };
                }
            }

            Mnemonic::BL => {
                if let Some(Operand::Label(offset)) = insn.operands.get(0) {
                    let target = (pc as i64).wrapping_add(*offset) as u64;
                    let ret_addr = pc + 4;

                    push_op!(OpKind::Mov {
                        dst: VReg::Arch(ArchReg::Arm(ArmReg::X(30))),
                        src: SrcOperand::Imm(ret_addr as i64),
                        width: OpWidth::W64,
                    });

                    control = ControlFlow::Call {
                        target: CallTarget::GuestAddr(target),
                    };
                }
            }

            Mnemonic::BR => {
                if let Some(Operand::Reg(rn)) = insn.operands.get(0) {
                    control = ControlFlow::IndirectBranch {
                        target: self.arm_reg(rn),
                    };
                }
            }

            Mnemonic::BLR => {
                if let Some(Operand::Reg(rn)) = insn.operands.get(0) {
                    let ret_addr = pc + 4;

                    push_op!(OpKind::Mov {
                        dst: VReg::Arch(ArchReg::Arm(ArmReg::X(30))),
                        src: SrcOperand::Imm(ret_addr as i64),
                        width: OpWidth::W64,
                    });

                    control = ControlFlow::Call {
                        target: CallTarget::Indirect(self.arm_reg(rn)),
                    };
                }
            }

            Mnemonic::RET => {
                let target = match insn.operands.get(0) {
                    Some(Operand::Reg(rn)) => self.arm_reg(rn),
                    _ => VReg::Arch(ArchReg::Arm(ArmReg::X(30))),
                };
                control = ControlFlow::IndirectBranch { target };
            }

            Mnemonic::BLRAA | Mnemonic::BLRAB => {
                if let Some(Operand::Reg(rn)) = insn.operands.get(0) {
                    let ret_addr = pc + 4;
                    push_op!(OpKind::Mov {
                        dst: VReg::Arch(ArchReg::Arm(ArmReg::X(30))),
                        src: SrcOperand::Imm(ret_addr as i64),
                        width: OpWidth::W64,
                    });
                    control = ControlFlow::Call {
                        target: CallTarget::Indirect(self.arm_reg(rn)),
                    };
                }
            }

            Mnemonic::RETAA | Mnemonic::RETAB => {
                let target = VReg::Arch(ArchReg::Arm(ArmReg::X(30)));
                control = ControlFlow::IndirectBranch { target };
            }

            Mnemonic::PACIA
            | Mnemonic::PACIB
            | Mnemonic::PACDA
            | Mnemonic::PACDB
            | Mnemonic::AUTIA
            | Mnemonic::AUTIB
            | Mnemonic::AUTDA
            | Mnemonic::AUTDB
            | Mnemonic::PACIZA
            | Mnemonic::PACIZB
            | Mnemonic::PACDZA
            | Mnemonic::PACDZB
            | Mnemonic::AUTIZA
            | Mnemonic::AUTIZB
            | Mnemonic::AUTDZA
            | Mnemonic::AUTDZB
            | Mnemonic::XPACI
            | Mnemonic::XPACD
            | Mnemonic::PACGA
            | Mnemonic::IRG
            | Mnemonic::GMI => {
                push_op!(OpKind::Nop);
            }

            Mnemonic::BCC => {
                if let (Some(Operand::Label(offset)), Some(cond)) =
                    (insn.operands.get(0), insn.cond)
                {
                    let target = (pc as i64).wrapping_add(*offset) as u64;
                    let fallthrough = pc + 4;
                    control = ControlFlow::CondBranch {
                        cond: self.arm_cond(cond),
                        target,
                        fallthrough,
                    };
                }
            }

            Mnemonic::CBZ => {
                if let (Some(Operand::Reg(rn)), Some(Operand::Label(offset))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let target = (pc as i64).wrapping_add(*offset) as u64;
                    let fallthrough = pc + 4;
                    let width = self.reg_width(rn);
                    let cond = if width == OpWidth::W32 {
                        let tmp = ctx.alloc_vreg();
                        push_op!(OpKind::ZeroExtend {
                            dst: tmp,
                            src: self.arm_reg(rn),
                            from_width: OpWidth::W32,
                            to_width: OpWidth::W64,
                        });
                        tmp
                    } else {
                        self.arm_reg(rn)
                    };

                    control = ControlFlow::CondBranchReg {
                        cond,
                        taken: fallthrough,
                        not_taken: target,
                    };
                }
            }

            Mnemonic::CBNZ => {
                if let (Some(Operand::Reg(rn)), Some(Operand::Label(offset))) =
                    (insn.operands.get(0), insn.operands.get(1))
                {
                    let target = (pc as i64).wrapping_add(*offset) as u64;
                    let fallthrough = pc + 4;
                    let width = self.reg_width(rn);
                    let cond = if width == OpWidth::W32 {
                        let tmp = ctx.alloc_vreg();
                        push_op!(OpKind::ZeroExtend {
                            dst: tmp,
                            src: self.arm_reg(rn),
                            from_width: OpWidth::W32,
                            to_width: OpWidth::W64,
                        });
                        tmp
                    } else {
                        self.arm_reg(rn)
                    };

                    control = ControlFlow::CondBranchReg {
                        cond,
                        taken: target,
                        not_taken: fallthrough,
                    };
                }
            }

            Mnemonic::TBZ | Mnemonic::TBNZ => {
                if let (
                    Some(Operand::Reg(rt)),
                    Some(Operand::Imm(bit)),
                    Some(Operand::Label(offset)),
                ) = (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    let bit = bit.effective_value() as u32;
                    let target = (pc as i64).wrapping_add(*offset) as u64;
                    let fallthrough = pc + 4;
                    let masked = ctx.alloc_vreg();
                    let mask = (1u64 << bit) as i64;

                    push_op!(OpKind::And {
                        dst: masked,
                        src1: self.arm_reg(rt),
                        src2: SrcOperand::Imm64(mask),
                        width: self.reg_width(rt),
                        flags: FlagUpdate::None,
                    });

                    let (taken, not_taken) = if insn.mnemonic == Mnemonic::TBNZ {
                        (target, fallthrough)
                    } else {
                        (fallthrough, target)
                    };
                    control = ControlFlow::CondBranchReg {
                        cond: masked,
                        taken,
                        not_taken,
                    };
                }
            }

            // =================================================================
            // System
            // =================================================================
            Mnemonic::NOP
            | Mnemonic::BTI
            | Mnemonic::YIELD
            | Mnemonic::WFE
            | Mnemonic::WFI
            | Mnemonic::SEV
            | Mnemonic::SEVL => {
                push_op!(OpKind::Nop);
            }

            Mnemonic::HINT => {
                let imm = match insn.operands.first() {
                    Some(Operand::Imm(imm)) => imm.effective_value(),
                    _ => return Err(LiftError::Internal("invalid HINT operands".to_string())),
                };
                match imm {
                    // DGH is a data-gathering hint with no visible architectural state.
                    6 => push_op!(OpKind::Nop),
                    _ if self.strict => {
                        return Err(LiftError::Unsupported {
                            addr: pc,
                            mnemonic: format!("HINT #{imm}"),
                        });
                    }
                    _ => {
                        control = ControlFlow::Trap {
                            kind: TrapKind::Undefined,
                        };
                    }
                }
            }

            Mnemonic::CFINV => {
                push_op!(OpKind::Xor {
                    dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                    src1: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                    src2: SrcOperand::Imm(NZCV_C),
                    width: OpWidth::W32,
                    flags: FlagUpdate::None,
                });
            }

            Mnemonic::AXFLAG => {
                self.lift_axflag(pc, &mut ops, ctx);
            }

            Mnemonic::XAFLAG => {
                self.lift_xaflag(pc, &mut ops, ctx);
            }

            Mnemonic::MRS => {
                self.lift_mrs(insn, pc, &mut ops, ctx)?;
            }

            Mnemonic::MSR => {
                self.lift_msr(insn, pc, &mut ops, ctx)?;
            }

            Mnemonic::SVC => {
                control = ControlFlow::Syscall;
            }

            Mnemonic::BRK => {
                control = ControlFlow::Trap {
                    kind: TrapKind::Breakpoint,
                };
            }

            Mnemonic::HLT => {
                control = ControlFlow::Trap {
                    kind: TrapKind::Halt,
                };
            }

            Mnemonic::UDF => {
                control = ControlFlow::Trap {
                    kind: TrapKind::Undefined,
                };
            }

            Mnemonic::CLREX => {
                push_op!(OpKind::ClearExclusive);
            }

            Mnemonic::DMB | Mnemonic::DSB | Mnemonic::ISB => {
                push_op!(OpKind::Fence {
                    kind: FenceKind::Full,
                });
            }

            // =================================================================
            // Scalar FP - 2-source
            // =================================================================
            Mnemonic::FADD
            | Mnemonic::FSUB
            | Mnemonic::FMUL
            | Mnemonic::FDIV
            | Mnemonic::FMAX
            | Mnemonic::FMIN
            | Mnemonic::FMAXNM
            | Mnemonic::FMINNM => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let rm = match insn.operands.get(2) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rm".to_string())),
                };
                let precision = Self::fp_precision(&rd.size);
                let kind = match insn.mnemonic {
                    Mnemonic::FADD => OpKind::FAdd {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                    Mnemonic::FSUB => OpKind::FSub {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                    Mnemonic::FMUL => OpKind::FMul {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                    Mnemonic::FDIV => OpKind::FDiv {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                    Mnemonic::FMAX | Mnemonic::FMAXNM => OpKind::FMax {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                    Mnemonic::FMIN | Mnemonic::FMINNM => OpKind::FMin {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                    _ => unreachable!(),
                };
                ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));
            }

            Mnemonic::FNMUL => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let rm = match insn.operands.get(2) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rm".to_string())),
                };
                let precision = Self::fp_precision(&rd.size);
                let mul_dst = ctx.alloc_vreg();
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FMul {
                        dst: mul_dst,
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                );
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FNeg {
                        dst: Self::fp_vreg(rd),
                        src: mul_dst,
                        precision,
                    },
                );
            }

            // =================================================================
            // Scalar FP - 3-source (FMA)
            // =================================================================
            Mnemonic::FMADD | Mnemonic::FMSUB | Mnemonic::FNMADD | Mnemonic::FNMSUB => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let rm = match insn.operands.get(2) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rm".to_string())),
                };
                let ra = match insn.operands.get(3) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp ra".to_string())),
                };
                let precision = Self::fp_precision(&rd.size);
                match insn.mnemonic {
                    Mnemonic::FMADD => {
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FFma {
                                dst: Self::fp_vreg(rd),
                                src1: Self::fp_vreg(rn),
                                src2: Self::fp_vreg(rm),
                                src3: Self::fp_vreg(ra),
                                precision,
                            },
                        );
                    }
                    Mnemonic::FMSUB => {
                        let mul_dst = ctx.alloc_vreg();
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FMul {
                                dst: mul_dst,
                                src1: Self::fp_vreg(rn),
                                src2: Self::fp_vreg(rm),
                                precision,
                            },
                        );
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FSub {
                                dst: Self::fp_vreg(rd),
                                src1: mul_dst,
                                src2: Self::fp_vreg(ra),
                                precision,
                            },
                        );
                    }
                    Mnemonic::FNMADD => {
                        let mul_dst = ctx.alloc_vreg();
                        let neg_dst = ctx.alloc_vreg();
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FMul {
                                dst: mul_dst,
                                src1: Self::fp_vreg(rn),
                                src2: Self::fp_vreg(rm),
                                precision,
                            },
                        );
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FNeg {
                                dst: neg_dst,
                                src: mul_dst,
                                precision,
                            },
                        );
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FAdd {
                                dst: Self::fp_vreg(rd),
                                src1: neg_dst,
                                src2: Self::fp_vreg(ra),
                                precision,
                            },
                        );
                    }
                    Mnemonic::FNMSUB => {
                        let fma_dst = ctx.alloc_vreg();
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FFma {
                                dst: fma_dst,
                                src1: Self::fp_vreg(rn),
                                src2: Self::fp_vreg(rm),
                                src3: Self::fp_vreg(ra),
                                precision,
                            },
                        );
                        Self::push_lifted_op(
                            &mut ops,
                            pc,
                            OpKind::FNeg {
                                dst: Self::fp_vreg(rd),
                                src: fma_dst,
                                precision,
                            },
                        );
                    }
                    _ => unreachable!(),
                }
            }

            // =================================================================
            // Scalar FP - Unary
            // =================================================================
            Mnemonic::FMOV | Mnemonic::FABS | Mnemonic::FNEG | Mnemonic::FSQRT => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let precision = Self::fp_precision(&rd.size);
                let kind = match insn.mnemonic {
                    Mnemonic::FMOV => OpKind::Mov {
                        dst: Self::fp_vreg(rd),
                        src: SrcOperand::Reg(Self::fp_vreg(rn)),
                        width: match precision {
                            FpPrecision::F32 => OpWidth::W32,
                            FpPrecision::F64 => OpWidth::W64,
                            _ => OpWidth::W32,
                        },
                    },
                    Mnemonic::FABS => OpKind::FAbs {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        precision,
                    },
                    Mnemonic::FNEG => OpKind::FNeg {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        precision,
                    },
                    Mnemonic::FSQRT => OpKind::FSqrt {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        precision,
                    },
                    _ => unreachable!(),
                };
                ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));
            }

            // =================================================================
            // Scalar FP - Compare
            // =================================================================
            Mnemonic::FCMP | Mnemonic::FCMPE => {
                let rn = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let precision = Self::fp_precision(&rn.size);
                let src2 = match insn.operands.get(1) {
                    Some(Operand::FpReg(rm)) => Self::fp_vreg(rm),
                    None => VReg::Imm(0),
                    _ => return Err(LiftError::Internal("invalid FCMP operand".to_string())),
                };
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FCmp {
                        src1: Self::fp_vreg(rn),
                        src2,
                        precision,
                    },
                );
            }

            Mnemonic::FCCMP | Mnemonic::FCCMPE => {
                let rn = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let rm = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rm".to_string())),
                };
                let _nzcv_imm = match insn.operands.get(2) {
                    Some(Operand::Imm(imm)) => imm.effective_value(),
                    _ => return Err(LiftError::Internal("missing FCCMP nzcv".to_string())),
                };
                let precision = Self::fp_precision(&rn.size);
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FCmp {
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        precision,
                    },
                );
            }

            // =================================================================
            // Scalar FP - Convert
            // =================================================================
            Mnemonic::FCVT => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let from = Self::fp_precision(&rn.size);
                let to = Self::fp_precision(&rd.size);
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FConvert {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        from,
                        to,
                    },
                );
            }

            Mnemonic::FCVTNS
            | Mnemonic::FCVTNU
            | Mnemonic::FCVTAS
            | Mnemonic::FCVTAU
            | Mnemonic::FCVTPS
            | Mnemonic::FCVTPU
            | Mnemonic::FCVTMS
            | Mnemonic::FCVTMU
            | Mnemonic::FCVTZS
            | Mnemonic::FCVTZU => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let precision = Self::fp_precision(&rn.size);
                let int_width = match precision {
                    FpPrecision::F32 => OpWidth::W32,
                    FpPrecision::F64 => OpWidth::W64,
                    _ => OpWidth::W32,
                };
                let (signed, round) = match insn.mnemonic {
                    Mnemonic::FCVTNS => (true, FpRoundMode::RoundNearest),
                    Mnemonic::FCVTNU => (false, FpRoundMode::RoundNearest),
                    Mnemonic::FCVTAS => (true, FpRoundMode::Dynamic),
                    Mnemonic::FCVTAU => (false, FpRoundMode::Dynamic),
                    Mnemonic::FCVTPS => (true, FpRoundMode::RoundUp),
                    Mnemonic::FCVTPU => (false, FpRoundMode::RoundUp),
                    Mnemonic::FCVTMS => (true, FpRoundMode::RoundDown),
                    Mnemonic::FCVTMU => (false, FpRoundMode::RoundDown),
                    Mnemonic::FCVTZS => (true, FpRoundMode::RoundTowardZero),
                    Mnemonic::FCVTZU => (false, FpRoundMode::RoundTowardZero),
                    _ => unreachable!(),
                };
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FpToInt {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        fp_precision: precision,
                        int_width,
                        signed,
                        round,
                    },
                );
            }

            Mnemonic::SCVTF | Mnemonic::UCVTF => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let fp_precision = Self::fp_precision(&rd.size);
                let int_width = match fp_precision {
                    FpPrecision::F32 => OpWidth::W32,
                    FpPrecision::F64 => OpWidth::W64,
                    _ => OpWidth::W32,
                };
                let signed = insn.mnemonic == Mnemonic::SCVTF;
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::IntToFp {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        int_width,
                        fp_precision,
                        signed,
                    },
                );
            }

            // =================================================================
            // Scalar FP - Round
            // =================================================================
            Mnemonic::FRINTN
            | Mnemonic::FRINTP
            | Mnemonic::FRINTM
            | Mnemonic::FRINTZ
            | Mnemonic::FRINTA
            | Mnemonic::FRINTX
            | Mnemonic::FRINTI => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let precision = Self::fp_precision(&rd.size);
                let mode = match insn.mnemonic {
                    Mnemonic::FRINTN => FpRoundMode::RoundNearest,
                    Mnemonic::FRINTP => FpRoundMode::RoundUp,
                    Mnemonic::FRINTM => FpRoundMode::RoundDown,
                    Mnemonic::FRINTZ => FpRoundMode::RoundTowardZero,
                    Mnemonic::FRINTA => FpRoundMode::RoundNearest,
                    Mnemonic::FRINTX => FpRoundMode::Dynamic,
                    Mnemonic::FRINTI => FpRoundMode::Dynamic,
                    _ => unreachable!(),
                };
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::FRound {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        precision,
                        mode,
                    },
                );
            }

            // =================================================================
            // Scalar FP - Conditional Select
            // =================================================================
            Mnemonic::FCSEL => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rn".to_string())),
                };
                let rm = match insn.operands.get(2) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing fp rm".to_string())),
                };
                let cond = match insn.operands.get(3) {
                    Some(Operand::Cond(c)) => self.arm_cond(*c),
                    _ => return Err(LiftError::Internal("missing FCSEL cond".to_string())),
                };
                let width = match Self::fp_precision(&rd.size) {
                    FpPrecision::F32 => OpWidth::W32,
                    FpPrecision::F64 => OpWidth::W64,
                    _ => OpWidth::W32,
                };
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::Mov {
                        dst: Self::fp_vreg(rd),
                        src: SrcOperand::Reg(Self::fp_vreg(rm)),
                        width,
                    },
                );
                Self::push_lifted_op(
                    &mut ops,
                    pc,
                    OpKind::CMove {
                        dst: Self::fp_vreg(rd),
                        src: Self::fp_vreg(rn),
                        cond,
                        width,
                    },
                );
            }

            // =================================================================
            // NEON Integer Three-Same
            // =================================================================
            Mnemonic::VADD | Mnemonic::VSUB | Mnemonic::VMUL | Mnemonic::VMAX | Mnemonic::VMIN => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing vec rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing vec rn".to_string())),
                };
                let rm = match insn.operands.get(2) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing vec rm".to_string())),
                };
                let q = (insn.raw >> 30) & 1;
                let size = (insn.raw >> 22) & 3;
                let (elem, lanes) = match (size, q) {
                    (0, 0) => (VecElementType::I8, 8),
                    (0, 1) => (VecElementType::I8, 16),
                    (1, 0) => (VecElementType::I16, 4),
                    (1, 1) => (VecElementType::I16, 8),
                    (2, 0) => (VecElementType::I32, 2),
                    (2, 1) => (VecElementType::I32, 4),
                    (3, 0) => (VecElementType::I64, 1),
                    (3, 1) => (VecElementType::I64, 2),
                    _ => (VecElementType::I8, 16),
                };
                let kind = match insn.mnemonic {
                    Mnemonic::VADD => OpKind::VAdd {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        elem,
                        lanes,
                    },
                    Mnemonic::VSUB => OpKind::VSub {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        elem,
                        lanes,
                    },
                    Mnemonic::VMUL => OpKind::VMul {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        elem,
                        lanes,
                    },
                    Mnemonic::VMAX => OpKind::VMax {
                        dst: Self::fp_vreg(rd),
                        src1: Self::fp_vreg(rn),
                        src2: Self::fp_vreg(rm),
                        elem,
                        lanes,
                    },
                    Mnemonic::VMIN => {
                        let u = (insn.raw >> 29) & 1;
                        OpKind::VMin {
                            dst: Self::fp_vreg(rd),
                            src1: Self::fp_vreg(rn),
                            src2: Self::fp_vreg(rm),
                            elem,
                            lanes,
                            signed: u == 0,
                        }
                    }
                    _ => unreachable!(),
                };
                push_op!(kind);
            }

            // =================================================================
            // NEON Logical Three-Same
            // =================================================================
            Mnemonic::VAND | Mnemonic::VBIC | Mnemonic::VORR | Mnemonic::VEOR => {
                let rd = match insn.operands.get(0) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing vec rd".to_string())),
                };
                let rn = match insn.operands.get(1) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing vec rn".to_string())),
                };
                let rm = match insn.operands.get(2) {
                    Some(Operand::FpReg(r)) => r,
                    _ => return Err(LiftError::Internal("missing vec rm".to_string())),
                };
                let q = (insn.raw >> 30) & 1;
                let vec_width = if q == 1 {
                    VecWidth::V128
                } else {
                    VecWidth::V64
                };
                match insn.mnemonic {
                    Mnemonic::VAND => {
                        push_op!(OpKind::VAnd {
                            dst: Self::fp_vreg(rd),
                            src1: Self::fp_vreg(rn),
                            src2: Self::fp_vreg(rm),
                            width: vec_width,
                        });
                    }
                    Mnemonic::VORR => {
                        push_op!(OpKind::VOr {
                            dst: Self::fp_vreg(rd),
                            src1: Self::fp_vreg(rn),
                            src2: Self::fp_vreg(rm),
                            width: vec_width,
                        });
                    }
                    Mnemonic::VEOR => {
                        push_op!(OpKind::VXor {
                            dst: Self::fp_vreg(rd),
                            src1: Self::fp_vreg(rn),
                            src2: Self::fp_vreg(rm),
                            width: vec_width,
                        });
                    }
                    Mnemonic::VBIC => {
                        let not_rm = ctx.alloc_vreg();
                        push_op!(OpKind::VXor {
                            dst: not_rm,
                            src1: Self::fp_vreg(rm),
                            src2: VReg::Imm(-1),
                            width: vec_width,
                        });
                        push_op!(OpKind::VAnd {
                            dst: Self::fp_vreg(rd),
                            src1: Self::fp_vreg(rn),
                            src2: not_rm,
                            width: vec_width,
                        });
                    }
                    _ => unreachable!(),
                };
            }

            // =================================================================
            // Unhandled
            // =================================================================
            _ => {
                if self.strict {
                    return Err(LiftError::Unsupported {
                        addr: pc,
                        mnemonic: format!("{:?}", insn.mnemonic),
                    });
                }
                control = ControlFlow::Trap {
                    kind: TrapKind::Undefined,
                };
            }
        }

        Ok((ops, control))
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    fn parse_arith_operands(
        &self,
        insn: &DecodedInsn,
        ctx: &mut LiftContext,
    ) -> Result<(VReg, VReg, SrcOperand, OpWidth), LiftError> {
        let rd = match insn.operands.get(0) {
            Some(Operand::Reg(r)) => r,
            _ => return Err(LiftError::Internal("missing rd".to_string())),
        };

        let rn = match insn.operands.get(1) {
            Some(Operand::Reg(r)) => r,
            _ => return Err(LiftError::Internal("missing rn".to_string())),
        };

        let src2 = self.parse_operand2(insn, 2, ctx)?;

        Ok((
            self.dst_reg(rd, ctx),
            self.arm_reg(rn),
            src2,
            self.reg_width(rd),
        ))
    }

    fn parse_operand2(
        &self,
        insn: &DecodedInsn,
        idx: usize,
        _ctx: &mut LiftContext,
    ) -> Result<SrcOperand, LiftError> {
        match insn.operands.get(idx) {
            Some(Operand::Reg(r)) => Ok(SrcOperand::Reg(self.arm_reg(r))),
            Some(Operand::Imm(imm)) => Ok(SrcOperand::Imm(imm.effective_value())),
            Some(Operand::ShiftedReg(sr)) => Ok(SrcOperand::Shifted {
                reg: self.arm_reg(&sr.reg),
                shift: self.arm_shift(sr.shift_type),
                amount: sr.amount,
            }),
            Some(Operand::ExtendedReg(er)) => Ok(SrcOperand::Extended {
                reg: self.arm_reg(&er.reg),
                extend: self.arm_extend(er.extend_type),
                shift: er.shift,
            }),
            _ => Err(LiftError::Internal("invalid operand2".to_string())),
        }
    }

    fn operand_to_src(
        &self,
        op: &Operand,
        _ctx: &mut LiftContext,
    ) -> Result<SrcOperand, LiftError> {
        match op {
            Operand::Reg(r) => Ok(SrcOperand::Reg(self.arm_reg(r))),
            Operand::Imm(imm) => Ok(SrcOperand::Imm(imm.effective_value())),
            Operand::ShiftedReg(sr) => Ok(SrcOperand::Shifted {
                reg: self.arm_reg(&sr.reg),
                shift: self.arm_shift(sr.shift_type),
                amount: sr.amount,
            }),
            Operand::ExtendedReg(er) => Ok(SrcOperand::Extended {
                reg: self.arm_reg(&er.reg),
                extend: self.arm_extend(er.extend_type),
                shift: er.shift,
            }),
            _ => Err(LiftError::Internal("invalid operand".to_string())),
        }
    }

    fn lift_rev(
        &self,
        insn: &DecodedInsn,
        kind: RevKind,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rd, rn) = match (insn.operands.get(0), insn.operands.get(1)) {
            (Some(Operand::Reg(rd)), Some(Operand::Reg(rn))) => (rd, rn),
            _ => return Err(LiftError::Internal("invalid REV operands".to_string())),
        };

        let dst = self.dst_reg(rd, ctx);
        let src = self.arm_reg(rn);
        let width = self.reg_width(rd);

        match kind {
            RevKind::Full => {
                Self::push_lifted_op(ops, pc, OpKind::Bswap { dst, src, width });
            }
            RevKind::Halfwords => {
                let lo = ctx.alloc_vreg();
                let hi = ctx.alloc_vreg();
                let lo_shifted = ctx.alloc_vreg();
                let hi_shifted = ctx.alloc_vreg();
                let (lo_mask, hi_mask) = if width == OpWidth::W64 {
                    (0x00ff_00ff_00ff_00ff_u64, 0xff00_ff00_ff00_ff00_u64)
                } else {
                    (0x00ff_00ff_u64, 0xff00_ff00_u64)
                };

                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::And {
                        dst: lo,
                        src1: src,
                        src2: SrcOperand::Imm64(lo_mask as i64),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::And {
                        dst: hi,
                        src1: src,
                        src2: SrcOperand::Imm64(hi_mask as i64),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Shl {
                        dst: lo_shifted,
                        src: lo,
                        amount: SrcOperand::Imm(8),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Shr {
                        dst: hi_shifted,
                        src: hi,
                        amount: SrcOperand::Imm(8),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Or {
                        dst,
                        src1: lo_shifted,
                        src2: SrcOperand::Reg(hi_shifted),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
            }
            RevKind::Words => {
                if width == OpWidth::W32 {
                    Self::push_lifted_op(ops, pc, OpKind::Bswap { dst, src, width });
                } else {
                    let lo_rev = ctx.alloc_vreg();
                    let hi = ctx.alloc_vreg();
                    let hi_rev = ctx.alloc_vreg();
                    let hi_shifted = ctx.alloc_vreg();

                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Bswap {
                            dst: lo_rev,
                            src,
                            width: OpWidth::W32,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Shr {
                            dst: hi,
                            src,
                            amount: SrcOperand::Imm(32),
                            width,
                            flags: FlagUpdate::None,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Bswap {
                            dst: hi_rev,
                            src: hi,
                            width: OpWidth::W32,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Shl {
                            dst: hi_shifted,
                            src: hi_rev,
                            amount: SrcOperand::Imm(32),
                            width,
                            flags: FlagUpdate::None,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Or {
                            dst,
                            src1: hi_shifted,
                            src2: SrcOperand::Reg(lo_rev),
                            width,
                            flags: FlagUpdate::None,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn lift_shift(
        &self,
        insn: &DecodedInsn,
        shift_op: ShiftOp,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(amount)) = (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            let dst = self.dst_reg(rd, ctx);
            let width = self.reg_width(rd);
            let flags = if insn.sets_flags {
                FlagUpdate::All
            } else {
                FlagUpdate::None
            };

            let amount_src = match amount {
                Operand::Imm(imm) => {
                    let value = if shift_op == ShiftOp::Lsl
                        && matches!(insn.operands.get(3), Some(Operand::Imm(_)))
                    {
                        i64::from(width.bits()) - imm.value
                    } else {
                        imm.value
                    };
                    SrcOperand::Imm(value)
                }
                Operand::Reg(r) => SrcOperand::Reg(self.arm_reg(r)),
                _ => return Err(LiftError::Internal("invalid shift amount".to_string())),
            };

            let kind = match shift_op {
                ShiftOp::Lsl => OpKind::Shl {
                    dst,
                    src: self.arm_reg(rn),
                    amount: amount_src,
                    width,
                    flags,
                },
                ShiftOp::Lsr => OpKind::Shr {
                    dst,
                    src: self.arm_reg(rn),
                    amount: amount_src,
                    width,
                    flags,
                },
                ShiftOp::Asr => OpKind::Sar {
                    dst,
                    src: self.arm_reg(rn),
                    amount: amount_src,
                    width,
                    flags,
                },
                ShiftOp::Ror | ShiftOp::Rrx => OpKind::Ror {
                    dst,
                    src: self.arm_reg(rn),
                    amount: amount_src,
                    width,
                    flags,
                },
            };

            ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));
        }

        Ok(())
    }

    fn lift_extract(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        if let (
            Some(Operand::Reg(rd)),
            Some(Operand::Reg(rn)),
            Some(Operand::Reg(rm)),
            Some(Operand::Imm(lsb)),
        ) = (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
            insn.operands.get(3),
        ) {
            let dst = self.dst_reg(rd, ctx);
            let width = self.reg_width(rd);
            let amount = lsb.value;

            if amount == 0 {
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Mov {
                        dst,
                        src: SrcOperand::Reg(self.arm_reg(rm)),
                        width,
                    },
                );
                return Ok(());
            }

            let lo = ctx.alloc_vreg();
            let hi = ctx.alloc_vreg();
            Self::push_lifted_op(
                ops,
                pc,
                OpKind::Shr {
                    dst: lo,
                    src: self.arm_reg(rm),
                    amount: SrcOperand::Imm(amount),
                    width,
                    flags: FlagUpdate::None,
                },
            );
            Self::push_lifted_op(
                ops,
                pc,
                OpKind::Shl {
                    dst: hi,
                    src: self.arm_reg(rn),
                    amount: SrcOperand::Imm(i64::from(width.bits()) - amount),
                    width,
                    flags: FlagUpdate::None,
                },
            );
            Self::push_lifted_op(
                ops,
                pc,
                OpKind::Or {
                    dst,
                    src1: lo,
                    src2: SrcOperand::Reg(hi),
                    width,
                    flags: FlagUpdate::None,
                },
            );
        }

        Ok(())
    }

    fn lift_bitfield(
        &self,
        insn: &DecodedInsn,
        kind: BitfieldKind,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let invalid = || LiftError::Internal("invalid bitfield operands".to_string());
        let (rd, rn, immr, imms) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
            insn.operands.get(3),
        ) {
            (
                Some(Operand::Reg(rd)),
                Some(Operand::Reg(rn)),
                Some(Operand::Imm(immr)),
                Some(Operand::Imm(imms)),
            ) => (rd, rn, immr.value, imms.value),
            _ => return Err(invalid()),
        };
        if !(0..=63).contains(&immr) || !(0..=63).contains(&imms) {
            return Err(invalid());
        }

        let dst = self.dst_reg(rd, ctx);
        let dst_in = self.arm_reg(rd);
        let src = self.arm_reg(rn);
        let width = self.reg_width(rd);
        let reg_bits = width.bits() as u8;
        let immr = immr as u8;
        let imms = imms as u8;
        if immr >= reg_bits || imms >= reg_bits {
            return Err(invalid());
        }

        match kind {
            BitfieldKind::Extract { sign_extend } => {
                if imms < immr {
                    return Err(invalid());
                }
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Bfx {
                        dst,
                        src,
                        lsb: immr,
                        width_bits: imms - immr + 1,
                        sign_extend,
                        op_width: width,
                    },
                );
            }
            BitfieldKind::Insert => {
                if imms >= immr {
                    return Err(invalid());
                }
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Bfi {
                        dst,
                        dst_in,
                        src,
                        lsb: reg_bits - immr,
                        width_bits: imms + 1,
                        op_width: width,
                    },
                );
            }
            BitfieldKind::InsertLow => {
                if imms < immr {
                    return Err(invalid());
                }
                let width_bits = imms - immr + 1;
                if width_bits == reg_bits {
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Mov {
                            dst,
                            src: SrcOperand::Reg(src),
                            width,
                        },
                    );
                } else {
                    let extracted = ctx.alloc_vreg();
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Bfx {
                            dst: extracted,
                            src,
                            lsb: immr,
                            width_bits,
                            sign_extend: false,
                            op_width: width,
                        },
                    );
                    Self::push_lifted_op(
                        ops,
                        pc,
                        OpKind::Bfi {
                            dst,
                            dst_in,
                            src: extracted,
                            lsb: 0,
                            width_bits,
                            op_width: width,
                        },
                    );
                }
            }
            BitfieldKind::InsertZero { sign_extend } => {
                if imms >= immr {
                    return Err(invalid());
                }
                let lsb = reg_bits - immr;
                let width_bits = imms + 1;
                let extracted = ctx.alloc_vreg();
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Bfx {
                        dst: extracted,
                        src,
                        lsb: 0,
                        width_bits,
                        sign_extend,
                        op_width: width,
                    },
                );
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Shl {
                        dst,
                        src: extracted,
                        amount: SrcOperand::Imm(i64::from(lsb)),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
            }
        }

        Ok(())
    }

    fn lift_cond_compare(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let invalid = || LiftError::Internal("invalid conditional compare operands".to_string());
        let (rn, op2, nzcv, cond) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
            insn.operands.get(3),
        ) {
            (
                Some(Operand::Reg(rn)),
                Some(op2),
                Some(Operand::Imm(nzcv)),
                Some(Operand::Cond(cond)),
            ) => (rn, op2, nzcv, cond),
            _ => return Err(invalid()),
        };

        let cond_reg = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::TestCondition {
                dst: cond_reg,
                cond: self.arm_cond(*cond),
            },
        );

        let cmp_result = ctx.alloc_vreg();
        let width = self.reg_width(rn);
        let src2 = self.operand_to_src(op2, ctx)?;
        let cmp_op = if insn.mnemonic == Mnemonic::CCMN {
            OpKind::Add {
                dst: cmp_result,
                src1: self.arm_reg(rn),
                src2,
                width,
                flags: FlagUpdate::All,
            }
        } else {
            OpKind::Sub {
                dst: cmp_result,
                src1: self.arm_reg(rn),
                src2,
                width,
                flags: FlagUpdate::All,
            }
        };
        Self::push_lifted_op(ops, pc, cmp_op);

        let cmp_nzcv = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Mov {
                dst: cmp_nzcv,
                src: SrcOperand::Reg(VReg::Arch(ArchReg::Arm(ArmReg::Nzcv))),
                width: OpWidth::W32,
            },
        );

        let final_nzcv = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Select {
                dst: final_nzcv,
                cond: cond_reg,
                src_true: cmp_nzcv,
                src_false: VReg::Imm((nzcv.value & 0xF) << 28),
                width: OpWidth::W32,
            },
        );
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(ArmReg::Nzcv)),
                src: SrcOperand::Reg(final_nzcv),
                width: OpWidth::W32,
            },
        );

        Ok(())
    }

    fn lift_axflag(&self, pc: u64, ops: &mut Vec<SmirOp>, ctx: &mut LiftContext) {
        let nzcv = VReg::Arch(ArchReg::Arm(ArmReg::Nzcv));

        let v_to_z = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Shl {
                dst: v_to_z,
                src: nzcv,
                amount: SrcOperand::Imm(2),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let z_or_v = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: z_or_v,
                src1: nzcv,
                src2: SrcOperand::Reg(v_to_z),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let z_bit = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: z_bit,
                src1: z_or_v,
                src2: SrcOperand::Imm(NZCV_Z),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let v_to_c = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Shl {
                dst: v_to_c,
                src: nzcv,
                amount: SrcOperand::Imm(1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let c_raw = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: c_raw,
                src1: nzcv,
                src2: SrcOperand::Imm(NZCV_C),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let c_bit = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::AndNot {
                dst: c_bit,
                src1: c_raw,
                src2: SrcOperand::Reg(v_to_c),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let result = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: result,
                src1: z_bit,
                src2: SrcOperand::Reg(c_bit),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Mov {
                dst: nzcv,
                src: SrcOperand::Reg(result),
                width: OpWidth::W32,
            },
        );
    }

    fn lift_mrs(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rd, sysreg) = match (insn.operands.get(0), insn.operands.get(1)) {
            (Some(Operand::Reg(rd)), Some(Operand::SysReg(sysreg))) => (rd, sysreg),
            _ => return Err(LiftError::Internal("invalid MRS operands".to_string())),
        };
        let Some(access) = Self::supported_sysreg(*sysreg) else {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("MRS sysreg {sysreg:#06x}"),
            });
        };

        let masked = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: masked,
                src1: VReg::Arch(ArchReg::Arm(access.reg)),
                src2: SrcOperand::Imm(access.mask),
                width: access.read_width,
                flags: FlagUpdate::None,
            },
        );
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Mov {
                dst: self.dst_reg(rd, ctx),
                src: SrcOperand::Reg(masked),
                width: OpWidth::W64,
            },
        );
        Ok(())
    }

    fn lift_msr(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (sysreg, rt) = match (insn.operands.get(0), insn.operands.get(1)) {
            (Some(Operand::SysReg(sysreg)), Some(Operand::Reg(rt))) => (sysreg, rt),
            _ => return Err(LiftError::Internal("invalid MSR operands".to_string())),
        };
        let Some(access) = Self::supported_sysreg(*sysreg) else {
            return Err(LiftError::Unsupported {
                addr: pc,
                mnemonic: format!("MSR sysreg {sysreg:#06x}"),
            });
        };

        let masked = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: masked,
                src1: self.arm_reg(rt),
                src2: SrcOperand::Imm(access.mask),
                width: OpWidth::W64,
                flags: FlagUpdate::None,
            },
        );
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(access.reg)),
                src: SrcOperand::Reg(masked),
                width: access.write_width,
            },
        );
        Ok(())
    }

    fn lift_xaflag(&self, pc: u64, ops: &mut Vec<SmirOp>, ctx: &mut LiftContext) {
        let nzcv = VReg::Arch(ArchReg::Arm(ArmReg::Nzcv));

        let shl1 = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Shl {
                dst: shl1,
                src: nzcv,
                amount: SrcOperand::Imm(1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let shl2 = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Shl {
                dst: shl2,
                src: nzcv,
                amount: SrcOperand::Imm(2),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let has_c_or_z_as_n = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: has_c_or_z_as_n,
                src1: shl1,
                src2: SrcOperand::Reg(shl2),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let n_bit = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::AndNot {
                dst: n_bit,
                src1: VReg::Imm(NZCV_N),
                src2: SrcOperand::Reg(has_c_or_z_as_n),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let z_raw = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: z_raw,
                src1: nzcv,
                src2: SrcOperand::Imm(NZCV_Z),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let z_bit = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: z_bit,
                src1: z_raw,
                src2: SrcOperand::Reg(shl1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let shr1 = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Shr {
                dst: shr1,
                src: nzcv,
                amount: SrcOperand::Imm(1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let c_or_z = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: c_or_z,
                src1: nzcv,
                src2: SrcOperand::Reg(shr1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let c_bit = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: c_bit,
                src1: c_or_z,
                src2: SrcOperand::Imm(NZCV_C),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let shr2 = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Shr {
                dst: shr2,
                src: nzcv,
                amount: SrcOperand::Imm(2),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let v_unmasked = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::AndNot {
                dst: v_unmasked,
                src1: shr2,
                src2: SrcOperand::Reg(shr1),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let v_bit = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::And {
                dst: v_bit,
                src1: v_unmasked,
                src2: SrcOperand::Imm(NZCV_V),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let nz = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: nz,
                src1: n_bit,
                src2: SrcOperand::Reg(z_bit),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let cv = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: cv,
                src1: c_bit,
                src2: SrcOperand::Reg(v_bit),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        let result = ctx.alloc_vreg();
        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Or {
                dst: result,
                src1: nz,
                src2: SrcOperand::Reg(cv),
                width: OpWidth::W32,
                flags: FlagUpdate::None,
            },
        );

        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Mov {
                dst: nzcv,
                src: SrcOperand::Reg(result),
                width: OpWidth::W32,
            },
        );
    }

    fn lift_cond_select(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let invalid = || LiftError::Internal("invalid conditional select operands".to_string());

        // Alias mnemonics keep the raw condition from the canonical CS* encoding.
        let (rd, src_true, src_false_base, false_op, cond) = match insn.mnemonic {
            Mnemonic::CSEL | Mnemonic::CSINC | Mnemonic::CSINV | Mnemonic::CSNEG => {
                let (rd, rn, rm, cond) = match (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                    insn.operands.get(3),
                ) {
                    (
                        Some(Operand::Reg(rd)),
                        Some(Operand::Reg(rn)),
                        Some(Operand::Reg(rm)),
                        Some(Operand::Cond(cond)),
                    ) => (*rd, *rn, *rm, *cond),
                    _ => return Err(invalid()),
                };
                let false_op = match insn.mnemonic {
                    Mnemonic::CSEL => CondSelectFalseOp::Identity,
                    Mnemonic::CSINC => CondSelectFalseOp::Increment,
                    Mnemonic::CSINV => CondSelectFalseOp::Invert,
                    Mnemonic::CSNEG => CondSelectFalseOp::Negate,
                    _ => unreachable!(),
                };
                (rd, self.arm_reg(&rn), self.arm_reg(&rm), false_op, cond)
            }
            Mnemonic::CINC | Mnemonic::CINV | Mnemonic::CNEG => {
                let (rd, rn, cond) = match (
                    insn.operands.get(0),
                    insn.operands.get(1),
                    insn.operands.get(2),
                ) {
                    (Some(Operand::Reg(rd)), Some(Operand::Reg(rn)), Some(Operand::Cond(cond))) => {
                        (*rd, *rn, *cond)
                    }
                    _ => return Err(invalid()),
                };
                let false_op = match insn.mnemonic {
                    Mnemonic::CINC => CondSelectFalseOp::Increment,
                    Mnemonic::CINV => CondSelectFalseOp::Invert,
                    Mnemonic::CNEG => CondSelectFalseOp::Negate,
                    _ => unreachable!(),
                };
                (rd, self.arm_reg(&rn), self.arm_reg(&rn), false_op, cond)
            }
            Mnemonic::CSET | Mnemonic::CSETM => {
                let (rd, cond) = match (insn.operands.get(0), insn.operands.get(1)) {
                    (Some(Operand::Reg(rd)), Some(Operand::Cond(cond))) => (*rd, *cond),
                    _ => return Err(invalid()),
                };
                let false_op = if insn.mnemonic == Mnemonic::CSET {
                    CondSelectFalseOp::Increment
                } else {
                    CondSelectFalseOp::Invert
                };
                (rd, VReg::Imm(0), VReg::Imm(0), false_op, cond)
            }
            _ => return Err(invalid()),
        };

        let dst = self.dst_reg(&rd, ctx);
        let width = self.reg_width(&rd);
        let cmp = ctx.alloc_vreg();

        Self::push_lifted_op(
            ops,
            pc,
            OpKind::TestCondition {
                dst: cmp,
                cond: self.arm_cond(cond),
            },
        );

        let src_false = match false_op {
            CondSelectFalseOp::Identity => src_false_base,
            CondSelectFalseOp::Increment => {
                let tmp = ctx.alloc_vreg();
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Add {
                        dst: tmp,
                        src1: src_false_base,
                        src2: SrcOperand::Imm(1),
                        width,
                        flags: FlagUpdate::None,
                    },
                );
                tmp
            }
            CondSelectFalseOp::Invert => {
                let tmp = ctx.alloc_vreg();
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Not {
                        dst: tmp,
                        src: src_false_base,
                        width,
                    },
                );
                tmp
            }
            CondSelectFalseOp::Negate => {
                let tmp = ctx.alloc_vreg();
                Self::push_lifted_op(
                    ops,
                    pc,
                    OpKind::Neg {
                        dst: tmp,
                        src: src_false_base,
                        width,
                        flags: FlagUpdate::None,
                    },
                );
                tmp
            }
        };

        Self::push_lifted_op(
            ops,
            pc,
            OpKind::Select {
                dst,
                cond: cmp,
                src_true,
                src_false,
                width,
            },
        );

        Ok(())
    }

    fn lift_extend(
        &self,
        insn: &DecodedInsn,
        from_width: OpWidth,
        signed: bool,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        if let (Some(Operand::Reg(rd)), Some(Operand::Reg(rn))) =
            (insn.operands.get(0), insn.operands.get(1))
        {
            let dst = self.dst_reg(rd, ctx);
            let to_width = self.reg_width(rd);

            let kind = if signed {
                OpKind::SignExtend {
                    dst,
                    src: self.arm_reg(rn),
                    from_width,
                    to_width,
                }
            } else {
                OpKind::ZeroExtend {
                    dst,
                    src: self.arm_reg(rn),
                    from_width,
                    to_width,
                }
            };

            ops.push(SmirOp::new(OpId(ops.len() as u16), pc, kind));
        }

        Ok(())
    }

    fn lift_load(
        &self,
        insn: &DecodedInsn,
        width: MemWidth,
        extend: SignExtend,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rd, mem) = match (insn.operands.get(0), insn.operands.get(1)) {
            (Some(Operand::Reg(r)), Some(Operand::Mem(m))) => (r, m),
            (Some(Operand::Reg(r)), Some(Operand::Label(off))) => {
                let dst = self.dst_reg(r, ctx);
                let load_dst = if extend == SignExtend::Sign && !r.is_64bit {
                    ctx.alloc_vreg()
                } else {
                    dst
                };

                ops.push(SmirOp::new(
                    OpId(ops.len() as u16),
                    pc,
                    OpKind::Load {
                        dst: load_dst,
                        addr: Address::PcRel {
                            offset: *off,
                            disp_size: DispSize::Auto,
                            base: None,
                        },
                        width,
                        sign: extend,
                    },
                ));
                if load_dst != dst {
                    ops.push(SmirOp::new(
                        OpId(ops.len() as u16),
                        pc,
                        OpKind::ZeroExtend {
                            dst,
                            src: load_dst,
                            from_width: OpWidth::W32,
                            to_width: OpWidth::W64,
                        },
                    ));
                }
                return Ok(());
            }
            _ => return Err(LiftError::Internal("invalid load operands".to_string())),
        };

        let dst = self.dst_reg(rd, ctx);
        let load_dst = if extend == SignExtend::Sign && !rd.is_64bit {
            ctx.alloc_vreg()
        } else {
            dst
        };
        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);

        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        if mem.mode == AddressingMode::PreIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        let load_addr = self.indexed_access_addr(mem, addr);

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Load {
                dst: load_dst,
                addr: load_addr,
                width,
                sign: extend,
            },
        ));
        if load_dst != dst {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::ZeroExtend {
                    dst,
                    src: load_dst,
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        }

        if mem.mode == AddressingMode::PostIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        Ok(())
    }

    fn lift_store(
        &self,
        insn: &DecodedInsn,
        width: MemWidth,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rt, mem) = match (insn.operands.get(0), insn.operands.get(1)) {
            (Some(Operand::Reg(r)), Some(Operand::Mem(m))) => (r, m),
            _ => return Err(LiftError::Internal("invalid store operands".to_string())),
        };

        let src = self.arm_reg(rt);
        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);

        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        if mem.mode == AddressingMode::PreIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        let store_addr = self.indexed_access_addr(mem, addr);

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Store {
                src,
                addr: store_addr,
                width,
            },
        ));

        if mem.mode == AddressingMode::PostIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        Ok(())
    }

    fn lift_load_exclusive(
        &self,
        insn: &DecodedInsn,
        width: MemWidth,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rd, mem) = match (insn.operands.get(0), insn.operands.get(1)) {
            (Some(Operand::Reg(r)), Some(Operand::Mem(m))) => (r, m),
            _ => {
                return Err(LiftError::Internal(
                    "invalid load-exclusive operands".to_string(),
                ));
            }
        };

        let dst = self.dst_reg(rd, ctx);
        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);

        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        let load_addr = self.indexed_access_addr(mem, addr);

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::LoadExclusive {
                dst,
                addr: load_addr,
                width,
            },
        ));

        Ok(())
    }

    fn lift_store_exclusive(
        &self,
        insn: &DecodedInsn,
        width: MemWidth,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (status, src, mem) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            (Some(Operand::Reg(status)), Some(Operand::Reg(src)), Some(Operand::Mem(mem))) => {
                (status, src, mem)
            }
            _ => {
                return Err(LiftError::Internal(
                    "invalid store-exclusive operands".to_string(),
                ));
            }
        };

        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);
        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        let store_addr = self.indexed_access_addr(mem, addr);

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::StoreExclusive {
                status: self.dst_reg(status, ctx),
                src: self.arm_reg(src),
                addr: store_addr,
                width,
            },
        ));

        Ok(())
    }

    fn lift_atomic_rmw(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rs, rt, mem) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            (Some(Operand::Reg(rs)), Some(Operand::Reg(rt)), Some(Operand::Mem(m))) => (rs, rt, m),
            _ => {
                return Err(LiftError::Internal(
                    "invalid atomic RMW operands".to_string(),
                ));
            }
        };

        let width = match (insn.raw >> 30) & 0x3 {
            0 => MemWidth::B1,
            1 => MemWidth::B2,
            2 => MemWidth::B4,
            _ => MemWidth::B8,
        };
        let order = match (((insn.raw >> 23) & 1) != 0, ((insn.raw >> 22) & 1) != 0) {
            (false, false) => MemoryOrder::Relaxed,
            (true, false) => MemoryOrder::Acquire,
            (false, true) => MemoryOrder::Release,
            (true, true) => MemoryOrder::AcqRel,
        };
        let atomic_op = match insn.mnemonic {
            Mnemonic::SWP | Mnemonic::SWPA | Mnemonic::SWPAL | Mnemonic::SWPL => AtomicOp::Swap,
            Mnemonic::LDADD | Mnemonic::LDADDA | Mnemonic::LDADDAL | Mnemonic::LDADDL => {
                AtomicOp::Add
            }
            Mnemonic::LDCLR => AtomicOp::And,
            Mnemonic::LDEOR => AtomicOp::Xor,
            Mnemonic::LDSET => AtomicOp::Or,
            Mnemonic::LDSMAX => AtomicOp::Max,
            Mnemonic::LDSMIN => AtomicOp::Min,
            Mnemonic::LDUMAX => AtomicOp::Umax,
            Mnemonic::LDUMIN => AtomicOp::Umin,
            _ => unreachable!(),
        };

        let dst = self.dst_reg(rt, ctx);
        let mut src = self.arm_reg(rs);
        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);

        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        if insn.mnemonic == Mnemonic::LDCLR {
            let inverted = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Not {
                    dst: inverted,
                    src,
                    width: OpWidth::W64,
                },
            ));
            src = inverted;
        }

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::AtomicRmw {
                dst,
                addr: self.indexed_access_addr(mem, addr),
                src,
                op: atomic_op,
                width,
                order,
            },
        ));

        Ok(())
    }

    fn lift_cas(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (compare, new_val, mem) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            (Some(Operand::Reg(compare)), Some(Operand::Reg(new_val)), Some(Operand::Mem(mem))) => {
                (compare, new_val, mem)
            }
            _ => return Err(LiftError::Internal("invalid CAS operands".to_string())),
        };

        let width = match (insn.raw >> 30) & 0x3 {
            0 => MemWidth::B1,
            1 => MemWidth::B2,
            2 => MemWidth::B4,
            _ => MemWidth::B8,
        };
        let order = match (((insn.raw >> 22) & 1) != 0, ((insn.raw >> 15) & 1) != 0) {
            (false, false) => MemoryOrder::Relaxed,
            (true, false) => MemoryOrder::Acquire,
            (false, true) => MemoryOrder::Release,
            (true, true) => MemoryOrder::AcqRel,
        };

        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);
        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::Cas {
                dst: self.dst_reg(compare, ctx),
                success: ctx.alloc_vreg(),
                addr: self.indexed_access_addr(mem, addr),
                expected: self.arm_reg(compare),
                new_val: self.arm_reg(new_val),
                width,
                order,
            },
        ));

        Ok(())
    }

    fn lift_load_pair(
        &self,
        insn: &DecodedInsn,
        extend: SignExtend,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rt1, rt2, mem) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            (Some(Operand::Reg(r1)), Some(Operand::Reg(r2)), Some(Operand::Mem(m))) => (r1, r2, m),
            _ => return Err(LiftError::Internal("invalid LDP operands".to_string())),
        };

        let dst1 = self.dst_reg(rt1, ctx);
        let dst2 = self.dst_reg(rt2, ctx);
        let width = if rt1.is_64bit {
            MemWidth::B8
        } else {
            MemWidth::B4
        };
        let offset2 = if rt1.is_64bit { 8i64 } else { 4i64 };

        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);

        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        if mem.mode == AddressingMode::PreIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        let load_addr = self.indexed_access_addr(mem, addr);

        if extend == SignExtend::Zero {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::LoadPair {
                    dst1,
                    dst2,
                    addr: load_addr,
                    width,
                },
            ));
        } else {
            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: dst1,
                    addr: load_addr.clone(),
                    width,
                    sign: extend,
                },
            ));

            let addr2 = match &load_addr {
                Address::Direct(base) => Address::BaseOffset {
                    base: *base,
                    offset: offset2,
                    disp_size: DispSize::Auto,
                },
                Address::BaseOffset {
                    base,
                    offset,
                    disp_size,
                } => Address::BaseOffset {
                    base: *base,
                    offset: *offset + offset2,
                    disp_size: *disp_size,
                },
                _ => load_addr,
            };

            ops.push(SmirOp::new(
                OpId(ops.len() as u16),
                pc,
                OpKind::Load {
                    dst: dst2,
                    addr: addr2,
                    width,
                    sign: extend,
                },
            ));
        }

        if mem.mode == AddressingMode::PostIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        Ok(())
    }

    fn lift_store_pair(
        &self,
        insn: &DecodedInsn,
        pc: u64,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
    ) -> Result<(), LiftError> {
        let (rt1, rt2, mem) = match (
            insn.operands.get(0),
            insn.operands.get(1),
            insn.operands.get(2),
        ) {
            (Some(Operand::Reg(r1)), Some(Operand::Reg(r2)), Some(Operand::Mem(m))) => (r1, r2, m),
            _ => return Err(LiftError::Internal("invalid STP operands".to_string())),
        };

        let src1 = self.arm_reg(rt1);
        let src2 = self.arm_reg(rt2);
        let width = if rt1.is_64bit {
            MemWidth::B8
        } else {
            MemWidth::B4
        };

        let (addr, pre_ops) = self.mem_to_addr(mem, ctx);

        for mut op in pre_ops {
            op.id = OpId(ops.len() as u16);
            ops.push(op);
        }

        if mem.mode == AddressingMode::PreIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        let store_addr = self.indexed_access_addr(mem, addr);

        ops.push(SmirOp::new(
            OpId(ops.len() as u16),
            pc,
            OpKind::StorePair {
                src1,
                src2,
                addr: store_addr,
                width,
            },
        ));

        if mem.mode == AddressingMode::PostIndex {
            self.handle_writeback(mem, pc, ops, ctx);
        }

        Ok(())
    }
}

impl Default for Aarch64Lifter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SmirLifter Implementation
// ============================================================================

impl crate::smir::lift::SmirLifter for Aarch64Lifter {
    fn source_arch(&self) -> SourceArch {
        SourceArch::Aarch64
    }

    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        use crate::arm::decoder::aarch64::Aarch64Decoder;

        if bytes.len() < 4 {
            return Err(LiftError::Incomplete {
                addr,
                have: bytes.len(),
                need: 4,
            });
        }

        // Decode the 32-bit instruction
        let raw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let insn = Aarch64Decoder::decode(raw).map_err(|_| LiftError::InvalidEncoding {
            addr,
            bytes: bytes[..4].to_vec(),
        })?;

        ctx.guest_pc = addr;
        let (ops, control_flow) = self.lift_insn_inner(&insn, addr, ctx)?;

        let mut branch_targets = Vec::new();
        match &control_flow {
            ControlFlow::Branch { target } => {
                branch_targets.push(*target);
            }
            ControlFlow::CondBranch {
                target,
                fallthrough,
                ..
            } => {
                branch_targets.push(*target);
                branch_targets.push(*fallthrough);
            }
            ControlFlow::CondBranchReg {
                taken, not_taken, ..
            } => {
                branch_targets.push(*taken);
                branch_targets.push(*not_taken);
            }
            ControlFlow::Call {
                target: CallTarget::GuestAddr(target),
            } => {
                branch_targets.push(*target);
            }
            _ => {}
        }

        Ok(LiftResult {
            ops,
            bytes_consumed: 4,
            control_flow,
            branch_targets,
        })
    }

    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError> {
        let block_id = ctx.get_or_create_block(addr);
        let mut all_ops = Vec::new();
        let mut current_addr = addr;

        loop {
            let bytes = mem
                .read(current_addr, 4)
                .map_err(|e| LiftError::MemoryError {
                    addr: current_addr,
                    error: e,
                })?;

            let result = self.lift_insn(current_addr, &bytes, ctx)?;
            all_ops.extend(result.ops);
            current_addr += result.bytes_consumed as u64;

            if result.control_flow.ends_block() {
                let terminator = match result.control_flow {
                    ControlFlow::Fallthrough | ControlFlow::NextInsn => unreachable!(),
                    ControlFlow::Branch { target } | ControlFlow::DirectBranch(target) => {
                        Terminator::Branch {
                            target: ctx.get_or_create_block(target),
                        }
                    }
                    ControlFlow::CondBranch {
                        cond,
                        target,
                        fallthrough,
                    } => {
                        let cond_vreg = ctx.alloc_vreg();
                        all_ops.push(SmirOp::new(
                            OpId(all_ops.len() as u16),
                            current_addr - result.bytes_consumed as u64,
                            OpKind::TestCondition {
                                dst: cond_vreg,
                                cond,
                            },
                        ));
                        Terminator::CondBranch {
                            cond: cond_vreg,
                            true_target: ctx.get_or_create_block(target),
                            false_target: ctx.get_or_create_block(fallthrough),
                        }
                    }
                    ControlFlow::CondBranchReg {
                        cond,
                        taken,
                        not_taken,
                    } => Terminator::CondBranch {
                        cond,
                        true_target: ctx.get_or_create_block(taken),
                        false_target: ctx.get_or_create_block(not_taken),
                    },
                    ControlFlow::IndirectBranch { target } => Terminator::IndirectBranch {
                        target,
                        possible_targets: vec![],
                    },
                    ControlFlow::IndirectBranchMem { addr } => Terminator::IndirectBranchMem {
                        addr,
                        possible_targets: vec![],
                    },
                    ControlFlow::Call { target } => Terminator::Call {
                        target,
                        args: vec![],
                        continuation: ctx.get_or_create_block(current_addr),
                    },
                    ControlFlow::Return => Terminator::Return { values: vec![] },
                    ControlFlow::Trap { kind } => Terminator::Trap { kind },
                    ControlFlow::Syscall => Terminator::Trap {
                        kind: TrapKind::SystemCall,
                    },
                };

                return Ok(SmirBlock {
                    id: block_id,
                    guest_pc: addr,
                    phis: vec![],
                    ops: all_ops,
                    terminator,
                    exec_count: 0,
                });
            }
        }
    }

    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError> {
        let func_id = FunctionId(ctx.known_functions.len() as u32);
        ctx.known_functions.insert(entry, func_id);

        let mut blocks = Vec::new();
        let mut worklist = vec![entry];
        let mut visited = HashSet::new();
        let mut min_addr = entry;
        let mut max_addr = entry;

        while let Some(addr) = worklist.pop() {
            if visited.contains(&addr) {
                continue;
            }
            visited.insert(addr);

            let block = self.lift_block(addr, mem, ctx)?;

            if block.guest_pc < min_addr {
                min_addr = block.guest_pc;
            }
            let block_end = block.guest_pc + (block.ops.len() * 4) as u64;
            if block_end > max_addr {
                max_addr = block_end;
            }

            for succ in block.successors() {
                if let Some(&succ_addr) = ctx
                    .block_cache
                    .iter()
                    .find(|(_, id)| **id == succ)
                    .map(|(addr, _)| addr)
                {
                    if !visited.contains(&succ_addr) {
                        worklist.push(succ_addr);
                    }
                }
            }

            blocks.push(block);
        }

        Ok(SmirFunction {
            id: func_id,
            entry: ctx.get_or_create_block(entry),
            blocks,
            locals: vec![],
            guest_range: (min_addr, max_addr),
            calling_convention: CallingConv::Aarch64Aapcs,
            attrs: FunctionAttrs::default(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smir::lift::SmirLifter;

    struct MockMemory {
        data: Vec<u8>,
        base: GuestAddr,
    }

    impl MemoryReader for MockMemory {
        fn read(&self, addr: GuestAddr, size: usize) -> Result<Vec<u8>, MemoryError> {
            let offset = (addr - self.base) as usize;
            if offset + size > self.data.len() {
                return Err(MemoryError::OutOfBounds { addr });
            }
            Ok(self.data[offset..offset + size].to_vec())
        }
    }

    #[test]
    fn test_aarch64_lifter_add() {
        let mut lifter = Aarch64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::Aarch64);

        // ADD X0, X1, X2 => 0x8b020020
        let bytes = [0x20, 0x00, 0x02, 0x8b];
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert!(!result.ops.is_empty());
        match &result.ops[0].kind {
            OpKind::Add { width, .. } => {
                assert_eq!(*width, OpWidth::W64);
            }
            _ => panic!("Expected Add operation"),
        }
    }

    #[test]
    fn test_aarch64_lifter_mov_imm() {
        let mut lifter = Aarch64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::Aarch64);

        // MOV X0, #0x1234 => MOVZ X0, #0x1234 => 0xd2824680
        let bytes = [0x80, 0x46, 0x82, 0xd2];
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert!(!result.ops.is_empty());
    }

    #[test]
    fn test_aarch64_lifter_branch() {
        let mut lifter = Aarch64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::Aarch64);

        // B #0x10 => 0x14000004
        let bytes = [0x04, 0x00, 0x00, 0x14];
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        match result.control_flow {
            ControlFlow::Branch { target } => {
                assert_eq!(target, 0x1010);
            }
            _ => panic!("Expected Branch control flow"),
        }
    }

    #[test]
    fn test_lift_block_cond_branch_defines_condition() {
        let mut lifter = Aarch64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::Aarch64);

        // B.EQ #8 => 0x54000040
        let mem = MockMemory {
            data: vec![0x40, 0x00, 0x00, 0x54],
            base: 0x1000,
        };
        let block = lifter.lift_block(0x1000, &mem, &mut ctx).unwrap();

        let cond_def = block
            .ops
            .last()
            .expect("conditional branch should define a condition vreg");
        let OpKind::TestCondition {
            dst,
            cond: Condition::Eq,
        } = &cond_def.kind
        else {
            panic!("expected TestCondition Eq, got {:?}", cond_def.kind);
        };
        let Terminator::CondBranch { cond, .. } = block.terminator else {
            panic!("expected conditional branch terminator");
        };
        assert_eq!(cond, *dst);
    }

    #[test]
    fn test_lift_context_aarch64() {
        let ctx = LiftContext::new(SourceArch::Aarch64);
        assert_eq!(ctx.endian, Endian::Little);
    }

    fn lift_single(bytes: [u8; 4]) -> (Vec<SmirOp>, ControlFlow) {
        let mut lifter = Aarch64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::Aarch64);
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();
        (result.ops, result.control_flow)
    }

    #[test]
    fn test_lift_fadd_scalar() {
        let (ops, _) = lift_single([0x20, 0x28, 0x22, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FAdd { precision, .. } => assert_eq!(*precision, FpPrecision::F32),
            other => panic!("expected FAdd, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fadd_scalar_d() {
        let (ops, _) = lift_single([0x20, 0x18, 0x62, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FAdd { precision, .. } => assert_eq!(*precision, FpPrecision::F64),
            other => panic!("expected FAdd F64, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fsub_scalar() {
        let (ops, _) = lift_single([0x20, 0x58, 0x22, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FSub { .. } => {}
            other => panic!("expected FSub, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fdiv_scalar() {
        let (ops, _) = lift_single([0x20, 0x68, 0x22, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FDiv { .. } => {}
            other => panic!("expected FDiv, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fabs_scalar() {
        let (ops, _) = lift_single([0x20, 0x40, 0x21, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FAbs { precision, .. } => assert_eq!(*precision, FpPrecision::F32),
            other => panic!("expected FAbs, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fneg_scalar() {
        let (ops, _) = lift_single([0x20, 0x40, 0x24, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FNeg { .. } => {}
            other => panic!("expected FNeg, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fsqrt_scalar() {
        let (ops, _) = lift_single([0x20, 0xc0, 0x21, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FSqrt { .. } => {}
            other => panic!("expected FSqrt, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fmadd_scalar() {
        let (ops, _) = lift_single([0x20, 0x0c, 0x02, 0x1f]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FFma { precision, .. } => assert_eq!(*precision, FpPrecision::F32),
            other => panic!("expected FFma, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fcmp_scalar() {
        let (ops, _) = lift_single([0x20, 0x1c, 0x22, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FCmp { precision, .. } => assert_eq!(*precision, FpPrecision::F32),
            other => panic!("expected FCmp, got {:?}", other),
        }
    }

    #[test]
    fn test_lift_fcvt_s_to_d() {
        let (ops, _) = lift_single([0x20, 0xc0, 0x22, 0x1e]);
        assert_eq!(ops.len(), 1);
        match &ops[0].kind {
            OpKind::FConvert { from, to, .. } => {
                assert_eq!(*from, FpPrecision::F32);
                assert_eq!(*to, FpPrecision::F64);
            }
            other => panic!("expected FConvert S->D, got {:?}", other),
        }
    }
}
