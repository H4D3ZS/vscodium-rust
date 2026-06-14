//! Arithmetic instruction tests for ARM execution.
//!
//! Tests for:
//! - ADD, ADC (addition with carry)
//! - SUB, SBC, RSB (subtraction with/without borrow, reverse subtract)
//! - Compare operations (CMP, CMN)

use crate::arm::common::bitops::test_add_with_carry_comprehensive;

fn make_insn(
    mnemonic: rax::arm::Mnemonic,
    raw: u32,
    cond: Option<rax::arm::Condition>,
    sets_flags: bool,
) -> rax::arm::DecodedInsn {
    let mut insn = rax::arm::DecodedInsn::new(mnemonic, rax::arm::ExecutionState::Arm, raw, 4);
    if let Some(c) = cond {
        insn = insn.with_cond(c);
    }
    if sets_flags {
        insn = insn.with_flags();
    }
    insn
}

fn exec_one(
    cpu: &mut rax::arm::Armv7Cpu,
    mem: &mut rax::arm::FlatMemory,
    insn: &rax::arm::DecodedInsn,
) -> rax::arm::ExecResult {
    let mut executor = rax::arm::Executor::new(cpu, mem);
    executor.execute(insn)
}

// ============================================================================
// ADD (Add) Tests
// ============================================================================

#[test]
fn test_add_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 100;
    cpu.regs[2] = 50;
    let insn = make_insn(
        rax::arm::Mnemonic::ADD,
        0xE0810002,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 150);
}

#[test]
fn test_add_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 42;
    let insn = make_insn(
        rax::arm::Mnemonic::ADD,
        0xE280002A,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 100);
}

#[test]
fn test_add_with_carry() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 100;
    cpu.cpsr.c = true;
    let insn = make_insn(
        rax::arm::Mnemonic::ADC,
        0xE2A10032,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 151);
}

#[test]
fn test_add_shifted_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 10;
    cpu.regs[2] = 5;
    let insn = make_insn(
        rax::arm::Mnemonic::ADD,
        0xE0810012,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 30);
}

// ============================================================================
// SUB (Subtract) Tests
// ============================================================================

#[test]
fn test_sub_register() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 100;
    cpu.regs[1] = 30;
    let insn = make_insn(
        rax::arm::Mnemonic::SUB,
        0xE0400001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 70);
}

#[test]
fn test_sub_with_negative_result() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 50;
    let insn = make_insn(
        rax::arm::Mnemonic::SUBS,
        0xE2510064,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 50u32.wrapping_sub(100));
    assert!(cpu.cpsr.n);
    assert!(!cpu.cpsr.z);
    assert!(!cpu.cpsr.c);
}

#[test]
fn test_sub_immediate() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 100;
    let insn = make_insn(
        rax::arm::Mnemonic::SUB,
        0xE2400020,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 80);
}

#[test]
fn test_sbc_subtract_with_carry() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 100;
    cpu.regs[1] = 50;
    cpu.cpsr.c = false;
    let insn = make_insn(
        rax::arm::Mnemonic::SBC,
        0xE0C00001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 49);
}

#[test]
fn test_rsb_reverse_subtract() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[1] = 30;
    let insn = make_insn(
        rax::arm::Mnemonic::RSB,
        0xE2610064,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 70);
}

// ============================================================================
// CMP/CMN (Compare) Tests
// ============================================================================

#[test]
fn test_cmp_equal() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 42;
    let insn = make_insn(
        rax::arm::Mnemonic::CMP,
        0xE350002A,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(cpu.cpsr.z);
    assert!(cpu.cpsr.c);
    assert!(!cpu.cpsr.n);
}

#[test]
fn test_cmp_less_than() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 10;
    let insn = make_insn(
        rax::arm::Mnemonic::CMP,
        0xE3500014,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(!cpu.cpsr.z);
    assert!(!cpu.cpsr.c);
    assert!(cpu.cpsr.n);
}

#[test]
fn test_cmn_compare_negative() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 50;
    cpu.regs[1] = 100;
    let insn = make_insn(
        rax::arm::Mnemonic::CMN,
        0xE1700001,
        Some(rax::arm::Condition::AL),
        true,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert!(!cpu.cpsr.z);
    assert!(cpu.cpsr.c);
    assert!(!cpu.cpsr.n);
}
