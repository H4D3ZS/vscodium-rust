//! SMIR IR structures.
//!
//! This module defines the hierarchical IR structure:
//! - Module: top-level container
//! - Function: a lifted function
//! - Block: a basic block
//! - Terminator: block terminators (branches, returns, etc.)

use std::collections::HashMap;

use crate::smir::ops::SmirOp;
use crate::smir::types::*;

// ============================================================================
// Module
// ============================================================================

/// Top-level IR module containing all lifted code
#[derive(Clone, Debug)]
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

impl SmirModule {
    /// Create a new empty module
    pub fn new(id: ModuleId, source_arch: SourceArch) -> Self {
        SmirModule {
            id,
            source_arch,
            functions: Vec::new(),
            symbols: HashMap::new(),
            externals: Vec::new(),
            metadata: ModuleMetadata::default(),
        }
    }

    /// Add a function to the module
    pub fn add_function(&mut self, func: SmirFunction) {
        self.functions.push(func);
    }

    /// Find a function by its entry address
    pub fn find_function(&self, addr: GuestAddr) -> Option<&SmirFunction> {
        self.functions
            .iter()
            .find(|f| f.guest_range.0 <= addr && addr < f.guest_range.1)
    }

    /// Find a function by its ID
    pub fn get_function(&self, id: FunctionId) -> Option<&SmirFunction> {
        self.functions.iter().find(|f| f.id == id)
    }
}

/// External reference (import)
#[derive(Clone, Debug)]
pub struct ExternalRef {
    /// Symbol name
    pub name: String,
    /// Expected address (if known)
    pub addr: Option<GuestAddr>,
}

/// Module metadata
#[derive(Clone, Debug, Default)]
pub struct ModuleMetadata {
    /// Guest entry point address
    pub entry_point: Option<GuestAddr>,
    /// Guest stack base (if known)
    pub stack_base: Option<GuestAddr>,
    /// Text section range
    pub text_range: Option<(GuestAddr, GuestAddr)>,
}

// ============================================================================
// Function
// ============================================================================

/// A lifted function
#[derive(Clone, Debug)]
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
    /// Calling convention
    pub calling_convention: CallingConv,
    /// Function attributes
    pub attrs: FunctionAttrs,
}

impl SmirFunction {
    /// Create a new function
    pub fn new(id: FunctionId, entry: BlockId, guest_start: GuestAddr) -> Self {
        SmirFunction {
            id,
            entry,
            blocks: Vec::new(),
            locals: Vec::new(),
            guest_range: (guest_start, guest_start),
            calling_convention: CallingConv::GuestPreserveAll,
            attrs: FunctionAttrs::default(),
        }
    }

    /// Add a block to the function
    pub fn add_block(&mut self, block: SmirBlock) {
        // Update guest range
        if block.guest_pc < self.guest_range.0 {
            self.guest_range.0 = block.guest_pc;
        }
        if block.guest_pc > self.guest_range.1 {
            self.guest_range.1 = block.guest_pc;
        }
        self.blocks.push(block);
    }

    /// Get a block by ID
    pub fn get_block(&self, id: BlockId) -> Option<&SmirBlock> {
        self.blocks.iter().find(|b| b.id == id)
    }

    /// Get a mutable block by ID
    pub fn get_block_mut(&mut self, id: BlockId) -> Option<&mut SmirBlock> {
        self.blocks.iter_mut().find(|b| b.id == id)
    }

    /// Get the entry block
    pub fn entry_block(&self) -> Option<&SmirBlock> {
        self.get_block(self.entry)
    }

    /// Total number of operations across all blocks
    pub fn op_count(&self) -> usize {
        self.blocks.iter().map(|b| b.ops.len()).sum()
    }
}

/// Calling convention
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CallingConv {
    /// Preserve all guest state
    #[default]
    GuestPreserveAll,
    /// x86_64 System V ABI
    X86SysV,
    /// x86_64 Windows ABI
    X86Win64,
    /// AArch64 AAPCS
    Aarch64Aapcs,
    /// Hexagon standard
    HexagonStd,
    /// RISC-V standard
    RiscVStd,
}

/// Local variable slot
#[derive(Clone, Debug)]
pub struct LocalSlot {
    pub id: LocalId,
    pub size: u32,
    pub align: u32,
}

/// Function attributes
#[derive(Clone, Copy, Debug, Default)]
pub struct FunctionAttrs {
    /// May not return (infinite loop, abort)
    pub no_return: bool,
    /// Has no side effects beyond return value
    pub pure_fn: bool,
    /// Entry point of the guest program
    pub is_entry: bool,
    /// Exception handler
    pub is_exception_handler: bool,
}

// ============================================================================
// Block
// ============================================================================

/// A basic block
#[derive(Clone, Debug)]
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

impl SmirBlock {
    /// Create a new block
    pub fn new(id: BlockId, guest_pc: GuestAddr) -> Self {
        SmirBlock {
            id,
            guest_pc,
            phis: Vec::new(),
            ops: Vec::new(),
            terminator: Terminator::Unreachable,
            exec_count: 0,
        }
    }

    /// Add an operation to the block
    pub fn push_op(&mut self, op: SmirOp) {
        self.ops.push(op);
    }

    /// Set the terminator
    pub fn set_terminator(&mut self, term: Terminator) {
        self.terminator = term;
    }

    /// Get successor block IDs
    pub fn successors(&self) -> Vec<BlockId> {
        self.terminator.successors()
    }

    /// Check if block is empty (no ops, unreachable terminator)
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty() && matches!(self.terminator, Terminator::Unreachable)
    }
}

/// Phi node (SSA)
#[derive(Clone, Debug)]
pub struct PhiNode {
    pub dst: VReg,
    pub sources: Vec<(BlockId, VReg)>,
}

// ============================================================================
// Terminator
// ============================================================================

/// Block terminator
#[derive(Clone, Debug)]
pub enum Terminator {
    /// Unconditional branch
    Branch { target: BlockId },

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
        /// Possible targets (for analysis, may be incomplete)
        possible_targets: Vec<BlockId>,
    },

    /// Indirect branch through memory
    IndirectBranchMem {
        addr: Address,
        /// Possible targets (for analysis, may be incomplete)
        possible_targets: Vec<BlockId>,
    },

    /// Function return
    Return { values: Vec<VReg> },

    /// Call with continuation
    Call {
        target: CallTarget,
        args: Vec<VReg>,
        continuation: BlockId,
    },

    /// Tail call (no return)
    TailCall { target: CallTarget, args: Vec<VReg> },

    /// Trap (undefined, breakpoint, etc.)
    Trap { kind: TrapKind },

    /// Unreachable (for dead code)
    Unreachable,
}

impl Terminator {
    /// Get successor block IDs
    pub fn successors(&self) -> Vec<BlockId> {
        match self {
            Terminator::Branch { target } => vec![*target],
            Terminator::CondBranch {
                true_target,
                false_target,
                ..
            } => vec![*true_target, *false_target],
            Terminator::Switch {
                targets, default, ..
            } => {
                let mut v = targets.clone();
                v.push(*default);
                v
            }
            Terminator::IndirectBranch {
                possible_targets, ..
            } => possible_targets.clone(),
            Terminator::IndirectBranchMem {
                possible_targets, ..
            } => possible_targets.clone(),
            Terminator::Call { continuation, .. } => vec![*continuation],
            Terminator::Return { .. }
            | Terminator::TailCall { .. }
            | Terminator::Trap { .. }
            | Terminator::Unreachable => vec![],
        }
    }

    /// Check if this is a return
    pub fn is_return(&self) -> bool {
        matches!(self, Terminator::Return { .. })
    }

    /// Check if this is a trap
    pub fn is_trap(&self) -> bool {
        matches!(self, Terminator::Trap { .. })
    }

    /// Check if this terminates the function (no successors)
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Terminator::Return { .. }
                | Terminator::TailCall { .. }
                | Terminator::Trap { .. }
                | Terminator::Unreachable
        )
    }
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
    /// Indirect call through memory
    IndirectMem(Address),
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
    /// Read timestamp counter
    Rdtsc,
}

/// Trap kinds
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrapKind {
    /// Debug breakpoint
    Breakpoint,
    /// Undefined instruction
    Undefined,
    /// Division by zero
    DivideByZero,
    /// Integer overflow
    Overflow,
    /// Bounds check failure
    Bounds,
    /// Invalid opcode
    InvalidOpcode,
    /// System call
    SystemCall,
    /// Halt and wait for interrupt
    Halt,
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for constructing SMIR functions
pub struct FunctionBuilder {
    func: SmirFunction,
    current_block: BlockId,
    next_op_id: u16,
    block_alloc: BlockIdAllocator,
    vreg_alloc: VRegAllocator,
}

impl FunctionBuilder {
    /// Create a new function builder
    pub fn new(func_id: FunctionId, entry_pc: GuestAddr) -> Self {
        let mut block_alloc = BlockIdAllocator::new();
        let entry = block_alloc.alloc();

        let mut func = SmirFunction::new(func_id, entry, entry_pc);
        func.blocks.push(SmirBlock::new(entry, entry_pc));

        FunctionBuilder {
            func,
            current_block: entry,
            next_op_id: 0,
            block_alloc,
            vreg_alloc: VRegAllocator::new(),
        }
    }

    /// Allocate a new virtual register
    pub fn alloc_vreg(&mut self) -> VReg {
        self.vreg_alloc.alloc()
    }

    /// Create a new block and return its ID
    pub fn create_block(&mut self, guest_pc: GuestAddr) -> BlockId {
        let id = self.block_alloc.alloc();
        self.func.blocks.push(SmirBlock::new(id, guest_pc));
        id
    }

    /// Switch to a different block for appending ops
    pub fn switch_to_block(&mut self, block: BlockId) {
        self.current_block = block;
        self.next_op_id = 0;
    }

    /// Get the current block ID
    pub fn current_block(&self) -> BlockId {
        self.current_block
    }

    /// Push an operation to the current block
    pub fn push_op(&mut self, guest_pc: GuestAddr, kind: crate::smir::ops::OpKind) {
        let op = SmirOp::new(OpId(self.next_op_id), guest_pc, kind);
        self.next_op_id += 1;

        if let Some(block) = self.func.get_block_mut(self.current_block) {
            block.push_op(op);
        }
    }

    /// Set the terminator for the current block
    pub fn set_terminator(&mut self, term: Terminator) {
        if let Some(block) = self.func.get_block_mut(self.current_block) {
            block.set_terminator(term);
        }
    }

    /// Finish building and return the function
    pub fn finish(self) -> SmirFunction {
        self.func
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smir::ops::OpKind;

    #[test]
    fn test_function_builder() {
        let mut builder = FunctionBuilder::new(FunctionId(0), 0x1000);

        let v0 = builder.alloc_vreg();
        let v1 = builder.alloc_vreg();
        let v2 = builder.alloc_vreg();

        builder.push_op(
            0x1000,
            OpKind::Mov {
                dst: v0,
                src: SrcOperand::imm(42),
                width: OpWidth::W64,
            },
        );

        builder.push_op(
            0x1004,
            OpKind::Add {
                dst: v2,
                src1: v0,
                src2: SrcOperand::Reg(v1),
                width: OpWidth::W64,
                flags: crate::smir::flags::FlagUpdate::None,
            },
        );

        builder.set_terminator(Terminator::Return { values: vec![v2] });

        let func = builder.finish();

        assert_eq!(func.blocks.len(), 1);
        assert_eq!(func.blocks[0].ops.len(), 2);
        assert!(func.blocks[0].terminator.is_return());
    }

    #[test]
    fn test_terminator_successors() {
        let term = Terminator::CondBranch {
            cond: VReg::virt(0),
            true_target: BlockId(1),
            false_target: BlockId(2),
        };

        let succs = term.successors();
        assert_eq!(succs.len(), 2);
        assert!(succs.contains(&BlockId(1)));
        assert!(succs.contains(&BlockId(2)));

        let term = Terminator::Return { values: vec![] };
        assert!(term.successors().is_empty());
        assert!(term.is_terminal());
    }
}
