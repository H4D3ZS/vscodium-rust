//! SMIR core type definitions.
//!
//! This module defines the fundamental types used throughout SMIR:
//! - Virtual registers and architecture-specific registers
//! - Operation and memory widths
//! - Addressing modes
//! - Source operands

use std::collections::HashMap;

/// Guest virtual address
pub type GuestAddr = u64;

// ============================================================================
// Identifiers
// ============================================================================

/// Module identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleId(pub u64);

/// Function identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId(pub u32);

/// Block identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

/// Operation identifier (block-local)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpId(pub u16);

/// Virtual register identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VirtualId(pub u32);

/// Local slot identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalId(pub u16);

// ============================================================================
// Source Architecture
// ============================================================================

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
    Mips64,
    Mips32,
    Sparc64,
    Sparc32,
}

impl SourceArch {
    /// Default endianness for this architecture
    pub fn default_endian(&self) -> Endian {
        match self {
            SourceArch::X86_64
            | SourceArch::Aarch64
            | SourceArch::Aarch32
            | SourceArch::Thumb
            | SourceArch::Hexagon
            | SourceArch::RiscV64
            | SourceArch::RiscV32 => Endian::Little,
            SourceArch::Mips64 | SourceArch::Mips32 | SourceArch::Sparc64 | SourceArch::Sparc32 => {
                Endian::Big
            }
        }
    }

    /// Whether this architecture requires strict alignment
    pub fn strict_alignment(&self) -> bool {
        !matches!(self, SourceArch::X86_64)
    }

    /// Register width in bits
    pub fn reg_width(&self) -> u32 {
        match self {
            SourceArch::X86_64
            | SourceArch::Aarch64
            | SourceArch::RiscV64
            | SourceArch::Mips64
            | SourceArch::Sparc64 => 64,
            SourceArch::Aarch32
            | SourceArch::Thumb
            | SourceArch::Hexagon
            | SourceArch::RiscV32
            | SourceArch::Mips32
            | SourceArch::Sparc32 => 32,
        }
    }
}

/// Byte order
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Endian {
    Little,
    Big,
}

// ============================================================================
// Virtual Registers
// ============================================================================

/// Virtual register (value holder)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VReg {
    /// Virtual register (SSA-style, unlimited count)
    Virtual(VirtualId),

    /// Architecture-specific register (pinned)
    Arch(ArchReg),

    /// Immediate value (small optimization for constants)
    Imm(i64),
}

impl VReg {
    /// Create a new virtual register
    pub fn virt(id: u32) -> Self {
        VReg::Virtual(VirtualId(id))
    }

    /// Create an immediate value
    pub fn imm(val: i64) -> Self {
        VReg::Imm(val)
    }

    /// Check if this is a virtual register
    pub fn is_virtual(&self) -> bool {
        matches!(self, VReg::Virtual(_))
    }

    /// Check if this is an architecture register
    pub fn is_arch(&self) -> bool {
        matches!(self, VReg::Arch(_))
    }

    /// Check if this is an immediate
    pub fn is_imm(&self) -> bool {
        matches!(self, VReg::Imm(_))
    }
}

/// Architecture-specific register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArchReg {
    X86(X86Reg),
    Arm(ArmReg),
    Hexagon(HexagonReg),
    RiscV(RiscVReg),
}

/// x86_64 register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum X86Reg {
    // General purpose (0-31). APX adds R16-R31.
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,

    // Instruction pointer
    Rip,

    // Flags
    Rflags,

    // Segment bases
    FsBase,
    GsBase,

    // SIMD registers (0-31)
    Xmm(u8),
    Ymm(u8),
    Zmm(u8),

    // Opmask registers (AVX-512)
    K(u8),
}

impl X86Reg {
    /// Get GPR by index (0=RAX, 1=RCX, ..., 31=R31).
    pub fn gpr(idx: u8) -> Self {
        match idx & 0x1f {
            0 => X86Reg::Rax,
            1 => X86Reg::Rcx,
            2 => X86Reg::Rdx,
            3 => X86Reg::Rbx,
            4 => X86Reg::Rsp,
            5 => X86Reg::Rbp,
            6 => X86Reg::Rsi,
            7 => X86Reg::Rdi,
            8 => X86Reg::R8,
            9 => X86Reg::R9,
            10 => X86Reg::R10,
            11 => X86Reg::R11,
            12 => X86Reg::R12,
            13 => X86Reg::R13,
            14 => X86Reg::R14,
            15 => X86Reg::R15,
            16 => X86Reg::R16,
            17 => X86Reg::R17,
            18 => X86Reg::R18,
            19 => X86Reg::R19,
            20 => X86Reg::R20,
            21 => X86Reg::R21,
            22 => X86Reg::R22,
            23 => X86Reg::R23,
            24 => X86Reg::R24,
            25 => X86Reg::R25,
            26 => X86Reg::R26,
            27 => X86Reg::R27,
            28 => X86Reg::R28,
            29 => X86Reg::R29,
            30 => X86Reg::R30,
            31 => X86Reg::R31,
            _ => unreachable!(),
        }
    }

    /// Return the x86 GPR encoding index for RAX..R31.
    pub fn gpr_index(self) -> Option<u8> {
        Some(match self {
            X86Reg::Rax => 0,
            X86Reg::Rcx => 1,
            X86Reg::Rdx => 2,
            X86Reg::Rbx => 3,
            X86Reg::Rsp => 4,
            X86Reg::Rbp => 5,
            X86Reg::Rsi => 6,
            X86Reg::Rdi => 7,
            X86Reg::R8 => 8,
            X86Reg::R9 => 9,
            X86Reg::R10 => 10,
            X86Reg::R11 => 11,
            X86Reg::R12 => 12,
            X86Reg::R13 => 13,
            X86Reg::R14 => 14,
            X86Reg::R15 => 15,
            X86Reg::R16 => 16,
            X86Reg::R17 => 17,
            X86Reg::R18 => 18,
            X86Reg::R19 => 19,
            X86Reg::R20 => 20,
            X86Reg::R21 => 21,
            X86Reg::R22 => 22,
            X86Reg::R23 => 23,
            X86Reg::R24 => 24,
            X86Reg::R25 => 25,
            X86Reg::R26 => 26,
            X86Reg::R27 => 27,
            X86Reg::R28 => 28,
            X86Reg::R29 => 29,
            X86Reg::R30 => 30,
            X86Reg::R31 => 31,
            _ => return None,
        })
    }

    /// True for APX EGPRs R16-R31.
    pub fn is_egpr(self) -> bool {
        matches!(
            self,
            X86Reg::R16
                | X86Reg::R17
                | X86Reg::R18
                | X86Reg::R19
                | X86Reg::R20
                | X86Reg::R21
                | X86Reg::R22
                | X86Reg::R23
                | X86Reg::R24
                | X86Reg::R25
                | X86Reg::R26
                | X86Reg::R27
                | X86Reg::R28
                | X86Reg::R29
                | X86Reg::R30
                | X86Reg::R31
        )
    }
}

/// AArch64 register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArmReg {
    /// General purpose X0-X30
    X(u8),
    /// Stack pointer
    Sp,
    /// Program counter (read-only)
    Pc,
    /// PSTATE flags (NZCV)
    Nzcv,
    /// SIMD registers V0-V31
    V(u8),
    /// FPCR/FPSR
    Fpcr,
    Fpsr,
    /// System register by encoding
    SysReg(u16),
}

/// Hexagon register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HexagonReg {
    /// General purpose R0-R31
    R(u8),
    /// Predicate registers P0-P3
    P(u8),
    /// Program counter
    Pc,
    /// Global pointer
    Gp,
    /// Link register
    Lr,
    /// Stack pointer
    Sp,
    /// Frame pointer
    Fp,
    /// Loop count registers
    Lc0,
    Lc1,
    /// Loop start address registers
    Sa0,
    Sa1,
    /// User status register
    Usr,
    /// HVX vector registers V0-V31 (1024-bit)
    V(u8),
    /// HVX vector predicate registers Q0-Q3 (128-bit)
    Q(u8),
    /// Modifier registers M0/M1 (circular/bit-reversed addressing)
    M(u8),
    /// Circular-buffer start registers CS0/CS1
    Cs(u8),
}

/// RISC-V register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RiscVReg {
    /// Integer registers x0-x31
    X(u8),
    /// Floating-point registers f0-f31
    F(u8),
    /// Vector registers v0-v31
    V(u8),
    /// Program counter
    Pc,
    /// CSR by number
    Csr(u16),
}

// ============================================================================
// Operation Width
// ============================================================================

/// Operation width (for integer ops)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum OpWidth {
    W8,
    W16,
    W32,
    #[default]
    W64,
    W128,
}

impl OpWidth {
    /// Width in bits
    pub const fn bits(&self) -> u32 {
        match self {
            OpWidth::W8 => 8,
            OpWidth::W16 => 16,
            OpWidth::W32 => 32,
            OpWidth::W64 => 64,
            OpWidth::W128 => 128,
        }
    }

    /// Width in bytes
    pub const fn bytes(&self) -> u32 {
        self.bits() / 8
    }

    /// Mask for this width
    pub const fn mask(&self) -> u64 {
        match self {
            OpWidth::W8 => 0xFF,
            OpWidth::W16 => 0xFFFF,
            OpWidth::W32 => 0xFFFF_FFFF,
            OpWidth::W64 | OpWidth::W128 => u64::MAX,
        }
    }

    /// Sign bit position
    pub const fn sign_bit(&self) -> u64 {
        1u64 << (self.bits() - 1)
    }

    /// Convert to MemWidth
    pub const fn to_mem_width(&self) -> MemWidth {
        match self {
            OpWidth::W8 => MemWidth::B1,
            OpWidth::W16 => MemWidth::B2,
            OpWidth::W32 => MemWidth::B4,
            OpWidth::W64 => MemWidth::B8,
            OpWidth::W128 => MemWidth::B16,
        }
    }
}

// ============================================================================
// Memory Width
// ============================================================================

/// Memory access width
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum MemWidth {
    B1, // 1 byte
    B2, // 2 bytes
    B4, // 4 bytes
    #[default]
    B8, // 8 bytes
    B16, // 16 bytes (XMM)
    B32, // 32 bytes (YMM)
    B64, // 64 bytes (ZMM)
}

impl MemWidth {
    /// Width in bytes
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

    /// Convert to OpWidth (for scalar loads)
    pub const fn to_op_width(&self) -> Option<OpWidth> {
        match self {
            MemWidth::B1 => Some(OpWidth::W8),
            MemWidth::B2 => Some(OpWidth::W16),
            MemWidth::B4 => Some(OpWidth::W32),
            MemWidth::B8 => Some(OpWidth::W64),
            MemWidth::B16 => Some(OpWidth::W128),
            MemWidth::B32 | MemWidth::B64 => None,
        }
    }
}

/// Sign extension mode for loads
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SignExtend {
    /// Zero-extend (unsigned)
    #[default]
    Zero,
    /// Sign-extend (signed)
    Sign,
}

// ============================================================================
// Address Modes
// ============================================================================

/// Displacement size hint (x86 addressing)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DispSize {
    /// Automatically choose smallest encoding
    Auto,
    /// Force 8-bit displacement
    Disp8,
    /// Force 32-bit displacement
    Disp32,
}

/// Memory address operand
#[derive(Clone, Debug, PartialEq)]
pub enum Address {
    /// Simple register indirect: [reg]
    Direct(VReg),

    /// Base + displacement: [reg + offset]
    BaseOffset {
        base: VReg,
        offset: i64,
        disp_size: DispSize,
    },

    /// Base + index + scale + displacement: [base + index*scale + disp]
    /// Used for x86 SIB addressing
    BaseIndexScale {
        base: Option<VReg>,
        index: VReg,
        scale: u8, // 1, 2, 4, or 8
        disp: i32,
        disp_size: DispSize,
    },

    /// PC-relative: [PC + offset]
    PcRel {
        offset: i64,
        disp_size: DispSize,
        base: Option<GuestAddr>,
    },

    /// GP-relative (Hexagon): [GP + offset]
    GpRel { offset: i32 },

    /// Absolute address (for MMIO, fixed addresses)
    Absolute(u64),

    /// x86 segment-relative: [segment_base + base + index*scale + disp].
    ///
    /// `segment` is the `FsBase`/`GsBase` architectural register holding the
    /// segment base (from the FS/GS descriptor or the IA32_FS_BASE/IA32_GS_BASE
    /// MSR). In 64-bit mode only FS and GS carry a non-zero base — CS/DS/ES/SS
    /// are flat — so this variant is emitted ONLY for FS/GS-overridden operands.
    /// A RIP-relative FS/GS operand is folded into `disp` at lift time (the
    /// next-RIP is a constant), so `base`/`index` here are true GPRs.
    SegmentRel {
        segment: VReg,
        base: Option<VReg>,
        index: Option<VReg>,
        scale: u8,
        disp: i64,
    },
}

impl Address {
    /// Create a simple register indirect address
    pub fn reg(r: VReg) -> Self {
        Address::Direct(r)
    }

    /// Create base + offset address
    pub fn base_off(base: VReg, offset: i64) -> Self {
        Address::BaseOffset {
            base,
            offset,
            disp_size: DispSize::Auto,
        }
    }

    /// Create x86-style SIB address
    pub fn sib(base: Option<VReg>, index: VReg, scale: u8, disp: i32) -> Self {
        Address::BaseIndexScale {
            base,
            index,
            scale,
            disp,
            disp_size: DispSize::Auto,
        }
    }

    /// Get all registers used in this address
    pub fn regs(&self) -> Vec<VReg> {
        match self {
            Address::Direct(r) => vec![*r],
            Address::BaseOffset { base, .. } => vec![*base],
            Address::BaseIndexScale { base, index, .. } => {
                let mut v = vec![*index];
                if let Some(b) = base {
                    v.push(*b);
                }
                v
            }
            Address::PcRel { .. } | Address::GpRel { .. } | Address::Absolute(_) => vec![],
            Address::SegmentRel {
                segment,
                base,
                index,
                ..
            } => {
                let mut v = vec![*segment];
                if let Some(b) = base {
                    v.push(*b);
                }
                if let Some(i) = index {
                    v.push(*i);
                }
                v
            }
        }
    }
}

// ============================================================================
// Source Operand
// ============================================================================

/// Source operand (can be register or immediate)
#[derive(Clone, Debug, PartialEq)]
pub enum SrcOperand {
    /// Register value
    Reg(VReg),

    /// Immediate value
    Imm(i64),

    /// Immediate value that should preserve 64-bit encoding (x86 MOV imm64)
    Imm64(i64),

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

impl SrcOperand {
    /// Create a register operand
    pub fn reg(r: VReg) -> Self {
        SrcOperand::Reg(r)
    }

    /// Create an immediate operand
    pub fn imm(val: i64) -> Self {
        SrcOperand::Imm(val)
    }

    /// Create a 64-bit immediate operand
    pub fn imm64(val: i64) -> Self {
        SrcOperand::Imm64(val)
    }

    /// Check if this is an immediate
    pub fn is_imm(&self) -> bool {
        matches!(self, SrcOperand::Imm(_) | SrcOperand::Imm64(_))
    }

    /// Get the register if this is a simple register operand
    pub fn as_reg(&self) -> Option<VReg> {
        match self {
            SrcOperand::Reg(r) => Some(*r),
            _ => None,
        }
    }

    /// Get the immediate value if this is an immediate
    pub fn as_imm(&self) -> Option<i64> {
        match self {
            SrcOperand::Imm(v) | SrcOperand::Imm64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get all registers used in this operand
    pub fn regs(&self) -> Vec<VReg> {
        match self {
            SrcOperand::Reg(r)
            | SrcOperand::Shifted { reg: r, .. }
            | SrcOperand::Extended { reg: r, .. } => {
                vec![*r]
            }
            SrcOperand::Imm(_) | SrcOperand::Imm64(_) => vec![],
        }
    }
}

impl From<VReg> for SrcOperand {
    fn from(r: VReg) -> Self {
        SrcOperand::Reg(r)
    }
}

impl From<i64> for SrcOperand {
    fn from(v: i64) -> Self {
        SrcOperand::Imm(v)
    }
}

/// Shift operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShiftOp {
    /// Logical shift left
    Lsl,
    /// Logical shift right
    Lsr,
    /// Arithmetic shift right
    Asr,
    /// Rotate right
    Ror,
    /// Rotate right through carry (x86/ARM32)
    Rrx,
}

/// Extend operation type (ARM)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExtendOp {
    /// Unsigned extend byte
    Uxtb,
    /// Unsigned extend halfword
    Uxth,
    /// Unsigned extend word
    Uxtw,
    /// Unsigned extend doubleword (no-op for 64-bit)
    Uxtx,
    /// Sign extend byte
    Sxtb,
    /// Sign extend halfword
    Sxth,
    /// Sign extend word
    Sxtw,
    /// Sign extend doubleword (no-op for 64-bit)
    Sxtx,
}

// ============================================================================
// Memory Ordering
// ============================================================================

/// Memory ordering constraints
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MemoryOrder {
    /// No ordering constraints
    #[default]
    Relaxed,
    /// Acquire: subsequent reads/writes cannot be reordered before this load
    Acquire,
    /// Release: previous reads/writes cannot be reordered after this store
    Release,
    /// Acquire-release: both acquire and release semantics
    AcqRel,
    /// Sequentially consistent: total ordering
    SeqCst,
}

/// Fence kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FenceKind {
    /// Load-load barrier
    LoadLoad,
    /// Load-store barrier
    LoadStore,
    /// Store-load barrier (most expensive)
    StoreLoad,
    /// Store-store barrier
    StoreStore,
    /// Full barrier (all of the above)
    Full,
    /// Instruction synchronization
    ISync,
    /// Data synchronization
    DSync,
}

/// Atomic operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AtomicOp {
    Add,
    Sub,
    And,
    Or,
    Xor,
    Nand,
    Max,  // Signed maximum
    Min,  // Signed minimum
    Umax, // Unsigned maximum
    Umin, // Unsigned minimum
    Swap, // Exchange
}

// ============================================================================
// Floating Point
// ============================================================================

/// FP precision
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FpPrecision {
    /// IEEE 754 half (16-bit)
    F16,
    /// IEEE 754 single (32-bit)
    F32,
    /// IEEE 754 double (64-bit)
    F64,
    /// x87 extended (80-bit, x86 only)
    F80,
}

impl FpPrecision {
    /// Size in bytes
    pub const fn bytes(&self) -> u32 {
        match self {
            FpPrecision::F16 => 2,
            FpPrecision::F32 => 4,
            FpPrecision::F64 => 8,
            FpPrecision::F80 => 10,
        }
    }
}

/// FP rounding mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FpRoundMode {
    /// Round to nearest, ties to even
    #[default]
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

// ============================================================================
// Vector Types
// ============================================================================

/// Vector element type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VecElementType {
    I8,
    I16,
    I32,
    I64,
    F16,
    F32,
    F64,
}

impl VecElementType {
    /// Size in bytes
    pub const fn bytes(&self) -> u32 {
        match self {
            VecElementType::I8 => 1,
            VecElementType::I16 | VecElementType::F16 => 2,
            VecElementType::I32 | VecElementType::F32 => 4,
            VecElementType::I64 | VecElementType::F64 => 8,
        }
    }

    /// Is this a floating-point type?
    pub const fn is_float(&self) -> bool {
        matches!(
            self,
            VecElementType::F16 | VecElementType::F32 | VecElementType::F64
        )
    }
}

/// Vector register width
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VecWidth {
    /// 64-bit (D registers on ARM)
    V64,
    /// 128-bit (XMM, Q registers)
    V128,
    /// 256-bit (YMM)
    V256,
    /// 512-bit (ZMM)
    V512,
}

impl VecWidth {
    /// Width in bytes
    pub const fn bytes(&self) -> u32 {
        match self {
            VecWidth::V64 => 8,
            VecWidth::V128 => 16,
            VecWidth::V256 => 32,
            VecWidth::V512 => 64,
        }
    }

    /// Number of lanes for a given element type
    pub const fn lanes(&self, elem: VecElementType) -> u32 {
        self.bytes() / elem.bytes()
    }
}

/// Vector comparison condition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VecCmpCond {
    Eq,
    Ne,
    Lt, // Signed
    Le,
    Gt,
    Ge,
    Ltu, // Unsigned
    Leu,
    Gtu,
    Geu,
}

/// Per-lane bidirectional shift kind for `OpKind::VShiftV` (HVX vector-amount
/// shifts). The shift amount is a per-lane signed value; positive shifts in the
/// op's nominal direction, negative in the opposite.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VShiftVKind {
    /// Arithmetic left (HVX vaslhv/vaslwv): +amt arithmetic left, -amt arithmetic right.
    AshiftL,
    /// Arithmetic right (HVX vasrhv/vasrwv): +amt arithmetic right, -amt arithmetic left.
    AshiftR,
    /// Logical right (HVX vlsrhv/vlsrwv): +amt logical right, -amt left.
    LshiftR,
}

/// Per-lane operation for the width-general, Arch-aware `OpKind::VLane`.
///
/// Unlike the AVX-oriented `VAdd`/`VAnd`/... (which are register-width-bounded
/// and, for the bitwise ops, virtual-register-only), `VLane` operates on any
/// vector register (including 1024-bit Hexagon HVX V regs) over `lanes`
/// elements of a given `VecElementType`, with signedness selected per op. This
/// is the lifting target for HVX integer elementwise instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VLaneOp {
    /// Wrapping add (signedness-agnostic).
    Add,
    /// Wrapping subtract (signedness-agnostic).
    Sub,
    /// Wrapping multiply, low half (signedness-agnostic for the low bits).
    Mul,
    /// Per-lane minimum (signed if the op's `signed` flag is set).
    Min,
    /// Per-lane maximum (signed if the op's `signed` flag is set).
    Max,
    /// Bitwise AND (element width irrelevant).
    And,
    /// Bitwise OR.
    Or,
    /// Bitwise XOR.
    Xor,
    /// Bitwise AND-NOT: `a & !b`.
    AndNot,
    /// Bitwise OR-NOT: `a | !b`.
    OrNot,
    /// Bitwise NOT of the first operand (second ignored): `!a`.
    Not,
    /// Saturating add (signed/unsigned per the `signed` flag).
    AddSat,
    /// Saturating subtract (signed/unsigned per the `signed` flag).
    SubSat,
    /// Average `(a + b) >> 1` (signed/unsigned per the `signed` flag), truncating.
    Avg,
    /// Rounding average `(a + b + 1) >> 1`.
    AvgRnd,
    /// Absolute difference `|a - b|` (signed/unsigned per the `signed` flag).
    AbsDiff,
}

// ============================================================================
// AVX10 Types
// ============================================================================

/// AVX10 FP16 arithmetic operation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Avx10FP16Op {
    Add,
    Sub,
    Mul,
    Div,
    Sqrt,
    Min,
    Max,
}

/// AVX10 dot product variant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Avx10DotProductKind {
    /// VPDPBUSD: unsigned bytes * signed bytes
    BuSd,
    /// VPDPWSSD: signed words * signed words
    WsSd,
    /// VPDPBSSD: signed bytes * signed bytes (AVX10.2)
    BsSd,
    /// VPDPBSUD: signed bytes * unsigned bytes (AVX10.2)
    BsUd,
    /// VPDPBUUD: unsigned bytes * unsigned bytes (AVX10.2)
    BuUd,
    /// VPDPWSUD: signed words * unsigned words (AVX10.2)
    WsUd,
    /// VPDPWUSD: unsigned words * signed words (AVX10.2)
    WuSd,
    /// VPDPWUUD: unsigned words * unsigned words (AVX10.2)
    WuUd,
}

/// AVX10 instruction encoding info (for roundtrip preservation)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Avx10Encoding {
    /// EVEX map (0F, 0F38, 0F3A)
    pub map: u8,
    /// EVEX.pp (none, 66, F3, F2)
    pub pp: u8,
    /// EVEX.W bit
    pub w: bool,
    /// Opcode byte
    pub opcode: u8,
    /// Vector length (128, 256, 512)
    pub vl: VecWidth,
    /// Opmask register (k0-k7)
    pub mask: Option<u8>,
    /// Zeroing masking
    pub zeroing: bool,
    /// Embedded rounding mode
    pub rounding: Option<u8>,
}

// ============================================================================
// Condition Codes
// ============================================================================

/// Condition code for branches and conditional ops
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    // Unified conditions (both x86 and ARM)
    /// Equal (Z=1)
    Eq,
    /// Not equal (Z=0)
    Ne,

    // Unsigned comparisons
    /// Unsigned less than (C=0 for x86, C=1 inverted for ARM)
    Ult,
    /// Unsigned less or equal
    Ule,
    /// Unsigned greater than
    Ugt,
    /// Unsigned greater or equal
    Uge,

    // Signed comparisons
    /// Signed less than (N != V)
    Slt,
    /// Signed less or equal
    Sle,
    /// Signed greater than
    Sgt,
    /// Signed greater or equal
    Sge,

    // Individual flags
    /// Negative (N=1 / SF=1)
    Negative,
    /// Positive (N=0 / SF=0)
    Positive,
    /// Overflow (V=1 / OF=1)
    Overflow,
    /// No overflow (V=0 / OF=0)
    NoOverflow,

    // x86-specific
    /// Parity even (PF=1)
    Parity,
    /// Parity odd (PF=0)
    NoParity,

    /// Always (unconditional)
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

// ============================================================================
// Virtual Register Allocator
// ============================================================================

/// Virtual register allocator
#[derive(Debug, Default)]
pub struct VRegAllocator {
    next_id: u32,
    /// Mapping from arch registers to current virtual registers
    arch_to_vreg: HashMap<ArchReg, VirtualId>,
}

impl VRegAllocator {
    /// Create a new allocator
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocate a new virtual register
    pub fn alloc(&mut self) -> VReg {
        let id = self.next_id;
        self.next_id += 1;
        VReg::Virtual(VirtualId(id))
    }

    /// Get the current virtual register for an arch register
    pub fn get_arch(&self, reg: ArchReg) -> VReg {
        self.arch_to_vreg
            .get(&reg)
            .map(|id| VReg::Virtual(*id))
            .unwrap_or(VReg::Arch(reg))
    }

    /// Define a new value for an arch register
    pub fn define_arch(&mut self, reg: ArchReg) -> VReg {
        let vreg = self.alloc();
        if let VReg::Virtual(id) = vreg {
            self.arch_to_vreg.insert(reg, id);
        }
        vreg
    }

    /// Reset mappings (for new block entry)
    pub fn reset(&mut self) {
        self.arch_to_vreg.clear();
    }

    /// Current count of allocated registers
    pub fn count(&self) -> u32 {
        self.next_id
    }
}

/// Block ID allocator
#[derive(Debug, Default)]
pub struct BlockIdAllocator {
    next_id: u32,
}

impl BlockIdAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc(&mut self) -> BlockId {
        let id = self.next_id;
        self.next_id += 1;
        BlockId(id)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_width() {
        assert_eq!(OpWidth::W8.bits(), 8);
        assert_eq!(OpWidth::W8.bytes(), 1);
        assert_eq!(OpWidth::W8.mask(), 0xFF);

        assert_eq!(OpWidth::W32.bits(), 32);
        assert_eq!(OpWidth::W32.bytes(), 4);
        assert_eq!(OpWidth::W32.mask(), 0xFFFF_FFFF);

        assert_eq!(OpWidth::W64.sign_bit(), 0x8000_0000_0000_0000);
    }

    #[test]
    fn test_vreg_alloc() {
        let mut alloc = VRegAllocator::new();

        let v0 = alloc.alloc();
        let v1 = alloc.alloc();

        assert!(v0.is_virtual());
        assert!(v1.is_virtual());
        assert_ne!(v0, v1);
        assert_eq!(alloc.count(), 2);
    }

    #[test]
    fn test_condition_invert() {
        assert_eq!(Condition::Eq.invert(), Condition::Ne);
        assert_eq!(Condition::Ne.invert(), Condition::Eq);
        assert_eq!(Condition::Slt.invert(), Condition::Sge);
        assert_eq!(Condition::Ult.invert(), Condition::Uge);
    }

    #[test]
    fn test_address() {
        let r = VReg::virt(0);
        let addr = Address::base_off(r, 16);

        assert_eq!(addr.regs(), vec![r]);
    }
}
