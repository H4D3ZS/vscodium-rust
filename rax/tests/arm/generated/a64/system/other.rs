//! A64 system other tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_system_sysops Tests
// ============================================================================

/// Provenance: aarch64_system_sysops
/// ASL: `field L 21 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_sysops_field_l_0_min_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field L = 0 (Min)
    // Fields: CRm=0, op1=0, L=0, CRn=0, op2=0, Rt=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field L 21 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_system_sysops_field_l_1_max_0_d5280000() {
    // Encoding: 0xD5280000
    // Test aarch64_system_sysops field L = 1 (Max)
    // Fields: L=1, Rt=0, op2=0, op1=0, CRn=0, CRm=0
    let encoding: u32 = 0xD5280000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_sysops_field_op1_0_min_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field op1 = 0 (Min)
    // Fields: L=0, CRm=0, Rt=0, op1=0, CRn=0, op2=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_sysops_field_op1_1_poweroftwo_0_d5090000() {
    // Encoding: 0xD5090000
    // Test aarch64_system_sysops field op1 = 1 (PowerOfTwo)
    // Fields: Rt=0, CRn=0, L=0, op1=1, op2=0, CRm=0
    let encoding: u32 = 0xD5090000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_sysops_field_op1_7_max_0_d50f0000() {
    // Encoding: 0xD50F0000
    // Test aarch64_system_sysops field op1 = 7 (Max)
    // Fields: L=0, op1=7, CRn=0, CRm=0, op2=0, Rt=0
    let encoding: u32 = 0xD50F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_sysops_field_crn_0_min_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field CRn = 0 (Min)
    // Fields: L=0, CRn=0, op1=0, CRm=0, op2=0, Rt=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_sysops_field_crn_1_poweroftwo_0_d5081000() {
    // Encoding: 0xD5081000
    // Test aarch64_system_sysops field CRn = 1 (PowerOfTwo)
    // Fields: CRm=0, Rt=0, op2=0, L=0, op1=0, CRn=1
    let encoding: u32 = 0xD5081000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_sysops_field_crn_7_poweroftwominusone_0_d5087000() {
    // Encoding: 0xD5087000
    // Test aarch64_system_sysops field CRn = 7 (PowerOfTwoMinusOne)
    // Fields: L=0, op1=0, op2=0, CRn=7, Rt=0, CRm=0
    let encoding: u32 = 0xD5087000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_sysops_field_crn_15_max_0_d508f000() {
    // Encoding: 0xD508F000
    // Test aarch64_system_sysops field CRn = 15 (Max)
    // Fields: CRm=0, op2=0, Rt=0, L=0, CRn=15, op1=0
    let encoding: u32 = 0xD508F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_sysops_field_crm_0_min_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field CRm = 0 (Min)
    // Fields: op2=0, CRn=0, CRm=0, L=0, Rt=0, op1=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_sysops_field_crm_1_poweroftwo_0_d5080100() {
    // Encoding: 0xD5080100
    // Test aarch64_system_sysops field CRm = 1 (PowerOfTwo)
    // Fields: op1=0, CRn=0, CRm=1, L=0, op2=0, Rt=0
    let encoding: u32 = 0xD5080100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_sysops_field_crm_7_poweroftwominusone_0_d5080700() {
    // Encoding: 0xD5080700
    // Test aarch64_system_sysops field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: op1=0, CRn=0, L=0, Rt=0, CRm=7, op2=0
    let encoding: u32 = 0xD5080700;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_sysops_field_crm_15_max_0_d5080f00() {
    // Encoding: 0xD5080F00
    // Test aarch64_system_sysops field CRm = 15 (Max)
    // Fields: CRn=0, L=0, op1=0, CRm=15, Rt=0, op2=0
    let encoding: u32 = 0xD5080F00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_sysops_field_op2_0_min_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field op2 = 0 (Min)
    // Fields: op2=0, Rt=0, op1=0, CRn=0, L=0, CRm=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_sysops_field_op2_1_poweroftwo_0_d5080020() {
    // Encoding: 0xD5080020
    // Test aarch64_system_sysops field op2 = 1 (PowerOfTwo)
    // Fields: op2=1, Rt=0, op1=0, CRn=0, CRm=0, L=0
    let encoding: u32 = 0xD5080020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_sysops_field_op2_7_max_0_d50800e0() {
    // Encoding: 0xD50800E0
    // Test aarch64_system_sysops field op2 = 7 (Max)
    // Fields: L=0, op1=0, op2=7, Rt=0, CRm=0, CRn=0
    let encoding: u32 = 0xD50800E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_system_sysops_field_rt_0_min_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field Rt = 0 (Min)
    // Fields: op2=0, L=0, op1=0, CRm=0, Rt=0, CRn=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_system_sysops_field_rt_1_poweroftwo_0_d5080001() {
    // Encoding: 0xD5080001
    // Test aarch64_system_sysops field Rt = 1 (PowerOfTwo)
    // Fields: op1=0, op2=0, L=0, CRm=0, CRn=0, Rt=1
    let encoding: u32 = 0xD5080001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_system_sysops_field_rt_30_poweroftwominusone_0_d508001e() {
    // Encoding: 0xD508001E
    // Test aarch64_system_sysops field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, op2=0, Rt=30, CRn=0, CRm=0, op1=0
    let encoding: u32 = 0xD508001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_system_sysops_field_rt_31_max_0_d508001f() {
    // Encoding: 0xD508001F
    // Test aarch64_system_sysops field Rt = 31 (Max)
    // Fields: L=0, op1=0, op2=0, CRn=0, CRm=0, Rt=31
    let encoding: u32 = 0xD508001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_system_sysops_combo_0_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRn=0, op1=0, L=0, CRm=0, op2=0, Rt=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_system_sysops_combo_1_0_d5280000() {
    // Encoding: 0xD5280000
    // Test aarch64_system_sysops field combination: L=1, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: op2=0, L=1, CRm=0, op1=0, CRn=0, Rt=0
    let encoding: u32 = 0xD5280000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=0 (minimum value)
#[test]
fn test_aarch64_system_sysops_combo_2_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: Rt=0, CRn=0, L=0, op1=0, CRm=0, op2=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=1 (value 1)
#[test]
fn test_aarch64_system_sysops_combo_3_0_d5090000() {
    // Encoding: 0xD5090000
    // Test aarch64_system_sysops field combination: L=0, op1=1, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRn=0, L=0, op2=0, Rt=0, CRm=0, op1=1
    let encoding: u32 = 0xD5090000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=7 (maximum value (7))
#[test]
fn test_aarch64_system_sysops_combo_4_0_d50f0000() {
    // Encoding: 0xD50F0000
    // Test aarch64_system_sysops field combination: L=0, op1=7, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRm=0, op1=7, Rt=0, L=0, CRn=0, op2=0
    let encoding: u32 = 0xD50F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=0 (minimum value)
#[test]
fn test_aarch64_system_sysops_combo_5_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRm=0, op1=0, L=0, op2=0, CRn=0, Rt=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=1 (value 1)
#[test]
fn test_aarch64_system_sysops_combo_6_0_d5081000() {
    // Encoding: 0xD5081000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=1, CRm=0, op2=0, Rt=0
    // Fields: CRm=0, Rt=0, op1=0, op2=0, CRn=1, L=0
    let encoding: u32 = 0xD5081000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=7 (midpoint (7))
#[test]
fn test_aarch64_system_sysops_combo_7_0_d5087000() {
    // Encoding: 0xD5087000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=7, CRm=0, op2=0, Rt=0
    // Fields: op1=0, CRm=0, Rt=0, op2=0, CRn=7, L=0
    let encoding: u32 = 0xD5087000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=15 (maximum value (15))
#[test]
fn test_aarch64_system_sysops_combo_8_0_d508f000() {
    // Encoding: 0xD508F000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=15, CRm=0, op2=0, Rt=0
    // Fields: op2=0, L=0, Rt=0, op1=0, CRm=0, CRn=15
    let encoding: u32 = 0xD508F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_system_sysops_combo_9_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRn=0, op2=0, Rt=0, op1=0, CRm=0, L=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_system_sysops_combo_10_0_d5080100() {
    // Encoding: 0xD5080100
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=1, op2=0, Rt=0
    // Fields: CRm=1, CRn=0, op2=0, Rt=0, L=0, op1=0
    let encoding: u32 = 0xD5080100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_system_sysops_combo_11_0_d5080700() {
    // Encoding: 0xD5080700
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=7, op2=0, Rt=0
    // Fields: op1=0, Rt=0, CRm=7, op2=0, L=0, CRn=0
    let encoding: u32 = 0xD5080700;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_system_sysops_combo_12_0_d5080f00() {
    // Encoding: 0xD5080F00
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=15, op2=0, Rt=0
    // Fields: CRm=15, op2=0, op1=0, CRn=0, Rt=0, L=0
    let encoding: u32 = 0xD5080F00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_system_sysops_combo_13_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRm=0, op1=0, CRn=0, op2=0, Rt=0, L=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_system_sysops_combo_14_0_d5080020() {
    // Encoding: 0xD5080020
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=1, Rt=0
    // Fields: CRn=0, op2=1, CRm=0, op1=0, Rt=0, L=0
    let encoding: u32 = 0xD5080020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_system_sysops_combo_15_0_d50800e0() {
    // Encoding: 0xD50800E0
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=7, Rt=0
    // Fields: CRn=0, CRm=0, op2=7, Rt=0, op1=0, L=0
    let encoding: u32 = 0xD50800E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_system_sysops_combo_16_0_d5080000() {
    // Encoding: 0xD5080000
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRn=0, op1=0, Rt=0, L=0, CRm=0, op2=0
    let encoding: u32 = 0xD5080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_system_sysops_combo_17_0_d5080001() {
    // Encoding: 0xD5080001
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=1
    // Fields: Rt=1, op1=0, CRn=0, L=0, op2=0, CRm=0
    let encoding: u32 = 0xD5080001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_system_sysops_combo_18_0_d508001e() {
    // Encoding: 0xD508001E
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=30
    // Fields: op2=0, Rt=30, CRm=0, L=0, op1=0, CRn=0
    let encoding: u32 = 0xD508001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_system_sysops_combo_19_0_d508001f() {
    // Encoding: 0xD508001F
    // Test aarch64_system_sysops field combination: L=0, op1=0, CRn=0, CRm=0, op2=0, Rt=31
    // Fields: L=0, op1=0, CRm=0, op2=0, CRn=0, Rt=31
    let encoding: u32 = 0xD508001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_system_sysops_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_d508001f()
 {
    // Encoding: 0xD508001F
    // Test aarch64_system_sysops special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt=31, op2=0, op1=0, CRn=0, L=0, CRm=0
    let encoding: u32 = 0xD508001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_sysops
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_system_sysops_reg_write_0_d5080000() {
    // Test aarch64_system_sysops register write: GpFromField("t")
    // Encoding: 0xD5080000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD5080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_system_sysops
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_system_sysops_zr_rt_d508001f() {
    // Test aarch64_system_sysops with Rt = ZR (31)
    // Encoding: 0xD508001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD508001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_system_barriers Tests
// ============================================================================

/// Provenance: aarch64_system_barriers
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_barriers_field_crm_0_min_309f_d503309f() {
    // Encoding: 0xD503309F
    // Test aarch64_system_barriers field CRm = 0 (Min)
    // Fields: CRm=0, opc=0
    let encoding: u32 = 0xD503309F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_barriers_field_crm_1_poweroftwo_309f_d503319f() {
    // Encoding: 0xD503319F
    // Test aarch64_system_barriers field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1, opc=0
    let encoding: u32 = 0xD503319F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_barriers_field_crm_7_poweroftwominusone_309f_d503379f() {
    // Encoding: 0xD503379F
    // Test aarch64_system_barriers field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7, opc=0
    let encoding: u32 = 0xD503379F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_barriers_field_crm_15_max_309f_d5033f9f() {
    // Encoding: 0xD5033F9F
    // Test aarch64_system_barriers field CRm = 15 (Max)
    // Fields: CRm=15, opc=0
    let encoding: u32 = 0xD5033F9F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc 5 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_system_barriers_field_opc_0_min_309f_d503309f() {
    // Encoding: 0xD503309F
    // Test aarch64_system_barriers field opc = 0 (Min)
    // Fields: opc=0, CRm=0
    let encoding: u32 = 0xD503309F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc 5 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_system_barriers_field_opc_1_poweroftwo_309f_d50330bf() {
    // Encoding: 0xD50330BF
    // Test aarch64_system_barriers field opc = 1 (PowerOfTwo)
    // Fields: opc=1, CRm=0
    let encoding: u32 = 0xD50330BF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc 5 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_system_barriers_field_opc_2_poweroftwo_309f_d50330df() {
    // Encoding: 0xD50330DF
    // Test aarch64_system_barriers field opc = 2 (PowerOfTwo)
    // Fields: CRm=0, opc=2
    let encoding: u32 = 0xD50330DF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc 5 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_system_barriers_field_opc_3_max_309f_d50330ff() {
    // Encoding: 0xD50330FF
    // Test aarch64_system_barriers field opc = 3 (Max)
    // Fields: CRm=0, opc=3
    let encoding: u32 = 0xD50330FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_system_barriers_combo_0_309f_d503309f() {
    // Encoding: 0xD503309F
    // Test aarch64_system_barriers field combination: CRm=0, opc=0
    // Fields: opc=0, CRm=0
    let encoding: u32 = 0xD503309F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_system_barriers_combo_1_309f_d503319f() {
    // Encoding: 0xD503319F
    // Test aarch64_system_barriers field combination: CRm=1, opc=0
    // Fields: CRm=1, opc=0
    let encoding: u32 = 0xD503319F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_system_barriers_combo_2_309f_d503379f() {
    // Encoding: 0xD503379F
    // Test aarch64_system_barriers field combination: CRm=7, opc=0
    // Fields: CRm=7, opc=0
    let encoding: u32 = 0xD503379F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_system_barriers_combo_3_309f_d5033f9f() {
    // Encoding: 0xD5033F9F
    // Test aarch64_system_barriers field combination: CRm=15, opc=0
    // Fields: CRm=15, opc=0
    let encoding: u32 = 0xD5033F9F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_system_barriers_combo_4_309f_d503309f() {
    // Encoding: 0xD503309F
    // Test aarch64_system_barriers field combination: CRm=0, opc=0
    // Fields: opc=0, CRm=0
    let encoding: u32 = 0xD503309F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_system_barriers_combo_5_309f_d50330bf() {
    // Encoding: 0xD50330BF
    // Test aarch64_system_barriers field combination: CRm=0, opc=1
    // Fields: CRm=0, opc=1
    let encoding: u32 = 0xD50330BF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_system_barriers_combo_6_309f_d50330df() {
    // Encoding: 0xD50330DF
    // Test aarch64_system_barriers field combination: CRm=0, opc=2
    // Fields: CRm=0, opc=2
    let encoding: u32 = 0xD50330DF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_system_barriers_combo_7_309f_d50330ff() {
    // Encoding: 0xD50330FF
    // Test aarch64_system_barriers field combination: CRm=0, opc=3
    // Fields: CRm=0, opc=3
    let encoding: u32 = 0xD50330FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_system_barriers_special_opc_0_size_variant_0_12447_d503309f() {
    // Encoding: 0xD503309F
    // Test aarch64_system_barriers special value opc = 0 (Size variant 0)
    // Fields: CRm=0, opc=0
    let encoding: u32 = 0xD503309F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_system_barriers_special_opc_1_size_variant_1_12447_d50330bf() {
    // Encoding: 0xD50330BF
    // Test aarch64_system_barriers special value opc = 1 (Size variant 1)
    // Fields: opc=1, CRm=0
    let encoding: u32 = 0xD50330BF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_system_barriers_special_opc_2_size_variant_2_12447_d50330df() {
    // Encoding: 0xD50330DF
    // Test aarch64_system_barriers special value opc = 2 (Size variant 2)
    // Fields: opc=2, CRm=0
    let encoding: u32 = 0xD50330DF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_barriers
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_system_barriers_special_opc_3_size_variant_3_12447_d50330ff() {
    // Encoding: 0xD50330FF
    // Test aarch64_system_barriers special value opc = 3 (Size variant 3)
    // Fields: opc=3, CRm=0
    let encoding: u32 = 0xD50330FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

// ============================================================================
// aarch64_system_monitors Tests
// ============================================================================

/// Provenance: aarch64_system_monitors
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_monitors_field_crm_0_min_305f_d503305f() {
    // Encoding: 0xD503305F
    // Test aarch64_system_monitors field CRm = 0 (Min)
    // Fields: CRm=0
    let encoding: u32 = 0xD503305F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_monitors_field_crm_1_poweroftwo_305f_d503315f() {
    // Encoding: 0xD503315F
    // Test aarch64_system_monitors field CRm = 1 (PowerOfTwo)
    // Fields: CRm=1
    let encoding: u32 = 0xD503315F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_monitors_field_crm_7_poweroftwominusone_305f_d503375f() {
    // Encoding: 0xD503375F
    // Test aarch64_system_monitors field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: CRm=7
    let encoding: u32 = 0xD503375F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_monitors_field_crm_15_max_305f_d5033f5f() {
    // Encoding: 0xD5033F5F
    // Test aarch64_system_monitors field CRm = 15 (Max)
    // Fields: CRm=15
    let encoding: u32 = 0xD5033F5F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_system_monitors_combo_0_305f_d503305f() {
    // Encoding: 0xD503305F
    // Test aarch64_system_monitors field combination: CRm=0
    // Fields: CRm=0
    let encoding: u32 = 0xD503305F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_system_monitors_combo_1_305f_d503315f() {
    // Encoding: 0xD503315F
    // Test aarch64_system_monitors field combination: CRm=1
    // Fields: CRm=1
    let encoding: u32 = 0xD503315F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_system_monitors_combo_2_305f_d503375f() {
    // Encoding: 0xD503375F
    // Test aarch64_system_monitors field combination: CRm=7
    // Fields: CRm=7
    let encoding: u32 = 0xD503375F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_monitors
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_system_monitors_combo_3_305f_d5033f5f() {
    // Encoding: 0xD5033F5F
    // Test aarch64_system_monitors field combination: CRm=15
    // Fields: CRm=15
    let encoding: u32 = 0xD5033F5F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}
