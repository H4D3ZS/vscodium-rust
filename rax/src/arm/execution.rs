//! ARM instruction execution semantics reference.
//!
//! This module provides the execution semantics for ARMv7 instructions,
//! derived from the arm-js-py reference implementation. It serves as
//! both documentation and implementation reference for the RAX ARM emulator.
//!
//! # Design Philosophy
//!
//! The ARM architecture uses a consistent pattern for instruction execution:
//! 1. Decode operands (registers, immediates, shifts)
//! 2. Compute the operation result
//! 3. Update destination register(s)
//! 4. Optionally update flags (NZCV) if S bit is set
//! 5. Handle PC as destination (triggers branch)
//!
//! # Flag Computation
//!
//! ARM flags are computed differently based on instruction type:
//! - **Logical ops** (AND, ORR, EOR, etc.): N from bit 31, Z from result==0, C from shifter
//! - **Arithmetic ops** (ADD, SUB, etc.): N, Z as above, C and V from add_with_carry
//!
//! # Key Implementation Notes
//!
//! - PC reads as PC+8 in ARM mode (2-instruction pipeline prefetch)
//! - Writing to PC triggers a branch, not a register write
//! - Shift amount of 0 for LSL means no shift (carry unchanged)
//! - Shift amount of 0 for LSR/ASR means shift by 32
//! - Shift type 3 with imm5=0 means RRX, otherwise ROR

use crate::arm::decoder::ShiftType;
use crate::arm::vfp::VfpState;

// =============================================================================
// Flag Computation
// =============================================================================

/// Compute N (Negative) flag from 32-bit result.
#[inline]
pub fn compute_n_flag(result: u32) -> bool {
    (result >> 31) != 0
}

/// Compute Z (Zero) flag from 32-bit result.
#[inline]
pub fn compute_z_flag(result: u32) -> bool {
    result == 0
}

/// Add with carry, returning (result, carry_out, overflow).
///
/// This is the fundamental arithmetic operation for ARM. It handles:
/// - ADD: add_with_carry(x, y, 0)
/// - ADC: add_with_carry(x, y, C)
/// - SUB: add_with_carry(x, !y, 1)
/// - SBC: add_with_carry(x, !y, C)
/// - RSB: add_with_carry(!x, y, 1)
/// - RSC: add_with_carry(!x, y, C)
/// - CMP: add_with_carry(x, !y, 1) (discard result, keep flags)
/// - CMN: add_with_carry(x, y, 0) (discard result, keep flags)
///
/// # Implementation
///
/// ```text
/// unsigned_sum = x + y + carry_in
/// signed_sum = (x as i32) + (y as i32) + carry_in
/// result = unsigned_sum & 0xFFFFFFFF
/// carry_out = (result != unsigned_sum)  // overflow in unsigned domain
/// overflow = (result as i32 != signed_sum)  // overflow in signed domain
/// ```
#[inline]
pub fn add_with_carry(x: u32, y: u32, carry_in: u32) -> (u32, bool, bool) {
    let unsigned_sum = (x as u64) + (y as u64) + (carry_in as u64);
    let signed_sum = (x as i32 as i64) + (y as i32 as i64) + (carry_in as i64);
    let result = unsigned_sum as u32;

    let carry_out = result as u64 != unsigned_sum;
    let overflow = result as i32 as i64 != signed_sum;

    (result, carry_out, overflow)
}

/// Sign-extend a value from `from_bits` to 32 bits.
#[inline]
pub fn sign_extend(value: u32, from_bits: u32) -> u32 {
    let sign_bit = 1u32 << (from_bits - 1);
    if (value & sign_bit) != 0 {
        value | !((1u32 << from_bits) - 1)
    } else {
        value
    }
}

/// Convert u32 to i32 (two's complement interpretation).
#[inline]
pub fn sint32(x: u32) -> i32 {
    x as i32
}

// =============================================================================
// Shift Operations with Carry
// =============================================================================

/// Perform shift operation with carry output.
///
/// Returns (result, carry_out).
///
/// # Shift Types
///
/// - **LSL (Logical Shift Left)**: Shifts bits left, fills with zeros.
///   Carry out is the last bit shifted out (bit 32-amount).
///   
/// - **LSR (Logical Shift Right)**: Shifts bits right, fills with zeros.
///   Carry out is the last bit shifted out (bit amount-1).
///   
/// - **ASR (Arithmetic Shift Right)**: Shifts bits right, fills with sign bit.
///   Carry out is the last bit shifted out (bit amount-1).
///   
/// - **ROR (Rotate Right)**: Rotates bits right.
///   Carry out is bit 31 of result.
///   
/// - **RRX (Rotate Right with Extend)**: 33-bit rotate through carry.
///   Result[31] = carry_in, Result[30:0] = value[31:1].
///   Carry out is value[0].
///
/// # Special Cases
///
/// - Amount = 0: Return value unchanged, carry_out = carry_in
/// - LSL by 32: Result = 0, carry_out = value[0]
/// - LSR/ASR by 32: Result = 0 (LSR) or all sign bits (ASR), carry_out = value[31]
pub fn shift_c(value: u32, shift_type: ShiftType, amount: u32, carry_in: bool) -> (u32, bool) {
    if amount == 0 {
        return (value, carry_in);
    }

    match shift_type {
        ShiftType::LSL => {
            if amount >= 32 {
                let carry = if amount == 32 {
                    (value & 1) != 0
                } else {
                    false
                };
                (0, carry)
            } else {
                let extended = (value as u64) << amount;
                let carry = ((extended >> 32) & 1) != 0;
                (extended as u32, carry)
            }
        }
        ShiftType::LSR => {
            if amount >= 32 {
                let carry = if amount == 32 {
                    (value >> 31) != 0
                } else {
                    false
                };
                (0, carry)
            } else {
                let carry = ((value >> (amount - 1)) & 1) != 0;
                (value >> amount, carry)
            }
        }
        ShiftType::ASR => {
            if amount >= 32 {
                let sign = (value >> 31) != 0;
                let result = if sign { 0xFFFFFFFF } else { 0 };
                (result, sign)
            } else {
                let carry = ((value >> (amount - 1)) & 1) != 0;
                let result = ((value as i32) >> amount) as u32;
                (result, carry)
            }
        }
        ShiftType::ROR => {
            let m = amount % 32;
            let result = if m == 0 { value } else { value.rotate_right(m) };
            let carry = (result >> 31) != 0;
            (result, carry)
        }
        ShiftType::RRX => {
            // 33-bit rotate: carry_in -> bit 31, bit 0 -> carry_out
            let carry = (value & 1) != 0;
            let result = (value >> 1) | (if carry_in { 1 << 31 } else { 0 });
            (result, carry)
        }
    }
}

/// Perform shift without carry (for cases where carry is not needed).
#[inline]
pub fn shift(value: u32, shift_type: ShiftType, amount: u32, carry_in: bool) -> u32 {
    shift_c(value, shift_type, amount, carry_in).0
}

/// Decode immediate shift encoding from instruction.
///
/// In ARM encoding, shift type is 2 bits and imm5 is 5 bits:
/// - Type 00 (LSL): shift_n = imm5
/// - Type 01 (LSR): shift_n = if imm5 == 0 then 32 else imm5
/// - Type 10 (ASR): shift_n = if imm5 == 0 then 32 else imm5  
/// - Type 11: if imm5 == 0 then RRX else ROR(imm5)
pub fn decode_imm_shift(shift_type_bits: u8, imm5: u8) -> (ShiftType, u8) {
    match shift_type_bits {
        0b00 => (ShiftType::LSL, imm5),
        0b01 => (ShiftType::LSR, if imm5 == 0 { 32 } else { imm5 }),
        0b10 => (ShiftType::ASR, if imm5 == 0 { 32 } else { imm5 }),
        0b11 => {
            if imm5 == 0 {
                (ShiftType::RRX, 1)
            } else {
                (ShiftType::ROR, imm5)
            }
        }
        _ => unreachable!(),
    }
}

/// Expand modified immediate constant (ARM encoding).
///
/// ARM uses a 12-bit immediate encoding: 4-bit rotation + 8-bit immediate.
/// Result = ROR(imm8, rotation * 2)
///
/// Returns (expanded_value, carry_out).
pub fn expand_imm_c(imm12: u32, carry_in: bool) -> (u32, bool) {
    let unrotated = imm12 & 0xFF;
    let rotation = ((imm12 >> 8) & 0xF) * 2;

    if rotation == 0 {
        (unrotated, carry_in)
    } else {
        let result = unrotated.rotate_right(rotation);
        let carry = (result >> 31) != 0;
        (result, carry)
    }
}

// =============================================================================
// Processor Modes and Banking
// =============================================================================

/// ARM processor modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProcessorMode {
    /// User mode (unprivileged)
    User = 0x10,
    /// FIQ mode (fast interrupt)
    Fiq = 0x11,
    /// IRQ mode (normal interrupt)
    Irq = 0x12,
    /// Supervisor mode (SVC/SWI)
    Supervisor = 0x13,
    /// Monitor mode (secure world)
    Monitor = 0x16,
    /// Abort mode (memory abort)
    Abort = 0x17,
    /// Undefined mode (undefined instruction)
    Undefined = 0x1B,
    /// System mode (privileged, but same regs as User)
    System = 0x1F,
}

impl ProcessorMode {
    /// Decode mode from 5-bit field.
    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0x10 => Some(ProcessorMode::User),
            0x11 => Some(ProcessorMode::Fiq),
            0x12 => Some(ProcessorMode::Irq),
            0x13 => Some(ProcessorMode::Supervisor),
            0x16 => Some(ProcessorMode::Monitor),
            0x17 => Some(ProcessorMode::Abort),
            0x1B => Some(ProcessorMode::Undefined),
            0x1F => Some(ProcessorMode::System),
            _ => None,
        }
    }

    /// Check if mode is privileged.
    pub fn is_privileged(self) -> bool {
        self != ProcessorMode::User
    }

    /// Check if mode has SPSR.
    pub fn has_spsr(self) -> bool {
        !matches!(self, ProcessorMode::User | ProcessorMode::System)
    }
}

/// Banked registers per mode.
///
/// Different modes have different banked registers:
/// - User/System: Share all registers
/// - FIQ: Banks R8-R14, SPSR
/// - IRQ/SVC/ABT/UND/MON: Bank R13, R14, SPSR
#[derive(Clone, Debug, Default)]
pub struct BankedRegisters {
    /// User mode R13-R14
    pub regs_usr: [u32; 2],
    /// FIQ mode R8-R14
    pub regs_fiq: [u32; 7],
    /// IRQ mode R13-R14
    pub regs_irq: [u32; 2],
    /// Supervisor mode R13-R14
    pub regs_svc: [u32; 2],
    /// Monitor mode R13-R14
    pub regs_mon: [u32; 2],
    /// Abort mode R13-R14
    pub regs_abt: [u32; 2],
    /// Undefined mode R13-R14
    pub regs_und: [u32; 2],

    /// Saved PSRs
    pub spsr_fiq: u32,
    pub spsr_irq: u32,
    pub spsr_svc: u32,
    pub spsr_mon: u32,
    pub spsr_abt: u32,
    pub spsr_und: u32,
}

// =============================================================================
// Program Status Register
// =============================================================================

/// CPSR/SPSR bit positions.
pub mod psr {
    pub const N: u32 = 31; // Negative
    pub const Z: u32 = 30; // Zero
    pub const C: u32 = 29; // Carry
    pub const V: u32 = 28; // Overflow
    pub const Q: u32 = 27; // Saturation
    pub const IT_HI: u32 = 26; // IT state high bits (26:25)
    pub const J: u32 = 24; // Jazelle
    pub const GE: u32 = 16; // Greater-than-or-Equal (19:16)
    pub const E: u32 = 9; // Endianness
    pub const A: u32 = 8; // Async abort disable
    pub const I: u32 = 7; // IRQ disable
    pub const F: u32 = 6; // FIQ disable
    pub const T: u32 = 5; // Thumb state
    pub const M: u32 = 0; // Mode (4:0)

    pub const N_MASK: u32 = 1 << N;
    pub const Z_MASK: u32 = 1 << Z;
    pub const C_MASK: u32 = 1 << C;
    pub const V_MASK: u32 = 1 << V;
    pub const NZCV_MASK: u32 = N_MASK | Z_MASK | C_MASK | V_MASK;
    pub const Q_MASK: u32 = 1 << Q;
    pub const MODE_MASK: u32 = 0x1F;
}

/// Parse PSR value into components.
#[derive(Clone, Debug, Default)]
pub struct Psr {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
    pub q: bool,
    /// GE[3:0] flags (bits 19:16), set by SIMD parallel add/sub, read by SEL.
    pub ge: u8,
    pub e: bool,
    pub a: bool,
    pub i: bool,
    pub f: bool,
    pub t: bool,
    pub mode: u8,
    /// IT state (8 bits: IT[7:0], condition and mask)
    /// IT[7:5] = base condition
    /// IT[4:0] = mask bits determining number of instructions
    pub it_state: u8,
}

impl Psr {
    pub fn from_u32(value: u32) -> Self {
        // IT state is split: bits 15:10 = IT[7:2], bits 26:25 = IT[1:0]
        let it_lo = ((value >> 25) & 3) as u8;
        let it_hi = ((value >> 10) & 0x3F) as u8;
        let it_state = (it_hi << 2) | it_lo;

        Psr {
            n: (value >> 31) != 0,
            z: ((value >> 30) & 1) != 0,
            c: ((value >> 29) & 1) != 0,
            v: ((value >> 28) & 1) != 0,
            q: ((value >> 27) & 1) != 0,
            ge: ((value >> 16) & 0xF) as u8,
            e: ((value >> 9) & 1) != 0,
            a: ((value >> 8) & 1) != 0,
            i: ((value >> 7) & 1) != 0,
            f: ((value >> 6) & 1) != 0,
            t: ((value >> 5) & 1) != 0,
            mode: (value & 0x1F) as u8,
            it_state,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut value = self.mode as u32;
        if self.t {
            value |= 1 << 5;
        }
        if self.f {
            value |= 1 << 6;
        }
        if self.i {
            value |= 1 << 7;
        }
        if self.a {
            value |= 1 << 8;
        }
        if self.e {
            value |= 1 << 9;
        }
        // IT state bits: IT[7:2] in 15:10, IT[1:0] in 26:25
        let it_lo = self.it_state & 3;
        let it_hi = (self.it_state >> 2) & 0x3F;
        value |= (it_lo as u32) << 25;
        value |= (it_hi as u32) << 10;
        if self.q {
            value |= 1 << 27;
        }
        value |= ((self.ge & 0xF) as u32) << 16;
        if self.v {
            value |= 1 << 28;
        }
        if self.c {
            value |= 1 << 29;
        }
        if self.z {
            value |= 1 << 30;
        }
        if self.n {
            value |= 1 << 31;
        }
        value
    }

    /// Check if currently in an IT block.
    pub fn in_it_block(&self) -> bool {
        (self.it_state & 0x0F) != 0
    }

    /// Check if this is the last instruction in an IT block.
    pub fn last_in_it_block(&self) -> bool {
        (self.it_state & 0x0F) == 0x08
    }

    /// Get the condition code for the current IT instruction.
    /// Returns the condition that should be evaluated for this instruction.
    pub fn it_condition(&self) -> u8 {
        if !self.in_it_block() {
            return 0x0E; // AL (always)
        }
        let base_cond = (self.it_state >> 4) & 0x0F;
        // Bit 4 of IT state XORed with bit 0 of base condition determines inversion
        let invert = ((self.it_state >> 4) & 1) != (base_cond & 1);
        if invert { base_cond ^ 1 } else { base_cond }
    }

    /// Advance IT state after executing an instruction.
    /// This shifts the mask left and clears IT state when mask becomes 0.
    pub fn advance_it_state(&mut self) {
        if self.in_it_block() {
            // Shift mask left by 1
            let mask = self.it_state & 0x1F;
            let new_mask = (mask << 1) & 0x1F;
            if new_mask == 0 || (new_mask & 0x0F) == 0 {
                // IT block complete
                self.it_state = 0;
            } else {
                self.it_state = (self.it_state & 0xE0) | new_mask;
            }
        }
    }

    /// Set IT state from IT instruction (firstcond[3:0], mask[3:0]).
    pub fn set_it_state(&mut self, firstcond: u8, mask: u8) {
        // IT state = firstcond[3:0] : mask[3:0]
        self.it_state = (firstcond << 4) | (mask & 0x0F);
    }
}

// =============================================================================
// Condition Code Evaluation
// =============================================================================

/// Evaluate condition code against current flags.
///
/// # Condition Codes (bits 31:28)
///
/// | Code | Suffix | Flags              | Meaning                    |
/// |------|--------|--------------------| ---------------------------|
/// | 0000 | EQ     | Z == 1             | Equal                      |
/// | 0001 | NE     | Z == 0             | Not equal                  |
/// | 0010 | CS/HS  | C == 1             | Carry set / Unsigned >=    |
/// | 0011 | CC/LO  | C == 0             | Carry clear / Unsigned <   |
/// | 0100 | MI     | N == 1             | Minus / Negative           |
/// | 0101 | PL     | N == 0             | Plus / Positive or zero    |
/// | 0110 | VS     | V == 1             | Overflow set               |
/// | 0111 | VC     | V == 0             | Overflow clear             |
/// | 1000 | HI     | C == 1 && Z == 0   | Unsigned higher            |
/// | 1001 | LS     | C == 0 || Z == 1   | Unsigned lower or same     |
/// | 1010 | GE     | N == V             | Signed >=                  |
/// | 1011 | LT     | N != V             | Signed <                   |
/// | 1100 | GT     | Z == 0 && N == V   | Signed >                   |
/// | 1101 | LE     | Z == 1 || N != V   | Signed <=                  |
/// | 1110 | AL     | -                  | Always (unconditional)     |
/// | 1111 | NV     | -                  | Never (or unconditional)   |
/// Map a register operand index to its AArch32 register file slot.
///
/// The instruction decoder is shared with AArch64, where the stack pointer is
/// encoding 31. In AArch32 there are only 16 GPRs and SP is r13, so an operand
/// that arrives as 31 (e.g. Thumb `ADD Rd, SP, #imm`) must fold to 13.
#[inline]
pub fn aarch32_reg_index(i: usize) -> usize {
    if i == 31 {
        13
    } else {
        i
    }
}

pub fn condition_passed(cond: u8, n: bool, z: bool, c: bool, v: bool) -> bool {
    let result = match cond >> 1 {
        0 => z,              // EQ/NE
        1 => c,              // CS/CC
        2 => n,              // MI/PL
        3 => v,              // VS/VC
        4 => c && !z,        // HI/LS
        5 => n == v,         // GE/LT
        6 => !z && (n == v), // GT/LE
        7 => true,           // AL
        _ => unreachable!(),
    };

    // Odd conditions (except 15) invert the result
    if (cond & 1) != 0 && cond != 0xF {
        !result
    } else {
        result
    }
}

// =============================================================================
// Bit Manipulation Operations
// =============================================================================

/// Count leading zeros.
#[inline]
pub fn count_leading_zeros(value: u32) -> u32 {
    value.leading_zeros()
}

/// Rotate right.
#[inline]
pub fn ror(value: u32, amount: u32) -> u32 {
    if amount == 0 {
        value
    } else {
        value.rotate_right(amount % 32)
    }
}

/// Byte reverse (REV instruction).
#[inline]
pub fn byte_reverse(value: u32) -> u32 {
    value.swap_bytes()
}

/// Byte reverse packed halfwords (REV16 instruction).
#[inline]
pub fn byte_reverse_16(value: u32) -> u32 {
    let hi = ((value >> 24) & 0xFF) | ((value >> 8) & 0xFF00);
    let lo = ((value >> 8) & 0xFF) | ((value << 8) & 0xFF00);
    (hi << 16) | lo
}

/// Bit reverse (RBIT instruction).
#[inline]
pub fn bit_reverse(value: u32) -> u32 {
    value.reverse_bits()
}

// =============================================================================
// ARMv7 Emulator CPU State
// =============================================================================

/// Complete ARMv7 CPU state for software emulation.
///
/// This structure follows the arm-js-py design pattern, holding all state
/// needed to emulate a complete ARMv7 processor including:
/// - Current register state
/// - Banked registers for all modes
/// - CPSR and all SPSRs
/// - Execution state (branch target, halted)
///
/// # Register Banking
///
/// Different modes have different banked registers:
/// - **User/System**: Share R0-R14 (System is privileged User)
/// - **FIQ**: Banks R8-R14, SPSR_fiq
/// - **IRQ/SVC/ABT/UND/MON**: Bank R13-R14, SPSR_xxx
#[derive(Clone, Debug)]
pub struct Armv7Cpu {
    /// Current register values R0-R15 (R15 = PC)
    pub regs: [u32; 16],
    /// Current Program Status Register
    pub cpsr: Psr,

    // Banked registers per mode
    /// User mode R13-R14
    pub regs_usr: [u32; 2], // SP, LR
    /// FIQ mode R8-R14
    pub regs_fiq: [u32; 7], // R8-R14
    /// User/common R8-R12, saved while executing in FIQ mode (FIQ banks
    /// R8-R12 in addition to SP/LR; every other mode shares the user copies).
    pub regs_usr_high: [u32; 5],
    /// IRQ mode R13-R14
    pub regs_irq: [u32; 2],
    /// Supervisor mode R13-R14
    pub regs_svc: [u32; 2],
    /// Monitor mode R13-R14
    pub regs_mon: [u32; 2],
    /// Abort mode R13-R14
    pub regs_abt: [u32; 2],
    /// Undefined mode R13-R14
    pub regs_und: [u32; 2],

    // Saved Program Status Registers
    pub spsr_fiq: Psr,
    pub spsr_irq: Psr,
    pub spsr_svc: Psr,
    pub spsr_mon: Psr,
    pub spsr_abt: Psr,
    pub spsr_und: Psr,

    // Execution state
    /// Pending branch target (None if no branch)
    pub branch_to: Option<u32>,
    /// Whether CPU is halted (WFI)
    pub is_halted: bool,

    // Shifter output (set by shift operations)
    pub carry_out: bool,
    pub overflow: bool,

    /// VFP/NEON register file and FP system registers.
    pub vfp: VfpState,

    /// CP15 system-control coprocessor (SCTLR/TTBR/DACR/FSR/FAR/...): the
    /// source of truth for MCR/MRC p15 and the v6 MMU configuration.
    pub cp15: crate::arm::cp15::Cp15State,
}

impl Default for Armv7Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Armv7Cpu {
    /// Create a new CPU in Supervisor mode.
    pub fn new() -> Self {
        let mut cpu = Armv7Cpu {
            regs: [0; 16],
            cpsr: Psr::default(),
            regs_usr: [0; 2],
            regs_fiq: [0; 7],
            regs_usr_high: [0; 5],
            regs_irq: [0; 2],
            regs_svc: [0; 2],
            regs_mon: [0; 2],
            regs_abt: [0; 2],
            regs_und: [0; 2],
            spsr_fiq: Psr::default(),
            spsr_irq: Psr::default(),
            spsr_svc: Psr::default(),
            spsr_mon: Psr::default(),
            spsr_abt: Psr::default(),
            spsr_und: Psr::default(),
            branch_to: None,
            is_halted: false,
            carry_out: false,
            overflow: false,
            vfp: VfpState::new(),
            cp15: crate::arm::cp15::Cp15State::new(),
        };
        // Start in Supervisor mode with IRQ/FIQ disabled
        cpu.cpsr.mode = ProcessorMode::Supervisor as u8;
        cpu.cpsr.i = true;
        cpu.cpsr.f = true;
        cpu
    }

    /// Get PC value for instruction execution (PC + 8 due to pipeline).
    #[inline]
    pub fn get_pc(&self) -> u32 {
        self.regs[15].wrapping_add(8)
    }



    /// Get register value, handling PC specially.
    #[inline]
    pub fn reg(&self, i: usize) -> u32 {
        let i = aarch32_reg_index(i);
        if i == 15 { self.get_pc() } else { self.regs[i] }
    }

    /// Set register value. Writing to PC sets branch_to instead.
    #[inline]
    pub fn set_reg(&mut self, i: usize, value: u32) {
        let i = aarch32_reg_index(i);
        if i == 15 {
            self.branch_to = Some(value);
        } else {
            self.regs[i] = value;
        }
    }

    /// Check if current mode is privileged.
    pub fn is_privileged(&self) -> bool {
        self.cpsr.mode != ProcessorMode::User as u8
    }

    /// Check if in User or System mode (no SPSR).
    pub fn is_user_or_system(&self) -> bool {
        self.cpsr.mode == ProcessorMode::User as u8 || self.cpsr.mode == ProcessorMode::System as u8
    }

    /// Get the current mode's SPSR.
    pub fn get_current_spsr(&self) -> Option<&Psr> {
        match ProcessorMode::from_bits(self.cpsr.mode) {
            Some(ProcessorMode::Fiq) => Some(&self.spsr_fiq),
            Some(ProcessorMode::Irq) => Some(&self.spsr_irq),
            Some(ProcessorMode::Supervisor) => Some(&self.spsr_svc),
            Some(ProcessorMode::Monitor) => Some(&self.spsr_mon),
            Some(ProcessorMode::Abort) => Some(&self.spsr_abt),
            Some(ProcessorMode::Undefined) => Some(&self.spsr_und),
            _ => None, // User and System don't have SPSR
        }
    }

    /// Get mutable reference to current mode's SPSR.
    pub fn get_current_spsr_mut(&mut self) -> Option<&mut Psr> {
        match ProcessorMode::from_bits(self.cpsr.mode) {
            Some(ProcessorMode::Fiq) => Some(&mut self.spsr_fiq),
            Some(ProcessorMode::Irq) => Some(&mut self.spsr_irq),
            Some(ProcessorMode::Supervisor) => Some(&mut self.spsr_svc),
            Some(ProcessorMode::Monitor) => Some(&mut self.spsr_mon),
            Some(ProcessorMode::Abort) => Some(&mut self.spsr_abt),
            Some(ProcessorMode::Undefined) => Some(&mut self.spsr_und),
            _ => None,
        }
    }

    /// Save current registers to banked storage for mode switch.
    pub fn save_to_banked(&mut self, mode: ProcessorMode) {
        match mode {
            ProcessorMode::User | ProcessorMode::System => {
                self.regs_usr[0] = self.regs[13];
                self.regs_usr[1] = self.regs[14];
            }
            ProcessorMode::Fiq => {
                for i in 0..7 {
                    self.regs_fiq[i] = self.regs[8 + i];
                }
            }
            ProcessorMode::Irq => {
                self.regs_irq[0] = self.regs[13];
                self.regs_irq[1] = self.regs[14];
            }
            ProcessorMode::Supervisor => {
                self.regs_svc[0] = self.regs[13];
                self.regs_svc[1] = self.regs[14];
            }
            ProcessorMode::Monitor => {
                self.regs_mon[0] = self.regs[13];
                self.regs_mon[1] = self.regs[14];
            }
            ProcessorMode::Abort => {
                self.regs_abt[0] = self.regs[13];
                self.regs_abt[1] = self.regs[14];
            }
            ProcessorMode::Undefined => {
                self.regs_und[0] = self.regs[13];
                self.regs_und[1] = self.regs[14];
            }
        }
    }

    /// Restore registers from banked storage for mode switch.
    pub fn restore_from_banked(&mut self, mode: ProcessorMode) {
        match mode {
            ProcessorMode::User | ProcessorMode::System => {
                self.regs[13] = self.regs_usr[0];
                self.regs[14] = self.regs_usr[1];
            }
            ProcessorMode::Fiq => {
                for i in 0..7 {
                    self.regs[8 + i] = self.regs_fiq[i];
                }
            }
            ProcessorMode::Irq => {
                self.regs[13] = self.regs_irq[0];
                self.regs[14] = self.regs_irq[1];
            }
            ProcessorMode::Supervisor => {
                self.regs[13] = self.regs_svc[0];
                self.regs[14] = self.regs_svc[1];
            }
            ProcessorMode::Monitor => {
                self.regs[13] = self.regs_mon[0];
                self.regs[14] = self.regs_mon[1];
            }
            ProcessorMode::Abort => {
                self.regs[13] = self.regs_abt[0];
                self.regs[14] = self.regs_abt[1];
            }
            ProcessorMode::Undefined => {
                self.regs[13] = self.regs_und[0];
                self.regs[14] = self.regs_und[1];
            }
        }
    }

    /// Change processor mode, handling register banking.
    pub fn change_mode(&mut self, new_mode: ProcessorMode) {
        let old_mode =
            ProcessorMode::from_bits(self.cpsr.mode).unwrap_or(ProcessorMode::Supervisor);

        if old_mode == new_mode {
            return;
        }

        // Save current registers
        self.save_to_banked(old_mode);

        // FIQ banks R8-R12 on top of SP/LR; all other modes share the user
        // copies, so swap them only when crossing the FIQ boundary.
        if new_mode == ProcessorMode::Fiq && old_mode != ProcessorMode::Fiq {
            self.regs_usr_high.copy_from_slice(&self.regs[8..13]);
        }
        if old_mode == ProcessorMode::Fiq && new_mode != ProcessorMode::Fiq {
            self.regs[8..13].copy_from_slice(&self.regs_usr_high);
        }

        // Update mode
        self.cpsr.mode = new_mode as u8;

        // Restore new mode's registers
        self.restore_from_banked(new_mode);
    }

    /// Set APSR flags from result.
    ///
    /// For logical operations, use `set_overflow = false`.
    /// For arithmetic operations, use `set_overflow = true`.
    pub fn set_apsr(&mut self, result: u32, set_overflow: bool) {
        self.cpsr.n = compute_n_flag(result);
        self.cpsr.z = compute_z_flag(result);
        self.cpsr.c = self.carry_out;
        if set_overflow {
            self.cpsr.v = self.overflow;
        }
    }

    /// Perform add with carry, updating carry_out and overflow.
    pub fn add_with_carry(&mut self, x: u32, y: u32, carry_in: bool) -> u32 {
        let (result, carry, overflow) = add_with_carry(x, y, carry_in as u32);
        self.carry_out = carry;
        self.overflow = overflow;
        result
    }

    /// Perform shift with carry, updating carry_out.
    pub fn shift_c(&mut self, value: u32, shift_type: ShiftType, amount: u32) -> u32 {
        let (result, carry) = shift_c(value, shift_type, amount, self.cpsr.c);
        self.carry_out = carry;
        result
    }

    /// Advance PC by instruction size (4 for ARM, 2/4 for Thumb).
    pub fn advance_pc(&mut self, size: u32) {
        self.regs[15] = self.regs[15].wrapping_add(size);
    }

    /// Apply pending branch if any.
    pub fn apply_branch(&mut self) {
        if let Some(target) = self.branch_to.take() {
            self.regs[15] = target;
        }
    }
}

// =============================================================================
// Memory Access Trait
// =============================================================================

/// Memory access interface for ARM emulation.
///
/// This trait abstracts memory operations, allowing the emulator to work
/// with different memory backends (simple array, MMU, etc.).
pub trait ArmMemory {
    /// Read a 32-bit word from address (must be word-aligned).
    fn read_word(&self, addr: u32) -> Result<u32, MemoryError>;

    /// Write a 32-bit word to address (must be word-aligned).
    fn write_word(&mut self, addr: u32, value: u32) -> Result<(), MemoryError>;

    /// Read a 16-bit halfword from address (must be halfword-aligned).
    fn read_halfword(&self, addr: u32) -> Result<u16, MemoryError>;

    /// Write a 16-bit halfword to address (must be halfword-aligned).
    fn write_halfword(&mut self, addr: u32, value: u16) -> Result<(), MemoryError>;

    /// Read a byte from address.
    fn read_byte(&self, addr: u32) -> Result<u8, MemoryError>;

    /// Write a byte to address.
    fn write_byte(&mut self, addr: u32, value: u8) -> Result<(), MemoryError>;

    /// Check if unaligned access is allowed.
    fn allows_unaligned(&self) -> bool {
        true
    }
}

/// Memory access error types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MemoryError {
    /// Address is not aligned for the access size.
    Unaligned(u32),
    /// Address is outside valid memory range.
    OutOfBounds(u32),
    /// Access permission denied.
    PermissionDenied(u32),
    /// Generic bus error.
    BusError(u32),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::Unaligned(addr) => write!(f, "unaligned access at 0x{:08x}", addr),
            MemoryError::OutOfBounds(addr) => write!(f, "out of bounds access at 0x{:08x}", addr),
            MemoryError::PermissionDenied(addr) => write!(f, "permission denied at 0x{:08x}", addr),
            MemoryError::BusError(addr) => write!(f, "bus error at 0x{:08x}", addr),
        }
    }
}

impl std::error::Error for MemoryError {}

// =============================================================================
// Simple Memory Implementation
// =============================================================================

/// Simple flat memory for testing.
pub struct FlatMemory {
    data: Vec<u8>,
    base: u32,
}

impl FlatMemory {
    pub fn new(size: usize, base: u32) -> Self {
        FlatMemory {
            data: vec![0; size],
            base,
        }
    }

    fn translate(&self, addr: u32) -> Option<usize> {
        if addr >= self.base && addr < self.base + self.data.len() as u32 {
            Some((addr - self.base) as usize)
        } else {
            None
        }
    }
}

impl ArmMemory for FlatMemory {
    fn read_word(&self, addr: u32) -> Result<u32, MemoryError> {
        let offset = self.translate(addr).ok_or(MemoryError::OutOfBounds(addr))?;
        if offset + 4 > self.data.len() {
            return Err(MemoryError::OutOfBounds(addr));
        }
        Ok(u32::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ]))
    }

    fn write_word(&mut self, addr: u32, value: u32) -> Result<(), MemoryError> {
        let offset = self.translate(addr).ok_or(MemoryError::OutOfBounds(addr))?;
        if offset + 4 > self.data.len() {
            return Err(MemoryError::OutOfBounds(addr));
        }
        let bytes = value.to_le_bytes();
        self.data[offset..offset + 4].copy_from_slice(&bytes);
        Ok(())
    }

    fn read_halfword(&self, addr: u32) -> Result<u16, MemoryError> {
        let offset = self.translate(addr).ok_or(MemoryError::OutOfBounds(addr))?;
        if offset + 2 > self.data.len() {
            return Err(MemoryError::OutOfBounds(addr));
        }
        Ok(u16::from_le_bytes([
            self.data[offset],
            self.data[offset + 1],
        ]))
    }

    fn write_halfword(&mut self, addr: u32, value: u16) -> Result<(), MemoryError> {
        let offset = self.translate(addr).ok_or(MemoryError::OutOfBounds(addr))?;
        if offset + 2 > self.data.len() {
            return Err(MemoryError::OutOfBounds(addr));
        }
        let bytes = value.to_le_bytes();
        self.data[offset..offset + 2].copy_from_slice(&bytes);
        Ok(())
    }

    fn read_byte(&self, addr: u32) -> Result<u8, MemoryError> {
        let offset = self.translate(addr).ok_or(MemoryError::OutOfBounds(addr))?;
        Ok(self.data[offset])
    }

    fn write_byte(&mut self, addr: u32, value: u8) -> Result<(), MemoryError> {
        let offset = self.translate(addr).ok_or(MemoryError::OutOfBounds(addr))?;
        self.data[offset] = value;
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_with_carry() {
        // Basic addition
        let (r, c, v) = add_with_carry(5, 3, 0);
        assert_eq!(r, 8);
        assert!(!c);
        assert!(!v);

        // Carry out
        let (r, c, v) = add_with_carry(0xFFFFFFFF, 1, 0);
        assert_eq!(r, 0);
        assert!(c);
        assert!(!v);

        // Overflow (positive + positive = negative)
        let (r, c, v) = add_with_carry(0x7FFFFFFF, 1, 0);
        assert_eq!(r, 0x80000000);
        assert!(!c);
        assert!(v);

        // Subtraction via NOT + 1
        let (r, c, v) = add_with_carry(10, !5u32, 1);
        assert_eq!(r, 5);
        assert!(c); // No borrow = carry set
        assert!(!v);

        // Subtraction with borrow
        let (r, c, v) = add_with_carry(3, !5u32, 1);
        assert_eq!(r, 0xFFFFFFFE); // -2
        assert!(!c); // Borrow = carry clear
        assert!(!v);
    }

    #[test]
    fn test_shift_lsl() {
        let (r, c) = shift_c(0x12345678, ShiftType::LSL, 4, false);
        assert_eq!(r, 0x23456780);
        assert!(c); // Bit 28 was 1

        let (r, c) = shift_c(0x80000000, ShiftType::LSL, 1, false);
        assert_eq!(r, 0);
        assert!(c);

        // No shift
        let (r, c) = shift_c(0x12345678, ShiftType::LSL, 0, true);
        assert_eq!(r, 0x12345678);
        assert!(c); // Carry preserved
    }

    #[test]
    fn test_shift_lsr() {
        let (r, c) = shift_c(0x12345678, ShiftType::LSR, 4, false);
        assert_eq!(r, 0x01234567);
        assert!(c); // Bit 3 was 1

        let (r, c) = shift_c(0x80000000, ShiftType::LSR, 32, false);
        assert_eq!(r, 0);
        assert!(c); // Bit 31 was 1
    }

    #[test]
    fn test_shift_asr() {
        let (r, c) = shift_c(0x80000000, ShiftType::ASR, 4, false);
        assert_eq!(r, 0xF8000000);
        assert!(!c);

        let (r, c) = shift_c(0x80000000, ShiftType::ASR, 32, false);
        assert_eq!(r, 0xFFFFFFFF);
        assert!(c);
    }

    #[test]
    fn test_shift_ror() {
        let (r, c) = shift_c(0x12345678, ShiftType::ROR, 4, false);
        assert_eq!(r, 0x81234567);
        assert!(c); // Bit 31 of result
    }

    #[test]
    fn test_shift_rrx() {
        let (r, c) = shift_c(0x80000001, ShiftType::RRX, 1, true);
        assert_eq!(r, 0xC0000000); // Carry in -> bit 31, original bit 0 was 1
        assert!(c); // Original bit 0

        let (r, c) = shift_c(0x80000000, ShiftType::RRX, 1, false);
        assert_eq!(r, 0x40000000);
        assert!(!c);
    }

    #[test]
    fn test_expand_imm_c() {
        // No rotation
        let (r, c) = expand_imm_c(0x0FF, false);
        assert_eq!(r, 0xFF);
        assert!(!c);

        // Rotation by 8 (rot field = 4)
        let (r, c) = expand_imm_c(0x4FF, false);
        assert_eq!(r, 0xFF000000);
        assert!(c); // MSB is 1
    }

    #[test]
    fn test_condition_passed() {
        // EQ: Z=1
        assert!(condition_passed(0b0000, false, true, false, false));
        assert!(!condition_passed(0b0000, false, false, false, false));

        // NE: Z=0
        assert!(condition_passed(0b0001, false, false, false, false));
        assert!(!condition_passed(0b0001, false, true, false, false));

        // GE: N=V
        assert!(condition_passed(0b1010, true, false, false, true));
        assert!(condition_passed(0b1010, false, false, false, false));
        assert!(!condition_passed(0b1010, true, false, false, false));

        // GT: Z=0 && N=V
        assert!(condition_passed(0b1100, false, false, false, false));
        assert!(!condition_passed(0b1100, false, true, false, false)); // Z=1

        // AL: always
        assert!(condition_passed(0b1110, false, false, false, false));
    }

    #[test]
    fn test_sign_extend() {
        assert_eq!(sign_extend(0x7F, 8), 0x7F);
        assert_eq!(sign_extend(0x80, 8), 0xFFFFFF80);
        assert_eq!(sign_extend(0x7FFF, 16), 0x7FFF);
        assert_eq!(sign_extend(0x8000, 16), 0xFFFF8000);
    }

    #[test]
    fn test_byte_reverse() {
        assert_eq!(byte_reverse(0x12345678), 0x78563412);
        assert_eq!(byte_reverse_16(0x12345678), 0x34127856);
    }

    #[test]
    fn test_psr_roundtrip() {
        let original = 0xF00000D3u32; // NZCV set, SVC mode, I+F set
        let psr = Psr::from_u32(original);
        assert!(psr.n);
        assert!(psr.z);
        assert!(psr.c);
        assert!(psr.v);
        assert!(psr.i);
        assert!(psr.f);
        assert_eq!(psr.mode, 0x13);

        let reconstructed = psr.to_u32();
        assert_eq!(reconstructed, original);
    }
}
