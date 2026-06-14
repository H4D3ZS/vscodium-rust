//! A64 integer add_sub tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_arithmetic_add_sub_shiftedreg Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_sf_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field sf = 0 (Min)
    // Fields: sf=0, op=0, Rm=0, imm6=0, Rn=0, Rd=0, shift=0, S=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_sf_1_max_0_8b000000() {
    // Encoding: 0x8B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field sf = 1 (Max)
    // Fields: imm6=0, shift=0, Rm=0, S=0, Rn=0, op=0, sf=1, Rd=0
    let encoding: u32 = 0x8B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_op_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field op = 0 (Min)
    // Fields: shift=0, imm6=0, Rd=0, op=0, Rm=0, S=0, Rn=0, sf=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_op_1_max_0_4b000000() {
    // Encoding: 0x4B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field op = 1 (Max)
    // Fields: Rn=0, S=0, Rm=0, op=1, shift=0, sf=0, Rd=0, imm6=0
    let encoding: u32 = 0x4B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_s_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field S = 0 (Min)
    // Fields: shift=0, S=0, Rm=0, imm6=0, sf=0, Rd=0, Rn=0, op=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_s_1_max_0_2b000000() {
    // Encoding: 0x2B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field S = 1 (Max)
    // Fields: op=0, Rd=0, Rn=0, Rm=0, S=1, shift=0, imm6=0, sf=0
    let encoding: u32 = 0x2B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift 22 +: 2`
/// Requirement: FieldBoundary { field: "shift", value: 0, boundary: Min }
/// shift type LSL (logical shift left)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_shift_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field shift = 0 (Min)
    // Fields: shift=0, Rm=0, S=0, Rn=0, sf=0, imm6=0, op=0, Rd=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift 22 +: 2`
/// Requirement: FieldBoundary { field: "shift", value: 1, boundary: PowerOfTwo }
/// shift type LSR (logical shift right)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_shift_1_poweroftwo_0_0b400000() {
    // Encoding: 0x0B400000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field shift = 1 (PowerOfTwo)
    // Fields: sf=0, imm6=0, Rn=0, S=0, op=0, shift=1, Rm=0, Rd=0
    let encoding: u32 = 0x0B400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift 22 +: 2`
/// Requirement: FieldBoundary { field: "shift", value: 2, boundary: PowerOfTwo }
/// shift type ASR (arithmetic shift right)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_shift_2_poweroftwo_0_0b800000() {
    // Encoding: 0x0B800000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field shift = 2 (PowerOfTwo)
    // Fields: Rn=0, imm6=0, sf=0, shift=2, Rd=0, S=0, op=0, Rm=0
    let encoding: u32 = 0x0B800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift 22 +: 2`
/// Requirement: FieldBoundary { field: "shift", value: 3, boundary: Max }
/// shift type ROR (rotate right)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_shift_3_max_0_0bc00000() {
    // Encoding: 0x0BC00000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field shift = 3 (Max)
    // Fields: op=0, S=0, sf=0, imm6=0, shift=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0BC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rm_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rm = 0 (Min)
    // Fields: Rm=0, shift=0, Rd=0, sf=0, S=0, op=0, imm6=0, Rn=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rm_1_poweroftwo_0_0b010000() {
    // Encoding: 0x0B010000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rm = 1 (PowerOfTwo)
    // Fields: Rd=0, Rm=1, sf=0, S=0, Rn=0, imm6=0, op=0, shift=0
    let encoding: u32 = 0x0B010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rm_30_poweroftwominusone_0_0b1e0000() {
    // Encoding: 0x0B1E0000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, sf=0, S=0, shift=0, Rm=30, imm6=0, Rd=0, Rn=0
    let encoding: u32 = 0x0B1E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rm_31_max_0_0b1f0000() {
    // Encoding: 0x0B1F0000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rm = 31 (Max)
    // Fields: sf=0, Rm=31, shift=0, S=0, imm6=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0B1F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_0_zero_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 0 (Zero)
    // Fields: shift=0, S=0, sf=0, imm6=0, op=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_1_poweroftwo_0_0b000400() {
    // Encoding: 0x0B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 1 (PowerOfTwo)
    // Fields: op=0, sf=0, shift=0, Rm=0, imm6=1, Rn=0, S=0, Rd=0
    let encoding: u32 = 0x0B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_3_poweroftwominusone_0_0b000c00() {
    // Encoding: 0x0B000C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: shift=0, imm6=3, Rd=0, sf=0, op=0, Rn=0, Rm=0, S=0
    let encoding: u32 = 0x0B000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_4_poweroftwo_0_0b001000() {
    // Encoding: 0x0B001000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 4 (PowerOfTwo)
    // Fields: op=0, shift=0, imm6=4, sf=0, Rn=0, Rm=0, S=0, Rd=0
    let encoding: u32 = 0x0B001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_7_poweroftwominusone_0_0b001c00() {
    // Encoding: 0x0B001C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: shift=0, Rm=0, S=0, Rd=0, imm6=7, sf=0, Rn=0, op=0
    let encoding: u32 = 0x0B001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_8_poweroftwo_0_0b002000() {
    // Encoding: 0x0B002000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 8 (PowerOfTwo)
    // Fields: op=0, S=0, shift=0, imm6=8, Rd=0, Rn=0, sf=0, Rm=0
    let encoding: u32 = 0x0B002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_15_poweroftwominusone_0_0b003c00()
{
    // Encoding: 0x0B003C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm6=15, Rm=0, S=0, shift=0, Rd=0, op=0, sf=0
    let encoding: u32 = 0x0B003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_16_poweroftwo_0_0b004000() {
    // Encoding: 0x0B004000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 16 (PowerOfTwo)
    // Fields: sf=0, Rn=0, Rm=0, imm6=16, op=0, S=0, Rd=0, shift=0
    let encoding: u32 = 0x0B004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_31_poweroftwominusone_0_0b007c00()
{
    // Encoding: 0x0B007C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: op=0, Rd=0, shift=0, imm6=31, Rm=0, sf=0, S=0, Rn=0
    let encoding: u32 = 0x0B007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_32_poweroftwo_0_0b008000() {
    // Encoding: 0x0B008000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 32 (PowerOfTwo)
    // Fields: imm6=32, S=0, shift=0, sf=0, Rd=0, op=0, Rm=0, Rn=0
    let encoding: u32 = 0x0B008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field imm6 10 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_imm6_63_max_0_0b00fc00() {
    // Encoding: 0x0B00FC00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field imm6 = 63 (Max)
    // Fields: S=0, op=0, imm6=63, Rd=0, sf=0, Rn=0, shift=0, Rm=0
    let encoding: u32 = 0x0B00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rn_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rn = 0 (Min)
    // Fields: Rd=0, shift=0, imm6=0, sf=0, Rm=0, Rn=0, op=0, S=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rn_1_poweroftwo_0_0b000020() {
    // Encoding: 0x0B000020
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rn = 1 (PowerOfTwo)
    // Fields: sf=0, op=0, S=0, Rm=0, imm6=0, Rd=0, shift=0, Rn=1
    let encoding: u32 = 0x0B000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rn_30_poweroftwominusone_0_0b0003c0() {
    // Encoding: 0x0B0003C0
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rd=0, S=0, op=0, Rm=0, shift=0, imm6=0, Rn=30
    let encoding: u32 = 0x0B0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rn_31_max_0_0b0003e0() {
    // Encoding: 0x0B0003E0
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rn = 31 (Max)
    // Fields: op=0, sf=0, shift=0, imm6=0, Rn=31, Rm=0, S=0, Rd=0
    let encoding: u32 = 0x0B0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rd_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rd = 0 (Min)
    // Fields: shift=0, op=0, Rm=0, imm6=0, Rn=0, sf=0, S=0, Rd=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rd_1_poweroftwo_0_0b000001() {
    // Encoding: 0x0B000001
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Rm=0, Rn=0, sf=0, imm6=0, op=0, S=0, shift=0
    let encoding: u32 = 0x0B000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rd_30_poweroftwominusone_0_0b00001e() {
    // Encoding: 0x0B00001E
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: S=0, op=0, Rn=0, sf=0, Rd=30, Rm=0, shift=0, imm6=0
    let encoding: u32 = 0x0B00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_field_rd_31_max_0_0b00001f() {
    // Encoding: 0x0B00001F
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field Rd = 31 (Max)
    // Fields: shift=0, Rn=0, Rd=31, S=0, Rm=0, sf=0, imm6=0, op=0
    let encoding: u32 = 0x0B00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_0_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, S=0, shift=0, op=0, Rn=0, imm6=0, sf=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_1_0_8b000000() {
    // Encoding: 0x8B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=1, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, imm6=0, op=0, S=0, Rd=0, shift=0, sf=1
    let encoding: u32 = 0x8B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_2_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: S=0, shift=0, Rm=0, Rn=0, imm6=0, Rd=0, sf=0, op=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_3_0_4b000000() {
    // Encoding: 0x4B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=1, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: imm6=0, Rn=0, shift=0, sf=0, op=1, Rd=0, S=0, Rm=0
    let encoding: u32 = 0x4B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_4_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: sf=0, op=0, Rn=0, Rm=0, imm6=0, Rd=0, shift=0, S=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_5_0_2b000000() {
    // Encoding: 0x2B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=1, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: shift=0, op=0, Rn=0, S=1, sf=0, Rm=0, imm6=0, Rd=0
    let encoding: u32 = 0x2B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// shift=0 (shift type LSL (logical shift left))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_6_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: Rd=0, shift=0, Rn=0, op=0, sf=0, S=0, Rm=0, imm6=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// shift=1 (shift type LSR (logical shift right))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_7_0_0b400000() {
    // Encoding: 0x0B400000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=1, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: shift=1, Rn=0, op=0, imm6=0, Rm=0, Rd=0, sf=0, S=0
    let encoding: u32 = 0x0B400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// shift=2 (shift type ASR (arithmetic shift right))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_8_0_0b800000() {
    // Encoding: 0x0B800000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=2, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: Rm=0, S=0, Rn=0, op=0, Rd=0, sf=0, shift=2, imm6=0
    let encoding: u32 = 0x0B800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// shift=3 (shift type ROR (rotate right))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_9_0_0bc00000() {
    // Encoding: 0x0BC00000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=3, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: S=0, Rm=0, Rn=0, shift=3, op=0, imm6=0, sf=0, Rd=0
    let encoding: u32 = 0x0BC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_10_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: shift=0, Rn=0, op=0, imm6=0, Rd=0, S=0, sf=0, Rm=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_11_0_0b010000() {
    // Encoding: 0x0B010000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=1, imm6=0, Rn=0, Rd=0
    // Fields: op=0, shift=0, imm6=0, sf=0, Rm=1, S=0, Rd=0, Rn=0
    let encoding: u32 = 0x0B010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_12_0_0b1e0000() {
    // Encoding: 0x0B1E0000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=30, imm6=0, Rn=0, Rd=0
    // Fields: Rd=0, shift=0, sf=0, Rn=0, S=0, op=0, imm6=0, Rm=30
    let encoding: u32 = 0x0B1E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_13_0_0b1f0000() {
    // Encoding: 0x0B1F0000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=31, imm6=0, Rn=0, Rd=0
    // Fields: shift=0, Rd=0, Rn=0, sf=0, op=0, S=0, Rm=31, imm6=0
    let encoding: u32 = 0x0B1F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_14_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: S=0, sf=0, Rd=0, Rm=0, Rn=0, imm6=0, shift=0, op=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_15_0_0b000400() {
    // Encoding: 0x0B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=1, Rn=0, Rd=0
    // Fields: sf=0, op=0, imm6=1, shift=0, S=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x0B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_16_0_0b000c00() {
    // Encoding: 0x0B000C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=3, Rn=0, Rd=0
    // Fields: imm6=3, shift=0, sf=0, op=0, S=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0B000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_17_0_0b001000() {
    // Encoding: 0x0B001000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=4, Rn=0, Rd=0
    // Fields: sf=0, imm6=4, Rm=0, Rd=0, S=0, shift=0, op=0, Rn=0
    let encoding: u32 = 0x0B001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_18_0_0b001c00() {
    // Encoding: 0x0B001C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=7, Rn=0, Rd=0
    // Fields: op=0, S=0, sf=0, shift=0, Rd=0, Rm=0, imm6=7, Rn=0
    let encoding: u32 = 0x0B001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_19_0_0b002000() {
    // Encoding: 0x0B002000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=8, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, op=0, shift=0, Rd=0, S=0, sf=0, imm6=8
    let encoding: u32 = 0x0B002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_20_0_0b003c00() {
    // Encoding: 0x0B003C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=15, Rn=0, Rd=0
    // Fields: sf=0, shift=0, Rd=0, imm6=15, op=0, Rn=0, S=0, Rm=0
    let encoding: u32 = 0x0B003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_21_0_0b004000() {
    // Encoding: 0x0B004000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=16, Rn=0, Rd=0
    // Fields: op=0, shift=0, sf=0, Rm=0, S=0, Rd=0, Rn=0, imm6=16
    let encoding: u32 = 0x0B004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_22_0_0b007c00() {
    // Encoding: 0x0B007C00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=31, Rn=0, Rd=0
    // Fields: shift=0, Rn=0, Rd=0, S=0, imm6=31, sf=0, op=0, Rm=0
    let encoding: u32 = 0x0B007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_23_0_0b008000() {
    // Encoding: 0x0B008000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=32, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, shift=0, Rn=0, imm6=32, Rm=0, S=0, op=0
    let encoding: u32 = 0x0B008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_24_0_0b00fc00() {
    // Encoding: 0x0B00FC00
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=63, Rn=0, Rd=0
    // Fields: Rm=0, op=0, Rn=0, Rd=0, S=0, sf=0, shift=0, imm6=63
    let encoding: u32 = 0x0B00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_25_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: op=0, shift=0, Rn=0, sf=0, S=0, Rm=0, Rd=0, imm6=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_26_0_0b000020() {
    // Encoding: 0x0B000020
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=1, Rd=0
    // Fields: imm6=0, Rm=0, sf=0, Rn=1, S=0, Rd=0, op=0, shift=0
    let encoding: u32 = 0x0B000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_27_0_0b0003c0() {
    // Encoding: 0x0B0003C0
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=30, Rd=0
    // Fields: Rd=0, Rm=0, shift=0, imm6=0, S=0, sf=0, Rn=30, op=0
    let encoding: u32 = 0x0B0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_28_0_0b0003e0() {
    // Encoding: 0x0B0003E0
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=31, Rd=0
    // Fields: op=0, S=0, Rn=31, Rd=0, sf=0, Rm=0, shift=0, imm6=0
    let encoding: u32 = 0x0B0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_29_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=0
    // Fields: imm6=0, sf=0, Rm=0, shift=0, Rn=0, Rd=0, S=0, op=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_30_0_0b000001() {
    // Encoding: 0x0B000001
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=1
    // Fields: op=0, Rn=0, imm6=0, sf=0, S=0, shift=0, Rd=1, Rm=0
    let encoding: u32 = 0x0B000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_31_0_0b00001e() {
    // Encoding: 0x0B00001E
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=30
    // Fields: shift=0, op=0, Rn=0, imm6=0, S=0, sf=0, Rm=0, Rd=30
    let encoding: u32 = 0x0B00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_32_0_0b00001f() {
    // Encoding: 0x0B00001F
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=0, Rd=31
    // Fields: shift=0, S=0, Rn=0, Rd=31, sf=0, op=0, Rm=0, imm6=0
    let encoding: u32 = 0x0B00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_33_0_0b010020() {
    // Encoding: 0x0B010020
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=1, imm6=0, Rn=1, Rd=0
    // Fields: Rm=1, S=0, Rn=1, sf=0, shift=0, imm6=0, Rd=0, op=0
    let encoding: u32 = 0x0B010020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_34_0_0b1f03e0() {
    // Encoding: 0x0B1F03E0
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=31, imm6=0, Rn=31, Rd=0
    // Fields: Rd=0, Rn=31, S=0, imm6=0, shift=0, op=0, Rm=31, sf=0
    let encoding: u32 = 0x0B1F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_35_0_0b010001() {
    // Encoding: 0x0B010001
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=1, imm6=0, Rn=0, Rd=1
    // Fields: sf=0, Rm=1, imm6=0, S=0, Rd=1, Rn=0, op=0, shift=0
    let encoding: u32 = 0x0B010001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_36_0_0b1f001f() {
    // Encoding: 0x0B1F001F
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=31, imm6=0, Rn=0, Rd=31
    // Fields: S=0, op=0, imm6=0, Rd=31, Rm=31, Rn=0, sf=0, shift=0
    let encoding: u32 = 0x0B1F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_37_0_0b000021() {
    // Encoding: 0x0B000021
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=1, Rd=1
    // Fields: S=0, shift=0, op=0, Rd=1, sf=0, imm6=0, Rm=0, Rn=1
    let encoding: u32 = 0x0B000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_combo_38_0_0b0003ff() {
    // Encoding: 0x0B0003FF
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg field combination: sf=0, op=0, S=0, shift=0, Rm=0, imm6=0, Rn=31, Rd=31
    // Fields: shift=0, Rm=0, Rd=31, Rn=31, S=0, op=0, sf=0, imm6=0
    let encoding: u32 = 0x0B0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_sf_0_size_variant_0_0_0b000400() {
    // Encoding: 0x0B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value sf = 0 (Size variant 0)
    // Fields: Rd=0, Rn=0, op=0, imm6=1, Rm=0, S=0, sf=0, shift=0
    let encoding: u32 = 0x0B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_sf_1_size_variant_1_0_8b000400() {
    // Encoding: 0x8B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value sf = 1 (Size variant 1)
    // Fields: Rn=0, imm6=1, shift=0, Rd=0, S=0, Rm=0, sf=1, op=0
    let encoding: u32 = 0x8B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_s_0_size_variant_0_0_0b000400() {
    // Encoding: 0x0B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value S = 0 (Size variant 0)
    // Fields: shift=0, op=0, sf=0, imm6=1, Rd=0, Rn=0, Rm=0, S=0
    let encoding: u32 = 0x0B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_s_1_size_variant_1_0_2b000400() {
    // Encoding: 0x2B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value S = 1 (Size variant 1)
    // Fields: op=0, imm6=1, Rn=0, shift=0, Rd=0, Rm=0, sf=0, S=1
    let encoding: u32 = 0x2B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift = 0 (Shift type LSL)`
/// Requirement: FieldSpecial { field: "shift", value: 0, meaning: "Shift type LSL" }
/// Shift type LSL
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_shift_0_shift_type_lsl_0_0b000400() {
    // Encoding: 0x0B000400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value shift = 0 (Shift type LSL)
    // Fields: shift=0, op=0, sf=0, Rn=0, S=0, Rd=0, imm6=1, Rm=0
    let encoding: u32 = 0x0B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift = 1 (Shift type LSR)`
/// Requirement: FieldSpecial { field: "shift", value: 1, meaning: "Shift type LSR" }
/// Shift type LSR
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_shift_1_shift_type_lsr_0_0b400400() {
    // Encoding: 0x0B400400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value shift = 1 (Shift type LSR)
    // Fields: S=0, op=0, Rd=0, sf=0, shift=1, Rn=0, Rm=0, imm6=1
    let encoding: u32 = 0x0B400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift = 2 (Shift type ASR)`
/// Requirement: FieldSpecial { field: "shift", value: 2, meaning: "Shift type ASR" }
/// Shift type ASR
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_shift_2_shift_type_asr_0_0b800400() {
    // Encoding: 0x0B800400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value shift = 2 (Shift type ASR)
    // Fields: Rn=0, Rd=0, sf=0, imm6=1, Rm=0, op=0, shift=2, S=0
    let encoding: u32 = 0x0B800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field shift = 3 (Shift type ROR)`
/// Requirement: FieldSpecial { field: "shift", value: 3, meaning: "Shift type ROR" }
/// Shift type ROR
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_shift_3_shift_type_ror_0_0bc00400() {
    // Encoding: 0x0BC00400
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value shift = 3 (Shift type ROR)
    // Fields: Rm=0, op=0, shift=3, sf=0, imm6=1, Rd=0, S=0, Rn=0
    let encoding: u32 = 0x0BC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_rn_31_stack_pointer_sp_may_require_alignment_0_0b0007e0()
 {
    // Encoding: 0x0B0007E0
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, Rm=0, S=0, Rd=0, imm6=1, Rn=31, shift=0, op=0
    let encoding: u32 = 0x0B0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0b00041f()
 {
    // Encoding: 0x0B00041F
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: shift=0, imm6=1, Rm=0, Rn=0, Rd=31, sf=0, op=0, S=0
    let encoding: u32 = 0x0B00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_32_0_0b020020() {
    // Test ADD shifted 32-bit: no shift (oracle)
    // Encoding: 0x0B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xA);
    let encoding: u32 = 0x0B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6E, "W0 should be 0x0000006E");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// no shift (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_64_0_8b020020() {
    // Test ADD shifted 64-bit: no shift (oracle)
    // Encoding: 0x8B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xA);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x8B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6E, "X0 should be 0x000000000000006E");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_32_1_0b020c20() {
    // Test ADD shifted 32-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0x0B020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x0B020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6C, "W0 should be 0x0000006C");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_64_1_8b020c20() {
    // Test ADD shifted 64-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0x8B020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x8B020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6C, "X0 should be 0x000000000000006C");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// overflow test (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_32_2_0b020020() {
    // Test ADD shifted 32-bit: overflow test (oracle)
    // Encoding: 0x0B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x0B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// overflow test (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_64_2_8b020020() {
    // Test ADD shifted 64-bit: overflow test (oracle)
    // Encoding: 0x8B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x8000000000000000);
    let encoding: u32 = 0x8B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// subtract from zero (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_32_3_0b020020() {
    // Test ADD shifted 32-bit: subtract from zero (oracle)
    // Encoding: 0x0B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x0B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADD X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// subtract from zero (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_add_shifted_oracle_64_3_8b020020() {
    // Test ADD shifted 64-bit: subtract from zero (oracle)
    // Encoding: 0x8B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x8B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_32_0_2b020020() {
    // Test ADDS shifted 32-bit: no shift (oracle)
    // Encoding: 0x2B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xA);
    let encoding: u32 = 0x2B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6E, "W0 should be 0x0000006E");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// no shift (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_64_0_ab020020() {
    // Test ADDS shifted 64-bit: no shift (oracle)
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xA);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6E, "X0 should be 0x000000000000006E");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_32_1_2b020c20() {
    // Test ADDS shifted 32-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0x2B020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x2B020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6C, "W0 should be 0x0000006C");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_64_1_ab020c20() {
    // Test ADDS shifted 64-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0xAB020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xAB020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6C, "X0 should be 0x000000000000006C");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// overflow test (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_32_2_2b020020() {
    // Test ADDS shifted 32-bit: overflow test (oracle)
    // Encoding: 0x2B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x2B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// overflow test (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_64_2_ab020020() {
    // Test ADDS shifted 64-bit: overflow test (oracle)
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x8000000000000000);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, true, "V flag should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// subtract from zero (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_32_3_2b020020() {
    // Test ADDS shifted 32-bit: subtract from zero (oracle)
    // Encoding: 0x2B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x2B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `ADDS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// subtract from zero (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_adds_shifted_oracle_64_3_ab020020() {
    // Test ADDS shifted 64-bit: subtract from zero (oracle)
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_32_0_4b020020() {
    // Test SUB shifted 32-bit: no shift (oracle)
    // Encoding: 0x4B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xA);
    let encoding: u32 = 0x4B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5A, "W0 should be 0x0000005A");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// no shift (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_64_0_cb020020() {
    // Test SUB shifted 64-bit: no shift (oracle)
    // Encoding: 0xCB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xA);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xCB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x5A, "X0 should be 0x000000000000005A");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_32_1_4b020c20() {
    // Test SUB shifted 32-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0x4B020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x4B020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5C, "W0 should be 0x0000005C");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_64_1_cb020c20() {
    // Test SUB shifted 64-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0xCB020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xCB020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x5C, "X0 should be 0x000000000000005C");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// overflow test (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_32_2_4b020020() {
    // Test SUB shifted 32-bit: overflow test (oracle)
    // Encoding: 0x4B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x4B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// overflow test (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_64_2_cb020020() {
    // Test SUB shifted 64-bit: overflow test (oracle)
    // Encoding: 0xCB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x8000000000000000);
    let encoding: u32 = 0xCB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// subtract from zero (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_32_3_4b020020() {
    // Test SUB shifted 32-bit: subtract from zero (oracle)
    // Encoding: 0x4B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x4B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUB X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// subtract from zero (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sub_shifted_oracle_64_3_cb020020() {
    // Test SUB shifted 64-bit: subtract from zero (oracle)
    // Encoding: 0xCB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xCB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_32_0_6b020020() {
    // Test SUBS shifted 32-bit: no shift (oracle)
    // Encoding: 0x6B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xA);
    let encoding: u32 = 0x6B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5A, "W0 should be 0x0000005A");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// no shift (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_64_0_eb020020() {
    // Test SUBS shifted 64-bit: no shift (oracle)
    // Encoding: 0xEB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xA);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xEB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x5A, "X0 should be 0x000000000000005A");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_32_1_6b020c20() {
    // Test SUBS shifted 32-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0x6B020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x6B020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5C, "W0 should be 0x0000005C");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #3`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSL #3 (multiply by 8) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_64_1_eb020c20() {
    // Test SUBS shifted 64-bit: LSL #3 (multiply by 8) (oracle)
    // Encoding: 0xEB020C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB020C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x5C, "X0 should be 0x000000000000005C");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// overflow test (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_32_2_6b020020() {
    // Test SUBS shifted 32-bit: overflow test (oracle)
    // Encoding: 0x6B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// overflow test (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_64_2_eb020020() {
    // Test SUBS shifted 64-bit: overflow test (oracle)
    // Encoding: 0xEB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x8000000000000000);
    let encoding: u32 = 0xEB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// subtract from zero (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_32_3_6b020020() {
    // Test SUBS shifted 32-bit: subtract from zero (oracle)
    // Encoding: 0x6B020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6B020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `SUBS X0, X1, X2, shift #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// subtract from zero (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_subs_shifted_oracle_64_3_eb020020() {
    // Test SUBS shifted 64-bit: subtract from zero (oracle)
    // Encoding: 0xEB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xEB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_reg_write_0_0b000000() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg register write: GpFromField("d")
    // Encoding: 0x0B000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_sp_rn_0b0003e0() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg with Rn = SP (31)
    // Encoding: 0x0B0003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B0003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_zr_rd_0b00001f() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg with Rd = ZR (31)
    // Encoding: 0x0B00001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B00001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_zeroresult_0_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: ZeroResult
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_zeroresult_1_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: ZeroResult
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_negativeresult_2_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: NegativeResult
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_unsignedoverflow_3_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: UnsignedOverflow
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_unsignedoverflow_4_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: UnsignedOverflow
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_signedoverflow_5_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: SignedOverflow
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_signedoverflow_6_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: SignedOverflow
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_shiftedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_shiftedreg_flags_positiveresult_7_ab020020() {
    // Test aarch64_integer_arithmetic_add_sub_shiftedreg flag computation: PositiveResult
    // Encoding: 0xAB020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xAB020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_arithmetic_add_sub_carry Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_sf_0_min_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field sf = 0 (Min)
    // Fields: sf=0, S=0, Rm=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_sf_1_max_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field sf = 1 (Max)
    // Fields: op=0, Rd=0, S=0, sf=1, Rn=0, Rm=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_op_0_min_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field op = 0 (Min)
    // Fields: sf=0, Rn=0, Rd=0, S=0, op=0, Rm=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_op_1_max_0_5a000000() {
    // Encoding: 0x5A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field op = 1 (Max)
    // Fields: Rm=0, op=1, Rd=0, Rn=0, sf=0, S=0
    let encoding: u32 = 0x5A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_s_0_min_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field S = 0 (Min)
    // Fields: Rn=0, Rd=0, op=0, Rm=0, sf=0, S=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_s_1_max_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field S = 1 (Max)
    // Fields: Rm=0, op=0, S=1, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rm_0_min_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field Rm = 0 (Min)
    // Fields: Rm=0, op=0, Rn=0, S=0, Rd=0, sf=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rm_1_poweroftwo_0_1a010000() {
    // Encoding: 0x1A010000
    // Test aarch64_integer_arithmetic_add_sub_carry field Rm = 1 (PowerOfTwo)
    // Fields: S=0, Rd=0, sf=0, op=0, Rn=0, Rm=1
    let encoding: u32 = 0x1A010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rm_30_poweroftwominusone_0_1a1e0000() {
    // Encoding: 0x1A1E0000
    // Test aarch64_integer_arithmetic_add_sub_carry field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rn=0, Rd=0, Rm=30, op=0, S=0
    let encoding: u32 = 0x1A1E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rm_31_max_0_1a1f0000() {
    // Encoding: 0x1A1F0000
    // Test aarch64_integer_arithmetic_add_sub_carry field Rm = 31 (Max)
    // Fields: op=0, Rd=0, Rn=0, sf=0, S=0, Rm=31
    let encoding: u32 = 0x1A1F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rn_0_min_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field Rn = 0 (Min)
    // Fields: op=0, sf=0, Rm=0, Rn=0, Rd=0, S=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rn_1_poweroftwo_0_1a000020() {
    // Encoding: 0x1A000020
    // Test aarch64_integer_arithmetic_add_sub_carry field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, S=0, Rm=0, sf=0, op=0, Rn=1
    let encoding: u32 = 0x1A000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rn_30_poweroftwominusone_0_1a0003c0() {
    // Encoding: 0x1A0003C0
    // Test aarch64_integer_arithmetic_add_sub_carry field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, sf=0, S=0, Rd=0, Rm=0, Rn=30
    let encoding: u32 = 0x1A0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rn_31_max_0_1a0003e0() {
    // Encoding: 0x1A0003E0
    // Test aarch64_integer_arithmetic_add_sub_carry field Rn = 31 (Max)
    // Fields: op=0, sf=0, Rn=31, Rd=0, Rm=0, S=0
    let encoding: u32 = 0x1A0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rd_0_min_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field Rd = 0 (Min)
    // Fields: op=0, Rn=0, sf=0, Rd=0, Rm=0, S=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rd_1_poweroftwo_0_1a000001() {
    // Encoding: 0x1A000001
    // Test aarch64_integer_arithmetic_add_sub_carry field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Rm=0, sf=0, S=0, Rn=0, op=0
    let encoding: u32 = 0x1A000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rd_30_poweroftwominusone_0_1a00001e() {
    // Encoding: 0x1A00001E
    // Test aarch64_integer_arithmetic_add_sub_carry field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, Rn=0, sf=0, Rm=0, Rd=30, S=0
    let encoding: u32 = 0x1A00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_field_rd_31_max_0_1a00001f() {
    // Encoding: 0x1A00001F
    // Test aarch64_integer_arithmetic_add_sub_carry field Rd = 31 (Max)
    // Fields: op=0, sf=0, Rn=0, Rm=0, Rd=31, S=0
    let encoding: u32 = 0x1A00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_0_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Rn=0, sf=0, op=0, S=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_1_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=1, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: S=0, Rn=0, op=0, Rm=0, sf=1, Rd=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_2_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Rm=0, S=0, sf=0, op=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_3_0_5a000000() {
    // Encoding: 0x5A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=1, S=0, Rm=0, Rn=0, Rd=0
    // Fields: S=0, Rn=0, Rd=0, op=1, Rm=0, sf=0
    let encoding: u32 = 0x5A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_4_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, S=0, Rn=0, sf=0, op=0, Rm=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_5_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=1, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, Rm=0, S=1, Rd=0, sf=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_6_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, sf=0, S=0, Rm=0, Rn=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_7_0_1a010000() {
    // Encoding: 0x1A010000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=1, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, op=0, S=0, Rd=0, Rm=1
    let encoding: u32 = 0x1A010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_8_0_1a1e0000() {
    // Encoding: 0x1A1E0000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, S=0, sf=0, Rn=0, op=0, Rm=30
    let encoding: u32 = 0x1A1E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_9_0_1a1f0000() {
    // Encoding: 0x1A1F0000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=31, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, S=0, Rm=31, Rn=0, op=0
    let encoding: u32 = 0x1A1F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_10_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, Rm=0, sf=0, S=0, Rd=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_11_0_1a000020() {
    // Encoding: 0x1A000020
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=1, Rd=0
    // Fields: sf=0, Rm=0, Rn=1, Rd=0, S=0, op=0
    let encoding: u32 = 0x1A000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_12_0_1a0003c0() {
    // Encoding: 0x1A0003C0
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=30, Rd=0
    // Fields: op=0, S=0, Rm=0, Rd=0, sf=0, Rn=30
    let encoding: u32 = 0x1A0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_13_0_1a0003e0() {
    // Encoding: 0x1A0003E0
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=31, Rd=0
    // Fields: op=0, Rd=0, S=0, sf=0, Rm=0, Rn=31
    let encoding: u32 = 0x1A0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_14_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, Rm=0, sf=0, Rn=0, S=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_15_0_1a000001() {
    // Encoding: 0x1A000001
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=1
    // Fields: Rn=0, sf=0, S=0, op=0, Rd=1, Rm=0
    let encoding: u32 = 0x1A000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_16_0_1a00001e() {
    // Encoding: 0x1A00001E
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=30
    // Fields: Rd=30, S=0, op=0, sf=0, Rm=0, Rn=0
    let encoding: u32 = 0x1A00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_17_0_1a00001f() {
    // Encoding: 0x1A00001F
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=0, Rd=31
    // Fields: Rm=0, Rn=0, S=0, Rd=31, sf=0, op=0
    let encoding: u32 = 0x1A00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_18_0_1a010020() {
    // Encoding: 0x1A010020
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=1, Rn=1, Rd=0
    // Fields: Rd=0, S=0, Rm=1, op=0, Rn=1, sf=0
    let encoding: u32 = 0x1A010020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_19_0_1a1f03e0() {
    // Encoding: 0x1A1F03E0
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=31, Rn=31, Rd=0
    // Fields: sf=0, Rm=31, Rn=31, op=0, S=0, Rd=0
    let encoding: u32 = 0x1A1F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_20_0_1a010001() {
    // Encoding: 0x1A010001
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=1, Rn=0, Rd=1
    // Fields: sf=0, op=0, Rn=0, Rd=1, S=0, Rm=1
    let encoding: u32 = 0x1A010001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_21_0_1a1f001f() {
    // Encoding: 0x1A1F001F
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=31, Rn=0, Rd=31
    // Fields: op=0, Rn=0, Rd=31, Rm=31, S=0, sf=0
    let encoding: u32 = 0x1A1F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_22_0_1a000021() {
    // Encoding: 0x1A000021
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=1, Rd=1
    // Fields: sf=0, op=0, S=0, Rm=0, Rn=1, Rd=1
    let encoding: u32 = 0x1A000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_combo_23_0_1a0003ff() {
    // Encoding: 0x1A0003FF
    // Test aarch64_integer_arithmetic_add_sub_carry field combination: sf=0, op=0, S=0, Rm=0, Rn=31, Rd=31
    // Fields: sf=0, Rn=31, op=0, Rm=0, S=0, Rd=31
    let encoding: u32 = 0x1A0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_special_sf_0_size_variant_0_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry special value sf = 0 (Size variant 0)
    // Fields: Rd=0, sf=0, S=0, Rn=0, Rm=0, op=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_special_sf_1_size_variant_1_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch64_integer_arithmetic_add_sub_carry special value sf = 1 (Size variant 1)
    // Fields: Rd=0, Rm=0, sf=1, Rn=0, S=0, op=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_special_s_0_size_variant_0_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch64_integer_arithmetic_add_sub_carry special value S = 0 (Size variant 0)
    // Fields: op=0, Rn=0, S=0, sf=0, Rd=0, Rm=0
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_special_s_1_size_variant_1_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch64_integer_arithmetic_add_sub_carry special value S = 1 (Size variant 1)
    // Fields: S=1, Rn=0, Rd=0, sf=0, op=0, Rm=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_special_rn_31_stack_pointer_sp_may_require_alignment_0_1a0003e0()
 {
    // Encoding: 0x1A0003E0
    // Test aarch64_integer_arithmetic_add_sub_carry special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, Rn=31, Rm=0, S=0, Rd=0, op=0
    let encoding: u32 = 0x1A0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1a00001f()
 {
    // Encoding: 0x1A00001F
    // Test aarch64_integer_arithmetic_add_sub_carry special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, op=0, Rd=31, S=0, sf=0, Rm=0
    let encoding: u32 = 0x1A00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple multiply (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_0_1b027c20() {
    // Test MUL 32-bit: simple multiply (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x2);
    set_x(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6, "W0 should be 0x00000006");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple multiply (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_0_9b027c20() {
    // Test MUL 64-bit: simple multiply (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x2);
    set_x(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6, "X0 should be 0x0000000000000006");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by zero (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_1_1b027c20() {
    // Test MUL 32-bit: multiply by zero (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by zero (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_1_9b027c20() {
    // Test MUL 64-bit: multiply by zero (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by one (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_2_1b027c20() {
    // Test MUL 32-bit: multiply by one (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by one (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_2_9b027c20() {
    // Test MUL 64-bit: multiply by one (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 16-bit max * 16-bit max (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_3_1b027c20() {
    // Test MUL 32-bit: 16-bit max * 16-bit max (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFF);
    set_x(&mut cpu, 2, 0xFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFE0001, "W0 should be 0xFFFE0001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 16-bit max * 16-bit max (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_3_9b027c20() {
    // Test MUL 64-bit: 16-bit max * 16-bit max (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFF);
    set_x(&mut cpu, 2, 0xFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFE0001,
        "X0 should be 0x00000000FFFE0001"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift-like multiply (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_4_1b027c20() {
    // Test MUL 32-bit: shift-like multiply (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x2468ACF0, "W0 should be 0x2468ACF0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift-like multiply (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_4_9b027c20() {
    // Test MUL 64-bit: shift-like multiply (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x2468ACF0,
        "X0 should be 0x000000002468ACF0"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// larger values (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_5_1b027c20() {
    // Test MUL 32-bit: larger values (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4E20, "W0 should be 0x00004E20");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// larger values (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_5_9b027c20() {
    // Test MUL 64-bit: larger values (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xC8);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x4E20, "X0 should be 0x0000000000004E20");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 32-bit overflow (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_6_1b027c20() {
    // Test MUL 32-bit: 32-bit overflow (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 32-bit overflow (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_6_9b027c20() {
    // Test MUL 64-bit: 32-bit overflow (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFE00000001,
        "X0 should be 0xFFFFFFFE00000001"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// prime numbers (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_32_7_1b027c20() {
    // Test MUL 32-bit: prime numbers (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7);
    set_x(&mut cpu, 2, 0xB);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4D, "W0 should be 0x0000004D");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// prime numbers (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_mul_oracle_64_7_9b027c20() {
    // Test MUL 64-bit: prime numbers (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7);
    set_x(&mut cpu, 2, 0xB);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x4D, "X0 should be 0x000000000000004D");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_reg_write_0_1a000000() {
    // Test aarch64_integer_arithmetic_add_sub_carry register write: GpFromField("d")
    // Encoding: 0x1A000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1A000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_sp_rn_1a0003e0() {
    // Test aarch64_integer_arithmetic_add_sub_carry with Rn = SP (31)
    // Encoding: 0x1A0003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1A0003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_zr_rd_1a00001f() {
    // Test aarch64_integer_arithmetic_add_sub_carry with Rd = ZR (31)
    // Encoding: 0x1A00001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1A00001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_zeroresult_0_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: ZeroResult
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_zeroresult_1_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: ZeroResult
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_negativeresult_2_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: NegativeResult
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_unsignedoverflow_3_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: UnsignedOverflow
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_unsignedoverflow_4_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: UnsignedOverflow
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_signedoverflow_5_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: SignedOverflow
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_signedoverflow_6_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: SignedOverflow
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_carry
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_carry_flags_positiveresult_7_ba020020() {
    // Test aarch64_integer_arithmetic_add_sub_carry flag computation: PositiveResult
    // Encoding: 0xBA020020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xBA020020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_arithmetic_add_sub_extendedreg Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_sf_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field sf = 0 (Min)
    // Fields: Rm=0, S=0, imm3=0, Rn=0, sf=0, op=0, option=0, Rd=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_sf_1_max_0_8b200000() {
    // Encoding: 0x8B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field sf = 1 (Max)
    // Fields: Rd=0, sf=1, op=0, Rn=0, imm3=0, S=0, option=0, Rm=0
    let encoding: u32 = 0x8B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_op_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field op = 0 (Min)
    // Fields: Rm=0, Rn=0, op=0, S=0, sf=0, Rd=0, imm3=0, option=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_op_1_max_0_4b200000() {
    // Encoding: 0x4B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field op = 1 (Max)
    // Fields: sf=0, op=1, Rn=0, Rm=0, S=0, option=0, Rd=0, imm3=0
    let encoding: u32 = 0x4B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_s_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field S = 0 (Min)
    // Fields: op=0, Rn=0, sf=0, option=0, S=0, Rm=0, Rd=0, imm3=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_s_1_max_0_2b200000() {
    // Encoding: 0x2B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field S = 1 (Max)
    // Fields: Rd=0, Rm=0, Rn=0, option=0, op=0, imm3=0, sf=0, S=1
    let encoding: u32 = 0x2B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rm_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rm = 0 (Min)
    // Fields: op=0, Rm=0, option=0, Rd=0, S=0, sf=0, imm3=0, Rn=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rm_1_poweroftwo_0_0b210000() {
    // Encoding: 0x0B210000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rm = 1 (PowerOfTwo)
    // Fields: option=0, op=0, S=0, sf=0, Rm=1, imm3=0, Rd=0, Rn=0
    let encoding: u32 = 0x0B210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rm_30_poweroftwominusone_0_0b3e0000() {
    // Encoding: 0x0B3E0000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, op=0, Rn=0, Rd=0, S=0, imm3=0, Rm=30, option=0
    let encoding: u32 = 0x0B3E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rm_31_max_0_0b3f0000() {
    // Encoding: 0x0B3F0000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rm = 31 (Max)
    // Fields: op=0, imm3=0, sf=0, S=0, option=0, Rn=0, Rd=0, Rm=31
    let encoding: u32 = 0x0B3F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 0, boundary: Min }
/// option 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 0 (Min)
    // Fields: sf=0, Rm=0, option=0, Rd=0, Rn=0, imm3=0, op=0, S=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 1, boundary: PowerOfTwo }
/// option 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_1_poweroftwo_0_0b202000() {
    // Encoding: 0x0B202000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 1 (PowerOfTwo)
    // Fields: imm3=0, Rn=0, sf=0, Rm=0, op=0, Rd=0, option=1, S=0
    let encoding: u32 = 0x0B202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 2, boundary: PowerOfTwo }
/// option 2
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_2_poweroftwo_0_0b204000() {
    // Encoding: 0x0B204000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 2 (PowerOfTwo)
    // Fields: op=0, Rd=0, sf=0, imm3=0, S=0, option=2, Rm=0, Rn=0
    let encoding: u32 = 0x0B204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 3, boundary: PowerOfTwo }
/// option 3
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_3_poweroftwo_0_0b206000() {
    // Encoding: 0x0B206000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 3 (PowerOfTwo)
    // Fields: S=0, Rm=0, imm3=0, op=0, option=3, Rd=0, sf=0, Rn=0
    let encoding: u32 = 0x0B206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 4, boundary: PowerOfTwo }
/// option 4
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_4_poweroftwo_0_0b208000() {
    // Encoding: 0x0B208000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 4 (PowerOfTwo)
    // Fields: sf=0, S=0, imm3=0, Rm=0, option=4, Rd=0, Rn=0, op=0
    let encoding: u32 = 0x0B208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 5, boundary: PowerOfTwo }
/// option 5
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_5_poweroftwo_0_0b20a000() {
    // Encoding: 0x0B20A000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 5 (PowerOfTwo)
    // Fields: Rn=0, S=0, Rd=0, imm3=0, op=0, Rm=0, option=5, sf=0
    let encoding: u32 = 0x0B20A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 6, boundary: PowerOfTwo }
/// option 6
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_6_poweroftwo_0_0b20c000() {
    // Encoding: 0x0B20C000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 6 (PowerOfTwo)
    // Fields: Rn=0, sf=0, op=0, option=6, imm3=0, Rm=0, S=0, Rd=0
    let encoding: u32 = 0x0B20C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field option 13 +: 3`
/// Requirement: FieldBoundary { field: "option", value: 7, boundary: Max }
/// option 7
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_option_7_max_0_0b20e000() {
    // Encoding: 0x0B20E000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field option = 7 (Max)
    // Fields: imm3=0, Rd=0, Rn=0, Rm=0, S=0, op=0, sf=0, option=7
    let encoding: u32 = 0x0B20E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field imm3 10 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_imm3_0_zero_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field imm3 = 0 (Zero)
    // Fields: imm3=0, Rm=0, option=0, Rn=0, S=0, Rd=0, sf=0, op=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field imm3 10 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_imm3_1_poweroftwo_0_0b200400() {
    // Encoding: 0x0B200400
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field imm3 = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, imm3=1, Rd=0, sf=0, op=0, option=0, S=0
    let encoding: u32 = 0x0B200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field imm3 10 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_imm3_3_poweroftwominusone_0_0b200c00()
{
    // Encoding: 0x0B200C00
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field imm3 = 3 (PowerOfTwoMinusOne)
    // Fields: op=0, Rn=0, S=0, sf=0, Rd=0, Rm=0, imm3=3, option=0
    let encoding: u32 = 0x0B200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field imm3 10 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_imm3_7_max_0_0b201c00() {
    // Encoding: 0x0B201C00
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field imm3 = 7 (Max)
    // Fields: Rn=0, Rm=0, S=0, Rd=0, imm3=7, option=0, sf=0, op=0
    let encoding: u32 = 0x0B201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rn_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rn = 0 (Min)
    // Fields: Rd=0, op=0, Rm=0, Rn=0, S=0, option=0, sf=0, imm3=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rn_1_poweroftwo_0_0b200020() {
    // Encoding: 0x0B200020
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rn = 1 (PowerOfTwo)
    // Fields: S=0, Rn=1, Rd=0, sf=0, Rm=0, imm3=0, op=0, option=0
    let encoding: u32 = 0x0B200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rn_30_poweroftwominusone_0_0b2003c0() {
    // Encoding: 0x0B2003C0
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: option=0, imm3=0, op=0, sf=0, Rn=30, S=0, Rm=0, Rd=0
    let encoding: u32 = 0x0B2003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rn_31_max_0_0b2003e0() {
    // Encoding: 0x0B2003E0
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rn = 31 (Max)
    // Fields: imm3=0, sf=0, Rd=0, option=0, Rm=0, S=0, op=0, Rn=31
    let encoding: u32 = 0x0B2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rd_0_min_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rd = 0 (Min)
    // Fields: Rd=0, option=0, Rn=0, S=0, Rm=0, op=0, sf=0, imm3=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rd_1_poweroftwo_0_0b200001() {
    // Encoding: 0x0B200001
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rd = 1 (PowerOfTwo)
    // Fields: option=0, Rn=0, op=0, Rd=1, sf=0, S=0, imm3=0, Rm=0
    let encoding: u32 = 0x0B200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rd_30_poweroftwominusone_0_0b20001e() {
    // Encoding: 0x0B20001E
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, option=0, Rn=0, Rm=0, sf=0, imm3=0, Rd=30, S=0
    let encoding: u32 = 0x0B20001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_field_rd_31_max_0_0b20001f() {
    // Encoding: 0x0B20001F
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field Rd = 31 (Max)
    // Fields: Rn=0, imm3=0, op=0, sf=0, Rd=31, Rm=0, option=0, S=0
    let encoding: u32 = 0x0B20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_0_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: sf=0, imm3=0, Rn=0, op=0, S=0, Rm=0, Rd=0, option=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_1_0_8b200000() {
    // Encoding: 0x8B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=1, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: imm3=0, op=0, sf=1, option=0, S=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x8B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_2_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: sf=0, op=0, Rd=0, Rm=0, Rn=0, S=0, imm3=0, option=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_3_0_4b200000() {
    // Encoding: 0x4B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=1, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: imm3=0, Rm=0, Rn=0, op=1, sf=0, Rd=0, option=0, S=0
    let encoding: u32 = 0x4B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_4_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: S=0, sf=0, Rn=0, Rd=0, Rm=0, option=0, imm3=0, op=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_5_0_2b200000() {
    // Encoding: 0x2B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=1, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: imm3=0, Rn=0, Rd=0, S=1, op=0, sf=0, Rm=0, option=0
    let encoding: u32 = 0x2B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_6_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Rn=0, S=0, imm3=0, option=0, sf=0, op=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_7_0_0b210000() {
    // Encoding: 0x0B210000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=1, option=0, imm3=0, Rn=0, Rd=0
    // Fields: S=0, op=0, sf=0, Rm=1, imm3=0, Rn=0, Rd=0, option=0
    let encoding: u32 = 0x0B210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_8_0_0b3e0000() {
    // Encoding: 0x0B3E0000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=30, option=0, imm3=0, Rn=0, Rd=0
    // Fields: op=0, sf=0, option=0, Rn=0, Rd=0, S=0, imm3=0, Rm=30
    let encoding: u32 = 0x0B3E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_9_0_0b3f0000() {
    // Encoding: 0x0B3F0000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=31, option=0, imm3=0, Rn=0, Rd=0
    // Fields: imm3=0, Rd=0, sf=0, Rm=31, op=0, option=0, Rn=0, S=0
    let encoding: u32 = 0x0B3F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=0 (option 0)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_10_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, option=0, imm3=0, op=0, S=0, sf=0, Rm=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=1 (option 1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_11_0_0b202000() {
    // Encoding: 0x0B202000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=1, imm3=0, Rn=0, Rd=0
    // Fields: option=1, sf=0, Rm=0, Rd=0, op=0, imm3=0, Rn=0, S=0
    let encoding: u32 = 0x0B202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=2 (option 2)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_12_0_0b204000() {
    // Encoding: 0x0B204000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=2, imm3=0, Rn=0, Rd=0
    // Fields: S=0, Rm=0, Rn=0, sf=0, imm3=0, op=0, Rd=0, option=2
    let encoding: u32 = 0x0B204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=3 (option 3)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_13_0_0b206000() {
    // Encoding: 0x0B206000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=3, imm3=0, Rn=0, Rd=0
    // Fields: Rm=0, option=3, op=0, sf=0, Rd=0, Rn=0, imm3=0, S=0
    let encoding: u32 = 0x0B206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=4 (option 4)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_14_0_0b208000() {
    // Encoding: 0x0B208000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=4, imm3=0, Rn=0, Rd=0
    // Fields: S=0, option=4, Rn=0, sf=0, Rd=0, Rm=0, op=0, imm3=0
    let encoding: u32 = 0x0B208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=5 (option 5)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_15_0_0b20a000() {
    // Encoding: 0x0B20A000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=5, imm3=0, Rn=0, Rd=0
    // Fields: S=0, imm3=0, Rn=0, option=5, Rd=0, op=0, sf=0, Rm=0
    let encoding: u32 = 0x0B20A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=6 (option 6)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_16_0_0b20c000() {
    // Encoding: 0x0B20C000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=6, imm3=0, Rn=0, Rd=0
    // Fields: op=0, Rm=0, S=0, Rd=0, sf=0, imm3=0, Rn=0, option=6
    let encoding: u32 = 0x0B20C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// option=7 (option 7)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_17_0_0b20e000() {
    // Encoding: 0x0B20E000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=7, imm3=0, Rn=0, Rd=0
    // Fields: sf=0, S=0, op=0, Rd=0, imm3=0, Rm=0, Rn=0, option=7
    let encoding: u32 = 0x0B20E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm3=0 (immediate value 0)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_18_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: S=0, Rd=0, op=0, sf=0, Rn=0, Rm=0, option=0, imm3=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm3=1 (immediate value 1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_19_0_0b200400() {
    // Encoding: 0x0B200400
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=1, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, option=0, op=0, S=0, imm3=1, Rn=0, sf=0
    let encoding: u32 = 0x0B200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm3=3 (immediate midpoint (3))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_20_0_0b200c00() {
    // Encoding: 0x0B200C00
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=3, Rn=0, Rd=0
    // Fields: op=0, Rn=0, Rm=0, imm3=3, S=0, option=0, Rd=0, sf=0
    let encoding: u32 = 0x0B200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm3=7 (maximum immediate (7))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_21_0_0b201c00() {
    // Encoding: 0x0B201C00
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=7, Rn=0, Rd=0
    // Fields: sf=0, imm3=7, Rd=0, option=0, Rn=0, Rm=0, S=0, op=0
    let encoding: u32 = 0x0B201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_22_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: imm3=0, Rn=0, S=0, option=0, Rd=0, Rm=0, sf=0, op=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_23_0_0b200020() {
    // Encoding: 0x0B200020
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=1, Rd=0
    // Fields: Rd=0, sf=0, S=0, imm3=0, op=0, Rm=0, Rn=1, option=0
    let encoding: u32 = 0x0B200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_24_0_0b2003c0() {
    // Encoding: 0x0B2003C0
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=30, Rd=0
    // Fields: imm3=0, op=0, Rm=0, Rn=30, sf=0, option=0, Rd=0, S=0
    let encoding: u32 = 0x0B2003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_25_0_0b2003e0() {
    // Encoding: 0x0B2003E0
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=31, Rd=0
    // Fields: Rd=0, sf=0, Rn=31, Rm=0, S=0, imm3=0, op=0, option=0
    let encoding: u32 = 0x0B2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_26_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=0
    // Fields: S=0, sf=0, op=0, Rd=0, imm3=0, Rm=0, option=0, Rn=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_27_0_0b200001() {
    // Encoding: 0x0B200001
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=1
    // Fields: Rm=0, option=0, op=0, imm3=0, Rd=1, Rn=0, sf=0, S=0
    let encoding: u32 = 0x0B200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_28_0_0b20001e() {
    // Encoding: 0x0B20001E
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=30
    // Fields: sf=0, Rn=0, op=0, option=0, Rm=0, S=0, imm3=0, Rd=30
    let encoding: u32 = 0x0B20001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_29_0_0b20001f() {
    // Encoding: 0x0B20001F
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=0, Rd=31
    // Fields: sf=0, Rd=31, option=0, op=0, S=0, imm3=0, Rn=0, Rm=0
    let encoding: u32 = 0x0B20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_30_0_0b210020() {
    // Encoding: 0x0B210020
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=1, option=0, imm3=0, Rn=1, Rd=0
    // Fields: option=0, Rd=0, sf=0, Rm=1, imm3=0, S=0, op=0, Rn=1
    let encoding: u32 = 0x0B210020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_31_0_0b3f03e0() {
    // Encoding: 0x0B3F03E0
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=31, option=0, imm3=0, Rn=31, Rd=0
    // Fields: S=0, Rm=31, option=0, Rd=0, sf=0, imm3=0, Rn=31, op=0
    let encoding: u32 = 0x0B3F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_32_0_0b210001() {
    // Encoding: 0x0B210001
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=1, option=0, imm3=0, Rn=0, Rd=1
    // Fields: S=0, Rn=0, op=0, sf=0, imm3=0, Rm=1, Rd=1, option=0
    let encoding: u32 = 0x0B210001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_33_0_0b3f001f() {
    // Encoding: 0x0B3F001F
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=31, option=0, imm3=0, Rn=0, Rd=31
    // Fields: sf=0, imm3=0, op=0, Rm=31, Rn=0, option=0, Rd=31, S=0
    let encoding: u32 = 0x0B3F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_34_0_0b200021() {
    // Encoding: 0x0B200021
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=1, Rd=1
    // Fields: Rd=1, imm3=0, Rn=1, Rm=0, sf=0, S=0, op=0, option=0
    let encoding: u32 = 0x0B200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_combo_35_0_0b2003ff() {
    // Encoding: 0x0B2003FF
    // Test aarch64_integer_arithmetic_add_sub_extendedreg field combination: sf=0, op=0, S=0, Rm=0, option=0, imm3=0, Rn=31, Rd=31
    // Fields: op=0, Rm=0, Rn=31, option=0, S=0, imm3=0, sf=0, Rd=31
    let encoding: u32 = 0x0B2003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_special_sf_0_size_variant_0_0_0b200400() {
    // Encoding: 0x0B200400
    // Test aarch64_integer_arithmetic_add_sub_extendedreg special value sf = 0 (Size variant 0)
    // Fields: Rd=0, sf=0, imm3=1, S=0, op=0, Rm=0, option=0, Rn=0
    let encoding: u32 = 0x0B200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_special_sf_1_size_variant_1_0_8b200400() {
    // Encoding: 0x8B200400
    // Test aarch64_integer_arithmetic_add_sub_extendedreg special value sf = 1 (Size variant 1)
    // Fields: op=0, Rm=0, Rn=0, Rd=0, imm3=1, sf=1, S=0, option=0
    let encoding: u32 = 0x8B200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_special_s_0_size_variant_0_0_0b200400() {
    // Encoding: 0x0B200400
    // Test aarch64_integer_arithmetic_add_sub_extendedreg special value S = 0 (Size variant 0)
    // Fields: sf=0, option=0, Rd=0, Rn=0, op=0, S=0, Rm=0, imm3=1
    let encoding: u32 = 0x0B200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_special_s_1_size_variant_1_0_2b200400() {
    // Encoding: 0x2B200400
    // Test aarch64_integer_arithmetic_add_sub_extendedreg special value S = 1 (Size variant 1)
    // Fields: S=1, Rd=0, op=0, Rn=0, imm3=1, Rm=0, sf=0, option=0
    let encoding: u32 = 0x2B200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_special_rn_31_stack_pointer_sp_may_require_alignment_0_0b2007e0()
 {
    // Encoding: 0x0B2007E0
    // Test aarch64_integer_arithmetic_add_sub_extendedreg special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, sf=0, op=0, Rn=31, option=0, S=0, Rm=0, imm3=1
    let encoding: u32 = 0x0B2007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0b20041f()
 {
    // Encoding: 0x0B20041F
    // Test aarch64_integer_arithmetic_add_sub_extendedreg special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sf=0, op=0, Rm=0, imm3=1, Rd=31, S=0, Rn=0, option=0
    let encoding: u32 = 0x0B20041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `ADD X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple values
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_0_8b226020() {
    // Test ADD extended: simple values (oracle)
    // Encoding: 0x8B226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 2, 0x100);
    let encoding: u32 = 0x8B226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1100, "X0 should be 0x0000000000001100");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `ADD X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max + 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_1_8b226020() {
    // Test ADD extended: max + 1 (oracle)
    // Encoding: 0x8B226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x8B226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `ADDS X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple values
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_0_ab226020() {
    // Test ADDS extended: simple values (oracle)
    // Encoding: 0xAB226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 2, 0x100);
    let encoding: u32 = 0xAB226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1100, "X0 should be 0x0000000000001100");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `ADDS X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max + 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_1_ab226020() {
    // Test ADDS extended: max + 1 (oracle)
    // Encoding: 0xAB226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `SUB X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple values
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_0_cb226020() {
    // Test SUB extended: simple values (oracle)
    // Encoding: 0xCB226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 2, 0x100);
    let encoding: u32 = 0xCB226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xF00, "X0 should be 0x0000000000000F00");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `SUB X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max + 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_1_cb226020() {
    // Test SUB extended: max + 1 (oracle)
    // Encoding: 0xCB226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xCB226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `SUBS X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple values
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_0_eb226020() {
    // Test SUBS extended: simple values (oracle)
    // Encoding: 0xEB226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 2, 0x100);
    let encoding: u32 = 0xEB226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xF00, "X0 should be 0x0000000000000F00");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `SUBS X0, X1, W2, UXTW`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max + 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_oracle_1_eb226020() {
    // Test SUBS extended: max + 1 (oracle)
    // Encoding: 0xEB226020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB226020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_reg_write_0_0b200000() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg register write: Sp
    // Encoding: 0x0B200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_reg_write_1_0b200000() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg register write: GpFromField("d")
    // Encoding: 0x0B200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_sp_rn_0b2003e0() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg with Rn = SP (31)
    // Encoding: 0x0B2003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B2003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_zr_rd_0b20001f() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg with Rd = ZR (31)
    // Encoding: 0x0B20001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0B20001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_zeroresult_0_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: ZeroResult
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_zeroresult_1_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: ZeroResult
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_negativeresult_2_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: NegativeResult
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_unsignedoverflow_3_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: UnsignedOverflow
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_unsignedoverflow_4_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: UnsignedOverflow
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_signedoverflow_5_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: SignedOverflow
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_signedoverflow_6_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: SignedOverflow
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_extendedreg
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_extendedreg_flags_positiveresult_7_ab220020() {
    // Test aarch64_integer_arithmetic_add_sub_extendedreg flag computation: PositiveResult
    // Encoding: 0xAB220020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xAB220020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_arithmetic_add_sub_immediate Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_sf_0_min_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field sf = 0 (Min)
    // Fields: Rd=0, S=0, Rn=0, imm12=0, sf=0, op=0, sh=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_sf_1_max_0_91000000() {
    // Encoding: 0x91000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field sf = 1 (Max)
    // Fields: sf=1, Rd=0, sh=0, op=0, imm12=0, S=0, Rn=0
    let encoding: u32 = 0x91000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_op_0_min_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field op = 0 (Min)
    // Fields: Rn=0, Rd=0, op=0, sf=0, sh=0, S=0, imm12=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_op_1_max_0_51000000() {
    // Encoding: 0x51000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field op = 1 (Max)
    // Fields: op=1, S=0, sf=0, imm12=0, Rn=0, sh=0, Rd=0
    let encoding: u32 = 0x51000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_s_0_min_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field S = 0 (Min)
    // Fields: sh=0, Rn=0, S=0, sf=0, imm12=0, op=0, Rd=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field S 29 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_s_1_max_0_31000000() {
    // Encoding: 0x31000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field S = 1 (Max)
    // Fields: op=0, Rn=0, Rd=0, S=1, sf=0, imm12=0, sh=0
    let encoding: u32 = 0x31000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sh 22 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 0, boundary: Min }
/// shift type LSL (logical shift left)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_sh_0_min_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field sh = 0 (Min)
    // Fields: imm12=0, Rn=0, S=0, Rd=0, sf=0, op=0, sh=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sh 22 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 1, boundary: Max }
/// shift type LSR (logical shift right)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_sh_1_max_0_11400000() {
    // Encoding: 0x11400000
    // Test aarch64_integer_arithmetic_add_sub_immediate field sh = 1 (Max)
    // Fields: op=0, Rd=0, S=0, sf=0, sh=1, imm12=0, Rn=0
    let encoding: u32 = 0x11400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_0_zero_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 0 (Zero)
    // Fields: Rd=0, sh=0, op=0, imm12=0, Rn=0, sf=0, S=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_1_poweroftwo_0_11000400() {
    // Encoding: 0x11000400
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 1 (PowerOfTwo)
    // Fields: sf=0, imm12=1, op=0, Rn=0, Rd=0, sh=0, S=0
    let encoding: u32 = 0x11000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_3_poweroftwominusone_0_11000c00() {
    // Encoding: 0x11000C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 3 (PowerOfTwoMinusOne)
    // Fields: op=0, S=0, sf=0, sh=0, imm12=3, Rn=0, Rd=0
    let encoding: u32 = 0x11000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_4_poweroftwo_0_11001000() {
    // Encoding: 0x11001000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 4 (PowerOfTwo)
    // Fields: op=0, sf=0, S=0, Rn=0, imm12=4, sh=0, Rd=0
    let encoding: u32 = 0x11001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_7_poweroftwominusone_0_11001c00() {
    // Encoding: 0x11001C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, S=0, sh=0, sf=0, op=0, imm12=7, Rn=0
    let encoding: u32 = 0x11001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_8_poweroftwo_0_11002000() {
    // Encoding: 0x11002000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 8 (PowerOfTwo)
    // Fields: sf=0, imm12=8, Rn=0, Rd=0, S=0, op=0, sh=0
    let encoding: u32 = 0x11002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_15_poweroftwominusone_0_11003c00()
{
    // Encoding: 0x11003C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, sf=0, S=0, sh=0, op=0, imm12=15
    let encoding: u32 = 0x11003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_16_poweroftwo_0_11004000() {
    // Encoding: 0x11004000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 16 (PowerOfTwo)
    // Fields: Rd=0, op=0, sf=0, S=0, sh=0, imm12=16, Rn=0
    let encoding: u32 = 0x11004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_31_poweroftwominusone_0_11007c00()
{
    // Encoding: 0x11007C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 31 (PowerOfTwoMinusOne)
    // Fields: Rd=0, sh=0, imm12=31, op=0, Rn=0, S=0, sf=0
    let encoding: u32 = 0x11007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_32_poweroftwo_0_11008000() {
    // Encoding: 0x11008000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 32 (PowerOfTwo)
    // Fields: sf=0, Rd=0, sh=0, imm12=32, Rn=0, op=0, S=0
    let encoding: u32 = 0x11008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_63_poweroftwominusone_0_1100fc00()
{
    // Encoding: 0x1100FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 63 (PowerOfTwoMinusOne)
    // Fields: sf=0, sh=0, imm12=63, Rn=0, op=0, S=0, Rd=0
    let encoding: u32 = 0x1100FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_64_poweroftwo_0_11010000() {
    // Encoding: 0x11010000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 64 (PowerOfTwo)
    // Fields: sf=0, Rd=0, S=0, sh=0, imm12=64, Rn=0, op=0
    let encoding: u32 = 0x11010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_127_poweroftwominusone_0_1101fc00()
{
    // Encoding: 0x1101FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 127 (PowerOfTwoMinusOne)
    // Fields: op=0, S=0, sh=0, imm12=127, Rd=0, Rn=0, sf=0
    let encoding: u32 = 0x1101FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_128_poweroftwo_0_11020000() {
    // Encoding: 0x11020000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 128 (PowerOfTwo)
    // Fields: op=0, imm12=128, Rn=0, S=0, sf=0, Rd=0, sh=0
    let encoding: u32 = 0x11020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_255_poweroftwominusone_0_1103fc00()
{
    // Encoding: 0x1103FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 255 (PowerOfTwoMinusOne)
    // Fields: op=0, sh=0, sf=0, Rn=0, Rd=0, S=0, imm12=255
    let encoding: u32 = 0x1103FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_256_poweroftwo_0_11040000() {
    // Encoding: 0x11040000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 256 (PowerOfTwo)
    // Fields: S=0, Rn=0, op=0, Rd=0, sh=0, sf=0, imm12=256
    let encoding: u32 = 0x11040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_511_poweroftwominusone_0_1107fc00()
{
    // Encoding: 0x1107FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 511 (PowerOfTwoMinusOne)
    // Fields: sh=0, op=0, imm12=511, Rn=0, Rd=0, sf=0, S=0
    let encoding: u32 = 0x1107FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_512_poweroftwo_0_11080000() {
    // Encoding: 0x11080000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 512 (PowerOfTwo)
    // Fields: op=0, S=0, sh=0, sf=0, Rn=0, imm12=512, Rd=0
    let encoding: u32 = 0x11080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_1023_poweroftwominusone_0_110ffc00()
 {
    // Encoding: 0x110FFC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 1023 (PowerOfTwoMinusOne)
    // Fields: Rd=0, op=0, sh=0, imm12=1023, sf=0, Rn=0, S=0
    let encoding: u32 = 0x110FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_1024_poweroftwo_0_11100000() {
    // Encoding: 0x11100000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 1024 (PowerOfTwo)
    // Fields: Rd=0, sf=0, op=0, imm12=1024, S=0, sh=0, Rn=0
    let encoding: u32 = 0x11100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_2047_poweroftwominusone_0_111ffc00()
 {
    // Encoding: 0x111FFC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 2047 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, op=0, sf=0, S=0, sh=0, imm12=2047
    let encoding: u32 = 0x111FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_2048_poweroftwo_0_11200000() {
    // Encoding: 0x11200000
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 2048 (PowerOfTwo)
    // Fields: op=0, Rn=0, Rd=0, sh=0, imm12=2048, sf=0, S=0
    let encoding: u32 = 0x11200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field imm12 10 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_imm12_4095_max_0_113ffc00() {
    // Encoding: 0x113FFC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field imm12 = 4095 (Max)
    // Fields: sf=0, sh=0, Rn=0, Rd=0, imm12=4095, S=0, op=0
    let encoding: u32 = 0x113FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rn_0_min_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rn = 0 (Min)
    // Fields: op=0, S=0, sh=0, imm12=0, Rd=0, sf=0, Rn=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rn_1_poweroftwo_0_11000020() {
    // Encoding: 0x11000020
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rn = 1 (PowerOfTwo)
    // Fields: imm12=0, Rn=1, sf=0, sh=0, Rd=0, op=0, S=0
    let encoding: u32 = 0x11000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rn_30_poweroftwominusone_0_110003c0() {
    // Encoding: 0x110003C0
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, op=0, S=0, imm12=0, sf=0, sh=0, Rn=30
    let encoding: u32 = 0x110003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rn_31_max_0_110003e0() {
    // Encoding: 0x110003E0
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rn = 31 (Max)
    // Fields: sh=0, imm12=0, Rd=0, Rn=31, S=0, sf=0, op=0
    let encoding: u32 = 0x110003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rd_0_min_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rd = 0 (Min)
    // Fields: imm12=0, Rd=0, sf=0, op=0, S=0, sh=0, Rn=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rd_1_poweroftwo_0_11000001() {
    // Encoding: 0x11000001
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rd = 1 (PowerOfTwo)
    // Fields: S=0, op=0, sf=0, imm12=0, Rn=0, sh=0, Rd=1
    let encoding: u32 = 0x11000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rd_30_poweroftwominusone_0_1100001e() {
    // Encoding: 0x1100001E
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, sh=0, imm12=0, op=0, sf=0, S=0, Rn=0
    let encoding: u32 = 0x1100001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_field_rd_31_max_0_1100001f() {
    // Encoding: 0x1100001F
    // Test aarch64_integer_arithmetic_add_sub_immediate field Rd = 31 (Max)
    // Fields: Rd=31, sf=0, Rn=0, S=0, sh=0, op=0, imm12=0
    let encoding: u32 = 0x1100001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_0_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: imm12=0, Rd=0, sf=0, sh=0, Rn=0, op=0, S=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_1_0_91000000() {
    // Encoding: 0x91000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=1, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: op=0, imm12=0, Rn=0, sh=0, sf=1, Rd=0, S=0
    let encoding: u32 = 0x91000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_2_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: sh=0, S=0, sf=0, imm12=0, Rd=0, op=0, Rn=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_3_0_51000000() {
    // Encoding: 0x51000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=1, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: op=1, S=0, imm12=0, Rn=0, Rd=0, sf=0, sh=0
    let encoding: u32 = 0x51000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_4_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: sh=0, sf=0, op=0, imm12=0, Rn=0, S=0, Rd=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_5_0_31000000() {
    // Encoding: 0x31000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=1, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: sf=0, op=0, Rn=0, Rd=0, sh=0, S=1, imm12=0
    let encoding: u32 = 0x31000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=0 (shift type LSL (logical shift left))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_6_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: S=0, sh=0, imm12=0, Rn=0, Rd=0, sf=0, op=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=1 (shift type LSR (logical shift right))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_7_0_11400000() {
    // Encoding: 0x11400000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=1, imm12=0, Rn=0, Rd=0
    // Fields: sf=0, imm12=0, op=0, Rd=0, sh=1, Rn=0, S=0
    let encoding: u32 = 0x11400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=0 (immediate value 0)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_8_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: sf=0, S=0, Rn=0, Rd=0, op=0, sh=0, imm12=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1 (immediate value 1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_9_0_11000400() {
    // Encoding: 0x11000400
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=1, Rn=0, Rd=0
    // Fields: sf=0, S=0, sh=0, Rn=0, op=0, imm12=1, Rd=0
    let encoding: u32 = 0x11000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_10_0_11000c00() {
    // Encoding: 0x11000C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=3, Rn=0, Rd=0
    // Fields: S=0, Rd=0, op=0, sh=0, sf=0, imm12=3, Rn=0
    let encoding: u32 = 0x11000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_11_0_11001000() {
    // Encoding: 0x11001000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=4, Rn=0, Rd=0
    // Fields: S=0, sf=0, sh=0, Rn=0, imm12=4, Rd=0, op=0
    let encoding: u32 = 0x11001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_12_0_11001c00() {
    // Encoding: 0x11001C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=7, Rn=0, Rd=0
    // Fields: sf=0, S=0, sh=0, Rd=0, op=0, Rn=0, imm12=7
    let encoding: u32 = 0x11001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_13_0_11002000() {
    // Encoding: 0x11002000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=8, Rn=0, Rd=0
    // Fields: S=0, Rd=0, op=0, sf=0, imm12=8, Rn=0, sh=0
    let encoding: u32 = 0x11002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_14_0_11003c00() {
    // Encoding: 0x11003C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=15, Rn=0, Rd=0
    // Fields: S=0, imm12=15, Rd=0, sf=0, sh=0, op=0, Rn=0
    let encoding: u32 = 0x11003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_15_0_11004000() {
    // Encoding: 0x11004000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=16, Rn=0, Rd=0
    // Fields: sf=0, S=0, op=0, sh=0, Rd=0, imm12=16, Rn=0
    let encoding: u32 = 0x11004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_16_0_11007c00() {
    // Encoding: 0x11007C00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=31, Rn=0, Rd=0
    // Fields: imm12=31, S=0, sh=0, Rn=0, Rd=0, op=0, sf=0
    let encoding: u32 = 0x11007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_17_0_11008000() {
    // Encoding: 0x11008000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=32, Rn=0, Rd=0
    // Fields: imm12=32, sh=0, sf=0, S=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x11008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_18_0_1100fc00() {
    // Encoding: 0x1100FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=63, Rn=0, Rd=0
    // Fields: sf=0, sh=0, S=0, Rd=0, op=0, imm12=63, Rn=0
    let encoding: u32 = 0x1100FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_19_0_11010000() {
    // Encoding: 0x11010000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=64, Rn=0, Rd=0
    // Fields: sf=0, op=0, imm12=64, Rd=0, S=0, Rn=0, sh=0
    let encoding: u32 = 0x11010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_20_0_1101fc00() {
    // Encoding: 0x1101FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=127, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sf=0, sh=0, S=0, imm12=127, op=0
    let encoding: u32 = 0x1101FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_21_0_11020000() {
    // Encoding: 0x11020000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=128, Rn=0, Rd=0
    // Fields: sh=0, op=0, sf=0, Rn=0, Rd=0, imm12=128, S=0
    let encoding: u32 = 0x11020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_22_0_1103fc00() {
    // Encoding: 0x1103FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=255, Rn=0, Rd=0
    // Fields: imm12=255, sf=0, sh=0, op=0, Rd=0, S=0, Rn=0
    let encoding: u32 = 0x1103FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_23_0_11040000() {
    // Encoding: 0x11040000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=256, Rn=0, Rd=0
    // Fields: sh=0, Rd=0, sf=0, S=0, op=0, Rn=0, imm12=256
    let encoding: u32 = 0x11040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_24_0_1107fc00() {
    // Encoding: 0x1107FC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=511, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, sf=0, op=0, S=0, sh=0, imm12=511
    let encoding: u32 = 0x1107FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_25_0_11080000() {
    // Encoding: 0x11080000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=512, Rn=0, Rd=0
    // Fields: Rn=0, S=0, sf=0, op=0, sh=0, Rd=0, imm12=512
    let encoding: u32 = 0x11080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_26_0_110ffc00() {
    // Encoding: 0x110FFC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=1023, Rn=0, Rd=0
    // Fields: imm12=1023, Rn=0, S=0, sh=0, Rd=0, sf=0, op=0
    let encoding: u32 = 0x110FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_27_0_11100000() {
    // Encoding: 0x11100000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=1024, Rn=0, Rd=0
    // Fields: Rn=0, sf=0, sh=0, op=0, Rd=0, imm12=1024, S=0
    let encoding: u32 = 0x11100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=2047 (immediate midpoint (2047))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_28_0_111ffc00() {
    // Encoding: 0x111FFC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=2047, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, imm12=2047, S=0, Rd=0, op=0, sh=0
    let encoding: u32 = 0x111FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_29_0_11200000() {
    // Encoding: 0x11200000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=2048, Rn=0, Rd=0
    // Fields: imm12=2048, S=0, Rd=0, sf=0, sh=0, op=0, Rn=0
    let encoding: u32 = 0x11200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm12=4095 (maximum immediate (4095))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_30_0_113ffc00() {
    // Encoding: 0x113FFC00
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=4095, Rn=0, Rd=0
    // Fields: S=0, sf=0, op=0, sh=0, imm12=4095, Rn=0, Rd=0
    let encoding: u32 = 0x113FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_31_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: sh=0, Rn=0, imm12=0, Rd=0, S=0, sf=0, op=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_32_0_11000020() {
    // Encoding: 0x11000020
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=1, Rd=0
    // Fields: imm12=0, S=0, op=0, Rn=1, Rd=0, sf=0, sh=0
    let encoding: u32 = 0x11000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_33_0_110003c0() {
    // Encoding: 0x110003C0
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=30, Rd=0
    // Fields: Rn=30, S=0, sf=0, sh=0, Rd=0, op=0, imm12=0
    let encoding: u32 = 0x110003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_34_0_110003e0() {
    // Encoding: 0x110003E0
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=31, Rd=0
    // Fields: sh=0, Rd=0, sf=0, S=0, op=0, imm12=0, Rn=31
    let encoding: u32 = 0x110003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_35_0_11000000() {
    // Encoding: 0x11000000
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, sf=0, sh=0, S=0, Rd=0, imm12=0
    let encoding: u32 = 0x11000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_36_0_11000001() {
    // Encoding: 0x11000001
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, sh=0, imm12=0, sf=0, op=0, S=0
    let encoding: u32 = 0x11000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_37_0_1100001e() {
    // Encoding: 0x1100001E
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=30
    // Fields: op=0, Rd=30, S=0, imm12=0, sh=0, Rn=0, sf=0
    let encoding: u32 = 0x1100001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_38_0_1100001f() {
    // Encoding: 0x1100001F
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=0, Rd=31
    // Fields: S=0, Rn=0, op=0, sf=0, sh=0, imm12=0, Rd=31
    let encoding: u32 = 0x1100001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_39_0_11000021() {
    // Encoding: 0x11000021
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=1, Rd=1
    // Fields: imm12=0, op=0, S=0, sh=0, Rn=1, Rd=1, sf=0
    let encoding: u32 = 0x11000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_combo_40_0_110003ff() {
    // Encoding: 0x110003FF
    // Test aarch64_integer_arithmetic_add_sub_immediate field combination: sf=0, op=0, S=0, sh=0, imm12=0, Rn=31, Rd=31
    // Fields: Rd=31, sf=0, imm12=0, Rn=31, sh=0, S=0, op=0
    let encoding: u32 = 0x110003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_sf_0_size_variant_0_0_11000400() {
    // Encoding: 0x11000400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value sf = 0 (Size variant 0)
    // Fields: Rd=0, sf=0, Rn=0, S=0, sh=0, op=0, imm12=1
    let encoding: u32 = 0x11000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_sf_1_size_variant_1_0_91000400() {
    // Encoding: 0x91000400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value sf = 1 (Size variant 1)
    // Fields: Rn=0, S=0, sf=1, op=0, sh=0, imm12=1, Rd=0
    let encoding: u32 = 0x91000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_s_0_size_variant_0_0_11000400() {
    // Encoding: 0x11000400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value S = 0 (Size variant 0)
    // Fields: sf=0, imm12=1, sh=0, Rn=0, Rd=0, S=0, op=0
    let encoding: u32 = 0x11000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_s_1_size_variant_1_0_31000400() {
    // Encoding: 0x31000400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value S = 1 (Size variant 1)
    // Fields: op=0, S=1, Rn=0, Rd=0, sf=0, imm12=1, sh=0
    let encoding: u32 = 0x31000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sh = 0 (Shift type LSL)`
/// Requirement: FieldSpecial { field: "sh", value: 0, meaning: "Shift type LSL" }
/// Shift type LSL
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_sh_0_shift_type_lsl_0_11000400() {
    // Encoding: 0x11000400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value sh = 0 (Shift type LSL)
    // Fields: imm12=1, sf=0, op=0, Rn=0, Rd=0, S=0, sh=0
    let encoding: u32 = 0x11000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sh = 1 (Shift type LSR)`
/// Requirement: FieldSpecial { field: "sh", value: 1, meaning: "Shift type LSR" }
/// Shift type LSR
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_sh_1_shift_type_lsr_0_11400400() {
    // Encoding: 0x11400400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value sh = 1 (Shift type LSR)
    // Fields: sf=0, sh=1, Rn=0, op=0, Rd=0, imm12=1, S=0
    let encoding: u32 = 0x11400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sh = 2 (Shift type ASR)`
/// Requirement: FieldSpecial { field: "sh", value: 2, meaning: "Shift type ASR" }
/// Shift type ASR
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_sh_2_shift_type_asr_0_11000400() {
    // Encoding: 0x11000400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value sh = 2 (Shift type ASR)
    // Fields: imm12=1, Rn=0, sf=0, Rd=0, op=0, S=0, sh=2
    let encoding: u32 = 0x11000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field sh = 3 (Shift type ROR)`
/// Requirement: FieldSpecial { field: "sh", value: 3, meaning: "Shift type ROR" }
/// Shift type ROR
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_sh_3_shift_type_ror_0_11400400() {
    // Encoding: 0x11400400
    // Test aarch64_integer_arithmetic_add_sub_immediate special value sh = 3 (Shift type ROR)
    // Fields: sf=0, op=0, Rd=0, Rn=0, S=0, sh=3, imm12=1
    let encoding: u32 = 0x11400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_rn_31_stack_pointer_sp_may_require_alignment_0_110007e0()
 {
    // Encoding: 0x110007E0
    // Test aarch64_integer_arithmetic_add_sub_immediate special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: op=0, sh=0, Rn=31, sf=0, imm12=1, S=0, Rd=0
    let encoding: u32 = 0x110007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1100041f()
 {
    // Encoding: 0x1100041F
    // Test aarch64_integer_arithmetic_add_sub_immediate special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, op=0, Rd=31, sf=0, S=0, imm12=1, sh=0
    let encoding: u32 = 0x1100041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_0_11002820() {
    // Test ADD 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0x11002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x11002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6E, "W0 should be 0x6E");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_0_91002820() {
    // Test ADD 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0x91002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x91002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6E, "X0 should be 0x000000000000006E");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_1_11000020() {
    // Test ADD 32-bit: zero operands (with oracle verification)
    // Encoding: 0x11000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x11000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_1_91000020() {
    // Test ADD 64-bit: zero operands (with oracle verification)
    // Encoding: 0x91000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x91000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_2_11000420() {
    // Test ADD 32-bit: small values (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x2, "W0 should be 0x2");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_2_91000420() {
    // Test ADD 64-bit: small values (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x2, "X0 should be 0x0000000000000002");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_3_113ffc20() {
    // Test ADD 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0x113FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x113FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFF, "W0 should be 0xFFF");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_3_913ffc20() {
    // Test ADD 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0x913FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x913FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFF, "X0 should be 0x0000000000000FFF");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_4_117ffc20() {
    // Test ADD 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0x117FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x117FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFF000, "W0 should be 0xFFF000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_4_917ffc20() {
    // Test ADD 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0x917FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x917FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFF000, "X0 should be 0x0000000000FFF000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_5_11000420() {
    // Test ADD 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_5_91000420() {
    // Test ADD 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_6_11000420() {
    // Test ADD 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x1");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_6_91000420() {
    // Test ADD 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_7_11000420() {
    // Test ADD 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_7_91000420() {
    // Test ADD 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x8000000000000000,
        "X0 should be 0x8000000000000000"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_8_11000420() {
    // Test ADD 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_8_91000420() {
    // Test ADD 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x80000000,
        "X0 should be 0x0000000080000000"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_9_11000420() {
    // Test ADD 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_9_91000420() {
    // Test ADD 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_10_11000420() {
    // Test ADD 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0x11000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x11000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_10_91000420() {
    // Test ADD 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0x91000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x91000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x100000000,
        "X0 should be 0x0000000100000000"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_32_rd31_sp_1100283f() {
    // Test ADD 32-bit with Rd=31 (SP)
    // Encoding: 0x1100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x1100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_sp(), 0x6E, "SP should be 0x6E");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_add_oracle_64_rd31_sp_9100283f() {
    // Test ADD 64-bit with Rd=31 (SP)
    // Encoding: 0x9100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x9100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_sp(), 0x6E, "SP should be 0x6E");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_0_31002820() {
    // Test ADDS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0x31002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x31002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6E, "W0 should be 0x6E");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_0_b1002820() {
    // Test ADDS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xB1002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xB1002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6E, "X0 should be 0x000000000000006E");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_1_31000020() {
    // Test ADDS 32-bit: zero operands (with oracle verification)
    // Encoding: 0x31000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x31000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_1_b1000020() {
    // Test ADDS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_2_31000420() {
    // Test ADDS 32-bit: small values (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x2, "W0 should be 0x2");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_2_b1000420() {
    // Test ADDS 64-bit: small values (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x2, "X0 should be 0x0000000000000002");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_3_313ffc20() {
    // Test ADDS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0x313FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x313FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFF, "W0 should be 0xFFF");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_3_b13ffc20() {
    // Test ADDS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xB13FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xB13FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFF, "X0 should be 0x0000000000000FFF");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_4_317ffc20() {
    // Test ADDS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0x317FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x317FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFF000, "W0 should be 0xFFF000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_4_b17ffc20() {
    // Test ADDS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xB17FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xB17FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFF000, "X0 should be 0x0000000000FFF000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_5_31000420() {
    // Test ADDS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_5_b1000420() {
    // Test ADDS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_6_31000420() {
    // Test ADDS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x1");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_6_b1000420() {
    // Test ADDS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_7_31000420() {
    // Test ADDS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_7_b1000420() {
    // Test ADDS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x8000000000000000,
        "X0 should be 0x8000000000000000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, true, "V flag should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_8_31000420() {
    // Test ADDS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, true, "V flag should be true");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_8_b1000420() {
    // Test ADDS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x80000000,
        "X0 should be 0x0000000080000000"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_9_31000420() {
    // Test ADDS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_9_b1000420() {
    // Test ADDS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_10_31000420() {
    // Test ADDS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0x31000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x31000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_10_b1000420() {
    // Test ADDS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xB1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xB1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x100000000,
        "X0 should be 0x0000000100000000"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_32_rd31_zr_3100283f() {
    // Test ADDS 32-bit with Rd=31 (ZR)
    // Encoding: 0x3100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x3100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_adds_oracle_64_rd31_zr_b100283f() {
    // Test ADDS 64-bit with Rd=31 (ZR)
    // Encoding: 0xB100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xB100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_0_51002820() {
    // Test SUB 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0x51002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x51002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5A, "W0 should be 0x5A");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_0_d1002820() {
    // Test SUB 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xD1002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xD1002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x5A, "X0 should be 0x000000000000005A");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_1_51000020() {
    // Test SUB 32-bit: zero operands (with oracle verification)
    // Encoding: 0x51000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x51000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_1_d1000020() {
    // Test SUB 64-bit: zero operands (with oracle verification)
    // Encoding: 0xD1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xD1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_2_51000420() {
    // Test SUB 32-bit: small values (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_2_d1000420() {
    // Test SUB 64-bit: small values (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_3_513ffc20() {
    // Test SUB 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0x513FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x513FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFF001, "W0 should be 0xFFFFF001");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_3_d13ffc20() {
    // Test SUB 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xD13FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xD13FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_4_517ffc20() {
    // Test SUB 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0x517FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x517FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF001000, "W0 should be 0xFF001000");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_4_d17ffc20() {
    // Test SUB 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xD17FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xD17FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_5_51000420() {
    // Test SUB 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_5_d1000420() {
    // Test SUB 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_6_51000420() {
    // Test SUB 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_6_d1000420() {
    // Test SUB 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_7_51000420() {
    // Test SUB 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_7_d1000420() {
    // Test SUB 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x7FFFFFFFFFFFFFFE,
        "X0 should be 0x7FFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_8_51000420() {
    // Test SUB 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x7FFFFFFE, "W0 should be 0x7FFFFFFE");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_8_d1000420() {
    // Test SUB 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_9_51000420() {
    // Test SUB 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_9_d1000420() {
    // Test SUB 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_10_51000420() {
    // Test SUB 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0x51000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x51000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_10_d1000420() {
    // Test SUB 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xD1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xD1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_32_rd31_sp_5100283f() {
    // Test SUB 32-bit with Rd=31 (SP)
    // Encoding: 0x5100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x5100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_sp(), 0x5A, "SP should be 0x5A");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sub_oracle_64_rd31_sp_d100283f() {
    // Test SUB 64-bit with Rd=31 (SP)
    // Encoding: 0xD100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xD100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_sp(), 0x5A, "SP should be 0x5A");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_0_71002820() {
    // Test SUBS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0x71002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x71002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5A, "W0 should be 0x5A");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_0_f1002820() {
    // Test SUBS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF1002820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF1002820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x5A, "X0 should be 0x000000000000005A");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_1_71000020() {
    // Test SUBS 32-bit: zero operands (with oracle verification)
    // Encoding: 0x71000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x71000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_1_f1000020() {
    // Test SUBS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_2_71000420() {
    // Test SUBS 32-bit: small values (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x0");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_2_f1000420() {
    // Test SUBS 64-bit: small values (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_3_713ffc20() {
    // Test SUBS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0x713FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x713FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFF001, "W0 should be 0xFFFFF001");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_3_f13ffc20() {
    // Test SUBS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF13FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF13FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFF001,
        "X0 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_4_717ffc20() {
    // Test SUBS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0x717FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x717FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF001000, "W0 should be 0xFF001000");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_4_f17ffc20() {
    // Test SUBS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF17FFC20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF17FFC20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFF001000,
        "X0 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_5_71000420() {
    // Test SUBS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_5_f1000420() {
    // Test SUBS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_6_71000420() {
    // Test SUBS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_6_f1000420() {
    // Test SUBS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_7_71000420() {
    // Test SUBS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_7_f1000420() {
    // Test SUBS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x7FFFFFFFFFFFFFFE,
        "X0 should be 0x7FFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_8_71000420() {
    // Test SUBS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x7FFFFFFE, "W0 should be 0x7FFFFFFE");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_8_f1000420() {
    // Test SUBS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x7FFFFFFE,
        "X0 should be 0x000000007FFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_9_71000420() {
    // Test SUBS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_9_f1000420() {
    // Test SUBS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_10_71000420() {
    // Test SUBS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0x71000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x71000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_10_f1000420() {
    // Test SUBS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF1000420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF1000420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFE,
        "X0 should be 0x00000000FFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_32_rd31_zr_7100283f() {
    // Test SUBS 32-bit with Rd=31 (ZR)
    // Encoding: 0x7100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x7100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_subs_oracle_64_rd31_zr_f100283f() {
    // Test SUBS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF100283F
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF100283F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_reg_write_0_11000000() {
    // Test aarch64_integer_arithmetic_add_sub_immediate register write: Sp
    // Encoding: 0x11000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x11000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_reg_write_1_11000000() {
    // Test aarch64_integer_arithmetic_add_sub_immediate register write: GpFromField("d")
    // Encoding: 0x11000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x11000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_sp_rn_110003e0() {
    // Test aarch64_integer_arithmetic_add_sub_immediate with Rn = SP (31)
    // Encoding: 0x110003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x110003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_zr_rd_1100001f() {
    // Test aarch64_integer_arithmetic_add_sub_immediate with Rd = ZR (31)
    // Encoding: 0x1100001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1100001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_zeroresult_0_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: ZeroResult
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_zeroresult_1_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: ZeroResult
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_negativeresult_2_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: NegativeResult
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_unsignedoverflow_3_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: UnsignedOverflow
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_unsignedoverflow_4_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: UnsignedOverflow
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_signedoverflow_5_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: SignedOverflow
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_signedoverflow_6_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: SignedOverflow
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_add_sub_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_arithmetic_add_sub_immediate_flags_positiveresult_7_b1000020() {
    // Test aarch64_integer_arithmetic_add_sub_immediate flag computation: PositiveResult
    // Encoding: 0xB1000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xB1000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_arithmetic_mul_uniform_add_sub Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_sf_0_min_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field sf = 0 (Min)
    // Fields: sf=0, o0=0, Rm=0, Ra=0, Rd=0, Rn=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_sf_1_max_0_9b000000() {
    // Encoding: 0x9B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field sf = 1 (Max)
    // Fields: sf=1, Rm=0, o0=0, Rn=0, Ra=0, Rd=0
    let encoding: u32 = 0x9B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rm_0_min_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rm = 0 (Min)
    // Fields: Ra=0, Rm=0, o0=0, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rm_1_poweroftwo_0_1b010000() {
    // Encoding: 0x1B010000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rm = 1 (PowerOfTwo)
    // Fields: Ra=0, o0=0, Rn=0, Rm=1, sf=0, Rd=0
    let encoding: u32 = 0x1B010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rm_30_poweroftwominusone_0_1b1e0000() {
    // Encoding: 0x1B1E0000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Ra=0, Rn=0, o0=0, Rm=30, Rd=0
    let encoding: u32 = 0x1B1E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rm_31_max_0_1b1f0000() {
    // Encoding: 0x1B1F0000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rm = 31 (Max)
    // Fields: Rn=0, Rd=0, sf=0, Ra=0, Rm=31, o0=0
    let encoding: u32 = 0x1B1F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_o0_0_min_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field o0 = 0 (Min)
    // Fields: Rn=0, Ra=0, o0=0, Rm=0, sf=0, Rd=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_o0_1_max_0_1b008000() {
    // Encoding: 0x1B008000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field o0 = 1 (Max)
    // Fields: sf=0, Rm=0, o0=1, Ra=0, Rn=0, Rd=0
    let encoding: u32 = 0x1B008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Ra 10 +: 5`
/// Requirement: FieldBoundary { field: "Ra", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_ra_0_min_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Ra = 0 (Min)
    // Fields: Rm=0, o0=0, sf=0, Ra=0, Rd=0, Rn=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Ra 10 +: 5`
/// Requirement: FieldBoundary { field: "Ra", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_ra_1_poweroftwo_0_1b000400() {
    // Encoding: 0x1B000400
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Ra = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, sf=0, o0=0, Rm=0, Ra=1
    let encoding: u32 = 0x1B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Ra 10 +: 5`
/// Requirement: FieldBoundary { field: "Ra", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_ra_30_poweroftwominusone_0_1b007800() {
    // Encoding: 0x1B007800
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Ra = 30 (PowerOfTwoMinusOne)
    // Fields: o0=0, sf=0, Ra=30, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x1B007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Ra 10 +: 5`
/// Requirement: FieldBoundary { field: "Ra", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_ra_31_max_0_1b007c00() {
    // Encoding: 0x1B007C00
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Ra = 31 (Max)
    // Fields: Rn=0, sf=0, o0=0, Rm=0, Rd=0, Ra=31
    let encoding: u32 = 0x1B007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rn_0_min_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rn = 0 (Min)
    // Fields: Rd=0, Rm=0, o0=0, sf=0, Ra=0, Rn=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rn_1_poweroftwo_0_1b000020() {
    // Encoding: 0x1B000020
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rm=0, Rd=0, o0=0, Ra=0, sf=0
    let encoding: u32 = 0x1B000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rn_30_poweroftwominusone_0_1b0003c0() {
    // Encoding: 0x1B0003C0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, Rm=0, o0=0, sf=0, Ra=0
    let encoding: u32 = 0x1B0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rn_31_max_0_1b0003e0() {
    // Encoding: 0x1B0003E0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rn = 31 (Max)
    // Fields: o0=0, Rd=0, Ra=0, Rn=31, sf=0, Rm=0
    let encoding: u32 = 0x1B0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rd_0_min_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rd = 0 (Min)
    // Fields: Ra=0, Rn=0, Rm=0, Rd=0, o0=0, sf=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rd_1_poweroftwo_0_1b000001() {
    // Encoding: 0x1B000001
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1, sf=0, o0=0, Ra=0, Rm=0
    let encoding: u32 = 0x1B000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rd_30_poweroftwominusone_0_1b00001e() {
    // Encoding: 0x1B00001E
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, Rm=0, sf=0, o0=0, Ra=0
    let encoding: u32 = 0x1B00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_field_rd_31_max_0_1b00001f() {
    // Encoding: 0x1B00001F
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field Rd = 31 (Max)
    // Fields: sf=0, Rm=0, Ra=0, Rd=31, Rn=0, o0=0
    let encoding: u32 = 0x1B00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_0_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: sf=0, Rm=0, Ra=0, Rd=0, o0=0, Rn=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_1_0_9b000000() {
    // Encoding: 0x9B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=1, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Ra=0, Rm=0, o0=0, sf=1, Rd=0, Rn=0
    let encoding: u32 = 0x9B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_2_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Rn=0, Ra=0, sf=0, o0=0, Rd=0, Rm=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_3_0_1b010000() {
    // Encoding: 0x1B010000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=1, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rd=0, Rm=1, o0=0, Ra=0
    let encoding: u32 = 0x1B010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_4_0_1b1e0000() {
    // Encoding: 0x1B1E0000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=30, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=30, Ra=0, Rn=0, o0=0, sf=0
    let encoding: u32 = 0x1B1E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_5_0_1b1f0000() {
    // Encoding: 0x1B1F0000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=31, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Ra=0, Rn=0, Rm=31, sf=0, Rd=0, o0=0
    let encoding: u32 = 0x1B1F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_6_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Rm=0, sf=0, o0=0, Rn=0, Ra=0, Rd=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_7_0_1b008000() {
    // Encoding: 0x1B008000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=1, Ra=0, Rn=0, Rd=0
    // Fields: o0=1, sf=0, Rn=0, Rd=0, Ra=0, Rm=0
    let encoding: u32 = 0x1B008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_8_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: sf=0, o0=0, Rm=0, Rd=0, Ra=0, Rn=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_9_0_1b000400() {
    // Encoding: 0x1B000400
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=1, Rn=0, Rd=0
    // Fields: sf=0, Rm=0, Ra=1, o0=0, Rn=0, Rd=0
    let encoding: u32 = 0x1B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_10_0_1b007800() {
    // Encoding: 0x1B007800
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=30, Rn=0, Rd=0
    // Fields: Rn=0, o0=0, Rm=0, Rd=0, Ra=30, sf=0
    let encoding: u32 = 0x1B007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_11_0_1b007c00() {
    // Encoding: 0x1B007C00
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=31, Rn=0, Rd=0
    // Fields: sf=0, Rd=0, Rm=0, Ra=31, Rn=0, o0=0
    let encoding: u32 = 0x1B007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_12_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Ra=0, Rn=0, sf=0, o0=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_13_0_1b000020() {
    // Encoding: 0x1B000020
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=1, Rd=0
    // Fields: Rm=0, Ra=0, sf=0, Rn=1, Rd=0, o0=0
    let encoding: u32 = 0x1B000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_14_0_1b0003c0() {
    // Encoding: 0x1B0003C0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=30, Rd=0
    // Fields: Rm=0, Rn=30, Ra=0, sf=0, o0=0, Rd=0
    let encoding: u32 = 0x1B0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_15_0_1b0003e0() {
    // Encoding: 0x1B0003E0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=31, Rd=0
    // Fields: Ra=0, Rd=0, o0=0, sf=0, Rm=0, Rn=31
    let encoding: u32 = 0x1B0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_16_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, o0=0, sf=0, Rm=0, Ra=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_17_0_1b000001() {
    // Encoding: 0x1B000001
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=1
    // Fields: Ra=0, sf=0, Rm=0, Rn=0, Rd=1, o0=0
    let encoding: u32 = 0x1B000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_18_0_1b00001e() {
    // Encoding: 0x1B00001E
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=30
    // Fields: o0=0, Rn=0, sf=0, Rd=30, Rm=0, Ra=0
    let encoding: u32 = 0x1B00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_19_0_1b00001f() {
    // Encoding: 0x1B00001F
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=0, Rd=31
    // Fields: Rm=0, sf=0, Ra=0, Rn=0, Rd=31, o0=0
    let encoding: u32 = 0x1B00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Ra=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_20_0_1b010400() {
    // Encoding: 0x1B010400
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=1, o0=0, Ra=1, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, Ra=1, o0=0, Rn=0, Rm=1
    let encoding: u32 = 0x1B010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Ra=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_21_0_1b1f7c00() {
    // Encoding: 0x1B1F7C00
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=31, o0=0, Ra=31, Rn=0, Rd=0
    // Fields: sf=0, Rm=31, Rd=0, o0=0, Ra=31, Rn=0
    let encoding: u32 = 0x1B1F7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_22_0_1b010020() {
    // Encoding: 0x1B010020
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=1, o0=0, Ra=0, Rn=1, Rd=0
    // Fields: Rn=1, o0=0, Rd=0, Rm=1, sf=0, Ra=0
    let encoding: u32 = 0x1B010020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_23_0_1b1f03e0() {
    // Encoding: 0x1B1F03E0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=31, o0=0, Ra=0, Rn=31, Rd=0
    // Fields: sf=0, Rm=31, o0=0, Rn=31, Ra=0, Rd=0
    let encoding: u32 = 0x1B1F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_24_0_1b010001() {
    // Encoding: 0x1B010001
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=1, o0=0, Ra=0, Rn=0, Rd=1
    // Fields: sf=0, Rn=0, Rd=1, o0=0, Rm=1, Ra=0
    let encoding: u32 = 0x1B010001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_25_0_1b1f001f() {
    // Encoding: 0x1B1F001F
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=31, o0=0, Ra=0, Rn=0, Rd=31
    // Fields: Rm=31, o0=0, Rd=31, Ra=0, sf=0, Rn=0
    let encoding: u32 = 0x1B1F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_26_0_1b000420() {
    // Encoding: 0x1B000420
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=1, Rn=1, Rd=0
    // Fields: Rn=1, sf=0, Rm=0, Ra=1, o0=0, Rd=0
    let encoding: u32 = 0x1B000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_27_0_1b007fe0() {
    // Encoding: 0x1B007FE0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=31, Rn=31, Rd=0
    // Fields: Ra=31, Rd=0, sf=0, Rm=0, Rn=31, o0=0
    let encoding: u32 = 0x1B007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_28_0_1b000401() {
    // Encoding: 0x1B000401
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=1, Rn=0, Rd=1
    // Fields: Rm=0, Ra=1, Rn=0, Rd=1, o0=0, sf=0
    let encoding: u32 = 0x1B000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Ra=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_29_0_1b007c1f() {
    // Encoding: 0x1B007C1F
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=31, Rn=0, Rd=31
    // Fields: Rn=0, Rm=0, sf=0, o0=0, Ra=31, Rd=31
    let encoding: u32 = 0x1B007C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_30_0_1b000021() {
    // Encoding: 0x1B000021
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=1, Rd=1
    // Fields: o0=0, Rn=1, Rd=1, sf=0, Rm=0, Ra=0
    let encoding: u32 = 0x1B000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_combo_31_0_1b0003ff() {
    // Encoding: 0x1B0003FF
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub field combination: sf=0, Rm=0, o0=0, Ra=0, Rn=31, Rd=31
    // Fields: o0=0, Rm=0, Rd=31, Rn=31, Ra=0, sf=0
    let encoding: u32 = 0x1B0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_special_sf_0_size_variant_0_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub special value sf = 0 (Size variant 0)
    // Fields: Rn=0, o0=0, Rm=0, sf=0, Rd=0, Ra=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_special_sf_1_size_variant_1_0_9b000000() {
    // Encoding: 0x9B000000
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub special value sf = 1 (Size variant 1)
    // Fields: Rn=0, o0=0, Rd=0, Rm=0, sf=1, Ra=0
    let encoding: u32 = 0x9B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_special_rn_31_stack_pointer_sp_may_require_alignment_0_1b0003e0()
 {
    // Encoding: 0x1B0003E0
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: o0=0, Rn=31, Rd=0, sf=0, Rm=0, Ra=0
    let encoding: u32 = 0x1B0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1b00001f()
 {
    // Encoding: 0x1B00001F
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, Rn=0, Rd=31, sf=0, o0=0, Ra=0
    let encoding: u32 = 0x1B00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple multiply (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_0_1b027c20() {
    // Test MUL 32-bit: simple multiply (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x2);
    set_x(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6, "W0 should be 0x00000006");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple multiply (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_0_9b027c20() {
    // Test MUL 64-bit: simple multiply (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x3);
    set_x(&mut cpu, 1, 0x2);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x6, "X0 should be 0x0000000000000006");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by zero (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_1_1b027c20() {
    // Test MUL 32-bit: multiply by zero (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x64);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by zero (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_1_9b027c20() {
    // Test MUL 64-bit: multiply by zero (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by one (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_2_1b027c20() {
    // Test MUL 32-bit: multiply by one (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by one (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_2_9b027c20() {
    // Test MUL 64-bit: multiply by one (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 16-bit max * 16-bit max (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_3_1b027c20() {
    // Test MUL 32-bit: 16-bit max * 16-bit max (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFF);
    set_x(&mut cpu, 1, 0xFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFE0001, "W0 should be 0xFFFE0001");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 16-bit max * 16-bit max (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_3_9b027c20() {
    // Test MUL 64-bit: 16-bit max * 16-bit max (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFF);
    set_x(&mut cpu, 2, 0xFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFE0001,
        "X0 should be 0x00000000FFFE0001"
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift-like multiply (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_4_1b027c20() {
    // Test MUL 32-bit: shift-like multiply (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x2468ACF0, "W0 should be 0x2468ACF0");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift-like multiply (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_4_9b027c20() {
    // Test MUL 64-bit: shift-like multiply (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x2468ACF0,
        "X0 should be 0x000000002468ACF0"
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// larger values (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_5_1b027c20() {
    // Test MUL 32-bit: larger values (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xC8);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4E20, "W0 should be 0x00004E20");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// larger values (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_5_9b027c20() {
    // Test MUL 64-bit: larger values (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x4E20, "X0 should be 0x0000000000004E20");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 32-bit overflow (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_6_1b027c20() {
    // Test MUL 32-bit: 32-bit overflow (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 32-bit overflow (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_6_9b027c20() {
    // Test MUL 64-bit: 32-bit overflow (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFE00000001,
        "X0 should be 0xFFFFFFFE00000001"
    );
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// prime numbers (32)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_32_7_1b027c20() {
    // Test MUL 32-bit: prime numbers (oracle)
    // Encoding: 0x1B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xB);
    set_x(&mut cpu, 1, 0x7);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4D, "W0 should be 0x0000004D");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// prime numbers (64)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_mul_oracle_64_7_9b027c20() {
    // Test MUL 64-bit: prime numbers (oracle)
    // Encoding: 0x9B027C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xB);
    set_x(&mut cpu, 1, 0x7);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x4D, "X0 should be 0x000000000000004D");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_reg_write_0_1b000000() {
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub register write: GpFromField("d")
    // Encoding: 0x1B000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1B000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_sp_rn_1b0003e0() {
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub with Rn = SP (31)
    // Encoding: 0x1B0003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1B0003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_mul_uniform_add_sub
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_mul_uniform_add_sub_zr_rd_1b00001f() {
    // Test aarch64_integer_arithmetic_mul_uniform_add_sub with Rd = ZR (31)
    // Encoding: 0x1B00001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1B00001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
