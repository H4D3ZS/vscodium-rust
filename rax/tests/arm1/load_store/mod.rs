//! Load/Store instruction tests for ARM execution.
//!
//! Tests for:
//! - LDR (Load Register)
//! - STR (Store Register)
//! - LDM/STM (Load/Store Multiple)
//! - PUSH/POP
//! - Byte and halfword variants

use crate::arm::arithmetic::{exec_one, make_insn};

// ============================================================================
// LDR/STR Tests
// ============================================================================

#[test]
fn test_str_ldr_roundtrip() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xDEADBEEF;
    cpu.regs[1] = 0x100;
    let str_insn = make_insn(
        rax::arm::Mnemonic::STR,
        0xE5810000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &str_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    cpu.regs[0] = 0;
    let ldr_insn = make_insn(
        rax::arm::Mnemonic::LDR,
        0xE5910000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &ldr_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xDEADBEEF);
}

#[test]
fn test_ldr_with_offset() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    mem.write_word(0x110, 0x12345678).unwrap();
    cpu.regs[1] = 0x100;
    let insn = make_insn(
        rax::arm::Mnemonic::LDR,
        0xE5910010,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x12345678);
}

#[test]
fn test_str_immediate_offset() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xCAFEBABE;
    cpu.regs[1] = 0x200;
    let insn = make_insn(
        rax::arm::Mnemonic::STR,
        0xE5800020,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    let val = mem.read_word(0x220).unwrap();
    assert_eq!(val, 0xCAFEBABE);
}

#[test]
fn test_ldr_negative_offset() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    mem.write_word(0x100, 0x11223344).unwrap();
    cpu.regs[1] = 0x110;
    let insn = make_insn(
        rax::arm::Mnemonic::LDR,
        0xE4910010,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x11223344);
}

// ============================================================================
// Byte Load/Store Tests
// ============================================================================

#[test]
fn test_ldrb_byte_load() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    mem.write_word(0x100, 0x04030201).unwrap();
    cpu.regs[1] = 0x100;
    let insn = make_insn(
        rax::arm::Mnemonic::LDRB,
        0xE5D10000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x01);
}

#[test]
fn test_strb_byte_store() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0xAB;
    cpu.regs[1] = 0x100;
    let insn = make_insn(
        rax::arm::Mnemonic::STRB,
        0xE5C00000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    let val = mem.read_byte(0x100).unwrap();
    assert_eq!(val, 0xAB);
}

#[test]
fn test_ldrb_from_middle() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    mem.write_word(0x100, 0x11223344).unwrap();
    cpu.regs[1] = 0x102;
    let insn = make_insn(
        rax::arm::Mnemonic::LDRB,
        0xE5D10000,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x33);
}

// ============================================================================
// Halfword Load/Store Tests
// ============================================================================

#[test]
fn test_strh_ldrh_halfword() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[0] = 0x1234;
    cpu.regs[1] = 0x100;
    let str_insn = make_insn(
        rax::arm::Mnemonic::STRH,
        0xE1C100B0,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &str_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    cpu.regs[0] = 0;
    let ldr_insn = make_insn(
        rax::arm::Mnemonic::LDRH,
        0xE1D100B0,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &ldr_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0x1234);
}

#[test]
fn test_ldrh_immediate_offset() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    mem.write_word(0x110, 0xAABBCCDD).unwrap();
    cpu.regs[1] = 0x100;
    let insn = make_insn(
        rax::arm::Mnemonic::LDRH,
        0xE1D10010,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xCCDD);
}

// ============================================================================
// Sign-Extended Load Tests
// ============================================================================

#[test]
fn test_ldrsb_sign_extend() {
    use rax::arm::execution::sign_extend;
    assert_eq!(sign_extend(0x80, 8), 0xFFFFFF80);
    assert_eq!(sign_extend(0x7F, 8), 0x0000007F);
}

#[test]
fn test_ldrsh_sign_extend_halfword() {
    use rax::arm::execution::sign_extend;
    assert_eq!(sign_extend(0x8000, 16), 0xFFFF8000);
    assert_eq!(sign_extend(0x7FFF, 16), 0x00007FFF);
}

// ============================================================================
// PUSH/POP Tests
// ============================================================================

#[test]
fn test_push_pop_single() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[13] = 0x800;
    cpu.regs[0] = 0xDEADBEEF;
    let push_insn = make_insn(
        rax::arm::Mnemonic::PUSH,
        0xE92D0001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &push_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[13], 0x7FC);
    cpu.regs[0] = 0;
    let pop_insn = make_insn(
        rax::arm::Mnemonic::POP,
        0xE8BD0001,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &pop_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[0], 0xDEADBEEF);
    assert_eq!(cpu.regs[13], 0x800);
}

#[test]
fn test_push_multiple_registers() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x1000, 0);
    cpu.regs[13] = 0x1000;
    cpu.regs[0] = 0x11111111;
    cpu.regs[1] = 0x22222222;
    cpu.regs[2] = 0x33333333;
    let push_insn = make_insn(
        rax::arm::Mnemonic::PUSH,
        0xE92D0007,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &push_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[13], 0x0FF4);
}

#[test]
fn test_pop_multiple_registers() {
    let mut cpu = rax::arm::Armv7Cpu::new();
    let mut mem = rax::arm::FlatMemory::new(0x2000, 0);
    mem.write_word(0x1FFC, 0x11111111).unwrap();
    mem.write_word(0x1FF8, 0x22222222).unwrap();
    mem.write_word(0x1FF4, 0x33333333).unwrap();
    cpu.regs[13] = 0x2000;
    let pop_insn = make_insn(
        rax::arm::Mnemonic::POP,
        0xE8BD0007,
        Some(rax::arm::Condition::AL),
        false,
    );
    let result = exec_one(&mut cpu, &mut mem, &pop_insn);
    assert!(matches!(result, rax::arm::ExecResult::Continue));
    assert_eq!(cpu.regs[13], 0x2000);
}
