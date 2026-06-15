//! Branch instruction tests for ARM execution.
//!
//! Tests for:
//! - B (Branch)
//! - BL (Branch with Link)
//! - BX (Branch and Exchange)
//! - BLX (Branch with Link and Exchange)

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// B (Branch) Tests
// ============================================================================

#[test]
fn test_branch_forward() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[15] = 0x1000;
    let insn = make_insn(
        rax::arm::Mnemonic::B,
        0xEA000040,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert_eq!(target, 0x1000 + 8 + 0x100);
    } else {
        panic!("Expected branch");
    }
}

#[test]
fn test_branch_backward() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[15] = 0x1000;
    let insn = make_insn(
        rax::arm::Mnemonic::B,
        0xEAFFFFFC,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert_eq!(target, 0x1000u32.wrapping_add(8).wrapping_sub(16));
    } else {
        panic!("Expected branch");
    }
}

#[test]
fn test_branch_conditional_eq() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[15] = 0x1000;
    cpu.cpsr.z = true;
    let insn = make_insn(
        rax::arm::Mnemonic::B,
        0x0A000040,
        Some(rax::arm::Condition::EQ),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Branch(_)));
}

#[test]
fn test_branch_conditional_ne_not_taken() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[15] = 0x1000;
    cpu.cpsr.z = false;
    let insn = make_insn(
        rax::arm::Mnemonic::B,
        0x1A000040,
        Some(rax::arm::Condition::NE),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
}

#[test]
fn test_branch_with_negative_offset() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x10000, 0);
    cpu.regs[15] = 0x10000;
    let insn = make_insn(
        rax::arm::Mnemonic::B,
        0xEAFFEFF0,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert!(target < 0x10000);
    } else {
        panic!("Expected branch");
    }
}

// ============================================================================
// BL (Branch with Link) Tests
// ============================================================================

#[test]
fn test_bl_saves_link() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[15] = 0x1000;
    cpu.regs[14] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::BL,
        0xEB000040,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert_eq!(cpu.regs[14], 0x1004);
    assert!(matches!(result, rax::arm::ExecResult::Branch(_)));
}

#[test]
fn test_bl_link_value() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x2000, 0);
    cpu.regs[15] = 0x2000;
    cpu.regs[14] = 0;
    let insn = make_insn(
        rax::arm::Mnemonic::BL,
        0xEB000020,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert_eq!(cpu.regs[14], 0x2004);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert_eq!(target, 0x2000 + 8 + 0x80);
    } else {
        panic!("Expected branch");
    }
}

// ============================================================================
// BX (Branch and Exchange) Tests
// ============================================================================

#[test]
fn test_bx_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x2000;
    let insn = make_insn(
        rax::arm::Mnemonic::BX,
        0xE12FFF11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert_eq!(target, 0x2000);
        assert!(!cpu.cpsr.t);
    } else {
        panic!("Expected branch");
    }
}

#[test]
fn test_bx_to_thumb() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 0x2001;
    let insn = make_insn(
        rax::arm::Mnemonic::BX,
        0xE12FFF11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert_eq!(target, 0x2000);
        assert!(cpu.cpsr.t);
    } else {
        panic!("Expected branch");
    }
}

#[test]
fn test_bx_preserve_thumb_state() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.cpsr.t = true;
    cpu.regs[1] = 0x3000;
    let insn = make_insn(
        rax::arm::Mnemonic::BX,
        0xE12FFF11,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    if let rax::arm::ExecResult::Branch(target) = result {
        assert_eq!(target, 0x3000);
        assert!(cpu.cpsr.t);
    } else {
        panic!("Expected branch");
    }
}
