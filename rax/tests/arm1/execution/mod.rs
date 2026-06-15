//! Execution integration tests for ARM instruction execution.
//!
//! Tests for:
//! - Instruction execution flow
//! - Conditional execution
//! - Flag preservation and setting
//! - Complex instruction sequences

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// Conditional Execution Tests
// ============================================================================

#[test]
fn test_conditional_eq_taken() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.z = true;
    cpu.regs[0] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0x03A00001,
        Some(rax::arm::Condition::EQ),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 1);
}

#[test]
fn test_conditional_eq_not_taken() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.z = false;
    cpu.regs[0] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0x03A00001,
        Some(rax::arm::Condition::EQ),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
}

#[test]
fn test_conditional_gt() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.z = false;
    cpu.cpsr.n = false;
    cpu.cpsr.v = false;
    cpu.regs[0] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xC3A00001,
        Some(rax::arm::Condition::GT),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 1);
}

#[test]
fn test_conditional_lt() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.n = true;
    cpu.cpsr.v = false;
    cpu.regs[0] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xB3A00001,
        Some(rax::arm::Condition::LT),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 1);
}

#[test]
fn test_conditional_always() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.z = true;
    cpu.cpsr.n = true;
    cpu.regs[0] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE3A00001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 1);
}

// ============================================================================
// Flag Setting Tests
// ============================================================================

#[test]
fn test_add_with_carry_flag() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xFFFFFFFF;
    cpu.regs[1] = 1;
    let insn = make_insn(
        rax::arm::Mnemonic::ADDS,
        0xE0900001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
    assert!(cpu.cpsr.z);
    assert!(cpu.cpsr.c);
    assert!(!cpu.cpsr.n);
    assert!(!cpu.cpsr.v);
}

#[test]
fn test_sub_with_borrow() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 10;
    cpu.regs[1] = 20;
    let insn = make_insn(
        rax::arm::Mnemonic::SUBS,
        0xE0500001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 10u32.wrapping_sub(20));
    assert!(!cpu.cpsr.z);
    assert!(!cpu.cpsr.c);
    assert!(cpu.cpsr.n);
}

#[test]
fn test_and_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xFF;
    cpu.regs[1] = 0x00;
    let insn = make_insn(
        rax::arm::Mnemonic::ANDS,
        0xE0000001,
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
// Shift Operation Tests
// ============================================================================

#[test]
fn test_shift_left_logical() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 1;
    cpu.regs[1] = 4;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE1A00110,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 1 << 4);
}

#[test]
fn test_shift_right_arithmetic() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0x80000000;
    cpu.regs[1] = 4;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE1A00150,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xF8000000);
}

#[test]
fn test_shift_right_logical() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0x80000000;
    cpu.regs[1] = 4;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE1A00130,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x08000000);
}

#[test]
fn test_rotate_right() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0x12345678;
    cpu.regs[1] = 8;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE1A00170,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x78123456);
}

// ============================================================================
// Instruction Sequence Tests
// ============================================================================

#[test]
fn test_simple_loop() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);

    // Initialize counter
    cpu.regs[0] = 0;
    cpu.regs[1] = 5;

    // MOV R0, #0
    let mov_zero = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE3A00000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let _ = exec_one(&mut cpu, &mut mem, &mov_zero);

    // MOV R1, #5
    let mov_five = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE3A01005,
        Some(rax::arm::Condition::AL),
        false,
    );
    let _ = exec_one(&mut cpu, &mut mem, &mov_five);

    // ADD R0, R0, #1
    let add_one = make_insn(
        rax::arm::Mnemonic::ADD,
        0xE2800001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let _ = exec_one(&mut cpu, &mut mem, &add_one);

    // SUBS R1, R1, #1
    let sub_one = make_insn(
        rax::arm::Mnemonic::SUBS,
        0xE2511001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let _ = exec_one(&mut cpu, &mut mem, &sub_one);

    assert_eq!(cpu.regs[0], 1);
    assert_eq!(cpu.regs[1], 4);
    assert!(!cpu.cpsr.z);
}

#[test]
fn test_memory_operations() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);

    // Store value
    cpu.regs[0] = 0xDEADBEEF;
    cpu.regs[1] = 0x100;
    let str_insn = make_insn(
        rax::arm::Mnemonic::STR,
        0xE5810000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let _ = exec_one(&mut cpu, &mut mem, &str_insn);

    // Load value back
    cpu.regs[0] = 0;
    let ldr_insn = make_insn(
        rax::arm::Mnemonic::LDR,
        0xE5910000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let _ = exec_one(&mut cpu, &mut mem, &ldr_insn);

    assert_eq!(cpu.regs[0], 0xDEADBEEF);
}

#[test]
fn test_arithmetic_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);

    // Test addition overflow
    cpu.regs[0] = 0x7FFFFFFF;
    let add_overflow = make_insn(
        rax::arm::Mnemonic::ADDS,
        0xE2900001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let _ = exec_one(&mut cpu, &mut mem, &add_overflow);

    assert_eq!(cpu.regs[0], 0x80000000);
    assert!(cpu.cpsr.v); // Overflow flag
    assert!(!cpu.cpsr.c); // No carry
    assert!(cpu.cpsr.n); // Negative
    assert!(!cpu.cpsr.z); // Not zero
}
