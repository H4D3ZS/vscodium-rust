//! RISC-V emulator backend: bridges the self-contained [`crate::riscv`]
//! interpreter to the VMM's [`VCpu`](crate::cpu::VCpu) interface.

mod cpu;

pub use cpu::RiscVVcpu;
