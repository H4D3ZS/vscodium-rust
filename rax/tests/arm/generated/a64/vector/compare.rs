//! A64 vector compare tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_u_0_min_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field U = 0 (Min)
    // Fields: Rd=0, size=0, U=0, op=0, Rn=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_u_1_max_8800_7e208800() {
    // Encoding: 0x7E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field U = 1 (Max)
    // Fields: op=0, U=1, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x7E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_size_0_min_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field size = 0 (Min)
    // Fields: Rn=0, op=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_size_1_poweroftwo_8800_5e608800() {
    // Encoding: 0x5E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field size = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, size=1, U=0, op=0
    let encoding: u32 = 0x5E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_size_2_poweroftwo_8800_5ea08800() {
    // Encoding: 0x5EA08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field size = 2 (PowerOfTwo)
    // Fields: U=0, size=2, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EA08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_size_3_max_8800_5ee08800() {
    // Encoding: 0x5EE08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field size = 3 (Max)
    // Fields: op=0, Rd=0, Rn=0, U=0, size=3
    let encoding: u32 = 0x5EE08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_op_0_min_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field op = 0 (Min)
    // Fields: size=0, op=0, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_op_1_max_8800_5e209800() {
    // Encoding: 0x5E209800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field op = 1 (Max)
    // Fields: U=0, Rd=0, Rn=0, size=0, op=1
    let encoding: u32 = 0x5E209800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rn_0_min_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rn = 0 (Min)
    // Fields: U=0, Rn=0, size=0, Rd=0, op=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rn_1_poweroftwo_8800_5e208820() {
    // Encoding: 0x5E208820
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rd=0, U=0, op=0, Rn=1
    let encoding: u32 = 0x5E208820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rn_30_poweroftwominusone_8800_5e208bc0()
 {
    // Encoding: 0x5E208BC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, op=0, Rn=30, Rd=0, size=0
    let encoding: u32 = 0x5E208BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rn_31_max_8800_5e208be0() {
    // Encoding: 0x5E208BE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rn = 31 (Max)
    // Fields: Rn=31, op=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x5E208BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rd_0_min_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rd = 0 (Min)
    // Fields: size=0, Rn=0, Rd=0, op=0, U=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rd_1_poweroftwo_8800_5e208801() {
    // Encoding: 0x5E208801
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, size=0, op=0, U=0, Rn=0
    let encoding: u32 = 0x5E208801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rd_30_poweroftwominusone_8800_5e20881e()
 {
    // Encoding: 0x5E20881E
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, op=0, Rn=0, Rd=30, U=0
    let encoding: u32 = 0x5E20881E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_field_rd_31_max_8800_5e20881f() {
    // Encoding: 0x5E20881F
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field Rd = 31 (Max)
    // Fields: Rd=31, size=0, op=0, U=0, Rn=0
    let encoding: u32 = 0x5E20881F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_0_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_1_8800_7e208800() {
    // Encoding: 0x7E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=1, size=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, U=1, size=0, Rd=0
    let encoding: u32 = 0x7E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_2_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, Rn=0, size=0, U=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_3_8800_5e608800() {
    // Encoding: 0x5E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=1, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, Rd=0, U=0, size=1
    let encoding: u32 = 0x5E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_4_8800_5ea08800() {
    // Encoding: 0x5EA08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=2, op=0, Rn=0, Rd=0
    // Fields: size=2, Rd=0, U=0, Rn=0, op=0
    let encoding: u32 = 0x5EA08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_5_8800_5ee08800() {
    // Encoding: 0x5EE08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=3, op=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, size=3, Rn=0, op=0
    let encoding: u32 = 0x5EE08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_6_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: size=0, U=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_7_8800_5e209800() {
    // Encoding: 0x5E209800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, size=0, op=1, U=0
    let encoding: u32 = 0x5E209800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_8_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_9_8800_5e208820() {
    // Encoding: 0x5E208820
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=1, Rd=0
    // Fields: Rn=1, op=0, U=0, size=0, Rd=0
    let encoding: u32 = 0x5E208820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_10_8800_5e208bc0() {
    // Encoding: 0x5E208BC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=30, Rd=0
    // Fields: Rd=0, U=0, op=0, Rn=30, size=0
    let encoding: u32 = 0x5E208BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_11_8800_5e208be0() {
    // Encoding: 0x5E208BE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=31, Rd=0
    // Fields: size=0, Rd=0, Rn=31, U=0, op=0
    let encoding: u32 = 0x5E208BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_12_8800_5e208800() {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, size=0, U=0, op=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_13_8800_5e208801() {
    // Encoding: 0x5E208801
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=1
    // Fields: size=0, Rn=0, U=0, Rd=1, op=0
    let encoding: u32 = 0x5E208801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_14_8800_5e20881e() {
    // Encoding: 0x5E20881E
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=30
    // Fields: Rd=30, size=0, U=0, op=0, Rn=0
    let encoding: u32 = 0x5E20881E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_15_8800_5e20881f() {
    // Encoding: 0x5E20881F
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, size=0, op=0, U=0
    let encoding: u32 = 0x5E20881F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_16_8800_5e208821() {
    // Encoding: 0x5E208821
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=1, Rd=1
    // Fields: op=0, U=0, size=0, Rn=1, Rd=1
    let encoding: u32 = 0x5E208821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_combo_17_8800_5e208bff() {
    // Encoding: 0x5E208BFF
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd field combination: U=0, size=0, op=0, Rn=31, Rd=31
    // Fields: Rn=31, op=0, size=0, Rd=31, U=0
    let encoding: u32 = 0x5E208BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_special_size_0_size_variant_0_34816_5e208800()
 {
    // Encoding: 0x5E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd special value size = 0 (Size variant 0)
    // Fields: op=0, Rn=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x5E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_special_size_1_size_variant_1_34816_5e608800()
 {
    // Encoding: 0x5E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd special value size = 1 (Size variant 1)
    // Fields: U=0, Rn=0, size=1, op=0, Rd=0
    let encoding: u32 = 0x5E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_special_size_2_size_variant_2_34816_5ea08800()
 {
    // Encoding: 0x5EA08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd special value size = 2 (Size variant 2)
    // Fields: size=2, op=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EA08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_special_size_3_size_variant_3_34816_5ee08800()
 {
    // Encoding: 0x5EE08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd special value size = 3 (Size variant 3)
    // Fields: U=0, size=3, Rd=0, Rn=0, op=0
    let encoding: u32 = 0x5EE08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_34816_5e608be0()
 {
    // Encoding: 0x5E608BE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rd=0, U=0, Rn=31, op=0
    let encoding: u32 = 0x5E608BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_34816_5e60881f()
 {
    // Encoding: 0x5E60881F
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, size=1, op=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E60881F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_q_0_min_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Q = 0 (Min)
    // Fields: U=0, op=0, Q=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_q_1_max_8800_4e208800() {
    // Encoding: 0x4E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Q = 1 (Max)
    // Fields: U=0, Rd=0, Q=1, op=0, size=0, Rn=0
    let encoding: u32 = 0x4E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_u_0_min_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field U = 0 (Min)
    // Fields: size=0, op=0, Rd=0, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_u_1_max_8800_2e208800() {
    // Encoding: 0x2E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field U = 1 (Max)
    // Fields: U=1, size=0, op=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_size_0_min_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field size = 0 (Min)
    // Fields: U=0, Q=0, size=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_size_1_poweroftwo_8800_0e608800() {
    // Encoding: 0x0E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field size = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, Q=0, op=0, size=1, Rd=0
    let encoding: u32 = 0x0E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_size_2_poweroftwo_8800_0ea08800() {
    // Encoding: 0x0EA08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field size = 2 (PowerOfTwo)
    // Fields: Q=0, U=0, op=0, Rn=0, Rd=0, size=2
    let encoding: u32 = 0x0EA08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_size_3_max_8800_0ee08800() {
    // Encoding: 0x0EE08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field size = 3 (Max)
    // Fields: Rn=0, Rd=0, size=3, U=0, Q=0, op=0
    let encoding: u32 = 0x0EE08800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_op_0_min_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field op = 0 (Min)
    // Fields: Q=0, U=0, op=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_op_1_max_8800_0e209800() {
    // Encoding: 0x0E209800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field op = 1 (Max)
    // Fields: U=0, Rn=0, Rd=0, size=0, op=1, Q=0
    let encoding: u32 = 0x0E209800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rn_0_min_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rn = 0 (Min)
    // Fields: Rd=0, size=0, Q=0, U=0, op=0, Rn=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rn_1_poweroftwo_8800_0e208820() {
    // Encoding: 0x0E208820
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rn = 1 (PowerOfTwo)
    // Fields: size=0, U=0, Q=0, op=0, Rd=0, Rn=1
    let encoding: u32 = 0x0E208820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rn_30_poweroftwominusone_8800_0e208bc0()
 {
    // Encoding: 0x0E208BC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, U=0, size=0, Rn=30, op=0, Rd=0
    let encoding: u32 = 0x0E208BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rn_31_max_8800_0e208be0() {
    // Encoding: 0x0E208BE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rn = 31 (Max)
    // Fields: Q=0, U=0, op=0, Rn=31, Rd=0, size=0
    let encoding: u32 = 0x0E208BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rd_0_min_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rd = 0 (Min)
    // Fields: Q=0, size=0, U=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rd_1_poweroftwo_8800_0e208801() {
    // Encoding: 0x0E208801
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rd = 1 (PowerOfTwo)
    // Fields: size=0, Rd=1, Q=0, U=0, op=0, Rn=0
    let encoding: u32 = 0x0E208801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rd_30_poweroftwominusone_8800_0e20881e()
 {
    // Encoding: 0x0E20881E
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, op=0, U=0, size=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E20881E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_field_rd_31_max_8800_0e20881f() {
    // Encoding: 0x0E20881F
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field Rd = 31 (Max)
    // Fields: Rn=0, size=0, Rd=31, U=0, op=0, Q=0
    let encoding: u32 = 0x0E20881F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_0_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, Rn=0, size=0, U=0, op=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_1_8800_4e208800() {
    // Encoding: 0x4E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=1, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, Rd=0, size=0, Q=1, op=0
    let encoding: u32 = 0x4E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_2_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rn=0, U=0, Rd=0, op=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_3_8800_2e208800() {
    // Encoding: 0x2E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=1, size=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, op=0, U=1, Rn=0, size=0
    let encoding: u32 = 0x2E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_4_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, op=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_5_8800_0e608800() {
    // Encoding: 0x0E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=1, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, size=1, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_6_8800_0ea08800() {
    // Encoding: 0x0EA08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=2, op=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, Q=0, Rd=0, op=0, size=2
    let encoding: u32 = 0x0EA08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_7_8800_0ee08800() {
    // Encoding: 0x0EE08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=3, op=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, size=3, Q=0, op=0, Rn=0
    let encoding: u32 = 0x0EE08800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_8_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: size=0, op=0, Rd=0, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_9_8800_0e209800() {
    // Encoding: 0x0E209800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, size=0, Q=0, op=1, U=0
    let encoding: u32 = 0x0E209800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_10_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, op=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_11_8800_0e208820() {
    // Encoding: 0x0E208820
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=1, Rd=0
    // Fields: op=0, Q=0, U=0, size=0, Rn=1, Rd=0
    let encoding: u32 = 0x0E208820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_12_8800_0e208bc0() {
    // Encoding: 0x0E208BC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=30, Rd=0
    // Fields: size=0, Q=0, Rn=30, U=0, op=0, Rd=0
    let encoding: u32 = 0x0E208BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_13_8800_0e208be0() {
    // Encoding: 0x0E208BE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=31, Rd=0
    // Fields: U=0, op=0, Rn=31, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E208BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_14_8800_0e208800() {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, op=0, size=0, Rd=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_15_8800_0e208801() {
    // Encoding: 0x0E208801
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=1
    // Fields: Q=0, op=0, size=0, U=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E208801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_16_8800_0e20881e() {
    // Encoding: 0x0E20881E
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=30
    // Fields: Rd=30, size=0, op=0, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0E20881E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_17_8800_0e20881f() {
    // Encoding: 0x0E20881F
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=31
    // Fields: size=0, Rd=31, Rn=0, op=0, U=0, Q=0
    let encoding: u32 = 0x0E20881F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_18_8800_0e208821() {
    // Encoding: 0x0E208821
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=1, Rd=1
    // Fields: Rn=1, size=0, U=0, op=0, Q=0, Rd=1
    let encoding: u32 = 0x0E208821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_combo_19_8800_0e208bff() {
    // Encoding: 0x0E208BFF
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd field combination: Q=0, U=0, size=0, op=0, Rn=31, Rd=31
    // Fields: Rd=31, U=0, size=0, Q=0, op=0, Rn=31
    let encoding: u32 = 0x0E208BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_q_0_size_variant_0_34816_0e608800()
 {
    // Encoding: 0x0E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, size=1, op=0, Q=0, U=0
    let encoding: u32 = 0x0E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_q_1_size_variant_1_34816_4e608800()
 {
    // Encoding: 0x4E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value Q = 1 (Size variant 1)
    // Fields: size=1, op=0, Rd=0, Q=1, U=0, Rn=0
    let encoding: u32 = 0x4E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_size_0_size_variant_0_34816_0e208800()
 {
    // Encoding: 0x0E208800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value size = 0 (Size variant 0)
    // Fields: Rd=0, U=0, Rn=0, op=0, size=0, Q=0
    let encoding: u32 = 0x0E208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_size_1_size_variant_1_34816_0e608800()
 {
    // Encoding: 0x0E608800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value size = 1 (Size variant 1)
    // Fields: U=0, size=1, op=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_size_2_size_variant_2_34816_0ea08800()
 {
    // Encoding: 0x0EA08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value size = 2 (Size variant 2)
    // Fields: size=2, Q=0, op=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x0EA08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_size_3_size_variant_3_34816_0ee08800()
 {
    // Encoding: 0x0EE08800
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value size = 3 (Size variant 3)
    // Fields: Q=0, U=0, Rd=0, size=3, Rn=0, op=0
    let encoding: u32 = 0x0EE08800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_rn_31_stack_pointer_sp_may_require_alignment_34816_0e608be0()
 {
    // Encoding: 0x0E608BE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, op=0, Rd=0, U=0, Rn=31, size=1
    let encoding: u32 = 0x0E608BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_34816_0e60881f()
 {
    // Encoding: 0x0E60881F
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, Rn=0, Rd=31, size=1, op=0, U=0
    let encoding: u32 = 0x0E60881F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_reg_write_0_5e208800() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd register write: SimdFromField("d")
    // Encoding: 0x5E208800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E208800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_sp_rn_5e208be0() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd with Rn = SP (31)
    // Encoding: 0x5E208BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E208BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_zr_rd_5e20881f() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd with Rd = ZR (31)
    // Encoding: 0x5E20881F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20881F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_reg_write_0_0e208800() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd register write: SimdFromField("d")
    // Encoding: 0x0E208800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E208800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_sp_rn_0e208be0() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd with Rn = SP (31)
    // Encoding: 0x0E208BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E208BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_zr_rd_0e20881f() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_bulk_simd with Rd = ZR (31)
    // Encoding: 0x0E20881F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20881F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_u_0_min_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field U = 0 (Min)
    // Fields: U=0, E=0, Rm=0, Rd=0, ac=0, Rn=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_u_1_max_2400_7e402400() {
    // Encoding: 0x7E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field U = 1 (Max)
    // Fields: Rm=0, Rd=0, Rn=0, U=1, ac=0, E=0
    let encoding: u32 = 0x7E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_e_0_min_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field E = 0 (Min)
    // Fields: U=0, ac=0, Rn=0, Rm=0, E=0, Rd=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_e_1_max_2400_5ec02400() {
    // Encoding: 0x5EC02400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field E = 1 (Max)
    // Fields: U=0, E=1, ac=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x5EC02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rm_0_min_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rm = 0 (Min)
    // Fields: Rn=0, Rm=0, E=0, ac=0, Rd=0, U=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rm_1_poweroftwo_2400_5e412400()
{
    // Encoding: 0x5E412400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, E=0, ac=0, Rd=0, Rm=1, U=0
    let encoding: u32 = 0x5E412400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rm_30_poweroftwominusone_2400_5e5e2400()
 {
    // Encoding: 0x5E5E2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=0, E=0, ac=0, Rm=30, Rd=0
    let encoding: u32 = 0x5E5E2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rm_31_max_2400_5e5f2400() {
    // Encoding: 0x5E5F2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rm = 31 (Max)
    // Fields: ac=0, Rd=0, U=0, E=0, Rm=31, Rn=0
    let encoding: u32 = 0x5E5F2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_ac_0_min_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field ac = 0 (Min)
    // Fields: Rn=0, U=0, E=0, Rm=0, ac=0, Rd=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_ac_1_max_2400_5e402c00() {
    // Encoding: 0x5E402C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field ac = 1 (Max)
    // Fields: E=0, Rn=0, Rd=0, ac=1, U=0, Rm=0
    let encoding: u32 = 0x5E402C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rn_0_min_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rn = 0 (Min)
    // Fields: Rd=0, U=0, Rm=0, E=0, ac=0, Rn=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rn_1_poweroftwo_2400_5e402420()
{
    // Encoding: 0x5E402420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rn = 1 (PowerOfTwo)
    // Fields: E=0, U=0, ac=0, Rn=1, Rd=0, Rm=0
    let encoding: u32 = 0x5E402420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rn_30_poweroftwominusone_2400_5e4027c0()
 {
    // Encoding: 0x5E4027C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, E=0, Rd=0, ac=0, Rn=30, Rm=0
    let encoding: u32 = 0x5E4027C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rn_31_max_2400_5e4027e0() {
    // Encoding: 0x5E4027E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rn = 31 (Max)
    // Fields: ac=0, E=0, Rd=0, Rm=0, Rn=31, U=0
    let encoding: u32 = 0x5E4027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rd_0_min_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rd = 0 (Min)
    // Fields: Rd=0, E=0, Rm=0, U=0, ac=0, Rn=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rd_1_poweroftwo_2400_5e402401()
{
    // Encoding: 0x5E402401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, E=0, U=0, Rm=0, ac=0, Rn=0
    let encoding: u32 = 0x5E402401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rd_30_poweroftwominusone_2400_5e40241e()
 {
    // Encoding: 0x5E40241E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, U=0, ac=0, Rd=30, Rn=0, E=0
    let encoding: u32 = 0x5E40241E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_field_rd_31_max_2400_5e40241f() {
    // Encoding: 0x5E40241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field Rd = 31 (Max)
    // Fields: ac=0, U=0, Rm=0, E=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E40241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_0_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, E=0, ac=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_1_2400_7e402400() {
    // Encoding: 0x7E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=1, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: ac=0, Rd=0, E=0, U=1, Rn=0, Rm=0
    let encoding: u32 = 0x7E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_2_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, U=0, E=0, ac=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_3_2400_5ec02400() {
    // Encoding: 0x5EC02400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=1, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, Rn=0, ac=0, U=0, E=1
    let encoding: u32 = 0x5EC02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_4_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rm=0, ac=0, E=0, Rn=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_5_2400_5e412400() {
    // Encoding: 0x5E412400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=1, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rm=1, E=0, Rn=0, ac=0
    let encoding: u32 = 0x5E412400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_6_2400_5e5e2400() {
    // Encoding: 0x5E5E2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=30, ac=0, Rn=0, Rd=0
    // Fields: ac=0, U=0, Rn=0, Rd=0, Rm=30, E=0
    let encoding: u32 = 0x5E5E2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_7_2400_5e5f2400() {
    // Encoding: 0x5E5F2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=31, ac=0, Rn=0, Rd=0
    // Fields: U=0, Rm=31, ac=0, Rd=0, E=0, Rn=0
    let encoding: u32 = 0x5E5F2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_8_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: U=0, ac=0, Rd=0, Rn=0, E=0, Rm=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_9_2400_5e402c00() {
    // Encoding: 0x5E402C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=1, Rn=0, Rd=0
    // Fields: E=0, U=0, Rm=0, Rn=0, ac=1, Rd=0
    let encoding: u32 = 0x5E402C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_10_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: U=0, Rm=0, ac=0, E=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_11_2400_5e402420() {
    // Encoding: 0x5E402420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=1, Rd=0
    // Fields: E=0, ac=0, U=0, Rd=0, Rm=0, Rn=1
    let encoding: u32 = 0x5E402420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_12_2400_5e4027c0() {
    // Encoding: 0x5E4027C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, ac=0, Rm=0, U=0, E=0
    let encoding: u32 = 0x5E4027C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_13_2400_5e4027e0() {
    // Encoding: 0x5E4027E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=31, Rd=0
    // Fields: Rm=0, Rn=31, U=0, E=0, Rd=0, ac=0
    let encoding: u32 = 0x5E4027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_14_2400_5e402400() {
    // Encoding: 0x5E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: ac=0, Rd=0, Rn=0, U=0, E=0, Rm=0
    let encoding: u32 = 0x5E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_15_2400_5e402401() {
    // Encoding: 0x5E402401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=1
    // Fields: E=0, ac=0, Rd=1, Rn=0, Rm=0, U=0
    let encoding: u32 = 0x5E402401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_16_2400_5e40241e() {
    // Encoding: 0x5E40241E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=30
    // Fields: U=0, E=0, ac=0, Rm=0, Rn=0, Rd=30
    let encoding: u32 = 0x5E40241E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_17_2400_5e40241f() {
    // Encoding: 0x5E40241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=0, Rd=31
    // Fields: ac=0, Rd=31, U=0, E=0, Rn=0, Rm=0
    let encoding: u32 = 0x5E40241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_18_2400_5e412420() {
    // Encoding: 0x5E412420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=1, ac=0, Rn=1, Rd=0
    // Fields: Rm=1, E=0, Rn=1, Rd=0, U=0, ac=0
    let encoding: u32 = 0x5E412420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_19_2400_5e5f27e0() {
    // Encoding: 0x5E5F27E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=31, ac=0, Rn=31, Rd=0
    // Fields: ac=0, Rn=31, Rd=0, E=0, Rm=31, U=0
    let encoding: u32 = 0x5E5F27E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_20_2400_5e412401() {
    // Encoding: 0x5E412401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=1, ac=0, Rn=0, Rd=1
    // Fields: Rm=1, ac=0, Rn=0, Rd=1, E=0, U=0
    let encoding: u32 = 0x5E412401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_21_2400_5e5f241f() {
    // Encoding: 0x5E5F241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=31, ac=0, Rn=0, Rd=31
    // Fields: Rd=31, ac=0, U=0, Rm=31, E=0, Rn=0
    let encoding: u32 = 0x5E5F241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_22_2400_5e402421() {
    // Encoding: 0x5E402421
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=1, Rd=1
    // Fields: E=0, U=0, ac=0, Rm=0, Rd=1, Rn=1
    let encoding: u32 = 0x5E402421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_combo_23_2400_5e4027ff() {
    // Encoding: 0x5E4027FF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd field combination: U=0, E=0, Rm=0, ac=0, Rn=31, Rd=31
    // Fields: E=0, Rm=0, U=0, ac=0, Rd=31, Rn=31
    let encoding: u32 = 0x5E4027FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_9216_5e4027e0()
 {
    // Encoding: 0x5E4027E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, Rm=0, E=0, Rd=0, Rn=31, ac=0
    let encoding: u32 = 0x5E4027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_9216_5e40241f()
 {
    // Encoding: 0x5E40241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: ac=0, Rd=31, Rn=0, E=0, U=0, Rm=0
    let encoding: u32 = 0x5E40241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_u_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field U = 0 (Min)
    // Fields: E=0, Rm=0, Rn=0, U=0, ac=0, Rd=0, sz=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_u_1_max_e400_7e20e400() {
    // Encoding: 0x7E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field U = 1 (Max)
    // Fields: U=1, sz=0, ac=0, Rn=0, E=0, Rd=0, Rm=0
    let encoding: u32 = 0x7E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_e_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field E = 0 (Min)
    // Fields: ac=0, U=0, Rm=0, sz=0, Rn=0, E=0, Rd=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_e_1_max_e400_5ea0e400() {
    // Encoding: 0x5EA0E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field E = 1 (Max)
    // Fields: U=0, E=1, sz=0, ac=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x5EA0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_sz_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field sz = 0 (Min)
    // Fields: ac=0, Rn=0, Rd=0, E=0, sz=0, U=0, Rm=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_sz_1_max_e400_5e60e400() {
    // Encoding: 0x5E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field sz = 1 (Max)
    // Fields: Rm=0, U=0, ac=0, Rn=0, Rd=0, sz=1, E=0
    let encoding: u32 = 0x5E60E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rm_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rm = 0 (Min)
    // Fields: U=0, Rm=0, ac=0, Rd=0, sz=0, E=0, Rn=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rm_1_poweroftwo_e400_5e21e400() {
    // Encoding: 0x5E21E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, E=0, Rd=0, U=0, sz=0, Rm=1, ac=0
    let encoding: u32 = 0x5E21E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rm_30_poweroftwominusone_e400_5e3ee400()
 {
    // Encoding: 0x5E3EE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: ac=0, U=0, sz=0, E=0, Rd=0, Rn=0, Rm=30
    let encoding: u32 = 0x5E3EE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rm_31_max_e400_5e3fe400() {
    // Encoding: 0x5E3FE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rm = 31 (Max)
    // Fields: U=0, E=0, Rm=31, ac=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E3FE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_ac_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field ac = 0 (Min)
    // Fields: ac=0, E=0, sz=0, U=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_ac_1_max_e400_5e20ec00() {
    // Encoding: 0x5E20EC00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field ac = 1 (Max)
    // Fields: Rm=0, ac=1, Rn=0, U=0, E=0, Rd=0, sz=0
    let encoding: u32 = 0x5E20EC00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rn_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rn = 0 (Min)
    // Fields: Rd=0, E=0, U=0, sz=0, Rm=0, ac=0, Rn=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rn_1_poweroftwo_e400_5e20e420() {
    // Encoding: 0x5E20E420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, E=0, Rm=0, Rn=1, sz=0, Rd=0, ac=0
    let encoding: u32 = 0x5E20E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rn_30_poweroftwominusone_e400_5e20e7c0()
 {
    // Encoding: 0x5E20E7C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: E=0, sz=0, Rn=30, Rd=0, U=0, ac=0, Rm=0
    let encoding: u32 = 0x5E20E7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rn_31_max_e400_5e20e7e0() {
    // Encoding: 0x5E20E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, E=0, U=0, sz=0, Rm=0, ac=0
    let encoding: u32 = 0x5E20E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rd_0_min_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rd = 0 (Min)
    // Fields: Rd=0, E=0, Rm=0, sz=0, U=0, ac=0, Rn=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rd_1_poweroftwo_e400_5e20e401() {
    // Encoding: 0x5E20E401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, sz=0, U=0, E=0, ac=0, Rm=0, Rn=0
    let encoding: u32 = 0x5E20E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rd_30_poweroftwominusone_e400_5e20e41e()
 {
    // Encoding: 0x5E20E41E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=0, Rd=30, E=0, sz=0, Rm=0, ac=0
    let encoding: u32 = 0x5E20E41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_field_rd_31_max_e400_5e20e41f() {
    // Encoding: 0x5E20E41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field Rd = 31 (Max)
    // Fields: Rm=0, ac=0, U=0, Rd=31, Rn=0, sz=0, E=0
    let encoding: u32 = 0x5E20E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_0_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, ac=0, E=0, U=0, sz=0, Rm=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_1_e400_7e20e400() {
    // Encoding: 0x7E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=1, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=0, ac=0, Rm=0, Rn=0, Rd=0, U=1, sz=0
    let encoding: u32 = 0x7E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_2_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: sz=0, ac=0, Rn=0, Rd=0, Rm=0, U=0, E=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_3_e400_5ea0e400() {
    // Encoding: 0x5EA0E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=1, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=1, Rn=0, Rd=0, Rm=0, ac=0, U=0, sz=0
    let encoding: u32 = 0x5EA0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_4_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: U=0, sz=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_5_e400_5e60e400() {
    // Encoding: 0x5E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=1, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: sz=1, Rm=0, U=0, ac=0, Rn=0, Rd=0, E=0
    let encoding: u32 = 0x5E60E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_6_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, Rd=0, U=0, E=0, ac=0, sz=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_7_e400_5e21e400() {
    // Encoding: 0x5E21E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=1, ac=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, E=0, sz=0, Rm=1, ac=0
    let encoding: u32 = 0x5E21E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_8_e400_5e3ee400() {
    // Encoding: 0x5E3EE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=30, ac=0, Rn=0, Rd=0
    // Fields: E=0, ac=0, sz=0, Rn=0, Rm=30, U=0, Rd=0
    let encoding: u32 = 0x5E3EE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_9_e400_5e3fe400() {
    // Encoding: 0x5E3FE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=31, ac=0, Rn=0, Rd=0
    // Fields: U=0, sz=0, E=0, Rm=31, ac=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E3FE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_10_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=0, Rm=0, ac=0, sz=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_11_e400_5e20ec00() {
    // Encoding: 0x5E20EC00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=1, Rn=0, Rd=0
    // Fields: U=0, sz=0, E=0, Rn=0, Rm=0, Rd=0, ac=1
    let encoding: u32 = 0x5E20EC00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_12_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=0, Rd=0, sz=0, Rm=0, Rn=0, U=0, ac=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_13_e400_5e20e420() {
    // Encoding: 0x5E20E420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=1, Rd=0
    // Fields: Rd=0, Rm=0, U=0, ac=0, sz=0, E=0, Rn=1
    let encoding: u32 = 0x5E20E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_14_e400_5e20e7c0() {
    // Encoding: 0x5E20E7C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=30, Rd=0
    // Fields: Rm=0, E=0, U=0, ac=0, sz=0, Rn=30, Rd=0
    let encoding: u32 = 0x5E20E7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_15_e400_5e20e7e0() {
    // Encoding: 0x5E20E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=31, Rd=0
    // Fields: U=0, E=0, Rn=31, Rd=0, ac=0, Rm=0, sz=0
    let encoding: u32 = 0x5E20E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_16_e400_5e20e400() {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=0, Rm=0, U=0, Rn=0, Rd=0, sz=0, ac=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_17_e400_5e20e401() {
    // Encoding: 0x5E20E401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=1
    // Fields: E=0, Rn=0, Rm=0, Rd=1, sz=0, U=0, ac=0
    let encoding: u32 = 0x5E20E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_18_e400_5e20e41e() {
    // Encoding: 0x5E20E41E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=30
    // Fields: sz=0, Rd=30, ac=0, E=0, Rn=0, U=0, Rm=0
    let encoding: u32 = 0x5E20E41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_19_e400_5e20e41f() {
    // Encoding: 0x5E20E41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=31
    // Fields: Rd=31, U=0, Rn=0, sz=0, E=0, Rm=0, ac=0
    let encoding: u32 = 0x5E20E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_20_e400_5e21e420() {
    // Encoding: 0x5E21E420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=1, ac=0, Rn=1, Rd=0
    // Fields: ac=0, E=0, Rm=1, Rd=0, U=0, sz=0, Rn=1
    let encoding: u32 = 0x5E21E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_21_e400_5e3fe7e0() {
    // Encoding: 0x5E3FE7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=31, ac=0, Rn=31, Rd=0
    // Fields: ac=0, Rd=0, sz=0, E=0, U=0, Rn=31, Rm=31
    let encoding: u32 = 0x5E3FE7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_22_e400_5e21e401() {
    // Encoding: 0x5E21E401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=1, ac=0, Rn=0, Rd=1
    // Fields: sz=0, Rm=1, Rn=0, U=0, E=0, ac=0, Rd=1
    let encoding: u32 = 0x5E21E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_23_e400_5e3fe41f() {
    // Encoding: 0x5E3FE41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=31, ac=0, Rn=0, Rd=31
    // Fields: E=0, Rm=31, Rd=31, ac=0, U=0, sz=0, Rn=0
    let encoding: u32 = 0x5E3FE41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_24_e400_5e20e421() {
    // Encoding: 0x5E20E421
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=1, Rd=1
    // Fields: U=0, E=0, Rm=0, ac=0, Rd=1, sz=0, Rn=1
    let encoding: u32 = 0x5E20E421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_combo_25_e400_5e20e7ff() {
    // Encoding: 0x5E20E7FF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd field combination: U=0, E=0, sz=0, Rm=0, ac=0, Rn=31, Rd=31
    // Fields: ac=0, E=0, sz=0, U=0, Rn=31, Rm=0, Rd=31
    let encoding: u32 = 0x5E20E7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_special_sz_0_size_variant_0_58368_5e20e400()
 {
    // Encoding: 0x5E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd special value sz = 0 (Size variant 0)
    // Fields: E=0, sz=0, Rm=0, ac=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_special_sz_1_size_variant_1_58368_5e60e400()
 {
    // Encoding: 0x5E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd special value sz = 1 (Size variant 1)
    // Fields: E=0, U=0, ac=0, Rn=0, sz=1, Rd=0, Rm=0
    let encoding: u32 = 0x5E60E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_58368_5e60e7e0()
 {
    // Encoding: 0x5E60E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, sz=1, Rn=31, U=0, ac=0, Rm=0, E=0
    let encoding: u32 = 0x5E60E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_58368_5e60e41f()
 {
    // Encoding: 0x5E60E41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: E=0, sz=1, Rm=0, Rd=31, U=0, ac=0, Rn=0
    let encoding: u32 = 0x5E60E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_q_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Q = 0 (Min)
    // Fields: U=0, E=0, Rm=0, ac=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_q_1_max_2400_4e402400() {
    // Encoding: 0x4E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Q = 1 (Max)
    // Fields: E=0, ac=0, Rm=0, Q=1, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x4E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_u_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field U = 0 (Min)
    // Fields: Q=0, Rn=0, E=0, Rd=0, Rm=0, U=0, ac=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_u_1_max_2400_2e402400() {
    // Encoding: 0x2E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field U = 1 (Max)
    // Fields: U=1, Rd=0, ac=0, Rn=0, Q=0, E=0, Rm=0
    let encoding: u32 = 0x2E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_e_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field E = 0 (Min)
    // Fields: E=0, Rd=0, Rm=0, ac=0, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_e_1_max_2400_0ec02400() {
    // Encoding: 0x0EC02400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field E = 1 (Max)
    // Fields: U=0, Q=0, Rn=0, Rm=0, ac=0, E=1, Rd=0
    let encoding: u32 = 0x0EC02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rm_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rm = 0 (Min)
    // Fields: U=0, ac=0, Rd=0, E=0, Q=0, Rn=0, Rm=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rm_1_poweroftwo_2400_0e412400()
{
    // Encoding: 0x0E412400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, Rm=1, Rd=0, E=0, ac=0, Q=0
    let encoding: u32 = 0x0E412400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rm_30_poweroftwominusone_2400_0e5e2400()
 {
    // Encoding: 0x0E5E2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, E=0, Rm=30, U=0, Q=0, ac=0, Rn=0
    let encoding: u32 = 0x0E5E2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rm_31_max_2400_0e5f2400() {
    // Encoding: 0x0E5F2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rm = 31 (Max)
    // Fields: E=0, U=0, ac=0, Rn=0, Rd=0, Q=0, Rm=31
    let encoding: u32 = 0x0E5F2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_ac_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field ac = 0 (Min)
    // Fields: ac=0, U=0, Q=0, Rn=0, Rd=0, E=0, Rm=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_ac_1_max_2400_0e402c00() {
    // Encoding: 0x0E402C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field ac = 1 (Max)
    // Fields: ac=1, Q=0, E=0, Rm=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E402C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rn_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rn = 0 (Min)
    // Fields: Q=0, Rd=0, E=0, Rm=0, ac=0, Rn=0, U=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rn_1_poweroftwo_2400_0e402420()
{
    // Encoding: 0x0E402420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, ac=0, Rd=0, Rm=0, U=0, E=0, Rn=1
    let encoding: u32 = 0x0E402420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rn_30_poweroftwominusone_2400_0e4027c0()
 {
    // Encoding: 0x0E4027C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, U=0, ac=0, Rd=0, Rn=30, Q=0, E=0
    let encoding: u32 = 0x0E4027C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rn_31_max_2400_0e4027e0() {
    // Encoding: 0x0E4027E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rn = 31 (Max)
    // Fields: Rn=31, Q=0, Rd=0, ac=0, U=0, E=0, Rm=0
    let encoding: u32 = 0x0E4027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rd_0_min_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rd = 0 (Min)
    // Fields: ac=0, Rn=0, Rd=0, Q=0, U=0, E=0, Rm=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rd_1_poweroftwo_2400_0e402401()
{
    // Encoding: 0x0E402401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, U=0, ac=0, Rn=0, E=0, Q=0, Rm=0
    let encoding: u32 = 0x0E402401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rd_30_poweroftwominusone_2400_0e40241e()
 {
    // Encoding: 0x0E40241E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: ac=0, Rn=0, Rm=0, Rd=30, Q=0, E=0, U=0
    let encoding: u32 = 0x0E40241E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_field_rd_31_max_2400_0e40241f() {
    // Encoding: 0x0E40241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31, E=0, U=0, Q=0, Rm=0, ac=0
    let encoding: u32 = 0x0E40241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_0_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, Rn=0, ac=0, U=0, E=0, Q=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_1_2400_4e402400() {
    // Encoding: 0x4E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=1, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Rn=0, Q=1, ac=0, Rd=0, E=0
    let encoding: u32 = 0x4E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_2_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, E=0, Q=0, U=0, ac=0, Rd=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_3_2400_2e402400() {
    // Encoding: 0x2E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=1, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, ac=0, Rn=0, Q=0, Rd=0, E=0, U=1
    let encoding: u32 = 0x2E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_4_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: ac=0, Rm=0, E=0, Rd=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_5_2400_0ec02400() {
    // Encoding: 0x0EC02400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=1, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, Q=0, E=1, Rm=0, ac=0
    let encoding: u32 = 0x0EC02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_6_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, ac=0, Rn=0, Rd=0, U=0, E=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_7_2400_0e412400() {
    // Encoding: 0x0E412400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=1, ac=0, Rn=0, Rd=0
    // Fields: Rm=1, Rn=0, Rd=0, E=0, ac=0, Q=0, U=0
    let encoding: u32 = 0x0E412400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_8_2400_0e5e2400() {
    // Encoding: 0x0E5E2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=30, ac=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, ac=0, E=0, Q=0, Rm=30, U=0
    let encoding: u32 = 0x0E5E2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_9_2400_0e5f2400() {
    // Encoding: 0x0E5F2400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=31, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, ac=0, Rn=0, Rm=31, Q=0, U=0, E=0
    let encoding: u32 = 0x0E5F2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_10_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Q=0, E=0, ac=0, Rn=0, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_11_2400_0e402c00() {
    // Encoding: 0x0E402C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=1, Rn=0, Rd=0
    // Fields: Q=0, ac=1, U=0, Rd=0, Rn=0, E=0, Rm=0
    let encoding: u32 = 0x0E402C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_12_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: U=0, ac=0, Rn=0, Rm=0, Q=0, E=0, Rd=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_13_2400_0e402420() {
    // Encoding: 0x0E402420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=1, Rd=0
    // Fields: Rm=0, E=0, ac=0, Rd=0, Q=0, U=0, Rn=1
    let encoding: u32 = 0x0E402420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_14_2400_0e4027c0() {
    // Encoding: 0x0E4027C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=30, Rd=0
    // Fields: U=0, Rm=0, ac=0, Rd=0, Rn=30, Q=0, E=0
    let encoding: u32 = 0x0E4027C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_15_2400_0e4027e0() {
    // Encoding: 0x0E4027E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=31, Rd=0
    // Fields: E=0, ac=0, Rn=31, Q=0, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x0E4027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_16_2400_0e402400() {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, U=0, Q=0, Rn=0, ac=0, E=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_17_2400_0e402401() {
    // Encoding: 0x0E402401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=1
    // Fields: Q=0, E=0, ac=0, U=0, Rm=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E402401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_18_2400_0e40241e() {
    // Encoding: 0x0E40241E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=30
    // Fields: Q=0, Rd=30, Rm=0, ac=0, Rn=0, U=0, E=0
    let encoding: u32 = 0x0E40241E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_19_2400_0e40241f() {
    // Encoding: 0x0E40241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=0, Rd=31
    // Fields: E=0, ac=0, Rm=0, Rn=0, Q=0, Rd=31, U=0
    let encoding: u32 = 0x0E40241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_20_2400_0e412420() {
    // Encoding: 0x0E412420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=1, ac=0, Rn=1, Rd=0
    // Fields: Rm=1, ac=0, Q=0, Rn=1, Rd=0, E=0, U=0
    let encoding: u32 = 0x0E412420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_21_2400_0e5f27e0() {
    // Encoding: 0x0E5F27E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=31, ac=0, Rn=31, Rd=0
    // Fields: Q=0, E=0, Rm=31, Rn=31, U=0, Rd=0, ac=0
    let encoding: u32 = 0x0E5F27E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_22_2400_0e412401() {
    // Encoding: 0x0E412401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=1, ac=0, Rn=0, Rd=1
    // Fields: ac=0, E=0, U=0, Rn=0, Rm=1, Q=0, Rd=1
    let encoding: u32 = 0x0E412401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_23_2400_0e5f241f() {
    // Encoding: 0x0E5F241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=31, ac=0, Rn=0, Rd=31
    // Fields: U=0, E=0, ac=0, Q=0, Rm=31, Rn=0, Rd=31
    let encoding: u32 = 0x0E5F241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_24_2400_0e402421() {
    // Encoding: 0x0E402421
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=1, Rd=1
    // Fields: Q=0, E=0, Rn=1, U=0, ac=0, Rd=1, Rm=0
    let encoding: u32 = 0x0E402421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_combo_25_2400_0e4027ff() {
    // Encoding: 0x0E4027FF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd field combination: Q=0, U=0, E=0, Rm=0, ac=0, Rn=31, Rd=31
    // Fields: Rd=31, E=0, U=0, Rm=0, Q=0, ac=0, Rn=31
    let encoding: u32 = 0x0E4027FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_special_q_0_size_variant_0_9216_0e402400()
 {
    // Encoding: 0x0E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, E=0, Q=0, Rd=0, U=0, Rm=0, ac=0
    let encoding: u32 = 0x0E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_special_q_1_size_variant_1_9216_4e402400()
 {
    // Encoding: 0x4E402400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd special value Q = 1 (Size variant 1)
    // Fields: E=0, Rm=0, Rn=0, ac=0, Rd=0, Q=1, U=0
    let encoding: u32 = 0x4E402400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_special_rn_31_stack_pointer_sp_may_require_alignment_9216_0e4027e0()
 {
    // Encoding: 0x0E4027E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, ac=0, Rn=31, Rd=0, E=0, U=0, Q=0
    let encoding: u32 = 0x0E4027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_9216_0e40241f()
 {
    // Encoding: 0x0E40241F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, E=0, Rd=31, Rm=0, Q=0, ac=0, Rn=0
    let encoding: u32 = 0x0E40241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_q_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Q = 0 (Min)
    // Fields: ac=0, E=0, U=0, Rn=0, Q=0, sz=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_q_1_max_e400_4e20e400() {
    // Encoding: 0x4E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Q = 1 (Max)
    // Fields: Rd=0, sz=0, Rn=0, ac=0, Q=1, U=0, Rm=0, E=0
    let encoding: u32 = 0x4E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_u_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field U = 0 (Min)
    // Fields: E=0, U=0, Rn=0, sz=0, Rm=0, Q=0, ac=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_u_1_max_e400_2e20e400() {
    // Encoding: 0x2E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field U = 1 (Max)
    // Fields: Rn=0, E=0, U=1, sz=0, Rm=0, Rd=0, Q=0, ac=0
    let encoding: u32 = 0x2E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_e_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field E = 0 (Min)
    // Fields: Rn=0, E=0, Rd=0, U=0, Rm=0, ac=0, Q=0, sz=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field E 23 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_e_1_max_e400_0ea0e400() {
    // Encoding: 0x0EA0E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field E = 1 (Max)
    // Fields: sz=0, Rn=0, Q=0, Rd=0, E=1, U=0, ac=0, Rm=0
    let encoding: u32 = 0x0EA0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_sz_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field sz = 0 (Min)
    // Fields: E=0, Rd=0, ac=0, sz=0, Rn=0, Rm=0, U=0, Q=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_sz_1_max_e400_0e60e400() {
    // Encoding: 0x0E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field sz = 1 (Max)
    // Fields: ac=0, E=0, Rn=0, Rm=0, sz=1, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E60E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rm_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rm = 0 (Min)
    // Fields: Rm=0, Rn=0, Q=0, U=0, sz=0, E=0, ac=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rm_1_poweroftwo_e400_0e21e400() {
    // Encoding: 0x0E21E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, U=0, Q=0, sz=0, E=0, ac=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E21E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rm_30_poweroftwominusone_e400_0e3ee400()
 {
    // Encoding: 0x0E3EE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: E=0, Rd=0, U=0, sz=0, Rn=0, Rm=30, ac=0, Q=0
    let encoding: u32 = 0x0E3EE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rm_31_max_e400_0e3fe400() {
    // Encoding: 0x0E3FE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rm = 31 (Max)
    // Fields: ac=0, Rd=0, Q=0, Rn=0, Rm=31, E=0, sz=0, U=0
    let encoding: u32 = 0x0E3FE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_ac_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field ac = 0 (Min)
    // Fields: ac=0, Rn=0, Rm=0, Q=0, sz=0, E=0, U=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field ac 11 +: 1`
/// Requirement: FieldBoundary { field: "ac", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_ac_1_max_e400_0e20ec00() {
    // Encoding: 0x0E20EC00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field ac = 1 (Max)
    // Fields: Rm=0, E=0, Rn=0, Rd=0, U=0, ac=1, Q=0, sz=0
    let encoding: u32 = 0x0E20EC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rn_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rn = 0 (Min)
    // Fields: ac=0, Rd=0, U=0, E=0, Rm=0, sz=0, Rn=0, Q=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rn_1_poweroftwo_e400_0e20e420() {
    // Encoding: 0x0E20E420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rn = 1 (PowerOfTwo)
    // Fields: ac=0, Rm=0, E=0, U=0, sz=0, Rd=0, Rn=1, Q=0
    let encoding: u32 = 0x0E20E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rn_30_poweroftwominusone_e400_0e20e7c0()
 {
    // Encoding: 0x0E20E7C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, Rm=0, ac=0, Rn=30, Q=0, U=0, E=0, Rd=0
    let encoding: u32 = 0x0E20E7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rn_31_max_e400_0e20e7e0() {
    // Encoding: 0x0E20E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rn = 31 (Max)
    // Fields: Q=0, sz=0, Rd=0, E=0, Rm=0, U=0, ac=0, Rn=31
    let encoding: u32 = 0x0E20E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rd_0_min_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rd = 0 (Min)
    // Fields: U=0, ac=0, E=0, sz=0, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rd_1_poweroftwo_e400_0e20e401() {
    // Encoding: 0x0E20E401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, ac=0, Rn=0, U=0, E=0, Rd=1, sz=0, Q=0
    let encoding: u32 = 0x0E20E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rd_30_poweroftwominusone_e400_0e20e41e()
 {
    // Encoding: 0x0E20E41E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: ac=0, sz=0, Rm=0, Q=0, Rn=0, U=0, E=0, Rd=30
    let encoding: u32 = 0x0E20E41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_field_rd_31_max_e400_0e20e41f() {
    // Encoding: 0x0E20E41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field Rd = 31 (Max)
    // Fields: E=0, U=0, sz=0, Rd=31, Rm=0, Q=0, ac=0, Rn=0
    let encoding: u32 = 0x0E20E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_0_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=0, sz=0, Q=0, Rm=0, Rn=0, U=0, ac=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_1_e400_4e20e400() {
    // Encoding: 0x4E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=1, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, E=0, Q=1, sz=0, ac=0, U=0, Rn=0
    let encoding: u32 = 0x4E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_2_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, Rn=0, Rd=0, ac=0, sz=0, E=0, U=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_3_e400_2e20e400() {
    // Encoding: 0x2E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=1, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rm=0, sz=0, ac=0, U=1, Rd=0, E=0
    let encoding: u32 = 0x2E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_4_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: ac=0, Rm=0, Rd=0, Rn=0, U=0, Q=0, E=0, sz=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// E=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_5_e400_0ea0e400() {
    // Encoding: 0x0EA0E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=1, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, Rm=0, ac=0, E=1, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0EA0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_6_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: ac=0, sz=0, E=0, Rn=0, U=0, Q=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_7_e400_0e60e400() {
    // Encoding: 0x0E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=1, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: U=0, ac=0, Rn=0, Q=0, Rd=0, E=0, Rm=0, sz=1
    let encoding: u32 = 0x0E60E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_8_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: sz=0, E=0, ac=0, Rn=0, U=0, Rm=0, Rd=0, Q=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_9_e400_0e21e400() {
    // Encoding: 0x0E21E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=1, ac=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rm=1, ac=0, Rd=0, Rn=0, E=0, sz=0
    let encoding: u32 = 0x0E21E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_10_e400_0e3ee400() {
    // Encoding: 0x0E3EE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=30, ac=0, Rn=0, Rd=0
    // Fields: E=0, Q=0, sz=0, Rd=0, Rm=30, ac=0, Rn=0, U=0
    let encoding: u32 = 0x0E3EE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_11_e400_0e3fe400() {
    // Encoding: 0x0E3FE400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=31, ac=0, Rn=0, Rd=0
    // Fields: U=0, ac=0, Rm=31, Rn=0, Q=0, sz=0, Rd=0, E=0
    let encoding: u32 = 0x0E3FE400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_12_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, Rn=0, Rm=0, E=0, sz=0, ac=0, Rd=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// ac=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_13_e400_0e20ec00() {
    // Encoding: 0x0E20EC00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=1, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, E=0, Rd=0, ac=1, sz=0, Rn=0, U=0
    let encoding: u32 = 0x0E20EC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_14_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: E=0, ac=0, Rd=0, Rn=0, Q=0, Rm=0, sz=0, U=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_15_e400_0e20e420() {
    // Encoding: 0x0E20E420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=1, Rd=0
    // Fields: Rm=0, U=0, sz=0, Rd=0, E=0, Q=0, Rn=1, ac=0
    let encoding: u32 = 0x0E20E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_16_e400_0e20e7c0() {
    // Encoding: 0x0E20E7C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=30, Rd=0
    // Fields: Q=0, sz=0, ac=0, Rd=0, U=0, Rn=30, Rm=0, E=0
    let encoding: u32 = 0x0E20E7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_17_e400_0e20e7e0() {
    // Encoding: 0x0E20E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=31, Rd=0
    // Fields: Q=0, Rd=0, ac=0, Rm=0, U=0, Rn=31, sz=0, E=0
    let encoding: u32 = 0x0E20E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_18_e400_0e20e400() {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, Q=0, U=0, Rm=0, Rn=0, ac=0, E=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_19_e400_0e20e401() {
    // Encoding: 0x0E20E401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=1
    // Fields: Q=0, Rm=0, ac=0, Rd=1, E=0, sz=0, Rn=0, U=0
    let encoding: u32 = 0x0E20E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_20_e400_0e20e41e() {
    // Encoding: 0x0E20E41E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=30
    // Fields: Rd=30, Rm=0, Rn=0, Q=0, U=0, sz=0, ac=0, E=0
    let encoding: u32 = 0x0E20E41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_21_e400_0e20e41f() {
    // Encoding: 0x0E20E41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=0, Rd=31
    // Fields: Rn=0, E=0, Rd=31, Rm=0, Q=0, U=0, sz=0, ac=0
    let encoding: u32 = 0x0E20E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_22_e400_0e21e420() {
    // Encoding: 0x0E21E420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=1, ac=0, Rn=1, Rd=0
    // Fields: U=0, E=0, Q=0, Rd=0, ac=0, sz=0, Rn=1, Rm=1
    let encoding: u32 = 0x0E21E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_23_e400_0e3fe7e0() {
    // Encoding: 0x0E3FE7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=31, ac=0, Rn=31, Rd=0
    // Fields: U=0, ac=0, E=0, Rn=31, Rd=0, sz=0, Q=0, Rm=31
    let encoding: u32 = 0x0E3FE7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_24_e400_0e21e401() {
    // Encoding: 0x0E21E401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=1, ac=0, Rn=0, Rd=1
    // Fields: E=0, Rn=0, sz=0, U=0, Q=0, Rm=1, ac=0, Rd=1
    let encoding: u32 = 0x0E21E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_25_e400_0e3fe41f() {
    // Encoding: 0x0E3FE41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=31, ac=0, Rn=0, Rd=31
    // Fields: E=0, sz=0, Rm=31, Rd=31, Q=0, U=0, Rn=0, ac=0
    let encoding: u32 = 0x0E3FE41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_26_e400_0e20e421() {
    // Encoding: 0x0E20E421
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=1, Rd=1
    // Fields: E=0, U=0, Rm=0, sz=0, ac=0, Rn=1, Q=0, Rd=1
    let encoding: u32 = 0x0E20E421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_combo_27_e400_0e20e7ff() {
    // Encoding: 0x0E20E7FF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd field combination: Q=0, U=0, E=0, sz=0, Rm=0, ac=0, Rn=31, Rd=31
    // Fields: ac=0, Rm=0, sz=0, Rd=31, U=0, E=0, Q=0, Rn=31
    let encoding: u32 = 0x0E20E7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_special_q_0_size_variant_0_58368_0e60e400()
 {
    // Encoding: 0x0E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, Rn=0, Rd=0, sz=1, ac=0, U=0, E=0, Rm=0
    let encoding: u32 = 0x0E60E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_special_q_1_size_variant_1_58368_4e60e400()
 {
    // Encoding: 0x4E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd special value Q = 1 (Size variant 1)
    // Fields: Rm=0, E=0, U=0, Rn=0, Q=1, ac=0, Rd=0, sz=1
    let encoding: u32 = 0x4E60E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_special_sz_0_size_variant_0_58368_0e20e400()
 {
    // Encoding: 0x0E20E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd special value sz = 0 (Size variant 0)
    // Fields: ac=0, Rm=0, U=0, sz=0, Rn=0, Q=0, Rd=0, E=0
    let encoding: u32 = 0x0E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_special_sz_1_size_variant_1_58368_0e60e400()
 {
    // Encoding: 0x0E60E400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, U=0, sz=1, E=0, Q=0, Rm=0, ac=0
    let encoding: u32 = 0x0E60E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_special_rn_31_stack_pointer_sp_may_require_alignment_58368_0e60e7e0()
 {
    // Encoding: 0x0E60E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, sz=1, Rm=0, E=0, ac=0, U=0, Q=0, Rd=0
    let encoding: u32 = 0x0E60E7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_58368_0e60e41f()
 {
    // Encoding: 0x0E60E41F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, U=0, ac=0, E=0, sz=1, Q=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E60E41F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_reg_write_0_5e402400() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd register write: SimdFromField("d")
    // Encoding: 0x5E402400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E402400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_sp_rn_5e4027e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd with Rn = SP (31)
    // Encoding: 0x5E4027E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E4027E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_zr_rd_5e40241f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd with Rd = ZR (31)
    // Encoding: 0x5E40241F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E40241F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_reg_write_0_5e20e400() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd register write: SimdFromField("d")
    // Encoding: 0x5E20E400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20E400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_sp_rn_5e20e7e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd with Rn = SP (31)
    // Encoding: 0x5E20E7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20E7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_zr_rd_5e20e41f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd with Rd = ZR (31)
    // Encoding: 0x5E20E41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20E41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_reg_write_0_0e402400() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd register write: SimdFromField("d")
    // Encoding: 0x0E402400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E402400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_sp_rn_0e4027e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd with Rn = SP (31)
    // Encoding: 0x0E4027E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E4027E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_zr_rd_0e40241f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd with Rd = ZR (31)
    // Encoding: 0x0E40241F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E40241F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_reg_write_0_0e20e400() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd register write: SimdFromField("d")
    // Encoding: 0x0E20E400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20E400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_sp_rn_0e20e7e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd with Rn = SP (31)
    // Encoding: 0x0E20E7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20E7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_zr_rd_0e20e41f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd with Rd = ZR (31)
    // Encoding: 0x0E20E41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20E41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_u_0_min_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field U = 0 (Min)
    // Fields: Rd=0, Rn=0, size=0, U=0, Rm=0, eq=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_u_1_max_3400_7e203400() {
    // Encoding: 0x7E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field U = 1 (Max)
    // Fields: U=1, eq=0, Rm=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_size_0_min_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field size = 0 (Min)
    // Fields: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_size_1_poweroftwo_3400_5e603400()
 {
    // Encoding: 0x5E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field size = 1 (PowerOfTwo)
    // Fields: Rd=0, U=0, Rm=0, Rn=0, size=1, eq=0
    let encoding: u32 = 0x5E603400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_size_2_poweroftwo_3400_5ea03400()
 {
    // Encoding: 0x5EA03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field size = 2 (PowerOfTwo)
    // Fields: Rm=0, U=0, Rd=0, eq=0, Rn=0, size=2
    let encoding: u32 = 0x5EA03400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_size_3_max_3400_5ee03400() {
    // Encoding: 0x5EE03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field size = 3 (Max)
    // Fields: Rn=0, Rd=0, U=0, size=3, Rm=0, eq=0
    let encoding: u32 = 0x5EE03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rm_0_min_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rm = 0 (Min)
    // Fields: Rm=0, Rd=0, size=0, eq=0, U=0, Rn=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rm_1_poweroftwo_3400_5e213400()
{
    // Encoding: 0x5E213400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, size=0, Rn=0, U=0, Rd=0, eq=0
    let encoding: u32 = 0x5E213400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rm_30_poweroftwominusone_3400_5e3e3400()
 {
    // Encoding: 0x5E3E3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, eq=0, U=0, size=0, Rm=30, Rd=0
    let encoding: u32 = 0x5E3E3400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rm_31_max_3400_5e3f3400() {
    // Encoding: 0x5E3F3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rm = 31 (Max)
    // Fields: size=0, Rm=31, eq=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5E3F3400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field eq 11 +: 1`
/// Requirement: FieldBoundary { field: "eq", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_eq_0_min_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field eq = 0 (Min)
    // Fields: Rm=0, size=0, U=0, Rd=0, Rn=0, eq=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field eq 11 +: 1`
/// Requirement: FieldBoundary { field: "eq", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_eq_1_max_3400_5e203c00() {
    // Encoding: 0x5E203C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field eq = 1 (Max)
    // Fields: size=0, eq=1, Rn=0, U=0, Rd=0, Rm=0
    let encoding: u32 = 0x5E203C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rn_0_min_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rn = 0 (Min)
    // Fields: Rm=0, Rn=0, size=0, U=0, eq=0, Rd=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rn_1_poweroftwo_3400_5e203420()
{
    // Encoding: 0x5E203420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, size=0, Rm=0, eq=0, U=0, Rn=1
    let encoding: u32 = 0x5E203420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rn_30_poweroftwominusone_3400_5e2037c0()
 {
    // Encoding: 0x5E2037C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, size=0, Rm=0, Rn=30, Rd=0, eq=0
    let encoding: u32 = 0x5E2037C0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rn_31_max_3400_5e2037e0() {
    // Encoding: 0x5E2037E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rn = 31 (Max)
    // Fields: size=0, U=0, eq=0, Rm=0, Rn=31, Rd=0
    let encoding: u32 = 0x5E2037E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rd_0_min_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rd = 0 (Min)
    // Fields: Rd=0, Rm=0, eq=0, size=0, U=0, Rn=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rd_1_poweroftwo_3400_5e203401()
{
    // Encoding: 0x5E203401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rd = 1 (PowerOfTwo)
    // Fields: eq=0, Rd=1, U=0, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x5E203401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rd_30_poweroftwominusone_3400_5e20341e()
 {
    // Encoding: 0x5E20341E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rm=0, size=0, eq=0, Rn=0, Rd=30
    let encoding: u32 = 0x5E20341E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_field_rd_31_max_3400_5e20341f() {
    // Encoding: 0x5E20341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field Rd = 31 (Max)
    // Fields: U=0, eq=0, size=0, Rm=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E20341F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_0_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: U=0, Rm=0, size=0, Rd=0, eq=0, Rn=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_1_3400_7e203400() {
    // Encoding: 0x7E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=1, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: eq=0, Rm=0, Rn=0, Rd=0, U=1, size=0
    let encoding: u32 = 0x7E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_2_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, size=0, Rm=0, eq=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_3_3400_5e603400() {
    // Encoding: 0x5E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=1, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, eq=0, Rn=0, Rd=0, size=1
    let encoding: u32 = 0x5E603400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_4_3400_5ea03400() {
    // Encoding: 0x5EA03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=2, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: eq=0, Rd=0, Rn=0, size=2, U=0, Rm=0
    let encoding: u32 = 0x5EA03400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_5_3400_5ee03400() {
    // Encoding: 0x5EE03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=3, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rm=0, size=3, Rn=0, U=0, eq=0, Rd=0
    let encoding: u32 = 0x5EE03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_6_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rm=0, size=0, eq=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_7_3400_5e213400() {
    // Encoding: 0x5E213400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=1, eq=0, Rn=0, Rd=0
    // Fields: Rm=1, Rd=0, U=0, Rn=0, size=0, eq=0
    let encoding: u32 = 0x5E213400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_8_3400_5e3e3400() {
    // Encoding: 0x5E3E3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=30, eq=0, Rn=0, Rd=0
    // Fields: Rm=30, U=0, eq=0, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x5E3E3400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_9_3400_5e3f3400() {
    // Encoding: 0x5E3F3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=31, eq=0, Rn=0, Rd=0
    // Fields: eq=0, Rd=0, size=0, Rm=31, Rn=0, U=0
    let encoding: u32 = 0x5E3F3400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// eq=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_10_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rd=0, Rm=0, Rn=0, eq=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// eq=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_11_3400_5e203c00() {
    // Encoding: 0x5E203C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=1, Rn=0, Rd=0
    // Fields: Rn=0, U=0, eq=1, size=0, Rm=0, Rd=0
    let encoding: u32 = 0x5E203C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_12_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: size=0, Rn=0, U=0, eq=0, Rm=0, Rd=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_13_3400_5e203420() {
    // Encoding: 0x5E203420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=1, Rd=0
    // Fields: Rn=1, Rm=0, size=0, eq=0, U=0, Rd=0
    let encoding: u32 = 0x5E203420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_14_3400_5e2037c0() {
    // Encoding: 0x5E2037C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=30, Rd=0
    // Fields: eq=0, U=0, Rd=0, Rn=30, Rm=0, size=0
    let encoding: u32 = 0x5E2037C0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_15_3400_5e2037e0() {
    // Encoding: 0x5E2037E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=31, Rd=0
    // Fields: U=0, Rm=0, Rn=31, size=0, eq=0, Rd=0
    let encoding: u32 = 0x5E2037E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_16_3400_5e203400() {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, U=0, size=0, eq=0, Rn=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_17_3400_5e203401() {
    // Encoding: 0x5E203401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, eq=0, size=0, U=0, Rm=0
    let encoding: u32 = 0x5E203401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_18_3400_5e20341e() {
    // Encoding: 0x5E20341E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=30
    // Fields: Rm=0, Rd=30, U=0, size=0, Rn=0, eq=0
    let encoding: u32 = 0x5E20341E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_19_3400_5e20341f() {
    // Encoding: 0x5E20341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=0, Rd=31
    // Fields: Rm=0, U=0, eq=0, Rn=0, Rd=31, size=0
    let encoding: u32 = 0x5E20341F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_20_3400_5e213420() {
    // Encoding: 0x5E213420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=1, eq=0, Rn=1, Rd=0
    // Fields: Rd=0, size=0, U=0, Rm=1, eq=0, Rn=1
    let encoding: u32 = 0x5E213420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_21_3400_5e3f37e0() {
    // Encoding: 0x5E3F37E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=31, eq=0, Rn=31, Rd=0
    // Fields: eq=0, Rd=0, size=0, Rm=31, Rn=31, U=0
    let encoding: u32 = 0x5E3F37E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_22_3400_5e213401() {
    // Encoding: 0x5E213401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=1, eq=0, Rn=0, Rd=1
    // Fields: Rm=1, U=0, Rn=0, eq=0, Rd=1, size=0
    let encoding: u32 = 0x5E213401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_23_3400_5e3f341f() {
    // Encoding: 0x5E3F341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=31, eq=0, Rn=0, Rd=31
    // Fields: size=0, U=0, Rm=31, eq=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E3F341F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_24_3400_5e203421() {
    // Encoding: 0x5E203421
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=1, Rd=1
    // Fields: size=0, U=0, eq=0, Rn=1, Rm=0, Rd=1
    let encoding: u32 = 0x5E203421;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_combo_25_3400_5e2037ff() {
    // Encoding: 0x5E2037FF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd field combination: U=0, size=0, Rm=0, eq=0, Rn=31, Rd=31
    // Fields: Rn=31, Rd=31, Rm=0, U=0, eq=0, size=0
    let encoding: u32 = 0x5E2037FF;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_special_size_0_size_variant_0_13312_5e203400()
 {
    // Encoding: 0x5E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rm=0, eq=0, U=0, size=0, Rd=0
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_special_size_1_size_variant_1_13312_5e603400()
 {
    // Encoding: 0x5E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd special value size = 1 (Size variant 1)
    // Fields: Rm=0, eq=0, size=1, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x5E603400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_special_size_2_size_variant_2_13312_5ea03400()
 {
    // Encoding: 0x5EA03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd special value size = 2 (Size variant 2)
    // Fields: eq=0, Rd=0, Rn=0, Rm=0, size=2, U=0
    let encoding: u32 = 0x5EA03400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_special_size_3_size_variant_3_13312_5ee03400()
 {
    // Encoding: 0x5EE03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd special value size = 3 (Size variant 3)
    // Fields: Rm=0, eq=0, Rn=0, Rd=0, U=0, size=3
    let encoding: u32 = 0x5EE03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_13312_5e6037e0()
 {
    // Encoding: 0x5E6037E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, U=0, eq=0, Rn=31, Rd=0, Rm=0
    let encoding: u32 = 0x5E6037E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_13312_5e60341f()
 {
    // Encoding: 0x5E60341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, size=1, U=0, Rn=0, Rd=31, eq=0
    let encoding: u32 = 0x5E60341F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_q_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Q = 0 (Min)
    // Fields: eq=0, Rn=0, Rd=0, Q=0, U=0, size=0, Rm=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_q_1_max_3400_4e203400() {
    // Encoding: 0x4E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Q = 1 (Max)
    // Fields: Rd=0, eq=0, U=0, Q=1, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x4E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_u_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field U = 0 (Min)
    // Fields: Q=0, eq=0, Rn=0, Rd=0, Rm=0, U=0, size=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_u_1_max_3400_2e203400() {
    // Encoding: 0x2E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field U = 1 (Max)
    // Fields: Rd=0, Q=0, eq=0, Rn=0, size=0, U=1, Rm=0
    let encoding: u32 = 0x2E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_size_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field size = 0 (Min)
    // Fields: Rm=0, size=0, Q=0, Rd=0, U=0, eq=0, Rn=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_size_1_poweroftwo_3400_0e603400()
 {
    // Encoding: 0x0E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field size = 1 (PowerOfTwo)
    // Fields: Rm=0, U=0, eq=0, Q=0, size=1, Rn=0, Rd=0
    let encoding: u32 = 0x0E603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_size_2_poweroftwo_3400_0ea03400()
 {
    // Encoding: 0x0EA03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field size = 2 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, eq=0, U=0, size=2, Rn=0, Q=0
    let encoding: u32 = 0x0EA03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_size_3_max_3400_0ee03400() {
    // Encoding: 0x0EE03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field size = 3 (Max)
    // Fields: Q=0, Rn=0, U=0, Rm=0, size=3, eq=0, Rd=0
    let encoding: u32 = 0x0EE03400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rm_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rm = 0 (Min)
    // Fields: Rd=0, size=0, Q=0, U=0, Rm=0, eq=0, Rn=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rm_1_poweroftwo_3400_0e213400()
{
    // Encoding: 0x0E213400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rm = 1 (PowerOfTwo)
    // Fields: U=0, eq=0, Rm=1, size=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E213400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rm_30_poweroftwominusone_3400_0e3e3400()
 {
    // Encoding: 0x0E3E3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: eq=0, U=0, Rn=0, Rd=0, Rm=30, Q=0, size=0
    let encoding: u32 = 0x0E3E3400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rm_31_max_3400_0e3f3400() {
    // Encoding: 0x0E3F3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rm = 31 (Max)
    // Fields: Q=0, U=0, Rd=0, size=0, Rm=31, eq=0, Rn=0
    let encoding: u32 = 0x0E3F3400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field eq 11 +: 1`
/// Requirement: FieldBoundary { field: "eq", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_eq_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field eq = 0 (Min)
    // Fields: Q=0, eq=0, U=0, size=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field eq 11 +: 1`
/// Requirement: FieldBoundary { field: "eq", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_eq_1_max_3400_0e203c00() {
    // Encoding: 0x0E203C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field eq = 1 (Max)
    // Fields: eq=1, Rd=0, Q=0, U=0, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x0E203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rn_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rn = 0 (Min)
    // Fields: U=0, size=0, Rd=0, Q=0, eq=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rn_1_poweroftwo_3400_0e203420()
{
    // Encoding: 0x0E203420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, U=0, eq=0, Rm=0, Q=0, Rd=0, size=0
    let encoding: u32 = 0x0E203420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rn_30_poweroftwominusone_3400_0e2037c0()
 {
    // Encoding: 0x0E2037C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, eq=0, Rn=30, size=0, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0E2037C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rn_31_max_3400_0e2037e0() {
    // Encoding: 0x0E2037E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rn = 31 (Max)
    // Fields: eq=0, U=0, Rn=31, Rm=0, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E2037E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rd_0_min_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rd = 0 (Min)
    // Fields: U=0, size=0, Rm=0, Q=0, Rn=0, Rd=0, eq=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rd_1_poweroftwo_3400_0e203401()
{
    // Encoding: 0x0E203401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, size=0, Rm=0, Rn=0, Q=0, U=0, eq=0
    let encoding: u32 = 0x0E203401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rd_30_poweroftwominusone_3400_0e20341e()
 {
    // Encoding: 0x0E20341E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Q=0, U=0, Rn=0, Rd=30, eq=0, size=0
    let encoding: u32 = 0x0E20341E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_field_rd_31_max_3400_0e20341f() {
    // Encoding: 0x0E20341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field Rd = 31 (Max)
    // Fields: U=0, Q=0, Rm=0, Rn=0, Rd=31, eq=0, size=0
    let encoding: u32 = 0x0E20341F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_0_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rm=0, eq=0, Rn=0, Rd=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_1_3400_4e203400() {
    // Encoding: 0x4E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=1, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rm=0, size=0, eq=0, Rn=0, U=0, Rd=0, Q=1
    let encoding: u32 = 0x4E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_2_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: eq=0, Rn=0, size=0, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_3_3400_2e203400() {
    // Encoding: 0x2E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=1, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, size=0, Rn=0, U=1, eq=0, Rd=0
    let encoding: u32 = 0x2E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_4_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, U=0, size=0, Rd=0, eq=0, Rm=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_5_3400_0e603400() {
    // Encoding: 0x0E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=1, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, eq=0, Rm=0, Rn=0, size=1, Rd=0
    let encoding: u32 = 0x0E603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_6_3400_0ea03400() {
    // Encoding: 0x0EA03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=2, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, eq=0, Rd=0, Rn=0, U=0, size=2
    let encoding: u32 = 0x0EA03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_7_3400_0ee03400() {
    // Encoding: 0x0EE03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=3, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Q=0, size=3, Rm=0, Rn=0, Rd=0, U=0, eq=0
    let encoding: u32 = 0x0EE03400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_8_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: size=0, U=0, Q=0, Rn=0, Rd=0, Rm=0, eq=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_9_3400_0e213400() {
    // Encoding: 0x0E213400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=1, eq=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, eq=0, U=0, Rm=1, size=0, Rd=0
    let encoding: u32 = 0x0E213400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_10_3400_0e3e3400() {
    // Encoding: 0x0E3E3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=30, eq=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, size=0, eq=0, Rm=30, Q=0, Rd=0
    let encoding: u32 = 0x0E3E3400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_11_3400_0e3f3400() {
    // Encoding: 0x0E3F3400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=31, eq=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=31, eq=0, Rn=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x0E3F3400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// eq=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_12_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: U=0, eq=0, Rm=0, Rd=0, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// eq=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_13_3400_0e203c00() {
    // Encoding: 0x0E203C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=1, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rn=0, Q=0, Rm=0, U=0, eq=1
    let encoding: u32 = 0x0E203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_14_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rm=0, Rd=0, Rn=0, eq=0, size=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_15_3400_0e203420() {
    // Encoding: 0x0E203420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=1, Rd=0
    // Fields: Q=0, eq=0, Rn=1, Rm=0, U=0, size=0, Rd=0
    let encoding: u32 = 0x0E203420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_16_3400_0e2037c0() {
    // Encoding: 0x0E2037C0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=30, Rd=0
    // Fields: U=0, Rd=0, eq=0, Rm=0, Q=0, size=0, Rn=30
    let encoding: u32 = 0x0E2037C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_17_3400_0e2037e0() {
    // Encoding: 0x0E2037E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=31, Rd=0
    // Fields: Rn=31, size=0, Rd=0, U=0, Q=0, eq=0, Rm=0
    let encoding: u32 = 0x0E2037E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_18_3400_0e203400() {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Q=0, U=0, Rm=0, eq=0, Rn=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_19_3400_0e203401() {
    // Encoding: 0x0E203401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=1
    // Fields: U=0, Q=0, Rd=1, eq=0, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x0E203401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_20_3400_0e20341e() {
    // Encoding: 0x0E20341E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=30
    // Fields: eq=0, size=0, Q=0, Rd=30, Rn=0, Rm=0, U=0
    let encoding: u32 = 0x0E20341E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_21_3400_0e20341f() {
    // Encoding: 0x0E20341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=0, Rd=31
    // Fields: Rd=31, Q=0, U=0, Rm=0, size=0, eq=0, Rn=0
    let encoding: u32 = 0x0E20341F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_22_3400_0e213420() {
    // Encoding: 0x0E213420
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=1, eq=0, Rn=1, Rd=0
    // Fields: U=0, Rn=1, Rd=0, size=0, Rm=1, Q=0, eq=0
    let encoding: u32 = 0x0E213420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_23_3400_0e3f37e0() {
    // Encoding: 0x0E3F37E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=31, eq=0, Rn=31, Rd=0
    // Fields: U=0, size=0, Rn=31, Rd=0, Q=0, eq=0, Rm=31
    let encoding: u32 = 0x0E3F37E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_24_3400_0e213401() {
    // Encoding: 0x0E213401
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=1, eq=0, Rn=0, Rd=1
    // Fields: U=0, size=0, Rm=1, eq=0, Rd=1, Q=0, Rn=0
    let encoding: u32 = 0x0E213401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_25_3400_0e3f341f() {
    // Encoding: 0x0E3F341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=31, eq=0, Rn=0, Rd=31
    // Fields: size=0, Q=0, Rn=0, U=0, Rm=31, eq=0, Rd=31
    let encoding: u32 = 0x0E3F341F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_26_3400_0e203421() {
    // Encoding: 0x0E203421
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=1, Rd=1
    // Fields: Rd=1, U=0, size=0, eq=0, Q=0, Rn=1, Rm=0
    let encoding: u32 = 0x0E203421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_combo_27_3400_0e2037ff() {
    // Encoding: 0x0E2037FF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd field combination: Q=0, U=0, size=0, Rm=0, eq=0, Rn=31, Rd=31
    // Fields: Rm=0, size=0, U=0, Rd=31, Rn=31, Q=0, eq=0
    let encoding: u32 = 0x0E2037FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_q_0_size_variant_0_13312_0e603400()
 {
    // Encoding: 0x0E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value Q = 0 (Size variant 0)
    // Fields: U=0, eq=0, Q=0, Rd=0, Rn=0, Rm=0, size=1
    let encoding: u32 = 0x0E603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_q_1_size_variant_1_13312_4e603400()
 {
    // Encoding: 0x4E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, size=1, Rd=0, U=0, Rm=0, eq=0, Q=1
    let encoding: u32 = 0x4E603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_size_0_size_variant_0_13312_0e203400()
 {
    // Encoding: 0x0E203400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value size = 0 (Size variant 0)
    // Fields: Q=0, U=0, eq=0, Rd=0, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x0E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_size_1_size_variant_1_13312_0e603400()
 {
    // Encoding: 0x0E603400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value size = 1 (Size variant 1)
    // Fields: Q=0, U=0, size=1, Rm=0, Rn=0, eq=0, Rd=0
    let encoding: u32 = 0x0E603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_size_2_size_variant_2_13312_0ea03400()
 {
    // Encoding: 0x0EA03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value size = 2 (Size variant 2)
    // Fields: Rm=0, U=0, eq=0, size=2, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0EA03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_size_3_size_variant_3_13312_0ee03400()
 {
    // Encoding: 0x0EE03400
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value size = 3 (Size variant 3)
    // Fields: size=3, Q=0, Rn=0, Rd=0, U=0, eq=0, Rm=0
    let encoding: u32 = 0x0EE03400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_rn_31_stack_pointer_sp_may_require_alignment_13312_0e6037e0()
 {
    // Encoding: 0x0E6037E0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, eq=0, Rn=31, Rd=0, Q=0, Rm=0, size=1
    let encoding: u32 = 0x0E6037E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_13312_0e60341f()
 {
    // Encoding: 0x0E60341F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, eq=0, Rn=0, Q=0, size=1, Rm=0, Rd=31
    let encoding: u32 = 0x0E60341F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_reg_write_0_5e203400() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd register write: SimdFromField("d")
    // Encoding: 0x5E203400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E203400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_sp_rn_5e2037e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd with Rn = SP (31)
    // Encoding: 0x5E2037E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E2037E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_zr_rd_5e20341f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd with Rd = ZR (31)
    // Encoding: 0x5E20341F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20341F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_reg_write_0_0e203400() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd register write: SimdFromField("d")
    // Encoding: 0x0E203400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E203400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_sp_rn_0e2037e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd with Rn = SP (31)
    // Encoding: 0x0E2037E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2037E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_zr_rd_0e20341f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_int_simd with Rd = ZR (31)
    // Encoding: 0x0E20341F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20341F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_u_0_min_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field U = 0 (Min)
    // Fields: op=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_u_1_max_c800_7ef8c800() {
    // Encoding: 0x7EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field U = 1 (Max)
    // Fields: U=1, Rd=0, op=0, Rn=0
    let encoding: u32 = 0x7EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_op_0_min_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field op = 0 (Min)
    // Fields: U=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_op_1_max_c800_5ef8d800() {
    // Encoding: 0x5EF8D800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field op = 1 (Max)
    // Fields: Rd=0, Rn=0, U=0, op=1
    let encoding: u32 = 0x5EF8D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rn_0_min_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rn = 0 (Min)
    // Fields: op=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rn_1_poweroftwo_c800_5ef8c820() {
    // Encoding: 0x5EF8C820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rn = 1 (PowerOfTwo)
    // Fields: op=0, Rn=1, Rd=0, U=0
    let encoding: u32 = 0x5EF8C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rn_30_poweroftwominusone_c800_5ef8cbc0()
 {
    // Encoding: 0x5EF8CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, U=0, Rd=0, Rn=30
    let encoding: u32 = 0x5EF8CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rn_31_max_c800_5ef8cbe0() {
    // Encoding: 0x5EF8CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rn = 31 (Max)
    // Fields: U=0, Rn=31, op=0, Rd=0
    let encoding: u32 = 0x5EF8CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rd_0_min_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rd = 0 (Min)
    // Fields: U=0, Rd=0, op=0, Rn=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rd_1_poweroftwo_c800_5ef8c801() {
    // Encoding: 0x5EF8C801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rd = 1 (PowerOfTwo)
    // Fields: U=0, op=0, Rn=0, Rd=1
    let encoding: u32 = 0x5EF8C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rd_30_poweroftwominusone_c800_5ef8c81e()
 {
    // Encoding: 0x5EF8C81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, op=0, Rn=0, Rd=30
    let encoding: u32 = 0x5EF8C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_field_rd_31_max_c800_5ef8c81f() {
    // Encoding: 0x5EF8C81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field Rd = 31 (Max)
    // Fields: op=0, Rd=31, U=0, Rn=0
    let encoding: u32 = 0x5EF8C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_0_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=0
    // Fields: U=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_1_c800_7ef8c800() {
    // Encoding: 0x7EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=1, op=0, Rn=0, Rd=0
    // Fields: U=1, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x7EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_2_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Rn=0, op=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_3_c800_5ef8d800() {
    // Encoding: 0x5EF8D800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=1, Rn=0, Rd=0
    // Fields: op=1, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5EF8D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_4_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, op=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_5_c800_5ef8c820() {
    // Encoding: 0x5EF8C820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=1, Rd=0
    // Fields: U=0, Rn=1, Rd=0, op=0
    let encoding: u32 = 0x5EF8C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_6_c800_5ef8cbc0() {
    // Encoding: 0x5EF8CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=30, Rd=0
    // Fields: U=0, op=0, Rd=0, Rn=30
    let encoding: u32 = 0x5EF8CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_7_c800_5ef8cbe0() {
    // Encoding: 0x5EF8CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=31, Rd=0
    // Fields: Rn=31, op=0, Rd=0, U=0
    let encoding: u32 = 0x5EF8CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_8_c800_5ef8c800() {
    // Encoding: 0x5EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, Rd=0, U=0
    let encoding: u32 = 0x5EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_9_c800_5ef8c801() {
    // Encoding: 0x5EF8C801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=1
    // Fields: op=0, Rn=0, Rd=1, U=0
    let encoding: u32 = 0x5EF8C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_10_c800_5ef8c81e() {
    // Encoding: 0x5EF8C81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=30
    // Fields: Rn=0, U=0, op=0, Rd=30
    let encoding: u32 = 0x5EF8C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_11_c800_5ef8c81f() {
    // Encoding: 0x5EF8C81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=0, Rd=31
    // Fields: op=0, U=0, Rd=31, Rn=0
    let encoding: u32 = 0x5EF8C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_12_c800_5ef8c821() {
    // Encoding: 0x5EF8C821
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=1, Rd=1
    // Fields: op=0, Rn=1, Rd=1, U=0
    let encoding: u32 = 0x5EF8C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_combo_13_c800_5ef8cbff() {
    // Encoding: 0x5EF8CBFF
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd field combination: U=0, op=0, Rn=31, Rd=31
    // Fields: Rn=31, Rd=31, op=0, U=0
    let encoding: u32 = 0x5EF8CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_5ef8cbe0()
 {
    // Encoding: 0x5EF8CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, Rd=0, op=0, Rn=31
    let encoding: u32 = 0x5EF8CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_5ef8c81f()
 {
    // Encoding: 0x5EF8C81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0, U=0, op=0
    let encoding: u32 = 0x5EF8C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_u_0_min_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field U = 0 (Min)
    // Fields: Rn=0, U=0, op=0, Rd=0, sz=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_u_1_max_c800_7ea0c800() {
    // Encoding: 0x7EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field U = 1 (Max)
    // Fields: Rd=0, op=0, sz=0, Rn=0, U=1
    let encoding: u32 = 0x7EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_sz_0_min_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field sz = 0 (Min)
    // Fields: sz=0, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_sz_1_max_c800_5ee0c800() {
    // Encoding: 0x5EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field sz = 1 (Max)
    // Fields: Rn=0, op=0, sz=1, Rd=0, U=0
    let encoding: u32 = 0x5EE0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_op_0_min_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field op = 0 (Min)
    // Fields: Rn=0, Rd=0, op=0, U=0, sz=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_op_1_max_c800_5ea0d800() {
    // Encoding: 0x5EA0D800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field op = 1 (Max)
    // Fields: sz=0, Rd=0, op=1, U=0, Rn=0
    let encoding: u32 = 0x5EA0D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rn_0_min_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0, U=0, sz=0, op=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rn_1_poweroftwo_c800_5ea0c820() {
    // Encoding: 0x5EA0C820
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rn = 1 (PowerOfTwo)
    // Fields: sz=0, U=0, op=0, Rn=1, Rd=0
    let encoding: u32 = 0x5EA0C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rn_30_poweroftwominusone_c800_5ea0cbc0()
 {
    // Encoding: 0x5EA0CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=30, op=0, Rd=0, sz=0
    let encoding: u32 = 0x5EA0CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rn_31_max_c800_5ea0cbe0() {
    // Encoding: 0x5EA0CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rn = 31 (Max)
    // Fields: Rn=31, U=0, op=0, Rd=0, sz=0
    let encoding: u32 = 0x5EA0CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rd_0_min_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rd = 0 (Min)
    // Fields: sz=0, Rd=0, U=0, op=0, Rn=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rd_1_poweroftwo_c800_5ea0c801() {
    // Encoding: 0x5EA0C801
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, sz=0, U=0, Rd=1, op=0
    let encoding: u32 = 0x5EA0C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rd_30_poweroftwominusone_c800_5ea0c81e()
 {
    // Encoding: 0x5EA0C81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, sz=0, Rn=0, Rd=30, op=0
    let encoding: u32 = 0x5EA0C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_field_rd_31_max_c800_5ea0c81f() {
    // Encoding: 0x5EA0C81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field Rd = 31 (Max)
    // Fields: op=0, Rd=31, Rn=0, sz=0, U=0
    let encoding: u32 = 0x5EA0C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_0_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: U=0, op=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_1_c800_7ea0c800() {
    // Encoding: 0x7EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=1, sz=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0, op=0, U=1
    let encoding: u32 = 0x7EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_2_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rn=0, sz=0, op=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_3_c800_5ee0c800() {
    // Encoding: 0x5EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=1, op=0, Rn=0, Rd=0
    // Fields: sz=1, U=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x5EE0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_4_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, sz=0, U=0, Rd=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_5_c800_5ea0d800() {
    // Encoding: 0x5EA0D800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=1, Rn=0, Rd=0
    // Fields: op=1, Rn=0, sz=0, U=0, Rd=0
    let encoding: u32 = 0x5EA0D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_6_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: sz=0, op=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_7_c800_5ea0c820() {
    // Encoding: 0x5EA0C820
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=1, Rd=0
    // Fields: Rd=0, sz=0, op=0, Rn=1, U=0
    let encoding: u32 = 0x5EA0C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_8_c800_5ea0cbc0() {
    // Encoding: 0x5EA0CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=30, Rd=0
    // Fields: Rd=0, op=0, U=0, Rn=30, sz=0
    let encoding: u32 = 0x5EA0CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_9_c800_5ea0cbe0() {
    // Encoding: 0x5EA0CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=31, Rd=0
    // Fields: Rd=0, U=0, sz=0, Rn=31, op=0
    let encoding: u32 = 0x5EA0CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_10_c800_5ea0c800() {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: sz=0, Rd=0, op=0, Rn=0, U=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_11_c800_5ea0c801() {
    // Encoding: 0x5EA0C801
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=1
    // Fields: Rn=0, U=0, sz=0, op=0, Rd=1
    let encoding: u32 = 0x5EA0C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_12_c800_5ea0c81e() {
    // Encoding: 0x5EA0C81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=30
    // Fields: op=0, sz=0, U=0, Rn=0, Rd=30
    let encoding: u32 = 0x5EA0C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_13_c800_5ea0c81f() {
    // Encoding: 0x5EA0C81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=0, Rd=31
    // Fields: op=0, Rn=0, Rd=31, U=0, sz=0
    let encoding: u32 = 0x5EA0C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_14_c800_5ea0c821() {
    // Encoding: 0x5EA0C821
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=1, Rd=1
    // Fields: U=0, op=0, Rn=1, Rd=1, sz=0
    let encoding: u32 = 0x5EA0C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_combo_15_c800_5ea0cbff() {
    // Encoding: 0x5EA0CBFF
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd field combination: U=0, sz=0, op=0, Rn=31, Rd=31
    // Fields: sz=0, U=0, Rn=31, Rd=31, op=0
    let encoding: u32 = 0x5EA0CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_special_sz_0_size_variant_0_51200_5ea0c800()
 {
    // Encoding: 0x5EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd special value sz = 0 (Size variant 0)
    // Fields: Rd=0, op=0, sz=0, U=0, Rn=0
    let encoding: u32 = 0x5EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_special_sz_1_size_variant_1_51200_5ee0c800()
 {
    // Encoding: 0x5EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd special value sz = 1 (Size variant 1)
    // Fields: sz=1, op=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EE0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_5ee0cbe0()
 {
    // Encoding: 0x5EE0CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, Rd=0, Rn=31, U=0, op=0
    let encoding: u32 = 0x5EE0CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_5ee0c81f()
 {
    // Encoding: 0x5EE0C81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, Rd=31, op=0, sz=1, Rn=0
    let encoding: u32 = 0x5EE0C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_q_0_min_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Q = 0 (Min)
    // Fields: Rd=0, op=0, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_q_1_max_c800_4ef8c800() {
    // Encoding: 0x4EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Q = 1 (Max)
    // Fields: Rd=0, Rn=0, Q=1, op=0, U=0
    let encoding: u32 = 0x4EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_u_0_min_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field U = 0 (Min)
    // Fields: Q=0, Rd=0, U=0, op=0, Rn=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_u_1_max_c800_2ef8c800() {
    // Encoding: 0x2EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field U = 1 (Max)
    // Fields: Q=0, op=0, U=1, Rn=0, Rd=0
    let encoding: u32 = 0x2EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_op_0_min_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field op = 0 (Min)
    // Fields: Q=0, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_op_1_max_c800_0ef8d800() {
    // Encoding: 0x0EF8D800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field op = 1 (Max)
    // Fields: Rd=0, Q=0, op=1, U=0, Rn=0
    let encoding: u32 = 0x0EF8D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rn_0_min_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rn = 0 (Min)
    // Fields: U=0, op=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rn_1_poweroftwo_c800_0ef8c820() {
    // Encoding: 0x0EF8C820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Q=0, Rd=0, U=0, op=0
    let encoding: u32 = 0x0EF8C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rn_30_poweroftwominusone_c800_0ef8cbc0()
 {
    // Encoding: 0x0EF8CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, Rd=0, U=0, Rn=30, Q=0
    let encoding: u32 = 0x0EF8CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rn_31_max_c800_0ef8cbe0() {
    // Encoding: 0x0EF8CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rn = 31 (Max)
    // Fields: op=0, U=0, Q=0, Rn=31, Rd=0
    let encoding: u32 = 0x0EF8CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rd_0_min_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rd = 0 (Min)
    // Fields: op=0, Rn=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rd_1_poweroftwo_c800_0ef8c801() {
    // Encoding: 0x0EF8C801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Q=0, Rn=0, U=0, op=0
    let encoding: u32 = 0x0EF8C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rd_30_poweroftwominusone_c800_0ef8c81e()
 {
    // Encoding: 0x0EF8C81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rd=30, U=0, op=0, Rn=0
    let encoding: u32 = 0x0EF8C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_field_rd_31_max_c800_0ef8c81f() {
    // Encoding: 0x0EF8C81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field Rd = 31 (Max)
    // Fields: Rn=0, op=0, Q=0, U=0, Rd=31
    let encoding: u32 = 0x0EF8C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_0_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_1_c800_4ef8c800() {
    // Encoding: 0x4EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=1, U=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, Q=1, op=0
    let encoding: u32 = 0x4EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_2_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=0
    // Fields: U=0, op=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_3_c800_2ef8c800() {
    // Encoding: 0x2EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=1, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, Q=0, U=1, Rd=0
    let encoding: u32 = 0x2EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_4_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, op=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_5_c800_0ef8d800() {
    // Encoding: 0x0EF8D800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=1, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rd=0, U=0, op=1
    let encoding: u32 = 0x0EF8D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_6_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Q=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_7_c800_0ef8c820() {
    // Encoding: 0x0EF8C820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=1, Rd=0
    // Fields: U=0, op=0, Rd=0, Rn=1, Q=0
    let encoding: u32 = 0x0EF8C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_8_c800_0ef8cbc0() {
    // Encoding: 0x0EF8CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=30, Rd=0
    // Fields: Q=0, op=0, Rd=0, U=0, Rn=30
    let encoding: u32 = 0x0EF8CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_9_c800_0ef8cbe0() {
    // Encoding: 0x0EF8CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=31, Rd=0
    // Fields: U=0, op=0, Q=0, Rd=0, Rn=31
    let encoding: u32 = 0x0EF8CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_10_c800_0ef8c800() {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, op=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_11_c800_0ef8c801() {
    // Encoding: 0x0EF8C801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=1
    // Fields: Rn=0, Q=0, op=0, Rd=1, U=0
    let encoding: u32 = 0x0EF8C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_12_c800_0ef8c81e() {
    // Encoding: 0x0EF8C81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, U=0, op=0, Q=0
    let encoding: u32 = 0x0EF8C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_13_c800_0ef8c81f() {
    // Encoding: 0x0EF8C81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=0, Rd=31
    // Fields: U=0, op=0, Rn=0, Q=0, Rd=31
    let encoding: u32 = 0x0EF8C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_14_c800_0ef8c821() {
    // Encoding: 0x0EF8C821
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=1, Rd=1
    // Fields: op=0, Rn=1, Q=0, Rd=1, U=0
    let encoding: u32 = 0x0EF8C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_combo_15_c800_0ef8cbff() {
    // Encoding: 0x0EF8CBFF
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd field combination: Q=0, U=0, op=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, Q=0, U=0, op=0
    let encoding: u32 = 0x0EF8CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_special_q_0_size_variant_0_51200_0ef8c800()
 {
    // Encoding: 0x0EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, Q=0, Rd=0, op=0, U=0
    let encoding: u32 = 0x0EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_special_q_1_size_variant_1_51200_4ef8c800()
 {
    // Encoding: 0x4EF8C800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, Q=1, Rd=0, op=0, U=0
    let encoding: u32 = 0x4EF8C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_0ef8cbe0()
 {
    // Encoding: 0x0EF8CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rn=31, Rd=0, U=0, op=0
    let encoding: u32 = 0x0EF8CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_0ef8c81f()
 {
    // Encoding: 0x0EF8C81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, U=0, op=0, Rd=31, Q=0
    let encoding: u32 = 0x0EF8C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_q_0_min_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Q = 0 (Min)
    // Fields: sz=0, Q=0, Rn=0, U=0, op=0, Rd=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_q_1_max_c800_4ea0c800() {
    // Encoding: 0x4EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Q = 1 (Max)
    // Fields: Rd=0, op=0, Q=1, sz=0, Rn=0, U=0
    let encoding: u32 = 0x4EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_u_0_min_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field U = 0 (Min)
    // Fields: Rn=0, sz=0, U=0, op=0, Q=0, Rd=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_u_1_max_c800_2ea0c800() {
    // Encoding: 0x2EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field U = 1 (Max)
    // Fields: Rn=0, U=1, sz=0, op=0, Q=0, Rd=0
    let encoding: u32 = 0x2EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_sz_0_min_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field sz = 0 (Min)
    // Fields: Rd=0, op=0, U=0, Q=0, Rn=0, sz=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_sz_1_max_c800_0ee0c800() {
    // Encoding: 0x0EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field sz = 1 (Max)
    // Fields: sz=1, Rn=0, op=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0EE0C800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_op_0_min_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field op = 0 (Min)
    // Fields: U=0, op=0, sz=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_op_1_max_c800_0ea0d800() {
    // Encoding: 0x0EA0D800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field op = 1 (Max)
    // Fields: U=0, Q=0, sz=0, op=1, Rn=0, Rd=0
    let encoding: u32 = 0x0EA0D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rn_0_min_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rn = 0 (Min)
    // Fields: Rd=0, op=0, Rn=0, U=0, Q=0, sz=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rn_1_poweroftwo_c800_0ea0c820() {
    // Encoding: 0x0EA0C820
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rn = 1 (PowerOfTwo)
    // Fields: sz=0, Q=0, Rd=0, Rn=1, U=0, op=0
    let encoding: u32 = 0x0EA0C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rn_30_poweroftwominusone_c800_0ea0cbc0()
 {
    // Encoding: 0x0EA0CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, U=0, op=0, Rd=0, Rn=30, sz=0
    let encoding: u32 = 0x0EA0CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rn_31_max_c800_0ea0cbe0() {
    // Encoding: 0x0EA0CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rn = 31 (Max)
    // Fields: Rd=0, sz=0, U=0, op=0, Q=0, Rn=31
    let encoding: u32 = 0x0EA0CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rd_0_min_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rd = 0 (Min)
    // Fields: op=0, Rn=0, U=0, Rd=0, sz=0, Q=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rd_1_poweroftwo_c800_0ea0c801() {
    // Encoding: 0x0EA0C801
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Q=0, U=0, Rn=0, sz=0, op=0
    let encoding: u32 = 0x0EA0C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rd_30_poweroftwominusone_c800_0ea0c81e()
 {
    // Encoding: 0x0EA0C81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Q=0, U=0, op=0, Rn=0, sz=0
    let encoding: u32 = 0x0EA0C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_field_rd_31_max_c800_0ea0c81f() {
    // Encoding: 0x0EA0C81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field Rd = 31 (Max)
    // Fields: Rn=0, sz=0, Q=0, op=0, U=0, Rd=31
    let encoding: u32 = 0x0EA0C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_0_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: U=0, sz=0, Q=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_1_c800_4ea0c800() {
    // Encoding: 0x4EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=1, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, sz=0, op=0, Rd=0, Q=1
    let encoding: u32 = 0x4EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_2_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0, Q=0, U=0, op=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_3_c800_2ea0c800() {
    // Encoding: 0x2EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=1, sz=0, op=0, Rn=0, Rd=0
    // Fields: op=0, U=1, sz=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_4_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: sz=0, op=0, Rn=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_5_c800_0ee0c800() {
    // Encoding: 0x0EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=1, op=0, Rn=0, Rd=0
    // Fields: Q=0, sz=1, Rd=0, op=0, U=0, Rn=0
    let encoding: u32 = 0x0EE0C800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_6_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, Rd=0, Q=0, U=0, op=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_7_c800_0ea0d800() {
    // Encoding: 0x0EA0D800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=1, Rn=0, Rd=0
    // Fields: op=1, Rd=0, Q=0, Rn=0, sz=0, U=0
    let encoding: u32 = 0x0EA0D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_8_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, op=0, sz=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_9_c800_0ea0c820() {
    // Encoding: 0x0EA0C820
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=1, Rd=0
    // Fields: sz=0, Rd=0, op=0, U=0, Q=0, Rn=1
    let encoding: u32 = 0x0EA0C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_10_c800_0ea0cbc0() {
    // Encoding: 0x0EA0CBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=30, Rd=0
    // Fields: op=0, sz=0, U=0, Rd=0, Q=0, Rn=30
    let encoding: u32 = 0x0EA0CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_11_c800_0ea0cbe0() {
    // Encoding: 0x0EA0CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=31, Rd=0
    // Fields: Q=0, Rn=31, op=0, Rd=0, sz=0, U=0
    let encoding: u32 = 0x0EA0CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_12_c800_0ea0c800() {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=0
    // Fields: sz=0, U=0, op=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_13_c800_0ea0c801() {
    // Encoding: 0x0EA0C801
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=1
    // Fields: Rn=0, sz=0, Rd=1, Q=0, op=0, U=0
    let encoding: u32 = 0x0EA0C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_14_c800_0ea0c81e() {
    // Encoding: 0x0EA0C81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=30
    // Fields: Rn=0, U=0, Rd=30, op=0, Q=0, sz=0
    let encoding: u32 = 0x0EA0C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_15_c800_0ea0c81f() {
    // Encoding: 0x0EA0C81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=0, Rd=31
    // Fields: Rn=0, U=0, Rd=31, Q=0, op=0, sz=0
    let encoding: u32 = 0x0EA0C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_16_c800_0ea0c821() {
    // Encoding: 0x0EA0C821
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=1, Rd=1
    // Fields: U=0, Rd=1, sz=0, Q=0, op=0, Rn=1
    let encoding: u32 = 0x0EA0C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_combo_17_c800_0ea0cbff() {
    // Encoding: 0x0EA0CBFF
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd field combination: Q=0, U=0, sz=0, op=0, Rn=31, Rd=31
    // Fields: op=0, Q=0, U=0, Rn=31, Rd=31, sz=0
    let encoding: u32 = 0x0EA0CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_special_q_0_size_variant_0_51200_0ee0c800()
 {
    // Encoding: 0x0EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, U=0, sz=1, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x0EE0C800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_special_q_1_size_variant_1_51200_4ee0c800()
 {
    // Encoding: 0x4EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd special value Q = 1 (Size variant 1)
    // Fields: Rd=0, sz=1, op=0, U=0, Q=1, Rn=0
    let encoding: u32 = 0x4EE0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_special_sz_0_size_variant_0_51200_0ea0c800()
 {
    // Encoding: 0x0EA0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd special value sz = 0 (Size variant 0)
    // Fields: Q=0, U=0, Rn=0, sz=0, Rd=0, op=0
    let encoding: u32 = 0x0EA0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_special_sz_1_size_variant_1_51200_0ee0c800()
 {
    // Encoding: 0x0EE0C800
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, sz=1, U=0, Rd=0, Q=0, op=0
    let encoding: u32 = 0x0EE0C800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_0ee0cbe0()
 {
    // Encoding: 0x0EE0CBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, Rn=31, U=0, Rd=0, Q=0, op=0
    let encoding: u32 = 0x0EE0CBE0;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_0ee0c81f()
 {
    // Encoding: 0x0EE0C81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: op=0, Q=0, Rn=0, U=0, Rd=31, sz=1
    let encoding: u32 = 0x0EE0C81F;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_reg_write_0_5ef8c800() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd register write: SimdFromField("d")
    // Encoding: 0x5EF8C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EF8C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_sp_rn_5ef8cbe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd with Rn = SP (31)
    // Encoding: 0x5EF8CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EF8CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_zr_rd_5ef8c81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd with Rd = ZR (31)
    // Encoding: 0x5EF8C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EF8C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_reg_write_0_5ea0c800() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd register write: SimdFromField("d")
    // Encoding: 0x5EA0C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EA0C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_sp_rn_5ea0cbe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd with Rn = SP (31)
    // Encoding: 0x5EA0CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EA0CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_zr_rd_5ea0c81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd with Rd = ZR (31)
    // Encoding: 0x5EA0C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EA0C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_reg_write_0_0ef8c800() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd register write: SimdFromField("d")
    // Encoding: 0x0EF8C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EF8C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_sp_rn_0ef8cbe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd with Rn = SP (31)
    // Encoding: 0x0EF8CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EF8CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_zr_rd_0ef8c81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd with Rd = ZR (31)
    // Encoding: 0x0EF8C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EF8C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_reg_write_0_0ea0c800() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd register write: SimdFromField("d")
    // Encoding: 0x0EA0C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_sp_rn_0ea0cbe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd with Rn = SP (31)
    // Encoding: 0x0EA0CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_zr_rd_0ea0c81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_bulk_simd with Rd = ZR (31)
    // Encoding: 0x0EA0C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_size_0_min_a800_5e20a800() {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field size = 0 (Min)
    // Fields: size=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_size_1_poweroftwo_a800_5e60a800()
 {
    // Encoding: 0x5E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field size = 1 (PowerOfTwo)
    // Fields: size=1, Rn=0, Rd=0
    let encoding: u32 = 0x5E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_size_2_poweroftwo_a800_5ea0a800()
 {
    // Encoding: 0x5EA0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field size = 2 (PowerOfTwo)
    // Fields: size=2, Rn=0, Rd=0
    let encoding: u32 = 0x5EA0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_size_3_max_a800_5ee0a800() {
    // Encoding: 0x5EE0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field size = 3 (Max)
    // Fields: size=3, Rn=0, Rd=0
    let encoding: u32 = 0x5EE0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rn_0_min_a800_5e20a800() {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, size=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rn_1_poweroftwo_a800_5e20a820()
{
    // Encoding: 0x5E20A820
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, size=0
    let encoding: u32 = 0x5E20A820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rn_30_poweroftwominusone_a800_5e20abc0()
 {
    // Encoding: 0x5E20ABC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rd=0, Rn=30
    let encoding: u32 = 0x5E20ABC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rn_31_max_a800_5e20abe0() {
    // Encoding: 0x5E20ABE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rn = 31 (Max)
    // Fields: size=0, Rd=0, Rn=31
    let encoding: u32 = 0x5E20ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rd_0_min_a800_5e20a800() {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rd = 0 (Min)
    // Fields: Rn=0, size=0, Rd=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rd_1_poweroftwo_a800_5e20a801()
{
    // Encoding: 0x5E20A801
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1, size=0
    let encoding: u32 = 0x5E20A801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rd_30_poweroftwominusone_a800_5e20a81e()
 {
    // Encoding: 0x5E20A81E
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rd=30, Rn=0
    let encoding: u32 = 0x5E20A81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_field_rd_31_max_a800_5e20a81f() {
    // Encoding: 0x5E20A81F
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field Rd = 31 (Max)
    // Fields: Rd=31, Rn=0, size=0
    let encoding: u32 = 0x5E20A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_0_a800_5e20a800() {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, size=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_1_a800_5e60a800() {
    // Encoding: 0x5E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=1, Rn=0, Rd=0
    // Fields: size=1, Rd=0, Rn=0
    let encoding: u32 = 0x5E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_2_a800_5ea0a800() {
    // Encoding: 0x5EA0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=2, Rn=0, Rd=0
    // Fields: Rn=0, size=2, Rd=0
    let encoding: u32 = 0x5EA0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_3_a800_5ee0a800() {
    // Encoding: 0x5EE0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=3, Rn=0, Rd=0
    // Fields: Rd=0, size=3, Rn=0
    let encoding: u32 = 0x5EE0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_4_a800_5e20a800() {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_5_a800_5e20a820() {
    // Encoding: 0x5E20A820
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=1, Rd=0
    // Fields: Rd=0, size=0, Rn=1
    let encoding: u32 = 0x5E20A820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_6_a800_5e20abc0() {
    // Encoding: 0x5E20ABC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=30, Rd=0
    // Fields: size=0, Rn=30, Rd=0
    let encoding: u32 = 0x5E20ABC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_7_a800_5e20abe0() {
    // Encoding: 0x5E20ABE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=31, Rd=0
    // Fields: size=0, Rn=31, Rd=0
    let encoding: u32 = 0x5E20ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_8_a800_5e20a800() {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, Rd=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_9_a800_5e20a801() {
    // Encoding: 0x5E20A801
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=0, Rd=1
    // Fields: size=0, Rn=0, Rd=1
    let encoding: u32 = 0x5E20A801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_10_a800_5e20a81e() {
    // Encoding: 0x5E20A81E
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=0, Rd=30
    // Fields: Rd=30, Rn=0, size=0
    let encoding: u32 = 0x5E20A81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_11_a800_5e20a81f() {
    // Encoding: 0x5E20A81F
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=0, Rd=31
    // Fields: size=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E20A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_12_a800_5e20a821() {
    // Encoding: 0x5E20A821
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=1, Rd=1
    // Fields: size=0, Rd=1, Rn=1
    let encoding: u32 = 0x5E20A821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_combo_13_a800_5e20abff() {
    // Encoding: 0x5E20ABFF
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd field combination: size=0, Rn=31, Rd=31
    // Fields: size=0, Rd=31, Rn=31
    let encoding: u32 = 0x5E20ABFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_special_size_0_size_variant_0_43008_5e20a800()
 {
    // Encoding: 0x5E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd special value size = 0 (Size variant 0)
    // Fields: Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_special_size_1_size_variant_1_43008_5e60a800()
 {
    // Encoding: 0x5E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd special value size = 1 (Size variant 1)
    // Fields: Rn=0, size=1, Rd=0
    let encoding: u32 = 0x5E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_special_size_2_size_variant_2_43008_5ea0a800()
 {
    // Encoding: 0x5EA0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd special value size = 2 (Size variant 2)
    // Fields: size=2, Rd=0, Rn=0
    let encoding: u32 = 0x5EA0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_special_size_3_size_variant_3_43008_5ee0a800()
 {
    // Encoding: 0x5EE0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd special value size = 3 (Size variant 3)
    // Fields: Rd=0, Rn=0, size=3
    let encoding: u32 = 0x5EE0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_43008_5e60abe0()
 {
    // Encoding: 0x5E60ABE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rn=31, Rd=0
    let encoding: u32 = 0x5E60ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_43008_5e60a81f()
 {
    // Encoding: 0x5E60A81F
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rd=31, Rn=0
    let encoding: u32 = 0x5E60A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_q_0_min_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Q = 0 (Min)
    // Fields: Rn=0, Q=0, size=0, Rd=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_q_1_max_a800_4e20a800() {
    // Encoding: 0x4E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Q = 1 (Max)
    // Fields: Q=1, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x4E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_size_0_min_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field size = 0 (Min)
    // Fields: Q=0, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_size_1_poweroftwo_a800_0e60a800()
 {
    // Encoding: 0x0E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field size = 1 (PowerOfTwo)
    // Fields: Q=0, Rn=0, size=1, Rd=0
    let encoding: u32 = 0x0E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_size_2_poweroftwo_a800_0ea0a800()
 {
    // Encoding: 0x0EA0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field size = 2 (PowerOfTwo)
    // Fields: Q=0, Rd=0, Rn=0, size=2
    let encoding: u32 = 0x0EA0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_size_3_max_a800_0ee0a800() {
    // Encoding: 0x0EE0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field size = 3 (Max)
    // Fields: Q=0, Rd=0, size=3, Rn=0
    let encoding: u32 = 0x0EE0A800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rn_0_min_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rn = 0 (Min)
    // Fields: Rd=0, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rn_1_poweroftwo_a800_0e20a820()
{
    // Encoding: 0x0E20A820
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, Rd=0, Rn=1, size=0
    let encoding: u32 = 0x0E20A820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rn_30_poweroftwominusone_a800_0e20abc0()
 {
    // Encoding: 0x0E20ABC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, Q=0, size=0
    let encoding: u32 = 0x0E20ABC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rn_31_max_a800_0e20abe0() {
    // Encoding: 0x0E20ABE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rn = 31 (Max)
    // Fields: Q=0, Rd=0, Rn=31, size=0
    let encoding: u32 = 0x0E20ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rd_0_min_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rd = 0 (Min)
    // Fields: Rn=0, size=0, Q=0, Rd=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rd_1_poweroftwo_a800_0e20a801()
{
    // Encoding: 0x0E20A801
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E20A801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rd_30_poweroftwominusone_a800_0e20a81e()
 {
    // Encoding: 0x0E20A81E
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, Q=0, size=0
    let encoding: u32 = 0x0E20A81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_field_rd_31_max_a800_0e20a81f() {
    // Encoding: 0x0E20A81F
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field Rd = 31 (Max)
    // Fields: size=0, Q=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E20A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_0_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_1_a800_4e20a800() {
    // Encoding: 0x4E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=1, size=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Q=1, Rn=0
    let encoding: u32 = 0x4E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_2_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_3_a800_0e60a800() {
    // Encoding: 0x0E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=1, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, size=1, Rd=0
    let encoding: u32 = 0x0E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_4_a800_0ea0a800() {
    // Encoding: 0x0EA0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=2, Rn=0, Rd=0
    // Fields: size=2, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0EA0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_5_a800_0ee0a800() {
    // Encoding: 0x0EE0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=3, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, size=3
    let encoding: u32 = 0x0EE0A800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_6_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_7_a800_0e20a820() {
    // Encoding: 0x0E20A820
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=1, Rd=0
    // Fields: Rn=1, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E20A820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_8_a800_0e20abc0() {
    // Encoding: 0x0E20ABC0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=30, Rd=0
    // Fields: size=0, Rn=30, Rd=0, Q=0
    let encoding: u32 = 0x0E20ABC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_9_a800_0e20abe0() {
    // Encoding: 0x0E20ABE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=31, Rd=0
    // Fields: size=0, Rd=0, Rn=31, Q=0
    let encoding: u32 = 0x0E20ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_10_a800_0e20a800() {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, size=0, Rd=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_11_a800_0e20a801() {
    // Encoding: 0x0E20A801
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=1
    // Fields: Rn=0, size=0, Rd=1, Q=0
    let encoding: u32 = 0x0E20A801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_12_a800_0e20a81e() {
    // Encoding: 0x0E20A81E
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=30
    // Fields: size=0, Q=0, Rd=30, Rn=0
    let encoding: u32 = 0x0E20A81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_13_a800_0e20a81f() {
    // Encoding: 0x0E20A81F
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=0, Rd=31
    // Fields: Rd=31, Rn=0, Q=0, size=0
    let encoding: u32 = 0x0E20A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_14_a800_0e20a821() {
    // Encoding: 0x0E20A821
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=1, Rd=1
    // Fields: Rd=1, Q=0, Rn=1, size=0
    let encoding: u32 = 0x0E20A821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_combo_15_a800_0e20abff() {
    // Encoding: 0x0E20ABFF
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd field combination: Q=0, size=0, Rn=31, Rd=31
    // Fields: Rn=31, size=0, Rd=31, Q=0
    let encoding: u32 = 0x0E20ABFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_q_0_size_variant_0_43008_0e60a800()
 {
    // Encoding: 0x0E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value Q = 0 (Size variant 0)
    // Fields: Rd=0, size=1, Rn=0, Q=0
    let encoding: u32 = 0x0E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_q_1_size_variant_1_43008_4e60a800()
 {
    // Encoding: 0x4E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value Q = 1 (Size variant 1)
    // Fields: size=1, Rd=0, Rn=0, Q=1
    let encoding: u32 = 0x4E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_size_0_size_variant_0_43008_0e20a800()
 {
    // Encoding: 0x0E20A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value size = 0 (Size variant 0)
    // Fields: Rn=0, size=0, Q=0, Rd=0
    let encoding: u32 = 0x0E20A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_size_1_size_variant_1_43008_0e60a800()
 {
    // Encoding: 0x0E60A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value size = 1 (Size variant 1)
    // Fields: size=1, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0E60A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_size_2_size_variant_2_43008_0ea0a800()
 {
    // Encoding: 0x0EA0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value size = 2 (Size variant 2)
    // Fields: Rd=0, size=2, Rn=0, Q=0
    let encoding: u32 = 0x0EA0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_size_3_size_variant_3_43008_0ee0a800()
 {
    // Encoding: 0x0EE0A800
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value size = 3 (Size variant 3)
    // Fields: Rd=0, Rn=0, Q=0, size=3
    let encoding: u32 = 0x0EE0A800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_rn_31_stack_pointer_sp_may_require_alignment_43008_0e60abe0()
 {
    // Encoding: 0x0E60ABE0
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rn=31, Rd=0, size=1
    let encoding: u32 = 0x0E60ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_43008_0e60a81f()
 {
    // Encoding: 0x0E60A81F
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, size=1, Q=0, Rd=31
    let encoding: u32 = 0x0E60A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_reg_write_0_5e20a800() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd register write: SimdFromField("d")
    // Encoding: 0x5E20A800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20A800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_sp_rn_5e20abe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd with Rn = SP (31)
    // Encoding: 0x5E20ABE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20ABE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_zr_rd_5e20a81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd with Rd = ZR (31)
    // Encoding: 0x5E20A81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20A81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_reg_write_0_0e20a800() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd register write: SimdFromField("d")
    // Encoding: 0x0E20A800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20A800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_sp_rn_0e20abe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd with Rn = SP (31)
    // Encoding: 0x0E20ABE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20ABE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_zr_rd_0e20a81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd with Rd = ZR (31)
    // Encoding: 0x0E20A81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20A81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rn_0_min_e800_5ef8e800() {
    // Encoding: 0x5EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0
    let encoding: u32 = 0x5EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rn_1_poweroftwo_e800_5ef8e820()
{
    // Encoding: 0x5EF8E820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0
    let encoding: u32 = 0x5EF8E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rn_30_poweroftwominusone_e800_5ef8ebc0()
 {
    // Encoding: 0x5EF8EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0
    let encoding: u32 = 0x5EF8EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rn_31_max_e800_5ef8ebe0() {
    // Encoding: 0x5EF8EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31
    let encoding: u32 = 0x5EF8EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rd_0_min_e800_5ef8e800() {
    // Encoding: 0x5EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rd = 0 (Min)
    // Fields: Rn=0, Rd=0
    let encoding: u32 = 0x5EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rd_1_poweroftwo_e800_5ef8e801()
{
    // Encoding: 0x5EF8E801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1
    let encoding: u32 = 0x5EF8E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rd_30_poweroftwominusone_e800_5ef8e81e()
 {
    // Encoding: 0x5EF8E81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30
    let encoding: u32 = 0x5EF8E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_field_rd_31_max_e800_5ef8e81f() {
    // Encoding: 0x5EF8E81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31
    let encoding: u32 = 0x5EF8E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_0_e800_5ef8e800() {
    // Encoding: 0x5EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=0, Rd=0
    // Fields: Rn=0, Rd=0
    let encoding: u32 = 0x5EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_1_e800_5ef8e820() {
    // Encoding: 0x5EF8E820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=1, Rd=0
    // Fields: Rn=1, Rd=0
    let encoding: u32 = 0x5EF8E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_2_e800_5ef8ebc0() {
    // Encoding: 0x5EF8EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=30, Rd=0
    // Fields: Rn=30, Rd=0
    let encoding: u32 = 0x5EF8EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_3_e800_5ef8ebe0() {
    // Encoding: 0x5EF8EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=31, Rd=0
    // Fields: Rn=31, Rd=0
    let encoding: u32 = 0x5EF8EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_4_e800_5ef8e800() {
    // Encoding: 0x5EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=0, Rd=0
    // Fields: Rd=0, Rn=0
    let encoding: u32 = 0x5EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_5_e800_5ef8e801() {
    // Encoding: 0x5EF8E801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=0, Rd=1
    // Fields: Rd=1, Rn=0
    let encoding: u32 = 0x5EF8E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_6_e800_5ef8e81e() {
    // Encoding: 0x5EF8E81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=0, Rd=30
    // Fields: Rn=0, Rd=30
    let encoding: u32 = 0x5EF8E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_7_e800_5ef8e81f() {
    // Encoding: 0x5EF8E81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=0, Rd=31
    // Fields: Rn=0, Rd=31
    let encoding: u32 = 0x5EF8E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_8_e800_5ef8e821() {
    // Encoding: 0x5EF8E821
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=1, Rd=1
    // Fields: Rn=1, Rd=1
    let encoding: u32 = 0x5EF8E821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_combo_9_e800_5ef8ebff() {
    // Encoding: 0x5EF8EBFF
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd field combination: Rn=31, Rd=31
    // Fields: Rn=31, Rd=31
    let encoding: u32 = 0x5EF8EBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_59392_5ef8ebe0()
 {
    // Encoding: 0x5EF8EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rd=0
    let encoding: u32 = 0x5EF8EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_59392_5ef8e81f()
 {
    // Encoding: 0x5EF8E81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0
    let encoding: u32 = 0x5EF8E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_sz_0_min_e800_5ea0e800() {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field sz = 0 (Min)
    // Fields: sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_sz_1_max_e800_5ee0e800() {
    // Encoding: 0x5EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field sz = 1 (Max)
    // Fields: sz=1, Rd=0, Rn=0
    let encoding: u32 = 0x5EE0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rn_0_min_e800_5ea0e800() {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rn = 0 (Min)
    // Fields: sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rn_1_poweroftwo_e800_5ea0e820()
 {
    // Encoding: 0x5EA0E820
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, sz=0, Rd=0
    let encoding: u32 = 0x5EA0E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rn_30_poweroftwominusone_e800_5ea0ebc0()
 {
    // Encoding: 0x5EA0EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, sz=0
    let encoding: u32 = 0x5EA0EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rn_31_max_e800_5ea0ebe0() {
    // Encoding: 0x5EA0EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rn = 31 (Max)
    // Fields: sz=0, Rn=31, Rd=0
    let encoding: u32 = 0x5EA0EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rd_0_min_e800_5ea0e800() {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rd = 0 (Min)
    // Fields: sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rd_1_poweroftwo_e800_5ea0e801()
 {
    // Encoding: 0x5EA0E801
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, sz=0, Rn=0
    let encoding: u32 = 0x5EA0E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rd_30_poweroftwominusone_e800_5ea0e81e()
 {
    // Encoding: 0x5EA0E81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, sz=0, Rn=0
    let encoding: u32 = 0x5EA0E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_field_rd_31_max_e800_5ea0e81f() {
    // Encoding: 0x5EA0E81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field Rd = 31 (Max)
    // Fields: sz=0, Rn=0, Rd=31
    let encoding: u32 = 0x5EA0E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_0_e800_5ea0e800() {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_1_e800_5ee0e800() {
    // Encoding: 0x5EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=1, Rn=0, Rd=0
    // Fields: Rd=0, sz=1, Rn=0
    let encoding: u32 = 0x5EE0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_2_e800_5ea0e800() {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_3_e800_5ea0e820() {
    // Encoding: 0x5EA0E820
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=1, Rd=0
    // Fields: sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x5EA0E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_4_e800_5ea0ebc0() {
    // Encoding: 0x5EA0EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=30, Rd=0
    // Fields: sz=0, Rn=30, Rd=0
    let encoding: u32 = 0x5EA0EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_5_e800_5ea0ebe0() {
    // Encoding: 0x5EA0EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, sz=0
    let encoding: u32 = 0x5EA0EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_6_e800_5ea0e800() {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_7_e800_5ea0e801() {
    // Encoding: 0x5EA0E801
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, sz=0
    let encoding: u32 = 0x5EA0E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_8_e800_5ea0e81e() {
    // Encoding: 0x5EA0E81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=0, Rd=30
    // Fields: sz=0, Rd=30, Rn=0
    let encoding: u32 = 0x5EA0E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_9_e800_5ea0e81f() {
    // Encoding: 0x5EA0E81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, sz=0
    let encoding: u32 = 0x5EA0E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_10_e800_5ea0e821() {
    // Encoding: 0x5EA0E821
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=1, Rd=1
    // Fields: Rd=1, Rn=1, sz=0
    let encoding: u32 = 0x5EA0E821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_combo_11_e800_5ea0ebff() {
    // Encoding: 0x5EA0EBFF
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd field combination: sz=0, Rn=31, Rd=31
    // Fields: Rd=31, sz=0, Rn=31
    let encoding: u32 = 0x5EA0EBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_special_sz_0_size_variant_0_59392_5ea0e800()
 {
    // Encoding: 0x5EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd special value sz = 0 (Size variant 0)
    // Fields: Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x5EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_special_sz_1_size_variant_1_59392_5ee0e800()
 {
    // Encoding: 0x5EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, sz=1
    let encoding: u32 = 0x5EE0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_59392_5ee0ebe0()
 {
    // Encoding: 0x5EE0EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rd=0, sz=1
    let encoding: u32 = 0x5EE0EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_59392_5ee0e81f()
 {
    // Encoding: 0x5EE0E81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sz=1, Rd=31, Rn=0
    let encoding: u32 = 0x5EE0E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_q_0_min_e800_0ef8e800() {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Q = 0 (Min)
    // Fields: Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_q_1_max_e800_4ef8e800() {
    // Encoding: 0x4EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Q = 1 (Max)
    // Fields: Rd=0, Rn=0, Q=1
    let encoding: u32 = 0x4EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rn_0_min_e800_0ef8e800() {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rn = 0 (Min)
    // Fields: Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rn_1_poweroftwo_e800_0ef8e820()
{
    // Encoding: 0x0EF8E820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Q=0, Rd=0
    let encoding: u32 = 0x0EF8E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rn_30_poweroftwominusone_e800_0ef8ebc0()
 {
    // Encoding: 0x0EF8EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Q=0, Rn=30
    let encoding: u32 = 0x0EF8EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rn_31_max_e800_0ef8ebe0() {
    // Encoding: 0x0EF8EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rn = 31 (Max)
    // Fields: Q=0, Rn=31, Rd=0
    let encoding: u32 = 0x0EF8EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rd_0_min_e800_0ef8e800() {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rd = 0 (Min)
    // Fields: Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rd_1_poweroftwo_e800_0ef8e801()
{
    // Encoding: 0x0EF8E801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, Rd=1, Rn=0
    let encoding: u32 = 0x0EF8E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rd_30_poweroftwominusone_e800_0ef8e81e()
 {
    // Encoding: 0x0EF8E81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=0, Rd=30
    let encoding: u32 = 0x0EF8E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_field_rd_31_max_e800_0ef8e81f() {
    // Encoding: 0x0EF8E81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field Rd = 31 (Max)
    // Fields: Q=0, Rn=0, Rd=31
    let encoding: u32 = 0x0EF8E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_0_e800_0ef8e800() {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_1_e800_4ef8e800() {
    // Encoding: 0x4EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=1, Rn=0, Rd=0
    // Fields: Q=1, Rn=0, Rd=0
    let encoding: u32 = 0x4EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_2_e800_0ef8e800() {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_3_e800_0ef8e820() {
    // Encoding: 0x0EF8E820
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=1, Rd=0
    // Fields: Rd=0, Q=0, Rn=1
    let encoding: u32 = 0x0EF8E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_4_e800_0ef8ebc0() {
    // Encoding: 0x0EF8EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, Q=0
    let encoding: u32 = 0x0EF8EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_5_e800_0ef8ebe0() {
    // Encoding: 0x0EF8EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=31, Rd=0
    // Fields: Rd=0, Q=0, Rn=31
    let encoding: u32 = 0x0EF8EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_6_e800_0ef8e800() {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_7_e800_0ef8e801() {
    // Encoding: 0x0EF8E801
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=0, Rd=1
    // Fields: Rd=1, Q=0, Rn=0
    let encoding: u32 = 0x0EF8E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_8_e800_0ef8e81e() {
    // Encoding: 0x0EF8E81E
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=0, Rd=30
    // Fields: Q=0, Rn=0, Rd=30
    let encoding: u32 = 0x0EF8E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_9_e800_0ef8e81f() {
    // Encoding: 0x0EF8E81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=0, Rd=31
    // Fields: Q=0, Rn=0, Rd=31
    let encoding: u32 = 0x0EF8E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_10_e800_0ef8e821() {
    // Encoding: 0x0EF8E821
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=1, Rd=1
    // Fields: Rd=1, Q=0, Rn=1
    let encoding: u32 = 0x0EF8E821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_combo_11_e800_0ef8ebff() {
    // Encoding: 0x0EF8EBFF
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd field combination: Q=0, Rn=31, Rd=31
    // Fields: Rn=31, Q=0, Rd=31
    let encoding: u32 = 0x0EF8EBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_special_q_0_size_variant_0_59392_0ef8e800()
 {
    // Encoding: 0x0EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_special_q_1_size_variant_1_59392_4ef8e800()
 {
    // Encoding: 0x4EF8E800
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, Q=1
    let encoding: u32 = 0x4EF8E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_special_rn_31_stack_pointer_sp_may_require_alignment_59392_0ef8ebe0()
 {
    // Encoding: 0x0EF8EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rn=31, Rd=0
    let encoding: u32 = 0x0EF8EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_59392_0ef8e81f()
 {
    // Encoding: 0x0EF8E81F
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Q=0, Rd=31
    let encoding: u32 = 0x0EF8E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_q_0_min_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Q = 0 (Min)
    // Fields: Q=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_q_1_max_e800_4ea0e800() {
    // Encoding: 0x4EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Q = 1 (Max)
    // Fields: Rn=0, Rd=0, Q=1, sz=0
    let encoding: u32 = 0x4EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_sz_0_min_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field sz = 0 (Min)
    // Fields: Q=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_sz_1_max_e800_0ee0e800() {
    // Encoding: 0x0EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field sz = 1 (Max)
    // Fields: Rd=0, Rn=0, sz=1, Q=0
    let encoding: u32 = 0x0EE0E800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rn_0_min_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rn = 0 (Min)
    // Fields: sz=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rn_1_poweroftwo_e800_0ea0e820()
 {
    // Encoding: 0x0EA0E820
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, sz=0, Rd=0, Q=0
    let encoding: u32 = 0x0EA0E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rn_30_poweroftwominusone_e800_0ea0ebc0()
 {
    // Encoding: 0x0EA0EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, Q=0, Rn=30, Rd=0
    let encoding: u32 = 0x0EA0EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rn_31_max_e800_0ea0ebe0() {
    // Encoding: 0x0EA0EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, sz=0, Q=0
    let encoding: u32 = 0x0EA0EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rd_0_min_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rd = 0 (Min)
    // Fields: sz=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rd_1_poweroftwo_e800_0ea0e801()
 {
    // Encoding: 0x0EA0E801
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, Rd=1, Rn=0, sz=0
    let encoding: u32 = 0x0EA0E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rd_30_poweroftwominusone_e800_0ea0e81e()
 {
    // Encoding: 0x0EA0E81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, Q=0, sz=0
    let encoding: u32 = 0x0EA0E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_field_rd_31_max_e800_0ea0e81f() {
    // Encoding: 0x0EA0E81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field Rd = 31 (Max)
    // Fields: Rn=0, sz=0, Q=0, Rd=31
    let encoding: u32 = 0x0EA0E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_0_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_1_e800_4ea0e800() {
    // Encoding: 0x4EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=1, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, Rn=0, Q=1
    let encoding: u32 = 0x4EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_2_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=0
    // Fields: Q=0, sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_3_e800_0ee0e800() {
    // Encoding: 0x0EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=1, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0, sz=1
    let encoding: u32 = 0x0EE0E800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_4_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, Q=0, Rn=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_5_e800_0ea0e820() {
    // Encoding: 0x0EA0E820
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=1, Rd=0
    // Fields: sz=0, Rd=0, Q=0, Rn=1
    let encoding: u32 = 0x0EA0E820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_6_e800_0ea0ebc0() {
    // Encoding: 0x0EA0EBC0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=30, Rd=0
    // Fields: sz=0, Rn=30, Q=0, Rd=0
    let encoding: u32 = 0x0EA0EBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_7_e800_0ea0ebe0() {
    // Encoding: 0x0EA0EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=31, Rd=0
    // Fields: sz=0, Rn=31, Q=0, Rd=0
    let encoding: u32 = 0x0EA0EBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_8_e800_0ea0e800() {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, sz=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_9_e800_0ea0e801() {
    // Encoding: 0x0EA0E801
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=1
    // Fields: sz=0, Q=0, Rd=1, Rn=0
    let encoding: u32 = 0x0EA0E801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_10_e800_0ea0e81e() {
    // Encoding: 0x0EA0E81E
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=30
    // Fields: Q=0, Rd=30, Rn=0, sz=0
    let encoding: u32 = 0x0EA0E81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_11_e800_0ea0e81f() {
    // Encoding: 0x0EA0E81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=0, Rd=31
    // Fields: sz=0, Rd=31, Rn=0, Q=0
    let encoding: u32 = 0x0EA0E81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_12_e800_0ea0e821() {
    // Encoding: 0x0EA0E821
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, sz=0, Q=0
    let encoding: u32 = 0x0EA0E821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_combo_13_e800_0ea0ebff() {
    // Encoding: 0x0EA0EBFF
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd field combination: Q=0, sz=0, Rn=31, Rd=31
    // Fields: sz=0, Rd=31, Q=0, Rn=31
    let encoding: u32 = 0x0EA0EBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_special_q_0_size_variant_0_59392_0ee0e800()
 {
    // Encoding: 0x0EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, Rn=0, sz=1, Rd=0
    let encoding: u32 = 0x0EE0E800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_special_q_1_size_variant_1_59392_4ee0e800()
 {
    // Encoding: 0x4EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, Q=1, Rd=0, sz=1
    let encoding: u32 = 0x4EE0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_special_sz_0_size_variant_0_59392_0ea0e800()
 {
    // Encoding: 0x0EA0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd special value sz = 0 (Size variant 0)
    // Fields: Rn=0, sz=0, Rd=0, Q=0
    let encoding: u32 = 0x0EA0E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_special_sz_1_size_variant_1_59392_0ee0e800()
 {
    // Encoding: 0x0EE0E800
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd special value sz = 1 (Size variant 1)
    // Fields: sz=1, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0EE0E800;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_special_rn_31_stack_pointer_sp_may_require_alignment_59392_0ee0ebe0()
 {
    // Encoding: 0x0EE0EBE0
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, sz=1, Q=0, Rd=0
    let encoding: u32 = 0x0EE0EBE0;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_59392_0ee0e81f()
 {
    // Encoding: 0x0EE0E81F
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, Q=0, sz=1
    let encoding: u32 = 0x0EE0E81F;
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

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_reg_write_0_5ef8e800() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd register write: SimdFromField("d")
    // Encoding: 0x5EF8E800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EF8E800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_sp_rn_5ef8ebe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd with Rn = SP (31)
    // Encoding: 0x5EF8EBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EF8EBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_zr_rd_5ef8e81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd with Rd = ZR (31)
    // Encoding: 0x5EF8E81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EF8E81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_reg_write_0_5ea0e800() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd register write: SimdFromField("d")
    // Encoding: 0x5EA0E800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EA0E800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_sp_rn_5ea0ebe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd with Rn = SP (31)
    // Encoding: 0x5EA0EBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EA0EBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_zr_rd_5ea0e81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd with Rd = ZR (31)
    // Encoding: 0x5EA0E81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5EA0E81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_reg_write_0_0ef8e800() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd register write: SimdFromField("d")
    // Encoding: 0x0EF8E800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EF8E800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_sp_rn_0ef8ebe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd with Rn = SP (31)
    // Encoding: 0x0EF8EBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EF8EBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_zr_rd_0ef8e81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd with Rd = ZR (31)
    // Encoding: 0x0EF8E81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EF8E81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_reg_write_0_0ea0e800() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd register write: SimdFromField("d")
    // Encoding: 0x0EA0E800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0E800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_sp_rn_0ea0ebe0() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd with Rn = SP (31)
    // Encoding: 0x0EA0EBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0EBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_zr_rd_0ea0e81f() {
    // Test aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd with Rd = ZR (31)
    // Encoding: 0x0EA0E81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0E81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_u_0_min_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field U = 0 (Min)
    // Fields: Rm=0, Rn=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_u_1_max_8c00_7e208c00() {
    // Encoding: 0x7E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field U = 1 (Max)
    // Fields: Rd=0, size=0, Rm=0, Rn=0, U=1
    let encoding: u32 = 0x7E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_size_0_min_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field size = 0 (Min)
    // Fields: U=0, Rn=0, size=0, Rd=0, Rm=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_size_1_poweroftwo_8c00_5e608c00()
 {
    // Encoding: 0x5E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field size = 1 (PowerOfTwo)
    // Fields: U=0, Rd=0, size=1, Rn=0, Rm=0
    let encoding: u32 = 0x5E608C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_size_2_poweroftwo_8c00_5ea08c00()
 {
    // Encoding: 0x5EA08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field size = 2 (PowerOfTwo)
    // Fields: Rm=0, U=0, Rn=0, size=2, Rd=0
    let encoding: u32 = 0x5EA08C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_size_3_max_8c00_5ee08c00() {
    // Encoding: 0x5EE08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field size = 3 (Max)
    // Fields: size=3, Rm=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5EE08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rm_0_min_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rm = 0 (Min)
    // Fields: U=0, Rn=0, Rd=0, size=0, Rm=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rm_1_poweroftwo_8c00_5e218c00()
 {
    // Encoding: 0x5E218C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rm = 1 (PowerOfTwo)
    // Fields: U=0, Rd=0, Rm=1, size=0, Rn=0
    let encoding: u32 = 0x5E218C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rm_30_poweroftwominusone_8c00_5e3e8c00()
 {
    // Encoding: 0x5E3E8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, U=0, Rm=30, size=0, Rn=0
    let encoding: u32 = 0x5E3E8C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rm_31_max_8c00_5e3f8c00() {
    // Encoding: 0x5E3F8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rm = 31 (Max)
    // Fields: U=0, size=0, Rm=31, Rn=0, Rd=0
    let encoding: u32 = 0x5E3F8C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rn_0_min_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rn = 0 (Min)
    // Fields: Rd=0, Rm=0, U=0, Rn=0, size=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rn_1_poweroftwo_8c00_5e208c20()
 {
    // Encoding: 0x5E208C20
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=1, U=0, size=0, Rd=0
    let encoding: u32 = 0x5E208C20;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rn_30_poweroftwominusone_8c00_5e208fc0()
 {
    // Encoding: 0x5E208FC0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, size=0, Rd=0, U=0, Rn=30
    let encoding: u32 = 0x5E208FC0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rn_31_max_8c00_5e208fe0() {
    // Encoding: 0x5E208FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rn = 31 (Max)
    // Fields: U=0, size=0, Rn=31, Rd=0, Rm=0
    let encoding: u32 = 0x5E208FE0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rd_0_min_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rd = 0 (Min)
    // Fields: U=0, Rd=0, size=0, Rn=0, Rm=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rd_1_poweroftwo_8c00_5e208c01()
 {
    // Encoding: 0x5E208C01
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, size=0, U=0, Rd=1
    let encoding: u32 = 0x5E208C01;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rd_30_poweroftwominusone_8c00_5e208c1e()
 {
    // Encoding: 0x5E208C1E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rm=0, U=0, size=0, Rd=30
    let encoding: u32 = 0x5E208C1E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_field_rd_31_max_8c00_5e208c1f() {
    // Encoding: 0x5E208C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field Rd = 31 (Max)
    // Fields: size=0, Rm=0, Rn=0, U=0, Rd=31
    let encoding: u32 = 0x5E208C1F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_0_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, U=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_1_8c00_7e208c00() {
    // Encoding: 0x7E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, size=0, U=1, Rd=0
    let encoding: u32 = 0x7E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_2_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_3_8c00_5e608c00() {
    // Encoding: 0x5E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x5E608C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_4_8c00_5ea08c00() {
    // Encoding: 0x5EA08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, size=2, U=0, Rn=0
    let encoding: u32 = 0x5EA08C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_5_8c00_5ee08c00() {
    // Encoding: 0x5EE08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x5EE08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_6_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_7_8c00_5e218c00() {
    // Encoding: 0x5E218C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rn=0, size=0, U=0, Rm=1, Rd=0
    let encoding: u32 = 0x5E218C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_8_8c00_5e3e8c00() {
    // Encoding: 0x5E3E8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rm=30, size=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5E3E8C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_9_8c00_5e3f8c00() {
    // Encoding: 0x5E3F8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rm=31, Rd=0, size=0
    let encoding: u32 = 0x5E3F8C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_10_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, size=0, U=0, Rn=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_11_8c00_5e208c20() {
    // Encoding: 0x5E208C20
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: size=0, Rn=1, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x5E208C20;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_12_8c00_5e208fc0() {
    // Encoding: 0x5E208FC0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: size=0, Rm=0, U=0, Rn=30, Rd=0
    let encoding: u32 = 0x5E208FC0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_13_8c00_5e208fe0() {
    // Encoding: 0x5E208FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: Rd=0, Rn=31, U=0, size=0, Rm=0
    let encoding: u32 = 0x5E208FE0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_14_8c00_5e208c00() {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rm=0, U=0, Rd=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_15_8c00_5e208c01() {
    // Encoding: 0x5E208C01
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rm=0, size=0, U=0, Rn=0, Rd=1
    let encoding: u32 = 0x5E208C01;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_16_8c00_5e208c1e() {
    // Encoding: 0x5E208C1E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: size=0, Rn=0, Rd=30, Rm=0, U=0
    let encoding: u32 = 0x5E208C1E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_17_8c00_5e208c1f() {
    // Encoding: 0x5E208C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: U=0, Rn=0, Rm=0, Rd=31, size=0
    let encoding: u32 = 0x5E208C1F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_18_8c00_5e218c20() {
    // Encoding: 0x5E218C20
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rd=0, size=0, Rn=1, U=0, Rm=1
    let encoding: u32 = 0x5E218C20;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_19_8c00_5e3f8fe0() {
    // Encoding: 0x5E3F8FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rm=31, U=0, Rn=31, Rd=0, size=0
    let encoding: u32 = 0x5E3F8FE0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_20_8c00_5e218c01() {
    // Encoding: 0x5E218C01
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Rd=1, Rn=0, size=0, U=0, Rm=1
    let encoding: u32 = 0x5E218C01;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_21_8c00_5e3f8c1f() {
    // Encoding: 0x5E3F8C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: U=0, Rd=31, size=0, Rm=31, Rn=0
    let encoding: u32 = 0x5E3F8C1F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_22_8c00_5e208c21() {
    // Encoding: 0x5E208C21
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: size=0, U=0, Rn=1, Rd=1, Rm=0
    let encoding: u32 = 0x5E208C21;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_combo_23_8c00_5e208fff() {
    // Encoding: 0x5E208FFF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Rd=31, U=0, Rm=0, Rn=31, size=0
    let encoding: u32 = 0x5E208FFF;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_special_size_0_size_variant_0_35840_5e208c00()
 {
    // Encoding: 0x5E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd special value size = 0 (Size variant 0)
    // Fields: Rd=0, size=0, Rm=0, U=0, Rn=0
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_special_size_1_size_variant_1_35840_5e608c00()
 {
    // Encoding: 0x5E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd special value size = 1 (Size variant 1)
    // Fields: Rm=0, size=1, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x5E608C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_special_size_2_size_variant_2_35840_5ea08c00()
 {
    // Encoding: 0x5EA08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd special value size = 2 (Size variant 2)
    // Fields: Rd=0, size=2, Rm=0, Rn=0, U=0
    let encoding: u32 = 0x5EA08C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_special_size_3_size_variant_3_35840_5ee08c00()
 {
    // Encoding: 0x5EE08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd special value size = 3 (Size variant 3)
    // Fields: U=0, size=3, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x5EE08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_35840_5e608fe0()
 {
    // Encoding: 0x5E608FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, Rn=31, Rm=0, size=1, U=0
    let encoding: u32 = 0x5E608FE0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_35840_5e608c1f()
 {
    // Encoding: 0x5E608C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rn=0, Rd=31, U=0, Rm=0
    let encoding: u32 = 0x5E608C1F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_q_0_min_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Q = 0 (Min)
    // Fields: U=0, Rm=0, size=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_q_1_max_8c00_4e208c00() {
    // Encoding: 0x4E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Q = 1 (Max)
    // Fields: Q=1, size=0, Rd=0, Rn=0, Rm=0, U=0
    let encoding: u32 = 0x4E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_u_0_min_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field U = 0 (Min)
    // Fields: Rm=0, Rn=0, size=0, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_u_1_max_8c00_2e208c00() {
    // Encoding: 0x2E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field U = 1 (Max)
    // Fields: size=0, Q=0, U=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x2E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_size_0_min_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field size = 0 (Min)
    // Fields: Rn=0, U=0, Q=0, Rd=0, size=0, Rm=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_size_1_poweroftwo_8c00_0e608c00()
 {
    // Encoding: 0x0E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field size = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, size=1, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_size_2_poweroftwo_8c00_0ea08c00()
 {
    // Encoding: 0x0EA08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field size = 2 (PowerOfTwo)
    // Fields: Rm=0, U=0, size=2, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EA08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_size_3_max_8c00_0ee08c00() {
    // Encoding: 0x0EE08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field size = 3 (Max)
    // Fields: Q=0, Rn=0, Rd=0, Rm=0, U=0, size=3
    let encoding: u32 = 0x0EE08C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rm_0_min_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rm = 0 (Min)
    // Fields: U=0, Rd=0, size=0, Rn=0, Q=0, Rm=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rm_1_poweroftwo_8c00_0e218c00()
 {
    // Encoding: 0x0E218C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rm = 1 (PowerOfTwo)
    // Fields: Rd=0, Q=0, size=0, Rm=1, Rn=0, U=0
    let encoding: u32 = 0x0E218C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rm_30_poweroftwominusone_8c00_0e3e8c00()
 {
    // Encoding: 0x0E3E8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, size=0, Q=0, Rm=30, U=0
    let encoding: u32 = 0x0E3E8C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rm_31_max_8c00_0e3f8c00() {
    // Encoding: 0x0E3F8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rm = 31 (Max)
    // Fields: Rm=31, Rn=0, U=0, Q=0, Rd=0, size=0
    let encoding: u32 = 0x0E3F8C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rn_0_min_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rn = 0 (Min)
    // Fields: Rm=0, size=0, Rd=0, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rn_1_poweroftwo_8c00_0e208c20()
 {
    // Encoding: 0x0E208C20
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, Rn=1, Rm=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x0E208C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rn_30_poweroftwominusone_8c00_0e208fc0()
 {
    // Encoding: 0x0E208FC0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=30, Rm=0, Rd=0, size=0, Q=0
    let encoding: u32 = 0x0E208FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rn_31_max_8c00_0e208fe0() {
    // Encoding: 0x0E208FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rn = 31 (Max)
    // Fields: Rd=0, Rm=0, size=0, Q=0, Rn=31, U=0
    let encoding: u32 = 0x0E208FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rd_0_min_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rd = 0 (Min)
    // Fields: size=0, U=0, Rn=0, Rm=0, Q=0, Rd=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rd_1_poweroftwo_8c00_0e208c01()
 {
    // Encoding: 0x0E208C01
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, size=0, Q=0, Rm=0, Rd=1
    let encoding: u32 = 0x0E208C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rd_30_poweroftwominusone_8c00_0e208c1e()
 {
    // Encoding: 0x0E208C1E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rm=0, size=0, Rn=0, Rd=30, U=0
    let encoding: u32 = 0x0E208C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_field_rd_31_max_8c00_0e208c1f() {
    // Encoding: 0x0E208C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field Rd = 31 (Max)
    // Fields: Q=0, U=0, Rn=0, size=0, Rd=31, Rm=0
    let encoding: u32 = 0x0E208C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_0_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, size=0, Q=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_1_8c00_4e208c00() {
    // Encoding: 0x4E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=1, U=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x4E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_2_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Q=0, Rm=0, size=0, Rd=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_3_8c00_2e208c00() {
    // Encoding: 0x2E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=1, Rm=0, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x2E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_4_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, size=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_5_8c00_0e608c00() {
    // Encoding: 0x0E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: size=1, U=0, Q=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_6_8c00_0ea08c00() {
    // Encoding: 0x0EA08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, U=0, Rd=0, Rn=0, size=2
    let encoding: u32 = 0x0EA08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_7_8c00_0ee08c00() {
    // Encoding: 0x0EE08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, U=0, Rm=0, size=3, Rn=0
    let encoding: u32 = 0x0EE08C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_8_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_9_8c00_0e218c00() {
    // Encoding: 0x0E218C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rn=0, size=0, U=0, Rd=0, Q=0, Rm=1
    let encoding: u32 = 0x0E218C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_10_8c00_0e3e8c00() {
    // Encoding: 0x0E3E8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rd=0, Rm=30, Q=0, U=0
    let encoding: u32 = 0x0E3E8C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_11_8c00_0e3f8c00() {
    // Encoding: 0x0E3F8C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rm=31, size=0, Rn=0, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E3F8C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_12_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, U=0, Rm=0, size=0, Rn=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_13_8c00_0e208c20() {
    // Encoding: 0x0E208C20
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: size=0, Rm=0, Rn=1, U=0, Rd=0, Q=0
    let encoding: u32 = 0x0E208C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_14_8c00_0e208fc0() {
    // Encoding: 0x0E208FC0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Q=0, U=0, Rm=0, Rd=0, Rn=30, size=0
    let encoding: u32 = 0x0E208FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_15_8c00_0e208fe0() {
    // Encoding: 0x0E208FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: U=0, Q=0, size=0, Rm=0, Rd=0, Rn=31
    let encoding: u32 = 0x0E208FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_16_8c00_0e208c00() {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rm=0, Rd=0, Rn=0, size=0, Q=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_17_8c00_0e208c01() {
    // Encoding: 0x0E208C01
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rm=0, U=0, Rn=0, Rd=1, Q=0, size=0
    let encoding: u32 = 0x0E208C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_18_8c00_0e208c1e() {
    // Encoding: 0x0E208C1E
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: U=0, Q=0, Rn=0, size=0, Rm=0, Rd=30
    let encoding: u32 = 0x0E208C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_19_8c00_0e208c1f() {
    // Encoding: 0x0E208C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E208C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_20_8c00_0e218c20() {
    // Encoding: 0x0E218C20
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rn=1, Rm=1, Rd=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E218C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_21_8c00_0e3f8fe0() {
    // Encoding: 0x0E3F8FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: size=0, Rm=31, Rn=31, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0E3F8FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_22_8c00_0e218c01() {
    // Encoding: 0x0E218C01
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Q=0, size=0, U=0, Rm=1, Rn=0, Rd=1
    let encoding: u32 = 0x0E218C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_23_8c00_0e3f8c1f() {
    // Encoding: 0x0E3F8C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Rn=0, Rm=31, U=0, Rd=31, Q=0, size=0
    let encoding: u32 = 0x0E3F8C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_24_8c00_0e208c21() {
    // Encoding: 0x0E208C21
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: size=0, Q=0, U=0, Rd=1, Rm=0, Rn=1
    let encoding: u32 = 0x0E208C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_combo_25_8c00_0e208fff() {
    // Encoding: 0x0E208FFF
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: U=0, size=0, Rm=0, Rn=31, Q=0, Rd=31
    let encoding: u32 = 0x0E208FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_q_0_size_variant_0_35840_0e608c00()
 {
    // Encoding: 0x0E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, U=0, Rn=0, Rd=0, size=1, Rm=0
    let encoding: u32 = 0x0E608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_q_1_size_variant_1_35840_4e608c00()
 {
    // Encoding: 0x4E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value Q = 1 (Size variant 1)
    // Fields: Rm=0, Rn=0, Rd=0, size=1, Q=1, U=0
    let encoding: u32 = 0x4E608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_size_0_size_variant_0_35840_0e208c00()
 {
    // Encoding: 0x0E208C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value size = 0 (Size variant 0)
    // Fields: Rm=0, Rd=0, size=0, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_size_1_size_variant_1_35840_0e608c00()
 {
    // Encoding: 0x0E608C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value size = 1 (Size variant 1)
    // Fields: Rm=0, U=0, Rn=0, Q=0, size=1, Rd=0
    let encoding: u32 = 0x0E608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_size_2_size_variant_2_35840_0ea08c00()
 {
    // Encoding: 0x0EA08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value size = 2 (Size variant 2)
    // Fields: Rm=0, size=2, Rn=0, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0EA08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_size_3_size_variant_3_35840_0ee08c00()
 {
    // Encoding: 0x0EE08C00
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value size = 3 (Size variant 3)
    // Fields: Rn=0, U=0, Rd=0, size=3, Q=0, Rm=0
    let encoding: u32 = 0x0EE08C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_rn_31_stack_pointer_sp_may_require_alignment_35840_0e608fe0()
 {
    // Encoding: 0x0E608FE0
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, size=1, Rn=31, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0E608FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_35840_0e608c1f()
 {
    // Encoding: 0x0E608C1F
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rd=31, Q=0, Rn=0, U=0, Rm=0
    let encoding: u32 = 0x0E608C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_reg_write_0_5e208c00() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd register write: SimdFromField("d")
    // Encoding: 0x5E208C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E208C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_sp_rn_5e208fe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd with Rn = SP (31)
    // Encoding: 0x5E208FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E208FE0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_zr_rd_5e208c1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd with Rd = ZR (31)
    // Encoding: 0x5E208C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E208C1F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_reg_write_0_0e208c00() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd register write: SimdFromField("d")
    // Encoding: 0x0E208C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E208C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_sp_rn_0e208fe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd with Rn = SP (31)
    // Encoding: 0x0E208FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E208FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_zr_rd_0e208c1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd with Rd = ZR (31)
    // Encoding: 0x0E208C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E208C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
