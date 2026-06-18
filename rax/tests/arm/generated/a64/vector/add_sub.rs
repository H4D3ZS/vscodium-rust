//! A64 vector add_sub tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_fp16 Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_q_0_min_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Q = 0 (Min)
    // Fields: U=0, Rd=0, Rm=0, Rn=0, Q=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_q_1_max_1400_4e401400() {
    // Encoding: 0x4E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Q = 1 (Max)
    // Fields: Rm=0, Q=1, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x4E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_u_0_min_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field U = 0 (Min)
    // Fields: U=0, Rn=0, Rm=0, Q=0, Rd=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_u_1_max_1400_2e401400() {
    // Encoding: 0x2E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field U = 1 (Max)
    // Fields: Q=0, U=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x2E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rm_0_min_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rm = 0 (Min)
    // Fields: Rm=0, Rn=0, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rm_1_poweroftwo_1400_0e411400() {
    // Encoding: 0x0E411400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, Q=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E411400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rm_30_poweroftwominusone_1400_0e5e1400()
 {
    // Encoding: 0x0E5E1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=0, Q=0, Rm=30, U=0
    let encoding: u32 = 0x0E5E1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rm_31_max_1400_0e5f1400() {
    // Encoding: 0x0E5F1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rm = 31 (Max)
    // Fields: U=0, Q=0, Rd=0, Rm=31, Rn=0
    let encoding: u32 = 0x0E5F1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rn_0_min_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rn = 0 (Min)
    // Fields: U=0, Q=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rn_1_poweroftwo_1400_0e401420() {
    // Encoding: 0x0E401420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, Q=0, Rn=1, U=0
    let encoding: u32 = 0x0E401420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rn_30_poweroftwominusone_1400_0e4017c0()
 {
    // Encoding: 0x0E4017C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rm=0, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E4017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rn_31_max_1400_0e4017e0() {
    // Encoding: 0x0E4017E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rn = 31 (Max)
    // Fields: Rm=0, U=0, Rd=0, Q=0, Rn=31
    let encoding: u32 = 0x0E4017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rd_0_min_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rd = 0 (Min)
    // Fields: Q=0, Rn=0, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rd_1_poweroftwo_1400_0e401401() {
    // Encoding: 0x0E401401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, Rm=0, Rn=0, Rd=1, U=0
    let encoding: u32 = 0x0E401401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rd_30_poweroftwominusone_1400_0e40141e()
 {
    // Encoding: 0x0E40141E
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rm=0, U=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E40141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_field_rd_31_max_1400_0e40141f() {
    // Encoding: 0x0E40141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field Rd = 31 (Max)
    // Fields: Rd=31, U=0, Rm=0, Q=0, Rn=0
    let encoding: u32 = 0x0E40141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_0_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_1_1400_4e401400() {
    // Encoding: 0x4E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=1, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=1, Rm=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x4E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_2_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_3_1400_2e401400() {
    // Encoding: 0x2E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=1, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rm=0, Rn=0, U=1
    let encoding: u32 = 0x2E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_4_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_5_1400_0e411400() {
    // Encoding: 0x0E411400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=1, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, Rd=0, Rm=1
    let encoding: u32 = 0x0E411400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_6_1400_0e5e1400() {
    // Encoding: 0x0E5E1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=30, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, Rn=0, U=0, Rm=30
    let encoding: u32 = 0x0E5E1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_7_1400_0e5f1400() {
    // Encoding: 0x0E5F1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=31, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rm=31, Rd=0, U=0
    let encoding: u32 = 0x0E5F1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_8_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, Rm=0, U=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_9_1400_0e401420() {
    // Encoding: 0x0E401420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=1, Rd=0
    // Fields: Rd=0, Rm=0, Rn=1, Q=0, U=0
    let encoding: u32 = 0x0E401420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_10_1400_0e4017c0() {
    // Encoding: 0x0E4017C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=30, Rd=0
    // Fields: Q=0, U=0, Rn=30, Rm=0, Rd=0
    let encoding: u32 = 0x0E4017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_11_1400_0e4017e0() {
    // Encoding: 0x0E4017E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0E4017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_12_1400_0e401400() {
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, U=0, Q=0, Rd=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_13_1400_0e401401() {
    // Encoding: 0x0E401401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=1
    // Fields: Rd=1, U=0, Rm=0, Q=0, Rn=0
    let encoding: u32 = 0x0E401401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_14_1400_0e40141e() {
    // Encoding: 0x0E40141E
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=30
    // Fields: Rd=30, Rm=0, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0E40141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_15_1400_0e40141f() {
    // Encoding: 0x0E40141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, Q=0, U=0, Rm=0, Rd=31
    let encoding: u32 = 0x0E40141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_16_1400_0e411420() {
    // Encoding: 0x0E411420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=1, Rn=1, Rd=0
    // Fields: Q=0, Rd=0, U=0, Rn=1, Rm=1
    let encoding: u32 = 0x0E411420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_17_1400_0e5f17e0() {
    // Encoding: 0x0E5F17E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=31, Rn=31, Rd=0
    // Fields: Q=0, U=0, Rm=31, Rn=31, Rd=0
    let encoding: u32 = 0x0E5F17E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_18_1400_0e411401() {
    // Encoding: 0x0E411401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=1, Rn=0, Rd=1
    // Fields: Q=0, Rm=1, U=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E411401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_19_1400_0e5f141f() {
    // Encoding: 0x0E5F141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=31, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, Q=0, U=0, Rm=31
    let encoding: u32 = 0x0E5F141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_20_1400_0e401421() {
    // Encoding: 0x0E401421
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=1, Rd=1
    // Fields: Rm=0, U=0, Rn=1, Rd=1, Q=0
    let encoding: u32 = 0x0E401421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_combo_21_1400_0e4017ff() {
    // Encoding: 0x0E4017FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 field combination: Q=0, U=0, Rm=0, Rn=31, Rd=31
    // Fields: Rm=0, U=0, Rn=31, Q=0, Rd=31
    let encoding: u32 = 0x0E4017FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_special_q_0_size_variant_0_5120_0e401400()
{
    // Encoding: 0x0E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 special value Q = 0 (Size variant 0)
    // Fields: U=0, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_special_q_1_size_variant_1_5120_4e401400()
{
    // Encoding: 0x4E401400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 special value Q = 1 (Size variant 1)
    // Fields: U=0, Rn=0, Rd=0, Rm=0, Q=1
    let encoding: u32 = 0x4E401400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_special_rn_31_stack_pointer_sp_may_require_alignment_5120_0e4017e0()
 {
    // Encoding: 0x0E4017E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rm=0, U=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E4017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_5120_0e40141f()
 {
    // Encoding: 0x0E40141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, Rm=0, U=0, Rd=31, Rn=0
    let encoding: u32 = 0x0E40141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_q_0_min_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Q = 0 (Min)
    // Fields: Rd=0, sz=0, Q=0, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_q_1_max_d400_4e20d400() {
    // Encoding: 0x4E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Q = 1 (Max)
    // Fields: U=0, sz=0, Rm=0, Rd=0, Rn=0, Q=1
    let encoding: u32 = 0x4E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_u_0_min_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field U = 0 (Min)
    // Fields: sz=0, Rm=0, Q=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_u_1_max_d400_2e20d400() {
    // Encoding: 0x2E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field U = 1 (Max)
    // Fields: U=1, sz=0, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_sz_0_min_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field sz = 0 (Min)
    // Fields: sz=0, Rn=0, Rm=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_sz_1_max_d400_0e60d400() {
    // Encoding: 0x0E60D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field sz = 1 (Max)
    // Fields: U=0, sz=1, Rm=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E60D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rm_0_min_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rm = 0 (Min)
    // Fields: U=0, Rm=0, Rn=0, Rd=0, sz=0, Q=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rm_1_poweroftwo_d400_0e21d400() {
    // Encoding: 0x0E21D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rm = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, Rd=0, sz=0, Q=0, Rm=1
    let encoding: u32 = 0x0E21D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rm_30_poweroftwominusone_d400_0e3ed400()
 {
    // Encoding: 0x0E3ED400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rd=0, Rn=0, Rm=30, U=0, sz=0
    let encoding: u32 = 0x0E3ED400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rm_31_max_d400_0e3fd400() {
    // Encoding: 0x0E3FD400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rm = 31 (Max)
    // Fields: Rm=31, Rd=0, U=0, Q=0, sz=0, Rn=0
    let encoding: u32 = 0x0E3FD400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rn_0_min_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0, Rm=0, Q=0, U=0, sz=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rn_1_poweroftwo_d400_0e20d420() {
    // Encoding: 0x0E20D420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, Q=0, U=0, sz=0, Rm=0
    let encoding: u32 = 0x0E20D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rn_30_poweroftwominusone_d400_0e20d7c0()
 {
    // Encoding: 0x0E20D7C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Q=0, Rd=0, U=0, sz=0, Rm=0
    let encoding: u32 = 0x0E20D7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rn_31_max_d400_0e20d7e0() {
    // Encoding: 0x0E20D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rn = 31 (Max)
    // Fields: U=0, Rm=0, Q=0, Rn=31, sz=0, Rd=0
    let encoding: u32 = 0x0E20D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rd_0_min_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rd = 0 (Min)
    // Fields: Rn=0, Q=0, Rm=0, Rd=0, U=0, sz=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rd_1_poweroftwo_d400_0e20d401() {
    // Encoding: 0x0E20D401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rd = 1 (PowerOfTwo)
    // Fields: sz=0, Q=0, U=0, Rm=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E20D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rd_30_poweroftwominusone_d400_0e20d41e()
 {
    // Encoding: 0x0E20D41E
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, U=0, Q=0, sz=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E20D41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_field_rd_31_max_d400_0e20d41f() {
    // Encoding: 0x0E20D41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field Rd = 31 (Max)
    // Fields: Rd=31, Rm=0, sz=0, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0E20D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_0_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, U=0, sz=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_1_d400_4e20d400() {
    // Encoding: 0x4E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=1, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: sz=0, Rm=0, Q=1, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x4E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_2_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, U=0, sz=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_3_d400_2e20d400() {
    // Encoding: 0x2E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=1, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: U=1, sz=0, Rm=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x2E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_4_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, U=0, Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_5_d400_0e60d400() {
    // Encoding: 0x0E60D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=1, Rm=0, Rn=0, Rd=0
    // Fields: sz=1, U=0, Rd=0, Q=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E60D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_6_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Rn=0, Rd=0, Q=0, sz=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_7_d400_0e21d400() {
    // Encoding: 0x0E21D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=1, Rn=0, Rd=0
    // Fields: Rm=1, Q=0, Rn=0, U=0, Rd=0, sz=0
    let encoding: u32 = 0x0E21D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_8_d400_0e3ed400() {
    // Encoding: 0x0E3ED400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=30, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rm=30, Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x0E3ED400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_9_d400_0e3fd400() {
    // Encoding: 0x0E3FD400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=31, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, U=0, sz=0, Rm=31
    let encoding: u32 = 0x0E3FD400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_10_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rm=0, Rd=0, U=0, sz=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_11_d400_0e20d420() {
    // Encoding: 0x0E20D420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=1, Rd=0
    // Fields: sz=0, Rn=1, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_12_d400_0e20d7c0() {
    // Encoding: 0x0E20D7C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=30, Rd=0
    // Fields: U=0, Q=0, Rn=30, Rm=0, Rd=0, sz=0
    let encoding: u32 = 0x0E20D7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_13_d400_0e20d7e0() {
    // Encoding: 0x0E20D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=31, Rd=0
    // Fields: sz=0, Rm=0, Q=0, Rn=31, Rd=0, U=0
    let encoding: u32 = 0x0E20D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_14_d400_0e20d400() {
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rd=0, sz=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_15_d400_0e20d401() {
    // Encoding: 0x0E20D401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=1
    // Fields: Q=0, Rm=0, Rn=0, Rd=1, U=0, sz=0
    let encoding: u32 = 0x0E20D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_16_d400_0e20d41e() {
    // Encoding: 0x0E20D41E
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=30
    // Fields: Rd=30, Rn=0, Rm=0, sz=0, U=0, Q=0
    let encoding: u32 = 0x0E20D41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_17_d400_0e20d41f() {
    // Encoding: 0x0E20D41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, Q=0, U=0, Rm=0, sz=0
    let encoding: u32 = 0x0E20D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_18_d400_0e21d420() {
    // Encoding: 0x0E21D420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=1, Rn=1, Rd=0
    // Fields: Q=0, Rm=1, U=0, sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x0E21D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_19_d400_0e3fd7e0() {
    // Encoding: 0x0E3FD7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=31, Rn=31, Rd=0
    // Fields: Rd=0, U=0, sz=0, Q=0, Rm=31, Rn=31
    let encoding: u32 = 0x0E3FD7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_20_d400_0e21d401() {
    // Encoding: 0x0E21D401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=1, Rn=0, Rd=1
    // Fields: Q=0, Rm=1, sz=0, Rn=0, Rd=1, U=0
    let encoding: u32 = 0x0E21D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_21_d400_0e3fd41f() {
    // Encoding: 0x0E3FD41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=31, Rn=0, Rd=31
    // Fields: Rn=0, U=0, sz=0, Q=0, Rm=31, Rd=31
    let encoding: u32 = 0x0E3FD41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_22_d400_0e20d421() {
    // Encoding: 0x0E20D421
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=1, Rd=1
    // Fields: sz=0, Q=0, Rn=1, Rd=1, U=0, Rm=0
    let encoding: u32 = 0x0E20D421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_combo_23_d400_0e20d7ff() {
    // Encoding: 0x0E20D7FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp field combination: Q=0, U=0, sz=0, Rm=0, Rn=31, Rd=31
    // Fields: Rd=31, U=0, Q=0, Rn=31, sz=0, Rm=0
    let encoding: u32 = 0x0E20D7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_special_q_0_size_variant_0_54272_0e60d400()
{
    // Encoding: 0x0E60D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp special value Q = 0 (Size variant 0)
    // Fields: Rd=0, Q=0, sz=1, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E60D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_special_q_1_size_variant_1_54272_4e60d400()
{
    // Encoding: 0x4E60D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp special value Q = 1 (Size variant 1)
    // Fields: Rm=0, Rd=0, Rn=0, Q=1, U=0, sz=1
    let encoding: u32 = 0x4E60D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_special_sz_0_size_variant_0_54272_0e20d400()
{
    // Encoding: 0x0E20D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp special value sz = 0 (Size variant 0)
    // Fields: Q=0, Rm=0, U=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x0E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_special_sz_1_size_variant_1_54272_0e60d400()
{
    // Encoding: 0x0E60D400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp special value sz = 1 (Size variant 1)
    // Fields: Rd=0, Rn=0, sz=1, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0E60D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_special_rn_31_stack_pointer_sp_may_require_alignment_54272_0e60d7e0()
 {
    // Encoding: 0x0E60D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, sz=1, U=0, Q=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E60D7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_54272_0e60d41f()
 {
    // Encoding: 0x0E60D41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sz=1, Rm=0, U=0, Rn=0, Rd=31, Q=0
    let encoding: u32 = 0x0E60D41F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_reg_write_0_0e401400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 register write: SimdFromField("d")
    // Encoding: 0x0E401400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E401400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_sp_rn_0e4017e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 with Rn = SP (31)
    // Encoding: 0x0E4017E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E4017E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp16
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp16_zr_rd_0e40141f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp16 with Rd = ZR (31)
    // Encoding: 0x0E40141F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E40141F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_reg_write_0_0e20d400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp register write: SimdFromField("d")
    // Encoding: 0x0E20D400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20D400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_sp_rn_0e20d7e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp with Rn = SP (31)
    // Encoding: 0x0E20D7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20D7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_zr_rd_0e20d41f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp with Rd = ZR (31)
    // Encoding: 0x0E20D41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20D41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_u_0_min_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field U = 0 (Min)
    // Fields: Rn=0, Rd=0, size=0, Rm=0, U=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_u_1_max_c00_7e200c00() {
    // Encoding: 0x7E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field U = 1 (Max)
    // Fields: size=0, U=1, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_size_0_min_c00_5e200c00()
{
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field size = 0 (Min)
    // Fields: Rd=0, Rm=0, U=0, size=0, Rn=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_size_1_poweroftwo_c00_5e600c00()
 {
    // Encoding: 0x5E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field size = 1 (PowerOfTwo)
    // Fields: Rm=0, size=1, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_size_2_poweroftwo_c00_5ea00c00()
 {
    // Encoding: 0x5EA00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field size = 2 (PowerOfTwo)
    // Fields: U=0, Rn=0, Rd=0, Rm=0, size=2
    let encoding: u32 = 0x5EA00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_size_3_max_c00_5ee00c00()
{
    // Encoding: 0x5EE00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field size = 3 (Max)
    // Fields: Rn=0, size=3, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x5EE00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rm_0_min_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rm = 0 (Min)
    // Fields: Rn=0, Rm=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rm_1_poweroftwo_c00_5e210c00()
 {
    // Encoding: 0x5E210C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, U=0, Rm=1, size=0
    let encoding: u32 = 0x5E210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rm_30_poweroftwominusone_c00_5e3e0c00()
 {
    // Encoding: 0x5E3E0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, U=0, Rm=30, Rd=0
    let encoding: u32 = 0x5E3E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rm_31_max_c00_5e3f0c00()
{
    // Encoding: 0x5E3F0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rm = 31 (Max)
    // Fields: size=0, Rm=31, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E3F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rn_0_min_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rn = 0 (Min)
    // Fields: size=0, Rm=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rn_1_poweroftwo_c00_5e200c20()
 {
    // Encoding: 0x5E200C20
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, size=0, Rd=0, Rn=1, Rm=0
    let encoding: u32 = 0x5E200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rn_30_poweroftwominusone_c00_5e200fc0()
 {
    // Encoding: 0x5E200FC0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, U=0, Rn=30, Rd=0, Rm=0
    let encoding: u32 = 0x5E200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rn_31_max_c00_5e200fe0()
{
    // Encoding: 0x5E200FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rn = 31 (Max)
    // Fields: size=0, Rn=31, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x5E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rd_0_min_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rd = 0 (Min)
    // Fields: size=0, Rm=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rd_1_poweroftwo_c00_5e200c01()
 {
    // Encoding: 0x5E200C01
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rd = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, Rd=1, size=0, Rm=0
    let encoding: u32 = 0x5E200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rd_30_poweroftwominusone_c00_5e200c1e()
 {
    // Encoding: 0x5E200C1E
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rd=30, Rn=0, U=0, Rm=0
    let encoding: u32 = 0x5E200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_field_rd_31_max_c00_5e200c1f()
{
    // Encoding: 0x5E200C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field Rd = 31 (Max)
    // Fields: size=0, Rd=31, Rn=0, U=0, Rm=0
    let encoding: u32 = 0x5E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_0_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Rm=0, size=0, Rn=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_1_c00_7e200c00() {
    // Encoding: 0x7E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=1, Rn=0, Rm=0, Rd=0, size=0
    let encoding: u32 = 0x7E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_2_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_3_c00_5e600c00() {
    // Encoding: 0x5E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, U=0, Rn=0, size=1
    let encoding: u32 = 0x5E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_4_c00_5ea00c00() {
    // Encoding: 0x5EA00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, size=2, Rd=0, Rn=0
    let encoding: u32 = 0x5EA00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_5_c00_5ee00c00() {
    // Encoding: 0x5EE00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, U=0, Rd=0, size=3
    let encoding: u32 = 0x5EE00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_6_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, size=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_7_c00_5e210c00() {
    // Encoding: 0x5E210C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rm=1, Rn=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x5E210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_8_c00_5e3e0c00() {
    // Encoding: 0x5E3E0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rm=30, Rn=0, U=0, size=0, Rd=0
    let encoding: u32 = 0x5E3E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_9_c00_5e3f0c00() {
    // Encoding: 0x5E3F0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rm=31, U=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x5E3F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_10_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, size=0, Rn=0, U=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_11_c00_5e200c20() {
    // Encoding: 0x5E200C20
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: Rn=1, Rd=0, U=0, size=0, Rm=0
    let encoding: u32 = 0x5E200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_12_c00_5e200fc0() {
    // Encoding: 0x5E200FC0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: U=0, size=0, Rn=30, Rd=0, Rm=0
    let encoding: u32 = 0x5E200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_13_c00_5e200fe0() {
    // Encoding: 0x5E200FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: size=0, Rm=0, Rn=31, U=0, Rd=0
    let encoding: u32 = 0x5E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_14_c00_5e200c00() {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, Rm=0, size=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_15_c00_5e200c01() {
    // Encoding: 0x5E200C01
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rm=0, Rd=1, Rn=0, U=0, size=0
    let encoding: u32 = 0x5E200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_16_c00_5e200c1e() {
    // Encoding: 0x5E200C1E
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Rn=0, Rm=0, Rd=30, size=0, U=0
    let encoding: u32 = 0x5E200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_17_c00_5e200c1f() {
    // Encoding: 0x5E200C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, Rm=0, size=0, Rd=31, U=0
    let encoding: u32 = 0x5E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_18_c00_5e210c20() {
    // Encoding: 0x5E210C20
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rm=1, U=0, size=0, Rd=0, Rn=1
    let encoding: u32 = 0x5E210C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_19_c00_5e3f0fe0() {
    // Encoding: 0x5E3F0FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: U=0, size=0, Rn=31, Rd=0, Rm=31
    let encoding: u32 = 0x5E3F0FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_20_c00_5e210c01() {
    // Encoding: 0x5E210C01
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: size=0, U=0, Rm=1, Rd=1, Rn=0
    let encoding: u32 = 0x5E210C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_21_c00_5e3f0c1f() {
    // Encoding: 0x5E3F0C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Rd=31, U=0, Rm=31, size=0, Rn=0
    let encoding: u32 = 0x5E3F0C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_22_c00_5e200c21() {
    // Encoding: 0x5E200C21
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: U=0, Rn=1, Rm=0, Rd=1, size=0
    let encoding: u32 = 0x5E200C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_combo_23_c00_5e200fff() {
    // Encoding: 0x5E200FFF
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Rm=0, U=0, Rd=31, size=0, Rn=31
    let encoding: u32 = 0x5E200FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_special_size_0_size_variant_0_3072_5e200c00()
 {
    // Encoding: 0x5E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd special value size = 0 (Size variant 0)
    // Fields: Rn=0, size=0, Rm=0, Rd=0, U=0
    let encoding: u32 = 0x5E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_special_size_1_size_variant_1_3072_5e600c00()
 {
    // Encoding: 0x5E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd special value size = 1 (Size variant 1)
    // Fields: Rd=0, Rm=0, U=0, size=1, Rn=0
    let encoding: u32 = 0x5E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_special_size_2_size_variant_2_3072_5ea00c00()
 {
    // Encoding: 0x5EA00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd special value size = 2 (Size variant 2)
    // Fields: U=0, size=2, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5EA00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_special_size_3_size_variant_3_3072_5ee00c00()
 {
    // Encoding: 0x5EE00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd special value size = 3 (Size variant 3)
    // Fields: Rm=0, size=3, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5EE00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_3072_5e600fe0()
 {
    // Encoding: 0x5E600FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rm=0, Rd=0, size=1, U=0
    let encoding: u32 = 0x5E600FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_3072_5e600c1f()
 {
    // Encoding: 0x5E600C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rm=0, U=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E600C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_q_0_min_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Q = 0 (Min)
    // Fields: Q=0, size=0, Rm=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_q_1_max_c00_4e200c00() {
    // Encoding: 0x4E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Q = 1 (Max)
    // Fields: Rn=0, Q=1, Rd=0, U=0, size=0, Rm=0
    let encoding: u32 = 0x4E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_u_0_min_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field U = 0 (Min)
    // Fields: Q=0, U=0, size=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_u_1_max_c00_2e200c00() {
    // Encoding: 0x2E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field U = 1 (Max)
    // Fields: U=1, size=0, Q=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x2E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_size_0_min_c00_0e200c00()
{
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field size = 0 (Min)
    // Fields: Q=0, Rd=0, Rn=0, U=0, Rm=0, size=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_size_1_poweroftwo_c00_0e600c00()
 {
    // Encoding: 0x0E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field size = 1 (PowerOfTwo)
    // Fields: U=0, Q=0, size=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_size_2_poweroftwo_c00_0ea00c00()
 {
    // Encoding: 0x0EA00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field size = 2 (PowerOfTwo)
    // Fields: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EA00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_size_3_max_c00_0ee00c00()
{
    // Encoding: 0x0EE00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field size = 3 (Max)
    // Fields: Rm=0, Q=0, Rd=0, U=0, Rn=0, size=3
    let encoding: u32 = 0x0EE00C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rm_0_min_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rm = 0 (Min)
    // Fields: Q=0, U=0, Rm=0, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rm_1_poweroftwo_c00_0e210c00()
 {
    // Encoding: 0x0E210C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rm = 1 (PowerOfTwo)
    // Fields: Q=0, size=0, U=0, Rn=0, Rm=1, Rd=0
    let encoding: u32 = 0x0E210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rm_30_poweroftwominusone_c00_0e3e0c00()
 {
    // Encoding: 0x0E3E0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Q=0, Rm=30, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E3E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rm_31_max_c00_0e3f0c00()
{
    // Encoding: 0x0E3F0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rm = 31 (Max)
    // Fields: Rd=0, Rn=0, U=0, size=0, Q=0, Rm=31
    let encoding: u32 = 0x0E3F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rn_0_min_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rn = 0 (Min)
    // Fields: U=0, Rn=0, size=0, Rm=0, Q=0, Rd=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rn_1_poweroftwo_c00_0e200c20()
 {
    // Encoding: 0x0E200C20
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, U=0, Q=0, Rm=0, size=0
    let encoding: u32 = 0x0E200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rn_30_poweroftwominusone_c00_0e200fc0()
 {
    // Encoding: 0x0E200FC0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Q=0, Rn=30, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0E200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rn_31_max_c00_0e200fe0()
{
    // Encoding: 0x0E200FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rn = 31 (Max)
    // Fields: U=0, Q=0, size=0, Rm=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rd_0_min_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rd = 0 (Min)
    // Fields: U=0, Q=0, Rd=0, Rn=0, Rm=0, size=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rd_1_poweroftwo_c00_0e200c01()
 {
    // Encoding: 0x0E200C01
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rd = 1 (PowerOfTwo)
    // Fields: U=0, size=0, Rm=0, Rn=0, Rd=1, Q=0
    let encoding: u32 = 0x0E200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rd_30_poweroftwominusone_c00_0e200c1e()
 {
    // Encoding: 0x0E200C1E
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rd=30, Rm=0, Rn=0, size=0, U=0
    let encoding: u32 = 0x0E200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_field_rd_31_max_c00_0e200c1f()
{
    // Encoding: 0x0E200C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field Rd = 31 (Max)
    // Fields: Q=0, Rd=31, Rm=0, size=0, Rn=0, U=0
    let encoding: u32 = 0x0E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_0_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rm=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_1_c00_4e200c00() {
    // Encoding: 0x4E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rn=0, Q=1, Rm=0, Rd=0
    let encoding: u32 = 0x4E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_2_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Rn=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_3_c00_2e200c00() {
    // Encoding: 0x2E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rm=0, U=1, Rn=0, Rd=0
    let encoding: u32 = 0x2E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_4_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Q=0, Rd=0, size=0, Rn=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_5_c00_0e600c00() {
    // Encoding: 0x0E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, size=1, Rm=0, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_6_c00_0ea00c00() {
    // Encoding: 0x0EA00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, size=2, Rd=0, Rm=0
    let encoding: u32 = 0x0EA00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_7_c00_0ee00c00() {
    // Encoding: 0x0EE00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: size=3, U=0, Q=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0EE00C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_8_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, Rn=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_9_c00_0e210c00() {
    // Encoding: 0x0E210C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rd=0, U=0, size=0, Rm=1
    let encoding: u32 = 0x0E210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_10_c00_0e3e0c00() {
    // Encoding: 0x0E3E0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: size=0, U=0, Rn=0, Rd=0, Rm=30, Q=0
    let encoding: u32 = 0x0E3E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_11_c00_0e3f0c00() {
    // Encoding: 0x0E3F0C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rm=31, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0E3F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_12_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rm=0, Rn=0, Q=0, size=0, Rd=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_13_c00_0e200c20() {
    // Encoding: 0x0E200C20
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: Q=0, size=0, Rn=1, Rm=0, Rd=0, U=0
    let encoding: u32 = 0x0E200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_14_c00_0e200fc0() {
    // Encoding: 0x0E200FC0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Rm=0, Rn=30, U=0, size=0, Rd=0, Q=0
    let encoding: u32 = 0x0E200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_15_c00_0e200fe0() {
    // Encoding: 0x0E200FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: U=0, Rd=0, Rm=0, Rn=31, size=0, Q=0
    let encoding: u32 = 0x0E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_16_c00_0e200c00() {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, U=0, Q=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_17_c00_0e200c01() {
    // Encoding: 0x0E200C01
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: size=0, Rm=0, U=0, Q=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_18_c00_0e200c1e() {
    // Encoding: 0x0E200C1E
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, Q=0, Rm=0, U=0, size=0
    let encoding: u32 = 0x0E200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_19_c00_0e200c1f() {
    // Encoding: 0x0E200C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rd=31, U=0, Q=0, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_20_c00_0e210c20() {
    // Encoding: 0x0E210C20
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: size=0, Rn=1, Rd=0, U=0, Rm=1, Q=0
    let encoding: u32 = 0x0E210C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_21_c00_0e3f0fe0() {
    // Encoding: 0x0E3F0FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: size=0, Rm=31, Q=0, U=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E3F0FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_22_c00_0e210c01() {
    // Encoding: 0x0E210C01
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Q=0, U=0, Rn=0, Rm=1, Rd=1, size=0
    let encoding: u32 = 0x0E210C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_23_c00_0e3f0c1f() {
    // Encoding: 0x0E3F0C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Rn=0, U=0, Rd=31, Q=0, Rm=31, size=0
    let encoding: u32 = 0x0E3F0C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_24_c00_0e200c21() {
    // Encoding: 0x0E200C21
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: size=0, Rm=0, Q=0, U=0, Rd=1, Rn=1
    let encoding: u32 = 0x0E200C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_combo_25_c00_0e200fff() {
    // Encoding: 0x0E200FFF
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Q=0, Rd=31, size=0, Rm=0, U=0, Rn=31
    let encoding: u32 = 0x0E200FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_q_0_size_variant_0_3072_0e600c00()
 {
    // Encoding: 0x0E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value Q = 0 (Size variant 0)
    // Fields: size=1, Rm=0, Rn=0, U=0, Q=0, Rd=0
    let encoding: u32 = 0x0E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_q_1_size_variant_1_3072_4e600c00()
 {
    // Encoding: 0x4E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value Q = 1 (Size variant 1)
    // Fields: Rm=0, Rd=0, size=1, Rn=0, Q=1, U=0
    let encoding: u32 = 0x4E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_size_0_size_variant_0_3072_0e200c00()
 {
    // Encoding: 0x0E200C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, size=0, Rm=0, Q=0, U=0
    let encoding: u32 = 0x0E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_size_1_size_variant_1_3072_0e600c00()
 {
    // Encoding: 0x0E600C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value size = 1 (Size variant 1)
    // Fields: U=0, size=1, Rd=0, Rn=0, Q=0, Rm=0
    let encoding: u32 = 0x0E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_size_2_size_variant_2_3072_0ea00c00()
 {
    // Encoding: 0x0EA00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value size = 2 (Size variant 2)
    // Fields: U=0, Rd=0, size=2, Q=0, Rn=0, Rm=0
    let encoding: u32 = 0x0EA00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_size_3_size_variant_3_3072_0ee00c00()
 {
    // Encoding: 0x0EE00C00
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value size = 3 (Size variant 3)
    // Fields: Q=0, Rm=0, Rd=0, size=3, U=0, Rn=0
    let encoding: u32 = 0x0EE00C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_rn_31_stack_pointer_sp_may_require_alignment_3072_0e600fe0()
 {
    // Encoding: 0x0E600FE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rm=0, Rn=31, Rd=0, U=0, size=1
    let encoding: u32 = 0x0E600FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_3072_0e600c1f()
 {
    // Encoding: 0x0E600C1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, Rn=0, Q=0, Rd=31, size=1, U=0
    let encoding: u32 = 0x0E600C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_reg_write_0_5e200c00() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd register write: SimdFromField("d")
    // Encoding: 0x5E200C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E200C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_sp_rn_5e200fe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd with Rn = SP (31)
    // Encoding: 0x5E200FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E200FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_zr_rd_5e200c1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd with Rd = ZR (31)
    // Encoding: 0x5E200C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E200C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_reg_write_0_0e200c00() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd register write: SimdFromField("d")
    // Encoding: 0x0E200C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E200C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_sp_rn_0e200fe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd with Rn = SP (31)
    // Encoding: 0x0E200FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E200FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_zr_rd_0e200c1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_saturating_simd with Rd = ZR (31)
    // Encoding: 0x0E200C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E200C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_disparate_add_sub_long Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_q_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Q = 0 (Min)
    // Fields: Rm=0, size=0, Q=0, U=0, Rn=0, Rd=0, o1=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_q_1_max_0_4e200000() {
    // Encoding: 0x4E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Q = 1 (Max)
    // Fields: U=0, o1=0, Rd=0, Rm=0, Rn=0, Q=1, size=0
    let encoding: u32 = 0x4E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_u_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field U = 0 (Min)
    // Fields: U=0, Q=0, Rm=0, o1=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_u_1_max_0_2e200000() {
    // Encoding: 0x2E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field U = 1 (Max)
    // Fields: Q=0, U=1, Rm=0, Rd=0, size=0, o1=0, Rn=0
    let encoding: u32 = 0x2E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_size_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field size = 0 (Min)
    // Fields: Q=0, Rm=0, size=0, U=0, Rn=0, o1=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_size_1_poweroftwo_0_0e600000()
{
    // Encoding: 0x0E600000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field size = 1 (PowerOfTwo)
    // Fields: Rm=0, size=1, Rd=0, Rn=0, Q=0, U=0, o1=0
    let encoding: u32 = 0x0E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_size_2_poweroftwo_0_0ea00000()
{
    // Encoding: 0x0EA00000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field size = 2 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, o1=0, Q=0, U=0, Rm=0, size=2
    let encoding: u32 = 0x0EA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_size_3_max_0_0ee00000() {
    // Encoding: 0x0EE00000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field size = 3 (Max)
    // Fields: Rn=0, size=3, o1=0, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EE00000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rm_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rm = 0 (Min)
    // Fields: U=0, Rm=0, Rn=0, o1=0, Q=0, size=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rm_1_poweroftwo_0_0e210000() {
    // Encoding: 0x0E210000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, U=0, o1=0, Q=0, size=0, Rm=1
    let encoding: u32 = 0x0E210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rm_30_poweroftwominusone_0_0e3e0000()
 {
    // Encoding: 0x0E3E0000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Rn=0, Rd=0, Rm=30, U=0, Q=0, size=0
    let encoding: u32 = 0x0E3E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rm_31_max_0_0e3f0000() {
    // Encoding: 0x0E3F0000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rm = 31 (Max)
    // Fields: Q=0, U=0, size=0, Rn=0, Rd=0, Rm=31, o1=0
    let encoding: u32 = 0x0E3F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_o1_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field o1 = 0 (Min)
    // Fields: U=0, o1=0, Rn=0, Q=0, size=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_o1_1_max_0_0e202000() {
    // Encoding: 0x0E202000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field o1 = 1 (Max)
    // Fields: Q=0, Rm=0, o1=1, U=0, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x0E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rn_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rn = 0 (Min)
    // Fields: Rd=0, size=0, U=0, Rm=0, o1=0, Rn=0, Q=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rn_1_poweroftwo_0_0e200020() {
    // Encoding: 0x0E200020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0, Q=0, U=0, size=0, Rm=0, o1=0
    let encoding: u32 = 0x0E200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rn_30_poweroftwominusone_0_0e2003c0()
 {
    // Encoding: 0x0E2003C0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Q=0, o1=0, size=0, Rn=30, Rd=0, Rm=0
    let encoding: u32 = 0x0E2003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rn_31_max_0_0e2003e0() {
    // Encoding: 0x0E2003E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rn = 31 (Max)
    // Fields: U=0, o1=0, Rn=31, size=0, Q=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rd_0_min_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rd = 0 (Min)
    // Fields: o1=0, Rn=0, Rd=0, Q=0, size=0, Rm=0, U=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rd_1_poweroftwo_0_0e200001() {
    // Encoding: 0x0E200001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Q=0, U=0, Rd=1, size=0, o1=0, Rm=0
    let encoding: u32 = 0x0E200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rd_30_poweroftwominusone_0_0e20001e()
 {
    // Encoding: 0x0E20001E
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rm=0, size=0, Rd=30, Rn=0, o1=0, U=0
    let encoding: u32 = 0x0E20001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_field_rd_31_max_0_0e20001f() {
    // Encoding: 0x0E20001F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field Rd = 31 (Max)
    // Fields: Q=0, o1=0, Rn=0, Rd=31, U=0, size=0, Rm=0
    let encoding: u32 = 0x0E20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_0_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, Q=0, size=0, Rm=0, o1=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_1_0_4e200000() {
    // Encoding: 0x4E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=1, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=1, U=0, size=0, Rn=0, Rd=0, o1=0
    let encoding: u32 = 0x4E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_2_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: U=0, o1=0, size=0, Q=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_3_0_2e200000() {
    // Encoding: 0x2E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=1, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rm=0, o1=0, size=0, Rn=0, Rd=0, Q=0, U=1
    let encoding: u32 = 0x2E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_4_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, U=0, size=0, Rm=0, o1=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_5_0_0e600000() {
    // Encoding: 0x0E600000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=1, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, U=0, o1=0, Rm=0, size=1, Rn=0
    let encoding: u32 = 0x0E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_6_0_0ea00000() {
    // Encoding: 0x0EA00000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=2, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Rm=0, U=0, o1=0, Q=0, size=2
    let encoding: u32 = 0x0EA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_7_0_0ee00000() {
    // Encoding: 0x0EE00000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=3, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Rn=0, Rd=0, Rm=0, Q=0, U=0, size=3
    let encoding: u32 = 0x0EE00000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_8_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rm=0, o1=0, U=0, Q=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_9_0_0e210000() {
    // Encoding: 0x0E210000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rm=1, o1=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_10_0_0e3e0000() {
    // Encoding: 0x0E3E0000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=30, o1=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, U=0, Rm=30, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E3E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_11_0_0e3f0000() {
    // Encoding: 0x0E3F0000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rm=31, o1=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0E3F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_12_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, size=0, U=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_13_0_0e202000() {
    // Encoding: 0x0E202000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=1, Rn=0, Rd=0
    // Fields: size=0, Q=0, Rn=0, Rd=0, o1=1, Rm=0, U=0
    let encoding: u32 = 0x0E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_14_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, o1=0, Rn=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_15_0_0e200020() {
    // Encoding: 0x0E200020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=1, Rd=0
    // Fields: o1=0, Rn=1, Rd=0, Q=0, U=0, size=0, Rm=0
    let encoding: u32 = 0x0E200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_16_0_0e2003c0() {
    // Encoding: 0x0E2003C0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=30, Rd=0
    // Fields: o1=0, Rm=0, Rn=30, Rd=0, U=0, size=0, Q=0
    let encoding: u32 = 0x0E2003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_17_0_0e2003e0() {
    // Encoding: 0x0E2003E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=0
    // Fields: Rn=31, size=0, Rm=0, Rd=0, U=0, o1=0, Q=0
    let encoding: u32 = 0x0E2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_18_0_0e200000() {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, Rm=0, Rd=0, Q=0, o1=0, size=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_19_0_0e200001() {
    // Encoding: 0x0E200001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=1
    // Fields: Rd=1, Q=0, U=0, size=0, Rm=0, o1=0, Rn=0
    let encoding: u32 = 0x0E200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_20_0_0e20001e() {
    // Encoding: 0x0E20001E
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=30
    // Fields: U=0, size=0, Q=0, Rm=0, o1=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E20001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_21_0_0e20001f() {
    // Encoding: 0x0E20001F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=31
    // Fields: Rd=31, Q=0, size=0, Rn=0, U=0, Rm=0, o1=0
    let encoding: u32 = 0x0E20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_22_0_0e210020() {
    // Encoding: 0x0E210020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=1, Rd=0
    // Fields: Q=0, U=0, size=0, Rm=1, Rn=1, o1=0, Rd=0
    let encoding: u32 = 0x0E210020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_23_0_0e3f03e0() {
    // Encoding: 0x0E3F03E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=31, Rd=0
    // Fields: Rm=31, size=0, Rn=31, o1=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0E3F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_24_0_0e210001() {
    // Encoding: 0x0E210001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=0, Rd=1
    // Fields: Rm=1, Rn=0, Rd=1, Q=0, size=0, U=0, o1=0
    let encoding: u32 = 0x0E210001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_25_0_0e3f001f() {
    // Encoding: 0x0E3F001F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=0, Rd=31
    // Fields: Q=0, U=0, o1=0, Rd=31, Rn=0, size=0, Rm=31
    let encoding: u32 = 0x0E3F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_26_0_0e200021() {
    // Encoding: 0x0E200021
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=1, Rd=1
    // Fields: Rn=1, Rm=0, size=0, Rd=1, U=0, o1=0, Q=0
    let encoding: u32 = 0x0E200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_combo_27_0_0e2003ff() {
    // Encoding: 0x0E2003FF
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=31
    // Fields: size=0, Rm=0, o1=0, Q=0, Rn=31, Rd=31, U=0
    let encoding: u32 = 0x0E2003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_q_0_size_variant_0_0_0e600000()
 {
    // Encoding: 0x0E600000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value Q = 0 (Size variant 0)
    // Fields: U=0, Rm=0, o1=0, Rn=0, Rd=0, size=1, Q=0
    let encoding: u32 = 0x0E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_q_1_size_variant_1_0_4e600000()
 {
    // Encoding: 0x4E600000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value Q = 1 (Size variant 1)
    // Fields: Rm=0, U=0, Rn=0, Rd=0, o1=0, Q=1, size=1
    let encoding: u32 = 0x4E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_size_0_size_variant_0_0_0e200000()
 {
    // Encoding: 0x0E200000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value size = 0 (Size variant 0)
    // Fields: o1=0, U=0, size=0, Rd=0, Rm=0, Rn=0, Q=0
    let encoding: u32 = 0x0E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_size_1_size_variant_1_0_0e600000()
 {
    // Encoding: 0x0E600000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value size = 1 (Size variant 1)
    // Fields: Rd=0, o1=0, size=1, U=0, Rm=0, Q=0, Rn=0
    let encoding: u32 = 0x0E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_size_2_size_variant_2_0_0ea00000()
 {
    // Encoding: 0x0EA00000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value size = 2 (Size variant 2)
    // Fields: Rm=0, Q=0, o1=0, size=2, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x0EA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_size_3_size_variant_3_0_0ee00000()
 {
    // Encoding: 0x0EE00000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value size = 3 (Size variant 3)
    // Fields: Q=0, size=3, U=0, Rm=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EE00000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_rn_31_stack_pointer_sp_may_require_alignment_0_0e6003e0()
 {
    // Encoding: 0x0E6003E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rm=0, Rn=31, o1=0, U=0, size=1, Rd=0
    let encoding: u32 = 0x0E6003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0e60001f()
 {
    // Encoding: 0x0E60001F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: o1=0, Rm=0, size=1, U=0, Q=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E60001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_reg_write_0_0e200000() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long register write: SimdFromField("d")
    // Encoding: 0x0E200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_sp_rn_0e2003e0() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long with Rn = SP (31)
    // Encoding: 0x0E2003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_long
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_long_zr_rd_0e20001f() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_long with Rd = ZR (31)
    // Encoding: 0x0E20001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_sub_int Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_q_0_min_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Q = 0 (Min)
    // Fields: Q=0, Rm=0, U=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_q_1_max_2400_4e202400() {
    // Encoding: 0x4E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Q = 1 (Max)
    // Fields: Q=1, Rd=0, U=0, Rn=0, Rm=0, size=0
    let encoding: u32 = 0x4E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_u_0_min_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field U = 0 (Min)
    // Fields: Q=0, size=0, Rm=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_u_1_max_2400_2e202400() {
    // Encoding: 0x2E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field U = 1 (Max)
    // Fields: Rd=0, U=1, size=0, Rm=0, Q=0, Rn=0
    let encoding: u32 = 0x2E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_size_0_min_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field size = 0 (Min)
    // Fields: Q=0, U=0, size=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_size_1_poweroftwo_2400_0e602400() {
    // Encoding: 0x0E602400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field size = 1 (PowerOfTwo)
    // Fields: U=0, size=1, Rm=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E602400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_size_2_poweroftwo_2400_0ea02400() {
    // Encoding: 0x0EA02400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field size = 2 (PowerOfTwo)
    // Fields: size=2, Q=0, Rn=0, Rm=0, Rd=0, U=0
    let encoding: u32 = 0x0EA02400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_size_3_max_2400_0ee02400() {
    // Encoding: 0x0EE02400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field size = 3 (Max)
    // Fields: size=3, Rd=0, Q=0, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0EE02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rm_0_min_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rm = 0 (Min)
    // Fields: Q=0, U=0, Rn=0, Rm=0, Rd=0, size=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rm_1_poweroftwo_2400_0e212400() {
    // Encoding: 0x0E212400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rm = 1 (PowerOfTwo)
    // Fields: Q=0, Rn=0, Rm=1, Rd=0, U=0, size=0
    let encoding: u32 = 0x0E212400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rm_30_poweroftwominusone_2400_0e3e2400()
 {
    // Encoding: 0x0E3E2400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, size=0, Rm=30, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E3E2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rm_31_max_2400_0e3f2400() {
    // Encoding: 0x0E3F2400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rm = 31 (Max)
    // Fields: size=0, Q=0, Rm=31, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x0E3F2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rn_0_min_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rn = 0 (Min)
    // Fields: U=0, Rn=0, Q=0, Rd=0, Rm=0, size=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rn_1_poweroftwo_2400_0e202420() {
    // Encoding: 0x0E202420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, U=0, size=0, Rn=1, Rm=0, Rd=0
    let encoding: u32 = 0x0E202420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rn_30_poweroftwominusone_2400_0e2027c0()
 {
    // Encoding: 0x0E2027C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, U=0, Rm=0, Rn=30, Rd=0, size=0
    let encoding: u32 = 0x0E2027C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rn_31_max_2400_0e2027e0() {
    // Encoding: 0x0E2027E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rn = 31 (Max)
    // Fields: Q=0, Rm=0, U=0, Rn=31, size=0, Rd=0
    let encoding: u32 = 0x0E2027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rd_0_min_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rd = 0 (Min)
    // Fields: Rm=0, Rn=0, size=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rd_1_poweroftwo_2400_0e202401() {
    // Encoding: 0x0E202401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rd = 1 (PowerOfTwo)
    // Fields: U=0, Rm=0, Rd=1, Q=0, size=0, Rn=0
    let encoding: u32 = 0x0E202401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rd_30_poweroftwominusone_2400_0e20241e()
 {
    // Encoding: 0x0E20241E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rd=30, Q=0, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E20241E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_field_rd_31_max_2400_0e20241f() {
    // Encoding: 0x0E20241F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field Rd = 31 (Max)
    // Fields: Rn=0, U=0, Rd=31, Q=0, size=0, Rm=0
    let encoding: u32 = 0x0E20241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_0_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Rn=0, U=0, Q=0, size=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_1_2400_4e202400() {
    // Encoding: 0x4E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Q=1, Rm=0, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x4E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_2_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, Rn=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_3_2400_2e202400() {
    // Encoding: 0x2E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=1, size=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_4_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, Rd=0, size=0, Rm=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_5_2400_0e602400() {
    // Encoding: 0x0E602400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, size=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E602400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_6_2400_0ea02400() {
    // Encoding: 0x0EA02400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, Rn=0, U=0, size=2, Q=0
    let encoding: u32 = 0x0EA02400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_7_2400_0ee02400() {
    // Encoding: 0x0EE02400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: size=3, U=0, Rm=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EE02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_8_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, U=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_9_2400_0e212400() {
    // Encoding: 0x0E212400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: size=0, Rm=1, Rd=0, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0E212400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_10_2400_0e3e2400() {
    // Encoding: 0x0E3E2400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rm=30, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E3E2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_11_2400_0e3f2400() {
    // Encoding: 0x0E3F2400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rm=31, Rn=0, U=0, Rd=0, size=0, Q=0
    let encoding: u32 = 0x0E3F2400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_12_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rm=0, Rd=0, size=0, Rn=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_13_2400_0e202420() {
    // Encoding: 0x0E202420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: U=0, size=0, Rm=0, Q=0, Rn=1, Rd=0
    let encoding: u32 = 0x0E202420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_14_2400_0e2027c0() {
    // Encoding: 0x0E2027C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Q=0, U=0, Rm=0, size=0, Rn=30, Rd=0
    let encoding: u32 = 0x0E2027C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_15_2400_0e2027e0() {
    // Encoding: 0x0E2027E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, Q=0, size=0, Rd=0, Rn=31, U=0
    let encoding: u32 = 0x0E2027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_16_2400_0e202400() {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Rn=0, Q=0, size=0, Rm=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_17_2400_0e202401() {
    // Encoding: 0x0E202401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rn=0, U=0, size=0, Rm=0, Rd=1, Q=0
    let encoding: u32 = 0x0E202401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_18_2400_0e20241e() {
    // Encoding: 0x0E20241E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: U=0, size=0, Rm=0, Q=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E20241E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_19_2400_0e20241f() {
    // Encoding: 0x0E20241F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rd=31, Rn=0, U=0, size=0, Rm=0, Q=0
    let encoding: u32 = 0x0E20241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_20_2400_0e212420() {
    // Encoding: 0x0E212420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rm=1, Rd=0, Q=0, U=0, size=0, Rn=1
    let encoding: u32 = 0x0E212420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_21_2400_0e3f27e0() {
    // Encoding: 0x0E3F27E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rm=31, size=0, Rd=0, Rn=31, Q=0, U=0
    let encoding: u32 = 0x0E3F27E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_22_2400_0e212401() {
    // Encoding: 0x0E212401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, size=0, Q=0, Rm=1, U=0
    let encoding: u32 = 0x0E212401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_23_2400_0e3f241f() {
    // Encoding: 0x0E3F241F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: U=0, Rn=0, Rm=31, size=0, Q=0, Rd=31
    let encoding: u32 = 0x0E3F241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_24_2400_0e202421() {
    // Encoding: 0x0E202421
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: Rm=0, size=0, Rn=1, Q=0, U=0, Rd=1
    let encoding: u32 = 0x0E202421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_combo_25_2400_0e2027ff() {
    // Encoding: 0x0E2027FF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: size=0, Q=0, U=0, Rm=0, Rn=31, Rd=31
    let encoding: u32 = 0x0E2027FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_q_0_size_variant_0_9216_0e602400()
{
    // Encoding: 0x0E602400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value Q = 0 (Size variant 0)
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, U=0, size=1
    let encoding: u32 = 0x0E602400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_q_1_size_variant_1_9216_4e602400()
{
    // Encoding: 0x4E602400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value Q = 1 (Size variant 1)
    // Fields: Rm=0, Rn=0, Rd=0, U=0, size=1, Q=1
    let encoding: u32 = 0x4E602400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_size_0_size_variant_0_9216_0e202400()
 {
    // Encoding: 0x0E202400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value size = 0 (Size variant 0)
    // Fields: Rn=0, U=0, Rd=0, size=0, Q=0, Rm=0
    let encoding: u32 = 0x0E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_size_1_size_variant_1_9216_0e602400()
 {
    // Encoding: 0x0E602400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value size = 1 (Size variant 1)
    // Fields: U=0, Rm=0, Rd=0, size=1, Rn=0, Q=0
    let encoding: u32 = 0x0E602400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_size_2_size_variant_2_9216_0ea02400()
 {
    // Encoding: 0x0EA02400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value size = 2 (Size variant 2)
    // Fields: Rn=0, Q=0, size=2, Rm=0, Rd=0, U=0
    let encoding: u32 = 0x0EA02400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_size_3_size_variant_3_9216_0ee02400()
 {
    // Encoding: 0x0EE02400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value size = 3 (Size variant 3)
    // Fields: Rm=0, size=3, U=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EE02400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_rn_31_stack_pointer_sp_may_require_alignment_9216_0e6027e0()
 {
    // Encoding: 0x0E6027E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, Q=0, size=1, Rm=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E6027E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_9216_0e60241f()
 {
    // Encoding: 0x0E60241F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Q=0, Rm=0, Rd=31, size=1, U=0
    let encoding: u32 = 0x0E60241F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_reg_write_0_0e202400() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int register write: SimdFromField("d")
    // Encoding: 0x0E202400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E202400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_sp_rn_0e2027e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int with Rn = SP (31)
    // Encoding: 0x0E2027E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2027E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_int
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_int_zr_rd_0e20241f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_int with Rd = ZR (31)
    // Encoding: 0x0E20241F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20241F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_disparate_add_sub_narrow Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_q_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Q = 0 (Min)
    // Fields: Q=0, size=0, U=0, Rd=0, Rm=0, o1=0, Rn=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_q_1_max_4000_4e204000() {
    // Encoding: 0x4E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Q = 1 (Max)
    // Fields: o1=0, U=0, Rm=0, Rn=0, Rd=0, size=0, Q=1
    let encoding: u32 = 0x4E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_u_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field U = 0 (Min)
    // Fields: size=0, Rm=0, U=0, Rd=0, Rn=0, o1=0, Q=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_u_1_max_4000_2e204000() {
    // Encoding: 0x2E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field U = 1 (Max)
    // Fields: Rd=0, o1=0, U=1, size=0, Q=0, Rm=0, Rn=0
    let encoding: u32 = 0x2E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_size_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field size = 0 (Min)
    // Fields: Rm=0, o1=0, Rn=0, Rd=0, U=0, Q=0, size=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_size_1_poweroftwo_4000_0e604000()
 {
    // Encoding: 0x0E604000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field size = 1 (PowerOfTwo)
    // Fields: size=1, U=0, Q=0, o1=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_size_2_poweroftwo_4000_0ea04000()
 {
    // Encoding: 0x0EA04000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field size = 2 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, o1=0, Q=0, U=0, size=2, Rm=0
    let encoding: u32 = 0x0EA04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_size_3_max_4000_0ee04000() {
    // Encoding: 0x0EE04000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field size = 3 (Max)
    // Fields: Q=0, Rm=0, o1=0, Rn=0, size=3, U=0, Rd=0
    let encoding: u32 = 0x0EE04000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rm_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rm = 0 (Min)
    // Fields: Rn=0, Q=0, U=0, size=0, Rm=0, o1=0, Rd=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rm_1_poweroftwo_4000_0e214000()
 {
    // Encoding: 0x0E214000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rm = 1 (PowerOfTwo)
    // Fields: o1=0, Rd=0, Rn=0, size=0, U=0, Rm=1, Q=0
    let encoding: u32 = 0x0E214000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rm_30_poweroftwominusone_4000_0e3e4000()
 {
    // Encoding: 0x0E3E4000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Q=0, U=0, size=0, Rm=30, o1=0, Rn=0
    let encoding: u32 = 0x0E3E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rm_31_max_4000_0e3f4000() {
    // Encoding: 0x0E3F4000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rm = 31 (Max)
    // Fields: Rd=0, Rm=31, size=0, Q=0, U=0, o1=0, Rn=0
    let encoding: u32 = 0x0E3F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_o1_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field o1 = 0 (Min)
    // Fields: Rm=0, size=0, o1=0, Rd=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_o1_1_max_4000_0e206000() {
    // Encoding: 0x0E206000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field o1 = 1 (Max)
    // Fields: Rn=0, o1=1, Rd=0, size=0, U=0, Rm=0, Q=0
    let encoding: u32 = 0x0E206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rn_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rn = 0 (Min)
    // Fields: Rd=0, U=0, Q=0, size=0, Rm=0, o1=0, Rn=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rn_1_poweroftwo_4000_0e204020()
 {
    // Encoding: 0x0E204020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0, size=0, Rm=0, U=0, Q=0, o1=0
    let encoding: u32 = 0x0E204020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rn_30_poweroftwominusone_4000_0e2043c0()
 {
    // Encoding: 0x0E2043C0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=30, o1=0, U=0, Q=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E2043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rn_31_max_4000_0e2043e0() {
    // Encoding: 0x0E2043E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rn = 31 (Max)
    // Fields: Rn=31, size=0, Rd=0, o1=0, Q=0, Rm=0, U=0
    let encoding: u32 = 0x0E2043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rd_0_min_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rd = 0 (Min)
    // Fields: o1=0, size=0, Rn=0, Rd=0, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rd_1_poweroftwo_4000_0e204001()
 {
    // Encoding: 0x0E204001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Rm=0, U=0, Q=0, size=0, o1=0, Rn=0
    let encoding: u32 = 0x0E204001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rd_30_poweroftwominusone_4000_0e20401e()
 {
    // Encoding: 0x0E20401E
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Rd=30, o1=0, size=0, U=0, Rn=0, Q=0
    let encoding: u32 = 0x0E20401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_field_rd_31_max_4000_0e20401f() {
    // Encoding: 0x0E20401F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field Rd = 31 (Max)
    // Fields: Q=0, U=0, Rm=0, size=0, o1=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E20401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_0_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rm=0, o1=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_1_4000_4e204000() {
    // Encoding: 0x4E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=1, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, size=0, Rm=0, Q=1, o1=0
    let encoding: u32 = 0x4E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_2_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, U=0, o1=0, Rn=0, size=0, Q=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_3_4000_2e204000() {
    // Encoding: 0x2E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=1, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Q=0, Rn=0, Rd=0, Rm=0, size=0, U=1
    let encoding: u32 = 0x2E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_4_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, o1=0, U=0, size=0, Q=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_5_4000_0e604000() {
    // Encoding: 0x0E604000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=1, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: size=1, Rm=0, Rn=0, Q=0, Rd=0, U=0, o1=0
    let encoding: u32 = 0x0E604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_6_4000_0ea04000() {
    // Encoding: 0x0EA04000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=2, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, size=2, Rn=0, o1=0, Rm=0, U=0
    let encoding: u32 = 0x0EA04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_7_4000_0ee04000() {
    // Encoding: 0x0EE04000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=3, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, o1=0, Rm=0, Rn=0, Rd=0, size=3
    let encoding: u32 = 0x0EE04000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_8_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, size=0, U=0, Rm=0, o1=0, Rn=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_9_4000_0e214000() {
    // Encoding: 0x0E214000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, o1=0, U=0, Q=0, Rm=1, Rn=0
    let encoding: u32 = 0x0E214000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_10_4000_0e3e4000() {
    // Encoding: 0x0E3E4000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=30, o1=0, Rn=0, Rd=0
    // Fields: size=0, o1=0, Rn=0, U=0, Rd=0, Q=0, Rm=30
    let encoding: u32 = 0x0E3E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_11_4000_0e3f4000() {
    // Encoding: 0x0E3F4000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=0, Rd=0
    // Fields: Q=0, o1=0, Rn=0, size=0, Rm=31, U=0, Rd=0
    let encoding: u32 = 0x0E3F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_12_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: U=0, o1=0, size=0, Rm=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_13_4000_0e206000() {
    // Encoding: 0x0E206000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=1, Rn=0, Rd=0
    // Fields: U=0, Rm=0, o1=1, Q=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_14_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, U=0, size=0, o1=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_15_4000_0e204020() {
    // Encoding: 0x0E204020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=1, Rd=0
    // Fields: size=0, Rn=1, Rd=0, Q=0, U=0, Rm=0, o1=0
    let encoding: u32 = 0x0E204020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_16_4000_0e2043c0() {
    // Encoding: 0x0E2043C0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=30, Rd=0
    // Fields: Rd=0, Q=0, Rm=0, U=0, o1=0, Rn=30, size=0
    let encoding: u32 = 0x0E2043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_17_4000_0e2043e0() {
    // Encoding: 0x0E2043E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=0
    // Fields: Q=0, Rd=0, o1=0, Rn=31, U=0, Rm=0, size=0
    let encoding: u32 = 0x0E2043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_18_4000_0e204000() {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, U=0, Rd=0, o1=0, size=0, Rn=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_19_4000_0e204001() {
    // Encoding: 0x0E204001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=1
    // Fields: Q=0, o1=0, Rn=0, Rm=0, U=0, Rd=1, size=0
    let encoding: u32 = 0x0E204001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_20_4000_0e20401e() {
    // Encoding: 0x0E20401E
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=30
    // Fields: U=0, Q=0, size=0, Rn=0, Rd=30, Rm=0, o1=0
    let encoding: u32 = 0x0E20401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_21_4000_0e20401f() {
    // Encoding: 0x0E20401F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=31
    // Fields: Rm=0, Rn=0, o1=0, size=0, Rd=31, Q=0, U=0
    let encoding: u32 = 0x0E20401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_22_4000_0e214020() {
    // Encoding: 0x0E214020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=1, Rd=0
    // Fields: Q=0, U=0, Rm=1, o1=0, Rd=0, Rn=1, size=0
    let encoding: u32 = 0x0E214020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_23_4000_0e3f43e0() {
    // Encoding: 0x0E3F43E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=31, Rd=0
    // Fields: Q=0, U=0, size=0, Rm=31, o1=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E3F43E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_24_4000_0e214001() {
    // Encoding: 0x0E214001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=0, Rd=1
    // Fields: Rd=1, size=0, o1=0, Rm=1, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0E214001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_25_4000_0e3f401f() {
    // Encoding: 0x0E3F401F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=0, Rd=31
    // Fields: Rm=31, o1=0, U=0, size=0, Rn=0, Rd=31, Q=0
    let encoding: u32 = 0x0E3F401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_26_4000_0e204021() {
    // Encoding: 0x0E204021
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=1, Rd=1
    // Fields: Q=0, U=0, Rm=0, Rn=1, Rd=1, o1=0, size=0
    let encoding: u32 = 0x0E204021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_combo_27_4000_0e2043ff() {
    // Encoding: 0x0E2043FF
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=31
    // Fields: Rd=31, Q=0, size=0, Rm=0, o1=0, U=0, Rn=31
    let encoding: u32 = 0x0E2043FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_q_0_size_variant_0_16384_0e604000()
 {
    // Encoding: 0x0E604000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value Q = 0 (Size variant 0)
    // Fields: Q=0, o1=0, Rn=0, Rd=0, size=1, Rm=0, U=0
    let encoding: u32 = 0x0E604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_q_1_size_variant_1_16384_4e604000()
 {
    // Encoding: 0x4E604000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value Q = 1 (Size variant 1)
    // Fields: Rd=0, o1=0, Rm=0, Q=1, U=0, size=1, Rn=0
    let encoding: u32 = 0x4E604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_size_0_size_variant_0_16384_0e204000()
 {
    // Encoding: 0x0E204000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value size = 0 (Size variant 0)
    // Fields: size=0, Rn=0, U=0, Rd=0, Rm=0, o1=0, Q=0
    let encoding: u32 = 0x0E204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_size_1_size_variant_1_16384_0e604000()
 {
    // Encoding: 0x0E604000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value size = 1 (Size variant 1)
    // Fields: U=0, size=1, Rm=0, Rn=0, o1=0, Q=0, Rd=0
    let encoding: u32 = 0x0E604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_size_2_size_variant_2_16384_0ea04000()
 {
    // Encoding: 0x0EA04000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value size = 2 (Size variant 2)
    // Fields: size=2, o1=0, Rn=0, U=0, Rd=0, Q=0, Rm=0
    let encoding: u32 = 0x0EA04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_size_3_size_variant_3_16384_0ee04000()
 {
    // Encoding: 0x0EE04000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value size = 3 (Size variant 3)
    // Fields: size=3, Q=0, U=0, o1=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EE04000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_rn_31_stack_pointer_sp_may_require_alignment_16384_0e6043e0()
 {
    // Encoding: 0x0E6043E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, Q=0, Rn=31, Rd=0, U=0, size=1, o1=0
    let encoding: u32 = 0x0E6043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_16384_0e60401f()
 {
    // Encoding: 0x0E60401F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, Rm=0, Rd=31, Rn=0, U=0, size=1, o1=0
    let encoding: u32 = 0x0E60401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_sp_rn_0e2043e0() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow with Rn = SP (31)
    // Encoding: 0x0E2043E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2043E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_zr_rd_0e20401f() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_narrow with Rd = ZR (31)
    // Encoding: 0x0E20401F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_unary_add_pairwise Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_q_0_min_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Q = 0 (Min)
    // Fields: Q=0, op=0, Rn=0, U=0, Rd=0, size=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_q_1_max_2800_4e202800() {
    // Encoding: 0x4E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Q = 1 (Max)
    // Fields: size=0, Rn=0, op=0, U=0, Q=1, Rd=0
    let encoding: u32 = 0x4E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_u_0_min_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field U = 0 (Min)
    // Fields: Q=0, op=0, Rn=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_u_1_max_2800_2e202800() {
    // Encoding: 0x2E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field U = 1 (Max)
    // Fields: op=0, U=1, Q=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_size_0_min_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field size = 0 (Min)
    // Fields: op=0, Rd=0, Rn=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_size_1_poweroftwo_2800_0e602800() {
    // Encoding: 0x0E602800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field size = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, Q=0, size=1, Rd=0, op=0
    let encoding: u32 = 0x0E602800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_size_2_poweroftwo_2800_0ea02800() {
    // Encoding: 0x0EA02800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field size = 2 (PowerOfTwo)
    // Fields: op=0, size=2, U=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0EA02800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_size_3_max_2800_0ee02800() {
    // Encoding: 0x0EE02800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field size = 3 (Max)
    // Fields: Rn=0, U=0, size=3, Q=0, op=0, Rd=0
    let encoding: u32 = 0x0EE02800;
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

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field op 14 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_op_0_min_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field op = 0 (Min)
    // Fields: size=0, Rd=0, U=0, Rn=0, Q=0, op=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field op 14 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_op_1_max_2800_0e206800() {
    // Encoding: 0x0E206800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field op = 1 (Max)
    // Fields: Rd=0, Q=0, size=0, U=0, op=1, Rn=0
    let encoding: u32 = 0x0E206800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rn_0_min_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rn = 0 (Min)
    // Fields: U=0, size=0, op=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rn_1_poweroftwo_2800_0e202820() {
    // Encoding: 0x0E202820
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, Rd=0, U=0, op=0, Rn=1, size=0
    let encoding: u32 = 0x0E202820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rn_30_poweroftwominusone_2800_0e202bc0()
{
    // Encoding: 0x0E202BC0
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=30, Rd=0, Q=0, size=0, op=0
    let encoding: u32 = 0x0E202BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rn_31_max_2800_0e202be0() {
    // Encoding: 0x0E202BE0
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rn = 31 (Max)
    // Fields: Q=0, Rn=31, size=0, U=0, op=0, Rd=0
    let encoding: u32 = 0x0E202BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rd_0_min_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rd = 0 (Min)
    // Fields: U=0, size=0, Q=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rd_1_poweroftwo_2800_0e202801() {
    // Encoding: 0x0E202801
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, U=0, op=0, Rn=0, size=0, Rd=1
    let encoding: u32 = 0x0E202801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rd_30_poweroftwominusone_2800_0e20281e()
{
    // Encoding: 0x0E20281E
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, U=0, size=0, op=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E20281E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_field_rd_31_max_2800_0e20281f() {
    // Encoding: 0x0E20281F
    // Test aarch64_vector_arithmetic_unary_add_pairwise field Rd = 31 (Max)
    // Fields: op=0, Rd=31, Rn=0, U=0, size=0, Q=0
    let encoding: u32 = 0x0E20281F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_0_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, U=0, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_1_2800_4e202800() {
    // Encoding: 0x4E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=1, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, U=0, op=0, Q=1, size=0
    let encoding: u32 = 0x4E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_2_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, op=0, Rn=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_3_2800_2e202800() {
    // Encoding: 0x2E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=1, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, op=0, Rn=0, Rd=0, U=1
    let encoding: u32 = 0x2E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_4_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, U=0, op=0, size=0, Rd=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_5_2800_0e602800() {
    // Encoding: 0x0E602800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=1, op=0, Rn=0, Rd=0
    // Fields: U=0, op=0, Q=0, Rn=0, size=1, Rd=0
    let encoding: u32 = 0x0E602800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_6_2800_0ea02800() {
    // Encoding: 0x0EA02800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=2, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, op=0, Rn=0, size=2, Rd=0
    let encoding: u32 = 0x0EA02800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_7_2800_0ee02800() {
    // Encoding: 0x0EE02800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=3, op=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Q=0, size=3, Rd=0, op=0
    let encoding: u32 = 0x0EE02800;
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

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_8_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, Rd=0, size=0, U=0, Q=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_9_2800_0e206800() {
    // Encoding: 0x0E206800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=1, Rn=0, Rd=0
    // Fields: op=1, Rd=0, Rn=0, size=0, Q=0, U=0
    let encoding: u32 = 0x0E206800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_10_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rn=0, Q=0, size=0, op=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_11_2800_0e202820() {
    // Encoding: 0x0E202820
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=1, Rd=0
    // Fields: Rd=0, op=0, size=0, Q=0, U=0, Rn=1
    let encoding: u32 = 0x0E202820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_12_2800_0e202bc0() {
    // Encoding: 0x0E202BC0
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=30, Rd=0
    // Fields: size=0, U=0, op=0, Rn=30, Rd=0, Q=0
    let encoding: u32 = 0x0E202BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_13_2800_0e202be0() {
    // Encoding: 0x0E202BE0
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=31, Rd=0
    // Fields: Rd=0, size=0, Rn=31, Q=0, U=0, op=0
    let encoding: u32 = 0x0E202BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_14_2800_0e202800() {
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_15_2800_0e202801() {
    // Encoding: 0x0E202801
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=1
    // Fields: Q=0, Rd=1, Rn=0, op=0, U=0, size=0
    let encoding: u32 = 0x0E202801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_16_2800_0e20281e() {
    // Encoding: 0x0E20281E
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=30
    // Fields: Q=0, Rd=30, op=0, U=0, Rn=0, size=0
    let encoding: u32 = 0x0E20281E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_17_2800_0e20281f() {
    // Encoding: 0x0E20281F
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=31
    // Fields: size=0, U=0, Q=0, op=0, Rd=31, Rn=0
    let encoding: u32 = 0x0E20281F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_18_2800_0e202821() {
    // Encoding: 0x0E202821
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=1, Rd=1
    // Fields: Rd=1, U=0, op=0, Rn=1, Q=0, size=0
    let encoding: u32 = 0x0E202821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_combo_19_2800_0e202bff() {
    // Encoding: 0x0E202BFF
    // Test aarch64_vector_arithmetic_unary_add_pairwise field combination: Q=0, U=0, size=0, op=0, Rn=31, Rd=31
    // Fields: Q=0, size=0, Rn=31, U=0, op=0, Rd=31
    let encoding: u32 = 0x0E202BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_q_0_size_variant_0_10240_0e602800() {
    // Encoding: 0x0E602800
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value Q = 0 (Size variant 0)
    // Fields: size=1, op=0, Rd=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0E602800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_q_1_size_variant_1_10240_4e602800() {
    // Encoding: 0x4E602800
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value Q = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, Q=1, op=0, U=0, size=1
    let encoding: u32 = 0x4E602800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_size_0_size_variant_0_10240_0e202800()
{
    // Encoding: 0x0E202800
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value size = 0 (Size variant 0)
    // Fields: Q=0, U=0, Rn=0, size=0, op=0, Rd=0
    let encoding: u32 = 0x0E202800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_size_1_size_variant_1_10240_0e602800()
{
    // Encoding: 0x0E602800
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value size = 1 (Size variant 1)
    // Fields: Q=0, Rd=0, Rn=0, U=0, op=0, size=1
    let encoding: u32 = 0x0E602800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_size_2_size_variant_2_10240_0ea02800()
{
    // Encoding: 0x0EA02800
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value size = 2 (Size variant 2)
    // Fields: Rn=0, U=0, Rd=0, Q=0, size=2, op=0
    let encoding: u32 = 0x0EA02800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_size_3_size_variant_3_10240_0ee02800()
{
    // Encoding: 0x0EE02800
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value size = 3 (Size variant 3)
    // Fields: Rn=0, Q=0, Rd=0, size=3, U=0, op=0
    let encoding: u32 = 0x0EE02800;
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

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_rn_31_stack_pointer_sp_may_require_alignment_10240_0e602be0()
 {
    // Encoding: 0x0E602BE0
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, op=0, Q=0, Rn=31, Rd=0, U=0
    let encoding: u32 = 0x0E602BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_10240_0e60281f()
 {
    // Encoding: 0x0E60281F
    // Test aarch64_vector_arithmetic_unary_add_pairwise special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, Q=0, size=1, op=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E60281F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_reg_write_0_0e202800() {
    // Test aarch64_vector_arithmetic_unary_add_pairwise register write: SimdFromField("d")
    // Encoding: 0x0E202800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E202800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_sp_rn_0e202be0() {
    // Test aarch64_vector_arithmetic_unary_add_pairwise with Rn = SP (31)
    // Encoding: 0x0E202BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E202BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_pairwise
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_pairwise_zr_rd_0e20281f() {
    // Test aarch64_vector_arithmetic_unary_add_pairwise with Rd = ZR (31)
    // Encoding: 0x0E20281F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20281F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_u_0_min_8400_5e208400()
 {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field U = 0 (Min)
    // Fields: Rm=0, U=0, Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_u_1_max_8400_7e208400()
 {
    // Encoding: 0x7E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field U = 1 (Max)
    // Fields: Rm=0, U=1, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x7E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_size_0_min_8400_5e208400()
 {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field size = 0 (Min)
    // Fields: U=0, size=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_size_1_poweroftwo_8400_5e608400()
 {
    // Encoding: 0x5E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field size = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, size=1, Rd=0, Rm=0
    let encoding: u32 = 0x5E608400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_size_2_poweroftwo_8400_5ea08400()
 {
    // Encoding: 0x5EA08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field size = 2 (PowerOfTwo)
    // Fields: size=2, U=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x5EA08400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_size_3_max_8400_5ee08400()
 {
    // Encoding: 0x5EE08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field size = 3 (Max)
    // Fields: size=3, U=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x5EE08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rm_0_min_8400_5e208400()
 {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rm = 0 (Min)
    // Fields: Rm=0, size=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rm_1_poweroftwo_8400_5e218400()
 {
    // Encoding: 0x5E218400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rm = 1 (PowerOfTwo)
    // Fields: U=0, size=0, Rm=1, Rn=0, Rd=0
    let encoding: u32 = 0x5E218400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rm_30_poweroftwominusone_8400_5e3e8400()
 {
    // Encoding: 0x5E3E8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rd=0, Rm=30, U=0
    let encoding: u32 = 0x5E3E8400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rm_31_max_8400_5e3f8400()
 {
    // Encoding: 0x5E3F8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rm = 31 (Max)
    // Fields: U=0, Rd=0, Rn=0, Rm=31, size=0
    let encoding: u32 = 0x5E3F8400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rn_0_min_8400_5e208400()
 {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rn = 0 (Min)
    // Fields: Rd=0, Rm=0, U=0, size=0, Rn=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rn_1_poweroftwo_8400_5e208420()
 {
    // Encoding: 0x5E208420
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, size=0, Rm=0, U=0, Rn=1
    let encoding: u32 = 0x5E208420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rn_30_poweroftwominusone_8400_5e2087c0()
 {
    // Encoding: 0x5E2087C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Rn=30, Rd=0, size=0, U=0
    let encoding: u32 = 0x5E2087C0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rn_31_max_8400_5e2087e0()
 {
    // Encoding: 0x5E2087E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rn = 31 (Max)
    // Fields: Rd=0, Rm=0, U=0, size=0, Rn=31
    let encoding: u32 = 0x5E2087E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rd_0_min_8400_5e208400()
 {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rd = 0 (Min)
    // Fields: Rd=0, U=0, Rn=0, size=0, Rm=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rd_1_poweroftwo_8400_5e208401()
 {
    // Encoding: 0x5E208401
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rd = 1 (PowerOfTwo)
    // Fields: size=0, U=0, Rm=0, Rd=1, Rn=0
    let encoding: u32 = 0x5E208401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rd_30_poweroftwominusone_8400_5e20841e()
 {
    // Encoding: 0x5E20841E
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rd=30, size=0, Rn=0, Rm=0
    let encoding: u32 = 0x5E20841E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_field_rd_31_max_8400_5e20841f()
 {
    // Encoding: 0x5E20841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field Rd = 31 (Max)
    // Fields: Rm=0, Rd=31, U=0, size=0, Rn=0
    let encoding: u32 = 0x5E20841F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_0_8400_5e208400() {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_1_8400_7e208400() {
    // Encoding: 0x7E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, U=1, size=0, Rn=0
    let encoding: u32 = 0x7E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_2_8400_5e208400() {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_3_8400_5e608400() {
    // Encoding: 0x5E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Rm=0, size=1, Rn=0
    let encoding: u32 = 0x5E608400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_4_8400_5ea08400() {
    // Encoding: 0x5EA08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Rn=0, Rm=0, size=2
    let encoding: u32 = 0x5EA08400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_5_8400_5ee08400() {
    // Encoding: 0x5EE08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, size=3, Rm=0, U=0
    let encoding: u32 = 0x5EE08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_6_8400_5e208400() {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_7_8400_5e218400() {
    // Encoding: 0x5E218400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: size=0, Rd=0, Rm=1, U=0, Rn=0
    let encoding: u32 = 0x5E218400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_8_8400_5e3e8400() {
    // Encoding: 0x5E3E8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, size=0, Rm=30
    let encoding: u32 = 0x5E3E8400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_9_8400_5e3f8400() {
    // Encoding: 0x5E3F8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rm=31, Rd=0, U=0
    let encoding: u32 = 0x5E3F8400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_10_8400_5e208400() {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, U=0, Rd=0, size=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_11_8400_5e208420() {
    // Encoding: 0x5E208420
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: U=0, Rd=0, Rm=0, Rn=1, size=0
    let encoding: u32 = 0x5E208420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_12_8400_5e2087c0() {
    // Encoding: 0x5E2087C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: U=0, size=0, Rm=0, Rn=30, Rd=0
    let encoding: u32 = 0x5E2087C0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_13_8400_5e2087e0() {
    // Encoding: 0x5E2087E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, U=0, Rd=0, Rn=31, size=0
    let encoding: u32 = 0x5E2087E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_14_8400_5e208400() {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_15_8400_5e208401() {
    // Encoding: 0x5E208401
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rn=0, U=0, size=0, Rm=0, Rd=1
    let encoding: u32 = 0x5E208401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_16_8400_5e20841e() {
    // Encoding: 0x5E20841E
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: size=0, Rm=0, Rn=0, U=0, Rd=30
    let encoding: u32 = 0x5E20841E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_17_8400_5e20841f() {
    // Encoding: 0x5E20841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: U=0, Rn=0, Rd=31, Rm=0, size=0
    let encoding: u32 = 0x5E20841F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_18_8400_5e218420() {
    // Encoding: 0x5E218420
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rd=0, U=0, size=0, Rm=1, Rn=1
    let encoding: u32 = 0x5E218420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_19_8400_5e3f87e0() {
    // Encoding: 0x5E3F87E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rm=31, Rd=0, U=0, size=0, Rn=31
    let encoding: u32 = 0x5E3F87E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_20_8400_5e218401() {
    // Encoding: 0x5E218401
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: size=0, Rm=1, Rn=0, Rd=1, U=0
    let encoding: u32 = 0x5E218401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_21_8400_5e3f841f() {
    // Encoding: 0x5E3F841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: size=0, Rn=0, Rd=31, Rm=31, U=0
    let encoding: u32 = 0x5E3F841F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_22_8400_5e208421() {
    // Encoding: 0x5E208421
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: size=0, Rn=1, Rd=1, Rm=0, U=0
    let encoding: u32 = 0x5E208421;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_combo_23_8400_5e2087ff() {
    // Encoding: 0x5E2087FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: size=0, U=0, Rm=0, Rd=31, Rn=31
    let encoding: u32 = 0x5E2087FF;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_special_size_0_size_variant_0_33792_5e208400()
 {
    // Encoding: 0x5E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd special value size = 0 (Size variant 0)
    // Fields: size=0, U=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_special_size_1_size_variant_1_33792_5e608400()
 {
    // Encoding: 0x5E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd special value size = 1 (Size variant 1)
    // Fields: size=1, U=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E608400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_special_size_2_size_variant_2_33792_5ea08400()
 {
    // Encoding: 0x5EA08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd special value size = 2 (Size variant 2)
    // Fields: Rd=0, U=0, Rn=0, size=2, Rm=0
    let encoding: u32 = 0x5EA08400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_special_size_3_size_variant_3_33792_5ee08400()
 {
    // Encoding: 0x5EE08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd special value size = 3 (Size variant 3)
    // Fields: Rn=0, Rm=0, Rd=0, size=3, U=0
    let encoding: u32 = 0x5EE08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_33792_5e6087e0()
 {
    // Encoding: 0x5E6087E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, Rm=0, size=1, Rn=31, U=0
    let encoding: u32 = 0x5E6087E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_33792_5e60841f()
 {
    // Encoding: 0x5E60841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, Rm=0, Rn=0, Rd=31, size=1
    let encoding: u32 = 0x5E60841F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_q_0_min_8400_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Q = 0 (Min)
    // Fields: Rm=0, Rd=0, size=0, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_q_1_max_8400_4e208400()
 {
    // Encoding: 0x4E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Q = 1 (Max)
    // Fields: Rn=0, size=0, Rd=0, U=0, Rm=0, Q=1
    let encoding: u32 = 0x4E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_u_0_min_8400_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field U = 0 (Min)
    // Fields: U=0, size=0, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_u_1_max_8400_2e208400()
 {
    // Encoding: 0x2E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field U = 1 (Max)
    // Fields: Q=0, Rn=0, Rd=0, U=1, Rm=0, size=0
    let encoding: u32 = 0x2E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_size_0_min_8400_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field size = 0 (Min)
    // Fields: U=0, size=0, Rm=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_size_1_poweroftwo_8400_0e608400()
 {
    // Encoding: 0x0E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field size = 1 (PowerOfTwo)
    // Fields: U=0, size=1, Rm=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0E608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_size_2_poweroftwo_8400_0ea08400()
 {
    // Encoding: 0x0EA08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field size = 2 (PowerOfTwo)
    // Fields: Rn=0, U=0, size=2, Rd=0, Q=0, Rm=0
    let encoding: u32 = 0x0EA08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_size_3_max_8400_0ee08400()
 {
    // Encoding: 0x0EE08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field size = 3 (Max)
    // Fields: Q=0, Rn=0, U=0, size=3, Rm=0, Rd=0
    let encoding: u32 = 0x0EE08400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rm_0_min_8400_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rm = 0 (Min)
    // Fields: U=0, Rn=0, Q=0, size=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rm_1_poweroftwo_8400_0e218400()
 {
    // Encoding: 0x0E218400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, Rm=1, Q=0, Rd=0, size=0
    let encoding: u32 = 0x0E218400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rm_30_poweroftwominusone_8400_0e3e8400()
 {
    // Encoding: 0x0E3E8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, size=0, Rn=0, Rm=30, Q=0, Rd=0
    let encoding: u32 = 0x0E3E8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rm_31_max_8400_0e3f8400()
 {
    // Encoding: 0x0E3F8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rm = 31 (Max)
    // Fields: Rm=31, Rd=0, Q=0, Rn=0, U=0, size=0
    let encoding: u32 = 0x0E3F8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rn_0_min_8400_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rn = 0 (Min)
    // Fields: Q=0, Rd=0, Rn=0, size=0, U=0, Rm=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rn_1_poweroftwo_8400_0e208420()
 {
    // Encoding: 0x0E208420
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, size=0, Q=0, Rn=1, Rd=0, Rm=0
    let encoding: u32 = 0x0E208420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rn_30_poweroftwominusone_8400_0e2087c0()
 {
    // Encoding: 0x0E2087C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rm=0, Q=0, Rn=30, Rd=0, U=0
    let encoding: u32 = 0x0E2087C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rn_31_max_8400_0e2087e0()
 {
    // Encoding: 0x0E2087E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, Rm=0, size=0, Q=0, U=0
    let encoding: u32 = 0x0E2087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rd_0_min_8400_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rd = 0 (Min)
    // Fields: size=0, Rm=0, Rn=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rd_1_poweroftwo_8400_0e208401()
 {
    // Encoding: 0x0E208401
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, U=0, Rm=0, Rn=0, Rd=1, size=0
    let encoding: u32 = 0x0E208401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rd_30_poweroftwominusone_8400_0e20841e()
 {
    // Encoding: 0x0E20841E
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, U=0, Rd=30, Rm=0, Q=0
    let encoding: u32 = 0x0E20841E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_field_rd_31_max_8400_0e20841f()
 {
    // Encoding: 0x0E20841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field Rd = 31 (Max)
    // Fields: size=0, U=0, Rn=0, Rd=31, Q=0, Rm=0
    let encoding: u32 = 0x0E20841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_0_8400_0e208400() {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, Q=0, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_1_8400_4e208400() {
    // Encoding: 0x4E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=1, Rm=0, Rn=0, size=0, U=0, Rd=0
    let encoding: u32 = 0x4E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_2_8400_0e208400() {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Q=0, U=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_3_8400_2e208400() {
    // Encoding: 0x2E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=1, Q=0, Rm=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_4_8400_0e208400() {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rm=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_5_8400_0e608400() {
    // Encoding: 0x0E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: size=1, Rn=0, Q=0, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x0E608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_6_8400_0ea08400() {
    // Encoding: 0x0EA08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Q=0, size=2, Rn=0, Rd=0
    let encoding: u32 = 0x0EA08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_7_8400_0ee08400() {
    // Encoding: 0x0EE08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Rm=0, size=3, U=0, Q=0
    let encoding: u32 = 0x0EE08400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_8_8400_0e208400() {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, U=0, Rm=0, size=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_9_8400_0e218400() {
    // Encoding: 0x0E218400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, size=0, U=0, Rm=1, Rd=0
    let encoding: u32 = 0x0E218400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_10_8400_0e3e8400() {
    // Encoding: 0x0E3E8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rm=30, size=0, U=0, Rn=0
    let encoding: u32 = 0x0E3E8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_11_8400_0e3f8400() {
    // Encoding: 0x0E3F8400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Q=0, U=0, size=0, Rn=0, Rd=0, Rm=31
    let encoding: u32 = 0x0E3F8400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_12_8400_0e208400() {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rn=0, U=0, Q=0, Rm=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_13_8400_0e208420() {
    // Encoding: 0x0E208420
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: U=0, size=0, Q=0, Rm=0, Rn=1, Rd=0
    let encoding: u32 = 0x0E208420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_14_8400_0e2087c0() {
    // Encoding: 0x0E2087C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: U=0, Q=0, Rn=30, Rm=0, Rd=0, size=0
    let encoding: u32 = 0x0E2087C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_15_8400_0e2087e0() {
    // Encoding: 0x0E2087E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, U=0, Rm=0, size=0, Q=0
    let encoding: u32 = 0x0E2087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_16_8400_0e208400() {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Rd=0, U=0, Q=0, Rn=0, Rm=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_17_8400_0e208401() {
    // Encoding: 0x0E208401
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: size=0, Q=0, U=0, Rm=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E208401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_18_8400_0e20841e() {
    // Encoding: 0x0E20841E
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Rm=0, U=0, Rn=0, Rd=30, Q=0, size=0
    let encoding: u32 = 0x0E20841E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_19_8400_0e20841f() {
    // Encoding: 0x0E20841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: size=0, Rn=0, U=0, Q=0, Rd=31, Rm=0
    let encoding: u32 = 0x0E20841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_20_8400_0e218420() {
    // Encoding: 0x0E218420
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rn=1, Rd=0, Q=0, U=0, size=0, Rm=1
    let encoding: u32 = 0x0E218420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_21_8400_0e3f87e0() {
    // Encoding: 0x0E3F87E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rm=31, Q=0, Rd=0, Rn=31, U=0, size=0
    let encoding: u32 = 0x0E3F87E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_22_8400_0e218401() {
    // Encoding: 0x0E218401
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: size=0, U=0, Rn=0, Rm=1, Rd=1, Q=0
    let encoding: u32 = 0x0E218401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_23_8400_0e3f841f() {
    // Encoding: 0x0E3F841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Rn=0, Rm=31, Q=0, U=0, Rd=31, size=0
    let encoding: u32 = 0x0E3F841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_24_8400_0e208421() {
    // Encoding: 0x0E208421
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: Q=0, Rn=1, Rd=1, Rm=0, size=0, U=0
    let encoding: u32 = 0x0E208421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_combo_25_8400_0e2087ff() {
    // Encoding: 0x0E2087FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Rd=31, Rm=0, Q=0, size=0, U=0, Rn=31
    let encoding: u32 = 0x0E2087FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_q_0_size_variant_0_33792_0e608400()
 {
    // Encoding: 0x0E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value Q = 0 (Size variant 0)
    // Fields: Rd=0, Rm=0, U=0, Rn=0, size=1, Q=0
    let encoding: u32 = 0x0E608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_q_1_size_variant_1_33792_4e608400()
 {
    // Encoding: 0x4E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value Q = 1 (Size variant 1)
    // Fields: size=1, Rm=0, Rn=0, Rd=0, Q=1, U=0
    let encoding: u32 = 0x4E608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_size_0_size_variant_0_33792_0e208400()
 {
    // Encoding: 0x0E208400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value size = 0 (Size variant 0)
    // Fields: U=0, size=0, Q=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_size_1_size_variant_1_33792_0e608400()
 {
    // Encoding: 0x0E608400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value size = 1 (Size variant 1)
    // Fields: Rn=0, size=1, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_size_2_size_variant_2_33792_0ea08400()
 {
    // Encoding: 0x0EA08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value size = 2 (Size variant 2)
    // Fields: Rd=0, Rn=0, Q=0, Rm=0, U=0, size=2
    let encoding: u32 = 0x0EA08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_size_3_size_variant_3_33792_0ee08400()
 {
    // Encoding: 0x0EE08400
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value size = 3 (Size variant 3)
    // Fields: U=0, Q=0, size=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0EE08400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_rn_31_stack_pointer_sp_may_require_alignment_33792_0e6087e0()
 {
    // Encoding: 0x0E6087E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Q=0, U=0, size=1, Rm=0, Rd=0
    let encoding: u32 = 0x0E6087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_33792_0e60841f()
 {
    // Encoding: 0x0E60841F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, U=0, size=1, Rn=0, Q=0, Rd=31
    let encoding: u32 = 0x0E60841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_reg_write_0_5e208400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd register write: SimdFromField("d")
    // Encoding: 0x5E208400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E208400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_sp_rn_5e2087e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd with Rn = SP (31)
    // Encoding: 0x5E2087E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E2087E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_zr_rd_5e20841f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd with Rd = ZR (31)
    // Encoding: 0x5E20841F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20841F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_reg_write_0_0e208400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd register write: SimdFromField("d")
    // Encoding: 0x0E208400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E208400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_sp_rn_0e2087e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd with Rn = SP (31)
    // Encoding: 0x0E2087E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2087E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_zr_rd_0e20841f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd with Rd = ZR (31)
    // Encoding: 0x0E20841F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20841F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_disparate_add_sub_wide Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_q_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Q = 0 (Min)
    // Fields: Rn=0, size=0, Rm=0, U=0, Q=0, o1=0, Rd=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_q_1_max_1000_4e201000() {
    // Encoding: 0x4E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Q = 1 (Max)
    // Fields: Q=1, U=0, Rm=0, o1=0, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x4E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_u_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field U = 0 (Min)
    // Fields: Q=0, Rd=0, U=0, o1=0, Rn=0, Rm=0, size=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_u_1_max_1000_2e201000() {
    // Encoding: 0x2E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field U = 1 (Max)
    // Fields: size=0, U=1, Rn=0, o1=0, Q=0, Rm=0, Rd=0
    let encoding: u32 = 0x2E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_size_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field size = 0 (Min)
    // Fields: Rm=0, size=0, o1=0, Rd=0, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_size_1_poweroftwo_1000_0e601000()
 {
    // Encoding: 0x0E601000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field size = 1 (PowerOfTwo)
    // Fields: size=1, Rm=0, o1=0, Rd=0, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_size_2_poweroftwo_1000_0ea01000()
 {
    // Encoding: 0x0EA01000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field size = 2 (PowerOfTwo)
    // Fields: U=0, Rm=0, Q=0, o1=0, size=2, Rn=0, Rd=0
    let encoding: u32 = 0x0EA01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_size_3_max_1000_0ee01000() {
    // Encoding: 0x0EE01000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field size = 3 (Max)
    // Fields: Q=0, U=0, Rn=0, Rd=0, size=3, Rm=0, o1=0
    let encoding: u32 = 0x0EE01000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rm_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rm = 0 (Min)
    // Fields: Rm=0, U=0, size=0, o1=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rm_1_poweroftwo_1000_0e211000()
 {
    // Encoding: 0x0E211000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, Q=0, U=0, size=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E211000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rm_30_poweroftwominusone_1000_0e3e1000()
 {
    // Encoding: 0x0E3E1000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Q=0, U=0, Rm=30, o1=0, Rn=0, size=0
    let encoding: u32 = 0x0E3E1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rm_31_max_1000_0e3f1000() {
    // Encoding: 0x0E3F1000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rm = 31 (Max)
    // Fields: Q=0, U=0, size=0, o1=0, Rn=0, Rm=31, Rd=0
    let encoding: u32 = 0x0E3F1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_o1_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field o1 = 0 (Min)
    // Fields: o1=0, Rn=0, size=0, U=0, Rm=0, Rd=0, Q=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_o1_1_max_1000_0e203000() {
    // Encoding: 0x0E203000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field o1 = 1 (Max)
    // Fields: o1=1, Rd=0, U=0, Q=0, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x0E203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rn_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rn = 0 (Min)
    // Fields: Rm=0, o1=0, U=0, Q=0, Rn=0, size=0, Rd=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rn_1_poweroftwo_1000_0e201020()
 {
    // Encoding: 0x0E201020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rn = 1 (PowerOfTwo)
    // Fields: o1=0, Rd=0, U=0, Q=0, Rm=0, Rn=1, size=0
    let encoding: u32 = 0x0E201020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rn_30_poweroftwominusone_1000_0e2013c0()
 {
    // Encoding: 0x0E2013C0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Rd=0, Rn=30, Rm=0, U=0, Q=0, size=0
    let encoding: u32 = 0x0E2013C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rn_31_max_1000_0e2013e0() {
    // Encoding: 0x0E2013E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rn = 31 (Max)
    // Fields: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E2013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rd_0_min_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rd = 0 (Min)
    // Fields: U=0, size=0, Rm=0, Rd=0, Q=0, Rn=0, o1=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rd_1_poweroftwo_1000_0e201001()
 {
    // Encoding: 0x0E201001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rd = 1 (PowerOfTwo)
    // Fields: o1=0, U=0, Rd=1, Rm=0, size=0, Rn=0, Q=0
    let encoding: u32 = 0x0E201001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rd_30_poweroftwominusone_1000_0e20101e()
 {
    // Encoding: 0x0E20101E
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Rn=0, Rd=30, U=0, Rm=0, Q=0, size=0
    let encoding: u32 = 0x0E20101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_field_rd_31_max_1000_0e20101f() {
    // Encoding: 0x0E20101F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field Rd = 31 (Max)
    // Fields: Rd=31, Q=0, U=0, o1=0, Rn=0, size=0, Rm=0
    let encoding: u32 = 0x0E20101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_0_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, U=0, size=0, Rm=0, o1=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_1_1000_4e201000() {
    // Encoding: 0x4E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=1, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Rd=0, Rm=0, size=0, Q=1, Rn=0, U=0
    let encoding: u32 = 0x4E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_2_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, U=0, size=0, o1=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_3_1000_2e201000() {
    // Encoding: 0x2E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=1, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, U=1, size=0, Rm=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_4_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: size=0, o1=0, Q=0, Rn=0, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_5_1000_0e601000() {
    // Encoding: 0x0E601000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=1, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, o1=0, Q=0, Rm=0, Rn=0, size=1, U=0
    let encoding: u32 = 0x0E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_6_1000_0ea01000() {
    // Encoding: 0x0EA01000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=2, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, size=2, o1=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EA01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_7_1000_0ee01000() {
    // Encoding: 0x0EE01000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=3, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: U=0, o1=0, Rn=0, Q=0, Rd=0, Rm=0, size=3
    let encoding: u32 = 0x0EE01000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_8_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Rm=0, Rn=0, Rd=0, U=0, Q=0, size=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_9_1000_0e211000() {
    // Encoding: 0x0E211000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=0, Rd=0
    // Fields: Rm=1, Q=0, U=0, o1=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E211000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_10_1000_0e3e1000() {
    // Encoding: 0x0E3E1000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=30, o1=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rm=30, o1=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E3E1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_11_1000_0e3f1000() {
    // Encoding: 0x0E3F1000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, size=0, Q=0, U=0, Rm=31, o1=0
    let encoding: u32 = 0x0E3F1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_12_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: size=0, Rm=0, U=0, Q=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_13_1000_0e203000() {
    // Encoding: 0x0E203000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=1, Rn=0, Rd=0
    // Fields: o1=1, size=0, U=0, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_14_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Q=0, U=0, size=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_15_1000_0e201020() {
    // Encoding: 0x0E201020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=1, Rd=0
    // Fields: Rd=0, Rm=0, size=0, o1=0, Q=0, U=0, Rn=1
    let encoding: u32 = 0x0E201020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_16_1000_0e2013c0() {
    // Encoding: 0x0E2013C0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=30, Rd=0
    // Fields: o1=0, Rn=30, Rd=0, Q=0, size=0, U=0, Rm=0
    let encoding: u32 = 0x0E2013C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_17_1000_0e2013e0() {
    // Encoding: 0x0E2013E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=0
    // Fields: Q=0, o1=0, Rn=31, Rm=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x0E2013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_18_1000_0e201000() {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=0
    // Fields: size=0, Rm=0, U=0, Rn=0, Rd=0, o1=0, Q=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_19_1000_0e201001() {
    // Encoding: 0x0E201001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=1
    // Fields: o1=0, Q=0, Rn=0, size=0, Rm=0, Rd=1, U=0
    let encoding: u32 = 0x0E201001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_20_1000_0e20101e() {
    // Encoding: 0x0E20101E
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=30
    // Fields: size=0, Rd=30, U=0, Q=0, Rm=0, o1=0, Rn=0
    let encoding: u32 = 0x0E20101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_21_1000_0e20101f() {
    // Encoding: 0x0E20101F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=0, Rd=31
    // Fields: size=0, o1=0, Rm=0, U=0, Rn=0, Q=0, Rd=31
    let encoding: u32 = 0x0E20101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_22_1000_0e211020() {
    // Encoding: 0x0E211020
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=1, Rd=0
    // Fields: size=0, Q=0, o1=0, Rm=1, Rn=1, Rd=0, U=0
    let encoding: u32 = 0x0E211020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_23_1000_0e3f13e0() {
    // Encoding: 0x0E3F13E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=31, Rd=0
    // Fields: Q=0, size=0, Rm=31, U=0, o1=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E3F13E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_24_1000_0e211001() {
    // Encoding: 0x0E211001
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=1, o1=0, Rn=0, Rd=1
    // Fields: size=0, o1=0, Rn=0, Q=0, Rd=1, U=0, Rm=1
    let encoding: u32 = 0x0E211001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_25_1000_0e3f101f() {
    // Encoding: 0x0E3F101F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=31, o1=0, Rn=0, Rd=31
    // Fields: U=0, Rd=31, Q=0, Rn=0, size=0, Rm=31, o1=0
    let encoding: u32 = 0x0E3F101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_26_1000_0e201021() {
    // Encoding: 0x0E201021
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=1, Rd=1
    // Fields: Q=0, U=0, Rm=0, o1=0, size=0, Rn=1, Rd=1
    let encoding: u32 = 0x0E201021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_combo_27_1000_0e2013ff() {
    // Encoding: 0x0E2013FF
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide field combination: Q=0, U=0, size=0, Rm=0, o1=0, Rn=31, Rd=31
    // Fields: size=0, Q=0, Rm=0, o1=0, Rn=31, Rd=31, U=0
    let encoding: u32 = 0x0E2013FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_q_0_size_variant_0_4096_0e601000()
 {
    // Encoding: 0x0E601000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value Q = 0 (Size variant 0)
    // Fields: Q=0, o1=0, Rm=0, Rn=0, size=1, Rd=0, U=0
    let encoding: u32 = 0x0E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_q_1_size_variant_1_4096_4e601000()
 {
    // Encoding: 0x4E601000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value Q = 1 (Size variant 1)
    // Fields: Rd=0, Q=1, U=0, o1=0, Rm=0, size=1, Rn=0
    let encoding: u32 = 0x4E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_size_0_size_variant_0_4096_0e201000()
 {
    // Encoding: 0x0E201000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value size = 0 (Size variant 0)
    // Fields: Q=0, Rm=0, Rn=0, Rd=0, U=0, size=0, o1=0
    let encoding: u32 = 0x0E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_size_1_size_variant_1_4096_0e601000()
 {
    // Encoding: 0x0E601000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value size = 1 (Size variant 1)
    // Fields: size=1, U=0, Q=0, o1=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_size_2_size_variant_2_4096_0ea01000()
 {
    // Encoding: 0x0EA01000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value size = 2 (Size variant 2)
    // Fields: Rd=0, Q=0, size=2, U=0, Rn=0, Rm=0, o1=0
    let encoding: u32 = 0x0EA01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_size_3_size_variant_3_4096_0ee01000()
 {
    // Encoding: 0x0EE01000
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value size = 3 (Size variant 3)
    // Fields: Q=0, Rm=0, size=3, U=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EE01000;
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

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_rn_31_stack_pointer_sp_may_require_alignment_4096_0e6013e0()
 {
    // Encoding: 0x0E6013E0
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rm=0, U=0, o1=0, Q=0, size=1, Rd=0
    let encoding: u32 = 0x0E6013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_4096_0e60101f()
 {
    // Encoding: 0x0E60101F
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Q=0, o1=0, Rm=0, size=1, U=0, Rd=31
    let encoding: u32 = 0x0E60101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_reg_write_0_0e201000() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide register write: SimdFromField("d")
    // Encoding: 0x0E201000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E201000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_sp_rn_0e2013e0() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide with Rn = SP (31)
    // Encoding: 0x0E2013E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2013E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_disparate_add_sub_wide
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_disparate_add_sub_wide_zr_rd_0e20101f() {
    // Test aarch64_vector_arithmetic_binary_disparate_add_sub_wide with Rd = ZR (31)
    // Encoding: 0x0E20101F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20101F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_q_0_min_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Q = 0 (Min)
    // Fields: U=0, Rm=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_q_1_max_1400_4ec01400() {
    // Encoding: 0x4EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Q = 1 (Max)
    // Fields: Rd=0, Rm=0, U=0, Q=1, Rn=0
    let encoding: u32 = 0x4EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_u_0_min_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field U = 0 (Min)
    // Fields: Rn=0, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_u_1_max_1400_2ec01400() {
    // Encoding: 0x2EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field U = 1 (Max)
    // Fields: Q=0, Rm=0, U=1, Rd=0, Rn=0
    let encoding: u32 = 0x2EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rm_0_min_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rm = 0 (Min)
    // Fields: Rd=0, Rm=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rm_1_poweroftwo_1400_0ec11400()
{
    // Encoding: 0x0EC11400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rm = 1 (PowerOfTwo)
    // Fields: U=0, Rm=1, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rm_30_poweroftwominusone_1400_0ede1400()
 {
    // Encoding: 0x0EDE1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rd=0, Q=0, Rm=30, Rn=0
    let encoding: u32 = 0x0EDE1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rm_31_max_1400_0edf1400() {
    // Encoding: 0x0EDF1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rm = 31 (Max)
    // Fields: Q=0, Rm=31, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EDF1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rn_0_min_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rn = 0 (Min)
    // Fields: Rm=0, U=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rn_1_poweroftwo_1400_0ec01420()
{
    // Encoding: 0x0EC01420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, Rn=1, Rm=0, U=0, Rd=0
    let encoding: u32 = 0x0EC01420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rn_30_poweroftwominusone_1400_0ec017c0()
 {
    // Encoding: 0x0EC017C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=30, Rd=0, Q=0, Rm=0
    let encoding: u32 = 0x0EC017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rn_31_max_1400_0ec017e0() {
    // Encoding: 0x0EC017E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rn = 31 (Max)
    // Fields: U=0, Q=0, Rn=31, Rm=0, Rd=0
    let encoding: u32 = 0x0EC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rd_0_min_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rd = 0 (Min)
    // Fields: Rn=0, Rd=0, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rd_1_poweroftwo_1400_0ec01401()
{
    // Encoding: 0x0EC01401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, U=0, Rd=1, Q=0
    let encoding: u32 = 0x0EC01401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rd_30_poweroftwominusone_1400_0ec0141e()
 {
    // Encoding: 0x0EC0141E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, U=0, Q=0, Rm=0
    let encoding: u32 = 0x0EC0141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_field_rd_31_max_1400_0ec0141f() {
    // Encoding: 0x0EC0141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field Rd = 31 (Max)
    // Fields: Rn=0, Q=0, U=0, Rd=31, Rm=0
    let encoding: u32 = 0x0EC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_0_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Rm=0, Q=0, U=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_1_1400_4ec01400() {
    // Encoding: 0x4EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=1, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, Rn=0, Q=1, U=0
    let encoding: u32 = 0x4EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_2_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rm=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_3_1400_2ec01400() {
    // Encoding: 0x2EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=1, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=1, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x2EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_4_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rm=0, U=0, Rn=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_5_1400_0ec11400() {
    // Encoding: 0x0EC11400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=1, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rm=1, Rd=0, Rn=0
    let encoding: u32 = 0x0EC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_6_1400_0ede1400() {
    // Encoding: 0x0EDE1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=30, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rm=30, Rd=0, Rn=0
    let encoding: u32 = 0x0EDE1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_7_1400_0edf1400() {
    // Encoding: 0x0EDF1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=31, Rn=0, Rd=0
    // Fields: Rm=31, U=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EDF1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_8_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_9_1400_0ec01420() {
    // Encoding: 0x0EC01420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=1, Rd=0
    // Fields: U=0, Rn=1, Rm=0, Q=0, Rd=0
    let encoding: u32 = 0x0EC01420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_10_1400_0ec017c0() {
    // Encoding: 0x0EC017C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=30, Rd=0
    // Fields: Rd=0, Q=0, Rm=0, U=0, Rn=30
    let encoding: u32 = 0x0EC017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_11_1400_0ec017e0() {
    // Encoding: 0x0EC017E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, Q=0, U=0, Rn=31, Rd=0
    let encoding: u32 = 0x0EC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_12_1400_0ec01400() {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_13_1400_0ec01401() {
    // Encoding: 0x0EC01401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=1
    // Fields: Q=0, Rn=0, Rm=0, U=0, Rd=1
    let encoding: u32 = 0x0EC01401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_14_1400_0ec0141e() {
    // Encoding: 0x0EC0141E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=30
    // Fields: Rd=30, Rn=0, U=0, Rm=0, Q=0
    let encoding: u32 = 0x0EC0141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_15_1400_0ec0141f() {
    // Encoding: 0x0EC0141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, U=0, Rm=0, Q=0
    let encoding: u32 = 0x0EC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_16_1400_0ec11420() {
    // Encoding: 0x0EC11420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=1, Rn=1, Rd=0
    // Fields: Rm=1, Q=0, Rn=1, Rd=0, U=0
    let encoding: u32 = 0x0EC11420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_17_1400_0edf17e0() {
    // Encoding: 0x0EDF17E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=31, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, U=0, Q=0, Rm=31
    let encoding: u32 = 0x0EDF17E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_18_1400_0ec11401() {
    // Encoding: 0x0EC11401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=1, Rn=0, Rd=1
    // Fields: Q=0, Rm=1, Rn=0, U=0, Rd=1
    let encoding: u32 = 0x0EC11401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_19_1400_0edf141f() {
    // Encoding: 0x0EDF141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=31, Rn=0, Rd=31
    // Fields: U=0, Rn=0, Rd=31, Rm=31, Q=0
    let encoding: u32 = 0x0EDF141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_20_1400_0ec01421() {
    // Encoding: 0x0EC01421
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0EC01421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_combo_21_1400_0ec017ff() {
    // Encoding: 0x0EC017FF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd field combination: Q=0, U=0, Rm=0, Rn=31, Rd=31
    // Fields: Q=0, Rn=31, Rm=0, Rd=31, U=0
    let encoding: u32 = 0x0EC017FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_special_q_0_size_variant_0_5120_0ec01400()
 {
    // Encoding: 0x0EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, U=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_special_q_1_size_variant_1_5120_4ec01400()
 {
    // Encoding: 0x4EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd special value Q = 1 (Size variant 1)
    // Fields: Q=1, U=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x4EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_special_rn_31_stack_pointer_sp_may_require_alignment_5120_0ec017e0()
 {
    // Encoding: 0x0EC017E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, Rn=31, Q=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_5120_0ec0141f()
 {
    // Encoding: 0x0EC0141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, Rm=0, Rd=31, Q=0, Rn=0
    let encoding: u32 = 0x0EC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_q_0_min_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Q = 0 (Min)
    // Fields: Rn=0, U=0, Rd=0, Q=0, sz=0, Rm=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_q_1_max_d400_4ea0d400() {
    // Encoding: 0x4EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Q = 1 (Max)
    // Fields: Rm=0, U=0, sz=0, Rn=0, Q=1, Rd=0
    let encoding: u32 = 0x4EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_u_0_min_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field U = 0 (Min)
    // Fields: Rd=0, Q=0, U=0, sz=0, Rm=0, Rn=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_u_1_max_d400_2ea0d400() {
    // Encoding: 0x2EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field U = 1 (Max)
    // Fields: Rd=0, Q=0, U=1, Rn=0, Rm=0, sz=0
    let encoding: u32 = 0x2EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_sz_0_min_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field sz = 0 (Min)
    // Fields: Rd=0, Rm=0, sz=0, U=0, Rn=0, Q=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_sz_1_max_d400_0ee0d400() {
    // Encoding: 0x0EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field sz = 1 (Max)
    // Fields: Rd=0, Rm=0, sz=1, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0EE0D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rm_0_min_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rm = 0 (Min)
    // Fields: Rn=0, sz=0, Rm=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rm_1_poweroftwo_d400_0ea1d400() {
    // Encoding: 0x0EA1D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rm = 1 (PowerOfTwo)
    // Fields: sz=0, Rn=0, U=0, Rd=0, Rm=1, Q=0
    let encoding: u32 = 0x0EA1D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rm_30_poweroftwominusone_d400_0ebed400()
 {
    // Encoding: 0x0EBED400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, sz=0, Q=0, Rn=0, Rm=30, Rd=0
    let encoding: u32 = 0x0EBED400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rm_31_max_d400_0ebfd400() {
    // Encoding: 0x0EBFD400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rm = 31 (Max)
    // Fields: sz=0, U=0, Rm=31, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0EBFD400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rn_0_min_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rn = 0 (Min)
    // Fields: Q=0, Rm=0, Rn=0, U=0, sz=0, Rd=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rn_1_poweroftwo_d400_0ea0d420() {
    // Encoding: 0x0EA0D420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, sz=0, Rn=1, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EA0D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rn_30_poweroftwominusone_d400_0ea0d7c0()
 {
    // Encoding: 0x0EA0D7C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, U=0, sz=0, Rd=0, Rm=0, Q=0
    let encoding: u32 = 0x0EA0D7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rn_31_max_d400_0ea0d7e0() {
    // Encoding: 0x0EA0D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rn = 31 (Max)
    // Fields: U=0, Rn=31, Rd=0, Q=0, sz=0, Rm=0
    let encoding: u32 = 0x0EA0D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rd_0_min_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rd = 0 (Min)
    // Fields: Rd=0, Q=0, sz=0, Rm=0, U=0, Rn=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rd_1_poweroftwo_d400_0ea0d401() {
    // Encoding: 0x0EA0D401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, Rd=1, U=0, sz=0, Q=0
    let encoding: u32 = 0x0EA0D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rd_30_poweroftwominusone_d400_0ea0d41e()
 {
    // Encoding: 0x0EA0D41E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Q=0, Rm=0, Rd=30, Rn=0, sz=0
    let encoding: u32 = 0x0EA0D41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_field_rd_31_max_d400_0ea0d41f() {
    // Encoding: 0x0EA0D41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field Rd = 31 (Max)
    // Fields: Q=0, U=0, Rm=0, Rn=0, Rd=31, sz=0
    let encoding: u32 = 0x0EA0D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_0_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, U=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_1_d400_4ea0d400() {
    // Encoding: 0x4EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=1, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=1, Rm=0, sz=0, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x4EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_2_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_3_d400_2ea0d400() {
    // Encoding: 0x2EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=1, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=1, Rm=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x2EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_4_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, sz=0, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_5_d400_0ee0d400() {
    // Encoding: 0x0EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=1, Rm=0, Rn=0, Rd=0
    // Fields: sz=1, Q=0, U=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EE0D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_6_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rm=0, sz=0, U=0, Rd=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_7_d400_0ea1d400() {
    // Encoding: 0x0EA1D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=1, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, Rm=1, sz=0, Q=0
    let encoding: u32 = 0x0EA1D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_8_d400_0ebed400() {
    // Encoding: 0x0EBED400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, Rn=0, Q=0, Rm=30, U=0
    let encoding: u32 = 0x0EBED400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_9_d400_0ebfd400() {
    // Encoding: 0x0EBFD400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=31, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rm=31, Rn=0, Q=0, sz=0
    let encoding: u32 = 0x0EBFD400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_10_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_11_d400_0ea0d420() {
    // Encoding: 0x0EA0D420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=1, Rd=0
    // Fields: Rn=1, sz=0, Q=0, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EA0D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_12_d400_0ea0d7c0() {
    // Encoding: 0x0EA0D7C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=30, Rd=0
    // Fields: Rm=0, Rd=0, Rn=30, Q=0, U=0, sz=0
    let encoding: u32 = 0x0EA0D7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_13_d400_0ea0d7e0() {
    // Encoding: 0x0EA0D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, Q=0, Rd=0, Rn=31, sz=0, U=0
    let encoding: u32 = 0x0EA0D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_14_d400_0ea0d400() {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Q=0, Rn=0, U=0, sz=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_15_d400_0ea0d401() {
    // Encoding: 0x0EA0D401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=1
    // Fields: U=0, Rm=0, Q=0, Rn=0, sz=0, Rd=1
    let encoding: u32 = 0x0EA0D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_16_d400_0ea0d41e() {
    // Encoding: 0x0EA0D41E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=30
    // Fields: Rd=30, sz=0, Rn=0, Q=0, Rm=0, U=0
    let encoding: u32 = 0x0EA0D41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_17_d400_0ea0d41f() {
    // Encoding: 0x0EA0D41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=0, Rd=31
    // Fields: sz=0, U=0, Rm=0, Q=0, Rd=31, Rn=0
    let encoding: u32 = 0x0EA0D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_18_d400_0ea1d420() {
    // Encoding: 0x0EA1D420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=1, Rn=1, Rd=0
    // Fields: sz=0, Rd=0, U=0, Q=0, Rn=1, Rm=1
    let encoding: u32 = 0x0EA1D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_19_d400_0ebfd7e0() {
    // Encoding: 0x0EBFD7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=31, Rn=31, Rd=0
    // Fields: Rn=31, sz=0, Rm=31, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0EBFD7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_20_d400_0ea1d401() {
    // Encoding: 0x0EA1D401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=1, Rn=0, Rd=1
    // Fields: U=0, Rn=0, Q=0, sz=0, Rm=1, Rd=1
    let encoding: u32 = 0x0EA1D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_21_d400_0ebfd41f() {
    // Encoding: 0x0EBFD41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=31, Rn=0, Rd=31
    // Fields: U=0, Rd=31, Rn=0, sz=0, Q=0, Rm=31
    let encoding: u32 = 0x0EBFD41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_22_d400_0ea0d421() {
    // Encoding: 0x0EA0D421
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=1, Rd=1
    // Fields: sz=0, Rm=0, U=0, Rn=1, Q=0, Rd=1
    let encoding: u32 = 0x0EA0D421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_combo_23_d400_0ea0d7ff() {
    // Encoding: 0x0EA0D7FF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd field combination: Q=0, U=0, sz=0, Rm=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, Q=0, U=0, Rm=0, sz=0
    let encoding: u32 = 0x0EA0D7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_special_q_0_size_variant_0_54272_0ee0d400()
 {
    // Encoding: 0x0EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, U=0, Rm=0, Rd=0, Q=0, sz=1
    let encoding: u32 = 0x0EE0D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_special_q_1_size_variant_1_54272_4ee0d400()
 {
    // Encoding: 0x4EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd special value Q = 1 (Size variant 1)
    // Fields: Rd=0, Q=1, U=0, Rn=0, Rm=0, sz=1
    let encoding: u32 = 0x4EE0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_special_sz_0_size_variant_0_54272_0ea0d400()
 {
    // Encoding: 0x0EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd special value sz = 0 (Size variant 0)
    // Fields: Rn=0, sz=0, Q=0, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_special_sz_1_size_variant_1_54272_0ee0d400()
 {
    // Encoding: 0x0EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, sz=1, U=0, Q=0, Rd=0, Rm=0
    let encoding: u32 = 0x0EE0D400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_special_rn_31_stack_pointer_sp_may_require_alignment_54272_0ee0d7e0()
 {
    // Encoding: 0x0EE0D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Q=0, U=0, Rm=0, sz=1, Rd=0
    let encoding: u32 = 0x0EE0D7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_54272_0ee0d41f()
 {
    // Encoding: 0x0EE0D41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, Rd=31, Rn=0, Q=0, sz=1, U=0
    let encoding: u32 = 0x0EE0D41F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_reg_write_0_0ec01400() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd register write: SimdFromField("d")
    // Encoding: 0x0EC01400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EC01400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_sp_rn_0ec017e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd with Rn = SP (31)
    // Encoding: 0x0EC017E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EC017E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_zr_rd_0ec0141f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd with Rd = ZR (31)
    // Encoding: 0x0EC0141F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EC0141F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_reg_write_0_0ea0d400() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd register write: SimdFromField("d")
    // Encoding: 0x0EA0D400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0D400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_sp_rn_0ea0d7e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd with Rn = SP (31)
    // Encoding: 0x0EA0D7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0D7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_zr_rd_0ea0d41f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_simd with Rd = ZR (31)
    // Encoding: 0x0EA0D41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0EA0D41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_unary_add_saturating_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_u_0_min_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field U = 0 (Min)
    // Fields: Rn=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_u_1_max_3800_7e203800() {
    // Encoding: 0x7E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field U = 1 (Max)
    // Fields: Rn=0, U=1, size=0, Rd=0
    let encoding: u32 = 0x7E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_size_0_min_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field size = 0 (Min)
    // Fields: Rn=0, size=0, U=0, Rd=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_size_1_poweroftwo_3800_5e603800()
{
    // Encoding: 0x5E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field size = 1 (PowerOfTwo)
    // Fields: U=0, Rd=0, size=1, Rn=0
    let encoding: u32 = 0x5E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_size_2_poweroftwo_3800_5ea03800()
{
    // Encoding: 0x5EA03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field size = 2 (PowerOfTwo)
    // Fields: U=0, Rn=0, Rd=0, size=2
    let encoding: u32 = 0x5EA03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_size_3_max_3800_5ee03800() {
    // Encoding: 0x5EE03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field size = 3 (Max)
    // Fields: Rn=0, Rd=0, U=0, size=3
    let encoding: u32 = 0x5EE03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rn_0_min_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rn = 0 (Min)
    // Fields: size=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rn_1_poweroftwo_3800_5e203820() {
    // Encoding: 0x5E203820
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rn = 1 (PowerOfTwo)
    // Fields: size=0, U=0, Rn=1, Rd=0
    let encoding: u32 = 0x5E203820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rn_30_poweroftwominusone_3800_5e203bc0()
 {
    // Encoding: 0x5E203BC0
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, U=0, size=0
    let encoding: u32 = 0x5E203BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rn_31_max_3800_5e203be0() {
    // Encoding: 0x5E203BE0
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rn = 31 (Max)
    // Fields: U=0, Rn=31, size=0, Rd=0
    let encoding: u32 = 0x5E203BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rd_0_min_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rd = 0 (Min)
    // Fields: size=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rd_1_poweroftwo_3800_5e203801() {
    // Encoding: 0x5E203801
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rd = 1 (PowerOfTwo)
    // Fields: size=0, U=0, Rn=0, Rd=1
    let encoding: u32 = 0x5E203801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rd_30_poweroftwominusone_3800_5e20381e()
 {
    // Encoding: 0x5E20381E
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, size=0, Rn=0, Rd=30
    let encoding: u32 = 0x5E20381E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_field_rd_31_max_3800_5e20381f() {
    // Encoding: 0x5E20381F
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field Rd = 31 (Max)
    // Fields: U=0, Rd=31, size=0, Rn=0
    let encoding: u32 = 0x5E20381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_0_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=0
    // Fields: size=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_1_3800_7e203800() {
    // Encoding: 0x7E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=1, size=0, Rn=0, Rd=0
    // Fields: U=1, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x7E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_2_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_3_3800_5e603800() {
    // Encoding: 0x5E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=1, Rn=0, Rd=0
    // Fields: U=0, size=1, Rn=0, Rd=0
    let encoding: u32 = 0x5E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_4_3800_5ea03800() {
    // Encoding: 0x5EA03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=2, Rn=0, Rd=0
    // Fields: Rd=0, size=2, U=0, Rn=0
    let encoding: u32 = 0x5EA03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_5_3800_5ee03800() {
    // Encoding: 0x5EE03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=3, Rn=0, Rd=0
    // Fields: size=3, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x5EE03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_6_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_7_3800_5e203820() {
    // Encoding: 0x5E203820
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=1, Rd=0
    // Fields: size=0, U=0, Rd=0, Rn=1
    let encoding: u32 = 0x5E203820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_8_3800_5e203bc0() {
    // Encoding: 0x5E203BC0
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=30, Rd=0
    // Fields: Rn=30, U=0, size=0, Rd=0
    let encoding: u32 = 0x5E203BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_9_3800_5e203be0() {
    // Encoding: 0x5E203BE0
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=31, Rd=0
    // Fields: U=0, Rd=0, size=0, Rn=31
    let encoding: u32 = 0x5E203BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_10_3800_5e203800() {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, size=0, Rn=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_11_3800_5e203801() {
    // Encoding: 0x5E203801
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=1
    // Fields: Rn=0, size=0, U=0, Rd=1
    let encoding: u32 = 0x5E203801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_12_3800_5e20381e() {
    // Encoding: 0x5E20381E
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=30
    // Fields: U=0, Rn=0, size=0, Rd=30
    let encoding: u32 = 0x5E20381E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_13_3800_5e20381f() {
    // Encoding: 0x5E20381F
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, size=0, U=0
    let encoding: u32 = 0x5E20381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_14_3800_5e203821() {
    // Encoding: 0x5E203821
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=1, Rd=1
    // Fields: Rd=1, size=0, Rn=1, U=0
    let encoding: u32 = 0x5E203821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_combo_15_3800_5e203bff() {
    // Encoding: 0x5E203BFF
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd field combination: U=0, size=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, size=0, U=0
    let encoding: u32 = 0x5E203BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_special_size_0_size_variant_0_14336_5e203800()
 {
    // Encoding: 0x5E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd special value size = 0 (Size variant 0)
    // Fields: Rn=0, size=0, Rd=0, U=0
    let encoding: u32 = 0x5E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_special_size_1_size_variant_1_14336_5e603800()
 {
    // Encoding: 0x5E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd special value size = 1 (Size variant 1)
    // Fields: size=1, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_special_size_2_size_variant_2_14336_5ea03800()
 {
    // Encoding: 0x5EA03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd special value size = 2 (Size variant 2)
    // Fields: Rd=0, Rn=0, U=0, size=2
    let encoding: u32 = 0x5EA03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_special_size_3_size_variant_3_14336_5ee03800()
 {
    // Encoding: 0x5EE03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd special value size = 3 (Size variant 3)
    // Fields: U=0, Rn=0, Rd=0, size=3
    let encoding: u32 = 0x5EE03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_14336_5e603be0()
 {
    // Encoding: 0x5E603BE0
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rd=0, U=0, Rn=31
    let encoding: u32 = 0x5E603BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_14336_5e60381f()
 {
    // Encoding: 0x5E60381F
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rn=0, U=0, Rd=31
    let encoding: u32 = 0x5E60381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_q_0_min_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Q = 0 (Min)
    // Fields: Q=0, size=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_q_1_max_3800_4e203800() {
    // Encoding: 0x4E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Q = 1 (Max)
    // Fields: size=0, U=0, Rn=0, Rd=0, Q=1
    let encoding: u32 = 0x4E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_u_0_min_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field U = 0 (Min)
    // Fields: Rd=0, Rn=0, size=0, Q=0, U=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_u_1_max_3800_2e203800() {
    // Encoding: 0x2E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field U = 1 (Max)
    // Fields: Rd=0, size=0, Rn=0, Q=0, U=1
    let encoding: u32 = 0x2E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_size_0_min_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field size = 0 (Min)
    // Fields: Rd=0, U=0, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_size_1_poweroftwo_3800_0e603800()
{
    // Encoding: 0x0E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field size = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, Q=0, Rd=0, size=1
    let encoding: u32 = 0x0E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_size_2_poweroftwo_3800_0ea03800()
{
    // Encoding: 0x0EA03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field size = 2 (PowerOfTwo)
    // Fields: Q=0, U=0, size=2, Rd=0, Rn=0
    let encoding: u32 = 0x0EA03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_size_3_max_3800_0ee03800() {
    // Encoding: 0x0EE03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field size = 3 (Max)
    // Fields: Rn=0, Rd=0, U=0, Q=0, size=3
    let encoding: u32 = 0x0EE03800;
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

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rn_0_min_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rn = 0 (Min)
    // Fields: Q=0, size=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rn_1_poweroftwo_3800_0e203820() {
    // Encoding: 0x0E203820
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, size=0, U=0, Rn=1, Rd=0
    let encoding: u32 = 0x0E203820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rn_30_poweroftwominusone_3800_0e203bc0()
 {
    // Encoding: 0x0E203BC0
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, size=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0E203BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rn_31_max_3800_0e203be0() {
    // Encoding: 0x0E203BE0
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rn = 31 (Max)
    // Fields: Rd=0, size=0, Rn=31, Q=0, U=0
    let encoding: u32 = 0x0E203BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rd_0_min_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rd = 0 (Min)
    // Fields: Q=0, Rn=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rd_1_poweroftwo_3800_0e203801() {
    // Encoding: 0x0E203801
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, Rd=1, U=0, size=0, Rn=0
    let encoding: u32 = 0x0E203801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rd_30_poweroftwominusone_3800_0e20381e()
 {
    // Encoding: 0x0E20381E
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, U=0, Rd=30, Q=0, size=0
    let encoding: u32 = 0x0E20381E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_field_rd_31_max_3800_0e20381f() {
    // Encoding: 0x0E20381F
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field Rd = 31 (Max)
    // Fields: Q=0, U=0, Rn=0, Rd=31, size=0
    let encoding: u32 = 0x0E20381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_0_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Q=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_1_3800_4e203800() {
    // Encoding: 0x4E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=1, U=0, size=0, Rn=0, Rd=0
    // Fields: Q=1, U=0, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x4E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_2_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_3_3800_2e203800() {
    // Encoding: 0x2E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=1, size=0, Rn=0, Rd=0
    // Fields: size=0, Q=0, Rd=0, U=1, Rn=0
    let encoding: u32 = 0x2E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_4_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_5_3800_0e603800() {
    // Encoding: 0x0E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=1, Rn=0, Rd=0
    // Fields: U=0, Q=0, Rn=0, Rd=0, size=1
    let encoding: u32 = 0x0E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_6_3800_0ea03800() {
    // Encoding: 0x0EA03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=2, Rn=0, Rd=0
    // Fields: size=2, Q=0, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x0EA03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_7_3800_0ee03800() {
    // Encoding: 0x0EE03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=3, Rn=0, Rd=0
    // Fields: U=0, Rn=0, size=3, Rd=0, Q=0
    let encoding: u32 = 0x0EE03800;
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

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_8_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_9_3800_0e203820() {
    // Encoding: 0x0E203820
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=1, Rd=0
    // Fields: U=0, Rn=1, size=0, Q=0, Rd=0
    let encoding: u32 = 0x0E203820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_10_3800_0e203bc0() {
    // Encoding: 0x0E203BC0
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=30, Rd=0
    // Fields: Q=0, size=0, U=0, Rn=30, Rd=0
    let encoding: u32 = 0x0E203BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_11_3800_0e203be0() {
    // Encoding: 0x0E203BE0
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=31, Rd=0
    // Fields: Rn=31, size=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0E203BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_12_3800_0e203800() {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_13_3800_0e203801() {
    // Encoding: 0x0E203801
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=1
    // Fields: size=0, Rn=0, Rd=1, Q=0, U=0
    let encoding: u32 = 0x0E203801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_14_3800_0e20381e() {
    // Encoding: 0x0E20381E
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=30
    // Fields: U=0, Rd=30, size=0, Rn=0, Q=0
    let encoding: u32 = 0x0E20381E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_15_3800_0e20381f() {
    // Encoding: 0x0E20381F
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=0, Rd=31
    // Fields: U=0, Q=0, size=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E20381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_16_3800_0e203821() {
    // Encoding: 0x0E203821
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=1, Rd=1
    // Fields: size=0, Q=0, U=0, Rn=1, Rd=1
    let encoding: u32 = 0x0E203821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_combo_17_3800_0e203bff() {
    // Encoding: 0x0E203BFF
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd field combination: Q=0, U=0, size=0, Rn=31, Rd=31
    // Fields: size=0, Q=0, U=0, Rn=31, Rd=31
    let encoding: u32 = 0x0E203BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_q_0_size_variant_0_14336_0e603800()
 {
    // Encoding: 0x0E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value Q = 0 (Size variant 0)
    // Fields: U=0, Q=0, Rd=0, Rn=0, size=1
    let encoding: u32 = 0x0E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_q_1_size_variant_1_14336_4e603800()
 {
    // Encoding: 0x4E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, size=1, U=0, Q=1
    let encoding: u32 = 0x4E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_size_0_size_variant_0_14336_0e203800()
 {
    // Encoding: 0x0E203800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value size = 0 (Size variant 0)
    // Fields: Q=0, size=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x0E203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_size_1_size_variant_1_14336_0e603800()
 {
    // Encoding: 0x0E603800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value size = 1 (Size variant 1)
    // Fields: Rd=0, U=0, Rn=0, Q=0, size=1
    let encoding: u32 = 0x0E603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_size_2_size_variant_2_14336_0ea03800()
 {
    // Encoding: 0x0EA03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value size = 2 (Size variant 2)
    // Fields: size=2, Rn=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0EA03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_size_3_size_variant_3_14336_0ee03800()
 {
    // Encoding: 0x0EE03800
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value size = 3 (Size variant 3)
    // Fields: Rd=0, Q=0, size=3, Rn=0, U=0
    let encoding: u32 = 0x0EE03800;
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

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_rn_31_stack_pointer_sp_may_require_alignment_14336_0e603be0()
 {
    // Encoding: 0x0E603BE0
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rn=31, U=0, size=1, Rd=0
    let encoding: u32 = 0x0E603BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_14336_0e60381f()
 {
    // Encoding: 0x0E60381F
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, U=0, Q=0, size=1
    let encoding: u32 = 0x0E60381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_reg_write_0_5e203800() {
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd register write: SimdFromField("d")
    // Encoding: 0x5E203800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E203800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_sp_rn_5e203be0() {
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd with Rn = SP (31)
    // Encoding: 0x5E203BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E203BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_sisd_zr_rd_5e20381f() {
    // Test aarch64_vector_arithmetic_unary_add_saturating_sisd with Rd = ZR (31)
    // Encoding: 0x5E20381F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E20381F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_reg_write_0_0e203800() {
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd register write: SimdFromField("d")
    // Encoding: 0x0E203800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E203800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_sp_rn_0e203be0() {
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd with Rn = SP (31)
    // Encoding: 0x0E203BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E203BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_unary_add_saturating_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_unary_add_saturating_simd_zr_rd_0e20381f() {
    // Test aarch64_vector_arithmetic_unary_add_saturating_simd with Rd = ZR (31)
    // Encoding: 0x0E20381F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20381F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_halving_rounding Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_q_0_min_1400_0e201400()
{
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Q = 0 (Min)
    // Fields: size=0, Q=0, Rm=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_q_1_max_1400_4e201400()
{
    // Encoding: 0x4E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Q = 1 (Max)
    // Fields: Rn=0, size=0, Q=1, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x4E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_u_0_min_1400_0e201400()
{
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field U = 0 (Min)
    // Fields: Q=0, size=0, Rn=0, U=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_u_1_max_1400_2e201400()
{
    // Encoding: 0x2E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field U = 1 (Max)
    // Fields: Rd=0, U=1, Rm=0, Q=0, Rn=0, size=0
    let encoding: u32 = 0x2E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_size_0_min_1400_0e201400()
 {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field size = 0 (Min)
    // Fields: Q=0, Rm=0, size=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_size_1_poweroftwo_1400_0e601400()
 {
    // Encoding: 0x0E601400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field size = 1 (PowerOfTwo)
    // Fields: U=0, Rd=0, Rn=0, Q=0, Rm=0, size=1
    let encoding: u32 = 0x0E601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_size_2_poweroftwo_1400_0ea01400()
 {
    // Encoding: 0x0EA01400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field size = 2 (PowerOfTwo)
    // Fields: Q=0, size=2, Rm=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EA01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_size_3_max_1400_0ee01400()
 {
    // Encoding: 0x0EE01400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field size = 3 (Max)
    // Fields: size=3, Rm=0, Q=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0EE01400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rm_0_min_1400_0e201400()
{
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rm = 0 (Min)
    // Fields: Rm=0, size=0, Rn=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rm_1_poweroftwo_1400_0e211400()
 {
    // Encoding: 0x0E211400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=1, Rd=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E211400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rm_30_poweroftwominusone_1400_0e3e1400()
 {
    // Encoding: 0x0E3E1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=30, Q=0, U=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E3E1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rm_31_max_1400_0e3f1400()
 {
    // Encoding: 0x0E3F1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rm = 31 (Max)
    // Fields: U=0, size=0, Rd=0, Q=0, Rn=0, Rm=31
    let encoding: u32 = 0x0E3F1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rn_0_min_1400_0e201400()
{
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rn = 0 (Min)
    // Fields: Q=0, size=0, U=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rn_1_poweroftwo_1400_0e201420()
 {
    // Encoding: 0x0E201420
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rn = 1 (PowerOfTwo)
    // Fields: U=0, Rn=1, Q=0, Rd=0, size=0, Rm=0
    let encoding: u32 = 0x0E201420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rn_30_poweroftwominusone_1400_0e2017c0()
 {
    // Encoding: 0x0E2017C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, size=0, U=0, Rd=0, Q=0, Rn=30
    let encoding: u32 = 0x0E2017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rn_31_max_1400_0e2017e0()
 {
    // Encoding: 0x0E2017E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rn = 31 (Max)
    // Fields: U=0, size=0, Q=0, Rm=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E2017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rd_0_min_1400_0e201400()
{
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rd = 0 (Min)
    // Fields: U=0, Q=0, size=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rd_1_poweroftwo_1400_0e201401()
 {
    // Encoding: 0x0E201401
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rd = 1 (PowerOfTwo)
    // Fields: U=0, size=0, Rm=0, Rd=1, Rn=0, Q=0
    let encoding: u32 = 0x0E201401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rd_30_poweroftwominusone_1400_0e20141e()
 {
    // Encoding: 0x0E20141E
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rd=30, Rn=0, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0E20141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_field_rd_31_max_1400_0e20141f()
 {
    // Encoding: 0x0E20141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field Rd = 31 (Max)
    // Fields: Q=0, Rd=31, U=0, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x0E20141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_0_1400_0e201400() {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, U=0, Rm=0, Rn=0, size=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_1_1400_4e201400() {
    // Encoding: 0x4E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, size=0, Q=1, Rm=0
    let encoding: u32 = 0x4E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_2_1400_0e201400() {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, Rn=0, size=0, U=0, Rd=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_3_1400_2e201400() {
    // Encoding: 0x2E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, U=1, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_4_1400_0e201400() {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, size=0, Rm=0, U=0, Rn=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_5_1400_0e601400() {
    // Encoding: 0x0E601400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, Rd=0, U=0, size=1, Rn=0
    let encoding: u32 = 0x0E601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_6_1400_0ea01400() {
    // Encoding: 0x0EA01400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, size=2, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EA01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_7_1400_0ee01400() {
    // Encoding: 0x0EE01400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rm=0, Q=0, Rn=0, Rd=0, size=3
    let encoding: u32 = 0x0EE01400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_8_1400_0e201400() {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, Q=0, Rm=0, size=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_9_1400_0e211400() {
    // Encoding: 0x0E211400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Q=0, Rm=1, U=0, Rn=0
    let encoding: u32 = 0x0E211400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_10_1400_0e3e1400() {
    // Encoding: 0x0E3E1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, size=0, Rn=0, Rm=30, U=0
    let encoding: u32 = 0x0E3E1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_11_1400_0e3f1400() {
    // Encoding: 0x0E3F1400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rm=31, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0E3F1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_12_1400_0e201400() {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Rm=0, Q=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_13_1400_0e201420() {
    // Encoding: 0x0E201420
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: Rm=0, size=0, Rn=1, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0E201420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_14_1400_0e2017c0() {
    // Encoding: 0x0E2017C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Rd=0, size=0, Rm=0, Rn=30, U=0, Q=0
    let encoding: u32 = 0x0E2017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_15_1400_0e2017e0() {
    // Encoding: 0x0E2017E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: U=0, Rd=0, Rm=0, Rn=31, size=0, Q=0
    let encoding: u32 = 0x0E2017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_16_1400_0e201400() {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, size=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_17_1400_0e201401() {
    // Encoding: 0x0E201401
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: size=0, Rm=0, Rd=1, Q=0, Rn=0, U=0
    let encoding: u32 = 0x0E201401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_18_1400_0e20141e() {
    // Encoding: 0x0E20141E
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Q=0, size=0, U=0, Rn=0, Rd=30, Rm=0
    let encoding: u32 = 0x0E20141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_19_1400_0e20141f() {
    // Encoding: 0x0E20141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, Q=0, Rm=0, Rd=31, U=0, size=0
    let encoding: u32 = 0x0E20141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_20_1400_0e211420() {
    // Encoding: 0x0E211420
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: U=0, size=0, Q=0, Rm=1, Rn=1, Rd=0
    let encoding: u32 = 0x0E211420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_21_1400_0e3f17e0() {
    // Encoding: 0x0E3F17E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, Q=0, Rm=31, size=0, U=0
    let encoding: u32 = 0x0E3F17E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_22_1400_0e211401() {
    // Encoding: 0x0E211401
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: size=0, Rn=0, Q=0, U=0, Rm=1, Rd=1
    let encoding: u32 = 0x0E211401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_23_1400_0e3f141f() {
    // Encoding: 0x0E3F141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Rm=31, U=0, Q=0, Rn=0, size=0, Rd=31
    let encoding: u32 = 0x0E3F141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_24_1400_0e201421() {
    // Encoding: 0x0E201421
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, Q=0, size=0, Rm=0, U=0
    let encoding: u32 = 0x0E201421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_combo_25_1400_0e2017ff() {
    // Encoding: 0x0E2017FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Q=0, size=0, Rm=0, Rd=31, U=0, Rn=31
    let encoding: u32 = 0x0E2017FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_q_0_size_variant_0_5120_0e601400()
 {
    // Encoding: 0x0E601400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value Q = 0 (Size variant 0)
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, U=0, size=1
    let encoding: u32 = 0x0E601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_q_1_size_variant_1_5120_4e601400()
 {
    // Encoding: 0x4E601400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value Q = 1 (Size variant 1)
    // Fields: size=1, U=0, Rm=0, Q=1, Rd=0, Rn=0
    let encoding: u32 = 0x4E601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_size_0_size_variant_0_5120_0e201400()
 {
    // Encoding: 0x0E201400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value size = 0 (Size variant 0)
    // Fields: U=0, size=0, Q=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_size_1_size_variant_1_5120_0e601400()
 {
    // Encoding: 0x0E601400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value size = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, Rm=0, Q=0, U=0, size=1
    let encoding: u32 = 0x0E601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_size_2_size_variant_2_5120_0ea01400()
 {
    // Encoding: 0x0EA01400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value size = 2 (Size variant 2)
    // Fields: Rn=0, size=2, Rm=0, U=0, Rd=0, Q=0
    let encoding: u32 = 0x0EA01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_size_3_size_variant_3_5120_0ee01400()
 {
    // Encoding: 0x0EE01400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value size = 3 (Size variant 3)
    // Fields: Q=0, Rd=0, size=3, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0EE01400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_rn_31_stack_pointer_sp_may_require_alignment_5120_0e6017e0()
 {
    // Encoding: 0x0E6017E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, size=1, Rn=31, Rm=0, U=0, Rd=0
    let encoding: u32 = 0x0E6017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_5120_0e60141f()
 {
    // Encoding: 0x0E60141F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Q=0, U=0, Rm=0, Rd=31, Rn=0
    let encoding: u32 = 0x0E60141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_reg_write_0_0e201400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding register write: SimdFromField("d")
    // Encoding: 0x0E201400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E201400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_sp_rn_0e2017e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding with Rn = SP (31)
    // Encoding: 0x0E2017E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2017E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_zr_rd_0e20141f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_rounding with Rd = ZR (31)
    // Encoding: 0x0E20141F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20141F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_u_0_min_2c00_5e202c00() {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field U = 0 (Min)
    // Fields: U=0, Rd=0, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_u_1_max_2c00_7e202c00() {
    // Encoding: 0x7E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field U = 1 (Max)
    // Fields: Rd=0, Rn=0, U=1, Rm=0, size=0
    let encoding: u32 = 0x7E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_size_0_min_2c00_5e202c00()
 {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field size = 0 (Min)
    // Fields: U=0, Rm=0, Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_size_1_poweroftwo_2c00_5e602c00()
 {
    // Encoding: 0x5E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field size = 1 (PowerOfTwo)
    // Fields: size=1, U=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_size_2_poweroftwo_2c00_5ea02c00()
 {
    // Encoding: 0x5EA02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field size = 2 (PowerOfTwo)
    // Fields: U=0, Rm=0, Rn=0, Rd=0, size=2
    let encoding: u32 = 0x5EA02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_size_3_max_2c00_5ee02c00()
 {
    // Encoding: 0x5EE02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field size = 3 (Max)
    // Fields: U=0, Rm=0, Rd=0, size=3, Rn=0
    let encoding: u32 = 0x5EE02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rm_0_min_2c00_5e202c00()
{
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rm = 0 (Min)
    // Fields: U=0, Rn=0, Rm=0, Rd=0, size=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rm_1_poweroftwo_2c00_5e212c00()
 {
    // Encoding: 0x5E212C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, size=0, U=0, Rm=1
    let encoding: u32 = 0x5E212C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rm_30_poweroftwominusone_2c00_5e3e2c00()
 {
    // Encoding: 0x5E3E2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, Rm=30, size=0, U=0
    let encoding: u32 = 0x5E3E2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rm_31_max_2c00_5e3f2c00()
{
    // Encoding: 0x5E3F2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rm = 31 (Max)
    // Fields: size=0, Rn=0, Rm=31, U=0, Rd=0
    let encoding: u32 = 0x5E3F2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rn_0_min_2c00_5e202c00()
{
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rn = 0 (Min)
    // Fields: U=0, size=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rn_1_poweroftwo_2c00_5e202c20()
 {
    // Encoding: 0x5E202C20
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, size=0, Rn=1, Rd=0, Rm=0
    let encoding: u32 = 0x5E202C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rn_30_poweroftwominusone_2c00_5e202fc0()
 {
    // Encoding: 0x5E202FC0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, U=0, Rn=30, size=0, Rd=0
    let encoding: u32 = 0x5E202FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rn_31_max_2c00_5e202fe0()
{
    // Encoding: 0x5E202FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, size=0, U=0, Rm=0
    let encoding: u32 = 0x5E202FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rd_0_min_2c00_5e202c00()
{
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rd = 0 (Min)
    // Fields: Rn=0, size=0, Rm=0, U=0, Rd=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rd_1_poweroftwo_2c00_5e202c01()
 {
    // Encoding: 0x5E202C01
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, U=0, Rm=0, size=0, Rn=0
    let encoding: u32 = 0x5E202C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rd_30_poweroftwominusone_2c00_5e202c1e()
 {
    // Encoding: 0x5E202C1E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, U=0, size=0, Rm=0, Rn=0
    let encoding: u32 = 0x5E202C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_field_rd_31_max_2c00_5e202c1f()
{
    // Encoding: 0x5E202C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field Rd = 31 (Max)
    // Fields: U=0, size=0, Rn=0, Rm=0, Rd=31
    let encoding: u32 = 0x5E202C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_0_2c00_5e202c00() {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_1_2c00_7e202c00() {
    // Encoding: 0x7E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Rn=0, U=1, Rd=0, Rm=0
    let encoding: u32 = 0x7E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_2_2c00_5e202c00() {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_3_2c00_5e602c00() {
    // Encoding: 0x5E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x5E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_4_2c00_5ea02c00() {
    // Encoding: 0x5EA02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: size=2, Rd=0, Rn=0, Rm=0, U=0
    let encoding: u32 = 0x5EA02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_5_2c00_5ee02c00() {
    // Encoding: 0x5EE02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, U=0, size=3, Rn=0
    let encoding: u32 = 0x5EE02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_6_2c00_5e202c00() {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, size=0, U=0, Rn=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_7_2c00_5e212c00() {
    // Encoding: 0x5E212C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Rm=1, size=0, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x5E212C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_8_2c00_5e3e2c00() {
    // Encoding: 0x5E3E2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rm=30, Rd=0, U=0, size=0, Rn=0
    let encoding: u32 = 0x5E3E2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_9_2c00_5e3f2c00() {
    // Encoding: 0x5E3F2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rm=31, Rd=0, U=0
    let encoding: u32 = 0x5E3F2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_10_2c00_5e202c00() {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_11_2c00_5e202c20() {
    // Encoding: 0x5E202C20
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: Rn=1, Rm=0, U=0, size=0, Rd=0
    let encoding: u32 = 0x5E202C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_12_2c00_5e202fc0() {
    // Encoding: 0x5E202FC0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Rd=0, Rm=0, U=0, Rn=30, size=0
    let encoding: u32 = 0x5E202FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_13_2c00_5e202fe0() {
    // Encoding: 0x5E202FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: size=0, Rn=31, Rm=0, U=0, Rd=0
    let encoding: u32 = 0x5E202FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_14_2c00_5e202c00() {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, size=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_15_2c00_5e202c01() {
    // Encoding: 0x5E202C01
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, size=0, Rm=0, U=0
    let encoding: u32 = 0x5E202C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_16_2c00_5e202c1e() {
    // Encoding: 0x5E202C1E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Rm=0, Rn=0, U=0, Rd=30, size=0
    let encoding: u32 = 0x5E202C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_17_2c00_5e202c1f() {
    // Encoding: 0x5E202C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rd=31, Rm=0, U=0, Rn=0, size=0
    let encoding: u32 = 0x5E202C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_18_2c00_5e212c20() {
    // Encoding: 0x5E212C20
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: U=0, size=0, Rn=1, Rd=0, Rm=1
    let encoding: u32 = 0x5E212C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_19_2c00_5e3f2fe0() {
    // Encoding: 0x5E3F2FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rm=31, size=0, Rd=0, Rn=31, U=0
    let encoding: u32 = 0x5E3F2FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_20_2c00_5e212c01() {
    // Encoding: 0x5E212C01
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, size=0, U=0, Rm=1
    let encoding: u32 = 0x5E212C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_21_2c00_5e3f2c1f() {
    // Encoding: 0x5E3F2C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: size=0, Rm=31, U=0, Rd=31, Rn=0
    let encoding: u32 = 0x5E3F2C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_22_2c00_5e202c21() {
    // Encoding: 0x5E202C21
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: Rn=1, Rm=0, Rd=1, U=0, size=0
    let encoding: u32 = 0x5E202C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_combo_23_2c00_5e202fff() {
    // Encoding: 0x5E202FFF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd field combination: U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Rd=31, size=0, U=0, Rm=0, Rn=31
    let encoding: u32 = 0x5E202FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_special_size_0_size_variant_0_11264_5e202c00()
 {
    // Encoding: 0x5E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd special value size = 0 (Size variant 0)
    // Fields: size=0, U=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_special_size_1_size_variant_1_11264_5e602c00()
 {
    // Encoding: 0x5E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd special value size = 1 (Size variant 1)
    // Fields: size=1, Rm=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_special_size_2_size_variant_2_11264_5ea02c00()
 {
    // Encoding: 0x5EA02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd special value size = 2 (Size variant 2)
    // Fields: U=0, Rd=0, Rm=0, Rn=0, size=2
    let encoding: u32 = 0x5EA02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_special_size_3_size_variant_3_11264_5ee02c00()
 {
    // Encoding: 0x5EE02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd special value size = 3 (Size variant 3)
    // Fields: size=3, U=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x5EE02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_11264_5e602fe0()
 {
    // Encoding: 0x5E602FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, Rn=31, size=1, U=0, Rd=0
    let encoding: u32 = 0x5E602FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_11264_5e602c1f()
 {
    // Encoding: 0x5E602C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, size=1, Rm=0, U=0, Rn=0
    let encoding: u32 = 0x5E602C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_q_0_min_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Q = 0 (Min)
    // Fields: size=0, Rm=0, Q=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_q_1_max_2c00_4e202c00() {
    // Encoding: 0x4E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Q = 1 (Max)
    // Fields: Rn=0, Rd=0, U=0, size=0, Rm=0, Q=1
    let encoding: u32 = 0x4E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_u_0_min_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field U = 0 (Min)
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_u_1_max_2c00_2e202c00() {
    // Encoding: 0x2E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field U = 1 (Max)
    // Fields: Q=0, Rn=0, size=0, Rd=0, U=1, Rm=0
    let encoding: u32 = 0x2E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_size_0_min_2c00_0e202c00()
 {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field size = 0 (Min)
    // Fields: size=0, Q=0, Rm=0, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_size_1_poweroftwo_2c00_0e602c00()
 {
    // Encoding: 0x0E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field size = 1 (PowerOfTwo)
    // Fields: size=1, Rd=0, Rn=0, Rm=0, U=0, Q=0
    let encoding: u32 = 0x0E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_size_2_poweroftwo_2c00_0ea02c00()
 {
    // Encoding: 0x0EA02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field size = 2 (PowerOfTwo)
    // Fields: Rm=0, Q=0, U=0, Rn=0, Rd=0, size=2
    let encoding: u32 = 0x0EA02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_size_3_max_2c00_0ee02c00()
 {
    // Encoding: 0x0EE02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field size = 3 (Max)
    // Fields: Q=0, size=3, Rm=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0EE02C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rm_0_min_2c00_0e202c00()
{
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rm = 0 (Min)
    // Fields: size=0, Rn=0, Q=0, Rd=0, Rm=0, U=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rm_1_poweroftwo_2c00_0e212c00()
 {
    // Encoding: 0x0E212C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, Q=0, Rn=0, U=0, Rd=0, size=0
    let encoding: u32 = 0x0E212C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rm_30_poweroftwominusone_2c00_0e3e2c00()
 {
    // Encoding: 0x0E3E2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=0, size=0, Rm=30, U=0, Rd=0
    let encoding: u32 = 0x0E3E2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rm_31_max_2c00_0e3f2c00()
{
    // Encoding: 0x0E3F2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rm = 31 (Max)
    // Fields: U=0, Rm=31, size=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E3F2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rn_0_min_2c00_0e202c00()
{
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rn = 0 (Min)
    // Fields: Rm=0, Rd=0, Rn=0, Q=0, U=0, size=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rn_1_poweroftwo_2c00_0e202c20()
 {
    // Encoding: 0x0E202C20
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Q=0, U=0, size=0, Rm=0, Rn=1
    let encoding: u32 = 0x0E202C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rn_30_poweroftwominusone_2c00_0e202fc0()
 {
    // Encoding: 0x0E202FC0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=30, Rm=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0E202FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rn_31_max_2c00_0e202fe0()
{
    // Encoding: 0x0E202FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rn = 31 (Max)
    // Fields: U=0, Q=0, size=0, Rn=31, Rd=0, Rm=0
    let encoding: u32 = 0x0E202FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rd_0_min_2c00_0e202c00()
{
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rd = 0 (Min)
    // Fields: Rm=0, size=0, Q=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rd_1_poweroftwo_2c00_0e202c01()
 {
    // Encoding: 0x0E202C01
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, size=0, Rm=0, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0E202C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rd_30_poweroftwominusone_2c00_0e202c1e()
 {
    // Encoding: 0x0E202C1E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rm=0, Q=0, size=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E202C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_field_rd_31_max_2c00_0e202c1f()
{
    // Encoding: 0x0E202C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field Rd = 31 (Max)
    // Fields: U=0, size=0, Rd=31, Q=0, Rn=0, Rm=0
    let encoding: u32 = 0x0E202C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_0_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rn=0, Rd=0, U=0, Rm=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_1_2c00_4e202c00() {
    // Encoding: 0x4E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, size=0, Rn=0, U=0, Q=1
    let encoding: u32 = 0x4E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_2_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, U=0, Rn=0, Rm=0, Rd=0, Q=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_3_2c00_2e202c00() {
    // Encoding: 0x2E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=1, Rm=0, Rn=0, size=0, Q=0, Rd=0
    let encoding: u32 = 0x2E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_4_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Rm=0, Rn=0, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_5_2c00_0e602c00() {
    // Encoding: 0x0E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Q=0, Rd=0, size=1, Rn=0
    let encoding: u32 = 0x0E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_6_2c00_0ea02c00() {
    // Encoding: 0x0EA02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, size=2, Q=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x0EA02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_7_2c00_0ee02c00() {
    // Encoding: 0x0EE02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Rn=0, Rd=0, size=3, Q=0
    let encoding: u32 = 0x0EE02C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_8_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rd=0, Rm=0, Rn=0, U=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_9_2c00_0e212c00() {
    // Encoding: 0x0E212C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: U=0, size=0, Rn=0, Q=0, Rd=0, Rm=1
    let encoding: u32 = 0x0E212C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_10_2c00_0e3e2c00() {
    // Encoding: 0x0E3E2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Q=0, Rn=0, size=0, Rm=30
    let encoding: u32 = 0x0E3E2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_11_2c00_0e3f2c00() {
    // Encoding: 0x0E3F2C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rm=31, U=0, Rn=0, Q=0, Rd=0, size=0
    let encoding: u32 = 0x0E3F2C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_12_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Q=0, size=0, U=0, Rn=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_13_2c00_0e202c20() {
    // Encoding: 0x0E202C20
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: size=0, Rn=1, Rm=0, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E202C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_14_2c00_0e202fc0() {
    // Encoding: 0x0E202FC0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Q=0, size=0, Rm=0, Rn=30, Rd=0, U=0
    let encoding: u32 = 0x0E202FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_15_2c00_0e202fe0() {
    // Encoding: 0x0E202FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, Rd=0, U=0, Q=0, size=0, Rn=31
    let encoding: u32 = 0x0E202FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_16_2c00_0e202c00() {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, Rd=0, size=0, Q=0, Rm=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_17_2c00_0e202c01() {
    // Encoding: 0x0E202C01
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: size=0, Rm=0, Rn=0, Rd=1, U=0, Q=0
    let encoding: u32 = 0x0E202C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_18_2c00_0e202c1e() {
    // Encoding: 0x0E202C1E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Q=0, Rn=0, size=0, Rm=0, U=0, Rd=30
    let encoding: u32 = 0x0E202C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_19_2c00_0e202c1f() {
    // Encoding: 0x0E202C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rd=31, size=0, U=0, Rn=0, Rm=0, Q=0
    let encoding: u32 = 0x0E202C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_20_2c00_0e212c20() {
    // Encoding: 0x0E212C20
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Q=0, size=0, Rn=1, U=0, Rm=1, Rd=0
    let encoding: u32 = 0x0E212C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_21_2c00_0e3f2fe0() {
    // Encoding: 0x0E3F2FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rd=0, size=0, Rm=31, U=0, Rn=31, Q=0
    let encoding: u32 = 0x0E3F2FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_22_2c00_0e212c01() {
    // Encoding: 0x0E212C01
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Q=0, size=0, Rn=0, Rd=1, U=0, Rm=1
    let encoding: u32 = 0x0E212C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_23_2c00_0e3f2c1f() {
    // Encoding: 0x0E3F2C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Rm=31, size=0, U=0, Q=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E3F2C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_24_2c00_0e202c21() {
    // Encoding: 0x0E202C21
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: Rd=1, U=0, size=0, Rm=0, Q=0, Rn=1
    let encoding: u32 = 0x0E202C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_combo_25_2c00_0e202fff() {
    // Encoding: 0x0E202FFF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Rm=0, Q=0, U=0, size=0, Rd=31, Rn=31
    let encoding: u32 = 0x0E202FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_q_0_size_variant_0_11264_0e602c00()
 {
    // Encoding: 0x0E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value Q = 0 (Size variant 0)
    // Fields: Rd=0, Rn=0, size=1, Rm=0, Q=0, U=0
    let encoding: u32 = 0x0E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_q_1_size_variant_1_11264_4e602c00()
 {
    // Encoding: 0x4E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value Q = 1 (Size variant 1)
    // Fields: Q=1, U=0, Rm=0, size=1, Rd=0, Rn=0
    let encoding: u32 = 0x4E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_size_0_size_variant_0_11264_0e202c00()
 {
    // Encoding: 0x0E202C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value size = 0 (Size variant 0)
    // Fields: size=0, Rd=0, Rn=0, U=0, Rm=0, Q=0
    let encoding: u32 = 0x0E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_size_1_size_variant_1_11264_0e602c00()
 {
    // Encoding: 0x0E602C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value size = 1 (Size variant 1)
    // Fields: Rd=0, Q=0, U=0, Rn=0, size=1, Rm=0
    let encoding: u32 = 0x0E602C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_size_2_size_variant_2_11264_0ea02c00()
 {
    // Encoding: 0x0EA02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value size = 2 (Size variant 2)
    // Fields: Rm=0, Q=0, U=0, Rn=0, Rd=0, size=2
    let encoding: u32 = 0x0EA02C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_size_3_size_variant_3_11264_0ee02c00()
 {
    // Encoding: 0x0EE02C00
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value size = 3 (Size variant 3)
    // Fields: Q=0, U=0, size=3, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EE02C00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_rn_31_stack_pointer_sp_may_require_alignment_11264_0e602fe0()
 {
    // Encoding: 0x0E602FE0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rn=31, Rd=0, Q=0, U=0, Rm=0
    let encoding: u32 = 0x0E602FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_11264_0e602c1f()
 {
    // Encoding: 0x0E602C1F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rn=0, Rm=0, Q=0, Rd=31, U=0
    let encoding: u32 = 0x0E602C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_reg_write_0_5e202c00() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd register write: SimdFromField("d")
    // Encoding: 0x5E202C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E202C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_sp_rn_5e202fe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd with Rn = SP (31)
    // Encoding: 0x5E202FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E202FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_zr_rd_5e202c1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd with Rd = ZR (31)
    // Encoding: 0x5E202C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E202C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_reg_write_0_0e202c00() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd register write: SimdFromField("d")
    // Encoding: 0x0E202C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E202C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_sp_rn_0e202fe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd with Rn = SP (31)
    // Encoding: 0x0E202FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E202FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_zr_rd_0e202c1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd with Rd = ZR (31)
    // Encoding: 0x0E202C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E202C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_halving_truncating Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_q_0_min_400_0e200400()
{
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Q = 0 (Min)
    // Fields: Rm=0, size=0, Rn=0, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_q_1_max_400_4e200400()
{
    // Encoding: 0x4E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Q = 1 (Max)
    // Fields: Q=1, Rm=0, Rn=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x4E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_u_0_min_400_0e200400()
{
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field U = 0 (Min)
    // Fields: Rd=0, Q=0, Rn=0, U=0, Rm=0, size=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_u_1_max_400_2e200400()
{
    // Encoding: 0x2E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field U = 1 (Max)
    // Fields: size=0, U=1, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_size_0_min_400_0e200400()
 {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field size = 0 (Min)
    // Fields: Rn=0, U=0, Rd=0, Q=0, size=0, Rm=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_size_1_poweroftwo_400_0e600400()
 {
    // Encoding: 0x0E600400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field size = 1 (PowerOfTwo)
    // Fields: size=1, Rd=0, Q=0, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_size_2_poweroftwo_400_0ea00400()
 {
    // Encoding: 0x0EA00400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field size = 2 (PowerOfTwo)
    // Fields: Q=0, size=2, Rd=0, U=0, Rm=0, Rn=0
    let encoding: u32 = 0x0EA00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_size_3_max_400_0ee00400()
 {
    // Encoding: 0x0EE00400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field size = 3 (Max)
    // Fields: Rm=0, U=0, size=3, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0EE00400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rm_0_min_400_0e200400()
 {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rm = 0 (Min)
    // Fields: Rd=0, Rn=0, U=0, Rm=0, Q=0, size=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rm_1_poweroftwo_400_0e210400()
 {
    // Encoding: 0x0E210400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rm = 1 (PowerOfTwo)
    // Fields: size=0, U=0, Q=0, Rd=0, Rm=1, Rn=0
    let encoding: u32 = 0x0E210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rm_30_poweroftwominusone_400_0e3e0400()
 {
    // Encoding: 0x0E3E0400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, size=0, Rm=30, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0E3E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rm_31_max_400_0e3f0400()
 {
    // Encoding: 0x0E3F0400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rm = 31 (Max)
    // Fields: Rd=0, Rm=31, size=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0E3F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rn_0_min_400_0e200400()
 {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rn = 0 (Min)
    // Fields: Q=0, U=0, Rn=0, size=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rn_1_poweroftwo_400_0e200420()
 {
    // Encoding: 0x0E200420
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, U=0, Rm=0, size=0, Q=0, Rd=0
    let encoding: u32 = 0x0E200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rn_30_poweroftwominusone_400_0e2007c0()
 {
    // Encoding: 0x0E2007C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Q=0, U=0, Rd=0, size=0, Rm=0
    let encoding: u32 = 0x0E2007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rn_31_max_400_0e2007e0()
 {
    // Encoding: 0x0E2007E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rn = 31 (Max)
    // Fields: U=0, Rd=0, Rm=0, Q=0, size=0, Rn=31
    let encoding: u32 = 0x0E2007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rd_0_min_400_0e200400()
 {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rd = 0 (Min)
    // Fields: size=0, Q=0, U=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rd_1_poweroftwo_400_0e200401()
 {
    // Encoding: 0x0E200401
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Q=0, Rn=0, U=0, size=0, Rm=0
    let encoding: u32 = 0x0E200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rd_30_poweroftwominusone_400_0e20041e()
 {
    // Encoding: 0x0E20041E
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Rn=0, Q=0, size=0, Rd=30, U=0
    let encoding: u32 = 0x0E20041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_field_rd_31_max_400_0e20041f()
 {
    // Encoding: 0x0E20041F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field Rd = 31 (Max)
    // Fields: Rd=31, U=0, Rn=0, Q=0, Rm=0, size=0
    let encoding: u32 = 0x0E20041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_0_400_0e200400() {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, Rm=0, Q=0, Rn=0, size=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_1_400_4e200400() {
    // Encoding: 0x4E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=1, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, Q=1, U=0, Rm=0, Rd=0
    let encoding: u32 = 0x4E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_2_400_0e200400() {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Q=0, Rn=0, Rd=0, size=0, U=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_3_400_2e200400() {
    // Encoding: 0x2E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=1, Rm=0, Rd=0, Rn=0, size=0, Q=0
    let encoding: u32 = 0x2E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_4_400_0e200400() {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, size=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_5_400_0e600400() {
    // Encoding: 0x0E600400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: size=1, U=0, Rm=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_6_400_0ea00400() {
    // Encoding: 0x0EA00400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, size=2, Rn=0, U=0, Q=0, Rm=0
    let encoding: u32 = 0x0EA00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_7_400_0ee00400() {
    // Encoding: 0x0EE00400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, U=0, Rn=0, size=3, Rm=0
    let encoding: u32 = 0x0EE00400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_8_400_0e200400() {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, U=0, Rn=0, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_9_400_0e210400() {
    // Encoding: 0x0E210400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, Rn=0, U=0, Rm=1, size=0
    let encoding: u32 = 0x0E210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_10_400_0e3e0400() {
    // Encoding: 0x0E3E0400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rn=0, Rm=30, size=0, Q=0
    let encoding: u32 = 0x0E3E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_11_400_0e3f0400() {
    // Encoding: 0x0E3F0400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: Rn=0, size=0, Rd=0, Rm=31, Q=0, U=0
    let encoding: u32 = 0x0E3F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_12_400_0e200400() {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rm=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_13_400_0e200420() {
    // Encoding: 0x0E200420
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: size=0, Rn=1, Rd=0, Rm=0, Q=0, U=0
    let encoding: u32 = 0x0E200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_14_400_0e2007c0() {
    // Encoding: 0x0E2007C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Rn=30, Rm=0, Rd=0, U=0, Q=0, size=0
    let encoding: u32 = 0x0E2007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_15_400_0e2007e0() {
    // Encoding: 0x0E2007E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: Q=0, Rm=0, Rd=0, Rn=31, size=0, U=0
    let encoding: u32 = 0x0E2007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_16_400_0e200400() {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, size=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_17_400_0e200401() {
    // Encoding: 0x0E200401
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rn=0, size=0, Q=0, U=0, Rd=1, Rm=0
    let encoding: u32 = 0x0E200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_18_400_0e20041e() {
    // Encoding: 0x0E20041E
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Rn=0, U=0, size=0, Rd=30, Q=0, Rm=0
    let encoding: u32 = 0x0E20041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_19_400_0e20041f() {
    // Encoding: 0x0E20041F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, U=0, size=0, Q=0, Rm=0, Rd=31
    let encoding: u32 = 0x0E20041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_20_400_0e210420() {
    // Encoding: 0x0E210420
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: Rd=0, U=0, size=0, Q=0, Rm=1, Rn=1
    let encoding: u32 = 0x0E210420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_21_400_0e3f07e0() {
    // Encoding: 0x0E3F07E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: Rd=0, U=0, Rm=31, Rn=31, Q=0, size=0
    let encoding: u32 = 0x0E3F07E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_22_400_0e210401() {
    // Encoding: 0x0E210401
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Rm=1, Rn=0, Q=0, Rd=1, size=0, U=0
    let encoding: u32 = 0x0E210401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_23_400_0e3f041f() {
    // Encoding: 0x0E3F041F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: Q=0, U=0, Rm=31, size=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E3F041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_24_400_0e200421() {
    // Encoding: 0x0E200421
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: U=0, Q=0, size=0, Rn=1, Rd=1, Rm=0
    let encoding: u32 = 0x0E200421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_combo_25_400_0e2007ff() {
    // Encoding: 0x0E2007FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating field combination: Q=0, U=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: Q=0, Rd=31, U=0, size=0, Rm=0, Rn=31
    let encoding: u32 = 0x0E2007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_q_0_size_variant_0_1024_0e600400()
 {
    // Encoding: 0x0E600400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value Q = 0 (Size variant 0)
    // Fields: U=0, Q=0, Rd=0, size=1, Rm=0, Rn=0
    let encoding: u32 = 0x0E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_q_1_size_variant_1_1024_4e600400()
 {
    // Encoding: 0x4E600400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value Q = 1 (Size variant 1)
    // Fields: U=0, size=1, Rm=0, Q=1, Rn=0, Rd=0
    let encoding: u32 = 0x4E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_size_0_size_variant_0_1024_0e200400()
 {
    // Encoding: 0x0E200400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value size = 0 (Size variant 0)
    // Fields: size=0, Q=0, Rm=0, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x0E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_size_1_size_variant_1_1024_0e600400()
 {
    // Encoding: 0x0E600400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value size = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, Rm=0, U=0, Q=0, size=1
    let encoding: u32 = 0x0E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_size_2_size_variant_2_1024_0ea00400()
 {
    // Encoding: 0x0EA00400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value size = 2 (Size variant 2)
    // Fields: Rn=0, Rd=0, U=0, Q=0, size=2, Rm=0
    let encoding: u32 = 0x0EA00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_size_3_size_variant_3_1024_0ee00400()
 {
    // Encoding: 0x0EE00400
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value size = 3 (Size variant 3)
    // Fields: Q=0, size=3, U=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0EE00400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_rn_31_stack_pointer_sp_may_require_alignment_1024_0e6007e0()
 {
    // Encoding: 0x0E6007E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, U=0, Q=0, size=1, Rn=31, Rd=0
    let encoding: u32 = 0x0E6007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_0e60041f()
 {
    // Encoding: 0x0E60041F
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, U=0, size=1, Rm=0, Rd=31, Rn=0
    let encoding: u32 = 0x0E60041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_reg_write_0_0e200400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating register write: SimdFromField("d")
    // Encoding: 0x0E200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_sp_rn_0e2007e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating with Rn = SP (31)
    // Encoding: 0x0E2007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E2007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_zr_rd_0e20041f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_halving_truncating with Rd = ZR (31)
    // Encoding: 0x0E20041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20041F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rm_0_min_1400_7ec01400() {
    // Encoding: 0x7EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rm = 0 (Min)
    // Fields: Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x7EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rm_1_poweroftwo_1400_7ec11400()
{
    // Encoding: 0x7EC11400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rm = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x7EC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rm_30_poweroftwominusone_1400_7ede1400()
 {
    // Encoding: 0x7EDE1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rm=30, Rn=0
    let encoding: u32 = 0x7EDE1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rm_31_max_1400_7edf1400() {
    // Encoding: 0x7EDF1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rm = 31 (Max)
    // Fields: Rn=0, Rd=0, Rm=31
    let encoding: u32 = 0x7EDF1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rn_0_min_1400_7ec01400() {
    // Encoding: 0x7EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rn = 0 (Min)
    // Fields: Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x7EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rn_1_poweroftwo_1400_7ec01420()
{
    // Encoding: 0x7EC01420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, Rm=0
    let encoding: u32 = 0x7EC01420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rn_30_poweroftwominusone_1400_7ec017c0()
 {
    // Encoding: 0x7EC017C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Rn=30, Rd=0
    let encoding: u32 = 0x7EC017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rn_31_max_1400_7ec017e0() {
    // Encoding: 0x7EC017E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rn = 31 (Max)
    // Fields: Rd=0, Rm=0, Rn=31
    let encoding: u32 = 0x7EC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rd_0_min_1400_7ec01400() {
    // Encoding: 0x7EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rd = 0 (Min)
    // Fields: Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x7EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rd_1_poweroftwo_1400_7ec01401()
{
    // Encoding: 0x7EC01401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, Rd=1, Rn=0
    let encoding: u32 = 0x7EC01401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rd_30_poweroftwominusone_1400_7ec0141e()
 {
    // Encoding: 0x7EC0141E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rm=0, Rn=0
    let encoding: u32 = 0x7EC0141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_field_rd_31_max_1400_7ec0141f() {
    // Encoding: 0x7EC0141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field Rd = 31 (Max)
    // Fields: Rd=31, Rm=0, Rn=0
    let encoding: u32 = 0x7EC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_0_1400_7ec01400() {
    // Encoding: 0x7EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x7EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_1_1400_7ec11400() {
    // Encoding: 0x7EC11400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=1, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x7EC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_2_1400_7ede1400() {
    // Encoding: 0x7EDE1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, Rm=30, Rn=0
    let encoding: u32 = 0x7EDE1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_3_1400_7edf1400() {
    // Encoding: 0x7EDF1400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=31, Rn=0, Rd=0
    // Fields: Rm=31, Rn=0, Rd=0
    let encoding: u32 = 0x7EDF1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_4_1400_7ec01400() {
    // Encoding: 0x7EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x7EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_5_1400_7ec01420() {
    // Encoding: 0x7EC01420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=1, Rd=0
    // Fields: Rm=0, Rn=1, Rd=0
    let encoding: u32 = 0x7EC01420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_6_1400_7ec017c0() {
    // Encoding: 0x7EC017C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=30, Rd=0
    // Fields: Rm=0, Rn=30, Rd=0
    let encoding: u32 = 0x7EC017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_7_1400_7ec017e0() {
    // Encoding: 0x7EC017E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=31, Rd=0
    // Fields: Rm=0, Rd=0, Rn=31
    let encoding: u32 = 0x7EC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_8_1400_7ec01400() {
    // Encoding: 0x7EC01400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x7EC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_9_1400_7ec01401() {
    // Encoding: 0x7EC01401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=0, Rd=1
    // Fields: Rd=1, Rm=0, Rn=0
    let encoding: u32 = 0x7EC01401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_10_1400_7ec0141e() {
    // Encoding: 0x7EC0141E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=0, Rd=30
    // Fields: Rm=0, Rn=0, Rd=30
    let encoding: u32 = 0x7EC0141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_11_1400_7ec0141f() {
    // Encoding: 0x7EC0141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=0, Rd=31
    // Fields: Rn=0, Rd=31, Rm=0
    let encoding: u32 = 0x7EC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_12_1400_7ec11420() {
    // Encoding: 0x7EC11420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=1, Rn=1, Rd=0
    // Fields: Rm=1, Rd=0, Rn=1
    let encoding: u32 = 0x7EC11420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_13_1400_7edf17e0() {
    // Encoding: 0x7EDF17E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=31, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, Rm=31
    let encoding: u32 = 0x7EDF17E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_14_1400_7ec11401() {
    // Encoding: 0x7EC11401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=1, Rn=0, Rd=1
    // Fields: Rn=0, Rm=1, Rd=1
    let encoding: u32 = 0x7EC11401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_15_1400_7edf141f() {
    // Encoding: 0x7EDF141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=31, Rn=0, Rd=31
    // Fields: Rm=31, Rd=31, Rn=0
    let encoding: u32 = 0x7EDF141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_16_1400_7ec01421() {
    // Encoding: 0x7EC01421
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=1, Rd=1
    // Fields: Rn=1, Rm=0, Rd=1
    let encoding: u32 = 0x7EC01421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_combo_17_1400_7ec017ff() {
    // Encoding: 0x7EC017FF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd field combination: Rm=0, Rn=31, Rd=31
    // Fields: Rm=0, Rn=31, Rd=31
    let encoding: u32 = 0x7EC017FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_5120_7ec017e0()
 {
    // Encoding: 0x7EC017E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, Rm=0, Rn=31
    let encoding: u32 = 0x7EC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_5120_7ec0141f()
 {
    // Encoding: 0x7EC0141F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, Rn=0, Rd=31
    let encoding: u32 = 0x7EC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_sz_0_min_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field sz = 0 (Min)
    // Fields: sz=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_sz_1_max_d400_7ee0d400() {
    // Encoding: 0x7EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field sz = 1 (Max)
    // Fields: Rm=0, Rd=0, Rn=0, sz=1
    let encoding: u32 = 0x7EE0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rm_0_min_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rm = 0 (Min)
    // Fields: sz=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rm_1_poweroftwo_d400_7ea1d400() {
    // Encoding: 0x7EA1D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rm = 1 (PowerOfTwo)
    // Fields: sz=0, Rm=1, Rn=0, Rd=0
    let encoding: u32 = 0x7EA1D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rm_30_poweroftwominusone_d400_7ebed400()
 {
    // Encoding: 0x7EBED400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rm=30, sz=0, Rn=0
    let encoding: u32 = 0x7EBED400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rm_31_max_d400_7ebfd400() {
    // Encoding: 0x7EBFD400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rm = 31 (Max)
    // Fields: Rm=31, Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x7EBFD400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rn_0_min_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rn = 0 (Min)
    // Fields: Rn=0, sz=0, Rm=0, Rd=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rn_1_poweroftwo_d400_7ea0d420() {
    // Encoding: 0x7EA0D420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, Rm=0, sz=0
    let encoding: u32 = 0x7EA0D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rn_30_poweroftwominusone_d400_7ea0d7c0()
 {
    // Encoding: 0x7EA0D7C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, sz=0, Rd=0, Rm=0
    let encoding: u32 = 0x7EA0D7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rn_31_max_d400_7ea0d7e0() {
    // Encoding: 0x7EA0D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rn = 31 (Max)
    // Fields: sz=0, Rm=0, Rd=0, Rn=31
    let encoding: u32 = 0x7EA0D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rd_0_min_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rd = 0 (Min)
    // Fields: Rm=0, Rd=0, Rn=0, sz=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rd_1_poweroftwo_d400_7ea0d401() {
    // Encoding: 0x7EA0D401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, sz=0, Rn=0, Rm=0
    let encoding: u32 = 0x7EA0D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rd_30_poweroftwominusone_d400_7ea0d41e()
 {
    // Encoding: 0x7EA0D41E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, sz=0, Rn=0, Rd=30
    let encoding: u32 = 0x7EA0D41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_field_rd_31_max_d400_7ea0d41f() {
    // Encoding: 0x7EA0D41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field Rd = 31 (Max)
    // Fields: Rm=0, Rn=0, sz=0, Rd=31
    let encoding: u32 = 0x7EA0D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_0_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rm=0, Rd=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_1_d400_7ee0d400() {
    // Encoding: 0x7EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=1, Rm=0, Rn=0, Rd=0
    // Fields: sz=1, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x7EE0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_2_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=0
    // Fields: sz=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_3_d400_7ea1d400() {
    // Encoding: 0x7EA1D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sz=0, Rm=1
    let encoding: u32 = 0x7EA1D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_4_d400_7ebed400() {
    // Encoding: 0x7EBED400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=30, Rn=0, Rd=0
    // Fields: sz=0, Rm=30, Rn=0, Rd=0
    let encoding: u32 = 0x7EBED400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_5_d400_7ebfd400() {
    // Encoding: 0x7EBFD400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=31, Rn=0, Rd=0
    // Fields: sz=0, Rd=0, Rn=0, Rm=31
    let encoding: u32 = 0x7EBFD400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_6_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=0
    // Fields: sz=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_7_d400_7ea0d420() {
    // Encoding: 0x7EA0D420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=1, Rd=0
    // Fields: Rn=1, sz=0, Rd=0, Rm=0
    let encoding: u32 = 0x7EA0D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_8_d400_7ea0d7c0() {
    // Encoding: 0x7EA0D7C0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=30, Rd=0
    // Fields: Rn=30, Rm=0, Rd=0, sz=0
    let encoding: u32 = 0x7EA0D7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_9_d400_7ea0d7e0() {
    // Encoding: 0x7EA0D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=31, Rd=0
    // Fields: sz=0, Rm=0, Rd=0, Rn=31
    let encoding: u32 = 0x7EA0D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_10_d400_7ea0d400() {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0, Rm=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_11_d400_7ea0d401() {
    // Encoding: 0x7EA0D401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=1
    // Fields: Rm=0, sz=0, Rd=1, Rn=0
    let encoding: u32 = 0x7EA0D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_12_d400_7ea0d41e() {
    // Encoding: 0x7EA0D41E
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, sz=0, Rm=0
    let encoding: u32 = 0x7EA0D41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_13_d400_7ea0d41f() {
    // Encoding: 0x7EA0D41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=0, Rd=31
    // Fields: sz=0, Rd=31, Rm=0, Rn=0
    let encoding: u32 = 0x7EA0D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_14_d400_7ea1d420() {
    // Encoding: 0x7EA1D420
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=1, Rn=1, Rd=0
    // Fields: Rm=1, sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x7EA1D420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_15_d400_7ebfd7e0() {
    // Encoding: 0x7EBFD7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=31, Rn=31, Rd=0
    // Fields: Rn=31, Rm=31, Rd=0, sz=0
    let encoding: u32 = 0x7EBFD7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_16_d400_7ea1d401() {
    // Encoding: 0x7EA1D401
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=1, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, sz=0, Rm=1
    let encoding: u32 = 0x7EA1D401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_17_d400_7ebfd41f() {
    // Encoding: 0x7EBFD41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=31, Rn=0, Rd=31
    // Fields: Rm=31, Rd=31, Rn=0, sz=0
    let encoding: u32 = 0x7EBFD41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_18_d400_7ea0d421() {
    // Encoding: 0x7EA0D421
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=1, Rd=1
    // Fields: Rm=0, sz=0, Rn=1, Rd=1
    let encoding: u32 = 0x7EA0D421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_combo_19_d400_7ea0d7ff() {
    // Encoding: 0x7EA0D7FF
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd field combination: sz=0, Rm=0, Rn=31, Rd=31
    // Fields: Rn=31, Rm=0, sz=0, Rd=31
    let encoding: u32 = 0x7EA0D7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_special_sz_0_size_variant_0_54272_7ea0d400()
 {
    // Encoding: 0x7EA0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd special value sz = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, Rm=0, sz=0
    let encoding: u32 = 0x7EA0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_special_sz_1_size_variant_1_54272_7ee0d400()
 {
    // Encoding: 0x7EE0D400
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd special value sz = 1 (Size variant 1)
    // Fields: Rm=0, Rn=0, sz=1, Rd=0
    let encoding: u32 = 0x7EE0D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_54272_7ee0d7e0()
 {
    // Encoding: 0x7EE0D7E0
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, sz=1, Rn=31, Rd=0
    let encoding: u32 = 0x7EE0D7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_54272_7ee0d41f()
 {
    // Encoding: 0x7EE0D41F
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, Rn=0, sz=1, Rd=31
    let encoding: u32 = 0x7EE0D41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_reg_write_0_7ec01400() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd register write: SimdFromField("d")
    // Encoding: 0x7EC01400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7EC01400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_sp_rn_7ec017e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd with Rn = SP (31)
    // Encoding: 0x7EC017E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7EC017E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_zr_rd_7ec0141f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd with Rd = ZR (31)
    // Encoding: 0x7EC0141F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7EC0141F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_reg_write_0_7ea0d400() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd register write: SimdFromField("d")
    // Encoding: 0x7EA0D400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7EA0D400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_sp_rn_7ea0d7e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd with Rn = SP (31)
    // Encoding: 0x7EA0D7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7EA0D7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_zr_rd_7ea0d41f() {
    // Test aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd with Rd = ZR (31)
    // Encoding: 0x7EA0D41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7EA0D41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_fp_complex Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_q_0_min_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Q = 0 (Min)
    // Fields: Rn=0, Rm=0, size=0, Q=0, Rd=0, rot=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_q_1_max_e400_6e00e400() {
    // Encoding: 0x6E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Q = 1 (Max)
    // Fields: Rm=0, Rd=0, rot=0, size=0, Q=1, Rn=0
    let encoding: u32 = 0x6E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_size_0_min_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field size = 0 (Min)
    // Fields: Rm=0, size=0, rot=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_size_1_poweroftwo_e400_2e40e400()
 {
    // Encoding: 0x2E40E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field size = 1 (PowerOfTwo)
    // Fields: Rm=0, Q=0, rot=0, size=1, Rn=0, Rd=0
    let encoding: u32 = 0x2E40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_size_2_poweroftwo_e400_2e80e400()
 {
    // Encoding: 0x2E80E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field size = 2 (PowerOfTwo)
    // Fields: Rd=0, Q=0, size=2, Rm=0, rot=0, Rn=0
    let encoding: u32 = 0x2E80E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_size_3_max_e400_2ec0e400() {
    // Encoding: 0x2EC0E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field size = 3 (Max)
    // Fields: rot=0, Rd=0, Q=0, Rm=0, size=3, Rn=0
    let encoding: u32 = 0x2EC0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rm_0_min_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rm = 0 (Min)
    // Fields: Rn=0, Rm=0, Rd=0, size=0, rot=0, Q=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rm_1_poweroftwo_e400_2e01e400()
 {
    // Encoding: 0x2E01E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rm = 1 (PowerOfTwo)
    // Fields: Q=0, Rm=1, Rd=0, size=0, rot=0, Rn=0
    let encoding: u32 = 0x2E01E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rm_30_poweroftwominusone_e400_2e1ee400()
 {
    // Encoding: 0x2E1EE400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rm=30, size=0, rot=0, Q=0, Rd=0
    let encoding: u32 = 0x2E1EE400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rm_31_max_e400_2e1fe400() {
    // Encoding: 0x2E1FE400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rm = 31 (Max)
    // Fields: Rm=31, Q=0, rot=0, size=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E1FE400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field rot 12 +: 1`
/// Requirement: FieldBoundary { field: "rot", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rot_0_min_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field rot = 0 (Min)
    // Fields: size=0, Rm=0, Q=0, rot=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field rot 12 +: 1`
/// Requirement: FieldBoundary { field: "rot", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rot_1_max_e400_2e00f400() {
    // Encoding: 0x2E00F400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field rot = 1 (Max)
    // Fields: Rm=0, Rn=0, rot=1, Q=0, size=0, Rd=0
    let encoding: u32 = 0x2E00F400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rn_0_min_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rn = 0 (Min)
    // Fields: Rn=0, rot=0, Rm=0, Rd=0, Q=0, size=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rn_1_poweroftwo_e400_2e00e420()
 {
    // Encoding: 0x2E00E420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rm=0, Rn=1, Q=0, rot=0, Rd=0
    let encoding: u32 = 0x2E00E420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rn_30_poweroftwominusone_e400_2e00e7c0()
 {
    // Encoding: 0x2E00E7C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: rot=0, Q=0, Rn=30, Rm=0, Rd=0, size=0
    let encoding: u32 = 0x2E00E7C0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rn_31_max_e400_2e00e7e0() {
    // Encoding: 0x2E00E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rn = 31 (Max)
    // Fields: Rn=31, Rm=0, Q=0, rot=0, size=0, Rd=0
    let encoding: u32 = 0x2E00E7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rd_0_min_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rd = 0 (Min)
    // Fields: rot=0, Rd=0, size=0, Rn=0, Rm=0, Q=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rd_1_poweroftwo_e400_2e00e401()
 {
    // Encoding: 0x2E00E401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, size=0, Q=0, rot=0, Rn=0, Rm=0
    let encoding: u32 = 0x2E00E401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rd_30_poweroftwominusone_e400_2e00e41e()
 {
    // Encoding: 0x2E00E41E
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rm=0, Rn=0, size=0, Q=0, rot=0
    let encoding: u32 = 0x2E00E41E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_field_rd_31_max_e400_2e00e41f() {
    // Encoding: 0x2E00E41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field Rd = 31 (Max)
    // Fields: Q=0, Rd=31, Rn=0, size=0, Rm=0, rot=0
    let encoding: u32 = 0x2E00E41F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_0_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: size=0, rot=0, Q=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_1_e400_6e00e400() {
    // Encoding: 0x6E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=1, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: rot=0, Rn=0, Rd=0, Q=1, size=0, Rm=0
    let encoding: u32 = 0x6E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_2_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: Q=0, Rm=0, rot=0, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_3_e400_2e40e400() {
    // Encoding: 0x2E40E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=1, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: Q=0, size=1, rot=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_4_e400_2e80e400() {
    // Encoding: 0x2E80E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=2, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: Rm=0, rot=0, Q=0, size=2, Rn=0, Rd=0
    let encoding: u32 = 0x2E80E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_5_e400_2ec0e400() {
    // Encoding: 0x2EC0E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=3, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=0, size=3, rot=0, Rd=0, Q=0
    let encoding: u32 = 0x2EC0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_6_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: Rm=0, size=0, Q=0, rot=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_7_e400_2e01e400() {
    // Encoding: 0x2E01E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=1, rot=0, Rn=0, Rd=0
    // Fields: Rm=1, Rn=0, rot=0, size=0, Q=0, Rd=0
    let encoding: u32 = 0x2E01E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_8_e400_2e1ee400() {
    // Encoding: 0x2E1EE400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=30, rot=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rm=30, size=0, Rd=0, rot=0
    let encoding: u32 = 0x2E1EE400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_9_e400_2e1fe400() {
    // Encoding: 0x2E1FE400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=31, rot=0, Rn=0, Rd=0
    // Fields: size=0, Q=0, Rm=31, Rn=0, rot=0, Rd=0
    let encoding: u32 = 0x2E1FE400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rot=0 (minimum value)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_10_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: rot=0, Rm=0, Rn=0, size=0, Q=0, Rd=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rot=1 (maximum value (1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_11_e400_2e00f400() {
    // Encoding: 0x2E00F400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=1, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Q=0, size=0, rot=1, Rn=0
    let encoding: u32 = 0x2E00F400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_12_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: Q=0, rot=0, size=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_13_e400_2e00e420() {
    // Encoding: 0x2E00E420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=1, Rd=0
    // Fields: size=0, rot=0, Rm=0, Rn=1, Rd=0, Q=0
    let encoding: u32 = 0x2E00E420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_14_e400_2e00e7c0() {
    // Encoding: 0x2E00E7C0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=30, Rd=0
    // Fields: Rm=0, Q=0, rot=0, Rn=30, size=0, Rd=0
    let encoding: u32 = 0x2E00E7C0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_15_e400_2e00e7e0() {
    // Encoding: 0x2E00E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=31, Rd=0
    // Fields: Rd=0, Q=0, Rn=31, rot=0, size=0, Rm=0
    let encoding: u32 = 0x2E00E7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_16_e400_2e00e400() {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=0
    // Fields: size=0, Rd=0, rot=0, Q=0, Rm=0, Rn=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_17_e400_2e00e401() {
    // Encoding: 0x2E00E401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=1
    // Fields: Rd=1, Q=0, Rn=0, size=0, rot=0, Rm=0
    let encoding: u32 = 0x2E00E401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_18_e400_2e00e41e() {
    // Encoding: 0x2E00E41E
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=30
    // Fields: Q=0, Rd=30, Rm=0, Rn=0, rot=0, size=0
    let encoding: u32 = 0x2E00E41E;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_19_e400_2e00e41f() {
    // Encoding: 0x2E00E41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=0, Rd=31
    // Fields: Rm=0, Rd=31, size=0, Q=0, rot=0, Rn=0
    let encoding: u32 = 0x2E00E41F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_20_e400_2e01e420() {
    // Encoding: 0x2E01E420
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=1, rot=0, Rn=1, Rd=0
    // Fields: rot=0, Q=0, Rm=1, Rn=1, Rd=0, size=0
    let encoding: u32 = 0x2E01E420;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_21_e400_2e1fe7e0() {
    // Encoding: 0x2E1FE7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=31, rot=0, Rn=31, Rd=0
    // Fields: size=0, Rd=0, rot=0, Rm=31, Rn=31, Q=0
    let encoding: u32 = 0x2E1FE7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_22_e400_2e01e401() {
    // Encoding: 0x2E01E401
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=1, rot=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, rot=0, size=0, Rm=1, Q=0
    let encoding: u32 = 0x2E01E401;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_23_e400_2e1fe41f() {
    // Encoding: 0x2E1FE41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=31, rot=0, Rn=0, Rd=31
    // Fields: Rm=31, rot=0, Rn=0, size=0, Q=0, Rd=31
    let encoding: u32 = 0x2E1FE41F;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_24_e400_2e00e421() {
    // Encoding: 0x2E00E421
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=1, Rd=1
    // Fields: Q=0, Rn=1, size=0, Rm=0, rot=0, Rd=1
    let encoding: u32 = 0x2E00E421;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_combo_25_e400_2e00e7ff() {
    // Encoding: 0x2E00E7FF
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex field combination: Q=0, size=0, Rm=0, rot=0, Rn=31, Rd=31
    // Fields: rot=0, Rd=31, size=0, Rm=0, Q=0, Rn=31
    let encoding: u32 = 0x2E00E7FF;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_q_0_size_variant_0_58368_2e40e400()
 {
    // Encoding: 0x2E40E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value Q = 0 (Size variant 0)
    // Fields: Rm=0, Rd=0, rot=0, size=1, Rn=0, Q=0
    let encoding: u32 = 0x2E40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_q_1_size_variant_1_58368_6e40e400()
 {
    // Encoding: 0x6E40E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value Q = 1 (Size variant 1)
    // Fields: size=1, rot=0, Rn=0, Q=1, Rd=0, Rm=0
    let encoding: u32 = 0x6E40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_size_0_size_variant_0_58368_2e00e400()
 {
    // Encoding: 0x2E00E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value size = 0 (Size variant 0)
    // Fields: Q=0, size=0, Rn=0, Rm=0, rot=0, Rd=0
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_size_1_size_variant_1_58368_2e40e400()
 {
    // Encoding: 0x2E40E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value size = 1 (Size variant 1)
    // Fields: Q=0, size=1, Rm=0, rot=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_size_2_size_variant_2_58368_2e80e400()
 {
    // Encoding: 0x2E80E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value size = 2 (Size variant 2)
    // Fields: Q=0, rot=0, Rd=0, Rn=0, size=2, Rm=0
    let encoding: u32 = 0x2E80E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_size_3_size_variant_3_58368_2ec0e400()
 {
    // Encoding: 0x2EC0E400
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value size = 3 (Size variant 3)
    // Fields: Rm=0, Q=0, rot=0, Rd=0, Rn=0, size=3
    let encoding: u32 = 0x2EC0E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_rn_31_stack_pointer_sp_may_require_alignment_58368_2e40e7e0()
 {
    // Encoding: 0x2E40E7E0
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: rot=0, Rd=0, Rm=0, Rn=31, Q=0, size=1
    let encoding: u32 = 0x2E40E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_58368_2e40e41f()
 {
    // Encoding: 0x2E40E41F
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rm=0, rot=0, Q=0, size=1, Rd=31
    let encoding: u32 = 0x2E40E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_reg_write_0_2e00e400() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex register write: SimdFromField("d")
    // Encoding: 0x2E00E400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E00E400;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_sp_rn_2e00e7e0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex with Rn = SP (31)
    // Encoding: 0x2E00E7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E00E7E0;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_fp_complex
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_zr_rd_2e00e41f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_fp_complex with Rd = ZR (31)
    // Encoding: 0x2E00E41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E00E41F;
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

// ============================================================================
// aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair Tests
// ============================================================================

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_q_0_min_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Q = 0 (Min)
    // Fields: Rm=0, Q=0, size=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_q_1_max_bc00_4e20bc00() {
    // Encoding: 0x4E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Q = 1 (Max)
    // Fields: Rm=0, Rd=0, size=0, Rn=0, Q=1
    let encoding: u32 = 0x4E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_size_0_min_bc00_0e20bc00()
{
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field size = 0 (Min)
    // Fields: size=0, Rd=0, Rm=0, Rn=0, Q=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_size_1_poweroftwo_bc00_0e60bc00()
 {
    // Encoding: 0x0E60BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field size = 1 (PowerOfTwo)
    // Fields: Rm=0, Q=0, Rn=0, Rd=0, size=1
    let encoding: u32 = 0x0E60BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_size_2_poweroftwo_bc00_0ea0bc00()
 {
    // Encoding: 0x0EA0BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field size = 2 (PowerOfTwo)
    // Fields: Rn=0, Q=0, Rm=0, Rd=0, size=2
    let encoding: u32 = 0x0EA0BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_size_3_max_bc00_0ee0bc00()
{
    // Encoding: 0x0EE0BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field size = 3 (Max)
    // Fields: Rd=0, Rn=0, size=3, Rm=0, Q=0
    let encoding: u32 = 0x0EE0BC00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rm_0_min_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rm = 0 (Min)
    // Fields: Rm=0, Q=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rm_1_poweroftwo_bc00_0e21bc00()
 {
    // Encoding: 0x0E21BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, Q=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E21BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rm_30_poweroftwominusone_bc00_0e3ebc00()
 {
    // Encoding: 0x0E3EBC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, Rm=30, Q=0, size=0
    let encoding: u32 = 0x0E3EBC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rm_31_max_bc00_0e3fbc00() {
    // Encoding: 0x0E3FBC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rm = 31 (Max)
    // Fields: Rn=0, size=0, Rm=31, Q=0, Rd=0
    let encoding: u32 = 0x0E3FBC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rn_0_min_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rn = 0 (Min)
    // Fields: size=0, Q=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rn_1_poweroftwo_bc00_0e20bc20()
 {
    // Encoding: 0x0E20BC20
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, size=0, Q=0, Rn=1
    let encoding: u32 = 0x0E20BC20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rn_30_poweroftwominusone_bc00_0e20bfc0()
 {
    // Encoding: 0x0E20BFC0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rm=0, Rn=30, Q=0, size=0
    let encoding: u32 = 0x0E20BFC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rn_31_max_bc00_0e20bfe0() {
    // Encoding: 0x0E20BFE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rn = 31 (Max)
    // Fields: size=0, Rm=0, Q=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E20BFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rd_0_min_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rd = 0 (Min)
    // Fields: size=0, Rm=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rd_1_poweroftwo_bc00_0e20bc01()
 {
    // Encoding: 0x0E20BC01
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, size=0, Rm=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E20BC01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rd_30_poweroftwominusone_bc00_0e20bc1e()
 {
    // Encoding: 0x0E20BC1E
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Q=0, Rm=0, size=0, Rn=0
    let encoding: u32 = 0x0E20BC1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_field_rd_31_max_bc00_0e20bc1f() {
    // Encoding: 0x0E20BC1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field Rd = 31 (Max)
    // Fields: Rm=0, Rd=31, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E20BC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_0_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, size=0, Rn=0, Rm=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_1_bc00_4e20bc00() {
    // Encoding: 0x4E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=1, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, Q=1, Rd=0, Rm=0
    let encoding: u32 = 0x4E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_2_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: size=0, Rm=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_3_bc00_0e60bc00() {
    // Encoding: 0x0E60BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=1, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, size=1, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E60BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_4_bc00_0ea0bc00() {
    // Encoding: 0x0EA0BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=2, Rm=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=2, Q=0, Rm=0
    let encoding: u32 = 0x0EA0BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_5_bc00_0ee0bc00() {
    // Encoding: 0x0EE0BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=3, Rm=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, size=3
    let encoding: u32 = 0x0EE0BC00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_6_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, size=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_7_bc00_0e21bc00() {
    // Encoding: 0x0E21BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=1, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rd=0, Q=0, Rm=1
    let encoding: u32 = 0x0E21BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_8_bc00_0e3ebc00() {
    // Encoding: 0x0E3EBC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=30, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Q=0, Rm=30, size=0
    let encoding: u32 = 0x0E3EBC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_9_bc00_0e3fbc00() {
    // Encoding: 0x0E3FBC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=31, Rn=0, Rd=0
    // Fields: size=0, Rn=0, Rd=0, Q=0, Rm=31
    let encoding: u32 = 0x0E3FBC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_10_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0, Rm=0, size=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_11_bc00_0e20bc20() {
    // Encoding: 0x0E20BC20
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=1, Rd=0
    // Fields: size=0, Rn=1, Rd=0, Rm=0, Q=0
    let encoding: u32 = 0x0E20BC20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_12_bc00_0e20bfc0() {
    // Encoding: 0x0E20BFC0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=30, Rd=0
    // Fields: Q=0, Rm=0, Rn=30, Rd=0, size=0
    let encoding: u32 = 0x0E20BFC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_13_bc00_0e20bfe0() {
    // Encoding: 0x0E20BFE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=31, Rd=0
    // Fields: size=0, Rm=0, Q=0, Rd=0, Rn=31
    let encoding: u32 = 0x0E20BFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_14_bc00_0e20bc00() {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_15_bc00_0e20bc01() {
    // Encoding: 0x0E20BC01
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=1
    // Fields: Rm=0, Q=0, Rn=0, Rd=1, size=0
    let encoding: u32 = 0x0E20BC01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_16_bc00_0e20bc1e() {
    // Encoding: 0x0E20BC1E
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, Q=0, size=0, Rm=0
    let encoding: u32 = 0x0E20BC1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_17_bc00_0e20bc1f() {
    // Encoding: 0x0E20BC1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=0, Rd=31
    // Fields: Q=0, Rn=0, Rm=0, size=0, Rd=31
    let encoding: u32 = 0x0E20BC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_18_bc00_0e21bc20() {
    // Encoding: 0x0E21BC20
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=1, Rn=1, Rd=0
    // Fields: size=0, Rd=0, Q=0, Rn=1, Rm=1
    let encoding: u32 = 0x0E21BC20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_19_bc00_0e3fbfe0() {
    // Encoding: 0x0E3FBFE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=31, Rn=31, Rd=0
    // Fields: size=0, Rn=31, Q=0, Rd=0, Rm=31
    let encoding: u32 = 0x0E3FBFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_20_bc00_0e21bc01() {
    // Encoding: 0x0E21BC01
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=1, Rn=0, Rd=1
    // Fields: Rn=0, Q=0, Rm=1, size=0, Rd=1
    let encoding: u32 = 0x0E21BC01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_21_bc00_0e3fbc1f() {
    // Encoding: 0x0E3FBC1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=31, Rn=0, Rd=31
    // Fields: size=0, Rd=31, Rm=31, Q=0, Rn=0
    let encoding: u32 = 0x0E3FBC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_22_bc00_0e20bc21() {
    // Encoding: 0x0E20BC21
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=1, Rd=1
    // Fields: Rm=0, size=0, Q=0, Rn=1, Rd=1
    let encoding: u32 = 0x0E20BC21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_combo_23_bc00_0e20bfff() {
    // Encoding: 0x0E20BFFF
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair field combination: Q=0, size=0, Rm=0, Rn=31, Rd=31
    // Fields: size=0, Rn=31, Q=0, Rd=31, Rm=0
    let encoding: u32 = 0x0E20BFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_q_0_size_variant_0_48128_0e60bc00()
 {
    // Encoding: 0x0E60BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value Q = 0 (Size variant 0)
    // Fields: size=1, Rm=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E60BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_q_1_size_variant_1_48128_4e60bc00()
 {
    // Encoding: 0x4E60BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value Q = 1 (Size variant 1)
    // Fields: Q=1, size=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x4E60BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_size_0_size_variant_0_48128_0e20bc00()
 {
    // Encoding: 0x0E20BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value size = 0 (Size variant 0)
    // Fields: Rn=0, Q=0, size=0, Rm=0, Rd=0
    let encoding: u32 = 0x0E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_size_1_size_variant_1_48128_0e60bc00()
 {
    // Encoding: 0x0E60BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value size = 1 (Size variant 1)
    // Fields: Rm=0, Rn=0, Rd=0, Q=0, size=1
    let encoding: u32 = 0x0E60BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_size_2_size_variant_2_48128_0ea0bc00()
 {
    // Encoding: 0x0EA0BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value size = 2 (Size variant 2)
    // Fields: Q=0, size=2, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x0EA0BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_size_3_size_variant_3_48128_0ee0bc00()
 {
    // Encoding: 0x0EE0BC00
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value size = 3 (Size variant 3)
    // Fields: Q=0, Rd=0, Rm=0, Rn=0, size=3
    let encoding: u32 = 0x0EE0BC00;
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

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_rn_31_stack_pointer_sp_may_require_alignment_48128_0e60bfe0()
 {
    // Encoding: 0x0E60BFE0
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, size=1, Rn=31, Q=0, Rm=0
    let encoding: u32 = 0x0E60BFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_48128_0e60bc1f()
 {
    // Encoding: 0x0E60BC1F
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: size=1, Rd=31, Rn=0, Rm=0, Q=0
    let encoding: u32 = 0x0E60BC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_reg_write_0_0e20bc00() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair register write: SimdFromField("d")
    // Encoding: 0x0E20BC00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20BC00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_sp_rn_0e20bfe0() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair with Rn = SP (31)
    // Encoding: 0x0E20BFE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20BFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_zr_rd_0e20bc1f() {
    // Test aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair with Rd = ZR (31)
    // Encoding: 0x0E20BC1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E20BC1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
