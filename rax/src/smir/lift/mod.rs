//! SMIR lifting interfaces.
//!
//! This module provides traits and types for lifting machine code to SMIR.

pub mod aarch64;
pub mod avx10;
pub mod hexagon;
pub mod riscv;
pub mod x86_64;

use std::collections::HashMap;

use crate::smir::ir::{CallTarget, SmirBlock, SmirFunction, Terminator, TrapKind};
use crate::smir::memory::MemoryError;
use crate::smir::ops::SmirOp;
use crate::smir::types::*;

// ============================================================================
// Lifter Trait
// ============================================================================

/// Lifter interface for converting machine code to SMIR
pub trait SmirLifter: Send {
    /// Source architecture
    fn source_arch(&self) -> SourceArch;

    /// Lift a single instruction at the given address
    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError>;

    /// Lift a basic block starting at the given address
    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError>;

    /// Lift a function (all reachable blocks from entry)
    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError>;
}

// ============================================================================
// Lift Result
// ============================================================================

/// Result of lifting a single instruction
#[derive(Clone, Debug)]
pub struct LiftResult {
    /// SMIR operations generated
    pub ops: Vec<SmirOp>,
    /// Number of bytes consumed
    pub bytes_consumed: usize,
    /// Control flow effect
    pub control_flow: ControlFlow,
    /// Branch targets (for block discovery)
    pub branch_targets: Vec<GuestAddr>,
}

impl LiftResult {
    /// Create a new result for a fallthrough instruction
    pub fn fallthrough(ops: Vec<SmirOp>, bytes: usize) -> Self {
        LiftResult {
            ops,
            bytes_consumed: bytes,
            control_flow: ControlFlow::Fallthrough,
            branch_targets: vec![],
        }
    }

    /// Create a new result for a branch instruction
    pub fn branch(ops: Vec<SmirOp>, bytes: usize, target: GuestAddr) -> Self {
        LiftResult {
            ops,
            bytes_consumed: bytes,
            control_flow: ControlFlow::Branch { target },
            branch_targets: vec![target],
        }
    }

    /// Create a new result for a conditional branch
    pub fn cond_branch(
        ops: Vec<SmirOp>,
        bytes: usize,
        cond: Condition,
        target: GuestAddr,
        fallthrough: GuestAddr,
    ) -> Self {
        LiftResult {
            ops,
            bytes_consumed: bytes,
            control_flow: ControlFlow::CondBranch {
                cond,
                target,
                fallthrough,
            },
            branch_targets: vec![target, fallthrough],
        }
    }

    /// Create a new result for a return instruction
    pub fn ret(ops: Vec<SmirOp>, bytes: usize) -> Self {
        LiftResult {
            ops,
            bytes_consumed: bytes,
            control_flow: ControlFlow::Return,
            branch_targets: vec![],
        }
    }
}

// ============================================================================
// Control Flow
// ============================================================================

/// Control flow after an instruction
#[derive(Clone, Debug)]
pub enum ControlFlow {
    /// Continue to next instruction
    Fallthrough,
    /// Continue to next instruction (alias for Fallthrough)
    NextInsn,
    /// Unconditional branch to known address
    Branch { target: GuestAddr },
    /// Unconditional branch (direct target, alias form)
    DirectBranch(GuestAddr),
    /// Conditional branch (condition code based)
    CondBranch {
        cond: Condition,
        target: GuestAddr,
        fallthrough: GuestAddr,
    },
    /// Conditional branch (VReg boolean condition)
    CondBranchReg {
        cond: VReg,
        taken: GuestAddr,
        not_taken: GuestAddr,
    },
    /// Indirect branch (computed target)
    IndirectBranch { target: VReg },
    /// Indirect branch through memory
    IndirectBranchMem { addr: Address },
    /// Function call
    Call { target: CallTarget },
    /// Return from function
    Return,
    /// Trap (exception, undefined)
    Trap { kind: TrapKind },
    /// System call
    Syscall,
}

impl ControlFlow {
    /// Check if this is a block-ending control flow
    pub fn ends_block(&self) -> bool {
        !matches!(self, ControlFlow::Fallthrough | ControlFlow::NextInsn)
    }

    /// Check if this is a function-ending control flow
    pub fn ends_function(&self) -> bool {
        matches!(
            self,
            ControlFlow::Return | ControlFlow::Trap { .. } | ControlFlow::Syscall
        )
    }

    /// Get direct branch targets (for block discovery)
    pub fn direct_targets(&self) -> Vec<GuestAddr> {
        match self {
            ControlFlow::Branch { target } | ControlFlow::DirectBranch(target) => vec![*target],
            ControlFlow::CondBranch {
                target,
                fallthrough,
                ..
            } => vec![*target, *fallthrough],
            ControlFlow::CondBranchReg {
                taken, not_taken, ..
            } => vec![*taken, *not_taken],
            _ => vec![],
        }
    }
}

// ============================================================================
// Lift Context
// ============================================================================

/// Lifting context (shared state across lifting)
pub struct LiftContext {
    /// Source architecture
    pub arch: SourceArch,
    /// Virtual register allocator
    pub vreg_alloc: VRegAllocator,
    /// Block ID allocator
    pub block_alloc: BlockIdAllocator,
    /// Current guest PC
    pub guest_pc: GuestAddr,
    /// Endianness
    pub endian: Endian,
    /// Known function entries (for call resolution)
    pub known_functions: HashMap<GuestAddr, FunctionId>,
    /// Symbol table
    pub symbols: HashMap<GuestAddr, String>,
    /// Lifted blocks cache (guest address -> block ID)
    pub block_cache: HashMap<GuestAddr, BlockId>,
    /// Current extended immediate (Hexagon)
    pub extended_imm: Option<u32>,
}

impl LiftContext {
    /// Create a new context for the given architecture
    pub fn new(arch: SourceArch) -> Self {
        LiftContext {
            arch,
            vreg_alloc: VRegAllocator::new(),
            block_alloc: BlockIdAllocator::new(),
            guest_pc: 0,
            endian: arch.default_endian(),
            known_functions: HashMap::new(),
            symbols: HashMap::new(),
            block_cache: HashMap::new(),
            extended_imm: None,
        }
    }

    /// Allocate a new virtual register
    pub fn alloc_vreg(&mut self) -> VReg {
        self.vreg_alloc.alloc()
    }

    /// Allocate a new block ID
    pub fn alloc_block(&mut self) -> BlockId {
        self.block_alloc.alloc()
    }

    /// Get or create a block ID for a guest address
    pub fn get_or_create_block(&mut self, addr: GuestAddr) -> BlockId {
        if let Some(&id) = self.block_cache.get(&addr) {
            id
        } else {
            let id = self.alloc_block();
            self.block_cache.insert(addr, id);
            id
        }
    }

    /// Set extended immediate (Hexagon constant extender)
    pub fn set_extended_imm(&mut self, value: u32) {
        self.extended_imm = Some(value);
    }

    /// Take extended immediate if set
    pub fn take_extended_imm(&mut self) -> Option<u32> {
        self.extended_imm.take()
    }

    /// Apply extended immediate to a value
    pub fn extend_imm(&mut self, base: i32) -> i32 {
        if let Some(ext) = self.take_extended_imm() {
            // Hexagon extension: upper 26 bits from extender, lower 6 from instruction
            ((ext << 6) as i32) | (base & 0x3F)
        } else {
            base
        }
    }

    /// Get the current virtual register for an architecture register
    pub fn get_arch_reg(&mut self, reg: ArchReg) -> VReg {
        self.vreg_alloc.get_arch(reg)
    }

    /// Define a new value for an architecture register
    pub fn define_arch_reg(&mut self, reg: ArchReg) -> VReg {
        self.vreg_alloc.define_arch(reg)
    }

    /// Allocate the next operation ID
    pub fn next_op_id(&mut self) -> OpId {
        static COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);
        OpId(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

impl Default for LiftContext {
    fn default() -> Self {
        Self::new(SourceArch::X86_64)
    }
}

// ============================================================================
// Lift Error
// ============================================================================

/// Lifting error
#[derive(Clone, Debug)]
pub enum LiftError {
    /// Invalid instruction encoding
    InvalidEncoding { addr: GuestAddr, bytes: Vec<u8> },
    /// Unsupported instruction
    Unsupported { addr: GuestAddr, mnemonic: String },
    /// Memory read error
    MemoryError { addr: GuestAddr, error: MemoryError },
    /// Incomplete instruction (need more bytes)
    Incomplete {
        addr: GuestAddr,
        have: usize,
        need: usize,
    },
    /// Internal lifter error
    Internal(String),
}

impl std::fmt::Display for LiftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiftError::InvalidEncoding { addr, bytes } => {
                write!(f, "invalid encoding at {:#x}: {:02x?}", addr, bytes)
            }
            LiftError::Unsupported { addr, mnemonic } => {
                write!(f, "unsupported instruction at {:#x}: {}", addr, mnemonic)
            }
            LiftError::MemoryError { addr, error } => {
                write!(f, "memory error at {:#x}: {}", addr, error)
            }
            LiftError::Incomplete { addr, have, need } => {
                write!(
                    f,
                    "incomplete instruction at {:#x}: have {} bytes, need {}",
                    addr, have, need
                )
            }
            LiftError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for LiftError {}

// ============================================================================
// Memory Reader Trait
// ============================================================================

/// Memory reader interface for lifting (read-only)
pub trait MemoryReader: Send + Sync {
    fn read(&self, addr: GuestAddr, size: usize) -> Result<Vec<u8>, MemoryError>;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lift_context() {
        let mut ctx = LiftContext::new(SourceArch::Hexagon);

        let v0 = ctx.alloc_vreg();
        let v1 = ctx.alloc_vreg();
        assert_ne!(v0, v1);

        let b0 = ctx.alloc_block();
        let b1 = ctx.alloc_block();
        assert_ne!(b0, b1);

        // Test block caching
        let cached = ctx.get_or_create_block(0x1000);
        let cached2 = ctx.get_or_create_block(0x1000);
        assert_eq!(cached, cached2);
    }

    #[test]
    fn test_extended_imm() {
        let mut ctx = LiftContext::new(SourceArch::Hexagon);

        // Without extension
        assert_eq!(ctx.extend_imm(0x20), 0x20);

        // With extension
        ctx.set_extended_imm(0x12345);
        assert_eq!(ctx.extend_imm(0x20), (0x12345 << 6) as i32 | 0x20);

        // Extension consumed
        assert_eq!(ctx.extend_imm(0x20), 0x20);
    }

    #[test]
    fn test_control_flow() {
        assert!(!ControlFlow::Fallthrough.ends_block());
        assert!(ControlFlow::Branch { target: 0x1000 }.ends_block());
        assert!(ControlFlow::Return.ends_function());
        assert!(!ControlFlow::Branch { target: 0x1000 }.ends_function());
    }
}
