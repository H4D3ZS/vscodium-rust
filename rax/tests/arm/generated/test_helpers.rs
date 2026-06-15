//! A64 test helpers.
//!
//! Common test infrastructure for AArch64 instruction tests.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use rax::arm::{AArch64Config, AArch64Cpu, FlatMemory};

// Re-export types so tests can use them directly
pub use rax::arm::{ArmCpu, CpuExit};

/// Create a test CPU with default configuration
pub fn create_test_cpu() -> AArch64Cpu {
    let memory = FlatMemory::new(0, 0x1000_0000);
    AArch64Cpu::new(AArch64Config::default(), Box::new(memory))
}

/// Write an instruction to memory
pub fn write_insn(cpu: &mut AArch64Cpu, addr: u64, insn: u32) {
    cpu.write_memory(addr, &insn.to_le_bytes()).unwrap();
}

/// Execute instructions until reaching target address
pub fn run_to(cpu: &mut AArch64Cpu, target_pc: u64) -> CpuExit {
    loop {
        let exit = cpu.step().unwrap();
        if cpu.get_pc() >= target_pc || !matches!(exit, CpuExit::Continue) {
            return exit;
        }
    }
}

/// Set a general purpose register (X0-X30)
pub fn set_x(cpu: &mut AArch64Cpu, reg: u8, value: u64) {
    cpu.set_gpr(reg, value);
}

/// Get a general purpose register (X0-X30)
pub fn get_x(cpu: &AArch64Cpu, reg: u8) -> u64 {
    cpu.get_gpr(reg)
}

/// Set the 32-bit view of a register (W0-W30)
pub fn set_w(cpu: &mut AArch64Cpu, reg: u8, value: u32) {
    cpu.set_gpr(reg, value as u64);
}

/// Get the 32-bit view of a register (W0-W30)
pub fn get_w(cpu: &AArch64Cpu, reg: u8) -> u32 {
    cpu.get_gpr(reg) as u32
}

/// Set SIMD register (V0-V31)
pub fn set_qreg(cpu: &mut AArch64Cpu, reg: u8, value: u128) {
    let low = value as u64;
    let high = (value >> 64) as u64;
    cpu.set_simd_reg(reg, low, high).unwrap();
}

/// Get SIMD register (V0-V31)
pub fn get_qreg(cpu: &AArch64Cpu, reg: u8) -> u128 {
    let (low, high) = cpu.get_simd_reg(reg).unwrap();
    (high as u128) << 64 | (low as u128)
}
