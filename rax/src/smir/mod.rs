//! SMIR - Sigma Machine IR
//!
//! This module provides a cross-platform intermediate representation for CPU emulation.
//! It supports lifting from multiple architectures (x86-64, AArch64, Hexagon, RISC-V)
//! and execution via interpretation or JIT compilation.
//!
//! # Architecture
//!
//! ```text
//! ┌────────────┐     ┌────────────┐     ┌────────────┐
//! │  x86-64    │     │  AArch64   │     │  Hexagon   │
//! │  Binary    │     │  Binary    │     │  Binary    │
//! └─────┬──────┘     └─────┬──────┘     └─────┬──────┘
//!       │                  │                  │
//!       ▼                  ▼                  ▼
//! ┌─────────────────────────────────────────────────┐
//! │                    Lifters                       │
//! │  (x86_lift, arm_lift, hexagon_lift, riscv_lift) │
//! └─────────────────────────────────────────────────┘
//!                         │
//!                         ▼
//! ┌─────────────────────────────────────────────────┐
//! │                    SMIR IR                       │
//! │  (SmirModule, SmirFunction, SmirBlock, SmirOp)  │
//! └─────────────────────────────────────────────────┘
//!                         │
//!           ┌─────────────┼─────────────┐
//!           ▼             ▼             ▼
//!     ┌──────────┐  ┌──────────┐  ┌──────────┐
//!     │Interpreter│  │   JIT    │  │ Analysis │
//!     │ (interp) │  │ (future) │  │ (future) │
//!     └──────────┘  └──────────┘  └──────────┘
//! ```
//!
//! # Key Features
//!
//! - **Lazy flag evaluation**: Flags are computed on-demand, critical for x86 performance
//! - **Virtual registers**: SSA-style unlimited registers
//! - **Unified addressing**: Common address modes across architectures
//! - **Memory model**: Support for atomics, exclusive monitors, fences
//!
//! # Example
//!
//! ```ignore
//! use rax::smir::{SmirContext, SmirInterpreter, FlatMemory};
//!
//! // Create execution context
//! let mut ctx = SmirContext::new_x86_64();
//! let mut memory = FlatMemory::new(0x10000);
//!
//! // Load code into memory...
//! memory.load(0, &code_bytes);
//!
//! // Create interpreter and run
//! let mut interp = SmirInterpreter::new(SourceArch::X86_64);
//! ctx.pc = 0x1000;
//! let exit = interp.run(&mut ctx, &mut memory);
//! ```

pub mod context;
pub mod flags;
pub mod interp;
pub mod ir;
pub mod lift;
pub mod lower;
pub mod memory;
pub mod ops;
pub mod opt;
pub mod types;

// Re-export commonly used types
pub use context::{
    Aarch64RegState, ArchRegState, DebugState, ExitReason, HexagonRegState, RiscVRegState,
    SmirContext, VRegFile, X86RegState,
};
pub use flags::{FlagSet, FlagState, FlagUpdate, LazyFlagOp, LazyFlags, MaterializedFlags};
pub use interp::{BlockResult, SmirInterpreter};
pub use ir::{
    CallTarget, CallingConv, FunctionBuilder, PhiNode, RuntimeFunc, SmirBlock, SmirFunction,
    SmirModule, Terminator, TrapKind,
};
pub use lift::aarch64::Aarch64Lifter;
pub use lift::avx10::{Avx10Lifter, EvexPrefix};
pub use lift::hexagon::HexagonLifter;
pub use lift::riscv::RiscVLifter;
pub use lift::x86_64::X86_64Lifter;
pub use lift::{ControlFlow, LiftContext, LiftError, LiftResult, SmirLifter};
pub use lower::avx10::{Avx10Lowerer, EvexEncoder};
pub use lower::regalloc::{PhysReg, RegAlloc, RegLocation};
pub use lower::x86_64::{X86_64Lowerer, X86Cond, X86Emitter};
pub use lower::{
    CodeBuffer, LowerError, LowerResult, RelocKind, RelocTarget, Relocation, RuntimeHelper,
    SmirLowerer,
};
pub use memory::{
    ExclusiveMonitor, FlatMemory, MemoryError, MemoryReader, SmirMemory, bytes_to_u64,
    check_alignment, u64_to_bytes,
};
pub use ops::{OpKind, SmirOp};
pub use opt::{OptLevel, OptStats, optimize_function};
pub use types::{
    Address, ArchReg, ArmReg, AtomicOp, Avx10DotProductKind, Avx10Encoding, Avx10FP16Op, BlockId,
    BlockIdAllocator, Condition, Endian, ExtendOp, FenceKind, FpPrecision, FpRoundMode, FunctionId,
    GuestAddr, HexagonReg, LocalId, MemWidth, MemoryOrder, ModuleId, OpId, OpWidth, RiscVReg,
    ShiftOp, SignExtend, SourceArch, SrcOperand, VLaneOp, VReg, VRegAllocator, VShiftVKind,
    VecCmpCond, VecElementType, VecWidth, VirtualId, X86Reg,
};
