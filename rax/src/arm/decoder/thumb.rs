//! Thumb (T16) and Thumb-2 (T32) instruction decoder.
//!
//! This module decodes Thumb instructions, which are either 16 or 32 bits wide.
//! 32-bit Thumb-2 instructions are encoded as two halfwords.

use super::{Condition, DecodeError, DecodedInsn, Mnemonic, ShiftType, operand::*};
use crate::arm::ExecutionState;

/// Thumb instruction decoder.
pub struct ThumbDecoder;

impl ThumbDecoder {
    /// Check if a 16-bit halfword indicates a 32-bit instruction.
    ///
    /// 32-bit Thumb instructions start with 0b11101, 0b11110, or 0b11111.
    pub fn is_32bit_instruction(hw1: u16) -> bool {
        let op = hw1 >> 11;
        op == 0b11101 || op == 0b11110 || op == 0b11111
    }

    /// Decode a 16-bit Thumb instruction.
    pub fn decode_16bit(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let op = raw >> 10;

        match op {
            // Shift (immediate), add, subtract, move, and compare
            0b000000..=0b001111 => Self::decode_shift_add_sub_mov_cmp(raw),
            // Data processing
            0b010000 => Self::decode_data_processing(raw),
            // Special data instructions and branch and exchange
            0b010001 => Self::decode_special_data_branch(raw),
            // Load from literal pool (PC-relative)
            0b01001_0 | 0b01001_1 => Self::decode_ldr_literal(raw),
            // Load/store single data item
            0b0101_00..=0b0101_11
            | 0b0110_00..=0b0110_11
            | 0b0111_00..=0b0111_11
            | 0b1000_00..=0b1000_11
            | 0b1001_00..=0b1001_11 => Self::decode_load_store(raw),
            // Generate PC-relative address (ADR)
            0b10100_0 | 0b10100_1 => Self::decode_adr(raw),
            // Generate SP-relative address (ADD SP)
            0b10101_0 | 0b10101_1 => Self::decode_add_sp(raw),
            // Miscellaneous 16-bit instructions
            0b1011_00..=0b1011_11 => Self::decode_miscellaneous(raw),
            // Store/load multiple
            0b11000_0 | 0b11000_1 => Self::decode_stm(raw),
            0b11001_0 | 0b11001_1 => Self::decode_ldm(raw),
            // Conditional branch and supervisor call
            0b1101_00..=0b1101_11 => Self::decode_cond_branch_svc(raw),
            // Unconditional branch
            0b11100_0 | 0b11100_1 => Self::decode_uncond_branch(raw),
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb,
                raw as u32,
                2,
            )),
        }
    }

    /// Decode a 32-bit Thumb-2 instruction.
    ///
    /// The raw value has hw1 in the high 16 bits and hw2 in the low 16 bits.
    pub fn decode_32bit(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;

        let op1 = (hw1 >> 11) & 0x3;
        let op2 = (hw1 >> 4) & 0x7F;
        let op = (hw2 >> 15) & 1;

        match op1 {
            0b01 => {
                // Load/store multiple, load/store dual, table branch
                if op2 & 0x64 == 0x00 {
                    Self::decode_32bit_load_store_multiple(raw)
                } else if op2 & 0x64 == 0x04 {
                    Self::decode_32bit_load_store_dual(raw)
                } else {
                    Self::decode_32bit_data_processing(raw)
                }
            }
            0b10 => {
                if op == 0 {
                    // bit25 (hw1 bit9) selects modified-immediate vs plain-binary.
                    if (hw1 >> 9) & 1 == 0 {
                        Self::decode_32bit_dp_modified_imm(raw)
                    } else {
                        Self::decode_32bit_dp_plain_imm(raw)
                    }
                } else {
                    // Branches and miscellaneous control
                    Self::decode_32bit_branch_misc(raw)
                }
            }
            0b11 => {
                if op2 & 0x70 == 0x20 {
                    // Data processing (register)
                    Self::decode_32bit_dp_register(raw)
                } else if op2 & 0x78 == 0x30 {
                    // Multiply, multiply accumulate, and absolute difference
                    Self::decode_32bit_multiply(raw)
                } else if op2 & 0x78 == 0x38 {
                    // Long multiply, long multiply accumulate, divide
                    Self::decode_32bit_long_multiply_divide(raw)
                } else if op2 & 0x67 == 0x01 {
                    // Load byte, memory hints
                    Self::decode_32bit_load_byte(raw)
                } else if op2 & 0x67 == 0x03 {
                    // Load halfword
                    Self::decode_32bit_load_halfword(raw)
                } else if op2 & 0x67 == 0x05 {
                    // Load word
                    Self::decode_32bit_load_word(raw)
                } else if op2 & 0x71 == 0x00 {
                    // Store single data item
                    Self::decode_32bit_store(raw)
                } else {
                    Ok(DecodedInsn::new(
                        Mnemonic::UNKNOWN,
                        ExecutionState::Thumb2,
                        raw,
                        4,
                    ))
                }
            }
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb2,
                raw,
                4,
            )),
        }
    }

    // =========================================================================
    // 16-bit Thumb Decoders
    // =========================================================================

    fn decode_shift_add_sub_mov_cmp(raw: u16) -> Result<DecodedInsn, DecodeError> {
        // Thumb 16-bit encoding for shift/add/sub/mov/cmp:
        // Bits [15:14] = 00
        // Bits [13:11] determine the major category
        let op_major = (raw >> 11) & 0x7;

        match op_major {
            // 000: LSL (immediate)
            0b000 => {
                let imm5 = ((raw >> 6) & 0x1F) as u8;
                let rm = ((raw >> 3) & 0x7) as u8;
                let rd = (raw & 0x7) as u8;

                // LSL with imm5=0 is MOV alias
                let mnemonic = if imm5 == 0 {
                    Mnemonic::MOVS
                } else {
                    Mnemonic::LSLS
                };
                let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rd)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
                if imm5 != 0 {
                    insn = insn.with_operand(Operand::Imm(Immediate::new(imm5 as i64)));
                }
                Ok(insn)
            }
            // 001: LSR (immediate)
            0b001 => {
                let imm5 = ((raw >> 6) & 0x1F) as u8;
                let rm = ((raw >> 3) & 0x7) as u8;
                let rd = (raw & 0x7) as u8;

                let shift_amt = if imm5 == 0 { 32 } else { imm5 as i64 };

                let mut insn =
                    DecodedInsn::new(Mnemonic::LSRS, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rd)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
                insn = insn.with_operand(Operand::Imm(Immediate::new(shift_amt)));
                Ok(insn)
            }
            // 010: ASR (immediate)
            0b010 => {
                let imm5 = ((raw >> 6) & 0x1F) as u8;
                let rm = ((raw >> 3) & 0x7) as u8;
                let rd = (raw & 0x7) as u8;

                let shift_amt = if imm5 == 0 { 32 } else { imm5 as i64 };

                let mut insn =
                    DecodedInsn::new(Mnemonic::ASRS, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rd)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
                insn = insn.with_operand(Operand::Imm(Immediate::new(shift_amt)));
                Ok(insn)
            }
            // 011: ADD/SUB (register or 3-bit immediate)
            0b011 => {
                let i = (raw >> 10) & 1; // 0 = register, 1 = immediate
                let op_bit = (raw >> 9) & 1; // 0 = ADD, 1 = SUB
                let rn = ((raw >> 3) & 0x7) as u8;
                let rd = (raw & 0x7) as u8;

                let mnemonic = if op_bit == 0 {
                    Mnemonic::ADDS
                } else {
                    Mnemonic::SUBS
                };

                let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rd)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rn)));

                if i == 0 {
                    // Register variant
                    let rm = ((raw >> 6) & 0x7) as u8;
                    insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
                } else {
                    // 3-bit immediate variant
                    let imm3 = ((raw >> 6) & 0x7) as i64;
                    insn = insn.with_operand(Operand::Imm(Immediate::new(imm3)));
                }
                Ok(insn)
            }
            // 100: MOV (immediate)
            0b100 => {
                let rd = ((raw >> 8) & 0x7) as u8;
                let imm8 = (raw & 0xFF) as i64;

                let mut insn =
                    DecodedInsn::new(Mnemonic::MOVS, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rd)));
                insn = insn.with_operand(Operand::Imm(Immediate::new(imm8)));
                Ok(insn)
            }
            // 101: CMP (immediate)
            0b101 => {
                let rn = ((raw >> 8) & 0x7) as u8;
                let imm8 = (raw & 0xFF) as i64;

                let mut insn =
                    DecodedInsn::new(Mnemonic::CMP, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rn)));
                insn = insn.with_operand(Operand::Imm(Immediate::new(imm8)));
                Ok(insn)
            }
            // 110: ADD (8-bit immediate)
            0b110 => {
                let rdn = ((raw >> 8) & 0x7) as u8;
                let imm8 = (raw & 0xFF) as i64;

                let mut insn =
                    DecodedInsn::new(Mnemonic::ADDS, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Imm(Immediate::new(imm8)));
                Ok(insn)
            }
            // 111: SUB (8-bit immediate)
            0b111 => {
                let rdn = ((raw >> 8) & 0x7) as u8;
                let imm8 = (raw & 0xFF) as i64;

                let mut insn =
                    DecodedInsn::new(Mnemonic::SUBS, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Imm(Immediate::new(imm8)));
                Ok(insn)
            }
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb,
                raw as u32,
                2,
            )),
        }
    }

    fn decode_data_processing(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let op = (raw >> 6) & 0xF;
        let rm = ((raw >> 3) & 0x7) as u8;
        let rdn = (raw & 0x7) as u8;

        let (mnemonic, uses_rd_as_rn) = match op {
            0b0000 => (Mnemonic::ANDS, true),
            0b0001 => (Mnemonic::EORS, true),
            0b0010 => (Mnemonic::LSLS, true),
            0b0011 => (Mnemonic::LSRS, true),
            0b0100 => (Mnemonic::ASRS, true),
            0b0101 => (Mnemonic::ADCS, true),
            0b0110 => (Mnemonic::SBCS, true),
            0b0111 => (Mnemonic::RORS, true),
            0b1000 => (Mnemonic::TST, false),  // Rd not written
            0b1001 => (Mnemonic::NEGS, false), // RSB Rd, Rm, #0
            0b1010 => (Mnemonic::CMP, false),
            0b1011 => (Mnemonic::CMN, false),
            0b1100 => (Mnemonic::ORRS, true),
            0b1101 => (Mnemonic::MULS, true),
            0b1110 => (Mnemonic::BICS, true),
            0b1111 => (Mnemonic::MVNS, false),
            _ => unreachable!(),
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2);
        insn.sets_flags = true;

        match op {
            // TST, CMP, CMN - no destination
            0b1000 | 0b1010 | 0b1011 => {
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
            }
            // NEG (RSB Rd, Rm, #0)
            0b1001 => {
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
            }
            // MVN
            0b1111 => {
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
            }
            // MUL
            0b1101 => {
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
            }
            // Other operations: Rd, Rd, Rm
            _ => {
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                if uses_rd_as_rn {
                    insn = insn.with_operand(Operand::Reg(Self::low_reg(rdn)));
                }
                insn = insn.with_operand(Operand::Reg(Self::low_reg(rm)));
            }
        }

        Ok(insn)
    }

    fn decode_special_data_branch(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let op = (raw >> 6) & 0xF;

        match op {
            // ADD (register) - high registers
            0b0000..=0b0011 => {
                let dn = (raw >> 7) & 1;
                let rm = ((raw >> 3) & 0xF) as u8;
                let rdn = (((dn << 3) as u8) | (raw & 0x7) as u8);

                Ok(
                    DecodedInsn::new(Mnemonic::ADD, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::any_reg(rdn)))
                        .with_operand(Operand::Reg(Self::any_reg(rdn)))
                        .with_operand(Operand::Reg(Self::any_reg(rm))),
                )
            }
            // CMP (register) - high registers
            0b0101 | 0b0110 | 0b0111 => {
                let n = (raw >> 7) & 1;
                let rm = ((raw >> 3) & 0xF) as u8;
                let rn = (((n << 3) as u8) | (raw & 0x7) as u8);

                let mut insn =
                    DecodedInsn::new(Mnemonic::CMP, ExecutionState::Thumb, raw as u32, 2);
                insn.sets_flags = true;
                insn = insn.with_operand(Operand::Reg(Self::any_reg(rn)));
                insn = insn.with_operand(Operand::Reg(Self::any_reg(rm)));
                Ok(insn)
            }
            // MOV (register) - high registers
            0b1000..=0b1011 => {
                let d = (raw >> 7) & 1;
                let rm = ((raw >> 3) & 0xF) as u8;
                let rd = (((d << 3) as u8) | (raw & 0x7) as u8);

                Ok(
                    DecodedInsn::new(Mnemonic::MOV, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::any_reg(rd)))
                        .with_operand(Operand::Reg(Self::any_reg(rm))),
                )
            }
            // BX
            0b1100 | 0b1101 => {
                let rm = ((raw >> 3) & 0xF) as u8;

                Ok(
                    DecodedInsn::new(Mnemonic::BX, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::any_reg(rm))),
                )
            }
            // BLX (register)
            0b1110 | 0b1111 => {
                let rm = ((raw >> 3) & 0xF) as u8;

                Ok(
                    DecodedInsn::new(Mnemonic::BLX, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::any_reg(rm))),
                )
            }
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Thumb,
                raw as u32,
                2,
            )),
        }
    }

    fn decode_ldr_literal(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let rt = ((raw >> 8) & 0x7) as u8;
        let imm8 = (raw & 0xFF) as i64;
        let offset = imm8 << 2;

        // LDR Rt, [PC, #offset]
        Ok(
            DecodedInsn::new(Mnemonic::LDR, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rt)))
                .with_operand(Operand::Label(offset)),
        )
    }

    fn decode_load_store(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let op_a = (raw >> 12) & 0xF;
        let op_b = (raw >> 9) & 0x7;

        match (op_a, op_b) {
            // STR (register)
            (0b0101, 0b000) => Self::decode_ls_reg(raw, Mnemonic::STR),
            // STRH (register)
            (0b0101, 0b001) => Self::decode_ls_reg(raw, Mnemonic::STRH),
            // STRB (register)
            (0b0101, 0b010) => Self::decode_ls_reg(raw, Mnemonic::STRB),
            // LDRSB (register)
            (0b0101, 0b011) => Self::decode_ls_reg(raw, Mnemonic::LDRSB),
            // LDR (register)
            (0b0101, 0b100) => Self::decode_ls_reg(raw, Mnemonic::LDR),
            // LDRH (register)
            (0b0101, 0b101) => Self::decode_ls_reg(raw, Mnemonic::LDRH),
            // LDRB (register)
            (0b0101, 0b110) => Self::decode_ls_reg(raw, Mnemonic::LDRB),
            // LDRSH (register)
            (0b0101, 0b111) => Self::decode_ls_reg(raw, Mnemonic::LDRSH),
            // STR (immediate, T1)
            (0b0110, _) if op_b & 0b100 == 0 => Self::decode_ls_imm_word(raw, false),
            // LDR (immediate, T1)
            (0b0110, _) if op_b & 0b100 != 0 => Self::decode_ls_imm_word(raw, true),
            // STRB (immediate)
            (0b0111, _) if op_b & 0b100 == 0 => Self::decode_ls_imm_byte(raw, false),
            // LDRB (immediate)
            (0b0111, _) if op_b & 0b100 != 0 => Self::decode_ls_imm_byte(raw, true),
            // STRH (immediate)
            (0b1000, _) if op_b & 0b100 == 0 => Self::decode_ls_imm_halfword(raw, false),
            // LDRH (immediate)
            (0b1000, _) if op_b & 0b100 != 0 => Self::decode_ls_imm_halfword(raw, true),
            // STR (immediate, T2) - SP relative
            (0b1001, _) if op_b & 0b100 == 0 => Self::decode_ls_sp_relative(raw, false),
            // LDR (immediate, T2) - SP relative
            (0b1001, _) if op_b & 0b100 != 0 => Self::decode_ls_sp_relative(raw, true),
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb,
                raw as u32,
                2,
            )),
        }
    }

    fn decode_ls_reg(raw: u16, mnemonic: Mnemonic) -> Result<DecodedInsn, DecodeError> {
        let rm = ((raw >> 6) & 0x7) as u8;
        let rn = ((raw >> 3) & 0x7) as u8;
        let rt = (raw & 0x7) as u8;

        let mem = MemOperand {
            base: Self::low_reg(rn),
            offset: MemOffset::Reg(Self::low_reg(rm)),
            mode: AddressingMode::Offset,
        };

        Ok(
            DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rt)))
                .with_operand(Operand::Mem(mem)),
        )
    }

    fn decode_ls_imm_word(raw: u16, is_load: bool) -> Result<DecodedInsn, DecodeError> {
        let imm5 = ((raw >> 6) & 0x1F) as i64;
        let rn = ((raw >> 3) & 0x7) as u8;
        let rt = (raw & 0x7) as u8;
        let offset = imm5 << 2;

        let mnemonic = if is_load {
            Mnemonic::LDR
        } else {
            Mnemonic::STR
        };

        Ok(
            DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rt)))
                .with_operand(Operand::Mem(MemOperand::imm_offset(
                    Self::low_reg(rn),
                    offset,
                ))),
        )
    }

    fn decode_ls_imm_byte(raw: u16, is_load: bool) -> Result<DecodedInsn, DecodeError> {
        let imm5 = ((raw >> 6) & 0x1F) as i64;
        let rn = ((raw >> 3) & 0x7) as u8;
        let rt = (raw & 0x7) as u8;

        let mnemonic = if is_load {
            Mnemonic::LDRB
        } else {
            Mnemonic::STRB
        };

        Ok(
            DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rt)))
                .with_operand(Operand::Mem(MemOperand::imm_offset(
                    Self::low_reg(rn),
                    imm5,
                ))),
        )
    }

    fn decode_ls_imm_halfword(raw: u16, is_load: bool) -> Result<DecodedInsn, DecodeError> {
        let imm5 = ((raw >> 6) & 0x1F) as i64;
        let rn = ((raw >> 3) & 0x7) as u8;
        let rt = (raw & 0x7) as u8;
        let offset = imm5 << 1;

        let mnemonic = if is_load {
            Mnemonic::LDRH
        } else {
            Mnemonic::STRH
        };

        Ok(
            DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rt)))
                .with_operand(Operand::Mem(MemOperand::imm_offset(
                    Self::low_reg(rn),
                    offset,
                ))),
        )
    }

    fn decode_ls_sp_relative(raw: u16, is_load: bool) -> Result<DecodedInsn, DecodeError> {
        let rt = ((raw >> 8) & 0x7) as u8;
        let imm8 = (raw & 0xFF) as i64;
        let offset = imm8 << 2;

        let mnemonic = if is_load {
            Mnemonic::LDR
        } else {
            Mnemonic::STR
        };

        // SP is r13
        Ok(
            DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rt)))
                .with_operand(Operand::Mem(MemOperand::imm_offset(
                    Register::sp32(),
                    offset,
                ))),
        )
    }

    fn decode_adr(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let rd = ((raw >> 8) & 0x7) as u8;
        let imm8 = (raw & 0xFF) as i64;
        let offset = imm8 << 2;

        // ADR Rd, label (actually ADD Rd, PC, #offset)
        Ok(
            DecodedInsn::new(Mnemonic::ADD, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rd)))
                .with_operand(Operand::Label(offset)),
        )
    }

    fn decode_add_sp(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let rd = ((raw >> 8) & 0x7) as u8;
        let imm8 = (raw & 0xFF) as i64;
        let offset = imm8 << 2;

        // ADD Rd, SP, #imm
        Ok(
            DecodedInsn::new(Mnemonic::ADD, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rd)))
                .with_operand(Operand::Reg(Register::sp32()))
                .with_operand(Operand::Imm(Immediate::new(offset))),
        )
    }

    fn decode_miscellaneous(raw: u16) -> Result<DecodedInsn, DecodeError> {
        // Miscellaneous 16-bit: bits [15:12] = 1011, bits [11:8] = opcode
        let op = (raw >> 8) & 0xF;

        match op {
            // 0000: ADD/SUB SP
            0b0000 => {
                let s = (raw >> 7) & 1;
                let imm7 = (raw & 0x7F) as i64;
                let offset = imm7 << 2;

                let mnemonic = if s == 0 { Mnemonic::ADD } else { Mnemonic::SUB };
                Ok(
                    DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Register::sp32()))
                        .with_operand(Operand::Reg(Register::sp32()))
                        .with_operand(Operand::Imm(Immediate::new(offset))),
                )
            }
            // 0001: CBZ (forward reference only)
            0b0001 | 0b0011 => {
                let i = (raw >> 9) & 1;
                let imm5 = (raw >> 3) & 0x1F;
                let rn = (raw & 0x7) as u8;
                let imm = ((i << 6) | (imm5 << 1)) as i64;

                let mnemonic = if op == 0b0001 {
                    Mnemonic::CBZ
                } else {
                    Mnemonic::CBZ
                };
                Ok(
                    DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::low_reg(rn)))
                        .with_operand(Operand::Imm(Immediate::new(imm))),
                )
            }
            // 0010: SXTH, SXTB, UXTH, UXTB
            0b0010 => {
                let rm = ((raw >> 3) & 0x7) as u8;
                let rd = (raw & 0x7) as u8;
                let op2 = (raw >> 6) & 0x3;

                let mnemonic = match op2 {
                    0b00 => Mnemonic::SXTH,
                    0b01 => Mnemonic::SXTB,
                    0b10 => Mnemonic::UXTH,
                    0b11 => Mnemonic::UXTB,
                    _ => unreachable!(),
                };

                Ok(
                    DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::low_reg(rd)))
                        .with_operand(Operand::Reg(Self::low_reg(rm))),
                )
            }
            // 0100, 0101: PUSH
            0b0100 | 0b0101 => {
                let m = (raw >> 8) & 1;
                let reg_list = ((raw & 0xFF) | ((m as u16) << 14)) as u16;

                Ok(
                    DecodedInsn::new(Mnemonic::PUSH, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::RegList(RegisterList::from_mask(reg_list))),
                )
            }
            // 1001: CBNZ
            0b1001 | 0b1011 => {
                let i = (raw >> 9) & 1;
                let imm5 = (raw >> 3) & 0x1F;
                let rn = (raw & 0x7) as u8;
                let imm = ((i << 6) | (imm5 << 1)) as i64;

                Ok(
                    DecodedInsn::new(Mnemonic::CBNZ, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::low_reg(rn)))
                        .with_operand(Operand::Imm(Immediate::new(imm))),
                )
            }
            // 1010: REV, REV16, REVSH
            0b1010 => {
                let rm = ((raw >> 3) & 0x7) as u8;
                let rd = (raw & 0x7) as u8;
                let op2 = (raw >> 6) & 0x3;

                let mnemonic = match op2 {
                    0b00 => Mnemonic::REV,
                    0b01 => Mnemonic::REV16,
                    0b11 => Mnemonic::REVSH,
                    _ => Mnemonic::UNDEFINED,
                };

                Ok(
                    DecodedInsn::new(mnemonic, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Reg(Self::low_reg(rd)))
                        .with_operand(Operand::Reg(Self::low_reg(rm))),
                )
            }
            // 1100, 1101: POP
            0b1100 | 0b1101 => {
                let p = (raw >> 8) & 1;
                let reg_list = ((raw & 0xFF) | ((p as u16) << 15)) as u16;

                Ok(
                    DecodedInsn::new(Mnemonic::POP, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::RegList(RegisterList::from_mask(reg_list))),
                )
            }
            // 1110: BKPT
            0b1110 => {
                let imm8 = (raw & 0xFF) as i64;

                Ok(
                    DecodedInsn::new(Mnemonic::BKPT, ExecutionState::Thumb, raw as u32, 2)
                        .with_operand(Operand::Imm(Immediate::new(imm8))),
                )
            }
            // 1111: If-Then/Hints
            0b1111 => {
                let op_a = (raw >> 4) & 0xF;
                let op_b = raw & 0xF;

                if op_a != 0 {
                    // IT instruction
                    return Ok(DecodedInsn::new(
                        Mnemonic::IT,
                        ExecutionState::Thumb,
                        raw as u32,
                        2,
                    ));
                }

                let mnemonic = match op_b {
                    0b0000 => Mnemonic::NOP,
                    0b0001 => Mnemonic::YIELD,
                    0b0010 => Mnemonic::WFE,
                    0b0011 => Mnemonic::WFI,
                    0b0100 => Mnemonic::SEV,
                    _ => Mnemonic::HINT,
                };

                Ok(DecodedInsn::new(
                    mnemonic,
                    ExecutionState::Thumb,
                    raw as u32,
                    2,
                ))
            }
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb,
                raw as u32,
                2,
            )),
        }
    }

    fn decode_stm(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let rn = ((raw >> 8) & 0x7) as u8;
        let reg_list = (raw & 0xFF) as u16;

        Ok(
            DecodedInsn::new(Mnemonic::STMIA, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rn)))
                .with_operand(Operand::RegList(RegisterList::from_mask(reg_list))),
        )
    }

    fn decode_ldm(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let rn = ((raw >> 8) & 0x7) as u8;
        let reg_list = (raw & 0xFF) as u16;

        Ok(
            DecodedInsn::new(Mnemonic::LDMIA, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Reg(Self::low_reg(rn)))
                .with_operand(Operand::RegList(RegisterList::from_mask(reg_list))),
        )
    }

    fn decode_cond_branch_svc(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let op = (raw >> 8) & 0xF;

        if op == 0b1110 {
            // UDF
            let imm8 = (raw & 0xFF) as i64;
            return Ok(
                DecodedInsn::new(Mnemonic::UDF, ExecutionState::Thumb, raw as u32, 2)
                    .with_operand(Operand::Imm(Immediate::new(imm8))),
            );
        }

        if op == 0b1111 {
            // SVC
            let imm8 = (raw & 0xFF) as i64;
            return Ok(
                DecodedInsn::new(Mnemonic::SVC, ExecutionState::Thumb, raw as u32, 2)
                    .with_operand(Operand::Imm(Immediate::new(imm8))),
            );
        }

        // Conditional branch
        let cond = Condition::from_bits(op as u8);
        let imm8 = (raw & 0xFF) as i64;

        // Sign extend and shift
        let offset = if imm8 & (1 << 7) != 0 {
            (imm8 | !0xFF) << 1
        } else {
            imm8 << 1
        };

        Ok(
            DecodedInsn::new(Mnemonic::BCC, ExecutionState::Thumb, raw as u32, 2)
                .with_cond(cond)
                .with_operand(Operand::Label(offset)),
        )
    }

    fn decode_uncond_branch(raw: u16) -> Result<DecodedInsn, DecodeError> {
        let imm11 = (raw & 0x7FF) as i64;

        // Sign extend and shift
        let offset = if imm11 & (1 << 10) != 0 {
            (imm11 | !0x7FF) << 1
        } else {
            imm11 << 1
        };

        Ok(
            DecodedInsn::new(Mnemonic::B, ExecutionState::Thumb, raw as u32, 2)
                .with_operand(Operand::Label(offset)),
        )
    }

    // =========================================================================
    // 32-bit Thumb-2 Decoders
    // =========================================================================

    fn decode_32bit_load_store_multiple(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let l = (raw >> 20) & 1;
        let w = (raw >> 21) & 1;
        let rn = ((raw >> 16) & 0xF) as u8;
        let reg_list = (raw & 0xFFFF) as u16;

        // op = bits[24:23]: 01 = increment-after, 10 = decrement-before.
        let mnemonic = match ((raw >> 23) & 3, l) {
            (0b01, 1) => Mnemonic::LDMIA,
            (0b01, 0) => Mnemonic::STMIA,
            (0b10, 1) => Mnemonic::LDMDB,
            (0b10, 0) => Mnemonic::STMDB,
            _ => Mnemonic::UNKNOWN,
        };

        // Check for PUSH/POP aliases
        let mnemonic = if rn == 13 && w == 1 {
            if l == 1 {
                Mnemonic::POP
            } else {
                Mnemonic::PUSH
            }
        } else {
            mnemonic
        };

        let is_push_pop = matches!(mnemonic, Mnemonic::PUSH | Mnemonic::POP);

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4);

        if !is_push_pop {
            insn = insn.with_operand(Operand::Reg(Self::any_reg(rn)));
        }

        insn = insn.with_operand(Operand::RegList(RegisterList::from_mask(reg_list)));

        Ok(insn)
    }

    fn decode_32bit_load_store_dual(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // LDRD/STRD (immediate): op1 = P:U:1:W (bit24..21), L = bit20. The
        // exclusive/table-branch members of this group (op1[1]=0 => P=0,W=0) are
        // not handled here.
        let p = (raw >> 24) & 1;
        let u = (raw >> 23) & 1;
        let w = (raw >> 21) & 1;
        let l = (raw >> 20) & 1;
        if p == 0 && w == 0 {
            // LDREX/STREX/TBB/TBH — not yet implemented.
            return Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb2,
                raw,
                4,
            ));
        }
        let rn = ((raw >> 16) & 0xF) as u8; // hw1[3:0]
        let rt = ((raw >> 12) & 0xF) as u8; // hw2[15:12]
        let rt2 = ((raw >> 8) & 0xF) as u8; // hw2[11:8]
        let imm8 = (raw & 0xFF) as i64; // hw2[7:0]
        let off = if u == 1 { imm8 << 2 } else { -(imm8 << 2) };
        let mode = if p == 0 {
            AddressingMode::PostIndex
        } else if w == 1 {
            AddressingMode::PreIndex
        } else {
            AddressingMode::Offset
        };
        let mnemonic = if l == 1 { Mnemonic::LDP } else { Mnemonic::STP };
        return Ok(DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4)
            .with_operand(Operand::Reg(Self::any_reg(rt)))
            .with_operand(Operand::Reg(Self::any_reg(rt2)))
            .with_operand(Operand::Mem(MemOperand {
                base: Self::any_reg(rn),
                offset: MemOffset::Imm(off),
                mode,
            })));

        #[allow(unreachable_code)]
        Ok(DecodedInsn::new(
            Mnemonic::UNKNOWN,
            ExecutionState::Thumb2,
            raw,
            4,
        ))
    }

    /// T32 data-processing (shifted register): AND/BIC/ORR/ORN/EOR/PKH/ADD/ADC/
    /// SBC/SUB/RSB with a shifted Rm, plus the MOV/MVN-with-shift and the
    /// TST/TEQ/CMP/CMN comparison forms.
    fn decode_32bit_data_processing(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;

        let op = (hw1 >> 5) & 0xF;
        let s = (hw1 >> 4) & 1;
        let rn = (hw1 & 0xF) as u8;
        let rd = ((hw2 >> 8) & 0xF) as u8;
        let rm = (hw2 & 0xF) as u8;
        let imm3 = (hw2 >> 12) & 0x7;
        let imm2 = (hw2 >> 6) & 0x3;
        let type_bits = (hw2 >> 4) & 0x3;
        let shift_imm = ((imm3 << 2) | imm2) as u8;

        // Decode the shift type/amount (RRX when type==11 && shift==0).
        let (shift_type, amount) = match type_bits {
            0b00 => (ShiftType::LSL, shift_imm),
            0b01 => (ShiftType::LSR, if shift_imm == 0 { 32 } else { shift_imm }),
            0b10 => (ShiftType::ASR, if shift_imm == 0 { 32 } else { shift_imm }),
            _ => {
                if shift_imm == 0 {
                    (ShiftType::RRX, 1)
                } else {
                    (ShiftType::ROR, shift_imm)
                }
            }
        };

        // PKHBT / PKHTB (op==0110): route to the A32 umbrella (T32-aware exec).
        if op == 0b0110 {
            return Ok(
                DecodedInsn::new(Mnemonic::A32_PKH, ExecutionState::Thumb2, raw, 4)
                    .with_operand(Operand::Reg(Self::any_reg(rd))),
            );
        }

        let (mnemonic, uses_rn, writes_rd) = match op {
            0b0000 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::TST, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ANDS
                        } else {
                            Mnemonic::AND
                        },
                        true,
                        true,
                    )
                }
            }
            0b0001 => (
                if s == 1 {
                    Mnemonic::BICS
                } else {
                    Mnemonic::BIC
                },
                true,
                true,
            ),
            0b0010 => {
                if rn == 15 {
                    (
                        if s == 1 {
                            Mnemonic::MOVS
                        } else {
                            Mnemonic::MOV
                        },
                        false,
                        true,
                    )
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ORRS
                        } else {
                            Mnemonic::ORR
                        },
                        true,
                        true,
                    )
                }
            }
            0b0011 => {
                if rn == 15 {
                    (
                        if s == 1 {
                            Mnemonic::MVNS
                        } else {
                            Mnemonic::MVN
                        },
                        false,
                        true,
                    )
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ORNS
                        } else {
                            Mnemonic::ORN
                        },
                        true,
                        true,
                    )
                }
            }
            0b0100 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::TEQ, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::EORS
                        } else {
                            Mnemonic::EOR
                        },
                        true,
                        true,
                    )
                }
            }
            0b1000 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::CMN, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ADDS
                        } else {
                            Mnemonic::ADD
                        },
                        true,
                        true,
                    )
                }
            }
            0b1010 => (
                if s == 1 {
                    Mnemonic::ADCS
                } else {
                    Mnemonic::ADC
                },
                true,
                true,
            ),
            0b1011 => (
                if s == 1 {
                    Mnemonic::SBCS
                } else {
                    Mnemonic::SBC
                },
                true,
                true,
            ),
            0b1101 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::CMP, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::SUBS
                        } else {
                            Mnemonic::SUB
                        },
                        true,
                        true,
                    )
                }
            }
            0b1110 => (
                if s == 1 {
                    Mnemonic::RSBS
                } else {
                    Mnemonic::RSB
                },
                true,
                true,
            ),
            _ => (Mnemonic::UNKNOWN, false, false),
        };

        if mnemonic == Mnemonic::UNKNOWN {
            return Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb2,
                raw,
                4,
            ));
        }

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4);
        if s == 1 && writes_rd {
            insn.sets_flags = true;
        }
        if writes_rd {
            insn = insn.with_operand(Operand::Reg(Self::any_reg(rd)));
        }
        if uses_rn {
            insn = insn.with_operand(Operand::Reg(Self::any_reg(rn)));
        }
        insn = insn.with_operand(Operand::ShiftedReg(ShiftedRegister::new(
            Self::any_reg(rm),
            shift_type,
            amount,
        )));
        Ok(insn)
    }

    fn decode_32bit_dp_modified_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;

        let op = (hw1 >> 5) & 0xF;
        let rn = (hw1 & 0xF) as u8;
        let s = (hw1 >> 4) & 1;
        let rd = ((hw2 >> 8) & 0xF) as u8;

        // Decode modified immediate
        let i = (hw1 >> 10) & 1;
        let imm3 = (hw2 >> 12) & 0x7;
        let imm8 = (hw2 & 0xFF) as u32;
        let imm12 = ((i as u32) << 11) | ((imm3 as u32) << 8) | imm8;
        let imm = Self::decode_thumb_modified_imm(imm12);

        let (mnemonic, uses_rn, writes_rd) = match op {
            0b0000 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::TST, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ANDS
                        } else {
                            Mnemonic::AND
                        },
                        true,
                        true,
                    )
                }
            }
            0b0001 => (
                if s == 1 {
                    Mnemonic::BICS
                } else {
                    Mnemonic::BIC
                },
                true,
                true,
            ),
            0b0010 => {
                if rn == 15 {
                    (
                        if s == 1 {
                            Mnemonic::MOVS
                        } else {
                            Mnemonic::MOV
                        },
                        false,
                        true,
                    )
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ORRS
                        } else {
                            Mnemonic::ORR
                        },
                        true,
                        true,
                    )
                }
            }
            0b0011 => {
                if rn == 15 {
                    (
                        if s == 1 {
                            Mnemonic::MVNS
                        } else {
                            Mnemonic::MVN
                        },
                        false,
                        true,
                    )
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ORNS
                        } else {
                            Mnemonic::ORN
                        },
                        true,
                        true,
                    )
                }
            }
            0b0100 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::TEQ, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::EORS
                        } else {
                            Mnemonic::EOR
                        },
                        true,
                        true,
                    )
                }
            }
            0b1000 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::CMN, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::ADDS
                        } else {
                            Mnemonic::ADD
                        },
                        true,
                        true,
                    )
                }
            }
            0b1010 => (
                if s == 1 {
                    Mnemonic::ADCS
                } else {
                    Mnemonic::ADC
                },
                true,
                true,
            ),
            0b1011 => (
                if s == 1 {
                    Mnemonic::SBCS
                } else {
                    Mnemonic::SBC
                },
                true,
                true,
            ),
            0b1101 => {
                if rd == 15 && s == 1 {
                    (Mnemonic::CMP, true, false)
                } else {
                    (
                        if s == 1 {
                            Mnemonic::SUBS
                        } else {
                            Mnemonic::SUB
                        },
                        true,
                        true,
                    )
                }
            }
            0b1110 => (
                if s == 1 {
                    Mnemonic::RSBS
                } else {
                    Mnemonic::RSB
                },
                true,
                true,
            ),
            _ => (Mnemonic::UNKNOWN, false, false),
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4);

        if s == 1 && writes_rd {
            insn.sets_flags = true;
        }

        if writes_rd {
            insn = insn.with_operand(Operand::Reg(Self::any_reg(rd)));
        }

        if uses_rn {
            insn = insn.with_operand(Operand::Reg(Self::any_reg(rn)));
        }

        insn = insn.with_operand(Operand::Imm(Immediate::new(imm as i64)));

        Ok(insn)
    }

    /// T32 data-processing (plain binary immediate): ADDW/SUBW/MOVW/MOVT and the
    /// bitfield/saturate group. Exec for the bitfield/sat ops reads the T32 raw
    /// layout (state == Thumb2); ADDW/SUBW/MOVW/MOVT are lowered to ADD/SUB/MOV/
    /// MOVK with an immediate operand.
    fn decode_32bit_dp_plain_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;
        let op = (hw1 >> 4) & 0x1F;
        let rn = (hw1 & 0xF) as u8;
        let rd = ((hw2 >> 8) & 0xF) as u8;
        let i = ((hw1 >> 10) & 1) as u32;
        let imm3 = ((hw2 >> 12) & 0x7) as u32;
        let imm8 = (hw2 & 0xFF) as u32;
        let shift_imm = (imm3 << 2) | (((hw2 >> 6) & 0x3) as u32);
        let imm12 = (i << 11) | (imm3 << 8) | imm8;
        let imm16 = ((rn as u32) << 12) | imm12;

        let reg = |n: u8| Operand::Reg(Self::any_reg(n));
        let mk = |m: Mnemonic, ops: Vec<Operand>| {
            let mut insn = DecodedInsn::new(m, ExecutionState::Thumb2, raw, 4);
            for o in ops {
                insn = insn.with_operand(o);
            }
            Ok(insn)
        };

        match op {
            0b00000 => mk(
                Mnemonic::ADD,
                vec![reg(rd), reg(rn), Operand::Imm(Immediate::new(imm12 as i64))],
            ),
            0b01010 => mk(
                Mnemonic::SUB,
                vec![reg(rd), reg(rn), Operand::Imm(Immediate::new(imm12 as i64))],
            ),
            0b00100 => mk(
                Mnemonic::MOV,
                vec![reg(rd), Operand::Imm(Immediate::new(imm16 as i64))],
            ),
            0b01100 => mk(
                Mnemonic::MOVK,
                vec![reg(rd), Operand::Imm(Immediate::new(imm16 as i64))],
            ),
            0b10000 => mk(Mnemonic::SSAT, vec![reg(rd)]),
            0b10010 => {
                if shift_imm == 0 {
                    mk(Mnemonic::A32_SAT16, vec![reg(rd)])
                } else {
                    mk(Mnemonic::SSAT, vec![reg(rd)])
                }
            }
            0b10100 => mk(Mnemonic::SBFX, vec![reg(rd)]),
            0b10110 => {
                if rn == 15 {
                    mk(Mnemonic::BFC, vec![reg(rd)])
                } else {
                    mk(Mnemonic::BFI, vec![reg(rd)])
                }
            }
            0b11000 => mk(Mnemonic::USAT, vec![reg(rd)]),
            0b11010 => {
                if shift_imm == 0 {
                    mk(Mnemonic::A32_SAT16, vec![reg(rd)])
                } else {
                    mk(Mnemonic::USAT, vec![reg(rd)])
                }
            }
            0b11100 => mk(Mnemonic::UBFX, vec![reg(rd)]),
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Thumb2,
                raw,
                4,
            )),
        }
    }

    fn decode_thumb_modified_imm(imm12: u32) -> u32 {
        let imm8 = imm12 & 0xFF;
        if (imm12 >> 10) & 0x3 == 0 {
            // Replicated patterns, selected by imm12[9:8].
            match (imm12 >> 8) & 0x3 {
                0b00 => imm8,
                0b01 => (imm8 << 16) | imm8,
                0b10 => (imm8 << 24) | (imm8 << 8),
                _ => (imm8 << 24) | (imm8 << 16) | (imm8 << 8) | imm8,
            }
        } else {
            // Rotated: value = 0x80:imm12[6:0], rotated right by imm12[11:7].
            let val = 0x80 | (imm12 & 0x7F);
            val.rotate_right((imm12 >> 7) & 0x1F)
        }
    }

    fn decode_32bit_branch_misc(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;

        let op1 = (hw1 >> 4) & 0x7F;
        let op2 = (hw2 >> 12) & 0x7;

        // Conditional branch
        if op2 & 0x5 == 0 && op1 & 0x38 != 0x38 {
            let s = (hw1 >> 10) & 1;
            let cond = ((hw1 >> 6) & 0xF) as u8;
            let imm6 = hw1 & 0x3F;
            let j1 = (hw2 >> 13) & 1;
            let j2 = (hw2 >> 11) & 1;
            let imm11 = hw2 & 0x7FF;

            let imm = ((s as u32) << 20)
                | ((j2 as u32) << 19)
                | ((j1 as u32) << 18)
                | ((imm6 as u32) << 12)
                | ((imm11 as u32) << 1);

            // Sign extend
            let offset = if s == 1 {
                (imm | 0xFFE0_0000) as i32
            } else {
                imm as i32
            } as i64;

            return Ok(
                DecodedInsn::new(Mnemonic::BCC, ExecutionState::Thumb2, raw, 4)
                    .with_cond(Condition::from_bits(cond))
                    .with_operand(Operand::Label(offset)),
            );
        }

        // Unconditional branch (B.W, BL, BLX)
        // BL:  hw2[15:14] = 11, hw2[12] = 1 → op2 = x1x where bit 0 = 1
        // BLX: hw2[15:14] = 11, hw2[12] = 0 → op2 = x0x where bit 0 = 0
        // B.W: hw2[15:14] = 10, hw2[12] = x → op2 bit 2 = 0
        if op2 & 0x4 == 0x4 || (op2 & 0x5 == 0x1) {
            let s = (hw1 >> 10) & 1;
            let imm10 = hw1 & 0x3FF;
            let j1 = (hw2 >> 13) & 1;
            let j2 = (hw2 >> 11) & 1;
            let imm11 = hw2 & 0x7FF;
            let link_bit = (hw2 >> 14) & 1; // L bit
            let exchange_bit = (hw2 >> 12) & 1; // For BLX, this is 0

            let i1 = !((j1 ^ s) & 1) & 1;
            let i2 = !((j2 ^ s) & 1) & 1;

            let imm = ((s as u32) << 24)
                | ((i1 as u32) << 23)
                | ((i2 as u32) << 22)
                | ((imm10 as u32) << 12)
                | ((imm11 as u32) << 1);

            // Sign extend
            let offset = if s == 1 {
                (imm | 0xFE00_0000) as i32
            } else {
                imm as i32
            } as i64;

            let mnemonic = if link_bit == 1 && exchange_bit == 0 {
                Mnemonic::BLX
            } else if link_bit == 1 {
                Mnemonic::BL
            } else {
                Mnemonic::B
            };

            return Ok(DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4)
                .with_operand(Operand::Label(offset)));
        }

        // MSR, MRS, hints, misc control
        if op1 == 0x38 && op2 == 0 {
            // Hints
            let op1 = (hw2 >> 8) & 0xFF;
            let op2 = hw2 & 0xFF;

            let mnemonic = match (op1, op2) {
                (0, 0) => Mnemonic::NOP,
                (0, 1) => Mnemonic::YIELD,
                (0, 2) => Mnemonic::WFE,
                (0, 3) => Mnemonic::WFI,
                (0, 4) => Mnemonic::SEV,
                _ => Mnemonic::HINT,
            };

            return Ok(DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4));
        }

        Ok(DecodedInsn::new(
            Mnemonic::UNKNOWN,
            ExecutionState::Thumb2,
            raw,
            4,
        ))
    }

    /// T32 data-processing (register): register-controlled shifts (LSL/LSR/ASR/
    /// ROR), register extends, parallel add/sub, and the miscellaneous ops
    /// (QADD/QSUB/QDADD/QDSUB, REV/REV16/RBIT/REVSH, SEL, CLZ).
    fn decode_32bit_dp_register(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;
        let op1 = (hw1 >> 4) & 0xF; // bits[23:20]
        let op2 = (hw2 >> 4) & 0xF; // bits[7:4]
        let rn = (hw1 & 0xF) as u8;
        let rd = ((hw2 >> 8) & 0xF) as u8;
        let rm = (hw2 & 0xF) as u8;
        let any = Self::any_reg;

        let mk = |m: Mnemonic, ops: &[u8], flags: bool| {
            let mut insn = DecodedInsn::new(m, ExecutionState::Thumb2, raw, 4);
            insn.sets_flags = flags;
            for &o in ops {
                insn = insn.with_operand(Operand::Reg(any(o)));
            }
            insn
        };

        if op2 & 0b1000 == 0 {
            if op1 & 0b1000 == 0 {
                // Register-controlled shift: type = op1[2:1], S = op1[0].
                let s = op1 & 1 == 1;
                let (base, sbase) = match (op1 >> 1) & 0x3 {
                    0 => (Mnemonic::LSL, Mnemonic::LSLS),
                    1 => (Mnemonic::LSR, Mnemonic::LSRS),
                    2 => (Mnemonic::ASR, Mnemonic::ASRS),
                    _ => (Mnemonic::ROR, Mnemonic::RORS),
                };
                let m = if s { sbase } else { base };
                // operands: [Rd, Rn(value), Rm(shift amount)]
                return Ok(mk(m, &[rd, rn, rm], s));
            } else {
                // Parallel add/sub.
                return Ok(mk(Mnemonic::A32_PARALLEL, &[rd, rn, rm], false));
            }
        } else if op1 & 0b1000 == 0 {
            // Register extends (op1 in 0..5).
            return Ok(mk(Mnemonic::A32_EXTEND, &[rd, rn, rm], false));
        } else {
            // Miscellaneous operations, keyed by op1[2:0] and op2[1:0].
            match op1 & 0x7 {
                0b000 => return Ok(mk(Mnemonic::A32_SAT_ADDSUB, &[rd, rn, rm], false)),
                0b001 => {
                    let m = match op2 & 0x3 {
                        0b00 => Mnemonic::REV,
                        0b01 => Mnemonic::REV16,
                        0b10 => Mnemonic::RBIT,
                        _ => Mnemonic::REVSH,
                    };
                    return Ok(mk(m, &[rd, rm], false));
                }
                0b010 => return Ok(mk(Mnemonic::A32_SEL, &[rd, rn, rm], false)),
                0b011 => return Ok(mk(Mnemonic::CLZ, &[rd, rm], false)),
                _ => {}
            }
        }

        Ok(DecodedInsn::new(
            Mnemonic::UNKNOWN,
            ExecutionState::Thumb2,
            raw,
            4,
        ))
    }

    fn decode_32bit_multiply(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;

        let op1 = (hw1 >> 4) & 0x7;
        let op2 = (hw2 >> 4) & 0x3;
        let rn = (hw1 & 0xF) as u8;
        let ra = ((hw2 >> 12) & 0xF) as u8;
        let rd = ((hw2 >> 8) & 0xF) as u8;
        let rm = (hw2 & 0xF) as u8;

        // DSP signed multiplies use the A32 umbrella mnemonics; their exec reads
        // the T32 raw layout (state == Thumb2).
        let dsp = match op1 {
            0b001 | 0b011 => Some(Mnemonic::A32_HMUL), // SMUL/SMLA + SMULW/SMLAW
            0b010 | 0b100 => Some(Mnemonic::A32_DUAL), // SMUAD/SMLAD + SMUSD/SMLSD
            0b101 | 0b110 => Some(Mnemonic::A32_SMMUL), // SMMUL/SMMLA + SMMLS
            0b111 => Some(Mnemonic::A32_USAD),         // USAD8/USADA8
            _ => None,
        };
        if let Some(m) = dsp {
            return Ok(DecodedInsn::new(m, ExecutionState::Thumb2, raw, 4)
                .with_operand(Operand::Reg(Self::any_reg(rd))));
        }

        let mnemonic = match (op1, op2) {
            (0b000, 0b00) if ra != 15 => Mnemonic::MLA,
            (0b000, 0b00) if ra == 15 => Mnemonic::MUL,
            (0b000, 0b01) => Mnemonic::MLS,
            _ => Mnemonic::UNKNOWN,
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4)
            .with_operand(Operand::Reg(Self::any_reg(rd)))
            .with_operand(Operand::Reg(Self::any_reg(rn)))
            .with_operand(Operand::Reg(Self::any_reg(rm)));

        if mnemonic == Mnemonic::MLA || mnemonic == Mnemonic::MLS {
            insn = insn.with_operand(Operand::Reg(Self::any_reg(ra)));
        }

        Ok(insn)
    }

    fn decode_32bit_long_multiply_divide(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;

        let op1 = (hw1 >> 4) & 0x7;
        let op2 = (hw2 >> 4) & 0xF;
        let rn = (hw1 & 0xF) as u8;
        let rd_lo = ((hw2 >> 12) & 0xF) as u8;
        let rd_hi = ((hw2 >> 8) & 0xF) as u8;
        let rm = (hw2 & 0xF) as u8;

        // UMAAL / SMLALD / SMLSLD use the umbrella mnemonics (exec reads T32 raw).
        // SMLALD/SMLSLD: op1=100/101 with op2=110x. UMAAL: op1=110, op2=0110.
        if (op1 == 0b100 || op1 == 0b101) && (op2 & 0xE) == 0xC {
            return Ok(
                DecodedInsn::new(Mnemonic::A32_SMLALD, ExecutionState::Thumb2, raw, 4)
                    .with_operand(Operand::Reg(Self::any_reg(rd_lo))),
            );
        }
        if op1 == 0b110 && op2 == 0b0110 {
            return Ok(
                DecodedInsn::new(Mnemonic::UMAAL, ExecutionState::Thumb2, raw, 4)
                    .with_operand(Operand::Reg(Self::any_reg(rd_lo)))
                    .with_operand(Operand::Reg(Self::any_reg(rd_hi)))
                    .with_operand(Operand::Reg(Self::any_reg(rn)))
                    .with_operand(Operand::Reg(Self::any_reg(rm))),
            );
        }

        let mnemonic = match (op1, op2) {
            (0b000, 0b0000) => Mnemonic::SMULL,
            (0b001, 0b1111) => Mnemonic::SDIV,
            (0b010, 0b0000) => Mnemonic::UMULL,
            (0b011, 0b1111) => Mnemonic::UDIV,
            (0b100, 0b0000) => Mnemonic::SMLAL,
            (0b110, 0b0000) => Mnemonic::UMLAL,
            _ => Mnemonic::UNKNOWN,
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4);

        match mnemonic {
            Mnemonic::SDIV | Mnemonic::UDIV => {
                insn = insn
                    .with_operand(Operand::Reg(Self::any_reg(rd_hi))) // Actually Rd
                    .with_operand(Operand::Reg(Self::any_reg(rn)))
                    .with_operand(Operand::Reg(Self::any_reg(rm)));
            }
            Mnemonic::SMULL | Mnemonic::UMULL | Mnemonic::SMLAL | Mnemonic::UMLAL => {
                insn = insn
                    .with_operand(Operand::Reg(Self::any_reg(rd_lo)))
                    .with_operand(Operand::Reg(Self::any_reg(rd_hi)))
                    .with_operand(Operand::Reg(Self::any_reg(rn)))
                    .with_operand(Operand::Reg(Self::any_reg(rm)));
            }
            _ => {}
        }

        Ok(insn)
    }

    /// Build the T32 single load/store memory operand, covering T3 (positive
    /// imm12), T4 (±imm8 with offset/pre/post-index + writeback), and the
    /// register-offset (LSL imm2) form.
    fn t32_mem_operand(raw: u32) -> MemOperand {
        let hw1 = (raw >> 16) as u16;
        let hw2 = raw as u16;
        let base = Self::any_reg((hw1 & 0xF) as u8);
        if (hw1 >> 7) & 1 == 1 {
            // T3: positive 12-bit immediate offset.
            MemOperand::imm_offset(base, (hw2 & 0xFFF) as i64)
        } else if (hw2 >> 11) & 1 == 1 {
            // T4: ±imm8 with P/U/W.
            let p = (hw2 >> 10) & 1;
            let u = (hw2 >> 9) & 1;
            let w = (hw2 >> 8) & 1;
            let imm8 = (hw2 & 0xFF) as i64;
            let off = if u == 1 { imm8 } else { -imm8 };
            let mode = if p == 0 {
                AddressingMode::PostIndex
            } else if w == 1 {
                AddressingMode::PreIndex
            } else {
                AddressingMode::Offset
            };
            MemOperand {
                base,
                offset: MemOffset::Imm(off),
                mode,
            }
        } else {
            // Register offset, optional LSL by imm2.
            let imm2 = ((hw2 >> 4) & 0x3) as u8;
            let rm = Self::any_reg((hw2 & 0xF) as u8);
            let offset = if imm2 == 0 {
                MemOffset::Reg(rm)
            } else {
                MemOffset::ShiftedReg(ShiftedRegister::new(rm, ShiftType::LSL, imm2))
            };
            MemOperand {
                base,
                offset,
                mode: AddressingMode::Offset,
            }
        }
    }

    fn decode_32bit_load_byte(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let rt = ((raw >> 12) & 0xF) as u8;
        let mnemonic = if (hw1 >> 8) & 1 == 0 {
            Mnemonic::LDRB
        } else {
            Mnemonic::LDRSB
        };
        Ok(DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4)
            .with_operand(Operand::Reg(Self::any_reg(rt)))
            .with_operand(Operand::Mem(Self::t32_mem_operand(raw))))
    }

    fn decode_32bit_load_halfword(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let rt = ((raw >> 12) & 0xF) as u8;
        let mnemonic = if (hw1 >> 8) & 1 == 1 {
            Mnemonic::LDRSH
        } else {
            Mnemonic::LDRH
        };
        Ok(DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4)
            .with_operand(Operand::Reg(Self::any_reg(rt)))
            .with_operand(Operand::Mem(Self::t32_mem_operand(raw))))
    }

    fn decode_32bit_load_word(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let rt = ((raw >> 12) & 0xF) as u8;
        Ok(
            DecodedInsn::new(Mnemonic::LDR, ExecutionState::Thumb2, raw, 4)
                .with_operand(Operand::Reg(Self::any_reg(rt)))
                .with_operand(Operand::Mem(Self::t32_mem_operand(raw))),
        )
    }

    fn decode_32bit_store(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let hw1 = (raw >> 16) as u16;
        let rt = ((raw >> 12) & 0xF) as u8;
        let mnemonic = match (hw1 >> 5) & 0x3 {
            0b00 => Mnemonic::STRB,
            0b01 => Mnemonic::STRH,
            0b10 => Mnemonic::STR,
            _ => Mnemonic::UNKNOWN,
        };
        Ok(DecodedInsn::new(mnemonic, ExecutionState::Thumb2, raw, 4)
            .with_operand(Operand::Reg(Self::any_reg(rt)))
            .with_operand(Operand::Mem(Self::t32_mem_operand(raw))))
    }

    // =========================================================================
    // Helper functions
    // =========================================================================

    /// Create a low register (r0-r7).
    fn low_reg(num: u8) -> Register {
        Register::raw(num & 0x7, false, false)
    }

    /// Create any register (r0-r15).
    fn any_reg(num: u8) -> Register {
        if num == 13 {
            Register::sp32()
        } else {
            Register::raw(num & 0xF, false, false)
        }
    }
}

// Placeholder for extension sign-extend mnemonics
#[allow(non_camel_case_types)]
impl Mnemonic {
    // Extension aliases not in the main enum
}

// Add these aliases to Mnemonic
const _SXTH: Mnemonic = Mnemonic::SBFM;
const _SXTB: Mnemonic = Mnemonic::SBFM;
const _UXTH: Mnemonic = Mnemonic::UBFM;
const _UXTB: Mnemonic = Mnemonic::UBFM;
const _REVSH: Mnemonic = Mnemonic::REV16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_32bit() {
        // 16-bit instructions
        assert!(!ThumbDecoder::is_32bit_instruction(0x4600)); // MOV r0, r0
        assert!(!ThumbDecoder::is_32bit_instruction(0xB500)); // PUSH {LR}

        // 32-bit instructions
        assert!(ThumbDecoder::is_32bit_instruction(0xF000)); // 0b11110...
        assert!(ThumbDecoder::is_32bit_instruction(0xE800)); // 0b11101...
        assert!(ThumbDecoder::is_32bit_instruction(0xF800)); // 0b11111...
    }

    #[test]
    fn test_16bit_nop() {
        // NOP: bf00
        let insn = ThumbDecoder::decode_16bit(0xbf00).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::NOP);
        assert_eq!(insn.size, 2);
    }

    #[test]
    fn test_16bit_mov_imm() {
        // MOVS R0, #0x42: 2042
        let insn = ThumbDecoder::decode_16bit(0x2042).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MOVS);
        assert!(insn.sets_flags);
    }

    #[test]
    fn test_16bit_add_reg() {
        // ADDS R0, R1, R2: 1888
        let insn = ThumbDecoder::decode_16bit(0x1888).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADDS);
    }

    #[test]
    fn test_16bit_ldr_imm() {
        // LDR R0, [R1, #0]: 6808
        let insn = ThumbDecoder::decode_16bit(0x6808).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LDR);
    }

    #[test]
    fn test_16bit_push() {
        // PUSH {R4, LR}: b510
        let insn = ThumbDecoder::decode_16bit(0xb510).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::PUSH);
    }

    #[test]
    fn test_16bit_pop() {
        // POP {R4, PC}: bd10
        let insn = ThumbDecoder::decode_16bit(0xbd10).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::POP);
    }

    #[test]
    fn test_16bit_bx_lr() {
        // BX LR: 4770
        let insn = ThumbDecoder::decode_16bit(0x4770).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BX);
    }

    #[test]
    fn test_16bit_b() {
        // B #0x10: e004
        let insn = ThumbDecoder::decode_16bit(0xe004).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::B);
    }

    #[test]
    fn test_16bit_beq() {
        // BEQ #0x10: d004
        let insn = ThumbDecoder::decode_16bit(0xd004).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BCC);
        assert_eq!(insn.cond, Some(Condition::EQ));
    }

    #[test]
    fn test_16bit_cmp_imm() {
        // CMP R0, #0: 2800
        let insn = ThumbDecoder::decode_16bit(0x2800).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CMP);
    }

    #[test]
    fn test_16bit_lsl() {
        // LSLS R0, R1, #4: 0108
        let insn = ThumbDecoder::decode_16bit(0x0108).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LSLS);
    }

    #[test]
    fn test_16bit_svc() {
        // SVC #0: df00
        let insn = ThumbDecoder::decode_16bit(0xdf00).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SVC);
    }

    #[test]
    fn test_32bit_bl() {
        // BL #0x100: f000 f880
        let raw = 0xf000_f880u32;
        let insn = ThumbDecoder::decode_32bit(raw).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BL);
        assert_eq!(insn.size, 4);
    }

    #[test]
    fn test_32bit_mov_imm() {
        // MOV.W R0, #1: f04f 0001
        let raw = 0xf04f_0001u32;
        let insn = ThumbDecoder::decode_32bit(raw).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MOV);
    }

    #[test]
    fn test_32bit_add_imm() {
        // ADD.W R0, R1, #1: f101 0001
        let raw = 0xf101_0001u32;
        let insn = ThumbDecoder::decode_32bit(raw).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADD);
    }

    #[test]
    fn test_32bit_ldr_imm() {
        // LDR.W R0, [R1]: f8d1 0000
        let raw = 0xf8d1_0000u32;
        let insn = ThumbDecoder::decode_32bit(raw).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LDR);
    }

    #[test]
    fn test_32bit_push_multiple() {
        // PUSH.W {R4-R11, LR}: e92d 4ff0
        let raw = 0xe92d_4ff0u32;
        let insn = ThumbDecoder::decode_32bit(raw).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::PUSH);
    }
}
