//! ARM architecture definitions and ISA support.
//!
//! This module provides comprehensive ARM architecture definitions including:
//! - Hierarchical ISA selection (Profile -> Version -> Extensions)
//! - Execution state management (ARM, Thumb, AArch32, AArch64)
//! - System register encoding/decoding for CP15 and AArch64
//! - Feature flags for optional extensions
//! - Instruction decoding for all ARM execution states
//!
//! # Architecture Hierarchy
//!
//! ARM processors are organized by profile:
//! - **A-profile**: Application processors (Cortex-A, mobile/server)
//! - **R-profile**: Real-time processors (Cortex-R, automotive/industrial)
//! - **M-profile**: Microcontroller processors (Cortex-M, embedded)
//!
//! Each profile has multiple architecture versions (v6, v7, v8, v9) with
//! different mandatory and optional features.
//!
//! # Instruction Decoder
//!
//! The decoder module provides comprehensive instruction decoding for:
//! - AArch64 (A64): 64-bit ARM instructions
//! - AArch32 (A32): 32-bit ARM instructions
//! - Thumb (T16): 16-bit compact instructions
//! - Thumb-2 (T32): Mixed 16/32-bit instructions
//!
//! ```ignore
//! use rax::arm::decoder::{Decoder, DecodedInsn};
//!
//! let decoder = Decoder::new_aarch64();
//! let insn = decoder.decode(&[0x20, 0x00, 0x80, 0xd2]).unwrap(); // mov x0, #1
//! println!("{}: {:?}", insn.mnemonic, insn.operands);
//! ```

pub mod aarch64;
pub mod cortex_m;
pub mod cp15;
pub mod cpu_trait;
pub mod decoder;
pub mod execution;
pub mod features;
pub mod instructions;
pub mod isa;
pub mod memory;
pub mod mmu_v6;
pub mod state;
pub mod sysreg;
pub mod vfp;

pub use decoder::{Condition, DecodeError, DecodedInsn, Decoder, Mnemonic};
pub use execution::{Armv7Cpu, ProcessorMode, Psr};
pub use features::*;
pub use instructions::{ExceptionType, ExecResult, Executor};
pub use isa::*;
pub use state::*;
pub use sysreg::{Aarch64SysReg, Aarch64SysRegEncoding, Cp15Encoding};

// Re-export unified CPU trait and memory subsystem
pub use cpu_trait::{
    AccessType, ArmCpu, ArmError, ArmException, ArmProfile, ArmVersion, CpuExit, DebugEvent,
    MemoryFaultInfo, MemoryFaultType, ProcessorState, WatchpointKind,
};
pub use memory::{
    AccessPermission, ArmMemory, BarrierKind, ExclusiveMonitor, FlatMemory, MemResult,
    MemoryAttributes, MemoryError, MemoryRegion, MemoryRegionType, MmioHandler, Mpu, MpuRegion,
    MpuRegionAttr, MpuType, StandardMemory,
};

// Re-export Cortex-M subsystem
pub use cortex_m::{CortexMCpu, CortexMVariant, Nvic, Scb, SysTick};

// Re-export AArch64 subsystem
pub use aarch64::{
    AArch64Config, AArch64Cpu, ExceptionClass, ExceptionType as Aarch64ExceptionType, Gic,
    GicConfig, GicVersion, Mmu, MmuConfig, SyndromeRegister, SystemRegisterBank, SystemRegisters,
    TranslationFault, TranslationGranule, TranslationRegime,
};
