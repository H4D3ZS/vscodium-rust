//! A64 branch unconditional tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_branch_unconditional_register Tests
// ============================================================================

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Z 24 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_register_field_z_0_min_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field Z = 0 (Min)
    // Fields: A=0, op=0, Rn=0, Rm=0, Z=0, M=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Z 24 +: 1`
/// Requirement: FieldBoundary { field: "Z", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_unconditional_register_field_z_1_max_0_d71f0000() {
    // Encoding: 0xD71F0000
    // Test aarch64_branch_unconditional_register field Z = 1 (Max)
    // Fields: Rm=0, Z=1, M=0, Rn=0, op=0, A=0
    let encoding: u32 = 0xD71F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field op 21 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_register_field_op_0_min_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field op = 0 (Min)
    // Fields: M=0, Rm=0, op=0, A=0, Z=0, Rn=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field op 21 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_branch_unconditional_register_field_op_1_poweroftwo_0_d63f0000() {
    // Encoding: 0xD63F0000
    // Test aarch64_branch_unconditional_register field op = 1 (PowerOfTwo)
    // Fields: Rn=0, M=0, op=1, Rm=0, Z=0, A=0
    let encoding: u32 = 0xD63F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field op 21 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_branch_unconditional_register_field_op_3_max_0_d67f0000() {
    // Encoding: 0xD67F0000
    // Test aarch64_branch_unconditional_register field op = 3 (Max)
    // Fields: A=0, M=0, Rn=0, Rm=0, op=3, Z=0
    let encoding: u32 = 0xD67F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field A 11 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_register_field_a_0_min_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field A = 0 (Min)
    // Fields: Rn=0, Z=0, Rm=0, M=0, op=0, A=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field A 11 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_unconditional_register_field_a_1_max_0_d61f0800() {
    // Encoding: 0xD61F0800
    // Test aarch64_branch_unconditional_register field A = 1 (Max)
    // Fields: Rm=0, Rn=0, Z=0, A=1, op=0, M=0
    let encoding: u32 = 0xD61F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field M 10 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_register_field_m_0_min_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field M = 0 (Min)
    // Fields: M=0, Rn=0, A=0, Rm=0, Z=0, op=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field M 10 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_unconditional_register_field_m_1_max_0_d61f0400() {
    // Encoding: 0xD61F0400
    // Test aarch64_branch_unconditional_register field M = 1 (Max)
    // Fields: M=1, A=0, Rn=0, Rm=0, op=0, Z=0
    let encoding: u32 = 0xD61F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_branch_unconditional_register_field_rn_0_min_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field Rn = 0 (Min)
    // Fields: A=0, op=0, Z=0, M=0, Rn=0, Rm=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_branch_unconditional_register_field_rn_1_poweroftwo_0_d61f0020() {
    // Encoding: 0xD61F0020
    // Test aarch64_branch_unconditional_register field Rn = 1 (PowerOfTwo)
    // Fields: Rm=0, Z=0, op=0, Rn=1, A=0, M=0
    let encoding: u32 = 0xD61F0020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_branch_unconditional_register_field_rn_30_poweroftwominusone_0_d61f03c0() {
    // Encoding: 0xD61F03C0
    // Test aarch64_branch_unconditional_register field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Z=0, M=0, Rm=0, Rn=30, op=0, A=0
    let encoding: u32 = 0xD61F03C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_branch_unconditional_register_field_rn_31_max_0_d61f03e0() {
    // Encoding: 0xD61F03E0
    // Test aarch64_branch_unconditional_register field Rn = 31 (Max)
    // Fields: op=0, M=0, Rm=0, A=0, Z=0, Rn=31
    let encoding: u32 = 0xD61F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rm 0 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_branch_unconditional_register_field_rm_0_min_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field Rm = 0 (Min)
    // Fields: Z=0, op=0, A=0, Rm=0, M=0, Rn=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rm 0 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_branch_unconditional_register_field_rm_1_poweroftwo_0_d61f0001() {
    // Encoding: 0xD61F0001
    // Test aarch64_branch_unconditional_register field Rm = 1 (PowerOfTwo)
    // Fields: M=0, Rn=0, op=0, Rm=1, A=0, Z=0
    let encoding: u32 = 0xD61F0001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rm 0 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_branch_unconditional_register_field_rm_30_poweroftwominusone_0_d61f001e() {
    // Encoding: 0xD61F001E
    // Test aarch64_branch_unconditional_register field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: M=0, Rm=30, A=0, Z=0, op=0, Rn=0
    let encoding: u32 = 0xD61F001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rm 0 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_branch_unconditional_register_field_rm_31_max_0_d61f001f() {
    // Encoding: 0xD61F001F
    // Test aarch64_branch_unconditional_register field Rm = 31 (Max)
    // Fields: op=0, Rm=31, A=0, M=0, Rn=0, Z=0
    let encoding: u32 = 0xD61F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_register_combo_0_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: A=0, M=0, Z=0, op=0, Rn=0, Rm=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Z=1 (maximum value (1))
#[test]
fn test_aarch64_branch_unconditional_register_combo_1_0_d71f0000() {
    // Encoding: 0xD71F0000
    // Test aarch64_branch_unconditional_register field combination: Z=1, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: Z=1, op=0, Rm=0, Rn=0, M=0, A=0
    let encoding: u32 = 0xD71F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_register_combo_2_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: Z=0, Rn=0, Rm=0, A=0, M=0, op=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (value 1)
#[test]
fn test_aarch64_branch_unconditional_register_combo_3_0_d63f0000() {
    // Encoding: 0xD63F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=1, A=0, M=0, Rn=0, Rm=0
    // Fields: Rm=0, M=0, Z=0, op=1, A=0, Rn=0
    let encoding: u32 = 0xD63F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=3 (maximum value (3))
#[test]
fn test_aarch64_branch_unconditional_register_combo_4_0_d67f0000() {
    // Encoding: 0xD67F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=3, A=0, M=0, Rn=0, Rm=0
    // Fields: op=3, Z=0, Rn=0, Rm=0, A=0, M=0
    let encoding: u32 = 0xD67F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_register_combo_5_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: Rn=0, Z=0, M=0, Rm=0, A=0, op=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=1 (maximum value (1))
#[test]
fn test_aarch64_branch_unconditional_register_combo_6_0_d61f0800() {
    // Encoding: 0xD61F0800
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=1, M=0, Rn=0, Rm=0
    // Fields: A=1, Rn=0, op=0, Rm=0, Z=0, M=0
    let encoding: u32 = 0xD61F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_register_combo_7_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: Rm=0, A=0, Z=0, M=0, op=0, Rn=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=1 (maximum value (1))
#[test]
fn test_aarch64_branch_unconditional_register_combo_8_0_d61f0400() {
    // Encoding: 0xD61F0400
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=1, Rn=0, Rm=0
    // Fields: Z=0, M=1, A=0, Rn=0, Rm=0, op=0
    let encoding: u32 = 0xD61F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_branch_unconditional_register_combo_9_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: M=0, Rm=0, Rn=0, Z=0, A=0, op=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_branch_unconditional_register_combo_10_0_d61f0020() {
    // Encoding: 0xD61F0020
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=1, Rm=0
    // Fields: Z=0, op=0, Rn=1, M=0, Rm=0, A=0
    let encoding: u32 = 0xD61F0020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_branch_unconditional_register_combo_11_0_d61f03c0() {
    // Encoding: 0xD61F03C0
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=30, Rm=0
    // Fields: Rm=0, A=0, M=0, Rn=30, Z=0, op=0
    let encoding: u32 = 0xD61F03C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_branch_unconditional_register_combo_12_0_d61f03e0() {
    // Encoding: 0xD61F03E0
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=31, Rm=0
    // Fields: Rn=31, M=0, Rm=0, Z=0, op=0, A=0
    let encoding: u32 = 0xD61F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_branch_unconditional_register_combo_13_0_d61f0000() {
    // Encoding: 0xD61F0000
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=0
    // Fields: Z=0, Rm=0, Rn=0, M=0, A=0, op=0
    let encoding: u32 = 0xD61F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_branch_unconditional_register_combo_14_0_d61f0001() {
    // Encoding: 0xD61F0001
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=1
    // Fields: op=0, Z=0, Rm=1, Rn=0, A=0, M=0
    let encoding: u32 = 0xD61F0001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_branch_unconditional_register_combo_15_0_d61f001e() {
    // Encoding: 0xD61F001E
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=30
    // Fields: Z=0, Rn=0, A=0, Rm=30, M=0, op=0
    let encoding: u32 = 0xD61F001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_branch_unconditional_register_combo_16_0_d61f001f() {
    // Encoding: 0xD61F001F
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=0, Rm=31
    // Fields: Z=0, M=0, A=0, op=0, Rm=31, Rn=0
    let encoding: u32 = 0xD61F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rm=1 (same register test (reg=1))
#[test]
fn test_aarch64_branch_unconditional_register_combo_17_0_d61f0021() {
    // Encoding: 0xD61F0021
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=1, Rm=1
    // Fields: Rn=1, op=0, A=0, Z=0, Rm=1, M=0
    let encoding: u32 = 0xD61F0021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rm=31 (same register test (reg=31))
#[test]
fn test_aarch64_branch_unconditional_register_combo_18_0_d61f03ff() {
    // Encoding: 0xD61F03FF
    // Test aarch64_branch_unconditional_register field combination: Z=0, op=0, A=0, M=0, Rn=31, Rm=31
    // Fields: Z=0, Rn=31, Rm=31, M=0, op=0, A=0
    let encoding: u32 = 0xD61F03FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_branch_unconditional_register_special_rn_31_stack_pointer_sp_may_require_alignment_0_d61f03e0()
 {
    // Encoding: 0xD61F03E0
    // Test aarch64_branch_unconditional_register special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Z=0, op=0, M=0, Rn=31, Rm=0, A=0
    let encoding: u32 = 0xD61F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero value, bit 0 (branch=true)
#[test]
fn test_aarch64_branch_unconditional_register_oracle_0_36000000() {
    // Test TBZ: zero value, bit 0 (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 0 set (branch=false)
#[test]
fn test_aarch64_branch_unconditional_register_oracle_1_36000000() {
    // Test TBZ: bit 0 set (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 63 set (branch=false)
#[test]
fn test_aarch64_branch_unconditional_register_oracle_2_b6f80000() {
    // Test TBZ: bit 63 set (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x8000000000000000);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero, testing bit 63 (branch=true)
#[test]
fn test_aarch64_branch_unconditional_register_oracle_3_b6f80000() {
    // Test TBZ: zero, testing bit 63 (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `GpFromField("30") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "30" }
/// verify register write to GpFromField("30")
#[test]
fn test_aarch64_branch_unconditional_register_reg_write_0_d61f0000() {
    // Test aarch64_branch_unconditional_register register write: GpFromField("30")
    // Encoding: 0xD61F0000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD61F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_register
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_branch_unconditional_register_sp_rn_d61f03e0() {
    // Test aarch64_branch_unconditional_register with Rn = SP (31)
    // Encoding: 0xD61F03E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD61F03E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_branch_unconditional_immediate Tests
// ============================================================================

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field op 31 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_immediate_field_op_0_min_0_14000000() {
    // Encoding: 0x14000000
    // Test aarch64_branch_unconditional_immediate field op = 0 (Min)
    // Fields: op=0, imm26=0
    let encoding: u32 = 0x14000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field op 31 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_op_1_max_0_94000000() {
    // Encoding: 0x94000000
    // Test aarch64_branch_unconditional_immediate field op = 1 (Max)
    // Fields: op=1, imm26=0
    let encoding: u32 = 0x94000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_0_zero_0_14000000() {
    // Encoding: 0x14000000
    // Test aarch64_branch_unconditional_immediate field imm26 = 0 (Zero)
    // Fields: imm26=0, op=0
    let encoding: u32 = 0x14000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_1_poweroftwo_0_14000001() {
    // Encoding: 0x14000001
    // Test aarch64_branch_unconditional_immediate field imm26 = 1 (PowerOfTwo)
    // Fields: imm26=1, op=0
    let encoding: u32 = 0x14000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_3_poweroftwominusone_0_14000003() {
    // Encoding: 0x14000003
    // Test aarch64_branch_unconditional_immediate field imm26 = 3 (PowerOfTwoMinusOne)
    // Fields: imm26=3, op=0
    let encoding: u32 = 0x14000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_4_poweroftwo_0_14000004() {
    // Encoding: 0x14000004
    // Test aarch64_branch_unconditional_immediate field imm26 = 4 (PowerOfTwo)
    // Fields: op=0, imm26=4
    let encoding: u32 = 0x14000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_7_poweroftwominusone_0_14000007() {
    // Encoding: 0x14000007
    // Test aarch64_branch_unconditional_immediate field imm26 = 7 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=7
    let encoding: u32 = 0x14000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_8_poweroftwo_0_14000008() {
    // Encoding: 0x14000008
    // Test aarch64_branch_unconditional_immediate field imm26 = 8 (PowerOfTwo)
    // Fields: op=0, imm26=8
    let encoding: u32 = 0x14000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_15_poweroftwominusone_0_1400000f() {
    // Encoding: 0x1400000F
    // Test aarch64_branch_unconditional_immediate field imm26 = 15 (PowerOfTwoMinusOne)
    // Fields: imm26=15, op=0
    let encoding: u32 = 0x1400000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_16_poweroftwo_0_14000010() {
    // Encoding: 0x14000010
    // Test aarch64_branch_unconditional_immediate field imm26 = 16 (PowerOfTwo)
    // Fields: imm26=16, op=0
    let encoding: u32 = 0x14000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_31_poweroftwominusone_0_1400001f() {
    // Encoding: 0x1400001F
    // Test aarch64_branch_unconditional_immediate field imm26 = 31 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=31
    let encoding: u32 = 0x1400001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_32_poweroftwo_0_14000020() {
    // Encoding: 0x14000020
    // Test aarch64_branch_unconditional_immediate field imm26 = 32 (PowerOfTwo)
    // Fields: imm26=32, op=0
    let encoding: u32 = 0x14000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_63_poweroftwominusone_0_1400003f() {
    // Encoding: 0x1400003F
    // Test aarch64_branch_unconditional_immediate field imm26 = 63 (PowerOfTwoMinusOne)
    // Fields: imm26=63, op=0
    let encoding: u32 = 0x1400003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_64_poweroftwo_0_14000040() {
    // Encoding: 0x14000040
    // Test aarch64_branch_unconditional_immediate field imm26 = 64 (PowerOfTwo)
    // Fields: op=0, imm26=64
    let encoding: u32 = 0x14000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_127_poweroftwominusone_0_1400007f() {
    // Encoding: 0x1400007F
    // Test aarch64_branch_unconditional_immediate field imm26 = 127 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=127
    let encoding: u32 = 0x1400007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_128_poweroftwo_0_14000080() {
    // Encoding: 0x14000080
    // Test aarch64_branch_unconditional_immediate field imm26 = 128 (PowerOfTwo)
    // Fields: op=0, imm26=128
    let encoding: u32 = 0x14000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_255_poweroftwominusone_0_140000ff() {
    // Encoding: 0x140000FF
    // Test aarch64_branch_unconditional_immediate field imm26 = 255 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=255
    let encoding: u32 = 0x140000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_256_poweroftwo_0_14000100() {
    // Encoding: 0x14000100
    // Test aarch64_branch_unconditional_immediate field imm26 = 256 (PowerOfTwo)
    // Fields: imm26=256, op=0
    let encoding: u32 = 0x14000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_511_poweroftwominusone_0_140001ff() {
    // Encoding: 0x140001FF
    // Test aarch64_branch_unconditional_immediate field imm26 = 511 (PowerOfTwoMinusOne)
    // Fields: imm26=511, op=0
    let encoding: u32 = 0x140001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_512_poweroftwo_0_14000200() {
    // Encoding: 0x14000200
    // Test aarch64_branch_unconditional_immediate field imm26 = 512 (PowerOfTwo)
    // Fields: imm26=512, op=0
    let encoding: u32 = 0x14000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_1023_poweroftwominusone_0_140003ff() {
    // Encoding: 0x140003FF
    // Test aarch64_branch_unconditional_immediate field imm26 = 1023 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=1023
    let encoding: u32 = 0x140003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_1024_poweroftwo_0_14000400() {
    // Encoding: 0x14000400
    // Test aarch64_branch_unconditional_immediate field imm26 = 1024 (PowerOfTwo)
    // Fields: imm26=1024, op=0
    let encoding: u32 = 0x14000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_2047_poweroftwominusone_0_140007ff() {
    // Encoding: 0x140007FF
    // Test aarch64_branch_unconditional_immediate field imm26 = 2047 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=2047
    let encoding: u32 = 0x140007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_2048_poweroftwo_0_14000800() {
    // Encoding: 0x14000800
    // Test aarch64_branch_unconditional_immediate field imm26 = 2048 (PowerOfTwo)
    // Fields: imm26=2048, op=0
    let encoding: u32 = 0x14000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_4095_poweroftwominusone_0_14000fff() {
    // Encoding: 0x14000FFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 4095 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=4095
    let encoding: u32 = 0x14000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_4096_poweroftwo_0_14001000() {
    // Encoding: 0x14001000
    // Test aarch64_branch_unconditional_immediate field imm26 = 4096 (PowerOfTwo)
    // Fields: op=0, imm26=4096
    let encoding: u32 = 0x14001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_8191_poweroftwominusone_0_14001fff() {
    // Encoding: 0x14001FFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 8191 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=8191
    let encoding: u32 = 0x14001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_8192_poweroftwo_0_14002000() {
    // Encoding: 0x14002000
    // Test aarch64_branch_unconditional_immediate field imm26 = 8192 (PowerOfTwo)
    // Fields: op=0, imm26=8192
    let encoding: u32 = 0x14002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_16383_poweroftwominusone_0_14003fff() {
    // Encoding: 0x14003FFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 16383 (PowerOfTwoMinusOne)
    // Fields: imm26=16383, op=0
    let encoding: u32 = 0x14003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_16384_poweroftwo_0_14004000() {
    // Encoding: 0x14004000
    // Test aarch64_branch_unconditional_immediate field imm26 = 16384 (PowerOfTwo)
    // Fields: imm26=16384, op=0
    let encoding: u32 = 0x14004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_32767_poweroftwominusone_0_14007fff() {
    // Encoding: 0x14007FFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 32767 (PowerOfTwoMinusOne)
    // Fields: imm26=32767, op=0
    let encoding: u32 = 0x14007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_32768_poweroftwo_0_14008000() {
    // Encoding: 0x14008000
    // Test aarch64_branch_unconditional_immediate field imm26 = 32768 (PowerOfTwo)
    // Fields: op=0, imm26=32768
    let encoding: u32 = 0x14008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_65535_poweroftwominusone_0_1400ffff() {
    // Encoding: 0x1400FFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 65535 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=65535
    let encoding: u32 = 0x1400FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_65536_poweroftwo_0_14010000() {
    // Encoding: 0x14010000
    // Test aarch64_branch_unconditional_immediate field imm26 = 65536 (PowerOfTwo)
    // Fields: imm26=65536, op=0
    let encoding: u32 = 0x14010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_131071_poweroftwominusone_0_1401ffff() {
    // Encoding: 0x1401FFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 131071 (PowerOfTwoMinusOne)
    // Fields: imm26=131071, op=0
    let encoding: u32 = 0x1401FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_131072_poweroftwo_0_14020000() {
    // Encoding: 0x14020000
    // Test aarch64_branch_unconditional_immediate field imm26 = 131072 (PowerOfTwo)
    // Fields: imm26=131072, op=0
    let encoding: u32 = 0x14020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 262143, boundary: PowerOfTwoMinusOne }
/// 2^18 - 1 = 262143
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_262143_poweroftwominusone_0_1403ffff() {
    // Encoding: 0x1403FFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 262143 (PowerOfTwoMinusOne)
    // Fields: imm26=262143, op=0
    let encoding: u32 = 0x1403FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_262144_poweroftwo_0_14040000() {
    // Encoding: 0x14040000
    // Test aarch64_branch_unconditional_immediate field imm26 = 262144 (PowerOfTwo)
    // Fields: op=0, imm26=262144
    let encoding: u32 = 0x14040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 524287, boundary: PowerOfTwoMinusOne }
/// 2^19 - 1 = 524287
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_524287_poweroftwominusone_0_1407ffff() {
    // Encoding: 0x1407FFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 524287 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=524287
    let encoding: u32 = 0x1407FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 524288, boundary: PowerOfTwo }
/// power of 2 (2^19 = 524288)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_524288_poweroftwo_0_14080000() {
    // Encoding: 0x14080000
    // Test aarch64_branch_unconditional_immediate field imm26 = 524288 (PowerOfTwo)
    // Fields: op=0, imm26=524288
    let encoding: u32 = 0x14080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 1048575, boundary: PowerOfTwoMinusOne }
/// 2^20 - 1 = 1048575
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_1048575_poweroftwominusone_0_140fffff() {
    // Encoding: 0x140FFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 1048575 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=1048575
    let encoding: u32 = 0x140FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 1048576, boundary: PowerOfTwo }
/// power of 2 (2^20 = 1048576)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_1048576_poweroftwo_0_14100000() {
    // Encoding: 0x14100000
    // Test aarch64_branch_unconditional_immediate field imm26 = 1048576 (PowerOfTwo)
    // Fields: op=0, imm26=1048576
    let encoding: u32 = 0x14100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 2097151, boundary: PowerOfTwoMinusOne }
/// 2^21 - 1 = 2097151
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_2097151_poweroftwominusone_0_141fffff() {
    // Encoding: 0x141FFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 2097151 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=2097151
    let encoding: u32 = 0x141FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 2097152, boundary: PowerOfTwo }
/// power of 2 (2^21 = 2097152)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_2097152_poweroftwo_0_14200000() {
    // Encoding: 0x14200000
    // Test aarch64_branch_unconditional_immediate field imm26 = 2097152 (PowerOfTwo)
    // Fields: imm26=2097152, op=0
    let encoding: u32 = 0x14200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 4194303, boundary: PowerOfTwoMinusOne }
/// 2^22 - 1 = 4194303
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_4194303_poweroftwominusone_0_143fffff() {
    // Encoding: 0x143FFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 4194303 (PowerOfTwoMinusOne)
    // Fields: imm26=4194303, op=0
    let encoding: u32 = 0x143FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 4194304, boundary: PowerOfTwo }
/// power of 2 (2^22 = 4194304)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_4194304_poweroftwo_0_14400000() {
    // Encoding: 0x14400000
    // Test aarch64_branch_unconditional_immediate field imm26 = 4194304 (PowerOfTwo)
    // Fields: imm26=4194304, op=0
    let encoding: u32 = 0x14400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 8388607, boundary: PowerOfTwoMinusOne }
/// 2^23 - 1 = 8388607
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_8388607_poweroftwominusone_0_147fffff() {
    // Encoding: 0x147FFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 8388607 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=8388607
    let encoding: u32 = 0x147FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 8388608, boundary: PowerOfTwo }
/// power of 2 (2^23 = 8388608)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_8388608_poweroftwo_0_14800000() {
    // Encoding: 0x14800000
    // Test aarch64_branch_unconditional_immediate field imm26 = 8388608 (PowerOfTwo)
    // Fields: imm26=8388608, op=0
    let encoding: u32 = 0x14800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 16777215, boundary: PowerOfTwoMinusOne }
/// 2^24 - 1 = 16777215
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_16777215_poweroftwominusone_0_14ffffff()
{
    // Encoding: 0x14FFFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 16777215 (PowerOfTwoMinusOne)
    // Fields: op=0, imm26=16777215
    let encoding: u32 = 0x14FFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 16777216, boundary: PowerOfTwo }
/// power of 2 (2^24 = 16777216)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_16777216_poweroftwo_0_15000000() {
    // Encoding: 0x15000000
    // Test aarch64_branch_unconditional_immediate field imm26 = 16777216 (PowerOfTwo)
    // Fields: op=0, imm26=16777216
    let encoding: u32 = 0x15000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 33554431, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (33554431)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_33554431_poweroftwominusone_0_15ffffff()
{
    // Encoding: 0x15FFFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 33554431 (PowerOfTwoMinusOne)
    // Fields: imm26=33554431, op=0
    let encoding: u32 = 0x15FFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 33554432, boundary: PowerOfTwo }
/// power of 2 (2^25 = 33554432)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_33554432_poweroftwo_0_16000000() {
    // Encoding: 0x16000000
    // Test aarch64_branch_unconditional_immediate field imm26 = 33554432 (PowerOfTwo)
    // Fields: imm26=33554432, op=0
    let encoding: u32 = 0x16000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field imm26 0 +: 26`
/// Requirement: FieldBoundary { field: "imm26", value: 67108863, boundary: Max }
/// maximum immediate (67108863)
#[test]
fn test_aarch64_branch_unconditional_immediate_field_imm26_67108863_max_0_17ffffff() {
    // Encoding: 0x17FFFFFF
    // Test aarch64_branch_unconditional_immediate field imm26 = 67108863 (Max)
    // Fields: imm26=67108863, op=0
    let encoding: u32 = 0x17FFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_0_0_14000000() {
    // Encoding: 0x14000000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=0
    // Fields: op=0, imm26=0
    let encoding: u32 = 0x14000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_1_0_94000000() {
    // Encoding: 0x94000000
    // Test aarch64_branch_unconditional_immediate field combination: op=1, imm26=0
    // Fields: op=1, imm26=0
    let encoding: u32 = 0x94000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=0 (immediate value 0)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_2_0_14000000() {
    // Encoding: 0x14000000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=0
    // Fields: op=0, imm26=0
    let encoding: u32 = 0x14000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=1 (immediate value 1)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_3_0_14000001() {
    // Encoding: 0x14000001
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=1
    // Fields: imm26=1, op=0
    let encoding: u32 = 0x14000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_4_0_14000003() {
    // Encoding: 0x14000003
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=3
    // Fields: op=0, imm26=3
    let encoding: u32 = 0x14000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_5_0_14000004() {
    // Encoding: 0x14000004
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=4
    // Fields: imm26=4, op=0
    let encoding: u32 = 0x14000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_6_0_14000007() {
    // Encoding: 0x14000007
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=7
    // Fields: op=0, imm26=7
    let encoding: u32 = 0x14000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_7_0_14000008() {
    // Encoding: 0x14000008
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=8
    // Fields: imm26=8, op=0
    let encoding: u32 = 0x14000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_8_0_1400000f() {
    // Encoding: 0x1400000F
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=15
    // Fields: imm26=15, op=0
    let encoding: u32 = 0x1400000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_9_0_14000010() {
    // Encoding: 0x14000010
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=16
    // Fields: imm26=16, op=0
    let encoding: u32 = 0x14000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_10_0_1400001f() {
    // Encoding: 0x1400001F
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=31
    // Fields: op=0, imm26=31
    let encoding: u32 = 0x1400001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_11_0_14000020() {
    // Encoding: 0x14000020
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=32
    // Fields: imm26=32, op=0
    let encoding: u32 = 0x14000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_12_0_1400003f() {
    // Encoding: 0x1400003F
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=63
    // Fields: op=0, imm26=63
    let encoding: u32 = 0x1400003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_13_0_14000040() {
    // Encoding: 0x14000040
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=64
    // Fields: imm26=64, op=0
    let encoding: u32 = 0x14000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_14_0_1400007f() {
    // Encoding: 0x1400007F
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=127
    // Fields: imm26=127, op=0
    let encoding: u32 = 0x1400007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_15_0_14000080() {
    // Encoding: 0x14000080
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=128
    // Fields: imm26=128, op=0
    let encoding: u32 = 0x14000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_16_0_140000ff() {
    // Encoding: 0x140000FF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=255
    // Fields: imm26=255, op=0
    let encoding: u32 = 0x140000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_17_0_14000100() {
    // Encoding: 0x14000100
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=256
    // Fields: imm26=256, op=0
    let encoding: u32 = 0x14000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_18_0_140001ff() {
    // Encoding: 0x140001FF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=511
    // Fields: imm26=511, op=0
    let encoding: u32 = 0x140001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_19_0_14000200() {
    // Encoding: 0x14000200
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=512
    // Fields: imm26=512, op=0
    let encoding: u32 = 0x14000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_20_0_140003ff() {
    // Encoding: 0x140003FF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=1023
    // Fields: op=0, imm26=1023
    let encoding: u32 = 0x140003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_21_0_14000400() {
    // Encoding: 0x14000400
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=1024
    // Fields: imm26=1024, op=0
    let encoding: u32 = 0x14000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_22_0_140007ff() {
    // Encoding: 0x140007FF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=2047
    // Fields: op=0, imm26=2047
    let encoding: u32 = 0x140007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_23_0_14000800() {
    // Encoding: 0x14000800
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=2048
    // Fields: imm26=2048, op=0
    let encoding: u32 = 0x14000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_24_0_14000fff() {
    // Encoding: 0x14000FFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=4095
    // Fields: imm26=4095, op=0
    let encoding: u32 = 0x14000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_25_0_14001000() {
    // Encoding: 0x14001000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=4096
    // Fields: op=0, imm26=4096
    let encoding: u32 = 0x14001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_26_0_14001fff() {
    // Encoding: 0x14001FFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=8191
    // Fields: imm26=8191, op=0
    let encoding: u32 = 0x14001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_27_0_14002000() {
    // Encoding: 0x14002000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=8192
    // Fields: op=0, imm26=8192
    let encoding: u32 = 0x14002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_28_0_14003fff() {
    // Encoding: 0x14003FFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=16383
    // Fields: imm26=16383, op=0
    let encoding: u32 = 0x14003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_29_0_14004000() {
    // Encoding: 0x14004000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=16384
    // Fields: imm26=16384, op=0
    let encoding: u32 = 0x14004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=32767 (2^15 - 1 = 32767)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_30_0_14007fff() {
    // Encoding: 0x14007FFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=32767
    // Fields: op=0, imm26=32767
    let encoding: u32 = 0x14007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_31_0_14008000() {
    // Encoding: 0x14008000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=32768
    // Fields: imm26=32768, op=0
    let encoding: u32 = 0x14008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=65535 (2^16 - 1 = 65535)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_32_0_1400ffff() {
    // Encoding: 0x1400FFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=65535
    // Fields: op=0, imm26=65535
    let encoding: u32 = 0x1400FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=65536 (power of 2 (2^16 = 65536))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_33_0_14010000() {
    // Encoding: 0x14010000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=65536
    // Fields: imm26=65536, op=0
    let encoding: u32 = 0x14010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=131071 (2^17 - 1 = 131071)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_34_0_1401ffff() {
    // Encoding: 0x1401FFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=131071
    // Fields: op=0, imm26=131071
    let encoding: u32 = 0x1401FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=131072 (power of 2 (2^17 = 131072))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_35_0_14020000() {
    // Encoding: 0x14020000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=131072
    // Fields: op=0, imm26=131072
    let encoding: u32 = 0x14020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=262143 (2^18 - 1 = 262143)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_36_0_1403ffff() {
    // Encoding: 0x1403FFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=262143
    // Fields: op=0, imm26=262143
    let encoding: u32 = 0x1403FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=262144 (power of 2 (2^18 = 262144))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_37_0_14040000() {
    // Encoding: 0x14040000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=262144
    // Fields: op=0, imm26=262144
    let encoding: u32 = 0x14040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=524287 (2^19 - 1 = 524287)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_38_0_1407ffff() {
    // Encoding: 0x1407FFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=524287
    // Fields: op=0, imm26=524287
    let encoding: u32 = 0x1407FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=524288 (power of 2 (2^19 = 524288))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_39_0_14080000() {
    // Encoding: 0x14080000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=524288
    // Fields: imm26=524288, op=0
    let encoding: u32 = 0x14080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=1048575 (2^20 - 1 = 1048575)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_40_0_140fffff() {
    // Encoding: 0x140FFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=1048575
    // Fields: op=0, imm26=1048575
    let encoding: u32 = 0x140FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=1048576 (power of 2 (2^20 = 1048576))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_41_0_14100000() {
    // Encoding: 0x14100000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=1048576
    // Fields: op=0, imm26=1048576
    let encoding: u32 = 0x14100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=2097151 (2^21 - 1 = 2097151)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_42_0_141fffff() {
    // Encoding: 0x141FFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=2097151
    // Fields: imm26=2097151, op=0
    let encoding: u32 = 0x141FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=2097152 (power of 2 (2^21 = 2097152))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_43_0_14200000() {
    // Encoding: 0x14200000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=2097152
    // Fields: op=0, imm26=2097152
    let encoding: u32 = 0x14200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 44`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=4194303 (2^22 - 1 = 4194303)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_44_0_143fffff() {
    // Encoding: 0x143FFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=4194303
    // Fields: op=0, imm26=4194303
    let encoding: u32 = 0x143FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 45`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=4194304 (power of 2 (2^22 = 4194304))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_45_0_14400000() {
    // Encoding: 0x14400000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=4194304
    // Fields: op=0, imm26=4194304
    let encoding: u32 = 0x14400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 46`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=8388607 (2^23 - 1 = 8388607)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_46_0_147fffff() {
    // Encoding: 0x147FFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=8388607
    // Fields: op=0, imm26=8388607
    let encoding: u32 = 0x147FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 47`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=8388608 (power of 2 (2^23 = 8388608))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_47_0_14800000() {
    // Encoding: 0x14800000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=8388608
    // Fields: op=0, imm26=8388608
    let encoding: u32 = 0x14800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 48`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=16777215 (2^24 - 1 = 16777215)
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_48_0_14ffffff() {
    // Encoding: 0x14FFFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=16777215
    // Fields: op=0, imm26=16777215
    let encoding: u32 = 0x14FFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 49`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=16777216 (power of 2 (2^24 = 16777216))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_49_0_15000000() {
    // Encoding: 0x15000000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=16777216
    // Fields: imm26=16777216, op=0
    let encoding: u32 = 0x15000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 50`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=33554431 (immediate midpoint (33554431))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_50_0_15ffffff() {
    // Encoding: 0x15FFFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=33554431
    // Fields: imm26=33554431, op=0
    let encoding: u32 = 0x15FFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 51`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=33554432 (power of 2 (2^25 = 33554432))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_51_0_16000000() {
    // Encoding: 0x16000000
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=33554432
    // Fields: imm26=33554432, op=0
    let encoding: u32 = 0x16000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `field combination 52`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm26=67108863 (maximum immediate (67108863))
#[test]
fn test_aarch64_branch_unconditional_immediate_combo_52_0_17ffffff() {
    // Encoding: 0x17FFFFFF
    // Test aarch64_branch_unconditional_immediate field combination: op=0, imm26=67108863
    // Fields: op=0, imm26=67108863
    let encoding: u32 = 0x17FFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_immediate
/// ASL: `GpFromField("30") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "30" }
/// verify register write to GpFromField("30")
#[test]
fn test_aarch64_branch_unconditional_immediate_reg_write_0_14000000() {
    // Test aarch64_branch_unconditional_immediate register write: GpFromField("30")
    // Encoding: 0x14000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x14000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_branch_unconditional_eret Tests
// ============================================================================

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field A 11 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_eret_field_a_0_min_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field A = 0 (Min)
    // Fields: op4=0, Rn=0, A=0, M=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field A 11 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_unconditional_eret_field_a_1_max_0_d69f0800() {
    // Encoding: 0xD69F0800
    // Test aarch64_branch_unconditional_eret field A = 1 (Max)
    // Fields: op4=0, M=0, Rn=0, A=1
    let encoding: u32 = 0xD69F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field M 10 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_eret_field_m_0_min_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field M = 0 (Min)
    // Fields: M=0, A=0, op4=0, Rn=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field M 10 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_unconditional_eret_field_m_1_max_0_d69f0400() {
    // Encoding: 0xD69F0400
    // Test aarch64_branch_unconditional_eret field M = 1 (Max)
    // Fields: Rn=0, op4=0, A=0, M=1
    let encoding: u32 = 0xD69F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_branch_unconditional_eret_field_rn_0_min_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field Rn = 0 (Min)
    // Fields: op4=0, Rn=0, A=0, M=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_branch_unconditional_eret_field_rn_1_poweroftwo_0_d69f0020() {
    // Encoding: 0xD69F0020
    // Test aarch64_branch_unconditional_eret field Rn = 1 (PowerOfTwo)
    // Fields: A=0, M=0, op4=0, Rn=1
    let encoding: u32 = 0xD69F0020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_branch_unconditional_eret_field_rn_30_poweroftwominusone_0_d69f03c0() {
    // Encoding: 0xD69F03C0
    // Test aarch64_branch_unconditional_eret field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op4=0, A=0, Rn=30, M=0
    let encoding: u32 = 0xD69F03C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_branch_unconditional_eret_field_rn_31_max_0_d69f03e0() {
    // Encoding: 0xD69F03E0
    // Test aarch64_branch_unconditional_eret field Rn = 31 (Max)
    // Fields: op4=0, M=0, A=0, Rn=31
    let encoding: u32 = 0xD69F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field op4 0 +: 5`
/// Requirement: FieldBoundary { field: "op4", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_unconditional_eret_field_op4_0_min_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field op4 = 0 (Min)
    // Fields: A=0, op4=0, M=0, Rn=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field op4 0 +: 5`
/// Requirement: FieldBoundary { field: "op4", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_branch_unconditional_eret_field_op4_1_poweroftwo_0_d69f0001() {
    // Encoding: 0xD69F0001
    // Test aarch64_branch_unconditional_eret field op4 = 1 (PowerOfTwo)
    // Fields: Rn=0, M=0, A=0, op4=1
    let encoding: u32 = 0xD69F0001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field op4 0 +: 5`
/// Requirement: FieldBoundary { field: "op4", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_aarch64_branch_unconditional_eret_field_op4_15_poweroftwominusone_0_d69f000f() {
    // Encoding: 0xD69F000F
    // Test aarch64_branch_unconditional_eret field op4 = 15 (PowerOfTwoMinusOne)
    // Fields: op4=15, A=0, Rn=0, M=0
    let encoding: u32 = 0xD69F000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field op4 0 +: 5`
/// Requirement: FieldBoundary { field: "op4", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_aarch64_branch_unconditional_eret_field_op4_31_max_0_d69f001f() {
    // Encoding: 0xD69F001F
    // Test aarch64_branch_unconditional_eret field op4 = 31 (Max)
    // Fields: M=0, Rn=0, op4=31, A=0
    let encoding: u32 = 0xD69F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_eret_combo_0_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=0
    // Fields: Rn=0, M=0, A=0, op4=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=1 (maximum value (1))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_1_0_d69f0800() {
    // Encoding: 0xD69F0800
    // Test aarch64_branch_unconditional_eret field combination: A=1, M=0, Rn=0, op4=0
    // Fields: op4=0, M=0, Rn=0, A=1
    let encoding: u32 = 0xD69F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_eret_combo_2_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=0
    // Fields: A=0, op4=0, M=0, Rn=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// M=1 (maximum value (1))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_3_0_d69f0400() {
    // Encoding: 0xD69F0400
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=1, Rn=0, op4=0
    // Fields: A=0, op4=0, M=1, Rn=0
    let encoding: u32 = 0xD69F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_4_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=0
    // Fields: M=0, op4=0, A=0, Rn=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_5_0_d69f0020() {
    // Encoding: 0xD69F0020
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=1, op4=0
    // Fields: op4=0, A=0, Rn=1, M=0
    let encoding: u32 = 0xD69F0020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_6_0_d69f03c0() {
    // Encoding: 0xD69F03C0
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=30, op4=0
    // Fields: Rn=30, op4=0, A=0, M=0
    let encoding: u32 = 0xD69F03C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_7_0_d69f03e0() {
    // Encoding: 0xD69F03E0
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=31, op4=0
    // Fields: Rn=31, M=0, op4=0, A=0
    let encoding: u32 = 0xD69F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op4=0 (minimum value)
#[test]
fn test_aarch64_branch_unconditional_eret_combo_8_0_d69f0000() {
    // Encoding: 0xD69F0000
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=0
    // Fields: Rn=0, op4=0, M=0, A=0
    let encoding: u32 = 0xD69F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op4=1 (value 1)
#[test]
fn test_aarch64_branch_unconditional_eret_combo_9_0_d69f0001() {
    // Encoding: 0xD69F0001
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=1
    // Fields: M=0, Rn=0, A=0, op4=1
    let encoding: u32 = 0xD69F0001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op4=15 (midpoint (15))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_10_0_d69f000f() {
    // Encoding: 0xD69F000F
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=15
    // Fields: A=0, Rn=0, op4=15, M=0
    let encoding: u32 = 0xD69F000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op4=31 (maximum value (31))
#[test]
fn test_aarch64_branch_unconditional_eret_combo_11_0_d69f001f() {
    // Encoding: 0xD69F001F
    // Test aarch64_branch_unconditional_eret field combination: A=0, M=0, Rn=0, op4=31
    // Fields: M=0, Rn=0, op4=31, A=0
    let encoding: u32 = 0xD69F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_branch_unconditional_eret_special_rn_31_stack_pointer_sp_may_require_alignment_0_d69f03e0()
 {
    // Encoding: 0xD69F03E0
    // Test aarch64_branch_unconditional_eret special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: A=0, op4=0, Rn=31, M=0
    let encoding: u32 = 0xD69F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero value, bit 0 (branch=true)
#[test]
fn test_aarch64_branch_unconditional_eret_oracle_0_36000000() {
    // Test TBZ: zero value, bit 0 (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 0 set (branch=false)
#[test]
fn test_aarch64_branch_unconditional_eret_oracle_1_36000000() {
    // Test TBZ: bit 0 set (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 63 set (branch=false)
#[test]
fn test_aarch64_branch_unconditional_eret_oracle_2_b6f80000() {
    // Test TBZ: bit 63 set (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x8000000000000000);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero, testing bit 63 (branch=true)
#[test]
fn test_aarch64_branch_unconditional_eret_oracle_3_b6f80000() {
    // Test TBZ: zero, testing bit 63 (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_eret
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_branch_unconditional_eret_sp_rn_d69f03e0() {
    // Test aarch64_branch_unconditional_eret with Rn = SP (31)
    // Encoding: 0xD69F03E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD69F03E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_branch_unconditional_dret Tests
// ============================================================================

/// Provenance: aarch64_branch_unconditional_dret
/// ASL: `fixed encoding (no variable fields)`
/// Requirement: BasicEncoding
/// instruction with no variable fields
#[test]
fn test_aarch64_branch_unconditional_dret_basic_encoding_d6bf03e0() {
    // Encoding: 0xD6BF03E0
    // Test aarch64_branch_unconditional_dret basic encoding
    let encoding: u32 = 0xD6BF03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_branch_unconditional_dret
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero value, bit 0 (branch=true)
#[test]
fn test_aarch64_branch_unconditional_dret_oracle_0_36000000() {
    // Test TBZ: zero value, bit 0 (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_dret
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 0 set (branch=false)
#[test]
fn test_aarch64_branch_unconditional_dret_oracle_1_36000000() {
    // Test TBZ: bit 0 set (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_dret
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 63 set (branch=false)
#[test]
fn test_aarch64_branch_unconditional_dret_oracle_2_b6f80000() {
    // Test TBZ: bit 63 set (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x8000000000000000);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_unconditional_dret
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero, testing bit 63 (branch=true)
#[test]
fn test_aarch64_branch_unconditional_dret_oracle_3_b6f80000() {
    // Test TBZ: zero, testing bit 63 (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
