//! x86_64 CPU emulator implementation.

mod aes;
pub mod bios;
mod cpu;
mod decoder;
mod dispatch;
pub mod flags;
mod mmu;
mod sha;
mod simd_native;
mod threaded;
pub mod timing;

mod insn;

pub use cpu::{CURRENT_RIP, RIP_HISTORY, RIP_IDX, X86_64Vcpu, get_total_instruction_count};
pub use mmu::{AccessType, Mmu};
