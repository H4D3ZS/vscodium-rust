//! SMIR flag handling.
//!
//! This module implements lazy flag evaluation, which is critical for efficient
//! x86 emulation. Instead of computing all flags after every arithmetic operation,
//! we store the operands and operation type, computing flags only when they are read.

use crate::smir::types::{Condition, OpWidth, SourceArch};

// ============================================================================
// Flag Set
// ============================================================================

/// Set of flags
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct FlagSet(pub u8);

impl FlagSet {
    pub const EMPTY: FlagSet = FlagSet(0);
    pub const CF: FlagSet = FlagSet(1 << 0); // Carry
    pub const ZF: FlagSet = FlagSet(1 << 1); // Zero
    pub const SF: FlagSet = FlagSet(1 << 2); // Sign/Negative
    pub const OF: FlagSet = FlagSet(1 << 3); // Overflow
    pub const PF: FlagSet = FlagSet(1 << 4); // Parity (x86)
    pub const AF: FlagSet = FlagSet(1 << 5); // Auxiliary carry (x86)

    /// All arithmetic flags (NZCV equivalent)
    pub const NZCV: FlagSet = FlagSet(0x0F);
    /// All x86 arithmetic flags
    pub const ALL_X86: FlagSet = FlagSet(0x3F);

    /// Union of two flag sets
    pub const fn union(self, other: FlagSet) -> FlagSet {
        FlagSet(self.0 | other.0)
    }

    /// Intersection of two flag sets
    pub const fn intersection(self, other: FlagSet) -> FlagSet {
        FlagSet(self.0 & other.0)
    }

    /// Difference of two flag sets
    pub const fn difference(self, other: FlagSet) -> FlagSet {
        FlagSet(self.0 & !other.0)
    }

    /// Check if this set contains a flag
    pub const fn contains(self, flag: FlagSet) -> bool {
        (self.0 & flag.0) == flag.0
    }

    /// Check if empty
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// ============================================================================
// Flag Update Mode
// ============================================================================

/// Flag update mode for arithmetic operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FlagUpdate {
    /// Don't update flags
    #[default]
    None,
    /// Update all arithmetic flags
    All,
    /// Update specific flags only
    Specific(FlagSet),
}

impl FlagUpdate {
    /// Get the flag set that will be updated
    pub fn as_set(&self) -> FlagSet {
        match self {
            FlagUpdate::None => FlagSet::EMPTY,
            FlagUpdate::All => FlagSet::ALL_X86,
            FlagUpdate::Specific(set) => *set,
        }
    }

    /// Check if any flags are updated
    pub fn updates_any(&self) -> bool {
        !matches!(self, FlagUpdate::None)
    }
}

// ============================================================================
// Lazy Flag Operation
// ============================================================================

/// Lazy flag operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LazyFlagOp {
    /// No lazy state (use materialized)
    #[default]
    None,
    /// Addition: result = left + right
    Add,
    /// Subtraction: result = left - right
    Sub,
    /// Add with carry: result = left + right + carry_in (carry_in stored in `high`)
    Adc,
    /// Subtract with borrow: result = left - right - carry_in (carry_in in `high`)
    Sbb,
    /// Logical (AND, OR, XOR): clears CF and OF; sets ZF, SF from result
    Logic,
    /// Increment: preserves CF
    Inc,
    /// Decrement: preserves CF
    Dec,
    /// Negate: result = -left (0 - left)
    Neg,
    /// Shift left
    Shl,
    /// Logical shift right
    Shr,
    /// Arithmetic shift right
    Sar,
    /// Rotate left (ROL): CF = LSB of result
    Rotate,
    /// Rotate right (ROR): CF = MSB of result
    Ror,
    /// Rotate through carry left (RCL): CF is stored in `high`
    Rcl,
    /// Rotate through carry right (RCR): CF is stored in `high`
    Rcr,
    /// Double-precision shift left (SHLD): CF = last bit shifted out of dst's
    /// top. `left` = original dst, `right` = masked count.
    Shld,
    /// Double-precision shift right (SHRD): CF = last bit shifted out of dst's
    /// bottom. `left` = original dst, `right` = masked count.
    Shrd,
    /// Unsigned multiply: high:low = left * right; CF=OF iff high != 0
    Mul,
    /// Signed multiply (IMUL); CF=OF iff the full product is not the
    /// sign-extension of the low half. `result` = low half, `high` = high half.
    Imul,
    /// Bit test: CF = (left >> right) & 1
    Bt,
    /// Bit field extract: ZF = result == 0; CF=OF=0; SF/PF/AF undefined.
    Bextr,
    /// Zero high bits: CF = low8(index) >= width; ZF/SF from result; OF=0; PF/AF undefined.
    Bzhi,
}

// ============================================================================
// Lazy Flags
// ============================================================================

/// Lazy flag state (deferred computation)
#[derive(Clone, Copy, Debug, Default)]
pub struct LazyFlags {
    /// Operation that produced these flags
    pub op: LazyFlagOp,
    /// Result of the operation
    pub result: u64,
    /// Left operand
    pub left: u64,
    /// Right operand
    pub right: u64,
    /// Operation width
    pub width: OpWidth,
    /// High part for multiply
    pub high: u64,
}

impl LazyFlags {
    /// Create new lazy flags for an add operation
    pub fn add(left: u64, right: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Add,
            result,
            left,
            right,
            width,
            high: 0,
        }
    }

    /// Create new lazy flags for a sub operation
    pub fn sub(left: u64, right: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Sub,
            result,
            left,
            right,
            width,
            high: 0,
        }
    }

    /// Create new lazy flags for an add-with-carry (ADC) operation. `carry_in`
    /// (0/1) is kept in `high` so CF/AF/OF are computed from the ORIGINAL
    /// operands — folding it into `right` loses the carry-out and flips signs
    /// (the SMIR bug the smir_alu differential test caught).
    pub fn adc(left: u64, right: u64, carry_in: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Adc,
            result,
            left,
            right,
            width,
            high: carry_in & 1,
        }
    }

    /// Create new lazy flags for a subtract-with-borrow (SBB) operation.
    pub fn sbb(left: u64, right: u64, carry_in: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Sbb,
            result,
            left,
            right,
            width,
            high: carry_in & 1,
        }
    }

    /// Create new lazy flags for a logical operation
    pub fn logic(result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Logic,
            result,
            left: 0,
            right: 0,
            width,
            high: 0,
        }
    }

    /// Create new lazy flags for increment
    pub fn inc(left: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Inc,
            result,
            left,
            right: 1,
            width,
            high: 0,
        }
    }

    /// Create new lazy flags for decrement
    pub fn dec(left: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Dec,
            result,
            left,
            right: 1,
            width,
            high: 0,
        }
    }

    /// Create new lazy flags for BEXTR.
    pub fn bextr(result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Bextr,
            result,
            left: 0,
            right: 0,
            width,
            high: 0,
        }
    }

    /// Create new lazy flags for BZHI. `index` must already be truncated to the
    /// architectural low 8 bits used by the instruction.
    pub fn bzhi(index: u64, result: u64, width: OpWidth) -> Self {
        LazyFlags {
            op: LazyFlagOp::Bzhi,
            result,
            left: 0,
            right: index,
            width,
            high: 0,
        }
    }
}

// ============================================================================
// Materialized Flags
// ============================================================================

/// Materialized (computed) flags
#[derive(Clone, Copy, Debug, Default)]
pub struct MaterializedFlags {
    pub cf: bool, // Carry
    pub zf: bool, // Zero
    pub sf: bool, // Sign/Negative
    pub of: bool, // Overflow
    pub pf: bool, // Parity (x86)
    pub af: bool, // Auxiliary carry (x86)
    pub df: bool, // Direction (x86)
}

impl MaterializedFlags {
    /// Convert to x86 RFLAGS value
    pub fn to_rflags(&self) -> u64 {
        let mut val = 0x02u64; // Bit 1 is always set
        if self.cf {
            val |= 1 << 0;
        }
        if self.pf {
            val |= 1 << 2;
        }
        if self.af {
            val |= 1 << 4;
        }
        if self.zf {
            val |= 1 << 6;
        }
        if self.sf {
            val |= 1 << 7;
        }
        if self.df {
            val |= 1 << 10;
        }
        if self.of {
            val |= 1 << 11;
        }
        val
    }

    /// Load from x86 RFLAGS value
    pub fn from_rflags(rflags: u64) -> Self {
        MaterializedFlags {
            cf: (rflags & (1 << 0)) != 0,
            pf: (rflags & (1 << 2)) != 0,
            af: (rflags & (1 << 4)) != 0,
            zf: (rflags & (1 << 6)) != 0,
            sf: (rflags & (1 << 7)) != 0,
            df: (rflags & (1 << 10)) != 0,
            of: (rflags & (1 << 11)) != 0,
        }
    }

    /// Convert to ARM NZCV value (bits 31:28)
    pub fn to_nzcv(&self) -> u32 {
        let mut val = 0u32;
        if self.sf {
            val |= 1 << 31;
        } // N
        if self.zf {
            val |= 1 << 30;
        } // Z
        // Note: ARM C is inverted from x86 CF for subtraction
        if !self.cf {
            val |= 1 << 29;
        } // C (no borrow)
        if self.of {
            val |= 1 << 28;
        } // V
        val
    }

    /// Load from ARM NZCV value
    pub fn from_nzcv(nzcv: u32) -> Self {
        MaterializedFlags {
            sf: (nzcv & (1 << 31)) != 0, // N -> SF
            zf: (nzcv & (1 << 30)) != 0, // Z
            cf: (nzcv & (1 << 29)) == 0, // C -> inverted CF
            of: (nzcv & (1 << 28)) != 0, // V -> OF
            pf: false,
            af: false,
            df: false,
        }
    }
}

// ============================================================================
// Flag State
// ============================================================================

/// Complete flag state with lazy evaluation
#[derive(Clone, Debug, Default)]
pub struct FlagState {
    /// Current lazy state (if any)
    pub lazy: Option<LazyFlags>,
    /// Materialized flags (valid if lazy is None)
    pub materialized: MaterializedFlags,
}

impl FlagState {
    /// Create new flag state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set lazy flags from an add operation
    pub fn set_lazy_add(&mut self, left: u64, right: u64, result: u64, width: OpWidth) {
        self.lazy = Some(LazyFlags::add(left, right, result, width));
    }

    /// Set lazy flags from a sub operation
    pub fn set_lazy_sub(&mut self, left: u64, right: u64, result: u64, width: OpWidth) {
        self.lazy = Some(LazyFlags::sub(left, right, result, width));
    }

    /// Set lazy flags from an add-with-carry (ADC) operation
    pub fn set_lazy_adc(
        &mut self,
        left: u64,
        right: u64,
        carry_in: u64,
        result: u64,
        width: OpWidth,
    ) {
        self.lazy = Some(LazyFlags::adc(left, right, carry_in, result, width));
    }

    /// Set lazy flags from a subtract-with-borrow (SBB) operation
    pub fn set_lazy_sbb(
        &mut self,
        left: u64,
        right: u64,
        carry_in: u64,
        result: u64,
        width: OpWidth,
    ) {
        self.lazy = Some(LazyFlags::sbb(left, right, carry_in, result, width));
    }

    /// Set lazy flags from a logical operation
    pub fn set_lazy_logic(&mut self, result: u64, width: OpWidth) {
        self.lazy = Some(LazyFlags::logic(result, width));
    }

    /// Set lazy flags from a BEXTR operation.
    pub fn set_lazy_bextr(&mut self, result: u64, width: OpWidth) {
        self.lazy = Some(LazyFlags::bextr(result, width));
    }

    /// Set lazy flags from a BZHI operation.
    pub fn set_lazy_bzhi(&mut self, index: u64, result: u64, width: OpWidth) {
        self.lazy = Some(LazyFlags::bzhi(index & 0xff, result, width));
    }

    /// Get the carry flag, materializing if needed
    pub fn get_cf(&mut self) -> bool {
        if let Some(ref lazy) = self.lazy {
            self.materialize_cf(lazy)
        } else {
            self.materialized.cf
        }
    }

    /// Get the zero flag
    pub fn get_zf(&mut self) -> bool {
        if let Some(ref lazy) = self.lazy {
            self.materialize_zf(lazy)
        } else {
            self.materialized.zf
        }
    }

    /// Get the sign flag
    pub fn get_sf(&mut self) -> bool {
        if let Some(ref lazy) = self.lazy {
            self.materialize_sf(lazy)
        } else {
            self.materialized.sf
        }
    }

    /// Get the overflow flag
    pub fn get_of(&mut self) -> bool {
        if let Some(ref lazy) = self.lazy {
            self.materialize_of(lazy)
        } else {
            self.materialized.of
        }
    }

    /// Get the parity flag (x86 only)
    pub fn get_pf(&mut self) -> bool {
        if let Some(ref lazy) = self.lazy {
            self.materialize_pf(lazy)
        } else {
            self.materialized.pf
        }
    }

    /// Get the auxiliary carry flag (x86 only)
    pub fn get_af(&mut self) -> bool {
        if let Some(ref lazy) = self.lazy {
            self.materialize_af(lazy)
        } else {
            self.materialized.af
        }
    }

    /// Materialize carry flag from lazy state
    fn materialize_cf(&self, lazy: &LazyFlags) -> bool {
        let mask = lazy.width.mask();
        match lazy.op {
            LazyFlagOp::Add => {
                // CF = carry out of MSB
                (lazy.result & mask) < (lazy.left & mask)
            }
            LazyFlagOp::Sub => {
                // CF = borrow (left < right)
                (lazy.left & mask) < (lazy.right & mask)
            }
            LazyFlagOp::Adc => {
                // CF = true carry out of (left + right + carry_in) at width
                let l = (lazy.left & mask) as u128;
                let r = (lazy.right & mask) as u128;
                (l + r + lazy.high as u128) > mask as u128
            }
            LazyFlagOp::Sbb => {
                // CF = borrow: left < right + carry_in
                ((lazy.left & mask) as u128) < (lazy.right & mask) as u128 + lazy.high as u128
            }
            LazyFlagOp::Logic => false,
            LazyFlagOp::Inc | LazyFlagOp::Dec => {
                // Preserve previous CF
                self.materialized.cf
            }
            LazyFlagOp::Neg => {
                // CF = (src != 0), tested at the operand width (the stored
                // operand may carry stale upper bits).
                (lazy.left & mask) != 0
            }
            LazyFlagOp::Shl => {
                if lazy.right == 0 {
                    self.materialized.cf
                } else {
                    let shift = lazy.right as u32;
                    let bits = lazy.width.bits();
                    if shift <= bits {
                        ((lazy.left >> (bits - shift)) & 1) != 0
                    } else {
                        false
                    }
                }
            }
            LazyFlagOp::Shr | LazyFlagOp::Sar => {
                if lazy.right == 0 {
                    self.materialized.cf
                } else {
                    ((lazy.left >> (lazy.right - 1)) & 1) != 0
                }
            }
            LazyFlagOp::Rotate => (lazy.result & 1) != 0,
            // ROR: CF = the bit rotated out of the low end = MSB of the result.
            LazyFlagOp::Ror => (lazy.result & lazy.width.sign_bit()) != 0,
            LazyFlagOp::Rcl | LazyFlagOp::Rcr => (lazy.high & 1) != 0,
            // SHLD CF = last bit shifted out of the top of dst = bit (width-count).
            LazyFlagOp::Shld => {
                let bits = lazy.width.bits() as u64;
                (lazy.left >> (bits - lazy.right)) & 1 != 0
            }
            // SHRD CF = last bit shifted out of the bottom of dst = bit (count-1).
            LazyFlagOp::Shrd => (lazy.left >> (lazy.right - 1)) & 1 != 0,
            LazyFlagOp::Mul => lazy.high != 0,
            LazyFlagOp::Imul => {
                // CF set iff high half != sign-extension of the low half.
                let expected_hi = if (lazy.result & lazy.width.sign_bit()) != 0 {
                    mask
                } else {
                    0
                };
                lazy.high != expected_hi
            }
            LazyFlagOp::Bt => {
                let bit_pos = lazy.right & (lazy.width.bits() as u64 - 1);
                ((lazy.left >> bit_pos) & 1) != 0
            }
            LazyFlagOp::Bextr => false,
            LazyFlagOp::Bzhi => lazy.right >= u64::from(lazy.width.bits()),
            LazyFlagOp::None => self.materialized.cf,
        }
    }

    /// Materialize zero flag
    fn materialize_zf(&self, lazy: &LazyFlags) -> bool {
        match lazy.op {
            LazyFlagOp::Rotate | LazyFlagOp::Ror | LazyFlagOp::Rcl | LazyFlagOp::Rcr => {
                self.materialized.zf
            }
            _ => (lazy.result & lazy.width.mask()) == 0,
        }
    }

    /// Materialize sign flag
    fn materialize_sf(&self, lazy: &LazyFlags) -> bool {
        match lazy.op {
            LazyFlagOp::Rotate
            | LazyFlagOp::Ror
            | LazyFlagOp::Rcl
            | LazyFlagOp::Rcr
            | LazyFlagOp::Bextr => self.materialized.sf,
            _ => (lazy.result & lazy.width.sign_bit()) != 0,
        }
    }

    /// Materialize overflow flag
    fn materialize_of(&self, lazy: &LazyFlags) -> bool {
        let sign_bit = lazy.width.sign_bit();

        match lazy.op {
            LazyFlagOp::Add => {
                let left_sign = (lazy.left & sign_bit) != 0;
                let right_sign = (lazy.right & sign_bit) != 0;
                let result_sign = (lazy.result & sign_bit) != 0;
                left_sign == right_sign && result_sign != left_sign
            }
            LazyFlagOp::Sub => {
                let left_sign = (lazy.left & sign_bit) != 0;
                let right_sign = (lazy.right & sign_bit) != 0;
                let result_sign = (lazy.result & sign_bit) != 0;
                left_sign != right_sign && result_sign != left_sign
            }
            LazyFlagOp::Adc => {
                // Same signed-overflow rule as ADD; `result` already includes
                // the carry-in, so comparing original-operand signs is correct.
                let left_sign = (lazy.left & sign_bit) != 0;
                let right_sign = (lazy.right & sign_bit) != 0;
                let result_sign = (lazy.result & sign_bit) != 0;
                left_sign == right_sign && result_sign != left_sign
            }
            LazyFlagOp::Sbb => {
                let left_sign = (lazy.left & sign_bit) != 0;
                let right_sign = (lazy.right & sign_bit) != 0;
                let result_sign = (lazy.result & sign_bit) != 0;
                left_sign != right_sign && result_sign != left_sign
            }
            LazyFlagOp::Logic => false,
            LazyFlagOp::Inc => (lazy.result & lazy.width.mask()) == sign_bit,
            LazyFlagOp::Dec => (lazy.result & lazy.width.mask()) == (sign_bit - 1),
            LazyFlagOp::Neg => (lazy.left & lazy.width.mask()) == sign_bit,
            LazyFlagOp::Shl => {
                if lazy.right == 1 {
                    let cf = self.materialize_cf(lazy);
                    let msb = (lazy.result & sign_bit) != 0;
                    cf != msb
                } else {
                    false
                }
            }
            LazyFlagOp::Shr => {
                if lazy.right == 1 {
                    (lazy.left & sign_bit) != 0
                } else {
                    false
                }
            }
            LazyFlagOp::Sar => false, // Always 0 for SAR
            LazyFlagOp::Rotate => {
                if lazy.right == 1 {
                    let cf = self.materialize_cf(lazy);
                    let msb = (lazy.result & sign_bit) != 0;
                    cf != msb
                } else {
                    false
                }
            }
            LazyFlagOp::Ror => {
                // ROR by 1: OF = the two most-significant bits of the result differ.
                if lazy.right == 1 {
                    let msb = (lazy.result & sign_bit) != 0;
                    let second = (lazy.result & (sign_bit >> 1)) != 0;
                    msb != second
                } else {
                    false
                }
            }
            LazyFlagOp::Rcl => {
                if lazy.right == 1 {
                    let cf = self.materialize_cf(lazy);
                    let msb = (lazy.result & sign_bit) != 0;
                    cf != msb
                } else {
                    false
                }
            }
            LazyFlagOp::Rcr => {
                if lazy.right == 1 {
                    let msb = (lazy.result & sign_bit) != 0;
                    let second = (lazy.result & (sign_bit >> 1)) != 0;
                    msb != second
                } else {
                    false
                }
            }
            // SHLD/SHRD OF (count==1 only): set iff the sign bit of dst changed.
            LazyFlagOp::Shld | LazyFlagOp::Shrd => {
                if lazy.right == 1 {
                    ((lazy.result ^ lazy.left) & sign_bit) != 0
                } else {
                    false
                }
            }
            LazyFlagOp::Mul => lazy.high != 0,
            LazyFlagOp::Imul => {
                let expected_hi = if (lazy.result & sign_bit) != 0 {
                    lazy.width.mask()
                } else {
                    0
                };
                lazy.high != expected_hi
            }
            LazyFlagOp::Bt => false,
            LazyFlagOp::Bextr | LazyFlagOp::Bzhi => false,
            LazyFlagOp::None => self.materialized.of,
        }
    }

    /// Materialize parity flag (x86 only)
    fn materialize_pf(&self, lazy: &LazyFlags) -> bool {
        match lazy.op {
            LazyFlagOp::Rotate
            | LazyFlagOp::Ror
            | LazyFlagOp::Rcl
            | LazyFlagOp::Rcr
            | LazyFlagOp::Bextr
            | LazyFlagOp::Bzhi => self.materialized.pf,
            _ => {
                let byte = (lazy.result & 0xFF) as u8;
                byte.count_ones() % 2 == 0
            }
        }
    }

    /// Materialize auxiliary carry (x86 only)
    fn materialize_af(&self, lazy: &LazyFlags) -> bool {
        match lazy.op {
            LazyFlagOp::Add | LazyFlagOp::Sub => {
                ((lazy.left ^ lazy.right ^ lazy.result) & 0x10) != 0
            }
            // AF = nibble carry/borrow including the carry-in.
            LazyFlagOp::Adc => ((lazy.left & 0xF) + (lazy.right & 0xF) + lazy.high) > 0xF,
            LazyFlagOp::Sbb => (lazy.left & 0xF) < (lazy.right & 0xF) + lazy.high,
            LazyFlagOp::Inc => (lazy.result & 0x0F) == 0,
            LazyFlagOp::Dec => (lazy.result & 0x0F) == 0x0F,
            // NEG borrows from bit 4 iff the low nibble of the source is nonzero.
            LazyFlagOp::Neg => (lazy.left & 0x0F) != 0,
            LazyFlagOp::Logic => false,
            _ => self.materialized.af,
        }
    }

    /// Fully materialize all flags
    pub fn materialize_all(&mut self) {
        if let Some(lazy) = self.lazy.take() {
            self.materialized.cf = self.materialize_cf(&lazy);
            self.materialized.zf = self.materialize_zf(&lazy);
            self.materialized.sf = self.materialize_sf(&lazy);
            self.materialized.of = self.materialize_of(&lazy);
            self.materialized.pf = self.materialize_pf(&lazy);
            self.materialized.af = self.materialize_af(&lazy);
        }
    }

    /// Evaluate a condition code
    pub fn eval_condition(&mut self, cond: Condition) -> bool {
        match cond {
            Condition::Eq => self.get_zf(),
            Condition::Ne => !self.get_zf(),

            // Unsigned comparisons (after CMP: left - right)
            Condition::Ult => self.get_cf(),
            Condition::Uge => !self.get_cf(),
            Condition::Ule => self.get_cf() || self.get_zf(),
            Condition::Ugt => !self.get_cf() && !self.get_zf(),

            // Signed comparisons
            Condition::Slt => self.get_sf() != self.get_of(),
            Condition::Sge => self.get_sf() == self.get_of(),
            Condition::Sle => self.get_zf() || (self.get_sf() != self.get_of()),
            Condition::Sgt => !self.get_zf() && (self.get_sf() == self.get_of()),

            // Individual flags
            Condition::Negative => self.get_sf(),
            Condition::Positive => !self.get_sf(),
            Condition::Overflow => self.get_of(),
            Condition::NoOverflow => !self.get_of(),

            // x86 specific
            Condition::Parity => self.get_pf(),
            Condition::NoParity => !self.get_pf(),

            Condition::Always => true,
        }
    }

    /// Get flags required to evaluate a condition
    pub fn required_flags(cond: Condition) -> FlagSet {
        match cond {
            Condition::Eq | Condition::Ne => FlagSet::ZF,
            Condition::Ult | Condition::Uge => FlagSet::CF,
            Condition::Ule | Condition::Ugt => FlagSet::CF.union(FlagSet::ZF),
            Condition::Slt | Condition::Sge => FlagSet::SF.union(FlagSet::OF),
            Condition::Sle | Condition::Sgt => FlagSet::ZF.union(FlagSet::SF).union(FlagSet::OF),
            Condition::Negative | Condition::Positive => FlagSet::SF,
            Condition::Overflow | Condition::NoOverflow => FlagSet::OF,
            Condition::Parity | Condition::NoParity => FlagSet::PF,
            Condition::Always => FlagSet::EMPTY,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_flags() {
        let mut flags = FlagState::new();

        // 0x7FFFFFFF + 1 = 0x80000000 (signed overflow)
        flags.set_lazy_add(0x7FFF_FFFF, 1, 0x8000_0000, OpWidth::W32);

        assert!(!flags.get_cf()); // No carry
        assert!(!flags.get_zf()); // Not zero
        assert!(flags.get_sf()); // Negative result
        assert!(flags.get_of()); // Signed overflow
    }

    #[test]
    fn test_sub_flags() {
        let mut flags = FlagState::new();

        // 5 - 10 = -5 (with borrow)
        let result = 5u64.wrapping_sub(10) & 0xFFFF_FFFF;
        flags.set_lazy_sub(5, 10, result, OpWidth::W32);

        assert!(flags.get_cf()); // Borrow occurred
        assert!(!flags.get_zf()); // Not zero
        assert!(flags.get_sf()); // Negative result
        assert!(!flags.get_of()); // No signed overflow
    }

    #[test]
    fn test_logic_flags() {
        let mut flags = FlagState::new();

        // 0xFF00 & 0x00FF = 0
        flags.set_lazy_logic(0, OpWidth::W32);

        assert!(!flags.get_cf()); // Cleared
        assert!(flags.get_zf()); // Zero
        assert!(!flags.get_sf()); // Not negative
        assert!(!flags.get_of()); // Cleared
    }

    #[test]
    fn test_condition_eval() {
        let mut flags = FlagState::new();

        // Set up: 10 - 5 = 5
        flags.set_lazy_sub(10, 5, 5, OpWidth::W32);

        assert!(!flags.eval_condition(Condition::Eq)); // Not equal
        assert!(flags.eval_condition(Condition::Ne)); // Not equal
        assert!(!flags.eval_condition(Condition::Ult)); // 10 >= 5 unsigned
        assert!(flags.eval_condition(Condition::Ugt)); // 10 > 5 unsigned
        assert!(flags.eval_condition(Condition::Sgt)); // 10 > 5 signed
    }

    #[test]
    fn test_rflags_conversion() {
        let flags = MaterializedFlags {
            cf: true,
            zf: true,
            sf: false,
            of: false,
            pf: true,
            af: false,
            df: false,
        };

        let rflags = flags.to_rflags();
        let restored = MaterializedFlags::from_rflags(rflags);

        assert_eq!(flags.cf, restored.cf);
        assert_eq!(flags.zf, restored.zf);
        assert_eq!(flags.sf, restored.sf);
        assert_eq!(flags.of, restored.of);
        assert_eq!(flags.pf, restored.pf);
    }
}
