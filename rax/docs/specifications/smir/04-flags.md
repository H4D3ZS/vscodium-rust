# SMIR Flag Handling

This document specifies how SMIR represents and manipulates processor flags across architectures.

## 1. Flag Architecture Comparison

| Flag | x86 RFLAGS | ARM PSTATE | Hexagon USR | SMIR |
|------|------------|------------|-------------|------|
| Carry | CF (bit 0) | C (bit 29) | - | CF |
| Zero | ZF (bit 6) | Z (bit 30) | - | ZF |
| Sign/Negative | SF (bit 7) | N (bit 31) | - | SF |
| Overflow | OF (bit 11) | V (bit 28) | - | OF |
| Parity | PF (bit 2) | - | - | PF |
| Aux Carry | AF (bit 4) | - | - | AF |
| Direction | DF (bit 10) | - | - | DF |

## 2. Flag State Representation

SMIR uses a **hybrid lazy/eager** flag model for efficiency.

```rust
/// Complete flag state
pub struct FlagState {
    /// Current lazy state (if any)
    lazy: Option<LazyFlags>,
    
    /// Materialized flags (valid if lazy is None)
    materialized: MaterializedFlags,
    
    /// Which flags are currently valid in materialized
    valid: FlagSet,
}

/// Materialized (computed) flags
#[derive(Clone, Copy, Debug, Default)]
pub struct MaterializedFlags {
    pub cf: bool,  // Carry
    pub zf: bool,  // Zero
    pub sf: bool,  // Sign/Negative
    pub of: bool,  // Overflow
    pub pf: bool,  // Parity (x86)
    pub af: bool,  // Auxiliary carry (x86)
    pub df: bool,  // Direction (x86)
}

/// Lazy flag state (deferred computation)
#[derive(Clone, Copy, Debug)]
pub struct LazyFlags {
    /// Operation that produced these flags
    pub op: LazyFlagOp,
    
    /// Result of the operation
    pub result: u64,
    
    /// Left operand (for sub: minuend)
    pub left: u64,
    
    /// Right operand (for sub: subtrahend)
    pub right: u64,
    
    /// Operation width in bits
    pub width: OpWidth,
    
    /// Architecture hint (affects PF, AF computation)
    pub arch: SourceArch,
}

/// Lazy flag operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LazyFlagOp {
    /// No lazy state (use materialized)
    None,
    
    /// Addition: result = left + right
    Add,
    
    /// Subtraction: result = left - right
    Sub,
    
    /// Logical (AND, OR, XOR): result = op(left, right)
    /// Clears CF and OF; sets ZF, SF from result
    Logic,
    
    /// Increment: result = left + 1
    /// Preserves CF; updates ZF, SF, OF from result
    Inc,
    
    /// Decrement: result = left - 1
    /// Preserves CF; updates ZF, SF, OF from result
    Dec,
    
    /// Negate: result = -left (0 - left)
    Neg,
    
    /// Shift left: result = left << right
    Shl,
    
    /// Logical shift right: result = left >> right
    Shr,
    
    /// Arithmetic shift right: result = left >> right (sign-extended)
    Sar,
    
    /// Rotate (only CF and possibly OF are meaningful)
    Rotate,
    
    /// Multiply: high:low = left * right
    /// Only CF and OF meaningful (set if high != 0)
    Mul { high: u64 },
    
    /// Bit test: CF = (left >> right) & 1
    Bt,
    
    /// BCD operations (x86 DAA/DAS)
    Bcd { was_af: bool, was_cf: bool },
}
```

## 3. Flag Materialization

When flags are read (by conditional branches, CMOV, etc.), lazy flags must be materialized.

```rust
impl FlagState {
    /// Get the carry flag, materializing if needed
    pub fn get_cf(&mut self) -> bool {
        if let Some(lazy) = &self.lazy {
            self.materialize_cf(lazy)
        } else {
            self.materialized.cf
        }
    }
    
    /// Get the zero flag
    pub fn get_zf(&mut self) -> bool {
        if let Some(lazy) = &self.lazy {
            self.materialize_zf(lazy)
        } else {
            self.materialized.zf
        }
    }
    
    /// Materialize carry flag from lazy state
    fn materialize_cf(&self, lazy: &LazyFlags) -> bool {
        let width = lazy.width;
        match lazy.op {
            LazyFlagOp::Add => {
                // CF = carry out of MSB
                // Equivalent to: result < left || result < right
                let mask = width.mask();
                lazy.result & mask < lazy.left & mask
            }
            LazyFlagOp::Sub => {
                // CF = borrow (left < right)
                let mask = width.mask();
                (lazy.left & mask) < (lazy.right & mask)
            }
            LazyFlagOp::Logic => false,
            LazyFlagOp::Inc | LazyFlagOp::Dec => {
                // Preserve previous CF (should be in materialized)
                self.materialized.cf
            }
            LazyFlagOp::Neg => {
                // CF = (src != 0)
                lazy.left != 0
            }
            LazyFlagOp::Shl => {
                // CF = last bit shifted out
                if lazy.right == 0 {
                    self.materialized.cf
                } else {
                    let shift = lazy.right as u32;
                    let bits = width.bits();
                    if shift <= bits {
                        ((lazy.left >> (bits - shift)) & 1) != 0
                    } else {
                        false
                    }
                }
            }
            LazyFlagOp::Shr | LazyFlagOp::Sar => {
                // CF = last bit shifted out
                if lazy.right == 0 {
                    self.materialized.cf
                } else {
                    ((lazy.left >> (lazy.right - 1)) & 1) != 0
                }
            }
            LazyFlagOp::Rotate => {
                // CF = bit 0 of result (for ROR) or MSB (for ROL)
                (lazy.result & 1) != 0
            }
            LazyFlagOp::Mul { high } => {
                // CF = OF = high part is non-zero (or sign extension of low)
                high != 0
            }
            LazyFlagOp::Bt => {
                // CF = tested bit
                ((lazy.left >> (lazy.right & (width.bits() as u64 - 1))) & 1) != 0
            }
            LazyFlagOp::Bcd { was_cf, .. } => was_cf,
            LazyFlagOp::None => self.materialized.cf,
        }
    }
    
    /// Materialize zero flag
    fn materialize_zf(&self, lazy: &LazyFlags) -> bool {
        (lazy.result & lazy.width.mask()) == 0
    }
    
    /// Materialize sign flag
    fn materialize_sf(&self, lazy: &LazyFlags) -> bool {
        let sign_bit = 1u64 << (lazy.width.bits() - 1);
        (lazy.result & sign_bit) != 0
    }
    
    /// Materialize overflow flag
    fn materialize_of(&self, lazy: &LazyFlags) -> bool {
        let bits = lazy.width.bits();
        let sign_bit = 1u64 << (bits - 1);
        let mask = lazy.width.mask();
        
        match lazy.op {
            LazyFlagOp::Add => {
                // OF = sign(left) == sign(right) && sign(result) != sign(left)
                let left_sign = (lazy.left & sign_bit) != 0;
                let right_sign = (lazy.right & sign_bit) != 0;
                let result_sign = (lazy.result & sign_bit) != 0;
                left_sign == right_sign && result_sign != left_sign
            }
            LazyFlagOp::Sub => {
                // OF = sign(left) != sign(right) && sign(result) != sign(left)
                let left_sign = (lazy.left & sign_bit) != 0;
                let right_sign = (lazy.right & sign_bit) != 0;
                let result_sign = (lazy.result & sign_bit) != 0;
                left_sign != right_sign && result_sign != left_sign
            }
            LazyFlagOp::Logic => false,
            LazyFlagOp::Inc => {
                // OF = result == 0x80...0 (positive to negative overflow)
                (lazy.result & mask) == sign_bit
            }
            LazyFlagOp::Dec => {
                // OF = result == 0x7F...F (negative to positive overflow)
                (lazy.result & mask) == (sign_bit - 1)
            }
            LazyFlagOp::Neg => {
                // OF = src == MIN_SIGNED
                (lazy.left & mask) == sign_bit
            }
            LazyFlagOp::Shl => {
                if lazy.right == 1 {
                    // OF = CF XOR MSB of result
                    let cf = self.materialize_cf(lazy);
                    let msb = (lazy.result & sign_bit) != 0;
                    cf != msb
                } else {
                    // Undefined for count > 1
                    false
                }
            }
            LazyFlagOp::Shr => {
                if lazy.right == 1 {
                    // OF = MSB of original value
                    (lazy.left & sign_bit) != 0
                } else {
                    false
                }
            }
            LazyFlagOp::Sar => false, // Always 0 for SAR
            LazyFlagOp::Rotate => {
                // OF = MSB XOR CF (for count == 1)
                if lazy.right == 1 {
                    let cf = self.materialize_cf(lazy);
                    let msb = (lazy.result & sign_bit) != 0;
                    cf != msb
                } else {
                    false
                }
            }
            LazyFlagOp::Mul { high } => high != 0,
            LazyFlagOp::Bt => false,
            LazyFlagOp::Bcd { .. } => false,
            LazyFlagOp::None => self.materialized.of,
        }
    }
    
    /// Materialize parity flag (x86 only)
    fn materialize_pf(&self, lazy: &LazyFlags) -> bool {
        // PF = parity of low byte of result
        let byte = (lazy.result & 0xFF) as u8;
        byte.count_ones() % 2 == 0
    }
    
    /// Materialize auxiliary carry (x86 only)
    fn materialize_af(&self, lazy: &LazyFlags) -> bool {
        match lazy.op {
            LazyFlagOp::Add | LazyFlagOp::Sub => {
                // AF = carry from bit 3 to bit 4
                ((lazy.left ^ lazy.right ^ lazy.result) & 0x10) != 0
            }
            LazyFlagOp::Inc => {
                (lazy.result & 0x0F) == 0
            }
            LazyFlagOp::Dec => {
                (lazy.result & 0x0F) == 0x0F
            }
            LazyFlagOp::Logic => false,
            LazyFlagOp::Bcd { was_af, .. } => was_af,
            _ => false,
        }
    }
    
    /// Fully materialize all flags
    pub fn materialize_all(&mut self) {
        if let Some(lazy) = self.lazy.take() {
            self.materialized.cf = self.materialize_cf(&lazy);
            self.materialized.zf = self.materialize_zf(&lazy);
            self.materialized.sf = self.materialize_sf(&lazy);
            self.materialized.of = self.materialize_of(&lazy);
            if lazy.arch == SourceArch::X86_64 {
                self.materialized.pf = self.materialize_pf(&lazy);
                self.materialized.af = self.materialize_af(&lazy);
            }
        }
    }
}
```

## 4. Condition Evaluation

```rust
impl FlagState {
    /// Evaluate a condition code
    pub fn eval_condition(&mut self, cond: Condition) -> bool {
        match cond {
            Condition::Eq => self.get_zf(),
            Condition::Ne => !self.get_zf(),
            
            // Unsigned comparisons (after CMP: left - right)
            Condition::Ult => self.get_cf(),               // CF=1 (borrow)
            Condition::Uge => !self.get_cf(),              // CF=0 (no borrow)
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
}
```

## 5. ARM Flag Mapping

ARM PSTATE.NZCV maps to SMIR flags as follows:

| ARM | SMIR | Notes |
|-----|------|-------|
| N | SF | Identical semantics |
| Z | ZF | Identical semantics |
| C | CF | **Inverted for subtraction** |
| V | OF | Identical semantics |

```rust
/// Convert ARM condition to SMIR condition
fn arm_cond_to_smir(arm_cond: u8) -> Condition {
    match arm_cond {
        0b0000 => Condition::Eq,      // EQ: Z=1
        0b0001 => Condition::Ne,      // NE: Z=0
        0b0010 => Condition::Uge,     // CS/HS: C=1 (note: ARM C is inverted)
        0b0011 => Condition::Ult,     // CC/LO: C=0
        0b0100 => Condition::Negative,// MI: N=1
        0b0101 => Condition::Positive,// PL: N=0
        0b0110 => Condition::Overflow,// VS: V=1
        0b0111 => Condition::NoOverflow,// VC: V=0
        0b1000 => Condition::Ugt,     // HI: C=1 && Z=0
        0b1001 => Condition::Ule,     // LS: C=0 || Z=1
        0b1010 => Condition::Sge,     // GE: N==V
        0b1011 => Condition::Slt,     // LT: N!=V
        0b1100 => Condition::Sgt,     // GT: Z=0 && N==V
        0b1101 => Condition::Sle,     // LE: Z=1 || N!=V
        0b1110 | 0b1111 => Condition::Always, // AL/NV
        _ => unreachable!(),
    }
}

/// Handle ARM's inverted carry for subtraction
fn arm_sub_flags(left: u64, right: u64, result: u64, width: OpWidth) -> LazyFlags {
    // ARM sets C=1 when there is NO borrow (opposite of x86)
    // SMIR uses x86 convention (CF=1 means borrow)
    // Lifter must invert the carry interpretation
    LazyFlags {
        op: LazyFlagOp::Sub,
        result,
        left,
        right,
        width,
        arch: SourceArch::Aarch64,
    }
}
```

## 6. Flag Operations in IR

```rust
/// Operations that affect flags
impl OpKind {
    /// Get the flags this operation writes
    pub fn flags_written(&self) -> FlagSet {
        match self {
            OpKind::Add { flags, .. } |
            OpKind::Sub { flags, .. } |
            OpKind::Adc { flags, .. } |
            OpKind::Sbb { flags, .. } => {
                match flags {
                    FlagUpdate::None => FlagSet::EMPTY,
                    FlagUpdate::All => FlagSet::NZCV.union(FlagSet::PF).union(FlagSet::AF),
                    FlagUpdate::Specific(set) => *set,
                }
            }
            OpKind::Cmp { .. } | OpKind::Test { .. } => FlagSet::NZCV,
            OpKind::And { flags, .. } |
            OpKind::Or { flags, .. } |
            OpKind::Xor { flags, .. } => {
                match flags {
                    FlagUpdate::None => FlagSet::EMPTY,
                    FlagUpdate::All => FlagSet::ZF.union(FlagSet::SF).union(FlagSet::PF),
                    FlagUpdate::Specific(set) => *set,
                }
            }
            OpKind::Shl { flags, .. } |
            OpKind::Shr { flags, .. } |
            OpKind::Sar { flags, .. } => {
                match flags {
                    FlagUpdate::None => FlagSet::EMPTY,
                    FlagUpdate::All => FlagSet::CF.union(FlagSet::ZF).union(FlagSet::SF).union(FlagSet::OF),
                    FlagUpdate::Specific(set) => *set,
                }
            }
            OpKind::Bt { .. } => FlagSet::CF,
            OpKind::SetCF { .. } | OpKind::CmcCF => FlagSet::CF,
            OpKind::FCmp { .. } => FlagSet::NZCV, // FP compare sets integer flags
            _ => FlagSet::EMPTY,
        }
    }
    
    /// Get the flags this operation reads
    pub fn flags_read(&self) -> FlagSet {
        match self {
            OpKind::Adc { .. } | OpKind::Sbb { .. } => FlagSet::CF,
            OpKind::Rcl { .. } | OpKind::Rcr { .. } => FlagSet::CF,
            OpKind::CMove { cond, .. } | OpKind::SetCC { cond, .. } => {
                cond.required_flags()
            }
            OpKind::CmcCF => FlagSet::CF,
            _ => FlagSet::EMPTY,
        }
    }
}

impl Condition {
    /// Get flags required to evaluate this condition
    pub fn required_flags(&self) -> FlagSet {
        match self {
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
```

## 7. Flag Optimization

Dead flag elimination is one of the most important optimizations for x86 emulation.

```rust
/// Analyze flag usage in a block
pub fn analyze_flag_liveness(block: &SmirBlock) -> Vec<FlagSet> {
    let mut live_out = FlagSet::EMPTY;
    let mut liveness = vec![FlagSet::EMPTY; block.ops.len()];
    
    // Analyze terminator
    if let Terminator::CondBranch { cond, .. } = &block.terminator {
        live_out = cond.required_flags();
    }
    
    // Backward pass
    for (i, op) in block.ops.iter().enumerate().rev() {
        let reads = op.kind.flags_read();
        let writes = op.kind.flags_written();
        
        liveness[i] = live_out;
        
        // live_in = (live_out - writes) | reads
        live_out = (live_out.difference(writes)).union(reads);
    }
    
    liveness
}

/// Remove flag updates that are never read
pub fn eliminate_dead_flags(block: &mut SmirBlock) {
    let liveness = analyze_flag_liveness(block);
    
    for (i, op) in block.ops.iter_mut().enumerate() {
        let live = liveness[i];
        
        // If no flags are live after this op, remove flag updates
        if let Some(flags) = op.kind.flags_written_mut() {
            if live.is_empty() {
                *flags = FlagUpdate::None;
            } else {
                // Only update flags that are actually live
                *flags = FlagUpdate::Specific(flags.as_set().intersection(live));
            }
        }
    }
}
```

## 8. x86 RFLAGS Encoding

```rust
/// x86 RFLAGS bit positions
pub mod rflags {
    pub const CF: u64 = 1 << 0;   // Carry flag
    pub const PF: u64 = 1 << 2;   // Parity flag
    pub const AF: u64 = 1 << 4;   // Auxiliary carry
    pub const ZF: u64 = 1 << 6;   // Zero flag
    pub const SF: u64 = 1 << 7;   // Sign flag
    pub const TF: u64 = 1 << 8;   // Trap flag
    pub const IF: u64 = 1 << 9;   // Interrupt enable
    pub const DF: u64 = 1 << 10;  // Direction flag
    pub const OF: u64 = 1 << 11;  // Overflow flag
    pub const IOPL: u64 = 3 << 12; // I/O privilege level
    pub const NT: u64 = 1 << 14;  // Nested task
    pub const RF: u64 = 1 << 16;  // Resume flag
    pub const VM: u64 = 1 << 17;  // Virtual 8086 mode
    pub const AC: u64 = 1 << 18;  // Alignment check
    pub const VIF: u64 = 1 << 19; // Virtual interrupt flag
    pub const VIP: u64 = 1 << 20; // Virtual interrupt pending
    pub const ID: u64 = 1 << 21;  // ID flag
    
    /// Arithmetic flags mask
    pub const ARITH: u64 = CF | PF | AF | ZF | SF | OF;
}

impl MaterializedFlags {
    /// Convert to x86 RFLAGS value
    pub fn to_rflags(&self) -> u64 {
        let mut val = 0u64;
        if self.cf { val |= rflags::CF; }
        if self.pf { val |= rflags::PF; }
        if self.af { val |= rflags::AF; }
        if self.zf { val |= rflags::ZF; }
        if self.sf { val |= rflags::SF; }
        if self.of { val |= rflags::OF; }
        if self.df { val |= rflags::DF; }
        val | 0x02  // Bit 1 is always set
    }
    
    /// Load from x86 RFLAGS value
    pub fn from_rflags(rflags: u64) -> Self {
        MaterializedFlags {
            cf: (rflags & rflags::CF) != 0,
            pf: (rflags & rflags::PF) != 0,
            af: (rflags & rflags::AF) != 0,
            zf: (rflags & rflags::ZF) != 0,
            sf: (rflags & rflags::SF) != 0,
            of: (rflags & rflags::OF) != 0,
            df: (rflags & rflags::DF) != 0,
        }
    }
}
```

## 9. ARM NZCV Encoding

```rust
/// ARM PSTATE NZCV bit positions
pub mod pstate {
    pub const V: u64 = 1 << 28;  // Overflow
    pub const C: u64 = 1 << 29;  // Carry
    pub const Z: u64 = 1 << 30;  // Zero
    pub const N: u64 = 1 << 31;  // Negative
    
    pub const NZCV: u64 = N | Z | C | V;
}

impl MaterializedFlags {
    /// Convert to ARM NZCV value
    pub fn to_nzcv(&self) -> u64 {
        let mut val = 0u64;
        if self.sf { val |= pstate::N; }  // SF -> N
        if self.zf { val |= pstate::Z; }
        if !self.cf { val |= pstate::C; } // Inverted: SMIR CF=borrow, ARM C=no borrow
        if self.of { val |= pstate::V; }
        val
    }
    
    /// Load from ARM NZCV value
    pub fn from_nzcv(nzcv: u64) -> Self {
        MaterializedFlags {
            sf: (nzcv & pstate::N) != 0,
            zf: (nzcv & pstate::Z) != 0,
            cf: (nzcv & pstate::C) == 0,  // Inverted
            of: (nzcv & pstate::V) != 0,
            pf: false,
            af: false,
            df: false,
        }
    }
}
```
