//! A64 system hints tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_system_hints Tests
// ============================================================================

/// Provenance: aarch64_system_hints
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_hints_field_crm_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_system_hints field CRm = 0 (Min)
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_hints_field_crm_1_poweroftwo_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_system_hints field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_hints_field_crm_7_poweroftwominusone_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_system_hints field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7, op2=0
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_hints_field_crm_15_max_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_system_hints field CRm = 15 (Max)
    // Fields: CRm=15, op2=0
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_hints_field_op2_0_min_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_system_hints field op2 = 0 (Min)
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_hints_field_op2_1_poweroftwo_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_system_hints field op2 = 1 (PowerOfTwo)
    // Fields: CRm=0, op2=1
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_hints_field_op2_7_max_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_system_hints field op2 = 7 (Max)
    // Fields: CRm=0, op2=7
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_system_hints_combo_0_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_system_hints field combination: CRm=0, op2=0
    // Fields: CRm=0, op2=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_system_hints_combo_1_201f_d503211f() {
    // Encoding: 0xD503211F
    // Test aarch64_system_hints field combination: CRm=1, op2=0
    // Fields: CRm=1, op2=0
    let encoding: u32 = 0xD503211F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_system_hints_combo_2_201f_d503271f() {
    // Encoding: 0xD503271F
    // Test aarch64_system_hints field combination: CRm=7, op2=0
    // Fields: CRm=7, op2=0
    let encoding: u32 = 0xD503271F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_system_hints_combo_3_201f_d5032f1f() {
    // Encoding: 0xD5032F1F
    // Test aarch64_system_hints field combination: CRm=15, op2=0
    // Fields: op2=0, CRm=15
    let encoding: u32 = 0xD5032F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_system_hints_combo_4_201f_d503201f() {
    // Encoding: 0xD503201F
    // Test aarch64_system_hints field combination: CRm=0, op2=0
    // Fields: op2=0, CRm=0
    let encoding: u32 = 0xD503201F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_system_hints_combo_5_201f_d503203f() {
    // Encoding: 0xD503203F
    // Test aarch64_system_hints field combination: CRm=0, op2=1
    // Fields: op2=1, CRm=0
    let encoding: u32 = 0xD503203F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_hints
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_system_hints_combo_6_201f_d50320ff() {
    // Encoding: 0xD50320FF
    // Test aarch64_system_hints field combination: CRm=0, op2=7
    // Fields: op2=7, CRm=0
    let encoding: u32 = 0xD50320FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}
