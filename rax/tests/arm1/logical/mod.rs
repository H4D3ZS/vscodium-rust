//! Logical instruction tests for ARM execution.
//!
//! Tests for:
//! - AND (bitwise AND)
//! - ORR (bitwise OR)
//! - EOR (bitwise XOR)
//! - BIC (bit clear)
//! - TST (test bits)

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// AND Tests
// ============================================================================

#[test]
fn test_and_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xFF00FF00;
    cpu.regs[2] = 0x0F0F0F0F;
    let insn = make_insn(
        rax::arm::Mnemonic::AND,
        0xE0010002,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x0F000F00);
}

#[test]
fn test_and_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xFFFFFFFF;
    let insn = make_insn(
        rax::arm::Mnemonic::AND,
        0xE20000FF,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x000000FF);
}

#[test]
fn test_and_sets_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0;
    cpu.regs[1] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::ANDS,
        0xE0100001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
    assert!(cpu.cpsr.z);
    assert!(!cpu.cpsr.n);
}

// ============================================================================
// ORR Tests
// ============================================================================

#[test]
fn test_orr_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00FF0000;
    cpu.regs[2] = 0x000000FF;
    let insn = make_insn(
        rax::arm::Mnemonic::ORR,
        0xE1810002,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x00FF00FF);
}

#[test]
fn test_orr_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00FF0000;
    let insn = make_insn(
        rax::arm::Mnemonic::ORR,
        0xE38100FF,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x00FF00FF);
}

#[test]
fn test_orr_with_shift() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 1;
    cpu.regs[2] = 2;
    let insn = make_insn(
        rax::arm::Mnemonic::ORR,
        0xE1810022,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 3);
}

// ============================================================================
// EOR Tests
// ============================================================================

#[test]
fn test_eor_xor() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xAAAAAAAA;
    cpu.regs[2] = 0x55555555;
    let insn = make_insn(
        rax::arm::Mnemonic::EOR,
        0xE0210002,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFFFFFFF);
}

#[test]
fn test_eor_self() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0x12345678;
    let insn = make_insn(
        rax::arm::Mnemonic::EOR,
        0xE0200000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
}

#[test]
fn test_eor_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xFF;
    let insn = make_insn(
        rax::arm::Mnemonic::EOR,
        0xE22000FF,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFFFFF00);
}

// ============================================================================
// BIC Tests
// ============================================================================

#[test]
fn test_bic_bit_clear() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xFFFFFFFF;
    let insn = make_insn(
        rax::arm::Mnemonic::BIC,
        0xE3C100FF,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFFFFF00);
}

#[test]
fn test_bic_clear_multiple_bits() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xFFFF;
    let insn = make_insn(
        rax::arm::Mnemonic::BIC,
        0xE3C00055,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFAA);
}

// ============================================================================
// TST Tests
// ============================================================================

#[test]
fn test_tst_and_test() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0x00000001;
    let insn = make_insn(
        rax::arm::Mnemonic::TST,
        0xE3100001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(!cpu.cpsr.z);
}

#[test]
fn test_tst_no_match() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xF0;
    let insn = make_insn(
        rax::arm::Mnemonic::TST,
        0xE310000F,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(cpu.cpsr.z);
}
