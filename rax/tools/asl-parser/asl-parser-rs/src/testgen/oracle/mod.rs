//! Oracle module for computing expected values from ASL expressions.
//!
//! The oracle evaluates ASL expressions at test generation time to compute
//! expected register values, flags, and memory state for execution tests.
//!
//! This is a simplified interpreter that handles common patterns found in
//! ARM instruction decode and execute blocks.

pub mod eval;

use std::collections::HashMap;

pub use eval::{ArithmeticResult, InstructionPattern, NzcvFlags, TestEvaluator};

/// Value type for the oracle - can be integer, bitvector, or boolean.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Integer value (unbounded)
    Int(i128),
    /// Bitvector with explicit width
    Bits { value: u128, width: u32 },
    /// Boolean value
    Bool(bool),
    /// Tuple of values
    Tuple(Vec<Value>),
    /// Unknown/unevaluated value
    Unknown(String),
}

impl Value {
    /// Create a 64-bit value
    pub fn bits64(value: u64) -> Self {
        Value::Bits {
            value: value as u128,
            width: 64,
        }
    }

    /// Create a 32-bit value
    pub fn bits32(value: u32) -> Self {
        Value::Bits {
            value: value as u128,
            width: 32,
        }
    }

    /// Convert to u64 if possible
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            Value::Int(v) if *v >= 0 && *v <= u64::MAX as i128 => Some(*v as u64),
            Value::Bits { value, .. } if *value <= u64::MAX as u128 => Some(*value as u64),
            _ => None,
        }
    }

    /// Convert to i64 if possible
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Value::Int(v) if *v >= i64::MIN as i128 && *v <= i64::MAX as i128 => Some(*v as i64),
            Value::Bits { value, width } => {
                // Sign extend based on width
                let mask = (1u128 << width) - 1;
                let val = *value & mask;
                let sign_bit = 1u128 << (width - 1);
                if val & sign_bit != 0 {
                    // Negative - sign extend
                    Some((val | !mask) as i128 as i64)
                } else {
                    Some(val as i64)
                }
            }
            _ => None,
        }
    }

    /// Convert to bool if possible
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            Value::Int(v) => Some(*v != 0),
            Value::Bits { value, .. } => Some(*value != 0),
            _ => None,
        }
    }

    /// Get bit width if this is a bitvector
    pub fn width(&self) -> Option<u32> {
        match self {
            Value::Bits { width, .. } => Some(*width),
            _ => None,
        }
    }

    /// Check if the value represents zero
    pub fn is_zero(&self) -> bool {
        match self {
            Value::Int(v) => *v == 0,
            Value::Bits { value, .. } => *value == 0,
            Value::Bool(b) => !*b,
            _ => false,
        }
    }
}

/// Environment holding variable bindings during evaluation.
#[derive(Debug, Clone)]
pub struct Environment {
    /// Variable bindings
    bindings: HashMap<String, Value>,
    /// GP register values (X0-X30)
    gp_regs: Vec<Value>,
    /// SIMD register values (V0-V31)
    simd_regs: Vec<Value>,
    /// Stack pointer
    sp: Value,
    /// Program counter
    pc: Value,
    /// Condition flags (NZCV packed)
    nzcv: Value,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    /// Create a new environment
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            gp_regs: (0..31).map(|_| Value::bits64(0)).collect(),
            simd_regs: (0..32)
                .map(|_| Value::Bits {
                    value: 0,
                    width: 128,
                })
                .collect(),
            sp: Value::bits64(0x1000_0000), // Default SP
            pc: Value::bits64(0),
            nzcv: Value::Bits { value: 0, width: 4 },
        }
    }

    /// Set a variable binding
    pub fn set(&mut self, name: &str, value: Value) {
        self.bindings.insert(name.to_string(), value);
    }

    /// Get a variable binding
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.bindings.get(name)
    }

    /// Set a GP register
    pub fn set_x(&mut self, reg: u8, value: u64) {
        if reg < 31 {
            self.gp_regs[reg as usize] = Value::bits64(value);
        }
        // reg 31 writes to ZR (discarded) or SP depending on context
    }

    /// Get a GP register
    pub fn get_x(&self, reg: u8) -> u64 {
        if reg < 31 {
            self.gp_regs[reg as usize].to_u64().unwrap_or(0)
        } else {
            0 // XZR reads as 0
        }
    }

    /// Set SP
    pub fn set_sp(&mut self, value: u64) {
        self.sp = Value::bits64(value);
    }

    /// Get SP
    pub fn get_sp(&self) -> u64 {
        self.sp.to_u64().unwrap_or(0)
    }

    /// Set PC
    pub fn set_pc(&mut self, value: u64) {
        self.pc = Value::bits64(value);
    }

    /// Get PC
    pub fn get_pc(&self) -> u64 {
        self.pc.to_u64().unwrap_or(0)
    }

    /// Set NZCV flags
    pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        let value = ((n as u128) << 3) | ((z as u128) << 2) | ((c as u128) << 1) | (v as u128);
        self.nzcv = Value::Bits { value, width: 4 };
    }

    /// Get NZCV as tuple
    pub fn get_nzcv(&self) -> (bool, bool, bool, bool) {
        let v = self.nzcv.to_u64().unwrap_or(0) as u8;
        ((v & 8) != 0, (v & 4) != 0, (v & 2) != 0, (v & 1) != 0)
    }
}

/// Oracle for evaluating ASL expressions.
pub struct Oracle {
    /// Current evaluation environment
    pub env: Environment,
}

impl Oracle {
    /// Create a new oracle with default environment
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    /// Create oracle with given environment
    pub fn with_env(env: Environment) -> Self {
        Self { env }
    }

    /// Evaluate an addition operation
    pub fn eval_add(&self, a: u64, b: u64, width: u32) -> (u64, bool, bool, bool, bool) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let result = a.wrapping_add(b) & mask;
        let unsigned_overflow = (a as u128) + (b as u128) > mask as u128;

        // Signed overflow: same signs in, different sign out
        let sign_bit = 1u64 << (width - 1);
        let a_neg = (a & sign_bit) != 0;
        let b_neg = (b & sign_bit) != 0;
        let r_neg = (result & sign_bit) != 0;
        let signed_overflow = (a_neg == b_neg) && (r_neg != a_neg);

        let n = (result & sign_bit) != 0;
        let z = result == 0;
        let c = unsigned_overflow;
        let v = signed_overflow;

        (result, n, z, c, v)
    }

    /// Evaluate a subtraction operation (a - b)
    pub fn eval_sub(&self, a: u64, b: u64, width: u32) -> (u64, bool, bool, bool, bool) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let result = a.wrapping_sub(b) & mask;
        let unsigned_overflow = a >= b; // C flag is inverted for sub

        let sign_bit = 1u64 << (width - 1);
        let a_neg = (a & sign_bit) != 0;
        let b_neg = (b & sign_bit) != 0;
        let r_neg = (result & sign_bit) != 0;
        let signed_overflow = (a_neg != b_neg) && (r_neg != a_neg);

        let n = (result & sign_bit) != 0;
        let z = result == 0;
        let c = unsigned_overflow;
        let v = signed_overflow;

        (result, n, z, c, v)
    }

    /// Evaluate logical AND
    pub fn eval_and(&self, a: u64, b: u64, width: u32) -> (u64, bool, bool) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let result = (a & b) & mask;
        let sign_bit = 1u64 << (width - 1);
        let n = (result & sign_bit) != 0;
        let z = result == 0;

        (result, n, z)
    }

    /// Evaluate logical OR
    pub fn eval_or(&self, a: u64, b: u64, width: u32) -> (u64, bool, bool) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let result = (a | b) & mask;
        let sign_bit = 1u64 << (width - 1);
        let n = (result & sign_bit) != 0;
        let z = result == 0;

        (result, n, z)
    }

    /// Evaluate logical XOR
    pub fn eval_xor(&self, a: u64, b: u64, width: u32) -> (u64, bool, bool) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let result = (a ^ b) & mask;
        let sign_bit = 1u64 << (width - 1);
        let n = (result & sign_bit) != 0;
        let z = result == 0;

        (result, n, z)
    }

    /// Evaluate left shift
    pub fn eval_lsl(&self, value: u64, shift: u32, width: u32) -> (u64, Option<bool>) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        if shift == 0 {
            (value & mask, None)
        } else if shift >= width {
            (0, Some(false))
        } else {
            let result = (value << shift) & mask;
            let carry_bit = (value >> (width - shift)) & 1;
            (result, Some(carry_bit != 0))
        }
    }

    /// Evaluate logical right shift
    pub fn eval_lsr(&self, value: u64, shift: u32, width: u32) -> (u64, Option<bool>) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let masked = value & mask;
        if shift == 0 {
            (masked, None)
        } else if shift >= width {
            (0, Some(false))
        } else {
            let result = masked >> shift;
            let carry_bit = (masked >> (shift - 1)) & 1;
            (result, Some(carry_bit != 0))
        }
    }

    /// Evaluate arithmetic right shift
    pub fn eval_asr(&self, value: u64, shift: u32, width: u32) -> (u64, Option<bool>) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let masked = value & mask;
        let sign_bit = 1u64 << (width - 1);
        let is_negative = (masked & sign_bit) != 0;

        if shift == 0 {
            (masked, None)
        } else if shift >= width {
            let result = if is_negative { mask } else { 0 };
            (result, Some(is_negative))
        } else {
            // Arithmetic shift right
            let logical_shift = masked >> shift;
            let result = if is_negative {
                // Fill with 1s from the top
                let fill = mask << (width - shift);
                (logical_shift | fill) & mask
            } else {
                logical_shift
            };
            let carry_bit = (masked >> (shift - 1)) & 1;
            (result, Some(carry_bit != 0))
        }
    }

    /// Evaluate rotate right
    pub fn eval_ror(&self, value: u64, shift: u32, width: u32) -> (u64, Option<bool>) {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };

        let masked = value & mask;
        let shift = shift % width;

        if shift == 0 {
            (masked, None)
        } else {
            let result = ((masked >> shift) | (masked << (width - shift))) & mask;
            let carry_bit = (result >> (width - 1)) & 1;
            (result, Some(carry_bit != 0))
        }
    }

    /// Zero extend a value to a wider width
    pub fn zero_extend(&self, value: u64, from_width: u32, to_width: u32) -> u64 {
        let from_mask = (1u64 << from_width) - 1;
        value & from_mask
    }

    /// Sign extend a value to a wider width
    pub fn sign_extend(&self, value: u64, from_width: u32, to_width: u32) -> u64 {
        let from_mask = (1u64 << from_width) - 1;
        let masked = value & from_mask;
        let sign_bit = 1u64 << (from_width - 1);

        if (masked & sign_bit) != 0 {
            // Negative - extend with 1s
            let extension = !from_mask;
            let to_mask = if to_width == 64 {
                u64::MAX
            } else {
                (1u64 << to_width) - 1
            };
            (masked | extension) & to_mask
        } else {
            masked
        }
    }

    /// Reverse bytes in a value
    pub fn reverse_bytes(&self, value: u64, width: u32) -> u64 {
        match width {
            16 => ((value & 0xFF) << 8) | ((value >> 8) & 0xFF),
            32 => {
                let b0 = (value >> 0) & 0xFF;
                let b1 = (value >> 8) & 0xFF;
                let b2 = (value >> 16) & 0xFF;
                let b3 = (value >> 24) & 0xFF;
                (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
            }
            64 => value.swap_bytes(),
            _ => value,
        }
    }

    /// Count leading zeros
    pub fn count_leading_zeros(&self, value: u64, width: u32) -> u32 {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let masked = value & mask;

        if masked == 0 {
            width
        } else {
            let lz = masked.leading_zeros();
            lz - (64 - width)
        }
    }

    /// Count leading sign bits (number of bits equal to MSB - 1)
    pub fn count_leading_sign_bits(&self, value: u64, width: u32) -> u32 {
        let mask = if width == 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let masked = value & mask;
        let sign_bit = (masked >> (width - 1)) & 1;

        if sign_bit == 0 {
            self.count_leading_zeros(masked, width).saturating_sub(1)
        } else {
            self.count_leading_zeros(!masked & mask, width)
                .saturating_sub(1)
        }
    }

    /// Bit reverse
    pub fn reverse_bits(&self, value: u64, width: u32) -> u64 {
        let mut result = 0u64;
        for i in 0..width {
            if (value >> i) & 1 != 0 {
                result |= 1 << (width - 1 - i);
            }
        }
        result
    }

    /// Decode ARM logical immediate bitmask.
    ///
    /// Implements the DecodeBitMasks() function from ARM ARM.
    /// Returns the 64-bit bitmask, or None if the encoding is invalid.
    ///
    /// # Arguments
    /// * `n` - The N bit (1 bit)
    /// * `imms` - The imms field (6 bits)
    /// * `immr` - The immr field (6 bits)
    /// * `is_64bit` - True for 64-bit register, false for 32-bit
    pub fn decode_bitmasks(&self, n: u32, imms: u32, immr: u32, is_64bit: bool) -> Option<u64> {
        // Determine the element size
        // len = HighestSetBit(N:NOT(imms<5:0>))
        // For 64-bit: N can be 1, giving 7-bit value N:~imms
        // For 32-bit: N must be 0

        if !is_64bit && n != 0 {
            return None; // Invalid: N=1 not allowed for 32-bit
        }

        // Combine N with NOT(imms) to find element length
        // This is a 7-bit value: N:~imms[5:0]
        let nimms = (n << 6) | ((!imms) & 0x3F);

        // Find highest set bit position (HighestSetBit in ARM pseudocode)
        if nimms == 0 {
            return None; // Invalid encoding
        }
        let len = 31 - nimms.leading_zeros();

        if len < 1 {
            return None; // Invalid encoding
        }

        // Element size is 2^len
        let esize = 1u32 << len;

        // levels = Ones(len) - a mask of 'len' 1-bits
        let levels = (1u32 << len) - 1;

        // Check for reserved value: imms AND levels == levels is reserved
        if (imms & levels) == levels {
            return None; // Reserved encoding
        }

        // S and R are masked by levels
        let s = imms & levels;
        let r = immr & levels;

        // Create the element pattern: (S+1) consecutive 1-bits
        let welem: u64 = (1u64 << (s + 1)) - 1;

        // Element mask (handle 64-bit specially to avoid overflow)
        let elem_mask: u64 = if esize >= 64 {
            u64::MAX
        } else {
            (1u64 << esize) - 1
        };

        // Rotate right within element
        let welem_rotated = if r == 0 {
            welem
        } else {
            let r = r as u32;
            ((welem >> r) | (welem << (esize - r))) & elem_mask
        };

        // Replicate element across the register width
        let mut result = 0u64;
        let reg_size = if is_64bit { 64 } else { 32 };
        if esize >= 64 {
            // No replication needed for 64-bit element
            result = welem_rotated;
        } else {
            let mut pos = 0u32;
            while pos < reg_size {
                result |= welem_rotated << pos;
                pos += esize;
            }
        }

        // Mask to register size
        if !is_64bit {
            result &= 0xFFFFFFFF;
        }

        Some(result)
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_no_overflow() {
        let oracle = Oracle::new();
        let (result, n, z, c, v) = oracle.eval_add(10, 20, 32);
        assert_eq!(result, 30);
        assert!(!n); // not negative
        assert!(!z); // not zero
        assert!(!c); // no carry
        assert!(!v); // no overflow
    }

    #[test]
    fn test_add_unsigned_overflow() {
        let oracle = Oracle::new();
        let (result, _, _, c, _) = oracle.eval_add(0xFFFF_FFFF, 1, 32);
        assert_eq!(result, 0);
        assert!(c); // carry occurred
    }

    #[test]
    fn test_add_signed_overflow() {
        let oracle = Oracle::new();
        let (result, n, _, _, v) = oracle.eval_add(0x7FFF_FFFF, 1, 32);
        assert_eq!(result, 0x8000_0000);
        assert!(n); // negative result
        assert!(v); // signed overflow
    }

    #[test]
    fn test_sub_zero() {
        let oracle = Oracle::new();
        let (result, n, z, c, v) = oracle.eval_sub(100, 100, 32);
        assert_eq!(result, 0);
        assert!(!n);
        assert!(z); // is zero
        assert!(c); // no borrow (a >= b)
        assert!(!v);
    }

    #[test]
    fn test_sign_extend() {
        let oracle = Oracle::new();
        // Sign extend -1 from 8 bits to 32 bits
        let result = oracle.sign_extend(0xFF, 8, 32);
        assert_eq!(result, 0xFFFF_FFFF);

        // Sign extend 127 from 8 bits to 32 bits (no extension needed)
        let result = oracle.sign_extend(0x7F, 8, 32);
        assert_eq!(result, 0x7F);
    }

    #[test]
    fn test_shifts() {
        let oracle = Oracle::new();

        let (result, _) = oracle.eval_lsl(0x1, 4, 32);
        assert_eq!(result, 0x10);

        let (result, _) = oracle.eval_lsr(0x100, 4, 32);
        assert_eq!(result, 0x10);

        let (result, _) = oracle.eval_asr(0x8000_0000, 4, 32);
        assert_eq!(result, 0xF800_0000);
    }

    #[test]
    fn test_clz() {
        let oracle = Oracle::new();
        assert_eq!(oracle.count_leading_zeros(0, 32), 32);
        assert_eq!(oracle.count_leading_zeros(1, 32), 31);
        assert_eq!(oracle.count_leading_zeros(0x8000_0000, 32), 0);
    }

    #[test]
    fn test_decode_bitmasks() {
        let oracle = Oracle::new();

        // Test 64-bit patterns with N=1
        // N=1, imms=0b000111 (7), immr=0 -> 8 consecutive 1s = 0xFF
        assert_eq!(oracle.decode_bitmasks(1, 0b000111, 0, true), Some(0xFF));

        // N=1, imms=0b001111 (15), immr=0 -> 16 consecutive 1s = 0xFFFF
        assert_eq!(oracle.decode_bitmasks(1, 0b001111, 0, true), Some(0xFFFF));

        // N=1, imms=0b011111 (31), immr=0 -> 32 consecutive 1s = 0xFFFFFFFF
        assert_eq!(
            oracle.decode_bitmasks(1, 0b011111, 0, true),
            Some(0xFFFFFFFF)
        );

        // N=1, imms=0b000000 (0), immr=0 -> 1 bit = 0x1
        assert_eq!(oracle.decode_bitmasks(1, 0b000000, 0, true), Some(0x1));

        // N=1, imms=0b111110 (62), immr=0 -> 63 consecutive 1s = 0x7FFFFFFFFFFFFFFF
        assert_eq!(
            oracle.decode_bitmasks(1, 0b111110, 0, true),
            Some(0x7FFFFFFFFFFFFFFF)
        );

        // Test rotation: N=1, imms=0b000111 (8 bits), immr=4 -> 0xFF rotated right by 4
        // 0xFF ROR 4 = 0xF00000000000000F
        assert_eq!(
            oracle.decode_bitmasks(1, 0b000111, 4, true),
            Some(0xF00000000000000F)
        );

        // Test 32-bit patterns (N must be 0)
        // N=0, imms=0b000111, immr=0 -> 0xFF
        assert_eq!(oracle.decode_bitmasks(0, 0b000111, 0, false), Some(0xFF));

        // N=1 is invalid for 32-bit
        assert_eq!(oracle.decode_bitmasks(1, 0b000111, 0, false), None);

        // Reserved encoding: imms == levels (all 1s in imms for that element size)
        // For N=1, 64-bit element, imms=0b111111 is reserved
        assert_eq!(oracle.decode_bitmasks(1, 0b111111, 0, true), None);
    }
}
