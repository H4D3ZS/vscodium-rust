# SMIR Type Definitions

This document specifies all core types used in SMIR.

## 1. Module Structure

```rust
/// Top-level IR module containing all lifted code
pub struct SmirModule {
    /// Unique module identifier
    pub id: ModuleId,
    
    /// Source architecture
    pub source_arch: SourceArch,
    
    /// Functions in this module
    pub functions: Vec<SmirFunction>,
    
    /// Global symbol table (name -> address)
    pub symbols: HashMap<String, GuestAddr>,
    
    /// External references (imports)
    pub externals: Vec<ExternalRef>,
    
    /// Module-level metadata
    pub metadata: ModuleMetadata,
}

/// Source architecture identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SourceArch {
    X86_64,
    Aarch64,
    Aarch32,
    Thumb,
    Hexagon,
    RiscV64,
    RiscV32,
}

/// Unique module identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleId(pub u64);

/// Guest virtual address
pub type GuestAddr = u64;

/// Module metadata
pub struct ModuleMetadata {
    /// Guest entry point address
    pub entry_point: Option<GuestAddr>,
    
    /// Guest stack base (if known)
    pub stack_base: Option<GuestAddr>,
    
    /// Text section range
    pub text_range: Option<(GuestAddr, GuestAddr)>,
    
    /// Lifted timestamp
    pub lifted_at: std::time::Instant,
}
```

## 2. Function Structure

```rust
/// A lifted function
pub struct SmirFunction {
    /// Function identifier
    pub id: FunctionId,
    
    /// Entry block ID
    pub entry: BlockId,
    
    /// All basic blocks
    pub blocks: Vec<SmirBlock>,
    
    /// Local variable slots (spills, temporaries)
    pub locals: Vec<LocalSlot>,
    
    /// Guest address range covered
    pub guest_range: (GuestAddr, GuestAddr),
    
    /// Calling convention (for cross-calls)
    pub calling_convention: CallingConv,
    
    /// Function attributes
    pub attrs: FunctionAttrs,
}

/// Function identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId(pub u32);

/// Calling convention
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallingConv {
    /// Preserve all guest state
    GuestPreserveAll,
    
    /// x86_64 System V ABI
    X86SysV,
    
    /// x86_64 Windows ABI
    X86Win64,
    
    /// AArch64 AAPCS
    Aarch64Aapcs,
    
    /// Hexagon standard
    HexagonStd,
}

/// Local variable slot
pub struct LocalSlot {
    pub id: LocalId,
    pub ty: SmirType,
    pub size: u32,
    pub align: u32,
}

/// Local slot identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalId(pub u16);

/// Function attributes
#[derive(Clone, Copy, Debug, Default)]
pub struct FunctionAttrs {
    /// May not return (infinite loop, abort)
    pub no_return: bool,
    
    /// Has no side effects beyond return value
    pub pure: bool,
    
    /// Entry point of the guest program
    pub is_entry: bool,
    
    /// Exception handler
    pub is_exception_handler: bool,
}
```

## 3. Basic Block Structure

```rust
/// A basic block
pub struct SmirBlock {
    /// Block identifier
    pub id: BlockId,
    
    /// Guest PC at block entry
    pub guest_pc: GuestAddr,
    
    /// Phi nodes (for SSA, may be empty)
    pub phis: Vec<PhiNode>,
    
    /// Operations in this block
    pub ops: Vec<SmirOp>,
    
    /// Block terminator
    pub terminator: Terminator,
    
    /// Estimated execution count (for hot path detection)
    pub exec_count: u64,
}

/// Block identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

/// Phi node (SSA)
pub struct PhiNode {
    pub dst: VReg,
    pub ty: SmirType,
    pub sources: Vec<(BlockId, VReg)>,
}

/// Block terminator
#[derive(Clone, Debug)]
pub enum Terminator {
    /// Unconditional branch
    Branch {
        target: BlockId,
    },
    
    /// Conditional branch
    CondBranch {
        cond: VReg,
        true_target: BlockId,
        false_target: BlockId,
    },
    
    /// Multi-way branch (switch)
    Switch {
        index: VReg,
        targets: Vec<BlockId>,
        default: BlockId,
    },
    
    /// Indirect branch (computed goto)
    IndirectBranch {
        target: VReg,
        possible_targets: Vec<BlockId>,
    },
    
    /// Function return
    Return {
        values: Vec<VReg>,
    },
    
    /// Call with continuation
    Call {
        target: CallTarget,
        args: Vec<VReg>,
        continuation: BlockId,
    },
    
    /// Trap (undefined, breakpoint, etc.)
    Trap {
        kind: TrapKind,
    },
    
    /// Unreachable (for dead code)
    Unreachable,
}

/// Call target
#[derive(Clone, Debug)]
pub enum CallTarget {
    /// Direct call to known function
    Direct(FunctionId),
    
    /// Direct call to guest address
    GuestAddr(GuestAddr),
    
    /// Indirect call through register
    Indirect(VReg),
    
    /// External runtime function
    Runtime(RuntimeFunc),
}

/// Runtime helper functions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeFunc {
    /// System call handler
    Syscall,
    
    /// Page fault handler
    PageFault,
    
    /// FP exception handler
    FpException,
    
    /// Undefined instruction handler
    Undefined,
    
    /// Debug breakpoint
    Breakpoint,
    
    /// Memory barrier (fence)
    MemoryBarrier,
    
    /// CPUID (x86)
    Cpuid,
    
    /// MSR read (x86)
    Rdmsr,
    
    /// MSR write (x86)
    Wrmsr,
}

/// Trap kinds
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrapKind {
    Breakpoint,
    Undefined,
    DivideByZero,
    Overflow,
    Bounds,
    InvalidOpcode,
    SystemCall,
    /// Halt and wait for interrupt
    Halt,
}
```

## 4. Virtual Register

```rust
/// Virtual register (value holder)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VReg {
    /// Virtual register (SSA-style, unlimited count)
    Virtual(VirtualId),
    
    /// Architecture-specific register (pinned)
    Arch(ArchReg),
    
    /// Immediate value (small optimization)
    Imm(i64),
}

/// Virtual register ID
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VirtualId(pub u32);

/// Architecture-specific register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArchReg {
    X86(X86Reg),
    Arm(ArmReg),
    Hexagon(HexagonReg),
}

/// x86_64 register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum X86Reg {
    // General purpose (0-15)
    Rax, Rcx, Rdx, Rbx, Rsp, Rbp, Rsi, Rdi,
    R8, R9, R10, R11, R12, R13, R14, R15,
    
    // Instruction pointer
    Rip,
    
    // Flags
    Rflags,
    
    // Segment selectors
    Cs, Ds, Es, Fs, Gs, Ss,
    
    // Segment bases (for FS/GS)
    FsBase, GsBase,
    
    // SIMD registers (0-31)
    Xmm(u8),
    Ymm(u8),
    Zmm(u8),
    
    // x87 FP stack
    St(u8),
    
    // Control registers
    Cr0, Cr2, Cr3, Cr4, Cr8,
    
    // Opmask registers (AVX-512)
    K(u8),
}

/// AArch64 register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArmReg {
    // General purpose X0-X30
    X(u8),
    
    // Stack pointer
    Sp,
    
    // Program counter (read-only via PC reads)
    Pc,
    
    // PSTATE flags
    Nzcv,
    
    // SIMD registers V0-V31
    V(u8),
    
    // SVE registers Z0-Z31 (extends V)
    Z(u8),
    
    // SVE predicate registers P0-P15
    P(u8),
    
    // FFR (first-fault register)
    Ffr,
    
    // System registers (using encoding)
    SysReg(u16),
    
    // FPCR/FPSR
    Fpcr,
    Fpsr,
}

/// Hexagon register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HexagonReg {
    // General purpose R0-R31
    R(u8),
    
    // Double registers R1:0, R3:2, etc.
    D(u8),
    
    // Predicate registers P0-P3
    P(u8),
    
    // Control registers
    Pc,
    Gp,
    Lr,
    Fp,
    Sp,
    
    // Loop registers
    Lc0, Lc1,
    Sa0, Sa1,
    
    // User status
    Usr,
}
```

## 5. SMIR Types

```rust
/// SMIR value type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SmirType {
    /// Integer types
    I8,
    I16,
    I32,
    I64,
    I128,
    
    /// Floating point
    F32,
    F64,
    
    /// Vectors (element_type, num_elements)
    Vec(VecElementType, u8),
    
    /// Pointer (guest address)
    Ptr,
    
    /// Flag state (architecture-dependent)
    Flags,
    
    /// Predicate (1-bit)
    Pred,
    
    /// Void (for side-effect-only ops)
    Void,
}

/// Vector element type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VecElementType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl SmirType {
    /// Size in bytes
    pub const fn size(&self) -> u32 {
        match self {
            SmirType::I8 | SmirType::Pred => 1,
            SmirType::I16 => 2,
            SmirType::I32 | SmirType::F32 => 4,
            SmirType::I64 | SmirType::F64 | SmirType::Ptr | SmirType::Flags => 8,
            SmirType::I128 => 16,
            SmirType::Vec(elem, count) => elem.size() * (*count as u32),
            SmirType::Void => 0,
        }
    }
    
    /// Alignment in bytes
    pub const fn align(&self) -> u32 {
        match self {
            SmirType::Vec(_, count) if *count >= 8 => 16,
            _ => self.size().min(8),
        }
    }
}

impl VecElementType {
    pub const fn size(&self) -> u32 {
        match self {
            VecElementType::I8 => 1,
            VecElementType::I16 => 2,
            VecElementType::I32 | VecElementType::F32 => 4,
            VecElementType::I64 | VecElementType::F64 => 8,
        }
    }
}
```

## 6. Operand Width

```rust
/// Operation width (for integer ops)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OpWidth {
    W8,
    W16,
    W32,
    W64,
    W128,
}

impl OpWidth {
    pub const fn bits(&self) -> u32 {
        match self {
            OpWidth::W8 => 8,
            OpWidth::W16 => 16,
            OpWidth::W32 => 32,
            OpWidth::W64 => 64,
            OpWidth::W128 => 128,
        }
    }
    
    pub const fn bytes(&self) -> u32 {
        self.bits() / 8
    }
    
    pub const fn mask(&self) -> u64 {
        match self {
            OpWidth::W8 => 0xFF,
            OpWidth::W16 => 0xFFFF,
            OpWidth::W32 => 0xFFFF_FFFF,
            OpWidth::W64 => u64::MAX,
            OpWidth::W128 => u64::MAX, // Lower 64 bits only
        }
    }
}
```

## 7. Memory Types

```rust
/// Memory access width
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemWidth {
    B1,   // 1 byte
    B2,   // 2 bytes
    B4,   // 4 bytes
    B8,   // 8 bytes
    B16,  // 16 bytes (XMM)
    B32,  // 32 bytes (YMM)
    B64,  // 64 bytes (ZMM)
}

impl MemWidth {
    pub const fn bytes(&self) -> u32 {
        match self {
            MemWidth::B1 => 1,
            MemWidth::B2 => 2,
            MemWidth::B4 => 4,
            MemWidth::B8 => 8,
            MemWidth::B16 => 16,
            MemWidth::B32 => 32,
            MemWidth::B64 => 64,
        }
    }
}

/// Sign extension mode for loads
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SignExtend {
    /// Zero-extend (unsigned)
    Zero,
    /// Sign-extend (signed)
    Sign,
}

/// Memory ordering for atomic operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryOrder {
    /// No ordering constraints
    Relaxed,
    /// Acquire semantics (loads)
    Acquire,
    /// Release semantics (stores)
    Release,
    /// Acquire-release (RMW)
    AcqRel,
    /// Sequentially consistent
    SeqCst,
}
```

## 8. Address Types

```rust
/// Memory address operand
#[derive(Clone, Debug, PartialEq)]
pub enum Address {
    /// Simple register indirect: [reg]
    Direct(VReg),
    
    /// Base + displacement: [reg + offset]
    BaseOffset {
        base: VReg,
        offset: i64,
    },
    
    /// Base + index + scale + displacement: [base + index*scale + disp]
    /// Used for x86 SIB addressing
    BaseIndexScale {
        base: Option<VReg>,
        index: VReg,
        scale: u8,  // 1, 2, 4, or 8
        disp: i32,
    },
    
    /// PC-relative: [PC + offset]
    PcRel {
        offset: i64,
    },
    
    /// GP-relative (Hexagon): [GP + offset]
    GpRel {
        offset: i32,
    },
    
    /// Absolute address (for MMIO, fixed addresses)
    Absolute(u64),
}

impl Address {
    /// Create a simple register indirect address
    pub fn reg(r: VReg) -> Self {
        Address::Direct(r)
    }
    
    /// Create base + offset address
    pub fn base_off(base: VReg, offset: i64) -> Self {
        Address::BaseOffset { base, offset }
    }
    
    /// Create x86-style SIB address
    pub fn sib(base: Option<VReg>, index: VReg, scale: u8, disp: i32) -> Self {
        Address::BaseIndexScale { base, index, scale, disp }
    }
}
```

## 9. Source Operand

```rust
/// Source operand (can be register or immediate)
#[derive(Clone, Debug, PartialEq)]
pub enum SrcOperand {
    /// Register value
    Reg(VReg),
    
    /// Immediate value
    Imm(i64),
    
    /// Shifted register (ARM)
    Shifted {
        reg: VReg,
        shift: ShiftOp,
        amount: u8,
    },
    
    /// Extended register (ARM)
    Extended {
        reg: VReg,
        extend: ExtendOp,
        shift: u8,
    },
}

/// Shift operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShiftOp {
    Lsl,  // Logical shift left
    Lsr,  // Logical shift right
    Asr,  // Arithmetic shift right
    Ror,  // Rotate right
    Rrx,  // Rotate right through carry (x86/ARM32)
}

/// Extend operation type (ARM)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExtendOp {
    Uxtb,  // Unsigned extend byte
    Uxth,  // Unsigned extend halfword
    Uxtw,  // Unsigned extend word
    Uxtx,  // Unsigned extend doubleword (no-op for 64-bit)
    Sxtb,  // Sign extend byte
    Sxth,  // Sign extend halfword
    Sxtw,  // Sign extend word
    Sxtx,  // Sign extend doubleword (no-op for 64-bit)
}
```

## 10. Condition Codes

```rust
/// Condition code for branches
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    // Unified conditions (both x86 and ARM)
    Eq,   // Equal (Z=1)
    Ne,   // Not equal (Z=0)
    
    // Unsigned comparisons
    Ult,  // Unsigned less than (C=0)
    Ule,  // Unsigned less or equal (C=0 or Z=1)
    Ugt,  // Unsigned greater than (C=1 and Z=0)
    Uge,  // Unsigned greater or equal (C=1)
    
    // Signed comparisons
    Slt,  // Signed less than (N != V)
    Sle,  // Signed less or equal (Z=1 or N != V)
    Sgt,  // Signed greater than (Z=0 and N == V)
    Sge,  // Signed greater or equal (N == V)
    
    // Individual flags
    Negative,  // N=1
    Positive,  // N=0
    Overflow,  // V=1
    NoOverflow,// V=0
    
    // x86-specific
    Parity,    // PF=1
    NoParity,  // PF=0
    
    // Always (unconditional)
    Always,
}

impl Condition {
    /// Invert the condition
    pub fn invert(self) -> Condition {
        match self {
            Condition::Eq => Condition::Ne,
            Condition::Ne => Condition::Eq,
            Condition::Ult => Condition::Uge,
            Condition::Ule => Condition::Ugt,
            Condition::Ugt => Condition::Ule,
            Condition::Uge => Condition::Ult,
            Condition::Slt => Condition::Sge,
            Condition::Sle => Condition::Sgt,
            Condition::Sgt => Condition::Sle,
            Condition::Sge => Condition::Slt,
            Condition::Negative => Condition::Positive,
            Condition::Positive => Condition::Negative,
            Condition::Overflow => Condition::NoOverflow,
            Condition::NoOverflow => Condition::Overflow,
            Condition::Parity => Condition::NoParity,
            Condition::NoParity => Condition::Parity,
            Condition::Always => Condition::Always,
        }
    }
}
```

## 11. Flag Handling Types

```rust
/// Flag update mode for arithmetic operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FlagUpdate {
    /// Don't update flags
    None,
    
    /// Update all arithmetic flags
    All,
    
    /// Update specific flags only
    Specific(FlagSet),
}

/// Set of flags to update/read
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FlagSet(u8);

impl FlagSet {
    pub const EMPTY: FlagSet = FlagSet(0);
    pub const CF: FlagSet = FlagSet(1 << 0);  // Carry
    pub const ZF: FlagSet = FlagSet(1 << 1);  // Zero
    pub const SF: FlagSet = FlagSet(1 << 2);  // Sign/Negative
    pub const OF: FlagSet = FlagSet(1 << 3);  // Overflow
    pub const PF: FlagSet = FlagSet(1 << 4);  // Parity (x86)
    pub const AF: FlagSet = FlagSet(1 << 5);  // Auxiliary carry (x86)
    
    /// All arithmetic flags
    pub const NZCV: FlagSet = FlagSet(0x0F);
    pub const ALL_X86: FlagSet = FlagSet(0x3F);
    
    pub const fn union(self, other: FlagSet) -> FlagSet {
        FlagSet(self.0 | other.0)
    }
    
    pub const fn contains(self, flag: FlagSet) -> bool {
        (self.0 & flag.0) == flag.0
    }
}

/// Lazy flag state (deferred computation)
#[derive(Clone, Copy, Debug)]
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
}

/// Lazy flag operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LazyFlagOp {
    /// No flags set (materialized or invalid)
    None,
    
    /// Addition (affects C, Z, S, O, P, A)
    Add,
    
    /// Subtraction/comparison (affects C, Z, S, O, P, A)
    Sub,
    
    /// Logical operation (clears C, O; affects Z, S, P)
    Logic,
    
    /// Increment (affects Z, S, O, P, A; preserves C)
    Inc,
    
    /// Decrement (affects Z, S, O, P, A; preserves C)
    Dec,
    
    /// Shift left (affects C, Z, S, P; O undefined for count > 1)
    Shl,
    
    /// Logical shift right
    Shr,
    
    /// Arithmetic shift right
    Sar,
    
    /// Rotate (only affects C, O)
    Rotate,
    
    /// Multiply (affects C, O; Z, S, P undefined)
    Mul,
    
    /// Bit test (affects C; clears O)
    Bt,
}
```

This completes the core type definitions. See `03-opcodes.md` for the complete operation catalog.
