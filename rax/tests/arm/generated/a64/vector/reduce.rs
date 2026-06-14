//! A64 vector reduce tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_vector_reduce_add_long Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_add_long_field_q_0_min_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field Q = 0 (Min)
    // Fields: Q=0, U=0, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_add_long_field_q_1_max_3800_4e303800() {
    // Encoding: 0x4E303800
    // Test aarch64_vector_reduce_add_long field Q = 1 (Max)
    // Fields: Q=1, Rn=0, U=0, size=0, Rd=0
    let encoding: u32 = 0x4E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_add_long_field_u_0_min_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field U = 0 (Min)
    // Fields: Q=0, Rd=0, Rn=0, size=0, U=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_add_long_field_u_1_max_3800_2e303800() {
    // Encoding: 0x2E303800
    // Test aarch64_vector_reduce_add_long field U = 1 (Max)
    // Fields: U=1, size=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_add_long_field_size_0_min_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field size = 0 (Min)
    // Fields: Q=0, U=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_add_long_field_size_1_poweroftwo_3800_0e703800() {
    // Encoding: 0x0E703800
    // Test aarch64_vector_reduce_add_long field size = 1 (PowerOfTwo)
    // Fields: Rn=0, size=1, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0E703800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_reduce_add_long_field_size_2_poweroftwo_3800_0eb03800() {
    // Encoding: 0x0EB03800
    // Test aarch64_vector_reduce_add_long field size = 2 (PowerOfTwo)
    // Fields: Rd=0, Q=0, U=0, size=2, Rn=0
    let encoding: u32 = 0x0EB03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_reduce_add_long_field_size_3_max_3800_0ef03800() {
    // Encoding: 0x0EF03800
    // Test aarch64_vector_reduce_add_long field size = 3 (Max)
    // Fields: Q=0, size=3, Rd=0, Rn=0, U=0
    let encoding: u32 = 0x0EF03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rn_0_min_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field Rn = 0 (Min)
    // Fields: Rd=0, U=0, Rn=0, Q=0, size=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rn_1_poweroftwo_3800_0e303820() {
    // Encoding: 0x0E303820
    // Test aarch64_vector_reduce_add_long field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Q=0, U=0, Rn=1, Rd=0
    let encoding: u32 = 0x0E303820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rn_30_poweroftwominusone_3800_0e303bc0() {
    // Encoding: 0x0E303BC0
    // Test aarch64_vector_reduce_add_long field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Q=0, Rn=30, Rd=0, size=0
    let encoding: u32 = 0x0E303BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rn_31_max_3800_0e303be0() {
    // Encoding: 0x0E303BE0
    // Test aarch64_vector_reduce_add_long field Rn = 31 (Max)
    // Fields: Q=0, U=0, size=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E303BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rd_0_min_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field Rd = 0 (Min)
    // Fields: size=0, Rd=0, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rd_1_poweroftwo_3800_0e303801() {
    // Encoding: 0x0E303801
    // Test aarch64_vector_reduce_add_long field Rd = 1 (PowerOfTwo)
    // Fields: size=0, Q=0, Rn=0, Rd=1, U=0
    let encoding: u32 = 0x0E303801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rd_30_poweroftwominusone_3800_0e30381e() {
    // Encoding: 0x0E30381E
    // Test aarch64_vector_reduce_add_long field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, size=0, Rn=0, Q=0, Rd=30
    let encoding: u32 = 0x0E30381E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_add_long_field_rd_31_max_3800_0e30381f() {
    // Encoding: 0x0E30381F
    // Test aarch64_vector_reduce_add_long field Rd = 31 (Max)
    // Fields: size=0, U=0, Rn=0, Q=0, Rd=31
    let encoding: u32 = 0x0E30381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_0_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rd=0, size=0, Rn=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_1_3800_4e303800() {
    // Encoding: 0x4E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=1, U=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Q=1, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x4E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_2_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Rd=0, U=0, Q=0, Rn=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_3_3800_2e303800() {
    // Encoding: 0x2E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=1, size=0, Rn=0, Rd=0
    // Fields: Rd=0, U=1, size=0, Rn=0, Q=0
    let encoding: u32 = 0x2E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_4_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_5_3800_0e703800() {
    // Encoding: 0x0E703800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=1, Rn=0, Rd=0
    // Fields: size=1, U=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0E703800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_6_3800_0eb03800() {
    // Encoding: 0x0EB03800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=2, Rn=0, Rd=0
    // Fields: Rn=0, U=0, Rd=0, Q=0, size=2
    let encoding: u32 = 0x0EB03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_reduce_add_long_combo_7_3800_0ef03800() {
    // Encoding: 0x0EF03800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=3, Rn=0, Rd=0
    // Fields: size=3, U=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_8_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Q=0, size=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_9_3800_0e303820() {
    // Encoding: 0x0E303820
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=1, Rd=0
    // Fields: U=0, Q=0, Rd=0, size=0, Rn=1
    let encoding: u32 = 0x0E303820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_10_3800_0e303bc0() {
    // Encoding: 0x0E303BC0
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=30, Rd=0
    // Fields: Rn=30, U=0, size=0, Rd=0, Q=0
    let encoding: u32 = 0x0E303BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_11_3800_0e303be0() {
    // Encoding: 0x0E303BE0
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=31, Rd=0
    // Fields: U=0, Rn=31, Q=0, size=0, Rd=0
    let encoding: u32 = 0x0E303BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_12_3800_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_13_3800_0e303801() {
    // Encoding: 0x0E303801
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, size=0, Q=0, U=0
    let encoding: u32 = 0x0E303801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_14_3800_0e30381e() {
    // Encoding: 0x0E30381E
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=30
    // Fields: U=0, Q=0, size=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E30381E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_15_3800_0e30381f() {
    // Encoding: 0x0E30381F
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=0, Rd=31
    // Fields: Q=0, size=0, Rd=31, Rn=0, U=0
    let encoding: u32 = 0x0E30381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_16_3800_0e303821() {
    // Encoding: 0x0E303821
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, size=0, U=0, Q=0
    let encoding: u32 = 0x0E303821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_add_long_combo_17_3800_0e303bff() {
    // Encoding: 0x0E303BFF
    // Test aarch64_vector_reduce_add_long field combination: Q=0, U=0, size=0, Rn=31, Rd=31
    // Fields: Rn=31, Q=0, size=0, Rd=31, U=0
    let encoding: u32 = 0x0E303BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_add_long_special_q_0_size_variant_0_14336_0e703800() {
    // Encoding: 0x0E703800
    // Test aarch64_vector_reduce_add_long special value Q = 0 (Size variant 0)
    // Fields: U=0, size=1, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0E703800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_add_long_special_q_1_size_variant_1_14336_4e703800() {
    // Encoding: 0x4E703800
    // Test aarch64_vector_reduce_add_long special value Q = 1 (Size variant 1)
    // Fields: U=0, Rn=0, Q=1, size=1, Rd=0
    let encoding: u32 = 0x4E703800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_add_long_special_size_0_size_variant_0_14336_0e303800() {
    // Encoding: 0x0E303800
    // Test aarch64_vector_reduce_add_long special value size = 0 (Size variant 0)
    // Fields: Rn=0, Q=0, Rd=0, U=0, size=0
    let encoding: u32 = 0x0E303800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_add_long_special_size_1_size_variant_1_14336_0e703800() {
    // Encoding: 0x0E703800
    // Test aarch64_vector_reduce_add_long special value size = 1 (Size variant 1)
    // Fields: U=0, size=1, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0E703800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_reduce_add_long_special_size_2_size_variant_2_14336_0eb03800() {
    // Encoding: 0x0EB03800
    // Test aarch64_vector_reduce_add_long special value size = 2 (Size variant 2)
    // Fields: Rd=0, Q=0, Rn=0, size=2, U=0
    let encoding: u32 = 0x0EB03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_reduce_add_long_special_size_3_size_variant_3_14336_0ef03800() {
    // Encoding: 0x0EF03800
    // Test aarch64_vector_reduce_add_long special value size = 3 (Size variant 3)
    // Fields: size=3, U=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0EF03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_add_long_special_rn_31_stack_pointer_sp_may_require_alignment_14336_0e703be0()
 {
    // Encoding: 0x0E703BE0
    // Test aarch64_vector_reduce_add_long special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rd=0, Rn=31, Q=0, U=0
    let encoding: u32 = 0x0E703BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_add_long_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_14336_0e70381f()
 {
    // Encoding: 0x0E70381F
    // Test aarch64_vector_reduce_add_long special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: U=0, size=1, Q=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E70381F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_add_long_reg_write_0_0e303800() {
    // Test aarch64_vector_reduce_add_long register write: SimdFromField("d")
    // Encoding: 0x0E303800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E303800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_add_long_sp_rn_0e303be0() {
    // Test aarch64_vector_reduce_add_long with Rn = SP (31)
    // Encoding: 0x0E303BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E303BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_add_long
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_add_long_zr_rd_0e30381f() {
    // Test aarch64_vector_reduce_add_long with Rd = ZR (31)
    // Encoding: 0x0E30381F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30381F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_reduce_fp16_add_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_sz_0_min_d800_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd field sz = 0 (Min)
    // Fields: sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_sz_1_max_d800_5e70d800() {
    // Encoding: 0x5E70D800
    // Test aarch64_vector_reduce_fp16_add_sisd field sz = 1 (Max)
    // Fields: sz=1, Rn=0, Rd=0
    let encoding: u32 = 0x5E70D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rn_0_min_d800_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rn_1_poweroftwo_d800_5e30d820() {
    // Encoding: 0x5E30D820
    // Test aarch64_vector_reduce_fp16_add_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, sz=0, Rd=0
    let encoding: u32 = 0x5E30D820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rn_30_poweroftwominusone_d800_5e30dbc0() {
    // Encoding: 0x5E30DBC0
    // Test aarch64_vector_reduce_fp16_add_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, sz=0
    let encoding: u32 = 0x5E30DBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rn_31_max_d800_5e30dbe0() {
    // Encoding: 0x5E30DBE0
    // Test aarch64_vector_reduce_fp16_add_sisd field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, sz=0
    let encoding: u32 = 0x5E30DBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rd_0_min_d800_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd field Rd = 0 (Min)
    // Fields: Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rd_1_poweroftwo_d800_5e30d801() {
    // Encoding: 0x5E30D801
    // Test aarch64_vector_reduce_fp16_add_sisd field Rd = 1 (PowerOfTwo)
    // Fields: sz=0, Rd=1, Rn=0
    let encoding: u32 = 0x5E30D801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rd_30_poweroftwominusone_d800_5e30d81e() {
    // Encoding: 0x5E30D81E
    // Test aarch64_vector_reduce_fp16_add_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, sz=0, Rd=30
    let encoding: u32 = 0x5E30D81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_field_rd_31_max_d800_5e30d81f() {
    // Encoding: 0x5E30D81F
    // Test aarch64_vector_reduce_fp16_add_sisd field Rd = 31 (Max)
    // Fields: Rd=31, sz=0, Rn=0
    let encoding: u32 = 0x5E30D81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_0_d800_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_1_d800_5e70d800() {
    // Encoding: 0x5E70D800
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=1, Rn=0, Rd=0
    // Fields: sz=1, Rd=0, Rn=0
    let encoding: u32 = 0x5E70D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_2_d800_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_3_d800_5e30d820() {
    // Encoding: 0x5E30D820
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=1, Rd=0
    // Fields: sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x5E30D820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_4_d800_5e30dbc0() {
    // Encoding: 0x5E30DBC0
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, sz=0
    let encoding: u32 = 0x5E30DBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_5_d800_5e30dbe0() {
    // Encoding: 0x5E30DBE0
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=31, Rd=0
    // Fields: sz=0, Rn=31, Rd=0
    let encoding: u32 = 0x5E30DBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_6_d800_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_7_d800_5e30d801() {
    // Encoding: 0x5E30D801
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, sz=0
    let encoding: u32 = 0x5E30D801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_8_d800_5e30d81e() {
    // Encoding: 0x5E30D81E
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=0, Rd=30
    // Fields: Rd=30, sz=0, Rn=0
    let encoding: u32 = 0x5E30D81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_9_d800_5e30d81f() {
    // Encoding: 0x5E30D81F
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=0, Rd=31
    // Fields: sz=0, Rd=31, Rn=0
    let encoding: u32 = 0x5E30D81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_10_d800_5e30d821() {
    // Encoding: 0x5E30D821
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=1, Rd=1
    // Fields: Rn=1, sz=0, Rd=1
    let encoding: u32 = 0x5E30D821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_combo_11_d800_5e30dbff() {
    // Encoding: 0x5E30DBFF
    // Test aarch64_vector_reduce_fp16_add_sisd field combination: sz=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, sz=0
    let encoding: u32 = 0x5E30DBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_special_sz_0_size_variant_0_55296_5e30d800() {
    // Encoding: 0x5E30D800
    // Test aarch64_vector_reduce_fp16_add_sisd special value sz = 0 (Size variant 0)
    // Fields: Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x5E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_special_sz_1_size_variant_1_55296_5e70d800() {
    // Encoding: 0x5E70D800
    // Test aarch64_vector_reduce_fp16_add_sisd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, sz=1, Rd=0
    let encoding: u32 = 0x5E70D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_55296_5e70dbe0()
 {
    // Encoding: 0x5E70DBE0
    // Test aarch64_vector_reduce_fp16_add_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, sz=1, Rd=0
    let encoding: u32 = 0x5E70DBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_55296_5e70d81f()
 {
    // Encoding: 0x5E70D81F
    // Test aarch64_vector_reduce_fp16_add_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0, sz=1
    let encoding: u32 = 0x5E70D81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_sz_0_min_d800_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd field sz = 0 (Min)
    // Fields: sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_sz_1_max_d800_7e70d800() {
    // Encoding: 0x7E70D800
    // Test aarch64_vector_reduce_fp_add_sisd field sz = 1 (Max)
    // Fields: Rn=0, Rd=0, sz=1
    let encoding: u32 = 0x7E70D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rn_0_min_d800_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd field Rn = 0 (Min)
    // Fields: Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rn_1_poweroftwo_d800_7e30d820() {
    // Encoding: 0x7E30D820
    // Test aarch64_vector_reduce_fp_add_sisd field Rn = 1 (PowerOfTwo)
    // Fields: sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x7E30D820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rn_30_poweroftwominusone_d800_7e30dbc0() {
    // Encoding: 0x7E30DBC0
    // Test aarch64_vector_reduce_fp_add_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, sz=0
    let encoding: u32 = 0x7E30DBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rn_31_max_d800_7e30dbe0() {
    // Encoding: 0x7E30DBE0
    // Test aarch64_vector_reduce_fp_add_sisd field Rn = 31 (Max)
    // Fields: Rd=0, sz=0, Rn=31
    let encoding: u32 = 0x7E30DBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rd_0_min_d800_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd field Rd = 0 (Min)
    // Fields: sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rd_1_poweroftwo_d800_7e30d801() {
    // Encoding: 0x7E30D801
    // Test aarch64_vector_reduce_fp_add_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Rn=0, sz=0
    let encoding: u32 = 0x7E30D801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rd_30_poweroftwominusone_d800_7e30d81e() {
    // Encoding: 0x7E30D81E
    // Test aarch64_vector_reduce_fp_add_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, sz=0
    let encoding: u32 = 0x7E30D81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_field_rd_31_max_d800_7e30d81f() {
    // Encoding: 0x7E30D81F
    // Test aarch64_vector_reduce_fp_add_sisd field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31, sz=0
    let encoding: u32 = 0x7E30D81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_0_d800_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_1_d800_7e70d800() {
    // Encoding: 0x7E70D800
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=1, Rn=0, Rd=0
    // Fields: Rd=0, sz=1, Rn=0
    let encoding: u32 = 0x7E70D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_2_d800_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_3_d800_7e30d820() {
    // Encoding: 0x7E30D820
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=1, Rd=0
    // Fields: sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x7E30D820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_4_d800_7e30dbc0() {
    // Encoding: 0x7E30DBC0
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, sz=0
    let encoding: u32 = 0x7E30DBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_5_d800_7e30dbe0() {
    // Encoding: 0x7E30DBE0
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=31, Rd=0
    // Fields: sz=0, Rn=31, Rd=0
    let encoding: u32 = 0x7E30DBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_6_d800_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_7_d800_7e30d801() {
    // Encoding: 0x7E30D801
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=0, Rd=1
    // Fields: Rn=0, sz=0, Rd=1
    let encoding: u32 = 0x7E30D801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_8_d800_7e30d81e() {
    // Encoding: 0x7E30D81E
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, sz=0
    let encoding: u32 = 0x7E30D81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_9_d800_7e30d81f() {
    // Encoding: 0x7E30D81F
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=0, Rd=31
    // Fields: Rn=0, sz=0, Rd=31
    let encoding: u32 = 0x7E30D81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_10_d800_7e30d821() {
    // Encoding: 0x7E30D821
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=1, Rd=1
    // Fields: Rd=1, sz=0, Rn=1
    let encoding: u32 = 0x7E30D821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_combo_11_d800_7e30dbff() {
    // Encoding: 0x7E30DBFF
    // Test aarch64_vector_reduce_fp_add_sisd field combination: sz=0, Rn=31, Rd=31
    // Fields: Rn=31, sz=0, Rd=31
    let encoding: u32 = 0x7E30DBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_special_sz_0_size_variant_0_55296_7e30d800() {
    // Encoding: 0x7E30D800
    // Test aarch64_vector_reduce_fp_add_sisd special value sz = 0 (Size variant 0)
    // Fields: sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E30D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_special_sz_1_size_variant_1_55296_7e70d800() {
    // Encoding: 0x7E70D800
    // Test aarch64_vector_reduce_fp_add_sisd special value sz = 1 (Size variant 1)
    // Fields: Rd=0, Rn=0, sz=1
    let encoding: u32 = 0x7E70D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_55296_7e70dbe0()
 {
    // Encoding: 0x7E70DBE0
    // Test aarch64_vector_reduce_fp_add_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, Rd=0, Rn=31
    let encoding: u32 = 0x7E70DBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_55296_7e70d81f()
 {
    // Encoding: 0x7E70D81F
    // Test aarch64_vector_reduce_fp_add_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0, sz=1
    let encoding: u32 = 0x7E70D81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_reg_write_0_5e30d800() {
    // Test aarch64_vector_reduce_fp16_add_sisd register write: SimdFromField("d")
    // Encoding: 0x5E30D800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30D800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_sp_rn_5e30dbe0() {
    // Test aarch64_vector_reduce_fp16_add_sisd with Rn = SP (31)
    // Encoding: 0x5E30DBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30DBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_add_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_add_sisd_zr_rd_5e30d81f() {
    // Test aarch64_vector_reduce_fp16_add_sisd with Rd = ZR (31)
    // Encoding: 0x5E30D81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30D81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_reg_write_0_7e30d800() {
    // Test aarch64_vector_reduce_fp_add_sisd register write: SimdFromField("d")
    // Encoding: 0x7E30D800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30D800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_sp_rn_7e30dbe0() {
    // Test aarch64_vector_reduce_fp_add_sisd with Rn = SP (31)
    // Encoding: 0x7E30DBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30DBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp_add_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp_add_sisd_zr_rd_7e30d81f() {
    // Test aarch64_vector_reduce_fp_add_sisd with Rd = ZR (31)
    // Encoding: 0x7E30D81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30D81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_reduce_fp16_maxnm_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_o1_0_min_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field o1 = 0 (Min)
    // Fields: o1=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_o1_1_max_c800_5eb0c800() {
    // Encoding: 0x5EB0C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field o1 = 1 (Max)
    // Fields: Rd=0, sz=0, o1=1, Rn=0
    let encoding: u32 = 0x5EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_sz_0_min_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field sz = 0 (Min)
    // Fields: Rd=0, Rn=0, o1=0, sz=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_sz_1_max_c800_5e70c800() {
    // Encoding: 0x5E70C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field sz = 1 (Max)
    // Fields: Rn=0, sz=1, o1=0, Rd=0
    let encoding: u32 = 0x5E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rn_0_min_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rn = 0 (Min)
    // Fields: Rn=0, o1=0, Rd=0, sz=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rn_1_poweroftwo_c800_5e30c820() {
    // Encoding: 0x5E30C820
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0, o1=0, sz=0
    let encoding: u32 = 0x5E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rn_30_poweroftwominusone_c800_5e30cbc0() {
    // Encoding: 0x5E30CBC0
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Rd=0, Rn=30, sz=0
    let encoding: u32 = 0x5E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rn_31_max_c800_5e30cbe0() {
    // Encoding: 0x5E30CBE0
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rn = 31 (Max)
    // Fields: o1=0, Rd=0, Rn=31, sz=0
    let encoding: u32 = 0x5E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rd_0_min_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rd = 0 (Min)
    // Fields: o1=0, Rd=0, Rn=0, sz=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rd_1_poweroftwo_c800_5e30c801() {
    // Encoding: 0x5E30C801
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, sz=0, Rd=1, o1=0
    let encoding: u32 = 0x5E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rd_30_poweroftwominusone_c800_5e30c81e() {
    // Encoding: 0x5E30C81E
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Rd=30, sz=0, Rn=0
    let encoding: u32 = 0x5E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_field_rd_31_max_c800_5e30c81f() {
    // Encoding: 0x5E30C81F
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field Rd = 31 (Max)
    // Fields: o1=0, sz=0, Rn=0, Rd=31
    let encoding: u32 = 0x5E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_0_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_1_c800_5eb0c800() {
    // Encoding: 0x5EB0C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=1, sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, o1=1, Rd=0
    let encoding: u32 = 0x5EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_2_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_3_c800_5e70c800() {
    // Encoding: 0x5E70C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=1, Rn=0, Rd=0
    // Fields: sz=1, Rd=0, Rn=0, o1=0
    let encoding: u32 = 0x5E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_4_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, o1=0, Rn=0, sz=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_5_c800_5e30c820() {
    // Encoding: 0x5E30C820
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=1, Rd=0
    // Fields: o1=0, Rd=0, sz=0, Rn=1
    let encoding: u32 = 0x5E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_6_c800_5e30cbc0() {
    // Encoding: 0x5E30CBC0
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, o1=0, sz=0
    let encoding: u32 = 0x5E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_7_c800_5e30cbe0() {
    // Encoding: 0x5E30CBE0
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=31, Rd=0
    // Fields: Rn=31, sz=0, o1=0, Rd=0
    let encoding: u32 = 0x5E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_8_c800_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_9_c800_5e30c801() {
    // Encoding: 0x5E30C801
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=1
    // Fields: o1=0, sz=0, Rn=0, Rd=1
    let encoding: u32 = 0x5E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_10_c800_5e30c81e() {
    // Encoding: 0x5E30C81E
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=30
    // Fields: sz=0, Rd=30, Rn=0, o1=0
    let encoding: u32 = 0x5E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_11_c800_5e30c81f() {
    // Encoding: 0x5E30C81F
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=31
    // Fields: Rd=31, o1=0, sz=0, Rn=0
    let encoding: u32 = 0x5E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_12_c800_5e30c821() {
    // Encoding: 0x5E30C821
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=1, Rd=1
    // Fields: Rn=1, o1=0, Rd=1, sz=0
    let encoding: u32 = 0x5E30C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_combo_13_c800_5e30cbff() {
    // Encoding: 0x5E30CBFF
    // Test aarch64_vector_reduce_fp16_maxnm_sisd field combination: o1=0, sz=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, sz=0, o1=0
    let encoding: u32 = 0x5E30CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_special_sz_0_size_variant_0_51200_5e30c800() {
    // Encoding: 0x5E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd special value sz = 0 (Size variant 0)
    // Fields: Rd=0, sz=0, Rn=0, o1=0
    let encoding: u32 = 0x5E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_special_sz_1_size_variant_1_51200_5e70c800() {
    // Encoding: 0x5E70C800
    // Test aarch64_vector_reduce_fp16_maxnm_sisd special value sz = 1 (Size variant 1)
    // Fields: o1=0, sz=1, Rn=0, Rd=0
    let encoding: u32 = 0x5E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_5e70cbe0()
 {
    // Encoding: 0x5E70CBE0
    // Test aarch64_vector_reduce_fp16_maxnm_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: o1=0, Rn=31, sz=1, Rd=0
    let encoding: u32 = 0x5E70CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_5e70c81f()
 {
    // Encoding: 0x5E70C81F
    // Test aarch64_vector_reduce_fp16_maxnm_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: o1=0, sz=1, Rn=0, Rd=31
    let encoding: u32 = 0x5E70C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_o1_0_min_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field o1 = 0 (Min)
    // Fields: sz=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_o1_1_max_c800_7eb0c800() {
    // Encoding: 0x7EB0C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field o1 = 1 (Max)
    // Fields: Rn=0, sz=0, Rd=0, o1=1
    let encoding: u32 = 0x7EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_sz_0_min_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field sz = 0 (Min)
    // Fields: sz=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_sz_1_max_c800_7e70c800() {
    // Encoding: 0x7E70C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field sz = 1 (Max)
    // Fields: Rd=0, o1=0, sz=1, Rn=0
    let encoding: u32 = 0x7E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rn_0_min_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, o1=0, sz=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rn_1_poweroftwo_c800_7e30c820() {
    // Encoding: 0x7E30C820
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rn = 1 (PowerOfTwo)
    // Fields: o1=0, sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x7E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rn_30_poweroftwominusone_c800_7e30cbc0() {
    // Encoding: 0x7E30CBC0
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, o1=0, sz=0, Rd=0
    let encoding: u32 = 0x7E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rn_31_max_c800_7e30cbe0() {
    // Encoding: 0x7E30CBE0
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rn = 31 (Max)
    // Fields: Rn=31, o1=0, sz=0, Rd=0
    let encoding: u32 = 0x7E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rd_0_min_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rd = 0 (Min)
    // Fields: o1=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rd_1_poweroftwo_c800_7e30c801() {
    // Encoding: 0x7E30C801
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rd = 1 (PowerOfTwo)
    // Fields: o1=0, Rn=0, Rd=1, sz=0
    let encoding: u32 = 0x7E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rd_30_poweroftwominusone_c800_7e30c81e() {
    // Encoding: 0x7E30C81E
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, Rn=0, o1=0, Rd=30
    let encoding: u32 = 0x7E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_field_rd_31_max_c800_7e30c81f() {
    // Encoding: 0x7E30C81F
    // Test aarch64_vector_reduce_fp_maxnm_sisd field Rd = 31 (Max)
    // Fields: o1=0, Rn=0, sz=0, Rd=31
    let encoding: u32 = 0x7E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_0_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, o1=0, sz=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_1_c800_7eb0c800() {
    // Encoding: 0x7EB0C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=1, sz=0, Rn=0, Rd=0
    // Fields: o1=1, Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x7EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_2_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, o1=0, sz=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_3_c800_7e70c800() {
    // Encoding: 0x7E70C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sz=1, o1=0
    let encoding: u32 = 0x7E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_4_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_5_c800_7e30c820() {
    // Encoding: 0x7E30C820
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=1, Rd=0
    // Fields: sz=0, Rd=0, Rn=1, o1=0
    let encoding: u32 = 0x7E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_6_c800_7e30cbc0() {
    // Encoding: 0x7E30CBC0
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=30, Rd=0
    // Fields: Rd=0, sz=0, o1=0, Rn=30
    let encoding: u32 = 0x7E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_7_c800_7e30cbe0() {
    // Encoding: 0x7E30CBE0
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=31, Rd=0
    // Fields: sz=0, Rn=31, Rd=0, o1=0
    let encoding: u32 = 0x7E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_8_c800_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0, o1=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_9_c800_7e30c801() {
    // Encoding: 0x7E30C801
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=1
    // Fields: sz=0, Rn=0, o1=0, Rd=1
    let encoding: u32 = 0x7E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_10_c800_7e30c81e() {
    // Encoding: 0x7E30C81E
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=30
    // Fields: o1=0, Rd=30, sz=0, Rn=0
    let encoding: u32 = 0x7E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_11_c800_7e30c81f() {
    // Encoding: 0x7E30C81F
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=0, Rd=31
    // Fields: Rd=31, Rn=0, sz=0, o1=0
    let encoding: u32 = 0x7E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_12_c800_7e30c821() {
    // Encoding: 0x7E30C821
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=1, Rd=1
    // Fields: Rn=1, o1=0, sz=0, Rd=1
    let encoding: u32 = 0x7E30C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_combo_13_c800_7e30cbff() {
    // Encoding: 0x7E30CBFF
    // Test aarch64_vector_reduce_fp_maxnm_sisd field combination: o1=0, sz=0, Rn=31, Rd=31
    // Fields: sz=0, Rd=31, Rn=31, o1=0
    let encoding: u32 = 0x7E30CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_special_sz_0_size_variant_0_51200_7e30c800() {
    // Encoding: 0x7E30C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd special value sz = 0 (Size variant 0)
    // Fields: sz=0, Rd=0, Rn=0, o1=0
    let encoding: u32 = 0x7E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_special_sz_1_size_variant_1_51200_7e70c800() {
    // Encoding: 0x7E70C800
    // Test aarch64_vector_reduce_fp_maxnm_sisd special value sz = 1 (Size variant 1)
    // Fields: o1=0, Rn=0, sz=1, Rd=0
    let encoding: u32 = 0x7E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_7e70cbe0()
 {
    // Encoding: 0x7E70CBE0
    // Test aarch64_vector_reduce_fp_maxnm_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rd=0, sz=1, o1=0
    let encoding: u32 = 0x7E70CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_7e70c81f()
 {
    // Encoding: 0x7E70C81F
    // Test aarch64_vector_reduce_fp_maxnm_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, o1=0, Rn=0, sz=1
    let encoding: u32 = 0x7E70C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_reg_write_0_5e30c800() {
    // Test aarch64_vector_reduce_fp16_maxnm_sisd register write: SimdFromField("d")
    // Encoding: 0x5E30C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_sp_rn_5e30cbe0() {
    // Test aarch64_vector_reduce_fp16_maxnm_sisd with Rn = SP (31)
    // Encoding: 0x5E30CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_sisd_zr_rd_5e30c81f() {
    // Test aarch64_vector_reduce_fp16_maxnm_sisd with Rd = ZR (31)
    // Encoding: 0x5E30C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_reg_write_0_7e30c800() {
    // Test aarch64_vector_reduce_fp_maxnm_sisd register write: SimdFromField("d")
    // Encoding: 0x7E30C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_sp_rn_7e30cbe0() {
    // Test aarch64_vector_reduce_fp_maxnm_sisd with Rn = SP (31)
    // Encoding: 0x7E30CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_sisd_zr_rd_7e30c81f() {
    // Test aarch64_vector_reduce_fp_maxnm_sisd with Rd = ZR (31)
    // Encoding: 0x7E30C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_reduce_fp16_max_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_o1_0_min_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field o1 = 0 (Min)
    // Fields: Rn=0, sz=0, o1=0, Rd=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_o1_1_max_f800_5eb0f800() {
    // Encoding: 0x5EB0F800
    // Test aarch64_vector_reduce_fp16_max_sisd field o1 = 1 (Max)
    // Fields: o1=1, Rd=0, Rn=0, sz=0
    let encoding: u32 = 0x5EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_sz_0_min_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field sz = 0 (Min)
    // Fields: Rn=0, o1=0, sz=0, Rd=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_sz_1_max_f800_5e70f800() {
    // Encoding: 0x5E70F800
    // Test aarch64_vector_reduce_fp16_max_sisd field sz = 1 (Max)
    // Fields: o1=0, Rd=0, Rn=0, sz=1
    let encoding: u32 = 0x5E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rn_0_min_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field Rn = 0 (Min)
    // Fields: o1=0, Rd=0, Rn=0, sz=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rn_1_poweroftwo_f800_5e30f820() {
    // Encoding: 0x5E30F820
    // Test aarch64_vector_reduce_fp16_max_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, o1=0, Rd=0, sz=0
    let encoding: u32 = 0x5E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rn_30_poweroftwominusone_f800_5e30fbc0() {
    // Encoding: 0x5E30FBC0
    // Test aarch64_vector_reduce_fp16_max_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, o1=0, Rd=0, Rn=30
    let encoding: u32 = 0x5E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rn_31_max_f800_5e30fbe0() {
    // Encoding: 0x5E30FBE0
    // Test aarch64_vector_reduce_fp16_max_sisd field Rn = 31 (Max)
    // Fields: o1=0, sz=0, Rd=0, Rn=31
    let encoding: u32 = 0x5E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rd_0_min_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field Rd = 0 (Min)
    // Fields: Rd=0, o1=0, sz=0, Rn=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rd_1_poweroftwo_f800_5e30f801() {
    // Encoding: 0x5E30F801
    // Test aarch64_vector_reduce_fp16_max_sisd field Rd = 1 (PowerOfTwo)
    // Fields: o1=0, sz=0, Rd=1, Rn=0
    let encoding: u32 = 0x5E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rd_30_poweroftwominusone_f800_5e30f81e() {
    // Encoding: 0x5E30F81E
    // Test aarch64_vector_reduce_fp16_max_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, sz=0, Rd=30, o1=0
    let encoding: u32 = 0x5E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_field_rd_31_max_f800_5e30f81f() {
    // Encoding: 0x5E30F81F
    // Test aarch64_vector_reduce_fp16_max_sisd field Rd = 31 (Max)
    // Fields: Rn=0, sz=0, Rd=31, o1=0
    let encoding: u32 = 0x5E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_0_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_1_f800_5eb0f800() {
    // Encoding: 0x5EB0F800
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=1, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, o1=1, Rn=0
    let encoding: u32 = 0x5EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_2_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_3_f800_5e70f800() {
    // Encoding: 0x5E70F800
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=1, Rn=0, Rd=0
    // Fields: Rd=0, o1=0, Rn=0, sz=1
    let encoding: u32 = 0x5E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_4_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, sz=0, Rn=0, o1=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_5_f800_5e30f820() {
    // Encoding: 0x5E30F820
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=1, Rd=0
    // Fields: o1=0, sz=0, Rn=1, Rd=0
    let encoding: u32 = 0x5E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_6_f800_5e30fbc0() {
    // Encoding: 0x5E30FBC0
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=30, Rd=0
    // Fields: Rn=30, sz=0, o1=0, Rd=0
    let encoding: u32 = 0x5E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_7_f800_5e30fbe0() {
    // Encoding: 0x5E30FBE0
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=31, Rd=0
    // Fields: sz=0, Rn=31, o1=0, Rd=0
    let encoding: u32 = 0x5E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_8_f800_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, Rd=0, o1=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_9_f800_5e30f801() {
    // Encoding: 0x5E30F801
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=1
    // Fields: o1=0, sz=0, Rd=1, Rn=0
    let encoding: u32 = 0x5E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_10_f800_5e30f81e() {
    // Encoding: 0x5E30F81E
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=30
    // Fields: Rn=0, o1=0, sz=0, Rd=30
    let encoding: u32 = 0x5E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_11_f800_5e30f81f() {
    // Encoding: 0x5E30F81F
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=31
    // Fields: sz=0, o1=0, Rd=31, Rn=0
    let encoding: u32 = 0x5E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_12_f800_5e30f821() {
    // Encoding: 0x5E30F821
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=1, Rd=1
    // Fields: sz=0, Rn=1, Rd=1, o1=0
    let encoding: u32 = 0x5E30F821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_combo_13_f800_5e30fbff() {
    // Encoding: 0x5E30FBFF
    // Test aarch64_vector_reduce_fp16_max_sisd field combination: o1=0, sz=0, Rn=31, Rd=31
    // Fields: sz=0, o1=0, Rd=31, Rn=31
    let encoding: u32 = 0x5E30FBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_special_sz_0_size_variant_0_63488_5e30f800() {
    // Encoding: 0x5E30F800
    // Test aarch64_vector_reduce_fp16_max_sisd special value sz = 0 (Size variant 0)
    // Fields: Rd=0, o1=0, Rn=0, sz=0
    let encoding: u32 = 0x5E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_special_sz_1_size_variant_1_63488_5e70f800() {
    // Encoding: 0x5E70F800
    // Test aarch64_vector_reduce_fp16_max_sisd special value sz = 1 (Size variant 1)
    // Fields: o1=0, Rd=0, sz=1, Rn=0
    let encoding: u32 = 0x5E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_63488_5e70fbe0()
 {
    // Encoding: 0x5E70FBE0
    // Test aarch64_vector_reduce_fp16_max_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, o1=0, Rd=0, Rn=31
    let encoding: u32 = 0x5E70FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_63488_5e70f81f()
 {
    // Encoding: 0x5E70F81F
    // Test aarch64_vector_reduce_fp16_max_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: o1=0, Rn=0, Rd=31, sz=1
    let encoding: u32 = 0x5E70F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_o1_0_min_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field o1 = 0 (Min)
    // Fields: o1=0, Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_o1_1_max_f800_7eb0f800() {
    // Encoding: 0x7EB0F800
    // Test aarch64_vector_reduce_fp_max_sisd field o1 = 1 (Max)
    // Fields: o1=1, Rn=0, sz=0, Rd=0
    let encoding: u32 = 0x7EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_sz_0_min_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field sz = 0 (Min)
    // Fields: o1=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_sz_1_max_f800_7e70f800() {
    // Encoding: 0x7E70F800
    // Test aarch64_vector_reduce_fp_max_sisd field sz = 1 (Max)
    // Fields: o1=0, sz=1, Rd=0, Rn=0
    let encoding: u32 = 0x7E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rn_0_min_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0, sz=0, o1=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rn_1_poweroftwo_f800_7e30f820() {
    // Encoding: 0x7E30F820
    // Test aarch64_vector_reduce_fp_max_sisd field Rn = 1 (PowerOfTwo)
    // Fields: sz=0, Rd=0, o1=0, Rn=1
    let encoding: u32 = 0x7E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rn_30_poweroftwominusone_f800_7e30fbc0() {
    // Encoding: 0x7E30FBC0
    // Test aarch64_vector_reduce_fp_max_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, o1=0, Rn=30, sz=0
    let encoding: u32 = 0x7E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rn_31_max_f800_7e30fbe0() {
    // Encoding: 0x7E30FBE0
    // Test aarch64_vector_reduce_fp_max_sisd field Rn = 31 (Max)
    // Fields: o1=0, Rd=0, Rn=31, sz=0
    let encoding: u32 = 0x7E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rd_0_min_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field Rd = 0 (Min)
    // Fields: o1=0, Rd=0, sz=0, Rn=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rd_1_poweroftwo_f800_7e30f801() {
    // Encoding: 0x7E30F801
    // Test aarch64_vector_reduce_fp_max_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, o1=0, sz=0, Rn=0
    let encoding: u32 = 0x7E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rd_30_poweroftwominusone_f800_7e30f81e() {
    // Encoding: 0x7E30F81E
    // Test aarch64_vector_reduce_fp_max_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, o1=0, sz=0, Rn=0
    let encoding: u32 = 0x7E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_field_rd_31_max_f800_7e30f81f() {
    // Encoding: 0x7E30F81F
    // Test aarch64_vector_reduce_fp_max_sisd field Rd = 31 (Max)
    // Fields: sz=0, o1=0, Rn=0, Rd=31
    let encoding: u32 = 0x7E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_0_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_1_f800_7eb0f800() {
    // Encoding: 0x7EB0F800
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=1, sz=0, Rn=0, Rd=0
    // Fields: o1=1, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x7EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_2_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, o1=0, Rd=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_3_f800_7e70f800() {
    // Encoding: 0x7E70F800
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=1, Rn=0, Rd=0
    // Fields: sz=1, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x7E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_4_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, o1=0, sz=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_5_f800_7e30f820() {
    // Encoding: 0x7E30F820
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=1, Rd=0
    // Fields: Rn=1, sz=0, Rd=0, o1=0
    let encoding: u32 = 0x7E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_6_f800_7e30fbc0() {
    // Encoding: 0x7E30FBC0
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=30, Rd=0
    // Fields: Rd=0, Rn=30, o1=0, sz=0
    let encoding: u32 = 0x7E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_7_f800_7e30fbe0() {
    // Encoding: 0x7E30FBE0
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=31, Rd=0
    // Fields: o1=0, Rn=31, sz=0, Rd=0
    let encoding: u32 = 0x7E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_8_f800_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=0
    // Fields: sz=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_9_f800_7e30f801() {
    // Encoding: 0x7E30F801
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=1
    // Fields: Rn=0, o1=0, Rd=1, sz=0
    let encoding: u32 = 0x7E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_10_f800_7e30f81e() {
    // Encoding: 0x7E30F81E
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=30
    // Fields: Rn=0, sz=0, o1=0, Rd=30
    let encoding: u32 = 0x7E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_11_f800_7e30f81f() {
    // Encoding: 0x7E30F81F
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=0, Rd=31
    // Fields: sz=0, Rn=0, Rd=31, o1=0
    let encoding: u32 = 0x7E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_12_f800_7e30f821() {
    // Encoding: 0x7E30F821
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=1, Rd=1
    // Fields: sz=0, o1=0, Rd=1, Rn=1
    let encoding: u32 = 0x7E30F821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_combo_13_f800_7e30fbff() {
    // Encoding: 0x7E30FBFF
    // Test aarch64_vector_reduce_fp_max_sisd field combination: o1=0, sz=0, Rn=31, Rd=31
    // Fields: o1=0, sz=0, Rd=31, Rn=31
    let encoding: u32 = 0x7E30FBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_special_sz_0_size_variant_0_63488_7e30f800() {
    // Encoding: 0x7E30F800
    // Test aarch64_vector_reduce_fp_max_sisd special value sz = 0 (Size variant 0)
    // Fields: o1=0, sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x7E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_special_sz_1_size_variant_1_63488_7e70f800() {
    // Encoding: 0x7E70F800
    // Test aarch64_vector_reduce_fp_max_sisd special value sz = 1 (Size variant 1)
    // Fields: o1=0, sz=1, Rd=0, Rn=0
    let encoding: u32 = 0x7E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_63488_7e70fbe0()
 {
    // Encoding: 0x7E70FBE0
    // Test aarch64_vector_reduce_fp_max_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: o1=0, Rd=0, sz=1, Rn=31
    let encoding: u32 = 0x7E70FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_63488_7e70f81f()
 {
    // Encoding: 0x7E70F81F
    // Test aarch64_vector_reduce_fp_max_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sz=1, Rn=0, o1=0, Rd=31
    let encoding: u32 = 0x7E70F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_reg_write_0_5e30f800() {
    // Test aarch64_vector_reduce_fp16_max_sisd register write: SimdFromField("d")
    // Encoding: 0x5E30F800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30F800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_sp_rn_5e30fbe0() {
    // Test aarch64_vector_reduce_fp16_max_sisd with Rn = SP (31)
    // Encoding: 0x5E30FBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30FBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_max_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_max_sisd_zr_rd_5e30f81f() {
    // Test aarch64_vector_reduce_fp16_max_sisd with Rd = ZR (31)
    // Encoding: 0x5E30F81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E30F81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_reg_write_0_7e30f800() {
    // Test aarch64_vector_reduce_fp_max_sisd register write: SimdFromField("d")
    // Encoding: 0x7E30F800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30F800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_sp_rn_7e30fbe0() {
    // Test aarch64_vector_reduce_fp_max_sisd with Rn = SP (31)
    // Encoding: 0x7E30FBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30FBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp_max_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp_max_sisd_zr_rd_7e30f81f() {
    // Test aarch64_vector_reduce_fp_max_sisd with Rd = ZR (31)
    // Encoding: 0x7E30F81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7E30F81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_reduce_fp16_maxnm_simd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_q_0_min_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Q = 0 (Min)
    // Fields: o1=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_q_1_max_c800_4e30c800() {
    // Encoding: 0x4E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Q = 1 (Max)
    // Fields: Rn=0, Q=1, o1=0, Rd=0
    let encoding: u32 = 0x4E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_o1_0_min_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field o1 = 0 (Min)
    // Fields: o1=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_o1_1_max_c800_0eb0c800() {
    // Encoding: 0x0EB0C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field o1 = 1 (Max)
    // Fields: Q=0, Rd=0, o1=1, Rn=0
    let encoding: u32 = 0x0EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rn_0_min_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rn = 0 (Min)
    // Fields: o1=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rn_1_poweroftwo_c800_0e30c820() {
    // Encoding: 0x0E30C820
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, o1=0, Rd=0, Rn=1
    let encoding: u32 = 0x0E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rn_30_poweroftwominusone_c800_0e30cbc0() {
    // Encoding: 0x0E30CBC0
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rd=0, Rn=30, o1=0
    let encoding: u32 = 0x0E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rn_31_max_c800_0e30cbe0() {
    // Encoding: 0x0E30CBE0
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rn = 31 (Max)
    // Fields: Q=0, o1=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rd_0_min_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rd = 0 (Min)
    // Fields: Q=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rd_1_poweroftwo_c800_0e30c801() {
    // Encoding: 0x0E30C801
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, o1=0, Q=0, Rn=0
    let encoding: u32 = 0x0E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rd_30_poweroftwominusone_c800_0e30c81e() {
    // Encoding: 0x0E30C81E
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, o1=0, Q=0
    let encoding: u32 = 0x0E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_field_rd_31_max_c800_0e30c81f() {
    // Encoding: 0x0E30C81F
    // Test aarch64_vector_reduce_fp16_maxnm_simd field Rd = 31 (Max)
    // Fields: Rn=0, Q=0, o1=0, Rd=31
    let encoding: u32 = 0x0E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_0_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_1_c800_4e30c800() {
    // Encoding: 0x4E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=1, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=1, o1=0, Rd=0
    let encoding: u32 = 0x4E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_2_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, o1=0, Rd=0, Q=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_3_c800_0eb0c800() {
    // Encoding: 0x0EB0C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=1, Rn=0, Rd=0
    // Fields: o1=1, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_4_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, Rn=0, o1=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_5_c800_0e30c820() {
    // Encoding: 0x0E30C820
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=1, Rd=0
    // Fields: o1=0, Rd=0, Rn=1, Q=0
    let encoding: u32 = 0x0E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_6_c800_0e30cbc0() {
    // Encoding: 0x0E30CBC0
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=30, Rd=0
    // Fields: Q=0, o1=0, Rd=0, Rn=30
    let encoding: u32 = 0x0E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_7_c800_0e30cbe0() {
    // Encoding: 0x0E30CBE0
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=31, Rd=0
    // Fields: o1=0, Rn=31, Rd=0, Q=0
    let encoding: u32 = 0x0E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_8_c800_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: o1=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_9_c800_0e30c801() {
    // Encoding: 0x0E30C801
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=1
    // Fields: o1=0, Q=0, Rn=0, Rd=1
    let encoding: u32 = 0x0E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_10_c800_0e30c81e() {
    // Encoding: 0x0E30C81E
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=30
    // Fields: o1=0, Rd=30, Rn=0, Q=0
    let encoding: u32 = 0x0E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_11_c800_0e30c81f() {
    // Encoding: 0x0E30C81F
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=0, Rd=31
    // Fields: o1=0, Rd=31, Q=0, Rn=0
    let encoding: u32 = 0x0E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_12_c800_0e30c821() {
    // Encoding: 0x0E30C821
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=1, Rd=1
    // Fields: Rd=1, o1=0, Rn=1, Q=0
    let encoding: u32 = 0x0E30C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_combo_13_c800_0e30cbff() {
    // Encoding: 0x0E30CBFF
    // Test aarch64_vector_reduce_fp16_maxnm_simd field combination: Q=0, o1=0, Rn=31, Rd=31
    // Fields: Rn=31, Q=0, o1=0, Rd=31
    let encoding: u32 = 0x0E30CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_special_q_0_size_variant_0_51200_0e30c800() {
    // Encoding: 0x0E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd special value Q = 0 (Size variant 0)
    // Fields: o1=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_special_q_1_size_variant_1_51200_4e30c800() {
    // Encoding: 0x4E30C800
    // Test aarch64_vector_reduce_fp16_maxnm_simd special value Q = 1 (Size variant 1)
    // Fields: Q=1, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x4E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_0e30cbe0()
 {
    // Encoding: 0x0E30CBE0
    // Test aarch64_vector_reduce_fp16_maxnm_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, o1=0, Q=0, Rn=31
    let encoding: u32 = 0x0E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_0e30c81f()
 {
    // Encoding: 0x0E30C81F
    // Test aarch64_vector_reduce_fp16_maxnm_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Q=0, o1=0, Rd=31
    let encoding: u32 = 0x0E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_q_0_min_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field Q = 0 (Min)
    // Fields: o1=0, Rd=0, Rn=0, Q=0, sz=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_q_1_max_c800_6e30c800() {
    // Encoding: 0x6E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field Q = 1 (Max)
    // Fields: Rd=0, Rn=0, o1=0, sz=0, Q=1
    let encoding: u32 = 0x6E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_o1_0_min_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field o1 = 0 (Min)
    // Fields: sz=0, Rn=0, Rd=0, Q=0, o1=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_o1_1_max_c800_2eb0c800() {
    // Encoding: 0x2EB0C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field o1 = 1 (Max)
    // Fields: sz=0, Rn=0, Q=0, Rd=0, o1=1
    let encoding: u32 = 0x2EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_sz_0_min_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field sz = 0 (Min)
    // Fields: Rn=0, sz=0, Rd=0, o1=0, Q=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_sz_1_max_c800_2e70c800() {
    // Encoding: 0x2E70C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field sz = 1 (Max)
    // Fields: Rd=0, Q=0, Rn=0, sz=1, o1=0
    let encoding: u32 = 0x2E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rn_0_min_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rn = 0 (Min)
    // Fields: Rn=0, Q=0, Rd=0, o1=0, sz=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rn_1_poweroftwo_c800_2e30c820() {
    // Encoding: 0x2E30C820
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, o1=0, Q=0, sz=0, Rd=0
    let encoding: u32 = 0x2E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rn_30_poweroftwominusone_c800_2e30cbc0() {
    // Encoding: 0x2E30CBC0
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=30, Rd=0, sz=0, o1=0
    let encoding: u32 = 0x2E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rn_31_max_c800_2e30cbe0() {
    // Encoding: 0x2E30CBE0
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rn = 31 (Max)
    // Fields: o1=0, sz=0, Rn=31, Q=0, Rd=0
    let encoding: u32 = 0x2E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rd_0_min_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rd = 0 (Min)
    // Fields: sz=0, o1=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rd_1_poweroftwo_c800_2e30c801() {
    // Encoding: 0x2E30C801
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, sz=0, o1=0, Q=0, Rd=1
    let encoding: u32 = 0x2E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rd_30_poweroftwominusone_c800_2e30c81e() {
    // Encoding: 0x2E30C81E
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Rn=0, Rd=30, Q=0, sz=0
    let encoding: u32 = 0x2E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_field_rd_31_max_c800_2e30c81f() {
    // Encoding: 0x2E30C81F
    // Test aarch64_vector_reduce_fp_maxnm_simd field Rd = 31 (Max)
    // Fields: Rd=31, o1=0, Rn=0, sz=0, Q=0
    let encoding: u32 = 0x2E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_0_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, sz=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_1_c800_6e30c800() {
    // Encoding: 0x6E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=1, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Q=1, o1=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x6E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_2_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, o1=0, sz=0, Rn=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_3_c800_2eb0c800() {
    // Encoding: 0x2EB0C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=1, sz=0, Rn=0, Rd=0
    // Fields: sz=0, o1=1, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x2EB0C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_4_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: o1=0, sz=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_5_c800_2e70c800() {
    // Encoding: 0x2E70C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=1, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0, sz=1, o1=0
    let encoding: u32 = 0x2E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_6_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Q=0, sz=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_7_c800_2e30c820() {
    // Encoding: 0x2E30C820
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=1, Rd=0
    // Fields: o1=0, sz=0, Rn=1, Rd=0, Q=0
    let encoding: u32 = 0x2E30C820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_8_c800_2e30cbc0() {
    // Encoding: 0x2E30CBC0
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=30, Rd=0
    // Fields: o1=0, Rd=0, Rn=30, Q=0, sz=0
    let encoding: u32 = 0x2E30CBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_9_c800_2e30cbe0() {
    // Encoding: 0x2E30CBE0
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=31, Rd=0
    // Fields: Rd=0, Q=0, o1=0, Rn=31, sz=0
    let encoding: u32 = 0x2E30CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_10_c800_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Q=0, o1=0, sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_11_c800_2e30c801() {
    // Encoding: 0x2E30C801
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=1
    // Fields: Q=0, Rn=0, Rd=1, sz=0, o1=0
    let encoding: u32 = 0x2E30C801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_12_c800_2e30c81e() {
    // Encoding: 0x2E30C81E
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=30
    // Fields: Rn=0, Q=0, o1=0, sz=0, Rd=30
    let encoding: u32 = 0x2E30C81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_13_c800_2e30c81f() {
    // Encoding: 0x2E30C81F
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=31
    // Fields: o1=0, Rn=0, Rd=31, Q=0, sz=0
    let encoding: u32 = 0x2E30C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_14_c800_2e30c821() {
    // Encoding: 0x2E30C821
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=1, Rd=1
    // Fields: o1=0, Q=0, Rn=1, Rd=1, sz=0
    let encoding: u32 = 0x2E30C821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_combo_15_c800_2e30cbff() {
    // Encoding: 0x2E30CBFF
    // Test aarch64_vector_reduce_fp_maxnm_simd field combination: Q=0, o1=0, sz=0, Rn=31, Rd=31
    // Fields: Rd=31, o1=0, sz=0, Q=0, Rn=31
    let encoding: u32 = 0x2E30CBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_special_q_0_size_variant_0_51200_2e70c800() {
    // Encoding: 0x2E70C800
    // Test aarch64_vector_reduce_fp_maxnm_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, Rd=0, o1=0, Rn=0, sz=1
    let encoding: u32 = 0x2E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_special_q_1_size_variant_1_51200_6e70c800() {
    // Encoding: 0x6E70C800
    // Test aarch64_vector_reduce_fp_maxnm_simd special value Q = 1 (Size variant 1)
    // Fields: o1=0, Rd=0, Q=1, sz=1, Rn=0
    let encoding: u32 = 0x6E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_special_sz_0_size_variant_0_51200_2e30c800() {
    // Encoding: 0x2E30C800
    // Test aarch64_vector_reduce_fp_maxnm_simd special value sz = 0 (Size variant 0)
    // Fields: Rd=0, o1=0, sz=0, Rn=0, Q=0
    let encoding: u32 = 0x2E30C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_special_sz_1_size_variant_1_51200_2e70c800() {
    // Encoding: 0x2E70C800
    // Test aarch64_vector_reduce_fp_maxnm_simd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, Rd=0, Q=0, o1=0, sz=1
    let encoding: u32 = 0x2E70C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_special_rn_31_stack_pointer_sp_may_require_alignment_51200_2e70cbe0()
 {
    // Encoding: 0x2E70CBE0
    // Test aarch64_vector_reduce_fp_maxnm_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, Rn=31, Q=0, o1=0, Rd=0
    let encoding: u32 = 0x2E70CBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_51200_2e70c81f()
 {
    // Encoding: 0x2E70C81F
    // Test aarch64_vector_reduce_fp_maxnm_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Q=0, o1=0, sz=1, Rd=31
    let encoding: u32 = 0x2E70C81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_reg_write_0_0e30c800() {
    // Test aarch64_vector_reduce_fp16_maxnm_simd register write: SimdFromField("d")
    // Encoding: 0x0E30C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_sp_rn_0e30cbe0() {
    // Test aarch64_vector_reduce_fp16_maxnm_simd with Rn = SP (31)
    // Encoding: 0x0E30CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30CBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_maxnm_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_maxnm_simd_zr_rd_0e30c81f() {
    // Test aarch64_vector_reduce_fp16_maxnm_simd with Rd = ZR (31)
    // Encoding: 0x0E30C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30C81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_reg_write_0_2e30c800() {
    // Test aarch64_vector_reduce_fp_maxnm_simd register write: SimdFromField("d")
    // Encoding: 0x2E30C800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E30C800;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_sp_rn_2e30cbe0() {
    // Test aarch64_vector_reduce_fp_maxnm_simd with Rn = SP (31)
    // Encoding: 0x2E30CBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E30CBE0;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_maxnm_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp_maxnm_simd_zr_rd_2e30c81f() {
    // Test aarch64_vector_reduce_fp_maxnm_simd with Rd = ZR (31)
    // Encoding: 0x2E30C81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E30C81F;
    write_insn(&mut cpu, 0, encoding);
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
// aarch64_vector_reduce_int_max Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_int_max_field_q_0_min_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field Q = 0 (Min)
    // Fields: Rn=0, size=0, Q=0, U=0, Rd=0, op=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_int_max_field_q_1_max_a800_4e30a800() {
    // Encoding: 0x4E30A800
    // Test aarch64_vector_reduce_int_max field Q = 1 (Max)
    // Fields: Rn=0, size=0, op=0, U=0, Rd=0, Q=1
    let encoding: u32 = 0x4E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_int_max_field_u_0_min_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field U = 0 (Min)
    // Fields: Rd=0, size=0, Rn=0, op=0, Q=0, U=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_int_max_field_u_1_max_a800_2e30a800() {
    // Encoding: 0x2E30A800
    // Test aarch64_vector_reduce_int_max field U = 1 (Max)
    // Fields: Q=0, U=1, size=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_int_max_field_size_0_min_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field size = 0 (Min)
    // Fields: U=0, Q=0, op=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_int_max_field_size_1_poweroftwo_a800_0e70a800() {
    // Encoding: 0x0E70A800
    // Test aarch64_vector_reduce_int_max field size = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, U=0, Q=0, size=1, op=0
    let encoding: u32 = 0x0E70A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_reduce_int_max_field_size_2_poweroftwo_a800_0eb0a800() {
    // Encoding: 0x0EB0A800
    // Test aarch64_vector_reduce_int_max field size = 2 (PowerOfTwo)
    // Fields: size=2, Rn=0, Q=0, op=0, Rd=0, U=0
    let encoding: u32 = 0x0EB0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_reduce_int_max_field_size_3_max_a800_0ef0a800() {
    // Encoding: 0x0EF0A800
    // Test aarch64_vector_reduce_int_max field size = 3 (Max)
    // Fields: Rd=0, op=0, Rn=0, Q=0, U=0, size=3
    let encoding: u32 = 0x0EF0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field op 16 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_int_max_field_op_0_min_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field op = 0 (Min)
    // Fields: Q=0, U=0, op=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field op 16 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_int_max_field_op_1_max_a800_0e31a800() {
    // Encoding: 0x0E31A800
    // Test aarch64_vector_reduce_int_max field op = 1 (Max)
    // Fields: Rd=0, U=0, Rn=0, op=1, size=0, Q=0
    let encoding: u32 = 0x0E31A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rn_0_min_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field Rn = 0 (Min)
    // Fields: U=0, op=0, Q=0, Rn=0, Rd=0, size=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rn_1_poweroftwo_a800_0e30a820() {
    // Encoding: 0x0E30A820
    // Test aarch64_vector_reduce_int_max field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rd=0, Rn=1, Q=0, U=0, op=0
    let encoding: u32 = 0x0E30A820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rn_30_poweroftwominusone_a800_0e30abc0() {
    // Encoding: 0x0E30ABC0
    // Test aarch64_vector_reduce_int_max field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, size=0, Rd=0, Q=0, U=0, Rn=30
    let encoding: u32 = 0x0E30ABC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rn_31_max_a800_0e30abe0() {
    // Encoding: 0x0E30ABE0
    // Test aarch64_vector_reduce_int_max field Rn = 31 (Max)
    // Fields: Q=0, Rn=31, Rd=0, size=0, op=0, U=0
    let encoding: u32 = 0x0E30ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rd_0_min_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field Rd = 0 (Min)
    // Fields: Rd=0, Rn=0, size=0, Q=0, op=0, U=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rd_1_poweroftwo_a800_0e30a801() {
    // Encoding: 0x0E30A801
    // Test aarch64_vector_reduce_int_max field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, size=0, Rn=0, U=0, op=0, Rd=1
    let encoding: u32 = 0x0E30A801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rd_30_poweroftwominusone_a800_0e30a81e() {
    // Encoding: 0x0E30A81E
    // Test aarch64_vector_reduce_int_max field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, size=0, Rn=0, Rd=30, Q=0, op=0
    let encoding: u32 = 0x0E30A81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_int_max_field_rd_31_max_a800_0e30a81f() {
    // Encoding: 0x0E30A81F
    // Test aarch64_vector_reduce_int_max field Rd = 31 (Max)
    // Fields: Q=0, size=0, U=0, op=0, Rn=0, Rd=31
    let encoding: u32 = 0x0E30A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_0_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, size=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_1_a800_4e30a800() {
    // Encoding: 0x4E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=1, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, op=0, Rn=0, size=0, Q=1
    let encoding: u32 = 0x4E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_2_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Rn=0, Q=0, op=0, size=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_3_a800_2e30a800() {
    // Encoding: 0x2E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=1, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, op=0, U=1, size=0, Rn=0, Rd=0
    let encoding: u32 = 0x2E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_4_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Q=0, U=0, size=0, op=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_5_a800_0e70a800() {
    // Encoding: 0x0E70A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=1, op=0, Rn=0, Rd=0
    // Fields: U=0, size=1, Q=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x0E70A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_6_a800_0eb0a800() {
    // Encoding: 0x0EB0A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=2, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, Q=0, U=0, Rn=0, size=2
    let encoding: u32 = 0x0EB0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_7_a800_0ef0a800() {
    // Encoding: 0x0EF0A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=3, op=0, Rn=0, Rd=0
    // Fields: Rd=0, size=3, Rn=0, Q=0, U=0, op=0
    let encoding: u32 = 0x0EF0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_int_max_combo_8_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_9_a800_0e31a800() {
    // Encoding: 0x0E31A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=1, Rn=0, Rd=0
    // Fields: Q=0, size=0, op=1, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E31A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_10_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: size=0, Q=0, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_11_a800_0e30a820() {
    // Encoding: 0x0E30A820
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=1, Rd=0
    // Fields: Q=0, U=0, Rn=1, op=0, Rd=0, size=0
    let encoding: u32 = 0x0E30A820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_12_a800_0e30abc0() {
    // Encoding: 0x0E30ABC0
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, U=0, size=0, Q=0, op=0
    let encoding: u32 = 0x0E30ABC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_13_a800_0e30abe0() {
    // Encoding: 0x0E30ABE0
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=31, Rd=0
    // Fields: Q=0, U=0, op=0, Rn=31, Rd=0, size=0
    let encoding: u32 = 0x0E30ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_14_a800_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, size=0, U=0, Rd=0, Q=0, op=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_15_a800_0e30a801() {
    // Encoding: 0x0E30A801
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=1
    // Fields: Q=0, size=0, Rn=0, U=0, op=0, Rd=1
    let encoding: u32 = 0x0E30A801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_16_a800_0e30a81e() {
    // Encoding: 0x0E30A81E
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=30
    // Fields: Q=0, Rd=30, U=0, size=0, Rn=0, op=0
    let encoding: u32 = 0x0E30A81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_17_a800_0e30a81f() {
    // Encoding: 0x0E30A81F
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=0, Rd=31
    // Fields: op=0, U=0, Q=0, Rd=31, size=0, Rn=0
    let encoding: u32 = 0x0E30A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_18_a800_0e30a821() {
    // Encoding: 0x0E30A821
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=1, Rd=1
    // Fields: Rd=1, size=0, U=0, Q=0, Rn=1, op=0
    let encoding: u32 = 0x0E30A821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_int_max_combo_19_a800_0e30abff() {
    // Encoding: 0x0E30ABFF
    // Test aarch64_vector_reduce_int_max field combination: Q=0, U=0, size=0, op=0, Rn=31, Rd=31
    // Fields: Rn=31, Rd=31, size=0, U=0, Q=0, op=0
    let encoding: u32 = 0x0E30ABFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_int_max_special_q_0_size_variant_0_43008_0e70a800() {
    // Encoding: 0x0E70A800
    // Test aarch64_vector_reduce_int_max special value Q = 0 (Size variant 0)
    // Fields: U=0, size=1, Q=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E70A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_int_max_special_q_1_size_variant_1_43008_4e70a800() {
    // Encoding: 0x4E70A800
    // Test aarch64_vector_reduce_int_max special value Q = 1 (Size variant 1)
    // Fields: op=0, Rd=0, size=1, Q=1, Rn=0, U=0
    let encoding: u32 = 0x4E70A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_int_max_special_size_0_size_variant_0_43008_0e30a800() {
    // Encoding: 0x0E30A800
    // Test aarch64_vector_reduce_int_max special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, Q=0, size=0, op=0, U=0
    let encoding: u32 = 0x0E30A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_int_max_special_size_1_size_variant_1_43008_0e70a800() {
    // Encoding: 0x0E70A800
    // Test aarch64_vector_reduce_int_max special value size = 1 (Size variant 1)
    // Fields: Rn=0, Q=0, Rd=0, size=1, op=0, U=0
    let encoding: u32 = 0x0E70A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_reduce_int_max_special_size_2_size_variant_2_43008_0eb0a800() {
    // Encoding: 0x0EB0A800
    // Test aarch64_vector_reduce_int_max special value size = 2 (Size variant 2)
    // Fields: Rn=0, Q=0, U=0, Rd=0, size=2, op=0
    let encoding: u32 = 0x0EB0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_reduce_int_max_special_size_3_size_variant_3_43008_0ef0a800() {
    // Encoding: 0x0EF0A800
    // Test aarch64_vector_reduce_int_max special value size = 3 (Size variant 3)
    // Fields: Q=0, op=0, U=0, Rn=0, size=3, Rd=0
    let encoding: u32 = 0x0EF0A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_int_max_special_rn_31_stack_pointer_sp_may_require_alignment_43008_0e70abe0()
 {
    // Encoding: 0x0E70ABE0
    // Test aarch64_vector_reduce_int_max special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, Rn=31, Q=0, U=0, op=0, size=1
    let encoding: u32 = 0x0E70ABE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_int_max_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_43008_0e70a81f()
 {
    // Encoding: 0x0E70A81F
    // Test aarch64_vector_reduce_int_max special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, op=0, U=0, size=1, Rn=0, Q=0
    let encoding: u32 = 0x0E70A81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_int_max_reg_write_0_0e30a800() {
    // Test aarch64_vector_reduce_int_max register write: SimdFromField("d")
    // Encoding: 0x0E30A800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30A800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_int_max_sp_rn_0e30abe0() {
    // Test aarch64_vector_reduce_int_max with Rn = SP (31)
    // Encoding: 0x0E30ABE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30ABE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_int_max
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_int_max_zr_rd_0e30a81f() {
    // Test aarch64_vector_reduce_int_max with Rd = ZR (31)
    // Encoding: 0x0E30A81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30A81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_reduce_add_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_size_0_min_b800_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd field size = 0 (Min)
    // Fields: Rd=0, Rn=0, size=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_size_1_poweroftwo_b800_5e71b800() {
    // Encoding: 0x5E71B800
    // Test aarch64_vector_reduce_add_sisd field size = 1 (PowerOfTwo)
    // Fields: size=1, Rn=0, Rd=0
    let encoding: u32 = 0x5E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_size_2_poweroftwo_b800_5eb1b800() {
    // Encoding: 0x5EB1B800
    // Test aarch64_vector_reduce_add_sisd field size = 2 (PowerOfTwo)
    // Fields: size=2, Rd=0, Rn=0
    let encoding: u32 = 0x5EB1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_size_3_max_b800_5ef1b800() {
    // Encoding: 0x5EF1B800
    // Test aarch64_vector_reduce_add_sisd field size = 3 (Max)
    // Fields: size=3, Rn=0, Rd=0
    let encoding: u32 = 0x5EF1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rn_0_min_b800_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd field Rn = 0 (Min)
    // Fields: size=0, Rn=0, Rd=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rn_1_poweroftwo_b800_5e31b820() {
    // Encoding: 0x5E31B820
    // Test aarch64_vector_reduce_add_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0, size=0
    let encoding: u32 = 0x5E31B820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rn_30_poweroftwominusone_b800_5e31bbc0() {
    // Encoding: 0x5E31BBC0
    // Test aarch64_vector_reduce_add_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=30, Rd=0
    let encoding: u32 = 0x5E31BBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rn_31_max_b800_5e31bbe0() {
    // Encoding: 0x5E31BBE0
    // Test aarch64_vector_reduce_add_sisd field Rn = 31 (Max)
    // Fields: Rn=31, size=0, Rd=0
    let encoding: u32 = 0x5E31BBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rd_0_min_b800_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd field Rd = 0 (Min)
    // Fields: Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rd_1_poweroftwo_b800_5e31b801() {
    // Encoding: 0x5E31B801
    // Test aarch64_vector_reduce_add_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, size=0, Rn=0
    let encoding: u32 = 0x5E31B801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rd_30_poweroftwominusone_b800_5e31b81e() {
    // Encoding: 0x5E31B81E
    // Test aarch64_vector_reduce_add_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, size=0, Rn=0
    let encoding: u32 = 0x5E31B81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_add_sisd_field_rd_31_max_b800_5e31b81f() {
    // Encoding: 0x5E31B81F
    // Test aarch64_vector_reduce_add_sisd field Rd = 31 (Max)
    // Fields: Rn=0, size=0, Rd=31
    let encoding: u32 = 0x5E31B81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_0_b800_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_1_b800_5e71b800() {
    // Encoding: 0x5E71B800
    // Test aarch64_vector_reduce_add_sisd field combination: size=1, Rn=0, Rd=0
    // Fields: Rd=0, size=1, Rn=0
    let encoding: u32 = 0x5E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_2_b800_5eb1b800() {
    // Encoding: 0x5EB1B800
    // Test aarch64_vector_reduce_add_sisd field combination: size=2, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=2
    let encoding: u32 = 0x5EB1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_3_b800_5ef1b800() {
    // Encoding: 0x5EF1B800
    // Test aarch64_vector_reduce_add_sisd field combination: size=3, Rn=0, Rd=0
    // Fields: size=3, Rn=0, Rd=0
    let encoding: u32 = 0x5EF1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_4_b800_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_5_b800_5e31b820() {
    // Encoding: 0x5E31B820
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=1, Rd=0
    // Fields: Rd=0, size=0, Rn=1
    let encoding: u32 = 0x5E31B820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_6_b800_5e31bbc0() {
    // Encoding: 0x5E31BBC0
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=30, Rd=0
    // Fields: size=0, Rd=0, Rn=30
    let encoding: u32 = 0x5E31BBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_7_b800_5e31bbe0() {
    // Encoding: 0x5E31BBE0
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=31, Rd=0
    // Fields: Rn=31, size=0, Rd=0
    let encoding: u32 = 0x5E31BBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_8_b800_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=0, Rd=0
    // Fields: Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_9_b800_5e31b801() {
    // Encoding: 0x5E31B801
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=0, Rd=1
    // Fields: Rn=0, size=0, Rd=1
    let encoding: u32 = 0x5E31B801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_10_b800_5e31b81e() {
    // Encoding: 0x5E31B81E
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=0, Rd=30
    // Fields: Rd=30, size=0, Rn=0
    let encoding: u32 = 0x5E31B81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_11_b800_5e31b81f() {
    // Encoding: 0x5E31B81F
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=0, Rd=31
    // Fields: Rd=31, size=0, Rn=0
    let encoding: u32 = 0x5E31B81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_12_b800_5e31b821() {
    // Encoding: 0x5E31B821
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, size=0
    let encoding: u32 = 0x5E31B821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_add_sisd_combo_13_b800_5e31bbff() {
    // Encoding: 0x5E31BBFF
    // Test aarch64_vector_reduce_add_sisd field combination: size=0, Rn=31, Rd=31
    // Fields: size=0, Rd=31, Rn=31
    let encoding: u32 = 0x5E31BBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_add_sisd_special_size_0_size_variant_0_47104_5e31b800() {
    // Encoding: 0x5E31B800
    // Test aarch64_vector_reduce_add_sisd special value size = 0 (Size variant 0)
    // Fields: Rd=0, size=0, Rn=0
    let encoding: u32 = 0x5E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_add_sisd_special_size_1_size_variant_1_47104_5e71b800() {
    // Encoding: 0x5E71B800
    // Test aarch64_vector_reduce_add_sisd special value size = 1 (Size variant 1)
    // Fields: Rd=0, size=1, Rn=0
    let encoding: u32 = 0x5E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_reduce_add_sisd_special_size_2_size_variant_2_47104_5eb1b800() {
    // Encoding: 0x5EB1B800
    // Test aarch64_vector_reduce_add_sisd special value size = 2 (Size variant 2)
    // Fields: Rn=0, Rd=0, size=2
    let encoding: u32 = 0x5EB1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_reduce_add_sisd_special_size_3_size_variant_3_47104_5ef1b800() {
    // Encoding: 0x5EF1B800
    // Test aarch64_vector_reduce_add_sisd special value size = 3 (Size variant 3)
    // Fields: size=3, Rd=0, Rn=0
    let encoding: u32 = 0x5EF1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_add_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_47104_5e71bbe0()
 {
    // Encoding: 0x5E71BBE0
    // Test aarch64_vector_reduce_add_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rd=0, Rn=31
    let encoding: u32 = 0x5E71BBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_add_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_47104_5e71b81f()
 {
    // Encoding: 0x5E71B81F
    // Test aarch64_vector_reduce_add_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, size=1, Rd=31
    let encoding: u32 = 0x5E71B81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_add_sisd_reg_write_0_5e31b800() {
    // Test aarch64_vector_reduce_add_sisd register write: SimdFromField("d")
    // Encoding: 0x5E31B800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E31B800;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_add_sisd_sp_rn_5e31bbe0() {
    // Test aarch64_vector_reduce_add_sisd with Rn = SP (31)
    // Encoding: 0x5E31BBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E31BBE0;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_add_sisd_zr_rd_5e31b81f() {
    // Test aarch64_vector_reduce_add_sisd with Rd = ZR (31)
    // Encoding: 0x5E31B81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5E31B81F;
    write_insn(&mut cpu, 0, encoding);
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
// aarch64_vector_reduce_fp16_max_simd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_q_0_min_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field Q = 0 (Min)
    // Fields: Q=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_q_1_max_f800_4e30f800() {
    // Encoding: 0x4E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field Q = 1 (Max)
    // Fields: Q=1, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x4E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_o1_0_min_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field o1 = 0 (Min)
    // Fields: Q=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_o1_1_max_f800_0eb0f800() {
    // Encoding: 0x0EB0F800
    // Test aarch64_vector_reduce_fp16_max_simd field o1 = 1 (Max)
    // Fields: o1=1, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rn_0_min_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0, o1=0, Q=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rn_1_poweroftwo_f800_0e30f820() {
    // Encoding: 0x0E30F820
    // Test aarch64_vector_reduce_fp16_max_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Q=0, o1=0, Rn=1
    let encoding: u32 = 0x0E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rn_30_poweroftwominusone_f800_0e30fbc0() {
    // Encoding: 0x0E30FBC0
    // Test aarch64_vector_reduce_fp16_max_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=30, Rd=0, o1=0
    let encoding: u32 = 0x0E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rn_31_max_f800_0e30fbe0() {
    // Encoding: 0x0E30FBE0
    // Test aarch64_vector_reduce_fp16_max_simd field Rn = 31 (Max)
    // Fields: o1=0, Rn=31, Rd=0, Q=0
    let encoding: u32 = 0x0E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rd_0_min_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field Rd = 0 (Min)
    // Fields: Q=0, Rn=0, Rd=0, o1=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rd_1_poweroftwo_f800_0e30f801() {
    // Encoding: 0x0E30F801
    // Test aarch64_vector_reduce_fp16_max_simd field Rd = 1 (PowerOfTwo)
    // Fields: o1=0, Q=0, Rd=1, Rn=0
    let encoding: u32 = 0x0E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rd_30_poweroftwominusone_f800_0e30f81e() {
    // Encoding: 0x0E30F81E
    // Test aarch64_vector_reduce_fp16_max_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, Q=0, Rn=0, Rd=30
    let encoding: u32 = 0x0E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_field_rd_31_max_f800_0e30f81f() {
    // Encoding: 0x0E30F81F
    // Test aarch64_vector_reduce_fp16_max_simd field Rd = 31 (Max)
    // Fields: o1=0, Q=0, Rd=31, Rn=0
    let encoding: u32 = 0x0E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_0_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Q=0, o1=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_1_f800_4e30f800() {
    // Encoding: 0x4E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=1, o1=0, Rn=0, Rd=0
    // Fields: Q=1, Rn=0, o1=0, Rd=0
    let encoding: u32 = 0x4E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_2_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: Rn=0, o1=0, Q=0, Rd=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_3_f800_0eb0f800() {
    // Encoding: 0x0EB0F800
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=1, Rn=0, Rd=0
    // Fields: o1=1, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_4_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, o1=0, Rn=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_5_f800_0e30f820() {
    // Encoding: 0x0E30F820
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=1, Rd=0
    // Fields: Rd=0, o1=0, Rn=1, Q=0
    let encoding: u32 = 0x0E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_6_f800_0e30fbc0() {
    // Encoding: 0x0E30FBC0
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=30, Rd=0
    // Fields: o1=0, Rn=30, Rd=0, Q=0
    let encoding: u32 = 0x0E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_7_f800_0e30fbe0() {
    // Encoding: 0x0E30FBE0
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=31, Rd=0
    // Fields: o1=0, Q=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_8_f800_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=0
    // Fields: Q=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_9_f800_0e30f801() {
    // Encoding: 0x0E30F801
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=1
    // Fields: Rn=0, o1=0, Q=0, Rd=1
    let encoding: u32 = 0x0E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_10_f800_0e30f81e() {
    // Encoding: 0x0E30F81E
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=30
    // Fields: Rd=30, Q=0, o1=0, Rn=0
    let encoding: u32 = 0x0E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_11_f800_0e30f81f() {
    // Encoding: 0x0E30F81F
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=0, Rd=31
    // Fields: o1=0, Rd=31, Q=0, Rn=0
    let encoding: u32 = 0x0E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_12_f800_0e30f821() {
    // Encoding: 0x0E30F821
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=1, Rd=1
    // Fields: Rd=1, Q=0, o1=0, Rn=1
    let encoding: u32 = 0x0E30F821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_combo_13_f800_0e30fbff() {
    // Encoding: 0x0E30FBFF
    // Test aarch64_vector_reduce_fp16_max_simd field combination: Q=0, o1=0, Rn=31, Rd=31
    // Fields: Rd=31, o1=0, Q=0, Rn=31
    let encoding: u32 = 0x0E30FBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_special_q_0_size_variant_0_63488_0e30f800() {
    // Encoding: 0x0E30F800
    // Test aarch64_vector_reduce_fp16_max_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, o1=0, Q=0
    let encoding: u32 = 0x0E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_special_q_1_size_variant_1_63488_4e30f800() {
    // Encoding: 0x4E30F800
    // Test aarch64_vector_reduce_fp16_max_simd special value Q = 1 (Size variant 1)
    // Fields: Rd=0, o1=0, Rn=0, Q=1
    let encoding: u32 = 0x4E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_special_rn_31_stack_pointer_sp_may_require_alignment_63488_0e30fbe0()
 {
    // Encoding: 0x0E30FBE0
    // Test aarch64_vector_reduce_fp16_max_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, o1=0, Rn=31, Rd=0
    let encoding: u32 = 0x0E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_63488_0e30f81f()
 {
    // Encoding: 0x0E30F81F
    // Test aarch64_vector_reduce_fp16_max_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0, Q=0, o1=0
    let encoding: u32 = 0x0E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_q_0_min_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field Q = 0 (Min)
    // Fields: Q=0, sz=0, Rd=0, o1=0, Rn=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_q_1_max_f800_6e30f800() {
    // Encoding: 0x6E30F800
    // Test aarch64_vector_reduce_fp_max_simd field Q = 1 (Max)
    // Fields: Q=1, o1=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x6E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_o1_0_min_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field o1 = 0 (Min)
    // Fields: Q=0, Rd=0, Rn=0, o1=0, sz=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field o1 23 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_o1_1_max_f800_2eb0f800() {
    // Encoding: 0x2EB0F800
    // Test aarch64_vector_reduce_fp_max_simd field o1 = 1 (Max)
    // Fields: Q=0, o1=1, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x2EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_sz_0_min_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field sz = 0 (Min)
    // Fields: Rd=0, sz=0, o1=0, Q=0, Rn=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field sz 22 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_sz_1_max_f800_2e70f800() {
    // Encoding: 0x2E70F800
    // Test aarch64_vector_reduce_fp_max_simd field sz = 1 (Max)
    // Fields: Rd=0, sz=1, Rn=0, Q=0, o1=0
    let encoding: u32 = 0x2E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rn_0_min_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field Rn = 0 (Min)
    // Fields: Q=0, o1=0, sz=0, Rd=0, Rn=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rn_1_poweroftwo_f800_2e30f820() {
    // Encoding: 0x2E30F820
    // Test aarch64_vector_reduce_fp_max_simd field Rn = 1 (PowerOfTwo)
    // Fields: sz=0, o1=0, Rd=0, Q=0, Rn=1
    let encoding: u32 = 0x2E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rn_30_poweroftwominusone_f800_2e30fbc0() {
    // Encoding: 0x2E30FBC0
    // Test aarch64_vector_reduce_fp_max_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=30, Rd=0, o1=0, sz=0
    let encoding: u32 = 0x2E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rn_31_max_f800_2e30fbe0() {
    // Encoding: 0x2E30FBE0
    // Test aarch64_vector_reduce_fp_max_simd field Rn = 31 (Max)
    // Fields: o1=0, Q=0, Rn=31, Rd=0, sz=0
    let encoding: u32 = 0x2E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rd_0_min_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field Rd = 0 (Min)
    // Fields: Q=0, Rn=0, sz=0, Rd=0, o1=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rd_1_poweroftwo_f800_2e30f801() {
    // Encoding: 0x2E30F801
    // Test aarch64_vector_reduce_fp_max_simd field Rd = 1 (PowerOfTwo)
    // Fields: o1=0, sz=0, Rd=1, Rn=0, Q=0
    let encoding: u32 = 0x2E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rd_30_poweroftwominusone_f800_2e30f81e() {
    // Encoding: 0x2E30F81E
    // Test aarch64_vector_reduce_fp_max_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, o1=0, Q=0, Rd=30, sz=0
    let encoding: u32 = 0x2E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_field_rd_31_max_f800_2e30f81f() {
    // Encoding: 0x2E30F81F
    // Test aarch64_vector_reduce_fp_max_simd field Rd = 31 (Max)
    // Fields: Q=0, sz=0, Rd=31, o1=0, Rn=0
    let encoding: u32 = 0x2E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_0_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, o1=0, sz=0, Q=0, Rn=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_1_f800_6e30f800() {
    // Encoding: 0x6E30F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=1, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sz=0, Q=1, o1=0
    let encoding: u32 = 0x6E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_2_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Q=0, Rd=0, o1=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_3_f800_2eb0f800() {
    // Encoding: 0x2EB0F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=1, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, o1=1, sz=0, Q=0, Rd=0
    let encoding: u32 = 0x2EB0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_4_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rn=0, sz=0, o1=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_5_f800_2e70f800() {
    // Encoding: 0x2E70F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=1, Rn=0, Rd=0
    // Fields: o1=0, Rn=0, Rd=0, Q=0, sz=1
    let encoding: u32 = 0x2E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_6_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: sz=0, o1=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_7_f800_2e30f820() {
    // Encoding: 0x2E30F820
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=1, Rd=0
    // Fields: Q=0, o1=0, Rn=1, Rd=0, sz=0
    let encoding: u32 = 0x2E30F820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_8_f800_2e30fbc0() {
    // Encoding: 0x2E30FBC0
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=30, Rd=0
    // Fields: Q=0, sz=0, Rn=30, o1=0, Rd=0
    let encoding: u32 = 0x2E30FBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_9_f800_2e30fbe0() {
    // Encoding: 0x2E30FBE0
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=31, Rd=0
    // Fields: Q=0, Rd=0, o1=0, sz=0, Rn=31
    let encoding: u32 = 0x2E30FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_10_f800_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, o1=0, sz=0, Q=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_11_f800_2e30f801() {
    // Encoding: 0x2E30F801
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=1
    // Fields: Q=0, o1=0, Rn=0, sz=0, Rd=1
    let encoding: u32 = 0x2E30F801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_12_f800_2e30f81e() {
    // Encoding: 0x2E30F81E
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=30
    // Fields: Q=0, Rd=30, Rn=0, o1=0, sz=0
    let encoding: u32 = 0x2E30F81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_13_f800_2e30f81f() {
    // Encoding: 0x2E30F81F
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=0, Rd=31
    // Fields: sz=0, Q=0, o1=0, Rd=31, Rn=0
    let encoding: u32 = 0x2E30F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_14_f800_2e30f821() {
    // Encoding: 0x2E30F821
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=1, Rd=1
    // Fields: Q=0, sz=0, o1=0, Rn=1, Rd=1
    let encoding: u32 = 0x2E30F821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_combo_15_f800_2e30fbff() {
    // Encoding: 0x2E30FBFF
    // Test aarch64_vector_reduce_fp_max_simd field combination: Q=0, o1=0, sz=0, Rn=31, Rd=31
    // Fields: o1=0, Rd=31, sz=0, Rn=31, Q=0
    let encoding: u32 = 0x2E30FBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_special_q_0_size_variant_0_63488_2e70f800() {
    // Encoding: 0x2E70F800
    // Test aarch64_vector_reduce_fp_max_simd special value Q = 0 (Size variant 0)
    // Fields: o1=0, Rn=0, Q=0, sz=1, Rd=0
    let encoding: u32 = 0x2E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_special_q_1_size_variant_1_63488_6e70f800() {
    // Encoding: 0x6E70F800
    // Test aarch64_vector_reduce_fp_max_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, o1=0, Q=1, Rd=0, sz=1
    let encoding: u32 = 0x6E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_special_sz_0_size_variant_0_63488_2e30f800() {
    // Encoding: 0x2E30F800
    // Test aarch64_vector_reduce_fp_max_simd special value sz = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, Q=0, o1=0, sz=0
    let encoding: u32 = 0x2E30F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_special_sz_1_size_variant_1_63488_2e70f800() {
    // Encoding: 0x2E70F800
    // Test aarch64_vector_reduce_fp_max_simd special value sz = 1 (Size variant 1)
    // Fields: Rn=0, o1=0, Rd=0, sz=1, Q=0
    let encoding: u32 = 0x2E70F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_special_rn_31_stack_pointer_sp_may_require_alignment_63488_2e70fbe0()
 {
    // Encoding: 0x2E70FBE0
    // Test aarch64_vector_reduce_fp_max_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, Rn=31, o1=0, Q=0, Rd=0
    let encoding: u32 = 0x2E70FBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_63488_2e70f81f()
 {
    // Encoding: 0x2E70F81F
    // Test aarch64_vector_reduce_fp_max_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, o1=0, sz=1, Rn=0, Rd=31
    let encoding: u32 = 0x2E70F81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_reg_write_0_0e30f800() {
    // Test aarch64_vector_reduce_fp16_max_simd register write: SimdFromField("d")
    // Encoding: 0x0E30F800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30F800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_sp_rn_0e30fbe0() {
    // Test aarch64_vector_reduce_fp16_max_simd with Rn = SP (31)
    // Encoding: 0x0E30FBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30FBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_fp16_max_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp16_max_simd_zr_rd_0e30f81f() {
    // Test aarch64_vector_reduce_fp16_max_simd with Rd = ZR (31)
    // Encoding: 0x0E30F81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E30F81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_reg_write_0_2e30f800() {
    // Test aarch64_vector_reduce_fp_max_simd register write: SimdFromField("d")
    // Encoding: 0x2E30F800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E30F800;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_sp_rn_2e30fbe0() {
    // Test aarch64_vector_reduce_fp_max_simd with Rn = SP (31)
    // Encoding: 0x2E30FBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E30FBE0;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_fp_max_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_fp_max_simd_zr_rd_2e30f81f() {
    // Test aarch64_vector_reduce_fp_max_simd with Rd = ZR (31)
    // Encoding: 0x2E30F81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2E30F81F;
    write_insn(&mut cpu, 0, encoding);
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
// aarch64_vector_reduce_add_simd Tests
// ============================================================================

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_add_simd_field_q_0_min_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field Q = 0 (Min)
    // Fields: Rn=0, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_add_simd_field_q_1_max_b800_4e31b800() {
    // Encoding: 0x4E31B800
    // Test aarch64_vector_reduce_add_simd field Q = 1 (Max)
    // Fields: Q=1, size=0, Rd=0, Rn=0
    let encoding: u32 = 0x4E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_reduce_add_simd_field_size_0_min_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field size = 0 (Min)
    // Fields: Q=0, Rd=0, Rn=0, size=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_reduce_add_simd_field_size_1_poweroftwo_b800_0e71b800() {
    // Encoding: 0x0E71B800
    // Test aarch64_vector_reduce_add_simd field size = 1 (PowerOfTwo)
    // Fields: Q=0, size=1, Rd=0, Rn=0
    let encoding: u32 = 0x0E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_vector_reduce_add_simd_field_size_2_poweroftwo_b800_0eb1b800() {
    // Encoding: 0x0EB1B800
    // Test aarch64_vector_reduce_add_simd field size = 2 (PowerOfTwo)
    // Fields: Rd=0, Q=0, Rn=0, size=2
    let encoding: u32 = 0x0EB1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_vector_reduce_add_simd_field_size_3_max_b800_0ef1b800() {
    // Encoding: 0x0EF1B800
    // Test aarch64_vector_reduce_add_simd field size = 3 (Max)
    // Fields: Rd=0, Rn=0, size=3, Q=0
    let encoding: u32 = 0x0EF1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rn_0_min_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field Rn = 0 (Min)
    // Fields: Rd=0, Q=0, Rn=0, size=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rn_1_poweroftwo_b800_0e31b820() {
    // Encoding: 0x0E31B820
    // Test aarch64_vector_reduce_add_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, size=0, Q=0
    let encoding: u32 = 0x0E31B820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rn_30_poweroftwominusone_b800_0e31bbc0() {
    // Encoding: 0x0E31BBC0
    // Test aarch64_vector_reduce_add_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, size=0, Rn=30, Rd=0
    let encoding: u32 = 0x0E31BBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rn_31_max_b800_0e31bbe0() {
    // Encoding: 0x0E31BBE0
    // Test aarch64_vector_reduce_add_simd field Rn = 31 (Max)
    // Fields: size=0, Rd=0, Q=0, Rn=31
    let encoding: u32 = 0x0E31BBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rd_0_min_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field Rd = 0 (Min)
    // Fields: size=0, Rd=0, Q=0, Rn=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rd_1_poweroftwo_b800_0e31b801() {
    // Encoding: 0x0E31B801
    // Test aarch64_vector_reduce_add_simd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=1, size=0, Q=0
    let encoding: u32 = 0x0E31B801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rd_30_poweroftwominusone_b800_0e31b81e() {
    // Encoding: 0x0E31B81E
    // Test aarch64_vector_reduce_add_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rn=0, Rd=30, Q=0
    let encoding: u32 = 0x0E31B81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_reduce_add_simd_field_rd_31_max_b800_0e31b81f() {
    // Encoding: 0x0E31B81F
    // Test aarch64_vector_reduce_add_simd field Rd = 31 (Max)
    // Fields: Rn=0, size=0, Q=0, Rd=31
    let encoding: u32 = 0x0E31B81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_0_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: size=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_1_b800_4e31b800() {
    // Encoding: 0x4E31B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=1, size=0, Rn=0, Rd=0
    // Fields: size=0, Q=1, Rn=0, Rd=0
    let encoding: u32 = 0x4E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_2_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, size=0, Rn=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_3_b800_0e71b800() {
    // Encoding: 0x0E71B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=1, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=1, Q=0
    let encoding: u32 = 0x0E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_4_b800_0eb1b800() {
    // Encoding: 0x0EB1B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=2, Rn=0, Rd=0
    // Fields: size=2, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0EB1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_5_b800_0ef1b800() {
    // Encoding: 0x0EF1B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=3, Rn=0, Rd=0
    // Fields: Q=0, size=3, Rd=0, Rn=0
    let encoding: u32 = 0x0EF1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_6_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=0, Q=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_7_b800_0e31b820() {
    // Encoding: 0x0E31B820
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=1, Rd=0
    // Fields: Q=0, Rn=1, Rd=0, size=0
    let encoding: u32 = 0x0E31B820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_8_b800_0e31bbc0() {
    // Encoding: 0x0E31BBC0
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=30, Rd=0
    // Fields: Q=0, size=0, Rd=0, Rn=30
    let encoding: u32 = 0x0E31BBC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_9_b800_0e31bbe0() {
    // Encoding: 0x0E31BBE0
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, Q=0, size=0
    let encoding: u32 = 0x0E31BBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_10_b800_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, size=0, Q=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_11_b800_0e31b801() {
    // Encoding: 0x0E31B801
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=1
    // Fields: Rd=1, size=0, Q=0, Rn=0
    let encoding: u32 = 0x0E31B801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_12_b800_0e31b81e() {
    // Encoding: 0x0E31B81E
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=30
    // Fields: Q=0, Rn=0, Rd=30, size=0
    let encoding: u32 = 0x0E31B81E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_13_b800_0e31b81f() {
    // Encoding: 0x0E31B81F
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=0, Rd=31
    // Fields: size=0, Rd=31, Q=0, Rn=0
    let encoding: u32 = 0x0E31B81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_14_b800_0e31b821() {
    // Encoding: 0x0E31B821
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=1, Rd=1
    // Fields: size=0, Q=0, Rd=1, Rn=1
    let encoding: u32 = 0x0E31B821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_reduce_add_simd_combo_15_b800_0e31bbff() {
    // Encoding: 0x0E31BBFF
    // Test aarch64_vector_reduce_add_simd field combination: Q=0, size=0, Rn=31, Rd=31
    // Fields: Rd=31, size=0, Rn=31, Q=0
    let encoding: u32 = 0x0E31BBFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_add_simd_special_q_0_size_variant_0_47104_0e71b800() {
    // Encoding: 0x0E71B800
    // Test aarch64_vector_reduce_add_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, Rn=0, size=1, Rd=0
    let encoding: u32 = 0x0E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_add_simd_special_q_1_size_variant_1_47104_4e71b800() {
    // Encoding: 0x4E71B800
    // Test aarch64_vector_reduce_add_simd special value Q = 1 (Size variant 1)
    // Fields: Rd=0, Q=1, size=1, Rn=0
    let encoding: u32 = 0x4E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_reduce_add_simd_special_size_0_size_variant_0_47104_0e31b800() {
    // Encoding: 0x0E31B800
    // Test aarch64_vector_reduce_add_simd special value size = 0 (Size variant 0)
    // Fields: size=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0E31B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_reduce_add_simd_special_size_1_size_variant_1_47104_0e71b800() {
    // Encoding: 0x0E71B800
    // Test aarch64_vector_reduce_add_simd special value size = 1 (Size variant 1)
    // Fields: size=1, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0E71B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_vector_reduce_add_simd_special_size_2_size_variant_2_47104_0eb1b800() {
    // Encoding: 0x0EB1B800
    // Test aarch64_vector_reduce_add_simd special value size = 2 (Size variant 2)
    // Fields: Rn=0, Rd=0, size=2, Q=0
    let encoding: u32 = 0x0EB1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_vector_reduce_add_simd_special_size_3_size_variant_3_47104_0ef1b800() {
    // Encoding: 0x0EF1B800
    // Test aarch64_vector_reduce_add_simd special value size = 3 (Size variant 3)
    // Fields: Q=0, Rd=0, Rn=0, size=3
    let encoding: u32 = 0x0EF1B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_reduce_add_simd_special_rn_31_stack_pointer_sp_may_require_alignment_47104_0e71bbe0()
 {
    // Encoding: 0x0E71BBE0
    // Test aarch64_vector_reduce_add_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, size=1, Q=0, Rn=31
    let encoding: u32 = 0x0E71BBE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_reduce_add_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_47104_0e71b81f()
 {
    // Encoding: 0x0E71B81F
    // Test aarch64_vector_reduce_add_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, Rd=31, Rn=0, size=1
    let encoding: u32 = 0x0E71B81F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_reduce_add_simd_reg_write_0_0e31b800() {
    // Test aarch64_vector_reduce_add_simd register write: SimdFromField("d")
    // Encoding: 0x0E31B800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E31B800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_reduce_add_simd_sp_rn_0e31bbe0() {
    // Test aarch64_vector_reduce_add_simd with Rn = SP (31)
    // Encoding: 0x0E31BBE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E31BBE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_reduce_add_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_reduce_add_simd_zr_rd_0e31b81f() {
    // Test aarch64_vector_reduce_add_simd with Rd = ZR (31)
    // Encoding: 0x0E31B81F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0E31B81F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
