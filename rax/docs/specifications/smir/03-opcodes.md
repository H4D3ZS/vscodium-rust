# SMIR Opcode Catalog

This document specifies all SMIR operations, their semantics, and execution behavior.

## 1. Operation Structure

```rust
/// A single SMIR operation
#[derive(Clone, Debug)]
pub struct SmirOp {
    /// Unique operation ID within the block
    pub id: OpId,
    
    /// Guest PC this operation corresponds to
    pub guest_pc: GuestAddr,
    
    /// The operation kind and operands
    pub kind: OpKind,
}

/// Operation ID (block-local)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpId(pub u16);
```

## 2. Complete OpKind Enumeration

```rust
/// All SMIR operation kinds
#[derive(Clone, Debug)]
pub enum OpKind {
    // ========================================================================
    // INTEGER ARITHMETIC
    // ========================================================================
    
    /// Integer addition: dst = src1 + src2
    Add {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Integer subtraction: dst = src1 - src2
    Sub {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Add with carry: dst = src1 + src2 + CF
    Adc {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Subtract with borrow: dst = src1 - src2 - CF
    Sbb {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Negate: dst = -src (two's complement)
    Neg {
        dst: VReg,
        src: VReg,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Compare (subtract without storing): flags = src1 - src2
    Cmp {
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
    },
    
    /// Unsigned multiply: (dst_hi, dst_lo) = src1 * src2
    MulU {
        dst_lo: VReg,
        dst_hi: Option<VReg>,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Signed multiply: (dst_hi, dst_lo) = src1 * src2
    MulS {
        dst_lo: VReg,
        dst_hi: Option<VReg>,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Multiply-add: dst = acc + (src1 * src2)
    MulAdd {
        dst: VReg,
        acc: VReg,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
    },
    
    /// Multiply-sub: dst = acc - (src1 * src2)
    MulSub {
        dst: VReg,
        acc: VReg,
        src1: VReg,
        src2: VReg,
        width: OpWidth,
    },
    
    /// Unsigned divide: (quotient, remainder) = src1 / src2
    DivU {
        quot: VReg,
        rem: Option<VReg>,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
    },
    
    /// Signed divide: (quotient, remainder) = src1 / src2
    DivS {
        quot: VReg,
        rem: Option<VReg>,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
    },
    
    // ========================================================================
    // BITWISE LOGICAL
    // ========================================================================
    
    /// Bitwise AND: dst = src1 & src2
    And {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Bitwise OR: dst = src1 | src2
    Or {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Bitwise XOR: dst = src1 ^ src2
    Xor {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Bitwise NOT: dst = ~src
    Not {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Test (AND without storing): flags = src1 & src2
    Test {
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
    },
    
    /// AND-NOT: dst = src1 & ~src2 (BMI1/ARM BIC)
    AndNot {
        dst: VReg,
        src1: VReg,
        src2: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    // ========================================================================
    // SHIFTS AND ROTATES
    // ========================================================================
    
    /// Logical shift left: dst = src << amount
    Shl {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Logical shift right: dst = src >> amount (zero-fill)
    Shr {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Arithmetic shift right: dst = src >> amount (sign-fill)
    Sar {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Rotate left: dst = (src << amount) | (src >> (width - amount))
    Rol {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Rotate right: dst = (src >> amount) | (src << (width - amount))
    Ror {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Rotate left through carry (x86 RCL)
    Rcl {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Rotate right through carry (x86 RCR)
    Rcr {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Double-precision shift left (x86 SHLD)
    Shld {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Double-precision shift right (x86 SHRD)
    Shrd {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        amount: SrcOperand,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    // ========================================================================
    // BIT MANIPULATION
    // ========================================================================
    
    /// Bit test: CF = bit(src, index)
    Bt {
        src: VReg,
        index: SrcOperand,
        width: OpWidth,
    },
    
    /// Bit test and set: CF = bit(src, index); bit(dst, index) = 1
    Bts {
        dst: VReg,
        src: VReg,
        index: SrcOperand,
        width: OpWidth,
    },
    
    /// Bit test and reset: CF = bit(src, index); bit(dst, index) = 0
    Btr {
        dst: VReg,
        src: VReg,
        index: SrcOperand,
        width: OpWidth,
    },
    
    /// Bit test and complement: CF = bit(src, index); bit(dst, index) ^= 1
    Btc {
        dst: VReg,
        src: VReg,
        index: SrcOperand,
        width: OpWidth,
    },
    
    /// Bit scan forward (find lowest set bit): dst = LSB(src)
    Bsf {
        dst: VReg,
        src: VReg,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Bit scan reverse (find highest set bit): dst = MSB(src)
    Bsr {
        dst: VReg,
        src: VReg,
        width: OpWidth,
        flags: FlagUpdate,
    },
    
    /// Count leading zeros
    Clz {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Count trailing zeros
    Ctz {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Population count (count set bits)
    Popcnt {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Byte swap (endian conversion)
    Bswap {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Bit reverse (ARM RBIT)
    Rbit {
        dst: VReg,
        src: VReg,
        width: OpWidth,
    },
    
    /// Extract bit field
    Bfx {
        dst: VReg,
        src: VReg,
        lsb: u8,
        width_bits: u8,
        sign_extend: bool,
        op_width: OpWidth,
    },
    
    /// Insert bit field
    Bfi {
        dst: VReg,
        dst_in: VReg,
        src: VReg,
        lsb: u8,
        width_bits: u8,
        op_width: OpWidth,
    },
    
    /// Deposit bits (x86 PDEP)
    Pdep {
        dst: VReg,
        src: VReg,
        mask: VReg,
        width: OpWidth,
    },
    
    /// Extract bits (x86 PEXT)
    Pext {
        dst: VReg,
        src: VReg,
        mask: VReg,
        width: OpWidth,
    },
    
    // ========================================================================
    // DATA MOVEMENT
    // ========================================================================
    
    /// Register-to-register move
    Mov {
        dst: VReg,
        src: SrcOperand,
        width: OpWidth,
    },
    
    /// Conditional move: dst = cond ? src : dst
    CMove {
        dst: VReg,
        src: VReg,
        cond: Condition,
        width: OpWidth,
    },
    
    /// Select: dst = cond ? src_true : src_false
    Select {
        dst: VReg,
        cond: VReg,
        src_true: VReg,
        src_false: VReg,
        width: OpWidth,
    },
    
    /// Zero-extend: dst = zext(src)
    ZeroExtend {
        dst: VReg,
        src: VReg,
        from_width: OpWidth,
        to_width: OpWidth,
    },
    
    /// Sign-extend: dst = sext(src)
    SignExtend {
        dst: VReg,
        src: VReg,
        from_width: OpWidth,
        to_width: OpWidth,
    },
    
    /// Truncate: dst = trunc(src)
    Truncate {
        dst: VReg,
        src: VReg,
        from_width: OpWidth,
        to_width: OpWidth,
    },
    
    /// Load effective address (compute address without memory access)
    Lea {
        dst: VReg,
        addr: Address,
    },
    
    /// Exchange registers: tmp = a; a = b; b = tmp
    Xchg {
        reg1: VReg,
        reg2: VReg,
        width: OpWidth,
    },
    
    // ========================================================================
    // MEMORY OPERATIONS
    // ========================================================================
    
    /// Load from memory
    Load {
        dst: VReg,
        addr: Address,
        width: MemWidth,
        sign: SignExtend,
    },
    
    /// Store to memory
    Store {
        src: VReg,
        addr: Address,
        width: MemWidth,
    },
    
    /// Load pair (ARM LDP)
    LoadPair {
        dst1: VReg,
        dst2: VReg,
        addr: Address,
        width: MemWidth,
    },
    
    /// Store pair (ARM STP)
    StorePair {
        src1: VReg,
        src2: VReg,
        addr: Address,
        width: MemWidth,
    },
    
    /// Atomic load
    AtomicLoad {
        dst: VReg,
        addr: Address,
        width: MemWidth,
        order: MemoryOrder,
    },
    
    /// Atomic store
    AtomicStore {
        src: VReg,
        addr: Address,
        width: MemWidth,
        order: MemoryOrder,
    },
    
    /// Atomic read-modify-write
    AtomicRmw {
        dst: VReg,
        addr: Address,
        src: VReg,
        op: AtomicOp,
        width: MemWidth,
        order: MemoryOrder,
    },
    
    /// Compare-and-swap
    Cas {
        dst: VReg,           // Old value loaded
        success: VReg,       // 1 if succeeded, 0 otherwise
        addr: Address,
        expected: VReg,
        new_val: VReg,
        width: MemWidth,
        order: MemoryOrder,
    },
    
    /// Load-exclusive (ARM LDXR)
    LoadExclusive {
        dst: VReg,
        addr: Address,
        width: MemWidth,
    },
    
    /// Store-exclusive (ARM STXR)
    StoreExclusive {
        status: VReg,        // 0 = success, 1 = failed
        src: VReg,
        addr: Address,
        width: MemWidth,
    },
    
    /// Clear exclusive monitor
    ClearExclusive,
    
    /// Prefetch hint
    Prefetch {
        addr: Address,
        locality: PrefetchLocality,
        write: bool,
    },
    
    /// Memory fence
    Fence {
        kind: FenceKind,
    },
    
    // ========================================================================
    // FLOATING POINT
    // ========================================================================
    
    /// FP add: dst = src1 + src2
    FAdd {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP subtract: dst = src1 - src2
    FSub {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP multiply: dst = src1 * src2
    FMul {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP divide: dst = src1 / src2
    FDiv {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP fused multiply-add: dst = (src1 * src2) + src3
    FFma {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        src3: VReg,
        precision: FpPrecision,
    },
    
    /// FP fused multiply-sub: dst = (src1 * src2) - src3
    FFms {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        src3: VReg,
        precision: FpPrecision,
    },
    
    /// FP absolute value: dst = |src|
    FAbs {
        dst: VReg,
        src: VReg,
        precision: FpPrecision,
    },
    
    /// FP negate: dst = -src
    FNeg {
        dst: VReg,
        src: VReg,
        precision: FpPrecision,
    },
    
    /// FP square root: dst = sqrt(src)
    FSqrt {
        dst: VReg,
        src: VReg,
        precision: FpPrecision,
    },
    
    /// FP minimum: dst = min(src1, src2)
    FMin {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP maximum: dst = max(src1, src2)
    FMax {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP compare: flags = compare(src1, src2)
    FCmp {
        src1: VReg,
        src2: VReg,
        precision: FpPrecision,
    },
    
    /// FP convert precision: dst = convert(src)
    FConvert {
        dst: VReg,
        src: VReg,
        from: FpPrecision,
        to: FpPrecision,
    },
    
    /// Convert int to float
    IntToFp {
        dst: VReg,
        src: VReg,
        int_width: OpWidth,
        fp_precision: FpPrecision,
        signed: bool,
    },
    
    /// Convert float to int
    FpToInt {
        dst: VReg,
        src: VReg,
        fp_precision: FpPrecision,
        int_width: OpWidth,
        signed: bool,
        round: FpRoundMode,
    },
    
    /// Round to integral value
    FRound {
        dst: VReg,
        src: VReg,
        precision: FpPrecision,
        mode: FpRoundMode,
    },
    
    // ========================================================================
    // SIMD / VECTOR OPERATIONS
    // ========================================================================
    
    /// Vector add (all lanes)
    VAdd {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector subtract (all lanes)
    VSub {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector multiply (all lanes)
    VMul {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector bitwise AND
    VAnd {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        width: VecWidth,
    },
    
    /// Vector bitwise OR
    VOr {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        width: VecWidth,
    },
    
    /// Vector bitwise XOR
    VXor {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        width: VecWidth,
    },
    
    /// Vector shift (all lanes)
    VShift {
        dst: VReg,
        src: VReg,
        amount: SrcOperand,
        shift: ShiftOp,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector compare (all lanes)
    VCmp {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        cond: VecCmpCond,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Vector move
    VMov {
        dst: VReg,
        src: VReg,
        width: VecWidth,
    },
    
    /// Insert scalar into vector lane
    VInsertLane {
        dst: VReg,
        vec: VReg,
        scalar: VReg,
        lane: u8,
        elem: VecElementType,
    },
    
    /// Extract scalar from vector lane
    VExtractLane {
        dst: VReg,
        vec: VReg,
        lane: u8,
        elem: VecElementType,
        sign: SignExtend,
    },
    
    /// Vector shuffle/permute
    VShuffle {
        dst: VReg,
        src1: VReg,
        src2: Option<VReg>,
        indices: VReg,
        elem: VecElementType,
    },
    
    /// Vector load
    VLoad {
        dst: VReg,
        addr: Address,
        width: VecWidth,
    },
    
    /// Vector store
    VStore {
        src: VReg,
        addr: Address,
        width: VecWidth,
    },
    
    /// Horizontal add (add adjacent pairs)
    VHAdd {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Broadcast scalar to all lanes
    VBroadcast {
        dst: VReg,
        scalar: VReg,
        elem: VecElementType,
        lanes: u8,
    },
    
    /// Blend (select lanes based on mask)
    VBlend {
        dst: VReg,
        src1: VReg,
        src2: VReg,
        mask: VReg,
        width: VecWidth,
    },
    
    // ========================================================================
    // FLAG OPERATIONS
    // ========================================================================
    
    /// Read flags to register
    ReadFlags {
        dst: VReg,
    },
    
    /// Write register to flags
    WriteFlags {
        src: VReg,
    },
    
    /// Set carry flag
    SetCF {
        value: bool,
    },
    
    /// Complement carry flag
    CmcCF,
    
    /// Compute flags from lazy state (force materialization)
    MaterializeFlags,
    
    /// Set flags from condition test
    TestCondition {
        dst: VReg,
        cond: Condition,
    },
    
    // ========================================================================
    // CONTROL FLOW (in-block, not terminators)
    // ========================================================================
    
    /// Conditional set: dst = cond ? 1 : 0
    SetCC {
        dst: VReg,
        cond: Condition,
        width: OpWidth,
    },
    
    /// Increment counter and branch if non-zero (loop)
    LoopDecBranch {
        counter: VReg,
        target: BlockId,
    },
    
    // ========================================================================
    // SYSTEM / PRIVILEGED
    // ========================================================================
    
    /// System call
    Syscall {
        num: VReg,
        args: Vec<VReg>,
    },
    
    /// Software interrupt
    Swi {
        imm: u32,
    },
    
    /// Read system register
    ReadSysReg {
        dst: VReg,
        reg: SysRegId,
    },
    
    /// Write system register
    WriteSysReg {
        reg: SysRegId,
        src: VReg,
    },
    
    /// Invalidate TLB entry
    TlbInvalidate {
        addr: Option<VReg>,
        asid: Option<VReg>,
    },
    
    /// Cache operation
    CacheOp {
        op: CacheOpKind,
        addr: VReg,
    },
    
    // ========================================================================
    // CRYPTO (optional extension)
    // ========================================================================
    
    /// AES single round encrypt
    AesEnc {
        dst: VReg,
        src: VReg,
        key: VReg,
    },
    
    /// AES single round decrypt
    AesDec {
        dst: VReg,
        src: VReg,
        key: VReg,
    },
    
    /// AES last round encrypt
    AesEncLast {
        dst: VReg,
        src: VReg,
        key: VReg,
    },
    
    /// AES last round decrypt
    AesDecLast {
        dst: VReg,
        src: VReg,
        key: VReg,
    },
    
    /// AES inverse mix columns
    AesImc {
        dst: VReg,
        src: VReg,
    },
    
    /// AES key generation assist
    AesKeyGen {
        dst: VReg,
        src: VReg,
        rcon: u8,
    },
    
    /// SHA256 round
    Sha256Round {
        dst: VReg,
        src1: VReg,
        src2: VReg,
    },
    
    /// CRC32 accumulate
    Crc32 {
        dst: VReg,
        crc: VReg,
        data: VReg,
        polynomial: Crc32Poly,
        width: OpWidth,
    },
    
    // ========================================================================
    // ARCHITECTURE-SPECIFIC ESCAPES
    // ========================================================================
    
    /// x86-specific operation (not easily abstractable)
    X86Specific(X86SpecificOp),
    
    /// ARM-specific operation
    ArmSpecific(ArmSpecificOp),
    
    /// Hexagon-specific operation
    HexagonSpecific(HexagonSpecificOp),
    
    // ========================================================================
    // META / DEBUG
    // ========================================================================
    
    /// No-op (placeholder, debugging)
    Nop,
    
    /// Debug print (interpreter only)
    DebugPrint {
        msg: String,
        values: Vec<VReg>,
    },
    
    /// Undefined instruction (trap on execution)
    Undefined {
        opcode: u32,
    },
}
```

## 3. Supporting Types for Operations

```rust
/// Atomic read-modify-write operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomicOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Nand,
    Max,   // Signed maximum
    Min,   // Signed minimum
    Umax,  // Unsigned maximum
    Umin,  // Unsigned minimum
    Swap,  // Exchange
}

/// Prefetch locality hint
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrefetchLocality {
    /// Non-temporal (streaming, don't pollute cache)
    NonTemporal,
    /// Low locality (L3 cache)
    Low,
    /// Medium locality (L2 cache)
    Medium,
    /// High locality (L1 cache)
    High,
}

/// Fence kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FenceKind {
    /// Load-load ordering
    LoadLoad,
    /// Load-store ordering
    LoadStore,
    /// Store-load ordering
    StoreLoad,
    /// Store-store ordering
    StoreStore,
    /// Full fence (all of the above)
    Full,
    /// Instruction serialization
    ISync,
}

/// FP precision
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpPrecision {
    F16,  // IEEE 754 half
    F32,  // IEEE 754 single
    F64,  // IEEE 754 double
    F80,  // x87 extended (x86 only)
}

/// FP rounding mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpRoundMode {
    /// Round to nearest, ties to even
    RoundNearest,
    /// Round toward zero (truncate)
    RoundTowardZero,
    /// Round toward positive infinity
    RoundUp,
    /// Round toward negative infinity
    RoundDown,
    /// Use current rounding mode (MXCSR/FPCR)
    Dynamic,
}

/// Vector comparison condition
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VecCmpCond {
    Eq,
    Ne,
    Lt,   // Signed
    Le,
    Gt,
    Ge,
    Ltu,  // Unsigned
    Leu,
    Gtu,
    Geu,
}

/// Vector register width
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VecWidth {
    V64,   // 64-bit (D registers on ARM)
    V128,  // 128-bit (XMM, Q registers)
    V256,  // 256-bit (YMM)
    V512,  // 512-bit (ZMM)
}

/// System register identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SysRegId {
    /// Architecture-specific encoding
    pub encoding: u32,
    /// Source architecture
    pub arch: SourceArch,
}

/// Cache operation kind
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheOpKind {
    /// Invalidate cache line
    Invalidate,
    /// Clean (write back) cache line
    Clean,
    /// Clean and invalidate
    CleanInvalidate,
    /// Zero cache line (ARM DC ZVA)
    Zero,
}

/// CRC32 polynomial
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crc32Poly {
    /// IEEE 802.3 (Ethernet)
    Ieee,
    /// Castagnoli (iSCSI)
    C,
}
```

## 4. Architecture-Specific Escapes

```rust
/// x86-specific operations not easily abstracted
#[derive(Clone, Debug)]
pub enum X86SpecificOp {
    /// CPUID instruction
    Cpuid {
        eax_out: VReg,
        ebx_out: VReg,
        ecx_out: VReg,
        edx_out: VReg,
        leaf: VReg,
        subleaf: VReg,
    },
    
    /// Read MSR
    Rdmsr {
        edx_out: VReg,
        eax_out: VReg,
        ecx: VReg,
    },
    
    /// Write MSR
    Wrmsr {
        ecx: VReg,
        edx: VReg,
        eax: VReg,
    },
    
    /// Read timestamp counter
    Rdtsc {
        edx_out: VReg,
        eax_out: VReg,
    },
    
    /// Read timestamp counter and processor ID
    Rdtscp {
        edx_out: VReg,
        eax_out: VReg,
        ecx_out: VReg,
    },
    
    /// String operation (REP MOVSB, etc.)
    StringOp {
        op: X86StringOp,
        rcx: VReg,
        rsi: VReg,
        rdi: VReg,
        width: OpWidth,
        rep: bool,
    },
    
    /// XSAVE/XRSTOR
    XSave {
        save: bool,
        addr: Address,
        mask: VReg,
    },
    
    /// I/O port access
    PortIo {
        is_out: bool,
        port: SrcOperand,
        data: VReg,
        width: OpWidth,
    },
    
    /// LAHF/SAHF
    LahfSahf {
        is_lahf: bool,
        ah: VReg,
    },
    
    /// ASCII adjust (AAA, AAS, AAM, AAD)
    AsciiAdjust {
        op: AsciiAdjustOp,
        al: VReg,
        ah: VReg,
    },
    
    /// Decimal adjust (DAA, DAS)
    DecimalAdjust {
        is_add: bool,
        al: VReg,
    },
}

/// x86 string operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X86StringOp {
    Movs,  // Move string
    Cmps,  // Compare string
    Scas,  // Scan string
    Lods,  // Load string
    Stos,  // Store string
}

/// ASCII adjust operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsciiAdjustOp {
    Aaa,  // ASCII adjust after addition
    Aas,  // ASCII adjust after subtraction
    Aam,  // ASCII adjust after multiplication
    Aad,  // ASCII adjust before division
}

/// ARM-specific operations
#[derive(Clone, Debug)]
pub enum ArmSpecificOp {
    /// Supervisor call
    Svc {
        imm: u16,
    },
    
    /// Hypervisor call
    Hvc {
        imm: u16,
    },
    
    /// Secure monitor call
    Smc {
        imm: u16,
    },
    
    /// Exception return
    Eret,
    
    /// Wait for interrupt
    Wfi,
    
    /// Wait for event
    Wfe,
    
    /// Send event
    Sev,
    
    /// Yield
    Yield,
    
    /// Instruction synchronization barrier
    Isb,
    
    /// Data synchronization barrier
    Dsb {
        option: u8,
    },
    
    /// Data memory barrier
    Dmb {
        option: u8,
    },
    
    /// MRS (read system register)
    Mrs {
        dst: VReg,
        sysreg: u16,
    },
    
    /// MSR (write system register)
    Msr {
        sysreg: u16,
        src: VReg,
    },
    
    /// AT (address translation)
    At {
        op: u8,
        addr: VReg,
    },
    
    /// DC (data cache operation)
    Dc {
        op: u8,
        addr: VReg,
    },
    
    /// IC (instruction cache operation)
    Ic {
        op: u8,
        addr: Option<VReg>,
    },
    
    /// TLBI (TLB invalidate)
    Tlbi {
        op: u8,
        addr: Option<VReg>,
    },
}

/// Hexagon-specific operations
#[derive(Clone, Debug)]
pub enum HexagonSpecificOp {
    /// Hardware loop setup
    Loop {
        id: u8,  // 0 or 1
        start_offset: i32,
        count: SrcOperand,
    },
    
    /// End of hardware loop (implicit)
    Endloop {
        id: u8,
    },
    
    /// Packet end marker
    PacketEnd,
    
    /// Allocate frame
    AllocFrame {
        size: u32,
    },
    
    /// Deallocate frame and return
    DeallocRet {
        pred: Option<(u8, bool)>,  // (pred_reg, sense)
    },
    
    /// Transfer to/from control register
    TfrCr {
        to_cr: bool,
        cr: u8,
        gpr: VReg,
    },
    
    /// Trap instruction
    Trap {
        imm: u8,
    },
}
```

## 5. Operation Semantics

### 5.1 Integer Arithmetic Semantics

All integer operations work on two's complement representation. Signedness is an operation property, not a type property.

**Add semantics:**
```
dst = (src1 + src2) & mask(width)
if flags.All:
    CF = carry_out
    OF = signed_overflow(src1, src2, dst)
    ZF = (dst == 0)
    SF = sign_bit(dst, width)
    AF = auxiliary_carry(src1, src2)  // x86 only
    PF = parity(dst)                   // x86 only
```

**Sub semantics:**
```
dst = (src1 - src2) & mask(width)
if flags.All:
    CF = borrow_out  // Note: x86 CF is inverted from ARM C
    OF = signed_overflow(src1, -src2, dst)
    ZF = (dst == 0)
    SF = sign_bit(dst, width)
```

### 5.2 Width Handling

Operations specify an `OpWidth` that determines:
1. Source operand masking
2. Result masking
3. Flag computation bits
4. Implicit zero-extension for 32-bit results in 64-bit mode

```rust
fn apply_width(value: u64, width: OpWidth, is_result: bool) -> u64 {
    let masked = value & width.mask();
    if is_result && width == OpWidth::W32 {
        // 32-bit results zero-extend to 64-bit (x86-64 semantics)
        masked
    } else {
        masked
    }
}
```

### 5.3 Memory Semantics

Memory operations are always host-endian in the IR. Architecture endianness is handled during lifting.

**Load semantics:**
```
raw_bytes = memory.read(addr, width.bytes())
value = bytes_to_u64(raw_bytes, host_endian)
if sign == SignExtend::Sign:
    dst = sign_extend(value, width.bits(), 64)
else:
    dst = value
```

### 5.4 Atomic Semantics

Atomic operations provide single-copy atomicity for aligned accesses up to 8 bytes. Larger atomics may require runtime locks.

```rust
fn atomic_rmw(addr: u64, op: AtomicOp, src: u64, width: MemWidth) -> u64 {
    let old = memory.atomic_load(addr, width);
    let new = match op {
        AtomicOp::Add => old.wrapping_add(src),
        AtomicOp::And => old & src,
        AtomicOp::Or => old | src,
        AtomicOp::Xor => old ^ src,
        AtomicOp::Swap => src,
        AtomicOp::Max => std::cmp::max(old as i64, src as i64) as u64,
        AtomicOp::Min => std::cmp::min(old as i64, src as i64) as u64,
        AtomicOp::Umax => std::cmp::max(old, src),
        AtomicOp::Umin => std::cmp::min(old, src),
        // ...
    };
    memory.atomic_store(addr, new, width);
    old
}
```

### 5.5 Floating Point Semantics

FP operations follow IEEE 754-2019 semantics. Exception handling is deferred to architecture-specific state (MXCSR for x86, FPSR for ARM).

Rounding mode is either explicit or read from architecture state (`FpRoundMode::Dynamic`).

### 5.6 Vector Semantics

Vector operations apply the same scalar operation to all lanes in parallel:

```rust
fn vadd(dst: &mut [u8], src1: &[u8], src2: &[u8], elem: VecElementType, lanes: u8) {
    for i in 0..lanes {
        let a = extract_lane(src1, i, elem);
        let b = extract_lane(src2, i, elem);
        insert_lane(dst, i, a.wrapping_add(b), elem);
    }
}
```

Lane indices are always little-endian (lane 0 is least significant).
