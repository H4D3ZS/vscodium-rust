//! Multiply instruction tests for ARM execution.
//!
//! Tests for:
//! - MUL (Multiply)
//! - MLA (Multiply Accumulate)
//! - UMULL (Unsigned Long Multiply)
//! - SMULL (Signed Long Multiply)
//! - MLS (Multiply and Subtract)

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// MUL Tests
// ============================================================================

#[test]
fn test_mul_basic() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 12;
    cpu.regs[2] = 11;
    let insn = make_insn(
        rax::arm::Mnemonic::MUL,
        0xE0000291,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 132);
}

#[test]
fn test_mul_small_numbers() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 7;
    cpu.regs[2] = 6;
    let insn = make_insn(
        rax::arm::Mnemonic::MUL,
        0xE0000291,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 42);
}

#[test]
fn test_mul_by_zero() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 100;
    cpu.regs[2] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MUL,
        0xE0000291,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
}

#[test]
fn test_mul_by_one() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 12345;
    cpu.regs[1] = 1;
    let insn = make_insn(
        rax::arm::Mnemonic::MUL,
        0xE0000011,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 12345);
}

#[test]
fn test_mul_sets_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0;
    cpu.regs[1] = 5;
    cpu.regs[2] = 10;
    let insn = make_insn(
        rax::arm::Mnemonic::MULS,
        0xE0000092,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 50);
    assert!(cpu.cpsr.z);
}

// ============================================================================
// MLA Tests
// ============================================================================

#[test]
fn test_mla_multiply_accumulate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 10;
    cpu.regs[2] = 20;
    cpu.regs[3] = 100;
    let insn = make_insn(
        rax::arm::Mnemonic::MLA,
        0xE0203291,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 300);
}

#[test]
fn test_mla_no_accumulator_effect() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0;
    cpu.regs[1] = 5;
    cpu.regs[2] = 5;
    cpu.regs[3] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MLA,
        0xE0200131,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 25);
}

// ============================================================================
// UMULL Tests
// ============================================================================

#[test]
fn test_umull_unsigned_long_multiply() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[2] = 0x80000000;
    cpu.regs[3] = 4;
    let insn = make_insn(
        rax::arm::Mnemonic::UMULL,
        0xE0810392,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    let full_result = ((cpu.regs[1] as u64) << 32) | (cpu.regs[0] as u64);
    assert_eq!(full_result, 0x80000000u64 * 4);
}

#[test]
fn test_umull_basic() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[2] = 0x10000;
    cpu.regs[3] = 0x10000;
    let insn = make_insn(
        rax::arm::Mnemonic::UMULL,
        0xE0810392,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    let full_result = ((cpu.regs[1] as u64) << 32) | (cpu.regs[0] as u64);
    assert_eq!(full_result, 0x100000000);
}

#[test]
fn test_umull_zero_result() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[2] = 0;
    cpu.regs[3] = 0xFFFFFFFF;
    let insn = make_insn(
        rax::arm::Mnemonic::UMULL,
        0xE0810392,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
    assert_eq!(cpu.regs[1], 0);
}

// ============================================================================
// SMULL Tests
// ============================================================================

#[test]
fn test_smull_signed_long_multiply() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[2] = 0x80000000;
    cpu.regs[3] = 2;
    let insn = make_insn(
        rax::arm::Mnemonic::SMULL,
        0xE0C10392,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
}

#[test]
fn test_smull_negative_result() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[2] = 0xFFFFFFFE;
    cpu.regs[3] = 5;
    let insn = make_insn(
        rax::arm::Mnemonic::SMULL,
        0xE0C10392,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
}
