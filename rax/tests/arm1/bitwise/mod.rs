//! Bit manipulation and extension instruction tests for ARM execution.
//!
//! Tests for:
//! - CLZ (Count Leading Zeros)
//! - REV (Byte Reverse)
//! - RBIT (Bit Reverse)
//! - SXTB/SXTH (Sign Extend Byte/Halfword)
//! - UXTB/UXTH (Zero Extend Byte/Halfword)

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// CLZ Tests
// ============================================================================

#[test]
fn test_clz() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00010000;
    let insn = make_insn(
        rax::arm::Mnemonic::CLZ,
        0xE16F0F11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 15);
}

#[test]
fn test_clz_zero() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::CLZ,
        0xE16F0F11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 32);
}

#[test]
fn test_clz_all_ones() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xFFFFFFFF;
    let insn = make_insn(
        rax::arm::Mnemonic::CLZ,
        0xE16F0F11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
}

#[test]
fn test_clz_single_bit() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x80000000;
    let insn = make_insn(
        rax::arm::Mnemonic::CLZ,
        0xE16F0F11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0);
}

// ============================================================================
// REV Tests
// ============================================================================

#[test]
fn test_rev() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x12345678;
    let insn = make_insn(
        rax::arm::Mnemonic::REV,
        0xE6BF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x78563412);
}

#[test]
fn test_rev_word() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00000001;
    let insn = make_insn(
        rax::arm::Mnemonic::REV,
        0xE6BF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x01000000);
}

#[test]
fn test_rev_endianness() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xFF000000;
    let insn = make_insn(
        rax::arm::Mnemonic::REV,
        0xE6BF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x000000FF);
}

// ============================================================================
// REV16 Tests
// ============================================================================

#[test]
fn test_rev16() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x12345678;
    let insn = make_insn(
        rax::arm::Mnemonic::REV16,
        0xE6BF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x34127856);
}

#[test]
fn test_rev16_aabbccdd() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xAABBCCDD;
    let insn = make_insn(
        rax::arm::Mnemonic::REV16,
        0xE6BF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xBBAADDCC);
}

// ============================================================================
// RBIT Tests
// ============================================================================

#[test]
fn test_rbit() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x80000000;
    let insn = make_insn(
        rax::arm::Mnemonic::RBIT,
        0xE6FF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x00000001);
}

#[test]
fn test_rbit_low_bit() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00000001;
    let insn = make_insn(
        rax::arm::Mnemonic::RBIT,
        0xE6FF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x80000000);
}

#[test]
fn test_rbit_pattern() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xF0000000;
    let insn = make_insn(
        rax::arm::Mnemonic::RBIT,
        0xE6FF0F31,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x0000000F);
}

// ============================================================================
// Sign Extension Tests
// ============================================================================

#[test]
fn test_sxtb() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00000080;
    let insn = make_insn(
        rax::arm::Mnemonic::SXTB,
        0xE6AF0071,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFFFFF80);
}

#[test]
fn test_sxtb_positive() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x1234567F;
    let insn = make_insn(
        rax::arm::Mnemonic::SXTB,
        0xE6AF0071,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x0000007F);
}

#[test]
fn test_uxtb() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x12345678;
    let insn = make_insn(
        rax::arm::Mnemonic::UXTB,
        0xE6EF0071,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x78);
}

#[test]
fn test_sxth() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x00008000;
    let insn = make_insn(
        rax::arm::Mnemonic::SXTH,
        0xE6BF0071,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xFFFF8000);
}

#[test]
fn test_sxth_positive() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x12347FFF;
    let insn = make_insn(
        rax::arm::Mnemonic::SXTH,
        0xE6BF0071,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x00007FFF);
}

#[test]
fn test_uxth() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0xDEADBEEF;
    let insn = make_insn(
        rax::arm::Mnemonic::UXTH,
        0xE6FF0071,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x0000BEEF);
}
