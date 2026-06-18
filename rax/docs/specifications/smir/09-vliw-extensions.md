# SMIR VLIW and DSP Extensions

This document extends the core SMIR specification to support VLIW architectures (Hexagon, Itanium), DSP features, and RISC oddities.

## 1. VLIW Bundle Representation

VLIW architectures execute multiple operations in parallel as a "bundle" or "packet". SMIR must represent this explicitly.

### 1.1 Bundle Structure

```rust
/// A VLIW instruction bundle (packet)
#[derive(Clone, Debug)]
pub struct SmirBundle {
    /// Bundle identifier
    pub id: BundleId,
    
    /// Guest PC of bundle start
    pub guest_pc: GuestAddr,
    
    /// Operations in this bundle (execute in parallel)
    pub ops: Vec<SmirOp>,
    
    /// Bundle properties
    pub props: BundleProps,
}

/// Bundle identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BundleId(pub u32);

/// Bundle properties
#[derive(Clone, Debug, Default)]
pub struct BundleProps {
    /// Maximum slots available (architecture-dependent)
    pub max_slots: u8,
    
    /// Slots actually used
    pub slots_used: u8,
    
    /// Bundle is end of packet (Hexagon endloop marker)
    pub end_of_packet: bool,
    
    /// Bundle contains a control flow change
    pub has_control_flow: bool,
    
    /// Hardware loop iteration (Hexagon)
    pub loop_iteration: Option<u8>,
}
```

### 1.2 Parallel Semantics

Within a bundle, all operations conceptually read their source operands at the start of the bundle, then all write their results at the end. This enables:

```rust
/// Parallel execution semantics for bundles
pub fn execute_bundle(ctx: &mut SmirContext, bundle: &SmirBundle) -> Result<(), ExecError> {
    // Phase 1: Read all source operands
    let mut reads: Vec<(OpId, Vec<u64>)> = Vec::new();
    for op in &bundle.ops {
        let sources = read_all_sources(ctx, op);
        reads.push((op.id, sources));
    }
    
    // Phase 2: Compute all results
    let mut results: Vec<(VReg, u64)> = Vec::new();
    for (op, sources) in bundle.ops.iter().zip(reads.iter()) {
        if let Some((dst, val)) = compute_op(op, &sources.1) {
            results.push((dst, val));
        }
    }
    
    // Phase 3: Write all results (simultaneously)
    for (dst, val) in results {
        write_vreg(ctx, dst, val);
    }
    
    Ok(())
}
```

### 1.3 Value Forwarding (`.new` Semantics)

Hexagon allows using a value produced in the same packet:

```rust
/// Value forwarding within a bundle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueTiming {
    /// Use value from previous bundle (normal)
    Old,
    
    /// Use value produced in this bundle (.new)
    New,
    
    /// Use predicate from this bundle (predicate.new)
    PredicateNew,
}

/// Extended source operand with timing
#[derive(Clone, Debug)]
pub enum BundleSrcOperand {
    /// Normal source (value from before bundle)
    Normal(SrcOperand),
    
    /// Forwarded from producer in same bundle
    Forwarded {
        /// Which op in the bundle produces this value
        producer_idx: u8,
        /// Original source operand (fallback if producer doesn't execute)
        fallback: SrcOperand,
    },
    
    /// Predicate result from this bundle
    PredicateNew {
        /// Which op produces the predicate
        producer_idx: u8,
    },
}
```

### 1.4 Slot Constraints

```rust
/// Execution slot/unit type (architecture-specific)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecSlot {
    /// Any slot
    Any,
    
    /// Hexagon slots
    HexSlot0,
    HexSlot1,
    HexSlot2,
    HexSlot3,
    
    /// Itanium units
    ItaniumI,  // Integer
    ItaniumM,  // Memory
    ItaniumF,  // Floating-point
    ItaniumB,  // Branch
}

/// Slot constraint for an operation
#[derive(Clone, Debug)]
pub struct SlotConstraint {
    /// Required slot(s)
    pub required: Vec<ExecSlot>,
    
    /// Latency to next use (cycles)
    pub latency: u8,
    
    /// Resource usage (for conflict detection)
    pub resources: ResourceMask,
}

/// Resource usage bitmask
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceMask(u32);

impl ResourceMask {
    pub const NONE: ResourceMask = ResourceMask(0);
    pub const MEMORY: ResourceMask = ResourceMask(1 << 0);
    pub const BRANCH: ResourceMask = ResourceMask(1 << 1);
    pub const MULTIPLY: ResourceMask = ResourceMask(1 << 2);
    pub const PREDICATE: ResourceMask = ResourceMask(1 << 3);
    pub const NEWVALUE: ResourceMask = ResourceMask(1 << 4);
    
    pub fn conflicts(self, other: ResourceMask) -> bool {
        (self.0 & other.0) != 0
    }
}
```

## 2. Block Structure for VLIW

```rust
/// Extended block for VLIW architectures
#[derive(Clone, Debug)]
pub struct SmirVliwBlock {
    /// Block identifier
    pub id: BlockId,
    
    /// Guest PC at block entry
    pub guest_pc: GuestAddr,
    
    /// Bundles in this block (not individual ops)
    pub bundles: Vec<SmirBundle>,
    
    /// Block terminator (from last bundle)
    pub terminator: Terminator,
    
    /// Hardware loop info
    pub hw_loop: Option<HardwareLoop>,
}

/// Hardware loop (Hexagon, TI DSP, etc.)
#[derive(Clone, Debug)]
pub struct HardwareLoop {
    /// Loop ID (0 or 1 for Hexagon)
    pub id: u8,
    
    /// Loop start address
    pub start: GuestAddr,
    
    /// Loop end address (where loop0/loop1 is)
    pub end: GuestAddr,
    
    /// Initial count (if known statically)
    pub initial_count: Option<u32>,
    
    /// Count register
    pub count_reg: VReg,
    
    /// Start address register (for sploop)
    pub start_reg: Option<VReg>,
}
```

## 3. DSP Arithmetic Operations

### 3.1 Saturating Arithmetic

```rust
/// Saturation mode for arithmetic
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Saturation {
    /// No saturation (wrap on overflow)
    None,
    
    /// Saturate to signed min/max
    Signed,
    
    /// Saturate to unsigned max (0 on underflow)
    Unsigned,
    
    /// Saturate with sticky overflow flag
    StickyOverflow,
}

// New opcodes for DSP arithmetic
pub enum OpKind {
    // ... existing opcodes ...
    
    /// Saturating add: dst = sat(src1 + src2)
    AddSat {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        saturation: Saturation,
    },
    
    /// Saturating subtract: dst = sat(src1 - src2)
    SubSat {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        saturation: Saturation,
    },
    
    /// Saturating negate: dst = sat(-src)
    NegSat {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Saturating absolute: dst = sat(|src|)
    AbsSat {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Saturating shift left: dst = sat(src << amount)
    ShlSat {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        saturation: Saturation,
    },
    
    /// Rounding shift right: dst = (src + round) >> amount
    ShrRound {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        round_mode: RoundMode,
    },
}

/// Rounding mode for DSP operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundMode {
    /// Truncate toward zero
    Truncate,
    
    /// Round toward nearest, ties to even
    NearestEven,
    
    /// Round toward nearest, ties away from zero
    NearestAway,
    
    /// Round toward positive infinity
    Ceiling,
    
    /// Round toward negative infinity
    Floor,
    
    /// Convergent rounding (banker's rounding)
    Convergent,
}
```

### 3.2 Fixed-Point Types

```rust
/// Fixed-point format
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedPointFormat {
    /// Total bits (including sign)
    pub total_bits: u8,
    
    /// Fractional bits
    pub frac_bits: u8,
    
    /// Signed or unsigned
    pub signed: bool,
}

impl FixedPointFormat {
    /// Q15: signed 16-bit with 15 fractional bits (range: -1.0 to ~0.999969)
    pub const Q15: Self = Self { total_bits: 16, frac_bits: 15, signed: true };
    
    /// Q31: signed 32-bit with 31 fractional bits
    pub const Q31: Self = Self { total_bits: 32, frac_bits: 31, signed: true };
    
    /// Q1.15: signed 16-bit with 1 integer, 15 fractional
    pub const Q1_15: Self = Self { total_bits: 16, frac_bits: 15, signed: true };
    
    /// Q1.31: signed 32-bit with 1 integer, 31 fractional
    pub const Q1_31: Self = Self { total_bits: 32, frac_bits: 31, signed: true };
}

pub enum OpKind {
    // ... existing opcodes ...
    
    /// Fixed-point multiply: dst = (src1 * src2) >> frac_bits (with saturation)
    MulFixed {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        format: FixedPointFormat,
        saturation: Saturation,
        round: RoundMode,
    },
    
    /// Fixed-point multiply-accumulate: acc = sat(acc + (src1 * src2) >> frac_bits)
    MacFixed {
        acc: VReg,
        src1: VReg,
        src2: VReg,
        format: FixedPointFormat,
        saturation: Saturation,
    },
    
    /// Convert float to fixed-point
    FloatToFixed {
        dst: VReg,
        src: VReg,
        fp_precision: FpPrecision,
        fixed_format: FixedPointFormat,
        saturation: Saturation,
    },
    
    /// Convert fixed-point to float
    FixedToFloat {
        dst: VReg,
        src: VReg,
        fixed_format: FixedPointFormat,
        fp_precision: FpPrecision,
    },
}
```

### 3.3 Accumulator Registers

```rust
/// Extended-precision accumulator (40-bit, 72-bit, etc.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccumulatorWidth {
    /// 40-bit (common in 16-bit DSPs)
    A40,
    
    /// 72-bit (32-bit DSPs)
    A72,
    
    /// 80-bit (some audio DSPs)
    A80,
}

/// Accumulator register reference
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccReg {
    /// Generic accumulator (mapped per-arch)
    Acc(u8),
    
    /// Hexagon-specific: register pair as 64-bit
    HexagonPair(u8),  // R1:0, R3:2, etc.
}

pub enum OpKind {
    // ... existing opcodes ...
    
    /// Multiply-accumulate into extended accumulator
    MacAcc {
        acc: AccReg,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
        sub: bool,  // false = MAC, true = MSC (multiply-subtract)
    },
    
    /// Read accumulator to GPR (with shift and saturation)
    ReadAcc {
        dst: VReg,
        acc: AccReg,
        shift: i8,  // Positive = right shift, negative = left
        saturation: Saturation,
        width: OpWidth,
    },
    
    /// Write GPR to accumulator
    WriteAcc {
        acc: AccReg,
        src: VReg,
        width: AccumulatorWidth,
    },
    
    /// Clear accumulator
    ClearAcc {
        acc: AccReg,
    },
}
```

### 3.4 Circular Addressing

```rust
/// Circular buffer descriptor
#[derive(Clone, Debug)]
pub struct CircularBuffer {
    /// Base address
    pub base: VReg,
    
    /// Buffer length in bytes
    pub length: VReg,
    
    /// Current index
    pub index: VReg,
    
    /// Modifier (increment)
    pub modifier: i32,
}

/// Addressing mode extension for circular buffers
pub enum Address {
    // ... existing variants ...
    
    /// Circular buffer addressing: addr = base + (index % length)
    Circular {
        buffer: CircularBuffer,
        post_modify: bool,  // Update index after access
    },
    
    /// Bit-reversed addressing (for FFT)
    BitReversed {
        base: VReg,
        index: VReg,
        bits: u8,  // Number of bits to reverse
    },
}

pub enum OpKind {
    // ... existing opcodes ...
    
    /// Load with circular addressing
    LoadCircular {
        dst: VReg,
        buffer: CircularBuffer,
        width: MemWidth,
        sign: SignExtend,
    },
    
    /// Store with circular addressing
    StoreCircular {
        src: VReg,
        buffer: CircularBuffer,
        width: MemWidth,
    },
    
    /// Modify circular index without memory access
    ModifyCircular {
        buffer: CircularBuffer,
        offset: i32,
    },
}
```

## 4. RISC Delay Slots

### 4.1 Delay Slot Representation

```rust
/// Branch with delay slot(s)
#[derive(Clone, Debug)]
pub struct DelayedBranch {
    /// The branch instruction
    pub branch: Terminator,
    
    /// Operations in delay slot(s)
    pub delay_ops: Vec<SmirOp>,
    
    /// Number of delay slots (usually 1, but some archs have 2)
    pub num_slots: u8,
    
    /// Delay slot behavior
    pub behavior: DelaySlotBehavior,
}

/// Delay slot execution behavior
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DelaySlotBehavior {
    /// Always execute delay slot (MIPS, SPARC)
    Always,
    
    /// Execute only if branch taken (MIPS "likely" branches)
    OnlyIfTaken,
    
    /// Execute only if branch not taken
    OnlyIfNotTaken,
    
    /// Annulled (never execute)
    Annulled,
}

/// Extended terminator with delay slot support
#[derive(Clone, Debug)]
pub enum TerminatorExt {
    /// Standard terminator (no delay slot)
    Standard(Terminator),
    
    /// Terminator with delay slot
    Delayed(DelayedBranch),
}
```

### 4.2 Nullification and Annulment

```rust
/// SPARC-style annulment for conditional branches
pub enum OpKind {
    // ... existing opcodes ...
    
    /// Conditional annul: skip next instruction if condition false
    ConditionalAnnul {
        cond: Condition,
    },
}
```

## 5. Register Windows (SPARC)

```rust
/// Register window state
#[derive(Clone, Debug)]
pub struct RegisterWindow {
    /// Current Window Pointer
    pub cwp: u8,
    
    /// Number of windows
    pub num_windows: u8,
    
    /// Window Invalid Mask
    pub wim: u32,
    
    /// Saved window stack (for overflow handling)
    pub saved_windows: Vec<[u64; 16]>,
}

/// SPARC-specific register categories
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SparcReg {
    /// Global registers (shared across windows)
    Global(u8),  // g0-g7
    
    /// Output registers (o0-o7, become inputs in callee)
    Out(u8),
    
    /// Local registers (l0-l7, private to window)
    Local(u8),
    
    /// Input registers (i0-i7, from caller's outputs)
    In(u8),
}

pub enum OpKind {
    // ... existing opcodes ...
    
    /// Save: allocate new register window
    WindowSave {
        src1: VReg,
        src2: SrcOperand,
        dst: VReg,  // dst = src1 + src2 in new window
    },
    
    /// Restore: deallocate register window
    WindowRestore {
        src1: VReg,
        src2: SrcOperand,
        dst: VReg,
    },
    
    /// Flush register windows to memory
    FlushWindows,
}
```

## 6. Vector/SIMD Extensions for DSP

```rust
/// Extended vector operations for DSP
pub enum OpKind {
    // ... existing opcodes ...
    
    /// Vector dot product: dst = sum(src1[i] * src2[i])
    VDotProduct {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        elem: VecElementType,
        lanes: u8,
        acc: Option<VReg>,  // Accumulate into existing value
    },
    
    /// Vector sum of absolute differences (SAD)
    VSumAbsDiff {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector min/max with index
    VMinMaxIdx {
        dst_val: VReg,
        dst_idx: VReg,
        src: VReg,
        is_max: bool,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector complex multiply (for FFT)
    VComplexMul {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        conjugate: bool,
        elem: VecElementType,  // F32 or F16
    },
    
    /// Vector butterfly (FFT primitive)
    VButterfly {
        out_a: VReg,
        out_b: VReg,
        in_a: VReg,
        in_b: VReg,
        twiddle: VReg,
        elem: VecElementType,
    },
    
    /// Vector polynomial evaluation (Horner's method)
    VPolyEval {
        dst: VReg,
        x: VReg,
        coeffs: VReg,
        degree: u8,
        elem: VecElementType,
    },
}
```

## 7. Predication Extensions

### 7.1 Full Predication (Itanium, ARM with IT block)

```rust
/// Predication mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Predication {
    /// No predication (always execute)
    None,
    
    /// Execute if predicate is true
    IfTrue(PredicateReg),
    
    /// Execute if predicate is false
    IfFalse(PredicateReg),
}

/// Predicate register reference
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PredicateReg(pub u8);

/// Operation with predication
pub struct PredicatedOp {
    /// The operation
    pub op: SmirOp,
    
    /// Predication condition
    pub pred: Predication,
    
    /// Use .new predicate value (Hexagon)
    pub pred_new: bool,
}
```

### 7.2 Predicate Defines

```rust
pub enum OpKind {
    // ... existing opcodes ...
    
    /// Compare and set predicate
    CmpSetPred {
        pred: PredicateReg,
        src1: VReg,
        src2: SrcOperand,
        cond: Condition,
        width: OpWidth,
    },
    
    /// Logical operation on predicates
    PredLogic {
        dst: PredicateReg,
        src1: PredicateReg,
        src2: PredicateReg,
        op: LogicOp,  // And, Or, Xor, AndNot, etc.
    },
    
    /// Transfer predicate to GPR
    PredToGpr {
        dst: VReg,
        pred: PredicateReg,
    },
    
    /// Transfer GPR to predicate
    GprToPred {
        pred: PredicateReg,
        src: VReg,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicOp {
    And,
    Or,
    Xor,
    AndNot,
    OrNot,
    Not,
}
```

## 8. Updated Architecture-Specific State

```rust
/// Extended Hexagon state with VLIW support
pub struct HexagonRegStateExt {
    /// Basic state
    pub base: HexagonRegState,
    
    /// Predicate registers (4 1-bit predicates)
    pub predicates: [bool; 4],
    
    /// Loop count registers
    pub lc: [u32; 2],
    
    /// Loop start address registers
    pub sa: [u32; 2],
    
    /// User status register (for saturation flags, etc.)
    pub usr: u32,
    
    /// Global pointer
    pub gp: u32,
    
    /// Current packet address
    pub packet_pc: u32,
}

/// SPARC state with register windows
pub struct SparcRegState {
    /// Global registers (always visible)
    pub globals: [u64; 8],
    
    /// Register windows (circular)
    pub windows: Vec<SparcWindow>,
    
    /// Current Window Pointer
    pub cwp: u8,
    
    /// Window Invalid Mask
    pub wim: u32,
    
    /// Condition codes (icc, xcc for 64-bit)
    pub ccr: u8,
    
    /// Floating-point registers
    pub fpr: [u64; 32],
    
    /// FSR (Floating-point State Register)
    pub fsr: u64,
}

pub struct SparcWindow {
    pub ins: [u64; 8],
    pub locals: [u64; 8],
    pub outs: [u64; 8],
}
```

## 9. Lifter Extensions for VLIW

```rust
/// Extended lifter result for VLIW
pub struct VliwLiftResult {
    /// The bundle of parallel ops
    pub bundle: SmirBundle,
    
    /// Bytes consumed (packet size)
    pub bytes_consumed: usize,
    
    /// Control flow from this bundle
    pub control_flow: ControlFlow,
    
    /// Does this bundle end a hardware loop?
    pub ends_loop: Option<u8>,
}

/// VLIW-aware lifter trait
pub trait SmirVliwLifter: SmirLifter {
    /// Lift a complete VLIW packet
    fn lift_packet(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<VliwLiftResult, LiftError>;
    
    /// Detect packet boundaries
    fn find_packet_end(&self, bytes: &[u8]) -> usize;
    
    /// Check for hardware loop setup
    fn detect_hardware_loop(&self, bundle: &SmirBundle) -> Option<HardwareLoop>;
}
```

## 10. Interpreter Extensions

```rust
impl SmirInterpreter {
    /// Execute a VLIW bundle
    fn execute_bundle(&self, ctx: &mut SmirContext, bundle: &SmirBundle) -> Result<(), ExecError> {
        // Phase 1: Snapshot all source values
        let snapshots: Vec<_> = bundle.ops.iter()
            .map(|op| self.snapshot_sources(ctx, op))
            .collect();
        
        // Phase 2: Evaluate all predicates
        let predicates: Vec<bool> = bundle.ops.iter()
            .map(|op| self.eval_predicate(ctx, op))
            .collect();
        
        // Phase 3: Compute results (using snapshots, respecting predicates)
        let results: Vec<Option<(VReg, u64)>> = bundle.ops.iter()
            .zip(snapshots.iter())
            .zip(predicates.iter())
            .map(|((op, snap), pred)| {
                if *pred {
                    self.compute_op_from_snapshot(op, snap)
                } else {
                    None
                }
            })
            .collect();
        
        // Phase 4: Commit all results simultaneously
        for result in results.into_iter().flatten() {
            self.write_vreg(ctx, result.0, result.1);
        }
        
        Ok(())
    }
}
```

## Summary of Additions

| Category | New Features |
|----------|--------------|
| **VLIW** | `SmirBundle`, parallel execution, `.new` forwarding, slot constraints |
| **Saturating** | `AddSat`, `SubSat`, `NegSat`, `AbsSat`, `ShlSat`, `ShrRound` |
| **Fixed-Point** | `MulFixed`, `MacFixed`, `FixedPointFormat`, Q15/Q31 |
| **Accumulators** | `MacAcc`, `ReadAcc`, `WriteAcc`, 40/72/80-bit widths |
| **Circular** | `Circular` addressing, `LoadCircular`, `StoreCircular`, bit-reverse |
| **Delay Slots** | `DelayedBranch`, `DelaySlotBehavior`, annulment |
| **Reg Windows** | `WindowSave`, `WindowRestore`, `FlushWindows`, SPARC support |
| **DSP SIMD** | `VDotProduct`, `VSumAbsDiff`, `VComplexMul`, `VButterfly` |
| **Predication** | `PredicatedOp`, `CmpSetPred`, `PredLogic`, `.new` predicates |

This makes SMIR suitable for:
- **Hexagon** (VLIW + DSP + predicates)
- **Itanium** (VLIW + predication + register rotation)
- **TI C6000** (VLIW + DSP + circular addressing)
- **MIPS** (delay slots)
- **SPARC** (delay slots + register windows)
- **Blackfin** (DSP + circular + MAC)
- **ARM Cortex-M with DSP** (saturating + MAC)
