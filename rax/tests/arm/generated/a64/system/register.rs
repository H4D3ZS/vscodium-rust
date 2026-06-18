//! A64 system register tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_system_register_system Tests
// ============================================================================

/// Provenance: aarch64_system_register_system
/// ASL: `field L 21 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_system_field_l_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field L = 0 (Min)
    // Fields: L=0, CRm=0, op1=0, op2=0, o0=0, CRn=0, Rt=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field L 21 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_system_register_system_field_l_1_max_0_d5300000() {
    // Encoding: 0xD5300000
    // Test aarch64_system_register_system field L = 1 (Max)
    // Fields: Rt=0, o0=0, L=1, op1=0, CRn=0, CRm=0, op2=0
    let encoding: u32 = 0xD5300000;
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

/// Provenance: aarch64_system_register_system
/// ASL: `field o0 19 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_system_field_o0_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field o0 = 0 (Min)
    // Fields: CRm=0, Rt=0, op1=0, CRn=0, op2=0, L=0, o0=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field o0 19 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_system_register_system_field_o0_1_max_0_d5180000() {
    // Encoding: 0xD5180000
    // Test aarch64_system_register_system field o0 = 1 (Max)
    // Fields: CRm=0, op2=0, Rt=0, L=0, op1=0, o0=1, CRn=0
    let encoding: u32 = 0xD5180000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_system_field_op1_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field op1 = 0 (Min)
    // Fields: Rt=0, L=0, op2=0, o0=0, op1=0, CRn=0, CRm=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_system_field_op1_1_poweroftwo_0_d5110000() {
    // Encoding: 0xD5110000
    // Test aarch64_system_register_system field op1 = 1 (PowerOfTwo)
    // Fields: CRm=0, o0=0, op2=0, Rt=0, CRn=0, op1=1, L=0
    let encoding: u32 = 0xD5110000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_register_system_field_op1_7_max_0_d5170000() {
    // Encoding: 0xD5170000
    // Test aarch64_system_register_system field op1 = 7 (Max)
    // Fields: CRn=0, CRm=0, Rt=0, L=0, op2=0, op1=7, o0=0
    let encoding: u32 = 0xD5170000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_system_field_crn_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field CRn = 0 (Min)
    // Fields: CRm=0, L=0, op1=0, o0=0, CRn=0, op2=0, Rt=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_system_field_crn_1_poweroftwo_0_d5101000() {
    // Encoding: 0xD5101000
    // Test aarch64_system_register_system field CRn = 1 (PowerOfTwo)
    // Fields: CRn=1, L=0, CRm=0, op2=0, op1=0, Rt=0, o0=0
    let encoding: u32 = 0xD5101000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_register_system_field_crn_7_poweroftwominusone_0_d5107000() {
    // Encoding: 0xD5107000
    // Test aarch64_system_register_system field CRn = 7 (PowerOfTwoMinusOne)
    // Fields: op2=0, Rt=0, CRm=0, L=0, o0=0, op1=0, CRn=7
    let encoding: u32 = 0xD5107000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRn 12 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_register_system_field_crn_15_max_0_d510f000() {
    // Encoding: 0xD510F000
    // Test aarch64_system_register_system field CRn = 15 (Max)
    // Fields: L=0, o0=0, op1=0, CRn=15, CRm=0, op2=0, Rt=0
    let encoding: u32 = 0xD510F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_system_field_crm_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field CRm = 0 (Min)
    // Fields: L=0, op1=0, op2=0, Rt=0, CRm=0, o0=0, CRn=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_system_field_crm_1_poweroftwo_0_d5100100() {
    // Encoding: 0xD5100100
    // Test aarch64_system_register_system field CRm = 1 (PowerOfTwo)
    // Fields: op2=0, Rt=0, L=0, op1=0, CRn=0, CRm=1, o0=0
    let encoding: u32 = 0xD5100100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_register_system_field_crm_7_poweroftwominusone_0_d5100700() {
    // Encoding: 0xD5100700
    // Test aarch64_system_register_system field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: o0=0, CRm=7, op2=0, op1=0, L=0, CRn=0, Rt=0
    let encoding: u32 = 0xD5100700;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_register_system_field_crm_15_max_0_d5100f00() {
    // Encoding: 0xD5100F00
    // Test aarch64_system_register_system field CRm = 15 (Max)
    // Fields: CRm=15, L=0, CRn=0, op2=0, Rt=0, op1=0, o0=0
    let encoding: u32 = 0xD5100F00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_system_field_op2_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field op2 = 0 (Min)
    // Fields: CRn=0, op2=0, op1=0, o0=0, Rt=0, CRm=0, L=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_system_field_op2_1_poweroftwo_0_d5100020() {
    // Encoding: 0xD5100020
    // Test aarch64_system_register_system field op2 = 1 (PowerOfTwo)
    // Fields: Rt=0, op1=0, CRn=0, o0=0, L=0, CRm=0, op2=1
    let encoding: u32 = 0xD5100020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_register_system_field_op2_7_max_0_d51000e0() {
    // Encoding: 0xD51000E0
    // Test aarch64_system_register_system field op2 = 7 (Max)
    // Fields: CRm=0, Rt=0, op1=0, o0=0, L=0, CRn=0, op2=7
    let encoding: u32 = 0xD51000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_system_register_system_field_rt_0_min_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field Rt = 0 (Min)
    // Fields: Rt=0, o0=0, CRn=0, CRm=0, L=0, op2=0, op1=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_system_register_system_field_rt_1_poweroftwo_0_d5100001() {
    // Encoding: 0xD5100001
    // Test aarch64_system_register_system field Rt = 1 (PowerOfTwo)
    // Fields: op1=0, CRn=0, op2=0, L=0, o0=0, Rt=1, CRm=0
    let encoding: u32 = 0xD5100001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_system_register_system_field_rt_30_poweroftwominusone_0_d510001e() {
    // Encoding: 0xD510001E
    // Test aarch64_system_register_system field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=30, L=0, op1=0, CRn=0, o0=0, CRm=0, op2=0
    let encoding: u32 = 0xD510001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_system_register_system_field_rt_31_max_0_d510001f() {
    // Encoding: 0xD510001F
    // Test aarch64_system_register_system field Rt = 31 (Max)
    // Fields: CRm=0, Rt=31, o0=0, CRn=0, L=0, op2=0, op1=0
    let encoding: u32 = 0xD510001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_system_register_system_combo_0_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRn=0, L=0, op1=0, op2=0, Rt=0, o0=0, CRm=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_system_register_system_combo_1_0_d5300000() {
    // Encoding: 0xD5300000
    // Test aarch64_system_register_system field combination: L=1, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: L=1, o0=0, op1=0, op2=0, Rt=0, CRn=0, CRm=0
    let encoding: u32 = 0xD5300000;
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

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_system_register_system_combo_2_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRm=0, op1=0, op2=0, Rt=0, L=0, o0=0, CRn=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_system_register_system_combo_3_0_d5180000() {
    // Encoding: 0xD5180000
    // Test aarch64_system_register_system field combination: L=0, o0=1, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: L=0, Rt=0, op1=0, CRm=0, CRn=0, o0=1, op2=0
    let encoding: u32 = 0xD5180000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=0 (minimum value)
#[test]
fn test_aarch64_system_register_system_combo_4_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: o0=0, op2=0, op1=0, CRn=0, Rt=0, L=0, CRm=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=1 (value 1)
#[test]
fn test_aarch64_system_register_system_combo_5_0_d5110000() {
    // Encoding: 0xD5110000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=1, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRm=0, op1=1, o0=0, CRn=0, Rt=0, L=0, op2=0
    let encoding: u32 = 0xD5110000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=7 (maximum value (7))
#[test]
fn test_aarch64_system_register_system_combo_6_0_d5170000() {
    // Encoding: 0xD5170000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=7, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: L=0, CRm=0, o0=0, CRn=0, op1=7, op2=0, Rt=0
    let encoding: u32 = 0xD5170000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=0 (minimum value)
#[test]
fn test_aarch64_system_register_system_combo_7_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: CRn=0, CRm=0, op1=0, o0=0, L=0, op2=0, Rt=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=1 (value 1)
#[test]
fn test_aarch64_system_register_system_combo_8_0_d5101000() {
    // Encoding: 0xD5101000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=1, CRm=0, op2=0, Rt=0
    // Fields: op1=0, L=0, o0=0, CRn=1, CRm=0, op2=0, Rt=0
    let encoding: u32 = 0xD5101000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=7 (midpoint (7))
#[test]
fn test_aarch64_system_register_system_combo_9_0_d5107000() {
    // Encoding: 0xD5107000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=7, CRm=0, op2=0, Rt=0
    // Fields: L=0, CRn=7, o0=0, CRm=0, op2=0, Rt=0, op1=0
    let encoding: u32 = 0xD5107000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRn=15 (maximum value (15))
#[test]
fn test_aarch64_system_register_system_combo_10_0_d510f000() {
    // Encoding: 0xD510F000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=15, CRm=0, op2=0, Rt=0
    // Fields: L=0, op1=0, Rt=0, o0=0, op2=0, CRn=15, CRm=0
    let encoding: u32 = 0xD510F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_system_register_system_combo_11_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: op1=0, op2=0, CRn=0, o0=0, CRm=0, Rt=0, L=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_system_register_system_combo_12_0_d5100100() {
    // Encoding: 0xD5100100
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=1, op2=0, Rt=0
    // Fields: Rt=0, op1=0, CRm=1, op2=0, o0=0, L=0, CRn=0
    let encoding: u32 = 0xD5100100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_system_register_system_combo_13_0_d5100700() {
    // Encoding: 0xD5100700
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=7, op2=0, Rt=0
    // Fields: CRn=0, o0=0, op2=0, CRm=7, Rt=0, L=0, op1=0
    let encoding: u32 = 0xD5100700;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_system_register_system_combo_14_0_d5100f00() {
    // Encoding: 0xD5100F00
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=15, op2=0, Rt=0
    // Fields: o0=0, op2=0, L=0, CRn=0, Rt=0, CRm=15, op1=0
    let encoding: u32 = 0xD5100F00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_system_register_system_combo_15_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: op1=0, o0=0, CRn=0, Rt=0, CRm=0, op2=0, L=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_system_register_system_combo_16_0_d5100020() {
    // Encoding: 0xD5100020
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=1, Rt=0
    // Fields: CRn=0, op2=1, Rt=0, L=0, op1=0, o0=0, CRm=0
    let encoding: u32 = 0xD5100020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_system_register_system_combo_17_0_d51000e0() {
    // Encoding: 0xD51000E0
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=7, Rt=0
    // Fields: o0=0, L=0, op2=7, op1=0, CRn=0, CRm=0, Rt=0
    let encoding: u32 = 0xD51000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_system_register_system_combo_18_0_d5100000() {
    // Encoding: 0xD5100000
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=0
    // Fields: op2=0, op1=0, o0=0, Rt=0, L=0, CRn=0, CRm=0
    let encoding: u32 = 0xD5100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_system_register_system_combo_19_0_d5100001() {
    // Encoding: 0xD5100001
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=1
    // Fields: CRn=0, CRm=0, o0=0, op2=0, Rt=1, op1=0, L=0
    let encoding: u32 = 0xD5100001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_system_register_system_combo_20_0_d510001e() {
    // Encoding: 0xD510001E
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=30
    // Fields: op2=0, CRm=0, Rt=30, op1=0, o0=0, L=0, CRn=0
    let encoding: u32 = 0xD510001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_system_register_system_combo_21_0_d510001f() {
    // Encoding: 0xD510001F
    // Test aarch64_system_register_system field combination: L=0, o0=0, op1=0, CRn=0, CRm=0, op2=0, Rt=31
    // Fields: op2=0, o0=0, Rt=31, L=0, CRn=0, op1=0, CRm=0
    let encoding: u32 = 0xD510001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_system_register_system_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_d510001f()
 {
    // Encoding: 0xD510001F
    // Test aarch64_system_register_system special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: op2=0, o0=0, L=0, CRn=0, Rt=31, op1=0, CRm=0
    let encoding: u32 = 0xD510001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_system
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_system_register_system_reg_write_0_d5100000() {
    // Test aarch64_system_register_system register write: GpFromField("t")
    // Encoding: 0xD5100000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD5100000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_system_register_system
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_system_register_system_zr_rt_d510001f() {
    // Test aarch64_system_register_system with Rt = ZR (31)
    // Encoding: 0xD510001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD510001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_system_register_cpsr Tests
// ============================================================================

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_cpsr_field_op1_0_min_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_system_register_cpsr field op1 = 0 (Min)
    // Fields: op1=0, CRm=0, op2=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_cpsr_field_op1_1_poweroftwo_401f_d501401f() {
    // Encoding: 0xD501401F
    // Test aarch64_system_register_cpsr field op1 = 1 (PowerOfTwo)
    // Fields: op1=1, op2=0, CRm=0
    let encoding: u32 = 0xD501401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field op1 16 +: 3`
/// Requirement: FieldBoundary { field: "op1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_register_cpsr_field_op1_7_max_401f_d507401f() {
    // Encoding: 0xD507401F
    // Test aarch64_system_register_cpsr field op1 = 7 (Max)
    // Fields: op2=0, CRm=0, op1=7
    let encoding: u32 = 0xD507401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_cpsr_field_crm_0_min_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_system_register_cpsr field CRm = 0 (Min)
    // Fields: op1=0, CRm=0, op2=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_cpsr_field_crm_1_poweroftwo_401f_d500411f() {
    // Encoding: 0xD500411F
    // Test aarch64_system_register_cpsr field CRm = 1 (PowerOfTwo)
    // Fields: op1=0, op2=0, CRm=1
    let encoding: u32 = 0xD500411F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_system_register_cpsr_field_crm_7_poweroftwominusone_401f_d500471f() {
    // Encoding: 0xD500471F
    // Test aarch64_system_register_cpsr field CRm = 7 (PowerOfTwoMinusOne)
    // Fields: op2=0, op1=0, CRm=7
    let encoding: u32 = 0xD500471F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field CRm 8 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_system_register_cpsr_field_crm_15_max_401f_d5004f1f() {
    // Encoding: 0xD5004F1F
    // Test aarch64_system_register_cpsr field CRm = 15 (Max)
    // Fields: CRm=15, op1=0, op2=0
    let encoding: u32 = 0xD5004F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_system_register_cpsr_field_op2_0_min_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_system_register_cpsr field op2 = 0 (Min)
    // Fields: op1=0, CRm=0, op2=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_system_register_cpsr_field_op2_1_poweroftwo_401f_d500403f() {
    // Encoding: 0xD500403F
    // Test aarch64_system_register_cpsr field op2 = 1 (PowerOfTwo)
    // Fields: op1=0, CRm=0, op2=1
    let encoding: u32 = 0xD500403F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field op2 5 +: 3`
/// Requirement: FieldBoundary { field: "op2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_system_register_cpsr_field_op2_7_max_401f_d50040ff() {
    // Encoding: 0xD50040FF
    // Test aarch64_system_register_cpsr field op2 = 7 (Max)
    // Fields: op1=0, CRm=0, op2=7
    let encoding: u32 = 0xD50040FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=0 (minimum value)
#[test]
fn test_aarch64_system_register_cpsr_combo_0_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=0, op2=0
    // Fields: op1=0, CRm=0, op2=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=1 (value 1)
#[test]
fn test_aarch64_system_register_cpsr_combo_1_401f_d501401f() {
    // Encoding: 0xD501401F
    // Test aarch64_system_register_cpsr field combination: op1=1, CRm=0, op2=0
    // Fields: CRm=0, op2=0, op1=1
    let encoding: u32 = 0xD501401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op1=7 (maximum value (7))
#[test]
fn test_aarch64_system_register_cpsr_combo_2_401f_d507401f() {
    // Encoding: 0xD507401F
    // Test aarch64_system_register_cpsr field combination: op1=7, CRm=0, op2=0
    // Fields: op1=7, CRm=0, op2=0
    let encoding: u32 = 0xD507401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=0 (minimum value)
#[test]
fn test_aarch64_system_register_cpsr_combo_3_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=0, op2=0
    // Fields: CRm=0, op1=0, op2=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=1 (value 1)
#[test]
fn test_aarch64_system_register_cpsr_combo_4_401f_d500411f() {
    // Encoding: 0xD500411F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=1, op2=0
    // Fields: CRm=1, op2=0, op1=0
    let encoding: u32 = 0xD500411F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=7 (midpoint (7))
#[test]
fn test_aarch64_system_register_cpsr_combo_5_401f_d500471f() {
    // Encoding: 0xD500471F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=7, op2=0
    // Fields: CRm=7, op2=0, op1=0
    let encoding: u32 = 0xD500471F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// CRm=15 (maximum value (15))
#[test]
fn test_aarch64_system_register_cpsr_combo_6_401f_d5004f1f() {
    // Encoding: 0xD5004F1F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=15, op2=0
    // Fields: CRm=15, op2=0, op1=0
    let encoding: u32 = 0xD5004F1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=0 (minimum value)
#[test]
fn test_aarch64_system_register_cpsr_combo_7_401f_d500401f() {
    // Encoding: 0xD500401F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=0, op2=0
    // Fields: CRm=0, op2=0, op1=0
    let encoding: u32 = 0xD500401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=1 (value 1)
#[test]
fn test_aarch64_system_register_cpsr_combo_8_401f_d500403f() {
    // Encoding: 0xD500403F
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=0, op2=1
    // Fields: op2=1, op1=0, CRm=0
    let encoding: u32 = 0xD500403F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_system_register_cpsr
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op2=7 (maximum value (7))
#[test]
fn test_aarch64_system_register_cpsr_combo_9_401f_d50040ff() {
    // Encoding: 0xD50040FF
    // Test aarch64_system_register_cpsr field combination: op1=0, CRm=0, op2=7
    // Fields: op1=0, CRm=0, op2=7
    let encoding: u32 = 0xD50040FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}
