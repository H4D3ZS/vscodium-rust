//! Oracle evaluation integration for test generation.
//!
//! This module provides functions to compute expected values for tests
//! by integrating the Oracle with the test generation pipeline.

use std::collections::HashMap;

use super::{Environment, Oracle, Value};
use crate::testgen::types::*;

/// Computes expected register values and flags for an arithmetic operation.
pub struct ArithmeticResult {
    /// The computed result value
    pub result: u64,
    /// Expected destination register value after execution
    pub dest_value: u64,
    /// Expected NZCV flags if instruction sets flags
    pub nzcv: Option<NzcvFlags>,
}

/// NZCV flag values
#[derive(Debug, Clone, Copy, Default)]
pub struct NzcvFlags {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

impl NzcvFlags {
    pub fn to_u8(&self) -> u8 {
        ((self.n as u8) << 3) | ((self.z as u8) << 2) | ((self.c as u8) << 1) | (self.v as u8)
    }
}

/// Evaluator that computes expected values for instruction tests.
pub struct TestEvaluator {
    oracle: Oracle,
}

impl TestEvaluator {
    pub fn new() -> Self {
        Self {
            oracle: Oracle::new(),
        }
    }

    /// Evaluate an ADD/ADDS instruction
    pub fn eval_add(
        &self,
        operand1: u64,
        operand2: u64,
        width: u32,
        set_flags: bool,
    ) -> ArithmeticResult {
        let (result, n, z, c, v) = self.oracle.eval_add(operand1, operand2, width);

        // For 32-bit operations, upper 32 bits are zeroed
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: if set_flags {
                Some(NzcvFlags { n, z, c, v })
            } else {
                None
            },
        }
    }

    /// Evaluate a SUB/SUBS instruction
    pub fn eval_sub(
        &self,
        operand1: u64,
        operand2: u64,
        width: u32,
        set_flags: bool,
    ) -> ArithmeticResult {
        let (result, n, z, c, v) = self.oracle.eval_sub(operand1, operand2, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: if set_flags {
                Some(NzcvFlags { n, z, c, v })
            } else {
                None
            },
        }
    }

    /// Evaluate an AND/ANDS instruction
    pub fn eval_and(
        &self,
        operand1: u64,
        operand2: u64,
        width: u32,
        set_flags: bool,
    ) -> ArithmeticResult {
        let (result, n, z) = self.oracle.eval_and(operand1, operand2, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: if set_flags {
                Some(NzcvFlags {
                    n,
                    z,
                    c: false,
                    v: false,
                })
            } else {
                None
            },
        }
    }

    /// Evaluate an ORR instruction
    pub fn eval_or(&self, operand1: u64, operand2: u64, width: u32) -> ArithmeticResult {
        let (result, n, z) = self.oracle.eval_or(operand1, operand2, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate an EOR instruction
    pub fn eval_xor(&self, operand1: u64, operand2: u64, width: u32) -> ArithmeticResult {
        let (result, n, z) = self.oracle.eval_xor(operand1, operand2, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate a MOV (register copy or immediate)
    pub fn eval_mov(&self, value: u64, width: u32) -> ArithmeticResult {
        let dest_value = if width == 32 {
            value & 0xFFFF_FFFF
        } else {
            value
        };

        ArithmeticResult {
            result: value,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate an LSL shift
    pub fn eval_lsl(&self, value: u64, shift: u32, width: u32) -> ArithmeticResult {
        let (result, _carry) = self.oracle.eval_lsl(value, shift, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate an LSR shift
    pub fn eval_lsr(&self, value: u64, shift: u32, width: u32) -> ArithmeticResult {
        let (result, _carry) = self.oracle.eval_lsr(value, shift, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate an ASR shift
    pub fn eval_asr(&self, value: u64, shift: u32, width: u32) -> ArithmeticResult {
        let (result, _carry) = self.oracle.eval_asr(value, shift, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate a ROR rotate
    pub fn eval_ror(&self, value: u64, shift: u32, width: u32) -> ArithmeticResult {
        let (result, _carry) = self.oracle.eval_ror(value, shift, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate a sign extension
    pub fn eval_sxt(&self, value: u64, from_width: u32, to_width: u32) -> ArithmeticResult {
        let result = self.oracle.sign_extend(value, from_width, to_width);

        let dest_value = if to_width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate a zero extension
    pub fn eval_uxt(&self, value: u64, from_width: u32, to_width: u32) -> ArithmeticResult {
        let result = self.oracle.zero_extend(value, from_width, to_width);

        let dest_value = if to_width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate CLZ (count leading zeros)
    pub fn eval_clz(&self, value: u64, width: u32) -> ArithmeticResult {
        let result = self.oracle.count_leading_zeros(value, width) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate CLS (count leading sign bits)
    pub fn eval_cls(&self, value: u64, width: u32) -> ArithmeticResult {
        let result = self.oracle.count_leading_sign_bits(value, width) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate REV (reverse bytes)
    pub fn eval_rev(&self, value: u64, width: u32) -> ArithmeticResult {
        let result = self.oracle.reverse_bytes(value, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate RBIT (reverse bits)
    pub fn eval_rbit(&self, value: u64, width: u32) -> ArithmeticResult {
        let result = self.oracle.reverse_bits(value, width);

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate REV16 (reverse bytes in each halfword)
    pub fn eval_rev16(&self, value: u64, width: u32) -> ArithmeticResult {
        let result = if width == 32 {
            let hw0 = ((value & 0xFF) << 8) | ((value >> 8) & 0xFF);
            let hw1 = ((value >> 16) & 0xFF) << 8 | ((value >> 24) & 0xFF);
            (hw1 << 16) | hw0
        } else {
            let hw0 = ((value & 0xFF) << 8) | ((value >> 8) & 0xFF);
            let hw1 = ((value >> 16) & 0xFF) << 8 | ((value >> 24) & 0xFF);
            let hw2 = ((value >> 32) & 0xFF) << 8 | ((value >> 40) & 0xFF);
            let hw3 = ((value >> 48) & 0xFF) << 8 | ((value >> 56) & 0xFF);
            (hw3 << 48) | (hw2 << 32) | (hw1 << 16) | hw0
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate REV32 (reverse bytes in each word, 64-bit only)
    pub fn eval_rev32(&self, value: u64) -> ArithmeticResult {
        let w0 = value & 0xFFFF_FFFF;
        let w1 = (value >> 32) & 0xFFFF_FFFF;
        let rev_w0 = w0.swap_bytes() >> 32;
        let rev_w1 = w1.swap_bytes() >> 32;
        let result = (rev_w1 << 32) | rev_w0;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate MUL (multiply)
    pub fn eval_mul(&self, operand1: u64, operand2: u64, width: u32) -> ArithmeticResult {
        let result = operand1.wrapping_mul(operand2);
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate MADD (multiply-add: Rd = Ra + Rn * Rm)
    pub fn eval_madd(
        &self,
        operand1: u64,
        operand2: u64,
        addend: u64,
        width: u32,
    ) -> ArithmeticResult {
        let product = operand1.wrapping_mul(operand2);
        let result = addend.wrapping_add(product);
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate MSUB (multiply-subtract: Rd = Ra - Rn * Rm)
    pub fn eval_msub(
        &self,
        operand1: u64,
        operand2: u64,
        addend: u64,
        width: u32,
    ) -> ArithmeticResult {
        let product = operand1.wrapping_mul(operand2);
        let result = addend.wrapping_sub(product);
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate SMULL (signed multiply long: 32x32->64)
    pub fn eval_smull(&self, operand1: u64, operand2: u64) -> ArithmeticResult {
        let op1 = (operand1 & 0xFFFF_FFFF) as i32 as i64;
        let op2 = (operand2 & 0xFFFF_FFFF) as i32 as i64;
        let result = (op1.wrapping_mul(op2)) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate UMULL (unsigned multiply long: 32x32->64)
    pub fn eval_umull(&self, operand1: u64, operand2: u64) -> ArithmeticResult {
        let op1 = operand1 & 0xFFFF_FFFF;
        let op2 = operand2 & 0xFFFF_FFFF;
        let result = op1.wrapping_mul(op2);

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate SMADDL (signed multiply-add long: 32x32+64->64)
    pub fn eval_smaddl(&self, operand1: u64, operand2: u64, addend: u64) -> ArithmeticResult {
        let op1 = (operand1 & 0xFFFF_FFFF) as i32 as i64;
        let op2 = (operand2 & 0xFFFF_FFFF) as i32 as i64;
        let product = op1.wrapping_mul(op2) as u64;
        let result = addend.wrapping_add(product);

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate UMADDL (unsigned multiply-add long: 32x32+64->64)
    pub fn eval_umaddl(&self, operand1: u64, operand2: u64, addend: u64) -> ArithmeticResult {
        let op1 = operand1 & 0xFFFF_FFFF;
        let op2 = operand2 & 0xFFFF_FFFF;
        let product = op1.wrapping_mul(op2);
        let result = addend.wrapping_add(product);

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate SMULH (signed multiply high: 64x64->high 64 bits)
    pub fn eval_smulh(&self, operand1: u64, operand2: u64) -> ArithmeticResult {
        let op1 = operand1 as i64 as i128;
        let op2 = operand2 as i64 as i128;
        let product = op1.wrapping_mul(op2);
        let result = (product >> 64) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate UMULH (unsigned multiply high: 64x64->high 64 bits)
    pub fn eval_umulh(&self, operand1: u64, operand2: u64) -> ArithmeticResult {
        let op1 = operand1 as u128;
        let op2 = operand2 as u128;
        let product = op1.wrapping_mul(op2);
        let result = (product >> 64) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate SDIV (signed divide)
    pub fn eval_sdiv(&self, dividend: u64, divisor: u64, width: u32) -> ArithmeticResult {
        let result = if width == 32 {
            let n = dividend as i32;
            let d = divisor as i32;
            if d == 0 {
                0u64
            } else if n == i32::MIN && d == -1 {
                // Overflow case: result is dividend
                n as u32 as u64
            } else {
                (n.wrapping_div(d)) as u32 as u64
            }
        } else {
            let n = dividend as i64;
            let d = divisor as i64;
            if d == 0 {
                0u64
            } else if n == i64::MIN && d == -1 {
                n as u64
            } else {
                (n.wrapping_div(d)) as u64
            }
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate UDIV (unsigned divide)
    pub fn eval_udiv(&self, dividend: u64, divisor: u64, width: u32) -> ArithmeticResult {
        let result = if width == 32 {
            let n = (dividend & 0xFFFF_FFFF) as u32;
            let d = (divisor & 0xFFFF_FFFF) as u32;
            if d == 0 {
                0u64
            } else {
                (n / d) as u64
            }
        } else {
            if divisor == 0 {
                0u64
            } else {
                dividend / divisor
            }
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate SBFM (signed bitfield move)
    /// LSB/ASR/SBFIZ/SBFX/SXTB/SXTH/SXTW are all aliases of SBFM
    pub fn eval_sbfm(&self, value: u64, immr: u32, imms: u32, width: u32) -> ArithmeticResult {
        // SBFM: extract bitfield, sign-extend, and optionally rotate
        let src = if width == 32 {
            value & 0xFFFF_FFFF
        } else {
            value
        };

        let result = if imms >= immr {
            // SBFX case: extract bits [imms:immr], sign extend
            let len = imms - immr + 1;
            let extracted = (src >> immr) & ((1u64 << len) - 1);
            // Sign extend from bit (len-1)
            if len < width && (extracted >> (len - 1)) & 1 != 0 {
                let sign_bits = if width == 32 {
                    0xFFFF_FFFF_FFFF_FFFFu64 << len
                } else {
                    u64::MAX << len
                };
                extracted | sign_bits
            } else {
                extracted
            }
        } else {
            // SBFIZ case: insert bitfield at position immr
            let len = imms + 1;
            let lsb = width - immr;
            let extracted = src & ((1u64 << len) - 1);
            let inserted = extracted << lsb;
            // Sign extend from bit (lsb + len - 1)
            let sign_bit = lsb + len - 1;
            if sign_bit < width && (inserted >> sign_bit) & 1 != 0 {
                let sign_bits = u64::MAX << (sign_bit + 1);
                inserted | sign_bits
            } else {
                inserted
            }
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate UBFM (unsigned bitfield move)
    /// LSR/LSL/UBFIZ/UBFX/UXTB/UXTH are all aliases of UBFM
    pub fn eval_ubfm(&self, value: u64, immr: u32, imms: u32, width: u32) -> ArithmeticResult {
        let src = if width == 32 {
            value & 0xFFFF_FFFF
        } else {
            value
        };

        let result = if imms >= immr {
            // UBFX case: extract bits [imms:immr], zero extend
            let len = imms - immr + 1;
            (src >> immr) & ((1u64 << len) - 1)
        } else {
            // UBFIZ case: insert bitfield at position (width - immr)
            let len = imms + 1;
            let lsb = width - immr;
            let extracted = src & ((1u64 << len) - 1);
            extracted << lsb
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate BFM (bitfield move - inserts into destination)
    pub fn eval_bfm(
        &self,
        dest: u64,
        src: u64,
        immr: u32,
        imms: u32,
        width: u32,
    ) -> ArithmeticResult {
        let src_val = if width == 32 { src & 0xFFFF_FFFF } else { src };
        let dst_val = if width == 32 {
            dest & 0xFFFF_FFFF
        } else {
            dest
        };

        let result = if imms >= immr {
            // BFX case: copy bits from src to dst
            let len = imms - immr + 1;
            let mask = ((1u64 << len) - 1) << 0; // destination lsb = 0
            let extracted = (src_val >> immr) & ((1u64 << len) - 1);
            (dst_val & !mask) | extracted
        } else {
            // BFI case: insert bits from src
            let len = imms + 1;
            let lsb = width - immr;
            let mask = ((1u64 << len) - 1) << lsb;
            let extracted = src_val & ((1u64 << len) - 1);
            (dst_val & !mask) | (extracted << lsb)
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate EXTR (extract register - concatenate and extract)
    pub fn eval_extr(&self, rn: u64, rm: u64, lsb: u32, width: u32) -> ArithmeticResult {
        let rn_val = if width == 32 { rn & 0xFFFF_FFFF } else { rn };
        let rm_val = if width == 32 { rm & 0xFFFF_FFFF } else { rm };

        // Concatenate Rn:Rm and extract bits [lsb+width-1:lsb]
        let result = if width == 32 {
            let concat = ((rn_val as u64) << 32) | (rm_val as u64);
            (concat >> lsb) & 0xFFFF_FFFF
        } else {
            let concat_hi = rn_val;
            let concat_lo = rm_val;
            if lsb == 0 {
                concat_lo
            } else if lsb >= 64 {
                concat_hi >> (lsb - 64)
            } else {
                (concat_hi << (64 - lsb)) | (concat_lo >> lsb)
            }
        };

        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate CSEL (conditional select: if cond then Rn else Rm)
    pub fn eval_csel(&self, rn: u64, rm: u64, cond_true: bool, width: u32) -> ArithmeticResult {
        let result = if cond_true { rn } else { rm };
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate CSINC (conditional select increment: if cond then Rn else Rm+1)
    pub fn eval_csinc(&self, rn: u64, rm: u64, cond_true: bool, width: u32) -> ArithmeticResult {
        let result = if cond_true { rn } else { rm.wrapping_add(1) };
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate CSINV (conditional select invert: if cond then Rn else ~Rm)
    pub fn eval_csinv(&self, rn: u64, rm: u64, cond_true: bool, width: u32) -> ArithmeticResult {
        let result = if cond_true { rn } else { !rm };
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate CSNEG (conditional select negate: if cond then Rn else -Rm)
    pub fn eval_csneg(&self, rn: u64, rm: u64, cond_true: bool, width: u32) -> ArithmeticResult {
        let result = if cond_true { rn } else { (-(rm as i64)) as u64 };
        let dest_value = if width == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        };

        ArithmeticResult {
            result,
            dest_value,
            nzcv: None,
        }
    }

    /// Evaluate ADR (PC-relative address)
    pub fn eval_adr(&self, pc: u64, imm21: i64) -> ArithmeticResult {
        let result = (pc as i64).wrapping_add(imm21) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Evaluate ADRP (PC-relative page address)
    pub fn eval_adrp(&self, pc: u64, imm21: i64) -> ArithmeticResult {
        // ADRP: base is PC with low 12 bits cleared, offset is imm21 << 12
        let base = pc & !0xFFF;
        let offset = imm21 << 12;
        let result = (base as i64).wrapping_add(offset) as u64;

        ArithmeticResult {
            result,
            dest_value: result,
            nzcv: None,
        }
    }

    /// Compute expected state for an ADD/SUB immediate instruction
    pub fn compute_add_sub_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        // Decode the instruction fields
        let sf = (encoding >> 31) & 1;
        let op = (encoding >> 30) & 1;
        let s = (encoding >> 29) & 1;
        let sh = (encoding >> 22) & 1;
        let imm12 = (encoding >> 10) & 0xFFF;
        let rn = ((encoding >> 5) & 0x1F) as u8;
        let rd = (encoding & 0x1F) as u8;

        let width = if sf == 1 { 64 } else { 32 };
        let set_flags = s == 1;

        // Get immediate value (with optional shift)
        let imm = if sh == 1 { imm12 << 12 } else { imm12 };

        // Get source register value
        let operand1 = if rn == 31 {
            // SP (not ZR for Rn in add/sub imm)
            initial_state.sp.unwrap_or(0)
        } else {
            *initial_state.gp_regs.get(&rn).unwrap_or(&0)
        };

        // Compute result
        let result = if op == 0 {
            // ADD
            self.eval_add(operand1, imm, width, set_flags)
        } else {
            // SUB
            self.eval_sub(operand1, imm, width, set_flags)
        };

        let mut expected_state = ProcessorState::default();
        let mut assertions = Vec::new();

        // Set expected destination register
        if rd == 31 {
            if set_flags {
                // ADDS/SUBS with Rd=31: result discarded, but still aliases ZR
                // Don't set any GP reg, but ZR should read as 0
            } else {
                // ADD/SUB with Rd=31: writes to SP
                expected_state.sp = Some(result.dest_value);
                assertions.push(TestAssertion {
                    check: AssertionCheck::Sp,
                    expected: AssertionValue::U64(result.dest_value),
                    message: format!("SP should be 0x{:X}", result.dest_value),
                });
            }
        } else {
            expected_state.gp_regs.insert(rd, result.dest_value);
            if width == 32 {
                assertions.push(TestAssertion {
                    check: AssertionCheck::GpReg32(rd),
                    expected: AssertionValue::U32(result.dest_value as u32),
                    message: format!("W{} should be 0x{:X}", rd, result.dest_value as u32),
                });
            } else {
                assertions.push(TestAssertion {
                    check: AssertionCheck::GpReg(rd),
                    expected: AssertionValue::U64(result.dest_value),
                    message: format!("X{} should be 0x{:016X}", rd, result.dest_value),
                });
            }
        }

        // Set expected flags if applicable
        if let Some(nzcv) = result.nzcv {
            expected_state.nzcv = Some(nzcv.to_u8());

            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(nzcv.n),
                message: format!("N flag should be {}", nzcv.n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(nzcv.z),
                message: format!("Z flag should be {}", nzcv.z),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::C),
                expected: AssertionValue::Bool(nzcv.c),
                message: format!("C flag should be {}", nzcv.c),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::V),
                expected: AssertionValue::Bool(nzcv.v),
                message: format!("V flag should be {}", nzcv.v),
            });
        }

        (expected_state, assertions)
    }

    /// Compute expected state for a logical immediate instruction (AND, ORR, EOR)
    pub fn compute_logical_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        // Decode the instruction fields
        let sf = (encoding >> 31) & 1;
        let opc = (encoding >> 29) & 3;
        let n = (encoding >> 22) & 1;
        let immr = ((encoding >> 16) & 0x3F) as u32;
        let imms = ((encoding >> 10) & 0x3F) as u32;
        let rn = ((encoding >> 5) & 0x1F) as u8;
        let rd = (encoding & 0x1F) as u8;

        let width = if sf == 1 { 64 } else { 32 };

        // Decode the bitmask immediate (simplified - full decode is complex)
        let imm = self.decode_bitmask_immediate(n as u32, imms, immr, width);

        // Get source register value
        let operand1 = if rn == 31 {
            0 // ZR
        } else {
            *initial_state.gp_regs.get(&rn).unwrap_or(&0)
        };

        // Compute result based on operation
        let (result, set_flags) = match opc {
            0b00 => (self.eval_and(operand1, imm, width, false), false), // AND
            0b01 => (self.eval_or(operand1, imm, width), false),         // ORR
            0b10 => (self.eval_xor(operand1, imm, width), false),        // EOR
            0b11 => (self.eval_and(operand1, imm, width, true), true),   // ANDS
            _ => unreachable!(),
        };

        let mut expected_state = ProcessorState::default();
        let mut assertions = Vec::new();

        // Set expected destination register
        if rd == 31 {
            if set_flags {
                // ANDS with Rd=31: result discarded
            } else {
                // AND/ORR/EOR with Rd=31: writes to SP
                expected_state.sp = Some(result.dest_value);
            }
        } else {
            expected_state.gp_regs.insert(rd, result.dest_value);
            if width == 32 {
                assertions.push(TestAssertion {
                    check: AssertionCheck::GpReg32(rd),
                    expected: AssertionValue::U32(result.dest_value as u32),
                    message: format!("W{} should be 0x{:X}", rd, result.dest_value as u32),
                });
            } else {
                assertions.push(TestAssertion {
                    check: AssertionCheck::GpReg(rd),
                    expected: AssertionValue::U64(result.dest_value),
                    message: format!("X{} should be 0x{:016X}", rd, result.dest_value),
                });
            }
        }

        // Set expected flags if applicable
        if let Some(nzcv) = result.nzcv {
            expected_state.nzcv = Some(nzcv.to_u8());

            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(nzcv.n),
                message: format!("N flag should be {}", nzcv.n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(nzcv.z),
                message: format!("Z flag should be {}", nzcv.z),
            });
        }

        (expected_state, assertions)
    }

    /// Decode a bitmask immediate (simplified version)
    fn decode_bitmask_immediate(&self, n: u32, imms: u32, immr: u32, reg_size: u32) -> u64 {
        // This is a simplified version - the full decode is quite complex
        // For now, return a placeholder value based on the immediate fields
        let len = if n == 1 {
            6
        } else {
            // Find highest set bit in ~imms
            let not_imms = (!imms) & 0x3F;
            if not_imms == 0 {
                return 0;
            }
            (not_imms.leading_zeros() - 26) as u32
        };

        if len < 1 {
            return 0;
        }

        let size = 1u32 << len;
        let levels = size - 1;
        let s = imms & levels;
        let r = immr & levels;

        if s == levels {
            return 0;
        }

        // Create base pattern
        let welem = (1u64 << (s + 1)) - 1;

        // Rotate right
        let elem = if r == 0 {
            welem
        } else {
            let shift = r as u32;
            ((welem >> shift) | (welem << (size - shift))) & ((1u64 << size) - 1)
        };

        // Replicate to register size
        let mut result = 0u64;
        let mut pos = 0;
        while pos < reg_size {
            result |= elem << pos;
            pos += size;
        }

        if reg_size == 32 {
            result & 0xFFFF_FFFF
        } else {
            result
        }
    }

    // ========================================================================
    // Load/Store and Branch Evaluation Methods
    // ========================================================================

    /// Compute expected values for LDR unsigned immediate
    /// Encoding: size[31:30] | 111 | 0 | 01 | opc[23:22] | imm12[21:10] | Rn[9:5] | Rt[4:0]
    pub fn compute_ldr_unsigned_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
        memory_value: u64,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        let size = ((encoding >> 30) & 3) as u8;
        let opc = (encoding >> 22) & 3;
        let imm12 = ((encoding >> 10) & 0xFFF) as u64;
        let rn = ((encoding >> 5) & 0x1F) as u8;
        let rt = (encoding & 0x1F) as u8;

        // Scale = 2^size
        let scale = 1u64 << size;
        let offset = imm12 * scale;
        let base = *initial_state.gp_regs.get(&rn).unwrap_or(&0);
        let _address = base.wrapping_add(offset);

        // Determine sign extension based on opc
        let is_signed = opc >= 2;
        let extend_to_64 = opc == 2;

        // For simplicity, use the provided memory value directly
        let result = if is_signed {
            // Sign extend
            match size {
                0 => {
                    if extend_to_64 {
                        (memory_value as i8) as i64 as u64
                    } else {
                        ((memory_value as i8) as i32 as u32) as u64
                    }
                }
                1 => {
                    if extend_to_64 {
                        (memory_value as i16) as i64 as u64
                    } else {
                        ((memory_value as i16) as i32 as u32) as u64
                    }
                }
                2 => (memory_value as i32) as i64 as u64,
                3 => memory_value,
                _ => memory_value,
            }
        } else {
            // Zero extend - just mask to size
            match size {
                0 => memory_value & 0xFF,
                1 => memory_value & 0xFFFF,
                2 => memory_value & 0xFFFFFFFF,
                3 => memory_value,
                _ => memory_value,
            }
        };

        let mut expected_state = initial_state.clone();
        if rt != 31 {
            // X31/WZR is zero register
            expected_state.gp_regs.insert(rt, result);
        }

        let is_64bit = size == 3 || (is_signed && extend_to_64);
        let assertions = if rt != 31 {
            if is_64bit {
                vec![TestAssertion {
                    check: AssertionCheck::GpReg(rt),
                    expected: AssertionValue::U64(result),
                    message: format!("X{} should be 0x{:016X}", rt, result),
                }]
            } else {
                vec![TestAssertion {
                    check: AssertionCheck::GpReg32(rt),
                    expected: AssertionValue::U32(result as u32),
                    message: format!("W{} should be 0x{:08X}", rt, result as u32),
                }]
            }
        } else {
            vec![]
        };

        (expected_state, assertions)
    }

    /// Compute expected values for STR unsigned immediate
    pub fn compute_str_unsigned_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>, u64, u64) {
        let size = ((encoding >> 30) & 3) as u8;
        let imm12 = ((encoding >> 10) & 0xFFF) as u64;
        let rn = ((encoding >> 5) & 0x1F) as u8;
        let rt = (encoding & 0x1F) as u8;

        let scale = 1u64 << size;
        let offset = imm12 * scale;
        let base = *initial_state.gp_regs.get(&rn).unwrap_or(&0);
        let address = base.wrapping_add(offset);
        let value = *initial_state.gp_regs.get(&rt).unwrap_or(&0);

        // Mask value to size
        let store_value = match size {
            0 => value & 0xFF,
            1 => value & 0xFFFF,
            2 => value & 0xFFFFFFFF,
            3 => value,
            _ => value,
        };

        let expected_state = initial_state.clone();
        let assertions = vec![TestAssertion {
            check: AssertionCheck::Memory {
                address,
                size: 1 << size,
            },
            expected: AssertionValue::U64(store_value),
            message: format!("Memory at 0x{:X} should be 0x{:X}", address, store_value),
        }];

        (expected_state, assertions, address, store_value)
    }

    /// Compute expected PC after branch
    pub fn compute_branch_target(&self, encoding: u64, pc: u64) -> u64 {
        let imm26 = encoding & 0x3FF_FFFF;
        // Sign extend 26-bit immediate
        let offset = if (imm26 >> 25) != 0 {
            (imm26 | 0xFFFF_FFFF_FC00_0000) as i64
        } else {
            imm26 as i64
        };
        // Offset is in instructions (4 bytes each)
        pc.wrapping_add((offset * 4) as u64)
    }

    /// Compute expected PC after CBZ/CBNZ
    pub fn compute_compare_branch_target(
        &self,
        encoding: u64,
        pc: u64,
        reg_value: u64,
        is_nonzero: bool,
    ) -> u64 {
        let sf = (encoding >> 31) & 1;
        let imm19 = (encoding >> 5) & 0x7_FFFF;
        let width = if sf == 1 { 64 } else { 32 };

        let test_value = if width == 32 {
            reg_value & 0xFFFF_FFFF
        } else {
            reg_value
        };

        let condition_met = if is_nonzero {
            test_value != 0
        } else {
            test_value == 0
        };

        if condition_met {
            // Sign extend 19-bit immediate
            let offset = if (imm19 >> 18) != 0 {
                (imm19 | 0xFFFF_FFFF_FFF8_0000) as i64
            } else {
                imm19 as i64
            };
            pc.wrapping_add((offset * 4) as u64)
        } else {
            pc.wrapping_add(4) // Fall through
        }
    }

    // ========================================================================
    // A32/T32/T16 Evaluation Methods
    // ========================================================================

    /// Expand A32 modified immediate constant (imm12 = rotate_imm:imm8)
    /// Returns the 32-bit immediate value
    pub fn expand_a32_imm12(&self, imm12: u32) -> u32 {
        let imm8 = imm12 & 0xFF;
        let rotate_imm = (imm12 >> 8) & 0xF;
        let rotation = rotate_imm * 2;
        if rotation == 0 {
            imm8
        } else {
            imm8.rotate_right(rotation)
        }
    }

    /// Compute expected values for A32 ADD/SUB immediate
    /// Encoding: cond[31:28] | 00 | 1 | opcode[24:21] | S[20] | Rn[19:16] | Rd[15:12] | imm12[11:0]
    pub fn compute_a32_add_sub_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        let encoding = encoding as u32;
        let opcode = (encoding >> 21) & 0xF;
        let s_bit = (encoding >> 20) & 1;
        let rn = ((encoding >> 16) & 0xF) as u8;
        let rd = ((encoding >> 12) & 0xF) as u8;
        let imm12 = encoding & 0xFFF;

        let imm = self.expand_a32_imm12(imm12);
        let operand1 = *initial_state.gp_regs.get(&rn).unwrap_or(&0) as u32;

        let (is_sub, is_rsb) = match opcode {
            0b0010 => (true, false),  // SUB
            0b0011 => (true, true),   // RSB (reverse subtract)
            0b0100 => (false, false), // ADD
            0b0101 => (false, false), // ADC (add with carry - simplified)
            0b0110 => (true, false),  // SBC (subtract with carry - simplified)
            0b0111 => (true, true),   // RSC (reverse subtract with carry - simplified)
            _ => (false, false),
        };

        let (result, carry, overflow) = if is_rsb {
            // RSB: Rd = imm - Rn
            let (res, c) = imm.overflowing_sub(operand1);
            let v = ((imm as i32).overflowing_sub(operand1 as i32)).1;
            (res, !c, v) // carry is inverted for ARM subtract
        } else if is_sub {
            // SUB: Rd = Rn - imm
            let (res, c) = operand1.overflowing_sub(imm);
            let v = ((operand1 as i32).overflowing_sub(imm as i32)).1;
            (res, !c, v) // carry is inverted for ARM subtract
        } else {
            // ADD: Rd = Rn + imm
            let (res, c) = operand1.overflowing_add(imm);
            let v = ((operand1 as i32).overflowing_add(imm as i32)).1;
            (res, c, v)
        };

        let mut expected_state = initial_state.clone();
        expected_state.gp_regs.insert(rd, result as u64);

        let mut assertions = vec![TestAssertion {
            check: AssertionCheck::GpReg32(rd),
            expected: AssertionValue::U32(result),
            message: format!("R{} should be 0x{:08X}", rd, result),
        }];

        if s_bit == 1 && rd != 15 {
            let n = (result >> 31) != 0;
            let z = result == 0;
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(n),
                message: format!("N flag should be {}", n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(z),
                message: format!("Z flag should be {}", z),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::C),
                expected: AssertionValue::Bool(carry),
                message: format!("C flag should be {}", carry),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::V),
                expected: AssertionValue::Bool(overflow),
                message: format!("V flag should be {}", overflow),
            });
        }

        (expected_state, assertions)
    }

    /// Compute expected values for A32 ADD/SUB register
    pub fn compute_a32_add_sub_reg_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        let encoding = encoding as u32;
        let opcode = (encoding >> 21) & 0xF;
        let s_bit = (encoding >> 20) & 1;
        let rn = ((encoding >> 16) & 0xF) as u8;
        let rd = ((encoding >> 12) & 0xF) as u8;
        let rm = (encoding & 0xF) as u8;
        let shift_type = (encoding >> 5) & 0x3;
        let shift_amount = (encoding >> 7) & 0x1F;

        let operand1 = *initial_state.gp_regs.get(&rn).unwrap_or(&0) as u32;
        let rm_val = *initial_state.gp_regs.get(&rm).unwrap_or(&0) as u32;

        // Apply shift to Rm
        let operand2 = match shift_type {
            0b00 => rm_val << shift_amount, // LSL
            0b01 => {
                // LSR
                if shift_amount == 0 {
                    0
                } else {
                    rm_val >> shift_amount
                }
            }
            0b10 => {
                // ASR
                if shift_amount == 0 {
                    if (rm_val as i32) < 0 {
                        0xFFFFFFFF
                    } else {
                        0
                    }
                } else {
                    ((rm_val as i32) >> shift_amount) as u32
                }
            }
            0b11 => {
                // ROR
                if shift_amount == 0 {
                    rm_val // RRX not handled
                } else {
                    rm_val.rotate_right(shift_amount)
                }
            }
            _ => rm_val,
        };

        let is_sub = match opcode {
            0b0010 | 0b0110 => true, // SUB, SBC
            _ => false,
        };

        let (result, carry, overflow) = if is_sub {
            let (res, c) = operand1.overflowing_sub(operand2);
            let v = ((operand1 as i32).overflowing_sub(operand2 as i32)).1;
            (res, !c, v)
        } else {
            let (res, c) = operand1.overflowing_add(operand2);
            let v = ((operand1 as i32).overflowing_add(operand2 as i32)).1;
            (res, c, v)
        };

        let mut expected_state = initial_state.clone();
        expected_state.gp_regs.insert(rd, result as u64);

        let mut assertions = vec![TestAssertion {
            check: AssertionCheck::GpReg32(rd),
            expected: AssertionValue::U32(result),
            message: format!("R{} should be 0x{:08X}", rd, result),
        }];

        if s_bit == 1 && rd != 15 {
            let n = (result >> 31) != 0;
            let z = result == 0;
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(n),
                message: format!("N flag should be {}", n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(z),
                message: format!("Z flag should be {}", z),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::C),
                expected: AssertionValue::Bool(carry),
                message: format!("C flag should be {}", carry),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::V),
                expected: AssertionValue::Bool(overflow),
                message: format!("V flag should be {}", overflow),
            });
        }

        (expected_state, assertions)
    }

    /// Compute expected values for A32 logical immediate (AND, ORR, EOR, BIC)
    pub fn compute_a32_logical_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        let encoding = encoding as u32;
        let opcode = (encoding >> 21) & 0xF;
        let s_bit = (encoding >> 20) & 1;
        let rn = ((encoding >> 16) & 0xF) as u8;
        let rd = ((encoding >> 12) & 0xF) as u8;
        let imm12 = encoding & 0xFFF;

        let imm = self.expand_a32_imm12(imm12);
        let operand1 = *initial_state.gp_regs.get(&rn).unwrap_or(&0) as u32;

        let result = match opcode {
            0b0000 => operand1 & imm,  // AND
            0b0001 => operand1 ^ imm,  // EOR
            0b1100 => operand1 | imm,  // ORR
            0b1110 => operand1 & !imm, // BIC
            0b1111 => !imm,            // MVN
            _ => operand1,
        };

        let mut expected_state = initial_state.clone();
        expected_state.gp_regs.insert(rd, result as u64);

        let mut assertions = vec![TestAssertion {
            check: AssertionCheck::GpReg32(rd),
            expected: AssertionValue::U32(result),
            message: format!("R{} should be 0x{:08X}", rd, result),
        }];

        if s_bit == 1 && rd != 15 {
            let n = (result >> 31) != 0;
            let z = result == 0;
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(n),
                message: format!("N flag should be {}", n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(z),
                message: format!("Z flag should be {}", z),
            });
            // C flag is set from shifter carry-out for logical ops
            // V flag is unaffected
        }

        (expected_state, assertions)
    }

    /// Compute expected values for A32 MOV immediate
    pub fn compute_a32_mov_imm_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        let encoding = encoding as u32;
        let opcode = (encoding >> 21) & 0xF;
        let s_bit = (encoding >> 20) & 1;
        let rd = ((encoding >> 12) & 0xF) as u8;
        let imm12 = encoding & 0xFFF;

        let imm = self.expand_a32_imm12(imm12);
        let result = if opcode == 0b1111 { !imm } else { imm }; // MVN vs MOV

        let mut expected_state = initial_state.clone();
        expected_state.gp_regs.insert(rd, result as u64);

        let mut assertions = vec![TestAssertion {
            check: AssertionCheck::GpReg32(rd),
            expected: AssertionValue::U32(result),
            message: format!("R{} should be 0x{:08X}", rd, result),
        }];

        if s_bit == 1 && rd != 15 {
            let n = (result >> 31) != 0;
            let z = result == 0;
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(n),
                message: format!("N flag should be {}", n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(z),
                message: format!("Z flag should be {}", z),
            });
        }

        (expected_state, assertions)
    }

    /// Compute expected values for A32 MUL/MLA
    pub fn compute_a32_multiply_expected(
        &self,
        encoding: u64,
        initial_state: &ProcessorState,
    ) -> (ProcessorState, Vec<TestAssertion>) {
        let encoding = encoding as u32;
        let s_bit = (encoding >> 20) & 1;
        let rd = ((encoding >> 16) & 0xF) as u8;
        let ra = ((encoding >> 12) & 0xF) as u8;
        let rm = ((encoding >> 8) & 0xF) as u8;
        let rn = (encoding & 0xF) as u8;
        let accumulate = (encoding >> 21) & 1;

        let rn_val = *initial_state.gp_regs.get(&rn).unwrap_or(&0) as u32;
        let rm_val = *initial_state.gp_regs.get(&rm).unwrap_or(&0) as u32;
        let ra_val = if accumulate == 1 {
            *initial_state.gp_regs.get(&ra).unwrap_or(&0) as u32
        } else {
            0
        };

        let result = rn_val.wrapping_mul(rm_val).wrapping_add(ra_val);

        let mut expected_state = initial_state.clone();
        expected_state.gp_regs.insert(rd, result as u64);

        let mut assertions = vec![TestAssertion {
            check: AssertionCheck::GpReg32(rd),
            expected: AssertionValue::U32(result),
            message: format!("R{} should be 0x{:08X}", rd, result),
        }];

        if s_bit == 1 {
            let n = (result >> 31) != 0;
            let z = result == 0;
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::N),
                expected: AssertionValue::Bool(n),
                message: format!("N flag should be {}", n),
            });
            assertions.push(TestAssertion {
                check: AssertionCheck::Flag(ProcessorFlag::Z),
                expected: AssertionValue::Bool(z),
                message: format!("Z flag should be {}", z),
            });
        }

        (expected_state, assertions)
    }
}

impl Default for TestEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Pattern matching for instruction types based on encoding
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionPattern {
    /// ADD/ADDS immediate
    AddSubImmediate { is_sub: bool, set_flags: bool },
    /// ADD/ADDS shifted register
    AddSubShiftedReg { is_sub: bool, set_flags: bool },
    /// ADD/ADDS extended register
    AddSubExtendedReg { is_sub: bool, set_flags: bool },
    /// Logical immediate (AND, ORR, EOR, ANDS)
    LogicalImmediate { op: LogicalOp },
    /// Logical shifted register
    LogicalShiftedReg { op: LogicalOp },
    /// Move immediate (MOVZ, MOVN, MOVK)
    MoveImmediate { op: MoveOp },
    /// Shift variable
    ShiftVariable { shift_type: ShiftType },
    /// Multiply (MUL, MADD, MSUB)
    Multiply { accumulate: bool, negate: bool },
    /// Multiply long (SMULL, UMULL, SMADDL, etc.)
    MultiplyLong {
        signed: bool,
        accumulate: bool,
        negate: bool,
    },
    /// Multiply high (SMULH, UMULH)
    MultiplyHigh { signed: bool },
    /// Divide (SDIV, UDIV)
    Divide { signed: bool },
    /// Bitfield (SBFM, UBFM, BFM)
    Bitfield { op: BitfieldOp },
    /// Extract (EXTR)
    Extract,
    /// Bit operations (CLZ, CLS, RBIT, REV, REV16, REV32)
    BitOp { op: BitOpType },
    /// Conditional select (CSEL, CSINC, CSINV, CSNEG)
    ConditionalSelect { op: CondSelectOp },
    /// Compare (CMP, CMN - aliases of SUBS, ADDS with ZR dest)
    Compare { is_neg: bool },
    /// Test (TST - alias of ANDS with ZR dest)
    Test,
    /// Address calculation (ADR, ADRP)
    PcRelAddress { is_page: bool },
    /// Bit manipulation
    BitManipulation,
    /// Load register (LDR, LDUR, LDRB, LDRH, LDRSB, LDRSH, LDRSW)
    Load {
        size: u8, // 0=byte, 1=halfword, 2=word, 3=doubleword
        signed: bool,
        addressing: LoadStoreAddressing,
    },
    /// Store register (STR, STUR, STRB, STRH)
    Store {
        size: u8, // 0=byte, 1=halfword, 2=word, 3=doubleword
        addressing: LoadStoreAddressing,
    },
    /// Load pair (LDP, LDNP, LDPSW)
    LoadPair { is_signed: bool },
    /// Store pair (STP, STNP)
    StorePair,
    /// Branch (B, BL)
    Branch { link: bool },
    /// Branch to register (BR, BLR, RET)
    BranchReg { link: bool, is_ret: bool },
    /// Compare and branch (CBZ, CBNZ)
    CompareBranch { nonzero: bool },
    /// Test and branch (TBZ, TBNZ)
    TestBranch { nonzero: bool },
    /// Unknown pattern
    Unknown,
}

/// Load/Store addressing mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadStoreAddressing {
    /// Unsigned immediate offset: [Xn, #imm]
    UnsignedImm,
    /// Signed immediate with pre-index: [Xn, #imm]!
    PreIndex,
    /// Signed immediate with post-index: [Xn], #imm
    PostIndex,
    /// Register offset: [Xn, Rm, extend]
    Register,
    /// Unscaled immediate: [Xn, #simm9]
    Unscaled,
    /// Literal (PC-relative): label
    Literal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitfieldOp {
    Sbfm, // Signed bitfield move
    Ubfm, // Unsigned bitfield move
    Bfm,  // Bitfield move
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BitOpType {
    Clz,   // Count leading zeros
    Cls,   // Count leading sign bits
    Rbit,  // Reverse bits
    Rev,   // Reverse bytes (full register)
    Rev16, // Reverse bytes in halfwords
    Rev32, // Reverse bytes in words
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CondSelectOp {
    Csel,  // Conditional select
    Csinc, // Conditional select increment
    Csinv, // Conditional select invert
    Csneg, // Conditional select negate
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
    Xor,
    Ands,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveOp {
    Movz,
    Movn,
    Movk,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftType {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

/// Identify instruction pattern from encoding (for A64)
pub fn identify_a64_pattern(encoding: u64) -> InstructionPattern {
    // Check specific instruction classes based on opcode bits

    // Check for ADD/SUB immediate (bits 28-24 = 10001)
    // BUT also verify shift field (bits 23:22) is valid: 00 or 01
    // Values 10/11 indicate ADDG/SUBG (memory tagging) which we don't handle
    let op_28_24 = (encoding >> 24) & 0x1F;
    let shift = (encoding >> 22) & 0x3;
    let rd = encoding & 0x1F;
    if op_28_24 == 0b10001 && shift <= 1 {
        // Add/subtract immediate (shift = 0 or 1 means no shift or LSL #12)
        let op = (encoding >> 30) & 1;
        let s = (encoding >> 29) & 1;
        // Check for CMP/CMN (SUBS/ADDS with Rd=31 i.e. zero register)
        if s == 1 && rd == 31 {
            return InstructionPattern::Compare { is_neg: op == 0 }; // CMN is op=0 (ADD), CMP is op=1 (SUB)
        }
        return InstructionPattern::AddSubImmediate {
            is_sub: op == 1,
            set_flags: s == 1,
        };
    }

    // Check for Logical immediate (bits 28-23 = 100100)
    let op_28_23 = (encoding >> 23) & 0x3F;
    if op_28_23 == 0b100100 {
        let opc = (encoding >> 29) & 3;
        // Check for TST (ANDS with Rd=31 i.e. zero register)
        if opc == 0b11 && rd == 31 {
            return InstructionPattern::Test;
        }
        return InstructionPattern::LogicalImmediate {
            op: match opc {
                0b00 => LogicalOp::And,
                0b01 => LogicalOp::Or,
                0b10 => LogicalOp::Xor,
                0b11 => LogicalOp::Ands,
                _ => unreachable!(),
            },
        };
    }

    // Check for Move wide immediate (bits 28-23 = 100101)
    if op_28_23 == 0b100101 {
        let opc = (encoding >> 29) & 3;
        return match opc {
            0b00 => InstructionPattern::MoveImmediate { op: MoveOp::Movn },
            0b10 => InstructionPattern::MoveImmediate { op: MoveOp::Movz },
            0b11 => InstructionPattern::MoveImmediate { op: MoveOp::Movk },
            _ => InstructionPattern::Unknown,
        };
    }

    // Check for Bitfield (bits 28-23 = 100110)
    if op_28_23 == 0b100110 {
        let opc = (encoding >> 29) & 3;
        return match opc {
            0b00 => InstructionPattern::Bitfield {
                op: BitfieldOp::Sbfm,
            },
            0b01 => InstructionPattern::Bitfield {
                op: BitfieldOp::Bfm,
            },
            0b10 => InstructionPattern::Bitfield {
                op: BitfieldOp::Ubfm,
            },
            _ => InstructionPattern::Unknown,
        };
    }

    // Check for Extract (bits 28-23 = 100111)
    if op_28_23 == 0b100111 {
        return InstructionPattern::Extract;
    }

    // Check for PC-relative addressing (bits 28-24 = 10000)
    if op_28_24 == 0b10000 {
        let op = (encoding >> 31) & 1;
        return InstructionPattern::PcRelAddress { is_page: op == 1 };
    }

    let op0 = (encoding >> 25) & 0xF;

    match op0 {
        // Data processing - register (ADD/SUB shifted, logical shifted, etc.)
        // NOTE: Only 0b0101 (bit28=0). 0b1101 (bit28=1) is other DP
        0b0101 => {
            let op1 = (encoding >> 21) & 0xF;
            let op2 = (encoding >> 10) & 0x3F;

            if op1 == 0b0000 {
                // Logical shifted register
                let opc = (encoding >> 29) & 3;
                // Check for TST (ANDS shifted reg with Rd=31)
                if opc == 0b11 && rd == 31 {
                    return InstructionPattern::Test;
                }
                InstructionPattern::LogicalShiftedReg {
                    op: match opc {
                        0b00 => LogicalOp::And,
                        0b01 => LogicalOp::Or,
                        0b10 => LogicalOp::Xor,
                        0b11 => LogicalOp::Ands,
                        _ => unreachable!(),
                    },
                }
            } else if op1 == 0b1000 {
                // Add/subtract shifted register
                let op = (encoding >> 30) & 1;
                let s = (encoding >> 29) & 1;
                // Check for CMP/CMN (SUBS/ADDS shifted reg with Rd=31)
                if s == 1 && rd == 31 {
                    return InstructionPattern::Compare { is_neg: op == 0 };
                }
                InstructionPattern::AddSubShiftedReg {
                    is_sub: op == 1,
                    set_flags: s == 1,
                }
            } else if op1 == 0b1001 {
                // Add/subtract extended register
                let op = (encoding >> 30) & 1;
                let s = (encoding >> 29) & 1;
                // Check for CMP/CMN (SUBS/ADDS extended reg with Rd=31)
                if s == 1 && rd == 31 {
                    return InstructionPattern::Compare { is_neg: op == 0 };
                }
                InstructionPattern::AddSubExtendedReg {
                    is_sub: op == 1,
                    set_flags: s == 1,
                }
            } else if op1 == 0b0110 {
                // Data processing 2-source
                let opcode2 = (encoding >> 10) & 0x3F;
                if opcode2 == 0b000010 {
                    // UDIV
                    return InstructionPattern::Divide { signed: false };
                } else if opcode2 == 0b000011 {
                    // SDIV
                    return InstructionPattern::Divide { signed: true };
                } else if opcode2 >> 4 == 0b10 {
                    // Variable shift
                    let shift_type = (encoding >> 10) & 3;
                    return InstructionPattern::ShiftVariable {
                        shift_type: match shift_type {
                            0 => ShiftType::Lsl,
                            1 => ShiftType::Lsr,
                            2 => ShiftType::Asr,
                            3 => ShiftType::Ror,
                            _ => unreachable!(),
                        },
                    };
                }
                InstructionPattern::Unknown
            } else {
                InstructionPattern::Unknown
            }
        }
        // Data processing - register (3-source: MADD, MSUB, etc.)
        0b1101 => {
            let op54 = (encoding >> 29) & 3;
            let op31 = (encoding >> 21) & 7;
            let o0 = (encoding >> 15) & 1;

            if op54 == 0b00 {
                if op31 == 0b000 {
                    // MADD/MSUB
                    return InstructionPattern::Multiply {
                        accumulate: true,
                        negate: o0 == 1,
                    };
                } else if op31 == 0b001 {
                    // SMADDL/SMSUBL
                    return InstructionPattern::MultiplyLong {
                        signed: true,
                        accumulate: true,
                        negate: o0 == 1,
                    };
                } else if op31 == 0b010 && o0 == 0 {
                    // SMULH
                    return InstructionPattern::MultiplyHigh { signed: true };
                } else if op31 == 0b101 {
                    // UMADDL/UMSUBL
                    return InstructionPattern::MultiplyLong {
                        signed: false,
                        accumulate: true,
                        negate: o0 == 1,
                    };
                } else if op31 == 0b110 && o0 == 0 {
                    // UMULH
                    return InstructionPattern::MultiplyHigh { signed: false };
                }
            }

            // Conditional select
            if op54 == 0b00 || op54 == 0b01 {
                let op_29 = (encoding >> 29) & 1;
                let op2 = (encoding >> 10) & 3;

                if (encoding >> 21) & 0xF == 0b0100 {
                    // Conditional select
                    return match (op_29, op2) {
                        (0, 0b00) => InstructionPattern::ConditionalSelect {
                            op: CondSelectOp::Csel,
                        },
                        (0, 0b01) => InstructionPattern::ConditionalSelect {
                            op: CondSelectOp::Csinc,
                        },
                        (1, 0b00) => InstructionPattern::ConditionalSelect {
                            op: CondSelectOp::Csinv,
                        },
                        (1, 0b01) => InstructionPattern::ConditionalSelect {
                            op: CondSelectOp::Csneg,
                        },
                        _ => InstructionPattern::Unknown,
                    };
                }
            }

            InstructionPattern::Unknown
        }
        // Data processing - 1 source (CLZ, CLS, RBIT, REV, etc.)
        // Encoding: sf 1 S 11010110 00000 opcode Rn Rd
        // Need to verify bits 29:21 = 1_11010110 (0x1D6) to distinguish from other op0=0b1010 instructions
        0b1010 | 0b0110 => {
            let sf = (encoding >> 31) & 1;
            let s = (encoding >> 29) & 1;
            let opcode2 = (encoding >> 16) & 0x1F;
            let opcode = (encoding >> 10) & 0x3F;
            // Check bits 29:21 to verify this is actually data processing 1-source
            // Pattern: bit29=1, bits28:21=11010110 (0xD6)
            let bits_29_21 = (encoding >> 21) & 0x1FF;
            let dp1_pattern = 0b1_11010110; // 0x1D6

            if bits_29_21 == dp1_pattern && s == 0 && opcode2 == 0 {
                match opcode {
                    0b000000 => InstructionPattern::BitOp {
                        op: BitOpType::Rbit,
                    },
                    0b000001 => InstructionPattern::BitOp {
                        op: BitOpType::Rev16,
                    },
                    0b000010 if sf == 1 => InstructionPattern::BitOp {
                        op: BitOpType::Rev32,
                    },
                    0b000010 if sf == 0 => InstructionPattern::BitOp { op: BitOpType::Rev },
                    0b000011 if sf == 1 => InstructionPattern::BitOp { op: BitOpType::Rev },
                    0b000100 => InstructionPattern::BitOp { op: BitOpType::Clz },
                    0b000101 => InstructionPattern::BitOp { op: BitOpType::Cls },
                    _ => InstructionPattern::Unknown,
                }
            } else {
                InstructionPattern::Unknown
            }
        }
        // Load/store register unsigned immediate: size[31:30] | 111 | V[26] | 01 | opc[23:22] | imm12[21:10] | Rn[9:5] | Rt[4:0]
        0b0100 | 0b1100 => {
            let size = (encoding >> 30) & 3;
            let v = (encoding >> 26) & 1;
            let opc = (encoding >> 22) & 3;
            let op_bits = (encoding >> 24) & 0x3;

            if v == 0 && op_bits == 0b01 {
                // Regular load/store unsigned immediate
                match opc {
                    0b00 => InstructionPattern::Store {
                        size: size as u8,
                        addressing: LoadStoreAddressing::UnsignedImm,
                    },
                    0b01 => InstructionPattern::Load {
                        size: size as u8,
                        signed: false,
                        addressing: LoadStoreAddressing::UnsignedImm,
                    },
                    0b10 => InstructionPattern::Load {
                        size: size as u8,
                        signed: true,
                        addressing: LoadStoreAddressing::UnsignedImm,
                    },
                    0b11 if size < 3 => InstructionPattern::Load {
                        size: size as u8,
                        signed: true,
                        addressing: LoadStoreAddressing::UnsignedImm,
                    },
                    _ => InstructionPattern::Unknown,
                }
            } else if v == 0 && op_bits == 0b00 {
                // Load/store unscaled, post-index, pre-index
                let op2 = (encoding >> 10) & 3;
                let addressing = match op2 {
                    0b00 => LoadStoreAddressing::Unscaled,
                    0b01 => LoadStoreAddressing::PostIndex,
                    0b11 => LoadStoreAddressing::PreIndex,
                    _ => LoadStoreAddressing::Unscaled,
                };
                match opc {
                    0b00 => InstructionPattern::Store {
                        size: size as u8,
                        addressing,
                    },
                    0b01 => InstructionPattern::Load {
                        size: size as u8,
                        signed: false,
                        addressing,
                    },
                    0b10 | 0b11 => InstructionPattern::Load {
                        size: size as u8,
                        signed: true,
                        addressing,
                    },
                    _ => InstructionPattern::Unknown,
                }
            } else {
                InstructionPattern::Unknown
            }
        }
        // Load/store register (register offset): size[31:30] | 111 | V[26] | 00 | opc[23:22] | 1 | Rm[20:16] | option[15:13] | S[12] | 10 | Rn[9:5] | Rt[4:0]
        0b0110 => {
            let size = (encoding >> 30) & 3;
            let v = (encoding >> 26) & 1;
            let opc = (encoding >> 22) & 3;
            let op2 = (encoding >> 10) & 3;

            if v == 0 && op2 == 0b10 {
                // Register offset
                match opc {
                    0b00 => InstructionPattern::Store {
                        size: size as u8,
                        addressing: LoadStoreAddressing::Register,
                    },
                    0b01 => InstructionPattern::Load {
                        size: size as u8,
                        signed: false,
                        addressing: LoadStoreAddressing::Register,
                    },
                    0b10 | 0b11 => InstructionPattern::Load {
                        size: size as u8,
                        signed: true,
                        addressing: LoadStoreAddressing::Register,
                    },
                    _ => InstructionPattern::Unknown,
                }
            } else {
                InstructionPattern::Unknown
            }
        }
        // Load/store pair: opc[31:30] | 101 | V[26] | 0 | L[22] | imm7[21:15] | Rt2[14:10] | Rn[9:5] | Rt[4:0]
        0b0010 | 0b1010 => {
            let opc = (encoding >> 30) & 3;
            let v = (encoding >> 26) & 1;
            let l = (encoding >> 22) & 1;
            let is_post = (encoding >> 23) & 1;
            let is_pre = (encoding >> 24) & 1;

            if v == 0 {
                if l == 1 {
                    InstructionPattern::LoadPair {
                        is_signed: opc == 0b01,
                    }
                } else {
                    InstructionPattern::StorePair
                }
            } else {
                InstructionPattern::Unknown
            }
        }
        // Branch immediate: 0 | op[26] | 00101 | imm26[25:0]
        0b0101 => {
            let op = (encoding >> 31) & 1;
            InstructionPattern::Branch { link: op == 1 }
        }
        // Compare and branch: sf[31] | 011010 | op[24] | imm19[23:5] | Rt[4:0]
        0b1010 => {
            let op = (encoding >> 24) & 1;
            let op_high = (encoding >> 25) & 0x7F;
            if op_high == 0b0110101 || op_high == 0b0110100 {
                InstructionPattern::CompareBranch { nonzero: op == 1 }
            } else {
                InstructionPattern::Unknown
            }
        }
        // Test and branch: b5[31] | 011011 | op[24] | b40[23:19] | imm14[18:5] | Rt[4:0]
        0b1011 => {
            let op = (encoding >> 24) & 1;
            InstructionPattern::TestBranch { nonzero: op == 1 }
        }
        _ => InstructionPattern::Unknown,
    }
}

/// Identify instruction pattern from encoding (for A32)
pub fn identify_a32_pattern(encoding: u64) -> InstructionPattern {
    let cond = (encoding >> 28) & 0xF;
    let op1 = (encoding >> 25) & 0x7;
    let op = (encoding >> 20) & 0x1F;

    match op1 {
        // Data processing immediate
        0b001 => {
            let opcode = (encoding >> 21) & 0xF;
            match opcode {
                0b0000 => InstructionPattern::LogicalImmediate { op: LogicalOp::And },
                0b0001 => InstructionPattern::LogicalImmediate { op: LogicalOp::Xor },
                0b0010 => InstructionPattern::AddSubImmediate {
                    is_sub: true,
                    set_flags: false,
                },
                0b0011 => InstructionPattern::AddSubImmediate {
                    is_sub: true,
                    set_flags: false,
                }, // RSB
                0b0100 => InstructionPattern::AddSubImmediate {
                    is_sub: false,
                    set_flags: false,
                },
                0b0101 => InstructionPattern::AddSubImmediate {
                    is_sub: false,
                    set_flags: false,
                }, // ADC
                0b0110 => InstructionPattern::AddSubImmediate {
                    is_sub: true,
                    set_flags: false,
                }, // SBC
                0b0111 => InstructionPattern::AddSubImmediate {
                    is_sub: true,
                    set_flags: false,
                }, // RSC
                0b1000 => InstructionPattern::Test, // TST
                0b1001 => InstructionPattern::Test, // TEQ
                0b1010 => InstructionPattern::Compare { is_neg: false }, // CMP
                0b1011 => InstructionPattern::Compare { is_neg: true }, // CMN
                0b1100 => InstructionPattern::LogicalImmediate { op: LogicalOp::Or },
                0b1101 => InstructionPattern::MoveImmediate { op: MoveOp::Movz },
                0b1110 => InstructionPattern::LogicalImmediate { op: LogicalOp::And }, // BIC
                0b1111 => InstructionPattern::MoveImmediate { op: MoveOp::Movn },      // MVN
                _ => InstructionPattern::Unknown,
            }
        }
        // Data processing register
        0b000 => {
            let opcode = (encoding >> 21) & 0xF;
            let bit4 = (encoding >> 4) & 1;
            let bit7 = (encoding >> 7) & 1;

            if bit4 == 1 && bit7 == 1 {
                // Multiply / multiply-accumulate
                let op_23_20 = (encoding >> 20) & 0xF;
                match op_23_20 {
                    0b0000 | 0b0001 => InstructionPattern::Multiply {
                        accumulate: false,
                        negate: false,
                    },
                    0b0010 | 0b0011 => InstructionPattern::Multiply {
                        accumulate: true,
                        negate: false,
                    },
                    0b0100..=0b0111 => InstructionPattern::MultiplyLong {
                        signed: false,
                        accumulate: true,
                        negate: false,
                    },
                    0b1100..=0b1111 => InstructionPattern::MultiplyLong {
                        signed: true,
                        accumulate: true,
                        negate: false,
                    },
                    _ => InstructionPattern::Unknown,
                }
            } else {
                // Regular data processing
                match opcode {
                    0b0000 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::And },
                    0b0001 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::Xor },
                    0b0010 => InstructionPattern::AddSubShiftedReg {
                        is_sub: true,
                        set_flags: false,
                    },
                    0b0100 => InstructionPattern::AddSubShiftedReg {
                        is_sub: false,
                        set_flags: false,
                    },
                    0b1100 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::Or },
                    0b1101 => InstructionPattern::ShiftVariable {
                        shift_type: ShiftType::Lsl,
                    }, // MOV with shift
                    _ => InstructionPattern::Unknown,
                }
            }
        }
        _ => InstructionPattern::Unknown,
    }
}

/// Identify instruction pattern from encoding (for T32/T16)
pub fn identify_thumb_pattern(encoding: u64, is_32bit: bool) -> InstructionPattern {
    if is_32bit {
        identify_t32_pattern(encoding)
    } else {
        identify_t16_pattern(encoding)
    }
}

fn identify_t32_pattern(encoding: u64) -> InstructionPattern {
    let op1 = (encoding >> 27) & 0x3;
    let op2 = (encoding >> 20) & 0x7F;
    let op = (encoding >> 15) & 1;

    match op1 {
        0b10 => {
            // Data processing (modified immediate) or (plain binary immediate)
            if op == 0 {
                let op_24_21 = (encoding >> 21) & 0xF;
                let rn = (encoding >> 16) & 0xF;
                let rd = (encoding >> 8) & 0xF;

                match op_24_21 {
                    0b0000 if rd == 0xF => InstructionPattern::Test,
                    0b0000 => InstructionPattern::LogicalImmediate { op: LogicalOp::And },
                    0b0001 => InstructionPattern::LogicalImmediate { op: LogicalOp::And }, // BIC
                    0b0010 if rn == 0xF => InstructionPattern::MoveImmediate { op: MoveOp::Movz },
                    0b0010 => InstructionPattern::LogicalImmediate { op: LogicalOp::Or },
                    0b0011 if rn == 0xF => InstructionPattern::MoveImmediate { op: MoveOp::Movn },
                    0b0011 => InstructionPattern::LogicalImmediate { op: LogicalOp::Or }, // ORN
                    0b0100 if rd == 0xF => InstructionPattern::Test,                      // TEQ
                    0b0100 => InstructionPattern::LogicalImmediate { op: LogicalOp::Xor },
                    0b1000 if rd == 0xF => InstructionPattern::Compare { is_neg: true },
                    0b1000 => InstructionPattern::AddSubImmediate {
                        is_sub: false,
                        set_flags: false,
                    },
                    0b1010 => InstructionPattern::AddSubImmediate {
                        is_sub: false,
                        set_flags: false,
                    }, // ADC
                    0b1011 => InstructionPattern::AddSubImmediate {
                        is_sub: true,
                        set_flags: false,
                    }, // SBC
                    0b1101 if rd == 0xF => InstructionPattern::Compare { is_neg: false },
                    0b1101 => InstructionPattern::AddSubImmediate {
                        is_sub: true,
                        set_flags: false,
                    },
                    0b1110 => InstructionPattern::AddSubImmediate {
                        is_sub: true,
                        set_flags: false,
                    }, // RSB
                    _ => InstructionPattern::Unknown,
                }
            } else {
                InstructionPattern::Unknown
            }
        }
        0b11 => {
            // Data processing (register) or multiply
            let op1_bits = (encoding >> 20) & 0xF;
            let op2_bits = (encoding >> 4) & 0xF;

            if op1_bits == 0b0001 && (op2_bits & 0b1100) == 0 {
                // Divide
                let op_21 = (encoding >> 21) & 1;
                return InstructionPattern::Divide { signed: op_21 == 1 };
            }

            if op1_bits == 0b0000 && op2_bits == 0b0000 {
                // MUL/MLA
                let ra = (encoding >> 12) & 0xF;
                if ra == 0xF {
                    return InstructionPattern::Multiply {
                        accumulate: false,
                        negate: false,
                    };
                } else {
                    return InstructionPattern::Multiply {
                        accumulate: true,
                        negate: false,
                    };
                }
            }

            InstructionPattern::Unknown
        }
        _ => InstructionPattern::Unknown,
    }
}

fn identify_t16_pattern(encoding: u64) -> InstructionPattern {
    let op = (encoding >> 10) & 0x3F;

    if op >> 4 == 0b00 {
        // Shift, add, subtract, move, compare
        let op_high = (encoding >> 13) & 0x7;
        match op_high {
            0b000 | 0b001 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Lsl,
            },
            0b010 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Lsr,
            },
            0b011 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Asr,
            },
            0b100 => InstructionPattern::MoveImmediate { op: MoveOp::Movz },
            0b101 => InstructionPattern::Compare { is_neg: false },
            0b110 => InstructionPattern::AddSubImmediate {
                is_sub: false,
                set_flags: true,
            },
            0b111 => InstructionPattern::AddSubImmediate {
                is_sub: true,
                set_flags: true,
            },
            _ => InstructionPattern::Unknown,
        }
    } else if op >> 2 == 0b010000 {
        // Data processing
        let dp_op = encoding & 0xF;
        match dp_op {
            0b0000 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::And },
            0b0001 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::Xor },
            0b0010 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Lsl,
            },
            0b0011 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Lsr,
            },
            0b0100 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Asr,
            },
            0b0101 => InstructionPattern::AddSubShiftedReg {
                is_sub: false,
                set_flags: true,
            }, // ADC
            0b0110 => InstructionPattern::AddSubShiftedReg {
                is_sub: true,
                set_flags: true,
            }, // SBC
            0b0111 => InstructionPattern::ShiftVariable {
                shift_type: ShiftType::Ror,
            },
            0b1000 => InstructionPattern::Test,
            0b1001 => InstructionPattern::AddSubShiftedReg {
                is_sub: true,
                set_flags: true,
            }, // RSB/NEG
            0b1010 => InstructionPattern::Compare { is_neg: false },
            0b1011 => InstructionPattern::Compare { is_neg: true },
            0b1100 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::Or },
            0b1101 => InstructionPattern::Multiply {
                accumulate: false,
                negate: false,
            },
            0b1110 => InstructionPattern::LogicalShiftedReg { op: LogicalOp::And }, // BIC
            0b1111 => InstructionPattern::MoveImmediate { op: MoveOp::Movn },       // MVN
            _ => InstructionPattern::Unknown,
        }
    } else {
        InstructionPattern::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_add() {
        let eval = TestEvaluator::new();
        let result = eval.eval_add(10, 20, 64, true);
        assert_eq!(result.dest_value, 30);
        assert!(!result.nzcv.unwrap().n);
        assert!(!result.nzcv.unwrap().z);
        assert!(!result.nzcv.unwrap().c);
        assert!(!result.nzcv.unwrap().v);
    }

    #[test]
    fn test_eval_add_zero_result() {
        let eval = TestEvaluator::new();
        let result = eval.eval_add(0, 0, 64, true);
        assert_eq!(result.dest_value, 0);
        assert!(result.nzcv.unwrap().z);
    }

    #[test]
    fn test_eval_add_overflow() {
        let eval = TestEvaluator::new();
        let result = eval.eval_add(0xFFFF_FFFF, 1, 32, true);
        assert_eq!(result.dest_value, 0);
        assert!(result.nzcv.unwrap().z);
        assert!(result.nzcv.unwrap().c); // Unsigned overflow
    }

    #[test]
    fn test_eval_sub() {
        let eval = TestEvaluator::new();
        let result = eval.eval_sub(30, 10, 64, true);
        assert_eq!(result.dest_value, 20);
        assert!(!result.nzcv.unwrap().n);
        assert!(!result.nzcv.unwrap().z);
        assert!(result.nzcv.unwrap().c); // No borrow
        assert!(!result.nzcv.unwrap().v);
    }

    #[test]
    fn test_add_sub_imm_expected() {
        let eval = TestEvaluator::new();

        // ADD X0, X1, #10
        // sf=1, op=0, S=0, sh=0, imm12=10, Rn=1, Rd=0
        let encoding: u64 = 0x91002820; // ADD X0, X1, #10

        let mut initial = ProcessorState::default();
        initial.gp_regs.insert(1, 100); // X1 = 100

        let (expected, assertions) = eval.compute_add_sub_imm_expected(encoding, &initial);

        assert_eq!(expected.gp_regs.get(&0), Some(&110)); // X0 = 100 + 10
        assert!(assertions.iter().any(|a| {
            matches!(
                (&a.check, &a.expected),
                (AssertionCheck::GpReg(0), AssertionValue::U64(110))
            )
        }));
    }

    #[test]
    fn test_identify_add_sub_imm() {
        // ADD X0, X1, #10
        let add_imm: u64 = 0x91002820;
        assert_eq!(
            identify_a64_pattern(add_imm),
            InstructionPattern::AddSubImmediate {
                is_sub: false,
                set_flags: false
            }
        );

        // ADDS X0, X1, #10
        let adds_imm: u64 = 0xB1002820;
        assert_eq!(
            identify_a64_pattern(adds_imm),
            InstructionPattern::AddSubImmediate {
                is_sub: false,
                set_flags: true
            }
        );

        // SUB X0, X1, #10
        let sub_imm: u64 = 0xD1002820;
        assert_eq!(
            identify_a64_pattern(sub_imm),
            InstructionPattern::AddSubImmediate {
                is_sub: true,
                set_flags: false
            }
        );
    }
}
