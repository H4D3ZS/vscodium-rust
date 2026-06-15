//! ARM instruction operand definitions.
//!
//! This module defines the various operand types used in ARM instructions.

use super::{ExtendType, ShiftType};

/// An instruction operand.
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    /// General-purpose register (X0-X30, W0-W30, or SP/WSP/XZR/WZR).
    Reg(Register),
    /// Immediate value.
    Imm(Immediate),
    /// Register with shift.
    ShiftedReg(ShiftedRegister),
    /// Register with extend.
    ExtendedReg(ExtendedRegister),
    /// Memory operand (for loads/stores).
    Mem(MemOperand),
    /// PC-relative label/offset.
    Label(i64),
    /// Register list (for LDM/STM/PUSH/POP).
    RegList(RegisterList),
    /// Condition code operand.
    Cond(super::Condition),
    /// System register.
    SysReg(u16),
    /// Barrier option.
    Barrier(BarrierOption),
    /// Prefetch operation.
    Prfop(PrefetchOp),
    /// SIMD/FP register.
    FpReg(FpRegister),
    /// SIMD vector arrangement.
    VecArrangement(VectorArrangement),
}

/// Register kind for distinguishing different register types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum RegisterKind {
    /// General purpose register (X/W).
    #[default]
    Gpr,
    /// Stack pointer.
    Sp,
    /// SVE Z (scalable vector) register.
    SveZ,
    /// SVE P (predicate) register.
    SveP,
}

/// General-purpose register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Register {
    /// Register number (0-30 for X0-X30/W0-W30, 31 for SP/ZR).
    pub num: u8,
    /// Register width (true = 64-bit X, false = 32-bit W).
    pub is_64bit: bool,
    /// Whether this is the stack pointer (when num=31).
    pub is_sp: bool,
    /// Register kind for SVE support (default: Gpr).
    #[allow(dead_code)]
    pub kind: RegisterKind,
}

impl Register {
    /// Create an AArch32 register (32-bit, GPR).
    /// This is a convenience function for the AArch32 decoder.
    pub fn arm32(num: u8) -> Self {
        Register {
            num: num & 0xF,
            is_64bit: false,
            is_sp: num == 13,
            kind: RegisterKind::Gpr,
        }
    }

    /// Create a raw register with the given properties.
    pub fn raw(num: u8, is_64bit: bool, is_sp: bool) -> Self {
        Register {
            num,
            is_64bit,
            is_sp,
            kind: RegisterKind::Gpr,
        }
    }

    /// Create a 64-bit X register.
    pub fn x(num: u8) -> Self {
        Register {
            num: num & 0x1F,
            is_64bit: true,
            is_sp: false,
            kind: RegisterKind::Gpr,
        }
    }

    /// Create a 32-bit W register.
    pub fn w(num: u8) -> Self {
        Register {
            num: num & 0x1F,
            is_64bit: false,
            is_sp: false,
            kind: RegisterKind::Gpr,
        }
    }

    /// Create the stack pointer (SP or WSP).
    pub fn sp(is_64bit: bool) -> Self {
        Register {
            num: 31,
            is_64bit,
            is_sp: true,
            kind: RegisterKind::Sp,
        }
    }

    /// Create the AArch32 stack pointer (r13). Unlike AArch64, where SP is
    /// encoding 31, the 32-bit architecture's SP is general register 13, so
    /// the operand must carry `num = 13` for the AArch32 executor.
    pub fn sp32() -> Self {
        Register {
            num: 13,
            is_64bit: false,
            is_sp: true,
            kind: RegisterKind::Sp,
        }
    }

    /// Create the zero register (XZR or WZR).
    pub fn zr(is_64bit: bool) -> Self {
        Register {
            num: 31,
            is_64bit,
            is_sp: false,
            kind: RegisterKind::Gpr,
        }
    }

    /// Create a register from encoding, treating r31 as SP.
    pub fn with_sp(num: u8, is_64bit: bool) -> Self {
        if num == 31 {
            Self::sp(is_64bit)
        } else {
            Register {
                num,
                is_64bit,
                is_sp: false,
                kind: RegisterKind::Gpr,
            }
        }
    }

    /// Create a register from encoding, treating r31 as ZR.
    pub fn with_zr(num: u8, is_64bit: bool) -> Self {
        Register {
            num: num & 0x1F,
            is_64bit,
            is_sp: false,
            kind: RegisterKind::Gpr,
        }
    }

    /// Create an SVE Z (scalable vector) register.
    pub fn sve_z(num: u8) -> Self {
        Register {
            num: num & 0x1F,
            is_64bit: true,
            is_sp: false,
            kind: RegisterKind::SveZ,
        }
    }

    /// Create an SVE P (predicate) register.
    pub fn sve_p(num: u8) -> Self {
        Register {
            num: num & 0xF, // Only 16 predicate registers
            is_64bit: true,
            is_sp: false,
            kind: RegisterKind::SveP,
        }
    }

    /// Check if this is an SVE Z register.
    pub fn is_sve_z(&self) -> bool {
        self.kind == RegisterKind::SveZ
    }

    /// Check if this is an SVE P register.
    pub fn is_sve_p(&self) -> bool {
        self.kind == RegisterKind::SveP
    }

    /// Get the register name.
    pub fn name(&self) -> String {
        match self.kind {
            RegisterKind::SveZ => format!("z{}", self.num),
            RegisterKind::SveP => format!("p{}", self.num),
            RegisterKind::Sp => {
                if self.is_64bit {
                    "sp".to_string()
                } else {
                    "wsp".to_string()
                }
            }
            RegisterKind::Gpr => {
                if self.num == 31 {
                    if self.is_sp {
                        if self.is_64bit {
                            "sp".to_string()
                        } else {
                            "wsp".to_string()
                        }
                    } else {
                        if self.is_64bit {
                            "xzr".to_string()
                        } else {
                            "wzr".to_string()
                        }
                    }
                } else {
                    if self.is_64bit {
                        format!("x{}", self.num)
                    } else {
                        format!("w{}", self.num)
                    }
                }
            }
        }
    }

    /// Check if this is the zero register.
    pub fn is_zero_reg(&self) -> bool {
        self.num == 31 && !self.is_sp && self.kind == RegisterKind::Gpr
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// AArch32 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Aarch32Register {
    /// Register number (0-15).
    pub num: u8,
}

impl Aarch32Register {
    /// Create an AArch32 register.
    pub fn new(num: u8) -> Self {
        Aarch32Register { num: num & 0xF }
    }

    /// Get the register name.
    pub fn name(&self) -> &'static str {
        match self.num {
            0 => "r0",
            1 => "r1",
            2 => "r2",
            3 => "r3",
            4 => "r4",
            5 => "r5",
            6 => "r6",
            7 => "r7",
            8 => "r8",
            9 => "r9",
            10 => "r10",
            11 => "r11", // Also fp
            12 => "r12", // Also ip
            13 => "sp",
            14 => "lr",
            15 => "pc",
            _ => "??",
        }
    }

    /// Check if this is the stack pointer.
    pub fn is_sp(&self) -> bool {
        self.num == 13
    }

    /// Check if this is the link register.
    pub fn is_lr(&self) -> bool {
        self.num == 14
    }

    /// Check if this is the program counter.
    pub fn is_pc(&self) -> bool {
        self.num == 15
    }
}

impl std::fmt::Display for Aarch32Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Immediate value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Immediate {
    /// The immediate value.
    pub value: i64,
    /// Shift amount for the immediate (e.g., LSL #12).
    pub shift: u8,
}

impl Immediate {
    /// Create a simple immediate.
    pub fn new(value: i64) -> Self {
        Immediate { value, shift: 0 }
    }

    /// Create an immediate with shift.
    pub fn shifted(value: i64, shift: u8) -> Self {
        Immediate { value, shift }
    }

    /// Get the effective value after shift.
    pub fn effective_value(&self) -> i64 {
        self.value << self.shift
    }
}

/// Register with shift applied.
#[derive(Clone, Debug, PartialEq)]
pub struct ShiftedRegister {
    /// The base register.
    pub reg: Register,
    /// The shift type.
    pub shift_type: ShiftType,
    /// The shift amount.
    pub amount: u8,
}

impl ShiftedRegister {
    /// Create a shifted register.
    pub fn new(reg: Register, shift_type: ShiftType, amount: u8) -> Self {
        ShiftedRegister {
            reg,
            shift_type,
            amount,
        }
    }
}

/// Register with extend applied.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtendedRegister {
    /// The base register.
    pub reg: Register,
    /// The extend type.
    pub extend_type: ExtendType,
    /// The shift amount (0-4).
    pub shift: u8,
}

impl ExtendedRegister {
    /// Create an extended register.
    pub fn new(reg: Register, extend_type: ExtendType, shift: u8) -> Self {
        ExtendedRegister {
            reg,
            extend_type,
            shift,
        }
    }
}

/// Memory operand for load/store instructions.
#[derive(Clone, Debug, PartialEq)]
pub struct MemOperand {
    /// Base register.
    pub base: Register,
    /// Offset type.
    pub offset: MemOffset,
    /// Addressing mode.
    pub mode: AddressingMode,
}

/// Memory offset type.
#[derive(Clone, Debug, PartialEq)]
pub enum MemOffset {
    /// No offset.
    None,
    /// Immediate offset.
    Imm(i64),
    /// Register offset.
    Reg(Register),
    /// Shifted register offset.
    ShiftedReg(ShiftedRegister),
    /// Extended register offset.
    ExtendedReg(ExtendedRegister),
}

/// Addressing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressingMode {
    /// [base] or [base, offset]
    Offset,
    /// [base, offset]! (pre-indexed)
    PreIndex,
    /// [base], offset (post-indexed)
    PostIndex,
}

impl MemOperand {
    /// Create a simple base-only memory operand.
    pub fn base(reg: Register) -> Self {
        MemOperand {
            base: reg,
            offset: MemOffset::None,
            mode: AddressingMode::Offset,
        }
    }

    /// Create a memory operand with immediate offset.
    pub fn imm_offset(base: Register, offset: i64) -> Self {
        MemOperand {
            base,
            offset: MemOffset::Imm(offset),
            mode: AddressingMode::Offset,
        }
    }

    /// Alias for imm_offset - create a memory operand with base and offset.
    pub fn base_offset(base: Register, offset: i64) -> Self {
        Self::imm_offset(base, offset)
    }

    /// Create a pre-indexed memory operand.
    pub fn pre_index(base: Register, offset: i64) -> Self {
        MemOperand {
            base,
            offset: MemOffset::Imm(offset),
            mode: AddressingMode::PreIndex,
        }
    }

    /// Create a post-indexed memory operand.
    pub fn post_index(base: Register, offset: i64) -> Self {
        MemOperand {
            base,
            offset: MemOffset::Imm(offset),
            mode: AddressingMode::PostIndex,
        }
    }

    /// Create a memory operand with register offset.
    pub fn reg_offset(base: Register, index: Register) -> Self {
        MemOperand {
            base,
            offset: MemOffset::Reg(index),
            mode: AddressingMode::Offset,
        }
    }
}

/// Register list for multiple-register operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterList {
    /// Bitmask of included registers (bit N = register N).
    pub mask: u16,
}

impl RegisterList {
    /// Create a register list from a bitmask.
    pub fn from_mask(mask: u16) -> Self {
        RegisterList { mask }
    }

    /// Check if a register is in the list.
    pub fn contains(&self, reg: u8) -> bool {
        (self.mask & (1 << reg)) != 0
    }

    /// Get the number of registers in the list.
    pub fn count(&self) -> u32 {
        self.mask.count_ones()
    }

    /// Iterate over the registers in the list.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..16).filter(|&i| self.contains(i))
    }
}

/// Barrier option for DMB/DSB/ISB.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierOption {
    /// Full system barrier.
    SY,
    /// Store-only barrier.
    ST,
    /// Load-only barrier.
    LD,
    /// Inner shareable full barrier.
    ISH,
    /// Inner shareable store barrier.
    ISHST,
    /// Inner shareable load barrier.
    ISHLD,
    /// Non-shareable full barrier.
    NSH,
    /// Non-shareable store barrier.
    NSHST,
    /// Non-shareable load barrier.
    NSHLD,
    /// Outer shareable full barrier.
    OSH,
    /// Outer shareable store barrier.
    OSHST,
    /// Outer shareable load barrier.
    OSHLD,
    /// Raw 4-bit option value.
    Raw(u8),
}

impl BarrierOption {
    /// Decode from 4-bit CRm field.
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0xF {
            0b1111 => BarrierOption::SY,
            0b1110 => BarrierOption::ST,
            0b1101 => BarrierOption::LD,
            0b1011 => BarrierOption::ISH,
            0b1010 => BarrierOption::ISHST,
            0b1001 => BarrierOption::ISHLD,
            0b0111 => BarrierOption::NSH,
            0b0110 => BarrierOption::NSHST,
            0b0101 => BarrierOption::NSHLD,
            0b0011 => BarrierOption::OSH,
            0b0010 => BarrierOption::OSHST,
            0b0001 => BarrierOption::OSHLD,
            n => BarrierOption::Raw(n),
        }
    }

    /// Get the mnemonic for this barrier option.
    pub fn mnemonic(&self) -> &'static str {
        match self {
            BarrierOption::SY => "sy",
            BarrierOption::ST => "st",
            BarrierOption::LD => "ld",
            BarrierOption::ISH => "ish",
            BarrierOption::ISHST => "ishst",
            BarrierOption::ISHLD => "ishld",
            BarrierOption::NSH => "nsh",
            BarrierOption::NSHST => "nshst",
            BarrierOption::NSHLD => "nshld",
            BarrierOption::OSH => "osh",
            BarrierOption::OSHST => "oshst",
            BarrierOption::OSHLD => "oshld",
            BarrierOption::Raw(_) => "#imm",
        }
    }
}

/// Prefetch operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrefetchOp {
    /// Prefetch for load, L1 cache.
    PLDL1KEEP,
    /// Prefetch for load, L1 cache, streaming.
    PLDL1STRM,
    /// Prefetch for load, L2 cache.
    PLDL2KEEP,
    /// Prefetch for load, L2 cache, streaming.
    PLDL2STRM,
    /// Prefetch for load, L3 cache.
    PLDL3KEEP,
    /// Prefetch for load, L3 cache, streaming.
    PLDL3STRM,
    /// Prefetch for store, L1 cache.
    PSTL1KEEP,
    /// Prefetch for store, L1 cache, streaming.
    PSTL1STRM,
    /// Prefetch for store, L2 cache.
    PSTL2KEEP,
    /// Prefetch for store, L2 cache, streaming.
    PSTL2STRM,
    /// Prefetch for store, L3 cache.
    PSTL3KEEP,
    /// Prefetch for store, L3 cache, streaming.
    PSTL3STRM,
    /// Prefetch for instruction, L1 cache.
    PLIL1KEEP,
    /// Prefetch for instruction, L1 cache, streaming.
    PLIL1STRM,
    /// Prefetch for instruction, L2 cache.
    PLIL2KEEP,
    /// Prefetch for instruction, L2 cache, streaming.
    PLIL2STRM,
    /// Prefetch for instruction, L3 cache.
    PLIL3KEEP,
    /// Prefetch for instruction, L3 cache, streaming.
    PLIL3STRM,
    /// Raw 5-bit prefetch operation.
    Raw(u8),
}

impl PrefetchOp {
    /// Decode from 5-bit Rt field.
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0x1F {
            0b00000 => PrefetchOp::PLDL1KEEP,
            0b00001 => PrefetchOp::PLDL1STRM,
            0b00010 => PrefetchOp::PLDL2KEEP,
            0b00011 => PrefetchOp::PLDL2STRM,
            0b00100 => PrefetchOp::PLDL3KEEP,
            0b00101 => PrefetchOp::PLDL3STRM,
            0b01000 => PrefetchOp::PLIL1KEEP,
            0b01001 => PrefetchOp::PLIL1STRM,
            0b01010 => PrefetchOp::PLIL2KEEP,
            0b01011 => PrefetchOp::PLIL2STRM,
            0b01100 => PrefetchOp::PLIL3KEEP,
            0b01101 => PrefetchOp::PLIL3STRM,
            0b10000 => PrefetchOp::PSTL1KEEP,
            0b10001 => PrefetchOp::PSTL1STRM,
            0b10010 => PrefetchOp::PSTL2KEEP,
            0b10011 => PrefetchOp::PSTL2STRM,
            0b10100 => PrefetchOp::PSTL3KEEP,
            0b10101 => PrefetchOp::PSTL3STRM,
            n => PrefetchOp::Raw(n),
        }
    }
}

/// SIMD/FP register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FpRegister {
    /// Register number (0-31).
    pub num: u8,
    /// Register size/type.
    pub size: FpRegSize,
}

/// SIMD/FP register size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FpRegSize {
    /// 8-bit (B register).
    B,
    /// 16-bit (H register).
    H,
    /// 32-bit (S register).
    S,
    /// 64-bit (D register).
    D,
    /// 128-bit (Q register / V register).
    Q,
}

impl FpRegister {
    /// Create a B (8-bit) register.
    pub fn b(num: u8) -> Self {
        FpRegister {
            num: num & 0x1F,
            size: FpRegSize::B,
        }
    }

    /// Create a H (16-bit) register.
    pub fn h(num: u8) -> Self {
        FpRegister {
            num: num & 0x1F,
            size: FpRegSize::H,
        }
    }

    /// Create a S (32-bit) register.
    pub fn s(num: u8) -> Self {
        FpRegister {
            num: num & 0x1F,
            size: FpRegSize::S,
        }
    }

    /// Create a D (64-bit) register.
    pub fn d(num: u8) -> Self {
        FpRegister {
            num: num & 0x1F,
            size: FpRegSize::D,
        }
    }

    /// Create a Q (128-bit) register.
    pub fn q(num: u8) -> Self {
        FpRegister {
            num: num & 0x1F,
            size: FpRegSize::Q,
        }
    }

    /// Get the register name.
    pub fn name(&self) -> String {
        let prefix = match self.size {
            FpRegSize::B => "b",
            FpRegSize::H => "h",
            FpRegSize::S => "s",
            FpRegSize::D => "d",
            FpRegSize::Q => "q",
        };
        format!("{}{}", prefix, self.num)
    }
}

impl std::fmt::Display for FpRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Vector arrangement specifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VectorArrangement {
    /// Register number.
    pub reg: u8,
    /// Element size.
    pub element_size: VectorElementSize,
    /// Number of elements (determined by Q bit and element size).
    pub num_elements: u8,
}

/// Vector element size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VectorElementSize {
    /// 8-bit elements.
    B,
    /// 16-bit elements.
    H,
    /// 32-bit elements.
    S,
    /// 64-bit elements.
    D,
}

impl VectorArrangement {
    /// Get the arrangement suffix (e.g., "8b", "4h", "2s", "2d").
    pub fn suffix(&self) -> String {
        let size_char = match self.element_size {
            VectorElementSize::B => 'b',
            VectorElementSize::H => 'h',
            VectorElementSize::S => 's',
            VectorElementSize::D => 'd',
        };
        format!("{}{}", self.num_elements, size_char)
    }

    /// Get the full register name with arrangement.
    pub fn name(&self) -> String {
        format!("v{}.{}", self.reg, self.suffix())
    }
}

impl std::fmt::Display for VectorArrangement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_names() {
        assert_eq!(Register::x(0).name(), "x0");
        assert_eq!(Register::w(15).name(), "w15");
        assert_eq!(Register::sp(true).name(), "sp");
        assert_eq!(Register::sp(false).name(), "wsp");
        assert_eq!(Register::zr(true).name(), "xzr");
        assert_eq!(Register::zr(false).name(), "wzr");
    }

    #[test]
    fn test_aarch32_register_names() {
        assert_eq!(Aarch32Register::new(0).name(), "r0");
        assert_eq!(Aarch32Register::new(13).name(), "sp");
        assert_eq!(Aarch32Register::new(14).name(), "lr");
        assert_eq!(Aarch32Register::new(15).name(), "pc");
    }

    #[test]
    fn test_immediate() {
        let imm = Immediate::new(42);
        assert_eq!(imm.effective_value(), 42);

        let shifted = Immediate::shifted(1, 12);
        assert_eq!(shifted.effective_value(), 4096);
    }

    #[test]
    fn test_register_list() {
        let list = RegisterList::from_mask(0b0000_0000_0000_1011);
        assert!(list.contains(0));
        assert!(list.contains(1));
        assert!(!list.contains(2));
        assert!(list.contains(3));
        assert_eq!(list.count(), 3);
    }

    #[test]
    fn test_fp_register_names() {
        assert_eq!(FpRegister::s(5).name(), "s5");
        assert_eq!(FpRegister::d(10).name(), "d10");
        assert_eq!(FpRegister::q(31).name(), "q31");
    }
}
