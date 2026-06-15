//! RISC-V instruction lifter for SMIR.
//!
//! This module lifts RISC-V instructions to SMIR operations.
//! Supports RV64I base, M (multiply/divide), A (atomics), and C (compressed) extensions.

use crate::riscv::{Isa as RvIsa, Op as RvOp, Xlen as RvXlen, decode as rv_decode};
use crate::smir::flags::FlagUpdate;
use crate::smir::ir::{SmirBlock, SmirFunction, Terminator};
use crate::smir::ops::{OpKind, SmirOp};
use crate::smir::types::*;

use super::{ControlFlow, LiftContext, LiftError, LiftResult, MemoryReader, SmirLifter};

/// A term in a SHA/SM3 xor-fold: a rotate / shift / identity of the source.
#[derive(Clone, Copy)]
enum CryptoTerm {
    /// ror, 32-bit width
    R,
    /// ror, 64-bit width
    RW,
    /// rol, 32-bit width
    L,
    /// logical shift right, 32-bit
    S,
    /// logical shift right, 64-bit
    SW,
    /// identity (use the source directly)
    X,
}

// ============================================================================
// RISC-V Extensions Configuration
// ============================================================================

/// RISC-V extension configuration
#[derive(Clone, Copy, Debug, Default)]
pub struct RiscVExtensions {
    /// M extension: Integer multiplication and division
    pub m: bool,
    /// A extension: Atomic instructions
    pub a: bool,
    /// F extension: Single-precision floating-point
    pub f: bool,
    /// D extension: Double-precision floating-point
    pub d: bool,
    /// C extension: Compressed instructions
    pub c: bool,
    /// Zba extension: Address bit manipulation
    pub zba: bool,
    /// Zbb extension: Basic bit manipulation
    pub zbb: bool,
}

impl RiscVExtensions {
    /// Standard RV64GC configuration (I + M + A + F + D + C)
    pub fn rv64gc() -> Self {
        Self {
            m: true,
            a: true,
            f: true,
            d: true,
            c: true,
            zba: false,
            zbb: false,
        }
    }

    /// Minimal RV64I configuration
    pub fn rv64i() -> Self {
        Self::default()
    }

    /// RV64IMAC (common embedded configuration)
    pub fn rv64imac() -> Self {
        Self {
            m: true,
            a: true,
            c: true,
            ..Default::default()
        }
    }
}

// ============================================================================
// RISC-V Lifter
// ============================================================================

/// RISC-V instruction lifter
pub struct RiscVLifter {
    /// Register width (32 or 64)
    xlen: u8,
    /// Enabled extensions
    extensions: RiscVExtensions,
}

impl RiscVLifter {
    /// Create a new RV64 lifter with specified extensions
    pub fn new_rv64(extensions: RiscVExtensions) -> Self {
        Self {
            xlen: 64,
            extensions,
        }
    }

    /// Create a new RV32 lifter with specified extensions
    pub fn new_rv32(extensions: RiscVExtensions) -> Self {
        Self {
            xlen: 32,
            extensions,
        }
    }

    /// Create a standard RV64GC lifter
    pub fn rv64gc() -> Self {
        Self::new_rv64(RiscVExtensions::rv64gc())
    }

    /// Get the operation width for this XLEN
    fn op_width(&self) -> OpWidth {
        if self.xlen == 64 {
            OpWidth::W64
        } else {
            OpWidth::W32
        }
    }

    /// Get a VReg for an integer register (x0 returns Imm(0))
    fn get_x_reg(&self, reg: u8, ctx: &mut LiftContext) -> VReg {
        if reg == 0 {
            VReg::Imm(0)
        } else {
            ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::X(reg)))
        }
    }

    /// Define a new value for an integer register (x0 writes are ignored)
    fn def_x_reg(&self, reg: u8, ctx: &mut LiftContext) -> Option<VReg> {
        if reg == 0 {
            None
        } else {
            Some(ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::X(reg))))
        }
    }

    /// Get the PC register
    fn get_pc(&self, ctx: &mut LiftContext) -> VReg {
        ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::Pc))
    }

    /// Define a new PC value
    fn def_pc(&self, ctx: &mut LiftContext) -> VReg {
        ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::Pc))
    }

    // ========================================================================
    // Instruction Format Extraction
    // ========================================================================

    /// Extract rd field (bits 11:7)
    fn rd(insn: u32) -> u8 {
        ((insn >> 7) & 0x1F) as u8
    }

    /// Extract rs1 field (bits 19:15)
    fn rs1(insn: u32) -> u8 {
        ((insn >> 15) & 0x1F) as u8
    }

    /// Extract rs2 field (bits 24:20)
    fn rs2(insn: u32) -> u8 {
        ((insn >> 20) & 0x1F) as u8
    }

    /// Extract funct3 field (bits 14:12)
    fn funct3(insn: u32) -> u8 {
        ((insn >> 12) & 0x7) as u8
    }

    /// Extract funct7 field (bits 31:25)
    fn funct7(insn: u32) -> u8 {
        ((insn >> 25) & 0x7F) as u8
    }

    /// Extract I-type immediate (bits 31:20, sign-extended)
    fn imm_i(insn: u32) -> i64 {
        ((insn as i32) >> 20) as i64
    }

    /// Extract S-type immediate (bits 31:25 | 11:7, sign-extended)
    fn imm_s(insn: u32) -> i64 {
        let hi = ((insn >> 25) & 0x7F) as i32;
        let lo = ((insn >> 7) & 0x1F) as i32;
        let imm = (hi << 5) | lo;
        // Sign-extend from bit 11
        ((imm << 20) >> 20) as i64
    }

    /// Extract B-type immediate (bits 31|7|30:25|11:8, sign-extended, shifted left by 1)
    fn imm_b(insn: u32) -> i64 {
        let bit12 = ((insn >> 31) & 1) as i32;
        let bit11 = ((insn >> 7) & 1) as i32;
        let bits10_5 = ((insn >> 25) & 0x3F) as i32;
        let bits4_1 = ((insn >> 8) & 0xF) as i32;
        let imm = (bit12 << 12) | (bit11 << 11) | (bits10_5 << 5) | (bits4_1 << 1);
        // Sign-extend from bit 12
        ((imm << 19) >> 19) as i64
    }

    /// Extract U-type immediate (bits 31:12, shifted left by 12)
    fn imm_u(insn: u32) -> i64 {
        ((insn & 0xFFFF_F000) as i32) as i64
    }

    /// Extract J-type immediate (bits 31|19:12|20|30:21, sign-extended, shifted left by 1)
    fn imm_j(insn: u32) -> i64 {
        let bit20 = ((insn >> 31) & 1) as i32;
        let bits19_12 = ((insn >> 12) & 0xFF) as i32;
        let bit11 = ((insn >> 20) & 1) as i32;
        let bits10_1 = ((insn >> 21) & 0x3FF) as i32;
        let imm = (bit20 << 20) | (bits19_12 << 12) | (bit11 << 11) | (bits10_1 << 1);
        // Sign-extend from bit 20
        ((imm << 11) >> 11) as i64
    }

    // ========================================================================
    // Instruction Lifting
    // ========================================================================

    /// Lift a single 32-bit RISC-V instruction
    fn lift_insn32(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let opcode = insn & 0x7F;

        match opcode {
            0x37 => self.lift_lui(insn, addr, ctx),
            0x17 => self.lift_auipc(insn, addr, ctx),
            0x6F => self.lift_jal(insn, addr, ctx),
            0x67 => self.lift_jalr(insn, addr, ctx),
            0x63 => self.lift_branch(insn, addr, ctx),
            0x03 => self.lift_load(insn, addr, ctx),
            0x23 => self.lift_store(insn, addr, ctx),
            0x13 => self.lift_op_imm(insn, addr, ctx),
            0x1B if self.xlen == 64 => self.lift_op_imm32(insn, addr, ctx),
            0x33 => self.lift_op(insn, addr, ctx),
            0x3B if self.xlen == 64 => self.lift_op32(insn, addr, ctx),
            0x0F => self.lift_fence(insn, addr, ctx),
            0x73 => self.lift_system(insn, addr, ctx),
            0x2F if self.extensions.a => self.lift_atomic(insn, addr, ctx),
            // FP load/store and OP-FP. Only the fflags/rounding-free ops (moves,
            // sign-inject, load/store) are lifted; arithmetic/convert/compare are
            // gaps until SMIR FP tracks fflags + NaN-boxing + frm.
            0x07 if self.extensions.f => self.lift_fp_ldst(insn, addr, ctx),
            0x27 if self.extensions.f => self.lift_fp_ldst(insn, addr, ctx),
            0x53 if self.extensions.f => self.lift_op_fp(insn, addr, ctx),
            // Fused multiply-add family (FMADD/FMSUB/FNMSUB/FNMADD .S/.D/.H).
            0x43 | 0x47 | 0x4b | 0x4f if self.extensions.f => self.lift_fp_fma(insn, addr, ctx),
            // OP-V (0x57): vector arithmetic + vset{i}vl{i} configuration. All
            // RVV ops route to the opaque RvVector engine.
            0x57 => self.lift_vector(insn, addr, ctx),
            _ => Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            }),
        }
    }

    /// LUI: Load Upper Immediate
    fn lift_lui(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let imm = Self::imm_u(insn);

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Imm(imm),
                    width: self.op_width(),
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// AUIPC: Add Upper Immediate to PC
    fn lift_auipc(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let imm = Self::imm_u(insn);
        let result = (addr as i64).wrapping_add(imm);

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Imm(result),
                    width: self.op_width(),
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// JAL: Jump and Link
    fn lift_jal(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let imm = Self::imm_j(insn);
        let target = (addr as i64).wrapping_add(imm) as u64;
        let return_addr = addr + 4;

        let mut ops = Vec::new();

        // Save return address to rd
        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Imm(return_addr as i64),
                    width: self.op_width(),
                },
            ));
        }

        Ok((ops, ControlFlow::DirectBranch(target)))
    }

    /// JALR: Jump and Link Register
    fn lift_jalr(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let imm = Self::imm_i(insn);
        let return_addr = addr + 4;

        let mut ops = Vec::new();

        // Compute target address: (rs1 + imm) & ~1
        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let target = ctx.alloc_vreg();

        if imm != 0 {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Add {
                    dst: target,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width: self.op_width(),
                    flags: FlagUpdate::None,
                },
            ));
        } else {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst: target,
                    src: SrcOperand::Reg(rs1),
                    width: self.op_width(),
                },
            ));
        }

        // Clear bit 0
        let target_aligned = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::And {
                dst: target_aligned,
                src1: target,
                src2: SrcOperand::Imm(!1i64),
                width: self.op_width(),
                flags: FlagUpdate::None,
            },
        ));

        // Save return address to rd
        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Imm(return_addr as i64),
                    width: self.op_width(),
                },
            ));
        }

        Ok((
            ops,
            ControlFlow::IndirectBranch {
                target: target_aligned,
            },
        ))
    }

    /// Branch instructions (BEQ, BNE, BLT, BGE, BLTU, BGEU)
    fn lift_branch(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);
        let imm = Self::imm_b(insn);
        let target = (addr as i64).wrapping_add(imm) as u64;
        let fallthrough = addr + 4;

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);

        let mut ops = Vec::new();

        // Compare rs1 and rs2
        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Cmp {
                src1: rs1,
                src2: SrcOperand::Reg(rs2),
                width: self.op_width(),
            },
        ));

        // Determine condition
        let cond = match funct3 {
            0b000 => Condition::Eq,  // BEQ
            0b001 => Condition::Ne,  // BNE
            0b100 => Condition::Slt, // BLT
            0b101 => Condition::Sge, // BGE
            0b110 => Condition::Ult, // BLTU
            0b111 => Condition::Uge, // BGEU
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        };

        // Set condition result
        let cond_reg = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::SetCC {
                dst: cond_reg,
                cond,
                width: OpWidth::W8,
            },
        ));

        Ok((
            ops,
            ControlFlow::CondBranchReg {
                cond: cond_reg,
                taken: target,
                not_taken: fallthrough,
            },
        ))
    }

    /// Load instructions (LB, LH, LW, LD, LBU, LHU, LWU)
    fn lift_load(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let funct3 = Self::funct3(insn);
        let imm = Self::imm_i(insn);

        let rs1 = self.get_x_reg(rs1_reg, ctx);

        let (width, sign) = match funct3 {
            0b000 => (MemWidth::B1, SignExtend::Sign), // LB
            0b001 => (MemWidth::B2, SignExtend::Sign), // LH
            0b010 => (MemWidth::B4, SignExtend::Sign), // LW
            0b011 => (MemWidth::B8, SignExtend::Zero), // LD (RV64)
            0b100 => (MemWidth::B1, SignExtend::Zero), // LBU
            0b101 => (MemWidth::B2, SignExtend::Zero), // LHU
            0b110 => (MemWidth::B4, SignExtend::Zero), // LWU (RV64)
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        };

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let address = Address::BaseOffset {
                base: rs1,
                offset: imm,
                disp_size: DispSize::Auto,
            };
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Load {
                    dst,
                    addr: address,
                    width,
                    sign,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Store instructions (SB, SH, SW, SD)
    fn lift_store(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);
        let imm = Self::imm_s(insn);

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);

        let width = match funct3 {
            0b000 => MemWidth::B1, // SB
            0b001 => MemWidth::B2, // SH
            0b010 => MemWidth::B4, // SW
            0b011 => MemWidth::B8, // SD (RV64)
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        };

        let mut ops = Vec::new();
        let address = Address::BaseOffset {
            base: rs1,
            offset: imm,
            disp_size: DispSize::Auto,
        };

        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Store {
                src: rs2,
                addr: address,
                width,
            },
        ));

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Integer register-immediate operations
    fn lift_op_imm(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let funct3 = Self::funct3(insn);
        let imm = Self::imm_i(insn);
        let shamt = (imm & 0x3F) as u8; // 6-bit shift amount for RV64

        // Route non-base OP-IMM (Zbb/Zbs immediates, unary count/extend) through
        // the decode-driven bit-manip path.
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let dop = rv_decode(insn, xl, &RvIsa::rv64gc()).op;
        if !matches!(
            dop,
            RvOp::Addi
                | RvOp::Slti
                | RvOp::Sltiu
                | RvOp::Xori
                | RvOp::Ori
                | RvOp::Andi
                | RvOp::Slli
                | RvOp::Srli
                | RvOp::Srai
        ) {
            return self.lift_zb_imm(insn, addr, ctx);
        }

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let width = self.op_width();

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let kind = match funct3 {
                0b000 => OpKind::Add {
                    // ADDI
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::None,
                },
                0b010 => {
                    // SLTI (set less than immediate)
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Cmp {
                            src1: rs1,
                            src2: SrcOperand::Imm(imm),
                            width,
                        },
                    ));
                    OpKind::SetCC {
                        dst,
                        cond: Condition::Slt,
                        width: OpWidth::W64,
                    }
                }
                0b011 => {
                    // SLTIU (set less than immediate unsigned)
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Cmp {
                            src1: rs1,
                            src2: SrcOperand::Imm(imm),
                            width,
                        },
                    ));
                    OpKind::SetCC {
                        dst,
                        cond: Condition::Ult,
                        width: OpWidth::W64,
                    }
                }
                0b100 => OpKind::Xor {
                    // XORI
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::None,
                },
                0b110 => OpKind::Or {
                    // ORI
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::None,
                },
                0b111 => OpKind::And {
                    // ANDI
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width,
                    flags: FlagUpdate::None,
                },
                // SLLI: RV64 funct6 (bits[31:26]) must be 0; other funct6 values
                // are Zbb/Zbs/crypto immediates handled elsewhere (or not yet
                // lifted) — never silently lower them as a plain shift.
                0b001 if (insn >> 26) & 0x3F == 0 => OpKind::Shl {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Imm(shamt as i64),
                    width,
                    flags: FlagUpdate::None,
                },
                // SRLI (funct6 == 0) / SRAI (funct6 == 0b010000). Any other funct6
                // (RORI/BEXTI/...) is not this instruction.
                0b101 if (insn >> 26) & 0x3F == 0 => OpKind::Shr {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Imm(shamt as i64),
                    width,
                    flags: FlagUpdate::None,
                },
                0b101 if (insn >> 26) & 0x3F == 0b010000 => OpKind::Sar {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Imm(shamt as i64),
                    width,
                    flags: FlagUpdate::None,
                },
                _ => {
                    return Err(LiftError::Unsupported {
                        addr,
                        mnemonic: format!("OP-IMM funct3={funct3:#05b} insn={insn:#010x}"),
                    });
                }
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Decode-driven lowering of OP-IMM bit-manipulation (Zbb/Zbs immediates
    /// and the unary count/extend/reverse ops). `Orc.b`/`Brev8` (no direct SMIR
    /// op) and crypto remain gaps.
    fn lift_zb_imm(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        use CryptoTerm::*;
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xl, &RvIsa::rv64gc());
        let rs1 = self.get_x_reg(d.rs1, ctx);
        let shamt = ((insn >> 20) & 0x3F) as i64; // 6-bit (RV64) bit/shift index
        let mut ops = Vec::new();
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let dst = match self.def_x_reg(d.rd, ctx) {
            Some(dst) => dst,
            None => return Ok((ops, ControlFlow::NextInsn)),
        };
        let w = OpWidth::W64;
        let bit = 1i64.wrapping_shl(shamt as u32);

        match d.op {
            RvOp::Rori => ops.push(mk(
                ctx,
                OpKind::Ror {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Imm(shamt),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Bclri => ops.push(mk(
                ctx,
                OpKind::AndNot {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(bit),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Bseti => ops.push(mk(
                ctx,
                OpKind::Or {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(bit),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Binvi => ops.push(mk(
                ctx,
                OpKind::Xor {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(bit),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Bexti => {
                let s = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Shr {
                        dst: s,
                        src: rs1,
                        amount: SrcOperand::Imm(shamt),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst,
                        src1: s,
                        src2: SrcOperand::Imm(1),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::Clz => ops.push(mk(
                ctx,
                OpKind::Clz {
                    dst,
                    src: rs1,
                    width: w,
                },
            )),
            RvOp::Ctz => ops.push(mk(
                ctx,
                OpKind::Ctz {
                    dst,
                    src: rs1,
                    width: w,
                },
            )),
            RvOp::Cpop => ops.push(mk(
                ctx,
                OpKind::Popcnt {
                    dst,
                    src: rs1,
                    width: w,
                },
            )),
            RvOp::SextB => ops.push(mk(
                ctx,
                OpKind::SignExtend {
                    dst,
                    src: rs1,
                    from_width: OpWidth::W8,
                    to_width: w,
                },
            )),
            RvOp::SextH => ops.push(mk(
                ctx,
                OpKind::SignExtend {
                    dst,
                    src: rs1,
                    from_width: OpWidth::W16,
                    to_width: w,
                },
            )),
            RvOp::Rev8 => ops.push(mk(
                ctx,
                OpKind::Bswap {
                    dst,
                    src: rs1,
                    width: w,
                },
            )),
            // Brev8 (reverse bits within each byte) = bswap(rbit(x)): a full bit
            // reverse moves byte i's bits (reversed) to byte 7-i; the byte swap
            // moves them back, leaving each byte bit-reversed in place.
            RvOp::Brev8 => {
                let t = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Rbit {
                        dst: t,
                        src: rs1,
                        width: w,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Bswap {
                        dst,
                        src: t,
                        width: w,
                    },
                ));
            }
            // SHA / SM3: xor-folds of rotates and a logical shift. 32-bit ops
            // (sha256/sm3) sign-extend the W32 result; sha512 are native W64.
            RvOp::Sha256Sig0 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(R, 7), (R, 18), (S, 3)],
                true,
            ),
            RvOp::Sha256Sig1 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(R, 17), (R, 19), (S, 10)],
                true,
            ),
            RvOp::Sha256Sum0 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(R, 2), (R, 13), (R, 22)],
                true,
            ),
            RvOp::Sha256Sum1 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(R, 6), (R, 11), (R, 25)],
                true,
            ),
            RvOp::Sha512Sig0 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(RW, 1), (RW, 8), (SW, 7)],
                false,
            ),
            RvOp::Sha512Sig1 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(RW, 19), (RW, 61), (SW, 6)],
                false,
            ),
            RvOp::Sha512Sum0 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(RW, 28), (RW, 34), (RW, 39)],
                false,
            ),
            RvOp::Sha512Sum1 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(RW, 14), (RW, 18), (RW, 41)],
                false,
            ),
            RvOp::Sm3p0 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(X, 0), (L, 9), (L, 17)],
                true,
            ),
            RvOp::Sm3p1 => self.crypto_xor3(
                ctx,
                &mut ops,
                addr,
                rs1,
                dst,
                &[(X, 0), (L, 15), (L, 23)],
                true,
            ),
            // AES-64 inverse-MixColumns (`aes64im`, unary) and the round-key
            // schedule step (`aes64ks1i`, with round number insn[23:20]) — S-box
            // table ops, computed bit-exactly by the RvIntCrypto op.
            RvOp::Aes64im | RvOp::Aes64ks1i => {
                let imm = match d.op {
                    RvOp::Aes64ks1i => ((insn >> 20) & 0xf) as u8,
                    _ => 0,
                };
                ops.push(mk(
                    ctx,
                    OpKind::RvIntCrypto {
                        dst,
                        src1: rs1,
                        src2: rs1,
                        op: d.op,
                        imm,
                    },
                ));
            }
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: format!("{:?}", d.op),
                });
            }
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Emit `dst = term0 ^ term1 ^ term2` where each term is a rotate / shift /
    /// identity of `src`, optionally sign-extending a 32-bit result to 64 bits.
    fn crypto_xor3(
        &mut self,
        ctx: &mut LiftContext,
        ops: &mut Vec<SmirOp>,
        addr: GuestAddr,
        src: VReg,
        dst: VReg,
        terms: &[(CryptoTerm, i64)],
        sext32: bool,
    ) {
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let term =
            |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, kind: CryptoTerm, amt: i64| -> VReg {
                if matches!(kind, CryptoTerm::X) {
                    return src;
                }
                let (tw, op): (OpWidth, u8) = match kind {
                    CryptoTerm::R => (OpWidth::W32, 0),
                    CryptoTerm::RW => (OpWidth::W64, 0),
                    CryptoTerm::L => (OpWidth::W32, 1),
                    CryptoTerm::S => (OpWidth::W32, 2),
                    CryptoTerm::SW => (OpWidth::W64, 2),
                    CryptoTerm::X => unreachable!(),
                };
                let t = ctx.alloc_vreg();
                let k = match op {
                    0 => OpKind::Ror {
                        dst: t,
                        src,
                        amount: SrcOperand::Imm(amt),
                        width: tw,
                        flags: FlagUpdate::None,
                    },
                    1 => OpKind::Rol {
                        dst: t,
                        src,
                        amount: SrcOperand::Imm(amt),
                        width: tw,
                        flags: FlagUpdate::None,
                    },
                    _ => OpKind::Shr {
                        dst: t,
                        src,
                        amount: SrcOperand::Imm(amt),
                        width: tw,
                        flags: FlagUpdate::None,
                    },
                };
                ops.push(mk(ctx, k));
                t
            };
        let xw = if sext32 { OpWidth::W32 } else { OpWidth::W64 };
        let a = term(ctx, ops, terms[0].0, terms[0].1);
        let b = term(ctx, ops, terms[1].0, terms[1].1);
        let c = term(ctx, ops, terms[2].0, terms[2].1);
        let ab = ctx.alloc_vreg();
        ops.push(mk(
            ctx,
            OpKind::Xor {
                dst: ab,
                src1: a,
                src2: SrcOperand::Reg(b),
                width: xw,
                flags: FlagUpdate::None,
            },
        ));
        if sext32 {
            let abc = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Xor {
                    dst: abc,
                    src1: ab,
                    src2: SrcOperand::Reg(c),
                    width: xw,
                    flags: FlagUpdate::None,
                },
            ));
            ops.push(mk(
                ctx,
                OpKind::SignExtend {
                    dst,
                    src: abc,
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        } else {
            ops.push(mk(
                ctx,
                OpKind::Xor {
                    dst,
                    src1: ab,
                    src2: SrcOperand::Reg(c),
                    width: xw,
                    flags: FlagUpdate::None,
                },
            ));
        }
    }

    /// 32-bit integer register-immediate operations (RV64 only)
    fn lift_op_imm32(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let funct3 = Self::funct3(insn);
        let imm = Self::imm_i(insn);
        let shamt = (imm & 0x1F) as u8; // 5-bit shift amount for 32-bit ops

        // Route non-base OP-IMM-32 (Zba slli.uw, Zbb roriw/clzw/cpopw/ctzw)
        // through the decode-driven word bit-manip path.
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let dop = rv_decode(insn, xl, &RvIsa::rv64gc()).op;
        if !matches!(dop, RvOp::Addiw | RvOp::Slliw | RvOp::Srliw | RvOp::Sraiw) {
            return self.lift_zb_imm32(insn, addr, ctx);
        }

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let width = OpWidth::W32;

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let tmp = ctx.alloc_vreg();

            let kind = match funct3 {
                0b000 => {
                    // ADDIW
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Add {
                            dst: tmp,
                            src1: rs1,
                            src2: SrcOperand::Imm(imm),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                    OpKind::SignExtend {
                        dst,
                        src: tmp,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    }
                }
                // SLLIW: funct7 must be 0 (Zba slli.uw uses funct7 0b0000010 and
                // a 6-bit shamt — not this instruction).
                0b001 if Self::funct7(insn) == 0 => {
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Shl {
                            dst: tmp,
                            src: rs1,
                            amount: SrcOperand::Imm(shamt as i64),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                    OpKind::SignExtend {
                        dst,
                        src: tmp,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    }
                }
                // SRLIW (funct7 == 0) / SRAIW (funct7 == 0b0100000). RORIW
                // (funct7 == 0b0110000) is not lowered here.
                0b101 if matches!(Self::funct7(insn), 0x00 | 0x20) => {
                    let arith = Self::funct7(insn) == 0x20;
                    let shift = if arith {
                        OpKind::Sar {
                            dst: tmp,
                            src: rs1,
                            amount: SrcOperand::Imm(shamt as i64),
                            width,
                            flags: FlagUpdate::None,
                        }
                    } else {
                        OpKind::Shr {
                            dst: tmp,
                            src: rs1,
                            amount: SrcOperand::Imm(shamt as i64),
                            width,
                            flags: FlagUpdate::None,
                        }
                    };
                    ops.push(SmirOp::new(ctx.next_op_id(), addr, shift));
                    OpKind::SignExtend {
                        dst,
                        src: tmp,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    }
                }
                _ => {
                    return Err(LiftError::Unsupported {
                        addr,
                        mnemonic: format!("OP-IMM-32 funct3={funct3:#05b} insn={insn:#010x}"),
                    });
                }
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Integer register-register operations
    fn lift_op(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);
        let funct7 = Self::funct7(insn);

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);
        let width = self.op_width();

        // M extension (multiply/divide)
        if funct7 == 0x01 && self.extensions.m {
            return self.lift_op_m(insn, addr, ctx);
        }
        // Anything that isn't a base RV64I register ALU op (Zba/Zbb/Zbs/Zbc/
        // Zicond/crypto) is lowered through the decode-driven bit-manip path.
        let is_base = funct7 == 0x00 || (funct7 == 0x20 && matches!(funct3, 0b000 | 0b101));
        if !is_base {
            return self.lift_zb_op(insn, addr, ctx);
        }

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let kind = match (funct7, funct3) {
                (0x00, 0b000) => OpKind::Add {
                    // ADD
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x20, 0b000) => OpKind::Sub {
                    // SUB
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x00, 0b001) => OpKind::Shl {
                    // SLL
                    dst,
                    src: rs1,
                    amount: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x00, 0b010) => {
                    // SLT
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Cmp {
                            src1: rs1,
                            src2: SrcOperand::Reg(rs2),
                            width,
                        },
                    ));
                    OpKind::SetCC {
                        dst,
                        cond: Condition::Slt,
                        width: OpWidth::W64,
                    }
                }
                (0x00, 0b011) => {
                    // SLTU
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Cmp {
                            src1: rs1,
                            src2: SrcOperand::Reg(rs2),
                            width,
                        },
                    ));
                    OpKind::SetCC {
                        dst,
                        cond: Condition::Ult,
                        width: OpWidth::W64,
                    }
                }
                (0x00, 0b100) => OpKind::Xor {
                    // XOR
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x00, 0b101) => OpKind::Shr {
                    // SRL
                    dst,
                    src: rs1,
                    amount: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x20, 0b101) => OpKind::Sar {
                    // SRA
                    dst,
                    src: rs1,
                    amount: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x00, 0b110) => OpKind::Or {
                    // OR
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x00, 0b111) => OpKind::And {
                    // AND
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                _ => {
                    return Err(LiftError::InvalidEncoding {
                        addr,
                        bytes: insn.to_le_bytes().to_vec(),
                    });
                }
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// 32-bit register-register operations (RV64 only)
    fn lift_op32(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);
        let funct7 = Self::funct7(insn);

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);
        let width = OpWidth::W32;

        // M extension (multiply/divide) - 32-bit variants
        if funct7 == 0x01 && self.extensions.m {
            return self.lift_op32_m(insn, addr, ctx);
        }
        // Non-base word ALU (Zba add.uw/sh*add.uw, Zbb rolw/rorw, Zbkb packw)
        // share the decode-driven bit-manip path.
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let dop = rv_decode(insn, xl, &RvIsa::rv64gc()).op;
        if !matches!(
            dop,
            RvOp::Addw | RvOp::Subw | RvOp::Sllw | RvOp::Srlw | RvOp::Sraw
        ) {
            return self.lift_zb_op(insn, addr, ctx);
        }

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let tmp = ctx.alloc_vreg();

            let inner_kind = match (funct7, funct3) {
                (0x00, 0b000) => OpKind::Add {
                    // ADDW
                    dst: tmp,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                (0x20, 0b000) => OpKind::Sub {
                    // SUBW
                    dst: tmp,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                // Word shifts: RISC-V masks the shift amount to 5 bits, but the
                // SMIR shift only zeroes at >= width.bits() (after a 6-bit mask),
                // so pre-mask rs2 to 0x1F.
                (0x00, 0b001) | (0x00, 0b101) | (0x20, 0b101) => {
                    let amt = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::And {
                            dst: amt,
                            src1: rs2,
                            src2: SrcOperand::Imm(0x1F),
                            width: OpWidth::W64,
                            flags: FlagUpdate::None,
                        },
                    ));
                    let amount = SrcOperand::Reg(amt);
                    match (funct7, funct3) {
                        (0x00, 0b001) => OpKind::Shl {
                            dst: tmp,
                            src: rs1,
                            amount,
                            width,
                            flags: FlagUpdate::None,
                        },
                        (0x00, 0b101) => OpKind::Shr {
                            dst: tmp,
                            src: rs1,
                            amount,
                            width,
                            flags: FlagUpdate::None,
                        },
                        _ => OpKind::Sar {
                            dst: tmp,
                            src: rs1,
                            amount,
                            width,
                            flags: FlagUpdate::None,
                        },
                    }
                }
                _ => {
                    return Err(LiftError::Unsupported {
                        addr,
                        mnemonic: format!("OP-32 funct7={funct7:#x} funct3={funct3:#05b}"),
                    });
                }
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, inner_kind));
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::SignExtend {
                    dst,
                    src: tmp,
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Decode-driven lowering of OP-space bit-manipulation / conditional ops
    /// (Zba/Zbb/Zbs/Zicond). Uses the verified RISC-V decoder for the precise
    /// operation; unsupported ops (Zbc carry-less mul, crypto, xperm) return
    /// `Unsupported`.
    fn lift_zb_op(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xlen = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xlen, &RvIsa::rv64gc());
        let rs1 = self.get_x_reg(d.rs1, ctx);
        let rs2 = self.get_x_reg(d.rs2, ctx);
        let mut ops = Vec::new();
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let dst = match self.def_x_reg(d.rd, ctx) {
            Some(dst) => dst,
            None => return Ok((ops, ControlFlow::NextInsn)), // rd == x0: pure no-op
        };
        let w = OpWidth::W64;

        // Helper: dst = min/max(rs1, rs2) using a compare + select.
        let mut minmax = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, cond: Condition| {
            ops.push(mk(
                ctx,
                OpKind::Cmp {
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width: w,
                },
            ));
            let c = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::SetCC {
                    dst: c,
                    cond,
                    width: w,
                },
            ));
            ops.push(mk(
                ctx,
                OpKind::Select {
                    dst,
                    cond: c,
                    src_true: rs1,
                    src_false: rs2,
                    width: w,
                },
            ));
        };

        // Helper: shift-add  dst = (rs1 << sh) + rs2  (optionally zext.w rs1 first)
        let mut shadd = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, sh: i64, uw: bool| {
            let base = if uw {
                let z = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::ZeroExtend {
                        dst: z,
                        src: rs1,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
                z
            } else {
                rs1
            };
            let s = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Shl {
                    dst: s,
                    src: base,
                    amount: SrcOperand::Imm(sh),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
            ops.push(mk(
                ctx,
                OpKind::Add {
                    dst,
                    src1: s,
                    src2: SrcOperand::Reg(rs2),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
        };

        // Helper: single-bit op  bit = 1 << (rs2 & (XLEN-1)); then apply.
        let mut bitop = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, which: u8| {
            let one = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Mov {
                    dst: one,
                    src: SrcOperand::Imm(1),
                    width: w,
                },
            ));
            let bit = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Shl {
                    dst: bit,
                    src: one,
                    amount: SrcOperand::Reg(rs2),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
            let k = match which {
                0 => OpKind::AndNot {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(bit),
                    width: w,
                    flags: FlagUpdate::None,
                }, // bclr
                1 => OpKind::Or {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(bit),
                    width: w,
                    flags: FlagUpdate::None,
                }, // bset
                _ => OpKind::Xor {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(bit),
                    width: w,
                    flags: FlagUpdate::None,
                }, // binv
            };
            ops.push(mk(ctx, k));
        };

        // Helper: word op into a temp, then sign-extend W32 -> W64.
        let mut wordret =
            |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, inner: OpKind, tmp: VReg| {
                ops.push(mk(ctx, inner));
                ops.push(mk(
                    ctx,
                    OpKind::SignExtend {
                        dst,
                        src: tmp,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
            };

        match d.op {
            RvOp::Andn => ops.push(mk(
                ctx,
                OpKind::AndNot {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Orn => {
                let n = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Not {
                        dst: n,
                        src: rs2,
                        width: w,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst,
                        src1: rs1,
                        src2: SrcOperand::Reg(n),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::Xnor => {
                let x = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Xor {
                        dst: x,
                        src1: rs1,
                        src2: SrcOperand::Reg(rs2),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Not {
                        dst,
                        src: x,
                        width: w,
                    },
                ));
            }
            RvOp::Rol => ops.push(mk(
                ctx,
                OpKind::Rol {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Reg(rs2),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Ror => ops.push(mk(
                ctx,
                OpKind::Ror {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Reg(rs2),
                    width: w,
                    flags: FlagUpdate::None,
                },
            )),
            RvOp::Rolw => {
                let t = ctx.alloc_vreg();
                wordret(
                    ctx,
                    &mut ops,
                    OpKind::Rol {
                        dst: t,
                        src: rs1,
                        amount: SrcOperand::Reg(rs2),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                    t,
                );
            }
            RvOp::Rorw => {
                let t = ctx.alloc_vreg();
                wordret(
                    ctx,
                    &mut ops,
                    OpKind::Ror {
                        dst: t,
                        src: rs1,
                        amount: SrcOperand::Reg(rs2),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                    t,
                );
            }
            RvOp::Min => minmax(ctx, &mut ops, Condition::Slt),
            RvOp::Minu => minmax(ctx, &mut ops, Condition::Ult),
            RvOp::Max => minmax(ctx, &mut ops, Condition::Sgt),
            RvOp::Maxu => minmax(ctx, &mut ops, Condition::Ugt),
            RvOp::SextB => ops.push(mk(
                ctx,
                OpKind::SignExtend {
                    dst,
                    src: rs1,
                    from_width: OpWidth::W8,
                    to_width: w,
                },
            )),
            RvOp::SextH => ops.push(mk(
                ctx,
                OpKind::SignExtend {
                    dst,
                    src: rs1,
                    from_width: OpWidth::W16,
                    to_width: w,
                },
            )),
            RvOp::ZextH => ops.push(mk(
                ctx,
                OpKind::ZeroExtend {
                    dst,
                    src: rs1,
                    from_width: OpWidth::W16,
                    to_width: w,
                },
            )),
            RvOp::Sh1add => shadd(ctx, &mut ops, 1, false),
            RvOp::Sh2add => shadd(ctx, &mut ops, 2, false),
            RvOp::Sh3add => shadd(ctx, &mut ops, 3, false),
            RvOp::Sh1addUw => shadd(ctx, &mut ops, 1, true),
            RvOp::Sh2addUw => shadd(ctx, &mut ops, 2, true),
            RvOp::Sh3addUw => shadd(ctx, &mut ops, 3, true),
            RvOp::AddUw => {
                let z = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::ZeroExtend {
                        dst: z,
                        src: rs1,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Add {
                        dst,
                        src1: z,
                        src2: SrcOperand::Reg(rs2),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::Bclr => bitop(ctx, &mut ops, 0),
            RvOp::Bset => bitop(ctx, &mut ops, 1),
            RvOp::Binv => bitop(ctx, &mut ops, 2),
            RvOp::Bext => {
                let s = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Shr {
                        dst: s,
                        src: rs1,
                        amount: SrcOperand::Reg(rs2),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst,
                        src1: s,
                        src2: SrcOperand::Imm(1),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::CzeroEqz => {
                // rd = (rs2 != 0) ? rs1 : 0
                ops.push(mk(
                    ctx,
                    OpKind::Cmp {
                        src1: rs2,
                        src2: SrcOperand::Imm(0),
                        width: w,
                    },
                ));
                let nz = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::SetCC {
                        dst: nz,
                        cond: Condition::Ne,
                        width: w,
                    },
                ));
                let zero = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Mov {
                        dst: zero,
                        src: SrcOperand::Imm(0),
                        width: w,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Select {
                        dst,
                        cond: nz,
                        src_true: rs1,
                        src_false: zero,
                        width: w,
                    },
                ));
            }
            RvOp::CzeroNez => {
                // rd = (rs2 == 0) ? rs1 : 0
                ops.push(mk(
                    ctx,
                    OpKind::Cmp {
                        src1: rs2,
                        src2: SrcOperand::Imm(0),
                        width: w,
                    },
                ));
                let z = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::SetCC {
                        dst: z,
                        cond: Condition::Eq,
                        width: w,
                    },
                ));
                let zero = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Mov {
                        dst: zero,
                        src: SrcOperand::Imm(0),
                        width: w,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Select {
                        dst,
                        cond: z,
                        src_true: rs1,
                        src_false: zero,
                        width: w,
                    },
                ));
            }
            // Pack: rd = (rs2[lo] << shift) | rs1[lo]. pack uses XLEN/2 halves
            // (zext.w on RV64), packh uses bytes, packw uses 16-bit halves with
            // a sign-extended 32-bit result.
            RvOp::Pack => {
                let a = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::ZeroExtend {
                        dst: a,
                        src: rs1,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
                let b = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::ZeroExtend {
                        dst: b,
                        src: rs2,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
                let bsh = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Shl {
                        dst: bsh,
                        src: b,
                        amount: SrcOperand::Imm(32),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst,
                        src1: a,
                        src2: SrcOperand::Reg(bsh),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::Packh => {
                let a = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: a,
                        src1: rs1,
                        src2: SrcOperand::Imm(0xff),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let b = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: b,
                        src1: rs2,
                        src2: SrcOperand::Imm(0xff),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let bsh = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Shl {
                        dst: bsh,
                        src: b,
                        amount: SrcOperand::Imm(8),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst,
                        src1: a,
                        src2: SrcOperand::Reg(bsh),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::Packw => {
                let a = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: a,
                        src1: rs1,
                        src2: SrcOperand::Imm(0xffff),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let b = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: b,
                        src1: rs2,
                        src2: SrcOperand::Imm(0xffff),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let bsh = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Shl {
                        dst: bsh,
                        src: b,
                        amount: SrcOperand::Imm(16),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let t = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst: t,
                        src1: a,
                        src2: SrcOperand::Reg(bsh),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::SignExtend {
                        dst,
                        src: t,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
            }
            // Carry-less multiply, crossbar permute, AES-64 / SM4 round and key
            // helpers — no clean SMIR primitive; computed bit-exactly by the
            // RvIntCrypto op. SM4 carries its `bs` field (insn[31:30]).
            RvOp::Clmul
            | RvOp::Clmulh
            | RvOp::Clmulr
            | RvOp::Xperm4
            | RvOp::Xperm8
            | RvOp::Sm4ed
            | RvOp::Sm4ks
            | RvOp::Aes64es
            | RvOp::Aes64esm
            | RvOp::Aes64ds
            | RvOp::Aes64dsm
            | RvOp::Aes64ks2 => {
                let imm = match d.op {
                    RvOp::Sm4ed | RvOp::Sm4ks => ((insn >> 30) & 3) as u8,
                    _ => 0,
                };
                ops.push(mk(
                    ctx,
                    OpKind::RvIntCrypto {
                        dst,
                        src1: rs1,
                        src2: rs2,
                        op: d.op,
                        imm,
                    },
                ));
            }
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: format!("{:?}", d.op),
                });
            }
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Decode-driven lowering of OP-IMM-32 bit-manipulation (slli.uw, roriw,
    /// clzw/cpopw/ctzw).
    fn lift_zb_imm32(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xl, &RvIsa::rv64gc());
        let rs1 = self.get_x_reg(d.rs1, ctx);
        let mut ops = Vec::new();
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let dst = match self.def_x_reg(d.rd, ctx) {
            Some(dst) => dst,
            None => return Ok((ops, ControlFlow::NextInsn)),
        };
        let w = OpWidth::W64;
        // Word results that are intrinsically <= 32 bits (counts) are zero-safe;
        // shift/rotate word results must be sign-extended.
        let sext32 = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, tmp: VReg| {
            ops.push(mk(
                ctx,
                OpKind::SignExtend {
                    dst,
                    src: tmp,
                    from_width: OpWidth::W32,
                    to_width: w,
                },
            ));
        };
        match d.op {
            RvOp::SlliUw => {
                let shamt = ((insn >> 20) & 0x3F) as i64; // 6-bit
                let z = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::ZeroExtend {
                        dst: z,
                        src: rs1,
                        from_width: OpWidth::W32,
                        to_width: w,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Shl {
                        dst,
                        src: z,
                        amount: SrcOperand::Imm(shamt),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::Roriw => {
                let shamt = ((insn >> 20) & 0x1F) as i64;
                let t = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Ror {
                        dst: t,
                        src: rs1,
                        amount: SrcOperand::Imm(shamt),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                ));
                sext32(ctx, &mut ops, t);
            }
            RvOp::Clzw => ops.push(mk(
                ctx,
                OpKind::Clz {
                    dst,
                    src: rs1,
                    width: OpWidth::W32,
                },
            )),
            RvOp::Ctzw => ops.push(mk(
                ctx,
                OpKind::Ctz {
                    dst,
                    src: rs1,
                    width: OpWidth::W32,
                },
            )),
            RvOp::Cpopw => ops.push(mk(
                ctx,
                OpKind::Popcnt {
                    dst,
                    src: rs1,
                    width: OpWidth::W32,
                },
            )),
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: format!("{:?}", d.op),
                });
            }
        }
        Ok((ops, ControlFlow::NextInsn))
    }

    /// FP load/store (FLW/FLD/FLH/FSW/FSD/FSH). Loads NaN-box narrower values.
    /// Vector loads/stores (same opcodes) are gaps.
    fn lift_fp_ldst(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xl, &RvIsa::rv64gc());
        let mut ops = Vec::new();
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let base = self.get_x_reg(d.rs1, ctx);
        let address = Address::BaseOffset {
            base,
            offset: d.imm,
            disp_size: DispSize::Auto,
        };
        // (load?, width, nan-box mask of the *upper* bits)
        let (is_load, width, boxmask): (bool, MemWidth, i64) = match d.op {
            RvOp::Flw => (true, MemWidth::B4, 0xffff_ffff_0000_0000u64 as i64),
            RvOp::Fld => (true, MemWidth::B8, 0),
            RvOp::Flh => (true, MemWidth::B2, 0xffff_ffff_ffff_0000u64 as i64),
            RvOp::Fsw => (false, MemWidth::B4, 0),
            RvOp::Fsd => (false, MemWidth::B8, 0),
            RvOp::Fsh => (false, MemWidth::B2, 0),
            _ => {
                // Vector load/store share the 0x07/0x27 major opcodes (the
                // mop/lumop fields distinguish them) — opaque RvVector.
                if !d.is_illegal() {
                    return self.emit_rv_vector(insn, &d, addr, ctx);
                }
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        };
        if is_load {
            let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
            if boxmask == 0 {
                ops.push(mk(
                    ctx,
                    OpKind::Load {
                        dst: fd,
                        addr: address,
                        width,
                        sign: SignExtend::Zero,
                    },
                ));
            } else {
                let t = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::Load {
                        dst: t,
                        addr: address,
                        width,
                        sign: SignExtend::Zero,
                    },
                ));
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst: fd,
                        src1: t,
                        src2: SrcOperand::Imm(boxmask),
                        width: OpWidth::W64,
                        flags: FlagUpdate::None,
                    },
                ));
            }
        } else {
            let fs = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rs2)));
            ops.push(mk(
                ctx,
                OpKind::Store {
                    src: fs,
                    addr: address,
                    width,
                },
            ));
        }
        Ok((ops, ControlFlow::NextInsn))
    }

    /// OP-V (0x57): RVV arithmetic and `vset{i}vl{i}` configuration. RVV element
    /// width and length are runtime `vtype`/`vl` state unknown at lift time, so
    /// the whole vector ISA is lifted to the opaque [`OpKind::RvVector`] engine.
    fn lift_vector(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xl, &RvIsa::rv64gc());
        if d.is_illegal() {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }
        self.emit_rv_vector(insn, &d, addr, ctx)
    }

    /// Emit an opaque [`OpKind::RvVector`] for one RVV instruction. `rs1`/`rs2`
    /// (x-register address/AVL/stride/index sources) are kept live for the
    /// optimizer; the interp writes all results (vector file, CSRs, x/f) directly
    /// into `ctx.arch_regs` by running the verified vector engine.
    fn emit_rv_vector(
        &mut self,
        insn: u32,
        d: &crate::riscv::Insn,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs1 = self.get_x_reg(d.rs1, ctx);
        let rs2 = self.get_x_reg(d.rs2, ctx);
        let ops = vec![SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::RvVector { insn, rs1, rs2 },
        )];
        Ok((ops, ControlFlow::NextInsn))
    }

    /// OP-FP (0x53): only the fflags/rounding-free ops — FP<->int bit moves
    /// (FMV.*) and sign injection (FSGNJ/N/X). All arithmetic / convert /
    /// compare / classify ops are gaps (need SMIR FP fflags support).
    fn lift_op_fp(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xl, &RvIsa::rv64gc());
        let mut ops = Vec::new();
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let w = OpWidth::W64;
        let getf =
            |reg: u8, ctx: &mut LiftContext| ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(reg)));

        // Sign-injection helper: fd = nanbox | (fs1 & ~signbit) | sign(fs1, fs2).
        // mode 0 = fsgnj (sign of fs2), 1 = fsgnjn (~sign of fs2), 2 = fsgnjx
        // (sign fs1 ^ fs2).
        // Canonicalize a narrow (.S/.H) operand: if it is not properly
        // NaN-boxed (upper bits all-1) it reads as the canonical NaN.
        let unbox = |ctx: &mut LiftContext,
                     ops: &mut Vec<SmirOp>,
                     f: VReg,
                     sh: i64,
                     himask: i64,
                     lomask: i64,
                     cn: i64|
         -> VReg {
            let hi = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Shr {
                    dst: hi,
                    src: f,
                    amount: SrcOperand::Imm(sh),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
            ops.push(mk(
                ctx,
                OpKind::Cmp {
                    src1: hi,
                    src2: SrcOperand::Imm(himask),
                    width: w,
                },
            ));
            let boxed = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::SetCC {
                    dst: boxed,
                    cond: Condition::Eq,
                    width: w,
                },
            ));
            let lo = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::And {
                    dst: lo,
                    src1: f,
                    src2: SrcOperand::Imm(lomask),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
            let cnr = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Mov {
                    dst: cnr,
                    src: SrcOperand::Imm(cn),
                    width: w,
                },
            ));
            let u = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Select {
                    dst: u,
                    cond: boxed,
                    src_true: lo,
                    src_false: cnr,
                    width: w,
                },
            ));
            u
        };
        let mut sgnj = |ctx: &mut LiftContext,
                        ops: &mut Vec<SmirOp>,
                        fs1: VReg,
                        fs2: VReg,
                        signbit: i64,
                        boxmask: i64,
                        mode: u8| {
            let a = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::And {
                    dst: a,
                    src1: fs1,
                    src2: SrcOperand::Imm(!signbit),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
            let sb = ctx.alloc_vreg();
            match mode {
                0 => ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: sb,
                        src1: fs2,
                        src2: SrcOperand::Imm(signbit),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                )),
                1 => {
                    // ~fs2 & signbit  ==  (fs2 & signbit) ^ signbit
                    let tmp = ctx.alloc_vreg();
                    ops.push(mk(
                        ctx,
                        OpKind::And {
                            dst: tmp,
                            src1: fs2,
                            src2: SrcOperand::Imm(signbit),
                            width: w,
                            flags: FlagUpdate::None,
                        },
                    ));
                    ops.push(mk(
                        ctx,
                        OpKind::Xor {
                            dst: sb,
                            src1: tmp,
                            src2: SrcOperand::Imm(signbit),
                            width: w,
                            flags: FlagUpdate::None,
                        },
                    ));
                }
                _ => {
                    let x = ctx.alloc_vreg();
                    ops.push(mk(
                        ctx,
                        OpKind::Xor {
                            dst: x,
                            src1: fs1,
                            src2: SrcOperand::Reg(fs2),
                            width: w,
                            flags: FlagUpdate::None,
                        },
                    ));
                    ops.push(mk(
                        ctx,
                        OpKind::And {
                            dst: sb,
                            src1: x,
                            src2: SrcOperand::Imm(signbit),
                            width: w,
                            flags: FlagUpdate::None,
                        },
                    ));
                }
            }
            let c = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Or {
                    dst: c,
                    src1: a,
                    src2: SrcOperand::Reg(sb),
                    width: w,
                    flags: FlagUpdate::None,
                },
            ));
            let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
            if boxmask == 0 {
                ops.push(mk(
                    ctx,
                    OpKind::Mov {
                        dst: fd,
                        src: SrcOperand::Reg(c),
                        width: w,
                    },
                ));
            } else {
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst: fd,
                        src1: c,
                        src2: SrcOperand::Imm(boxmask),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
        };
        const SB_D: i64 = 0x8000_0000_0000_0000u64 as i64;
        const BOX_S: i64 = 0xffff_ffff_0000_0000u64 as i64;
        const BOX_H: i64 = 0xffff_ffff_ffff_0000u64 as i64;

        match d.op {
            // FP -> int bit moves.
            RvOp::FmvXW => {
                let fs = getf(d.rs1, ctx);
                if let Some(dst) = self.def_x_reg(d.rd, ctx) {
                    ops.push(mk(
                        ctx,
                        OpKind::SignExtend {
                            dst,
                            src: fs,
                            from_width: OpWidth::W32,
                            to_width: w,
                        },
                    ));
                }
            }
            RvOp::FmvXH => {
                let fs = getf(d.rs1, ctx);
                if let Some(dst) = self.def_x_reg(d.rd, ctx) {
                    ops.push(mk(
                        ctx,
                        OpKind::SignExtend {
                            dst,
                            src: fs,
                            from_width: OpWidth::W16,
                            to_width: w,
                        },
                    ));
                }
            }
            RvOp::FmvXD => {
                let fs = getf(d.rs1, ctx);
                if let Some(dst) = self.def_x_reg(d.rd, ctx) {
                    ops.push(mk(
                        ctx,
                        OpKind::Mov {
                            dst,
                            src: SrcOperand::Reg(fs),
                            width: w,
                        },
                    ));
                }
            }
            // int -> FP bit moves (NaN-box narrow values).
            RvOp::FmvWX => {
                let xs = self.get_x_reg(d.rs1, ctx);
                let t = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: t,
                        src1: xs,
                        src2: SrcOperand::Imm(0xffff_ffff),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst: fd,
                        src1: t,
                        src2: SrcOperand::Imm(BOX_S),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::FmvHX => {
                let xs = self.get_x_reg(d.rs1, ctx);
                let t = ctx.alloc_vreg();
                ops.push(mk(
                    ctx,
                    OpKind::And {
                        dst: t,
                        src1: xs,
                        src2: SrcOperand::Imm(0xffff),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
                let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
                ops.push(mk(
                    ctx,
                    OpKind::Or {
                        dst: fd,
                        src1: t,
                        src2: SrcOperand::Imm(BOX_H),
                        width: w,
                        flags: FlagUpdate::None,
                    },
                ));
            }
            RvOp::FmvDX => {
                let xs = self.get_x_reg(d.rs1, ctx);
                let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
                ops.push(mk(
                    ctx,
                    OpKind::Mov {
                        dst: fd,
                        src: SrcOperand::Reg(xs),
                        width: w,
                    },
                ));
            }
            // Sign injection. Only .D is lifted: .S/.H must canonicalize an
            // improperly-NaN-boxed operand to the canonical NaN first, which
            // needs extra unbox conditionals — gapped for now.
            RvOp::FsgnjD | RvOp::FsgnjnD | RvOp::FsgnjxD => {
                let m = match d.op {
                    RvOp::FsgnjD => 0,
                    RvOp::FsgnjnD => 1,
                    _ => 2,
                };
                let fs1 = getf(d.rs1, ctx);
                let fs2 = getf(d.rs2, ctx);
                sgnj(ctx, &mut ops, fs1, fs2, SB_D, 0, m);
            }
            RvOp::FsgnjS | RvOp::FsgnjnS | RvOp::FsgnjxS => {
                let m = match d.op {
                    RvOp::FsgnjS => 0,
                    RvOp::FsgnjnS => 1,
                    _ => 2,
                };
                let fs1 = getf(d.rs1, ctx);
                let fs2 = getf(d.rs2, ctx);
                let u1 = unbox(
                    ctx,
                    &mut ops,
                    fs1,
                    32,
                    0xffff_ffff,
                    0xffff_ffff,
                    0x7fc0_0000,
                );
                let u2 = unbox(
                    ctx,
                    &mut ops,
                    fs2,
                    32,
                    0xffff_ffff,
                    0xffff_ffff,
                    0x7fc0_0000,
                );
                sgnj(ctx, &mut ops, u1, u2, 0x8000_0000u64 as i64, BOX_S, m);
            }
            RvOp::FsgnjH | RvOp::FsgnjnH | RvOp::FsgnjxH => {
                let m = match d.op {
                    RvOp::FsgnjH => 0,
                    RvOp::FsgnjnH => 1,
                    _ => 2,
                };
                let fs1 = getf(d.rs1, ctx);
                let fs2 = getf(d.rs2, ctx);
                let u1 = unbox(ctx, &mut ops, fs1, 16, 0xffff_ffff_ffff, 0xffff, 0x7e00);
                let u2 = unbox(ctx, &mut ops, fs2, 16, 0xffff_ffff_ffff, 0xffff, 0x7e00);
                sgnj(ctx, &mut ops, u1, u2, 0x8000, BOX_H, m);
            }
            // Classify (fflags-free): canonicalize narrow operands, then build
            // the 10-bit class mask. (FCLASS does not depend on rounding.)
            RvOp::FclassD => {
                let fs = getf(d.rs1, ctx);
                if let Some(dst) = self.def_x_reg(d.rd, ctx) {
                    self.emit_fclass(ctx, &mut ops, addr, fs, 52, 11, dst);
                }
            }
            RvOp::FclassS => {
                let fs = getf(d.rs1, ctx);
                let u = unbox(ctx, &mut ops, fs, 32, 0xffff_ffff, 0xffff_ffff, 0x7fc0_0000);
                if let Some(dst) = self.def_x_reg(d.rd, ctx) {
                    self.emit_fclass(ctx, &mut ops, addr, u, 23, 8, dst);
                }
            }
            RvOp::FclassH => {
                let fs = getf(d.rs1, ctx);
                let u = unbox(ctx, &mut ops, fs, 16, 0xffff_ffff_ffff, 0xffff, 0x7e00);
                if let Some(dst) = self.def_x_reg(d.rd, ctx) {
                    self.emit_fclass(ctx, &mut ops, addr, u, 10, 5, dst);
                }
            }
            // Zfa load-immediate: `rs1` is a 32-entry table index (NOT a
            // register), so materialise the constant directly and NaN-box it.
            RvOp::FliS | RvOp::FliD | RvOp::FliH => {
                use crate::riscv::float as ff;
                let (bits, boxmask) = match d.op {
                    RvOp::FliS => (ff::fli(ff::F32, d.rs1) as u32 as i64, BOX_S),
                    RvOp::FliH => (ff::fli(ff::F16, d.rs1) as u16 as i64, BOX_H),
                    _ => (ff::fli(ff::F64, d.rs1) as i64, 0),
                };
                let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
                ops.push(mk(
                    ctx,
                    OpKind::Mov {
                        dst: fd,
                        src: SrcOperand::Imm(bits | boxmask),
                        width: w,
                    },
                ));
            }
            // All remaining OP-FP arithmetic / convert / compare / min-max /
            // round ops are computed bit-exactly via the RvFp op (fflags + NaN
            // canonicalisation + dynamic rounding).
            _ => {
                if d.is_illegal() {
                    return Err(LiftError::InvalidEncoding {
                        addr,
                        bytes: insn.to_le_bytes().to_vec(),
                    });
                }
                self.emit_rvfp(&d, addr, ctx, &mut ops);
            }
        }
        Ok((ops, ControlFlow::NextInsn))
    }

    /// Lift the FMA family (opcodes 0x43/0x47/0x4b/0x4f) via the bit-exact RvFp
    /// op. The decoder maps each to the concrete `Fmadd/Fmsub/Fnmsub/Fnmadd`
    /// `.S/.D/.H` opcode; the operand sign flips live in `eval_scalar_fp`.
    fn lift_fp_fma(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = rv_decode(insn, xl, &RvIsa::rv64gc());
        if d.is_illegal() {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }
        let mut ops = Vec::new();
        self.emit_rvfp(&d, addr, ctx, &mut ops);
        Ok((ops, ControlFlow::NextInsn))
    }

    /// Emit a bit-exact [`OpKind::RvFp`] for a scalar OP-FP / FMA instruction
    /// whose result depends on `fflags` / NaN-canonicalisation / dynamic
    /// rounding. Routes the source/destination register *files* per
    /// [`crate::riscv::float::fp_uses_int_src1`] /
    /// [`crate::riscv::float::fp_writes_int_dst`], threads `fcsr` in and out, and
    /// updates `fcsr` even when an integer destination is `x0` (exceptions still
    /// accrue). Read all source vregs (incl. `fcsr_src`) BEFORE defining any
    /// destination so a `rd == rs1` aliasing case reads the old value.
    fn emit_rvfp(
        &mut self,
        d: &crate::riscv::Insn,
        addr: GuestAddr,
        ctx: &mut LiftContext,
        ops: &mut Vec<SmirOp>,
    ) {
        use crate::riscv::float::{fp_uses_int_src1, fp_writes_int_dst};
        let op = d.op;
        let src1 = if fp_uses_int_src1(op) {
            self.get_x_reg(d.rs1, ctx)
        } else {
            ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rs1)))
        };
        let src2 = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rs2)));
        let src3 = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rs3)));
        let fcsr_src = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::Csr(0x003)));
        let fcsr_dst = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::Csr(0x003)));
        let dst = if fp_writes_int_dst(op) {
            // rd == x0 discards the result but `fcsr` must still update.
            self.def_x_reg(d.rd, ctx)
                .unwrap_or_else(|| ctx.alloc_vreg())
        } else {
            ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)))
        };
        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::RvFp {
                dst,
                fcsr_dst,
                src1,
                src2,
                src3,
                fcsr_src,
                op,
                rm_field: d.rm(),
            },
        ));
    }

    /// Emit the RISC-V FCLASS 10-bit classification of FP value `f` (the value
    /// must already be unboxed for .S/.H) into integer register `dst`.
    fn emit_fclass(
        &mut self,
        ctx: &mut LiftContext,
        ops: &mut Vec<SmirOp>,
        addr: GuestAddr,
        f: VReg,
        mant_bits: u32,
        exp_bits: u32,
        dst: VReg,
    ) {
        let w = OpWidth::W64;
        let emask = (1i64 << exp_bits) - 1;
        let mmask = (1i64 << mant_bits) - 1;
        let push = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, k: OpKind| {
            ops.push(SmirOp::new(ctx.next_op_id(), addr, k));
        };
        let shr = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, src: VReg, amt: i64| -> VReg {
            let r = ctx.alloc_vreg();
            push(
                ctx,
                ops,
                OpKind::Shr {
                    dst: r,
                    src,
                    amount: SrcOperand::Imm(amt),
                    width: w,
                    flags: FlagUpdate::None,
                },
            );
            r
        };
        let andi = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, src: VReg, imm: i64| -> VReg {
            let r = ctx.alloc_vreg();
            push(
                ctx,
                ops,
                OpKind::And {
                    dst: r,
                    src1: src,
                    src2: SrcOperand::Imm(imm),
                    width: w,
                    flags: FlagUpdate::None,
                },
            );
            r
        };
        let cmpset = |ctx: &mut LiftContext,
                      ops: &mut Vec<SmirOp>,
                      a: VReg,
                      imm: i64,
                      cond: Condition|
         -> VReg {
            push(
                ctx,
                ops,
                OpKind::Cmp {
                    src1: a,
                    src2: SrcOperand::Imm(imm),
                    width: w,
                },
            );
            let r = ctx.alloc_vreg();
            push(
                ctx,
                ops,
                OpKind::SetCC {
                    dst: r,
                    cond,
                    width: w,
                },
            );
            r
        };
        let andv = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, a: VReg, b: VReg| -> VReg {
            let r = ctx.alloc_vreg();
            push(
                ctx,
                ops,
                OpKind::And {
                    dst: r,
                    src1: a,
                    src2: SrcOperand::Reg(b),
                    width: w,
                    flags: FlagUpdate::None,
                },
            );
            r
        };
        let not1 = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, a: VReg| -> VReg {
            let r = ctx.alloc_vreg();
            push(
                ctx,
                ops,
                OpKind::Xor {
                    dst: r,
                    src1: a,
                    src2: SrcOperand::Imm(1),
                    width: w,
                    flags: FlagUpdate::None,
                },
            );
            r
        };

        let exp_sh = shr(ctx, ops, f, mant_bits as i64);
        let exp = andi(ctx, ops, exp_sh, emask);
        let sign_sh = shr(ctx, ops, f, (mant_bits + exp_bits) as i64);
        let sign = andi(ctx, ops, sign_sh, 1);
        let mant = andi(ctx, ops, f, mmask);
        let mq_sh = shr(ctx, ops, mant, (mant_bits - 1) as i64);
        let mq = andi(ctx, ops, mq_sh, 1);

        let emax = cmpset(ctx, ops, exp, emask, Condition::Eq);
        let ezero = cmpset(ctx, ops, exp, 0, Condition::Eq);
        let mzero = cmpset(ctx, ops, mant, 0, Condition::Eq);
        let mnz = not1(ctx, ops, mzero);
        let pos = not1(ctx, ops, sign);
        let nmq = not1(ctx, ops, mq);
        let enorm = {
            let a = not1(ctx, ops, emax);
            let b = not1(ctx, ops, ezero);
            andv(ctx, ops, a, b)
        };

        // (class bit index, condition vreg)
        let emz = andv(ctx, ops, emax, mzero);
        let emnz = andv(ctx, ops, emax, mnz);
        let ezmz = andv(ctx, ops, ezero, mzero);
        let ezmnz = andv(ctx, ops, ezero, mnz);
        let bits: [(i64, VReg); 10] = [
            (0, andv(ctx, ops, emz, sign)),   // -inf
            (7, andv(ctx, ops, emz, pos)),    // +inf
            (9, andv(ctx, ops, emnz, mq)),    // qNaN
            (8, andv(ctx, ops, emnz, nmq)),   // sNaN
            (3, andv(ctx, ops, ezmz, sign)),  // -0
            (4, andv(ctx, ops, ezmz, pos)),   // +0
            (2, andv(ctx, ops, ezmnz, sign)), // -subnormal
            (5, andv(ctx, ops, ezmnz, pos)),  // +subnormal
            (1, andv(ctx, ops, enorm, sign)), // -normal
            (6, andv(ctx, ops, enorm, pos)),  // +normal
        ];
        // acc = OR of (bit << k); first term initializes via Shl into acc.
        let mut acc: Option<VReg> = None;
        for (k, b) in bits {
            let t = ctx.alloc_vreg();
            push(
                ctx,
                ops,
                OpKind::Shl {
                    dst: t,
                    src: b,
                    amount: SrcOperand::Imm(k),
                    width: w,
                    flags: FlagUpdate::None,
                },
            );
            acc = Some(match acc {
                None => t,
                Some(a) => {
                    let r = ctx.alloc_vreg();
                    push(
                        ctx,
                        ops,
                        OpKind::Or {
                            dst: r,
                            src1: a,
                            src2: SrcOperand::Reg(t),
                            width: w,
                            flags: FlagUpdate::None,
                        },
                    );
                    r
                }
            });
        }
        push(
            ctx,
            ops,
            OpKind::Mov {
                dst,
                src: SrcOperand::Reg(acc.unwrap()),
                width: w,
            },
        );
    }

    /// M extension multiply/divide operations
    fn lift_op_m(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);
        let width = self.op_width();

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let kind = match funct3 {
                0b000 => OpKind::MulS {
                    // MUL (lower bits)
                    dst_lo: dst,
                    dst_hi: None,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                0b001 => {
                    // MULH (upper bits, signed * signed)
                    let lo = ctx.alloc_vreg();
                    OpKind::MulS {
                        dst_lo: lo,
                        dst_hi: Some(dst),
                        src1: rs1,
                        src2: SrcOperand::Reg(rs2),
                        width,
                        flags: FlagUpdate::None,
                    }
                }
                0b010 => {
                    // MULHSU (signed * unsigned, high word). No direct SMIR op,
                    // but the identity mulhsu(a,b) = mulhu(a,b) - (a<0 ? b : 0)
                    // holds (s_a = u_a - 2^64*(a<0)).
                    let lo = ctx.alloc_vreg();
                    let hi = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::MulU {
                            dst_lo: lo,
                            dst_hi: Some(hi),
                            src1: rs1,
                            src2: SrcOperand::Reg(rs2),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Cmp {
                            src1: rs1,
                            src2: SrcOperand::Imm(0),
                            width,
                        },
                    ));
                    let neg = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::SetCC {
                            dst: neg,
                            cond: Condition::Slt,
                            width,
                        },
                    ));
                    let zero = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Mov {
                            dst: zero,
                            src: SrcOperand::Imm(0),
                            width,
                        },
                    ));
                    let subv = ctx.alloc_vreg();
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Select {
                            dst: subv,
                            cond: neg,
                            src_true: rs2,
                            src_false: zero,
                            width,
                        },
                    ));
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Sub {
                            dst,
                            src1: hi,
                            src2: SrcOperand::Reg(subv),
                            width,
                            flags: FlagUpdate::None,
                        },
                    ));
                    return Ok((ops, ControlFlow::NextInsn));
                }
                0b011 => {
                    // MULHU (upper bits, unsigned * unsigned)
                    let lo = ctx.alloc_vreg();
                    OpKind::MulU {
                        dst_lo: lo,
                        dst_hi: Some(dst),
                        src1: rs1,
                        src2: SrcOperand::Reg(rs2),
                        width,
                        flags: FlagUpdate::None,
                    }
                }
                // DIV/DIVU/REM/REMU: SMIR's DivS/DivU trap (x86 #DE) on a zero
                // divisor and don't implement RISC-V's div-by-zero/overflow
                // results; lifted via a non-trapping sequence below instead.
                0b100 | 0b101 | 0b110 | 0b111 => {
                    return self.lift_div_rem(insn, addr, dst, rs1, rs2, width, i64::MIN, ctx);
                }
                _ => unreachable!(),
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Lift DIV/DIVU/REM/REMU via a non-trapping sequence implementing RISC-V's
    /// divide-by-zero and signed MIN/-1 overflow results (SMIR's DivS/DivU trap
    /// like x86 #DE, so the divisor is first sanitized and the special results
    /// are selected afterward). `width` is W64 (64-bit forms only).
    fn lift_div_rem(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        dst: VReg,
        rs1: VReg,
        rs2: VReg,
        width: OpWidth,
        ovf_min: i64,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let funct3 = Self::funct3(insn);
        let signed = matches!(funct3, 0b100 | 0b110); // DIV, REM
        let is_rem = matches!(funct3, 0b110 | 0b111);
        let mut ops = Vec::new();
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let mov = |ctx: &mut LiftContext, ops: &mut Vec<SmirOp>, v: i64| -> VReg {
            let t = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst: t,
                    src: SrcOperand::Imm(v),
                    width,
                },
            ));
            t
        };
        let setcc = |ctx: &mut LiftContext,
                     ops: &mut Vec<SmirOp>,
                     a: VReg,
                     b: i64,
                     cond: Condition|
         -> VReg {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Cmp {
                    src1: a,
                    src2: SrcOperand::Imm(b),
                    width,
                },
            ));
            let r = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::SetCC {
                    dst: r,
                    cond,
                    width: OpWidth::W64,
                },
            ));
            r
        };

        // is_zero = (rs2 == 0)
        let is_zero = setcc(ctx, &mut ops, rs2, 0, Condition::Eq);
        // For signed forms, detect MIN / -1 overflow.
        let (need_special, ovf) = if signed {
            let min = ovf_min;
            let is_min = setcc(ctx, &mut ops, rs1, min, Condition::Eq);
            let is_neg1 = setcc(ctx, &mut ops, rs2, -1, Condition::Eq);
            let ovf = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::And {
                    dst: ovf,
                    src1: is_min,
                    src2: SrcOperand::Reg(is_neg1),
                    width: OpWidth::W64,
                    flags: FlagUpdate::None,
                },
            ));
            let nsp = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Or {
                    dst: nsp,
                    src1: is_zero,
                    src2: SrcOperand::Reg(ovf),
                    width: OpWidth::W64,
                    flags: FlagUpdate::None,
                },
            ));
            (nsp, Some(ovf))
        } else {
            (is_zero, None)
        };

        // safe_divisor = need_special ? 1 : rs2  (avoids /0 and signed MIN/-1).
        let one = mov(ctx, &mut ops, 1);
        let safe = ctx.alloc_vreg();
        ops.push(mk(
            ctx,
            OpKind::Select {
                dst: safe,
                cond: need_special,
                src_true: one,
                src_false: rs2,
                width,
            },
        ));
        // Raw quotient / remainder over the sanitized divisor.
        let raw = ctx.alloc_vreg();
        let divkind = if signed {
            OpKind::DivS {
                quot: if is_rem { ctx.alloc_vreg() } else { raw },
                rem: if is_rem { Some(raw) } else { None },
                src1: rs1,
                src2: SrcOperand::Reg(safe),
                width,
                flags: FlagUpdate::None,
            }
        } else {
            OpKind::DivU {
                quot: if is_rem { ctx.alloc_vreg() } else { raw },
                rem: if is_rem { Some(raw) } else { None },
                src1: rs1,
                src2: SrcOperand::Reg(safe),
                width,
                flags: FlagUpdate::None,
            }
        };
        ops.push(mk(ctx, divkind));

        // Apply the overflow special-case for signed forms.
        let after_ovf = if let Some(ovf) = ovf {
            let ov_val = mov(ctx, &mut ops, if is_rem { 0 } else { i64::MIN });
            let t = ctx.alloc_vreg();
            ops.push(mk(
                ctx,
                OpKind::Select {
                    dst: t,
                    cond: ovf,
                    src_true: ov_val,
                    src_false: raw,
                    width,
                },
            ));
            t
        } else {
            raw
        };
        // Apply the divide-by-zero special-case: REM->dividend, DIV->all-ones.
        let zero_val = if is_rem { rs1 } else { mov(ctx, &mut ops, -1) };
        ops.push(mk(
            ctx,
            OpKind::Select {
                dst,
                cond: is_zero,
                src_true: zero_val,
                src_false: after_ovf,
                width,
            },
        ));

        Ok((ops, ControlFlow::NextInsn))
    }

    /// M extension 32-bit multiply/divide operations
    fn lift_op32_m(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);
        let width = OpWidth::W32;

        // Word div/rem: sign/zero-extend the operands to 64 bits, run the
        // non-trapping div sequence at W64 with a 32-bit overflow min, then
        // sign-extend the low 32 bits of the result into rd. (Operating the div
        // directly at W32 trips the interp's x86-style quotient-overflow #DE.)
        if matches!(funct3, 0b100 | 0b101 | 0b110 | 0b111) {
            let signed = matches!(funct3, 0b100 | 0b110);
            let ext_kind = |dst, src| {
                if signed {
                    OpKind::SignExtend {
                        dst,
                        src,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    }
                } else {
                    OpKind::ZeroExtend {
                        dst,
                        src,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    }
                }
            };
            let mut ops2 = Vec::new();
            let e1 = ctx.alloc_vreg();
            ops2.push(SmirOp::new(ctx.next_op_id(), addr, ext_kind(e1, rs1)));
            let e2 = ctx.alloc_vreg();
            ops2.push(SmirOp::new(ctx.next_op_id(), addr, ext_kind(e2, rs2)));
            let tmp = ctx.alloc_vreg();
            let (dr_ops, cf) =
                self.lift_div_rem(insn, addr, tmp, e1, e2, OpWidth::W64, -(1i64 << 31), ctx)?;
            ops2.extend(dr_ops);
            if let Some(dst) = self.def_x_reg(rd, ctx) {
                ops2.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::SignExtend {
                        dst,
                        src: tmp,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    },
                ));
            }
            return Ok((ops2, cf));
        }

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let tmp = ctx.alloc_vreg();

            let inner_kind = match funct3 {
                0b000 => OpKind::MulS {
                    // MULW
                    dst_lo: tmp,
                    dst_hi: None,
                    src1: rs1,
                    src2: SrcOperand::Reg(rs2),
                    width,
                    flags: FlagUpdate::None,
                },
                _ => {
                    return Err(LiftError::Unsupported {
                        addr,
                        mnemonic: format!("OP-32-M funct3={funct3:#05b}"),
                    });
                }
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, inner_kind));
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::SignExtend {
                    dst,
                    src: tmp,
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Atomic instructions (A extension)
    fn lift_atomic(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let rs2_reg = Self::rs2(insn);
        let funct3 = Self::funct3(insn);
        let funct5 = (insn >> 27) & 0x1F;
        let aq = ((insn >> 26) & 1) != 0;
        let rl = ((insn >> 25) & 1) != 0;

        let rs1 = self.get_x_reg(rs1_reg, ctx);
        let rs2 = self.get_x_reg(rs2_reg, ctx);

        let width = match funct3 {
            0b010 => MemWidth::B4, // 32-bit
            0b011 => MemWidth::B8, // 64-bit
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        };

        let order = match (aq, rl) {
            (false, false) => MemoryOrder::Relaxed,
            (true, false) => MemoryOrder::Acquire,
            (false, true) => MemoryOrder::Release,
            (true, true) => MemoryOrder::AcqRel,
        };

        let mut ops = Vec::new();
        let address = Address::Direct(rs1);

        {
            // AMO/SC have a memory side effect that must occur even when rd==x0
            // (the loaded value is simply discarded), so never gate the whole
            // op on a non-x0 destination — use a throwaway vreg for rd==x0.
            let dst = self.def_x_reg(rd, ctx).unwrap_or_else(|| ctx.alloc_vreg());
            // Word LR/AMO results are sign-extended into rd (SC writes a 0/1
            // status, so it is excluded).
            let needs_sext = width == MemWidth::B4 && funct5 != 0b00011;
            let result = if needs_sext { ctx.alloc_vreg() } else { dst };
            let kind = match funct5 {
                0b00010 => {
                    // LR.W/D (Load Reserved)
                    OpKind::LoadExclusive {
                        dst: result,
                        addr: address,
                        width,
                    }
                }
                0b00011 => {
                    // SC.W/D (Store Conditional)
                    let status = dst; // SC writes status to rd
                    OpKind::StoreExclusive {
                        status,
                        src: rs2,
                        addr: address,
                        width,
                    }
                }
                0b00001 => OpKind::AtomicRmw {
                    // AMOSWAP
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Swap,
                    width,
                    order,
                },
                0b00000 => OpKind::AtomicRmw {
                    // AMOADD
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Add,
                    width,
                    order,
                },
                0b00100 => OpKind::AtomicRmw {
                    // AMOXOR
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Xor,
                    width,
                    order,
                },
                0b01100 => OpKind::AtomicRmw {
                    // AMOAND
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::And,
                    width,
                    order,
                },
                0b01000 => OpKind::AtomicRmw {
                    // AMOOR
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Or,
                    width,
                    order,
                },
                0b10000 => OpKind::AtomicRmw {
                    // AMOMIN
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Min,
                    width,
                    order,
                },
                0b10100 => OpKind::AtomicRmw {
                    // AMOMAX
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Max,
                    width,
                    order,
                },
                0b11000 => OpKind::AtomicRmw {
                    // AMOMINU
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Umin,
                    width,
                    order,
                },
                0b11100 => OpKind::AtomicRmw {
                    // AMOMAXU
                    dst: result,
                    addr: address,
                    src: rs2,
                    op: AtomicOp::Umax,
                    width,
                    order,
                },
                _ => {
                    return Err(LiftError::InvalidEncoding {
                        addr,
                        bytes: insn.to_le_bytes().to_vec(),
                    });
                }
            };

            ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
            if needs_sext {
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::SignExtend {
                        dst,
                        src: result,
                        from_width: OpWidth::W32,
                        to_width: OpWidth::W64,
                    },
                ));
            }
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// Fence instructions
    fn lift_fence(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let funct3 = Self::funct3(insn);

        let mut ops = Vec::new();

        match funct3 {
            0b000 => {
                // FENCE
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::Fence {
                        kind: FenceKind::Full,
                    },
                ));
            }
            0b001 => {
                // FENCE.I (instruction fence)
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::Fence {
                        kind: FenceKind::ISync,
                    },
                ));
            }
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    /// System instructions (ECALL, EBREAK, CSR ops)
    fn lift_system(
        &mut self,
        insn: u32,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::rd(insn);
        let rs1_reg = Self::rs1(insn);
        let funct3 = Self::funct3(insn);
        let csr = ((insn >> 20) & 0xFFF) as u32;

        let mut ops = Vec::new();

        match funct3 {
            0b000 => {
                // ECALL, EBREAK, privileged instructions
                match insn {
                    0x00000073 => {
                        // ECALL
                        // System call - use a7 as syscall number
                        let syscall_num = self.get_x_reg(17, ctx); // a7
                        let args = (10..=16).map(|r| self.get_x_reg(r, ctx)).collect();
                        ops.push(SmirOp::new(
                            ctx.next_op_id(),
                            addr,
                            OpKind::Syscall {
                                num: syscall_num,
                                args,
                            },
                        ));
                        return Ok((ops, ControlFlow::NextInsn));
                    }
                    0x00100073 => {
                        // EBREAK
                        ops.push(SmirOp::new(ctx.next_op_id(), addr, OpKind::Breakpoint));
                        return Ok((ops, ControlFlow::NextInsn));
                    }
                    _ => {
                        return Err(LiftError::Unsupported {
                            addr,
                            mnemonic: "privileged instruction".to_string(),
                        });
                    }
                }
            }
            // CSR instructions (Zicsr). Lifted+verified for the application-
            // visible CSRs that SMIR models: fcsr (0x003, read+write) and the
            // read-only fflags/frm/vl/vtype/vlenb. Other CSRs (privileged state,
            // counters, and the fflags/frm/vector-fixedpoint *writes* that alias
            // fcsr/vcsr) are honest gaps. Read-old → rd, then conditionally write
            // the new value; the mask is applied in-IR because the vreg writeback
            // bypasses write_arch_reg's masking.
            0b001 | 0b010 | 0b011 | 0b101 | 0b110 | 0b111 => {
                let is_imm = funct3 & 0b100 != 0;
                let op = funct3 & 0b011; // 1=rw, 2=rs, 3=rc
                let zimm = rs1_reg as i64; // 5-bit immediate (csrr*i forms)
                let writes = match op {
                    1 => true, // csrrw / csrrwi always write
                    _ => {
                        if is_imm {
                            zimm != 0
                        } else {
                            rs1_reg != 0
                        }
                    }
                };
                // fcsr-family CSRs are a (shift, field-mask) view of fcsr (0x003);
                // the read-only CSRs are read straight from their arch reg.
                let fcsr_field: Option<(i64, i64)> = match csr {
                    0x003 => Some((0, 0xff)), // fcsr
                    0x001 => Some((0, 0x1f)), // fflags
                    0x002 => Some((5, 0x7)),  // frm
                    _ => None,
                };
                let modeled_ro = matches!(csr, 0xc20 | 0xc21 | 0xc22);
                let w = self.op_width();
                let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
                let src_operand = |slf: &Self, ctx: &mut LiftContext| -> SrcOperand {
                    if is_imm {
                        SrcOperand::Imm(zimm)
                    } else {
                        SrcOperand::Reg(slf.get_x_reg(rs1_reg, ctx))
                    }
                };

                if let Some((shift, mask)) = fcsr_field {
                    // Read the whole fcsr, extract the addressed field → rd.
                    let fcsr_cur = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::Csr(0x003)));
                    let old_full = ctx.alloc_vreg();
                    ops.push(mk(
                        ctx,
                        OpKind::Mov {
                            dst: old_full,
                            src: SrcOperand::Reg(fcsr_cur),
                            width: w,
                        },
                    ));
                    let shifted = if shift != 0 {
                        let s = ctx.alloc_vreg();
                        ops.push(mk(
                            ctx,
                            OpKind::Shr {
                                dst: s,
                                src: old_full,
                                amount: SrcOperand::Imm(shift),
                                width: w,
                                flags: FlagUpdate::None,
                            },
                        ));
                        s
                    } else {
                        old_full
                    };
                    let old_field = ctx.alloc_vreg();
                    ops.push(mk(
                        ctx,
                        OpKind::And {
                            dst: old_field,
                            src1: shifted,
                            src2: SrcOperand::Imm(mask),
                            width: w,
                            flags: FlagUpdate::None,
                        },
                    ));
                    // Read rs1 BEFORE writing rd (CSR reads the source first; rd may alias rs1).
                    let src = if writes {
                        Some(src_operand(self, ctx))
                    } else {
                        None
                    };
                    if let Some(dst) = self.def_x_reg(rd, ctx) {
                        ops.push(mk(
                            ctx,
                            OpKind::Mov {
                                dst,
                                src: SrcOperand::Reg(old_field),
                                width: w,
                            },
                        ));
                    }
                    if let Some(src) = src {
                        // new_field (pre-mask) = src | (old|src) | (old&~src).
                        let nf = ctx.alloc_vreg();
                        match op {
                            1 => ops.push(mk(
                                ctx,
                                OpKind::Mov {
                                    dst: nf,
                                    src,
                                    width: w,
                                },
                            )),
                            2 => ops.push(mk(
                                ctx,
                                OpKind::Or {
                                    dst: nf,
                                    src1: old_field,
                                    src2: src,
                                    width: w,
                                    flags: FlagUpdate::None,
                                },
                            )),
                            _ => ops.push(mk(
                                ctx,
                                OpKind::AndNot {
                                    dst: nf,
                                    src1: old_field,
                                    src2: src,
                                    width: w,
                                    flags: FlagUpdate::None,
                                },
                            )),
                        }
                        let nfm = ctx.alloc_vreg();
                        ops.push(mk(
                            ctx,
                            OpKind::And {
                                dst: nfm,
                                src1: nf,
                                src2: SrcOperand::Imm(mask),
                                width: w,
                                flags: FlagUpdate::None,
                            },
                        ));
                        // new_fcsr = (old_full & ~(mask<<shift)) | (nfm << shift).
                        let cleared = ctx.alloc_vreg();
                        ops.push(mk(
                            ctx,
                            OpKind::AndNot {
                                dst: cleared,
                                src1: old_full,
                                src2: SrcOperand::Imm(mask << shift),
                                width: w,
                                flags: FlagUpdate::None,
                            },
                        ));
                        let placed = if shift != 0 {
                            let p = ctx.alloc_vreg();
                            ops.push(mk(
                                ctx,
                                OpKind::Shl {
                                    dst: p,
                                    src: nfm,
                                    amount: SrcOperand::Imm(shift),
                                    width: w,
                                    flags: FlagUpdate::None,
                                },
                            ));
                            p
                        } else {
                            nfm
                        };
                        let new_full = ctx.alloc_vreg();
                        ops.push(mk(
                            ctx,
                            OpKind::Or {
                                dst: new_full,
                                src1: cleared,
                                src2: SrcOperand::Reg(placed),
                                width: w,
                                flags: FlagUpdate::None,
                            },
                        ));
                        let csr_dst = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::Csr(0x003)));
                        ops.push(mk(
                            ctx,
                            OpKind::Mov {
                                dst: csr_dst,
                                src: SrcOperand::Reg(new_full),
                                width: w,
                            },
                        ));
                    }
                } else if modeled_ro && !writes {
                    // Read-only CSR: rd = csr value (a write would trap on hardware).
                    let cur = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::Csr(csr as u16)));
                    if let Some(dst) = self.def_x_reg(rd, ctx) {
                        ops.push(mk(
                            ctx,
                            OpKind::Mov {
                                dst,
                                src: SrcOperand::Reg(cur),
                                width: w,
                            },
                        ));
                    }
                } else {
                    return Err(LiftError::Unsupported {
                        addr,
                        mnemonic: format!("csr {csr:#x}"),
                    });
                }
                return Ok((ops, ControlFlow::NextInsn));
            }
            _ => {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }
        }
    }

    // ========================================================================
    // Compressed Instructions (C extension)
    // ========================================================================

    /// Lift a 16-bit compressed instruction
    fn lift_insn16(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let op = insn & 0x3;
        let funct3 = (insn >> 13) & 0x7;

        match (op, funct3) {
            // Quadrant 0
            (0b00, 0b000) if insn != 0 => self.lift_c_addi4spn(insn, addr, ctx),
            (0b00, 0b100) => self.lift_c_zcb_ldst(insn, addr, ctx),
            (0b00, 0b010) => self.lift_c_lw(insn, addr, ctx),
            (0b00, 0b001) if self.extensions.d => self.lift_c_fp_ldst(insn, addr, ctx), // c.fld
            (0b00, 0b011) if self.xlen == 64 => self.lift_c_ld(insn, addr, ctx),
            (0b00, 0b110) => self.lift_c_sw(insn, addr, ctx),
            (0b00, 0b101) if self.extensions.d => self.lift_c_fp_ldst(insn, addr, ctx), // c.fsd
            (0b00, 0b111) if self.xlen == 64 => self.lift_c_sd(insn, addr, ctx),

            // Quadrant 1
            (0b01, 0b000) => self.lift_c_addi(insn, addr, ctx), // C.NOP/C.ADDI
            (0b01, 0b001) if self.xlen == 64 => self.lift_c_addiw(insn, addr, ctx),
            (0b01, 0b001) if self.xlen == 32 => self.lift_c_jal(insn, addr, ctx),
            (0b01, 0b010) => self.lift_c_li(insn, addr, ctx),
            (0b01, 0b011) => self.lift_c_lui_addi16sp(insn, addr, ctx),
            (0b01, 0b100) => self.lift_c_misc_alu(insn, addr, ctx),
            (0b01, 0b101) => self.lift_c_j(insn, addr, ctx),
            (0b01, 0b110) => self.lift_c_beqz(insn, addr, ctx),
            (0b01, 0b111) => self.lift_c_bnez(insn, addr, ctx),

            // Quadrant 2
            (0b10, 0b000) => self.lift_c_slli(insn, addr, ctx),
            (0b10, 0b010) => self.lift_c_lwsp(insn, addr, ctx),
            (0b10, 0b001) if self.extensions.d => self.lift_c_fp_ldst(insn, addr, ctx), // c.fldsp
            (0b10, 0b011) if self.xlen == 64 => self.lift_c_ldsp(insn, addr, ctx),
            (0b10, 0b100) => self.lift_c_jr_mv_add(insn, addr, ctx),
            (0b10, 0b110) => self.lift_c_swsp(insn, addr, ctx),
            (0b10, 0b101) if self.extensions.d => self.lift_c_fp_ldst(insn, addr, ctx), // c.fsdsp
            (0b10, 0b111) if self.xlen == 64 => self.lift_c_sdsp(insn, addr, ctx),

            _ => Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            }),
        }
    }

    /// Get compressed register (rd', rs1', rs2' - maps 0-7 to x8-x15)
    fn creg(r: u8) -> u8 {
        8 + (r & 0x7)
    }

    // Compressed FP load/store (c.fld/c.fsd/c.fldsp/c.fsdsp). Doubles need no
    // NaN-boxing; decoded through the rax decoder for the resolved operands.
    fn lift_c_fp_ldst(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = crate::riscv::decode::decode_compressed(insn, xl, &RvIsa::rv64gc());
        if d.is_illegal() {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }
        let base = self.get_x_reg(d.rs1, ctx);
        let address = Address::BaseOffset {
            base,
            offset: d.imm,
            disp_size: DispSize::Auto,
        };
        let mk = |ctx: &mut LiftContext, k: OpKind| SmirOp::new(ctx.next_op_id(), addr, k);
        let mut ops = Vec::new();
        match d.op {
            RvOp::Fld => {
                let fd = ctx.define_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rd)));
                ops.push(mk(
                    ctx,
                    OpKind::Load {
                        dst: fd,
                        addr: address,
                        width: MemWidth::B8,
                        sign: SignExtend::Zero,
                    },
                ));
            }
            RvOp::Fsd => {
                let fs = ctx.get_arch_reg(ArchReg::RiscV(RiscVReg::F(d.rs2)));
                ops.push(mk(
                    ctx,
                    OpKind::Store {
                        src: fs,
                        addr: address,
                        width: MemWidth::B8,
                    },
                ));
            }
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: format!("{:?}", d.op),
                });
            }
        }
        Ok((ops, ControlFlow::NextInsn))
    }

    // Zcb quadrant-0 byte/half loads and stores (c.lbu/lhu/lh/sb/sh). Decoded
    // through the rax decoder for the precise op and resolved rd'/rs1'/rs2'/imm.
    fn lift_c_zcb_ldst(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let xl = if self.xlen == 64 {
            RvXlen::Rv64
        } else {
            RvXlen::Rv32
        };
        let d = crate::riscv::decode::decode_compressed(insn, xl, &RvIsa::rv64gc());
        if d.is_illegal() {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }
        let base = self.get_x_reg(d.rs1, ctx);
        let address = Address::BaseOffset {
            base,
            offset: d.imm,
            disp_size: DispSize::Auto,
        };
        let mut ops = Vec::new();
        let (width, sign, is_store) = match d.op {
            RvOp::Lbu => (MemWidth::B1, SignExtend::Zero, false),
            RvOp::Lhu => (MemWidth::B2, SignExtend::Zero, false),
            RvOp::Lh => (MemWidth::B2, SignExtend::Sign, false),
            RvOp::Sb => (MemWidth::B1, SignExtend::Zero, true),
            RvOp::Sh => (MemWidth::B2, SignExtend::Zero, true),
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: format!("{:?}", d.op),
                });
            }
        };
        if is_store {
            let src = self.get_x_reg(d.rs2, ctx);
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Store {
                    src,
                    addr: address,
                    width,
                },
            ));
        } else if let Some(dst) = self.def_x_reg(d.rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Load {
                    dst,
                    addr: address,
                    width,
                    sign,
                },
            ));
        }
        Ok((ops, ControlFlow::NextInsn))
    }

    // C.ADDI4SPN: rd' = sp + nzuimm
    fn lift_c_addi4spn(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::creg(((insn >> 2) & 0x7) as u8);
        let nzuimm = ((((insn >> 5) & 1) << 3)
            | (((insn >> 6) & 1) << 2)
            | (((insn >> 7) & 0xF) << 6)
            | (((insn >> 11) & 0x3) << 4)) as i64;

        if nzuimm == 0 {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }

        let sp = self.get_x_reg(2, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Add {
                    dst,
                    src1: sp,
                    src2: SrcOperand::Imm(nzuimm),
                    width: self.op_width(),
                    flags: FlagUpdate::None,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.LW: rd' = mem[rs1' + uimm]
    fn lift_c_lw(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::creg(((insn >> 2) & 0x7) as u8);
        let rs1 = Self::creg(((insn >> 7) & 0x7) as u8);
        let uimm = ((((insn >> 5) & 1) << 6)
            | (((insn >> 6) & 1) << 2)
            | (((insn >> 10) & 0x7) << 3)) as i64;

        let base = self.get_x_reg(rs1, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Load {
                    dst,
                    addr: Address::BaseOffset {
                        base,
                        offset: uimm,
                        disp_size: DispSize::Auto,
                    },
                    width: MemWidth::B4,
                    sign: SignExtend::Sign,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.LD: rd' = mem[rs1' + uimm] (RV64)
    fn lift_c_ld(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::creg(((insn >> 2) & 0x7) as u8);
        let rs1 = Self::creg(((insn >> 7) & 0x7) as u8);
        let uimm = ((((insn >> 5) & 0x3) << 6) | (((insn >> 10) & 0x7) << 3)) as i64;

        let base = self.get_x_reg(rs1, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Load {
                    dst,
                    addr: Address::BaseOffset {
                        base,
                        offset: uimm,
                        disp_size: DispSize::Auto,
                    },
                    width: MemWidth::B8,
                    sign: SignExtend::Zero,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.SW: mem[rs1' + uimm] = rs2'
    fn lift_c_sw(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs2 = Self::creg(((insn >> 2) & 0x7) as u8);
        let rs1 = Self::creg(((insn >> 7) & 0x7) as u8);
        let uimm = ((((insn >> 5) & 1) << 6)
            | (((insn >> 6) & 1) << 2)
            | (((insn >> 10) & 0x7) << 3)) as i64;

        let base = self.get_x_reg(rs1, ctx);
        let src = self.get_x_reg(rs2, ctx);

        let ops = vec![SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Store {
                src,
                addr: Address::BaseOffset {
                    base,
                    offset: uimm,
                    disp_size: DispSize::Auto,
                },
                width: MemWidth::B4,
            },
        )];

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.SD: mem[rs1' + uimm] = rs2' (RV64)
    fn lift_c_sd(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs2 = Self::creg(((insn >> 2) & 0x7) as u8);
        let rs1 = Self::creg(((insn >> 7) & 0x7) as u8);
        let uimm = ((((insn >> 5) & 0x3) << 6) | (((insn >> 10) & 0x7) << 3)) as i64;

        let base = self.get_x_reg(rs1, ctx);
        let src = self.get_x_reg(rs2, ctx);

        let ops = vec![SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Store {
                src,
                addr: Address::BaseOffset {
                    base,
                    offset: uimm,
                    disp_size: DispSize::Auto,
                },
                width: MemWidth::B8,
            },
        )];

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.ADDI / C.NOP
    fn lift_c_addi(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let imm = {
            let imm5 = ((insn >> 12) & 1) as i8;
            let imm4_0 = ((insn >> 2) & 0x1F) as i8;
            ((((imm5 << 5) | imm4_0) << 2) >> 2) as i64 // sign-extend from bit 5
        };

        if rd == 0 {
            // C.NOP
            return Ok((
                vec![SmirOp::new(ctx.next_op_id(), addr, OpKind::Nop)],
                ControlFlow::NextInsn,
            ));
        }

        let rs1 = self.get_x_reg(rd, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Add {
                    dst,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width: self.op_width(),
                    flags: FlagUpdate::None,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.ADDIW (RV64)
    fn lift_c_addiw(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let imm = {
            let imm5 = ((insn >> 12) & 1) as i8;
            let imm4_0 = ((insn >> 2) & 0x1F) as i8;
            ((((imm5 << 5) | imm4_0) << 2) >> 2) as i64 // sign-extend from bit 5
        };

        let rs1 = self.get_x_reg(rd, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            let tmp = ctx.alloc_vreg();
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Add {
                    dst: tmp,
                    src1: rs1,
                    src2: SrcOperand::Imm(imm),
                    width: OpWidth::W32,
                    flags: FlagUpdate::None,
                },
            ));
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::SignExtend {
                    dst,
                    src: tmp,
                    from_width: OpWidth::W32,
                    to_width: OpWidth::W64,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.JAL (RV32 only)
    fn lift_c_jal(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let imm = self.c_j_offset(insn);
        let target = (addr as i64).wrapping_add(imm) as u64;
        let return_addr = addr + 2;

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(1, ctx) {
            // ra
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Imm(return_addr as i64),
                    width: self.op_width(),
                },
            ));
        }

        Ok((ops, ControlFlow::DirectBranch(target)))
    }

    // C.LI
    fn lift_c_li(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let imm = {
            let imm5 = ((insn >> 12) & 1) as i8;
            let imm4_0 = ((insn >> 2) & 0x1F) as i8;
            ((((imm5 << 5) | imm4_0) << 2) >> 2) as i64 // sign-extend from bit 5
        };

        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Mov {
                    dst,
                    src: SrcOperand::Imm(imm),
                    width: self.op_width(),
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.LUI / C.ADDI16SP
    fn lift_c_lui_addi16sp(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;

        let mut ops = Vec::new();

        if rd == 2 {
            // C.ADDI16SP
            let imm = {
                let bit9 = ((insn >> 12) & 1) as i16;
                let bit4 = ((insn >> 6) & 1) as i16;
                let bit6 = ((insn >> 5) & 1) as i16;
                let bit8_7 = ((insn >> 3) & 0x3) as i16;
                let bit5 = ((insn >> 2) & 1) as i16;
                let raw = (bit9 << 9) | (bit8_7 << 7) | (bit6 << 6) | (bit5 << 5) | (bit4 << 4);
                ((raw << 6) >> 6) as i64 // Sign-extend from 10 bits
            };

            if imm == 0 {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }

            let sp = self.get_x_reg(2, ctx);
            if let Some(dst) = self.def_x_reg(2, ctx) {
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::Add {
                        dst,
                        src1: sp,
                        src2: SrcOperand::Imm(imm),
                        width: self.op_width(),
                        flags: FlagUpdate::None,
                    },
                ));
            }
        } else {
            // C.LUI
            let nzimm = {
                let bit17 = ((insn >> 12) & 1) as i32;
                let bits16_12 = ((insn >> 2) & 0x1F) as i32;
                let raw = (bit17 << 17) | (bits16_12 << 12);
                ((raw << 14) >> 14) as i64 // Sign-extend from 18 bits
            };

            if nzimm == 0 {
                return Err(LiftError::InvalidEncoding {
                    addr,
                    bytes: insn.to_le_bytes().to_vec(),
                });
            }

            if let Some(dst) = self.def_x_reg(rd, ctx) {
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::Mov {
                        dst,
                        src: SrcOperand::Imm(nzimm),
                        width: self.op_width(),
                    },
                ));
            }
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.SRLI, C.SRAI, C.ANDI, C.SUB, C.XOR, C.OR, C.AND, C.SUBW, C.ADDW
    fn lift_c_misc_alu(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = Self::creg(((insn >> 7) & 0x7) as u8);
        let funct2 = (insn >> 10) & 0x3;

        let rs1 = self.get_x_reg(rd, ctx);
        let mut ops = Vec::new();

        match funct2 {
            0b00 | 0b01 => {
                // C.SRLI / C.SRAI
                let shamt = ((((insn >> 12) & 1) << 5) | ((insn >> 2) & 0x1F)) as i64;
                if let Some(dst) = self.def_x_reg(rd, ctx) {
                    let kind = if funct2 == 0b00 {
                        OpKind::Shr {
                            dst,
                            src: rs1,
                            amount: SrcOperand::Imm(shamt),
                            width: self.op_width(),
                            flags: FlagUpdate::None,
                        }
                    } else {
                        OpKind::Sar {
                            dst,
                            src: rs1,
                            amount: SrcOperand::Imm(shamt),
                            width: self.op_width(),
                            flags: FlagUpdate::None,
                        }
                    };
                    ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
                }
            }
            0b10 => {
                // C.ANDI
                let imm = {
                    let imm5 = ((insn >> 12) & 1) as i8;
                    let imm4_0 = ((insn >> 2) & 0x1F) as i8;
                    ((((imm5 << 5) | imm4_0) << 2) >> 2) as i64 // sign-extend from bit 5
                };
                if let Some(dst) = self.def_x_reg(rd, ctx) {
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::And {
                            dst,
                            src1: rs1,
                            src2: SrcOperand::Imm(imm),
                            width: self.op_width(),
                            flags: FlagUpdate::None,
                        },
                    ));
                }
            }
            0b11 => {
                // Register-register ops
                let rs2 = Self::creg(((insn >> 2) & 0x7) as u8);
                let rs2_val = self.get_x_reg(rs2, ctx);
                let funct2b = (insn >> 5) & 0x3;
                let funct1 = (insn >> 12) & 1;

                if let Some(dst) = self.def_x_reg(rd, ctx) {
                    if funct1 == 0 {
                        let kind = match funct2b {
                            0b00 => OpKind::Sub {
                                dst,
                                src1: rs1,
                                src2: SrcOperand::Reg(rs2_val),
                                width: self.op_width(),
                                flags: FlagUpdate::None,
                            },
                            0b01 => OpKind::Xor {
                                dst,
                                src1: rs1,
                                src2: SrcOperand::Reg(rs2_val),
                                width: self.op_width(),
                                flags: FlagUpdate::None,
                            },
                            0b10 => OpKind::Or {
                                dst,
                                src1: rs1,
                                src2: SrcOperand::Reg(rs2_val),
                                width: self.op_width(),
                                flags: FlagUpdate::None,
                            },
                            0b11 => OpKind::And {
                                dst,
                                src1: rs1,
                                src2: SrcOperand::Reg(rs2_val),
                                width: self.op_width(),
                                flags: FlagUpdate::None,
                            },
                            _ => unreachable!(),
                        };
                        ops.push(SmirOp::new(ctx.next_op_id(), addr, kind));
                    } else if self.xlen == 64 {
                        let w = self.op_width();
                        match funct2b {
                            // C.SUBW / C.ADDW: W32 op then sign-extend.
                            0b00 | 0b01 => {
                                let tmp = ctx.alloc_vreg();
                                let k = if funct2b == 0b00 {
                                    OpKind::Sub {
                                        dst: tmp,
                                        src1: rs1,
                                        src2: SrcOperand::Reg(rs2_val),
                                        width: OpWidth::W32,
                                        flags: FlagUpdate::None,
                                    }
                                } else {
                                    OpKind::Add {
                                        dst: tmp,
                                        src1: rs1,
                                        src2: SrcOperand::Reg(rs2_val),
                                        width: OpWidth::W32,
                                        flags: FlagUpdate::None,
                                    }
                                };
                                ops.push(SmirOp::new(ctx.next_op_id(), addr, k));
                                ops.push(SmirOp::new(
                                    ctx.next_op_id(),
                                    addr,
                                    OpKind::SignExtend {
                                        dst,
                                        src: tmp,
                                        from_width: OpWidth::W32,
                                        to_width: OpWidth::W64,
                                    },
                                ));
                            }
                            // Zcb c.mul.
                            0b10 => ops.push(SmirOp::new(
                                ctx.next_op_id(),
                                addr,
                                OpKind::MulS {
                                    dst_lo: dst,
                                    dst_hi: None,
                                    src1: rs1,
                                    src2: SrcOperand::Reg(rs2_val),
                                    width: w,
                                    flags: FlagUpdate::None,
                                },
                            )),
                            // Zcb unary: c.zext.b/sext.b/zext.h/sext.h/zext.w/not.
                            _ => {
                                let sub = (insn >> 2) & 0x7;
                                let k = match sub {
                                    0b000 => OpKind::And {
                                        dst,
                                        src1: rs1,
                                        src2: SrcOperand::Imm(0xff),
                                        width: w,
                                        flags: FlagUpdate::None,
                                    },
                                    0b001 => OpKind::SignExtend {
                                        dst,
                                        src: rs1,
                                        from_width: OpWidth::W8,
                                        to_width: w,
                                    },
                                    0b010 => OpKind::ZeroExtend {
                                        dst,
                                        src: rs1,
                                        from_width: OpWidth::W16,
                                        to_width: w,
                                    },
                                    0b011 => OpKind::SignExtend {
                                        dst,
                                        src: rs1,
                                        from_width: OpWidth::W16,
                                        to_width: w,
                                    },
                                    0b100 => OpKind::ZeroExtend {
                                        dst,
                                        src: rs1,
                                        from_width: OpWidth::W32,
                                        to_width: w,
                                    },
                                    0b101 => OpKind::Xor {
                                        dst,
                                        src1: rs1,
                                        src2: SrcOperand::Imm(-1),
                                        width: w,
                                        flags: FlagUpdate::None,
                                    },
                                    _ => {
                                        return Err(LiftError::Unsupported {
                                            addr,
                                            mnemonic: format!("c.zcb sub={sub:#05b}"),
                                        });
                                    }
                                };
                                ops.push(SmirOp::new(ctx.next_op_id(), addr, k));
                            }
                        }
                    } else {
                        return Err(LiftError::InvalidEncoding {
                            addr,
                            bytes: insn.to_le_bytes().to_vec(),
                        });
                    }
                }
            }
            _ => unreachable!(),
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // Extract C.J / C.JAL offset
    fn c_j_offset(&self, insn: u16) -> i64 {
        let bit11 = ((insn >> 12) & 1) as i16;
        let bit4 = ((insn >> 11) & 1) as i16;
        let bit9_8 = ((insn >> 9) & 0x3) as i16;
        let bit10 = ((insn >> 8) & 1) as i16;
        let bit6 = ((insn >> 7) & 1) as i16;
        let bit7 = ((insn >> 6) & 1) as i16;
        let bit3_1 = ((insn >> 3) & 0x7) as i16;
        let bit5 = ((insn >> 2) & 1) as i16;

        let raw = (bit11 << 11)
            | (bit10 << 10)
            | (bit9_8 << 8)
            | (bit7 << 7)
            | (bit6 << 6)
            | (bit5 << 5)
            | (bit4 << 4)
            | (bit3_1 << 1);
        ((raw << 4) >> 4) as i64 // Sign-extend from 12 bits
    }

    // C.J
    fn lift_c_j(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let imm = self.c_j_offset(insn);
        let target = (addr as i64).wrapping_add(imm) as u64;
        Ok((vec![], ControlFlow::DirectBranch(target)))
    }

    // C.BEQZ
    fn lift_c_beqz(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs1 = Self::creg(((insn >> 7) & 0x7) as u8);
        let imm = self.c_branch_offset(insn);
        let target = (addr as i64).wrapping_add(imm) as u64;
        let fallthrough = addr + 2;

        let rs1_val = self.get_x_reg(rs1, ctx);
        let mut ops = Vec::new();

        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Cmp {
                src1: rs1_val,
                src2: SrcOperand::Imm(0),
                width: self.op_width(),
            },
        ));

        let cond_reg = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::SetCC {
                dst: cond_reg,
                cond: Condition::Eq,
                width: OpWidth::W8,
            },
        ));

        Ok((
            ops,
            ControlFlow::CondBranchReg {
                cond: cond_reg,
                taken: target,
                not_taken: fallthrough,
            },
        ))
    }

    // C.BNEZ
    fn lift_c_bnez(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs1 = Self::creg(((insn >> 7) & 0x7) as u8);
        let imm = self.c_branch_offset(insn);
        let target = (addr as i64).wrapping_add(imm) as u64;
        let fallthrough = addr + 2;

        let rs1_val = self.get_x_reg(rs1, ctx);
        let mut ops = Vec::new();

        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Cmp {
                src1: rs1_val,
                src2: SrcOperand::Imm(0),
                width: self.op_width(),
            },
        ));

        let cond_reg = ctx.alloc_vreg();
        ops.push(SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::SetCC {
                dst: cond_reg,
                cond: Condition::Ne,
                width: OpWidth::W8,
            },
        ));

        Ok((
            ops,
            ControlFlow::CondBranchReg {
                cond: cond_reg,
                taken: target,
                not_taken: fallthrough,
            },
        ))
    }

    fn c_branch_offset(&self, insn: u16) -> i64 {
        let bit8 = ((insn >> 12) & 1) as i16;
        let bit4_3 = ((insn >> 10) & 0x3) as i16;
        let bit7_6 = ((insn >> 5) & 0x3) as i16;
        let bit2_1 = ((insn >> 3) & 0x3) as i16;
        let bit5 = ((insn >> 2) & 1) as i16;

        let raw = (bit8 << 8) | (bit7_6 << 6) | (bit5 << 5) | (bit4_3 << 3) | (bit2_1 << 1);
        ((raw << 7) >> 7) as i64 // Sign-extend from 9 bits
    }

    // C.SLLI
    fn lift_c_slli(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let shamt = ((((insn >> 12) & 1) << 5) | ((insn >> 2) & 0x1F)) as i64;

        let rs1 = self.get_x_reg(rd, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Shl {
                    dst,
                    src: rs1,
                    amount: SrcOperand::Imm(shamt),
                    width: self.op_width(),
                    flags: FlagUpdate::None,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.LWSP
    fn lift_c_lwsp(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let uimm = ((((insn >> 12) & 1) << 5)
            | (((insn >> 4) & 0x7) << 2)
            | (((insn >> 2) & 0x3) << 6)) as i64;

        if rd == 0 {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }

        let sp = self.get_x_reg(2, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Load {
                    dst,
                    addr: Address::BaseOffset {
                        base: sp,
                        offset: uimm,
                        disp_size: DispSize::Auto,
                    },
                    width: MemWidth::B4,
                    sign: SignExtend::Sign,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.LDSP (RV64)
    fn lift_c_ldsp(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let uimm = ((((insn >> 12) & 1) << 5)
            | (((insn >> 5) & 0x3) << 3)
            | (((insn >> 2) & 0x7) << 6)) as i64;

        if rd == 0 {
            return Err(LiftError::InvalidEncoding {
                addr,
                bytes: insn.to_le_bytes().to_vec(),
            });
        }

        let sp = self.get_x_reg(2, ctx);
        let mut ops = Vec::new();

        if let Some(dst) = self.def_x_reg(rd, ctx) {
            ops.push(SmirOp::new(
                ctx.next_op_id(),
                addr,
                OpKind::Load {
                    dst,
                    addr: Address::BaseOffset {
                        base: sp,
                        offset: uimm,
                        disp_size: DispSize::Auto,
                    },
                    width: MemWidth::B8,
                    sign: SignExtend::Zero,
                },
            ));
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.JR, C.MV, C.JALR, C.ADD
    fn lift_c_jr_mv_add(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rd = ((insn >> 7) & 0x1F) as u8;
        let rs2 = ((insn >> 2) & 0x1F) as u8;
        let bit12 = (insn >> 12) & 1;

        let mut ops = Vec::new();

        if bit12 == 0 {
            if rs2 == 0 {
                // C.JR
                if rd == 0 {
                    return Err(LiftError::InvalidEncoding {
                        addr,
                        bytes: insn.to_le_bytes().to_vec(),
                    });
                }
                // C.JR == JALR x0, rd, 0: the target clears bit 0.
                let rs1 = self.get_x_reg(rd, ctx);
                let aligned = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::And {
                        dst: aligned,
                        src1: rs1,
                        src2: SrcOperand::Imm(!1i64),
                        width: self.op_width(),
                        flags: FlagUpdate::None,
                    },
                ));
                return Ok((ops, ControlFlow::IndirectBranch { target: aligned }));
            } else {
                // C.MV
                let rs2_val = self.get_x_reg(rs2, ctx);
                if let Some(dst) = self.def_x_reg(rd, ctx) {
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Mov {
                            dst,
                            src: SrcOperand::Reg(rs2_val),
                            width: self.op_width(),
                        },
                    ));
                }
            }
        } else {
            if rs2 == 0 && rd == 0 {
                // C.EBREAK
                ops.push(SmirOp::new(ctx.next_op_id(), addr, OpKind::Breakpoint));
            } else if rs2 == 0 {
                // C.JALR == JALR x1, rd, 0: link to x1, target clears bit 0.
                let rs1 = self.get_x_reg(rd, ctx);
                let aligned = ctx.alloc_vreg();
                ops.push(SmirOp::new(
                    ctx.next_op_id(),
                    addr,
                    OpKind::And {
                        dst: aligned,
                        src1: rs1,
                        src2: SrcOperand::Imm(!1i64),
                        width: self.op_width(),
                        flags: FlagUpdate::None,
                    },
                ));
                let return_addr = addr + 2;

                if let Some(ra) = self.def_x_reg(1, ctx) {
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Mov {
                            dst: ra,
                            src: SrcOperand::Imm(return_addr as i64),
                            width: self.op_width(),
                        },
                    ));
                }

                return Ok((ops, ControlFlow::IndirectBranch { target: aligned }));
            } else {
                // C.ADD
                let rs1 = self.get_x_reg(rd, ctx);
                let rs2_val = self.get_x_reg(rs2, ctx);
                if let Some(dst) = self.def_x_reg(rd, ctx) {
                    ops.push(SmirOp::new(
                        ctx.next_op_id(),
                        addr,
                        OpKind::Add {
                            dst,
                            src1: rs1,
                            src2: SrcOperand::Reg(rs2_val),
                            width: self.op_width(),
                            flags: FlagUpdate::None,
                        },
                    ));
                }
            }
        }

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.SWSP
    fn lift_c_swsp(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs2 = ((insn >> 2) & 0x1F) as u8;
        let uimm = ((((insn >> 9) & 0xF) << 2) | (((insn >> 7) & 0x3) << 6)) as i64;

        let sp = self.get_x_reg(2, ctx);
        let src = self.get_x_reg(rs2, ctx);

        let ops = vec![SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Store {
                src,
                addr: Address::BaseOffset {
                    base: sp,
                    offset: uimm,
                    disp_size: DispSize::Auto,
                },
                width: MemWidth::B4,
            },
        )];

        Ok((ops, ControlFlow::NextInsn))
    }

    // C.SDSP (RV64)
    fn lift_c_sdsp(
        &mut self,
        insn: u16,
        addr: GuestAddr,
        ctx: &mut LiftContext,
    ) -> Result<(Vec<SmirOp>, ControlFlow), LiftError> {
        let rs2 = ((insn >> 2) & 0x1F) as u8;
        let uimm = ((((insn >> 10) & 0x7) << 3) | (((insn >> 7) & 0x7) << 6)) as i64;

        let sp = self.get_x_reg(2, ctx);
        let src = self.get_x_reg(rs2, ctx);

        let ops = vec![SmirOp::new(
            ctx.next_op_id(),
            addr,
            OpKind::Store {
                src,
                addr: Address::BaseOffset {
                    base: sp,
                    offset: uimm,
                    disp_size: DispSize::Auto,
                },
                width: MemWidth::B8,
            },
        )];

        Ok((ops, ControlFlow::NextInsn))
    }
}

// ============================================================================
// SmirLifter Implementation
// ============================================================================

impl SmirLifter for RiscVLifter {
    fn source_arch(&self) -> SourceArch {
        if self.xlen == 64 {
            SourceArch::RiscV64
        } else {
            SourceArch::RiscV32
        }
    }

    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.is_empty() {
            return Err(LiftError::Incomplete {
                addr,
                have: 0,
                need: 2,
            });
        }

        // Determine instruction length from low bits
        let len = if bytes[0] & 0x03 != 0x03 {
            // Compressed instruction (16-bit)
            if !self.extensions.c {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: "compressed instruction (C extension disabled)".to_string(),
                });
            }
            2
        } else if bytes[0] & 0x1F == 0x1F {
            // 48-bit or longer (future extension)
            return Err(LiftError::Unsupported {
                addr,
                mnemonic: "extended instruction (>32 bits)".to_string(),
            });
        } else {
            // Standard 32-bit
            4
        };

        if bytes.len() < len {
            return Err(LiftError::Incomplete {
                addr,
                have: bytes.len(),
                need: len,
            });
        }

        let (ops, control_flow) = if len == 2 {
            let insn = u16::from_le_bytes([bytes[0], bytes[1]]);
            self.lift_insn16(insn, addr, ctx)?
        } else {
            let insn = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            self.lift_insn32(insn, addr, ctx)?
        };

        // Collect branch targets for block discovery
        let branch_targets = match &control_flow {
            ControlFlow::DirectBranch(target) => vec![*target],
            ControlFlow::CondBranchReg {
                taken, not_taken, ..
            } => vec![*taken, *not_taken],
            _ => vec![],
        };

        Ok(LiftResult {
            ops,
            bytes_consumed: len,
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
        use crate::smir::ir::{SmirBlock, Terminator, TrapKind};

        let block_id = ctx.get_or_create_block(addr);
        let mut all_ops = Vec::new();
        let mut current_addr = addr;

        loop {
            // Read enough bytes for a compressed or normal instruction
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
                        target,
                        fallthrough,
                        ..
                    } => {
                        let cond_vreg = ctx.alloc_vreg();
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
        use crate::smir::ir::{CallingConv, FunctionAttrs, SmirFunction};
        use std::collections::HashSet;

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
            // Estimate block end (varies due to compressed instructions)
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

        let calling_convention = if self.xlen == 64 {
            CallingConv::RiscVStd
        } else {
            CallingConv::RiscVStd
        };

        Ok(SmirFunction {
            id: func_id,
            entry: ctx.get_or_create_block(entry),
            blocks,
            locals: vec![],
            guest_range: (min_addr, max_addr),
            calling_convention,
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

    fn test_ctx() -> LiftContext {
        LiftContext::new(SourceArch::RiscV64)
    }

    #[test]
    fn test_riscv_lifter_addi() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // addi x1, x0, 42  (encoded as: 0x02a00093)
        let bytes = [0x93, 0x00, 0xa0, 0x02];
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 4);
        assert!(matches!(result.control_flow, ControlFlow::NextInsn));
        assert_eq!(result.ops.len(), 1);

        if let OpKind::Mov {
            src: SrcOperand::Imm(42),
            ..
        } = &result.ops[0].kind
        {
            // x0 + 42 optimizes to mov 42
        } else if let OpKind::Add {
            src2: SrcOperand::Imm(42),
            ..
        } = &result.ops[0].kind
        {
            // Or add x0, 42
        } else {
            panic!(
                "Expected ADDI to generate Mov or Add: {:?}",
                result.ops[0].kind
            );
        }
    }

    #[test]
    fn test_riscv_lifter_jal() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // jal x1, 0x100  (J-type, jump forward 256 bytes)
        // imm[20|10:1|11|19:12] = 0x100 = 0b0000_0001_0000_0000
        // Encoding: imm[20]=0, imm[10:1]=0x80, imm[11]=0, imm[19:12]=0
        let bytes = [0xef, 0x00, 0x00, 0x10]; // jal ra, 0x100
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 4);

        if let ControlFlow::DirectBranch(target) = result.control_flow {
            // Should jump to 0x1000 + offset
            assert!(target > 0x1000);
        } else {
            panic!("Expected DirectBranch");
        }
    }

    #[test]
    fn test_riscv_lifter_beq() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // beq x1, x2, 0x10  (B-type)
        let bytes = [0x63, 0x08, 0x20, 0x00]; // beq x1, x2, 16
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 4);

        if let ControlFlow::CondBranchReg {
            taken, not_taken, ..
        } = result.control_flow
        {
            assert_eq!(taken, 0x1010);
            assert_eq!(not_taken, 0x1004);
        } else {
            panic!("Expected CondBranchReg");
        }
    }

    #[test]
    fn test_riscv_lifter_load_store() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // ld x1, 8(x2)
        let bytes = [0x83, 0x30, 0x81, 0x00]; // ld x1, 8(x2)
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 4);
        assert!(matches!(result.control_flow, ControlFlow::NextInsn));
        assert_eq!(result.ops.len(), 1);

        if let OpKind::Load {
            width: MemWidth::B8,
            ..
        } = &result.ops[0].kind
        {
            // OK
        } else {
            panic!("Expected 64-bit Load");
        }
    }

    #[test]
    fn test_riscv_lifter_compressed_addi() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // c.addi x1, 5  (encoded as: 0x0515)
        let bytes = [0x85, 0x00]; // c.addi x1, 1
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 2);
        assert!(matches!(result.control_flow, ControlFlow::NextInsn));
    }

    #[test]
    fn test_riscv_lifter_compressed_j() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // c.j 0x10 (jump forward 16 bytes)
        let bytes = [0x21, 0xa0]; // c.j 8
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 2);

        if let ControlFlow::DirectBranch(_target) = result.control_flow {
            // OK - target varies by encoding
        } else {
            panic!("Expected DirectBranch");
        }
    }

    #[test]
    fn test_riscv_mul_div() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // mul x1, x2, x3  (M extension)
        let bytes = [0xb3, 0x80, 0x31, 0x02]; // mul x1, x3, x3
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 4);
        assert!(matches!(result.control_flow, ControlFlow::NextInsn));
    }

    #[test]
    fn test_riscv_atomic() {
        let mut lifter = RiscVLifter::rv64gc();
        let mut ctx = test_ctx();

        // amoadd.d x1, x2, (x3)
        let bytes = [0xaf, 0x30, 0x21, 0x00]; // amoadd.w x1, x2, (x2)
        let result = lifter.lift_insn(0x1000, &bytes, &mut ctx).unwrap();

        assert_eq!(result.bytes_consumed, 4);

        if let OpKind::AtomicRmw {
            op: AtomicOp::Add, ..
        } = &result.ops[0].kind
        {
            // OK
        } else {
            panic!("Expected AtomicRmw Add");
        }
    }

    #[test]
    fn test_lift_context_riscv() {
        let mut ctx = LiftContext::new(SourceArch::RiscV64);

        let v0 = ctx.alloc_vreg();
        let v1 = ctx.alloc_vreg();

        assert_ne!(v0, v1);
        assert!(v0.is_virtual());
    }
}
