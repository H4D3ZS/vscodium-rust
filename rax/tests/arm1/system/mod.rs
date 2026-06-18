//! System instruction tests for ARM execution.
//!
//! Tests for:
//! - SVC (Supervisor Call)
//! - MRS (Move PSR to Register)
//! - MSR (Move Register to PSR)
//! - NOP (No Operation)
//! - CPS (Change Processor State)

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// SVC Tests
// ============================================================================

#[test]
fn test_svc_exception() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let insn = make_insn(
        rax::arm::Mnemonic::SVC,
        0xEF000080,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Exception(rax::arm::ExceptionType::SupervisorCall(imm)) = result {
        assert_eq!(imm, 0x80);
    } else {
        panic!("Expected SupervisorCall exception");
    }
}

#[test]
fn test_svc_zero_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let insn = make_insn(
        rax::arm::Mnemonic::SVC,
        0xEF000000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Exception(rax::arm::ExceptionType::SupervisorCall(imm)) = result {
        assert_eq!(imm, 0);
    } else {
        panic!("Expected SupervisorCall exception");
    }
}

#[test]
fn test_svc_large_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let insn = make_insn(
        rax::arm::Mnemonic::SVC,
        0xEF00FFFF,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Exception(rax::arm::ExceptionType::SupervisorCall(imm)) = result {
        assert_eq!(imm, 0xFFFF);
    } else {
        panic!("Expected SupervisorCall exception");
    }
}

// ============================================================================
// NOP Tests
// ============================================================================

#[test]
fn test_nop() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let orig_regs = cpu.regs.clone();
    let insn = make_insn(
        rax::arm::Mnemonic::NOP,
        0xE1A00000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs, orig_regs);
}

#[test]
fn test_nop_preserves_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.n = true;
    cpu.cpsr.z = false;
    cpu.cpsr.c = true;
    cpu.cpsr.v = false;
    let insn = make_insn(
        rax::arm::Mnemonic::NOP,
        0xE1A00000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(cpu.cpsr.n);
    assert!(!cpu.cpsr.z);
    assert!(cpu.cpsr.c);
    assert!(!cpu.cpsr.v);
}

// ============================================================================
// MRS Tests
// ============================================================================

#[test]
fn test_mrs_cpsr() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.n = true;
    cpu.cpsr.z = false;
    cpu.cpsr.c = true;
    cpu.cpsr.v = false;
    let insn = make_insn(
        rax::arm::Mnemonic::MRS,
        0xE10F0000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    let cpsr = cpu.regs[0];
    assert!((cpsr >> 31) & 1 == 1);
    assert!((cpsr >> 30) & 1 == 0);
    assert!((cpsr >> 29) & 1 == 1);
    assert!((cpsr >> 28) & 1 == 0);
}

#[test]
fn test_mrs_cpsr_all_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.n = true;
    cpu.cpsr.z = true;
    cpu.cpsr.c = true;
    cpu.cpsr.v = true;
    cpu.cpsr.q = true;
    let insn = make_insn(
        rax::arm::Mnemonic::MRS,
        0xE10F0000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    let cpsr = cpu.regs[0];
    assert!((cpsr >> 27) & 1 == 1);
}

// ============================================================================
// MOV Tests
// ============================================================================

#[test]
fn test_mov_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xDEADBEEF;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE1A00001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xDEADBEEF);
}

#[test]
fn test_mov_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE3A00042,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x42);
}

#[test]
fn test_mov_shifted_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00000001;
    let insn = make_insn(
        rax::arm::Mnemonic::MOV,
        0xE1A000A1,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x400);
}

#[test]
fn test_mov_sets_flags() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let insn = make_insn(
        rax::arm::Mnemonic::MOVS,
        0xE3B00000,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(cpu.cpsr.z);
    assert!(!cpu.cpsr.n);
}

// ============================================================================
// MVN Tests
// ============================================================================

#[test]
fn test_mvn_not_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x000000FF;
    let insn = make_insn(
        rax::arm::Mnemonic::MVN,
        0xE1E00001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], !0x000000FFu32);
}

#[test]
fn test_mvn_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    let insn = make_insn(
        rax::arm::Mnemonic::MVN,
        0xE3E00000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFFFFF);
}
