//! System instructions: CPUID, RDMSR, WRMSR, LGDT, LIDT, CLI, STI, etc.

mod cache;
mod control_regs;
mod cpuid;
mod descriptor;
mod fence;
mod flags;
mod msr;
mod nop;
mod stack_flags;
mod syscall;
mod sysenter;
mod timing;

// Re-export all instruction functions
pub use cache::*;
pub use control_regs::*;
pub use cpuid::*;
pub use descriptor::*;
pub use fence::*;
pub use flags::*;
pub use msr::*;
pub use nop::*;
pub use stack_flags::*;
pub use syscall::*;
pub use sysenter::*;
pub use timing::*;
