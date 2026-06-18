//! AArch64 (A64) instruction decoder.
//!
//! This module decodes 64-bit ARM instructions (AArch64/A64).
//! All A64 instructions are 32 bits wide.

use super::{Condition, DecodeError, DecodedInsn, ExtendType, Mnemonic, ShiftType, operand::*};
use crate::arm::ExecutionState;

/// AArch64 instruction decoder.
pub struct Aarch64Decoder;

impl Aarch64Decoder {
    /// Decode a 32-bit AArch64 instruction.
    pub fn decode(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // Extract top-level opcode bits for classification
        // A64 encoding uses bits [28:25] as the main encoding group
        let op0 = (raw >> 25) & 0xF;

        match op0 {
            // 0b0000: Reserved / unallocated
            0b0000 => Self::decode_reserved(raw),

            // 0b0001: Unallocated
            0b0001 => Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            )),

            // 0b0010: SVE encodings
            0b0010 => Self::decode_sve(raw),

            // 0b0011: Unallocated
            0b0011 => Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            )),

            // 0b100x: Data processing - immediate
            0b1000 | 0b1001 => Self::decode_dp_imm(raw),

            // 0b101x: Branch, exception generation, and system
            0b1010 | 0b1011 => Self::decode_branch_sys(raw),

            // 0b1100, 0b1110: Loads and stores
            0b0100 | 0b0110 | 0b1100 | 0b1110 => Self::decode_load_store(raw),

            // 0b0101, 0b1101: Data processing - register
            0b0101 | 0b1101 => Self::decode_dp_reg(raw),

            // 0b0111, 0b1111: Data processing - SIMD and FP
            0b0111 | 0b1111 => Self::decode_simd_fp(raw),

            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Aarch64,
                raw,
                4,
            )),
        }
    }

    // =========================================================================
    // Reserved / Unallocated
    // =========================================================================

    fn decode_reserved(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // UDF is encoded with all zeros in specific fields
        if raw == 0 {
            return Ok(
                DecodedInsn::new(Mnemonic::UDF, ExecutionState::Aarch64, raw, 4)
                    .with_operand(Operand::Imm(Immediate::new(0))),
            );
        }
        Ok(DecodedInsn::new(
            Mnemonic::UNDEFINED,
            ExecutionState::Aarch64,
            raw,
            4,
        ))
    }

    // =========================================================================
    // Data Processing - Immediate
    // =========================================================================

    fn decode_dp_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op0 = (raw >> 23) & 0x7;

        match op0 {
            // PC-rel addressing
            0b000 | 0b001 => Self::decode_pc_rel(raw),
            // Add/sub immediate
            0b010 => Self::decode_add_sub_imm(raw),
            // Add/sub immediate with tags
            0b011 => Self::decode_add_sub_imm_tags(raw),
            // Logical immediate
            0b100 => Self::decode_logical_imm(raw),
            // Move wide immediate
            0b101 => Self::decode_move_wide_imm(raw),
            // Bitfield
            0b110 => Self::decode_bitfield(raw),
            // Extract
            0b111 => Self::decode_extract(raw),
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Aarch64,
                raw,
                4,
            )),
        }
    }

    fn decode_pc_rel(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op = (raw >> 31) & 1;
        let immlo = (raw >> 29) & 0x3;
        let immhi = (raw >> 5) & 0x7FFFF;
        let rd = (raw & 0x1F) as u8;

        let imm = ((immhi << 2) | immlo) as i64;
        // Sign extend 21-bit value
        let imm = if imm & (1 << 20) != 0 {
            imm | !0x1FFFFF
        } else {
            imm
        };

        // ADR: op=0, forms PC + imm
        // ADRP: op=1, forms (PC & ~0xFFF) + (imm << 12)
        let (mnemonic, label) = if op == 0 {
            (Mnemonic::ADR, imm)
        } else {
            (Mnemonic::ADRP, imm << 12)
        };

        let insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::x(rd)))
            .with_operand(Operand::Label(label));

        Ok(insn)
    }

    fn decode_add_sub_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 30) & 1;
        let s = (raw >> 29) & 1;
        let sh = (raw >> 22) & 1;
        let imm12 = ((raw >> 10) & 0xFFF) as i64;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;
        let shift = if sh == 1 { 12 } else { 0 };

        // Determine mnemonic
        let mnemonic = match (op, s) {
            (0, 0) => Mnemonic::ADD,
            (0, 1) => Mnemonic::ADDS,
            (1, 0) => Mnemonic::SUB,
            (1, 1) => Mnemonic::SUBS,
            _ => unreachable!(),
        };

        // Check for aliases
        // CMN is ADDS with Rd = XZR/WZR
        // CMP is SUBS with Rd = XZR/WZR
        // MOV (to/from SP) is ADD with imm=0 and one operand is SP
        let (mnemonic, skip_rd) = if s == 1 && rd == 31 {
            if op == 0 {
                (Mnemonic::CMN, true)
            } else {
                (Mnemonic::CMP, true)
            }
        } else {
            (mnemonic, false)
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        if s == 1 {
            insn.sets_flags = true;
        }

        if !skip_rd {
            // For ADD/SUB, Rd can be SP
            insn = insn.with_operand(Operand::Reg(Register::with_sp(rd, is_64bit)));
        }

        // Rn can be SP for ADD/SUB immediate
        insn = insn.with_operand(Operand::Reg(Register::with_sp(rn, is_64bit)));
        insn = insn.with_operand(Operand::Imm(Immediate::shifted(imm12, shift)));

        Ok(insn)
    }

    fn decode_add_sub_imm_tags(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // MTE instructions: ADDG, SUBG
        let op = (raw >> 30) & 1;
        let s = (raw >> 29) & 1;

        if s == 1 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = if op == 0 {
            Mnemonic::ADDG
        } else {
            Mnemonic::SUBG
        };

        let uimm6 = ((raw >> 16) & 0x3F) as i64;
        let uimm4 = ((raw >> 10) & 0xF) as i64;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_sp(rd, true)))
            .with_operand(Operand::Reg(Register::with_sp(rn, true)))
            .with_operand(Operand::Imm(Immediate::new(uimm6 << 4)))
            .with_operand(Operand::Imm(Immediate::new(uimm4))))
    }

    fn decode_logical_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let opc = (raw >> 29) & 0x3;
        let n = (raw >> 22) & 1;
        let immr = ((raw >> 16) & 0x3F) as u8;
        let imms = ((raw >> 10) & 0x3F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // For 32-bit, N must be 0
        if !is_64bit && n == 1 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match opc {
            0b00 => Mnemonic::AND,
            0b01 => Mnemonic::ORR,
            0b10 => Mnemonic::EOR,
            0b11 => Mnemonic::ANDS,
            _ => unreachable!(),
        };

        // Decode bitmask immediate
        let imm = Self::decode_bitmask_imm(n as u8, imms, immr, is_64bit);

        // Check for aliases
        // TST is ANDS with Rd = XZR/WZR
        // MOV (bitmask immediate) is ORR with Rn = XZR/WZR
        let (mnemonic, skip_rd) = if opc == 0b11 && rd == 31 {
            (Mnemonic::TST, true)
        } else if opc == 0b01 && rn == 31 {
            (Mnemonic::MOV, false)
        } else {
            (mnemonic, false)
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        if opc == 0b11 {
            insn.sets_flags = true;
        }

        if !skip_rd {
            // For logical immediate, Rd can be SP only for AND/ORR/EOR (not ANDS)
            if opc != 0b11 {
                insn = insn.with_operand(Operand::Reg(Register::with_sp(rd, is_64bit)));
            } else {
                insn = insn.with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)));
            }
        }

        if mnemonic != Mnemonic::MOV {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));
        }

        insn = insn.with_operand(Operand::Imm(Immediate::new(imm as i64)));

        Ok(insn)
    }

    fn decode_bitmask_imm(n: u8, imms: u8, immr: u8, is_64bit: bool) -> u64 {
        // Decode the bitmask immediate encoding
        let len = if n == 1 {
            6
        } else {
            // Find highest bit set in ~imms
            let not_imms = !imms & 0x3F;
            (0..6)
                .rev()
                .find(|&i| (not_imms >> i) & 1 == 1)
                .unwrap_or(0)
        };

        let levels = (1u64 << len) - 1;
        let s = (imms as u64) & levels;
        let r = (immr as u64) & levels;
        let esize = 1u64 << len;

        // Create the basic pattern
        let welem = (1u64 << (s + 1)) - 1;

        // Rotate right
        let pattern = if r == 0 {
            welem
        } else {
            let mask = (1u64 << esize) - 1;
            ((welem >> r) | (welem << (esize - r))) & mask
        };

        // Replicate the pattern
        let mut result = pattern;
        let mut pos = esize;
        while pos < 64 {
            result |= pattern << pos;
            pos += esize;
        }

        if !is_64bit {
            result &= 0xFFFFFFFF;
        }

        result
    }

    fn decode_move_wide_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let opc = (raw >> 29) & 0x3;
        let hw = ((raw >> 21) & 0x3) as u8;
        let imm16 = ((raw >> 5) & 0xFFFF) as i64;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // For 32-bit, hw must be 0 or 1
        if !is_64bit && hw >= 2 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match opc {
            0b00 => Mnemonic::MOVN,
            0b10 => Mnemonic::MOVZ,
            0b11 => Mnemonic::MOVK,
            _ => {
                return Ok(DecodedInsn::new(
                    Mnemonic::UNDEFINED,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ));
            }
        };

        let shift = hw * 16;

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)))
            .with_operand(Operand::Imm(Immediate::shifted(imm16, shift))))
    }

    fn decode_bitfield(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let opc = (raw >> 29) & 0x3;
        let n = (raw >> 22) & 1;
        let immr = ((raw >> 16) & 0x3F) as u8;
        let imms = ((raw >> 10) & 0x3F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // sf and N must match
        if sf != n {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let base_mnemonic = match opc {
            0b00 => Mnemonic::SBFM,
            0b01 => Mnemonic::BFI, // Actually BFM, but we'll determine alias
            0b10 => Mnemonic::UBFM,
            _ => {
                return Ok(DecodedInsn::new(
                    Mnemonic::UNDEFINED,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ));
            }
        };

        // Determine aliases based on immr/imms values
        let reg_size = if is_64bit { 64 } else { 32 };

        let mnemonic = match opc {
            0b00 => {
                // SBFM aliases
                if imms == (reg_size - 1) as u8 {
                    Mnemonic::ASR // ASR when imms == regsize-1
                } else if imms < immr {
                    Mnemonic::SBFIZ
                } else if immr == 0 {
                    match imms {
                        7 => Mnemonic::SXTB,
                        15 => Mnemonic::SXTH,
                        31 => Mnemonic::SXTW,
                        _ => Mnemonic::SBFX,
                    }
                } else {
                    Mnemonic::SBFX
                }
            }
            0b01 => {
                // BFM aliases
                if rn == 31 {
                    Mnemonic::BFC // BFC when Rn == WZR/XZR
                } else if imms < immr {
                    Mnemonic::BFI
                } else {
                    Mnemonic::BFXIL
                }
            }
            0b10 => {
                // UBFM aliases
                if imms == (reg_size - 1) as u8 {
                    Mnemonic::LSR // LSR when imms == regsize-1
                } else if imms + 1 == immr {
                    Mnemonic::LSL // LSL when imms+1 == immr
                } else if imms < immr {
                    Mnemonic::UBFIZ
                } else if immr == 0 {
                    match imms {
                        7 => Mnemonic::UBFX,  // Could be UXTB
                        15 => Mnemonic::UBFX, // Could be UXTH
                        _ => Mnemonic::UBFX,
                    }
                } else {
                    Mnemonic::UBFX
                }
            }
            _ => base_mnemonic,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)))
            .with_operand(Operand::Imm(Immediate::new(immr as i64)))
            .with_operand(Operand::Imm(Immediate::new(imms as i64))))
    }

    fn decode_extract(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let n = (raw >> 22) & 1;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let imms = ((raw >> 10) & 0x3F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // sf and N must match
        if sf != n {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        // For 32-bit, imms must be < 32
        if !is_64bit && imms >= 32 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        // ROR is EXTR when Rn == Rm
        let mnemonic = if rn == rm {
            Mnemonic::ROR
        } else {
            Mnemonic::EXTR
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));

        if mnemonic == Mnemonic::EXTR {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rm, is_64bit)));
        }

        insn = insn.with_operand(Operand::Imm(Immediate::new(imms as i64)));

        Ok(insn)
    }

    // =========================================================================
    // Branch, Exception, and System
    // =========================================================================

    fn decode_branch_sys(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op0 = (raw >> 29) & 0x7;
        let op1 = (raw >> 22) & 0x7F;
        let op_bit25 = (raw >> 25) & 0x1; // Bit 25 distinguishes CBZ/CBNZ from TBZ/TBNZ

        match op0 {
            // Unconditional branch (immediate)
            0b000 | 0b100 => Self::decode_uncond_branch_imm(raw),
            // Compare and branch (bit 25 = 0) or Test and branch (bit 25 = 1)
            0b001 | 0b011 | 0b101 | 0b111 => {
                if op_bit25 == 0 {
                    Self::decode_compare_branch(raw)
                } else {
                    Self::decode_test_branch(raw)
                }
            }
            // Conditional branch
            0b010 => Self::decode_cond_branch(raw),
            // System, exception, unconditional branch (register)
            0b110 => {
                // Check bits [31:24] for instruction type
                let top8 = (raw >> 24) & 0xFF;
                match top8 {
                    // Exception generating: 1101 0100 xxx = 0xD4
                    0xD4 => Self::decode_exception(raw),
                    // System: 1101 0101 0xx = 0xD5 with bit 22 = 0
                    0xD5 if (raw >> 22) & 1 == 0 => Self::decode_system(raw),
                    // Unconditional branch (register): 1101 011x = 0xD6/0xD7
                    0xD6 | 0xD7 => Self::decode_uncond_branch_reg(raw),
                    _ => Ok(DecodedInsn::new(
                        Mnemonic::UNKNOWN,
                        ExecutionState::Aarch64,
                        raw,
                        4,
                    )),
                }
            }
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Aarch64,
                raw,
                4,
            )),
        }
    }

    fn decode_uncond_branch_imm(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op = (raw >> 31) & 1;
        let imm26 = (raw & 0x3FFFFFF) as i64;

        // Sign extend 26-bit offset
        let offset = if imm26 & (1 << 25) != 0 {
            (imm26 | !0x3FFFFFF) << 2
        } else {
            imm26 << 2
        };

        let mnemonic = if op == 0 { Mnemonic::B } else { Mnemonic::BL };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Label(offset)))
    }

    fn decode_compare_branch(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 24) & 1;
        let imm19 = ((raw >> 5) & 0x7FFFF) as i64;
        let rt = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // Sign extend 19-bit offset
        let offset = if imm19 & (1 << 18) != 0 {
            (imm19 | !0x7FFFF) << 2
        } else {
            imm19 << 2
        };

        let mnemonic = if op == 0 {
            Mnemonic::CBZ
        } else {
            Mnemonic::CBNZ
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
            .with_operand(Operand::Label(offset)))
    }

    fn decode_test_branch(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let b5 = (raw >> 31) & 1;
        let op = (raw >> 24) & 1;
        let b40 = ((raw >> 19) & 0x1F) as u8;
        let imm14 = ((raw >> 5) & 0x3FFF) as i64;
        let rt = (raw & 0x1F) as u8;

        let bit_pos = (b5 << 5) as u8 | b40;
        let is_64bit = b5 == 1;

        // Sign extend 14-bit offset
        let offset = if imm14 & (1 << 13) != 0 {
            (imm14 | !0x3FFF) << 2
        } else {
            imm14 << 2
        };

        let mnemonic = if op == 0 {
            Mnemonic::TBZ
        } else {
            Mnemonic::TBNZ
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
            .with_operand(Operand::Imm(Immediate::new(bit_pos as i64)))
            .with_operand(Operand::Label(offset)))
    }

    fn decode_cond_branch(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let o1 = (raw >> 24) & 1;
        let imm19 = ((raw >> 5) & 0x7FFFF) as i64;
        let o0 = (raw >> 4) & 1;
        let cond = (raw & 0xF) as u8;

        if o1 != 0 || o0 != 0 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        // Sign extend 19-bit offset
        let offset = if imm19 & (1 << 18) != 0 {
            (imm19 | !0x7FFFF) << 2
        } else {
            imm19 << 2
        };

        Ok(
            DecodedInsn::new(Mnemonic::BCC, ExecutionState::Aarch64, raw, 4)
                .with_cond(Condition::from_bits(cond))
                .with_operand(Operand::Label(offset)),
        )
    }

    fn decode_exception(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let opc = (raw >> 21) & 0x7;
        let imm16 = ((raw >> 5) & 0xFFFF) as i64;
        let ll = raw & 0x3;

        let mnemonic = match (opc, ll) {
            (0b000, 0b01) => Mnemonic::SVC,
            (0b000, 0b10) => Mnemonic::HVC,
            (0b000, 0b11) => Mnemonic::SMC,
            (0b001, 0b00) => Mnemonic::BRK,
            (0b010, 0b00) => Mnemonic::HLT,
            (0b101, 0b01) => Mnemonic::UDF, // DCPS1
            (0b101, 0b10) => Mnemonic::UDF, // DCPS2
            (0b101, 0b11) => Mnemonic::UDF, // DCPS3
            _ => Mnemonic::UNDEFINED,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Imm(Immediate::new(imm16))))
    }

    fn decode_system(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let l = (raw >> 21) & 1;
        let op0 = (raw >> 19) & 0x3;
        let op1 = ((raw >> 16) & 0x7) as u8;
        let crn = ((raw >> 12) & 0xF) as u8;
        let crm = ((raw >> 8) & 0xF) as u8;
        let op2 = ((raw >> 5) & 0x7) as u8;
        let rt = (raw & 0x1F) as u8;

        // Flag manipulation system instructions.
        if op0 == 0 && op1 == 0 && crn == 4 && crm == 0 && rt == 31 {
            let mnemonic = match op2 {
                0b000 => Mnemonic::CFINV,
                0b001 => Mnemonic::XAFLAG,
                0b010 => Mnemonic::AXFLAG,
                _ => Mnemonic::UNDEFINED,
            };
            return Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4));
        }

        // Hints: op0 = 0, op1 = 3, CRn = 2, Rt = 31
        if op0 == 0 && op1 == 3 && crn == 2 && rt == 31 {
            return Self::decode_hint_barrier(raw, op1, crm, op2, rt);
        }

        // Barriers: op0 = 0, op1 = 3, CRn = 3, Rt = 31
        if op0 == 0 && op1 == 3 && crn == 3 && rt == 31 {
            return Self::decode_barrier(raw, crm, op2);
        }

        // MSR/MRS (op0 >= 2)
        if op0 >= 2 {
            let sysreg = ((op0 as u16) << 14)
                | ((op1 as u16) << 11)
                | ((crn as u16) << 7)
                | ((crm as u16) << 3)
                | (op2 as u16);

            if l == 0 {
                // MSR
                Ok(
                    DecodedInsn::new(Mnemonic::MSR, ExecutionState::Aarch64, raw, 4)
                        .with_operand(Operand::SysReg(sysreg))
                        .with_operand(Operand::Reg(Register::with_zr(rt, true))),
                )
            } else {
                // MRS
                Ok(
                    DecodedInsn::new(Mnemonic::MRS, ExecutionState::Aarch64, raw, 4)
                        .with_operand(Operand::Reg(Register::with_zr(rt, true)))
                        .with_operand(Operand::SysReg(sysreg)),
                )
            }
        } else {
            Ok(DecodedInsn::new(
                Mnemonic::SYS,
                ExecutionState::Aarch64,
                raw,
                4,
            ))
        }
    }

    fn decode_ldr_literal(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // LDR (literal) - Load register from PC-relative address
        let opc = (raw >> 30) & 0x3;
        let v = (raw >> 26) & 1;
        let imm19 = ((raw >> 5) & 0x7FFFF) as i64;
        let rt = (raw & 0x1F) as u8;

        // Sign extend 19-bit offset
        let offset = if imm19 & (1 << 18) != 0 {
            (imm19 | !0x7FFFF) << 2
        } else {
            imm19 << 2
        };

        if opc == 0b11 && v == 0 {
            return Ok(
                DecodedInsn::new(Mnemonic::PRFM, ExecutionState::Aarch64, raw, 4)
                    .with_operand(Operand::Prfop(PrefetchOp::from_bits(rt)))
                    .with_operand(Operand::Label(offset)),
            );
        }

        let (mnemonic, is_64bit) = match (opc, v) {
            (0b00, 0) => (Mnemonic::LDR, false),  // LDR Wt
            (0b01, 0) => (Mnemonic::LDR, true),   // LDR Xt
            (0b10, 0) => (Mnemonic::LDRSW, true), // LDRSW
            (0b00, 1) => (Mnemonic::LDR, false),  // LDR St (SIMD)
            (0b01, 1) => (Mnemonic::LDR, true),   // LDR Dt (SIMD)
            (0b10, 1) => (Mnemonic::LDR, true),   // LDR Qt (SIMD)
            _ => (Mnemonic::UNKNOWN, true),
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
            .with_operand(Operand::Label(offset)))
    }

    fn decode_barrier(raw: u32, crm: u8, op2: u8) -> Result<DecodedInsn, DecodeError> {
        let mnemonic = match op2 {
            0b010 => Mnemonic::CLREX,
            0b100 => Mnemonic::DSB,
            0b101 => Mnemonic::DMB,
            0b110 => Mnemonic::ISB,
            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Barrier(BarrierOption::from_bits(crm))))
    }

    fn decode_hint_barrier(
        raw: u32,
        _op1: u8,
        crm: u8,
        op2: u8,
        rt: u8,
    ) -> Result<DecodedInsn, DecodeError> {
        // NOP, YIELD, WFE, WFI, SEV, SEVL, etc.
        if rt != 31 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match (crm, op2) {
            (0b0000, 0b000) => Mnemonic::NOP,
            (0b0000, 0b001) => Mnemonic::YIELD,
            (0b0000, 0b010) => Mnemonic::WFE,
            (0b0000, 0b011) => Mnemonic::WFI,
            (0b0000, 0b100) => Mnemonic::SEV,
            (0b0000, 0b101) => Mnemonic::SEVL,
            (0b0010, 0b000) => Mnemonic::NOP, // ESB
            (0b0010, 0b001) => Mnemonic::NOP, // PSB CSYNC
            (0b0010, 0b010) => Mnemonic::NOP, // TSB CSYNC
            (0b0010, 0b100) => Mnemonic::NOP, // CSDB
            (0b0100, 0b000 | 0b010 | 0b100 | 0b110) => Mnemonic::BTI,
            _ => Mnemonic::HINT,
        };

        if mnemonic == Mnemonic::HINT {
            let imm = (crm << 3) | op2;
            Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                .with_operand(Operand::Imm(Immediate::new(imm as i64))))
        } else {
            Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4))
        }
    }

    fn decode_uncond_branch_reg(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let opc = (raw >> 21) & 0xF;
        let op2 = (raw >> 16) & 0x1F;
        let op3 = (raw >> 10) & 0x3F;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let op4 = raw & 0x1F;

        if op2 != 0x1F {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match (opc, op3, op4) {
            (0b0000, 0b000000, 0b00000) => Mnemonic::BR,
            (0b0001, 0b000000, 0b00000) => Mnemonic::BLR,
            (0b0010, 0b000000, 0b00000) => Mnemonic::RET,
            (0b0100, 0b000000, 0b00000) => Mnemonic::ERET,
            (0b0101, 0b000000, 0b00000) => Mnemonic::DRPS,
            // PAC variants
            (0b1000, 0b000010, _) => Mnemonic::BRAA,
            (0b1000, 0b000011, _) => Mnemonic::BRAB,
            (0b1001, 0b000010, _) => Mnemonic::BLRAA,
            (0b1001, 0b000011, _) => Mnemonic::BLRAB,
            (0b0010, 0b000010, 0b11111) => Mnemonic::RETAA,
            (0b0010, 0b000011, 0b11111) => Mnemonic::RETAB,
            _ => Mnemonic::UNDEFINED,
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        // Add register operand for most variants
        match mnemonic {
            Mnemonic::BR
            | Mnemonic::BLR
            | Mnemonic::RET
            | Mnemonic::BRAA
            | Mnemonic::BRAB
            | Mnemonic::BLRAA
            | Mnemonic::BLRAB => {
                insn = insn.with_operand(Operand::Reg(Register::x(rn)));
            }
            _ => {}
        }

        Ok(insn)
    }

    // =========================================================================
    // Load/Store
    // =========================================================================

    fn decode_load_store(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // AArch64 load/store encoding uses multiple bits for dispatch
        // bits [29:27] gives the major category
        let op_cat = (raw >> 27) & 0x7;

        // Check for load/store pair: bits [29:27] = 101
        // This needs to come before the 0b101 match for register!
        if op_cat == 0b101 {
            return Self::decode_load_store_pair(raw);
        }

        match op_cat {
            // 0b000, 0b001, 0b010: Exclusive, atomic, ordered
            0b000 | 0b001 | 0b010 => Self::decode_load_store_exclusive(raw),
            // 0b011: Load register literal
            0b011 => Self::decode_ldr_literal(raw),
            // 0b100, 0b110: Load/store register variants
            0b100 | 0b110 => Self::decode_load_store_register(raw),
            // 0b111: Load/store register unsigned offset
            0b111 => Self::decode_load_store_register(raw),
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Aarch64,
                raw,
                4,
            )),
        }
    }

    fn decode_load_store_exclusive(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 30) & 0x3;
        let l = (raw >> 22) & 1;
        let o0 = (raw >> 15) & 1;
        let rs = ((raw >> 16) & 0x1F) as u8;
        let o1 = (raw >> 21) & 1;
        let o2 = (raw >> 23) & 1;
        let rt2 = ((raw >> 10) & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rt = (raw & 0x1F) as u8;

        let is_64bit = size == 0b11;
        let is_pair = o1 == 1;
        let is_ordered_non_exclusive = o2 == 1;
        let is_ordered_exclusive = o0 == 1;

        if o2 == 1 && o1 == 1 {
            let mnemonic = match (l, o0) {
                (0, 0) => Mnemonic::CAS,
                (1, 0) => Mnemonic::CASA,
                (0, 1) => Mnemonic::CASL,
                (1, 1) => Mnemonic::CASAL,
                _ => unreachable!(),
            };
            return Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                .with_operand(Operand::Reg(Register::with_zr(rs, is_64bit)))
                .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
                .with_operand(Operand::Mem(MemOperand::base(Register::with_sp(rn, true)))));
        }

        let mnemonic = match (
            l,
            is_pair,
            is_ordered_non_exclusive,
            is_ordered_exclusive,
            size,
        ) {
            // Load exclusive
            (1, false, false, false, 0b00) => Mnemonic::LDXRB,
            (1, false, false, false, 0b01) => Mnemonic::LDXRH,
            (1, false, false, false, _) => Mnemonic::LDXR,
            // Load-acquire exclusive
            (1, false, false, true, 0b00) => Mnemonic::LDAXRB,
            (1, false, false, true, 0b01) => Mnemonic::LDAXRH,
            (1, false, false, true, _) => Mnemonic::LDAXR,
            // Load-acquire
            (1, false, true, true, 0b00) => Mnemonic::LDARB,
            (1, false, true, true, 0b01) => Mnemonic::LDARH,
            (1, false, true, true, _) => Mnemonic::LDAR,
            // Load exclusive pair
            (1, true, false, false, _) => Mnemonic::LDXP,
            (1, true, false, true, _) => Mnemonic::LDAXP,
            // Store exclusive
            (0, false, false, false, 0b00) => Mnemonic::STXRB,
            (0, false, false, false, 0b01) => Mnemonic::STXRH,
            (0, false, false, false, _) => Mnemonic::STXR,
            // Store-release exclusive
            (0, false, false, true, 0b00) => Mnemonic::STLXRB,
            (0, false, false, true, 0b01) => Mnemonic::STLXRH,
            (0, false, false, true, _) => Mnemonic::STLXR,
            // Store-release
            (0, false, true, true, 0b00) => Mnemonic::STLRB,
            (0, false, true, true, 0b01) => Mnemonic::STLRH,
            (0, false, true, true, _) => Mnemonic::STLR,
            // Store exclusive pair
            (0, true, false, false, _) => Mnemonic::STXP,
            (0, true, false, true, _) => Mnemonic::STLXP,
            _ => Mnemonic::UNKNOWN,
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        // For store-exclusive, Rs comes first. STLR uses o2=1 and has no
        // status register, while STLXR/STLXP still have Rs.
        if l == 0 && !is_ordered_non_exclusive {
            insn = insn.with_operand(Operand::Reg(Register::w(rs)));
        }

        insn = insn.with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)));

        if is_pair {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rt2, is_64bit)));
        }

        insn = insn.with_operand(Operand::Mem(MemOperand::base(Register::with_sp(rn, true))));

        Ok(insn)
    }

    fn decode_load_store_pair(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let opc = (raw >> 30) & 0x3;
        let v = (raw >> 26) & 1;
        let mode = (raw >> 23) & 0x3;
        let l = (raw >> 22) & 1;
        let imm7 = ((raw >> 15) & 0x7F) as i64;
        let rt2 = ((raw >> 10) & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rt = (raw & 0x1F) as u8;

        // Sign extend 7-bit immediate
        let imm7 = if imm7 & (1 << 6) != 0 {
            imm7 | !0x7F
        } else {
            imm7
        };

        let scale = if v == 0 { 2 + (opc >> 1) } else { 2 + opc };
        let offset = imm7 << scale;

        let is_64bit = opc & 1 == 1 || (opc == 0b10);

        let mnemonic = match (l, v, mode, opc) {
            // Integer no-allocate pair forms. The opc==01 form is not allocated.
            (1, 0, 0b00, 0b00 | 0b10) => Mnemonic::LDNP,
            (0, 0, 0b00, 0b00 | 0b10) => Mnemonic::STNP,
            // Integer load pair
            (1, 0, _, 0b00) => Mnemonic::LDP,
            (1, 0, 0b01 | 0b10 | 0b11, 0b01) => Mnemonic::LDPSW,
            (1, 0, _, 0b10) => Mnemonic::LDP,
            // Integer store pair
            (0, 0, _, 0b00 | 0b10) => Mnemonic::STP,
            // SIMD load/store pair
            (1, 1, _, _) => Mnemonic::LDP,
            (0, 1, _, _) => Mnemonic::STP,
            _ => Mnemonic::UNKNOWN,
        };

        let addr_mode = match mode {
            0b01 => AddressingMode::PostIndex,
            0b10 => AddressingMode::Offset,
            0b11 => AddressingMode::PreIndex,
            _ => AddressingMode::Offset,
        };

        let mem = MemOperand {
            base: Register::with_sp(rn, true),
            offset: MemOffset::Imm(offset),
            mode: addr_mode,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rt2, is_64bit)))
            .with_operand(Operand::Mem(mem)))
    }

    fn decode_load_store_register(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 30) & 0x3;
        let v = (raw >> 26) & 1;
        let opc = (raw >> 22) & 0x3;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rt = (raw & 0x1F) as u8;

        // Determine if unsigned offset or other addressing mode
        let op2 = (raw >> 10) & 0x3;
        let addr_class = (raw >> 24) & 0x3;

        let is_unsigned_imm = addr_class == 0b01;
        let bit21 = (raw >> 21) & 1;
        let is_signed_offset = !is_unsigned_imm && bit21 == 0 && op2 == 0b00;
        let is_register_offset = !is_unsigned_imm && bit21 == 1 && op2 == 0b10;

        // Atomic memory operations (FEAT_LSE): size 111 0 00 A R 1 Rs o3 opc 00 Rn Rt.
        if v == 0 && ((raw >> 24) & 1) == 0 && bit21 == 1 && op2 == 0b00 {
            return Self::decode_atomic_memory(raw);
        }

        if v == 0 && size == 0b11 && opc == 0b10 {
            let mem = if is_unsigned_imm {
                let imm12 = ((raw >> 10) & 0xFFF) as i64;
                MemOperand::imm_offset(Register::with_sp(rn, true), imm12 << 3)
            } else if is_signed_offset {
                let imm9 = ((raw >> 12) & 0x1FF) as i64;
                let imm9 = if imm9 & (1 << 8) != 0 {
                    imm9 | !0x1FF
                } else {
                    imm9
                };
                MemOperand::imm_offset(Register::with_sp(rn, true), imm9)
            } else if is_register_offset {
                let rm = ((raw >> 16) & 0x1F) as u8;
                let option = ((raw >> 13) & 0x7) as u8;
                if option & 0b010 == 0 {
                    return Ok(DecodedInsn::new(
                        Mnemonic::UNKNOWN,
                        ExecutionState::Aarch64,
                        raw,
                        4,
                    ));
                }
                let shift = if ((raw >> 12) & 1) != 0 { 3 } else { 0 };
                let rm_is_64bit = option & 0x3 == 0x3;
                MemOperand {
                    base: Register::with_sp(rn, true),
                    offset: MemOffset::ExtendedReg(ExtendedRegister::new(
                        Register::with_zr(rm, rm_is_64bit),
                        ExtendType::from_bits(option),
                        shift,
                    )),
                    mode: AddressingMode::Offset,
                }
            } else {
                return Ok(DecodedInsn::new(
                    Mnemonic::UNKNOWN,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ));
            };

            return Ok(
                DecodedInsn::new(Mnemonic::PRFM, ExecutionState::Aarch64, raw, 4)
                    .with_operand(Operand::Prfop(PrefetchOp::from_bits(rt)))
                    .with_operand(Operand::Mem(mem)),
            );
        }

        let (mnemonic, is_64bit) = match (size, v, opc) {
            // Byte
            (0b00, 0, 0b00) => (Mnemonic::STRB, false),
            (0b00, 0, 0b01) => (Mnemonic::LDRB, false),
            (0b00, 0, 0b10) => (Mnemonic::LDRSB, true), // 64-bit sign-extend
            (0b00, 0, 0b11) => (Mnemonic::LDRSB, false), // 32-bit sign-extend
            // Halfword
            (0b01, 0, 0b00) => (Mnemonic::STRH, false),
            (0b01, 0, 0b01) => (Mnemonic::LDRH, false),
            (0b01, 0, 0b10) => (Mnemonic::LDRSH, true),
            (0b01, 0, 0b11) => (Mnemonic::LDRSH, false),
            // Word
            (0b10, 0, 0b00) => (Mnemonic::STR, false),
            (0b10, 0, 0b01) => (Mnemonic::LDR, false),
            (0b10, 0, 0b10) => (Mnemonic::LDRSW, true),
            // Doubleword
            (0b11, 0, 0b00) => (Mnemonic::STR, true),
            (0b11, 0, 0b01) => (Mnemonic::LDR, true),
            // SIMD
            (_, 1, _) => {
                let is_load = opc & 1 == 1;
                if is_load {
                    (Mnemonic::LDR, true)
                } else {
                    (Mnemonic::STR, true)
                }
            }
            _ => (Mnemonic::UNKNOWN, false),
        };

        let mem = if is_unsigned_imm {
            let imm12 = ((raw >> 10) & 0xFFF) as i64;
            let scale = size as i64;
            let offset = imm12 << scale;
            MemOperand::imm_offset(Register::with_sp(rn, true), offset)
        } else if is_register_offset {
            let rm = ((raw >> 16) & 0x1F) as u8;
            let option = ((raw >> 13) & 0x7) as u8;
            if option & 0b010 == 0 {
                return Ok(DecodedInsn::new(
                    Mnemonic::UNKNOWN,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ));
            }
            let shift = if ((raw >> 12) & 1) != 0 {
                size as u8
            } else {
                0
            };
            let rm_is_64bit = option & 0x3 == 0x3;
            MemOperand {
                base: Register::with_sp(rn, true),
                offset: MemOffset::ExtendedReg(ExtendedRegister::new(
                    Register::with_zr(rm, rm_is_64bit),
                    ExtendType::from_bits(option),
                    shift,
                )),
                mode: AddressingMode::Offset,
            }
        } else {
            let imm9 = ((raw >> 12) & 0x1FF) as i64;
            let imm9 = if imm9 & (1 << 8) != 0 {
                imm9 | !0x1FF
            } else {
                imm9
            };

            match op2 {
                0b00 | 0b10 => MemOperand::imm_offset(Register::with_sp(rn, true), imm9),
                0b01 => MemOperand::post_index(Register::with_sp(rn, true), imm9),
                0b11 => MemOperand::pre_index(Register::with_sp(rn, true), imm9),
                _ => unreachable!(),
            }
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
            .with_operand(Operand::Mem(mem)))
    }

    fn decode_atomic_memory(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 30) & 0x3;
        let acquire = ((raw >> 23) & 1) != 0;
        let release = ((raw >> 22) & 1) != 0;
        let rs = ((raw >> 16) & 0x1F) as u8;
        let o3 = (raw >> 15) & 1;
        let opc = (raw >> 12) & 0x7;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rt = (raw & 0x1F) as u8;
        let is_64bit = size == 0b11;

        let suffix = (acquire, release);
        let mnemonic = if o3 == 1 {
            match (opc, suffix) {
                (0b000, (false, false)) => Mnemonic::SWP,
                (0b000, (true, false)) => Mnemonic::SWPA,
                (0b000, (false, true)) => Mnemonic::SWPL,
                (0b000, (true, true)) => Mnemonic::SWPAL,
                _ => Mnemonic::UNKNOWN,
            }
        } else {
            match (opc, suffix) {
                (0b000, (false, false)) => Mnemonic::LDADD,
                (0b000, (true, false)) => Mnemonic::LDADDA,
                (0b000, (false, true)) => Mnemonic::LDADDL,
                (0b000, (true, true)) => Mnemonic::LDADDAL,
                (0b001, _) => Mnemonic::LDCLR,
                (0b010, _) => Mnemonic::LDEOR,
                (0b011, _) => Mnemonic::LDSET,
                (0b100, _) => Mnemonic::LDSMAX,
                (0b101, _) => Mnemonic::LDSMIN,
                (0b110, _) => Mnemonic::LDUMAX,
                (0b111, _) => Mnemonic::LDUMIN,
                _ => Mnemonic::UNKNOWN,
            }
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rs, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rt, is_64bit)))
            .with_operand(Operand::Mem(MemOperand::base(Register::with_sp(rn, true)))))
    }

    // =========================================================================
    // Data Processing - Register
    // =========================================================================

    fn decode_dp_reg(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op0 = (raw >> 30) & 1;
        let op1 = (raw >> 28) & 1;
        let op2 = (raw >> 21) & 0xF;

        if op1 == 0 {
            // Logical shifted register, add/sub shifted register, add/sub extended register
            // Logical: op2 = 0xxx (bit 24 = 0)
            // Add/sub shifted: op2 = 1xx0 (bit 24 = 1, bit 21 = 0)
            // Add/sub extended: op2 = 1xx1 (bit 24 = 1, bit 21 = 1)
            if (op2 & 0x8) == 0x0 {
                Self::decode_logical_shifted_reg(raw)
            } else if (op2 & 0x9) == 0x8 {
                Self::decode_add_sub_shifted_reg(raw)
            } else if (op2 & 0x9) == 0x9 {
                Self::decode_add_sub_extended_reg(raw)
            } else {
                Ok(DecodedInsn::new(
                    Mnemonic::UNKNOWN,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ))
            }
        } else {
            // Data processing (3 source), data processing (2 source), etc.
            if op2 == 0b0110 {
                if op0 == 0 {
                    Self::decode_dp_2_source(raw)
                } else {
                    Self::decode_dp_1_source(raw)
                }
            } else if op2 == 0b0000 {
                // Add/sub with carry
                Self::decode_add_sub_carry(raw)
            } else if op2 & 0xE == 0x2 {
                // Conditional compare
                Self::decode_cond_compare(raw)
            } else if op2 & 0xE == 0x4 {
                // Conditional select
                Self::decode_cond_select(raw)
            } else if op2 & 0x8 == 0x8 {
                // Data processing (3 source)
                Self::decode_dp_3_source(raw)
            } else {
                Ok(DecodedInsn::new(
                    Mnemonic::UNKNOWN,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ))
            }
        }
    }

    fn decode_logical_shifted_reg(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let opc = (raw >> 29) & 0x3;
        let shift = ShiftType::from_bits(((raw >> 22) & 0x3) as u8);
        let n = (raw >> 21) & 1;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let imm6 = ((raw >> 10) & 0x3F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // For 32-bit, imm6 must be < 32
        if !is_64bit && imm6 >= 32 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match (opc, n) {
            (0b00, 0) => Mnemonic::AND,
            (0b00, 1) => Mnemonic::BIC,
            (0b01, 0) => Mnemonic::ORR,
            (0b01, 1) => Mnemonic::ORN,
            (0b10, 0) => Mnemonic::EOR,
            (0b10, 1) => Mnemonic::EON,
            (0b11, 0) => Mnemonic::ANDS,
            (0b11, 1) => Mnemonic::BICS,
            _ => unreachable!(),
        };

        // Check for aliases
        // MOV is ORR with Rn = XZR/WZR and shift = 0
        // MVN is ORN with Rn = XZR/WZR
        // TST is ANDS with Rd = XZR/WZR
        let (mnemonic, skip_rn, skip_rd) = if opc == 0b01 && n == 0 && rn == 31 && imm6 == 0 {
            (Mnemonic::MOV, true, false)
        } else if opc == 0b01 && n == 1 && rn == 31 {
            (Mnemonic::MVN, true, false)
        } else if opc == 0b11 && n == 0 && rd == 31 {
            (Mnemonic::TST, false, true)
        } else {
            (mnemonic, false, false)
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        if opc == 0b11 {
            insn.sets_flags = true;
        }

        if !skip_rd {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)));
        }

        if !skip_rn {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));
        }

        if imm6 == 0 && shift == ShiftType::LSL {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rm, is_64bit)));
        } else {
            insn = insn.with_operand(Operand::ShiftedReg(ShiftedRegister::new(
                Register::with_zr(rm, is_64bit),
                shift,
                imm6,
            )));
        }

        Ok(insn)
    }

    fn decode_add_sub_shifted_reg(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 30) & 1;
        let s = (raw >> 29) & 1;
        let shift = ShiftType::from_bits(((raw >> 22) & 0x3) as u8);
        let rm = ((raw >> 16) & 0x1F) as u8;
        let imm6 = ((raw >> 10) & 0x3F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // ROR is not allowed
        if shift == ShiftType::ROR {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        // For 32-bit, imm6 must be < 32
        if !is_64bit && imm6 >= 32 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match (op, s) {
            (0, 0) => Mnemonic::ADD,
            (0, 1) => Mnemonic::ADDS,
            (1, 0) => Mnemonic::SUB,
            (1, 1) => Mnemonic::SUBS,
            _ => unreachable!(),
        };

        // Check for aliases
        // NEG is SUB with Rn = XZR/WZR
        // NEGS is SUBS with Rn = XZR/WZR
        // CMP is SUBS with Rd = XZR/WZR
        // CMN is ADDS with Rd = XZR/WZR
        let (mnemonic, skip_rn, skip_rd) = if op == 1 && rn == 31 {
            if s == 0 {
                (Mnemonic::NEG, true, false)
            } else {
                (Mnemonic::NEGS, true, false)
            }
        } else if s == 1 && rd == 31 {
            if op == 0 {
                (Mnemonic::CMN, false, true)
            } else {
                (Mnemonic::CMP, false, true)
            }
        } else {
            (mnemonic, false, false)
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        if s == 1 {
            insn.sets_flags = true;
        }

        if !skip_rd {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)));
        }

        if !skip_rn {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));
        }

        if imm6 == 0 && shift == ShiftType::LSL {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rm, is_64bit)));
        } else {
            insn = insn.with_operand(Operand::ShiftedReg(ShiftedRegister::new(
                Register::with_zr(rm, is_64bit),
                shift,
                imm6,
            )));
        }

        Ok(insn)
    }

    fn decode_add_sub_extended_reg(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 30) & 1;
        let s = (raw >> 29) & 1;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let option = ((raw >> 13) & 0x7) as u8;
        let imm3 = ((raw >> 10) & 0x7) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        // imm3 must be <= 4
        if imm3 > 4 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let mnemonic = match (op, s) {
            (0, 0) => Mnemonic::ADD,
            (0, 1) => Mnemonic::ADDS,
            (1, 0) => Mnemonic::SUB,
            (1, 1) => Mnemonic::SUBS,
            _ => unreachable!(),
        };

        // Check for CMP/CMN aliases
        let (mnemonic, skip_rd) = if s == 1 && rd == 31 {
            if op == 0 {
                (Mnemonic::CMN, true)
            } else {
                (Mnemonic::CMP, true)
            }
        } else {
            (mnemonic, false)
        };

        let extend_type = ExtendType::from_bits(option);
        let rm_is_64bit = option & 0x3 == 0x3;

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        if s == 1 {
            insn.sets_flags = true;
        }

        if !skip_rd {
            insn = insn.with_operand(Operand::Reg(Register::with_sp(rd, is_64bit)));
        }

        insn = insn.with_operand(Operand::Reg(Register::with_sp(rn, is_64bit)));
        insn = insn.with_operand(Operand::ExtendedReg(ExtendedRegister::new(
            Register::with_zr(rm, rm_is_64bit),
            extend_type,
            imm3,
        )));

        Ok(insn)
    }

    fn decode_add_sub_carry(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 30) & 1;
        let s = (raw >> 29) & 1;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let is_64bit = sf == 1;

        let mnemonic = match (op, s) {
            (0, 0) => Mnemonic::ADC,
            (0, 1) => Mnemonic::ADCS,
            (1, 0) => Mnemonic::SBC,
            (1, 1) => Mnemonic::SBCS,
            _ => unreachable!(),
        };

        // NGC is SBC with Rn = XZR
        // NGCS is SBCS with Rn = XZR
        let (mnemonic, skip_rn) = if op == 1 && rn == 31 {
            if s == 0 {
                (Mnemonic::SBC, true) // Could be NGC alias
            } else {
                (Mnemonic::SBCS, true) // Could be NGCS alias
            }
        } else {
            (mnemonic, false)
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4);

        if s == 1 {
            insn.sets_flags = true;
        }

        insn = insn.with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)));

        if !skip_rn {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));
        }

        insn = insn.with_operand(Operand::Reg(Register::with_zr(rm, is_64bit)));

        Ok(insn)
    }

    fn decode_cond_compare(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 30) & 1;
        let o2 = (raw >> 10) & 1;
        let o3 = (raw >> 4) & 1;
        let rm_or_imm = ((raw >> 16) & 0x1F) as u8;
        let cond = ((raw >> 12) & 0xF) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let nzcv = (raw & 0xF) as u8;
        let is_imm = (raw >> 11) & 1 == 1;

        if o2 != 0 || o3 != 0 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let is_64bit = sf == 1;
        let mnemonic = if op == 0 {
            Mnemonic::CCMN
        } else {
            Mnemonic::CCMP
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));

        if is_imm {
            insn = insn.with_operand(Operand::Imm(Immediate::new(rm_or_imm as i64)));
        } else {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rm_or_imm, is_64bit)));
        }

        insn = insn.with_operand(Operand::Imm(Immediate::new(nzcv as i64)));
        insn = insn.with_operand(Operand::Cond(Condition::from_bits(cond)));

        Ok(insn)
    }

    fn decode_cond_select(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op = (raw >> 30) & 1;
        let s = (raw >> 29) & 1;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let cond = ((raw >> 12) & 0xF) as u8;
        let op2 = (raw >> 10) & 0x3;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        if s != 0 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let is_64bit = sf == 1;

        let mnemonic = match (op, op2) {
            (0, 0b00) => Mnemonic::CSEL,
            (0, 0b01) => Mnemonic::CSINC,
            (1, 0b00) => Mnemonic::CSINV,
            (1, 0b01) => Mnemonic::CSNEG,
            _ => {
                return Ok(DecodedInsn::new(
                    Mnemonic::UNDEFINED,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ));
            }
        };

        // Check for aliases
        // CSET is CSINC with Rn = Rm = XZR and inverted condition
        // CSETM is CSINV with Rn = Rm = XZR and inverted condition
        // CINC is CSINC with Rn = Rm and inverted condition
        // CINV is CSINV with Rn = Rm and inverted condition
        // CNEG is CSNEG with Rn = Rm and inverted condition
        let mnemonic = if rn == 31 && rm == 31 {
            match (op, op2) {
                (0, 0b01) => Mnemonic::CSET,
                (1, 0b00) => Mnemonic::CSETM,
                _ => mnemonic,
            }
        } else if rn == rm {
            match (op, op2) {
                (0, 0b01) => Mnemonic::CINC,
                (1, 0b00) => Mnemonic::CINV,
                (1, 0b01) => Mnemonic::CNEG,
                _ => mnemonic,
            }
        } else {
            mnemonic
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)));

        // For CSET/CSETM, don't add Rn/Rm.
        let is_cset_alias = rn == 31 && rm == 31 && matches!((op, op2), (0, 0b01) | (1, 0b00));
        if !is_cset_alias {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)));

            // For CINC/CINV/CNEG, don't add Rm (it's same as Rn)
            if !(rn == rm && matches!(mnemonic, Mnemonic::CINC | Mnemonic::CINV | Mnemonic::CNEG)) {
                insn = insn.with_operand(Operand::Reg(Register::with_zr(rm, is_64bit)));
            }
        }

        insn = insn.with_operand(Operand::Cond(Condition::from_bits(cond)));

        Ok(insn)
    }

    fn decode_dp_2_source(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let s = (raw >> 29) & 1;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let opcode = (raw >> 10) & 0x3F;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        if s != 0 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let is_64bit = sf == 1;

        let mnemonic = match opcode {
            0b000010 => Mnemonic::UDIV,
            0b000011 => Mnemonic::SDIV,
            0b001000 => Mnemonic::LSL,
            0b001001 => Mnemonic::LSR,
            0b001010 => Mnemonic::ASR,
            0b001011 => Mnemonic::ROR,
            // CRC32
            0b010000 if !is_64bit => Mnemonic::CRC32B,
            0b010001 if !is_64bit => Mnemonic::CRC32H,
            0b010010 if !is_64bit => Mnemonic::CRC32W,
            0b010011 if is_64bit => Mnemonic::CRC32X,
            0b010100 if !is_64bit => Mnemonic::CRC32CB,
            0b010101 if !is_64bit => Mnemonic::CRC32CH,
            0b010110 if !is_64bit => Mnemonic::CRC32CW,
            0b010111 if is_64bit => Mnemonic::CRC32CX,
            // Subp
            0b000000 if is_64bit => Mnemonic::UNKNOWN, // SUBP
            0b000001 if is_64bit => Mnemonic::UNKNOWN, // SUBPS
            // IRG, GMI
            0b000100 if is_64bit => Mnemonic::IRG,
            0b000101 if is_64bit => Mnemonic::GMI,
            // PAC
            0b001100 if is_64bit => Mnemonic::PACGA,
            _ => Mnemonic::UNKNOWN,
        };

        if matches!(
            mnemonic,
            Mnemonic::CRC32B
                | Mnemonic::CRC32H
                | Mnemonic::CRC32W
                | Mnemonic::CRC32X
                | Mnemonic::CRC32CB
                | Mnemonic::CRC32CH
                | Mnemonic::CRC32CW
                | Mnemonic::CRC32CX
        ) {
            let source_is_64bit = matches!(mnemonic, Mnemonic::CRC32X | Mnemonic::CRC32CX);
            return Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                .with_operand(Operand::Reg(Register::with_zr(rd, false)))
                .with_operand(Operand::Reg(Register::with_zr(rn, false)))
                .with_operand(Operand::Reg(Register::with_zr(rm, source_is_64bit))));
        }

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rn, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rm, is_64bit))))
    }

    fn decode_dp_1_source(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let s = (raw >> 29) & 1;
        let opcode = (raw >> 10) & 0x3F;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        if s != 0 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let is_64bit = sf == 1;
        let mnemonic = match opcode {
            0b000000 => Mnemonic::RBIT,
            0b000001 => Mnemonic::REV16,
            0b000010 if is_64bit => Mnemonic::REV32,
            0b000010 => Mnemonic::REV,
            0b000011 if is_64bit => Mnemonic::REV,
            0b000100 => Mnemonic::CLZ,
            0b000101 => Mnemonic::CLS,
            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)))
            .with_operand(Operand::Reg(Register::with_zr(rn, is_64bit))))
    }

    fn decode_dp_3_source(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let sf = (raw >> 31) & 1;
        let op54 = (raw >> 29) & 0x3;
        let op31 = (raw >> 21) & 0x7;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let o0 = (raw >> 15) & 1;
        let ra = ((raw >> 10) & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        if op54 != 0 {
            return Ok(DecodedInsn::new(
                Mnemonic::UNDEFINED,
                ExecutionState::Aarch64,
                raw,
                4,
            ));
        }

        let is_64bit = sf == 1;

        let mnemonic = match (sf, op31, o0) {
            (_, 0b000, 0) => Mnemonic::MADD,
            (_, 0b000, 1) => Mnemonic::MSUB,
            (1, 0b001, 0) => Mnemonic::SMADDL,
            (1, 0b001, 1) => Mnemonic::SMSUBL,
            (1, 0b010, 0) => Mnemonic::SMULH,
            (1, 0b101, 0) => Mnemonic::UMADDL,
            (1, 0b101, 1) => Mnemonic::UMSUBL,
            (1, 0b110, 0) => Mnemonic::UMULH,
            _ => Mnemonic::UNDEFINED,
        };

        // Check for MUL/MNEG aliases
        // MUL is MADD with Ra = XZR
        // MNEG is MSUB with Ra = XZR
        let (mnemonic, skip_ra) = if ra == 31 {
            match mnemonic {
                Mnemonic::MADD => (Mnemonic::MUL, true),
                Mnemonic::MSUB => (Mnemonic::MNEG, true),
                Mnemonic::SMADDL => (Mnemonic::SMULL, true),
                Mnemonic::UMADDL => (Mnemonic::UMULL, true),
                _ => (mnemonic, false),
            }
        } else {
            (mnemonic, false)
        };

        let mut insn = DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::Reg(Register::with_zr(rd, is_64bit)));

        // For long multiplies, Rn and Rm are 32-bit
        let src_64bit = if matches!(
            mnemonic,
            Mnemonic::SMADDL
                | Mnemonic::SMSUBL
                | Mnemonic::UMADDL
                | Mnemonic::UMSUBL
                | Mnemonic::SMULL
                | Mnemonic::UMULL
        ) {
            false
        } else {
            is_64bit
        };

        insn = insn.with_operand(Operand::Reg(Register::with_zr(rn, src_64bit)));
        insn = insn.with_operand(Operand::Reg(Register::with_zr(rm, src_64bit)));

        if !skip_ra {
            insn = insn.with_operand(Operand::Reg(Register::with_zr(ra, is_64bit)));
        }

        Ok(insn)
    }

    // =========================================================================
    // SIMD and FP
    // =========================================================================

    fn decode_simd_fp(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op0 = (raw >> 28) & 0xF;
        let op1 = (raw >> 23) & 0x3;
        let op2 = (raw >> 19) & 0xF;
        let op3 = (raw >> 10) & 0x1FF;

        // Bits [31:30] = Q:U for many SIMD ops
        let q = (raw >> 30) & 1;
        let u = (raw >> 29) & 1;

        // SIMD three-same: [31:30]=Qx, [28:24]=0111x, [21]=1, [15:11]=opcode
        // Encoding: 0_Q_U_01110_size_1_Rm_opcode_1_Rn_Rd
        if (raw >> 24) & 0x1F == 0b01110 && (raw >> 21) & 1 == 1 && (raw >> 10) & 1 == 1 {
            return Self::decode_simd_three_same(raw, q, u);
        }

        // SIMD two-reg misc: [31:30]=Qx, [28:24]=0111x, [21:17]=10000, [16:12]=opcode
        if (raw >> 24) & 0x1F == 0b01110 && (raw >> 17) & 0x1F == 0b10000 {
            return Self::decode_simd_two_reg_misc(raw, q, u);
        }

        // SIMD across lanes: [31:30]=Qx, [28:24]=0111x, [21:17]=11000
        if (raw >> 24) & 0x1F == 0b01110 && (raw >> 17) & 0x1F == 0b11000 {
            return Self::decode_simd_across_lanes(raw, q, u);
        }

        // SIMD scalar pairwise: [31:30]=01, [28:24]=11110
        if (raw >> 30) & 0x3 == 0b01 && (raw >> 24) & 0x1F == 0b11110 {
            return Self::decode_simd_scalar_pairwise(raw);
        }

        // SIMD copy (DUP, MOV, INS): [31:30]=0x, [28:24]=01110, [21]=0
        if (raw >> 24) & 0x1F == 0b01110 && (raw >> 21) & 1 == 0 {
            return Self::decode_simd_copy(raw, q);
        }

        // Advanced SIMD scalar three-same: [31:30]=01, [28:24]=11110
        // Advanced SIMD scalar two-reg misc: similar encoding
        if (raw >> 30) & 0x3 == 0b01 && (raw >> 24) & 0x1F == 0b11110 {
            return Self::decode_simd_scalar_three(raw, u);
        }

        // Scalar FP data-processing: [31:30]=00, [28:24]=11110
        if (raw >> 30) & 0x3 == 0b00 && (raw >> 24) & 0x1F == 0b11110 {
            return Self::decode_scalar_fp(raw);
        }

        // SIMD load/store: various encodings
        if (raw >> 24) & 0x1F == 0b01100 || (raw >> 24) & 0x1F == 0b01101 {
            return Self::decode_simd_ldst(raw, q);
        }

        Ok(DecodedInsn::new(
            Mnemonic::UNKNOWN,
            ExecutionState::Aarch64,
            raw,
            4,
        ))
    }

    /// Decode SIMD three-same register instructions.
    fn decode_simd_three_same(raw: u32, q: u32, u: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 22) & 0x3;
        let opcode = (raw >> 11) & 0x1F;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let vec_size = if q == 1 { FpRegSize::Q } else { FpRegSize::D };
        let fp_reg = |num| {
            Operand::FpReg(FpRegister {
                num,
                size: vec_size,
            })
        };

        let mnemonic = match (u, opcode) {
            // Integer operations
            (0, 0b00000) => Mnemonic::VADD, // SHADD actually, simplified
            (0, 0b00001) => Mnemonic::VADD, // SQADD
            (0, 0b00010) => Mnemonic::VSUB, // SRHADD
            (0, 0b00100) => Mnemonic::VSUB, // SHSUB
            (0, 0b00110) => Mnemonic::VMAX, // SMAX
            (0, 0b00111) => Mnemonic::VMIN, // SMIN
            (0, 0b10000) => Mnemonic::VADD, // ADD
            (0, 0b10001) => Mnemonic::VMLA, // CMTST
            (0, 0b10011) => Mnemonic::VMUL, // MUL
            (0, 0b10100) => Mnemonic::VMAX, // SMAXP
            (0, 0b10101) => Mnemonic::VMIN, // SMINP
            (0, 0b10111) => Mnemonic::VADD, // ADDP

            (1, 0b00000) => Mnemonic::VADD, // UHADD
            (1, 0b00001) => Mnemonic::VADD, // UQADD
            (1, 0b00010) => Mnemonic::VSUB, // URHADD
            (1, 0b00100) => Mnemonic::VSUB, // UHSUB
            (1, 0b00110) => Mnemonic::VMAX, // UMAX
            (1, 0b00111) => Mnemonic::VMIN, // UMIN
            (1, 0b10000) => Mnemonic::VSUB, // SUB
            (1, 0b10001) => Mnemonic::VCMP, // CMEQ
            (1, 0b10100) => Mnemonic::VMAX, // UMAXP
            (1, 0b10101) => Mnemonic::VMIN, // UMINP

            // Logical operations
            (0, 0b00011) if size == 0b00 => Mnemonic::VAND,
            (0, 0b00011) if size == 0b01 => Mnemonic::VBIC,
            (0, 0b00011) if size == 0b10 => Mnemonic::VORR,
            (0, 0b00011) if size == 0b11 => Mnemonic::VEOR, // ORN actually

            (1, 0b00011) if size == 0b00 => Mnemonic::VEOR,
            (1, 0b00011) if size == 0b01 => Mnemonic::VBIC, // BSL
            (1, 0b00011) if size == 0b10 => Mnemonic::VORR, // BIT
            (1, 0b00011) if size == 0b11 => Mnemonic::VORR, // BIF

            // FP operations (size bit 0 = single, bit 1 = double when Q=1)
            (0, 0b11000) => Mnemonic::FADD, // FMAXNM
            (0, 0b11001) => Mnemonic::FMLA, // FMLA
            (0, 0b11010) => Mnemonic::FADD, // FADD
            (0, 0b11011) => Mnemonic::FMUL, // FMULX
            (0, 0b11100) => Mnemonic::FCMP, // FCMEQ
            (0, 0b11110) => Mnemonic::FMAX, // FMAX
            (0, 0b11111) => Mnemonic::FADD, // FRECPS

            (1, 0b11000) => Mnemonic::FSUB, // FMINNM
            (1, 0b11001) => Mnemonic::FMLS, // FMLS
            (1, 0b11010) => Mnemonic::FSUB, // FSUB
            (1, 0b11101) => Mnemonic::FMUL, // FMUL
            (1, 0b11110) => Mnemonic::FMIN, // FMIN
            (1, 0b11111) => Mnemonic::FDIV, // FDIV (or FRSQRTS)

            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(fp_reg(rd))
            .with_operand(fp_reg(rn))
            .with_operand(fp_reg(rm)))
    }

    /// Decode SIMD two-register miscellaneous instructions.
    fn decode_simd_two_reg_misc(raw: u32, q: u32, u: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 22) & 0x3;
        let opcode = (raw >> 12) & 0x1F;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let vec_size = if q == 1 { FpRegSize::Q } else { FpRegSize::D };
        let fp_reg = |num| {
            Operand::FpReg(FpRegister {
                num,
                size: vec_size,
            })
        };

        let mnemonic = match (u, opcode) {
            // Integer unary
            (0, 0b00000) => Mnemonic::VNEG, // REV64
            (0, 0b00001) => Mnemonic::VNEG, // REV16
            (0, 0b00101) => Mnemonic::VNEG, // CNT
            (0, 0b01000) => Mnemonic::VCMP, // CMGT #0
            (0, 0b01001) => Mnemonic::VCMP, // CMEQ #0
            (0, 0b01010) => Mnemonic::VCMP, // CMLT #0
            (0, 0b01011) => Mnemonic::VABS, // ABS
            (0, 0b10100) => Mnemonic::VCMP, // CLS

            (1, 0b00000) => Mnemonic::VNEG, // REV32
            (1, 0b00101) => Mnemonic::VMVN, // NOT / MVN
            (1, 0b01000) => Mnemonic::VCMP, // CMGE #0
            (1, 0b01001) => Mnemonic::VCMP, // CMLE #0
            (1, 0b01011) => Mnemonic::VNEG, // NEG
            (1, 0b10100) => Mnemonic::VCMP, // CLZ

            // FP unary
            (0, 0b01100) => Mnemonic::FCMP,   // FCMGT #0
            (0, 0b01101) => Mnemonic::FCMP,   // FCMEQ #0
            (0, 0b01110) => Mnemonic::FCMP,   // FCMLT #0
            (0, 0b01111) => Mnemonic::FABS,   // FABS
            (0, 0b11000) => Mnemonic::FRINT,  // FRINTN
            (0, 0b11001) => Mnemonic::FRINT,  // FRINTM
            (0, 0b11010) => Mnemonic::FCVT,   // FCVTNS
            (0, 0b11011) => Mnemonic::FCVT,   // FCVTMS
            (0, 0b11100) => Mnemonic::FCVT,   // FCVTAS
            (0, 0b11101) => Mnemonic::SCVTF,  // SCVTF
            (0, 0b11111) => Mnemonic::VRECPE, // FRECPE

            (1, 0b01100) => Mnemonic::FCMP,    // FCMGE #0
            (1, 0b01101) => Mnemonic::FCMP,    // FCMLE #0
            (1, 0b01111) => Mnemonic::FNEG,    // FNEG
            (1, 0b11000) => Mnemonic::FRINT,   // FRINTP
            (1, 0b11001) => Mnemonic::FRINT,   // FRINTZ
            (1, 0b11010) => Mnemonic::FCVT,    // FCVTPS
            (1, 0b11011) => Mnemonic::FCVTZS,  // FCVTZS
            (1, 0b11101) => Mnemonic::UCVTF,   // UCVTF
            (1, 0b11111) => Mnemonic::VRSQRTE, // FRSQRTE
            (1, 0b10111) => Mnemonic::FSQRT,   // FSQRT

            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(fp_reg(rd))
            .with_operand(fp_reg(rn)))
    }

    /// Decode SIMD across-lanes instructions (reduction).
    fn decode_simd_across_lanes(raw: u32, q: u32, u: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 22) & 0x3;
        let opcode = (raw >> 12) & 0x1F;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let vec_size = if q == 1 { FpRegSize::Q } else { FpRegSize::D };
        let scalar_size = match size {
            0b00 => FpRegSize::B,
            0b01 => FpRegSize::H,
            0b10 => FpRegSize::S,
            0b11 => FpRegSize::D,
            _ => FpRegSize::S,
        };

        let mnemonic = match (u, opcode) {
            (0, 0b00011) => Mnemonic::SADDV, // SADDLV
            (0, 0b01010) => Mnemonic::SMAXV, // SMAXV
            (0, 0b11010) => Mnemonic::SMINV, // SMINV
            (0, 0b11011) => Mnemonic::VADD,  // ADDV

            (1, 0b00011) => Mnemonic::UADDV, // UADDLV
            (1, 0b01010) => Mnemonic::UMAXV, // UMAXV
            (1, 0b11010) => Mnemonic::UMINV, // UMINV

            // FP across lanes
            (0, 0b01100) => Mnemonic::FMAXNM, // FMAXNMV
            (0, 0b01111) => Mnemonic::FMAX,   // FMAXV
            (1, 0b01100) => Mnemonic::FMINNM, // FMINNMV
            (1, 0b01111) => Mnemonic::FMIN,   // FMINV

            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::FpReg(FpRegister {
                num: rd,
                size: scalar_size,
            }))
            .with_operand(Operand::FpReg(FpRegister {
                num: rn,
                size: vec_size,
            })))
    }

    /// Decode SIMD scalar pairwise instructions.
    fn decode_simd_scalar_pairwise(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let u = (raw >> 29) & 1;
        let size = (raw >> 22) & 0x3;
        let opcode = (raw >> 12) & 0x1F;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let fp_size = if size & 1 == 0 {
            FpRegSize::S
        } else {
            FpRegSize::D
        };
        let fp_reg = |num| Operand::FpReg(FpRegister { num, size: fp_size });

        let mnemonic = match (u, opcode) {
            (0, 0b01100) => Mnemonic::FMAXNM, // FMAXNMP
            (0, 0b01101) => Mnemonic::FADD,   // FADDP
            (0, 0b01111) => Mnemonic::FMAX,   // FMAXP
            (1, 0b01100) => Mnemonic::FMINNM, // FMINNMP
            (1, 0b01111) => Mnemonic::FMIN,   // FMINP
            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(fp_reg(rd))
            .with_operand(fp_reg(rn)))
    }

    /// Decode SIMD copy instructions (DUP, MOV, INS).
    fn decode_simd_copy(raw: u32, q: u32) -> Result<DecodedInsn, DecodeError> {
        let op = (raw >> 29) & 1;
        let imm5 = (raw >> 16) & 0x1F;
        let imm4 = (raw >> 11) & 0xF;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let vec_size = if q == 1 { FpRegSize::Q } else { FpRegSize::D };

        // Determine element size from imm5
        let (esize, idx) = if imm5 & 1 != 0 {
            (FpRegSize::B, (imm5 >> 1) as u8)
        } else if imm5 & 2 != 0 {
            (FpRegSize::H, (imm5 >> 2) as u8)
        } else if imm5 & 4 != 0 {
            (FpRegSize::S, (imm5 >> 3) as u8)
        } else if imm5 & 8 != 0 {
            (FpRegSize::D, (imm5 >> 4) as u8)
        } else {
            (FpRegSize::B, 0)
        };

        let mnemonic = match (op, imm4) {
            (0, 0b0000) => Mnemonic::VDUP, // DUP (element)
            (0, 0b0001) => Mnemonic::VDUP, // DUP (general)
            (0, 0b0101) => Mnemonic::VMOV, // SMOV
            (0, 0b0111) => Mnemonic::VMOV, // UMOV
            (1, _) => Mnemonic::VMOV,      // INS (general or element)
            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::FpReg(FpRegister {
                num: rd,
                size: vec_size,
            }))
            .with_operand(Operand::FpReg(FpRegister {
                num: rn,
                size: esize,
            })))
    }

    /// Decode SIMD scalar three-same instructions.
    fn decode_simd_scalar_three(raw: u32, u: u32) -> Result<DecodedInsn, DecodeError> {
        let size = (raw >> 22) & 0x3;
        let opcode = (raw >> 11) & 0x1F;
        let rm = ((raw >> 16) & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let fp_size = if size & 1 == 0 {
            FpRegSize::S
        } else {
            FpRegSize::D
        };
        let fp_reg = |num| Operand::FpReg(FpRegister { num, size: fp_size });

        let mnemonic = match (u, opcode) {
            // Scalar FP
            (0, 0b11010) => Mnemonic::FADD,
            (0, 0b11011) => Mnemonic::FMUL,
            (0, 0b11110) => Mnemonic::FMAX,
            (0, 0b11111) => Mnemonic::FMIN,

            (1, 0b11010) => Mnemonic::FSUB,
            (1, 0b11101) => Mnemonic::FMUL,
            (1, 0b11111) => Mnemonic::FDIV,

            _ => Mnemonic::UNKNOWN,
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(fp_reg(rd))
            .with_operand(fp_reg(rn))
            .with_operand(fp_reg(rm)))
    }

    /// Decode SIMD load/store instructions.
    fn decode_simd_ldst(raw: u32, q: u32) -> Result<DecodedInsn, DecodeError> {
        let l = (raw >> 22) & 1; // 1=load, 0=store
        let opcode = (raw >> 12) & 0xF;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rt = (raw & 0x1F) as u8;

        let vec_size = if q == 1 { FpRegSize::Q } else { FpRegSize::D };

        let mnemonic = if l == 1 {
            match opcode {
                0b0000 | 0b0010 => Mnemonic::VLD1,
                0b0100 | 0b0110 => Mnemonic::VLD2,
                0b0001 | 0b0011 => Mnemonic::VLD3,
                0b0101 | 0b0111 => Mnemonic::VLD4,
                _ => Mnemonic::VLDR,
            }
        } else {
            match opcode {
                0b0000 | 0b0010 => Mnemonic::VST1,
                0b0100 | 0b0110 => Mnemonic::VST2,
                0b0001 | 0b0011 => Mnemonic::VST3,
                0b0101 | 0b0111 => Mnemonic::VST4,
                _ => Mnemonic::VSTR,
            }
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(Operand::FpReg(FpRegister {
                num: rt,
                size: vec_size,
            }))
            .with_operand(Operand::Mem(MemOperand::base(Register::x(rn)))))
    }

    fn decode_scalar_fp(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let m = (raw >> 31) & 1;
        let s = (raw >> 29) & 1;
        let ptype = (raw >> 22) & 0x3;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let rd = (raw & 0x1F) as u8;

        let fp_size = match ptype {
            0b00 => FpRegSize::S,
            0b01 => FpRegSize::D,
            0b11 => FpRegSize::H,
            _ => {
                return Ok(DecodedInsn::new(
                    Mnemonic::UNDEFINED,
                    ExecutionState::Aarch64,
                    raw,
                    4,
                ));
            }
        };

        // Get opcode bits for determining operation
        let opcode = (raw >> 10) & 0x3F;

        // Common FP operations
        let mnemonic = match opcode & 0xF {
            0b0000 => Mnemonic::FMOV,
            0b0001 => Mnemonic::FABS,
            0b0010 => Mnemonic::FNEG,
            0b0011 => Mnemonic::FSQRT,
            0b0100 => Mnemonic::FCVT,
            0b1000 => Mnemonic::FRINTN,
            0b1001 => Mnemonic::FRINTP,
            0b1010 => Mnemonic::FRINTM,
            0b1011 => Mnemonic::FRINTZ,
            0b1100 => Mnemonic::FRINTA,
            0b1110 => Mnemonic::FRINTX,
            0b1111 => Mnemonic::FRINTI,
            _ => Mnemonic::UNKNOWN,
        };

        let fp_reg = |num, size| Operand::FpReg(FpRegister { num, size });

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(fp_reg(rd, fp_size))
            .with_operand(fp_reg(rn, fp_size)))
    }

    // =========================================================================
    // SVE
    // =========================================================================

    /// Decode SVE (Scalable Vector Extension) instructions.
    ///
    /// SVE encoding space: bits[28:25] = 0010
    /// Further classification by bits[24:21] and other fields.
    fn decode_sve(raw: u32) -> Result<DecodedInsn, DecodeError> {
        // Extract primary classification bits
        let op0 = (raw >> 29) & 0x7; // bits[31:29]
        let op1 = (raw >> 23) & 0x3; // bits[24:23]
        let op2 = (raw >> 17) & 0x1F; // bits[21:17]
        let op3 = (raw >> 10) & 0x3F; // bits[15:10]

        // Common register fields
        let zd = (raw & 0x1F) as u8;
        let zn = ((raw >> 5) & 0x1F) as u8;
        let zm = ((raw >> 16) & 0x1F) as u8;
        let pg = ((raw >> 10) & 0x7) as u8; // predicate register (3 bits)
        let size = (raw >> 22) & 0x3; // element size

        // Create SVE register operands
        let sve_z = |num| Operand::Reg(Register::sve_z(num));
        let sve_p = |num| Operand::Reg(Register::sve_p(num));

        match op0 {
            // Integer multiply-add predicated
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x10) == 0 && (op3 & 0x20) != 0 => {
                let op = (raw >> 13) & 1;
                let mnemonic = match op {
                    0 => Mnemonic::SVE_MLA,
                    1 => Mnemonic::SVE_MLS,
                    _ => unreachable!(),
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zn))
                    .with_operand(sve_z(zm)))
            }

            // Integer predicated binary operations
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x10) == 0 && (op3 & 0x38) == 0 => {
                let opc = (raw >> 16) & 0x7;
                let mnemonic = match opc {
                    0b000 => Mnemonic::SVE_ADD,
                    0b001 => Mnemonic::SVE_SUB,
                    0b011 => Mnemonic::SVE_SUBR,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zn)))
            }

            // Integer min/max predicated
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x18) == 0x08 => {
                let opc = (raw >> 17) & 0x3;
                let u = (raw >> 16) & 1;
                let mnemonic = match (opc, u) {
                    (0b00, 0) => Mnemonic::SVE_SMAX,
                    (0b00, 1) => Mnemonic::SVE_UMAX,
                    (0b01, 0) => Mnemonic::SVE_SMIN,
                    (0b01, 1) => Mnemonic::SVE_UMIN,
                    (0b10, 0) => Mnemonic::SVE_SABD,
                    (0b10, 1) => Mnemonic::SVE_UABD,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zm)))
            }

            // Integer multiply/divide predicated
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x18) == 0x10 => {
                let h = (raw >> 17) & 1;
                let u = (raw >> 16) & 1;
                let mnemonic = match (h, u) {
                    (0, 0) => Mnemonic::SVE_MUL,
                    (1, 0) => Mnemonic::SVE_SMULH,
                    (1, 1) => Mnemonic::SVE_UMULH,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zm)))
            }

            // Integer divide predicated
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x18) == 0x14 => {
                let r = (raw >> 17) & 1;
                let u = (raw >> 16) & 1;
                let mnemonic = match (r, u) {
                    (0, 0) => Mnemonic::SVE_SDIV,
                    (0, 1) => Mnemonic::SVE_UDIV,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zm)))
            }

            // Logical predicated
            0b000 if (op1 & 0x2) == 0 && (op2 & 0x18) == 0x18 => {
                let opc = (raw >> 16) & 0x7;
                let mnemonic = match opc {
                    0b000 => Mnemonic::SVE_ORR,
                    0b001 => Mnemonic::SVE_EOR,
                    0b010 => Mnemonic::SVE_AND,
                    0b011 => Mnemonic::SVE_BIC,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zm)))
            }

            // Unpredicated arithmetic
            0b000 if op1 == 0b01 => {
                let opc = (raw >> 10) & 0x7;
                let mnemonic = match opc {
                    0b000 => Mnemonic::SVE_ADD,
                    0b001 => Mnemonic::SVE_SUB,
                    0b100 => Mnemonic::SVE_MUL, // sqadd actually, simplified
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_z(zn))
                    .with_operand(sve_z(zm)))
            }

            // Predicate operations
            0b001 if op1 == 0b00 => {
                // While instructions
                let lt = (raw >> 4) & 1;
                let sf = (raw >> 12) & 1;
                let mnemonic = if lt == 1 {
                    Mnemonic::SVE_WHILELT
                } else {
                    Mnemonic::SVE_WHILELE
                };
                let pd = (raw & 0xF) as u8;
                let rn = ((raw >> 5) & 0x1F) as u8;
                let rm = ((raw >> 16) & 0x1F) as u8;
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_p(pd))
                    .with_operand(Operand::Reg(if sf == 1 {
                        Register::x(rn)
                    } else {
                        Register::w(rn)
                    }))
                    .with_operand(Operand::Reg(if sf == 1 {
                        Register::x(rm)
                    } else {
                        Register::w(rm)
                    })))
            }

            // PTRUE/PFALSE
            0b001 if op1 == 0b01 && (op3 & 0x30) == 0x10 => {
                let s = (raw >> 16) & 1;
                let mnemonic = if s == 0 {
                    Mnemonic::SVE_PTRUE
                } else {
                    Mnemonic::SVE_PFALSE
                };
                let pd = (raw & 0xF) as u8;
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_p(pd)))
            }

            // DUP/SEL/MOV
            0b000 if op1 == 0b10 => {
                let mnemonic = Mnemonic::SVE_DUP;
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_z(zn)))
            }

            // INDEX
            0b000 if op1 == 0b11 && (op2 & 0x10) == 0 => {
                let mnemonic = Mnemonic::SVE_INDEX;
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(Operand::Reg(Register::x(zn)))
                    .with_operand(Operand::Reg(Register::x(zm))))
            }

            // Count/Inc/Dec
            0b000 if op1 == 0b11 && (op2 & 0x18) == 0x10 => {
                let opc = (raw >> 16) & 0x7;
                let mnemonic = match opc {
                    0b000 => Mnemonic::SVE_CNTB,
                    0b001 => Mnemonic::SVE_CNTH,
                    0b010 => Mnemonic::SVE_CNTW,
                    0b011 => Mnemonic::SVE_CNTD,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(Operand::Reg(Register::x(zd))))
            }

            // Permute - ZIP/UZP/TRN
            0b000 if op1 == 0b10 && (op3 & 0x30) == 0x00 => {
                let opc = (raw >> 10) & 0x7;
                let mnemonic = match opc {
                    0b000 => Mnemonic::SVE_ZIP1,
                    0b001 => Mnemonic::SVE_ZIP2,
                    0b010 => Mnemonic::SVE_UZP1,
                    0b011 => Mnemonic::SVE_UZP2,
                    0b100 => Mnemonic::SVE_TRN1,
                    0b101 => Mnemonic::SVE_TRN2,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_z(zn))
                    .with_operand(sve_z(zm)))
            }

            // REV
            0b000 if op1 == 0b10 && (op3 & 0x38) == 0x18 => {
                let opc = (raw >> 16) & 0x3;
                let mnemonic = match opc {
                    0b00 => Mnemonic::SVE_REV,
                    0b01 => Mnemonic::SVE_REVB,
                    0b10 => Mnemonic::SVE_REVH,
                    0b11 => Mnemonic::SVE_REVW,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_z(zn)))
            }

            // Load/Store contiguous
            0b100 | 0b101 | 0b110 | 0b111 => Self::decode_sve_ldst(raw),

            // FP arithmetic predicated
            0b011 if (op1 & 0x2) == 0 => {
                let opc = (raw >> 16) & 0xF;
                let mnemonic = match opc {
                    0b0000 => Mnemonic::SVE_FADD,
                    0b0001 => Mnemonic::SVE_FSUB,
                    0b0010 => Mnemonic::SVE_FMUL,
                    0b0011 => Mnemonic::SVE_FDIV,
                    0b0100 => Mnemonic::SVE_FMIN,
                    0b0101 => Mnemonic::SVE_FMAX,
                    0b1000 => Mnemonic::SVE_FABS,
                    0b1001 => Mnemonic::SVE_FNEG,
                    0b1010 => Mnemonic::SVE_FSQRT,
                    _ => Mnemonic::UNKNOWN,
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zn)))
            }

            // FP multiply-add predicated
            0b011 if op1 == 0b01 => {
                let op = (raw >> 13) & 1;
                let mnemonic = match op {
                    0 => Mnemonic::SVE_FMLA,
                    1 => Mnemonic::SVE_FMLS,
                    _ => unreachable!(),
                };
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(sve_z(zd))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zn))
                    .with_operand(sve_z(zm)))
            }

            // Reduction operations
            0b000 if (op1 & 0x2) == 0x2 && (op3 & 0x20) != 0 => {
                let opc = (raw >> 16) & 0x7;
                let u = (raw >> 19) & 1;
                let mnemonic = match (opc, u) {
                    (0b000, 0) => Mnemonic::SVE_SADDV,
                    (0b000, 1) => Mnemonic::SVE_UADDV,
                    (0b001, 0) => Mnemonic::SVE_SMAXV,
                    (0b001, 1) => Mnemonic::SVE_UMAXV,
                    (0b010, 0) => Mnemonic::SVE_SMINV,
                    (0b010, 1) => Mnemonic::SVE_UMINV,
                    (0b011, _) => Mnemonic::SVE_ANDV,
                    (0b100, _) => Mnemonic::SVE_ORV,
                    (0b101, _) => Mnemonic::SVE_EORV,
                    _ => Mnemonic::UNKNOWN,
                };
                let vd = zd; // destination is a scalar V register
                Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
                    .with_operand(Operand::FpReg(FpRegister {
                        num: vd,
                        size: FpRegSize::D,
                    }))
                    .with_operand(sve_p(pg))
                    .with_operand(sve_z(zn)))
            }

            // RDVL - read vector length
            0b000 if op1 == 0b11 && op2 == 0x1F && (op3 & 0x3E) == 0x10 => {
                let rd = zd;
                let imm6 = ((raw >> 5) & 0x3F) as i8;
                // Sign extend
                let imm = if imm6 & 0x20 != 0 {
                    imm6 as i64 | !0x3F
                } else {
                    imm6 as i64
                };
                Ok(
                    DecodedInsn::new(Mnemonic::SVE_RDVL, ExecutionState::Aarch64, raw, 4)
                        .with_operand(Operand::Reg(Register::x(rd)))
                        .with_operand(Operand::Imm(Immediate::new(imm))),
                )
            }

            // Fallback for unrecognized patterns
            _ => Ok(DecodedInsn::new(
                Mnemonic::UNKNOWN,
                ExecutionState::Aarch64,
                raw,
                4,
            )),
        }
    }

    /// Decode SVE load/store instructions.
    fn decode_sve_ldst(raw: u32) -> Result<DecodedInsn, DecodeError> {
        let op0 = (raw >> 29) & 0x7;
        let msz = (raw >> 23) & 0x3; // memory element size
        let opc = (raw >> 21) & 0x3;

        let zt = (raw & 0x1F) as u8;
        let rn = ((raw >> 5) & 0x1F) as u8;
        let pg = ((raw >> 10) & 0x7) as u8;

        let sve_z = |num| Operand::Reg(Register::sve_z(num));
        let sve_p = |num| Operand::Reg(Register::sve_p(num));

        // Determine if load or store and element size
        let is_load = (op0 & 0x1) == 0;

        let mnemonic = if is_load {
            match msz {
                0b00 => Mnemonic::SVE_LD1B,
                0b01 => Mnemonic::SVE_LD1H,
                0b10 => Mnemonic::SVE_LD1W,
                0b11 => Mnemonic::SVE_LD1D,
                _ => Mnemonic::SVE_LD1,
            }
        } else {
            match msz {
                0b00 => Mnemonic::SVE_ST1B,
                0b01 => Mnemonic::SVE_ST1H,
                0b10 => Mnemonic::SVE_ST1W,
                0b11 => Mnemonic::SVE_ST1D,
                _ => Mnemonic::SVE_ST1,
            }
        };

        // Extract immediate offset if present
        let imm4 = ((raw >> 16) & 0xF) as i64;
        let offset = if imm4 & 0x8 != 0 {
            imm4 | !0xF // sign extend
        } else {
            imm4
        };

        Ok(DecodedInsn::new(mnemonic, ExecutionState::Aarch64, raw, 4)
            .with_operand(sve_z(zt))
            .with_operand(sve_p(pg))
            .with_operand(Operand::Mem(MemOperand::base_offset(
                Register::x(rn),
                offset * (1 << msz), // scale by element size
            ))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to decode little-endian bytes
    fn decode_bytes(bytes: &[u8; 4]) -> Result<DecodedInsn, DecodeError> {
        let raw = u32::from_le_bytes(*bytes);
        Aarch64Decoder::decode(raw)
    }

    #[test]
    fn test_nop() {
        // NOP: d503201f
        let insn = decode_bytes(&[0x1f, 0x20, 0x03, 0xd5]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::NOP);
        assert_eq!(insn.size, 4);
    }

    #[test]
    fn test_ret() {
        // RET: d65f03c0
        let insn = decode_bytes(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::RET);
    }

    #[test]
    fn test_mov_imm() {
        // MOV X0, #1 -> MOVZ X0, #1: d2800020
        let insn = decode_bytes(&[0x20, 0x00, 0x80, 0xd2]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MOVZ);
        assert_eq!(insn.operands.len(), 2);
    }

    #[test]
    fn test_add_imm() {
        // ADD X0, X1, #0x10: 91004020
        let insn = decode_bytes(&[0x20, 0x40, 0x00, 0x91]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADD);
        assert_eq!(insn.operands.len(), 3);
    }

    #[test]
    fn test_atomic_swp_decode() {
        // SWP X2, X0, [X1].
        let insn = Aarch64Decoder::decode(
            (0b11 << 30) | (0b111 << 27) | (1 << 21) | (2 << 16) | (1 << 15) | (1 << 5),
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SWP);
        assert_eq!(insn.operands.len(), 3);

        // SWPAL W2, W0, [X1].
        let insn = Aarch64Decoder::decode(
            (0b111 << 27) | (1 << 23) | (1 << 22) | (1 << 21) | (2 << 16) | (1 << 15) | (1 << 5),
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SWPAL);
        assert_eq!(insn.operands.len(), 3);
    }

    #[test]
    fn test_atomic_cas_decode() {
        // CAS X2, X0, [X1].
        let insn = Aarch64Decoder::decode(
            (0b11 << 30)
                | (0b001000 << 24)
                | (1 << 23)
                | (1 << 21)
                | (2 << 16)
                | (0b11111 << 10)
                | (1 << 5),
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CAS);
        assert_eq!(insn.operands.len(), 3);

        // CASAL W2, W0, [X1].
        let insn = Aarch64Decoder::decode(
            (0b001000 << 24)
                | (1 << 23)
                | (1 << 22)
                | (1 << 21)
                | (2 << 16)
                | (1 << 15)
                | (0b11111 << 10)
                | (1 << 5),
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CASAL);
        assert_eq!(insn.operands.len(), 3);
    }

    #[test]
    fn test_store_exclusive_release_decode_operands() {
        // LDAXR X0, [X1].
        let insn = Aarch64Decoder::decode(
            (0b11 << 30)
                | (0b001000 << 24)
                | (1 << 22)
                | (0b11111 << 16)
                | (1 << 15)
                | (0b11111 << 10)
                | (1 << 5),
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LDAXR);
        assert_eq!(insn.operands.len(), 2);

        // STLXR W2, X3, [X1].
        let insn = Aarch64Decoder::decode(
            (0b11 << 30)
                | (0b001000 << 24)
                | (2 << 16)
                | (1 << 15)
                | (0b11111 << 10)
                | (1 << 5)
                | 3,
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::STLXR);
        assert_eq!(insn.operands.len(), 3);

        // STLR X3, [X1] has no status operand.
        let insn = Aarch64Decoder::decode(
            (0b11 << 30)
                | (0b001000 << 24)
                | (1 << 23)
                | (0b11111 << 16)
                | (1 << 15)
                | (0b11111 << 10)
                | (1 << 5)
                | 3,
        )
        .unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::STLR);
        assert_eq!(insn.operands.len(), 2);
    }

    #[test]
    fn test_clrex_decode() {
        // CLREX: d5033f5f.
        let insn = decode_bytes(&[0x5f, 0x3f, 0x03, 0xd5]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CLREX);
        assert_eq!(insn.operands.len(), 1);
    }

    #[test]
    fn test_sub_imm() {
        // SUB X0, X1, #0x10: d1004020
        let insn = decode_bytes(&[0x20, 0x40, 0x00, 0xd1]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SUB);
    }

    #[test]
    fn test_add_sub_imm_tags() {
        // ADDG X0, X1, #0x10, #0: 91810020
        let insn = decode_bytes(&[0x20, 0x00, 0x81, 0x91]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADDG);
        assert_eq!(insn.operands.len(), 4);
        if let Some(Operand::Imm(offset)) = insn.operands.get(2) {
            assert_eq!(offset.effective_value(), 0x10);
        } else {
            panic!("Expected ADDG offset immediate");
        }

        // SUBG X0, X1, #0x10, #0: d1810020
        let insn = decode_bytes(&[0x20, 0x00, 0x81, 0xd1]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SUBG);
    }

    #[test]
    fn test_cmp_imm() {
        // CMP X0, #0 -> SUBS XZR, X0, #0: f100001f
        let insn = decode_bytes(&[0x1f, 0x00, 0x00, 0xf1]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CMP);
        assert!(insn.sets_flags);
    }

    #[test]
    fn test_b() {
        // B #0x100: 14000040
        let insn = decode_bytes(&[0x40, 0x00, 0x00, 0x14]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::B);
        if let Some(Operand::Label(offset)) = insn.operands.first() {
            assert_eq!(*offset, 0x100);
        } else {
            panic!("Expected label operand");
        }
    }

    #[test]
    fn test_bl() {
        // BL #0x100: 94000040
        let insn = decode_bytes(&[0x40, 0x00, 0x00, 0x94]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BL);
    }

    #[test]
    fn test_cbz() {
        // CBZ X0, #0x10: b4000080
        let insn = decode_bytes(&[0x80, 0x00, 0x00, 0xb4]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CBZ);
    }

    #[test]
    fn test_cbnz() {
        // CBNZ W0, #0x10: 35000080
        let insn = decode_bytes(&[0x80, 0x00, 0x00, 0x35]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CBNZ);
    }

    #[test]
    fn test_ldr_imm() {
        // LDR X0, [X1]: f9400020
        let insn = decode_bytes(&[0x20, 0x00, 0x40, 0xf9]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LDR);
        assert_eq!(insn.operands.len(), 2);
    }

    #[test]
    fn test_str_imm() {
        // STR X0, [X1]: f9000020
        let insn = decode_bytes(&[0x20, 0x00, 0x00, 0xf9]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::STR);
    }

    #[test]
    fn test_ldp() {
        // LDP X0, X1, [SP]: a9400be0
        let insn = decode_bytes(&[0xe0, 0x07, 0x40, 0xa9]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LDP);
        assert_eq!(insn.operands.len(), 3);
    }

    #[test]
    fn test_stp() {
        // STP X29, X30, [SP, #-16]!: a9bf7bfd
        let insn = decode_bytes(&[0xfd, 0x7b, 0xbf, 0xa9]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::STP);
    }

    #[test]
    fn test_svc() {
        // SVC #0: d4000001
        let insn = decode_bytes(&[0x01, 0x00, 0x00, 0xd4]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SVC);
    }

    #[test]
    fn test_brk() {
        // BRK #0: d4200000
        let insn = decode_bytes(&[0x00, 0x00, 0x20, 0xd4]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BRK);
    }

    #[test]
    fn test_and_shifted_reg() {
        // AND X0, X1, X2: 8a020020
        let insn = decode_bytes(&[0x20, 0x00, 0x02, 0x8a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::AND);
    }

    #[test]
    fn test_orr_shifted_reg() {
        // ORR X0, X1, X2: aa020020
        let insn = decode_bytes(&[0x20, 0x00, 0x02, 0xaa]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ORR);
    }

    #[test]
    fn test_mov_reg() {
        // MOV X0, X1 -> ORR X0, XZR, X1: aa0103e0
        let insn = decode_bytes(&[0xe0, 0x03, 0x01, 0xaa]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MOV);
    }

    #[test]
    fn test_madd() {
        // MADD X0, X1, X2, X3: 9b020c20
        let insn = decode_bytes(&[0x20, 0x0c, 0x02, 0x9b]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MADD);
    }

    #[test]
    fn test_mul() {
        // MUL X0, X1, X2 -> MADD X0, X1, X2, XZR: 9b027c20
        let insn = decode_bytes(&[0x20, 0x7c, 0x02, 0x9b]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MUL);
    }

    #[test]
    fn test_udiv() {
        // UDIV X0, X1, X2: 9ac20820
        let insn = decode_bytes(&[0x20, 0x08, 0xc2, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::UDIV);
    }

    #[test]
    fn test_sdiv() {
        // SDIV X0, X1, X2: 9ac20c20
        let insn = decode_bytes(&[0x20, 0x0c, 0xc2, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::SDIV);
    }

    #[test]
    fn test_csel() {
        // CSEL X0, X1, X2, EQ: 9a820020
        let insn = decode_bytes(&[0x20, 0x00, 0x82, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CSEL);
    }

    #[test]
    fn test_csinv_keeps_source_operands() {
        // CSINV X0, X1, X2, NE: da821020
        let insn = decode_bytes(&[0x20, 0x10, 0x82, 0xda]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CSINV);
        assert_eq!(insn.operands.len(), 4);
        assert!(matches!(insn.operands.get(1), Some(Operand::Reg(reg)) if reg.num == 1));
        assert!(matches!(insn.operands.get(2), Some(Operand::Reg(reg)) if reg.num == 2));
        assert!(matches!(
            insn.operands.get(3),
            Some(Operand::Cond(Condition::NE))
        ));
    }

    #[test]
    fn test_csetm_omits_source_operands() {
        // CSETM X0, EQ (CSINV X0, XZR, XZR, NE): da9f13e0
        let insn = decode_bytes(&[0xe0, 0x13, 0x9f, 0xda]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CSETM);
        assert_eq!(insn.operands.len(), 2);
        assert!(matches!(
            insn.operands.get(1),
            Some(Operand::Cond(Condition::NE))
        ));
    }

    #[test]
    fn test_adc() {
        // ADC X0, X1, X2: 9a020020
        let insn = decode_bytes(&[0x20, 0x00, 0x02, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADC);
    }

    #[test]
    fn test_rbit() {
        // RBIT X0, X1: dac00020
        let insn = decode_bytes(&[0x20, 0x00, 0xc0, 0xda]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::RBIT);
    }

    #[test]
    fn test_rev() {
        // REV X0, X1: dac00c20
        let insn = decode_bytes(&[0x20, 0x0c, 0xc0, 0xda]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::REV);

        // REV W0, W1: 5ac00820
        let insn = decode_bytes(&[0x20, 0x08, 0xc0, 0x5a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::REV);
    }

    #[test]
    fn test_clz() {
        // CLZ X0, X1: dac01020
        let insn = decode_bytes(&[0x20, 0x10, 0xc0, 0xda]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::CLZ);
    }

    #[test]
    fn test_lsl_reg() {
        // LSL X0, X1, X2: 9ac22020
        let insn = decode_bytes(&[0x20, 0x20, 0xc2, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LSL);
    }

    #[test]
    fn test_lsr_reg() {
        // LSR X0, X1, X2: 9ac22420
        let insn = decode_bytes(&[0x20, 0x24, 0xc2, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::LSR);
    }

    #[test]
    fn test_asr_reg() {
        // ASR X0, X1, X2: 9ac22820
        let insn = decode_bytes(&[0x20, 0x28, 0xc2, 0x9a]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ASR);
    }

    #[test]
    fn test_tbz() {
        // TBZ X0, #0, #0x10: 36000080
        let insn = decode_bytes(&[0x80, 0x00, 0x00, 0x36]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::TBZ);
    }

    #[test]
    fn test_tbnz() {
        // TBNZ X0, #0, #0x10: 37000080
        let insn = decode_bytes(&[0x80, 0x00, 0x00, 0x37]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::TBNZ);
    }

    #[test]
    fn test_br() {
        // BR X0: d61f0000
        let insn = decode_bytes(&[0x00, 0x00, 0x1f, 0xd6]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BR);
    }

    #[test]
    fn test_blr() {
        // BLR X0: d63f0000
        let insn = decode_bytes(&[0x00, 0x00, 0x3f, 0xd6]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BLR);
    }

    #[test]
    fn test_neg() {
        // NEG X0, X1 -> SUB X0, XZR, X1: cb0103e0
        let insn = decode_bytes(&[0xe0, 0x03, 0x01, 0xcb]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::NEG);
    }

    #[test]
    fn test_mvn() {
        // MVN X0, X1 -> ORN X0, XZR, X1: aa2103e0
        let insn = decode_bytes(&[0xe0, 0x03, 0x21, 0xaa]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::MVN);
    }

    #[test]
    fn test_tst_shifted_reg() {
        // TST X0, X1 -> ANDS XZR, X0, X1: ea01001f
        let insn = decode_bytes(&[0x1f, 0x00, 0x01, 0xea]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::TST);
    }

    #[test]
    fn test_32bit_operations() {
        // ADD W0, W1, W2: 0b020020
        let insn = decode_bytes(&[0x20, 0x00, 0x02, 0x0b]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADD);

        if let Some(Operand::Reg(reg)) = insn.operands.first() {
            assert!(!reg.is_64bit);
        }
    }

    #[test]
    fn test_conditional_branch() {
        // B.EQ #0x10: 54000080
        let insn = decode_bytes(&[0x80, 0x00, 0x00, 0x54]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::BCC);
        assert_eq!(insn.cond, Some(Condition::EQ));
    }

    // =========================================================================
    // ADR/ADRP - PC-relative addressing (ASL compliance tests)
    // ASL Reference: arch_decode.asl PC-rel addressing
    // Encoding: op (bit 31), immlo (bits 30:29), 10000 (bits 28:24),
    //           immhi (bits 23:5), Rd (bits 4:0)
    // =========================================================================

    #[test]
    fn test_adr_basic() {
        // ADR X0, #0x40 (bit 31 = 0 means ADR)
        // Encoding: 0 00 10000 0000000000000100 00000 = 0x10000800
        let insn = decode_bytes(&[0x00, 0x08, 0x00, 0x10]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADR);
        assert_eq!(insn.state, ExecutionState::Aarch64);
    }

    #[test]
    fn test_adr_different_register() {
        // ADR X15, #0x40
        let insn = decode_bytes(&[0x0f, 0x08, 0x00, 0x10]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADR);
        // Check destination register is X15
        if let Some(Operand::Reg(reg)) = insn.operands.first() {
            assert_eq!(reg.num, 15);
            assert!(reg.is_64bit);
        } else {
            panic!("Expected register operand");
        }
    }

    #[test]
    fn test_adr_negative_offset() {
        // ADR X0, #-4 (negative offset uses sign-extended immediate)
        // Encoding: op=0, immlo=11, 10000, immhi=1111111111111111111, rd=00000
        // op(31)=0, immlo(30:29)=11, 10000(28:24), immhi(23:5)=all 1s, rd(4:0)=0
        // = 0 11 10000 1111111111111111111 00000
        // = 0111_0000_1111_1111_1111_1111_1110_0000
        // = 0x70FFFFE0
        let insn = decode_bytes(&[0xe0, 0xff, 0xff, 0x70]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADR);
        // Check the label is negative
        if let Some(Operand::Label(offset)) = insn.operands.get(1) {
            assert!(*offset < 0, "Expected negative offset, got {}", offset);
        } else {
            panic!("Expected label operand");
        }
    }

    #[test]
    fn test_adrp_basic() {
        // ADRP X0, #0x1000 (bit 31 = 1 means ADRP)
        // Encoding: 1 00 10000 0000000000000001 00000 = 0x90000020
        let insn = decode_bytes(&[0x20, 0x00, 0x00, 0x90]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADRP);
        assert_eq!(insn.state, ExecutionState::Aarch64);
    }

    #[test]
    fn test_adrp_different_register() {
        // ADRP X30, #0x1000
        let insn = decode_bytes(&[0x3e, 0x00, 0x00, 0x90]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADRP);
        // Check destination register is X30
        if let Some(Operand::Reg(reg)) = insn.operands.first() {
            assert_eq!(reg.num, 30);
            assert!(reg.is_64bit);
        } else {
            panic!("Expected register operand");
        }
    }

    #[test]
    fn test_adrp_page_aligned() {
        // ADRP produces 4KB page-aligned addresses (imm << 12)
        // ADRP X0, #0x2000 -> label should be 0x2000 (page 2)
        let insn = decode_bytes(&[0x40, 0x00, 0x00, 0x90]).unwrap();
        assert_eq!(insn.mnemonic, Mnemonic::ADRP);
        if let Some(Operand::Label(offset)) = insn.operands.get(1) {
            // The offset should be a multiple of 4096 (page size)
            assert_eq!(*offset % 4096, 0);
        }
    }

    #[test]
    fn test_adr_vs_adrp_bit31_difference() {
        // Same encoding except bit 31 - should produce different mnemonics
        // Base encoding with immlo=0, immhi=1, rd=0

        // ADR: bit 31 = 0 -> 0x10000020
        let adr_insn = decode_bytes(&[0x20, 0x00, 0x00, 0x10]).unwrap();
        assert_eq!(adr_insn.mnemonic, Mnemonic::ADR);

        // ADRP: bit 31 = 1 -> 0x90000020
        let adrp_insn = decode_bytes(&[0x20, 0x00, 0x00, 0x90]).unwrap();
        assert_eq!(adrp_insn.mnemonic, Mnemonic::ADRP);
    }
}
