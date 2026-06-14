//! A64 float compare tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_float_compare_uncond Tests
// ============================================================================

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_compare_uncond_field_type1_0_min_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field type1 = 0 (Min)
    // Fields: type1=0, Rm=0, opc=0, Rn=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_compare_uncond_field_type1_1_poweroftwo_2000_1e602000() {
    // Encoding: 0x1E602000
    // Test aarch64_float_compare_uncond field type1 = 1 (PowerOfTwo)
    // Fields: Rm=0, type1=1, opc=0, Rn=0
    let encoding: u32 = 0x1E602000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_compare_uncond_field_type1_3_max_2000_1ee02000() {
    // Encoding: 0x1EE02000
    // Test aarch64_float_compare_uncond field type1 = 3 (Max)
    // Fields: opc=0, Rn=0, type1=3, Rm=0
    let encoding: u32 = 0x1EE02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_compare_uncond_field_rm_0_min_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field Rm = 0 (Min)
    // Fields: Rm=0, type1=0, Rn=0, opc=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_compare_uncond_field_rm_1_poweroftwo_2000_1e212000() {
    // Encoding: 0x1E212000
    // Test aarch64_float_compare_uncond field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, opc=0, type1=0, Rm=1
    let encoding: u32 = 0x1E212000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_compare_uncond_field_rm_30_poweroftwominusone_2000_1e3e2000() {
    // Encoding: 0x1E3E2000
    // Test aarch64_float_compare_uncond field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=30, opc=0, type1=0, Rn=0
    let encoding: u32 = 0x1E3E2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_float_compare_uncond_field_rm_31_max_2000_1e3f2000() {
    // Encoding: 0x1E3F2000
    // Test aarch64_float_compare_uncond field Rm = 31 (Max)
    // Fields: Rm=31, type1=0, Rn=0, opc=0
    let encoding: u32 = 0x1E3F2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_compare_uncond_field_rn_0_min_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field Rn = 0 (Min)
    // Fields: Rm=0, type1=0, opc=0, Rn=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_compare_uncond_field_rn_1_poweroftwo_2000_1e202020() {
    // Encoding: 0x1E202020
    // Test aarch64_float_compare_uncond field Rn = 1 (PowerOfTwo)
    // Fields: Rm=0, opc=0, type1=0, Rn=1
    let encoding: u32 = 0x1E202020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_compare_uncond_field_rn_30_poweroftwominusone_2000_1e2023c0() {
    // Encoding: 0x1E2023C0
    // Test aarch64_float_compare_uncond field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rn=30, Rm=0, opc=0
    let encoding: u32 = 0x1E2023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_float_compare_uncond_field_rn_31_max_2000_1e2023e0() {
    // Encoding: 0x1E2023E0
    // Test aarch64_float_compare_uncond field Rn = 31 (Max)
    // Fields: type1=0, opc=0, Rm=0, Rn=31
    let encoding: u32 = 0x1E2023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc 3 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_float_compare_uncond_field_opc_0_min_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field opc = 0 (Min)
    // Fields: Rm=0, type1=0, opc=0, Rn=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc 3 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_float_compare_uncond_field_opc_1_poweroftwo_2000_1e202008() {
    // Encoding: 0x1E202008
    // Test aarch64_float_compare_uncond field opc = 1 (PowerOfTwo)
    // Fields: type1=0, Rn=0, Rm=0, opc=1
    let encoding: u32 = 0x1E202008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc 3 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_float_compare_uncond_field_opc_2_poweroftwo_2000_1e202010() {
    // Encoding: 0x1E202010
    // Test aarch64_float_compare_uncond field opc = 2 (PowerOfTwo)
    // Fields: type1=0, Rm=0, Rn=0, opc=2
    let encoding: u32 = 0x1E202010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc 3 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_float_compare_uncond_field_opc_3_max_2000_1e202018() {
    // Encoding: 0x1E202018
    // Test aarch64_float_compare_uncond field opc = 3 (Max)
    // Fields: type1=0, Rm=0, Rn=0, opc=3
    let encoding: u32 = 0x1E202018;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_compare_uncond_combo_0_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=0
    // Fields: opc=0, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_compare_uncond_combo_1_2000_1e602000() {
    // Encoding: 0x1E602000
    // Test aarch64_float_compare_uncond field combination: type1=1, Rm=0, Rn=0, opc=0
    // Fields: Rn=0, Rm=0, opc=0, type1=1
    let encoding: u32 = 0x1E602000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_compare_uncond_combo_2_2000_1ee02000() {
    // Encoding: 0x1EE02000
    // Test aarch64_float_compare_uncond field combination: type1=3, Rm=0, Rn=0, opc=0
    // Fields: Rn=0, opc=0, Rm=0, type1=3
    let encoding: u32 = 0x1EE02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_compare_uncond_combo_3_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=0
    // Fields: Rn=0, type1=0, Rm=0, opc=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_compare_uncond_combo_4_2000_1e212000() {
    // Encoding: 0x1E212000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=1, Rn=0, opc=0
    // Fields: opc=0, Rm=1, Rn=0, type1=0
    let encoding: u32 = 0x1E212000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_compare_uncond_combo_5_2000_1e3e2000() {
    // Encoding: 0x1E3E2000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=30, Rn=0, opc=0
    // Fields: Rn=0, type1=0, opc=0, Rm=30
    let encoding: u32 = 0x1E3E2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_float_compare_uncond_combo_6_2000_1e3f2000() {
    // Encoding: 0x1E3F2000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=31, Rn=0, opc=0
    // Fields: Rm=31, Rn=0, type1=0, opc=0
    let encoding: u32 = 0x1E3F2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_compare_uncond_combo_7_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=0
    // Fields: type1=0, Rn=0, opc=0, Rm=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_compare_uncond_combo_8_2000_1e202020() {
    // Encoding: 0x1E202020
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=1, opc=0
    // Fields: Rm=0, type1=0, Rn=1, opc=0
    let encoding: u32 = 0x1E202020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_compare_uncond_combo_9_2000_1e2023c0() {
    // Encoding: 0x1E2023C0
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=30, opc=0
    // Fields: type1=0, Rm=0, Rn=30, opc=0
    let encoding: u32 = 0x1E2023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_float_compare_uncond_combo_10_2000_1e2023e0() {
    // Encoding: 0x1E2023E0
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=31, opc=0
    // Fields: type1=0, Rm=0, opc=0, Rn=31
    let encoding: u32 = 0x1E2023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_float_compare_uncond_combo_11_2000_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=0
    // Fields: opc=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_float_compare_uncond_combo_12_2000_1e202008() {
    // Encoding: 0x1E202008
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=1
    // Fields: type1=0, Rn=0, opc=1, Rm=0
    let encoding: u32 = 0x1E202008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_float_compare_uncond_combo_13_2000_1e202010() {
    // Encoding: 0x1E202010
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=2
    // Fields: Rm=0, Rn=0, opc=2, type1=0
    let encoding: u32 = 0x1E202010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_float_compare_uncond_combo_14_2000_1e202018() {
    // Encoding: 0x1E202018
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=0, Rn=0, opc=3
    // Fields: Rn=0, Rm=0, type1=0, opc=3
    let encoding: u32 = 0x1E202018;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_compare_uncond_combo_15_2000_1e212020() {
    // Encoding: 0x1E212020
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=1, Rn=1, opc=0
    // Fields: Rm=1, opc=0, type1=0, Rn=1
    let encoding: u32 = 0x1E212020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_compare_uncond_combo_16_2000_1e3f23e0() {
    // Encoding: 0x1E3F23E0
    // Test aarch64_float_compare_uncond field combination: type1=0, Rm=31, Rn=31, opc=0
    // Fields: type1=0, Rm=31, Rn=31, opc=0
    let encoding: u32 = 0x1E3F23E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_float_compare_uncond_special_rn_31_stack_pointer_sp_may_require_alignment_8192_1e2023e0()
 {
    // Encoding: 0x1E2023E0
    // Test aarch64_float_compare_uncond special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: type1=0, Rn=31, opc=0, Rm=0
    let encoding: u32 = 0x1E2023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_float_compare_uncond_special_opc_0_size_variant_0_8192_1e202000() {
    // Encoding: 0x1E202000
    // Test aarch64_float_compare_uncond special value opc = 0 (Size variant 0)
    // Fields: Rm=0, opc=0, type1=0, Rn=0
    let encoding: u32 = 0x1E202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_float_compare_uncond_special_opc_1_size_variant_1_8192_1e202008() {
    // Encoding: 0x1E202008
    // Test aarch64_float_compare_uncond special value opc = 1 (Size variant 1)
    // Fields: opc=1, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E202008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_float_compare_uncond_special_opc_2_size_variant_2_8192_1e202010() {
    // Encoding: 0x1E202010
    // Test aarch64_float_compare_uncond special value opc = 2 (Size variant 2)
    // Fields: Rm=0, opc=2, type1=0, Rn=0
    let encoding: u32 = 0x1E202010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_float_compare_uncond_special_opc_3_size_variant_3_8192_1e202018() {
    // Encoding: 0x1E202018
    // Test aarch64_float_compare_uncond special value opc = 3 (Size variant 3)
    // Fields: type1=0, Rn=0, Rm=0, opc=3
    let encoding: u32 = 0x1E202018;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_float_compare_uncond_sp_rn_1e2023e0() {
    // Test aarch64_float_compare_uncond with Rn = SP (31)
    // Encoding: 0x1E2023E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1E2023E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_zeroresult_0_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: ZeroResult
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_zeroresult_1_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: ZeroResult
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_negativeresult_2_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: NegativeResult
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_unsignedoverflow_3_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: UnsignedOverflow
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_unsignedoverflow_4_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: UnsignedOverflow
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_signedoverflow_5_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: SignedOverflow
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_float_compare_uncond_flags_signedoverflow_6_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: SignedOverflow
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_uncond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_float_compare_uncond_flags_positiveresult_7_1e222020() {
    // Test aarch64_float_compare_uncond flag computation: PositiveResult
    // Encoding: 0x1E222020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x32);
    set_x(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x1E222020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_float_compare_cond Tests
// ============================================================================

/// Provenance: aarch64_float_compare_cond
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_compare_cond_field_type1_0_min_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field type1 = 0 (Min)
    // Fields: type1=0, cond=0, nzcv=0, Rm=0, Rn=0, op=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_compare_cond_field_type1_1_poweroftwo_400_1e600400() {
    // Encoding: 0x1E600400
    // Test aarch64_float_compare_cond field type1 = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, nzcv=0, type1=1, op=0, cond=0
    let encoding: u32 = 0x1E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_compare_cond_field_type1_3_max_400_1ee00400() {
    // Encoding: 0x1EE00400
    // Test aarch64_float_compare_cond field type1 = 3 (Max)
    // Fields: cond=0, Rm=0, op=0, type1=3, nzcv=0, Rn=0
    let encoding: u32 = 0x1EE00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_compare_cond_field_rm_0_min_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field Rm = 0 (Min)
    // Fields: type1=0, Rn=0, cond=0, Rm=0, op=0, nzcv=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_compare_cond_field_rm_1_poweroftwo_400_1e210400() {
    // Encoding: 0x1E210400
    // Test aarch64_float_compare_cond field Rm = 1 (PowerOfTwo)
    // Fields: cond=0, type1=0, nzcv=0, op=0, Rn=0, Rm=1
    let encoding: u32 = 0x1E210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_compare_cond_field_rm_30_poweroftwominusone_400_1e3e0400() {
    // Encoding: 0x1E3E0400
    // Test aarch64_float_compare_cond field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rn=0, nzcv=0, Rm=30, op=0, cond=0
    let encoding: u32 = 0x1E3E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_float_compare_cond_field_rm_31_max_400_1e3f0400() {
    // Encoding: 0x1E3F0400
    // Test aarch64_float_compare_cond field Rm = 31 (Max)
    // Fields: nzcv=0, type1=0, Rm=31, Rn=0, op=0, cond=0
    let encoding: u32 = 0x1E3F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch64_float_compare_cond_field_cond_0_min_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field cond = 0 (Min)
    // Fields: Rm=0, nzcv=0, Rn=0, op=0, cond=0, type1=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch64_float_compare_cond_field_cond_1_poweroftwo_400_1e201400() {
    // Encoding: 0x1E201400
    // Test aarch64_float_compare_cond field cond = 1 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, nzcv=0, type1=0, cond=1, op=0
    let encoding: u32 = 0x1E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch64_float_compare_cond_field_cond_2_poweroftwo_400_1e202400() {
    // Encoding: 0x1E202400
    // Test aarch64_float_compare_cond field cond = 2 (PowerOfTwo)
    // Fields: op=0, nzcv=0, Rn=0, type1=0, cond=2, Rm=0
    let encoding: u32 = 0x1E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch64_float_compare_cond_field_cond_3_poweroftwo_400_1e203400() {
    // Encoding: 0x1E203400
    // Test aarch64_float_compare_cond field cond = 3 (PowerOfTwo)
    // Fields: type1=0, cond=3, Rn=0, Rm=0, op=0, nzcv=0
    let encoding: u32 = 0x1E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch64_float_compare_cond_field_cond_4_poweroftwo_400_1e204400() {
    // Encoding: 0x1E204400
    // Test aarch64_float_compare_cond field cond = 4 (PowerOfTwo)
    // Fields: type1=0, nzcv=0, op=0, cond=4, Rm=0, Rn=0
    let encoding: u32 = 0x1E204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch64_float_compare_cond_field_cond_5_poweroftwo_400_1e205400() {
    // Encoding: 0x1E205400
    // Test aarch64_float_compare_cond field cond = 5 (PowerOfTwo)
    // Fields: Rm=0, Rn=0, nzcv=0, type1=0, op=0, cond=5
    let encoding: u32 = 0x1E205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch64_float_compare_cond_field_cond_6_poweroftwo_400_1e206400() {
    // Encoding: 0x1E206400
    // Test aarch64_float_compare_cond field cond = 6 (PowerOfTwo)
    // Fields: nzcv=0, Rm=0, type1=0, Rn=0, cond=6, op=0
    let encoding: u32 = 0x1E206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch64_float_compare_cond_field_cond_7_poweroftwo_400_1e207400() {
    // Encoding: 0x1E207400
    // Test aarch64_float_compare_cond field cond = 7 (PowerOfTwo)
    // Fields: Rm=0, op=0, Rn=0, cond=7, type1=0, nzcv=0
    let encoding: u32 = 0x1E207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch64_float_compare_cond_field_cond_8_poweroftwo_400_1e208400() {
    // Encoding: 0x1E208400
    // Test aarch64_float_compare_cond field cond = 8 (PowerOfTwo)
    // Fields: cond=8, Rm=0, Rn=0, op=0, nzcv=0, type1=0
    let encoding: u32 = 0x1E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch64_float_compare_cond_field_cond_9_poweroftwo_400_1e209400() {
    // Encoding: 0x1E209400
    // Test aarch64_float_compare_cond field cond = 9 (PowerOfTwo)
    // Fields: op=0, Rm=0, cond=9, Rn=0, type1=0, nzcv=0
    let encoding: u32 = 0x1E209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch64_float_compare_cond_field_cond_10_poweroftwo_400_1e20a400() {
    // Encoding: 0x1E20A400
    // Test aarch64_float_compare_cond field cond = 10 (PowerOfTwo)
    // Fields: Rm=0, cond=10, op=0, type1=0, nzcv=0, Rn=0
    let encoding: u32 = 0x1E20A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch64_float_compare_cond_field_cond_11_poweroftwo_400_1e20b400() {
    // Encoding: 0x1E20B400
    // Test aarch64_float_compare_cond field cond = 11 (PowerOfTwo)
    // Fields: nzcv=0, op=0, type1=0, cond=11, Rm=0, Rn=0
    let encoding: u32 = 0x1E20B400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch64_float_compare_cond_field_cond_12_poweroftwo_400_1e20c400() {
    // Encoding: 0x1E20C400
    // Test aarch64_float_compare_cond field cond = 12 (PowerOfTwo)
    // Fields: Rm=0, nzcv=0, cond=12, type1=0, Rn=0, op=0
    let encoding: u32 = 0x1E20C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch64_float_compare_cond_field_cond_13_poweroftwo_400_1e20d400() {
    // Encoding: 0x1E20D400
    // Test aarch64_float_compare_cond field cond = 13 (PowerOfTwo)
    // Fields: op=0, nzcv=0, Rm=0, cond=13, Rn=0, type1=0
    let encoding: u32 = 0x1E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch64_float_compare_cond_field_cond_14_poweroftwo_400_1e20e400() {
    // Encoding: 0x1E20E400
    // Test aarch64_float_compare_cond field cond = 14 (PowerOfTwo)
    // Fields: nzcv=0, type1=0, Rm=0, cond=14, op=0, Rn=0
    let encoding: u32 = 0x1E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch64_float_compare_cond_field_cond_15_max_400_1e20f400() {
    // Encoding: 0x1E20F400
    // Test aarch64_float_compare_cond field cond = 15 (Max)
    // Fields: op=0, type1=0, Rm=0, cond=15, Rn=0, nzcv=0
    let encoding: u32 = 0x1E20F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_compare_cond_field_rn_0_min_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field Rn = 0 (Min)
    // Fields: op=0, Rm=0, nzcv=0, cond=0, type1=0, Rn=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_compare_cond_field_rn_1_poweroftwo_400_1e200420() {
    // Encoding: 0x1E200420
    // Test aarch64_float_compare_cond field Rn = 1 (PowerOfTwo)
    // Fields: Rm=0, type1=0, cond=0, nzcv=0, op=0, Rn=1
    let encoding: u32 = 0x1E200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_compare_cond_field_rn_30_poweroftwominusone_400_1e2007c0() {
    // Encoding: 0x1E2007C0
    // Test aarch64_float_compare_cond field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: cond=0, nzcv=0, Rn=30, type1=0, op=0, Rm=0
    let encoding: u32 = 0x1E2007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_float_compare_cond_field_rn_31_max_400_1e2007e0() {
    // Encoding: 0x1E2007E0
    // Test aarch64_float_compare_cond field Rn = 31 (Max)
    // Fields: Rn=31, type1=0, Rm=0, cond=0, op=0, nzcv=0
    let encoding: u32 = 0x1E2007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field op 4 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_compare_cond_field_op_0_min_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field op = 0 (Min)
    // Fields: Rn=0, type1=0, Rm=0, op=0, nzcv=0, cond=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field op 4 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_float_compare_cond_field_op_1_max_400_1e200410() {
    // Encoding: 0x1E200410
    // Test aarch64_float_compare_cond field op = 1 (Max)
    // Fields: Rn=0, nzcv=0, op=1, Rm=0, cond=0, type1=0
    let encoding: u32 = 0x1E200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_compare_cond_field_nzcv_0_min_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field nzcv = 0 (Min)
    // Fields: Rn=0, cond=0, nzcv=0, op=0, type1=0, Rm=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_compare_cond_field_nzcv_1_poweroftwo_400_1e200401() {
    // Encoding: 0x1E200401
    // Test aarch64_float_compare_cond field nzcv = 1 (PowerOfTwo)
    // Fields: type1=0, Rn=0, nzcv=1, cond=0, op=0, Rm=0
    let encoding: u32 = 0x1E200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_float_compare_cond_field_nzcv_7_poweroftwominusone_400_1e200407() {
    // Encoding: 0x1E200407
    // Test aarch64_float_compare_cond field nzcv = 7 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rn=0, op=0, cond=0, nzcv=7, Rm=0
    let encoding: u32 = 0x1E200407;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_float_compare_cond_field_nzcv_15_max_400_1e20040f() {
    // Encoding: 0x1E20040F
    // Test aarch64_float_compare_cond field nzcv = 15 (Max)
    // Fields: Rn=0, type1=0, Rm=0, cond=0, op=0, nzcv=15
    let encoding: u32 = 0x1E20040F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_compare_cond_combo_0_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: cond=0, type1=0, Rm=0, Rn=0, op=0, nzcv=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_compare_cond_combo_1_400_1e600400() {
    // Encoding: 0x1E600400
    // Test aarch64_float_compare_cond field combination: type1=1, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: op=0, cond=0, type1=1, Rm=0, Rn=0, nzcv=0
    let encoding: u32 = 0x1E600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_compare_cond_combo_2_400_1ee00400() {
    // Encoding: 0x1EE00400
    // Test aarch64_float_compare_cond field combination: type1=3, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: nzcv=0, type1=3, Rn=0, op=0, Rm=0, cond=0
    let encoding: u32 = 0x1EE00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_compare_cond_combo_3_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: nzcv=0, cond=0, Rm=0, type1=0, op=0, Rn=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_compare_cond_combo_4_400_1e210400() {
    // Encoding: 0x1E210400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=1, cond=0, Rn=0, op=0, nzcv=0
    // Fields: type1=0, Rm=1, cond=0, Rn=0, op=0, nzcv=0
    let encoding: u32 = 0x1E210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_compare_cond_combo_5_400_1e3e0400() {
    // Encoding: 0x1E3E0400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=30, cond=0, Rn=0, op=0, nzcv=0
    // Fields: cond=0, nzcv=0, Rm=30, op=0, type1=0, Rn=0
    let encoding: u32 = 0x1E3E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_float_compare_cond_combo_6_400_1e3f0400() {
    // Encoding: 0x1E3F0400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=31, cond=0, Rn=0, op=0, nzcv=0
    // Fields: Rm=31, nzcv=0, cond=0, type1=0, Rn=0, op=0
    let encoding: u32 = 0x1E3F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch64_float_compare_cond_combo_7_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: op=0, type1=0, nzcv=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=1 (condition NE (not equal))
#[test]
fn test_aarch64_float_compare_cond_combo_8_400_1e201400() {
    // Encoding: 0x1E201400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=1, Rn=0, op=0, nzcv=0
    // Fields: nzcv=0, Rm=0, type1=0, cond=1, op=0, Rn=0
    let encoding: u32 = 0x1E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=2 (condition CS/HS (carry set))
#[test]
fn test_aarch64_float_compare_cond_combo_9_400_1e202400() {
    // Encoding: 0x1E202400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=2, Rn=0, op=0, nzcv=0
    // Fields: Rn=0, op=0, nzcv=0, cond=2, type1=0, Rm=0
    let encoding: u32 = 0x1E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=3 (condition CC/LO (carry clear))
#[test]
fn test_aarch64_float_compare_cond_combo_10_400_1e203400() {
    // Encoding: 0x1E203400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=3, Rn=0, op=0, nzcv=0
    // Fields: Rm=0, cond=3, Rn=0, op=0, nzcv=0, type1=0
    let encoding: u32 = 0x1E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=4 (condition MI (minus/negative))
#[test]
fn test_aarch64_float_compare_cond_combo_11_400_1e204400() {
    // Encoding: 0x1E204400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=4, Rn=0, op=0, nzcv=0
    // Fields: Rn=0, type1=0, cond=4, op=0, nzcv=0, Rm=0
    let encoding: u32 = 0x1E204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=5 (condition PL (plus/positive))
#[test]
fn test_aarch64_float_compare_cond_combo_12_400_1e205400() {
    // Encoding: 0x1E205400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=5, Rn=0, op=0, nzcv=0
    // Fields: nzcv=0, type1=0, cond=5, Rm=0, Rn=0, op=0
    let encoding: u32 = 0x1E205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=6 (condition VS (overflow set))
#[test]
fn test_aarch64_float_compare_cond_combo_13_400_1e206400() {
    // Encoding: 0x1E206400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=6, Rn=0, op=0, nzcv=0
    // Fields: type1=0, cond=6, Rn=0, op=0, Rm=0, nzcv=0
    let encoding: u32 = 0x1E206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=7 (condition VC (overflow clear))
#[test]
fn test_aarch64_float_compare_cond_combo_14_400_1e207400() {
    // Encoding: 0x1E207400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=7, Rn=0, op=0, nzcv=0
    // Fields: type1=0, cond=7, nzcv=0, Rn=0, op=0, Rm=0
    let encoding: u32 = 0x1E207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=8 (condition HI (unsigned higher))
#[test]
fn test_aarch64_float_compare_cond_combo_15_400_1e208400() {
    // Encoding: 0x1E208400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=8, Rn=0, op=0, nzcv=0
    // Fields: nzcv=0, Rm=0, type1=0, cond=8, Rn=0, op=0
    let encoding: u32 = 0x1E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=9 (condition LS (unsigned lower or same))
#[test]
fn test_aarch64_float_compare_cond_combo_16_400_1e209400() {
    // Encoding: 0x1E209400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=9, Rn=0, op=0, nzcv=0
    // Fields: type1=0, op=0, nzcv=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x1E209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=10 (condition GE (signed >=))
#[test]
fn test_aarch64_float_compare_cond_combo_17_400_1e20a400() {
    // Encoding: 0x1E20A400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=10, Rn=0, op=0, nzcv=0
    // Fields: type1=0, op=0, Rn=0, Rm=0, nzcv=0, cond=10
    let encoding: u32 = 0x1E20A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=11 (condition LT (signed <))
#[test]
fn test_aarch64_float_compare_cond_combo_18_400_1e20b400() {
    // Encoding: 0x1E20B400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=11, Rn=0, op=0, nzcv=0
    // Fields: type1=0, Rn=0, Rm=0, cond=11, nzcv=0, op=0
    let encoding: u32 = 0x1E20B400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=12 (condition GT (signed >))
#[test]
fn test_aarch64_float_compare_cond_combo_19_400_1e20c400() {
    // Encoding: 0x1E20C400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=12, Rn=0, op=0, nzcv=0
    // Fields: Rm=0, cond=12, nzcv=0, type1=0, Rn=0, op=0
    let encoding: u32 = 0x1E20C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=13 (condition LE (signed <=))
#[test]
fn test_aarch64_float_compare_cond_combo_20_400_1e20d400() {
    // Encoding: 0x1E20D400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=13, Rn=0, op=0, nzcv=0
    // Fields: nzcv=0, type1=0, Rn=0, Rm=0, cond=13, op=0
    let encoding: u32 = 0x1E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=14 (condition AL (always))
#[test]
fn test_aarch64_float_compare_cond_combo_21_400_1e20e400() {
    // Encoding: 0x1E20E400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=14, Rn=0, op=0, nzcv=0
    // Fields: Rn=0, op=0, Rm=0, nzcv=0, cond=14, type1=0
    let encoding: u32 = 0x1E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=15 (condition NV (never, reserved))
#[test]
fn test_aarch64_float_compare_cond_combo_22_400_1e20f400() {
    // Encoding: 0x1E20F400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=15, Rn=0, op=0, nzcv=0
    // Fields: Rm=0, type1=0, op=0, nzcv=0, cond=15, Rn=0
    let encoding: u32 = 0x1E20F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_compare_cond_combo_23_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: cond=0, Rn=0, op=0, nzcv=0, Rm=0, type1=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_compare_cond_combo_24_400_1e200420() {
    // Encoding: 0x1E200420
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=1, op=0, nzcv=0
    // Fields: op=0, nzcv=0, Rn=1, Rm=0, cond=0, type1=0
    let encoding: u32 = 0x1E200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_compare_cond_combo_25_400_1e2007c0() {
    // Encoding: 0x1E2007C0
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=30, op=0, nzcv=0
    // Fields: nzcv=0, Rn=30, op=0, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x1E2007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_float_compare_cond_combo_26_400_1e2007e0() {
    // Encoding: 0x1E2007E0
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=31, op=0, nzcv=0
    // Fields: cond=0, nzcv=0, Rn=31, type1=0, Rm=0, op=0
    let encoding: u32 = 0x1E2007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_float_compare_cond_combo_27_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: type1=0, op=0, nzcv=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_float_compare_cond_combo_28_400_1e200410() {
    // Encoding: 0x1E200410
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=1, nzcv=0
    // Fields: nzcv=0, cond=0, Rm=0, Rn=0, op=1, type1=0
    let encoding: u32 = 0x1E200410;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=0 (minimum value)
#[test]
fn test_aarch64_float_compare_cond_combo_29_400_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=0
    // Fields: type1=0, Rn=0, op=0, nzcv=0, Rm=0, cond=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=1 (value 1)
#[test]
fn test_aarch64_float_compare_cond_combo_30_400_1e200401() {
    // Encoding: 0x1E200401
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=1
    // Fields: nzcv=1, Rm=0, op=0, type1=0, Rn=0, cond=0
    let encoding: u32 = 0x1E200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=7 (midpoint (7))
#[test]
fn test_aarch64_float_compare_cond_combo_31_400_1e200407() {
    // Encoding: 0x1E200407
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=7
    // Fields: nzcv=7, op=0, cond=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x1E200407;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=15 (maximum value (15))
#[test]
fn test_aarch64_float_compare_cond_combo_32_400_1e20040f() {
    // Encoding: 0x1E20040F
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=0, cond=0, Rn=0, op=0, nzcv=15
    // Fields: Rn=0, nzcv=15, Rm=0, type1=0, cond=0, op=0
    let encoding: u32 = 0x1E20040F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_compare_cond_combo_33_400_1e210420() {
    // Encoding: 0x1E210420
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=1, cond=0, Rn=1, op=0, nzcv=0
    // Fields: Rm=1, type1=0, nzcv=0, Rn=1, cond=0, op=0
    let encoding: u32 = 0x1E210420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_compare_cond_combo_34_400_1e3f07e0() {
    // Encoding: 0x1E3F07E0
    // Test aarch64_float_compare_cond field combination: type1=0, Rm=31, cond=0, Rn=31, op=0, nzcv=0
    // Fields: Rn=31, type1=0, cond=0, op=0, Rm=31, nzcv=0
    let encoding: u32 = 0x1E3F07E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch64_float_compare_cond_special_cond_0_condition_eq_1024_1e200400() {
    // Encoding: 0x1E200400
    // Test aarch64_float_compare_cond special value cond = 0 (Condition EQ)
    // Fields: type1=0, nzcv=0, Rn=0, op=0, Rm=0, cond=0
    let encoding: u32 = 0x1E200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch64_float_compare_cond_special_cond_1_condition_ne_1024_1e201400() {
    // Encoding: 0x1E201400
    // Test aarch64_float_compare_cond special value cond = 1 (Condition NE)
    // Fields: cond=1, Rn=0, Rm=0, type1=0, op=0, nzcv=0
    let encoding: u32 = 0x1E201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch64_float_compare_cond_special_cond_2_condition_cs_hs_1024_1e202400() {
    // Encoding: 0x1E202400
    // Test aarch64_float_compare_cond special value cond = 2 (Condition CS/HS)
    // Fields: cond=2, type1=0, op=0, nzcv=0, Rn=0, Rm=0
    let encoding: u32 = 0x1E202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch64_float_compare_cond_special_cond_3_condition_cc_lo_1024_1e203400() {
    // Encoding: 0x1E203400
    // Test aarch64_float_compare_cond special value cond = 3 (Condition CC/LO)
    // Fields: type1=0, Rm=0, cond=3, Rn=0, op=0, nzcv=0
    let encoding: u32 = 0x1E203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch64_float_compare_cond_special_cond_4_condition_mi_1024_1e204400() {
    // Encoding: 0x1E204400
    // Test aarch64_float_compare_cond special value cond = 4 (Condition MI)
    // Fields: cond=4, Rm=0, nzcv=0, Rn=0, op=0, type1=0
    let encoding: u32 = 0x1E204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch64_float_compare_cond_special_cond_5_condition_pl_1024_1e205400() {
    // Encoding: 0x1E205400
    // Test aarch64_float_compare_cond special value cond = 5 (Condition PL)
    // Fields: type1=0, cond=5, Rm=0, nzcv=0, Rn=0, op=0
    let encoding: u32 = 0x1E205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch64_float_compare_cond_special_cond_6_condition_vs_1024_1e206400() {
    // Encoding: 0x1E206400
    // Test aarch64_float_compare_cond special value cond = 6 (Condition VS)
    // Fields: cond=6, type1=0, Rn=0, nzcv=0, Rm=0, op=0
    let encoding: u32 = 0x1E206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch64_float_compare_cond_special_cond_7_condition_vc_1024_1e207400() {
    // Encoding: 0x1E207400
    // Test aarch64_float_compare_cond special value cond = 7 (Condition VC)
    // Fields: op=0, cond=7, nzcv=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch64_float_compare_cond_special_cond_8_condition_hi_1024_1e208400() {
    // Encoding: 0x1E208400
    // Test aarch64_float_compare_cond special value cond = 8 (Condition HI)
    // Fields: op=0, nzcv=0, Rm=0, Rn=0, type1=0, cond=8
    let encoding: u32 = 0x1E208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch64_float_compare_cond_special_cond_9_condition_ls_1024_1e209400() {
    // Encoding: 0x1E209400
    // Test aarch64_float_compare_cond special value cond = 9 (Condition LS)
    // Fields: Rm=0, cond=9, Rn=0, op=0, nzcv=0, type1=0
    let encoding: u32 = 0x1E209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch64_float_compare_cond_special_cond_10_condition_ge_1024_1e20a400() {
    // Encoding: 0x1E20A400
    // Test aarch64_float_compare_cond special value cond = 10 (Condition GE)
    // Fields: type1=0, op=0, nzcv=0, cond=10, Rm=0, Rn=0
    let encoding: u32 = 0x1E20A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch64_float_compare_cond_special_cond_11_condition_lt_1024_1e20b400() {
    // Encoding: 0x1E20B400
    // Test aarch64_float_compare_cond special value cond = 11 (Condition LT)
    // Fields: op=0, cond=11, Rm=0, nzcv=0, type1=0, Rn=0
    let encoding: u32 = 0x1E20B400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch64_float_compare_cond_special_cond_12_condition_gt_1024_1e20c400() {
    // Encoding: 0x1E20C400
    // Test aarch64_float_compare_cond special value cond = 12 (Condition GT)
    // Fields: op=0, nzcv=0, type1=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0x1E20C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch64_float_compare_cond_special_cond_13_condition_le_1024_1e20d400() {
    // Encoding: 0x1E20D400
    // Test aarch64_float_compare_cond special value cond = 13 (Condition LE)
    // Fields: op=0, nzcv=0, Rn=0, type1=0, Rm=0, cond=13
    let encoding: u32 = 0x1E20D400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch64_float_compare_cond_special_cond_14_condition_al_1024_1e20e400() {
    // Encoding: 0x1E20E400
    // Test aarch64_float_compare_cond special value cond = 14 (Condition AL)
    // Fields: cond=14, op=0, type1=0, nzcv=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch64_float_compare_cond_special_cond_15_condition_nv_1024_1e20f400() {
    // Encoding: 0x1E20F400
    // Test aarch64_float_compare_cond special value cond = 15 (Condition NV)
    // Fields: Rn=0, op=0, cond=15, nzcv=0, type1=0, Rm=0
    let encoding: u32 = 0x1E20F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_float_compare_cond_special_rn_31_stack_pointer_sp_may_require_alignment_1024_1e2007e0()
 {
    // Encoding: 0x1E2007E0
    // Test aarch64_float_compare_cond special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: op=0, nzcv=0, type1=0, cond=0, Rm=0, Rn=31
    let encoding: u32 = 0x1E2007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_float_compare_cond_sp_rn_1e2007e0() {
    // Test aarch64_float_compare_cond with Rn = SP (31)
    // Encoding: 0x1E2007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1E2007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_float_compare_cond_flags_zeroresult_0_1e220420() {
    // Test aarch64_float_compare_cond flag computation: ZeroResult
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_float_compare_cond_flags_zeroresult_1_1e220420() {
    // Test aarch64_float_compare_cond flag computation: ZeroResult
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_float_compare_cond_flags_negativeresult_2_1e220420() {
    // Test aarch64_float_compare_cond flag computation: NegativeResult
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_float_compare_cond_flags_unsignedoverflow_3_1e220420() {
    // Test aarch64_float_compare_cond flag computation: UnsignedOverflow
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_float_compare_cond_flags_unsignedoverflow_4_1e220420() {
    // Test aarch64_float_compare_cond flag computation: UnsignedOverflow
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_float_compare_cond_flags_signedoverflow_5_1e220420() {
    // Test aarch64_float_compare_cond flag computation: SignedOverflow
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_float_compare_cond_flags_signedoverflow_6_1e220420() {
    // Test aarch64_float_compare_cond flag computation: SignedOverflow
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_compare_cond
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_float_compare_cond_flags_positiveresult_7_1e220420() {
    // Test aarch64_float_compare_cond flag computation: PositiveResult
    // Encoding: 0x1E220420
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x1E220420;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}
